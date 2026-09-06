//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net_tstamp.h
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
// struct hwtstamp_provider_desc - hwtstamp provider description
//
// @index: index of the hwtstamp provider.
// @qualifier: hwtstamp provider qualifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwtstamp_provider_desc {
    pub index: c_int,
    pub qualifier: hwtstamp_provider_qualifier,
}

//
// struct hwtstamp_provider - hwtstamp provider object
//
// @rcu_head: RCU callback used to free the struct.
// @source: source of the hwtstamp provider.
// @phydev: pointer of the phydev source in case a PTP coming from phylib
// @desc: hwtstamp provider description.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwtstamp_provider {
    pub rcu_head: rcu_head,
    pub source: hwtstamp_source,
    pub phydev: *mut phy_device,
    pub desc: hwtstamp_provider_desc,
}

//
// struct kernel_hwtstamp_config - Kernel copy of struct hwtstamp_config
//
// @flags: see struct hwtstamp_config
// @tx_type: see struct hwtstamp_config
// @rx_filter: see struct hwtstamp_config
// @ifr: pointer to ifreq structure from the original ioctl request, to pass to
// a legacy implementation of a lower driver
// @copied_to_user: request was passed to a legacy implementation which already
// copied the ioctl request back to user space
// @source: indication whether timestamps should come from the netdev or from
// an attached phylib PHY
// @qualifier: qualifier of the hwtstamp provider
//
// Prefer using this structure for in-kernel processing of hardware
// timestamping configuration, over the inextensible struct hwtstamp_config
// exposed to the %SIOCGHWTSTAMP and %SIOCSHWTSTAMP ioctl UAPI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_hwtstamp_config {
    pub flags: c_int,
    pub tx_type: c_int,
    pub rx_filter: c_int,
    pub ifr: *mut ifreq,
    pub copied_to_user: bool,
    pub source: hwtstamp_source,
    pub qualifier: hwtstamp_provider_qualifier,
}
