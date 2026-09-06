//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/fcoe_common.h
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
// Copyright (c) 2015 QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//
// FCOE FW CONSTANTS
//
pub const FC_ABTS_REPLY_MAX_PAYLOAD_LEN: c_int = 12;
// The fcoe storm task context protection-information of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protection_info_ctx {
    pub flags: __le16,
pub const PROTECTION_INFO_CTX_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const PROTECTION_INFO_CTX_HOST_INTERFACE_SHIFT: c_int = 0;
pub const PROTECTION_INFO_CTX_DIF_TO_PEER_MASK: c_uint = 0x1;
pub const PROTECTION_INFO_CTX_DIF_TO_PEER_SHIFT: c_int = 2;
pub const PROTECTION_INFO_CTX_VALIDATE_DIX_APP_TAG_MASK: c_uint = 0x1;
pub const PROTECTION_INFO_CTX_VALIDATE_DIX_APP_TAG_SHIFT: c_int = 3;
pub const PROTECTION_INFO_CTX_INTERVAL_SIZE_LOG_MASK: c_uint = 0xF;
pub const PROTECTION_INFO_CTX_INTERVAL_SIZE_LOG_SHIFT: c_int = 4;
pub const PROTECTION_INFO_CTX_VALIDATE_DIX_REF_TAG_MASK: c_uint = 0x1;
pub const PROTECTION_INFO_CTX_VALIDATE_DIX_REF_TAG_SHIFT: c_int = 8;
pub const PROTECTION_INFO_CTX_RESERVED0_MASK: c_uint = 0x7F;
pub const PROTECTION_INFO_CTX_RESERVED0_SHIFT: c_int = 9;
    pub dix_block_size: u8,
    pub dst_size: u8,
}

// The fcoe storm task context protection-information of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub union protection_info_union_ctx {
    pub info: protection_info_ctx,
    pub value: __le32,
}

// FCP CMD payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_cmd_payload {
    pub opaque: [__le32; 8],
}

// FCP RSP payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_rsp_payload {
    pub opaque: [__le32; 6],
}

// FCP RSP payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_rsp_payload_padded {
    pub rsp_payload: fcoe_fcp_rsp_payload,
    pub reserved: [__le32; 2],
}

// FCP RSP payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcp_xfer_payload {
    pub opaque: [__le32; 3],
}

// FCP RSP payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_xfer_payload_padded {
    pub xfer_payload: fcoe_fcp_xfer_payload,
    pub reserved: [__le32; 5],
}

// Task params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_data_params {
    pub data_offset: __le32,
    pub offset_in_io: __le32,
    pub flags: u8,
pub const FCOE_TX_DATA_PARAMS_OFFSET_IN_IO_VALID_MASK: c_uint = 0x1;
pub const FCOE_TX_DATA_PARAMS_OFFSET_IN_IO_VALID_SHIFT: c_int = 0;
pub const FCOE_TX_DATA_PARAMS_DROP_DATA_MASK: c_uint = 0x1;
pub const FCOE_TX_DATA_PARAMS_DROP_DATA_SHIFT: c_int = 1;
pub const FCOE_TX_DATA_PARAMS_AFTER_SEQ_REC_MASK: c_uint = 0x1;
pub const FCOE_TX_DATA_PARAMS_AFTER_SEQ_REC_SHIFT: c_int = 2;
pub const FCOE_TX_DATA_PARAMS_RESERVED0_MASK: c_uint = 0x1F;
pub const FCOE_TX_DATA_PARAMS_RESERVED0_SHIFT: c_int = 3;
    pub dif_residual: u8,
    pub seq_cnt: __le16,
    pub single_sge_saved_offset: __le16,
    pub next_dif_offset: __le16,
    pub seq_id: __le16,
    pub reserved3: __le16,
}

// Middle path parameters: FC header fields provided by the driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_mid_path_params {
    pub parameter: __le32,
    pub r_ctl: u8,
    pub type: u8,
    pub cs_ctl: u8,
    pub df_ctl: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
}

