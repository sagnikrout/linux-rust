//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-designware-core.h
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
// Synopsys DesignWare I2C adapter driver.
//
// Based on the TI DAVINCI I2C adapter driver.
//
// Copyright (C) 2006 Texas Instruments.
// Copyright (C) 2007 MontaVista Software Inc.
// Copyright (C) 2009 Provigent Ltd.
//

//
// Register access parameters
//
pub const DW_IC_REG_STEP_BYTES: c_int = 2;
pub const DW_IC_REG_WORD_SHIFT: c_int = 16;
//
// FIFO depth configuration
//

pub const DW_IC_FIFO_MIN_DEPTH: c_int = 2;
pub const DW_IC_SDA_HOLD_MIN_VERS: c_uint = 0x3131312A /* "111*" == v1.11* */;
pub const DW_IC_COMP_TYPE_VALUE: c_uint = 0x44570140 /* "DW" + 0x0140 */;

pub const DW_IC_SDA_HOLD_RX_SHIFT: c_int = 16;

pub const DW_IC_ERR_TX_ABRT: c_uint = 0x1;

//
// Sofware status flags
//

//
// operation modes
//
pub const DW_IC_MASTER: c_int = 0;
pub const DW_IC_SLAVE: c_int = 1;
//
// Hardware abort codes from the DW_IC_TX_ABRT_SOURCE register.
//
// Only expected abort codes are listed here,
// refer to the datasheet for the full list.
//
pub const ABRT_7B_ADDR_NOACK: c_int = 0;
pub const ABRT_10ADDR1_NOACK: c_int = 1;
pub const ABRT_10ADDR2_NOACK: c_int = 2;
pub const ABRT_TXDATA_NOACK: c_int = 3;
pub const ABRT_GCALL_NOACK: c_int = 4;
pub const ABRT_GCALL_READ: c_int = 5;
pub const ABRT_SBYTE_ACKDET: c_int = 7;
pub const ABRT_SBYTE_NORSTRT: c_int = 9;
pub const ABRT_10B_RD_NORSTRT: c_int = 10;
pub const ABRT_MASTER_DIS: c_int = 11;
pub const ARB_LOST: c_int = 12;
pub const ABRT_SLAVE_FLUSH_TXFIFO: c_int = 13;
pub const ABRT_SLAVE_ARBLOST: c_int = 14;
pub const ABRT_SLAVE_RD_INTX: c_int = 15;

