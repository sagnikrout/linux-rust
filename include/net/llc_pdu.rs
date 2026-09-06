//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_pdu.h
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
// Copyright (c) 1997 by Procom Technology,Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

// Lengths of frame formats

pub const LLC_PDU_LEN_S: c_int = 4;

// header and 1 control byte and XID info

// Known SAP addresses
pub const LLC_GLOBAL_SAP: c_uint = 0xFF;
pub const LLC_NULL_SAP: c_uint = 0x00	/* not network-layer visible */;
pub const LLC_MGMT_INDIV: c_uint = 0x02	/* station LLC mgmt indiv addr */;
pub const LLC_MGMT_GRP: c_uint = 0x03	/* station LLC mgmt group addr */;
pub const LLC_RDE_SAP: c_uint = 0xA6	/* route ... */;
// SAP field bit masks
pub const LLC_ISO_RESERVED_SAP: c_uint = 0x02;
pub const LLC_SAP_GROUP_DSAP: c_uint = 0x01;
pub const LLC_SAP_RESP_SSAP: c_uint = 0x01;
// Group/individual DSAP indicator is DSAP field
pub const LLC_PDU_GROUP_DSAP_MASK: c_uint = 0x01;

// Command/response PDU indicator in SSAP field
pub const LLC_PDU_CMD_RSP_MASK: c_uint = 0x01;
pub const LLC_PDU_CMD: c_int = 0;
pub const LLC_PDU_RSP: c_int = 1;

// Get PDU type from 2 lowest-order bits of control field first byte
pub const LLC_PDU_TYPE_I_MASK: c_uint = 0x01	/* 16-bit control field */;
pub const LLC_PDU_TYPE_S_MASK: c_uint = 0x03;
pub const LLC_PDU_TYPE_U_MASK: c_uint = 0x03	/* 8-bit control field */;
pub const LLC_PDU_TYPE_MASK: c_uint = 0x03;

// U-format PDU control field masks
pub const LLC_U_PF_BIT_MASK: c_uint = 0x10	/* P/F bit mask */;

pub const LLC_U_PDU_CMD_MASK: c_uint = 0xEC	/* cmd/rsp mask */;

pub const LLC_1_PDU_CMD_UI: c_uint = 0x00	/* Type 1 cmds/rsps */;
pub const LLC_1_PDU_CMD_XID: c_uint = 0xAC;
pub const LLC_1_PDU_CMD_TEST: c_uint = 0xE0;
pub const LLC_2_PDU_CMD_SABME: c_uint = 0x6C	/* Type 2 cmds/rsps */;
pub const LLC_2_PDU_CMD_DISC: c_uint = 0x40;
pub const LLC_2_PDU_RSP_UA: c_uint = 0x60;
pub const LLC_2_PDU_RSP_DM: c_uint = 0x0C;
pub const LLC_2_PDU_RSP_FRMR: c_uint = 0x84;
// Type 1 operations
// XID information field bit masks
// LLC format identifier (byte 1)
pub const LLC_XID_FMT_ID: c_uint = 0x81	/* first byte must be this */;
// LLC types/classes identifier (byte 2)
pub const LLC_XID_CLASS_ZEROS_MASK: c_uint = 0xE0	/* these must be zeros */;
pub const LLC_XID_CLASS_MASK: c_uint = 0x1F	/* AND with byte to get below */;
pub const LLC_XID_NULL_CLASS_1: c_uint = 0x01	/* if NULL LSAP...use these */;
pub const LLC_XID_NULL_CLASS_2: c_uint = 0x03;
pub const LLC_XID_NULL_CLASS_3: c_uint = 0x05;
pub const LLC_XID_NULL_CLASS_4: c_uint = 0x07;
pub const LLC_XID_NNULL_TYPE_1: c_uint = 0x01	/* if non-NULL LSAP...use these */;
pub const LLC_XID_NNULL_TYPE_2: c_uint = 0x02;
pub const LLC_XID_NNULL_TYPE_3: c_uint = 0x04;
pub const LLC_XID_NNULL_TYPE_1_2: c_uint = 0x03;
pub const LLC_XID_NNULL_TYPE_1_3: c_uint = 0x05;
pub const LLC_XID_NNULL_TYPE_2_3: c_uint = 0x06;
pub const LLC_XID_NNULL_ALL: c_uint = 0x07;
// Sender Receive Window (byte 3)
pub const LLC_XID_RW_MASK: c_uint = 0xFE	/* AND with value to get below */;
pub const LLC_XID_MIN_RW: c_uint = 0x02	/* lowest-order bit always zero */;
// Type 2 operations

// I-PDU masks ('ctrl' is I-PDU control word)

pub const LLC_I_PF_BIT_MASK: c_uint = 0x01;

// S-PDU supervisory commands and responses
pub const LLC_S_PDU_CMD_MASK: c_uint = 0x0C;