// Task params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_params {
    pub data: fcoe_tx_data_params,
    pub mid_path: fcoe_tx_mid_path_params,
}

// Union of FCP CMD payload \ TX params \ ABTS \ Cleanup
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_tx_info_union_ctx {
    pub fcp_cmd_payload: fcoe_fcp_cmd_payload,
    pub fcp_rsp_payload: fcp_rsp_payload_padded,
    pub fcp_xfer_payload: fcp_xfer_payload_padded,
    pub tx_params: fcoe_tx_params,
}

// Data sgl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_slow_sgl_ctx {
    pub base_sgl_addr: regpair,
    pub curr_sge_off: __le16,
    pub remainder_num_sges: __le16,
    pub curr_sgl_index: __le16,
    pub reserved: __le16,
}

// Union of DIX SGL \ cached DIX sges
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_dix_desc_ctx {
    pub dix_sgl: fcoe_slow_sgl_ctx,
    pub cached_dix_sge: scsi_sge,
}

// The fcoe storm task context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_fcoe_task_st_ctx {
    pub task_type: u8,
    pub sgl_mode: u8,
pub const YSTORM_FCOE_TASK_ST_CTX_TX_SGL_MODE_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_ST_CTX_TX_SGL_MODE_SHIFT: c_int = 0;
pub const YSTORM_FCOE_TASK_ST_CTX_RSRV_MASK: c_uint = 0x7F;
pub const YSTORM_FCOE_TASK_ST_CTX_RSRV_SHIFT: c_int = 1;
    pub cached_dix_sge: u8,
    pub expect_first_xfer: u8,
    pub num_pbf_zero_write: __le32,
    pub protection_info_union: protection_info_union_ctx,
    pub data_2_trns_rem: __le32,
    pub sgl_params: scsi_sgl_params,
    pub reserved1: [u8; 12],
    pub tx_info_union: fcoe_tx_info_union_ctx,
    pub dix_desc: fcoe_dix_desc_ctx,
    pub data_desc: scsi_cached_sges,
    pub ox_id: __le16,
    pub rx_id: __le16,
    pub task_rety_identifier: __le32,
    pub reserved2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_fcoe_task_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub word0: __le16,
    pub flags0: u8,
pub const YSTORM_FCOE_TASK_AG_CTX_NIBBLE0_MASK: c_uint = 0xF;
pub const YSTORM_FCOE_TASK_AG_CTX_NIBBLE0_SHIFT: c_int = 0;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT0_SHIFT: c_int = 4;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT2_SHIFT: c_int = 6;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT3_SHIFT: c_int = 7;
    pub flags1: u8,
pub const YSTORM_FCOE_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_TASK_AG_CTX_CF0_SHIFT: c_int = 0;
pub const YSTORM_FCOE_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const YSTORM_FCOE_TASK_AG_CTX_CF2SPECIAL_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_TASK_AG_CTX_CF2SPECIAL_SHIFT: c_int = 4;
pub const YSTORM_FCOE_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const YSTORM_FCOE_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const YSTORM_FCOE_TASK_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_BIT4_SHIFT: c_int = 0;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 6;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub reg0: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub rx_id: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub word5: __le16,
    pub reg1: __le32,
    pub reg2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_task_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const TSTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const TSTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const TSTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const TSTORM_FCOE_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const TSTORM_FCOE_TASK_AG_CTX_WAIT_ABTS_RSP_F_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_WAIT_ABTS_RSP_F_SHIFT: c_int = 6;
pub const TSTORM_FCOE_TASK_AG_CTX_VALID_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_VALID_SHIFT: c_int = 7;
    pub flags1: u8,
