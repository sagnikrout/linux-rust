//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mii_timestamper.h
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
// Support for generic time stamping devices on MII buses.
// Copyright (C) 2018 Richard Cochran <richardcochran@gmail.com>
//

//
// struct mii_timestamper - Callback interface to MII time stamping devices.
//
// @rxtstamp:	Requests a Rx timestamp for 'skb'.  If the skb is accepted,
// the MII time stamping device promises to deliver it using
// netif_rx() as soon as a timestamp becomes available. One of
// the PTP_CLASS_ values is passed in 'type'.  The function
// must return true if the skb is accepted for delivery.
//
// @txtstamp:	Requests a Tx timestamp for 'skb'.  The MII time stamping
// device promises to deliver it using skb_complete_tx_timestamp()
// as soon as a timestamp becomes available. One of the PTP_CLASS_
// values is passed in 'type'.
//
// @hwtstamp_set: Handles SIOCSHWTSTAMP ioctl for hardware time stamping.
//
// @hwtstamp_get: Handles SIOCGHWTSTAMP ioctl for hardware time stamping.
//
// @link_state: Allows the device to respond to changes in the link
// state.  The caller invokes this function while holding
// the phy_device mutex.
//
// @ts_info:	Handles ethtool queries for hardware time stamping.
// @device:	Remembers the device to which the instance belongs.
//
// Drivers for PHY time stamping devices should embed their
// mii_timestamper within a private structure, obtaining a reference
// to it using container_of().
//
// Drivers for non-PHY time stamping devices should return a pointer
// to a mii_timestamper from the probe_channel() callback of their
// mii_timestamping_ctrl interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_timestamper {
    pub type): *mut *mut sk_buff skb, int,
    pub type): *mut *mut sk_buff skb, int,
    pub extack): *mut netlink_ext_ack,
    pub kernel_config): *mut kernel_hwtstamp_config,
    pub phydev): *mut phy_device,
    pub ts_info): *mut kernel_ethtool_ts_info,
    pub device: *mut device,
}

//
// struct mii_timestamping_ctrl - MII time stamping controller interface.
//
// @probe_channel:	Callback into the controller driver announcing the
// presence of the 'port' channel.  The 'device' field
// had been passed to register_mii_tstamp_controller().
// The driver must return either a pointer to a valid
// MII timestamper instance or PTR_ERR.
//
// @release_channel:	Releases an instance obtained via .probe_channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_timestamping_ctrl {
    pub port): c_uint,
    pub mii_ts): *mut mii_timestamper,
}

extern "C" {
    pub fn unregister_mii_tstamp_controller(device: *mut device);
}
extern "C" {
    pub fn unregister_mii_timestamper(mii_ts: *mut mii_timestamper);
}

