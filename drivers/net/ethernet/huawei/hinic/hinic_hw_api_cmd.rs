//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_api_cmd.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_API_CMD_PI_IDX_SHIFT: c_int = 0;
pub const HINIC_API_CMD_PI_IDX_MASK: c_uint = 0xFFFFFF;

pub const HINIC_API_CMD_CHAIN_REQ_RESTART_SHIFT: c_int = 1;
pub const HINIC_API_CMD_CHAIN_REQ_RESTART_MASK: c_uint = 0x1;

pub const HINIC_API_CMD_CHAIN_CTRL_RESTART_WB_STAT_SHIFT: c_int = 1;
pub const HINIC_API_CMD_CHAIN_CTRL_XOR_ERR_SHIFT: c_int = 2;
pub const HINIC_API_CMD_CHAIN_CTRL_AEQE_EN_SHIFT: c_int = 4;
pub const HINIC_API_CMD_CHAIN_CTRL_AEQ_ID_SHIFT: c_int = 8;
pub const HINIC_API_CMD_CHAIN_CTRL_XOR_CHK_EN_SHIFT: c_int = 28;
pub const HINIC_API_CMD_CHAIN_CTRL_CELL_SIZE_SHIFT: c_int = 30;
pub const HINIC_API_CMD_CHAIN_CTRL_RESTART_WB_STAT_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_CHAIN_CTRL_XOR_ERR_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_CHAIN_CTRL_AEQE_EN_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_CHAIN_CTRL_AEQ_ID_MASK: c_uint = 0x3;
pub const HINIC_API_CMD_CHAIN_CTRL_XOR_CHK_EN_MASK: c_uint = 0x3;
pub const HINIC_API_CMD_CHAIN_CTRL_CELL_SIZE_MASK: c_uint = 0x3;

pub const HINIC_API_CMD_CELL_CTRL_DATA_SZ_SHIFT: c_int = 0;
pub const HINIC_API_CMD_CELL_CTRL_RD_DMA_ATTR_SHIFT: c_int = 16;
pub const HINIC_API_CMD_CELL_CTRL_WR_DMA_ATTR_SHIFT: c_int = 24;
pub const HINIC_API_CMD_CELL_CTRL_XOR_CHKSUM_SHIFT: c_int = 56;
pub const HINIC_API_CMD_CELL_CTRL_DATA_SZ_MASK: c_uint = 0x3F;
pub const HINIC_API_CMD_CELL_CTRL_RD_DMA_ATTR_MASK: c_uint = 0x3F;
pub const HINIC_API_CMD_CELL_CTRL_WR_DMA_ATTR_MASK: c_uint = 0x3F;
pub const HINIC_API_CMD_CELL_CTRL_XOR_CHKSUM_MASK: c_uint = 0xFF;

pub const HINIC_API_CMD_DESC_API_TYPE_SHIFT: c_int = 0;
pub const HINIC_API_CMD_DESC_RD_WR_SHIFT: c_int = 1;
pub const HINIC_API_CMD_DESC_MGMT_BYPASS_SHIFT: c_int = 2;
pub const HINIC_API_CMD_DESC_DEST_SHIFT: c_int = 32;
pub const HINIC_API_CMD_DESC_SIZE_SHIFT: c_int = 40;
pub const HINIC_API_CMD_DESC_XOR_CHKSUM_SHIFT: c_int = 56;
pub const HINIC_API_CMD_DESC_API_TYPE_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_DESC_RD_WR_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_DESC_MGMT_BYPASS_MASK: c_uint = 0x1;
pub const HINIC_API_CMD_DESC_DEST_MASK: c_uint = 0x1F;
pub const HINIC_API_CMD_DESC_SIZE_MASK: c_uint = 0x7FF;
pub const HINIC_API_CMD_DESC_XOR_CHKSUM_MASK: c_uint = 0xFF;

pub const HINIC_API_CMD_STATUS_HEADER_CHAIN_ID_SHIFT: c_int = 16;
pub const HINIC_API_CMD_STATUS_HEADER_CHAIN_ID_MASK: c_uint = 0xFF;

pub const HINIC_API_CMD_STATUS_CONS_IDX_SHIFT: c_int = 0;
pub const HINIC_API_CMD_STATUS_FSM_SHIFT: c_int = 24;
pub const HINIC_API_CMD_STATUS_CHKSUM_ERR_SHIFT: c_int = 28;
pub const HINIC_API_CMD_STATUS_CPLD_ERR_SHIFT: c_int = 30;
pub const HINIC_API_CMD_STATUS_CONS_IDX_MASK: c_uint = 0xFFFFFF;
pub const HINIC_API_CMD_STATUS_FSM_MASK: c_uint = 0xFU;
pub const HINIC_API_CMD_STATUS_CHKSUM_ERR_MASK: c_uint = 0x3;
pub const HINIC_API_CMD_STATUS_CPLD_ERR_MASK: c_uint = 0x1U;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_api_cmd_chain_type {
    HINIC_API_CMD_WRITE_TO_MGMT_CPU = 2,

    HINIC_API_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_api_cmd_chain_attr {
    pub hwif: *mut hinic_hwif,
    pub chain_type: hinic_api_cmd_chain_type,
    pub num_cells: u32,
    pub cell_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_api_cmd_status {
    pub header: u64,
    pub status: u32,
    pub rsvd0: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub rsvd3: u64,
}

// HW struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_api_cmd_cell {
    pub ctrl: u64,
// address is 64 bit in HW struct
    pub next_cell_paddr: u64,
    pub desc: u64,
// HW struct
    pub hw_cmd_paddr: u64,
    pub write: },
    pub hw_wb_resp_paddr: u64,
    pub hw_cmd_paddr: u64,
    pub read: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_api_cmd_cell_ctxt {
    pub cell_paddr: dma_addr_t,
    pub cell_vaddr: *mut hinic_api_cmd_cell,
    pub api_cmd_paddr: dma_addr_t,
    pub api_cmd_vaddr: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_api_cmd_chain {
    pub hwif: *mut hinic_hwif,
    pub chain_type: hinic_api_cmd_chain_type,
    pub num_cells: u32,
    pub cell_size: u16,
// HW members in 24 bit format
    pub prod_idx: u32,
    pub cons_idx: u32,
    pub sem: semaphore,
    pub cell_ctxt: *mut hinic_api_cmd_cell_ctxt,
    pub wb_status_paddr: dma_addr_t,
    pub wb_status: *mut hinic_api_cmd_status,
    pub head_cell_paddr: dma_addr_t,
    pub head_node: *mut hinic_api_cmd_cell,
    pub curr_node: *mut hinic_api_cmd_cell,
}

extern "C" {
    pub fn hinic_api_cmd_free(chain: *mut hinic_api_cmd_chain);
}
