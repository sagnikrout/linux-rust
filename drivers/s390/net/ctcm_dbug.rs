//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/ctcm_dbug.h
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
// Copyright IBM Corp. 2001, 2007
// Authors:	Peter Tiedemann (ptiedem@de.ibm.com)
//
// Debug Facility stuff
//

pub const do_debug: c_int = 1;

pub const do_debug: c_int = 0;

pub const do_debug_ccw: c_int = 1;
pub const DEBUGDATA: c_int = 1;

pub const do_debug_ccw: c_int = 0;

pub const do_debug_data: c_int = 1;

pub const do_debug_data: c_int = 0;

// define dbf debug levels similar to kernel msg levels

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctcm_dbf_names {
    CTCM_DBF_SETUP,
    CTCM_DBF_ERROR,
    CTCM_DBF_TRACE,
    CTCM_DBF_MPC_SETUP,
    CTCM_DBF_MPC_ERROR,
    CTCM_DBF_MPC_TRACE,
    CTCM_DBF_INFOS	/* must be last element */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcm_dbf_info {
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub pages: c_int,
    pub areas: c_int,
    pub len: c_int,
    pub level: c_int,
    pub id: *mut debug_info_t,
}

extern "C" {
    pub fn ctcm_register_dbf_views() -> c_int;
}
extern "C" {
    pub fn ctcm_unregister_dbf_views();
}
extern "C" {
    pub fn ctcm_dbf_longtext(dbf_nix: ctcm_dbf_names, level: c_int, text: *mut c_char, ...);
}

//
// cat : one of {setup, mpc_setup, trace, mpc_trace, error, mpc_error}.
// dev : netdevice with valid name field.
// text: any text string.
//

//
// cat : one of {setup, mpc_setup, trace, mpc_trace, error, mpc_error}.
// dev : netdevice.
// text: any text string.
//

