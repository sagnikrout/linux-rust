//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/hal.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const HAL_LINK_DESC_ALIGN: c_int = 128;
pub const HAL_NUM_MPDUS_PER_LINK_DESC: c_int = 6;
pub const HAL_NUM_TX_MSDUS_PER_LINK_DESC: c_int = 7;
pub const HAL_NUM_RX_MSDUS_PER_LINK_DESC: c_int = 6;
pub const HAL_NUM_MPDU_LINKS_PER_QUEUE_DESC: c_int = 12;
pub const HAL_MAX_AVAIL_BLK_RES: c_int = 3;
pub const HAL_RING_BASE_ALIGN: c_int = 8;
pub const HAL_WBM_IDLE_SCATTER_BUF_SIZE_MAX: c_int = 32704;
// TODO: Check with hw team on the supported scatter buf size
pub const HAL_WBM_IDLE_SCATTER_NEXT_PTR_SIZE: c_int = 8;

pub const HAL_DSCP_TID_MAP_TBL_NUM_ENTRIES_MAX: c_int = 48;
pub const HAL_DSCP_TID_TBL_SIZE: c_int = 24;
// calculate the register address from bar0 of shadow register x

pub const HAL_SHADOW_NUM_REGS: c_int = 36;
pub const HAL_HP_OFFSET_IN_REG_START: c_int = 1;
pub const HAL_OFFSET_FROM_HP_TO_TP: c_int = 4;

// WCSS Relative address
pub const HAL_SEQ_WCSS_UMAC_OFFSET: c_uint = 0x00a00000;
pub const HAL_SEQ_WCSS_UMAC_REO_REG: c_uint = 0x00a38000;
pub const HAL_SEQ_WCSS_UMAC_TCL_REG: c_uint = 0x00a44000;

pub const HAL_SEQ_WCSS_UMAC_WBM_REG: c_uint = 0x00a34000;
pub const HAL_CE_WFSS_CE_REG_BASE: c_uint = 0x01b80000;
pub const HAL_WLAON_REG_BASE: c_uint = 0x01f80000;
// SW2TCL(x) R0 ring configuration address
pub const HAL_TCL1_RING_CMN_CTRL_REG: c_uint = 0x00000014;
pub const HAL_TCL1_RING_DSCP_TID_MAP: c_uint = 0x0000002c;

// SW2TCL(x) R2 ring pointers (head/tail) address
pub const HAL_TCL1_RING_HP: c_uint = 0x00002000;
pub const HAL_TCL1_RING_TP: c_uint = 0x00002004;
pub const HAL_TCL2_RING_HP: c_uint = 0x00002008;
pub const HAL_TCL_RING_HP: c_uint = 0x00002018;

// TCL STATUS ring address

pub const HAL_TCL_STATUS_RING_HP: c_uint = 0x00002030;
// REO2SW(x) R0 ring configuration address
pub const HAL_REO1_GEN_ENABLE: c_uint = 0x00000000;
pub const HAL_REO1_DEST_RING_CTRL_IX_0: c_uint = 0x00000004;
pub const HAL_REO1_DEST_RING_CTRL_IX_1: c_uint = 0x00000008;
pub const HAL_REO1_DEST_RING_CTRL_IX_2: c_uint = 0x0000000c;
pub const HAL_REO1_DEST_RING_CTRL_IX_3: c_uint = 0x00000010;

// REO2SW(x) R2 ring pointers (head/tail) address

// REO2TCL R0 ring configuration address

// REO2TCL R2 ring pointer (head/tail) address

// REO CMD R0 address

// REO CMD R2 address

// SW2REO R0 address

// SW2REO R2 address

// CE ring R0 address
pub const HAL_CE_DST_RING_BASE_LSB: c_uint = 0x00000000;
pub const HAL_CE_DST_STATUS_RING_BASE_LSB: c_uint = 0x00000058;
pub const HAL_CE_DST_RING_CTRL: c_uint = 0x000000b0;
// CE ring R2 address
pub const HAL_CE_DST_RING_HP: c_uint = 0x00000400;
pub const HAL_CE_DST_STATUS_RING_HP: c_uint = 0x00000408;
// REO status address

