//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-at91.h
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
// i2c Support for Atmel's AT91 Two-Wire Interface (TWI)
//
// Copyright (C) 2011 Weinmann Medical GmbH
// Author: Nikolaus Voss <n.voss@weinmann.de>
//
// Evolved from original work by:
// Copyright (C) 2004 Rick Bronson
// Converted to 2.6 by Andrew Victor <andrew@sanpeople.com>
//
// Borrowed heavily from original work by:
// Copyright (C) 2000 Philip Edelbrock <phil@stimpy.netroedge.com>
//

pub const AUTOSUSPEND_TIMEOUT: c_int = 2000;
pub const AT91_I2C_MAX_ALT_CMD_DATA_SIZE: c_int = 256;
// AT91 TWI register definitions
pub const AT91_TWI_CR: c_uint = 0x0000	/* Control Register */;

pub const AT91_TWI_MMR: c_uint = 0x0004	/* Master Mode Register */;
pub const AT91_TWI_IADRSZ_1: c_uint = 0x0100	/* Internal Device Address Size */;

pub const AT91_TWI_SMR: c_uint = 0x0008	/* Slave Mode Register */;
pub const AT91_TWI_SMR_SADR_MAX: c_uint = 0x007f;

pub const AT91_TWI_IADR: c_uint = 0x000c	/* Internal Address Register */;
pub const AT91_TWI_CWGR: c_uint = 0x0010	/* Clock Waveform Generator Reg */;
pub const AT91_TWI_CWGR_HOLD_MAX: c_uint = 0x1f;

pub const AT91_TWI_SR: c_uint = 0x0020	/* Status Register */;

pub const AT91_TWI_IER: c_uint = 0x0024	/* Interrupt Enable Register */;
pub const AT91_TWI_IDR: c_uint = 0x0028	/* Interrupt Disable Register */;
pub const AT91_TWI_IMR: c_uint = 0x002c	/* Interrupt Mask Register */;
pub const AT91_TWI_RHR: c_uint = 0x0030	/* Receive Holding Register */;
pub const AT91_TWI_THR: c_uint = 0x0034	/* Transmit Holding Register */;
pub const AT91_TWI_ACR: c_uint = 0x0040	/* Alternative Command Register */;

pub const AT91_TWI_FILTR: c_uint = 0x0044;

pub const AT91_TWI_FILTR_THRES_MAX: c_int = 7;

pub const AT91_TWI_FMR: c_uint = 0x0050	/* FIFO Mode Register */;

pub const AT91_TWI_ONE_DATA: c_uint = 0x0;
pub const AT91_TWI_TWO_DATA: c_uint = 0x1;
pub const AT91_TWI_FOUR_DATA: c_uint = 0x2;
pub const AT91_TWI_FLR: c_uint = 0x0054	/* FIFO Level Register */;
pub const AT91_TWI_FSR: c_uint = 0x0060	/* FIFO Status Register */;
pub const AT91_TWI_FIER: c_uint = 0x0064	/* FIFO Interrupt Enable Register */;
pub const AT91_TWI_FIDR: c_uint = 0x0068	/* FIFO Interrupt Disable Register */;
pub const AT91_TWI_FIMR: c_uint = 0x006c	/* FIFO Interrupt Mask Register */;
pub const AT91_TWI_VER: c_uint = 0x00fc	/* Version Register */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_twi_pdata {
    pub clk_max_div: unsigned,
    pub clk_offset: unsigned,
    pub has_unre_flag: bool,
    pub has_alt_cmd: bool,
    pub has_hold_field: bool,
    pub has_dig_filtr: bool,
    pub has_adv_dig_filtr: bool,
    pub has_ana_filtr: bool,
    pub has_clear_cmd: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_twi_dma {
    pub chan_rx: *mut dma_chan,
    pub chan_tx: *mut dma_chan,
    pub sg: [scatterlist; 2],
    pub data_desc: *mut dma_async_tx_descriptor,
    pub direction: dma_data_direction,
    pub buf_mapped: bool,
    pub xfer_in_progress: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_twi_dev {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub cmd_complete: completion,
    pub clk: *mut clk,
    pub buf: *mut u8,
    pub buf_len: usize,
    pub msg: *mut i2c_msg,
    pub irq: c_int,
    pub imr: unsigned,
    pub transfer_status: unsigned,
    pub adapter: i2c_adapter,
    pub twi_cwgr_reg: unsigned,
    pub pdata: *mut at91_twi_pdata,
    pub use_dma: bool,
    pub use_alt_cmd: bool,
    pub recv_len_abort: bool,
    pub fifo_size: u32,
    pub dma: at91_twi_dma,
    pub slave_detected: bool,
    pub rinfo: i2c_bus_recovery_info,

    pub smr: unsigned,
    pub slave: *mut i2c_client,

    pub enable_dig_filt: bool,
    pub enable_ana_filt: bool,
    pub filter_width: u32,
}

extern "C" {
    pub fn at91_twi_read(dev: *mut at91_twi_dev, reg: unsigned) -> unsigned;
}
extern "C" {
    pub fn at91_twi_write(dev: *mut at91_twi_dev, reg: unsigned, val: unsigned);
}
extern "C" {
    pub fn at91_disable_twi_interrupts(dev: *mut at91_twi_dev);
}
extern "C" {
    pub fn at91_twi_irq_save(dev: *mut at91_twi_dev);
}
extern "C" {
    pub fn at91_twi_irq_restore(dev: *mut at91_twi_dev);
}
extern "C" {
    pub fn at91_init_twi_bus(dev: *mut at91_twi_dev);
}
extern "C" {
    pub fn at91_init_twi_bus_master(dev: *mut at91_twi_dev);
}

extern "C" {
    pub fn at91_init_twi_bus_slave(dev: *mut at91_twi_dev);
}

