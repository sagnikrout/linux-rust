//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/context.h
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
// Copyright (C) 2012-2014, 2022, 2024 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_context_h__
//
// enum iwl_ctxt_id_and_color - ID and color fields in context dword
// @FW_CTXT_ID_POS: position of the ID
// @FW_CTXT_ID_MSK: mask of the ID
// @FW_CTXT_COLOR_POS: position of the color
// @FW_CTXT_COLOR_MSK: mask of the color
// @FW_CTXT_INVALID: value used to indicate unused/invalid
// @FW_CTXT_ID_INVALID: value used to indicate unused/invalid. This can be
// used with newer firmware which no longer use the color. Typically,
// firmware versions supported by iwlmld can use this value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ctxt_id_and_color {
    FW_CTXT_ID_POS		= 0,
    FW_CTXT_ID_MSK		= 0xff << FW_CTXT_ID_POS,
    FW_CTXT_COLOR_POS	= 8,
    FW_CTXT_COLOR_MSK	= 0xff << FW_CTXT_COLOR_POS,
    FW_CTXT_INVALID		= 0xffffffff,
    FW_CTXT_ID_INVALID	= 0xff,
}

//
// enum iwl_ctxt_action - Posssible actions on PHYs, MACs, Bindings and other
// @FW_CTXT_ACTION_INVALID: unused, invalid action
// @FW_CTXT_ACTION_ADD: add the context
// @FW_CTXT_ACTION_MODIFY: modify the context
// @FW_CTXT_ACTION_REMOVE: remove the context
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ctxt_action {
    FW_CTXT_ACTION_INVALID = 0,
    FW_CTXT_ACTION_ADD,
    FW_CTXT_ACTION_MODIFY,
    FW_CTXT_ACTION_REMOVE,
}

pub const IWL_LMAC_24G_INDEX: c_int = 0;
pub const IWL_LMAC_5G_INDEX: c_int = 1;
