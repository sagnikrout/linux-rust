//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_proto.c
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
// ip_vs_proto.c: transport protocol load balancing support for IPVS
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
// Julian Anastasov <ja@ssi.bg>
//
// Changes:
//

//
// IPVS protocols can only be registered/unregistered when the ipvs
// module is loaded/unloaded, so no lock is needed in accessing the
// ipvs protocol table.
//

    static struct ip_vs_protocol *ip_vs_proto_table[IP_VS_PROTO_TAB_SIZE];
// States for conn templates: NONE or words separated with ",", max 15 chars
    static const char *ip_vs_ctpl_state_name_table[IP_VS_CTPL_S_LAST] = {
    [IP_VS_CTPL_S_NONE]			= "NONE",
    [IP_VS_CTPL_S_ASSURED]			= "ASSURED",
    };
//
// register an ipvs protocol
//
#[no_mangle]
unsafe extern "C" fn register_ip_vs_protocol(pp: *mut ip_vs_protocol) -> int __used __init {
    static int __used __init register_ip_vs_protocol(struct ip_vs_protocol *pp)
    {
    let mut hash: c_uint = IP_VS_PROTO_HASH(pp.protocol);
    pp.next = ip_vs_proto_table[hash];
    ip_vs_proto_table[hash] = pp;
    if (pp.init != core::ptr::null_mut())
    pp.init(pp);
    return 0;
    }
//
// register an ipvs protocols netns related data
//
    static int
    register_ip_vs_proto_netns(struct netns_ipvs *ipvs, struct ip_vs_protocol *pp)
    {
    let mut hash: c_uint = IP_VS_PROTO_HASH(pp.protocol);
    struct ip_vs_proto_data *pd =
    kzalloc_obj(struct ip_vs_proto_data);
    if (!pd)
    return -ENOMEM;
    pd.pp = pp;	/* For speed issues */
    pd.next = ipvs.proto_data_table[hash];
    ipvs.proto_data_table[hash] = pd;
    atomic_set(&pd.appcnt, 0);	/* Init app counter */
    if (pp.init_netns != core::ptr::null_mut()) {
    let mut ret: c_int = pp.init_netns(ipvs, pd);
    if (ret) {
// unlink an free proto data
    ipvs.proto_data_table[hash] = pd.next;
    kfree(pd);
    return ret;
    }
    }
    return 0;
    }
//
// unregister an ipvs protocol
//
#[no_mangle]
unsafe extern "C" fn unregister_ip_vs_protocol(pp: *mut ip_vs_protocol) -> c_int {
    static int unregister_ip_vs_protocol(struct ip_vs_protocol *pp)
    {
    struct ip_vs_protocol **pp_p;
    let mut hash: c_uint = IP_VS_PROTO_HASH(pp.protocol);
    pp_p = &ip_vs_proto_table[hash];
    for (; *pp_p; pp_p = &(*pp_p).next) {
    if (*pp_p == pp) {
// pp_p = pp->next;
    if (pp.exit != core::ptr::null_mut())
    pp.exit(pp);
    return 0;
    }
    }
    return -ESRCH;
    }
//
// unregister an ipvs protocols netns data
//
    static int
    unregister_ip_vs_proto_netns(struct netns_ipvs *ipvs, struct ip_vs_proto_data *pd)
    {
    struct ip_vs_proto_data **pd_p;
    let mut hash: c_uint = IP_VS_PROTO_HASH(pd.pp.protocol);
    pd_p = &ipvs.proto_data_table[hash];
    for (; *pd_p; pd_p = &(*pd_p).next) {
    if (*pd_p == pd) {
// pd_p = pd->next;
    if (pd.pp.exit_netns != core::ptr::null_mut())
    pd.pp.exit_netns(ipvs, pd);
    kfree(pd);
    return 0;
    }
    }
    return -ESRCH;
    }
//
// get ip_vs_protocol object by its proto.
//
#[no_mangle]
pub unsafe extern "C" fn ip_vs_proto_get(proto: c_ushort) -> *mut ip_vs_protocol {
    struct ip_vs_protocol * ip_vs_proto_get(unsigned short proto)
    {
    struct ip_vs_protocol *pp;
    let mut hash: c_uint = IP_VS_PROTO_HASH(proto);
    for (pp = ip_vs_proto_table[hash]; pp; pp = pp.next) {
    if (pp.protocol == proto)
    return pp;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(ip_vs_proto_get);
//
// get ip_vs_protocol object data by netns and proto
//
    struct ip_vs_proto_data *
    ip_vs_proto_data_get(struct netns_ipvs *ipvs, unsigned short proto)
    {
    struct ip_vs_proto_data *pd;
    let mut hash: c_uint = IP_VS_PROTO_HASH(proto);
    for (pd = ipvs.proto_data_table[hash]; pd; pd = pd.next) {
    if (pd.pp.protocol == proto)
    return pd;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(ip_vs_proto_data_get);
//
// Propagate event for state change to all protocols
//
#[no_mangle]
pub unsafe extern "C" fn ip_vs_protocol_timeout_change(ipvs: *mut netns_ipvs, flags: c_int) {
    void ip_vs_protocol_timeout_change(struct netns_ipvs *ipvs, int flags)
    {
    struct ip_vs_proto_data *pd;
    int i;
    for (i = 0; i < IP_VS_PROTO_TAB_SIZE; i++) {
    for (pd = ipvs.proto_data_table[i]; pd; pd = pd.next) {
    if (pd.pp.timeout_change)
    pd.pp.timeout_change(pd, flags);
    }
    }
    }
    int *
    ip_vs_create_timeout_table(int *table, int size)
    {
    return kmemdup(table, size, GFP_KERNEL);
    }
    const char *ip_vs_state_name(const struct ip_vs_conn *cp)
    {
    let mut state: c_uint = cp.state;
    struct ip_vs_protocol *pp;
    if (cp.flags & IP_VS_CONN_F_TEMPLATE) {
    if (state >= IP_VS_CTPL_S_LAST)
    return "ERR!";
    return ip_vs_ctpl_state_name_table[state] ? : "?";
    }
    pp = ip_vs_proto_get(cp.protocol);
    if (pp == core::ptr::null_mut() || pp.state_name == core::ptr::null_mut())
    return (cp.protocol == IPPROTO_IP) ? "NONE" : "ERR!";
    return pp.state_name(state);
    }
    static void
    ip_vs_tcpudp_debug_packet_v4(struct ip_vs_protocol *pp,
    const struct sk_buff *skb,
    int offset,
    const char *msg)
    {
    char buf[128];
    struct iphdr _iph, *ih;
    ih = skb_header_pointer(skb, offset, sizeof(_iph), &_iph);
    if (ih == core::ptr::null_mut())
    sprintf(buf, "TRUNCATED");
#[no_mangle]
pub unsafe extern "C" fn if(htons(IP_OFFSET): ih->frag_off &) -> else {
    else if (ih.frag_off & htons(IP_OFFSET))
    sprintf(buf, "%pI4.%pI4 frag", &ih.saddr, &ih.daddr);
    else {
    __be16 _ports[2], *pptr;
    pptr = skb_header_pointer(skb, offset + ih.ihl*4,
    sizeof(_ports), _ports);
    if (pptr == core::ptr::null_mut())
    sprintf(buf, "TRUNCATED %pI4.%pI4",
    &ih.saddr, &ih.daddr);
    else
    sprintf(buf, "%pI4:%u.%pI4:%u",
    &ih.saddr, ntohs(pptr[0]),
    &ih.daddr, ntohs(pptr[1]));
    }
    pr_debug("%s: %s %s\n", msg, pp.name, buf);
    }

    static void
    ip_vs_tcpudp_debug_packet_v6(struct ip_vs_protocol *pp,
    const struct sk_buff *skb,
    int offset,
    const char *msg)
    {
    char buf[192];
    struct ipv6hdr _iph, *ih;
    ih = skb_header_pointer(skb, offset, sizeof(_iph), &_iph);
    if (ih == core::ptr::null_mut())
    sprintf(buf, "TRUNCATED");
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_FRAGMENT: ih->nexthdr ==) -> else {
    else if (ih.nexthdr == IPPROTO_FRAGMENT)
    sprintf(buf, "%pI6c.%pI6c frag", &ih.saddr, &ih.daddr);
    else {
    __be16 _ports[2], *pptr;
    pptr = skb_header_pointer(skb, offset + sizeof(struct ipv6hdr),
    sizeof(_ports), _ports);
    if (pptr == core::ptr::null_mut())
    sprintf(buf, "TRUNCATED %pI6c.%pI6c",
    &ih.saddr, &ih.daddr);
    else
    sprintf(buf, "%pI6c:%u.%pI6c:%u",
    &ih.saddr, ntohs(pptr[0]),
    &ih.daddr, ntohs(pptr[1]));
    }
    pr_debug("%s: %s %s\n", msg, pp.name, buf);
    }

    void
    ip_vs_tcpudp_debug_packet(int af, struct ip_vs_protocol *pp,
    const struct sk_buff *skb,
    int offset,
    const char *msg)
    {

    if (af == AF_INET6)
    ip_vs_tcpudp_debug_packet_v6(pp, skb, offset, msg);
    else

    ip_vs_tcpudp_debug_packet_v4(pp, skb, offset, msg);
    }
//
// per network name-space init
//
#[no_mangle]
pub unsafe extern "C" fn ip_vs_protocol_net_init(ipvs: *mut netns_ipvs) -> int __net_init {
    int __net_init ip_vs_protocol_net_init(struct netns_ipvs *ipvs)
    {
    int i, ret;
    static struct ip_vs_protocol *protos[] = {

    &ip_vs_protocol_tcp,

    &ip_vs_protocol_udp,

    &ip_vs_protocol_sctp,

    &ip_vs_protocol_ah,

    &ip_vs_protocol_esp,

    };
    for (i = 0; i < ARRAY_SIZE(protos); i++) {
    ret = register_ip_vs_proto_netns(ipvs, protos[i]);
    if (ret < 0)
    goto cleanup;
    }
    return 0;
    cleanup:
    ip_vs_protocol_net_cleanup(ipvs);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_protocol_net_cleanup(ipvs: *mut netns_ipvs) -> void __net_exit {
    void __net_exit ip_vs_protocol_net_cleanup(struct netns_ipvs *ipvs)
    {
    struct ip_vs_proto_data *pd;
    int i;
// unregister all the ipvs proto data for this netns
    for (i = 0; i < IP_VS_PROTO_TAB_SIZE; i++) {
    while ((pd = ipvs.proto_data_table[i]) != core::ptr::null_mut())
    unregister_ip_vs_proto_netns(ipvs, pd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_protocol_init() -> int __init {
    int __init ip_vs_protocol_init(void)
    {
    char protocols[64] = { 0 };

    do {					\
    register_ip_vs_protocol(p);	\
    strcat(protocols, ", ");	\
    strcat(protocols, (p).name);	\
    } while (0)

    REGISTER_PROTOCOL(&ip_vs_protocol_tcp);

    REGISTER_PROTOCOL(&ip_vs_protocol_udp);

    REGISTER_PROTOCOL(&ip_vs_protocol_sctp);

    REGISTER_PROTOCOL(&ip_vs_protocol_ah);

    REGISTER_PROTOCOL(&ip_vs_protocol_esp);

    pr_info("Registered protocols (%s)\n", &protocols[2]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_protocol_cleanup() {
    void ip_vs_protocol_cleanup(void)
    {
    struct ip_vs_protocol *pp;
    int i;
// unregister all the ipvs protocols
    for (i = 0; i < IP_VS_PROTO_TAB_SIZE; i++) {
    while ((pp = ip_vs_proto_table[i]) != core::ptr::null_mut())
    unregister_ip_vs_protocol(pp);
    }
    }
