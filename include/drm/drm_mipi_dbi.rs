//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_mipi_dbi.h
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
// MIPI Display Bus Interface (DBI) LCD controller support
//
// Copyright 2016 Noralf Trønnes
//

//
// struct mipi_dbi - MIPI DBI interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dbi {
//
// @cmdlock: Command lock
//
    pub cmdlock: mutex,
//
// @command: Bus specific callback executing commands.
//
    pub num): *mut *mut *mut *mut *mut int (command)(struct mipi_dbi dbi, u8 cmd, u8 param, size_t,
//
// @read_commands: Array of read commands terminated by a zero entry.
// Reading is disabled if this is NULL.
//
    pub read_commands: *const u8,
//
// @swap_bytes: Swap bytes in buffer before transfer
//
    pub swap_bytes: bool,
//
// @reset: Optional reset gpio
//
    pub reset: *mut gpio_desc,
// Type C specific
//
// @spi: SPI device
//
    pub spi: *mut spi_device,
//
// @write_memory_bpw: Bits per word used on a MIPI_DCS_WRITE_MEMORY_START transfer
//
    pub write_memory_bpw: c_uint,
//
// @dc: Optional D/C gpio.
//
    pub dc: *mut gpio_desc,
//
// @tx_buf9: Buffer used for Option 1 9-bit conversion
//
    pub tx_buf9: *mut c_void,
//
// @tx_buf9_len: Size of tx_buf9.
//
    pub tx_buf9_len: usize,
}

//
// struct mipi_dbi_dev - MIPI DBI device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dbi_dev {
//
// @drm: DRM device
//
    pub drm: drm_device,
//
// @mode: Fixed display mode
//
    pub mode: drm_display_mode,
//
// @pixel_format: Native pixel format (DRM_FORMAT\_\*)
//
    pub pixel_format: u32,
//
// @tx_buf: Buffer used for transfer (copy clip rect area)
//
    pub tx_buf: *mut u16,
//
// @rotation: initial rotation in degrees Counter Clock Wise
//
    pub rotation: c_uint,
//
// @left_offset: Horizontal offset of the display relative to the
// controller's driver array
//
    pub left_offset: c_uint,
//
// @top_offset: Vertical offset of the display relative to the
// controller's driver array
//
    pub top_offset: c_uint,
//
// @backlight: backlight device (optional)
//
    pub backlight: *mut backlight_device,
//
// @regulator: power regulator (Vdd) (optional)
//
    pub regulator: *mut regulator,
//
// @io_regulator: I/O power regulator (Vddi) (optional)
//
    pub io_regulator: *mut regulator,
//
// @dbi: MIPI DBI interface
//
    pub dbi: mipi_dbi,
//
// @driver_private: Driver private data.
// Necessary for drivers with private data since devm_drm_dev_alloc()
// can't allocate structures that embed a structure which then again
// embeds drm_device.
//
    pub driver_private: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: drm, mipi_dbi_dev: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn mipi_dbi_hw_reset(dbi: *mut mipi_dbi);
}
extern "C" {
    pub fn mipi_dbi_display_is_on(dbi: *mut mipi_dbi) -> bool;
}
extern "C" {
    pub fn mipi_dbi_poweron_reset(dbidev: *mut mipi_dbi_dev) -> c_int;
}
extern "C" {
    pub fn mipi_dbi_poweron_conditional_reset(dbidev: *mut mipi_dbi_dev) -> c_int;
}
extern "C" {
    pub fn mipi_dbi_spi_cmd_max_speed(spi: *mut spi_device, len: usize) -> u32;
}
extern "C" {
    pub fn mipi_dbi_command_read(dbi: *mut mipi_dbi, cmd: u8, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn mipi_dbi_command_buf(dbi: *mut mipi_dbi, cmd: u8, data: *mut u8, len: usize) -> c_int;
}
//
// mipi_dbi_command - MIPI DCS command with optional parameter(s)
// @dbi: MIPI DBI structure
// @cmd: Command
// @seq: Optional parameter(s)
//
// Send MIPI DCS command to the controller. Use mipi_dbi_command_read() for
// get/read.
//
// Returns:
// Zero on success, negative error code on failure.
//

//
// Plane
//

//
// CRTC
//

//
// Connector
//

extern "C" {
    pub fn drm_mipi_dbi_connector_helper_get_modes(connector: *mut drm_connector) -> c_int;
}

//
// Mode config
//

//
// Debug FS
//

extern "C" {
    pub fn mipi_dbi_debugfs_init(minor: *mut drm_minor);
}

