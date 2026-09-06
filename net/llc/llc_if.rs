//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_if.c
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
// llc_if.c - Defines LLC interface to upper layer
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

//
// llc_build_and_send_pkt - Connection data sending for upper layers.
// @sk: connection
// @skb: packet to send
//
// This function is called when upper layer wants to send data using
// connection oriented communication mode. During sending data, connection
// will be locked and received frames and expired timers will be queued.
// Returns 0 for success, -ECONNABORTED when the connection already
// closed and -EBUSY when sending data is not permitted in this state or
// LLC has send an I pdu with p bit set to 1 and is waiting for it's
// response.
//
// This function always consumes a reference to the skb.
//
#[no_mangle]
pub unsafe extern "C" fn llc_build_and_send_pkt(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int llc_build_and_send_pkt(struct sock *sk, struct sk_buff *skb)
    {
    struct llc_conn_state_ev *ev;
    let mut rc: c_int = -ECONNABORTED;
    struct llc_sock *llc = llc_sk(sk);
    if (unlikely(llc.state == LLC_CONN_STATE_ADM))
    goto out_free;
    rc = -EBUSY;
    if (unlikely(llc_data_accept_state(llc.state) || /* data_conn_refuse */
    llc.p_flag)) {
    llc.failed_data_req = 1;
    goto out_free;
    }
    ev = llc_conn_ev(skb);
    ev.type      = LLC_CONN_EV_TYPE_PRIM;
    ev.prim      = LLC_DATA_PRIM;
    ev.prim_type = LLC_PRIM_TYPE_REQ;
    skb.dev      = llc.dev;
    return llc_conn_state_process(sk, skb);
    out_free:
    kfree_skb(skb);
    return rc;
    }
//
// llc_establish_connection - Called by upper layer to establish a conn
// @sk: connection
// @lmac: local mac address
// @dmac: destination mac address
// @dsap: destination sap
//
// Upper layer calls this to establish an LLC connection with a remote
// machine. This function packages a proper event and sends it connection
// component state machine. Success or failure of connection
// establishment will inform to upper layer via calling it's confirm
// function and passing proper information.
//
#[no_mangle]
pub unsafe extern "C" fn llc_establish_connection(sk: *mut sock, lmac: *const u8, dmac: *mut u8, dsap: u8) -> c_int {
    int llc_establish_connection(struct sock *sk, const u8 *lmac, u8 *dmac, u8 dsap)
    {
    let mut rc: c_int = -EISCONN;
    struct llc_addr laddr, daddr;
    struct sk_buff *skb;
    struct llc_sock *llc = llc_sk(sk);
    struct sock *existing;
    laddr.lsap = llc.sap.laddr.lsap;
    daddr.lsap = dsap;
    memcpy(daddr.mac, dmac, sizeof(daddr.mac));
    memcpy(laddr.mac, lmac, sizeof(laddr.mac));
    existing = llc_lookup_established(llc.sap, &daddr, &laddr, sock_net(sk));
    if (existing) {
    if (existing.sk_state == TCP_ESTABLISHED) {
    sk = existing;
    goto out_put;
    } else
    sock_put(existing);
    }
    sock_hold(sk);
    rc = -ENOMEM;
    skb = alloc_skb(0, GFP_ATOMIC);
    if (skb) {
    struct llc_conn_state_ev *ev = llc_conn_ev(skb);
    ev.type      = LLC_CONN_EV_TYPE_PRIM;
    ev.prim      = LLC_CONN_PRIM;
    ev.prim_type = LLC_PRIM_TYPE_REQ;
    skb_set_owner_w(skb, sk);
    rc = llc_conn_state_process(sk, skb);
    }
    out_put:
    sock_put(sk);
    return rc;
    }
//
// llc_send_disc - Called by upper layer to close a connection
// @sk: connection to be closed
//
// Upper layer calls this when it wants to close an established LLC
// connection with a remote machine. This function packages a proper event
// and sends it to connection component state machine. Returns 0 for
// success, 1 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn llc_send_disc(sk: *mut sock) -> c_int {
    int llc_send_disc(struct sock *sk)
    {
    let mut rc: u16 = 1;
    struct llc_conn_state_ev *ev;
    struct sk_buff *skb;
    sock_hold(sk);
    if (sk.sk_type != SOCK_STREAM || sk.sk_state != TCP_ESTABLISHED ||
    llc_sk(sk).state == LLC_CONN_STATE_ADM ||
    llc_sk(sk).state == LLC_CONN_OUT_OF_SVC)
    goto out;
//
// Postpone unassigning the connection from its SAP and returning the
// connection until all ACTIONs have been completely executed
//
    skb = alloc_skb(0, GFP_ATOMIC);
    if (!skb)
    goto out;
    skb_set_owner_w(skb, sk);
    sk.sk_state  = TCP_CLOSING;
    ev	      = llc_conn_ev(skb);
    ev.type      = LLC_CONN_EV_TYPE_PRIM;
    ev.prim      = LLC_DISC_PRIM;
    ev.prim_type = LLC_PRIM_TYPE_REQ;
    rc = llc_conn_state_process(sk, skb);
    out:
    sock_put(sk);
    return rc;
    }
