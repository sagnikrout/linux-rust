//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rs780_dpm.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rs780_vddc_level {
    RS780_VDDC_LEVEL_UNKNOWN = 0,
    RS780_VDDC_LEVEL_LOW = 1,
    RS780_VDDC_LEVEL_HIGH = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igp_power_info {
// flags
    pub invert_pwm_required: bool,
    pub pwm_voltage_control: bool,
    pub voltage_control: bool,
    pub gfx_clock_gating: bool,
// stored values
    pub system_config: u32,
    pub bootup_uma_clk: u32,
    pub max_voltage: u16,
    pub min_voltage: u16,
    pub boot_voltage: u16,
    pub inter_voltage_low: u16,
    pub inter_voltage_high: u16,
    pub num_of_cycles_in_period: u16,
// variable
    pub crtc_id: c_int,
    pub refresh_rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igp_ps {
    pub min_voltage: rs780_vddc_level,
    pub max_voltage: rs780_vddc_level,
    pub sclk_low: u32,
    pub sclk_high: u32,
    pub flags: u32,
}

pub const RS780_CGFTV_DFLT: c_uint = 0x0303000f;
pub const RS780_FBDIVTIMERVAL_DFLT: c_uint = 0x2710;
pub const RS780_FVTHROTUTC0_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTUTC1_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTUTC2_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTUTC3_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTUTC4_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTDTC0_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTDTC1_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTDTC2_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTDTC3_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTDTC4_DFLT: c_uint = 0x04010040;
pub const RS780_FVTHROTFBUSREG0_DFLT: c_uint = 0x00001001;
pub const RS780_FVTHROTFBUSREG1_DFLT: c_uint = 0x00002002;
pub const RS780_FVTHROTFBDSREG0_DFLT: c_uint = 0x00004001;
pub const RS780_FVTHROTFBDSREG1_DFLT: c_uint = 0x00020010;
pub const RS780_FVTHROTPWMUSREG0_DFLT: c_uint = 0x00002001;
pub const RS780_FVTHROTPWMUSREG1_DFLT: c_uint = 0x00004003;
pub const RS780_FVTHROTPWMDSREG0_DFLT: c_uint = 0x00002001;
pub const RS780_FVTHROTPWMDSREG1_DFLT: c_uint = 0x00004003;
pub const RS780_FVTHROTPWMFBDIVRANGEREG0_DFLT: c_uint = 0x37;
pub const RS780_FVTHROTPWMFBDIVRANGEREG1_DFLT: c_uint = 0x4b;
pub const RS780_FVTHROTPWMFBDIVRANGEREG2_DFLT: c_uint = 0x8b;
pub const RS780D_FVTHROTPWMFBDIVRANGEREG0_DFLT: c_uint = 0x8b;
pub const RS780D_FVTHROTPWMFBDIVRANGEREG1_DFLT: c_uint = 0x8c;
pub const RS780D_FVTHROTPWMFBDIVRANGEREG2_DFLT: c_uint = 0xb5;
pub const RS880D_FVTHROTPWMFBDIVRANGEREG0_DFLT: c_uint = 0x8d;
pub const RS880D_FVTHROTPWMFBDIVRANGEREG1_DFLT: c_uint = 0x8e;
pub const RS880D_FVTHROTPWMFBDIVRANGEREG2_DFLT: c_uint = 0xBa;
pub const RS780_FVTHROTPWMRANGE0_GPIO_DFLT: c_uint = 0x1a;
pub const RS780_FVTHROTPWMRANGE1_GPIO_DFLT: c_uint = 0x1a;
pub const RS780_FVTHROTPWMRANGE2_GPIO_DFLT: c_uint = 0x0;
pub const RS780_FVTHROTPWMRANGE3_GPIO_DFLT: c_uint = 0x0;
pub const RS780_SLOWCLKFEEDBACKDIV_DFLT: c_int = 110;
pub const RS780_CGCLKGATING_DFLT: c_uint = 0x0000E204;

