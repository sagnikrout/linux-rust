//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/storage_common.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//
// SCSI CONSTANTS
//

pub const SCSI_NUM_SGES_SLOW_SGL_THR: c_int = 8;

// SCSI op codes

// iSCSI Drv opaque
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_drv_opaque {
    pub reserved_zero: [__le16; 3],
    pub opaque: __le16,
}

// Scsi 2B/8B opaque union
#[repr(C)]
#[derive(Copy, Clone)]
pub union scsi_opaque {
    pub fcoe_opaque: regpair,
    pub iscsi_opaque: iscsi_drv_opaque,
}

// SCSI buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_bd {
    pub address: regpair,
    pub opaque: scsi_opaque,
}

// Scsi Drv BDQ struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_bdq_ram_drv_data {
    pub external_producer: __le16,
    pub reserved0: [__le16; 3],
}

// SCSI SGE entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_sge {
    pub sge_addr: regpair,
    pub sge_len: __le32,
    pub reserved: __le32,
}

// Cached SGEs section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_cached_sges {
    pub sge: [scsi_sge; 4],
}

// Scsi Drv CMDQ struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_drv_cmdq {
    pub cmdq_cons: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

// Common SCSI init params passed by driver to FW in function init ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_init_func_params {
    pub num_tasks: __le16,
    pub log_page_size: u8,
    pub log_page_size_conn: u8,
    pub debug_mode: u8,
    pub reserved2: [u8; 11],
}

// SCSI RQ/CQ/CMDQ firmware function init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_init_func_queues {
    pub glbl_q_params_addr: regpair,
    pub rq_buffer_size: __le16,
    pub cq_num_entries: __le16,
    pub cmdq_num_entries: __le16,
    pub bdq_resource_id: u8,
    pub q_validity: u8,
pub const SCSI_INIT_FUNC_QUEUES_RQ_VALID_MASK: c_uint = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_RQ_VALID_SHIFT: c_int = 0;
pub const SCSI_INIT_FUNC_QUEUES_IMM_DATA_VALID_MASK: c_uint = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_IMM_DATA_VALID_SHIFT: c_int = 1;
pub const SCSI_INIT_FUNC_QUEUES_CMD_VALID_MASK: c_uint = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_CMD_VALID_SHIFT: c_int = 2;
pub const SCSI_INIT_FUNC_QUEUES_TQ_VALID_MASK: c_uint = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_TQ_VALID_SHIFT: c_int = 3;
pub const SCSI_INIT_FUNC_QUEUES_SOC_EN_MASK: c_uint = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_SOC_EN_SHIFT: c_int = 4;
pub const SCSI_INIT_FUNC_QUEUES_SOC_NUM_OF_BLOCKS_LOG_MASK: c_uint = 0x7;
pub const SCSI_INIT_FUNC_QUEUES_SOC_NUM_OF_BLOCKS_LOG_SHIFT: c_int = 5;
    pub cq_cmdq_sb_num_arr: [__le16; SCSI_MAX_NUM_OF_CMDQS],
    pub num_queues: u8,
    pub queue_relative_offset: u8,
    pub cq_sb_pi: u8,
    pub cmdq_sb_pi: u8,
    pub bdq_pbl_num_entries: [u8; BDQ_NUM_IDS],
    pub reserved1: u8,
    pub bdq_pbl_base_address: [regpair; BDQ_NUM_IDS],
    pub bdq_xoff_threshold: [__le16; BDQ_NUM_IDS],
    pub cmdq_xoff_threshold: __le16,
    pub bdq_xon_threshold: [__le16; BDQ_NUM_IDS],
    pub cmdq_xon_threshold: __le16,
}

// Scsi Drv BDQ Data struct (2 BDQ IDs: 0 - RQ, 1 - Immediate Data)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_ram_per_bdq_resource_drv_data {
    pub drv_data_per_bdq_id: [scsi_bdq_ram_drv_data; BDQ_NUM_IDS],
}

// SCSI SGL types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_sgl_mode {
    SCSI_TX_SLOW_SGL,
    SCSI_FAST_SGL,
    MAX_SCSI_SGL_MODE
}

// SCSI SGL parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_sgl_params {
    pub sgl_addr: regpair,
    pub sgl_total_length: __le32,
    pub sge_offset: __le32,
    pub sgl_num_sges: __le16,
    pub sgl_index: u8,
    pub reserved: u8,
}

// SCSI terminate connection params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_terminate_extra_params {
    pub unsolicited_cq_count: __le16,
    pub cmdq_count: __le16,
    pub reserved: [u8; 4],
}

// SCSI Task Queue Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_tqe {
    pub itid: __le16,
}
