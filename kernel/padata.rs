//! Automatically rewritten from C to Rust
//! Source: kernel/padata.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0
//
// padata.c - generic interface to process data streams in parallel
//
// See Documentation/core-api/padata.rst for more information.
//
// Copyright (C) 2008, 2009 secunet Security Networks AG
// Copyright (C) 2008, 2009 Steffen Klassert <steffen.klassert@secunet.com>
//
// Copyright (c) 2020 Oracle and/or its affiliates.
// Author: Daniel Jordan <daniel.m.jordan@oracle.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_work {
    pub pw_work: work_struct,
//     pub /: *mut *mut list_head pw_list; / padata_free_works linkage,
    pub pw_data: *mut c_void,
}
// static DEFINE_SPINLOCK(padata_works_lock);
pub static mut padata_works: *mut c_void = core::ptr::null_mut();
// static LIST_HEAD(padata_free_works);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_mt_job_state {
    pub lock: spinlock_t,
    pub completion: completion,
    pub job: *mut padata_mt_job,
    pub nworks: c_int,
    pub nworks_fini: c_int,
    pub chunk_size: c_ulong,
}

// forward_decl: padata_free_pd;
    static void __init padata_mt_helper(work_struct *work);
#[no_mangle]
pub unsafe extern "C" fn padata_get_pd(pd: *mut parallel_data) {
    refcount_inc(&pd.refcnt);
    }
