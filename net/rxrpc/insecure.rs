//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/insecure.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Null security operations.
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static int none_init_connection_security(struct rxrpc_connection *conn,
    struct rxrpc_key_token *token)
    {
    return 0;
    }
//
// Allocate an appropriately sized buffer for the amount of data remaining.
//
    static struct rxrpc_txbuf *none_alloc_txbuf(struct rxrpc_call *call, size_t remain, gfp_t gfp)
    {
    return rxrpc_alloc_data_txbuf(call, umin(remain, RXRPC_JUMBO_DATALEN), 1, gfp);
    }
#[no_mangle]
unsafe extern "C" fn none_secure_packet(call: *mut rxrpc_call, txb: *mut rxrpc_txbuf) -> c_int {
    static int none_secure_packet(struct rxrpc_call *call, struct rxrpc_txbuf *txb)
    {
    txb.pkt_len = txb.len;
    if (txb.len == RXRPC_JUMBO_DATALEN)
    txb.jumboable = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn none_verify_packet(call: *mut rxrpc_call, skb: *mut sk_buff) -> c_int {
    static int none_verify_packet(struct rxrpc_call *call, struct sk_buff *skb)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn none_free_call_crypto(call: *mut rxrpc_call) {
    static void none_free_call_crypto(struct rxrpc_call *call)
    {
    }
    static bool none_validate_challenge(struct rxrpc_connection *conn,
    struct sk_buff *skb)
    {
    rxrpc_abort_conn(conn, skb, RX_PROTOCOL_ERROR, -EPROTO,
    rxrpc_eproto_rxnull_challenge);
    return true;
    }
    static int none_sendmsg_respond_to_challenge(struct sk_buff *challenge,
    struct msghdr *msg)
    {
    return -EINVAL;
    }
    static int none_verify_response(struct rxrpc_connection *conn,
    struct sk_buff *response_skb,
    void *response, unsigned int len)
    {
    return rxrpc_abort_conn(conn, response_skb, RX_PROTOCOL_ERROR, -EPROTO,
    rxrpc_eproto_rxnull_response);
    }
#[no_mangle]
unsafe extern "C" fn none_clear(conn: *mut rxrpc_connection) {
    static void none_clear(struct rxrpc_connection *conn)
    {
    }
#[no_mangle]
unsafe extern "C" fn none_init() -> c_int {
    static int none_init(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn none_exit() {
    static void none_exit(void)
    {
    }
//
// RxRPC Kerberos-based security
//
    const struct rxrpc_security rxrpc_no_security = {
    .name				= "none",
    .security_index			= RXRPC_SECURITY_NONE,
    .init				= none_init,
    .exit				= none_exit,
    .init_connection_security	= none_init_connection_security,
    .free_call_crypto		= none_free_call_crypto,
    .alloc_txbuf			= none_alloc_txbuf,
    .secure_packet			= none_secure_packet,
    .verify_packet			= none_verify_packet,
    .validate_challenge		= none_validate_challenge,
    .sendmsg_respond_to_challenge	= none_sendmsg_respond_to_challenge,
    .verify_response		= none_verify_response,
    .clear				= none_clear,
    };
