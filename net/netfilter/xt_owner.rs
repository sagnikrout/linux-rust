//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_owner.c
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
// Kernel module to match various things tied to sockets associated with
// locally generated outgoing packets.
//
// (C) 2000 Marc Boucher <marc@mbsi.ca>
//
// Copyright © CC Computer Consultants GmbH, 2007 - 2008
//

#[no_mangle]
unsafe extern "C" fn owner_check(par: *const xt_mtchk_param) -> c_int {
    static int owner_check(const struct xt_mtchk_param *par)
    {
    struct xt_owner_match_info *info = par.matchinfo;
    struct net *net = par.net;
    if (info.match & ~XT_OWNER_MASK)
    return -EINVAL;
// Only allow the common case where the userns of the writer
// matches the userns of the network namespace.
//
    if ((info.match & (XT_OWNER_UID|XT_OWNER_GID)) &&
    (current_user_ns() != net.user_ns))
    return -EINVAL;
// Ensure the uids are valid
    if (info.match & XT_OWNER_UID) {
    let mut uid_min: kuid_t = make_kuid(net.user_ns, info.uid_min);
    let mut uid_max: kuid_t = make_kuid(net.user_ns, info.uid_max);
    if (!uid_valid(uid_min) || !uid_valid(uid_max) ||
    (info.uid_max < info.uid_min) ||
    uid_lt(uid_max, uid_min)) {
    return -EINVAL;
    }
    }
// Ensure the gids are valid
    if (info.match & XT_OWNER_GID) {
    let mut gid_min: kgid_t = make_kgid(net.user_ns, info.gid_min);
    let mut gid_max: kgid_t = make_kgid(net.user_ns, info.gid_max);
    if (!gid_valid(gid_min) || !gid_valid(gid_max) ||
    (info.gid_max < info.gid_min) ||
    gid_lt(gid_max, gid_min)) {
    return -EINVAL;
    }
    }
    return 0;
    }
    static bool
    owner_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_owner_match_info *info = par.matchinfo;
    struct sock *sk = skb_to_full_sk(skb);
    struct net *net = xt_net(par);
    const struct socket *sock;
    const struct file *filp;
    if (!sk || !READ_ONCE(sk.sk_socket) || !net_eq(net, sock_net(sk)))
    return (info.match ^ info.invert) == 0;
#[no_mangle]
pub unsafe extern "C" fn if(XT_OWNER_SOCKET: info->match & info->invert &) -> else {
    else if (info.match & info.invert & XT_OWNER_SOCKET)
//
// Socket exists but user wanted ! --socket-exists.
// (Single ampersands intended.)
//
    return false;
// The sk pointer remains valid as long as the skb is. The sk_socket and
// file pointer may become NULL if the socket is closed. Both structures
// (including file->cred) are RCU freed which means they can be accessed
// within a RCU read section.
//
    sock = READ_ONCE(sk.sk_socket);
    filp = sock ? READ_ONCE(sock.file) : core::ptr::null_mut();
    if (filp == core::ptr::null_mut())
    return ((info.match ^ info.invert) &
    (XT_OWNER_UID | XT_OWNER_GID)) == 0;
    if (info.match & XT_OWNER_UID) {
    let mut uid_min: kuid_t = make_kuid(net.user_ns, info.uid_min);
    let mut uid_max: kuid_t = make_kuid(net.user_ns, info.uid_max);
    if ((uid_gte(filp.f_cred.fsuid, uid_min) &&
    uid_lte(filp.f_cred.fsuid, uid_max)) ^
    !(info.invert & XT_OWNER_UID))
    return false;
    }
    if (info.match & XT_OWNER_GID) {
    unsigned int i, match = false;
    let mut gid_min: kgid_t = make_kgid(net.user_ns, info.gid_min);
    let mut gid_max: kgid_t = make_kgid(net.user_ns, info.gid_max);
    struct group_info *gi = filp.f_cred.group_info;
    if (gid_gte(filp.f_cred.fsgid, gid_min) &&
    gid_lte(filp.f_cred.fsgid, gid_max))
    match = true;
    if (!match && (info.match & XT_OWNER_SUPPL_GROUPS) && gi) {
    for (i = 0; i < gi.ngroups; ++i) {
    let mut group: kgid_t = gi.gid[i];
    if (gid_gte(group, gid_min) &&
    gid_lte(group, gid_max)) {
    match = true;
    break;
    }
    }
    }
    if (match ^ !(info.invert & XT_OWNER_GID))
    return false;
    }
    return true;
    }
    static struct xt_match owner_mt_reg[] __read_mostly = {
    {
    .name       = "owner",
    .revision   = 1,
    .family     = NFPROTO_IPV4,
    .checkentry = owner_check,
    .match      = owner_mt,
    .matchsize  = sizeof(struct xt_owner_match_info),
    .hooks      = (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING),
    .me         = THIS_MODULE,
    },
    {
    .name       = "owner",
    .revision   = 1,
    .family     = NFPROTO_IPV6,
    .checkentry = owner_check,
    .match      = owner_mt,
    .matchsize  = sizeof(struct xt_owner_match_info),
    .hooks      = (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING),
    .me         = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn owner_mt_init() -> int __init {
    static int __init owner_mt_init(void)
    {
    return xt_register_matches(owner_mt_reg, ARRAY_SIZE(owner_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn owner_mt_exit() -> void __exit {
    static void __exit owner_mt_exit(void)
    {
    xt_unregister_matches(owner_mt_reg, ARRAY_SIZE(owner_mt_reg));
    }
    module_init(owner_mt_init);
    module_exit(owner_mt_exit);
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("Xtables: socket owner matching");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_owner");
    MODULE_ALIAS("ip6t_owner");
