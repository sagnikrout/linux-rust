//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_ets.c
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
// net/sched/sch_ets.c         Enhanced Transmission Selection scheduler
//
// Description
// -----------
//
// The Enhanced Transmission Selection scheduler is a classful queuing
// discipline that merges functionality of PRIO and DRR qdiscs in one scheduler.
// ETS makes it easy to configure a set of strict and bandwidth-sharing bands to
// implement the transmission selection described in 802.1Qaz.
//
// Although ETS is technically classful, it's not possible to add and remove
// classes at will. Instead one specifies number of classes, how many are
// PRIO-like and how many DRR-like, and quanta for the latter.
//
// Algorithm
// ---------
//
// The strict classes, if any, are tried for traffic first: first band 0, if it
// has no traffic then band 1, etc.
//
// When there is no traffic in any of the strict queues, the bandwidth-sharing
// ones are tried next. Each band is assigned a deficit counter, initialized to
// "quantum" of that band. ETS maintains a list of active bandwidth-sharing
// bands whose qdiscs are non-empty. A packet is dequeued from the band at the
// head of the list if the packet size is smaller or equal to the deficit
// counter. If the counter is too small, it is increased by "quantum" and the
// scheduler moves on to the next band in the active list.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ets_class {
    pub /: *mut *mut list_head alist; / In ets_sched.active.,
    pub qdisc: *mut Qdisc,
    pub quantum: u32,
    pub deficit: u32,
    pub bstats: gnet_stats_basic_sync,
    pub qstats: gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ets_sched {
    pub active: list_head,
    pub filter_list: *mut tcf_proto __rcu,
    pub block: *mut tcf_block,
    pub nbands: c_uint,
    pub nstrict: c_uint,
    pub 1]: u8 prio2band[TC_PRIO_MAX +,
    pub classes: [ets_class; TCQ_ETS_MAX_BANDS],
}

    static const struct nla_policy ets_policy[TCA_ETS_MAX + 1] = {
    [TCA_ETS_NBANDS] = { .type = NLA_U8 },
    [TCA_ETS_NSTRICT] = { .type = NLA_U8 },
    [TCA_ETS_QUANTA] = { .type = NLA_NESTED },
    [TCA_ETS_PRIOMAP] = { .type = NLA_NESTED },
    };
    static const struct nla_policy ets_priomap_policy[TCA_ETS_MAX + 1] = {
    [TCA_ETS_PRIOMAP_BAND] = { .type = NLA_U8 },
    };
    static const struct nla_policy ets_quanta_policy[TCA_ETS_MAX + 1] = {
    [TCA_ETS_QUANTA_BAND] = { .type = NLA_U32 },
    };
    static const struct nla_policy ets_class_policy[TCA_ETS_MAX + 1] = {
    [TCA_ETS_QUANTA_BAND] = { .type = NLA_U32 },
    };