// WBM Idle R0 address

pub const HAL_WBM_R0_IDLE_LIST_CONTROL_ADDR: c_uint = 0x00000048;
pub const HAL_WBM_R0_IDLE_LIST_SIZE_ADDR: c_uint = 0x0000004c;
pub const HAL_WBM_SCATTERED_RING_BASE_LSB: c_uint = 0x00000058;
pub const HAL_WBM_SCATTERED_RING_BASE_MSB: c_uint = 0x0000005c;
pub const HAL_WBM_SCATTERED_DESC_PTR_HEAD_INFO_IX0: c_uint = 0x00000068;
pub const HAL_WBM_SCATTERED_DESC_PTR_HEAD_INFO_IX1: c_uint = 0x0000006c;
pub const HAL_WBM_SCATTERED_DESC_PTR_TAIL_INFO_IX0: c_uint = 0x00000078;
pub const HAL_WBM_SCATTERED_DESC_PTR_TAIL_INFO_IX1: c_uint = 0x0000007c;
pub const HAL_WBM_SCATTERED_DESC_PTR_HP_ADDR: c_uint = 0x00000084;
// WBM Idle R2 address
pub const HAL_WBM_IDLE_LINK_RING_HP: c_uint = 0x000030b0;
// SW2WBM R0 release address

// SW2WBM R2 release address
pub const HAL_WBM_RELEASE_RING_HP: c_uint = 0x00003018;
// WBM2SW R0 release address

// WBM2SW R2 release address
pub const HAL_WBM0_RELEASE_RING_HP: c_uint = 0x000030c0;
pub const HAL_WBM1_RELEASE_RING_HP: c_uint = 0x000030c8;
// TCL ring field mask and offset

// REO ring field mask and offset

// CE ring bit field mask and shift

pub const HAL_ADDR_LSB_REG_MASK: c_uint = 0xffffffff;
pub const HAL_ADDR_MSB_REG_SHIFT: c_int = 32;
// WBM ring bit field mask and shift

