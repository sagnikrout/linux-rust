//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/inc/mod_hdcp.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

// Forward Declarations
pub const MAX_NUM_OF_DISPLAYS: c_int = 6;
pub const MAX_NUM_OF_ATTEMPTS: c_int = 4;
pub const MAX_NUM_OF_ERROR_TRACE: c_int = 10;

// detailed return status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_status {
    MOD_HDCP_STATUS_LIST(ENUM_FORMAT)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_displayport {
    pub rev: u8,
    pub assr_enabled: u8,
    pub mst_enabled: u8,
    pub dp2_enabled: u8,
    pub usb4_enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_hdmi {
    pub frl_enabled: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_operation_mode {
    MOD_HDCP_MODE_OFF,
    MOD_HDCP_MODE_DEFAULT,
    MOD_HDCP_MODE_DP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_display_state {
    MOD_HDCP_DISPLAY_INACTIVE = 0,
    MOD_HDCP_DISPLAY_ACTIVE,
    MOD_HDCP_DISPLAY_ENCRYPTION_ENABLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_psp_caps {
    pub dtm_v3_supported: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_display_disable_option {
    MOD_HDCP_DISPLAY_NOT_DISABLE = 0,
    MOD_HDCP_DISPLAY_DISABLE_AUTHENTICATION,
    MOD_HDCP_DISPLAY_DISABLE_ENCRYPTION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_atomic_op_i2c {
    pub address: u8,
    pub offset: u8,
    pub data: *mut u8,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_atomic_op_aux {
    pub address: u32,
    pub data: *mut u8,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_ddc {
    pub handle: *mut c_void,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_ddc_funcs {
    pub size): u32,
    pub size): u32,
    pub size): u32,
    pub size): u32,
    pub funcs: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_psp {
    pub handle: *mut c_void,
    pub funcs: *mut c_void,
    pub caps: mod_hdcp_psp_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_display_adjustment {
    pub 2: uint8_t disable :,
    pub 6: uint8_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_link_adjustment_hdcp1 {
    pub 1: uint8_t disable :,
    pub 1: uint8_t postpone_encryption :,
    pub 1: uint8_t min_auth_retries_wa :,
    pub 5: uint8_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_force_hdcp_type {
    MOD_HDCP_FORCE_TYPE_MAX = 0,
    MOD_HDCP_FORCE_TYPE_0,
    MOD_HDCP_FORCE_TYPE_1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_link_adjustment_hdcp2 {
    pub 1: uint8_t disable :,
    pub 2: uint8_t force_type :,
    pub 1: uint8_t force_no_stored_km :,
    pub 1: uint8_t increase_h_prime_timeout:,
    pub 1: uint8_t use_fw_locality_check :,
    pub 1: uint8_t use_sw_locality_fallback:,
    pub 1: uint8_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_link_adjustment {
    pub auth_delay: u8,
    pub retry_limit: u8,
    pub hdcp1: mod_hdcp_link_adjustment_hdcp1,
    pub hdcp2: mod_hdcp_link_adjustment_hdcp2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_error {
    pub status: mod_hdcp_status,
    pub state_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp1_trace {
    pub attempt_count: u8,
    pub downstream_device_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp2_trace {
    pub attempt_count: u8,
    pub downstream_device_count: u8,
    pub hdcp1_device_downstream: u8,
    pub hdcp2_legacy_device_downstream: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_trace {
    pub errors: [mod_hdcp_error; MAX_NUM_OF_ERROR_TRACE],
    pub error_count: u8,
    pub hdcp1: mod_hdcp1_trace,
    pub hdcp2: mod_hdcp2_trace,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_encryption_status {
    MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF = 0,
    MOD_HDCP_ENCRYPTION_STATUS_HDCP1_ON,
    MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE0_ON,
    MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE1_ON,
    MOD_HDCP_ENCRYPTION_STATUS_HDCP2_ON
}

// per link events dm has to notify to hdcp module
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_event {
    MOD_HDCP_EVENT_CALLBACK = 0,
    MOD_HDCP_EVENT_WATCHDOG_TIMEOUT,
    MOD_HDCP_EVENT_CPIRQ
}

// output flags from module requesting timer operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_output {
    pub callback_needed: u8,
    pub callback_stop: u8,
    pub watchdog_timer_needed: u8,
    pub watchdog_timer_stop: u8,
    pub callback_delay: u16,
    pub watchdog_timer_delay: u16,
    pub auth_complete: u8,
}

// used to represent per display info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_display {
    pub state: mod_hdcp_display_state,
    pub index: u8,
    pub controller: u8,
    pub dig_fe: u8,
    pub stream_enc_idx: u8,
    pub vc_id: u8,
}

// used to represent per link info
// in case a link has multiple displays, they share the same link info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_link {
    pub mode: mod_hdcp_operation_mode,
    pub dig_be: u8,
    pub ddc_line: u8,
    pub link_enc_idx: u8,
    pub phy_idx: u8,
    pub dio_output_id: u8,
    pub hdcp_supported_informational: u8,
    pub dp: mod_hdcp_displayport,
    pub hdmi: mod_hdcp_hdmi,
}

// a query structure for a display's hdcp information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_display_query {
    pub display: *const mod_hdcp_display,
    pub link: *const mod_hdcp_link,
    pub trace: *const mod_hdcp_trace,
    pub encryption_status: mod_hdcp_encryption_status,
}

// contains values per on external display configuration change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_config {
    pub psp: mod_hdcp_psp,
    pub ddc: mod_hdcp_ddc,
    pub index: u8,
}

// dm allocates memory of mod_hdcp per dc_link on dm init based on memory size
extern "C" {
    pub fn mod_hdcp_get_memory_size() -> usize;
}
// called per link on link creation
// called per link on link destroy
extern "C" {
    pub fn mod_hdcp_teardown(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
// called per display after stream is enabled
// called per display before stream is disabled
// called per display to apply new authentication adjustment
// called to query hdcp information on a specific index
// called per link on connectivity change
// called per link on events (i.e. callback, watchdog, CP_IRQ)
// called to convert enum mod_hdcp_status to c string
// called to convert state id to c string
// called to convert signal type to operation mode
