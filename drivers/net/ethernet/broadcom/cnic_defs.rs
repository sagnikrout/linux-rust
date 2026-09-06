//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/cnic_defs.h
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


// cnic.c: QLogic CNIC core network driver.
//
// Copyright (c) 2006-2014 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// KWQ (kernel work queue) request op codes

// KCQ (kernel completion queue) response op codes

// KCQ (kernel completion queue) completion status

//
// L4 KCQ CQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kcq {
    pub cid: u32,
    pub pg_cid: u32,
    pub conn_id: u32,
    pub pg_host_opaque: u32,

    pub status: u16,
    pub reserved1: u16,

    pub reserved1: u16,
    pub status: u16,
    pub reserved2: [u32; 2],
    pub flags: u8,

pub const L4_KCQ_RESERVED3_SHIFT: c_int = 0;

pub const L4_KCQ_RAMROD_COMPLETION_SHIFT: c_int = 3;

pub const L4_KCQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KCQ_RESERVED4_SHIFT: c_int = 7;
    pub op_code: u8,
    pub qe_self_seq: u16,

    pub qe_self_seq: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KCQ_RESERVED3_SHIFT: c_int = 0;

pub const L4_KCQ_RAMROD_COMPLETION_SHIFT: c_int = 3;

pub const L4_KCQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KCQ_RESERVED4_SHIFT: c_int = 7;

}

//
// L4 KCQ CQE PG upload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kcq_upload_pg {
    pub pg_cid: u32,

    pub pg_status: u16,
    pub pg_ipid_count: u16,

    pub pg_ipid_count: u16,
    pub pg_status: u16,
    pub reserved1: [u32; 5],
    pub flags: u8,

pub const L4_KCQ_UPLOAD_PG_RESERVED3_SHIFT: c_int = 0;

pub const L4_KCQ_UPLOAD_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KCQ_UPLOAD_PG_RESERVED4_SHIFT: c_int = 7;
    pub op_code: u8,
    pub qe_self_seq: u16,

    pub qe_self_seq: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KCQ_UPLOAD_PG_RESERVED3_SHIFT: c_int = 0;

pub const L4_KCQ_UPLOAD_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KCQ_UPLOAD_PG_RESERVED4_SHIFT: c_int = 7;

}

//
// Gracefully close the connection request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_close_req {

    pub flags: u8,

pub const L4_KWQ_CLOSE_REQ_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CLOSE_REQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CLOSE_REQ_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_CLOSE_REQ_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CLOSE_REQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CLOSE_REQ_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub cid: u32,
    pub reserved2: [u32; 6],
}

//
// The first request to be passed in order to establish connection in option2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_connect_req1 {

    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ1_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ1_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u8,
    pub conn_flags: u8,

pub const L4_KWQ_CONNECT_REQ1_IS_PG_HOST_OPAQUE_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_IP_V6_SHIFT: c_int = 1;

pub const L4_KWQ_CONNECT_REQ1_PASSIVE_FLAG_SHIFT: c_int = 2;

pub const L4_KWQ_CONNECT_REQ1_RSRV_SHIFT: c_int = 3;

    pub conn_flags: u8,

pub const L4_KWQ_CONNECT_REQ1_IS_PG_HOST_OPAQUE_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_IP_V6_SHIFT: c_int = 1;

pub const L4_KWQ_CONNECT_REQ1_PASSIVE_FLAG_SHIFT: c_int = 2;

pub const L4_KWQ_CONNECT_REQ1_RSRV_SHIFT: c_int = 3;
    pub reserved0: u8,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ1_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ1_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub cid: u32,
    pub pg_cid: u32,
    pub src_ip: u32,
    pub dst_ip: u32,

    pub dst_port: u16,
    pub src_port: u16,

    pub src_port: u16,
    pub dst_port: u16,
    pub rsrv1: [u8; 3],
    pub tcp_flags: u8,

pub const L4_KWQ_CONNECT_REQ1_NO_DELAY_ACK_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_KEEP_ALIVE_SHIFT: c_int = 1;

pub const L4_KWQ_CONNECT_REQ1_NAGLE_ENABLE_SHIFT: c_int = 2;

pub const L4_KWQ_CONNECT_REQ1_TIME_STAMP_SHIFT: c_int = 3;

pub const L4_KWQ_CONNECT_REQ1_SACK_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ1_SEG_SCALING_SHIFT: c_int = 5;

pub const L4_KWQ_CONNECT_REQ1_RESERVED2_SHIFT: c_int = 6;

    pub tcp_flags: u8,

pub const L4_KWQ_CONNECT_REQ1_NO_DELAY_ACK_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ1_KEEP_ALIVE_SHIFT: c_int = 1;

pub const L4_KWQ_CONNECT_REQ1_NAGLE_ENABLE_SHIFT: c_int = 2;

pub const L4_KWQ_CONNECT_REQ1_TIME_STAMP_SHIFT: c_int = 3;

pub const L4_KWQ_CONNECT_REQ1_SACK_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ1_SEG_SCALING_SHIFT: c_int = 5;

pub const L4_KWQ_CONNECT_REQ1_RESERVED2_SHIFT: c_int = 6;
    pub rsrv1: [u8; 3],
    pub rsrv2: u32,
}

//
// The second ( optional )request to be passed in order to establish
// connection in option2 - for IPv6 only
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_connect_req2 {

    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ2_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ2_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ2_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u8,
    pub rsrv: u8,

    pub rsrv: u8,
    pub reserved0: u8,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ2_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ2_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ2_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub reserved2: u32,
    pub src_ip_v6_2: u32,
    pub src_ip_v6_3: u32,
    pub src_ip_v6_4: u32,
    pub dst_ip_v6_2: u32,
    pub dst_ip_v6_3: u32,
    pub dst_ip_v6_4: u32,
}

//
// The third ( and last )request to be passed in order to establish
// connection in option2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_connect_req3 {

    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ3_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ3_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ3_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_CONNECT_REQ3_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_CONNECT_REQ3_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_CONNECT_REQ3_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub ka_timeout: u32,
    pub ka_interval: u32,

    pub snd_seq_scale: u8,
    pub ttl: u8,
    pub tos: u8,
    pub ka_max_probe_count: u8,

    pub ka_max_probe_count: u8,
    pub tos: u8,
    pub ttl: u8,
    pub snd_seq_scale: u8,

    pub pmtu: u16,
    pub mss: u16,

    pub mss: u16,
    pub pmtu: u16,

    pub rcv_buf: u32,
    pub snd_buf: u32,
    pub seed: u32,
}

//
// a KWQE request to offload a PG connection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_offload_pg {

    pub flags: u8,

pub const L4_KWQ_OFFLOAD_PG_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_OFFLOAD_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_OFFLOAD_PG_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_OFFLOAD_PG_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_OFFLOAD_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_OFFLOAD_PG_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub l2hdr_nbytes: u8,
    pub pg_flags: u8,

pub const L4_KWQ_OFFLOAD_PG_SNAP_ENCAP_SHIFT: c_int = 0;

pub const L4_KWQ_OFFLOAD_PG_VLAN_TAGGING_SHIFT: c_int = 1;

pub const L4_KWQ_OFFLOAD_PG_RESERVED2_SHIFT: c_int = 2;
    pub da0: u8,
    pub da1: u8,

    pub da1: u8,
    pub da0: u8,
    pub pg_flags: u8,

pub const L4_KWQ_OFFLOAD_PG_SNAP_ENCAP_SHIFT: c_int = 0;

pub const L4_KWQ_OFFLOAD_PG_VLAN_TAGGING_SHIFT: c_int = 1;

pub const L4_KWQ_OFFLOAD_PG_RESERVED2_SHIFT: c_int = 2;
    pub l2hdr_nbytes: u8,

    pub da2: u8,
    pub da3: u8,
    pub da4: u8,
    pub da5: u8,

    pub da5: u8,
    pub da4: u8,
    pub da3: u8,
    pub da2: u8,

    pub sa0: u8,
    pub sa1: u8,
    pub sa2: u8,
    pub sa3: u8,

    pub sa3: u8,
    pub sa2: u8,
    pub sa1: u8,
    pub sa0: u8,

    pub sa4: u8,
    pub sa5: u8,
    pub etype: u16,

    pub etype: u16,
    pub sa5: u8,
    pub sa4: u8,

    pub vlan_tag: u16,
    pub ipid_start: u16,

    pub ipid_start: u16,
    pub vlan_tag: u16,

    pub ipid_count: u16,
    pub reserved3: u16,

    pub reserved3: u16,
    pub ipid_count: u16,

    pub host_opaque: u32,
}

//
// Abortively close the connection request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_reset_req {

    pub flags: u8,

pub const L4_KWQ_RESET_REQ_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_RESET_REQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_RESET_REQ_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub op_code: u8,
    pub reserved0: u16,

    pub reserved0: u16,
    pub op_code: u8,
    pub flags: u8,

pub const L4_KWQ_RESET_REQ_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_RESET_REQ_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_RESET_REQ_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub cid: u32,
    pub reserved2: [u32; 6],
}

//
// a KWQE request to update a PG connection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_update_pg {

    pub flags: u8,

pub const L4_KWQ_UPDATE_PG_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_UPDATE_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_UPDATE_PG_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub opcode: u8,
    pub oper16: u16,

    pub oper16: u16,
    pub opcode: u8,
    pub flags: u8,

pub const L4_KWQ_UPDATE_PG_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_UPDATE_PG_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_UPDATE_PG_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub pg_cid: u32,
    pub pg_host_opaque: u32,

    pub pg_valids: u8,

pub const L4_KWQ_UPDATE_PG_VALIDS_IPID_COUNT_SHIFT: c_int = 0;

pub const L4_KWQ_UPDATE_PG_VALIDS_DA_SHIFT: c_int = 1;

pub const L4_KWQ_UPDATE_PG_RESERVERD2_SHIFT: c_int = 2;
    pub pg_unused_a: u8,
    pub pg_ipid_count: u16,

    pub pg_ipid_count: u16,
    pub pg_unused_a: u8,
    pub pg_valids: u8,

pub const L4_KWQ_UPDATE_PG_VALIDS_IPID_COUNT_SHIFT: c_int = 0;

pub const L4_KWQ_UPDATE_PG_VALIDS_DA_SHIFT: c_int = 1;

pub const L4_KWQ_UPDATE_PG_RESERVERD2_SHIFT: c_int = 2;

    pub reserved3: u16,
    pub da0: u8,
    pub da1: u8,

    pub da1: u8,
    pub da0: u8,
    pub reserved3: u16,

    pub da2: u8,
    pub da3: u8,
    pub da4: u8,
    pub da5: u8,

    pub da5: u8,
    pub da4: u8,
    pub da3: u8,
    pub da2: u8,

    pub reserved4: u32,
    pub reserved5: u32,
}

//
// a KWQE request to upload a PG or L4 context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4_kwq_upload {

    pub flags: u8,

pub const L4_KWQ_UPLOAD_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_UPLOAD_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_UPLOAD_LINKED_WITH_NEXT_SHIFT: c_int = 7;
    pub opcode: u8,
    pub oper16: u16,

    pub oper16: u16,
    pub opcode: u8,
    pub flags: u8,

pub const L4_KWQ_UPLOAD_RESERVED1_SHIFT: c_int = 0;

pub const L4_KWQ_UPLOAD_LAYER_CODE_SHIFT: c_int = 4;

pub const L4_KWQ_UPLOAD_LINKED_WITH_NEXT_SHIFT: c_int = 7;

    pub cid: u32,
    pub reserved2: [u32; 6],
}

//
// bnx2x structures
//
// The iscsi aggregative context of Cstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_iscsi_ag_context {
    pub agg_vars1: u32,

pub const CSTORM_ISCSI_AG_CONTEXT_STATE_SHIFT: c_int = 0;

pub const __CSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 8;

pub const __CSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 9;

pub const __CSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 10;

pub const __CSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 11;

pub const __CSTORM_ISCSI_AG_CONTEXT_RESERVED_ULP_RX_SE_CF_EN_SHIFT: c_int = 12;

pub const __CSTORM_ISCSI_AG_CONTEXT_RESERVED_ULP_RX_INV_CF_EN_SHIFT: c_int = 13;

pub const __CSTORM_ISCSI_AG_CONTEXT_AUX4_CF_SHIFT: c_int = 14;

pub const __CSTORM_ISCSI_AG_CONTEXT_RESERVED66_SHIFT: c_int = 16;

pub const __CSTORM_ISCSI_AG_CONTEXT_FIN_RECEIVED_CF_EN_SHIFT: c_int = 18;

pub const __CSTORM_ISCSI_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 19;

pub const __CSTORM_ISCSI_AG_CONTEXT_AUX2_CF_EN_SHIFT: c_int = 20;

pub const __CSTORM_ISCSI_AG_CONTEXT_AUX3_CF_EN_SHIFT: c_int = 21;

pub const __CSTORM_ISCSI_AG_CONTEXT_AUX4_CF_EN_SHIFT: c_int = 22;

pub const __CSTORM_ISCSI_AG_CONTEXT_REL_SEQ_RULE_SHIFT: c_int = 23;

pub const CSTORM_ISCSI_AG_CONTEXT_HQ_PROD_RULE_SHIFT: c_int = 26;

pub const __CSTORM_ISCSI_AG_CONTEXT_RESERVED52_SHIFT: c_int = 28;

pub const __CSTORM_ISCSI_AG_CONTEXT_RESERVED53_SHIFT: c_int = 30;

    pub __aux1_th: u8,
    pub __aux1_val: u8,
    pub __agg_vars2: u16,

    pub __agg_vars2: u16,
    pub __aux1_val: u8,
    pub __aux1_th: u8,

    pub rel_seq: u32,
    pub rel_seq_th: u32,

    pub hq_cons: u16,
    pub hq_prod: u16,

    pub hq_prod: u16,
    pub hq_cons: u16,

    pub __reserved62: u8,
    pub __reserved61: u8,
    pub __reserved60: u8,
    pub __reserved59: u8,

    pub __reserved59: u8,
    pub __reserved60: u8,
    pub __reserved61: u8,
    pub __reserved62: u8,

    pub __reserved64: u16,
    pub cq_u_prod: u16,

    pub cq_u_prod: u16,
    pub __reserved64: u16,

    pub __cq_u_prod1: u32,

    pub __agg_vars3: u16,
    pub cq_u_pend: u16,

    pub cq_u_pend: u16,
    pub __agg_vars3: u16,

    pub __aux2_th: u16,
    pub aux2_val: u16,

    pub aux2_val: u16,
    pub __aux2_th: u16,

}

//
// The fcoe extra aggregative context section of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_extra_ag_context_section {
    pub __agg_val1: u32,

    pub __tcp_agg_vars2: u8,
    pub __agg_val3: u8,
    pub __agg_val2: u16,

    pub __agg_val2: u16,
    pub __agg_val3: u8,
    pub __tcp_agg_vars2: u8,

    pub __agg_val5: u16,
    pub __agg_val6: u8,
    pub __tcp_agg_vars3: u8,

    pub __tcp_agg_vars3: u8,
    pub __agg_val6: u8,
    pub __agg_val5: u16,

    pub __lcq_prod: u32,
    pub rtt_seq: u32,
    pub rtt_time: u32,
    pub __reserved66: u32,
    pub wnd_right_edge: u32,
    pub tcp_agg_vars1: u32,

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_FIN_SENT_FLAG_SHIFT: c_int = 0;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_LAST_PACKET_FIN_FLAG_SHIFT: c_int = 1;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_WND_UPD_CF_SHIFT: c_int = 2;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TIMEOUT_CF_SHIFT: c_int = 4;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_WND_UPD_CF_EN_SHIFT: c_int = 6;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TIMEOUT_CF_EN_SHIFT: c_int = 7;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RETRANSMIT_SEQ_EN_SHIFT: c_int = 8;

