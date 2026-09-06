//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_flow_table_ip.c
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

// For layer 4 checksum field offset.

    static int nf_flow_state_check(struct flow_offload *flow, int proto,
    struct sk_buff *skb, unsigned int thoff)
    {
    struct tcphdr *tcph;
    if (proto != IPPROTO_TCP)
    return 0;
    tcph = (void *)(skb_network_header(skb) + thoff);
    if (tcph.syn && test_bit(NF_FLOW_CLOSING, &flow.flags)) {
    flow_offload_teardown(flow);
    return -1;
    }
    if ((tcph.fin || tcph.rst) &&
    !test_bit(NF_FLOW_CLOSING, &flow.flags))
    set_bit(NF_FLOW_CLOSING, &flow.flags);
    return 0;
    }
    static void nf_flow_nat_ip_tcp(struct sk_buff *skb, unsigned int thoff,
    __be32 addr, __be32 new_addr)
    {
    struct tcphdr *tcph;
    tcph = (void *)(skb_network_header(skb) + thoff);
    inet_proto_csum_replace4(&tcph.check, skb, addr, new_addr, true);
    }
    static void nf_flow_nat_ip_udp(struct sk_buff *skb, unsigned int thoff,
    __be32 addr, __be32 new_addr)
    {
    struct udphdr *udph;
    udph = (void *)(skb_network_header(skb) + thoff);
    if (udph.check || skb.ip_summed == CHECKSUM_PARTIAL) {
    inet_proto_csum_replace4(&udph.check, skb, addr,
    new_addr, true);
    if (!udph.check)
    udph.check = CSUM_MANGLED_0;
    }
    }
    static void nf_flow_nat_ip_l4proto(struct sk_buff *skb, struct iphdr *iph,
    unsigned int thoff, __be32 addr,
    __be32 new_addr)
    {
    switch (iph.protocol) {
    case IPPROTO_TCP:
    nf_flow_nat_ip_tcp(skb, thoff, addr, new_addr);
    break;
    case IPPROTO_UDP:
    nf_flow_nat_ip_udp(skb, thoff, addr, new_addr);
    break;
    }
    }
    static void nf_flow_snat_ip(const struct flow_offload *flow,
    struct sk_buff *skb, struct iphdr *iph,
    unsigned int thoff, enum flow_offload_tuple_dir dir)
    {
    __be32 addr, new_addr;
    switch (dir) {
    case FLOW_OFFLOAD_DIR_ORIGINAL:
    addr = iph.saddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.dst_v4.s_addr;
    iph.saddr = new_addr;
    break;
    case FLOW_OFFLOAD_DIR_REPLY:
    addr = iph.daddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.src_v4.s_addr;
    iph.daddr = new_addr;
    break;
    }
    csum_replace4(&iph.check, addr, new_addr);
    nf_flow_nat_ip_l4proto(skb, iph, thoff, addr, new_addr);
    }
    static void nf_flow_dnat_ip(const struct flow_offload *flow,
    struct sk_buff *skb, struct iphdr *iph,
    unsigned int thoff, enum flow_offload_tuple_dir dir)
    {
    __be32 addr, new_addr;
    switch (dir) {
    case FLOW_OFFLOAD_DIR_ORIGINAL:
    addr = iph.daddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.src_v4.s_addr;
    iph.daddr = new_addr;
    break;
    case FLOW_OFFLOAD_DIR_REPLY:
    addr = iph.saddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.dst_v4.s_addr;
    iph.saddr = new_addr;
    break;
    }
    csum_replace4(&iph.check, addr, new_addr);
    nf_flow_nat_ip_l4proto(skb, iph, thoff, addr, new_addr);
    }
    static void nf_flow_nat_ip(const struct flow_offload *flow, struct sk_buff *skb,
    unsigned int thoff, enum flow_offload_tuple_dir dir,
    struct iphdr *iph)
    {
    if (test_bit(NF_FLOW_SNAT, &flow.flags)) {
    nf_flow_snat_port(flow, skb, thoff, iph.protocol, dir);
    nf_flow_snat_ip(flow, skb, iph, thoff, dir);
    }
    if (test_bit(NF_FLOW_DNAT, &flow.flags)) {
    nf_flow_dnat_port(flow, skb, thoff, iph.protocol, dir);
    nf_flow_dnat_ip(flow, skb, iph, thoff, dir);
    }
    }
