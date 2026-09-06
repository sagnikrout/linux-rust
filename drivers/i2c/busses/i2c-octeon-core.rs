//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-octeon-core.h
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

// Controller command patterns

pub const SW_TWSI_SIZE_SHIFT: c_int = 52;
pub const SW_TWSI_ADDR_SHIFT: c_int = 40;

// Controller opcode word (bits 60:57)
pub const SW_TWSI_OP_SHIFT: c_int = 57;

// Controller extended opcode word (bits 34:32)
pub const SW_TWSI_EOP_SHIFT: c_int = 32;

// Controller command and status bits
pub const TWSI_CTL_CE: c_uint = 0x80	/* High level controller enable */;
pub const TWSI_CTL_ENAB: c_uint = 0x40	/* Bus enable */;
pub const TWSI_CTL_STA: c_uint = 0x20	/* Controller-mode start, HW clears when done */;
pub const TWSI_CTL_STP: c_uint = 0x10	/* Controller-mode stop, HW clears when done */;
pub const TWSI_CTL_IFLG: c_uint = 0x08	/* HW event, SW writes 0 to ACK */;
pub const TWSI_CTL_AAK: c_uint = 0x04	/* Assert ACK */;
// Status values
pub const STAT_BUS_ERROR: c_uint = 0x00;
pub const STAT_START: c_uint = 0x08;
pub const STAT_REP_START: c_uint = 0x10;
pub const STAT_TXADDR_ACK: c_uint = 0x18;
pub const STAT_TXADDR_NAK: c_uint = 0x20;
pub const STAT_TXDATA_ACK: c_uint = 0x28;
pub const STAT_TXDATA_NAK: c_uint = 0x30;
pub const STAT_LOST_ARB_38: c_uint = 0x38;
pub const STAT_RXADDR_ACK: c_uint = 0x40;
pub const STAT_RXADDR_NAK: c_uint = 0x48;
pub const STAT_RXDATA_ACK: c_uint = 0x50;
pub const STAT_RXDATA_NAK: c_uint = 0x58;
pub const STAT_SLAVE_60: c_uint = 0x60;
pub const STAT_LOST_ARB_68: c_uint = 0x68;
pub const STAT_SLAVE_70: c_uint = 0x70;
pub const STAT_LOST_ARB_78: c_uint = 0x78;
pub const STAT_SLAVE_80: c_uint = 0x80;
pub const STAT_SLAVE_88: c_uint = 0x88;
pub const STAT_GENDATA_ACK: c_uint = 0x90;
pub const STAT_GENDATA_NAK: c_uint = 0x98;
pub const STAT_SLAVE_A0: c_uint = 0xA0;
pub const STAT_SLAVE_A8: c_uint = 0xA8;
pub const STAT_LOST_ARB_B0: c_uint = 0xB0;
pub const STAT_SLAVE_LOST: c_uint = 0xB8;
pub const STAT_SLAVE_NAK: c_uint = 0xC0;
pub const STAT_SLAVE_ACK: c_uint = 0xC8;
pub const STAT_AD2W_ACK: c_uint = 0xD0;
pub const STAT_AD2W_NAK: c_uint = 0xD8;
pub const STAT_WDOG_TOUT: c_uint = 0xF0;
pub const STAT_IDLE: c_uint = 0xF8;
// TWSI_INT values

// Register offsets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_i2c_reg_offset {
    pub sw_twsi: c_uint,
    pub twsi_int: c_uint,
    pub sw_twsi_ext: c_uint,
    pub mode: c_uint,
    pub block_ctl: c_uint,
    pub block_sts: c_uint,
    pub block_fifo: c_uint,
}

// TWSX_MODE register

// TWSX_BLOCK_STS register

// Set BUS_MON_RST to reset bus monitor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_i2c {
    pub queue: wait_queue_head_t,
    pub adap: i2c_adapter,
    pub roff: octeon_i2c_reg_offset,
    pub clk: *mut clk,
    pub irq: c_int,
    pub /: *mut *mut int hlc_irq; / For cn7890 only,
    pub twsi_freq: u32,
    pub sys_freq: c_int,
    pub twsi_base: *mut void __iomem,
    pub dev: *mut device,
    pub hlc_enabled: bool,
    pub block_enabled: bool,
    pub broken_irq_mode: bool,
    pub broken_irq_check: bool,
    pub ): *mut *mut void (int_enable)(struct octeon_i2c,
    pub ): *mut *mut void (int_disable)(struct octeon_i2c,
    pub ): *mut *mut void (hlc_int_enable)(struct octeon_i2c,
    pub ): *mut *mut void (hlc_int_disable)(struct octeon_i2c,
    pub int_enable_cnt: core::sync::atomic::AtomicI32,
    pub hlc_int_enable_cnt: core::sync::atomic::AtomicI32,
    pub alert_data: i2c_smbus_alert_setup,
    pub ara: *mut i2c_client,
}

//
// octeon_i2c_reg_write - write an I2C core register
// @i2c: The struct octeon_i2c
// @eop_reg: Register selector
// @data: Value to be written
//
// The I2C core registers are accessed indirectly via the OCTEON_REG_SW_TWSI CSR.
//

//
// octeon_i2c_reg_read - read lower bits of an I2C core register
// @i2c: The struct octeon_i2c
// @eop_reg: Register selector
//
// Returns the data.
//
// The I2C core registers are accessed indirectly via the SW_TWSI CSR.
//
// signal that the returned data is invalid
// error = -EIO;

//
// octeon_i2c_read_int - read the OCTEON_REG_TWSI_INT register
// @i2c: The struct octeon_i2c
//
// Returns the value of the register.
//
extern "C" {
    pub fn __raw_readq(OCTEON_REG_TWSI_INT(i2c): i2c->twsi_base +) -> return;
}
//
// octeon_i2c_write_int - write the OCTEON_REG_TWSI_INT register
// @i2c: The struct octeon_i2c
// @data: Value to be written
//

pub const PCI_SUBSYS_DEVID_9XXX: c_uint = 0xB;

//
// octeon_i2c_is_otx2 - check for chip ID
// @pdev: PCI dev structure
//
// Returns true if the device is an OcteonTX2, false otherwise.
//
// Prototypes
extern "C" {
    pub fn octeon_i2c_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn octeon_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
}
extern "C" {
    pub fn octeon_i2c_init_lowlevel(i2c: *mut octeon_i2c) -> c_int;
}
extern "C" {
    pub fn octeon_i2c_set_clock(i2c: *mut octeon_i2c);
}
