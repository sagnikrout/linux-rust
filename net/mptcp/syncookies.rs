//! Automatically rewritten from C to Rust
//! Source: net/mptcp/syncookies.c
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

// Syncookies do not work for JOIN requests.
//
// Unlike MP_CAPABLE, where the ACK cookie contains the needed MPTCP
// options to reconstruct the initial syn state, MP_JOIN does not contain
// the token to obtain the mptcp socket nor the server-generated nonce
// that was used in the cookie SYN/ACK response.
//
// Keep a small best effort state table to store the syn/synack data,
// indexed by skb hash.
//
// A MP_JOIN SYN packet handled by syn cookies is only stored if the 32bit
// token matches a known mptcp connection that can still accept more subflows.
//
// There is no timeout handling -- state is only re-constructed
// when the TCP ACK passed the cookie validation check.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct join_entry {
    pub token: u32,
    pub remote_nonce: u32,
    pub local_nonce: u32,
    pub join_id: u8,
    pub local_id: u8,
    pub backup: u8,
    pub valid: u8,
}

pub const COOKIE_JOIN_SLOTS: c_int = 1024;
    static struct join_entry join_entries[COOKIE_JOIN_SLOTS] __cacheline_aligned_in_smp;
    static spinlock_t join_entry_locks[COOKIE_JOIN_SLOTS] __cacheline_aligned_in_smp;
#[no_mangle]
unsafe extern "C" fn mptcp_join_entry_hash(skb: *mut sk_buff, net: *mut net) -> u32 {
    static u32 mptcp_join_entry_hash(struct sk_buff *skb, struct net *net)
    {
    static u32 mptcp_join_hash_secret __read_mostly;
    struct tcphdr *th = tcp_hdr(skb);
    u32 seq, i;
    net_get_random_once(&mptcp_join_hash_secret,
    sizeof(mptcp_join_hash_secret));
    if (th.syn)
    seq = TCP_SKB_CB(skb).seq;
    else
    seq = TCP_SKB_CB(skb).seq - 1;
    i = jhash_3words(seq, net_hash_mix(net),
    ( __u32)th.source << 16 | ( __u32)th.dest,
    mptcp_join_hash_secret);
    return i % ARRAY_SIZE(join_entries);
    }
    static void mptcp_join_store_state(struct join_entry *entry,
    const struct mptcp_subflow_request_sock *subflow_req)
    {
    entry.token = subflow_req.token;
    entry.remote_nonce = subflow_req.remote_nonce;
    entry.local_nonce = subflow_req.local_nonce;
    entry.backup = subflow_req.backup;
    entry.join_id = subflow_req.remote_id;
    entry.local_id = subflow_req.local_id;
    entry.valid = 1;
    }
    void subflow_init_req_cookie_join_save(const struct mptcp_subflow_request_sock *subflow_req,
    struct sk_buff *skb)
    {
    struct net *net = read_pnet(&subflow_req.sk.req.ireq_net);
    let mut i: u32 = mptcp_join_entry_hash(skb, net);
// No use in waiting if other cpu is already using this slot --
// would overwrite the data that got stored.
//
    spin_lock_bh(&join_entry_locks[i]);
    mptcp_join_store_state(&join_entries[i], subflow_req);
    spin_unlock_bh(&join_entry_locks[i]);
    }
// Called for a cookie-ack with MP_JOIN option present.
// Look up the saved state based on skb hash & check token matches msk
// in same netns.
//
// Caller will check msk can still accept another subflow.  The hmac
// present in the cookie ACK mptcp option space will be checked later.
//
    bool mptcp_token_join_cookie_init_state(struct mptcp_subflow_request_sock *subflow_req,
    struct sk_buff *skb)
    {
    struct net *net = read_pnet(&subflow_req.sk.req.ireq_net);
    let mut i: u32 = mptcp_join_entry_hash(skb, net);
    struct mptcp_sock *msk;
    struct join_entry *e;
    e = &join_entries[i];
    spin_lock_bh(&join_entry_locks[i]);
    if (e.valid == 0) {
    spin_unlock_bh(&join_entry_locks[i]);
    return false;
    }
    e.valid = 0;
    msk = mptcp_token_get_sock(net, e.token);
    if (!msk) {
    spin_unlock_bh(&join_entry_locks[i]);
    return false;
    }
    subflow_req.remote_nonce = e.remote_nonce;
    subflow_req.local_nonce = e.local_nonce;
    subflow_req.backup = e.backup;
    subflow_req.remote_id = e.join_id;
    subflow_req.local_id = e.local_id;
    subflow_req.token = e.token;
    subflow_req.msk = msk;
    spin_unlock_bh(&join_entry_locks[i]);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_join_cookie_init() -> void __init {
    void __init mptcp_join_cookie_init(void)
    {
    int i;
    for (i = 0; i < COOKIE_JOIN_SLOTS; i++)
    spin_lock_init(&join_entry_locks[i]);
    }
