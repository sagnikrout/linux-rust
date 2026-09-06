//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu8.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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

// Macro flag: #define ENABLE_DEBUG_FEATURES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Firmware_Header {
    pub Version: u32,
    pub ImageSize: u32,
    pub CodeSize: u32,
    pub HeaderSize: u32,
    pub EntryPoint: u32,
    pub Rtos: u32,
    pub UcodeLoadStatus: u32,
    pub DpmTable: u32,
    pub FanTable: u32,
    pub PmFuseTable: u32,
    pub Globals: u32,
    pub Reserved: [u32; 20],
    pub Signature: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_MultimediaPowerLogData {
    pub avgTotalPower: u32,
    pub avgGpuPower: u32,
    pub avgUvdPower: u32,
    pub avgVcePower: u32,
    pub avgSclk: u32,
    pub avgDclk: u32,
    pub avgVclk: u32,
    pub avgEclk: u32,
    pub startTimeHi: u32,
    pub startTimeLo: u32,
    pub endTimeHi: u32,
    pub endTimeLo: u32,
}

pub const SMU8_FIRMWARE_HEADER_LOCATION: c_uint = 0x1FF80;
pub const SMU8_UNBCSR_START_ADDR: c_uint = 0xC0100000;
pub const SMN_MP1_SRAM_START_ADDR: c_uint = 0x10000000;

