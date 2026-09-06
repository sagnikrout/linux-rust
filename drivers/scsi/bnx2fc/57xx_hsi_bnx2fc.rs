//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2fc/57xx_hsi_bnx2fc.h
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


// 57xx_hsi_bnx2fc.h: QLogic Linux FCoE offload driver.
// Handles operations such as session offload/upload etc, and manages
// session resources such as connection id and qp resources.
//
// Copyright (c) 2008-2013 Broadcom Corporation
// Copyright (c) 2014-2016 QLogic Corporation
// Copyright (c) 2016-2017 Cavium Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// common data for all protocols
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b577xx_doorbell_hdr {
    pub header: u8,

pub const B577XX_DOORBELL_HDR_RX_SHIFT: c_int = 0;

pub const B577XX_DOORBELL_HDR_DB_TYPE_SHIFT: c_int = 1;

pub const B577XX_DOORBELL_HDR_DPM_SIZE_SHIFT: c_int = 2;

pub const B577XX_DOORBELL_HDR_CONN_TYPE_SHIFT: c_int = 4;
}

//
// doorbell message sent to the chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b577xx_doorbell {

    pub zero_fill2: u16,
    pub zero_fill1: u8,
    pub header: b577xx_doorbell_hdr,

    pub header: b577xx_doorbell_hdr,
    pub zero_fill1: u8,
    pub zero_fill2: u16,

}

//
// doorbell message sent to the chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b577xx_doorbell_set_prod {

    pub prod: u16,
    pub zero_fill1: u8,
    pub header: b577xx_doorbell_hdr,

    pub header: b577xx_doorbell_hdr,
    pub zero_fill1: u8,
    pub prod: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regpair {
    pub lo: __le32,
    pub hi: __le32,
}

//
// ABTS info $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_abts_info {
    pub aborted_task_id: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

//
// Fixed size structure in order to plant it in Union structure
// $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_abts_rsp_union {
    pub r_ctl: u8,
    pub rsrv: [u8; 3],
    pub abts_rsp_payload: [__le32; 7],
}

//
// 4 regs size $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_bd_ctx {
    pub buf_addr_hi: __le32,
    pub buf_addr_lo: __le32,
    pub buf_len: __le16,
    pub rsrv0: __le16,
    pub flags: __le16,
    pub rsrv1: __le16,
}

//
// FCoE cached sges context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cached_sge_ctx {
    pub cur_buf_addr: regpair,
    pub cur_buf_rem: __le16,
    pub second_buf_rem: __le16,
    pub second_buf_addr: regpair,
}

//
// Cleanup info $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cleanup_info {
    pub cleaned_task_id: __le16,
    pub rolled_tx_seq_cnt: __le16,
    pub rolled_tx_data_offset: __le32,
}

//
// Fcp RSP flags $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_rsp_flags {
    pub flags: u8,

pub const FCOE_FCP_RSP_FLAGS_FCP_RSP_LEN_VALID_SHIFT: c_int = 0;

pub const FCOE_FCP_RSP_FLAGS_FCP_SNS_LEN_VALID_SHIFT: c_int = 1;

pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_OVER_SHIFT: c_int = 2;

pub const FCOE_FCP_RSP_FLAGS_FCP_RESID_UNDER_SHIFT: c_int = 3;

pub const FCOE_FCP_RSP_FLAGS_FCP_CONF_REQ_SHIFT: c_int = 4;

pub const FCOE_FCP_RSP_FLAGS_FCP_BIDI_FLAGS_SHIFT: c_int = 5;
}

//
// Fcp RSP payload $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_rsp_payload {
    pub reserved0: regpair,
    pub fcp_resid: __le32,
    pub scsi_status_code: u8,
    pub fcp_flags: fcoe_fcp_rsp_flags,
    pub retry_delay_timer: __le16,
    pub fcp_rsp_len: __le32,
    pub fcp_sns_len: __le32,
}

