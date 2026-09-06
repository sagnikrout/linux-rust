//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mempolicy.h
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
// NUMA memory policies for Linux.
// Copyright 2003,2004 Andi Kleen SuSE Labs
//
pub const _LINUX_MEMPOLICY_H: c_int = 1;

//
// Describe a memory policy.
//
// A mempolicy can be either associated with a process or with a VMA.
// For VMA related allocations the VMA policy is preferred, otherwise
// the process policy is used. Interrupts ignore the memory policy
// of the current process.
//
// Locking policy for interleave:
// In process context there is no locking because only the process accesses
// its own state. All vma manipulation is somewhat protected by a down_read on
// mmap_lock.
//
// Freeing policy:
// Mempolicy objects are reference counted.  A mempolicy will be freed when
// mpol_put() decrements the reference count to zero.
//
// Duplicating policy objects:
// mpol_dup() allocates a new mempolicy and copies the specified mempolicy
// to the new storage.  The reference count of the new object is initialized
// to 1, representing the caller of mpol_dup().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mempolicy {
    pub refcnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut unsigned short mode; / See MPOL_ above,
    pub /: *mut *mut *mut unsigned short flags; / See set_mempolicy() MPOL_F_ above,
    pub /: *mut *mut nodemask_t nodes; / interleave/bind/preferred/etc,
    pub /: *mut *mut int home_node; / Home node to use for MPOL_BIND and MPOL_PREFERRED_MANY,
    pub /: *mut *mut nodemask_t cpuset_mems_allowed; / relative to these nodes,
    pub /: *mut *mut nodemask_t user_nodemask; / nodemask passed by user,
    pub w: },
    pub rcu: rcu_head,
}

//
// Support for managing mempolicy data objects (clone, copy, destroy)
// The default fast path of a NULL MPOL_DEFAULT policy is always inlined.
//
extern "C" {
    pub fn __mpol_put(pol: *mut mempolicy);
}
//
// Does mempolicy pol need explicit unref after use?
// Currently only needed for shared policies.
//
extern "C" {
    pub fn __mpol_equal(a: *mut mempolicy, b: *mut mempolicy) -> bool;
}
extern "C" {
    pub fn __mpol_equal(_arg: a, _arg: b) -> return;
}
//
// Tree of shared policies for a shared memory region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_policy {
    pub root: rb_root,
    pub lock: rwlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_node {
    pub nd: rb_node,
    pub end: pgoff_t start,,
    pub policy: *mut mempolicy,
}

extern "C" {
    pub fn vma_dup_policy(src: *mut vm_area_struct, dst: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn mpol_shared_policy_init(sp: *mut shared_policy, mpol: *mut mempolicy);
}
extern "C" {
    pub fn mpol_free_shared_policy(sp: *mut shared_policy);
}
extern "C" {
    pub fn vma_policy_mof(vma: *mut vm_area_struct) -> bool;
}
extern "C" {
    pub fn numa_default_policy();
}
extern "C" {
    pub fn numa_policy_init();
}
extern "C" {
    pub fn mpol_rebind_task(tsk: *mut task_struct, new: *const nodemask_t);
}
extern "C" {
    pub fn mpol_rebind_mm(mm: *mut mm_struct, new: *mut nodemask_t);
}
extern "C" {
    pub fn init_nodemask_of_mempolicy(mask: *mut nodemask_t) -> bool;
}
extern "C" {
    pub fn mempolicy_slab_node() -> c_uint;
}

extern "C" {
    pub fn mpol_parse_str(str: *mut c_char, mpol: *mut mempolicy) -> c_int;
}

extern "C" {
    pub fn mpol_to_str(buffer: *mut c_char, maxlen: c_int, pol: *mut mempolicy);
}
// Check if a vma is migratable
extern "C" {
    pub fn vma_migratable(vma: *mut vm_area_struct) -> bool;
}
extern "C" {
    pub fn mpol_put_task_policy(: *mut task_struct);
}
extern "C" {
    pub fn apply_policy_zone(policy: *mut mempolicy, zone: zone_type) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mempolicy {
    pub NULL: return,
    pub true: return,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_policy {
    pub NULL: return,
// ilx = 0;
    pub NULL: return,
    pub 0: return,
// mpol = NULL;
// nodemask = NULL;
    pub 0: return,
    pub false: return,
    pub 0: return,

    pub /: *mut *mut return 1; / error,

    pub /: *mut *mut return -1; / no node preference,
    pub false: return,