pub const __TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_LCQ_SND_EN_SHIFT: c_int = 9;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 10;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX2_FLAG_SHIFT: c_int = 11;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX1_CF_EN_SHIFT: c_int = 12;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX2_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX1_CF_SHIFT: c_int = 14;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX2_CF_SHIFT: c_int = 16;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TX_BLOCKED_SHIFT: c_int = 18;

pub const __TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX10_CF_EN_SHIFT: c_int = 19;

pub const __TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX11_CF_EN_SHIFT: c_int = 20;

pub const __TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX12_CF_EN_SHIFT: c_int = 21;

pub const __TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED1_SHIFT: c_int = 22;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RETRANSMIT_PEND_SEQ_SHIFT: c_int = 24;

pub const TSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RETRANSMIT_DONE_SEQ_SHIFT: c_int = 28;
    pub snd_max: u32,
    pub __lcq_cons: u32,
    pub __reserved2: u32,
}

//
// The fcoe aggregative context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_ag_context {

    pub ulp_credit: u16,
    pub agg_vars1: u8,

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __TSTORM_FCOE_AG_CONTEXT_QUEUE0_FLUSH_CF_SHIFT: c_int = 4;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX3_FLAG_SHIFT: c_int = 6;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX4_FLAG_SHIFT: c_int = 7;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const TSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __TSTORM_FCOE_AG_CONTEXT_QUEUE0_FLUSH_CF_SHIFT: c_int = 4;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX3_FLAG_SHIFT: c_int = 6;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX4_FLAG_SHIFT: c_int = 7;
    pub ulp_credit: u16,

    pub __agg_val4: u16,
    pub agg_vars2: u16,

pub const __TSTORM_FCOE_AG_CONTEXT_AUX5_FLAG_SHIFT: c_int = 0;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX6_FLAG_SHIFT: c_int = 1;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX4_CF_SHIFT: c_int = 2;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX5_CF_SHIFT: c_int = 4;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX6_CF_SHIFT: c_int = 6;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX7_CF_SHIFT: c_int = 8;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX7_FLAG_SHIFT: c_int = 10;

pub const __TSTORM_FCOE_AG_CONTEXT_QUEUE0_FLUSH_CF_EN_SHIFT: c_int = 11;

pub const TSTORM_FCOE_AG_CONTEXT_AUX4_CF_EN_SHIFT: c_int = 12;

pub const TSTORM_FCOE_AG_CONTEXT_AUX5_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_FCOE_AG_CONTEXT_AUX6_CF_EN_SHIFT: c_int = 14;

pub const TSTORM_FCOE_AG_CONTEXT_AUX7_CF_EN_SHIFT: c_int = 15;

    pub agg_vars2: u16,

pub const __TSTORM_FCOE_AG_CONTEXT_AUX5_FLAG_SHIFT: c_int = 0;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX6_FLAG_SHIFT: c_int = 1;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX4_CF_SHIFT: c_int = 2;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX5_CF_SHIFT: c_int = 4;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX6_CF_SHIFT: c_int = 6;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX7_CF_SHIFT: c_int = 8;

pub const __TSTORM_FCOE_AG_CONTEXT_AUX7_FLAG_SHIFT: c_int = 10;

pub const __TSTORM_FCOE_AG_CONTEXT_QUEUE0_FLUSH_CF_EN_SHIFT: c_int = 11;

pub const TSTORM_FCOE_AG_CONTEXT_AUX4_CF_EN_SHIFT: c_int = 12;

pub const TSTORM_FCOE_AG_CONTEXT_AUX5_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_FCOE_AG_CONTEXT_AUX6_CF_EN_SHIFT: c_int = 14;

pub const TSTORM_FCOE_AG_CONTEXT_AUX7_CF_EN_SHIFT: c_int = 15;
    pub __agg_val4: u16,

    pub __extra_section: tstorm_fcoe_extra_ag_context_section,
}

//
// The tcp aggregative context section of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_tcp_tcp_ag_context_section {
    pub __agg_val1: u32,

    pub __tcp_agg_vars2: u8,
    pub __agg_val3: u8,
    pub __agg_val2: u16,

    pub __agg_val2: u16,
    pub __agg_val3: u8,
    pub __tcp_agg_vars2: u8,

    pub __agg_val5: u16,
    pub __agg_val6: u8,
    pub __tcp_agg_vars3: u8,

    pub __tcp_agg_vars3: u8,
    pub __agg_val6: u8,
    pub __agg_val5: u16,

    pub snd_nxt: u32,
    pub rtt_seq: u32,
    pub rtt_time: u32,
    pub wnd_right_edge_local: u32,
    pub wnd_right_edge: u32,
    pub tcp_agg_vars1: u32,

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_FIN_SENT_FLAG_SHIFT: c_int = 0;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_LAST_PACKET_FIN_FLAG_SHIFT: c_int = 1;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_WND_UPD_CF_SHIFT: c_int = 2;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_TIMEOUT_CF_SHIFT: c_int = 4;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_WND_UPD_CF_EN_SHIFT: c_int = 6;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_TIMEOUT_CF_EN_SHIFT: c_int = 7;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_RETRANSMIT_SEQ_EN_SHIFT: c_int = 8;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_SND_NXT_EN_SHIFT: c_int = 9;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 10;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX2_FLAG_SHIFT: c_int = 11;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX1_CF_EN_SHIFT: c_int = 12;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX2_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX1_CF_SHIFT: c_int = 14;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX2_CF_SHIFT: c_int = 16;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_BLOCKED_SHIFT: c_int = 18;

pub const __TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX10_CF_EN_SHIFT: c_int = 19;

pub const __TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX11_CF_EN_SHIFT: c_int = 20;

pub const __TSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX12_CF_EN_SHIFT: c_int = 21;

pub const __TSTORM_TCP_TCP_AG_CONTEXT_SECTION_RESERVED1_SHIFT: c_int = 22;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_RETRANSMIT_PEND_SEQ_SHIFT: c_int = 24;

pub const TSTORM_TCP_TCP_AG_CONTEXT_SECTION_RETRANSMIT_DONE_SEQ_SHIFT: c_int = 28;
    pub snd_max: u32,
    pub snd_una: u32,
    pub __reserved2: u32,
}

//
// The iscsi aggregative context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_ag_context {

    pub ulp_credit: u16,
    pub agg_vars1: u8,

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __TSTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_SHIFT: c_int = 4;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX3_FLAG_SHIFT: c_int = 6;

pub const __TSTORM_ISCSI_AG_CONTEXT_ACK_ON_FIN_SENT_FLAG_SHIFT: c_int = 7;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const TSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __TSTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_SHIFT: c_int = 4;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX3_FLAG_SHIFT: c_int = 6;

pub const __TSTORM_ISCSI_AG_CONTEXT_ACK_ON_FIN_SENT_FLAG_SHIFT: c_int = 7;
    pub ulp_credit: u16,

    pub __agg_val4: u16,
    pub agg_vars2: u16,

pub const __TSTORM_ISCSI_AG_CONTEXT_MSL_TIMER_SET_FLAG_SHIFT: c_int = 0;

pub const __TSTORM_ISCSI_AG_CONTEXT_FIN_SENT_FIRST_FLAG_SHIFT: c_int = 1;

pub const __TSTORM_ISCSI_AG_CONTEXT_RST_SENT_CF_SHIFT: c_int = 2;

pub const __TSTORM_ISCSI_AG_CONTEXT_WAKEUP_CALL_CF_SHIFT: c_int = 4;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX6_CF_SHIFT: c_int = 6;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX7_CF_SHIFT: c_int = 8;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX7_FLAG_SHIFT: c_int = 10;

pub const __TSTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_EN_SHIFT: c_int = 11;

pub const __TSTORM_ISCSI_AG_CONTEXT_RST_SENT_CF_EN_SHIFT: c_int = 12;

pub const __TSTORM_ISCSI_AG_CONTEXT_WAKEUP_CALL_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_ISCSI_AG_CONTEXT_AUX6_CF_EN_SHIFT: c_int = 14;

pub const TSTORM_ISCSI_AG_CONTEXT_AUX7_CF_EN_SHIFT: c_int = 15;

    pub agg_vars2: u16,

pub const __TSTORM_ISCSI_AG_CONTEXT_MSL_TIMER_SET_FLAG_SHIFT: c_int = 0;

pub const __TSTORM_ISCSI_AG_CONTEXT_FIN_SENT_FIRST_FLAG_SHIFT: c_int = 1;

pub const __TSTORM_ISCSI_AG_CONTEXT_RST_SENT_CF_SHIFT: c_int = 2;

pub const __TSTORM_ISCSI_AG_CONTEXT_WAKEUP_CALL_CF_SHIFT: c_int = 4;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX6_CF_SHIFT: c_int = 6;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX7_CF_SHIFT: c_int = 8;

pub const __TSTORM_ISCSI_AG_CONTEXT_AUX7_FLAG_SHIFT: c_int = 10;

pub const __TSTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_EN_SHIFT: c_int = 11;

pub const __TSTORM_ISCSI_AG_CONTEXT_RST_SENT_CF_EN_SHIFT: c_int = 12;

pub const __TSTORM_ISCSI_AG_CONTEXT_WAKEUP_CALL_CF_EN_SHIFT: c_int = 13;

pub const TSTORM_ISCSI_AG_CONTEXT_AUX6_CF_EN_SHIFT: c_int = 14;

pub const TSTORM_ISCSI_AG_CONTEXT_AUX7_CF_EN_SHIFT: c_int = 15;
    pub __agg_val4: u16,

    pub tcp: tstorm_tcp_tcp_ag_context_section,
}

//
// The fcoe aggregative context of Ustorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_ag_context {

    pub __aux_counter_flags: u8,
    pub agg_vars2: u8,

pub const USTORM_FCOE_AG_CONTEXT_TX_CF_SHIFT: c_int = 0;

pub const __USTORM_FCOE_AG_CONTEXT_TIMER_CF_SHIFT: c_int = 2;

pub const USTORM_FCOE_AG_CONTEXT_AGG_MISC4_RULE_SHIFT: c_int = 4;

pub const __USTORM_FCOE_AG_CONTEXT_AGG_VAL2_MASK_SHIFT: c_int = 7;
    pub agg_vars1: u8,

pub const __USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const USTORM_FCOE_AG_CONTEXT_INV_CF_SHIFT: c_int = 4;

pub const USTORM_FCOE_AG_CONTEXT_COMPLETION_CF_SHIFT: c_int = 6;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const __USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const USTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const USTORM_FCOE_AG_CONTEXT_INV_CF_SHIFT: c_int = 4;

pub const USTORM_FCOE_AG_CONTEXT_COMPLETION_CF_SHIFT: c_int = 6;
    pub agg_vars2: u8,

pub const USTORM_FCOE_AG_CONTEXT_TX_CF_SHIFT: c_int = 0;

pub const __USTORM_FCOE_AG_CONTEXT_TIMER_CF_SHIFT: c_int = 2;

pub const USTORM_FCOE_AG_CONTEXT_AGG_MISC4_RULE_SHIFT: c_int = 4;

pub const __USTORM_FCOE_AG_CONTEXT_AGG_VAL2_MASK_SHIFT: c_int = 7;
    pub __aux_counter_flags: u8,

    pub cdu_usage: u8,
    pub agg_misc2: u8,
    pub pbf_tx_seq_ack: u16,

    pub pbf_tx_seq_ack: u16,
    pub agg_misc2: u8,
    pub cdu_usage: u8,

    pub agg_misc4: u32,

    pub agg_val3_th: u8,
    pub agg_val3: u8,
    pub agg_misc3: u16,

    pub agg_misc3: u16,
    pub agg_val3: u8,
    pub agg_val3_th: u8,

    pub expired_task_id: u32,
    pub agg_misc4_th: u32,

    pub cq_prod: u16,
    pub cq_cons: u16,

    pub cq_cons: u16,
    pub cq_prod: u16,

    pub __reserved2: u16,
    pub decision_rules: u8,

pub const USTORM_FCOE_AG_CONTEXT_CQ_DEC_RULE_SHIFT: c_int = 0;

pub const __USTORM_FCOE_AG_CONTEXT_AGG_VAL3_RULE_SHIFT: c_int = 3;

pub const USTORM_FCOE_AG_CONTEXT_CQ_ARM_N_FLAG_SHIFT: c_int = 6;

pub const __USTORM_FCOE_AG_CONTEXT_RESERVED1_SHIFT: c_int = 7;
    pub decision_rule_enable_bits: u8,

pub const __USTORM_FCOE_AG_CONTEXT_RESERVED_INV_CF_EN_SHIFT: c_int = 0;

pub const USTORM_FCOE_AG_CONTEXT_COMPLETION_CF_EN_SHIFT: c_int = 1;

pub const USTORM_FCOE_AG_CONTEXT_TX_CF_EN_SHIFT: c_int = 2;

pub const __USTORM_FCOE_AG_CONTEXT_TIMER_CF_EN_SHIFT: c_int = 3;

pub const __USTORM_FCOE_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 4;

pub const __USTORM_FCOE_AG_CONTEXT_QUEUE0_CF_EN_SHIFT: c_int = 5;

pub const __USTORM_FCOE_AG_CONTEXT_AUX3_CF_EN_SHIFT: c_int = 6;

pub const __USTORM_FCOE_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;

    pub decision_rule_enable_bits: u8,

pub const __USTORM_FCOE_AG_CONTEXT_RESERVED_INV_CF_EN_SHIFT: c_int = 0;

pub const USTORM_FCOE_AG_CONTEXT_COMPLETION_CF_EN_SHIFT: c_int = 1;

pub const USTORM_FCOE_AG_CONTEXT_TX_CF_EN_SHIFT: c_int = 2;

pub const __USTORM_FCOE_AG_CONTEXT_TIMER_CF_EN_SHIFT: c_int = 3;

pub const __USTORM_FCOE_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 4;

pub const __USTORM_FCOE_AG_CONTEXT_QUEUE0_CF_EN_SHIFT: c_int = 5;

pub const __USTORM_FCOE_AG_CONTEXT_AUX3_CF_EN_SHIFT: c_int = 6;

pub const __USTORM_FCOE_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;
    pub decision_rules: u8,

pub const USTORM_FCOE_AG_CONTEXT_CQ_DEC_RULE_SHIFT: c_int = 0;

pub const __USTORM_FCOE_AG_CONTEXT_AGG_VAL3_RULE_SHIFT: c_int = 3;

pub const USTORM_FCOE_AG_CONTEXT_CQ_ARM_N_FLAG_SHIFT: c_int = 6;

pub const __USTORM_FCOE_AG_CONTEXT_RESERVED1_SHIFT: c_int = 7;
    pub __reserved2: u16,

}

//
// The iscsi aggregative context of Ustorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_ag_context {

    pub __aux_counter_flags: u8,
    pub agg_vars2: u8,

