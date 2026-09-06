//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_cgroup.c
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
// Xtables module to match the process control group.
//
// Might be used to implement individual "per-application" firewall
// policies in contrast to global policies based on control groups.
// Matching is based upon processes tagged to net_cls' classid marker.
//
// (C) 2013 Daniel Borkmann <dborkman@redhat.com>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Daniel Borkmann <dborkman@redhat.com>");
    MODULE_DESCRIPTION("Xtables: process control group matching");
    MODULE_ALIAS("ipt_cgroup");
    MODULE_ALIAS("ip6t_cgroup");

#[no_mangle]
unsafe extern "C" fn cgroup_mt_check_v0(par: *const xt_mtchk_param) -> c_int {
    static int cgroup_mt_check_v0(const struct xt_mtchk_param *par)
    {
    struct xt_cgroup_info_v0 *info = par.matchinfo;
    if (info.invert & ~1)
    return -EINVAL;
    if (!IS_ENABLED(CONFIG_CGROUP_NET_CLASSID)) {
    pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_check_v1(par: *const xt_mtchk_param) -> c_int {
    static int cgroup_mt_check_v1(const struct xt_mtchk_param *par)
    {
    struct xt_cgroup_info_v1 *info = par.matchinfo;
    struct cgroup *cgrp;
    if ((info.invert_path & ~1) || (info.invert_classid & ~1))
    return -EINVAL;
    if (!info.has_path && !info.has_classid) {
    pr_info_ratelimited("no path or classid specified\n");
    return -EINVAL;
    }
    if (info.has_path && info.has_classid) {
    pr_info_ratelimited("path and classid specified\n");
    return -EINVAL;
    }
    if (info.has_classid && !IS_ENABLED(CONFIG_CGROUP_NET_CLASSID)) {
    pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG);
    return -EINVAL;
    }
    info.priv = core::ptr::null_mut();
    if (info.has_path) {
    if (strnlen(info.path, sizeof(info.path)) >= sizeof(info.path))
    return -ENAMETOOLONG;
    cgrp = cgroup_get_from_path(info.path);
    if (IS_ERR(cgrp)) {
    pr_info_ratelimited("invalid path, errno=%ld\n",
    PTR_ERR(cgrp));
    return -EINVAL;
    }
    info.priv = cgrp;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_check_v2(par: *const xt_mtchk_param) -> c_int {
    static int cgroup_mt_check_v2(const struct xt_mtchk_param *par)
    {
    struct xt_cgroup_info_v2 *info = par.matchinfo;
    struct cgroup *cgrp;
    if ((info.invert_path & ~1) || (info.invert_classid & ~1))
    return -EINVAL;
    if (!info.has_path && !info.has_classid) {
    pr_info_ratelimited("no path or classid specified\n");
    return -EINVAL;
    }
    if (info.has_path && info.has_classid) {
    pr_info_ratelimited("path and classid specified\n");
    return -EINVAL;
    }
    if (info.has_classid && !IS_ENABLED(CONFIG_CGROUP_NET_CLASSID)) {
    pr_info_ratelimited(NET_CLS_CLASSID_INVALID_MSG);
    return -EINVAL;
    }
    info.priv = core::ptr::null_mut();
    if (info.has_path) {
    if (strnlen(info.path, sizeof(info.path)) >= sizeof(info.path))
    return -ENAMETOOLONG;
    cgrp = cgroup_get_from_path(info.path);
    if (IS_ERR(cgrp)) {
    pr_info_ratelimited("invalid path, errno=%ld\n",
    PTR_ERR(cgrp));
    return -EINVAL;
    }
    info.priv = cgrp;
    }
    return 0;
    }
    static bool
    cgroup_mt_v0(const struct sk_buff *skb, struct xt_action_param *par)
    {

    const struct xt_cgroup_info_v0 *info = par.matchinfo;
    struct sock *sk = skb.sk;
    if (!sk || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)))
    return false;
    return (info.id == sock_cgroup_classid(&skb.sk.sk_cgrp_data)) ^
    info.invert;

    return false;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool cgroup_mt_v1(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_cgroup_info_v1 *info = par.matchinfo;
    struct sock_cgroup_data *skcd = &skb.sk.sk_cgrp_data;
    struct cgroup *ancestor = info.priv;
    struct sock *sk = skb.sk;
    if (!sk || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)))
    return false;
    if (ancestor)
    return cgroup_is_descendant(sock_cgroup_ptr(skcd), ancestor) ^
    info.invert_path;

    else
    return (info.classid == sock_cgroup_classid(skcd)) ^
    info.invert_classid;

    return false;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_v2(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool cgroup_mt_v2(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_cgroup_info_v2 *info = par.matchinfo;
    struct sock_cgroup_data *skcd = &skb.sk.sk_cgrp_data;
    struct cgroup *ancestor = info.priv;
    struct sock *sk = skb.sk;
    if (!sk || !sk_fullsock(sk) || !net_eq(xt_net(par), sock_net(sk)))
    return false;
    if (ancestor)
    return cgroup_is_descendant(sock_cgroup_ptr(skcd), ancestor) ^
    info.invert_path;

    else
    return (info.classid == sock_cgroup_classid(skcd)) ^
    info.invert_classid;

    return false;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_destroy_v1(par: *const xt_mtdtor_param) {
    static void cgroup_mt_destroy_v1(const struct xt_mtdtor_param *par)
    {
    struct xt_cgroup_info_v1 *info = par.matchinfo;
    if (info.priv)
    cgroup_put(info.priv);
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_destroy_v2(par: *const xt_mtdtor_param) {
    static void cgroup_mt_destroy_v2(const struct xt_mtdtor_param *par)
    {
    struct xt_cgroup_info_v2 *info = par.matchinfo;
    if (info.priv)
    cgroup_put(info.priv);
    }
    static struct xt_match cgroup_mt_reg[] __read_mostly = {
    {
    .name		= "cgroup",
    .revision	= 0,
    .family		= NFPROTO_UNSPEC,
    .checkentry	= cgroup_mt_check_v0,
    .match		= cgroup_mt_v0,
    .matchsize	= sizeof(struct xt_cgroup_info_v0),
    .me		= THIS_MODULE,
    .hooks		= (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    },
    {
    .name		= "cgroup",
    .revision	= 1,
    .family		= NFPROTO_UNSPEC,
    .checkentry	= cgroup_mt_check_v1,
    .match		= cgroup_mt_v1,
    .matchsize	= sizeof(struct xt_cgroup_info_v1),
    .usersize	= offsetof(struct xt_cgroup_info_v1, priv),
    .destroy	= cgroup_mt_destroy_v1,
    .me		= THIS_MODULE,
    .hooks		= (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    },
    {
    .name		= "cgroup",
    .revision	= 2,
    .family		= NFPROTO_UNSPEC,
    .checkentry	= cgroup_mt_check_v2,
    .match		= cgroup_mt_v2,
    .matchsize	= sizeof(struct xt_cgroup_info_v2),
    .usersize	= offsetof(struct xt_cgroup_info_v2, priv),
    .destroy	= cgroup_mt_destroy_v2,
    .me		= THIS_MODULE,
    .hooks		= (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    },
    };
#[no_mangle]
unsafe extern "C" fn cgroup_mt_init() -> int __init {
    static int __init cgroup_mt_init(void)
    {
    return xt_register_matches(cgroup_mt_reg, ARRAY_SIZE(cgroup_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mt_exit() -> void __exit {
    static void __exit cgroup_mt_exit(void)
    {
    xt_unregister_matches(cgroup_mt_reg, ARRAY_SIZE(cgroup_mt_reg));
    }
    module_init(cgroup_mt_init);
    module_exit(cgroup_mt_exit);
