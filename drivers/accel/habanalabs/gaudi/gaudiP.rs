//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/gaudi/gaudiP.h
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
// Copyright 2019-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const NUMBER_OF_EXT_HW_QUEUES: c_int = 8;

pub const NUMBER_OF_CPU_HW_QUEUES: c_int = 1;
pub const NUMBER_OF_INT_HW_QUEUES: c_int = 100;

// 10 NIC QMANs, DMA5 QMAN, TPC7 QMAN
pub const NUMBER_OF_COLLECTIVE_QUEUES: c_int = 12;
pub const NUMBER_OF_SOBS_IN_GRP: c_int = 11;
pub const GAUDI_STREAM_MASTER_ARR_SIZE: c_int = 8;

pub const TPC_ENABLED_MASK: c_uint = 0xFF;
pub const GAUDI_HBM_SIZE_32GB: c_uint = 0x800000000ull;
pub const GAUDI_HBM_DEVICES: c_int = 4;
pub const GAUDI_HBM_CHANNELS: c_int = 8;

pub const PCI_DMA_NUMBER_OF_CHNLS: c_int = 2;
pub const HBM_DMA_NUMBER_OF_CHNLS: c_int = 6;

pub const MME_NUMBER_OF_SLAVE_ENGINES: c_int = 2;

pub const QMAN_STREAMS: c_int = 4;
pub const PQ_FETCHER_CACHE_SIZE: c_int = 8;

pub const MONITOR_MAX_SOBS: c_int = 8;
// DRAM Memory Map
pub const CPU_FW_IMAGE_SIZE: c_uint = 0x10000000	/* 256MB */;
pub const MMU_PAGE_TABLES_SIZE: c_uint = 0x0BF00000	/* 191MB */;
pub const MMU_CACHE_MNG_SIZE: c_uint = 0x00100000	/* 1MB */;
pub const RESERVED: c_uint = 0x04000000	/* 64MB */;

pub const DRAM_BASE_ADDR_USER: c_uint = 0x20000000;

// Internal QMANs PQ sizes
pub const MME_QMAN_LENGTH: c_int = 1024;

pub const HBM_DMA_QMAN_LENGTH: c_int = 4096;

pub const TPC_QMAN_LENGTH: c_int = 1024;

pub const NIC_QMAN_LENGTH: c_int = 4096;

// Virtual address space
pub const VA_HOST_SPACE_START: c_uint = 0x1000000000000ull	/* 256TB */;
pub const VA_HOST_SPACE_END: c_uint = 0x3FF8000000000ull	/* 1PB - 512GB */;

pub const HW_CAP_NIC_SHIFT: c_int = 14;

pub const HW_CAP_TPC_SHIFT: c_int = 24;

