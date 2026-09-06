//! Automatically rewritten from C to Rust
//! Source: net/xfrm/xfrm_input.c
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
// xfrm_input.c
//
// Changes:
// YOSHIFUJI Hideaki @USAGI
// Split up af-specific portion
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_trans_tasklet {
    pub work: work_struct,
    pub queue_lock: spinlock_t,
    pub queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_trans_cb {
    union {
    pub h4: inet_skb_parm,

    pub h6: inet6_skb_parm,

    pub header: },
    pub skb): *mut *mut *mut *mut int (finish)(struct net net, struct sock sk, struct sk_buff,
    pub net: *mut net,
}

    static DEFINE_SPINLOCK(xfrm_input_afinfo_lock);
    static struct xfrm_input_afinfo const __rcu *xfrm_input_afinfo[2][AF_INET6 + 1];
    static struct gro_cells gro_cells;
    static struct net_device *xfrm_napi_dev;
    static DEFINE_PER_CPU(struct xfrm_trans_tasklet, xfrm_trans_tasklet);
#[no_mangle]
pub unsafe extern "C" fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int {
    int xfrm_input_register_afinfo(const struct xfrm_input_afinfo *afinfo)
    {
    let mut err: c_int = 0;
    if (WARN_ON(afinfo.family > AF_INET6))
    return -EAFNOSUPPORT;
    spin_lock_bh(&xfrm_input_afinfo_lock);
    if (unlikely(xfrm_input_afinfo[afinfo.is_ipip][afinfo.family]))
    err = -EEXIST;
    else
    rcu_assign_pointer(xfrm_input_afinfo[afinfo.is_ipip][afinfo.family], afinfo);
    spin_unlock_bh(&xfrm_input_afinfo_lock);
    return err;
    }
    EXPORT_SYMBOL(xfrm_input_register_afinfo);
#[no_mangle]
pub unsafe extern "C" fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int {
    int xfrm_input_unregister_afinfo(const struct xfrm_input_afinfo *afinfo)
    {
    let mut err: c_int = 0;
    spin_lock_bh(&xfrm_input_afinfo_lock);
    if (likely(xfrm_input_afinfo[afinfo.is_ipip][afinfo.family])) {
    const struct xfrm_input_afinfo *cur;
    cur = rcu_access_pointer(xfrm_input_afinfo[afinfo.is_ipip][afinfo.family]);
    if (unlikely(cur != afinfo))
    err = -EINVAL;
    else
    RCU_INIT_POINTER(xfrm_input_afinfo[afinfo.is_ipip][afinfo.family], core::ptr::null_mut());
    }
    spin_unlock_bh(&xfrm_input_afinfo_lock);
    synchronize_rcu();
    return err;
    }
    EXPORT_SYMBOL(xfrm_input_unregister_afinfo);
    static const struct xfrm_input_afinfo *xfrm_input_get_afinfo(u8 family, bool is_ipip)
    {
    const struct xfrm_input_afinfo *afinfo;
    if (WARN_ON_ONCE(family > AF_INET6))
    return core::ptr::null_mut();
    rcu_read_lock();
    afinfo = rcu_dereference(xfrm_input_afinfo[is_ipip][family]);
    if (unlikely(!afinfo))
    rcu_read_unlock();
    return afinfo;
    }
    static int xfrm_rcv_cb(struct sk_buff *skb, unsigned int family, u8 protocol,
    int err)
    {
    let mut is_ipip: bool = (protocol == IPPROTO_IPIP || protocol == IPPROTO_IPV6);
    const struct xfrm_input_afinfo *afinfo;
    int ret;
    afinfo = xfrm_input_get_afinfo(family, is_ipip);
    if (!afinfo)
    return -EAFNOSUPPORT;
    ret = afinfo.callback(skb, protocol, err);
    rcu_read_unlock();
    return ret;
    }
    struct sec_path *secpath_set(struct sk_buff *skb)
    {
    struct sec_path *sp, *tmp = skb_ext_find(skb, SKB_EXT_SEC_PATH);
    sp = skb_ext_add(skb, SKB_EXT_SEC_PATH);
    if (!sp)
    return core::ptr::null_mut();
    if (tmp) /* reused existing one (was COW'd if needed) */
    return sp;
// allocated new secpath
    memset(sp.ovec, 0, sizeof(sp.ovec));
    sp.olen = 0;
    sp.len = 0;
    sp.verified_cnt = 0;
    return sp;
    }
    EXPORT_SYMBOL(secpath_set);
// Fetch spi and seq from ipsec header
#[no_mangle]
pub unsafe extern "C" fn xfrm_parse_spi(skb: *mut sk_buff, nexthdr: u8, spi: *mut __be32, seq: *mut __be32) -> c_int {
    int xfrm_parse_spi(struct sk_buff *skb, u8 nexthdr, __be32 *spi, __be32 *seq)
    {
    int offset, offset_seq;
    int hlen;
    switch (nexthdr) {
    case IPPROTO_AH:
    hlen = sizeof(struct ip_auth_hdr);
    offset = offsetof(struct ip_auth_hdr, spi);
    offset_seq = offsetof(struct ip_auth_hdr, seq_no);
    break;
    case IPPROTO_ESP:
    hlen = sizeof(struct ip_esp_hdr);
    offset = offsetof(struct ip_esp_hdr, spi);
    offset_seq = offsetof(struct ip_esp_hdr, seq_no);
    break;
    case IPPROTO_COMP:
    if (!pskb_may_pull(skb, sizeof(struct ip_comp_hdr)))
    return -EINVAL;
// spi = htonl(ntohs(*(__be16 *)(skb_transport_header(skb) + 2)));
// seq = 0;
    return 0;
    default:
    return 1;
    }
    if (!pskb_may_pull(skb, hlen))
    return -EINVAL;
// spi = *(__be32 *)(skb_transport_header(skb) + offset);
// seq = *(__be32 *)(skb_transport_header(skb) + offset_seq);
    return 0;
    }
    EXPORT_SYMBOL(xfrm_parse_spi);
#[no_mangle]
unsafe extern "C" fn xfrm4_remove_beet_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm4_remove_beet_encap(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct iphdr *iph;
    let mut optlen: c_int = 0;
    let mut err: c_int = -EINVAL;
    skb.protocol = htons(ETH_P_IP);
    if (unlikely(XFRM_MODE_SKB_CB(skb).protocol == IPPROTO_BEETPH)) {
    struct ip_beet_phdr *ph;
    int phlen;
    if (!pskb_may_pull(skb, sizeof(*ph)))
    goto out;
    ph = (struct ip_beet_phdr *)skb.data;
    phlen = sizeof(*ph) + ph.padlen;
    optlen = ph.hdrlen * 8 + (IPV4_BEET_PHMAXLEN - phlen);
    if (optlen < 0 || optlen & 3 || optlen > 250)
    goto out;
    XFRM_MODE_SKB_CB(skb).protocol = ph.nexthdr;
    if (!pskb_may_pull(skb, phlen))
    goto out;
    __skb_pull(skb, phlen);
    }
    skb_push(skb, sizeof(*iph));
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);
    xfrm4_beet_make_header(skb);
    iph = ip_hdr(skb);
    iph.ihl += optlen / 4;
    iph.tot_len = htons(skb.len);
    iph.daddr = x.sel.daddr.a4;
    iph.saddr = x.sel.saddr.a4;
    iph.check = 0;
    iph.check = ip_fast_csum(skb_network_header(skb), iph.ihl);
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ipip_ecn_decapsulate(skb: *mut sk_buff) {
    static void ipip_ecn_decapsulate(struct sk_buff *skb)
    {
    struct iphdr *inner_iph = ipip_hdr(skb);
    if (INET_ECN_is_ce(XFRM_MODE_SKB_CB(skb).tos))
    IP_ECN_set_ce(inner_iph);
    }
#[no_mangle]
unsafe extern "C" fn xfrm4_remove_tunnel_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm4_remove_tunnel_encap(struct xfrm_state *x, struct sk_buff *skb)
    {
    let mut err: c_int = -EINVAL;
    skb.protocol = htons(ETH_P_IP);
    if (!pskb_may_pull(skb, sizeof(struct iphdr)))
    goto out;
    err = skb_unclone(skb, GFP_ATOMIC);
    if (err)
    goto out;
    if (x.props.flags & XFRM_STATE_DECAP_DSCP)
    ipv4_copy_dscp(XFRM_MODE_SKB_CB(skb).tos, ipip_hdr(skb));
    if (!(x.props.flags & XFRM_STATE_NOECN))
    ipip_ecn_decapsulate(skb);
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);
    if (skb.mac_len)
    eth_hdr(skb).h_proto = skb.protocol;
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ipip6_ecn_decapsulate(skb: *mut sk_buff) {
    static void ipip6_ecn_decapsulate(struct sk_buff *skb)
    {
    struct ipv6hdr *inner_iph = ipipv6_hdr(skb);
    if (INET_ECN_is_ce(XFRM_MODE_SKB_CB(skb).tos))
    IP6_ECN_set_ce(skb, inner_iph);
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_remove_tunnel_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm6_remove_tunnel_encap(struct xfrm_state *x, struct sk_buff *skb)
    {
    let mut err: c_int = -EINVAL;
    skb.protocol = htons(ETH_P_IPV6);
    if (!pskb_may_pull(skb, sizeof(struct ipv6hdr)))
    goto out;
    err = skb_unclone(skb, GFP_ATOMIC);
    if (err)
    goto out;
    if (x.props.flags & XFRM_STATE_DECAP_DSCP)
    ipv6_copy_dscp(XFRM_MODE_SKB_CB(skb).tos, ipipv6_hdr(skb));
    if (!(x.props.flags & XFRM_STATE_NOECN))
    ipip6_ecn_decapsulate(skb);
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);
    if (skb.mac_len)
    eth_hdr(skb).h_proto = skb.protocol;
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_remove_beet_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm6_remove_beet_encap(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct ipv6hdr *ip6h;
    let mut size: c_int = sizeof(struct ipv6hdr);
    int err;
    skb.protocol = htons(ETH_P_IPV6);
    err = skb_cow_head(skb, size + skb.mac_len);
    if (err)
    goto out;
    __skb_push(skb, size);
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);
    xfrm6_beet_make_header(skb);
    ip6h = ipv6_hdr(skb);
    ip6h.payload_len = htons(skb.len - size);
    ip6h.daddr = x.sel.daddr.in6;
    ip6h.saddr = x.sel.saddr.in6;
    err = 0;
    out:
    return err;
    }
// Remove encapsulation header.
//
// The IP header will be moved over the top of the encapsulation
// header.
//
// On entry, the transport header shall point to where the IP header
// should be and the network header shall be set to where the IP
// header currently is.  skb->data shall point to the start of the
// payload.
//
    static int
    xfrm_inner_mode_encap_remove(struct xfrm_state *x,
    struct sk_buff *skb)
    {
    switch (x.props.mode) {
    case XFRM_MODE_BEET:
    switch (x.sel.family) {
    case AF_INET:
    return xfrm4_remove_beet_encap(x, skb);
    case AF_INET6:
    return xfrm6_remove_beet_encap(x, skb);
    }
    break;
    case XFRM_MODE_TUNNEL:
    switch (XFRM_MODE_SKB_CB(skb).protocol) {
    case IPPROTO_IPIP:
    return xfrm4_remove_tunnel_encap(x, skb);
    case IPPROTO_IPV6:
    return xfrm6_remove_tunnel_encap(x, skb);
    break;
    }
    return -EINVAL;
    }
    WARN_ON_ONCE(1);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn xfrm_prepare_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm_prepare_input(struct xfrm_state *x, struct sk_buff *skb)
    {
    switch (x.props.family) {
    case AF_INET:
    xfrm4_extract_header(skb);
    break;
    case AF_INET6:
    xfrm6_extract_header(skb);
    break;
    default:
    WARN_ON_ONCE(1);
    return -EAFNOSUPPORT;
    }
    return xfrm_inner_mode_encap_remove(x, skb);
    }
// Remove encapsulation header.
//
// The IP header will be moved over the top of the encapsulation header.
//
// On entry, skb_transport_header() shall point to where the IP header
// should be and skb_network_header() shall be set to where the IP header
// currently is.  skb->data shall point to the start of the payload.
//
#[no_mangle]
unsafe extern "C" fn xfrm4_transport_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm4_transport_input(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct xfrm_offload *xo = xfrm_offload(skb);
    let mut ihl: c_int = skb.data - skb_transport_header(skb);
    if (skb.transport_header != skb.network_header) {
    memmove(skb_transport_header(skb),
    skb_network_header(skb), ihl);
    if (xo)
    xo.orig_mac_len =
    skb_mac_header_was_set(skb) ? skb_mac_header_len(skb) : 0;
    skb.network_header = skb.transport_header;
    }
    ip_hdr(skb).tot_len = htons(skb.len + ihl);
    skb_reset_transport_header(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfrm6_transport_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int xfrm6_transport_input(struct xfrm_state *x, struct sk_buff *skb)
    {

    struct xfrm_offload *xo = xfrm_offload(skb);
    let mut ihl: c_int = skb.data - skb_transport_header(skb);
    if (skb.transport_header != skb.network_header) {
    memmove(skb_transport_header(skb),
    skb_network_header(skb), ihl);
    if (xo)
    xo.orig_mac_len =
    skb_mac_header_was_set(skb) ? skb_mac_header_len(skb) : 0;
    skb.network_header = skb.transport_header;
    }
    ipv6_hdr(skb).payload_len = htons(skb.len + ihl -
    sizeof(struct ipv6hdr));
    skb_reset_transport_header(skb);
    return 0;

    WARN_ON_ONCE(1);
    return -EAFNOSUPPORT;

    }
    static int xfrm_inner_mode_input(struct xfrm_state *x,
    struct sk_buff *skb)
    {
    switch (x.props.mode) {
    case XFRM_MODE_BEET:
    case XFRM_MODE_TUNNEL:
    return xfrm_prepare_input(x, skb);
    case XFRM_MODE_TRANSPORT:
    if (x.props.family == AF_INET)
    return xfrm4_transport_input(x, skb);
    if (x.props.family == AF_INET6)
    return xfrm6_transport_input(x, skb);
    break;
    case XFRM_MODE_ROUTEOPTIMIZATION:
    WARN_ON_ONCE(1);
    break;
    default:
    if (x.mode_cbs && x.mode_cbs.input)
    return x.mode_cbs.input(x, skb);
    WARN_ON_ONCE(1);
    break;
    }
    return -EOPNOTSUPP;
    }
// NOTE: encap_type - In addition to the normal (non-negative) values for
// encap_type, a negative value of -1 or -2 can be used to resume/restart this
// function after a previous invocation early terminated for async operation.
//
#[no_mangle]
pub unsafe extern "C" fn xfrm_input(skb: *mut sk_buff, nexthdr: c_int, spi: __be32, encap_type: c_int) -> c_int {
    int xfrm_input(struct sk_buff *skb, int nexthdr, __be32 spi, int encap_type)
    {
    const struct xfrm_state_afinfo *afinfo;
    struct net *net = dev_net(skb.dev);
    struct net_device *dev = skb.dev;
    int err;
    __be32 seq;
    __be32 seq_hi;
    struct xfrm_state *x = core::ptr::null_mut();
    xfrm_address_t *daddr;
    let mut mark: u32 = skb.mark;
    let mut family: c_uint = AF_UNSPEC;
    let mut decaps: c_int = 0;
    let mut async: c_int = 0;
    let mut xfrm_gro: bool = false;
    let mut crypto_done: bool = false;
    struct xfrm_offload *xo = xfrm_offload(skb);
    struct sec_path *sp;
    if (encap_type < 0 || (xo && (xo.flags & XFRM_GRO || encap_type == 0 ||
    encap_type == UDP_ENCAP_ESPINUDP))) {
    x = xfrm_input_state(skb);
    if (unlikely(x.km.state != XFRM_STATE_VALID)) {
    if (x.km.state == XFRM_STATE_ACQ)
    XFRM_INC_STATS(net, LINUX_MIB_XFRMACQUIREERROR);
    else
    XFRM_INC_STATS(net,
    LINUX_MIB_XFRMINSTATEINVALID);
    if (encap_type == -1)
    dev_put(dev);
    goto drop;
    }
    family = x.props.family;
// An encap_type of -2 indicates reconstructed inner packet
    if (encap_type == -2)
    goto resume_decapped;
// An encap_type of -1 indicates async resumption.
    if (encap_type == -1) {
    async = 1;
    seq = XFRM_SKB_CB(skb).seq.input.low;
    spin_lock(&x.lock);
    goto resume;
    }
// GRO call
    seq = XFRM_SPI_SKB_CB(skb).seq;
    if (xo && (xo.flags & CRYPTO_DONE)) {
    crypto_done = true;
    family = XFRM_SPI_SKB_CB(skb).family;
    if (!(xo.status & CRYPTO_SUCCESS)) {
    if (xo.status &
    (CRYPTO_TRANSPORT_AH_AUTH_FAILED |
    CRYPTO_TRANSPORT_ESP_AUTH_FAILED |
    CRYPTO_TUNNEL_AH_AUTH_FAILED |
    CRYPTO_TUNNEL_ESP_AUTH_FAILED)) {
    xfrm_audit_state_icvfail(x, skb,
    x.type.proto);
    x.stats.integrity_failed++;
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEPROTOERROR);
    goto drop;
    }
    if (xo.status & CRYPTO_INVALID_PROTOCOL) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEPROTOERROR);
    goto drop;
    }
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINBUFFERERROR);
    goto drop;
    }
    if (xfrm_parse_spi(skb, nexthdr, &spi, &seq)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINHDRERROR);
    goto drop;
    }
    nexthdr = x.type_offload.input_tail(x, skb);
    }
    goto process;
    }
    family = XFRM_SPI_SKB_CB(skb).family;
// if tunnel is present override skb->mark value with tunnel i_key
    switch (family) {
    case AF_INET:
    if (XFRM_TUNNEL_SKB_CB(skb).tunnel.ip4)
    mark = be32_to_cpu(XFRM_TUNNEL_SKB_CB(skb).tunnel.ip4.parms.i_key);
    break;
    case AF_INET6:
    if (XFRM_TUNNEL_SKB_CB(skb).tunnel.ip6)
    mark = be32_to_cpu(XFRM_TUNNEL_SKB_CB(skb).tunnel.ip6.parms.i_key);
    break;
    }
    sp = secpath_set(skb);
    if (!sp) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINERROR);
    goto drop;
    }
    seq = 0;
    if (!spi && xfrm_parse_spi(skb, nexthdr, &spi, &seq)) {
    secpath_reset(skb);
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINHDRERROR);
    goto drop;
    }
    daddr = (xfrm_address_t *)(skb_network_header(skb) +
    XFRM_SPI_SKB_CB(skb).daddroff);
    do {
    sp = skb_sec_path(skb);
    if (sp.len == XFRM_MAX_DEPTH) {
    secpath_reset(skb);
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINBUFFERERROR);
    goto drop;
    }
    x = xfrm_input_state_lookup(net, mark, daddr, spi, nexthdr, family);
    if (x == core::ptr::null_mut()) {
    secpath_reset(skb);
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINNOSTATES);
    xfrm_audit_state_notfound(skb, family, spi, seq);
    goto drop;
    }
    if (unlikely(x.dir && x.dir != XFRM_SA_DIR_IN)) {
    secpath_reset(skb);
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEDIRERROR);
    xfrm_audit_state_notfound(skb, family, spi, seq);
    xfrm_state_put(x);
    x = core::ptr::null_mut();
    goto drop;
    }
    skb.mark = xfrm_smark_get(skb.mark, x);
    sp.xvec[sp.len++] = x;
    skb_dst_force(skb);
    if (!skb_dst(skb)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINERROR);
    goto drop;
    }
    process:
    seq_hi = htonl(xfrm_replay_seqhi(x, seq));
    XFRM_SKB_CB(skb).seq.input.low = seq;
    XFRM_SKB_CB(skb).seq.input.hi = seq_hi;
    spin_lock(&x.lock);
    if (unlikely(x.km.state != XFRM_STATE_VALID)) {
    if (x.km.state == XFRM_STATE_ACQ)
    XFRM_INC_STATS(net, LINUX_MIB_XFRMACQUIREERROR);
    else
    XFRM_INC_STATS(net,
    LINUX_MIB_XFRMINSTATEINVALID);
    goto drop_unlock;
    }
    if ((x.encap ? x.encap.encap_type : 0) != encap_type) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEMISMATCH);
    goto drop_unlock;
    }
    if (xfrm_replay_check(x, skb, seq)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATESEQERROR);
    goto drop_unlock;
    }
    if (xfrm_state_check_expire(x)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEEXPIRED);
    goto drop_unlock;
    }
    if (xfrm_tunnel_check(skb, x, family)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEMODEERROR);
    goto drop_unlock;
    }
    if (!crypto_done) {
    spin_unlock(&x.lock);
    dev_hold(dev);
    nexthdr = x.type.input(x, skb);
    if (nexthdr == -EINPROGRESS) {
    if (async)
    dev_put(dev);
    return 0;
    }
    dev_put(dev);
    spin_lock(&x.lock);
    }
    resume:
    if (nexthdr < 0) {
    if (nexthdr == -EBADMSG) {
    xfrm_audit_state_icvfail(x, skb,
    x.type.proto);
    x.stats.integrity_failed++;
    }
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEPROTOERROR);
    goto drop_unlock;
    }