pub const USTORM_ISCSI_AG_CONTEXT_TX_CF_SHIFT: c_int = 0;

pub const __USTORM_ISCSI_AG_CONTEXT_TIMER_CF_SHIFT: c_int = 2;

pub const USTORM_ISCSI_AG_CONTEXT_AGG_MISC4_RULE_SHIFT: c_int = 4;

pub const __USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_MASK_SHIFT: c_int = 7;
    pub agg_vars1: u8,

pub const __USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const USTORM_ISCSI_AG_CONTEXT_INV_CF_SHIFT: c_int = 4;

pub const USTORM_ISCSI_AG_CONTEXT_COMPLETION_CF_SHIFT: c_int = 6;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const __USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const USTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const USTORM_ISCSI_AG_CONTEXT_INV_CF_SHIFT: c_int = 4;

pub const USTORM_ISCSI_AG_CONTEXT_COMPLETION_CF_SHIFT: c_int = 6;
    pub agg_vars2: u8,

pub const USTORM_ISCSI_AG_CONTEXT_TX_CF_SHIFT: c_int = 0;

pub const __USTORM_ISCSI_AG_CONTEXT_TIMER_CF_SHIFT: c_int = 2;

pub const USTORM_ISCSI_AG_CONTEXT_AGG_MISC4_RULE_SHIFT: c_int = 4;

pub const __USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_MASK_SHIFT: c_int = 7;
    pub __aux_counter_flags: u8,

    pub cdu_usage: u8,
    pub agg_misc2: u8,
    pub __cq_local_comp_itt_val: u16,

    pub __cq_local_comp_itt_val: u16,
    pub agg_misc2: u8,
    pub cdu_usage: u8,

    pub agg_misc4: u32,

    pub agg_val3_th: u8,
    pub agg_val3: u8,
    pub agg_misc3: u16,

    pub agg_misc3: u16,
    pub agg_val3: u8,
    pub agg_val3_th: u8,

    pub agg_val1: u32,
    pub agg_misc4_th: u32,

    pub agg_val2_th: u16,
    pub agg_val2: u16,

    pub agg_val2: u16,
    pub agg_val2_th: u16,

    pub __reserved2: u16,
    pub decision_rules: u8,

pub const USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_RULE_SHIFT: c_int = 0;

pub const __USTORM_ISCSI_AG_CONTEXT_AGG_VAL3_RULE_SHIFT: c_int = 3;

pub const USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_ARM_N_FLAG_SHIFT: c_int = 6;

pub const __USTORM_ISCSI_AG_CONTEXT_RESERVED1_SHIFT: c_int = 7;
    pub decision_rule_enable_bits: u8,

pub const USTORM_ISCSI_AG_CONTEXT_INV_CF_EN_SHIFT: c_int = 0;

pub const USTORM_ISCSI_AG_CONTEXT_COMPLETION_CF_EN_SHIFT: c_int = 1;

pub const USTORM_ISCSI_AG_CONTEXT_TX_CF_EN_SHIFT: c_int = 2;

pub const __USTORM_ISCSI_AG_CONTEXT_TIMER_CF_EN_SHIFT: c_int = 3;

pub const __USTORM_ISCSI_AG_CONTEXT_CQ_LOCAL_COMP_CF_EN_SHIFT: c_int = 4;

pub const __USTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;

pub const __USTORM_ISCSI_AG_CONTEXT_AUX3_CF_EN_SHIFT: c_int = 6;

pub const __USTORM_ISCSI_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;

    pub decision_rule_enable_bits: u8,

pub const USTORM_ISCSI_AG_CONTEXT_INV_CF_EN_SHIFT: c_int = 0;

pub const USTORM_ISCSI_AG_CONTEXT_COMPLETION_CF_EN_SHIFT: c_int = 1;

pub const USTORM_ISCSI_AG_CONTEXT_TX_CF_EN_SHIFT: c_int = 2;

pub const __USTORM_ISCSI_AG_CONTEXT_TIMER_CF_EN_SHIFT: c_int = 3;

pub const __USTORM_ISCSI_AG_CONTEXT_CQ_LOCAL_COMP_CF_EN_SHIFT: c_int = 4;

pub const __USTORM_ISCSI_AG_CONTEXT_QUEUES_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;

pub const __USTORM_ISCSI_AG_CONTEXT_AUX3_CF_EN_SHIFT: c_int = 6;

pub const __USTORM_ISCSI_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;
    pub decision_rules: u8,

pub const USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_RULE_SHIFT: c_int = 0;

pub const __USTORM_ISCSI_AG_CONTEXT_AGG_VAL3_RULE_SHIFT: c_int = 3;

pub const USTORM_ISCSI_AG_CONTEXT_AGG_VAL2_ARM_N_FLAG_SHIFT: c_int = 6;

pub const __USTORM_ISCSI_AG_CONTEXT_RESERVED1_SHIFT: c_int = 7;
    pub __reserved2: u16,

}

//
// The fcoe aggregative context section of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_extra_ag_context_section {

    pub tcp_agg_vars1: u8,

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED51_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_SHIFT: c_int = 2;

pub const XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_SHIFT: c_int = 4;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_CLEAR_DA_TIMER_EN_SHIFT: c_int = 6;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_DA_EXPIRATION_FLAG_SHIFT: c_int = 7;
    pub __reserved_da_cnt: u8,
    pub __mtu: u16,

    pub __mtu: u16,
    pub __reserved_da_cnt: u8,
    pub tcp_agg_vars1: u8,

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED51_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_SHIFT: c_int = 2;

pub const XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_SHIFT: c_int = 4;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_CLEAR_DA_TIMER_EN_SHIFT: c_int = 6;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_DA_EXPIRATION_FLAG_SHIFT: c_int = 7;

    pub snd_nxt: u32,
    pub __xfrqe_bd_addr_lo: u32,
    pub __xfrqe_bd_addr_hi: u32,
    pub __xfrqe_data1: u32,

    pub __agg_val8_th: u8,
    pub __tx_dest: u8,
    pub tcp_agg_vars2: u16,

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED57_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED58_SHIFT: c_int = 1;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED59_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX3_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX4_FLAG_SHIFT: c_int = 4;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED60_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_ACK_TO_FE_UPDATED_EN_SHIFT: c_int = 6;

pub const XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_EN_SHIFT: c_int = 7;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_TX_FIN_FLAG_EN_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 9;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SET_RTO_CF_SHIFT: c_int = 10;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TS_TO_ECHO_UPDATED_CF_SHIFT: c_int = 12;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TX_DEST_UPDATED_CF_SHIFT: c_int = 14;

    pub tcp_agg_vars2: u16,

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED57_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED58_SHIFT: c_int = 1;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED59_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX3_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX4_FLAG_SHIFT: c_int = 4;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED60_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_ACK_TO_FE_UPDATED_EN_SHIFT: c_int = 6;

pub const XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_EN_SHIFT: c_int = 7;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_RESERVED_TX_FIN_FLAG_EN_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 9;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_SET_RTO_CF_SHIFT: c_int = 10;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TS_TO_ECHO_UPDATED_CF_SHIFT: c_int = 12;

pub const __XSTORM_FCOE_EXTRA_AG_CONTEXT_SECTION_TX_DEST_UPDATED_CF_SHIFT: c_int = 14;
    pub __tx_dest: u8,
    pub __agg_val8_th: u8,

    pub __sq_base_addr_lo: u32,
    pub __sq_base_addr_hi: u32,
    pub __xfrq_base_addr_lo: u32,
    pub __xfrq_base_addr_hi: u32,

    pub __xfrq_cons: u16,
    pub __xfrq_prod: u16,

    pub __xfrq_prod: u16,
    pub __xfrq_cons: u16,

    pub __tcp_agg_vars5: u8,
    pub __tcp_agg_vars4: u8,
    pub __tcp_agg_vars3: u8,
    pub __reserved_force_pure_ack_cnt: u8,

    pub __reserved_force_pure_ack_cnt: u8,
    pub __tcp_agg_vars3: u8,
    pub __tcp_agg_vars4: u8,
    pub __tcp_agg_vars5: u8,

    pub __tcp_agg_vars6: u32,

    pub __xfrqe_mng: u16,
    pub __tcp_agg_vars7: u16,

    pub __tcp_agg_vars7: u16,
    pub __xfrqe_mng: u16,

    pub __xfrqe_data0: u32,
    pub __agg_val10_th: u32,

    pub __reserved3: u16,
    pub __reserved2: u8,
    pub __da_only_cnt: u8,

    pub __da_only_cnt: u8,
    pub __reserved2: u8,
    pub __reserved3: u16,

}

//
// The fcoe aggregative context of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_ag_context {

    pub agg_val1: u16,
    pub agg_vars1: u8,

pub const __XSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED51_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED52_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub __state: u8,

    pub __state: u8,
    pub agg_vars1: u8,

pub const __XSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED51_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED52_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub agg_val1: u16,

    pub cdu_reserved: u8,
    pub __agg_vars4: u8,
    pub agg_vars3: u8,

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX19_CF_SHIFT: c_int = 6;
    pub agg_vars2: u8,

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_CF_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;

    pub agg_vars2: u8,

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_CF_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const __XSTORM_FCOE_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;
    pub agg_vars3: u8,

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX19_CF_SHIFT: c_int = 6;
    pub __agg_vars4: u8,
    pub cdu_reserved: u8,

    pub more_to_send: u32,

    pub agg_vars5: u16,

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_AG_CONTEXT_CONFQ_DEC_RULE_SHIFT: c_int = 14;
    pub sq_cons: u16,

    pub sq_cons: u16,
    pub agg_vars5: u16,

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_FCOE_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_AG_CONTEXT_CONFQ_DEC_RULE_SHIFT: c_int = 14;

    pub __extra_section: xstorm_fcoe_extra_ag_context_section,

    pub agg_vars7: u16,

pub const __XSTORM_FCOE_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_QUEUE0_CF_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_FCOE_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED62_SHIFT: c_int = 10;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX2_FLAG_SHIFT: c_int = 15;
    pub agg_val3_th: u8,
    pub agg_vars6: u8,

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_XFRQ_DEC_RULE_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_SQ_DEC_RULE_SHIFT: c_int = 6;

    pub agg_vars6: u8,

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_XFRQ_DEC_RULE_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_SQ_DEC_RULE_SHIFT: c_int = 6;
    pub agg_val3_th: u8,
    pub agg_vars7: u16,

pub const __XSTORM_FCOE_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_FCOE_AG_CONTEXT_QUEUE0_CF_SHIFT: c_int = 4;

pub const XSTORM_FCOE_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_FCOE_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_FCOE_AG_CONTEXT_RESERVED62_SHIFT: c_int = 10;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_FCOE_AG_CONTEXT_AUX2_FLAG_SHIFT: c_int = 15;

    pub __agg_val11_th: u16,
    pub __agg_val11: u16,

    pub __agg_val11: u16,
    pub __agg_val11_th: u16,

    pub __reserved1: u8,
    pub __agg_val6_th: u8,
    pub __agg_val9: u16,

    pub __agg_val9: u16,
    pub __agg_val6_th: u8,
    pub __reserved1: u8,

    pub confq_cons: u16,
    pub confq_prod: u16,

    pub confq_prod: u16,
    pub confq_cons: u16,

    pub agg_vars8: u32,

pub const XSTORM_FCOE_AG_CONTEXT_AGG_MISC2_SHIFT: c_int = 0;

pub const XSTORM_FCOE_AG_CONTEXT_AGG_MISC3_SHIFT: c_int = 24;

    pub __cache_wqe_db: u16,
    pub sq_prod: u16,

    pub sq_prod: u16,
    pub __cache_wqe_db: u16,

    pub agg_val3: u8,
    pub agg_val6: u8,
    pub agg_val5_th: u8,
    pub agg_val5: u8,

    pub agg_val5: u8,
    pub agg_val5_th: u8,
    pub agg_val6: u8,
    pub agg_val3: u8,

    pub __agg_misc1: u16,
    pub agg_limit1: u16,

    pub agg_limit1: u16,
    pub __agg_misc1: u16,

    pub completion_seq: u32,
    pub confq_pbl_base_lo: u32,
    pub confq_pbl_base_hi: u32,
}

//
// The tcp aggregative context section of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_tcp_tcp_ag_context_section {

    pub tcp_agg_vars1: u8,

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SET_DA_TIMER_CF_SHIFT: c_int = 0;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_SHIFT: c_int = 2;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_SHIFT: c_int = 4;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_CLEAR_DA_TIMER_EN_SHIFT: c_int = 6;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_EXPIRATION_FLAG_SHIFT: c_int = 7;
    pub __da_cnt: u8,
    pub mss: u16,

    pub mss: u16,
    pub __da_cnt: u8,
    pub tcp_agg_vars1: u8,

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SET_DA_TIMER_CF_SHIFT: c_int = 0;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_SHIFT: c_int = 2;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_SHIFT: c_int = 4;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_CLEAR_DA_TIMER_EN_SHIFT: c_int = 6;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_EXPIRATION_FLAG_SHIFT: c_int = 7;

    pub snd_nxt: u32,
    pub tx_wnd: u32,
    pub snd_una: u32,
    pub local_adv_wnd: u32,

    pub __agg_val8_th: u8,
    pub __tx_dest: u8,
    pub tcp_agg_vars2: u16,

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_FIN_FLAG_SHIFT: c_int = 0;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_UNBLOCKED_SHIFT: c_int = 1;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_TIMER_ACTIVE_SHIFT: c_int = 2;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX3_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX4_FLAG_SHIFT: c_int = 4;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_ENABLE_SHIFT: c_int = 5;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_EN_SHIFT: c_int = 6;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_EN_SHIFT: c_int = 7;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_FIN_FLAG_EN_SHIFT: c_int = 8;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 9;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SET_RTO_CF_SHIFT: c_int = 10;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TS_TO_ECHO_UPDATED_CF_SHIFT: c_int = 12;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_DEST_UPDATED_CF_SHIFT: c_int = 14;

    pub tcp_agg_vars2: u16,

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_FIN_FLAG_SHIFT: c_int = 0;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_UNBLOCKED_SHIFT: c_int = 1;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_TIMER_ACTIVE_SHIFT: c_int = 2;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX3_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX4_FLAG_SHIFT: c_int = 4;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DA_ENABLE_SHIFT: c_int = 5;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_ACK_TO_FE_UPDATED_EN_SHIFT: c_int = 6;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SIDEBAND_SENT_CF_EN_SHIFT: c_int = 7;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_FIN_FLAG_EN_SHIFT: c_int = 8;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX1_FLAG_SHIFT: c_int = 9;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_SET_RTO_CF_SHIFT: c_int = 10;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TS_TO_ECHO_UPDATED_CF_SHIFT: c_int = 12;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_DEST_UPDATED_CF_SHIFT: c_int = 14;
    pub __tx_dest: u8,
    pub __agg_val8_th: u8,

    pub ack_to_far_end: u32,
    pub rto_timer: u32,
    pub ka_timer: u32,
    pub ts_to_echo: u32,

    pub __agg_val7_th: u16,
    pub __agg_val7: u16,

    pub __agg_val7: u16,
    pub __agg_val7_th: u16,

    pub __tcp_agg_vars5: u8,
    pub __tcp_agg_vars4: u8,
    pub __tcp_agg_vars3: u8,
    pub _pure_ack_cnt: u8,

    pub _pure_ack_cnt: u8,
    pub __tcp_agg_vars3: u8,
    pub __tcp_agg_vars4: u8,
    pub __tcp_agg_vars5: u8,

    pub tcp_agg_vars6: u32,

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TS_TO_ECHO_CF_EN_SHIFT: c_int = 0;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TX_DEST_UPDATED_CF_EN_SHIFT: c_int = 1;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX9_CF_EN_SHIFT: c_int = 2;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX10_CF_EN_SHIFT: c_int = 3;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX6_FLAG_SHIFT: c_int = 4;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX7_FLAG_SHIFT: c_int = 5;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX5_CF_SHIFT: c_int = 6;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX9_CF_SHIFT: c_int = 8;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX10_CF_SHIFT: c_int = 10;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX11_CF_SHIFT: c_int = 12;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX12_CF_SHIFT: c_int = 14;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX13_CF_SHIFT: c_int = 16;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX14_CF_SHIFT: c_int = 18;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX15_CF_SHIFT: c_int = 20;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX16_CF_SHIFT: c_int = 22;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_AUX17_CF_SHIFT: c_int = 24;

