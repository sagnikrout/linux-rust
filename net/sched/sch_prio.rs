//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_prio.c
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
// net/sched/sch_prio.c	Simple 3-band priority "scheduler".
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
// Fixes:       19990609: J Hadi Salim <hadi@nortelnetworks.com>:
// Init --  EINVAL when opt undefined
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prio_sched_data {
    pub bands: c_int,
    pub filter_list: *mut tcf_proto __rcu,
    pub block: *mut tcf_block,
    pub prio2band: [u8; TC_PRIO_MAX+1],
    pub queues: [*mut Qdisc; TCQ_PRIO_BANDS],
}

    static struct Qdisc *
    prio_classify(struct sk_buff *skb, struct Qdisc *sch, int *qerr)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    let mut band: u32 = skb.priority;
    struct tcf_result res;
    struct tcf_proto *fl;
    int err;
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    if (TC_H_MAJ(skb.priority) != sch.handle) {
    fl = rcu_dereference_bh(q.filter_list);
    err = tcf_classify_qdisc(skb, fl, &res, false);

    switch (err) {
    case TC_ACT_STOLEN:
    case TC_ACT_QUEUED:
    case TC_ACT_TRAP:
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
    fallthrough;
    case TC_ACT_SHOT:
    return core::ptr::null_mut();
    }

    if (!fl || err < 0) {
    if (TC_H_MAJ(band))
    band = 0;
    return q.queues[q.prio2band[band & TC_PRIO_MAX]];
    }
    band = res.classid;
    }
    band = TC_H_MIN(band) - 1;
    if (band >= q.bands)
    return q.queues[q.prio2band[0]];
    return q.queues[band];
    }
    static int
    prio_enqueue(struct sk_buff *skb, struct Qdisc *sch, struct sk_buff **to_free)
    {
    let mut len: c_uint = qdisc_pkt_len(skb);
    struct Qdisc *qdisc;
    int ret;
    qdisc = prio_classify(skb, sch, &ret);

    if (qdisc == core::ptr::null_mut()) {
    if (ret & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return ret;
    }

    ret = qdisc_enqueue(skb, qdisc, to_free);
    if (ret == NET_XMIT_SUCCESS) {
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    return NET_XMIT_SUCCESS;
    }
    if (net_xmit_drop_count(ret))
    qdisc_qstats_drop(sch);
    return ret;
    }
    static struct sk_buff *prio_peek(struct Qdisc *sch)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    int prio;
    for (prio = 0; prio < q.bands; prio++) {
    struct Qdisc *qdisc = q.queues[prio];
    struct sk_buff *skb = qdisc.ops.peek(qdisc);
    if (skb)
    return skb;
    }
    return core::ptr::null_mut();
    }
    static struct sk_buff *prio_dequeue(struct Qdisc *sch)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    int prio;
    for (prio = 0; prio < q.bands; prio++) {
    struct Qdisc *qdisc = q.queues[prio];
    struct sk_buff *skb = qdisc_dequeue_peeked(qdisc);
    if (skb) {
    qdisc_bstats_update(sch, skb);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_qlen_dec(sch);
    return skb;
    }
    }
    return core::ptr::null_mut();
    }
    static void
    prio_reset(struct Qdisc *sch)
    {
    int prio;
    struct prio_sched_data *q = qdisc_priv(sch);
    for (prio = 0; prio < q.bands; prio++)
    qdisc_reset(q.queues[prio]);
    }
