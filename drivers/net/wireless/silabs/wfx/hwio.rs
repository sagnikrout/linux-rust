//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hwio.h
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
// Low-level I/O functions.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

// Caution: in the functions below, 'buf' will used with a DMA. So, it must be kmalloc'd (do not use
// stack allocated buffers). In doubt, enable CONFIG_DEBUG_SG to detect badly located buffer.
//
extern "C" {
    pub fn wfx_data_read(wdev: *mut wfx_dev, buf: *mut c_void, buf_len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_data_write(wdev: *mut wfx_dev, buf: *const c_void, buf_len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_sram_buf_read(wdev: *mut wfx_dev, addr: u32, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_sram_buf_write(wdev: *mut wfx_dev, addr: u32, buf: *const c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_ahb_buf_read(wdev: *mut wfx_dev, addr: u32, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_ahb_buf_write(wdev: *mut wfx_dev, addr: u32, buf: *const c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_sram_reg_read(wdev: *mut wfx_dev, addr: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn wfx_sram_reg_write(wdev: *mut wfx_dev, addr: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn wfx_ahb_reg_read(wdev: *mut wfx_dev, addr: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn wfx_ahb_reg_write(wdev: *mut wfx_dev, addr: u32, val: u32) -> c_int;
}
pub const CFG_ERR_SPI_FRAME: c_uint = 0x00000001 /* only with SPI */;
pub const CFG_ERR_SDIO_BUF_MISMATCH: c_uint = 0x00000001 /* only with SDIO */;
pub const CFG_ERR_BUF_UNDERRUN: c_uint = 0x00000002;
pub const CFG_ERR_DATA_IN_TOO_LARGE: c_uint = 0x00000004;
pub const CFG_ERR_HOST_NO_OUT_QUEUE: c_uint = 0x00000008;
pub const CFG_ERR_BUF_OVERRUN: c_uint = 0x00000010;
pub const CFG_ERR_DATA_OUT_TOO_LARGE: c_uint = 0x00000020;
pub const CFG_ERR_HOST_NO_IN_QUEUE: c_uint = 0x00000040;
pub const CFG_ERR_HOST_CRC_MISS: c_uint = 0x00000080 /* only with SDIO */;
pub const CFG_SPI_IGNORE_CS: c_uint = 0x00000080 /* only with SPI */;
pub const CFG_BYTE_ORDER_MASK: c_uint = 0x00000300 /* only writable with SPI */;
pub const CFG_BYTE_ORDER_BADC: c_uint = 0x00000000;
pub const CFG_BYTE_ORDER_DCBA: c_uint = 0x00000100;
pub const CFG_BYTE_ORDER_ABCD: c_uint = 0x00000200 /* SDIO always use this value */;
pub const CFG_DIRECT_ACCESS_MODE: c_uint = 0x00000400;
pub const CFG_PREFETCH_AHB: c_uint = 0x00000800;
pub const CFG_DISABLE_CPU_CLK: c_uint = 0x00001000;
pub const CFG_PREFETCH_SRAM: c_uint = 0x00002000;
pub const CFG_CPU_RESET: c_uint = 0x00004000;
pub const CFG_SDIO_DISABLE_IRQ: c_uint = 0x00008000 /* only with SDIO */;
pub const CFG_IRQ_ENABLE_DATA: c_uint = 0x00010000;
pub const CFG_IRQ_ENABLE_WRDY: c_uint = 0x00020000;
pub const CFG_CLK_RISE_EDGE: c_uint = 0x00040000;
pub const CFG_SDIO_DISABLE_CRC_CHK: c_uint = 0x00080000 /* only with SDIO */;
pub const CFG_RESERVED: c_uint = 0x00F00000;
pub const CFG_DEVICE_ID_MAJOR: c_uint = 0x07000000;
pub const CFG_DEVICE_ID_RESERVED: c_uint = 0x78000000;
pub const CFG_DEVICE_ID_TYPE: c_uint = 0x80000000;
extern "C" {
    pub fn wfx_config_reg_read(wdev: *mut wfx_dev, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn wfx_config_reg_write(wdev: *mut wfx_dev, val: u32) -> c_int;
}
extern "C" {
    pub fn wfx_config_reg_write_bits(wdev: *mut wfx_dev, mask: u32, val: u32) -> c_int;
}
pub const CTRL_NEXT_LEN_MASK: c_uint = 0x00000FFF;
pub const CTRL_WLAN_WAKEUP: c_uint = 0x00001000;
pub const CTRL_WLAN_READY: c_uint = 0x00002000;
extern "C" {
    pub fn wfx_control_reg_read(wdev: *mut wfx_dev, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn wfx_control_reg_write(wdev: *mut wfx_dev, val: u32) -> c_int;
}
extern "C" {
    pub fn wfx_control_reg_write_bits(wdev: *mut wfx_dev, mask: u32, val: u32) -> c_int;
}
pub const IGPR_RW: c_uint = 0x80000000;
pub const IGPR_INDEX: c_uint = 0x7F000000;
pub const IGPR_VALUE: c_uint = 0x00FFFFFF;
extern "C" {
    pub fn wfx_igpr_reg_read(wdev: *mut wfx_dev, index: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn wfx_igpr_reg_write(wdev: *mut wfx_dev, index: c_int, val: u32) -> c_int;
}
