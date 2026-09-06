//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_proto_gre.c
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
// Connection tracking protocol helper module for GRE.
//
// GRE is a generic encapsulation protocol, which is generally not very
// suited for NAT, as it has no protocol-specific part as port numbers.
//
// It has an optional key field, which may help us distinguishing two
// connections between the same two hosts.
//
// GRE is defined in RFC 1701 and RFC 1702, as well as RFC 2784
//
// PPTP is built on top of a modified version of GRE, and has a mandatory
// field called "CallID", which serves us for the same purpose as the key
// field in plain GRE.
//
// Documentation about PPTP can be found in RFC 2637
//
// (C) 2000-2005 by Harald Welte <laforge@gnumonks.org>
//
// Development of this code funded by Astaro AG (http://www.astaro.com/)
//
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

    static const unsigned int gre_timeouts[GRE_CT_MAX] = {
    [GRE_CT_UNREPLIED]	= 30*HZ,
    [GRE_CT_REPLIED]	= 180*HZ,
    };
// used when expectation is added
    static DEFINE_SPINLOCK(keymap_lock);
    static inline struct nf_gre_net *gre_pernet(struct net *net)
    {
    return &net.ct.nf_ct_proto.gre;
    }
    static inline int gre_key_cmpfn(const struct nf_ct_gre_keymap *km,
    const struct nf_conntrack_tuple *t)
    {
    return km.tuple.src.l3num == t.src.l3num &&
    !memcmp(&km.tuple.src.u3, &t.src.u3, sizeof(t.src.u3)) &&
    !memcmp(&km.tuple.dst.u3, &t.dst.u3, sizeof(t.dst.u3)) &&
    km.tuple.dst.protonum == t.dst.protonum &&
    km.tuple.dst.u.all == t.dst.u.all;
    }
