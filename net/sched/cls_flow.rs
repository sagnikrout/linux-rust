//! Automatically rewritten from C to Rust
//! Source: net/sched/cls_flow.c
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
// net/sched/cls_flow.c		Generic flow classifier
//
// Copyright (c) 2007, 2008 Patrick McHardy <kaber@trash.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_head {
    pub filters: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_filter {
    pub list: list_head,
    pub exts: tcf_exts,
    pub ematches: tcf_ematch_tree,
    pub tp: *mut tcf_proto,
    pub perturb_timer: timer_list,
    pub perturb_period: u32,
    pub handle: u32,
    pub nkeys: u32,
    pub keymask: u32,
    pub mode: u32,
    pub mask: u32,
    pub xor: u32,
    pub rshift: u32,
    pub addend: u32,
    pub divisor: u32,
    pub baseclass: u32,
    pub hashrnd: u32,
    pub rwork: rcu_work,
}

    static siphash_aligned_key_t flow_keys_secret __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn addr_fold(addr: *mut c_void) -> u32 {
    static inline u32 addr_fold(void *addr)
    {

    return (u32)siphash_1u64((u64)addr, &flow_keys_secret);

    return (u32)siphash_1u32((u32)addr, &flow_keys_secret);

    }
#[no_mangle]
unsafe extern "C" fn flow_get_src(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    static u32 flow_get_src(const struct sk_buff *skb, const struct flow_keys *flow)
    {
    let mut src: __be32 = flow_get_u32_src(flow);
    if (src)
    return ntohl(src);
    return addr_fold(skb.sk);
    }
#[no_mangle]
unsafe extern "C" fn flow_get_dst(skb: *const sk_buff, flow: *const flow_keys) -> u32 {
    static u32 flow_get_dst(const struct sk_buff *skb, const struct flow_keys *flow)
    {
    let mut dst: __be32 = flow_get_u32_dst(flow);
    if (dst)
    return ntohl(dst);
    return addr_fold(skb_dst(skb)) ^ ( u16)skb_protocol(skb, true);
    }
    static u32 flow_get_proto(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    return flow.basic.ip_proto;
    }
    static u32 flow_get_proto_src(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    if (flow.ports.ports)
    return ntohs(flow.ports.src);
    return addr_fold(skb.sk);
    }
    static u32 flow_get_proto_dst(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    if (flow.ports.ports)
    return ntohs(flow.ports.dst);
    return addr_fold(skb_dst(skb)) ^ ( u16)skb_protocol(skb, true);
    }
#[no_mangle]
unsafe extern "C" fn flow_get_iif(skb: *const sk_buff) -> u32 {
    static u32 flow_get_iif(const struct sk_buff *skb)
    {
    return skb.skb_iif;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_priority(skb: *const sk_buff) -> u32 {
    static u32 flow_get_priority(const struct sk_buff *skb)
    {
    return skb.priority;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_mark(skb: *const sk_buff) -> u32 {
    static u32 flow_get_mark(const struct sk_buff *skb)
    {
    return skb.mark;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_nfct(skb: *const sk_buff) -> u32 {
    static u32 flow_get_nfct(const struct sk_buff *skb)
    {

    return addr_fold(skb_nfct(skb));

    return 0;

    }

    ({									\
    enum ip_conntrack_info ctinfo;					\
    const struct nf_conn *ct = nf_ct_get(skb, &ctinfo);		\
    if (ct == core::ptr::null_mut())							\
    goto fallback;						\
    ct.tuplehash[CTINFO2DIR(ctinfo)].tuple.member;			\
    })

    ({									\
    goto fallback;							\
    0;								\
    })

    static u32 flow_get_nfct_src(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    return ntohl(CTTUPLE(skb, src.u3.ip));
    case htons(ETH_P_IPV6):
    return ntohl(CTTUPLE(skb, src.u3.ip6[3]));
    }
    fallback:
    return flow_get_src(skb, flow);
    }
    static u32 flow_get_nfct_dst(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    return ntohl(CTTUPLE(skb, dst.u3.ip));
    case htons(ETH_P_IPV6):
    return ntohl(CTTUPLE(skb, dst.u3.ip6[3]));
    }
    fallback:
    return flow_get_dst(skb, flow);
    }
    static u32 flow_get_nfct_proto_src(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    return ntohs(CTTUPLE(skb, src.u.all));
    fallback:
    return flow_get_proto_src(skb, flow);
    }
    static u32 flow_get_nfct_proto_dst(const struct sk_buff *skb,
    const struct flow_keys *flow)
    {
    return ntohs(CTTUPLE(skb, dst.u.all));
    fallback:
    return flow_get_proto_dst(skb, flow);
    }
#[no_mangle]
unsafe extern "C" fn flow_get_rtclassid(skb: *const sk_buff) -> u32 {
    static u32 flow_get_rtclassid(const struct sk_buff *skb)
    {

    if (skb_dst(skb))
    return skb_dst(skb).tclassid;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_skuid(skb: *const sk_buff) -> u32 {
    static u32 flow_get_skuid(const struct sk_buff *skb)
    {
    struct sock *sk = skb_to_full_sk(skb);
    if (sk && sk.sk_socket && sk.sk_socket.file) {
    let mut skuid: kuid_t = sk.sk_socket.file.f_cred.fsuid;
    return from_kuid(&init_user_ns, skuid);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_skgid(skb: *const sk_buff) -> u32 {
    static u32 flow_get_skgid(const struct sk_buff *skb)
    {
    struct sock *sk = skb_to_full_sk(skb);
    if (sk && sk.sk_socket && sk.sk_socket.file) {
    let mut skgid: kgid_t = sk.sk_socket.file.f_cred.fsgid;
    return from_kgid(&init_user_ns, skgid);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_vlan_tag(skb: *const sk_buff) -> u32 {
    static u32 flow_get_vlan_tag(const struct sk_buff *skb)
    {
    u16 tag;
    if (vlan_get_tag(skb, &tag) < 0)
    return 0;
    return tag & VLAN_VID_MASK;
    }
#[no_mangle]
unsafe extern "C" fn flow_get_rxhash(skb: *mut sk_buff) -> u32 {
    static u32 flow_get_rxhash(struct sk_buff *skb)
    {
    return skb_get_hash(skb);
    }
#[no_mangle]
unsafe extern "C" fn flow_key_get(skb: *mut sk_buff, key: c_int, flow: *mut flow_keys) -> u32 {
    static u32 flow_key_get(struct sk_buff *skb, int key, struct flow_keys *flow)
    {
    switch (key) {
    case FLOW_KEY_SRC:
    return flow_get_src(skb, flow);
    case FLOW_KEY_DST:
    return flow_get_dst(skb, flow);
    case FLOW_KEY_PROTO:
    return flow_get_proto(skb, flow);
    case FLOW_KEY_PROTO_SRC:
    return flow_get_proto_src(skb, flow);
    case FLOW_KEY_PROTO_DST:
    return flow_get_proto_dst(skb, flow);
    case FLOW_KEY_IIF:
    return flow_get_iif(skb);
    case FLOW_KEY_PRIORITY:
    return flow_get_priority(skb);
    case FLOW_KEY_MARK:
    return flow_get_mark(skb);
    case FLOW_KEY_NFCT:
    return flow_get_nfct(skb);
    case FLOW_KEY_NFCT_SRC:
    return flow_get_nfct_src(skb, flow);
    case FLOW_KEY_NFCT_DST:
    return flow_get_nfct_dst(skb, flow);
    case FLOW_KEY_NFCT_PROTO_SRC:
    return flow_get_nfct_proto_src(skb, flow);
    case FLOW_KEY_NFCT_PROTO_DST:
    return flow_get_nfct_proto_dst(skb, flow);
    case FLOW_KEY_RTCLASSID:
    return flow_get_rtclassid(skb);
    case FLOW_KEY_SKUID:
    return flow_get_skuid(skb);
    case FLOW_KEY_SKGID:
    return flow_get_skgid(skb);
    case FLOW_KEY_VLAN_TAG:
    return flow_get_vlan_tag(skb);
    case FLOW_KEY_RXHASH:
    return flow_get_rxhash(skb);
    default:
    WARN_ON(1);
    return 0;
    }
    }

    (1 << FLOW_KEY_DST) |			\
    (1 << FLOW_KEY_PROTO) |		\
    (1 << FLOW_KEY_PROTO_SRC) |		\
    (1 << FLOW_KEY_PROTO_DST) | 		\
    (1 << FLOW_KEY_NFCT_SRC) |		\
    (1 << FLOW_KEY_NFCT_DST) |		\
    (1 << FLOW_KEY_NFCT_PROTO_SRC) |	\
    (1 << FLOW_KEY_NFCT_PROTO_DST))
    TC_INDIRECT_SCOPE int flow_classify(struct sk_buff *skb,
    const struct tcf_proto *tp,
    struct tcf_result *res)
    {
    struct flow_head *head = rcu_dereference_bh(tp.root);
    struct flow_filter *f;
    u32 keymask;
    u32 classid;
    unsigned int n, key;
    int r;
    list_for_each_entry_rcu(f, &head.filters, list) {
    u32 keys[FLOW_KEY_MAX + 1];
    struct flow_keys flow_keys;
    if (!tcf_em_tree_match(skb, &f.ematches, core::ptr::null_mut()))
    continue;
    keymask = f.keymask;
    if (keymask & FLOW_KEYS_NEEDED)
    skb_flow_dissect_flow_keys(skb, &flow_keys, 0);
    for (n = 0; n < f.nkeys; n++) {
    key = ffs(keymask) - 1;
    keymask &= ~(1 << key);
    keys[n] = flow_key_get(skb, key, &flow_keys);
    }
    if (f.mode == FLOW_MODE_HASH)
    classid = jhash2(keys, f.nkeys, f.hashrnd);
    else {
    classid = keys[0];
    classid = (classid & f.mask) ^ f.xor;
    classid = (classid >> f.rshift) + f.addend;
    }
    if (f.divisor)
    classid %= f.divisor;
    res.class   = 0;
    res.classid = TC_H_MAKE(f.baseclass, f.baseclass + classid);
    r = tcf_exts_exec(skb, &f.exts, res);
    if (r < 0)
    continue;
    return r;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn flow_perturbation(t: *mut timer_list) {
    static void flow_perturbation(struct timer_list *t)
    {
    struct flow_filter *f = timer_container_of(f, t, perturb_timer);
    get_random_bytes(&f.hashrnd, 4);
    if (f.perturb_period)
    mod_timer(&f.perturb_timer, jiffies + f.perturb_period);
    }
    static const struct nla_policy flow_policy[TCA_FLOW_MAX + 1] = {
    [TCA_FLOW_KEYS]		= { .type = NLA_U32 },
    [TCA_FLOW_MODE]		= { .type = NLA_U32 },
    [TCA_FLOW_BASECLASS]	= { .type = NLA_U32 },
    [TCA_FLOW_RSHIFT]	= NLA_POLICY_MAX(NLA_U32,
    31 /* BITS_PER_U32 - 1 */),
    [TCA_FLOW_ADDEND]	= { .type = NLA_U32 },
    [TCA_FLOW_MASK]		= { .type = NLA_U32 },
    [TCA_FLOW_XOR]		= { .type = NLA_U32 },
    [TCA_FLOW_DIVISOR]	= { .type = NLA_U32 },
    [TCA_FLOW_ACT]		= { .type = NLA_NESTED },
    [TCA_FLOW_POLICE]	= { .type = NLA_NESTED },
    [TCA_FLOW_EMATCHES]	= { .type = NLA_NESTED },
    [TCA_FLOW_PERTURB]	= { .type = NLA_U32 },
    };
#[no_mangle]
unsafe extern "C" fn __flow_destroy_filter(f: *mut flow_filter) {
    static void __flow_destroy_filter(struct flow_filter *f)
    {
    timer_shutdown_sync(&f.perturb_timer);
    tcf_exts_destroy(&f.exts);
    tcf_em_tree_destroy(&f.ematches);
    tcf_exts_put_net(&f.exts);
    kfree(f);
    }
#[no_mangle]
unsafe extern "C" fn flow_destroy_filter_work(work: *mut work_struct) {
    static void flow_destroy_filter_work(struct work_struct *work)
    {
    struct flow_filter *f = container_of(to_rcu_work(work),
    struct flow_filter,
    rwork);
    rtnl_lock();
    __flow_destroy_filter(f);
    rtnl_unlock();
    }
    static int flow_change(struct net *net, struct sk_buff *in_skb,
    struct tcf_proto *tp, unsigned long base,
    u32 handle, struct nlattr **tca,
    void **arg, u32 flags,
    struct netlink_ext_ack *extack)
    {
    struct flow_head *head = rtnl_dereference(tp.root);
    struct flow_filter *fold, *fnew;
    struct nlattr *opt = tca[TCA_OPTIONS];
    struct nlattr *tb[TCA_FLOW_MAX + 1];
    let mut nkeys: c_uint = 0;
    let mut perturb_period: c_uint = 0;
    let mut baseclass: u32 = 0;
    let mut keymask: u32 = 0;
    u32 mode;
    int err;
    if (opt == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_FLOW_MAX, opt, flow_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_FLOW_BASECLASS]) {
    baseclass = nla_get_u32(tb[TCA_FLOW_BASECLASS]);
    if (TC_H_MIN(baseclass) == 0)
    return -EINVAL;
    }
    if (tb[TCA_FLOW_KEYS]) {
    keymask = nla_get_u32(tb[TCA_FLOW_KEYS]);
    nkeys = hweight32(keymask);
    if (nkeys == 0)
    return -EINVAL;
    if (fls(keymask) - 1 > FLOW_KEY_MAX)
    return -EOPNOTSUPP;
    if ((keymask & (FLOW_KEY_SKUID|FLOW_KEY_SKGID)) &&
    sk_user_ns(NETLINK_CB(in_skb).sk) != &init_user_ns)
    return -EOPNOTSUPP;
    }
    fnew = kzalloc_obj(*fnew, GFP_KERNEL_ACCOUNT);
    if (!fnew)
    return -ENOBUFS;
    err = tcf_em_tree_validate(tp, tb[TCA_FLOW_EMATCHES], &fnew.ematches);
    if (err < 0)
    goto err1;
    err = tcf_exts_init(&fnew.exts, net, TCA_FLOW_ACT, TCA_FLOW_POLICE);
    if (err < 0)
    goto err2;
    err = tcf_exts_validate(net, tp, tb, tca[TCA_RATE], &fnew.exts, flags,
    extack);
    if (err < 0)
    goto err2;
    fold = *arg;
    if (fold) {
    err = -EINVAL;
    if (fold.handle != handle && handle)
    goto err2;
// Copy fold into fnew
    fnew.tp = fold.tp;
    fnew.handle = fold.handle;
    fnew.nkeys = fold.nkeys;
    fnew.keymask = fold.keymask;
    fnew.mode = fold.mode;
    fnew.mask = fold.mask;
    fnew.xor = fold.xor;
    fnew.rshift = fold.rshift;
    fnew.addend = fold.addend;
    fnew.divisor = fold.divisor;
    fnew.baseclass = fold.baseclass;
    fnew.hashrnd = fold.hashrnd;
    mode = fold.mode;
    if (tb[TCA_FLOW_MODE])
    mode = nla_get_u32(tb[TCA_FLOW_MODE]);
    if (mode != FLOW_MODE_HASH && nkeys > 1)
    goto err2;
    if (mode == FLOW_MODE_HASH)
    perturb_period = fold.perturb_period;
    if (tb[TCA_FLOW_PERTURB]) {
    if (mode != FLOW_MODE_HASH)
    goto err2;
    perturb_period = nla_get_u32(tb[TCA_FLOW_PERTURB]) * HZ;
    }
    } else {
    err = -EINVAL;
    if (!handle)
    goto err2;
    if (!tb[TCA_FLOW_KEYS])
    goto err2;
    mode = FLOW_MODE_MAP;
    if (tb[TCA_FLOW_MODE])
    mode = nla_get_u32(tb[TCA_FLOW_MODE]);
    if (mode != FLOW_MODE_HASH && nkeys > 1)
    goto err2;
    if (tb[TCA_FLOW_PERTURB]) {
    if (mode != FLOW_MODE_HASH)
    goto err2;
    perturb_period = nla_get_u32(tb[TCA_FLOW_PERTURB]) * HZ;
    }
    if (TC_H_MAJ(baseclass) == 0) {
    struct tcf_block *block = tp.chain.block;
    struct Qdisc *q;
    if (tcf_block_shared(block)) {
    NL_SET_ERR_MSG(extack,
    "Must specify baseclass when attaching flow filter to block");
    goto err2;
    }
    q = tcf_block_q(block);
    baseclass = TC_H_MAKE(q.handle, baseclass);
    }
    if (TC_H_MIN(baseclass) == 0)
    baseclass = TC_H_MAKE(baseclass, 1);
    fnew.handle = handle;
    fnew.mask  = ~0U;
    fnew.tp = tp;
    get_random_bytes(&fnew.hashrnd, 4);
    }
    timer_setup(&fnew.perturb_timer, flow_perturbation, TIMER_DEFERRABLE);
    tcf_block_netif_keep_dst(tp.chain.block);
    if (tb[TCA_FLOW_KEYS]) {
    fnew.keymask = keymask;
    fnew.nkeys   = nkeys;
    }
    fnew.mode = mode;
    if (tb[TCA_FLOW_MASK])
    fnew.mask = nla_get_u32(tb[TCA_FLOW_MASK]);
    if (tb[TCA_FLOW_XOR])
    fnew.xor = nla_get_u32(tb[TCA_FLOW_XOR]);
    if (tb[TCA_FLOW_RSHIFT])
    fnew.rshift = nla_get_u32(tb[TCA_FLOW_RSHIFT]);
    if (tb[TCA_FLOW_ADDEND])
    fnew.addend = nla_get_u32(tb[TCA_FLOW_ADDEND]);
    if (tb[TCA_FLOW_DIVISOR])
    fnew.divisor = nla_get_u32(tb[TCA_FLOW_DIVISOR]);
    if (baseclass)
    fnew.baseclass = baseclass;
    fnew.perturb_period = perturb_period;
    if (perturb_period)
    mod_timer(&fnew.perturb_timer, jiffies + perturb_period);
    if (!*arg)
    list_add_tail_rcu(&fnew.list, &head.filters);
    else
    list_replace_rcu(&fold.list, &fnew.list);
// arg = fnew;
    if (fold) {
    tcf_exts_get_net(&fold.exts);
    tcf_queue_work(&fold.rwork, flow_destroy_filter_work);
    }
    return 0;
    err2:
    tcf_exts_destroy(&fnew.exts);
    tcf_em_tree_destroy(&fnew.ematches);
    err1:
    kfree(fnew);
    return err;
    }
    static int flow_delete(struct tcf_proto *tp, void *arg, bool *last,
    bool rtnl_held, struct netlink_ext_ack *extack)
    {
    struct flow_head *head = rtnl_dereference(tp.root);
    struct flow_filter *f = arg;
    list_del_rcu(&f.list);
    tcf_exts_get_net(&f.exts);
    tcf_queue_work(&f.rwork, flow_destroy_filter_work);
// last = list_empty(&head->filters);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flow_init(tp: *mut tcf_proto) -> c_int {
    static int flow_init(struct tcf_proto *tp)
    {
    struct flow_head *head;
    head = kzalloc_obj(*head);
    if (head == core::ptr::null_mut())
    return -ENOBUFS;
    INIT_LIST_HEAD(&head.filters);
    rcu_assign_pointer(tp.root, head);
    net_get_random_once(&flow_keys_secret, sizeof(flow_keys_secret));
    return 0;
    }
    static void flow_destroy(struct tcf_proto *tp, bool rtnl_held,
    struct netlink_ext_ack *extack)
    {
    struct flow_head *head = rtnl_dereference(tp.root);
    struct flow_filter *f, *next;
    list_for_each_entry_safe(f, next, &head.filters, list) {
    list_del_rcu(&f.list);
    if (tcf_exts_get_net(&f.exts))
    tcf_queue_work(&f.rwork, flow_destroy_filter_work);
    else
    __flow_destroy_filter(f);
    }
    kfree_rcu(head, rcu);
    }
    static void *flow_get(struct tcf_proto *tp, u32 handle)
    {
    struct flow_head *head = rtnl_dereference(tp.root);
    struct flow_filter *f;
    list_for_each_entry(f, &head.filters, list)
    if (f.handle == handle)
    return f;
    return core::ptr::null_mut();
    }
    static int flow_dump(struct net *net, struct tcf_proto *tp, void *fh,
    struct sk_buff *skb, struct tcmsg *t, bool rtnl_held)
    {
    struct flow_filter *f = fh;
    struct nlattr *nest;
    if (f == core::ptr::null_mut())
    return skb.len;
    t.tcm_handle = f.handle;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (nest == core::ptr::null_mut())
    goto nla_put_failure;
    if (nla_put_u32(skb, TCA_FLOW_KEYS, f.keymask) ||
    nla_put_u32(skb, TCA_FLOW_MODE, f.mode))
    goto nla_put_failure;
    if (f.mask != ~0 || f.xor != 0) {
    if (nla_put_u32(skb, TCA_FLOW_MASK, f.mask) ||
    nla_put_u32(skb, TCA_FLOW_XOR, f.xor))
    goto nla_put_failure;
    }
    if (f.rshift &&
    nla_put_u32(skb, TCA_FLOW_RSHIFT, f.rshift))
    goto nla_put_failure;
    if (f.addend &&
    nla_put_u32(skb, TCA_FLOW_ADDEND, f.addend))
    goto nla_put_failure;
    if (f.divisor &&
    nla_put_u32(skb, TCA_FLOW_DIVISOR, f.divisor))
    goto nla_put_failure;
    if (f.baseclass &&
    nla_put_u32(skb, TCA_FLOW_BASECLASS, f.baseclass))
    goto nla_put_failure;
    if (f.perturb_period &&
    nla_put_u32(skb, TCA_FLOW_PERTURB, f.perturb_period / HZ))
    goto nla_put_failure;
    if (tcf_exts_dump(skb, &f.exts) < 0)
    goto nla_put_failure;

    if (f.ematches.hdr.nmatches &&
    tcf_em_tree_dump(skb, &f.ematches, TCA_FLOW_EMATCHES) < 0)
    goto nla_put_failure;

    nla_nest_end(skb, nest);
    if (tcf_exts_dump_stats(skb, &f.exts) < 0)
    goto nla_put_failure;
    return skb.len;
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -1;
    }
    static void flow_walk(struct tcf_proto *tp, struct tcf_walker *arg,
    bool rtnl_held)
    {
    struct flow_head *head = rtnl_dereference(tp.root);
    struct flow_filter *f;
    list_for_each_entry(f, &head.filters, list) {
    if (!tc_cls_stats_dump(tp, arg, f))
    break;
    }
    }
    static struct tcf_proto_ops cls_flow_ops __read_mostly = {
    .kind		= "flow",
    .classify	= flow_classify,
    .init		= flow_init,
    .destroy	= flow_destroy,
    .change		= flow_change,
    .delete		= flow_delete,
    .get		= flow_get,
    .dump		= flow_dump,
    .walk		= flow_walk,
    .owner		= THIS_MODULE,
    };
    MODULE_ALIAS_NET_CLS("flow");
#[no_mangle]
unsafe extern "C" fn cls_flow_init() -> int __init {
    static int __init cls_flow_init(void)
    {
    return register_tcf_proto_ops(&cls_flow_ops);
    }
#[no_mangle]
unsafe extern "C" fn cls_flow_exit() -> void __exit {
    static void __exit cls_flow_exit(void)
    {
    unregister_tcf_proto_ops(&cls_flow_ops);
    }
    module_init(cls_flow_init);
    module_exit(cls_flow_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_DESCRIPTION("TC flow classifier");
