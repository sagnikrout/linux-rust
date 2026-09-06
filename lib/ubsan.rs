//! Automatically rewritten from C Header to Rust Module
//! Source: lib/ubsan.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// ABI defined by Clang's UBSAN enum SanitizerHandler:
// https://github.com/llvm/llvm-project/blob/release/16.x/clang/lib/CodeGen/CodeGenFunction.h#L113
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ubsan_checks {
    ubsan_add_overflow,
    ubsan_builtin_unreachable,
    ubsan_cfi_check_fail,
    ubsan_divrem_overflow,
    ubsan_dynamic_type_cache_miss,
    ubsan_float_cast_overflow,
    ubsan_function_type_mismatch,
    ubsan_implicit_conversion,
    ubsan_invalid_builtin,
    ubsan_invalid_objc_cast,
    ubsan_load_invalid_value,
    ubsan_missing_return,
    ubsan_mul_overflow,
    ubsan_negate_overflow,
    ubsan_nullability_arg,
    ubsan_nullability_return,
    ubsan_nonnull_arg,
    ubsan_nonnull_return,
    ubsan_out_of_bounds,
    ubsan_pointer_overflow,
    ubsan_shift_out_of_bounds,
    ubsan_sub_overflow,
    ubsan_type_mismatch,
    ubsan_alignment_assumption,
    ubsan_vla_bound_not_positive,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_descriptor {
    pub type_kind: u16,
    pub type_info: u16,
    pub type_name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct source_location {
    pub file_name: *const c_char,
    pub reported: c_ulong,
    pub line: u32,
    pub column: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct overflow_data {
    pub location: source_location,
    pub type: *mut type_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct implicit_conversion_data {
    pub location: source_location,
    pub from_type: *mut type_descriptor,
    pub to_type: *mut type_descriptor,
    pub type_check_kind: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_mismatch_data {
    pub location: source_location,
    pub type: *mut type_descriptor,
    pub alignment: c_ulong,
    pub type_check_kind: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_mismatch_data_v1 {
    pub location: source_location,
    pub type: *mut type_descriptor,
    pub log_alignment: c_uchar,
    pub type_check_kind: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_mismatch_data_common {
    pub location: *mut source_location,
    pub type: *mut type_descriptor,
    pub alignment: c_ulong,
    pub type_check_kind: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nonnull_arg_data {
    pub location: source_location,
    pub attr_location: source_location,
    pub arg_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct out_of_bounds_data {
    pub location: source_location,
    pub array_type: *mut type_descriptor,
    pub index_type: *mut type_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shift_out_of_bounds_data {
    pub location: source_location,
    pub lhs_type: *mut type_descriptor,
    pub rhs_type: *mut type_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unreachable_data {
    pub location: source_location,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct invalid_value_data {
    pub location: source_location,
    pub type: *mut type_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alignment_assumption_data {
    pub location: source_location,
    pub assumption_location: source_location,
    pub type: *mut type_descriptor,
}

pub type s_max = __int128;
pub type u_max = unsigned __int128;

pub type s_max = i64;
pub type u_max = u64;

//
// When generating Runtime Calls, Clang doesn't respect the -mregparm=3
// option used on i386: https://github.com/llvm/llvm-project/issues/89670
// Fix this for earlier Clang versions by forcing the calling convention
// to use non-register arguments.
//

extern "C" {
    pub fn __ubsan_handle_add_overflow(data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_sub_overflow(data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_mul_overflow(data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_negate_overflow(_data: *mut c_void, old_val: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_divrem_overflow(_data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_implicit_conversion(_data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_type_mismatch(data: *mut type_mismatch_data, ptr: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_type_mismatch_v1(_data: *mut c_void, ptr: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_out_of_bounds(_data: *mut c_void, index: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_shift_out_of_bounds(_data: *mut c_void, lhs: *mut c_void, rhs: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_builtin_unreachable(_data: *mut c_void) -> void ubsan_linkage;
}
extern "C" {
    pub fn __ubsan_handle_load_invalid_value(_data: *mut c_void, val: *mut c_void) -> void ubsan_linkage;
}
