//! Automatically rewritten from C to Rust
//! Source: net/ipv4/ip_vti.c
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
// Linux NET3: IP/IP protocol decoder modified to support
// virtual tunnel interface
//
// Authors:
// Saurabh Mohan (saurabh.mohan@vyatta.com) 05/07/2012
//
    This version of net/ipv4/ip_vti.c is cloned of net/ipv4/ipip.c
    For comments look at net/ipv4/ip_gre.c --ANK
//

    static struct rtnl_link_ops vti_link_ops __read_mostly;
    static unsigned int vti_net_id __read_mostly;
    static int vti_tunnel_init(struct net_device *dev);
    static int vti_input(struct sk_buff *skb, int nexthdr, __be32 spi,
    int encap_type, bool update_skb_dev)
    {
    struct ip_tunnel *tunnel;
    const struct iphdr *iph = ip_hdr(skb);
    struct net *net = dev_net(skb.dev);
    struct ip_tunnel_net *itn = net_generic(net, vti_net_id);
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    __set_bit(IP_TUNNEL_NO_KEY_BIT, flags);
    tunnel = ip_tunnel_lookup(itn, skb.dev.ifindex, flags,
    iph.saddr, iph.daddr, 0);
    if (tunnel) {
    if (!xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb))
    goto drop;
    XFRM_TUNNEL_SKB_CB(skb).tunnel.ip4 = tunnel;
    if (update_skb_dev)
    skb.dev = tunnel.dev;
    return xfrm_input(skb, nexthdr, spi, encap_type);
    }
    return -EINVAL;
    drop:
    kfree_skb(skb);
    return 0;
    }
    static int vti_input_proto(struct sk_buff *skb, int nexthdr, __be32 spi,
    int encap_type)
    {
    return vti_input(skb, nexthdr, spi, encap_type, false);
    }
