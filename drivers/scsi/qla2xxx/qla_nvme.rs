//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_nvme.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2017 QLogic Corporation
//

pub const MIN_NVME_HW_QUEUES: c_int = 1;
pub const DEF_NVME_HW_QUEUES: c_int = 8;
pub const NVME_ATIO_CMD_OFF: c_int = 32;

pub const Q2T_NVME_NUM_TAGS: c_int = 2048;
pub const QLA_MAX_FC_SEGMENTS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_private {
    pub sp: *mut srb,
    pub fd: *mut nvmefc_ls_req,
    pub ls_work: work_struct,
    pub abort_work: work_struct,
    pub comp_status: c_int,
    pub cmd_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_nvme_rport {
    pub fcport: *mut fc_port,
    pub uctx: *mut qla_nvme_unsol_ctx,
}

pub const COMMAND_NVME: c_uint = 0x88            /* Command Type FC-NVMe IOCB */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_nvme {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 nvme_rsp_dsd_len; / NVMe RSP DSD length,
    pub rsvd: u64,
    pub /: *mut *mut __le16 control_flags; / Control Flags,

    pub /: *mut *mut __le16 nvme_cmnd_dseg_len; / Data segment length.,
    pub /: *mut *mut __le64 nvme_cmnd_dseg_address __packed;/ Data segment address.,
    pub /: *mut *mut __le64 nvme_rsp_dseg_address __packed; / Data segment address.,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub vp_index: u8,
    pub nvme_dsd: dsd64,
}

pub const PURLS_MSLEEP_INTERVAL: c_int = 1;
pub const PURLS_RETRY_COUNT: c_int = 5;
pub const PT_LS4_REQUEST: c_uint = 0x89	/* Link Service pass-through IOCB (request) */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_ls4_request {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_define: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub status: __le16,
    pub nport_handle: __le16,
    pub tx_dseg_count: __le16,
    pub vp_index: u8,
    pub rsvd: u8,
    pub timeout: __le16,
    pub control_flags: __le16,
pub const CF_LS4_SHIFT: c_int = 13;
pub const CF_LS4_ORIGINATOR: c_int = 0;
pub const CF_LS4_RESPONDER: c_int = 1;
pub const CF_LS4_RESPONDER_TERM: c_int = 2;
    pub rx_dseg_count: __le16,
    pub rsvd2: __le16,
    pub exchange_address: __le32,
    pub rsvd3: __le32,
    pub rx_byte_count: __le32,
    pub tx_byte_count: __le32,
    pub dsd: [dsd64; 2],
}

pub const PT_LS4_UNSOL: c_uint = 0x56	/* pass-up unsolicited rec FC-NVMe request */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_ls4_rx_unsol {
    pub entry_type: u8,
    pub entry_count: u8,
    pub rsvd0: __le16,
    pub rsvd1: __le16,
    pub vp_index: u8,
    pub rsvd2: u8,
    pub rsvd3: __le16,
    pub nport_handle: __le16,
    pub frame_size: __le16,
    pub rsvd4: __le16,
    pub exchange_address: __le32,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub s_id: le_id_t,
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub desc0: __le32,
pub const PT_LS4_PAYLOAD_OFFSET: c_uint = 0x2c;
pub const PT_LS4_FIRST_PACKET_LEN: c_int = 20;
    pub payload: [__le32; 5],
}

//
// Global functions prototype in qla_nvme.c source file.
//
extern "C" {
    pub fn qla_nvme_register_hba(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla_nvme_register_remote(: *mut scsi_qla_host, : *mut fc_port) -> c_int;
}
extern "C" {
    pub fn qla_nvme_delete(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla24xx_async_gffid_sp_done(sp: *mut srb, _arg: c_int);
}
