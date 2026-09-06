//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/rmnet/rmnet_private.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2013-2014, 2016-2018 The Linux Foundation. All rights reserved.
//
pub const RMNET_MAX_PACKET_SIZE: c_int = 16384;
pub const RMNET_DFLT_PACKET_SIZE: c_int = 1500;
pub const RMNET_NEEDED_HEADROOM: c_int = 16;
pub const RMNET_TX_QUEUE_LEN: c_int = 1000;
// Replace skb->dev to a virtual rmnet device and pass up the stack

// Pass the frame directly to another device with dev_queue_xmit()

