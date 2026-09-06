//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_fq_codel.c
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
// Fair Queue CoDel discipline
//
// Copyright (C) 2012,2015 Eric Dumazet <edumazet@google.com>
//

// Fair Queue CoDel.
//
// Principles :
// Packets are classified (internal classifier or external) on flows.
// This is a Stochastic model (as we use a hash, several flows
// might be hashed on same slot)
// Each flow has a CoDel managed queue.
// Flows are linked onto two (Round Robin) lists,
// so that new flows have priority on old ones.
//
// For a given flow, packets are not reordered (CoDel uses a FIFO)
// head drops only.
// ECN capability is on by default.
// Low memory footprint (64 bytes per flow)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq_codel_flow {
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub flowchain: list_head,
    pub deficit: c_int,
    pub cvars: codel_vars,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fq_codel_sched_data {
    pub /: *mut *mut *mut tcf_proto __rcu filter_list; / optional external classifier,
    pub block: *mut tcf_block,
    pub /: *mut *mut *mut fq_codel_flow flows; / Flows table [flows_cnt],
    pub /: *mut *mut *mut u32 backlogs; / backlog table [flows_cnt],
    pub /: *mut *mut u32 flows_cnt; / number of flows,
    pub /: *mut *mut u32 quantum; / psched_mtu(qdisc_dev(sch));,
    pub drop_batch_size: u32,
    pub memory_limit: u32,
    pub cparams: codel_params,
    pub cstats: codel_stats,
    pub memory_usage: u32,
    pub drop_overmemory: u32,
    pub drop_overlimit: u32,
    pub new_flow_count: u32,
    pub /: *mut *mut list_head new_flows; / list of new flows,
    pub /: *mut *mut list_head old_flows; / list of old flows,
}

    static unsigned int fq_codel_hash(const struct fq_codel_sched_data *q,
    struct sk_buff *skb)
    {
    return reciprocal_scale(skb_get_hash(skb), q.flows_cnt);
    }
    static unsigned int fq_codel_classify(struct sk_buff *skb, struct Qdisc *sch,
    int *qerr)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct tcf_proto *filter;
    struct tcf_result res;
    int result;
    if (TC_H_MAJ(skb.priority) == sch.handle &&
    TC_H_MIN(skb.priority) > 0 &&
    TC_H_MIN(skb.priority) <= q.flows_cnt)
    return TC_H_MIN(skb.priority);
    filter = rcu_dereference_bh(q.filter_list);
    if (!filter)
    return fq_codel_hash(q, skb) + 1;
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    result = tcf_classify_qdisc(skb, filter, &res, false);
    if (result >= 0) {

    switch (result) {
    case TC_ACT_STOLEN:
    case TC_ACT_QUEUED:
    case TC_ACT_TRAP:
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
    fallthrough;
    case TC_ACT_SHOT:
    return 0;
    }

    if (TC_H_MIN(res.classid) <= q.flows_cnt)
    return TC_H_MIN(res.classid);
    }
    return 0;
    }
// helper functions : might be changed when/if skb use a standard list_head
// remove one skb from head of slot queue
    static inline struct sk_buff *dequeue_head(struct fq_codel_flow *flow)
    {
    struct sk_buff *skb = flow.head;
    WRITE_ONCE(flow.head, skb.next);
    skb_mark_not_on_list(skb);
    return skb;
    }
// add skb to flow queue (tail add)
    static inline void flow_queue_add(struct fq_codel_flow *flow,
    struct sk_buff *skb)
    {
    if (flow.head == core::ptr::null_mut())
    WRITE_ONCE(flow.head, skb);
    else
    flow.tail.next = skb;
    flow.tail = skb;
    skb.next = core::ptr::null_mut();
    }
    static unsigned int fq_codel_drop(struct Qdisc *sch, unsigned int max_packets,
    struct sk_buff **to_free)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb;
    let mut maxbacklog: c_uint = 0, idx = 0, i, len;
    struct fq_codel_flow *flow;
    unsigned int threshold;
    let mut mem: c_uint = 0;
