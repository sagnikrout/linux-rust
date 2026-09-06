//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_hdmi_types.h
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


//
// Copyright 2022 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

// Address range from 0x00 to 0x1F.
pub const DP_ADAPTOR_TYPE2_SIZE: c_uint = 0x20;
pub const DP_ADAPTOR_TYPE2_REG_ID: c_uint = 0x10;
pub const DP_ADAPTOR_TYPE2_REG_MAX_TMDS_CLK: c_uint = 0x1D;
// Identifies adaptor as Dual-mode adaptor
pub const DP_ADAPTOR_TYPE2_ID: c_uint = 0xA0;
// MHz
pub const DP_ADAPTOR_TYPE2_MAX_TMDS_CLK: c_int = 600;
// MHz
pub const DP_ADAPTOR_TYPE2_MIN_TMDS_CLK: c_int = 25;
// kHZ
pub const DP_ADAPTOR_DVI_MAX_TMDS_CLK: c_int = 165000;
// kHZ
pub const DP_ADAPTOR_HDMI_SAFE_MAX_TMDS_CLK: c_int = 340000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_hdmi_dongle_signature_data {
    pub ADAPTOR"*/: *mut *mut int8_t id[15];/ "DP-HDMI,
    pub /: *mut *mut uint8_t eot;/ end of transmition '\x4',
}

