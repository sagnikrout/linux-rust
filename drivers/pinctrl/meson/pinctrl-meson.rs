//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/meson/pinctrl-meson.h
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
// Pin controller and GPIO driver for Amlogic Meson SoCs
//
// Copyright (C) 2014 Beniamino Galvani <b.galvani@gmail.com>
//

//
// struct meson_pmx_group - a pinmux group
//
// @name:	group name
// @pins:	pins in the group
// @num_pins:	number of pins in the group
// @is_gpio:	whether the group is a single GPIO group
// @reg:	register offset for the group in the domain mux registers
// @bit		bit index enabling the group
// @domain:	index of the domain this group belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pmx_group {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub num_pins: c_uint,
    pub data: *const c_void,
}

//
// struct meson_pmx_func - a pinmux function
//
// @name:	function name
// @groups:	groups in the function
// @num_groups:	number of groups in the function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pmx_func {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub num_groups: c_uint,
}

//
// struct meson_reg_desc - a register descriptor
//
// @reg:	register offset in the regmap
// @bit:	bit index in register
//
// The structure describes the information needed to control pull,
// pull-enable, direction, etc. for a single pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_reg_desc {
    pub reg: c_uint,
    pub bit: c_uint,
}

//
// enum meson_reg_type - type of registers encoded in @meson_reg_desc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum meson_reg_type {
    MESON_REG_PULLEN,
    MESON_REG_PULL,
    MESON_REG_DIR,
    MESON_REG_OUT,
    MESON_REG_IN,
    MESON_REG_DS,
    MESON_NUM_REG,
}

//
// enum meson_pinconf_drv - value of drive-strength supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum meson_pinconf_drv {
    MESON_PINCONF_DRV_500UA,
    MESON_PINCONF_DRV_2500UA,
    MESON_PINCONF_DRV_3000UA,
    MESON_PINCONF_DRV_4000UA,
}

//
// struct meson bank
//
// @name:	bank name
// @first:	first pin of the bank
// @last:	last pin of the bank
// @irq:	hwirq base number of the bank
// @regs:	array of register descriptors
//
// A bank represents a set of pins controlled by a contiguous set of
// bits in the domain registers. The structure specifies which bits in
// the regmap control the different functionalities. Each member of
// the @regs array refers to the first pin of the bank.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_bank {
    pub name: *const c_char,
    pub first: c_uint,
    pub last: c_uint,
    pub irq_first: c_int,
    pub irq_last: c_int,
    pub regs: [meson_reg_desc; MESON_NUM_REG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pinctrl_data {
    pub name: *const c_char,
    pub pins: *const pinctrl_pin_desc,
    pub groups: *const meson_pmx_group,
    pub funcs: *const meson_pmx_func,
    pub num_pins: c_uint,
    pub num_groups: c_uint,
    pub num_funcs: c_uint,
    pub banks: *const meson_bank,
    pub num_banks: c_uint,
    pub pmx_ops: *const pinmux_ops,
    pub pmx_data: *const c_void,
    pub pc): *mut *mut int (parse_dt)(struct meson_pinctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pinctrl {
    pub dev: *mut device,
    pub pcdev: *mut pinctrl_dev,
    pub desc: pinctrl_desc,
    pub data: *mut meson_pinctrl_data,
    pub reg_mux: *mut regmap,
    pub reg_pullen: *mut regmap,
    pub reg_pull: *mut regmap,
    pub reg_gpio: *mut regmap,
    pub reg_ds: *mut regmap,
    pub chip: gpio_chip,
    pub fwnode: *mut fwnode_handle,
}

// Common pmx functions
extern "C" {
    pub fn meson_pmx_get_funcs_count(pcdev: *mut pinctrl_dev) -> c_int;
}
// Common probe function
extern "C" {
    pub fn meson_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
// Common ao groups extra dt parse function for SoCs before g12a
extern "C" {
    pub fn meson8_aobus_parse_dt_extra(pc: *mut meson_pinctrl) -> c_int;
}
// Common extra dt parse function for SoCs like A1
extern "C" {
    pub fn meson_a1_parse_dt_extra(pc: *mut meson_pinctrl) -> c_int;
}
