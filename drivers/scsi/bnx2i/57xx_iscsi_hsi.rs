//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2i/57xx_iscsi_hsi.h
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


// 57xx_iscsi_hsi.h: QLogic NetXtreme II iSCSI HSI.
//
// Copyright (c) 2006 - 2013 Broadcom Corporation
// Copyright (c) 2014, QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Anil Veerabhadrappa (anilgv@broadcom.com)
// Previously Maintained by: Eddie Wai (eddie.wai@broadcom.com)
// Maintained by: QLogic-Storage-Upstream@qlogic.com
//
// iSCSI Async CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_async_msg {

    pub op_code: u8,
    pub reserved1: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub reserved1: u8,
    pub op_code: u8,

    pub reserved2: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved3: [u32; 2],
    pub reserved5: u16,
    pub err_code: u8,
    pub reserved4: u8,

    pub reserved4: u8,
    pub err_code: u8,
    pub reserved5: u16,

    pub reserved6: u32,
    pub lun: [u32; 2],
    pub async_event: u8,
    pub async_vcode: u8,
    pub param1: u16,

    pub param1: u16,
    pub async_vcode: u8,
    pub async_event: u8,

    pub param2: u16,
    pub param3: u16,

    pub param3: u16,
    pub param2: u16,
    pub reserved7: [u32; 3],
    pub cq_req_sn: u32,
}

//
// iSCSI Buffer Descriptor (BD)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bd {
    pub buffer_addr_hi: u32,
    pub buffer_addr_lo: u32,

    pub reserved0: u16,
    pub buffer_length: u16,

    pub buffer_length: u16,
    pub reserved0: u16,

    pub reserved3: u16,
    pub flags: u16,

pub const ISCSI_BD_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_BD_LAST_IN_BD_CHAIN_SHIFT: c_int = 6;

pub const ISCSI_BD_FIRST_IN_BD_CHAIN_SHIFT: c_int = 7;

pub const ISCSI_BD_RESERVED2_SHIFT: c_int = 8;

    pub flags: u16,

pub const ISCSI_BD_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_BD_LAST_IN_BD_CHAIN_SHIFT: c_int = 6;

pub const ISCSI_BD_FIRST_IN_BD_CHAIN_SHIFT: c_int = 7;

pub const ISCSI_BD_RESERVED2_SHIFT: c_int = 8;
    pub reserved3: u16,

}

//
// iSCSI Cleanup SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_cleanup_request {

    pub op_code: u8,
    pub reserved1: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub reserved1: u8,
    pub op_code: u8,
    pub reserved2: [u32; 3],
    pub reserved3: u16,
    pub itt: u16,

pub const ISCSI_CLEANUP_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CLEANUP_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_CLEANUP_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CLEANUP_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved3: u16,
    pub reserved4: [u32; 10],
    pub cq_index: u8,
    pub reserved6: u8,
    pub reserved5: u16,

    pub reserved5: u16,
    pub reserved6: u8,
    pub cq_index: u8,

}

//
// iSCSI Cleanup CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_cleanup_response {

    pub op_code: u8,
    pub status: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub status: u8,
    pub op_code: u8,
    pub reserved1: [u32; 3],
    pub reserved2: [u32; 2],
    pub reserved4: u16,
    pub err_code: u8,
    pub reserved3: u8,

    pub reserved3: u8,
    pub err_code: u8,
    pub reserved4: u16,
    pub reserved5: [u32; 7],
    pub reserved6: u16,
    pub itt: u16,

pub const ISCSI_CLEANUP_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CLEANUP_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_CLEANUP_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CLEANUP_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved6: u16,

    pub cq_req_sn: u32,
}

//
// SCSI read/write SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_cmd_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_CMD_REQUEST_TASK_ATTR_SHIFT: c_int = 0;

pub const ISCSI_CMD_REQUEST_RESERVED1_SHIFT: c_int = 3;

pub const ISCSI_CMD_REQUEST_WRITE_SHIFT: c_int = 5;

pub const ISCSI_CMD_REQUEST_READ_SHIFT: c_int = 6;

pub const ISCSI_CMD_REQUEST_FINAL_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_attr: u8,

