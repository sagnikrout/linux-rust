//! Automatically rewritten from C to Rust
//! Source: drivers/net/pfcp.c
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
//
// PFCP according to 3GPP TS 29.244
//
// Copyright (C) 2022, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfcp_dev {
    pub list: list_head,
    pub sk: *mut sock,
    pub dev: *mut net_device,
    pub net: *mut net,
    pub gro_cells: gro_cells,
}

    static unsigned int pfcp_net_id __read_mostly;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfcp_net {
    pub pfcp_dev_list: list_head,
    pub lock: mutex,
}

    static void
    pfcp_session_recv(struct pfcp_dev *pfcp, struct sk_buff *skb,
    struct pfcp_metadata *md)
    {
    struct pfcphdr_session *unparsed = pfcp_hdr_session(skb);
    md.seid = unparsed.seid;
    md.type = PFCP_TYPE_SESSION;
    }
    static void
    pfcp_node_recv(struct pfcp_dev *pfcp, struct sk_buff *skb,
    struct pfcp_metadata *md)
    {
    md.type = PFCP_TYPE_NODE;
    }
#[no_mangle]
unsafe extern "C" fn pfcp_encap_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int pfcp_encap_recv(struct sock *sk, struct sk_buff *skb)
    {
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    struct metadata_dst *tun_dst;
    struct pfcp_metadata *md;
    struct pfcphdr *unparsed;
    struct pfcp_dev *pfcp;
    if (unlikely(!pskb_may_pull(skb, PFCP_HLEN)))
    goto drop;
    pfcp = rcu_dereference_sk_user_data(sk);
    if (unlikely(!pfcp))
    goto drop;
    unparsed = pfcp_hdr(skb);
    ip_tunnel_flags_zero(flags);
    tun_dst = udp_tun_rx_dst(skb, sk.sk_family, flags, 0,
    sizeof(*md));
    if (unlikely(!tun_dst))
    goto drop;
    md = ip_tunnel_info_opts(&tun_dst.u.tun_info);
    if (unlikely(!md))
    goto drop;
    if (unparsed.flags & PFCP_SEID_FLAG)
    pfcp_session_recv(pfcp, skb, md);
    else
    pfcp_node_recv(pfcp, skb, md);
    __set_bit(IP_TUNNEL_PFCP_OPT_BIT, tun_dst.u.tun_info.key.tun_flags);
    tun_dst.u.tun_info.options_len = sizeof(*md);
    if (unlikely(iptunnel_pull_header(skb, PFCP_HLEN, skb.protocol,
    !net_eq(sock_net(sk),
    dev_net(pfcp.dev)))))
    goto drop;
    skb_dst_set(skb, (struct dst_entry *)tun_dst);
    skb_reset_network_header(skb);
    skb_reset_mac_header(skb);
    skb.dev = pfcp.dev;
    gro_cells_receive(&pfcp.gro_cells, skb);
    return 0;
    drop:
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pfcp_del_sock(pfcp: *mut pfcp_dev) {
    static void pfcp_del_sock(struct pfcp_dev *pfcp)
    {
    udp_tunnel_sock_release(pfcp.sk);
    pfcp.sk = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pfcp_dev_uninit(dev: *mut net_device) {
    static void pfcp_dev_uninit(struct net_device *dev)
    {
    struct pfcp_dev *pfcp = netdev_priv(dev);
    gro_cells_destroy(&pfcp.gro_cells);
    pfcp_del_sock(pfcp);
    }
#[no_mangle]
unsafe extern "C" fn pfcp_dev_init(dev: *mut net_device) -> c_int {
    static int pfcp_dev_init(struct net_device *dev)
    {
    struct pfcp_dev *pfcp = netdev_priv(dev);
    pfcp.dev = dev;
    return gro_cells_init(&pfcp.gro_cells, dev);
    }
    static const struct net_device_ops pfcp_netdev_ops = {
    .ndo_init		= pfcp_dev_init,
    .ndo_uninit		= pfcp_dev_uninit,
    .ndo_get_stats64	= dev_get_tstats64,
    };
    static const struct device_type pfcp_type = {
    .name = "pfcp",
    };
#[no_mangle]
unsafe extern "C" fn pfcp_link_setup(dev: *mut net_device) {
    static void pfcp_link_setup(struct net_device *dev)
    {
    dev.netdev_ops = &pfcp_netdev_ops;
    dev.needs_free_netdev = true;
    SET_NETDEV_DEVTYPE(dev, &pfcp_type);
    dev.hard_header_len = 0;
    dev.addr_len = 0;
    dev.type = ARPHRD_NONE;
    dev.flags = IFF_POINTOPOINT | IFF_NOARP | IFF_MULTICAST;
    dev.priv_flags |= IFF_NO_QUEUE;
    dev.pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS;
    netif_keep_dst(dev);
    }
    static struct sock *pfcp_create_sock(struct pfcp_dev *pfcp)
    {
    let mut tuncfg: udp_tunnel_sock_cfg = {};
    struct udp_port_cfg udp_conf = {
    .local_ip.s_addr	= htonl(INADDR_ANY),
    .family			= AF_INET,
    };
    struct net *net = pfcp.net;
    struct socket *sock;
    int err;
    udp_conf.local_udp_port = htons(PFCP_PORT);
    err = udp_sock_create(net, &udp_conf, &sock);
    if (err)
    return ERR_PTR(err);
    tuncfg.sk_user_data = pfcp;
    tuncfg.encap_rcv = pfcp_encap_recv;
    tuncfg.encap_type = 1;
    setup_udp_tunnel_sock(net, sock.sk, &tuncfg);
    return sock.sk;
    }
#[no_mangle]
unsafe extern "C" fn pfcp_add_sock(pfcp: *mut pfcp_dev) -> c_int {
    static int pfcp_add_sock(struct pfcp_dev *pfcp)
    {
    pfcp.sk = pfcp_create_sock(pfcp);
    return PTR_ERR_OR_ZERO(pfcp.sk);
    }
    static int pfcp_newlink(struct net_device *dev,
    struct rtnl_newlink_params *params,
    struct netlink_ext_ack *extack)
    {
    struct net *link_net = rtnl_newlink_link_net(params);
    struct pfcp_dev *pfcp = netdev_priv(dev);
    struct pfcp_net *pn;
    int err;
    pfcp.net = link_net;
    err = pfcp_add_sock(pfcp);
    if (err) {
    netdev_dbg(dev, "failed to add pfcp socket %d\n", err);
    goto exit_err;
    }
    err = register_netdevice(dev);
    if (err) {
    netdev_dbg(dev, "failed to register pfcp netdev %d\n", err);
    goto exit_del_pfcp_sock;
    }
    pn = net_generic(link_net, pfcp_net_id);
    mutex_lock(&pn.lock);
    list_add(&pfcp.list, &pn.pfcp_dev_list);
    mutex_unlock(&pn.lock);
    netdev_dbg(dev, "registered new PFCP interface\n");
    return 0;
    exit_del_pfcp_sock:
    pfcp_del_sock(pfcp);
    synchronize_rcu();
    exit_err:
    pfcp.net = core::ptr::null_mut();
    return err;
    }
    static void __pfcp_dellink(struct net *net, struct net_device *dev,
    struct list_head *head)
    {
    struct pfcp_dev *pfcp = netdev_priv(dev);
    list_del_init(&pfcp.list);
    unregister_netdevice_queue_net(net, dev, head);
    }
#[no_mangle]
unsafe extern "C" fn pfcp_dellink(dev: *mut net_device, head: *mut list_head) {
    static void pfcp_dellink(struct net_device *dev, struct list_head *head)
    {
    struct pfcp_dev *pfcp = netdev_priv(dev);
    struct pfcp_net *pn;
    pn = net_generic(pfcp.net, pfcp_net_id);
    mutex_lock(&pn.lock);
    if (!list_empty(&pfcp.list))
    __pfcp_dellink(dev_net(dev), dev, head);
    mutex_unlock(&pn.lock);
    }
    static struct rtnl_link_ops pfcp_link_ops __read_mostly = {
    .kind		= "pfcp",
    .priv_size	= sizeof(struct pfcp_dev),
    .setup		= pfcp_link_setup,
    .newlink	= pfcp_newlink,
    .dellink	= pfcp_dellink,
    };
#[no_mangle]
unsafe extern "C" fn pfcp_net_init(net: *mut net) -> int __net_init {
    static int __net_init pfcp_net_init(struct net *net)
    {
    struct pfcp_net *pn = net_generic(net, pfcp_net_id);
    INIT_LIST_HEAD(&pn.pfcp_dev_list);
    mutex_init(&pn.lock);
    return 0;
    }
    static void __net_exit pfcp_net_exit_rtnl(struct net *net,
    struct list_head *dev_to_kill)
    {
    struct pfcp_net *pn = net_generic(net, pfcp_net_id);
    struct pfcp_dev *pfcp, *pfcp_next;
    mutex_lock(&pn.lock);
    list_for_each_entry_safe(pfcp, pfcp_next, &pn.pfcp_dev_list, list)
    __pfcp_dellink(net, pfcp.dev, dev_to_kill);
    mutex_unlock(&pn.lock);
    }
    static struct pernet_operations pfcp_net_ops = {
    .init = pfcp_net_init,
    .exit_rtnl = pfcp_net_exit_rtnl,
    .id = &pfcp_net_id,
    .size = sizeof(struct pfcp_net),
    };
#[no_mangle]
unsafe extern "C" fn pfcp_init() -> int __init {
    static int __init pfcp_init(void)
    {
    int err;
    err = register_pernet_subsys(&pfcp_net_ops);
    if (err)
    goto exit_err;
    err = rtnl_link_register(&pfcp_link_ops);
    if (err)
    goto exit_unregister_subsys;
    return 0;
    exit_unregister_subsys:
    unregister_pernet_subsys(&pfcp_net_ops);
    exit_err:
    pr_err("loading PFCP module failed: err %d\n", err);
    return err;
    }
    late_initcall(pfcp_init);
#[no_mangle]
unsafe extern "C" fn pfcp_exit() -> void __exit {
    static void __exit pfcp_exit(void)
    {
    rtnl_link_unregister(&pfcp_link_ops);
    unregister_pernet_subsys(&pfcp_net_ops);
    pr_info("PFCP module unloaded\n");
    }
    module_exit(pfcp_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Wojciech Drewek <wojciech.drewek@intel.com>");
    MODULE_DESCRIPTION("Interface driver for PFCP encapsulated traffic");
    MODULE_ALIAS_RTNL_LINK("pfcp");
