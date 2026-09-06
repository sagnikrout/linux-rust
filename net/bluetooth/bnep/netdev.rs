//! Automatically rewritten from C to Rust
//! Source: net/bluetooth/bnep/netdev.c
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
    BNEP implementation for Linux Bluetooth stack (BlueZ).
    Copyright (C) 2001-2002 Inventel Systemes
    Written 2001-2002 by
    Clément Moreau <clement.moreau@inventel.fr>
    David Libault  <david.libault@inventel.fr>
    Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
    OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
    IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
    CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
    WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
    ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
    OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
    ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
    COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
    SOFTWARE IS DISCLAIMED.
//

pub const BNEP_TX_QUEUE_LEN: c_int = 20;
#[no_mangle]
unsafe extern "C" fn bnep_net_open(dev: *mut net_device) -> c_int {
    static int bnep_net_open(struct net_device *dev)
    {
    netif_start_queue(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bnep_net_close(dev: *mut net_device) -> c_int {
    static int bnep_net_close(struct net_device *dev)
    {
    netif_stop_queue(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bnep_net_set_mc_list(dev: *mut net_device) {
    static void bnep_net_set_mc_list(struct net_device *dev)
    {

    struct bnep_session *s = netdev_priv(dev);
    struct sock *sk = s.sock.sk;
    struct bnep_set_filter_req *r;
    struct sk_buff *skb;
    int size;
    BT_DBG("%s mc_count %d", dev.name, netdev_mc_count(dev));
    size = sizeof(*r) + (BNEP_MAX_MULTICAST_FILTERS + 1) * ETH_ALEN * 2;
    skb  = alloc_skb(size, GFP_ATOMIC);
    if (!skb) {
    BT_ERR("%s Multicast list allocation failed", dev.name);
    return;
    }
    r = (void *) skb.data;
    __skb_put(skb, sizeof(*r));
    r.type = BNEP_CONTROL;
    r.ctrl = BNEP_FILTER_MULTI_ADDR_SET;
    if (dev.flags & (IFF_PROMISC | IFF_ALLMULTI)) {
    u8 start[ETH_ALEN] = { 0x01 };
// Request all addresses
    __skb_put_data(skb, start, ETH_ALEN);
    __skb_put_data(skb, dev.broadcast, ETH_ALEN);
    r.len = htons(ETH_ALEN * 2);
    } else {
    struct netdev_hw_addr *ha;
    int i, len = skb.len;
    if (dev.flags & IFF_BROADCAST) {
    __skb_put_data(skb, dev.broadcast, ETH_ALEN);
    __skb_put_data(skb, dev.broadcast, ETH_ALEN);
    }
// FIXME: We should group addresses here.
    i = 0;
    netdev_for_each_mc_addr(ha, dev) {
    if (i == BNEP_MAX_MULTICAST_FILTERS)
    break;
    __skb_put_data(skb, ha.addr, ETH_ALEN);
    __skb_put_data(skb, ha.addr, ETH_ALEN);
    i++;
    }
    r.len = htons(skb.len - len);
    }
    skb_queue_tail(&sk.sk_write_queue, skb);
    wake_up_interruptible(sk_sleep(sk));

    }
#[no_mangle]
unsafe extern "C" fn bnep_net_set_mac_addr(dev: *mut net_device, arg: *mut c_void) -> c_int {
    static int bnep_net_set_mac_addr(struct net_device *dev, void *arg)
    {
    BT_DBG("%s", dev.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bnep_net_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void bnep_net_timeout(struct net_device *dev, unsigned int txqueue)
    {
    BT_DBG("net_timeout");
    netif_wake_queue(dev);
    }

#[no_mangle]
unsafe extern "C" fn bnep_net_mc_filter(skb: *mut sk_buff, s: *mut bnep_session) -> c_int {
    static int bnep_net_mc_filter(struct sk_buff *skb, struct bnep_session *s)
    {
    struct ethhdr *eh = (void *) skb.data;
    if ((eh.h_dest[0] & 1) && !test_bit(bnep_mc_hash(eh.h_dest), (ulong *) &s.mc_filter))
    return 1;
    return 0;
    }

// Determine ether protocol. Based on eth_type_trans.
#[no_mangle]
unsafe extern "C" fn bnep_net_eth_proto(skb: *mut sk_buff) -> u16 {
    static u16 bnep_net_eth_proto(struct sk_buff *skb)
    {
    struct ethhdr *eh = (void *) skb.data;
    let mut proto: u16 = ntohs(eh.h_proto);
    if (proto >= ETH_P_802_3_MIN)
    return proto;
    if (get_unaligned((__be16 *) skb.data) == htons(0xFFFF))
    return ETH_P_802_3;
    return ETH_P_802_2;
    }
#[no_mangle]
unsafe extern "C" fn bnep_net_proto_filter(skb: *mut sk_buff, s: *mut bnep_session) -> c_int {
    static int bnep_net_proto_filter(struct sk_buff *skb, struct bnep_session *s)
    {
    let mut proto: u16 = bnep_net_eth_proto(skb);
    struct bnep_proto_filter *f = s.proto_filter;
    int i;
    for (i = 0; i < BNEP_MAX_PROTO_FILTERS && f[i].end; i++) {
    if (proto >= f[i].start && proto <= f[i].end)
    return 0;
    }
    BT_DBG("BNEP: filtered skb %p, proto 0x%.4x", skb, proto);
    return 1;
    }

    static netdev_tx_t bnep_net_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct bnep_session *s = netdev_priv(dev);
    struct sock *sk = s.sock.sk;
    BT_DBG("skb %p, dev %p", skb, dev);

    if (bnep_net_mc_filter(skb, s)) {
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }

    if (bnep_net_proto_filter(skb, s)) {
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }

//
// We cannot send L2CAP packets from here as we are potentially in a bh.
// So we have to queue them and wake up session thread which is sleeping
// on the sk_sleep(sk).
//
    netif_trans_update(dev);
    skb_queue_tail(&sk.sk_write_queue, skb);
    wake_up_interruptible(sk_sleep(sk));
    if (skb_queue_len(&sk.sk_write_queue) >= BNEP_TX_QUEUE_LEN) {
    BT_DBG("tx queue is full");
// Stop queuing.
// Session thread will do netif_wake_queue()
    netif_stop_queue(dev);
    }
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops bnep_netdev_ops = {
    .ndo_open            = bnep_net_open,
    .ndo_stop            = bnep_net_close,
    .ndo_start_xmit	     = bnep_net_xmit,
    .ndo_validate_addr   = eth_validate_addr,
    .ndo_set_rx_mode     = bnep_net_set_mc_list,
    .ndo_set_mac_address = bnep_net_set_mac_addr,
    .ndo_tx_timeout      = bnep_net_timeout,
    };
#[no_mangle]
pub unsafe extern "C" fn bnep_net_setup(dev: *mut net_device) {
    void bnep_net_setup(struct net_device *dev)
    {
    eth_broadcast_addr(dev.broadcast);
    dev.addr_len = ETH_ALEN;
    ether_setup(dev);
    dev.min_mtu = 0;
    dev.max_mtu = ETH_MAX_MTU;
    dev.priv_flags &= ~IFF_TX_SKB_SHARING;
    dev.netdev_ops = &bnep_netdev_ops;
    dev.watchdog_timeo  = HZ * 2;
    }
