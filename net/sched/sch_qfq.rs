//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_qfq.c
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
// net/sched/sch_qfq.c         Quick Fair Queueing Plus Scheduler.
//
// Copyright (c) 2009 Fabio Checconi, Luigi Rizzo, and Paolo Valente.
// Copyright (c) 2012 Paolo Valente.
//

// Quick Fair Queueing Plus
    ========================
    Sources:
    [1] Paolo Valente,
    "Reducing the Execution Time of Fair-Queueing Schedulers."
    http://algo.ing.unimo.it/people/paolo/agg-sched/agg-sched.pdf
    Sources for QFQ:
    [2] Fabio Checconi, Luigi Rizzo, and Paolo Valente: "QFQ: Efficient
    Packet Scheduling with Tight Bandwidth Distribution Guarantees."
    See also:
    http://retis.sssup.it/~fabio/linux/qfq/
//
    QFQ+ divides classes into aggregates of at most MAX_AGG_CLASSES
    classes. Each aggregate is timestamped with a virtual start time S
    and a virtual finish time F, and scheduled according to its
    timestamps. S and F are computed as a function of a system virtual
    time function V. The classes within each aggregate are instead
    scheduled with DRR.
    To speed up operations, QFQ+ divides also aggregates into a limited
    number of groups. Which group a class belongs to depends on the
    ratio between the maximum packet length for the class and the weight
    of the class. Groups have their own S and F. In the end, QFQ+
    schedules groups, then aggregates within groups, then classes within
    aggregates. See [1] and [2] for a full description.
    Virtual time computations.
    S, F and V are all computed in fixed point arithmetic with
    FRAC_BITS decimal bits.
    QFQ_MAX_INDEX is the maximum index allowed for a group. We need
    one bit per index.
    QFQ_MAX_WSHIFT is the maximum power of two supported as a weight.
    The layout of the bits is as below:
    [ MTU_SHIFT ][      FRAC_BITS    ]
    [ MAX_INDEX    ][ MIN_SLOT_SHIFT ]
    ^.__grp.index = 0
// .__grp->slot_shift
    where MIN_SLOT_SHIFT is derived by difference from the others.
    The max group index corresponds to Lmax/w_min, where
    Lmax=1<<MTU_SHIFT, w_min = 1 .
    From this, and knowing how many groups (MAX_INDEX) we want,
    we can derive the shift corresponding to each group.
    Because we often need to compute
    F = S + len/w_i  and V = V + len/wsum
    instead of storing w_i store the value
    inv_w = (1<<FRAC_BITS)/w_i
    so we can do F = S + len * inv_w * wsum.
    We use W_TOT in the formulas so we can easily move between
    static and adaptive weight sum.
    The per-scheduler-instance data contain all the data structures
    for the scheduler: bitmaps and bucket lists.
//
// Maximum number of consecutive slots occupied by backlogged classes
// inside a group.
//
pub const QFQ_MAX_SLOTS: c_int = 32;
//
// Shifts used for aggregate<->group mapping.  We allow class weights that are
// in the range [1, 2^MAX_WSHIFT], and we try to map each aggregate i to the
// group with the smallest index that can support the L_i / r_i configured
// for the classes in the aggregate.
//
// grp->index is the index of the group; and grp->slot_shift
// is the shift for the corresponding (scaled) sigma_i.
//
pub const QFQ_MAX_INDEX: c_int = 24;
pub const QFQ_MAX_WSHIFT: c_int = 10;

