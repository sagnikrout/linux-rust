//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd72720.h
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
// Copyright 2025 ROHM Semiconductors.
//
// Author: Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>
//

// BD72720 interrupts

//
// The IRQs excluding GPIO1 and GPIO2 are ordered in a same way as the
// respective IRQ bits in status and mask registers are ordered.
//
// The BD72720_INT_GPIO1_IN and BD72720_INT_GPIO2_IN are IRQs which can
// be used by other devices. Let's have  GPIO1 and GPIO2 as first IRQs
// here so we can use the regmap-IRQ with standard device tree xlate
// while devices connected to the BD72720 IRQ input pins can refer to
// the first two interrupt numbers in their device tree. If we placed
// BD72720_INT_GPIO1_IN and BD72720_INT_GPIO2_IN after the CC_MON_DET
// interrupts (like they are in the registers), the devices using
// BD72720 as an IRQ parent should refer the interrupts starting with
// an offset which might not be trivial to understand.
//
// BD72720 Registers:
// The BD72720 has two sets of registers behind two different I2C slave
// addresses. "Common" registers being behind 0x4b, the charger registers
// being behind 0x4c.
//
// Registers behind I2C slave 0x4b
// Deep idle vsel
// Idle vsel
// Suspend vsel
// Run boot vsel
// Run0 ... run3 vsel
// Run vsel

pub const BD72720_GPIO_IRQ_TYPE_FALLING: c_uint = 0x0;
pub const BD72720_GPIO_IRQ_TYPE_RISING: c_uint = 0x1;
pub const BD72720_GPIO_IRQ_TYPE_BOTH: c_uint = 0x2;
pub const BD72720_GPIO_IRQ_TYPE_HIGH: c_uint = 0x3;
pub const BD72720_GPIO_IRQ_TYPE_LOW: c_uint = 0x4;

//
// The _STAT registers inform IRQ line state, and are used to ack IRQ.
// The _SRC registers below indicate current state of the function
// connected to the line.
//

// Register masks

pub const BD72720_I2C4C_ADDR_OFFSET: c_uint = 0x100;
// Registers behind I2C slave 0x4c

