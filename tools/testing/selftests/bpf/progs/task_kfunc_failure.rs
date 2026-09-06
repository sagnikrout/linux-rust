//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_kfunc_failure.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// Prototype for all of the program trace events below:
//
// TRACE_EVENT(task_newtask,
// TP_PROTO(struct task_struct *p, u64 clone_flags)
//
    static struct __tasks_kfunc_map_value *insert_lookup_task(struct task_struct *task)
    {
    int status;
    status = tasks_kfunc_map_insert(task);
    if (status)
    return core::ptr::null_mut();
    return tasks_kfunc_map_value_lookup(task);
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_untrusted, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_untrusted, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    struct __tasks_kfunc_map_value *v;
    v = insert_lookup_task(task);
    if (!v)
    return 0;
// Can't invoke bpf_task_acquire() on an untrusted pointer.
    acquired = bpf_task_acquire(v.task);
    if (!acquired)
    return 0;
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(task_struct": "R1 is fp expected STRUCT) -> __failure {
    __failure __msg("R1 is fp expected STRUCT task_struct")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_fp, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_fp, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired, *stack_task = (struct task_struct *)&clone_flags;
// Can't invoke bpf_task_acquire() on a random frame pointer.
    acquired = bpf_task_acquire((struct task_struct *)&stack_task);
    if (!acquired)
    return 0;
    bpf_task_release(acquired);
    return 0;
    }
    SEC("kretprobe/free_task")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_task_acquire is not) -> __failure {
    __failure __msg("calling kernel function bpf_task_acquire is not allowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_unsafe_kretprobe, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_unsafe_kretprobe, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
// Can't call bpf_task_acquire() or bpf_task_release() in an untrusted prog.
    acquired = bpf_task_acquire(task);
    if (!acquired)
    return 0;
    bpf_task_release(acquired);
    return 0;
    }
    SEC("kretprobe/free_task")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_task_acquire is not) -> __failure {
    __failure __msg("calling kernel function bpf_task_acquire is not allowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_unsafe_kretprobe_rcu, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_unsafe_kretprobe_rcu, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    bpf_rcu_read_lock();
    if (!task) {
    bpf_rcu_read_unlock();
    return 0;
    }
// Can't call bpf_task_acquire() or bpf_task_release() in an untrusted prog.
    acquired = bpf_task_acquire(task);
    if (acquired)
    bpf_task_release(acquired);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_null, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_null, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
// Can't invoke bpf_task_acquire() on a NULL pointer.
    acquired = bpf_task_acquire(core::ptr::null_mut());
    if (!acquired)
    return 0;
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_unreleased, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_unreleased, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    acquired = bpf_task_acquire(task);
// Acquired task is never released.
    __sink(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_xchg_unreleased, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_xchg_unreleased, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *kptr;
    struct __tasks_kfunc_map_value *v;
    v = insert_lookup_task(task);
    if (!v)
    return 0;
    kptr = bpf_kptr_xchg(&v.task, core::ptr::null_mut());
    if (!kptr)
    return 0;
// Kptr retrieved from map is never released.
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_release_no_null_check, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_acquire_release_no_null_check, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    acquired = bpf_task_acquire(task);
// Can't invoke bpf_task_release() on an acquired task without a NULL check.
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_release_untrusted, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_release_untrusted, struct task_struct *task, u64 clone_flags)
    {
    struct __tasks_kfunc_map_value *v;
    v = insert_lookup_task(task);
    if (!v)
    return 0;
// Can't invoke bpf_task_release() on an untrusted pointer.
    bpf_task_release(v.task);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_release_fp, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_release_fp, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired = (struct task_struct *)&clone_flags;
// Cannot release random frame pointer.
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_release_null, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_release_null, struct task_struct *task, u64 clone_flags)
    {
    struct __tasks_kfunc_map_value local, *v;
    long status;
    struct task_struct *acquired, *old;
    s32 pid;
    status = bpf_probe_read_kernel(&pid, sizeof(pid), &task.pid);
    if (status)
    return 0;
    local.task = core::ptr::null_mut();
    status = bpf_map_update_elem(&__tasks_kfunc_map, &pid, &local, BPF_NOEXIST);
    if (status)
    return status;
    v = bpf_map_lookup_elem(&__tasks_kfunc_map, &pid);
    if (!v)
    return -ENOENT;
    acquired = bpf_task_acquire(task);
    if (!acquired)
    return -EEXIST;
    old = bpf_kptr_xchg(&v.task, acquired);
// old cannot be passed to bpf_task_release() without a NULL check.
    bpf_task_release(old);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_release_unacquired, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_release_unacquired, struct task_struct *task, u64 clone_flags)
    {
// Cannot release trusted task pointer which was not acquired.
    bpf_task_release(task);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(fields": "bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe) -> __failure {
    __failure __msg("bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe fields")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_obj_drop_with_kptr, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_obj_drop_with_kptr, struct task_struct *task, u64 clone_flags)
    {
    struct __tasks_kfunc_map_value *local;
    local = bpf_obj_new(typeof(*local));
    if (!local)
    return 0;
    bpf_obj_drop(local);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(fields": "bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe) -> __failure {
    __failure __msg("bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe fields")
    int BPF_PROG(task_kfunc_obj_drop_nmi_with_kptr, struct task_struct *task,
    u64 clone_flags)
    {
    struct __tasks_kfunc_map_value *local;
    struct task_struct *acquired, *old;
    (void)clone_flags;
    local = bpf_obj_new(typeof(*local));
    if (!local)
    return 0;
    acquired = bpf_task_acquire(task);
    if (acquired) {
    old = bpf_kptr_xchg(&local.task, acquired);
    if (old)
    bpf_task_release(old);
    }
    bpf_obj_drop(local);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_from_pid_no_null_check, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_from_pid_no_null_check, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    acquired = bpf_task_from_pid(task.pid);
// Releasing bpf_task_from_pid() lookup without a NULL check.
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_from_vpid_no_null_check, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_from_vpid_no_null_check, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *acquired;
    acquired = bpf_task_from_vpid(task.pid);
// Releasing bpf_task_from_vpid() lookup without a NULL check.
    bpf_task_release(acquired);
    return 0;
    }
    SEC("lsm/task_free")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 must be a rcu) -> __failure {
    __failure __msg("R1 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_from_lsm_task_free, task: *mut task_struct) -> c_int {
    int BPF_PROG(task_kfunc_from_lsm_task_free, struct task_struct *task)
    {
    struct task_struct *acquired;
// the argument of lsm task_free hook is untrusted.
    acquired = bpf_task_acquire(task);
    if (!acquired)
    return 0;
    bpf_task_release(acquired);
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(comm": "access beyond the end of member) -> __failure {
    __failure __msg("access beyond the end of member comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_access_comm1, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_access_comm1, struct task_struct *task, u64 clone_flags)
    {
    bpf_strncmp(task.comm, 17, "foo");
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(comm": "access beyond the end of member) -> __failure {
    __failure __msg("access beyond the end of member comm")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_access_comm2, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_access_comm2, struct task_struct *task, u64 clone_flags)
    {
    bpf_strncmp(task.comm + 1, 16, "foo");
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(memory": "write into) -> __failure {
    __failure __msg("write into memory")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_access_comm3, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_access_comm3, struct task_struct *task, u64 clone_flags)
    {
    bpf_probe_read_kernel(task.comm, 16, task.comm);
    return 0;
    }
    SEC("fentry/__set_task_comm")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected": "R1 type=ptr_) -> __failure {
    __failure __msg("R1 type=ptr_ expected")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_access_comm4, task: *mut task_struct, buf: *const c_char, exec: bool) -> c_int {
    int BPF_PROG(task_access_comm4, struct task_struct *task, const char *buf, bool exec)
    {
//
// task->comm is a legacy ptr_to_btf_id. The verifier cannot guarantee
// its safety. Hence it cannot be accessed with normal load insns.
//
    bpf_strncmp(task.comm, 16, "foo");
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to) -> __failure {
    __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_release_in_map, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(task_kfunc_release_in_map, struct task_struct *task, u64 clone_flags)
    {
    struct task_struct *local;
    struct __tasks_kfunc_map_value *v;
    if (tasks_kfunc_map_insert(task))
    return 0;
    v = tasks_kfunc_map_value_lookup(task);
    if (!v)
    return 0;
    bpf_rcu_read_lock();
    local = v.task;
    if (!local) {
    bpf_rcu_read_unlock();
    return 0;
    }
// Can't release a kptr that's still stored in a map.
    bpf_task_release(local);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 must be a rcu) -> __failure {
    __failure __msg("R1 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_after_final_spin_unlock) -> c_int {
    int BPF_PROG(task_kfunc_acquire_after_final_spin_unlock)
    {
    struct task_kptr_lock_value *v;
    struct task_struct *task, *acquired;
    let mut key: c_int = 0;
    v = bpf_map_lookup_elem(&task_kptr_lock_map, &key);
    if (!v)
    return 0;
    bpf_spin_lock(&v.lock);
    task = v.task;
    bpf_spin_unlock(&v.lock);
    if (!task)
    return 0;
    acquired = bpf_task_acquire(task);
    if (acquired)
    bpf_task_release(acquired);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 must be a rcu) -> __failure {
    __failure __msg("R1 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_after_preempt_enable) -> c_int {
    int BPF_PROG(task_kfunc_acquire_after_preempt_enable)
    {
    struct task_kptr_lock_value *v;
    struct task_struct *task, *acquired;
    let mut key: c_int = 0;
    v = bpf_map_lookup_elem(&task_kptr_lock_map, &key);
    if (!v)
    return 0;
    bpf_preempt_disable();
    task = v.task;
    bpf_preempt_enable();
    if (!task)
    return 0;
    acquired = bpf_task_acquire(task);
    if (acquired)
    bpf_task_release(acquired);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 must be a rcu) -> __failure {
    __failure __msg("R1 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: task_kfunc_acquire_after_irq_restore) -> c_int {
    int BPF_PROG(task_kfunc_acquire_after_irq_restore)
    {
    struct task_kptr_lock_value *v;
    struct task_struct *task, *acquired;
    unsigned long flags;
    let mut key: c_int = 0;
    v = bpf_map_lookup_elem(&task_kptr_lock_map, &key);
    if (!v)
    return 0;
    bpf_local_irq_save(&flags);
    task = v.task;
    bpf_local_irq_restore(&flags);
    if (!task)
    return 0;
    acquired = bpf_task_acquire(task);
    if (acquired)
    bpf_task_release(acquired);
    return 0;
    }