#[no_mangle]
unsafe extern "C" fn prio_offload(sch: *mut Qdisc, qopt: *mut tc_prio_qopt) -> c_int {
    static int prio_offload(struct Qdisc *sch, struct tc_prio_qopt *qopt)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct tc_prio_qopt_offload opt = {
    .handle = sch.handle,
    .parent = sch.parent,
    };
    if (!tc_can_offload(dev) || !dev.netdev_ops.ndo_setup_tc)
    return -EOPNOTSUPP;
    if (qopt) {
    opt.command = TC_PRIO_REPLACE;
    opt.replace_params.bands = qopt.bands;
    memcpy(&opt.replace_params.priomap, qopt.priomap,
    TC_PRIO_MAX + 1);
    opt.replace_params.qstats = &sch.qstats;
    } else {
    opt.command = TC_PRIO_DESTROY;
    }
    return dev.netdev_ops.ndo_setup_tc(dev, TC_SETUP_QDISC_PRIO, &opt);
    }
    static void
    prio_destroy(struct Qdisc *sch)
    {
    int prio;
    struct prio_sched_data *q = qdisc_priv(sch);
    tcf_block_put(q.block);
    prio_offload(sch, core::ptr::null_mut());
    for (prio = 0; prio < q.bands; prio++)
    qdisc_put(q.queues[prio]);
    }
    static int prio_tune(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    struct Qdisc *queues[TCQ_PRIO_BANDS];
    let mut oldbands: c_int = q.bands, i;
    struct tc_prio_qopt *qopt;
    if (nla_len(opt) < sizeof(*qopt))
    return -EINVAL;
    qopt = nla_data(opt);
    if (qopt.bands > TCQ_PRIO_BANDS || qopt.bands < TCQ_MIN_PRIO_BANDS)
    return -EINVAL;
    for (i = 0; i <= TC_PRIO_MAX; i++) {
    if (qopt.priomap[i] >= qopt.bands)
    return -EINVAL;
    }
// Before commit, make sure we can allocate all new qdiscs
    for (i = oldbands; i < qopt.bands; i++) {
    queues[i] = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    TC_H_MAKE(sch.handle, i + 1),
    extack);
    if (!queues[i]) {
    while (i > oldbands)
    qdisc_put(queues[--i]);
    return -ENOMEM;
    }
    }
    prio_offload(sch, qopt);
    sch_tree_lock(sch);
    q.bands = qopt.bands;
    memcpy(q.prio2band, qopt.priomap, TC_PRIO_MAX+1);
    for (i = q.bands; i < oldbands; i++)
    qdisc_purge_queue(q.queues[i]);
    for (i = oldbands; i < q.bands; i++) {
    q.queues[i] = queues[i];
    if (q.queues[i] != &noop_qdisc)
    qdisc_hash_add(q.queues[i], true);
    }
    sch_tree_unlock(sch);
    for (i = q.bands; i < oldbands; i++)
    qdisc_put(q.queues[i]);
    return 0;
    }
    static int prio_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    int err;
    if (!opt)
    return -EINVAL;
    err = tcf_block_get(&q.block, &q.filter_list, sch, extack);
    if (err)
    return err;
    return prio_tune(sch, opt, extack);
    }
#[no_mangle]
unsafe extern "C" fn prio_dump_offload(sch: *mut Qdisc) -> c_int {
    static int prio_dump_offload(struct Qdisc *sch)
    {
    struct tc_prio_qopt_offload hw_stats = {
    .command = TC_PRIO_STATS,
    .handle = sch.handle,
    .parent = sch.parent,
    {
    .stats = {
    .bstats = &sch.bstats,
    .qstats = &sch.qstats,
    },
    },
    };
    return qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_PRIO, &hw_stats);
    }
#[no_mangle]
unsafe extern "C" fn prio_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int prio_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    unsigned char *b = skb_tail_pointer(skb);
    struct tc_prio_qopt opt;
    int err;
    opt.bands = q.bands;
    memcpy(&opt.priomap, q.prio2band, TC_PRIO_MAX + 1);
    err = prio_dump_offload(sch);
    if (err)
    goto nla_put_failure;
    if (nla_put(skb, TCA_OPTIONS, sizeof(opt), &opt))
    goto nla_put_failure;
    return skb.len;
    nla_put_failure:
    nlmsg_trim(skb, b);
    return -1;
    }
    static int prio_graft(struct Qdisc *sch, unsigned long arg, struct Qdisc *new,
    struct Qdisc **old, struct netlink_ext_ack *extack)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    struct tc_prio_qopt_offload graft_offload;
    let mut band: c_ulong = arg - 1;
    if (!new) {
    new = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    TC_H_MAKE(sch.handle, arg), extack);
    if (!new)
    new = &noop_qdisc;
    else
    qdisc_hash_add(new, true);
    }
