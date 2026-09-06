//! Automatically rewritten from C to Rust
//! Source: net/unix/unix_bpf.c
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
// Copyright (c) 2021 Cong Wang <cong.wang@bytedance.com>

    ({	!skb_queue_empty(&__sk.sk_receive_queue) ||	\
    !skb_queue_empty(&__psock.ingress_skb) ||	\
    !list_empty(&__psock.ingress_msg);		\
    })
    static int unix_msg_wait_data(struct sock *sk, struct sk_psock *psock,
    long timeo)
    {
    DEFINE_WAIT_FUNC(wait, woken_wake_function);
    struct unix_sock *u = unix_sk(sk);
    let mut ret: c_int = 0;
    if (sk.sk_shutdown & RCV_SHUTDOWN)
    return 1;
    if (!timeo)
    return ret;
    add_wait_queue(sk_sleep(sk), &wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    if (!unix_sk_has_data(sk, psock)) {
    mutex_unlock(&u.iolock);
    wait_woken(&wait, TASK_INTERRUPTIBLE, timeo);
    mutex_lock(&u.iolock);
    ret = unix_sk_has_data(sk, psock);
    }
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), &wait);
    return ret;
    }
    static int __unix_recvmsg(struct sock *sk, struct msghdr *msg,
    size_t len, int flags)
    {
    if (sk.sk_type == SOCK_DGRAM)
    return __unix_dgram_recvmsg(sk, msg, len, flags);
    else
    return __unix_stream_recvmsg(sk, msg, len, flags);
    }
    static int unix_bpf_recvmsg(struct sock *sk, struct msghdr *msg,
    size_t len, int flags)
    {
    struct unix_sock *u = unix_sk(sk);
    struct sk_psock *psock;
    int copied;
    if (flags & MSG_OOB)
    return -EOPNOTSUPP;
    if (!len)
    return 0;
    psock = sk_psock_get(sk);
    if (unlikely(!psock))
    return __unix_recvmsg(sk, msg, len, flags);
    mutex_lock(&u.iolock);
    if (!skb_queue_empty(&sk.sk_receive_queue) &&
    sk_psock_queue_empty(psock)) {
    mutex_unlock(&u.iolock);
    sk_psock_put(sk, psock);
    return __unix_recvmsg(sk, msg, len, flags);
    }
    msg_bytes_ready:
    copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
    if (!copied) {
    long timeo;
    int data;
    timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);
    data = unix_msg_wait_data(sk, psock, timeo);
    if (data) {
    if (!sk_psock_queue_empty(psock))
    goto msg_bytes_ready;
    mutex_unlock(&u.iolock);
    sk_psock_put(sk, psock);
    return __unix_recvmsg(sk, msg, len, flags);
    }
    copied = -EAGAIN;
    }
    mutex_unlock(&u.iolock);
    sk_psock_put(sk, psock);
    return copied;
    }
    static struct proto *unix_dgram_prot_saved __read_mostly;
    static DEFINE_SPINLOCK(unix_dgram_prot_lock);
    static struct proto unix_dgram_bpf_prot;
    static struct proto *unix_stream_prot_saved __read_mostly;
    static DEFINE_SPINLOCK(unix_stream_prot_lock);
    static struct proto unix_stream_bpf_prot;
#[no_mangle]
unsafe extern "C" fn unix_dgram_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    static void unix_dgram_bpf_rebuild_protos(struct proto *prot, const struct proto *base)
    {
// prot        = *base;
    prot.close  = sock_map_close;
    prot.recvmsg = unix_bpf_recvmsg;
    prot.sock_is_readable = sk_msg_is_readable;
    }
    static void unix_stream_bpf_rebuild_protos(struct proto *prot,
    const struct proto *base)
    {
// prot        = *base;
    prot.close  = sock_map_close;
    prot.recvmsg = unix_bpf_recvmsg;
    prot.sock_is_readable = sk_msg_is_readable;
    prot.unhash  = sock_map_unhash;
    }
#[no_mangle]
unsafe extern "C" fn unix_dgram_bpf_check_needs_rebuild(ops: *mut proto) {
    static void unix_dgram_bpf_check_needs_rebuild(struct proto *ops)
    {
    if (unlikely(ops != smp_load_acquire(&unix_dgram_prot_saved))) {
    spin_lock_bh(&unix_dgram_prot_lock);
    if (likely(ops != unix_dgram_prot_saved)) {
    unix_dgram_bpf_rebuild_protos(&unix_dgram_bpf_prot, ops);
    smp_store_release(&unix_dgram_prot_saved, ops);
    }
    spin_unlock_bh(&unix_dgram_prot_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn unix_stream_bpf_check_needs_rebuild(ops: *mut proto) {
    static void unix_stream_bpf_check_needs_rebuild(struct proto *ops)
    {
    if (unlikely(ops != smp_load_acquire(&unix_stream_prot_saved))) {
    spin_lock_bh(&unix_stream_prot_lock);
    if (likely(ops != unix_stream_prot_saved)) {
    unix_stream_bpf_rebuild_protos(&unix_stream_bpf_prot, ops);
    smp_store_release(&unix_stream_prot_saved, ops);
    }
    spin_unlock_bh(&unix_stream_prot_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unix_dgram_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int {
    int unix_dgram_bpf_update_proto(struct sock *sk, struct sk_psock *psock, bool restore)
    {
    if (sk.sk_type != SOCK_DGRAM)
    return -EOPNOTSUPP;
    if (restore) {
    sk.sk_write_space = psock.saved_write_space;
    sock_replace_proto(sk, psock.sk_proto);
    return 0;
    }
    unix_dgram_bpf_check_needs_rebuild(psock.sk_proto);
    sock_replace_proto(sk, &unix_dgram_bpf_prot);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_stream_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int {
    int unix_stream_bpf_update_proto(struct sock *sk, struct sk_psock *psock, bool restore)
    {
    struct sock *sk_pair;
// Restore does not decrement the sk_pair reference yet because we must
// keep the a reference to the socket until after an RCU grace period
// and any pending sends have completed.
//
    if (restore) {
    sk.sk_write_space = psock.saved_write_space;
    sock_replace_proto(sk, psock.sk_proto);
    return 0;
    }
// psock_update_sk_prot can be called multiple times if psock is
// added to multiple maps and/or slots in the same map. There is
// also an edge case where replacing a psock with itself can trigger
// an extra psock_update_sk_prot during the insert process. So it
// must be safe to do multiple calls. Here we need to ensure we don't
// increment the refcnt through sock_hold many times. There will only
// be a single matching destroy operation.
//
    if (!psock.sk_pair) {
    sk_pair = unix_peer(sk);
    if (unlikely(!sk_pair))
    return -EINVAL;
    sock_hold(sk_pair);
    psock.sk_pair = sk_pair;
    }
    unix_stream_bpf_check_needs_rebuild(psock.sk_proto);
    sock_replace_proto(sk, &unix_stream_bpf_prot);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_bpf_build_proto() -> void __init {
    void __init unix_bpf_build_proto(void)
    {
    unix_dgram_bpf_rebuild_protos(&unix_dgram_bpf_prot, &unix_dgram_proto);
    unix_stream_bpf_rebuild_protos(&unix_stream_bpf_prot, &unix_stream_proto);
    }
