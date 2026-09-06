//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_s_ev.c
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
// llc_s_ev.c - Defines SAP component events
//
// The followed event functions are SAP component events which are described
// in 802.2 LLC protocol standard document.
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_activation_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_activation_req(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    return ev.type == LLC_SAP_EV_TYPE_SIMPLE &&
    ev.prim_type == LLC_SAP_EV_ACTIVATION_REQ ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_rx_ui(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_rx_ui(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return ev.type == LLC_SAP_EV_TYPE_PDU && LLC_PDU_IS_CMD(pdu) &&
    LLC_PDU_TYPE_IS_U(pdu) &&
    LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_UI ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_unitdata_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_unitdata_req(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    return ev.type == LLC_SAP_EV_TYPE_PRIM &&
    ev.prim == LLC_DATAUNIT_PRIM &&
    ev.prim_type == LLC_PRIM_TYPE_REQ ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_xid_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_xid_req(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    return ev.type == LLC_SAP_EV_TYPE_PRIM &&
    ev.prim == LLC_XID_PRIM &&
    ev.prim_type == LLC_PRIM_TYPE_REQ ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_rx_xid_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_rx_xid_c(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return ev.type == LLC_SAP_EV_TYPE_PDU && LLC_PDU_IS_CMD(pdu) &&
    LLC_PDU_TYPE_IS_U(pdu) &&
    LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_XID ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_rx_xid_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_rx_xid_r(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return ev.type == LLC_SAP_EV_TYPE_PDU && LLC_PDU_IS_RSP(pdu) &&
    LLC_PDU_TYPE_IS_U(pdu) &&
    LLC_U_PDU_RSP(pdu) == LLC_1_PDU_CMD_XID ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_test_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_test_req(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    return ev.type == LLC_SAP_EV_TYPE_PRIM &&
    ev.prim == LLC_TEST_PRIM &&
    ev.prim_type == LLC_PRIM_TYPE_REQ ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_rx_test_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_rx_test_c(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return ev.type == LLC_SAP_EV_TYPE_PDU && LLC_PDU_IS_CMD(pdu) &&
    LLC_PDU_TYPE_IS_U(pdu) &&
    LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_TEST ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_rx_test_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_rx_test_r(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return ev.type == LLC_SAP_EV_TYPE_PDU && LLC_PDU_IS_RSP(pdu) &&
    LLC_PDU_TYPE_IS_U(pdu) &&
    LLC_U_PDU_RSP(pdu) == LLC_1_PDU_CMD_TEST ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_sap_ev_deactivation_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int {
    int llc_sap_ev_deactivation_req(struct llc_sap *sap, struct sk_buff *skb)
    {
    struct llc_sap_state_ev *ev = llc_sap_ev(skb);
    return ev.type == LLC_SAP_EV_TYPE_SIMPLE &&
    ev.prim_type == LLC_SAP_EV_DEACTIVATION_REQ ? 0 : 1;
    }
