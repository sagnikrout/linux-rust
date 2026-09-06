//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/txbuf.c
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
// RxRPC Tx data buffering.
//
// Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static atomic_t rxrpc_txbuf_debug_ids;
    atomic_t rxrpc_nr_txbuf;
//
// Allocate and partially initialise a data transmission buffer.
//
    struct rxrpc_txbuf *rxrpc_alloc_data_txbuf(struct rxrpc_call *call, size_t data_size,
    size_t data_align, gfp_t gfp)
    {
    struct rxrpc_txbuf *txb;
    size_t total, doff, jsize = sizeof(struct rxrpc_jumbo_header);
    void *buf;
    txb = kzalloc_obj(*txb, gfp);
    if (!txb)
    return core::ptr::null_mut();
// We put a jumbo header in the buffer, but not a full wire header to
// avoid delayed-corruption problems with zerocopy.
//
    doff = round_up(jsize, data_align);
    total = doff + data_size;
    data_align = umax(data_align, L1_CACHE_BYTES);
    mutex_lock(&call.conn.tx_data_alloc_lock);
    buf = page_frag_alloc_align(&call.conn.tx_data_alloc, total, gfp,
    data_align);
    mutex_unlock(&call.conn.tx_data_alloc_lock);
    if (!buf) {
    kfree(txb);
    return core::ptr::null_mut();
    }
    refcount_set(&txb.ref, 1);
    txb.call_debug_id	= call.debug_id;
    txb.debug_id		= atomic_inc_return(&rxrpc_txbuf_debug_ids);
    txb.alloc_size		= data_size;
    txb.space		= data_size;
    txb.offset		= 0;
    txb.flags		= call.conn.out_clientflag;
    txb.seq		= call.send_top + 1;
    txb.data		= buf + doff;
    trace_rxrpc_txbuf(txb.debug_id, txb.call_debug_id, txb.seq, 1,
    rxrpc_txbuf_alloc_data);
    atomic_inc(&rxrpc_nr_txbuf);
    return txb;
    }
#[no_mangle]
pub unsafe extern "C" fn rxrpc_see_txbuf(txb: *mut rxrpc_txbuf, what: enum rxrpc_txbuf_trace) {
    void rxrpc_see_txbuf(struct rxrpc_txbuf *txb, enum rxrpc_txbuf_trace what)
    {
    let mut r: c_int = refcount_read(&txb.ref);
    trace_rxrpc_txbuf(txb.debug_id, txb.call_debug_id, txb.seq, r, what);
    }
#[no_mangle]
unsafe extern "C" fn rxrpc_free_txbuf(txb: *mut rxrpc_txbuf) {
    static void rxrpc_free_txbuf(struct rxrpc_txbuf *txb)
    {
    trace_rxrpc_txbuf(txb.debug_id, txb.call_debug_id, txb.seq, 0,
    rxrpc_txbuf_free);
    if (txb.data)
    page_frag_free(txb.data);
    kfree(txb);
    atomic_dec(&rxrpc_nr_txbuf);
    }
#[no_mangle]
pub unsafe extern "C" fn rxrpc_put_txbuf(txb: *mut rxrpc_txbuf, what: enum rxrpc_txbuf_trace) {
    void rxrpc_put_txbuf(struct rxrpc_txbuf *txb, enum rxrpc_txbuf_trace what)
    {
    unsigned int debug_id, call_debug_id;
    rxrpc_seq_t seq;
    bool dead;
    int r;
    if (txb) {
    debug_id = txb.debug_id;
    call_debug_id = txb.call_debug_id;
    seq = txb.seq;
    dead = __refcount_dec_and_test(&txb.ref, &r);
    trace_rxrpc_txbuf(debug_id, call_debug_id, seq, r - 1, what);
    if (dead)
    rxrpc_free_txbuf(txb);
    }
    }
