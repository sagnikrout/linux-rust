//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_cmdq.h
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

pub const HINIC_CMDQ_CTXT_CURR_WQE_PAGE_PFN_SHIFT: c_int = 0;
pub const HINIC_CMDQ_CTXT_EQ_ID_SHIFT: c_int = 56;
pub const HINIC_CMDQ_CTXT_CEQ_ARM_SHIFT: c_int = 61;
pub const HINIC_CMDQ_CTXT_CEQ_EN_SHIFT: c_int = 62;
pub const HINIC_CMDQ_CTXT_WRAPPED_SHIFT: c_int = 63;
pub const HINIC_CMDQ_CTXT_CURR_WQE_PAGE_PFN_MASK: c_uint = 0xFFFFFFFFFFFFF;
pub const HINIC_CMDQ_CTXT_EQ_ID_MASK: c_uint = 0x1F;
pub const HINIC_CMDQ_CTXT_CEQ_ARM_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_CTXT_CEQ_EN_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_CTXT_WRAPPED_MASK: c_uint = 0x1;

pub const HINIC_CMDQ_CTXT_WQ_BLOCK_PFN_SHIFT: c_int = 0;
pub const HINIC_CMDQ_CTXT_CI_SHIFT: c_int = 52;
pub const HINIC_CMDQ_CTXT_WQ_BLOCK_PFN_MASK: c_uint = 0xFFFFFFFFFFFFF;
pub const HINIC_CMDQ_CTXT_CI_MASK: c_uint = 0xFFF;

pub const HINIC_SAVED_DATA_ARM_SHIFT: c_int = 31;
pub const HINIC_SAVED_DATA_ARM_MASK: c_uint = 0x1;

pub const HINIC_CMDQ_DB_INFO_HI_PROD_IDX_SHIFT: c_int = 0;
pub const HINIC_CMDQ_DB_INFO_PATH_SHIFT: c_int = 23;
pub const HINIC_CMDQ_DB_INFO_CMDQ_TYPE_SHIFT: c_int = 24;
pub const HINIC_CMDQ_DB_INFO_DB_TYPE_SHIFT: c_int = 27;
pub const HINIC_CMDQ_DB_INFO_HI_PROD_IDX_MASK: c_uint = 0xFF;
pub const HINIC_CMDQ_DB_INFO_PATH_MASK: c_uint = 0x1;
pub const HINIC_CMDQ_DB_INFO_CMDQ_TYPE_MASK: c_uint = 0x7;
pub const HINIC_CMDQ_DB_INFO_DB_TYPE_MASK: c_uint = 0x1F;

pub const HINIC_CMDQ_BUF_SIZE: c_int = 2048;
pub const HINIC_CMDQ_BUF_HW_RSVD: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_cmdq_type {
    HINIC_CMDQ_SYNC,

    HINIC_MAX_CMDQ_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_set_arm_qtype {
    HINIC_SET_ARM_CMDQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_cmd_ack_type {
    HINIC_CMD_ACK_TYPE_CMDQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_buf {
    pub buf: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_arm_bit {
    pub q_type: u32,
    pub q_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_ctxt_info {
    pub curr_wqe_page_pfn: u64,
    pub wq_block_pfn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_ctxt {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub cmdq_type: u8,
    pub ppf_idx: u8,
    pub rsvd2: [u8; 4],
    pub ctxt_info: hinic_cmdq_ctxt_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq {
    pub hwdev: *mut hinic_hwdev,
    pub wq: *mut hinic_wq,
    pub cmdq_type: hinic_cmdq_type,
    pub wrapped: c_int,
// Lock for keeping the doorbell order
    pub cmdq_lock: spinlock_t,
    pub done: *mut completion,
    pub errcode: *mut c_int,
// doorbell area
    pub db_base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdqs {
    pub hwif: *mut hinic_hwif,
    pub cmdq_buf_pool: *mut dma_pool,
    pub saved_wqs: *mut hinic_wq,
    pub cmdq_pages: hinic_cmdq_pages,
    pub cmdq: [hinic_cmdq; HINIC_MAX_CMDQ_TYPES],
}

extern "C" {
    pub fn hinic_free_cmdqs(cmdqs: *mut hinic_cmdqs);
}
