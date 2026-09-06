//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_extern.c
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

// non-existing BPF helper, to test dead code elimination
    static int (*bpf_missing_helper)(const void *arg1, int arg2) = (void *) 999;
    extern int LINUX_KERNEL_VERSION __kconfig;
    extern int LINUX_UNKNOWN_VIRTUAL_EXTERN __kconfig __weak;
    extern bool CONFIG_BPF_SYSCALL __kconfig; /* strong */
    extern enum libbpf_tristate CONFIG_TRISTATE __kconfig __weak;
    extern bool CONFIG_BOOL __kconfig __weak;
    extern char CONFIG_CHAR __kconfig __weak;
    extern uint16_t CONFIG_USHORT __kconfig __weak;
    extern int CONFIG_INT __kconfig __weak;
    extern uint64_t CONFIG_ULONG __kconfig __weak;
    extern const char CONFIG_STR[8] __kconfig __weak;
    extern uint64_t CONFIG_MISSING __kconfig __weak;
    let mut kern_ver: u64 = -1;
    let mut unkn_virt_val: u64 = -1;
    let mut bpf_syscall: u64 = -1;
    let mut tristate_val: u64 = -1;
    let mut bool_val: u64 = -1;
    let mut char_val: u64 = -1;
    let mut ushort_val: u64 = -1;
    let mut int_val: u64 = -1;
    let mut ulong_val: u64 = -1;
    char str_val[8] = {-1, -1, -1, -1, -1, -1, -1, -1};
    let mut missing_val: u64 = -1;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut pt_regs) -> c_int {
    int handle_sys_enter(struct pt_regs *ctx)
    {
    int i;
    kern_ver = LINUX_KERNEL_VERSION;
    unkn_virt_val = LINUX_UNKNOWN_VIRTUAL_EXTERN;
    bpf_syscall = CONFIG_BPF_SYSCALL;
    tristate_val = CONFIG_TRISTATE;
    bool_val = CONFIG_BOOL;
    char_val = CONFIG_CHAR;
    ushort_val = CONFIG_USHORT;
    int_val = CONFIG_INT;
    ulong_val = CONFIG_ULONG;
    for (i = 0; i < sizeof(CONFIG_STR); i++) {
    str_val[i] = CONFIG_STR[i];
    }
    if (CONFIG_MISSING)
// invalid, but dead code - never executed
    missing_val = bpf_missing_helper(ctx, 123);
    else
    missing_val = 0xDEADC0DE;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
