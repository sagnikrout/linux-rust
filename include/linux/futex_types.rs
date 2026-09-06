//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/futex_types.h
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
// struct futex_sched_data - Futex related per task data
// @robust_list:	User space registered robust list pointer
// @compat_robust_list:	User space registered robust list pointer for compat tasks
// @pi_state_list:	List head for Priority Inheritance (PI) state management
// @pi_state_cache:	Pointer to cache one PI state object per task
// @exit_mutex:		Mutex for serializing exit
// @state:		Futex handling state to handle exit races correctly
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_sched_data {
    pub robust_list: *mut robust_list_head __user,

    pub compat_robust_list: *mut compat_robust_list_head __user,

    pub pi_state_list: list_head,
    pub pi_state_cache: *mut futex_pi_state,
    pub exit_mutex: mutex,
    pub state: c_uint,
}

//
// struct futex_mm_phash - Futex private hash related per MM data
// @lock:	Mutex to protect the private hash operations
// @hash:	RCU managed pointer to the private hash
// @hash_new:	Pointer to a newly allocated private hash
// @batches:	Batch state for RCU synchronization
// @rcu:	RCU head for call_rcu()
// @atomic:	Aggregate value for @hash_ref
// @ref:	Per CPU reference counter for a private hash
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_mm_phash {
    pub lock: mutex,
    pub hash: *mut futex_private_hash __rcu,
    pub hash_new: *mut futex_private_hash,
    pub batches: c_ulong,
    pub rcu: rcu_head,
    pub atomic: atomic_long_t,
    pub ref: *mut unsigned int __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_mm_phash {

//
// struct futex_unlock_cs_range - Range for the VDSO unlock critical section
// @start_ip:	The start IP of the robust futex unlock critical section (inclusive)
// @len:	The length of the robust futex unlock critical section
// @pop_size32:	Pending OP pointer size indicator. 0 == 64-bit, 1 == 32-bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_unlock_cs_range {
    pub start_ip: c_ulong,
    pub len: c_uint,
    pub pop_size32: c_uint,
}

//
// struct futex_unlock_cs_ranges - Futex unlock VSDO critical sections
// @cs_ranges:	Array of critical section ranges
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_unlock_cs_ranges {
    pub cs_ranges: [futex_unlock_cs_range; FUTEX_ROBUST_MAX_CS_RANGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_unlock_cs_ranges {

//
// struct futex_mm_data - Futex related per MM data
// @phash:	Futex private hash related data
// @unlock:	Futex unlock VDSO critical sections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_mm_data {
    pub phash: futex_mm_phash,
    pub unlock: futex_unlock_cs_ranges,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_sched_data {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_mm_data {

