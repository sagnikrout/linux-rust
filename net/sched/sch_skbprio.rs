//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_skbprio.c
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
// net/sched/sch_skbprio.c  SKB Priority Queue.
//
// Authors:	Nishanth Devarajan, <ndev2021@gmail.com>
// Cody Doucette, <doucette@bu.edu>
// original idea by Michel Machado, Cody Doucette, and Qiaobin Fu
//

// SKB Priority Queue
// =================================
//
// Skbprio (SKB Priority Queue) is a queueing discipline that prioritizes
// packets according to their skb->priority field. Under congestion,
// Skbprio drops already-enqueued lower priority packets to make space
// available for higher priority packets; it was conceived as a solution
// for denial-of-service defenses that need to route packets with different
// priorities as a mean to overcome DoS attacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skbprio_sched_data {
// Queue state.
    pub qdiscs: [sk_buff_head; SKBPRIO_MAX_PRIORITY],
    pub qstats: [gnet_stats_queue; SKBPRIO_MAX_PRIORITY],
    pub highest_prio: u16,
    pub lowest_prio: u16,
}

#[no_mangle]
unsafe extern "C" fn calc_new_high_prio(q: *const skbprio_sched_data) -> u16 {
    static u16 calc_new_high_prio(const struct skbprio_sched_data *q)
    {
    int prio;
    for (prio = q.highest_prio - 1; prio >= q.lowest_prio; prio--) {
    if (!skb_queue_empty(&q.qdiscs[prio]))
    return prio;
    }
// SKB queue is empty, return 0 (default highest priority setting).
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn calc_new_low_prio(q: *const skbprio_sched_data) -> u16 {
    static u16 calc_new_low_prio(const struct skbprio_sched_data *q)
    {
    int prio;
    for (prio = q.lowest_prio + 1; prio <= q.highest_prio; prio++) {
    if (!skb_queue_empty(&q.qdiscs[prio]))
    return prio;
    }
// SKB queue is empty, return SKBPRIO_MAX_PRIORITY - 1
// (default lowest priority setting).
//
    return SKBPRIO_MAX_PRIORITY - 1;
    }
    static int skbprio_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    let mut max_priority: c_uint = SKBPRIO_MAX_PRIORITY - 1;
    struct skbprio_sched_data *q = qdisc_priv(sch);
    struct sk_buff_head *qdisc;
    struct sk_buff_head *lp_qdisc;
    struct sk_buff *to_drop;
    u16 prio, lp;
// Obtain the priority of @skb.
    prio = min(skb.priority, max_priority);
    qdisc = &q.qdiscs[prio];
// sch->limit can change under us from skbprio_change()
    if (sch.q.qlen < READ_ONCE(sch.limit)) {
    __skb_queue_tail(qdisc, skb);
    qdisc_qstats_backlog_inc(sch, skb);
    q.qstats[prio].backlog += qdisc_pkt_len(skb);
// Check to update highest and lowest priorities.
    if (prio > q.highest_prio)
    q.highest_prio = prio;
    if (prio < q.lowest_prio)
    q.lowest_prio = prio;
    qdisc_qlen_inc(sch);
    return NET_XMIT_SUCCESS;
    }
// If this packet has the lowest priority, drop it.
    lp = q.lowest_prio;
    if (prio <= lp) {
    q.qstats[prio].drops++;
    q.qstats[prio].overlimits++;
    return qdisc_drop(skb, sch, to_free);
    }
    __skb_queue_tail(qdisc, skb);
    qdisc_qstats_backlog_inc(sch, skb);
    q.qstats[prio].backlog += qdisc_pkt_len(skb);
// Drop the packet at the tail of the lowest priority qdisc.
    lp_qdisc = &q.qdiscs[lp];
    to_drop = __skb_dequeue_tail(lp_qdisc);
    BUG_ON(!to_drop);
    qdisc_qstats_backlog_dec(sch, to_drop);
    qdisc_drop(to_drop, sch, to_free);
    q.qstats[lp].backlog -= qdisc_pkt_len(to_drop);
    q.qstats[lp].drops++;
    q.qstats[lp].overlimits++;
// Check to update highest and lowest priorities.
    if (skb_queue_empty(lp_qdisc)) {
    if (q.lowest_prio == q.highest_prio) {
    q.lowest_prio = prio;
    q.highest_prio = prio;
    } else {
    q.lowest_prio = calc_new_low_prio(q);
    }
    }
    if (prio > q.highest_prio)
    q.highest_prio = prio;
    return NET_XMIT_CN;
    }
    static struct sk_buff *skbprio_dequeue(struct Qdisc *sch)
    {
    struct skbprio_sched_data *q = qdisc_priv(sch);
    struct sk_buff_head *hpq = &q.qdiscs[q.highest_prio];
    struct sk_buff *skb = __skb_dequeue(hpq);
    if (unlikely(!skb))
    return core::ptr::null_mut();
    qdisc_qlen_dec(sch);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
    q.qstats[q.highest_prio].backlog -= qdisc_pkt_len(skb);
// Update highest priority field.
    if (skb_queue_empty(hpq)) {
    if (q.lowest_prio == q.highest_prio) {
    q.highest_prio = 0;
    q.lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
    } else {
    q.highest_prio = calc_new_high_prio(q);
    }
    }
    return skb;
    }
    static int skbprio_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct tc_skbprio_qopt *ctl = nla_data(opt);
    if (opt.nla_len != nla_attr_size(sizeof(*ctl)))
    return -EINVAL;
    WRITE_ONCE(sch.limit, ctl.limit);
    return 0;
    }
    static int skbprio_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct skbprio_sched_data *q = qdisc_priv(sch);
    int prio;
// Initialise all queues, one for each possible priority.
    for (prio = 0; prio < SKBPRIO_MAX_PRIORITY; prio++)
    __skb_queue_head_init(&q.qdiscs[prio]);
    memset(&q.qstats, 0, sizeof(q.qstats));
    q.highest_prio = 0;
    q.lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
    sch.limit = 64;
    if (!opt)
    return 0;
    return skbprio_change(sch, opt, extack);
    }
