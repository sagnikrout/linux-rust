//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dcbnl.h
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
//
// Copyright (c) 2008, Intel Corporation.
//
// Author: Lucy Liu <lucy.liu@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_app_type {
    pub ifindex: c_int,
    pub app: dcb_app,
    pub list: list_head,
    pub dcbx: u8,
}

extern "C" {
    pub fn dcb_getrewr(dev: *mut net_device, app: *mut dcb_app) -> u16;
}
extern "C" {
    pub fn dcb_setrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int;
}
extern "C" {
    pub fn dcb_delrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int;
}
extern "C" {
    pub fn dcb_setapp(: *mut net_device, : *mut dcb_app) -> c_int;
}
extern "C" {
    pub fn dcb_getapp(: *mut net_device, : *mut dcb_app) -> u8;
}
extern "C" {
    pub fn dcb_ieee_setapp(: *mut net_device, : *mut dcb_app) -> c_int;
}
extern "C" {
    pub fn dcb_ieee_delapp(: *mut net_device, : *mut dcb_app) -> c_int;
}
extern "C" {
    pub fn dcb_ieee_getapp_mask(: *mut net_device, : *mut dcb_app) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_rewr_prio_pcp_map {
    pub map: [u16; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_ieee_app_prio_map {
    pub map: [u64; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_ieee_app_dscp_map {
    pub map: [u8; 64],
}

extern "C" {
    pub fn dcb_ieee_getapp_default_prio_mask(dev: *const net_device) -> u8;
}
//
// Ops struct for the netlink callbacks.  Used by DCB-enabled drivers through
// the netdevice struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbnl_rtnl_ops {
// IEEE 802.1Qaz std
    pub ): *mut *mut *mut int (ieee_getets) (struct net_device , struct ieee_ets,
    pub ): *mut *mut *mut int (ieee_setets) (struct net_device , struct ieee_ets,
    pub ): *mut *mut *mut int (ieee_getmaxrate) (struct net_device , struct ieee_maxrate,
    pub ): *mut *mut *mut int (ieee_setmaxrate) (struct net_device , struct ieee_maxrate,
    pub ): *mut *mut *mut int (ieee_getqcn) (struct net_device , struct ieee_qcn,
    pub ): *mut *mut *mut int (ieee_setqcn) (struct net_device , struct ieee_qcn,
    pub ): *mut *mut *mut int (ieee_getqcnstats) (struct net_device , struct ieee_qcn_stats,
    pub ): *mut *mut *mut int (ieee_getpfc) (struct net_device , struct ieee_pfc,
    pub ): *mut *mut *mut int (ieee_setpfc) (struct net_device , struct ieee_pfc,
    pub ): *mut *mut *mut int (ieee_getapp) (struct net_device , struct dcb_app,
    pub ): *mut *mut *mut int (ieee_setapp) (struct net_device , struct dcb_app,
    pub ): *mut *mut *mut int (ieee_delapp) (struct net_device , struct dcb_app,
    pub ): *mut *mut *mut int (ieee_peer_getets) (struct net_device , struct ieee_ets,
    pub ): *mut *mut *mut int (ieee_peer_getpfc) (struct net_device , struct ieee_pfc,
// CEE std
    pub ): *mut *mut u8 (getstate)(struct net_device,
    pub u8): *mut *mut *mut u8 (setstate)(struct net_device ,,
    pub ): *mut *mut *mut void (getpermhwaddr)(struct net_device , u8,
    pub u8): *mut *mut *mut void (setpgtccfgtx)(struct net_device , int, u8, u8, u8,,
    pub u8): *mut *mut *mut void (setpgbwgcfgtx)(struct net_device , int,,
    pub u8): *mut *mut *mut void (setpgtccfgrx)(struct net_device , int, u8, u8, u8,,
    pub u8): *mut *mut *mut void (setpgbwgcfgrx)(struct net_device , int,,
    pub ): *mut *mut *mut *mut *mut *mut void (getpgtccfgtx)(struct net_device , int, u8 , u8 , u8 , u8,
    pub ): *mut *mut *mut void (getpgbwgcfgtx)(struct net_device , int, u8,
    pub ): *mut *mut *mut *mut *mut *mut void (getpgtccfgrx)(struct net_device , int, u8 , u8 , u8 , u8,
    pub ): *mut *mut *mut void (getpgbwgcfgrx)(struct net_device , int, u8,
    pub u8): *mut *mut *mut void (setpfccfg)(struct net_device , int,,
    pub ): *mut *mut *mut void (getpfccfg)(struct net_device , int, u8,
    pub ): *mut *mut u8 (setall)(struct net_device,
    pub ): *mut *mut *mut u8 (getcap)(struct net_device , int, u8,
    pub ): *mut *mut *mut int (getnumtcs)(struct net_device , int, u8,
    pub u8): *mut *mut *mut int (setnumtcs)(struct net_device , int,,
    pub ): *mut *mut u8 (getpfcstate)(struct net_device,
    pub u8): *mut *mut *mut void (setpfcstate)(struct net_device ,,
    pub ): *mut *mut *mut void (getbcncfg)(struct net_device , int, u32,
    pub u32): *mut *mut *mut void (setbcncfg)(struct net_device , int,,
    pub ): *mut *mut *mut void (getbcnrp)(struct net_device , int, u8,
    pub u8): *mut *mut *mut void (setbcnrp)(struct net_device , int,,
    pub u8): *mut *mut *mut int (setapp)(struct net_device , u8, u16,,
    pub u16): *mut *mut *mut int (getapp)(struct net_device , u8,,
    pub ): *mut *mut *mut u8 (getfeatcfg)(struct net_device , int, u8,
    pub u8): *mut *mut *mut u8 (setfeatcfg)(struct net_device , int,,
// DCBX configuration
    pub ): *mut *mut u8 (getdcbx)(struct net_device,
    pub u8): *mut *mut *mut u8 (setdcbx)(struct net_device ,,
// peer apps
    pub ): *mut u16,
    pub ): *mut *mut *mut int (peer_getapptable)(struct net_device , struct dcb_app,
// CEE peer
    pub ): *mut *mut *mut int (cee_peer_getpg) (struct net_device , struct cee_pg,
    pub ): *mut *mut *mut int (cee_peer_getpfc) (struct net_device , struct cee_pfc,
// buffer settings
    pub ): *mut *mut *mut int (dcbnl_getbuffer)(struct net_device , struct dcbnl_buffer,
    pub ): *mut *mut *mut int (dcbnl_setbuffer)(struct net_device , struct dcbnl_buffer,
// apptrust
    pub int): *mut *mut *mut *mut int (dcbnl_setapptrust)(struct net_device , u8 ,,
    pub ): *mut *mut *mut *mut int (dcbnl_getapptrust)(struct net_device , u8 , int,
// rewrite
    pub app): *mut *mut *mut int (dcbnl_setrewr)(struct net_device dev, struct dcb_app,
    pub app): *mut *mut *mut int (dcbnl_delrewr)(struct net_device dev, struct dcb_app,
}
