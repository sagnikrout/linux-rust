//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/m_can/tcan4x5x.h
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
// tcan4x5x - Texas Instruments TCAN4x5x Family CAN controller driver
//
// Copyright (c) 2020 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

pub const TCAN4X5X_SANITIZE_SPI: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcan4x5x_map_buf {
    pub cmd: tcan4x5x_buf_cmd,
    pub sizeof(u32)]: *mut *mut u8 data[256,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcan4x5x_priv {
    pub cdev: m_can_classdev,
    pub regmap: *mut regmap,
    pub spi: *mut spi_device,
    pub reset_gpio: *mut gpio_desc,
    pub device_wake_gpio: *mut gpio_desc,
    pub device_state_gpio: *mut gpio_desc,
    pub power: *mut regulator,
    pub map_buf_rx: tcan4x5x_map_buf,
    pub map_buf_tx: tcan4x5x_map_buf,
    pub nwkrq_voltage_vio: bool,
}

// number of u32
extern "C" {
    pub fn tcan4x5x_regmap_init(priv: *mut tcan4x5x_priv) -> c_int;
}
