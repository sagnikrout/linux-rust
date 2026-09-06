//! Automatically rewritten from C to Rust
//! Source: kernel/sched/swait.c
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
// <linux/swait.h> (simple wait queues ) implementation:
//

    void __init_swait_queue_head(struct swait_queue_head *q, const char *name,
    struct lock_class_key *key)
    {
    raw_spin_lock_init(&q.lock);
    lockdep_set_class_and_name(&q.lock, key, name);
    INIT_LIST_HEAD(&q.task_list);
    }
    EXPORT_SYMBOL(__init_swait_queue_head);
//
// The thing about the wake_up_state() return value; I think we can ignore it.
//
// If for some reason it would return 0, that means the previously waiting
// task is already running, so it will observe condition true (or has already).
//
#[no_mangle]
pub unsafe extern "C" fn swake_up_locked(q: *mut swait_queue_head, wake_flags: c_int) {
    void swake_up_locked(struct swait_queue_head *q, int wake_flags)
    {
    struct swait_queue *curr;
    if (list_empty(&q.task_list))
    return;
    curr = list_first_entry(&q.task_list, typeof(*curr), task_list);
    try_to_wake_up(curr.task, TASK_NORMAL, wake_flags);
    list_del_init(&curr.task_list);
    }
    EXPORT_SYMBOL(swake_up_locked);
//
// Wake up all waiters. This is an interface which is solely exposed for
// completions and not for general usage.
//
// It is intentionally different from swake_up_all() to allow usage from
// hard interrupt context and interrupt disabled regions.
//
#[no_mangle]
pub unsafe extern "C" fn swake_up_all_locked(q: *mut swait_queue_head) {
    void swake_up_all_locked(struct swait_queue_head *q)
    {
    while (!list_empty(&q.task_list))
    swake_up_locked(q, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn swake_up_one(q: *mut swait_queue_head) {
    void swake_up_one(struct swait_queue_head *q)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&q.lock, flags);
    swake_up_locked(q, 0);
    raw_spin_unlock_irqrestore(&q.lock, flags);
    }
    EXPORT_SYMBOL(swake_up_one);
//
// Does not allow usage from IRQ disabled, since we must be able to
// release IRQs to guarantee bounded hold time.
//
#[no_mangle]
pub unsafe extern "C" fn swake_up_all(q: *mut swait_queue_head) {
    void swake_up_all(struct swait_queue_head *q)
    {
    struct swait_queue *curr;
    LIST_HEAD(tmp);
    raw_spin_lock_irq(&q.lock);
    list_splice_init(&q.task_list, &tmp);
    while (!list_empty(&tmp)) {
    curr = list_first_entry(&tmp, typeof(*curr), task_list);
    wake_up_state(curr.task, TASK_NORMAL);
    list_del_init(&curr.task_list);
    if (list_empty(&tmp))
    break;
    raw_spin_unlock_irq(&q.lock);
    raw_spin_lock_irq(&q.lock);
    }
    raw_spin_unlock_irq(&q.lock);
    }
    EXPORT_SYMBOL(swake_up_all);
#[no_mangle]
pub unsafe extern "C" fn __prepare_to_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    void __prepare_to_swait(struct swait_queue_head *q, struct swait_queue *wait)
    {
    wait.task = current;
    if (list_empty(&wait.task_list))
    list_add_tail(&wait.task_list, &q.task_list);
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_to_swait_exclusive(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) {
    void prepare_to_swait_exclusive(struct swait_queue_head *q, struct swait_queue *wait, int state)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&q.lock, flags);
    __prepare_to_swait(q, wait);
    set_current_state(state);
    raw_spin_unlock_irqrestore(&q.lock, flags);
    }
    EXPORT_SYMBOL(prepare_to_swait_exclusive);
#[no_mangle]
pub unsafe extern "C" fn prepare_to_swait_event(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) -> c_long {
    long prepare_to_swait_event(struct swait_queue_head *q, struct swait_queue *wait, int state)
    {
    unsigned long flags;
    let mut ret: c_long = 0;
    raw_spin_lock_irqsave(&q.lock, flags);
    if (signal_pending_state(state, current)) {
//
// See prepare_to_wait_event(). TL;DR, subsequent swake_up_one()
// must not see us.
//
    list_del_init(&wait.task_list);
    ret = -ERESTARTSYS;
    } else {
    __prepare_to_swait(q, wait);
    set_current_state(state);
    }
    raw_spin_unlock_irqrestore(&q.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(prepare_to_swait_event);
#[no_mangle]
pub unsafe extern "C" fn __finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    void __finish_swait(struct swait_queue_head *q, struct swait_queue *wait)
    {
    __set_current_state(TASK_RUNNING);
    if (!list_empty(&wait.task_list))
    list_del_init(&wait.task_list);
    }
#[no_mangle]
pub unsafe extern "C" fn finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    void finish_swait(struct swait_queue_head *q, struct swait_queue *wait)
    {
    unsigned long flags;
    __set_current_state(TASK_RUNNING);
    if (!list_empty_careful(&wait.task_list)) {
    raw_spin_lock_irqsave(&q.lock, flags);
    list_del_init(&wait.task_list);
    raw_spin_unlock_irqrestore(&q.lock, flags);
    }
    }
    EXPORT_SYMBOL(finish_swait);
