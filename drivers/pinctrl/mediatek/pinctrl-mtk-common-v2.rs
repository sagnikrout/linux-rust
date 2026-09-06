//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/mediatek/pinctrl-mtk-common-v2.h
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
// Copyright (C) 2018 MediaTek Inc.
//
// Author: Sean Wang <sean.wang@mediatek.com>
//

pub const MTK_INPUT: c_int = 0;
pub const MTK_OUTPUT: c_int = 1;
pub const MTK_DISABLE: c_int = 0;
pub const MTK_ENABLE: c_int = 1;
pub const MTK_PULLDOWN: c_int = 0;
pub const MTK_PULLUP: c_int = 1;

// MTK_PULL_RSEL_TYPE can select resistance and can be
// turned on/off itself. But it can't be selected pull up/down
//

// MTK_PULL_PU_PD_RSEL_TYPE is a type which is controlled by
// MTK_PULL_PU_PD_TYPE and MTK_PULL_RSEL_TYPE.
//

// List these attributes which could be modified for the pin
// Group the pins by the driving current
// struct mtk_pin_field - the structure that holds the information of the field
// used to describe the attribute for the pin
// @base:		the index pointing to the entry in base address list
// @offset:		the register offset relative to the base address
// @mask:		the mask used to filter out the field from the register
// @bitpos:		the start bit relative to the register
// @next:		the indication that the field would be extended to the
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_field {
    pub index: u8,
    pub offset: u32,
    pub mask: u32,
    pub bitpos: u8,
    pub next: u8,
}

// struct mtk_pin_field_calc - the structure that holds the range providing
// the guide used to look up the relevant field
// @s_pin:		the start pin within the range
// @e_pin:		the end pin within the range
// @i_base:		the index pointing to the entry in base address list
// @s_addr:		the start address for the range
// @x_addrs:		the address distance between two consecutive registers
// within the range
// @s_bit:		the start bit for the first register within the range
// @x_bits:		the bit distance between two consecutive pins within
// the range
// @sz_reg:		the size of bits in a register
// @fixed:		the consecutive pins share the same bits with the 1st
// pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_field_calc {
    pub s_pin: u16,
    pub e_pin: u16,
    pub i_base: u8,
    pub s_addr: u32,
    pub x_addrs: u8,
    pub s_bit: u8,
    pub x_bits: u8,
    pub sz_reg: u8,
    pub fixed: u8,
}

//
// struct mtk_pin_rsel - the structure that provides bias resistance selection.
// @s_pin:		the start pin within the rsel range
// @e_pin:		the end pin within the rsel range
// @rsel_index:	the rsel bias resistance index
// @up_rsel:	the pullup rsel bias resistance value
// @down_rsel:	the pulldown rsel bias resistance value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_rsel {
    pub s_pin: u16,
    pub e_pin: u16,
    pub rsel_index: u16,
    pub up_rsel: u32,
    pub down_rsel: u32,
}

// struct mtk_pin_reg_calc - the structure that holds all ranges used to
// determine which register the pin would make use of
// for certain pin attribute.
// @range:		     the start address for the range
// @nranges:		     the number of items in the range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_reg_calc {
    pub range: *const mtk_pin_field_calc,
    pub nranges: c_uint,
}

//
// struct mtk_func_desc - the structure that providing information
// all the funcs for this pin
// @name:		the name of function
// @muxval:		the mux to the function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_func_desc {
    pub name: *const c_char,
    pub muxval: u8,
}

//
// struct mtk_eint_desc - the structure that providing information
// for eint data per pin
// @eint_m:		the eint mux for this pin
// @eitn_n:		the eint number for this pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_desc {
    pub eint_m: u16,
    pub eint_n: u16,
}

//
// struct mtk_pin_desc - the structure that providing information
// for each pin of chips
// @number:		unique pin number from the global pin number space
// @name:		name for this pin
// @eint:		the eint data for this pin
// @drv_n:		the index with the driving group
// @funcs:		all available functions for this pins (only used in
// those drivers compatible to pinctrl-mtk-common.c-like
// ones)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_desc {
    pub number: c_uint,
    pub name: *const c_char,
    pub eint: mtk_eint_desc,
    pub drv_n: u8,
    pub funcs: *mut mtk_func_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pinctrl_group {
    pub name: *const c_char,
    pub config: c_ulong,
    pub pin: unsigned,
}

// struct mtk_pin_soc - the structure that holds SoC-specific data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_soc {
    pub reg_cal: *const mtk_pin_reg_calc,
    pub pins: *const mtk_pin_desc,
    pub npins: c_uint,
    pub grps: *const group_desc,
    pub ngrps: c_uint,
    pub funcs: *const pinfunction,
    pub nfuncs: c_uint,
    pub eint_regs: *const mtk_eint_regs,
    pub eint_hw: *const mtk_eint_hw,
    pub eint_pin: *mut mtk_eint_pin,
// Specific parameters per SoC
    pub gpio_m: u8,
    pub ies_present: bool,
    pub base_names: *const *const c_char,
    pub nbase_names: c_uint,
    pub pull_type: *const c_uint,
    pub pin_rsel: *const mtk_pin_rsel,
    pub npin_rsel: c_uint,
// Specific pinconfig operations
    pub desc): *const mtk_pin_desc,
    pub res): *const *const mtk_pin_desc desc, int,
    pub pullup): *const *const mtk_pin_desc desc, bool,
    pub res): *const *const mtk_pin_desc desc, bool pullup, int,
    pub arg): *const *const mtk_pin_desc desc, u32 pullup, u32,
    pub arg): *const *const *const mtk_pin_desc desc, u32 pullup, u32,
    pub arg): *const *const mtk_pin_desc desc, u32,
    pub val): *const *const mtk_pin_desc desc, int,
    pub arg): u32,
    pub val): *mut u32,
    pub arg): *const *const mtk_pin_desc desc, u32,
    pub val): *const *const mtk_pin_desc desc, u32,
// Specific driver data
    pub driver_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pinctrl {
    pub pctrl: *mut pinctrl_dev,
    pub base: *mut void __iomem,
    pub nbase: u8,
    pub dev: *mut device,
    pub chip: gpio_chip,
    pub soc: *const mtk_pin_soc,
    pub eint: *mut mtk_eint,
    pub groups: *mut mtk_pinctrl_group,
    pub grp_names: *const c_char,
// lock pin's register resource to avoid multiple threads issue
    pub lock: spinlock_t,
// identify rsel setting by si unit or rsel define in dts node
    pub rsel_si_unit: bool,
}

extern "C" {
    pub fn mtk_rmw(pctl: *mut mtk_pinctrl, i: u8, reg: u32, mask: u32, set: u32);
}
extern "C" {
    pub fn mtk_build_eint(hw: *mut mtk_pinctrl, pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mtk_is_virt_gpio(hw: *mut mtk_pinctrl, gpio_n: c_uint) -> bool;
}
