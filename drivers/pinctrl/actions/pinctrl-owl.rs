//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/actions/pinctrl-owl.h
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
// OWL SoC's Pinctrl definitions
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//
pub const OWL_PINCONF_SLEW_SLOW: c_int = 0;
pub const OWL_PINCONF_SLEW_FAST: c_int = 1;

// PAD PULL UP/DOWN CONFIGURES

pub const OWL_GPIO_PORT_A: c_int = 0;
pub const OWL_GPIO_PORT_B: c_int = 1;
pub const OWL_GPIO_PORT_C: c_int = 2;
pub const OWL_GPIO_PORT_D: c_int = 3;
pub const OWL_GPIO_PORT_E: c_int = 4;
pub const OWL_GPIO_PORT_F: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum owl_pinconf_drv {
    OWL_PINCONF_DRV_2MA,
    OWL_PINCONF_DRV_4MA,
    OWL_PINCONF_DRV_8MA,
    OWL_PINCONF_DRV_12MA,
}

// GPIO CTRL Bit Definition
pub const OWL_GPIO_CTLR_PENDING: c_int = 0;
pub const OWL_GPIO_CTLR_ENABLE: c_int = 1;
pub const OWL_GPIO_CTLR_SAMPLE_CLK_24M: c_int = 2;
// GPIO TYPE Bit Definition
pub const OWL_GPIO_INT_LEVEL_HIGH: c_int = 0;
pub const OWL_GPIO_INT_LEVEL_LOW: c_int = 1;
pub const OWL_GPIO_INT_EDGE_RISING: c_int = 2;
pub const OWL_GPIO_INT_EDGE_FALLING: c_int = 3;
pub const OWL_GPIO_INT_MASK: c_int = 3;
//
// struct owl_pullctl - Actions pad pull control register
// @reg: offset to the pull control register
// @shift: shift value of the register
// @width: width of the register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_pullctl {
    pub reg: c_int,
    pub shift: c_uint,
    pub width: c_uint,
}

//
// struct owl_st - Actions pad schmitt trigger enable register
// @reg: offset to the schmitt trigger enable register
// @shift: shift value of the register
// @width: width of the register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_st {
    pub reg: c_int,
    pub shift: c_uint,
    pub width: c_uint,
}

//
// struct owl_pingroup - Actions pingroup definition
// @name: name of the  pin group
// @pads: list of pins assigned to this pingroup
// @npads: size of @pads array
// @funcs: list of pinmux functions for this pingroup
// @nfuncs: size of @funcs array
// @mfpctl_reg: multiplexing control register offset
// @mfpctl_shift: multiplexing control register bit mask
// @mfpctl_width: multiplexing control register width
// @drv_reg: drive control register offset
// @drv_shift: drive control register bit mask
// @drv_width: driver control register width
// @sr_reg: slew rate control register offset
// @sr_shift: slew rate control register bit mask
// @sr_width: slew rate control register width
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_pingroup {
    pub name: *const c_char,
    pub pads: *mut c_uint,
    pub npads: c_uint,
    pub funcs: *mut c_uint,
    pub nfuncs: c_uint,
    pub mfpctl_reg: c_int,
    pub mfpctl_shift: c_uint,
    pub mfpctl_width: c_uint,
    pub drv_reg: c_int,
    pub drv_shift: c_uint,
    pub drv_width: c_uint,
    pub sr_reg: c_int,
    pub sr_shift: c_uint,
    pub sr_width: c_uint,
}

//
// struct owl_padinfo - Actions pinctrl pad info
// @pad: pad name of the SoC
// @pullctl: pull control register info
// @st: schmitt trigger register info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_padinfo {
    pub pad: c_int,
    pub pullctl: *mut owl_pullctl,
    pub st: *mut owl_st,
}

//
// struct owl_pinmux_func - Actions pinctrl mux functions
// @name: name of the pinmux function.
// @groups: array of pin groups that may select this function.
// @ngroups: number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_pinmux_func {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: c_uint,
}

//
// struct owl_gpio_port - Actions GPIO port info
// @offset: offset of the GPIO port.
// @pins: number of pins belongs to the GPIO port.
// @outen: offset of the output enable register.
// @inen: offset of the input enable register.
// @dat: offset of the data register.
// @intc_ctl: offset of the interrupt control register.
// @intc_pd: offset of the interrupt pending register.
// @intc_msk: offset of the interrupt mask register.
// @intc_type: offset of the interrupt type register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_gpio_port {
    pub offset: c_uint,
    pub pins: c_uint,
    pub outen: c_uint,
    pub inen: c_uint,
    pub dat: c_uint,
    pub intc_ctl: c_uint,
    pub intc_pd: c_uint,
    pub intc_msk: c_uint,
    pub intc_type: c_uint,
    pub shared_ctl_offset: u8,
}

//
// struct owl_pinctrl_soc_data - Actions pin controller driver configuration
// @pins: array describing all pins of the pin controller.
// @npins: number of entries in @pins.
// @functions: array describing all mux functions of this SoC.
// @nfunction: number of entries in @functions.
// @groups: array describing all pin groups of this SoC.
// @ngroups: number of entries in @groups.
// @padinfo: array describing the pad info of this SoC.
// @ngpios: number of pingroups the driver should expose as GPIOs.
// @ports: array describing all GPIO ports of this SoC.
// @nports: number of GPIO ports in this SoC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_pinctrl_soc_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub functions: *const owl_pinmux_func,
    pub nfunctions: c_uint,
    pub groups: *const owl_pingroup,
    pub ngroups: c_uint,
    pub padinfo: *const owl_padinfo,
    pub ngpios: c_uint,
    pub ports: *const owl_gpio_port,
    pub nports: c_uint,
    pub arg): *mut u32,
    pub arg): *mut u32,
}