#[no_mangle]
unsafe extern "C" fn skbprio_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int skbprio_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct tc_skbprio_qopt opt;
    opt.limit = READ_ONCE(sch.limit);
    if (nla_put(skb, TCA_OPTIONS, sizeof(opt), &opt))
    return -1;
    return skb.len;
    }
#[no_mangle]
unsafe extern "C" fn skbprio_reset(sch: *mut Qdisc) {
    static void skbprio_reset(struct Qdisc *sch)
    {
    struct skbprio_sched_data *q = qdisc_priv(sch);
    int prio;
    for (prio = 0; prio < SKBPRIO_MAX_PRIORITY; prio++)
    __skb_queue_purge(&q.qdiscs[prio]);
    memset(&q.qstats, 0, sizeof(q.qstats));
    q.highest_prio = 0;
    q.lowest_prio = SKBPRIO_MAX_PRIORITY - 1;
    }
#[no_mangle]
unsafe extern "C" fn skbprio_destroy(sch: *mut Qdisc) {
    static void skbprio_destroy(struct Qdisc *sch)
    {
    struct skbprio_sched_data *q = qdisc_priv(sch);
    int prio;
    for (prio = 0; prio < SKBPRIO_MAX_PRIORITY; prio++)
    __skb_queue_purge(&q.qdiscs[prio]);
    }
    static struct Qdisc *skbprio_leaf(struct Qdisc *sch, unsigned long arg)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn skbprio_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long skbprio_find(struct Qdisc *sch, u32 classid)
    {
    return 0;
    }
    static int skbprio_dump_class(struct Qdisc *sch, unsigned long cl,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    tcm.tcm_handle |= TC_H_MIN(cl);
    return 0;
    }
    static int skbprio_dump_class_stats(struct Qdisc *sch, unsigned long cl,
    struct gnet_dump *d)
    {
    struct skbprio_sched_data *q = qdisc_priv(sch);
    if (gnet_stats_copy_queue(d, core::ptr::null_mut(), &q.qstats[cl - 1],
    q.qstats[cl - 1].qlen) < 0)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skbprio_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void skbprio_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    unsigned int i;
    if (arg.stop)
    return;
    for (i = 0; i < SKBPRIO_MAX_PRIORITY; i++) {
    if (!tc_qdisc_stats_dump(sch, i + 1, arg))
    break;
    }
    }
    static const struct Qdisc_class_ops skbprio_class_ops = {
    .leaf		=	skbprio_leaf,
    .find		=	skbprio_find,
    .dump		=	skbprio_dump_class,
    .dump_stats	=	skbprio_dump_class_stats,
    .walk		=	skbprio_walk,
    };
    static struct Qdisc_ops skbprio_qdisc_ops __read_mostly = {
    .cl_ops		=	&skbprio_class_ops,
    .id		=	"skbprio",
    .priv_size	=	sizeof(struct skbprio_sched_data),
    .enqueue	=	skbprio_enqueue,
    .dequeue	=	skbprio_dequeue,
    .peek		=	qdisc_peek_dequeued,
    .init		=	skbprio_init,
    .reset		=	skbprio_reset,
    .change		=	skbprio_change,
    .dump		=	skbprio_dump,
    .destroy	=	skbprio_destroy,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("skbprio");
#[no_mangle]
unsafe extern "C" fn skbprio_module_init() -> int __init {
    static int __init skbprio_module_init(void)
    {
    return register_qdisc(&skbprio_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn skbprio_module_exit() -> void __exit {
    static void __exit skbprio_module_exit(void)
    {
    unregister_qdisc(&skbprio_qdisc_ops);
    }
    module_init(skbprio_module_init)
    module_exit(skbprio_module_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("SKB priority based scheduling qdisc");
