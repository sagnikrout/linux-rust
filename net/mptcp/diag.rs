//! Automatically rewritten from C to Rust
//! Source: net/mptcp/diag.c
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
// MPTCP socket monitoring support
//
// Copyright (c) 2019 Red Hat
//
// Author: Davide Caratti <dcaratti@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn subflow_get_info(sk: *mut sock, skb: *mut sk_buff, net_admin: bool) -> c_int {
    static int subflow_get_info(struct sock *sk, struct sk_buff *skb, bool net_admin)
    {
    struct mptcp_subflow_context *sf;
    struct nlattr *start;
    let mut flags: u32 = 0;
    bool slow;
    int err;
    if (inet_sk_state_load(sk) == TCP_LISTEN)
    return 0;
    start = nla_nest_start_noflag(skb, INET_ULP_INFO_MPTCP);
    if (!start)
    return -EMSGSIZE;
    slow = lock_sock_fast(sk);
    rcu_read_lock();
    sf = rcu_dereference(inet_csk(sk).icsk_ulp_data);
    if (!sf) {
    err = 0;
    goto nla_failure;
    }
    if (sf.mp_capable)
    flags |= MPTCP_SUBFLOW_FLAG_MCAP_REM;
    if (sf.request_mptcp)
    flags |= MPTCP_SUBFLOW_FLAG_MCAP_LOC;
    if (sf.mp_join)
    flags |= MPTCP_SUBFLOW_FLAG_JOIN_REM;
    if (sf.request_join)
    flags |= MPTCP_SUBFLOW_FLAG_JOIN_LOC;
    if (sf.backup)
    flags |= MPTCP_SUBFLOW_FLAG_BKUP_REM;
    if (sf.request_bkup)
    flags |= MPTCP_SUBFLOW_FLAG_BKUP_LOC;
    if (READ_ONCE(sf.fully_established))
    flags |= MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED;
    if (sf.conn_finished)
    flags |= MPTCP_SUBFLOW_FLAG_CONNECTED;
    if (sf.map_valid)
    flags |= MPTCP_SUBFLOW_FLAG_MAPVALID;
    if (nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_TOKEN_REM, sf.remote_token) ||
    nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_TOKEN_LOC, sf.token) ||
    nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_FLAGS, flags) ||
    nla_put_u8(skb, MPTCP_SUBFLOW_ATTR_ID_REM, sf.remote_id) ||
    nla_put_u8(skb, MPTCP_SUBFLOW_ATTR_ID_LOC, subflow_get_local_id(sf))) {
    err = -EMSGSIZE;
    goto nla_failure;
    }
// Only export seq related counters to user with CAP_NET_ADMIN
    if (net_admin &&
    (nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_RELWRITE_SEQ,
    sf.rel_write_seq) ||
    nla_put_u64_64bit(skb, MPTCP_SUBFLOW_ATTR_MAP_SEQ, sf.map_seq,
    MPTCP_SUBFLOW_ATTR_PAD) ||
    nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_MAP_SFSEQ,
    sf.map_subflow_seq) ||
    nla_put_u32(skb, MPTCP_SUBFLOW_ATTR_SSN_OFFSET, sf.ssn_offset) ||
    nla_put_u16(skb, MPTCP_SUBFLOW_ATTR_MAP_DATALEN,
    sf.map_data_len))) {
    err = -EMSGSIZE;
    goto nla_failure;
    }
    rcu_read_unlock();
    unlock_sock_fast(sk, slow);
    nla_nest_end(skb, start);
    return 0;
    nla_failure:
    rcu_read_unlock();
    unlock_sock_fast(sk, slow);
    nla_nest_cancel(skb, start);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn subflow_get_info_size(sk: *const sock, net_admin: bool) -> usize {
    static size_t subflow_get_info_size(const struct sock *sk, bool net_admin)
    {
    let mut size: usize = 0;
    size += nla_total_size(0) +	/* INET_ULP_INFO_MPTCP */
    nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_TOKEN_REM */
    nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_TOKEN_LOC */
    nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_FLAGS */
    nla_total_size(1) +	/* MPTCP_SUBFLOW_ATTR_ID_REM */
    nla_total_size(1) +	/* MPTCP_SUBFLOW_ATTR_ID_LOC */
    0;
    if (net_admin)
    size += nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_RELWRITE_SEQ */
    nla_total_size_64bit(8) +	/* MPTCP_SUBFLOW_ATTR_MAP_SEQ */
    nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_MAP_SFSEQ */
    nla_total_size(4) +	/* MPTCP_SUBFLOW_ATTR_SSN_OFFSET */
    nla_total_size(2) +	/* MPTCP_SUBFLOW_ATTR_MAP_DATALEN */
    0;
    return size;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_diag_subflow_init(ops: *mut tcp_ulp_ops) {
    void mptcp_diag_subflow_init(struct tcp_ulp_ops *ops)
    {
    ops.get_info = subflow_get_info;
    ops.get_info_size = subflow_get_info_size;
    }
