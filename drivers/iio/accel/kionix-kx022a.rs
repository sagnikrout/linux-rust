//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/kionix-kx022a.h
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
// Copyright (C) 2022 ROHM Semiconductors
//
// ROHM/KIONIX KX022A accelerometer driver
//

pub const KX022A_REG_WHO: c_uint = 0x0f;
pub const KX022A_ID: c_uint = 0xc8;
pub const KX132ACR_LBZ_ID: c_uint = 0xd8;
pub const KX134ACR_LBZ_ID: c_uint = 0xcc;
pub const KX022A_REG_CNTL2: c_uint = 0x19;

pub const KX022A_REG_CNTL: c_uint = 0x18;

pub const KX022A_GSEL_SHIFT: c_int = 3;
pub const KX022A_GSEL_2: c_uint = 0x0;

pub const KX022A_REG_INS2: c_uint = 0x13;

pub const KX022A_REG_XHP_L: c_uint = 0x0;
pub const KX022A_REG_XOUT_L: c_uint = 0x06;
pub const KX022A_REG_YOUT_L: c_uint = 0x08;
pub const KX022A_REG_ZOUT_L: c_uint = 0x0a;
pub const KX022A_REG_COTR: c_uint = 0x0c;
pub const KX022A_REG_TSCP: c_uint = 0x10;
pub const KX022A_REG_INT_REL: c_uint = 0x17;
pub const KX022A_REG_ODCNTL: c_uint = 0x1b;
pub const KX022A_REG_BTS_WUF_TH: c_uint = 0x31;
pub const KX022A_REG_MAN_WAKE: c_uint = 0x2c;
pub const KX022A_REG_BUF_CNTL1: c_uint = 0x3a;

pub const KX022A_REG_BUF_CNTL2: c_uint = 0x3b;

pub const KX022A_REG_BUF_STATUS_1: c_uint = 0x3c;
pub const KX022A_REG_BUF_STATUS_2: c_uint = 0x3d;
pub const KX022A_REG_BUF_CLEAR: c_uint = 0x3e;
pub const KX022A_REG_BUF_READ: c_uint = 0x3f;

pub const KX022A_ODR_SHIFT: c_int = 3;
pub const KX022A_FIFO_MAX_WMI_TH: c_int = 41;
pub const KX022A_REG_INC1: c_uint = 0x1c;
pub const KX022A_REG_INC5: c_uint = 0x20;
pub const KX022A_REG_INC6: c_uint = 0x21;

pub const KX022A_IPOL_LOW: c_int = 0;

pub const KX022A_ITYP_LEVEL: c_int = 0;
pub const KX022A_REG_INC4: c_uint = 0x1f;

pub const KX022A_REG_SELF_TEST: c_uint = 0x60;
pub const KX022A_MAX_REGISTER: c_uint = 0x60;
pub const KX132_REG_WHO: c_uint = 0x13;
pub const KX132_ID: c_uint = 0x3d;
pub const KX134_1211_ID: c_uint = 0x46;
pub const KX132_FIFO_LENGTH: c_int = 86;
pub const KX132_REG_CNTL: c_uint = 0x1b;
pub const KX132_REG_CNTL2: c_uint = 0x1c;
pub const KX132_REG_CNTL5: c_uint = 0x1f;

pub const KX132_GSEL_2: c_uint = 0x0;

pub const KX132_REG_INS2: c_uint = 0x17;

pub const KX132_REG_XADP_L: c_uint = 0x02;
pub const KX132_REG_XOUT_L: c_uint = 0x08;
pub const KX132_REG_YOUT_L: c_uint = 0x0a;
pub const KX132_REG_ZOUT_L: c_uint = 0x0c;
pub const KX132_REG_COTR: c_uint = 0x12;
pub const KX132_REG_TSCP: c_uint = 0x14;
pub const KX132_REG_INT_REL: c_uint = 0x1a;
pub const KX132_REG_ODCNTL: c_uint = 0x21;
pub const KX132_REG_BTS_WUF_TH: c_uint = 0x4a;
pub const KX132_REG_BUF_CNTL1: c_uint = 0x5e;
pub const KX132_REG_BUF_CNTL2: c_uint = 0x5f;
pub const KX132_REG_BUF_STATUS_1: c_uint = 0x60;
pub const KX132_REG_BUF_STATUS_2: c_uint = 0x61;

pub const KX132_REG_BUF_CLEAR: c_uint = 0x62;
pub const KX132_REG_BUF_READ: c_uint = 0x63;
pub const KX132_ODR_SHIFT: c_int = 3;
pub const KX132_FIFO_MAX_WMI_TH: c_int = 86;
pub const KX132_REG_INC1: c_uint = 0x22;
pub const KX132_REG_INC5: c_uint = 0x26;
pub const KX132_REG_INC6: c_uint = 0x27;
pub const KX132_IPOL_LOW: c_int = 0;

pub const KX132_REG_INC4: c_uint = 0x25;
pub const KX132_REG_SELF_TEST: c_uint = 0x5d;
pub const KX132_MAX_REGISTER: c_uint = 0x76;
//
// struct kx022a_chip_info - Kionix accelerometer chip specific information
//
// @name:			name of the device
// @regmap_config:		pointer to register map configuration
// @scale_table:		An array of tables of scaling factors for
// a supported acceleration measurement range.
// Each table containing a single scaling
// factor consisting of two integers. The first
// value in a table is the integer part, and
// the second value is the	fractional part as
// parts per billion.
// @scale_table_size:		Amount of values in tables.
// @channels:			pointer to iio_chan_spec array
// @num_channels:		number of iio_chan_spec channels
// @fifo_length:		number of 16-bit samples in a full buffer
// @buf_smp_lvl_mask:		buffer sample level mask
// @who:			WHO_AM_I register
// @id:				WHO_AM_I register value
// @cntl:			control register 1
// @cntl2:			control register 2
// @odcntl:			output data control register
// @buf_cntl1:			buffer control register 1
// @buf_cntl2:			buffer control register 2
// @buf_clear:			buffer clear register
// @buf_status1:		buffer status register 1
// @buf_read:			buffer read register
// @inc1:			interrupt control register 1
// @inc4:			interrupt control register 4
// @inc5:			interrupt control register 5
// @inc6:			interrupt control register 6
// @xout_l:			x-axis output least significant byte
// @get_fifo_bytes_available:	function pointer to get amount of acceleration
// data bytes currently stored in the sensor's FIFO
// buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kx022a_chip_info {
    pub name: *const c_char,
    pub regmap_config: *const regmap_config,
    pub (*scale_table)[2]: *const c_int,
    pub scale_table_size: c_int,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub fifo_length: c_uint,
    pub buf_smp_lvl_mask: u16,
    pub who: u8,
    pub id: u8,
    pub cntl: u8,
    pub cntl2: u8,
    pub odcntl: u8,
    pub buf_cntl1: u8,
    pub buf_cntl2: u8,
    pub buf_clear: u8,
    pub buf_status1: u8,
    pub buf_read: u8,
    pub inc1: u8,
    pub inc4: u8,
    pub inc5: u8,
    pub inc6: u8,
    pub xout_l: u8,
    pub ): *mut *mut int (get_fifo_bytes_available)(struct kx022a_data,
}

extern "C" {
    pub fn kx022a_probe_internal(dev: *mut device, chip_info: *const kx022a_chip_info) -> c_int;
}
