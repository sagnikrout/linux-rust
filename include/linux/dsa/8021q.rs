//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/8021q.h
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
// Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
//

// VBID is limited to three bits only and zero is reserved.
// Only 7 bridges can be enumerated.
//
pub const DSA_TAG_8021Q_MAX_NUM_BRIDGES: c_int = 7;
extern "C" {
    pub fn dsa_tag_8021q_register(ds: *mut dsa_switch, proto: __be16) -> c_int;
}
extern "C" {
    pub fn dsa_tag_8021q_unregister(ds: *mut dsa_switch);
}
extern "C" {
    pub fn dsa_tag_8021q_bridge_vid(bridge_num: c_uint) -> u16;
}
extern "C" {
    pub fn dsa_tag_8021q_standalone_vid(dp: *const dsa_port) -> u16;
}
extern "C" {
    pub fn dsa_8021q_rx_switch_id(vid: u16) -> c_int;
}
extern "C" {
    pub fn dsa_8021q_rx_source_port(vid: u16) -> c_int;
}
extern "C" {
    pub fn vid_is_dsa_8021q(vid: u16) -> bool;
}
