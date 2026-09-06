//! Automatically rewritten from C to Rust
//! Source: net/sched/em_ipt.c
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
// net/sched/em_ipt.c IPtables matches Ematch
//
// (c) 2018 Eyal Birger <eyal.birger@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_ipt_match {
    pub match: *const xt_match,
    pub hook: u32,
    pub nfproto: u8,
    pub __aligned(8): u8 match_data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_ipt_xt_match {
    pub match_name: *mut c_char,
    pub mrev): *mut *mut *mut *mut int (validate_match_data)(struct nlattr tb, u8,
}

    static const struct nla_policy em_ipt_policy[TCA_EM_IPT_MAX + 1] = {
    [TCA_EM_IPT_MATCH_NAME]		= { .type = NLA_STRING,
    .len = XT_EXTENSION_MAXNAMELEN },
    [TCA_EM_IPT_MATCH_REVISION]	= { .type = NLA_U8 },
    [TCA_EM_IPT_HOOK]		= { .type = NLA_U32 },
    [TCA_EM_IPT_NFPROTO]		= { .type = NLA_U8 },
    [TCA_EM_IPT_MATCH_DATA]		= { .type = NLA_UNSPEC },
    };
#[no_mangle]
unsafe extern "C" fn check_match(net: *mut net, im: *mut em_ipt_match, mdata_len: c_int) -> c_int {
    static int check_match(struct net *net, struct em_ipt_match *im, int mdata_len)
    {
    let mut mtpar: xt_mtchk_param = {};
    union {
    struct ipt_entry e4;
    struct ip6t_entry e6;
    } e = {};
    mtpar.net	= net;
    mtpar.table	= "filter";
    mtpar.hook_mask	= 1 << im.hook;
    mtpar.family	= im.match.family;
    mtpar.match	= im.match;
    mtpar.entryinfo = &e;
    mtpar.matchinfo	= (void *)im.match_data;
    return xt_check_match(&mtpar, mdata_len, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn policy_validate_match_data(tb: *mut nlattr, mrev: u8) -> c_int {
    static int policy_validate_match_data(struct nlattr **tb, u8 mrev)
    {
    if (mrev != 0) {
    pr_err("only policy match revision 0 supported");
    return -EINVAL;
    }
    if (nla_get_u32(tb[TCA_EM_IPT_HOOK]) != NF_INET_PRE_ROUTING) {
    pr_err("policy can only be matched on NF_INET_PRE_ROUTING");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn addrtype_validate_match_data(tb: *mut nlattr, mrev: u8) -> c_int {
    static int addrtype_validate_match_data(struct nlattr **tb, u8 mrev)
    {
    if (mrev != 1) {
    pr_err("only addrtype match revision 1 supported");
    return -EINVAL;
    }
    return 0;
    }
    static const struct em_ipt_xt_match em_ipt_xt_matches[] = {
    {
    .match_name = "policy",
    .validate_match_data = policy_validate_match_data
    },
    {
    .match_name = "addrtype",
    .validate_match_data = addrtype_validate_match_data
    },
    {}
    };
    static struct xt_match *get_xt_match(struct nlattr **tb)
    {
    const struct em_ipt_xt_match *m;
    struct nlattr *mname_attr;
    u8 nfproto, mrev = 0;
    int ret;
    mname_attr = tb[TCA_EM_IPT_MATCH_NAME];
    for (m = em_ipt_xt_matches; m.match_name; m++) {
    if (!nla_strcmp(mname_attr, m.match_name))
    break;
    }
    if (!m.match_name) {
    pr_err("Unsupported xt match");
    return ERR_PTR(-EINVAL);
    }
    if (tb[TCA_EM_IPT_MATCH_REVISION])
    mrev = nla_get_u8(tb[TCA_EM_IPT_MATCH_REVISION]);
    ret = m.validate_match_data(tb, mrev);
    if (ret < 0)
    return ERR_PTR(ret);
    nfproto = nla_get_u8(tb[TCA_EM_IPT_NFPROTO]);
    return xt_request_find_match(nfproto, m.match_name, mrev);
    }
    static int em_ipt_change(struct net *net, void *data, int data_len,
    struct tcf_ematch *em)
    {
    struct nlattr *tb[TCA_EM_IPT_MAX + 1];
    struct em_ipt_match *im = core::ptr::null_mut();
    struct xt_match *match;
    int mdata_len, ret;
    u8 nfproto;
    ret = nla_parse_deprecated(tb, TCA_EM_IPT_MAX, data, data_len,
    em_ipt_policy, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    if (!tb[TCA_EM_IPT_HOOK] || !tb[TCA_EM_IPT_MATCH_NAME] ||
    !tb[TCA_EM_IPT_MATCH_DATA] || !tb[TCA_EM_IPT_NFPROTO])
    return -EINVAL;
    nfproto = nla_get_u8(tb[TCA_EM_IPT_NFPROTO]);
    switch (nfproto) {
    case NFPROTO_IPV4:
    case NFPROTO_IPV6:
    break;
    default:
    return -EINVAL;
    }
    match = get_xt_match(tb);
    if (IS_ERR(match)) {
    pr_err("unable to load match\n");
    return PTR_ERR(match);
    }
    mdata_len = XT_ALIGN(nla_len(tb[TCA_EM_IPT_MATCH_DATA]));
    im = kzalloc(sizeof(*im) + mdata_len, GFP_KERNEL);
    if (!im) {
    ret = -ENOMEM;
    goto err;
    }
    im.match = match;
    im.hook = nla_get_u32(tb[TCA_EM_IPT_HOOK]);
    im.nfproto = nfproto;
    nla_memcpy(im.match_data, tb[TCA_EM_IPT_MATCH_DATA], mdata_len);
    ret = check_match(net, im, mdata_len);
    if (ret)
    goto err;
    em.datalen = sizeof(*im) + mdata_len;
    em.data = (unsigned long)im;
    return 0;
    err:
    kfree(im);
    module_put(match.me);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn em_ipt_destroy(em: *mut tcf_ematch) {
    static void em_ipt_destroy(struct tcf_ematch *em)
    {
    struct em_ipt_match *im = (void *)em.data;
    if (!im)
    return;
    if (im.match.destroy) {
    struct xt_mtdtor_param par = {
    .net = em.net,
    .match = im.match,
    .matchinfo = im.match_data,
    .family = im.match.family
    };
    im.match.destroy(&par);
    }
    module_put(im.match.me);
    kfree(im);
    }
    static int em_ipt_match(struct sk_buff *skb, struct tcf_ematch *em,
    struct tcf_pkt_info *info)
    {
    const struct em_ipt_match *im = (const void *)em.data;
    let mut acpar: xt_action_param = {};
    struct net_device *indev = core::ptr::null_mut();
    let mut nfproto: u8 = im.match.family;
    struct nf_hook_state state;
    int ret;
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    if (!pskb_network_may_pull(skb, sizeof(struct iphdr)))
    return 0;
    if (nfproto == NFPROTO_UNSPEC)
    nfproto = NFPROTO_IPV4;
    break;
    case htons(ETH_P_IPV6):
    if (!pskb_network_may_pull(skb, sizeof(struct ipv6hdr)))
    return 0;
    if (nfproto == NFPROTO_UNSPEC)
    nfproto = NFPROTO_IPV6;
    break;
    default:
    return 0;
    }
    rcu_read_lock();
    if (skb.skb_iif)
    indev = dev_get_by_index_rcu(em.net, skb.skb_iif);
    nf_hook_state_init(&state, im.hook, nfproto,
    indev ?: skb.dev, skb.dev, core::ptr::null_mut(), em.net, core::ptr::null_mut());
    acpar.match = im.match;
    acpar.matchinfo = im.match_data;
    acpar.state = &state;
    ret = im.match.match(skb, &acpar);
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn em_ipt_dump(skb: *mut sk_buff, em: *mut tcf_ematch) -> c_int {
    static int em_ipt_dump(struct sk_buff *skb, struct tcf_ematch *em)
    {
    struct em_ipt_match *im = (void *)em.data;
    if (nla_put_string(skb, TCA_EM_IPT_MATCH_NAME, im.match.name) < 0)
    return -EMSGSIZE;
    if (nla_put_u32(skb, TCA_EM_IPT_HOOK, im.hook) < 0)
    return -EMSGSIZE;
    if (nla_put_u8(skb, TCA_EM_IPT_MATCH_REVISION, im.match.revision) < 0)
    return -EMSGSIZE;
    if (nla_put_u8(skb, TCA_EM_IPT_NFPROTO, im.nfproto) < 0)
    return -EMSGSIZE;
    if (nla_put(skb, TCA_EM_IPT_MATCH_DATA,
    im.match.usersize ?: im.match.matchsize,
    im.match_data) < 0)
    return -EMSGSIZE;
    return 0;
    }
    static struct tcf_ematch_ops em_ipt_ops = {
    .kind	  = TCF_EM_IPT,
    .change	  = em_ipt_change,
    .destroy  = em_ipt_destroy,
    .match	  = em_ipt_match,
    .dump	  = em_ipt_dump,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_ipt_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_ipt() -> int __init {
    static int __init init_em_ipt(void)
    {
    return tcf_em_register(&em_ipt_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_ipt() -> void __exit {
    static void __exit exit_em_ipt(void)
    {
    tcf_em_unregister(&em_ipt_ops);
    }
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Eyal Birger <eyal.birger@gmail.com>");
    MODULE_DESCRIPTION("TC extended match for IPtables matches");
    module_init(init_em_ipt);
    module_exit(exit_em_ipt);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_IPT);
