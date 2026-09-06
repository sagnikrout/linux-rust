//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ioam6_genl.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// IPv6 IOAM Generic Netlink API
//
// Author:
// Justin Iurman <justin.iurman@uliege.be>
//

pub const IOAM6_GENL_VERSION: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ioam6_event_type {
    IOAM6_EVENT_UNSPEC,
    IOAM6_EVENT_TRACE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ioam6_event_attr {
    IOAM6_EVENT_ATTR_UNSPEC,

    IOAM6_EVENT_ATTR_TRACE_NAMESPACE,	/* u16 */
    IOAM6_EVENT_ATTR_TRACE_NODELEN,		/* u8 */
    IOAM6_EVENT_ATTR_TRACE_TYPE,		/* u32 */
    IOAM6_EVENT_ATTR_TRACE_DATA,		/* Binary */

    __IOAM6_EVENT_ATTR_MAX
}

