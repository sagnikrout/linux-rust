//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/vega10_processpptables.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Vega10_I2CLineID {
    Vega10_I2CLineID_DDC1 = 0x90,
    Vega10_I2CLineID_DDC2 = 0x91,
    Vega10_I2CLineID_DDC3 = 0x92,
    Vega10_I2CLineID_DDC4 = 0x93,
    Vega10_I2CLineID_DDC5 = 0x94,
    Vega10_I2CLineID_DDC6 = 0x95,
    Vega10_I2CLineID_SCLSDA = 0x96,
    Vega10_I2CLineID_DDCVGA = 0x97
}

pub const Vega10_I2C_DDC1DATA: c_int = 0;
pub const Vega10_I2C_DDC1CLK: c_int = 1;
pub const Vega10_I2C_DDC2DATA: c_int = 2;
pub const Vega10_I2C_DDC2CLK: c_int = 3;
pub const Vega10_I2C_DDC3DATA: c_int = 4;
pub const Vega10_I2C_DDC3CLK: c_int = 5;
pub const Vega10_I2C_SDA: c_int = 40;
pub const Vega10_I2C_SCL: c_int = 41;
pub const Vega10_I2C_DDC4DATA: c_int = 65;
pub const Vega10_I2C_DDC4CLK: c_int = 66;
pub const Vega10_I2C_DDC5DATA: c_uint = 0x48;
pub const Vega10_I2C_DDC5CLK: c_uint = 0x49;
pub const Vega10_I2C_DDC6DATA: c_uint = 0x4a;
pub const Vega10_I2C_DDC6CLK: c_uint = 0x4b;
pub const Vega10_I2C_DDCVGADATA: c_uint = 0x4c;
pub const Vega10_I2C_DDCVGACLK: c_uint = 0x4d;
extern "C" {
    pub fn vega10_get_number_of_powerplay_table_entries(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn vega10_baco_set_cap(hwmgr: *mut pp_hwmgr) -> c_int;
}
