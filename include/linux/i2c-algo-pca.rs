//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-algo-pca.h
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
// Chips known to the pca algo
pub const I2C_PCA_CHIP_9564: c_uint = 0x00;
pub const I2C_PCA_CHIP_9665: c_uint = 0x01;
// Internal period for PCA9665 oscilator

// Clock speeds for the bus for PCA9564
pub const I2C_PCA_CON_330kHz: c_uint = 0x00;
pub const I2C_PCA_CON_288kHz: c_uint = 0x01;
pub const I2C_PCA_CON_217kHz: c_uint = 0x02;
pub const I2C_PCA_CON_146kHz: c_uint = 0x03;
pub const I2C_PCA_CON_88kHz: c_uint = 0x04;
pub const I2C_PCA_CON_59kHz: c_uint = 0x05;
pub const I2C_PCA_CON_44kHz: c_uint = 0x06;
pub const I2C_PCA_CON_36kHz: c_uint = 0x07;
// PCA9564 registers
pub const I2C_PCA_STA: c_uint = 0x00 /* STATUS  Read Only  */;
pub const I2C_PCA_TO: c_uint = 0x00 /* TIMEOUT Write Only */;
pub const I2C_PCA_DAT: c_uint = 0x01 /* DATA    Read/Write */;
pub const I2C_PCA_ADR: c_uint = 0x02 /* OWN ADR Read/Write */;
pub const I2C_PCA_CON: c_uint = 0x03 /* CONTROL Read/Write */;
// PCA9665 registers
pub const I2C_PCA_INDPTR: c_uint = 0x00 /* INDIRECT Pointer Write Only */;
pub const I2C_PCA_IND: c_uint = 0x02 /* INDIRECT Read/Write */;
// PCA9665 indirect registers
pub const I2C_PCA_ICOUNT: c_uint = 0x00 /* Byte Count for buffered mode */;
pub const I2C_PCA_IADR: c_uint = 0x01 /* OWN ADR */;
pub const I2C_PCA_ISCLL: c_uint = 0x02 /* SCL LOW period */;
pub const I2C_PCA_ISCLH: c_uint = 0x03 /* SCL HIGH period */;
pub const I2C_PCA_ITO: c_uint = 0x04 /* TIMEOUT */;
pub const I2C_PCA_IPRESET: c_uint = 0x05 /* Parallel bus reset */;
pub const I2C_PCA_IMODE: c_uint = 0x06 /* I2C Bus mode */;
// PCA9665 I2C bus mode
pub const I2C_PCA_MODE_STD: c_uint = 0x00 /* Standard mode */;
pub const I2C_PCA_MODE_FAST: c_uint = 0x01 /* Fast mode */;
pub const I2C_PCA_MODE_FASTP: c_uint = 0x02 /* Fast Plus mode */;
pub const I2C_PCA_MODE_TURBO: c_uint = 0x03 /* Turbo mode */;
pub const I2C_PCA_CON_AA: c_uint = 0x80 /* Assert Acknowledge */;
pub const I2C_PCA_CON_ENSIO: c_uint = 0x40 /* Enable */;
pub const I2C_PCA_CON_STA: c_uint = 0x20 /* Start */;
pub const I2C_PCA_CON_STO: c_uint = 0x10 /* Stop */;
pub const I2C_PCA_CON_SI: c_uint = 0x08 /* Serial Interrupt */;
pub const I2C_PCA_CON_CR: c_uint = 0x07 /* Clock Rate (MASK) */;
//
// struct pca_i2c_bus_settings - The configured PCA i2c bus settings
// @mode: Configured i2c bus mode
// @tlow: Configured SCL LOW period
// @thi: Configured SCL HIGH period
// @clock_freq: The configured clock frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca_i2c_bus_settings {
    pub mode: c_int,
    pub tlow: c_int,
    pub thi: c_int,
    pub clock_freq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_pca_data {
    pub /: *mut *mut *mut void data; / private low level data,
    pub val): *mut *mut *mut void (write_byte) (void data, int reg, int,
    pub reg): *mut *mut *mut int (read_byte) (void data, int,
    pub data): *mut *mut int (wait_for_completion_cb) (void,
    pub data): *mut *mut void (reset_chip) (void,
// For PCA9564, use one of the predefined frequencies:
// 330000, 288000, 217000, 146000, 88000, 59000, 44000, 36000
// For PCA9665, use the frequency you want here.
    pub i2c_clock: c_uint,
    pub chip: c_uint,
    pub bus_settings: pca_i2c_bus_settings,
}

extern "C" {
    pub fn i2c_pca_add_bus(: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn i2c_pca_add_numbered_bus(: *mut i2c_adapter) -> c_int;
}