#[no_mangle]
unsafe extern "C" fn cl_is_active(cl: *mut ets_class) -> bool {
    static bool cl_is_active(struct ets_class *cl)
    {
    return !list_empty(&cl.alist);
    }
    static int ets_quantum_parse(struct Qdisc *sch, const struct nlattr *attr,
    unsigned int *quantum,
    struct netlink_ext_ack *extack)
    {
// quantum = nla_get_u32(attr);
    if (!*quantum) {
    NL_SET_ERR_MSG(extack, "ETS quantum cannot be zero");
    return -EINVAL;
    }
    return 0;
    }
    static struct ets_class *
    ets_class_from_arg(struct Qdisc *sch, unsigned long arg)
    {
    struct ets_sched *q = qdisc_priv(sch);
    if (arg == 0 || arg > q.nbands)
    return core::ptr::null_mut();
    return &q.classes[arg - 1];
    }
#[no_mangle]
unsafe extern "C" fn ets_class_id(sch: *mut Qdisc, cl: *const ets_class) -> u32 {
    static u32 ets_class_id(struct Qdisc *sch, const struct ets_class *cl)
    {
    struct ets_sched *q = qdisc_priv(sch);
    let mut band: c_int = cl - q.classes;
    return TC_H_MAKE(sch.handle, band + 1);
    }
#[no_mangle]
unsafe extern "C" fn ets_offload_change(sch: *mut Qdisc) {
    static void ets_offload_change(struct Qdisc *sch)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct ets_sched *q = qdisc_priv(sch);
    struct tc_ets_qopt_offload qopt;
    let mut w_psum_prev: c_uint = 0;
    unsigned int quantum;
    unsigned int w_psum;
    unsigned int weight;
    unsigned int i;
    let mut q_psum: u64 = 0;
    let mut q_sum: u64 = 0;
    if (!tc_can_offload(dev) || !dev.netdev_ops.ndo_setup_tc)
    return;
    qopt.command = TC_ETS_REPLACE;
    qopt.handle = sch.handle;
    qopt.parent = sch.parent;
    qopt.replace_params.bands = q.nbands;
    qopt.replace_params.qstats = &sch.qstats;
    memcpy(&qopt.replace_params.priomap,
    q.prio2band, sizeof(q.prio2band));
    for (i = 0; i < q.nbands; i++)
    q_sum += q.classes[i].quantum;
    for (i = 0; i < q.nbands; i++) {
    quantum = q.classes[i].quantum;
    if (quantum) {
    q_psum += quantum;
    w_psum = div64_u64(q_psum * 100, q_sum);
    } else {
    w_psum = 0;
    }
    weight = w_psum - w_psum_prev;
    w_psum_prev = w_psum;
    qopt.replace_params.quanta[i] = quantum;
    qopt.replace_params.weights[i] = weight;
    }
    dev.netdev_ops.ndo_setup_tc(dev, TC_SETUP_QDISC_ETS, &qopt);
    }
#[no_mangle]
unsafe extern "C" fn ets_offload_destroy(sch: *mut Qdisc) {
    static void ets_offload_destroy(struct Qdisc *sch)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct tc_ets_qopt_offload qopt;
    if (!tc_can_offload(dev) || !dev.netdev_ops.ndo_setup_tc)
    return;
    qopt.command = TC_ETS_DESTROY;
    qopt.handle = sch.handle;
    qopt.parent = sch.parent;
    dev.netdev_ops.ndo_setup_tc(dev, TC_SETUP_QDISC_ETS, &qopt);
    }
    static void ets_offload_graft(struct Qdisc *sch, struct Qdisc *new,
    struct Qdisc *old, unsigned long arg,
    struct netlink_ext_ack *extack)
    {
    struct net_device *dev = qdisc_dev(sch);
    struct tc_ets_qopt_offload qopt;
    qopt.command = TC_ETS_GRAFT;
    qopt.handle = sch.handle;
    qopt.parent = sch.parent;
    qopt.graft_params.band = arg - 1;
    qopt.graft_params.child_handle = new.handle;
    qdisc_offload_graft_helper(dev, sch, new, old, TC_SETUP_QDISC_ETS,
    &qopt, extack);
    }
#[no_mangle]
unsafe extern "C" fn ets_offload_dump(sch: *mut Qdisc) -> c_int {
    static int ets_offload_dump(struct Qdisc *sch)
    {
    struct tc_ets_qopt_offload qopt;
    qopt.command = TC_ETS_STATS;
    qopt.handle = sch.handle;
    qopt.parent = sch.parent;
    qopt.stats.bstats = &sch.bstats;
    qopt.stats.qstats = &sch.qstats;
    return qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_ETS, &qopt);
    }
#[no_mangle]
unsafe extern "C" fn ets_class_is_strict(q: *mut ets_sched, cl: *const ets_class) -> bool {
    static bool ets_class_is_strict(struct ets_sched *q, const struct ets_class *cl)
    {
    let mut band: c_uint = cl - q.classes;
    return band < q.nstrict;
    }
    static int ets_class_change(struct Qdisc *sch, u32 classid, u32 parentid,
    struct nlattr **tca, unsigned long *arg,
    struct netlink_ext_ack *extack)
    {
    struct ets_class *cl = ets_class_from_arg(sch, *arg);
    struct ets_sched *q = qdisc_priv(sch);
    struct nlattr *opt = tca[TCA_OPTIONS];
    struct nlattr *tb[TCA_ETS_MAX + 1];
    unsigned int quantum;
    int err;
// Classes can be added and removed only through Qdisc_ops.change
// interface.
//
    if (!cl) {
    NL_SET_ERR_MSG(extack, "Fine-grained class addition and removal is not supported");
    return -EOPNOTSUPP;
    }
    if (!opt) {
    NL_SET_ERR_MSG(extack, "ETS options are required for this operation");
    return -EINVAL;
    }
    err = nla_parse_nested(tb, TCA_ETS_MAX, opt, ets_class_policy, extack);
    if (err < 0)
    return err;
    if (!tb[TCA_ETS_QUANTA_BAND])
// Nothing to configure.
    return 0;
    if (ets_class_is_strict(q, cl)) {
    NL_SET_ERR_MSG(extack, "Strict bands do not have a configurable quantum");
    return -EINVAL;
    }
    err = ets_quantum_parse(sch, tb[TCA_ETS_QUANTA_BAND], &quantum,
    extack);
    if (err)
    return err;
    WRITE_ONCE(cl.quantum, quantum);
    ets_offload_change(sch);
    return 0;
    }
    static int ets_class_graft(struct Qdisc *sch, unsigned long arg,
    struct Qdisc *new, struct Qdisc **old,
    struct netlink_ext_ack *extack)
    {
    struct ets_class *cl = ets_class_from_arg(sch, arg);
    if (!new) {
    new = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    ets_class_id(sch, cl), core::ptr::null_mut());
    if (!new)
    new = &noop_qdisc;
    else
    qdisc_hash_add(new, true);
    }
// old = qdisc_replace(sch, new, &cl->qdisc);
    ets_offload_graft(sch, new, *old, arg, extack);
    return 0;
    }
    static struct Qdisc *ets_class_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct ets_class *cl = ets_class_from_arg(sch, arg);
    return cl.qdisc;
    }
#[no_mangle]
unsafe extern "C" fn ets_class_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long ets_class_find(struct Qdisc *sch, u32 classid)
    {
    let mut band: c_ulong = TC_H_MIN(classid);
    struct ets_sched *q = qdisc_priv(sch);
    if (band - 1 >= q.nbands)
    return 0;
    return band;
    }
#[no_mangle]
unsafe extern "C" fn ets_class_qlen_notify(sch: *mut Qdisc, arg: c_ulong) {
    static void ets_class_qlen_notify(struct Qdisc *sch, unsigned long arg)
    {
    struct ets_class *cl = ets_class_from_arg(sch, arg);
    struct ets_sched *q = qdisc_priv(sch);
// We get notified about zero-length child Qdiscs as well if they are
// offloaded. Those aren't on the active list though, so don't attempt
// to remove them.
//
    if (!ets_class_is_strict(q, cl) && sch.q.qlen)
    list_del_init(&cl.alist);
    }
    static int ets_class_dump(struct Qdisc *sch, unsigned long arg,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    struct ets_class *cl = ets_class_from_arg(sch, arg);
    struct ets_sched *q = qdisc_priv(sch);
    struct nlattr *nest;
    tcm.tcm_parent = TC_H_ROOT;
    tcm.tcm_handle = ets_class_id(sch, cl);
    tcm.tcm_info = cl.qdisc.handle;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (!nest)
    goto nla_put_failure;
    if (!ets_class_is_strict(q, cl)) {
    if (nla_put_u32(skb, TCA_ETS_QUANTA_BAND, READ_ONCE(cl.quantum)))
    goto nla_put_failure;
    }
    return nla_nest_end(skb, nest);
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -EMSGSIZE;
    }
    static int ets_class_dump_stats(struct Qdisc *sch, unsigned long arg,
    struct gnet_dump *d)
    {
    struct ets_class *cl = ets_class_from_arg(sch, arg);
    struct Qdisc *cl_q = cl.qdisc;
    if (gnet_stats_copy_basic(d, core::ptr::null_mut(), &cl_q.bstats, true) < 0 ||
    qdisc_qstats_copy(d, cl_q) < 0)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ets_qdisc_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void ets_qdisc_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    struct ets_sched *q = qdisc_priv(sch);
    int i;
    if (arg.stop)
    return;
    for (i = 0; i < q.nbands; i++) {
    if (!tc_qdisc_stats_dump(sch, i + 1, arg))
    break;
    }
    }
    static struct tcf_block *
    ets_qdisc_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct ets_sched *q = qdisc_priv(sch);
    if (cl) {
    NL_SET_ERR_MSG(extack, "ETS classid must be zero");
    return core::ptr::null_mut();
    }
    return q.block;
    }
    static unsigned long ets_qdisc_bind_tcf(struct Qdisc *sch, unsigned long parent,
    u32 classid)
    {
    return ets_class_find(sch, classid);
    }
#[no_mangle]
unsafe extern "C" fn ets_qdisc_unbind_tcf(sch: *mut Qdisc, arg: c_ulong) {
    static void ets_qdisc_unbind_tcf(struct Qdisc *sch, unsigned long arg)
    {
    }
    static struct ets_class *ets_classify(struct sk_buff *skb, struct Qdisc *sch,
    int *qerr)
    {
    struct ets_sched *q = qdisc_priv(sch);
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
    return &q.classes[q.prio2band[band & TC_PRIO_MAX]];
    }
    band = res.classid;
    }
    band = TC_H_MIN(band) - 1;
    if (band >= q.nbands)
    return &q.classes[q.prio2band[0]];
    return &q.classes[band];
    }
    static int ets_qdisc_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    let mut len: c_uint = qdisc_pkt_len(skb);
    struct ets_sched *q = qdisc_priv(sch);
    struct ets_class *cl;
    let mut err: c_int = 0;
    cl = ets_classify(skb, sch, &err);
    if (!cl) {
    if (err & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return err;
    }
    err = qdisc_enqueue(skb, cl.qdisc, to_free);
    if (unlikely(err != NET_XMIT_SUCCESS)) {
    if (net_xmit_drop_count(err)) {
    cl.qstats.drops++;
    qdisc_qstats_drop(sch);
    }
    return err;
    }
    if (!cl_is_active(cl) && !ets_class_is_strict(q, cl)) {
    list_add_tail(&cl.alist, &q.active);
    cl.deficit = READ_ONCE(cl.quantum);
    }
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    return err;
    }
    static struct sk_buff *
    ets_qdisc_dequeue_skb(struct Qdisc *sch, struct sk_buff *skb)
    {
    qdisc_bstats_update(sch, skb);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_qlen_dec(sch);
    return skb;
    }
    static struct sk_buff *ets_qdisc_dequeue(struct Qdisc *sch)
    {
    struct ets_sched *q = qdisc_priv(sch);
    struct ets_class *cl;
    struct sk_buff *skb;
    unsigned int band;
    unsigned int len;
    while (1) {
    for (band = 0; band < q.nstrict; band++) {
    cl = &q.classes[band];
    skb = qdisc_dequeue_peeked(cl.qdisc);
    if (skb)
    return ets_qdisc_dequeue_skb(sch, skb);
    }
    if (list_empty(&q.active))
    goto out;
    cl = list_first_entry(&q.active, struct ets_class, alist);
    skb = cl.qdisc.ops.peek(cl.qdisc);
    if (!skb) {
    qdisc_warn_nonwc(__func__, cl.qdisc);
    goto out;
    }
    len = qdisc_pkt_len(skb);
    if (len <= cl.deficit) {
    cl.deficit -= len;
    skb = qdisc_dequeue_peeked(cl.qdisc);
    if (unlikely(!skb))
    goto out;
    if (cl.qdisc.q.qlen == 0)
    list_del_init(&cl.alist);
    return ets_qdisc_dequeue_skb(sch, skb);
    }
    cl.deficit += READ_ONCE(cl.quantum);
    list_move_tail(&cl.alist, &q.active);
    }
    out:
    return core::ptr::null_mut();
    }
    static int ets_qdisc_priomap_parse(struct nlattr *priomap_attr,
    unsigned int nbands, u8 *priomap,
    struct netlink_ext_ack *extack)
    {
    const struct nlattr *attr;
    let mut prio: c_int = 0;
    u8 band;
    int rem;
    int err;
    err = __nla_validate_nested(priomap_attr, TCA_ETS_MAX,
    ets_priomap_policy, NL_VALIDATE_STRICT,
    extack);
    if (err)
    return err;
    nla_for_each_nested(attr, priomap_attr, rem) {
    switch (nla_type(attr)) {
    case TCA_ETS_PRIOMAP_BAND:
    if (prio > TC_PRIO_MAX) {
    NL_SET_ERR_MSG_MOD(extack, "Too many priorities in ETS priomap");
    return -EINVAL;
    }
    band = nla_get_u8(attr);
    if (band >= nbands) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid band number in ETS priomap");
    return -EINVAL;
    }
    priomap[prio++] = band;
    break;
    default:
    WARN_ON_ONCE(1); /* Validate should have caught this. */
    return -EINVAL;
    }
    }
    return 0;
    }
    static int ets_qdisc_quanta_parse(struct Qdisc *sch, struct nlattr *quanta_attr,
    unsigned int nbands, unsigned int nstrict,
    unsigned int *quanta,
    struct netlink_ext_ack *extack)
    {
    const struct nlattr *attr;
    let mut band: c_int = nstrict;
    int rem;
    int err;
    err = __nla_validate_nested(quanta_attr, TCA_ETS_MAX,
    ets_quanta_policy, NL_VALIDATE_STRICT,
    extack);
    if (err < 0)
    return err;
    nla_for_each_nested(attr, quanta_attr, rem) {
    switch (nla_type(attr)) {
    case TCA_ETS_QUANTA_BAND:
    if (band >= nbands) {
    NL_SET_ERR_MSG_MOD(extack, "ETS quanta has more values than bands");
    return -EINVAL;
    }
    err = ets_quantum_parse(sch, attr, &quanta[band++],
    extack);
    if (err)
    return err;
    break;
    default:
    WARN_ON_ONCE(1); /* Validate should have caught this. */
    return -EINVAL;
    }
    }
    return 0;
    }
    static int ets_qdisc_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    unsigned int quanta[TCQ_ETS_MAX_BANDS] = {0};
    struct Qdisc *queues[TCQ_ETS_MAX_BANDS];
    struct ets_sched *q = qdisc_priv(sch);
    struct nlattr *tb[TCA_ETS_MAX + 1];
    let mut oldbands: c_uint = q.nbands;
    u8 priomap[TC_PRIO_MAX + 1];
    let mut nstrict: c_uint = 0;
    unsigned int nbands;
    unsigned int i;
    int err;
    err = nla_parse_nested(tb, TCA_ETS_MAX, opt, ets_policy, extack);
    if (err < 0)
    return err;
    if (!tb[TCA_ETS_NBANDS]) {
    NL_SET_ERR_MSG_MOD(extack, "Number of bands is a required argument");
    return -EINVAL;
    }
    nbands = nla_get_u8(tb[TCA_ETS_NBANDS]);
    if (nbands < 1 || nbands > TCQ_ETS_MAX_BANDS) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid number of bands");
    return -EINVAL;
    }
