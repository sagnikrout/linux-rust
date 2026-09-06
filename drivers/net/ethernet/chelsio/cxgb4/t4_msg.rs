//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_msg.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2014 Chelsio Communications, Inc. All rights reserved.
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
pub enum CPL_error {
    CPL_ERR_NONE               = 0,
    CPL_ERR_TCAM_PARITY        = 1,
    CPL_ERR_TCAM_MISS          = 2,
    CPL_ERR_TCAM_FULL          = 3,
    CPL_ERR_BAD_LENGTH         = 15,
    CPL_ERR_BAD_ROUTE          = 18,
    CPL_ERR_CONN_RESET         = 20,
    CPL_ERR_CONN_EXIST_SYNRECV = 21,
    CPL_ERR_CONN_EXIST         = 22,
    CPL_ERR_ARP_MISS           = 23,
    CPL_ERR_BAD_SYN            = 24,
    CPL_ERR_CONN_TIMEDOUT      = 30,
    CPL_ERR_XMIT_TIMEDOUT      = 31,
    CPL_ERR_PERSIST_TIMEDOUT   = 32,
    CPL_ERR_FINWAIT2_TIMEDOUT  = 33,
    CPL_ERR_KEEPALIVE_TIMEDOUT = 34,
    CPL_ERR_RTX_NEG_ADVICE     = 35,
    CPL_ERR_PERSIST_NEG_ADVICE = 36,
    CPL_ERR_KEEPALV_NEG_ADVICE = 37,
    CPL_ERR_ABORT_FAILED       = 42,
    CPL_ERR_IWARP_FLM          = 50,
    CPL_CONTAINS_READ_RPL      = 60,
    CPL_CONTAINS_WRITE_RPL     = 61,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union opcode_tid {
    pub opcode_tid: __be32,
    pub opcode: u8,
}

pub const CPL_OPCODE_S: c_int = 24;

// tid is assumed to be 24-bits

// extract the TID from a CPL command

// partitioning of TID fields that also carry a queue id
pub const TID_TID_S: c_int = 0;
pub const TID_TID_M: c_uint = 0x3fff;

pub const TID_QID_S: c_int = 14;
pub const TID_QID_M: c_uint = 0x3ff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_header {
    pub opcode: u8,

    pub channel:2: u8,
    pub filter_hit:1: u8,
    pub filter_tid:1: u8,
    pub hash_type:2: u8,
    pub ipv6:1: u8,
    pub send2fw:1: u8,

    pub send2fw:1: u8,
    pub ipv6:1: u8,
    pub hash_type:2: u8,
    pub filter_tid:1: u8,
    pub filter_hit:1: u8,
    pub channel:2: u8,

    pub qid: __be16,
    pub hash_val: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_request_hdr {
    pub wr_hi: __be32,
    pub wr_mid: __be32,
    pub wr_lo: __be64,
}

// wr_hi fields
pub const WR_OP_S: c_int = 24;

// option 0 fields
pub const TX_CHAN_S: c_int = 2;

pub const ULP_MODE_S: c_int = 8;

pub const RCV_BUFSIZ_S: c_int = 12;
pub const RCV_BUFSIZ_M: c_uint = 0x3FFU;

pub const SMAC_SEL_S: c_int = 28;

pub const L2T_IDX_S: c_int = 36;

pub const WND_SCALE_S: c_int = 50;

pub const KEEP_ALIVE_S: c_int = 54;

pub const MSS_IDX_S: c_int = 60;
pub const MSS_IDX_M: c_uint = 0xF;

// option 2 fields
pub const RSS_QUEUE_S: c_int = 0;
pub const RSS_QUEUE_M: c_uint = 0x3FF;

pub const RSS_QUEUE_VALID_S: c_int = 10;

pub const RX_FC_DISABLE_S: c_int = 20;

pub const RX_FC_VALID_S: c_int = 22;

pub const RX_CHANNEL_S: c_int = 26;

pub const WND_SCALE_EN_S: c_int = 28;

pub const T5_OPT_2_VALID_S: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0: __be64,
    pub opt1: __be64,
}

// option 0 fields
pub const NO_CONG_S: c_int = 4;

pub const DELACK_S: c_int = 5;

pub const NON_OFFLOAD_S: c_int = 7;

pub const DSCP_S: c_int = 22;
pub const DSCP_M: c_uint = 0x3F;

pub const TCAM_BYPASS_S: c_int = 48;

pub const NAGLE_S: c_int = 49;

// option 1 fields
pub const SYN_RSS_ENABLE_S: c_int = 0;

pub const SYN_RSS_QUEUE_S: c_int = 2;

pub const CONN_POLICY_S: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_req6 {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip_hi: __be64,
    pub local_ip_lo: __be64,
    pub peer_ip_hi: __be64,
    pub peer_ip_lo: __be64,
    pub opt0: __be64,
    pub opt1: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_rpl {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_options {
    pub mss: __be16,
    pub wsf: __u8,

    pub unknown:1: __u8,
    pub sack:1: __u8,
    pub tstamp:1: __u8,

    pub tstamp:1: __u8,
    pub sack:1: __u8,
    pub unknown:1: __u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_req {
    pub ot: opcode_tid,
    pub rsvd: __be16,
    pub len: __be16,
    pub hdr_len: __be32,
    pub vlan: __be16,
    pub l2info: __be16,
    pub tos_stid: __be32,
    pub tcpopt: tcp_options,
}

// cpl_pass_accept_req.hdr_len fields
pub const SYN_RX_CHAN_S: c_int = 0;
pub const SYN_RX_CHAN_M: c_uint = 0xF;

pub const TCP_HDR_LEN_S: c_int = 10;
pub const TCP_HDR_LEN_M: c_uint = 0x3F;

pub const IP_HDR_LEN_S: c_int = 16;
pub const IP_HDR_LEN_M: c_uint = 0x3FF;

pub const ETH_HDR_LEN_S: c_int = 26;
pub const ETH_HDR_LEN_M: c_uint = 0x1F;

// cpl_pass_accept_req.l2info fields
pub const SYN_MAC_IDX_S: c_int = 0;
pub const SYN_MAC_IDX_M: c_uint = 0x1FF;

pub const SYN_XACT_MATCH_S: c_int = 9;

pub const SYN_INTF_S: c_int = 12;
pub const SYN_INTF_M: c_uint = 0xF;

pub const CONG_CNTRL_S: c_int = 14;
pub const CONG_CNTRL_M: c_uint = 0x3;

pub const T5_ISS_S: c_int = 18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_rpl {
    pub ot: opcode_tid,
    pub opt2: __be32,
    pub opt0: __be64,
}

// option 2 fields
pub const RX_COALESCE_VALID_S: c_int = 11;

pub const RX_COALESCE_S: c_int = 12;

pub const PACE_S: c_int = 16;

pub const TX_QUEUE_S: c_int = 23;
pub const TX_QUEUE_M: c_uint = 0x7;

pub const CCTRL_ECN_S: c_int = 27;

pub const TSTAMPS_EN_S: c_int = 29;

pub const SACK_EN_S: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t5_pass_accept_rpl {
    pub ot: opcode_tid,
    pub opt2: __be32,
    pub opt0: __be64,
    pub iss: __be32,
    pub rsvd: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0: __be64,
    pub params: __be32,
    pub opt2: __be32,
}

pub const FILTER_TUPLE_S: c_int = 24;
pub const FILTER_TUPLE_M: c_uint = 0xFFFFFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t5_act_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0: __be64,
    pub rsvd: __be32,
    pub opt2: __be32,
    pub params: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t6_act_open_req {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip: __be32,
    pub peer_ip: __be32,
    pub opt0: __be64,
    pub rsvd: __be32,
    pub opt2: __be32,
    pub params: __be64,
    pub rsvd2: __be32,
    pub opt3: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_req6 {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip_hi: __be64,
    pub local_ip_lo: __be64,
    pub peer_ip_hi: __be64,
    pub peer_ip_lo: __be64,
    pub opt0: __be64,
    pub params: __be32,
    pub opt2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t5_act_open_req6 {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip_hi: __be64,
    pub local_ip_lo: __be64,
    pub peer_ip_hi: __be64,
    pub peer_ip_lo: __be64,
    pub opt0: __be64,
    pub rsvd: __be32,
    pub opt2: __be32,
    pub params: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t6_act_open_req6 {
    pub ot: opcode_tid,
    pub local_port: __be16,
    pub peer_port: __be16,
    pub local_ip_hi: __be64,
    pub local_ip_lo: __be64,
    pub peer_ip_hi: __be64,
    pub peer_ip_lo: __be64,
    pub opt0: __be64,
    pub rsvd: __be32,
    pub opt2: __be32,
    pub params: __be64,
    pub rsvd2: __be32,
    pub opt3: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_rpl {
    pub ot: opcode_tid,
    pub atid_status: __be32,
}

// cpl_act_open_rpl.atid_status fields
pub const AOPEN_STATUS_S: c_int = 0;
pub const AOPEN_STATUS_M: c_uint = 0xFF;

pub const AOPEN_ATID_S: c_int = 8;
pub const AOPEN_ATID_M: c_uint = 0xFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_establish {
    pub ot: opcode_tid,
    pub rsvd: __be32,
    pub tos_stid: __be32,
    pub mac_idx: __be16,
    pub tcp_opt: __be16,
    pub snd_isn: __be32,
    pub rcv_isn: __be32,
}

// cpl_pass_establish.tos_stid fields
pub const PASS_OPEN_TID_S: c_int = 0;
pub const PASS_OPEN_TID_M: c_uint = 0xFFFFFF;

pub const PASS_OPEN_TOS_S: c_int = 24;
pub const PASS_OPEN_TOS_M: c_uint = 0xFF;

// cpl_pass_establish.tcp_opt fields (also applies to act_open_establish)
pub const TCPOPT_WSCALE_OK_S: c_int = 5;
pub const TCPOPT_WSCALE_OK_M: c_uint = 0x1;

pub const TCPOPT_SACK_S: c_int = 6;
pub const TCPOPT_SACK_M: c_uint = 0x1;

pub const TCPOPT_TSTAMP_S: c_int = 7;
pub const TCPOPT_TSTAMP_M: c_uint = 0x1;

pub const TCPOPT_SND_WSCALE_S: c_int = 8;
pub const TCPOPT_SND_WSCALE_M: c_uint = 0xF;

pub const TCPOPT_MSS_S: c_int = 12;
pub const TCPOPT_MSS_M: c_uint = 0xF;

pub const T6_TCP_HDR_LEN_S: c_int = 8;

pub const T6_IP_HDR_LEN_S: c_int = 14;

pub const T6_ETH_HDR_LEN_S: c_int = 24;
pub const T6_ETH_HDR_LEN_M: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_establish {
    pub ot: opcode_tid,
    pub rsvd: __be32,
    pub tos_atid: __be32,
    pub mac_idx: __be16,
    pub tcp_opt: __be16,
    pub snd_isn: __be32,
    pub rcv_isn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb {
    pub ot: opcode_tid,
    pub reply_ctrl: __be16,
    pub cookie: __be16,
}

// cpl_get_tcb.reply_ctrl fields
pub const QUEUENO_S: c_int = 0;

pub const REPLY_CHAN_S: c_int = 14;

pub const NO_REPLY_S: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb_rpl {
    pub ot: opcode_tid,
    pub cookie: __u8,
    pub status: __u8,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_field {
    pub ot: opcode_tid,
    pub reply_ctrl: __be16,
    pub word_cookie: __be16,
    pub mask: __be64,
    pub val: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_field_core {
    pub ot: opcode_tid,
    pub reply_ctrl: __be16,
    pub word_cookie: __be16,
    pub mask: __be64,
    pub val: __be64,
}

// cpl_set_tcb_field.word_cookie fields
pub const TCB_WORD_S: c_int = 0;

pub const TCB_COOKIE_S: c_int = 5;
pub const TCB_COOKIE_M: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_rpl {
    pub ot: opcode_tid,
    pub rsvd: __be16,
    pub cookie: u8,
    pub status: u8,
    pub oldval: __be64,
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
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
    pub snd_nxt: __be32,
    pub rcv_nxt: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listsvr_req {
    pub ot: opcode_tid,
    pub reply_ctrl: __be16,
    pub rsvd: __be16,
}

// additional cpl_close_listsvr_req.reply_ctrl field
pub const LISTSVR_IPV6_S: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listsvr_rpl {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req_rss {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req_rss6 {
    pub ot: opcode_tid,
    pub srqidx_status: __be32,
}

pub const ABORT_RSS_STATUS_S: c_int = 0;
pub const ABORT_RSS_STATUS_M: c_uint = 0xff;

pub const ABORT_RSS_SRQIDX_S: c_int = 8;
pub const ABORT_RSS_SRQIDX_M: c_uint = 0xffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req {
    pub ot: opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: u8,
    pub cmd: u8,
    pub rsvd2: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl_rss {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl_rss6 {
    pub ot: opcode_tid,
    pub srqidx_status: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl {
    pub ot: opcode_tid,
    pub rsvd0: __be32,
    pub rsvd1: u8,
    pub cmd: u8,
    pub rsvd2: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_peer_close {
    pub ot: opcode_tid,
    pub rcv_nxt: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tid_release {
    pub ot: opcode_tid,
    pub rsvd: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt_core {
    pub ctrl0: __be32,
    pub pack: __be16,
    pub len: __be16,
    pub ctrl1: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt {
    pub c: cpl_tx_pkt_core,
}

// cpl_tx_pkt_core.ctrl0 fields
pub const TXPKT_VF_S: c_int = 0;

pub const TXPKT_PF_S: c_int = 8;

pub const TXPKT_VF_VLD_S: c_int = 11;

pub const TXPKT_OVLAN_IDX_S: c_int = 12;

pub const TXPKT_T5_OVLAN_IDX_S: c_int = 12;

pub const TXPKT_INTF_S: c_int = 16;

pub const TXPKT_INS_OVLAN_S: c_int = 21;

pub const TXPKT_TSTAMP_S: c_int = 23;

pub const TXPKT_OPCODE_S: c_int = 24;

// cpl_tx_pkt_core.ctrl1 fields
pub const TXPKT_CSUM_END_S: c_int = 12;

pub const TXPKT_CSUM_START_S: c_int = 20;

pub const TXPKT_IPHDR_LEN_S: c_int = 20;

pub const TXPKT_CSUM_LOC_S: c_int = 30;

pub const TXPKT_ETHHDR_LEN_S: c_int = 34;

pub const T6_TXPKT_ETHHDR_LEN_S: c_int = 32;

pub const TXPKT_CSUM_TYPE_S: c_int = 40;

pub const TXPKT_VLAN_S: c_int = 44;

pub const TXPKT_VLAN_VLD_S: c_int = 60;

pub const TXPKT_IPCSUM_DIS_S: c_int = 62;

pub const TXPKT_L4CSUM_DIS_S: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt_lso_core {
    pub lso_ctrl: __be32,
    pub ipid_ofst: __be16,
    pub mss: __be16,
    pub seqno_offset: __be32,
    pub len: __be32,
// encapsulated CPL (TX_PKT, TX_PKT_XT or TX_DATA) follows here
}

// cpl_tx_pkt_lso_core.lso_ctrl fields
pub const LSO_TCPHDR_LEN_S: c_int = 0;

pub const LSO_IPHDR_LEN_S: c_int = 4;

pub const LSO_ETHHDR_LEN_S: c_int = 16;

pub const LSO_IPV6_S: c_int = 20;

pub const LSO_LAST_SLICE_S: c_int = 22;

pub const LSO_FIRST_SLICE_S: c_int = 23;

pub const LSO_OPCODE_S: c_int = 24;

pub const LSO_T5_XFER_SIZE_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt_lso {
    pub c: cpl_tx_pkt_lso_core,
// encapsulated CPL (TX_PKT, TX_PKT_XT or TX_DATA) follows here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_iscsi_hdr {
    pub ot: opcode_tid,
    pub pdu_len_ddp: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,
    pub rsvd: u8,
    pub status: u8,
}

// cpl_iscsi_hdr.pdu_len_ddp fields
pub const ISCSI_PDU_LEN_S: c_int = 0;
pub const ISCSI_PDU_LEN_M: c_uint = 0x7FFF;

pub const ISCSI_DDP_S: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ddp {
    pub ot: opcode_tid,
    pub urg: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub nxt_seq: __be32,
    pub ddp_report: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_iscsi_data {
    pub ot: opcode_tid,
    pub rsvd0: [__u8; 2],
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,
    pub rsvd1: __u8,
    pub status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_iscsi_cmp {
    pub ot: opcode_tid,
    pub pdu_len_ddp: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,
    pub rsvd: __u8,
    pub status: __u8,
    pub ulp_crc: __be32,
    pub ddpvld: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data_iso {
    pub op_to_scsi: __be32,
    pub reserved1: __u8,
    pub ahs_len: __u8,
    pub mpdu: __be16,
    pub burst_size: __be32,
    pub len: __be32,
    pub reserved2_seglen_offset: __be32,
    pub datasn_offset: __be32,
    pub buffer_offset: __be32,
    pub reserved3: __be32,
// encapsulated CPL_TX_DATA follows here
}

// cpl_tx_data_iso.op_to_scsi fields
pub const CPL_TX_DATA_ISO_OP_S: c_int = 24;
pub const CPL_TX_DATA_ISO_OP_M: c_uint = 0xff;

pub const CPL_TX_DATA_ISO_FIRST_S: c_int = 23;
pub const CPL_TX_DATA_ISO_FIRST_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_LAST_S: c_int = 22;
pub const CPL_TX_DATA_ISO_LAST_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_CPLHDRLEN_S: c_int = 21;
pub const CPL_TX_DATA_ISO_CPLHDRLEN_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_HDRCRC_S: c_int = 20;
pub const CPL_TX_DATA_ISO_HDRCRC_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_PLDCRC_S: c_int = 19;
pub const CPL_TX_DATA_ISO_PLDCRC_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_IMMEDIATE_S: c_int = 18;
pub const CPL_TX_DATA_ISO_IMMEDIATE_M: c_uint = 0x1;

pub const CPL_TX_DATA_ISO_SCSI_S: c_int = 16;
pub const CPL_TX_DATA_ISO_SCSI_M: c_uint = 0x3;

// cpl_tx_data_iso.reserved2_seglen_offset fields
pub const CPL_TX_DATA_ISO_SEGLEN_OFFSET_S: c_int = 0;
pub const CPL_TX_DATA_ISO_SEGLEN_OFFSET_M: c_uint = 0xffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data {
    pub ot: opcode_tid,
    pub rsvd: __be16,
    pub len: __be16,
    pub seq: __be32,
    pub urg: __be16,

    pub dack_mode:2: u8,
    pub psh:1: u8,
    pub heartbeat:1: u8,
    pub ddp_off:1: u8,
    pub :3: u8,

    pub :3: u8,
    pub ddp_off:1: u8,
    pub heartbeat:1: u8,
    pub psh:1: u8,
    pub dack_mode:2: u8,

    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ack {
    pub ot: opcode_tid,
    pub credit_dack: __be32,
}

// cpl_rx_data_ack.ack_seq fields
pub const RX_CREDITS_S: c_int = 0;

pub const RX_FORCE_ACK_S: c_int = 28;

pub const RX_DACK_MODE_S: c_int = 29;
pub const RX_DACK_MODE_M: c_uint = 0x3;

pub const RX_DACK_CHANGE_S: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_pkt {
    pub rsshdr: rss_header,
    pub opcode: u8,

    pub iff:4: u8,
    pub csum_calc:1: u8,
    pub ipmi_pkt:1: u8,
    pub vlan_ex:1: u8,
    pub ip_frag:1: u8,

    pub ip_frag:1: u8,
    pub vlan_ex:1: u8,
    pub ipmi_pkt:1: u8,
    pub csum_calc:1: u8,
    pub iff:4: u8,

    pub csum: __be16,
    pub vlan: __be16,
    pub len: __be16,
    pub l2info: __be32,
    pub hdr_len: __be16,
    pub err_vec: __be16,
}

pub const RX_T6_ETHHDR_LEN_M: c_uint = 0xFF;

pub const RXF_PSH_S: c_int = 20;

pub const RXF_SYN_S: c_int = 21;

pub const RXF_UDP_S: c_int = 22;

pub const RXF_TCP_S: c_int = 23;

pub const RXF_IP_S: c_int = 24;

pub const RXF_IP6_S: c_int = 25;

pub const RXF_SYN_COOKIE_S: c_int = 26;

pub const RXF_FCOE_S: c_int = 26;

pub const RXF_LRO_S: c_int = 27;

// rx_pkt.l2info fields
pub const RX_ETHHDR_LEN_S: c_int = 0;
pub const RX_ETHHDR_LEN_M: c_uint = 0x1F;

pub const RX_T5_ETHHDR_LEN_S: c_int = 0;
pub const RX_T5_ETHHDR_LEN_M: c_uint = 0x3F;

pub const RX_MACIDX_S: c_int = 8;
pub const RX_MACIDX_M: c_uint = 0x1FF;

pub const RXF_SYN_S: c_int = 21;

pub const RX_CHAN_S: c_int = 28;
pub const RX_CHAN_M: c_uint = 0xF;

// rx_pkt.hdr_len fields
pub const RX_TCPHDR_LEN_S: c_int = 0;
pub const RX_TCPHDR_LEN_M: c_uint = 0x3F;

pub const RX_IPHDR_LEN_S: c_int = 6;
pub const RX_IPHDR_LEN_M: c_uint = 0x3FF;

// rx_pkt.err_vec fields
pub const RXERR_CSUM_S: c_int = 13;

pub const T6_COMPR_RXERR_LEN_S: c_int = 1;

pub const T6_COMPR_RXERR_VEC_S: c_int = 0;
pub const T6_COMPR_RXERR_VEC_M: c_uint = 0x3F;

// Logical OR of RX_ERROR_CSUM, RX_ERROR_CSIP
pub const T6_COMPR_RXERR_SUM_S: c_int = 4;

pub const T6_RX_TNLHDR_LEN_S: c_int = 8;
pub const T6_RX_TNLHDR_LEN_M: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_trace_pkt {
    pub opcode: u8,
    pub intf: u8,

    pub runt:4: u8,
    pub filter_hit:4: u8,
    pub :6: u8,
    pub err:1: u8,
    pub trunc:1: u8,

    pub filter_hit:4: u8,
    pub runt:4: u8,
    pub trunc:1: u8,
    pub err:1: u8,
    pub :6: u8,

    pub rsvd: __be16,
    pub len: __be16,
    pub tstamp: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t5_trace_pkt {
    pub opcode: __u8,
    pub intf: __u8,

    pub runt:4: __u8,
    pub filter_hit:4: __u8,
    pub err:1: __u8,
    pub trunc:1: __u8,

    pub filter_hit:4: __u8,
    pub runt:4: __u8,
    pub trunc:1: __u8,
    pub err:1: __u8,

    pub rsvd: __be16,
    pub len: __be16,
    pub tstamp: __be64,
    pub rsvd1: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_req {
    pub ot: opcode_tid,
    pub params: __be16,
    pub l2t_idx: __be16,
    pub vlan: __be16,
    pub dst_mac: [u8; 6],
}

// cpl_l2t_write_req.params fields
pub const L2T_W_INFO_S: c_int = 2;

pub const L2T_W_PORT_S: c_int = 8;

pub const L2T_W_NOREPLY_S: c_int = 15;

pub const CPL_L2T_VLAN_NONE: c_uint = 0xfff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_req {
    pub ot: opcode_tid,
    pub params: __be32,
    pub pfvf1: __be16,
    pub src_mac1: [u8; 6],
    pub pfvf0: __be16,
    pub src_mac0: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_t6_smt_write_req {
    pub ot: opcode_tid,
    pub params: __be32,
    pub tag: __be64,
    pub pfvf0: __be16,
    pub src_mac0: [u8; 6],
    pub local_ip: __be32,
    pub rsvd: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

// cpl_smt_{read,write}_req.params fields
pub const SMTW_OVLAN_IDX_S: c_int = 16;

pub const SMTW_IDX_S: c_int = 20;

pub const SMTW_NORPL_S: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rdma_terminate {
    pub ot: opcode_tid,
    pub rsvd: __be16,
    pub len: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_sge_egr_update {
    pub opcode_qid: __be32,
    pub cidx: __be16,
    pub pidx: __be16,
}

// cpl_sge_egr_update.ot fields
pub const EGR_QID_S: c_int = 0;
pub const EGR_QID_M: c_uint = 0x1FFFF;

// cpl_fw*.type values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw4_pld {
    pub opcode: u8,
    pub rsvd0: [u8; 3],
    pub type: u8,
    pub rsvd1: u8,
    pub len: __be16,
    pub data: __be64,
    pub rsvd2: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw6_pld {
    pub opcode: u8,
    pub rsvd: [u8; 5],
    pub len: __be16,
    pub data: [__be64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw4_msg {
    pub opcode: u8,
    pub type: u8,
    pub rsvd0: __be16,
    pub rsvd1: __be32,
    pub data: [__be64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw4_ack {
    pub ot: opcode_tid,
    pub credits: u8,
    pub rsvd0: [u8; 2],
    pub seq_vld: u8,
    pub snd_nxt: __be32,
    pub snd_una: __be32,
    pub rsvd1: __be64,
}

pub const CPL_FW4_ACK_FLOWID_S: c_int = 0;
pub const CPL_FW4_ACK_FLOWID_M: c_uint = 0xffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw6_msg {
    pub opcode: u8,
    pub type: u8,
    pub rsvd0: __be16,
    pub rsvd1: __be32,
    pub data: [__be64; 4],
}

// cpl_fw6_msg.type values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_fw6_msg_ofld_connection_wr_rpl {
    pub cookie: __u64,
    pub /: *mut *mut __be32 tid; / or atid in case of active failure,
    pub t_state: __u8,
    pub retval: __u8,
    pub rsvd: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data {
    pub ot: opcode_tid,
    pub len: __be32,
    pub rsvd: __be32,
    pub flags: __be32,
}

// cpl_tx_data.flags field
pub const TX_FORCE_S: c_int = 13;

pub const TX_DATA_MSS_S: c_int = 16;
pub const TX_DATA_MSS_M: c_uint = 0xFFFF;

pub const TX_LENGTH_S: c_int = 0;
pub const TX_LENGTH_M: c_uint = 0xFFFF;

pub const T6_TX_FORCE_S: c_int = 20;

pub const TX_URG_S: c_int = 16;

pub const TX_SHOVE_S: c_int = 14;

pub const TX_BYPASS_S: c_int = 21;

pub const TX_PUSH_S: c_int = 22;

pub const TX_ULP_MODE_S: c_int = 10;
pub const TX_ULP_MODE_M: c_uint = 0x7;

pub const ULPTX_CMD_S: c_int = 24;

pub const ULPTX_LEN16_S: c_int = 0;
pub const ULPTX_LEN16_M: c_uint = 0xFF;

pub const ULP_TX_SC_MORE_S: c_int = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulptx_sge_pair {
    pub len: [__be32; 2],
    pub addr: [__be64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulptx_sgl {
    pub cmd_nsge: __be32,
    pub len0: __be32,
    pub addr0: __be64,
    pub sge: [ulptx_sge_pair; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulptx_idata {
    pub cmd_more: __be32,
    pub len: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_txpkt {
    pub cmd_dest: __be32,
    pub len: __be32,
}

pub const ULPTX_CMD_S: c_int = 24;
pub const ULPTX_CMD_M: c_uint = 0xFF;

pub const ULPTX_NSGE_S: c_int = 0;

pub const ULPTX_MORE_S: c_int = 23;

pub const ULP_TXPKT_DEST_S: c_int = 16;
pub const ULP_TXPKT_DEST_M: c_uint = 0x3;

pub const ULP_TXPKT_FID_S: c_int = 4;
pub const ULP_TXPKT_FID_M: c_uint = 0x7ff;

pub const ULP_TXPKT_RO_S: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpl_tx_tnl_lso_type {
    TX_TNL_TYPE_OPAQUE,
    TX_TNL_TYPE_NVGRE,
    TX_TNL_TYPE_VXLAN,
    TX_TNL_TYPE_GENEVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_tnl_lso {
    pub op_to_IpIdSplitOut: __be32,
    pub IpIdOffsetOut: __be16,
    pub UdpLenSetOut_to_TnlHdrLen: __be16,
    pub r1: __be64,
    pub Flow_to_TcpHdrLen: __be32,
    pub IpIdOffset: __be16,
    pub IpIdSplit_to_Mss: __be16,
    pub TCPSeqOffset: __be32,
    pub EthLenOffset_Size: __be32,
// encapsulated CPL (TX_PKT_XT) follows here
}

pub const CPL_TX_TNL_LSO_OPCODE_S: c_int = 24;
pub const CPL_TX_TNL_LSO_OPCODE_M: c_uint = 0xff;

pub const CPL_TX_TNL_LSO_FIRST_S: c_int = 23;
pub const CPL_TX_TNL_LSO_FIRST_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_LAST_S: c_int = 22;
pub const CPL_TX_TNL_LSO_LAST_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_ETHHDRLENXOUT_S: c_int = 21;
pub const CPL_TX_TNL_LSO_ETHHDRLENXOUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_IPV6OUT_S: c_int = 20;
pub const CPL_TX_TNL_LSO_IPV6OUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_ETHHDRLEN_S: c_int = 16;
pub const CPL_TX_TNL_LSO_ETHHDRLEN_M: c_uint = 0xf;

pub const CPL_TX_TNL_LSO_IPHDRLEN_S: c_int = 4;
pub const CPL_TX_TNL_LSO_IPHDRLEN_M: c_uint = 0xfff;

pub const CPL_TX_TNL_LSO_TCPHDRLEN_S: c_int = 0;
pub const CPL_TX_TNL_LSO_TCPHDRLEN_M: c_uint = 0xf;

pub const CPL_TX_TNL_LSO_MSS_S: c_int = 0;
pub const CPL_TX_TNL_LSO_MSS_M: c_uint = 0x3fff;

pub const CPL_TX_TNL_LSO_SIZE_S: c_int = 0;
pub const CPL_TX_TNL_LSO_SIZE_M: c_uint = 0xfffffff;

pub const CPL_TX_TNL_LSO_ETHHDRLENOUT_S: c_int = 16;
pub const CPL_TX_TNL_LSO_ETHHDRLENOUT_M: c_uint = 0xf;

pub const CPL_TX_TNL_LSO_IPHDRLENOUT_S: c_int = 4;
pub const CPL_TX_TNL_LSO_IPHDRLENOUT_M: c_uint = 0xfff;

pub const CPL_TX_TNL_LSO_IPHDRCHKOUT_S: c_int = 3;
pub const CPL_TX_TNL_LSO_IPHDRCHKOUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_IPLENSETOUT_S: c_int = 2;
pub const CPL_TX_TNL_LSO_IPLENSETOUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_IPIDINCOUT_S: c_int = 1;
pub const CPL_TX_TNL_LSO_IPIDINCOUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_UDPCHKCLROUT_S: c_int = 14;
pub const CPL_TX_TNL_LSO_UDPCHKCLROUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_UDPLENSETOUT_S: c_int = 15;
pub const CPL_TX_TNL_LSO_UDPLENSETOUT_M: c_uint = 0x1;

pub const CPL_TX_TNL_LSO_TNLTYPE_S: c_int = 12;
pub const CPL_TX_TNL_LSO_TNLTYPE_M: c_uint = 0x3;

pub const S_CPL_TX_TNL_LSO_ETHHDRLEN: c_int = 16;
pub const M_CPL_TX_TNL_LSO_ETHHDRLEN: c_uint = 0xf;

pub const CPL_TX_TNL_LSO_TNLHDRLEN_S: c_int = 0;
pub const CPL_TX_TNL_LSO_TNLHDRLEN_M: c_uint = 0xfff;

pub const CPL_TX_TNL_LSO_IPV6_S: c_int = 20;
pub const CPL_TX_TNL_LSO_IPV6_M: c_uint = 0x1;

pub const ULP_TX_SC_MORE_S: c_int = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_mem_io {
    pub cmd: __be32,
    pub /: *mut *mut __be32 len16; / command length,
    pub /: *mut *mut __be32 dlen; / data length in 32-byte units,
    pub lock_addr: __be32,
}

pub const ULP_MEMIO_LOCK_S: c_int = 31;

// additional ulp_mem_io.cmd fields
pub const ULP_MEMIO_ORDER_S: c_int = 23;

pub const T5_ULP_MEMIO_IMM_S: c_int = 23;

pub const T5_ULP_MEMIO_ORDER_S: c_int = 22;

pub const T5_ULP_MEMIO_FID_S: c_int = 4;
pub const T5_ULP_MEMIO_FID_M: c_uint = 0x7ff;

// ulp_mem_io.lock_addr fields
pub const ULP_MEMIO_ADDR_S: c_int = 0;

// ulp_mem_io.dlen fields
pub const ULP_MEMIO_DATA_LEN_S: c_int = 0;

pub const ULPTX_NSGE_S: c_int = 0;
pub const ULPTX_NSGE_M: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulptx_sc_memrd {
    pub cmd_to_len: __be32,
    pub addr: __be32,
}

pub const ULP_TXPKT_DATAMODIFY_S: c_int = 23;
pub const ULP_TXPKT_DATAMODIFY_M: c_uint = 0x1;

pub const ULP_TXPKT_CHANNELID_S: c_int = 22;
pub const ULP_TXPKT_CHANNELID_M: c_uint = 0x1;

pub const SCMD_SEQ_NO_CTRL_S: c_int = 29;
pub const SCMD_SEQ_NO_CTRL_M: c_uint = 0x3;

// StsFieldPrsnt- Status field at the end of the TLS PDU
pub const SCMD_STATUS_PRESENT_S: c_int = 28;
pub const SCMD_STATUS_PRESENT_M: c_uint = 0x1;

// ProtoVersion - Protocol Version 0: 1.2, 1:1.1, 2:DTLS, 3:Generic,
// 3-15: Reserved.
//
pub const SCMD_PROTO_VERSION_S: c_int = 24;
pub const SCMD_PROTO_VERSION_M: c_uint = 0xf;

// EncDecCtrl - Encryption/Decryption Control. 0: Encrypt, 1: Decrypt
pub const SCMD_ENC_DEC_CTRL_S: c_int = 23;
pub const SCMD_ENC_DEC_CTRL_M: c_uint = 0x1;

// CipherAuthSeqCtrl - Cipher Authentication Sequence Control.
pub const SCMD_CIPH_AUTH_SEQ_CTRL_S: c_int = 22;
pub const SCMD_CIPH_AUTH_SEQ_CTRL_M: c_uint = 0x1;

// CiphMode -  Cipher Mode. 0: NOP, 1:AES-CBC, 2:AES-GCM, 3:AES-CTR,
// 4:Generic-AES, 5-15: Reserved.
//
pub const SCMD_CIPH_MODE_S: c_int = 18;
pub const SCMD_CIPH_MODE_M: c_uint = 0xf;

// AuthMode - Auth Mode. 0: NOP, 1:SHA1, 2:SHA2-224, 3:SHA2-256
// 4-15: Reserved
//
pub const SCMD_AUTH_MODE_S: c_int = 14;
pub const SCMD_AUTH_MODE_M: c_uint = 0xf;

// HmacCtrl - HMAC Control. 0:NOP, 1:No truncation, 2:Support HMAC Truncation
// per RFC 4366, 3:IPSec 96 bits, 4-7:Reserved
//
pub const SCMD_HMAC_CTRL_S: c_int = 11;
pub const SCMD_HMAC_CTRL_M: c_uint = 0x7;

// IvSize - IV size in units of 2 bytes
pub const SCMD_IV_SIZE_S: c_int = 7;
pub const SCMD_IV_SIZE_M: c_uint = 0xf;

// NumIVs - Number of IVs
pub const SCMD_NUM_IVS_S: c_int = 0;
pub const SCMD_NUM_IVS_M: c_uint = 0x7f;

// EnbDbgId - If this is enabled upper 20 (63:44) bits if SeqNumber
// (below) are used as Cid (connection id for debug status), these
// bits are padded to zero for forming the 64 bit
// sequence number for TLS
//
pub const SCMD_ENB_DBGID_S: c_int = 31;
pub const SCMD_ENB_DBGID_M: c_uint = 0x1;

// IV generation in SW.
pub const SCMD_IV_GEN_CTRL_S: c_int = 30;
pub const SCMD_IV_GEN_CTRL_M: c_uint = 0x1;

// More frags
pub const SCMD_MORE_FRAGS_S: c_int = 20;
pub const SCMD_MORE_FRAGS_M: c_uint = 0x1;

// last frag
pub const SCMD_LAST_FRAG_S: c_int = 19;
pub const SCMD_LAST_FRAG_M: c_uint = 0x1;

// TlsCompPdu
pub const SCMD_TLS_COMPPDU_S: c_int = 18;
pub const SCMD_TLS_COMPPDU_M: c_uint = 0x1;

// KeyCntxtInline - Key context inline after the scmd  OR PayloadOnly
pub const SCMD_KEY_CTX_INLINE_S: c_int = 17;
pub const SCMD_KEY_CTX_INLINE_M: c_uint = 0x1;

// TLSFragEnable - 0: Host created TLS PDUs, 1: TLS Framgmentation in ASIC
pub const SCMD_TLS_FRAG_ENABLE_S: c_int = 16;
pub const SCMD_TLS_FRAG_ENABLE_M: c_uint = 0x1;

// MacOnly - Only send the MAC and discard PDU. This is valid for hash only
// modes, in this case TLS_TX  will drop the PDU and only
// send back the MAC bytes.
//
pub const SCMD_MAC_ONLY_S: c_int = 15;
pub const SCMD_MAC_ONLY_M: c_uint = 0x1;

// AadIVDrop - Drop the AAD and IV fields. Useful in protocols
// which have complex AAD and IV formations Eg:AES-CCM
//
pub const SCMD_AADIVDROP_S: c_int = 14;
pub const SCMD_AADIVDROP_M: c_uint = 0x1;

// HdrLength - Length of all headers excluding TLS header
// present before start of crypto PDU/payload.
//
pub const SCMD_HDR_LEN_S: c_int = 0;
pub const SCMD_HDR_LEN_M: c_uint = 0x3fff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_sec_pdu {
    pub op_ivinsrtofst: __be32,
    pub pldlen: __be32,
    pub aadstart_cipherstop_hi: __be32,
    pub cipherstop_lo_authinsert: __be32,
    pub seqno_numivs: __be32,
    pub ivgen_hdrlen: __be32,
    pub scmd1: __be64,
}

pub const CPL_TX_SEC_PDU_OPCODE_S: c_int = 24;
pub const CPL_TX_SEC_PDU_OPCODE_M: c_uint = 0xff;

// RX Channel Id
pub const CPL_TX_SEC_PDU_RXCHID_S: c_int = 22;
pub const CPL_TX_SEC_PDU_RXCHID_M: c_uint = 0x1;

// Ack Follows
pub const CPL_TX_SEC_PDU_ACKFOLLOWS_S: c_int = 21;
pub const CPL_TX_SEC_PDU_ACKFOLLOWS_M: c_uint = 0x1;

// Loopback bit in cpl_tx_sec_pdu
pub const CPL_TX_SEC_PDU_ULPTXLPBK_S: c_int = 20;
pub const CPL_TX_SEC_PDU_ULPTXLPBK_M: c_uint = 0x1;

// Length of cpl header encapsulated
pub const CPL_TX_SEC_PDU_CPLLEN_S: c_int = 16;
pub const CPL_TX_SEC_PDU_CPLLEN_M: c_uint = 0xf;

// PlaceHolder
pub const CPL_TX_SEC_PDU_PLACEHOLDER_S: c_int = 10;
pub const CPL_TX_SEC_PDU_PLACEHOLDER_M: c_uint = 0x1;

// IvInsrtOffset: Insertion location for IV
pub const CPL_TX_SEC_PDU_IVINSRTOFST_S: c_int = 0;
pub const CPL_TX_SEC_PDU_IVINSRTOFST_M: c_uint = 0x3ff;

// AadStartOffset: Offset in bytes for AAD start from
// the first byte following the pkt headers (0-255 bytes)
//
pub const CPL_TX_SEC_PDU_AADSTART_S: c_int = 24;
pub const CPL_TX_SEC_PDU_AADSTART_M: c_uint = 0xff;

// AadStopOffset: offset in bytes for AAD stop/end from the first byte following
// the pkt headers (0-511 bytes)
//
pub const CPL_TX_SEC_PDU_AADSTOP_S: c_int = 15;
pub const CPL_TX_SEC_PDU_AADSTOP_M: c_uint = 0x1ff;

// CipherStartOffset: offset in bytes for encryption/decryption start from the
// first byte following the pkt headers (0-1023 bytes)
//
pub const CPL_TX_SEC_PDU_CIPHERSTART_S: c_int = 5;
pub const CPL_TX_SEC_PDU_CIPHERSTART_M: c_uint = 0x3ff;

// CipherStopOffset: offset in bytes for encryption/decryption end
// from end of the payload of this command (0-511 bytes)
//
pub const CPL_TX_SEC_PDU_CIPHERSTOP_HI_S: c_int = 0;
pub const CPL_TX_SEC_PDU_CIPHERSTOP_HI_M: c_uint = 0x1f;

pub const CPL_TX_SEC_PDU_CIPHERSTOP_LO_S: c_int = 28;
pub const CPL_TX_SEC_PDU_CIPHERSTOP_LO_M: c_uint = 0xf;

// AuthStartOffset: offset in bytes for authentication start from
// the first byte following the pkt headers (0-1023)
//
pub const CPL_TX_SEC_PDU_AUTHSTART_S: c_int = 18;
pub const CPL_TX_SEC_PDU_AUTHSTART_M: c_uint = 0x3ff;

// AuthStopOffset: offset in bytes for authentication
// end from end of the payload of this command (0-511 Bytes)
//
pub const CPL_TX_SEC_PDU_AUTHSTOP_S: c_int = 9;
pub const CPL_TX_SEC_PDU_AUTHSTOP_M: c_uint = 0x1ff;

// AuthInsrtOffset: offset in bytes for authentication insertion
// from end of the payload of this command (0-511 bytes)
//
pub const CPL_TX_SEC_PDU_AUTHINSERT_S: c_int = 0;
pub const CPL_TX_SEC_PDU_AUTHINSERT_M: c_uint = 0x1ff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_phys_dsgl {
    pub op_to_tid: __be32,
    pub pcirlxorder_to_noofsgentr: __be32,
    pub rss_hdr_int: rss_header,
}

pub const CPL_RX_PHYS_DSGL_OPCODE_S: c_int = 24;
pub const CPL_RX_PHYS_DSGL_OPCODE_M: c_uint = 0xff;

pub const CPL_RX_PHYS_DSGL_ISRDMA_S: c_int = 23;
pub const CPL_RX_PHYS_DSGL_ISRDMA_M: c_uint = 0x1;

pub const CPL_RX_PHYS_DSGL_RSVD1_S: c_int = 20;
pub const CPL_RX_PHYS_DSGL_RSVD1_M: c_uint = 0x7;

pub const CPL_RX_PHYS_DSGL_PCIRLXORDER_S: c_int = 31;
pub const CPL_RX_PHYS_DSGL_PCIRLXORDER_M: c_uint = 0x1;

pub const CPL_RX_PHYS_DSGL_PCINOSNOOP_S: c_int = 30;
pub const CPL_RX_PHYS_DSGL_PCINOSNOOP_M: c_uint = 0x1;

pub const CPL_RX_PHYS_DSGL_PCITPHNTENB_S: c_int = 29;
pub const CPL_RX_PHYS_DSGL_PCITPHNTENB_M: c_uint = 0x1;

pub const CPL_RX_PHYS_DSGL_PCITPHNT_S: c_int = 27;
pub const CPL_RX_PHYS_DSGL_PCITPHNT_M: c_uint = 0x3;

pub const CPL_RX_PHYS_DSGL_DCAID_S: c_int = 16;
pub const CPL_RX_PHYS_DSGL_DCAID_M: c_uint = 0x7ff;

pub const CPL_RX_PHYS_DSGL_NOOFSGENTR_S: c_int = 0;
pub const CPL_RX_PHYS_DSGL_NOOFSGENTR_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_mps_pkt {
    pub op_to_r1_hi: __be32,
    pub r1_lo_length: __be32,
}

pub const CPL_RX_MPS_PKT_OP_S: c_int = 24;
pub const CPL_RX_MPS_PKT_OP_M: c_uint = 0xff;

pub const CPL_RX_MPS_PKT_TYPE_S: c_int = 20;
pub const CPL_RX_MPS_PKT_TYPE_M: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_srq_table_req {
    pub ot: opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 2],
    pub idx: __u8,
    pub rsvd_pdid: __be64,
    pub qlen_qbase: __be32,
    pub cur_msn: __be16,
    pub max_msn: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_srq_table_rpl {
    pub ot: opcode_tid,
    pub status: __u8,
    pub rsvd: [__u8; 2],
    pub idx: __u8,
    pub rsvd_pdid: __be64,
    pub qlen_qbase: __be32,
    pub cur_msn: __be16,
    pub max_msn: __be16,
}

// cpl_srq_table_{req,rpl}.params fields
pub const SRQT_QLEN_S: c_int = 28;
pub const SRQT_QLEN_M: c_uint = 0xF;

pub const SRQT_QBASE_S: c_int = 0;
pub const SRQT_QBASE_M: c_uint = 0x3FFFFFF;

pub const SRQT_PDID_S: c_int = 0;
pub const SRQT_PDID_M: c_uint = 0xFF;

pub const SRQT_IDX_S: c_int = 0;
pub const SRQT_IDX_M: c_uint = 0xF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_tls_sfo {
    pub op_to_seg_len: __be32,
    pub pld_len: __be32,
    pub type_protover: __be32,
    pub r1_lo: __be32,
    pub seqno_numivs: __be32,
    pub ivgen_hdrlen: __be32,
    pub scmd1: __be64,
}

// cpl_tx_tls_sfo macros
pub const CPL_TX_TLS_SFO_OPCODE_S: c_int = 24;

pub const CPL_TX_TLS_SFO_DATA_TYPE_S: c_int = 20;

pub const CPL_TX_TLS_SFO_CPL_LEN_S: c_int = 16;

pub const CPL_TX_TLS_SFO_SEG_LEN_S: c_int = 0;
pub const CPL_TX_TLS_SFO_SEG_LEN_M: c_uint = 0xffff;

pub const CPL_TX_TLS_SFO_TYPE_S: c_int = 24;
pub const CPL_TX_TLS_SFO_TYPE_M: c_uint = 0xff;

pub const CPL_TX_TLS_SFO_PROTOVER_S: c_int = 8;
pub const CPL_TX_TLS_SFO_PROTOVER_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tls_data {
    pub rsshdr: rss_header,
    pub ot: opcode_tid,
    pub length_pkd: __be32,
    pub seq: __be32,
    pub r1: __be32,
}

pub const CPL_TLS_DATA_OPCODE_S: c_int = 24;
pub const CPL_TLS_DATA_OPCODE_M: c_uint = 0xff;

pub const CPL_TLS_DATA_TID_S: c_int = 0;
pub const CPL_TLS_DATA_TID_M: c_uint = 0xffffff;

pub const CPL_TLS_DATA_LENGTH_S: c_int = 0;
pub const CPL_TLS_DATA_LENGTH_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_tls_cmp {
    pub rsshdr: rss_header,
    pub ot: opcode_tid,
    pub pdulength_length: __be32,
    pub seq: __be32,
    pub ddp_report: __be32,
    pub r: __be32,
    pub ddp_valid: __be32,
}

pub const CPL_RX_TLS_CMP_OPCODE_S: c_int = 24;
pub const CPL_RX_TLS_CMP_OPCODE_M: c_uint = 0xff;

pub const CPL_RX_TLS_CMP_TID_S: c_int = 0;
pub const CPL_RX_TLS_CMP_TID_M: c_uint = 0xffffff;

pub const CPL_RX_TLS_CMP_PDULENGTH_S: c_int = 16;
pub const CPL_RX_TLS_CMP_PDULENGTH_M: c_uint = 0xffff;

pub const CPL_RX_TLS_CMP_LENGTH_S: c_int = 0;
pub const CPL_RX_TLS_CMP_LENGTH_M: c_uint = 0xffff;