//
// Fixed size structure in order to plant it in Union structure
// $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_rsp_union {
    pub payload: fcoe_fcp_rsp_payload,
    pub reserved0: regpair,
}

//
// FC header $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fc_hdr {
    pub s_id: [u8; 3],
    pub cs_ctl: u8,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub parameters: __le32,
    pub rx_id: __le16,
    pub ox_id: __le16,
}

//
// FC header union $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_mp_rsp_union {
    pub fc_hdr: fcoe_fc_hdr,
    pub mp_payload_len: __le32,
    pub rsrv: __le32,
}

//
// Completion information $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_comp_flow_info {
    pub fcp_rsp: fcoe_fcp_rsp_union,
    pub abts_rsp: fcoe_abts_rsp_union,
    pub mp_rsp: fcoe_mp_rsp_union,
    pub opaque: [__le32; 8],
}

//
// External ABTS info $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ext_abts_info {
    pub rsrv0: [__le32; 6],
    pub ctx: fcoe_abts_info,
}

//
// External cleanup info $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ext_cleanup_info {
    pub rsrv0: [__le32; 6],
    pub ctx: fcoe_cleanup_info,
}

//
// Fcoe FW Tx sequence context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fw_tx_seq_ctx {
    pub data_offset: __le32,
    pub seq_cnt: __le16,
    pub rsrv0: __le16,
}

//
// Fcoe external FW Tx sequence context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ext_fw_tx_seq_ctx {
    pub rsrv0: [__le32; 6],
    pub ctx: fcoe_fw_tx_seq_ctx,
}

//
// FCoE multiple sges context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_mul_sges_ctx {
    pub cur_sge_addr: regpair,
    pub cur_sge_off: __le16,
    pub cur_sge_idx: u8,
    pub sgl_size: u8,
}

//
// FCoE external multiple sges context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ext_mul_sges_ctx {
    pub mul_sgl: fcoe_mul_sges_ctx,
    pub rsrv0: regpair,
}

//
// FCP CMD payload $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_cmd_payload {
    pub opaque: [__le32; 8],
}

//
// Fcp xfr rdy payload $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_xfr_rdy_payload {
    pub burst_len: __le32,
    pub data_ro: __le32,
}

//
// FC frame $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fc_frame {
    pub fc_hdr: fcoe_fc_hdr,
    pub reserved0: [__le32; 2],
}

//
// FCoE KCQ CQE parameters $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_kcqe_params {
    pub reserved0: [__le32; 4],
}

//
// FCoE KCQ CQE $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kcqe {
    pub fcoe_conn_id: __le32,
    pub completion_status: __le32,
    pub fcoe_conn_context_id: __le32,
    pub params: fcoe_kcqe_params,
    pub qe_self_seq: __le16,
    pub op_code: u8,
    pub flags: u8,

pub const FCOE_KCQE_RESERVED0_SHIFT: c_int = 0;

pub const FCOE_KCQE_RAMROD_COMPLETION_SHIFT: c_int = 3;

pub const FCOE_KCQE_LAYER_CODE_SHIFT: c_int = 4;

pub const FCOE_KCQE_LINKED_WITH_NEXT_SHIFT: c_int = 7;
}

//
// FCoE KWQE header $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_header {
    pub op_code: u8,
    pub flags: u8,

pub const FCOE_KWQE_HEADER_RESERVED0_SHIFT: c_int = 0;

pub const FCOE_KWQE_HEADER_LAYER_CODE_SHIFT: c_int = 4;

pub const FCOE_KWQE_HEADER_RESERVED1_SHIFT: c_int = 7;
}

//
// FCoE firmware init request 1 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_init1 {
    pub num_tasks: __le16,
    pub hdr: fcoe_kwqe_header,
    pub task_list_pbl_addr_lo: __le32,
    pub task_list_pbl_addr_hi: __le32,
    pub dummy_buffer_addr_lo: __le32,
    pub dummy_buffer_addr_hi: __le32,
    pub sq_num_wqes: __le16,
    pub rq_num_wqes: __le16,
    pub rq_buffer_log_size: __le16,
    pub cq_num_wqes: __le16,
    pub mtu: __le16,
    pub num_sessions_log: u8,
    pub flags: u8,

