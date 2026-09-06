//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/microchip/wilc1000/mon.c
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
//
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_radiotap_hdr {
    pub hdr: ieee80211_radiotap_header_fixed,
    pub rate: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_radiotap_cb_hdr {
    pub hdr: ieee80211_radiotap_header_fixed,
    pub rate: u8,
    pub dump: u8,
    pub tx_flags: u16,
    pub __packed: },

    (1 << IEEE80211_RADIOTAP_TX_FLAGS))
#[no_mangle]
pub unsafe extern "C" fn wilc_wfi_monitor_rx(mon_dev: *mut net_device, buff: *mut u8, size: u32) {
    void wilc_wfi_monitor_rx(struct net_device *mon_dev, u8 *buff, u32 size)
    {
    pub pkt_offset: u32 header,,
    pub NULL: *mut *mut sk_buff skb =,
    pub hdr: *mut wilc_wfi_radiotap_hdr,
    pub cb_hdr: *mut wilc_wfi_radiotap_cb_hdr,
    if (!mon_dev)
    if (!netif_running(mon_dev))
// Get WILC header
    pub HOST_HDR_OFFSET): header = get_unaligned_le32(buff -,
//
// The packet offset field contain info about what type of management
// the frame we are dealing with and ack status
//
    pub header): pkt_offset = FIELD_GET(WILC_PKT_HDR_OFFSET_FIELD,,
    if (pkt_offset & IS_MANAGMEMENT_CALLBACK) {
// hostapd callback mgmt frame
    pub sizeof(*cb_hdr)): *mut skb = dev_alloc_skb(size +,
    if (!skb)
    pub size): skb_put_data(skb, buff,,
    pub sizeof(*cb_hdr)): *mut cb_hdr = skb_push(skb,,
    pub sizeof(*cb_hdr)): *mut memset(cb_hdr, 0,,
    pub /: *mut *mut cb_hdr->hdr.it_version = 0; / PKTHDR_RADIOTAP_VERSION;,
    pub cpu_to_le16(sizeof(*cb_hdr)): *mut cb_hdr->hdr.it_len =,
    pub cpu_to_le32(TX_RADIOTAP_PRESENT): cb_hdr->hdr.it_present =,
    pub 5: cb_hdr->rate =,
    if (pkt_offset & IS_MGMT_STATUS_SUCCES)	{
// success
    pub IEEE80211_RADIOTAP_F_TX_RTS: cb_hdr->tx_flags =,
    } else {
    pub IEEE80211_RADIOTAP_F_TX_FAIL: cb_hdr->tx_flags =,
    }
    } else {
    pub sizeof(*hdr)): *mut skb = dev_alloc_skb(size +,
    if (!skb)
    pub size): skb_put_data(skb, buff,,
    pub sizeof(*hdr)): *mut hdr = skb_push(skb,,
    pub wilc_wfi_radiotap_hdr)): memset(hdr, 0, sizeof(struct,
    pub /: *mut *mut hdr->hdr.it_version = 0; / PKTHDR_RADIOTAP_VERSION;,
    pub cpu_to_le16(sizeof(*hdr)): *mut hdr->hdr.it_len =,
    hdr.hdr.it_present = cpu_to_le32
    pub IEEE80211_RADIOTAP_RATE): (1 <<,
    pub 5: hdr->rate =,
    }
    pub mon_dev: skb->dev =,
    pub CHECKSUM_UNNECESSARY: skb->ip_summed =,
    pub PACKET_OTHERHOST: skb->pkt_type =,
    pub htons(ETH_P_802_2): skb->protocol =,
    pub sizeof(skb->cb)): memset(skb->cb, 0,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_complete_mon_data {
    pub size: c_int,
    pub buff: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn mgmt_tx_complete(priv: *mut c_void, status: c_int) {
    static void mgmt_tx_complete(void *priv, int status)
    {
    struct tx_complete_mon_data *pv_data = priv;
//
// in case of fully hosting mode, the freeing will be done
// in response to the cfg packet
//
    kfree(pv_data.buff);
    kfree(pv_data);
    }
#[no_mangle]
unsafe extern "C" fn mon_mgmt_tx(dev: *mut net_device, buf: *const u8, len: usize) -> c_int {
    static int mon_mgmt_tx(struct net_device *dev, const u8 *buf, size_t len)
    {
    struct tx_complete_mon_data *mgmt_tx = core::ptr::null_mut();
    if (!dev)
    return -EFAULT;
    netif_stop_queue(dev);
    mgmt_tx = kmalloc_obj(*mgmt_tx, GFP_ATOMIC);
    if (!mgmt_tx)
    return -ENOMEM;
    mgmt_tx.buff = kmemdup(buf, len, GFP_ATOMIC);
    if (!mgmt_tx.buff) {
    kfree(mgmt_tx);
    return -ENOMEM;
    }
    mgmt_tx.size = len;
    wilc_wlan_txq_add_mgmt_pkt(dev, mgmt_tx, mgmt_tx.buff, mgmt_tx.size,
    mgmt_tx_complete);
    netif_wake_queue(dev);
    return 0;
    }
    static netdev_tx_t wilc_wfi_mon_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct ieee80211_radiotap_header_fixed *rtap_hdr;
    struct ieee80211_hdr_3addr *hdr;
    unsigned int hdr_len;
    u32 rtap_len, ret = 0;
    struct wilc_wfi_mon_priv  *mon_priv;
    struct sk_buff *skb2;
    struct wilc_wfi_radiotap_cb_hdr *cb_hdr;
    u8 srcadd[ETH_ALEN];
    u8 bssid[ETH_ALEN];
    mon_priv = netdev_priv(dev);
    if (!mon_priv)
    return -EFAULT;
    if (skb.len < sizeof(*rtap_hdr))
    goto drop;
    rtap_hdr = (void *)skb.data;
    if (rtap_hdr.it_version)
    goto drop;
    rtap_len = ieee80211_get_radiotap_len(skb.data);
    if (rtap_len < sizeof(*rtap_hdr) || skb.len < rtap_len)
    goto drop;
    skb_pull(skb, rtap_len);
    hdr_len = ieee80211_get_hdrlen_from_skb(skb);
    if (hdr_len < sizeof(*hdr))
    goto drop;
    hdr = (void *)skb.data;
    if (ieee80211_is_deauth(hdr.frame_control) &&
    is_broadcast_ether_addr(hdr.addr1)) {
    skb2 = dev_alloc_skb(skb.len + sizeof(*cb_hdr));
    if (!skb2)
    return -ENOMEM;
    skb_put_data(skb2, skb.data, skb.len);
    cb_hdr = skb_push(skb2, sizeof(*cb_hdr));
    memset(cb_hdr, 0, sizeof(struct wilc_wfi_radiotap_cb_hdr));
    cb_hdr.hdr.it_version = 0; /* PKTHDR_RADIOTAP_VERSION; */
    cb_hdr.hdr.it_len = cpu_to_le16(sizeof(*cb_hdr));
    cb_hdr.hdr.it_present = cpu_to_le32(TX_RADIOTAP_PRESENT);
    cb_hdr.rate = 5;
    cb_hdr.tx_flags = 0x0004;
    skb2.dev = dev;
    skb_reset_mac_header(skb2);
    skb2.ip_summed = CHECKSUM_UNNECESSARY;
    skb2.pkt_type = PACKET_OTHERHOST;
    skb2.protocol = htons(ETH_P_802_2);
    memset(skb2.cb, 0, sizeof(skb2.cb));
    netif_rx(skb2);
    return 0;
    }
    skb.dev = mon_priv.real_ndev;
    ether_addr_copy(srcadd, hdr.addr2);
    ether_addr_copy(bssid, hdr.addr3);
//
// Identify if data or mgmt packet, if source address and bssid
// fields are equal send it to mgmt frames handler
//
    if (!(memcmp(srcadd, bssid, 6))) {
    ret = mon_mgmt_tx(mon_priv.real_ndev, skb.data, skb.len);
    if (ret)
    netdev_err(dev, "fail to mgmt tx\n");
    dev_kfree_skb(skb);
    } else {
    ret = wilc_mac_xmit(skb, mon_priv.real_ndev);
    }
    return ret;
    drop:
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops wilc_wfi_netdev_ops = {
    .ndo_start_xmit         = wilc_wfi_mon_xmit,
    };
    struct net_device *wilc_wfi_init_mon_interface(struct wilc *wl,
    const char *name,
    struct net_device *real_dev)
    {
    struct wilc_wfi_mon_priv *priv;
// If monitor interface is already initialized, return it
    if (wl.monitor_dev)
    return wl.monitor_dev;
    wl.monitor_dev = alloc_etherdev(sizeof(struct wilc_wfi_mon_priv));
    if (!wl.monitor_dev)
    return core::ptr::null_mut();
    wl.monitor_dev.type = ARPHRD_IEEE80211_RADIOTAP;
    strscpy(wl.monitor_dev.name, name, IFNAMSIZ);
    wl.monitor_dev.netdev_ops = &wilc_wfi_netdev_ops;
    wl.monitor_dev.needs_free_netdev = true;
    if (register_netdevice(wl.monitor_dev)) {
    netdev_err(real_dev, "register_netdevice failed\n");
    free_netdev(wl.monitor_dev);
    return core::ptr::null_mut();
    }
    priv = netdev_priv(wl.monitor_dev);
    priv.real_ndev = real_dev;
    return wl.monitor_dev;
    }
#[no_mangle]
pub unsafe extern "C" fn wilc_wfi_deinit_mon_interface(wl: *mut wilc, rtnl_locked: bool) {
    void wilc_wfi_deinit_mon_interface(struct wilc *wl, bool rtnl_locked)
    {
    if (!wl.monitor_dev)
    return;
    if (rtnl_locked)
    unregister_netdevice(wl.monitor_dev);
    else
    unregister_netdev(wl.monitor_dev);
    wl.monitor_dev = core::ptr::null_mut();
    }
