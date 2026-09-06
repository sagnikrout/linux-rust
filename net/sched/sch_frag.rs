//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_frag.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch_frag_data {
    pub dst: c_ulong,
    pub cb: qdisc_skb_cb,
    pub inner_protocol: __be16,
    pub vlan_tci: u16,
    pub vlan_proto: __be16,
    pub l2_len: c_uint,
    pub l2_data: [u8; VLAN_ETH_HLEN],
    pub skb): *mut *mut int (xmit)(struct sk_buff,
    pub bh_lock: local_lock_t,
}

    static DEFINE_PER_CPU(struct sch_frag_data, sch_frag_data_storage) = {
    .bh_lock = INIT_LOCAL_LOCK(bh_lock),
    };
#[no_mangle]
unsafe extern "C" fn sch_frag_xmit(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int sch_frag_xmit(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    struct sch_frag_data *data = this_cpu_ptr(&sch_frag_data_storage);
    lockdep_assert_held(&data.bh_lock);
    if (skb_cow_head(skb, data.l2_len) < 0) {
    kfree_skb(skb);
    return -ENOMEM;
    }
    __skb_dst_copy(skb, data.dst);
// qdisc_skb_cb(skb) = data->cb;
    skb.inner_protocol = data.inner_protocol;
    if (data.vlan_tci & VLAN_CFI_MASK)
    __vlan_hwaccel_put_tag(skb, data.vlan_proto,
    data.vlan_tci & ~VLAN_CFI_MASK);
    else
    __vlan_hwaccel_clear_tag(skb);
// Reconstruct the MAC header.
    skb_push(skb, data.l2_len);
    memcpy(skb.data, &data.l2_data, data.l2_len);
    skb_postpush_rcsum(skb, skb.data, data.l2_len);
    skb_reset_mac_header(skb);
    return data.xmit(skb);
    }
    static void sch_frag_prepare_frag(struct sk_buff *skb,
    int (*xmit)(struct sk_buff *skb))
    {
    let mut hlen: c_uint = skb_network_offset(skb);
    struct sch_frag_data *data;
    data = this_cpu_ptr(&sch_frag_data_storage);
    data.dst = skb._skb_refdst;
    data.cb = *qdisc_skb_cb(skb);
    data.xmit = xmit;
    data.inner_protocol = skb.inner_protocol;
    if (skb_vlan_tag_present(skb))
    data.vlan_tci = skb_vlan_tag_get(skb) | VLAN_CFI_MASK;
    else
    data.vlan_tci = 0;
    data.vlan_proto = skb.vlan_proto;
    data.l2_len = hlen;
    memcpy(&data.l2_data, skb.data, hlen);
    memset(IPCB(skb), 0, sizeof(struct inet_skb_parm));
    skb_pull(skb, hlen);
    }
    static unsigned int
    sch_frag_dst_get_mtu(const struct dst_entry *dst)
    {
    return dst.dev.mtu;
    }
    static struct dst_ops sch_frag_dst_ops = {
    .family = AF_UNSPEC,
    .mtu = sch_frag_dst_get_mtu,
    };
    static int sch_fragment(struct net *net, struct sk_buff *skb,
    u16 mru, int (*xmit)(struct sk_buff *skb))
    {
    let mut ret: c_int = -1;
    if (skb_network_offset(skb) > VLAN_ETH_HLEN) {
    net_warn_ratelimited("L2 header too long to fragment\n");
    goto err;
    }
    if (skb_protocol(skb, true) == htons(ETH_P_IP)) {
    let mut sch_frag_rt: rtable = { 0 };
    unsigned long orig_dst;
    local_lock_nested_bh(&sch_frag_data_storage.bh_lock);
    sch_frag_prepare_frag(skb, xmit);
    dst_init(&sch_frag_rt.dst, &sch_frag_dst_ops, core::ptr::null_mut(),
    DST_OBSOLETE_NONE, DST_NOCOUNT);
    sch_frag_rt.dst.dev = skb.dev;
    orig_dst = skb._skb_refdst;
    skb_dst_set_noref(skb, &sch_frag_rt.dst);
    IPCB(skb).frag_max_size = mru;
    ret = ip_do_fragment(net, skb.sk, skb, sch_frag_xmit);
    local_unlock_nested_bh(&sch_frag_data_storage.bh_lock);
    refdst_drop(orig_dst);
    } else if (skb_protocol(skb, true) == htons(ETH_P_IPV6)) {
    unsigned long orig_dst;
    struct rt6_info sch_frag_rt;
    local_lock_nested_bh(&sch_frag_data_storage.bh_lock);
    sch_frag_prepare_frag(skb, xmit);
    memset(&sch_frag_rt, 0, sizeof(sch_frag_rt));
    dst_init(&sch_frag_rt.dst, &sch_frag_dst_ops, core::ptr::null_mut(),
    DST_OBSOLETE_NONE, DST_NOCOUNT);
    sch_frag_rt.dst.dev = skb.dev;
    orig_dst = skb._skb_refdst;
    skb_dst_set_noref(skb, &sch_frag_rt.dst);
    IP6CB(skb).frag_max_size = mru;
    ret = ip6_fragment(net, skb.sk, skb, sch_frag_xmit);
    local_unlock_nested_bh(&sch_frag_data_storage.bh_lock);
    refdst_drop(orig_dst);
    } else {
    net_warn_ratelimited("Fail frag %s: eth=%x, MRU=%d, MTU=%d\n",
    netdev_name(skb.dev),
    ntohs(skb_protocol(skb, true)), mru,
    skb.dev.mtu);
    goto err;
    }
    return ret;
    err:
    kfree_skb(skb);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sch_frag_xmit_hook(skb: *mut sk_buff, skb): *mut *mut int (xmit)(struct sk_buff) -> c_int {
    int sch_frag_xmit_hook(struct sk_buff *skb, int (*xmit)(struct sk_buff *skb))
    {
    let mut mru: u16 = tc_skb_cb(skb).mru;
    int err;
    if (mru && skb.len > mru + skb.dev.hard_header_len)
    err = sch_fragment(dev_net(skb.dev), skb, mru, xmit);
    else
    err = xmit(skb);
    return err;
    }
    EXPORT_SYMBOL_GPL(sch_frag_xmit_hook);
