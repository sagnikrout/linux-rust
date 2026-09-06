//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_ddc_types.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_transaction_type {
    AUX_TRANSACTION_TYPE_DP,
    AUX_TRANSACTION_TYPE_I2C
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2caux_transaction_action {
    I2CAUX_TRANSACTION_ACTION_I2C_WRITE = 0x00,
    I2CAUX_TRANSACTION_ACTION_I2C_READ = 0x10,
    I2CAUX_TRANSACTION_ACTION_I2C_STATUS_REQUEST = 0x20,

    I2CAUX_TRANSACTION_ACTION_I2C_WRITE_MOT = 0x40,
    I2CAUX_TRANSACTION_ACTION_I2C_READ_MOT = 0x50,
    I2CAUX_TRANSACTION_ACTION_I2C_STATUS_REQUEST_MOT = 0x60,

    I2CAUX_TRANSACTION_ACTION_DP_WRITE = 0x80,
    I2CAUX_TRANSACTION_ACTION_DP_READ = 0x90
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_request_transaction_data {
    pub type: aux_transaction_type,
    pub action: i2caux_transaction_action,
// 20-bit AUX channel transaction address
    pub address: u32,
// delay, in 100-microsecond units
    pub delay: u8,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_transaction_reply {
    AUX_TRANSACTION_REPLY_AUX_ACK = 0x00,
    AUX_TRANSACTION_REPLY_AUX_NACK = 0x01,
    AUX_TRANSACTION_REPLY_AUX_DEFER = 0x02,
    AUX_TRANSACTION_REPLY_I2C_OVER_AUX_NACK = 0x04,
    AUX_TRANSACTION_REPLY_I2C_OVER_AUX_DEFER = 0x08,

    AUX_TRANSACTION_REPLY_I2C_ACK = 0x00,
    AUX_TRANSACTION_REPLY_I2C_NACK = 0x10,
    AUX_TRANSACTION_REPLY_I2C_DEFER = 0x20,

    AUX_TRANSACTION_REPLY_HPD_DISCON = 0x40,

    AUX_TRANSACTION_REPLY_INVALID = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_reply_transaction_data {
    pub status: aux_transaction_reply,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_payload {
// set following flag to read/write I2C data,
// reset it to read/write DPCD data
    pub i2c_over_aux: bool,
// set following flag to write data,
// reset it to read data
    pub write: bool,
    pub mot: bool,
    pub write_status_update: bool,
    pub address: u32,
    pub length: u32,
    pub data: *mut u8,
//
// used to return the reply type of the transaction
// ignored if NULL
//
    pub reply: *mut u8,
// expressed in milliseconds
// zero means "use default value"
//
    pub defer_delay: u32,
}

pub const DEFAULT_AUX_MAX_DATA_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_payload {
    pub write: bool,
    pub address: u8,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_command_engine {
    I2C_COMMAND_ENGINE_DEFAULT,
    I2C_COMMAND_ENGINE_SW,
    I2C_COMMAND_ENGINE_HW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_command {
    pub payloads: *mut i2c_payload,
    pub number_of_payloads: u8,
    pub engine: i2c_command_engine,
// expressed in KHz
// zero means "use default value"
    pub speed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_ddc_hw_info {
    pub hw_supported: bool,
    pub ddc_channel: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc {
    pub pin_data: *mut gpio,
    pub pin_clock: *mut gpio,
    pub hw_info: gpio_ddc_hw_info,
    pub ctx: *mut dc_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ddc_wa {
    pub DP_SKIP_POWER_OFF:1: u32,
    pub DP_AUX_POWER_UP_WA_DELAY:1: u32,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc_flags {
    pub EDID_QUERY_DONE_ONCE:1: u8,
    pub IS_INTERNAL_DISPLAY:1: u8,
    pub FORCE_READ_REPEATED_START:1: u8,
    pub EDID_STRESS_READ:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddc_transaction_type {
    DDC_TRANSACTION_TYPE_NONE = 0,
    DDC_TRANSACTION_TYPE_I2C,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX_WITH_DEFER,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX_RETRY_DEFER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_dongle_type {
    DISPLAY_DONGLE_NONE = 0,
// Active converter types
    DISPLAY_DONGLE_DP_VGA_CONVERTER,
    DISPLAY_DONGLE_DP_DVI_CONVERTER,
    DISPLAY_DONGLE_DP_HDMI_CONVERTER,
// DP-HDMI/DVI passive dongles (Type 1 and Type 2)
    DISPLAY_DONGLE_DP_DVI_DONGLE,
    DISPLAY_DONGLE_DP_HDMI_DONGLE,
// Other types of dongle
    DISPLAY_DONGLE_DP_HDMI_MISMATCHED_DONGLE,
}

pub const DC_MAX_EDID_BUFFER_SIZE: c_int = 2048;
pub const DC_EDID_BLOCK_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc_service {
    pub ddc_pin: *mut ddc,
    pub flags: ddc_flags,
    pub wa: ddc_wa,
    pub transaction_type: ddc_transaction_type,
    pub dongle_type: display_dongle_type,
    pub ctx: *mut dc_context,
    pub link: *mut dc_link,
    pub address: u32,
    pub edid_buf_len: u32,
    pub edid_buf: [u8; DC_MAX_EDID_BUFFER_SIZE],
}