pub const FCOE_KWQE_INIT1_LOG_PAGE_SIZE_SHIFT: c_int = 0;

pub const FCOE_KWQE_INIT1_LOG_CACHED_PBES_PER_FUNC_SHIFT: c_int = 4;

pub const FCOE_KWQE_INIT1_RESERVED1_SHIFT: c_int = 7;
}

//
// FCoE firmware init request 2 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_init2 {
    pub hsi_major_version: u8,
    pub hsi_minor_version: u8,
    pub hdr: fcoe_kwqe_header,
    pub hash_tbl_pbl_addr_lo: __le32,
    pub hash_tbl_pbl_addr_hi: __le32,
    pub t2_hash_tbl_addr_lo: __le32,
    pub t2_hash_tbl_addr_hi: __le32,
    pub t2_ptr_hash_tbl_addr_lo: __le32,
    pub t2_ptr_hash_tbl_addr_hi: __le32,
    pub free_list_count: __le32,
}

//
// FCoE firmware init request 3 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_init3 {
    pub reserved0: __le16,
    pub hdr: fcoe_kwqe_header,
    pub error_bit_map_lo: __le32,
    pub error_bit_map_hi: __le32,
    pub perf_config: u8,
    pub reserved21: [u8; 3],
    pub reserved2: [__le32; 4],
}

//
// FCoE connection offload request 1 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_offload1 {
    pub fcoe_conn_id: __le16,
    pub hdr: fcoe_kwqe_header,
    pub sq_addr_lo: __le32,
    pub sq_addr_hi: __le32,
    pub rq_pbl_addr_lo: __le32,
    pub rq_pbl_addr_hi: __le32,
    pub rq_first_pbe_addr_lo: __le32,
    pub rq_first_pbe_addr_hi: __le32,
    pub rq_prod: __le16,
    pub reserved0: __le16,
}

//
// FCoE connection offload request 2 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_offload2 {
    pub tx_max_fc_pay_len: __le16,
    pub hdr: fcoe_kwqe_header,
    pub cq_addr_lo: __le32,
    pub cq_addr_hi: __le32,
    pub xferq_addr_lo: __le32,
    pub xferq_addr_hi: __le32,
    pub conn_db_addr_lo: __le32,
    pub conn_db_addr_hi: __le32,
    pub reserved1: __le32,
}

//
// FCoE connection offload request 3 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_offload3 {
    pub vlan_tag: __le16,

pub const FCOE_KWQE_CONN_OFFLOAD3_VLAN_ID_SHIFT: c_int = 0;

pub const FCOE_KWQE_CONN_OFFLOAD3_CFI_SHIFT: c_int = 12;

pub const FCOE_KWQE_CONN_OFFLOAD3_PRIORITY_SHIFT: c_int = 13;
    pub hdr: fcoe_kwqe_header,
    pub s_id: [u8; 3],
    pub tx_max_conc_seqs_c3: u8,
    pub d_id: [u8; 3],
    pub flags: u8,

pub const FCOE_KWQE_CONN_OFFLOAD3_B_MUL_N_PORT_IDS_SHIFT: c_int = 0;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_E_D_TOV_RES_SHIFT: c_int = 1;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_CONT_INCR_SEQ_CNT_SHIFT: c_int = 2;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_CONF_REQ_SHIFT: c_int = 3;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_REC_VALID_SHIFT: c_int = 4;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_C2_VALID_SHIFT: c_int = 5;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_ACK_0_SHIFT: c_int = 6;

