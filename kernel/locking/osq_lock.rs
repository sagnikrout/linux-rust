//! Automatically rewritten from C to Rust
//! Source: kernel/locking/osq_lock.c
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
// An MCS like lock especially tailored for optimistic spinning for sleeping
// lock implementations (mutex, rwsem, etc).
//
// Using a single mcs node per CPU is safe because sleeping locks should not be
// called from interrupt context and we have preemption disabled while
// spinning.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimistic_spin_node {
    pub prev: *mut *mut optimistic_spin_node next,,
    pub /: *mut *mut int locked; / 1 if lock acquired,
    pub /: *mut *mut int cpu; / encoded CPU # + 1 value,
}

    static DEFINE_PER_CPU_SHARED_ALIGNED(struct optimistic_spin_node, osq_node);
//
// We use the value 0 to represent "no CPU", thus the encoded value
// will be the CPU number incremented by 1.
//
#[no_mangle]
pub unsafe extern "C" fn encode_cpu(cpu_nr: c_int) -> c_int {
    static inline int encode_cpu(int cpu_nr)
    {
    return cpu_nr + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn node_cpu(node: *mut optimistic_spin_node) -> c_int {
    static inline int node_cpu(struct optimistic_spin_node *node)
    {
    return node.cpu - 1;
    }
    static inline struct optimistic_spin_node *decode_cpu(int encoded_cpu_val)
    {
    let mut cpu_nr: c_int = encoded_cpu_val - 1;
    return per_cpu_ptr(&osq_node, cpu_nr);
    }
//
// Get a stable @node->next pointer, either for unlock() or unqueue() purposes.
// Can return NULL in case we were the last queued and we updated @lock instead.
//
// If osq_lock() is being cancelled there must be a previous node
// and 'old_cpu' is its CPU #.
// For osq_unlock() there is never a previous node and old_cpu is
// set to OSQ_UNLOCKED_VAL.
//
    static inline struct optimistic_spin_node *
    osq_wait_next(struct optimistic_spin_queue *lock,
    struct optimistic_spin_node *node,
    int old_cpu)
    {
    let mut curr: c_int = encode_cpu(smp_processor_id());
    for (;;) {
    if (atomic_read(&lock.tail) == curr &&
    atomic_cmpxchg_acquire(&lock.tail, curr, old_cpu) == curr) {
//
// We were the last queued, we moved @lock back. @prev
// will now observe @lock and will complete its
// unlock()/unqueue().
//
    return core::ptr::null_mut();
    }
//
// We must xchg() the @node->next value, because if we were to
// leave it in, a concurrent unlock()/unqueue() from
// @node->next might complete Step-A and think its @prev is
// still valid.
//
// If the concurrent unlock()/unqueue() wins the race, we'll
// wait for either @lock to point to us, through its Step-B, or
// wait for a new @node->next from its Step-C.
//
    if (node.next) {
    struct optimistic_spin_node *next;
    next = xchg(&node.next, core::ptr::null_mut());
    if (next)
    return next;
    }
    cpu_relax();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn osq_lock(lock: *mut optimistic_spin_queue) -> bool {
    bool osq_lock(struct optimistic_spin_queue *lock)
    {
    struct optimistic_spin_node *node = this_cpu_ptr(&osq_node);
    struct optimistic_spin_node *prev, *next;
    let mut curr: c_int = encode_cpu(smp_processor_id());
    int old;
    node.locked = 0;
    node.next = core::ptr::null_mut();
    node.cpu = curr;
//
// We need both ACQUIRE (pairs with corresponding RELEASE in
// unlock() uncontended, or fastpath) and RELEASE (to publish
// the node fields we just initialised) semantics when updating
// the lock tail.
//
    old = atomic_xchg(&lock.tail, curr);
    if (old == OSQ_UNLOCKED_VAL)
    return true;
    prev = decode_cpu(old);
    node.prev = prev;
//
// osq_lock()			unqueue
//
// node->prev = prev		osq_wait_next()
// WMB				MB
// prev->next = node		next->prev = prev // unqueue-C
//
// Here 'node->prev' and 'next->prev' are the same variable and we need
// to ensure these stores happen in-order to avoid corrupting the list.
//
    smp_wmb();
    WRITE_ONCE(prev.next, node);
//
// Normally @prev is untouchable after the above store; because at that
// moment unlock can proceed and wipe the node element from stack.
//
// However, since our nodes are static per-cpu storage, we're
// guaranteed their existence -- this allows us to apply
// cmpxchg in an attempt to undo our queueing.
//
// Wait to acquire the lock or cancellation. Note that need_resched()
// will come with an IPI, which will wake smp_cond_load_relaxed() if it
// is implemented with a monitor-wait. vcpu_is_preempted() relies on
// polling, be careful.
//
    if (smp_cond_load_relaxed(&node.locked, VAL || need_resched() ||
    vcpu_is_preempted(node_cpu(node.prev))))
    return true;
// unqueue
//
// Step - A  -- stabilize @prev
//
// Undo our @prev->next assignment; this will make @prev's
// unlock()/unqueue() wait for a next pointer since @lock points to us
// (or later).
//
    for (;;) {
//
// cpu_relax() below implies a compiler barrier which would
// prevent this comparison being optimized away.
//
    if (data_race(prev.next) == node &&
    cmpxchg(&prev.next, node, core::ptr::null_mut()) == node)
    break;
//
// We can only fail the cmpxchg() racing against an unlock(),
// in which case we should observe @node->locked becoming
// true.
//
    if (smp_load_acquire(&node.locked))
    return true;
    cpu_relax();
//
// Or we race against a concurrent unqueue()'s step-B, in which
// case its step-C will write us a new @node->prev pointer.
//
    prev = READ_ONCE(node.prev);
    }
//
// Step - B -- stabilize @next
//
// Similar to unlock(), wait for @node->next or move @lock from @node
// back to @prev.
//
    next = osq_wait_next(lock, node, prev.cpu);
    if (!next)
    return false;
//
// Step - C -- unlink
//
// @prev is stable because its still waiting for a new @prev->next
// pointer, @next is stable because our @node->next pointer is NULL and
// it will wait in Step-A.
//
    WRITE_ONCE(next.prev, prev);
    WRITE_ONCE(prev.next, next);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn osq_unlock(lock: *mut optimistic_spin_queue) {
    void osq_unlock(struct optimistic_spin_queue *lock)
    {
    struct optimistic_spin_node *node, *next;
    let mut curr: c_int = encode_cpu(smp_processor_id());
//
// Fast path for the uncontended case.
//
    if (atomic_try_cmpxchg_release(&lock.tail, &curr, OSQ_UNLOCKED_VAL))
    return;
//
// Second most likely case.
//
    node = this_cpu_ptr(&osq_node);
    next = xchg(&node.next, core::ptr::null_mut());
    if (next) {
    WRITE_ONCE(next.locked, 1);
    return;
    }
    next = osq_wait_next(lock, node, OSQ_UNLOCKED_VAL);
    if (next)
    WRITE_ONCE(next.locked, 1);
    }
