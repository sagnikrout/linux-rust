//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_red.c
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
// net/sched/sch_red.c	Random Early Detection queue.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//
// Changes:
// J Hadi Salim 980914:	computation fixes
// Alexey Makarenko <makar@phoenix.kharkov.ua> 990814: qave on idle link was calculated incorrectly.
// J Hadi Salim 980816:  ECN support
//

// Parameters, settable by user:
    -----------------------------
    limit		- bytes (must be > qth_max + burst)
    Hard limit on queue length, should be chosen >qth_max
    to allow packet bursts. This parameter does not
    affect the algorithms behaviour and can be chosen
#[no_mangle]
pub unsafe extern "C" fn high(_arg: well, size: less than ram) -> arbitrarily {
    arbitrarily high (well, less than ram size)
    Really, this limit will never be reached
    if RED works correctly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct red_sched_data {
    pub /: *mut *mut u32 limit; / HARD maximal queue length,
    pub flags: c_uchar,
// Non-flags in tc_red_qopt.flags.
    pub userbits: c_uchar,
    pub adapt_timer: timer_list,
    pub sch: *mut Qdisc,
    pub parms: red_parms,
    pub vars: red_vars,
    pub stats: red_stats,
    pub qdisc: *mut Qdisc,
    pub qe_early_drop: tcf_qevent,
    pub qe_mark: tcf_qevent,
}

#[no_mangle]
pub unsafe extern "C" fn red_use_ecn(q: *mut red_sched_data) -> c_int {
    static inline int red_use_ecn(struct red_sched_data *q)
    {
    return q.flags & TC_RED_ECN;
    }
#[no_mangle]
pub unsafe extern "C" fn red_use_harddrop(q: *mut red_sched_data) -> c_int {
    static inline int red_use_harddrop(struct red_sched_data *q)
    {
    return q.flags & TC_RED_HARDDROP;
    }
#[no_mangle]
unsafe extern "C" fn red_use_nodrop(q: *mut red_sched_data) -> c_int {
    static int red_use_nodrop(struct red_sched_data *q)
    {
    return q.flags & TC_RED_NODROP;
    }
    static int red_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    let mut reason: enum qdisc_drop_reason = QDISC_DROP_CONGESTED;
    struct red_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child = q.qdisc;
    unsigned int len;
    int ret;
    q.vars.qavg = red_calc_qavg(&q.parms,
    &q.vars,
    child.qstats.backlog);
    if (red_is_idling(&q.vars))
    red_end_of_idle_period(&q.vars);
    switch (red_action(&q.parms, &q.vars, q.vars.qavg)) {
    case RED_DONT_MARK:
    break;
    case RED_PROB_MARK:
    qdisc_qstats_overlimit(sch);
    if (!red_use_ecn(q)) {
    WRITE_ONCE(q.stats.prob_drop,
    q.stats.prob_drop + 1);
    goto congestion_drop;
    }
    if (INET_ECN_set_ce(skb)) {
    WRITE_ONCE(q.stats.prob_mark,
    q.stats.prob_mark + 1);
    skb = tcf_qevent_handle(&q.qe_mark, sch, skb, to_free, &ret);
    if (!skb)
    return NET_XMIT_CN | ret;
    } else if (!red_use_nodrop(q)) {
    WRITE_ONCE(q.stats.prob_drop,
    q.stats.prob_drop + 1);
    goto congestion_drop;
    }
// Non-ECT packet in ECN nodrop mode: queue it.
    break;
    case RED_HARD_MARK:
    reason = QDISC_DROP_OVERLIMIT;
    qdisc_qstats_overlimit(sch);
    if (red_use_harddrop(q) || !red_use_ecn(q)) {
    WRITE_ONCE(q.stats.forced_drop,
    q.stats.forced_drop + 1);
    goto congestion_drop;
    }
    if (INET_ECN_set_ce(skb)) {
    WRITE_ONCE(q.stats.forced_mark,
    q.stats.forced_mark + 1);
    skb = tcf_qevent_handle(&q.qe_mark, sch, skb, to_free, &ret);
    if (!skb)
    return NET_XMIT_CN | ret;
    } else if (!red_use_nodrop(q)) {
    WRITE_ONCE(q.stats.forced_drop,
    q.stats.forced_drop + 1);
    goto congestion_drop;
    }
// Non-ECT packet in ECN nodrop mode: queue it.
    break;
    }
    len = qdisc_pkt_len(skb);
    ret = qdisc_enqueue(skb, child, to_free);
    if (likely(ret == NET_XMIT_SUCCESS)) {
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    } else if (net_xmit_drop_count(ret)) {
    WRITE_ONCE(q.stats.pdrop,
    q.stats.pdrop + 1);
    qdisc_qstats_drop(sch);
    }
    return ret;
    congestion_drop:
    skb = tcf_qevent_handle(&q.qe_early_drop, sch, skb, to_free, &ret);
    if (!skb)
    return NET_XMIT_CN | ret;
    qdisc_drop_reason(skb, sch, to_free, reason);
    return NET_XMIT_CN;
    }
    static struct sk_buff *red_dequeue(struct Qdisc *sch)
    {
    struct sk_buff *skb;
    struct red_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child = q.qdisc;
    skb = qdisc_dequeue_peeked(child);
    if (skb) {
    qdisc_bstats_update(sch, skb);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_qlen_dec(sch);
    } else {
    if (!red_is_idling(&q.vars))
    red_start_of_idle_period(&q.vars);
    }
    return skb;
    }
    static struct sk_buff *red_peek(struct Qdisc *sch)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child = q.qdisc;
    return child.ops.peek(child);
    }
#[no_mangle]
unsafe extern "C" fn red_reset(sch: *mut Qdisc) {
    static void red_reset(struct Qdisc *sch)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    qdisc_reset(q.qdisc);
    red_restart(&q.vars);
    }
#[no_mangle]
unsafe extern "C" fn red_offload(sch: *mut Qdisc, enable: bool) -> c_int {
    static int red_offload(struct Qdisc *sch, bool enable)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct net_device *dev = qdisc_dev(sch);
    struct tc_red_qopt_offload opt = {
    .handle = sch.handle,
    .parent = sch.parent,
    };
    if (!tc_can_offload(dev) || !dev.netdev_ops.ndo_setup_tc)
    return -EOPNOTSUPP;
    if (enable) {
    opt.command = TC_RED_REPLACE;
    opt.set.min = q.parms.qth_min >> q.parms.Wlog;
    opt.set.max = q.parms.qth_max >> q.parms.Wlog;
    opt.set.probability = q.parms.max_P;
    opt.set.limit = q.limit;
    opt.set.is_ecn = red_use_ecn(q);
    opt.set.is_harddrop = red_use_harddrop(q);
    opt.set.is_nodrop = red_use_nodrop(q);
    opt.set.qstats = &sch.qstats;
    } else {
    opt.command = TC_RED_DESTROY;
    }
    return dev.netdev_ops.ndo_setup_tc(dev, TC_SETUP_QDISC_RED, &opt);
    }
#[no_mangle]
unsafe extern "C" fn red_destroy(sch: *mut Qdisc) {
    static void red_destroy(struct Qdisc *sch)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    tcf_qevent_destroy(&q.qe_mark, sch);
    tcf_qevent_destroy(&q.qe_early_drop, sch);
    timer_delete_sync(&q.adapt_timer);
    red_offload(sch, false);
    qdisc_put(q.qdisc);
    }
    static const struct nla_policy red_policy[TCA_RED_MAX + 1] = {
    [TCA_RED_UNSPEC] = { .strict_start_type = TCA_RED_FLAGS },
    [TCA_RED_PARMS]	= { .len = sizeof(struct tc_red_qopt) },
    [TCA_RED_STAB]	= { .len = RED_STAB_SIZE },
    [TCA_RED_MAX_P] = { .type = NLA_U32 },
    [TCA_RED_FLAGS] = NLA_POLICY_BITFIELD32(TC_RED_SUPPORTED_FLAGS),
    [TCA_RED_EARLY_DROP_BLOCK] = { .type = NLA_U32 },
    [TCA_RED_MARK_BLOCK] = { .type = NLA_U32 },
    };
    static int __red_change(struct Qdisc *sch, struct nlattr **tb,
    struct netlink_ext_ack *extack)
    {
    struct Qdisc *old_child = core::ptr::null_mut(), *child = core::ptr::null_mut();
    struct red_sched_data *q = qdisc_priv(sch);
    struct nla_bitfield32 flags_bf;
    struct tc_red_qopt *ctl;
    unsigned char userbits;
    unsigned char flags;
    int err;
    u32 max_P;
    u8 *stab;
    if (tb[TCA_RED_PARMS] == core::ptr::null_mut() ||
    tb[TCA_RED_STAB] == core::ptr::null_mut())
    return -EINVAL;
    max_P = nla_get_u32_default(tb[TCA_RED_MAX_P], 0);
    ctl = nla_data(tb[TCA_RED_PARMS]);
    stab = nla_data(tb[TCA_RED_STAB]);
    if (!red_check_params(ctl.qth_min, ctl.qth_max, ctl.Wlog,
    ctl.Scell_log, stab))
    return -EINVAL;
    err = red_get_flags(ctl.flags, TC_RED_HISTORIC_FLAGS,
    tb[TCA_RED_FLAGS], TC_RED_SUPPORTED_FLAGS,
    &flags_bf, &userbits, extack);
    if (err)
    return err;
    if (ctl.limit > 0) {
    child = fifo_create_dflt(sch, &bfifo_qdisc_ops, ctl.limit,
    extack);
    if (IS_ERR(child))
    return PTR_ERR(child);
// child is fifo, no need to check for noop_qdisc
    qdisc_hash_add(child, true);
    }
    sch_tree_lock(sch);
    flags = (q.flags & ~flags_bf.selector) | flags_bf.value;
    err = red_validate_flags(flags, extack);
    if (err)
    goto unlock_out;
    q.flags = flags;
    q.userbits = userbits;
    q.limit = ctl.limit;
    if (child) {
    qdisc_purge_queue(q.qdisc);
    old_child = q.qdisc;
    q.qdisc = child;
    }
    red_set_parms(&q.parms,
    ctl.qth_min, ctl.qth_max, ctl.Wlog,
    ctl.Plog, ctl.Scell_log,
    stab,
    max_P);
    red_set_vars(&q.vars);
    timer_delete(&q.adapt_timer);
    if (ctl.flags & TC_RED_ADAPTATIVE)
    mod_timer(&q.adapt_timer, jiffies + HZ/2);
    if (!q.qdisc.q.qlen)
    red_start_of_idle_period(&q.vars);
    sch_tree_unlock(sch);
    red_offload(sch, true);
    if (old_child)
    qdisc_put(old_child);
    return 0;
    unlock_out:
    sch_tree_unlock(sch);
    if (child)
    qdisc_put(child);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn red_adaptative_timer(t: *mut timer_list) {
    static inline void red_adaptative_timer(struct timer_list *t)
    {
    struct red_sched_data *q = timer_container_of(q, t, adapt_timer);
    struct Qdisc *sch = q.sch;
    spinlock_t *root_lock;
    rcu_read_lock();
    root_lock = qdisc_lock(qdisc_root_sleeping(sch));
    spin_lock(root_lock);
    red_adaptative_algo(&q.parms, &q.vars);
    mod_timer(&q.adapt_timer, jiffies + HZ/2);
    spin_unlock(root_lock);
    rcu_read_unlock();
    }
    static int red_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct nlattr *tb[TCA_RED_MAX + 1];
    int err;
    q.qdisc = &noop_qdisc;
    q.sch = sch;
    timer_setup(&q.adapt_timer, red_adaptative_timer, 0);
    if (!opt)
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_RED_MAX, opt, red_policy,
    extack);
    if (err < 0)
    return err;
    err = __red_change(sch, tb, extack);
    if (err)
    return err;
    err = tcf_qevent_init(&q.qe_early_drop, sch,
    FLOW_BLOCK_BINDER_TYPE_RED_EARLY_DROP,
    tb[TCA_RED_EARLY_DROP_BLOCK], extack);
    if (err)
    return err;
    return tcf_qevent_init(&q.qe_mark, sch,
    FLOW_BLOCK_BINDER_TYPE_RED_MARK,
    tb[TCA_RED_MARK_BLOCK], extack);
    }
    static int red_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct nlattr *tb[TCA_RED_MAX + 1];
    int err;
    err = nla_parse_nested_deprecated(tb, TCA_RED_MAX, opt, red_policy,
    extack);
    if (err < 0)
    return err;
    err = tcf_qevent_validate_change(&q.qe_early_drop,
    tb[TCA_RED_EARLY_DROP_BLOCK], extack);
    if (err)
    return err;
    err = tcf_qevent_validate_change(&q.qe_mark,
    tb[TCA_RED_MARK_BLOCK], extack);
    if (err)
    return err;
    return __red_change(sch, tb, extack);
    }
#[no_mangle]
unsafe extern "C" fn red_dump_offload_stats(sch: *mut Qdisc) -> c_int {
    static int red_dump_offload_stats(struct Qdisc *sch)
    {
    struct tc_red_qopt_offload hw_stats = {
    .command = TC_RED_STATS,
    .handle = sch.handle,
    .parent = sch.parent,
    {
    .stats.bstats = &sch.bstats,
    .stats.qstats = &sch.qstats,
    },
    };
    return qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_RED, &hw_stats);
    }
#[no_mangle]
unsafe extern "C" fn red_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int red_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct nlattr *opts = core::ptr::null_mut();
    struct tc_red_qopt opt = {
    .limit		= q.limit,
    .flags		= (q.flags & TC_RED_HISTORIC_FLAGS) |
    q.userbits,
    .qth_min	= q.parms.qth_min >> q.parms.Wlog,
    .qth_max	= q.parms.qth_max >> q.parms.Wlog,
    .Wlog		= q.parms.Wlog,
    .Plog		= q.parms.Plog,
    .Scell_log	= q.parms.Scell_log,
    };
    int err;
    err = red_dump_offload_stats(sch);
    if (err)
    goto nla_put_failure;
    opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (opts == core::ptr::null_mut())
    goto nla_put_failure;
    if (nla_put(skb, TCA_RED_PARMS, sizeof(opt), &opt) ||
    nla_put_u32(skb, TCA_RED_MAX_P, q.parms.max_P) ||
    nla_put_bitfield32(skb, TCA_RED_FLAGS,
    q.flags, TC_RED_SUPPORTED_FLAGS) ||
    tcf_qevent_dump(skb, TCA_RED_MARK_BLOCK, &q.qe_mark) ||
    tcf_qevent_dump(skb, TCA_RED_EARLY_DROP_BLOCK, &q.qe_early_drop))
    goto nla_put_failure;
    return nla_nest_end(skb, opts);
    nla_put_failure:
    nla_nest_cancel(skb, opts);
    return -EMSGSIZE;
    }
