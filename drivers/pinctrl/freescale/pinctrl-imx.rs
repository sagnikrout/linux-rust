//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/freescale/pinctrl-imx.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// IMX pinmux core definitions
//
// Copyright (C) 2012 Freescale Semiconductor, Inc.
// Copyright (C) 2012 Linaro Ltd.
//
// Author: Dong Aisheng <dong.aisheng@linaro.org>
//

//
// struct imx_pin_mmio - MMIO pin configurations
// @mux_mode: the mux mode for this pin.
// @input_reg: the select input register offset for this pin if any
// 0 if no select input setting needed.
// @input_val: the select input value for this pin.
// @configs: the config for this pin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pin_mmio {
    pub mux_mode: c_uint,
    pub input_reg: u16,
    pub input_val: c_uint,
    pub config: c_ulong,
}

//
// struct imx_pin_scu - SCU pin configurations
// @mux: the mux mode for this pin.
// @configs: the config for this pin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pin_scu {
    pub mux_mode: c_uint,
    pub config: c_ulong,
}

//
// struct imx_pin - describes a single i.MX pin
// @pin: the pin_id of this pin
// @conf: config type of this pin, either mmio or scu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pin {
    pub pin: c_uint,
    pub mmio: imx_pin_mmio,
    pub scu: imx_pin_scu,
    pub conf: },
}

//
// struct imx_pin_reg - describe a pin reg map
// @mux_reg: mux register offset
// @conf_reg: config register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pin_reg {
    pub mux_reg: i16,
    pub conf_reg: i16,
}

//
// @dev: a pointer back to containing device
// @base: the offset to the controller in virtual memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub base: *mut void __iomem,
    pub input_sel_base: *mut void __iomem,
    pub info: *const imx_pinctrl_soc_info,
    pub pin_regs: *mut imx_pin_reg,
    pub group_index: c_uint,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pinctrl_soc_info {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub flags: c_uint,
    pub gpr_compatible: *const c_char,
// MUX_MODE shift and mask in case SHARE_MUX_CONF_REG
    pub mux_mask: c_uint,
    pub mux_shift: u8,
    pub input): bool,
    pub config): *mut c_ulong,
    pub num_configs): *mut *mut unsigned long configs, unsigned int,
    pub list_p): *const __be32,
}

pub const NO_MUX: c_uint = 0x0;
pub const NO_PAD: c_uint = 0x0;

pub const IMX_MUX_MASK: c_uint = 0x7;

pub const BP_PAD_CTL_IFMUX: c_int = 27;
extern "C" {
    pub fn imx_pinctrl_sc_ipc_init(pdev: *mut platform_device) -> c_int;
}
