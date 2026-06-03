// Code generation module - generates LLVM IR using Inkwell

use crate::ast::{Ast, BinaryOp, Expr, Factor, WithDecl};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::IntValue;
use inkwell::AddressSpace;
use std::collections::HashMap;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    builder: Builder<'ctx>,
    name_map: HashMap<String, IntValue<'ctx>>,
    i32_type: inkwell::types::IntType<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("calc.expr");
        let builder = context.create_builder();
        let i32_type = context.i32_type();

        Self {
            context,
            module,
            builder,
            name_map: HashMap::new(),
            i32_type,
        }
    }

    pub fn compile(&mut self, ast: &Ast) -> Result<(), String> {
        // Create main function
        let i32_type = self.i32_type;
        let void_type = self.context.void_type();
        let pointer_type = self.context.ptr_type(AddressSpace::default());

        // Main function signature: i32 (i32, i8**)
        let main_type = i32_type.fn_type(&[i32_type.into(), pointer_type.into()], false);
        let main_fn = self.module.add_function("main", main_type, None);

        // Create entry block
        let entry_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_block);

        // Compile the expression
        let value = self.compile_ast(ast)?;

        // Declare calc_write function
        let calc_write_type = void_type.fn_type(&[i32_type.into()], false);
        let calc_write_fn = self.module.add_function("calc_write", calc_write_type, None);

        // Call calc_write with the result
        let _ = self.builder.build_call(calc_write_fn, &[value.into()], "call");

        // Return 0
        let zero = i32_type.const_int(0, true);
        let _ = self.builder.build_return(Some(&zero));

        Ok(())
    }

    fn compile_ast(&mut self, ast: &Ast) -> Result<IntValue<'ctx>, String> {
        match ast {
            Ast::Expr(expr) => self.compile_expr(expr),
            Ast::WithDecl(decl) => self.compile_with_decl(decl),
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<IntValue<'ctx>, String> {
        match expr {
            Expr::Factor(factor) => self.compile_factor(factor),
            Expr::BinaryOp(op) => self.compile_binary_op(op),
        }
    }

    fn compile_factor(&mut self, factor: &Factor) -> Result<IntValue<'ctx>, String> {
        match factor {
            Factor::Number(n) => Ok(self.i32_type.const_int(*n as u64, true)),
            Factor::Ident(name) => {
                self.name_map
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("Variable '{}' not found in code generation", name))
            }
        }
    }

    fn compile_binary_op(&mut self, op: &BinaryOp) -> Result<IntValue<'ctx>, String> {
        let left = self.compile_expr(&op.left())?;
        let right = self.compile_expr(&op.right())?;

        let result = match op {
            BinaryOp::Add { .. } => self.builder.build_int_add(left, right, "add").map_err(|e| e.to_string())?,
            BinaryOp::Sub { .. } => self.builder.build_int_sub(left, right, "sub").map_err(|e| e.to_string())?,
            BinaryOp::Mul { .. } => self.builder.build_int_mul(left, right, "mul").map_err(|e| e.to_string())?,
            BinaryOp::Div { .. } => self.builder.build_int_signed_div(left, right, "div").map_err(|e| e.to_string())?,
        };

        Ok(result)
    }

    fn compile_with_decl(&mut self, decl: &WithDecl) -> Result<IntValue<'ctx>, String> {
        // Declare calc_read function: i32 (i8*)
        let i32_type = self.i32_type;
        let _i8_type = self.context.i8_type();
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
        let calc_read_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
        let calc_read_fn = self.module.add_function("calc_read", calc_read_type, None);

        // Read all variables
        for var_name in &decl.vars {
            // Create global string for variable name
            let var_name_str = format!("{}.str", var_name);
            let string_val = self.context.const_string(var_name.as_bytes(), true);
            let global = self.module.add_global(string_val.get_type(), None, &var_name_str);
            global.set_initializer(&string_val);
            global.set_constant(true);
            global.set_linkage(inkwell::module::Linkage::Private);

            // Get pointer to the string data
            let string_ptr = global
                .as_pointer_value()
                .const_cast(i8_ptr_type);

            // Call calc_read
            let call_site = self
                .builder
                .build_call(calc_read_fn, &[string_ptr.into()], "read")
                .map_err(|e| e.to_string())?;

            // Get the return value
            let value = call_site
                .try_as_basic_value()
                .left()
                .unwrap()
                .into_int_value();

            // Store in name_map
            self.name_map.insert(var_name.clone(), value);
        }

        // Compile the expression
        self.compile_expr(&decl.expr)
    }

    pub fn print_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn print_to_file(&self) -> Result<(), String> {
        self.module.print_to_file("calc.expr.ll").map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::sema::Sema;

    #[test]
    fn test_codegen_simple_number() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context);

        let input = "42";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        sema.analyze(&ast).unwrap();

        let result = codegen.compile(&ast);
        assert!(result.is_ok());

        let ir = codegen.print_to_string();
        assert!(ir.contains("i32 42"));
    }

    #[test]
    fn test_codegen_arithmetic() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context);

        let input = "1 + 2 * 3";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        sema.analyze(&ast).unwrap();

        let result = codegen.compile(&ast);
        assert!(result.is_ok());

        let ir = codegen.print_to_string();
        assert!(ir.contains("define i32 @main"));
        assert!(ir.contains("calc_write"));
    }

    #[test]
    fn test_codegen_with_decl() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context);

        let input = "with x: x + 1";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        sema.analyze(&ast).unwrap();

        let result = codegen.compile(&ast);
        assert!(result.is_ok());

        let ir = codegen.print_to_string();
        assert!(ir.contains("calc_read"));
        assert!(ir.contains("x.str"));
    }
}
