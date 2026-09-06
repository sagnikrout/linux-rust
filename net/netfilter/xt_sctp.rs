//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_sctp.c
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

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kiran Kumar Immidi");
    MODULE_DESCRIPTION("Xtables: SCTP protocol packet match");
    MODULE_ALIAS("ipt_sctp");
    MODULE_ALIAS("ip6t_sctp");

    || (!!((invflag) & (option)) ^ (cond)))
    static bool
    match_flags(const struct xt_sctp_flag_info *flag_info,
    const int flag_count,
    u_int8_t chunktype,
    u_int8_t chunkflags)
    {
    int i;
    for (i = 0; i < flag_count; i++)
    if (flag_info[i].chunktype == chunktype)
    return (chunkflags & flag_info[i].flag_mask) == flag_info[i].flag;
    return true;
    }
    static inline bool
    match_packet(const struct sk_buff *skb,
    unsigned int offset,
    const struct xt_sctp_info *info,
    bool *hotdrop)
    {
    u_int32_t chunkmapcopy[256 / sizeof (u_int32_t)];
    const struct sctp_chunkhdr *sch;
    struct sctp_chunkhdr _sch;
    let mut chunk_match_type: c_int = info.chunk_match_type;
    const struct xt_sctp_flag_info *flag_info = info.flag_info;
    let mut flag_count: c_int = info.flag_count;
    if (chunk_match_type == SCTP_CHUNK_MATCH_ALL)
    SCTP_CHUNKMAP_COPY(chunkmapcopy, info.chunkmap);
    do {
    sch = skb_header_pointer(skb, offset, sizeof(_sch), &_sch);
    if (sch == core::ptr::null_mut() || sch.length == 0) {
// hotdrop = true;
    return false;
    }
    offset += SCTP_PAD4(ntohs(sch.length));
    if (SCTP_CHUNKMAP_IS_SET(info.chunkmap, sch.type)) {
    switch (chunk_match_type) {
    case SCTP_CHUNK_MATCH_ANY:
    if (match_flags(flag_info, flag_count,
    sch.type, sch.flags)) {
    return true;
    }
    break;
    case SCTP_CHUNK_MATCH_ALL:
    if (match_flags(flag_info, flag_count,
    sch.type, sch.flags))
    SCTP_CHUNKMAP_CLEAR(chunkmapcopy, sch.type);
    break;
    case SCTP_CHUNK_MATCH_ONLY:
    if (!match_flags(flag_info, flag_count,
    sch.type, sch.flags))
    return false;
    break;
    }
    } else {
    switch (chunk_match_type) {
    case SCTP_CHUNK_MATCH_ONLY:
    return false;
    }
    }
    } while (offset < skb.len);
    switch (chunk_match_type) {
    case SCTP_CHUNK_MATCH_ALL:
    return SCTP_CHUNKMAP_IS_CLEAR(chunkmapcopy);
    case SCTP_CHUNK_MATCH_ANY:
    return false;
    case SCTP_CHUNK_MATCH_ONLY:
    return true;
    }
// This will never be reached, but required to stop compiler whine
    return false;
    }
    static bool
    sctp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_sctp_info *info = par.matchinfo;
    const struct sctphdr *sh;
    struct sctphdr _sh;
    if (par.fragoff != 0)
    return false;
    sh = skb_header_pointer(skb, par.thoff, sizeof(_sh), &_sh);
    if (sh == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    return  SCCHECK(ntohs(sh.source) >= info.spts[0]
    && ntohs(sh.source) <= info.spts[1],
    XT_SCTP_SRC_PORTS, info.flags, info.invflags) &&
    SCCHECK(ntohs(sh.dest) >= info.dpts[0]
    && ntohs(sh.dest) <= info.dpts[1],
    XT_SCTP_DEST_PORTS, info.flags, info.invflags) &&
    SCCHECK(match_packet(skb, par.thoff + sizeof(_sh),
    info, &par.hotdrop),
    XT_SCTP_CHUNK_TYPES, info.flags, info.invflags);
    }
#[no_mangle]
unsafe extern "C" fn sctp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int sctp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_sctp_info *info = par.matchinfo;
    if (info.flag_count > ARRAY_SIZE(info.flag_info))
    return -EINVAL;
    if (info.flags & ~XT_SCTP_VALID_FLAGS)
    return -EINVAL;
    if (info.invflags & ~XT_SCTP_VALID_FLAGS)
    return -EINVAL;
    if (info.invflags & ~info.flags)
    return -EINVAL;
    if (!(info.flags & XT_SCTP_CHUNK_TYPES))
    return 0;
    if (info.chunk_match_type & (SCTP_CHUNK_MATCH_ALL |
    SCTP_CHUNK_MATCH_ANY | SCTP_CHUNK_MATCH_ONLY))
    return 0;
    return -EINVAL;
    }
    static struct xt_match sctp_mt_reg[] __read_mostly = {
    {
    .name		= "sctp",
    .family		= NFPROTO_IPV4,
    .checkentry	= sctp_mt_check,
    .match		= sctp_mt,
    .matchsize	= sizeof(struct xt_sctp_info),
    .proto		= IPPROTO_SCTP,
    .me		= THIS_MODULE
    },
    {
    .name		= "sctp",
    .family		= NFPROTO_IPV6,
    .checkentry	= sctp_mt_check,
    .match		= sctp_mt,
    .matchsize	= sizeof(struct xt_sctp_info),
    .proto		= IPPROTO_SCTP,
    .me		= THIS_MODULE
    },
    };
#[no_mangle]
unsafe extern "C" fn sctp_mt_init() -> int __init {
    static int __init sctp_mt_init(void)
    {
    return xt_register_matches(sctp_mt_reg, ARRAY_SIZE(sctp_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn sctp_mt_exit() -> void __exit {
    static void __exit sctp_mt_exit(void)
    {
    xt_unregister_matches(sctp_mt_reg, ARRAY_SIZE(sctp_mt_reg));
    }
    module_init(sctp_mt_init);
    module_exit(sctp_mt_exit);
