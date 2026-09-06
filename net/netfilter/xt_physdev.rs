//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_physdev.c
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
// Kernel module to match the bridge port in and
// out device for IP packets coming into contact with a bridge.
// (C) 2001-2003 Bart De Schuymer <bdschuym@pandora.be>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Bart De Schuymer <bdschuym@pandora.be>");
    MODULE_DESCRIPTION("Xtables: Bridge physical device match");
    MODULE_ALIAS("ipt_physdev");
    MODULE_ALIAS("ip6t_physdev");
    static bool
    physdev_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_physdev_info *info = par.matchinfo;
    const struct net_device *physdev;
    unsigned long ret;
    const char *indev, *outdev;
// Not a bridged IP packet or no info available yet:
// LOCAL_OUT/mangle and LOCAL_OUT/nat don't know if
// the destination device will be a bridge.
    if (!nf_bridge_info_exists(skb)) {
// Return MATCH if the invert flags of the used options are on
    if ((info.bitmask & XT_PHYSDEV_OP_BRIDGED) &&
    !(info.invert & XT_PHYSDEV_OP_BRIDGED))
    return false;
    if ((info.bitmask & XT_PHYSDEV_OP_ISIN) &&
    !(info.invert & XT_PHYSDEV_OP_ISIN))
    return false;
    if ((info.bitmask & XT_PHYSDEV_OP_ISOUT) &&
    !(info.invert & XT_PHYSDEV_OP_ISOUT))
    return false;
    if ((info.bitmask & XT_PHYSDEV_OP_IN) &&
    !(info.invert & XT_PHYSDEV_OP_IN))
    return false;
    if ((info.bitmask & XT_PHYSDEV_OP_OUT) &&
    !(info.invert & XT_PHYSDEV_OP_OUT))
    return false;
    return true;
    }
    physdev = nf_bridge_get_physoutdev(skb);
    outdev = physdev ? physdev.name : core::ptr::null_mut();
// This only makes sense in the FORWARD and POSTROUTING chains
    if ((info.bitmask & XT_PHYSDEV_OP_BRIDGED) &&
    (!!outdev ^ !(info.invert & XT_PHYSDEV_OP_BRIDGED)))
    return false;
    physdev = nf_bridge_get_physindev(skb, xt_net(par));
    indev = physdev ? physdev.name : core::ptr::null_mut();
    if ((info.bitmask & XT_PHYSDEV_OP_ISIN &&
    (!indev ^ !!(info.invert & XT_PHYSDEV_OP_ISIN))) ||
    (info.bitmask & XT_PHYSDEV_OP_ISOUT &&
    (!outdev ^ !!(info.invert & XT_PHYSDEV_OP_ISOUT))))
    return false;
    if (!(info.bitmask & XT_PHYSDEV_OP_IN))
    goto match_outdev;
    if (indev) {
    ret = ifname_compare_aligned(indev, info.physindev,
    info.in_mask);
    if (!ret ^ !(info.invert & XT_PHYSDEV_OP_IN))
    return false;
    }
    match_outdev:
    if (!(info.bitmask & XT_PHYSDEV_OP_OUT))
    return true;
    if (!outdev)
    return false;
    ret = ifname_compare_aligned(outdev, info.physoutdev, info.out_mask);
    return (!!ret ^ !(info.invert & XT_PHYSDEV_OP_OUT));
    }
#[no_mangle]
unsafe extern "C" fn physdev_mt_check_hooks(par: *const xt_mtchk_param) -> c_int {
    static int physdev_mt_check_hooks(const struct xt_mtchk_param *par)
    {
    const struct xt_physdev_info *info = par.matchinfo;
    if (info.bitmask & (XT_PHYSDEV_OP_OUT | XT_PHYSDEV_OP_ISOUT) &&
    (!(info.bitmask & XT_PHYSDEV_OP_BRIDGED) ||
    info.invert & XT_PHYSDEV_OP_BRIDGED) &&
    par.hook_mask & (1 << NF_INET_LOCAL_OUT)) {
    pr_info_ratelimited("--physdev-out and --physdev-is-out only supported in the FORWARD and POSTROUTING chains with bridged traffic\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn physdev_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int physdev_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_physdev_info *info = par.matchinfo;
    static bool brnf_probed __read_mostly;
    if (!(info.bitmask & XT_PHYSDEV_OP_MASK) ||
    info.bitmask & ~XT_PHYSDEV_OP_MASK)
    return -EINVAL;

    if (info.bitmask & XT_PHYSDEV_OP_IN) {
    if (info.physindev[0] == '\0')
    return -EINVAL;
    if (X(physindev))
    return -ENAMETOOLONG;
    }
    if (info.bitmask & XT_PHYSDEV_OP_OUT) {
    if (info.physoutdev[0] == '\0')
    return -EINVAL;
    if (X(physoutdev))
    return -ENAMETOOLONG;
    }

    if (!brnf_probed) {
    brnf_probed = true;
    request_module("br_netfilter");
    }
    return 0;
    }
    static struct xt_match physdev_mt_reg[] __read_mostly = {
    {
    .name		= "physdev",
    .family		= NFPROTO_IPV4,
    .check_hooks	= physdev_mt_check_hooks,
    .checkentry	= physdev_mt_check,
    .match		= physdev_mt,
    .matchsize	= sizeof(struct xt_physdev_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "physdev",
    .family		= NFPROTO_IPV6,
    .check_hooks	= physdev_mt_check_hooks,
    .checkentry	= physdev_mt_check,
    .match		= physdev_mt,
    .matchsize	= sizeof(struct xt_physdev_info),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn physdev_mt_init() -> int __init {
    static int __init physdev_mt_init(void)
    {
    return xt_register_matches(physdev_mt_reg, ARRAY_SIZE(physdev_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn physdev_mt_exit() -> void __exit {
    static void __exit physdev_mt_exit(void)
    {
    xt_unregister_matches(physdev_mt_reg, ARRAY_SIZE(physdev_mt_reg));
    }
    module_init(physdev_mt_init);
    module_exit(physdev_mt_exit);
