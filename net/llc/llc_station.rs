//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_station.c
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
// llc_station.c - station component of LLC
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

#[no_mangle]
unsafe extern "C" fn llc_stat_ev_rx_null_dsap_xid_c(skb: *mut sk_buff) -> c_int {
    static int llc_stat_ev_rx_null_dsap_xid_c(struct sk_buff *skb)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return LLC_PDU_IS_CMD(pdu) &&			/* command PDU */
    LLC_PDU_TYPE_IS_U(pdu) &&		/* U type PDU */
    LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_XID &&
    !pdu.dsap;				/* core::ptr::null_mut() DSAP value */
    }
#[no_mangle]
unsafe extern "C" fn llc_stat_ev_rx_null_dsap_test_c(skb: *mut sk_buff) -> c_int {
    static int llc_stat_ev_rx_null_dsap_test_c(struct sk_buff *skb)
    {
    struct llc_pdu_un *pdu = llc_pdu_un_hdr(skb);
    return LLC_PDU_IS_CMD(pdu) &&			/* command PDU */
    LLC_PDU_TYPE_IS_U(pdu) &&		/* U type PDU */
    LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_TEST &&
    !pdu.dsap;				/* core::ptr::null_mut() DSAP */
    }
#[no_mangle]
unsafe extern "C" fn llc_station_ac_send_xid_r(skb: *mut sk_buff) -> c_int {
    static int llc_station_ac_send_xid_r(struct sk_buff *skb)
    {
    u8 mac_da[ETH_ALEN], dsap;
    let mut rc: c_int = 1;
    struct sk_buff *nskb = llc_alloc_frame(core::ptr::null_mut(), skb.dev, LLC_PDU_TYPE_U,
    sizeof(struct llc_xid_info));
    if (!nskb)
    goto out;
    llc_pdu_decode_sa(skb, mac_da);
    llc_pdu_decode_ssap(skb, &dsap);
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, 0, dsap, LLC_PDU_RSP);
    llc_pdu_init_as_xid_rsp(nskb, LLC_XID_NULL_CLASS_2, 127);
    rc = llc_mac_hdr_init(nskb, skb.dev.dev_addr, mac_da);
    if (unlikely(rc))
    goto free;
    dev_queue_xmit(nskb);
    out:
    return rc;
    free:
    kfree_skb(nskb);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn llc_station_ac_send_test_r(skb: *mut sk_buff) -> c_int {
    static int llc_station_ac_send_test_r(struct sk_buff *skb)
    {
    u8 mac_da[ETH_ALEN], dsap;
    let mut rc: c_int = 1;
    u32 data_size;
    struct sk_buff *nskb;
    if (skb.mac_len < ETH_HLEN)
    goto out;
// The test request command is type U (llc_len = 3)
    data_size = ntohs(eth_hdr(skb).h_proto) - 3;
    nskb = llc_alloc_frame(core::ptr::null_mut(), skb.dev, LLC_PDU_TYPE_U, data_size);
    if (!nskb)
    goto out;
    llc_pdu_decode_sa(skb, mac_da);
    llc_pdu_decode_ssap(skb, &dsap);
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, 0, dsap, LLC_PDU_RSP);
    llc_pdu_init_as_test_rsp(nskb, skb);
    rc = llc_mac_hdr_init(nskb, skb.dev.dev_addr, mac_da);
    if (unlikely(rc))
    goto free;
    dev_queue_xmit(nskb);
    out:
    return rc;
    free:
    kfree_skb(nskb);
    goto out;
    }
//
// llc_station_rcv - send received pdu to the station state machine
// @skb: received frame.
//
// Sends data unit to station state machine.
//
#[no_mangle]
unsafe extern "C" fn llc_station_rcv(skb: *mut sk_buff) {
    static void llc_station_rcv(struct sk_buff *skb)
    {
    if (llc_stat_ev_rx_null_dsap_xid_c(skb))
    llc_station_ac_send_xid_r(skb);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: llc_stat_ev_rx_null_dsap_test_c(skb)) -> else {
    else if (llc_stat_ev_rx_null_dsap_test_c(skb))
    llc_station_ac_send_test_r(skb);
    kfree_skb(skb);
    }
#[no_mangle]
pub unsafe extern "C" fn llc_station_init() -> void __init {
    void __init llc_station_init(void)
    {
    llc_set_station_handler(llc_station_rcv);
    }
#[no_mangle]
pub unsafe extern "C" fn llc_station_exit() {
    void llc_station_exit(void)
    {
    llc_set_station_handler(core::ptr::null_mut());
    }
