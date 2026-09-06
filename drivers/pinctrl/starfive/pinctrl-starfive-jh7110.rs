//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/starfive/pinctrl-starfive-jh7110.h
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
// Pinctrl / GPIO driver for StarFive JH7110 SoC
//
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pinctrl {
    pub dev: *mut device,
    pub gc: gpio_chip,
    pub gpios: pinctrl_gpio_range,
    pub lock: raw_spinlock_t,
    pub base: *mut void __iomem,
    pub pctl: *mut pinctrl_dev,
// register read/write mutex
    pub mutex: mutex,
    pub info: *const jh7110_pinctrl_soc_info,
    pub num_saved_regs: c_uint,
    pub __counted_by(num_saved_regs): u32 saved_regs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_gpio_irq_reg {
    pub is_reg_base: c_uint,
    pub ic_reg_base: c_uint,
    pub ibe_reg_base: c_uint,
    pub iev_reg_base: c_uint,
    pub ie_reg_base: c_uint,
    pub ris_reg_base: c_uint,
    pub mis_reg_base: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pinctrl_soc_info {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub ngpios: c_uint,
// gpio dout/doen/din/gpioinput register
    pub dout_reg_base: c_uint,
    pub dout_mask: c_uint,
    pub doen_reg_base: c_uint,
    pub doen_mask: c_uint,
    pub gpi_reg_base: c_uint,
    pub gpi_mask: c_uint,
    pub gpioin_reg_base: c_uint,
    pub irq_reg: *const jh7110_gpio_irq_reg,
    pub nsaved_regs: c_uint,
// generic pinmux
    pub func): u32 doen, u32,
// gpio chip
    pub pin): c_uint,
    pub desc): *mut *mut void (jh7110_gpio_irq_handler)(struct irq_desc,
    pub gc): *mut *mut int (jh7110_gpio_init_hw)(struct gpio_chip,
}

extern "C" {
    pub fn jh7110_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
