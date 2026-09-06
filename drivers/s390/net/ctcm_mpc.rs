//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/ctcm_mpc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright IBM Corp. 2007
// Authors:	Peter Tiedemann (ptiedem@de.ibm.com)
//
// MPC additions:
// Belinda Thompson (belindat@us.ibm.com)
// Andy Richter (richtera@us.ibm.com)
//

//
// MPC external interface
// Note that ctc_mpc_xyz are called with a lock on ................
//
// port_number is the mpc device 0, 1, 2 etc mpc2 is port_number 2
// passive open  Just wait for XID2 exchange
// active open  Alloc then send XID2
extern "C" {
    pub fn ctc_mpc_dealloc_ch(port: c_int);
}
extern "C" {
    pub fn ctc_mpc_flow_control(port: c_int, flowc: c_int);
}
//
// other MPC Group prototypes and structures
//
pub const ETH_P_SNA_DIX: c_uint = 0x80D5;
//
// Declaration of an XID2
//
pub const ALLZEROS: c_uint = 0x0000000000000000;
pub const XID_FM2: c_uint = 0x20;
pub const XID2_0: c_uint = 0x00;
pub const XID2_7: c_uint = 0x07;
pub const XID2_WRITE_SIDE: c_uint = 0x04;
pub const XID2_READ_SIDE: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xid2 {
    pub xid2_type_id: __u8,
    pub xid2_len: __u8,
    pub xid2_adj_id: __u32,
    pub xid2_rlen: __u8,
    pub xid2_resv1: __u8,
    pub xid2_flag1: __u8,
    pub xid2_fmtt: __u8,
    pub xid2_flag4: __u8,
    pub xid2_resv2: __u16,
    pub xid2_tgnum: __u8,
    pub xid2_sender_id: __u32,
    pub xid2_flag2: __u8,
    pub xid2_option: __u8,
    pub xid2_resv3: [c_char; 8],
    pub xid2_resv4: __u16,
    pub xid2_dlc_type: __u8,
    pub xid2_resv5: __u16,
    pub xid2_mpc_flag: __u8,
    pub xid2_resv6: __u8,
    pub xid2_buf_len: __u16,
    pub sizeof(char))]: *mut *mut 8,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct th_header {
    pub th_seg: __u8,
    pub th_ch_flag: __u8,
pub const TH_HAS_PDU: c_uint = 0xf0;
pub const TH_IS_XID: c_uint = 0x01;
pub const TH_SWEEP_REQ: c_uint = 0xfe;
pub const TH_SWEEP_RESP: c_uint = 0xff;
    pub th_blk_flag: __u8,
pub const TH_DATA_IS_XID: c_uint = 0x80;
pub const TH_RETRY: c_uint = 0x40;
pub const TH_DISCONTACT: c_uint = 0xc0;
pub const TH_SEG_BLK: c_uint = 0x20;
pub const TH_LAST_SEG: c_uint = 0x10;
pub const TH_PDU_PART: c_uint = 0x08;
    pub /: *mut *mut __u8 th_is_xid; / is 0x01 if this is XID,
    pub th_seq_num: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th_addon {
    pub th_last_seq: __u32,
    pub th_resvd: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th_sweep {
    pub th: th_header,
    pub sw: th_addon,
// C attribute field omitted

pub const PDU_LAST: c_uint = 0x80;
pub const PDU_CNTL: c_uint = 0x40;
pub const PDU_FIRST: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdu {
    pub pdu_offset: __u32,
    pub pdu_flag: __u8,
    pub /: *mut *mut __u8 pdu_proto; / 0x01 is APPN SNA,
    pub pdu_seq: __u16,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qllc {
    pub qllc_address: __u8,
pub const QLLC_REQ: c_uint = 0xFF;
pub const QLLC_RESP: c_uint = 0x00;
    pub qllc_commands: __u8,
pub const QLLC_DISCONNECT: c_uint = 0x53;
pub const QLLC_UNSEQACK: c_uint = 0x73;
pub const QLLC_SETMODE: c_uint = 0x93;
pub const QLLC_EXCHID: c_uint = 0xBF;
// C attribute field omitted
//
// Definition of one MPC group
//
pub const MAX_MPCGCHAN: c_int = 10;
pub const MPC_XID_TIMEOUT_VALUE: c_int = 10000;
pub const MPC_CHANNEL_ADD: c_int = 0;
pub const MPC_CHANNEL_REMOVE: c_int = 1;
pub const MPC_CHANNEL_ATTN: c_int = 2;
pub const XSIDE: c_int = 1;
pub const YSIDE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpcg_info {
    pub skb: *mut sk_buff,
    pub ch: *mut channel,
    pub xid: *mut xid2,
    pub sweep: *mut th_sweep,
    pub th: *mut th_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_group {
    pub mpc_tasklet: tasklet_struct,
    pub mpc_tasklet2: tasklet_struct,
    pub changed_side: c_int,
    pub saved_state: c_int,
    pub channels_terminating: c_int,
    pub out_of_sequence: c_int,
    pub flow_off_called: c_int,
    pub port_num: c_int,
    pub port_persist: c_int,
    pub alloc_called: c_int,
    pub xid2_adj_id: __u32,
    pub xid2_tgnum: __u8,
    pub xid2_sender_id: __u32,
    pub num_channel_paths: c_int,
    pub active_channels: [c_int; 2],
    pub group_max_buflen: __u16,
    pub outstanding_xid2: c_int,
    pub outstanding_xid7: c_int,
    pub outstanding_xid7_p2: c_int,
    pub sweep_req_pend_num: c_int,
    pub sweep_rsp_pend_num: c_int,
    pub xid_skb: *mut sk_buff,
    pub xid_skb_data: *mut c_char,
    pub xid_th: *mut th_header,
    pub xid: *mut xid2,
    pub xid_id: *mut c_char,
    pub rcvd_xid_th: *mut th_header,
    pub rcvd_xid_skb: *mut sk_buff,
    pub rcvd_xid_data: *mut c_char,
    pub in_sweep: __u8,
    pub roll: __u8,
    pub saved_xid2: *mut xid2,
    pub int): *mut *mut void (allochanfunc)(int,,
    pub allocchan_callback_retries: c_int,
    pub int): *mut *mut void (estconnfunc)(int, int,,
    pub estconn_callback_retries: c_int,
    pub estconn_called: c_int,
    pub xidnogood: c_int,
    pub send_qllc_disc: c_int,
    pub timer: fsm_timer,
    pub /: *mut *mut *mut fsm_instance fsm; / group xid fsm,
}

extern "C" {
    pub fn ctcmpc_dumpit(buf: *mut c_char, len: c_int);
}

//
// Dump header and first 16 bytes of an sk_buff for debugging purposes.
//
// skb	 The struct sk_buff to dump.
// offset Offset relative to skb-data, where to start the dump.
//
extern "C" {
    pub fn ctcmpc_dump_skb(skb: *mut sk_buff, offset: c_int);
}

extern "C" {
    pub fn ctcm_ccw_check_rc(: *mut channel, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn mpc_group_ready(adev: c_ulong);
}
extern "C" {
    pub fn mpc_channel_action(ch: *mut channel, direction: c_int, action: c_int);
}
extern "C" {
    pub fn mpc_action_send_discontact(thischan: c_ulong);
}
extern "C" {
    pub fn mpc_action_discontact(fi: *mut fsm_instance, event: c_int, arg: *mut c_void);
}
extern "C" {
    pub fn ctcmpc_bh(thischan: c_ulong);
}

// --- This is the END my friend ---
