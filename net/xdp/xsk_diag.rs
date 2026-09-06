//! Automatically rewritten from C to Rust
//! Source: net/xdp/xsk_diag.c
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
// XDP sockets monitoring support
//
// Copyright(c) 2019 Intel Corporation.
//
// Author: Björn Töpel <bjorn.topel@intel.com>
//

#[no_mangle]
unsafe extern "C" fn xsk_diag_put_info(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    static int xsk_diag_put_info(const struct xdp_sock *xs, struct sk_buff *nlskb)
    {
    let mut di: xdp_diag_info = {};
    di.ifindex = xs.dev ? xs.dev.ifindex : 0;
    di.queue_id = xs.queue_id;
    return nla_put(nlskb, XDP_DIAG_INFO, sizeof(di), &di);
    }
    static int xsk_diag_put_ring(const struct xsk_queue *queue, int nl_type,
    struct sk_buff *nlskb)
    {
    let mut dr: xdp_diag_ring = {};
    dr.entries = queue.nentries;
    return nla_put(nlskb, nl_type, sizeof(dr), &dr);
    }
    static int xsk_diag_put_rings_cfg(const struct xdp_sock *xs,
    struct sk_buff *nlskb)
    {
    let mut err: c_int = 0;
    if (xs.rx)
    err = xsk_diag_put_ring(xs.rx, XDP_DIAG_RX_RING, nlskb);
    if (!err && xs.tx)
    err = xsk_diag_put_ring(xs.tx, XDP_DIAG_TX_RING, nlskb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xsk_diag_put_umem(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    static int xsk_diag_put_umem(const struct xdp_sock *xs, struct sk_buff *nlskb)
    {
    struct xsk_buff_pool *pool = xs.pool;
    struct xdp_umem *umem = xs.umem;
    let mut du: xdp_diag_umem = {};
    int err;
    if (!umem)
    return 0;
    du.id = umem.id;
    du.size = umem.size;
    du.num_pages = umem.npgs;
    du.chunk_size = umem.chunk_size;
    du.headroom = umem.headroom;
    du.ifindex = (pool && pool.netdev) ? pool.netdev.ifindex : 0;
    du.queue_id = pool ? pool.queue_id : 0;
    du.flags = 0;
    if (umem.zc)
    du.flags |= XDP_DU_F_ZEROCOPY;
    du.refs = refcount_read(&umem.users);
    err = nla_put(nlskb, XDP_DIAG_UMEM, sizeof(du), &du);
    if (!err && pool && pool.fq)
    err = xsk_diag_put_ring(pool.fq,
    XDP_DIAG_UMEM_FILL_RING, nlskb);
    if (!err && pool && pool.cq)
    err = xsk_diag_put_ring(pool.cq,
    XDP_DIAG_UMEM_COMPLETION_RING, nlskb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xsk_diag_put_stats(xs: *const xdp_sock, nlskb: *mut sk_buff) -> c_int {
    static int xsk_diag_put_stats(const struct xdp_sock *xs, struct sk_buff *nlskb)
    {
    let mut du: xdp_diag_stats = {};
    du.n_rx_dropped = xs.rx_dropped;
    du.n_rx_invalid = xskq_nb_invalid_descs(xs.rx);
    du.n_rx_full = xs.rx_queue_full;
    du.n_fill_ring_empty = xs.pool ? xskq_nb_queue_empty_descs(xs.pool.fq) : 0;
    du.n_tx_invalid = xskq_nb_invalid_descs(xs.tx);
    du.n_tx_ring_empty = xskq_nb_queue_empty_descs(xs.tx);
    return nla_put(nlskb, XDP_DIAG_STATS, sizeof(du), &du);
    }
    static int xsk_diag_fill(struct sock *sk, struct sk_buff *nlskb,
    struct xdp_diag_req *req,
    struct user_namespace *user_ns,
    u32 portid, u32 seq, u32 flags, u64 sk_ino)
    {
    struct xdp_sock *xs = xdp_sk(sk);
    struct xdp_diag_msg *msg;
    struct nlmsghdr *nlh;
    nlh = nlmsg_put(nlskb, portid, seq, SOCK_DIAG_BY_FAMILY, sizeof(*msg),
    flags);
    if (!nlh)
    return -EMSGSIZE;
    msg = nlmsg_data(nlh);
    memset(msg, 0, sizeof(*msg));
    msg.xdiag_family = AF_XDP;
    msg.xdiag_type = sk.sk_type;
    msg.xdiag_ino = sk_ino;
    sock_diag_save_cookie(sk, msg.xdiag_cookie);
    mutex_lock(&xs.mutex);
    if (READ_ONCE(xs.state) == XSK_UNBOUND)
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_INFO) && xsk_diag_put_info(xs, nlskb))
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_INFO) &&
    nla_put_u32(nlskb, XDP_DIAG_UID,
    from_kuid_munged(user_ns, sk_uid(sk))))
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_RING_CFG) &&
    xsk_diag_put_rings_cfg(xs, nlskb))
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_UMEM) &&
    xsk_diag_put_umem(xs, nlskb))
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_MEMINFO) &&
    sock_diag_put_meminfo(sk, nlskb, XDP_DIAG_MEMINFO))
    goto out_nlmsg_trim;
    if ((req.xdiag_show & XDP_SHOW_STATS) &&
    xsk_diag_put_stats(xs, nlskb))
    goto out_nlmsg_trim;
    mutex_unlock(&xs.mutex);
    nlmsg_end(nlskb, nlh);
    return 0;
    out_nlmsg_trim:
    mutex_unlock(&xs.mutex);
    nlmsg_cancel(nlskb, nlh);
    return -EMSGSIZE;
    }
