extern "C" {
#endif
int test_op(buffer_t *_in_f32_buffer, buffer_t *_in_f64_buffer, buffer_t *_in_i8_buffer, buffer_t *_in_u8_buffer, buffer_t *_in_i16_buffer, buffer_t *_in_u16_buffer, buffer_t *_in_i32_buffer, buffer_t *_in_u32_buffer, buffer_t *_in_i64_buffer, buffer_t *_in_u64_buffer, buffer_t *_out_op_buffer) HALIDE_FUNCTION_ATTRS;
int test_op_argv(void **args) HALIDE_FUNCTION_ATTRS;
extern const struct halide_filter_metadata_t test_op_metadata;

int scalar_test_op(buffer_t *_in_f32_buffer, buffer_t *_in_f64_buffer, buffer_t *_in_i8_buffer, buffer_t *_in_u8_buffer, buffer_t *_in_i16_buffer, buffer_t *_in_u16_buffer, buffer_t *_in_i32_buffer, buffer_t *_in_u32_buffer, buffer_t *_in_i64_buffer, buffer_t *_in_u64_buffer, buffer_t *_out_op_buffer) HALIDE_FUNCTION_ATTRS;
int scalar_test_op_argv(void **args) HALIDE_FUNCTION_ATTRS;
extern const struct halide_filter_metadata_t scalar_test_op_metadata;
#ifdef __cplusplus
struct halide_filter_metadata_t;
#ifndef HALIDE_FUNCTION_ATTRS
#define HALIDE_FUNCTION_ATTRS
#endif
