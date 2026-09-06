//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/set_mode_types.h
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

// Info frame packet status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum info_frame_flag {
    INFO_PACKET_PACKET_INVALID = 0,
    INFO_PACKET_PACKET_VALID = 1,
    INFO_PACKET_PACKET_RESET = 2,
    INFO_PACKET_PACKET_UPDATE_SCAN_TYPE = 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_info_frame_header {
    pub info_frame_type: u8,
    pub version: u8,
    pub length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct info_packet_raw_data {
    pub hb0: u8,
    pub hb1: u8,
    pub hb2: u8,
    pub /: *mut *mut uint8_t sb[28]; / sb0~sb27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_info_packet {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avi_info_frame {
    pub header: hdmi_info_frame_header,
    pub CHECK_SUM:8: u8,
    pub S0_S1:2: u8,
    pub B0_B1:2: u8,
    pub A0:1: u8,
    pub Y0_Y1_Y2:3: u8,
    pub R0_R3:4: u8,
    pub M0_M1:2: u8,
    pub C0_C1:2: u8,
    pub SC0_SC1:2: u8,
    pub Q0_Q1:2: u8,
    pub EC0_EC2:3: u8,
    pub ITC:1: u8,
    pub VIC0_VIC7:8: u8,
    pub PR0_PR3:4: u8,
    pub CN0_CN1:2: u8,
    pub YQ0_YQ1:2: u8,
    pub bar_top: u16,
    pub bar_bottom: u16,
    pub bar_left: u16,
    pub bar_right: u16,
    pub FR0_FR3:4: u8,
    pub ACE0_ACE3:4: u8,
    pub RID0_RID5:6: u8,
    pub FR4:1: u8,
    pub F157:1: u8,
    pub reserved: [u8; 12],
    pub bits: },
    pub packet_raw_data: info_packet_raw_data,
}

