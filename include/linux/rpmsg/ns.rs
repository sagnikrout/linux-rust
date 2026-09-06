//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rpmsg/ns.h
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
// struct rpmsg_ns_msg - dynamic name service announcement message
// @name: name of remote service that is published
// @addr: address of remote service that is published
// @flags: indicates whether service is created or destroyed
//
// This message is sent across to publish a new service, or announce
// about its removal. When we receive these messages, an appropriate
// rpmsg channel (i.e device) is created/destroyed. In turn, the ->probe()
// or ->remove() handler of the appropriate rpmsg driver will be invoked
// (if/as-soon-as one is registered).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_ns_msg {
    pub name: [c_char; RPMSG_NAME_SIZE],
    pub addr: __rpmsg32,
    pub flags: __rpmsg32,
    pub __packed: },
//
// enum rpmsg_ns_flags - dynamic name service announcement flags
//
// @RPMSG_NS_CREATE: a new remote service was just created
// @RPMSG_NS_DESTROY: a known remote service was just destroyed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmsg_ns_flags {
    RPMSG_NS_CREATE		= 0,
    RPMSG_NS_DESTROY	= 1,
}

// Address 53 is reserved for advertising remote services

    pub rpdev): *mut int rpmsg_ns_register_device(struct rpmsg_device,
