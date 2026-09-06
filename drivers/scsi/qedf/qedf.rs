//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedf/qedf.h
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//

// qedf_hsi.h needs to before included any qed includes

// Helpers to extract upper and lower 32-bits of pointer

pub const QEDF_FLOGI_RETRY_CNT: c_int = 3;
pub const QEDF_RPORT_RETRY_CNT: c_int = 255;
pub const QEDF_MAX_SESSIONS: c_int = 1024;
pub const QEDF_MAX_PAYLOAD: c_int = 2048;
pub const QEDF_MAX_BDS_PER_CMD: c_int = 256;
pub const QEDF_MAX_BD_LEN: c_uint = 0xffff;
pub const QEDF_BD_SPLIT_SZ: c_uint = 0x1000;
pub const QEDF_PAGE_SIZE: c_int = 4096;
pub const QED_HW_DMA_BOUNDARY: c_uint = 0xfff;

pub const QEDF_MAX_NPIV: c_int = 64;
pub const QEDF_TM_TIMEOUT: c_int = 10;

pub const QEDF_CLEANUP_TIMEOUT: c_int = 1;
pub const QEDF_MAX_CDB_LEN: c_int = 16;

pub const UPSTREAM_REMOVE: c_int = 1;
pub const UPSTREAM_KEEP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_mp_req {
    pub req_len: u32,
    pub req_buf: *mut c_void,
    pub req_buf_dma: dma_addr_t,
    pub mp_req_bd: *mut scsi_sge,
    pub mp_req_bd_dma: dma_addr_t,
    pub req_fc_hdr: fc_frame_header,
    pub resp_len: u32,
    pub resp_buf: *mut c_void,
    pub resp_buf_dma: dma_addr_t,
    pub mp_resp_bd: *mut scsi_sge,
    pub mp_resp_bd_dma: dma_addr_t,
    pub resp_fc_hdr: fc_frame_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_els_cb_arg {
    pub aborted_io_req: *mut qedf_ioreq,
    pub io_req: *mut qedf_ioreq,
    pub /: *mut *mut u8 op; / Used to keep track of ELS op,
    pub l2_oxid: u16,
    pub /: *mut *mut u32 offset; / Used for sequence cleanup,
    pub /: *mut *mut u8 r_ctl; / Used for sequence cleanup,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedf_ioreq_event {
    QEDF_IOREQ_EV_NONE,
    QEDF_IOREQ_EV_ABORT_SUCCESS,
    QEDF_IOREQ_EV_ABORT_FAILED,
    QEDF_IOREQ_EV_SEND_RRQ,
    QEDF_IOREQ_EV_ELS_TMO,
    QEDF_IOREQ_EV_ELS_ERR_DETECT,
    QEDF_IOREQ_EV_ELS_FLUSH,
    QEDF_IOREQ_EV_CLEANUP_SUCCESS,
    QEDF_IOREQ_EV_CLEANUP_FAILED,
}

pub const FC_GOOD: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_ioreq {
    pub link: list_head,
    pub xid: u16,
    pub sc_cmd: *mut scsi_cmnd,
pub const QEDF_SCSI_CMD: c_int = 1;
pub const QEDF_TASK_MGMT_CMD: c_int = 2;
pub const QEDF_ABTS: c_int = 3;
pub const QEDF_ELS: c_int = 4;
pub const QEDF_CLEANUP: c_int = 5;
pub const QEDF_SEQ_CLEANUP: c_int = 6;
    pub cmd_type: u8,
pub const QEDF_CMD_OUTSTANDING: c_uint = 0x0;
pub const QEDF_CMD_IN_ABORT: c_uint = 0x1;
pub const QEDF_CMD_IN_CLEANUP: c_uint = 0x2;
pub const QEDF_CMD_SRR_SENT: c_uint = 0x3;
pub const QEDF_CMD_DIRTY: c_uint = 0x4;
pub const QEDF_CMD_ERR_SCSI_DONE: c_uint = 0x5;
    pub io_req_flags: u8,
    pub tm_flags: u8,
    pub tm_lun: u64,
    pub fcport: *mut qedf_rport,
pub const QEDF_CMD_ST_INACTIVE: c_int = 0;
pub const QEDFC_CMD_ST_IO_ACTIVE: c_int = 1;
pub const QEDFC_CMD_ST_ABORT_ACTIVE: c_int = 2;
pub const QEDFC_CMD_ST_ABORT_ACTIVE_EH: c_int = 3;
pub const QEDFC_CMD_ST_CLEANUP_ACTIVE: c_int = 4;
pub const QEDFC_CMD_ST_CLEANUP_ACTIVE_EH: c_int = 5;
pub const QEDFC_CMD_ST_RRQ_ACTIVE: c_int = 6;
pub const QEDFC_CMD_ST_RRQ_WAIT: c_int = 7;
pub const QEDFC_CMD_ST_OXID_RETIRE_WAIT: c_int = 8;
pub const QEDFC_CMD_ST_TMF_ACTIVE: c_int = 9;
pub const QEDFC_CMD_ST_DRAIN_ACTIVE: c_int = 10;
pub const QEDFC_CMD_ST_CLEANED: c_int = 11;
pub const QEDFC_CMD_ST_ELS_ACTIVE: c_int = 12;
    pub state: core::sync::atomic::AtomicI32,
    pub flags: c_ulong,
    pub event: qedf_ioreq_event,
    pub data_xfer_len: usize,
// ID: 001: Alloc cmd (qedf_alloc_cmd)
// ID: 002: Initiate ABTS (qedf_initiate_abts)
// ID: 003: For RRQ (qedf_process_abts_compl)
    pub refcount: kref,
    pub cmd_mgr: *mut qedf_cmd_mgr,
    pub bd_tbl: *mut io_bdt,
    pub timeout_work: delayed_work,
    pub tm_done: completion,
    pub abts_done: completion,
    pub cleanup_done: completion,
    pub task: *mut fcoe_task_context,
    pub task_params: *mut fcoe_task_params,
    pub sgl_task_params: *mut scsi_sgl_task_params,
    pub idx: c_int,
    pub lun: c_int,
//
// Need to allocate enough room for both sense data and FCP response data
// which has a max length of 8 bytes according to spec.
//

    pub sense_buffer: *mut u8,
    pub sense_buffer_dma: dma_addr_t,
    pub fcp_resid: u32,
    pub fcp_rsp_len: u32,
    pub fcp_sns_len: u32,
    pub cdb_status: u8,
    pub fcp_status: u8,
    pub fcp_rsp_code: u8,
    pub scsi_comp_flags: u8,
pub const QEDF_MAX_REUSE: c_uint = 0xfff;
    pub reuse_count: u16,
    pub mp_req: qedf_mp_req,
    pub cb_arg): *mut *mut void (cb_func)(struct qedf_els_cb_arg,
    pub cb_arg: *mut qedf_els_cb_arg,
    pub fp_idx: c_int,
    pub cpu: c_uint,
    pub int_cpu: c_uint,
pub const QEDF_IOREQ_UNKNOWN_SGE: c_int = 1;
pub const QEDF_IOREQ_SLOW_SGE: c_int = 2;
pub const QEDF_IOREQ_FAST_SGE: c_int = 3;
    pub sge_type: u8,
    pub rrq_work: delayed_work,
// Used for sequence level recovery; i.e. REC/SRR
    pub rx_buf_off: u32,
    pub tx_buf_off: u32,
    pub rx_id: u32,
    pub task_retry_identifier: u32,
//
// Used to tell if we need to return a SCSI command
// during some form of error processing.
//
    pub return_scsi_cmd_on_abts: bool,
    pub alloc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_cmd_priv {
    pub io_req: *mut qedf_ioreq,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_rport {
    pub rport_lock: spinlock_t,
pub const QEDF_RPORT_SESSION_READY: c_int = 1;
pub const QEDF_RPORT_UPLOADING_CONNECTION: c_int = 2;
pub const QEDF_RPORT_IN_RESET: c_int = 3;
pub const QEDF_RPORT_IN_LUN_RESET: c_int = 4;
pub const QEDF_RPORT_IN_TARGET_RESET: c_int = 5;
    pub flags: c_ulong,
    pub lun_reset_lun: c_int,
    pub retry_delay_timestamp: c_ulong,
    pub rport: *mut fc_rport,
    pub rdata: *mut fc_rport_priv,
    pub qedf: *mut qedf_ctx,
    pub /: *mut *mut u32 handle; / Handle from qed,
    pub /: *mut *mut u32 fw_cid; / fw_cid from qed,
    pub p_doorbell: *mut void __iomem,
// Send queue management
    pub free_sqes: core::sync::atomic::AtomicI32,
    pub ios_to_queue: core::sync::atomic::AtomicI32,
    pub num_active_ios: core::sync::atomic::AtomicI32,
    pub sq: *mut fcoe_wqe,
    pub sq_dma: dma_addr_t,
    pub sq_prod_idx: u16,
    pub fw_sq_prod_idx: u16,
    pub sq_con_idx: u16,
    pub sq_mem_size: u32,
    pub sq_pbl: *mut c_void,
    pub sq_pbl_dma: dma_addr_t,
    pub sq_pbl_size: u32,
    pub sid: u32,
pub const QEDF_RPORT_TYPE_DISK: c_int = 0;
pub const QEDF_RPORT_TYPE_TAPE: c_int = 1;
    pub /: *mut *mut uint dev_type; / Disk or tape,
    pub peers: list_head,
}

// Used to contain LL2 skb's in ll2_skb_list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_skb_work {
    pub work: work_struct,
    pub skb: *mut sk_buff,
    pub qedf: *mut qedf_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_fastpath {
pub const QEDF_SB_ID_NULL: c_uint = 0xffff;
    pub sb_id: u16,
    pub sb_info: *mut qed_sb_info,
    pub qedf: *mut qedf_ctx,
// Keep track of number of completions on this fastpath
    pub completions: c_ulong,
    pub cq_num_entries: u32,
}

// Used to pass fastpath information needed to process CQEs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_io_work {
    pub work: work_struct,
    pub cqe: fcoe_cqe,
    pub qedf: *mut qedf_ctx,
    pub fp: *mut fc_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_glbl_q_params {
    pub /: *mut *mut u64 hw_p_cq; / Completion queue PBL,
    pub /: *mut *mut u64 hw_p_rq; / Request queue PBL,
    pub /: *mut *mut u64 hw_p_cmdq; / Command queue PBL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct global_queue {
    pub cq: *mut fcoe_cqe,
    pub cq_dma: dma_addr_t,
    pub cq_mem_size: u32,
    pub /: *mut *mut u32 cq_cons_idx; / Completion queue consumer index,
    pub cq_prod_idx: u32,
    pub cq_pbl: *mut c_void,
    pub cq_pbl_dma: dma_addr_t,
    pub cq_pbl_size: u32,
}

// I/O tracing entry
pub const QEDF_IO_TRACE_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_io_log {
pub const QEDF_IO_TRACE_REQ: c_int = 0;
pub const QEDF_IO_TRACE_RSP: c_int = 1;
    pub direction: u8,
    pub task_id: u16,
    pub /: *mut *mut uint32_t port_id; / Remote port fabric ID,
    pub lun: c_int,
    pub /: *mut *mut unsigned char op; / SCSI CDB,
    pub lba: [u8; 4],
    pub /: *mut *mut unsigned int bufflen; / SCSI buffer length,
    pub /: *mut *mut unsigned int sg_count; / Number of SG elements,
    pub /: *mut *mut int result; / Result passed back to mid-layer,
    pub /: *mut *mut unsigned long jiffies; / Time stamp when I/O logged,
    pub /: *mut *mut int refcount; / Reference count for task id,
    pub /: *mut *mut unsigned int req_cpu; / CPU that the task is queued on,
    pub /: *mut *mut unsigned int int_cpu; / Interrupt CPU that the task is received on,
    pub /: *mut *mut unsigned int rsp_cpu; / CPU that task is returned on,
    pub /: *mut *mut u8 sge_type; / Did we take the slow, single or fast SGE path,
}

// Number of entries in BDQ
pub const QEDF_BDQ_SIZE: c_int = 256;
pub const QEDF_BDQ_BUF_SIZE: c_int = 2072;
// DMA coherent buffers for BDQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_bdq_buf {
    pub buf_addr: *mut c_void,
    pub buf_dma: dma_addr_t,
}

// Main adapter struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_ctx {
    pub dbg_ctx: qedf_dbg_ctx,
    pub ctlr: fcoe_ctlr,
    pub lport: *mut fc_lport,
    pub data_src_addr: [u8; ETH_ALEN],
pub const QEDF_LINK_DOWN: c_int = 0;
pub const QEDF_LINK_UP: c_int = 1;
    pub link_state: core::sync::atomic::AtomicI32,
pub const QEDF_DCBX_PENDING: c_int = 0;
pub const QEDF_DCBX_DONE: c_int = 1;
    pub dcbx: core::sync::atomic::AtomicI32,

pub const QEDF_FALLBACK_VLAN: c_int = 1002;
pub const QEDF_DEFAULT_PRIO: c_int = 3;
    pub vlan_id: c_int,
    pub prio: u8,
    pub cdev: *mut qed_dev,
    pub dev_info: qed_dev_fcoe_info,
    pub int_info: qed_int_info,
    pub last_command: u16,
    pub hba_lock: spinlock_t,
    pub pdev: *mut pci_dev,
    pub wwnn: u64,
    pub wwpn: u64,
    pub mac: [u8 __aligned(16); ETH_ALEN],
    pub fcports: list_head,
    pub num_offloads: core::sync::atomic::AtomicI32,
    pub curr_conn_id: c_uint,
    pub ll2_recv_wq: *mut workqueue_struct,
    pub link_update_wq: *mut workqueue_struct,
    pub devlink: *mut devlink,
    pub link_update: delayed_work,
    pub link_recovery: delayed_work,
    pub flogi_compl: completion,
    pub fipvlan_compl: completion,
//
// Used to tell if we're in the window where we are waiting for
// the link to come back up before informting fcoe that the link is
// done.
//
    pub link_down_tmo_valid: core::sync::atomic::AtomicI32,

    pub /: *mut *mut timer_list timer; / One second book keeping timer,
pub const QEDF_DRAIN_ACTIVE: c_int = 1;
pub const QEDF_LL2_STARTED: c_int = 2;
pub const QEDF_UNLOADING: c_int = 3;
pub const QEDF_GRCDUMP_CAPTURE: c_int = 4;
pub const QEDF_IN_RECOVERY: c_int = 5;
pub const QEDF_DBG_STOP_IO: c_int = 6;
pub const QEDF_PROBING: c_int = 8;
pub const QEDF_STAG_IN_PROGRESS: c_int = 9;
    pub /: *mut *mut unsigned long flags; / Miscellaneous state flags,
    pub fipvlan_retries: c_int,
    pub num_queues: u8,
    pub global_queues: *mut global_queue,
// Pointer to array of queue structures
    pub p_cpuq: *mut qedf_glbl_q_params,
// Physical address of array of queue structures
    pub hw_p_cpuq: dma_addr_t,
    pub bdq: [qedf_bdq_buf; QEDF_BDQ_SIZE],
    pub bdq_pbl: *mut c_void,
    pub bdq_pbl_dma: dma_addr_t,
    pub bdq_pbl_mem_size: usize,
    pub bdq_pbl_list: *mut c_void,
    pub bdq_pbl_list_dma: dma_addr_t,
    pub bdq_pbl_list_num_entries: u8,
    pub bdq_primary_prod: *mut void __iomem,
    pub bdq_secondary_prod: *mut void __iomem,
    pub bdq_prod_idx: u16,
// Structure for holding all the fastpath for this qedf_ctx
    pub fp_array: *mut qedf_fastpath,
    pub tasks: qed_fcoe_tid,
    pub cmd_mgr: *mut qedf_cmd_mgr,
// Holds the PF parameters we pass to qed to start he FCoE function
    pub pf_params: qed_pf_params,
// Used to time middle path ELS and TM commands
    pub timer_work_queue: *mut workqueue_struct,
pub const QEDF_IO_WORK_MIN: c_int = 64;
    pub io_mempool: *mut mempool_t,
    pub dpc_wq: *mut workqueue_struct,
    pub recovery_work: delayed_work,
    pub board_disable_work: delayed_work,
    pub grcdump_work: delayed_work,
    pub stag_work: delayed_work,
    pub slow_sge_ios: u32,
    pub fast_sge_ios: u32,
    pub grcdump: *mut u8,
    pub grcdump_size: u32,
    pub io_trace_buf: [qedf_io_log; QEDF_IO_TRACE_SIZE],
    pub io_trace_lock: spinlock_t,
    pub io_trace_idx: u16,
    pub stop_io_on_error: bool,
    pub flogi_cnt: u32,
    pub flogi_failed: u32,
    pub flogi_pending: u32,
// Used for fc statistics
    pub stats_mutex: mutex,
    pub input_requests: u64,
    pub output_requests: u64,
    pub control_requests: u64,
    pub packet_aborts: u64,
    pub alloc_failures: u64,
    pub lun_resets: u8,
    pub target_resets: u8,
    pub task_set_fulls: u8,
    pub busy: u8,
// Used for flush routine
    pub flush_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_bdt {
    pub io_req: *mut qedf_ioreq,
    pub bd_tbl: *mut scsi_sge,
    pub bd_tbl_dma: dma_addr_t,
    pub bd_valid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_cmd_mgr {
    pub qedf: *mut qedf_ctx,
    pub idx: u16,
    pub io_bdt_pool: *mut io_bdt,
pub const FCOE_PARAMS_NUM_TASKS: c_int = 2048;
    pub cmds: [qedf_ioreq; FCOE_PARAMS_NUM_TASKS],
    pub lock: spinlock_t,
    pub free_list_cnt: core::sync::atomic::AtomicI32,
}

// Stolen from qed_cxt_api.h and adapted for qed_fcoe_info
// Usage:
//
// void *ptr;
// ptr = qedf_get_task_mem(&qedf->tasks, 128);
//
// Externs
//
// (QEDF_LOG_NPIV | QEDF_LOG_SESS | QEDF_LOG_LPORT | QEDF_LOG_ELS | QEDF_LOG_MQ
// | QEDF_LOG_IO | QEDF_LOG_UNSOL | QEDF_LOG_SCSI_TM | QEDF_LOG_MP_REQ |
// QEDF_LOG_EVT | QEDF_LOG_CONN | QEDF_LOG_DISC | QEDF_LOG_INFO)
//
pub const QEDF_DEFAULT_LOG_MASK: c_uint = 0x3CFB6;

extern "C" {
    pub fn qedf_cmd_mgr_free(cmgr: *mut qedf_cmd_mgr);
}
extern "C" {
    pub fn qedf_fip_send(fip: *mut fcoe_ctlr, skb: *mut sk_buff);
}
extern "C" {
    pub fn qedf_fip_recv(qedf: *mut qedf_ctx, skb: *mut sk_buff);
}
extern "C" {
    pub fn qedf_fcoe_send_vlan_req(qedf: *mut qedf_ctx);
}
extern "C" {
    pub fn qedf_flush_active_ios(fcport: *mut qedf_rport, lun: u64);
}
extern "C" {
    pub fn qedf_release_cmd(ref: *mut kref);
}
extern "C" {
    pub fn qedf_init_mp_req(io_req: *mut qedf_ioreq) -> c_int;
}
extern "C" {
    pub fn qedf_get_sqe_idx(fcport: *mut qedf_rport) -> u16;
}
extern "C" {
    pub fn qedf_ring_doorbell(fcport: *mut qedf_rport);
}
extern "C" {
    pub fn qedf_send_rrq(aborted_io_req: *mut qedf_ioreq) -> c_int;
}
extern "C" {
    pub fn qedf_send_adisc(fcport: *mut qedf_rport, fp: *mut fc_frame) -> c_int;
}
extern "C" {
    pub fn qedf_initiate_tmf(rport: *mut fc_rport, lun: u64, tm_flags: u8) -> c_int;
}
extern "C" {
    pub fn qedf_process_cqe(qedf: *mut qedf_ctx, cqe: *mut fcoe_cqe);
}
extern "C" {
    pub fn qedf_set_vlan_id(qedf: *mut qedf_ctx, vlan_id: c_int);
}
extern "C" {
    pub fn qedf_create_sysfs_ctx_attr(qedf: *mut qedf_ctx);
}
extern "C" {
    pub fn qedf_remove_sysfs_ctx_attr(qedf: *mut qedf_ctx);
}
extern "C" {
    pub fn qedf_capture_grc_dump(qedf: *mut qedf_ctx);
}
extern "C" {
    pub fn qedf_wait_for_upload(qedf: *mut qedf_ctx) -> bool;
}
extern "C" {
    pub fn qedf_restart_rport(fcport: *mut qedf_rport);
}
extern "C" {
    pub fn qedf_send_rec(orig_io_req: *mut qedf_ioreq) -> c_int;
}
extern "C" {
    pub fn qedf_send_flogi(qedf: *mut qedf_ctx) -> c_int;
}
extern "C" {
    pub fn qedf_get_protocol_tlv_data(dev: *mut c_void, data: *mut c_void);
}
extern "C" {
    pub fn qedf_fp_io_handler(work: *mut work_struct);
}
extern "C" {
    pub fn qedf_get_generic_tlv_data(dev: *mut c_void, data: *mut qed_generic_tlvs);
}
extern "C" {
    pub fn qedf_wq_grcdump(work: *mut work_struct);
}
extern "C" {
    pub fn qedf_stag_change_work(work: *mut work_struct);
}
extern "C" {
    pub fn qedf_ctx_soft_reset(lport: *mut fc_lport);
}
pub const FCOE_WORD_TO_BYTE: c_int = 4;
pub const QEDF_MAX_TASK_NUM: c_uint = 0xFFFF;
pub const QL45xxx: c_uint = 0x165C;
pub const QL41xxx: c_uint = 0x8080;
pub const MAX_CT_PAYLOAD: c_int = 2048;
pub const DISCOVERED_PORTS: c_int = 4;
pub const NUMBER_OF_PORTS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vlan {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac: fip_mac_desc,
    pub wwnn: fip_wwn_desc,
    pub desc: },
}

// SQ/CQ Sizes
pub const GBL_RSVD_TASKS: c_int = 16;
pub const NUM_TASKS_PER_CONNECTION: c_int = 1024;
pub const NUM_RW_TASKS_PER_CONNECTION: c_int = 512;

pub const QEDF_FCOE_PARAMS_GL_RQ_PI: c_int = 0;
pub const QEDF_FCOE_PARAMS_GL_CMD_PI: c_int = 1;

pub const MAX_FIBRE_LUNS: c_uint = 0xffffffff;

//
// PCI function probe defines
//
// Probe/remove called during normal PCI probe
pub const QEDF_MODE_NORMAL: c_int = 0;
// Probe/remove called from qed error recovery
pub const QEDF_MODE_RECOVERY: c_int = 1;