pub const XSTORM_TCP_TCP_AG_CONTEXT_SECTION_ECE_FLAG_SHIFT: c_int = 26;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_RESERVED71_SHIFT: c_int = 27;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_FORCE_PURE_ACK_CNT_DIRTY_SHIFT: c_int = 28;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_TCP_AUTO_STOP_FLAG_SHIFT: c_int = 29;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_DO_TS_UPDATE_FLAG_SHIFT: c_int = 30;

pub const __XSTORM_TCP_TCP_AG_CONTEXT_SECTION_CANCEL_RETRANSMIT_FLAG_SHIFT: c_int = 31;

    pub __agg_misc6: u16,
    pub __tcp_agg_vars7: u16,

    pub __tcp_agg_vars7: u16,
    pub __agg_misc6: u16,

    pub __agg_val10: u32,
    pub __agg_val10_th: u32,

    pub __reserved3: u16,
    pub __reserved2: u8,
    pub __da_only_cnt: u8,

    pub __da_only_cnt: u8,
    pub __reserved2: u8,
    pub __reserved3: u16,

}

//
// The iscsi aggregative context of Xstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_ag_context {

    pub agg_val1: u16,
    pub agg_vars1: u8,

pub const __XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_ISCSI_AG_CONTEXT_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const __XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_ISCSI_AG_CONTEXT_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub agg_val1: u16,

    pub cdu_reserved: u8,
    pub __agg_vars4: u8,
    pub agg_vars3: u8,

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_RX_TS_EN_CF_SHIFT: c_int = 6;
    pub agg_vars2: u8,

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_CF_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;

    pub agg_vars2: u8,

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_CF_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const __XSTORM_ISCSI_AG_CONTEXT_DQ_CF_EN_SHIFT: c_int = 7;
    pub agg_vars3: u8,

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_RX_TS_EN_CF_SHIFT: c_int = 6;
    pub __agg_vars4: u8,
    pub cdu_reserved: u8,

    pub more_to_send: u32,

    pub agg_vars5: u16,

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE2_SHIFT: c_int = 14;
    pub sq_cons: u16,

    pub sq_cons: u16,
    pub agg_vars5: u16,

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE2_SHIFT: c_int = 14;

    pub tcp: xstorm_tcp_tcp_ag_context_section,

    pub agg_vars7: u16,

pub const __XSTORM_ISCSI_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_STORMS_SYNC_CF_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_ISCSI_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_ISCSI_AG_CONTEXT_COMPLETION_SEQ_DECISION_MASK_SHIFT: c_int = 10;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_ISCSI_AG_CONTEXT_RX_WND_SCL_EN_SHIFT: c_int = 15;
    pub agg_val3_th: u8,
    pub agg_vars6: u8,

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE7_SHIFT: c_int = 3;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE4_SHIFT: c_int = 6;

    pub agg_vars6: u8,

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE7_SHIFT: c_int = 3;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE4_SHIFT: c_int = 6;
    pub agg_val3_th: u8,
    pub agg_vars7: u16,

pub const __XSTORM_ISCSI_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_ISCSI_AG_CONTEXT_STORMS_SYNC_CF_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_ISCSI_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_ISCSI_AG_CONTEXT_COMPLETION_SEQ_DECISION_MASK_SHIFT: c_int = 10;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_ISCSI_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_ISCSI_AG_CONTEXT_RX_WND_SCL_EN_SHIFT: c_int = 15;

    pub __agg_val11_th: u16,
    pub __gen_data: u16,

    pub __gen_data: u16,
    pub __agg_val11_th: u16,

    pub __reserved1: u8,
    pub __agg_val6_th: u8,
    pub __agg_val9: u16,

    pub __agg_val9: u16,
    pub __agg_val6_th: u8,
    pub __reserved1: u8,

    pub hq_prod: u16,
    pub hq_cons: u16,

    pub hq_cons: u16,
    pub hq_prod: u16,

    pub agg_vars8: u32,

pub const XSTORM_ISCSI_AG_CONTEXT_AGG_MISC2_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_AG_CONTEXT_AGG_MISC3_SHIFT: c_int = 24;

    pub r2tq_prod: u16,
    pub sq_prod: u16,

    pub sq_prod: u16,
    pub r2tq_prod: u16,

    pub agg_val3: u8,
    pub agg_val6: u8,
    pub agg_val5_th: u8,
    pub agg_val5: u8,

    pub agg_val5: u8,
    pub agg_val5_th: u8,
    pub agg_val6: u8,
    pub agg_val3: u8,

    pub __agg_misc1: u16,
    pub agg_limit1: u16,

    pub agg_limit1: u16,
    pub __agg_misc1: u16,

    pub hq_cons_tcp_seq: u32,
    pub exp_stat_sn: u32,
    pub rst_seq_num: u32,
}

//
// The L5cm aggregative context of XStorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_l5cm_ag_context {

    pub agg_val1: u16,
    pub agg_vars1: u8,

pub const __XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_L5CM_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_L5CM_AG_CONTEXT_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub state: u8,

    pub state: u8,
    pub agg_vars1: u8,

pub const __XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM0_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM1_SHIFT: c_int = 1;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM2_SHIFT: c_int = 2;

pub const XSTORM_L5CM_AG_CONTEXT_EXISTS_IN_QM3_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_MORE_TO_SEND_EN_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_NAGLE_EN_SHIFT: c_int = 5;

pub const __XSTORM_L5CM_AG_CONTEXT_DQ_SPARE_FLAG_SHIFT: c_int = 6;

pub const __XSTORM_L5CM_AG_CONTEXT_UNA_GT_NXT_EN_SHIFT: c_int = 7;
    pub agg_val1: u16,

    pub cdu_reserved: u8,
    pub __agg_vars4: u8,
    pub agg_vars3: u8,

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_RX_TS_EN_CF_SHIFT: c_int = 6;
    pub agg_vars2: u8,

pub const XSTORM_L5CM_AG_CONTEXT_AUX4_CF_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const XSTORM_L5CM_AG_CONTEXT_AUX4_CF_EN_SHIFT: c_int = 7;

    pub agg_vars2: u8,

pub const XSTORM_L5CM_AG_CONTEXT_AUX4_CF_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_DQ_SPARE_FLAG_EN_SHIFT: c_int = 2;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX8_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX9_FLAG_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE1_SHIFT: c_int = 5;

pub const XSTORM_L5CM_AG_CONTEXT_AUX4_CF_EN_SHIFT: c_int = 7;
    pub agg_vars3: u8,

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM2_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_RX_TS_EN_CF_SHIFT: c_int = 6;
    pub __agg_vars4: u8,
    pub cdu_reserved: u8,

    pub more_to_send: u32,

    pub agg_vars5: u16,

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE2_SHIFT: c_int = 14;
    pub agg_val4_th: u16,

    pub agg_val4_th: u16,
    pub agg_vars5: u16,

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE5_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM0_SHIFT: c_int = 2;

pub const XSTORM_L5CM_AG_CONTEXT_PHYSICAL_QUEUE_NUM1_SHIFT: c_int = 8;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE2_SHIFT: c_int = 14;

    pub tcp: xstorm_tcp_tcp_ag_context_section,

    pub agg_vars7: u16,

pub const __XSTORM_L5CM_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_STORMS_SYNC_CF_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_L5CM_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_L5CM_AG_CONTEXT_COMPLETION_SEQ_DECISION_MASK_SHIFT: c_int = 10;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_L5CM_AG_CONTEXT_RX_WND_SCL_EN_SHIFT: c_int = 15;
    pub agg_val3_th: u8,
    pub agg_vars6: u8,

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE7_SHIFT: c_int = 3;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE4_SHIFT: c_int = 6;

    pub agg_vars6: u8,

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE6_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE7_SHIFT: c_int = 3;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE4_SHIFT: c_int = 6;
    pub agg_val3_th: u8,
    pub agg_vars7: u16,

pub const __XSTORM_L5CM_AG_CONTEXT_AGG_VAL11_DECISION_RULE_SHIFT: c_int = 0;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX13_FLAG_SHIFT: c_int = 3;

pub const __XSTORM_L5CM_AG_CONTEXT_STORMS_SYNC_CF_SHIFT: c_int = 4;

pub const XSTORM_L5CM_AG_CONTEXT_DECISION_RULE3_SHIFT: c_int = 6;

pub const XSTORM_L5CM_AG_CONTEXT_AUX1_CF_SHIFT: c_int = 8;

pub const __XSTORM_L5CM_AG_CONTEXT_COMPLETION_SEQ_DECISION_MASK_SHIFT: c_int = 10;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX1_CF_EN_SHIFT: c_int = 11;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX10_FLAG_SHIFT: c_int = 12;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX11_FLAG_SHIFT: c_int = 13;

pub const __XSTORM_L5CM_AG_CONTEXT_AUX12_FLAG_SHIFT: c_int = 14;

pub const __XSTORM_L5CM_AG_CONTEXT_RX_WND_SCL_EN_SHIFT: c_int = 15;

    pub __agg_val11_th: u16,
    pub __gen_data: u16,

    pub __gen_data: u16,
    pub __agg_val11_th: u16,

    pub __reserved1: u8,
    pub __agg_val6_th: u8,
    pub __agg_val9: u16,

    pub __agg_val9: u16,
    pub __agg_val6_th: u8,
    pub __reserved1: u8,

    pub agg_val2_th: u16,
    pub agg_val2: u16,

    pub agg_val2: u16,
    pub agg_val2_th: u16,

    pub agg_vars8: u32,

pub const XSTORM_L5CM_AG_CONTEXT_AGG_MISC2_SHIFT: c_int = 0;

pub const XSTORM_L5CM_AG_CONTEXT_AGG_MISC3_SHIFT: c_int = 24;

    pub agg_misc0: u16,
    pub agg_val4: u16,

    pub agg_val4: u16,
    pub agg_misc0: u16,

    pub agg_val3: u8,
    pub agg_val6: u8,
    pub agg_val5_th: u8,
    pub agg_val5: u8,

    pub agg_val5: u8,
    pub agg_val5_th: u8,
    pub agg_val6: u8,
    pub agg_val3: u8,

    pub __agg_misc1: u16,
    pub agg_limit1: u16,

    pub agg_limit1: u16,
    pub __agg_misc1: u16,

    pub completion_seq: u32,
    pub agg_misc4: u32,
    pub rst_seq_num: u32,
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

pub const FCOE_TCE_TX_WR_RX_RD_CONST_TX_COMP_TRNS_SHIFT: c_int = 7;
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
// Cached SGEs $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_fcoe_sgl {
    pub sge: [fcoe_bd_ctx; 3],
}

//
// FCoE SQ\XFRQ element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cached_wqe {
    pub sqe: fcoe_sqe,
    pub xfrqe: fcoe_xfrqe,
}

//
// FCoE connection enable\disable params passed by driver to FW in FCoE enable
// ramrod $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_enable_disable_ramrod_params {
    pub enable_disable_kwqe: fcoe_kwqe_conn_enable_disable,
}

//
// FCoE connection offload params passed by driver to FW in FCoE offload ramrod
// $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_offload_ramrod_params {
    pub offload_kwqe1: fcoe_kwqe_conn_offload1,
    pub offload_kwqe2: fcoe_kwqe_conn_offload2,
    pub offload_kwqe3: fcoe_kwqe_conn_offload3,
    pub offload_kwqe4: fcoe_kwqe_conn_offload4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_mng_ctx {

    pub mid_seq_proc_flag: u8,
    pub tce_in_cam_flag: u8,
    pub tce_on_ior_flag: u8,
    pub en_cached_tce_flag: u8,

    pub en_cached_tce_flag: u8,
    pub tce_on_ior_flag: u8,
    pub tce_in_cam_flag: u8,
    pub mid_seq_proc_flag: u8,

    pub tce_cam_addr: u8,
    pub cached_conn_flag: u8,
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub cached_conn_flag: u8,
    pub tce_cam_addr: u8,

    pub dma_tce_ram_addr: u16,
    pub tce_ram_addr: u16,

    pub tce_ram_addr: u16,
    pub dma_tce_ram_addr: u16,

    pub ox_id: u16,
    pub wr_done_seq: u16,

    pub wr_done_seq: u16,
    pub ox_id: u16,

    pub task_addr: regpair,
}

//
// Parameters initialized during offloaded according to FLOGI/PLOGI/PRLI and
// used in FCoE context section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_params {

    pub fcoe_conn_id: u16,
    pub flags: u16,

pub const USTORM_FCOE_PARAMS_B_MUL_N_PORT_IDS_SHIFT: c_int = 0;

pub const USTORM_FCOE_PARAMS_B_E_D_TOV_RES_SHIFT: c_int = 1;

pub const USTORM_FCOE_PARAMS_B_CONT_INCR_SEQ_CNT_SHIFT: c_int = 2;

pub const USTORM_FCOE_PARAMS_B_CONF_REQ_SHIFT: c_int = 3;

pub const USTORM_FCOE_PARAMS_B_REC_VALID_SHIFT: c_int = 4;

pub const USTORM_FCOE_PARAMS_B_CQ_TOGGLE_BIT_SHIFT: c_int = 5;

pub const USTORM_FCOE_PARAMS_B_XFRQ_TOGGLE_BIT_SHIFT: c_int = 6;

pub const USTORM_FCOE_PARAMS_RSRV0_SHIFT: c_int = 7;

    pub flags: u16,

pub const USTORM_FCOE_PARAMS_B_MUL_N_PORT_IDS_SHIFT: c_int = 0;

pub const USTORM_FCOE_PARAMS_B_E_D_TOV_RES_SHIFT: c_int = 1;

pub const USTORM_FCOE_PARAMS_B_CONT_INCR_SEQ_CNT_SHIFT: c_int = 2;

pub const USTORM_FCOE_PARAMS_B_CONF_REQ_SHIFT: c_int = 3;

pub const USTORM_FCOE_PARAMS_B_REC_VALID_SHIFT: c_int = 4;

pub const USTORM_FCOE_PARAMS_B_CQ_TOGGLE_BIT_SHIFT: c_int = 5;

pub const USTORM_FCOE_PARAMS_B_XFRQ_TOGGLE_BIT_SHIFT: c_int = 6;