pub const ISCSI_CMD_REQUEST_TASK_ATTR_SHIFT: c_int = 0;

pub const ISCSI_CMD_REQUEST_RESERVED1_SHIFT: c_int = 3;

pub const ISCSI_CMD_REQUEST_WRITE_SHIFT: c_int = 5;

pub const ISCSI_CMD_REQUEST_READ_SHIFT: c_int = 6;

pub const ISCSI_CMD_REQUEST_FINAL_SHIFT: c_int = 7;
    pub op_code: u8,

    pub ud_buffer_offset: u16,
    pub sd_buffer_offset: u16,

    pub sd_buffer_offset: u16,
    pub ud_buffer_offset: u16,
    pub lun: [u32; 2],
    pub reserved2: u16,
    pub itt: u16,

pub const ISCSI_CMD_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CMD_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_CMD_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CMD_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved2: u16,

    pub total_data_transfer_length: u32,
    pub cmd_sn: u32,
    pub reserved3: u32,
    pub cdb: [u32; 4],
    pub zero_fill: u32,
    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub sd_start_bd_index: u8,
    pub ud_start_bd_index: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub ud_start_bd_index: u8,
    pub sd_start_bd_index: u8,
    pub cq_index: u8,

}

//
// task statistics for write response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_write_resp_task_stat {

    pub num_r2ts: u16,
    pub num_data_outs: u16,

    pub num_data_outs: u16,
    pub num_r2ts: u16,

}

//
// task statistics for read response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_read_resp_task_stat {

    pub reserved: u16,
    pub num_data_ins: u16,

    pub num_data_ins: u16,
    pub reserved: u16,

}

//
// task statistics for iSCSI cmd response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2i_cmd_resp_task_stat {
    pub write_stat: bnx2i_write_resp_task_stat,
    pub read_stat: bnx2i_read_resp_task_stat,
}

//
// SCSI Command CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_cmd_response {

    pub op_code: u8,
    pub response_flags: u8,

pub const ISCSI_CMD_RESPONSE_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_CMD_RESPONSE_RESIDUAL_UNDERFLOW_SHIFT: c_int = 1;

pub const ISCSI_CMD_RESPONSE_RESIDUAL_OVERFLOW_SHIFT: c_int = 2;

pub const ISCSI_CMD_RESPONSE_BR_RESIDUAL_UNDERFLOW_SHIFT: c_int = 3;

pub const ISCSI_CMD_RESPONSE_BR_RESIDUAL_OVERFLOW_SHIFT: c_int = 4;

pub const ISCSI_CMD_RESPONSE_RESERVED1_SHIFT: c_int = 5;
    pub response: u8,
    pub status: u8,

    pub status: u8,
    pub response: u8,
    pub response_flags: u8,

pub const ISCSI_CMD_RESPONSE_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_CMD_RESPONSE_RESIDUAL_UNDERFLOW_SHIFT: c_int = 1;

pub const ISCSI_CMD_RESPONSE_RESIDUAL_OVERFLOW_SHIFT: c_int = 2;

pub const ISCSI_CMD_RESPONSE_BR_RESIDUAL_UNDERFLOW_SHIFT: c_int = 3;

pub const ISCSI_CMD_RESPONSE_BR_RESIDUAL_OVERFLOW_SHIFT: c_int = 4;

pub const ISCSI_CMD_RESPONSE_RESERVED1_SHIFT: c_int = 5;
    pub op_code: u8,

    pub data_length: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved2: u32,
    pub residual_count: u32,

    pub reserved4: u16,
    pub err_code: u8,
    pub reserved3: u8,

    pub reserved3: u8,
    pub err_code: u8,
    pub reserved4: u16,
    pub reserved5: [u32; 5],
    pub task_stat: bnx2i_cmd_resp_task_stat,
    pub reserved6: u32,

    pub reserved7: u16,
    pub itt: u16,

pub const ISCSI_CMD_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CMD_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_CMD_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_CMD_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved7: u16,

    pub cq_req_sn: u32,
}

