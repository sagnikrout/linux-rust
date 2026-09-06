//! Automatically rewritten from C to Rust
//! Source: samples/bpf/task_fd_query_kern.c
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

    SEC("kprobe/blk_mq_start_request")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> c_int {
    int bpf_prog1(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("kretprobe/__blk_account_io_done")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut pt_regs) -> c_int {
    int bpf_prog2(struct pt_regs *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
