//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/evergreen_smc.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
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

pub const SMC_EVERGREEN_MC_REGISTER_ARRAY_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_Evergreen_MCRegisterAddress {
    pub s0: u16,
    pub s1: u16,
}

pub type SMC_Evergreen_MCRegisterAddress = SMC_Evergreen_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_Evergreen_MCRegisterSet {
    pub value: [u32; SMC_EVERGREEN_MC_REGISTER_ARRAY_SIZE],
}

pub type SMC_Evergreen_MCRegisterSet = SMC_Evergreen_MCRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_Evergreen_MCRegisters {
    pub last: u8,
    pub reserved: [u8; 3],
    pub address: [SMC_Evergreen_MCRegisterAddress; SMC_EVERGREEN_MC_REGISTER_ARRAY_SIZE],
    pub data: [SMC_Evergreen_MCRegisterSet; 5],
}

pub type SMC_Evergreen_MCRegisters = SMC_Evergreen_MCRegisters;
pub const EVERGREEN_SMC_FIRMWARE_HEADER_LOCATION: c_uint = 0x100;
pub const EVERGREEN_SMC_FIRMWARE_HEADER_softRegisters: c_uint = 0x8;
pub const EVERGREEN_SMC_FIRMWARE_HEADER_stateTable: c_uint = 0xC;
pub const EVERGREEN_SMC_FIRMWARE_HEADER_mcRegisterTable: c_uint = 0x20;

