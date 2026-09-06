//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/amt.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (c) 2021 Taehee Yoo <ap420073@gmail.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifla_amt_mode {
// AMT interface works as Gateway mode.
// The Gateway mode encapsulates IGMP/MLD traffic and decapsulates
// multicast traffic.
//
    AMT_MODE_GATEWAY = 0,
// AMT interface works as Relay mode.
// The Relay mode encapsulates multicast traffic and decapsulates
// IGMP/MLD traffic.
//
    AMT_MODE_RELAY,
    __AMT_MODE_MAX,
}

// This attribute specify mode etier Gateway or Relay.
// This attribute specify Relay port.
// AMT interface is created as Gateway mode, this attribute is used
// to specify relay(remote) port.
// AMT interface is created as Relay mode, this attribute is used
// as local port.
//
// This attribute specify Gateway port.
// AMT interface is created as Gateway mode, this attribute is used
// as local port.
// AMT interface is created as Relay mode, this attribute is not used.
//
// This attribute specify physical device
// This attribute specify local ip address
// This attribute specify Relay ip address.
// So, this is not used by Relay.
//
// This attribute specify Discovery ip address.
// When Gateway get started, it send discovery message to find the
// Relay's ip address.
// So, this is not used by Relay.
//
// This attribute specify number of maximum tunnel.

