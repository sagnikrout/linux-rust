//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/nvmetcp_common.h
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
// Copyright 2021 Marvell. All rights reserved.

// NVMeTCP firmware function init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_spe_func_init {
    pub half_way_close_timeout: __le16,
    pub num_sq_pages_in_ring: u8,
    pub num_r2tq_pages_in_ring: u8,
    pub num_uhq_pages_in_ring: u8,
    pub ll2_rx_queue_id: u8,
    pub flags: u8,
pub const NVMETCP_SPE_FUNC_INIT_COUNTERS_EN_MASK: c_uint = 0x1;
pub const NVMETCP_SPE_FUNC_INIT_COUNTERS_EN_SHIFT: c_int = 0;
pub const NVMETCP_SPE_FUNC_INIT_NVMETCP_MODE_MASK: c_uint = 0x1;
pub const NVMETCP_SPE_FUNC_INIT_NVMETCP_MODE_SHIFT: c_int = 1;
pub const NVMETCP_SPE_FUNC_INIT_RESERVED0_MASK: c_uint = 0x3F;
pub const NVMETCP_SPE_FUNC_INIT_RESERVED0_SHIFT: c_int = 2;
    pub debug_flags: u8,
    pub reserved1: __le16,
    pub params: u8,
pub const NVMETCP_SPE_FUNC_INIT_MAX_SYN_RT_MASK: c_uint = 0xF;
pub const NVMETCP_SPE_FUNC_INIT_MAX_SYN_RT_SHIFT: c_int = 0;
pub const NVMETCP_SPE_FUNC_INIT_RESERVED1_MASK: c_uint = 0xF;
pub const NVMETCP_SPE_FUNC_INIT_RESERVED1_SHIFT: c_int = 4;
    pub reserved2: [u8; 5],
    pub func_params: scsi_init_func_params,
    pub q_params: scsi_init_func_queues,
}

// NVMeTCP init params passed by driver to FW in NVMeTCP init ramrod.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_init_ramrod_params {
    pub nvmetcp_init_spe: nvmetcp_spe_func_init,
    pub tcp_init: tcp_init_params,
}

// NVMeTCP Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmetcp_ramrod_cmd_id {
    NVMETCP_RAMROD_CMD_ID_UNUSED = 0,
    NVMETCP_RAMROD_CMD_ID_INIT_FUNC = 1,
    NVMETCP_RAMROD_CMD_ID_DESTROY_FUNC = 2,
    NVMETCP_RAMROD_CMD_ID_OFFLOAD_CONN = 3,
    NVMETCP_RAMROD_CMD_ID_UPDATE_CONN = 4,
    NVMETCP_RAMROD_CMD_ID_TERMINATION_CONN = 5,
    NVMETCP_RAMROD_CMD_ID_CLEAR_SQ = 6,
    MAX_NVMETCP_RAMROD_CMD_ID
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_glbl_queue_entry {
    pub cq_pbl_addr: regpair,
    pub reserved: regpair,
}

