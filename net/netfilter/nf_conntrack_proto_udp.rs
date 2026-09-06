//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_proto_udp.c
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
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

    static const unsigned int udp_timeouts[UDP_CT_MAX] = {
    [UDP_CT_UNREPLIED]	= 30*HZ,
    [UDP_CT_REPLIED]	= 120*HZ,
    };
    static unsigned int *udp_get_timeouts(struct net *net)
    {
    return nf_udp_pernet(net).timeouts;
    }
    static void udp_error_log(const struct sk_buff *skb,
    const struct nf_hook_state *state,
    const char *msg)
    {
    nf_l4proto_log_invalid(skb, state, IPPROTO_UDP, "%s", msg);
    }
    static bool udp_validate_len(struct sk_buff *skb,
    const struct udphdr *hdr,
    unsigned int dataoff)
    {
    let mut udplen: c_uint = udp_get_len(skb, hdr, dataoff);
    let mut skblen: c_uint = skb.len - dataoff;
    if (udplen > skblen || udplen < sizeof(*hdr))
    return false;
    return true;
    }
    static bool udp_error(struct sk_buff *skb,
    unsigned int dataoff,
    const struct nf_hook_state *state)
    {
    const struct udphdr *hdr;
    struct udphdr _hdr;
// Header is too small?
    hdr = skb_header_pointer(skb, dataoff, sizeof(_hdr), &_hdr);
    if (!hdr) {
    udp_error_log(skb, state, "short packet");
    return true;
    }
// Truncated/malformed packets
    if (!udp_validate_len(skb, hdr, dataoff)) {
    udp_error_log(skb, state, "truncated/malformed packet");
    return true;
    }
// Packet with no checksum
    if (!hdr.check)
    return false;
// Checksum invalid? Ignore.
// We skip checking packets on the outgoing path
// because the checksum is assumed to be correct.
// FIXME: Source route IP option packets --RR
    if (state.hook == NF_INET_PRE_ROUTING &&
    state.net.ct.sysctl_checksum &&
    nf_checksum(skb, state.hook, dataoff, IPPROTO_UDP, state.pf)) {
    udp_error_log(skb, state, "bad checksum");
    return true;
    }
    return false;
    }
// Returns verdict for packet, and may modify conntracktype
    int nf_conntrack_udp_packet(struct nf_conn *ct,
    struct sk_buff *skb,
    unsigned int dataoff,
    enum ip_conntrack_info ctinfo,
    const struct nf_hook_state *state)
    {
    unsigned int *timeouts;
    unsigned long status;
    if (udp_error(skb, dataoff, state))
    return -NF_ACCEPT;
    timeouts = nf_ct_timeout_lookup(ct);
    if (!timeouts)
    timeouts = udp_get_timeouts(nf_ct_net(ct));
    status = READ_ONCE(ct.status);
    if ((status & IPS_CONFIRMED) == 0)
    ct.proto.udp.stream_ts = 2 * HZ + jiffies;
// If we've seen traffic both ways, this is some kind of UDP
// stream. Set Assured.
//
    if (status & IPS_SEEN_REPLY) {
    let mut extra: c_ulong = timeouts[UDP_CT_UNREPLIED];
    let mut stream: bool = false;
// Still active after two seconds? Extend timeout.
    if (time_after(jiffies, ct.proto.udp.stream_ts)) {
    extra = timeouts[UDP_CT_REPLIED];
    stream = (status & IPS_ASSURED) == 0;
    }
    nf_ct_refresh_acct(ct, ctinfo, skb, extra);
// never set ASSURED for IPS_NAT_CLASH, they time out soon
    if (unlikely((status & IPS_NAT_CLASH)))
    return NF_ACCEPT;
// Also, more likely to be important, and not a probe
    if (stream && !test_and_set_bit(IPS_ASSURED_BIT, &ct.status))
    nf_conntrack_event_cache(IPCT_ASSURED, ct);
    } else {
    nf_ct_refresh_acct(ct, ctinfo, skb, timeouts[UDP_CT_UNREPLIED]);
    }
    return NF_ACCEPT;
    }

    static int udp_timeout_nlattr_to_obj(struct nlattr *tb[],
    struct net *net, void *data)
    {
    unsigned int *timeouts = data;
    struct nf_udp_net *un = nf_udp_pernet(net);
    if (!timeouts)
    timeouts = un.timeouts;
// set default timeouts for UDP.
    timeouts[UDP_CT_UNREPLIED] = un.timeouts[UDP_CT_UNREPLIED];
    timeouts[UDP_CT_REPLIED] = un.timeouts[UDP_CT_REPLIED];
    if (tb[CTA_TIMEOUT_UDP_UNREPLIED]) {
    timeouts[UDP_CT_UNREPLIED] =
    ntohl(nla_get_be32(tb[CTA_TIMEOUT_UDP_UNREPLIED])) * HZ;
    }
    if (tb[CTA_TIMEOUT_UDP_REPLIED]) {
    timeouts[UDP_CT_REPLIED] =
    ntohl(nla_get_be32(tb[CTA_TIMEOUT_UDP_REPLIED])) * HZ;
    }
    return 0;
    }
    static int
    udp_timeout_obj_to_nlattr(struct sk_buff *skb, const void *data)
    {
    const unsigned int *timeouts = data;
    if (nla_put_be32(skb, CTA_TIMEOUT_UDP_UNREPLIED,
    htonl(timeouts[UDP_CT_UNREPLIED] / HZ)) ||
    nla_put_be32(skb, CTA_TIMEOUT_UDP_REPLIED,
    htonl(timeouts[UDP_CT_REPLIED] / HZ)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -ENOSPC;
    }
    static const struct nla_policy
    udp_timeout_nla_policy[CTA_TIMEOUT_UDP_MAX+1] = {
    [CTA_TIMEOUT_UDP_UNREPLIED]	= { .type = NLA_U32 },
    [CTA_TIMEOUT_UDP_REPLIED]	= { .type = NLA_U32 },
    };

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_udp_init_net(net: *mut net) {
    void nf_conntrack_udp_init_net(struct net *net)
    {
    struct nf_udp_net *un = nf_udp_pernet(net);
    int i;
    for (i = 0; i < UDP_CT_MAX; i++)
    un.timeouts[i] = udp_timeouts[i];

    un.offload_timeout = 30 * HZ;

    }
    const struct nf_conntrack_l4proto nf_conntrack_l4proto_udp =
    {
    .l4proto		= IPPROTO_UDP,
    .allow_clash		= true,

    .tuple_to_nlattr	= nf_ct_port_tuple_to_nlattr,
    .nlattr_to_tuple	= nf_ct_port_nlattr_to_tuple,
    .nlattr_tuple_size	= nf_ct_port_nlattr_tuple_size,
    .nla_policy		= nf_ct_port_nla_policy,

    .ctnl_timeout		= {
    .nlattr_to_obj	= udp_timeout_nlattr_to_obj,
    .obj_to_nlattr	= udp_timeout_obj_to_nlattr,
    .nlattr_max	= CTA_TIMEOUT_UDP_MAX,
    .obj_size	= sizeof(unsigned int) * CTA_TIMEOUT_UDP_MAX,
    .nla_policy	= udp_timeout_nla_policy,
    },

    };
