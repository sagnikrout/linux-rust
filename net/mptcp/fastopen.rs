//! Automatically rewritten from C to Rust
//! Source: net/mptcp/fastopen.c
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
// MPTCP Fast Open Mechanism
//
// Copyright (c) 2021-2022, Dmytro SHYTYI
//

    void mptcp_fastopen_subflow_synack_set_params(struct mptcp_subflow_context *subflow,
    struct request_sock *req)
    {
    struct sock *sk, *ssk;
    struct sk_buff *skb;
    struct tcp_sock *tp;
    bool has_rxtstamp;
// on early fallback the subflow context is deleted by
// subflow_syn_recv_sock()
//
    if (!subflow)
    return;
    ssk = subflow.tcp_sock;
    sk = subflow.conn;
    tp = tcp_sk(ssk);
// A valid TFO cookie does not guarantee SYN data.
    skb = skb_peek(&ssk.sk_receive_queue);
    if (!skb)
    return;
    subflow.is_mptfo = 1;
// dequeue the skb from sk receive queue
    __skb_unlink(skb, &ssk.sk_receive_queue);
    skb_ext_reset(skb);
    mptcp_subflow_lend_fwdmem(subflow, skb);
// We copy the fastopen data, but that don't belong to the mptcp sequence
// space, need to offset it in the subflow sequence, see mptcp_subflow_get_map_offset()
//
    tp.copied_seq += skb.len;
    subflow.ssn_offset += skb.len;
    has_rxtstamp = TCP_SKB_CB(skb).has_rxtstamp;
// Only the sequence delta is relevant
    MPTCP_SKB_CB(skb).map_seq = -skb.len;
    MPTCP_SKB_CB(skb).end_seq = 0;
    MPTCP_SKB_CB(skb).offset = 0;
    MPTCP_SKB_CB(skb).has_rxtstamp = has_rxtstamp;
    MPTCP_SKB_CB(skb).cant_coalesce = 1;
    mptcp_data_lock(sk);
    DEBUG_NET_WARN_ON_ONCE(sock_owned_by_user_nocheck(sk));
    mptcp_borrow_fwdmem(sk, skb);
    skb_set_owner_r(skb, sk);
    __skb_queue_tail(&sk.sk_receive_queue, skb);
    mptcp_sk(sk).bytes_received += skb.len;
    sk.sk_data_ready(sk);
    mptcp_data_unlock(sk);
    }
