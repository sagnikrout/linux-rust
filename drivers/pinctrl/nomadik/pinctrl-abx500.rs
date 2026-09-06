//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/nomadik/pinctrl-abx500.h
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

// Macro flag: #define PINCTRL_PINCTRL_ABx500_H

// Package definitions
pub const PINCTRL_AB8500: c_int = 0;
pub const PINCTRL_AB8505: c_int = 1;
// pins alternate function
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum abx500_pin_func {
    ABX500_DEFAULT,
    ABX500_ALT_A,
    ABX500_ALT_B,
    ABX500_ALT_C,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum abx500_gpio_pull_updown {
    ABX500_GPIO_PULL_DOWN = 0x0,
    ABX500_GPIO_PULL_NONE = 0x1,
    ABX500_GPIO_PULL_UP = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum abx500_gpio_vinsel {
    ABX500_GPIO_VINSEL_VBAT = 0x0,
    ABX500_GPIO_VINSEL_VIN_1V8 = 0x1,
    ABX500_GPIO_VINSEL_VDD_BIF = 0x2,
}

//
// struct abx500_function - ABx500 pinctrl mux function
// @name: The name of the function, exported to pinctrl core.
// @groups: An array of pin groups that may select this function.
// @ngroups: The number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: unsigned,
}

//
// struct abx500_pingroup - describes a ABx500 pin group
// @name: the name of this specific pin group
// @pins: an array of discrete physical pins used in this group, taken
// from the driver-local pin enumeration space
// @num_pins: the number of pins in this group array, i.e. the number of
// elements in .pins so we can iterate over that array
// @altsetting: the altsetting to apply to all pins in this group to
// configure them to be used by a function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_pingroup {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub npins: unsigned,
    pub altsetting: c_int,
}

//
// struct alternate_functions
// @pin_number:		The pin number
// @gpiosel_bit:	Control bit in GPIOSEL register,
// @alt_bit1:		First AlternateFunction bit used to select the
// alternate function
// @alt_bit2:		Second AlternateFunction bit used to select the
// alternate function
//
// these 3 following fields are necessary due to none
// coherency on how to select the altA, altB and altC
// function between the ABx500 SOC family when using
// alternatfunc register.
// @alta_val:		value to write in alternatfunc to select altA function
// @altb_val:		value to write in alternatfunc to select altB function
// @altc_val:		value to write in alternatfunc to select altC function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alternate_functions {
    pub pin_number: unsigned,
    pub gpiosel_bit: i8,
    pub alt_bit1: i8,
    pub alt_bit2: i8,
    pub alta_val: u8,
    pub altb_val: u8,
    pub altc_val: u8,
}

//
// struct abx500_gpio_irq_cluster - indicates GPIOs which are interrupt
// capable
// @start:		The pin number of the first pin interrupt capable
// @end:		The pin number of the last pin interrupt capable
// @to_irq:		The ABx500 GPIO's associated IRQs are clustered
// together throughout the interrupt numbers at irregular
// intervals. To solve this quandary, we will place the
// read-in values into the cluster information table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_gpio_irq_cluster {
    pub start: c_int,
    pub end: c_int,
    pub to_irq: c_int,
}

//
// struct abx500_pinrange - map pin numbers to GPIO offsets
// @offset:		offset into the GPIO local numberspace, incidentally
// identical to the offset into the local pin numberspace
// @npins:		number of pins to map from both offsets
// @altfunc:		altfunc setting to be used to enable GPIO on a pin in
// this range (may vary)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_pinrange {
    pub offset: c_uint,
    pub npins: c_uint,
    pub altfunc: c_int,
}

//
// struct abx500_pinctrl_soc_data - ABx500 pin controller per-SoC configuration
// @gpio_ranges:	An array of GPIO ranges for this SoC
// @gpio_num_ranges:	The number of GPIO ranges for this SoC
// @pins:		An array describing all pins the pin controller affects.
// All pins which are also GPIOs must be listed first within the
// array, and be numbered identically to the GPIO controller's
// numbering.
// @npins:		The number of entries in @pins.
// @functions:		The functions supported on this SoC.
// @nfunction:		The number of entries in @functions.
// @groups:		An array describing all pin groups the pin SoC supports.
// @ngroups:		The number of entries in @groups.
// @alternate_functions: array describing pins which supports alternate and
// how to set it.
// @gpio_irq_cluster:	An array of GPIO interrupt capable for this SoC
// @ngpio_irq_cluster:	The number of GPIO inetrrupt capable for this SoC
// @irq_gpio_rising_offset: Interrupt offset used as base to compute specific
// setting strategy of the rising interrupt line
// @irq_gpio_falling_offset: Interrupt offset used as base to compute specific
// setting strategy of the falling interrupt line
// @irq_gpio_factor:	Factor used to compute specific setting strategy of
// the interrupt line
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_pinctrl_soc_data {
    pub gpio_ranges: *const abx500_pinrange,
    pub gpio_num_ranges: unsigned,
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub functions: *const abx500_function,
    pub nfunctions: unsigned,
    pub groups: *const abx500_pingroup,
    pub ngroups: unsigned,
    pub alternate_functions: *mut alternate_functions,
    pub gpio_irq_cluster: *mut abx500_gpio_irq_cluster,
    pub ngpio_irq_cluster: unsigned,
    pub irq_gpio_rising_offset: c_int,
    pub irq_gpio_falling_offset: c_int,
    pub irq_gpio_factor: c_int,
}

extern "C" {
    pub fn abx500_pinctrl_ab8500_init(soc: *mut abx500_pinctrl_soc_data);
}

extern "C" {
    pub fn abx500_pinctrl_ab8505_init(soc: *mut abx500_pinctrl_soc_data);
}