//
// firmware middle-path request SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_fw_mp_request {

    pub op_code: u8,
    pub op_attr: u8,
    pub hdr_opaque1: u16,

    pub hdr_opaque1: u16,
    pub op_attr: u8,
    pub op_code: u8,

    pub data_length: u32,
    pub hdr_opaque2: [u32; 2],
    pub reserved0: u16,
    pub itt: u16,

pub const ISCSI_FW_MP_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_FW_MP_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_FW_MP_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_FW_MP_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved0: u16,
    pub hdr_opaque3: [u32; 4],
    pub resp_bd_list_addr_lo: u32,
    pub resp_bd_list_addr_hi: u32,
    pub resp_buffer: u32,

pub const ISCSI_FW_MP_REQUEST_RESP_BUFFER_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_FW_MP_REQUEST_NUM_RESP_BDS_SHIFT: c_int = 24;

    pub reserved4: u16,
    pub reserved3: u8,
    pub flags: u8,

pub const ISCSI_FW_MP_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_FW_MP_REQUEST_LOCAL_COMPLETION_SHIFT: c_int = 1;

pub const ISCSI_FW_MP_REQUEST_UPDATE_EXP_STAT_SN_SHIFT: c_int = 2;

pub const ISCSI_FW_MP_REQUEST_RESERVED2_SHIFT: c_int = 3;

    pub flags: u8,

pub const ISCSI_FW_MP_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_FW_MP_REQUEST_LOCAL_COMPLETION_SHIFT: c_int = 1;

pub const ISCSI_FW_MP_REQUEST_UPDATE_EXP_STAT_SN_SHIFT: c_int = 2;

pub const ISCSI_FW_MP_REQUEST_RESERVED2_SHIFT: c_int = 3;
    pub reserved3: u8,
    pub reserved4: u16,

    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved6: u8,
    pub reserved5: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved5: u8,
    pub reserved6: u8,
    pub cq_index: u8,

}

//
// firmware response - CQE: used only by firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_fw_response {
    pub hdr_dword1: [u32; 2],
    pub hdr_exp_cmd_sn: u32,
    pub hdr_max_cmd_sn: u32,
    pub hdr_ttt: u32,
    pub hdr_res_cnt: u32,
    pub cqe_flags: u32,

pub const ISCSI_FW_RESPONSE_RESERVED2_SHIFT: c_int = 0;

pub const ISCSI_FW_RESPONSE_ERR_CODE_SHIFT: c_int = 8;

pub const ISCSI_FW_RESPONSE_RESERVED3_SHIFT: c_int = 16;
    pub stat_sn: u32,
    pub hdr_dword2: [u32; 2],
    pub hdr_dword3: [u32; 2],
    pub task_stat: u32,
    pub reserved0: u32,
    pub hdr_itt: u32,
    pub cq_req_sn: u32,
}

//
// iSCSI KCQ CQE parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_kcqe_params {
    pub reserved0: [u32; 4],
}

//
// iSCSI KCQ CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kcqe {
    pub iscsi_conn_id: u32,
    pub completion_status: u32,
    pub iscsi_conn_context_id: u32,
    pub params: iscsi_kcqe_params,

    pub flags: u8,

pub const ISCSI_KCQE_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_KCQE_LAYER_CODE_SHIFT: c_int = 4;

pub const ISCSI_KCQE_RESERVED1_SHIFT: c_int = 7;
    pub op_code: u8,
    pub qe_self_seq: u16,

    pub qe_self_seq: u16,
    pub op_code: u8,
    pub flags: u8,

pub const ISCSI_KCQE_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_KCQE_LAYER_CODE_SHIFT: c_int = 4;

pub const ISCSI_KCQE_RESERVED1_SHIFT: c_int = 7;

}

//
// iSCSI KWQE header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_header {

    pub flags: u8,

pub const ISCSI_KWQE_HEADER_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_KWQE_HEADER_LAYER_CODE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_HEADER_RESERVED1_SHIFT: c_int = 7;
    pub op_code: u8,

    pub op_code: u8,
    pub flags: u8,

pub const ISCSI_KWQE_HEADER_RESERVED0_SHIFT: c_int = 0;

pub const ISCSI_KWQE_HEADER_LAYER_CODE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_HEADER_RESERVED1_SHIFT: c_int = 7;

}

