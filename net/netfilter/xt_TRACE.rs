//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_TRACE.c
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
// This is a module which is used to mark packets for tracing.
//

    MODULE_DESCRIPTION("Xtables: packet flow tracing");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_TRACE");
    MODULE_ALIAS("ip6t_TRACE");
#[no_mangle]
unsafe extern "C" fn trace_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int trace_tg_check(const struct xt_tgchk_param *par)
    {
    return nf_logger_find_get(par.family, NF_LOG_TYPE_LOG);
    }
#[no_mangle]
unsafe extern "C" fn trace_tg_destroy(par: *const xt_tgdtor_param) {
    static void trace_tg_destroy(const struct xt_tgdtor_param *par)
    {
    nf_logger_put(par.family, NF_LOG_TYPE_LOG);
    }
    static unsigned int
    trace_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    skb.nf_trace = 1;
    return XT_CONTINUE;
    }
    static struct xt_target trace_tg_reg[] __read_mostly = {
    {
    .name		= "TRACE",
    .revision	= 0,
    .family		= NFPROTO_IPV4,
    .table		= "raw",
    .target		= trace_tg,
    .checkentry	= trace_tg_check,
    .destroy	= trace_tg_destroy,
    .me		= THIS_MODULE,
    },

    {
    .name		= "TRACE",
    .revision	= 0,
    .family		= NFPROTO_IPV6,
    .table		= "raw",
    .target		= trace_tg,
    .checkentry	= trace_tg_check,
    .destroy	= trace_tg_destroy,
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn trace_tg_init() -> int __init {
    static int __init trace_tg_init(void)
    {
    return xt_register_targets(trace_tg_reg, ARRAY_SIZE(trace_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn trace_tg_exit() -> void __exit {
    static void __exit trace_tg_exit(void)
    {
    xt_unregister_targets(trace_tg_reg, ARRAY_SIZE(trace_tg_reg));
    }
    module_init(trace_tg_init);
    module_exit(trace_tg_exit);
    MODULE_SOFTDEP("pre: nf_log_syslog");