#[no_mangle]
unsafe extern "C" fn red_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> c_int {
    static int red_dump_stats(struct Qdisc *sch, struct gnet_dump *d)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    struct net_device *dev = qdisc_dev(sch);
    let mut st: tc_red_xstats = {0};
    if (sch.flags & TCQ_F_OFFLOADED) {
    struct tc_red_qopt_offload hw_stats_request = {
    .command = TC_RED_XSTATS,
    .handle = sch.handle,
    .parent = sch.parent,
    {
    .xstats = &q.stats,
    },
    };
    dev.netdev_ops.ndo_setup_tc(dev, TC_SETUP_QDISC_RED,
    &hw_stats_request);
    }
    st.early = READ_ONCE(q.stats.prob_drop) +
    READ_ONCE(q.stats.forced_drop);
    st.pdrop = READ_ONCE(q.stats.pdrop);
    st.marked = READ_ONCE(q.stats.prob_mark) +
    READ_ONCE(q.stats.forced_mark);
    return gnet_stats_copy_app(d, &st, sizeof(st));
    }
    static int red_dump_class(struct Qdisc *sch, unsigned long cl,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    tcm.tcm_handle |= TC_H_MIN(1);
    tcm.tcm_info = q.qdisc.handle;
    return 0;
    }
    static void red_graft_offload(struct Qdisc *sch,
    struct Qdisc *new, struct Qdisc *old,
    struct netlink_ext_ack *extack)
    {
    struct tc_red_qopt_offload graft_offload = {
    .handle		= sch.handle,
    .parent		= sch.parent,
    .child_handle	= new.handle,
    .command	= TC_RED_GRAFT,
    };
    qdisc_offload_graft_helper(qdisc_dev(sch), sch, new, old,
    TC_SETUP_QDISC_RED, &graft_offload, extack);
    }
    static int red_graft(struct Qdisc *sch, unsigned long arg, struct Qdisc *new,
    struct Qdisc **old, struct netlink_ext_ack *extack)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    if (new == core::ptr::null_mut())
    new = &noop_qdisc;