//
// struct dw_i2c_dev - private i2c-designware data
// @dev: driver model device node
// @map: IO registers map
// @sysmap: System controller registers map
// @base: IO registers pointer
// @ext: Extended IO registers pointer
// @cmd_complete: tx completion indicator
// @clk: input reference clock
// @pclk: clock required to access the registers
// @rst: optional reset for the controller
// @slave: represent an I2C slave device
// @get_clk_rate_khz: callback to retrieve IP specific bus speed
// @cmd_err: run time hardware error code
// @msgs: points to an array of messages currently being transferred
// @msgs_num: the number of elements in msgs
// @msg_write_idx: the element index of the current tx message in the msgs array
// @tx_buf_len: the length of the current tx buffer
// @tx_buf: the current tx buffer
// @msg_read_idx: the element index of the current rx message in the msgs array
// @rx_buf_len: the length of the current rx buffer
// @rx_buf: the current rx buffer
// @msg_err: error status of the current transfer
// @status: i2c master status, one of STATUS_
// @abort_source: copy of the TX_ABRT_SOURCE register
// @sw_mask: SW mask of DW_IC_INTR_MASK used in polling mode
// @irq: interrupt number for the i2c master
// @flags: platform specific flags like type of IO accessors or model
// @adapter: i2c subsystem adapter node
// @functionality: I2C_FUNC_* ORed bits to reflect what controller does support
// @master_cfg: configuration for the master device
// @slave_cfg: configuration for the slave device
// @tx_fifo_depth: depth of the hardware tx fifo
// @rx_fifo_depth: depth of the hardware rx fifo
// @rx_outstanding: current master-rx elements in tx fifo
// @timings: bus clock frequency, SDA hold and other timings
// @sda_hold_time: SDA hold value
// @ss_hcnt: standard speed HCNT value
// @ss_lcnt: standard speed LCNT value
// @fs_hcnt: fast speed HCNT value
// @fs_lcnt: fast speed LCNT value
// @fp_hcnt: fast plus HCNT value
// @fp_lcnt: fast plus LCNT value
// @hs_hcnt: high speed HCNT value
// @hs_lcnt: high speed LCNT value
// @acquire_lock: function to acquire a hardware lock on the bus
// @release_lock: function to release a hardware lock on the bus
// @semaphore_idx: Index of table with semaphore type attached to the bus. It's
// -1 if there is no semaphore.
// @shared_with_punit: true if this bus is shared with the SoC's PUNIT
// @set_sda_hold_time: callback to retrieve IP specific SDA hold timing
// @mode: operation mode - DW_IC_MASTER or DW_IC_SLAVE
// @rinfo: I²C GPIO recovery information
// @bus_capacitance_pF: bus capacitance in picofarads
// @clk_freq_optimized: if this value is true, it means the hardware reduces
// its internal clock frequency by reducing the internal latency required
// to generate the high period and low period of SCL line.
// @emptyfifo_hold_master: true if the controller acting as master holds
// the clock when the Tx FIFO is empty instead of emitting a stop.
//
// HCNT and LCNT parameters can be used if the platform knows more accurate
// values than the one computed based only on the input clock frequency.
// Leave them to be %0 if not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i2c_dev {
    pub dev: *mut device,
    pub map: *mut regmap,
    pub sysmap: *mut regmap,
    pub base: *mut void __iomem,
    pub ext: *mut void __iomem,
    pub cmd_complete: completion,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub rst: *mut reset_control,
    pub slave: *mut i2c_client,
    pub dev): *mut *mut u32 (get_clk_rate_khz) (struct dw_i2c_dev,
    pub cmd_err: c_int,
    pub msgs: *mut i2c_msg,
    pub msgs_num: c_int,
    pub msg_write_idx: c_int,
    pub tx_buf_len: u32,
    pub tx_buf: *mut u8,
    pub msg_read_idx: c_int,
    pub rx_buf_len: u32,
    pub rx_buf: *mut u8,
    pub msg_err: c_int,
    pub status: c_uint,
    pub abort_source: c_uint,
    pub sw_mask: c_uint,
    pub irq: c_int,
    pub flags: u32,
    pub adapter: i2c_adapter,
    pub functionality: u32,
    pub master_cfg: u32,
    pub slave_cfg: u32,
    pub tx_fifo_depth: c_uint,
    pub rx_fifo_depth: c_uint,
    pub rx_outstanding: c_int,
    pub timings: i2c_timings,
    pub sda_hold_time: u32,
    pub ss_hcnt: u16,
    pub ss_lcnt: u16,
    pub fs_hcnt: u16,
    pub fs_lcnt: u16,
    pub fp_hcnt: u16,
    pub fp_lcnt: u16,
    pub hs_hcnt: u16,
    pub hs_lcnt: u16,
    pub (*acquire_lock)(void): *mut c_int,
    pub (*release_lock)(void): *mut c_void,
    pub semaphore_idx: c_int,
    pub shared_with_punit: bool,
    pub dev): *mut *mut int (set_sda_hold_time)(struct dw_i2c_dev,
    pub mode: c_int,
    pub rinfo: i2c_bus_recovery_info,
    pub bus_capacitance_pF: u32,
    pub clk_freq_optimized: bool,
    pub emptyfifo_hold_master: bool,
}

//
// Enable UCSI interrupt by writing 0xd at register
// offset 0x474 specified in hardware specification.
//
pub const AMD_UCSI_INTR_REG: c_uint = 0x474;
pub const AMD_UCSI_INTR_EN: c_uint = 0xd;
pub const TXGBE_TX_FIFO_DEPTH: c_int = 4;
pub const TXGBE_RX_FIFO_DEPTH: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_dw_semaphore_callbacks {
    pub dev): *mut *mut int (probe)(struct dw_i2c_dev,
}

extern "C" {
    pub fn i2c_dw_clk_rate(dev: *mut dw_i2c_dev) -> u32;
}
extern "C" {
    pub fn i2c_dw_prepare_clk(dev: *mut dw_i2c_dev, prepare: bool) -> c_int;
}
extern "C" {
    pub fn i2c_dw_acquire_lock(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_release_lock(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_wait_bus_not_busy(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_handle_tx_abort(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_func(adap: *mut i2c_adapter) -> u32;
}
extern "C" {
    pub fn i2c_dw_isr_master(dev: *mut dw_i2c_dev) -> irqreturn_t;
}
// intr_mask = dev->sw_mask;
extern "C" {
    pub fn __i2c_dw_disable(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_disable(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_configure_master(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_probe_master(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
}

extern "C" {
    pub fn i2c_dw_configure_slave(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_isr_slave(dev: *mut dw_i2c_dev) -> irqreturn_t;
}
extern "C" {
    pub fn i2c_dw_reg_slave(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn i2c_dw_unreg_slave(client: *mut i2c_client) -> c_int;
}

extern "C" {
    pub fn i2c_dw_probe(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_init(dev: *mut dw_i2c_dev) -> c_int;
}
extern "C" {
    pub fn i2c_dw_shutdown(dev: *mut dw_i2c_dev);
}
extern "C" {
    pub fn i2c_dw_set_mode(dev: *mut dw_i2c_dev, mode: c_int);
}

extern "C" {
    pub fn i2c_dw_baytrail_probe_lock_support(dev: *mut dw_i2c_dev) -> c_int;
}

extern "C" {
    pub fn i2c_dw_amdpsp_probe_lock_support(dev: *mut dw_i2c_dev) -> c_int;
}

extern "C" {
    pub fn i2c_dw_fw_parse_and_configure(dev: *mut dw_i2c_dev) -> c_int;
}