//
// iSCSI firmware init request 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_init1 {

    pub hdr: iscsi_kwqe_header,
    pub reserved0: u8,
    pub num_cqs: u8,

    pub num_cqs: u8,
    pub reserved0: u8,
    pub hdr: iscsi_kwqe_header,

    pub dummy_buffer_addr_lo: u32,
    pub dummy_buffer_addr_hi: u32,

    pub num_ccells_per_conn: u16,
    pub num_tasks_per_conn: u16,

    pub num_tasks_per_conn: u16,
    pub num_ccells_per_conn: u16,

    pub sq_wqes_per_page: u16,
    pub sq_num_wqes: u16,

    pub sq_num_wqes: u16,
    pub sq_wqes_per_page: u16,

    pub cq_log_wqes_per_page: u8,
    pub flags: u8,

pub const ISCSI_KWQE_INIT1_PAGE_SIZE_SHIFT: c_int = 0;

pub const ISCSI_KWQE_INIT1_DELAYED_ACK_ENABLE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_INIT1_KEEP_ALIVE_ENABLE_SHIFT: c_int = 5;

pub const ISCSI_KWQE_INIT1_TIME_STAMPS_ENABLE_SHIFT: c_int = 6;

pub const ISCSI_KWQE_INIT1_RESERVED1_SHIFT: c_int = 7;
    pub cq_num_wqes: u16,

    pub cq_num_wqes: u16,
    pub flags: u8,

pub const ISCSI_KWQE_INIT1_PAGE_SIZE_SHIFT: c_int = 0;

pub const ISCSI_KWQE_INIT1_DELAYED_ACK_ENABLE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_INIT1_KEEP_ALIVE_ENABLE_SHIFT: c_int = 5;

pub const ISCSI_KWQE_INIT1_TIME_STAMPS_ENABLE_SHIFT: c_int = 6;

pub const ISCSI_KWQE_INIT1_RESERVED1_SHIFT: c_int = 7;
    pub cq_log_wqes_per_page: u8,

    pub cq_num_pages: u16,
    pub sq_num_pages: u16,

    pub sq_num_pages: u16,
    pub cq_num_pages: u16,

    pub rq_buffer_size: u16,
    pub rq_num_wqes: u16,

    pub rq_num_wqes: u16,
    pub rq_buffer_size: u16,

}

//
// iSCSI firmware init request 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_init2 {

    pub hdr: iscsi_kwqe_header,
    pub max_cq_sqn: u16,

    pub max_cq_sqn: u16,
    pub hdr: iscsi_kwqe_header,
    pub error_bit_map: [u32; 2],
    pub reserved1: [u32; 5],
}

//
// Initial iSCSI connection offload request 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_conn_offload1 {

    pub hdr: iscsi_kwqe_header,
    pub iscsi_conn_id: u16,

    pub iscsi_conn_id: u16,
    pub hdr: iscsi_kwqe_header,

    pub sq_page_table_addr_lo: u32,
    pub sq_page_table_addr_hi: u32,
    pub cq_page_table_addr_lo: u32,
    pub cq_page_table_addr_hi: u32,
    pub reserved0: [u32; 3],
}

//
// iSCSI Page Table Entry (PTE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_pte {
    pub hi: u32,
    pub lo: u32,
}

//
// Initial iSCSI connection offload request 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_conn_offload2 {

    pub hdr: iscsi_kwqe_header,
    pub reserved0: u16,

    pub reserved0: u16,
    pub hdr: iscsi_kwqe_header,

    pub rq_page_table_addr_lo: u32,
    pub rq_page_table_addr_hi: u32,
    pub sq_first_pte: iscsi_pte,
    pub cq_first_pte: iscsi_pte,
    pub num_additional_wqes: u32,
}

//
// Initial iSCSI connection offload request 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_conn_offload3 {

    pub hdr: iscsi_kwqe_header,
    pub reserved0: u16,

    pub reserved0: u16,
    pub hdr: iscsi_kwqe_header,

    pub reserved1: u32,
    pub qp_first_pte: [iscsi_pte; 3],
}

