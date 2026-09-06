//! Automatically rewritten from C to Rust
//! Source: net/mptcp/token.c
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
// Multipath TCP token management
// Copyright (c) 2017 - 2019, Intel Corporation.
//
// Note: This code is based on mptcp_ctrl.c from multipath-tcp.org,
// authored by:
//
// Sébastien Barré <sebastien.barre@uclouvain.be>
// Christoph Paasch <christoph.paasch@uclouvain.be>
// Jaakko Korkeaniemi <jaakko.korkeaniemi@aalto.fi>
// Gregory Detal <gregory.detal@uclouvain.be>
// Fabien Duchêne <fabien.duchene@uclouvain.be>
// Andreas Seelinger <Andreas.Seelinger@rwth-aachen.de>
// Lavkesh Lahngir <lavkesh51@gmail.com>
// Andreas Ripke <ripke@neclab.eu>
// Vlad Dogaru <vlad.dogaru@intel.com>
// Octavian Purdila <octavian.purdila@intel.com>
// John Ronan <jronan@tssg.org>
// Catalin Nicutar <catalin.nicutar@gmail.com>
// Brandon Heller <brandonh@stanford.edu>
//

pub const TOKEN_MAX_CHAIN_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct token_bucket {
    pub lock: spinlock_t,
    pub chain_len: c_int,
    pub req_chain: hlist_nulls_head,
    pub msk_chain: hlist_nulls_head,
}

    static struct token_bucket *token_hash __read_mostly;
    static unsigned int token_mask __read_mostly;
    static struct token_bucket *token_bucket(u32 token)
    {
    return &token_hash[token & token_mask];
    }
// called with bucket lock held
    static struct mptcp_subflow_request_sock *
    __token_lookup_req(struct token_bucket *t, u32 token)
    {
    struct mptcp_subflow_request_sock *req;
    struct hlist_nulls_node *pos;
    hlist_nulls_for_each_entry_rcu(req, pos, &t.req_chain, token_node)
    if (req.token == token)
    return req;
    return core::ptr::null_mut();
    }
