//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/gpio/hw_gpio.h
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
#[derive(Copy, Clone)]
pub struct addr_mask {
    pub addr: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_gpio_pin {
    pub funcs: *const hw_gpio_pin_funcs,
    pub id: gpio_id,
    pub en: u32,
    pub mode: gpio_mode,
    pub opened: bool,
    pub ctx: *mut dc_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_gpio_pin_funcs {
    pub ptr): *mut hw_gpio_pin,
    pub mode): gpio_mode,
    pub value): *mut u32,
    pub value): u32,
    pub config_data): *const gpio_config_data,
    pub mode): gpio_mode,
    pub pin): *mut hw_gpio_pin,
}

// Register indices are represented by member variables
// and are to be filled in by constructors of derived classes.
// These members permit the use of common code
// for programming registers, where the sequence is the same
// but register sets are different.
// Some GPIOs have HW mux which allows to choose
// what is the source of the signal in HW mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_gpio_pin_reg {
    pub DC_GPIO_DATA_MASK: addr_mask,
    pub DC_GPIO_DATA_A: addr_mask,
    pub DC_GPIO_DATA_EN: addr_mask,
    pub DC_GPIO_DATA_Y: addr_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_gpio_mux_reg {
    pub GPIO_MUX_CONTROL: addr_mask,
    pub GPIO_MUX_STEREO_SEL: addr_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_gpio {
    pub base: hw_gpio_pin,
// variables to save register value
    pub mask: u32,
    pub a: u32,
    pub en: u32,
    pub mux: u32,
    pub store: },
// GPIO MUX support
    pub mux_supported: bool,
    pub regs: *const gpio_registers,
}

//
// Shared helper used by all GPIO register helpers that pass a field shift
// (stored as uint32_t) into register functions that expect uint8_t.
//
