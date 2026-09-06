//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/serial_mctrl_gpio.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Helpers for controlling modem lines via GPIO
//
// Copyright (C) 2014 Paratronic S.A.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mctrl_gpio_idx {
    UART_GPIO_CTS,
    UART_GPIO_DSR,
    UART_GPIO_DCD,
    UART_GPIO_RNG,
    UART_GPIO_RI = UART_GPIO_RNG,
    UART_GPIO_RTS,
    UART_GPIO_DTR,
    UART_GPIO_MAX,
}

//
// Opaque descriptor for modem lines controlled by GPIOs
//

//
// Set state of the modem control output lines via GPIOs.
//
extern "C" {
    pub fn mctrl_gpio_set(gpios: *mut mctrl_gpios, mctrl: c_uint);
}
//
// Get state of the modem control input lines from GPIOs.
// The mctrl flags are updated and returned.
//
extern "C" {
    pub fn mctrl_gpio_get(gpios: *mut mctrl_gpios, mctrl: *mut c_uint) -> c_uint;
}
//
// Get state of the modem control output lines from GPIOs.
// The mctrl flags are updated and returned.
//
// Returns the associated struct gpio_desc to the modem line gidx
//
// Request and set direction of modem control line GPIOs and set up irq
// handling.
// devm_* functions are used, so there's no need to explicitly free.
// Returns a pointer to the allocated mctrl structure if ok, -ENOMEM on
// allocation error.
//
// Request and set direction of modem control line GPIOs.
// devm_* functions are used, so there's no need to explicitly free.
// Returns a pointer to the allocated mctrl structure if ok, -ENOMEM on
// allocation error.
//
// Enable gpio interrupts to report status line changes.
//
extern "C" {
    pub fn mctrl_gpio_enable_ms(gpios: *mut mctrl_gpios);
}
//
// Disable gpio interrupts to report status line changes, and block until
// any corresponding IRQ is processed
//
extern "C" {
    pub fn mctrl_gpio_disable_ms_sync(gpios: *mut mctrl_gpios);
}
//
// Disable gpio interrupts to report status line changes, and return
// immediately
//
extern "C" {
    pub fn mctrl_gpio_disable_ms_no_sync(gpios: *mut mctrl_gpios);
}
//
// Enable gpio wakeup interrupts to enable wake up source.
//
extern "C" {
    pub fn mctrl_gpio_enable_irq_wake(gpios: *mut mctrl_gpios);
}
//
// Disable gpio wakeup interrupts to enable wake up source.
//
extern "C" {
    pub fn mctrl_gpio_disable_irq_wake(gpios: *mut mctrl_gpios);
}

