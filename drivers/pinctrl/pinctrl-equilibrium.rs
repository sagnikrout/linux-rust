//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-equilibrium.h
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
// Copyright(c) 2019 Intel Corporation.
//
// PINPAD register offset
pub const REG_PMX_BASE: c_uint = 0x0	/* Port Multiplexer Control Register */;
pub const REG_PUEN: c_uint = 0x80	/* PULL UP Enable Register */;
pub const REG_PDEN: c_uint = 0x84	/* PULL DOWN Enable Register */;
pub const REG_SRC: c_uint = 0x88	/* Slew Rate Control Register */;
pub const REG_DCC0: c_uint = 0x8C	/* Drive Current Control Register 0 */;
pub const REG_DCC1: c_uint = 0x90	/* Drive Current Control Register 1 */;
pub const REG_OD: c_uint = 0x94	/* Open Drain Enable Register */;
pub const REG_AVAIL: c_uint = 0x98	/* Pad Control Availability Register */;

// GPIO register offset
pub const GPIO_OUT: c_uint = 0x0	/* Data Output Register */;
pub const GPIO_IN: c_uint = 0x4	/* Data Input Register */;
pub const GPIO_DIR: c_uint = 0x8	/* Direction Register */;
pub const GPIO_EXINTCR0: c_uint = 0x18	/* External Interrupt Control Register 0 */;
pub const GPIO_EXINTCR1: c_uint = 0x1C	/* External Interrupt Control Register 1 */;
pub const GPIO_IRNCR: c_uint = 0x20	/* IRN Capture Register */;
pub const GPIO_IRNICR: c_uint = 0x24	/* IRN Interrupt Control Register */;
pub const GPIO_IRNEN: c_uint = 0x28	/* IRN Interrupt Enable Register */;
pub const GPIO_IRNCFG: c_uint = 0x2C	/* IRN Interrupt Configuration Register */;
pub const GPIO_IRNRNSET: c_uint = 0x30	/* IRN Interrupt Enable Set Register */;
pub const GPIO_IRNENCLR: c_uint = 0x34	/* IRN Interrupt Enable Clear Register */;
pub const GPIO_OUTSET: c_uint = 0x40	/* Output Set Register */;
pub const GPIO_OUTCLR: c_uint = 0x44	/* Output Clear Register */;
pub const GPIO_DIRSET: c_uint = 0x48	/* Direction Set Register */;
pub const GPIO_DIRCLR: c_uint = 0x4C	/* Direction Clear Register */;
// parse given pin's driver current value

pub const GPIO_EDGE_TRIG: c_int = 0;
pub const GPIO_LEVEL_TRIG: c_int = 1;
pub const GPIO_SINGLE_EDGE: c_int = 0;
pub const GPIO_BOTH_EDGE: c_int = 1;
pub const GPIO_POSITIVE_TRIG: c_int = 0;
pub const GPIO_NEGATIVE_TRIG: c_int = 1;
pub const EQBR_GPIO_MODE: c_int = 0;
//
// struct gpio_irq_type: gpio irq configuration
// @trig_type: level trigger or edge trigger
// @edge_type: sigle edge or both edge
// @logic_type: positive trigger or negative trigger
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_irq_type {
    pub trig_type: c_uint,
    pub edge_type: c_uint,
    pub logic_type: c_uint,
}

//
// struct eqbr_pin_bank: represent a pin bank.
// @membase: base address of the pin bank register.
// @id: bank id, to idenify the unique bank.
// @pin_base: starting pin number of the pin bank.
// @nr_pins: number of the pins of the pin bank.
// @aval_pinmap: available pin bitmap of the pin bank.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eqbr_pin_bank {
    pub membase: *mut void __iomem,
    pub id: c_uint,
    pub pin_base: c_uint,
    pub nr_pins: c_uint,
    pub aval_pinmap: u32,
}

//
// struct eqbr_gpio_ctrl: represent a gpio controller.
// @chip: gpio chip.
// @fwnode: firmware node of gpio controller.
// @bank: pointer to corresponding pin bank.
// @membase: base address of the gpio controller.
// @name: gpio chip name.
// @virq: irq number of the gpio chip to parent's irq domain.
// @lock: spin lock to protect gpio register write.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eqbr_gpio_ctrl {
    pub chip: gpio_generic_chip,
    pub fwnode: *mut fwnode_handle,
    pub bank: *mut eqbr_pin_bank,
    pub membase: *mut void __iomem,
    pub name: *const c_char,
    pub virq: c_uint,
    pub /: *mut *mut raw_spinlock_t lock; / protect gpio register,
}

//
// struct eqbr_pinctrl_drv_data:
// @dev: device instance representing the controller.
// @pctl_desc: pin controller descriptor.
// @pctl_dev: pin control class device
// @membase: base address of pin controller
// @pin_banks: list of pin banks of the driver.
// @nr_banks: number of pin banks.
// @gpio_ctrls: list of gpio controllers.
// @nr_gpio_ctrls: number of gpio controllers.
// @lock: protect pinctrl register write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eqbr_pinctrl_drv_data {
    pub dev: *mut device,
    pub pctl_desc: pinctrl_desc,
    pub pctl_dev: *mut pinctrl_dev,
    pub membase: *mut void __iomem,
    pub pin_banks: *mut eqbr_pin_bank,
    pub nr_banks: c_uint,
    pub gpio_ctrls: *mut eqbr_gpio_ctrl,
    pub nr_gpio_ctrls: c_uint,
    pub /: *mut *mut raw_spinlock_t lock; / protect pinpad register,
}