pub const USTORM_FCOE_PARAMS_RSRV0_SHIFT: c_int = 7;
    pub fcoe_conn_id: u16,

    pub hc_csdm_byte_en: u8,
    pub func_id: u8,
    pub port_id: u8,
    pub vnic_id: u8,

    pub vnic_id: u8,
    pub port_id: u8,
    pub func_id: u8,
    pub hc_csdm_byte_en: u8,

    pub rx_total_conc_seqs: u16,
    pub rx_max_fc_pay_len: u16,

    pub rx_max_fc_pay_len: u16,
    pub rx_total_conc_seqs: u16,

    pub task_pbe_idx_off: u8,
    pub task_in_page_log_size: u8,
    pub rx_max_conc_seqs: u16,

    pub rx_max_conc_seqs: u16,
    pub task_in_page_log_size: u8,
    pub task_pbe_idx_off: u8,

}

//
// FCoE 16-bits index structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_idx16_fields {
    pub fields: u16,

pub const FCOE_IDX16_FIELDS_IDX_SHIFT: c_int = 0;

pub const FCOE_IDX16_FIELDS_MSB_SHIFT: c_int = 15;
}

//
// FCoE 16-bits index union
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_idx16_field_union {
    pub fields: fcoe_idx16_fields,
    pub val: u16,
}

//
// Parameters required for placement according to SGL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_data_place_mng {

    pub sge_off: u16,
    pub num_sges: u8,
    pub sge_idx: u8,

    pub sge_idx: u8,
    pub num_sges: u8,
    pub sge_off: u16,

}

//
// Parameters required for placement according to SGL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_data_place {
    pub cached_mng: ustorm_fcoe_data_place_mng,
    pub cached_sge: [fcoe_bd_ctx; 2],
}

//
// TX processing shall write and RX processing shall read from this section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_u_tce_tx_wr_rx_rd_union {
    pub abts: fcoe_abts_info,
    pub cleanup: fcoe_cleanup_info,
    pub tx_seq_ctx: fcoe_fw_tx_seq_ctx,
    pub opaque: [u32; 2],
}

//
// TX processing shall write and RX processing shall read from this section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_u_tce_tx_wr_rx_rd {
    pub union_ctx: fcoe_u_tce_tx_wr_rx_rd_union,
    pub const_ctx: fcoe_tce_tx_wr_rx_rd_const,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_tce {
    pub txwr_rxrd: fcoe_u_tce_tx_wr_rx_rd,
    pub rxwr_txrd: fcoe_tce_rx_wr_tx_rd,
    pub rxwr: fcoe_tce_rx_only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_cache_ctx {
    pub rsrv0: u32,
    pub data_place: ustorm_fcoe_data_place,
    pub tce: ustorm_fcoe_tce,
}

//
// Ustorm FCoE Storm Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_st_context {
    pub mng_ctx: ustorm_fcoe_mng_ctx,
    pub fcoe_params: ustorm_fcoe_params,
    pub cq_base_addr: regpair,
    pub rq_pbl_base: regpair,
    pub rq_cur_page_addr: regpair,
    pub confq_pbl_base_addr: regpair,
    pub conn_db_base: regpair,
    pub xfrq_base_addr: regpair,
    pub lcq_base_addr: regpair,

    pub rq_cons: fcoe_idx16_field_union,
    pub rq_prod: fcoe_idx16_field_union,

    pub rq_prod: fcoe_idx16_field_union,
    pub rq_cons: fcoe_idx16_field_union,

    pub xfrq_prod: u16,
    pub cq_cons: u16,

    pub cq_cons: u16,
    pub xfrq_prod: u16,

    pub lcq_cons: u16,
    pub hc_cram_address: u16,

    pub hc_cram_address: u16,
    pub lcq_cons: u16,

    pub sq_xfrq_lcq_confq_size: u16,
    pub confq_prod: u16,

    pub confq_prod: u16,
    pub sq_xfrq_lcq_confq_size: u16,

    pub hc_csdm_agg_int: u8,
    pub rsrv2: u8,
    pub available_rqes: u8,
    pub sp_q_flush_cnt: u8,

    pub sp_q_flush_cnt: u8,
    pub available_rqes: u8,
    pub rsrv2: u8,
    pub hc_csdm_agg_int: u8,

    pub num_pend_tasks: u16,
    pub pbf_ack_ram_addr: u16,

    pub pbf_ack_ram_addr: u16,
    pub num_pend_tasks: u16,

    pub cache_ctx: ustorm_fcoe_cache_ctx,
}

//
// The FCoE non-aggregative context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_st_context {
    pub reserved0: regpair,
    pub reserved1: regpair,
}

//
// Ethernet context section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_eth_context_section {

    pub remote_addr_4: u8,
    pub remote_addr_5: u8,
    pub local_addr_0: u8,
    pub local_addr_1: u8,

    pub local_addr_1: u8,
    pub local_addr_0: u8,
    pub remote_addr_5: u8,
    pub remote_addr_4: u8,

    pub remote_addr_0: u8,
    pub remote_addr_1: u8,
    pub remote_addr_2: u8,
    pub remote_addr_3: u8,

    pub remote_addr_3: u8,
    pub remote_addr_2: u8,
    pub remote_addr_1: u8,
    pub remote_addr_0: u8,

    pub reserved_vlan_type: u16,
    pub params: u16,

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_VLAN_ID_SHIFT: c_int = 0;

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_CFI_SHIFT: c_int = 12;

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_PRIORITY_SHIFT: c_int = 13;

    pub params: u16,

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_VLAN_ID_SHIFT: c_int = 0;

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_CFI_SHIFT: c_int = 12;

pub const XSTORM_FCOE_ETH_CONTEXT_SECTION_PRIORITY_SHIFT: c_int = 13;
    pub reserved_vlan_type: u16,

    pub local_addr_2: u8,
    pub local_addr_3: u8,
    pub local_addr_4: u8,
    pub local_addr_5: u8,

    pub local_addr_5: u8,
    pub local_addr_4: u8,
    pub local_addr_3: u8,
    pub local_addr_2: u8,

}

//
// Flags used in FCoE context section - 1 byte
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_context_flags {
    pub flags: u8,

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_PROC_Q_SHIFT: c_int = 0;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_MID_SEQ_SHIFT: c_int = 2;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_BLOCK_SQ_SHIFT: c_int = 3;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_REC_SUPPORT_SHIFT: c_int = 4;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_SQ_TOGGLE_SHIFT: c_int = 5;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_XFRQ_TOGGLE_SHIFT: c_int = 6;

pub const XSTORM_FCOE_CONTEXT_FLAGS_B_VNTAG_VLAN_SHIFT: c_int = 7;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_tce {
    pub txwr: fcoe_tce_tx_only,
    pub txwr_rxrd: fcoe_tce_tx_wr_rx_rd,
}

//
// FCP_DATA parameters required for transmission
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_fcp_data {
    pub io_rem: u32,

    pub cached_sge_off: u16,
    pub cached_num_sges: u8,
    pub cached_sge_idx: u8,

    pub cached_sge_idx: u8,
    pub cached_num_sges: u8,
    pub cached_sge_off: u16,

    pub buf_addr_hi_0: u32,
    pub buf_addr_lo_0: u32,

    pub num_of_pending_tasks: u16,
    pub buf_len_0: u16,

    pub buf_len_0: u16,
    pub num_of_pending_tasks: u16,

    pub buf_addr_hi_1: u32,
    pub buf_addr_lo_1: u32,

    pub task_pbe_idx_off: u16,
    pub buf_len_1: u16,

    pub buf_len_1: u16,
    pub task_pbe_idx_off: u16,

    pub buf_addr_hi_2: u32,
    pub buf_addr_lo_2: u32,

    pub ox_id: u16,
    pub buf_len_2: u16,

    pub buf_len_2: u16,
    pub ox_id: u16,

}

//
// vlan configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_vlan_conf {
    pub vlan_conf: u8,

pub const XSTORM_FCOE_VLAN_CONF_PRIORITY_SHIFT: c_int = 0;

pub const XSTORM_FCOE_VLAN_CONF_INNER_VLAN_FLAG_SHIFT: c_int = 3;

pub const XSTORM_FCOE_VLAN_CONF_RESERVED_SHIFT: c_int = 4;
}

//
// FCoE 16-bits vlan structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_vlan_fields {
    pub fields: u16,

pub const FCOE_VLAN_FIELDS_VID_SHIFT: c_int = 0;

pub const FCOE_VLAN_FIELDS_CLI_SHIFT: c_int = 12;

pub const FCOE_VLAN_FIELDS_PRI_SHIFT: c_int = 13;
}

//
// FCoE 16-bits vlan union
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_vlan_field_union {
    pub fields: fcoe_vlan_fields,
    pub val: u16,
}

//
// FCoE 16-bits vlan, vif union
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_vlan_vif_field_union {
    pub vlan: fcoe_vlan_field_union,
    pub vif: u16,
}

//
// FCoE context section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_context_section {

    pub cs_ctl: u8,
    pub s_id: [u8; 3],    pub s_id: [u8; 3],
    pub cs_ctl: u8,

    pub rctl: u8,
    pub d_id: [u8; 3],    pub d_id: [u8; 3],
    pub rctl: u8,

    pub sq_xfrq_lcq_confq_size: u16,
    pub tx_max_fc_pay_len: u16,

    pub tx_max_fc_pay_len: u16,
    pub sq_xfrq_lcq_confq_size: u16,

    pub lcq_prod: u32,

    pub port_id: u8,
    pub func_id: u8,
    pub seq_id: u8,
    pub tx_flags: xstorm_fcoe_context_flags,

    pub tx_flags: xstorm_fcoe_context_flags,
    pub seq_id: u8,
    pub func_id: u8,
    pub port_id: u8,

    pub mtu: u16,
    pub func_mode: u8,
    pub vnic_id: u8,

    pub vnic_id: u8,
    pub func_mode: u8,
    pub mtu: u16,

    pub confq_curr_page_addr: regpair,
    pub cached_wqe: [fcoe_cached_wqe; 8],
    pub lcq_base_addr: regpair,
    pub tce: xstorm_fcoe_tce,
    pub fcp_data: xstorm_fcoe_fcp_data,

    pub tx_max_conc_seqs_c3: u8,
    pub vlan_flag: u8,
    pub dcb_val: u8,
    pub data_pb_cmd_size: u8,

    pub data_pb_cmd_size: u8,
    pub dcb_val: u8,
    pub vlan_flag: u8,
    pub tx_max_conc_seqs_c3: u8,

    pub fcoe_tx_stat_params_ram_addr: u16,
    pub fcoe_tx_fc_seq_ram_addr: u16,

    pub fcoe_tx_fc_seq_ram_addr: u16,
    pub fcoe_tx_stat_params_ram_addr: u16,

    pub fcp_cmd_line_credit: u8,
    pub eth_hdr_size: u8,
    pub pbf_addr: u16,

    pub pbf_addr: u16,
    pub eth_hdr_size: u8,
    pub fcp_cmd_line_credit: u8,

    pub multi_func_val: fcoe_vlan_vif_field_union,
    pub page_log_size: u8,
    pub orig_vlan_conf: xstorm_fcoe_vlan_conf,

    pub orig_vlan_conf: xstorm_fcoe_vlan_conf,
    pub page_log_size: u8,
    pub multi_func_val: fcoe_vlan_vif_field_union,

    pub fcp_cmd_frame_size: u16,
    pub pbf_addr_ff: u16,

    pub pbf_addr_ff: u16,
    pub fcp_cmd_frame_size: u16,

    pub vlan_num: u8,
    pub cos: u8,
    pub cache_xfrq_cons: u8,
    pub cache_sq_cons: u8,

    pub cache_sq_cons: u8,
    pub cache_xfrq_cons: u8,
    pub cos: u8,
    pub vlan_num: u8,

    pub verify_tx_seq: u32,
}

//
// Xstorm FCoE Storm Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_st_context {
    pub eth: xstorm_fcoe_eth_context_section,
    pub fcoe: xstorm_fcoe_context_section,
}

//
// Fcoe connection context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_context {
    pub ustorm_st_context: ustorm_fcoe_st_context,
    pub tstorm_st_context: tstorm_fcoe_st_context,
    pub xstorm_ag_context: xstorm_fcoe_ag_context,
    pub tstorm_ag_context: tstorm_fcoe_ag_context,
    pub ustorm_ag_context: ustorm_fcoe_ag_context,
    pub timers_context: timers_block_context,
    pub xstorm_st_context: xstorm_fcoe_st_context,
}

//
// FCoE init params passed by driver to FW in FCoE init ramrod
// $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_init_ramrod_params {
    pub init_kwqe1: fcoe_kwqe_init1,
    pub init_kwqe2: fcoe_kwqe_init2,
    pub init_kwqe3: fcoe_kwqe_init3,
    pub eq_pbl_base: regpair,
    pub eq_pbl_size: __le32,
    pub reserved2: __le32,
    pub eq_prod: __le16,
    pub sb_num: __le16,
    pub sb_id: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
}

//
// FCoE statistics params buffer passed by driver to FW in FCoE statistics
// ramrod $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_stat_ramrod_params {
    pub stat_kwqe: fcoe_kwqe_stat,
}

//
// CQ DB CQ producer and pending completion counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cq_db_prod_pnd_cmpltn_cnt {

    pub cntr: u16,
    pub prod: u16,

    pub prod: u16,
    pub cntr: u16,

}

//
// CQ DB pending completion ITT array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cq_db_prod_pnd_cmpltn_cnt_arr {
    pub prod_pend_comp: [iscsi_cq_db_prod_pnd_cmpltn_cnt; 8],
}

//
// Cstorm CQ sequence to notify array, updated by driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cq_db_sqn_2_notify_arr {
    pub sqn: [u16; 8],
}

//
// Cstorm iSCSI Storm Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstorm_iscsi_st_context {
    pub cq_c_prod_pend_comp_ctr_arr: iscsi_cq_db_prod_pnd_cmpltn_cnt_arr,
    pub cq_c_prod_sqn_arr: iscsi_cq_db_sqn_2_notify_arr,
    pub cq_c_sqn_2_notify_arr: iscsi_cq_db_sqn_2_notify_arr,
    pub hq_pbl_base: regpair,
    pub hq_curr_pbe: regpair,
    pub task_pbl_base: regpair,
    pub cq_db_base: regpair,

    pub hq_bd_itt: u16,
    pub iscsi_conn_id: u16,

    pub iscsi_conn_id: u16,
    pub hq_bd_itt: u16,

    pub hq_bd_data_segment_len: u32,
    pub hq_bd_buffer_offset: u32,

    pub rsrv: u8,
    pub cq_proc_en_bit_map: u8,
    pub cq_pend_comp_itt_valid_bit_map: u8,
    pub hq_bd_opcode: u8,

    pub hq_bd_opcode: u8,
    pub cq_pend_comp_itt_valid_bit_map: u8,
    pub cq_proc_en_bit_map: u8,
    pub rsrv: u8,

    pub hq_tcp_seq: u32,

    pub flags: u16,

pub const CSTORM_ISCSI_ST_CONTEXT_DATA_DIGEST_EN_SHIFT: c_int = 0;

pub const CSTORM_ISCSI_ST_CONTEXT_HDR_DIGEST_EN_SHIFT: c_int = 1;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_CTXT_VALID_SHIFT: c_int = 2;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_LCL_CMPLN_FLG_SHIFT: c_int = 3;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_WRITE_TASK_SHIFT: c_int = 4;