// Queue is full! Find the fat flow and drop packet(s) from it.
// This might sound expensive, but with 1024 flows, we scan
// 4KB of memory, and we dont need to handle a complex tree
// in fast path (packet queue/enqueue) with many cache misses.
// In stress mode, we'll try to drop 64 packets from the flow,
// amortizing this linear lookup to one cache line per drop.
//
    for (i = 0; i < q.flows_cnt; i++) {
    if (q.backlogs[i] > maxbacklog) {
    maxbacklog = q.backlogs[i];
    idx = i;
    }
    }
// Our goal is to drop half of this fat flow backlog
    threshold = maxbacklog >> 1;
    flow = &q.flows[idx];
    len = 0;
    i = 0;
    do {
    skb = dequeue_head(flow);
    len += qdisc_pkt_len(skb);
    mem += get_codel_cb(skb).mem_usage;
    tcf_set_qdisc_drop_reason(skb, QDISC_DROP_OVERLIMIT);
    __qdisc_drop(skb, to_free);
    } while (++i < max_packets && len < threshold);
// Tell codel to increase its signal strength also
    WRITE_ONCE(flow.cvars.count, flow.cvars.count + i);
    WRITE_ONCE(q.backlogs[idx], q.backlogs[idx] - len);
    q.memory_usage -= mem;
    __qdisc_qstats_drop(sch, i);
    qstats_backlog_sub(sch, len);
    WRITE_ONCE(sch.q.qlen, sch.q.qlen - i);
    return idx;
    }
    static int fq_codel_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    unsigned int idx, prev_backlog, prev_qlen;
    struct fq_codel_flow *flow;
    int ret;
    unsigned int pkt_len;
    bool memory_limited;
    idx = fq_codel_classify(skb, sch, &ret);
    if (idx == 0) {
    if (ret & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return ret;
    }
    idx--;
    codel_set_enqueue_time(skb);
    flow = &q.flows[idx];
    flow_queue_add(flow, skb);
    WRITE_ONCE(q.backlogs[idx], q.backlogs[idx] + qdisc_pkt_len(skb));
    qdisc_qstats_backlog_inc(sch, skb);
    if (list_empty(&flow.flowchain)) {
    list_add_tail(&flow.flowchain, &q.new_flows);
    q.new_flow_count++;
    WRITE_ONCE(flow.deficit, q.quantum);
    }
    get_codel_cb(skb).mem_usage = is_skb_wmem(skb) ? 0 : skb.truesize;
    q.memory_usage += get_codel_cb(skb).mem_usage;
    memory_limited = q.memory_usage > q.memory_limit;
    qdisc_qlen_inc(sch);
    if (sch.q.qlen <= sch.limit && !memory_limited)
    return NET_XMIT_SUCCESS;
    prev_backlog = sch.qstats.backlog;
    prev_qlen = sch.q.qlen;
// save this packet length as it might be dropped by fq_codel_drop()
    pkt_len = qdisc_pkt_len(skb);
// fq_codel_drop() is quite expensive, as it performs a linear search
// in q->backlogs[] to find a fat flow.
// So instead of dropping a single packet, drop half of its backlog
// with a 64 packets limit to not add a too big cpu spike here.
//
    ret = fq_codel_drop(sch, q.drop_batch_size, to_free);
    prev_qlen -= sch.q.qlen;
    prev_backlog -= sch.qstats.backlog;
    q.drop_overlimit += prev_qlen;
    if (memory_limited)
    q.drop_overmemory += prev_qlen;
// As we dropped packet(s), better let upper stack know this.
// If we dropped a packet for this flow, return NET_XMIT_CN,
// but in this case, our parents wont increase their backlogs.
//
    if (ret == idx) {
    qdisc_tree_reduce_backlog(sch, prev_qlen - 1,
    prev_backlog - pkt_len);
    return NET_XMIT_CN;
    }
    qdisc_tree_reduce_backlog(sch, prev_qlen, prev_backlog);
    return NET_XMIT_SUCCESS;
    }
