//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/tegra/pinctrl-tegra.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for the NVIDIA Tegra pinmux
//
// Copyright (c) 2011, NVIDIA CORPORATION.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pingroup_config {
    pub is_sfsel: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pmx {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub soc: *const tegra_pinctrl_soc_data,
    pub functions: *mut tegra_function,
    pub group_pins: *const c_char,
    pub gpio_range: pinctrl_gpio_range,
    pub desc: pinctrl_desc,
    pub nbanks: c_int,
    pub regs: *mut void __iomem,
    pub backup_regs: *mut u32,
    pub num_pingroup_configs: c_uint,
    pub __counted_by(num_pingroup_configs): tegra_pingroup_config pingroup_configs[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_pinconf_param {
// argument: tegra_pinconf_pull
    TEGRA_PINCONF_PARAM_PULL,
// argument: tegra_pinconf_tristate
    TEGRA_PINCONF_PARAM_TRISTATE,
// argument: Boolean
    TEGRA_PINCONF_PARAM_ENABLE_INPUT,
// argument: Boolean
    TEGRA_PINCONF_PARAM_OPEN_DRAIN,
// argument: Boolean
    TEGRA_PINCONF_PARAM_LOCK,
// argument: Boolean
    TEGRA_PINCONF_PARAM_IORESET,
// argument: Boolean
    TEGRA_PINCONF_PARAM_RCV_SEL,
// argument: Boolean
    TEGRA_PINCONF_PARAM_HIGH_SPEED_MODE,
// argument: Boolean
    TEGRA_PINCONF_PARAM_SCHMITT,
// argument: Boolean
    TEGRA_PINCONF_PARAM_LOW_POWER_MODE,
// argument: Integer, range is HW-dependant
    TEGRA_PINCONF_PARAM_DRIVE_DOWN_STRENGTH,
// argument: Integer, range is HW-dependant
    TEGRA_PINCONF_PARAM_DRIVE_UP_STRENGTH,
// argument: Integer, range is HW-dependant
    TEGRA_PINCONF_PARAM_SLEW_RATE_FALLING,
// argument: Integer, range is HW-dependant
    TEGRA_PINCONF_PARAM_SLEW_RATE_RISING,
// argument: Integer, range is HW-dependant
    TEGRA_PINCONF_PARAM_DRIVE_TYPE,
// argument: Boolean
    TEGRA_PINCONF_PARAM_GPIO_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_pinconf_pull {
    TEGRA_PINCONFIG_PULL_NONE,
    TEGRA_PINCONFIG_PULL_DOWN,
    TEGRA_PINCONFIG_PULL_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_pinconf_tristate {
    TEGRA_PINCONFIG_DRIVEN,
    TEGRA_PINCONFIG_TRISTATE,
}

//
// struct tegra_function - Tegra pinctrl mux function
// @name: The name of the function, exported to pinctrl core.
// @groups: An array of pin groups that may select this function.
// @ngroups: The number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_function {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub ngroups: unsigned,
}

//
// struct tegra_pingroup - Tegra pin group
// @name		The name of the pin group.
// @pins		An array of pin IDs included in this pin group.
// @npins		The number of entries in @pins.
// @funcs		The mux functions which can be muxed onto this group.
// @mux_reg:		Mux register offset.
// This register contains the mux, einput, odrain, lock,
// ioreset, rcv_sel parameters.
// @mux_bank:		Mux register bank.
// @mux_bit:		Mux register bit.
// @pupd_reg:		Pull-up/down register offset.
// @pupd_bank:		Pull-up/down register bank.
// @pupd_bit:		Pull-up/down register bit.
// @tri_reg:		Tri-state register offset.
// @tri_bank:		Tri-state register bank.
// @tri_bit:		Tri-state register bit.
// @einput_bit:		Enable-input register bit.
// @odrain_bit:		Open-drain register bit.
// @lock_bit:		Lock register bit.
// @ioreset_bit:	IO reset register bit.
// @rcv_sel_bit:	Receiver select bit.
// @drv_reg:		Drive fields register offset.
// This register contains hsm, schmitt, lpmd, drvdn,
// drvup, slwr, slwf, and drvtype parameters.
// @drv_bank:		Drive fields register bank.
// @hsm_bit:		High Speed Mode register bit.
// @sfsel_bit:		GPIO/SFIO selection register bit.
// @schmitt_bit:	Schmitt register bit.
// @lpmd_bit:		Low Power Mode register bit.
// @drvdn_bit:		Drive Down register bit.
// @drvdn_width:	Drive Down field width.
// @drvup_bit:		Drive Up register bit.
// @drvup_width:	Drive Up field width.
// @slwr_bit:		Slew Rising register bit.
// @slwr_width:		Slew Rising field width.
// @slwf_bit:		Slew Falling register bit.
// @slwf_width:		Slew Falling field width.
// @lpdr_bit:		Base driver enabling bit.
// @drvtype_bit:	Drive type register bit.
// @parked_bitmask:	Parked register mask. 0 if unsupported.
//
// -1 in a *_reg field means that feature is unsupported for this group.
// *_bank and *_reg values are irrelevant when *_reg is -1.
// When *_reg is valid, *_bit may be -1 to indicate an unsupported feature.
//
// A representation of a group of pins (possibly just one pin) in the Tegra
// pin controller. Each group allows some parameter or parameters to be
// configured. The most common is mux function selection. Many others exist
// such as pull-up/down, tri-state, etc. Tegra's pin controller is complex;
// certain groups may only support configuring certain parameters, hence
// each parameter is optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pingroup {
    pub name: *const c_char,
    pub pins: *const unsigned,
    pub npins: u8,
    pub funcs: [u8; 4],
    pub mux_reg: i32,
    pub pupd_reg: i32,
    pub tri_reg: i32,
    pub drv_reg: i32,
    pub mux_bank:2: u32,
    pub pupd_bank:2: u32,
    pub tri_bank:2: u32,
    pub drv_bank:2: u32,
    pub mux_bit:6: i32,
    pub pupd_bit:6: i32,
    pub tri_bit:6: i32,
    pub einput_bit:6: i32,
    pub odrain_bit:6: i32,
    pub lock_bit:6: i32,
    pub ioreset_bit:6: i32,
    pub rcv_sel_bit:6: i32,
    pub hsm_bit:6: i32,
    pub sfsel_bit:6: i32,
    pub schmitt_bit:6: i32,
    pub lpmd_bit:6: i32,
    pub drvdn_bit:6: i32,
    pub drvup_bit:6: i32,
    pub slwr_bit:6: i32,
    pub slwf_bit:6: i32,
    pub lpdr_bit:6: i32,
    pub drvtype_bit:6: i32,
    pub drvdn_width:6: i32,
    pub drvup_width:6: i32,
    pub slwr_width:6: i32,
    pub slwf_width:6: i32,
    pub parked_bitmask: u32,
}

//
// struct tegra_pinctrl_soc_data - Tegra pin controller driver configuration
// @ngpios:		The number of GPIO pins the pin controller HW affects.
// @gpio_compatible:	Device-tree GPIO compatible string.
// @pins:		An array describing all pins the pin controller affects.
// All pins which are also GPIOs must be listed first within the
// array, and be numbered identically to the GPIO controller's
// numbering.
// @npins:		The number of entries in @pins.
// @functions:		An array describing all mux functions the SoC supports.
// @nfunctions:		The number of entries in @functions.
// @groups:		An array describing all pin groups the pin SoC supports.
// @ngroups:		The number of entries in @groups.
// @hsm_in_mux:		High-speed mode field. Only applicable to devices with one pin per group.
// @schmitt_in_mux:	Schmitt trigger field. Only applicable to devices with one pin per group.
// @drvtype_in_mux:	Drivetype field. Only applicable to devices with one pin per group.
// @sfsel_in_mux:	Special function selection field.
// Only applicable to devices with one pin per group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_pinctrl_soc_data {
    pub ngpios: unsigned,
    pub gpio_compatible: *const c_char,
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub functions: *const *const c_char,
    pub nfunctions: unsigned,
    pub groups: *const tegra_pingroup,
    pub ngroups: unsigned,
    pub hsm_in_mux: bool,
    pub schmitt_in_mux: bool,
    pub drvtype_in_mux: bool,
    pub sfsel_in_mux: bool,
}
