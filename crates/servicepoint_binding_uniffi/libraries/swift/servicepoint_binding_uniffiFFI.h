// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// The following structs are used to implement the lowest level
// of the FFI, and thus useful to multiple uniffied crates.
// We ensure they are declared exactly once, with a header guard, UNIFFI_SHARED_H.
#ifdef UNIFFI_SHARED_H
    // We also try to prevent mixing versions of shared uniffi header structs.
    // If you add anything to the #else block, you must increment the version suffix in UNIFFI_SHARED_HEADER_V4
    #ifndef UNIFFI_SHARED_HEADER_V4
        #error Combining helper code from multiple versions of uniffi is not supported
    #endif // ndef UNIFFI_SHARED_HEADER_V4
#else
#define UNIFFI_SHARED_H
#define UNIFFI_SHARED_HEADER_V4
// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️

typedef struct RustBuffer
{
    int32_t capacity;
    int32_t len;
    uint8_t *_Nullable data;
} RustBuffer;

typedef int32_t (*ForeignCallback)(uint64_t, int32_t, const uint8_t *_Nonnull, int32_t, RustBuffer *_Nonnull);

// Task defined in Rust that Swift executes
typedef void (*UniFfiRustTaskCallback)(const void * _Nullable, int8_t);

// Callback to execute Rust tasks using a Swift Task
//
// Args:
//   executor: ForeignExecutor lowered into a size_t value
//   delay: Delay in MS
//   task: UniFfiRustTaskCallback to call
//   task_data: data to pass the task callback
typedef int8_t (*UniFfiForeignExecutorCallback)(size_t, uint32_t, UniFfiRustTaskCallback _Nullable, const void * _Nullable);

typedef struct ForeignBytes
{
    int32_t len;
    const uint8_t *_Nullable data;
} ForeignBytes;

// Error definitions
typedef struct RustCallStatus {
    int8_t code;
    RustBuffer errorBuf;
} RustCallStatus;

// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️
#endif // def UNIFFI_SHARED_H

// Continuation callback for UniFFI Futures
typedef void (*UniFfiRustFutureContinuation)(void * _Nonnull, int8_t);