pub const CSTORM_ISCSI_ST_CONTEXT_CTRL_FLAGS_RSRV_SHIFT: c_int = 5;
    pub hq_cons: u16,

    pub hq_cons: u16,
    pub flags: u16,

pub const CSTORM_ISCSI_ST_CONTEXT_DATA_DIGEST_EN_SHIFT: c_int = 0;

pub const CSTORM_ISCSI_ST_CONTEXT_HDR_DIGEST_EN_SHIFT: c_int = 1;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_CTXT_VALID_SHIFT: c_int = 2;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_LCL_CMPLN_FLG_SHIFT: c_int = 3;

pub const CSTORM_ISCSI_ST_CONTEXT_HQ_BD_WRITE_TASK_SHIFT: c_int = 4;

pub const CSTORM_ISCSI_ST_CONTEXT_CTRL_FLAGS_RSRV_SHIFT: c_int = 5;

    pub rsrv1: regpair,
}

//
// SCSI read/write SQ WQE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cmd_pdu_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_ATTRIBUTES_SHIFT: c_int = 0;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 3;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_WRITE_FLAG_SHIFT: c_int = 5;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_READ_FLAG_SHIFT: c_int = 6;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_FINAL_FLAG_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_ATTRIBUTES_SHIFT: c_int = 0;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 3;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_WRITE_FLAG_SHIFT: c_int = 5;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_READ_FLAG_SHIFT: c_int = 6;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_FINAL_FLAG_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_CMD_PDU_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: u32,
    pub expected_data_transfer_length: u32,
    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub scsi_command_block: [u32; 4],
}

//
// Buffer per connection, used in Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_buf {
    pub reserved: [regpair; 8],
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_rq_db {
    pub pbl_base: regpair,
    pub curr_pbe: regpair,
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_r2tq_db {
    pub pbl_base: regpair,
    pub curr_pbe: regpair,
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_cq_db {

    pub cq_sn: u16,
    pub prod: u16,

    pub prod: u16,
    pub cq_sn: u16,

    pub curr_pbe: regpair,
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rings_db {
    pub rq: ustorm_iscsi_rq_db,
    pub r2tq: ustorm_iscsi_r2tq_db,
    pub cq: [ustorm_iscsi_cq_db; 8],
    pub rq_prod: u16,
    pub r2tq_prod: u16,

    pub r2tq_prod: u16,
    pub rq_prod: u16,

    pub cq_pbl_base: regpair,
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_placement_db {
    pub sgl_base_lo: u32,
    pub sgl_base_hi: u32,
    pub local_sge_0_address_hi: u32,
    pub local_sge_0_address_lo: u32,

    pub curr_sge_offset: u16,
    pub local_sge_0_size: u16,

    pub local_sge_0_size: u16,
    pub curr_sge_offset: u16,

    pub local_sge_1_address_hi: u32,
    pub local_sge_1_address_lo: u32,

    pub exp_padding_2b: u8,
    pub nal_len_3b: u8,
    pub local_sge_1_size: u16,

    pub local_sge_1_size: u16,
    pub nal_len_3b: u8,
    pub exp_padding_2b: u8,

    pub sgl_size: u8,
    pub local_sge_index_2b: u8,
    pub reserved7: u16,

    pub reserved7: u16,
    pub local_sge_index_2b: u8,
    pub sgl_size: u8,

    pub rem_pdu: u32,
    pub place_db_bitfield_1: u32,

pub const USTORM_ISCSI_PLACEMENT_DB_REM_PDU_PAYLOAD_SHIFT: c_int = 0;

pub const USTORM_ISCSI_PLACEMENT_DB_CQ_ID_SHIFT: c_int = 24;
    pub place_db_bitfield_2: u32,

pub const USTORM_ISCSI_PLACEMENT_DB_BYTES_2_TRUNCATE_SHIFT: c_int = 0;

pub const USTORM_ISCSI_PLACEMENT_DB_HOST_SGE_INDEX_SHIFT: c_int = 24;
    pub nal: u32,

pub const USTORM_ISCSI_PLACEMENT_DB_REM_SGE_SIZE_SHIFT: c_int = 0;

pub const USTORM_ISCSI_PLACEMENT_DB_EXP_DIGEST_3B_SHIFT: c_int = 24;
}

//
// Ustorm iSCSI Storm Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_st_context {
    pub exp_stat_sn: u32,
    pub exp_data_sn: u32,
    pub ring: rings_db,
    pub task_pbl_base: regpair,
    pub tce_phy_addr: regpair,
    pub place_db: ustorm_iscsi_placement_db,
    pub reserved8: u32,
    pub rem_rcv_len: u32,

    pub hdr_itt: u16,
    pub iscsi_conn_id: u16,

    pub iscsi_conn_id: u16,
    pub hdr_itt: u16,

    pub nal_bytes: u32,

    pub hdr_second_byte_union: u8,
    pub bitfield_0: u8,

pub const USTORM_ISCSI_ST_CONTEXT_BMIDDLEOFPDU_SHIFT: c_int = 0;

pub const USTORM_ISCSI_ST_CONTEXT_BFENCECQE_SHIFT: c_int = 1;

pub const USTORM_ISCSI_ST_CONTEXT_BRESETCRC_SHIFT: c_int = 2;

pub const USTORM_ISCSI_ST_CONTEXT_RESERVED1_SHIFT: c_int = 3;
    pub task_pdu_cache_index: u8,
    pub task_pbe_cache_index: u8,

    pub task_pbe_cache_index: u8,
    pub task_pdu_cache_index: u8,
    pub bitfield_0: u8,

pub const USTORM_ISCSI_ST_CONTEXT_BMIDDLEOFPDU_SHIFT: c_int = 0;

pub const USTORM_ISCSI_ST_CONTEXT_BFENCECQE_SHIFT: c_int = 1;

pub const USTORM_ISCSI_ST_CONTEXT_BRESETCRC_SHIFT: c_int = 2;

pub const USTORM_ISCSI_ST_CONTEXT_RESERVED1_SHIFT: c_int = 3;
    pub hdr_second_byte_union: u8,

    pub reserved3: u16,
    pub reserved2: u8,
    pub acDecrement: u8,

    pub acDecrement: u8,
    pub reserved2: u8,
    pub reserved3: u16,

    pub task_stat: u32,

    pub hdr_opcode: u8,
    pub num_cqs: u8,
    pub reserved5: u16,

    pub reserved5: u16,
    pub num_cqs: u8,
    pub hdr_opcode: u8,

    pub negotiated_rx: u32,

pub const USTORM_ISCSI_ST_CONTEXT_MAX_RECV_PDU_LENGTH_SHIFT: c_int = 0;

pub const USTORM_ISCSI_ST_CONTEXT_MAX_OUTSTANDING_R2TS_SHIFT: c_int = 24;
    pub negotiated_rx_and_flags: u32,

pub const USTORM_ISCSI_ST_CONTEXT_MAX_BURST_LENGTH_SHIFT: c_int = 0;

pub const USTORM_ISCSI_ST_CONTEXT_B_CQE_POSTED_OR_HEADER_CACHED_SHIFT: c_int = 24;

pub const USTORM_ISCSI_ST_CONTEXT_B_HDR_DIGEST_EN_SHIFT: c_int = 25;

pub const USTORM_ISCSI_ST_CONTEXT_B_DATA_DIGEST_EN_SHIFT: c_int = 26;

pub const USTORM_ISCSI_ST_CONTEXT_B_PROTOCOL_ERROR_SHIFT: c_int = 27;

pub const USTORM_ISCSI_ST_CONTEXT_B_TASK_VALID_SHIFT: c_int = 28;

pub const USTORM_ISCSI_ST_CONTEXT_TASK_TYPE_SHIFT: c_int = 29;

pub const USTORM_ISCSI_ST_CONTEXT_B_ALL_DATA_ACKED_SHIFT: c_int = 31;
}

//
// TCP context region, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_tcp_st_context_section {
    pub flags1: u32,

pub const TSTORM_TCP_ST_CONTEXT_SECTION_RTT_SRTT_SHIFT: c_int = 0;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_PAWS_INVALID_SHIFT: c_int = 24;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_TIMESTAMP_EXISTS_SHIFT: c_int = 25;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_RESERVED0_SHIFT: c_int = 26;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_STOP_RX_PAYLOAD_SHIFT: c_int = 27;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_KA_ENABLED_SHIFT: c_int = 28;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_FIRST_RTO_ESTIMATE_SHIFT: c_int = 29;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_MAX_SEG_RETRANSMIT_EN_SHIFT: c_int = 30;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_LAST_ISLE_HAS_FIN_SHIFT: c_int = 31;
    pub flags2: u32,

pub const TSTORM_TCP_ST_CONTEXT_SECTION_RTT_VARIATION_SHIFT: c_int = 0;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_DA_EN_SHIFT: c_int = 24;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_DA_COUNTER_EN_SHIFT: c_int = 25;

pub const __TSTORM_TCP_ST_CONTEXT_SECTION_KA_PROBE_SENT_SHIFT: c_int = 26;

pub const __TSTORM_TCP_ST_CONTEXT_SECTION_PERSIST_PROBE_SENT_SHIFT: c_int = 27;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_UPDATE_L2_STATSTICS_SHIFT: c_int = 28;

pub const TSTORM_TCP_ST_CONTEXT_SECTION_UPDATE_L4_STATSTICS_SHIFT: c_int = 29;

pub const __TSTORM_TCP_ST_CONTEXT_SECTION_IN_WINDOW_RST_ATTACK_SHIFT: c_int = 30;

pub const __TSTORM_TCP_ST_CONTEXT_SECTION_IN_WINDOW_SYN_ATTACK_SHIFT: c_int = 31;

    pub mss: u16,
    pub tcp_sm_state: u8,
    pub rto_exp: u8,

    pub rto_exp: u8,
    pub tcp_sm_state: u8,
    pub mss: u16,

    pub rcv_nxt: u32,
    pub timestamp_recent: u32,
    pub timestamp_recent_time: u32,
    pub cwnd: u32,
    pub ss_thresh: u32,
    pub cwnd_accum: u32,
    pub prev_seg_seq: u32,
    pub expected_rel_seq: u32,
    pub recover: u32,

    pub retransmit_count: u8,
    pub ka_max_probe_count: u8,
    pub persist_probe_count: u8,
    pub ka_probe_count: u8,

    pub ka_probe_count: u8,
    pub persist_probe_count: u8,
    pub ka_max_probe_count: u8,
    pub retransmit_count: u8,

    pub statistics_counter_id: u8,
    pub ooo_support_mode: u8,
    pub snd_wnd_scale: u8,
    pub dup_ack_count: u8,

    pub dup_ack_count: u8,
    pub snd_wnd_scale: u8,
    pub ooo_support_mode: u8,
    pub statistics_counter_id: u8,

    pub retransmit_start_time: u32,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub isle_start_seq: u32,
    pub isle_end_seq: u32,

    pub second_isle_address: u16,
    pub recent_seg_wnd: u16,

    pub recent_seg_wnd: u16,
    pub second_isle_address: u16,

    pub max_isles_ever_happened: u8,
    pub isles_number: u8,
    pub last_isle_address: u16,

    pub last_isle_address: u16,
    pub isles_number: u8,
    pub max_isles_ever_happened: u8,

    pub max_rt_time: u32,

    pub lsb_mac_address: u16,
    pub vlan_id: u16,

    pub vlan_id: u16,
    pub lsb_mac_address: u16,

    pub msb_mac_address: u16,
    pub mid_mac_address: u16,

    pub mid_mac_address: u16,
    pub msb_mac_address: u16,

    pub rightmost_received_seq: u32,
}

//
// Termination variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_term_vars {
    pub BitMap: u8,

pub const ISCSI_TERM_VARS_TCP_STATE_SHIFT: c_int = 0;

pub const ISCSI_TERM_VARS_FIN_RECEIVED_SBIT_SHIFT: c_int = 4;

pub const ISCSI_TERM_VARS_ACK_ON_FIN_RECEIVED_SBIT_SHIFT: c_int = 5;

pub const ISCSI_TERM_VARS_TERM_ON_CHIP_SHIFT: c_int = 6;

pub const ISCSI_TERM_VARS_RSRV_SHIFT: c_int = 7;
}

//
// iSCSI context region, used only in iSCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_st_context_section {
    pub nalPayload: u32,
    pub b2nh: u32,

    pub rq_cons: u16,
    pub flags: u8,

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_HDR_DIGEST_EN_SHIFT: c_int = 0;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_DATA_DIGEST_EN_SHIFT: c_int = 1;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_PARTIAL_HEADER_SHIFT: c_int = 2;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_FULL_FEATURE_SHIFT: c_int = 3;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_DROP_ALL_PDUS_SHIFT: c_int = 4;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_NALLEN_SHIFT: c_int = 5;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_RSRV0_SHIFT: c_int = 7;
    pub hdr_bytes_2_fetch: u8,

    pub hdr_bytes_2_fetch: u8,
    pub flags: u8,

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_HDR_DIGEST_EN_SHIFT: c_int = 0;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_DATA_DIGEST_EN_SHIFT: c_int = 1;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_PARTIAL_HEADER_SHIFT: c_int = 2;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_FULL_FEATURE_SHIFT: c_int = 3;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_B_DROP_ALL_PDUS_SHIFT: c_int = 4;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_NALLEN_SHIFT: c_int = 5;

pub const TSTORM_ISCSI_ST_CONTEXT_SECTION_RSRV0_SHIFT: c_int = 7;
    pub rq_cons: u16,

    pub rq_db_phy_addr: regpair,

    pub term_vars: iscsi_term_vars,
    pub rsrv1: u8,
    pub iscsi_conn_id: u16,

    pub iscsi_conn_id: u16,
    pub rsrv1: u8,
    pub term_vars: iscsi_term_vars,

    pub process_nxt: u32,
}

//
// The iSCSI non-aggregative context of Tstorm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_st_context {
    pub tcp: tstorm_tcp_st_context_section,
    pub iscsi: tstorm_iscsi_st_context_section,
}

//
// Ethernet context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_context_section {

    pub remote_addr_4: u8,
    pub remote_addr_5: u8,
    pub local_addr_0: u8,
    pub local_addr_1: u8,

    pub local_addr_1: u8,
    pub local_addr_0: u8,
    pub remote_addr_5: u8,
    pub remote_addr_4: u8,

    pub remote_addr_0: u8,
    pub remote_addr_1: u8,
    pub remote_addr_2: u8,
    pub remote_addr_3: u8,

    pub remote_addr_3: u8,
    pub remote_addr_2: u8,
    pub remote_addr_1: u8,
    pub remote_addr_0: u8,

    pub reserved_vlan_type: u16,
    pub vlan_params: u16,

pub const XSTORM_ETH_CONTEXT_SECTION_VLAN_ID_SHIFT: c_int = 0;

pub const XSTORM_ETH_CONTEXT_SECTION_CFI_SHIFT: c_int = 12;

pub const XSTORM_ETH_CONTEXT_SECTION_PRIORITY_SHIFT: c_int = 13;

    pub vlan_params: u16,

pub const XSTORM_ETH_CONTEXT_SECTION_VLAN_ID_SHIFT: c_int = 0;

pub const XSTORM_ETH_CONTEXT_SECTION_CFI_SHIFT: c_int = 12;

