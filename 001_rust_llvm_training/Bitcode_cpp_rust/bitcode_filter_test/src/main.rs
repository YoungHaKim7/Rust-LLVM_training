use std::ffi::c_void;

#[repr(C)]
pub struct buffer_t {
    _private: [u8; 0],
}

impl buffer_t {
    pub fn uninit() -> Self {
        Self { _private: [] }
    }
}

#[repr(C)]
pub struct halide_filter_metadata_t {
    _private: [u8; 0],
}

// Stub implementation of test_op for testing
#[unsafe(no_mangle)]
pub extern "C" fn test_op(
    in_f32_buffer: *mut buffer_t,
    in_f64_buffer: *mut buffer_t,
    in_i8_buffer: *mut buffer_t,
    in_u8_buffer: *mut buffer_t,
    in_i16_buffer: *mut buffer_t,
    in_u16_buffer: *mut buffer_t,
    in_i32_buffer: *mut buffer_t,
    in_u32_buffer: *mut buffer_t,
    in_i64_buffer: *mut buffer_t,
    in_u64_buffer: *mut buffer_t,
    out_op_buffer: *mut buffer_t,
) -> i32 {
    // Stub implementation - returns 0 for success
    // Suppress unused warnings by using parameters
    let _ = (
        in_f32_buffer, in_f64_buffer, in_i8_buffer, in_u8_buffer,
        in_i16_buffer, in_u16_buffer, in_i32_buffer, in_u32_buffer,
        in_i64_buffer, in_u64_buffer, out_op_buffer,
    );
    println!("test_op called (stub implementation)");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn test_op_argv(args: *mut *mut c_void) -> i32 {
    let _ = args;
    println!("test_op_argv called (stub implementation)");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn scalar_test_op(
    in_f32_buffer: *mut buffer_t,
    in_f64_buffer: *mut buffer_t,
    in_i8_buffer: *mut buffer_t,
    in_u8_buffer: *mut buffer_t,
    in_i16_buffer: *mut buffer_t,
    in_u16_buffer: *mut buffer_t,
    in_i32_buffer: *mut buffer_t,
    in_u32_buffer: *mut buffer_t,
    in_i64_buffer: *mut buffer_t,
    in_u64_buffer: *mut buffer_t,
    out_op_buffer: *mut buffer_t,
) -> i32 {
    let _ = (
        in_f32_buffer, in_f64_buffer, in_i8_buffer, in_u8_buffer,
        in_i16_buffer, in_u16_buffer, in_i32_buffer, in_u32_buffer,
        in_i64_buffer, in_u64_buffer, out_op_buffer,
    );
    println!("scalar_test_op called (stub implementation)");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn scalar_test_op_argv(args: *mut *mut c_void) -> i32 {
    let _ = args;
    println!("scalar_test_op_argv called (stub implementation)");
    0
}

#[repr(C)]
pub struct halide_dimension_t {
    pub min: i32,
    pub extent: i32,
    pub stride: i32,
    pub flags: u32,
}

#[repr(C)]
pub struct halide_type_t {
    pub code: u8,
    pub bits: u8,
    pub lanes: u16,
}

#[repr(C)]
pub struct halide_buffer_t {
    pub device: u64,
    pub device_interface: *mut c_void,
    pub host: *mut u8,
    pub flags: u64,
    pub type_: halide_type_t,
    pub dimensions: i32,
    pub dim: *mut halide_dimension_t,
    pub padding: *mut c_void,
}
fn main() {
    let mut in_f32 = buffer_t::uninit();
    let mut in_f64 = buffer_t::uninit();
    let mut in_i8 = buffer_t::uninit();
    let mut in_u8 = buffer_t::uninit();
    let mut in_i16 = buffer_t::uninit();
    let mut in_u16 = buffer_t::uninit();
    let mut in_i32 = buffer_t::uninit();
    let mut in_u32 = buffer_t::uninit();
    let mut in_i64 = buffer_t::uninit();
    let mut in_u64 = buffer_t::uninit();
    let mut out = buffer_t::uninit();

    let ret = test_op(
        &mut in_f32,
        &mut in_f64,
        &mut in_i8,
        &mut in_u8,
        &mut in_i16,
        &mut in_u16,
        &mut in_i32,
        &mut in_u32,
        &mut in_i64,
        &mut in_u64,
        &mut out,
    );

    println!("return code = {}", ret);
}