pub const NUM_OF_MME_ENGINES: c_int = 2;
pub const NUM_OF_MME_SUB_ENGINES: c_int = 2;
pub const NUM_OF_TPC_ENGINES: c_int = 8;
pub const NUM_OF_DMA_ENGINES: c_int = 8;
pub const NUM_OF_QUEUES: c_int = 5;
pub const NUM_OF_STREAMS: c_int = 4;
pub const NUM_OF_FENCES: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi_dma_channels {
    GAUDI_PCI_DMA_1,
    GAUDI_PCI_DMA_2,
    GAUDI_HBM_DMA_1,
    GAUDI_HBM_DMA_2,
    GAUDI_HBM_DMA_3,
    GAUDI_HBM_DMA_4,
    GAUDI_HBM_DMA_5,
    GAUDI_HBM_DMA_6,
    GAUDI_DMA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi_tpc_mask {
    GAUDI_TPC_MASK_TPC0 = 0x01,
    GAUDI_TPC_MASK_TPC1 = 0x02,
    GAUDI_TPC_MASK_TPC2 = 0x04,
    GAUDI_TPC_MASK_TPC3 = 0x08,
    GAUDI_TPC_MASK_TPC4 = 0x10,
    GAUDI_TPC_MASK_TPC5 = 0x20,
    GAUDI_TPC_MASK_TPC6 = 0x40,
    GAUDI_TPC_MASK_TPC7 = 0x80,
    GAUDI_TPC_MASK_ALL = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi_nic_mask {
    GAUDI_NIC_MASK_NIC0 = 0x01,
    GAUDI_NIC_MASK_NIC1 = 0x02,
    GAUDI_NIC_MASK_NIC2 = 0x04,
    GAUDI_NIC_MASK_NIC3 = 0x08,
    GAUDI_NIC_MASK_NIC4 = 0x10,
    GAUDI_NIC_MASK_NIC5 = 0x20,
    GAUDI_NIC_MASK_NIC6 = 0x40,
    GAUDI_NIC_MASK_NIC7 = 0x80,
    GAUDI_NIC_MASK_NIC8 = 0x100,
    GAUDI_NIC_MASK_NIC9 = 0x200,
    GAUDI_NIC_MASK_ALL = 0x3FF
}

//
// struct gaudi_hw_sob_group - H/W SOB group info.
// @hdev: habanalabs device structure.
// @kref: refcount of this SOB group. group will reset once refcount is zero.
// @base_sob_id: base sob id of this SOB group.
// @queue_id: id of the queue that waits on this sob group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi_hw_sob_group {
    pub hdev: *mut hl_device,
    pub kref: kref,
    pub base_sob_id: u32,
    pub queue_id: u32,
}

//
// struct gaudi_collective_properties -
// holds all SOB groups and queues info reserved for the collective
// @hw_sob_group: H/W SOB groups.
// @next_sob_group_val: the next value to use for the currently used SOB group.
// @curr_sob_group_idx: the index of the currently used SOB group.
// @mstr_sob_mask: pre-defined masks for collective master monitors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi_collective_properties {
    pub hw_sob_group: [gaudi_hw_sob_group; NUM_SOB_GROUPS],
    pub next_sob_group_val: [u16; QMAN_STREAMS],
    pub curr_sob_group_idx: [u8; QMAN_STREAMS],
    pub mstr_sob_mask: [u8; HL_COLLECTIVE_RSVD_MSTR_MONS],
}

//
// struct gaudi_internal_qman_info - Internal QMAN information.
// @pq_kernel_addr: Kernel address of the PQ memory area in the host.
// @pq_dma_addr: DMA address of the PQ memory area in the host.
// @pq_size: Size of allocated host memory for PQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi_internal_qman_info {
    pub pq_kernel_addr: *mut c_void,
    pub pq_dma_addr: dma_addr_t,
    pub pq_size: usize,
}

//
// struct gaudi_device - ASIC specific manage structure.
// @cpucp_info_get: get information on device from CPU-CP
// @hw_queues_lock: protects the H/W queues from concurrent access.
// @internal_qmans: Internal QMANs information. The array size is larger than
// the actual number of internal queues because they are not in
// consecutive order.
// @hbm_bar_cur_addr: current address of HBM PCI bar.
// @events: array that holds all event id's
// @events_stat: array that holds histogram of all received events.
// @events_stat_aggregate: same as events_stat but doesn't get cleared on reset
// @hw_cap_initialized: This field contains a bit per H/W engine. When that
// engine is initialized, that bit is set by the driver to
// signal we can use this engine in later code paths.
// Each bit is cleared upon reset of its corresponding H/W
// engine.
// @mmu_cache_inv_pi: PI for MMU cache invalidation flow. The H/W expects an
// 8-bit value so use u8.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi_device {
    pub hdev): *mut *mut int (cpucp_info_get)(struct hl_device,
// TODO: remove hw_queues_lock after moving to scheduler code
    pub hw_queues_lock: spinlock_t,
    pub internal_qmans: [gaudi_internal_qman_info; GAUDI_QUEUE_ID_SIZE],
    pub collective_props: gaudi_collective_properties,
    pub hbm_bar_cur_addr: u64,
    pub events: [u32; GAUDI_EVENT_SIZE],
    pub events_stat: [u32; GAUDI_EVENT_SIZE],
    pub events_stat_aggregate: [u32; GAUDI_EVENT_SIZE],
    pub hw_cap_initialized: u32,
    pub mmu_cache_inv_pi: u8,
}

extern "C" {
    pub fn gaudi_init_security(hdev: *mut hl_device);
}
extern "C" {
    pub fn gaudi_ack_protection_bits_errors(hdev: *mut hl_device);
}
extern "C" {
    pub fn gaudi_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn gaudi_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn gaudi_mmu_prepare_reg(hdev: *mut hl_device, reg: u64, asid: u32);
}
