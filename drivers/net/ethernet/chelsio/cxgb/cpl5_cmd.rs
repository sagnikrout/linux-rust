//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/cpl5_cmd.h
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
// File: cpl5_cmd.h
// $Revision: 1.6 $
// $Date: 2005/06/21 18:29:47 $
// Description:
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Copyright (c) 2003 - 2005 Chelsio Communications, Inc.
// All rights reserved.
//
// Maintainers: maintainers@chelsio.com
//
// Authors: Dimitrios Michailidis   <dm@chelsio.com>
// Tina Yang               <tainay@chelsio.com>
// Felix Marti             <felix@chelsio.com>
// Scott Bardone           <sbardone@chelsio.com>
// Kurt Ottaway            <kottaway@chelsio.com>
// Frank DiMambro          <frank@chelsio.com>
//
// History:
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CPL_opcode {
    CPL_PASS_OPEN_REQ     = 0x1,
    CPL_PASS_OPEN_RPL     = 0x2,
    CPL_PASS_ESTABLISH    = 0x3,
    CPL_PASS_ACCEPT_REQ   = 0xE,
    CPL_PASS_ACCEPT_RPL   = 0x4,
    CPL_ACT_OPEN_REQ      = 0x5,
    CPL_ACT_OPEN_RPL      = 0x6,
    CPL_CLOSE_CON_REQ     = 0x7,
    CPL_CLOSE_CON_RPL     = 0x8,
    CPL_CLOSE_LISTSRV_REQ = 0x9,
    CPL_CLOSE_LISTSRV_RPL = 0xA,
    CPL_ABORT_REQ         = 0xB,
    CPL_ABORT_RPL         = 0xC,
    CPL_PEER_CLOSE        = 0xD,
    CPL_ACT_ESTABLISH     = 0x17,

    CPL_GET_TCB           = 0x24,
    CPL_GET_TCB_RPL       = 0x25,
    CPL_SET_TCB           = 0x26,
    CPL_SET_TCB_FIELD     = 0x27,
    CPL_SET_TCB_RPL       = 0x28,
    CPL_PCMD              = 0x29,

    CPL_PCMD_READ         = 0x31,
    CPL_PCMD_READ_RPL     = 0x32,


    CPL_RX_DATA           = 0xA0,
    CPL_RX_DATA_DDP       = 0xA1,
    CPL_RX_DATA_ACK       = 0xA3,
    CPL_RX_PKT            = 0xAD,
    CPL_RX_ISCSI_HDR      = 0xAF,
    CPL_TX_DATA_ACK       = 0xB0,
    CPL_TX_DATA           = 0xB1,
    CPL_TX_PKT            = 0xB2,
    CPL_TX_PKT_LSO        = 0xB6,

    CPL_RTE_DELETE_REQ    = 0xC0,
    CPL_RTE_DELETE_RPL    = 0xC1,
    CPL_RTE_WRITE_REQ     = 0xC2,
    CPL_RTE_WRITE_RPL     = 0xD3,
    CPL_RTE_READ_REQ      = 0xC3,
    CPL_RTE_READ_RPL      = 0xC4,
    CPL_L2T_WRITE_REQ     = 0xC5,
    CPL_L2T_WRITE_RPL     = 0xD4,
    CPL_L2T_READ_REQ      = 0xC6,
    CPL_L2T_READ_RPL      = 0xC7,
    CPL_SMT_WRITE_REQ     = 0xC8,
    CPL_SMT_WRITE_RPL     = 0xD5,
    CPL_SMT_READ_REQ      = 0xC9,
    CPL_SMT_READ_RPL      = 0xCA,
    CPL_ARP_MISS_REQ      = 0xCD,
    CPL_ARP_MISS_RPL      = 0xCE,
    CPL_MIGRATE_C2T_REQ   = 0xDC,
    CPL_MIGRATE_C2T_RPL   = 0xDD,
    CPL_ERROR             = 0xD7,

// internal: driver -> TOM
    CPL_MSS_CHANGE        = 0xE1
}

