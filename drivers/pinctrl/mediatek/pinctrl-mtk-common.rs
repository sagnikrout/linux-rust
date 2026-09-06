//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/mediatek/pinctrl-mtk-common.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Hongzhou.Yang <hongzhou.yang@mediatek.com>
//

pub const NO_EINT_SUPPORT: c_int = 255;
pub const MT_EDGE_SENSITIVE: c_int = 0;
pub const MT_LEVEL_SENSITIVE: c_int = 1;
pub const EINT_DBNC_SET_DBNC_BITS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_desc_function {
    pub name: *const c_char,
    pub muxval: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_desc_eint {
    pub eintmux: c_uchar,
    pub eintnum: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_desc_pin {
    pub pin: pinctrl_pin_desc,
    pub eint: mtk_desc_eint,
    pub functions: *const mtk_desc_function,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pinctrl_group {
    pub name: *const c_char,
    pub config: c_ulong,
    pub pin: unsigned,
}

//
// struct mtk_drv_group_desc - Provide driving group data.
// @max_drv: The maximum current of this group.
// @min_drv: The minimum current of this group.
// @low_bit: The lowest bit of this group.
// @high_bit: The highest bit of this group.
// @step: The step current of this group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_drv_group_desc {
    pub min_drv: c_uchar,
    pub max_drv: c_uchar,
    pub low_bit: c_uchar,
    pub high_bit: c_uchar,
    pub step: c_uchar,
}

//
// struct mtk_pin_drv_grp - Provide each pin driving info.
// @pin: The pin number.
// @offset: The offset of driving register for this pin.
// @bit: The bit of driving register for this pin.
// @grp: The group for this pin belongs to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_drv_grp {
    pub pin: c_ushort,
    pub offset: c_ushort,
    pub bit: c_uchar,
    pub grp: c_uchar,
}

//
// struct mtk_pin_spec_pupd_set_samereg
// - For special pins' pull up/down setting which resides in same register
// @pin: The pin number.
// @offset: The offset of special pull up/down setting register.
// @pupd_bit: The pull up/down bit in this register.
// @r0_bit: The r0 bit of pull resistor.
// @r1_bit: The r1 bit of pull resistor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_spec_pupd_set_samereg {
    pub pin: c_ushort,
    pub offset: c_ushort,
    pub pupd_bit: c_uchar,
    pub r1_bit: c_uchar,
    pub r0_bit: c_uchar,
}

//
// struct mtk_pin_ies_set - For special pins' ies and smt setting.
// @start: The start pin number of those special pins.
// @end: The end pin number of those special pins.
// @offset: The offset of special setting register.
// @bit: The bit of special setting register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pin_ies_smt_set {
    pub start: c_ushort,
    pub end: c_ushort,
    pub offset: c_ushort,
    pub bit: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_offsets {
    pub name: *const c_char,
    pub stat: c_uint,
    pub ack: c_uint,
    pub mask: c_uint,
    pub mask_set: c_uint,
    pub mask_clr: c_uint,
    pub sens: c_uint,
    pub sens_set: c_uint,
    pub sens_clr: c_uint,
    pub soft: c_uint,
    pub soft_set: c_uint,
    pub soft_clr: c_uint,
    pub pol: c_uint,
    pub pol_set: c_uint,
    pub pol_clr: c_uint,
    pub dom_en: c_uint,
    pub dbnc_ctrl: c_uint,
    pub dbnc_set: c_uint,
    pub dbnc_clr: c_uint,
    pub port_mask: u8,
    pub ports: u8,
}