pub const XSTORM_ETH_CONTEXT_SECTION_PRIORITY_SHIFT: c_int = 13;
    pub reserved_vlan_type: u16,

    pub local_addr_2: u8,
    pub local_addr_3: u8,
    pub local_addr_4: u8,
    pub local_addr_5: u8,

    pub local_addr_5: u8,
    pub local_addr_4: u8,
    pub local_addr_3: u8,
    pub local_addr_2: u8,

}

//
// IpV4 context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_ip_v4_context_section {

    pub __pbf_hdr_cmd_rsvd_id: u16,
    pub __pbf_hdr_cmd_rsvd_flags_offset: u16,

    pub __pbf_hdr_cmd_rsvd_flags_offset: u16,
    pub __pbf_hdr_cmd_rsvd_id: u16,

    pub __pbf_hdr_cmd_rsvd_ver_ihl: u8,
    pub tos: u8,
    pub __pbf_hdr_cmd_rsvd_length: u16,

    pub __pbf_hdr_cmd_rsvd_length: u16,
    pub tos: u8,
    pub __pbf_hdr_cmd_rsvd_ver_ihl: u8,

    pub ip_local_addr: u32,

    pub ttl: u8,
    pub __pbf_hdr_cmd_rsvd_protocol: u8,
    pub __pbf_hdr_cmd_rsvd_csum: u16,

    pub __pbf_hdr_cmd_rsvd_csum: u16,
    pub __pbf_hdr_cmd_rsvd_protocol: u8,
    pub ttl: u8,

    pub __pbf_hdr_cmd_rsvd_1: u32,
    pub ip_remote_addr: u32,
}

//
// context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_padded_ip_v4_context_section {
    pub ip_v4: xstorm_ip_v4_context_section,
    pub reserved1: [u32; 4],
}

//
// IpV6 context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_ip_v6_context_section {

    pub pbf_hdr_cmd_rsvd_payload_len: u16,
    pub pbf_hdr_cmd_rsvd_nxt_hdr: u8,
    pub hop_limit: u8,

    pub hop_limit: u8,
    pub pbf_hdr_cmd_rsvd_nxt_hdr: u8,
    pub pbf_hdr_cmd_rsvd_payload_len: u16,

    pub priority_flow_label: u32,

pub const XSTORM_IP_V6_CONTEXT_SECTION_FLOW_LABEL_SHIFT: c_int = 0;

pub const XSTORM_IP_V6_CONTEXT_SECTION_TRAFFIC_CLASS_SHIFT: c_int = 20;

pub const XSTORM_IP_V6_CONTEXT_SECTION_PBF_HDR_CMD_RSVD_VER_SHIFT: c_int = 28;
    pub ip_local_addr_lo_hi: u32,
    pub ip_local_addr_lo_lo: u32,
    pub ip_local_addr_hi_hi: u32,
    pub ip_local_addr_hi_lo: u32,
    pub ip_remote_addr_lo_hi: u32,
    pub ip_remote_addr_lo_lo: u32,
    pub ip_remote_addr_hi_hi: u32,
    pub ip_remote_addr_hi_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xstorm_ip_context_section_types {
    pub padded_ip_v4: xstorm_padded_ip_v4_context_section,
    pub ip_v6: xstorm_ip_v6_context_section,
}

//
// TCP context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_tcp_context_section {
    pub snd_max: u32,

    pub remote_port: u16,
    pub local_port: u16,

    pub local_port: u16,
    pub remote_port: u16,

    pub original_nagle_1b: u8,
    pub ts_enabled: u8,
    pub tcp_params: u16,

pub const XSTORM_TCP_CONTEXT_SECTION_TOTAL_HEADER_SIZE_SHIFT: c_int = 0;

pub const __XSTORM_TCP_CONTEXT_SECTION_ECT_BIT_SHIFT: c_int = 8;

pub const __XSTORM_TCP_CONTEXT_SECTION_ECN_ENABLED_SHIFT: c_int = 9;

pub const XSTORM_TCP_CONTEXT_SECTION_SACK_ENABLED_SHIFT: c_int = 10;

pub const XSTORM_TCP_CONTEXT_SECTION_SMALL_WIN_ADV_SHIFT: c_int = 11;

pub const XSTORM_TCP_CONTEXT_SECTION_FIN_SENT_FLAG_SHIFT: c_int = 12;

pub const XSTORM_TCP_CONTEXT_SECTION_WINDOW_SATURATED_SHIFT: c_int = 13;

pub const XSTORM_TCP_CONTEXT_SECTION_SLOWPATH_QUEUES_FLUSH_COUNTER_SHIFT: c_int = 14;

    pub tcp_params: u16,

pub const XSTORM_TCP_CONTEXT_SECTION_TOTAL_HEADER_SIZE_SHIFT: c_int = 0;

pub const __XSTORM_TCP_CONTEXT_SECTION_ECT_BIT_SHIFT: c_int = 8;

pub const __XSTORM_TCP_CONTEXT_SECTION_ECN_ENABLED_SHIFT: c_int = 9;

pub const XSTORM_TCP_CONTEXT_SECTION_SACK_ENABLED_SHIFT: c_int = 10;

pub const XSTORM_TCP_CONTEXT_SECTION_SMALL_WIN_ADV_SHIFT: c_int = 11;

pub const XSTORM_TCP_CONTEXT_SECTION_FIN_SENT_FLAG_SHIFT: c_int = 12;

pub const XSTORM_TCP_CONTEXT_SECTION_WINDOW_SATURATED_SHIFT: c_int = 13;

pub const XSTORM_TCP_CONTEXT_SECTION_SLOWPATH_QUEUES_FLUSH_COUNTER_SHIFT: c_int = 14;
    pub ts_enabled: u8,
    pub original_nagle_1b: u8,

    pub pseudo_csum: u16,
    pub window_scaling_factor: u16,

    pub window_scaling_factor: u16,
    pub pseudo_csum: u16,

    pub reserved2: u16,
    pub statistics_counter_id: u8,
    pub statistics_params: u8,

pub const XSTORM_TCP_CONTEXT_SECTION_UPDATE_L2_STATSTICS_SHIFT: c_int = 0;

pub const XSTORM_TCP_CONTEXT_SECTION_UPDATE_L4_STATSTICS_SHIFT: c_int = 1;

pub const XSTORM_TCP_CONTEXT_SECTION_RESERVED_SHIFT: c_int = 2;

    pub statistics_params: u8,

pub const XSTORM_TCP_CONTEXT_SECTION_UPDATE_L2_STATSTICS_SHIFT: c_int = 0;

pub const XSTORM_TCP_CONTEXT_SECTION_UPDATE_L4_STATSTICS_SHIFT: c_int = 1;

pub const XSTORM_TCP_CONTEXT_SECTION_RESERVED_SHIFT: c_int = 2;
    pub statistics_counter_id: u8,
    pub reserved2: u16,

    pub ts_time_diff: u32,
    pub __next_timer_expir: u32,
}

//
// Common context section, shared in TOE, RDMA and ISCSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_common_context_section {
    pub ethernet: xstorm_eth_context_section,
    pub ip_union: xstorm_ip_context_section_types,
    pub tcp: xstorm_tcp_context_section,

    pub __dcb_val: u8,
    pub flags: u8,

pub const XSTORM_COMMON_CONTEXT_SECTION_PHYSQ_INITIALIZED_SHIFT: c_int = 0;

pub const XSTORM_COMMON_CONTEXT_SECTION_PBF_PORT_SHIFT: c_int = 1;

pub const XSTORM_COMMON_CONTEXT_SECTION_VLAN_MODE_SHIFT: c_int = 4;

pub const XSTORM_COMMON_CONTEXT_SECTION_ORIGINAL_PRIORITY_SHIFT: c_int = 5;
    pub reserved: u8,
    pub ip_version_1b: u8,

    pub ip_version_1b: u8,
    pub reserved: u8,
    pub flags: u8,

pub const XSTORM_COMMON_CONTEXT_SECTION_PHYSQ_INITIALIZED_SHIFT: c_int = 0;

pub const XSTORM_COMMON_CONTEXT_SECTION_PBF_PORT_SHIFT: c_int = 1;

pub const XSTORM_COMMON_CONTEXT_SECTION_VLAN_MODE_SHIFT: c_int = 4;

pub const XSTORM_COMMON_CONTEXT_SECTION_ORIGINAL_PRIORITY_SHIFT: c_int = 5;
    pub __dcb_val: u8,

}

//
// Flags used in ISCSI context section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_context_flags {
    pub flags: u8,

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_IMMEDIATE_DATA_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_INITIAL_R2T_SHIFT: c_int = 1;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_EN_HEADER_DIGEST_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_EN_DATA_DIGEST_SHIFT: c_int = 3;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_HQ_BD_WRITTEN_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_LAST_OP_SQ_SHIFT: c_int = 5;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_B_UPDATE_SND_NXT_SHIFT: c_int = 6;

pub const XSTORM_ISCSI_CONTEXT_FLAGS_RESERVED4_SHIFT: c_int = 7;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_x {
    pub data_out_buffer_offset: u32,
    pub itt: u32,
    pub data_sn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc_x_write_only {
    pub tx_r2t_sn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc_xu_write_both {
    pub sgl_base_lo: u32,
    pub sgl_base_hi: u32,

    pub sgl_size: u8,
    pub sge_index: u8,
    pub sge_offset: u16,

    pub sge_offset: u16,
    pub sge_index: u8,
    pub sgl_size: u8,

}

//
// iSCSI context section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_context_section {
    pub first_burst_length: u32,
    pub max_send_pdu_length: u32,
    pub sq_pbl_base: regpair,
    pub sq_curr_pbe: regpair,
    pub hq_pbl_base: regpair,
    pub hq_curr_pbe_base: regpair,
    pub r2tq_pbl_base: regpair,
    pub r2tq_curr_pbe_base: regpair,
    pub task_pbl_base: regpair,

    pub data_out_count: u16,
    pub flags: xstorm_iscsi_context_flags,
    pub task_pbl_cache_idx: u8,

    pub task_pbl_cache_idx: u8,
    pub flags: xstorm_iscsi_context_flags,
    pub data_out_count: u16,

    pub seq_more_2_send: u32,
    pub pdu_more_2_send: u32,
    pub temp_tce_x: iscsi_task_context_entry_x,
    pub temp_tce_x_wr: iscsi_task_context_entry_xuc_x_write_only,
    pub temp_tce_xu_wr: iscsi_task_context_entry_xuc_xu_write_both,
    pub lun: regpair,
    pub exp_data_transfer_len_ttt: u32,
    pub pdu_data_2_rxmit: u32,
    pub rxmit_bytes_2_dr: u32,

    pub rxmit_sge_offset: u16,
    pub hq_rxmit_cons: u16,

    pub hq_rxmit_cons: u16,
    pub rxmit_sge_offset: u16,

    pub r2tq_cons: u16,
    pub rxmit_flags: u8,

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_NEW_HQ_BD_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_PDU_HDR_SHIFT: c_int = 1;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_END_PDU_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_DR_SHIFT: c_int = 3;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_START_DR_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_PADDING_SHIFT: c_int = 5;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_ISCSI_CONT_FAST_RXMIT_SHIFT: c_int = 7;
    pub rxmit_sge_idx: u8,

    pub rxmit_sge_idx: u8,
    pub rxmit_flags: u8,

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_NEW_HQ_BD_SHIFT: c_int = 0;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_PDU_HDR_SHIFT: c_int = 1;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_END_PDU_SHIFT: c_int = 2;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_DR_SHIFT: c_int = 3;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_START_DR_SHIFT: c_int = 4;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_RXMIT_PADDING_SHIFT: c_int = 5;

pub const XSTORM_ISCSI_CONTEXT_SECTION_B_ISCSI_CONT_FAST_RXMIT_SHIFT: c_int = 7;
    pub r2tq_cons: u16,

    pub hq_rxmit_tcp_seq: u32,
}

//
// Xstorm iSCSI Storm Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_st_context {
    pub common: xstorm_common_context_section,
    pub iscsi: xstorm_iscsi_context_section,
}

//
// Iscsi connection context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_context {
    pub ustorm_st_context: ustorm_iscsi_st_context,
    pub tstorm_st_context: tstorm_iscsi_st_context,
    pub xstorm_ag_context: xstorm_iscsi_ag_context,
    pub tstorm_ag_context: tstorm_iscsi_ag_context,
    pub cstorm_ag_context: cstorm_iscsi_ag_context,
    pub ustorm_ag_context: ustorm_iscsi_ag_context,
    pub timers_context: timers_block_context,
    pub upb_context: regpair,
    pub xstorm_st_context: xstorm_iscsi_st_context,
    pub xpb_context: regpair,
    pub cstorm_st_context: cstorm_iscsi_st_context,
}

//
// PDU header of an iSCSI DATA-OUT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_data_pdu_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_FINAL_FLAG_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_FINAL_FLAG_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_DATA_PDU_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: u32,
    pub ttt: u32,
    pub rsrv2: u32,
    pub exp_stat_sn: u32,
    pub rsrv3: u32,
    pub data_sn: u32,
    pub buffer_offset: u32,
    pub rsrv4: u32,
}

//
// PDU header of an iSCSI login request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_req_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_NSG_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_CSG_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_RSRV0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_CONTINUE_FLG_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_TRANSIT_SHIFT: c_int = 7;
    pub version_max: u8,
    pub version_min: u8,

    pub version_min: u8,
    pub version_max: u8,
    pub op_attr: u8,

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_NSG_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_CSG_SHIFT: c_int = 2;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_RSRV0_SHIFT: c_int = 4;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_CONTINUE_FLG_SHIFT: c_int = 6;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_TRANSIT_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_LOGIN_REQ_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub isid_lo: u32,

    pub isid_hi: u16,
    pub tsih: u16,

    pub tsih: u16,
    pub isid_hi: u16,

    pub itt: u32,

    pub cid: u16,
    pub rsrv1: u16,

    pub rsrv1: u16,
    pub cid: u16,

    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub rsrv2: [u32; 4],
}

//
// PDU header of an iSCSI logout request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout_req_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_REASON_CODE_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_RSRV1_1_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_REASON_CODE_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_RSRV1_1_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_LOGOUT_REQ_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub rsrv2: [u32; 2],
    pub itt: u32,

    pub cid: u16,
    pub rsrv1: u16,

    pub rsrv1: u16,
    pub cid: u16,

    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub rsrv3: [u32; 4],
}

//
// PDU header of an iSCSI TMF request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tmf_req_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_FUNCTION_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_RSRV1_1_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_FUNCTION_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_RSRV1_1_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_TMF_REQ_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: u32,
    pub referenced_task_tag: u32,
    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub ref_cmd_sn: u32,
    pub exp_data_sn: u32,
    pub rsrv2: [u32; 2],
}

//
// PDU header of an iSCSI Text request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_text_req_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_CONTINUE_FLG_SHIFT: c_int = 6;

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_FINAL_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_CONTINUE_FLG_SHIFT: c_int = 6;

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_FINAL_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_TEXT_REQ_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: u32,
    pub ttt: u32,
    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub rsrv3: [u32; 4],
}