// called with bucket lock held
    static struct mptcp_sock *
    __token_lookup_msk(struct token_bucket *t, u32 token)
    {
    struct hlist_nulls_node *pos;
    struct sock *sk;
    sk_nulls_for_each_rcu(sk, pos, &t.msk_chain)
    if (mptcp_sk(sk).token == token)
    return mptcp_sk(sk);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __token_bucket_busy(t: *mut token_bucket, token: u32) -> bool {
    static bool __token_bucket_busy(struct token_bucket *t, u32 token)
    {
    return !token || t.chain_len >= TOKEN_MAX_CHAIN_LEN ||
    __token_lookup_req(t, token) || __token_lookup_msk(t, token);
    }
#[no_mangle]
unsafe extern "C" fn mptcp_crypto_key_gen_sha(key: *mut u64, token: *mut u32, idsn: *mut u64) {
    static void mptcp_crypto_key_gen_sha(u64 *key, u32 *token, u64 *idsn)
    {
// we might consider a faster version that computes the key as a
// hash of some information available in the MPTCP socket. Use
// random data at the moment, as it's probably the safest option
// in case multiple sockets are opened in different namespaces at
// the same time.
//
    get_random_bytes(key, sizeof(u64));
    mptcp_crypto_key_sha(*key, token, idsn);
    }
//
// mptcp_token_new_request - create new key/idsn/token for subflow_request
// @req: the request socket
//
// This function is called when a new mptcp connection is coming in.
//
// It creates a unique token to identify the new mptcp connection,
// a secret local key and the initial data sequence number (idsn).
//
// Return: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_new_request(req: *mut request_sock) -> c_int {
    int mptcp_token_new_request(struct request_sock *req)
    {
    struct mptcp_subflow_request_sock *subflow_req = mptcp_subflow_rsk(req);
    struct token_bucket *bucket;
    u32 token;
    mptcp_crypto_key_sha(subflow_req.local_key,
    &subflow_req.token,
    &subflow_req.idsn);
    pr_debug("req=%p local_key=%llu, token=%u, idsn=%llu\n",
    req, subflow_req.local_key, subflow_req.token,
    subflow_req.idsn);
    token = subflow_req.token;
    bucket = token_bucket(token);
    spin_lock_bh(&bucket.lock);
    if (__token_bucket_busy(bucket, token)) {
    spin_unlock_bh(&bucket.lock);
    return -EBUSY;
    }
    hlist_nulls_add_head_rcu(&subflow_req.token_node, &bucket.req_chain);
    bucket.chain_len++;
    spin_unlock_bh(&bucket.lock);
    return 0;
    }
//
// mptcp_token_new_connect - create new key/idsn/token for subflow
// @ssk: the socket that will initiate a connection
//
// This function is called when a new outgoing mptcp connection is
// initiated.
//
// It creates a unique token to identify the new mptcp connection,
// a secret local key and the initial data sequence number (idsn).
//
// On success, the mptcp connection can be found again using
// the computed token at a later time, this is needed to process
// join requests.
//
// Return: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_new_connect(ssk: *mut sock) -> c_int {
    int mptcp_token_new_connect(struct sock *ssk)
    {
    struct mptcp_subflow_context *subflow = mptcp_subflow_ctx(ssk);
    struct mptcp_sock *msk = mptcp_sk(subflow.conn);
    let mut retries: c_int = MPTCP_TOKEN_MAX_RETRIES;
    struct sock *sk = subflow.conn;
    struct token_bucket *bucket;
    again:
    mptcp_crypto_key_gen_sha(&subflow.local_key, &subflow.token,
    &subflow.idsn);
    bucket = token_bucket(subflow.token);
    spin_lock_bh(&bucket.lock);
    if (__token_bucket_busy(bucket, subflow.token)) {
    spin_unlock_bh(&bucket.lock);
    if (!--retries)
    return -EBUSY;
    goto again;
    }
    pr_debug("ssk=%p, local_key=%llu, token=%u, idsn=%llu\n",
    ssk, subflow.local_key, subflow.token, subflow.idsn);
    WRITE_ONCE(msk.token, subflow.token);
    __sk_nulls_add_node_rcu((struct sock *)msk, &bucket.msk_chain);
    bucket.chain_len++;
    spin_unlock_bh(&bucket.lock);
    sock_prot_inuse_add(sock_net(sk), sk.sk_prot, 1);
    return 0;
    }
//
// mptcp_token_accept - replace a req sk with full sock in token hash
// @req: the request socket to be removed
// @msk: the just cloned socket linked to the new connection
//
// Called when a SYN packet creates a new logical connection, i.e.
// is not a join request.
//
    void mptcp_token_accept(struct mptcp_subflow_request_sock *req,
    struct mptcp_sock *msk)
    {
    struct mptcp_subflow_request_sock *pos;
    struct sock *sk = (struct sock *)msk;
    struct token_bucket *bucket;
    sock_prot_inuse_add(sock_net(sk), sk.sk_prot, 1);
    bucket = token_bucket(req.token);
    spin_lock_bh(&bucket.lock);
// pedantic lookup check for the moved token
    pos = __token_lookup_req(bucket, req.token);
    if (!WARN_ON_ONCE(pos != req))
    hlist_nulls_del_init_rcu(&req.token_node);
    __sk_nulls_add_node_rcu((struct sock *)msk, &bucket.msk_chain);
    spin_unlock_bh(&bucket.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_exists(token: u32) -> bool {
    bool mptcp_token_exists(u32 token)
    {
    struct hlist_nulls_node *pos;
    struct token_bucket *bucket;
    struct mptcp_sock *msk;
    struct sock *sk;
    rcu_read_lock();
    bucket = token_bucket(token);
    again:
    sk_nulls_for_each_rcu(sk, pos, &bucket.msk_chain) {
    msk = mptcp_sk(sk);
    if (READ_ONCE(msk.token) == token)
    goto found;
    }
    if (get_nulls_value(pos) != (token & token_mask))
    goto again;
    rcu_read_unlock();
    return false;
    found:
    rcu_read_unlock();
    return true;
    }
//
// mptcp_token_get_sock - retrieve mptcp connection sock using its token
// @net: restrict to this namespace
// @token: token of the mptcp connection to retrieve
//
// This function returns the mptcp connection structure with the given token.
// A reference count on the mptcp socket returned is taken.
//
// Return: NULL if no connection with the given token value exists.
//
    struct mptcp_sock *mptcp_token_get_sock(struct net *net, u32 token)
    {
    struct hlist_nulls_node *pos;
    struct token_bucket *bucket;
    struct mptcp_sock *msk;
    struct sock *sk;
    rcu_read_lock();
    bucket = token_bucket(token);
    again:
    sk_nulls_for_each_rcu(sk, pos, &bucket.msk_chain) {
    msk = mptcp_sk(sk);
    if (READ_ONCE(msk.token) != token ||
    !net_eq(sock_net(sk), net))
    continue;
    if (!refcount_inc_not_zero(&sk.sk_refcnt))
    goto not_found;
    if (READ_ONCE(msk.token) != token ||
    !net_eq(sock_net(sk), net)) {
    sock_put(sk);
    goto again;
    }
    goto found;
    }
    if (get_nulls_value(pos) != (token & token_mask))
    goto again;
    not_found:
    msk = core::ptr::null_mut();
    found:
    rcu_read_unlock();
    return msk;
    }
    EXPORT_SYMBOL_GPL(mptcp_token_get_sock);
//
// mptcp_token_iter_next - iterate over the token container from given pos
// @net: namespace to be iterated
// @s_slot: start slot number
// @s_num: start number inside the given lock
//
// Description:
// On successful iteration, the iterator is moved to the next position and a
// reference to the returned socket is acquired.
//
// Return:
// The first mptcp connection structure found inside the token container
// starting from the specified position, or NULL.
//
    struct mptcp_sock *mptcp_token_iter_next(const struct net *net, long *s_slot,
    long *s_num)
    {
    struct mptcp_sock *ret = core::ptr::null_mut();
    struct hlist_nulls_node *pos;
    int slot, num = 0;
    for (slot = *s_slot; slot <= token_mask; *s_num = 0, slot++) {
    struct token_bucket *bucket = &token_hash[slot];
    struct sock *sk;
    num = 0;
    if (hlist_nulls_empty(&bucket.msk_chain))
    continue;
    rcu_read_lock();
    sk_nulls_for_each_rcu(sk, pos, &bucket.msk_chain) {
    ++num;
    if (!net_eq(sock_net(sk), net))
    continue;
    if (num <= *s_num)
    continue;
    if (!refcount_inc_not_zero(&sk.sk_refcnt))
    continue;
    if (!net_eq(sock_net(sk), net)) {
    sock_put(sk);
    continue;
    }
    ret = mptcp_sk(sk);
    rcu_read_unlock();
    goto out;
    }
    rcu_read_unlock();
    }
    out:
// s_slot = slot;
// s_num = num;
    return ret;
    }
    EXPORT_SYMBOL_GPL(mptcp_token_iter_next);
//
// mptcp_token_destroy_request - remove mptcp connection/token
// @req: mptcp request socket dropping the token
//
// Remove the token associated to @req.
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_destroy_request(req: *mut request_sock) {
    void mptcp_token_destroy_request(struct request_sock *req)
    {
    struct mptcp_subflow_request_sock *subflow_req = mptcp_subflow_rsk(req);
    struct mptcp_subflow_request_sock *pos;
    struct token_bucket *bucket;
    if (hlist_nulls_unhashed(&subflow_req.token_node))
    return;
    bucket = token_bucket(subflow_req.token);
    spin_lock_bh(&bucket.lock);
    pos = __token_lookup_req(bucket, subflow_req.token);
    if (!WARN_ON_ONCE(pos != subflow_req)) {
    hlist_nulls_del_init_rcu(&pos.token_node);
    bucket.chain_len--;
    }
    spin_unlock_bh(&bucket.lock);
    }
//
// mptcp_token_destroy - remove mptcp connection/token
// @msk: mptcp connection dropping the token
//
// Remove the token associated to @msk
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_destroy(msk: *mut mptcp_sock) {
    void mptcp_token_destroy(struct mptcp_sock *msk)
    {
    struct sock *sk = (struct sock *)msk;
    struct token_bucket *bucket;
    struct mptcp_sock *pos;
    if (sk_unhashed((struct sock *)msk))
    return;
    sock_prot_inuse_add(sock_net(sk), sk.sk_prot, -1);
    bucket = token_bucket(msk.token);
    spin_lock_bh(&bucket.lock);
    pos = __token_lookup_msk(bucket, msk.token);
    if (!WARN_ON_ONCE(pos != msk)) {
    __sk_nulls_del_node_init_rcu((struct sock *)pos);
    bucket.chain_len--;
    }
    spin_unlock_bh(&bucket.lock);
    WRITE_ONCE(msk.token, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_token_init() -> void __init {
    void __init mptcp_token_init(void)
    {
    int i;
    token_hash = alloc_large_system_hash("MPTCP token",
    sizeof(struct token_bucket),
    0,
    20,/* one slot per 1MB of memory */
    HASH_ZERO,
    core::ptr::null_mut(),
    &token_mask,
    0,
    64 * 1024);
    for (i = 0; i < token_mask + 1; ++i) {
    INIT_HLIST_NULLS_HEAD(&token_hash[i].req_chain, i);
    INIT_HLIST_NULLS_HEAD(&token_hash[i].msk_chain, i);
    spin_lock_init(&token_hash[i].lock);
    }
    }

    EXPORT_SYMBOL_GPL(mptcp_token_new_request);
    EXPORT_SYMBOL_GPL(mptcp_token_new_connect);
    EXPORT_SYMBOL_GPL(mptcp_token_accept);
    EXPORT_SYMBOL_GPL(mptcp_token_destroy_request);
    EXPORT_SYMBOL_GPL(mptcp_token_destroy);
