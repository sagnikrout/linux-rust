//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_task_pt_regs.c
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

//
// The kernel struct pt_regs isn't exported in its entirety to userspace.
// Pass it as an array to task_pt_regs.c
//
    char current_regs[PT_REGS_SIZE] = {};
    char ctx_regs[PT_REGS_SIZE] = {};
    let mut uprobe_res: c_int = 0;
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe(struct pt_regs *ctx)
    {
    struct task_struct *current;
    struct pt_regs *regs;
    current = bpf_get_current_task_btf();
    regs = (struct pt_regs *) bpf_task_pt_regs(current);
    if (bpf_probe_read_kernel(current_regs, PT_REGS_SIZE, regs))
    return 0;
    if (bpf_probe_read_kernel(ctx_regs, PT_REGS_SIZE, ctx))
    return 0;
// Prove that uprobe was run
    uprobe_res = 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
