//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_ism.h
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
// Shared Memory Communications Direct over ISM devices (SMC-D)
//
// SMC-D ISM device structure definitions.
//
// Copyright IBM Corp. 2018
//

pub const SMC_EMULATED_ISM_CHID_MASK: c_uint = 0xFF00;
pub const SMC_ISM_IDENT_MASK: c_uint = 0x00FFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_dev_list {
    pub list: list_head,
    pub /: *mut *mut mutex mutex; / Protects list of devices,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_ism_vlanid {
    pub list: list_head,
    pub /: *mut *mut unsigned short vlanid; / Vlan id,
    pub /: *mut *mut refcount_t refcnt; / Reference count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_ism_seid {
    pub seid_string: [u8; 24],
    pub serial_number: [u8; 4],
    pub type: [u8; 4],
}

extern "C" {
    pub fn smc_ism_set_conn(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_ism_unset_conn(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_ism_get_vlan(dev: *mut smcd_dev, vlan_id: c_ushort) -> c_int;
}
extern "C" {
    pub fn smc_ism_put_vlan(dev: *mut smcd_dev, vlan_id: c_ushort) -> c_int;
}
extern "C" {
    pub fn smc_ism_support_dmb_nocopy(smcd: *mut smcd_dev) -> bool;
}
extern "C" {
    pub fn smc_ism_detach_dmb(dev: *mut smcd_dev, token: u64) -> c_int;
}
extern "C" {
    pub fn smc_ism_signal_shutdown(lgr: *mut smc_link_group) -> c_int;
}
extern "C" {
    pub fn smc_ism_get_system_eid(eid: *mut u8);
}
extern "C" {
    pub fn smc_ism_get_chid(dev: *mut smcd_dev) -> u16;
}
extern "C" {
    pub fn smc_ism_is_v2_capable() -> bool;
}
extern "C" {
    pub fn smc_ism_set_v2_capable();
}
extern "C" {
    pub fn smc_ism_init() -> c_int;
}
extern "C" {
    pub fn smc_ism_exit();
}
extern "C" {
    pub fn smcd_nl_get_device(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
// CHIDs in range of 0xFF00 to 0xFFFF are reserved
// for Emulated-ISM device.
//
// loopback-ism:	0xFFFF
// virtio-ism:		0xFF00 ~ 0xFFFE
//
extern "C" {
    pub fn __smc_ism_is_emulated(_arg: chid) -> return;
}