// only the first xfrm gets the encap type
    encap_type = 0;
    if (!crypto_done && xfrm_replay_recheck(x, skb, seq)) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATESEQERROR);
    goto drop_unlock;
    }
    xfrm_replay_advance(x, seq);
    x.curlft.bytes += skb.len;
    x.curlft.packets++;
    x.lastused = ktime_get_real_seconds();
    spin_unlock(&x.lock);
    XFRM_MODE_SKB_CB(skb).protocol = nexthdr;
    err = xfrm_inner_mode_input(x, skb);
    if (err == -EINPROGRESS) {
    if (async)
    dev_put(dev);
    return 0;
    } else if (err) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINSTATEMODEERROR);
    goto drop;
    }
    resume_decapped:
    if (x.outer_mode.flags & XFRM_MODE_FLAG_TUNNEL) {
    decaps = 1;
    break;
    }
//
// We need the inner address.  However, we only get here for
// transport mode so the outer address is identical.
//
    daddr = &x.id.daddr;
    family = x.props.family;
    err = xfrm_parse_spi(skb, nexthdr, &spi, &seq);
    if (err < 0) {
    XFRM_INC_STATS(net, LINUX_MIB_XFRMINHDRERROR);
    goto drop;
    }
    crypto_done = false;
    } while (!err);
    rcu_read_lock();
    err = xfrm_rcv_cb(skb, family, x.type.proto, 0);
    if (err) {
    rcu_read_unlock();
    goto drop;
    }
    nf_reset_ct(skb);
    if (decaps) {
    sp = skb_sec_path(skb);
    if (sp)
    sp.olen = 0;
    if (skb_valid_dst(skb))
    skb_dst_drop(skb);
    if (async)
    dev_put(dev);
    gro_cells_receive(&gro_cells, skb);
    rcu_read_unlock();
    return 0;
    } else {
    xo = xfrm_offload(skb);
    if (xo)
    xfrm_gro = xo.flags & XFRM_GRO;
    err = -EAFNOSUPPORT;
    afinfo = xfrm_state_afinfo_get_rcu(x.props.family);
    if (likely(afinfo))
    err = afinfo.transport_finish(skb, xfrm_gro || async);
    if (xfrm_gro) {
    sp = skb_sec_path(skb);
    if (sp)
    sp.olen = 0;
    if (skb_valid_dst(skb))
    skb_dst_drop(skb);
    gro_cells_receive(&gro_cells, skb);
    }
    if (async)
    dev_put(dev);
    rcu_read_unlock();
    return err;
    }
    drop_unlock:
    spin_unlock(&x.lock);
    drop:
    if (async)
    dev_put(dev);
    xfrm_rcv_cb(skb, family, x && x.type ? x.type.proto : nexthdr, -1);
    kfree_skb(skb);
    return 0;
    }
    EXPORT_SYMBOL(xfrm_input);
