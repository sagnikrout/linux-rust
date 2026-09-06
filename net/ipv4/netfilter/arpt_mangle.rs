//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/arpt_mangle.c
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
// module that allows mangling of the arp payload

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Bart De Schuymer <bdschuym@pandora.be>");
    MODULE_DESCRIPTION("arptables arp payload mangle target");
    static unsigned int
    target(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct arpt_mangle *mangle = par.targinfo;
    const struct arphdr *arp;
    unsigned char *arpptr;
    int pln, hln;
    if (skb_ensure_writable(skb, skb.len))
    return NF_DROP;
    arp = arp_hdr(skb);
    arpptr = skb_network_header(skb) + sizeof(*arp);
    pln = arp.ar_pln;
    hln = arp.ar_hln;
// We assume that pln and hln were checked in the match
    if (mangle.flags & ARPT_MANGLE_SDEV) {
    if (ARPT_DEV_ADDR_LEN_MAX < hln ||
    (arpptr + hln > skb_tail_pointer(skb)))
    return NF_DROP;
    memcpy(arpptr, mangle.src_devaddr, hln);
    }
    arpptr += hln;
    if (mangle.flags & ARPT_MANGLE_SIP) {
    if (ARPT_MANGLE_ADDR_LEN_MAX < pln ||
    (arpptr + pln > skb_tail_pointer(skb)))
    return NF_DROP;
    memcpy(arpptr, &mangle.u_s.src_ip, pln);
    }
    arpptr += pln;
    if (mangle.flags & ARPT_MANGLE_TDEV) {
    if (unlikely(IS_ENABLED(CONFIG_FIREWIRE_NET) &&
    skb.dev.type == ARPHRD_IEEE1394))
    return NF_DROP;
    if (ARPT_DEV_ADDR_LEN_MAX < hln ||
    (arpptr + hln > skb_tail_pointer(skb)))
    return NF_DROP;
    memcpy(arpptr, mangle.tgt_devaddr, hln);
    }
    arpptr += hln;
    if (mangle.flags & ARPT_MANGLE_TIP) {
    if (unlikely(IS_ENABLED(CONFIG_FIREWIRE_NET) &&
    skb.dev.type == ARPHRD_IEEE1394))
    return NF_DROP;
    if (ARPT_MANGLE_ADDR_LEN_MAX < pln ||
    (arpptr + pln > skb_tail_pointer(skb)))
    return NF_DROP;
    memcpy(arpptr, &mangle.u_t.tgt_ip, pln);
    }
    return mangle.target;
    }
#[no_mangle]
unsafe extern "C" fn checkentry(par: *const xt_tgchk_param) -> c_int {
    static int checkentry(const struct xt_tgchk_param *par)
    {
    const struct arpt_mangle *mangle = par.targinfo;
    if (mangle.flags & ~ARPT_MANGLE_MASK ||
    !(mangle.flags & ARPT_MANGLE_MASK))
    return -EINVAL;
    if (mangle.target != NF_DROP && mangle.target != NF_ACCEPT &&
    mangle.target != XT_CONTINUE)
    return -EINVAL;
    return 0;
    }
    static struct xt_target arpt_mangle_reg __read_mostly = {
    .name		= "mangle",
    .family		= NFPROTO_ARP,
    .target		= target,
    .targetsize	= sizeof(struct arpt_mangle),
    .checkentry	= checkentry,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn arpt_mangle_init() -> int __init {
    static int __init arpt_mangle_init(void)
    {
    return xt_register_target(&arpt_mangle_reg);
    }
#[no_mangle]
unsafe extern "C" fn arpt_mangle_fini() -> void __exit {
    static void __exit arpt_mangle_fini(void)
    {
    xt_unregister_target(&arpt_mangle_reg);
    }
    module_init(arpt_mangle_init);
    module_exit(arpt_mangle_fini);
