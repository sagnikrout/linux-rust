//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cpumask_failure.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kptr_nested_array_2 {
    pub mask: *mut *mut bpf_cpumask __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kptr_nested_array_1 {
// Make btf_parse_fields() in map_create() return -E2BIG
    pub 1]: kptr_nested_array_2 d_2[CPUMASK_KPTR_FIELDS_MAX +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kptr_nested_array {
    pub d_1: kptr_nested_array_1,
}

    private(MASK_NESTED) static struct kptr_nested_array global_mask_nested_arr;
// Prototype for all of the program trace events below:
//
// TRACE_EVENT(task_newtask,
// TP_PROTO(struct task_struct *p, u64 clone_flags)
//
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_alloc_no_release, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_alloc_no_release, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *cpumask;
    cpumask = create_cpumask();
    __sink(cpumask);
// cpumask is never released.
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "NULL pointer passed to trusted) -> __failure {
    __failure __msg("core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_alloc_double_release, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_alloc_double_release, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *cpumask;
    cpumask = create_cpumask();
// cpumask is released twice.
    bpf_cpumask_release(cpumask);
    bpf_cpumask_release(cpumask);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(referenced": "must be) -> __failure {
    __failure __msg("must be referenced")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_acquire_wrong_cpumask, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_acquire_wrong_cpumask, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *cpumask;
// Can't acquire a non-struct bpf_cpumask.
    cpumask = bpf_cpumask_acquire((struct bpf_cpumask *)task.cpus_ptr);
    __sink(cpumask);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_cpumask": "bpf_cpumask_set_cpu R2 expected pointer to STRUCT) -> __failure {
    __failure __msg("bpf_cpumask_set_cpu R2 expected pointer to STRUCT bpf_cpumask")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_mutate_cpumask, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_mutate_cpumask, struct task_struct *task, u64 clone_flags)
    {
// Can't set the CPU of a non-struct bpf_cpumask.
    bpf_cpumask_set_cpu(0, (struct bpf_cpumask *)task.cpus_ptr);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_insert_remove_no_release, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_insert_remove_no_release, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *cpumask;
    struct __cpumask_map_value *v;
    cpumask = create_cpumask();
    if (!cpumask)
    return 0;
    if (cpumask_map_insert(cpumask))
    return 0;
    v = cpumask_map_value_lookup();
    if (!v)
    return 0;
    cpumask = bpf_kptr_xchg(&v.cpumask, core::ptr::null_mut());
// cpumask is never released.
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "NULL pointer passed to trusted) -> __failure {
    __failure __msg("core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_cpumask_null, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_cpumask_null, struct task_struct *task, u64 clone_flags)
    {
// NULL passed to kfunc.
    bpf_cpumask_empty(core::ptr::null_mut());
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R2 must be a rcu) -> __failure {
    __failure __msg("R2 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_global_mask_out_of_rcu) -> c_int {
    int BPF_PROG(test_global_mask_out_of_rcu)
    {
    struct bpf_cpumask *local, *prev;
    local = create_cpumask();
    if (!local)
    return 0;
    prev = bpf_kptr_xchg(&global_mask, local);
    if (prev) {
    bpf_cpumask_release(prev);
    err = 3;
    return 0;
    }
//
// Use a sleepable program so explicit RCU is the only source of RCU
// protection.
//
    bpf_rcu_read_lock();
    local = global_mask;
    if (!local) {
    err = 4;
    bpf_rcu_read_unlock();
    return 0;
    }
    bpf_rcu_read_unlock();
// RCU region is exited before calling KF_RCU kfunc.
    bpf_cpumask_test_cpu(0, (const struct cpumask *)local);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "NULL pointer passed to trusted) -> __failure {
    __failure __msg("core::ptr::null_mut() pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_global_mask_no_null_check, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_global_mask_no_null_check, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *local, *prev;
    local = create_cpumask();
    if (!local)
    return 0;
    prev = bpf_kptr_xchg(&global_mask, local);
    if (prev) {
    bpf_cpumask_release(prev);
    err = 3;
    return 0;
    }
    bpf_rcu_read_lock();
    local = global_mask;
// No NULL check is performed on global cpumask kptr.
    bpf_cpumask_test_cpu(0, (const struct cpumask *)local);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to helper) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to helper R2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_global_mask_rcu_no_null_check, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_global_mask_rcu_no_null_check, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *prev, *curr;
    curr = bpf_cpumask_create();
    if (!curr)
    return 0;
    prev = bpf_kptr_xchg(&global_mask, curr);
    if (prev)
    bpf_cpumask_release(prev);
    bpf_rcu_read_lock();
    curr = global_mask;
// PTR_TO_BTF_ID | PTR_MAYBE_NULL | MEM_RCU passed to bpf_kptr_xchg()
    prev = bpf_kptr_xchg(&global_mask, curr);
    bpf_rcu_read_unlock();
    if (prev)
    bpf_cpumask_release(prev);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(kptr": "has no valid) -> __failure {
    __failure __msg("has no valid kptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_invalid_nested_array, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_invalid_nested_array, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *local, *prev;
    local = create_cpumask();
    if (!local)
    return 0;
    prev = bpf_kptr_xchg(&global_mask_nested_arr.d_1.d_2[CPUMASK_KPTR_FIELDS_MAX].mask, local);
    if (prev) {
    bpf_cpumask_release(prev);
    err = 3;
    return 0;
    }
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp": "type=scalar) -> __failure {
    __failure __msg("type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_populate_invalid_destination, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_populate_invalid_destination, struct task_struct *task, u64 clone_flags)
    {
    struct bpf_cpumask *invalid = (struct bpf_cpumask *)0x123456;
    u64 bits;
    int ret;
    ret = bpf_cpumask_populate(invalid, &bits, sizeof(bits));
    if (!ret)
    err = 2;
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "leads to invalid memory) -> __failure {
    __failure __msg("leads to invalid memory access")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_populate_invalid_source, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_populate_invalid_source, struct task_struct *task, u64 clone_flags)
    {
    void *garbage = (void *)0x123456;
    struct bpf_cpumask *local;
    int ret;
    local = create_cpumask();
    if (!local) {
    err = 1;
    return 0;
    }
    ret = bpf_cpumask_populate(local, garbage, 8);
    if (!ret)
    err = 2;
    bpf_cpumask_release(local);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(cpumask": "expected pointer to STRUCT bpf_cpumask but R1 has a pointer to STRUCT) -> __failure {
    __failure __msg("expected pointer to STRUCT bpf_cpumask but R1 has a pointer to STRUCT cpumask")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_populate_borrowed_destination, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_populate_borrowed_destination, struct task_struct *task, u64 clone_flags)
    {
    u64 bits;
    int ret;
//
// task->cpus_ptr is a borrowed, read-only struct cpumask *, not an
// owned struct bpf_cpumask *. The verifier must reject it as a
// writable destination for bpf_cpumask_populate().
//
    ret = bpf_cpumask_populate((struct bpf_cpumask *)task.cpus_ptr, &bits, sizeof(bits));
    if (!ret)
    err = 2;
    return 0;
    }
