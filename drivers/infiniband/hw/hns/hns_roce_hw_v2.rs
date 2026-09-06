//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_hw_v2.h
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


//
// Copyright (c) 2016-2017 Hisilicon Limited.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const HNS_ROCE_V2_MAX_RC_INL_INN_SZ: c_int = 32;
pub const HNS_ROCE_V2_MTT_ENTRY_SZ: c_int = 64;
pub const HNS_ROCE_V2_AEQE_VEC_NUM: c_int = 1;
pub const HNS_ROCE_V2_ABNORMAL_VEC_NUM: c_int = 1;
pub const HNS_ROCE_V2_MAX_SRQWQE_SEGS: c_uint = 0x1000000;
pub const HNS_ROCE_V2_MAX_IDX_SEGS: c_uint = 0x1000000;
pub const HNS_ROCE_V2_MAX_XRCD_NUM: c_uint = 0x1000000;
pub const HNS_ROCE_V2_QP_ACK_TIMEOUT_OFS_HIP08: c_int = 10;
pub const HNS_ROCE_V3_SCCC_SZ: c_int = 64;
pub const HNS_ROCE_V3_GMV_ENTRY_SZ: c_int = 32;
pub const HNS_ROCE_V2_EXT_LLM_ENTRY_SZ: c_int = 8;
pub const HNS_ROCE_V2_EXT_LLM_MAX_DEPTH: c_int = 4096;

pub const HNS_ROCE_V2_PAGE_SIZE_SUPPORTED: c_uint = 0xFFFF000;
pub const HNS_ROCE_V2_MAX_INNER_MTPT_NUM: c_int = 2;
pub const HNS_ROCE_INVALID_LKEY: c_uint = 0x0;
pub const HNS_ROCE_INVALID_SGE_LENGTH: c_uint = 0x80000000;
pub const HNS_ROCE_CMQ_TX_TIMEOUT: c_int = 30000;
pub const HNS_ROCE_V2_RSV_QPS: c_int = 8;
pub const HNS_ROCE_V2_HW_RST_TIMEOUT: c_int = 1000;
pub const HNS_ROCE_V2_HW_RST_UNINT_DELAY: c_int = 100;
pub const HNS_ROCE_V2_HW_RST_COMPLETION_WAIT: c_int = 20;
pub const HNS_ROCE_CONTEXT_HOP_NUM: c_int = 1;
pub const HNS_ROCE_SCCC_HOP_NUM: c_int = 1;
pub const HNS_ROCE_MTT_HOP_NUM: c_int = 1;
pub const HNS_ROCE_CQE_HOP_NUM: c_int = 1;
pub const HNS_ROCE_SRQWQE_HOP_NUM: c_int = 1;
pub const HNS_ROCE_PBL_HOP_NUM: c_int = 2;
pub const HNS_ROCE_IDX_HOP_NUM: c_int = 1;
pub const HNS_ROCE_SQWQE_HOP_NUM: c_int = 2;
pub const HNS_ROCE_EXT_SGE_HOP_NUM: c_int = 1;
pub const HNS_ROCE_RQWQE_HOP_NUM: c_int = 2;
pub const HNS_ROCE_V2_EQE_HOP_NUM: c_int = 2;
pub const HNS_ROCE_V3_EQE_HOP_NUM: c_int = 1;
pub const HNS_ROCE_BA_PG_SZ_SUPPORTED_256K: c_int = 6;
pub const HNS_ROCE_BA_PG_SZ_SUPPORTED_16K: c_int = 2;
pub const HNS_ROCE_V2_GID_INDEX_NUM: c_int = 16;

// budget must be smaller than aeqe_depth to guarantee that we update
// the ci before we polled all the entries in the EQ.
//
pub const HNS_AEQ_POLLING_BUDGET: c_int = 64;
pub const HNS_ROCE_CMQ_DESC_NUM_S: c_int = 3;
pub const HNS_ROCE_CMQ_SCC_CLR_DONE_CNT: c_int = 5;
pub const HNS_ROCE_CONG_SIZE: c_int = 64;

pub const HNS_ICL_SWITCH_CMD_ROCEE_SEL_SHIFT: c_int = 0;

