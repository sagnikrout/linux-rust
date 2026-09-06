//! Automatically rewritten from C to Rust
//! Source: drivers/net/vsockmon.c
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

// Virtio transport max packet size plus header

    sizeof(struct af_vsockmon_hdr))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsockmon {
    pub vt: vsock_tap,
}

#[no_mangle]
unsafe extern "C" fn vsockmon_open(dev: *mut net_device) -> c_int {
    static int vsockmon_open(struct net_device *dev)
    {
    struct vsockmon *vsockmon = netdev_priv(dev);
    vsockmon.vt.dev = dev;
    vsockmon.vt.module = THIS_MODULE;
    return vsock_add_tap(&vsockmon.vt);
    }
#[no_mangle]
unsafe extern "C" fn vsockmon_close(dev: *mut net_device) -> c_int {
    static int vsockmon_close(struct net_device *dev)
    {
    struct vsockmon *vsockmon = netdev_priv(dev);
    return vsock_remove_tap(&vsockmon.vt);
    }
#[no_mangle]
unsafe extern "C" fn vsockmon_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t vsockmon_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    dev_lstats_add(dev, skb.len);
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    static void
    vsockmon_get_stats64(struct net_device *dev, struct rtnl_link_stats64 *stats)
    {
    dev_lstats_read(dev, &stats.rx_packets, &stats.rx_bytes);
    }
#[no_mangle]
unsafe extern "C" fn vsockmon_is_valid_mtu(new_mtu: c_int) -> c_int {
    static int vsockmon_is_valid_mtu(int new_mtu)
    {
    return new_mtu >= (int)sizeof(struct af_vsockmon_hdr);
    }
#[no_mangle]
unsafe extern "C" fn vsockmon_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int vsockmon_change_mtu(struct net_device *dev, int new_mtu)
    {
    if (!vsockmon_is_valid_mtu(new_mtu))
    return -EINVAL;
    WRITE_ONCE(dev.mtu, new_mtu);
    return 0;
    }
    static const struct net_device_ops vsockmon_ops = {
    .ndo_open = vsockmon_open,
    .ndo_stop = vsockmon_close,
    .ndo_start_xmit = vsockmon_xmit,
    .ndo_get_stats64 = vsockmon_get_stats64,
    .ndo_change_mtu = vsockmon_change_mtu,
    };
#[no_mangle]
unsafe extern "C" fn always_on(dev: *mut net_device) -> u32 {
    static u32 always_on(struct net_device *dev)
    {
    return 1;
    }
    static const struct ethtool_ops vsockmon_ethtool_ops = {
    .get_link = always_on,
    };
#[no_mangle]
unsafe extern "C" fn vsockmon_setup(dev: *mut net_device) {
    static void vsockmon_setup(struct net_device *dev)
    {
    dev.type = ARPHRD_VSOCKMON;
    dev.priv_flags |= IFF_NO_QUEUE;
    dev.lltx = true;
    dev.netdev_ops	= &vsockmon_ops;
    dev.ethtool_ops = &vsockmon_ethtool_ops;
    dev.needs_free_netdev = true;
    dev.features = NETIF_F_SG | NETIF_F_FRAGLIST | NETIF_F_HIGHDMA;
    dev.flags = IFF_NOARP;
    dev.mtu = DEFAULT_MTU;
    dev.pcpu_stat_type = NETDEV_PCPU_STAT_LSTATS;
    }
    static struct rtnl_link_ops vsockmon_link_ops __read_mostly = {
    .kind			= "vsockmon",
    .priv_size		= sizeof(struct vsockmon),
    .setup			= vsockmon_setup,
    };
#[no_mangle]
unsafe extern "C" fn vsockmon_register() -> __init int {
    static __init int vsockmon_register(void)
    {
    return rtnl_link_register(&vsockmon_link_ops);
    }
#[no_mangle]
unsafe extern "C" fn vsockmon_unregister() -> __exit void {
    static __exit void vsockmon_unregister(void)
    {
    rtnl_link_unregister(&vsockmon_link_ops);
    }
    module_init(vsockmon_register);
    module_exit(vsockmon_unregister);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Gerard Garcia <ggarcia@deic.uab.cat>");
    MODULE_DESCRIPTION("Vsock monitoring device. Based on nlmon device.");
    MODULE_ALIAS_RTNL_LINK("vsockmon");
