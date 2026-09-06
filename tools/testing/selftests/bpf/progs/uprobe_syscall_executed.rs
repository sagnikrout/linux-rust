//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_syscall_executed.c
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

    struct pt_regs regs;
    char _license[] SEC("license") = "GPL";
    let mut executed: c_int = 0;
    int pid;
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test_uprobe) -> c_int {
    int BPF_UPROBE(test_uprobe)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
    SEC("uretprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: test_uretprobe) -> c_int {
    int BPF_URETPROBE(test_uretprobe)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
    SEC("uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_uprobe_multi(ctx: *mut pt_regs) -> c_int {
    int test_uprobe_multi(struct pt_regs *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
    SEC("uretprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_uretprobe_multi(ctx: *mut pt_regs) -> c_int {
    int test_uretprobe_multi(struct pt_regs *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
    SEC("uprobe.session")
#[no_mangle]
pub unsafe extern "C" fn test_uprobe_session(ctx: *mut pt_regs) -> c_int {
    int test_uprobe_session(struct pt_regs *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn test_usdt(ctx: *mut pt_regs) -> c_int {
    int test_usdt(struct pt_regs *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    executed++;
    return 0;
    }
