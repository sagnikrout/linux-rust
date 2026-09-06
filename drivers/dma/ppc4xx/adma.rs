//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ppc4xx/adma.h
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
// 2006-2009 (C) DENX Software Engineering.
//
// Author: Yuri Tikhonov <yur@emcraft.com>
//

// Default polynomial (for 440SP is only available)
pub const PPC440SPE_DEFAULT_POLY: c_uint = 0x4d;

pub const PPC440SPE_ADMA_WATCHDOG_MSEC: c_int = 3;
pub const PPC440SPE_ADMA_THRESHOLD: c_int = 1;
pub const PPC440SPE_DMA0_ID: c_int = 0;
pub const PPC440SPE_DMA1_ID: c_int = 1;
pub const PPC440SPE_XOR_ID: c_int = 2;
pub const PPC440SPE_ADMA_DMA_MAX_BYTE_COUNT: c_uint = 0xFFFFFFUL;
// this is the XOR_CBBCR width

pub const PPC440SPE_RXOR_RUN: c_int = 0;
pub const MQ0_CF2H_RXOR_BS_MASK: c_uint = 0x1FF;

//
// struct ppc440spe_adma_device - internal representation of an ADMA device
// @dev: device
// @dma_reg: base for DMAx register access
// @xor_reg: base for XOR register access
// @i2o_reg: base for I2O register access
// @id: HW ADMA Device selector
// @dma_desc_pool_virt: base of DMA descriptor region (CPU address)
// @dma_desc_pool: base of DMA descriptor region (DMA address)
// @pool_size: size of the pool
// @irq: DMAx or XOR irq number
// @err_irq: DMAx error irq number
// @common: embedded struct dma_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc440spe_adma_device {
    pub dev: *mut device,
    pub dma_reg: *mut dma_regs __iomem,
    pub xor_reg: *mut xor_regs __iomem,
    pub i2o_reg: *mut i2o_regs __iomem,
    pub id: c_int,
    pub dma_desc_pool_virt: *mut c_void,
    pub dma_desc_pool: dma_addr_t,
    pub pool_size: usize,
    pub irq: c_int,
    pub err_irq: c_int,
    pub common: dma_device,
}

//
// struct ppc440spe_adma_chan - internal representation of an ADMA channel
// @lock: serializes enqueue/dequeue operations to the slot pool
// @device: parent device
// @chain: device chain view of the descriptors
// @common: common dmaengine channel object members
// @all_slots: complete domain of slots usable by the channel
// @pending: allows batching of hardware operations
// @slots_allocated: records the actual size of the descriptor slot pool
// @hw_chain_inited: h/w descriptor chain initialization flag
// @irq_tasklet: bottom half where ppc440spe_adma_slot_cleanup runs
// @needs_unmap: if buffers should not be unmapped upon final processing
// @pdest_page: P destination page for async validate operation
// @qdest_page: Q destination page for async validate operation
// @pdest: P dma addr for async validate operation
// @qdest: Q dma addr for async validate operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc440spe_adma_chan {
    pub lock: spinlock_t,
    pub device: *mut ppc440spe_adma_device,
    pub chain: list_head,
    pub common: dma_chan,
    pub all_slots: list_head,
    pub last_used: *mut ppc440spe_adma_desc_slot,
    pub pending: c_int,
    pub slots_allocated: c_int,
    pub hw_chain_inited: c_int,
    pub irq_tasklet: tasklet_struct,
    pub needs_unmap: u8,
    pub pdest_page: *mut page,
    pub qdest_page: *mut page,
    pub pdest: dma_addr_t,
    pub qdest: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc440spe_rxor {
    pub addrl: u32,
    pub addrh: u32,
    pub len: c_int,
    pub xor_count: c_int,
    pub addr_count: c_int,
    pub desc_count: c_int,
    pub state: c_int,
}

//
// struct ppc440spe_adma_desc_slot - PPC440SPE-ADMA software descriptor
// @phys: hardware address of the hardware descriptor chain
// @group_head: first operation in a transaction
// @hw_next: pointer to the next descriptor in chain
// @async_tx: support for the async_tx api
// @slot_node: node on the iop_adma_chan.all_slots list
// @chain_node: node on the op_adma_chan.chain list
// @group_list: list of slots that make up a multi-descriptor transaction
// for example transfer lengths larger than the supported hw max
// @unmap_len: transaction bytecount
// @hw_desc: virtual address of the hardware descriptor chain
// @stride: currently chained or not
// @idx: pool index
// @slot_cnt: total slots used in an transaction (group of operations)
// @src_cnt: number of sources set in this descriptor
// @dst_cnt: number of destinations set in the descriptor
// @slots_per_op: number of slots per operation
// @descs_per_op: number of slot per P/Q operation see comment
// for ppc440spe_prep_dma_pqxor function
// @flags: desc state/type
// @reverse_flags: 1 if a corresponding rxor address uses reversed address order
// @xor_check_result: result of zero sum
// @crc32_result: result crc calculation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc440spe_adma_desc_slot {
    pub phys: dma_addr_t,
    pub group_head: *mut ppc440spe_adma_desc_slot,
    pub hw_next: *mut ppc440spe_adma_desc_slot,
    pub async_tx: dma_async_tx_descriptor,
    pub slot_node: list_head,
    pub /: *mut *mut list_head chain_node; / node in channel ops list,
    pub /: *mut *mut list_head group_list; / list,
    pub unmap_len: c_uint,
    pub hw_desc: *mut c_void,
    pub stride: u16,
    pub idx: u16,
    pub slot_cnt: u16,
    pub src_cnt: u8,
    pub dst_cnt: u8,
    pub slots_per_op: u8,
    pub descs_per_op: u8,
    pub flags: c_ulong,
    pub reverse_flags: [c_ulong; 8],
pub const PPC440SPE_DESC_PCHECK: c_int = 13;
pub const PPC440SPE_DESC_QCHECK: c_int = 14;
pub const PPC440SPE_DESC_RXOR_MSK: c_uint = 0x3;
    pub rxor_cursor: ppc440spe_rxor,
    pub xor_check_result: *mut u32,
    pub crc32_result: *mut u32,
}