// old = qdisc_replace(sch, new, &q->queues[band]);
    graft_offload.handle = sch.handle;
    graft_offload.parent = sch.parent;
    graft_offload.graft_params.band = band;
    graft_offload.graft_params.child_handle = new.handle;
    graft_offload.command = TC_PRIO_GRAFT;
    qdisc_offload_graft_helper(qdisc_dev(sch), sch, new, *old,
    TC_SETUP_QDISC_PRIO, &graft_offload,
    extack);
    return 0;
    }
    static struct Qdisc *
    prio_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    let mut band: c_ulong = arg - 1;
    return q.queues[band];
    }
#[no_mangle]
unsafe extern "C" fn prio_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long prio_find(struct Qdisc *sch, u32 classid)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    let mut band: c_ulong = TC_H_MIN(classid);
    if (band - 1 >= q.bands)
    return 0;
    return band;
    }
#[no_mangle]
unsafe extern "C" fn prio_bind(sch: *mut Qdisc, parent: c_ulong, classid: u32) -> c_ulong {
    static unsigned long prio_bind(struct Qdisc *sch, unsigned long parent, u32 classid)
    {
    return prio_find(sch, classid);
    }
#[no_mangle]
unsafe extern "C" fn prio_unbind(q: *mut Qdisc, cl: c_ulong) {
    static void prio_unbind(struct Qdisc *q, unsigned long cl)
    {
    }
    static int prio_dump_class(struct Qdisc *sch, unsigned long cl, struct sk_buff *skb,
    struct tcmsg *tcm)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    tcm.tcm_handle |= TC_H_MIN(cl);
    tcm.tcm_info = q.queues[cl-1].handle;
    return 0;
    }
    static int prio_dump_class_stats(struct Qdisc *sch, unsigned long cl,
    struct gnet_dump *d)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    struct Qdisc *cl_q;
    cl_q = q.queues[cl - 1];
    if (gnet_stats_copy_basic(d, cl_q.cpu_bstats,
    &cl_q.bstats, true) < 0 ||
    qdisc_qstats_copy(d, cl_q) < 0)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prio_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void prio_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    int prio;
    if (arg.stop)
    return;
    for (prio = 0; prio < q.bands; prio++) {
    if (!tc_qdisc_stats_dump(sch, prio + 1, arg))
    break;
    }
    }
    static struct tcf_block *prio_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct prio_sched_data *q = qdisc_priv(sch);
    if (cl)
    return core::ptr::null_mut();
    return q.block;
    }
    static const struct Qdisc_class_ops prio_class_ops = {
    .graft		=	prio_graft,
    .leaf		=	prio_leaf,
    .find		=	prio_find,
    .walk		=	prio_walk,
    .tcf_block	=	prio_tcf_block,
    .bind_tcf	=	prio_bind,
    .unbind_tcf	=	prio_unbind,
    .dump		=	prio_dump_class,
    .dump_stats	=	prio_dump_class_stats,
    };
    static struct Qdisc_ops prio_qdisc_ops __read_mostly = {
    .next		=	core::ptr::null_mut(),
    .cl_ops		=	&prio_class_ops,
    .id		=	"prio",
    .priv_size	=	sizeof(struct prio_sched_data),
    .enqueue	=	prio_enqueue,
    .dequeue	=	prio_dequeue,
    .peek		=	prio_peek,
    .init		=	prio_init,
    .reset		=	prio_reset,
    .destroy	=	prio_destroy,
    .change		=	prio_tune,
    .dump		=	prio_dump,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("prio");
#[no_mangle]
unsafe extern "C" fn prio_module_init() -> int __init {
    static int __init prio_module_init(void)
    {
    return register_qdisc(&prio_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn prio_module_exit() -> void __exit {
    static void __exit prio_module_exit(void)
    {
    unregister_qdisc(&prio_qdisc_ops);
    }
    module_init(prio_module_init)
    module_exit(prio_module_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Simple 3-band priority qdisc");
