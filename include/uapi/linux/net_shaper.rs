//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/net_shaper.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/net_shaper.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const NET_SHAPER_FAMILY_VERSION: c_int = 1;
//
// enum net_shaper_scope - Defines the shaper @id interpretation.
// @NET_SHAPER_SCOPE_UNSPEC: The scope is not specified.
// @NET_SHAPER_SCOPE_NETDEV: The main shaper for the given network device.
// @NET_SHAPER_SCOPE_QUEUE: The shaper is attached to the given device queue,
// the @id represents the queue number.
// @NET_SHAPER_SCOPE_NODE: The shaper allows grouping of queues or other node
// shapers; can be nested in either @netdev shapers or other @node shapers,
// allowing placement in any location of the scheduling tree, except leaves
// and root.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_shaper_scope {
    NET_SHAPER_SCOPE_UNSPEC,
    NET_SHAPER_SCOPE_NETDEV,
    NET_SHAPER_SCOPE_QUEUE,
    NET_SHAPER_SCOPE_NODE,

// private:
    __NET_SHAPER_SCOPE_MAX,
    NET_SHAPER_SCOPE_MAX = (__NET_SHAPER_SCOPE_MAX - 1)
}

//
// enum net_shaper_metric - Different metric supported by the shaper.
// @NET_SHAPER_METRIC_BPS: Shaper operates on a bits per second basis.
// @NET_SHAPER_METRIC_PPS: Shaper operates on a packets per second basis.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_shaper_metric {
    NET_SHAPER_METRIC_BPS,
    NET_SHAPER_METRIC_PPS,
}