pub const BASE_ADDR_MATCH_TAG_VAL: c_uint = 0x5;
pub const HAL_REO_REO2SW1_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_REO_REO2TCL_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_REO_SW2REO_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_REO_CMD_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_REO_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_SW2TCL1_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_SW2TCL1_CMD_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_TCL_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_SRC_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_DST_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_CE_DST_STATUS_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_WBM_IDLE_LINK_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_SW2WBM_RELEASE_RING_BASE_MSB_RING_SIZE: c_uint = 0x0000ffff;
pub const HAL_WBM2SW_RELEASE_RING_BASE_MSB_RING_SIZE: c_uint = 0x000fffff;
pub const HAL_RXDMA_RING_MAX_SIZE: c_uint = 0x0000ffff;
// IPQ5018 ce registers
pub const HAL_IPQ5018_CE_WFSS_REG_BASE: c_uint = 0x08400000;
pub const HAL_IPQ5018_CE_SIZE: c_uint = 0x200000;
// Add any other errors here and return them in
// ath11k_hal_rx_desc_get_err().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_srng_ring_id {
    HAL_SRNG_RING_ID_REO2SW1 = 0,
    HAL_SRNG_RING_ID_REO2SW2,
    HAL_SRNG_RING_ID_REO2SW3,
    HAL_SRNG_RING_ID_REO2SW4,
    HAL_SRNG_RING_ID_REO2TCL,
    HAL_SRNG_RING_ID_SW2REO,

    HAL_SRNG_RING_ID_REO_CMD = 8,
    HAL_SRNG_RING_ID_REO_STATUS,

    HAL_SRNG_RING_ID_SW2TCL1 = 16,
    HAL_SRNG_RING_ID_SW2TCL2,
    HAL_SRNG_RING_ID_SW2TCL3,
    HAL_SRNG_RING_ID_SW2TCL4,

    HAL_SRNG_RING_ID_SW2TCL_CMD = 24,
    HAL_SRNG_RING_ID_TCL_STATUS,

    HAL_SRNG_RING_ID_CE0_SRC = 32,
    HAL_SRNG_RING_ID_CE1_SRC,
    HAL_SRNG_RING_ID_CE2_SRC,
    HAL_SRNG_RING_ID_CE3_SRC,
    HAL_SRNG_RING_ID_CE4_SRC,
    HAL_SRNG_RING_ID_CE5_SRC,
    HAL_SRNG_RING_ID_CE6_SRC,
    HAL_SRNG_RING_ID_CE7_SRC,
    HAL_SRNG_RING_ID_CE8_SRC,
    HAL_SRNG_RING_ID_CE9_SRC,
    HAL_SRNG_RING_ID_CE10_SRC,
    HAL_SRNG_RING_ID_CE11_SRC,

    HAL_SRNG_RING_ID_CE0_DST = 56,
    HAL_SRNG_RING_ID_CE1_DST,
    HAL_SRNG_RING_ID_CE2_DST,
    HAL_SRNG_RING_ID_CE3_DST,
    HAL_SRNG_RING_ID_CE4_DST,
    HAL_SRNG_RING_ID_CE5_DST,
    HAL_SRNG_RING_ID_CE6_DST,
    HAL_SRNG_RING_ID_CE7_DST,
    HAL_SRNG_RING_ID_CE8_DST,
    HAL_SRNG_RING_ID_CE9_DST,
    HAL_SRNG_RING_ID_CE10_DST,
    HAL_SRNG_RING_ID_CE11_DST,

    HAL_SRNG_RING_ID_CE0_DST_STATUS = 80,
    HAL_SRNG_RING_ID_CE1_DST_STATUS,
    HAL_SRNG_RING_ID_CE2_DST_STATUS,
    HAL_SRNG_RING_ID_CE3_DST_STATUS,
    HAL_SRNG_RING_ID_CE4_DST_STATUS,
    HAL_SRNG_RING_ID_CE5_DST_STATUS,
    HAL_SRNG_RING_ID_CE6_DST_STATUS,
    HAL_SRNG_RING_ID_CE7_DST_STATUS,
    HAL_SRNG_RING_ID_CE8_DST_STATUS,
    HAL_SRNG_RING_ID_CE9_DST_STATUS,
    HAL_SRNG_RING_ID_CE10_DST_STATUS,
    HAL_SRNG_RING_ID_CE11_DST_STATUS,

    HAL_SRNG_RING_ID_WBM_IDLE_LINK = 104,
    HAL_SRNG_RING_ID_WBM_SW_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW0_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW1_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW2_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW3_RELEASE,
    HAL_SRNG_RING_ID_WBM2SW4_RELEASE,

    HAL_SRNG_RING_ID_UMAC_ID_END = 127,
    HAL_SRNG_RING_ID_LMAC1_ID_START,

    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA0_BUF = HAL_SRNG_RING_ID_LMAC1_ID_START,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA1_BUF,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA2_BUF,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA0_STATBUF,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA1_STATBUF,
    HAL_SRNG_RING_ID_WMAC1_RXDMA2SW0,
    HAL_SRNG_RING_ID_WMAC1_RXDMA2SW1,
    HAL_SRNG_RING_ID_WMAC1_SW2RXDMA1_DESC,
    HAL_SRNG_RING_ID_RXDMA_DIR_BUF,

    HAL_SRNG_RING_ID_LMAC1_ID_END = 143
}