// DP-HDMI dongle slave address for retrieving dongle signature
pub const DP_HDMI_DONGLE_ADDRESS: c_uint = 0x40;
pub const DP_HDMI_DONGLE_SIGNATURE_EOT: c_uint = 0x04;
// SCDC Address defines (HDMI 2.0)
pub const HDMI_SCDC_WRITE_UPDATE_0_ARRAY: c_int = 3;
pub const HDMI_SCDC_ADDRESS: c_uint = 0x54;
pub const HDMI_SCDC_SINK_VERSION: c_uint = 0x01;
pub const HDMI_SCDC_SOURCE_VERSION: c_uint = 0x02;
pub const HDMI_SCDC_UPDATE_0: c_uint = 0x10;
pub const HDMI_SCDC_TMDS_CONFIG: c_uint = 0x20;
pub const HDMI_SCDC_SCRAMBLER_STATUS: c_uint = 0x21;
pub const HDMI_SCDC_CONFIG_0: c_uint = 0x30;
pub const HDMI_SCDC_CONFIG_1: c_uint = 0x31;
pub const HDMI_SCDC_SOURCE_TEST_REQ: c_uint = 0x35;
pub const HDMI_SCDC_STATUS_FLAGS: c_uint = 0x40;
pub const HDMI_SCDC_LTP_REQ: c_uint = 0x41;
pub const HDMI_SCDC_ERR_DETECT: c_uint = 0x50;
pub const HDMI_SCDC_TEST_CONFIG: c_uint = 0xC0;
pub const HDMI_SCDC_MANUFACTURER_OUI: c_uint = 0xD0;
pub const HDMI_SCDC_DEVICE_ID: c_uint = 0xDB;
// IDCC defines (HDMI 2.0)
pub const HDMI_IDCC_ADDRESS: c_uint = 0x50;
pub const HDMI_IDCC_MARKER0: c_uint = 0xAE;
pub const HDMI_IDCC_MARKER1: c_uint = 0x6E;
pub const HDMI_IDCC_MARKER2: c_uint = 0x60;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_idcc_scope {
    HDMI_IDCC_SCOPE_WRITE = 0x00,
    HDMI_IDCC_SCOPE_RW_CA = 0x01,
    HDMI_IDCC_SCOPE_RW_SINK = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_idcc_source_id {
    pub SI_PCA_n:1: u8,
    pub AC_n:1: u8,
    pub RESERVED:6: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_idcc_cable_id {
    pub Cat1_n:1: u8,
    pub Cat2_n:1: u8,
    pub Cat3_n:1: u8,
    pub Cat4_n:1: u8,
    pub RESERVED:4: u8,
    pub HEAC_n:1: u8,
    pub PCA_DEP_n:1: u8,
    pub MonoDir_n:1: u8,
    pub MonoDirErr_n:1: u8,
    pub PCA_ON_n:1: u8,
    pub no_DeEmphasis_n:1: u8,
    pub no_PreShoot_n:1: u8,
    pub RESERVED2:1: u8,
    pub RND_bits_7_0:8: u8,
    pub RND_bits_15_8:8: u8,
    pub bits: },
    pub raw: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_update_read_data {
    pub byte: [u8; 2],
    pub STATUS_UPDATE:1: u8,
    pub CED_UPDATE:1: u8,
    pub RR_TEST:1: u8,
    pub SOURCE_TEST_UPDATE:1: u8,
    pub FRL_START:1: u8,
    pub FLT_UPDATE:1: u8,
    pub RSED_UPDATE:1: u8,
    pub RESERVED:1: u8,
    pub RESERVED2:8: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_status_flags_data {
    pub byte: u8,
    pub CLOCK_DETECTED:1: u8,
    pub CH0_LOCKED:1: u8,
    pub CH1_LOCKED:1: u8,
    pub CH2_LOCKED:1: u8,
    pub LANE3_LOCKED:1: u8,
    pub RESERVED:1: u8,
    pub FLT_READY:1: u8,
    pub DSC_DECODEFAIL:1: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_LTP_req_data {
    pub byte: [u8; 2],
    pub LN0_LTP_REQ:4: u8,
    pub LN1_LTP_REQ:4: u8,
    pub LN2_LTP_REQ:4: u8,
    pub LN3_LTP_REQ:4: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_ced_data {
    pub byte: [u8; 11],
    pub CH0_8LOW:8: u8,
    pub CH0_7HIGH:7: u8,
    pub CH0_VALID:1: u8,
    pub CH1_8LOW:8: u8,
    pub CH1_7HIGH:7: u8,
    pub CH1_VALID:1: u8,
    pub CH2_8LOW:8: u8,
    pub CH2_7HIGH:7: u8,
    pub CH2_VALID:1: u8,
    pub CHECKSUM:8: u8,
    pub LN3_8LOW:8: u8,
    pub LN3_7HIGH:7: u8,
    pub LN3_VALID:1: u8,
    pub RSC_8LOW:8: u8,
    pub RSC_7HIGH:7: u8,
    pub RSC_VALID:1: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_manufacturer_OUI_data {
    pub byte: [u8; 3],
    pub Manufacturer_OUI_1:8: u8,
    pub Manufacturer_OUI_2:8: u8,
    pub Manufacturer_OUI_3:8: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_device_id_data {
    pub byte: u8,
    pub Hardware_Minor_Rev:4: u8,
    pub Hardware_Major_Rev:4: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_configuration {
    pub byte: [u8; 2],
    pub RR_ENABLE:1: u8,
    pub FLT_NO_RETRAIN:1: u8,
    pub RESERVED:6: u8,
    pub FRL_RATE:4: u8,
    pub FFE_LEVELS:4: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_source_test_req {
    pub byte: u8,
    pub RESERVED:1: u8,
    pub TXFFE_PRESHOOT:1: u8,
    pub TXFFE_DEEMPHASIS:1: u8,
    pub TXFFE_NOFFE:1: u8,
    pub RESERVED2:1: u8,
    pub FLT_NO_TIMEOUT:1: u8,
    pub DSC_FRL_MAX:1: u8,
    pub FRL_MAX:1: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_scdc_test_config_Data {
    pub byte: u8,
    pub TEST_READ_REQUEST_DELAY:7: u8,
    pub 1: uint8_t TEST_READ_REQUEST:,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_frl_borrow_mode {
    HDMI_FRL_BORROW_MODE_NONE,
    HDMI_FRL_BORROW_MODE_FROM_ACTIVE,
    HDMI_FRL_BORROW_MODE_FROM_BLANK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_result {
    LINK_RESULT_UNKNOWN = 0,
    LINK_RESULT_SUCCESS,
    LINK_RESULT_LOWER_LINKRATE,
    LINK_RESULT_TIMEOUT,
    LINK_RESULT_FALLBACK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_frl_link_rate {
    HDMI_FRL_LINK_RATE_DISABLE = 0,
    HDMI_FRL_LINK_RATE_3GBPS,
    HDMI_FRL_LINK_RATE_6GBPS,
    HDMI_FRL_LINK_RATE_6GBPS_4LANE,
    HDMI_FRL_LINK_RATE_8GBPS,
    HDMI_FRL_LINK_RATE_10GBPS,
    HDMI_FRL_LINK_RATE_12GBPS,
    HDMI_FRL_LINK_RATE_16GBPS,
    HDMI_FRL_LINK_RATE_20GBPS,
    HDMI_FRL_LINK_RATE_24GBPS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_borrow_params {
    pub audio_packets_line: c_int,
    pub hc_active_target: c_int,
    pub hc_blank_target: c_int,
    pub borrow_mode: hdmi_frl_borrow_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_hdmi_frl_link_settings {
    pub frl_link_rate: hdmi_frl_link_rate,
    pub frl_num_lanes: u8,
    pub borrow_params: frl_borrow_params,
    pub average_tribyte_rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_hdmi_frl_flags {
    pub force_frl_rate: c_uint,
    pub ignore_ffe: bool,
    pub select_ffe: c_int,
    pub limit_ffe: c_int,
    pub force_frl_always: bool,
    pub force_frl_dsc: bool,
    pub force_frl_max: bool,
    pub apply_vsdb_rcc_wa: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_hdmi_frl_link_training_overrides {
    pub force_frl_always: bool,
    pub force_frl_max: bool,
    pub max_retries: u8,
    pub valid: bool,
}