pub const FCOE_KWQE_CONN_OFFLOAD3_B_VLAN_FLAG_SHIFT: c_int = 7;
    pub reserved: __le32,
    pub confq_first_pbe_addr_lo: __le32,
    pub confq_first_pbe_addr_hi: __le32,
    pub tx_total_conc_seqs: __le16,
    pub rx_max_fc_pay_len: __le16,
    pub rx_total_conc_seqs: __le16,
    pub rx_max_conc_seqs_c3: u8,
    pub rx_open_seqs_exch_c3: u8,
}

//
// FCoE connection offload request 4 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_offload4 {
    pub e_d_tov_timer_val: u8,
    pub reserved2: u8,
    pub hdr: fcoe_kwqe_header,
    pub src_mac_addr_lo: [u8; 2],
    pub src_mac_addr_mid: [u8; 2],
    pub src_mac_addr_hi: [u8; 2],
    pub dst_mac_addr_hi: [u8; 2],
    pub dst_mac_addr_lo: [u8; 2],
    pub dst_mac_addr_mid: [u8; 2],
    pub lcq_addr_lo: __le32,
    pub lcq_addr_hi: __le32,
    pub confq_pbl_base_addr_lo: __le32,
    pub confq_pbl_base_addr_hi: __le32,
}

//
// FCoE connection enable request $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_enable_disable {
    pub reserved0: __le16,
    pub hdr: fcoe_kwqe_header,
    pub src_mac_addr_lo: [u8; 2],
    pub src_mac_addr_mid: [u8; 2],
    pub src_mac_addr_hi: [u8; 2],
    pub vlan_tag: u16,

pub const FCOE_KWQE_CONN_ENABLE_DISABLE_VLAN_ID_SHIFT: c_int = 0;

pub const FCOE_KWQE_CONN_ENABLE_DISABLE_CFI_SHIFT: c_int = 12;

pub const FCOE_KWQE_CONN_ENABLE_DISABLE_PRIORITY_SHIFT: c_int = 13;
    pub dst_mac_addr_lo: [u8; 2],
    pub dst_mac_addr_mid: [u8; 2],
    pub dst_mac_addr_hi: [u8; 2],
    pub reserved1: __le16,
    pub s_id: [u8; 3],
    pub vlan_flag: u8,
    pub d_id: [u8; 3],
    pub reserved3: u8,
    pub context_id: __le32,
    pub conn_id: __le32,
    pub reserved4: __le32,
}

//
// FCoE connection destroy request $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_conn_destroy {
    pub reserved0: __le16,
    pub hdr: fcoe_kwqe_header,
    pub context_id: __le32,
    pub conn_id: __le32,
    pub reserved1: [__le32; 5],
}

//
// FCoe destroy request $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_destroy {
    pub reserved0: __le16,
    pub hdr: fcoe_kwqe_header,
    pub reserved1: [__le32; 7],
}

//
// FCoe statistics request $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_kwqe_stat {
    pub reserved0: __le16,
    pub hdr: fcoe_kwqe_header,
    pub stat_params_addr_lo: __le32,
    pub stat_params_addr_hi: __le32,
    pub reserved1: [__le32; 5],
}

//
// FCoE KWQ WQE $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_kwqe {
    pub init1: fcoe_kwqe_init1,
    pub init2: fcoe_kwqe_init2,
    pub init3: fcoe_kwqe_init3,
    pub conn_offload1: fcoe_kwqe_conn_offload1,
    pub conn_offload2: fcoe_kwqe_conn_offload2,
    pub conn_offload3: fcoe_kwqe_conn_offload3,
    pub conn_offload4: fcoe_kwqe_conn_offload4,
    pub conn_enable_disable: fcoe_kwqe_conn_enable_disable,
    pub conn_destroy: fcoe_kwqe_conn_destroy,
    pub destroy: fcoe_kwqe_destroy,
    pub statistics: fcoe_kwqe_stat,
}

//
// TX SGL context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_sgl_union_ctx {
    pub cached_sge: fcoe_cached_sge_ctx,
    pub sgl: fcoe_ext_mul_sges_ctx,
    pub opaque: [__le32; 5],
}

