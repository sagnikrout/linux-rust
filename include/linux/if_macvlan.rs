//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_macvlan.h
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

pub const MACVLAN_MC_FILTER_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macvlan_dev {
    pub dev: *mut net_device,
    pub list: list_head,
    pub hlist: hlist_node,
    pub port: *mut macvlan_port,
    pub lowerdev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub accel_priv: *mut c_void,
    pub pcpu_stats: *mut vlan_pcpu_stats __percpu,
    pub MACVLAN_MC_FILTER_SZ): DECLARE_BITMAP(mc_filter,,
    pub set_features: netdev_features_t,
    pub mode: macvlan_mode,
    pub flags: u16,
    pub macaddr_count: c_uint,
    pub bc_queue_len_req: u32,

    pub netpoll: *mut netpoll,

}

extern "C" {
    pub fn macvlan_common_setup(dev: *mut net_device);
}
extern "C" {
    pub fn macvlan_dellink(dev: *mut net_device, head: *mut list_head);
}
extern "C" {
    pub fn macvlan_link_register(ops: *mut rtnl_link_ops) -> c_int;
}

extern "C" {
    pub fn dev_uc_add(_arg: macvlan->lowerdev, _arg: dev->dev_addr) -> return;
}
