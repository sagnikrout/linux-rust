//! Automatically rewritten from C to Rust
//! Source: net/xfrm/xfrm_device.c
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
// xfrm_device.c - IPsec device offloading code.
//
// Copyright (c) 2015 secunet Security Networks AG
//
// Author:
// Steffen Klassert <steffen.klassert@secunet.com>
//

    static void __xfrm_transport_prep(struct xfrm_state *x, struct sk_buff *skb,
    unsigned int hsize)
    {
    struct xfrm_offload *xo = xfrm_offload(skb);
    skb_reset_mac_len(skb);
    if (xo.flags & XFRM_GSO_SEGMENT)
    skb.transport_header -= x.props.header_len;
    pskb_pull(skb, skb_transport_offset(skb) + x.props.header_len);
    }
    static void __xfrm_mode_tunnel_prep(struct xfrm_state *x, struct sk_buff *skb,
    unsigned int hsize)
    {
    struct xfrm_offload *xo = xfrm_offload(skb);
    if (xo.flags & XFRM_GSO_SEGMENT)
    skb.transport_header = skb.network_header + hsize;
    skb_reset_mac_len(skb);
    pskb_pull(skb,
    skb.mac_len + x.props.header_len - x.props.enc_hdr_len);
    }
    static void __xfrm_mode_beet_prep(struct xfrm_state *x, struct sk_buff *skb,
    unsigned int hsize)
    {
    struct xfrm_offload *xo = xfrm_offload(skb);
    let mut phlen: c_int = 0;
    if (xo.flags & XFRM_GSO_SEGMENT)
    skb.transport_header = skb.network_header + hsize;
    skb_reset_mac_len(skb);
    if (x.sel.family != AF_INET6) {
    phlen = IPV4_BEET_PHMAXLEN;
    if (x.outer_mode.family == AF_INET6)
    phlen += sizeof(struct ipv6hdr) - sizeof(struct iphdr);
    }
    pskb_pull(skb, skb.mac_len + hsize + (x.props.header_len - phlen));
    }
