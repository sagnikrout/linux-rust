//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/sunxi/pinctrl-sunxi.h
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


//
// Allwinner A1X SoCs pinctrl driver.
//
// Copyright (C) 2012 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const PA_BASE: c_int = 0;
pub const PB_BASE: c_int = 32;
pub const PC_BASE: c_int = 64;
pub const PD_BASE: c_int = 96;
pub const PE_BASE: c_int = 128;
pub const PF_BASE: c_int = 160;
pub const PG_BASE: c_int = 192;
pub const PH_BASE: c_int = 224;
pub const PI_BASE: c_int = 256;
pub const PJ_BASE: c_int = 288;
pub const PK_BASE: c_int = 320;
pub const PL_BASE: c_int = 352;
pub const PM_BASE: c_int = 384;
pub const PN_BASE: c_int = 416;
// maximum number of banks per controller (PA -> PK)
pub const SUNXI_PINCTRL_MAX_BANKS: c_int = 11;

pub const SUNXI_PIN_NAME_MAX_LEN: c_int = 5;
pub const BANK_MEM_SIZE: c_uint = 0x24;
pub const MUX_REGS_OFFSET: c_uint = 0x0;
pub const MUX_FIELD_WIDTH: c_int = 4;
pub const DATA_REGS_OFFSET: c_uint = 0x10;
pub const DATA_FIELD_WIDTH: c_int = 1;
pub const DLEVEL_REGS_OFFSET: c_uint = 0x14;
pub const DLEVEL_FIELD_WIDTH: c_int = 2;
pub const PULL_REGS_OFFSET: c_uint = 0x1c;
pub const PULL_FIELD_WIDTH: c_int = 2;
pub const D1_BANK_MEM_SIZE: c_uint = 0x30;
pub const D1_DLEVEL_FIELD_WIDTH: c_int = 4;
pub const D1_PULL_REGS_OFFSET: c_uint = 0x24;
pub const PINS_PER_BANK: c_int = 32;
pub const IRQ_PER_BANK: c_int = 32;
pub const IRQ_CFG_REG: c_uint = 0x200;
pub const IRQ_CFG_IRQ_PER_REG: c_int = 8;
pub const IRQ_CFG_IRQ_BITS: c_int = 4;

pub const IRQ_CTRL_REG: c_uint = 0x210;
pub const IRQ_CTRL_IRQ_PER_REG: c_int = 32;
pub const IRQ_CTRL_IRQ_BITS: c_int = 1;

pub const IRQ_STATUS_REG: c_uint = 0x214;
pub const IRQ_STATUS_IRQ_PER_REG: c_int = 32;
pub const IRQ_STATUS_IRQ_BITS: c_int = 1;

pub const IRQ_DEBOUNCE_REG: c_uint = 0x218;
pub const IRQ_MEM_SIZE: c_uint = 0x20;
pub const IRQ_EDGE_RISING: c_uint = 0x00;
pub const IRQ_EDGE_FALLING: c_uint = 0x01;
pub const IRQ_LEVEL_HIGH: c_uint = 0x02;
pub const IRQ_LEVEL_LOW: c_uint = 0x03;
pub const IRQ_EDGE_BOTH: c_uint = 0x04;
pub const GRP_CFG_REG: c_uint = 0x300;

pub const SUN4I_FUNC_INPUT: c_int = 0;
pub const SUN4I_FUNC_IRQ: c_int = 6;
pub const SUN4I_FUNC_DISABLED_OLD: c_int = 7;
pub const SUN4I_FUNC_DISABLED_NEW: c_int = 15;

pub const PIO_POW_MOD_SEL_REG: c_uint = 0x340;
pub const PIO_11B_POW_MOD_SEL_REG: c_uint = 0x380;
pub const PIO_POW_MOD_CTL_OFS: c_uint = 0x004;
pub const PIO_BANK_K_OFFSET: c_uint = 0x500;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sunxi_desc_bias_voltage {
    BIAS_VOLTAGE_NONE,
//
// Bias voltage configuration is done through
// Pn_GRP_CONFIG registers, as seen on A80 SoC.
//
    BIAS_VOLTAGE_GRP_CONFIG,
//
// Bias voltage is set through PIO_POW_MOD_SEL_REG
// register, as seen on H6 SoC, for example.
//
    BIAS_VOLTAGE_PIO_POW_MODE_SEL,
//
// Bias voltage is set through PIO_POW_MOD_SEL_REG
// and PIO_POW_MOD_CTL_REG register, as seen on
// A100 and D1 SoC, for example.
//
    BIAS_VOLTAGE_PIO_POW_MODE_CTL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_desc_function {
    pub variant: c_ulong,
    pub name: *const c_char,
    pub muxval: u8,
    pub irqbank: u8,
    pub irqnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_desc_pin {
    pub pin: pinctrl_pin_desc,
    pub variant: c_ulong,
    pub functions: *mut sunxi_desc_function,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pinctrl_desc {
    pub pins: *const sunxi_desc_pin,
    pub npins: c_int,
    pub pin_base: unsigned,
    pub irq_banks: unsigned,
    pub irq_bank_map: *const c_uint,
    pub irq_read_needs_mux: bool,
    pub disable_strict_mode: bool,
    pub io_bias_cfg_variant: sunxi_desc_bias_voltage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pinctrl_function {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub ngroups: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pinctrl_group {
    pub name: *const c_char,
    pub pin: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pinctrl_regulator {
    pub regulator: *mut regulator,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pinctrl {
    pub membase: *mut void __iomem,
    pub chip: *mut gpio_chip,
    pub desc: *const sunxi_pinctrl_desc,
    pub dev: *mut device,
    pub regulators: [sunxi_pinctrl_regulator; 11],
    pub domain: *mut irq_domain,
    pub functions: *mut sunxi_pinctrl_function,
    pub nfunctions: unsigned,
    pub groups: *mut sunxi_pinctrl_group,
    pub ngroups: unsigned,
    pub irq: *mut c_int,
    pub irq_array: *mut unsigned,
    pub lock: raw_spinlock_t,
    pub pctl_dev: *mut pinctrl_dev,
    pub flags: c_ulong,
    pub bank_mem_size: u32,
    pub pull_regs_offset: u32,
    pub dlevel_field_width: u32,
    pub pow_mod_sel_offset: u32,
}

extern "C" {
    pub fn sunxi_irq_ctrl_reg_from_bank(_arg: desc, _arg: bank) -> return;
}
extern "C" {
    pub fn sunxi_irq_status_reg_from_bank(_arg: desc, _arg: bank) -> return;
}

