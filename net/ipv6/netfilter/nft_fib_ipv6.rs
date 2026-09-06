//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/nft_fib_ipv6.c
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
unsafe extern "C" fn get_ifindex(dev: *const net_device) -> c_int {
    static int get_ifindex(const struct net_device *dev)
    {
    return dev ? dev.ifindex : 0;
    }
    static int nft_fib6_flowi_init(struct flowi6 *fl6, const struct nft_fib *priv,
    const struct nft_pktinfo *pkt,
    const struct net_device *dev,
    struct ipv6hdr *iph)
    {
    let mut lookup_flags: c_int = 0;
    if (priv.flags & NFTA_FIB_F_DADDR) {
    fl6.daddr = iph.daddr;
    fl6.saddr = iph.saddr;
    } else {
    if (nft_hook(pkt) == NF_INET_FORWARD &&
    priv.flags & NFTA_FIB_F_IIF)
    fl6.flowi6_iif = nft_out(pkt).ifindex;
    fl6.daddr = iph.saddr;
    fl6.saddr = iph.daddr;
    }
    if (ipv6_addr_type(&fl6.daddr) & IPV6_ADDR_LINKLOCAL) {
    lookup_flags |= RT6_LOOKUP_F_IFACE;
    fl6.flowi6_oif = get_ifindex(dev ? dev : pkt.skb.dev);
    }
    if (ipv6_addr_type(&fl6.saddr) & IPV6_ADDR_UNICAST)
    lookup_flags |= RT6_LOOKUP_F_HAS_SADDR;
    if (priv.flags & NFTA_FIB_F_MARK)
    fl6.flowi6_mark = pkt.skb.mark;
    fl6.flowlabel = (*(__be32 *)iph) & IPV6_FLOWINFO_MASK;
    fl6.flowi6_l3mdev = nft_fib_l3mdev_master_ifindex_rcu(pkt, dev);
    return lookup_flags | RT6_LOOKUP_F_DST_NOREF;
    }
    static int nft_fib6_lookup(struct net *net, struct flowi6 *fl6,
    struct fib6_result *res, int flags)
    {
    return fib6_lookup(net, fl6.flowi6_oif, fl6, res, flags);
    }
    static u32 __nft_fib6_eval_type(const struct nft_fib *priv,
    const struct nft_pktinfo *pkt,
    struct ipv6hdr *iph)
    {
    const struct net_device *dev = core::ptr::null_mut();
    let mut res: fib6_result = {};
    int route_err, addrtype;
    struct flowi6 fl6 = {
    .flowi6_iif = LOOPBACK_IFINDEX,
    .flowi6_proto = pkt.tprot,
    .flowi6_uid = sock_net_uid(nft_net(pkt), core::ptr::null_mut()),
    };
    int lookup_flags;
    let mut ret: u32 = 0;
    if (priv.flags & NFTA_FIB_F_IIF)
    dev = nft_in(pkt);
#[no_mangle]
pub unsafe extern "C" fn if(NFTA_FIB_F_OIF: priv->flags &) -> else {
    else if (priv.flags & NFTA_FIB_F_OIF)
    dev = nft_out(pkt);
    lookup_flags = nft_fib6_flowi_init(&fl6, priv, pkt, dev, iph);
    if (dev && nf_ipv6_chk_addr(nft_net(pkt), &fl6.daddr, dev, true))
    ret = RTN_LOCAL;
    route_err = nft_fib6_lookup(nft_net(pkt), &fl6, &res, lookup_flags);
    if (route_err)
    goto err;
    if (res.fib6_flags & RTF_REJECT)
    return res.fib6_type;
    if (__ipv6_anycast_destination(&res.f6i.fib6_dst, res.fib6_flags, &fl6.daddr))
    ret = RTN_ANYCAST;
#[no_mangle]
pub unsafe extern "C" fn if(RTF_LOCAL: !dev && res.fib6_flags &) -> else {
    else if (!dev && res.fib6_flags & RTF_LOCAL)
    ret = RTN_LOCAL;
    if (ret)
    return ret;
    addrtype = ipv6_addr_type(&fl6.daddr);
    if (addrtype & IPV6_ADDR_MULTICAST)
    return RTN_MULTICAST;
    if (addrtype & IPV6_ADDR_UNICAST)
    return RTN_UNICAST;
    return RTN_UNSPEC;
    err:
    switch (route_err) {
    case -EINVAL:
    return RTN_BLACKHOLE;
    case -EACCES:
    return RTN_PROHIBIT;
    case -EAGAIN:
    return RTN_THROW;
    default:
    break;
    }
    return RTN_UNREACHABLE;
    }
    void nft_fib6_eval_type(const struct nft_expr *expr, struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    let mut noff: c_int = skb_network_offset(pkt.skb);
    u32 *dest = &regs.data[priv.dreg];
    struct ipv6hdr *iph, _iph;
    iph = skb_header_pointer(pkt.skb, noff, sizeof(_iph), &_iph);
    if (!iph) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
// dest = __nft_fib6_eval_type(priv, pkt, iph);
    }
    EXPORT_SYMBOL_GPL(nft_fib6_eval_type);
#[no_mangle]
unsafe extern "C" fn nft_fib_v6_skip_icmpv6(skb: *const sk_buff, next: u8, iph: *const ipv6hdr) -> bool {
    static bool nft_fib_v6_skip_icmpv6(const struct sk_buff *skb, u8 next, const struct ipv6hdr *iph)
    {
    if (likely(next != IPPROTO_ICMPV6))
    return false;
    if (ipv6_addr_type(&iph.saddr) != IPV6_ADDR_ANY)
    return false;
    return ipv6_addr_type(&iph.daddr) & IPV6_ADDR_LINKLOCAL;
    }
    static bool nft_fib6_info_nh_dev_match(const struct net_device *nh_dev,
    const struct net_device *dev)
    {
    return nh_dev == dev ||
    l3mdev_master_ifindex_rcu(nh_dev) == dev.ifindex;
    }
#[no_mangle]
unsafe extern "C" fn nft_fib6_nh_match_dev_cb(nh: *mut fib6_nh, arg: *mut c_void) -> c_int {
    static int nft_fib6_nh_match_dev_cb(struct fib6_nh *nh, void *arg)
    {
    const struct net_device *dev = arg;
    return nft_fib6_info_nh_dev_match(nh.fib_nh_dev, dev);
    }
    static bool nft_fib6_info_nh_uses_dev(struct fib6_info *rt,
    const struct net_device *dev)
    {
    const struct net_device *nh_dev;
    struct fib6_info *iter;
// External nexthop: fib6_siblings slot aliases nh_list, walk via nh.
    if (rt.nh)
    return nexthop_for_each_fib6_nh(rt.nh,
    nft_fib6_nh_match_dev_cb,
    (void *)dev);
    nh_dev = fib6_info_nh_dev(rt);
    if (nft_fib6_info_nh_dev_match(nh_dev, dev))
    return true;
    if (!READ_ONCE(rt.fib6_nsiblings))
    return false;
    list_for_each_entry_rcu(iter, &rt.fib6_siblings, fib6_siblings) {
    nh_dev = fib6_info_nh_dev(iter);
    if (nft_fib6_info_nh_dev_match(nh_dev, dev))
    return true;
    if (!READ_ONCE(rt.fib6_nsiblings))
    return false;
    }
    return false;
    }
    void nft_fib6_eval(const struct nft_expr *expr, struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    let mut noff: c_int = skb_network_offset(pkt.skb);
    const struct net_device *found = core::ptr::null_mut();
    const struct net_device *oif = core::ptr::null_mut();
    u32 *dest = &regs.data[priv.dreg];
    let mut res: fib6_result = {};
    struct ipv6hdr *iph, _iph;
    struct flowi6 fl6 = {
    .flowi6_iif = LOOPBACK_IFINDEX,
    .flowi6_proto = pkt.tprot,
    .flowi6_uid = sock_net_uid(nft_net(pkt), core::ptr::null_mut()),
    };
    int lookup_flags, ret;
    if (nft_fib_can_skip(pkt)) {
    nft_fib_store_result(dest, priv, nft_in(pkt));
    return;
    }
    if (priv.flags & NFTA_FIB_F_IIF)
    oif = nft_in(pkt);
#[no_mangle]
pub unsafe extern "C" fn if(NFTA_FIB_F_OIF: priv->flags &) -> else {
    else if (priv.flags & NFTA_FIB_F_OIF)
    oif = nft_out(pkt);
    iph = skb_header_pointer(pkt.skb, noff, sizeof(_iph), &_iph);
    if (!iph) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    if (nft_fib_v6_skip_icmpv6(pkt.skb, pkt.tprot, iph)) {
    nft_fib_store_result(dest, priv, nft_in(pkt));
    return;
    }
    lookup_flags = nft_fib6_flowi_init(&fl6, priv, pkt, oif, iph);
    nft_fib_store_result(dest, priv, core::ptr::null_mut());
    ret = nft_fib6_lookup(nft_net(pkt), &fl6, &res, lookup_flags);
    if (ret || res.fib6_flags & (RTF_REJECT | RTF_ANYCAST | RTF_LOCAL))
    return;
    if (!oif) {
    found = fib6_info_nh_dev(res.f6i);
    } else {
    if (nft_fib6_info_nh_uses_dev(res.f6i, oif))
    found = oif;
    }
    nft_fib_store_result(dest, priv, found);
    }
    EXPORT_SYMBOL_GPL(nft_fib6_eval);
    static struct nft_expr_type nft_fib6_type;
    static const struct nft_expr_ops nft_fib6_type_ops = {
    .type		= &nft_fib6_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_fib)),
    .eval		= nft_fib6_eval_type,
    .init		= nft_fib_init,
    .dump		= nft_fib_dump,
    .validate	= nft_fib_validate,
    };
    static const struct nft_expr_ops nft_fib6_ops = {
    .type		= &nft_fib6_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_fib)),
    .eval		= nft_fib6_eval,
    .init		= nft_fib_init,
    .dump		= nft_fib_dump,
    .validate	= nft_fib_validate,
    };
    static const struct nft_expr_ops *
    nft_fib6_select_ops(const struct nft_ctx *ctx,
    const struct nlattr * const tb[])
    {
    enum nft_fib_result result;
    if (!tb[NFTA_FIB_RESULT])
    return ERR_PTR(-EINVAL);
    result = ntohl(nla_get_be32(tb[NFTA_FIB_RESULT]));
    switch (result) {
    case NFT_FIB_RESULT_OIF:
    return &nft_fib6_ops;
    case NFT_FIB_RESULT_OIFNAME:
    return &nft_fib6_ops;
    case NFT_FIB_RESULT_ADDRTYPE:
    return &nft_fib6_type_ops;
    default:
    return ERR_PTR(-EOPNOTSUPP);
    }
    }
    static struct nft_expr_type nft_fib6_type __read_mostly = {
    .name		= "fib",
    .select_ops	= nft_fib6_select_ops,
    .policy		= nft_fib_policy,
    .maxattr	= NFTA_FIB_MAX,
    .family		= NFPROTO_IPV6,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_fib6_module_init() -> int __init {
    static int __init nft_fib6_module_init(void)
    {
    return nft_register_expr(&nft_fib6_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_fib6_module_exit() -> void __exit {
    static void __exit nft_fib6_module_exit(void)
    {
    nft_unregister_expr(&nft_fib6_type);
    }
    module_init(nft_fib6_module_init);
    module_exit(nft_fib6_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_ALIAS_NFT_AF_EXPR(10, "fib");
    MODULE_DESCRIPTION("nftables fib / ipv6 route lookup support");
