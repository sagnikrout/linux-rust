//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_autosize.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
// fields of exactly the same size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct___samesize {
    pub ptr: *mut c_void,
    pub val1: c_ulonglong,
    pub val2: c_uint,
    pub val3: c_ushort,
    pub val4: c_uchar,
    pub __attribute((preserve_access_index)): },
// unsigned fields that have to be downsized by libbpf
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct___downsize {
    pub ptr: *mut c_void,
    pub val1: c_ulong,
    pub val2: c_ulong,
    pub val3: c_ulong,
    pub val4: c_ulong,
// total sz: 40
    pub __attribute__((preserve_access_index)): },
// fields with signed integers of wrong size, should be rejected
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct___signed {
    pub ptr: *mut c_void,
    pub val1: c_long,
    pub val2: c_long,
    pub val3: c_long,
    pub val4: c_long,
    pub __attribute((preserve_access_index)): },
// real layout and sizes according to test's (32-bit) BTF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct___real {
    pub /: *mut *mut *mut unsigned int ptr; / can't use `void `, it is always 8 byte in BPF target,
    pub val2: c_uint,
    pub val1: c_ulonglong,
    pub val3: c_ushort,
    pub val4: c_uchar,
    pub _pad: c_uchar,
// total sz: 20
}

    struct test_struct___real input = {
    .ptr = 0x01020304,
    .val1 = 0x1020304050607080,
    .val2 = 0x0a0b0c0d,
    .val3 = 0xfeed,
    .val4 = 0xb9,
    ._pad = 0xff, /* make sure no accidental zeros are present */
    };
    let mut ptr_samesized: c_ulonglong = 0;
    let mut val1_samesized: c_ulonglong = 0;
    let mut val2_samesized: c_ulonglong = 0;
    let mut val3_samesized: c_ulonglong = 0;
    let mut val4_samesized: c_ulonglong = 0;
    let mut output_samesized: test_struct___real = {};
    let mut ptr_downsized: c_ulonglong = 0;
    let mut val1_downsized: c_ulonglong = 0;
    let mut val2_downsized: c_ulonglong = 0;
    let mut val3_downsized: c_ulonglong = 0;
    let mut val4_downsized: c_ulonglong = 0;
    let mut output_downsized: test_struct___real = {};
    let mut ptr_probed: c_ulonglong = 0;
    let mut val1_probed: c_ulonglong = 0;
    let mut val2_probed: c_ulonglong = 0;
    let mut val3_probed: c_ulonglong = 0;
    let mut val4_probed: c_ulonglong = 0;
    let mut ptr_signed: c_ulonglong = 0;
    let mut val1_signed: c_ulonglong = 0;
    let mut val2_signed: c_ulonglong = 0;
    let mut val3_signed: c_ulonglong = 0;
    let mut val4_signed: c_ulonglong = 0;
    let mut output_signed: test_struct___real = {};
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn handle_samesize(ctx: *mut c_void) -> c_int {
    int handle_samesize(void *ctx)
    {
    struct test_struct___samesize *in = (void *)&input;
    struct test_struct___samesize *out = (void *)&output_samesized;
    ptr_samesized = (unsigned long long)in.ptr;
    val1_samesized = in.val1;
    val2_samesized = in.val2;
    val3_samesized = in.val3;
    val4_samesized = in.val4;
    out.ptr = in.ptr;
    out.val1 = in.val1;
    out.val2 = in.val2;
    out.val3 = in.val3;
    out.val4 = in.val4;
    return 0;
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn handle_downsize(ctx: *mut c_void) -> c_int {
    int handle_downsize(void *ctx)
    {
    struct test_struct___downsize *in = (void *)&input;
    struct test_struct___downsize *out = (void *)&output_downsized;
    ptr_downsized = (unsigned long long)in.ptr;
    val1_downsized = in.val1;
    val2_downsized = in.val2;
    val3_downsized = in.val3;
    val4_downsized = in.val4;
    out.ptr = in.ptr;
    out.val1 = in.val1;
    out.val2 = in.val2;
    out.val3 = in.val3;
    out.val4 = in.val4;
    return 0;
    }

// Prevent "subtraction from stack pointer prohibited" */ \
    volatile long __off = sizeof(*dst) - (sz); \
    bpf_core_read((char *)(dst) + __off, sz, src); \
    })

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_probed(ctx: *mut c_void) -> c_int {
    int handle_probed(void *ctx)
    {
    struct test_struct___downsize *in = (void *)&input;
    __u64 tmp;
    tmp = 0;
    bpf_core_read_int(&tmp, bpf_core_field_size(in.ptr), &in.ptr);
    ptr_probed = tmp;
    tmp = 0;
    bpf_core_read_int(&tmp, bpf_core_field_size(in.val1), &in.val1);
    val1_probed = tmp;
    tmp = 0;
    bpf_core_read_int(&tmp, bpf_core_field_size(in.val2), &in.val2);
    val2_probed = tmp;
    tmp = 0;
    bpf_core_read_int(&tmp, bpf_core_field_size(in.val3), &in.val3);
    val3_probed = tmp;
    tmp = 0;
    bpf_core_read_int(&tmp, bpf_core_field_size(in.val4), &in.val4);
    val4_probed = tmp;
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_signed(ctx: *mut c_void) -> c_int {
    int handle_signed(void *ctx)
    {
    struct test_struct___signed *in = (void *)&input;
    struct test_struct___signed *out = (void *)&output_signed;
    val2_signed = in.val2;
    val3_signed = in.val3;
    val4_signed = in.val4;
    out.val2= in.val2;
    out.val3= in.val3;
    out.val4= in.val4;
    return 0;
    }