// look up the source key for a given tuple
#[no_mangle]
unsafe extern "C" fn gre_keymap_lookup(net: *mut net, t: *mut nf_conntrack_tuple) -> __be16 {
    static __be16 gre_keymap_lookup(struct net *net, struct nf_conntrack_tuple *t)
    {
    struct nf_gre_net *net_gre = gre_pernet(net);
    struct nf_ct_gre_keymap *km;
    let mut key: __be16 = 0;
    list_for_each_entry_rcu(km, &net_gre.keymap_list, list) {
    if (gre_key_cmpfn(km, t)) {
    key = km.tuple.src.u.gre.key;
    break;
    }
    }
    pr_debug("lookup src key 0x%x for ", key);
    nf_ct_dump_tuple(t);
    return key;
    }
    enum nf_ct_gre_km_act {
    NF_CT_GRE_KM_NEW,
    NF_CT_GRE_KM_BAD,
    NF_CT_GRE_KM_DUP
    };
    static enum nf_ct_gre_km_act
    nf_ct_gre_km_acceptable(const struct nf_ct_pptp_master *ct_pptp_info,
    const struct nf_conntrack_tuple *orig,
    const struct nf_conntrack_tuple *repl)
    {
    struct nf_ct_gre_keymap *km_orig, *km_repl;
    lockdep_assert_held(&keymap_lock);
    km_orig = ct_pptp_info.keymap[IP_CT_DIR_ORIGINAL];
    km_repl = ct_pptp_info.keymap[IP_CT_DIR_REPLY];
    if (km_orig && km_repl) {
    if (!gre_key_cmpfn(km_orig, orig))
    return NF_CT_GRE_KM_BAD;
    if (!gre_key_cmpfn(km_repl, repl))
    return NF_CT_GRE_KM_BAD;
    return NF_CT_GRE_KM_DUP;
    }
    DEBUG_NET_WARN_ON_ONCE(km_orig);
    DEBUG_NET_WARN_ON_ONCE(km_repl);
    return NF_CT_GRE_KM_NEW;
    }
// add keymap entries, associate with specified master ct
    bool nf_ct_gre_keymap_add(struct nf_conn *ct,
    const struct nf_conntrack_tuple *orig,
    const struct nf_conntrack_tuple *repl)
    {
    struct net *net = nf_ct_net(ct);
    struct nf_gre_net *net_gre = gre_pernet(net);
    struct nf_ct_pptp_master *ct_pptp_info = nfct_help_data(ct);
    struct nf_ct_gre_keymap *km_orig, *km_repl;
    let mut ret: bool = false;
    if (!ct_pptp_info)
    return false;
    km_orig = kmalloc_obj(*km_orig, GFP_ATOMIC);
    if (!km_orig)
    return false;
    km_repl = kmalloc_obj(*km_repl, GFP_ATOMIC);
    if (!km_repl)
    goto km_free;
    memcpy(&km_orig.tuple, orig, sizeof(*orig));
    memcpy(&km_repl.tuple, repl, sizeof(*repl));
    spin_lock_bh(&keymap_lock);
    if (nf_ct_is_dying(ct))
    goto unlock_free;
    switch (nf_ct_gre_km_acceptable(ct_pptp_info, orig, repl)) {
    case NF_CT_GRE_KM_NEW:
    break;
    case NF_CT_GRE_KM_DUP:
    ret = true;
    goto unlock_free;
    case NF_CT_GRE_KM_BAD:
    pr_debug("trying to override keymap for ct %p\n", ct);
    goto unlock_free;
    }
    if (ct_pptp_info.keymap[IP_CT_DIR_ORIGINAL] ||
    ct_pptp_info.keymap[IP_CT_DIR_REPLY])
    goto unlock_free;
    pr_debug("adding new entries %p,%p: ", km_orig, km_repl);
    nf_ct_dump_tuple(&km_orig.tuple);
    nf_ct_dump_tuple(&km_repl.tuple);
    list_add_tail_rcu(&km_orig.list, &net_gre.keymap_list);
    list_add_tail_rcu(&km_repl.list, &net_gre.keymap_list);
    ct_pptp_info.keymap[IP_CT_DIR_ORIGINAL] = km_orig;
    ct_pptp_info.keymap[IP_CT_DIR_REPLY] = km_repl;
    spin_unlock_bh(&keymap_lock);
    return true;
    unlock_free:
    spin_unlock_bh(&keymap_lock);
    km_free:
    kfree(km_orig);
    kfree(km_repl);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_ct_gre_keymap_add);
// destroy the keymap entries associated with specified master ct
#[no_mangle]
pub unsafe extern "C" fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn) {
    void nf_ct_gre_keymap_destroy(struct nf_conn *ct)
    {
    struct nf_ct_pptp_master *ct_pptp_info = nfct_help_data(ct);
    enum ip_conntrack_dir dir;
    if (!ct_pptp_info)
    return;
    pr_debug("entering for ct %p\n", ct);
    spin_lock_bh(&keymap_lock);
    for (dir = IP_CT_DIR_ORIGINAL; dir < IP_CT_DIR_MAX; dir++) {
    if (ct_pptp_info.keymap[dir]) {
    pr_debug("removing %p from list\n",
    ct_pptp_info.keymap[dir]);
    list_del_rcu(&ct_pptp_info.keymap[dir].list);
    kfree_rcu(ct_pptp_info.keymap[dir], rcu);
    ct_pptp_info.keymap[dir] = core::ptr::null_mut();
    }
    }
    spin_unlock_bh(&keymap_lock);
    }
    EXPORT_SYMBOL_GPL(nf_ct_gre_keymap_destroy);
// PUBLIC CONNTRACK PROTO HELPER FUNCTIONS
// gre hdr info to tuple
    bool gre_pkt_to_tuple(const struct sk_buff *skb, unsigned int dataoff,
    struct net *net, struct nf_conntrack_tuple *tuple)
    {
    const struct pptp_gre_header *pgrehdr;
    struct pptp_gre_header _pgrehdr;
    __be16 srckey;
    const struct gre_base_hdr *grehdr;
    struct gre_base_hdr _grehdr;
// first only delinearize old RFC1701 GRE header
    grehdr = skb_header_pointer(skb, dataoff, sizeof(_grehdr), &_grehdr);
    if (!grehdr || (grehdr.flags & GRE_VERSION) != GRE_VERSION_1) {
// try to behave like "nf_conntrack_proto_generic"
    tuple.src.u.all = 0;
    tuple.dst.u.all = 0;
    return true;
    }
// PPTP header is variable length, only need up to the call_id field
    pgrehdr = skb_header_pointer(skb, dataoff, 8, &_pgrehdr);
    if (!pgrehdr)
    return true;
    if (grehdr.protocol != GRE_PROTO_PPP) {
    pr_debug("Unsupported GRE proto(0x%x)\n", ntohs(grehdr.protocol));
    return false;
    }
    tuple.dst.u.gre.key = pgrehdr.call_id;
    srckey = gre_keymap_lookup(net, tuple);
    tuple.src.u.gre.key = srckey;
    return true;
    }

// print private data for conntrack
#[no_mangle]
unsafe extern "C" fn gre_print_conntrack(s: *mut seq_file, ct: *mut nf_conn) {
    static void gre_print_conntrack(struct seq_file *s, struct nf_conn *ct)
    {
    seq_printf(s, "timeout=%u, stream_timeout=%u ",
    (ct.proto.gre.timeout / HZ),
    (ct.proto.gre.stream_timeout / HZ));
    }

    static unsigned int *gre_get_timeouts(struct net *net)
    {
    return gre_pernet(net).timeouts;
    }
// Returns verdict for packet, and may modify conntrack
    int nf_conntrack_gre_packet(struct nf_conn *ct,
    struct sk_buff *skb,
    unsigned int dataoff,
    enum ip_conntrack_info ctinfo,
    const struct nf_hook_state *state)
    {
    unsigned long status;
    if (!nf_ct_is_confirmed(ct)) {
    unsigned int *timeouts = nf_ct_timeout_lookup(ct);
    if (!timeouts)
    timeouts = gre_get_timeouts(nf_ct_net(ct));
// initialize to sane value.  Ideally a conntrack helper
// (e.g. in case of pptp) is increasing them
    ct.proto.gre.stream_timeout = timeouts[GRE_CT_REPLIED];
    ct.proto.gre.timeout = timeouts[GRE_CT_UNREPLIED];
    }
    status = READ_ONCE(ct.status);
// If we've seen traffic both ways, this is a GRE connection.
// Extend timeout.
    if (status & IPS_SEEN_REPLY) {
    nf_ct_refresh_acct(ct, ctinfo, skb,
    ct.proto.gre.stream_timeout);
// never set ASSURED for IPS_NAT_CLASH, they time out soon
    if (unlikely((status & IPS_NAT_CLASH)))
    return NF_ACCEPT;
// Also, more likely to be important, and not a probe.
    if (!test_and_set_bit(IPS_ASSURED_BIT, &ct.status))
    nf_conntrack_event_cache(IPCT_ASSURED, ct);
    } else
    nf_ct_refresh_acct(ct, ctinfo, skb,
    ct.proto.gre.timeout);
    return NF_ACCEPT;
    }

    static int gre_timeout_nlattr_to_obj(struct nlattr *tb[],
    struct net *net, void *data)
    {
    unsigned int *timeouts = data;
    struct nf_gre_net *net_gre = gre_pernet(net);
    if (!timeouts)
    timeouts = gre_get_timeouts(net);
// set default timeouts for GRE.
    timeouts[GRE_CT_UNREPLIED] = net_gre.timeouts[GRE_CT_UNREPLIED];
    timeouts[GRE_CT_REPLIED] = net_gre.timeouts[GRE_CT_REPLIED];
    if (tb[CTA_TIMEOUT_GRE_UNREPLIED]) {
    timeouts[GRE_CT_UNREPLIED] =
    ntohl(nla_get_be32(tb[CTA_TIMEOUT_GRE_UNREPLIED])) * HZ;
    }
    if (tb[CTA_TIMEOUT_GRE_REPLIED]) {
    timeouts[GRE_CT_REPLIED] =
    ntohl(nla_get_be32(tb[CTA_TIMEOUT_GRE_REPLIED])) * HZ;
    }
    return 0;
    }
    static int
    gre_timeout_obj_to_nlattr(struct sk_buff *skb, const void *data)
    {
    const unsigned int *timeouts = data;
    if (nla_put_be32(skb, CTA_TIMEOUT_GRE_UNREPLIED,
    htonl(timeouts[GRE_CT_UNREPLIED] / HZ)) ||
    nla_put_be32(skb, CTA_TIMEOUT_GRE_REPLIED,
    htonl(timeouts[GRE_CT_REPLIED] / HZ)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -ENOSPC;
    }
    static const struct nla_policy
    gre_timeout_nla_policy[CTA_TIMEOUT_GRE_MAX+1] = {
    [CTA_TIMEOUT_GRE_UNREPLIED]	= { .type = NLA_U32 },
    [CTA_TIMEOUT_GRE_REPLIED]	= { .type = NLA_U32 },
    };

    static int destroy_sibling_or_exp(struct net *net, struct nf_conn *ct,
    const struct nf_conntrack_tuple *t)
    {
    const struct nf_conntrack_tuple_hash *h;
    const struct nf_conntrack_zone *zone;
    struct nf_conntrack_expect *exp;
    struct nf_conn *sibling;
    pr_debug("trying to timeout ct or exp for tuple ");
    nf_ct_dump_tuple(t);
    zone = nf_ct_zone(ct);
    h = nf_conntrack_find_get(net, zone, t);
    if (h)  {
    sibling = nf_ct_tuplehash_to_ctrack(h);
    pr_debug("setting timeout of conntrack %p to 0\n", sibling);
    sibling.proto.gre.timeout        = 0;
    sibling.proto.gre.stream_timeout = 0;
    nf_ct_kill(sibling);
    nf_ct_put(sibling);
    return 1;
    } else {
    exp = nf_ct_expect_find_get(net, zone, t);
    if (exp) {
    pr_debug("unexpect_related of expect %p\n", exp);
    nf_ct_unexpect_related(exp);
    nf_ct_expect_put(exp);
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gre_pptp_destroy_siblings(ct: *mut nf_conn) {
    void gre_pptp_destroy_siblings(struct nf_conn *ct)
    {
    struct net *net = nf_ct_net(ct);
    const struct nf_ct_pptp_master *ct_pptp_info = nfct_help_data(ct);
    struct nf_conntrack_tuple t;
    if (!ct_pptp_info)
    return;
    nf_ct_gre_keymap_destroy(ct);
// try original (pns->pac) tuple
    memcpy(&t, &ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple, sizeof(t));
    t.dst.protonum = IPPROTO_GRE;
    t.src.u.gre.key = ct_pptp_info.pns_call_id;
    t.dst.u.gre.key = ct_pptp_info.pac_call_id;
    if (!destroy_sibling_or_exp(net, ct, &t))
    pr_debug("failed to timeout original pns.pac ct/exp\n");
// try reply (pac->pns) tuple
    memcpy(&t, &ct.tuplehash[IP_CT_DIR_REPLY].tuple, sizeof(t));
    t.dst.protonum = IPPROTO_GRE;
    t.src.u.gre.key = ct_pptp_info.pac_call_id;
    t.dst.u.gre.key = ct_pptp_info.pns_call_id;
    if (!destroy_sibling_or_exp(net, ct, &t))
    pr_debug("failed to timeout reply pac.pns ct/exp\n");
    }
    EXPORT_SYMBOL_GPL(gre_pptp_destroy_siblings);

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_gre_init_net(net: *mut net) {
    void nf_conntrack_gre_init_net(struct net *net)
    {
    struct nf_gre_net *net_gre = gre_pernet(net);
    int i;
    INIT_LIST_HEAD(&net_gre.keymap_list);
    for (i = 0; i < GRE_CT_MAX; i++)
    net_gre.timeouts[i] = gre_timeouts[i];
    }
// protocol helper struct
    const struct nf_conntrack_l4proto nf_conntrack_l4proto_gre = {
    .l4proto	 = IPPROTO_GRE,
    .allow_clash	 = true,

    .print_conntrack = gre_print_conntrack,

    .tuple_to_nlattr = nf_ct_port_tuple_to_nlattr,
    .nlattr_tuple_size = nf_ct_port_nlattr_tuple_size,
    .nlattr_to_tuple = nf_ct_port_nlattr_to_tuple,
    .nla_policy	 = nf_ct_port_nla_policy,

    .ctnl_timeout    = {
    .nlattr_to_obj	= gre_timeout_nlattr_to_obj,
    .obj_to_nlattr	= gre_timeout_obj_to_nlattr,
    .nlattr_max	= CTA_TIMEOUT_GRE_MAX,
    .obj_size	= sizeof(unsigned int) * GRE_CT_MAX,
    .nla_policy	= gre_timeout_nla_policy,
    },

    };
