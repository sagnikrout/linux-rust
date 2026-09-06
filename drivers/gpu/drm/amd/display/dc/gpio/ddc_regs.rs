//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/gpio/ddc_regs.h
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
// Copyright 2012-16 Advanced Micro Devices, Inc.
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

// new register headers
// following in header

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc_registers {
    pub gpio: gpio_registers,
    pub ddc_setup: u32,
    pub phy_aux_cntl: u32,
    pub dc_gpio_aux_ctrl_5: u32,
    pub dc_i3cpad_control0: u32,
    pub dc_i3cpad_control1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc_sh_mask {
// i2c_dd_setup
    pub DC_I2C_DDC1_ENABLE: u32,
    pub DC_I2C_DDC1_EDID_DETECT_ENABLE: u32,
    pub DC_I2C_DDC1_EDID_DETECT_MODE: u32,
// ddc1_mask
    pub DC_GPIO_DDC1DATA_PD_EN: u32,
    pub DC_GPIO_DDC1CLK_PD_EN: u32,
    pub AUX_PAD1_MODE: u32,
// i2cpad_mask
    pub DC_GPIO_SDA_PD_DIS: u32,
    pub DC_GPIO_SCL_PD_DIS: u32,
// phy_aux_cntl
    pub AUX_PAD_RXSEL: u32,
    pub DDC_PAD_I2CMODE: u32,
// dc_i3cpad_control0
    pub DC_I3CPAD_DDCCLK_MASK: u32,
    pub DC_I3CPAD_DDCDATA_MASK: u32,
    pub DC_I3CPAD_PD_EN: u32,
    pub DC_I3CPAD_CLK_A: u32,
    pub DC_I3CPAD_DATA_A: u32,
    pub DC_I3CPAD_CLK_EN: u32,
    pub DC_I3CPAD_DATA_EN: u32,
    pub DC_I3CPAD_CLK_Y: u32,
    pub DC_I3CPAD_DATA_Y: u32,
// dc_i3cpad_control1
    pub DC_I3CPAD_STR: u32,
    pub DC_I3CPAD_RXSEL: u32,
}

// following in dc_resource

