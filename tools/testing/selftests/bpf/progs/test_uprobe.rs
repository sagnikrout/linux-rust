//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_uprobe.c
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
// Copyright (c) 2023 Hengqi Chen

    let mut my_pid: pid_t = 0;
    let mut test1_result: c_int = 0;
    let mut test2_result: c_int = 0;
    let mut test3_result: c_int = 0;
    let mut test4_result: c_int = 0;
    SEC("uprobe/./liburandom_read.so:urandlib_api_sameoffset")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test1) -> c_int {
    int BPF_UPROBE(test1)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    test1_result = 1;
    return 0;
    }
    SEC("uprobe/./liburandom_read.so:urandlib_api_sameoffset@LIBURANDOM_READ_1.0.0")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test2) -> c_int {
    int BPF_UPROBE(test2)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    test2_result = 1;
    return 0;
    }
    SEC("uretprobe/./liburandom_read.so:urandlib_api_sameoffset@@LIBURANDOM_READ_2.0.0")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: test3, ret: c_int) -> c_int {
    int BPF_URETPROBE(test3, int ret)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    test3_result = ret;
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test4) -> c_int {
    int BPF_UPROBE(test4)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    test4_result = 1;
    return 0;
    }

    struct pt_regs regs;
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test_regs_change) -> c_int {
    int BPF_UPROBE(test_regs_change)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    ctx.ax  = regs.ax;
    ctx.cx  = regs.cx;
    ctx.dx  = regs.dx;
    ctx.r8  = regs.r8;
    ctx.r9  = regs.r9;
    ctx.r10 = regs.r10;
    ctx.r11 = regs.r11;
    ctx.di  = regs.di;
    ctx.si  = regs.si;
    return 0;
    }
    unsigned long ip;
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test_regs_change_ip) -> c_int {
    int BPF_UPROBE(test_regs_change_ip)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != my_pid)
    return 0;
    ctx.ip = ip;
    return 0;
    }