// Scaffolding functions
void uniffi_servicepoint_binding_uniffi_fn_free_bitvec(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_clone(void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_load(RustBuffer data, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_new(uint64_t size, RustCallStatus *_Nonnull out_status
);
RustBuffer uniffi_servicepoint_binding_uniffi_fn_method_bitvec_copy_raw(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_bitvec_equals(void*_Nonnull ptr, void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_bitvec_fill(void*_Nonnull ptr, int8_t value, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_bitvec_get(void*_Nonnull ptr, uint64_t index, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_bitvec_len(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_bitvec_set(void*_Nonnull ptr, uint64_t index, int8_t value, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_free_bitmap(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_clone(void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_load(uint64_t width, uint64_t height, RustBuffer data, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new(uint64_t width, uint64_t height, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new_max_sized(RustCallStatus *_Nonnull out_status
    
);
RustBuffer uniffi_servicepoint_binding_uniffi_fn_method_bitmap_copy_raw(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_bitmap_equals(void*_Nonnull ptr, void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_bitmap_fill(void*_Nonnull ptr, int8_t value, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_bitmap_get(void*_Nonnull ptr, uint64_t x, uint64_t y, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_bitmap_height(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_bitmap_set(void*_Nonnull ptr, uint64_t x, uint64_t y, int8_t value, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_bitmap_width(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_free_brightnessgrid(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_clone(void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_load(uint64_t width, uint64_t height, RustBuffer data, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_new(uint64_t width, uint64_t height, RustCallStatus *_Nonnull out_status
);
RustBuffer uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_copy_raw(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_equals(void*_Nonnull ptr, void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_fill(void*_Nonnull ptr, uint8_t value, RustCallStatus *_Nonnull out_status
);
uint8_t uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_get(void*_Nonnull ptr, uint64_t x, uint64_t y, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_height(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_set(void*_Nonnull ptr, uint64_t x, uint64_t y, uint8_t value, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_width(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_free_command(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear(uint64_t offset, void*_Nonnull bitmap, RustBuffer compression, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_and(uint64_t offset, void*_Nonnull bitmap, RustBuffer compression, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_or(uint64_t offset, void*_Nonnull bitmap, RustBuffer compression, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_win(uint64_t offset_x, uint64_t offset_y, void*_Nonnull bitmap, RustBuffer compression, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_xor(uint64_t offset, void*_Nonnull bitmap, RustBuffer compression, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_brightness(uint8_t brightness, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_char_brightness(uint64_t offset_x, uint64_t offset_y, void*_Nonnull grid, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_clear(RustCallStatus *_Nonnull out_status
    
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_clone(void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_cp437_data(uint64_t offset_x, uint64_t offset_y, void*_Nonnull grid, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_fade_out(RustCallStatus *_Nonnull out_status
    
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_command_hard_reset(RustCallStatus *_Nonnull out_status
    
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_command_equals(void*_Nonnull ptr, void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_free_connection(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new(RustBuffer host, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new_fake(RustCallStatus *_Nonnull out_status
    
);
void uniffi_servicepoint_binding_uniffi_fn_method_connection_send(void*_Nonnull ptr, void*_Nonnull command, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_free_cp437grid(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_clone(void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_load(uint64_t width, uint64_t height, RustBuffer data, RustCallStatus *_Nonnull out_status
);
void*_Nonnull uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_new(uint64_t width, uint64_t height, RustCallStatus *_Nonnull out_status
);
RustBuffer uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_copy_raw(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
int8_t uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_equals(void*_Nonnull ptr, void*_Nonnull other, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_fill(void*_Nonnull ptr, uint8_t value, RustCallStatus *_Nonnull out_status
);
uint8_t uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_get(void*_Nonnull ptr, uint64_t x, uint64_t y, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_height(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
void uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_set(void*_Nonnull ptr, uint64_t x, uint64_t y, uint8_t value, RustCallStatus *_Nonnull out_status
);
uint64_t uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_width(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
RustBuffer ffi_servicepoint_binding_uniffi_rustbuffer_alloc(int32_t size, RustCallStatus *_Nonnull out_status
);
RustBuffer ffi_servicepoint_binding_uniffi_rustbuffer_from_bytes(ForeignBytes bytes, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rustbuffer_free(RustBuffer buf, RustCallStatus *_Nonnull out_status
);
RustBuffer ffi_servicepoint_binding_uniffi_rustbuffer_reserve(RustBuffer buf, int32_t additional, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_continuation_callback_set(UniFfiRustFutureContinuation _Nonnull callback
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_u8(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_u8(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_u8(void* _Nonnull handle
);
uint8_t ffi_servicepoint_binding_uniffi_rust_future_complete_u8(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_i8(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_i8(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_i8(void* _Nonnull handle
);
int8_t ffi_servicepoint_binding_uniffi_rust_future_complete_i8(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_u16(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_u16(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_u16(void* _Nonnull handle
);
uint16_t ffi_servicepoint_binding_uniffi_rust_future_complete_u16(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_i16(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_i16(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_i16(void* _Nonnull handle
);
int16_t ffi_servicepoint_binding_uniffi_rust_future_complete_i16(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_u32(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_u32(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_u32(void* _Nonnull handle
);
uint32_t ffi_servicepoint_binding_uniffi_rust_future_complete_u32(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_i32(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_i32(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_i32(void* _Nonnull handle
);
int32_t ffi_servicepoint_binding_uniffi_rust_future_complete_i32(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_u64(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_u64(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_u64(void* _Nonnull handle
);
uint64_t ffi_servicepoint_binding_uniffi_rust_future_complete_u64(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_i64(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_i64(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_i64(void* _Nonnull handle
);
int64_t ffi_servicepoint_binding_uniffi_rust_future_complete_i64(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_f32(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_f32(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_f32(void* _Nonnull handle
);
float ffi_servicepoint_binding_uniffi_rust_future_complete_f32(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_f64(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_f64(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_f64(void* _Nonnull handle
);
double ffi_servicepoint_binding_uniffi_rust_future_complete_f64(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_pointer(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_pointer(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_pointer(void* _Nonnull handle
);
void*_Nonnull ffi_servicepoint_binding_uniffi_rust_future_complete_pointer(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_rust_buffer(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_rust_buffer(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_rust_buffer(void* _Nonnull handle
);
RustBuffer ffi_servicepoint_binding_uniffi_rust_future_complete_rust_buffer(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
void ffi_servicepoint_binding_uniffi_rust_future_poll_void(void* _Nonnull handle, void* _Nonnull uniffi_callback
);
void ffi_servicepoint_binding_uniffi_rust_future_cancel_void(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_free_void(void* _Nonnull handle
);
void ffi_servicepoint_binding_uniffi_rust_future_complete_void(void* _Nonnull handle, RustCallStatus *_Nonnull out_status
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_copy_raw(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_equals(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_fill(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_get(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_len(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_set(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_copy_raw(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_equals(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_fill(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_get(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_height(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_set(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_width(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_copy_raw(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_equals(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_fill(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_get(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_height(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_set(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_width(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_command_equals(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_connection_send(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_copy_raw(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_equals(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_fill(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_get(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_height(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_set(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_width(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_clone(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_load(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_new(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_clone(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_load(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new_max_sized(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_clone(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_load(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_new(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_and(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_or(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_win(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_xor(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_brightness(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_char_brightness(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clone(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_cp437_data(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_fade_out(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_command_hard_reset(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new_fake(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_clone(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_load(void
    
);
uint16_t uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_new(void
    
);
uint32_t ffi_servicepoint_binding_uniffi_uniffi_contract_version(void
    
);

