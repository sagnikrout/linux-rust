//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_testmod.c
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
    SEC("raw_tp/sys_enter")
    __success
#[no_mangle]
pub unsafe extern "C" fn iter_next_trusted(ctx: *const c_void) -> c_int {
    int iter_next_trusted(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct bpf_iter_task_vma vma_it;
    struct vm_area_struct *vma_ptr;
    bpf_iter_task_vma_new(&vma_it, cur_task, 0);
    vma_ptr = bpf_iter_task_vma_next(&vma_it);
    if (vma_ptr == core::ptr::null_mut())
    goto out;
    bpf_kfunc_trusted_vma_test(vma_ptr);
    out:
    bpf_iter_task_vma_destroy(&vma_it);
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn iter_next_trusted_or_null(ctx: *const c_void) -> c_int {
    int iter_next_trusted_or_null(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct bpf_iter_task_vma vma_it;
    struct vm_area_struct *vma_ptr;
    bpf_iter_task_vma_new(&vma_it, cur_task, 0);
    vma_ptr = bpf_iter_task_vma_next(&vma_it);
    bpf_kfunc_trusted_vma_test(vma_ptr);
    bpf_iter_task_vma_destroy(&vma_it);
    return 0;
    }
    SEC("raw_tp/sys_enter")
    __success
#[no_mangle]
pub unsafe extern "C" fn iter_next_rcu(ctx: *const c_void) -> c_int {
    int iter_next_rcu(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct bpf_iter_task task_it;
    struct task_struct *task_ptr;
    bpf_iter_task_new(&task_it, cur_task, 0);
    task_ptr = bpf_iter_task_next(&task_it);
    if (task_ptr == core::ptr::null_mut())
    goto out;
    bpf_kfunc_rcu_task_test(task_ptr);
    out:
    bpf_iter_task_destroy(&task_it);
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn iter_next_rcu_or_null(ctx: *const c_void) -> c_int {
    int iter_next_rcu_or_null(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct bpf_iter_task task_it;
    struct task_struct *task_ptr;
    bpf_iter_task_new(&task_it, cur_task, 0);
    task_ptr = bpf_iter_task_next(&task_it);
    bpf_kfunc_rcu_task_test(task_ptr);
    bpf_iter_task_destroy(&task_it);
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __msg(trusted": "R1 must be referenced or) -> __failure {
    __failure __msg("R1 must be referenced or trusted")
#[no_mangle]
pub unsafe extern "C" fn iter_next_rcu_not_trusted(ctx: *const c_void) -> c_int {
    int iter_next_rcu_not_trusted(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct bpf_iter_task task_it;
    struct task_struct *task_ptr;
    bpf_iter_task_new(&task_it, cur_task, 0);
    task_ptr = bpf_iter_task_next(&task_it);
    if (task_ptr == core::ptr::null_mut())
    goto out;
    bpf_kfunc_trusted_task_test(task_ptr);
    out:
    bpf_iter_task_destroy(&task_it);
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __msg(rdonly_mem": "R1 cannot write into) -> __failure {
    __failure __msg("R1 cannot write into rdonly_mem")
// Message should not be 'R1 cannot write into rdonly_trusted_mem'
#[no_mangle]
pub unsafe extern "C" fn iter_next_ptr_mem_not_trusted(ctx: *const c_void) -> c_int {
    int iter_next_ptr_mem_not_trusted(const void *ctx)
    {
    struct bpf_iter_num num_it;
    int *num_ptr;
    bpf_iter_num_new(&num_it, 0, 10);
    num_ptr = bpf_iter_num_next(&num_it);
    if (num_ptr == core::ptr::null_mut())
    goto out;
    bpf_kfunc_trusted_num_test(num_ptr);
    out:
    bpf_iter_num_destroy(&num_it);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(protection": "kernel func bpf_kfunc_ret_rcu_test requires RCU critical section) -> __failure {
    __failure __msg("kernel func bpf_kfunc_ret_rcu_test requires RCU critical section protection")
#[no_mangle]
pub unsafe extern "C" fn iter_ret_rcu_test_protected(ctx: *const c_void) -> c_int {
    int iter_ret_rcu_test_protected(const void *ctx)
    {
    struct task_struct *p;
    p = bpf_kfunc_ret_rcu_test();
    return p.pid;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=": "R1 type=rcu_ptr_or_null_) -> __failure {
    __failure __msg("R1 type=rcu_ptr_or_null_ expected=")
#[no_mangle]
pub unsafe extern "C" fn iter_ret_rcu_test_type(ctx: *const c_void) -> c_int {
    int iter_ret_rcu_test_type(const void *ctx)
    {
    struct task_struct *p;
    bpf_rcu_read_lock();
    p = bpf_kfunc_ret_rcu_test();
    bpf_this_cpu_ptr(p);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(protection": "kernel func bpf_kfunc_ret_rcu_test_nostruct requires RCU critical section) -> __failure {
    __failure __msg("kernel func bpf_kfunc_ret_rcu_test_nostruct requires RCU critical section protection")
#[no_mangle]
pub unsafe extern "C" fn iter_ret_rcu_test_protected_nostruct(ctx: *const c_void) -> c_int {
    int iter_ret_rcu_test_protected_nostruct(const void *ctx)
    {
    void *p;
    p = bpf_kfunc_ret_rcu_test_nostruct(4);
    return *(int *)p;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=": "R1 type=rdonly_rcu_mem_or_null) -> __failure {
    __failure __msg("R1 type=rdonly_rcu_mem_or_null expected=")
#[no_mangle]
pub unsafe extern "C" fn iter_ret_rcu_test_type_nostruct(ctx: *const c_void) -> c_int {
    int iter_ret_rcu_test_type_nostruct(const void *ctx)
    {
    void *p;
    bpf_rcu_read_lock();
    p = bpf_kfunc_ret_rcu_test_nostruct(4);
    bpf_this_cpu_ptr(p);
    bpf_rcu_read_unlock();
    return 0;
    }
