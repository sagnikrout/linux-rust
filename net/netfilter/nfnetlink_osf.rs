//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nfnetlink_osf.c
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
// Indexed by dont-fragment bit.
// It is the only constant value in the fingerprint.
//
    struct list_head nf_osf_fingers[2];
    EXPORT_SYMBOL_GPL(nf_osf_fingers);
    static inline int nf_osf_ttl(const struct sk_buff *skb,
    int ttl_check, unsigned char f_ttl)
    {
    const struct iphdr *ip = ip_hdr(skb);
    switch (ttl_check) {
    case NF_OSF_TTL_TRUE:
    return ip.ttl == f_ttl;
    break;
    case NF_OSF_TTL_NOCHECK:
    return 1;
    case NF_OSF_TTL_LESS:
    default:
    return ip.ttl <= f_ttl;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_hdr_ctx {
    pub df: bool,
    pub window: u16,
    pub totlen: u16,
    pub optp: *const c_uchar,
    pub optsize: c_uint,
}

    static bool nf_osf_match_one(const struct sk_buff *skb,
    const struct nf_osf_user_finger *f,
    int ttl_check,
    const struct nf_osf_hdr_ctx *ctx)
    {
    const __u8 *optp = ctx.optp;
    let mut check_WSS: c_uint = 0;
    let mut fmatch: c_int = FMATCH_WRONG;
    int foptsize, optnum;
    let mut mss: u16 = 0;
    if (ctx.totlen != f.ss || !nf_osf_ttl(skb, ttl_check, f.ttl))
    return false;
//
// Should not happen if userspace parser was written correctly.
//
    if (f.wss.wc >= OSF_WSS_MAX)
    return false;
// Check options
    foptsize = 0;
    for (optnum = 0; optnum < f.opt_num; ++optnum)
    foptsize += f.opt[optnum].length;
    if (foptsize > MAX_IPOPTLEN ||
    ctx.optsize > MAX_IPOPTLEN ||
    ctx.optsize != foptsize)
    return false;
    check_WSS = f.wss.wc;
    for (optnum = 0; optnum < f.opt_num; ++optnum) {
    if (f.opt[optnum].kind == *optp) {
    let mut len: __u32 = f.opt[optnum].length;
    const __u8 *optend = optp + len;
    fmatch = FMATCH_OK;
    switch (*optp) {
    case OSFOPT_MSS:
    mss = get_unaligned_be16(&optp[2]);
    break;
    case OSFOPT_TS:
    break;
    }
    optp = optend;
    } else
    fmatch = FMATCH_OPT_WRONG;
    if (fmatch != FMATCH_OK)
    break;
    }
    if (fmatch != FMATCH_OPT_WRONG) {
    fmatch = FMATCH_WRONG;
    switch (check_WSS) {
    case OSF_WSS_PLAIN:
    if (f.wss.val == 0 || ctx.window == f.wss.val)
    fmatch = FMATCH_OK;
    break;
    case OSF_WSS_MSS:
//
// Some smart modems decrease mangle MSS to
// SMART_MSS_2, so we check standard, decreased
// and the one provided in the fingerprint MSS
// values.
//
pub const SMART_MSS_1: c_int = 1460;
pub const SMART_MSS_2: c_int = 1448;
    if (ctx.window == f.wss.val * mss ||
    ctx.window == f.wss.val * SMART_MSS_1 ||
    ctx.window == f.wss.val * SMART_MSS_2)
    fmatch = FMATCH_OK;
    break;
    case OSF_WSS_MTU:
    if (ctx.window == f.wss.val * (mss + 40) ||
    ctx.window == f.wss.val * (SMART_MSS_1 + 40) ||
    ctx.window == f.wss.val * (SMART_MSS_2 + 40))
    fmatch = FMATCH_OK;
    break;
    case OSF_WSS_MODULO:
    if ((ctx.window % f.wss.val) == 0)
    fmatch = FMATCH_OK;
    break;
    }
    }
    let mut fmatch: return = = FMATCH_OK;
    }
    static const struct tcphdr *nf_osf_hdr_ctx_init(struct nf_osf_hdr_ctx *ctx,
    const struct sk_buff *skb,
    const struct iphdr *ip,
    unsigned char *opts,
    struct tcphdr *_tcph)
    {
    const struct tcphdr *tcp;
    tcp = skb_header_pointer(skb, ip_hdrlen(skb), sizeof(struct tcphdr), _tcph);
    if (!tcp)
    return core::ptr::null_mut();
    if (!tcp.syn)
    return core::ptr::null_mut();
    ctx.totlen = ntohs(ip.tot_len);
    ctx.df = ntohs(ip.frag_off) & IP_DF;
    ctx.window = ntohs(tcp.window);
    if (tcp.doff * 4 > sizeof(struct tcphdr)) {
    ctx.optsize = tcp.doff * 4 - sizeof(struct tcphdr);
    ctx.optp = skb_header_pointer(skb, ip_hdrlen(skb) +
    sizeof(struct tcphdr), ctx.optsize, opts);
    if (!ctx.optp)
    return core::ptr::null_mut();
    }
    return tcp;
    }
    bool
    nf_osf_match(const struct sk_buff *skb, u_int8_t family,
    int hooknum, struct net_device *in, struct net_device *out,
    const struct nf_osf_info *info, struct net *net,
    const struct list_head *nf_osf_fingers)
    {
    const struct iphdr *ip = ip_hdr(skb);
    const struct nf_osf_user_finger *f;
    unsigned char opts[MAX_IPOPTLEN];
    const struct nf_osf_finger *kf;
    let mut fcount: c_int = 0, ttl_check;
    let mut fmatch: c_int = FMATCH_WRONG;
    struct nf_osf_hdr_ctx ctx;
    const struct tcphdr *tcp;
    struct tcphdr _tcph;
    memset(&ctx, 0, sizeof(ctx));
    tcp = nf_osf_hdr_ctx_init(&ctx, skb, ip, opts, &_tcph);
    if (!tcp)
    return false;
    ttl_check = (info.flags & NF_OSF_TTL) ? info.ttl : 0;
    list_for_each_entry_rcu(kf, &nf_osf_fingers[ctx.df], finger_entry) {
    f = &kf.finger;
    if (!(info.flags & NF_OSF_LOG) && strcmp(info.genre, f.genre))
    continue;
    if (!nf_osf_match_one(skb, f, ttl_check, &ctx))
    continue;
    fmatch = FMATCH_OK;
    fcount++;
    if (info.flags & NF_OSF_LOG)
    nf_log_packet(net, family, hooknum, skb,
    in, out, core::ptr::null_mut(),
    "%s [%s:%s] : %pI4:%d . %pI4:%d hops=%d\n",
    f.genre, f.version, f.subtype,
    &ip.saddr, ntohs(tcp.source),
    &ip.daddr, ntohs(tcp.dest),
    f.ttl - ip.ttl);
    if ((info.flags & NF_OSF_LOG) &&
    info.loglevel == NF_OSF_LOGLEVEL_FIRST)
    break;
    }
    if (!fcount && (info.flags & NF_OSF_LOG))
    nf_log_packet(net, family, hooknum, skb, in, out, core::ptr::null_mut(),
    "Remote OS is not known: %pI4:%u . %pI4:%u\n",
    &ip.saddr, ntohs(tcp.source),
    &ip.daddr, ntohs(tcp.dest));
    if (fcount)
    fmatch = FMATCH_OK;
    let mut fmatch: return = = FMATCH_OK;
    }
    EXPORT_SYMBOL_GPL(nf_osf_match);
    bool nf_osf_find(const struct sk_buff *skb,
    const struct list_head *nf_osf_fingers,
    const int ttl_check, struct nf_osf_data *data)
    {
    const struct iphdr *ip = ip_hdr(skb);
    const struct nf_osf_user_finger *f;
    unsigned char opts[MAX_IPOPTLEN];
    const struct nf_osf_finger *kf;
    struct nf_osf_hdr_ctx ctx;
    const struct tcphdr *tcp;
    struct tcphdr _tcph;
    let mut found: bool = false;
    memset(&ctx, 0, sizeof(ctx));
    tcp = nf_osf_hdr_ctx_init(&ctx, skb, ip, opts, &_tcph);
    if (!tcp)
    return false;
    list_for_each_entry_rcu(kf, &nf_osf_fingers[ctx.df], finger_entry) {
    f = &kf.finger;
    if (!nf_osf_match_one(skb, f, ttl_check, &ctx))
    continue;
    data.genre = f.genre;
    data.version = f.version;
    found = true;
    break;
    }
    return found;
    }
    EXPORT_SYMBOL_GPL(nf_osf_find);
    static const struct nla_policy nfnl_osf_policy[OSF_ATTR_MAX + 1] = {
    [OSF_ATTR_FINGER]	= NLA_POLICY_EXACT_LEN(sizeof(struct nf_osf_user_finger)),
    };
    static int nfnl_osf_add_callback(struct sk_buff *skb,
    const struct nfnl_info *info,
    const struct nlattr * const osf_attrs[])
    {
    struct nf_osf_user_finger *f;
    struct nf_osf_finger *kf = core::ptr::null_mut(), *sf;
    let mut tot_opt_len: c_uint = 0;
    let mut err: c_int = 0;
    int i;
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    if (!osf_attrs[OSF_ATTR_FINGER])
    return -EINVAL;
    if (!(info.nlh.nlmsg_flags & NLM_F_CREATE))
    return -EINVAL;
    f = nla_data(osf_attrs[OSF_ATTR_FINGER]);
    if (f.opt_num > ARRAY_SIZE(f.opt))
    return -EINVAL;
    if (f.wss.wc >= OSF_WSS_MAX ||
    (f.wss.wc == OSF_WSS_MODULO && f.wss.val == 0))
    return -EINVAL;
    for (i = 0; i < f.opt_num; i++) {
    if (!f.opt[i].length || f.opt[i].length > MAX_IPOPTLEN)
    return -EINVAL;
    if (f.opt[i].kind == OSFOPT_MSS && f.opt[i].length < 4)
    return -EINVAL;
    tot_opt_len += f.opt[i].length;
    if (tot_opt_len > MAX_IPOPTLEN)
    return -EINVAL;
    }
    if (!memchr(f.genre, 0, MAXGENRELEN) ||
    !memchr(f.subtype, 0, MAXGENRELEN) ||
    !memchr(f.version, 0, MAXGENRELEN))
    return -EINVAL;
    kf = kmalloc_obj(struct nf_osf_finger);
    if (!kf)
    return -ENOMEM;
    memcpy(&kf.finger, f, sizeof(struct nf_osf_user_finger));
    list_for_each_entry(sf, &nf_osf_fingers[!!f.df], finger_entry) {
    if (memcmp(&sf.finger, f, sizeof(struct nf_osf_user_finger)))
    continue;
    kfree(kf);
    kf = core::ptr::null_mut();
    if (info.nlh.nlmsg_flags & NLM_F_EXCL)
    err = -EEXIST;
    break;
    }
//
// We are protected by nfnl mutex.
//
    if (kf)
    list_add_tail_rcu(&kf.finger_entry, &nf_osf_fingers[!!f.df]);
    return err;
    }
    static int nfnl_osf_remove_callback(struct sk_buff *skb,
    const struct nfnl_info *info,
    const struct nlattr * const osf_attrs[])
    {
    struct nf_osf_user_finger *f;
    struct nf_osf_finger *sf;
    let mut err: c_int = -ENOENT;
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    if (!osf_attrs[OSF_ATTR_FINGER])
    return -EINVAL;
    f = nla_data(osf_attrs[OSF_ATTR_FINGER]);
    list_for_each_entry(sf, &nf_osf_fingers[!!f.df], finger_entry) {
    if (memcmp(&sf.finger, f, sizeof(struct nf_osf_user_finger)))
    continue;
//
// We are protected by nfnl mutex.
//
    list_del_rcu(&sf.finger_entry);
    kfree_rcu(sf, rcu_head);
    err = 0;
    break;
    }
    return err;
    }
    static const struct nfnl_callback nfnl_osf_callbacks[OSF_MSG_MAX] = {
    [OSF_MSG_ADD]	= {
    .call		= nfnl_osf_add_callback,
    .type		= NFNL_CB_MUTEX,
    .attr_count	= OSF_ATTR_MAX,
    .policy		= nfnl_osf_policy,
    },
    [OSF_MSG_REMOVE]	= {
    .call		= nfnl_osf_remove_callback,
    .type		= NFNL_CB_MUTEX,
    .attr_count	= OSF_ATTR_MAX,
    .policy		= nfnl_osf_policy,
    },
    };
    static const struct nfnetlink_subsystem nfnl_osf_subsys = {
    .name			= "osf",
    .subsys_id		= NFNL_SUBSYS_OSF,
    .cb_count		= OSF_MSG_MAX,
    .cb			= nfnl_osf_callbacks,
    };
#[no_mangle]
unsafe extern "C" fn nfnl_osf_init() -> int __init {
    static int __init nfnl_osf_init(void)
    {
    let mut err: c_int = -EINVAL;
    int i;
    for (i = 0; i < ARRAY_SIZE(nf_osf_fingers); ++i)
    INIT_LIST_HEAD(&nf_osf_fingers[i]);
    err = nfnetlink_subsys_register(&nfnl_osf_subsys);
    if (err < 0) {
    pr_err("Failed to register OSF nsfnetlink helper (%d)\n", err);
    goto err_out_exit;
    }
    return 0;
    err_out_exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nfnl_osf_fini() -> void __exit {
    static void __exit nfnl_osf_fini(void)
    {
    struct nf_osf_finger *f;
    int i;
    nfnetlink_subsys_unregister(&nfnl_osf_subsys);
    rcu_read_lock();
    for (i = 0; i < ARRAY_SIZE(nf_osf_fingers); ++i) {
    list_for_each_entry_rcu(f, &nf_osf_fingers[i], finger_entry) {
    list_del_rcu(&f.finger_entry);
    kfree_rcu(f, rcu_head);
    }
    }
    rcu_read_unlock();
    rcu_barrier();
    }
    module_init(nfnl_osf_init);
    module_exit(nfnl_osf_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Passive OS fingerprint matching");
    MODULE_ALIAS_NFNL_SUBSYS(NFNL_SUBSYS_OSF);
