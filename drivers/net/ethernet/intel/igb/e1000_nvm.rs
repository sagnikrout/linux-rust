//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_nvm.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.
extern "C" {
    pub fn igb_acquire_nvm(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_release_nvm(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_read_mac_addr(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_read_nvm_eerd(hw: *mut e1000_hw, offset: u16, words: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_read_nvm_spi(hw: *mut e1000_hw, offset: u16, words: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_nvm_spi(hw: *mut e1000_hw, offset: u16, words: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_validate_nvm_checksum(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_update_nvm_checksum(hw: *mut e1000_hw) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_fw_version {
    pub etrack_id: u32,
    pub eep_major: u16,
    pub eep_minor: u16,
    pub eep_build: u16,
    pub invm_major: u8,
    pub invm_minor: u8,
    pub invm_img_type: u8,
    pub or_valid: bool,
    pub or_major: u16,
    pub or_build: u16,
    pub or_patch: u16,
}

extern "C" {
    pub fn igb_get_fw_version(hw: *mut e1000_hw, fw_vers: *mut e1000_fw_version);
}
