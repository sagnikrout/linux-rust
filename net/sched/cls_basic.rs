//! Automatically rewritten from C to Rust
//! Source: net/sched/cls_basic.c
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
// net/sched/cls_basic.c	Basic Packet Classifier.
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_head {
    pub flist: list_head,
    pub handle_idr: idr,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_filter {
    pub handle: u32,
    pub exts: tcf_exts,
    pub ematches: tcf_ematch_tree,
    pub res: tcf_result,
    pub tp: *mut tcf_proto,
    pub link: list_head,
    pub pf: *mut tc_basic_pcnt __percpu,
    pub rwork: rcu_work,
}

    TC_INDIRECT_SCOPE int basic_classify(struct sk_buff *skb,
    const struct tcf_proto *tp,
    struct tcf_result *res)
    {
    int r;
    struct basic_head *head = rcu_dereference_bh(tp.root);
    struct basic_filter *f;
    list_for_each_entry_rcu(f, &head.flist, link) {
    __this_cpu_inc(f.pf.rcnt);
    if (!tcf_em_tree_match(skb, &f.ematches, core::ptr::null_mut()))
    continue;
    __this_cpu_inc(f.pf.rhit);
// res = f->res;
    r = tcf_exts_exec(skb, &f.exts, res);
    if (r < 0)
    continue;
    return r;
    }
    return -1;
    }
    static void *basic_get(struct tcf_proto *tp, u32 handle)
    {
    struct basic_head *head = rtnl_dereference(tp.root);
    struct basic_filter *f;
    list_for_each_entry(f, &head.flist, link) {
    if (f.handle == handle) {
    return f;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn basic_init(tp: *mut tcf_proto) -> c_int {
    static int basic_init(struct tcf_proto *tp)
    {
    struct basic_head *head;
    head = kzalloc_obj(*head);
    if (head == core::ptr::null_mut())
    return -ENOBUFS;
    INIT_LIST_HEAD(&head.flist);
    idr_init(&head.handle_idr);
    rcu_assign_pointer(tp.root, head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __basic_delete_filter(f: *mut basic_filter) {
    static void __basic_delete_filter(struct basic_filter *f)
    {
    tcf_exts_destroy(&f.exts);
    tcf_em_tree_destroy(&f.ematches);
    tcf_exts_put_net(&f.exts);
    free_percpu(f.pf);
    kfree(f);
    }
#[no_mangle]
unsafe extern "C" fn basic_delete_filter_work(work: *mut work_struct) {
    static void basic_delete_filter_work(struct work_struct *work)
    {
    struct basic_filter *f = container_of(to_rcu_work(work),
    struct basic_filter,
    rwork);
    rtnl_lock();
    __basic_delete_filter(f);
    rtnl_unlock();
    }
    static void basic_destroy(struct tcf_proto *tp, bool rtnl_held,
    struct netlink_ext_ack *extack)
    {
    struct basic_head *head = rtnl_dereference(tp.root);
    struct basic_filter *f, *n;
    list_for_each_entry_safe(f, n, &head.flist, link) {
    list_del_rcu(&f.link);
    tcf_unbind_filter(tp, &f.res);
    idr_remove(&head.handle_idr, f.handle);
    if (tcf_exts_get_net(&f.exts))
    tcf_queue_work(&f.rwork, basic_delete_filter_work);
    else
    __basic_delete_filter(f);
    }
    idr_destroy(&head.handle_idr);
    kfree_rcu(head, rcu);
    }
    static int basic_delete(struct tcf_proto *tp, void *arg, bool *last,
    bool rtnl_held, struct netlink_ext_ack *extack)
    {
    struct basic_head *head = rtnl_dereference(tp.root);
    struct basic_filter *f = arg;
    list_del_rcu(&f.link);
    tcf_unbind_filter(tp, &f.res);
    idr_remove(&head.handle_idr, f.handle);
    tcf_exts_get_net(&f.exts);
    tcf_queue_work(&f.rwork, basic_delete_filter_work);
// last = list_empty(&head->flist);
    return 0;
    }
    static const struct nla_policy basic_policy[TCA_BASIC_MAX + 1] = {
    [TCA_BASIC_CLASSID]	= { .type = NLA_U32 },
    [TCA_BASIC_EMATCHES]	= { .type = NLA_NESTED },
    };
    static int basic_set_parms(struct net *net, struct tcf_proto *tp,
    struct basic_filter *f, unsigned long base,
    struct nlattr **tb,
    struct nlattr *est, u32 flags,
    struct netlink_ext_ack *extack)
    {
    int err;
    err = tcf_exts_validate(net, tp, tb, est, &f.exts, flags, extack);
    if (err < 0)
    return err;
    err = tcf_em_tree_validate(tp, tb[TCA_BASIC_EMATCHES], &f.ematches);
    if (err < 0)
    return err;
    if (tb[TCA_BASIC_CLASSID]) {
    f.res.classid = nla_get_u32(tb[TCA_BASIC_CLASSID]);
    tcf_bind_filter(tp, &f.res, base);
    }
    f.tp = tp;
    return 0;
    }
    static int basic_change(struct net *net, struct sk_buff *in_skb,
    struct tcf_proto *tp, unsigned long base, u32 handle,
    struct nlattr **tca, void **arg,
    u32 flags, struct netlink_ext_ack *extack)
    {
    int err;
    struct basic_head *head = rtnl_dereference(tp.root);
    struct nlattr *tb[TCA_BASIC_MAX + 1];
    struct basic_filter *fold = (struct basic_filter *) *arg;
    struct basic_filter *fnew;
    if (tca[TCA_OPTIONS] == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_BASIC_MAX, tca[TCA_OPTIONS],
    basic_policy, core::ptr::null_mut());
    if (err < 0)
    return err;
    if (fold != core::ptr::null_mut()) {
    if (handle && fold.handle != handle)
    return -EINVAL;
    }
    fnew = kzalloc_obj(*fnew, GFP_KERNEL_ACCOUNT);
    if (!fnew)
    return -ENOBUFS;
    err = tcf_exts_init(&fnew.exts, net, TCA_BASIC_ACT, TCA_BASIC_POLICE);
    if (err < 0)
    goto errout;
    if (!handle) {
    handle = 1;
    err = idr_alloc_u32(&head.handle_idr, fnew, &handle,
    INT_MAX, GFP_KERNEL);
    } else if (!fold) {
    err = idr_alloc_u32(&head.handle_idr, fnew, &handle,
    handle, GFP_KERNEL);
    }
    if (err)
    goto errout;
    fnew.handle = handle;
    fnew.pf = alloc_percpu_gfp(struct tc_basic_pcnt, GFP_KERNEL_ACCOUNT);
    if (!fnew.pf) {
    err = -ENOMEM;
    if (!fold)
    idr_remove(&head.handle_idr, fnew.handle);
    goto errout;
    }
    err = basic_set_parms(net, tp, fnew, base, tb, tca[TCA_RATE], flags,
    extack);
    if (err < 0) {
    if (!fold)
    idr_remove(&head.handle_idr, fnew.handle);
    goto errout;
    }
// arg = fnew;
    if (fold) {
    idr_replace(&head.handle_idr, fnew, fnew.handle);
    list_replace_rcu(&fold.link, &fnew.link);
    tcf_unbind_filter(tp, &fold.res);
    tcf_exts_get_net(&fold.exts);
    tcf_queue_work(&fold.rwork, basic_delete_filter_work);
    } else {
    list_add_rcu(&fnew.link, &head.flist);
    }
    return 0;
    errout:
    free_percpu(fnew.pf);
    tcf_exts_destroy(&fnew.exts);
    kfree(fnew);
    return err;
    }
    static void basic_walk(struct tcf_proto *tp, struct tcf_walker *arg,
    bool rtnl_held)
    {
    struct basic_head *head = rtnl_dereference(tp.root);
    struct basic_filter *f;
    list_for_each_entry(f, &head.flist, link) {
    if (!tc_cls_stats_dump(tp, arg, f))
    break;
    }
    }
    static void basic_bind_class(void *fh, u32 classid, unsigned long cl, void *q,
    unsigned long base)
    {
    struct basic_filter *f = fh;
    tc_cls_bind_class(classid, cl, q, &f.res, base);
    }
    static int basic_dump(struct net *net, struct tcf_proto *tp, void *fh,
    struct sk_buff *skb, struct tcmsg *t, bool rtnl_held)
    {
    let mut gpf: tc_basic_pcnt = {};
    struct basic_filter *f = fh;
    struct nlattr *nest;
    int cpu;
    if (f == core::ptr::null_mut())
    return skb.len;
    t.tcm_handle = f.handle;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (nest == core::ptr::null_mut())
    goto nla_put_failure;
    if (f.res.classid &&
    nla_put_u32(skb, TCA_BASIC_CLASSID, f.res.classid))
    goto nla_put_failure;
    for_each_possible_cpu(cpu) {
    struct tc_basic_pcnt *pf = per_cpu_ptr(f.pf, cpu);
    gpf.rcnt += pf.rcnt;
    gpf.rhit += pf.rhit;
    }
    if (nla_put_64bit(skb, TCA_BASIC_PCNT,
    sizeof(struct tc_basic_pcnt),
    &gpf, TCA_BASIC_PAD))
    goto nla_put_failure;
    if (tcf_exts_dump(skb, &f.exts) < 0 ||
    tcf_em_tree_dump(skb, &f.ematches, TCA_BASIC_EMATCHES) < 0)
    goto nla_put_failure;
    nla_nest_end(skb, nest);
    if (tcf_exts_dump_stats(skb, &f.exts) < 0)
    goto nla_put_failure;
    return skb.len;
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -1;
    }
    static struct tcf_proto_ops cls_basic_ops __read_mostly = {
    .kind		=	"basic",
    .classify	=	basic_classify,
    .init		=	basic_init,
    .destroy	=	basic_destroy,
    .get		=	basic_get,
    .change		=	basic_change,
    .delete		=	basic_delete,
    .walk		=	basic_walk,
    .dump		=	basic_dump,
    .bind_class	=	basic_bind_class,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_CLS("basic");
#[no_mangle]
unsafe extern "C" fn init_basic() -> int __init {
    static int __init init_basic(void)
    {
    return register_tcf_proto_ops(&cls_basic_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_basic() -> void __exit {
    static void __exit exit_basic(void)
    {
    unregister_tcf_proto_ops(&cls_basic_ops);
    }
    module_init(init_basic)
    module_exit(exit_basic)
    MODULE_DESCRIPTION("TC basic classifier");
    MODULE_LICENSE("GPL");