// SRNG registers are split into two groups R0 and R2
pub const HAL_SRNG_REG_GRP_R0: c_int = 0;
pub const HAL_SRNG_REG_GRP_R2: c_int = 1;
pub const HAL_SRNG_NUM_REG_GRP: c_int = 2;
pub const HAL_SRNG_NUM_LMACS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_ring_type {
    HAL_REO_DST,
    HAL_REO_EXCEPTION,
    HAL_REO_REINJECT,
    HAL_REO_CMD,
    HAL_REO_STATUS,
    HAL_TCL_DATA,
    HAL_TCL_CMD,
    HAL_TCL_STATUS,
    HAL_CE_SRC,
    HAL_CE_DST,
    HAL_CE_DST_STATUS,
    HAL_WBM_IDLE_LINK,
    HAL_SW2WBM_RELEASE,
    HAL_WBM2SW_RELEASE,
    HAL_RXDMA_BUF,
    HAL_RXDMA_DST,
    HAL_RXDMA_MONITOR_BUF,
    HAL_RXDMA_MONITOR_STATUS,
    HAL_RXDMA_MONITOR_DST,
    HAL_RXDMA_MONITOR_DESC,
    HAL_RXDMA_DIR_BUF,
    HAL_MAX_RING_TYPES,
}

pub const HAL_RX_MAX_BA_WINDOW: c_int = 256;

//
// enum hal_reo_cmd_type: Enum for REO command type
// @HAL_REO_CMD_GET_QUEUE_STATS: Get REO queue status/stats
// @HAL_REO_CMD_FLUSH_QUEUE: Flush all frames in REO queue
// @HAL_REO_CMD_FLUSH_CACHE: Flush descriptor entries in the cache
// @HAL_REO_CMD_UNBLOCK_CACHE: Unblock a descriptor's address that was blocked
// earlier with a 'REO_FLUSH_CACHE' command
// @HAL_REO_CMD_FLUSH_TIMEOUT_LIST: Flush buffers/descriptors from timeout list
// @HAL_REO_CMD_UPDATE_RX_QUEUE: Update REO queue settings
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_cmd_type {
    HAL_REO_CMD_GET_QUEUE_STATS     = 0,
    HAL_REO_CMD_FLUSH_QUEUE         = 1,
    HAL_REO_CMD_FLUSH_CACHE         = 2,
    HAL_REO_CMD_UNBLOCK_CACHE       = 3,
    HAL_REO_CMD_FLUSH_TIMEOUT_LIST  = 4,
    HAL_REO_CMD_UPDATE_RX_QUEUE     = 5,
}