//
// Data-In/ELS/BLS information $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_read_flow_info {
    pub sgl_ctx: fcoe_sgl_union_ctx,
    pub rsrv0: [__le32; 3],
}

//
// Fcoe stat context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_s_stat_ctx {
    pub flags: u8,

pub const FCOE_S_STAT_CTX_ACTIVE_SHIFT: c_int = 0;

pub const FCOE_S_STAT_CTX_ACK_ABORT_SEQ_COND_SHIFT: c_int = 1;

pub const FCOE_S_STAT_CTX_ABTS_PERFORMED_SHIFT: c_int = 2;

pub const FCOE_S_STAT_CTX_SEQ_TIMEOUT_SHIFT: c_int = 3;

pub const FCOE_S_STAT_CTX_P_RJT_SHIFT: c_int = 4;

pub const FCOE_S_STAT_CTX_ACK_EOFT_SHIFT: c_int = 5;

pub const FCOE_S_STAT_CTX_RSRV1_SHIFT: c_int = 6;
}

//
// Fcoe rx seq context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_seq_ctx {
    pub seq_id: u8,
    pub s_stat: fcoe_s_stat_ctx,
    pub seq_cnt: __le16,
    pub low_exp_ro: __le32,
    pub high_exp_ro: __le32,
}

//
// Fcoe rx_wr union context $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_rx_wr_union_ctx {
    pub read_info: fcoe_read_flow_info,
    pub comp_info: fcoe_comp_flow_info,
    pub opaque: [__le32; 8],
}

//
// FCoE SQ element $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_sqe {
    pub wqe: __le16,

pub const FCOE_SQE_TASK_ID_SHIFT: c_int = 0;

pub const FCOE_SQE_TOGGLE_BIT_SHIFT: c_int = 15;
}

//
// 14 regs $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_tx_only {
    pub sgl_ctx: fcoe_sgl_union_ctx,
    pub rsrv0: __le32,
}

//
// 32 bytes (8 regs) used for TX only purposes $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_tx_wr_rx_rd_union_ctx {
    pub tx_frame: fcoe_fc_frame,
    pub fcp_cmd: fcoe_fcp_cmd_payload,
    pub cleanup: fcoe_ext_cleanup_info,
    pub abts: fcoe_ext_abts_info,
    pub tx_seq: fcoe_ext_fw_tx_seq_ctx,
    pub opaque: [__le32; 8],
}

//
// tce_tx_wr_rx_rd_const $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_tx_wr_rx_rd_const {
    pub init_flags: u8,

pub const FCOE_TCE_TX_WR_RX_RD_CONST_TASK_TYPE_SHIFT: c_int = 0;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_DEV_TYPE_SHIFT: c_int = 3;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_CLASS_TYPE_SHIFT: c_int = 4;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_CACHED_SGE_SHIFT: c_int = 5;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_SUPPORT_REC_TOV_SHIFT: c_int = 7;
    pub tx_flags: u8,

pub const FCOE_TCE_TX_WR_RX_RD_CONST_TX_VALID_SHIFT: c_int = 0;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_TX_STATE_SHIFT: c_int = 1;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_RSRV1_SHIFT: c_int = 5;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_TX_SEQ_INIT_SHIFT: c_int = 6;

pub const FCOE_TCE_TX_WR_RX_RD_CONST_RSRV2_SHIFT: c_int = 7;
    pub rsrv3: __le16,
    pub verify_tx_seq: __le32,
}

//
// tce_tx_wr_rx_rd $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_tx_wr_rx_rd {
    pub union_ctx: fcoe_tx_wr_rx_rd_union_ctx,
    pub const_ctx: fcoe_tce_tx_wr_rx_rd_const,
}

//
// tce_rx_wr_tx_rd_const $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_rx_wr_tx_rd_const {
    pub data_2_trns: __le32,
    pub init_flags: __le32,

