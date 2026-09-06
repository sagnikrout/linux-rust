//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_cmdq.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const CMDQ_DEPTH: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_db {
    pub db_head: __le32,
    pub db_info: __le32,
}

// hw defined cmdq wqe header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_header {
    pub header_info: __le32,
    pub saved_data: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_lcmd_bufdesc {
    pub sge: hinic3_sge,
    pub rsvd2: __le64,
    pub rsvd3: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_status {
    pub status_info: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_ctrl {
    pub ctrl_info: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_direct_resp {
    pub val: __le64,
    pub rsvd: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_completion {
    pub sge: hinic3_sge,
    pub direct: cmdq_direct_resp,
    pub resp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_wqe_scmd {
    pub header: cmdq_header,
    pub rsvd3: __le64,
    pub status: cmdq_status,
    pub ctrl: cmdq_ctrl,
    pub completion: cmdq_completion,
    pub rsvd10: [__le32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_wqe_lcmd {
    pub header: cmdq_header,
    pub status: cmdq_status,
    pub ctrl: cmdq_ctrl,
    pub completion: cmdq_completion,
    pub buf_desc: cmdq_lcmd_bufdesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_wqe {
    pub wqe_scmd: cmdq_wqe_scmd,
    pub wqe_lcmd: cmdq_wqe_lcmd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_cmdq_type {
    HINIC3_CMDQ_SYNC      = 0,
    HINIC3_MAX_CMDQ_TYPES = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_cmdq_status {
    HINIC3_CMDQ_ENABLE = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_cmdq_cmd_type {
    HINIC3_CMD_TYPE_NONE,
    HINIC3_CMD_TYPE_SET_ARM,
    HINIC3_CMD_TYPE_DIRECT_RESP,
    HINIC3_CMD_TYPE_SGE_RESP,
    HINIC3_CMD_TYPE_FAKE_TIMEOUT,
    HINIC3_CMD_TYPE_TIMEOUT,
    HINIC3_CMD_TYPE_FORCE_STOP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cmd_buf {
    pub buf: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: __le16,
    pub ref_cnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cmd_buf_pair {
    pub in: *mut hinic3_cmd_buf,
    pub out: *mut hinic3_cmd_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cmdq_cmd_info {
    pub cmd_type: hinic3_cmdq_cmd_type,
    pub done: *mut completion,
    pub errcode: *mut c_int,
// completion code
    pub cmpt_code: *mut c_int,
    pub direct_resp: *mut __le64,
    pub cmdq_msg_id: u64,
    pub buf_in: *mut hinic3_cmd_buf,
    pub buf_out: *mut hinic3_cmd_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cmdq {
    pub wq: hinic3_wq,
    pub cmdq_type: hinic3_cmdq_type,
    pub wrapped: u8,
// synchronize command submission with completions via event queue
    pub cmdq_lock: spinlock_t,
    pub cmd_infos: *mut hinic3_cmdq_cmd_info,
    pub hwdev: *mut hinic3_hwdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cmdqs {
    pub hwdev: *mut hinic3_hwdev,
    pub cmdq: [hinic3_cmdq; HINIC3_MAX_CMDQ_TYPES],
    pub cmd_buf_pool: *mut dma_pool,
// doorbell area
    pub cmdqs_db_base: *mut u8 __iomem,
// When command queue uses multiple memory pages (1-level CLA), this
// block will hold aggregated indirection table for all command queues
// of cmdqs. Not used for small cmdq (0-level CLA).
//
    pub wq_block_paddr: dma_addr_t,
    pub wq_block_vaddr: *mut c_void,
    pub status: u32,
    pub disable_flag: u32,
    pub cmdq_num: u8,
}

extern "C" {
    pub fn hinic3_cmdqs_init(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_cmdqs_free(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_cmdq_ceq_handler(hwdev: *mut hinic3_hwdev, ceqe_data: __le32);
}
extern "C" {
    pub fn hinic3_cmdq_flush_sync_cmd(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_reinit_cmdq_ctxts(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_cmdq_idle(cmdq: *mut hinic3_cmdq) -> bool;
}
