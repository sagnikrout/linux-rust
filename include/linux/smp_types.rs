//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smp_types.h
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
// struct __call_single_node is the primary type on
// smp.c:call_single_queue.
//
// flush_smp_call_function_queue() only reads the type from
// __call_single_node::u_flags as a regular load, the above
// (anonymous) enum defines all the bits of this word.
//
// Other bits are not modified until the type is known.
//
// CSD_TYPE_SYNC/ASYNC:
// struct {
// struct llist_node node;
// unsigned int flags;
// smp_call_func_t func;
// void *info;
// };
//
// CSD_TYPE_IRQ_WORK:
// struct {
// struct llist_node node;
// atomic_t flags;
// void (*func)(struct irq_work *);
// };
//
// CSD_TYPE_TTWU:
// struct {
// struct llist_node node;
// unsigned int flags;
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __call_single_node {
    pub llist: llist_node,
    pub u_flags: c_uint,
    pub a_flags: core::sync::atomic::AtomicI32,
}