#[no_mangle]
pub unsafe extern "C" fn xfrm_input_resume(skb: *mut sk_buff, nexthdr: c_int) -> c_int {
    int xfrm_input_resume(struct sk_buff *skb, int nexthdr)
    {
    return xfrm_input(skb, nexthdr, 0, -1);
    }
    EXPORT_SYMBOL(xfrm_input_resume);
#[no_mangle]
unsafe extern "C" fn xfrm_trans_reinject(work: *mut work_struct) {
    static void xfrm_trans_reinject(struct work_struct *work)
    {
    struct xfrm_trans_tasklet *trans = container_of(work, struct xfrm_trans_tasklet, work);
    struct sk_buff_head queue;
    struct sk_buff *skb;
    __skb_queue_head_init(&queue);
    spin_lock_bh(&trans.queue_lock);
    skb_queue_splice_init(&trans.queue, &queue);
    spin_unlock_bh(&trans.queue_lock);
    local_bh_disable();
    while ((skb = __skb_dequeue(&queue))) {
    struct net *net = XFRM_TRANS_SKB_CB(skb).net;
    XFRM_TRANS_SKB_CB(skb).finish(net, core::ptr::null_mut(), skb);
    put_net(net);
    }
    local_bh_enable();
    }
    int xfrm_trans_queue_net(struct net *net, struct sk_buff *skb,
    int (*finish)(struct net *, struct sock *,
    struct sk_buff *))
    {
    struct xfrm_trans_tasklet *trans;
    struct net *hold_net;
    trans = this_cpu_ptr(&xfrm_trans_tasklet);
    if (skb_queue_len(&trans.queue) >= READ_ONCE(net_hotdata.max_backlog))
    return -ENOBUFS;
    BUILD_BUG_ON(sizeof(struct xfrm_trans_cb) > sizeof(skb.cb));
    hold_net = maybe_get_net(net);
    if (!hold_net)
    return -ENODEV;
    XFRM_TRANS_SKB_CB(skb).finish = finish;
    XFRM_TRANS_SKB_CB(skb).net = hold_net;
    spin_lock_bh(&trans.queue_lock);
    __skb_queue_tail(&trans.queue, skb);
    spin_unlock_bh(&trans.queue_lock);
    schedule_work(&trans.work);
    return 0;
    }
    EXPORT_SYMBOL(xfrm_trans_queue_net);
    int xfrm_trans_queue(struct sk_buff *skb,
    int (*finish)(struct net *, struct sock *,
    struct sk_buff *))
    {
    return xfrm_trans_queue_net(dev_net(skb.dev), skb, finish);
    }
    EXPORT_SYMBOL(xfrm_trans_queue);
#[no_mangle]
pub unsafe extern "C" fn xfrm_input_init() -> void __init {
    void __init xfrm_input_init(void)
    {
    int err;
    int i;
    xfrm_napi_dev = alloc_netdev_dummy(0);
    if (!xfrm_napi_dev)
    panic("Failed to allocate XFRM dummy netdev\n");
    err = gro_cells_init(&gro_cells, xfrm_napi_dev);
    if (err)
    gro_cells.cells = core::ptr::null_mut();
    for_each_possible_cpu(i) {
    struct xfrm_trans_tasklet *trans;
    trans = &per_cpu(xfrm_trans_tasklet, i);
    spin_lock_init(&trans.queue_lock);
    __skb_queue_head_init(&trans.queue);
    INIT_WORK(&trans.work, xfrm_trans_reinject);
    }
    }
