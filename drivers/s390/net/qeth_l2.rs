//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/qeth_l2.h
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
// Copyright IBM Corp. 2013
// Author(s): Eugene Crosser <eugene.crosser@ru.ibm.com>
//

extern "C" {
    pub fn qeth_bridgeport_setrole(card: *mut qeth_card, role: qeth_sbp_roles) -> c_int;
}
extern "C" {
    pub fn qeth_bridgeport_an_set(card: *mut qeth_card, enable: c_int) -> c_int;
}
extern "C" {
    pub fn qeth_l2_vnicc_set_state(card: *mut qeth_card, vnicc: u32, state: bool) -> c_int;
}
extern "C" {
    pub fn qeth_l2_vnicc_get_state(card: *mut qeth_card, vnicc: u32, state: *mut bool) -> c_int;
}
extern "C" {
    pub fn qeth_l2_vnicc_set_timeout(card: *mut qeth_card, timeout: u32) -> c_int;
}
extern "C" {
    pub fn qeth_l2_vnicc_get_timeout(card: *mut qeth_card, timeout: *mut u32) -> c_int;
}
extern "C" {
    pub fn qeth_bridgeport_allowed(card: *mut qeth_card) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_mac {
    pub mac_addr: [u8; ETH_ALEN],
    pub disp_flag:2: u8,
    pub hnode: hlist_node,
}