#[no_mangle]
pub unsafe extern "C" fn padata_put_pd_cnt(pd: *mut parallel_data, cnt: c_int) {
    if (refcount_sub_and_test(cnt, &pd.refcnt)) {
    padata_free_pd(pd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn padata_put_pd(pd: *mut parallel_data) {
    padata_put_pd_cnt(pd, 1);
    }
#[no_mangle]
unsafe extern "C" fn padata_cpu_hash(pd: *mut parallel_data, seq_nr: c_uint) -> c_int {
//
// Hash the sequence numbers to the cpus by taking
// seq_nr mod. number of cpus in use.
//
pub static mut cpu_index: c_int = 0;
    return cpumask_nth(cpu_index, pd.cpumask.pcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn padata_work_alloc() -> *mut c_void {
pub static mut pw: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&padata_works_lock);
    if (list_empty(&padata_free_works)) {
    return core::ptr::null_mut();	/* No more work items allowed to be queued. */
    }
    pw = list_first_entry(&padata_free_works, padata_work, pw_list);
    list_del(&pw.pw_list);
    return pw;
    }
//
// This function is marked __ref because this function may be optimized in such
// a way that it directly refers to work_fn's address, which causes modpost to
// complain when work_fn is marked __init. This scenario was observed with clang
// LTO, where padata_work_init() was optimized to refer directly to
// padata_mt_helper() because the calls to padata_work_init() with other work_fn
// values were eliminated or inlined.
//
    static void __ref padata_work_init(padata_work *pw, work_func_t work_fn,
    void *data, int flags)
    {
    if (flags & PADATA_WORK_ONSTACK) {
    INIT_WORK_ONSTACK(&pw.pw_work, work_fn);
    }
    else {
    INIT_WORK(&pw.pw_work, work_fn);
    }
    pw.pw_data = data;
    }
    static int __init padata_work_alloc_mt(int nworks, void *data, list_head *head)
    {
    let mut i = 0;
    spin_lock_bh(&padata_works_lock);
// Start at 1 because the current task participates in the job.
    while (i < nworks) {
    let mut pw = padata_work_alloc();
    if (!pw) {
    break;
    }
    padata_work_init(pw, padata_mt_helper, data, 0);
    list_add(&pw.pw_list, head);
    }
    spin_unlock_bh(&padata_works_lock);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn padata_work_free(pw: *mut padata_work) {
    lockdep_assert_held(&padata_works_lock);
    list_add(&pw.pw_list, &padata_free_works);
    }
#[no_mangle]
unsafe extern "C" fn padata_works_free(works: *mut list_head) -> c_int {
    let mut cur = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    if (list_empty(works)) {
    return;
    }
    spin_lock_bh(&padata_works_lock);
    list_for_each_entry_safe(cur, next, works, pw_list) {
    list_del(&cur.pw_list);
    padata_work_free(cur);
    }
    spin_unlock_bh(&padata_works_lock);
    }
#[no_mangle]
unsafe extern "C" fn padata_parallel_worker(parallel_work: *mut work_struct) {
    let mut pw = container_of!(parallel_work, padata_work,
    pw_work);
    let mut padata = pw.pw_data;
    local_bh_disable();
    padata.parallel(padata);
    spin_lock(&padata_works_lock);
    padata_work_free(pw);
    spin_unlock(&padata_works_lock);
    local_bh_enable();
    }
//
// padata_do_parallel - padata parallelization function
//
// @ps: padatashell
// @padata: object to be parallelized
// @cb_cpu: pointer to the CPU that the serialization callback function should
// run on.  If it's not in the serial cpumask of @pinst
// (i.e. cpumask.cbcpu), this function selects a fallback CPU and if
// none found, returns -EINVAL.
//
// The parallelization callback function will run with BHs off.
// Note: Every object which is parallelized by padata_do_parallel
// must be seen by padata_do_serial.
//
// Return: 0 on success or else negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn padata_do_parallel(ps: *mut padata_shell, padata: *mut padata_priv, cb_cpu: *mut c_int) -> c_int {
    let mut pinst = ps.pinst;
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut pw: *mut c_void = core::ptr::null_mut();
    let mut cpu_index = 0;
    let mut err = 0;
    rcu_read_lock_bh();
    pd = rcu_dereference_bh(ps.pd);
    err = -EINVAL;
    if (!(pinst.flags & PADATA_INIT) || pinst.flags & PADATA_INVALID) {
// goto;
    }
    if (!cpumask_test_cpu(*cb_cpu, pd.cpumask.cbcpu)) {
    if (cpumask_empty(pd.cpumask.cbcpu)) {
// goto;
    }
// Select an alternate fallback CPU and notify the caller.
    cpu_index = *cb_cpu % cpumask_weight(pd.cpumask.cbcpu);
// cb_cpu = cpumask_nth(cpu_index, pd->cpumask.cbcpu);
    }
    err = -EBUSY;
    if ((pinst.flags & PADATA_RESET)) {
// goto;
    }
    padata_get_pd(pd);
    padata.pd = pd;
    padata.cb_cpu = *cb_cpu;
    spin_lock(&padata_works_lock);
    padata.seq_nr = ++pd.seq_nr;
    pw = padata_work_alloc();
    spin_unlock(&padata_works_lock);
    if (!pw) {
// Maximum works limit exceeded, run in the current task.
    padata.parallel(padata);
    }
    rcu_read_unlock_bh();
    if (pw) {
    padata_work_init(pw, padata_parallel_worker, padata, 0);
    queue_work(pinst.parallel_wq, &pw.pw_work);
    }
    return 0;
// label;
    rcu_read_unlock_bh();
    return err;
    }
    EXPORT_SYMBOL(padata_do_parallel);
//
// padata_find_next - Find the next object that needs serialization.
//
// Return:
// * A pointer to the control struct of the next object that needs
// serialization, if present in one of the percpu reorder queues.
// * NULL, if the next object that needs serialization will
// be parallel processed by another cpu and is not yet present in
// the cpu's reorder queue.
//
#[no_mangle]
pub unsafe extern "C" fn padata_find_next(pd: *mut parallel_data, cpu: c_int, processed: c_uint) -> *mut c_void {
pub static mut padata: *mut c_void = core::ptr::null_mut();
pub static mut reorder: *mut c_void = core::ptr::null_mut();
    reorder = per_cpu_ptr(pd.reorder_list, cpu);
    spin_lock(&reorder.lock);
    if (list_empty(&reorder.list)) {
// goto;
    }
    padata = list_entry(reorder.list.next, padata_priv, list);
//
// Checks the rare case where two or more parallel jobs have hashed to
// the same CPU and one of the later ones finishes first.
//
    if (padata.seq_nr != processed) {
// goto;
    }
    list_del_init(&padata.list);
    spin_unlock(&reorder.lock);
    return padata;
// label;
    pd.processed = processed;
    pd.cpu = cpu;
    spin_unlock(&reorder.lock);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn padata_reorder(padata: *mut padata_priv) {
    let mut pd = padata.pd;
    let mut pinst = pd.ps.pinst;
    let mut processed = 0;
    let mut cpu = 0;
    processed = pd.processed;
    cpu = pd.cpu;
    do {
pub static mut squeue: *mut c_void = core::ptr::null_mut();
    let mut cb_cpu = 0;
    processed += 1;
// When sequence wraps around, reset to the first CPU.
    if (unlikely(processed == 0)) {
    cpu = cpumask_first(pd.cpumask.pcpu);
    }
    else {
    cpu = cpumask_next_wrap(cpu, pd.cpumask.pcpu);
    }
    cb_cpu = padata.cb_cpu;
    squeue = per_cpu_ptr(pd.squeue, cb_cpu);
    spin_lock(&squeue.serial.lock);
    list_add_tail(&padata.list, &squeue.serial.list);
    queue_work_on(cb_cpu, pinst.serial_wq, &squeue.work);
//
// If the next object that needs serialization is parallel
// processed by another cpu and is still on it's way to the
// cpu's reorder queue, end the loop.
//
    padata = padata_find_next(pd, cpu, processed);
    spin_unlock(&squeue.serial.lock);
    } while (padata);
    }
#[no_mangle]
unsafe extern "C" fn padata_serial_worker(serial_work: *mut work_struct) {
pub static mut squeue: *mut c_void = core::ptr::null_mut();
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut local_list: usize = 0;
    let mut cnt = 0;
    local_bh_disable();
    squeue = container_of!(serial_work, padata_serial_queue, work);
    pd = squeue.pd;
    spin_lock(&squeue.serial.lock);
    list_replace_init(&squeue.serial.list, &local_list);
    spin_unlock(&squeue.serial.lock);
    cnt = 0;
    while (!list_empty(&local_list)) {
pub static mut padata: *mut c_void = core::ptr::null_mut();
    padata = list_entry(local_list.next, padata_priv, list);
    list_del_init(&padata.list);
    padata.serial(padata);
    cnt += 1;
    }
    local_bh_enable();
    padata_put_pd_cnt(pd, cnt);
    }
//
// padata_do_serial - padata serialization function
//
// @padata: object to be serialized.
//
// padata_do_serial must be called for every parallelized object.
// The serialization callback function will run with BHs off.
//
#[no_mangle]
pub unsafe extern "C" fn padata_do_serial(padata: *mut padata_priv) {
    let mut pd = padata.pd;
pub static mut hashed_cpu: c_int = 0;
    let mut reorder = per_cpu_ptr(pd.reorder_list, hashed_cpu);
pub static mut cur: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut gotit: bool = true;
    spin_lock(&reorder.lock);
// Sort in ascending order of sequence number.
    list_for_each_prev(pos, &reorder.list) {
    cur = list_entry(pos, padata_priv, list);
// Compare by difference to consider integer wrap around
    if ((signed int)(cur.seq_nr - padata.seq_nr) < 0) {
    break;
    }
    }
    if (padata.seq_nr != pd.processed) {
    gotit = false;
    list_add(&padata.list, pos);
    }
    spin_unlock(&reorder.lock);
    if (gotit) {
    padata_reorder(padata);
    }
    }
    EXPORT_SYMBOL(padata_do_serial);
#[no_mangle]
unsafe extern "C" fn padata_setup_cpumasks(pinst: *mut padata_instance) -> c_int {
pub static mut attrs: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    attrs = alloc_workqueue_attrs();
    if (!attrs) {
    return -ENOMEM;
    }
// Restrict parallel_wq workers to pd->cpumask.pcpu.
    cpumask_copy(attrs.cpumask, pinst.cpumask.pcpu);
    err = apply_workqueue_attrs(pinst.parallel_wq, attrs);
    free_workqueue_attrs(attrs);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn padata_mt_helper(w: *mut work_struct) -> c_int {
    let mut pw = container_of!(w, padata_work, pw_work);
    let mut ps = pw.pw_data;
    let mut job = ps.job;
    let mut done = 0;
    spin_lock(&ps.lock);
    while (job.size > 0) {
    unsigned long start, size, end;
    start = job.start;
// So end is chunk size aligned if enough work remains.
    size = roundup(start + 1, ps.chunk_size) - start;
    size = min(size, job.size);
    end = start + size;
    job.start = end;
    job.size -= size;
    spin_unlock(&ps.lock);
    job.thread_fn(start, end, job.fn_arg);
    spin_lock(&ps.lock);
    }
    ++ps.nworks_fini;
    done = (ps.nworks_fini == ps.nworks);
    spin_unlock(&ps.lock);
    if (done) {
    complete(&ps.completion);
    }
    }
//
// padata_do_multithreaded - run a multithreaded job
// @job: Description of the job.
//
// See the definition of struct padata_mt_job for more details.
//
#[no_mangle]
pub unsafe extern "C" fn padata_do_multithreaded(job: *mut padata_mt_job) -> c_int {
// In case threads finish at different times.
pub static mut load_balance_factor: unsigned long = 4;
    struct padata_work my_work, *pw;
pub static mut ps: usize = 0;
pub static mut works: usize = 0;
    let mut nworks = 0;
    let mut nid = 0;
    static atomic_t last_used_nid __initdata;
    if (job.size == 0) {
    return;
    }
// Ensure at least one thread when size < min_chunk.
    nworks = max(job.size / max(job.min_chunk, job.align), 1ul);
    nworks = min(nworks, job.max_threads);
    if (nworks == 1) {
// Single thread, no coordination needed, cut to the chase.
    job.thread_fn(job.start, job.start + job.size, job.fn_arg);
    return;
    }
    spin_lock_init(&ps.lock);
    init_completion(&ps.completion);
    ps.job	       = job;
    ps.nworks      = padata_work_alloc_mt(nworks, &ps, &works);
    ps.nworks_fini = 0;
//
// Chunk size is the amount of work a helper does per call to the
// thread function.  Load balance large jobs between threads by
// increasing the number of chunks, guarantee at least the minimum
// chunk size from the caller, and honor the caller's alignment.
// Ensure chunk_size is at least 1 to prevent divide-by-0
// panic in padata_mt_helper().
//
    ps.chunk_size = job.size / (ps.nworks * load_balance_factor);
    ps.chunk_size = max(ps.chunk_size, job.min_chunk);
    ps.chunk_size = max(ps.chunk_size, 1ul);
    ps.chunk_size = roundup(ps.chunk_size, job.align);
    list_for_each_entry(pw, &works, pw_list) {
    if (job.numa_aware) {
    }
pub static mut old_node: c_int = 0;
    do {
    nid = next_node_in(old_node, node_states[N_CPU]);
    } while (!atomic_try_cmpxchg(&last_used_nid, &old_node, nid));
    queue_work_node(nid, system_dfl_wq, &pw.pw_work);
    } else {
    queue_work(system_dfl_wq, &pw.pw_work);
    }
// Use the current thread, which saves starting a workqueue worker.
    padata_work_init(&my_work, padata_mt_helper, &ps, PADATA_WORK_ONSTACK);
    padata_mt_helper(&my_work.pw_work);
// Wait for all the helpers to finish.
    wait_for_completion(&ps.completion);
    destroy_work_on_stack(&my_work.pw_work);
    padata_works_free(&works);
    }
// Initialize all percpu queues used by serial workers
#[no_mangle]
unsafe extern "C" fn padata_init_squeues(pd: *mut parallel_data) {
    let mut cpu = 0;
pub static mut squeue: *mut c_void = core::ptr::null_mut();
    for_each_cpu(cpu, pd.cpumask.cbcpu) {
    squeue = per_cpu_ptr(pd.squeue, cpu);
    squeue.pd = pd;
    INIT_LIST_HEAD(&squeue.serial.list);
    spin_lock_init(&squeue.serial.lock);
    INIT_WORK(&squeue.work, padata_serial_worker);
    }
    }
// Initialize per-CPU reorder lists
#[no_mangle]
unsafe extern "C" fn padata_init_reorder_list(pd: *mut parallel_data) {
    let mut cpu = 0;
pub static mut list: *mut c_void = core::ptr::null_mut();
    for_each_cpu(cpu, pd.cpumask.pcpu) {
    list = per_cpu_ptr(pd.reorder_list, cpu);
    INIT_LIST_HEAD(&list.list);
    spin_lock_init(&list.lock);
    }
    }
// Allocate and initialize the internal cpumask dependend resources.
#[no_mangle]
pub unsafe extern "C" fn padata_alloc_pd(ps: *mut padata_shell, offlining_cpu: c_int) -> *mut c_void {
    let mut pinst = ps.pinst;
pub static mut pd: *mut c_void = core::ptr::null_mut();
    pd = kzalloc_obj(parallel_data);
    if (!pd) {
// goto;
    }
    pd.reorder_list = alloc_percpu(padata_list);
    if (!pd.reorder_list) {
// goto;
    }
    pd.squeue = alloc_percpu(padata_serial_queue);
    if (!pd.squeue) {
// goto;
    }
    pd.ps = ps;
    if (!alloc_cpumask_var(&pd.cpumask.pcpu, GFP_KERNEL)) {
// goto;
    }
    if (!alloc_cpumask_var(&pd.cpumask.cbcpu, GFP_KERNEL)) {
// goto;
    }
    cpumask_and(pd.cpumask.pcpu, pinst.cpumask.pcpu, cpu_online_mask);
    cpumask_and(pd.cpumask.cbcpu, pinst.cpumask.cbcpu, cpu_online_mask);
    if (offlining_cpu >= 0) {
    __cpumask_clear_cpu(offlining_cpu, pd.cpumask.pcpu);
    __cpumask_clear_cpu(offlining_cpu, pd.cpumask.cbcpu);
    }
    padata_init_reorder_list(pd);
    padata_init_squeues(pd);
    pd.seq_nr = -1;
    refcount_set(&pd.refcnt, 1);
    pd.cpu = cpumask_first(pd.cpumask.pcpu);
    return pd;
// label;
    free_cpumask_var(pd.cpumask.pcpu);
// label;
    free_percpu(pd.squeue);
// label;
    free_percpu(pd.reorder_list);
// label;
    kfree(pd);
// label;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn padata_free_pd(pd: *mut parallel_data) {
    free_cpumask_var(pd.cpumask.pcpu);
    free_cpumask_var(pd.cpumask.cbcpu);
    free_percpu(pd.reorder_list);
    free_percpu(pd.squeue);
    kfree(pd);
    }
#[no_mangle]
unsafe extern "C" fn __padata_start(pinst: *mut padata_instance) {
    pinst.flags |= PADATA_INIT;
    }
#[no_mangle]
unsafe extern "C" fn __padata_stop(pinst: *mut padata_instance) {
    if (!(pinst.flags & PADATA_INIT)) {
    return;
    }
    pinst.flags &= ~PADATA_INIT;
    synchronize_rcu();
    }
// Replace the internal control structure with a new one.
#[no_mangle]
unsafe extern "C" fn padata_replace_one(ps: *mut padata_shell, offlining_cpu: c_int) -> c_int {
pub static mut pd_new: *mut c_void = core::ptr::null_mut();
    pd_new = padata_alloc_pd(ps, offlining_cpu);
    if (!pd_new) {
    return -ENOMEM;
    }
    ps.opd = rcu_dereference_protected(ps.pd, 1);
    rcu_assign_pointer(ps.pd, pd_new);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn padata_replace(pinst: *mut padata_instance, offlining_cpu: c_int) -> c_int {
pub static mut ps: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    pinst.flags |= PADATA_RESET;
    list_for_each_entry(ps, &pinst.pslist, list) {
    err = padata_replace_one(ps, offlining_cpu);
    if (err) {
    break;
    }
    }
    synchronize_rcu();
    list_for_each_entry_continue_reverse(ps, &pinst.pslist, list) {
    padata_put_pd(ps.opd);
    }
    pinst.flags &= ~PADATA_RESET;
    return err;
    }
// If cpumask contains no active cpu, we mark the instance as invalid.
#[no_mangle]
pub unsafe extern "C" fn padata_validate_cpumask(pinst: *mut padata_instance, cpumask: *mut cpumask, offlining_cpu: c_int) -> bool {
    cpumask_copy(pinst.validate_cpumask, cpu_online_mask);
//
// @offlining_cpu is still in cpu_online_mask, so remove it here for
// validation.  Using a sub-CPUHP_TEARDOWN_CPU hotplug state where
// @offlining_cpu wouldn't be in the online mask doesn't work because
// padata_cpu_offline() can fail but such a state doesn't allow failure.
//
    if (offlining_cpu >= 0) {
    __cpumask_clear_cpu(offlining_cpu, pinst.validate_cpumask);
    }
    if (!cpumask_intersects(cpumask, pinst.validate_cpumask)) {
    pinst.flags |= PADATA_INVALID;
    return false;
    }
    pinst.flags &= ~PADATA_INVALID;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __padata_set_cpumasks(pinst: *mut padata_instance, pcpumask: cpumask_var_t, cbcpumask: cpumask_var_t) -> c_int {
    let mut valid = 0;
    let mut err = 0;
    valid = padata_validate_cpumask(pinst, pcpumask, -1);
    if (!valid) {
    __padata_stop(pinst);
// goto;
    }
    valid = padata_validate_cpumask(pinst, cbcpumask, -1);
    if (!valid) {
    __padata_stop(pinst);
    }
// label;
    cpumask_copy(pinst.cpumask.pcpu, pcpumask);
    cpumask_copy(pinst.cpumask.cbcpu, cbcpumask);
    err = padata_setup_cpumasks(pinst) ?: padata_replace(pinst, -1);
    if (valid) {
    __padata_start(pinst);
    }
    return err;
    }
//
// padata_set_cpumask - Sets specified by @cpumask_type cpumask to the value
// equivalent to @cpumask.
// @pinst: padata instance
// @cpumask_type: PADATA_CPU_SERIAL or PADATA_CPU_PARALLEL corresponding
// to parallel and serial cpumasks respectively.
// @cpumask: the cpumask to use
//
// Return: 0 on success or negative error code
//
#[no_mangle]
pub unsafe extern "C" fn padata_set_cpumask(pinst: *mut padata_instance, cpumask_type: c_int, cpumask: cpumask_var_t) -> c_int {
    let mut serial_mask = core::ptr::null_mut();
    let mut parallel_mask = core::ptr::null_mut();
pub static mut err: c_int = 0;
    cpus_read_lock();
    mutex_lock(&pinst.lock);
    match (cpumask_type) {
    PADATA_CPU_PARALLEL => {
    serial_mask = pinst.cpumask.cbcpu;
    parallel_mask = cpumask;
    // break;
    }
    PADATA_CPU_SERIAL => {
    parallel_mask = pinst.cpumask.pcpu;
    serial_mask = cpumask;
    // break;
    }
    _ => {
// goto;
    }
    }
    err =  __padata_set_cpumasks(pinst, parallel_mask, serial_mask);
// label;
    mutex_unlock(&pinst.lock);
    cpus_read_unlock();
    return err;
    }
    EXPORT_SYMBOL(padata_set_cpumask);

#[no_mangle]
pub unsafe extern "C" fn pinst_has_cpu(pinst: *mut padata_instance, cpu: c_int) -> c_int {
    return cpumask_test_cpu(cpu, pinst.cpumask.pcpu) ||
    cpumask_test_cpu(cpu, pinst.cpumask.cbcpu);
    }
#[no_mangle]
unsafe extern "C" fn padata_cpu_online(cpu: c_uint, node: *mut hlist_node) -> c_int {
pub static mut pinst: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    pinst = hlist_entry_safe(node, padata_instance, cpuhp_node);
    if (!pinst_has_cpu(pinst, cpu)) {
    return 0;
    }
    mutex_lock(&pinst.lock);
    ret = padata_replace(pinst, -1);
    if (padata_validate_cpumask(pinst, pinst.cpumask.pcpu, -1) &&
    padata_validate_cpumask(pinst, pinst.cpumask.cbcpu, -1)) {
    __padata_start(pinst);
    }
    mutex_unlock(&pinst.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn padata_cpu_offline(cpu: c_uint, node: *mut hlist_node) -> c_int {
pub static mut pinst: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    pinst = hlist_entry_safe(node, padata_instance, cpuhp_node);
    if (!pinst_has_cpu(pinst, cpu)) {
    return 0;
    }
    mutex_lock(&pinst.lock);
    if (!padata_validate_cpumask(pinst, pinst.cpumask.pcpu, cpu) ||
    !padata_validate_cpumask(pinst, pinst.cpumask.cbcpu, cpu)) {
    __padata_stop(pinst);
    }
    ret = padata_replace(pinst, cpu);
    mutex_unlock(&pinst.lock);
    return ret;
    }
    static enum cpuhp_state hp_online;

#[no_mangle]
unsafe extern "C" fn __padata_free(pinst: *mut padata_instance) {

    cpuhp_state_remove_instance_nocalls(hp_online, &pinst.cpuhp_node);

    WARN_ON!(!list_empty(&pinst.pslist));
    free_cpumask_var(pinst.cpumask.pcpu);
    free_cpumask_var(pinst.cpumask.cbcpu);
    free_cpumask_var(pinst.validate_cpumask);
    destroy_workqueue(pinst.serial_wq);
    destroy_workqueue(pinst.parallel_wq);
    kfree(pinst);
    }

    container_of!(_kobj, padata_instance, kobj)

    container_of_const(_attr, padata_sysfs_entry, attr)
#[no_mangle]
unsafe extern "C" fn padata_sysfs_release(kobj: *mut kobject) {
    let mut pinst = kobj2pinst(kobj);
    __padata_free(pinst);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct padata_sysfs_entry {
    pub attr: attribute,
// fn ptr field
    ssize_t (*store)(padata_instance *, const struct attribute *,
    pub size_t): *const *const char ,,
}

#[no_mangle]
pub unsafe extern "C" fn show_cpumask(pinst: *mut padata_instance, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
pub static mut cpumask: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    mutex_lock(&pinst.lock);
    if (!strcmp(attr.name, "serial_cpumask")) {
    cpumask = pinst.cpumask.cbcpu;
    }
    else {
    cpumask = pinst.cpumask.pcpu;
    }
    len = snprintf(buf, PAGE_SIZE, "%*pb\n",
    nr_cpu_ids, cpumask_bits(cpumask));
    mutex_unlock(&pinst.lock);
    return len < PAGE_SIZE ? len : -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn store_cpumask(pinst: *mut padata_instance, attr: *mut attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut new_cpumask;
    let mut ret = 0;
    let mut mask_type = 0;
    if (!alloc_cpumask_var(&new_cpumask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    ret = bitmap_parse(buf, count, cpumask_bits(new_cpumask),
    nr_cpumask_bits);
    if (ret < 0) {
// goto;
    }
    mask_type = !strcmp(attr.name, "serial_cpumask") ?
    PADATA_CPU_SERIAL : PADATA_CPU_PARALLEL;
    ret = padata_set_cpumask(pinst, mask_type, new_cpumask);
    if (!ret) {
    ret = count;
    }
// label;
    free_cpumask_var(new_cpumask);
    return ret;
    }

    static const struct padata_sysfs_entry _name##_attr =	
    __ATTR(_name, 0644, _show_name, _store_name)

    static const struct padata_sysfs_entry _name##_attr =	
    __ATTR(_name, 0400, _show_name, core::ptr::null_mut())
    PADATA_ATTR_RW(serial_cpumask, show_cpumask, store_cpumask);
    PADATA_ATTR_RW(parallel_cpumask, show_cpumask, store_cpumask);
//
// Padata sysfs provides the following objects:
// serial_cpumask   [RW] - cpumask for serial workers
// parallel_cpumask [RW] - cpumask for parallel workers
//
    static const struct attribute *const padata_default_attrs[] = {
    &serial_cpumask_attr.attr,
    &parallel_cpumask_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(padata_default);
#[no_mangle]
pub unsafe extern "C" fn padata_sysfs_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
pub static mut pentry: *mut c_void = core::ptr::null_mut();
pub static mut pinst: *mut c_void = core::ptr::null_mut();
pub static mut ret: isize = 0;
    pinst = kobj2pinst(kobj);
    pentry = attr2pentry(attr);
    if (pentry.show) {
    ret = pentry.show(pinst, attr, buf);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn padata_sysfs_store(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char, count: size_t) -> ssize_t {
pub static mut pentry: *mut c_void = core::ptr::null_mut();
pub static mut pinst: *mut c_void = core::ptr::null_mut();
pub static mut ret: isize = 0;
    pinst = kobj2pinst(kobj);
    pentry = attr2pentry(attr);
    if (pentry.store) {
    ret = pentry.store(pinst, attr, buf, count);
    }
    return ret;
    }
pub static mut sysfs_ops: usize = 0;
pub static mut kobj_type: usize = 0;
//
// padata_alloc - allocate and initialize a padata instance
// @name: used to identify the instance
//
// Return: new instance on success, NULL on error
//
#[no_mangle]
pub unsafe extern "C" fn padata_alloc(name: *mut c_char) -> *mut c_void {
pub static mut pinst: *mut c_void = core::ptr::null_mut();
    pinst = kzalloc_obj(padata_instance);
    if (!pinst) {
// goto;
    }
    pinst.parallel_wq = alloc_workqueue("%s_parallel", WQ_UNBOUND, 0,
    name);
    if (!pinst.parallel_wq) {
// goto;
    }
    cpus_read_lock();
    pinst.serial_wq = alloc_workqueue("%s_serial",
    WQ_MEM_RECLAIM | WQ_CPU_INTENSIVE | WQ_PERCPU,
    1, name);
    if (!pinst.serial_wq) {
// goto;
    }
    if (!alloc_cpumask_var(&pinst.cpumask.pcpu, GFP_KERNEL)) {
// goto;
    }
    if (!alloc_cpumask_var(&pinst.cpumask.cbcpu, GFP_KERNEL)) {
// goto;
    }
    if (!alloc_cpumask_var(&pinst.validate_cpumask, GFP_KERNEL)) {
// goto;
    }
    INIT_LIST_HEAD(&pinst.pslist);
    cpumask_copy(pinst.cpumask.pcpu, cpu_possible_mask);
    cpumask_copy(pinst.cpumask.cbcpu, cpu_possible_mask);
    if (padata_setup_cpumasks(pinst)) {
// goto;
    }
    __padata_start(pinst);
    kobject_init(&pinst.kobj, &padata_attr_type);
    mutex_init(&pinst.lock);

    cpuhp_state_add_instance_nocalls_cpuslocked(hp_online,
    &pinst.cpuhp_node);

    cpus_read_unlock();
    return pinst;
// label;
    free_cpumask_var(pinst.validate_cpumask);
// label;
    free_cpumask_var(pinst.cpumask.cbcpu);
// label;
    free_cpumask_var(pinst.cpumask.pcpu);
// label;
    destroy_workqueue(pinst.serial_wq);
// label;
    cpus_read_unlock();
    destroy_workqueue(pinst.parallel_wq);
// label;
    kfree(pinst);
// label;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(padata_alloc);
//
// padata_free - free a padata instance
//
// @pinst: padata instance to free
//
#[no_mangle]
pub unsafe extern "C" fn padata_free(pinst: *mut padata_instance) {
    kobject_put(&pinst.kobj);
    }
    EXPORT_SYMBOL(padata_free);
//
// padata_alloc_shell - Allocate and initialize padata shell.
//
// @pinst: Parent padata_instance object.
//
// Return: new shell on success, NULL on error
//
#[no_mangle]
pub unsafe extern "C" fn padata_alloc_shell(pinst: *mut padata_instance) -> *mut c_void {
pub static mut pd: *mut c_void = core::ptr::null_mut();
pub static mut ps: *mut c_void = core::ptr::null_mut();
    ps = kzalloc_obj(*ps);
    if (!ps) {
// goto;
    }
    ps.pinst = pinst;
    cpus_read_lock();
    pd = padata_alloc_pd(ps, -1);
    cpus_read_unlock();
    if (!pd) {
// goto;
    }
    mutex_lock(&pinst.lock);
    RCU_INIT_POINTER(ps.pd, pd);
    list_add(&ps.list, &pinst.pslist);
    mutex_unlock(&pinst.lock);
    return ps;
// label;
    kfree(ps);
// label;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(padata_alloc_shell);
//
// padata_free_shell - free a padata shell
//
// @ps: padata shell to free
//
#[no_mangle]
pub unsafe extern "C" fn padata_free_shell(ps: *mut padata_shell) {
pub static mut pd: *mut c_void = core::ptr::null_mut();
    if (!ps) {
    return;
    }
    mutex_lock(&ps.pinst.lock);
    list_del(&ps.list);
    pd = rcu_dereference_protected(ps.pd, 1);
    padata_put_pd(pd);
    mutex_unlock(&ps.pinst.lock);
    kfree(ps);
    }
    EXPORT_SYMBOL(padata_free_shell);
#[no_mangle]
pub unsafe extern "C" fn padata_init() -> c_int {
    let mut i = 0;
    let mut possible_cpus = 0;

    let mut ret = 0;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN, "padata:online",
    padata_cpu_online, padata_cpu_offline);
    if (ret < 0) {
// goto;
    }
    hp_online = ret;

    possible_cpus = num_possible_cpus();
    padata_works = kmalloc_objs(padata_work, possible_cpus);
    if (!padata_works) {
// goto;
    }
    for (i = 0; i < possible_cpus; ++i) {
    list_add(&padata_works[i].pw_list, &padata_free_works);
    }
    return;
// label;
    cpuhp_remove_multi_state(hp_online);
// label;
    pr_warn!("padata: initialization failed\n");
    }