//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/srcutiny.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Sleepable Read-Copy Update mechanism for mutual exclusion,
// tiny version for non-preemptible single-CPU use.
//
// Copyright (C) IBM Corporation, 2017
//
// Author: Paul McKenney <paulmck@linux.ibm.com>
//

    int rcu_scheduler_active __read_mostly;

    extern int rcu_scheduler_active;

    static LIST_HEAD(srcu_boot_list);
    static bool srcu_init_done;
#[no_mangle]
unsafe extern "C" fn init_srcu_struct_fields(ssp: *mut srcu_struct) -> c_int {
    static int init_srcu_struct_fields(struct srcu_struct *ssp)
    {
    ssp.srcu_lock_nesting[0] = 0;
    ssp.srcu_lock_nesting[1] = 0;
    init_swait_queue_head(&ssp.srcu_wq);
    ssp.srcu_cb_head = core::ptr::null_mut();
    ssp.srcu_cb_tail = &ssp.srcu_cb_head;
    ssp.srcu_gp_running = false;
    ssp.srcu_gp_waiting = false;
    ssp.srcu_idx = 0;
    ssp.srcu_idx_max = 0;
    INIT_WORK(&ssp.srcu_work, srcu_drive_gp);
    INIT_LIST_HEAD(&ssp.srcu_work.entry);
    init_irq_work(&ssp.srcu_irq_work, srcu_tiny_irq_work);
    return 0;
    }

    int init_srcu_struct_lockdep(struct srcu_struct *ssp, const char *name,
    struct lock_class_key *key)
    {
// Don't re-initialize a lock while it is held.
    debug_check_no_locks_freed((void *)ssp, sizeof(*ssp));
    lockdep_init_map(&ssp.dep_map, name, key, 0);
    return init_srcu_struct_fields(ssp);
    }
    EXPORT_SYMBOL_GPL(init_srcu_struct_lockdep);

//
// init_srcu_struct_generic - initialize a sleep-RCU structure
// @ssp: structure to initialize.
//
// Must invoke this on a given srcu_struct before passing that srcu_struct
// to any other function.  Each srcu_struct represents a separate domain
// of SRCU protection.
//
#[no_mangle]
pub unsafe extern "C" fn init_srcu_struct_generic(ssp: *mut srcu_struct) -> c_int {
    int init_srcu_struct_generic(struct srcu_struct *ssp)
    {
    return init_srcu_struct_fields(ssp);
    }
    EXPORT_SYMBOL_GPL(init_srcu_struct_generic);

//
// cleanup_srcu_struct - deconstruct a sleep-RCU structure
// @ssp: structure to clean up.
//
// Must invoke this after you are finished using a given srcu_struct that
// was initialized via init_srcu_struct(), else you leak memory.
//
#[no_mangle]
pub unsafe extern "C" fn cleanup_srcu_struct(ssp: *mut srcu_struct) {
    void cleanup_srcu_struct(struct srcu_struct *ssp)
    {
    WARN_ON(srcu_readers_active(ssp));
    irq_work_sync(&ssp.srcu_irq_work);
    flush_work(&ssp.srcu_work);
    WARN_ON(ssp.srcu_gp_running);
    WARN_ON(ssp.srcu_gp_waiting);
    WARN_ON(ssp.srcu_cb_head);
    WARN_ON(&ssp.srcu_cb_head != ssp.srcu_cb_tail);
    WARN_ON(ssp.srcu_idx != ssp.srcu_idx_max);
    WARN_ON(ssp.srcu_idx & 0x1);
    }
    EXPORT_SYMBOL_GPL(cleanup_srcu_struct);
//
// Removes the count for the old reader from the appropriate element of
// the srcu_struct.
//
#[no_mangle]
pub unsafe extern "C" fn __srcu_read_unlock(ssp: *mut srcu_struct, idx: c_int) {
    void __srcu_read_unlock(struct srcu_struct *ssp, int idx)
    {
    int newval;
    preempt_disable();  // Needed for PREEMPT_LAZY
    newval = READ_ONCE(ssp.srcu_lock_nesting[idx]) - 1;
    WRITE_ONCE(ssp.srcu_lock_nesting[idx], newval);
    preempt_enable();
    if (!newval && READ_ONCE(ssp.srcu_gp_waiting) && in_task() && !irqs_disabled())
    swake_up_one(&ssp.srcu_wq);
    }
    EXPORT_SYMBOL_GPL(__srcu_read_unlock);
//
// Workqueue handler to drive one grace period and invoke any callbacks
// that become ready as a result.  Single-CPU operation and preemption
// disabling mean that we get away with murder on synchronization.  ;-)
//
#[no_mangle]
pub unsafe extern "C" fn srcu_drive_gp(wp: *mut work_struct) {
    void srcu_drive_gp(struct work_struct *wp)
    {
    int idx;
    struct rcu_head *lh;
    struct rcu_head *rhp;
    struct srcu_struct *ssp;
    ssp = container_of(wp, struct srcu_struct, srcu_work);
    preempt_disable();  // Needed for PREEMPT_LAZY
    if (ssp.srcu_gp_running || ULONG_CMP_GE(ssp.srcu_idx, READ_ONCE(ssp.srcu_idx_max))) {
    preempt_enable();
    return; /* Already running or nothing to do. */
    }
// Remove recently arrived callbacks and wait for readers.
    WRITE_ONCE(ssp.srcu_gp_running, true);
    local_irq_disable();
    lh = ssp.srcu_cb_head;
    ssp.srcu_cb_head = core::ptr::null_mut();
    ssp.srcu_cb_tail = &ssp.srcu_cb_head;
    local_irq_enable();
    idx = (ssp.srcu_idx & 0x2) / 2;
    WRITE_ONCE(ssp.srcu_idx, ssp.srcu_idx + 1);
    WRITE_ONCE(ssp.srcu_gp_waiting, true);  /* srcu_read_unlock() wakes! */
    preempt_enable();
    do {
// Deadlock issues prevent __srcu_read_unlock() from
// doing an unconditional wakeup, so polling is required.
    swait_event_timeout_exclusive(ssp.srcu_wq,
    !READ_ONCE(ssp.srcu_lock_nesting[idx]), HZ / 10);
    } while (READ_ONCE(ssp.srcu_lock_nesting[idx]));
    preempt_disable();  // Needed for PREEMPT_LAZY
    WRITE_ONCE(ssp.srcu_gp_waiting, false); /* srcu_read_unlock() cheap. */
    WRITE_ONCE(ssp.srcu_idx, ssp.srcu_idx + 1);
    preempt_enable();
// Invoke the callbacks we removed above.
    while (lh) {
    rhp = lh;
    lh = lh.next;
    debug_rcu_head_callback(rhp);
    local_bh_disable();
    rhp.func(rhp);
    local_bh_enable();
    }
//
// Enable rescheduling, and if there are more callbacks,
// reschedule ourselves.  This can race with a call_srcu()
// at interrupt level, but the ->srcu_gp_running checks will
// straighten that out.
//
    preempt_disable();  // Needed for PREEMPT_LAZY
    WRITE_ONCE(ssp.srcu_gp_running, false);
    idx = ULONG_CMP_LT(ssp.srcu_idx, READ_ONCE(ssp.srcu_idx_max));
    preempt_enable();
    if (idx)
    schedule_work(&ssp.srcu_work);
    }
    EXPORT_SYMBOL_GPL(srcu_drive_gp);
//
// Use an irq_work to defer schedule_work() to avoid acquiring the workqueue
// pool->lock while the caller might hold scheduler locks, causing lockdep
// splats due to workqueue_init() doing a wakeup.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_tiny_irq_work(irq_work: *mut irq_work) {
    void srcu_tiny_irq_work(struct irq_work *irq_work)
    {
    struct srcu_struct *ssp;
    ssp = container_of(irq_work, struct srcu_struct, srcu_irq_work);
    schedule_work(&ssp.srcu_work);
    }
    EXPORT_SYMBOL_GPL(srcu_tiny_irq_work);
#[no_mangle]
unsafe extern "C" fn srcu_gp_start_if_needed(ssp: *mut srcu_struct) {
    static void srcu_gp_start_if_needed(struct srcu_struct *ssp)
    {
    unsigned long cookie;
    lockdep_assert_preemption_disabled(); // Needed for PREEMPT_LAZY
    cookie = get_state_synchronize_srcu(ssp);
    if (ULONG_CMP_GE(READ_ONCE(ssp.srcu_idx_max), cookie)) {
    return;
    }
    WRITE_ONCE(ssp.srcu_idx_max, cookie);
    if (!READ_ONCE(ssp.srcu_gp_running)) {
    if (likely(srcu_init_done))
    irq_work_queue(&ssp.srcu_irq_work);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: list_empty(&ssp->srcu_work.entry)) -> else {
    else if (list_empty(&ssp.srcu_work.entry))
    list_add(&ssp.srcu_work.entry, &srcu_boot_list);
    }
    }
//
// Enqueue an SRCU callback on the specified srcu_struct structure,
// initiating grace-period processing if it is not already running.
//
    void call_srcu(struct srcu_struct *ssp, struct rcu_head *rhp,
    rcu_callback_t func)
    {
    unsigned long flags;
    rhp.func = func;
    rhp.next = core::ptr::null_mut();
    preempt_disable();  // Needed for PREEMPT_LAZY
    local_irq_save(flags);
// ssp->srcu_cb_tail = rhp;
    ssp.srcu_cb_tail = &rhp.next;
    local_irq_restore(flags);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
    }
    EXPORT_SYMBOL_GPL(call_srcu);
//
// synchronize_srcu - wait for prior SRCU read-side critical-section completion
//
#[no_mangle]
pub unsafe extern "C" fn synchronize_srcu(ssp: *mut srcu_struct) {
    void synchronize_srcu(struct srcu_struct *ssp)
    {
    struct rcu_synchronize rs;
    srcu_lock_sync(&ssp.dep_map);
    RCU_LOCKDEP_WARN(lockdep_is_held(ssp) ||
    lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_sched_lock_map),
    "Illegal synchronize_srcu() in same-type SRCU (or in RCU) read-side critical section");
    if (rcu_scheduler_active == RCU_SCHEDULER_INACTIVE)
    return;
    might_sleep();
    init_rcu_head_on_stack(&rs.head);
    init_completion(&rs.completion);
    call_srcu(ssp, &rs.head, wakeme_after_rcu);
    wait_for_completion(&rs.completion);
    destroy_rcu_head_on_stack(&rs.head);
    }
    EXPORT_SYMBOL_GPL(synchronize_srcu);
//
// get_state_synchronize_srcu - Provide an end-of-grace-period cookie
//
#[no_mangle]
pub unsafe extern "C" fn get_state_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong {
    unsigned long get_state_synchronize_srcu(struct srcu_struct *ssp)
    {
    unsigned long ret;
    barrier();
    ret = (READ_ONCE(ssp.srcu_idx) + 3) & ~0x1;
    barrier();
    return ret;
    }
    EXPORT_SYMBOL_GPL(get_state_synchronize_srcu);
//
// start_poll_synchronize_srcu - Provide cookie and start grace period
//
// The difference between this and get_state_synchronize_srcu() is that
// this function ensures that the poll_state_synchronize_srcu() will
// eventually return the value true.
//
#[no_mangle]
pub unsafe extern "C" fn start_poll_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong {
    unsigned long start_poll_synchronize_srcu(struct srcu_struct *ssp)
    {
    unsigned long ret;
    preempt_disable();  // Needed for PREEMPT_LAZY
    ret = get_state_synchronize_srcu(ssp);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
    return ret;
    }
    EXPORT_SYMBOL_GPL(start_poll_synchronize_srcu);
//
// poll_state_synchronize_srcu - Has cookie's grace period ended?
//
#[no_mangle]
pub unsafe extern "C" fn poll_state_synchronize_srcu(ssp: *mut srcu_struct, cookie: c_ulong) -> bool {
    bool poll_state_synchronize_srcu(struct srcu_struct *ssp, unsigned long cookie)
    {
    let mut cur_s: c_ulong = READ_ONCE(ssp.srcu_idx);
    barrier();
    return cookie == SRCU_GET_STATE_COMPLETED ||
    ULONG_CMP_GE(cur_s, cookie) || ULONG_CMP_LT(cur_s, cookie - 3);
    }
    EXPORT_SYMBOL_GPL(poll_state_synchronize_srcu);

// Lockdep diagnostics.
#[no_mangle]
pub unsafe extern "C" fn rcu_scheduler_starting() -> void __init {
    void __init rcu_scheduler_starting(void)
    {
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
    }

//
// Queue work for srcu_struct structures with early boot callbacks.
// The work won't actually execute until the workqueue initialization
// phase that takes place after the scheduler starts.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_init() -> void __init {
    void __init srcu_init(void)
    {
    struct srcu_struct *ssp;
    srcu_init_done = true;
    while (!list_empty(&srcu_boot_list)) {
    ssp = list_first_entry(&srcu_boot_list,
    struct srcu_struct, srcu_work.entry);
    list_del_init(&ssp.srcu_work.entry);
    schedule_work(&ssp.srcu_work);
    }
    }
