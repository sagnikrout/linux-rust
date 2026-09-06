//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/skb.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// linux/can/skb.h
//
// Definitions for the CAN network socket buffer
//
// Copyright (C) 2012 Oliver Hartkopp <socketcan@hartkopp.net>
//

extern "C" {
    pub fn can_flush_echo_skb(dev: *mut net_device);
}
extern "C" {
    pub fn can_dropped_invalid_skb(dev: *mut net_device, skb: *mut sk_buff) -> bool;
}
// skb_ext_add() returns uninitialized space
extern "C" {
    pub fn skb_ext_find(_arg: skb, _arg: SKB_EXT_CAN) -> return;
}
// If the socket has already been closed by user space, the
// refcount may already be 0 (and the socket will be freed
// after the last TX skb has been freed). So only increase
// socket refcount if the refcount is > 0.
//
// returns an unshared skb owned by the original sock to be echo'ed back
//
// the CAN specific type of skb is identified by its data length
// this also checks valid CAN XL data length boundaries
// get length element value from can[|fd|xl]_frame structure
// get needed data length inside CAN frame for all frame types (RTR aware)
// RTR frames have an actual length of zero
