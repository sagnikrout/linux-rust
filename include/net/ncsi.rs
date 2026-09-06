//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ncsi.h
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
// The NCSI device states seen from external. More NCSI device states are
// only visible internally (in net/ncsi/internal.h). When the NCSI device
// is registered, it's in ncsi_dev_state_registered state. The state
// ncsi_dev_state_start is used to drive to choose active package and
// channel. After that, its state is changed to ncsi_dev_state_functional.
//
// The state ncsi_dev_state_stop helps to shut down the currently active
// package and channel while ncsi_dev_state_config helps to reconfigure
// them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_dev {
    pub state: c_int,
    pub link_up: c_int,
    pub dev: *mut net_device,
    pub ndev): *mut *mut void (handler)(struct ncsi_dev,
}

extern "C" {
    pub fn ncsi_vlan_rx_add_vid(dev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn ncsi_vlan_rx_kill_vid(dev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn ncsi_start_dev(nd: *mut ncsi_dev) -> c_int;
}
extern "C" {
    pub fn ncsi_stop_dev(nd: *mut ncsi_dev);
}
extern "C" {
    pub fn ncsi_unregister_dev(nd: *mut ncsi_dev);
}

