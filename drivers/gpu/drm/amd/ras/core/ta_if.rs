//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ta_if.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const RAS_TA_HOST_IF_VER: c_int = 0;
// Responses have bit 31 set

// invalid node instance value
pub const RAS_TA_INV_NODE: c_uint = 0xffff;
// RAS related enumerations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_cmd_id {
    RAS_TA_CMD_ID__ENABLE_FEATURES = 0,
    RAS_TA_CMD_ID__DISABLE_FEATURES,
    RAS_TA_CMD_ID__TRIGGER_ERROR,
    RAS_TA_CMD_ID__QUERY_BLOCK_INFO,
    RAS_TA_CMD_ID__QUERY_SUB_BLOCK_INFO,
    RAS_TA_CMD_ID__QUERY_ADDRESS,
    MAX_RAS_TA_CMD_ID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_status {
    RAS_TA_STATUS__SUCCESS                          = 0x0000,
    RAS_TA_STATUS__RESET_NEEDED                     = 0xA001,
    RAS_TA_STATUS__ERROR_INVALID_PARAMETER          = 0xA002,
    RAS_TA_STATUS__ERROR_RAS_NOT_AVAILABLE          = 0xA003,
    RAS_TA_STATUS__ERROR_RAS_DUPLICATE_CMD          = 0xA004,
    RAS_TA_STATUS__ERROR_INJECTION_FAILED           = 0xA005,
    RAS_TA_STATUS__ERROR_ASD_READ_WRITE             = 0xA006,
    RAS_TA_STATUS__ERROR_TOGGLE_DF_CSTATE           = 0xA007,
    RAS_TA_STATUS__ERROR_TIMEOUT                    = 0xA008,
    RAS_TA_STATUS__ERROR_BLOCK_DISABLED             = 0XA009,
    RAS_TA_STATUS__ERROR_GENERIC                    = 0xA00A,
    RAS_TA_STATUS__ERROR_RAS_MMHUB_INIT             = 0xA00B,
    RAS_TA_STATUS__ERROR_GET_DEV_INFO               = 0xA00C,
    RAS_TA_STATUS__ERROR_UNSUPPORTED_DEV            = 0xA00D,
    RAS_TA_STATUS__ERROR_NOT_INITIALIZED            = 0xA00E,
    RAS_TA_STATUS__ERROR_TEE_INTERNAL               = 0xA00F,
    RAS_TA_STATUS__ERROR_UNSUPPORTED_FUNCTION       = 0xA010,
    RAS_TA_STATUS__ERROR_SYS_DRV_REG_ACCESS         = 0xA011,
    RAS_TA_STATUS__ERROR_RAS_READ_WRITE             = 0xA012,
    RAS_TA_STATUS__ERROR_NULL_PTR                   = 0xA013,
    RAS_TA_STATUS__ERROR_UNSUPPORTED_IP             = 0xA014,
    RAS_TA_STATUS__ERROR_PCS_STATE_QUIET            = 0xA015,
    RAS_TA_STATUS__ERROR_PCS_STATE_ERROR            = 0xA016,
    RAS_TA_STATUS__ERROR_PCS_STATE_HANG             = 0xA017,
    RAS_TA_STATUS__ERROR_PCS_STATE_UNKNOWN          = 0xA018,
    RAS_TA_STATUS__ERROR_UNSUPPORTED_ERROR_INJ      = 0xA019,
    RAS_TA_STATUS__TEE_ERROR_ACCESS_DENIED          = 0xA01A
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_block {
    RAS_TA_BLOCK__UMC = 0,
    RAS_TA_BLOCK__SDMA,
    RAS_TA_BLOCK__GFX,
    RAS_TA_BLOCK__MMHUB,
    RAS_TA_BLOCK__ATHUB,
    RAS_TA_BLOCK__PCIE_BIF,
    RAS_TA_BLOCK__HDP,
    RAS_TA_BLOCK__XGMI_WAFL,
    RAS_TA_BLOCK__DF,
    RAS_TA_BLOCK__SMN,
    RAS_TA_BLOCK__SEM,
    RAS_TA_BLOCK__MP0,
    RAS_TA_BLOCK__MP1,
    RAS_TA_BLOCK__FUSE,
    RAS_TA_BLOCK__MCA,
    RAS_TA_BLOCK__VCN,
    RAS_TA_BLOCK__JPEG,
    RAS_TA_BLOCK__IH,
    RAS_TA_BLOCK__MPIO,
    RAS_TA_BLOCK__MMSCH,
    RAS_TA_NUM_BLOCK_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_mca_block {
    RAS_TA_MCA_BLOCK__MP0   = 0,
    RAS_TA_MCA_BLOCK__MP1   = 1,
    RAS_TA_MCA_BLOCK__MPIO  = 2,
    RAS_TA_MCA_BLOCK__IOHC  = 3,
    RAS_TA_MCA_NUM_BLOCK_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_error_type {
    RAS_TA_ERROR__NONE			= 0,
    RAS_TA_ERROR__PARITY			= 1,
    RAS_TA_ERROR__SINGLE_CORRECTABLE	= 2,
    RAS_TA_ERROR__MULTI_UNCORRECTABLE	= 4,
    RAS_TA_ERROR__POISON			= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_address_type {
    RAS_TA_MCA_TO_PA,
    RAS_TA_PA_TO_MCA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ta_nps_mode {
    RAS_TA_UNKNOWN_MODE = 0,
    RAS_TA_NPS1_MODE = 1,
    RAS_TA_NPS2_MODE = 2,
    RAS_TA_NPS4_MODE = 4,
    RAS_TA_NPS8_MODE = 8,
}

// Input/output structures for RAS commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_enable_features_input {
    pub block_id: ras_ta_block,
    pub error_type: ras_ta_error_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_disable_features_input {
    pub block_id: ras_ta_block,
    pub error_type: ras_ta_error_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_trigger_error_input {
// ras-block. i.e. umc, gfx
    pub block_id: ras_ta_block,
// type of error. i.e. single_correctable
    pub inject_error_type: ras_ta_error_type,
// mem block. i.e. hbm, sram etc.
    pub sub_block_index: u32,
// explicit address of error
    pub address: u64,
// method if error injection. i.e persistent, coherent etc.
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_init_flags {
    pub poison_mode_en: u8,
    pub dgpu_mode: u8,
    pub xcc_mask: u16,
    pub channel_dis_num: u8,
    pub nps_mode: u8,
    pub active_umc_mask: u32,
    pub vram_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_mca_addr {
    pub err_addr: u64,
    pub ch_inst: u32,
    pub umc_inst: u32,
    pub node_inst: u32,
    pub socket_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_phy_addr {
    pub pa: u64,
    pub bank: u32,
    pub channel_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_query_address_input {
    pub addr_type: ras_ta_address_type,
    pub ma: ras_ta_mca_addr,
    pub pa: ras_ta_phy_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_output_flags {
    pub ras_init_success_flag: u8,
    pub err_inject_switch_disable_flag: u8,
    pub reg_access_failure_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_query_address_output {
// don't use the flags here
    pub flags: ras_ta_output_flags,
    pub ma: ras_ta_mca_addr,
    pub pa: ras_ta_phy_addr,
}

// Common input structure for RAS callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ras_ta_cmd_input {
    pub init_flags: ras_ta_init_flags,
    pub enable_features: ras_ta_enable_features_input,
    pub disable_features: ras_ta_disable_features_input,
    pub trigger_error: ras_ta_trigger_error_input,
    pub address: ras_ta_query_address_input,
    pub reserve_pad: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ras_ta_cmd_output {
    pub flags: ras_ta_output_flags,
    pub address: ras_ta_query_address_output,
    pub reserve_pad: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_cmd {
    pub cmd_id: u32,
    pub resp_id: u32,
    pub ras_status: u32,
    pub if_version: u32,
    pub ras_in_message: ras_ta_cmd_input,
    pub ras_out_message: ras_ta_cmd_output,
}
