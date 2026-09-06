//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/spi/mcp251xfd/mcp251xfd-ram.h
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
// mcp251xfd - Microchip MCP251xFD Family CAN controller driver
//
// Copyright (c) 2021, 2022 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_ram_mode {
    CAN_RAM_MODE_CAN,
    CAN_RAM_MODE_CANFD,
    __CAN_RAM_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_ram_obj_config {
    pub size: [u8; __CAN_RAM_MODE_MAX],
    pub def: [u8; __CAN_RAM_MODE_MAX],
    pub min: u8,
    pub max: u8,
    pub fifo_num: u8,
    pub fifo_depth_min: u8,
    pub fifo_depth_coalesce_min: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_ram_config {
    pub rx: can_ram_obj_config,
    pub tx: can_ram_obj_config,
    pub size: u16,
    pub fifo_depth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_ram_layout {
    pub default_rx: u8,
    pub default_tx: u8,
    pub max_rx: u8,
    pub max_tx: u8,
    pub cur_rx: u8,
    pub cur_tx: u8,
    pub rx_coalesce: u8,
    pub tx_coalesce: u8,
}
