//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/signal_types.h
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
// Minimum pixel clock, in KHz. For TMDS signal is 25.00 MHz
pub const TMDS_MIN_PIXEL_CLOCK: c_int = 25000;
// Maximum pixel clock, in KHz. For TMDS signal is 165.00 MHz
pub const TMDS_MAX_PIXEL_CLOCK: c_int = 165000;
// Maximum pixel clock, in KHz. For HDMI2 TMDS signal is 600 MHz
pub const HDMI2_TMDS_MAX_PIXEL_CLOCK: c_int = 600000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum signal_type {
    SIGNAL_TYPE_NONE		= 0L,		/* no signal */
    SIGNAL_TYPE_DVI_SINGLE_LINK	= (1 << 0),
    SIGNAL_TYPE_DVI_DUAL_LINK	= (1 << 1),
    SIGNAL_TYPE_HDMI_TYPE_A		= (1 << 2),
    SIGNAL_TYPE_LVDS		= (1 << 3),
    SIGNAL_TYPE_RGB			= (1 << 4),
    SIGNAL_TYPE_DISPLAY_PORT	= (1 << 5),
    SIGNAL_TYPE_DISPLAY_PORT_MST	= (1 << 6),
    SIGNAL_TYPE_EDP			= (1 << 7),
    SIGNAL_TYPE_HDMI_FRL		= (1 << 8),
    SIGNAL_TYPE_VIRTUAL		= (1 << 9),	/* Virtual Display */
}

// help functions for signal types manipulation
//
// dc_is_rgb_signal() - Whether the signal is analog RGB.
//
// Returns whether the given signal type is an analog RGB signal
// that is used with a DAC on VGA or DVI-I connectors.
// Not to be confused with other uses of "RGB", such as RGB color space.
//
