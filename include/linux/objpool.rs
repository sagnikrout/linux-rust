//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/objpool.h
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
// objpool: ring-array based lockless MPMC queue
//
// Copyright: wuqiang.matt@bytedance.com,mhiramat@kernel.org
//
// objpool is a scalable implementation of high performance queue for
// object allocation and reclamation, such as kretprobe instances.
//
// With leveraging percpu ring-array to mitigate hot spots of memory
// contention, it delivers near-linear scalability for high parallel
// scenarios. The objpool is best suited for the following cases:
// 1) Memory allocation or reclamation are prohibited or too expensive
// 2) Consumers are of different priorities, such as irqs and threads
//
// Limitations:
// 1) Maximum objects (capacity) is fixed after objpool creation
// 2) All pre-allocated objects are managed in percpu ring array,
// which consumes more memory than linked lists
//
// struct objpool_slot - percpu ring array of objpool
// @head: head sequence of the local ring array (to retrieve at)
// @tail: tail sequence of the local ring array (to append at)
// @last: the last sequence number marked as ready for retrieve
// @mask: bits mask for modulo capacity to compute array indexes
// @entries: object entries on this slot
//
// Represents a cpu-local array-based ring buffer, its size is specialized
// during initialization of object pool. The percpu objpool node is to be
// allocated from local memory for NUMA system, and to be kept compact in
// continuous memory: CPU assigned number of objects are stored just after
// the body of objpool_node.
//
// Real size of the ring array is far too smaller than the value range of
// head and tail, typed as uint32_t: [0, 2^32), so only lower bits (mask)
// of head and tail are used as the actual position in the ring array. In
// general the ring array is acting like a small sliding window, which is
// always moving forward in the loop of [0, 2^32).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct objpool_slot {
    pub head: u32,
    pub tail: u32,
    pub last: u32,
    pub mask: u32,
    pub entries: [*mut c_void; ],
    pub __packed: },
    pub objpool_head: struct,
//
// caller-specified callback for object initial setup, it's only called
// once for each object (just after the memory allocation of the object)
//
    pub context): *mut *mut *mut typedef int (objpool_init_obj_cb)(void obj, void,
// caller-specified cleanup callback for objpool destruction
    pub context): *mut *mut *mut typedef int (objpool_fini_cb)(struct objpool_head head, void,
//
// struct objpool_head - object pooling metadata
// @obj_size:   object size, aligned to sizeof(void *)
// @nr_objs:    total objs (to be pre-allocated with objpool)
// @nr_possible_cpus: cached value of num_possible_cpus()
// @capacity:   max objs can be managed by one objpool_slot
// @gfp:        gfp flags for kmalloc & vmalloc
// @ref:        refcount of objpool
// @flags:      flags for objpool management
// @cpu_slots:  pointer to the array of objpool_slot
// @release:    resource cleanup callback
// @context:    caller-provided context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct objpool_head {
    pub obj_size: c_int,
    pub nr_objs: c_int,
    pub nr_possible_cpus: c_int,
    pub capacity: c_int,
    pub gfp: gfp_t,
    pub ref: refcount_t,
    pub flags: c_ulong,
    pub cpu_slots: *mut objpool_slot,
    pub release: objpool_fini_cb,
    pub context: *mut c_void,
}

//
// objpool_init() - initialize objpool and pre-allocated objects
// @pool:    the object pool to be initialized, declared by caller
// @nr_objs: total objects to be pre-allocated by this object pool
// @object_size: size of an object (should be > 0)
// @gfp:     flags for memory allocation (via kmalloc or vmalloc)
// @context: user context for object initialization callback
// @objinit: object initialization callback for extra setup
// @release: cleanup callback for extra cleanup task
//
// return value: 0 for success, otherwise error code
//
// All pre-allocated objects are to be zeroed after memory allocation.
// Caller could do extra initialization in objinit callback. objinit()
// will be called just after slot allocation and called only once for
// each object. After that the objpool won't touch any content of the
// objects. It's caller's duty to perform reinitialization after each
// pop (object allocation) or do clearance before each push (object
// reclamation).
//
// try to retrieve object from slot
// load head snapshot, other cpus may change it
//
// data visibility of 'last' and 'head' could be out of
// order since memory updating of 'last' and 'head' are
// performed in push() and pop() independently
//
// before any retrieving attempts, pop() must guarantee
// 'last' is behind 'head', that is to say, there must
// be available objects in slot, which could be ensured
// by condition 'last != head && last - head <= nr_objs'
// that is equivalent to 'last - head - 1 < nr_objs' as
// 'last' and 'head' are both unsigned int32
//
// obj must be retrieved before moving forward head
// move head forward to mark it's consumption
//
// objpool_pop() - allocate an object from objpool
// @pool: object pool
//
// return value: object ptr or NULL if failed
//
// disable local irq to avoid preemption & interruption
// adding object to slot, abort if the slot was already full
// loading tail and head as a local snapshot, tail first
// fault caught: something must be wrong
// now the tail position is reserved for the given obj
// update sequence to make this obj available for pop()
//
// objpool_push() - reclaim the object and return back to objpool
// @obj:  object ptr to be pushed to objpool
// @pool: object pool
//
// return: 0 or error code (it fails only when user tries to push
// the same object multiple times or wrong "objects" into objpool)
//
// disable local irq to avoid preemption & interruption
//
// objpool_drop() - discard the object and deref objpool
// @obj:  object ptr to be discarded
// @pool: object pool
//
// return: 0 if objpool was released; -EAGAIN if there are still
// outstanding objects
//
// objpool_drop is normally for the release of outstanding objects
// after objpool cleanup (objpool_fini). Thinking of this example:
// kretprobe is unregistered and objpool_fini() is called to release
// all remained objects, but there are still objects being used by
// unfinished kretprobes (like blockable function: sys_accept). So
// only when the last outstanding object is dropped could the whole
// objpool be released along with the call of objpool_drop()
//
extern "C" {
    pub fn objpool_drop(obj: *mut c_void, pool: *mut objpool_head) -> c_int;
}
//
// objpool_free() - release objpool forcely (all objects to be freed)
// @pool: object pool to be released
//
extern "C" {
    pub fn objpool_free(pool: *mut objpool_head);
}
//
// objpool_fini() - deref object pool (also releasing unused objects)
// @pool: object pool to be dereferenced
//
// objpool_fini() will try to release all remained free objects and
// then drop an extra reference of the objpool. If all objects are
// already returned to objpool (so called synchronous use cases),
// the objpool itself will be freed together. But if there are still
// outstanding objects (so called asynchronous use cases, such like
// blockable kretprobe), the objpool won't be released until all
// the outstanding objects are dropped, but the caller must assure
// there are no concurrent objpool_push() on the fly. Normally RCU
// is being required to make sure all ongoing objpool_push() must
// be finished before calling objpool_fini(), so does test_objpool,
// kretprobe or rethook
//
extern "C" {
    pub fn objpool_fini(pool: *mut objpool_head);
}
