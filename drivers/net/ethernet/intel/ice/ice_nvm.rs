//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_nvm.h
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
// Copyright (c) 2019, Intel Corporation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_orom_civd_info {
    pub /: *mut *mut u8 signature[4]; / Must match ASCII '$CIV' characters,
    pub /: *mut *mut u8 checksum; / Simple modulo 256 sum of all structure bytes must equal 0,
    pub /: *mut *mut __le32 combo_ver; / Combo Image Version number,
    pub /: *mut *mut u8 combo_name_len; / Length of the unicode combo image version string, max of 32,
    pub /: *mut *mut __le16 combo_name[32]; / Unicode string representing the Combo Image version,
    pub __packed: },
    pub access): *mut *mut int ice_acquire_nvm(struct ice_hw hw, enum ice_aq_res_access_type,
    pub hw): *mut void ice_release_nvm(struct ice_hw,
    pub cd): *mut bool read_shadow_ram, struct ice_sq_cd,
    pub read_aq_err): *mut bool read_shadow_ram, enum libie_aq_err,
    pub module_type): u16,
    pub orom): *mut *mut int ice_get_inactive_orom_ver(struct ice_hw hw, struct ice_orom_info,
    pub nvm): *mut *mut int ice_get_inactive_nvm_ver(struct ice_hw hw, struct ice_nvm_info,
    pub netlist): *mut *mut ice_get_inactive_netlist_ver(struct ice_hw hw, struct ice_netlist_info,
    pub pba_num_size): *mut *mut *mut int ice_read_pba_string(struct ice_hw hw, u8 pba_num, u32,
    pub hw): *mut int ice_init_nvm(struct ice_hw,
    pub data): *mut *mut int ice_read_sr_word(struct ice_hw hw, u16 offset, u16,
    pub cd): *mut ice_sq_cd,
    pub cd): *mut *mut ice_aq_erase_nvm(struct ice_hw hw, u16 module_typeid, struct ice_sq_cd,
    pub hw): *mut int ice_nvm_validate_checksum(struct ice_hw,
    pub response_flags): *mut *mut int ice_nvm_write_activate(struct ice_hw hw, u16 cmd_flags, u8,
    pub hw): *mut int ice_aq_nvm_update_empr(struct ice_hw,
    pub cd): *mut u16 length, struct ice_sq_cd,
    pub cd): *mut *mut u8 comp_response_code, struct ice_sq_cd,
