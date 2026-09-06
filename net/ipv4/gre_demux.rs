//! Automatically rewritten from C to Rust
//! Source: net/ipv4/gre_demux.c
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
// GRE over IPv4 demultiplexer driver
//
// Authors: Dmitry Kozlov (xeb@mail.ru)
//

    static const struct gre_protocol __rcu *gre_proto[GREPROTO_MAX] __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    int gre_add_protocol(const struct gre_protocol *proto, u8 version)
    {
    if (version >= GREPROTO_MAX)
    return -EINVAL;
    return (cmpxchg((const struct gre_protocol **)&gre_proto[version], core::ptr::null_mut(), proto) == core::ptr::null_mut()) ?
    0 : -EBUSY;
    }
    EXPORT_SYMBOL_GPL(gre_add_protocol);
#[no_mangle]
pub unsafe extern "C" fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    int gre_del_protocol(const struct gre_protocol *proto, u8 version)
    {
    int ret;
    if (version >= GREPROTO_MAX)
    return -EINVAL;
    ret = (cmpxchg((const struct gre_protocol **)&gre_proto[version], proto, core::ptr::null_mut()) == proto) ?
    0 : -EBUSY;
    if (ret)
    return ret;
    synchronize_rcu();
    return 0;
    }
    EXPORT_SYMBOL_GPL(gre_del_protocol);
// Fills in tpi and returns header length to be pulled.
// Note that caller must use pskb_may_pull() before pulling GRE header.
//
    int gre_parse_header(struct sk_buff *skb, struct tnl_ptk_info *tpi,
    bool *csum_err, __be16 proto, int nhs)
    {
    const struct gre_base_hdr *greh;
    __be32 *options;
    int hdr_len;
    if (unlikely(!pskb_may_pull(skb, nhs + sizeof(struct gre_base_hdr))))
    return -EINVAL;
    greh = (struct gre_base_hdr *)(skb.data + nhs);
    if (unlikely(greh.flags & (GRE_VERSION | GRE_ROUTING)))
    return -EINVAL;
    gre_flags_to_tnl_flags(tpi.flags, greh.flags);
    hdr_len = gre_calc_hlen(tpi.flags);
    if (!pskb_may_pull(skb, nhs + hdr_len))
    return -EINVAL;
    greh = (struct gre_base_hdr *)(skb.data + nhs);
    tpi.proto = greh.protocol;
    options = (__be32 *)(greh + 1);
    if (greh.flags & GRE_CSUM) {
    if (!skb_checksum_simple_validate(skb)) {
    skb_checksum_try_convert(skb, IPPROTO_GRE,
    null_compute_pseudo);
    } else if (csum_err) {
// csum_err = true;
    return -EINVAL;
    }
    options++;
    }
    if (greh.flags & GRE_KEY) {
    tpi.key = *options;
    options++;
    } else {
    tpi.key = 0;
    }
    if (unlikely(greh.flags & GRE_SEQ)) {
    tpi.seq = *options;
    options++;
    } else {
    tpi.seq = 0;
    }
// WCCP version 1 and 2 protocol decoding.
// - Change protocol to IPv4/IPv6
// - When dealing with WCCPv2, Skip extra 4 bytes in GRE header
//
    if (greh.flags == 0 && tpi.proto == htons(ETH_P_WCCP)) {
    u8 _val, *val;
    val = skb_header_pointer(skb, nhs + hdr_len,
    sizeof(_val), &_val);
    if (!val)
    return -EINVAL;
    tpi.proto = proto;
    if ((*val & 0xF0) != 0x40)
    hdr_len += 4;
    }
    tpi.hdr_len = hdr_len;
// ERSPAN ver 1 and 2 protocol sets GRE key field
// to 0 and sets the configured key in the
// inner erspan header field
//
    if ((greh.protocol == htons(ETH_P_ERSPAN) && hdr_len != 4) ||
    greh.protocol == htons(ETH_P_ERSPAN2)) {
    struct erspan_base_hdr *ershdr;
    if (!pskb_may_pull(skb, nhs + hdr_len + sizeof(*ershdr)))
    return -EINVAL;
    ershdr = (struct erspan_base_hdr *)(skb.data + nhs + hdr_len);
    tpi.key = cpu_to_be32(get_session_id(ershdr));
    }
    return hdr_len;
    }
    EXPORT_SYMBOL(gre_parse_header);
#[no_mangle]
unsafe extern "C" fn gre_rcv(skb: *mut sk_buff) -> c_int {
    static int gre_rcv(struct sk_buff *skb)
    {
    const struct gre_protocol *proto;
    u8 ver;
    int ret;
    if (!pskb_may_pull(skb, 12))
    goto drop;
    ver = skb.data[1]&0x7f;
    if (ver >= GREPROTO_MAX)
    goto drop;
    rcu_read_lock();
    proto = rcu_dereference(gre_proto[ver]);
    if (!proto || !proto.handler)
    goto drop_nohandler;
    ret = proto.handler(skb);
    rcu_read_unlock();
    return ret;
    drop_nohandler:
    rcu_read_unlock();
    dev_core_stats_rx_nohandler_inc(skb.dev);
    kfree_skb(skb);
    return NET_RX_DROP;
    drop:
    dev_core_stats_rx_dropped_inc(skb.dev);
    kfree_skb(skb);
    return NET_RX_DROP;
    }
#[no_mangle]
unsafe extern "C" fn gre_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int gre_err(struct sk_buff *skb, u32 info)
    {
    const struct gre_protocol *proto;
    const struct iphdr *iph = (const struct iphdr *)skb.data;
    let mut ver: u8 = skb.data[(iph.ihl<<2) + 1]&0x7f;
    let mut err: c_int = 0;
    if (ver >= GREPROTO_MAX)
    return -EINVAL;
    rcu_read_lock();
    proto = rcu_dereference(gre_proto[ver]);
    if (proto && proto.err_handler)
    proto.err_handler(skb, info);
    else
    err = -EPROTONOSUPPORT;
    rcu_read_unlock();
    return err;
    }
    static const struct net_protocol net_gre_protocol = {
    .handler     = gre_rcv,
    .err_handler = gre_err,
    };
#[no_mangle]
unsafe extern "C" fn gre_init() -> int __init {
    static int __init gre_init(void)
    {
    pr_info("GRE over IPv4 demultiplexer driver\n");
    if (inet_add_protocol(&net_gre_protocol, IPPROTO_GRE) < 0) {
    pr_err("can't add protocol\n");
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gre_exit() -> void __exit {
    static void __exit gre_exit(void)
    {
    inet_del_protocol(&net_gre_protocol, IPPROTO_GRE);
    }
    module_init(gre_init);
    module_exit(gre_exit);
    MODULE_DESCRIPTION("GRE over IPv4 demultiplexer driver");
    MODULE_AUTHOR("D. Kozlov <xeb@mail.ru>");
    MODULE_LICENSE("GPL");
