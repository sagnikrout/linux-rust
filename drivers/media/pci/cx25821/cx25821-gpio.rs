//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cx25821/cx25821-gpio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
//

// GPIO stuffs
    void cx25821_set_gpiopin_direction(struct cx25821_dev *dev,
    int pin_number, int pin_logic_value)
    {
    let mut bit: c_int = pin_number;
    let mut gpio_oe_reg: u32 = GPIO_LO_OE;
    let mut gpio_register: u32 = 0;
    let mut value: u32 = 0;
// Check for valid pinNumber
    if (pin_number >= 47)
    return;
    if (pin_number > 31) {
    bit = pin_number - 31;
    gpio_oe_reg = GPIO_HI_OE;
    }
// Here we will make sure that the GPIOs 0 and 1 are output. keep the
// rest as is
    gpio_register = cx_read(gpio_oe_reg);
    if (pin_logic_value == 1)
    value = gpio_register | Set_GPIO_Bit(bit);
    else
    value = gpio_register & Clear_GPIO_Bit(bit);
    cx_write(gpio_oe_reg, value);
    }
    EXPORT_SYMBOL(cx25821_set_gpiopin_direction);
    static void cx25821_set_gpiopin_logicvalue(struct cx25821_dev *dev,
    int pin_number, int pin_logic_value)
    {
    let mut bit: c_int = pin_number;
    let mut gpio_reg: u32 = GPIO_LO;
    let mut value: u32 = 0;
// Check for valid pinNumber
    if (pin_number >= 47)
    return;
// change to output direction
    cx25821_set_gpiopin_direction(dev, pin_number, 0);
    if (pin_number > 31) {
    bit = pin_number - 31;
    gpio_reg = GPIO_HI;
    }
    value = cx_read(gpio_reg);
    if (pin_logic_value == 0)
    value &= Clear_GPIO_Bit(bit);
    else
    value |= Set_GPIO_Bit(bit);
    cx_write(gpio_reg, value);
    }
#[no_mangle]
pub unsafe extern "C" fn cx25821_gpio_init(dev: *mut cx25821_dev) {
    void cx25821_gpio_init(struct cx25821_dev *dev)
    {
    if (dev == core::ptr::null_mut())
    return;
    switch (dev.board) {
    case CX25821_BOARD_CONEXANT_ATHENA10:
    default:
// set GPIO 5 to select the path for Medusa/Athena
    cx25821_set_gpiopin_logicvalue(dev, 5, 1);
    msleep(20);
    break;
    }
    }
