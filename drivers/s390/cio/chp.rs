//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/chp.h
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
// Copyright IBM Corp. 2007, 2010
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

pub const CHP_STATUS_STANDBY: c_int = 0;
pub const CHP_STATUS_CONFIGURED: c_int = 1;
pub const CHP_STATUS_RESERVED: c_int = 2;
pub const CHP_STATUS_NOT_RECOGNIZED: c_int = 3;
pub const CHP_ONLINE: c_int = 0;
pub const CHP_OFFLINE: c_int = 1;
pub const CHP_VARY_ON: c_int = 2;
pub const CHP_VARY_OFF: c_int = 3;
pub const CHP_FCES_EVENT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chp_link {
    pub chpid: chp_id,
    pub fla_mask: u32,
    pub fla: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_path {
    pub dev: device,
    pub chpid: chp_id,
    pub /: *mut *mut mutex lock; / Serialize access to below members.,
    pub state: c_int,
    pub desc: channel_path_desc_fmt0,
    pub desc_fmt1: channel_path_desc_fmt1,
    pub desc_fmt3: channel_path_desc_fmt3,
// Channel-measurement related stuff:
    pub cmg: c_int,
    pub shared: c_int,
    pub extended: c_int,
    pub speed: c_ulong,
    pub cmg_chars: cmg_chars,
    pub cmcb: cmg_cmcb,
}

// Return channel_path struct for given chpid.
extern "C" {
    pub fn chp_get_status(chpid: chp_id) -> c_int;
}
extern "C" {
    pub fn chp_get_sch_opm(sch: *mut subchannel) -> u8;
}
extern "C" {
    pub fn chp_is_registered(chpid: chp_id) -> c_int;
}
extern "C" {
    pub fn chp_remove_cmg_attr(chp: *mut channel_path);
}
extern "C" {
    pub fn chp_add_cmg_attr(chp: *mut channel_path) -> c_int;
}
extern "C" {
    pub fn chp_update_desc(chp: *mut channel_path) -> c_int;
}
extern "C" {
    pub fn chp_new(chpid: chp_id) -> c_int;
}
extern "C" {
    pub fn chp_cfg_schedule(chpid: chp_id, configure: c_int);
}
extern "C" {
    pub fn chp_cfg_cancel_deconfigure(chpid: chp_id);
}
extern "C" {
    pub fn chp_info_get_status(chpid: chp_id) -> c_int;
}
extern "C" {
    pub fn chp_ssd_get_mask(: *mut chsc_ssd_info, : *mut chp_link) -> c_int;
}