pub const LLC_2_PDU_CMD_RR: c_uint = 0x00	/* rx ready cmd */;
pub const LLC_2_PDU_RSP_RR: c_uint = 0x00	/* rx ready rsp */;
pub const LLC_2_PDU_CMD_REJ: c_uint = 0x08	/* reject PDU cmd */;
pub const LLC_2_PDU_RSP_REJ: c_uint = 0x08	/* reject PDU rsp */;
pub const LLC_2_PDU_CMD_RNR: c_uint = 0x04	/* rx not ready cmd */;
pub const LLC_2_PDU_RSP_RNR: c_uint = 0x04	/* rx not ready rsp */;
pub const LLC_S_PF_BIT_MASK: c_uint = 0x01;

// FRMR information field macros

//
// info is pointer to FRMR info field structure; 'rej_ctrl' is byte pointer
// (if U-PDU) or word pointer to rejected PDU control field
//

//
// Info is pointer to FRMR info field structure; 'vs' is a byte containing
// send state variable value in low-order 7 bits (insure the lowest-order
// bit remains zero (0))
//

//
// Info is pointer to FRMR info field structure; 'cr' is a byte containing
// the C/R bit value in the low-order bit
//

//
// In the remaining five macros, 'info' is pointer to FRMR info field
// structure; 'ind' is a byte containing the bit value to set in the
// lowest-order bit)
//

// Sequence-numbered PDU format (4 bytes in length)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_pdu_sn {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl_1: u8,
    pub ctrl_2: u8,
    pub __packed: },
    pub )skb_network_header(skb): *mut return (struct llc_pdu_sn,
// Un-numbered PDU format (3 bytes in length)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_pdu_un {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl_1: u8,
    pub __packed: },
    pub )skb_network_header(skb): *mut return (struct llc_pdu_un,
//
// llc_pdu_header_init - initializes pdu header
// @skb: input skb that header must be set into it.
// @type: type of PDU (U, I or S).
// @ssap: source sap.
// @dsap: destination sap.
// @cr: command/response bit (0 or 1).
//
// This function sets DSAP, SSAP and command/Response bit in LLC header.
//
    pub /: *mut *mut int hlen = 4; / default value for I and S types,
    pub pdu: *mut llc_pdu_un,
    pub 3: hlen =,
    pub 6: hlen =,
    pub hlen): skb_push(skb,,
    pub llc_pdu_un_hdr(skb): pdu =,
    pub dsap: pdu->dsap =,
    pub ssap: pdu->ssap =,
    pub cr: pdu->ssap |=,
//
// llc_pdu_decode_sa - extracts, source address (MAC) of input frame
// @skb: input skb that source address must be extracted from it.
// @sa: pointer to source address (6 byte array).
//
// This function extracts source address(MAC) of input frame.
//
    pub ETH_ALEN): memcpy(sa, eth_hdr(skb)->h_source,,
//
// llc_pdu_decode_da - extracts dest address of input frame
// @skb: input skb that destination address must be extracted from it
// @da: pointer to destination address (6 byte array).
//
// This function extracts destination address(MAC) of input frame.
//
    pub ETH_ALEN): memcpy(da, eth_hdr(skb)->h_dest,,
//
// llc_pdu_decode_ssap - extracts source SAP of input frame
// @skb: input skb that source SAP must be extracted from it.
// @ssap: source SAP (output argument).
//
// This function extracts source SAP of input frame. Right bit of SSAP is
// command/response bit.
//
// ssap = llc_pdu_un_hdr(skb)->ssap & 0xFE;
//
// llc_pdu_decode_dsap - extracts dest SAP of input frame
// @skb: input skb that destination SAP must be extracted from it.
// @dsap: destination SAP (output argument).
//
// This function extracts destination SAP of input frame. right bit of
// DSAP designates individual/group SAP.
//
// dsap = llc_pdu_un_hdr(skb)->dsap & 0xFE;
//
// llc_pdu_init_as_ui_cmd - sets LLC header as UI PDU
// @skb: input skb that header must be set into it.
//
// This function sets third byte of LLC header as a UI PDU.
//
    pub llc_pdu_un_hdr(skb): *mut *mut llc_pdu_un pdu =,
    pub LLC_PDU_TYPE_U: pdu->ctrl_1 =,
    pub LLC_1_PDU_CMD_UI: pdu->ctrl_1 |=,
//
// llc_pdu_init_as_test_cmd - sets PDU as TEST
// @skb: Address of the skb to build
//
// Sets a PDU as TEST
//
    pub llc_pdu_un_hdr(skb): *mut *mut llc_pdu_un pdu =,
    pub LLC_PDU_TYPE_U: pdu->ctrl_1 =,
    pub LLC_1_PDU_CMD_TEST: pdu->ctrl_1 |=,
    pub LLC_U_PF_BIT_MASK: pdu->ctrl_1 |=,
//
// llc_pdu_init_as_test_rsp - build TEST response PDU
// @skb: Address of the skb to build
// @ev_skb: The received TEST command PDU frame
//
// Builds a pdu frame as a TEST response.
//
    pub llc_pdu_un_hdr(skb): *mut *mut llc_pdu_un pdu =,
    pub LLC_PDU_TYPE_U: pdu->ctrl_1 =,
    pub LLC_1_PDU_CMD_TEST: pdu->ctrl_1 |=,
    pub LLC_U_PF_BIT_MASK: pdu->ctrl_1 |=,
    pub llc_pdu_un_hdr(ev_skb): *mut *mut llc_pdu_un ev_pdu =,
    pub dsize: c_int,
    pub 3: dsize = ntohs(eth_hdr(ev_skb)->h_proto) -,
    pub dsize): *mut *mut *mut memcpy(((u8 )pdu) + 3, ((u8 )ev_pdu) + 3,,
    pub dsize): skb_put(skb,,
// LLC Type 1 XID command/response information fields format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_xid_info {
    pub /: *mut *mut u8 fmt_id; / always 0x81 for LLC,
    pub /: *mut *mut u8 type; / different if NULL/non-NULL LSAP,
    pub /: *mut *mut u8 rw; / sender receive window,
    pub __packed: },
