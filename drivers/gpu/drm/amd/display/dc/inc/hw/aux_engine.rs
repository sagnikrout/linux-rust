//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/aux_engine.h
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
pub enum i2caux_transaction_operation {
    I2CAUX_TRANSACTION_READ,
    I2CAUX_TRANSACTION_WRITE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2caux_transaction_address_space {
    I2CAUX_TRANSACTION_ADDRESS_SPACE_I2C = 1,
    I2CAUX_TRANSACTION_ADDRESS_SPACE_DPCD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2caux_transaction_payload {
    pub address_space: i2caux_transaction_address_space,
    pub address: u32,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2caux_transaction_status {
    I2CAUX_TRANSACTION_STATUS_UNKNOWN = (-1L),
    I2CAUX_TRANSACTION_STATUS_SUCCEEDED,
    I2CAUX_TRANSACTION_STATUS_FAILED_CHANNEL_BUSY,
    I2CAUX_TRANSACTION_STATUS_FAILED_TIMEOUT,
    I2CAUX_TRANSACTION_STATUS_FAILED_PROTOCOL_ERROR,
    I2CAUX_TRANSACTION_STATUS_FAILED_NACK,
    I2CAUX_TRANSACTION_STATUS_FAILED_INCOMPLETE,
    I2CAUX_TRANSACTION_STATUS_FAILED_OPERATION,
    I2CAUX_TRANSACTION_STATUS_FAILED_INVALID_OPERATION,
    I2CAUX_TRANSACTION_STATUS_FAILED_BUFFER_OVERFLOW,
    I2CAUX_TRANSACTION_STATUS_FAILED_HPD_DISCON
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2caux_transaction_request {
    pub operation: i2caux_transaction_operation,
    pub payload: i2caux_transaction_payload,
    pub status: i2caux_transaction_status,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2caux_engine_type {
    I2CAUX_ENGINE_TYPE_UNKNOWN = (-1L),
    I2CAUX_ENGINE_TYPE_AUX,
    I2CAUX_ENGINE_TYPE_I2C_DDC_HW,
    I2CAUX_ENGINE_TYPE_I2C_GENERIC_HW,
    I2CAUX_ENGINE_TYPE_I2C_SW
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_default_speed {
    I2CAUX_DEFAULT_I2C_HW_SPEED = 50,
    I2CAUX_DEFAULT_I2C_SW_SPEED = 50
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aux_config {
    pub ALLOW_AUX_WHEN_HPD_LOW:1: u32,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_engine {
    pub inst: u32,
    pub ddc: *mut ddc,
    pub ctx: *mut dc_context,
    pub funcs: *const aux_engine_funcs,
// following values are expressed in milliseconds
    pub delay: u32,
    pub max_defer_write_retry: u32,
    pub acquire_reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_command_context {
    pub buffer: *mut u8,
    pub current_read_length: u32,
    pub offset: u32,
    pub status: i2caux_transaction_status,
    pub request: aux_request_transaction_data,
    pub reply: aux_reply_transaction_data,
    pub returned_byte: u8,
    pub timed_out_retry_aux: u32,
    pub invalid_reply_retry_aux: u32,
    pub defer_retry_aux: u32,
    pub defer_retry_i2c: u32,
    pub invalid_reply_retry_aux_on_ack: u32,
    pub transaction_complete: bool,
    pub operation_succeeded: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct write_command_context {
    pub mot: bool,
    pub buffer: *mut u8,
    pub current_write_length: u32,
    pub status: i2caux_transaction_status,
    pub request: aux_request_transaction_data,
    pub reply: aux_reply_transaction_data,
    pub returned_byte: u8,
    pub timed_out_retry_aux: u32,
    pub invalid_reply_retry_aux: u32,
    pub defer_retry_aux: u32,
    pub defer_retry_i2c: u32,
    pub max_defer_retry: u32,
    pub ack_m_retry: u32,
    pub reply_data: [u8; DEFAULT_AUX_MAX_DATA_SIZE],
    pub transaction_complete: bool,
    pub operation_succeeded: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_engine_funcs {
    pub timeout): u32,
    pub ptr): *mut aux_engine,
    pub engine): *mut aux_engine,
    pub cfg): aux_config,
    pub request): *mut aux_request_transaction_data,
    pub reply): *mut aux_reply_transaction_data,
    pub sw_status): *mut u32,
    pub returned_bytes): *mut u8,
    pub engine): *mut *mut bool (is_engine_available)(struct aux_engine,
    pub ddc): *mut ddc,
    pub middle_of_transaction): bool,
    pub engine): *mut aux_engine,
    pub engine): *mut aux_engine,
}
