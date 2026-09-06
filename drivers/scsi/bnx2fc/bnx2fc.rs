//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2fc/bnx2fc.h
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


// bnx2fc.h: QLogic Linux FCoE offload driver.
//
// Copyright (c) 2008-2013 Broadcom Corporation
// Copyright (c) 2014-2016 QLogic Corporation
// Copyright (c) 2016-2017 Cavium Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Bhanu Prakash Gollapudi (bprakash@broadcom.com)
//

pub const BCM_CHIP_LEN: c_int = 16;
pub const BNX2X_DOORBELL_PCI_BAR: c_int = 2;
pub const BNX2FC_MAX_BD_LEN: c_uint = 0xffff;
pub const BNX2FC_BD_SPLIT_SZ: c_uint = 0xffff;
pub const BNX2FC_MAX_BDS_PER_CMD: c_int = 255;
pub const BNX2FC_FW_MAX_BDS_PER_CMD: c_int = 255;
pub const BNX2FC_SQ_WQES_MAX: c_int = 256;

pub const BNX2FC_RQ_WQES_MAX: c_int = 16;

pub const BNX2FC_NUM_MAX_SESS: c_int = 1024;

pub const BNX2FC_MAX_NPIV: c_int = 256;
pub const BNX2FC_MIN_PAYLOAD: c_int = 256;
pub const BNX2FC_MAX_PAYLOAD: c_int = 2048;

pub const BNX2FC_MINI_JUMBO_MTU: c_int = 2500;
pub const BNX2FC_RQ_BUF_SZ: c_int = 256;

pub const BNX2X_DB_SHIFT: c_int = 3;
pub const BNX2FC_TASK_SIZE: c_int = 128;

pub const BNX2FC_MAX_ROWS_IN_HASH_TBL: c_int = 8;

pub const BNX2FC_MAX_SEQS: c_int = 255;
pub const BNX2FC_MAX_RETRY_CNT: c_int = 3;
pub const BNX2FC_MAX_RPORT_RETRY_CNT: c_int = 255;

pub const BNX2FC_MIN_XID: c_int = 0;
pub const FCOE_MAX_NUM_XIDS: c_uint = 0x2000;

pub const BNX2FC_MAX_LUN: c_uint = 0xFFFF;
pub const BNX2FC_MAX_FCP_TGT: c_int = 256;
pub const BNX2FC_MAX_CMD_LEN: c_int = 16;

pub const BNX2FC_WAIT_CNT: c_int = 1200;

pub const PORT_MAX: c_int = 2;
// FC FCP Status
pub const FC_GOOD: c_int = 0;
pub const BNX2FC_RNID_HBA: c_uint = 0x7;
pub const SRR_RETRY_COUNT: c_int = 5;
pub const REC_RETRY_COUNT: c_int = 1;
pub const BNX2FC_NUM_ERR_BITS: c_int = 63;
pub const BNX2FC_RELOGIN_WAIT_TIME: c_int = 200;
pub const BNX2FC_RELOGIN_WAIT_CNT: c_int = 10;