//
// struct mtk_pinctrl_devdata - Provide HW GPIO related data.
// @pins: An array describing all pins the pin controller affects.
// @npins: The number of entries in @pins.
//
// @grp_desc: The driving group info.
// @pin_drv_grp: The driving group for all pins.
// @spec_ies: Special pin setting for input enable
// @n_spec_ies: Number of entries in spec_ies
// @spec_pupd: Special pull up/down setting
// @n_spec_pupd: Number of entries in spec_pupd
// @spec_smt: Special pin setting for schmitt
// @n_spec_smt: Number of entries in spec_smt
// @spec_pull_set: Each SoC may have special pins for pull up/down setting,
// these pins' pull setting are very different, they have separate pull
// up/down bit, R0 and R1 resistor bit, so they need special pull setting.
// If special setting is success, this should return 0, otherwise it should
// return non-zero value.
// @spec_ies_smt_set: Some pins are irregular, their input enable and smt
// control register are discontinuous, but they are mapping together. That
// means when user set smt, input enable is set at the same time. So they
// also need special control. If special control is success, this should
// return 0, otherwise return non-zero value.
// @spec_pinmux_set: In some cases, there are two pinmux functions share
// the same value in the same segment of pinmux control register. If user
// want to use one of the two functions, they need an extra bit setting to
// select the right one.
// @spec_dir_set: In very few SoCs, direction control registers are not
// arranged continuously, they may be cut to parts. So they need special
// dir setting.
// @mt8365_set_clr_mode: In mt8365, some pins won't set correcty because they
// need to use the main R/W register to read/update/write the modes instead of
// the SET/CLR register.
//
// @dir_offset: The direction register offset.
// @pullen_offset: The pull-up/pull-down enable register offset.
// @pinmux_offset: The pinmux register offset.
//
// @type1_start: Some chips have two base addresses for pull select register,
// that means some pins use the first address and others use the second. This
// member record the start of pin number to use the second address.
// @type1_end: The end of pin number to use the second address.
//
// @port_shf: The shift between two registers.
// @port_mask: The mask of register.
// @port_align: Provide clear register and set register step.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pinctrl_devdata {
    pub pins: *const mtk_desc_pin,
    pub npins: c_uint,
    pub grp_desc: *const mtk_drv_group_desc,
    pub n_grp_cls: c_uint,
    pub pin_drv_grp: *const mtk_pin_drv_grp,
    pub n_pin_drv_grps: c_uint,
    pub spec_ies: *const mtk_pin_ies_smt_set,
    pub n_spec_ies: c_uint,
    pub spec_pupd: *const mtk_pin_spec_pupd_set_samereg,
    pub n_spec_pupd: c_uint,
    pub spec_smt: *const mtk_pin_ies_smt_set,
    pub n_spec_smt: c_uint,
    pub r1r0): unsigned int pin, bool isup, unsigned int,
    pub arg): unsigned int pin, int value, enum pin_config_param,
    pub mode): c_uint,
    pub pin): *mut *mut *mut void (spec_dir_set)(unsigned int reg_addr, unsigned int,
    pub isup): bool enable, bool,
    pub dir_offset: c_uint,
    pub ies_offset: c_uint,
    pub smt_offset: c_uint,
    pub pullen_offset: c_uint,
    pub pullsel_offset: c_uint,
    pub dout_offset: c_uint,
    pub din_offset: c_uint,
    pub pinmux_offset: c_uint,
    pub type1_start: c_ushort,
    pub type1_end: c_ushort,
    pub port_shf: c_uchar,
    pub port_mask: c_uchar,
    pub port_align: c_uchar,
    pub eint_hw: mtk_eint_hw,
    pub eint_regs: *mut mtk_eint_regs,
    pub mode_mask: c_uint,
    pub mode_per_reg: c_uint,
    pub mode_shf: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pinctrl {
    pub regmap1: *mut regmap,
    pub regmap2: *mut regmap,
    pub pctl_desc: pinctrl_desc,
    pub dev: *mut device,
    pub chip: *mut gpio_chip,
    pub groups: *mut mtk_pinctrl_group,
    pub ngroups: unsigned,
    pub grp_names: *const c_char,
    pub pctl_dev: *mut pinctrl_dev,
    pub devdata: *const mtk_pinctrl_devdata,
    pub eint: *mut mtk_eint,
}

extern "C" {
    pub fn mtk_pctrl_common_probe(pdev: *mut platform_device) -> c_int;
}
