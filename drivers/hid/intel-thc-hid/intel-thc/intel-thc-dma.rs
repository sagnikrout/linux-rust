//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-thc/intel-thc-dma.h
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

pub const THC_POINTER_WRAPAROUND: c_uint = 0x80;
pub const THC_WRAPAROUND_VALUE_ODD: c_uint = 0x10;
pub const THC_WRAPAROUND_VALUE_EVEN: c_uint = 0x90;

pub const THC_DEFAULT_RXDMA_POLLING_US_INTERVAL: c_int = 100;

//
// THC needs 1KB aligned address, dest_addr is 54 bits, not 64,
// so don't need to send the lower 10-bits of address.
//
pub const THC_ADDRESS_SHIFT: c_int = 10;
//
// THC DMA channels:
// @THC_RXDMA1: Legacy channel, reserved for raw data reading
// @THC_RXDMA2: DMA to read HID data from touch device
// @THC_TXDMA: DMA to write to touch device
// @THC_SWDMA: SW triggered DMA to write and read from touch device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_dma_channel {
    THC_RXDMA1 = 0,
    THC_RXDMA2 = 1,
    THC_TXDMA = 2,
    THC_SWDMA = 3,
    MAX_THC_DMA_CHANNEL
}

//
// THC DMA Physical Memory Descriptor (PRD)
// @dest_addr:		Bit[53:0], destination address in system memory
// @int_on_completion:	Bit[63], if set, thc will trigger interrupt to driver
// @len:		Bit[87:64], length of this entry
// @end_of_prd:		Bit[88], if set, this entry is last one of current PRD table
// @hw_status:		Bit[90:89], hardware status bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_prd_entry {
    pub 54: u64 dest_addr :,
    pub 9: u64 reserved1 :,
    pub 1: u64 int_on_completion :,
    pub 24: u64 len :,
    pub 1: u64 end_of_prd :,
    pub 2: u64 hw_status :,
    pub 37: u64 reserved2 :,
}

//
// Max OS memory fragmentation will be at a 4KB boundary, thus to address 1MB
// of virtually contiguous memory 256 PRD entries are required for a single
// PRD Table. SW writes the number of PRD Entries for each PRD table in the
// THC_M_PRT_RPRD_CNTRL.PTEC register field. The PRD entry's length must be
// multiple of 4KB except for the last entry in a PRD table.
// This is the max possible number of etries supported by HW, in practise we
// there will be less entries in each prd table(the actual number will be
// given by scatter-gather list allocation).
//
pub const PRD_ENTRIES_NUM: c_int = 16;
//
// Number of PRD tables equals to number of data buffers.
// The max number of PRD tables supported by the HW is 128,
// but we allocate only 16.
//
pub const PRD_TABLES_NUM: c_int = 16;
// THC DMA Physical Memory Descriptor Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_prd_table {
    pub entries: [thc_prd_entry; PRD_ENTRIES_NUM],
}

//
// struct thc_dma_configuration - THC DMA configure
// @dma_channel: DMA channel for current DMA configuration
// @prd_tbls_dma_handle: DMA buffer handle
// @dir: Direction of DMA for this config
// @prd_tbls: PRD tables for current DMA
// @sgls: Array of pointers to scatter-gather lists
// @sgls_nent_pages: Number of pages per scatter-gather list
// @sgls_nent: Actual number of entries per scatter-gather list
// @prd_tbl_num: Actual number of PRD tables
// @max_packet_size: Size of the buffer needed for 1 DMA message (1 PRD table)
// @prd_base_addr_high: High 32bits memory address where stores PRD table
// @prd_base_addr_low: Low 32bits memory address where stores PRD table
// @prd_cntrl: PRD control register value
// @dma_cntrl: DMA control register value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_dma_configuration {
    pub dma_channel: thc_dma_channel,
    pub prd_tbls_dma_handle: dma_addr_t,
    pub dir: dma_data_direction,
    pub is_enabled: bool,
    pub prd_tbls: *mut thc_prd_table,
    pub sgls: [*mut scatterlist; PRD_TABLES_NUM],
    pub sgls_nent_pages: [u8; PRD_TABLES_NUM],
    pub sgls_nent: [u8; PRD_TABLES_NUM],
    pub prd_tbl_num: u8,
    pub max_packet_size: usize,
    pub prd_base_addr_high: u32,
    pub prd_base_addr_low: u32,
    pub prd_cntrl: u32,
    pub dma_cntrl: u32,
}

//
// struct thc_dma_context - THC DMA context
// @thc_dma_configuration: Array of all THC Channel configures
// @use_write_interrupts: Indicate TxDMA using interrupt or polling
// @rx_max_size_en: Temp flag to indicate THC I2C Rx max input size control feature
// enabled or not, only be used during SWDMA operation.
// @rx_int_delay_en: Temp flag to indicate THC I2C Rx interrupt delay feature
// enabled or not, only be used during SWDMA operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_dma_context {
    pub dma_config: [thc_dma_configuration; MAX_THC_DMA_CHANNEL],
    pub use_write_interrupts: u8,
    pub rx_max_size_en: bool,
    pub rx_int_delay_en: bool,
}

extern "C" {
    pub fn thc_dma_allocate(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_dma_configure(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_dma_unconfigure(dev: *mut thc_device);
}
extern "C" {
    pub fn thc_dma_release(dev: *mut thc_device);
}
extern "C" {
    pub fn thc_rxdma_reset(dev: *mut thc_device) -> c_int;
}
extern "C" {
    pub fn thc_dma_write(dev: *mut thc_device, buffer: *mut c_void, buf_len: usize) -> c_int;
}
