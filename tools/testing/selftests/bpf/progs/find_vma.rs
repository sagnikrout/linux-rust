//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/find_vma.c
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
// Copyright (c) 2021 Facebook

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_ctx {
    pub dummy: c_int,
}

pub const VM_EXEC: c_uint = 0x00000004;
pub const DNAME_INLINE_LEN: c_int = 32;
    let mut target_pid: pid_t = 0;
    char d_iname[DNAME_INLINE_LEN] = {0};
    let mut found_vm_exec: __u32 = 0;
    let mut addr: __u64 = 0;
    let mut find_zero_ret: c_int = -1;
    let mut find_addr_ret: c_int = -1;
    static long check_vma(struct task_struct *task, struct vm_area_struct *vma,
    struct callback_ctx *data)
    {
    if (vma.vm_file)
    bpf_probe_read_kernel_str(d_iname, DNAME_INLINE_LEN - 1,
    vma.vm_file.f_path.dentry.d_shortname.string);
// check for VM_EXEC
    if (vma.vm_flags & VM_EXEC)
    found_vm_exec = 1;
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_getpid() -> c_int {
    int handle_getpid(void)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    let mut data: callback_ctx = {};
    if (task.pid != target_pid)
    return 0;
    find_addr_ret = bpf_find_vma(task, addr, check_vma, &data, 0);
// this should return -ENOENT
    find_zero_ret = bpf_find_vma(task, 0, check_vma, &data, 0);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn handle_pe() -> c_int {
    int handle_pe(void)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    let mut data: callback_ctx = {};
    if (task.pid != target_pid)
    return 0;
    find_addr_ret = bpf_find_vma(task, addr, check_vma, &data, 0);
// In NMI, this should return -EBUSY, as the previous call is using
// the irq_work.
//
    find_zero_ret = bpf_find_vma(task, 0, check_vma, &data, 0);
    return 0;
    }