//
// Possible group states.  These values are used as indexes for the bitmaps
// array of struct qfq_queue.
//
    enum qfq_state { ER, IR, EB, IB, QFQ_MAX_STATE };
    struct qfq_group;
    struct qfq_aggregate;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qfq_class {
    pub common: Qdisc_class_common,
    pub bstats: gnet_stats_basic_sync,
    pub qstats: gnet_stats_queue,
    pub rate_est: *mut net_rate_estimator __rcu,
    pub qdisc: *mut Qdisc,
    pub /: *mut *mut list_head alist; / Link for active-classes list.,
    pub /: *mut *mut *mut qfq_aggregate agg; / Parent aggregate.,
    pub /: *mut *mut int deficit; / DRR deficit counter.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qfq_aggregate {
    pub /: *mut *mut hlist_node next; / Link for the slot list.,
    pub /: *mut *mut u64 S, F; / flow timestamps (exact),
// group we belong to. In principle we would need the index,
// which is log_2(lmax/weight), but we never reference it
// directly, only the group.
//
    pub grp: *mut qfq_group,
// these are copied from the flowset.
    pub /: *mut *mut u32 class_weight; / Weight of each class in this aggregate.,
// Max pkt size for the classes in this aggregate, DRR quantum.
    pub lmax: c_int,
    pub /: *mut *mut u32 inv_w; / ONE_FP/(sum of weights of classes in aggr.).,
    pub /: *mut *mut u32 budgetmax; / Max budget for this aggregate.,
    pub /: *mut *mut u32 initial_budget, budget; / Initial and current budget.,
    pub /: *mut *mut int num_classes; / Number of classes in this aggr.,
    pub /: *mut *mut list_head active; / DRR queue of active classes.,
    pub /: *mut *mut hlist_node nonfull_next; / See nonfull_aggs in qfq_sched.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qfq_group {
    pub /: *mut *mut u64 S, F; / group timestamps (approx).,
    pub /: *mut *mut unsigned int slot_shift; / Slot shift.,
    pub /: *mut *mut unsigned int index; / Group index.,
    pub /: *mut *mut unsigned int front; / Index of the front slot.,
    pub /: *mut *mut unsigned long full_slots; / non-empty slots,
// Array of RR lists of active aggregates.
    pub slots: [hlist_head; QFQ_MAX_SLOTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qfq_sched {
    pub filter_list: *mut tcf_proto __rcu,
    pub block: *mut tcf_block,
    pub clhash: Qdisc_class_hash,
    pub /: *mut *mut u64 oldV, V; / Precise virtual times.,
    pub /: *mut *mut *mut qfq_aggregate in_serv_agg; / Aggregate being served.,
    pub /: *mut *mut u32 wsum; / weight sum,
    pub /: *mut *mut u32 iwsum; / inverse weight sum,
    pub /: *mut *mut unsigned long bitmaps[QFQ_MAX_STATE]; / Group bitmaps.,
    pub /: *mut *mut qfq_group groups[QFQ_MAX_INDEX + 1]; / The groups.,
    pub /: *mut *mut u32 min_slot_shift; / Index of the group-0 bit in the bitmaps.,
    pub /: *mut *mut u32 max_agg_classes; / Max number of classes per aggr.,
    pub /: *mut *mut hlist_head nonfull_aggs; / Aggs with room for more classes.,
}

//
// Possible reasons why the timestamps of an aggregate are updated
// enqueue: the aggregate switches from idle to active and must scheduled
// for service
// requeue: the aggregate finishes its budget, so it stops being served and
// must be rescheduled for service
//
    enum update_reason {enqueue, requeue};
#[no_mangle]
unsafe extern "C" fn cl_is_active(cl: *mut qfq_class) -> bool {
    static bool cl_is_active(struct qfq_class *cl)
    {
    return !list_empty(&cl.alist);
    }
    static struct qfq_class *qfq_find_class(struct Qdisc *sch, u32 classid)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct Qdisc_class_common *clc;
    clc = qdisc_class_find(&q.clhash, classid);
    if (clc == core::ptr::null_mut())
    return core::ptr::null_mut();
    return container_of(clc, struct qfq_class, common);
    }
    static const struct netlink_range_validation lmax_range = {
    .min = QFQ_MIN_LMAX,
    .max = QFQ_MAX_LMAX,
    };
    static const struct nla_policy qfq_policy[TCA_QFQ_MAX + 1] = {
    [TCA_QFQ_WEIGHT] = NLA_POLICY_RANGE(NLA_U32, 1, QFQ_MAX_WEIGHT),
    [TCA_QFQ_LMAX] = NLA_POLICY_FULL_RANGE(NLA_U32, &lmax_range),
    };
//
// Calculate a flow index, given its weight and maximum packet length.
// index = log_2(maxlen/weight) but we need to apply the scaling.
// This is used only once at flow creation.
//
#[no_mangle]
unsafe extern "C" fn qfq_calc_index(inv_w: u32, maxlen: c_uint, min_slot_shift: u32) -> c_int {
    static int qfq_calc_index(u32 inv_w, unsigned int maxlen, u32 min_slot_shift)
    {
    let mut slot_size: u64 = (u64)maxlen * inv_w;
    unsigned long size_map;
    let mut index: c_int = 0;
    size_map = slot_size >> min_slot_shift;
    if (!size_map)
    goto out;
    index = __fls(size_map) + 1;	/* basically a log_2 */
    index -= !(slot_size - (1ULL << (index + min_slot_shift - 1)));
    if (index < 0)
    index = 0;
    out:
    pr_debug("qfq calc_index: W = %lu, L = %u, I = %d\n",
    (unsigned long) ONE_FP/inv_w, maxlen, index);
    return index;
    }
    static void qfq_deactivate_agg(struct qfq_sched *, struct qfq_aggregate *);
    static void qfq_activate_agg(struct qfq_sched *, struct qfq_aggregate *,
    enum update_reason);
    static void qfq_init_agg(struct qfq_sched *q, struct qfq_aggregate *agg,
    u32 lmax, u32 weight)
    {
    INIT_LIST_HEAD(&agg.active);
    hlist_add_head(&agg.nonfull_next, &q.nonfull_aggs);
    agg.lmax = lmax;
    agg.class_weight = weight;
    }
    static struct qfq_aggregate *qfq_find_agg(struct qfq_sched *q,
    u32 lmax, u32 weight)
    {
    struct qfq_aggregate *agg;
    hlist_for_each_entry(agg, &q.nonfull_aggs, nonfull_next)
    if (agg.lmax == lmax && agg.class_weight == weight)
    return agg;
    return core::ptr::null_mut();
    }
// Update aggregate as a function of the new number of classes.
    static void qfq_update_agg(struct qfq_sched *q, struct qfq_aggregate *agg,
    int new_num_classes)
    {
    u32 new_agg_weight;
    if (new_num_classes == q.max_agg_classes)
    hlist_del_init(&agg.nonfull_next);
    if (agg.num_classes > new_num_classes &&
    new_num_classes == q.max_agg_classes - 1) /* agg no more full */
    hlist_add_head(&agg.nonfull_next, &q.nonfull_aggs);
// The next assignment may let
// agg->initial_budget > agg->budgetmax
// hold, we will take it into account in charge_actual_service().
//
    agg.budgetmax = new_num_classes * agg.lmax;
    new_agg_weight = agg.class_weight * new_num_classes;
    agg.inv_w = ONE_FP/new_agg_weight;
    if (agg.grp == core::ptr::null_mut()) {
    int i = qfq_calc_index(agg.inv_w, agg.budgetmax,
    q.min_slot_shift);
    agg.grp = &q.groups[i];
    }
    q.wsum +=
    (int) agg.class_weight * (new_num_classes - agg.num_classes);
    q.iwsum = ONE_FP / q.wsum;
    agg.num_classes = new_num_classes;
    }
// Add class to aggregate.
    static void qfq_add_to_agg(struct qfq_sched *q,
    struct qfq_aggregate *agg,
    struct qfq_class *cl)
    {
    cl.agg = agg;
    qfq_update_agg(q, agg, agg.num_classes+1);
    if (cl.qdisc.q.qlen > 0) { /* adding an active class */
    list_add_tail(&cl.alist, &agg.active);
    if (list_first_entry(&agg.active, struct qfq_class, alist) ==
    cl && q.in_serv_agg != agg) /* agg was inactive */
    qfq_activate_agg(q, agg, enqueue); /* schedule agg */
    }
    }
    static struct qfq_aggregate *qfq_choose_next_agg(struct qfq_sched *);
#[no_mangle]
unsafe extern "C" fn qfq_destroy_agg(q: *mut qfq_sched, agg: *mut qfq_aggregate) {
    static void qfq_destroy_agg(struct qfq_sched *q, struct qfq_aggregate *agg)
    {
    hlist_del_init(&agg.nonfull_next);
    q.wsum -= agg.class_weight;
    if (q.wsum != 0)
    q.iwsum = ONE_FP / q.wsum;
    if (q.in_serv_agg == agg)
    q.in_serv_agg = qfq_choose_next_agg(q);
    kfree(agg);
    }
// Deschedule class from within its parent aggregate.
#[no_mangle]
unsafe extern "C" fn qfq_deactivate_class(q: *mut qfq_sched, cl: *mut qfq_class) {
    static void qfq_deactivate_class(struct qfq_sched *q, struct qfq_class *cl)
    {
    struct qfq_aggregate *agg = cl.agg;
    list_del_init(&cl.alist); /* remove from RR queue of the aggregate */
    if (list_empty(&agg.active)) /* agg is now inactive */
    qfq_deactivate_agg(q, agg);
    }
// Remove class from its parent aggregate.
#[no_mangle]
unsafe extern "C" fn qfq_rm_from_agg(q: *mut qfq_sched, cl: *mut qfq_class) {
    static void qfq_rm_from_agg(struct qfq_sched *q, struct qfq_class *cl)
    {
    struct qfq_aggregate *agg = cl.agg;
    cl.agg = core::ptr::null_mut();
    if (agg.num_classes == 1) { /* agg being emptied, destroy it */
    qfq_destroy_agg(q, agg);
    return;
    }
    qfq_update_agg(q, agg, agg.num_classes-1);
    }
// Deschedule class and remove it from its parent aggregate.
#[no_mangle]
unsafe extern "C" fn qfq_deact_rm_from_agg(q: *mut qfq_sched, cl: *mut qfq_class) {
    static void qfq_deact_rm_from_agg(struct qfq_sched *q, struct qfq_class *cl)
    {
    if (cl_is_active(cl)) /* class is active */
    qfq_deactivate_class(q, cl);
    qfq_rm_from_agg(q, cl);
    }
// Move class to a new aggregate, matching the new class weight and/or lmax
    static int qfq_change_agg(struct Qdisc *sch, struct qfq_class *cl, u32 weight,
    u32 lmax)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_aggregate *new_agg;
// 'lmax' can range from [QFQ_MIN_LMAX, pktlen + stab overhead]
    if (lmax > QFQ_MAX_LMAX)
    return -EINVAL;
    new_agg = qfq_find_agg(q, lmax, weight);
    if (new_agg == core::ptr::null_mut()) { /* create new aggregate */
    new_agg = kzalloc_obj(*new_agg, GFP_ATOMIC);
    if (new_agg == core::ptr::null_mut())
    return -ENOBUFS;
    qfq_init_agg(q, new_agg, lmax, weight);
    }
    qfq_deact_rm_from_agg(q, cl);
    qfq_add_to_agg(q, new_agg, cl);
    return 0;
    }
    static int qfq_change_class(struct Qdisc *sch, u32 classid, u32 parentid,
    struct nlattr **tca, unsigned long *arg,
    struct netlink_ext_ack *extack)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl = (struct qfq_class *)*arg;
    let mut existing: bool = false;
    struct nlattr *tb[TCA_QFQ_MAX + 1];
    struct qfq_aggregate *new_agg = core::ptr::null_mut();
    u32 weight, lmax, inv_w, old_weight, old_lmax;
    int err;
    int delta_w;
    if (NL_REQ_ATTR_CHECK(extack, core::ptr::null_mut(), tca, TCA_OPTIONS)) {
    NL_SET_ERR_MSG_MOD(extack, "missing options");
    return -EINVAL;
    }
    err = nla_parse_nested_deprecated(tb, TCA_QFQ_MAX, tca[TCA_OPTIONS],
    qfq_policy, extack);
    if (err < 0)
    return err;
    weight = nla_get_u32_default(tb[TCA_QFQ_WEIGHT], 1);
    if (tb[TCA_QFQ_LMAX]) {
    lmax = nla_get_u32(tb[TCA_QFQ_LMAX]);
    } else {
// MTU size is user controlled
    lmax = psched_mtu(qdisc_dev(sch));
    if (lmax < QFQ_MIN_LMAX || lmax > QFQ_MAX_LMAX) {
    NL_SET_ERR_MSG_MOD(extack,
    "MTU size out of bounds for qfq");
    return -EINVAL;
    }
    }
    inv_w = ONE_FP / weight;
    weight = ONE_FP / inv_w;
    if (cl != core::ptr::null_mut()) {
    sch_tree_lock(sch);
    old_weight = cl.agg.class_weight;
    old_lmax   = cl.agg.lmax;
    sch_tree_unlock(sch);
    if (lmax == old_lmax && weight == old_weight)
    return 0; /* nothing to change */
    }
    delta_w = weight - (cl ? old_weight : 0);
    if (q.wsum + delta_w > QFQ_MAX_WSUM) {
    NL_SET_ERR_MSG_FMT_MOD(extack,
    "total weight out of range (%d + %u)",
    delta_w, q.wsum);
    return -EINVAL;
    }
    if (cl != core::ptr::null_mut()) { /* modify existing class */
    if (tca[TCA_RATE]) {
    err = gen_replace_estimator(&cl.bstats, core::ptr::null_mut(),
    &cl.rate_est,
    core::ptr::null_mut(),
    true,
    tca[TCA_RATE]);
    if (err)
    return err;
    }
    existing = true;
    goto set_change_agg;
    }
// create and init new class
    cl = kzalloc_obj(struct qfq_class);
    if (cl == core::ptr::null_mut())
    return -ENOBUFS;
    gnet_stats_basic_sync_init(&cl.bstats);
    cl.common.classid = classid;
    cl.deficit = lmax;
    INIT_LIST_HEAD(&cl.alist);
    cl.qdisc = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    classid, core::ptr::null_mut());
    if (cl.qdisc == core::ptr::null_mut())
    cl.qdisc = &noop_qdisc;
    if (tca[TCA_RATE]) {
    err = gen_new_estimator(&cl.bstats, core::ptr::null_mut(),
    &cl.rate_est,
    core::ptr::null_mut(),
    true,
    tca[TCA_RATE]);
    if (err)
    goto destroy_class;
    }
    if (cl.qdisc != &noop_qdisc)
    qdisc_hash_add(cl.qdisc, true);
    set_change_agg:
    sch_tree_lock(sch);
    new_agg = qfq_find_agg(q, lmax, weight);
    if (new_agg == core::ptr::null_mut()) { /* create new aggregate */
    sch_tree_unlock(sch);
    new_agg = kzalloc_obj(*new_agg);
    if (new_agg == core::ptr::null_mut()) {
    err = -ENOBUFS;
    gen_kill_estimator(&cl.rate_est);
    goto destroy_class;
    }
    sch_tree_lock(sch);
    qfq_init_agg(q, new_agg, lmax, weight);
    }
    if (existing)
    qfq_deact_rm_from_agg(q, cl);
    else
    qdisc_class_hash_insert(&q.clhash, &cl.common);
    qfq_add_to_agg(q, new_agg, cl);
    sch_tree_unlock(sch);
    qdisc_class_hash_grow(sch, &q.clhash);
// arg = (unsigned long)cl;
    return 0;
    destroy_class:
    if (!existing) {
    qdisc_put(cl.qdisc);
    kfree(cl);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn qfq_destroy_class(sch: *mut Qdisc, cl: *mut qfq_class) {
    static void qfq_destroy_class(struct Qdisc *sch, struct qfq_class *cl)
    {
    gen_kill_estimator(&cl.rate_est);
    qdisc_put(cl.qdisc);
    kfree(cl);
    }
    static int qfq_delete_class(struct Qdisc *sch, unsigned long arg,
    struct netlink_ext_ack *extack)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl = (struct qfq_class *)arg;
    if (qdisc_class_in_use(&cl.common)) {
    NL_SET_ERR_MSG_MOD(extack, "QFQ class in use");
    return -EBUSY;
    }
    sch_tree_lock(sch);
    qdisc_purge_queue(cl.qdisc);
    qdisc_class_hash_remove(&q.clhash, &cl.common);
    qfq_rm_from_agg(q, cl);
    sch_tree_unlock(sch);
    qfq_destroy_class(sch, cl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qfq_search_class(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long qfq_search_class(struct Qdisc *sch, u32 classid)
    {
    return (unsigned long)qfq_find_class(sch, classid);
    }
    static struct tcf_block *qfq_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    if (cl)
    return core::ptr::null_mut();
    return q.block;
    }
    static unsigned long qfq_bind_tcf(struct Qdisc *sch, unsigned long parent,
    u32 classid)
    {
    struct qfq_class *cl = qfq_find_class(sch, classid);
    if (cl)
    qdisc_class_get(&cl.common);
    return (unsigned long)cl;
    }
#[no_mangle]
unsafe extern "C" fn qfq_unbind_tcf(sch: *mut Qdisc, arg: c_ulong) {
    static void qfq_unbind_tcf(struct Qdisc *sch, unsigned long arg)
    {
    struct qfq_class *cl = (struct qfq_class *)arg;
    qdisc_class_put(&cl.common);
    }
    static int qfq_graft_class(struct Qdisc *sch, unsigned long arg,
    struct Qdisc *new, struct Qdisc **old,
    struct netlink_ext_ack *extack)
    {
    struct qfq_class *cl = (struct qfq_class *)arg;
    if (new == core::ptr::null_mut()) {
    new = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    cl.common.classid, core::ptr::null_mut());
    if (new == core::ptr::null_mut())
    new = &noop_qdisc;
    }
// old = qdisc_replace(sch, new, &cl->qdisc);
    return 0;
    }
    static struct Qdisc *qfq_class_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct qfq_class *cl = (struct qfq_class *)arg;
    return cl.qdisc;
    }
    static int qfq_dump_class(struct Qdisc *sch, unsigned long arg,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    struct qfq_class *cl = (struct qfq_class *)arg;
    struct nlattr *nest;
    u32 class_weight, lmax;
    tcm.tcm_parent	= TC_H_ROOT;
    tcm.tcm_handle	= cl.common.classid;
    tcm.tcm_info	= cl.qdisc.handle;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (nest == core::ptr::null_mut())
    goto nla_put_failure;
    sch_tree_lock(sch);
    class_weight	= cl.agg.class_weight;
    lmax		= cl.agg.lmax;
    sch_tree_unlock(sch);
    if (nla_put_u32(skb, TCA_QFQ_WEIGHT, class_weight) ||
    nla_put_u32(skb, TCA_QFQ_LMAX, lmax))
    goto nla_put_failure;
    return nla_nest_end(skb, nest);
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int qfq_dump_class_stats(struct Qdisc *sch, unsigned long arg,
    struct gnet_dump *d)
    {
    struct qfq_class *cl = (struct qfq_class *)arg;
    struct tc_qfq_stats xstats;
    memset(&xstats, 0, sizeof(xstats));
    sch_tree_lock(sch);
    xstats.weight = cl.agg.class_weight;
    xstats.lmax = cl.agg.lmax;
    sch_tree_unlock(sch);
    if (gnet_stats_copy_basic(d, core::ptr::null_mut(), &cl.bstats, true) < 0 ||
    gnet_stats_copy_rate_est(d, &cl.rate_est) < 0 ||
    qdisc_qstats_copy(d, cl.qdisc) < 0)
    return -1;
    return gnet_stats_copy_app(d, &xstats, sizeof(xstats));
    }
#[no_mangle]
unsafe extern "C" fn qfq_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void qfq_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl;
    unsigned int i;
    if (arg.stop)
    return;
    for (i = 0; i < q.clhash.hashsize; i++) {
    hlist_for_each_entry(cl, &q.clhash.hash[i], common.hnode) {
    if (!tc_qdisc_stats_dump(sch, (unsigned long)cl, arg))
    return;
    }
    }
    }
    static struct qfq_class *qfq_classify(struct sk_buff *skb, struct Qdisc *sch,
    int *qerr)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl;
    struct tcf_result res;
    struct tcf_proto *fl;
    int result;
    if (TC_H_MAJ(skb.priority ^ sch.handle) == 0) {
    pr_debug("qfq_classify: found %d\n", skb.priority);
    cl = qfq_find_class(sch, skb.priority);
    if (cl != core::ptr::null_mut())
    return cl;
    }
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    fl = rcu_dereference_bh(q.filter_list);
    result = tcf_classify_qdisc(skb, fl, &res, false);
    if (result >= 0) {

    switch (result) {
    case TC_ACT_QUEUED:
    case TC_ACT_STOLEN:
    case TC_ACT_TRAP:
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
    fallthrough;
    case TC_ACT_SHOT:
    return core::ptr::null_mut();
    }

    cl = (struct qfq_class *)res.class;
    if (cl == core::ptr::null_mut())
    cl = qfq_find_class(sch, res.classid);
    return cl;
    }
    return core::ptr::null_mut();
    }
// Generic comparison function, handling wraparound.
#[no_mangle]
pub unsafe extern "C" fn qfq_gt(a: u64, b: u64) -> c_int {
    static inline int qfq_gt(u64 a, u64 b)
    {
    return (s64)(a - b) > 0;
    }
// Round a precise timestamp to its slotted value.
#[no_mangle]
pub unsafe extern "C" fn qfq_round_down(ts: u64, shift: c_uint) -> u64 {
    static inline u64 qfq_round_down(u64 ts, unsigned int shift)
    {
    return ts & ~((1ULL << shift) - 1);
    }
// return the pointer to the group with lowest index in the bitmap
    static inline struct qfq_group *qfq_ffs(struct qfq_sched *q,
    unsigned long bitmap)
    {
    let mut index: c_int = __ffs(bitmap);
    return &q.groups[index];
    }
// Calculate a mask to mimic what would be ffs_from().
#[no_mangle]
pub unsafe extern "C" fn mask_from(bitmap: c_ulong, from: c_int) -> c_ulong {
    static inline unsigned long mask_from(unsigned long bitmap, int from)
    {
    return bitmap & ~((1UL << from) - 1);
    }
//
// The state computation relies on ER=0, IR=1, EB=2, IB=3
// First compute eligibility comparing grp->S, q->V,
// then check if someone is blocking us and possibly add EB
//
#[no_mangle]
unsafe extern "C" fn qfq_calc_state(q: *mut qfq_sched, grp: *const qfq_group) -> c_int {
    static int qfq_calc_state(struct qfq_sched *q, const struct qfq_group *grp)
    {
// if S > V we are not eligible
    let mut state: c_uint = qfq_gt(grp.S, q.V);
    let mut mask: c_ulong = mask_from(q.bitmaps[ER], grp.index);
    struct qfq_group *next;
    if (mask) {
    next = qfq_ffs(q, mask);
    if (qfq_gt(grp.F, next.F))
    state |= EB;
    }
    return state;
    }
//
// In principle
// q->bitmaps[dst] |= q->bitmaps[src] & mask;
// q->bitmaps[src] &= ~mask;
// but we should make sure that src != dst
//
    static inline void qfq_move_groups(struct qfq_sched *q, unsigned long mask,
    int src, int dst)
    {
    q.bitmaps[dst] |= q.bitmaps[src] & mask;
    q.bitmaps[src] &= ~mask;
    }
#[no_mangle]
unsafe extern "C" fn qfq_unblock_groups(q: *mut qfq_sched, index: c_int, old_F: u64) {
    static void qfq_unblock_groups(struct qfq_sched *q, int index, u64 old_F)
    {
    let mut mask: c_ulong = mask_from(q.bitmaps[ER], index + 1);
    struct qfq_group *next;
    if (mask) {
    next = qfq_ffs(q, mask);
    if (!qfq_gt(next.F, old_F))
    return;
    }
    mask = (1UL << index) - 1;
    qfq_move_groups(q, mask, EB, ER);
    qfq_move_groups(q, mask, IB, IR);
    }
//
// perhaps
//
    old_V ^= q.V;
    old_V >>= q.min_slot_shift;
    if (old_V) {
    ...
    }
//
#[no_mangle]
unsafe extern "C" fn qfq_make_eligible(q: *mut qfq_sched) {
    static void qfq_make_eligible(struct qfq_sched *q)
    {
    let mut vslot: c_ulong = q.V >> q.min_slot_shift;
    let mut old_vslot: c_ulong = q.oldV >> q.min_slot_shift;
    if (vslot != old_vslot) {
    unsigned long mask;
    let mut last_flip_pos: c_int = fls(vslot ^ old_vslot);
    if (last_flip_pos > 31) /* higher than the number of groups */
    mask = ~0UL;    /* make all groups eligible */
    else
    mask = (1UL << last_flip_pos) - 1;
    qfq_move_groups(q, mask, IR, ER);
    qfq_move_groups(q, mask, IB, EB);
    }
    }
//
// The index of the slot in which the input aggregate agg is to be
// inserted must not be higher than QFQ_MAX_SLOTS-2. There is a '-2'
// and not a '-1' because the start time of the group may be moved
// backward by one slot after the aggregate has been inserted, and
// this would cause non-empty slots to be right-shifted by one
// position.
//
// QFQ+ fully satisfies this bound to the slot index if the parameters
// of the classes are not changed dynamically, and if QFQ+ never
// happens to postpone the service of agg unjustly, i.e., it never
// happens that the aggregate becomes backlogged and eligible, or just
// eligible, while an aggregate with a higher approximated finish time
// is being served. In particular, in this case QFQ+ guarantees that
// the timestamps of agg are low enough that the slot index is never
// higher than 2. Unfortunately, QFQ+ cannot provide the same
// guarantee if it happens to unjustly postpone the service of agg, or
// if the parameters of some class are changed.
//
// As for the first event, i.e., an out-of-order service, the
// upper bound to the slot index guaranteed by QFQ+ grows to
// 2 +
// QFQ_MAX_AGG_CLASSES * ((1<<QFQ_MTU_SHIFT)/QFQ_MIN_LMAX)
// (current_max_weight/current_wsum) <= 2 + 8 * 128 * 1.
//
// The following function deals with this problem by backward-shifting
// the timestamps of agg, if needed, so as to guarantee that the slot
// index is never higher than QFQ_MAX_SLOTS-2. This backward-shift may
// cause the service of other aggregates to be postponed, yet the
// worst-case guarantees of these aggregates are not violated.  In
// fact, in case of no out-of-order service, the timestamps of agg
// would have been even lower than they are after the backward shift,
// because QFQ+ would have guaranteed a maximum value equal to 2 for
// the slot index, and 2 < QFQ_MAX_SLOTS-2. Hence the aggregates whose
// service is postponed because of the backward-shift would have
// however waited for the service of agg before being served.
//
// The other event that may cause the slot index to be higher than 2
// for agg is a recent change of the parameters of some class. If the
// weight of a class is increased or the lmax (max_pkt_size) of the
// class is decreased, then a new aggregate with smaller slot size
// than the original parent aggregate of the class may happen to be
// activated. The activation of this aggregate should be properly
// delayed to when the service of the class has finished in the ideal
// system tracked by QFQ+. If the activation of the aggregate is not
// delayed to this reference time instant, then this aggregate may be
// unjustly served before other aggregates waiting for service. This
// may cause the above bound to the slot index to be violated for some
// of these unlucky aggregates.
//
// Instead of delaying the activation of the new aggregate, which is
// quite complex, the above-discussed capping of the slot index is
// used to handle also the consequences of a change of the parameters
// of a class.
//
    static void qfq_slot_insert(struct qfq_group *grp, struct qfq_aggregate *agg,
    u64 roundedS)
    {
    let mut slot: u64 = (roundedS - grp.S) >> grp.slot_shift;
    unsigned int i; /* slot index in the bucket list */
    if (unlikely(slot > QFQ_MAX_SLOTS - 2)) {
    u64 deltaS = roundedS - grp.S -
    ((u64)(QFQ_MAX_SLOTS - 2)<<grp.slot_shift);
    agg.S -= deltaS;
    agg.F -= deltaS;
    slot = QFQ_MAX_SLOTS - 2;
    }
    i = (grp.front + slot) % QFQ_MAX_SLOTS;
    hlist_add_head(&agg.next, &grp.slots[i]);
    __set_bit(slot, &grp.full_slots);
    }
// Maybe introduce hlist_first_entry??
    static struct qfq_aggregate *qfq_slot_head(struct qfq_group *grp)
    {
    return hlist_entry(grp.slots[grp.front].first,
    struct qfq_aggregate, next);
    }
//
// remove the entry from the slot
//
#[no_mangle]
unsafe extern "C" fn qfq_front_slot_remove(grp: *mut qfq_group) {
    static void qfq_front_slot_remove(struct qfq_group *grp)
    {
    struct qfq_aggregate *agg = qfq_slot_head(grp);
    BUG_ON(!agg);
    hlist_del(&agg.next);
    if (hlist_empty(&grp.slots[grp.front]))
    __clear_bit(0, &grp.full_slots);
    }
//
// Returns the first aggregate in the first non-empty bucket of the
// group. As a side effect, adjusts the bucket list so the first
// non-empty bucket is at position 0 in full_slots.
//
    static struct qfq_aggregate *qfq_slot_scan(struct qfq_group *grp)
    {
    unsigned int i;
    pr_debug("qfq slot_scan: grp %u full %#lx\n",
    grp.index, grp.full_slots);
    if (grp.full_slots == 0)
    return core::ptr::null_mut();
    i = __ffs(grp.full_slots);  /* zero based */
    if (i > 0) {
    grp.front = (grp.front + i) % QFQ_MAX_SLOTS;
    grp.full_slots >>= i;
    }
    return qfq_slot_head(grp);
    }
//
// adjust the bucket list. When the start time of a group decreases,
// we move the index down (modulo QFQ_MAX_SLOTS) so we don't need to
// move the objects. The mask of occupied slots must be shifted
// because we use ffs() to find the first non-empty slot.
// This covers decreases in the group's start time, but what about
// increases of the start time ?
// Here too we should make sure that i is less than 32
//
#[no_mangle]
unsafe extern "C" fn qfq_slot_rotate(grp: *mut qfq_group, roundedS: u64) {
    static void qfq_slot_rotate(struct qfq_group *grp, u64 roundedS)
    {
    let mut i: c_uint = (grp.S - roundedS) >> grp.slot_shift;
    grp.full_slots <<= i;
    grp.front = (grp.front - i) % QFQ_MAX_SLOTS;
    }
#[no_mangle]
unsafe extern "C" fn qfq_update_eligible(q: *mut qfq_sched) {
    static void qfq_update_eligible(struct qfq_sched *q)
    {
    struct qfq_group *grp;
    unsigned long ineligible;
    ineligible = q.bitmaps[IR] | q.bitmaps[IB];
    if (ineligible) {
    if (!q.bitmaps[ER]) {
    grp = qfq_ffs(q, ineligible);
    if (qfq_gt(grp.S, q.V))
    q.V = grp.S;
    }
    qfq_make_eligible(q);
    }
    }
// Dequeue head packet of the head class in the DRR queue of the aggregate.
    static struct sk_buff *agg_dequeue(struct qfq_aggregate *agg,
    struct qfq_class *cl, unsigned int len)
    {
    struct sk_buff *skb = qdisc_dequeue_peeked(cl.qdisc);
    if (!skb)
    return core::ptr::null_mut();
    cl.deficit -= (int) len;
    if (cl.qdisc.q.qlen == 0) /* no more packets, remove from list */
    list_del_init(&cl.alist);
#[no_mangle]
pub unsafe extern "C" fn if(qdisc_peek_len(cl->qdisc): cl->deficit <) -> else {
    cl.deficit += agg.lmax;
    list_move_tail(&cl.alist, &agg.active);
    }
    return skb;
    }
    static inline struct sk_buff *qfq_peek_skb(struct qfq_aggregate *agg,
    struct qfq_class **cl,
    unsigned int *len)
    {
    struct sk_buff *skb;
// cl = list_first_entry(&agg->active, struct qfq_class, alist);
    skb = (*cl).qdisc.ops.peek((*cl).qdisc);
    if (skb == core::ptr::null_mut())
    qdisc_warn_nonwc("qfq_dequeue", (*cl).qdisc);
    else
// len = qdisc_pkt_len(skb);
    return skb;
    }
// Update F according to the actual service received by the aggregate.
#[no_mangle]
pub unsafe extern "C" fn charge_actual_service(agg: *mut qfq_aggregate) {
    static inline void charge_actual_service(struct qfq_aggregate *agg)
    {
// Compute the service received by the aggregate, taking into
// account that, after decreasing the number of classes in
// agg, it may happen that
// agg->initial_budget - agg->budget > agg->bugdetmax
//
    u32 service_received = min(agg.budgetmax,
    agg.initial_budget - agg.budget);
    agg.F = agg.S + (u64)service_received * agg.inv_w;
    }
// Assign a reasonable start time for a new aggregate in group i.
// Admissible values for \hat(F) are multiples of \sigma_i
// no greater than V+\sigma_i . Larger values mean that
// we had a wraparound so we consider the timestamp to be stale.
//
// If F is not stale and F >= V then we set S = F.
// Otherwise we should assign S = V, but this may violate
// the ordering in EB (see [2]). So, if we have groups in ER,
// set S to the F_j of the first group j which would be blocking us.
// We are guaranteed not to move S backward because
// otherwise our group i would still be blocked.
//
#[no_mangle]
unsafe extern "C" fn qfq_update_start(q: *mut qfq_sched, agg: *mut qfq_aggregate) {
    static void qfq_update_start(struct qfq_sched *q, struct qfq_aggregate *agg)
    {
    unsigned long mask;
    u64 limit, roundedF;
    let mut slot_shift: c_int = agg.grp.slot_shift;
    roundedF = qfq_round_down(agg.F, slot_shift);
    limit = qfq_round_down(q.V, slot_shift) + (1ULL << slot_shift);
    if (!qfq_gt(agg.F, q.V) || qfq_gt(roundedF, limit)) {
// timestamp was stale
    mask = mask_from(q.bitmaps[ER], agg.grp.index);
    if (mask) {
    struct qfq_group *next = qfq_ffs(q, mask);
    if (qfq_gt(roundedF, next.F)) {
    if (qfq_gt(limit, next.F))
    agg.S = next.F;
    else /* preserve timestamp correctness */
    agg.S = limit;
    return;
    }
    }
    agg.S = q.V;
    } else  /* timestamp is not stale */
    agg.S = agg.F;
    }
// Update the timestamps of agg before scheduling/rescheduling it for
// service.  In particular, assign to agg->F its maximum possible
// value, i.e., the virtual finish time with which the aggregate
// should be labeled if it used all its budget once in service.
//
    static inline void
    qfq_update_agg_ts(struct qfq_sched *q,
    struct qfq_aggregate *agg, enum update_reason reason)
    {
    if (reason != requeue)
    qfq_update_start(q, agg);
    else /* just charge agg for the service received */
    agg.S = agg.F;
    agg.F = agg.S + (u64)agg.budgetmax * agg.inv_w;
    }
    static void qfq_schedule_agg(struct qfq_sched *q, struct qfq_aggregate *agg);
    static struct sk_buff *qfq_dequeue(struct Qdisc *sch)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_aggregate *in_serv_agg = q.in_serv_agg;
    struct qfq_class *cl;
    struct sk_buff *skb = core::ptr::null_mut();
// next-packet len, 0 means no more active classes in in-service agg
    let mut len: c_uint = 0;
    if (in_serv_agg == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (!list_empty(&in_serv_agg.active))
    skb = qfq_peek_skb(in_serv_agg, &cl, &len);
//
// If there are no active classes in the in-service aggregate,
// or if the aggregate has not enough budget to serve its next
// class, then choose the next aggregate to serve.
//
    if (len == 0 || in_serv_agg.budget < len) {
    charge_actual_service(in_serv_agg);
// recharge the budget of the aggregate
    in_serv_agg.initial_budget = in_serv_agg.budget =
    in_serv_agg.budgetmax;
    if (!list_empty(&in_serv_agg.active)) {
//
// Still active: reschedule for
// service. Possible optimization: if no other
// aggregate is active, then there is no point
// in rescheduling this aggregate, and we can
// just keep it as the in-service one. This
// should be however a corner case, and to
// handle it, we would need to maintain an
// extra num_active_aggs field.
//
    qfq_update_agg_ts(q, in_serv_agg, requeue);
    qfq_schedule_agg(q, in_serv_agg);
    } else if (sch.q.qlen == 0) { /* no aggregate to serve */
    q.in_serv_agg = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
//
// If we get here, there are other aggregates queued:
// choose the new aggregate to serve.
//
    in_serv_agg = q.in_serv_agg = qfq_choose_next_agg(q);
    skb = qfq_peek_skb(in_serv_agg, &cl, &len);
    }
    if (!skb)
    return core::ptr::null_mut();
    qdisc_qlen_dec(sch);
    skb = agg_dequeue(in_serv_agg, cl, len);
    if (!skb) {
    qdisc_qlen_inc(sch);
    return core::ptr::null_mut();
    }
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
// If lmax is lowered, through qfq_change_class, for a class
// owning pending packets with larger size than the new value
// of lmax, then the following condition may hold.
//
    if (unlikely(in_serv_agg.budget < len))
    in_serv_agg.budget = 0;
    else
    in_serv_agg.budget -= len;
    q.V += (u64)len * q.iwsum;
    pr_debug("qfq dequeue: len %u F %lld now %lld\n",
    len, (unsigned long long) in_serv_agg.F,
    (unsigned long long) q.V);
    return skb;
    }
    static struct qfq_aggregate *qfq_choose_next_agg(struct qfq_sched *q)
    {
    struct qfq_group *grp;
    struct qfq_aggregate *agg, *new_front_agg;
    u64 old_F;
    qfq_update_eligible(q);
    q.oldV = q.V;
    if (!q.bitmaps[ER])
    return core::ptr::null_mut();
    grp = qfq_ffs(q, q.bitmaps[ER]);
    old_F = grp.F;
    agg = qfq_slot_head(grp);
// agg starts to be served, remove it from schedule
    qfq_front_slot_remove(grp);
    new_front_agg = qfq_slot_scan(grp);
    if (new_front_agg == core::ptr::null_mut()) /* group is now inactive, remove from ER */
    __clear_bit(grp.index, &q.bitmaps[ER]);
    else {
    u64 roundedS = qfq_round_down(new_front_agg.S,
    grp.slot_shift);
    unsigned int s;
    if (grp.S == roundedS)
    return agg;
    grp.S = roundedS;
    grp.F = roundedS + (2ULL << grp.slot_shift);
    __clear_bit(grp.index, &q.bitmaps[ER]);
    s = qfq_calc_state(q, grp);
    __set_bit(grp.index, &q.bitmaps[s]);
    }
    qfq_unblock_groups(q, grp.index, old_F);
    return agg;
    }
    static int qfq_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    let mut len: c_uint = qdisc_pkt_len(skb), gso_segs;
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl;
    struct qfq_aggregate *agg;
    let mut err: c_int = 0;
    cl = qfq_classify(skb, sch, &err);
    if (cl == core::ptr::null_mut()) {
    if (err & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return err;
    }
    pr_debug("qfq_enqueue: cl = %x\n", cl.common.classid);
    if (unlikely(cl.agg.lmax < len)) {
    pr_debug("qfq: increasing maxpkt from %u to %u for class %u",
    cl.agg.lmax, len, cl.common.classid);
    err = qfq_change_agg(sch, cl, cl.agg.class_weight, len);
    if (err) {
    cl.qstats.drops++;
    return qdisc_drop(skb, sch, to_free);
    }
    }
    gso_segs = qdisc_pkt_segs(skb);
    err = qdisc_enqueue(skb, cl.qdisc, to_free);
    if (unlikely(err != NET_XMIT_SUCCESS)) {
    pr_debug("qfq_enqueue: enqueue failed %d\n", err);
    if (net_xmit_drop_count(err)) {
    cl.qstats.drops++;
    qdisc_qstats_drop(sch);
    }
    return err;
    }
    _bstats_update(&cl.bstats, len, gso_segs);
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    agg = cl.agg;
// if the class is active, then done here
    if (cl_is_active(cl)) {
    if (unlikely(skb == cl.qdisc.ops.peek(cl.qdisc)) &&
    list_first_entry(&agg.active, struct qfq_class, alist)
    == cl && cl.deficit < len)
    list_move_tail(&cl.alist, &agg.active);
    return err;
    }
// schedule class for service within the aggregate
    cl.deficit = agg.lmax;
    list_add_tail(&cl.alist, &agg.active);
    if (list_first_entry(&agg.active, struct qfq_class, alist) != cl ||
    q.in_serv_agg == agg)
    return err; /* non-empty or in service, nothing else to do */
    qfq_activate_agg(q, agg, enqueue);
    return err;
    }
//
// Schedule aggregate according to its timestamps.
//
#[no_mangle]
unsafe extern "C" fn qfq_schedule_agg(q: *mut qfq_sched, agg: *mut qfq_aggregate) {
    static void qfq_schedule_agg(struct qfq_sched *q, struct qfq_aggregate *agg)
    {
    struct qfq_group *grp = agg.grp;
    u64 roundedS;
    int s;
    roundedS = qfq_round_down(agg.S, grp.slot_shift);
//
// Insert agg in the correct bucket.
// If agg->S >= grp->S we don't need to adjust the
// bucket list and simply go to the insertion phase.
// Otherwise grp->S is decreasing, we must make room
// in the bucket list, and also recompute the group state.
// Finally, if there were no flows in this group and nobody
// was in ER make sure to adjust V.
//
    if (grp.full_slots) {
    if (!qfq_gt(grp.S, agg.S))
    goto skip_update;
// create a slot for this agg->S
    qfq_slot_rotate(grp, roundedS);
// group was surely ineligible, remove
    __clear_bit(grp.index, &q.bitmaps[IR]);
    __clear_bit(grp.index, &q.bitmaps[IB]);
    } else if (!q.bitmaps[ER] && qfq_gt(roundedS, q.V) &&
    q.in_serv_agg == core::ptr::null_mut())
    q.V = roundedS;
    grp.S = roundedS;
    grp.F = roundedS + (2ULL << grp.slot_shift);
    s = qfq_calc_state(q, grp);
    __set_bit(grp.index, &q.bitmaps[s]);
    pr_debug("qfq enqueue: new state %d %#lx S %lld F %lld V %lld\n",
    s, q.bitmaps[s],
    (unsigned long long) agg.S,
    (unsigned long long) agg.F,
    (unsigned long long) q.V);
    skip_update:
    qfq_slot_insert(grp, agg, roundedS);
    }
// Update agg ts and schedule agg for service
    static void qfq_activate_agg(struct qfq_sched *q, struct qfq_aggregate *agg,
    enum update_reason reason)
    {
    agg.initial_budget = agg.budget = agg.budgetmax; /* recharge budg. */
    qfq_update_agg_ts(q, agg, reason);
    if (q.in_serv_agg == core::ptr::null_mut()) { /* no aggr. in service or scheduled */
    q.in_serv_agg = agg; /* start serving this aggregate */
// update V: to be in service, agg must be eligible
    q.oldV = q.V = agg.S;
    } else if (agg != q.in_serv_agg)
    qfq_schedule_agg(q, agg);
    }
    static void qfq_slot_remove(struct qfq_sched *q, struct qfq_group *grp,
    struct qfq_aggregate *agg)
    {
    unsigned int i, offset;
    u64 roundedS;
    roundedS = qfq_round_down(agg.S, grp.slot_shift);
    offset = (roundedS - grp.S) >> grp.slot_shift;
    i = (grp.front + offset) % QFQ_MAX_SLOTS;
    hlist_del(&agg.next);
    if (hlist_empty(&grp.slots[i]))
    __clear_bit(offset, &grp.full_slots);
    }
//
// Called to forcibly deschedule an aggregate.  If the aggregate is
// not in the front bucket, or if the latter has other aggregates in
// the front bucket, we can simply remove the aggregate with no other
// side effects.
// Otherwise we must propagate the event up.
//
#[no_mangle]
unsafe extern "C" fn qfq_deactivate_agg(q: *mut qfq_sched, agg: *mut qfq_aggregate) {
    static void qfq_deactivate_agg(struct qfq_sched *q, struct qfq_aggregate *agg)
    {
    struct qfq_group *grp = agg.grp;
    unsigned long mask;
    u64 roundedS;
    int s;
    if (agg == q.in_serv_agg) {
    charge_actual_service(agg);
    q.in_serv_agg = qfq_choose_next_agg(q);
    return;
    }
    agg.F = agg.S;
    qfq_slot_remove(q, grp, agg);
    if (!grp.full_slots) {
    __clear_bit(grp.index, &q.bitmaps[IR]);
    __clear_bit(grp.index, &q.bitmaps[EB]);
    __clear_bit(grp.index, &q.bitmaps[IB]);
    if (test_bit(grp.index, &q.bitmaps[ER]) &&
    !(q.bitmaps[ER] & ~((1UL << grp.index) - 1))) {
    mask = q.bitmaps[ER] & ((1UL << grp.index) - 1);
    if (mask)
    mask = ~((1UL << __fls(mask)) - 1);
    else
    mask = ~0UL;
    qfq_move_groups(q, mask, EB, ER);
    qfq_move_groups(q, mask, IB, IR);
    }
    __clear_bit(grp.index, &q.bitmaps[ER]);
    } else if (hlist_empty(&grp.slots[grp.front])) {
    agg = qfq_slot_scan(grp);
    roundedS = qfq_round_down(agg.S, grp.slot_shift);
    if (grp.S != roundedS) {
    __clear_bit(grp.index, &q.bitmaps[ER]);
    __clear_bit(grp.index, &q.bitmaps[IR]);
    __clear_bit(grp.index, &q.bitmaps[EB]);
    __clear_bit(grp.index, &q.bitmaps[IB]);
    grp.S = roundedS;
    grp.F = roundedS + (2ULL << grp.slot_shift);
    s = qfq_calc_state(q, grp);
    __set_bit(grp.index, &q.bitmaps[s]);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn qfq_qlen_notify(sch: *mut Qdisc, arg: c_ulong) {
    static void qfq_qlen_notify(struct Qdisc *sch, unsigned long arg)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl = (struct qfq_class *)arg;
    if (list_empty(&cl.alist))
    return;
    qfq_deactivate_class(q, cl);
    }
    static int qfq_init_qdisc(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_group *grp;
    int i, j, err;
    u32 max_cl_shift, maxbudg_shift, max_classes;
    err = tcf_block_get(&q.block, &q.filter_list, sch, extack);
    if (err)
    return err;
    err = qdisc_class_hash_init(&q.clhash);
    if (err < 0)
    return err;
    max_classes = min_t(u64, (u64)qdisc_dev(sch).tx_queue_len + 1,
    QFQ_MAX_AGG_CLASSES);
// max_cl_shift = floor(log_2(max_classes))
    max_cl_shift = __fls(max_classes);
    q.max_agg_classes = 1<<max_cl_shift;
// maxbudg_shift = log2(max_len * max_classes_per_agg)
    maxbudg_shift = QFQ_MTU_SHIFT + max_cl_shift;
    q.min_slot_shift = FRAC_BITS + maxbudg_shift - QFQ_MAX_INDEX;
    for (i = 0; i <= QFQ_MAX_INDEX; i++) {
    grp = &q.groups[i];
    grp.index = i;
    grp.slot_shift = q.min_slot_shift + i;
    for (j = 0; j < QFQ_MAX_SLOTS; j++)
    INIT_HLIST_HEAD(&grp.slots[j]);
    }
    INIT_HLIST_HEAD(&q.nonfull_aggs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qfq_reset_qdisc(sch: *mut Qdisc) {
    static void qfq_reset_qdisc(struct Qdisc *sch)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl;
    unsigned int i;
    for (i = 0; i < q.clhash.hashsize; i++) {
    hlist_for_each_entry(cl, &q.clhash.hash[i], common.hnode) {
    if (cl_is_active(cl))
    qfq_deactivate_class(q, cl);
    qdisc_reset(cl.qdisc);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn qfq_destroy_qdisc(sch: *mut Qdisc) {
    static void qfq_destroy_qdisc(struct Qdisc *sch)
    {
    struct qfq_sched *q = qdisc_priv(sch);
    struct qfq_class *cl;
    struct hlist_node *next;
    unsigned int i;
    tcf_block_put(q.block);
    for (i = 0; i < q.clhash.hashsize; i++) {
    hlist_for_each_entry_safe(cl, next, &q.clhash.hash[i],
    common.hnode) {
    qfq_rm_from_agg(q, cl);
    qfq_destroy_class(sch, cl);
    }
    }
    qdisc_class_hash_destroy(&q.clhash);
    }
    static const struct Qdisc_class_ops qfq_class_ops = {
    .change		= qfq_change_class,
    .delete		= qfq_delete_class,
    .find		= qfq_search_class,
    .tcf_block	= qfq_tcf_block,
    .bind_tcf	= qfq_bind_tcf,
    .unbind_tcf	= qfq_unbind_tcf,
    .graft		= qfq_graft_class,
    .leaf		= qfq_class_leaf,
    .qlen_notify	= qfq_qlen_notify,
    .dump		= qfq_dump_class,
    .dump_stats	= qfq_dump_class_stats,
    .walk		= qfq_walk,
    };
    static struct Qdisc_ops qfq_qdisc_ops __read_mostly = {
    .cl_ops		= &qfq_class_ops,
    .id		= "qfq",
    .priv_size	= sizeof(struct qfq_sched),
    .enqueue	= qfq_enqueue,
    .dequeue	= qfq_dequeue,
    .peek		= qdisc_peek_dequeued,
    .init		= qfq_init_qdisc,
    .reset		= qfq_reset_qdisc,
    .destroy	= qfq_destroy_qdisc,
    .owner		= THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("qfq");
#[no_mangle]
unsafe extern "C" fn qfq_init() -> int __init {
    static int __init qfq_init(void)
    {
    return register_qdisc(&qfq_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn qfq_exit() -> void __exit {
    static void __exit qfq_exit(void)
    {
    unregister_qdisc(&qfq_qdisc_ops);
    }
    module_init(qfq_init);
    module_exit(qfq_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Quick Fair Queueing Plus qdisc");
