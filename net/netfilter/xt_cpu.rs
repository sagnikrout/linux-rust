//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_cpu.c
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
// Kernel module to match running CPU
//
// Might be used to distribute connections on several daemons, if
// RPS (Remote Packet Steering) is enabled or NIC is multiqueue capable,
// each RX queue IRQ affined to one CPU (1:1 mapping)
//
// (C) 2010 Eric Dumazet
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Eric Dumazet <eric.dumazet@gmail.com>");
    MODULE_DESCRIPTION("Xtables: CPU match");
    MODULE_ALIAS("ipt_cpu");
    MODULE_ALIAS("ip6t_cpu");
#[no_mangle]
unsafe extern "C" fn cpu_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int cpu_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_cpu_info *info = par.matchinfo;
    if (info.invert & ~1)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool cpu_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_cpu_info *info = par.matchinfo;
    return (info.cpu == raw_smp_processor_id()) ^ info.invert;
    }
    static struct xt_match cpu_mt_reg __read_mostly = {
    .name       = "cpu",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .checkentry = cpu_mt_check,
    .match      = cpu_mt,
    .matchsize  = sizeof(struct xt_cpu_info),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn cpu_mt_init() -> int __init {
    static int __init cpu_mt_init(void)
    {
    return xt_register_match(&cpu_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn cpu_mt_exit() -> void __exit {
    static void __exit cpu_mt_exit(void)
    {
    xt_unregister_match(&cpu_mt_reg);
    }
    module_init(cpu_mt_init);
    module_exit(cpu_mt_exit);
