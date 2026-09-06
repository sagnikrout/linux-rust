//! Automatically rewritten from C to Rust
//! Source: net/phonet/pep-gprs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// File: pep-gprs.c
//
// GPRS over Phonet pipe end point socket
//
// Copyright (C) 2008 Nokia Corporation.
//
// Author: Rémi Denis-Courmont
//

pub const GPRS_DEFAULT_MTU: c_int = 1400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gprs_dev {
    pub sk: *mut sock,
    pub ): *mut *mut void (old_state_change)(struct sock,
    pub ): *mut *mut void (old_data_ready)(struct sock,
    pub ): *mut *mut void (old_write_space)(struct sock,
    pub dev: *mut net_device,
}

#[no_mangle]
unsafe extern "C" fn gprs_type_trans(skb: *mut sk_buff) -> __be16 {
    static __be16 gprs_type_trans(struct sk_buff *skb)
    {
    const u8 *pvfc;
    u8 buf;
    pvfc = skb_header_pointer(skb, 0, 1, &buf);
    if (!pvfc)
    return htons(0);
// Look at IP version field
    switch (*pvfc >> 4) {
    case 4:
    return htons(ETH_P_IP);
    case 6:
    return htons(ETH_P_IPV6);
    }
    return htons(0);
    }
#[no_mangle]
unsafe extern "C" fn gprs_writeable(gp: *mut gprs_dev) {
    static void gprs_writeable(struct gprs_dev *gp)
    {
    struct net_device *dev = gp.dev;
    if (pep_writeable(gp.sk))
    netif_wake_queue(dev);
    }
//
// Socket callbacks
//
#[no_mangle]
unsafe extern "C" fn gprs_state_change(sk: *mut sock) {
    static void gprs_state_change(struct sock *sk)
    {
    struct gprs_dev *gp = sk.sk_user_data;
    if (sk.sk_state == TCP_CLOSE_WAIT) {
    struct net_device *dev = gp.dev;
    netif_stop_queue(dev);
    netif_carrier_off(dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn gprs_recv(gp: *mut gprs_dev, skb: *mut sk_buff) -> c_int {
    static int gprs_recv(struct gprs_dev *gp, struct sk_buff *skb)
    {
    struct net_device *dev = gp.dev;
    let mut err: c_int = 0;
    let mut protocol: __be16 = gprs_type_trans(skb);
    if (!protocol) {
    err = -EINVAL;
    goto drop;
    }
    if (skb_headroom(skb) & 3) {
    struct sk_buff *rskb, *fs;
    let mut flen: c_int = 0;
// Phonet Pipe data header may be misaligned (3 bytes),
// so wrap the IP packet as a single fragment of an head-less
// socket buffer. The network stack will pull what it needs,
// but at least, the whole IP payload is not memcpy'd.
    rskb = netdev_alloc_skb(dev, 0);
    if (!rskb) {
    err = -ENOBUFS;
    goto drop;
    }
    skb_shinfo(rskb).frag_list = skb;
    rskb.len += skb.len;
    rskb.data_len += rskb.len;
    rskb.truesize += rskb.len;
// Avoid nested fragments
    skb_walk_frags(skb, fs)
    flen += fs.len;
    skb.next = skb_shinfo(skb).frag_list;
    skb_frag_list_init(skb);
    skb.len -= flen;
    skb.data_len -= flen;
    skb.truesize -= flen;
    skb = rskb;
    }
    skb.protocol = protocol;
    skb_reset_mac_header(skb);
    skb.dev = dev;
    if (likely(dev.flags & IFF_UP)) {
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += skb.len;
    netif_rx(skb);
    skb = core::ptr::null_mut();
    } else
    err = -ENODEV;
    drop:
    if (skb) {
    dev_kfree_skb(skb);
    dev.stats.rx_dropped++;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn gprs_data_ready(sk: *mut sock) {
    static void gprs_data_ready(struct sock *sk)
    {
    struct gprs_dev *gp = sk.sk_user_data;
    struct sk_buff *skb;
    trace_sk_data_ready(sk);
    while ((skb = pep_read(sk)) != core::ptr::null_mut()) {
    skb_orphan(skb);
    gprs_recv(gp, skb);
    }
    }
#[no_mangle]
unsafe extern "C" fn gprs_write_space(sk: *mut sock) {
    static void gprs_write_space(struct sock *sk)
    {
    struct gprs_dev *gp = sk.sk_user_data;
    if (netif_running(gp.dev))
    gprs_writeable(gp);
    }
//
// Network device callbacks
//
#[no_mangle]
unsafe extern "C" fn gprs_open(dev: *mut net_device) -> c_int {
    static int gprs_open(struct net_device *dev)
    {
    struct gprs_dev *gp = netdev_priv(dev);
    gprs_writeable(gp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gprs_close(dev: *mut net_device) -> c_int {
    static int gprs_close(struct net_device *dev)
    {
    netif_stop_queue(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gprs_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t gprs_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct gprs_dev *gp = netdev_priv(dev);
    struct sock *sk = gp.sk;
    int len, err;
    switch (skb.protocol) {
    case  htons(ETH_P_IP):
    case  htons(ETH_P_IPV6):
    break;
    default:
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    skb_orphan(skb);
    skb_set_owner_w(skb, sk);
    len = skb.len;
    err = pep_write(sk, skb);
    if (err) {
    net_dbg_ratelimited("%s: TX error (%d)\n", dev.name, err);
    dev.stats.tx_aborted_errors++;
    dev.stats.tx_errors++;
    } else {
    dev.stats.tx_packets++;
    dev.stats.tx_bytes += len;
    }
    netif_stop_queue(dev);
    if (pep_writeable(sk))
    netif_wake_queue(dev);
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops gprs_netdev_ops = {
    .ndo_open	= gprs_open,
    .ndo_stop	= gprs_close,
    .ndo_start_xmit	= gprs_xmit,
    };
#[no_mangle]
unsafe extern "C" fn gprs_setup(dev: *mut net_device) {
    static void gprs_setup(struct net_device *dev)
    {
    dev.features		= NETIF_F_FRAGLIST;
    dev.type		= ARPHRD_PHONET_PIPE;
    dev.flags		= IFF_POINTOPOINT | IFF_NOARP;
    dev.mtu		= GPRS_DEFAULT_MTU;
    dev.min_mtu		= 576;
    dev.max_mtu		= (PHONET_MAX_MTU - 11);
    dev.hard_header_len	= 0;
    dev.addr_len		= 0;
    dev.tx_queue_len	= 10;
    dev.netdev_ops		= &gprs_netdev_ops;
    dev.needs_free_netdev	= true;
    }
//
// External interface
//
// Attach a GPRS interface to a datagram socket.
// Returns the interface index on success, negative error code on error.
//
#[no_mangle]
pub unsafe extern "C" fn gprs_attach(sk: *mut sock) -> c_int {
    int gprs_attach(struct sock *sk)
    {
    static const char ifname[] = "gprs%d";
    struct gprs_dev *gp;
    struct net_device *dev;
    int err;
    if (unlikely(sk.sk_type == SOCK_STREAM))
    return -EINVAL; /* need packet boundaries */
// Create net device
    dev = alloc_netdev(sizeof(*gp), ifname, NET_NAME_UNKNOWN, gprs_setup);
    if (!dev)
    return -ENOMEM;
    gp = netdev_priv(dev);
    gp.sk = sk;
    gp.dev = dev;
    netif_stop_queue(dev);
    err = register_netdev(dev);
    if (err) {
    free_netdev(dev);
    return err;
    }
    lock_sock(sk);
    if (unlikely(sk.sk_user_data)) {
    err = -EBUSY;
    goto out_rel;
    }
    if (unlikely((1 << sk.sk_state & (TCPF_CLOSE|TCPF_LISTEN)) ||
    sock_flag(sk, SOCK_DEAD))) {
    err = -EINVAL;
    goto out_rel;
    }
    sk.sk_user_data	= gp;
    gp.old_state_change	= sk.sk_state_change;
    gp.old_data_ready	= sk.sk_data_ready;
    gp.old_write_space	= sk.sk_write_space;
    sk.sk_state_change	= gprs_state_change;
    sk.sk_data_ready	= gprs_data_ready;
    sk.sk_write_space	= gprs_write_space;
    release_sock(sk);
    sock_hold(sk);
    printk(KERN_DEBUG"%s: attached\n", dev.name);
    return dev.ifindex;
    out_rel:
    release_sock(sk);
    unregister_netdev(dev);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn gprs_detach(sk: *mut sock) {
    void gprs_detach(struct sock *sk)
    {
    struct gprs_dev *gp = sk.sk_user_data;
    struct net_device *dev = gp.dev;
    lock_sock(sk);
    sk.sk_user_data	= core::ptr::null_mut();
    sk.sk_state_change	= gp.old_state_change;
    sk.sk_data_ready	= gp.old_data_ready;
    sk.sk_write_space	= gp.old_write_space;
    release_sock(sk);
    printk(KERN_DEBUG"%s: detached\n", dev.name);
    unregister_netdev(dev);
    sock_put(sk);
    }
