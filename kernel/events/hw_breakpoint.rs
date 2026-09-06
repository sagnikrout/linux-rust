//! Automatically rewritten from C to Rust
//! Source: kernel/events/hw_breakpoint.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2007 Alan Stern
// Copyright (C) IBM Corporation, 2009
// Copyright (C) 2009, Frederic Weisbecker <fweisbec@gmail.com>
//
// Thanks to Ingo Molnar for his many suggestions.
//
// Authors: Alan Stern <stern@rowland.harvard.edu>
// K.Prasad <prasad@linux.vnet.ibm.com>
// Frederic Weisbecker <fweisbec@gmail.com>
//
// HW_breakpoint: a unified kernel/user-space hardware breakpoint facility,
// using the CPU's debug registers.
// This file contains the arch-independent routines.
//

//
// Datastructure to track the total uses of N slots across tasks or CPUs;
// bp_slots_histogram::count[N] is the number of assigned N+1 breakpoint slots.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_slots_histogram {
    pub count: [core::sync::atomic::AtomicI32; hw_breakpoint_slots(0)],
    pub count: *mut core::sync::atomic::AtomicI32,

}

//
// Per-CPU constraints data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_cpuinfo {
// Number of pinned CPU breakpoints in a CPU.
    pub cpu_pinned: c_uint,
// Histogram of pinned task breakpoints in a CPU.
    pub tsk_pinned: bp_slots_histogram,
}

pub static mut struct bp_cpuinfo: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn get_bp_info(cpu: c_int, type: bp_type_idx) -> *mut c_void {
    return per_cpu_ptr(bp_cpuinfo + type, cpu);
    }
// Number of pinned CPU breakpoints globally.
    static struct bp_slots_histogram cpu_pinned[TYPE_MAX];
// Number of pinned CPU-independent task breakpoints.
    static struct bp_slots_histogram tsk_pinned_all[TYPE_MAX];
// Keep track of the breakpoints attached to tasks
pub static mut task_bps_ht: usize = 0;
pub static mut rhashtable_params: usize = 0;
    static bool constraints_initialized __ro_after_init;