// bnx2fc driver uses only one instance of fcoe_percpu_s
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_percpu_s {
    pub iothread: *mut task_struct,
    pub work_list: list_head,
    pub fp_work_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_fw_stats {
    pub fc_crc_cnt: u64,
    pub fcoe_tx_pkt_cnt: u64,
    pub fcoe_rx_pkt_cnt: u64,
    pub fcoe_tx_byte_cnt: u64,
    pub fcoe_rx_byte_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_hba {
    pub list: list_head,
    pub cnic: *mut cnic_dev,
    pub pcidev: *mut pci_dev,
    pub phys_dev: *mut net_device,
    pub reg_with_cnic: c_ulong,
pub const BNX2FC_CNIC_REGISTERED: c_int = 1;
    pub cmd_mgr: *mut bnx2fc_cmd_mgr,
    pub hba_lock: spinlock_t,
    pub hba_mutex: mutex,
    pub hba_stats_mutex: mutex,
    pub adapter_state: c_ulong,
pub const ADAPTER_STATE_UP: c_int = 0;
pub const ADAPTER_STATE_GOING_DOWN: c_int = 1;
pub const ADAPTER_STATE_LINK_DOWN: c_int = 2;
pub const ADAPTER_STATE_READY: c_int = 3;
    pub flags: c_ulong,
pub const BNX2FC_FLAG_FW_INIT_DONE: c_int = 0;
pub const BNX2FC_FLAG_DESTROY_CMPL: c_int = 1;
    pub next_conn_id: u32,
// xid resources
    pub max_xid: u16,
    pub max_tasks: u32,
    pub max_outstanding_cmds: u32,
    pub elstm_xids: u32,
    pub task_ctx: *mut fcoe_task_ctx_entry,
    pub task_ctx_dma: *mut dma_addr_t,
    pub task_ctx_bd_tbl: *mut regpair,
    pub task_ctx_bd_dma: dma_addr_t,
    pub hash_tbl_segment_count: c_int,
    pub hash_tbl_segments: *mut c_void,
    pub hash_tbl_pbl: *mut c_void,
    pub hash_tbl_pbl_dma: dma_addr_t,
    pub t2_hash_tbl: *mut fcoe_t2_hash_table_entry,
    pub t2_hash_tbl_dma: dma_addr_t,
    pub t2_hash_tbl_ptr: *mut c_char,
    pub t2_hash_tbl_ptr_dma: dma_addr_t,
    pub dummy_buffer: *mut c_char,
    pub dummy_buf_dma: dma_addr_t,
// Active list of offloaded sessions
    pub tgt_ofld_list: *mut bnx2fc_rport,
// statistics
    pub bfw_stats: bnx2fc_fw_stats,
    pub prev_stats: fcoe_statistics_params,
    pub stats_buffer: *mut fcoe_statistics_params,
    pub stats_buf_dma: dma_addr_t,
    pub stat_req_done: completion,
    pub fcoe_cap: fcoe_capabilities,
// destroy handling
    pub destroy_timer: timer_list,
    pub destroy_wait: wait_queue_head_t,
// linkdown handling
    pub shutdown_wait: wait_queue_head_t,
    pub wait_for_link_down: c_int,
    pub num_ofld_sess: c_int,
    pub vports: list_head,
    pub chip_num: [c_char; BCM_CHIP_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_interface {
    pub list: list_head,
    pub if_flags: c_ulong,
pub const BNX2FC_CTLR_INIT_DONE: c_int = 0;
    pub hba: *mut bnx2fc_hba,
    pub netdev: *mut net_device,
    pub fcoe_packet_type: packet_type,
    pub fip_packet_type: packet_type,
    pub timer_work_queue: *mut workqueue_struct,
    pub kref: kref,
    pub vlan_enabled: u8,
    pub vlan_id: c_int,
    pub enabled: bool,
    pub tm_timeout: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_lport {
    pub list: list_head,
    pub lport: *mut fc_lport,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_cmd_mgr {
    pub hba: *mut bnx2fc_hba,
    pub next_idx: u16,
    pub free_list: *mut list_head,
    pub free_list_lock: *mut spinlock_t,
    pub io_bdt_pool: *mut io_bdt,
    pub cmds: *mut bnx2fc_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_rport {
    pub port: *mut fcoe_port,
    pub rport: *mut fc_rport,
    pub rdata: *mut fc_rport_priv,
    pub ctx_base: *mut void __iomem,
pub const DPM_TRIGER_TYPE: c_uint = 0x40;
    pub io_timeout: u32,
    pub fcoe_conn_id: u32,
    pub context_id: u32,
    pub sid: u32,
    pub dev_type: c_int,
    pub flags: c_ulong,
pub const BNX2FC_FLAG_SESSION_READY: c_uint = 0x1;
pub const BNX2FC_FLAG_OFFLOADED: c_uint = 0x2;
pub const BNX2FC_FLAG_DISABLED: c_uint = 0x3;
pub const BNX2FC_FLAG_DESTROYED: c_uint = 0x4;
pub const BNX2FC_FLAG_OFLD_REQ_CMPL: c_uint = 0x5;
pub const BNX2FC_FLAG_CTX_ALLOC_FAILURE: c_uint = 0x6;
pub const BNX2FC_FLAG_UPLD_REQ_COMPL: c_uint = 0x7;
pub const BNX2FC_FLAG_DISABLE_FAILED: c_uint = 0x9;
pub const BNX2FC_FLAG_ENABLED: c_uint = 0xa;
    pub src_addr: [u8; ETH_ALEN],
    pub max_sqes: u32,
    pub max_rqes: u32,
    pub max_cqes: u32,
    pub free_sqes: core::sync::atomic::AtomicI32,
    pub sq_db: b577xx_doorbell_set_prod,
    pub rx_db: b577xx_fcoe_rx_doorbell,
    pub sq: *mut fcoe_sqe,
    pub sq_dma: dma_addr_t,
    pub sq_prod_idx: u16,
    pub sq_curr_toggle_bit: u8,
    pub sq_mem_size: u32,
    pub cq: *mut fcoe_cqe,
    pub cq_dma: dma_addr_t,
    pub cq_cons_idx: u16,
    pub cq_curr_toggle_bit: u8,
    pub cq_mem_size: u32,
    pub rq: *mut c_void,
    pub rq_dma: dma_addr_t,
    pub rq_prod_idx: u32,
    pub rq_cons_idx: u32,
    pub rq_mem_size: u32,
    pub rq_pbl: *mut c_void,
    pub rq_pbl_dma: dma_addr_t,
    pub rq_pbl_size: u32,
    pub xferq: *mut fcoe_xfrqe,
    pub xferq_dma: dma_addr_t,
    pub xferq_mem_size: u32,
    pub confq: *mut fcoe_confqe,
    pub confq_dma: dma_addr_t,
    pub confq_mem_size: u32,
    pub confq_pbl: *mut c_void,
    pub confq_pbl_dma: dma_addr_t,
    pub confq_pbl_size: u32,
    pub conn_db: *mut fcoe_conn_db,
    pub conn_db_dma: dma_addr_t,
    pub conn_db_mem_size: u32,
    pub lcq: *mut fcoe_sqe,
    pub lcq_dma: dma_addr_t,
    pub lcq_mem_size: u32,
    pub tgt_lock: spinlock_t,
    pub cq_lock: spinlock_t,
    pub num_active_ios: core::sync::atomic::AtomicI32,
    pub flush_in_prog: u32,
    pub timestamp: c_ulong,
    pub retry_delay_timestamp: c_ulong,
    pub pending_queue: [*mut bnx2fc_cmd; BNX2FC_SQ_WQES_MAX+1],
    pub active_cmd_queue: list_head,
    pub els_queue: list_head,
    pub io_retire_queue: list_head,
    pub active_tm_queue: list_head,
    pub ofld_timer: timer_list,
    pub ofld_wait: wait_queue_head_t,
    pub upld_timer: timer_list,
    pub upld_wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_mp_req {
    pub tm_lun: u64,
    pub tm_flags: u8,
    pub req_len: u32,
    pub req_buf: *mut c_void,
    pub req_buf_dma: dma_addr_t,
    pub mp_req_bd: *mut fcoe_bd_ctx,
    pub mp_req_bd_dma: dma_addr_t,
    pub req_fc_hdr: fc_frame_header,
    pub resp_len: u32,
    pub resp_buf: *mut c_void,
    pub resp_buf_dma: dma_addr_t,
    pub mp_resp_bd: *mut fcoe_bd_ctx,
    pub mp_resp_bd_dma: dma_addr_t,
    pub resp_fc_hdr: fc_frame_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_els_cb_arg {
    pub aborted_io_req: *mut bnx2fc_cmd,
    pub io_req: *mut bnx2fc_cmd,
    pub l2_oxid: u16,
    pub offset: u32,
    pub r_ctl: fc_rctl,
}

// bnx2fc command structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_cmd {
    pub link: list_head,
    pub on_active_queue: u8,
    pub on_tmf_queue: u8,
    pub cmd_type: u8,
pub const BNX2FC_SCSI_CMD: c_int = 1;
pub const BNX2FC_TASK_MGMT_CMD: c_int = 2;
pub const BNX2FC_ABTS: c_int = 3;
pub const BNX2FC_ELS: c_int = 4;
pub const BNX2FC_CLEANUP: c_int = 5;
pub const BNX2FC_SEQ_CLEANUP: c_int = 6;
    pub io_req_flags: u8,
    pub refcount: kref,
    pub port: *mut fcoe_port,
    pub tgt: *mut bnx2fc_rport,
    pub sc_cmd: *mut scsi_cmnd,
    pub cmd_mgr: *mut bnx2fc_cmd_mgr,
    pub mp_req: bnx2fc_mp_req,
    pub cb_arg): *mut *mut void (cb_func)(struct bnx2fc_els_cb_arg,
    pub cb_arg: *mut bnx2fc_els_cb_arg,
    pub /: *mut *mut delayed_work timeout_work; / timer for ULP timeouts,
    pub abts_done: completion,
    pub cleanup_done: completion,
    pub wait_for_abts_comp: c_int,
    pub wait_for_cleanup_comp: c_int,
    pub xid: u16,
    pub err_entry: fcoe_err_report_entry,
    pub task: *mut fcoe_task_ctx_entry,
    pub bd_tbl: *mut io_bdt,
    pub rsp: *mut fcp_rsp,
    pub data_xfer_len: usize,
    pub req_flags: c_ulong,
pub const BNX2FC_FLAG_ISSUE_RRQ: c_uint = 0x1;
pub const BNX2FC_FLAG_ISSUE_ABTS: c_uint = 0x2;
pub const BNX2FC_FLAG_ABTS_DONE: c_uint = 0x3;
pub const BNX2FC_FLAG_TM_COMPL: c_uint = 0x4;
pub const BNX2FC_FLAG_TM_TIMEOUT: c_uint = 0x5;
pub const BNX2FC_FLAG_IO_CLEANUP: c_uint = 0x6;
pub const BNX2FC_FLAG_RETIRE_OXID: c_uint = 0x7;
pub const BNX2FC_FLAG_EH_ABORT: c_uint = 0x8;
pub const BNX2FC_FLAG_IO_COMPL: c_uint = 0x9;
pub const BNX2FC_FLAG_ELS_DONE: c_uint = 0xa;
pub const BNX2FC_FLAG_ELS_TIMEOUT: c_uint = 0xb;
pub const BNX2FC_FLAG_CMD_LOST: c_uint = 0xc;
pub const BNX2FC_FLAG_SRR_SENT: c_uint = 0xd;
pub const BNX2FC_FLAG_ISSUE_CLEANUP_REQ: c_uint = 0xe;
    pub rec_retry: u8,
    pub srr_retry: u8,
    pub srr_offset: u32,
    pub srr_rctl: u8,
    pub fcp_resid: u32,
    pub fcp_rsp_len: u32,
    pub fcp_sns_len: u32,
    pub /: *mut *mut u8 cdb_status; / SCSI IO status,
    pub /: *mut *mut u8 fcp_status; / FCP IO status,
    pub fcp_rsp_code: u8,
    pub scsi_comp_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_bdt {
    pub io_req: *mut bnx2fc_cmd,
    pub bd_tbl: *mut fcoe_bd_ctx,
    pub bd_tbl_dma: dma_addr_t,
    pub bd_valid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_work {
    pub list: list_head,
    pub tgt: *mut bnx2fc_rport,
    pub task: *mut fcoe_task_ctx_entry,
    pub rq_data: [c_uchar; BNX2FC_RQ_BUF_SZ],
    pub wqe: u16,
    pub num_rq: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_unsol_els {
    pub lport: *mut fc_lport,
    pub fp: *mut fc_frame,
    pub hba: *mut bnx2fc_hba,
    pub unsol_els_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2fc_priv {
    pub io_req: *mut bnx2fc_cmd,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
extern "C" {
    pub fn bnx2fc_cmd_release(ref: *mut kref);
}
extern "C" {
    pub fn bnx2fc_send_fw_fcoe_init_msg(hba: *mut bnx2fc_hba) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_fw_fcoe_destroy_msg(hba: *mut bnx2fc_hba) -> c_int;
}
extern "C" {
    pub fn bnx2fc_map_doorbell(tgt: *mut bnx2fc_rport) -> c_int;
}
extern "C" {
    pub fn bnx2fc_setup_task_ctx(hba: *mut bnx2fc_hba) -> c_int;
}
extern "C" {
    pub fn bnx2fc_free_task_ctx(hba: *mut bnx2fc_hba);
}
extern "C" {
    pub fn bnx2fc_setup_fw_resc(hba: *mut bnx2fc_hba) -> c_int;
}
extern "C" {
    pub fn bnx2fc_free_fw_resc(hba: *mut bnx2fc_hba);
}
extern "C" {
    pub fn bnx2fc_cmd_mgr_free(cmgr: *mut bnx2fc_cmd_mgr);
}
extern "C" {
    pub fn bnx2fc_get_link_state(hba: *mut bnx2fc_hba);
}
extern "C" {
    pub fn bnx2fc_return_rqe(tgt: *mut bnx2fc_rport, num_items: u8);
}
extern "C" {
    pub fn bnx2fc_get_paged_crc_eof(skb: *mut sk_buff, tlen: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_rrq(aborted_io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_adisc(tgt: *mut bnx2fc_rport, fp: *mut fc_frame) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_logo(tgt: *mut bnx2fc_rport, fp: *mut fc_frame) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_rls(tgt: *mut bnx2fc_rport, fp: *mut fc_frame) -> c_int;
}
extern "C" {
    pub fn bnx2fc_initiate_cleanup(io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_initiate_abts(io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_init_mp_req(io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_add_2_sq(tgt: *mut bnx2fc_rport, xid: u16);
}
extern "C" {
    pub fn bnx2fc_ring_doorbell(tgt: *mut bnx2fc_rport);
}
extern "C" {
    pub fn bnx2fc_eh_abort(sc_cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_eh_target_reset(sc_cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_eh_device_reset(sc_cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_flush_active_ios(tgt: *mut bnx2fc_rport);
}
extern "C" {
    pub fn bnx2fc_arm_cq(tgt: *mut bnx2fc_rport);
}
extern "C" {
    pub fn bnx2fc_process_new_cqes(tgt: *mut bnx2fc_rport) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_stat_req(hba: *mut bnx2fc_hba) -> c_int;
}
extern "C" {
    pub fn bnx2fc_post_io_req(tgt: *mut bnx2fc_rport, io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_rec(orig_io_req: *mut bnx2fc_cmd) -> c_int;
}
extern "C" {
    pub fn bnx2fc_send_srr(orig_io_req: *mut bnx2fc_cmd, offset: u32, r_ctl: u8) -> c_int;
}

