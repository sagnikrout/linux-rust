//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf_dump_test_case_syntax.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// BTF-to-C dumper test for majority of C syntax quirks.
//
// Copyright (c) 2019 Facebook
//
// ----- START-EXPECTED-OUTPUT -----
    enum e1 {
    A = 0,
    B = 1,
    };
    enum e2 {
    C = 100,
    D = 4294967295,
    E = 0,
    };
    typedef enum e2 e2_t;
    typedef enum {
    F = 0,
    G = 1,
    H = 2,
    } e3_t;
// ----- START-EXPECTED-OUTPUT -----
//
// enum e_byte {
// EBYTE_1 = 0,
// EBYTE_2 = 1,
// } __attribute__((mode(byte)));
//
// ----- END-EXPECTED-OUTPUT -----
    enum e_byte {
    EBYTE_1,
    EBYTE_2,
    } __attribute__((mode(byte)));
// ----- START-EXPECTED-OUTPUT -----
//
// enum e_word {
// EWORD_1 = 0LL,
// EWORD_2 = 1LL,
// } __attribute__((mode(word)));
//
// ----- END-EXPECTED-OUTPUT -----
    enum e_word {
    EWORD_1,
    EWORD_2,
    } __attribute__((mode(word))); /* force to use 8-byte backing for this enum */
// ----- START-EXPECTED-OUTPUT -----
    enum e_big {
    EBIG_1 = 1000000000000ULL,
    };
    typedef int int_t;
    typedef volatile const int * volatile const crazy_ptr_t;
    typedef int *****we_need_to_go_deeper_ptr_t;
    typedef volatile const we_need_to_go_deeper_ptr_t * restrict * volatile * const * restrict volatile * restrict const * volatile const * restrict volatile const how_about_this_ptr_t;
    typedef int *ptr_arr_t[10];
    typedef void (*fn_ptr1_t)(int);
    typedef void (*printf_fn_t)(const char *, ...);
// ------ END-EXPECTED-OUTPUT ------
//
// While previous function pointers are pretty trivial (C-syntax-level
// trivial), the following are deciphered here for future generations:
//
// - `fn_ptr2_t`: function, taking anonymous struct as a first arg and pointer
// to a function, that takes int and returns int, as a second arg; returning
// a pointer to a const pointer to a char. Equivalent to:
// typedef struct { int a; } s_t;
// typedef int (*fn_t)(int);
// typedef char * const * (*fn_ptr2_t)(s_t, fn_t);
//
// - `fn_complex_t`: pointer to a function returning struct and accepting
// union and struct. All structs and enum are anonymous and defined inline.
//
// - `signal_t: pointer to a function accepting a pointer to a function as an
// argument and returning pointer to a function as a result. Sane equivalent:
// typedef void (*signal_handler_t)(int);
// typedef signal_handler_t (*signal_ptr_t)(int, signal_handler_t);
//
// - fn_ptr_arr1_t: array of pointers to a function accepting pointer to
// a pointer to an int and returning pointer to a char. Easy.
//
// - fn_ptr_arr2_t: array of const pointers to a function taking no arguments
// and returning a const pointer to a function, that takes pointer to a
// `int -> char *` function and returns pointer to a char. Equivalent:
// typedef char * (*fn_input_t)(int);
// typedef char * (*fn_output_outer_t)(fn_input_t);
// typedef const fn_output_outer_t (* fn_output_inner_t)(void);
// typedef const fn_output_inner_t fn_ptr_arr2_t[5];
//
// ----- START-EXPECTED-OUTPUT -----
    typedef char * const * (*fn_ptr2_t)(struct {
    int a;
    }, int (*)(int));
    typedef struct {
    int a;
    void (*b)(int, struct {
    int c;
    }, union {
    char d;
    int e[5];
    });
    } (*fn_complex_t)(union {
    void *f;
    char g[16];
    }, struct {
    int h;
    });
    typedef void (* (*signal_t)(int, void (*)(int)))(int);
    typedef char * (*fn_ptr_arr1_t[10])(int **);
    typedef char * (* (* const fn_ptr_arr2_t[5])(void))(char * (*)(int));
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_w_typedefs {
    pub a: int_t,
    pub b: crazy_ptr_t,
    pub c: we_need_to_go_deeper_ptr_t,
    pub d: how_about_this_ptr_t,
    pub e: ptr_arr_t,
    pub f: fn_ptr1_t,
    pub g: printf_fn_t,
    pub h: fn_ptr2_t,
    pub i: fn_complex_t,
    pub j: signal_t,
    pub k: fn_ptr_arr1_t,
    pub l: fn_ptr_arr2_t,
}

    typedef struct {
    int x;
    int y;
    int z;
    } anon_struct_t;
    struct struct_fwd;
    typedef struct struct_fwd struct_fwd_t;
    typedef struct struct_fwd *struct_fwd_ptr_t;
    union union_fwd;
    typedef union union_fwd union_fwd_t;
    typedef union union_fwd *union_fwd_ptr_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_empty {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_simple {
    pub a: c_int,
    pub b: c_char,
    pub p: *const int_t,
    pub s: struct_empty,
    pub e: enum e2,
    enum {
    ANON_VAL1 = 1,
    ANON_VAL2 = 2,
    pub f: },
    pub arr1: [c_int; 13],
    pub arr2: [enum e2; 5],
}

    union union_empty {};
    union union_simple {
    void *ptr;
    int num;
    int_t num2;
    union union_empty u;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_in_struct {
    pub simple: struct_simple,
    pub also_simple: union union_simple,
    struct {
    pub a: c_int,
    pub not_so_hard_as_well: },
    union {
    pub b: c_int,
    pub c: c_int,
    pub anon_union_is_good: },
    struct {
    pub d: c_int,
    pub e: c_int,
}

    union {
    int f;
    int g;
    };
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_in_array {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_in_array_typed {
    pub struct_in_array_t: [typedef struct struct_in_array_typed; 2],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_with_embedded_stuff {
    pub a: c_int,
    struct {
    pub b: c_int,
    struct {
    pub c: *mut struct_with_embedded_stuff,
    pub d: *const c_char,
    pub e: },
    union {
    pub f: volatile long,
    pub g: *mut *mut void  restrict,
}

    };
    union {
    const int_t *h;
    void (*i)(char, int, void *);
    } j;
    enum {
    K = 100,
    L = 200,
    } m;
    char n[16];
    struct {
    char o;
    int p;
    void (*q)(int);
    } r[5];
    struct struct_in_struct s[10];
    int t[11];
    struct struct_in_array (*u)[2];
    struct_in_array_t *v;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct float_struct {
    pub f: float,
    pub d: *const double,
    pub ld: *mut volatile long double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_struct {
    pub _1: enum e1,
    pub _2: enum e2,
    pub _2_1: e2_t,
    pub _2_2: e3_t,
    pub _100: enum e_byte,
    pub _101: enum e_word,
    pub _102: enum e_big,
    pub _3: struct_w_typedefs,
    pub _7: anon_struct_t,
    pub _8: *mut struct_fwd,
    pub _9: *mut struct_fwd_t,
    pub _10: struct_fwd_ptr_t,
    pub _11: *mut union union_fwd,
    pub _12: *mut union_fwd_t,
    pub _13: union_fwd_ptr_t,
    pub _14: struct_with_embedded_stuff,
    pub _15: float_struct,
}

// ------ END-EXPECTED-OUTPUT ------
#[no_mangle]
pub unsafe extern "C" fn f(s: *mut root_struct) -> c_int {
    int f(struct root_struct *s)
    {
    return 0;
    }