//
// enum hal_reo_cmd_status: Enum for execution status of REO command
// @HAL_REO_CMD_SUCCESS: Command has successfully executed
// @HAL_REO_CMD_BLOCKED: Command could not be executed as the queue
// or cache was blocked
// @HAL_REO_CMD_FAILED: Command execution failed, could be due to
// invalid queue desc
// @HAL_REO_CMD_RESOURCE_BLOCKED:
// @HAL_REO_CMD_DRAIN:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_cmd_status {
    HAL_REO_CMD_SUCCESS		= 0,
    HAL_REO_CMD_BLOCKED		= 1,
    HAL_REO_CMD_FAILED		= 2,
    HAL_REO_CMD_RESOURCE_BLOCKED	= 3,
    HAL_REO_CMD_DRAIN		= 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_wbm_idle_scatter_list {
    pub paddr: dma_addr_t,
    pub vaddr: *mut hal_wbm_link_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng_params {
    pub ring_base_paddr: dma_addr_t,
    pub ring_base_vaddr: *mut u32,
    pub num_entries: c_int,
    pub intr_batch_cntr_thres_entries: u32,
    pub intr_timer_thres_us: u32,
    pub flags: u32,
    pub max_buffer_len: u32,
    pub low_threshold: u32,
    pub msi_addr: dma_addr_t,
    pub msi_data: u32,
// Add more params as needed
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_srng_dir {
    HAL_SRNG_DIR_SRC,
    HAL_SRNG_DIR_DST
}

// srng flags
pub const HAL_SRNG_FLAGS_MSI_SWAP: c_uint = 0x00000008;
pub const HAL_SRNG_FLAGS_RING_PTR_SWAP: c_uint = 0x00000010;
pub const HAL_SRNG_FLAGS_DATA_TLV_SWAP: c_uint = 0x00000020;
pub const HAL_SRNG_FLAGS_LOW_THRESH_INTR_EN: c_uint = 0x00010000;
pub const HAL_SRNG_FLAGS_MSI_INTR: c_uint = 0x00020000;
pub const HAL_SRNG_FLAGS_CACHED: c_uint = 0x20000000;
pub const HAL_SRNG_FLAGS_LMAC_RING: c_uint = 0x80000000;
pub const HAL_SRNG_FLAGS_REMAP_CE_RING: c_uint = 0x10000000;

// Common SRNG ring structure for source and destination rings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng {
// Unique SRNG ring ID
    pub ring_id: u8,
// Ring initialization done
    pub initialized: u8,
// Interrupt/MSI value assigned to this ring
    pub irq: c_int,
// Physical base address of the ring
    pub ring_base_paddr: dma_addr_t,
// Virtual base address of the ring
    pub ring_base_vaddr: *mut u32,
// Number of entries in ring
    pub num_entries: u32,
// Ring size
    pub ring_size: u32,
// Ring size mask
    pub ring_size_mask: u32,
// Size of ring entry
    pub entry_size: u32,
// Interrupt timer threshold - in micro seconds
    pub intr_timer_thres_us: u32,
// Interrupt batch counter threshold - in number of ring entries
    pub intr_batch_cntr_thres_entries: u32,
// MSI Address
    pub msi_addr: dma_addr_t,
// MSI data
    pub msi_data: u32,
// Misc flags
    pub flags: u32,
// Lock for serializing ring index updates
    pub lock: spinlock_t,
// Start offset of SRNG register groups for this ring
// TBD: See if this is required - register address can be derived
// from ring ID
//
    pub hwreg_base: [u32; HAL_SRNG_NUM_REG_GRP],
    pub timestamp: u64,
// Source or Destination ring
    pub ring_dir: hal_srng_dir,
// SW tail pointer
    pub tp: u32,
// Shadow head pointer location to be updated by HW
    pub hp_addr: *mut volatile u32,
// Cached head pointer
    pub cached_hp: u32,
// Tail pointer location to be updated by SW - This
// will be a register address and need not be
// accessed through SW structure
//
    pub tp_addr: *mut u32,
// Current SW loop cnt
    pub loop_cnt: u32,
// max transfer size
    pub max_buffer_length: u16,
// head pointer at access end
    pub last_hp: u32,
    pub dst_ring: },
// SW head pointer
    pub hp: u32,
// SW reap head pointer
    pub reap_hp: u32,
// Shadow tail pointer location to be updated by HW
    pub tp_addr: *mut u32,
// Cached tail pointer
    pub cached_tp: u32,
// Head pointer location to be updated by SW - This
// will be a register address and need not be accessed
// through SW structure
//
    pub hp_addr: *mut u32,
// Low threshold - in number of ring entries
    pub low_threshold: u32,
// tail pointer at access end
    pub last_tp: u32,
    pub src_ring: },
    pub u: },
}

// Interrupt mitigation - Batch threshold in terms of number of frames
pub const HAL_SRNG_INT_BATCH_THRESHOLD_TX: c_int = 256;
pub const HAL_SRNG_INT_BATCH_THRESHOLD_RX: c_int = 128;
pub const HAL_SRNG_INT_BATCH_THRESHOLD_OTHER: c_int = 1;
// Interrupt mitigation - timer threshold in us
pub const HAL_SRNG_INT_TIMER_THRESHOLD_TX: c_int = 1000;
pub const HAL_SRNG_INT_TIMER_THRESHOLD_RX: c_int = 500;
pub const HAL_SRNG_INT_TIMER_THRESHOLD_OTHER: c_int = 256;
// HW SRNG configuration table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_srng_config {
    pub start_ring_id: c_int,
    pub max_rings: u16,
    pub entry_size: u16,
    pub reg_start: [u32; HAL_SRNG_NUM_REG_GRP],
    pub reg_size: [u16; HAL_SRNG_NUM_REG_GRP],
    pub lmac_ring: u8,
    pub ring_dir: hal_srng_dir,
    pub max_size: u32,
}