//
// Synchronizes accesses to the per-CPU constraints; the locking rules are:
//
// 1. Atomic updates to bp_cpuinfo::tsk_pinned only require a held read-lock
// (due to bp_slots_histogram::count being atomic, no update are lost).
//
// 2. Holding a write-lock is required for computations that require a
// stable snapshot of all bp_cpuinfo::tsk_pinned.
//
// 3. In all other cases, non-atomic accesses require the appropriately held
// lock (read-lock for read-only accesses; write-lock for reads/writes).
//
pub static mut bp_cpuinfo_sem: usize = 0;
//
// Return mutex to serialize accesses to per-task lists in task_bps_ht. Since
// rhltable synchronizes concurrent insertions/deletions, independent tasks may
// insert/delete concurrently; therefore, a mutex per task is sufficient.
//
// Uses task_struct::perf_event_mutex, to avoid extending task_struct with a
// hw_breakpoint-only mutex, which may be infrequently used. The caveat here is
// that hw_breakpoint may contend with per-task perf event list management. The
// assumption is that perf usecases involving hw_breakpoints are very unlikely
// to result in unnecessary contention.
//
#[no_mangle]
pub unsafe extern "C" fn get_task_bps_mutex(bp: *mut perf_event) -> *mut c_void {
    let mut tsk = bp.hw.target;
    return tsk ? &tsk.perf_event_mutex : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bp_constraints_lock(bp: *mut perf_event) -> *mut c_void {
    let mut tsk_mtx = get_task_bps_mutex(bp);
    if (tsk_mtx) {
//
// Fully analogous to the perf_try_init_event() nesting
// argument in the comment near perf_event_ctx_lock_nested();
// this child->perf_event_mutex cannot ever deadlock against
// the parent->perf_event_mutex usage from
// perf_event_task_{en,dis}able().
//
// Specifically, inherited events will never occur on
// ->perf_event_list.
//
    mutex_lock_nested(tsk_mtx, SINGLE_DEPTH_NESTING);
    percpu_down_read(&bp_cpuinfo_sem);
    } else {
    percpu_down_write(&bp_cpuinfo_sem);
    }
    return tsk_mtx;
    }
#[no_mangle]
unsafe extern "C" fn bp_constraints_unlock(tsk_mtx: *mut mutex) {
    if (tsk_mtx) {
    percpu_up_read(&bp_cpuinfo_sem);
    mutex_unlock(tsk_mtx);
    } else {
    percpu_up_write(&bp_cpuinfo_sem);
    }
    }
#[no_mangle]
unsafe extern "C" fn bp_constraints_is_locked(bp: *mut perf_event) -> bool {
    let mut tsk_mtx = get_task_bps_mutex(bp);
    return percpu_is_write_locked(&bp_cpuinfo_sem) ||
    (tsk_mtx ? mutex_is_locked(tsk_mtx) :
    percpu_is_read_locked(&bp_cpuinfo_sem));
    }
#[no_mangle]
pub unsafe extern "C" fn assert_bp_constraints_lock_held(bp: *mut perf_event) {
    let mut tsk_mtx = get_task_bps_mutex(bp);
    if (tsk_mtx) {
    lockdep_assert_held(tsk_mtx);
    }
    lockdep_assert_held(&bp_cpuinfo_sem);
    }

//
// Number of breakpoint slots is constant, and the same for all types.
//
    static_assert(hw_breakpoint_slots(TYPE_INST) == hw_breakpoint_slots(TYPE_DATA));
#[no_mangle]
pub unsafe extern "C" fn hw_breakpoint_slots_cached(type: c_int) -> c_int { return hw_breakpoint_slots(type); }
#[no_mangle]
pub unsafe extern "C" fn init_breakpoint_slots() -> c_int { return 0; }

//
// Dynamic number of breakpoint slots.
//
    static int __nr_bp_slots[TYPE_MAX] __ro_after_init;
#[no_mangle]
#[no_mangle]
// duplicate fn: hw_breakpoint_slots_cached
pub unsafe extern "C" fn hw_breakpoint_slots_cached_dup(type: c_int) -> c_int {
    return __nr_bp_slots[type];
    }
#[no_mangle]
pub unsafe extern "C" fn bp_slots_histogram_alloc(hist: *mut bp_slots_histogram, type: bp_type_idx) -> bool {
    hist.count = kzalloc_objs(*hist.count,
    hw_breakpoint_slots_cached(type));
    return hist.count;
    }
#[no_mangle]
unsafe extern "C" fn bp_slots_histogram_free(hist: *mut bp_slots_histogram) -> __init void {
    kfree(hist.count);
    }
#[no_mangle]
unsafe extern "C" fn init_breakpoint_slots() -> __init int {
    let mut i = 0;
    let mut cpu = 0;
    let mut err_cpu = 0;
    for (i = 0; i < TYPE_MAX; i++) {
    __nr_bp_slots[i] = hw_breakpoint_slots(i);
    }
    for_each_possible_cpu(cpu) {
    while (i < TYPE_MAX) {
    let mut info = get_bp_info(cpu, i);
    if (!bp_slots_histogram_alloc(&info.tsk_pinned, i)) {
// goto;
    }
    }
    }
    while (i < TYPE_MAX) {
    if (!bp_slots_histogram_alloc(&cpu_pinned[i], i)) {
// goto;
    }
    if (!bp_slots_histogram_alloc(&tsk_pinned_all[i], i)) {
// goto;
    }
    }
    return 0;
// label;
    for_each_possible_cpu(err_cpu) {
    for (i = 0; i < TYPE_MAX; i++) {
    bp_slots_histogram_free(&get_bp_info(err_cpu, i).tsk_pinned);
    }
    if (err_cpu == cpu) {
    break;
    }
    }
    while (i < TYPE_MAX) {
    bp_slots_histogram_free(&cpu_pinned[i]);
    bp_slots_histogram_free(&tsk_pinned_all[i]);
    }
    return -ENOMEM;
    }

#[no_mangle]
pub unsafe extern "C" fn bp_slots_histogram_add(hist: *mut bp_slots_histogram, old: c_int, val: c_int) {
pub static mut old_idx: c_int = 0;
pub static mut new_idx: c_int = 0;
    if (old_idx >= 0) {
    WARN_ON!(atomic_dec_return_relaxed(&hist.count[old_idx]) < 0);
    }
    if (new_idx >= 0) {
    WARN_ON!(atomic_inc_return_relaxed(&hist.count[new_idx]) < 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bp_slots_histogram_max(hist: *mut bp_slots_histogram, type: bp_type_idx) -> c_int {
    while (i >= 0) {
pub static mut count: c_int = 0;
// Catch unexpected writers; we want a stable snapshot.
    ASSERT_EXCLUSIVE_WRITER(hist.count[i]);
    if (count > 0) {
    return i + 1;
    }
    WARN(count < 0, "inconsistent breakpoint slots histogram");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bp_slots_histogram_max_merge(hist1: *mut bp_slots_histogram, hist2: *mut bp_slots_histogram, type: bp_type_idx) -> c_int {
    while (i >= 0) {
pub static mut count1: c_int = 0;
pub static mut count2: c_int = 0;
// Catch unexpected writers; we want a stable snapshot.
    ASSERT_EXCLUSIVE_WRITER(hist1.count[i]);
    ASSERT_EXCLUSIVE_WRITER(hist2.count[i]);
    if (count1 + count2 > 0) {
    return i + 1;
    }
    WARN(count1 < 0, "inconsistent breakpoint slots histogram");
    WARN(count2 < 0, "inconsistent breakpoint slots histogram");
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hw_breakpoint_weight(bp: *mut perf_event) -> c_int {
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn find_slot_idx(bp_type: u64) -> enum bp_type_idx {
    if (bp_type & HW_BREAKPOINT_RW) {
    return TYPE_DATA;
    }
    return TYPE_INST;
    }
//
// Return the maximum number of pinned breakpoints a task has in this CPU.
//
#[no_mangle]
unsafe extern "C" fn max_task_bp_pinned(cpu: c_int, type: bp_type_idx) -> c_uint {
    let mut tsk_pinned = &get_bp_info(cpu, type).tsk_pinned;
//
// At this point we want to have acquired the bp_cpuinfo_sem as a
// writer to ensure that there are no concurrent writers in
// toggle_bp_task_slot() to tsk_pinned, and we get a stable snapshot.
//
    lockdep_assert_held_write(&bp_cpuinfo_sem);
    return bp_slots_histogram_max_merge(tsk_pinned, &tsk_pinned_all[type], type);
    }
//
// Count the number of breakpoints of the same type and same task.
// The given event must be not on the list.
//
// If @cpu is -1, but the result of task_bp_pinned() is not CPU-independent,
// returns a negative value.
//
#[no_mangle]
unsafe extern "C" fn task_bp_pinned(cpu: c_int, bp: *mut perf_event, type: bp_type_idx) -> c_int {
    let mut head = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
//
// We need a stable snapshot of the per-task breakpoint list.
//
    assert_bp_constraints_lock_held(bp);
    rcu_read_lock();
    head = rhltable_lookup(&task_bps_ht, &bp.hw.target, task_bps_ht_params);
    if (!head) {
// goto;
    }
    rhl_for_each_entry_rcu(iter, pos, head, hw.bp_list) {
    if (find_slot_idx(iter.attr.bp_type) != type) {
    continue;
    }
    if (iter.cpu >= 0) {
    if (cpu == -1) {
    count = -1;
// goto;
    } else if (cpu != iter.cpu) {
    continue;
    }
    }
    count += hw_breakpoint_weight(iter);
    }
// label;
    rcu_read_unlock();
    return count;
    }
    static const struct cpumask *cpumask_of_bp(perf_event *bp)
    {
    if (bp.cpu >= 0) {
    return cpumask_of(bp.cpu);
    }
    return cpu_possible_mask;
    }
//
// Returns the max pinned breakpoint slots in a given
// CPU (cpu > -1) or across all of them (cpu = -1).
//
#[no_mangle]
pub unsafe extern "C" fn max_bp_pinned_slots(bp: *mut perf_event, type: bp_type_idx) -> c_int {
    let mut cpumask = cpumask_of_bp(bp);
pub static mut pinned_slots: c_int = 0;
    let mut cpu = 0;
    if (bp.hw.target && bp.cpu < 0) {
pub static mut max_pinned: c_int = 0;
    if (max_pinned >= 0) {
//
// Fast path: task_bp_pinned() is CPU-independent and
// returns the same value for any CPU.
//
    max_pinned += bp_slots_histogram_max(&cpu_pinned[type], type);
    return max_pinned;
    }
    }
    for_each_cpu(cpu, cpumask) {
    let mut info = get_bp_info(cpu, type);
    let mut nr = 0;
    nr = info.cpu_pinned;
    if (!bp.hw.target) {
    nr += max_task_bp_pinned(cpu, type);
    }
    else {
    nr += task_bp_pinned(cpu, bp, type);
    }
    pinned_slots = max(nr, pinned_slots);
    }
    return pinned_slots;
    }
//
// Add/remove the given breakpoint in our constraint table
//
#[no_mangle]
pub unsafe extern "C" fn toggle_bp_slot(bp: *mut perf_event, enable: bool, type: bp_type_idx, weight: c_int) -> c_int {
    let mut cpu = 0;
    let mut next_tsk_pinned = 0;
    if (!enable) {
    weight = -weight;
    }
    if (!bp.hw.target) {
//
// Update the pinned CPU slots, in per-CPU bp_cpuinfo and in the
// global histogram.
//
    let mut info = get_bp_info(bp.cpu, type);
    lockdep_assert_held_write(&bp_cpuinfo_sem);
    bp_slots_histogram_add(&cpu_pinned[type], info.cpu_pinned, weight);
    info.cpu_pinned += weight;
    return 0;
    }
//
// If bp->hw.target, tsk_pinned is only modified, but not used
// otherwise. We can permit concurrent updates as long as there are no
// other uses: having acquired bp_cpuinfo_sem as a reader allows
// concurrent updates here. Uses of tsk_pinned will require acquiring
// bp_cpuinfo_sem as a writer to stabilize tsk_pinned's value.
//
    lockdep_assert_held_read(&bp_cpuinfo_sem);
//
// Update the pinned task slots, in per-CPU bp_cpuinfo and in the global
// histogram. We need to take care of 4 cases:
//
// 1. This breakpoint targets all CPUs (cpu < 0), and there may only
// exist other task breakpoints targeting all CPUs. In this case we
// can simply update the global slots histogram.
//
// 2. This breakpoint targets a specific CPU (cpu >= 0), but there may
// only exist other task breakpoints targeting all CPUs.
//
// a. On enable: remove the existing breakpoints from the global
// slots histogram and use the per-CPU histogram.
//
// b. On disable: re-insert the existing breakpoints into the global
// slots histogram and remove from per-CPU histogram.
//
// 3. Some other existing task breakpoints target specific CPUs. Only
// update the per-CPU slots histogram.
//
    if (!enable) {
//
// Remove before updating histograms so we can determine if this
// was the last task breakpoint for a specific CPU.
//
pub static mut ret: c_int = 0;
    if (ret) {
    return ret;
    }
    }
//
// Note: If !enable, next_tsk_pinned will not count the to-be-removed breakpoint.
//
    next_tsk_pinned = task_bp_pinned(-1, bp, type);
    if (next_tsk_pinned >= 0) {
    if (bp.cpu < 0) { /* Case 1: fast path */ {
    if (!enable)
    next_tsk_pinned += hw_breakpoint_weight(bp);
    }
    bp_slots_histogram_add(&tsk_pinned_all[type], next_tsk_pinned, weight);
    } else if (enable) { /* Case 2.a: slow path */ {
// Add existing to per-CPU histograms.
    for_each_possible_cpu(cpu) {
    }
    bp_slots_histogram_add(&get_bp_info(cpu, type).tsk_pinned,
    0, next_tsk_pinned);
    }
// Add this first CPU-pinned task breakpoint.
    bp_slots_histogram_add(&get_bp_info(bp.cpu, type).tsk_pinned,
    next_tsk_pinned, weight);
// Rebalance global task pinned histogram.
    bp_slots_histogram_add(&tsk_pinned_all[type], next_tsk_pinned,
    -next_tsk_pinned);
    } else { /* Case 2.b: slow path */
// Remove this last CPU-pinned task breakpoint.
    bp_slots_histogram_add(&get_bp_info(bp.cpu, type).tsk_pinned,
    next_tsk_pinned + hw_breakpoint_weight(bp), weight);
// Remove all from per-CPU histograms.
    for_each_possible_cpu(cpu) {
    bp_slots_histogram_add(&get_bp_info(cpu, type).tsk_pinned,
    next_tsk_pinned, -next_tsk_pinned);
    }
// Rebalance global task pinned histogram.
    bp_slots_histogram_add(&tsk_pinned_all[type], 0, next_tsk_pinned);
    }
    } else { /* Case 3: slow path */
    let mut cpumask = cpumask_of_bp(bp);
    for_each_cpu(cpu, cpumask) {
    next_tsk_pinned = task_bp_pinned(cpu, bp, type);
    if (!enable) {
    next_tsk_pinned += hw_breakpoint_weight(bp);
    }
    bp_slots_histogram_add(&get_bp_info(cpu, type).tsk_pinned,
    next_tsk_pinned, weight);
    }
    }
//
// Readers want a stable snapshot of the per-task breakpoint list.
//
    assert_bp_constraints_lock_held(bp);
    if (enable) {
    return rhltable_insert(&task_bps_ht, &bp.hw.bp_list, task_bps_ht_params);
    }
    return 0;
    }
//
// Constraints to check before allowing this new breakpoint counter.
//
// Note: Flexible breakpoints are currently unimplemented, but outlined in the
// below algorithm for completeness.  The implementation treats flexible as
// pinned due to no guarantee that we currently always schedule flexible events
// before a pinned event in a same CPU.
//
// == Non-pinned counter == (Considered as pinned for now)
//
// - If attached to a single cpu, check:
//
// (per_cpu(info->flexible, cpu) || (per_cpu(info->cpu_pinned, cpu)
// + max(per_cpu(info->tsk_pinned, cpu)))) < HBP_NUM
//
// -> If there are already non-pinned counters in this cpu, it means
// there is already a free slot for them.
// Otherwise, we check that the maximum number of per task
// breakpoints (for this cpu) plus the number of per cpu breakpoint
// (for this cpu) doesn't cover every registers.
//
// - If attached to every cpus, check:
//
// (per_cpu(info->flexible, *) || (max(per_cpu(info->cpu_pinned, *))
// + max(per_cpu(info->tsk_pinned, *)))) < HBP_NUM
//
// -> This is roughly the same, except we check the number of per cpu
// bp for every cpu and we keep the max one. Same for the per tasks
// breakpoints.
//
// == Pinned counter ==
//
// - If attached to a single cpu, check:
//
// ((per_cpu(info->flexible, cpu) > 1) + per_cpu(info->cpu_pinned, cpu)
// + max(per_cpu(info->tsk_pinned, cpu))) < HBP_NUM
//
// -> Same checks as before. But now the info->flexible, if any, must keep
// one register at least (or they will never be fed).
//
// - If attached to every cpus, check:
//
// ((per_cpu(info->flexible, *) > 1) + max(per_cpu(info->cpu_pinned, *))
// + max(per_cpu(info->tsk_pinned, *))) < HBP_NUM
//
#[no_mangle]
unsafe extern "C" fn __reserve_bp_slot(bp: *mut perf_event, bp_type: u64) -> c_int {
    enum bp_type_idx type;
    let mut max_pinned_slots = 0;
    let mut weight = 0;
// We couldn't initialize breakpoint constraints on boot
    if (!constraints_initialized) {
    return -ENOMEM;
    }
// Basic checks
    if (bp_type == HW_BREAKPOINT_EMPTY ||
    bp_type == HW_BREAKPOINT_INVALID) {
    return -EINVAL;
    }
    type = find_slot_idx(bp_type);
    weight = hw_breakpoint_weight(bp);
// Check if this new breakpoint can be satisfied across all CPUs.
    max_pinned_slots = max_bp_pinned_slots(bp, type) + weight;
    if (max_pinned_slots > hw_breakpoint_slots_cached(type)) {
    return -ENOSPC;
    }
    return toggle_bp_slot(bp, true, type, weight);
    }
#[no_mangle]
pub unsafe extern "C" fn reserve_bp_slot(bp: *mut perf_event) -> c_int {
    let mut mtx = bp_constraints_lock(bp);
pub static mut ret: c_int = 0;
    bp_constraints_unlock(mtx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __release_bp_slot(bp: *mut perf_event, bp_type: u64) {
    enum bp_type_idx type;
    let mut weight = 0;
    type = find_slot_idx(bp_type);
    weight = hw_breakpoint_weight(bp);
    WARN_ON!(toggle_bp_slot(bp, false, type, weight));
    }
#[no_mangle]
pub unsafe extern "C" fn release_bp_slot(bp: *mut perf_event) {
    let mut mtx = bp_constraints_lock(bp);
    __release_bp_slot(bp, bp.attr.bp_type);
    bp_constraints_unlock(mtx);
    }
#[no_mangle]
unsafe extern "C" fn __modify_bp_slot(bp: *mut perf_event, old_type: u64, new_type: u64) -> c_int {
    let mut err = 0;
    __release_bp_slot(bp, old_type);
    err = __reserve_bp_slot(bp, new_type);
    if (err) {
//
// Reserve the old_type slot back in case
// there's no space for the new type.
//
// This must succeed, because we just released
// the old_type slot in the __release_bp_slot
// call above. If not, something is broken.
//
    WARN_ON!(__reserve_bp_slot(bp, old_type));
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn modify_bp_slot(bp: *mut perf_event, old_type: u64, new_type: u64) -> c_int {
    let mut mtx = bp_constraints_lock(bp);
pub static mut ret: c_int = 0;
    bp_constraints_unlock(mtx);
    return ret;
    }
//
// Allow the kernel debugger to reserve breakpoint slots without
// taking a lock using the dbg_* variant of for the reserve and
// release breakpoint slots.
//
#[no_mangle]
pub unsafe extern "C" fn dbg_reserve_bp_slot(bp: *mut perf_event) -> c_int {
    let mut ret = 0;
    if (bp_constraints_is_locked(bp)) {
    return -1;
    }
// Locks aren't held; disable lockdep assert checking.
    lockdep_off();
    ret = __reserve_bp_slot(bp, bp.attr.bp_type);
    lockdep_on();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dbg_release_bp_slot(bp: *mut perf_event) -> c_int {
    if (bp_constraints_is_locked(bp)) {
    return -1;
    }
// Locks aren't held; disable lockdep assert checking.
    lockdep_off();
    __release_bp_slot(bp, bp.attr.bp_type);
    lockdep_on();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hw_breakpoint_parse(bp: *mut perf_event, attr: *mut perf_event_attr, hw: *mut arch_hw_breakpoint) -> c_int {
    let mut err = 0;
    err = hw_breakpoint_arch_parse(bp, attr, hw);
    if (err) {
    return err;
    }
    if (arch_check_bp_in_kernelspace(hw)) {
    if (attr.exclude_kernel) {
    return -EINVAL;
    }
//
// Don't let unprivileged users set a breakpoint in the trap
// path to avoid trap recursion attacks.
//
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn register_perf_hw_breakpoint(bp: *mut perf_event) -> c_int {
pub static mut hw: arch_hw_breakpoint = 0;
    let mut err = 0;
    err = reserve_bp_slot(bp);
    if (err) {
    return err;
    }
    err = hw_breakpoint_parse(bp, &bp.attr, &hw);
    if (err) {
    release_bp_slot(bp);
    return err;
    }
    bp.hw.info = hw;
    return 0;
    }
//
// register_user_hw_breakpoint - register a hardware breakpoint for user space
// @attr: breakpoint attributes
// @triggered: callback to trigger when we hit the breakpoint
// @context: context data could be used in the triggered callback
// @tsk: pointer to 'task_struct' of the process to which the address belongs
//
#[no_mangle]
pub unsafe extern "C" fn register_user_hw_breakpoint(attr: *mut perf_event_attr, triggered: perf_overflow_handler_t, context: *mut c_void, tsk: *mut task_struct) -> *mut c_void {
    return perf_event_create_kernel_counter(attr, -1, tsk, triggered,
    context);
    }
    EXPORT_SYMBOL_GPL(register_user_hw_breakpoint);
#[no_mangle]
pub unsafe extern "C" fn hw_breakpoint_copy_attr(to: *mut perf_event_attr, from: *mut perf_event_attr) {
    to.bp_addr = from.bp_addr;
    to.bp_type = from.bp_type;
    to.bp_len  = from.bp_len;
    to.disabled = from.disabled;
    }
#[no_mangle]
pub unsafe extern "C" fn modify_user_hw_breakpoint_check(bp: *mut perf_event, attr: *mut perf_event_attr, check: bool) -> c_int {
pub static mut hw: arch_hw_breakpoint = 0;
    let mut err = 0;
    err = hw_breakpoint_parse(bp, attr, &hw);
    if (err) {
    return err;
    }
    if (check) {
pub static mut old_attr: usize = 0;
    old_attr = bp.attr;
    hw_breakpoint_copy_attr(&old_attr, attr);
    if (memcmp(&old_attr, attr, sizeof!(*attr))) {
    return -EINVAL;
    }
    }
    if (bp.attr.bp_type != attr.bp_type) {
    err = modify_bp_slot(bp, bp.attr.bp_type, attr.bp_type);
    if (err) {
    return err;
    }
    }
    hw_breakpoint_copy_attr(&bp.attr, attr);
    bp.hw.info = hw;
    return 0;
    }
//
// modify_user_hw_breakpoint - modify a user-space hardware breakpoint
// @bp: the breakpoint structure to modify
// @attr: new breakpoint attributes
//
#[no_mangle]
pub unsafe extern "C" fn modify_user_hw_breakpoint(bp: *mut perf_event, attr: *mut perf_event_attr) -> c_int {
    let mut err = 0;
//
// modify_user_hw_breakpoint can be invoked with IRQs disabled and hence it
// will not be possible to raise IPIs that invoke __perf_event_disable.
// So call the function directly after making sure we are targeting the
// current task.
//
    if (irqs_disabled() && bp.ctx && bp.ctx.task == current) {
    perf_event_disable_local(bp);
    }
    else {
    perf_event_disable(bp);
    }
    err = modify_user_hw_breakpoint_check(bp, attr, false);
    if (!bp.attr.disabled) {
    perf_event_enable(bp);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(modify_user_hw_breakpoint);
//
// unregister_hw_breakpoint - unregister a user-space hardware breakpoint
// @bp: the breakpoint structure to unregister
//
#[no_mangle]
pub unsafe extern "C" fn unregister_hw_breakpoint(bp: *mut perf_event) {
    if (!bp) {
    return;
    }
    perf_event_release_kernel(bp);
    }
    EXPORT_SYMBOL_GPL(unregister_hw_breakpoint);
//
// register_wide_hw_breakpoint - register a wide breakpoint in the kernel
// @attr: breakpoint attributes
// @triggered: callback to trigger when we hit the breakpoint
// @context: context data could be used in the triggered callback
//
// @return a set of per_cpu pointers to perf events
//
    struct perf_event *  *
    register_wide_hw_breakpoint(perf_event_attr *attr,
    perf_overflow_handler_t triggered,
    void *context)
    {
    struct perf_event *  *cpu_events, *bp;
pub static mut err: c_long = 0;
    let mut cpu = 0;
    cpu_events = alloc_percpu(typeof(*cpu_events));
    if (!cpu_events) {
    return ERR_PTR_PCPU(-ENOMEM);
    }
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    bp = perf_event_create_kernel_counter(attr, cpu, core::ptr::null_mut(),
    triggered, context);
    if (IS_ERR(bp)) {
    err = PTR_ERR(bp);
    break;
    }
    per_cpu(*cpu_events, cpu) = bp;
    }
    cpus_read_unlock();
    if (likely(!err)) {
    return cpu_events;
    }
    unregister_wide_hw_breakpoint(cpu_events);
    return ERR_PTR_PCPU(err);
    }
    EXPORT_SYMBOL_GPL(register_wide_hw_breakpoint);
//
// unregister_wide_hw_breakpoint - unregister a wide breakpoint in the kernel
// @cpu_events: the per cpu set of events to unregister
//
#[no_mangle]
pub unsafe extern "C" fn unregister_wide_hw_breakpoint(cpu_events: *mut *mut perf_event  ) {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    unregister_hw_breakpoint(per_cpu(*cpu_events, cpu));
    }
    free_percpu(cpu_events);
    }
    EXPORT_SYMBOL_GPL(unregister_wide_hw_breakpoint);
//
// hw_breakpoint_is_used - check if breakpoints are currently used
//
// Returns: true if breakpoints are used, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn hw_breakpoint_is_used() -> bool {
    let mut cpu = 0;
    if (!constraints_initialized) {
    return false;
    }
    for_each_possible_cpu(cpu) {
    while (type < TYPE_MAX) {
    let mut info = get_bp_info(cpu, type);
    if (info.cpu_pinned) {
    return true;
    }
    while (slot < hw_breakpoint_slots_cached(type)) {
    if (atomic_read(&info.tsk_pinned.count[slot])) {
    return true;
    }
    }
    }
    }
    while (type < TYPE_MAX) {
    while (slot < hw_breakpoint_slots_cached(type)) {
//
// Warn, because if there are CPU pinned counters,
// should never get here; bp_cpuinfo::cpu_pinned should
// be consistent with the global cpu_pinned histogram.
//
    if (WARN_ON!(atomic_read(&cpu_pinned[type].count[slot]))) {
    return true;
    }
    if (atomic_read(&tsk_pinned_all[type].count[slot])) {
    return true;
    }
    }
    }
    return false;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn bp_perf_event_destroy(event: *mut perf_event) {
    release_bp_slot(event);
    }
#[no_mangle]
unsafe extern "C" fn hw_breakpoint_event_init(bp: *mut perf_event) -> c_int {
    let mut err = 0;
    if (bp.attr.type != PERF_TYPE_BREAKPOINT) {
    return -ENOENT;
    }
//
// Check if breakpoint type is supported before proceeding.
// Also, no branch sampling for breakpoint events.
//
    if (!hw_breakpoint_slots_cached(find_slot_idx(bp.attr.bp_type)) || has_branch_stack(bp)) {
    return -EOPNOTSUPP;
    }
    err = register_perf_hw_breakpoint(bp);
    if (err) {
    return err;
    }
    bp.destroy = bp_perf_event_destroy;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hw_breakpoint_add(bp: *mut perf_event, flags: c_int) -> c_int {
    if (!(flags & PERF_EF_START)) {
    bp.hw.state = PERF_HES_STOPPED;
    }
    if (is_sampling_event(bp)) {
    bp.hw.last_period = bp.hw.sample_period;
    perf_swevent_set_period(bp);
    }
    return arch_install_hw_breakpoint(bp);
    }
#[no_mangle]
unsafe extern "C" fn hw_breakpoint_del(bp: *mut perf_event, flags: c_int) {
    arch_uninstall_hw_breakpoint(bp);
    }
#[no_mangle]
unsafe extern "C" fn hw_breakpoint_start(bp: *mut perf_event, flags: c_int) {
    bp.hw.state = 0;
    }
#[no_mangle]
unsafe extern "C" fn hw_breakpoint_stop(bp: *mut perf_event, flags: c_int) {
    bp.hw.state = PERF_HES_STOPPED;
    }
pub static mut pmu: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_hw_breakpoint() -> c_int {
    let mut ret = 0;
    ret = rhltable_init(&task_bps_ht, &task_bps_ht_params);
    if (ret) {
    return ret;
    }
    ret = init_breakpoint_slots();
    if (ret) {
    return ret;
    }
    constraints_initialized = true;
    perf_pmu_register(&perf_breakpoint, "breakpoint", PERF_TYPE_BREAKPOINT);
    return register_die_notifier(&hw_breakpoint_exceptions_nb);
    }