#[no_mangle]
unsafe extern "C" fn vti_rcv(skb: *mut sk_buff, spi: __be32, update_skb_dev: bool) -> c_int {
    static int vti_rcv(struct sk_buff *skb, __be32 spi, bool update_skb_dev)
    {
    XFRM_SPI_SKB_CB(skb).family = AF_INET;
    XFRM_SPI_SKB_CB(skb).daddroff = offsetof(struct iphdr, daddr);
    return vti_input(skb, ip_hdr(skb).protocol, spi, 0, update_skb_dev);
    }
#[no_mangle]
unsafe extern "C" fn vti_rcv_proto(skb: *mut sk_buff) -> c_int {
    static int vti_rcv_proto(struct sk_buff *skb)
    {
    return vti_rcv(skb, 0, false);
    }
#[no_mangle]
unsafe extern "C" fn vti_rcv_cb(skb: *mut sk_buff, err: c_int) -> c_int {
    static int vti_rcv_cb(struct sk_buff *skb, int err)
    {
    unsigned short family;
    struct net_device *dev;
    struct xfrm_state *x;
    const struct xfrm_mode *inner_mode;
    struct ip_tunnel *tunnel = XFRM_TUNNEL_SKB_CB(skb).tunnel.ip4;
    let mut orig_mark: u32 = skb.mark;
    int ret;
    if (!tunnel)
    return 1;
    dev = tunnel.dev;
    if (err) {
    DEV_STATS_INC(dev, rx_errors);
    DEV_STATS_INC(dev, rx_dropped);
    return 0;
    }
    x = xfrm_input_state(skb);
    inner_mode = &x.inner_mode;
    if (x.sel.family == AF_UNSPEC) {
    inner_mode = xfrm_ip2inner_mode(x, XFRM_MODE_SKB_CB(skb).protocol);
    if (inner_mode == core::ptr::null_mut()) {
    XFRM_INC_STATS(dev_net(skb.dev),
    LINUX_MIB_XFRMINSTATEMODEERROR);
    return -EINVAL;
    }
    }
    family = inner_mode.family;
    skb.mark = be32_to_cpu(tunnel.parms.i_key);
    ret = xfrm_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb, family);
    skb.mark = orig_mark;
    if (!ret)
    return -EPERM;
    skb_scrub_packet(skb, !net_eq(tunnel.net, dev_net(skb.dev)));
    skb.dev = dev;
    dev_sw_netstats_rx_add(dev, skb.len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vti_state_check(x: *const xfrm_state, dst: __be32, src: __be32) -> bool {
    static bool vti_state_check(const struct xfrm_state *x, __be32 dst, __be32 src)
    {
    xfrm_address_t *daddr = (xfrm_address_t *)&dst;
    xfrm_address_t *saddr = (xfrm_address_t *)&src;
// if there is no transform then this tunnel is not functional.
// Or if the xfrm is not mode tunnel.
//
    if (!x || x.props.mode != XFRM_MODE_TUNNEL ||
    x.props.family != AF_INET)
    return false;
    if (!dst)
    return xfrm_addr_equal(saddr, &x.props.saddr, AF_INET);
    if (!xfrm_state_addr_check(x, daddr, saddr, AF_INET))
    return false;
    return true;
    }
    static netdev_tx_t vti_xmit(struct sk_buff *skb, struct net_device *dev,
    struct flowi *fl)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct ip_tunnel_parm_kern *parms = &tunnel.parms;
    struct dst_entry *dst = skb_dst(skb);
    struct net_device *tdev;	/* Device to other host */
    let mut pkt_len: c_int = skb.len;
    int err;
    int mtu;
    if (!dst) {
    switch (skb.protocol) {
    case htons(ETH_P_IP): {
    struct rtable *rt;
    fl.u.ip4.flowi4_oif = dev.ifindex;
    fl.u.ip4.flowi4_flags |= FLOWI_FLAG_ANYSRC;
    rt = __ip_route_output_key(dev_net(dev), &fl.u.ip4);
    if (IS_ERR(rt)) {
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error_icmp;
    }
    dst = &rt.dst;
    skb_dst_set(skb, dst);
    break;
    }

    case htons(ETH_P_IPV6):
    fl.u.ip6.flowi6_oif = dev.ifindex;
    fl.u.ip6.flowi6_flags |= FLOWI_FLAG_ANYSRC;
    dst = ip6_route_output(dev_net(dev), core::ptr::null_mut(), &fl.u.ip6);
    if (dst.error) {
    dst_release(dst);
    dst = core::ptr::null_mut();
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error_icmp;
    }
    skb_dst_set(skb, dst);
    break;

    default:
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error_icmp;
    }
    }
    dst_hold(dst);
    dst = xfrm_lookup_route(tunnel.net, dst, fl, core::ptr::null_mut(), 0);
    if (IS_ERR(dst)) {
    DEV_STATS_INC(dev, tx_carrier_errors);
    goto tx_error_icmp;
    }
    if (dst.flags & DST_XFRM_QUEUE)
    goto xmit;
    if (!vti_state_check(dst.xfrm, parms.iph.daddr, parms.iph.saddr)) {
    DEV_STATS_INC(dev, tx_carrier_errors);
    dst_release(dst);
    goto tx_error_icmp;
    }
    tdev = dst_dev(dst);
    if (tdev == dev) {
    dst_release(dst);
    DEV_STATS_INC(dev, collisions);
    goto tx_error;
    }
    mtu = dst_mtu(dst);
    if (skb.len > mtu) {
    skb_dst_update_pmtu_no_confirm(skb, mtu);
    if (skb.protocol == htons(ETH_P_IP)) {
    if (!(ip_hdr(skb).frag_off & htons(IP_DF)))
    goto xmit;
    icmp_ndo_send(skb, ICMP_DEST_UNREACH, ICMP_FRAG_NEEDED,
    htonl(mtu));
    } else {
    if (mtu < IPV6_MIN_MTU)
    mtu = IPV6_MIN_MTU;
    icmpv6_ndo_send(skb, ICMPV6_PKT_TOOBIG, 0, mtu);
    }
    dst_release(dst);
    goto tx_error;
    }
    xmit:
    skb_scrub_packet(skb, !net_eq(tunnel.net, dev_net(dev)));
    skb_dst_set(skb, dst);
    skb.dev = skb_dst_dev(skb);
    err = dst_output(tunnel.net, skb.sk, skb);
    if (net_xmit_eval(err) == 0)
    err = pkt_len;
    iptunnel_xmit_stats(dev, err);
    return NETDEV_TX_OK;
    tx_error_icmp:
    dst_link_failure(skb);
    tx_error:
    DEV_STATS_INC(dev, tx_errors);
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }
// This function assumes it is being called from dev_queue_xmit()
// and that skb is filled properly by that function.
//
#[no_mangle]
unsafe extern "C" fn vti_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t vti_tunnel_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct flowi fl;
    if (!pskb_inet_may_pull(skb))
    goto tx_err;
    memset(&fl, 0, sizeof(fl));
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    memset(IPCB(skb), 0, sizeof(*IPCB(skb)));
    xfrm_decode_session(dev_net(dev), skb, &fl, AF_INET);
    break;
    case htons(ETH_P_IPV6):
    memset(IP6CB(skb), 0, sizeof(*IP6CB(skb)));
    xfrm_decode_session(dev_net(dev), skb, &fl, AF_INET6);
    break;
    default:
    goto tx_err;
    }
// override mark with tunnel output key
    fl.flowi_mark = be32_to_cpu(tunnel.parms.o_key);
    return vti_xmit(skb, dev, &fl);
    tx_err:
    DEV_STATS_INC(dev, tx_errors);
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn vti4_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int vti4_err(struct sk_buff *skb, u32 info)
    {
    __be32 spi;
    __u32 mark;
    struct xfrm_state *x;
    struct ip_tunnel *tunnel;
    struct ip_esp_hdr *esph;
    struct ip_auth_hdr *ah ;
    struct ip_comp_hdr *ipch;
    struct net *net = dev_net(skb.dev);
    const struct iphdr *iph = (const struct iphdr *)skb.data;
    let mut protocol: c_int = iph.protocol;
    struct ip_tunnel_net *itn = net_generic(net, vti_net_id);
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    __set_bit(IP_TUNNEL_NO_KEY_BIT, flags);
    tunnel = ip_tunnel_lookup(itn, skb.dev.ifindex, flags,
    iph.daddr, iph.saddr, 0);
    if (!tunnel)
    return -1;
    mark = be32_to_cpu(tunnel.parms.o_key);
    switch (protocol) {
    case IPPROTO_ESP:
    esph = (struct ip_esp_hdr *)(skb.data+(iph.ihl<<2));
    spi = esph.spi;
    break;
    case IPPROTO_AH:
    ah = (struct ip_auth_hdr *)(skb.data+(iph.ihl<<2));
    spi = ah.spi;
    break;
    case IPPROTO_COMP:
    ipch = (struct ip_comp_hdr *)(skb.data+(iph.ihl<<2));
    spi = htonl(ntohs(ipch.cpi));
    break;
    default:
    return 0;
    }
    switch (icmp_hdr(skb).type) {
    case ICMP_DEST_UNREACH:
    if (icmp_hdr(skb).code != ICMP_FRAG_NEEDED)
    return 0;
    break;
    case ICMP_REDIRECT:
    break;
    default:
    return 0;
    }
    x = xfrm_state_lookup(net, mark, (const xfrm_address_t *)&iph.daddr,
    spi, protocol, AF_INET);
    if (!x)
    return 0;
    if (icmp_hdr(skb).type == ICMP_DEST_UNREACH)
    ipv4_update_pmtu(skb, net, info, 0, protocol);
    else
    ipv4_redirect(skb, net, 0, protocol);
    xfrm_state_put(x);
    return 0;
    }
    static int
    vti_tunnel_ctl(struct net_device *dev, struct ip_tunnel_parm_kern *p, int cmd)
    {
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    let mut err: c_int = 0;
    if (cmd == SIOCADDTUNNEL || cmd == SIOCCHGTUNNEL) {
    if (p.iph.version != 4 || p.iph.protocol != IPPROTO_IPIP ||
    p.iph.ihl != 5)
    return -EINVAL;
    }
    if (!ip_tunnel_flags_is_be16_compat(p.i_flags) ||
    !ip_tunnel_flags_is_be16_compat(p.o_flags))
    return -EOVERFLOW;
    if (!(ip_tunnel_flags_to_be16(p.i_flags) & GRE_KEY))
    p.i_key = 0;
    if (!(ip_tunnel_flags_to_be16(p.o_flags) & GRE_KEY))
    p.o_key = 0;
    __set_bit(IP_TUNNEL_VTI_BIT, flags);
    ip_tunnel_flags_copy(p.i_flags, flags);
    err = ip_tunnel_ctl(dev, p, cmd);
    if (err)
    return err;
    if (cmd != SIOCDELTUNNEL) {
    ip_tunnel_flags_from_be16(flags, GRE_KEY);
    ip_tunnel_flags_or(p.i_flags, p.i_flags, flags);
    ip_tunnel_flags_or(p.o_flags, p.o_flags, flags);
    }
    return 0;
    }
    static const struct net_device_ops vti_netdev_ops = {
    .ndo_init	= vti_tunnel_init,
    .ndo_uninit	= ip_tunnel_uninit,
    .ndo_start_xmit	= vti_tunnel_xmit,
    .ndo_siocdevprivate = ip_tunnel_siocdevprivate,
    .ndo_change_mtu	= ip_tunnel_change_mtu,
    .ndo_get_stats64 = dev_get_tstats64,
    .ndo_get_iflink = ip_tunnel_get_iflink,
    .ndo_tunnel_ctl	= vti_tunnel_ctl,
    };
#[no_mangle]
unsafe extern "C" fn vti_tunnel_setup(dev: *mut net_device) {
    static void vti_tunnel_setup(struct net_device *dev)
    {
    dev.netdev_ops		= &vti_netdev_ops;
    dev.header_ops		= &ip_tunnel_header_ops;
    dev.type		= ARPHRD_TUNNEL;
    ip_tunnel_setup(dev, vti_net_id);
    }
#[no_mangle]
unsafe extern "C" fn vti_tunnel_init(dev: *mut net_device) -> c_int {
    static int vti_tunnel_init(struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct iphdr *iph = &tunnel.parms.iph;
    __dev_addr_set(dev, &iph.saddr, 4);
    memcpy(dev.broadcast, &iph.daddr, 4);
    dev.flags		= IFF_NOARP;
    dev.addr_len		= 4;
    dev.lltx		= true;
    netif_keep_dst(dev);
    return ip_tunnel_init(dev);
    }
#[no_mangle]
unsafe extern "C" fn vti_fb_tunnel_init(dev: *mut net_device) -> void __net_init {
    static void __net_init vti_fb_tunnel_init(struct net_device *dev)
    {
    struct ip_tunnel *tunnel = netdev_priv(dev);
    struct iphdr *iph = &tunnel.parms.iph;
    iph.version		= 4;
    iph.protocol		= IPPROTO_IPIP;
    iph.ihl		= 5;
    }
    static struct xfrm4_protocol vti_esp4_protocol __read_mostly = {
    .handler	=	vti_rcv_proto,
    .input_handler	=	vti_input_proto,
    .cb_handler	=	vti_rcv_cb,
    .err_handler	=	vti4_err,
    .priority	=	100,
    };
    static struct xfrm4_protocol vti_ah4_protocol __read_mostly = {
    .handler	=	vti_rcv_proto,
    .input_handler	=	vti_input_proto,
    .cb_handler	=	vti_rcv_cb,
    .err_handler	=	vti4_err,
    .priority	=	100,
    };
    static struct xfrm4_protocol vti_ipcomp4_protocol __read_mostly = {
    .handler	=	vti_rcv_proto,
    .input_handler	=	vti_input_proto,
    .cb_handler	=	vti_rcv_cb,
    .err_handler	=	vti4_err,
    .priority	=	100,
    };

#[no_mangle]
unsafe extern "C" fn vti_rcv_tunnel(skb: *mut sk_buff) -> c_int {
    static int vti_rcv_tunnel(struct sk_buff *skb)
    {
    XFRM_SPI_SKB_CB(skb).family = AF_INET;
    XFRM_SPI_SKB_CB(skb).daddroff = offsetof(struct iphdr, daddr);
    return vti_input(skb, IPPROTO_IPIP, ip_hdr(skb).saddr, 0, false);
    }
    static struct xfrm_tunnel vti_ipip_handler __read_mostly = {
    .handler	=	vti_rcv_tunnel,
    .cb_handler	=	vti_rcv_cb,
    .err_handler	=	vti4_err,
    .priority	=	0,
    };

    static struct xfrm_tunnel vti_ipip6_handler __read_mostly = {
    .handler	=	vti_rcv_tunnel,
    .cb_handler	=	vti_rcv_cb,
    .err_handler	=	vti4_err,
    .priority	=	0,
    };

#[no_mangle]
unsafe extern "C" fn vti_init_net(net: *mut net) -> int __net_init {
    static int __net_init vti_init_net(struct net *net)
    {
    int err;
    struct ip_tunnel_net *itn;
    err = ip_tunnel_init_net(net, vti_net_id, &vti_link_ops, "ip_vti0");
    if (err)
    return err;
    itn = net_generic(net, vti_net_id);
    if (itn.fb_tunnel_dev)
    vti_fb_tunnel_init(itn.fb_tunnel_dev);
    return 0;
    }
    static void __net_exit vti_exit_rtnl(struct net *net,
    struct list_head *dev_to_kill)
    {
    ip_tunnel_delete_net(net, vti_net_id, &vti_link_ops, dev_to_kill);
    }
    static struct pernet_operations vti_net_ops = {
    .init = vti_init_net,
    .exit_rtnl = vti_exit_rtnl,
    .id   = &vti_net_id,
    .size = sizeof(struct ip_tunnel_net),
    };
    static int vti_tunnel_validate(struct nlattr *tb[], struct nlattr *data[],
    struct netlink_ext_ack *extack)
    {
    return 0;
    }
    static void vti_netlink_parms(struct nlattr *data[],
    struct ip_tunnel_parm_kern *parms,
    __u32 *fwmark)
    {
    memset(parms, 0, sizeof(*parms));
    parms.iph.protocol = IPPROTO_IPIP;
    if (!data)
    return;
    __set_bit(IP_TUNNEL_VTI_BIT, parms.i_flags);
    if (data[IFLA_VTI_LINK])
    parms.link = nla_get_u32(data[IFLA_VTI_LINK]);
    if (data[IFLA_VTI_IKEY])
    parms.i_key = nla_get_be32(data[IFLA_VTI_IKEY]);
    if (data[IFLA_VTI_OKEY])
    parms.o_key = nla_get_be32(data[IFLA_VTI_OKEY]);
    if (data[IFLA_VTI_LOCAL])
    parms.iph.saddr = nla_get_in_addr(data[IFLA_VTI_LOCAL]);
    if (data[IFLA_VTI_REMOTE])
    parms.iph.daddr = nla_get_in_addr(data[IFLA_VTI_REMOTE]);
    if (data[IFLA_VTI_FWMARK])
// fwmark = nla_get_u32(data[IFLA_VTI_FWMARK]);
    }
    static int vti_newlink(struct net_device *dev,
    struct rtnl_newlink_params *params,
    struct netlink_ext_ack *extack)
    {
    struct nlattr **data = params.data;
    struct ip_tunnel_parm_kern parms;
    struct nlattr **tb = params.tb;
    let mut fwmark: __u32 = 0;
    vti_netlink_parms(data, &parms, &fwmark);
    return ip_tunnel_newlink(params.link_net ? : dev_net(dev), dev, tb,
    &parms, fwmark);
    }
    static int vti_changelink(struct net_device *dev, struct nlattr *tb[],
    struct nlattr *data[],
    struct netlink_ext_ack *extack)
    {
    struct ip_tunnel *t = netdev_priv(dev);
    struct ip_tunnel_parm_kern p;
    let mut fwmark: __u32 = t.fwmark;
    if (!rtnl_dev_link_net_capable(dev, t.net))
    return -EPERM;
    vti_netlink_parms(data, &p, &fwmark);
    return ip_tunnel_changelink(dev, tb, &p, fwmark);
    }
#[no_mangle]
unsafe extern "C" fn vti_get_size(dev: *const net_device) -> usize {
    static size_t vti_get_size(const struct net_device *dev)
    {
    return
// IFLA_VTI_LINK
    nla_total_size(4) +
// IFLA_VTI_IKEY
    nla_total_size(4) +
// IFLA_VTI_OKEY
    nla_total_size(4) +
// IFLA_VTI_LOCAL
    nla_total_size(4) +
// IFLA_VTI_REMOTE
    nla_total_size(4) +
// IFLA_VTI_FWMARK
    nla_total_size(4) +
    0;
    }
#[no_mangle]
unsafe extern "C" fn vti_fill_info(skb: *mut sk_buff, dev: *const net_device) -> c_int {
    static int vti_fill_info(struct sk_buff *skb, const struct net_device *dev)
    {
    struct ip_tunnel *t = netdev_priv(dev);
    struct ip_tunnel_parm_kern *p = &t.parms;
    if (nla_put_u32(skb, IFLA_VTI_LINK, p.link) ||
    nla_put_be32(skb, IFLA_VTI_IKEY, p.i_key) ||
    nla_put_be32(skb, IFLA_VTI_OKEY, p.o_key) ||
    nla_put_in_addr(skb, IFLA_VTI_LOCAL, p.iph.saddr) ||
    nla_put_in_addr(skb, IFLA_VTI_REMOTE, p.iph.daddr) ||
    nla_put_u32(skb, IFLA_VTI_FWMARK, t.fwmark))
    return -EMSGSIZE;
    return 0;
    }
    static const struct nla_policy vti_policy[IFLA_VTI_MAX + 1] = {
    [IFLA_VTI_LINK]		= { .type = NLA_U32 },
    [IFLA_VTI_IKEY]		= { .type = NLA_U32 },
    [IFLA_VTI_OKEY]		= { .type = NLA_U32 },
    [IFLA_VTI_LOCAL]	= { .len = sizeof_field(struct iphdr, saddr) },
    [IFLA_VTI_REMOTE]	= { .len = sizeof_field(struct iphdr, daddr) },
    [IFLA_VTI_FWMARK]	= { .type = NLA_U32 },
    };
    static struct rtnl_link_ops vti_link_ops __read_mostly = {
    .kind		= "vti",
    .maxtype	= IFLA_VTI_MAX,
    .policy		= vti_policy,
    .priv_size	= sizeof(struct ip_tunnel),
    .setup		= vti_tunnel_setup,
    .validate	= vti_tunnel_validate,
    .newlink	= vti_newlink,
    .changelink	= vti_changelink,
    .dellink        = ip_tunnel_dellink,
    .get_size	= vti_get_size,
    .fill_info	= vti_fill_info,
    .get_link_net	= ip_tunnel_get_link_net,
    };
#[no_mangle]
unsafe extern "C" fn vti_init() -> int __init {
    static int __init vti_init(void)
    {
    const char *msg;
    int err;
    pr_info("IPv4 over IPsec tunneling driver\n");
    msg = "tunnel device";
    err = register_pernet_device(&vti_net_ops);
    if (err < 0)
    goto pernet_dev_failed;
    msg = "tunnel protocols";
    err = xfrm4_protocol_register(&vti_esp4_protocol, IPPROTO_ESP);
    if (err < 0)
    goto xfrm_proto_esp_failed;
    err = xfrm4_protocol_register(&vti_ah4_protocol, IPPROTO_AH);
    if (err < 0)
    goto xfrm_proto_ah_failed;
    err = xfrm4_protocol_register(&vti_ipcomp4_protocol, IPPROTO_COMP);
    if (err < 0)
    goto xfrm_proto_comp_failed;

    msg = "ipip tunnel";
    err = xfrm4_tunnel_register(&vti_ipip_handler, AF_INET);
    if (err < 0)
    goto xfrm_tunnel_ipip_failed;

    err = xfrm4_tunnel_register(&vti_ipip6_handler, AF_INET6);
    if (err < 0)
    goto xfrm_tunnel_ipip6_failed;

    msg = "netlink interface";
    err = rtnl_link_register(&vti_link_ops);
    if (err < 0)
    goto rtnl_link_failed;
    return err;
    rtnl_link_failed:

    xfrm4_tunnel_deregister(&vti_ipip6_handler, AF_INET6);
    xfrm_tunnel_ipip6_failed:

    xfrm4_tunnel_deregister(&vti_ipip_handler, AF_INET);
    xfrm_tunnel_ipip_failed:

    xfrm4_protocol_deregister(&vti_ipcomp4_protocol, IPPROTO_COMP);
    xfrm_proto_comp_failed:
    xfrm4_protocol_deregister(&vti_ah4_protocol, IPPROTO_AH);
    xfrm_proto_ah_failed:
    xfrm4_protocol_deregister(&vti_esp4_protocol, IPPROTO_ESP);
    xfrm_proto_esp_failed:
    unregister_pernet_device(&vti_net_ops);
    pernet_dev_failed:
    pr_err("vti init: failed to register %s\n", msg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vti_fini() -> void __exit {
    static void __exit vti_fini(void)
    {
    rtnl_link_unregister(&vti_link_ops);

    xfrm4_tunnel_deregister(&vti_ipip6_handler, AF_INET6);

    xfrm4_tunnel_deregister(&vti_ipip_handler, AF_INET);

    xfrm4_protocol_deregister(&vti_ipcomp4_protocol, IPPROTO_COMP);
    xfrm4_protocol_deregister(&vti_ah4_protocol, IPPROTO_AH);
    xfrm4_protocol_deregister(&vti_esp4_protocol, IPPROTO_ESP);
    unregister_pernet_device(&vti_net_ops);
    }
    module_init(vti_init);
    module_exit(vti_fini);
    MODULE_DESCRIPTION("Virtual (secure) IP tunneling library");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_RTNL_LINK("vti");
    MODULE_ALIAS_NETDEV("ip_vti0");
