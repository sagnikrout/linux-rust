//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-modparams.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2022, 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_modparams_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_power_level {
    IWL_POWER_INDEX_1,
    IWL_POWER_INDEX_2,
    IWL_POWER_INDEX_3,
    IWL_POWER_INDEX_4,
    IWL_POWER_INDEX_5,
    IWL_POWER_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_disable_11n {
    IWL_DISABLE_HT_ALL	 = BIT(0),
    IWL_DISABLE_HT_TXAGG	 = BIT(1),
    IWL_DISABLE_HT_RXAGG	 = BIT(2),
    IWL_ENABLE_HT_TXAGG	 = BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_amsdu_size {
    IWL_AMSDU_DEF = 0,
    IWL_AMSDU_4K = 1,
    IWL_AMSDU_8K = 2,
    IWL_AMSDU_12K = 3,
// Add 2K at the end to avoid breaking current API
    IWL_AMSDU_2K = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_uapsd_disable {
    IWL_DISABLE_UAPSD_BSS		= BIT(0),
    IWL_DISABLE_UAPSD_P2P_CLIENT	= BIT(1),
}

//
// struct iwl_mod_params - module parameters for iwlwifi
//
// Holds the module parameters
//
// @swcrypto: using hardware encryption, default = 0
// @disable_11n: disable 11n capabilities, default = 0,
// use IWL_[DIS,EN]ABLE_HT_* constants
// @amsdu_size: See &enum iwl_amsdu_size.
// @fw_restart: restart firmware, default = 1
// @bt_coex_active: enable bt coex, default = true
// @led_mode: system default, default = 0
// @power_save: enable power save, default = false
// @power_level: power level, default = 1
// @debug_level: levels are IWL_DL_
// @nvm_file: specifies a external NVM file
// @uapsd_disable: disable U-APSD, see &enum iwl_uapsd_disable, default =
// IWL_DISABLE_UAPSD_BSS | IWL_DISABLE_UAPSD_P2P_CLIENT
// @disable_11ac: disable VHT capabilities, default = false.
// @remove_when_gone: remove an inaccessible device from the PCIe bus.
// @enable_ini: enable new FW debug infratructure (INI TLVs)
// @disable_11be: disable EHT capabilities, default = false.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mod_params {
    pub swcrypto: c_int,
    pub disable_11n: c_uint,
    pub amsdu_size: c_int,
    pub fw_restart: bool,
    pub bt_coex_active: bool,
    pub led_mode: c_int,
    pub power_save: bool,
    pub power_level: c_int,

    pub debug_level: u32,

    pub nvm_file: *mut c_char,
    pub uapsd_disable: u32,
    pub disable_11ac: bool,
//
// @disable_11ax: disable HE capabilities, default = false
//
    pub disable_11ax: bool,
    pub remove_when_gone: bool,
    pub enable_ini: u32,
    pub disable_11be: bool,
}

// enabled by default
// Verify amsdu_size module parameter and convert it to a rxb size
