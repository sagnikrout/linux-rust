//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/closure.h
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
// Closure is perhaps the most overused and abused term in computer science, but
// since I've been unable to come up with anything better you're stuck with it
// again.
//
// What are closures?
//
// They embed a refcount. The basic idea is they count "things that are in
// progress" - in flight bios, some other thread that's doing something else -
// anything you might want to wait on.
//
// The refcount may be manipulated with closure_get() and closure_put().
// closure_put() is where many of the interesting things happen, when it causes
// the refcount to go to 0.
//
// Closures can be used to wait on things both synchronously and asynchronously,
// and synchronous and asynchronous use can be mixed without restriction. To
// wait synchronously, use closure_sync() - you will sleep until your closure's
// refcount hits 1.
//
// To wait asynchronously, use
// continue_at(cl, next_function, workqueue);
//
// passing it, as you might expect, the function to run when nothing is pending
// and the workqueue to run that function out of.
//
// continue_at() also, critically, requires a 'return' immediately following the
// location where this macro is referenced, to return to the calling function.
// There's good reason for this.
//
// To use safely closures asynchronously, they must always have a refcount while
// they are running owned by the thread that is running them. Otherwise, suppose
// you submit some bios and wish to have a function run when they all complete:
//
// foo_endio(struct bio *bio)
// {
// closure_put(cl);
// }
//
// closure_init(cl);
//
// do_stuff();
// closure_get(cl);
// bio1->bi_endio = foo_endio;
// bio_submit(bio1);
//
// do_more_stuff();
// closure_get(cl);
// bio2->bi_endio = foo_endio;
// bio_submit(bio2);
//
// continue_at(cl, complete_some_read, system_wq);
//
// If closure's refcount started at 0, complete_some_read() could run before the
// second bio was submitted - which is almost always not what you want! More
// importantly, it wouldn't be possible to say whether the original thread or
// complete_some_read()'s thread owned the closure - and whatever state it was
// associated with!
//
// So, closure_init() initializes a closure's refcount to 1 - and when a
// closure_fn is run, the refcount will be reset to 1 first.
//
// Then, the rule is - if you got the refcount with closure_get(), release it
// with closure_put() (i.e, in a bio->bi_endio function). If you have a refcount
// on a closure because you called closure_init() or you were run out of a
// closure - _always_ use continue_at(). Doing so consistently will help
// eliminate an entire class of particularly pernicious races.
//
// Lastly, you might have a wait list dedicated to a specific event, and have no
// need for specifying the condition - you just want to wait until someone runs
// closure_wake_up() on the appropriate wait list. In that case, just use
// closure_wait(). It will return either true or false, depending on whether the
// closure was already on a wait list or not - a closure can only be on one wait
// list at a time.
//
// Parents:
//
// closure_init() takes two arguments - it takes the closure to initialize, and
// a (possibly null) parent.
//
// If parent is non null, the new closure will have a refcount for its lifetime;
// a closure is considered to be "finished" when its refcount hits 0 and the
// function to run is null. Hence
//
// continue_at(cl, NULL, NULL);
//
// returns up the (spaghetti) stack of closures, precisely like normal return
// returns up the C stack. continue_at() with non null fn is better thought of
// as doing a tail call.
//
// All this implies that a closure should typically be embedded in a particular
// struct (which its refcount will normally control the lifetime of), and that
// struct can very much be thought of as a stack frame.
//
extern "C" {
    pub fn void(: *mut closure_fn) (struct work_struct) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct closure_waitlist {
    pub list: llist_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum closure_state {
//
// CLOSURE_WAITING: Set iff the closure is on a waitlist. Must be set by
// the thread that owns the closure, and cleared by the thread that's
// waking up the closure.
//
// The rest are for debugging and don't affect behaviour:
//
// CLOSURE_RUNNING: Set when a closure is running (i.e. by
// closure_init() and when closure_put() runs then next function), and
// must be cleared before remaining hits 0. Primarily to help guard
// against incorrect usage and accidentally transferring references.
// continue_at() and closure_return() clear it for you, if you're doing
// something unusual you can use closure_set_dead() which also helps
// annotate where references are being transferred.
//

    CLOSURE_BITS_START	= (1U << 26),
    CLOSURE_DESTRUCTOR	= (1U << 26),
    CLOSURE_WAITING		= (1U << 28),
    CLOSURE_RUNNING		= (1U << 30),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct closure {
    pub wq: *mut workqueue_struct,
    pub s: *mut closure_syncer,
    pub list: llist_node,
    pub fn: *mut closure_fn,
}

pub const CLOSURE_MAGIC_DEAD: c_uint = 0xc054dead;
pub const CLOSURE_MAGIC_ALIVE: c_uint = 0xc054a11e;
pub const CLOSURE_MAGIC_STACK: c_uint = 0xc05451cc;

extern "C" {
    pub fn closure_sub(cl: *mut closure, v: c_int);
}
extern "C" {
    pub fn closure_put(cl: *mut closure);
}
extern "C" {
    pub fn __closure_wake_up(list: *mut closure_waitlist);
}
extern "C" {
    pub fn closure_wait(list: *mut closure_waitlist, cl: *mut closure) -> bool;
}
extern "C" {
    pub fn __closure_sync(cl: *mut closure);
}
//
// closure_sync - sleep until a closure a closure has nothing left to wait on
//
// Sleeps until the refcount hits 1 - the thread that's running the closure owns
// the last refcount.
//

extern "C" {
    pub fn __closure_sync_timeout(cl: *mut closure, timeout: c_ulong) -> c_int;
}

extern "C" {
    pub fn closure_debug_create(cl: *mut closure);
}
extern "C" {
    pub fn closure_debug_destroy(cl: *mut closure);
}

//
// Changes made to closure, work_struct, or a couple of other structs
// may cause work.func not pointing to the right location.
//
// closure_get - increment a closure's refcount
//

//
// closure_get_not_zero
//
// closure_init - Initialize a closure, setting the refcount to 1
// @cl:		closure to initialize
// @parent:	parent of the new closure. cl will take a refcount on it for its
// lifetime; may be NULL.
//

//
// closure_wake_up - wake up all closures on a wait list,
// with memory barrier
//
// Memory barrier for the wait list

//
// continue_at - jump to another function with barrier
//
// After @cl is no longer waiting on anything (i.e. all outstanding refs have
// been dropped with closure_put()), it will resume execution at @fn running out
// of @wq (or, if @wq is NULL, @fn will be called by closure_put() directly).
//
// This is because after calling continue_at() you no longer have a ref on @cl,
// and whatever @cl owns may be freed out from under you - a running closure fn
// has a ref on its own closure which continue_at() drops.
//
// Note you are expected to immediately return after using this macro.
//

//
// closure_return - finish execution of a closure
//
// This is used to indicate that @cl is finished: when all outstanding refs on
// @cl have been dropped @cl's ref on its parent closure (as passed to
// closure_init()) will be dropped, if one was specified - thus this can be
// thought of as returning to the parent closure.
//

extern "C" {
    pub fn closure_return_sync(cl: *mut closure);
}
//
// continue_at_nobarrier - jump to another function without barrier
//
// Causes @fn to be executed out of @cl, in @wq context (or called directly if
// @wq is NULL).
//
// The ref the caller of continue_at_nobarrier() had on @cl is now owned by @fn,
// thus it's not safe to touch anything protected by @cl after a
// continue_at_nobarrier().
//

//
// closure_return_with_destructor - finish execution of a closure,
// with destructor
//
// Works like closure_return(), except @destructor will be called when all
// outstanding refs on @cl have been dropped; @destructor may be used to safely
// free the memory occupied by @cl, and it is called with the ref on the parent
// closure still held - so @destructor could safely return an item to a
// freelist protected by @cl's parent.
//

//
// closure_call - execute @fn out of a new, uninitialized closure
//
// Typically used when running out of one closure, and we want to run @fn
// asynchronously out of a new closure - @parent will then wait for @cl to
// finish.
//

//
// Returns 0 if timeout expired, remaining time in jiffies (at least 1) if
// condition became true
//