// old = qdisc_replace(sch, new, &q->qdisc);
    red_graft_offload(sch, new, *old, extack);
    return 0;
    }
    static struct Qdisc *red_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct red_sched_data *q = qdisc_priv(sch);
    return q.qdisc;
    }
#[no_mangle]
unsafe extern "C" fn red_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long red_find(struct Qdisc *sch, u32 classid)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn red_walk(sch: *mut Qdisc, walker: *mut qdisc_walker) {
    static void red_walk(struct Qdisc *sch, struct qdisc_walker *walker)
    {
    if (!walker.stop) {
    tc_qdisc_stats_dump(sch, 1, walker);
    }
    }
    static const struct Qdisc_class_ops red_class_ops = {
    .graft		=	red_graft,
    .leaf		=	red_leaf,
    .find		=	red_find,
    .walk		=	red_walk,
    .dump		=	red_dump_class,
    };
    static struct Qdisc_ops red_qdisc_ops __read_mostly = {
    .id		=	"red",
    .priv_size	=	sizeof(struct red_sched_data),
    .cl_ops		=	&red_class_ops,
    .enqueue	=	red_enqueue,
    .dequeue	=	red_dequeue,
    .peek		=	red_peek,
    .init		=	red_init,
    .reset		=	red_reset,
    .destroy	=	red_destroy,
    .change		=	red_change,
    .dump		=	red_dump,
    .dump_stats	=	red_dump_stats,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("red");
#[no_mangle]
unsafe extern "C" fn red_module_init() -> int __init {
    static int __init red_module_init(void)
    {
    return register_qdisc(&red_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn red_module_exit() -> void __exit {
    static void __exit red_module_exit(void)
    {
    unregister_qdisc(&red_qdisc_ops);
    }
    module_init(red_module_init)
    module_exit(red_module_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Random Early Detection qdisc");
