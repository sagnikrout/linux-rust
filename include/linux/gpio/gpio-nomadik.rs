//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/gpio-nomadik.h
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
// Package definitions
pub const PINCTRL_NMK_STN8815: c_int = 0;
pub const PINCTRL_NMK_DB8500: c_int = 1;
pub const GPIO_BLOCK_SHIFT: c_int = 5;

// Register in the logic block
pub const NMK_GPIO_DAT: c_uint = 0x00;
pub const NMK_GPIO_DATS: c_uint = 0x04;
pub const NMK_GPIO_DATC: c_uint = 0x08;
pub const NMK_GPIO_PDIS: c_uint = 0x0c;
pub const NMK_GPIO_DIR: c_uint = 0x10;
pub const NMK_GPIO_DIRS: c_uint = 0x14;
pub const NMK_GPIO_DIRC: c_uint = 0x18;
pub const NMK_GPIO_SLPC: c_uint = 0x1c;
pub const NMK_GPIO_AFSLA: c_uint = 0x20;
pub const NMK_GPIO_AFSLB: c_uint = 0x24;
pub const NMK_GPIO_LOWEMI: c_uint = 0x28;
pub const NMK_GPIO_RIMSC: c_uint = 0x40;
pub const NMK_GPIO_FIMSC: c_uint = 0x44;
pub const NMK_GPIO_IS: c_uint = 0x48;
pub const NMK_GPIO_IC: c_uint = 0x4c;
pub const NMK_GPIO_RWIMSC: c_uint = 0x50;
pub const NMK_GPIO_FWIMSC: c_uint = 0x54;
pub const NMK_GPIO_WKS: c_uint = 0x58;
// Pull up/down values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nmk_gpio_pull {
    NMK_GPIO_PULL_NONE,
    NMK_GPIO_PULL_UP,
    NMK_GPIO_PULL_DOWN,
}

// Sleep mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nmk_gpio_slpm {
    NMK_GPIO_SLPM_INPUT,
    NMK_GPIO_SLPM_WAKEUP_ENABLE = NMK_GPIO_SLPM_INPUT,
    NMK_GPIO_SLPM_NOCHANGE,
    NMK_GPIO_SLPM_WAKEUP_DISABLE = NMK_GPIO_SLPM_NOCHANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_gpio_chip {
    pub chip: gpio_chip,
    pub addr: *mut void __iomem,
    pub clk: *mut clk,
    pub bank: c_uint,
    pub enable): *mut *mut void (set_ioforce)(bool,
    pub lock: spinlock_t,
    pub sleepmode: bool,
    pub is_mobileye_soc: bool,
// Keep track of configured edges
    pub edge_rising: u32,
    pub edge_falling: u32,
    pub real_wake: u32,
    pub rwimsc: u32,
    pub fwimsc: u32,
    pub rimsc: u32,
    pub fimsc: u32,
    pub pull_up: u32,
    pub lowemi: u32,
}

// Alternate functions: function C is set in hw by setting both A and B
pub const NMK_GPIO_ALT_GPIO: c_int = 0;
pub const NMK_GPIO_ALT_A: c_int = 1;
pub const NMK_GPIO_ALT_B: c_int = 2;

pub const NMK_GPIO_ALT_CX_SHIFT: c_int = 2;

// Macro flag: #define PRCM_GPIOCR_ALTCX(pin_num,\
//
// enum prcm_gpiocr_reg_index - Used to reference a PRCM GPIOCR register address.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcm_gpiocr_reg_index {
    PRCM_IDX_GPIOCR1,
    PRCM_IDX_GPIOCR2,
    PRCM_IDX_GPIOCR3
}

//
// enum prcm_gpiocr_altcx_index - Used to reference an Other alternate-C function.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prcm_gpiocr_altcx_index {
    PRCM_IDX_GPIOCR_ALTC1,
    PRCM_IDX_GPIOCR_ALTC2,
    PRCM_IDX_GPIOCR_ALTC3,
    PRCM_IDX_GPIOCR_ALTC4,
    PRCM_IDX_GPIOCR_ALTC_MAX,
}

//
// struct prcm_gpiocr_altcx - Other alternate-C function
// @used: other alternate-C function availability
// @reg_index: PRCM GPIOCR register index used to control the function
// @control_bit: PRCM GPIOCR bit used to control the function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prcm_gpiocr_altcx {
    pub used:1: bool,
    pub reg_index:2: u8,
    pub control_bit:5: u8,
    pub __packed: },
//
// struct prcm_gpiocr_altcx_pin_desc - Other alternate-C pin
// @pin: The pin number
// @altcx: array of other alternate-C[1-4] functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prcm_gpiocr_altcx_pin_desc {
    pub pin: c_ushort,
    pub altcx: [prcm_gpiocr_altcx; PRCM_IDX_GPIOCR_ALTC_MAX],
}

//
// struct nmk_function - Nomadik pinctrl mux function
// @name: The name of the function, exported to pinctrl core.
// @groups: An array of pin groups that may select this function.
// @ngroups: The number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: c_uint,
}

//
// struct nmk_pingroup - describes a Nomadik pin group
// @grp: Generic data of the pin group (name and pins)
// @altsetting: the altsetting to apply to all pins in this group to
// configure them to be used by a function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_pingroup {
    pub grp: pingroup,
    pub altsetting: c_int,
}

//
// struct nmk_pinctrl_soc_data - Nomadik pin controller per-SoC configuration
// @pins:	An array describing all pins the pin controller affects.
// All pins which are also GPIOs must be listed first within the
// array, and be numbered identically to the GPIO controller's
// numbering.
// @npins:	The number of entries in @pins.
// @functions:	The functions supported on this SoC.
// @nfunctions:	The number of entries in @functions.
// @groups:	An array describing all pin groups the pin SoC supports.
// @ngroups:	The number of entries in @groups.
// @altcx_pins:	The pins that support Other alternate-C function on this SoC
// @npins_altcx: The number of Other alternate-C pins
// @prcm_gpiocr_registers: The array of PRCM GPIOCR registers on this SoC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_pinctrl_soc_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub functions: *const nmk_function,
    pub nfunctions: c_uint,
    pub groups: *const nmk_pingroup,
    pub ngroups: c_uint,
    pub altcx_pins: *const prcm_gpiocr_altcx_pin_desc,
    pub npins_altcx: c_uint,
    pub prcm_gpiocr_registers: *const u16,
}

extern "C" {
    pub fn nmk_pinctrl_stn8815_init(soc: *const nmk_pinctrl_soc_data);
}

extern "C" {
    pub fn nmk_pinctrl_db8500_init(soc: *const nmk_pinctrl_soc_data);
}

//
// Symbols declared in gpio-nomadik used by pinctrl-nomadik. If pinctrl-nomadik
// is enabled, then gpio-nomadik is enabled as well; the reverse if not always
// true.
//

// Symbols declared in pinctrl-nomadik used by gpio-nomadik.

