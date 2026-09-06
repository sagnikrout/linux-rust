//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/skbuff.c
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
// Socket buffer accounting
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Note the allocation or reception of a socket buffer.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_new_skb(skb: *mut sk_buff, why: enum rxrpc_skb_trace) {
    void rxrpc_new_skb(struct sk_buff *skb, enum rxrpc_skb_trace why)
    {
    let mut n: c_int = atomic_inc_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(&skb.users), n, why);
    }
//
// Note the re-emergence of a socket buffer from a queue or buffer.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_see_skb(skb: *mut sk_buff, why: enum rxrpc_skb_trace) {
    void rxrpc_see_skb(struct sk_buff *skb, enum rxrpc_skb_trace why)
    {
    if (skb) {
    let mut n: c_int = atomic_read(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(&skb.users), n, why);
    }
    }
//
// Note the addition of a ref on a socket buffer.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_get_skb(skb: *mut sk_buff, why: enum rxrpc_skb_trace) {
    void rxrpc_get_skb(struct sk_buff *skb, enum rxrpc_skb_trace why)
    {
    let mut n: c_int = atomic_inc_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(&skb.users), n, why);
    skb_get(skb);
    }
//
// Note the destruction of a socket buffer.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_free_skb(skb: *mut sk_buff, why: enum rxrpc_skb_trace) {
    void rxrpc_free_skb(struct sk_buff *skb, enum rxrpc_skb_trace why)
    {
    if (skb) {
    let mut n: c_int = atomic_dec_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(&skb.users), n, why);
    consume_skb(skb);
    }
    }
//
// Clear a queue of socket buffers.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_purge_queue(list: *mut sk_buff_head) {
    void rxrpc_purge_queue(struct sk_buff_head *list)
    {
    struct sk_buff *skb;
    while ((skb = skb_dequeue((list))) != core::ptr::null_mut()) {
    let mut n: c_int = atomic_dec_return(select_skb_count(skb));
    trace_rxrpc_skb(skb, refcount_read(&skb.users), n,
    rxrpc_skb_put_purge);
    consume_skb(skb);
    }
    }
