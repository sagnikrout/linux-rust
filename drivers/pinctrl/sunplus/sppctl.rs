//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/sunplus/sppctl.h
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


// SPDX-License-Identifier: GPL-2.0
//
// SP7021 Pin Controller Driver.
// Copyright (C) Sunplus Tech / Tibbo Tech.
//

pub const SPPCTL_GPIO_OFF_FIRST: c_uint = 0x00;
pub const SPPCTL_GPIO_OFF_MASTER: c_uint = 0x00;
pub const SPPCTL_GPIO_OFF_OE: c_uint = 0x20;
pub const SPPCTL_GPIO_OFF_OUT: c_uint = 0x40;
pub const SPPCTL_GPIO_OFF_IN: c_uint = 0x60;
pub const SPPCTL_GPIO_OFF_IINV: c_uint = 0x80;
pub const SPPCTL_GPIO_OFF_OINV: c_uint = 0xa0;
pub const SPPCTL_GPIO_OFF_OD: c_uint = 0xc0;

pub const SPPCTL_FULLY_PINMUX_UPPER_SHIFT: c_int = 8;
//
// Mask-fields and control-fields of MOON registers of SP7021 are
// arranged as shown below:
//
// register |  mask-fields | control-fields
// ----------+--------------+----------------
// base[0]  |  (31 : 16)   |   (15 : 0)
// base[1]  |  (31 : 24)   |   (15 : 0)
// base[2]  |  (31 : 24)   |   (15 : 0)
// :     |      :       |       :
//
// where mask-fields are used to protect control-fields from write-in
// accidentally. Set the corresponding bits in the mask-field before
// you write a value into a control-field.
//
pub const SPPCTL_MOON_REG_MASK_SHIFT: c_int = 16;

pub const SPPCTL_IOP_CONFIGS: c_uint = 0xff;

//
// enum mux_first_reg - Define modes of access of FIRST register
// @mux_f_mux:  Set the corresponding pin to a fully-pinmux pin
// @mux_f_gpio: Set the corresponding pin to a GPIO or IOP pin
// @mux_f_keep: Don't change (keep intact)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mux_first_reg {
    mux_f_mux = 0,
    mux_f_gpio = 1,
    mux_f_keep = 2,
}

//
// enum mux_master_reg - Define modes of access of MASTER register
// @mux_m_iop:  Set the corresponding pin to an IO processor (IOP) pin
// @mux_m_gpio: Set the corresponding pin to a digital GPIO pin
// @mux_m_keep: Don't change (keep intact)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mux_master_reg {
    mux_m_iop = 0,
    mux_m_gpio = 1,
    mux_m_keep = 2,
}

//
// enum pinmux_type - Define types of pinmux pins
// @pinmux_type_fpmx: A fully-pinmux pin
// @pinmux_type_grp:  A group-pinmux pin
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pinmux_type {
    pinmux_type_fpmx,
    pinmux_type_grp,
}

//
// struct grp2fp_map - A map storing indexes
// @f_idx: an index to function table
// @g_idx: an index to group table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grp2fp_map {
    pub f_idx: u16,
    pub g_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sppctl_pdata {
    pub /: *mut *mut *mut void __iomem moon2_base; / MOON2,
    pub /: *mut *mut *mut void __iomem gpioxt_base; / MASTER, OE, OUT, IN, I_INV, O_INV, OD,
    pub /: *mut *mut *mut void __iomem first_base; / FIRST,
    pub /: *mut *mut *mut void __iomem moon1_base; / MOON1,
    pub pctl_desc: pinctrl_desc,
    pub pctl_dev: *mut pinctrl_dev,
    pub pctl_grange: pinctrl_gpio_range,
    pub spp_gchip: *mut sppctl_gpio_chip,
    pub unq_grps: *const c_char,
    pub unq_grps_sz: usize,
    pub g2fp_maps: *mut grp2fp_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sppctl_grp {
    pub name: *const *const c_char,
    pub /: *const *const u8 gval; / group number,
    pub /: *const *const *const unsigned  pins; / list of pins,
    pub /: *const *const unsigned int pnum; / number of pins,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sppctl_func {
    pub name: *const *const c_char,
    pub /: *const *const pinmux_type type; / function type,
    pub /: *const *const u8 roff; / register offset,
    pub /: *const *const u8 boff; / bit offset,
    pub /: *const *const u8 blen; / bit length,
    pub /: *const *const *const sppctl_grp  grps; / list of groups,
    pub /: *const *const unsigned int gnum; / number of groups,
}