//
// enum hal_rx_buf_return_buf_manager - manager for returned rx buffers
//
// @HAL_RX_BUF_RBM_WBM_IDLE_BUF_LIST: Buffer returned to WBM idle buffer list
// @HAL_RX_BUF_RBM_WBM_IDLE_DESC_LIST: Descriptor returned to WBM idle
// descriptor list.
// @HAL_RX_BUF_RBM_FW_BM: Buffer returned to FW
// @HAL_RX_BUF_RBM_SW0_BM: For Tx completion -- returned to host
// @HAL_RX_BUF_RBM_SW1_BM: For Tx completion -- returned to host
// @HAL_RX_BUF_RBM_SW2_BM: For Tx completion -- returned to host
// @HAL_RX_BUF_RBM_SW3_BM: For Rx release -- returned to host
// @HAL_RX_BUF_RBM_SW4_BM: For Tx completion -- returned to host
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_rx_buf_return_buf_manager {
    HAL_RX_BUF_RBM_WBM_IDLE_BUF_LIST,
    HAL_RX_BUF_RBM_WBM_IDLE_DESC_LIST,
    HAL_RX_BUF_RBM_FW_BM,
    HAL_RX_BUF_RBM_SW0_BM,
    HAL_RX_BUF_RBM_SW1_BM,
    HAL_RX_BUF_RBM_SW2_BM,
    HAL_RX_BUF_RBM_SW3_BM,
    HAL_RX_BUF_RBM_SW4_BM,
}

pub const HAL_SRNG_DESC_LOOP_CNT: c_uint = 0xf0000000;

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO0_UPD_* fields

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO1_* fields

