//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_rdonly_maps.c
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
// Copyright (c) 2019 Facebook

    const struct {
    unsigned a[4];
//
// if the struct's size is multiple of 16, compiler will put it into
// .rodata.cst16 section, which is not recognized by libbpf; work
// around this by ensuring we don't have 16-aligned struct
//
    char _y;
    } rdonly_values = { .a = {2, 3, 4, 5} };
    struct {
    unsigned did_run;
    unsigned iters;
    unsigned sum;
    } res = {};
    SEC("raw_tracepoint/sys_enter:skip_loop")
#[no_mangle]
pub unsafe extern "C" fn skip_loop(ctx: *mut pt_regs) -> c_int {
    int skip_loop(struct pt_regs *ctx)
    {
// prevent compiler to optimize everything out
    let mut p: *mut unsigned  volatile = (void *)&rdonly_values.a;
    let mut iters: unsigned = 0, sum = 0;
// we should never enter this loop
    while (*p & 1) {
    iters++;
    sum += *p;
    p++;
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    return 0;
    }
    SEC("raw_tracepoint/sys_enter:part_loop")
#[no_mangle]
pub unsafe extern "C" fn part_loop(ctx: *mut pt_regs) -> c_int {
    int part_loop(struct pt_regs *ctx)
    {
// prevent compiler to optimize everything out
    let mut p: *mut unsigned  volatile = (void *)&rdonly_values.a;
    let mut iters: unsigned = 0, sum = 0;
// validate verifier can derive loop termination
    while (*p < 5) {
    iters++;
    sum += *p;
    p++;
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    return 0;
    }
    SEC("raw_tracepoint/sys_enter:full_loop")
#[no_mangle]
pub unsafe extern "C" fn full_loop(ctx: *mut pt_regs) -> c_int {
    int full_loop(struct pt_regs *ctx)
    {
// prevent compiler to optimize everything out
    let mut p: *mut unsigned  volatile = (void *)&rdonly_values.a;
    let mut i: c_int = ARRAY_SIZE(rdonly_values.a);
    let mut iters: unsigned = 0, sum = 0;
// validate verifier can allow full loop as well
    while (i > 0 ) {
    iters++;
    sum += *p;
    p++;
    i--;
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
