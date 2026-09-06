//! Automatically rewritten from C to Rust
//! Source: lib/closure.c
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
// Asynchronous refcounty things
//
// Copyright 2010, 2011 Kent Overstreet <kent.overstreet@gmail.com>
// Copyright 2012 Google, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn closure_put_after_sub_checks(flags: c_int) {
    static inline void closure_put_after_sub_checks(int flags)
    {
    let mut r: c_int = flags & CLOSURE_REMAINING_MASK;
    if (WARN(flags & CLOSURE_GUARD_MASK,
    "closure has guard bits set: %x (%u)",
    flags & CLOSURE_GUARD_MASK, (unsigned) __fls(r)))
    r &= ~CLOSURE_GUARD_MASK;
    WARN(!r && (flags & ~CLOSURE_DESTRUCTOR),
    "closure ref hit 0 with incorrect flags set: %x (%u)",
    flags & ~CLOSURE_DESTRUCTOR, (unsigned) __fls(flags));
    }
#[no_mangle]
pub unsafe extern "C" fn closure_put_after_sub(cl: *mut closure, flags: c_int) {
    static inline void closure_put_after_sub(struct closure *cl, int flags)
    {
    closure_put_after_sub_checks(flags);
    if (!(flags & CLOSURE_REMAINING_MASK)) {
    smp_acquire__after_ctrl_dep();
    cl.closure_get_happened = false;
    if (cl.fn && !(flags & CLOSURE_DESTRUCTOR)) {
    atomic_set(&cl.remaining,
    CLOSURE_REMAINING_INITIALIZER);
    closure_queue(cl);
    } else {
    struct closure *parent = cl.parent;
    closure_fn *destructor = cl.fn;
    closure_debug_destroy(cl);
    if (destructor)
    destructor(&cl.work);
    if (parent)
    closure_put(parent);
    }
    }
    }
// For clearing flags with the same atomic op as a put
#[no_mangle]
pub unsafe extern "C" fn closure_sub(cl: *mut closure, v: c_int) {
    void closure_sub(struct closure *cl, int v)
    {
    closure_put_after_sub(cl, atomic_sub_return_release(v, &cl.remaining));
    }
    EXPORT_SYMBOL(closure_sub);
//
// closure_put - decrement a closure's refcount
//
#[no_mangle]
pub unsafe extern "C" fn closure_put(cl: *mut closure) {
    void closure_put(struct closure *cl)
    {
    closure_put_after_sub(cl, atomic_dec_return_release(&cl.remaining));
    }
    EXPORT_SYMBOL(closure_put);
//
// closure_wake_up - wake up all closures on a wait list, without memory barrier
//
#[no_mangle]
pub unsafe extern "C" fn __closure_wake_up(wait_list: *mut closure_waitlist) {
    void __closure_wake_up(struct closure_waitlist *wait_list)
    {
    struct llist_node *list;
    struct closure *cl, *t;
    struct llist_node *reverse = core::ptr::null_mut();
    list = llist_del_all(&wait_list.list);
// We first reverse the list to preserve FIFO ordering and fairness
    reverse = llist_reverse_order(list);
// Then do the wakeups
    llist_for_each_entry_safe(cl, t, reverse, list) {
    closure_set_waiting(cl, 0);
    closure_sub(cl, CLOSURE_WAITING + 1);
    }
    }
    EXPORT_SYMBOL(__closure_wake_up);
//
// closure_wait - add a closure to a waitlist
// @waitlist: will own a ref on @cl, which will be released when
// closure_wake_up() is called on @waitlist.
// @cl: closure pointer.
//
#[no_mangle]
pub unsafe extern "C" fn closure_wait(waitlist: *mut closure_waitlist, cl: *mut closure) -> bool {
    bool closure_wait(struct closure_waitlist *waitlist, struct closure *cl)
    {
    if (atomic_read(&cl.remaining) & CLOSURE_WAITING)
    return false;
    cl.closure_get_happened = true;
    closure_set_waiting(cl, _RET_IP_);
    atomic_add(CLOSURE_WAITING + 1, &cl.remaining);
    llist_add(&cl.list, &waitlist.list);
    return true;
    }
    EXPORT_SYMBOL(closure_wait);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct closure_syncer {
    pub task: *mut task_struct,
    pub done: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn CLOSURE_CALLBACK(_arg: closure_sync_fn) -> static {
    static CLOSURE_CALLBACK(closure_sync_fn)
    {
    struct closure *cl = container_of(ws, struct closure, work);
    struct closure_syncer *s = cl.s;
    struct task_struct *p;
    rcu_read_lock();
    p = READ_ONCE(s.task);
    s.done = 1;
    wake_up_process(p);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __closure_sync(cl: *mut closure) -> void __sched {
    void __sched __closure_sync(struct closure *cl)
    {
    let mut s: closure_syncer = { .task = current };
    cl.s = &s;
    continue_at(cl, closure_sync_fn, core::ptr::null_mut());
    while (1) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    if (s.done)
    break;
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    }
    EXPORT_SYMBOL(__closure_sync);
//
// closure_return_sync - finish running a closure, synchronously (i.e. waiting
// for outstanding get()s to finish) and returning once closure refcount is 0.
//
// Unlike closure_sync() this doesn't reinit the ref to 1; subsequent
// closure_get_not_zero() calls waill fail.
//
#[no_mangle]
pub unsafe extern "C" fn closure_return_sync(cl: *mut closure) -> void __sched {
    void __sched closure_return_sync(struct closure *cl)
    {
    let mut s: closure_syncer = { .task = current };
    cl.s = &s;
    set_closure_fn(cl, closure_sync_fn, core::ptr::null_mut());
    unsigned flags = atomic_sub_return_release(1 + CLOSURE_RUNNING - CLOSURE_DESTRUCTOR,
    &cl.remaining);
    closure_put_after_sub_checks(flags);
    if (unlikely(flags & CLOSURE_REMAINING_MASK)) {
    while (1) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    if (s.done)
    break;
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    }
    if (cl.parent)
    closure_put(cl.parent);
    }
    EXPORT_SYMBOL(closure_return_sync);
#[no_mangle]
pub unsafe extern "C" fn __closure_sync_timeout(cl: *mut closure, timeout: c_ulong) -> int __sched {
    int __sched __closure_sync_timeout(struct closure *cl, unsigned long timeout)
    {
    let mut s: closure_syncer = { .task = current };
    let mut ret: c_int = 0;
    cl.s = &s;
    continue_at(cl, closure_sync_fn, core::ptr::null_mut());
    while (1) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    if (s.done)
    break;
    if (!timeout) {
//
// Carefully undo the continue_at() - but only if it
// hasn't completed, i.e. the final closure_put() hasn't
// happened yet:
//
    unsigned old, new, v = atomic_read(&cl.remaining);
    do {
    old = v;
    if (!old || (old & CLOSURE_RUNNING))
    goto success;
    new = old + CLOSURE_REMAINING_INITIALIZER;
    } while ((v = atomic_cmpxchg(&cl.remaining, old, new)) != old);
    ret = -ETIME;
    }
    timeout = schedule_timeout(timeout);
    }
    success:
    __set_current_state(TASK_RUNNING);
    return ret;
    }
    EXPORT_SYMBOL(__closure_sync_timeout);

    static LIST_HEAD(closure_list);
    static DEFINE_SPINLOCK(closure_list_lock);
#[no_mangle]
pub unsafe extern "C" fn closure_debug_create(cl: *mut closure) {
    void closure_debug_create(struct closure *cl)
    {
    unsigned long flags;
    BUG_ON(cl.magic == CLOSURE_MAGIC_ALIVE);
    cl.magic = CLOSURE_MAGIC_ALIVE;
    spin_lock_irqsave(&closure_list_lock, flags);
    list_add(&cl.all, &closure_list);
    spin_unlock_irqrestore(&closure_list_lock, flags);
    }
    EXPORT_SYMBOL(closure_debug_create);
#[no_mangle]
pub unsafe extern "C" fn closure_debug_destroy(cl: *mut closure) {
    void closure_debug_destroy(struct closure *cl)
    {
    unsigned long flags;
    if (cl.magic == CLOSURE_MAGIC_STACK)
    return;
    BUG_ON(cl.magic != CLOSURE_MAGIC_ALIVE);
    cl.magic = CLOSURE_MAGIC_DEAD;
    spin_lock_irqsave(&closure_list_lock, flags);
    list_del(&cl.all);
    spin_unlock_irqrestore(&closure_list_lock, flags);
    }
    EXPORT_SYMBOL(closure_debug_destroy);
#[no_mangle]
unsafe extern "C" fn debug_show(f: *mut seq_file, data: *mut c_void) -> c_int {
    static int debug_show(struct seq_file *f, void *data)
    {
    struct closure *cl;
    spin_lock_irq(&closure_list_lock);
    list_for_each_entry(cl, &closure_list, all) {
    let mut r: c_int = atomic_read(&cl.remaining);
    seq_printf(f, "%p: %pS . %pS p %p r %i ",
    cl, (void *) cl.ip, cl.fn, cl.parent,
    r & CLOSURE_REMAINING_MASK);
    seq_printf(f, "%s%s\n",
    test_bit(WORK_STRUCT_PENDING_BIT,
    work_data_bits(&cl.work)) ? "Q" : "",
    r & CLOSURE_RUNNING	? "R" : "");
    if (r & CLOSURE_WAITING)
    seq_printf(f, " W %pS\n",
    (void *) cl.waiting_on);
    seq_putc(f, '\n');
    }
    spin_unlock_irq(&closure_list_lock);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(debug);
#[no_mangle]
unsafe extern "C" fn closure_debug_init() -> int __init {
    static int __init closure_debug_init(void)
    {
    debugfs_create_file("closures", 0400, core::ptr::null_mut(), core::ptr::null_mut(), &debug_fops);
    return 0;
    }
    late_initcall(closure_debug_init)
