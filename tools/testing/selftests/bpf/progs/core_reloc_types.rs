//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/core_reloc_types.h
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


//
// KERNEL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_kernel_output {
    pub valid: [c_int; 10],
    pub comm: [c_char; sizeof("test_progs")],
    pub comm_len: c_int,
    pub local_task_struct_matches: bool,
}

//
// MODULE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_module_output {
    pub len: c_longlong,
    pub off: c_longlong,
    pub read_ctx_sz: c_int,
    pub read_ctx_exists: bool,
    pub buf_exists: bool,
    pub len_exists: bool,
    pub off_exists: bool,
// we have test_progs[-flavor], so cut flavor part
    pub comm: [c_char; sizeof("test_progs")],
    pub comm_len: c_int,
}

//
// FLAVORS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_flavors {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

// this is not a flavor, as it doesn't have triple underscore
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_flavors__err_wrong_name {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

//
// NESTING
//
// original set up, used to record relocations in BPF program
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting_substruct {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union core_reloc_nesting_subunion {
    pub b: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting {
    pub a: core_reloc_nesting_substruct,
    pub a: },
    pub b: core_reloc_nesting_subunion,
    pub b: },
}

// inlined anonymous struct/union instead of named structs in original
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___anon_embed {
    pub __just_for_padding: c_int,
    pub a: c_int,
    pub a: },
    pub a: },
    pub b: c_int,
    pub b: },
    pub b: },
}

// different mix of nested structs/unions than in original
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___struct_union_mixup {
    pub __a: c_int,
    pub __a: c_int,
    pub __a: c_char,
    pub a: c_int,
    pub a: },
    pub a: },
    pub __b: c_int,
    pub __b: c_int,
    pub __b: c_char,
    pub b: c_int,
    pub b: },
    pub b: },
}

// extra anon structs/unions, but still valid a.a.a and b.b.b accessors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___extra_nesting {
    pub __padding: c_int,
    pub a: c_int,
    pub a: },
}

// three flavors of same struct with different structure but same layout for
// a.a.a and b.b.b, thus successfully resolved and relocatable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___dup_compat_types {
    pub __just_for_padding: c_char,
// 3 more bytes of padding
    pub /: *mut *mut int a; / offset 4,
    pub a: },
    pub a: },
    pub __more_padding: c_longlong,
    pub /: *mut *mut int b; / offset 16,
    pub b: },
    pub b: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___dup_compat_types__2 {
    pub __aligned_padding: c_int,
    pub __trickier_noop: [c_int; 0],
    pub __some_more_noops: [c_char; 0],
    pub /: *mut *mut int a; / offset 4,
    pub a: },
    pub a: },
    pub __more_padding: c_int,
    pub __critical_padding: c_int,
    pub /: *mut *mut int b; / offset 16,
    pub b: },
    pub __does_not_matter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___dup_compat_types__3 {
    pub __correct_padding: [c_char; 4],
    pub /: *mut *mut int a; / offset 4,
    pub a: },
    pub a: },
// 8 byte padding due to next struct's alignment
    pub b: c_int,
    pub b: },
    pub __attribute__((aligned(16))): } b,
}

// b.b.b field is missing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_missing_field {
    pub a: c_int,
    pub a: },
    pub a: },
    pub x: c_int,
    pub b: },
    pub b: },
}

// b.b.b field is an array of integers instead of plain int
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_array_field {
    pub a: c_int,
    pub a: },
    pub a: },
    pub b: [c_int; 1],
    pub b: },
    pub b: },
}

// middle b container is missing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_missing_container {
    pub a: c_int,
    pub a: },
    pub a: },
    pub x: c_int,
    pub b: },
}

// middle b container is referenced through pointer instead of being embedded
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_nonstruct_container {
    pub a: c_int,
    pub a: },
    pub a: },
    pub b: c_int,
    pub b: *mut },
    pub b: },
}

// middle b container is an array of structs instead of plain struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_array_container {
    pub a: c_int,
    pub a: },
    pub a: },
    pub b: c_int,
    pub b: [}; 1],
    pub b: },
}

// two flavors of same struct with incompatible layout for b.b.b
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_dup_incompat_types__1 {
    pub /: *mut *mut int a; / offset 0,
    pub a: },
    pub a: },
    pub /: *mut *mut int b; / offset 4,
    pub b: },
    pub b: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_dup_incompat_types__2 {
    pub /: *mut *mut int a; / offset 0,
    pub a: },
    pub a: },
    pub __extra_padding: c_int,
    pub /: *mut *mut int b; / offset 8 (!),
    pub b: },
    pub b: },
}

