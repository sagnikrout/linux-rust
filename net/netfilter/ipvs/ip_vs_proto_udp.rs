//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_proto_udp.c
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
// ip_vs_proto_udp.c:	UDP load balancing support for IPVS
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
// Julian Anastasov <ja@ssi.bg>
//
// Changes:     Hans Schillstrom <hans.schillstrom@ericsson.com>
// Network name space (netns) aware.
//

    static int
    udp_csum_check(int af, struct sk_buff *skb, struct ip_vs_protocol *pp,
    struct ip_vs_iphdr *iph);
    static int
    udp_conn_schedule(struct netns_ipvs *ipvs, int af, struct sk_buff *skb,
    struct ip_vs_proto_data *pd,
    int *verdict, struct ip_vs_conn **cpp,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_service *svc;
    struct udphdr _udph, *uh;
    __be16 _ports[2], *ports = core::ptr::null_mut();
    if (likely(!ip_vs_iph_icmp(iph))) {
// IPv6 fragments, only first fragment will hit this
    uh = skb_header_pointer(skb, iph.len, sizeof(_udph), &_udph);
    if (uh)
    ports = &uh.source;
    } else {
    ports = skb_header_pointer(
    skb, iph.len, sizeof(_ports), &_ports);
    }
    if (!ports) {
// verdict = NF_DROP;
    return 0;
    }
    if (likely(!ip_vs_iph_inverse(iph)))
    svc = ip_vs_service_find(ipvs, af, skb.mark, iph.protocol,
    &iph.daddr, ports[1]);
    else
    svc = ip_vs_service_find(ipvs, af, skb.mark, iph.protocol,
    &iph.saddr, ports[0]);
    if (svc) {
    int ignored;
    if (ip_vs_todrop(ipvs)) {
//
// It seems that we are very loaded.
// We have to drop this packet :(
//
// verdict = NF_DROP;
    return 0;
    }
//
// Let the virtual server select a real server for the
// incoming connection, and create a connection entry.
//
// cpp = ip_vs_schedule(svc, skb, pd, &ignored, iph);
    if (!*cpp && ignored <= 0) {
    if (!ignored)
// verdict = ip_vs_leave(svc, skb, pd, iph);
    else
// verdict = NF_DROP;
    return 0;
    }
    }
// NF_ACCEPT
    return 1;
    }
    static inline void
    udp_fast_csum_update(int af, struct udphdr *uhdr,
    const union nf_inet_addr *oldip,
    const union nf_inet_addr *newip,
    __be16 oldport, __be16 newport)
    {

    if (af == AF_INET6)
    uhdr.check =
    csum_fold(ip_vs_check_diff16(oldip.ip6, newip.ip6,
    ip_vs_check_diff2(oldport, newport,
    ~csum_unfold(uhdr.check))));
    else

    uhdr.check =
    csum_fold(ip_vs_check_diff4(oldip.ip, newip.ip,
    ip_vs_check_diff2(oldport, newport,
    ~csum_unfold(uhdr.check))));
    if (!uhdr.check)
    uhdr.check = CSUM_MANGLED_0;
    }
    static inline void
    udp_partial_csum_update(int af, struct udphdr *uhdr,
    const union nf_inet_addr *oldip,
    const union nf_inet_addr *newip,
    __be16 oldlen, __be16 newlen)
    {

    if (af == AF_INET6)
    uhdr.check =
    ~csum_fold(ip_vs_check_diff16(oldip.ip6, newip.ip6,
    ip_vs_check_diff2(oldlen, newlen,
    csum_unfold(uhdr.check))));
    else

    uhdr.check =
    ~csum_fold(ip_vs_check_diff4(oldip.ip, newip.ip,
    ip_vs_check_diff2(oldlen, newlen,
    csum_unfold(uhdr.check))));
    }
    INDIRECT_CALLABLE_SCOPE int
    udp_snat_handler(struct sk_buff *skb, struct ip_vs_protocol *pp,
    struct ip_vs_conn *cp, struct ip_vs_iphdr *iph)
    {
    struct udphdr *udph;
    let mut udphoff: c_uint = iph.len;
    let mut payload_csum: bool = false;
    int oldlen;

    if (cp.af == AF_INET6 && iph.fragoffs)
    return 1;

    oldlen = skb.len - udphoff;
// csum_check requires unshared skb
    if (skb_ensure_writable(skb, udphoff + sizeof(*udph)))
    return 0;
    if (unlikely(cp.app != core::ptr::null_mut())) {
    int ret;
// Some checks before mangling
    if (!udp_csum_check(cp.af, skb, pp, iph))
    return 0;
//
// Call application helper if needed
//
    if (!(ret = ip_vs_app_pkt_out(cp, skb, iph)))
    return 0;
// ret=2: csum update is needed after payload mangling
    if (ret == 1)
    oldlen = skb.len - udphoff;
    else
    payload_csum = true;
    }
    udph = (void *)skb.data + udphoff;
    udph.source = cp.vport;
//
// Adjust UDP checksums
//
    if (skb.ip_summed == CHECKSUM_PARTIAL) {
    udp_partial_csum_update(cp.af, udph, &cp.daddr, &cp.vaddr,
    htons(oldlen),
    htons(skb.len - udphoff));
    } else if (!payload_csum && (udph.check != 0)) {
// Only port and addr are changed, do fast csum update
    udp_fast_csum_update(cp.af, udph, &cp.daddr, &cp.vaddr,
    cp.dport, cp.vport);
    if (skb.ip_summed == CHECKSUM_COMPLETE)
    skb.ip_summed = cp.app ?
    CHECKSUM_UNNECESSARY : CHECKSUM_NONE;
    } else {
// full checksum calculation
    udph.check = 0;
    skb.csum = skb_checksum(skb, udphoff, skb.len - udphoff, 0);

    if (cp.af == AF_INET6)
    udph.check = csum_ipv6_magic(&cp.vaddr.in6,
    &cp.caddr.in6,
    skb.len - udphoff,
    cp.protocol, skb.csum);
    else

    udph.check = csum_tcpudp_magic(cp.vaddr.ip,
    cp.caddr.ip,
    skb.len - udphoff,
    cp.protocol,
    skb.csum);
    if (udph.check == 0)
    udph.check = CSUM_MANGLED_0;
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    IP_VS_DBG(11, "O-pkt: %s O-csum=%d (+%zd)\n",
    pp.name, udph.check,
    (char*)&(udph.check) - (char*)udph);
    }
    return 1;
    }
    static int
    udp_dnat_handler(struct sk_buff *skb, struct ip_vs_protocol *pp,
    struct ip_vs_conn *cp, struct ip_vs_iphdr *iph)
    {
    struct udphdr *udph;
    let mut udphoff: c_uint = iph.len;
    let mut payload_csum: bool = false;
    int oldlen;

    if (cp.af == AF_INET6 && iph.fragoffs)
    return 1;

    oldlen = skb.len - udphoff;
// csum_check requires unshared skb
    if (skb_ensure_writable(skb, udphoff + sizeof(*udph)))
    return 0;
    if (unlikely(cp.app != core::ptr::null_mut())) {
    int ret;
// Some checks before mangling
    if (!udp_csum_check(cp.af, skb, pp, iph))
    return 0;
//
// Attempt ip_vs_app call.
// It will fix ip_vs_conn
//
    if (!(ret = ip_vs_app_pkt_in(cp, skb, iph)))
    return 0;
// ret=2: csum update is needed after payload mangling
    if (ret == 1)
    oldlen = skb.len - udphoff;
    else
    payload_csum = true;
    }
    udph = (void *)skb.data + udphoff;
    udph.dest = cp.dport;
//
// Adjust UDP checksums
//
    if (skb.ip_summed == CHECKSUM_PARTIAL) {
    udp_partial_csum_update(cp.af, udph, &cp.vaddr, &cp.daddr,
    htons(oldlen),
    htons(skb.len - udphoff));
    } else if (!payload_csum && (udph.check != 0)) {
// Only port and addr are changed, do fast csum update
    udp_fast_csum_update(cp.af, udph, &cp.vaddr, &cp.daddr,
    cp.vport, cp.dport);
    if (skb.ip_summed == CHECKSUM_COMPLETE)
    skb.ip_summed = cp.app ?
    CHECKSUM_UNNECESSARY : CHECKSUM_NONE;
    } else {
// full checksum calculation
    udph.check = 0;
    skb.csum = skb_checksum(skb, udphoff, skb.len - udphoff, 0);

    if (cp.af == AF_INET6)
    udph.check = csum_ipv6_magic(&cp.caddr.in6,
    &cp.daddr.in6,
    skb.len - udphoff,
    cp.protocol, skb.csum);
    else

    udph.check = csum_tcpudp_magic(cp.caddr.ip,
    cp.daddr.ip,
    skb.len - udphoff,
    cp.protocol,
    skb.csum);
    if (udph.check == 0)
    udph.check = CSUM_MANGLED_0;
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    }
    return 1;
    }
    static int
    udp_csum_check(int af, struct sk_buff *skb, struct ip_vs_protocol *pp,
    struct ip_vs_iphdr *iph)
    {
    struct udphdr _udph, *uh;
    uh = skb_header_pointer(skb, iph.len, sizeof(_udph), &_udph);
    if (uh == core::ptr::null_mut())
    return 0;
    if (!uh.check)
    return 1;
    if (!ip_vs_checksum_common_check(skb, iph.len, IPPROTO_UDP, af)) {
    IP_VS_DBG_RL_PKT(0, af, pp, skb, iph.off,
    "Failed checksum for");
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn udp_app_hashkey(port: __be16) -> __u16 {
    static inline __u16 udp_app_hashkey(__be16 port)
    {
    return ((( u16)port >> UDP_APP_TAB_BITS) ^ ( u16)port)
    & UDP_APP_TAB_MASK;
    }
#[no_mangle]
unsafe extern "C" fn udp_register_app(ipvs: *mut netns_ipvs, inc: *mut ip_vs_app) -> c_int {
    static int udp_register_app(struct netns_ipvs *ipvs, struct ip_vs_app *inc)
    {
    struct ip_vs_app *i;
    __u16 hash;
    let mut port: __be16 = inc.port;
    let mut ret: c_int = 0;
    struct ip_vs_proto_data *pd = ip_vs_proto_data_get(ipvs, IPPROTO_UDP);
    hash = udp_app_hashkey(port);
    list_for_each_entry(i, &ipvs.udp_apps[hash], p_list) {
    if (i.port == port) {
    ret = -EEXIST;
    goto out;
    }
    }
    list_add_rcu(&inc.p_list, &ipvs.udp_apps[hash]);
    atomic_inc(&pd.appcnt);
    out:
    return ret;
    }
    static void
    udp_unregister_app(struct netns_ipvs *ipvs, struct ip_vs_app *inc)
    {
    struct ip_vs_proto_data *pd = ip_vs_proto_data_get(ipvs, IPPROTO_UDP);
    atomic_dec(&pd.appcnt);
    list_del_rcu(&inc.p_list);
    }
#[no_mangle]
unsafe extern "C" fn udp_app_conn_bind(cp: *mut ip_vs_conn) -> c_int {
    static int udp_app_conn_bind(struct ip_vs_conn *cp)
    {
    struct netns_ipvs *ipvs = cp.ipvs;
    int hash;
    struct ip_vs_app *inc;
    let mut result: c_int = 0;
// Default binding: bind app only for NAT
    if (IP_VS_FWD_METHOD(cp) != IP_VS_CONN_F_MASQ)
    return 0;
// Lookup application incarnations and bind the right one
    hash = udp_app_hashkey(cp.vport);
    list_for_each_entry_rcu(inc, &ipvs.udp_apps[hash], p_list) {
    if (inc.port == cp.vport) {
    if (unlikely(!ip_vs_app_inc_get(inc)))
    break;
    IP_VS_DBG_BUF(9, "%s(): Binding conn %s:%u."
    "%s:%u to app %s on port %u\n",
    __func__,
    IP_VS_DBG_ADDR(cp.af, &cp.caddr),
    ntohs(cp.cport),
    IP_VS_DBG_ADDR(cp.af, &cp.vaddr),
    ntohs(cp.vport),
    inc.name, ntohs(inc.port));
    cp.app = inc;
    if (inc.init_conn)
    result = inc.init_conn(inc, cp);
    break;
    }
    }
    return result;
    }
    static const int udp_timeouts[IP_VS_UDP_S_LAST+1] = {
    [IP_VS_UDP_S_NORMAL]		=	5*60*HZ,
    [IP_VS_UDP_S_LAST]		=	2*HZ,
    };
    static const char *const udp_state_name_table[IP_VS_UDP_S_LAST+1] = {
    [IP_VS_UDP_S_NORMAL]		=	"UDP",
    [IP_VS_UDP_S_LAST]		=	"BUG!",
    };
#[no_mangle]
unsafe extern "C" fn udp_state_name(state: c_int) -> *const c_char {
    static const char * udp_state_name(int state)
    {
    if (state >= IP_VS_UDP_S_LAST)
    return "ERR!";
    return udp_state_name_table[state] ? udp_state_name_table[state] : "?";
    }
    static void
    udp_state_transition(struct ip_vs_conn *cp, int direction,
    const struct sk_buff *skb,
    struct ip_vs_proto_data *pd,
    unsigned int iph_len)
    {
    if (unlikely(!pd)) {
    pr_err("UDP no ns data\n");
    return;
    }
    cp.timeout = pd.timeout_table[IP_VS_UDP_S_NORMAL];
    if (direction == IP_VS_DIR_OUTPUT)
    ip_vs_control_assure_ct(cp);
    }
#[no_mangle]
unsafe extern "C" fn __udp_init(ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) -> c_int {
    static int __udp_init(struct netns_ipvs *ipvs, struct ip_vs_proto_data *pd)
    {
    ip_vs_init_hash_table(ipvs.udp_apps, UDP_APP_TAB_SIZE);
    pd.timeout_table = ip_vs_create_timeout_table((int *)udp_timeouts,
    sizeof(udp_timeouts));
    if (!pd.timeout_table)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __udp_exit(ipvs: *mut netns_ipvs, pd: *mut ip_vs_proto_data) {
    static void __udp_exit(struct netns_ipvs *ipvs, struct ip_vs_proto_data *pd)
    {
    kfree(pd.timeout_table);
    }
    struct ip_vs_protocol ip_vs_protocol_udp = {
    .name =			"UDP",
    .protocol =		IPPROTO_UDP,
    .num_states =		IP_VS_UDP_S_LAST,
    .dont_defrag =		0,
    .init =			core::ptr::null_mut(),
    .exit =			core::ptr::null_mut(),
    .init_netns =		__udp_init,
    .exit_netns =		__udp_exit,
    .conn_schedule =	udp_conn_schedule,
    .conn_in_get =		ip_vs_conn_in_get_proto,
    .conn_out_get =		ip_vs_conn_out_get_proto,
    .snat_handler =		udp_snat_handler,
    .dnat_handler =		udp_dnat_handler,
    .state_transition =	udp_state_transition,
    .state_name =		udp_state_name,
    .register_app =		udp_register_app,
    .unregister_app =	udp_unregister_app,
    .app_conn_bind =	udp_app_conn_bind,
    .debug_packet =		ip_vs_tcpudp_debug_packet,
    .timeout_change =	core::ptr::null_mut(),
    };
