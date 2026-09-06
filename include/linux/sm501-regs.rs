//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sm501-regs.h
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
// sm501-regs.h
//
// Copyright 2006 Simtec Electronics
//
// Silicon Motion SM501 register definitions
//
// System Configuration area
// System config base

// config 1

// miscellaneous control

// command list

// command list

// interrupt debug

// power management

// power gates for units within the 501

// panel clock

// crt clock

// main clock

// SDRAM controller clock

// config 2

// 0x050100A0

// GPIO base

// I2C controller base

// SSP base

// Uart 0 base

// Uart 1 base

// USB host port base

// USB slave/gadget base

// USB slave/gadget data port base

// Display controller/video engine base

// common defines for the SM501 address registers

// common registers for panel and the crt

// Zoom Video port base

// AC97/I2S base

// 8051 micro controller base

// 8051 micro controller SRAM base

// DMA base

// 2d engine base

// 2d engine data port base