// NVMeTCP conn level EQEs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmetcp_eqe_opcode {
    NVMETCP_EVENT_TYPE_INIT_FUNC = 0, /* Response after init Ramrod */
    NVMETCP_EVENT_TYPE_DESTROY_FUNC, /* Response after destroy Ramrod */
    NVMETCP_EVENT_TYPE_OFFLOAD_CONN,/* Response after option 2 offload Ramrod */
    NVMETCP_EVENT_TYPE_UPDATE_CONN, /* Response after update Ramrod */
    NVMETCP_EVENT_TYPE_CLEAR_SQ, /* Response after clear sq Ramrod */
    NVMETCP_EVENT_TYPE_TERMINATE_CONN, /* Response after termination Ramrod */
    NVMETCP_EVENT_TYPE_RESERVED0,
    NVMETCP_EVENT_TYPE_RESERVED1,
    NVMETCP_EVENT_TYPE_ASYN_CONNECT_COMPLETE, /* Connect completed (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_TERMINATE_DONE, /* Termination completed (A-syn EQE) */
    NVMETCP_EVENT_TYPE_START_OF_ERROR_TYPES = 10, /* Separate EQs from err EQs */
    NVMETCP_EVENT_TYPE_ASYN_ABORT_RCVD, /* TCP RST packet receive (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_CLOSE_RCVD, /* TCP FIN packet receive (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_SYN_RCVD, /* TCP SYN+ACK packet receive (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_MAX_RT_TIME, /* TCP max retransmit time (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_MAX_RT_CNT, /* TCP max retransmit count (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_MAX_KA_PROBES_CNT, /* TCP ka probes count (A-syn EQE) */
    NVMETCP_EVENT_TYPE_ASYN_FIN_WAIT2, /* TCP fin wait 2 (A-syn EQE) */
    NVMETCP_EVENT_TYPE_NVMETCP_CONN_ERROR, /* NVMeTCP error response (A-syn EQE) */
    NVMETCP_EVENT_TYPE_TCP_CONN_ERROR, /* NVMeTCP error - tcp error (A-syn EQE) */
    MAX_NVMETCP_EQE_OPCODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_conn_offload_section {
    pub /: *mut *mut regpair cccid_itid_table_addr; / CCCID to iTID table address,
    pub /: *mut *mut __le16 cccid_max_range; / CCCID max value - used for validation,
    pub reserved: [__le16; 3],
}

// NVMe TCP connection offload params passed by driver to FW in NVMeTCP offload ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_conn_offload_params {
    pub sq_pbl_addr: regpair,
    pub r2tq_pbl_addr: regpair,
    pub xhq_pbl_addr: regpair,
    pub uhq_pbl_addr: regpair,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub flags: u8,
pub const NVMETCP_CONN_OFFLOAD_PARAMS_TCP_ON_CHIP_1B_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_TCP_ON_CHIP_1B_SHIFT: c_int = 0;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_TARGET_MODE_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_TARGET_MODE_SHIFT: c_int = 1;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_RESTRICTED_MODE_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_RESTRICTED_MODE_SHIFT: c_int = 2;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_NVMETCP_MODE_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_NVMETCP_MODE_SHIFT: c_int = 3;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_RESERVED1_MASK: c_uint = 0xF;
pub const NVMETCP_CONN_OFFLOAD_PARAMS_RESERVED1_SHIFT: c_int = 4;
    pub default_cq: u8,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub initial_ack: __le32,
    pub /: *mut *mut nvmetcp_conn_offload_section nvmetcp; / NVMe/TCP section,
}

// NVMe TCP and TCP connection offload params passed by driver to FW in NVMeTCP offload ramrod.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_spe_conn_offload {
    pub reserved: __le16,
    pub conn_id: __le16,
    pub fw_cid: __le32,
    pub nvmetcp: nvmetcp_conn_offload_params,
    pub tcp: tcp_offload_params_opt2,
}

// NVMeTCP connection update params passed by driver to FW in NVMETCP update ramrod.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_conn_update_ramrod_params {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub flags: u8,
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_HD_EN_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_HD_EN_SHIFT: c_int = 0;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_DD_EN_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_DD_EN_SHIFT: c_int = 1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED0_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED0_SHIFT: c_int = 2;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED1_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED1_DATA_SHIFT: c_int = 3;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED2_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED2_SHIFT: c_int = 4;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED3_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED3_SHIFT: c_int = 5;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED4_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED4_SHIFT: c_int = 6;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED5_MASK: c_uint = 0x1;
pub const NVMETCP_CONN_UPDATE_RAMROD_PARAMS_RESERVED5_SHIFT: c_int = 7;
    pub reserved3: [u8; 3],
    pub max_seq_size: __le32,
    pub max_send_pdu_length: __le32,
    pub max_recv_pdu_length: __le32,
    pub first_seq_length: __le32,
    pub reserved4: [__le32; 5],
}