// two flavors of same struct having one of a.a.a and b.b.b, but not both
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_partial_match_dups__a {
    pub a: c_int,
    pub a: },
    pub a: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_partial_match_dups__b {
    pub b: c_int,
    pub b: },
    pub b: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting___err_too_deep {
    pub a: c_int,
    pub a: },
    pub a: },
// 65 levels of nestedness for b.b.b
// this one is one too much
    pub b: c_int,
}

//
// ARRAYS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays_output {
    pub a2: c_int,
    pub a3: c_int,
    pub b123: c_char,
    pub c1c: c_int,
    pub d00d: c_int,
    pub f10c: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays_substruct {
    pub c: c_int,
    pub d: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

// bigger array dimensions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___diff_arr_dim {
    pub a: [c_int; 7],
    pub b: [c_char; 3][4][5],
    pub c: [core_reloc_arrays_substruct; 4],
    pub d: [core_reloc_arrays_substruct; 2][3],
    pub f: [core_reloc_arrays_substruct; 1][3],
}

// different size of array's value (struct)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___diff_arr_val_sz {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub __padding1: c_int,
    pub c: c_int,
    pub __padding2: c_int,
    pub c: [}; 3],
    pub __padding1: c_int,
    pub d: c_int,
    pub __padding2: c_int,
    pub d: [}; 1][2],
    pub __padding1: c_int,
    pub c: c_int,
    pub __padding2: c_int,
    pub f: [}; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___equiv_zero_sz_arr {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
// equivalent to flexible array
    pub f: [core_reloc_arrays_substruct; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___fixed_arr {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
// not a flexible array anymore, but within access bounds
    pub f: [core_reloc_arrays_substruct; 1][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_too_small {
    pub /: *mut *mut int a[2]; / this one is too small,
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_too_shallow {
    pub a: [c_int; 5],
    pub /: *mut *mut char b[2][3]; / this one lacks one dimension,
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_non_array {
    pub /: *mut *mut int a; / not an array,
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_wrong_val_type {
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub /: *mut *mut int c[3]; / value is not a struct,
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_bad_zero_sz_arr {
// zero-sized array, but not at the end
    pub f: [core_reloc_arrays_substruct; 0][2],
    pub a: [c_int; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays___err_bad_signed_arr_elem_sz {
// int -> short (signed!): not supported case
    pub a: [c_short; 5],
    pub b: [c_char; 2][3][4],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [core_reloc_arrays_substruct; 1][2],
    pub f: [core_reloc_arrays_substruct; ][2],
}

//
// PRIMITIVES
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_reloc_primitives_enum {
    A = 0,
    B = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives {
    pub a: c_char,
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub __bpf_aligned: *mut *mut void d,
    pub __bpf_aligned: *const *const *const int (f)(char ),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___diff_enum_def {
    pub a: c_char,
    pub b: c_int,
    pub __bpf_aligned: *mut *mut void d,
    pub __bpf_aligned: *const *const *const int (f)(char ),
    pub /: *mut *mut } c __bpf_aligned; / inline enum def with differing set of values,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___diff_func_proto {
    pub /: *mut *mut *mut void (f)(int) __bpf_aligned; / incompatible function prototype,
    pub __bpf_aligned: *mut *mut void d,
    pub __bpf_aligned: core_reloc_primitives_c,
    pub b: c_int,
    pub a: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___diff_ptr_type {
    pub /: *const *const *const char  d __bpf_aligned; / different pointee type + modifiers,
    pub __bpf_aligned: char a,
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub __bpf_aligned: *const *const *const int (f)(char ),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___err_non_enum {
    pub a: [c_char; 1],
    pub b: c_int,
    pub /: *mut *mut int c; / int instead of enum,
    pub __bpf_aligned: *mut *mut void d,
    pub __bpf_aligned: *const *const *const int (f)(char ),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___err_non_int {
    pub a: [c_char; 1],
    pub /: *mut *mut *mut int b __bpf_aligned; / ptr instead of int,
    pub __bpf_aligned: core_reloc_primitives_c,
    pub __bpf_aligned: *mut *mut void d,
    pub __bpf_aligned: *const *const *const int (f)(char ),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_primitives___err_non_ptr {
    pub a: [c_char; 1],
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub /: *mut *mut int d; / int instead of ptr,
    pub __bpf_aligned: *const *const *const int (f)(char ),
}

//
// MODS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods_output {
    pub h: int a, b, c, d, e, f, g,,
}

pub type int_t = c_int;
pub type __bpf_aligned = *const char char_ptr_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods_substruct {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods {
    pub a: c_int,
    pub b: int_t,
    pub __bpf_aligned: *mut *mut char c,
    pub d: char_ptr_t,
    pub __bpf_aligned: int e[3],
    pub f: arr_t,
    pub g: core_reloc_mods_substruct,
    pub h: core_reloc_mods_substruct_t,
}

// a/b, c/d, e/f, and g/h pairs are swapped
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods___mod_swap {
    pub b: c_int,
    pub a: int_t,
    pub __bpf_aligned: *mut *mut char d,
    pub c: char_ptr_t,
    pub __bpf_aligned: int f[3],
    pub e: arr_t,
    pub y: c_int,
    pub x: c_int,
    pub h: },
    pub g: core_reloc_mods_substruct_t,
}

pub type int1_t = c_int;
pub type int2_t = int1_t;
pub type int3_t = int2_t;
pub type arr2_t = arr1_t;
pub type arr3_t = arr2_t;
pub type arr4_t = arr3_t;
pub type __bpf_aligned = *const char  volatile fancy_char_ptr_t;
pub type core_reloc_mods_substruct_tt = core_reloc_mods_substruct_t;
// we need more typedefs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_mods___typedefs {
    pub g: core_reloc_mods_substruct_tt,
    pub h: core_reloc_mods_substruct_tt,
    pub f: arr4_t,
    pub e: arr4_t,
    pub d: fancy_char_ptr_t,
    pub c: fancy_char_ptr_t,
    pub __bpf_aligned: int3_t b,
    pub a: int3_t,
}

//
// PTR_AS_ARR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ptr_as_arr {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ptr_as_arr___diff_sz {
    pub /: *mut *mut int :32; / padding,
    pub __some_more_padding: c_char,
    pub a: c_int,
}

//
// INTS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ints {
    pub u8_field: u8,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

// signed/unsigned types swap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ints___reverse_sign {
    pub u8_field: i8,
    pub s8_field: u8,
    pub u16_field: i16,
    pub s16_field: u16,
    pub u32_field: i32,
    pub s32_field: u32,
    pub u64_field: i64,
    pub s64_field: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_ints___bool {
    pub /: *mut *mut bool u8_field; / bool instead of uint8,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

//
// MISC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc_output {
    pub c: int a, b,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc___a {
    pub a1: c_int,
    pub a2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc___b {
    pub b1: c_int,
    pub b2: c_int,
}

// this one extends core_reloc_misc_extensible struct from BPF prog
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_misc_extensible {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub d: c_int,
}

//
// FIELD EXISTENCE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_existence_output {
    pub a_exists: c_int,
    pub a_value: c_int,
    pub b_exists: c_int,
    pub b_value: c_int,
    pub c_exists: c_int,
    pub c_value: c_int,
    pub arr_exists: c_int,
    pub arr_value: c_int,
    pub s_exists: c_int,
    pub s_value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_existence {
    pub a: c_int,
    pub b: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_existence___minimal {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_existence___wrong_field_defs {
    pub a: *mut c_void,
    pub b: [c_int; 1],
    pub c: struct{ int x; },
    pub arr: c_int,
    pub s: c_int,
}

//
// BITFIELDS
//
// bitfield read results, all as plain integers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields_output {
    pub ub1: i64,
    pub ub2: i64,
    pub ub7: i64,
    pub sb4: i64,
    pub sb20: i64,
    pub u32: i64,
    pub s32: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields {
// unsigned bitfields
    pub 1: uint8_t ub1:,
    pub 2: uint8_t ub2:,
    pub 7: uint32_t ub7:,
// signed bitfields
    pub 4: int8_t sb4:,
    pub 20: int32_t sb20:,
// non-bitfields
    pub u32: u32,
    pub s32: i32,
}

// different bit sizes (both up and down)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields___bit_sz_change {
// unsigned bitfields
    pub /: *mut *mut uint16_t ub1: 3; / 1 -> 3,
    pub /: *mut *mut uint32_t ub2: 20; / 2 -> 20,
    pub /: *mut *mut uint8_t ub7: 1; / 7 -> 1,
// signed bitfields
    pub /: *mut *mut int8_t sb4: 1; / 4 -> 1,
    pub /: *mut *mut int32_t sb20: 30; / 20 -> 30,
// non-bitfields
    pub /: *mut *mut uint16_t u32; / 32 -> 16,
    pub /: *mut *mut int64_t s32 __bpf_aligned; / 32 -> 64,
}

// turn bitfield into non-bitfield and vice versa
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields___bitfield_vs_int {
    pub /: *mut *mut uint64_t ub1; / 3 -> 64 non-bitfield,
    pub /: *mut *mut uint8_t ub2; / 20 -> 8 non-bitfield,
    pub /: *mut *mut int64_t ub7 __bpf_aligned; / 7 -> 64 non-bitfield signed,
    pub /: *mut *mut int64_t sb4 __bpf_aligned; / 4 -> 64 non-bitfield signed,
    pub /: *mut *mut uint64_t sb20 __bpf_aligned; / 20 -> 16 non-bitfield unsigned,
    pub /: *mut *mut int32_t u32: 20; / 32 non-bitfield -> 20 bitfield,
    pub /: *mut *mut uint64_t s32: 60 __bpf_aligned; / 32 non-bitfield -> 60 bitfield,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields___just_big_enough {
    pub 4: uint64_t ub1:,
    pub /: *mut *mut uint64_t ub2: 60; / packed tightly,
    pub ub7: u32,
    pub sb4: u32,
    pub sb20: u32,
    pub u32: u32,
    pub s32: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields___err_too_big_bitfield {
    pub 4: uint64_t ub1:,
    pub /: *mut *mut uint64_t ub2: 61; / packed tightly,
    pub ub7: u32,
    pub sb4: u32,
    pub sb20: u32,
    pub u32: u32,
    pub s32: u32,
    pub __attribute__((packed)): },
//
// SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size_output {
    pub int_sz: c_int,
    pub int_off: c_int,
    pub struct_sz: c_int,
    pub struct_off: c_int,
    pub union_sz: c_int,
    pub union_off: c_int,
    pub arr_sz: c_int,
    pub arr_off: c_int,
    pub arr_elem_sz: c_int,
    pub arr_elem_off: c_int,
    pub ptr_sz: c_int,
    pub ptr_off: c_int,
    pub enum_sz: c_int,
    pub enum_off: c_int,
    pub float_sz: c_int,
    pub float_off: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size {
    pub int_field: c_int,
    pub struct_field: { int x; },
    pub union_field: { int x; },
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: { VALUE = 123 },
    pub float_field: float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size___diff_sz {
    pub int_field: u64,
    pub struct_field: { int x; int y; int z; },
    pub union_field: { int x; char bla[123]; },
    pub arr_field: [c_char; 10],
    pub ptr_field: *mut c_void,
    pub enum_field: { OTHER_VALUE = 0xFFFFFFFFFFFFFFFF },
    pub float_field: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size___diff_offs {
    pub float_field: float,
    pub enum_field: { YET_OTHER_VALUE = 123 },
    pub ptr_field: *mut c_void,
    pub arr_field: [c_int; 4],
    pub union_field: { int x; },
    pub struct_field: { int x; },
    pub int_field: c_int,
}

// Error case of two candidates with the fields (int_field) at the same
// offset, but with differing final relocation values: size 4 vs size 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size___err_ambiguous1 {
// int at offset 0
    pub int_field: c_int,
    pub struct_field: { int x; },
    pub union_field: { int x; },
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: { VALUE___1 = 123 },
    pub float_field: float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_size___err_ambiguous2 {
// char at offset 0
    pub int_field: c_char,
    pub struct_field: { int x; },
    pub union_field: { int x; },
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: { VALUE___2 = 123 },
    pub float_field: float,
}

//
// TYPE EXISTENCE, MATCH & SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based_output {
    pub struct_exists: bool,
    pub complex_struct_exists: bool,
    pub union_exists: bool,
    pub enum_exists: bool,
    pub typedef_named_struct_exists: bool,
    pub typedef_anon_struct_exists: bool,
    pub typedef_struct_ptr_exists: bool,
    pub typedef_int_exists: bool,
    pub typedef_enum_exists: bool,
    pub typedef_void_ptr_exists: bool,
    pub typedef_restrict_ptr_exists: bool,
    pub typedef_func_proto_exists: bool,
    pub typedef_arr_exists: bool,
    pub struct_matches: bool,
    pub complex_struct_matches: bool,
    pub union_matches: bool,
    pub enum_matches: bool,
    pub typedef_named_struct_matches: bool,
    pub typedef_anon_struct_matches: bool,
    pub typedef_struct_ptr_matches: bool,
    pub typedef_int_matches: bool,
    pub typedef_enum_matches: bool,
    pub typedef_void_ptr_matches: bool,
    pub typedef_restrict_ptr_matches: bool,
    pub typedef_func_proto_matches: bool,
    pub typedef_arr_matches: bool,
    pub struct_sz: c_int,
    pub union_sz: c_int,
    pub enum_sz: c_int,
    pub typedef_named_struct_sz: c_int,
    pub typedef_anon_struct_sz: c_int,
    pub typedef_struct_ptr_sz: c_int,
    pub typedef_int_sz: c_int,
    pub typedef_enum_sz: c_int,
    pub typedef_void_ptr_sz: c_int,
    pub typedef_func_proto_sz: c_int,
    pub typedef_arr_sz: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_struct {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_complex_struct {
    pub a: *mut *mut a_ restrict,
    pub b: *mut c_void,
    pub x: },
    pub y: volatile long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union a_union {
    pub y: c_int,
    pub z: c_int,
}

pub type named_struct_typedef = a_struct;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
}

pub type int_typedef = c_int;
pub type restrict_ptr_typedef = *mut int restrict;
extern "C" {
    pub fn int(_arg: *mut func_proto_typedef)(long) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based {
    pub f1: a_struct,
    pub f2: a_complex_struct,
    pub f3: a_union,
    pub f4: an_enum,
    pub f5: named_struct_typedef,
    pub f6: anon_struct_typedef,
    pub f7: struct_ptr_typedef,
    pub f8: int_typedef,
    pub f9: enum_typedef,
    pub f10: void_ptr_typedef,
    pub f11: restrict_ptr_typedef,
    pub f12: func_proto_typedef,
    pub f13: arr_typedef,
}

// no types in target
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based___all_missing {
}

// different member orders, enum variant values, signedness, etc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_struct___diff {
    pub x: c_int,
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_complex_struct___diff {
    pub a: *mut a_struct___forward,
    pub b: *mut c_void,
    pub x: },
    pub y: volatile long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union a_union___diff {
    pub z: c_int,
    pub y: c_int,
}

pub type named_struct_typedef___diff = a_struct___diff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum an_enum___diff {
    AN_ENUM_VAL2___diff = 0,
    AN_ENUM_VAL1___diff = 42,
    AN_ENUM_VAL3___diff = 1,
}

pub type int_typedef___diff = c_uint;
extern "C" {
    pub fn int_typedef___diff(_arg: *mut func_proto_typedef___diff)(long) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based___diff {
    pub f1: a_struct___diff,
    pub f2: a_complex_struct___diff,
    pub f3: a_union___diff,
    pub f4: an_enum___diff,
    pub f5: named_struct_typedef___diff,
    pub f6: anon_struct_typedef___diff,
    pub f7: struct_ptr_typedef___diff,
    pub f8: int_typedef___diff,
    pub f9: enum_typedef___diff,
    pub f10: void_ptr_typedef___diff,
    pub f11: func_proto_typedef___diff,
    pub f12: arr_typedef___diff,
}

// different type sizes, extra modifiers, anon vs named enums, etc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_struct___diff_sz {
    pub x: c_long,
    pub y: c_int,
    pub z: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union a_union___diff_sz {
    pub yy: c_char,
    pub zz: c_char,
}

pub type named_struct_typedef___diff_sz = a_struct___diff_sz;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum an_enum___diff_sz {
    AN_ENUM_VAL1___diff_sz = 0x123412341234,
    AN_ENUM_VAL2___diff_sz = 2,
}

pub type int_typedef___diff_sz = c_ulong;
pub type enum_typedef___diff_sz = an_enum___diff_sz;
pub type void_ptr_typedef___diff_sz = *const void  const;
extern "C" {
    pub fn int_typedef___diff_sz(_arg: *mut func_proto_typedef___diff_sz)(char) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based___diff_sz {
    pub f1: a_struct___diff_sz,
    pub f2: a_union___diff_sz,
    pub f3: an_enum___diff_sz,
    pub f4: named_struct_typedef___diff_sz,
    pub f5: anon_struct_typedef___diff_sz,
    pub f6: struct_ptr_typedef___diff_sz,
    pub f7: int_typedef___diff_sz,
    pub f8: enum_typedef___diff_sz,
    pub f9: void_ptr_typedef___diff_sz,
    pub f10: func_proto_typedef___diff_sz,
    pub f11: arr_typedef___diff_sz,
}

// incompatibilities between target and local types
#[repr(C)]
#[derive(Copy, Clone)]
pub union a_struct___incompat {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a_union___incompat {
    pub y: c_int,
    pub z: c_int,
}

// typedef to union, not to struct
pub type named_struct_typedef___incompat = a_struct___incompat;
// typedef to void pointer, instead of struct
// extra pointer indirection
// typedef of a struct with int, instead of int
// typedef to func_proto, instead of enum
extern "C" {
    pub fn int(_arg: *mut enum_typedef___incompat)(void) -> typedef;
}
// pointer to char instead of void
// void return type instead of int
extern "C" {
    pub fn void(_arg: *mut func_proto_typedef___incompat)(long) -> typedef;
}
// multi-dimensional array instead of a single-dimensional
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based___incompat {
    pub f1: a_struct___incompat,
    pub f2: a_union___incompat,
// the only valid one is enum, to check that something still succeeds
    pub f3: an_enum,
    pub f4: named_struct_typedef___incompat,
    pub f5: anon_struct_typedef___incompat,
    pub f6: struct_ptr_typedef___incompat,
    pub f7: int_typedef___incompat,
    pub f8: enum_typedef___incompat,
    pub f9: void_ptr_typedef___incompat,
    pub f10: func_proto_typedef___incompat,
    pub f11: arr_typedef___incompat,
}

// func_proto with incompatible signature
extern "C" {
    pub fn void(_arg: *mut func_proto_typedef___fn_wrong_ret1)(long) -> typedef;
}
extern "C" {
    pub fn int_struct_typedef(_arg: *mut func_proto_typedef___fn_wrong_ret3)(long) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut func_proto_typedef___fn_wrong_arg)(void) -> typedef;
}
extern "C" {
    pub fn int(_arg: *mut func_proto_typedef___fn_wrong_arg_cnt1)(long, _arg: c_long) -> typedef;
}
extern "C" {
    pub fn int(_arg: *mut func_proto_typedef___fn_wrong_arg_cnt2)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_based___fn_wrong_args {
// one valid type to make sure relos still work
    pub f1: a_struct,
    pub f2: func_proto_typedef___fn_wrong_ret1,
    pub f3: func_proto_typedef___fn_wrong_ret2,
    pub f4: func_proto_typedef___fn_wrong_ret3,
    pub f5: func_proto_typedef___fn_wrong_arg,
    pub f6: func_proto_typedef___fn_wrong_arg_cnt1,
    pub f7: func_proto_typedef___fn_wrong_arg_cnt2,
}

//
// TYPE ID MAPPING (LOCAL AND TARGET)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_id_output {
    pub local_anon_struct: c_int,
    pub local_anon_union: c_int,
    pub local_anon_enum: c_int,
    pub local_anon_func_proto_ptr: c_int,
    pub local_anon_void_ptr: c_int,
    pub local_anon_arr: c_int,
    pub local_struct: c_int,
    pub local_union: c_int,
    pub local_enum: c_int,
    pub local_int: c_int,
    pub local_struct_typedef: c_int,
    pub local_func_proto_typedef: c_int,
    pub local_arr_typedef: c_int,
    pub targ_struct: c_int,
    pub targ_union: c_int,
    pub targ_enum: c_int,
    pub targ_int: c_int,
    pub targ_struct_typedef: c_int,
    pub targ_func_proto_typedef: c_int,
    pub targ_arr_typedef: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_id {
    pub f1: a_struct,
    pub f2: a_union,
    pub f3: an_enum,
    pub f4: named_struct_typedef,
    pub f5: func_proto_typedef,
    pub f6: arr_typedef,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_type_id___missing_targets {
// nothing
}

//
// ENUMERATOR VALUE EXISTENCE AND VALUE RELOCATION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval_output {
    pub named_val1_exists: bool,
    pub named_val2_exists: bool,
    pub named_val3_exists: bool,
    pub anon_val1_exists: bool,
    pub anon_val2_exists: bool,
    pub anon_val3_exists: bool,
    pub named_val1: c_int,
    pub named_val2: c_int,
    pub anon_val1: c_int,
    pub anon_val2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val_output {
    pub unsigned_val1_exists: bool,
    pub unsigned_val2_exists: bool,
    pub unsigned_val3_exists: bool,
    pub signed_val1_exists: bool,
    pub signed_val2_exists: bool,
    pub signed_val3_exists: bool,
    pub unsigned_val1: c_long,
    pub unsigned_val2: c_long,
    pub signed_val1: c_long,
    pub signed_val2: c_long,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_enum {
    NAMED_ENUM_VAL1 = 1,
    NAMED_ENUM_VAL2 = 2,
    NAMED_ENUM_VAL3 = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval {
    pub f1: named_enum,
    pub f2: anon_enum,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_unsigned_enum64 {
    UNSIGNED_ENUM64_VAL1 = 0x1ffffffffULL,
    UNSIGNED_ENUM64_VAL2 = 0x2,
    UNSIGNED_ENUM64_VAL3 = 0x3ffffffffULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_signed_enum64 {
    SIGNED_ENUM64_VAL1 = 0x1ffffffffLL,
    SIGNED_ENUM64_VAL2 = -2,
    SIGNED_ENUM64_VAL3 = 0x3ffffffffLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val {
    pub f1: named_unsigned_enum64,
    pub f2: named_signed_enum64,
}

// differing enumerator values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_enum___diff {
    NAMED_ENUM_VAL1___diff = 101,
    NAMED_ENUM_VAL2___diff = 202,
    NAMED_ENUM_VAL3___diff = 303,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval___diff {
    pub f1: named_enum___diff,
    pub f2: anon_enum___diff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_unsigned_enum64___diff {
    UNSIGNED_ENUM64_VAL1___diff = 0x101ffffffffULL,
    UNSIGNED_ENUM64_VAL2___diff = 0x202ffffffffULL,
    UNSIGNED_ENUM64_VAL3___diff = 0x303ffffffffULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_signed_enum64___diff {
    SIGNED_ENUM64_VAL1___diff = -101,
    SIGNED_ENUM64_VAL2___diff = -202,
    SIGNED_ENUM64_VAL3___diff = -303,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val___diff {
    pub f1: named_unsigned_enum64___diff,
    pub f2: named_signed_enum64___diff,
}

// missing (optional) third enum value
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_enum___val3_missing {
    NAMED_ENUM_VAL1___val3_missing = 111,
    NAMED_ENUM_VAL2___val3_missing = 222,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval___val3_missing {
    pub f1: named_enum___val3_missing,
    pub f2: anon_enum___val3_missing,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_unsigned_enum64___val3_missing {
    UNSIGNED_ENUM64_VAL1___val3_missing = 0x111ffffffffULL,
    UNSIGNED_ENUM64_VAL2___val3_missing = 0x222,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_signed_enum64___val3_missing {
    SIGNED_ENUM64_VAL1___val3_missing = 0x111ffffffffLL,
    SIGNED_ENUM64_VAL2___val3_missing = -222,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val___val3_missing {
    pub f1: named_unsigned_enum64___val3_missing,
    pub f2: named_signed_enum64___val3_missing,
}

// missing (mandatory) second enum value, should fail
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_enum___err_missing {
    NAMED_ENUM_VAL1___err_missing = 1,
    NAMED_ENUM_VAL3___err_missing = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enumval___err_missing {
    pub f1: named_enum___err_missing,
    pub f2: anon_enum___err_missing,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_unsigned_enum64___err_missing {
    UNSIGNED_ENUM64_VAL1___err_missing = 0x1ffffffffULL,
    UNSIGNED_ENUM64_VAL3___err_missing = 0x3ffffffffULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum named_signed_enum64___err_missing {
    SIGNED_ENUM64_VAL1___err_missing = 0x1ffffffffLL,
    SIGNED_ENUM64_VAL3___err_missing = -3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_enum64val___err_missing {
    pub f1: named_unsigned_enum64___err_missing,
    pub f2: named_signed_enum64___err_missing,
}
