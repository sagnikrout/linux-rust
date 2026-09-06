//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_dualpi2.c
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
// Copyright (C) 2024 Nokia
//
// Author: Koen De Schepper <koen.de_schepper@nokia-bell-labs.com>
// Author: Olga Albisser <olga@albisser.org>
// Author: Henrik Steen <henrist@henrist.net>
// Author: Olivier Tilmans <olivier.tilmans@nokia.com>
// Author: Chia-Yu Chang <chia-yu.chang@nokia-bell-labs.com>
//
// DualPI Improved with a Square (dualpi2):
// - Supports congestion controls that comply with the Prague requirements
// in RFC9331 (e.g. TCP-Prague)
// - Supports coupled dual-queue with PI2 as defined in RFC9332
// - Supports ECN L4S-identifier (IP.ECN==0b*1)
//
// note: Although DCTCP and BBRv3 can use shallow-threshold ECN marks,
// they do not meet the 'Prague L4S Requirements' listed in RFC 9331
// Section 4, so they can only be used with DualPI2 in a datacenter
// context.
//
// References:
// - RFC9332: https://datatracker.ietf.org/doc/html/rfc9332
// - De Schepper, Koen, et al. "PI 2: A linearized AQM for both classic and
// scalable TCP."  in proc. ACM CoNEXT'16, 2016.
//

// 32b enable to support flows with windows up to ~8.6 * 1e9 packets
// i.e., twice the maximal snd_cwnd.
// MAX_PROB must be consistent with the RNG in dualpi2_roll().
//

// alpha/beta values exchanged over netlink are in units of 256ns
pub const ALPHA_BETA_SHIFT: c_int = 8;
// Scaled values of alpha/beta must fit in 32b to avoid overflow in later
// computations. Consequently (see and dualpi2_scale_alpha_beta()), their
// netlink-provided values can use at most 31b, i.e. be at most (2^23)-1
// (~4MHz) as those are given in 1/256th. This enable to tune alpha/beta to
// control flows whose maximal RTTs can be in usec up to few secs.
//

// Internal alpha/beta are in units of 64ns.
// This enables to use all alpha/beta values in the allowed range without loss
// of precision due to rounding when scaling them internally, e.g.,
// scale_alpha_beta(1) will not round down to 0.
//
pub const ALPHA_BETA_GRANULARITY: c_int = 6;

