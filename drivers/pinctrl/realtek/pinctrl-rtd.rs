//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/realtek/pinctrl-rtd.h
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
// Copyright (c) 2023 Realtek Semiconductor Corp.
//
pub const NA: c_uint = 0xffffffff;
pub const PADDRI_4_8: c_int = 1;
pub const PADDRI_2_4: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_group_desc {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub num_pins: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_func_desc {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub num_groups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_mux_desc {
    pub name: *const c_char,
    pub mux_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_config_desc {
    pub name: *const c_char,
    pub reg_offset: c_uint,
    pub base_bit: c_uint,
    pub pud_en_offset: c_uint,
    pub pud_sel_offset: c_uint,
    pub curr_offset: c_uint,
    pub smt_offset: c_uint,
    pub power_offset: c_uint,
    pub curr_type: c_uint,
    pub input_volt_offset: c_uint,
    pub slew_rate_offset: c_uint,
    pub hvil_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_sconfig_desc {
    pub name: *const c_char,
    pub reg_offset: c_uint,
    pub dcycle_offset: c_uint,
    pub dcycle_maskbits: c_uint,
    pub ndrive_offset: c_uint,
    pub ndrive_maskbits: c_uint,
    pub pdrive_offset: c_uint,
    pub pdrive_maskbits: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_reg_range {
    pub offset: c_uint,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_range {
    pub ranges: *const rtd_reg_range,
    pub num_ranges: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_desc {
    pub name: *const c_char,
    pub mux_offset: c_uint,
    pub mux_mask: u32,
    pub functions: *const rtd_pin_mux_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pin_reg_list {
    pub reg_offset: c_uint,
    pub val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd_pinctrl_desc {
    pub pins: *const pinctrl_pin_desc,
    pub num_pins: c_uint,
    pub groups: *const rtd_pin_group_desc,
    pub num_groups: c_uint,
    pub functions: *const rtd_pin_func_desc,
    pub num_functions: c_uint,
    pub muxes: *const rtd_pin_desc,
    pub num_muxes: c_uint,
    pub configs: *const rtd_pin_config_desc,
    pub num_configs: c_uint,
    pub sconfigs: *const rtd_pin_sconfig_desc,
    pub num_sconfigs: c_uint,
    pub lists: *mut rtd_pin_reg_list,
    pub num_regs: c_uint,
    pub pin_range: *const rtd_pin_range,
}

extern "C" {
    pub fn rtd_pinctrl_probe(pdev: *mut platform_device, desc: *const rtd_pinctrl_desc) -> c_int;
}