pub const CMD_CSQ_DESC_NUM: c_int = 1024;
pub const CMD_CRQ_DESC_NUM: c_int = 1024;
// Free mr used parameters
pub const HNS_ROCE_FREE_MR_USED_CQE_NUM: c_int = 128;
pub const HNS_ROCE_FREE_MR_USED_QP_NUM: c_uint = 0x8;
pub const HNS_ROCE_FREE_MR_USED_PSN: c_uint = 0x0808;
pub const HNS_ROCE_FREE_MR_USED_QP_RETRY_CNT: c_uint = 0x7;
pub const HNS_ROCE_FREE_MR_USED_QP_TIMEOUT: c_uint = 0x12;
pub const HNS_ROCE_FREE_MR_USED_SQWQE_NUM: c_int = 128;
pub const HNS_ROCE_FREE_MR_USED_SQSGE_NUM: c_uint = 0x2;
pub const HNS_ROCE_FREE_MR_USED_RQWQE_NUM: c_int = 128;
pub const HNS_ROCE_FREE_MR_USED_RQSGE_NUM: c_uint = 0x2;
pub const HNS_ROCE_V2_FREE_MR_TIMEOUT: c_int = 4500;
pub const V2_CQ_DB_REQ_NOT_SOL: c_int = 0;
pub const V2_CQ_DB_REQ_NOT: c_int = 1;
pub const V2_CQ_STATE_VALID: c_int = 1;
pub const V2_QKEY_VAL: c_uint = 0x80010000;
pub const GID_LEN_V2: c_int = 16;
// rq operations
// CMQ command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_opcode_type {
    HNS_QUERY_FW_VER				= 0x0001,
    HNS_ROCE_OPC_CFG_DCQCN_PARAM			= 0x1A80,
    HNS_ROCE_OPC_CFG_LDCP_PARAM			= 0x1A81,
    HNS_ROCE_OPC_CFG_HC3_PARAM			= 0x1A82,
    HNS_ROCE_OPC_CFG_DIP_PARAM			= 0x1A83,
    HNS_ROCE_OPC_QUERY_HW_VER			= 0x8000,
    HNS_ROCE_OPC_CFG_GLOBAL_PARAM			= 0x8001,
    HNS_ROCE_OPC_ALLOC_PF_RES			= 0x8004,
    HNS_ROCE_OPC_QUERY_COUNTER			= 0x8206,
    HNS_ROCE_OPC_QUERY_PF_RES			= 0x8400,
    HNS_ROCE_OPC_ALLOC_VF_RES			= 0x8401,
    HNS_ROCE_OPC_CFG_EXT_LLM			= 0x8403,
    HNS_ROCE_OPC_QUERY_PF_TIMER_RES			= 0x8406,
    HNS_ROCE_OPC_QUERY_FUNC_INFO			= 0x8407,
    HNS_ROCE_OPC_QUERY_PF_CAPS_NUM                  = 0x8408,
    HNS_ROCE_OPC_CFG_ENTRY_SIZE			= 0x8409,
    HNS_ROCE_OPC_QUERY_VF_CAPS_NUM			= 0x8410,
    HNS_ROCE_OPC_CFG_SGID_TB			= 0x8500,
    HNS_ROCE_OPC_CFG_SMAC_TB			= 0x8501,
    HNS_ROCE_OPC_POST_MB				= 0x8504,
    HNS_ROCE_OPC_QUERY_MB_ST			= 0x8505,
    HNS_ROCE_OPC_CFG_BT_ATTR			= 0x8506,
    HNS_ROCE_OPC_FUNC_CLEAR				= 0x8508,
    HNS_ROCE_OPC_CLR_SCCC				= 0x8509,
    HNS_ROCE_OPC_QUERY_SCCC				= 0x850a,
    HNS_ROCE_OPC_RESET_SCCC				= 0x850b,
    HNS_ROCE_OPC_CLEAR_EXTDB_LIST_INFO		= 0x850d,
    HNS_ROCE_OPC_QUERY_VF_RES			= 0x850e,
    HNS_ROCE_OPC_CFG_GMV_TBL			= 0x850f,
    HNS_ROCE_OPC_CFG_GMV_BT				= 0x8510,
    HNS_ROCE_QUERY_RAM_ECC				= 0x8513,
    HNS_SWITCH_PARAMETER_CFG			= 0x1033,
    HNS_ROCE_OPC_SET_BOND_INFO                      = 0x8601,
    HNS_ROCE_OPC_CLEAR_BOND_INFO                    = 0x8602,
    HNS_ROCE_OPC_CHANGE_ACTIVE_PORT                 = 0x8603,
}

