//! Automatically rewritten from C to Rust
//! Source: net/ipv6/xfrm6_tunnel.c
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
// Copyright (C)2003,2004 USAGI/WIDE Project
//
// Authors	Mitsuru KANDA  <mk@linux-ipv6.org>
// YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
//
// Based on net/ipv4/xfrm4_tunnel.c
//

pub const XFRM6_TUNNEL_SPI_BYADDR_HSIZE: c_int = 256;
pub const XFRM6_TUNNEL_SPI_BYSPI_HSIZE: c_int = 256;
pub const XFRM6_TUNNEL_SPI_MIN: c_int = 1;
pub const XFRM6_TUNNEL_SPI_MAX: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_tunnel_net {
    pub spi_byaddr: [hlist_head; XFRM6_TUNNEL_SPI_BYADDR_HSIZE],
    pub spi_byspi: [hlist_head; XFRM6_TUNNEL_SPI_BYSPI_HSIZE],
    pub spi: u32,
}

    static unsigned int xfrm6_tunnel_net_id __read_mostly;
    static inline struct xfrm6_tunnel_net *xfrm6_tunnel_pernet(struct net *net)
    {
    return net_generic(net, xfrm6_tunnel_net_id);
    }
//
// xfrm_tunnel_spi things are for allocating unique id ("spi")
// per xfrm_address_t.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_tunnel_spi {
    pub list_byaddr: hlist_node,
    pub list_byspi: hlist_node,
    pub addr: xfrm_address_t,
    pub spi: u32,
    pub refcnt: refcount_t,
    pub rcu_head: rcu_head,
}

    static DEFINE_SPINLOCK(xfrm6_tunnel_spi_lock);
    static struct kmem_cache *xfrm6_tunnel_spi_kmem __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byaddr(addr: *const xfrm_address_t) -> c_uint {
    static inline unsigned int xfrm6_tunnel_spi_hash_byaddr(const xfrm_address_t *addr)
    {
    unsigned int h;
    h = ipv6_addr_hash((const struct in6_addr *)addr);
    h ^= h >> 16;
    h ^= h >> 8;
    h &= XFRM6_TUNNEL_SPI_BYADDR_HSIZE - 1;
    return h;
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byspi(spi: u32) -> c_uint {
    static inline unsigned int xfrm6_tunnel_spi_hash_byspi(u32 spi)
    {
    return spi % XFRM6_TUNNEL_SPI_BYSPI_HSIZE;
    }
    static struct xfrm6_tunnel_spi *__xfrm6_tunnel_spi_lookup(struct net *net, const xfrm_address_t *saddr)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    struct xfrm6_tunnel_spi *x6spi;
    hlist_for_each_entry_rcu(x6spi,
    &xfrm6_tn.spi_byaddr[xfrm6_tunnel_spi_hash_byaddr(saddr)],
    list_byaddr, lockdep_is_held(&xfrm6_tunnel_spi_lock)) {
    if (xfrm6_addr_equal(&x6spi.addr, saddr))
    return x6spi;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_lookup(net: *mut net, saddr: *const xfrm_address_t) -> __be32 {
    __be32 xfrm6_tunnel_spi_lookup(struct net *net, const xfrm_address_t *saddr)
    {
    struct xfrm6_tunnel_spi *x6spi;
    u32 spi;
    rcu_read_lock_bh();
    x6spi = __xfrm6_tunnel_spi_lookup(net, saddr);
    spi = x6spi ? x6spi.spi : 0;
    rcu_read_unlock_bh();
    return htonl(spi);
    }
    EXPORT_SYMBOL(xfrm6_tunnel_spi_lookup);
#[no_mangle]
unsafe extern "C" fn __xfrm6_tunnel_spi_check(net: *mut net, spi: u32) -> c_int {
    static int __xfrm6_tunnel_spi_check(struct net *net, u32 spi)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    struct xfrm6_tunnel_spi *x6spi;
    let mut index: c_int = xfrm6_tunnel_spi_hash_byspi(spi);
    hlist_for_each_entry(x6spi,
    &xfrm6_tn.spi_byspi[index],
    list_byspi) {
    if (x6spi.spi == spi)
    return -1;
    }
    return index;
    }
#[no_mangle]
unsafe extern "C" fn __xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> u32 {
    static u32 __xfrm6_tunnel_alloc_spi(struct net *net, xfrm_address_t *saddr)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    u32 spi;
    struct xfrm6_tunnel_spi *x6spi;
    int index;
    if (xfrm6_tn.spi < XFRM6_TUNNEL_SPI_MIN ||
    xfrm6_tn.spi >= XFRM6_TUNNEL_SPI_MAX)
    xfrm6_tn.spi = XFRM6_TUNNEL_SPI_MIN;
    else
    xfrm6_tn.spi++;
    for (spi = xfrm6_tn.spi; spi <= XFRM6_TUNNEL_SPI_MAX; spi++) {
    index = __xfrm6_tunnel_spi_check(net, spi);
    if (index >= 0)
    goto alloc_spi;
    if (spi == XFRM6_TUNNEL_SPI_MAX)
    break;
    }
    for (spi = XFRM6_TUNNEL_SPI_MIN; spi < xfrm6_tn.spi; spi++) {
    index = __xfrm6_tunnel_spi_check(net, spi);
    if (index >= 0)
    goto alloc_spi;
    }
    spi = 0;
    goto out;
    alloc_spi:
    xfrm6_tn.spi = spi;
    x6spi = kmem_cache_alloc(xfrm6_tunnel_spi_kmem, GFP_ATOMIC);
    if (!x6spi)
    goto out;
    memcpy(&x6spi.addr, saddr, sizeof(x6spi.addr));
    x6spi.spi = spi;
    refcount_set(&x6spi.refcnt, 1);
    hlist_add_head_rcu(&x6spi.list_byspi, &xfrm6_tn.spi_byspi[index]);
    index = xfrm6_tunnel_spi_hash_byaddr(saddr);
    hlist_add_head_rcu(&x6spi.list_byaddr, &xfrm6_tn.spi_byaddr[index]);
    out:
    return spi;
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> __be32 {
    __be32 xfrm6_tunnel_alloc_spi(struct net *net, xfrm_address_t *saddr)
    {
    struct xfrm6_tunnel_spi *x6spi;
    u32 spi;
    spin_lock_bh(&xfrm6_tunnel_spi_lock);
    x6spi = __xfrm6_tunnel_spi_lookup(net, saddr);
    if (x6spi) {
    refcount_inc(&x6spi.refcnt);
    spi = x6spi.spi;
    } else
    spi = __xfrm6_tunnel_alloc_spi(net, saddr);
    spin_unlock_bh(&xfrm6_tunnel_spi_lock);
    return htonl(spi);
    }
    EXPORT_SYMBOL(xfrm6_tunnel_alloc_spi);
#[no_mangle]
unsafe extern "C" fn x6spi_destroy_rcu(head: *mut rcu_head) {
    static void x6spi_destroy_rcu(struct rcu_head *head)
    {
    kmem_cache_free(xfrm6_tunnel_spi_kmem,
    container_of(head, struct xfrm6_tunnel_spi, rcu_head));
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_free_spi(net: *mut net, saddr: *mut xfrm_address_t) {
    static void xfrm6_tunnel_free_spi(struct net *net, xfrm_address_t *saddr)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    struct xfrm6_tunnel_spi *x6spi;
    struct hlist_node *n;
    spin_lock_bh(&xfrm6_tunnel_spi_lock);
    hlist_for_each_entry_safe(x6spi, n,
    &xfrm6_tn.spi_byaddr[xfrm6_tunnel_spi_hash_byaddr(saddr)],
    list_byaddr)
    {
    if (xfrm6_addr_equal(&x6spi.addr, saddr)) {
    if (refcount_dec_and_test(&x6spi.refcnt)) {
    hlist_del_rcu(&x6spi.list_byaddr);
    hlist_del_rcu(&x6spi.list_byspi);
    call_rcu(&x6spi.rcu_head, x6spi_destroy_rcu);
    break;
    }
    }
    }
    spin_unlock_bh(&xfrm6_tunnel_spi_lock);
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm6_tunnel_output(struct xfrm_state *x, struct sk_buff *skb)
    {
    skb_push(skb, -skb_network_offset(skb));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm6_tunnel_input(struct xfrm_state *x, struct sk_buff *skb)
    {
    return skb_network_header(skb)[IP6CB(skb).nhoff];
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_rcv(skb: *mut sk_buff) -> c_int {
    static int xfrm6_tunnel_rcv(struct sk_buff *skb)
    {
    struct net *net = dev_net(skb.dev);
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    __be32 spi;
    spi = xfrm6_tunnel_spi_lookup(net, (const xfrm_address_t *)&iph.saddr);
    return xfrm6_rcv_spi(skb, IPPROTO_IPV6, spi, core::ptr::null_mut());
    }
    static int xfrm6_tunnel_err(struct sk_buff *skb, struct inet6_skb_parm *opt,
    u8 type, u8 code, int offset, __be32 info)
    {
// xfrm6_tunnel native err handling
    switch (type) {
    case ICMPV6_DEST_UNREACH:
    switch (code) {
    case ICMPV6_NOROUTE:
    case ICMPV6_ADM_PROHIBITED:
    case ICMPV6_NOT_NEIGHBOUR:
    case ICMPV6_ADDR_UNREACH:
    case ICMPV6_PORT_UNREACH:
    default:
    break;
    }
    break;
    case ICMPV6_PKT_TOOBIG:
    break;
    case ICMPV6_TIME_EXCEED:
    switch (code) {
    case ICMPV6_EXC_HOPLIMIT:
    break;
    case ICMPV6_EXC_FRAGTIME:
    default:
    break;
    }
    break;
    case ICMPV6_PARAMPROB:
    switch (code) {
    case ICMPV6_HDR_FIELD: break;
    case ICMPV6_UNK_NEXTHDR: break;
    case ICMPV6_UNK_OPTION: break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int {
    static int xfrm6_tunnel_init_state(struct xfrm_state *x, struct netlink_ext_ack *extack)
    {
    if (x.props.mode != XFRM_MODE_TUNNEL) {
    NL_SET_ERR_MSG(extack, "IPv6 tunnel can only be used with tunnel mode");
    return -EINVAL;
    }
    if (x.encap) {
    NL_SET_ERR_MSG(extack, "IPv6 tunnel is not compatible with encapsulation");
    return -EINVAL;
    }
    x.props.header_len = sizeof(struct ipv6hdr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_destroy(x: *mut xfrm_state) {
    static void xfrm6_tunnel_destroy(struct xfrm_state *x)
    {
    struct net *net = xs_net(x);
    xfrm6_tunnel_free_spi(net, (xfrm_address_t *)&x.props.saddr);
    }
    static const struct xfrm_type xfrm6_tunnel_type = {
    .owner          = THIS_MODULE,
    .proto		= IPPROTO_IPV6,
    .init_state	= xfrm6_tunnel_init_state,
    .destructor	= xfrm6_tunnel_destroy,
    .input		= xfrm6_tunnel_input,
    .output		= xfrm6_tunnel_output,
    };
    static struct xfrm6_tunnel xfrm6_tunnel_handler __read_mostly = {
    .handler	= xfrm6_tunnel_rcv,
    .err_handler	= xfrm6_tunnel_err,
    .priority	= 3,
    };
    static struct xfrm6_tunnel xfrm46_tunnel_handler __read_mostly = {
    .handler	= xfrm6_tunnel_rcv,
    .err_handler	= xfrm6_tunnel_err,
    .priority	= 3,
    };
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_net_init(net: *mut net) -> int __net_init {
    static int __net_init xfrm6_tunnel_net_init(struct net *net)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    unsigned int i;
    for (i = 0; i < XFRM6_TUNNEL_SPI_BYADDR_HSIZE; i++)
    INIT_HLIST_HEAD(&xfrm6_tn.spi_byaddr[i]);
    for (i = 0; i < XFRM6_TUNNEL_SPI_BYSPI_HSIZE; i++)
    INIT_HLIST_HEAD(&xfrm6_tn.spi_byspi[i]);
    xfrm6_tn.spi = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit xfrm6_tunnel_net_exit(struct net *net)
    {
    struct xfrm6_tunnel_net *xfrm6_tn = xfrm6_tunnel_pernet(net);
    unsigned int i;
    xfrm_state_flush(net, 0, false);
    xfrm_flush_gc();
    for (i = 0; i < XFRM6_TUNNEL_SPI_BYADDR_HSIZE; i++)
    WARN_ON_ONCE(!hlist_empty(&xfrm6_tn.spi_byaddr[i]));
    for (i = 0; i < XFRM6_TUNNEL_SPI_BYSPI_HSIZE; i++)
    WARN_ON_ONCE(!hlist_empty(&xfrm6_tn.spi_byspi[i]));
    }
    static struct pernet_operations xfrm6_tunnel_net_ops = {
    .init	= xfrm6_tunnel_net_init,
    .exit	= xfrm6_tunnel_net_exit,
    .id	= &xfrm6_tunnel_net_id,
    .size	= sizeof(struct xfrm6_tunnel_net),
    };
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_init() -> int __init {
    static int __init xfrm6_tunnel_init(void)
    {
    int rv;
    xfrm6_tunnel_spi_kmem = KMEM_CACHE(xfrm6_tunnel_spi, SLAB_HWCACHE_ALIGN);
    if (!xfrm6_tunnel_spi_kmem)
    return -ENOMEM;
    rv = register_pernet_subsys(&xfrm6_tunnel_net_ops);
    if (rv < 0)
    goto out_pernet;
    rv = xfrm_register_type(&xfrm6_tunnel_type, AF_INET6);
    if (rv < 0)
    goto out_type;
    rv = xfrm6_tunnel_register(&xfrm6_tunnel_handler, AF_INET6);
    if (rv < 0)
    goto out_xfrm6;
    rv = xfrm6_tunnel_register(&xfrm46_tunnel_handler, AF_INET);
    if (rv < 0)
    goto out_xfrm46;
    return 0;
    out_xfrm46:
    xfrm6_tunnel_deregister(&xfrm6_tunnel_handler, AF_INET6);
    out_xfrm6:
    xfrm_unregister_type(&xfrm6_tunnel_type, AF_INET6);
    out_type:
    unregister_pernet_subsys(&xfrm6_tunnel_net_ops);
    out_pernet:
    kmem_cache_destroy(xfrm6_tunnel_spi_kmem);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_tunnel_fini() -> void __exit {
    static void __exit xfrm6_tunnel_fini(void)
    {
    xfrm6_tunnel_deregister(&xfrm46_tunnel_handler, AF_INET);
    xfrm6_tunnel_deregister(&xfrm6_tunnel_handler, AF_INET6);
    xfrm_unregister_type(&xfrm6_tunnel_type, AF_INET6);
    unregister_pernet_subsys(&xfrm6_tunnel_net_ops);
// Someone maybe has gotten the xfrm6_tunnel_spi.
// So need to wait it.
//
    rcu_barrier();
    kmem_cache_destroy(xfrm6_tunnel_spi_kmem);
    }
    module_init(xfrm6_tunnel_init);
    module_exit(xfrm6_tunnel_fini);
    MODULE_DESCRIPTION("IPv6 XFRM tunnel driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_XFRM_TYPE(AF_INET6, XFRM_PROTO_IPV6);