//
// llc_pdu_init_as_xid_cmd - sets bytes 3, 4 & 5 of LLC header as XID
// @skb: input skb that header must be set into it.
// @svcs_supported: The class of the LLC (I or II)
// @rx_window: The size of the receive window of the LLC
//
// This function sets third,fourth,fifth and sixth bytes of LLC header as
// a XID PDU.
//
    pub xid_info: *mut llc_xid_info,
    pub llc_pdu_un_hdr(skb): *mut *mut llc_pdu_un pdu =,
    pub LLC_PDU_TYPE_U: pdu->ctrl_1 =,
    pub LLC_1_PDU_CMD_XID: pdu->ctrl_1 |=,
    pub LLC_U_PF_BIT_MASK: pdu->ctrl_1 |=,
    pub 1): *mut *mut *mut xid_info = (struct llc_xid_info )(((u8 )&pdu->ctrl_1) +,
    pub /: *mut *mut xid_info->fmt_id = LLC_XID_FMT_ID; / 0x81,
    pub svcs_supported: xid_info->type =,
    pub /: *mut *mut xid_info->rw = rx_window << 1; / size of receive window,
// no need to push/put since llc_pdu_header_init() has already
// pushed 3 + 3 bytes
//
// llc_pdu_init_as_xid_rsp - builds XID response PDU
// @skb: Address of the skb to build
// @svcs_supported: The class of the LLC (I or II)
// @rx_window: The size of the receive window of the LLC
//
// Builds a pdu frame as an XID response.
//
    pub xid_info: *mut llc_xid_info,
    pub llc_pdu_un_hdr(skb): *mut *mut llc_pdu_un pdu =,
    pub LLC_PDU_TYPE_U: pdu->ctrl_1 =,
    pub LLC_1_PDU_CMD_XID: pdu->ctrl_1 |=,
    pub LLC_U_PF_BIT_MASK: pdu->ctrl_1 |=,
    pub 1): *mut *mut *mut xid_info = (struct llc_xid_info )(((u8 )&pdu->ctrl_1) +,
    pub LLC_XID_FMT_ID: xid_info->fmt_id =,
    pub svcs_supported: xid_info->type =,
    pub 1: xid_info->rw = rx_window <<,
    pub llc_xid_info)): skb_put(skb, sizeof(struct,
// LLC Type 2 FRMR response information field format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_frmr_info {
    pub /: *mut *mut u16 rej_pdu_ctrl; / bits 1-8 if U-PDU,
    pub /: *mut *mut u8 curr_ssv; / current send state variable val,
    pub /: *mut *mut u8 curr_rsv; / current receive state variable,
    pub /: *mut *mut u8 ind_bits; / indicator bits set with macro,
    pub __packed: },
    pub type): *mut *mut void llc_pdu_set_cmd_rsp(struct sk_buff skb, u8,
    pub bit_value): *mut *mut void llc_pdu_set_pf_bit(struct sk_buff skb, u8,
    pub pf_bit): *mut *mut void llc_pdu_decode_pf_bit(struct sk_buff skb, u8,
    pub p_bit): *mut *mut void llc_pdu_init_as_disc_cmd(struct sk_buff skb, u8,
    pub nr): *mut *mut void llc_pdu_init_as_i_cmd(struct sk_buff skb, u8 p_bit, u8 ns, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rej_cmd(struct sk_buff skb, u8 p_bit, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rnr_cmd(struct sk_buff skb, u8 p_bit, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rr_cmd(struct sk_buff skb, u8 p_bit, u8,
    pub p_bit): *mut *mut void llc_pdu_init_as_sabme_cmd(struct sk_buff skb, u8,
    pub f_bit): *mut *mut void llc_pdu_init_as_dm_rsp(struct sk_buff skb, u8,
    pub vzyxw): u8 f_bit, u8 vs, u8 vr, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rr_rsp(struct sk_buff skb, u8 f_bit, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rej_rsp(struct sk_buff skb, u8 f_bit, u8,
    pub nr): *mut *mut void llc_pdu_init_as_rnr_rsp(struct sk_buff skb, u8 f_bit, u8,
    pub f_bit): *mut *mut void llc_pdu_init_as_ua_rsp(struct sk_buff skb, u8,
