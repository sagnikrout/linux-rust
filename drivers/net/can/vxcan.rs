//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/vxcan.c
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
// vxcan.c - Virtual CAN Tunnel for cross namespace communication
//
// This code is derived from drivers/net/can/vcan.c for the virtual CAN
// specific parts and from drivers/net/veth.c to implement the netlink API
// for network interface pairs in a common and established way.
//
// Copyright (c) 2017 Oliver Hartkopp <socketcan@hartkopp.net>
//

    MODULE_DESCRIPTION("Virtual CAN Tunnel");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Oliver Hartkopp <socketcan@hartkopp.net>");
    MODULE_ALIAS_RTNL_LINK(DRV_NAME);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxcan_priv {
    pub peer: *mut net_device __rcu,
    pub peer_tracker: netdevice_tracker,
}

#[no_mangle]
unsafe extern "C" fn vxcan_xmit(oskb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t vxcan_xmit(struct sk_buff *oskb, struct net_device *dev)
    {
    struct vxcan_priv *priv = netdev_priv(dev);
    struct net_device *peer;
    struct net_device_stats *peerstats, *srcstats = &dev.stats;
    struct can_skb_ext *csx;
    struct sk_buff *skb;
    unsigned int len;
    if (can_dropped_invalid_skb(dev, oskb))
    return NETDEV_TX_OK;
    rcu_read_lock();
    peer = rcu_dereference(priv.peer);
    if (unlikely(!peer)) {
    kfree_skb(oskb);
    dev.stats.tx_dropped++;
    goto out_unlock;
    }
    skb_tx_timestamp(oskb);
    skb = skb_clone(oskb, GFP_ATOMIC);
    if (skb) {
    consume_skb(oskb);
    } else {
    kfree_skb(oskb);
    goto out_unlock;
    }
// the cloned skb points to the skb extension of the already cloned
// oskb with an increased refcount. skb_ext_add() creates a copy to
// separate the skb extension data which is needed to start with a
// fresh can_gw_hops counter in the other namespace.
//
    csx = skb_ext_add(skb, SKB_EXT_CAN);
    if (!csx) {
    kfree_skb(skb);
    goto out_unlock;
    }
// reset CAN GW hop counter
    csx.can_gw_hops = 0;
    skb.pkt_type   = PACKET_BROADCAST;
    skb.dev        = peer;
    skb.ip_summed  = CHECKSUM_UNNECESSARY;
    len = can_skb_get_data_len(skb);
    if (netif_rx(skb) == NET_RX_SUCCESS) {
    srcstats.tx_packets++;
    srcstats.tx_bytes += len;
    peerstats = &peer.stats;
    peerstats.rx_packets++;
    peerstats.rx_bytes += len;
    }
    out_unlock:
    rcu_read_unlock();
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn vxcan_open(dev: *mut net_device) -> c_int {
    static int vxcan_open(struct net_device *dev)
    {
    struct vxcan_priv *priv = netdev_priv(dev);
    struct net_device *peer = rtnl_dereference(priv.peer);
    if (!peer)
    return -ENOTCONN;
    if (peer.flags & IFF_UP) {
    netif_carrier_on(dev);
    netif_carrier_on(peer);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vxcan_close(dev: *mut net_device) -> c_int {
    static int vxcan_close(struct net_device *dev)
    {
    struct vxcan_priv *priv = netdev_priv(dev);
    struct net_device *peer = rtnl_dereference(priv.peer);
    netif_carrier_off(dev);
    if (peer)
    netif_carrier_off(peer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vxcan_get_iflink(dev: *const net_device) -> c_int {
    static int vxcan_get_iflink(const struct net_device *dev)
    {
    struct vxcan_priv *priv = netdev_priv(dev);
    struct net_device *peer;
    int iflink;
    rcu_read_lock();
    peer = rcu_dereference(priv.peer);
    iflink = peer ? READ_ONCE(peer.ifindex) : 0;
    rcu_read_unlock();
    return iflink;
    }
#[no_mangle]
unsafe extern "C" fn vxcan_set_cap_info(dev: *mut net_device) {
    static void vxcan_set_cap_info(struct net_device *dev)
    {
    let mut can_cap: u32 = CAN_CAP_CC;
    if (dev.mtu > CAN_MTU)
    can_cap |= CAN_CAP_FD;
    if (dev.mtu >= CANXL_MIN_MTU)
    can_cap |= CAN_CAP_XL;
    can_set_cap(dev, can_cap);
    }
#[no_mangle]
unsafe extern "C" fn vxcan_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int vxcan_change_mtu(struct net_device *dev, int new_mtu)
    {
// Do not allow changing the MTU while running
    if (dev.flags & IFF_UP)
    return -EBUSY;
    if (new_mtu != CAN_MTU && new_mtu != CANFD_MTU &&
    !can_is_canxl_dev_mtu(new_mtu))
    return -EINVAL;
    WRITE_ONCE(dev.mtu, new_mtu);
    vxcan_set_cap_info(dev);
    return 0;
    }
    static const struct net_device_ops vxcan_netdev_ops = {
    .ndo_open	= vxcan_open,
    .ndo_stop	= vxcan_close,
    .ndo_start_xmit	= vxcan_xmit,
    .ndo_get_iflink	= vxcan_get_iflink,
    .ndo_change_mtu = vxcan_change_mtu,
    };
    static const struct ethtool_ops vxcan_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
#[no_mangle]
unsafe extern "C" fn vxcan_setup(dev: *mut net_device) {
    static void vxcan_setup(struct net_device *dev)
    {
    struct can_ml_priv *can_ml;
    dev.type		= ARPHRD_CAN;
    dev.mtu		= CANXL_MTU;
    dev.hard_header_len	= 0;
    dev.addr_len		= 0;
    dev.tx_queue_len	= 0;
    dev.flags		= IFF_NOARP;
    dev.netdev_ops		= &vxcan_netdev_ops;
    dev.ethtool_ops	= &vxcan_ethtool_ops;
    dev.needs_free_netdev	= true;
    can_ml = netdev_priv(dev) + ALIGN(sizeof(struct vxcan_priv), NETDEV_ALIGN);
    can_set_ml_priv(dev, can_ml);
    vxcan_set_cap_info(dev);
    }
// forward declaration for rtnl_create_link()
    static struct rtnl_link_ops vxcan_link_ops;
    static int vxcan_newlink(struct net_device *dev,
    struct rtnl_newlink_params *params,
    struct netlink_ext_ack *extack)
    {
    struct net *peer_net = rtnl_newlink_peer_net(params);
    struct nlattr **data = params.data;
    struct nlattr **tb = params.tb;
    struct vxcan_priv *priv;
    struct net_device *peer;
    struct nlattr *peer_tb[IFLA_MAX + 1], **tbp = tb;
    char ifname[IFNAMSIZ];
    unsigned char name_assign_type;
    struct ifinfomsg *ifmp = core::ptr::null_mut();
    int err;
// register peer device
    if (data && data[VXCAN_INFO_PEER]) {
    struct nlattr *nla_peer = data[VXCAN_INFO_PEER];
    ifmp = nla_data(nla_peer);
    rtnl_nla_parse_ifinfomsg(peer_tb, nla_peer, extack);
    tbp = peer_tb;
    }
    if (ifmp && tbp[IFLA_IFNAME]) {
    nla_strscpy(ifname, tbp[IFLA_IFNAME], IFNAMSIZ);
    name_assign_type = NET_NAME_USER;
    } else {
    snprintf(ifname, IFNAMSIZ, DRV_NAME "%%d");
    name_assign_type = NET_NAME_ENUM;
    }
    peer = rtnl_create_link(peer_net, ifname, name_assign_type,
    &vxcan_link_ops, tbp, extack);
    if (IS_ERR(peer))
    return PTR_ERR(peer);
    if (ifmp && dev.ifindex)
    peer.ifindex = ifmp.ifi_index;
    err = register_netdevice(peer);
    if (err < 0) {
    free_netdev(peer);
    return err;
    }
    netif_carrier_off(peer);
    err = rtnl_configure_link(peer, ifmp, 0, core::ptr::null_mut());
    if (err < 0)
    goto unregister_network_device;
// register first device
    if (tb[IFLA_IFNAME])
    nla_strscpy(dev.name, tb[IFLA_IFNAME], IFNAMSIZ);
    else
    snprintf(dev.name, IFNAMSIZ, DRV_NAME "%%d");
    err = register_netdevice(dev);
    if (err < 0)
    goto unregister_network_device;
    netif_carrier_off(dev);
// cross link the device pair
    priv = netdev_priv(dev);
    rcu_assign_pointer(priv.peer, peer);
    netdev_hold(peer, &priv.peer_tracker, GFP_KERNEL);
    priv = netdev_priv(peer);
    rcu_assign_pointer(priv.peer, dev);
    netdev_hold(dev, &priv.peer_tracker, GFP_KERNEL);
    return 0;
    unregister_network_device:
    unregister_netdevice(peer);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vxcan_dellink(dev: *mut net_device, head: *mut list_head) {
    static void vxcan_dellink(struct net_device *dev, struct list_head *head)
    {
    netdevice_tracker *peer_tracker;
    struct vxcan_priv *priv;
    struct net_device *peer;
    priv = netdev_priv(dev);
    peer_tracker = &priv.peer_tracker;
    peer = unrcu_pointer(xchg(&priv.peer, core::ptr::null_mut()));
    if (!peer)
    return;
    unregister_netdevice_queue(dev, head);
    priv = netdev_priv(peer);
    dev = unrcu_pointer(xchg(&priv.peer, core::ptr::null_mut()));
    if (dev)
    unregister_netdevice_queue_net(dev_net(dev), peer, head);
    netdev_put(peer, peer_tracker);
    netdev_put(dev, &priv.peer_tracker);
    }
    static const struct nla_policy vxcan_policy[VXCAN_INFO_MAX + 1] = {
    [VXCAN_INFO_PEER] = { .len = sizeof(struct ifinfomsg) },
    };
    static struct net *vxcan_get_link_net(const struct net_device *dev)
    {
    struct vxcan_priv *priv = netdev_priv(dev);
    struct net_device *peer = rtnl_dereference(priv.peer);
    return peer ? dev_net(peer) : dev_net(dev);
    }
    static struct rtnl_link_ops vxcan_link_ops = {
    .kind		= DRV_NAME,
    .priv_size	= ALIGN(sizeof(struct vxcan_priv), NETDEV_ALIGN) + sizeof(struct can_ml_priv),
    .setup		= vxcan_setup,
    .newlink	= vxcan_newlink,
    .dellink	= vxcan_dellink,
    .policy		= vxcan_policy,
    .peer_type	= VXCAN_INFO_PEER,
    .maxtype	= VXCAN_INFO_MAX,
    .get_link_net	= vxcan_get_link_net,
    };
#[no_mangle]
unsafe extern "C" fn vxcan_init() -> __init int {
    static __init int vxcan_init(void)
    {
    pr_info("vxcan: Virtual CAN Tunnel driver\n");
    return rtnl_link_register(&vxcan_link_ops);
    }
#[no_mangle]
unsafe extern "C" fn vxcan_exit() -> __exit void {
    static __exit void vxcan_exit(void)
    {
    rtnl_link_unregister(&vxcan_link_ops);
    }
    module_init(vxcan_init);
    module_exit(vxcan_exit);