//
// iSCSI connection update request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_conn_update {

    pub hdr: iscsi_kwqe_header,
    pub reserved0: u16,

    pub reserved0: u16,
    pub hdr: iscsi_kwqe_header,

    pub session_error_recovery_level: u8,
    pub max_outstanding_r2ts: u8,
    pub reserved2: u8,
    pub conn_flags: u8,

pub const ISCSI_KWQE_CONN_UPDATE_HEADER_DIGEST_SHIFT: c_int = 0;

pub const ISCSI_KWQE_CONN_UPDATE_DATA_DIGEST_SHIFT: c_int = 1;

pub const ISCSI_KWQE_CONN_UPDATE_INITIAL_R2T_SHIFT: c_int = 2;

pub const ISCSI_KWQE_CONN_UPDATE_IMMEDIATE_DATA_SHIFT: c_int = 3;

pub const ISCSI_KWQE_CONN_UPDATE_OOO_SUPPORT_MODE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_CONN_UPDATE_RESERVED1_SHIFT: c_int = 6;

    pub conn_flags: u8,

pub const ISCSI_KWQE_CONN_UPDATE_HEADER_DIGEST_SHIFT: c_int = 0;

pub const ISCSI_KWQE_CONN_UPDATE_DATA_DIGEST_SHIFT: c_int = 1;

pub const ISCSI_KWQE_CONN_UPDATE_INITIAL_R2T_SHIFT: c_int = 2;

pub const ISCSI_KWQE_CONN_UPDATE_IMMEDIATE_DATA_SHIFT: c_int = 3;

pub const ISCSI_KWQE_CONN_UPDATE_OOO_SUPPORT_MODE_SHIFT: c_int = 4;

pub const ISCSI_KWQE_CONN_UPDATE_RESERVED1_SHIFT: c_int = 6;
    pub reserved2: u8,
    pub max_outstanding_r2ts: u8,
    pub session_error_recovery_level: u8,

    pub context_id: u32,
    pub max_send_pdu_length: u32,
    pub max_recv_pdu_length: u32,
    pub first_burst_length: u32,
    pub max_burst_length: u32,
    pub exp_stat_sn: u32,
}

//
// iSCSI destroy connection request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_kwqe_conn_destroy {

    pub hdr: iscsi_kwqe_header,
    pub reserved0: u16,

    pub reserved0: u16,
    pub hdr: iscsi_kwqe_header,

    pub context_id: u32,
    pub reserved1: [u32; 6],
}

//
// iSCSI KWQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_kwqe {
    pub init1: iscsi_kwqe_init1,
    pub init2: iscsi_kwqe_init2,
    pub conn_offload1: iscsi_kwqe_conn_offload1,
    pub conn_offload2: iscsi_kwqe_conn_offload2,
    pub conn_update: iscsi_kwqe_conn_update,
    pub conn_destroy: iscsi_kwqe_conn_destroy,
}

//
// iSCSI Login SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_login_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_LOGIN_REQUEST_NEXT_STAGE_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_CURRENT_STAGE_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQUEST_RESERVED0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_REQUEST_CONT_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_REQUEST_TRANSIT_SHIFT: c_int = 7;
    pub version_max: u8,
    pub version_min: u8,

    pub version_min: u8,
    pub version_max: u8,
    pub op_attr: u8,

pub const ISCSI_LOGIN_REQUEST_NEXT_STAGE_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_CURRENT_STAGE_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQUEST_RESERVED0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_REQUEST_CONT_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_REQUEST_TRANSIT_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub isid_lo: u32,

    pub isid_hi: u16,
    pub tsih: u16,

    pub tsih: u16,
    pub isid_hi: u16,

    pub reserved2: u16,
    pub itt: u16,

pub const ISCSI_LOGIN_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_LOGIN_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved2: u16,

    pub cid: u16,
    pub reserved3: u16,

    pub reserved3: u16,
    pub cid: u16,

    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub reserved4: u32,
    pub resp_bd_list_addr_lo: u32,
    pub resp_bd_list_addr_hi: u32,
    pub resp_buffer: u32,

pub const ISCSI_LOGIN_REQUEST_RESP_BUFFER_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_NUM_RESP_BDS_SHIFT: c_int = 24;

    pub reserved8: u16,
    pub reserved7: u8,
    pub flags: u8,

