//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iocontext.h
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
// An io_cq (icq) is association between an io_context (ioc) and a
// request_queue (q).  This is used by elevators which need to track
// information per ioc - q pair.
//
// Elevator can request use of icq by setting elevator_type->icq_size and
// ->icq_align.  Both size and align must be larger than that of struct
// io_cq and elevator can use the tail area for private information.  The
// recommended way to do this is defining a struct which contains io_cq as
// the first member followed by private members and using its size and
// align.  For example,
//
// struct snail_io_cq {
// struct io_cq	icq;
// int		poke_snail;
// int		feed_snail;
// };
//
// struct elevator_type snail_elv_type {
// .ops =		{ ... },
// .icq_size =	sizeof(struct snail_io_cq),
// .icq_align =	__alignof__(struct snail_io_cq),
// ...
// };
//
// If icq_size is set, block core will manage icq's.  All requests will
// have its ->elv.icq field set before elevator_ops->elevator_set_req_fn()
// is called and be holding a reference to the associated io_context.
//
// Whenever a new icq is created, elevator_ops->elevator_init_icq_fn() is
// called and, on destruction, ->elevator_exit_icq_fn().  Both functions
// are called with both the associated io_context and queue locks held.
//
// Elevator is allowed to lookup icq using ioc_lookup_icq() while holding
// queue lock but the returned icq is valid only until the queue lock is
// released.  Elevators can not and should not try to create or destroy
// icq's.
//
// As icq's are linked from both ioc and q, the locking rules are a bit
// complex.
//
// - ioc lock nests inside q lock.
//
// - ioc->icq_list and icq->ioc_node are protected by ioc lock.
// q->icq_list and icq->q_node by q lock.
//
// - ioc->icq_tree and ioc->icq_hint are protected by ioc lock, while icq
// itself is protected by q lock.  However, both the indexes and icq
// itself are also RCU managed and lookup can be performed holding only
// the q lock.
//
// - icq's are not reference counted.  They are destroyed when either the
// ioc or q goes away.  Each request with icq set holds an extra
// reference to ioc to ensure it stays until the request is completed.
//
// - Linking and unlinking icq's are performed while holding both ioc and q
// locks.  Due to the lock ordering, q exit is simple but ioc exit
// requires reverse-order double lock dance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cq {
    pub q: *mut request_queue,
    pub ioc: *mut io_context,
//
// q_node and ioc_node link io_cq through icq_list of q and ioc
// respectively.  Both fields are unused once ioc_exit_icq() is
// called and shared with __rcu_icq_cache and __rcu_head which are
// used for RCU free of io_cq.
//
    pub q_node: list_head,
    pub __rcu_icq_cache: *mut kmem_cache,
}

//
// I/O subsystem state of the associated processes.  It is refcounted
// and kmalloc'ed. These could be shared between processes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_context {
    pub refcount: atomic_long_t,
    pub active_ref: core::sync::atomic::AtomicI32,
    pub ioprio: c_ushort,

// all the fields below are protected by this lock
    pub lock: spinlock_t,
    pub icq_tree: radix_tree_root,
    pub icq_hint: *mut io_cq __rcu,
    pub icq_list: hlist_head,
    pub release_work: work_struct,

}

extern "C" {
    pub fn put_io_context(ioc: *mut io_context);
}
extern "C" {
    pub fn exit_io_context(task: *mut task_struct);
}
extern "C" {
    pub fn __copy_io(clone_flags: u64, tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn __copy_io(_arg: clone_flags, _arg: tsk) -> return;
}

