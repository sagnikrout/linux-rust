//! Automatically rewritten from C to Rust
//! Source: drivers/net/nlmon.c
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

#[no_mangle]
unsafe extern "C" fn nlmon_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t nlmon_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    dev_lstats_add(dev, skb.len);
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmon {
    pub nt: netlink_tap,
}

#[no_mangle]
unsafe extern "C" fn nlmon_open(dev: *mut net_device) -> c_int {
    static int nlmon_open(struct net_device *dev)
    {
    struct nlmon *nlmon = netdev_priv(dev);
    nlmon.nt.dev = dev;
    nlmon.nt.module = THIS_MODULE;
    return netlink_add_tap(&nlmon.nt);
    }
#[no_mangle]
unsafe extern "C" fn nlmon_close(dev: *mut net_device) -> c_int {
    static int nlmon_close(struct net_device *dev)
    {
    struct nlmon *nlmon = netdev_priv(dev);
    return netlink_remove_tap(&nlmon.nt);
    }
    static void
    nlmon_get_stats64(struct net_device *dev, struct rtnl_link_stats64 *stats)
    {
    dev_lstats_read(dev, &stats.rx_packets, &stats.rx_bytes);
    }
#[no_mangle]
unsafe extern "C" fn always_on(dev: *mut net_device) -> u32 {
    static u32 always_on(struct net_device *dev)
    {
    return 1;
    }
    static const struct ethtool_ops nlmon_ethtool_ops = {
    .get_link = always_on,
    };
    static const struct net_device_ops nlmon_ops = {
    .ndo_open = nlmon_open,
    .ndo_stop = nlmon_close,
    .ndo_start_xmit = nlmon_xmit,
    .ndo_get_stats64 = nlmon_get_stats64,
    };
#[no_mangle]
unsafe extern "C" fn nlmon_setup(dev: *mut net_device) {
    static void nlmon_setup(struct net_device *dev)
    {
    dev.type = ARPHRD_NETLINK;
    dev.priv_flags |= IFF_NO_QUEUE;
    dev.lltx = true;
    dev.netdev_ops	= &nlmon_ops;
    dev.ethtool_ops = &nlmon_ethtool_ops;
    dev.needs_free_netdev = true;
    dev.features = NETIF_F_SG | NETIF_F_FRAGLIST | NETIF_F_HIGHDMA;
    dev.flags = IFF_NOARP;
    dev.pcpu_stat_type = NETDEV_PCPU_STAT_LSTATS;
// That's rather a softlimit here, which, of course,
// can be altered. Not a real MTU, but what is to be
// expected in most cases.
//
    dev.mtu = NLMSG_GOODSIZE;
    dev.min_mtu = sizeof(struct nlmsghdr);
    }
    static int nlmon_validate(struct nlattr *tb[], struct nlattr *data[],
    struct netlink_ext_ack *extack)
    {
    if (tb[IFLA_ADDRESS])
    return -EINVAL;
    return 0;
    }
    static struct rtnl_link_ops nlmon_link_ops __read_mostly = {
    .kind			= "nlmon",
    .priv_size		= sizeof(struct nlmon),
    .setup			= nlmon_setup,
    .validate		= nlmon_validate,
    };
#[no_mangle]
unsafe extern "C" fn nlmon_register() -> __init int {
    static __init int nlmon_register(void)
    {
    return rtnl_link_register(&nlmon_link_ops);
    }
#[no_mangle]
unsafe extern "C" fn nlmon_unregister() -> __exit void {
    static __exit void nlmon_unregister(void)
    {
    rtnl_link_unregister(&nlmon_link_ops);
    }
    module_init(nlmon_register);
    module_exit(nlmon_unregister);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Daniel Borkmann <dborkman@redhat.com>");
    MODULE_AUTHOR("Mathieu Geli <geli@enseirb.fr>");
    MODULE_DESCRIPTION("Netlink monitoring device");
    MODULE_ALIAS_RTNL_LINK("nlmon");