pub const TSTORM_FCOE_TASK_AG_CTX_FALSE_RR_TOV_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_FALSE_RR_TOV_SHIFT: c_int = 0;
pub const TSTORM_FCOE_TASK_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_BIT5_SHIFT: c_int = 1;
pub const TSTORM_FCOE_TASK_AG_CTX_REC_RR_TOV_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_REC_RR_TOV_CF_SHIFT: c_int = 2;
pub const TSTORM_FCOE_TASK_AG_CTX_ED_TOV_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_ED_TOV_CF_SHIFT: c_int = 4;
pub const TSTORM_FCOE_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_FCOE_TASK_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 0;
pub const TSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_SHIFT: c_int = 2;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_INIT_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_INIT_CF_SHIFT: c_int = 4;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_RECOVERY_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_RECOVERY_CF_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_FCOE_TASK_AG_CTX_UNSOL_COMP_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_TASK_AG_CTX_UNSOL_COMP_CF_SHIFT: c_int = 0;
pub const TSTORM_FCOE_TASK_AG_CTX_REC_RR_TOV_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_REC_RR_TOV_CF_EN_SHIFT: c_int = 2;
pub const TSTORM_FCOE_TASK_AG_CTX_ED_TOV_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_ED_TOV_CF_EN_SHIFT: c_int = 3;
pub const TSTORM_FCOE_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_CF2EN_SHIFT: c_int = 4;
pub const TSTORM_FCOE_TASK_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 5;
pub const TSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_INIT_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_INIT_CF_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_RECOVERY_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_SEQ_RECOVERY_CF_EN_SHIFT: c_int = 0;
pub const TSTORM_FCOE_TASK_AG_CTX_UNSOL_COMP_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_UNSOL_COMP_CF_EN_SHIFT: c_int = 1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 2;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 3;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 4;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 5;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 6;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 7;
    pub cleanup_state: u8,
    pub last_sent_tid: __le16,
    pub rec_rr_tov_exp_timeout: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub data_offset_end_of_seq: __le32,
    pub data_offset_next: __le32,
}

// Cached data sges
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_exp_ro {
    pub data_offset: __le32,
    pub reserved: __le32,
}

// Union of Cleanup address \ expected relative offsets
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_cleanup_addr_exp_ro_union {
    pub abts_rsp_fc_payload_hi: regpair,
    pub exp_ro: fcoe_exp_ro,
}

// Fields coppied from ABTSrsp pckt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_abts_pkt {
    pub abts_rsp_fc_payload_lo: __le32,
    pub abts_rsp_rx_id: __le16,
    pub abts_rsp_rctl: u8,
    pub reserved2: u8,
}

// FW read- write (modifyable) part The fcoe task storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tstorm_fcoe_task_st_ctx_read_write {
    pub cleanup_addr_exp_ro_union: fcoe_cleanup_addr_exp_ro_union,
    pub flags: __le16,
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_RX_SGL_MODE_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_RX_SGL_MODE_SHIFT: c_int = 0;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_EXP_FIRST_FRAME_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_EXP_FIRST_FRAME_SHIFT: c_int = 1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SEQ_ACTIVE_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SEQ_ACTIVE_SHIFT: c_int = 2;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SEQ_TIMEOUT_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SEQ_TIMEOUT_SHIFT: c_int = 3;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SINGLE_PKT_IN_EX_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_SINGLE_PKT_IN_EX_SHIFT: c_int = 4;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_OOO_RX_SEQ_STAT_MASK: c_uint = 0x1;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_OOO_RX_SEQ_STAT_SHIFT: c_int = 5;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_CQ_ADD_ADV_MASK: c_uint = 0x3;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_CQ_ADD_ADV_SHIFT: c_int = 6;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_RSRV1_MASK: c_uint = 0xFF;
pub const FCOE_TSTORM_FCOE_TASK_ST_CTX_READ_WRITE_RSRV1_SHIFT: c_int = 8;
    pub seq_cnt: __le16,
    pub seq_id: u8,
    pub ooo_rx_seq_id: u8,
    pub rx_id: __le16,
    pub abts_data: fcoe_abts_pkt,
    pub e_d_tov_exp_timeout_val: __le32,
    pub ooo_rx_seq_cnt: __le16,
    pub reserved1: __le16,
}

