//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_string.c
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
// String matching match for iptables
//
// (C) 2005 Pablo Neira Ayuso <pablo@eurodev.net>
//

    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@eurodev.net>");
    MODULE_DESCRIPTION("Xtables: string-based matching");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_string");
    MODULE_ALIAS("ip6t_string");
    MODULE_ALIAS("ebt_string");
    static bool
    string_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_string_info *conf = par.matchinfo;
    bool invert;
    invert = conf.u.v1.flags & XT_STRING_FLAG_INVERT;
    return (skb_find_text((struct sk_buff *)skb, conf.from_offset,
    conf.to_offset, conf.config)
    != UINT_MAX) ^ invert;
    }

#[no_mangle]
unsafe extern "C" fn string_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int string_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_string_info *conf = par.matchinfo;
    struct ts_config *ts_conf;
    let mut flags: c_int = TS_AUTOLOAD;
// Damn, can't handle this case properly with iptables...
    if (conf.from_offset > conf.to_offset)
    return -EINVAL;
    if (conf.algo[XT_STRING_MAX_ALGO_NAME_SIZE - 1] != '\0')
    return -EINVAL;
    if (conf.patlen > XT_STRING_MAX_PATTERN_SIZE)
    return -EINVAL;
    if (conf.u.v1.flags &
    ~(XT_STRING_FLAG_IGNORECASE | XT_STRING_FLAG_INVERT))
    return -EINVAL;
    if (conf.u.v1.flags & XT_STRING_FLAG_IGNORECASE)
    flags |= TS_IGNORECASE;
    ts_conf = textsearch_prepare(conf.algo, conf.pattern, conf.patlen,
    GFP_KERNEL, flags);
    if (IS_ERR(ts_conf))
    return PTR_ERR(ts_conf);
    conf.config = ts_conf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn string_mt_destroy(par: *const xt_mtdtor_param) {
    static void string_mt_destroy(const struct xt_mtdtor_param *par)
    {
    textsearch_destroy(STRING_TEXT_PRIV(par.matchinfo).config);
    }
    static struct xt_match xt_string_mt_reg __read_mostly = {
    .name       = "string",
    .revision   = 1,
    .family     = NFPROTO_UNSPEC,
    .checkentry = string_mt_check,
    .match      = string_mt,
    .destroy    = string_mt_destroy,
    .matchsize  = sizeof(struct xt_string_info),
    .usersize   = offsetof(struct xt_string_info, config),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn string_mt_init() -> int __init {
    static int __init string_mt_init(void)
    {
    return xt_register_match(&xt_string_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn string_mt_exit() -> void __exit {
    static void __exit string_mt_exit(void)
    {
    xt_unregister_match(&xt_string_mt_reg);
    }
    module_init(string_mt_init);
    module_exit(string_mt_exit);