pub const ISCSI_LOGIN_REQUEST_RESERVED5_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_UPDATE_EXP_STAT_SN_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQUEST_RESERVED6_SHIFT: c_int = 3;

    pub flags: u8,

pub const ISCSI_LOGIN_REQUEST_RESERVED5_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQUEST_UPDATE_EXP_STAT_SN_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQUEST_RESERVED6_SHIFT: c_int = 3;
    pub reserved7: u8,
    pub reserved8: u16,

    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved10: u8,
    pub reserved9: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved9: u8,
    pub reserved10: u8,
    pub cq_index: u8,

}

//
// iSCSI Login CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_login_response {

    pub op_code: u8,
    pub response_flags: u8,

pub const ISCSI_LOGIN_RESPONSE_NEXT_STAGE_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_RESPONSE_CURRENT_STAGE_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_RESPONSE_RESERVED0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_RESPONSE_CONT_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_RESPONSE_TRANSIT_SHIFT: c_int = 7;
    pub version_max: u8,
    pub version_active: u8,

    pub version_active: u8,
    pub version_max: u8,
    pub response_flags: u8,

pub const ISCSI_LOGIN_RESPONSE_NEXT_STAGE_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_RESPONSE_CURRENT_STAGE_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_RESPONSE_RESERVED0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_RESPONSE_CONT_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_RESPONSE_TRANSIT_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved1: [u32; 2],
    pub reserved3: u16,
    pub err_code: u8,
    pub reserved2: u8,

    pub reserved2: u8,
    pub err_code: u8,
    pub reserved3: u16,

    pub stat_sn: u32,
    pub isid_lo: u32,

    pub isid_hi: u16,
    pub tsih: u16,

    pub tsih: u16,
    pub isid_hi: u16,

    pub status_class: u8,
    pub status_detail: u8,
    pub reserved4: u16,

    pub reserved4: u16,
    pub status_detail: u8,
    pub status_class: u8,
    pub reserved5: [u32; 3],
    pub reserved6: u16,
    pub itt: u16,

pub const ISCSI_LOGIN_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_LOGIN_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved6: u16,

    pub cq_req_sn: u32,
}

//
// iSCSI Logout SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_logout_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_LOGOUT_REQUEST_REASON_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_attr: u8,

pub const ISCSI_LOGOUT_REQUEST_REASON_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub reserved1: [u32; 2],
    pub reserved2: u16,
    pub itt: u16,

pub const ISCSI_LOGOUT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_LOGOUT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved2: u16,

    pub cid: u16,
    pub reserved3: u16,

    pub reserved3: u16,
    pub cid: u16,

    pub cmd_sn: u32,
    pub reserved4: [u32; 5],
    pub zero_fill: u32,
    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved6: u8,
    pub reserved5: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved5: u8,
    pub reserved6: u8,
    pub cq_index: u8,

}

//
// iSCSI Logout CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_logout_response {

    pub op_code: u8,
    pub reserved1: u8,
    pub response: u8,
    pub reserved0: u8,

    pub reserved0: u8,
    pub response: u8,
    pub reserved1: u8,
    pub op_code: u8,

    pub reserved2: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved3: [u32; 2],
    pub reserved5: u16,
    pub err_code: u8,
    pub reserved4: u8,

    pub reserved4: u8,
    pub err_code: u8,
    pub reserved5: u16,
    pub reserved6: [u32; 3],
    pub time_to_wait: u16,
    pub time_to_retain: u16,

    pub time_to_retain: u16,
    pub time_to_wait: u16,
    pub reserved7: [u32; 3],
    pub reserved8: u16,
    pub itt: u16,

pub const ISCSI_LOGOUT_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_LOGOUT_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved8: u16,

    pub cq_req_sn: u32,
}