// FW read only part The fcoe task storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tstorm_fcoe_task_st_ctx_read_only {
    pub task_type: u8,
    pub dev_type: u8,
    pub conf_supported: u8,
    pub glbl_q_num: u8,
    pub cid: __le32,
    pub fcp_cmd_trns_size: __le32,
    pub rsrv: __le32,
}

// The fcoe task storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_task_st_ctx {
    pub read_write: fcoe_tstorm_fcoe_task_st_ctx_read_write,
    pub read_only: fcoe_tstorm_fcoe_task_st_ctx_read_only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_fcoe_task_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const MSTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const MSTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const MSTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const MSTORM_FCOE_TASK_AG_CTX_CQE_PLACED_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_CQE_PLACED_SHIFT: c_int = 5;
pub const MSTORM_FCOE_TASK_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_BIT2_SHIFT: c_int = 6;
pub const MSTORM_FCOE_TASK_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_BIT3_SHIFT: c_int = 7;
    pub flags1: u8,
pub const MSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_SHIFT: c_int = 0;
pub const MSTORM_FCOE_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const MSTORM_FCOE_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_AG_CTX_CF2_SHIFT: c_int = 4;
pub const MSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_EX_CLEANUP_CF_EN_SHIFT: c_int = 6;
pub const MSTORM_FCOE_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const MSTORM_FCOE_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const MSTORM_FCOE_TASK_AG_CTX_XFER_PLACEMENT_EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_XFER_PLACEMENT_EN_SHIFT: c_int = 6;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub cleanup_state: u8,
    pub received_bytes: __le32,
    pub byte3: u8,
    pub glbl_q_num: u8,
    pub word1: __le16,
    pub tid_to_xfer: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub word5: __le16,
    pub expected_bytes: __le32,
    pub reg2: __le32,
}

// The fcoe task storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_fcoe_task_st_ctx {
    pub rsp_buf_addr: regpair,
    pub rsrv: [__le32; 2],
    pub sgl_params: scsi_sgl_params,
    pub data_2_trns_rem: __le32,
    pub data_buffer_offset: __le32,
    pub parent_id: __le16,
    pub flags: __le16,
pub const MSTORM_FCOE_TASK_ST_CTX_INTERVAL_SIZE_LOG_MASK: c_uint = 0xF;
pub const MSTORM_FCOE_TASK_ST_CTX_INTERVAL_SIZE_LOG_SHIFT: c_int = 0;
pub const MSTORM_FCOE_TASK_ST_CTX_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_ST_CTX_HOST_INTERFACE_SHIFT: c_int = 4;
pub const MSTORM_FCOE_TASK_ST_CTX_DIF_TO_PEER_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_DIF_TO_PEER_SHIFT: c_int = 6;
pub const MSTORM_FCOE_TASK_ST_CTX_MP_INCLUDE_FC_HEADER_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_MP_INCLUDE_FC_HEADER_SHIFT: c_int = 7;
pub const MSTORM_FCOE_TASK_ST_CTX_DIX_BLOCK_SIZE_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_ST_CTX_DIX_BLOCK_SIZE_SHIFT: c_int = 8;
pub const MSTORM_FCOE_TASK_ST_CTX_VALIDATE_DIX_REF_TAG_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_VALIDATE_DIX_REF_TAG_SHIFT: c_int = 10;
pub const MSTORM_FCOE_TASK_ST_CTX_DIX_CACHED_SGE_FLG_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_DIX_CACHED_SGE_FLG_SHIFT: c_int = 11;
pub const MSTORM_FCOE_TASK_ST_CTX_DIF_SUPPORTED_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_DIF_SUPPORTED_SHIFT: c_int = 12;
pub const MSTORM_FCOE_TASK_ST_CTX_TX_SGL_MODE_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_TASK_ST_CTX_TX_SGL_MODE_SHIFT: c_int = 13;
pub const MSTORM_FCOE_TASK_ST_CTX_RESERVED_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_TASK_ST_CTX_RESERVED_SHIFT: c_int = 14;
    pub data_desc: scsi_cached_sges,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_task_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const USTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const USTORM_FCOE_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const USTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const USTORM_FCOE_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const USTORM_FCOE_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_FCOE_TASK_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_FCOE_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_FCOE_TASK_AG_CTX_CF1_SHIFT: c_int = 0;
