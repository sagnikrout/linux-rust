//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/t3_cpl.h
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
// Copyright (c) 2004-2008 Chelsio, Inc. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CPL_opcode {
    CPL_PASS_OPEN_REQ = 0x1,
    CPL_PASS_ACCEPT_RPL = 0x2,
    CPL_ACT_OPEN_REQ = 0x3,
    CPL_SET_TCB = 0x4,
    CPL_SET_TCB_FIELD = 0x5,
    CPL_GET_TCB = 0x6,
    CPL_PCMD = 0x7,
    CPL_CLOSE_CON_REQ = 0x8,
    CPL_CLOSE_LISTSRV_REQ = 0x9,
    CPL_ABORT_REQ = 0xA,
    CPL_ABORT_RPL = 0xB,
    CPL_TX_DATA = 0xC,
    CPL_RX_DATA_ACK = 0xD,
    CPL_TX_PKT = 0xE,
    CPL_RTE_DELETE_REQ = 0xF,
    CPL_RTE_WRITE_REQ = 0x10,
    CPL_RTE_READ_REQ = 0x11,
    CPL_L2T_WRITE_REQ = 0x12,
    CPL_L2T_READ_REQ = 0x13,
    CPL_SMT_WRITE_REQ = 0x14,
    CPL_SMT_READ_REQ = 0x15,
    CPL_TX_PKT_LSO = 0x16,
    CPL_PCMD_READ = 0x17,
    CPL_BARRIER = 0x18,
    CPL_TID_RELEASE = 0x1A,

    CPL_CLOSE_LISTSRV_RPL = 0x20,
    CPL_ERROR = 0x21,
    CPL_GET_TCB_RPL = 0x22,
    CPL_L2T_WRITE_RPL = 0x23,
    CPL_PCMD_READ_RPL = 0x24,
    CPL_PCMD_RPL = 0x25,
    CPL_PEER_CLOSE = 0x26,
    CPL_RTE_DELETE_RPL = 0x27,
    CPL_RTE_WRITE_RPL = 0x28,
    CPL_RX_DDP_COMPLETE = 0x29,
    CPL_RX_PHYS_ADDR = 0x2A,
    CPL_RX_PKT = 0x2B,
    CPL_RX_URG_NOTIFY = 0x2C,
    CPL_SET_TCB_RPL = 0x2D,
    CPL_SMT_WRITE_RPL = 0x2E,
    CPL_TX_DATA_ACK = 0x2F,

    CPL_ABORT_REQ_RSS = 0x30,
    CPL_ABORT_RPL_RSS = 0x31,
    CPL_CLOSE_CON_RPL = 0x32,
    CPL_ISCSI_HDR = 0x33,
    CPL_L2T_READ_RPL = 0x34,
    CPL_RDMA_CQE = 0x35,
    CPL_RDMA_CQE_READ_RSP = 0x36,
    CPL_RDMA_CQE_ERR = 0x37,
    CPL_RTE_READ_RPL = 0x38,
    CPL_RX_DATA = 0x39,

    CPL_ACT_OPEN_RPL = 0x40,
    CPL_PASS_OPEN_RPL = 0x41,
    CPL_RX_DATA_DDP = 0x42,
    CPL_SMT_READ_RPL = 0x43,

    CPL_ACT_ESTABLISH = 0x50,
    CPL_PASS_ESTABLISH = 0x51,

    CPL_PASS_ACCEPT_REQ = 0x70,

    CPL_ASYNC_NOTIF = 0x80,	/* fake opcode for async notifications */

    CPL_TX_DMA_ACK = 0xA0,
    CPL_RDMA_READ_REQ = 0xA1,
    CPL_RDMA_TERMINATE = 0xA2,
    CPL_TRACE_PKT = 0xA3,
    CPL_RDMA_EC_STATUS = 0xA5,

    NUM_CPL_CMDS		/* must be last and previous entries must be sorted */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CPL_error {
    CPL_ERR_NONE = 0,
    CPL_ERR_TCAM_PARITY = 1,
    CPL_ERR_TCAM_FULL = 3,
    CPL_ERR_CONN_RESET = 20,
    CPL_ERR_CONN_EXIST = 22,
    CPL_ERR_ARP_MISS = 23,
    CPL_ERR_BAD_SYN = 24,
    CPL_ERR_CONN_TIMEDOUT = 30,
    CPL_ERR_XMIT_TIMEDOUT = 31,
    CPL_ERR_PERSIST_TIMEDOUT = 32,
    CPL_ERR_FINWAIT2_TIMEDOUT = 33,
    CPL_ERR_KEEPALIVE_TIMEDOUT = 34,
    CPL_ERR_RTX_NEG_ADVICE = 35,
    CPL_ERR_PERSIST_NEG_ADVICE = 36,
    CPL_ERR_ABORT_FAILED = 42,
    CPL_ERR_GENERAL = 99
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union opcode_tid {
    pub opcode_tid: __be32,
    pub opcode: __u8,
}

pub const S_OPCODE: c_int = 24;

pub const S_QNUM: c_int = 0;

pub const S_HASHTYPE: c_int = 22;
pub const M_HASHTYPE: c_uint = 0x3;

// tid is assumed to be 24-bits

// extract the TID from a CPL command

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_options {
    pub mss: __be16,
    pub wsf: __u8,

    pub ecn:1: __u8,
    pub sack:1: __u8,
    pub tstamp:1: __u8,

    pub tstamp:1: __u8,
    pub sack:1: __u8,
    pub ecn:1: __u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_header {
    pub opcode: __u8,

    pub cpu_idx:6: __u8,
    pub hash_type:2: __u8,

    pub hash_type:2: __u8,
    pub cpu_idx:6: __u8,

    pub cq_idx: __be16,
    pub rss_hash_val: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_request_hdr {
    pub wr_hi: __be32,
    pub wr_lo: __be32,
}

// wr_hi fields
pub const S_WR_SGE_CREDITS: c_int = 0;
pub const M_WR_SGE_CREDITS: c_uint = 0xFF;

pub const S_WR_SGLSFLT: c_int = 8;
pub const M_WR_SGLSFLT: c_uint = 0xFF;

pub const S_WR_BCNTLFLT: c_int = 16;
pub const M_WR_BCNTLFLT: c_uint = 0xF;

pub const S_WR_DATATYPE: c_int = 20;

pub const S_WR_COMPL: c_int = 21;

pub const S_WR_EOP: c_int = 22;

pub const S_WR_SOP: c_int = 23;

pub const S_WR_OP: c_int = 24;
pub const M_WR_OP: c_uint = 0xFF;

// wr_lo fields
pub const S_WR_LEN: c_int = 0;
pub const M_WR_LEN: c_uint = 0xFF;

pub const S_WR_TID: c_int = 8;
pub const M_WR_TID: c_uint = 0xFFFFF;

pub const S_WR_CR_FLUSH: c_int = 30;

pub const S_WR_GEN: c_int = 31;

// option 0 lower-half fields
pub const S_CPL_STATUS: c_int = 0;
pub const M_CPL_STATUS: c_uint = 0xFF;

pub const S_INJECT_TIMER: c_int = 6;

pub const S_NO_OFFLOAD: c_int = 7;

pub const S_ULP_MODE: c_int = 8;
pub const M_ULP_MODE: c_uint = 0xF;

pub const S_RCV_BUFSIZ: c_int = 12;
pub const M_RCV_BUFSIZ: c_uint = 0x3FFF;

pub const S_TOS: c_int = 26;
pub const M_TOS: c_uint = 0x3F;

// option 0 upper-half fields
pub const S_DELACK: c_int = 0;

pub const S_NO_CONG: c_int = 1;

pub const S_SRC_MAC_SEL: c_int = 2;
pub const M_SRC_MAC_SEL: c_uint = 0x3;

pub const S_L2T_IDX: c_int = 4;
pub const M_L2T_IDX: c_uint = 0x7FF;

pub const S_TX_CHANNEL: c_int = 15;

pub const S_TCAM_BYPASS: c_int = 16;

pub const S_NAGLE: c_int = 17;

pub const S_WND_SCALE: c_int = 18;
pub const M_WND_SCALE: c_uint = 0xF;

pub const S_KEEP_ALIVE: c_int = 22;

pub const S_MAX_RETRANS: c_int = 23;
pub const M_MAX_RETRANS: c_uint = 0xF;

pub const S_MAX_RETRANS_OVERRIDE: c_int = 27;

pub const S_MSS_IDX: c_int = 28;
pub const M_MSS_IDX: c_uint = 0xF;

// option 1 fields
pub const S_RSS_ENABLE: c_int = 0;

pub const S_RSS_MASK_LEN: c_int = 1;
pub const M_RSS_MASK_LEN: c_uint = 0x7;

pub const S_CPU_IDX: c_int = 4;
pub const M_CPU_IDX: c_uint = 0x3F;

pub const S_MAC_MATCH_VALID: c_int = 18;

pub const S_CONN_POLICY: c_int = 19;
pub const M_CONN_POLICY: c_uint = 0x3;

pub const S_SYN_DEFENSE: c_int = 21;

pub const S_VLAN_PRI: c_int = 22;
pub const M_VLAN_PRI: c_uint = 0x3;

pub const S_VLAN_PRI_VALID: c_int = 24;

pub const S_PKT_TYPE: c_int = 25;
pub const M_PKT_TYPE: c_uint = 0x3;

pub const S_MAC_MATCH: c_int = 27;
pub const M_MAC_MATCH: c_uint = 0x1F;

// option 2 fields
pub const S_CPU_INDEX: c_int = 0;
pub const M_CPU_INDEX: c_uint = 0x7F;

pub const S_CPU_INDEX_VALID: c_int = 7;

pub const S_RX_COALESCE: c_int = 8;
pub const M_RX_COALESCE: c_uint = 0x3;

pub const S_RX_COALESCE_VALID: c_int = 10;

pub const S_CONG_CONTROL_FLAVOR: c_int = 11;
pub const M_CONG_CONTROL_FLAVOR: c_uint = 0x3;

pub const S_PACING_FLAVOR: c_int = 13;
pub const M_PACING_FLAVOR: c_uint = 0x3;

pub const S_FLAVORS_VALID: c_int = 15;

pub const S_RX_FC_DISABLE: c_int = 16;

pub const S_RX_FC_VALID: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0h: __be32,
    pub opt0l: __be32,
    pub peer_netmask: __be32,
    pub opt1: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub resvd: [__u8; 7],
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_establish {
    pub ot: RSS_HDR union opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub tos_tid: __be32,
    pub l2t_idx: __be16,
    pub tcp_opt: __be16,
    pub snd_isn: __be32,
    pub rcv_isn: __be32,
}

// cpl_pass_establish.tos_tid fields
pub const S_PASS_OPEN_TID: c_int = 0;
pub const M_PASS_OPEN_TID: c_uint = 0xFFFFFF;

pub const S_PASS_OPEN_TOS: c_int = 24;
pub const M_PASS_OPEN_TOS: c_uint = 0xFF;

// cpl_pass_establish.l2t_idx fields
pub const S_L2T_IDX16: c_int = 5;
pub const M_L2T_IDX16: c_uint = 0x7FF;

// cpl_pass_establish.tcp_opt fields (also applies act_open_establish)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_req {
    pub ot: RSS_HDR union opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub tos_tid: __be32,
    pub tcp_options: tcp_options,
    pub dst_mac: [__u8; 6],
    pub vlan_tag: __be16,
    pub src_mac: [__u8; 6],
    pub addr_idx:3: __u8,
    pub port_idx:1: __u8,
    pub exact_match:1: __u8,

    pub exact_match:1: __u8,
    pub port_idx:1: __u8,
    pub addr_idx:3: __u8,

    pub rsvd: __u8,
    pub rcv_isn: __be32,
    pub rsvd2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_rpl {
    pub ot: opcode_tid,
    pub opt2: __be32,
    pub rsvd: __be32,
    pub peer_ip: __be32,
    pub opt0h: __be32,
    pub opt0l_status: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0h: __be32,
    pub opt0l: __be32,
    pub params: __be32,
    pub opt2: __be32,
}

// cpl_act_open_req.params fields
pub const S_AOPEN_VLAN_PRI: c_int = 9;
pub const M_AOPEN_VLAN_PRI: c_uint = 0x3;

pub const S_AOPEN_VLAN_PRI_VALID: c_int = 11;

pub const S_AOPEN_PKT_TYPE: c_int = 12;
pub const M_AOPEN_PKT_TYPE: c_uint = 0x3;

pub const S_AOPEN_MAC_MATCH: c_int = 14;
pub const M_AOPEN_MAC_MATCH: c_uint = 0x1F;

pub const S_AOPEN_MAC_MATCH_VALID: c_int = 19;

pub const S_AOPEN_IFF_VLAN: c_int = 20;
pub const M_AOPEN_IFF_VLAN: c_uint = 0xFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub atid: __be32,
    pub rsvd: [__u8; 3],
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_establish {
    pub ot: RSS_HDR union opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub tos_tid: __be32,
    pub l2t_idx: __be16,
    pub tcp_opt: __be16,
    pub snd_isn: __be32,
    pub rcv_isn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb {
    pub ot: opcode_tid,
    pub cpuno: __be16,
    pub rsvd: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: __u8,
    pub status: __u8,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb {
    pub ot: opcode_tid,
    pub reply: __u8,
    pub cpu_idx: __u8,
    pub len: __be16,
}

// cpl_set_tcb.reply fields
pub const S_NO_REPLY: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_field {
    pub ot: opcode_tid,
    pub reply: __u8,
    pub cpu_idx: __u8,
    pub word: __be16,
    pub mask: __be64,
    pub val: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: [__u8; 3],
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pcmd {
    pub ot: opcode_tid,
    pub rsvd: [__u8; 3],
    pub src:1: __u8,
    pub bundle:1: __u8,
    pub channel:1: __u8,

    pub channel:1: __u8,
    pub bundle:1: __u8,
    pub src:1: __u8,
    pub pcmd_parm: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pcmd_reply {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd: __u8,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_con_req {
    pub ot: opcode_tid,
    pub rsvd: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_con_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: [__u8; 3],
    pub status: __u8,
    pub snd_nxt: __be32,
    pub rcv_nxt: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listserv_req {
    pub ot: opcode_tid,
    pub rsvd0: __u8,
    pub cpu_idx: __u8,
    pub rsvd1: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listserv_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: [__u8; 3],
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req_rss {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: __u8,
    pub status: __u8,
    pub rsvd2: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req {
    pub ot: opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: __u8,
    pub cmd: __u8,
    pub rsvd2: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl_rss {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: __u8,
    pub status: __u8,
    pub rsvd2: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl {
    pub ot: opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: __u8,
    pub cmd: __u8,
    pub rsvd2: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_peer_close {
    pub ot: RSS_HDR union opcode_tid,
    pub rcv_nxt: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_data_wr {
    pub wr_hi: __be32,
    pub wr_lo: __be32,
    pub len: __be32,
    pub flags: __be32,
    pub sndseq: __be32,
    pub param: __be32,
}

// tx_data_wr.flags fields
pub const S_TX_ACK_PAGES: c_int = 21;
pub const M_TX_ACK_PAGES: c_uint = 0x7;

// tx_data_wr.param fields
pub const S_TX_PORT: c_int = 0;
pub const M_TX_PORT: c_uint = 0x7;

pub const S_TX_MSS: c_int = 4;
pub const M_TX_MSS: c_uint = 0xF;

pub const S_TX_QOS: c_int = 8;
pub const M_TX_QOS: c_uint = 0xFF;

pub const S_TX_SNDBUF: c_int = 16;
pub const M_TX_SNDBUF: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data {
    pub ot: opcode_tid,
    pub len: __be32,
    pub rsvd: __be32,
    pub urg: __be16,
    pub flags: __be16,
}

// cpl_tx_data.flags fields
pub const S_TX_ULP_SUBMODE: c_int = 6;
pub const M_TX_ULP_SUBMODE: c_uint = 0xF;

pub const S_TX_ULP_MODE: c_int = 10;
pub const M_TX_ULP_MODE: c_uint = 0xF;

pub const S_TX_SHOVE: c_int = 14;

pub const S_TX_MORE: c_int = 15;

// additional tx_data_wr.flags fields
pub const S_TX_CPU_IDX: c_int = 0;
pub const M_TX_CPU_IDX: c_uint = 0x3F;

pub const S_TX_URG: c_int = 16;

pub const S_TX_CLOSE: c_int = 17;

pub const S_TX_INIT: c_int = 18;

pub const S_TX_IMM_ACK: c_int = 19;

pub const S_TX_IMM_DMA: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data_ack {
    pub ot: RSS_HDR union opcode_tid,
    pub ack_seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_wr_ack {
    pub ot: RSS_HDR union opcode_tid,
    pub credits: __be16,
    pub rsvd: __be16,
    pub snd_nxt: __be32,
    pub snd_una: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rdma_ec_status {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: [__u8; 3],
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mngt_pktsched_wr {
    pub wr_hi: __be32,
    pub wr_lo: __be32,
    pub mngt_opcode: __u8,
    pub rsvd: [__u8; 7],
    pub sched: __u8,
    pub idx: __u8,
    pub min: __u8,
    pub max: __u8,
    pub binding: __u8,
    pub rsvd1: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_iscsi_hdr {
    pub ot: RSS_HDR union opcode_tid,
    pub pdu_len_ddp: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,
    pub rsvd: __u8,
    pub status: __u8,
}

// cpl_iscsi_hdr.pdu_len_ddp fields
pub const S_ISCSI_PDU_LEN: c_int = 0;
pub const M_ISCSI_PDU_LEN: c_uint = 0x7FFF;

pub const S_ISCSI_DDP: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data {
    pub ot: RSS_HDR union opcode_tid,
    pub rsvd: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,

    pub dack_mode:2: __u8,
    pub psh:1: __u8,
    pub heartbeat:1: __u8,

    pub heartbeat:1: __u8,
    pub psh:1: __u8,
    pub dack_mode:2: __u8,

    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ack {
    pub ot: opcode_tid,
    pub credit_dack: __be32,
}

// cpl_rx_data_ack.ack_seq fields
pub const S_RX_CREDITS: c_int = 0;
pub const M_RX_CREDITS: c_uint = 0x7FFFFFF;

pub const S_RX_MODULATE: c_int = 27;

pub const S_RX_FORCE_ACK: c_int = 28;

pub const S_RX_DACK_MODE: c_int = 29;
pub const M_RX_DACK_MODE: c_uint = 0x3;

pub const S_RX_DACK_CHANGE: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_urg_notify {
    pub ot: RSS_HDR union opcode_tid,
    pub seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_ddp_complete {
    pub ot: RSS_HDR union opcode_tid,
    pub ddp_report: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ddp {
    pub ot: RSS_HDR union opcode_tid,
    pub urg: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub nxt_seq: __be32,
    pub ddp_report: __be32,
}

// cpl_rx_data_ddp.ddpvld_status fields
pub const S_DDP_STATUS: c_int = 0;
pub const M_DDP_STATUS: c_uint = 0xFF;

pub const S_DDP_VALID: c_int = 15;
pub const M_DDP_VALID: c_uint = 0x1FFFF;

pub const S_DDP_PPOD_MISMATCH: c_int = 15;

pub const S_DDP_PDU: c_int = 16;

pub const S_DDP_LLIMIT_ERR: c_int = 17;

pub const S_DDP_PPOD_PARITY_ERR: c_int = 18;

pub const S_DDP_PADDING_ERR: c_int = 19;

pub const S_DDP_HDRCRC_ERR: c_int = 20;

pub const S_DDP_DATACRC_ERR: c_int = 21;

pub const S_DDP_INVALID_TAG: c_int = 22;

pub const S_DDP_ULIMIT_ERR: c_int = 23;

pub const S_DDP_OFFSET_ERR: c_int = 24;

pub const S_DDP_COLOR_ERR: c_int = 25;

pub const S_DDP_TID_MISMATCH: c_int = 26;

pub const S_DDP_INVALID_PPOD: c_int = 27;

pub const S_DDP_ULP_MODE: c_int = 28;
pub const M_DDP_ULP_MODE: c_uint = 0xF;

// cpl_rx_data_ddp.ddp_report fields
pub const S_DDP_OFFSET: c_int = 0;
pub const M_DDP_OFFSET: c_uint = 0x3FFFFF;

pub const S_DDP_URG: c_int = 24;

pub const S_DDP_PSH: c_int = 25;

pub const S_DDP_BUF_COMPLETE: c_int = 26;

pub const S_DDP_BUF_TIMED_OUT: c_int = 27;

pub const S_DDP_BUF_IDX: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt {
    pub cntrl: __be32,
    pub len: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt_lso {
    pub cntrl: __be32,
    pub len: __be32,
    pub rsvd: __be32,
    pub lso_info: __be32,
}

// cpl_tx_pkt*.cntrl fields
pub const S_TXPKT_VLAN: c_int = 0;
pub const M_TXPKT_VLAN: c_uint = 0xFFFF;

pub const S_TXPKT_INTF: c_int = 16;
pub const M_TXPKT_INTF: c_uint = 0xF;

pub const S_TXPKT_IPCSUM_DIS: c_int = 20;

pub const S_TXPKT_L4CSUM_DIS: c_int = 21;

pub const S_TXPKT_VLAN_VLD: c_int = 22;

pub const S_TXPKT_LOOPBACK: c_int = 23;

pub const S_TXPKT_OPCODE: c_int = 24;
pub const M_TXPKT_OPCODE: c_uint = 0xFF;

// cpl_tx_pkt_lso.lso_info fields
pub const S_LSO_MSS: c_int = 0;
pub const M_LSO_MSS: c_uint = 0x3FFF;

pub const S_LSO_ETH_TYPE: c_int = 14;
pub const M_LSO_ETH_TYPE: c_uint = 0x3;

pub const S_LSO_TCPHDR_WORDS: c_int = 16;
pub const M_LSO_TCPHDR_WORDS: c_uint = 0xF;

pub const S_LSO_IPHDR_WORDS: c_int = 20;
pub const M_LSO_IPHDR_WORDS: c_uint = 0xF;

pub const S_LSO_IPV6: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_trace_pkt {

    pub rss_opcode: __u8,

    pub err:1: __u8,

    pub err:1: __u8,

    pub rsvd0: __u8,

    pub qid:4: __u8,

    pub qid:4: __u8,

    pub tstamp: __be32,

    pub opcode: __u8,

    pub iff:4: __u8,

    pub iff:4: __u8,
    pub rsvd: [__u8; 4],
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_pkt {
    pub opcode: RSS_HDR __u8,

    pub iff:4: __u8,
    pub csum_valid:1: __u8,
    pub ipmi_pkt:1: __u8,
    pub vlan_valid:1: __u8,
    pub fragment:1: __u8,

    pub fragment:1: __u8,
    pub vlan_valid:1: __u8,
    pub ipmi_pkt:1: __u8,
    pub csum_valid:1: __u8,
    pub iff:4: __u8,

    pub csum: __be16,
    pub vlan: __be16,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_req {
    pub ot: opcode_tid,
    pub params: __be32,
    pub rsvd: [__u8; 2],
    pub dst_mac: [__u8; 6],
}

// cpl_l2t_write_req.params fields
pub const S_L2T_W_IDX: c_int = 0;
pub const M_L2T_W_IDX: c_uint = 0x7FF;

pub const S_L2T_W_VLAN: c_int = 11;
pub const M_L2T_W_VLAN: c_uint = 0xFFF;

pub const S_L2T_W_IFF: c_int = 23;
pub const M_L2T_W_IFF: c_uint = 0xF;

pub const S_L2T_W_PRIO: c_int = 27;
pub const M_L2T_W_PRIO: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_read_req {
    pub ot: opcode_tid,
    pub rsvd: __be16,
    pub l2t_idx: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_read_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub params: __be32,
    pub rsvd: [__u8; 2],
    pub dst_mac: [__u8; 6],
}

// cpl_l2t_read_rpl.params fields
pub const S_L2T_R_PRIO: c_int = 0;
pub const M_L2T_R_PRIO: c_uint = 0x7;

pub const S_L2T_R_VLAN: c_int = 8;
pub const M_L2T_R_VLAN: c_uint = 0xFFF;

pub const S_L2T_R_IFF: c_int = 20;
pub const M_L2T_R_IFF: c_uint = 0xF;

pub const S_L2T_STATUS: c_int = 24;
pub const M_L2T_STATUS: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_req {
    pub ot: opcode_tid,
    pub rsvd0: __u8,

    pub mtu_idx:4: __u8,
    pub iff:4: __u8,

    pub iff:4: __u8,
    pub mtu_idx:4: __u8,

    pub rsvd2: __be16,
    pub rsvd3: __be16,
    pub src_mac1: [__u8; 6],
    pub rsvd4: __be16,
    pub src_mac0: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_read_req {
    pub ot: opcode_tid,
    pub rsvd0: __u8,

    pub iff:4: __u8,

    pub iff:4: __u8,

    pub rsvd2: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_read_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,

    pub mtu_idx:4: __u8,

    pub mtu_idx:4: __u8,

    pub rsvd2: __be16,
    pub rsvd3: __be16,
    pub src_mac1: [__u8; 6],
    pub rsvd4: __be16,
    pub src_mac0: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_delete_req {
    pub ot: opcode_tid,
    pub params: __be32,
}

// { cpl_rte_delete_req, cpl_rte_read_req }.params fields
pub const S_RTE_REQ_LUT_IX: c_int = 8;
pub const M_RTE_REQ_LUT_IX: c_uint = 0x7FF;

pub const S_RTE_REQ_LUT_BASE: c_int = 19;
pub const M_RTE_REQ_LUT_BASE: c_uint = 0x7FF;

pub const S_RTE_READ_REQ_SELECT: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_delete_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_write_req {
    pub ot: opcode_tid,

    pub write_tcam:1: __u8,
    pub write_l2t_lut:1: __u8,

    pub write_l2t_lut:1: __u8,
    pub write_tcam:1: __u8,
    pub rsvd: [__u8; 3],
    pub lut_params: __be32,
    pub rsvd2: __be16,
    pub l2t_idx: __be16,
    pub netmask: __be32,
    pub faddr: __be32,
}

// cpl_rte_write_req.lut_params fields
pub const S_RTE_WRITE_REQ_LUT_IX: c_int = 10;
pub const M_RTE_WRITE_REQ_LUT_IX: c_uint = 0x7FF;

pub const S_RTE_WRITE_REQ_LUT_BASE: c_int = 21;
pub const M_RTE_WRITE_REQ_LUT_BASE: c_uint = 0x7FF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_write_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_read_req {
    pub ot: opcode_tid,
    pub params: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_read_rpl {
    pub ot: RSS_HDR union opcode_tid,
    pub status: __u8,
    pub rsvd0: __u8,
    pub l2t_idx: __be16,

    pub select:1: __u8,

    pub select:1: __u8,
    pub rsvd2: [__u8; 3],
    pub addr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tid_release {
    pub ot: opcode_tid,
    pub rsvd: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_barrier {
    pub opcode: __u8,
    pub rsvd: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rdma_read_req {
    pub opcode: __u8,
    pub rsvd: [__u8; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rdma_terminate {

    pub opcode: __u8,
    pub rsvd: [__u8; 2],
    pub rspq:3: __u8,

    pub rspq:3: __u8,

    pub tid_len: __be32,

    pub msn: __be32,
    pub mo: __be32,
    pub data: [__u8; ],
}

// cpl_rdma_terminate.tid_len fields
pub const S_FLIT_CNT: c_int = 0;
pub const M_FLIT_CNT: c_uint = 0xFF;

pub const S_TERM_TID: c_int = 8;
pub const M_TERM_TID: c_uint = 0xFFFFF;

// ULP_TX opcodes
pub const S_ULPTX_CMD: c_int = 28;
pub const M_ULPTX_CMD: c_uint = 0xF;

pub const S_ULPTX_NFLITS: c_int = 0;
pub const M_ULPTX_NFLITS: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_mem_io {
    pub cmd_lock_addr: __be32,
    pub len: __be32,
}

// ulp_mem_io.cmd_lock_addr fields
pub const S_ULP_MEMIO_ADDR: c_int = 0;
pub const M_ULP_MEMIO_ADDR: c_uint = 0x7FFFFFF;

pub const S_ULP_MEMIO_LOCK: c_int = 27;

// ulp_mem_io.len fields
pub const S_ULP_MEMIO_DATA_LEN: c_int = 28;
pub const M_ULP_MEMIO_DATA_LEN: c_uint = 0xF;

