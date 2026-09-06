//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/spi/mcp251xfd/mcp251xfd-dump.h
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
// Copyright (c) 2019, 2020, 2021 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//
pub const MCP251XFD_DUMP_MAGIC: c_uint = 0x1825434d;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcp251xfd_dump_object_type {
    MCP251XFD_DUMP_OBJECT_TYPE_REG,
    MCP251XFD_DUMP_OBJECT_TYPE_TEF,
    MCP251XFD_DUMP_OBJECT_TYPE_RX,
    MCP251XFD_DUMP_OBJECT_TYPE_TX,
    MCP251XFD_DUMP_OBJECT_TYPE_END = -1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcp251xfd_dump_object_ring_key {
    MCP251XFD_DUMP_OBJECT_RING_KEY_HEAD,
    MCP251XFD_DUMP_OBJECT_RING_KEY_TAIL,
    MCP251XFD_DUMP_OBJECT_RING_KEY_BASE,
    MCP251XFD_DUMP_OBJECT_RING_KEY_NR,
    MCP251XFD_DUMP_OBJECT_RING_KEY_FIFO_NR,
    MCP251XFD_DUMP_OBJECT_RING_KEY_OBJ_NUM,
    MCP251XFD_DUMP_OBJECT_RING_KEY_OBJ_SIZE,
    __MCP251XFD_DUMP_OBJECT_RING_KEY_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_dump_object_header {
    pub magic: __le32,
    pub type: __le32,
    pub offset: __le32,
    pub len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_dump_object_reg {
    pub reg: __le32,
    pub val: __le32,
}