pub const USTORM_FCOE_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_FCOE_TASK_AG_CTX_CF2_SHIFT: c_int = 2;
pub const USTORM_FCOE_TASK_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_FCOE_TASK_AG_CTX_CF3_SHIFT: c_int = 4;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_CF_MASK: c_uint = 0x3;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_FCOE_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_FCOE_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_FCOE_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_FCOE_TASK_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_CF_EN_SHIFT: c_int = 4;
pub const USTORM_FCOE_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 5;
pub const USTORM_FCOE_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 6;
pub const USTORM_FCOE_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_FCOE_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 0;
pub const USTORM_FCOE_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 2;
pub const USTORM_FCOE_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 3;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_TYPE_MASK: c_uint = 0xF;
pub const USTORM_FCOE_TASK_AG_CTX_DIF_ERROR_TYPE_SHIFT: c_int = 4;
    pub dif_err_intervals: __le32,
    pub dif_error_1st_interval: __le32,
    pub global_cq_num: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
}

// FCoE task context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_task_context {
    pub ystorm_st_context: ystorm_fcoe_task_st_ctx,
    pub ystorm_st_padding: [regpair; 2],
    pub tdif_context: tdif_task_context,
    pub ystorm_ag_context: ystorm_fcoe_task_ag_ctx,
    pub tstorm_ag_context: tstorm_fcoe_task_ag_ctx,
    pub timer_context: timers_context,
    pub tstorm_st_context: tstorm_fcoe_task_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
    pub mstorm_ag_context: mstorm_fcoe_task_ag_ctx,
    pub mstorm_st_context: mstorm_fcoe_task_st_ctx,
    pub ustorm_ag_context: ustorm_fcoe_task_ag_ctx,
    pub rdif_context: rdif_task_context,
}

// FCoE additional WQE (Sq/XferQ) information
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_additional_info_union {
    pub previous_tid: __le32,
    pub parent_tid: __le32,
    pub burst_length: __le32,
    pub seq_rec_updated_offset: __le32,
}

// FCoE Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_completion_status {
    FCOE_COMPLETION_STATUS_SUCCESS,
    FCOE_COMPLETION_STATUS_FCOE_VER_ERR,
    FCOE_COMPLETION_STATUS_SRC_MAC_ADD_ARR_ERR,
    MAX_FCOE_COMPLETION_STATUS
}

// FC address (SID/DID) network presentation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_addr_nw {
    pub addr_lo: u8,
    pub addr_mid: u8,
    pub addr_hi: u8,
}

// FCoE connection offload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_offload_ramrod_data {
    pub sq_pbl_addr: regpair,
    pub sq_curr_page_addr: regpair,
    pub sq_next_page_addr: regpair,
    pub xferq_pbl_addr: regpair,
    pub xferq_curr_page_addr: regpair,
    pub xferq_next_page_addr: regpair,
    pub respq_pbl_addr: regpair,
    pub respq_curr_page_addr: regpair,
    pub respq_next_page_addr: regpair,
    pub dst_mac_addr_lo: __le16,
    pub dst_mac_addr_mid: __le16,
    pub dst_mac_addr_hi: __le16,
    pub src_mac_addr_lo: __le16,
    pub src_mac_addr_mid: __le16,
    pub src_mac_addr_hi: __le16,
    pub tx_max_fc_pay_len: __le16,
    pub e_d_tov_timer_val: __le16,
    pub rx_max_fc_pay_len: __le16,
    pub vlan_tag: __le16,
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_VLAN_ID_MASK: c_uint = 0xFFF;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_VLAN_ID_SHIFT: c_int = 0;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_CFI_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_CFI_SHIFT: c_int = 12;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_PRIORITY_MASK: c_uint = 0x7;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_PRIORITY_SHIFT: c_int = 13;
    pub physical_q0: __le16,
    pub rec_rr_tov_timer_val: __le16,
    pub s_id: fc_addr_nw,
    pub max_conc_seqs_c3: u8,
    pub d_id: fc_addr_nw,
    pub flags: u8,
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_CONT_INCR_SEQ_CNT_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_CONT_INCR_SEQ_CNT_SHIFT: c_int = 0;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_CONF_REQ_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_CONF_REQ_SHIFT: c_int = 1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_REC_VALID_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_REC_VALID_SHIFT: c_int = 2;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_VLAN_FLAG_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_VLAN_FLAG_SHIFT: c_int = 3;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_SINGLE_VLAN_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_B_SINGLE_VLAN_SHIFT: c_int = 4;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_MODE_MASK: c_uint = 0x3;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_MODE_SHIFT: c_int = 5;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const FCOE_CONN_OFFLOAD_RAMROD_DATA_RESERVED0_SHIFT: c_int = 7;
    pub conn_id: __le16,
    pub def_q_idx: u8,
    pub reserved: [u8; 5],
}

