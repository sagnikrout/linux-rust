//! Automatically rewritten from C to Rust
//! Source: lib/timerqueue.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Generic Timer-queue
//
// Manages a simple queue of timers, ordered by expiration time.
// Uses rbtrees for quick list adds and expiration.
//
// NOTE: All of the following functions need to be serialized
// to avoid races. No locking is done by this library code.
//

    rb_entry((_n), struct timerqueue_node, node)
#[no_mangle]
pub unsafe extern "C" fn __timerqueue_less(a: *mut rb_node, b: *const rb_node) -> bool {
    static inline bool __timerqueue_less(struct rb_node *a, const struct rb_node *b)
    {
    return __node_2_tq(a).expires < __node_2_tq(b).expires;
    }
//
// timerqueue_add - Adds timer to timerqueue.
//
// @head: head of timerqueue
// @node: timer node to be added
//
// Adds the timer node to the timerqueue, sorted by the node's expires
// value. Returns true if the newly added timer is the first expiring timer in
// the queue.
//
#[no_mangle]
pub unsafe extern "C" fn timerqueue_add(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    bool timerqueue_add(struct timerqueue_head *head, struct timerqueue_node *node)
    {
// Make sure we don't add nodes that are already added
    WARN_ON_ONCE(!RB_EMPTY_NODE(&node.node));
    return rb_add_cached(&node.node, &head.rb_root, __timerqueue_less);
    }
    EXPORT_SYMBOL_GPL(timerqueue_add);
//
// timerqueue_del - Removes a timer from the timerqueue.
//
// @head: head of timerqueue
// @node: timer node to be removed
//
// Removes the timer node from the timerqueue. Returns true if the queue is
// not empty after the remove.
//
#[no_mangle]
pub unsafe extern "C" fn timerqueue_del(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    bool timerqueue_del(struct timerqueue_head *head, struct timerqueue_node *node)
    {
    WARN_ON_ONCE(RB_EMPTY_NODE(&node.node));
    rb_erase_cached(&node.node, &head.rb_root);
    RB_CLEAR_NODE(&node.node);
    return !RB_EMPTY_ROOT(&head.rb_root.rb_root);
    }
    EXPORT_SYMBOL_GPL(timerqueue_del);
//
// timerqueue_iterate_next - Returns the timer after the provided timer
//
// @node: Pointer to a timer.
//
// Provides the timer that is after the given node. This is used, when
// necessary, to iterate through the list of timers in a timer list
// without modifying the list.
//
    struct timerqueue_node *timerqueue_iterate_next(struct timerqueue_node *node)
    {
    struct rb_node *next;
    if (!node)
    return core::ptr::null_mut();
    next = rb_next(&node.node);
    if (!next)
    return core::ptr::null_mut();
    return container_of(next, struct timerqueue_node, node);
    }
    EXPORT_SYMBOL_GPL(timerqueue_iterate_next);

    container_of(rb_entry((_n), struct rb_node_linked, node), struct timerqueue_linked_node, node)
#[no_mangle]
unsafe extern "C" fn __tq_linked_less(a: *mut rb_node, b: *const rb_node) -> __always_inline bool {
    static __always_inline bool __tq_linked_less(struct rb_node *a, const struct rb_node *b)
    {
    return __node_2_tq_linked(a).expires < __node_2_tq_linked(b).expires;
    }
#[no_mangle]
pub unsafe extern "C" fn timerqueue_linked_add(head: *mut timerqueue_linked_head, node: *mut timerqueue_linked_node) -> bool {
    bool timerqueue_linked_add(struct timerqueue_linked_head *head, struct timerqueue_linked_node *node)
    {
    return rb_add_linked(&node.node, &head.rb_root, __tq_linked_less);
    }
    EXPORT_SYMBOL_GPL(timerqueue_linked_add);