// Adjust pointers into the packet when IPsec is done at layer2
#[no_mangle]
unsafe extern "C" fn xfrm_outer_mode_prep(x: *mut xfrm_state, skb: *mut sk_buff) {
    static void xfrm_outer_mode_prep(struct xfrm_state *x, struct sk_buff *skb)
    {
    switch (x.outer_mode.encap) {
    case XFRM_MODE_IPTFS:
    case XFRM_MODE_TUNNEL:
    if (x.outer_mode.family == AF_INET)
    return __xfrm_mode_tunnel_prep(x, skb,
    sizeof(struct iphdr));
    if (x.outer_mode.family == AF_INET6)
    return __xfrm_mode_tunnel_prep(x, skb,
    sizeof(struct ipv6hdr));
    break;
    case XFRM_MODE_TRANSPORT:
    if (x.outer_mode.family == AF_INET)
    return __xfrm_transport_prep(x, skb,
    sizeof(struct iphdr));
    if (x.outer_mode.family == AF_INET6)
    return __xfrm_transport_prep(x, skb,
    sizeof(struct ipv6hdr));
    break;
    case XFRM_MODE_BEET:
    if (x.outer_mode.family == AF_INET)
    return __xfrm_mode_beet_prep(x, skb,
    sizeof(struct iphdr));
    if (x.outer_mode.family == AF_INET6)
    return __xfrm_mode_beet_prep(x, skb,
    sizeof(struct ipv6hdr));
    break;
    case XFRM_MODE_ROUTEOPTIMIZATION:
    case XFRM_MODE_IN_TRIGGER:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xmit_xfrm_check_overflow(skb: *mut sk_buff) -> bool {
    static inline bool xmit_xfrm_check_overflow(struct sk_buff *skb)
    {
    struct xfrm_offload *xo = xfrm_offload(skb);
    let mut seq: __u32 = xo.seq.low;
    seq += skb_shinfo(skb).gso_segs;
    if (unlikely(seq < xo.seq.low))
    return true;
    return false;
    }
    struct sk_buff *validate_xmit_xfrm(struct sk_buff *skb, netdev_features_t features, bool *again)
    {
    int err;
    unsigned long flags;
    struct xfrm_state *x;
    struct softnet_data *sd;
    struct sk_buff *skb2, *nskb, *pskb = core::ptr::null_mut();
    let mut esp_features: netdev_features_t = features;
    struct xfrm_offload *xo = xfrm_offload(skb);
    struct net_device *dev = skb.dev;
    struct sec_path *sp;
    if (!xo || (xo.flags & XFRM_XMIT))
    return skb;
    if (!(features & NETIF_F_HW_ESP))
    esp_features = features & ~(NETIF_F_SG | NETIF_F_CSUM_MASK);
    sp = skb_sec_path(skb);
    x = sp.xvec[sp.len - 1];
    if (xo.flags & XFRM_GRO || x.xso.dir == XFRM_DEV_OFFLOAD_IN)
    return skb;
// The packet was sent to HW IPsec packet offload engine,
// but to wrong device. Drop the packet, so it won't skip
// XFRM stack.
//
    if (x.xso.type == XFRM_DEV_OFFLOAD_PACKET && x.xso.dev != dev) {
    kfree_skb(skb);
    dev_core_stats_tx_dropped_inc(dev);
    return core::ptr::null_mut();
    }
    local_irq_save(flags);
    sd = this_cpu_ptr(&softnet_data);
    err = !skb_queue_empty(&sd.xfrm_backlog);
    local_irq_restore(flags);
    if (err) {
// again = true;
    return skb;
    }
    if (skb_is_gso(skb) && (unlikely(x.xso.dev != dev) ||
    unlikely(xmit_xfrm_check_overflow(skb)))) {
    struct sk_buff *segs;
// Packet got rerouted, fixup features and segment it.
    esp_features = esp_features & ~(NETIF_F_HW_ESP | NETIF_F_GSO_ESP);
    segs = skb_gso_segment(skb, esp_features);
    if (IS_ERR(segs)) {
    kfree_skb(skb);
    dev_core_stats_tx_dropped_inc(dev);
    return core::ptr::null_mut();
    } else {
    consume_skb(skb);
    skb = segs;
    }
    }
    if (!skb.next) {
    esp_features |= skb.dev.gso_partial_features;
    xfrm_outer_mode_prep(x, skb);
    xo.flags |= XFRM_DEV_RESUME;
    err = x.type_offload.xmit(x, skb, esp_features);
    if (err) {
    if (err == -EINPROGRESS)
    return ERR_PTR(-EINPROGRESS);
    XFRM_INC_STATS(xs_net(x), LINUX_MIB_XFRMOUTSTATEPROTOERROR);
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
    skb_push(skb, skb.data - skb_mac_header(skb));
    return skb;
    }
    skb_list_walk_safe(skb, skb2, nskb) {
    esp_features |= skb.dev.gso_partial_features;
    skb_mark_not_on_list(skb2);
    xo = xfrm_offload(skb2);
    xo.flags |= XFRM_DEV_RESUME;
    xfrm_outer_mode_prep(x, skb2);
    err = x.type_offload.xmit(x, skb2, esp_features);
    if (!err) {
    skb2.next = nskb;
    } else if (err != -EINPROGRESS) {
    XFRM_INC_STATS(xs_net(x), LINUX_MIB_XFRMOUTSTATEPROTOERROR);
    skb2.next = nskb;
    kfree_skb_list(skb2);
    return core::ptr::null_mut();
    } else {
    if (skb == skb2)
    skb = nskb;
    else
    pskb.next = nskb;
    continue;
    }
    skb_push(skb2, skb2.data - skb_mac_header(skb2));
    pskb = skb2;
    }
// skb_gso_segment() set skb->prev to the last segment, but async
// crypto may have stolen it above without updating ->prev.  Repoint
// it at the last retained segment so validate_xmit_skb_list() does
// not chain onto a segment now owned by the crypto engine.
//
    if (skb)
    skb.prev = pskb;
    return skb ? skb : ERR_PTR(-EINPROGRESS);
    }
    EXPORT_SYMBOL_GPL(validate_xmit_xfrm);
    int xfrm_dev_state_add(struct net *net, struct xfrm_state *x,
    const struct xfrm_user_offload *xuo,
    struct netlink_ext_ack *extack)
    {
    int err;
    struct dst_entry *dst;
    struct net_device *dev;
    struct xfrm_dev_offload *xso = &x.xso;
    xfrm_address_t *saddr;
    xfrm_address_t *daddr;
    bool is_packet_offload;
    if (xuo.flags &
    ~(XFRM_OFFLOAD_IPV6 | XFRM_OFFLOAD_INBOUND | XFRM_OFFLOAD_PACKET)) {
    NL_SET_ERR_MSG(extack, "Unrecognized flags in offload request");
    return -EINVAL;
    }
    if ((xuo.flags & XFRM_OFFLOAD_INBOUND && x.dir == XFRM_SA_DIR_OUT) ||
    (!(xuo.flags & XFRM_OFFLOAD_INBOUND) && x.dir == XFRM_SA_DIR_IN)) {
    NL_SET_ERR_MSG(extack, "Mismatched SA and offload direction");
    return -EINVAL;
    }
    if (xuo.flags & XFRM_OFFLOAD_INBOUND && x.if_id) {
    NL_SET_ERR_MSG(extack, "XFRM if_id is not supported in RX path");
    return -EINVAL;
    }
    is_packet_offload = xuo.flags & XFRM_OFFLOAD_PACKET;
// We don't yet support TFC padding.
    if (x.tfcpad) {
    NL_SET_ERR_MSG(extack, "TFC padding can't be offloaded");
    return -EINVAL;
    }
    dev = dev_get_by_index(net, xuo.ifindex);
    if (!dev) {
    struct xfrm_dst_lookup_params params;
    if (!(xuo.flags & XFRM_OFFLOAD_INBOUND)) {
    saddr = &x.props.saddr;
    daddr = &x.id.daddr;
    } else {
    saddr = &x.id.daddr;
    daddr = &x.props.saddr;
    }
    memset(&params, 0, sizeof(params));
    params.net = net;
    params.saddr = saddr;
    params.daddr = daddr;
    params.mark = xfrm_smark_get(0, x);
    dst = __xfrm_dst_lookup(x.props.family, &params);
    if (IS_ERR(dst))
    return (is_packet_offload) ? -EINVAL : 0;
    dev = dst.dev;
    dev_hold(dev);
    dst_release(dst);
    }
    if (!dev.xfrmdev_ops || !dev.xfrmdev_ops.xdo_dev_state_add) {
    xso.dev = core::ptr::null_mut();
    dev_put(dev);
    return (is_packet_offload) ? -EINVAL : 0;
    }
    if (!is_packet_offload && x.props.flags & XFRM_STATE_ESN &&
    !dev.xfrmdev_ops.xdo_dev_state_advance_esn) {
    NL_SET_ERR_MSG(extack, "Device doesn't support offload with ESN");
    xso.dev = core::ptr::null_mut();
    dev_put(dev);
    return -EINVAL;
    }
    if (!x.type_offload) {
    NL_SET_ERR_MSG(extack, "Type doesn't support offload");
    dev_put(dev);
    return -EINVAL;
    }
    xso.dev = dev;
    xso.ifindex = dev.ifindex;
    netdev_tracker_alloc(dev, &xso.dev_tracker, GFP_ATOMIC);
    if (xuo.flags & XFRM_OFFLOAD_INBOUND)
    xso.dir = XFRM_DEV_OFFLOAD_IN;
    else
    xso.dir = XFRM_DEV_OFFLOAD_OUT;
    if (is_packet_offload)
    xso.type = XFRM_DEV_OFFLOAD_PACKET;
    else
    xso.type = XFRM_DEV_OFFLOAD_CRYPTO;
    err = dev.xfrmdev_ops.xdo_dev_state_add(dev, x, extack);
    if (err) {
    xso.dev = core::ptr::null_mut();
    xso.dir = 0;
    netdev_put(dev, &xso.dev_tracker);
    xso.type = XFRM_DEV_OFFLOAD_UNSPECIFIED;
    xfrm_unset_type_offload(x);
// User explicitly requested packet offload mode and configured
// policy in addition to the XFRM state. So be civil to users,
// and return an error instead of taking fallback path.
//
    if ((err != -EOPNOTSUPP && !is_packet_offload) || is_packet_offload) {
    NL_SET_ERR_MSG_WEAK(extack, "Device failed to offload this state");
    return err;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(xfrm_dev_state_add);
    int xfrm_dev_policy_add(struct net *net, struct xfrm_policy *xp,
    struct xfrm_user_offload *xuo, u8 dir,
    struct netlink_ext_ack *extack)
    {
    struct xfrm_dev_offload *xdo = &xp.xdo;
    struct net_device *dev;
    int err;
    if (!xuo.flags || xuo.flags & ~XFRM_OFFLOAD_PACKET) {
// We support only packet offload mode and it means
// that user must set XFRM_OFFLOAD_PACKET bit.
//
    NL_SET_ERR_MSG(extack, "Unrecognized flags in offload request");
    return -EINVAL;
    }
    dev = dev_get_by_index(net, xuo.ifindex);
    if (!dev)
    return -EINVAL;
    if (!dev.xfrmdev_ops || !dev.xfrmdev_ops.xdo_dev_policy_add) {
    xdo.dev = core::ptr::null_mut();
    dev_put(dev);
    NL_SET_ERR_MSG(extack, "Policy offload is not supported");
    return -EINVAL;
    }
    xdo.dev = dev;
    netdev_tracker_alloc(dev, &xdo.dev_tracker, GFP_ATOMIC);
    xdo.type = XFRM_DEV_OFFLOAD_PACKET;
    switch (dir) {
    case XFRM_POLICY_IN:
    xdo.dir = XFRM_DEV_OFFLOAD_IN;
    break;
    case XFRM_POLICY_OUT:
    xdo.dir = XFRM_DEV_OFFLOAD_OUT;
    break;
    case XFRM_POLICY_FWD:
    xdo.dir = XFRM_DEV_OFFLOAD_FWD;
    break;
    default:
    xdo.dev = core::ptr::null_mut();
    netdev_put(dev, &xdo.dev_tracker);
    NL_SET_ERR_MSG(extack, "Unrecognized offload direction");
    return -EINVAL;
    }
    err = dev.xfrmdev_ops.xdo_dev_policy_add(xp, extack);
    if (err) {
    xdo.dev = core::ptr::null_mut();
    xdo.type = XFRM_DEV_OFFLOAD_UNSPECIFIED;
    xdo.dir = 0;
    netdev_put(dev, &xdo.dev_tracker);
    NL_SET_ERR_MSG_WEAK(extack, "Device failed to offload this policy");
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(xfrm_dev_policy_add);
#[no_mangle]
pub unsafe extern "C" fn xfrm_dev_offload_ok(skb: *mut sk_buff, x: *mut xfrm_state) -> bool {
    bool xfrm_dev_offload_ok(struct sk_buff *skb, struct xfrm_state *x)
    {
    int mtu;
    struct dst_entry *dst = skb_dst(skb);
    struct xfrm_dst *xdst = (struct xfrm_dst *)dst;
    struct net_device *dev = x.xso.dev;
    bool check_tunnel_size;
    if (!x.type_offload ||
    (x.xso.type == XFRM_DEV_OFFLOAD_UNSPECIFIED && x.encap))
    return false;
    if ((!dev || dev == xfrm_dst_path(dst).dev) &&
    !xdst.child.xfrm) {
    mtu = xfrm_state_mtu(x, xdst.child_mtu_cached);
    if (skb.len <= mtu)
    goto ok;
    if (skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu))
    goto ok;
    }
    return false;
    ok:
    if (!dev)
    return true;
    check_tunnel_size = x.xso.type == XFRM_DEV_OFFLOAD_PACKET &&
    x.props.mode == XFRM_MODE_TUNNEL;
    switch (skb_dst(skb).ops.family) {
    case AF_INET:
// Check for IPv4 options
    if (ip_hdr(skb).ihl != 5)
    return false;
    if (check_tunnel_size && xfrm4_tunnel_check_size(skb))
    return false;
    break;
    case AF_INET6:
// Check for IPv6 extensions
    if (ipv6_ext_hdr(ipv6_hdr(skb).nexthdr))
    return false;
    if (check_tunnel_size && xfrm6_tunnel_check_size(skb))
    return false;
    break;
    default:
    break;
    }
    if (dev.xfrmdev_ops.xdo_dev_offload_ok)
    return dev.xfrmdev_ops.xdo_dev_offload_ok(skb, x);
    return true;
    }
    EXPORT_SYMBOL_GPL(xfrm_dev_offload_ok);
#[no_mangle]
pub unsafe extern "C" fn xfrm_dev_resume(skb: *mut sk_buff) {
    void xfrm_dev_resume(struct sk_buff *skb)
    {
    struct net_device *dev = skb.dev;
    let mut ret: c_int = NETDEV_TX_BUSY;
    struct netdev_queue *txq;
    struct softnet_data *sd;
    unsigned long flags;
    rcu_read_lock();
    txq = netdev_core_pick_tx(dev, skb, core::ptr::null_mut());
    HARD_TX_LOCK(dev, txq, smp_processor_id());
    if (!netif_xmit_frozen_or_stopped(txq))
    skb = dev_hard_start_xmit(skb, dev, txq, &ret);
    HARD_TX_UNLOCK(dev, txq);
    if (!dev_xmit_complete(ret)) {
    local_irq_save(flags);
    sd = this_cpu_ptr(&softnet_data);
    skb_queue_tail(&sd.xfrm_backlog, skb);
    raise_softirq_irqoff(NET_TX_SOFTIRQ);
    local_irq_restore(flags);
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(xfrm_dev_resume);
#[no_mangle]
pub unsafe extern "C" fn xfrm_dev_backlog(sd: *mut softnet_data) {
    void xfrm_dev_backlog(struct softnet_data *sd)
    {
    struct sk_buff_head *xfrm_backlog = &sd.xfrm_backlog;
    struct sk_buff_head list;
    struct sk_buff *skb;
    if (skb_queue_empty(xfrm_backlog))
    return;
    __skb_queue_head_init(&list);
    spin_lock(&xfrm_backlog.lock);
    skb_queue_splice_init(xfrm_backlog, &list);
    spin_unlock(&xfrm_backlog.lock);
    while (!skb_queue_empty(&list)) {
    skb = __skb_dequeue(&list);
    xfrm_dev_resume(skb);
    }
    }

#[no_mangle]
unsafe extern "C" fn xfrm_api_check(dev: *mut net_device) -> c_int {
    static int xfrm_api_check(struct net_device *dev)
    {

    if ((dev.features & NETIF_F_HW_ESP_TX_CSUM) &&
    !(dev.features & NETIF_F_HW_ESP))
    return NOTIFY_BAD;
    if ((dev.features & NETIF_F_HW_ESP) &&
    (!(dev.xfrmdev_ops &&
    dev.xfrmdev_ops.xdo_dev_state_add &&
    dev.xfrmdev_ops.xdo_dev_state_delete)))
    return NOTIFY_BAD;

    if (dev.features & (NETIF_F_HW_ESP | NETIF_F_HW_ESP_TX_CSUM))
    return NOTIFY_BAD;

    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn xfrm_dev_down(dev: *mut net_device) -> c_int {
    static int xfrm_dev_down(struct net_device *dev)
    {
    if (dev.features & NETIF_F_HW_ESP) {
    xfrm_dev_state_flush(dev_net(dev), dev, true);
    xfrm_dev_policy_flush(dev_net(dev), dev, true);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn xfrm_dev_unregister(dev: *mut net_device) -> c_int {
    static int xfrm_dev_unregister(struct net_device *dev)
    {
    xfrm_dev_state_flush(dev_net(dev), dev, true);
    xfrm_dev_policy_flush(dev_net(dev), dev, true);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn xfrm_dev_event(this: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    static int xfrm_dev_event(struct notifier_block *this, unsigned long event, void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    switch (event) {
    case NETDEV_REGISTER:
    return xfrm_api_check(dev);
    case NETDEV_FEAT_CHANGE:
    return xfrm_api_check(dev);
    case NETDEV_DOWN:
    return xfrm_dev_down(dev);
    case NETDEV_UNREGISTER:
    return xfrm_dev_unregister(dev);
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block xfrm_dev_notifier = {
    .notifier_call	= xfrm_dev_event,
    };
#[no_mangle]
pub unsafe extern "C" fn xfrm_dev_init() -> void __init {
    void __init xfrm_dev_init(void)
    {
    register_netdevice_notifier(&xfrm_dev_notifier);
    }