pub const NUM_CPL_CMDS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CPL_error {
    CPL_ERR_NONE               = 0,
    CPL_ERR_TCAM_PARITY        = 1,
    CPL_ERR_TCAM_FULL          = 3,
    CPL_ERR_CONN_RESET         = 20,
    CPL_ERR_CONN_EXIST         = 22,
    CPL_ERR_ARP_MISS           = 23,
    CPL_ERR_BAD_SYN            = 24,
    CPL_ERR_CONN_TIMEDOUT      = 30,
    CPL_ERR_XMIT_TIMEDOUT      = 31,
    CPL_ERR_PERSIST_TIMEDOUT   = 32,
    CPL_ERR_FINWAIT2_TIMEDOUT  = 33,
    CPL_ERR_KEEPALIVE_TIMEDOUT = 34,
    CPL_ERR_ABORT_FAILED       = 42,
    CPL_ERR_GENERAL            = 99
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union opcode_tid {
    pub opcode_tid: u32,
    pub opcode: u8,
}

pub const S_OPCODE: c_int = 24;

// tid is assumed to be 24-bits

// extract the TID from a CPL command

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_options {
    pub mss: u16,
    pub wsf: u8,

    pub rsvd:4: u8,
    pub ecn:1: u8,
    pub sack:1: u8,
    pub tstamp:1: u8,

    pub tstamp:1: u8,
    pub sack:1: u8,
    pub ecn:1: u8,
    pub rsvd:4: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_req {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub opt0h: u32,
    pub opt0l: u32,
    pub peer_netmask: u32,
    pub opt1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_open_rpl {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub resvd: [u8; 7],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_establish {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub tos_tid: u32,
    pub l2t_idx: u8,
    pub rsvd: [u8; 3],
    pub snd_isn: u32,
    pub rcv_isn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_req {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub tos_tid: u32,
    pub tcp_options: tcp_options,
    pub dst_mac: [u8; 6],
    pub vlan_tag: u16,
    pub src_mac: [u8; 6],
    pub rsvd: [u8; 2],
    pub rcv_isn: u32,
    pub unknown_tcp_options: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pass_accept_rpl {
    pub ot: opcode_tid,
    pub rsvd0: u32,
    pub rsvd1: u32,
    pub peer_ip: u32,
    pub opt0h: u32,
    pub opt0l: u32,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_req {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub opt0h: u32,
    pub opt0l: u32,
    pub iff_vlantag: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_open_rpl {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub new_tid: u32,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_act_establish {
    pub ot: opcode_tid,
    pub local_port: u16,
    pub peer_port: u16,
    pub local_ip: u32,
    pub peer_ip: u32,
    pub tos_tid: u32,
    pub rsvd: u32,
    pub snd_isn: u32,
    pub rcv_isn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb {
    pub ot: opcode_tid,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_get_tcb_rpl {
    pub ot: opcode_tid,
    pub len: u16,
    pub rsvd: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb {
    pub ot: opcode_tid,
    pub len: u16,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_field {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub offset: u8,
    pub mask: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_set_tcb_rpl {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pcmd {
    pub ot: opcode_tid,
    pub dlen_in: u16,
    pub dlen_out: u16,
    pub pcmd_parm: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pcmd_read {
    pub ot: opcode_tid,
    pub rsvd1: u32,
    pub rsvd2: u16,
    pub addr: u32,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_pcmd_read_rpl {
    pub ot: opcode_tid,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_con_req {
    pub ot: opcode_tid,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_con_rpl {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
    pub snd_nxt: u32,
    pub rcv_nxt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listserv_req {
    pub ot: opcode_tid,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_close_listserv_rpl {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_req {
    pub ot: opcode_tid,
    pub rsvd0: u32,
    pub rsvd1: u8,
    pub cmd: u8,
    pub rsvd2: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_abort_rpl {
    pub ot: opcode_tid,
    pub rsvd0: u32,
    pub rsvd1: u8,
    pub status: u8,
    pub rsvd2: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_peer_close {
    pub ot: opcode_tid,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data {
    pub ot: opcode_tid,
    pub len: u32,
    pub rsvd0: u32,
    pub urg: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_data_ack {
    pub ot: opcode_tid,
    pub ack_seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data {
    pub ot: opcode_tid,
    pub len: u32,
    pub seq: u32,
    pub urg: u16,
    pub rsvd: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ack {
    pub ot: opcode_tid,
    pub credit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ddp {
    pub ot: opcode_tid,
    pub len: u32,
    pub seq: u32,
    pub nxt_seq: u32,
    pub ulp_crc: u32,
    pub ddp_status: u16,
    pub rsvd: u8,
    pub status: u8,
}

//
// We want this header's alignment to be no more stringent than 2-byte aligned.
// All fields are u8 or u16 except for the length.  However that field is not
// used so we break it into 2 16-bit parts to easily meet our alignment needs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt {
    pub opcode: u8,

    pub iff:4: u8,
    pub ip_csum_dis:1: u8,
    pub l4_csum_dis:1: u8,
    pub vlan_valid:1: u8,
    pub rsvd:1: u8,

    pub rsvd:1: u8,
    pub vlan_valid:1: u8,
    pub l4_csum_dis:1: u8,
    pub ip_csum_dis:1: u8,
    pub iff:4: u8,

    pub vlan: u16,
    pub len_hi: u16,
    pub len_lo: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_tx_pkt_lso {
    pub opcode: u8,

    pub iff:4: u8,
    pub ip_csum_dis:1: u8,
    pub l4_csum_dis:1: u8,
    pub vlan_valid:1: u8,
    pub :1: u8,

    pub :1: u8,
    pub vlan_valid:1: u8,
    pub l4_csum_dis:1: u8,
    pub ip_csum_dis:1: u8,
    pub iff:4: u8,

    pub vlan: u16,
    pub len: __be32,
    pub rsvd: [u8; 5],
    pub tcp_hdr_words:4: u8,
    pub ip_hdr_words:4: u8,

    pub ip_hdr_words:4: u8,
    pub tcp_hdr_words:4: u8,

    pub eth_type_mss: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_pkt {
    pub opcode: u8,

    pub iff:4: u8,
    pub csum_valid:1: u8,
    pub bad_pkt:1: u8,
    pub vlan_valid:1: u8,
    pub rsvd:1: u8,

    pub rsvd:1: u8,
    pub vlan_valid:1: u8,
    pub bad_pkt:1: u8,
    pub csum_valid:1: u8,
    pub iff:4: u8,

    pub csum: u16,
    pub vlan: u16,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_req {
    pub ot: opcode_tid,
    pub params: u32,
    pub rsvd1: [u8; 2],
    pub dst_mac: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_write_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_read_req {
    pub ot: opcode_tid,
    pub rsvd: [u8; 3],
    pub l2t_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_l2t_read_rpl {
    pub ot: opcode_tid,
    pub params: u32,
    pub rsvd1: [u8; 2],
    pub dst_mac: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_req {
    pub ot: opcode_tid,
    pub rsvd0: u8,

    pub rsvd1:1: u8,
    pub mtu_idx:3: u8,
    pub iff:4: u8,

    pub iff:4: u8,
    pub mtu_idx:3: u8,
    pub rsvd1:1: u8,

    pub rsvd2: u16,
    pub rsvd3: u16,
    pub src_mac1: [u8; 6],
    pub rsvd4: u16,
    pub src_mac0: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_write_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_read_req {
    pub ot: opcode_tid,
    pub rsvd0: u8,

    pub rsvd1:4: u8,
    pub iff:4: u8,

    pub iff:4: u8,
    pub rsvd1:4: u8,

    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_smt_read_rpl {
    pub ot: opcode_tid,
    pub status: u8,

    pub rsvd1:1: u8,
    pub mtu_idx:3: u8,
    pub rsvd0:4: u8,

    pub rsvd0:4: u8,
    pub mtu_idx:3: u8,
    pub rsvd1:1: u8,

    pub rsvd2: u16,
    pub rsvd3: u16,
    pub src_mac1: [u8; 6],
    pub rsvd4: u16,
    pub src_mac0: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_delete_req {
    pub ot: opcode_tid,
    pub params: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_delete_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_write_req {
    pub ot: opcode_tid,
    pub params: u32,
    pub netmask: u32,
    pub faddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_write_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_read_req {
    pub ot: opcode_tid,
    pub params: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rte_read_rpl {
    pub ot: opcode_tid,
    pub status: u8,
    pub rsvd0: [u8; 2],
    pub l2t_idx: u8,

    pub rsvd1:7: u8,
    pub select:1: u8,

    pub select:1: u8,
    pub rsvd1:7: u8,
    pub rsvd2: [u8; 3],
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_mss_change {
    pub ot: opcode_tid,
    pub mss: u32,
}
