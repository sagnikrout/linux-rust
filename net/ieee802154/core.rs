//! Automatically rewritten from C Header to Rust Module
//! Source: net/ieee802154/core.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg802154_registered_device {
    pub ops: *const cfg802154_ops,
    pub list: list_head,
// wpan_phy index, internal only
    pub wpan_phy_idx: c_int,
// also protected by devlist_mtx
    pub opencount: c_int,
    pub dev_wait: wait_queue_head_t,
// protected by RTNL only
    pub num_running_ifaces: c_int,
// associated wpan interfaces, protected by rtnl or RCU
    pub wpan_dev_list: list_head,
    pub wpan_dev_id: int devlist_generation,,
// must be last because of the way we do wpan_phy_priv(),
// and it should at least be aligned to NETDEV_ALIGN
//
    pub __aligned(NETDEV_ALIGN): wpan_phy wpan_phy,
}

// free object
extern "C" {
    pub fn cfg802154_dev_free(rdev: *mut cfg802154_registered_device);
}