// Unless overridden, traffic goes to the last band.
    memset(priomap, nbands - 1, sizeof(priomap));
    if (tb[TCA_ETS_NSTRICT]) {
    nstrict = nla_get_u8(tb[TCA_ETS_NSTRICT]);
    if (nstrict > nbands) {
    NL_SET_ERR_MSG_MOD(extack, "Invalid number of strict bands");
    return -EINVAL;
    }
    }
    if (tb[TCA_ETS_PRIOMAP]) {
    err = ets_qdisc_priomap_parse(tb[TCA_ETS_PRIOMAP],
    nbands, priomap, extack);
    if (err)
    return err;
    }
    if (tb[TCA_ETS_QUANTA]) {
    err = ets_qdisc_quanta_parse(sch, tb[TCA_ETS_QUANTA],
    nbands, nstrict, quanta, extack);
    if (err)
    return err;
    }
// If there are more bands than strict + quanta provided, the remaining
// ones are ETS with quantum of MTU. Initialize the missing values here.
//
    for (i = nstrict; i < nbands; i++) {
    if (!quanta[i])
    quanta[i] = psched_mtu(qdisc_dev(sch));
    }
// Before commit, make sure we can allocate all new qdiscs
    for (i = oldbands; i < nbands; i++) {
    queues[i] = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    ets_class_id(sch, &q.classes[i]),
    extack);
    if (!queues[i]) {
    while (i > oldbands)
    qdisc_put(queues[--i]);
    return -ENOMEM;
    }
    }
    sch_tree_lock(sch);
    for (i = nbands; i < oldbands; i++) {
    if (cl_is_active(&q.classes[i]))
    list_del_init(&q.classes[i].alist);
    qdisc_purge_queue(q.classes[i].qdisc);
    }
    WRITE_ONCE(q.nbands, nbands);
    for (i = nstrict; i < q.nstrict; i++) {
    if (q.classes[i].qdisc.q.qlen) {
    list_add_tail(&q.classes[i].alist, &q.active);
    q.classes[i].deficit = quanta[i];
    }
    }
    for (i = q.nstrict; i < nstrict; i++) {
    if (cl_is_active(&q.classes[i]))
    list_del_init(&q.classes[i].alist);
    }
    WRITE_ONCE(q.nstrict, nstrict);
    memcpy(q.prio2band, priomap, sizeof(priomap));
    for (i = 0; i < q.nbands; i++)
    WRITE_ONCE(q.classes[i].quantum, quanta[i]);
    for (i = oldbands; i < q.nbands; i++) {
    q.classes[i].qdisc = queues[i];
    if (q.classes[i].qdisc != &noop_qdisc)
    qdisc_hash_add(q.classes[i].qdisc, true);
    }
    sch_tree_unlock(sch);
    ets_offload_change(sch);
    for (i = q.nbands; i < oldbands; i++) {
    qdisc_put(q.classes[i].qdisc);
    q.classes[i].qdisc = core::ptr::null_mut();
    WRITE_ONCE(q.classes[i].quantum, 0);
    q.classes[i].deficit = 0;
    gnet_stats_basic_sync_init(&q.classes[i].bstats);
    memset(&q.classes[i].qstats, 0, sizeof(q.classes[i].qstats));
    }
    return 0;
    }
    static int ets_qdisc_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct ets_sched *q = qdisc_priv(sch);
    int err, i;
    if (!opt)
    return -EINVAL;
    err = tcf_block_get(&q.block, &q.filter_list, sch, extack);
    if (err)
    return err;
    INIT_LIST_HEAD(&q.active);
    for (i = 0; i < TCQ_ETS_MAX_BANDS; i++)
    INIT_LIST_HEAD(&q.classes[i].alist);
    return ets_qdisc_change(sch, opt, extack);
    }
