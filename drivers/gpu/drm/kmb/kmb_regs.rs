//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/kmb/kmb_regs.h
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
//
// Copyright © 2018-2020 Intel Corporation
//
// LCD controller control register defines
//

// interrupts

// LCD_VSTATUS_COMPARE Vertcal interval in which to generate vertcal
// interval interrupt
//
// BITS 13 and 14

//
// LCD controller Layer config register
//

// RGB multiplied with alpha

// LCD controller Layer DMA config register

//
// LCD controller output format register defines
//

//
// MIPI controller control register defines
//

// 1:CSI 0:DSI

// 1:LCD, 0:DMA

// MIPI IRQ

pub const MIPI_DPHY_ERR_IRQ: c_int = 1;
pub const MIPI_DPHY_ERR_MASK: c_uint = 0x7FE	/*bits 1-10 */;
pub const MIPI_HS_IRQ: c_int = 13;
// bits 13-22
pub const MIPI_HS_IRQ_MASK: c_uint = 0x7FE000;
pub const MIPI_LP_EVENT_IRQ: c_int = 25;

pub const MIPI_HS_RX_EVENT_IRQ: c_int = 0;

// MIPI Test Pattern Generation

// D-PHY regs

pub const SHUTDOWNZ: c_int = 0;
pub const RESETZ: c_int = 12;

pub const PLL_CLKSEL_0: c_int = 18;
pub const PLL_SHADOW_CTRL: c_int = 16;

// icam lcd qos

