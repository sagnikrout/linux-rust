//! Automatically rewritten from C to Rust
//! Source: net/sched/cls_fw.c
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
// net/sched/cls_fw.c	Classifier mapping ipchains' fwmark to traffic class.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//
// Changes:
// Karlis Peisenieks <karlis@mt.lv> : 990415 : fw_walk off by one
// Karlis Peisenieks <karlis@mt.lv> : 990415 : fw_delete killed all the filter (and kernel).
// Alex <alex@pilotsoft.com> : 2004xxyy: Added Action extension
//

pub const HTSIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_head {
    pub mask: u32,
    pub ht: [*mut fw_filter __rcu; HTSIZE],
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_filter {
    pub next: *mut fw_filter __rcu,
    pub id: u32,
    pub res: tcf_result,
    pub ifindex: c_int,
    pub exts: tcf_exts,
    pub tp: *mut tcf_proto,
    pub rwork: rcu_work,
}

#[no_mangle]
unsafe extern "C" fn fw_hash(handle: u32) -> u32 {
    static u32 fw_hash(u32 handle)
    {
    handle ^= (handle >> 16);
    handle ^= (handle >> 8);
    return handle % HTSIZE;
    }
    TC_INDIRECT_SCOPE int fw_classify(struct sk_buff *skb,
    const struct tcf_proto *tp,
    struct tcf_result *res)
    {
    struct fw_head *head = rcu_dereference_bh(tp.root);
    struct fw_filter *f;
    int r;
    let mut id: u32 = skb.mark;
    if (head != core::ptr::null_mut()) {
    id &= head.mask;
    for (f = rcu_dereference_bh(head.ht[fw_hash(id)]); f;
    f = rcu_dereference_bh(f.next)) {
    if (f.id == id) {
// res = f->res;
    if (!tcf_match_indev(skb, f.ifindex))
    continue;
    r = tcf_exts_exec(skb, &f.exts, res);
    if (r < 0)
    continue;
    return r;
    }
    }
    } else {
    struct Qdisc *q;
// Old method: classify the packet using its skb mark.
    if (tcf_block_shared(tp.chain.block))
    return -1;
    q = tcf_block_q(tp.chain.block);
    if (id && (TC_H_MAJ(id) == 0 ||
    !(TC_H_MAJ(id ^ q.handle)))) {
    res.classid = id;
    res.class = 0;
    return 0;
    }
    }
    return -1;
    }
    static void *fw_get(struct tcf_proto *tp, u32 handle)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    struct fw_filter *f;
    if (head == core::ptr::null_mut())
    return core::ptr::null_mut();
    f = rtnl_dereference(head.ht[fw_hash(handle)]);
    for (; f; f = rtnl_dereference(f.next)) {
    if (f.id == handle)
    return f;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fw_init(tp: *mut tcf_proto) -> c_int {
    static int fw_init(struct tcf_proto *tp)
    {
// We don't allocate fw_head here, because in the old method
// we don't need it at all.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __fw_delete_filter(f: *mut fw_filter) {
    static void __fw_delete_filter(struct fw_filter *f)
    {
    tcf_exts_destroy(&f.exts);
    tcf_exts_put_net(&f.exts);
    kfree(f);
    }
#[no_mangle]
unsafe extern "C" fn fw_delete_filter_work(work: *mut work_struct) {
    static void fw_delete_filter_work(struct work_struct *work)
    {
    struct fw_filter *f = container_of(to_rcu_work(work),
    struct fw_filter,
    rwork);
    rtnl_lock();
    __fw_delete_filter(f);
    rtnl_unlock();
    }
    static void fw_destroy(struct tcf_proto *tp, bool rtnl_held,
    struct netlink_ext_ack *extack)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    struct fw_filter *f;
    int h;
    if (head == core::ptr::null_mut())
    return;
    for (h = 0; h < HTSIZE; h++) {
    while ((f = rtnl_dereference(head.ht[h])) != core::ptr::null_mut()) {
    RCU_INIT_POINTER(head.ht[h],
    rtnl_dereference(f.next));
    tcf_unbind_filter(tp, &f.res);
    if (tcf_exts_get_net(&f.exts))
    tcf_queue_work(&f.rwork, fw_delete_filter_work);
    else
    __fw_delete_filter(f);
    }
    }
    kfree_rcu(head, rcu);
    }
    static int fw_delete(struct tcf_proto *tp, void *arg, bool *last,
    bool rtnl_held, struct netlink_ext_ack *extack)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    struct fw_filter *f = arg;
    struct fw_filter __rcu **fp;
    struct fw_filter *pfp;
    let mut ret: c_int = -EINVAL;
    int h;
    if (head == core::ptr::null_mut() || f == core::ptr::null_mut())
    goto out;
    fp = &head.ht[fw_hash(f.id)];
    for (pfp = rtnl_dereference(*fp); pfp;
    fp = &pfp.next, pfp = rtnl_dereference(*fp)) {
    if (pfp == f) {
    RCU_INIT_POINTER(*fp, rtnl_dereference(f.next));
    tcf_unbind_filter(tp, &f.res);
    tcf_exts_get_net(&f.exts);
    tcf_queue_work(&f.rwork, fw_delete_filter_work);
    ret = 0;
    break;
    }
    }
// last = true;
    for (h = 0; h < HTSIZE; h++) {
    if (rcu_access_pointer(head.ht[h])) {
// last = false;
    break;
    }
    }
    out:
    return ret;
    }
    static const struct nla_policy fw_policy[TCA_FW_MAX + 1] = {
    [TCA_FW_CLASSID]	= { .type = NLA_U32 },
    [TCA_FW_INDEV]		= { .type = NLA_STRING, .len = IFNAMSIZ },
    [TCA_FW_MASK]		= { .type = NLA_U32 },
    };
    static int fw_set_parms(struct net *net, struct tcf_proto *tp,
    struct fw_filter *f, struct nlattr **tb,
    struct nlattr **tca, unsigned long base, u32 flags,
    struct netlink_ext_ack *extack)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    u32 mask;
    int err;
    err = tcf_exts_validate(net, tp, tb, tca[TCA_RATE], &f.exts, flags,
    extack);
    if (err < 0)
    return err;
    if (tb[TCA_FW_INDEV]) {
    int ret;
    ret = tcf_change_indev(net, tb[TCA_FW_INDEV], extack);
    if (ret < 0)
    return ret;
    f.ifindex = ret;
    }
    err = -EINVAL;
    if (tb[TCA_FW_MASK]) {
    mask = nla_get_u32(tb[TCA_FW_MASK]);
    if (mask != head.mask)
    return err;
    } else if (head.mask != 0xFFFFFFFF)
    return err;
    if (tb[TCA_FW_CLASSID]) {
    f.res.classid = nla_get_u32(tb[TCA_FW_CLASSID]);
    tcf_bind_filter(tp, &f.res, base);
    }
    return 0;
    }
    static int fw_change(struct net *net, struct sk_buff *in_skb,
    struct tcf_proto *tp, unsigned long base,
    u32 handle, struct nlattr **tca, void **arg,
    u32 flags, struct netlink_ext_ack *extack)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    struct fw_filter *f = *arg;
    struct nlattr *opt = tca[TCA_OPTIONS];
    struct nlattr *tb[TCA_FW_MAX + 1];
    int err;
    if (!opt) {
    if (handle)
    return -EINVAL;
    if (tcf_block_shared(tp.chain.block)) {
    NL_SET_ERR_MSG(extack,
    "Must specify mark when attaching fw filter to block");
    return -EINVAL;
    }
    return 0; /* Succeed if it is old method. */
    }
    err = nla_parse_nested_deprecated(tb, TCA_FW_MAX, opt, fw_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (f) {
    struct fw_filter *pfp, *fnew;
    struct fw_filter __rcu **fp;
    if (f.id != handle && handle)
    return -EINVAL;
    fnew = kzalloc_obj(struct fw_filter, GFP_KERNEL_ACCOUNT);
    if (!fnew)
    return -ENOBUFS;
    fnew.id = f.id;
    fnew.ifindex = f.ifindex;
    fnew.tp = f.tp;
    err = tcf_exts_init(&fnew.exts, net, TCA_FW_ACT,
    TCA_FW_POLICE);
    if (err < 0) {
    kfree(fnew);
    return err;
    }
    err = fw_set_parms(net, tp, fnew, tb, tca, base, flags, extack);
    if (err < 0) {
    tcf_exts_destroy(&fnew.exts);
    kfree(fnew);
    return err;
    }
    fp = &head.ht[fw_hash(fnew.id)];
    for (pfp = rtnl_dereference(*fp); pfp;
    fp = &pfp.next, pfp = rtnl_dereference(*fp))
    if (pfp == f)
    break;
    RCU_INIT_POINTER(fnew.next, rtnl_dereference(pfp.next));
    rcu_assign_pointer(*fp, fnew);
    tcf_unbind_filter(tp, &f.res);
    tcf_exts_get_net(&f.exts);
    tcf_queue_work(&f.rwork, fw_delete_filter_work);
// arg = fnew;
    return err;
    }
    if (!handle)
    return -EINVAL;
    if (!head) {
    let mut mask: u32 = 0xFFFFFFFF;
    if (tb[TCA_FW_MASK])
    mask = nla_get_u32(tb[TCA_FW_MASK]);
    head = kzalloc_obj(*head);
    if (!head)
    return -ENOBUFS;
    head.mask = mask;
    rcu_assign_pointer(tp.root, head);
    }
    f = kzalloc_obj(struct fw_filter, GFP_KERNEL_ACCOUNT);
    if (f == core::ptr::null_mut())
    return -ENOBUFS;
    err = tcf_exts_init(&f.exts, net, TCA_FW_ACT, TCA_FW_POLICE);
    if (err < 0)
    goto errout;
    f.id = handle;
    f.tp = tp;
    err = fw_set_parms(net, tp, f, tb, tca, base, flags, extack);
    if (err < 0)
    goto errout;
    RCU_INIT_POINTER(f.next, head.ht[fw_hash(handle)]);
    rcu_assign_pointer(head.ht[fw_hash(handle)], f);
// arg = f;
    return 0;
    errout:
    tcf_exts_destroy(&f.exts);
    kfree(f);
    return err;
    }
    static void fw_walk(struct tcf_proto *tp, struct tcf_walker *arg,
    bool rtnl_held)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    int h;
    if (head == core::ptr::null_mut())
    arg.stop = 1;
    if (arg.stop)
    return;
    for (h = 0; h < HTSIZE; h++) {
    struct fw_filter *f;
    for (f = rtnl_dereference(head.ht[h]); f;
    f = rtnl_dereference(f.next)) {
    if (!tc_cls_stats_dump(tp, arg, f))
    return;
    }
    }
    }
    static int fw_dump(struct net *net, struct tcf_proto *tp, void *fh,
    struct sk_buff *skb, struct tcmsg *t, bool rtnl_held)
    {
    struct fw_head *head = rtnl_dereference(tp.root);
    struct fw_filter *f = fh;
    struct nlattr *nest;
    if (f == core::ptr::null_mut())
    return skb.len;
    t.tcm_handle = f.id;
    if (!f.res.classid && !tcf_exts_has_actions(&f.exts))
    return skb.len;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (nest == core::ptr::null_mut())
    goto nla_put_failure;
    if (f.res.classid &&
    nla_put_u32(skb, TCA_FW_CLASSID, f.res.classid))
    goto nla_put_failure;
    if (f.ifindex) {
    struct net_device *dev;
    dev = __dev_get_by_index(net, f.ifindex);
    if (dev && nla_put_string(skb, TCA_FW_INDEV, dev.name))
    goto nla_put_failure;
    }
    if (head.mask != 0xFFFFFFFF &&
    nla_put_u32(skb, TCA_FW_MASK, head.mask))
    goto nla_put_failure;
    if (tcf_exts_dump(skb, &f.exts) < 0)
    goto nla_put_failure;
    nla_nest_end(skb, nest);
    if (tcf_exts_dump_stats(skb, &f.exts) < 0)
    goto nla_put_failure;
    return skb.len;
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -1;
    }
    static void fw_bind_class(void *fh, u32 classid, unsigned long cl, void *q,
    unsigned long base)
    {
    struct fw_filter *f = fh;
    tc_cls_bind_class(classid, cl, q, &f.res, base);
    }
    static struct tcf_proto_ops cls_fw_ops __read_mostly = {
    .kind		=	"fw",
    .classify	=	fw_classify,
    .init		=	fw_init,
    .destroy	=	fw_destroy,
    .get		=	fw_get,
    .change		=	fw_change,
    .delete		=	fw_delete,
    .walk		=	fw_walk,
    .dump		=	fw_dump,
    .bind_class	=	fw_bind_class,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_CLS("fw");
#[no_mangle]
unsafe extern "C" fn init_fw() -> int __init {
    static int __init init_fw(void)
    {
    return register_tcf_proto_ops(&cls_fw_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_fw() -> void __exit {
    static void __exit exit_fw(void)
    {
    unregister_tcf_proto_ops(&cls_fw_ops);
    }
    module_init(init_fw)
    module_exit(exit_fw)
    MODULE_DESCRIPTION("SKB mark based TC classifier");
    MODULE_LICENSE("GPL");
