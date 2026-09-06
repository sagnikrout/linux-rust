//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_common.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// PCI configuration read
extern "C" {
    pub fn fm10k_read_pci_cfg_word(hw: *mut fm10k_hw, reg: u32) -> u16;
}
// read operations, indexed using DWORDS
extern "C" {
    pub fn fm10k_read_reg(hw: *mut fm10k_hw, reg: c_int) -> u32;
}
// write operations, indexed using DWORDS

// Switch register write operations, index using DWORDS

// read ctrl register which has no clear on read fields as PCIe flush

extern "C" {
    pub fn fm10k_get_bus_info_generic(hw: *mut fm10k_hw) -> i32;
}
extern "C" {
    pub fn fm10k_get_invariants_generic(hw: *mut fm10k_hw) -> i32;
}
extern "C" {
    pub fn fm10k_disable_queues_generic(hw: *mut fm10k_hw, q_cnt: u16) -> i32;
}
extern "C" {
    pub fn fm10k_start_hw_generic(hw: *mut fm10k_hw) -> i32;
}
extern "C" {
    pub fn fm10k_stop_hw_generic(hw: *mut fm10k_hw) -> i32;
}

extern "C" {
    pub fn fm10k_unbind_hw_stats_q(q: *mut fm10k_hw_stats_q, count: u32);
}
extern "C" {
    pub fn fm10k_get_host_state_generic(hw: *mut fm10k_hw, host_ready: *mut bool) -> i32;
}
