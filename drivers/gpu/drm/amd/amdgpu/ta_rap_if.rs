//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/ta_rap_if.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
// Responses have bit 31 set

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_rap_status {
    TA_RAP_STATUS__SUCCESS                              = 1,
    TA_RAP_STATUS__ERROR_GENERIC_FAILURE                = 2,
    TA_RAP_STATUS__ERROR_CMD_NOT_SUPPORTED              = 3,
    TA_RAP_STATUS__ERROR_INVALID_VALIDATION_METHOD      = 4,
    TA_RAP_STATUS__ERROR_NULL_POINTER                   = 5,
    TA_RAP_STATUS__ERROR_NOT_INITIALIZED                = 6,
    TA_RAP_STATUS__ERROR_VALIDATION_FAILED              = 7,
    TA_RAP_STATUS__ERROR_ASIC_NOT_SUPPORTED             = 8,
    TA_RAP_STATUS__ERROR_OPERATION_NOT_PERMISSABLE      = 9,
    TA_RAP_STATUS__ERROR_ALREADY_INIT                   = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_rap_cmd {
    TA_CMD_RAP__INITIALIZE              = 1,
    TA_CMD_RAP__VALIDATE_L0             = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_rap_validation_method {
    METHOD_A           = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_rap_cmd_input_data {
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_rap_cmd_output_data {
    pub last_subsection: u32,
    pub num_total_validate: u32,
    pub num_valid: u32,
    pub last_validate_addr: u32,
    pub last_validate_val: u32,
    pub last_validate_val_exptd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_rap_cmd_input {
    pub input: ta_rap_cmd_input_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_rap_cmd_output {
    pub output: ta_rap_cmd_output_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_rap_shared_memory {
    pub cmd_id: u32,
    pub validation_method_id: u32,
    pub resp_id: u32,
    pub rap_status: ta_rap_status,
    pub rap_in_message: ta_rap_cmd_input,
    pub rap_out_message: ta_rap_cmd_output,
    pub reserved: [u8; 64],
}