//
// iSCSI Nop-In CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_nop_in_msg {

    pub op_code: u8,
    pub reserved1: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub reserved1: u8,
    pub op_code: u8,

    pub data_length: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub ttt: u32,
    pub reserved2: u32,

    pub reserved4: u16,
    pub err_code: u8,
    pub reserved3: u8,

    pub reserved3: u8,
    pub err_code: u8,
    pub reserved4: u16,

    pub reserved5: u32,
    pub lun: [u32; 2],
    pub reserved6: [u32; 4],
    pub reserved7: u16,
    pub itt: u16,

pub const ISCSI_NOP_IN_MSG_INDEX_SHIFT: c_int = 0;

pub const ISCSI_NOP_IN_MSG_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_NOP_IN_MSG_INDEX_SHIFT: c_int = 0;

pub const ISCSI_NOP_IN_MSG_TYPE_SHIFT: c_int = 14;
    pub reserved7: u16,

    pub cq_req_sn: u32,
}

//
// iSCSI NOP-OUT SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_nop_out_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_NOP_OUT_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_attr: u8,

pub const ISCSI_NOP_OUT_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub lun: [u32; 2],
    pub reserved2: u16,
    pub itt: u16,

pub const ISCSI_NOP_OUT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_NOP_OUT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved2: u16,

    pub ttt: u32,
    pub cmd_sn: u32,
    pub reserved3: [u32; 2],
    pub resp_bd_list_addr_lo: u32,
    pub resp_bd_list_addr_hi: u32,
    pub resp_buffer: u32,

pub const ISCSI_NOP_OUT_REQUEST_RESP_BUFFER_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_NUM_RESP_BDS_SHIFT: c_int = 24;

    pub reserved7: u16,
    pub reserved6: u8,
    pub flags: u8,

pub const ISCSI_NOP_OUT_REQUEST_RESERVED4_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_LOCAL_COMPLETION_SHIFT: c_int = 1;

pub const ISCSI_NOP_OUT_REQUEST_ZERO_FILL_SHIFT: c_int = 2;

    pub flags: u8,

pub const ISCSI_NOP_OUT_REQUEST_RESERVED4_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_REQUEST_LOCAL_COMPLETION_SHIFT: c_int = 1;

pub const ISCSI_NOP_OUT_REQUEST_ZERO_FILL_SHIFT: c_int = 2;
    pub reserved6: u8,
    pub reserved7: u16,

    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved9: u8,
    pub reserved8: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved8: u8,
    pub reserved9: u8,
    pub cq_index: u8,

}

//
// iSCSI Reject CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_reject_msg {

    pub op_code: u8,
    pub reserved1: u8,
    pub reason: u8,
    pub reserved0: u8,

    pub reserved0: u8,
    pub reason: u8,
    pub reserved1: u8,
    pub op_code: u8,

    pub data_length: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved2: [u32; 2],
    pub reserved4: u16,
    pub err_code: u8,
    pub reserved3: u8,

    pub reserved3: u8,
    pub err_code: u8,
    pub reserved4: u16,
    pub reserved5: [u32; 8],
    pub cq_req_sn: u32,
}

//
// bnx2i iSCSI TMF SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_tmf_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_TMF_REQUEST_FUNCTION_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_attr: u8,

pub const ISCSI_TMF_REQUEST_FUNCTION_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQUEST_ALWAYS_ONE_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub lun: [u32; 2],
    pub reserved1: u16,
    pub itt: u16,

pub const ISCSI_TMF_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_TMF_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved1: u16,

    pub ref_itt: u32,
    pub cmd_sn: u32,
    pub reserved2: u32,
    pub ref_cmd_sn: u32,
    pub reserved3: [u32; 3],
    pub zero_fill: u32,
    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved5: u8,
    pub reserved4: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved4: u8,
    pub reserved5: u8,
    pub cq_index: u8,

}

//
// iSCSI Text SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_text_request {

    pub op_code: u8,
    pub op_attr: u8,

pub const ISCSI_TEXT_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQUEST_CONT_SHIFT: c_int = 6;

pub const ISCSI_TEXT_REQUEST_FINAL_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_attr: u8,

pub const ISCSI_TEXT_REQUEST_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQUEST_CONT_SHIFT: c_int = 6;

pub const ISCSI_TEXT_REQUEST_FINAL_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub lun: [u32; 2],
    pub reserved3: u16,
    pub itt: u16,