// Should be matching with HAL_REO_UPD_RX_QUEUE_INFO2_* fields

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hal_reo_cmd {
    pub addr_lo: u32,
    pub flag: u32,
    pub upd0: u32,
    pub upd1: u32,
    pub upd2: u32,
    pub pn: [u32; 4],
    pub rx_queue_num: u16,
    pub min_rel: u16,
    pub min_fwd: u16,
    pub addr_hi: u8,
    pub ac_list: u8,
    pub blocking_idx: u8,
    pub ba_window_size: u16,
    pub pn_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_pn_type {
    HAL_PN_TYPE_NONE,
    HAL_PN_TYPE_WPA,
    HAL_PN_TYPE_WAPI_EVEN,
    HAL_PN_TYPE_WAPI_UNEVEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_ce_desc {
    HAL_CE_DESC_SRC,
    HAL_CE_DESC_DST,
    HAL_CE_DESC_DST_STATUS,
}

pub const HAL_HASH_ROUTING_RING_TCL: c_int = 0;
pub const HAL_HASH_ROUTING_RING_SW1: c_int = 1;
pub const HAL_HASH_ROUTING_RING_SW2: c_int = 2;
pub const HAL_HASH_ROUTING_RING_SW3: c_int = 3;
pub const HAL_HASH_ROUTING_RING_SW4: c_int = 4;
pub const HAL_HASH_ROUTING_RING_REL: c_int = 5;
pub const HAL_HASH_ROUTING_RING_FW: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_header {
    pub cmd_num: u16,
    pub cmd_status: hal_reo_cmd_status,
    pub cmd_exe_time: u16,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_queue_stats {
    pub ssn: u16,
    pub curr_idx: u16,
    pub pn: [u32; 4],
    pub last_rx_queue_ts: u32,
    pub last_rx_dequeue_ts: u32,
    pub /: *mut *mut u32 rx_bitmap[8]; / Bitmap from 0-255,
    pub curr_mpdu_cnt: u32,
    pub curr_msdu_cnt: u32,
    pub fwd_due_to_bar_cnt: u16,
    pub dup_cnt: u16,
    pub frames_in_order_cnt: u32,
    pub num_mpdu_processed_cnt: u32,
    pub num_msdu_processed_cnt: u32,
    pub total_num_processed_byte_cnt: u32,
    pub late_rx_mpdu_cnt: u32,
    pub reorder_hole_cnt: u32,
    pub timeout_cnt: u8,
    pub bar_rx_cnt: u8,
    pub num_window_2k_jump_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_queue {
    pub err_detected: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_status_flush_cache_err_code {
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_SUCCESS,
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_IN_USE,
    HAL_REO_STATUS_FLUSH_CACHE_ERR_CODE_NOT_FOUND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_cache {
    pub err_detected: bool,
    pub err_code: hal_reo_status_flush_cache_err_code,
    pub cache_controller_flush_status_hit: bool,
    pub cache_controller_flush_status_desc_type: u8,
    pub cache_controller_flush_status_client_id: u8,
    pub cache_controller_flush_status_err: u8,
    pub cache_controller_flush_status_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_status_unblock_cache_type {
    HAL_REO_STATUS_UNBLOCK_BLOCKING_RESOURCE,
    HAL_REO_STATUS_UNBLOCK_ENTIRE_CACHE_USAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_unblock_cache {
    pub err_detected: bool,
    pub unblock_type: hal_reo_status_unblock_cache_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_flush_timeout_list {
    pub err_detected: bool,
    pub list_empty: bool,
    pub release_desc_cnt: u16,
    pub fwd_buf_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_reo_threshold_idx {
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER0,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER1,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER2,
    HAL_REO_THRESHOLD_IDX_DESC_COUNTER_SUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status_desc_thresh_reached {
    pub threshold_idx: hal_reo_threshold_idx,
    pub link_desc_counter0: u32,
    pub link_desc_counter1: u32,
    pub link_desc_counter2: u32,
    pub link_desc_counter_sum: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reo_status {
    pub uniform_hdr: hal_reo_status_header,
    pub loop_cnt: u8,
    pub queue_stats: hal_reo_status_queue_stats,
    pub flush_queue: hal_reo_status_flush_queue,
    pub flush_cache: hal_reo_status_flush_cache,
    pub unblock_cache: hal_reo_status_unblock_cache,
    pub timeout_list: hal_reo_status_flush_timeout_list,
    pub desc_thresh_reached: hal_reo_status_desc_thresh_reached,
    pub u: },
}

// HAL context to be used to access SRNG APIs (currently used by data path
// and transport (CE) modules)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hal {
// HAL internal state for all SRNG rings.
//
    pub srng_list: [hal_srng; HAL_SRNG_RING_ID_MAX],
// SRNG configuration table
    pub srng_config: *mut hal_srng_config,
// Remote pointer memory for HW/FW updates
    pub vaddr: *mut u32,
    pub paddr: dma_addr_t,
    pub rdp: },
// Shared memory for ring pointer updates from host to FW
    pub vaddr: *mut u32,
    pub paddr: dma_addr_t,
    pub wrp: },
// Available REO blocking resources bitmap
    pub avail_blk_resource: u8,
    pub current_blk_index: u8,
// shadow register configuration
    pub shadow_reg_addr: [u32; HAL_SHADOW_NUM_REGS],
    pub num_shadow_reg_configured: c_int,
    pub srng_key: [lock_class_key; HAL_SRNG_RING_ID_MAX],
}

extern "C" {
    pub fn ath11k_hal_reo_qdesc_size(ba_window_size: u32, tid: u8) -> u32;
}
extern "C" {
    pub fn ath11k_hal_ce_get_desc_size(type: hal_ce_desc) -> u32;
}
extern "C" {
    pub fn ath11k_hal_ce_dst_set_desc(buf: *mut c_void, paddr: dma_addr_t);
}
extern "C" {
    pub fn ath11k_hal_ce_dst_status_get_length(buf: *mut c_void) -> u32;
}
extern "C" {
    pub fn ath11k_hal_srng_get_entrysize(ab: *mut ath11k_base, ring_type: u32) -> c_int;
}
extern "C" {
    pub fn ath11k_hal_srng_get_max_entries(ab: *mut ath11k_base, ring_type: u32) -> c_int;
}
extern "C" {
    pub fn ath11k_hal_srng_access_end(ab: *mut ath11k_base, srng: *mut hal_srng);
}
extern "C" {
    pub fn ath11k_hal_srng_init(ath11k: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_hal_srng_deinit(ath11k: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_hal_srng_clear(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_hal_dump_srng_stats(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_hal_srng_shadow_config(ab: *mut ath11k_base);
}
