//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_CT.c
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
// Copyright (c) 2010 Patrick McHardy <kaber@trash.net>
//

#[no_mangle]
pub unsafe extern "C" fn xt_ct_target(skb: *mut sk_buff, ct: *mut nf_conn) -> c_int {
    static inline int xt_ct_target(struct sk_buff *skb, struct nf_conn *ct)
    {
// Previously seen (loopback)? Ignore.
    if (skb._nfct != 0)
    return XT_CONTINUE;
    if (ct) {
    refcount_inc(&ct.ct_general.use);
    nf_ct_set(skb, ct, IP_CT_NEW);
    } else {
    nf_ct_set(skb, ct, IP_CT_UNTRACKED);
    }
    return XT_CONTINUE;
    }
    static unsigned int xt_ct_target_v0(struct sk_buff *skb,
    const struct xt_action_param *par)
    {
    const struct xt_ct_target_info *info = par.targinfo;
    struct nf_conn *ct = info.ct;
    return xt_ct_target(skb, ct);
    }
    static unsigned int xt_ct_target_v1(struct sk_buff *skb,
    const struct xt_action_param *par)
    {
    const struct xt_ct_target_info_v1 *info = par.targinfo;
    struct nf_conn *ct = info.ct;
    return xt_ct_target(skb, ct);
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_find_proto(par: *const xt_tgchk_param) -> u8 {
    static u8 xt_ct_find_proto(const struct xt_tgchk_param *par)
    {
    if (par.family == NFPROTO_IPV4) {
    const struct ipt_entry *e = par.entryinfo;
    if (e.ip.invflags & IPT_INV_PROTO)
    return 0;
    return e.ip.proto;
    } else if (par.family == NFPROTO_IPV6) {
    const struct ip6t_entry *e = par.entryinfo;
    if (e.ipv6.invflags & IP6T_INV_PROTO)
    return 0;
    return e.ipv6.proto;
    } else
    return 0;
    }
    static int
    xt_ct_set_helper(struct nf_conn *ct, const char *helper_name,
    const struct xt_tgchk_param *par)
    {
    struct nf_conntrack_helper *helper;
    struct nf_conn_help *help;
    u8 proto;
    proto = xt_ct_find_proto(par);
    if (!proto) {
    pr_info_ratelimited("You must specify a L4 protocol and not use inversions on it\n");
    return -ENOENT;
    }
    helper = nf_conntrack_helper_try_module_get(helper_name, par.family,
    proto);
    if (helper == core::ptr::null_mut()) {
    pr_info_ratelimited("No such helper \"%s\"\n", helper_name);
    return -ENOENT;
    }
    help = nf_ct_helper_ext_add(ct, GFP_KERNEL);
    if (help == core::ptr::null_mut()) {
    nf_conntrack_helper_put(helper);
    return -ENOMEM;
    }
    rcu_assign_pointer(help.helper, helper);
    return 0;
    }
    static int
    xt_ct_set_timeout(struct nf_conn *ct, const struct xt_tgchk_param *par,
    const char *timeout_name)
    {

    const struct nf_conntrack_l4proto *l4proto;
    u8 proto;
    proto = xt_ct_find_proto(par);
    if (!proto) {
    pr_info_ratelimited("You must specify a L4 protocol and not "
    "use inversions on it");
    return -EINVAL;
    }
    l4proto = nf_ct_l4proto_find(proto);
    return nf_ct_set_timeout(par.net, ct, par.family, l4proto.l4proto,
    timeout_name);

    return -EOPNOTSUPP;

    }
#[no_mangle]
unsafe extern "C" fn xt_ct_flags_to_dir(info: *const xt_ct_target_info_v1) -> u16 {
    static u16 xt_ct_flags_to_dir(const struct xt_ct_target_info_v1 *info)
    {
    switch (info.flags & (XT_CT_ZONE_DIR_ORIG |
    XT_CT_ZONE_DIR_REPL)) {
    case XT_CT_ZONE_DIR_ORIG:
    return NF_CT_ZONE_DIR_ORIG;
    case XT_CT_ZONE_DIR_REPL:
    return NF_CT_ZONE_DIR_REPL;
    default:
    return NF_CT_DEFAULT_ZONE_DIR;
    }
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_put_helper(help: *mut nf_conn_help) {
    static void xt_ct_put_helper(struct nf_conn_help *help)
    {
    struct nf_conntrack_helper *helper;
    if (!help)
    return;
// not yet exposed to other cpus, or ruleset
// already detached (post-replacement).
//
    helper = rcu_dereference_raw(help.helper);
    if (helper)
    nf_conntrack_helper_put(helper);
    }
    static int xt_ct_tg_check(const struct xt_tgchk_param *par,
    struct xt_ct_target_info_v1 *info)
    {
    struct nf_conntrack_zone zone;
    struct nf_conn_help *help;
    struct nf_conn *ct;
    let mut ret: c_int = -EOPNOTSUPP;
    if (info.flags & XT_CT_NOTRACK) {
    ct = core::ptr::null_mut();
    goto out;
    }

    if (info.zone || info.flags & (XT_CT_ZONE_DIR_ORIG |
    XT_CT_ZONE_DIR_REPL |
    XT_CT_ZONE_MARK))
    goto err1;

    ret = nf_ct_netns_get(par.net, par.family);
    if (ret < 0)
    goto err1;
    memset(&zone, 0, sizeof(zone));
    zone.id = info.zone;
    zone.dir = xt_ct_flags_to_dir(info);
    if (info.flags & XT_CT_ZONE_MARK)
    zone.flags |= NF_CT_FLAG_MARK;
    ct = nf_ct_tmpl_alloc(par.net, &zone, GFP_KERNEL);
    if (!ct) {
    ret = -ENOMEM;
    goto err2;
    }
    if ((info.ct_events || info.exp_events) &&
    !nf_ct_ecache_ext_add(ct, info.ct_events, info.exp_events,
    GFP_KERNEL)) {
    ret = -EINVAL;
    goto err3;
    }
    if (info.helper[0]) {
    if (strnlen(info.helper, sizeof(info.helper)) == sizeof(info.helper)) {
    ret = -ENAMETOOLONG;
    goto err3;
    }
    ret = xt_ct_set_helper(ct, info.helper, par);
    if (ret < 0)
    goto err3;
    }
    if (info.timeout[0]) {
    if (strnlen(info.timeout, sizeof(info.timeout)) == sizeof(info.timeout)) {
    ret = -ENAMETOOLONG;
    goto err4;
    }
    ret = xt_ct_set_timeout(ct, par, info.timeout);
    if (ret < 0)
    goto err4;
    }
    __set_bit(IPS_CONFIRMED_BIT, &ct.status);
    out:
    info.ct = ct;
    return 0;
    err4:
    help = nfct_help(ct);
    xt_ct_put_helper(help);
    err3:
    nf_ct_tmpl_free(ct);
    err2:
    nf_ct_netns_put(par.net, par.family);
    err1:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_check_v0(par: *const xt_tgchk_param) -> c_int {
    static int xt_ct_tg_check_v0(const struct xt_tgchk_param *par)
    {
    struct xt_ct_target_info *info = par.targinfo;
    struct xt_ct_target_info_v1 info_v1 = {
    .flags 		= info.flags,
    .zone		= info.zone,
    .ct_events	= info.ct_events,
    .exp_events	= info.exp_events,
    };
    int ret;
    if (info.flags & ~XT_CT_NOTRACK)
    return -EINVAL;
    memcpy(info_v1.helper, info.helper, sizeof(info.helper));
    ret = xt_ct_tg_check(par, &info_v1);
    if (ret < 0)
    return ret;
    info.ct = info_v1.ct;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_check_v1(par: *const xt_tgchk_param) -> c_int {
    static int xt_ct_tg_check_v1(const struct xt_tgchk_param *par)
    {
    struct xt_ct_target_info_v1 *info = par.targinfo;
    if (info.flags & ~XT_CT_NOTRACK)
    return -EINVAL;
    return xt_ct_tg_check(par, par.targinfo);
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_check_v2(par: *const xt_tgchk_param) -> c_int {
    static int xt_ct_tg_check_v2(const struct xt_tgchk_param *par)
    {
    struct xt_ct_target_info_v1 *info = par.targinfo;
    if (info.flags & ~XT_CT_MASK)
    return -EINVAL;
    return xt_ct_tg_check(par, par.targinfo);
    }
    static void xt_ct_tg_destroy(const struct xt_tgdtor_param *par,
    struct xt_ct_target_info_v1 *info)
    {
    struct nf_conn *ct = info.ct;
    struct nf_conn_help *help;
    if (ct) {
    help = nfct_help(ct);
    xt_ct_put_helper(help);
    nf_ct_netns_put(par.net, par.family);
    nf_ct_destroy_timeout(ct);
    nf_ct_put(info.ct);
    }
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_destroy_v0(par: *const xt_tgdtor_param) {
    static void xt_ct_tg_destroy_v0(const struct xt_tgdtor_param *par)
    {
    struct xt_ct_target_info *info = par.targinfo;
    struct xt_ct_target_info_v1 info_v1 = {
    .flags 		= info.flags,
    .zone		= info.zone,
    .ct_events	= info.ct_events,
    .exp_events	= info.exp_events,
    .ct		= info.ct,
    };
    memcpy(info_v1.helper, info.helper, sizeof(info.helper));
    xt_ct_tg_destroy(par, &info_v1);
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_destroy_v1(par: *const xt_tgdtor_param) {
    static void xt_ct_tg_destroy_v1(const struct xt_tgdtor_param *par)
    {
    xt_ct_tg_destroy(par, par.targinfo);
    }
    static unsigned int
    notrack_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
// Previously seen (loopback)? Ignore.
    if (skb._nfct != 0)
    return XT_CONTINUE;
    nf_ct_set(skb, core::ptr::null_mut(), IP_CT_UNTRACKED);
    return XT_CONTINUE;
    }
    static struct xt_target xt_ct_tg_reg[] __read_mostly = {
    {
    .name		= "NOTRACK",
    .revision	= 0,
    .family		= NFPROTO_IPV4,
    .target		= notrack_tg,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV4,
    .targetsize	= sizeof(struct xt_ct_target_info),
    .usersize	= offsetof(struct xt_ct_target_info, ct),
    .checkentry	= xt_ct_tg_check_v0,
    .destroy	= xt_ct_tg_destroy_v0,
    .target		= xt_ct_target_v0,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV4,
    .revision	= 1,
    .targetsize	= sizeof(struct xt_ct_target_info_v1),
    .usersize	= offsetof(struct xt_ct_target_info_v1, ct),
    .checkentry	= xt_ct_tg_check_v1,
    .destroy	= xt_ct_tg_destroy_v1,
    .target		= xt_ct_target_v1,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV4,
    .revision	= 2,
    .targetsize	= sizeof(struct xt_ct_target_info_v1),
    .usersize	= offsetof(struct xt_ct_target_info_v1, ct),
    .checkentry	= xt_ct_tg_check_v2,
    .destroy	= xt_ct_tg_destroy_v1,
    .target		= xt_ct_target_v1,
    .table		= "raw",
    .me		= THIS_MODULE,
    },

    {
    .name		= "NOTRACK",
    .revision	= 0,
    .family		= NFPROTO_IPV6,
    .target		= notrack_tg,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV6,
    .targetsize	= sizeof(struct xt_ct_target_info),
    .usersize	= offsetof(struct xt_ct_target_info, ct),
    .checkentry	= xt_ct_tg_check_v0,
    .destroy	= xt_ct_tg_destroy_v0,
    .target		= xt_ct_target_v0,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV6,
    .revision	= 1,
    .targetsize	= sizeof(struct xt_ct_target_info_v1),
    .usersize	= offsetof(struct xt_ct_target_info_v1, ct),
    .checkentry	= xt_ct_tg_check_v1,
    .destroy	= xt_ct_tg_destroy_v1,
    .target		= xt_ct_target_v1,
    .table		= "raw",
    .me		= THIS_MODULE,
    },
    {
    .name		= "CT",
    .family		= NFPROTO_IPV6,
    .revision	= 2,
    .targetsize	= sizeof(struct xt_ct_target_info_v1),
    .usersize	= offsetof(struct xt_ct_target_info_v1, ct),
    .checkentry	= xt_ct_tg_check_v2,
    .destroy	= xt_ct_tg_destroy_v1,
    .target		= xt_ct_target_v1,
    .table		= "raw",
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_init() -> int __init {
    static int __init xt_ct_tg_init(void)
    {
    return xt_register_targets(xt_ct_tg_reg, ARRAY_SIZE(xt_ct_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn xt_ct_tg_exit() -> void __exit {
    static void __exit xt_ct_tg_exit(void)
    {
    xt_unregister_targets(xt_ct_tg_reg, ARRAY_SIZE(xt_ct_tg_reg));
    }
    module_init(xt_ct_tg_init);
    module_exit(xt_ct_tg_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: connection tracking target");
    MODULE_ALIAS("ipt_CT");
    MODULE_ALIAS("ip6t_CT");
    MODULE_ALIAS("ipt_NOTRACK");
    MODULE_ALIAS("ip6t_NOTRACK");