//
// PDU header of an iSCSI Nop-Out
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_nop_out_hdr_little_endian {

    pub opcode: u8,
    pub op_attr: u8,

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_RSRV2_1_SHIFT: c_int = 7;
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub op_attr: u8,

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_RSRV1_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_RSRV2_1_SHIFT: c_int = 7;
    pub opcode: u8,

    pub data_fields: u32,

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_DATA_SEGMENT_LENGTH_SHIFT: c_int = 0;

pub const ISCSI_NOP_OUT_HDR_LITTLE_ENDIAN_TOTAL_AHS_LENGTH_SHIFT: c_int = 24;
    pub lun: regpair,
    pub itt: u32,
    pub ttt: u32,
    pub cmd_sn: u32,
    pub exp_stat_sn: u32,
    pub rsrv3: [u32; 4],
}

//
// iscsi pdu headers in little endian form.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iscsi_pdu_headers_little_endian {
    pub fullHeaderSize: [u32; 12],
    pub command_pdu_hdr: iscsi_cmd_pdu_hdr_little_endian,
    pub data_out_pdu_hdr: iscsi_data_pdu_hdr_little_endian,
    pub login_req_pdu_hdr: iscsi_login_req_hdr_little_endian,
    pub logout_req_pdu_hdr: iscsi_logout_req_hdr_little_endian,
    pub tmf_req_pdu_hdr: iscsi_tmf_req_hdr_little_endian,
    pub text_req_pdu_hdr: iscsi_text_req_hdr_little_endian,
    pub nop_out_pdu_hdr: iscsi_nop_out_hdr_little_endian,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_hq_bd {
    pub pdu_header: iscsi_pdu_headers_little_endian,

    pub reserved1: u16,
    pub lcl_cmp_flg: u16,

    pub lcl_cmp_flg: u16,
    pub reserved1: u16,

    pub sgl_base_lo: u32,
    pub sgl_base_hi: u32,

    pub sgl_size: u8,
    pub sge_index: u8,
    pub sge_offset: u16,

    pub sge_offset: u16,
    pub sge_index: u8,
    pub sgl_size: u8,

}

//
// CQE data for L2 OOO connection $$KEEP_ENDIANNESS$$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_l2_ooo_data {
    pub iscsi_cid: __le32,
    pub drop_isle: u8,
    pub drop_size: u8,
    pub ooo_opcode: u8,
    pub ooo_isle: u8,
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc_c_write_only {
    pub total_data_acked: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_r2t_table_entry {
    pub ttt: u32,
    pub desired_data_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc_u_write_only {
    pub exp_r2t_sn: u32,
    pub r2t_table: [iscsi_task_context_r2t_table_entry; 4],
    pub data_in_count: u16,
    pub cq_id: u8,
    pub valid_1b: u8,

    pub valid_1b: u8,
    pub cq_id: u8,
    pub data_in_count: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc {
    pub write_c: iscsi_task_context_entry_xuc_c_write_only,
    pub exp_data_transfer_len: u32,
    pub write_x: iscsi_task_context_entry_xuc_x_write_only,
    pub lun_lo: u32,
    pub write_xu: iscsi_task_context_entry_xuc_xu_write_both,
    pub lun_hi: u32,
    pub write_u: iscsi_task_context_entry_xuc_u_write_only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_u {
    pub exp_r2t_buff_offset: u32,
    pub rem_rcv_len: u32,
    pub exp_data_sn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry {
    pub tce_x: iscsi_task_context_entry_x,

    pub data_out_count: u16,
    pub rsrv0: u16,

    pub rsrv0: u16,
    pub data_out_count: u16,

    pub tce_xuc: iscsi_task_context_entry_xuc,
    pub tce_u: iscsi_task_context_entry_u,
    pub rsrv1: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_context_entry_xuc_x_init_only {
    pub lun: regpair,
    pub exp_data_transfer_len: u32,
}

//
// ipv6 structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_v6_addr {
    pub ip_addr_lo_lo: u32,
    pub ip_addr_lo_hi: u32,
    pub ip_addr_hi_lo: u32,
    pub ip_addr_hi_hi: u32,
}

//
// l5cm- connection identification params
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_conn_addr_params {
    pub pmtu: u32,

    pub remote_addr_3: u8,
    pub remote_addr_2: u8,
    pub remote_addr_1: u8,
    pub remote_addr_0: u8,

    pub remote_addr_0: u8,
    pub remote_addr_1: u8,
    pub remote_addr_2: u8,
    pub remote_addr_3: u8,

    pub params: u16,

pub const L5CM_CONN_ADDR_PARAMS_IP_VERSION_SHIFT: c_int = 0;

pub const L5CM_CONN_ADDR_PARAMS_RSRV_SHIFT: c_int = 1;
    pub remote_addr_5: u8,
    pub remote_addr_4: u8,

    pub remote_addr_4: u8,
    pub remote_addr_5: u8,
    pub params: u16,

pub const L5CM_CONN_ADDR_PARAMS_IP_VERSION_SHIFT: c_int = 0;

pub const L5CM_CONN_ADDR_PARAMS_RSRV_SHIFT: c_int = 1;

    pub local_ip_addr: ip_v6_addr,
    pub remote_ip_addr: ip_v6_addr,
    pub ipv6_flow_label_20b: u32,
    pub reserved1: u32,

    pub remote_tcp_port: u16,
    pub local_tcp_port: u16,

    pub local_tcp_port: u16,
    pub remote_tcp_port: u16,

}

//
// l5cm-xstorm connection buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_xstorm_conn_buffer {

    pub rsrv1: u16,
    pub params: u16,

pub const L5CM_XSTORM_CONN_BUFFER_NAGLE_ENABLE_SHIFT: c_int = 0;

pub const L5CM_XSTORM_CONN_BUFFER_RSRV_SHIFT: c_int = 1;

    pub params: u16,

pub const L5CM_XSTORM_CONN_BUFFER_NAGLE_ENABLE_SHIFT: c_int = 0;

pub const L5CM_XSTORM_CONN_BUFFER_RSRV_SHIFT: c_int = 1;
    pub rsrv1: u16,

    pub mss: u16,
    pub pseudo_header_checksum: u16,

    pub pseudo_header_checksum: u16,
    pub mss: u16,

    pub rcv_buf: u32,
    pub rsrv2: u32,
    pub context_addr: regpair,
}

//
// l5cm-tstorm connection buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_tstorm_conn_buffer {
    pub rsrv1: [u32; 2],
    pub params: u16,

pub const L5CM_TSTORM_CONN_BUFFER_DELAYED_ACK_ENABLE_SHIFT: c_int = 0;

pub const L5CM_TSTORM_CONN_BUFFER_RSRV_SHIFT: c_int = 1;
    pub ka_max_probe_count: u8,
    pub ka_enable: u8,

    pub ka_enable: u8,
    pub ka_max_probe_count: u8,
    pub params: u16,

pub const L5CM_TSTORM_CONN_BUFFER_DELAYED_ACK_ENABLE_SHIFT: c_int = 0;

pub const L5CM_TSTORM_CONN_BUFFER_RSRV_SHIFT: c_int = 1;

    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
}

//
// l5cm connection buffer for active side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_active_conn_buffer {
    pub conn_addr_buf: l5cm_conn_addr_params,
    pub xstorm_conn_buffer: l5cm_xstorm_conn_buffer,
    pub tstorm_conn_buffer: l5cm_tstorm_conn_buffer,
}

//
// The l5cm opaque buffer passed in add new connection ramrod passive side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_hash_input_string {
    pub __opaque1: u32,

    pub __opaque3: u16,
    pub __opaque2: u16,

    pub __opaque2: u16,
    pub __opaque3: u16,

    pub __opaque4: ip_v6_addr,
    pub __opaque5: ip_v6_addr,
    pub __opaque6: u32,
    pub __opaque7: [u32; 5],
}

//
// syn cookie component
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_syn_cookie_comp {
    pub __opaque: u32,
}

//
// data related to listeners of a TCP port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_port_listener_data {
    pub params: u8,

pub const L5CM_PORT_LISTENER_DATA_ENABLE_SHIFT: c_int = 0;

pub const L5CM_PORT_LISTENER_DATA_IP_INDEX_SHIFT: c_int = 1;

pub const L5CM_PORT_LISTENER_DATA_NET_FILTER_SHIFT: c_int = 5;

pub const L5CM_PORT_LISTENER_DATA_DEFFERED_MODE_SHIFT: c_int = 6;

pub const L5CM_PORT_LISTENER_DATA_MPA_MODE_SHIFT: c_int = 7;
}

//
// Opaque structure passed from U to X when final ack arrives
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_opaque_buf {
    pub __opaque1: u32,
    pub __opaque2: u32,
    pub __opaque3: u32,
    pub __opaque4: u32,
    pub __opaque5: l5cm_syn_cookie_comp,

    pub rsrv2: u16,
    pub rsrv: u8,
    pub __opaque6: l5cm_port_listener_data,

    pub __opaque6: l5cm_port_listener_data,
    pub rsrv: u8,
    pub rsrv2: u16,

}

//
// l5cm slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_packet_size {
    pub size: u32,
    pub rsrv: u32,
}

//
// The final-ack union structure in PCS entry after final ack arrived
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_pcse_ack {
    pub tx_socket_params: l5cm_xstorm_conn_buffer,
    pub opaque_buf: l5cm_opaque_buf,
    pub rx_socket_params: l5cm_tstorm_conn_buffer,
}

//
// The syn union structure in PCS entry after syn arrived
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_pcse_syn {
    pub opaque_buf: l5cm_opaque_buf,
    pub rsrv: [u32; 12],
}

//
// pcs entry data for passive connections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_pcs_attributes {

    pub pcs_id: u16,
    pub status: u8,
    pub flags: u8,

pub const L5CM_PCS_ATTRIBUTES_NET_FILTER_SHIFT: c_int = 0;

pub const L5CM_PCS_ATTRIBUTES_CALCULATE_HASH_SHIFT: c_int = 1;

pub const L5CM_PCS_ATTRIBUTES_COMPARE_HASH_RESULT_SHIFT: c_int = 2;

pub const L5CM_PCS_ATTRIBUTES_QUERY_ULP_ACCEPT_SHIFT: c_int = 3;

pub const L5CM_PCS_ATTRIBUTES_FIND_DEST_MAC_SHIFT: c_int = 4;

pub const L5CM_PCS_ATTRIBUTES_L4_OFFLOAD_SHIFT: c_int = 5;

pub const L5CM_PCS_ATTRIBUTES_FORWARD_PACKET_SHIFT: c_int = 6;

pub const L5CM_PCS_ATTRIBUTES_RSRV_SHIFT: c_int = 7;

    pub flags: u8,

pub const L5CM_PCS_ATTRIBUTES_NET_FILTER_SHIFT: c_int = 0;

pub const L5CM_PCS_ATTRIBUTES_CALCULATE_HASH_SHIFT: c_int = 1;

pub const L5CM_PCS_ATTRIBUTES_COMPARE_HASH_RESULT_SHIFT: c_int = 2;

pub const L5CM_PCS_ATTRIBUTES_QUERY_ULP_ACCEPT_SHIFT: c_int = 3;

pub const L5CM_PCS_ATTRIBUTES_FIND_DEST_MAC_SHIFT: c_int = 4;

pub const L5CM_PCS_ATTRIBUTES_L4_OFFLOAD_SHIFT: c_int = 5;

pub const L5CM_PCS_ATTRIBUTES_FORWARD_PACKET_SHIFT: c_int = 6;

pub const L5CM_PCS_ATTRIBUTES_RSRV_SHIFT: c_int = 7;
    pub status: u8,
    pub pcs_id: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union l5cm_seg_params {
    pub syn_seg_params: l5cm_pcse_syn,
    pub ack_seg_params: l5cm_pcse_ack,
}

//
// pcs entry data for passive connections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_pcs_hdr {
    pub hash_input_string: l5cm_hash_input_string,
    pub conn_addr_buf: l5cm_conn_addr_params,
    pub cid: u32,
    pub hash_result: u32,
    pub seg_params: l5cm_seg_params,
    pub att: l5cm_pcs_attributes,

    pub rsrv: u16,
    pub rx_seg_size: u16,

    pub rx_seg_size: u16,
    pub rsrv: u16,

}

//
// pcs entry for passive connections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_pcs_entry {
    pub hdr: l5cm_pcs_hdr,
    pub rx_segment: [u8; 1516],
}

//
// l5cm connection parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union l5cm_reduce_param_union {
    pub opaque1: u32,
    pub opaque2: u32,
}

//
// l5cm connection parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_reduce_conn {
    pub opaque1: l5cm_reduce_param_union,
    pub opaque2: u32,
}

//
// l5cm slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union l5cm_specific_data {
    pub protocol_data: [u8; 8],
    pub phy_address: regpair,
    pub packet_size: l5cm_packet_size,
    pub reduced_conn: l5cm_reduce_conn,
}

//
// l5 slow path element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_spe {
    pub hdr: spe_hdr,
    pub data: l5cm_specific_data,
}

//
// Termination variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l5cm_term_vars {
    pub BitMap: u8,

pub const L5CM_TERM_VARS_TCP_STATE_SHIFT: c_int = 0;

pub const L5CM_TERM_VARS_FIN_RECEIVED_SBIT_SHIFT: c_int = 4;

pub const L5CM_TERM_VARS_ACK_ON_FIN_RECEIVED_SBIT_SHIFT: c_int = 5;

pub const L5CM_TERM_VARS_TERM_ON_CHIP_SHIFT: c_int = 6;

pub const L5CM_TERM_VARS_RSRV_SHIFT: c_int = 7;
}

//
// Tstorm Tcp flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_l5cm_tcp_flags {
    pub flags: u16,

pub const TSTORM_L5CM_TCP_FLAGS_VLAN_ID_SHIFT: c_int = 0;

pub const TSTORM_L5CM_TCP_FLAGS_DELAYED_ACK_SHIFT: c_int = 12;

pub const TSTORM_L5CM_TCP_FLAGS_TS_ENABLED_SHIFT: c_int = 13;

pub const TSTORM_L5CM_TCP_FLAGS_RSRV1_SHIFT: c_int = 14;
}

//
// Xstorm Tcp flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_l5cm_tcp_flags {
    pub flags: u8,

pub const XSTORM_L5CM_TCP_FLAGS_ENC_ENABLED_SHIFT: c_int = 0;

pub const XSTORM_L5CM_TCP_FLAGS_TS_ENABLED_SHIFT: c_int = 1;

pub const XSTORM_L5CM_TCP_FLAGS_WND_SCL_EN_SHIFT: c_int = 2;

pub const XSTORM_L5CM_TCP_FLAGS_RSRV_SHIFT: c_int = 3;
}

//
// Out-of-order states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ooo_event {
    TCP_EVENT_ADD_PEN = 0,
    TCP_EVENT_ADD_NEW_ISLE = 1,
    TCP_EVENT_ADD_ISLE_RIGHT = 2,
    TCP_EVENT_ADD_ISLE_LEFT = 3,
    TCP_EVENT_JOIN = 4,
    TCP_EVENT_NOP = 5,
    MAX_TCP_OOO_EVENT
}

//
// OOO support modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_tstorm_ooo {
    TCP_TSTORM_OOO_DROP_AND_PROC_ACK = 0,
    TCP_TSTORM_OOO_SEND_PURE_ACK = 1,
    TCP_TSTORM_OOO_SUPPORTED = 2,
    MAX_TCP_TSTORM_OOO
}