// NVMeTCP connection termination request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_spe_conn_termination {
    pub reserved0: __le16,
    pub conn_id: __le16,
    pub reserved1: __le32,
    pub abortive: u8,
    pub reserved2: [u8; 7],
    pub reserved3: regpair,
    pub reserved4: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_dif_flags {
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmetcp_wqe_type {
    NVMETCP_WQE_TYPE_NORMAL,
    NVMETCP_WQE_TYPE_TASK_CLEANUP,
    NVMETCP_WQE_TYPE_MIDDLE_PATH,
    NVMETCP_WQE_TYPE_IC,
    MAX_NVMETCP_WQE_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_wqe {
    pub task_id: __le16,
    pub flags: u8,
pub const NVMETCP_WQE_WQE_TYPE_MASK: c_uint = 0x7 /* [use nvmetcp_wqe_type] */;
pub const NVMETCP_WQE_WQE_TYPE_SHIFT: c_int = 0;
pub const NVMETCP_WQE_NUM_SGES_MASK: c_uint = 0xF;
pub const NVMETCP_WQE_NUM_SGES_SHIFT: c_int = 3;
pub const NVMETCP_WQE_RESPONSE_MASK: c_uint = 0x1;
pub const NVMETCP_WQE_RESPONSE_SHIFT: c_int = 7;
    pub prot_flags: nvmetcp_dif_flags,
    pub contlen_cdbsize: __le32,
pub const NVMETCP_WQE_CONT_LEN_MASK: c_uint = 0xFFFFFF;
pub const NVMETCP_WQE_CONT_LEN_SHIFT: c_int = 0;
pub const NVMETCP_WQE_CDB_SIZE_OR_NVMETCP_CMD_MASK: c_uint = 0xFF;
pub const NVMETCP_WQE_CDB_SIZE_OR_NVMETCP_CMD_SHIFT: c_int = 24;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_host_cccid_itid_entry {
    pub itid: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_connect_done_results {
    pub icid: __le16,
    pub conn_id: __le16,
    pub params: tcp_ulp_connect_done_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_eqe_data {
    pub icid: __le16,
    pub conn_id: __le16,
    pub reserved: __le16,
    pub error_code: u8,
    pub error_pdu_opcode_reserved: u8,
pub const NVMETCP_EQE_DATA_ERROR_PDU_OPCODE_MASK: c_uint = 0x3F;
pub const NVMETCP_EQE_DATA_ERROR_PDU_OPCODE_SHIFT: c_int = 0;
pub const NVMETCP_EQE_DATA_ERROR_PDU_OPCODE_VALID_MASK: c_uint = 0x1;
pub const NVMETCP_EQE_DATA_ERROR_PDU_OPCODE_VALID_SHIFT: c_int = 6;
pub const NVMETCP_EQE_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const NVMETCP_EQE_DATA_RESERVED0_SHIFT: c_int = 7;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmetcp_task_type {
    NVMETCP_TASK_TYPE_HOST_WRITE,
    NVMETCP_TASK_TYPE_HOST_READ,
    NVMETCP_TASK_TYPE_INIT_CONN_REQUEST,
    NVMETCP_TASK_TYPE_RESERVED0,
    NVMETCP_TASK_TYPE_CLEANUP,
    NVMETCP_TASK_TYPE_HOST_READ_NO_CQE,
    MAX_NVMETCP_TASK_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_db_data {
    pub params: u8,
pub const NVMETCP_DB_DATA_DEST_MASK: c_uint = 0x3 /* destination of doorbell (use enum db_dest) */;
pub const NVMETCP_DB_DATA_DEST_SHIFT: c_int = 0;
pub const NVMETCP_DB_DATA_AGG_CMD_MASK: c_uint = 0x3 /* aggregative command to CM (use enum db_agg_cmd_sel) */;
pub const NVMETCP_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const NVMETCP_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1 /* enable QM bypass */;
pub const NVMETCP_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const NVMETCP_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const NVMETCP_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const NVMETCP_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3 /* aggregative value selection */;
pub const NVMETCP_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub /: *mut *mut u8 agg_flags; / bit for every DQ counter flags in CM context that DQ can increment,
    pub sq_prod: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_fw_nvmf_cqe {
    pub reserved: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_icresp_mdata {
    pub digest: u8,
    pub cpda: u8,
    pub pfv: __le16,
    pub maxdata: __le32,
    pub rsvd: [__le16; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmetcp_fw_cqe_data {
    pub nvme_cqe: nvmetcp_fw_nvmf_cqe,
    pub icresp_mdata: nvmetcp_icresp_mdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_fw_cqe {
    pub conn_id: __le16,
    pub cqe_type: u8,
    pub cqe_error_status_bits: u8,
pub const CQE_ERROR_BITMAP_DIF_ERR_BITS_MASK: c_uint = 0x7;
pub const CQE_ERROR_BITMAP_DIF_ERR_BITS_SHIFT: c_int = 0;
pub const CQE_ERROR_BITMAP_DATA_DIGEST_ERR_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_DATA_DIGEST_ERR_SHIFT: c_int = 3;
pub const CQE_ERROR_BITMAP_RCV_ON_INVALID_CONN_MASK: c_uint = 0x1;
pub const CQE_ERROR_BITMAP_RCV_ON_INVALID_CONN_SHIFT: c_int = 4;
    pub itid: __le16,
    pub task_type: u8,
    pub fw_dbg_field: u8,
    pub caused_conn_err: u8,
    pub reserved0: [u8; 3],
    pub reserved1: __le32,
    pub cqe_data: nvmetcp_fw_cqe_data,
    pub task_opaque: regpair,
    pub reserved: [__le32; 6],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmetcp_fw_cqes_type {
    NVMETCP_FW_CQE_TYPE_NORMAL = 1,
    NVMETCP_FW_CQE_TYPE_RESERVED0,
    NVMETCP_FW_CQE_TYPE_RESERVED1,
    NVMETCP_FW_CQE_TYPE_CLEANUP,
    NVMETCP_FW_CQE_TYPE_DUMMY,
    MAX_NVMETCP_FW_CQES_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_nvmetcp_task_state {
    pub data_desc: scsi_cached_sges,
    pub sgl_params: scsi_sgl_params,
    pub resrved0: __le32,
    pub buffer_offset: __le32,
    pub cccid: __le16,
    pub dif_flags: nvmetcp_dif_flags,
    pub flags: u8,
pub const YSTORM_NVMETCP_TASK_STATE_LOCAL_COMP_MASK: c_uint = 0x1;
pub const YSTORM_NVMETCP_TASK_STATE_LOCAL_COMP_SHIFT: c_int = 0;
pub const YSTORM_NVMETCP_TASK_STATE_SLOW_IO_MASK: c_uint = 0x1;
pub const YSTORM_NVMETCP_TASK_STATE_SLOW_IO_SHIFT: c_int = 1;
pub const YSTORM_NVMETCP_TASK_STATE_SET_DIF_OFFSET_MASK: c_uint = 0x1;
pub const YSTORM_NVMETCP_TASK_STATE_SET_DIF_OFFSET_SHIFT: c_int = 2;
pub const YSTORM_NVMETCP_TASK_STATE_SEND_W_RSP_MASK: c_uint = 0x1;
pub const YSTORM_NVMETCP_TASK_STATE_SEND_W_RSP_SHIFT: c_int = 3;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_nvmetcp_task_rxmit_opt {
    pub reserved: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_task_hdr {
    pub reg: [__le32; 18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_task_hdr_aligned {
    pub task_hdr: nvmetcp_task_hdr,
    pub /: *mut *mut __le32 reserved[2]; / HSI_COMMENT: Align to QREG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_tdif_task_context {
    pub reserved: [__le32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_rdif_task_context {
    pub reserved: [__le32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_nvmetcp_task_st_ctx {
    pub state: ystorm_nvmetcp_task_state,
    pub rxmit_opt: ystorm_nvmetcp_task_rxmit_opt,
    pub pdu_hdr: nvmetcp_task_hdr_aligned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_nvmetcp_task_st_ctx {
    pub data_desc: scsi_cached_sges,
    pub sgl_params: scsi_sgl_params,
    pub rem_task_size: __le32,
    pub data_buffer_offset: __le32,
    pub task_type: u8,
    pub dif_flags: nvmetcp_dif_flags,
    pub dif_task_icid: __le16,
    pub reserved0: regpair,
    pub expected_itt: __le32,
    pub reserved1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_nvmetcp_task_st_ctx {
    pub rem_rcv_len: __le32,
    pub exp_data_transfer_len: __le32,
    pub exp_data_sn: __le32,
    pub reserved0: regpair,
    pub reg1_map: __le32,
pub const REG1_NUM_SGES_MASK: c_uint = 0xF;
pub const REG1_NUM_SGES_SHIFT: c_int = 0;
pub const REG1_RESERVED1_MASK: c_uint = 0xFFFFFFF;
pub const REG1_RESERVED1_SHIFT: c_int = 4;
    pub flags2: u8,
pub const USTORM_NVMETCP_TASK_ST_CTX_AHS_EXIST_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_AHS_EXIST_SHIFT: c_int = 0;
pub const USTORM_NVMETCP_TASK_ST_CTX_RESERVED1_MASK: c_uint = 0x7F;
pub const USTORM_NVMETCP_TASK_ST_CTX_RESERVED1_SHIFT: c_int = 1;
    pub dif_flags: nvmetcp_dif_flags,
    pub reserved3: __le16,
    pub tqe_opaque: [__le16; 2],
    pub reserved5: __le32,
    pub nvme_tcp_opaque_lo: __le32,
    pub nvme_tcp_opaque_hi: __le32,
    pub task_type: u8,
    pub error_flags: u8,
pub const USTORM_NVMETCP_TASK_ST_CTX_DATA_DIGEST_ERROR_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_DATA_DIGEST_ERROR_SHIFT: c_int = 0;
pub const USTORM_NVMETCP_TASK_ST_CTX_DATA_TRUNCATED_ERROR_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_DATA_TRUNCATED_ERROR_SHIFT: c_int = 1;
pub const USTORM_NVMETCP_TASK_ST_CTX_UNDER_RUN_ERROR_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_UNDER_RUN_ERROR_SHIFT: c_int = 2;
pub const USTORM_NVMETCP_TASK_ST_CTX_NVME_TCP_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_NVME_TCP_SHIFT: c_int = 3;
    pub flags: u8,
pub const USTORM_NVMETCP_TASK_ST_CTX_CQE_WRITE_MASK: c_uint = 0x3;
pub const USTORM_NVMETCP_TASK_ST_CTX_CQE_WRITE_SHIFT: c_int = 0;
pub const USTORM_NVMETCP_TASK_ST_CTX_LOCAL_COMP_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_LOCAL_COMP_SHIFT: c_int = 2;
pub const USTORM_NVMETCP_TASK_ST_CTX_Q0_R2TQE_WRITE_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_Q0_R2TQE_WRITE_SHIFT: c_int = 3;
pub const USTORM_NVMETCP_TASK_ST_CTX_TOTAL_DATA_ACKED_DONE_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_TOTAL_DATA_ACKED_DONE_SHIFT: c_int = 4;
pub const USTORM_NVMETCP_TASK_ST_CTX_HQ_SCANNED_DONE_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_HQ_SCANNED_DONE_SHIFT: c_int = 5;
pub const USTORM_NVMETCP_TASK_ST_CTX_R2T2RECV_DONE_MASK: c_uint = 0x1;
pub const USTORM_NVMETCP_TASK_ST_CTX_R2T2RECV_DONE_SHIFT: c_int = 6;
    pub cq_rss_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_ystorm_nvmetcp_task_ag_ctx {
    pub /: *mut *mut u8 reserved / cdu_validation,
    pub /: *mut *mut u8 byte1 / state_and_core_id,
    pub /: *mut *mut __le16 word0 / icid,
    pub flags0: u8,
    pub flags1: u8,
    pub flags2: u8,
    pub flags3: u8,
    pub TTT: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub byte4: u8,
    pub reserved7: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_mstorm_nvmetcp_task_ag_ctx {
    pub cdu_validation: u8,
    pub byte1: u8,
    pub task_cid: __le16,
    pub flags0: u8,
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_SHIFT: c_int = 5;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_VALID_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_VALID_SHIFT: c_int = 6;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_FLAG_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_FLAG_SHIFT: c_int = 7;
    pub flags1: u8,
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_CF_MASK: c_uint = 0x3;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_CF_SHIFT: c_int = 0;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF2_SHIFT: c_int = 4;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_CF_EN_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_TASK_CLEANUP_CF_EN_SHIFT: c_int = 6;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const E5_MSTORM_NVMETCP_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
    pub flags3: u8,
    pub reg0: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub byte4: u8,
    pub reserved7: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_ustorm_nvmetcp_task_ag_ctx {
    pub reserved: u8,
    pub state_and_core_id: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CONN_CLEAR_SQ_FLAG_SHIFT: c_int = 5;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_HQ_SCANNED_CF_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_HQ_SCANNED_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED1_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED1_SHIFT: c_int = 0;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_R2T2RECV_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_R2T2RECV_SHIFT: c_int = 2;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CF3_SHIFT: c_int = 4;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_CF_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_HQ_SCANNED_CF_EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_HQ_SCANNED_CF_EN_SHIFT: c_int = 0;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DISABLE_DATA_ACKED_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DISABLE_DATA_ACKED_SHIFT: c_int = 1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_R2T2RECV_EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_R2T2RECV_EN_SHIFT: c_int = 2;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_CF_EN_SHIFT: c_int = 4;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CMP_DATA_TOTAL_EXP_EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CMP_DATA_TOTAL_EXP_EN_SHIFT: c_int = 5;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 6;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CMP_CONT_RCV_EXP_EN_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_CMP_CONT_RCV_EXP_EN_SHIFT: c_int = 7;
    pub flags3: u8,
    pub flags4: u8,
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED5_MASK: c_uint = 0x3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED5_SHIFT: c_int = 0;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED6_SHIFT: c_int = 2;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_RESERVED7_SHIFT: c_int = 3;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_TYPE_MASK: c_uint = 0xF;
pub const E5_USTORM_NVMETCP_TASK_AG_CTX_DIF_ERROR_TYPE_SHIFT: c_int = 4;
    pub byte2: u8,
    pub byte3: u8,
    pub reserved8: u8,
    pub dif_err_intervals: __le32,
    pub dif_error_1st_interval: __le32,
    pub rcv_cont_len: __le32,
    pub exp_cont_len: __le32,
    pub total_data_acked: __le32,
    pub exp_data_acked: __le32,
    pub word1: __le16,
    pub next_tid: __le16,
    pub hdr_residual_count: __le32,
    pub exp_r2t_sn: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5_nvmetcp_task_context {
    pub ystorm_st_context: ystorm_nvmetcp_task_st_ctx,
    pub ystorm_ag_context: e5_ystorm_nvmetcp_task_ag_ctx,
    pub ystorm_ag_padding: [regpair; 2],
    pub tdif_context: e5_tdif_task_context,
    pub mstorm_ag_context: e5_mstorm_nvmetcp_task_ag_ctx,
    pub mstorm_ag_padding: [regpair; 2],
    pub ustorm_ag_context: e5_ustorm_nvmetcp_task_ag_ctx,
    pub ustorm_ag_padding: [regpair; 2],
    pub mstorm_st_context: mstorm_nvmetcp_task_st_ctx,
    pub mstorm_st_padding: [regpair; 2],
    pub ustorm_st_context: ustorm_nvmetcp_task_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
    pub rdif_context: e5_rdif_task_context,
}