pub const ISCSI_TEXT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQUEST_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_TEXT_REQUEST_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQUEST_TYPE_SHIFT: c_int = 14;
    pub reserved3: u16,

    pub ttt: u32,
    pub cmd_sn: u32,
    pub reserved4: [u32; 2],
    pub resp_bd_list_addr_lo: u32,
    pub resp_bd_list_addr_hi: u32,
    pub resp_buffer: u32,

pub const ISCSI_TEXT_REQUEST_RESP_BUFFER_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQUEST_NUM_RESP_BDS_SHIFT: c_int = 24;
    pub zero_fill: u32,
    pub bd_list_addr_lo: u32,
    pub bd_list_addr_hi: u32,

    pub cq_index: u8,
    pub reserved7: u8,
    pub reserved6: u8,
    pub num_bds: u8,

    pub num_bds: u8,
    pub reserved6: u8,
    pub reserved7: u8,
    pub cq_index: u8,

}

//
// iSCSI SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_request {
    pub cmd: bnx2i_cmd_request,
    pub tmf: bnx2i_tmf_request,
    pub nop_out: bnx2i_nop_out_request,
    pub login_req: bnx2i_login_request,
    pub text: bnx2i_text_request,
    pub logout_req: bnx2i_logout_request,
    pub cleanup: bnx2i_cleanup_request,
}

//
// iSCSI TMF CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_tmf_response {

    pub op_code: u8,
    pub reserved1: u8,
    pub response: u8,
    pub reserved0: u8,

    pub reserved0: u8,
    pub response: u8,
    pub reserved1: u8,
    pub op_code: u8,

    pub reserved2: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub reserved3: [u32; 2],
    pub reserved5: u16,
    pub err_code: u8,
    pub reserved4: u8,

    pub reserved4: u8,
    pub err_code: u8,
    pub reserved5: u16,
    pub reserved6: [u32; 7],
    pub reserved7: u16,
    pub itt: u16,

pub const ISCSI_TMF_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TMF_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_TMF_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TMF_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved7: u16,

    pub cq_req_sn: u32,
}

//
// iSCSI Text CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_text_response {

    pub op_code: u8,
    pub response_flags: u8,

pub const ISCSI_TEXT_RESPONSE_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_RESPONSE_CONT_SHIFT: c_int = 6;

pub const ISCSI_TEXT_RESPONSE_FINAL_SHIFT: c_int = 7;
    pub reserved0: u16,

    pub reserved0: u16,
    pub response_flags: u8,

pub const ISCSI_TEXT_RESPONSE_RESERVED1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_RESPONSE_CONT_SHIFT: c_int = 6;

pub const ISCSI_TEXT_RESPONSE_FINAL_SHIFT: c_int = 7;
    pub op_code: u8,

    pub data_length: u32,
    pub exp_cmd_sn: u32,
    pub max_cmd_sn: u32,
    pub ttt: u32,
    pub reserved2: u32,

    pub reserved4: u16,
    pub err_code: u8,
    pub reserved3: u8,

    pub reserved3: u8,
    pub err_code: u8,
    pub reserved4: u16,

    pub reserved5: u32,
    pub lun: [u32; 2],
    pub reserved6: [u32; 4],
    pub reserved7: u16,
    pub itt: u16,

pub const ISCSI_TEXT_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TEXT_RESPONSE_TYPE_SHIFT: c_int = 14;

    pub itt: u16,

pub const ISCSI_TEXT_RESPONSE_INDEX_SHIFT: c_int = 0;

pub const ISCSI_TEXT_RESPONSE_TYPE_SHIFT: c_int = 14;
    pub reserved7: u16,

    pub cq_req_sn: u32,
}

//
// iSCSI CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_response {
    pub cmd: bnx2i_cmd_response,
    pub tmf: bnx2i_tmf_response,
    pub login_resp: bnx2i_login_response,
    pub text: bnx2i_text_response,
    pub logout_resp: bnx2i_logout_response,
    pub cleanup: bnx2i_cleanup_response,
    pub reject: bnx2i_reject_msg,
    pub async: bnx2i_async_msg,
    pub nop_in: bnx2i_nop_in_msg,
}