pub const HNS_ROCE_OPC_POST_MB_TIMEOUT: c_int = 35000;
pub const HNS_ROCE_OPC_POST_MB_TRY_CNT: c_int = 8;
pub const HNS_ROCE_OPC_POST_MB_RETRY_GAP_MSEC: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmdq_tx_timeout_map {
    pub opcode: u16,
    pub tx_timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_cmd_return_status {
    CMD_EXEC_SUCCESS,
    CMD_NO_AUTH,
    CMD_NOT_EXIST,
    CMD_CRQ_FULL,
    CMD_NEXT_ERR,
    CMD_NOT_EXEC,
    CMD_PARA_ERR,
    CMD_RESULT_ERR,
    CMD_TIMEOUT,
    CMD_HILINK_ERR,
    CMD_INFO_ILLEGAL,
    CMD_INVALID,
    CMD_ROH_CHECK_FAIL,
    CMD_OTHER_ERR = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmd_errcode {
    pub return_status: hns_roce_cmd_return_status,
    pub errno: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_sgid_type {
    GID_TYPE_FLAG_ROCE_V1 = 0,
    GID_TYPE_FLAG_ROCE_V2_IPV4,
    GID_TYPE_FLAG_ROCE_V2_IPV6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_cq_context {
    pub byte_4_pg_ceqn: __le32,
    pub byte_8_cqn: __le32,
    pub cqe_cur_blk_addr: __le32,
    pub byte_16_hop_addr: __le32,
    pub cqe_nxt_blk_addr: __le32,
    pub byte_24_pgsz_addr: __le32,
    pub byte_28_cq_pi: __le32,
    pub byte_32_cq_ci: __le32,
    pub cqe_ba: __le32,
    pub byte_40_cqe_ba: __le32,
    pub byte_44_db_record: __le32,
    pub db_record_addr: __le32,
    pub byte_52_cqe_cnt: __le32,
    pub byte_56_cqe_period_maxcnt: __le32,
    pub cqe_report_timer: __le32,
    pub byte_64_se_cqe_idx: __le32,
}

pub const CQC_CQE_BA_L_S: c_int = 3;

pub const CQC_CQE_DB_RECORD_ADDR_H_S: c_int = 32;
pub const HNS_ROCE_V2_CQ_DEFAULT_BURST_NUM: c_uint = 0x0;
pub const HNS_ROCE_V2_CQ_DEFAULT_INTERVAL: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_srq_context {
    pub data: [__le32; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_v2_qp_state {
    HNS_ROCE_QP_ST_RST,
    HNS_ROCE_QP_ST_INIT,
    HNS_ROCE_QP_ST_RTR,
    HNS_ROCE_QP_ST_RTS,
    HNS_ROCE_QP_ST_SQD,
    HNS_ROCE_QP_ST_SQER,
    HNS_ROCE_QP_ST_ERR,
    HNS_ROCE_QP_ST_SQ_DRAINING,
    HNS_ROCE_QP_NUM_ST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_qp_context_ex {
    pub data: [__le32; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_qp_context {
    pub byte_4_sqpn_tst: __le32,
    pub wqe_sge_ba: __le32,
    pub byte_12_sq_hop: __le32,
    pub byte_16_buf_ba_pg_sz: __le32,
    pub byte_20_smac_sgid_idx: __le32,
    pub byte_24_mtu_tc: __le32,
    pub byte_28_at_fl: __le32,
    pub dgid: [u8; GID_LEN_V2],
    pub dmac: __le32,
    pub byte_52_udpspn_dmac: __le32,
    pub byte_56_dqpn_err: __le32,
    pub byte_60_qpst_tempid: __le32,
    pub qkey_xrcd: __le32,
    pub byte_68_rq_db: __le32,
    pub rq_db_record_addr: __le32,
    pub byte_76_srqn_op_en: __le32,
    pub byte_80_rnr_rx_cqn: __le32,
    pub byte_84_rq_ci_pi: __le32,
    pub rq_cur_blk_addr: __le32,
    pub byte_92_srq_info: __le32,
    pub byte_96_rx_reqmsn: __le32,
    pub rq_nxt_blk_addr: __le32,
    pub byte_104_rq_sge: __le32,
    pub byte_108_rx_reqepsn: __le32,
    pub rq_rnr_timer: __le32,
    pub rx_msg_len: __le32,
    pub rx_rkey_pkt_info: __le32,
    pub rx_va: __le64,
    pub byte_132_trrl: __le32,
    pub trrl_ba: __le32,
    pub byte_140_raq: __le32,
    pub byte_144_raq: __le32,
    pub byte_148_raq: __le32,
    pub byte_152_raq: __le32,
    pub byte_156_raq: __le32,
    pub byte_160_sq_ci_pi: __le32,
    pub sq_cur_blk_addr: __le32,
    pub byte_168_irrl_idx: __le32,
    pub byte_172_sq_psn: __le32,
    pub byte_176_msg_pktn: __le32,
    pub sq_cur_sge_blk_addr: __le32,
    pub byte_184_irrl_idx: __le32,
    pub cur_sge_offset: __le32,
    pub byte_192_ext_sge: __le32,
    pub byte_196_sq_psn: __le32,
    pub byte_200_sq_max: __le32,
    pub irrl_ba: __le32,
    pub byte_208_irrl: __le32,
    pub byte_212_lsn: __le32,
    pub sq_timer: __le32,
    pub byte_220_retry_psn_msn: __le32,
    pub byte_224_retry_msg: __le32,
    pub rx_sq_cur_blk_addr: __le32,
    pub byte_232_irrl_sge: __le32,
    pub irrl_cur_sge_offset: __le32,
    pub byte_240_irrl_tail: __le32,
    pub byte_244_rnr_rxack: __le32,
    pub byte_248_ack_psn: __le32,
    pub byte_252_err_txcqn: __le32,
    pub byte_256_sqflush_rqcqe: __le32,
    pub ext: hns_roce_v2_qp_context_ex,
}

pub const QPC_TRRL_BA_L_S: c_int = 4;

pub const QPC_IRRL_BA_L_S: c_int = 6;

pub const RETRY_MSG_PSN_SHIFT: c_int = 16;

pub const SCC_CONTEXT_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_scc_context {
    pub data: [__le32; SCC_CONTEXT_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_cqe {
    pub byte_4: __le32,
    pub rkey: __le32,
    pub immtdata: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_mpt_entry {
    pub byte_4_pd_hop_st: __le32,
    pub byte_8_mw_cnt_en: __le32,
    pub byte_12_mw_pa: __le32,
    pub bound_lkey: __le32,
    pub len_l: __le32,
    pub len_h: __le32,
    pub lkey: __le32,
    pub va_l: __le32,
    pub va_h: __le32,
    pub pbl_size: __le32,
    pub pbl_ba_l: __le32,
    pub byte_48_mode_ba: __le32,
    pub pa0_l: __le32,
    pub byte_56_pa0_h: __le32,
    pub pa1_l: __le32,
    pub byte_64_buf_pa1: __le32,
}

pub const MPT_PBL_BUF_ADDR_S: c_int = 6;
pub const MPT_PBL_BA_ADDR_S: c_int = 3;

pub const V2_MPT_BYTE_4_MPT_ST_S: c_int = 0;

pub const V2_MPT_BYTE_4_PBL_HOP_NUM_S: c_int = 2;

pub const V2_MPT_BYTE_4_PBL_BA_PG_SZ_S: c_int = 4;

pub const V2_MPT_BYTE_4_PD_S: c_int = 8;

pub const V2_MPT_BYTE_8_RA_EN_S: c_int = 0;
pub const V2_MPT_BYTE_8_R_INV_EN_S: c_int = 1;
pub const V2_MPT_BYTE_8_L_INV_EN_S: c_int = 2;
pub const V2_MPT_BYTE_8_BIND_EN_S: c_int = 3;
pub const V2_MPT_BYTE_8_ATOMIC_EN_S: c_int = 4;
pub const V2_MPT_BYTE_8_RR_EN_S: c_int = 5;
pub const V2_MPT_BYTE_8_RW_EN_S: c_int = 6;
pub const V2_MPT_BYTE_8_LW_EN_S: c_int = 7;
pub const V2_MPT_BYTE_12_FRE_S: c_int = 0;
pub const V2_MPT_BYTE_12_PA_S: c_int = 1;
pub const V2_MPT_BYTE_12_BPD_S: c_int = 5;
pub const V2_MPT_BYTE_12_BQP_S: c_int = 6;
pub const V2_MPT_BYTE_12_INNER_PA_VLD_S: c_int = 7;
pub const V2_MPT_BYTE_48_PBL_BA_H_S: c_int = 0;

pub const V2_MPT_BYTE_48_BLK_MODE_S: c_int = 29;
pub const V2_MPT_BYTE_56_PA0_H_S: c_int = 0;

pub const V2_MPT_BYTE_64_PA1_H_S: c_int = 0;

pub const V2_MPT_BYTE_64_PBL_BUF_PG_SZ_S: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_db {
    pub data: [__le32; 2],
}

pub const V2_DB_PRODUCER_IDX_S: c_int = 0;

pub const V2_CQ_DB_CONS_IDX_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_ud_send_wqe {
    pub byte_4: __le32,
    pub msg_len: __le32,
    pub immtdata: __le32,
    pub byte_16: __le32,
    pub byte_20: __le32,
    pub byte_24: __le32,
    pub qkey: __le32,
    pub byte_32: __le32,
    pub byte_36: __le32,
    pub byte_40: __le32,
    pub dmac: [u8; ETH_ALEN],
    pub sgid_index: u8,
    pub smac_index: u8,
    pub dgid: [u8; GID_LEN_V2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_rc_send_wqe {
    pub byte_4: __le32,
    pub msg_len: __le32,
    pub inv_key: __le32,
    pub immtdata: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_wqe_frmr_seg {
    pub pbl_size: __le32,
    pub byte_40: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_wqe_data_seg {
    pub len: __le32,
    pub lkey: __le32,
    pub addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_version {
    pub rocee_vendor_id: __le16,
    pub rocee_hw_version: __le16,
    pub rsv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_fw_info {
    pub fw_ver: __le32,
    pub rsv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_func_clear {
    pub rst_funcid_en: __le32,
    pub func_done: __le32,
    pub rsv: [__le32; 4],
}

// Each physical function manages up to 248 virtual functions, it takes up to
// 100ms for each function to execute clear. If an abnormal reset occurs, it is
// executed twice at most, so it takes up to 249 * 2 * 100ms.
//

pub const HNS_ROCE_V2_READ_FUNC_CLEAR_FLAG_INTERVAL: c_int = 40;
pub const HNS_ROCE_V2_READ_FUNC_CLEAR_FLAG_FAIL_WAIT: c_int = 20;

// Fields of HNS_ROCE_OPC_CFG_GLOBAL_PARAM

//
// Fields of HNS_ROCE_OPC_QUERY_PF_RES, HNS_ROCE_OPC_QUERY_VF_RES
// and HNS_ROCE_OPC_ALLOC_VF_RES
//

// Fields of HNS_ROCE_OPC_QUERY_PF_TIMER_RES

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_vf_switch {
    pub rocee_sel: __le32,
    pub fun_id: __le32,
    pub cfg: __le32,
    pub resv1: __le32,
    pub resv2: __le32,
    pub resv3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_post_mbox {
    pub in_param_l: __le32,
    pub in_param_h: __le32,
    pub out_param_l: __le32,
    pub out_param_h: __le32,
    pub cmd_tag: __le32,
    pub token_event_en: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_mbox_status {
    pub mb_status_hw_run: __le32,
    pub rsv: [__le32; 5],
}

pub const HNS_ROCE_V2_GO_BIT_TIMEOUT_MSECS: c_int = 10000;

pub const MB_ST_COMPLETE_SUCC: c_int = 1;
// Fields of HNS_ROCE_OPC_CFG_BT_ATTR

// Fields of HNS_ROCE_OPC_CFG_ENTRY_SIZE

// Fields of HNS_ROCE_OPC_CFG_GMV_BT

// Fields of HNS_ROCE_QUERY_RAM_ECC

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cfg_sgid_tb {
    pub table_idx_rsv: __le32,
    pub vf_sgid_l: __le32,
    pub vf_sgid_ml: __le32,
    pub vf_sgid_mh: __le32,
    pub vf_sgid_h: __le32,
    pub vf_sgid_type_rsv: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cfg_smac_tb {
    pub tb_idx_rsv: __le32,
    pub vf_smac_l: __le32,
    pub vf_smac_h_rsv: __le32,
    pub rsv: [__le32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cfg_gmv_tb_a {
    pub vf_sgid_l: __le32,
    pub vf_sgid_ml: __le32,
    pub vf_sgid_mh: __le32,
    pub vf_sgid_h: __le32,
    pub vf_sgid_type_vlan: __le32,
    pub resv: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cfg_gmv_tb_b {
    pub vf_smac_l: __le32,
    pub vf_smac_h: __le32,
    pub table_idx_rsv: __le32,
    pub resv: [__le32; 3],
}

pub const HNS_ROCE_QUERY_PF_CAPS_CMD_NUM_HIP08: c_int = 5;
pub const HNS_ROCE_QUERY_PF_CAPS_CMD_NUM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_a {
    pub number_ports: u8,
    pub local_ca_ack_delay: u8,
    pub max_sq_sg: __le16,
    pub max_sq_inline: __le16,
    pub max_rq_sg: __le16,
    pub rsv0: __le32,
    pub num_qpc_timer: __le16,
    pub num_cqc_timer: __le16,
    pub max_srq_sges: __le16,
    pub num_aeq_vectors: u8,
    pub num_other_vectors: u8,
    pub max_sq_desc_sz: u8,
    pub max_rq_desc_sz: u8,
    pub rsv1: u8,
    pub cqe_sz: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_b {
    pub mtpt_entry_sz: u8,
    pub irrl_entry_sz: u8,
    pub trrl_entry_sz: u8,
    pub cqc_entry_sz: u8,
    pub srqc_entry_sz: u8,
    pub idx_entry_sz: u8,
    pub sccc_sz: u8,
    pub max_mtu: u8,
    pub qpc_sz: __le16,
    pub qpc_timer_entry_sz: __le16,
    pub cqc_timer_entry_sz: __le16,
    pub min_cqes: u8,
    pub min_wqes: u8,
    pub page_size_cap: __le32,
    pub pkey_table_len: u8,
    pub phy_num_uars: u8,
    pub ctx_hop_num: u8,
    pub pbl_hop_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_c {
    pub cap_flags_num_pds: __le32,
    pub max_gid_num_cqs: __le32,
    pub cq_depth: __le32,
    pub num_mrws: __le32,
    pub ord_num_qps: __le32,
    pub sq_depth: __le16,
    pub rq_depth: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_d {
    pub wq_hop_num_max_srqs: __le32,
    pub srq_depth: __le16,
    pub cap_flags_ex: __le16,
    pub num_ceqs_ceq_depth: __le32,
    pub arm_st_aeq_depth: __le32,
    pub num_uars_rsv_pds: __le32,
    pub rsv_uars_rsv_qps: __le32,
}

pub const HNS_ROCE_CAP_FLAGS_EX_SHIFT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_congestion_algorithm {
    pub alg_sel: u8,
    pub alg_sub_sel: u8,
    pub dip_vld: u8,
    pub wnd_mode_sel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_e {
    pub chunk_size_shift_rsv_mrws: __le32,
    pub rsv_cqs: __le32,
    pub rsv_srqs: __le32,
    pub rsv_lkey: __le32,
    pub ceq_max_cnt: __le16,
    pub ceq_period: __le16,
    pub aeq_max_cnt: __le16,
    pub aeq_period: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_query_pf_caps_f {
    pub max_ack_req_msg_len: __le32,
    pub rsv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmq_req {
    pub data: [__le32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmq_desc {
    pub opcode: __le16,
    pub flag: __le16,
    pub retval: __le16,
    pub rsv: __le16,
    pub data: [__le32; 6],
    pub own_func_num: __le32,
    pub own_mac_id: __le32,
    pub rsv: [__le32; 4],
    pub func_info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_cmq_ring {
    pub desc_dma_addr: dma_addr_t,
    pub desc: *mut hns_roce_cmq_desc,
    pub head: u32,
    pub buf_size: u16,
    pub desc_num: u16,
    pub flag: u8,
    pub /: *mut *mut spinlock_t lock; / command queue lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_cmq {
    pub csq: hns_roce_v2_cmq_ring,
    pub tx_timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_link_table {
    pub table: hns_roce_buf_list,
    pub buf: *mut hns_roce_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_free_mr {
    pub rsv_qp: [*mut hns_roce_qp; HNS_ROCE_FREE_MR_USED_QP_NUM],
    pub rsv_cq: *mut hns_roce_cq,
    pub rsv_pd: *mut hns_roce_pd,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_v2_priv {
    pub handle: *mut hnae3_handle,
    pub cmq: hns_roce_v2_cmq,
    pub ext_llm: hns_roce_link_table,
    pub free_mr: hns_roce_v2_free_mr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_dip {
    pub dgid: [u8; GID_LEN_V2],
    pub dip_idx: u32,
    pub qp_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmea_ram_ecc {
    pub is_ecc_err: u32,
    pub res_type: u32,
    pub index: u32,
}

// only for RNR timeout issue of HIP08
pub const HNS_ROCE_CLOCK_ADJUST: c_int = 1000;
pub const HNS_ROCE_MAX_CQ_PERIOD_HIP08: c_int = 65;
pub const HNS_ROCE_MAX_EQ_PERIOD: c_int = 65;
pub const HNS_ROCE_RNR_TIMER_10NS: c_int = 1;
pub const HNS_ROCE_1US_CFG: c_int = 999;
pub const HNS_ROCE_1NS_CFG: c_int = 0;
pub const HNS_ROCE_AEQ_DEFAULT_BURST_NUM: c_uint = 0x0;
pub const HNS_ROCE_AEQ_DEFAULT_INTERVAL: c_uint = 0x0;
pub const HNS_ROCE_CEQ_DEFAULT_BURST_NUM: c_uint = 0x0;
pub const HNS_ROCE_CEQ_DEFAULT_INTERVAL: c_uint = 0x0;
pub const HNS_ROCE_V2_EQ_STATE_INVALID: c_int = 0;
pub const HNS_ROCE_V2_EQ_STATE_VALID: c_int = 1;
pub const HNS_ROCE_V2_EQ_STATE_OVERFLOW: c_int = 2;
pub const HNS_ROCE_V2_EQ_STATE_FAILURE: c_int = 3;
pub const HNS_ROCE_V2_EQ_OVER_IGNORE_0: c_int = 0;
pub const HNS_ROCE_V2_EQ_OVER_IGNORE_1: c_int = 1;
pub const HNS_ROCE_V2_EQ_COALESCE_0: c_int = 0;
pub const HNS_ROCE_V2_EQ_COALESCE_1: c_int = 1;
pub const HNS_ROCE_V2_EQ_FIRED: c_int = 0;
pub const HNS_ROCE_V2_EQ_ARMED: c_int = 1;
pub const HNS_ROCE_V2_EQ_ALWAYS_ARMED: c_int = 3;
pub const HNS_ROCE_EQ_INIT_EQE_CNT: c_int = 0;
pub const HNS_ROCE_EQ_INIT_PROD_IDX: c_int = 0;
pub const HNS_ROCE_EQ_INIT_REPORT_TIMER: c_int = 0;
pub const HNS_ROCE_EQ_INIT_MSI_IDX: c_int = 0;
pub const HNS_ROCE_EQ_INIT_CONS_IDX: c_int = 0;
pub const HNS_ROCE_EQ_INIT_NXT_EQE_BA: c_int = 0;
pub const HNS_ROCE_V2_COMP_EQE_NUM: c_uint = 0x1000;
pub const HNS_ROCE_V2_ASYNC_EQE_NUM: c_uint = 0x1000;
pub const HNS_ROCE_V2_VF_INT_ST_AEQ_OVERFLOW_S: c_int = 0;
pub const HNS_ROCE_EQ_DB_CMD_AEQ: c_uint = 0x0;
pub const HNS_ROCE_EQ_DB_CMD_AEQ_ARMED: c_uint = 0x1;
pub const HNS_ROCE_EQ_DB_CMD_CEQ: c_uint = 0x2;
pub const HNS_ROCE_EQ_DB_CMD_CEQ_ARMED: c_uint = 0x3;
pub const EQ_ENABLE: c_int = 1;
pub const EQ_DISABLE: c_int = 0;
pub const EQ_REG_OFFSET: c_uint = 0x4;
pub const HNS_ROCE_INT_NAME_LEN: c_int = 32;

pub const HNS_ROCE_V2_VF_ABN_INT_EN_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_eq_context {
    pub data: [__le32; 16],
}

pub const MAX_SERVICE_LEVEL: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_wqe_atomic_seg {
    pub fetchadd_swap_data: __le64,
    pub cmp_data: __le64,
}

pub const HNS_ROCE_DCQCN_AI_OFS: c_int = 0;

pub const HNS_ROCE_DCQCN_TKP_MAX: c_int = 10;

pub const HNS_ROCE_DCQCN_TMP_MAX: c_int = 15;

pub const HNS_ROCE_DCQCN_G_MAX: c_int = 15;

pub const HNS_ROCE_DCQCN_ASHIFT_MAX: c_int = 15;
pub const HNS_ROCE_LDCP_CWD0_OFS: c_int = 0;

pub const HNS_ROCE_LDCP_GAMMA_MAX: c_int = 7;

pub const HNS_ROCE_LDCP_BETA_MAX: c_int = 7;

pub const HNS_ROCE_LDCP_ETA_MAX: c_int = 7;
pub const HNS_ROCE_HC3_INITIAL_WINDOW_OFS: c_int = 0;

pub const HNS_ROCE_HC3_INITIAL_WINDOW_MIN: c_int = 0;

pub const HNS_ROCE_HC3_BANDWIDTH_MIN: c_int = 1000;

pub const HNS_ROCE_HC3_QLEN_SHIFT_MIN: c_int = 0;
pub const HNS_ROCE_HC3_QLEN_SHIFT_MAX: c_int = 31;

pub const HNS_ROCE_HC3_PORT_USAGE_SHIFT_MIN: c_int = 0;
pub const HNS_ROCE_HC3_PORT_USAGE_SHIFT_MAX: c_int = 100;

pub const HNS_ROCE_HC3_OVER_PERIOD_MIN: c_int = 0;

pub const HNS_ROCE_HC3_MAX_STAGE_MIN: c_int = 0;

pub const HNS_ROCE_HC3_GAMMA_SHIFT_MIN: c_int = 0;
pub const HNS_ROCE_HC3_GAMMA_SHIFT_MAX: c_int = 15;
pub const HNS_ROCE_DIP_AI_OFS: c_int = 0;

pub const HNS_ROCE_DIP_TKP_MAX: c_int = 10;

pub const HNS_ROCE_DIP_TMP_MAX: c_int = 15;

pub const HNS_ROCE_DIP_G_MAX: c_int = 15;

pub const HNS_ROCE_DIP_ASHIFT_MAX: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_sccc_clr {
    pub qpn: __le32,
    pub rsv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_sccc_clr_done {
    pub clr_done: __le32,
    pub rsv: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_bond_info {
    pub bond_id: __le32,
    pub bond_mode: __le32,
    pub active_slave_cnt: __le32,
    pub active_slave_mask: __le32,
    pub slave_mask: __le32,
    pub hash_policy: __le32,
}

// hns_roce_bond_init_client(struct hns_roce_bond_group *bond_grp,
extern "C" {
    pub fn hns_roce_v2_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
