//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-iop3xx.h
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
// -------------------------------------------------------------------------
// i2c-iop3xx.h algorithm driver definitions private to i2c-iop3xx.c
// -------------------------------------------------------------------------
// Copyright (C) 2003 Peter Milne, D-TACQ Solutions Ltd
// <Peter dot Milne at D hyphen TACQ dot com>
//
// -------------------------------------------------------------------------
pub const I2C_IOP3XX_H: c_int = 1;
//
// iop321 hardware bit definitions
//
pub const IOP3XX_ICR_FAST_MODE: c_uint = 0x8000	/* 1=400kBps, 0=100kBps */;
pub const IOP3XX_ICR_UNIT_RESET: c_uint = 0x4000	/* 1=RESET */;
pub const IOP3XX_ICR_SAD_IE: c_uint = 0x2000	/* 1=Slave Detect Interrupt Enable */;
pub const IOP3XX_ICR_ALD_IE: c_uint = 0x1000	/* 1=Arb Loss Detect Interrupt Enable */;
pub const IOP3XX_ICR_SSD_IE: c_uint = 0x0800	/* 1=Slave STOP Detect Interrupt Enable */;
pub const IOP3XX_ICR_BERR_IE: c_uint = 0x0400	/* 1=Bus Error Interrupt Enable */;
pub const IOP3XX_ICR_RXFULL_IE: c_uint = 0x0200	/* 1=Receive Full Interrupt Enable */;
pub const IOP3XX_ICR_TXEMPTY_IE: c_uint = 0x0100	/* 1=Transmit Empty Interrupt Enable */;
pub const IOP3XX_ICR_GCD: c_uint = 0x0080	/* 1=General Call Disable */;
//
// IOP3XX_ICR_GCD: 1 disables response as slave. "This bit must be set
// when sending a master mode general call message from the I2C unit"
//
pub const IOP3XX_ICR_UE: c_uint = 0x0040	/* 1=Unit Enable */;
//
// "NOTE: To avoid I2C bus integrity problems,
// the user needs to ensure that the GPIO Output Data Register -
// GPOD bits associated with an I2C port are cleared prior to setting
// the enable bit for that I2C serial port.
// The user prepares to enable I2C port 0 and
// I2C port 1 by clearing GPOD bits 7:6 and GPOD bits 5:4, respectively.
//
pub const IOP3XX_ICR_SCLEN: c_uint = 0x0020	/* 1=SCL enable for master mode */;
pub const IOP3XX_ICR_MABORT: c_uint = 0x0010	/* 1=Send a STOP with no data;
// NB TBYTE must be clear
pub const IOP3XX_ICR_TBYTE: c_uint = 0x0008	/* 1=Send/Receive a byte. i2c clears */;
pub const IOP3XX_ICR_NACK: c_uint = 0x0004	/* 1=reply with NACK */;
pub const IOP3XX_ICR_MSTOP: c_uint = 0x0002	/* 1=send a STOP after next data byte */;
pub const IOP3XX_ICR_MSTART: c_uint = 0x0001	/* 1=initiate a START */;
pub const IOP3XX_ISR_BERRD: c_uint = 0x0400	/* 1=BUS ERROR Detected */;
pub const IOP3XX_ISR_SAD: c_uint = 0x0200	/* 1=Slave ADdress Detected */;
pub const IOP3XX_ISR_GCAD: c_uint = 0x0100	/* 1=General Call Address Detected */;
pub const IOP3XX_ISR_RXFULL: c_uint = 0x0080	/* 1=Receive Full */;
pub const IOP3XX_ISR_TXEMPTY: c_uint = 0x0040	/* 1=Transmit Empty */;
pub const IOP3XX_ISR_ALD: c_uint = 0x0020	/* 1=Arbitration Loss Detected */;
pub const IOP3XX_ISR_SSD: c_uint = 0x0010	/* 1=Slave STOP Detected */;
pub const IOP3XX_ISR_BBUSY: c_uint = 0x0008	/* 1=Bus BUSY */;
pub const IOP3XX_ISR_UNITBUSY: c_uint = 0x0004	/* 1=Unit Busy */;
pub const IOP3XX_ISR_NACK: c_uint = 0x0002	/* 1=Unit Rx or Tx a NACK */;
pub const IOP3XX_ISR_RXREAD: c_uint = 0x0001	/* 1=READ 0=WRITE (R/W bit of slave addr */;
pub const IOP3XX_ISR_CLEARBITS: c_uint = 0x07f0;
pub const IOP3XX_ISAR_SAMASK: c_uint = 0x007f;
pub const IOP3XX_IDBR_MASK: c_uint = 0x00ff;
pub const IOP3XX_IBMR_SCL: c_uint = 0x0002;
pub const IOP3XX_IBMR_SDA: c_uint = 0x0001;
pub const IOP3XX_GPOD_I2C0: c_uint = 0x00c0	/* clear these bits to enable ch0 */;
pub const IOP3XX_GPOD_I2C1: c_uint = 0x0030	/* clear these bits to enable ch1 */;

pub const I2C_ERR: c_int = 321;

pub const CR_OFFSET: c_int = 0;
pub const SR_OFFSET: c_uint = 0x4;
pub const SAR_OFFSET: c_uint = 0x8;
pub const DBR_OFFSET: c_uint = 0xc;
pub const CCR_OFFSET: c_uint = 0x10;
pub const BMR_OFFSET: c_uint = 0x14;
pub const IOP3XX_I2C_IO_SIZE: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_iop3xx_data {
    pub ioaddr: *mut void __iomem,
    pub waitq: wait_queue_head_t,
    pub lock: spinlock_t,
    pub SR_received: u32 SR_enabled,,
    pub id: c_int,
    pub gpio_scl: *mut gpio_desc,
    pub gpio_sda: *mut gpio_desc,
}
