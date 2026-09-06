//! Automatically rewritten from C to Rust
//! Source: net/vmw_vsock/vsock_bpf.c
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
// Copyright (c) 2022 Bobby Eshleman <bobby.eshleman@bytedance.com>
//
// Based off of net/unix/unix_bpf.c
//

    ({	!skb_queue_empty(&(__sk).sk_receive_queue) ||	\
    !skb_queue_empty(&(__psock).ingress_skb) ||	\
    !list_empty(&(__psock).ingress_msg);		\
    })
    static struct proto *vsock_prot_saved __read_mostly;
    static DEFINE_SPINLOCK(vsock_prot_lock);
    static struct proto vsock_bpf_prot;
#[no_mangle]
unsafe extern "C" fn vsock_has_data(sk: *mut sock, psock: *mut sk_psock) -> bool {
    static bool vsock_has_data(struct sock *sk, struct sk_psock *psock)
    {
    struct vsock_sock *vsk = vsock_sk(sk);
    s64 ret;
    ret = vsock_connectible_has_data(vsk);
    if (ret > 0)
    return true;
    return vsock_sk_has_data(sk, psock);
    }
#[no_mangle]
unsafe extern "C" fn vsock_msg_wait_data(sk: *mut sock, psock: *mut sk_psock, timeo: c_long) -> bool {
    static bool vsock_msg_wait_data(struct sock *sk, struct sk_psock *psock, long timeo)
    {
    bool ret;
    DEFINE_WAIT_FUNC(wait, woken_wake_function);
    if (sk.sk_shutdown & RCV_SHUTDOWN)
    return true;
    if (!timeo)
    return false;
    add_wait_queue(sk_sleep(sk), &wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    ret = vsock_has_data(sk, psock);
    if (!ret) {
    wait_woken(&wait, TASK_INTERRUPTIBLE, timeo);
    ret = vsock_has_data(sk, psock);
    }
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), &wait);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __vsock_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int {
    static int __vsock_recvmsg(struct sock *sk, struct msghdr *msg, size_t len, int flags)
    {
    struct socket *sock = sk.sk_socket;
    int err;
    if (sk.sk_type == SOCK_STREAM || sk.sk_type == SOCK_SEQPACKET)
    err = __vsock_connectible_recvmsg(sock, msg, len, flags);
#[no_mangle]
pub unsafe extern "C" fn if(SOCK_DGRAM: sk->sk_type ==) -> else {
    else if (sk.sk_type == SOCK_DGRAM)
    err = __vsock_dgram_recvmsg(sock, msg, len, flags);
    else
    err = -EPROTOTYPE;
    return err;
    }
    static int vsock_bpf_recvmsg(struct sock *sk, struct msghdr *msg,
    size_t len, int flags)
    {
    struct sk_psock *psock;
    struct vsock_sock *vsk;
    int copied;
    psock = sk_psock_get(sk);
    if (unlikely(!psock))
    return __vsock_recvmsg(sk, msg, len, flags);
    lock_sock(sk);
    vsk = vsock_sk(sk);
    if (WARN_ON_ONCE(!vsk.transport)) {
    copied = -ENODEV;
    goto out;
    }
    if (vsock_has_data(sk, psock) && sk_psock_queue_empty(psock)) {
    release_sock(sk);
    sk_psock_put(sk, psock);
    return __vsock_recvmsg(sk, msg, len, flags);
    }
    copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
    while (copied == 0) {
    let mut timeo: c_long = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);
    if (!vsock_msg_wait_data(sk, psock, timeo)) {
    copied = -EAGAIN;
    break;
    }
    if (sk_psock_queue_empty(psock)) {
    release_sock(sk);
    sk_psock_put(sk, psock);
    return __vsock_recvmsg(sk, msg, len, flags);
    }
    copied = sk_msg_recvmsg(sk, psock, msg, len, flags);
    }
    out:
    release_sock(sk);
    sk_psock_put(sk, psock);
    return copied;
    }
#[no_mangle]
unsafe extern "C" fn vsock_bpf_rebuild_protos(prot: *mut proto, base: *const proto) {
    static void vsock_bpf_rebuild_protos(struct proto *prot, const struct proto *base)
    {
// prot        = *base;
    prot.close  = sock_map_close;
    prot.recvmsg = vsock_bpf_recvmsg;
    prot.sock_is_readable = sk_msg_is_readable;
    }
#[no_mangle]
unsafe extern "C" fn vsock_bpf_check_needs_rebuild(ops: *mut proto) {
    static void vsock_bpf_check_needs_rebuild(struct proto *ops)
    {
// Paired with the smp_store_release() below.
    if (unlikely(ops != smp_load_acquire(&vsock_prot_saved))) {
    spin_lock_bh(&vsock_prot_lock);
    if (likely(ops != vsock_prot_saved)) {
    vsock_bpf_rebuild_protos(&vsock_bpf_prot, ops);
// Make sure proto function pointers are updated before publishing the
// pointer to the struct.
//
    smp_store_release(&vsock_prot_saved, ops);
    }
    spin_unlock_bh(&vsock_prot_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vsock_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int {
    int vsock_bpf_update_proto(struct sock *sk, struct sk_psock *psock, bool restore)
    {
    struct vsock_sock *vsk;
    if (restore) {
    sk.sk_write_space = psock.saved_write_space;
    sock_replace_proto(sk, psock.sk_proto);
    return 0;
    }
    vsk = vsock_sk(sk);
    if (!vsk.transport)
    return -ENODEV;
    if (!vsk.transport.read_skb)
    return -EOPNOTSUPP;
    vsock_bpf_check_needs_rebuild(psock.sk_proto);
    sock_replace_proto(sk, &vsock_bpf_prot);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vsock_bpf_build_proto() -> void __init {
    void __init vsock_bpf_build_proto(void)
    {
    vsock_bpf_rebuild_protos(&vsock_bpf_prot, &vsock_proto);
    }
