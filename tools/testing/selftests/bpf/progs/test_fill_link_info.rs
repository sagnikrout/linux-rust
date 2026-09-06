//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_fill_link_info.c
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
// Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com>

    extern bool CONFIG_X86_KERNEL_IBT __kconfig __weak;
    extern bool CONFIG_PPC_FTRACE_OUT_OF_LINE __kconfig __weak;
    extern bool CONFIG_KPROBES_ON_FTRACE __kconfig __weak;
    extern bool CONFIG_PPC64 __kconfig __weak;
// This function is here to have CONFIG_X86_KERNEL_IBT,
// CONFIG_PPC_FTRACE_OUT_OF_LINE, CONFIG_KPROBES_ON_FTRACE,
// CONFIG_PPC64 used and added to object BTF.
//
#[no_mangle]
pub unsafe extern "C" fn unused() -> c_int {
    int unused(void)
    {
    return CONFIG_X86_KERNEL_IBT ||
    CONFIG_PPC_FTRACE_OUT_OF_LINE ||
    CONFIG_KPROBES_ON_FTRACE ||
    CONFIG_PPC64 ? 0 : 1;
    }
    SEC("kprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: kprobe_run) -> c_int {
    int BPF_PROG(kprobe_run)
    {
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: uprobe_run) -> c_int {
    int BPF_PROG(uprobe_run)
    {
    return 0;
    }
    SEC("tracepoint")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tp_run) -> c_int {
    int BPF_PROG(tp_run)
    {
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn event_run(ctx: *mut c_void) -> c_int {
    int event_run(void *ctx)
    {
    return 0;
    }
    SEC("kprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: kmulti_run) -> c_int {
    int BPF_PROG(kmulti_run)
    {
    return 0;
    }
    SEC("uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: umulti_run) -> c_int {
    int BPF_PROG(umulti_run)
    {
    return 0;
    }
    SEC("fentry.multi")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tmulti_run) -> c_int {
    int BPF_PROG(tmulti_run)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