// FCoE terminate connection request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_terminate_ramrod_data {
    pub terminate_params_addr: regpair,
}

// FCoE device type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_device_type {
    FCOE_TASK_DEV_TYPE_DISK,
    FCOE_TASK_DEV_TYPE_TAPE,
    MAX_FCOE_DEVICE_TYPE
}

// Data sgl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fast_sgl_ctx {
    pub sgl_start_addr: regpair,
    pub sgl_byte_offset: __le32,
    pub task_reuse_cnt: __le16,
    pub init_offset_in_first_sge: __le16,
}

// FCoE firmware function init
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_init_func_ramrod_data {
    pub func_params: scsi_init_func_params,
    pub q_params: scsi_init_func_queues,
    pub mtu: __le16,
    pub sq_num_pages_in_pbl: __le16,
    pub reserved: [__le32; 3],
}

// FCoE: Mode of the connection: Target or Initiator or both
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_mode_type {
    FCOE_INITIATOR_MODE = 0x0,
    FCOE_TARGET_MODE = 0x1,
    FCOE_BOTH_OR_NOT_CHOSEN = 0x3,
    MAX_FCOE_MODE_TYPE
}

// Per PF FCoE receive path statistics - tStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rx_stat {
    pub fcoe_rx_byte_cnt: regpair,
    pub fcoe_rx_data_pkt_cnt: regpair,
    pub fcoe_rx_xfer_pkt_cnt: regpair,
    pub fcoe_rx_other_pkt_cnt: regpair,
    pub fcoe_silent_drop_pkt_cmdq_full_cnt: __le32,
    pub fcoe_silent_drop_pkt_rq_full_cnt: __le32,
    pub fcoe_silent_drop_pkt_crc_error_cnt: __le32,
    pub fcoe_silent_drop_pkt_task_invalid_cnt: __le32,
    pub fcoe_silent_drop_total_pkt_cnt: __le32,
    pub rsrv: __le32,
}

// FCoE SQE request type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_sqe_request_type {
    SEND_FCOE_CMD,
    SEND_FCOE_MIDPATH,
    SEND_FCOE_ABTS_REQUEST,
    FCOE_EXCHANGE_CLEANUP,
    FCOE_SEQUENCE_RECOVERY,
    SEND_FCOE_XFER_RDY,
    SEND_FCOE_RSP,
    SEND_FCOE_RSP_WITH_SENSE_DATA,
    SEND_FCOE_TARGET_DATA,
    SEND_FCOE_INITIATOR_DATA,
    SEND_FCOE_XFER_CONTINUATION_RDY,
    SEND_FCOE_TARGET_ABTS_RSP,
    MAX_FCOE_SQE_REQUEST_TYPE
}

// FCoe statistics request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_stat_ramrod_data {
    pub stat_params_addr: regpair,
}