pub const FCOE_TCE_RX_WR_TX_RD_CONST_CID_SHIFT: c_int = 0;

pub const FCOE_TCE_RX_WR_TX_RD_CONST_RSRV0_SHIFT: c_int = 24;
}

//
// tce_rx_wr_tx_rd_var $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_rx_wr_tx_rd_var {
    pub rx_flags: __le16,

pub const FCOE_TCE_RX_WR_TX_RD_VAR_RSRV1_SHIFT: c_int = 0;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_NUM_RQ_WQE_SHIFT: c_int = 4;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_CONF_REQ_SHIFT: c_int = 7;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_RX_STATE_SHIFT: c_int = 8;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_EXP_FIRST_FRAME_SHIFT: c_int = 12;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_RX_SEQ_INIT_SHIFT: c_int = 13;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_RSRV2_SHIFT: c_int = 14;

pub const FCOE_TCE_RX_WR_TX_RD_VAR_RX_VALID_SHIFT: c_int = 15;
    pub rx_id: __le16,
    pub fcp_xfr_rdy: fcoe_fcp_xfr_rdy_payload,
}

//
// tce_rx_wr_tx_rd $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_rx_wr_tx_rd {
    pub const_ctx: fcoe_tce_rx_wr_tx_rd_const,
    pub var_ctx: fcoe_tce_rx_wr_tx_rd_var,
}

//
// tce_rx_only $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tce_rx_only {
    pub rx_seq_ctx: fcoe_rx_seq_ctx,
    pub union_ctx: fcoe_rx_wr_union_ctx,
}

//
// task_ctx_entry $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_task_ctx_entry {
    pub txwr_only: fcoe_tce_tx_only,
    pub txwr_rxrd: fcoe_tce_tx_wr_rx_rd,
    pub rxwr_txrd: fcoe_tce_rx_wr_tx_rd,
    pub rxwr_only: fcoe_tce_rx_only,
}

//
// FCoE XFRQ element $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_xfrqe {
    pub wqe: __le16,

pub const FCOE_XFRQE_TASK_ID_SHIFT: c_int = 0;

pub const FCOE_XFRQE_TOGGLE_BIT_SHIFT: c_int = 15;
}

//
// fcoe rx doorbell message sent to the chip $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b577xx_fcoe_rx_doorbell {
    pub hdr: b577xx_doorbell_hdr,
    pub params: u8,

pub const B577XX_FCOE_RX_DOORBELL_NEGATIVE_ARM_SHIFT: c_int = 0;

pub const B577XX_FCOE_RX_DOORBELL_OPCODE_SHIFT: c_int = 5;
    pub doorbell_cq_cons: __le16,
}

//
// FCoE CONFQ element $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_confqe {
    pub ox_id: __le16,
    pub rx_id: __le16,
    pub param: __le32,
}

//
// FCoE connection data base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_db {

    pub rsrv0: u16,
    pub rq_prod: u16,

    pub rq_prod: u16,
    pub rsrv0: u16,

    pub rsrv1: u32,
    pub cq_arm: regpair,
}

//
// FCoE CQ element $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cqe {
    pub wqe: __le16,

pub const FCOE_CQE_CQE_INFO_SHIFT: c_int = 0;

pub const FCOE_CQE_CQE_TYPE_SHIFT: c_int = 14;

pub const FCOE_CQE_TOGGLE_BIT_SHIFT: c_int = 15;
}

//
// FCoE error/warning reporting entry $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_partial_err_report_entry {
    pub err_warn_bitmap_lo: __le32,
    pub err_warn_bitmap_hi: __le32,
    pub tx_buf_off: __le32,
    pub rx_buf_off: __le32,
}

//
// FCoE error/warning reporting entry $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_err_report_entry {
    pub data: fcoe_partial_err_report_entry,
    pub fc_hdr: fcoe_fc_hdr,
}

