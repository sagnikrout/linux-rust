//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/airoha/airoha-common.h
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
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
// Author: Benjamin Larsson <benjamin.larsson@genexis.eu>
// Author: Markus Gothe <markus.gothe@genexis.eu>
//

pub const AIROHA_NUM_PINS: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_reg {
    pub offset: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_pinctrl_mux_func {
    AIROHA_FUNC_MUX,
    AIROHA_FUNC_PWM_MUX,
    AIROHA_FUNC_PWM_EXT_MUX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_func_group {
    pub name: *const c_char,
    pub mux: airoha_pinctrl_mux_func,
    pub offset: u32,
    pub mask: u32,
    pub val: u32,
    pub regmap: [}; 2],
    pub regmap_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_func {
    pub desc: pinfunction,
    pub groups: *const airoha_pinctrl_func_group,
    pub group_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_conf {
    pub pin: u32,
    pub reg: airoha_pinctrl_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_gpiochip_regs {
// gpio
    pub data: *const u32,
    pub dir: *const u32,
    pub out: *const u32,
// irq
    pub status: *const u32,
    pub level: *const u32,
    pub edge: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_confs_info {
    pub confs: *const airoha_pinctrl_conf,
    pub num_confs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_pinctrl_confs_type {
    AIROHA_PINCTRL_CONFS_PULLUP,
    AIROHA_PINCTRL_CONFS_PULLDOWN,
    AIROHA_PINCTRL_CONFS_DRIVE_E2,
    AIROHA_PINCTRL_CONFS_DRIVE_E4,
    AIROHA_PINCTRL_CONFS_PCIE_RST_OD,

    AIROHA_PINCTRL_CONFS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl {
    pub ctrl: *mut pinctrl_dev,
    pub desc: pinctrl_desc,
    pub grps: *const pingroup,
    pub funcs: *const airoha_pinctrl_func,
    pub confs_info: *const airoha_pinctrl_confs_info,
    pub chip_scu: *mut regmap,
    pub regmap: *mut regmap,
    pub gpiochip: gpio_chip,
    pub gpio_regs: *mut airoha_gpiochip_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_pinctrl_match_data {
    pub chip_scu_compatible: *const c_char,
    pub pinctrl_name: *const c_char,
    pub pinctrl_owner: *mut module,
    pub pins: *const pinctrl_pin_desc,
    pub num_pins: c_uint,
    pub grps: *const pingroup,
    pub num_grps: c_uint,
    pub funcs: *const airoha_pinctrl_func,
    pub num_funcs: c_uint,
    pub confs_info: [airoha_pinctrl_confs_info; AIROHA_PINCTRL_CONFS_MAX],
}

extern "C" {
    pub fn airoha_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
