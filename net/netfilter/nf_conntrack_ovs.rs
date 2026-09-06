//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_ovs.c
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
// Support ct functions for openvswitch and used by OVS and TC conntrack.

// 'skb' should already be pulled to nh_ofs.
    int nf_ct_helper(struct sk_buff *skb, struct nf_conn *ct,
    enum ip_conntrack_info ctinfo, u16 proto)
    {
    int (*helper_cb)(struct sk_buff *skb, unsigned int protoff,
    struct nf_conn *ct,
    enum ip_conntrack_info conntrackinfo);
    const struct nf_conntrack_helper *helper;
    const struct nf_conn_help *help;
    unsigned int protoff;
    int err;
    if (ctinfo == IP_CT_RELATED_REPLY)
    return NF_ACCEPT;
    help = nfct_help(ct);
    if (!help)
    return NF_ACCEPT;
    helper = rcu_dereference(help.helper);
    if (!helper)
    return NF_ACCEPT;
    if (helper.nfproto != NFPROTO_UNSPEC &&
    helper.nfproto != proto)
    return NF_ACCEPT;
    switch (proto) {
    case NFPROTO_IPV4:
    protoff = ip_hdrlen(skb);
    proto = ip_hdr(skb).protocol;
    break;
    case NFPROTO_IPV6: {
    let mut nexthdr: u8 = ipv6_hdr(skb).nexthdr;
    __be16 frag_off;
    int ofs;
    ofs = ipv6_skip_exthdr(skb, sizeof(struct ipv6hdr), &nexthdr,
    &frag_off);
    if (ofs < 0 || (frag_off & htons(~0x7)) != 0) {
    pr_debug("proto header not found\n");
    return NF_ACCEPT;
    }
    protoff = ofs;
    proto = nexthdr;
    break;
    }
    default:
    WARN_ONCE(1, "helper invoked on non-IP family!");
    return NF_DROP;
    }
    if (helper.l4proto != proto)
    return NF_ACCEPT;
    helper_cb = rcu_dereference(helper.help);
    if (!helper_cb)
    return NF_ACCEPT;
    err = helper_cb(skb, protoff, ct, ctinfo);
    if (err != NF_ACCEPT)
    return err;
// Adjust seqs after helper.  This is needed due to some helpers (e.g.,
// FTP with NAT) adusting the TCP payload size when mangling IP
// addresses and/or port numbers in the text-based control connection.
//
    if (test_bit(IPS_SEQ_ADJUST_BIT, &ct.status) &&
    !nf_ct_seq_adjust(skb, ct, ctinfo, protoff))
    return NF_DROP;
    return NF_ACCEPT;
    }
    EXPORT_SYMBOL_GPL(nf_ct_helper);
    int nf_ct_add_helper(struct nf_conn *ct, const char *name, u8 family,
    u8 proto, bool nat, struct nf_conntrack_helper **hp)
    {
    struct nf_conntrack_helper *helper;
    struct nf_conn_help *help;
    let mut ret: c_int = 0;
    helper = nf_conntrack_helper_try_module_get(name, family, proto);
    if (!helper)
    return -EINVAL;
    help = nf_ct_helper_ext_add(ct, GFP_KERNEL);
    if (!help) {
    nf_conntrack_helper_put(helper);
    return -ENOMEM;
    }

    if (nat) {
    ret = nf_nat_helper_try_module_get(name, family, proto);
    if (ret) {
    nf_conntrack_helper_put(helper);
    return ret;
    }
    }

    rcu_assign_pointer(help.helper, helper);
// hp = helper;
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_ct_add_helper);
// Trim the skb to the length specified by the IP/IPv6 header,
// removing any trailing lower-layer padding. This prepares the skb
// for higher-layer processing that assumes skb->len excludes padding
// (such as nf_ip_checksum). The caller needs to pull the skb to the
// network header, and ensure ip_hdr/ipv6_hdr points to valid data.
//
#[no_mangle]
pub unsafe extern "C" fn nf_ct_skb_network_trim(skb: *mut sk_buff, family: c_int) -> c_int {
    int nf_ct_skb_network_trim(struct sk_buff *skb, int family)
    {
    unsigned int len;
    switch (family) {
    case NFPROTO_IPV4:
    len = skb_ip_totlen(skb);
    break;
    case NFPROTO_IPV6:
    len = skb_ipv6_payload_len(skb);
    if (ipv6_hdr(skb).nexthdr == NEXTHDR_HOP) {
    let mut err: c_int = nf_ip6_check_hbh_len(skb, &len);
    if (err)
    return err;
    }
    len += sizeof(struct ipv6hdr);
    break;
    default:
    len = skb.len;
    }
    return pskb_trim_rcsum(skb, len);
    }
    EXPORT_SYMBOL_GPL(nf_ct_skb_network_trim);
// Returns 0 on success, -EINPROGRESS if 'skb' is stolen, or other nonzero
// value if 'skb' is freed.
//
    int nf_ct_handle_fragments(struct net *net, struct sk_buff *skb,
    u16 zone, u8 family, u8 *proto, u16 *mru)
    {
    int err;
    if (family == NFPROTO_IPV4) {
    let mut user: enum ip_defrag_users = IP_DEFRAG_CONNTRACK_IN + zone;
    memset(IPCB(skb), 0, sizeof(struct inet_skb_parm));
    local_bh_disable();
    err = ip_defrag(net, skb, user);
    local_bh_enable();
    if (err)
    return err;
// mru = IPCB(skb)->frag_max_size;

    } else if (family == NFPROTO_IPV6) {
    let mut user: enum ip6_defrag_users = IP6_DEFRAG_CONNTRACK_IN + zone;
    memset(IP6CB(skb), 0, sizeof(struct inet6_skb_parm));
    err = nf_ct_frag6_gather(net, skb, user);
    if (err) {
    if (err != -EINPROGRESS)
    kfree_skb(skb);
    return err;
    }
// proto = ipv6_hdr(skb)->nexthdr;
// mru = IP6CB(skb)->frag_max_size;

    } else {
    kfree_skb(skb);
    return -EPFNOSUPPORT;
    }
    skb_clear_hash(skb);
    skb.ignore_df = 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(nf_ct_handle_fragments);