#[no_mangle]
unsafe extern "C" fn ets_qdisc_reset(sch: *mut Qdisc) {
    static void ets_qdisc_reset(struct Qdisc *sch)
    {
    struct ets_sched *q = qdisc_priv(sch);
    int band;
    for (band = q.nstrict; band < q.nbands; band++) {
    if (q.classes[band].qdisc.q.qlen)
    list_del_init(&q.classes[band].alist);
    }
    for (band = 0; band < q.nbands; band++)
    qdisc_reset(q.classes[band].qdisc);
    }
#[no_mangle]
unsafe extern "C" fn ets_qdisc_destroy(sch: *mut Qdisc) {
    static void ets_qdisc_destroy(struct Qdisc *sch)
    {
    struct ets_sched *q = qdisc_priv(sch);
    int band;
    ets_offload_destroy(sch);
    tcf_block_put(q.block);
    for (band = 0; band < q.nbands; band++)
    qdisc_put(q.classes[band].qdisc);
    }
#[no_mangle]
unsafe extern "C" fn ets_qdisc_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int ets_qdisc_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct ets_sched *q = qdisc_priv(sch);
    struct nlattr *opts;
    struct nlattr *nest;
    u8 nbands, nstrict;
    int band;
    int prio;
    int err;
    err = ets_offload_dump(sch);
    if (err)
    return err;
    opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (!opts)
    goto nla_err;
    nbands = READ_ONCE(q.nbands);
    if (nla_put_u8(skb, TCA_ETS_NBANDS, nbands))
    goto nla_err;
    nstrict = READ_ONCE(q.nstrict);
    if (nstrict && nla_put_u8(skb, TCA_ETS_NSTRICT, nstrict))
    goto nla_err;
    if (nbands > nstrict) {
    nest = nla_nest_start(skb, TCA_ETS_QUANTA);
    if (!nest)
    goto nla_err;
    for (band = nstrict; band < nbands; band++) {
    if (nla_put_u32(skb, TCA_ETS_QUANTA_BAND,
    READ_ONCE(q.classes[band].quantum)))
    goto nla_err;
    }
    nla_nest_end(skb, nest);
    }
    nest = nla_nest_start(skb, TCA_ETS_PRIOMAP);
    if (!nest)
    goto nla_err;
    for (prio = 0; prio <= TC_PRIO_MAX; prio++) {
    if (nla_put_u8(skb, TCA_ETS_PRIOMAP_BAND,
    READ_ONCE(q.prio2band[prio])))
    goto nla_err;
    }
    nla_nest_end(skb, nest);
    return nla_nest_end(skb, opts);
    nla_err:
    nla_nest_cancel(skb, opts);
    return -EMSGSIZE;
    }
    static const struct Qdisc_class_ops ets_class_ops = {
    .change		= ets_class_change,
    .graft		= ets_class_graft,
    .leaf		= ets_class_leaf,
    .find		= ets_class_find,
    .qlen_notify	= ets_class_qlen_notify,
    .dump		= ets_class_dump,
    .dump_stats	= ets_class_dump_stats,
    .walk		= ets_qdisc_walk,
    .tcf_block	= ets_qdisc_tcf_block,
    .bind_tcf	= ets_qdisc_bind_tcf,
    .unbind_tcf	= ets_qdisc_unbind_tcf,
    };
    static struct Qdisc_ops ets_qdisc_ops __read_mostly = {
    .cl_ops		= &ets_class_ops,
    .id		= "ets",
    .priv_size	= sizeof(struct ets_sched),
    .enqueue	= ets_qdisc_enqueue,
    .dequeue	= ets_qdisc_dequeue,
    .peek		= qdisc_peek_dequeued,
    .change		= ets_qdisc_change,
    .init		= ets_qdisc_init,
    .reset		= ets_qdisc_reset,
    .destroy	= ets_qdisc_destroy,
    .dump		= ets_qdisc_dump,
    .owner		= THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("ets");
#[no_mangle]
unsafe extern "C" fn ets_init() -> int __init {
    static int __init ets_init(void)
    {
    return register_qdisc(&ets_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn ets_exit() -> void __exit {
    static void __exit ets_exit(void)
    {
    unregister_qdisc(&ets_qdisc_ops);
    }
    module_init(ets_init);
    module_exit(ets_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Enhanced Transmission Selection(ETS) scheduler");
