//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/mv_xor.h
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
// Copyright (C) 2007, 2008, Marvell International Ltd.
//

pub const MV_XOR_SLOT_SIZE: c_int = 64;
pub const MV_XOR_THRESHOLD: c_int = 1;
pub const MV_XOR_MAX_CHANNELS: c_int = 2;

// Values for the XOR_CONFIG register
pub const XOR_OPERATION_MODE_XOR: c_int = 0;
pub const XOR_OPERATION_MODE_MEMCPY: c_int = 2;
pub const XOR_OPERATION_MODE_IN_DESC: c_int = 7;

pub const XOR_DESC_SUCCESS: c_uint = 0x40000000;

pub const WINDOW_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_xor_device {
    pub xor_base: *mut void __iomem,
    pub xor_high_base: *mut void __iomem,
    pub clk: *mut clk,
    pub channels: [*mut mv_xor_chan; MV_XOR_MAX_CHANNELS],
    pub xor_type: c_int,
    pub win_start: [u32; WINDOW_COUNT],
    pub win_end: [u32; WINDOW_COUNT],
}

//
// struct mv_xor_chan - internal representation of a XOR channel
// @pending: allows batching of hardware operations
// @lock: serializes enqueue/dequeue operations to the descriptors pool
// @mmr_base: memory mapped register base
// @idx: the index of the xor channel
// @chain: device chain view of the descriptors
// @free_slots: free slots usable by the channel
// @allocated_slots: slots allocated by the driver
// @completed_slots: slots completed by HW but still need to be acked
// @device: parent device
// @common: common dmaengine channel object members
// @slots_allocated: records the actual size of the descriptor slot pool
// @irq_tasklet: bottom half where mv_xor_slot_cleanup runs
// @op_in_desc: new mode of driver, each op is written to descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_xor_chan {
    pub pending: c_int,
    pub /: *mut *mut spinlock_t lock; / protects the descriptor slot pool,
    pub mmr_base: *mut void __iomem,
    pub mmr_high_base: *mut void __iomem,
    pub idx: c_uint,
    pub irq: c_int,
    pub chain: list_head,
    pub free_slots: list_head,
    pub allocated_slots: list_head,
    pub completed_slots: list_head,
    pub dma_desc_pool: dma_addr_t,
    pub dma_desc_pool_virt: *mut c_void,
    pub pool_size: usize,
    pub dmadev: dma_device,
    pub dmachan: dma_chan,
    pub slots_allocated: c_int,
    pub irq_tasklet: tasklet_struct,
    pub op_in_desc: c_int,
    pub dummy_src: [c_char; MV_XOR_MIN_BYTE_COUNT],
    pub dummy_dst: [c_char; MV_XOR_MIN_BYTE_COUNT],
    pub dummy_dst_addr: dma_addr_t dummy_src_addr,,
    pub saved_int_mask_reg: u32 saved_config_reg,,
    pub xordev: *mut mv_xor_device,
}

//
// struct mv_xor_desc_slot - software descriptor
// @node: node on the mv_xor_chan lists
// @hw_desc: virtual address of the hardware descriptor chain
// @phys: hardware address of the hardware descriptor chain
// @slot_used: slot in use or not
// @idx: pool index
// @tx_list: list of slots that make up a multi-descriptor transaction
// @async_tx: support for the async_tx api
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_xor_desc_slot {
    pub node: list_head,
    pub sg_tx_list: list_head,
    pub type: dma_transaction_type,
    pub hw_desc: *mut c_void,
    pub idx: u16,
    pub async_tx: dma_async_tx_descriptor,
}

//
// This structure describes XOR descriptor size 64bytes. The
// mv_phy_src_idx() macro must be used when indexing the values of the
// phy_src_addr[] array. This is due to the fact that the 'descriptor
// swap' feature, used on big endian systems, swaps descriptors data
// within blocks of 8 bytes. So two consecutive values of the
// phy_src_addr[] array are actually swapped in big-endian, which
// explains the different mv_phy_src_idx() implementation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_xor_desc {
    pub /: *mut *mut u32 status; / descriptor execution status,
    pub /: *mut *mut u32 crc32_result; / result of CRC-32 calculation,
    pub /: *mut *mut u32 desc_command; / type of operation to be carried out,
    pub /: *mut *mut u32 phy_next_desc; / next descriptor address pointer,
    pub /: *mut *mut u32 byte_count; / size of src/dst blocks in bytes,
    pub /: *mut *mut u32 phy_dest_addr; / destination block address,
    pub /: *mut *mut u32 phy_src_addr[8]; / source block addresses,
    pub reserved0: u32,
    pub reserved1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_xor_desc {
    pub /: *mut *mut u32 crc32_result; / result of CRC-32 calculation,
    pub /: *mut *mut u32 status; / descriptor execution status,
    pub /: *mut *mut u32 phy_next_desc; / next descriptor address pointer,
    pub /: *mut *mut u32 desc_command; / type of operation to be carried out,
    pub /: *mut *mut u32 phy_dest_addr; / destination block address,
    pub /: *mut *mut u32 byte_count; / size of src/dst blocks in bytes,
    pub /: *mut *mut u32 phy_src_addr[8]; / source block addresses,
    pub reserved1: u32,
    pub reserved0: u32,
}

