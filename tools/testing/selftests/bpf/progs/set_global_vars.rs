//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/set_global_vars.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    typedef __s32 s32;
    typedef s32 i32;
    typedef __u8 u8;
    enum Enum { EA1 = 0, EA2 = 11, EA3 = 10 };
    enum Enumu64 {EB1 = 0llu, EB2 = 12llu };
    enum Enums64 { EC1 = 0ll, EC2 = 13ll };
    let mut var_s64: volatile __s64 = -1;
    let mut var_u64: volatile __u64 = 0;
    let mut var_s32: volatile i32 = -1;
    let mut var_u32: volatile __u32 = 0;
    let mut var_s16: volatile __s16 = -1;
    let mut var_u16: volatile __u16 = 0;
    let mut var_s8: volatile __s8 = -1;
    let mut var_u8: volatile u8 = 0;
    let mut var_ea: volatile enum Enum = EA1;
    let mut var_eb: volatile enum Enumu64 = EB1;
    let mut var_ec: volatile enum Enums64 = EC1;
    let mut var_b: volatile bool = false;
    const volatile i32 arr[32];
    const volatile enum Enum enum_arr[32];
    const volatile i32 three_d[47][19][17];
    const volatile i32 *ptr_arr[32];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct {
    pub filler: __u16,
    struct {
    pub filler2: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct2 {
    pub filler: __u16,
    volatile struct {
    pub int:1: const,
    union {
    pub var_u8: [volatile u8; 3],
    pub filler3: volatile __s16,
    pub int:1: const,
    pub mat: [i32; 7][5],
    pub u: },
}

    } struct2[2][4];
    };
    const volatile __u32 stru = 0; /* same prefix as below */
    const volatile struct Struct struct1[3];
    const volatile struct Struct struct11[11][7];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct3 {
    struct {
    pub var_u8_l: u8,
}

    struct {
    struct {
    u8 var_u8_h;
    };
    };
    };
    typedef struct Struct3 Struct3_t;
    union Union {
    __u16 var_u16;
    Struct3_t struct3;
    };
    let mut union1: volatile union Union = {.var_u16 = -1};
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn test_set_globals(ctx: *mut c_void) -> c_int {
    int test_set_globals(void *ctx)
    {
    volatile __s8 a;
    a = var_s64;
    a = var_u64;
    a = var_s32;
    a = var_u32;
    a = var_s16;
    a = var_u16;
    a = var_s8;
    a = var_u8;
    a = var_ea;
    a = var_eb;
    a = var_ec;
    a = var_b;
    a = struct1[2].struct2[1][2].u.var_u8[2];
    a = union1.var_u16;
    a = arr[3];
    a = arr[EA2];
    a = enum_arr[EC2];
    a = three_d[31][7][EA2];
    a = struct1[2].struct2[1][2].u.mat[5][3];
    a = struct11[7][5].struct2[0][1].u.mat[3][0];
    return a;
    }
