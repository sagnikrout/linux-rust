//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/amd/xgbe/xgbe-selftest.c
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-3-Clause)
//
// Copyright (c) 2014-2025, Advanced Micro Devices, Inc.
// Copyright (c) 2014, Synopsys, Inc.
// All rights reserved
//
// Author: Raju Rangoju <Raju.Rangoju@amd.com>
//

pub const XGBE_LOOPBACK_NONE: c_int = 0;
pub const XGBE_LOOPBACK_MAC: c_int = 1;
pub const XGBE_LOOPBACK_PHY: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_test {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub lb: c_int,
    pub pdata): *mut *mut int (fn)(struct xgbe_prv_data,
}

    static u8 xgbe_test_id;
    static int xgbe_test_loopback_validate(struct sk_buff *skb,
    struct net_device *ndev,
    struct packet_type *pt,
    struct net_device *orig_ndev)
    {
    struct net_test_priv *tdata = pt.af_packet_priv;
    const unsigned char *dst = tdata.packet.dst;
    const unsigned char *src = tdata.packet.src;
    struct netsfhdr *hdr;
    struct ethhdr *eh;
    struct tcphdr *th;
    struct udphdr *uh;
    struct iphdr *ih;
    int eat;
    skb = skb_unshare(skb, GFP_ATOMIC);
    if (!skb)
    goto out;
    eat = (skb.tail + skb.data_len) - skb.end;
    if (eat > 0 && skb_shared(skb)) {
    skb = skb_share_check(skb, GFP_ATOMIC);
    if (!skb)
    goto out;
    }
    if (skb_linearize(skb))
    goto out;
    if (skb_headlen(skb) < (NET_TEST_PKT_SIZE - ETH_HLEN))
    goto out;
    eh = (struct ethhdr *)skb_mac_header(skb);
    if (dst) {
    if (!ether_addr_equal_unaligned(eh.h_dest, dst))
    goto out;
    }
    if (src) {
    if (!ether_addr_equal_unaligned(eh.h_source, src))
    goto out;
    }
    ih = ip_hdr(skb);
    if (tdata.packet.tcp) {
    if (ih.protocol != IPPROTO_TCP)
    goto out;
    th = (struct tcphdr *)((u8 *)ih + 4 * ih.ihl);
    if (th.dest != htons(tdata.packet.dport))
    goto out;
    hdr = (struct netsfhdr *)((u8 *)th + sizeof(*th));
    } else {
    if (ih.protocol != IPPROTO_UDP)
    goto out;
    uh = (struct udphdr *)((u8 *)ih + 4 * ih.ihl);
    if (uh.dest != htons(tdata.packet.dport))
    goto out;
    hdr = (struct netsfhdr *)((u8 *)uh + sizeof(*uh));
    }
    if (hdr.magic != cpu_to_be64(NET_TEST_PKT_MAGIC))
    goto out;
    if (tdata.packet.id != hdr.id)
    goto out;
    tdata.ok = true;
    complete(&tdata.comp);
    out:
    kfree_skb(skb);
    return 0;
    }
    static int __xgbe_test_loopback(struct xgbe_prv_data *pdata,
    struct net_packet_attrs *attr)
    {
    struct net_test_priv *tdata;
    struct sk_buff *skb = core::ptr::null_mut();
    let mut ret: c_int = 0;
    tdata = kzalloc_obj(*tdata);
    if (!tdata)
    return -ENOMEM;
    tdata.ok = false;
    init_completion(&tdata.comp);
    tdata.pt.type = htons(ETH_P_IP);
    tdata.pt.func = xgbe_test_loopback_validate;
    tdata.pt.dev = pdata.netdev;
    tdata.pt.af_packet_priv = tdata;
    tdata.packet = attr;
    dev_add_pack(&tdata.pt);
    skb = net_test_get_skb(pdata.netdev, xgbe_test_id, attr);
    if (!skb) {
    ret = -ENOMEM;
    goto cleanup;
    }
    xgbe_test_id++;
    ret = dev_direct_xmit(skb, attr.queue_mapping);
    if (ret)
    goto cleanup;
    if (!attr.timeout)
    attr.timeout = NET_LB_TIMEOUT;
    wait_for_completion_timeout(&tdata.comp, attr.timeout);
    ret = tdata.ok ? 0 : -ETIMEDOUT;
    if (ret)
    netdev_err(pdata.netdev, "Response timedout: ret %d\n", ret);
    cleanup:
    dev_remove_pack(&tdata.pt);
    kfree(tdata);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xgbe_test_mac_loopback(pdata: *mut xgbe_prv_data) -> c_int {
    static int xgbe_test_mac_loopback(struct xgbe_prv_data *pdata)
    {
    let mut attr: net_packet_attrs = {};
    attr.dst = pdata.netdev.dev_addr;
    return __xgbe_test_loopback(pdata, &attr);
    }
#[no_mangle]
unsafe extern "C" fn xgbe_test_phy_loopback(pdata: *mut xgbe_prv_data) -> c_int {
    static int xgbe_test_phy_loopback(struct xgbe_prv_data *pdata)
    {
    let mut attr: net_packet_attrs = {};
    int ret;
    if (!pdata.netdev.phydev) {
    netdev_err(pdata.netdev, "phydev not found: cannot start PHY loopback test\n");
    return -EOPNOTSUPP;
    }
    ret = phy_loopback(pdata.netdev.phydev, true, 0);
    if (ret)
    return ret;
    attr.dst = pdata.netdev.dev_addr;
    ret = __xgbe_test_loopback(pdata, &attr);
    phy_loopback(pdata.netdev.phydev, false, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xgbe_test_sph(pdata: *mut xgbe_prv_data) -> c_int {
    static int xgbe_test_sph(struct xgbe_prv_data *pdata)
    {
    let mut attr: net_packet_attrs = {};
    unsigned long cnt_end, cnt_start;
    int ret;
    cnt_start = pdata.ext_stats.rx_split_header_packets;
    if (!pdata.sph) {
    netdev_err(pdata.netdev, "Split Header not enabled\n");
    return -EOPNOTSUPP;
    }
// UDP test
    attr.dst = pdata.netdev.dev_addr;
    attr.tcp = false;
    ret = __xgbe_test_loopback(pdata, &attr);
    if (ret)
    return ret;
    cnt_end = pdata.ext_stats.rx_split_header_packets;
    if (cnt_end <= cnt_start)
    return -EINVAL;
// TCP test
    cnt_start = cnt_end;
    attr.dst = pdata.netdev.dev_addr;
    attr.tcp = true;
    ret = __xgbe_test_loopback(pdata, &attr);
    if (ret)
    return ret;
    cnt_end = pdata.ext_stats.rx_split_header_packets;
    if (cnt_end <= cnt_start)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgbe_test_jumbo(pdata: *mut xgbe_prv_data) -> c_int {
    static int xgbe_test_jumbo(struct xgbe_prv_data *pdata)
    {
    let mut attr: net_packet_attrs = {};
    let mut size: c_int = pdata.rx_buf_size;
    attr.dst = pdata.netdev.dev_addr;
    attr.max_size = size - ETH_FCS_LEN;
    return __xgbe_test_loopback(pdata, &attr);
    }
    static const struct xgbe_test xgbe_selftests[] = {
    {
    .name = "MAC Loopback   ",
    .lb = XGBE_LOOPBACK_MAC,
    .fn = xgbe_test_mac_loopback,
    }, {
    .name = "PHY Loopback   ",
    .lb = XGBE_LOOPBACK_NONE,
    .fn = xgbe_test_phy_loopback,
    }, {
    .name = "Split Header   ",
    .lb = XGBE_LOOPBACK_PHY,
    .fn = xgbe_test_sph,
    }, {
    .name = "Jumbo Frame    ",
    .lb = XGBE_LOOPBACK_PHY,
    .fn = xgbe_test_jumbo,
    },
    };
    void xgbe_selftest_run(struct net_device *dev,
    struct ethtool_test *etest, u64 *buf)
    {
    struct xgbe_prv_data *pdata = netdev_priv(dev);
    let mut count: c_int = xgbe_selftest_get_count(pdata);
    int i, ret;
    memset(buf, 0, sizeof(*buf) * count);
    xgbe_test_id = 0;
    if (etest.flags != ETH_TEST_FL_OFFLINE) {
    netdev_err(pdata.netdev, "Only offline tests are supported\n");
    etest.flags |= ETH_TEST_FL_FAILED;
    return;
    } else if (!netif_carrier_ok(dev)) {
    netdev_err(pdata.netdev,
    "Invalid link, cannot execute tests\n");
    etest.flags |= ETH_TEST_FL_FAILED;
    return;
    }
// Wait for queues drain
    msleep(200);
    for (i = 0; i < count; i++) {
    ret = 0;
    switch (xgbe_selftests[i].lb) {
    case XGBE_LOOPBACK_PHY:
    ret = -EOPNOTSUPP;
    if (dev.phydev)
    ret = phy_loopback(dev.phydev, true, 0);
    if (!ret)
    break;
    fallthrough;
    case XGBE_LOOPBACK_MAC:
    ret = xgbe_enable_mac_loopback(pdata);
    break;
    case XGBE_LOOPBACK_NONE:
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
//
// First tests will always be MAC / PHY loopback.
// If any of them is not supported we abort earlier.
//
    if (ret) {
    netdev_err(pdata.netdev, "Loopback not supported\n");
    etest.flags |= ETH_TEST_FL_FAILED;
    break;
    }
    ret = xgbe_selftests[i].fn(pdata);
    if (ret && (ret != -EOPNOTSUPP))
    etest.flags |= ETH_TEST_FL_FAILED;
    buf[i] = ret;
    switch (xgbe_selftests[i].lb) {
    case XGBE_LOOPBACK_PHY:
    ret = -EOPNOTSUPP;
    if (dev.phydev)
    ret = phy_loopback(dev.phydev, false, 0);
    if (!ret)
    break;
    fallthrough;
    case XGBE_LOOPBACK_MAC:
    xgbe_disable_mac_loopback(pdata);
    break;
    default:
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xgbe_selftest_get_strings(pdata: *mut xgbe_prv_data, data: *mut u8) {
    void xgbe_selftest_get_strings(struct xgbe_prv_data *pdata, u8 *data)
    {
    u8 *p = data;
    int i;
    for (i = 0; i < xgbe_selftest_get_count(pdata); i++)
    ethtool_puts(&p, xgbe_selftests[i].name);
    }
#[no_mangle]
pub unsafe extern "C" fn xgbe_selftest_get_count(pdata: *mut xgbe_prv_data) -> c_int {
    int xgbe_selftest_get_count(struct xgbe_prv_data *pdata)
    {
    return ARRAY_SIZE(xgbe_selftests);
    }
