//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/mvebu/pinctrl-mvebu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Marvell MVEBU pinctrl driver
//
// Authors: Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// struct mvebu_mpp_ctrl_data - private data for the mpp ctrl operations
// @base: base address of pinctrl hardware
// @regmap.map: regmap structure
// @regmap.offset: regmap offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_mpp_ctrl_data {
    pub base: *mut void __iomem,
    pub map: *mut regmap,
    pub offset: u32,
    pub regmap: },
}

//
// struct mvebu_mpp_ctrl - describe a mpp control
// @name: name of the control group
// @pid: first pin id handled by this control
// @npins: number of pins controlled by this control
// @mpp_get: (optional) special function to get mpp setting
// @mpp_set: (optional) special function to set mpp setting
// @mpp_gpio_req: (optional) special function to request gpio
// @mpp_gpio_dir: (optional) special function to set gpio direction
//
// A mpp_ctrl describes a muxable unit, e.g. pin, group of pins, or
// internal function, inside the SoC. Each muxable unit can be switched
// between two or more different settings, e.g. assign mpp pin 13 to
// uart1 or sata.
//
// The mpp_get/_set functions are mandatory and are used to get/set a
// specific mode. The optional mpp_gpio_req/_dir functions can be used
// to allow pin settings with varying gpio pins.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_mpp_ctrl {
    pub name: *const c_char,
    pub pid: u8,
    pub npins: u8,
    pub pins: *mut unsigned,
    pub config): *mut c_ulong,
    pub config): c_ulong,
    pub pid): *mut *mut *mut int (mpp_gpio_req)(struct mvebu_mpp_ctrl_data data, unsigned,
    pub input): bool,
}

//
// struct mvebu_mpp_ctrl_setting - describe a mpp ctrl setting
// @val: ctrl setting value
// @name: ctrl setting name, e.g. uart2, spi0 - unique per mpp_mode
// @subname: (optional) additional ctrl setting name, e.g. rts, cts
// @variant: (optional) variant identifier mask
// @flags: (private) flags to store gpi/gpo/gpio capabilities
//
// A ctrl_setting describes a specific internal mux function that a mpp pin
// can be switched to. The value (val) will be written in the corresponding
// register for common mpp pin configuration registers on MVEBU. SoC specific
// mpp_get/_set function may use val to distinguish between different settings.
//
// The name will be used to switch to this setting in DT description, e.g.
// marvell,function = "uart2". subname is only for debugging purposes.
//
// If name is one of "gpi", "gpo", "gpio" gpio capabilities are
// parsed during initialization and stored in flags.
//
// The variant can be used to combine different revisions of one SoC to a
// common pinctrl driver. It is matched (AND) with variant of soc_info to
// determine if a setting is available on the current SoC revision.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_mpp_ctrl_setting {
    pub val: u8,
    pub name: *const c_char,
    pub subname: *const c_char,
    pub variant: u8,
    pub flags: u8,

}

//
// struct mvebu_mpp_mode - link ctrl and settings
// @pid: first pin id handled by this mode
// @settings: list of settings available for this mode
//
// A mode connects all available settings with the corresponding mpp_ctrl
// given by pid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_mpp_mode {
    pub pid: u8,
    pub settings: *mut mvebu_mpp_ctrl_setting,
}

//
// struct mvebu_pinctrl_soc_info - SoC specific info passed to pinctrl-mvebu
// @variant: variant mask of soc_info
// @controls: list of available mvebu_mpp_ctrls
// @control_data: optional array, one entry for each control
// @ncontrols: number of available mvebu_mpp_ctrls
// @modes: list of available mvebu_mpp_modes
// @nmodes: number of available mvebu_mpp_modes
// @gpioranges: list of pinctrl_gpio_ranges
// @ngpioranges: number of available pinctrl_gpio_ranges
//
// This struct describes all pinctrl related information for a specific SoC.
// If variant is unequal 0 it will be matched (AND) with variant of each
// setting and allows to distinguish between different revisions of one SoC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_pinctrl_soc_info {
    pub variant: u8,
    pub controls: *const mvebu_mpp_ctrl,
    pub control_data: *mut mvebu_mpp_ctrl_data,
    pub ncontrols: c_int,
    pub modes: *mut mvebu_mpp_mode,
    pub nmodes: c_int,
    pub gpioranges: *mut pinctrl_gpio_range,
    pub ngpioranges: c_int,
}

pub const MVEBU_MPPS_PER_REG: c_int = 8;
pub const MVEBU_MPP_BITS: c_int = 4;
pub const MVEBU_MPP_MASK: c_uint = 0xf;
extern "C" {
    pub fn mvebu_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mvebu_pinctrl_simple_mmio_probe(pdev: *mut platform_device) -> c_int;
}