#[no_mangle]
unsafe extern "C" fn ip_has_options(thoff: c_uint) -> bool {
    static bool ip_has_options(unsigned int thoff)
    {
    return thoff != sizeof(struct iphdr);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flowtable_ctx {
    pub in: *const net_device,
    pub ether_type: __be16,
    pub offset: u32,
    pub hdrsize: u32,
    struct {
// Tunnel IP header size
    pub hdr_size: u32,
// IP tunnel protocol
    pub inner_proto: u8,
    pub tun: },
}

    static void nf_flow_tuple_encap(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb,
    struct flow_offload_tuple *tuple)
    {
    struct vlan_ethhdr *veth;
    struct pppoe_hdr *phdr;
    struct ipv6hdr *ip6h;
    struct iphdr *iph;
    let mut offset: u16 = 0;
    let mut i: c_int = 0;
    if (skb_vlan_tag_present(skb)) {
    tuple.encap[i].id = skb_vlan_tag_get(skb);
    tuple.encap[i].proto = skb.vlan_proto;
    i++;
    }
    switch (skb.protocol) {
    case htons(ETH_P_8021Q):
    veth = (struct vlan_ethhdr *)skb_mac_header(skb);
    tuple.encap[i].id = ntohs(veth.h_vlan_TCI);
    tuple.encap[i].proto = skb.protocol;
    offset += VLAN_HLEN;
    break;
    case htons(ETH_P_PPP_SES):
    phdr = (struct pppoe_hdr *)skb_network_header(skb);
    tuple.encap[i].id = ntohs(phdr.sid);
    tuple.encap[i].proto = skb.protocol;
    offset += PPPOE_SES_HLEN;
    break;
    }
    switch (ctx.ether_type) {
    case htons(ETH_P_IP):
    iph = (struct iphdr *)(skb_network_header(skb) + offset);
    if (ctx.tun.inner_proto == IPPROTO_IPIP) {
    tuple.tun.dst_v4.s_addr = iph.daddr;
    tuple.tun.src_v4.s_addr = iph.saddr;
    tuple.tun.inner_proto = IPPROTO_IPIP;
    }
    break;
    case htons(ETH_P_IPV6):
    ip6h = (struct ipv6hdr *)(skb_network_header(skb) + offset);
    if (ctx.tun.inner_proto == IPPROTO_IPV6) {
    tuple.tun.dst_v6 = ip6h.daddr;
    tuple.tun.src_v6 = ip6h.saddr;
    tuple.tun.inner_proto = IPPROTO_IPV6;
    }
    break;
    default:
    break;
    }
    }
    static int nf_flow_tuple_ip(struct nf_flowtable_ctx *ctx, struct sk_buff *skb,
    struct flow_offload_tuple *tuple)
    {
    struct flow_ports *ports;
    unsigned int thoff;
    struct iphdr *iph;
    u8 ipproto;
    if (!pskb_may_pull(skb, sizeof(*iph) + ctx.offset))
    return -1;
    iph = (struct iphdr *)(skb_network_header(skb) + ctx.offset);
    thoff = (iph.ihl * 4);
    if (ip_is_fragment(iph) ||
    unlikely(ip_has_options(thoff)))
    return -1;
    thoff += ctx.offset;
    ipproto = iph.protocol;
    switch (ipproto) {
    case IPPROTO_TCP:
    ctx.hdrsize = sizeof(struct tcphdr);
    break;
    case IPPROTO_UDP:
    ctx.hdrsize = sizeof(struct udphdr);
    break;

    case IPPROTO_GRE:
    ctx.hdrsize = sizeof(struct gre_base_hdr);
    break;

    default:
    return -1;
    }
    if (iph.ttl <= 1)
    return -1;
    if (!pskb_may_pull(skb, thoff + ctx.hdrsize))
    return -1;
    switch (ipproto) {
    case IPPROTO_TCP:
    case IPPROTO_UDP:
    ports = (struct flow_ports *)(skb_network_header(skb) + thoff);
    tuple.src_port		= ports.source;
    tuple.dst_port		= ports.dest;
    break;
    case IPPROTO_GRE: {
    struct gre_base_hdr *greh;
    greh = (struct gre_base_hdr *)(skb_network_header(skb) + thoff);
    if ((greh.flags & GRE_VERSION) != GRE_VERSION_0)
    return -1;
    break;
    }
    }
    iph = (struct iphdr *)(skb_network_header(skb) + ctx.offset);
    tuple.src_v4.s_addr	= iph.saddr;
    tuple.dst_v4.s_addr	= iph.daddr;
    tuple.l3proto		= AF_INET;
    tuple.l4proto		= ipproto;
    tuple.iifidx		= ctx.in.ifindex;
    nf_flow_tuple_encap(ctx, skb, tuple);
    return 0;
    }
// Based on ip_exceeds_mtu().
#[no_mangle]
unsafe extern "C" fn nf_flow_exceeds_mtu(skb: *const sk_buff, mtu: c_uint) -> bool {
    static bool nf_flow_exceeds_mtu(const struct sk_buff *skb, unsigned int mtu)
    {
    if (skb.len <= mtu)
    return false;
    if (skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu))
    return false;
    return true;
    }
    static unsigned int nf_flow_xmit_xfrm(struct sk_buff *skb,
    const struct nf_hook_state *state,
    struct dst_entry *dst)
    {
    skb_orphan(skb);
    skb_dst_drop(skb);
    skb_dst_set_noref(skb, dst);
    dst_output(state.net, state.sk, skb);
    return NF_STOLEN;
    }
    static bool nf_flow_ip4_tunnel_proto(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb)
    {
    struct iphdr *iph;
    u16 size;
    if (!pskb_may_pull(skb, sizeof(*iph) + ctx.offset))
    return false;
    iph = (struct iphdr *)(skb_network_header(skb) + ctx.offset);
    if (iph.ihl < 5)
    return false;
    size = iph.ihl << 2;
    if (ip_is_fragment(iph) || unlikely(ip_has_options(size)))
    return false;
    if (iph.ttl <= 1)
    return false;
    if (iph.protocol == IPPROTO_IPIP) {
    ctx.tun.inner_proto = iph.protocol;
    ctx.tun.hdr_size = size;
    ctx.offset += ctx.tun.hdr_size;
    }
    return true;
    }
    static bool nf_flow_ip6_tunnel_proto(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb)
    {

    struct ipv6hdr *ip6h;
    if (!pskb_may_pull(skb, sizeof(*ip6h) + ctx.offset))
    return false;
    ip6h = (struct ipv6hdr *)(skb_network_header(skb) + ctx.offset);
    if (ip6h.hop_limit <= 1)
    return false;
    if (ipv6_ext_hdr(ip6h.nexthdr))
    return false;
    if (ip6h.nexthdr == IPPROTO_IPV6) {
    ctx.tun.inner_proto = ip6h.nexthdr;
    ctx.tun.hdr_size = sizeof(*ip6h);
    ctx.offset += ctx.tun.hdr_size;
    }
    return true;

    return false;

    }
    static void nf_flow_ip_tunnel_pop(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb)
    {
    if (ctx.tun.inner_proto != IPPROTO_IPIP &&
    ctx.tun.inner_proto != IPPROTO_IPV6)
    return;
    skb_pull(skb, ctx.tun.hdr_size);
    skb_reset_network_header(skb);
    }
    static bool nf_flow_skb_encap_protocol(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb)
    {
    struct vlan_ethhdr *veth;
    __be16 ether_type;
    let mut ret: bool = false;
    switch (skb.protocol) {
    case htons(ETH_P_8021Q):
    if (!pskb_may_pull(skb, skb_mac_offset(skb) + sizeof(*veth)))
    return false;
    veth = (struct vlan_ethhdr *)skb_mac_header(skb);
    ctx.ether_type = veth.h_vlan_encapsulated_proto;
    ctx.offset += VLAN_HLEN;
    ret = true;
    break;
    case htons(ETH_P_PPP_SES):
    if (!nf_flow_pppoe_proto(skb, &ether_type))
    return false;
    ctx.ether_type = ether_type;
    ctx.offset += PPPOE_SES_HLEN;
    ret = true;
    break;
    case htons(ETH_P_IP):
    case htons(ETH_P_IPV6):
    ctx.ether_type = skb.protocol;
    break;
    default:
    return false;
    }
    switch (ctx.ether_type) {
    case htons(ETH_P_IP):
    ret = nf_flow_ip4_tunnel_proto(ctx, skb);
    break;
    case htons(ETH_P_IPV6):
    ret = nf_flow_ip6_tunnel_proto(ctx, skb);
    break;
    default:
    break;
    }
    return ret;
    }
    static void nf_flow_encap_pop(struct nf_flowtable_ctx *ctx,
    struct sk_buff *skb,
    struct flow_offload_tuple_rhash *tuplehash)
    {
    struct vlan_hdr *vlan_hdr;
    int i;
    for (i = 0; i < tuplehash.tuple.encap_num; i++) {
    if (skb_vlan_tag_present(skb)) {
    __vlan_hwaccel_clear_tag(skb);
    continue;
    }
    switch (skb.protocol) {
    case htons(ETH_P_8021Q):
    vlan_hdr = (struct vlan_hdr *)skb.data;
    skb_pull_rcsum(skb, VLAN_HLEN);
    vlan_set_encap_proto(skb, vlan_hdr);
    skb_reset_network_header(skb);
    break;
    case htons(ETH_P_PPP_SES):
    skb.protocol = __nf_flow_pppoe_proto(skb);
    skb_pull_rcsum(skb, PPPOE_SES_HLEN);
    skb_reset_network_header(skb);
    break;
    }
    }
    if (skb.protocol == htons(ETH_P_IP) ||
    skb.protocol == htons(ETH_P_IPV6))
    nf_flow_ip_tunnel_pop(ctx, skb);
    }
    static struct flow_offload_tuple_rhash *
    nf_flow_offload_lookup(struct nf_flowtable_ctx *ctx,
    struct nf_flowtable *flow_table, struct sk_buff *skb)
    {
    let mut tuple: flow_offload_tuple = {};
    if (nf_flow_tuple_ip(ctx, skb, &tuple) < 0)
    return core::ptr::null_mut();
    return flow_offload_lookup(flow_table, &tuple);
    }
    static int nf_flow_offload_forward(struct nf_flowtable_ctx *ctx,
    struct nf_flowtable *flow_table,
    struct flow_offload_tuple_rhash *tuplehash,
    struct sk_buff *skb)
    {
    enum flow_offload_tuple_dir dir;
    struct flow_offload *flow;
    unsigned int thoff, mtu;
    struct iphdr *iph;
    dir = tuplehash.tuple.dir;
    flow = container_of(tuplehash, struct flow_offload, tuplehash[dir]);
    mtu = flow.tuplehash[dir].tuple.mtu + ctx.offset;
    if (flow.tuplehash[!dir].tuple.tun_num)
    mtu -= sizeof(*iph);
    if (unlikely(nf_flow_exceeds_mtu(skb, mtu)))
    return 0;
    iph = (struct iphdr *)(skb_network_header(skb) + ctx.offset);
    thoff = (iph.ihl * 4) + ctx.offset;
    if (nf_flow_state_check(flow, iph.protocol, skb, thoff))
    return 0;
    if (!nf_flow_dst_check(&tuplehash.tuple)) {
    flow_offload_teardown(flow);
    return 0;
    }
    if (skb_ensure_writable(skb, thoff + ctx.hdrsize))
    return -1;
    flow_offload_refresh(flow_table, flow, false);
    nf_flow_encap_pop(ctx, skb, tuplehash);
    thoff -= ctx.offset;
    iph = ip_hdr(skb);
    nf_flow_nat_ip(flow, skb, thoff, dir, iph);
    ip_decrease_ttl(iph);
    skb_clear_tstamp(skb);
    if (flow_table.flags & NF_FLOWTABLE_COUNTER)
    nf_ct_acct_update(flow.ct, tuplehash.tuple.dir, skb.len);
    return 1;
    }
// Similar to skb_vlan_push.
    static int nf_flow_vlan_push(struct sk_buff *skb, __be16 proto, u16 id,
    u32 needed_headroom)
    {
    if (skb_vlan_tag_present(skb)) {
    struct vlan_hdr *vhdr;
    if (skb_cow_head(skb, needed_headroom + VLAN_HLEN))
    return -1;
    __skb_push(skb, VLAN_HLEN);
    if (skb_mac_header_was_set(skb))
    skb.mac_header -= VLAN_HLEN;
    vhdr = (struct vlan_hdr *)skb.data;
    skb.network_header -= VLAN_HLEN;
    vhdr.h_vlan_TCI = htons(skb_vlan_tag_get(skb));
    vhdr.h_vlan_encapsulated_proto = skb.protocol;
    skb.protocol = skb.vlan_proto;
    skb_postpush_rcsum(skb, skb.data, VLAN_HLEN);
    }
    __vlan_hwaccel_put_tag(skb, proto, id);
    return 0;
    }
    static int nf_flow_pppoe_push(struct sk_buff *skb, u16 id,
    u32 needed_headroom)
    {
    let mut data_len: c_int = skb.len + sizeof(__be16);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_hdr {
    pub hdr: pppoe_hdr,
    pub proto: __be16,
    pub ph: *mut },
    pub proto: __be16,
    if (skb_cow_head(skb, needed_headroom + PPPOE_SES_HLEN))
    pub -1: return,
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    pub htons(PPP_IP): proto =,
    case htons(ETH_P_IPV6):
    pub htons(PPP_IPV6): proto =,
    default:
    pub -1: return,
    }
    pub PPPOE_SES_HLEN): __skb_push(skb,,
    pub )(skb->data): *mut ph = (struct ppp_hdr,
    pub 1: ph->hdr.ver =,
    pub 1: ph->hdr.type =,
    pub 0: ph->hdr.code =,
    pub htons(id): ph->hdr.sid =,
    pub htons(data_len): ph->hdr.length =,
    pub proto: ph->proto =,
    pub htons(ETH_P_PPP_SES): skb->protocol =,
    pub 0: return,
    }
    static int nf_flow_tunnel_ipip_push(struct net *net, struct sk_buff *skb,
    struct flow_offload_tuple *tuple,
    struct dst_entry *dst, __be32 *ip_daddr)
    {
    pub )skb_network_header(skb): *mut *mut iphdr iph = (iphdr,
    pub dst_rtable(dst): *mut *mut rtable rt =,
    pub iph->ttl: u8 tos = iph->tos, ttl =,
    pub iph->frag_off: __be16 frag_off =,
    pub sizeof(*iph): *mut u32 headroom =,
    pub err: c_int,
    pub SKB_GSO_IPXIP4): err = iptunnel_handle_offloads(skb,,
    if (err)
    pub err: return,
    pub IPPROTO_IPIP): skb_set_inner_ipproto(skb,,
    pub rt->dst.header_len: headroom += LL_RESERVED_SPACE(rt->dst.dev) +,
    pub headroom): err = skb_cow_head(skb,,
    if (err)
    pub err: return,
    pub true): skb_scrub_packet(skb,,
// Push down and install the IP header.
    pub sizeof(*iph)): *mut skb_push(skb,,
    pub ip_hdr(skb): iph =,
    pub 4: iph->version =,
    pub 2: *mut *mut iph->ihl = sizeof(iph) >>,
    pub frag_off: iph->frag_off = ip_mtu_locked(&rt->dst) ? 0 :,
    pub tuple->tun.inner_proto: iph->protocol =,
    pub tos: iph->tos =,
    pub tuple->tun.src_v4.s_addr: iph->daddr =,
    pub tuple->tun.dst_v4.s_addr: iph->saddr =,
    pub ttl: iph->ttl =,
    pub htons(skb->len): iph->tot_len =,
    pub 1): __ip_select_ident(net, iph, skb_shinfo(skb)->gso_segs ?:,
// ip_daddr = tuple->tun.src_v4.s_addr;
    pub 0: return,
    }
    static int nf_flow_tunnel_v4_push(struct net *net, struct sk_buff *skb,
    struct flow_offload_tuple *tuple,
    struct dst_entry *dst,  __be32 *ip_daddr)
    {
    if (tuple.tun_num)
    pub ip_daddr): return nf_flow_tunnel_ipip_push(net, skb, tuple, dst,,
    pub 0: return,
    }
    static int nf_flow_tunnel_ip6ip6_push(struct net *net, struct sk_buff *skb,
    struct flow_offload_tuple *tuple,
    struct dst_entry *dst,
    struct in6_addr **ip6_daddr)
    {
    pub )skb_network_header(skb): *mut *mut ipv6hdr ip6h = (ipv6hdr,
    pub ipv6_get_dsfield(ip6h): __u8 dsfield =,
    pub dst_rtable(dst): *mut *mut rtable rt =,
    struct flowi6 fl6 = {
    .daddr = tuple.tun.src_v6,
    .saddr = tuple.tun.dst_v6,
    .flowi6_proto = IPPROTO_IPV6,
}

    let mut hop_limit: u8 = ip6h.hop_limit;
    int err, mtu;
    u32 headroom;
    err = iptunnel_handle_offloads(skb, SKB_GSO_IPXIP6);
    if (err)
    return err;
    skb_set_inner_ipproto(skb, IPPROTO_IPV6);
    headroom = sizeof(*ip6h) + LL_RESERVED_SPACE(rt.dst.dev) +
    rt.dst.header_len;
    err = skb_cow_head(skb, headroom);
    if (err)
    return err;
    skb_scrub_packet(skb, true);
    mtu = dst_mtu(&rt.dst) - sizeof(*ip6h);
    mtu = max(mtu, IPV6_MIN_MTU);
    skb_dst_update_pmtu_no_confirm(skb, mtu);
    skb_push(skb, sizeof(*ip6h));
    skb_reset_network_header(skb);
    ip6h = ipv6_hdr(skb);
    ip6_flow_hdr(ip6h, dsfield,
    ip6_make_flowlabel(net, skb, fl6.flowlabel, true, &fl6));
    ip6h.hop_limit = hop_limit;
    ip6h.nexthdr = IPPROTO_IPV6;
    ip6h.daddr = tuple.tun.src_v6;
    ip6h.saddr = tuple.tun.dst_v6;
    ipv6_hdr(skb).payload_len = htons(skb.len - sizeof(*ip6h));
    IP6CB(skb).nhoff = offsetof(struct ipv6hdr, nexthdr);
// ip6_daddr = &tuple->tun.src_v6;
    return 0;
    }
    static int nf_flow_tunnel_v6_push(struct net *net, struct sk_buff *skb,
    struct flow_offload_tuple *tuple,
    struct dst_entry *dst,
    struct in6_addr **ip6_daddr)
    {
    if (tuple.tun_num)
    return nf_flow_tunnel_ip6ip6_push(net, skb, tuple, dst, ip6_daddr);
    return 0;
    }
    static int nf_flow_encap_push(struct sk_buff *skb,
    struct flow_offload_tuple *tuple,
    struct net_device *outdev)
    {
    let mut needed_headroom: u32 = LL_RESERVED_SPACE(outdev);
    int i;
    for (i = tuple.encap_num - 1; i >= 0; i--) {
    switch (tuple.encap[i].proto) {
    case htons(ETH_P_8021Q):
    case htons(ETH_P_8021AD):
    if (nf_flow_vlan_push(skb, tuple.encap[i].proto,
    tuple.encap[i].id,
    needed_headroom) < 0)
    return -1;
    break;
    case htons(ETH_P_PPP_SES):
    if (nf_flow_pppoe_push(skb, tuple.encap[i].id,
    needed_headroom) < 0)
    return -1;
    break;
    }
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flow_xmit {
    pub dest: *const c_void,
    pub source: *const c_void,
    pub outdev: *mut net_device,
    pub tuple: *mut flow_offload_tuple,
    pub needs_gso_segment: bool,
}

    static void __nf_flow_queue_xmit(struct net *net, struct sk_buff *skb,
    struct nf_flow_xmit *xmit)
    {
    struct net_device *dev = xmit.outdev;
    let mut hh_len: c_uint = LL_RESERVED_SPACE(dev);
    if (unlikely(skb_headroom(skb) < hh_len && dev.header_ops)) {
    skb = skb_expand_head(skb, hh_len);
    if (!skb)
    return;
    }
    skb.dev = dev;
    dev_hard_header(skb, dev, ntohs(skb.protocol),
    xmit.dest, xmit.source, skb.len);
    dev_queue_xmit(skb);
    }
    static unsigned int nf_flow_encap_gso_xmit(struct net *net, struct sk_buff *skb,
    struct nf_flow_xmit *xmit)
    {
    struct sk_buff *segs, *nskb;
    segs = skb_gso_segment(skb, 0);
    if (IS_ERR(segs))
    return NF_DROP;
    if (segs)
    consume_skb(skb);
    else
    segs = skb;
    skb_list_walk_safe(segs, segs, nskb) {
    skb_mark_not_on_list(segs);
    if (nf_flow_encap_push(segs, xmit.tuple, xmit.outdev) < 0) {
    kfree_skb(segs);
    kfree_skb_list(nskb);
    return NF_STOLEN;
    }
    __nf_flow_queue_xmit(net, segs, xmit);
    }
    return NF_STOLEN;
    }
    static unsigned int nf_flow_queue_xmit(struct net *net, struct sk_buff *skb,
    struct nf_flow_xmit *xmit)
    {
    if (xmit.tuple.encap_num) {
    if (skb_is_gso(skb) && xmit.needs_gso_segment)
    return nf_flow_encap_gso_xmit(net, skb, xmit);
    if (nf_flow_encap_push(skb, xmit.tuple, xmit.outdev) < 0)
    return NF_DROP;
    }
    __nf_flow_queue_xmit(net, skb, xmit);
    return NF_STOLEN;
    }
    static int nf_flow_queue_xmit4(struct sk_buff *skb,
    struct flow_offload_tuple_rhash *tuplehash,
    const struct nf_hook_state *state)
    {
    struct flow_offload_tuple *other_tuple;
    enum flow_offload_tuple_dir dir;
    let mut xmit: nf_flow_xmit = {};
    struct flow_offload *flow;
    struct neighbour *neigh;
    struct rtable *rt;
    __be32 ip_daddr;
    if (unlikely(tuplehash.tuple.xmit_type == FLOW_OFFLOAD_XMIT_XFRM)) {
    rt = dst_rtable(tuplehash.tuple.dst_cache);
    memset(skb.cb, 0, sizeof(struct inet_skb_parm));
    IPCB(skb).iif = skb.dev.ifindex;
    IPCB(skb).flags = IPSKB_FORWARDED;
    return nf_flow_xmit_xfrm(skb, state, &rt.dst);
    }
    dir = tuplehash.tuple.dir;
    flow = container_of(tuplehash, struct flow_offload, tuplehash[dir]);
    other_tuple = &flow.tuplehash[!dir].tuple;
    ip_daddr = other_tuple.src_v4.s_addr;
    if (nf_flow_tunnel_v4_push(state.net, skb, other_tuple,
    tuplehash.tuple.dst_cache, &ip_daddr) < 0)
    return NF_DROP;
    switch (tuplehash.tuple.xmit_type) {
    case FLOW_OFFLOAD_XMIT_NEIGH:
    rt = dst_rtable(tuplehash.tuple.dst_cache);
    xmit.outdev = dev_get_by_index_rcu(state.net, tuplehash.tuple.ifidx);
    if (!xmit.outdev) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    neigh = ip_neigh_gw4(rt.dst.dev, rt_nexthop(rt, ip_daddr));
    if (IS_ERR(neigh)) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    xmit.dest = neigh.ha;
    skb_dst_drop(skb);
    skb_dst_set_noref(skb, &rt.dst);
    break;
    case FLOW_OFFLOAD_XMIT_DIRECT:
    xmit.outdev = dev_get_by_index_rcu(state.net, tuplehash.tuple.out.ifidx);
    if (!xmit.outdev) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    xmit.dest = tuplehash.tuple.out.h_dest;
    xmit.source = tuplehash.tuple.out.h_source;
    break;
    default:
    WARN_ON_ONCE(1);
    return NF_DROP;
    }
    xmit.tuple = other_tuple;
    xmit.needs_gso_segment = tuplehash.tuple.needs_gso_segment;
    return nf_flow_queue_xmit(state.net, skb, &xmit);
    }
    unsigned int
    nf_flow_offload_ip_hook(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct flow_offload_tuple_rhash *tuplehash;
    struct nf_flowtable *flow_table = priv;
    struct nf_flowtable_ctx ctx = {
    .in	= state.in,
    };
    int ret;
    if (!nf_flow_skb_encap_protocol(&ctx, skb))
    return NF_ACCEPT;
    if (unlikely(ctx.ether_type != htons(ETH_P_IP)))
    return NF_ACCEPT;
    tuplehash = nf_flow_offload_lookup(&ctx, flow_table, skb);
    if (!tuplehash)
    return NF_ACCEPT;
    ret = nf_flow_offload_forward(&ctx, flow_table, tuplehash, skb);
    if (ret < 0)
    return NF_DROP;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret ==) -> else {
    else if (ret == 0)
    return NF_ACCEPT;
    return nf_flow_queue_xmit4(skb, tuplehash, state);
    }
    EXPORT_SYMBOL_GPL(nf_flow_offload_ip_hook);
    static void nf_flow_nat_ipv6_tcp(struct sk_buff *skb, unsigned int thoff,
    struct in6_addr *addr,
    struct in6_addr *new_addr,
    struct ipv6hdr *ip6h)
    {
    struct tcphdr *tcph;
    tcph = (void *)(skb_network_header(skb) + thoff);
    inet_proto_csum_replace16(&tcph.check, skb, addr.s6_addr32,
    new_addr.s6_addr32, true);
    }
    static void nf_flow_nat_ipv6_udp(struct sk_buff *skb, unsigned int thoff,
    struct in6_addr *addr,
    struct in6_addr *new_addr)
    {
    struct udphdr *udph;
    udph = (void *)(skb_network_header(skb) + thoff);
    if (udph.check || skb.ip_summed == CHECKSUM_PARTIAL) {
    inet_proto_csum_replace16(&udph.check, skb, addr.s6_addr32,
    new_addr.s6_addr32, true);
    if (!udph.check)
    udph.check = CSUM_MANGLED_0;
    }
    }
    static void nf_flow_nat_ipv6_l4proto(struct sk_buff *skb, struct ipv6hdr *ip6h,
    unsigned int thoff, struct in6_addr *addr,
    struct in6_addr *new_addr)
    {
    switch (ip6h.nexthdr) {
    case IPPROTO_TCP:
    nf_flow_nat_ipv6_tcp(skb, thoff, addr, new_addr, ip6h);
    break;
    case IPPROTO_UDP:
    nf_flow_nat_ipv6_udp(skb, thoff, addr, new_addr);
    break;
    }
    }
    static void nf_flow_snat_ipv6(const struct flow_offload *flow,
    struct sk_buff *skb, struct ipv6hdr *ip6h,
    unsigned int thoff,
    enum flow_offload_tuple_dir dir)
    {
    struct in6_addr addr, new_addr;
    switch (dir) {
    case FLOW_OFFLOAD_DIR_ORIGINAL:
    addr = ip6h.saddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.dst_v6;
    ip6h.saddr = new_addr;
    break;
    case FLOW_OFFLOAD_DIR_REPLY:
    addr = ip6h.daddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.src_v6;
    ip6h.daddr = new_addr;
    break;
    }
    nf_flow_nat_ipv6_l4proto(skb, ip6h, thoff, &addr, &new_addr);
    }
    static void nf_flow_dnat_ipv6(const struct flow_offload *flow,
    struct sk_buff *skb, struct ipv6hdr *ip6h,
    unsigned int thoff,
    enum flow_offload_tuple_dir dir)
    {
    struct in6_addr addr, new_addr;
    switch (dir) {
    case FLOW_OFFLOAD_DIR_ORIGINAL:
    addr = ip6h.daddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.src_v6;
    ip6h.daddr = new_addr;
    break;
    case FLOW_OFFLOAD_DIR_REPLY:
    addr = ip6h.saddr;
    new_addr = flow.tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.dst_v6;
    ip6h.saddr = new_addr;
    break;
    }
    nf_flow_nat_ipv6_l4proto(skb, ip6h, thoff, &addr, &new_addr);
    }
    static void nf_flow_nat_ipv6(const struct flow_offload *flow,
    struct sk_buff *skb,
    enum flow_offload_tuple_dir dir,
    struct ipv6hdr *ip6h)
    {
    let mut thoff: c_uint = sizeof(*ip6h);
    if (test_bit(NF_FLOW_SNAT, &flow.flags)) {
    nf_flow_snat_port(flow, skb, thoff, ip6h.nexthdr, dir);
    nf_flow_snat_ipv6(flow, skb, ip6h, thoff, dir);
    }
    if (test_bit(NF_FLOW_DNAT, &flow.flags)) {
    nf_flow_dnat_port(flow, skb, thoff, ip6h.nexthdr, dir);
    nf_flow_dnat_ipv6(flow, skb, ip6h, thoff, dir);
    }
    }
    static int nf_flow_tuple_ipv6(struct nf_flowtable_ctx *ctx, struct sk_buff *skb,
    struct flow_offload_tuple *tuple)
    {
    struct flow_ports *ports;
    struct ipv6hdr *ip6h;
    unsigned int thoff;
    u8 nexthdr;
    thoff = sizeof(*ip6h) + ctx.offset;
    if (!pskb_may_pull(skb, thoff))
    return -1;
    ip6h = (struct ipv6hdr *)(skb_network_header(skb) + ctx.offset);
    nexthdr = ip6h.nexthdr;
    switch (nexthdr) {
    case IPPROTO_TCP:
    ctx.hdrsize = sizeof(struct tcphdr);
    break;
    case IPPROTO_UDP:
    ctx.hdrsize = sizeof(struct udphdr);
    break;

    case IPPROTO_GRE:
    ctx.hdrsize = sizeof(struct gre_base_hdr);
    break;

    default:
    return -1;
    }
    if (ip6h.hop_limit <= 1)
    return -1;
    if (!pskb_may_pull(skb, thoff + ctx.hdrsize))
    return -1;
    switch (nexthdr) {
    case IPPROTO_TCP:
    case IPPROTO_UDP:
    ports = (struct flow_ports *)(skb_network_header(skb) + thoff);
    tuple.src_port		= ports.source;
    tuple.dst_port		= ports.dest;
    break;
    case IPPROTO_GRE: {
    struct gre_base_hdr *greh;
    greh = (struct gre_base_hdr *)(skb_network_header(skb) + thoff);
    if ((greh.flags & GRE_VERSION) != GRE_VERSION_0)
    return -1;
    break;
    }
    }
    ip6h = (struct ipv6hdr *)(skb_network_header(skb) + ctx.offset);
    tuple.src_v6		= ip6h.saddr;
    tuple.dst_v6		= ip6h.daddr;
    tuple.l3proto		= AF_INET6;
    tuple.l4proto		= nexthdr;
    tuple.iifidx		= ctx.in.ifindex;
    nf_flow_tuple_encap(ctx, skb, tuple);
    return 0;
    }
    static int nf_flow_offload_ipv6_forward(struct nf_flowtable_ctx *ctx,
    struct nf_flowtable *flow_table,
    struct flow_offload_tuple_rhash *tuplehash,
    struct sk_buff *skb)
    {
    enum flow_offload_tuple_dir dir;
    struct flow_offload *flow;
    unsigned int thoff, mtu;
    struct ipv6hdr *ip6h;
    dir = tuplehash.tuple.dir;
    flow = container_of(tuplehash, struct flow_offload, tuplehash[dir]);
    mtu = flow.tuplehash[dir].tuple.mtu + ctx.offset;
    if (flow.tuplehash[!dir].tuple.tun_num)
    mtu -= sizeof(*ip6h);
    if (unlikely(nf_flow_exceeds_mtu(skb, mtu)))
    return 0;
    ip6h = (struct ipv6hdr *)(skb_network_header(skb) + ctx.offset);
    thoff = sizeof(*ip6h) + ctx.offset;
    if (nf_flow_state_check(flow, ip6h.nexthdr, skb, thoff))
    return 0;
    if (!nf_flow_dst_check(&tuplehash.tuple)) {
    flow_offload_teardown(flow);
    return 0;
    }
    if (skb_ensure_writable(skb, thoff + ctx.hdrsize))
    return -1;
    flow_offload_refresh(flow_table, flow, false);
    nf_flow_encap_pop(ctx, skb, tuplehash);
    ip6h = ipv6_hdr(skb);
    nf_flow_nat_ipv6(flow, skb, dir, ip6h);
    ip6h.hop_limit--;
    skb_clear_tstamp(skb);
    if (flow_table.flags & NF_FLOWTABLE_COUNTER)
    nf_ct_acct_update(flow.ct, tuplehash.tuple.dir, skb.len);
    return 1;
    }
    static struct flow_offload_tuple_rhash *
    nf_flow_offload_ipv6_lookup(struct nf_flowtable_ctx *ctx,
    struct nf_flowtable *flow_table,
    struct sk_buff *skb)
    {
    let mut tuple: flow_offload_tuple = {};
    if (nf_flow_tuple_ipv6(ctx, skb, &tuple) < 0)
    return core::ptr::null_mut();
    return flow_offload_lookup(flow_table, &tuple);
    }
    static int nf_flow_queue_xmit6(struct sk_buff *skb,
    struct flow_offload_tuple_rhash *tuplehash,
    const struct nf_hook_state *state)
    {
    struct flow_offload_tuple *other_tuple;
    enum flow_offload_tuple_dir dir;
    let mut xmit: nf_flow_xmit = {};
    struct in6_addr *ip6_daddr;
    struct flow_offload *flow;
    struct neighbour *neigh;
    struct rt6_info *rt;
    if (unlikely(tuplehash.tuple.xmit_type == FLOW_OFFLOAD_XMIT_XFRM)) {
    rt = dst_rt6_info(tuplehash.tuple.dst_cache);
    memset(skb.cb, 0, sizeof(struct inet6_skb_parm));
    IP6CB(skb).iif = skb.dev.ifindex;
    IP6CB(skb).flags = IP6SKB_FORWARDED;
    return nf_flow_xmit_xfrm(skb, state, &rt.dst);
    }
    dir = tuplehash.tuple.dir;
    flow = container_of(tuplehash, struct flow_offload, tuplehash[dir]);
    other_tuple = &flow.tuplehash[!dir].tuple;
    ip6_daddr = &other_tuple.src_v6;
    if (nf_flow_tunnel_v6_push(state.net, skb, other_tuple,
    tuplehash.tuple.dst_cache,
    &ip6_daddr) < 0)
    return NF_DROP;
    switch (tuplehash.tuple.xmit_type) {
    case FLOW_OFFLOAD_XMIT_NEIGH:
    rt = dst_rt6_info(tuplehash.tuple.dst_cache);
    xmit.outdev = dev_get_by_index_rcu(state.net, tuplehash.tuple.ifidx);
    if (!xmit.outdev) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    neigh = ip_neigh_gw6(rt.dst.dev, rt6_nexthop(rt, ip6_daddr));
    if (IS_ERR(neigh)) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    xmit.dest = neigh.ha;
    skb_dst_drop(skb);
    skb_dst_set_noref(skb, &rt.dst);
    break;
    case FLOW_OFFLOAD_XMIT_DIRECT:
    xmit.outdev = dev_get_by_index_rcu(state.net, tuplehash.tuple.out.ifidx);
    if (!xmit.outdev) {
    flow_offload_teardown(flow);
    return NF_DROP;
    }
    xmit.dest = tuplehash.tuple.out.h_dest;
    xmit.source = tuplehash.tuple.out.h_source;
    break;
    default:
    WARN_ON_ONCE(1);
    return NF_DROP;
    }
    xmit.tuple = other_tuple;
    xmit.needs_gso_segment = tuplehash.tuple.needs_gso_segment;
    return nf_flow_queue_xmit(state.net, skb, &xmit);
    }
    unsigned int
    nf_flow_offload_ipv6_hook(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct flow_offload_tuple_rhash *tuplehash;
    struct nf_flowtable *flow_table = priv;
    struct nf_flowtable_ctx ctx = {
    .in	= state.in,
    };
    int ret;
    if (!nf_flow_skb_encap_protocol(&ctx, skb))
    return NF_ACCEPT;
    if (unlikely(ctx.ether_type != htons(ETH_P_IPV6)))
    return NF_ACCEPT;
    tuplehash = nf_flow_offload_ipv6_lookup(&ctx, flow_table, skb);
    if (!tuplehash)
    return NF_ACCEPT;
    ret = nf_flow_offload_ipv6_forward(&ctx, flow_table, tuplehash, skb);
    if (ret < 0)
    return NF_DROP;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret ==) -> else {
    else if (ret == 0)
    return NF_ACCEPT;
    return nf_flow_queue_xmit6(skb, tuplehash, state);
    }
    EXPORT_SYMBOL_GPL(nf_flow_offload_ipv6_hook);
