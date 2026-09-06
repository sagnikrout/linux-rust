//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_pdu.c
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
// llc_pdu.c - access to PDU internals
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

    static void llc_pdu_decode_pdu_type(struct sk_buff *skb, u8 *type);
    static u8 llc_pdu_get_pf_bit(struct llc_pdu_sn *pdu);
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_set_cmd_rsp(skb: *mut sk_buff, pdu_type: u8) {
    void llc_pdu_set_cmd_rsp(struct sk_buff *skb, u8 pdu_type)
    {
    llc_pdu_un_hdr(skb).ssap |= pdu_type;
    }
//
// llc_pdu_set_pf_bit - sets poll/final bit in LLC header
// @skb: Frame to set bit in
// @bit_value: poll/final bit (0 or 1).
//
// This function sets poll/final bit in LLC header (based on type of PDU).
// in I or S pdus, p/f bit is right bit of fourth byte in header. in U
// pdus p/f bit is fifth bit of third byte.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_set_pf_bit(skb: *mut sk_buff, bit_value: u8) {
    void llc_pdu_set_pf_bit(struct sk_buff *skb, u8 bit_value)
    {
    u8 pdu_type;
    struct llc_pdu_sn *pdu;
    llc_pdu_decode_pdu_type(skb, &pdu_type);
    pdu = llc_pdu_sn_hdr(skb);
    switch (pdu_type) {
    case LLC_PDU_TYPE_I:
    case LLC_PDU_TYPE_S:
    pdu.ctrl_2 = (pdu.ctrl_2 & 0xFE) | bit_value;
    break;
    case LLC_PDU_TYPE_U:
    pdu.ctrl_1 |= (pdu.ctrl_1 & 0xEF) | (bit_value << 4);
    break;
    }
    }
//
// llc_pdu_decode_pf_bit - extracs poll/final bit from LLC header
// @skb: input skb that p/f bit must be extracted from it
// @pf_bit: poll/final bit (0 or 1)
//
// This function extracts poll/final bit from LLC header (based on type of
// PDU). In I or S pdus, p/f bit is right bit of fourth byte in header. In
// U pdus p/f bit is fifth bit of third byte.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_decode_pf_bit(skb: *mut sk_buff, pf_bit: *mut u8) {
    void llc_pdu_decode_pf_bit(struct sk_buff *skb, u8 *pf_bit)
    {
    u8 pdu_type;
    struct llc_pdu_sn *pdu;
    llc_pdu_decode_pdu_type(skb, &pdu_type);
    pdu = llc_pdu_sn_hdr(skb);
    switch (pdu_type) {
    case LLC_PDU_TYPE_I:
    case LLC_PDU_TYPE_S:
// pf_bit = pdu->ctrl_2 & LLC_S_PF_BIT_MASK;
    break;
    case LLC_PDU_TYPE_U:
// pf_bit = (pdu->ctrl_1 & LLC_U_PF_BIT_MASK) >> 4;
    break;
    }
    }
//
// llc_pdu_init_as_disc_cmd - Builds DISC PDU
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
//
// Builds a pdu frame as a DISC command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_disc_cmd(skb: *mut sk_buff, p_bit: u8) {
    void llc_pdu_init_as_disc_cmd(struct sk_buff *skb, u8 p_bit)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_U;
    pdu.ctrl_1 |= LLC_2_PDU_CMD_DISC;
    pdu.ctrl_1 |= ((p_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
    }
//
// llc_pdu_init_as_i_cmd - builds I pdu
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
// @ns: The sequence number of the data PDU
// @nr: The seq. number of the expected I PDU from the remote
//
// Builds a pdu frame as an I command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_i_cmd(skb: *mut sk_buff, p_bit: u8, ns: u8, nr: u8) {
    void llc_pdu_init_as_i_cmd(struct sk_buff *skb, u8 p_bit, u8 ns, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_I;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= (p_bit & LLC_I_PF_BIT_MASK); /* p/f bit */
    pdu.ctrl_1 |= (ns << 1) & 0xFE;   /* set N(S) in bits 2..8 */
    pdu.ctrl_2 |= (nr << 1) & 0xFE;   /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_rej_cmd - builds REJ PDU
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
// @nr: The seq. number of the expected I PDU from the remote
//
// Builds a pdu frame as a REJ command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rej_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    void llc_pdu_init_as_rej_cmd(struct sk_buff *skb, u8 p_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_CMD_REJ;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= p_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE; /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_rnr_cmd - builds RNR pdu
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
// @nr: The seq. number of the expected I PDU from the remote
//
// Builds a pdu frame as an RNR command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rnr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    void llc_pdu_init_as_rnr_cmd(struct sk_buff *skb, u8 p_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_CMD_RNR;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= p_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE; /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_rr_cmd - Builds RR pdu
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
// @nr: The seq. number of the expected I PDU from the remote
//
// Builds a pdu frame as an RR command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    void llc_pdu_init_as_rr_cmd(struct sk_buff *skb, u8 p_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_CMD_RR;
    pdu.ctrl_2  = p_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE; /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_sabme_cmd - builds SABME pdu
// @skb: Address of the skb to build
// @p_bit: The P bit to set in the PDU
//
// Builds a pdu frame as an SABME command.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_sabme_cmd(skb: *mut sk_buff, p_bit: u8) {
    void llc_pdu_init_as_sabme_cmd(struct sk_buff *skb, u8 p_bit)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_U;
    pdu.ctrl_1 |= LLC_2_PDU_CMD_SABME;
    pdu.ctrl_1 |= ((p_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
    }
//
// llc_pdu_init_as_dm_rsp - builds DM response pdu
// @skb: Address of the skb to build
// @f_bit: The F bit to set in the PDU
//
// Builds a pdu frame as a DM response.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_dm_rsp(skb: *mut sk_buff, f_bit: u8) {
    void llc_pdu_init_as_dm_rsp(struct sk_buff *skb, u8 f_bit)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_U;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_DM;
    pdu.ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
    }
//
// llc_pdu_init_as_frmr_rsp - builds FRMR response PDU
// @skb: Address of the frame to build
// @prev_pdu: The rejected PDU frame
// @f_bit: The F bit to set in the PDU
// @vs: tx state vari value for the data link conn at the rejecting LLC
// @vr: rx state var value for the data link conn at the rejecting LLC
// @vzyxw: completely described in the IEEE Std 802.2 document (Pg 55)
//
// Builds a pdu frame as a FRMR response.
//
    void llc_pdu_init_as_frmr_rsp(struct sk_buff *skb, struct llc_pdu_sn *prev_pdu,
    u8 f_bit, u8 vs, u8 vr, u8 vzyxw)
    {
    struct llc_frmr_info *frmr_info;
    let mut prev_pf: u8 = 0;
    u8 *ctrl;
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_U;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_FRMR;
    pdu.ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
    frmr_info = (struct llc_frmr_info *)&pdu.ctrl_2;
    ctrl = (u8 *)&prev_pdu.ctrl_1;
    FRMR_INFO_SET_REJ_CNTRL(frmr_info,ctrl);
    FRMR_INFO_SET_Vs(frmr_info, vs);
    FRMR_INFO_SET_Vr(frmr_info, vr);
    prev_pf = llc_pdu_get_pf_bit(prev_pdu);
    FRMR_INFO_SET_C_R_BIT(frmr_info, prev_pf);
    FRMR_INFO_SET_INVALID_PDU_CTRL_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_INVALID_PDU_INFO_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INFO_2LONG_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INVALID_Nr_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INVALID_Ns_IND(frmr_info, vzyxw);
    skb_put(skb, sizeof(struct llc_frmr_info));
    }
//
// llc_pdu_init_as_rr_rsp - builds RR response pdu
// @skb: Address of the skb to build
// @f_bit: The F bit to set in the PDU
// @nr: The seq. number of the expected data PDU from the remote
//
// Builds a pdu frame as an RR response.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    void llc_pdu_init_as_rr_rsp(struct sk_buff *skb, u8 f_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_RR;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE;  /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_rej_rsp - builds REJ response pdu
// @skb: Address of the skb to build
// @f_bit: The F bit to set in the PDU
// @nr: The seq. number of the expected data PDU from the remote
//
// Builds a pdu frame as a REJ response.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rej_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    void llc_pdu_init_as_rej_rsp(struct sk_buff *skb, u8 f_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_REJ;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE;  /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_rnr_rsp - builds RNR response pdu
// @skb: Address of the frame to build
// @f_bit: The F bit to set in the PDU
// @nr: The seq. number of the expected data PDU from the remote
//
// Builds a pdu frame as an RNR response.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_rnr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    void llc_pdu_init_as_rnr_rsp(struct sk_buff *skb, u8 f_bit, u8 nr)
    {
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_S;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_RNR;
    pdu.ctrl_2  = 0;
    pdu.ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    pdu.ctrl_1 &= 0x0F;    /* setting bits 5..8 to zero(reserved) */
    pdu.ctrl_2 |= (nr << 1) & 0xFE;  /* set N(R) in bits 10..16 */
    }
//
// llc_pdu_init_as_ua_rsp - builds UA response pdu
// @skb: Address of the frame to build
// @f_bit: The F bit to set in the PDU
//
// Builds a pdu frame as a UA response.
//
#[no_mangle]
pub unsafe extern "C" fn llc_pdu_init_as_ua_rsp(skb: *mut sk_buff, f_bit: u8) {
    void llc_pdu_init_as_ua_rsp(struct sk_buff *skb, u8 f_bit)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    pdu.ctrl_1  = LLC_PDU_TYPE_U;
    pdu.ctrl_1 |= LLC_2_PDU_RSP_UA;
    pdu.ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
    }
//
// llc_pdu_decode_pdu_type - designates PDU type
// @skb: input skb that type of it must be designated.
// @type: type of PDU (output argument).
//
// This function designates type of PDU (I, S or U).
//
#[no_mangle]
unsafe extern "C" fn llc_pdu_decode_pdu_type(skb: *mut sk_buff, type: *mut u8) {
    static void llc_pdu_decode_pdu_type(struct sk_buff *skb, u8 *type)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    if (pdu.ctrl_1 & 1) {
    if ((pdu.ctrl_1 & LLC_PDU_TYPE_U) == LLC_PDU_TYPE_U)
// type = LLC_PDU_TYPE_U;
    else
// type = LLC_PDU_TYPE_S;
    } else
// type = LLC_PDU_TYPE_I;
    }
//
// llc_pdu_get_pf_bit - extracts p/f bit of input PDU
// @pdu: pointer to LLC header.
//
// This function extracts p/f bit of input PDU. at first examines type of
// PDU and then extracts p/f bit. Returns the p/f bit.
//
#[no_mangle]
unsafe extern "C" fn llc_pdu_get_pf_bit(pdu: *mut llc_pdu_sn) -> u8 {
    static u8 llc_pdu_get_pf_bit(struct llc_pdu_sn *pdu)
    {
    u8 pdu_type;
    let mut pf_bit: u8 = 0;
    if (pdu.ctrl_1 & 1) {
    if ((pdu.ctrl_1 & LLC_PDU_TYPE_U) == LLC_PDU_TYPE_U)
    pdu_type = LLC_PDU_TYPE_U;
    else
    pdu_type = LLC_PDU_TYPE_S;
    } else
    pdu_type = LLC_PDU_TYPE_I;
    switch (pdu_type) {
    case LLC_PDU_TYPE_I:
    case LLC_PDU_TYPE_S:
    pf_bit = pdu.ctrl_2 & LLC_S_PF_BIT_MASK;
    break;
    case LLC_PDU_TYPE_U:
    pf_bit = (pdu.ctrl_1 & LLC_U_PF_BIT_MASK) >> 4;
    break;
    }
    return pf_bit;
    }
