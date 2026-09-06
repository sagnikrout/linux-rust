//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_attach_probe_manual.c
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
// Copyright (c) 2017 Facebook

    let mut kprobe_res: c_int = 0;
    let mut kretprobe_res: c_int = 0;
    let mut uprobe_res: c_int = 0;
    let mut uretprobe_res: c_int = 0;
    let mut uprobe_byname_res: c_int = 0;
    void *user_ptr = 0;
    SEC("kprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_kprobe(ctx: *mut pt_regs) -> c_int {
    int handle_kprobe(struct pt_regs *ctx)
    {
    kprobe_res = 1;
    return 0;
    }
    SEC("kretprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_kretprobe(ctx: *mut pt_regs) -> c_int {
    int handle_kretprobe(struct pt_regs *ctx)
    {
    kretprobe_res = 2;
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe(struct pt_regs *ctx)
    {
    uprobe_res = 3;
    return 0;
    }
    SEC("uretprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe(struct pt_regs *ctx)
    {
    uretprobe_res = 4;
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_byname(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_byname(struct pt_regs *ctx)
    {
    uprobe_byname_res = 5;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
