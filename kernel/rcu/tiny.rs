//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/tiny.c
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
// Read-Copy Update mechanism for mutual exclusion, the Bloatwatch edition.
//
// Copyright IBM Corporation, 2008
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//
// For detailed explanation of Read-Copy Update mechanism see -
// Documentation/RCU
//

// Global control variables for rcupdate callback mechanism.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_ctrlblk {
    pub /: *mut *mut *mut rcu_head rcucblist; / List of pending callbacks (CBs).,
    pub /: *mut *mut *mut *mut rcu_head donetail; / ->next pointer of last "done" CB.,
    pub /: *mut *mut *mut *mut rcu_head curtail; / ->next pointer of last CB.,
    pub /: *mut *mut unsigned long gp_seq; / Grace-period counter.,
}

// Definition for rcupdate control block.
    static struct rcu_ctrlblk rcu_ctrlblk = {
    .donetail	= &rcu_ctrlblk.rcucblist,
    .curtail	= &rcu_ctrlblk.rcucblist,
    .gp_seq		= 0 - 300UL,
    };
#[no_mangle]
pub unsafe extern "C" fn rcu_barrier() {
    void rcu_barrier(void)
    {
    wait_rcu_gp(call_rcu_hurry);
    }
    EXPORT_SYMBOL(rcu_barrier);
// Record an rcu quiescent state.
#[no_mangle]
pub unsafe extern "C" fn rcu_qs() {
    void rcu_qs(void)
    {
    unsigned long flags;
    local_irq_save(flags);
    if (rcu_ctrlblk.donetail != rcu_ctrlblk.curtail) {
    rcu_ctrlblk.donetail = rcu_ctrlblk.curtail;
    raise_softirq_irqoff(RCU_SOFTIRQ);
    }
    WRITE_ONCE(rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq + 2);
    local_irq_restore(flags);
    }
//
// Check to see if the scheduling-clock interrupt came from an extended
// quiescent state, and, if so, tell RCU about it.  This function must
// be called from hardirq context.  It is normally called from the
// scheduling-clock interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sched_clock_irq(user: c_int) {
    void rcu_sched_clock_irq(int user)
    {
    if (user)
    rcu_qs();
#[no_mangle]
pub unsafe extern "C" fn if(rcu_ctrlblk.curtail: rcu_ctrlblk.donetail !=) -> else {
    else if (rcu_ctrlblk.donetail != rcu_ctrlblk.curtail)
    set_need_resched_current();
    }
//
// Reclaim the specified callback, either by invoking it for non-kfree cases or
// freeing it directly (for kfree). Return true if kfreeing, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_reclaim_tiny(head: *mut rcu_head) -> bool {
    static inline bool rcu_reclaim_tiny(struct rcu_head *head)
    {
    rcu_callback_t f;
    rcu_lock_acquire(&rcu_callback_map);
    trace_rcu_invoke_callback("", head);
    f = head.func;
    debug_rcu_head_callback(head);
    WRITE_ONCE(head.func, (rcu_callback_t)0L);
    f(head);
    rcu_lock_release(&rcu_callback_map);
    return false;
    }
// Invoke the RCU callbacks whose grace period has elapsed.
#[no_mangle]
unsafe extern "C" fn rcu_process_callbacks() -> __latent_entropy void {
    static __latent_entropy void rcu_process_callbacks(void)
    {
    struct rcu_head *next, *list;
    unsigned long flags;
// Move the ready-to-invoke callbacks to a local list.
    local_irq_save(flags);
    if (rcu_ctrlblk.donetail == &rcu_ctrlblk.rcucblist) {
// No callbacks ready, so just leave.
    local_irq_restore(flags);
    return;
    }
    list = rcu_ctrlblk.rcucblist;
    rcu_ctrlblk.rcucblist = *rcu_ctrlblk.donetail;
// rcu_ctrlblk.donetail = NULL;
    if (rcu_ctrlblk.curtail == rcu_ctrlblk.donetail)
    rcu_ctrlblk.curtail = &rcu_ctrlblk.rcucblist;
    rcu_ctrlblk.donetail = &rcu_ctrlblk.rcucblist;
    local_irq_restore(flags);
// Invoke the callbacks on the local list.
    while (list) {
    next = list.next;
    prefetch(next);
    debug_rcu_head_unqueue(list);
    rcu_reclaim_tiny(list);
    list = next;
    }
    }
//
// Wait for a grace period to elapse.  But it is illegal to invoke
// synchronize_rcu() from within an RCU read-side critical section.
// Therefore, any legal call to synchronize_rcu() is a quiescent state,
// and so on a UP system, synchronize_rcu() need do nothing, other than
// let the polled APIs know that another grace period elapsed.
//
// (But Lai Jiangshan points out the benefits of doing might_sleep()
// to reduce latency.)
//
// Cool, huh?  (Due to Josh Triplett.)
//
#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu() {
    void synchronize_rcu(void)
    {
    RCU_LOCKDEP_WARN(lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_sched_lock_map),
    "Illegal synchronize_rcu() in RCU read-side critical section");
    preempt_disable();
    WRITE_ONCE(rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq + 2);
    preempt_enable();
    }
    EXPORT_SYMBOL_GPL(synchronize_rcu);
//
// Post an RCU callback to be invoked after the end of an RCU grace
// period.  But since we have but one CPU, that would be after any
// quiescent state.
//
#[no_mangle]
pub unsafe extern "C" fn call_rcu(head: *mut rcu_head, func: rcu_callback_t) {
    void call_rcu(struct rcu_head *head, rcu_callback_t func)
    {
    static atomic_t doublefrees;
    unsigned long flags;
    if (debug_rcu_head_queue(head)) {
    if (atomic_inc_return(&doublefrees) < 4) {
    pr_err("%s(): Double-freed CB %p.%pS()!!!  ", __func__, head, head.func);
    mem_dump_obj(head);
    }
    return;
    }
    head.func = func;
    head.next = core::ptr::null_mut();
    local_irq_save(flags);
// rcu_ctrlblk.curtail = head;
    rcu_ctrlblk.curtail = &head.next;
    local_irq_restore(flags);
    if (unlikely(is_idle_task(current))) {
// force scheduling for rcu_qs()
    resched_cpu(0);
    }
    }
    EXPORT_SYMBOL_GPL(call_rcu);
//
// Store a grace-period-counter "cookie".  For more information,
// see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn get_completed_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    void get_completed_synchronize_rcu_full(struct rcu_gp_seq *gsp)
    {
    gsp.norm = RCU_GET_STATE_COMPLETED;
    }
    EXPORT_SYMBOL_GPL(get_completed_synchronize_rcu_full);
//
// Return a grace-period-counter "cookie".  For more information,
// see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn get_state_synchronize_rcu() -> c_ulong {
    unsigned long get_state_synchronize_rcu(void)
    {
    return READ_ONCE(rcu_ctrlblk.gp_seq);
    }
    EXPORT_SYMBOL_GPL(get_state_synchronize_rcu);
//
// Return a grace-period-counter "cookie" and ensure that a future grace
// period completes.  For more information, see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn start_poll_synchronize_rcu() -> c_ulong {
    unsigned long start_poll_synchronize_rcu(void)
    {
    let mut gp_seq: c_ulong = get_state_synchronize_rcu();
    if (unlikely(is_idle_task(current))) {
// force scheduling for rcu_qs()
    resched_cpu(0);
    }
    return gp_seq;
    }
    EXPORT_SYMBOL_GPL(start_poll_synchronize_rcu);
//
// Return true if the grace period corresponding to oldstate has completed
// and false otherwise.  For more information, see the Tree RCU header
// comment.
//
#[no_mangle]
pub unsafe extern "C" fn poll_state_synchronize_rcu(oldstate: c_ulong) -> bool {
    bool poll_state_synchronize_rcu(unsigned long oldstate)
    {
    let mut oldstate: return = = RCU_GET_STATE_COMPLETED || READ_ONCE(rcu_ctrlblk.gp_seq) != oldstate;
    }
    EXPORT_SYMBOL_GPL(poll_state_synchronize_rcu);

#[no_mangle]
pub unsafe extern "C" fn rcutorture_gather_gp_seqs() -> c_ulonglong {
    unsigned long long rcutorture_gather_gp_seqs(void)
    {
    return READ_ONCE(rcu_ctrlblk.gp_seq) & 0xffffULL;
    }
    EXPORT_SYMBOL_GPL(rcutorture_gather_gp_seqs);
#[no_mangle]
pub unsafe extern "C" fn rcutorture_format_gp_seqs(seqs: c_ulonglong, cp: *mut c_char, len: usize) {
    void rcutorture_format_gp_seqs(unsigned long long seqs, char *cp, size_t len)
    {
    snprintf(cp, len, "g%04llx", seqs & 0xffffULL);
    }
    EXPORT_SYMBOL_GPL(rcutorture_format_gp_seqs);

#[no_mangle]
pub unsafe extern "C" fn rcu_init() -> void __init {
    void __init rcu_init(void)
    {
    open_softirq(RCU_SOFTIRQ, rcu_process_callbacks);
    rcu_early_boot_tests();
    tasks_cblist_init_generic();
    }
