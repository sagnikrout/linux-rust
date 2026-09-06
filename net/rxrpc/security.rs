//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/security.c
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
// RxRPC security handling
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static const struct rxrpc_security *rxrpc_security_types[] = {
    [RXRPC_SECURITY_NONE]	= &rxrpc_no_security,

    [RXRPC_SECURITY_RXKAD]	= &rxkad,

    [RXRPC_SECURITY_YFS_RXGK] = &rxgk_yfs,

    };
#[no_mangle]
pub unsafe extern "C" fn rxrpc_init_security() -> int __init {
    int __init rxrpc_init_security(void)
    {
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(rxrpc_security_types); i++) {
    if (rxrpc_security_types[i]) {
    ret = rxrpc_security_types[i].init();
    if (ret < 0)
    goto failed;
    }
    }
    return 0;
    failed:
    for (i--; i >= 0; i--)
    if (rxrpc_security_types[i])
    rxrpc_security_types[i].exit();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rxrpc_exit_security() {
    void rxrpc_exit_security(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(rxrpc_security_types); i++)
    if (rxrpc_security_types[i])
    rxrpc_security_types[i].exit();
    }
//
// look up an rxrpc security module
//
    const struct rxrpc_security *rxrpc_security_lookup(u8 security_index)
    {
    if (security_index >= ARRAY_SIZE(rxrpc_security_types))
    return core::ptr::null_mut();
    return rxrpc_security_types[security_index];
    }
//
// Initialise the security on a client call.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_init_client_call_security(call: *mut rxrpc_call) -> c_int {
    int rxrpc_init_client_call_security(struct rxrpc_call *call)
    {
    const struct rxrpc_security *sec = &rxrpc_no_security;
    struct rxrpc_key_token *token;
    struct key *key = call.key;
    int ret;
    if (!key)
    goto found;
    ret = key_validate(key);
    if (ret < 0)
    return ret;
    for (token = key.payload.data[0]; token; token = token.next) {
    sec = rxrpc_security_lookup(token.security_index);
    if (sec)
    goto found;
    }
    return -EKEYREJECTED;
    found:
    call.security = sec;
    call.security_ix = sec.security_index;
    return 0;
    }
//
// initialise the security on a client connection
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_init_client_conn_security(conn: *mut rxrpc_connection) -> c_int {
    int rxrpc_init_client_conn_security(struct rxrpc_connection *conn)
    {
    struct rxrpc_key_token *token;
    struct key *key = conn.key;
    let mut ret: c_int = 0;
    _enter("{%d},{%x}", conn.debug_id, key_serial(key));
    for (token = key.payload.data[0]; token; token = token.next) {
    if (token.security_index == conn.security.security_index)
    goto found;
    }
    return -EKEYREJECTED;
    found:
    mutex_lock(&conn.security_lock);
    if (conn.state == RXRPC_CONN_CLIENT_UNSECURED) {
    ret = conn.security.init_connection_security(conn, token);
    if (ret == 0) {
    spin_lock_irq(&conn.state_lock);
    if (conn.state == RXRPC_CONN_CLIENT_UNSECURED)
    conn.state = RXRPC_CONN_CLIENT;
    spin_unlock_irq(&conn.state_lock);
    }
    }
    mutex_unlock(&conn.security_lock);
    return ret;
    }
//
// Set the ops a server connection.
//
    const struct rxrpc_security *rxrpc_get_incoming_security(struct rxrpc_sock *rx,
    struct sk_buff *skb)
    {
    const struct rxrpc_security *sec;
    struct rxrpc_skb_priv *sp = rxrpc_skb(skb);
    _enter("");
    sec = rxrpc_security_lookup(sp.hdr.securityIndex);
    if (!sec) {
    rxrpc_direct_conn_abort(skb, rxrpc_abort_unsupported_security,
    RX_INVALID_OPERATION, -EKEYREJECTED);
    return core::ptr::null_mut();
    }
    if (sp.hdr.securityIndex != RXRPC_SECURITY_NONE &&
    !rx.securities) {
    rxrpc_direct_conn_abort(skb, rxrpc_abort_no_service_key,
    sec.no_key_abort, -EKEYREJECTED);
    return core::ptr::null_mut();
    }
    return sec;
    }
//
// Find the security key for a server connection.
//
    struct key *rxrpc_look_up_server_security(struct rxrpc_connection *conn,
    struct sk_buff *skb,
    u32 kvno, u32 enctype)
    {
    struct rxrpc_skb_priv *sp = rxrpc_skb(skb);
    struct rxrpc_sock *rx;
    struct key *key = ERR_PTR(-EKEYREJECTED);
    let mut kref: key_ref_t = core::ptr::null_mut();
    char kdesc[5 + 1 + 3 + 1 + 12 + 1 + 12 + 1];
    int ret;
    _enter("");
    if (enctype)
    sprintf(kdesc, "%u:%u:%u:%u",
    sp.hdr.serviceId, sp.hdr.securityIndex, kvno, enctype);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: kvno) -> else {
    else if (kvno)
    sprintf(kdesc, "%u:%u:%u",
    sp.hdr.serviceId, sp.hdr.securityIndex, kvno);
    else
    sprintf(kdesc, "%u:%u",
    sp.hdr.serviceId, sp.hdr.securityIndex);
    read_lock(&conn.local.services_lock);
    rx = conn.local.service;
    if (!rx)
    goto out;
// look through the service's keyring
    kref = keyring_search(make_key_ref(rx.securities, 1UL),
    &key_type_rxrpc_s, kdesc, true);
    if (IS_ERR(kref)) {
    key = ERR_CAST(kref);
    goto out;
    }
    key = key_ref_to_ptr(kref);
    ret = key_validate(key);
    if (ret < 0) {
    key_put(key);
    key = ERR_PTR(ret);
    goto out;
    }
    out:
    read_unlock(&conn.local.services_lock);
    return key;
    }
