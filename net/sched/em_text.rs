//! Automatically rewritten from C to Rust
//! Source: net/sched/em_text.c
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
// net/sched/em_text.c	Textsearch ematch
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct text_match {
    pub from_offset: u16,
    pub to_offset: u16,
    pub from_layer: u8,
    pub to_layer: u8,
    pub config: *mut ts_config,
}

    static int em_text_match(struct sk_buff *skb, struct tcf_ematch *m,
    struct tcf_pkt_info *info)
    {
    struct text_match *tm = EM_TEXT_PRIV(m);
    unsigned char *ptr;
    int from, to;
    ptr = tcf_get_base_ptr(skb, tm.from_layer);
    if (!ptr)
    return 0;
    from = ptr - skb.data;
    from += tm.from_offset;
    ptr = tcf_get_base_ptr(skb, tm.to_layer);
    if (!ptr)
    return 0;
    to = ptr - skb.data;
    to += tm.to_offset;
    return skb_find_text(skb, from, to, tm.config) != UINT_MAX;
    }
    static int em_text_change(struct net *net, void *data, int len,
    struct tcf_ematch *m)
    {
    struct text_match *tm;
    struct tcf_em_text *conf = data;
    struct ts_config *ts_conf;
    let mut flags: c_int = 0;
    if (len < sizeof(*conf) || len < (sizeof(*conf) + conf.pattern_len))
    return -EINVAL;
    if (conf.from_layer > conf.to_layer)
    return -EINVAL;
    if (conf.from_layer == conf.to_layer &&
    conf.from_offset > conf.to_offset)
    return -EINVAL;
    retry:
    ts_conf = textsearch_prepare(conf.algo, (u8 *) conf + sizeof(*conf),
    conf.pattern_len, GFP_KERNEL, flags);
    if (flags & TS_AUTOLOAD)
    rtnl_lock();
    if (IS_ERR(ts_conf)) {
    if (PTR_ERR(ts_conf) == -ENOENT && !(flags & TS_AUTOLOAD)) {
    rtnl_unlock();
    flags |= TS_AUTOLOAD;
    goto retry;
    } else
    return PTR_ERR(ts_conf);
    } else if (flags & TS_AUTOLOAD) {
    textsearch_destroy(ts_conf);
    return -EAGAIN;
    }
    tm = kmalloc_obj(*tm);
    if (tm == core::ptr::null_mut()) {
    textsearch_destroy(ts_conf);
    return -ENOBUFS;
    }
    tm.from_offset = conf.from_offset;
    tm.to_offset   = conf.to_offset;
    tm.from_layer  = conf.from_layer;
    tm.to_layer    = conf.to_layer;
    tm.config      = ts_conf;
    m.datalen = sizeof(*tm);
    m.data = (unsigned long) tm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_text_destroy(m: *mut tcf_ematch) {
    static void em_text_destroy(struct tcf_ematch *m)
    {
    if (EM_TEXT_PRIV(m) && EM_TEXT_PRIV(m).config) {
    textsearch_destroy(EM_TEXT_PRIV(m).config);
    kfree(EM_TEXT_PRIV(m));
    }
    }
#[no_mangle]
unsafe extern "C" fn em_text_dump(skb: *mut sk_buff, m: *mut tcf_ematch) -> c_int {
    static int em_text_dump(struct sk_buff *skb, struct tcf_ematch *m)
    {
    struct text_match *tm = EM_TEXT_PRIV(m);
    struct tcf_em_text conf;
    strscpy(conf.algo, tm.config.ops.name);
    conf.from_offset = tm.from_offset;
    conf.to_offset = tm.to_offset;
    conf.from_layer = tm.from_layer;
    conf.to_layer = tm.to_layer;
    conf.pattern_len = textsearch_get_pattern_len(tm.config);
    conf.pad = 0;
    if (nla_put_nohdr(skb, sizeof(conf), &conf) < 0)
    goto nla_put_failure;
    if (nla_append(skb, conf.pattern_len,
    textsearch_get_pattern(tm.config)) < 0)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static struct tcf_ematch_ops em_text_ops = {
    .kind	  = TCF_EM_TEXT,
    .change	  = em_text_change,
    .match	  = em_text_match,
    .destroy  = em_text_destroy,
    .dump	  = em_text_dump,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_text_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_text() -> int __init {
    static int __init init_em_text(void)
    {
    return tcf_em_register(&em_text_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_text() -> void __exit {
    static void __exit exit_em_text(void)
    {
    tcf_em_unregister(&em_text_ops);
    }
    MODULE_DESCRIPTION("ematch classifier for embedded text in skbs");
    MODULE_LICENSE("GPL");
    module_init(init_em_text);
    module_exit(exit_em_text);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_TEXT);
