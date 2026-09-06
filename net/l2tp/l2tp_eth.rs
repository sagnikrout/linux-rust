//! Automatically rewritten from C to Rust
//! Source: net/l2tp/l2tp_eth.c
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
// L2TPv3 ethernet pseudowire driver
//
// Copyright (c) 2008,2009,2010 Katalix Systems Ltd
//

// Default device name. May be overridden by name specified by user

// via netdev_priv()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_eth {
    pub session: *mut l2tp_session,
}

// via l2tp_session_priv()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_eth_sess {
    pub dev: *mut net_device __rcu,
}

#[no_mangle]
unsafe extern "C" fn l2tp_eth_dev_init(dev: *mut net_device) -> c_int {
    static int l2tp_eth_dev_init(struct net_device *dev)
    {
    eth_hw_addr_random(dev);
    eth_broadcast_addr(dev.broadcast);
    netdev_lockdep_set_classes(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_dev_uninit(dev: *mut net_device) {
    static void l2tp_eth_dev_uninit(struct net_device *dev)
    {
    struct l2tp_eth *priv = netdev_priv(dev);
    struct l2tp_eth_sess *spriv;
    spriv = l2tp_session_priv(priv.session);
    RCU_INIT_POINTER(spriv.dev, core::ptr::null_mut());
// No need for synchronize_net() here. We're called by
// unregister_netdev*(), which does the synchronisation for us.
//
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_dev_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t l2tp_eth_dev_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct l2tp_eth *priv = netdev_priv(dev);
    struct l2tp_session *session = priv.session;
    let mut len: c_uint = skb.len;
    let mut ret: c_int = l2tp_xmit_skb(session, skb);
    if (likely(ret == NET_XMIT_SUCCESS))
    dev_dstats_tx_add(dev, len);
    else
    dev_dstats_tx_dropped(dev);
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops l2tp_eth_netdev_ops = {
    .ndo_init		= l2tp_eth_dev_init,
    .ndo_uninit		= l2tp_eth_dev_uninit,
    .ndo_start_xmit		= l2tp_eth_dev_xmit,
    .ndo_set_mac_address	= eth_mac_addr,
    };
    static const struct device_type l2tpeth_type = {
    .name = "l2tpeth",
    };
#[no_mangle]
unsafe extern "C" fn l2tp_eth_dev_setup(dev: *mut net_device) {
    static void l2tp_eth_dev_setup(struct net_device *dev)
    {
    SET_NETDEV_DEVTYPE(dev, &l2tpeth_type);
    ether_setup(dev);
    dev.priv_flags		&= ~IFF_TX_SKB_SHARING;
    dev.lltx		= true;
    dev.netdev_ops		= &l2tp_eth_netdev_ops;
    dev.needs_free_netdev	= true;
    dev.pcpu_stat_type	= NETDEV_PCPU_STAT_DSTATS;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_dev_recv(session: *mut l2tp_session, skb: *mut sk_buff, data_len: c_int) {
    static void l2tp_eth_dev_recv(struct l2tp_session *session, struct sk_buff *skb, int data_len)
    {
    struct l2tp_eth_sess *spriv = l2tp_session_priv(session);
    struct net_device *dev;
    if (!pskb_may_pull(skb, ETH_HLEN))
    goto error;
    secpath_reset(skb);
// checksums verified by L2TP
    skb.ip_summed = CHECKSUM_NONE;
// drop outer flow-hash
    skb_clear_hash(skb);
    skb_dst_drop(skb);
    nf_reset_ct(skb);
    rcu_read_lock();
    dev = rcu_dereference(spriv.dev);
    if (!dev)
    goto error_rcu;
    if (dev_forward_skb(dev, skb) == NET_RX_SUCCESS)
    dev_dstats_rx_add(dev, data_len);
    else
    DEV_STATS_INC(dev, rx_errors);
    rcu_read_unlock();
    return;
    error_rcu:
    rcu_read_unlock();
    error:
    kfree_skb(skb);
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_delete(session: *mut l2tp_session) {
    static void l2tp_eth_delete(struct l2tp_session *session)
    {
    struct l2tp_eth_sess *spriv;
    struct net_device *dev;
    if (session) {
    spriv = l2tp_session_priv(session);
    rtnl_lock();
    dev = rtnl_dereference(spriv.dev);
    if (dev) {
    unregister_netdevice(dev);
    rtnl_unlock();
    module_put(THIS_MODULE);
    } else {
    rtnl_unlock();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_show(m: *mut seq_file, arg: *mut c_void) {
    static void l2tp_eth_show(struct seq_file *m, void *arg)
    {
    struct l2tp_session *session = arg;
    struct l2tp_eth_sess *spriv = l2tp_session_priv(session);
    struct net_device *dev;
    rcu_read_lock();
    dev = rcu_dereference(spriv.dev);
    if (!dev) {
    rcu_read_unlock();
    return;
    }
    dev_hold(dev);
    rcu_read_unlock();
    seq_printf(m, "   interface %s\n", dev.name);
    dev_put(dev);
    }
    static void l2tp_eth_adjust_mtu(struct l2tp_tunnel *tunnel,
    struct l2tp_session *session,
    struct net_device *dev)
    {
    let mut overhead: c_uint = 0;
    let mut l3_overhead: u32 = 0;
    u32 mtu;
// if the encap is UDP, account for UDP header size
    if (tunnel.encap == L2TP_ENCAPTYPE_UDP) {
    overhead += sizeof(struct udphdr);
    dev.needed_headroom += sizeof(struct udphdr);
    }
    lock_sock(tunnel.sock);
    l3_overhead = kernel_sock_ip_overhead(tunnel.sock);
    release_sock(tunnel.sock);
    if (l3_overhead == 0) {
// L3 Overhead couldn't be identified, this could be
// because tunnel->sock was NULL or the socket's
// address family was not IPv4 or IPv6,
// dev mtu stays at 1500.
//
    return;
    }
// Adjust MTU, factor overhead - underlay L3, overlay L2 hdr
// UDP overhead, if any, was already factored in above.
//
    overhead += session.hdr_len + ETH_HLEN + l3_overhead;
    mtu = l2tp_tunnel_dst_mtu(tunnel) - overhead;
    if (mtu < dev.min_mtu || mtu > dev.max_mtu)
    dev.mtu = ETH_DATA_LEN - overhead;
    else
    dev.mtu = mtu;
    dev.needed_headroom += session.hdr_len;
    }
    static int l2tp_eth_create(struct net *net, struct l2tp_tunnel *tunnel,
    u32 session_id, u32 peer_session_id,
    struct l2tp_session_cfg *cfg)
    {
    unsigned char name_assign_type;
    struct net_device *dev;
    char name[IFNAMSIZ];
    struct l2tp_session *session;
    struct l2tp_eth *priv;
    struct l2tp_eth_sess *spriv;
    int rc;
    if (cfg.ifname) {
    strscpy(name, cfg.ifname);
    name_assign_type = NET_NAME_USER;
    } else {
    strscpy(name, L2TP_ETH_DEV_NAME);
    name_assign_type = NET_NAME_ENUM;
    }
    session = l2tp_session_create(sizeof(*spriv), tunnel, session_id,
    peer_session_id, cfg);
    if (IS_ERR(session)) {
    rc = PTR_ERR(session);
    goto err;
    }
    dev = alloc_netdev(sizeof(*priv), name, name_assign_type,
    l2tp_eth_dev_setup);
    if (!dev) {
    rc = -ENOMEM;
    goto err_sess;
    }
    dev_net_set(dev, net);
    dev.min_mtu = 0;
    dev.max_mtu = ETH_MAX_MTU;
    l2tp_eth_adjust_mtu(tunnel, session, dev);
    priv = netdev_priv(dev);
    priv.session = session;
    session.recv_skb = l2tp_eth_dev_recv;
    session.session_close = l2tp_eth_delete;
    if (IS_ENABLED(CONFIG_L2TP_DEBUGFS))
    session.show = l2tp_eth_show;
    spriv = l2tp_session_priv(session);
    refcount_inc(&session.ref_count);
    rtnl_lock();
// Register both device and session while holding the rtnl lock. This
// ensures that l2tp_eth_delete() will see that there's a device to
// unregister, even if it happened to run before we assign spriv->dev.
//
    rc = l2tp_session_register(session, tunnel);
    if (rc < 0) {
    rtnl_unlock();
    goto err_sess_dev;
    }
    rc = register_netdevice(dev);
    if (rc < 0) {
    rtnl_unlock();
    l2tp_session_delete(session);
    l2tp_session_put(session);
    free_netdev(dev);
    return rc;
    }
    strscpy(session.ifname, dev.name, IFNAMSIZ);
    rcu_assign_pointer(spriv.dev, dev);
    rtnl_unlock();
    l2tp_session_put(session);
    __module_get(THIS_MODULE);
    return 0;
    err_sess_dev:
    l2tp_session_put(session);
    free_netdev(dev);
    err_sess:
    l2tp_session_put(session);
    err:
    return rc;
    }
    static const struct l2tp_nl_cmd_ops l2tp_eth_nl_cmd_ops = {
    .session_create	= l2tp_eth_create,
    .session_delete	= l2tp_session_delete,
    };
#[no_mangle]
unsafe extern "C" fn l2tp_eth_init() -> int __init {
    static int __init l2tp_eth_init(void)
    {
    let mut err: c_int = 0;
    err = l2tp_nl_register_ops(L2TP_PWTYPE_ETH, &l2tp_eth_nl_cmd_ops);
    if (err)
    goto err;
    pr_info("L2TP ethernet pseudowire support (L2TPv3)\n");
    return 0;
    err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn l2tp_eth_exit() -> void __exit {
    static void __exit l2tp_eth_exit(void)
    {
    l2tp_nl_unregister_ops(L2TP_PWTYPE_ETH);
    }
    module_init(l2tp_eth_init);
    module_exit(l2tp_eth_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Chapman <jchapman@katalix.com>");
    MODULE_DESCRIPTION("L2TP ethernet pseudowire driver");
    MODULE_VERSION("1.0");
    MODULE_ALIAS_L2TP_PWTYPE(5);