// This is the specific function called from codel_dequeue()
// to dequeue a packet from queue. Note: backlog is handled in
// codel, we dont need to reduce it here.
//
    static struct sk_buff *dequeue_func(struct codel_vars *vars, void *ctx)
    {
    struct Qdisc *sch = ctx;
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct fq_codel_flow *flow;
    struct sk_buff *skb = core::ptr::null_mut();
    flow = container_of(vars, struct fq_codel_flow, cvars);
    if (flow.head) {
    skb = dequeue_head(flow);
    WRITE_ONCE(q.backlogs[flow - q.flows],
    q.backlogs[flow - q.flows] - qdisc_pkt_len(skb));
    q.memory_usage -= get_codel_cb(skb).mem_usage;
    qdisc_qlen_dec(sch);
    qdisc_qstats_backlog_dec(sch, skb);
    }
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn drop_func(skb: *mut sk_buff, ctx: *mut c_void) {
    static void drop_func(struct sk_buff *skb, void *ctx)
    {
    struct Qdisc *sch = ctx;
    qdisc_dequeue_drop(sch, skb, QDISC_DROP_CONGESTED);
    qdisc_qstats_drop(sch);
    }
    static struct sk_buff *__fq_codel_dequeue(struct Qdisc *sch)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb;
    struct fq_codel_flow *flow;
    struct list_head *head;
    begin:
    head = &q.new_flows;
    if (list_empty(head)) {
    head = &q.old_flows;
    if (list_empty(head))
    return core::ptr::null_mut();
    }
    flow = list_first_entry(head, struct fq_codel_flow, flowchain);
    if (flow.deficit <= 0) {
    WRITE_ONCE(flow.deficit, flow.deficit + q.quantum);
    list_move_tail(&flow.flowchain, &q.old_flows);
    goto begin;
    }
    skb = codel_dequeue(sch, &sch.qstats.backlog, &q.cparams,
    &flow.cvars, &q.cstats, qdisc_pkt_len,
    codel_get_enqueue_time, drop_func, dequeue_func);
    if (!skb) {
// force a pass through old_flows to prevent starvation
    if ((head == &q.new_flows) && !list_empty(&q.old_flows))
    list_move_tail(&flow.flowchain, &q.old_flows);
    else
    list_del_init(&flow.flowchain);
    goto begin;
    }
    qdisc_bstats_update(sch, skb);
    WRITE_ONCE(flow.deficit, flow.deficit - qdisc_pkt_len(skb));
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_dequeue_drop(sch: *mut Qdisc) {
    static void fq_codel_dequeue_drop(struct Qdisc *sch)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    if (q.cstats.drop_count) {
    qdisc_tree_reduce_backlog(sch, q.cstats.drop_count,
    q.cstats.drop_len);
    q.cstats.drop_count = 0;
    q.cstats.drop_len = 0;
    }
    }
    static struct sk_buff *fq_codel_dequeue(struct Qdisc *sch)
    {
    struct sk_buff *skb;
    skb =  __fq_codel_dequeue(sch);
    fq_codel_dequeue_drop(sch);
    return skb;
    }
    static struct sk_buff *fq_codel_peek(struct Qdisc *sch)
    {
    struct sk_buff *skb = skb_peek(&sch.gso_skb);
    if (!skb) {
    skb = __fq_codel_dequeue(sch);
    if (skb) {
    __skb_queue_head(&sch.gso_skb, skb);
// it's still part of the queue
    qdisc_qstats_backlog_inc(sch, skb);
    sch.q.qlen++;
    }
    fq_codel_dequeue_drop(sch);
    }
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_flow_purge(flow: *mut fq_codel_flow) {
    static void fq_codel_flow_purge(struct fq_codel_flow *flow)
    {
    rtnl_kfree_skbs(flow.head, flow.tail);
    WRITE_ONCE(flow.head, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_reset(sch: *mut Qdisc) {
    static void fq_codel_reset(struct Qdisc *sch)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    int i;
    INIT_LIST_HEAD(&q.new_flows);
    INIT_LIST_HEAD(&q.old_flows);
    for (i = 0; i < q.flows_cnt; i++) {
    struct fq_codel_flow *flow = q.flows + i;
    fq_codel_flow_purge(flow);
    INIT_LIST_HEAD(&flow.flowchain);
    codel_vars_init(&flow.cvars);
    }
    memset(q.backlogs, 0, q.flows_cnt * sizeof(u32));
    q.memory_usage = 0;
    }
    static const struct nla_policy fq_codel_policy[TCA_FQ_CODEL_MAX + 1] = {
    [TCA_FQ_CODEL_TARGET]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_LIMIT]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_INTERVAL]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_ECN]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_FLOWS]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_QUANTUM]	= { .type = NLA_U32 },
    [TCA_FQ_CODEL_CE_THRESHOLD] = { .type = NLA_U32 },
    [TCA_FQ_CODEL_DROP_BATCH_SIZE] = { .type = NLA_U32 },
    [TCA_FQ_CODEL_MEMORY_LIMIT] = { .type = NLA_U32 },
    [TCA_FQ_CODEL_CE_THRESHOLD_SELECTOR] = { .type = NLA_U8 },
    [TCA_FQ_CODEL_CE_THRESHOLD_MASK] = { .type = NLA_U8 },
    };
    static int fq_codel_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    let mut dropped_pkts: c_uint = 0, dropped_bytes = 0;
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct nlattr *tb[TCA_FQ_CODEL_MAX + 1];
    let mut quantum: u32 = 0;
    int err;
    err = nla_parse_nested_deprecated(tb, TCA_FQ_CODEL_MAX, opt,
    fq_codel_policy, core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_FQ_CODEL_FLOWS]) {
    if (q.flows)
    return -EINVAL;
    q.flows_cnt = nla_get_u32(tb[TCA_FQ_CODEL_FLOWS]);
    if (!q.flows_cnt ||
    q.flows_cnt > 65536)
    return -EINVAL;
    }
    if (tb[TCA_FQ_CODEL_QUANTUM]) {
    quantum = max(256U, nla_get_u32(tb[TCA_FQ_CODEL_QUANTUM]));
    if (quantum > FQ_CODEL_QUANTUM_MAX) {
    NL_SET_ERR_MSG(extack, "Invalid quantum");
    return -EINVAL;
    }
    }
    sch_tree_lock(sch);
    if (tb[TCA_FQ_CODEL_TARGET]) {
    let mut target: u64 = nla_get_u32(tb[TCA_FQ_CODEL_TARGET]);
    WRITE_ONCE(q.cparams.target,
    (target * NSEC_PER_USEC) >> CODEL_SHIFT);
    }
    if (tb[TCA_FQ_CODEL_CE_THRESHOLD]) {
    let mut val: u64 = nla_get_u32(tb[TCA_FQ_CODEL_CE_THRESHOLD]);
    WRITE_ONCE(q.cparams.ce_threshold,
    (val * NSEC_PER_USEC) >> CODEL_SHIFT);
    }
    if (tb[TCA_FQ_CODEL_CE_THRESHOLD_SELECTOR])
    WRITE_ONCE(q.cparams.ce_threshold_selector,
    nla_get_u8(tb[TCA_FQ_CODEL_CE_THRESHOLD_SELECTOR]));
    if (tb[TCA_FQ_CODEL_CE_THRESHOLD_MASK])
    WRITE_ONCE(q.cparams.ce_threshold_mask,
    nla_get_u8(tb[TCA_FQ_CODEL_CE_THRESHOLD_MASK]));
    if (tb[TCA_FQ_CODEL_INTERVAL]) {
    let mut interval: u64 = nla_get_u32(tb[TCA_FQ_CODEL_INTERVAL]);
    WRITE_ONCE(q.cparams.interval,
    (interval * NSEC_PER_USEC) >> CODEL_SHIFT);
    }
    if (tb[TCA_FQ_CODEL_LIMIT])
    WRITE_ONCE(sch.limit,
    nla_get_u32(tb[TCA_FQ_CODEL_LIMIT]));
    if (tb[TCA_FQ_CODEL_ECN])
    WRITE_ONCE(q.cparams.ecn,
    !!nla_get_u32(tb[TCA_FQ_CODEL_ECN]));
    if (quantum)
    WRITE_ONCE(q.quantum, quantum);
    if (tb[TCA_FQ_CODEL_DROP_BATCH_SIZE])
    WRITE_ONCE(q.drop_batch_size,
    max(1U, nla_get_u32(tb[TCA_FQ_CODEL_DROP_BATCH_SIZE])));
    if (tb[TCA_FQ_CODEL_MEMORY_LIMIT])
    WRITE_ONCE(q.memory_limit,
    min(1U << 31, nla_get_u32(tb[TCA_FQ_CODEL_MEMORY_LIMIT])));
    while (sch.q.qlen > sch.limit ||
    q.memory_usage > q.memory_limit) {
    struct sk_buff *skb = qdisc_dequeue_internal(sch, false);
    if (!skb)
    break;
    dropped_pkts++;
    dropped_bytes += qdisc_pkt_len(skb);
    rtnl_kfree_skbs(skb, skb);
    }
    qdisc_tree_reduce_backlog(sch, dropped_pkts, dropped_bytes);
    sch_tree_unlock(sch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_destroy(sch: *mut Qdisc) {
    static void fq_codel_destroy(struct Qdisc *sch)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    tcf_block_put(q.block);
    kvfree(q.backlogs);
    kvfree(q.flows);
    }
    static int fq_codel_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    u32 mtu;
    int i;
    int err;
    sch.limit = 10*1024;
    q.flows_cnt = 1024;
    q.memory_limit = 32 << 20; /* 32 MBytes */
    q.drop_batch_size = 64;
    mtu = clamp_t(u32, psched_mtu(qdisc_dev(sch)), 256, FQ_CODEL_QUANTUM_MAX);
    q.quantum = mtu;
    INIT_LIST_HEAD(&q.new_flows);
    INIT_LIST_HEAD(&q.old_flows);
    codel_params_init(&q.cparams);
    codel_stats_init(&q.cstats);
    q.cparams.ecn = true;
    q.cparams.mtu = mtu;
    if (opt) {
    err = fq_codel_change(sch, opt, extack);
    if (err)
    goto init_failure;
    }
    err = tcf_block_get(&q.block, &q.filter_list, sch, extack);
    if (err)
    goto init_failure;
    if (!q.flows) {
    q.flows = kvzalloc_objs(struct fq_codel_flow, q.flows_cnt);
    if (!q.flows) {
    err = -ENOMEM;
    goto init_failure;
    }
    q.backlogs = kvcalloc(q.flows_cnt, sizeof(u32), GFP_KERNEL);
    if (!q.backlogs) {
    err = -ENOMEM;
    goto alloc_failure;
    }
    for (i = 0; i < q.flows_cnt; i++) {
    struct fq_codel_flow *flow = q.flows + i;
    INIT_LIST_HEAD(&flow.flowchain);
    codel_vars_init(&flow.cvars);
    }
    }
    if (sch.limit >= 1)
    sch.flags |= TCQ_F_CAN_BYPASS;
    else
    sch.flags &= ~TCQ_F_CAN_BYPASS;
    sch.flags |= TCQ_F_DEQUEUE_DROPS;
    return 0;
    alloc_failure:
    kvfree(q.flows);
    q.flows = core::ptr::null_mut();
    init_failure:
    q.flows_cnt = 0;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int fq_codel_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    codel_time_t ce_threshold;
    struct nlattr *opts;
    opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (opts == core::ptr::null_mut())
    goto nla_put_failure;
    if (nla_put_u32(skb, TCA_FQ_CODEL_TARGET,
    codel_time_to_us(READ_ONCE(q.cparams.target))) ||
    nla_put_u32(skb, TCA_FQ_CODEL_LIMIT,
    READ_ONCE(sch.limit)) ||
    nla_put_u32(skb, TCA_FQ_CODEL_INTERVAL,
    codel_time_to_us(READ_ONCE(q.cparams.interval))) ||
    nla_put_u32(skb, TCA_FQ_CODEL_ECN,
    READ_ONCE(q.cparams.ecn)) ||
    nla_put_u32(skb, TCA_FQ_CODEL_QUANTUM,
    READ_ONCE(q.quantum)) ||
    nla_put_u32(skb, TCA_FQ_CODEL_DROP_BATCH_SIZE,
    READ_ONCE(q.drop_batch_size)) ||
    nla_put_u32(skb, TCA_FQ_CODEL_MEMORY_LIMIT,
    READ_ONCE(q.memory_limit)) ||
    nla_put_u32(skb, TCA_FQ_CODEL_FLOWS,
    READ_ONCE(q.flows_cnt)))
    goto nla_put_failure;
    ce_threshold = READ_ONCE(q.cparams.ce_threshold);
    if (ce_threshold != CODEL_DISABLED_THRESHOLD) {
    if (nla_put_u32(skb, TCA_FQ_CODEL_CE_THRESHOLD,
    codel_time_to_us(ce_threshold)))
    goto nla_put_failure;
    if (nla_put_u8(skb, TCA_FQ_CODEL_CE_THRESHOLD_SELECTOR,
    READ_ONCE(q.cparams.ce_threshold_selector)))
    goto nla_put_failure;
    if (nla_put_u8(skb, TCA_FQ_CODEL_CE_THRESHOLD_MASK,
    READ_ONCE(q.cparams.ce_threshold_mask)))
    goto nla_put_failure;
    }
    return nla_nest_end(skb, opts);
    nla_put_failure:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> c_int {
    static int fq_codel_dump_stats(struct Qdisc *sch, struct gnet_dump *d)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    struct tc_fq_codel_xstats st = {
    .type				= TCA_FQ_CODEL_XSTATS_QDISC,
    };
    struct list_head *pos;
    sch_tree_lock(sch);
    st.qdisc_stats.maxpacket = q.cstats.maxpacket;
    st.qdisc_stats.drop_overlimit = q.drop_overlimit;
    st.qdisc_stats.ecn_mark = q.cstats.ecn_mark;
    st.qdisc_stats.new_flow_count = q.new_flow_count;
    st.qdisc_stats.ce_mark = q.cstats.ce_mark;
    st.qdisc_stats.memory_usage  = q.memory_usage;
    st.qdisc_stats.drop_overmemory = q.drop_overmemory;
    list_for_each(pos, &q.new_flows)
    st.qdisc_stats.new_flows_len++;
    list_for_each(pos, &q.old_flows)
    st.qdisc_stats.old_flows_len++;
    sch_tree_unlock(sch);
    return gnet_stats_copy_app(d, &st, sizeof(st));
    }
    static struct Qdisc *fq_codel_leaf(struct Qdisc *sch, unsigned long arg)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long fq_codel_find(struct Qdisc *sch, u32 classid)
    {
    return 0;
    }
    static unsigned long fq_codel_bind(struct Qdisc *sch, unsigned long parent,
    u32 classid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_unbind(q: *mut Qdisc, cl: c_ulong) {
    static void fq_codel_unbind(struct Qdisc *q, unsigned long cl)
    {
    }
    static struct tcf_block *fq_codel_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    if (cl)
    return core::ptr::null_mut();
    return q.block;
    }
    static int fq_codel_dump_class(struct Qdisc *sch, unsigned long cl,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    tcm.tcm_handle |= TC_H_MIN(cl);
    return 0;
    }
    static int fq_codel_dump_class_stats(struct Qdisc *sch, unsigned long cl,
    struct gnet_dump *d)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    let mut idx: u32 = cl - 1;
    let mut qs: gnet_stats_queue = { 0 };
    struct tc_fq_codel_xstats xstats;
    if (idx < q.flows_cnt) {
    const struct fq_codel_flow *flow = &q.flows[idx];
    const struct sk_buff *skb;
    memset(&xstats, 0, sizeof(xstats));
    xstats.type = TCA_FQ_CODEL_XSTATS_CLASS;
    xstats.class_stats.deficit = READ_ONCE(flow.deficit);
    xstats.class_stats.ldelay =
    codel_time_to_us(READ_ONCE(flow.cvars.ldelay));
    xstats.class_stats.count = READ_ONCE(flow.cvars.count);
    xstats.class_stats.lastcount = READ_ONCE(flow.cvars.lastcount);
    xstats.class_stats.dropping = READ_ONCE(flow.cvars.dropping);
    if (xstats.class_stats.dropping) {
    codel_tdiff_t delta = READ_ONCE(flow.cvars.drop_next) -
    codel_get_time();
    xstats.class_stats.drop_next = (delta >= 0) ?
    codel_time_to_us(delta) :
    -codel_time_to_us(-delta);
    }
    if (READ_ONCE(flow.head)) {
    sch_tree_lock(sch);
    skb = flow.head;
    while (skb) {
    qs.qlen++;
    skb = skb.next;
    }
    sch_tree_unlock(sch);
    }
    qs.backlog = READ_ONCE(q.backlogs[idx]);
    qs.drops = 0;
    }
    if (gnet_stats_copy_queue(d, core::ptr::null_mut(), &qs, qs.qlen) < 0)
    return -1;
    if (idx < q.flows_cnt)
    return gnet_stats_copy_app(d, &xstats, sizeof(xstats));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void fq_codel_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    struct fq_codel_sched_data *q = qdisc_priv(sch);
    unsigned int i;
    if (arg.stop)
    return;
    for (i = 0; i < q.flows_cnt; i++) {
    if (list_empty(&q.flows[i].flowchain)) {
    arg.count++;
    continue;
    }
    if (!tc_qdisc_stats_dump(sch, i + 1, arg))
    break;
    }
    }
    static const struct Qdisc_class_ops fq_codel_class_ops = {
    .leaf		=	fq_codel_leaf,
    .find		=	fq_codel_find,
    .tcf_block	=	fq_codel_tcf_block,
    .bind_tcf	=	fq_codel_bind,
    .unbind_tcf	=	fq_codel_unbind,
    .dump		=	fq_codel_dump_class,
    .dump_stats	=	fq_codel_dump_class_stats,
    .walk		=	fq_codel_walk,
    };
    static struct Qdisc_ops fq_codel_qdisc_ops __read_mostly = {
    .cl_ops		=	&fq_codel_class_ops,
    .id		=	"fq_codel",
    .priv_size	=	sizeof(struct fq_codel_sched_data),
    .enqueue	=	fq_codel_enqueue,
    .dequeue	=	fq_codel_dequeue,
    .peek		=	fq_codel_peek,
    .init		=	fq_codel_init,
    .reset		=	fq_codel_reset,
    .destroy	=	fq_codel_destroy,
    .change		=	fq_codel_change,
    .dump		=	fq_codel_dump,
    .dump_stats =	fq_codel_dump_stats,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("fq_codel");
#[no_mangle]
unsafe extern "C" fn fq_codel_module_init() -> int __init {
    static int __init fq_codel_module_init(void)
    {
    return register_qdisc(&fq_codel_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn fq_codel_module_exit() -> void __exit {
    static void __exit fq_codel_module_exit(void)
    {
    unregister_qdisc(&fq_codel_qdisc_ops);
    }
    module_init(fq_codel_module_init)
    module_exit(fq_codel_module_exit)
    MODULE_AUTHOR("Eric Dumazet");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Fair Queue CoDel discipline");
