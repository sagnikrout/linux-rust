//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/spear/pinctrl-spear.h
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
// Driver header file for the ST Microelectronics SPEAr pinmux
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// struct spear_pmx_mode - SPEAr pmx mode
// @name: name of pmx mode
// @mode: mode id
// @reg: register for configuring this mode
// @mask: mask of this mode in reg
// @val: val to be configured at reg after doing (val & mask)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_pmx_mode {
    pub name: *const *const c_char,
    pub mode: u16,
    pub reg: u16,
    pub mask: u16,
    pub val: u32,
}

//
// struct spear_muxreg - SPEAr mux reg configuration
// @reg: register offset
// @mask: mask bits
// @val: val to be written on mask bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_muxreg {
    pub reg: u16,
    pub mask: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_gpio_pingroup {
    pub pins: *const unsigned,
    pub npins: unsigned,
    pub muxregs: *mut spear_muxreg,
    pub nmuxregs: u8,
}

// ste: set to enable

//
// struct spear_modemux - SPEAr mode mux configuration
// @modes: mode ids supported by this group of muxregs
// @nmuxregs: number of muxreg configurations to be done for modes
// @muxregs: array of muxreg configurations to be done for modes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_modemux {
    pub modes: u16,
    pub nmuxregs: u8,
    pub muxregs: *mut spear_muxreg,
}

//
// struct spear_pingroup - SPEAr pin group configurations
// @name: name of pin group
// @pins: array containing pin numbers
// @npins: size of pins array
// @modemuxs: array of modemux configurations for this pin group
// @nmodemuxs: size of array modemuxs
//
// A representation of a group of pins in the SPEAr pin controller. Each group
// allows some parameter or parameters to be configured.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_pingroup {
    pub name: *const c_char,
    pub pins: *const unsigned,
    pub npins: unsigned,
    pub modemuxs: *mut spear_modemux,
    pub nmodemuxs: unsigned,
}

//
// struct spear_function - SPEAr pinctrl mux function
// @name: The name of the function, exported to pinctrl core.
// @groups: An array of pin groups that may select this function.
// @ngroups: The number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: unsigned,
}

//
// struct spear_pinctrl_machdata - SPEAr pin controller machine driver
// configuration
// @pins: An array describing all pins the pin controller affects.
// All pins which are also GPIOs must be listed first within the *array,
// and be numbered identically to the GPIO controller's *numbering.
// @npins: The numbmer of entries in @pins.
// @functions: An array describing all mux functions the SoC supports.
// @nfunctions: The numbmer of entries in @functions.
// @groups: An array describing all pin groups the pin SoC supports.
// @ngroups: The numbmer of entries in @groups.
// @gpio_pingroups: gpio pingroups
// @ngpio_pingroups: gpio pingroups count
//
// @modes_supported: Does SoC support modes
// @mode: mode configured from probe
// @pmx_modes: array of modes supported by SoC
// @npmx_modes: number of entries in pmx_modes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_pinctrl_machdata {
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub functions: *mut spear_function,
    pub nfunctions: unsigned,
    pub groups: *mut spear_pingroup,
    pub ngroups: unsigned,
    pub gpio_pingroups: *mut spear_gpio_pingroup,
    pub enable): bool,
    pub ngpio_pingroups: unsigned,
    pub modes_supported: bool,
    pub mode: u16,
    pub pmx_modes: *mut spear_pmx_mode,
    pub npmx_modes: unsigned,
}

//
// struct spear_pmx - SPEAr pinctrl mux
// @dev: pointer to struct dev of platform_device registered
// @pctl: pointer to struct pinctrl_dev
// @machdata: pointer to SoC or machine specific structure
// @regmap: regmap of pinmux controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_pmx {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub machdata: *mut spear_pinctrl_machdata,
    pub regmap: *mut regmap,
}

// exported routines
extern "C" {
    pub fn pmx_init_addr(machdata: *mut spear_pinctrl_machdata, reg: u16);
}

