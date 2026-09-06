//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-thc/intel-thc-dev.h
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
// Copyright (c) 2024 Intel Corporation

pub const THC_REGMAP_COMMON_OFFSET: c_uint = 0x10;
pub const THC_REGMAP_MMIO_OFFSET: c_uint = 0x1000;
//
// THC Port type
// @THC_PORT_TYPE_SPI: This port is used for HIDSPI
// @THC_PORT_TYPE_I2C: This port is used for HIDI2C
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_port_type {
    THC_PORT_TYPE_SPI = 0,
    THC_PORT_TYPE_I2C = 1,
}

//
// THC interrupt flag
// @THC_NONDMA_INT: THC non-DMA interrupt
// @THC_RXDMA1_INT: THC RxDMA1 interrupt
// @THC_RXDMA2_INT: THC RxDMA2 interrupt
// @THC_SWDMA_INT: THC SWDMA interrupt
// @THC_TXDMA_INT: THC TXDMA interrupt
// @THC_PIO_DONE_INT: THC PIO complete interrupt
// @THC_I2CSUBIP_INT: THC I2C subsystem interrupt
// @THC_TXN_ERR_INT: THC transfer error interrupt
// @THC_FATAL_ERR_INT: THC fatal error interrupt
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_int_type {
    THC_NONDMA_INT = 0,
    THC_RXDMA1_INT = 1,
    THC_RXDMA2_INT = 2,
    THC_SWDMA_INT = 3,
    THC_TXDMA_INT = 4,
    THC_PIO_DONE_INT = 5,
    THC_I2CSUBIP_INT = 6,
    THC_TXN_ERR_INT = 7,
    THC_FATAL_ERR_INT = 8,
    THC_UNKNOWN_INT
}

//
// struct thc_i2c_config - THC I2C bus configuration
// @target_addr: Slave address of touch device (TIC)
// @addr_mode: Slave address mode of touch device (TIC), 7bit or 10bit
// @speed: I2C bus frequency speed mode
// @scl_hcnt: I2C clock SCL high count
// @scl_lcnt: I2C clock SCL low count
// @sda_tx_hold: I2C Data SDA transmit hold period
// @sda_rx_hold: I2C Data SDA receive hold period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_i2c_config {
    pub target_addr: u16,
    pub addr_mode: u8,
    pub speed: u32,
    pub scl_hcnt: u32,
    pub scl_lcnt: u32,
    pub sda_tx_hold: u32,
    pub sda_rx_hold: u32,
}

//
// struct thc_device - THC private device struct
// @thc_regmap: MMIO regmap structure for accessing THC registers
// @mmio_addr: MMIO registers address
// @thc_bus_lock: Mutex locker for THC config
// @port_type: Port type of THC port instance
// @pio_int_supported: PIO interrupt supported flag
// @dma_ctx: DMA specific data
// @wot: THC Wake-on-Touch data
// @write_complete_wait: Signal event for DMA write complete
// @swdma_complete_wait: Signal event for SWDMA sequence complete
// @write_done: Bool value that indicates if DMA write is done
// @swdma_done: Bool value that indicates if SWDMA sequence is done
// @perf_limit: The delay between read operation and write operation
// @i2c_subip_regs: The copy of THC I2C sub-system registers for resuming restore
// @i2c_max_rx_size: I2C Rx transfer max input size
// @i2c_int_delay_us: I2C input interrupt delay, unit is us
// @i2c_max_rx_size_en: Bool value that indicates I2C max input size control enabled or not
// @i2c_int_delay_en: Bool value that indicates I2C input interrupt delay enabled or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_device {
    pub dev: *mut device,
    pub thc_regmap: *mut regmap,
    pub mmio_addr: *mut void __iomem,
    pub thc_bus_lock: mutex,
    pub port_type: thc_port_type,
    pub pio_int_supported: bool,
    pub dma_ctx: *mut thc_dma_context,
    pub wot: thc_wot,
    pub write_complete_wait: wait_queue_head_t,
    pub swdma_complete_wait: wait_queue_head_t,
    pub write_done: bool,
    pub swdma_done: bool,
    pub perf_limit: u32,
    pub i2c_subip_regs: *mut u32,
    pub i2c_max_rx_size: u32,
    pub i2c_int_delay_us: u32,
    pub i2c_max_rx_size_en: bool,
    pub i2c_int_delay_en: bool,
}

extern "C" {
    pub fn thc_interrupt_config(dev: *mut thc_device);
}
extern "C" {
    pub fn thc_int_trigger_type_select(dev: *mut thc_device, edge_trigger: bool);
}
extern "C" {
    pub fn thc_interrupt_enable(dev: *mut thc_device, int_enable: bool);
}
extern "C" {
    pub fn thc_set_pio_interrupt_support(dev: *mut thc_device, supported: bool);
}
extern "C" {
    pub fn thc_interrupt_quiesce(dev: *const thc_device, int_quiesce: bool) -> c_int;
}
extern "C" {
    pub fn thc_ltr_config(dev: *mut thc_device, active_ltr_us: u32, lp_ltr_us: u32);
}
extern "C" {
    pub fn thc_change_ltr_mode(dev: *mut thc_device, ltr_mode: u32);
}
extern "C" {
    pub fn thc_ltr_unconfig(dev: *mut thc_device);
}
extern "C" {
    pub fn thc_int_cause_read(dev: *mut thc_device) -> u32;
}
extern "C" {
    pub fn thc_interrupt_handler(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_port_select(dev: *mut thc_device, port_type: thc_port_type) -> c_int;
}
extern "C" {
    pub fn thc_i2c_subip_init(dev: *mut thc_device, i2c_config: *const thc_i2c_config) -> c_int;
}
extern "C" {
    pub fn thc_i2c_subip_regs_save(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_i2c_subip_regs_restore(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_i2c_set_rx_max_size(dev: *mut thc_device, max_rx_size: u32) -> c_int;
}
extern "C" {
    pub fn thc_i2c_rx_max_size_enable(dev: *mut thc_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn thc_i2c_set_rx_int_delay(dev: *mut thc_device, delay_us: u32) -> c_int;
}
extern "C" {
    pub fn thc_i2c_rx_int_delay_enable(dev: *mut thc_device, enable: bool) -> c_int;
}
