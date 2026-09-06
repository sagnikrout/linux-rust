//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/cmp.h
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

// Macro flag: #define SOUND_FIREWIRE_CMP_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmp_direction {
    CMP_INPUT = 0,
    CMP_OUTPUT,
}

//
// struct cmp_connection - manages an isochronous connection to a device
// @speed: the connection's actual speed
//
// This structure manages (using CMP) an isochronous stream between the local
// computer and a device's input plug (iPCR) and output plug (oPCR).
//
// There is no corresponding oPCR created on the local computer, so it is not
// possible to overlay connections on top of this one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmp_connection {
    pub speed: c_int,
// private:
    pub connected: bool,
    pub mutex: mutex,
    pub resources: fw_iso_resources,
    pub last_pcr_value: __be32,
    pub pcr_index: c_uint,
    pub max_speed: c_uint,
    pub direction: cmp_direction,
}

extern "C" {
    pub fn cmp_connection_check_used(connection: *mut cmp_connection, used: *mut bool) -> c_int;
}
extern "C" {
    pub fn cmp_connection_destroy(connection: *mut cmp_connection);
}
extern "C" {
    pub fn cmp_connection_release(connection: *mut cmp_connection);
}
extern "C" {
    pub fn cmp_connection_establish(connection: *mut cmp_connection) -> c_int;
}
extern "C" {
    pub fn cmp_connection_break(connection: *mut cmp_connection);
}