// FCoE task type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_task_type {
    FCOE_TASK_TYPE_WRITE_INITIATOR,
    FCOE_TASK_TYPE_READ_INITIATOR,
    FCOE_TASK_TYPE_MIDPATH,
    FCOE_TASK_TYPE_UNSOLICITED,
    FCOE_TASK_TYPE_ABTS,
    FCOE_TASK_TYPE_EXCHANGE_CLEANUP,
    FCOE_TASK_TYPE_SEQUENCE_CLEANUP,
    FCOE_TASK_TYPE_WRITE_TARGET,
    FCOE_TASK_TYPE_READ_TARGET,
    FCOE_TASK_TYPE_RSP,
    FCOE_TASK_TYPE_RSP_SENSE_DATA,
    FCOE_TASK_TYPE_ABTS_TARGET,
    FCOE_TASK_TYPE_ENUM_SIZE,
    MAX_FCOE_TASK_TYPE
}

// Per PF FCoE transmit path statistics - pStorm RAM structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_tx_stat {
    pub fcoe_tx_byte_cnt: regpair,
    pub fcoe_tx_data_pkt_cnt: regpair,
    pub fcoe_tx_xfer_pkt_cnt: regpair,
    pub fcoe_tx_other_pkt_cnt: regpair,
}

// FCoE SQ/XferQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_wqe {
    pub task_id: __le16,
    pub flags: __le16,
pub const FCOE_WQE_REQ_TYPE_MASK: c_uint = 0xF;
pub const FCOE_WQE_REQ_TYPE_SHIFT: c_int = 0;
pub const FCOE_WQE_SGL_MODE_MASK: c_uint = 0x1;
pub const FCOE_WQE_SGL_MODE_SHIFT: c_int = 4;
pub const FCOE_WQE_CONTINUATION_MASK: c_uint = 0x1;
pub const FCOE_WQE_CONTINUATION_SHIFT: c_int = 5;
pub const FCOE_WQE_SEND_AUTO_RSP_MASK: c_uint = 0x1;
pub const FCOE_WQE_SEND_AUTO_RSP_SHIFT: c_int = 6;
pub const FCOE_WQE_RESERVED_MASK: c_uint = 0x1;
pub const FCOE_WQE_RESERVED_SHIFT: c_int = 7;
pub const FCOE_WQE_NUM_SGES_MASK: c_uint = 0xF;
pub const FCOE_WQE_NUM_SGES_SHIFT: c_int = 8;
pub const FCOE_WQE_RESERVED1_MASK: c_uint = 0xF;
pub const FCOE_WQE_RESERVED1_SHIFT: c_int = 12;
    pub additional_info_union: fcoe_additional_info_union,
}

// FCoE XFRQ element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrqe_prot_flags {
    pub flags: u8,
pub const XFRQE_PROT_FLAGS_PROT_INTERVAL_SIZE_LOG_MASK: c_uint = 0xF;
pub const XFRQE_PROT_FLAGS_PROT_INTERVAL_SIZE_LOG_SHIFT: c_int = 0;
pub const XFRQE_PROT_FLAGS_DIF_TO_PEER_MASK: c_uint = 0x1;
pub const XFRQE_PROT_FLAGS_DIF_TO_PEER_SHIFT: c_int = 4;
pub const XFRQE_PROT_FLAGS_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const XFRQE_PROT_FLAGS_HOST_INTERFACE_SHIFT: c_int = 5;
pub const XFRQE_PROT_FLAGS_RESERVED_MASK: c_uint = 0x1;
pub const XFRQE_PROT_FLAGS_RESERVED_SHIFT: c_int = 7;
}

// FCoE doorbell data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_db_data {
    pub params: u8,
pub const FCOE_DB_DATA_DEST_MASK: c_uint = 0x3;
pub const FCOE_DB_DATA_DEST_SHIFT: c_int = 0;
pub const FCOE_DB_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const FCOE_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const FCOE_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const FCOE_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const FCOE_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const FCOE_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const FCOE_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3;
pub const FCOE_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub agg_flags: u8,
    pub sq_prod: __le16,
}
