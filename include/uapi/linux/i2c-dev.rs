//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/i2c-dev.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// i2c-dev.h - I2C bus char device interface
//
// Copyright (C) 1995-97 Simon G. Vogl
// Copyright (C) 1998-99 Frodo Looijaard <frodol@dds.nl>
//

// /dev/i2c-X ioctl commands.  The ioctl's parameter is always an
// unsigned long, except for:
// - I2C_FUNCS, takes pointer to an unsigned long
// - I2C_RDWR, takes pointer to struct i2c_rdwr_ioctl_data
// - I2C_SMBUS, takes pointer to struct i2c_smbus_ioctl_data
//
pub const I2C_RETRIES: c_uint = 0x0701	/* number of times a device address should;
pub const I2C_TIMEOUT: c_uint = 0x0702	/* set timeout in units of 10 ms */;
// NOTE: Slave address is 7 or 10 bits, but 10-bit addresses
// are NOT supported! (due to code brokenness)
//
pub const I2C_SLAVE: c_uint = 0x0703	/* Use this slave address */;
pub const I2C_SLAVE_FORCE: c_uint = 0x0706	/* Use this slave address, even if it;
pub const I2C_TENBIT: c_uint = 0x0704	/* 0 for 7 bit addrs, != 0 for 10 bit */;
pub const I2C_FUNCS: c_uint = 0x0705	/* Get the adapter functionality mask */;
pub const I2C_RDWR: c_uint = 0x0707	/* Combined R/W transfer (one STOP only) */;
pub const I2C_PEC: c_uint = 0x0708	/* != 0 to use PEC with SMBus */;
pub const I2C_SMBUS: c_uint = 0x0720	/* SMBus transfer */;
// This is the structure as used in the I2C_SMBUS ioctl call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_smbus_ioctl_data {
    pub read_write: __u8,
    pub command: __u8,
    pub size: __u32,
    pub data: *mut i2c_smbus_data __user,
}

// This is the structure as used in the I2C_RDWR ioctl call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_rdwr_ioctl_data {
    pub /: *mut *mut *mut i2c_msg __user msgs; / pointers to i2c_msgs,
    pub /: *mut *mut __u32 nmsgs; / number of i2c_msgs,
}

pub const I2C_RDWR_IOCTL_MAX_MSGS: c_int = 42;
// Originally defined with a typo, keep it for compatibility