// We express the weights (wc, wl) in %, i.e., wc + wl = 100
pub const MAX_WC: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dualpi2_sched_data {
    pub /: *mut *mut *mut Qdisc l_queue; / The L4S Low latency queue (L-queue),
    pub /: *mut *mut *mut Qdisc sch; / The Classic queue (C-queue),
// Registered tc filters
    pub tcf_filters: *mut tcf_proto __rcu,
    pub tcf_block: *mut tcf_block,
// PI2 parameters
    pub /: *mut *mut u64 pi2_target; / Target delay in nanoseconds,
    pub /: *mut *mut u32 pi2_tupdate; / Timer frequency in nanoseconds,
    pub /: *mut *mut u32 pi2_prob; / Base PI probability,
    pub /: *mut *mut u32 pi2_alpha; / Gain factor for the integral rate response,
    pub /: *mut *mut u32 pi2_beta; / Gain factor for the proportional response,
    pub /: *mut *mut hrtimer pi2_timer; / prob update timer,
// Step AQM (L-queue only) parameters
    pub /: *mut *mut u32 step_thresh; / Step threshold,
    pub /: *mut *mut bool step_in_packets; / Step thresh in packets (1) or time (0),
// C-queue starvation protection
    pub /: *mut *mut s32 c_protection_credit; / Credit (sign indicates which queue),
    pub /: *mut *mut s32 c_protection_init; / Reset value of the credit,
    pub /: *mut *mut u8 c_protection_wc; / C-queue weight (between 0 and MAX_WC),
    pub /: *mut *mut u8 c_protection_wl; / L-queue weight (MAX_WC - wc),
// General dualQ parameters
    pub /: *mut *mut u32 memory_limit; / Memory limit of both queues,
    pub /: *mut *mut u8 coupling_factor;/ Coupling factor (k) between both queues,
    pub /: *mut *mut u8 ecn_mask; / Mask to match packets into L-queue,
    pub /: *mut *mut u32 min_qlen_step; / Minimum queue length to apply step thresh,
    pub /: *mut *mut bool drop_early; / Drop at enqueue (1) instead of dequeue (0),
    pub /: *mut *mut bool drop_overload; / Drop (1) on overload, or overflow (0),
    pub /: *mut *mut bool split_gso; / Split aggregated skb (1) or leave as is (0),
// Statistics
    pub /: *mut *mut u64 c_head_ts; / Enqueue timestamp of the C-queue head,
    pub /: *mut *mut u64 l_head_ts; / Enqueue timestamp of the L-queue head,
    pub /: *mut *mut u64 last_qdelay; / Q delay val at the last probability update,
    pub /: *mut *mut u32 packets_in_c; / Enqueue packet counter of the C-queue,
    pub /: *mut *mut u32 packets_in_l; / Enqueue packet counter of the L-queue,
    pub /: *mut *mut u32 maxq; / Maximum queue size of the C-queue,
    pub /: *mut *mut u32 ecn_mark; / ECN mark pkt counter due to PI probability,
    pub /: *mut *mut u32 step_marks; / ECN mark pkt counter due to step AQM,
    pub /: *mut *mut u32 memory_used; / Memory used of both queues,
    pub /: *mut *mut u32 max_memory_used;/ Maximum used memory,
// Deferred drop statistics
    pub /: *mut *mut u32 deferred_drops_cnt; / Packets dropped,
    pub /: *mut *mut u32 deferred_drops_len; / Bytes dropped,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dualpi2_skb_cb {
    pub /: *mut *mut u64 ts; / Timestamp at enqueue,
    u8 apply_step:1,	/* Can we apply the step threshold */
    classified:2,	/* Packet classification results */
    pub /: *mut *mut ect:2; / Packet ECT codepoint,
}

    enum dualpi2_classification_results {
    DUALPI2_C_CLASSIC	= 0,	/* C-queue */
    DUALPI2_C_L4S		= 1,	/* L-queue (scale mark/classic drop) */
    DUALPI2_C_LLLL		= 2,	/* L-queue (no drops/marks) */
    __DUALPI2_C_MAX			/* Keep last*/
    };
    static struct dualpi2_skb_cb *dualpi2_skb_cb(struct sk_buff *skb)
    {
    qdisc_cb_private_validate(skb, sizeof(struct dualpi2_skb_cb));
    return (struct dualpi2_skb_cb *)qdisc_skb_cb(skb).data;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_sojourn_time(skb: *mut sk_buff, reference: u64) -> u64 {
    static u64 dualpi2_sojourn_time(struct sk_buff *skb, u64 reference)
    {
    return reference - dualpi2_skb_cb(skb).ts;
    }
#[no_mangle]
unsafe extern "C" fn head_enqueue_time(q: *mut Qdisc) -> u64 {
    static u64 head_enqueue_time(struct Qdisc *q)
    {
    struct sk_buff *skb = qdisc_peek_head(q);
    return skb ? dualpi2_skb_cb(skb).ts : 0;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_scale_alpha_beta(param: u32) -> u32 {
    static u32 dualpi2_scale_alpha_beta(u32 param)
    {
    let mut tmp: u64 = ((u64)param * MAX_PROB >> ALPHA_BETA_SCALING);
    do_div(tmp, NSEC_PER_SEC);
    return tmp;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_unscale_alpha_beta(param: u32) -> u32 {
    static u32 dualpi2_unscale_alpha_beta(u32 param)
    {
    let mut tmp: u64 = ((u64)param * NSEC_PER_SEC << ALPHA_BETA_SCALING);
    do_div(tmp, MAX_PROB);
    return tmp;
    }
#[no_mangle]
unsafe extern "C" fn next_pi2_timeout(q: *mut dualpi2_sched_data) -> ktime_t {
    static ktime_t next_pi2_timeout(struct dualpi2_sched_data *q)
    {
    return ktime_add_ns(ktime_get_ns(), q.pi2_tupdate);
    }
#[no_mangle]
unsafe extern "C" fn skb_is_l4s(skb: *mut sk_buff) -> bool {
    static bool skb_is_l4s(struct sk_buff *skb)
    {
    return dualpi2_skb_cb(skb).classified == DUALPI2_C_L4S;
    }
#[no_mangle]
unsafe extern "C" fn skb_in_l_queue(skb: *mut sk_buff) -> bool {
    static bool skb_in_l_queue(struct sk_buff *skb)
    {
    return dualpi2_skb_cb(skb).classified != DUALPI2_C_CLASSIC;
    }
#[no_mangle]
unsafe extern "C" fn skb_apply_step(skb: *mut sk_buff, q: *mut dualpi2_sched_data) -> bool {
    static bool skb_apply_step(struct sk_buff *skb, struct dualpi2_sched_data *q)
    {
    return skb_is_l4s(skb) && qdisc_qlen(q.l_queue) >= q.min_qlen_step;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_mark(q: *mut dualpi2_sched_data, skb: *mut sk_buff) -> bool {
    static bool dualpi2_mark(struct dualpi2_sched_data *q, struct sk_buff *skb)
    {
    if (INET_ECN_set_ce(skb)) {
    WRITE_ONCE(q.ecn_mark, q.ecn_mark + 1);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_reset_c_protection(q: *mut dualpi2_sched_data) {
    static void dualpi2_reset_c_protection(struct dualpi2_sched_data *q)
    {
    WRITE_ONCE(q.c_protection_credit, q.c_protection_init);
    }
// This computes the initial credit value and WRR weight for the L queue (wl)
// from the weight of the C queue (wc).
// If wl > wc, the scheduler will start with the L queue when reset.
//
    static void dualpi2_calculate_c_protection(struct Qdisc *sch,
    struct dualpi2_sched_data *q, u32 wc)
    {
    q.c_protection_wc = wc;
    q.c_protection_wl = MAX_WC - wc;
    q.c_protection_init = (s32)psched_mtu(qdisc_dev(sch)) *
    ((int)q.c_protection_wc - (int)q.c_protection_wl);
    dualpi2_reset_c_protection(q);
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_roll(prob: u32) -> bool {
    static bool dualpi2_roll(u32 prob)
    {
    return get_random_u32() <= prob;
    }
// Packets in the C-queue are subject to a marking probability pC, which is the
// square of the internal PI probability (i.e., have an overall lower mark/drop
// probability). If the qdisc is overloaded, ignore ECT values and only drop.
//
// Note that this marking scheme is also applied to L4S packets during overload.
// Return true if packet dropping is required in C queue
//
    static bool dualpi2_classic_marking(struct dualpi2_sched_data *q,
    struct sk_buff *skb, u32 prob,
    bool overload)
    {
    if (dualpi2_roll(prob) && dualpi2_roll(prob)) {
    if (overload || dualpi2_skb_cb(skb).ect == INET_ECN_NOT_ECT)
    return true;
    dualpi2_mark(q, skb);
    }
    return false;
    }
// Packets in the L-queue are subject to a marking probability pL given by the
// internal PI probability scaled by the coupling factor.
//
// On overload (i.e., @local_l_prob is >= 100%):
// - if the qdisc is configured to trade losses to preserve latency (i.e.,
// @q->drop_overload), apply classic drops first before marking.
// - otherwise, preserve the "no loss" property of ECN at the cost of queueing
// delay, eventually resulting in taildrop behavior once sch->limit is
// reached.
// Return true if packet dropping is required in L queue
//
    static bool dualpi2_scalable_marking(struct dualpi2_sched_data *q,
    struct sk_buff *skb,
    u64 local_l_prob, u32 prob,
    bool overload)
    {
    if (overload) {
// Apply classic drop
    if (!q.drop_overload ||
    !(dualpi2_roll(prob) && dualpi2_roll(prob)))
    goto mark;
    return true;
    }
// We can safely cut the upper 32b as overload==false
    if (dualpi2_roll(local_l_prob)) {
// Non-ECT packets could have classified as L4S by filters.
    if (dualpi2_skb_cb(skb).ect == INET_ECN_NOT_ECT)
    return true;
    mark:
    dualpi2_mark(q, skb);
    }
    return false;
    }
// Decide whether a given packet must be dropped (or marked if ECT), according
// to the PI2 probability.
//
// Never mark/drop if we have a standing queue of less than 2 MTUs.
//
    static bool must_drop(struct Qdisc *sch, struct dualpi2_sched_data *q,
    struct sk_buff *skb)
    {
    u64 local_l_prob;
    bool overload;
    u32 prob;
    if (sch.qstats.backlog < 2 * psched_mtu(qdisc_dev(sch)))
    return false;
    prob = READ_ONCE(q.pi2_prob);
    local_l_prob = (u64)prob * q.coupling_factor;
    overload = local_l_prob > MAX_PROB;
    switch (dualpi2_skb_cb(skb).classified) {
    case DUALPI2_C_CLASSIC:
    return dualpi2_classic_marking(q, skb, prob, overload);
    case DUALPI2_C_L4S:
    return dualpi2_scalable_marking(q, skb, local_l_prob, prob,
    overload);
    default: /* DUALPI2_C_LLLL */
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_read_ect(skb: *mut sk_buff) {
    static void dualpi2_read_ect(struct sk_buff *skb)
    {
    struct dualpi2_skb_cb *cb = dualpi2_skb_cb(skb);
    let mut wlen: c_int = skb_network_offset(skb);
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    wlen += sizeof(struct iphdr);
    if (!pskb_may_pull(skb, wlen) ||
    skb_try_make_writable(skb, wlen))
    goto not_ecn;
    cb.ect = ipv4_get_dsfield(ip_hdr(skb)) & INET_ECN_MASK;
    break;
    case htons(ETH_P_IPV6):
    wlen += sizeof(struct ipv6hdr);
    if (!pskb_may_pull(skb, wlen) ||
    skb_try_make_writable(skb, wlen))
    goto not_ecn;
    cb.ect = ipv6_get_dsfield(ipv6_hdr(skb)) & INET_ECN_MASK;
    break;
    default:
    goto not_ecn;
    }
    return;
    not_ecn:
// Non pullable/writable packets can only be dropped hence are
// classified as not ECT.
//
    cb.ect = INET_ECN_NOT_ECT;
    }
    static int dualpi2_skb_classify(struct dualpi2_sched_data *q,
    struct sk_buff *skb)
    {
    struct dualpi2_skb_cb *cb = dualpi2_skb_cb(skb);
    struct tcf_result res;
    struct tcf_proto *fl;
    int result;
    cb.classified = DUALPI2_C_CLASSIC;
    dualpi2_read_ect(skb);
    if (cb.ect & q.ecn_mask) {
    cb.classified = DUALPI2_C_L4S;
    return NET_XMIT_SUCCESS;
    }
    if (TC_H_MAJ(skb.priority) == q.sch.handle &&
    TC_H_MIN(skb.priority) < __DUALPI2_C_MAX) {
    cb.classified = TC_H_MIN(skb.priority);
    return NET_XMIT_SUCCESS;
    }
    fl = rcu_dereference_bh(q.tcf_filters);
    if (!fl)
    return NET_XMIT_SUCCESS;
    result = tcf_classify_qdisc(skb, fl, &res, false);
    if (result >= 0) {

    switch (result) {
    case TC_ACT_STOLEN:
    case TC_ACT_QUEUED:
    case TC_ACT_TRAP:
    return NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
    case TC_ACT_SHOT:
    return NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    }

    cb.classified = TC_H_MIN(res.classid) < __DUALPI2_C_MAX ?
    TC_H_MIN(res.classid) : DUALPI2_C_CLASSIC;
    }
    return NET_XMIT_SUCCESS;
    }
    static int dualpi2_enqueue_skb(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    struct dualpi2_skb_cb *cb;
    if (unlikely(qdisc_qlen(sch) >= sch.limit) ||
    unlikely((u64)q.memory_used + skb.truesize > q.memory_limit)) {
    qdisc_qstats_overlimit(sch);
    if (skb_in_l_queue(skb))
    qdisc_qstats_overlimit(q.l_queue);
    return qdisc_drop_reason(skb, sch, to_free, QDISC_DROP_OVERLIMIT);
    }
    if (q.drop_early && must_drop(sch, q, skb)) {
    qdisc_drop_reason(skb, sch, to_free, QDISC_DROP_CONGESTED);
    return NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    }
    cb = dualpi2_skb_cb(skb);
    cb.ts = ktime_get_ns();
    WRITE_ONCE(q.memory_used, q.memory_used + skb.truesize);
    if (q.memory_used > q.max_memory_used)
    WRITE_ONCE(q.max_memory_used, q.memory_used);
    if (qdisc_qlen(sch) > q.maxq)
    WRITE_ONCE(q.maxq, qdisc_qlen(sch));
    if (skb_in_l_queue(skb)) {
// Apply step thresh if skb is L4S && L-queue len >= min_qlen
    dualpi2_skb_cb(skb).apply_step = skb_apply_step(skb, q);
// Keep the overall qdisc stats consistent
    qdisc_qlen_inc(sch);
    qdisc_qstats_backlog_inc(sch, skb);
    WRITE_ONCE(q.packets_in_l, q.packets_in_l + 1);
    if (!q.l_head_ts)
    WRITE_ONCE(q.l_head_ts, cb.ts);
    return qdisc_enqueue_tail(skb, q.l_queue);
    }
    WRITE_ONCE(q.packets_in_c, q.packets_in_c + 1);
    if (!q.c_head_ts)
    WRITE_ONCE(q.c_head_ts, cb.ts);
    return qdisc_enqueue_tail(skb, sch);
    }
// By default, dualpi2 will split GSO skbs into independent skbs and enqueue
// each of those individually. This yields the following benefits, at the
// expense of CPU usage:
// - Finer-grained AQM actions as the sub-packets of a burst no longer share the
// same fate (e.g., the random mark/drop probability is applied individually)
// - Improved precision of the starvation protection/WRR scheduler at dequeue,
// as the size of the dequeued packets will be smaller.
//
    static int dualpi2_qdisc_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    int err;
    err = dualpi2_skb_classify(q, skb);
    if (err != NET_XMIT_SUCCESS) {
    if (err & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return err;
    }
    if (q.split_gso && skb_is_gso(skb)) {
    netdev_features_t features;
    struct sk_buff *nskb, *next;
    int cnt, byte_len, orig_len;
    int err;
    features = netif_skb_features(skb);
    nskb = skb_gso_segment(skb, features & ~NETIF_F_GSO_MASK);
    if (IS_ERR_OR_NULL(nskb))
    return qdisc_drop(skb, sch, to_free);
    cnt = 0;
    byte_len = 0;
    orig_len = qdisc_pkt_len(skb);
    skb_list_walk_safe(nskb, nskb, next) {
    skb_mark_not_on_list(nskb);
// Iterate through GSO fragments of an skb:
// (1) Set pkt_len from the single GSO fragments
// (2) Copy classified and ect values of an skb
// (3) Enqueue fragment & set ts in dualpi2_enqueue_skb
//
    qdisc_skb_cb(nskb).pkt_len = nskb.len;
    qdisc_skb_cb(nskb).pkt_segs = 1;
    dualpi2_skb_cb(nskb).classified =
    dualpi2_skb_cb(skb).classified;
    dualpi2_skb_cb(nskb).ect = dualpi2_skb_cb(skb).ect;
    err = dualpi2_enqueue_skb(nskb, sch, to_free);
    if (err == NET_XMIT_SUCCESS) {
// Compute the backlog adjustment that needs
// to be propagated in the qdisc tree to reflect
// all new skbs successfully enqueued.
//
    ++cnt;
    byte_len += nskb.len;
    }
    }
    if (cnt > 0) {
// The caller will add the original skb stats to its
// backlog, compensate this if any nskb is enqueued.
//
    qdisc_tree_reduce_backlog(sch, 1 - cnt,
    orig_len - byte_len);
    }
    consume_skb(skb);
    return cnt > 0 ? NET_XMIT_SUCCESS : err;
    }
    return dualpi2_enqueue_skb(skb, sch, to_free);
    }
// Select the queue from which the next packet can be dequeued, ensuring that
// neither queue can starve the other with a WRR scheduler.
//
// The sign of the WRR credit determines the next queue, while the size of
// the dequeued packet determines the magnitude of the WRR credit change. If
// either queue is empty, the WRR credit is kept unchanged.
//
// As the dequeued packet can be dropped later, the caller has to perform the
// qdisc_bstats_update() calls.
//
    static struct sk_buff *dequeue_packet(struct Qdisc *sch,
    struct dualpi2_sched_data *q,
    int *credit_change,
    u64 now)
    {
    struct sk_buff *skb = core::ptr::null_mut();
    int c_len;
// credit_change = 0;
    c_len = qdisc_qlen(sch) - qdisc_qlen(q.l_queue);
    if (qdisc_qlen(q.l_queue) && (!c_len || q.c_protection_credit <= 0)) {
    skb = __qdisc_dequeue_head(&q.l_queue.q);
    WRITE_ONCE(q.l_head_ts, head_enqueue_time(q.l_queue));
    if (c_len)
// credit_change = q->c_protection_wc;
    qdisc_qstats_backlog_dec(q.l_queue, skb);
// Keep the global queue size consistent
    qdisc_qlen_dec(sch);
    } else if (c_len) {
    skb = __qdisc_dequeue_head(&sch.q);
    WRITE_ONCE(q.c_head_ts, head_enqueue_time(sch));
    if (qdisc_qlen(q.l_queue))
// credit_change = ~((s32)q->c_protection_wl) + 1;
    } else {
    dualpi2_reset_c_protection(q);
    return core::ptr::null_mut();
    }
    WRITE_ONCE(q.memory_used, q.memory_used - skb.truesize);
// credit_change *= qdisc_pkt_len(skb);
    qdisc_qstats_backlog_dec(sch, skb);
    return skb;
    }
    static int do_step_aqm(struct dualpi2_sched_data *q, struct sk_buff *skb,
    u64 now)
    {
    let mut qdelay: u64 = 0;
    if (q.step_in_packets)
    qdelay = qdisc_qlen(q.l_queue);
    else
    qdelay = dualpi2_sojourn_time(skb, now);
    if (dualpi2_skb_cb(skb).apply_step && qdelay > q.step_thresh) {
    if (!dualpi2_skb_cb(skb).ect) {
// Drop this non-ECT packet
    return 1;
    }
    if (dualpi2_mark(q, skb))
    WRITE_ONCE(q.step_marks, q.step_marks + 1);
    }
    qdisc_bstats_update(q.l_queue, skb);
    return 0;
    }
    static void drop_and_retry(struct dualpi2_sched_data *q, struct sk_buff *skb,
    struct Qdisc *sch, enum qdisc_drop_reason reason)
    {
    ++q.deferred_drops_cnt;
    q.deferred_drops_len += qdisc_pkt_len(skb);
    qdisc_dequeue_drop(sch, skb, reason);
    qdisc_qstats_drop(sch);
    }
    static struct sk_buff *__dualpi2_qdisc_dequeue(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb;
    int credit_change;
    u64 now;
    now = ktime_get_ns();
    while ((skb = dequeue_packet(sch, q, &credit_change, now))) {
    if (!q.drop_early && must_drop(sch, q, skb)) {
    drop_and_retry(q, skb, sch, QDISC_DROP_CONGESTED);
    continue;
    }
    if (skb_in_l_queue(skb) && do_step_aqm(q, skb, now)) {
    qdisc_qstats_drop(q.l_queue);
    drop_and_retry(q, skb, sch, QDISC_DROP_L4S_STEP_NON_ECN);
    continue;
    }
    WRITE_ONCE(q.c_protection_credit,
    q.c_protection_credit + credit_change);
    qdisc_bstats_update(sch, skb);
    break;
    }
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_dequeue_drop(sch: *mut Qdisc) {
    static void dualpi2_dequeue_drop(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    if (q.deferred_drops_cnt) {
    qdisc_tree_reduce_backlog(sch, q.deferred_drops_cnt,
    q.deferred_drops_len);
    q.deferred_drops_cnt = 0;
    q.deferred_drops_len = 0;
    }
    }
    static struct sk_buff *dualpi2_qdisc_dequeue(struct Qdisc *sch)
    {
    struct sk_buff *skb;
    skb = __dualpi2_qdisc_dequeue(sch);
    dualpi2_dequeue_drop(sch);
    return skb;
    }
    static struct sk_buff *dualpi2_peek(struct Qdisc *sch)
    {
    struct sk_buff *skb = skb_peek(&sch.gso_skb);
    if (!skb) {
    skb = __dualpi2_qdisc_dequeue(sch);
    if (skb) {
    __skb_queue_head(&sch.gso_skb, skb);
// it's still part of the queue
    qdisc_qstats_backlog_inc(sch, skb);
    sch.q.qlen++;
    }
    dualpi2_dequeue_drop(sch);
    }
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn __scale_delta(diff: u64) -> i64 {
    static s64 __scale_delta(u64 diff)
    {
    do_div(diff, 1 << ALPHA_BETA_GRANULARITY);
    return diff;
    }
    static void get_queue_delays(struct dualpi2_sched_data *q, u64 *qdelay_c,
    u64 *qdelay_l)
    {
    u64 now, qc, ql;
    now = ktime_get_ns();
    qc = READ_ONCE(q.c_head_ts);
    ql = READ_ONCE(q.l_head_ts);
// qdelay_c = qc ? now - qc : 0;
// qdelay_l = ql ? now - ql : 0;
    }
#[no_mangle]
unsafe extern "C" fn calculate_probability(sch: *mut Qdisc) -> u32 {
    static u32 calculate_probability(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    u32 new_prob;
    u64 qdelay_c;
    u64 qdelay_l;
    u64 qdelay;
    s64 delta;
    get_queue_delays(q, &qdelay_c, &qdelay_l);
    qdelay = max(qdelay_l, qdelay_c);
// Alpha and beta take at most 32b, i.e, the delay difference would
// overflow for queuing delay differences > ~4.2sec.
//
    delta = ((s64)qdelay - (s64)q.pi2_target) * q.pi2_alpha;
    delta += ((s64)qdelay - (s64)q.last_qdelay) * q.pi2_beta;
    q.last_qdelay = qdelay;
// Bound new_prob between 0 and MAX_PROB
    if (delta > 0) {
    new_prob = __scale_delta(delta) + q.pi2_prob;
    if (new_prob < q.pi2_prob)
    new_prob = MAX_PROB;
    } else {
    new_prob = q.pi2_prob - __scale_delta(~delta + 1);
    if (new_prob > q.pi2_prob)
    new_prob = 0;
    }
// If we do not drop on overload, ensure we cap the L4S probability to
// 100% to keep window fairness when overflowing.
//
    if (!q.drop_overload)
    return min_t(u32, new_prob, MAX_PROB / q.coupling_factor);
    return new_prob;
    }
#[no_mangle]
unsafe extern "C" fn get_memory_limit(sch: *mut Qdisc, limit: u32) -> u32 {
    static u32 get_memory_limit(struct Qdisc *sch, u32 limit)
    {
// Apply rule of thumb, i.e., doubling the packet length,
// to further include per packet overhead in memory_limit.
//
    let mut memlim: u64 = mul_u32_u32(limit, 2 * psched_mtu(qdisc_dev(sch)));
    if (upper_32_bits(memlim))
    return U32_MAX;
    else
    return lower_32_bits(memlim);
    }
#[no_mangle]
unsafe extern "C" fn convert_us_to_nsec(us: u32) -> u32 {
    static u32 convert_us_to_nsec(u32 us)
    {
    let mut ns: u64 = mul_u32_u32(us, NSEC_PER_USEC);
    if (upper_32_bits(ns))
    return U32_MAX;
    return lower_32_bits(ns);
    }
#[no_mangle]
unsafe extern "C" fn convert_ns_to_usec(ns: u64) -> u32 {
    static u32 convert_ns_to_usec(u64 ns)
    {
    do_div(ns, NSEC_PER_USEC);
    if (upper_32_bits(ns))
    return U32_MAX;
    return lower_32_bits(ns);
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_timer(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart dualpi2_timer(struct hrtimer *timer)
    {
    struct dualpi2_sched_data *q = timer_container_of(q, timer, pi2_timer);
    struct Qdisc *sch = q.sch;
    spinlock_t *root_lock; /* to lock qdisc for probability calculations */
    rcu_read_lock();
    root_lock = qdisc_lock(qdisc_root_sleeping(sch));
    spin_lock(root_lock);
    WRITE_ONCE(q.pi2_prob, calculate_probability(sch));
    hrtimer_set_expires(&q.pi2_timer, next_pi2_timeout(q));
    spin_unlock(root_lock);
    rcu_read_unlock();
    return HRTIMER_RESTART;
    }
    static struct netlink_range_validation dualpi2_alpha_beta_range = {
    .min = 1,
    .max = ALPHA_BETA_MAX,
    };
    static const struct nla_policy dualpi2_policy[TCA_DUALPI2_MAX + 1] = {
    [TCA_DUALPI2_LIMIT]		= NLA_POLICY_MIN(NLA_U32, 1),
    [TCA_DUALPI2_MEMORY_LIMIT]	= NLA_POLICY_MIN(NLA_U32, 1),
    [TCA_DUALPI2_TARGET]		= { .type = NLA_U32 },
    [TCA_DUALPI2_TUPDATE]		= NLA_POLICY_MIN(NLA_U32, 1),
    [TCA_DUALPI2_ALPHA]		=
    NLA_POLICY_FULL_RANGE(NLA_U32, &dualpi2_alpha_beta_range),
    [TCA_DUALPI2_BETA]		=
    NLA_POLICY_FULL_RANGE(NLA_U32, &dualpi2_alpha_beta_range),
    [TCA_DUALPI2_STEP_THRESH_PKTS]	= { .type = NLA_U32 },
    [TCA_DUALPI2_STEP_THRESH_US]	= { .type = NLA_U32 },
    [TCA_DUALPI2_MIN_QLEN_STEP]	= { .type = NLA_U32 },
    [TCA_DUALPI2_COUPLING]		= NLA_POLICY_MIN(NLA_U8, 1),
    [TCA_DUALPI2_DROP_OVERLOAD]	=
    NLA_POLICY_MAX(NLA_U8, TCA_DUALPI2_DROP_OVERLOAD_MAX),
    [TCA_DUALPI2_DROP_EARLY]	=
    NLA_POLICY_MAX(NLA_U8, TCA_DUALPI2_DROP_EARLY_MAX),
    [TCA_DUALPI2_C_PROTECTION]	=
    NLA_POLICY_RANGE(NLA_U8, 0, MAX_WC),
    [TCA_DUALPI2_ECN_MASK]		=
    NLA_POLICY_RANGE(NLA_U8, TC_DUALPI2_ECN_MASK_L4S_ECT,
    TCA_DUALPI2_ECN_MASK_MAX),
    [TCA_DUALPI2_SPLIT_GSO]		=
    NLA_POLICY_MAX(NLA_U8, TCA_DUALPI2_SPLIT_GSO_MAX),
    };
    static int dualpi2_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct nlattr *tb[TCA_DUALPI2_MAX + 1];
    struct dualpi2_sched_data *q;
    int old_backlog;
    int old_qlen;
    int err;
    if (!opt || !nla_len(opt)) {
    NL_SET_ERR_MSG_MOD(extack, "Dualpi2 options are required");
    return -EINVAL;
    }
    err = nla_parse_nested(tb, TCA_DUALPI2_MAX, opt, dualpi2_policy,
    extack);
    if (err < 0)
    return err;
    if (tb[TCA_DUALPI2_STEP_THRESH_PKTS] && tb[TCA_DUALPI2_STEP_THRESH_US]) {
    NL_SET_ERR_MSG_MOD(extack, "multiple step thresh attributes");
    return -EINVAL;
    }
    q = qdisc_priv(sch);
    sch_tree_lock(sch);
    if (tb[TCA_DUALPI2_LIMIT]) {
    let mut limit: u32 = nla_get_u32(tb[TCA_DUALPI2_LIMIT]);
    WRITE_ONCE(sch.limit, limit);
    WRITE_ONCE(q.memory_limit, get_memory_limit(sch, limit));
    }
    if (tb[TCA_DUALPI2_MEMORY_LIMIT])
    WRITE_ONCE(q.memory_limit,
    nla_get_u32(tb[TCA_DUALPI2_MEMORY_LIMIT]));
    if (tb[TCA_DUALPI2_TARGET]) {
    let mut target: u64 = nla_get_u32(tb[TCA_DUALPI2_TARGET]);
    WRITE_ONCE(q.pi2_target, target * NSEC_PER_USEC);
    }
    if (tb[TCA_DUALPI2_TUPDATE]) {
    let mut tupdate: u64 = nla_get_u32(tb[TCA_DUALPI2_TUPDATE]);
    WRITE_ONCE(q.pi2_tupdate, convert_us_to_nsec(tupdate));
    }
    if (tb[TCA_DUALPI2_ALPHA]) {
    let mut alpha: u32 = nla_get_u32(tb[TCA_DUALPI2_ALPHA]);
    WRITE_ONCE(q.pi2_alpha, dualpi2_scale_alpha_beta(alpha));
    }
    if (tb[TCA_DUALPI2_BETA]) {
    let mut beta: u32 = nla_get_u32(tb[TCA_DUALPI2_BETA]);
    WRITE_ONCE(q.pi2_beta, dualpi2_scale_alpha_beta(beta));
    }
    if (tb[TCA_DUALPI2_STEP_THRESH_PKTS]) {
    let mut step_th: u32 = nla_get_u32(tb[TCA_DUALPI2_STEP_THRESH_PKTS]);
    WRITE_ONCE(q.step_in_packets, true);
    WRITE_ONCE(q.step_thresh, step_th);
    } else if (tb[TCA_DUALPI2_STEP_THRESH_US]) {
    let mut step_th: u32 = nla_get_u32(tb[TCA_DUALPI2_STEP_THRESH_US]);
    WRITE_ONCE(q.step_in_packets, false);
    WRITE_ONCE(q.step_thresh, convert_us_to_nsec(step_th));
    }
    if (tb[TCA_DUALPI2_MIN_QLEN_STEP])
    WRITE_ONCE(q.min_qlen_step,
    nla_get_u32(tb[TCA_DUALPI2_MIN_QLEN_STEP]));
    if (tb[TCA_DUALPI2_COUPLING]) {
    let mut coupling: u8 = nla_get_u8(tb[TCA_DUALPI2_COUPLING]);
    WRITE_ONCE(q.coupling_factor, coupling);
    }
    if (tb[TCA_DUALPI2_DROP_OVERLOAD]) {
    let mut drop_overload: u8 = nla_get_u8(tb[TCA_DUALPI2_DROP_OVERLOAD]);
    WRITE_ONCE(q.drop_overload, (bool)drop_overload);
    }
    if (tb[TCA_DUALPI2_DROP_EARLY]) {
    let mut drop_early: u8 = nla_get_u8(tb[TCA_DUALPI2_DROP_EARLY]);
    WRITE_ONCE(q.drop_early, (bool)drop_early);
    }
    if (tb[TCA_DUALPI2_C_PROTECTION]) {
    let mut wc: u8 = nla_get_u8(tb[TCA_DUALPI2_C_PROTECTION]);
    dualpi2_calculate_c_protection(sch, q, wc);
    }
    if (tb[TCA_DUALPI2_ECN_MASK]) {
    let mut ecn_mask: u8 = nla_get_u8(tb[TCA_DUALPI2_ECN_MASK]);
    WRITE_ONCE(q.ecn_mask, ecn_mask);
    }
    if (tb[TCA_DUALPI2_SPLIT_GSO]) {
    let mut split_gso: u8 = nla_get_u8(tb[TCA_DUALPI2_SPLIT_GSO]);
    WRITE_ONCE(q.split_gso, (bool)split_gso);
    }
    old_qlen = qdisc_qlen(sch);
    old_backlog = sch.qstats.backlog;
    while (qdisc_qlen(sch) > sch.limit ||
    q.memory_used > q.memory_limit) {
    struct sk_buff *skb = core::ptr::null_mut();
    if (qdisc_qlen(sch) > qdisc_qlen(q.l_queue)) {
    skb = qdisc_dequeue_internal(sch, true);
    if (unlikely(!skb)) {
    WARN_ON_ONCE(1);
    break;
    }
    WRITE_ONCE(q.memory_used, q.memory_used - skb.truesize);
    rtnl_qdisc_drop(skb, sch);
    } else if (qdisc_qlen(q.l_queue)) {
    skb = qdisc_dequeue_internal(q.l_queue, true);
    if (unlikely(!skb)) {
    WARN_ON_ONCE(1);
    break;
    }
// L-queue packets are counted in both sch and
// l_queue on enqueue; qdisc_dequeue_internal()
// handled l_queue, so we further account for sch.
//
    qdisc_qlen_dec(sch);
    qdisc_qstats_backlog_dec(sch, skb);
    WRITE_ONCE(q.memory_used, q.memory_used - skb.truesize);
    rtnl_qdisc_drop(skb, q.l_queue);
    qdisc_qstats_drop(sch);
    } else {
    WARN_ON_ONCE(1);
    break;
    }
    }
    qdisc_tree_reduce_backlog(sch, old_qlen - qdisc_qlen(sch),
    old_backlog - sch.qstats.backlog);
    sch_tree_unlock(sch);
    return 0;
    }
// Default alpha/beta values give a 10dB stability margin with max_rtt=100ms.
#[no_mangle]
unsafe extern "C" fn dualpi2_reset_default(sch: *mut Qdisc) {
    static void dualpi2_reset_default(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    q.sch.limit = 10000;				/* Max 125ms at 1Gbps */
    q.memory_limit = get_memory_limit(sch, q.sch.limit);
    q.pi2_target = 15 * NSEC_PER_MSEC;
    q.pi2_tupdate = 16 * NSEC_PER_MSEC;
    q.pi2_alpha = dualpi2_scale_alpha_beta(41);	/* ~0.16 Hz * 256 */
    q.pi2_beta = dualpi2_scale_alpha_beta(819);	/* ~3.20 Hz * 256 */
    q.step_thresh = 1 * NSEC_PER_MSEC;
    q.step_in_packets = false;
    dualpi2_calculate_c_protection(q.sch, q, 10);	/* wc=10%, wl=90% */
    q.ecn_mask = TC_DUALPI2_ECN_MASK_L4S_ECT;	/* INET_ECN_ECT_1 */
    q.min_qlen_step = 0;		/* Always apply step mark in L-queue */
    q.coupling_factor = 2;		/* window fairness for equal RTTs */
    q.drop_overload = TC_DUALPI2_DROP_OVERLOAD_DROP; /* Drop overload */
    q.drop_early = TC_DUALPI2_DROP_EARLY_DROP_DEQUEUE; /* Drop dequeue */
    q.split_gso = TC_DUALPI2_SPLIT_GSO_SPLIT_GSO;	/* Split GSO */
    }
    static int dualpi2_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    int err;
    sch.flags |= TCQ_F_DEQUEUE_DROPS;
    hrtimer_setup(&q.pi2_timer, dualpi2_timer, CLOCK_MONOTONIC,
    HRTIMER_MODE_ABS_PINNED_SOFT);
    q.l_queue = qdisc_create_dflt(sch.dev_queue, &pfifo_qdisc_ops,
    TC_H_MAKE(sch.handle, 1), extack);
    if (!q.l_queue)
    return -ENOMEM;
    err = tcf_block_get(&q.tcf_block, &q.tcf_filters, sch, extack);
    if (err)
    return err;
    q.sch = sch;
    dualpi2_reset_default(sch);
    if (opt && nla_len(opt)) {
    err = dualpi2_change(sch, opt, extack);
    if (err)
    return err;
    }
    hrtimer_start(&q.pi2_timer, next_pi2_timeout(q),
    HRTIMER_MODE_ABS_PINNED_SOFT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int dualpi2_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    struct nlattr *opts;
    bool step_in_pkts;
    u32 step_th;
    step_in_pkts = READ_ONCE(q.step_in_packets);
    step_th = READ_ONCE(q.step_thresh);
    opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (!opts)
    goto nla_put_failure;
    if (step_in_pkts &&
    (nla_put_u32(skb, TCA_DUALPI2_LIMIT, READ_ONCE(sch.limit)) ||
    nla_put_u32(skb, TCA_DUALPI2_MEMORY_LIMIT,
    READ_ONCE(q.memory_limit)) ||
    nla_put_u32(skb, TCA_DUALPI2_TARGET,
    convert_ns_to_usec(READ_ONCE(q.pi2_target))) ||
    nla_put_u32(skb, TCA_DUALPI2_TUPDATE,
    convert_ns_to_usec(READ_ONCE(q.pi2_tupdate))) ||
    nla_put_u32(skb, TCA_DUALPI2_ALPHA,
    dualpi2_unscale_alpha_beta(READ_ONCE(q.pi2_alpha))) ||
    nla_put_u32(skb, TCA_DUALPI2_BETA,
    dualpi2_unscale_alpha_beta(READ_ONCE(q.pi2_beta))) ||
    nla_put_u32(skb, TCA_DUALPI2_STEP_THRESH_PKTS, step_th) ||
    nla_put_u32(skb, TCA_DUALPI2_MIN_QLEN_STEP,
    READ_ONCE(q.min_qlen_step)) ||
    nla_put_u8(skb, TCA_DUALPI2_COUPLING,
    READ_ONCE(q.coupling_factor)) ||
    nla_put_u8(skb, TCA_DUALPI2_DROP_OVERLOAD,
    READ_ONCE(q.drop_overload)) ||
    nla_put_u8(skb, TCA_DUALPI2_DROP_EARLY,
    READ_ONCE(q.drop_early)) ||
    nla_put_u8(skb, TCA_DUALPI2_C_PROTECTION,
    READ_ONCE(q.c_protection_wc)) ||
    nla_put_u8(skb, TCA_DUALPI2_ECN_MASK, READ_ONCE(q.ecn_mask)) ||
    nla_put_u8(skb, TCA_DUALPI2_SPLIT_GSO, READ_ONCE(q.split_gso))))
    goto nla_put_failure;
    if (!step_in_pkts &&
    (nla_put_u32(skb, TCA_DUALPI2_LIMIT, READ_ONCE(sch.limit)) ||
    nla_put_u32(skb, TCA_DUALPI2_MEMORY_LIMIT,
    READ_ONCE(q.memory_limit)) ||
    nla_put_u32(skb, TCA_DUALPI2_TARGET,
    convert_ns_to_usec(READ_ONCE(q.pi2_target))) ||
    nla_put_u32(skb, TCA_DUALPI2_TUPDATE,
    convert_ns_to_usec(READ_ONCE(q.pi2_tupdate))) ||
    nla_put_u32(skb, TCA_DUALPI2_ALPHA,
    dualpi2_unscale_alpha_beta(READ_ONCE(q.pi2_alpha))) ||
    nla_put_u32(skb, TCA_DUALPI2_BETA,
    dualpi2_unscale_alpha_beta(READ_ONCE(q.pi2_beta))) ||
    nla_put_u32(skb, TCA_DUALPI2_STEP_THRESH_US,
    convert_ns_to_usec(step_th)) ||
    nla_put_u32(skb, TCA_DUALPI2_MIN_QLEN_STEP,
    READ_ONCE(q.min_qlen_step)) ||
    nla_put_u8(skb, TCA_DUALPI2_COUPLING,
    READ_ONCE(q.coupling_factor)) ||
    nla_put_u8(skb, TCA_DUALPI2_DROP_OVERLOAD,
    READ_ONCE(q.drop_overload)) ||
    nla_put_u8(skb, TCA_DUALPI2_DROP_EARLY,
    READ_ONCE(q.drop_early)) ||
    nla_put_u8(skb, TCA_DUALPI2_C_PROTECTION,
    READ_ONCE(q.c_protection_wc)) ||
    nla_put_u8(skb, TCA_DUALPI2_ECN_MASK, READ_ONCE(q.ecn_mask)) ||
    nla_put_u8(skb, TCA_DUALPI2_SPLIT_GSO, READ_ONCE(q.split_gso))))
    goto nla_put_failure;
    return nla_nest_end(skb, opts);
    nla_put_failure:
    nla_nest_cancel(skb, opts);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> c_int {
    static int dualpi2_dump_stats(struct Qdisc *sch, struct gnet_dump *d)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    struct tc_dualpi2_xstats st = {
    .prob			= READ_ONCE(q.pi2_prob),
    .packets_in_c		= READ_ONCE(q.packets_in_c),
    .packets_in_l		= READ_ONCE(q.packets_in_l),
    .maxq			= READ_ONCE(q.maxq),
    .ecn_mark		= READ_ONCE(q.ecn_mark),
    .credit			= READ_ONCE(q.c_protection_credit),
    .step_marks		= READ_ONCE(q.step_marks),
    .memory_used		= READ_ONCE(q.memory_used),
    .max_memory_used	= READ_ONCE(q.max_memory_used),
    .memory_limit		= READ_ONCE(q.memory_limit),
    };
    u64 qc, ql;
    get_queue_delays(q, &qc, &ql);
    st.delay_l = convert_ns_to_usec(ql);
    st.delay_c = convert_ns_to_usec(qc);
    return gnet_stats_copy_app(d, &st, sizeof(st));
    }
// Reset both L-queue and C-queue, internal packet counters, PI probability,
// C-queue protection credit, and timestamps, while preserving current
// configuration of DUALPI2.
//
#[no_mangle]
unsafe extern "C" fn dualpi2_reset(sch: *mut Qdisc) {
    static void dualpi2_reset(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    qdisc_reset_queue(sch);
    qdisc_reset_queue(q.l_queue);
    WRITE_ONCE(q.c_head_ts, 0);
    WRITE_ONCE(q.l_head_ts, 0);
    WRITE_ONCE(q.pi2_prob, 0);
    WRITE_ONCE(q.packets_in_c, 0);
    WRITE_ONCE(q.packets_in_l, 0);
    WRITE_ONCE(q.maxq, 0);
    WRITE_ONCE(q.ecn_mark, 0);
    WRITE_ONCE(q.step_marks, 0);
    WRITE_ONCE(q.memory_used, 0);
    WRITE_ONCE(q.max_memory_used, 0);
    dualpi2_reset_c_protection(q);
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_destroy(sch: *mut Qdisc) {
    static void dualpi2_destroy(struct Qdisc *sch)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    q.pi2_tupdate = 0;
    hrtimer_cancel(&q.pi2_timer);
    if (q.l_queue)
    qdisc_put(q.l_queue);
    tcf_block_put(q.tcf_block);
    }
    static struct Qdisc *dualpi2_leaf(struct Qdisc *sch, unsigned long arg)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long dualpi2_find(struct Qdisc *sch, u32 classid)
    {
    return 0;
    }
    static unsigned long dualpi2_bind(struct Qdisc *sch, unsigned long parent,
    u32 classid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_unbind(q: *mut Qdisc, cl: c_ulong) {
    static void dualpi2_unbind(struct Qdisc *q, unsigned long cl)
    {
    }
    static struct tcf_block *dualpi2_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct dualpi2_sched_data *q = qdisc_priv(sch);
    if (cl)
    return core::ptr::null_mut();
    return q.tcf_block;
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    static void dualpi2_walk(struct Qdisc *sch, struct qdisc_walker *arg)
    {
    unsigned int i;
    if (arg.stop)
    return;
// We statically define only 2 queues
    for (i = 0; i < 2; i++) {
    if (arg.count < arg.skip) {
    arg.count++;
    continue;
    }
    if (arg.fn(sch, i + 1, arg) < 0) {
    arg.stop = 1;
    break;
    }
    arg.count++;
    }
    }
// Minimal class support to handle tc filters
    static const struct Qdisc_class_ops dualpi2_class_ops = {
    .leaf		= dualpi2_leaf,
    .find		= dualpi2_find,
    .tcf_block	= dualpi2_tcf_block,
    .bind_tcf	= dualpi2_bind,
    .unbind_tcf	= dualpi2_unbind,
    .walk		= dualpi2_walk,
    };
    static struct Qdisc_ops dualpi2_qdisc_ops __read_mostly = {
    .id		= "dualpi2",
    .cl_ops		= &dualpi2_class_ops,
    .priv_size	= sizeof(struct dualpi2_sched_data),
    .enqueue	= dualpi2_qdisc_enqueue,
    .dequeue	= dualpi2_qdisc_dequeue,
    .peek		= dualpi2_peek,
    .init		= dualpi2_init,
    .destroy	= dualpi2_destroy,
    .reset		= dualpi2_reset,
    .change		= dualpi2_change,
    .dump		= dualpi2_dump,
    .dump_stats	= dualpi2_dump_stats,
    .owner		= THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("dualpi2");
#[no_mangle]
unsafe extern "C" fn dualpi2_module_init() -> int __init {
    static int __init dualpi2_module_init(void)
    {
    return register_qdisc(&dualpi2_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn dualpi2_module_exit() -> void __exit {
    static void __exit dualpi2_module_exit(void)
    {
    unregister_qdisc(&dualpi2_qdisc_ops);
    }
    module_init(dualpi2_module_init);
    module_exit(dualpi2_module_exit);
    MODULE_DESCRIPTION("Dual Queue with Proportional Integral controller Improved with a Square (dualpi2) scheduler");
    MODULE_AUTHOR("Koen De Schepper <koen.de_schepper@nokia-bell-labs.com>");
    MODULE_AUTHOR("Chia-Yu Chang <chia-yu.chang@nokia-bell-labs.com>");
    MODULE_AUTHOR("Olga Albisser <olga@albisser.org>");
    MODULE_AUTHOR("Henrik Steen <henrist@henrist.net>");
    MODULE_AUTHOR("Olivier Tilmans <olivier.tilmans@nokia.com>");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_VERSION("1.0");