#[no_mangle]
unsafe extern "C" fn xsk_diag_dump(nlskb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int xsk_diag_dump(struct sk_buff *nlskb, struct netlink_callback *cb)
    {
    struct xdp_diag_req *req = nlmsg_data(cb.nlh);
    struct net *net = sock_net(nlskb.sk);
    let mut num: c_int = 0, s_num = cb.args[0];
    struct sock *sk;
    mutex_lock(&net.xdp.lock);
    sk_for_each(sk, &net.xdp.list) {
    if (!net_eq(sock_net(sk), net))
    continue;
    if (num++ < s_num)
    continue;
    if (xsk_diag_fill(sk, nlskb, req,
    sk_user_ns(NETLINK_CB(cb.skb).sk),
    NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, NLM_F_MULTI,
    sock_i_ino(sk)) < 0) {
    num--;
    break;
    }
    }
    mutex_unlock(&net.xdp.lock);
    cb.args[0] = num;
    return nlskb.len;
    }
#[no_mangle]
unsafe extern "C" fn xsk_diag_handler_dump(nlskb: *mut sk_buff, hdr: *mut nlmsghdr) -> c_int {
    static int xsk_diag_handler_dump(struct sk_buff *nlskb, struct nlmsghdr *hdr)
    {
    let mut c: netlink_dump_control = { .dump = xsk_diag_dump };
    let mut hdrlen: c_int = sizeof(struct xdp_diag_req);
    struct net *net = sock_net(nlskb.sk);
    if (nlmsg_len(hdr) < hdrlen)
    return -EINVAL;
    if (!(hdr.nlmsg_flags & NLM_F_DUMP))
    return -EOPNOTSUPP;
    return netlink_dump_start(net.diag_nlsk, nlskb, hdr, &c);
    }
    static const struct sock_diag_handler xsk_diag_handler = {
    .owner = THIS_MODULE,
    .family = AF_XDP,
    .dump = xsk_diag_handler_dump,
    };
#[no_mangle]
unsafe extern "C" fn xsk_diag_init() -> int __init {
    static int __init xsk_diag_init(void)
    {
    return sock_diag_register(&xsk_diag_handler);
    }
#[no_mangle]
unsafe extern "C" fn xsk_diag_exit() -> void __exit {
    static void __exit xsk_diag_exit(void)
    {
    sock_diag_unregister(&xsk_diag_handler);
    }
    module_init(xsk_diag_init);
    module_exit(xsk_diag_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("XDP socket monitoring via SOCK_DIAG");
    MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, AF_XDP);
