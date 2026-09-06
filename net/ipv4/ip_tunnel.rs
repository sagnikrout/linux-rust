//! Automatically rewritten from C to Rust
//! Source: net/ipv4/ip_tunnel.c
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
//
// Copyright (c) 2013 Nicira, Inc.
//

#[no_mangle]
unsafe extern "C" fn ip_tunnel_hash(key: __be32, remote: __be32) -> c_uint {
    static unsigned int ip_tunnel_hash(__be32 key, __be32 remote)
    {
    return hash_32(( u32)key ^ ( u32)remote,
    IP_TNL_HASH_BITS);
    }
    static bool ip_tunnel_key_match(const struct ip_tunnel_parm_kern *p,
    const unsigned long *flags, __be32 key)
    {
    if (!test_bit(IP_TUNNEL_KEY_BIT, flags))
    return !test_bit(IP_TUNNEL_KEY_BIT, p.i_flags);
    return test_bit(IP_TUNNEL_KEY_BIT, p.i_flags) && p.i_key == key;
    }
// Fallback tunnel: no source, no destination, no key, no options
    Tunnel hash table:
    We require exact key match i.e. if a key is present in packet
    it will match only tunnel with the same key; if it is not present,
    it will match only keyless tunnel.
    All keysless packets, if not matched configured keyless tunnels
    will match fallback tunnel.
    Given src, dst and key, find appropriate for input tunnel.
//
    struct ip_tunnel *ip_tunnel_lookup(struct ip_tunnel_net *itn,
    int link, const unsigned long *flags,
    __be32 remote, __be32 local,
    __be32 key)
    {
    struct ip_tunnel *t, *cand = core::ptr::null_mut();
    struct hlist_head *head;
    struct net_device *ndev;
    unsigned int hash;
    hash = ip_tunnel_hash(key, remote);
    head = &itn.tunnels[hash];
    hlist_for_each_entry_rcu(t, head, hash_node) {
    if (local != t.parms.iph.saddr ||
    remote != t.parms.iph.daddr ||
    !(t.dev.flags & IFF_UP))
    continue;
    if (!ip_tunnel_key_match(&t.parms, flags, key))
    continue;
    if (READ_ONCE(t.parms.link) == link)
    return t;
    cand = t;
    }
    hlist_for_each_entry_rcu(t, head, hash_node) {
    if (remote != t.parms.iph.daddr ||
    t.parms.iph.saddr != 0 ||
    !(t.dev.flags & IFF_UP))
    continue;
    if (!ip_tunnel_key_match(&t.parms, flags, key))
    continue;
    if (READ_ONCE(t.parms.link) == link)
    return t;
    if (!cand)
    cand = t;
    }
    hash = ip_tunnel_hash(key, 0);
    head = &itn.tunnels[hash];
    hlist_for_each_entry_rcu(t, head, hash_node) {
    if ((local != t.parms.iph.saddr || t.parms.iph.daddr != 0) &&
    (local != t.parms.iph.daddr || !ipv4_is_multicast(local)))
    continue;
    if (!(t.dev.flags & IFF_UP))
    continue;
    if (!ip_tunnel_key_match(&t.parms, flags, key))
    continue;
    if (READ_ONCE(t.parms.link) == link)
    return t;
    if (!cand)
    cand = t;
    }
    hlist_for_each_entry_rcu(t, head, hash_node) {
    if ((!test_bit(IP_TUNNEL_NO_KEY_BIT, flags) &&
    t.parms.i_key != key) ||
    t.parms.iph.saddr != 0 ||
    t.parms.iph.daddr != 0 ||
    !(t.dev.flags & IFF_UP))
    continue;
    if (READ_ONCE(t.parms.link) == link)
    return t;
    if (!cand)
    cand = t;
    }
    if (cand)
    return cand;
    t = rcu_dereference(itn.collect_md_tun);
    if (t && t.dev.flags & IFF_UP)
    return t;
    ndev = READ_ONCE(itn.fb_tunnel_dev);
    if (ndev && ndev.flags & IFF_UP)
    return netdev_priv(ndev);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_lookup);
    static struct hlist_head *ip_bucket(struct ip_tunnel_net *itn,
    struct ip_tunnel_parm_kern *parms)
    {
    unsigned int h;
    __be32 remote;
    let mut i_key: __be32 = parms.i_key;
    if (parms.iph.daddr && !ipv4_is_multicast(parms.iph.daddr))
    remote = parms.iph.daddr;
    else
    remote = 0;
    if (!test_bit(IP_TUNNEL_KEY_BIT, parms.i_flags) &&
    test_bit(IP_TUNNEL_VTI_BIT, parms.i_flags))
    i_key = 0;
    h = ip_tunnel_hash(i_key, remote);
    return &itn.tunnels[h];
    }
#[no_mangle]
unsafe extern "C" fn ip_tunnel_add(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    static void ip_tunnel_add(struct ip_tunnel_net *itn, struct ip_tunnel *t)
    {
    struct hlist_head *head = ip_bucket(itn, &t.parms);
    if (t.collect_md)
    rcu_assign_pointer(itn.collect_md_tun, t);
    hlist_add_head_rcu(&t.hash_node, head);
    }
#[no_mangle]
unsafe extern "C" fn ip_tunnel_del(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    static void ip_tunnel_del(struct ip_tunnel_net *itn, struct ip_tunnel *t)
    {
    if (t.collect_md)
    rcu_assign_pointer(itn.collect_md_tun, core::ptr::null_mut());
    hlist_del_init_rcu(&t.hash_node);
    }
    static struct ip_tunnel *ip_tunnel_find(struct ip_tunnel_net *itn,
    struct ip_tunnel_parm_kern *parms,
    int type)
    {
    let mut remote: __be32 = parms.iph.daddr;
    let mut local: __be32 = parms.iph.saddr;
    IP_TUNNEL_DECLARE_FLAGS(flags);
    let mut key: __be32 = parms.i_key;
    let mut link: c_int = parms.link;
    struct ip_tunnel *t = core::ptr::null_mut();
    struct hlist_head *head = ip_bucket(itn, parms);
    ip_tunnel_flags_copy(flags, parms.i_flags);
    hlist_for_each_entry_rcu(t, head, hash_node, lockdep_rtnl_is_held()) {
    if (local == t.parms.iph.saddr &&
    remote == t.parms.iph.daddr &&
    link == READ_ONCE(t.parms.link) &&
    type == t.dev.type &&
    ip_tunnel_key_match(&t.parms, flags, key))
    break;
    }
    return t;
    }
    static struct net_device *__ip_tunnel_create(struct net *net,
    const struct rtnl_link_ops *ops,
    struct ip_tunnel_parm_kern *parms)
    {
    int err;
    struct ip_tunnel *tunnel;
    struct net_device *dev;
    char name[IFNAMSIZ];
    err = -E2BIG;
    if (parms.name[0]) {
    if (!dev_valid_name(parms.name))
    goto failed;
    strscpy(name, parms.name);
    } else {
    if (strlen(ops.kind) > (IFNAMSIZ - 3))
    goto failed;
    strscpy(name, ops.kind);
    strcat(name, "%d");
    }
    ASSERT_RTNL();
    dev = alloc_netdev(ops.priv_size, name, NET_NAME_UNKNOWN, ops.setup);
    if (!dev) {
    err = -ENOMEM;
    goto failed;
    }
    dev_net_set(dev, net);
    dev.rtnl_link_ops = ops;
    tunnel = netdev_priv(dev);
    tunnel.parms = *parms;
    tunnel.net = net;
    err = register_netdevice(dev);
    if (err)
    goto failed_free;
    return dev;
    failed_free:
    free_netdev(dev);
    failed:
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn ip_tunnel_bind_dev(dev: *mut net_device) -> c_int {
    static int ip_tunnel_bind_dev(struct net_device *dev)
    {
    struct net_device *tdev = core::ptr::null_mut();
    struct ip_tunnel *tunnel = netdev_priv(dev);
    const struct iphdr *iph;
    let mut hlen: c_int = LL_MAX_HEADER;
    let mut mtu: c_int = ETH_DATA_LEN;
    let mut t_hlen: c_int = tunnel.hlen + sizeof(struct iphdr);
    iph = &tunnel.parms.iph;
// Guess output device to choose reasonable mtu and needed_headroom
    if (iph.daddr) {
    struct flowi4 fl4;
    struct rtable *rt;
    ip_tunnel_init_flow(&fl4, iph.protocol, iph.daddr,
    iph.saddr, tunnel.parms.o_key,
    iph.tos & INET_DSCP_MASK, tunnel.net,
    tunnel.parms.link, tunnel.fwmark, 0, 0);
    rt = ip_route_output_key(tunnel.net, &fl4);
    if (!IS_ERR(rt)) {
    tdev = rt.dst.dev;
    ip_rt_put(rt);
    }
    if (dev.type != ARPHRD_ETHER)
    dev.flags |= IFF_POINTOPOINT;
    dst_cache_reset(&tunnel.dst_cache);
    }
    if (!tdev && tunnel.parms.link)
    tdev = __dev_get_by_index(tunnel.net, tunnel.parms.link);
    if (tdev) {
    hlen = tdev.hard_header_len + tdev.needed_headroom;
    mtu = min(tdev.mtu, IP_MAX_MTU);
    }
    dev.needed_headroom = ip_tunnel_limit_headroom(t_hlen + hlen);
    mtu -= t_hlen + (dev.type == ARPHRD_ETHER ? dev.hard_header_len : 0);
    if (mtu < IPV4_MIN_MTU)
    mtu = IPV4_MIN_MTU;
    return mtu;
    }
    static struct ip_tunnel *ip_tunnel_create(struct net *net,
    struct ip_tunnel_net *itn,
    struct ip_tunnel_parm_kern *parms)
    {
    struct ip_tunnel *nt;
    struct net_device *dev;
    int t_hlen;
    int mtu;
    int err;
    dev = __ip_tunnel_create(net, itn.rtnl_link_ops, parms);
    if (IS_ERR(dev))
    return ERR_CAST(dev);
    mtu = ip_tunnel_bind_dev(dev);
    err = dev_set_mtu(dev, mtu);
    if (err)
    goto err_dev_set_mtu;
    nt = netdev_priv(dev);
    t_hlen = nt.hlen + sizeof(struct iphdr);
    dev.min_mtu = ETH_MIN_MTU;
    dev.max_mtu = IP_MAX_MTU - t_hlen;
    if (dev.type == ARPHRD_ETHER)
    dev.max_mtu -= dev.hard_header_len;
    ip_tunnel_add(itn, nt);
    return nt;
    err_dev_set_mtu:
    unregister_netdevice(dev);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_md_udp_encap(skb: *mut sk_buff, info: *mut ip_tunnel_info) {
    void ip_tunnel_md_udp_encap(struct sk_buff *skb, struct ip_tunnel_info *info)
    {
    const struct iphdr *iph = ip_hdr(skb);
    const struct udphdr *udph;
    if (iph.protocol != IPPROTO_UDP)
    return;
    udph = (struct udphdr *)((__u8 *)iph + (iph.ihl << 2));
    info.encap.sport = udph.source;
    info.encap.dport = udph.dest;
    }
    EXPORT_SYMBOL(ip_tunnel_md_udp_encap);
    int ip_tunnel_rcv(struct ip_tunnel *tunnel, struct sk_buff *skb,
    const struct tnl_ptk_info *tpi, struct metadata_dst *tun_dst,
    bool log_ecn_error)
    {
    const struct iphdr *iph = ip_hdr(skb);
    int nh, err;

    if (ipv4_is_multicast(iph.daddr)) {
    DEV_STATS_INC(tunnel.dev, multicast);
    skb.pkt_type = PACKET_BROADCAST;
    }

    if (test_bit(IP_TUNNEL_CSUM_BIT, tunnel.parms.i_flags) !=
    test_bit(IP_TUNNEL_CSUM_BIT, tpi.flags)) {
    DEV_STATS_INC(tunnel.dev, rx_crc_errors);
    DEV_STATS_INC(tunnel.dev, rx_errors);
    goto drop;
    }
    if (test_bit(IP_TUNNEL_SEQ_BIT, tunnel.parms.i_flags)) {
    if (!test_bit(IP_TUNNEL_SEQ_BIT, tpi.flags) ||
    (tunnel.i_seqno && (s32)(ntohl(tpi.seq) - tunnel.i_seqno) < 0)) {
    DEV_STATS_INC(tunnel.dev, rx_fifo_errors);
    DEV_STATS_INC(tunnel.dev, rx_errors);
    goto drop;
    }
    tunnel.i_seqno = ntohl(tpi.seq) + 1;
    }
// Save offset of outer header relative to skb->head,
// because we are going to reset the network header to the inner header
// and might change skb->head.
//
    nh = skb_network_header(skb) - skb.head;
    skb_set_network_header(skb, (tunnel.dev.type == ARPHRD_ETHER) ? ETH_HLEN : 0);
    if (!pskb_inet_may_pull(skb)) {
    DEV_STATS_INC(tunnel.dev, rx_length_errors);
    DEV_STATS_INC(tunnel.dev, rx_errors);
    goto drop;
    }
    iph = (struct iphdr *)(skb.head + nh);
    err = IP_ECN_decapsulate(iph, skb);
    if (unlikely(err)) {
    if (log_ecn_error)
    net_info_ratelimited("non-ECT from %pI4 with TOS=%#x\n",
    &iph.saddr, iph.tos);
    if (err > 1) {
    DEV_STATS_INC(tunnel.dev, rx_frame_errors);
    DEV_STATS_INC(tunnel.dev, rx_errors);
    goto drop;
    }
    }
    dev_sw_netstats_rx_add(tunnel.dev, skb.len);
    skb_scrub_packet(skb, !net_eq(tunnel.net, dev_net(tunnel.dev)));
    if (tunnel.dev.type == ARPHRD_ETHER) {
    skb.protocol = eth_type_trans(skb, tunnel.dev);
    skb_postpull_rcsum(skb, eth_hdr(skb), ETH_HLEN);
    } else {
    skb.dev = tunnel.dev;
    }
    if (tun_dst)
    skb_dst_set(skb, (struct dst_entry *)tun_dst);
    gro_cells_receive(&tunnel.gro_cells, skb);
    return 0;
    drop:
    if (tun_dst)
    dst_release((struct dst_entry *)tun_dst);
    kfree_skb(skb);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_rcv);
    int ip_tunnel_encap_add_ops(const struct ip_tunnel_encap_ops *ops,
    unsigned int num)
    {
    if (num >= MAX_IPTUN_ENCAP_OPS)
    return -ERANGE;
    return !cmpxchg((const struct ip_tunnel_encap_ops **)
    &iptun_encaps[num],
    core::ptr::null_mut(), ops) ? 0 : -1;
    }
    EXPORT_SYMBOL(ip_tunnel_encap_add_ops);
    int ip_tunnel_encap_del_ops(const struct ip_tunnel_encap_ops *ops,
    unsigned int num)
    {
    int ret;
    if (num >= MAX_IPTUN_ENCAP_OPS)
    return -ERANGE;
    ret = (cmpxchg((const struct ip_tunnel_encap_ops **)
    &iptun_encaps[num],
    ops, core::ptr::null_mut()) == ops) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(ip_tunnel_encap_del_ops);
    int ip_tunnel_encap_setup(struct ip_tunnel *t,
    struct ip_tunnel_encap *ipencap)
    {
    int hlen;
    memset(&t.encap, 0, sizeof(t.encap));
    hlen = ip_encap_hlen(ipencap);
    if (hlen < 0)
    return hlen;
    t.encap.type = ipencap.type;
    t.encap.sport = ipencap.sport;
    t.encap.dport = ipencap.dport;
    t.encap.flags = ipencap.flags;
    t.encap_hlen = hlen;
    t.hlen = t.encap_hlen + t.tun_hlen;
    return 0;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_encap_setup);
    static int tnl_update_pmtu(struct net_device *dev, struct sk_buff *skb,
    struct rtable *rt, __be16 df,
    const struct iphdr *inner_iph,
    int tunnel_hlen, __be32 dst, bool md)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    int pkt_size;
    int mtu;
    tunnel_hlen = md ? tunnel_hlen : tunnel.hlen;
    pkt_size = skb.len - tunnel_hlen;
    pkt_size -= dev.type == ARPHRD_ETHER ? dev.hard_header_len : 0;
    if (df) {
    mtu = dst_mtu(&rt.dst) - (sizeof(struct iphdr) + tunnel_hlen);
    mtu -= dev.type == ARPHRD_ETHER ? dev.hard_header_len : 0;
    } else {
    mtu = skb_valid_dst(skb) ? dst_mtu(skb_dst(skb)) : dev.mtu;
    }
    if (skb_valid_dst(skb))
    skb_dst_update_pmtu_no_confirm(skb, mtu);
    if (skb.protocol == htons(ETH_P_IP)) {
    if (!skb_is_gso(skb) &&
    (inner_iph.frag_off & htons(IP_DF)) &&
    mtu < pkt_size) {
    icmp_ndo_send(skb, ICMP_DEST_UNREACH, ICMP_FRAG_NEEDED, htonl(mtu));
    return -E2BIG;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): skb->protocol ==) -> else {
    struct rt6_info *rt6;
    __be32 daddr;
    rt6 = skb_valid_dst(skb) ? dst_rt6_info(skb_dst(skb)) :
    core::ptr::null_mut();
    daddr = md ? dst : tunnel.parms.iph.daddr;
    if (rt6 && mtu < dst_mtu(skb_dst(skb)) &&
    mtu >= IPV6_MIN_MTU) {
    if ((daddr && !ipv4_is_multicast(daddr)) ||
    rt6.rt6i_dst.plen == 128) {
    rt6.rt6i_flags |= RTF_MODIFIED;
    dst_metric_set(skb_dst(skb), RTAX_MTU, mtu);
    }
    }
    if (!skb_is_gso(skb) && mtu >= IPV6_MIN_MTU &&
    mtu < pkt_size) {
    icmpv6_ndo_send(skb, ICMPV6_PKT_TOOBIG, 0, mtu);
    return -E2BIG;
    }
    }

    return 0;
    }
    void ip_md_tunnel_xmit(struct sk_buff *skb, struct net_device *dev,
    u8 proto, int tunnel_hlen)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    let mut headroom: u32 = sizeof(struct iphdr);
    struct ip_tunnel_info *tun_info;
    const struct ip_tunnel_key *key;
    const struct iphdr *inner_iph;
    struct rtable *rt = core::ptr::null_mut();
    struct flowi4 fl4;
    let mut df: __be16 = 0;
    u8 tos, ttl;
    bool use_cache;
    tun_info = skb_tunnel_info(skb);
    if (unlikely(!tun_info || !(tun_info.mode & IP_TUNNEL_INFO_TX) ||
    ip_tunnel_info_af(tun_info) != AF_INET))
    goto tx_error;
    key = &tun_info.key;
    memset(&(IPCB(skb).opt), 0, sizeof(IPCB(skb).opt));
    inner_iph = (const struct iphdr *)skb_inner_network_header(skb);
    tos = key.tos;
    if (tos == 1) {
    if (skb.protocol == htons(ETH_P_IP))
    tos = inner_iph.tos;
#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): skb->protocol ==) -> else {
    else if (skb.protocol == htons(ETH_P_IPV6))
    tos = ipv6_get_dsfield((const struct ipv6hdr *)inner_iph);
    }
    ip_tunnel_init_flow(&fl4, proto, key.u.ipv4.dst, key.u.ipv4.src,
    tunnel_id_to_key32(key.tun_id),
    tos & INET_DSCP_MASK, tunnel.net, 0, skb.mark,
    skb_get_hash(skb), key.flow_flags);
    if (!tunnel_hlen)
    tunnel_hlen = ip_encap_hlen(&tun_info.encap);
    if (ip_tunnel_encap(skb, &tun_info.encap, &proto, &fl4) < 0)
    goto tx_error;
    use_cache = ip_tunnel_dst_cache_usable(skb, tun_info);
    if (use_cache)
    rt = dst_cache_get_ip4(&tun_info.dst_cache, &fl4.saddr);
    if (!rt) {
    rt = ip_route_output_key(tunnel.net, &fl4);
    if (IS_ERR(rt)) {
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error;
    }
    if (use_cache)
    dst_cache_set_ip4(&tun_info.dst_cache, &rt.dst,
    fl4.saddr);
    }
    if (rt.dst.dev == dev) {
    ip_rt_put(rt);
    DEV_STATS_INC(dev, collisions);
    goto tx_error;
    }
    if (test_bit(IP_TUNNEL_DONT_FRAGMENT_BIT, key.tun_flags))
    df = htons(IP_DF);
    if (tnl_update_pmtu(dev, skb, rt, df, inner_iph, tunnel_hlen,
    key.u.ipv4.dst, true)) {
    ip_rt_put(rt);
    goto tx_error;
    }
    tos = ip_tunnel_ecn_encap(tos, inner_iph, skb);
    ttl = key.ttl;
    if (ttl == 0) {
    if (skb.protocol == htons(ETH_P_IP))
    ttl = inner_iph.ttl;
#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): skb->protocol ==) -> else {
    else if (skb.protocol == htons(ETH_P_IPV6))
    ttl = ((const struct ipv6hdr *)inner_iph).hop_limit;
    else
    ttl = ip4_dst_hoplimit(&rt.dst);
    }
    headroom += LL_RESERVED_SPACE(rt.dst.dev) + rt.dst.header_len;
    if (skb_cow_head(skb, headroom)) {
    ip_rt_put(rt);
    goto tx_dropped;
    }
    ip_tunnel_adj_headroom(dev, headroom);
    iptunnel_xmit(core::ptr::null_mut(), rt, skb, fl4.saddr, fl4.daddr, proto, tos, ttl,
    df, !net_eq(tunnel.net, dev_net(dev)), 0);
    return;
    tx_error:
    DEV_STATS_INC(dev, tx_errors);
    goto kfree;
    tx_dropped:
    DEV_STATS_INC(dev, tx_dropped);
    kfree:
    kfree_skb(skb);
    }
    EXPORT_SYMBOL_GPL(ip_md_tunnel_xmit);
    void ip_tunnel_xmit(struct sk_buff *skb, struct net_device *dev,
    const struct iphdr *tnl_params, u8 protocol)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct ip_tunnel_info *tun_info = core::ptr::null_mut();
    const struct iphdr *inner_iph;
    unsigned int max_headroom;	/* The extra header space needed */
    struct rtable *rt = core::ptr::null_mut();		/* Route to the other host */
    __be16 payload_protocol;
    let mut use_cache: bool = false;
    struct flowi4 fl4;
    let mut md: bool = false;
    bool connected;
    int err_count;
    u8 tos, ttl;
    __be32 dst;
    __be16 df;
    inner_iph = (const struct iphdr *)skb_inner_network_header(skb);
    connected = (tunnel.parms.iph.daddr != 0);
    payload_protocol = skb_protocol(skb, true);
    memset(&(IPCB(skb).opt), 0, sizeof(IPCB(skb).opt));
    dst = tnl_params.daddr;
    if (dst == 0) {
// NBMA tunnel
    if (!skb_dst(skb)) {
    DEV_STATS_INC(dev, tx_fifo_errors);
    goto tx_error;
    }
    tun_info = skb_tunnel_info(skb);
    if (tun_info && (tun_info.mode & IP_TUNNEL_INFO_TX) &&
    ip_tunnel_info_af(tun_info) == AF_INET &&
    tun_info.key.u.ipv4.dst) {
    dst = tun_info.key.u.ipv4.dst;
    md = true;
    connected = true;
    } else if (payload_protocol == htons(ETH_P_IP)) {
    rt = skb_rtable(skb);
    dst = rt_nexthop(rt, inner_iph.daddr);
    }

#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): payload_protocol ==) -> else {
    const struct in6_addr *addr6;
    struct neighbour *neigh;
    bool do_tx_error_icmp;
    int addr_type;
    neigh = dst_neigh_lookup(skb_dst(skb),
    &ipv6_hdr(skb).daddr);
    if (!neigh)
    goto tx_error;
    addr6 = (const struct in6_addr *)&neigh.primary_key;
    addr_type = ipv6_addr_type(addr6);
    if (addr_type == IPV6_ADDR_ANY) {
    addr6 = &ipv6_hdr(skb).daddr;
    addr_type = ipv6_addr_type(addr6);
    }
    if ((addr_type & IPV6_ADDR_COMPATv4) == 0)
    do_tx_error_icmp = true;
    else {
    do_tx_error_icmp = false;
    dst = addr6.s6_addr32[3];
    }
    neigh_release(neigh);
    if (do_tx_error_icmp)
    goto tx_error_icmp;
    }

    else
    goto tx_error;
    if (!md)
    connected = false;
    }
    tos = tnl_params.tos;
    if (tos & 0x1) {
    tos &= ~0x1;
    if (payload_protocol == htons(ETH_P_IP)) {
    tos = inner_iph.tos;
    connected = false;
    } else if (payload_protocol == htons(ETH_P_IPV6)) {
    tos = ipv6_get_dsfield((const struct ipv6hdr *)inner_iph);
    connected = false;
    }
    }
    ip_tunnel_init_flow(&fl4, protocol, dst, tnl_params.saddr,
    tunnel.parms.o_key, tos & INET_DSCP_MASK,
    tunnel.net, READ_ONCE(tunnel.parms.link),
    tunnel.fwmark, skb_get_hash(skb), 0);
    if (ip_tunnel_encap(skb, &tunnel.encap, &protocol, &fl4) < 0)
    goto tx_error;
    if (connected && md) {
    use_cache = ip_tunnel_dst_cache_usable(skb, tun_info);
    if (use_cache)
    rt = dst_cache_get_ip4(&tun_info.dst_cache,
    &fl4.saddr);
    } else {
    rt = connected ? dst_cache_get_ip4(&tunnel.dst_cache,
    &fl4.saddr) : core::ptr::null_mut();
    }
    if (!rt) {
    rt = ip_route_output_key(tunnel.net, &fl4);
    if (IS_ERR(rt)) {
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error;
    }
    if (use_cache)
    dst_cache_set_ip4(&tun_info.dst_cache, &rt.dst,
    fl4.saddr);
#[no_mangle]
pub unsafe extern "C" fn if(connected: !md &&) -> else {
    else if (!md && connected)
    dst_cache_set_ip4(&tunnel.dst_cache, &rt.dst,
    fl4.saddr);
    }
    if (rt.dst.dev == dev) {
    ip_rt_put(rt);
    DEV_STATS_INC(dev, collisions);
    goto tx_error;
    }
    df = tnl_params.frag_off;
    if (payload_protocol == htons(ETH_P_IP) && !tunnel.ignore_df)
    df |= (inner_iph.frag_off & htons(IP_DF));
    if (tnl_update_pmtu(dev, skb, rt, df, inner_iph, 0, 0, false)) {
    ip_rt_put(rt);
    goto tx_error;
    }
    err_count = READ_ONCE(tunnel.err_count);
    if (err_count > 0) {
    if (time_before(jiffies,
    READ_ONCE(tunnel.err_time) + IPTUNNEL_ERR_TIMEO)) {
    WRITE_ONCE(tunnel.err_count, err_count - 1);
    dst_link_failure(skb);
    } else {
    WRITE_ONCE(tunnel.err_count, 0);
    }
    }
    tos = ip_tunnel_ecn_encap(tos, inner_iph, skb);
    ttl = tnl_params.ttl;
    if (ttl == 0) {
    if (payload_protocol == htons(ETH_P_IP))
    ttl = inner_iph.ttl;

#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): payload_protocol ==) -> else {
    else if (payload_protocol == htons(ETH_P_IPV6))
    ttl = ((const struct ipv6hdr *)inner_iph).hop_limit;

    else
    ttl = ip4_dst_hoplimit(&rt.dst);
    }
    max_headroom = LL_RESERVED_SPACE(rt.dst.dev) + sizeof(struct iphdr)
    + rt.dst.header_len + ip_encap_hlen(&tunnel.encap);
    if (skb_cow_head(skb, max_headroom)) {
    ip_rt_put(rt);
    DEV_STATS_INC(dev, tx_dropped);
    kfree_skb(skb);
    return;
    }
    ip_tunnel_adj_headroom(dev, max_headroom);
    iptunnel_xmit(core::ptr::null_mut(), rt, skb, fl4.saddr, fl4.daddr, protocol, tos, ttl,
    df, !net_eq(tunnel.net, dev_net(dev)), 0);
    return;

    tx_error_icmp:
    dst_link_failure(skb);

    tx_error:
    DEV_STATS_INC(dev, tx_errors);
    kfree_skb(skb);
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_xmit);
    static void ip_tunnel_update(struct ip_tunnel_net *itn,
    struct ip_tunnel *t,
    struct net_device *dev,
    struct ip_tunnel_parm_kern *p,
    bool set_mtu,
    __u32 fwmark)
    {
    ip_tunnel_del(itn, t);
    t.parms.iph.saddr = p.iph.saddr;
    t.parms.iph.daddr = p.iph.daddr;
    t.parms.i_key = p.i_key;
    t.parms.o_key = p.o_key;
    if (dev.type != ARPHRD_ETHER) {
    __dev_addr_set(dev, &p.iph.saddr, 4);
    memcpy(dev.broadcast, &p.iph.daddr, 4);
    }
    ip_tunnel_add(itn, t);
    t.parms.iph.ttl = p.iph.ttl;
    t.parms.iph.tos = p.iph.tos;
    t.parms.iph.frag_off = p.iph.frag_off;
    if (t.parms.link != p.link || t.fwmark != fwmark) {
    int mtu;
    WRITE_ONCE(t.parms.link, p.link);
    t.fwmark = fwmark;
    mtu = ip_tunnel_bind_dev(dev);
    if (set_mtu)
    WRITE_ONCE(dev.mtu, mtu);
    }
    dst_cache_reset(&t.dst_cache);
    netdev_state_change(dev);
    }
    int ip_tunnel_ctl(struct net_device *dev, struct ip_tunnel_parm_kern *p,
    int cmd)
    {
    let mut err: c_int = 0;
    struct ip_tunnel *t = netdev_priv(dev);
    struct net *net = t.net;
    struct ip_tunnel_net *itn = net_generic(net, t.ip_tnl_net_id);
    switch (cmd) {
    case SIOCGETTUNNEL:
    if (dev == itn.fb_tunnel_dev) {
    t = ip_tunnel_find(itn, p, itn.fb_tunnel_dev.type);
    if (!t)
    t = netdev_priv(dev);
    }
    memcpy(p, &t.parms, sizeof(*p));
    break;
    case SIOCADDTUNNEL:
    case SIOCCHGTUNNEL:
    err = -EPERM;
    if (!ns_capable(net.user_ns, CAP_NET_ADMIN))
    goto done;
    if (p.iph.ttl)
    p.iph.frag_off |= htons(IP_DF);
    if (!test_bit(IP_TUNNEL_VTI_BIT, p.i_flags)) {
    if (!test_bit(IP_TUNNEL_KEY_BIT, p.i_flags))
    p.i_key = 0;
    if (!test_bit(IP_TUNNEL_KEY_BIT, p.o_flags))
    p.o_key = 0;
    }
    t = ip_tunnel_find(itn, p, itn.type);
    if (cmd == SIOCADDTUNNEL) {
    if (!t) {
    t = ip_tunnel_create(net, itn, p);
    err = PTR_ERR_OR_ZERO(t);
    break;
    }
    err = -EEXIST;
    break;
    }
    if (dev != itn.fb_tunnel_dev && cmd == SIOCCHGTUNNEL) {
    if (t) {
    if (t.dev != dev) {
    err = -EEXIST;
    break;
    }
    } else {
    let mut nflags: c_uint = 0;
    if (ipv4_is_multicast(p.iph.daddr))
    nflags = IFF_BROADCAST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: p->iph.daddr) -> else {
    else if (p.iph.daddr)
    nflags = IFF_POINTOPOINT;
    if ((dev.flags^nflags)&(IFF_POINTOPOINT|IFF_BROADCAST)) {
    err = -EINVAL;
    break;
    }
    t = netdev_priv(dev);
    }
    }
    if (t) {
    err = 0;
    ip_tunnel_update(itn, t, dev, p, true, 0);
    } else {
    err = -ENOENT;
    }
    break;
    case SIOCDELTUNNEL:
    err = -EPERM;
    if (!ns_capable(net.user_ns, CAP_NET_ADMIN))
    goto done;
    if (dev == itn.fb_tunnel_dev) {
    err = -ENOENT;
    t = ip_tunnel_find(itn, p, itn.fb_tunnel_dev.type);
    if (!t)
    goto done;
    err = -EPERM;
    if (t == netdev_priv(itn.fb_tunnel_dev))
    goto done;
    dev = t.dev;
    }
    unregister_netdevice(dev);
    err = 0;
    break;
    default:
    err = -EINVAL;
    }
    done:
    return err;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_ctl);
    bool ip_tunnel_parm_from_user(struct ip_tunnel_parm_kern *kp,
    const void __user *data)
    {
    struct ip_tunnel_parm p;
    if (copy_from_user(&p, data, sizeof(p)))
    return false;
    strscpy(kp.name, p.name);
    kp.link = p.link;
    ip_tunnel_flags_from_be16(kp.i_flags, p.i_flags);
    ip_tunnel_flags_from_be16(kp.o_flags, p.o_flags);
    kp.i_key = p.i_key;
    kp.o_key = p.o_key;
    memcpy(&kp.iph, &p.iph, min(sizeof(kp.iph), sizeof(p.iph)));
    return true;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_parm_from_user);
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_parm_to_user(data: *mut void __user, kp: *mut ip_tunnel_parm_kern) -> bool {
    bool ip_tunnel_parm_to_user(void __user *data, struct ip_tunnel_parm_kern *kp)
    {
    struct ip_tunnel_parm p;
    if (!ip_tunnel_flags_is_be16_compat(kp.i_flags) ||
    !ip_tunnel_flags_is_be16_compat(kp.o_flags))
    return false;
    memset(&p, 0, sizeof(p));
    strscpy(p.name, kp.name);
    p.link = kp.link;
    p.i_flags = ip_tunnel_flags_to_be16(kp.i_flags);
    p.o_flags = ip_tunnel_flags_to_be16(kp.o_flags);
    p.i_key = kp.i_key;
    p.o_key = kp.o_key;
    memcpy(&p.iph, &kp.iph, min(sizeof(p.iph), sizeof(kp.iph)));
    return !copy_to_user(data, &p, sizeof(p));
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_parm_to_user);
    int ip_tunnel_siocdevprivate(struct net_device *dev, struct ifreq *ifr,
    void __user *data, int cmd)
    {
    struct ip_tunnel_parm_kern p;
    int err;
    if (!ip_tunnel_parm_from_user(&p, data))
    return -EFAULT;
    err = dev.netdev_ops.ndo_tunnel_ctl(dev, &p, cmd);
    if (!err && !ip_tunnel_parm_to_user(data, &p))
    return -EFAULT;
    return err;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_siocdevprivate);
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    int ip_tunnel_change_mtu(struct net_device *dev, int new_mtu)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    let mut t_hlen: c_int = tunnel.hlen + sizeof(struct iphdr);
    let mut max_mtu: c_int = IP_MAX_MTU - t_hlen;
    if (dev.type == ARPHRD_ETHER)
    max_mtu -= dev.hard_header_len;
    if (new_mtu < ETH_MIN_MTU || new_mtu > max_mtu)
    return -EINVAL;
    WRITE_ONCE(dev.mtu, new_mtu);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_change_mtu);
#[no_mangle]
unsafe extern "C" fn ip_tunnel_dev_free(dev: *mut net_device) {
    static void ip_tunnel_dev_free(struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    gro_cells_destroy(&tunnel.gro_cells);
    dst_cache_destroy(&tunnel.dst_cache);
    }
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_dellink(dev: *mut net_device, head: *mut list_head) {
    void ip_tunnel_dellink(struct net_device *dev, struct list_head *head)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct ip_tunnel_net *itn;
    itn = net_generic(tunnel.net, tunnel.ip_tnl_net_id);
    if (itn.fb_tunnel_dev != dev) {
    ip_tunnel_del(itn, netdev_priv(dev));
    unregister_netdevice_queue(dev, head);
    }
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_dellink);
    struct net *ip_tunnel_get_link_net(const struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    return READ_ONCE(tunnel.net);
    }
    EXPORT_SYMBOL(ip_tunnel_get_link_net);
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_get_iflink(dev: *const net_device) -> c_int {
    int ip_tunnel_get_iflink(const struct net_device *dev)
    {
    const struct ip_tunnel *tunnel = netdev_priv(dev);
    return READ_ONCE(tunnel.parms.link);
    }
    EXPORT_SYMBOL(ip_tunnel_get_iflink);
    int ip_tunnel_init_net(struct net *net, unsigned int ip_tnl_net_id,
    struct rtnl_link_ops *ops, char *devname)
    {
    struct ip_tunnel_net *itn = net_generic(net, ip_tnl_net_id);
    struct ip_tunnel_parm_kern parms;
    unsigned int i;
    itn.rtnl_link_ops = ops;
    for (i = 0; i < IP_TNL_HASH_SIZE; i++)
    INIT_HLIST_HEAD(&itn.tunnels[i]);
    if (!ops || !net_has_fallback_tunnels(net)) {
    struct ip_tunnel_net *it_init_net;
    it_init_net = net_generic(&init_net, ip_tnl_net_id);
    itn.type = it_init_net.type;
    itn.fb_tunnel_dev = core::ptr::null_mut();
    return 0;
    }
    memset(&parms, 0, sizeof(parms));
    if (devname)
    strscpy(parms.name, devname, IFNAMSIZ);
    rtnl_lock();
    itn.fb_tunnel_dev = __ip_tunnel_create(net, ops, &parms);
// FB netdevice is special: we have one, and only one per netns.
// Allowing to move it to another netns is clearly unsafe.
//
    if (!IS_ERR(itn.fb_tunnel_dev)) {
    itn.fb_tunnel_dev.netns_immutable = true;
    itn.fb_tunnel_dev.mtu = ip_tunnel_bind_dev(itn.fb_tunnel_dev);
    ip_tunnel_add(itn, netdev_priv(itn.fb_tunnel_dev));
    itn.type = itn.fb_tunnel_dev.type;
    }
    rtnl_unlock();
    return PTR_ERR_OR_ZERO(itn.fb_tunnel_dev);
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_init_net);
    void ip_tunnel_delete_net(struct net *net, unsigned int id,
    struct rtnl_link_ops *ops,
    struct list_head *head)
    {
    struct ip_tunnel_net *itn = net_generic(net, id);
    struct net_device *dev, *aux;
    int h;
    ASSERT_RTNL_NET(net);
    for_each_netdev_safe(net, dev, aux)
    if (dev.rtnl_link_ops == ops)
    unregister_netdevice_queue(dev, head);
    for (h = 0; h < IP_TNL_HASH_SIZE; h++) {
    struct ip_tunnel *t;
    struct hlist_node *n;
    struct hlist_head *thead = &itn.tunnels[h];
    hlist_for_each_entry_safe(t, n, thead, hash_node)
// If dev is in the same netns, it has already
// been added to the list by the previous loop.
//
    if (!net_eq(dev_net(t.dev), net))
    unregister_netdevice_queue(t.dev, head);
    }
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_delete_net);
    int ip_tunnel_newlink(struct net *net, struct net_device *dev,
    struct nlattr *tb[], struct ip_tunnel_parm_kern *p,
    __u32 fwmark)
    {
    struct ip_tunnel *nt;
    struct ip_tunnel_net *itn;
    int mtu;
    int err;
    nt = netdev_priv(dev);
    itn = net_generic(net, nt.ip_tnl_net_id);
    if (nt.collect_md) {
    if (rtnl_dereference(itn.collect_md_tun))
    return -EEXIST;
    } else {
    if (ip_tunnel_find(itn, p, dev.type))
    return -EEXIST;
    }
    nt.net = net;
    nt.parms = *p;
    nt.fwmark = fwmark;
    err = register_netdevice(dev);
    if (err)
    goto err_register_netdevice;
    if (dev.type == ARPHRD_ETHER && !tb[IFLA_ADDRESS])
    eth_hw_addr_random(dev);
    mtu = ip_tunnel_bind_dev(dev);
    if (tb[IFLA_MTU]) {
    let mut max: c_uint = IP_MAX_MTU - (nt.hlen + sizeof(struct iphdr));
    if (dev.type == ARPHRD_ETHER)
    max -= dev.hard_header_len;
    mtu = clamp(dev.mtu, (unsigned int)ETH_MIN_MTU, max);
    }
    err = dev_set_mtu(dev, mtu);
    if (err)
    goto err_dev_set_mtu;
    ip_tunnel_add(itn, nt);
    return 0;
    err_dev_set_mtu:
    unregister_netdevice(dev);
    err_register_netdevice:
    return err;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_newlink);
    int ip_tunnel_changelink(struct net_device *dev, struct nlattr *tb[],
    struct ip_tunnel_parm_kern *p, __u32 fwmark)
    {
    struct ip_tunnel *t;
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct net *net = tunnel.net;
    struct ip_tunnel_net *itn = net_generic(net, tunnel.ip_tnl_net_id);
    if (dev == itn.fb_tunnel_dev)
    return -EINVAL;
    t = ip_tunnel_find(itn, p, dev.type);
    if (t) {
    if (t.dev != dev)
    return -EEXIST;
    } else {
    t = tunnel;
    if (dev.type != ARPHRD_ETHER) {
    let mut nflags: c_uint = 0;
    if (ipv4_is_multicast(p.iph.daddr))
    nflags = IFF_BROADCAST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: p->iph.daddr) -> else {
    else if (p.iph.daddr)
    nflags = IFF_POINTOPOINT;
    if ((dev.flags ^ nflags) &
    (IFF_POINTOPOINT | IFF_BROADCAST))
    return -EINVAL;
    }
    }
    ip_tunnel_update(itn, t, dev, p, !tb[IFLA_MTU], fwmark);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_changelink);
#[no_mangle]
pub unsafe extern "C" fn __ip_tunnel_init(dev: *mut net_device) -> c_int {
    int __ip_tunnel_init(struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct iphdr *iph = &tunnel.parms.iph;
    int err;
    dev.needs_free_netdev = true;
    dev.priv_destructor = ip_tunnel_dev_free;
    dev.pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS;
    err = dst_cache_init(&tunnel.dst_cache, GFP_KERNEL);
    if (err)
    return err;
    err = gro_cells_init(&tunnel.gro_cells, dev);
    if (err) {
    dst_cache_destroy(&tunnel.dst_cache);
    return err;
    }
    tunnel.dev = dev;
    strscpy(tunnel.parms.name, dev.name);
    iph.version		= 4;
    iph.ihl		= 5;
    if (tunnel.collect_md)
    netif_keep_dst(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(__ip_tunnel_init);
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_uninit(dev: *mut net_device) {
    void ip_tunnel_uninit(struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct net *net = tunnel.net;
    struct ip_tunnel_net *itn;
    itn = net_generic(net, tunnel.ip_tnl_net_id);
    ip_tunnel_del(itn, netdev_priv(dev));
    if (itn.fb_tunnel_dev == dev)
    WRITE_ONCE(itn.fb_tunnel_dev, core::ptr::null_mut());
    dst_cache_reset(&tunnel.dst_cache);
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_uninit);
// Do least required initialization, rest of init is done in tunnel_init call
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_setup(dev: *mut net_device, net_id: c_uint) {
    void ip_tunnel_setup(struct net_device *dev, unsigned int net_id)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    tunnel.ip_tnl_net_id = net_id;
    }
    EXPORT_SYMBOL_GPL(ip_tunnel_setup);
    MODULE_DESCRIPTION("IPv4 tunnel implementation library");
    MODULE_LICENSE("GPL");
