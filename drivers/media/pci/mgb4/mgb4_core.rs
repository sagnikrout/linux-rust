//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mgb4/mgb4_core.h
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
// Copyright (C) 2021-2023 Digiteq Automotive
// author: Martin Tuma <martin.tuma@digiteqautomotive.com>
//

pub const MGB4_HW_FREQ: c_int = 125000000;
pub const MGB4_VIN_DEVICES: c_int = 2;
pub const MGB4_VOUT_DEVICES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_dma_channel {
    pub chan: *mut dma_chan,
    pub req_compl: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_dev {
    pub pdev: *mut pci_dev,
    pub xdev: *mut platform_device,
    pub vin: [*mut mgb4_vin_dev; MGB4_VIN_DEVICES],
    pub vout: [*mut mgb4_vout_dev; MGB4_VOUT_DEVICES],
    pub c2h_chan: [mgb4_dma_channel; MGB4_VIN_DEVICES],
    pub h2c_chan: [mgb4_dma_channel; MGB4_VOUT_DEVICES],
    pub MGB4_VOUT_DEVICES]: dma_slave_map slave_map[MGB4_VIN_DEVICES +,
    pub video: mgb4_regs,
    pub cmt: mgb4_regs,
    pub i2c_clk: *mut clk_hw,
    pub i2c_cl: *mut clk_lookup,
    pub i2c_pdev: *mut platform_device,
    pub i2c_adap: *mut i2c_adapter,
    pub /: *mut *mut mutex i2c_lock; / I2C bus access lock,
    pub spi_pdev: *mut platform_device,
    pub flash_data: flash_platform_data,
    pub partitions: [mtd_partition; 2],
    pub flash_name: [c_char; 16],
    pub fw_part_name: [c_char; 16],
    pub data_part_name: [c_char; 16],
    pub MGB4_VOUT_DEVICES][16]: char channel_names[MGB4_VIN_DEVICES +,
    pub indio_dev: *mut iio_dev,

    pub hwmon_dev: *mut device,

    pub io_reconfig: c_ulong,
    pub module_version: u8,
    pub serial_number: u32,
    pub debugfs: *mut dentry,
}
