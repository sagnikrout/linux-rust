//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_assoc.c
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

    char _license[] SEC("license") = "GPL";
    int test_pid;
// Programs associated with st_ops_map_a
pub const MAP_A_MAGIC: c_int = 1234;
    int test_err_a;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1_a, args: *mut st_ops_args) -> c_int {
    int BPF_PROG(test_1_a, struct st_ops_args *args)
    {
    return MAP_A_MAGIC;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sys_enter_prog_a, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(sys_enter_prog_a, struct pt_regs *regs, long id)
    {
    let mut args: st_ops_args = {};
    struct task_struct *task;
    int ret;
    task = bpf_get_current_task_btf();
    if (!test_pid || task.pid != test_pid)
    return 0;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_A_MAGIC)
    test_err_a++;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_prog_a(ctx: *mut c_void) -> c_int {
    int syscall_prog_a(void *ctx)
    {
    let mut args: st_ops_args = {};
    int ret;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_A_MAGIC)
    test_err_a++;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_multi_st_ops st_ops_map_a = {
    .test_1 = (void *)test_1_a,
    };
// Programs associated with st_ops_map_b
pub const MAP_B_MAGIC: c_int = 5678;
    int test_err_b;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1_b, args: *mut st_ops_args) -> c_int {
    int BPF_PROG(test_1_b, struct st_ops_args *args)
    {
    return MAP_B_MAGIC;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sys_enter_prog_b, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(sys_enter_prog_b, struct pt_regs *regs, long id)
    {
    let mut args: st_ops_args = {};
    struct task_struct *task;
    int ret;
    task = bpf_get_current_task_btf();
    if (!test_pid || task.pid != test_pid)
    return 0;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_B_MAGIC)
    test_err_b++;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_prog_b(ctx: *mut c_void) -> c_int {
    int syscall_prog_b(void *ctx)
    {
    let mut args: st_ops_args = {};
    int ret;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_B_MAGIC)
    test_err_b++;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_multi_st_ops st_ops_map_b = {
    .test_1 = (void *)test_1_b,
    };
