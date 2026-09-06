//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_iscsi.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

pub const ISCSI_MAX_SESS_PER_HBA: c_int = 4096;
pub const DEF_KA_TIMEOUT: c_int = 7200000;
pub const DEF_KA_INTERVAL: c_int = 10000;
pub const DEF_KA_MAX_PROBE_COUNT: c_int = 10;
pub const DEF_TOS: c_int = 0;
pub const DEF_TTL: c_uint = 0xfe;
pub const DEF_SND_SEQ_SCALE: c_int = 0;
pub const DEF_RCV_BUF: c_uint = 0xffff;
pub const DEF_SND_BUF: c_uint = 0xffff;
pub const DEF_SEED: c_int = 0;
pub const DEF_MAX_RT_TIME: c_int = 8000;
pub const DEF_MAX_DA_COUNT: c_int = 2;
pub const DEF_SWS_TIMER: c_int = 1000;
pub const DEF_MAX_CWND: c_int = 2;
pub const DEF_PATH_MTU: c_int = 1500;
pub const DEF_MSS: c_int = 1460;
pub const DEF_LL2_MTU: c_int = 1560;
pub const JUMBO_MTU: c_int = 9000;

pub const IPV4_HDR_LEN: c_int = 20;
pub const IPV6_HDR_LEN: c_int = 40;
pub const TCP_HDR_LEN: c_int = 20;
pub const TCP_OPTION_LEN: c_int = 12;
pub const VLAN_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_endpoint {
    pub qedi: *mut qedi_ctx,
    pub dst_addr: [u32; 4],
    pub src_addr: [u32; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub vlan_id: u16,
    pub pmtu: u16,
    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],
    pub ip_type: u8,
    pub state: c_int,
    pub ofld_wait: wait_queue_head_t,
    pub tcp_ofld_wait: wait_queue_head_t,
    pub iscsi_cid: u32,
// identifier of the connection from qed
    pub handle: u32,
    pub fw_cid: u32,
    pub p_doorbell: *mut void __iomem,
    pub db_data: iscsi_db_data,
// Send queue management
    pub sq: *mut iscsi_wqe,
    pub sq_dma: dma_addr_t,
    pub sq_prod_idx: u16,
    pub fw_sq_prod_idx: u16,
    pub sq_con_idx: u16,
    pub sq_mem_size: u32,
    pub sq_pbl: *mut c_void,
    pub sq_pbl_dma: dma_addr_t,
    pub sq_pbl_size: u32,
    pub conn: *mut qedi_conn,
    pub offload_work: work_struct,
}

pub const QEDI_SQ_WQES_MIN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_io_bdt {
    pub sge_tbl: *mut scsi_sge,
    pub sge_tbl_dma: dma_addr_t,
    pub sge_valid: u16,
}

//
// struct generic_pdu_resc - login pdu resource structure
//
// @req_buf:            driver buffer used to stage payload associated with
// the login request
// @req_dma_addr:       dma address for iscsi login request payload buffer
// @req_buf_size:       actual login request payload length
// @req_wr_ptr:         pointer into login request buffer when next data is
// to be written
// @resp_hdr:           iscsi header where iscsi login response header is to
// be recreated
// @resp_buf:           buffer to stage login response payload
// @resp_dma_addr:      login response payload buffer dma address
// @resp_buf_size:      login response paylod length
// @resp_wr_ptr:        pointer into login response buffer when next data is
// to be written
// @req_bd_tbl:         iscsi login request payload BD table
// @req_bd_dma:         login request BD table dma address
// @resp_bd_tbl:        iscsi login response payload BD table
// @resp_bd_dma:        login request BD table dma address
//
// following structure defines buffer info for generic pdus such as iSCSI Login,
// Logout and NOP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_pdu_resc {
    pub req_buf: *mut c_char,
    pub req_dma_addr: dma_addr_t,
    pub req_buf_size: u32,
    pub req_wr_ptr: *mut c_char,
    pub resp_hdr: iscsi_hdr,
    pub resp_buf: *mut c_char,
    pub resp_dma_addr: dma_addr_t,
    pub resp_buf_size: u32,
    pub resp_wr_ptr: *mut c_char,
    pub req_bd_tbl: *mut c_char,
    pub req_bd_dma: dma_addr_t,
    pub resp_bd_tbl: *mut c_char,
    pub resp_bd_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_conn {
    pub cls_conn: *mut iscsi_cls_conn,
    pub qedi: *mut qedi_ctx,
    pub ep: *mut qedi_endpoint,
    pub iscsi_ep: *mut iscsi_endpoint,
    pub active_cmd_list: list_head,
    pub /: *mut *mut spinlock_t list_lock; / internal conn lock,
    pub active_cmd_count: u32,
    pub cmd_cleanup_req: u32,
    pub cmd_cleanup_cmpl: core::sync::atomic::AtomicI32,
    pub iscsi_conn_id: u32,
    pub itt: c_int,
    pub abrt_conn: c_int,
pub const QEDI_CID_RESERVED: c_uint = 0x5AFF;
    pub fw_cid: u32,
//
// Buffer for login negotiation process
//
    pub gen_pdu: generic_pdu_resc,
    pub tmf_work_list: list_head,
    pub wait_queue: wait_queue_head_t,
    pub /: *mut *mut spinlock_t tmf_work_lock; / tmf work lock,
    pub ep_disconnect_starting: bool,
    pub fw_cleanup_works: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_cmd {
    pub io_cmd: list_head,
    pub io_cmd_in_list: bool,
    pub hdr: iscsi_hdr,
    pub conn: *mut qedi_conn,
    pub scsi_cmd: *mut scsi_cmnd,
    pub sg: *mut scatterlist,
    pub io_tbl: qedi_io_bdt,
    pub request: iscsi_task_context,
    pub sense_buffer: *mut c_uchar,
    pub sense_buffer_dma: dma_addr_t,
    pub task_id: u16,
// field populated for tmf work queue
    pub task: *mut iscsi_task,
    pub tmf_work: work_struct,
    pub state: c_int,
pub const CLEANUP_WAIT: c_int = 1;
pub const CLEANUP_RECV: c_int = 2;
pub const CLEANUP_WAIT_FAILED: c_int = 3;
pub const CLEANUP_NOT_REQUIRED: c_int = 4;
pub const LUN_RESET_RESPONSE_RECEIVED: c_int = 5;
pub const RESPONSE_RECEIVED: c_int = 6;
    pub type: c_int,
pub const TYPEIO: c_int = 1;
pub const TYPERESET: c_int = 2;
    pub list_tmf_work: *mut qedi_work_map,
// slowpath management
    pub use_slowpath: bool,
    pub tmf_resp_buf: *mut iscsi_tm_rsp,
    pub cqe_work: qedi_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_work_map {
    pub list: list_head,
    pub qedi_cmd: *mut qedi_cmd,
    pub ctask: *mut iscsi_task,
    pub rtid: c_int,
    pub state: c_int,
pub const QEDI_WORK_QUEUED: c_int = 1;
pub const QEDI_WORK_SCHEDULED: c_int = 2;
pub const QEDI_WORK_EXIT: c_int = 3;
    pub ptr_tmf_work: *mut work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_boot_target {
    pub ip_addr: [c_char; 64],
    pub iscsi_name: [c_char; 255],
    pub ipv6_en: u32,
}