//
// FCoE hash table entry (32 bytes) $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_hash_table_entry {
    pub s_id_0: u8,
    pub s_id_1: u8,
    pub s_id_2: u8,
    pub d_id_0: u8,
    pub d_id_1: u8,
    pub d_id_2: u8,
    pub dst_mac_addr_hi: __le16,
    pub dst_mac_addr_mid: __le16,
    pub dst_mac_addr_lo: __le16,
    pub src_mac_addr_hi: __le16,
    pub vlan_id: __le16,
    pub src_mac_addr_lo: __le16,
    pub src_mac_addr_mid: __le16,
    pub vlan_flag: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub field_id: __le32,

pub const FCOE_HASH_TABLE_ENTRY_CID_SHIFT: c_int = 0;

pub const FCOE_HASH_TABLE_ENTRY_RESERVED3_SHIFT: c_int = 24;

pub const FCOE_HASH_TABLE_ENTRY_VALID_SHIFT: c_int = 31;
}

//
// FCoE LCQ element $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_lcqe {
    pub wqe: __le32,

pub const FCOE_LCQE_TASK_ID_SHIFT: c_int = 0;

pub const FCOE_LCQE_LCQE_TYPE_SHIFT: c_int = 16;

pub const FCOE_LCQE_RESERVED_SHIFT: c_int = 24;
}

//
// FCoE pending work request CQE $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_pend_wq_cqe {
    pub wqe: __le16,

pub const FCOE_PEND_WQ_CQE_TASK_ID_SHIFT: c_int = 0;

pub const FCOE_PEND_WQ_CQE_CQE_TYPE_SHIFT: c_int = 14;

pub const FCOE_PEND_WQ_CQE_TOGGLE_BIT_SHIFT: c_int = 15;
}

//
// FCoE RX statistics parameters section#0 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section0 {
    pub fcoe_rx_pkt_cnt: __le32,
    pub fcoe_rx_byte_cnt: __le32,
}

//
// FCoE RX statistics parameters section#1 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section1 {
    pub fcoe_ver_cnt: __le32,
    pub fcoe_rx_drop_pkt_cnt: __le32,
}

//
// FCoE RX statistics parameters section#2 $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat_params_section2 {
    pub fc_crc_cnt: __le32,
    pub eofa_del_cnt: __le32,
    pub miss_frame_cnt: __le32,
    pub seq_timeout_cnt: __le32,
    pub drop_seq_cnt: __le32,
    pub fcoe_rx_drop_pkt_cnt: __le32,
    pub fcp_rx_pkt_cnt: __le32,
    pub reserved0: __le32,
}

//
// FCoE TX statistics parameters $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_stat_params {
    pub fcoe_tx_pkt_cnt: __le32,
    pub fcoe_tx_byte_cnt: __le32,
    pub fcp_tx_pkt_cnt: __le32,
    pub reserved0: __le32,
}

//
// FCoE statistics parameters $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_statistics_params {
    pub tx_stat: fcoe_tx_stat_params,
    pub rx_stat0: fcoe_rx_stat_params_section0,
    pub rx_stat1: fcoe_rx_stat_params_section1,
    pub rx_stat2: fcoe_rx_stat_params_section2,
}

//
// FCoE t2 hash table entry (64 bytes) $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_t2_hash_table_entry {
    pub data: fcoe_hash_table_entry,
    pub next: regpair,
    pub reserved0: [regpair; 3],
}

//
// FCoE unsolicited CQE $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_unsolicited_cqe {
    pub wqe: __le16,

pub const FCOE_UNSOLICITED_CQE_SUBTYPE_SHIFT: c_int = 0;

pub const FCOE_UNSOLICITED_CQE_PKT_LEN_SHIFT: c_int = 2;

pub const FCOE_UNSOLICITED_CQE_CQE_TYPE_SHIFT: c_int = 14;

pub const FCOE_UNSOLICITED_CQE_TOGGLE_BIT_SHIFT: c_int = 15;
}
