//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_sfb.c
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
// net/sched/sch_sfb.c	  Stochastic Fair Blue
//
// Copyright (c) 2008-2011 Juliusz Chroboczek <jch@pps.jussieu.fr>
// Copyright (c) 2011 Eric Dumazet <eric.dumazet@gmail.com>
//
// W. Feng, D. Kandlur, D. Saha, K. Shin. Blue:
// A New Class of Active Queue Management Algorithms.
// U. Michigan CSE-TR-387-99, April 1999.
//
// http://www.thefengs.com/wuchang/blue/CSE-TR-387-99.pdf
//

//
// SFB uses two B[l][n] : L x N arrays of bins (L levels, N bins per level)
// This implementation uses L = 8 and N = 16
// This permits us to split one 32bit hash (provided per packet by rxhash or
// external classifier) into 8 subhashes of 4 bits.
//
pub const SFB_BUCKET_SHIFT: c_int = 4;

// SFB algo uses a virtual queue, named "bin"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfb_bucket {
    pub /: *mut *mut u16 qlen; / length of virtual queue,
    pub /: *mut *mut u16 p_mark; / marking probability,
}

// We use a double buffering right before hash change
// (Section 4.4 of SFB reference : moving hash functions)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfb_bins {
    pub /: *mut *mut siphash_key_t perturbation; / siphash key,
    pub bins: [sfb_bucket; SFB_LEVELS][SFB_NUMBUCKETS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfb_sched_data {
    pub qdisc: *mut Qdisc,
    pub filter_list: *mut tcf_proto __rcu,
    pub block: *mut tcf_block,
    pub rehash_interval: c_ulong,
    pub /: *mut *mut unsigned long warmup_time; / double buffering warmup time in jiffies,
    pub max: u32,
    pub /: *mut *mut u32 bin_size; / maximum queue length per bin,
    pub /: *mut *mut u32 increment; / d1,
    pub /: *mut *mut u32 decrement; / d2,
    pub /: *mut *mut u32 limit; / HARD maximal queue length,
    pub penalty_rate: u32,
    pub penalty_burst: u32,
    pub tokens_avail: u32,
    pub rehash_time: c_ulong,
    pub token_time: c_ulong,
    pub /: *mut *mut u8 slot; / current active bins (0 or 1),
    pub double_buffering: bool,
    pub bins: [sfb_bins; 2],
    struct {
    pub earlydrop: u32,
    pub penaltydrop: u32,
    pub bucketdrop: u32,
    pub queuedrop: u32,
    pub /: *mut *mut u32 childdrop; / drops in child qdisc,
    pub /: *mut *mut u32 marked; / ECN mark,
    pub stats: },
}

//
// Each queued skb might be hashed on one or two bins
// We store in skb_cb the two hash values.
// (A zero value means double buffering was not used)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfb_skb_cb {
    pub hashes: [u32; 2],
}

    static inline struct sfb_skb_cb *sfb_skb_cb(const struct sk_buff *skb)
    {
    qdisc_cb_private_validate(skb, sizeof(struct sfb_skb_cb));
    return (struct sfb_skb_cb *)qdisc_skb_cb(skb).data;
    }
//
// If using 'internal' SFB flow classifier, hash comes from skb rxhash
// If using external classifier, hash comes from the classid.
//
#[no_mangle]
unsafe extern "C" fn sfb_hash(skb: *const sk_buff, slot: u32) -> u32 {
    static u32 sfb_hash(const struct sk_buff *skb, u32 slot)
    {
    return sfb_skb_cb(skb).hashes[slot];
    }
// Probabilities are coded as Q0.16 fixed-point values,
// with 0xFFFF representing 65535/65536 (almost 1.0)
// Addition and subtraction are saturating in [0, 65535]
//
#[no_mangle]
unsafe extern "C" fn prob_plus(p1: u32, p2: u32) -> u32 {
    static u32 prob_plus(u32 p1, u32 p2)
    {
    let mut res: u32 = p1 + p2;
    return min_t(u32, res, SFB_MAX_PROB);
    }
#[no_mangle]
unsafe extern "C" fn prob_minus(p1: u32, p2: u32) -> u32 {
    static u32 prob_minus(u32 p1, u32 p2)
    {
    return p1 > p2 ? p1 - p2 : 0;
    }
#[no_mangle]
unsafe extern "C" fn increment_one_qlen(sfbhash: u32, slot: u32, q: *mut sfb_sched_data) {
    static void increment_one_qlen(u32 sfbhash, u32 slot, struct sfb_sched_data *q)
    {
    int i;
    struct sfb_bucket *b = &q.bins[slot].bins[0][0];
    for (i = 0; i < SFB_LEVELS; i++) {
    let mut hash: u32 = sfbhash & SFB_BUCKET_MASK;
    sfbhash >>= SFB_BUCKET_SHIFT;
    if (b[hash].qlen < 0xFFFF)
    WRITE_ONCE(b[hash].qlen, b[hash].qlen + 1);
    b += SFB_NUMBUCKETS; /* next level */
    }
    }
#[no_mangle]
unsafe extern "C" fn increment_qlen(cb: *const sfb_skb_cb, q: *mut sfb_sched_data) {
    static void increment_qlen(const struct sfb_skb_cb *cb, struct sfb_sched_data *q)
    {
    u32 sfbhash;
    sfbhash = cb.hashes[0];
    if (sfbhash)
    increment_one_qlen(sfbhash, 0, q);
    sfbhash = cb.hashes[1];
    if (sfbhash)
    increment_one_qlen(sfbhash, 1, q);
    }
    static void decrement_one_qlen(u32 sfbhash, u32 slot,
    struct sfb_sched_data *q)
    {
    int i;
    struct sfb_bucket *b = &q.bins[slot].bins[0][0];
    for (i = 0; i < SFB_LEVELS; i++) {
    let mut hash: u32 = sfbhash & SFB_BUCKET_MASK;
    sfbhash >>= SFB_BUCKET_SHIFT;
    if (b[hash].qlen > 0)
    WRITE_ONCE(b[hash].qlen, b[hash].qlen - 1);
    b += SFB_NUMBUCKETS; /* next level */
    }
    }
#[no_mangle]
unsafe extern "C" fn decrement_qlen(skb: *const sk_buff, q: *mut sfb_sched_data) {
    static void decrement_qlen(const struct sk_buff *skb, struct sfb_sched_data *q)
    {
    u32 sfbhash;
    sfbhash = sfb_hash(skb, 0);
    if (sfbhash)
    decrement_one_qlen(sfbhash, 0, q);
    sfbhash = sfb_hash(skb, 1);
    if (sfbhash)
    decrement_one_qlen(sfbhash, 1, q);
    }
#[no_mangle]
unsafe extern "C" fn decrement_prob(b: *mut sfb_bucket, q: *mut sfb_sched_data) {
    static void decrement_prob(struct sfb_bucket *b, struct sfb_sched_data *q)
    {
    WRITE_ONCE(b.p_mark, prob_minus(b.p_mark, q.decrement));
    }
#[no_mangle]
unsafe extern "C" fn increment_prob(b: *mut sfb_bucket, q: *mut sfb_sched_data) {
    static void increment_prob(struct sfb_bucket *b, struct sfb_sched_data *q)
    {
    WRITE_ONCE(b.p_mark, prob_plus(b.p_mark, q.increment));
    }
#[no_mangle]
unsafe extern "C" fn sfb_zero_all_buckets(q: *mut sfb_sched_data) {
    static void sfb_zero_all_buckets(struct sfb_sched_data *q)
    {
    memset(&q.bins, 0, sizeof(q.bins));
    }
//
// compute max qlen, max p_mark, and avg p_mark
//
#[no_mangle]
unsafe extern "C" fn sfb_compute_qlen(prob_r: *mut u32, avgpm_r: *mut u32, q: *const sfb_sched_data) -> u32 {
    static u32 sfb_compute_qlen(u32 *prob_r, u32 *avgpm_r, const struct sfb_sched_data *q)
    {
    int i;
    let mut qlen: u32 = 0, prob = 0, totalpm = 0;
    const struct sfb_bucket *b = &q.bins[q.slot].bins[0][0];
    for (i = 0; i < SFB_LEVELS * SFB_NUMBUCKETS; i++) {
    let mut b_qlen: u32 = READ_ONCE(b.qlen);
    let mut b_mark: u32 = READ_ONCE(b.p_mark);
    if (qlen < b_qlen)
    qlen = b_qlen;
    totalpm += b_mark;
    if (prob < b_mark)
    prob = b_mark;
    b++;
    }
// prob_r = prob;
// avgpm_r = totalpm / (SFB_LEVELS * SFB_NUMBUCKETS);
    return qlen;
    }
#[no_mangle]
unsafe extern "C" fn sfb_init_perturbation(slot: u32, q: *mut sfb_sched_data) {
    static void sfb_init_perturbation(u32 slot, struct sfb_sched_data *q)
    {
    get_random_bytes(&q.bins[slot].perturbation,
    sizeof(q.bins[slot].perturbation));
    }
#[no_mangle]
unsafe extern "C" fn sfb_swap_slot(q: *mut sfb_sched_data) {
    static void sfb_swap_slot(struct sfb_sched_data *q)
    {
    sfb_init_perturbation(q.slot, q);
    q.slot ^= 1;
    q.double_buffering = false;
    }
// Non elastic flows are allowed to use part of the bandwidth, expressed
// in "penalty_rate" packets per second, with "penalty_burst" burst
//
#[no_mangle]
unsafe extern "C" fn sfb_rate_limit(skb: *mut sk_buff, q: *mut sfb_sched_data) -> bool {
    static bool sfb_rate_limit(struct sk_buff *skb, struct sfb_sched_data *q)
    {
    if (q.penalty_rate == 0 || q.penalty_burst == 0)
    return true;
    if (q.tokens_avail < 1) {
    let mut age: c_ulong = min(10UL * HZ, jiffies - q.token_time);
    q.tokens_avail = (age * q.penalty_rate) / HZ;
    if (q.tokens_avail > q.penalty_burst)
    q.tokens_avail = q.penalty_burst;
    q.token_time = jiffies;
    if (q.tokens_avail < 1)
    return true;
    }
    q.tokens_avail--;
    return false;
    }
    static bool sfb_classify(struct sk_buff *skb, struct tcf_proto *fl,
    int *qerr, u32 *salt)
    {
    struct tcf_result res;
    int result;
    result = tcf_classify_qdisc(skb, fl, &res, false);
    if (result >= 0) {

    switch (result) {
    case TC_ACT_STOLEN:
    case TC_ACT_QUEUED:
    case TC_ACT_TRAP:
// qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
    fallthrough;
    case TC_ACT_SHOT:
    return false;
    }

// salt = TC_H_MIN(res.classid);
    return true;
    }
    return false;
    }
    static int sfb_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    let mut reason: enum qdisc_drop_reason = QDISC_DROP_OVERLIMIT;
    struct sfb_sched_data *q = qdisc_priv(sch);
    let mut len: c_uint = qdisc_pkt_len(skb);
    struct Qdisc *child = q.qdisc;
    struct tcf_proto *fl;
    struct sfb_skb_cb cb;
    int i;
    let mut p_min: u32 = ~0;
    let mut minqlen: u32 = ~0;
    u32 r, sfbhash;
    let mut slot: u32 = q.slot;
    let mut ret: c_int = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    if (unlikely(sch.q.qlen >= q.limit)) {
    qdisc_qstats_overlimit(sch);
    WRITE_ONCE(q.stats.queuedrop,
    q.stats.queuedrop + 1);
    goto drop;
    }
    if (q.rehash_interval > 0) {
    let mut limit: c_ulong = q.rehash_time + q.rehash_interval;
    if (unlikely(time_after(jiffies, limit))) {
    sfb_swap_slot(q);
    q.rehash_time = jiffies;
    } else if (unlikely(!q.double_buffering && q.warmup_time > 0 &&
    time_after(jiffies, limit - q.warmup_time))) {
    q.double_buffering = true;
    }
    }
    fl = rcu_dereference_bh(q.filter_list);
    if (fl) {
    u32 salt;
// If using external classifiers, get result and record it.
    if (!sfb_classify(skb, fl, &ret, &salt))
    goto other_drop;
    sfbhash = siphash_1u32(salt, &q.bins[slot].perturbation);
    } else {
    sfbhash = skb_get_hash_perturb(skb, &q.bins[slot].perturbation);
    }
    if (!sfbhash)
    sfbhash = 1;
    sfb_skb_cb(skb).hashes[slot] = sfbhash;
    for (i = 0; i < SFB_LEVELS; i++) {
    let mut hash: u32 = sfbhash & SFB_BUCKET_MASK;
    struct sfb_bucket *b = &q.bins[slot].bins[i][hash];
    sfbhash >>= SFB_BUCKET_SHIFT;
    if (b.qlen == 0)
    decrement_prob(b, q);
#[no_mangle]
pub unsafe extern "C" fn if(q->bin_size: b->qlen >=) -> else {
    else if (b.qlen >= q.bin_size)
    increment_prob(b, q);
    if (minqlen > b.qlen)
    minqlen = b.qlen;
    if (p_min > b.p_mark)
    p_min = b.p_mark;
    }
    slot ^= 1;
    sfb_skb_cb(skb).hashes[slot] = 0;
    if (unlikely(minqlen >= q.max)) {
    qdisc_qstats_overlimit(sch);
    WRITE_ONCE(q.stats.bucketdrop,
    q.stats.bucketdrop + 1);
    goto drop;
    }
    if (unlikely(p_min >= SFB_MAX_PROB)) {
// Inelastic flow
    if (q.double_buffering) {
    sfbhash = skb_get_hash_perturb(skb,
    &q.bins[slot].perturbation);
    if (!sfbhash)
    sfbhash = 1;
    sfb_skb_cb(skb).hashes[slot] = sfbhash;
    for (i = 0; i < SFB_LEVELS; i++) {
    let mut hash: u32 = sfbhash & SFB_BUCKET_MASK;
    struct sfb_bucket *b = &q.bins[slot].bins[i][hash];
    sfbhash >>= SFB_BUCKET_SHIFT;
    if (b.qlen == 0)
    decrement_prob(b, q);
#[no_mangle]
pub unsafe extern "C" fn if(q->bin_size: b->qlen >=) -> else {
    else if (b.qlen >= q.bin_size)
    increment_prob(b, q);
    }
    }
    if (sfb_rate_limit(skb, q)) {
    qdisc_qstats_overlimit(sch);
    WRITE_ONCE(q.stats.penaltydrop,
    q.stats.penaltydrop + 1);
    goto drop;
    }
    goto enqueue;
    }
    r = get_random_u16() & SFB_MAX_PROB;
    reason = QDISC_DROP_CONGESTED;
    if (unlikely(r < p_min)) {
    if (unlikely(p_min > SFB_MAX_PROB / 2)) {
// If we're marking that many packets, then either
// this flow is unresponsive, or we're badly congested.
// In either case, we want to start dropping packets.
//
    if (r < (p_min - SFB_MAX_PROB / 2) * 2) {
    WRITE_ONCE(q.stats.earlydrop,
    q.stats.earlydrop + 1);
    goto drop;
    }
    }
    if (INET_ECN_set_ce(skb)) {
    WRITE_ONCE(q.stats.marked,
    q.stats.marked + 1);
    } else {
    WRITE_ONCE(q.stats.earlydrop,
    q.stats.earlydrop + 1);
    goto drop;
    }
    }
    enqueue:
    memcpy(&cb, sfb_skb_cb(skb), sizeof(cb));
    ret = qdisc_enqueue(skb, child, to_free);
    if (likely(ret == NET_XMIT_SUCCESS)) {
    qstats_backlog_add(sch, len);
    qdisc_qlen_inc(sch);
    increment_qlen(&cb, q);
    } else if (net_xmit_drop_count(ret)) {
    WRITE_ONCE(q.stats.childdrop,
    q.stats.childdrop + 1);
    qdisc_qstats_drop(sch);
    }
    return ret;
    drop:
    qdisc_drop_reason(skb, sch, to_free, reason);
    return NET_XMIT_CN;
    other_drop:
    if (ret & __NET_XMIT_BYPASS)
    qdisc_qstats_drop(sch);
    kfree_skb(skb);
    return ret;
    }
    static struct sk_buff *sfb_dequeue(struct Qdisc *sch)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child = q.qdisc;
    struct sk_buff *skb;
    skb = qdisc_dequeue_peeked(child);
    if (skb) {
    qdisc_bstats_update(sch, skb);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_qlen_dec(sch);
    decrement_qlen(skb, q);
    }
    return skb;
    }
    static struct sk_buff *sfb_peek(struct Qdisc *sch)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child = q.qdisc;
    return child.ops.peek(child);
    }
// No sfb_drop -- impossible since the child doesn't return the dropped skb.
#[no_mangle]
unsafe extern "C" fn sfb_reset(sch: *mut Qdisc) {
    static void sfb_reset(struct Qdisc *sch)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    if (likely(q.qdisc))
    qdisc_reset(q.qdisc);
    q.slot = 0;
    q.double_buffering = false;
    sfb_zero_all_buckets(q);
    sfb_init_perturbation(0, q);
    }
#[no_mangle]
unsafe extern "C" fn sfb_destroy(sch: *mut Qdisc) {
    static void sfb_destroy(struct Qdisc *sch)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    tcf_block_put(q.block);
    qdisc_put(q.qdisc);
    }
    static const struct nla_policy sfb_policy[TCA_SFB_MAX + 1] = {
    [TCA_SFB_PARMS]	= { .len = sizeof(struct tc_sfb_qopt) },
    };
    static const struct tc_sfb_qopt sfb_default_ops = {
    .rehash_interval = 600 * MSEC_PER_SEC,
    .warmup_time = 60 * MSEC_PER_SEC,
    .limit = 0,
    .max = 25,
    .bin_size = 20,
    .increment = (SFB_MAX_PROB + 500) / 1000, /* 0.1 % */
    .decrement = (SFB_MAX_PROB + 3000) / 6000,
    .penalty_rate = 10,
    .penalty_burst = 20,
    };
    static int sfb_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    struct Qdisc *child, *old;
    struct nlattr *tb[TCA_SFB_MAX + 1];
    const struct tc_sfb_qopt *ctl = &sfb_default_ops;
    u32 limit;
    int err;
    if (opt) {
    err = nla_parse_nested_deprecated(tb, TCA_SFB_MAX, opt,
    sfb_policy, core::ptr::null_mut());
    if (err < 0)
    return -EINVAL;
    if (tb[TCA_SFB_PARMS] == core::ptr::null_mut())
    return -EINVAL;
    ctl = nla_data(tb[TCA_SFB_PARMS]);
    }
    limit = ctl.limit;
    if (limit == 0)
    limit = qdisc_dev(sch).tx_queue_len;
    child = fifo_create_dflt(sch, &pfifo_qdisc_ops, limit, extack);
    if (IS_ERR(child))
    return PTR_ERR(child);
    if (child != &noop_qdisc)
    qdisc_hash_add(child, true);
    sch_tree_lock(sch);
    qdisc_purge_queue(q.qdisc);
    old = q.qdisc;
    q.qdisc = child;
    q.rehash_interval = msecs_to_jiffies(ctl.rehash_interval);
    q.warmup_time = msecs_to_jiffies(ctl.warmup_time);
    q.rehash_time = jiffies;
    q.limit = limit;
    q.increment = ctl.increment;
    q.decrement = ctl.decrement;
    q.max = ctl.max;
    q.bin_size = ctl.bin_size;
    q.penalty_rate = ctl.penalty_rate;
    q.penalty_burst = ctl.penalty_burst;
    q.tokens_avail = ctl.penalty_burst;
    q.token_time = jiffies;
    q.slot = 0;
    q.double_buffering = false;
    sfb_zero_all_buckets(q);
    sfb_init_perturbation(0, q);
    sfb_init_perturbation(1, q);
    sch_tree_unlock(sch);
    qdisc_put(old);
    return 0;
    }
    static int sfb_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    int err;
    err = tcf_block_get(&q.block, &q.filter_list, sch, extack);
    if (err)
    return err;
    q.qdisc = &noop_qdisc;
    return sfb_change(sch, opt, extack);
    }
#[no_mangle]
unsafe extern "C" fn sfb_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int sfb_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    struct nlattr *opts;
    struct tc_sfb_qopt opt = {
    .rehash_interval = jiffies_to_msecs(q.rehash_interval),
    .warmup_time = jiffies_to_msecs(q.warmup_time),
    .limit = q.limit,
    .max = q.max,
    .bin_size = q.bin_size,
    .increment = q.increment,
    .decrement = q.decrement,
    .penalty_rate = q.penalty_rate,
    .penalty_burst = q.penalty_burst,
    };
    WRITE_ONCE(sch.qstats.backlog, READ_ONCE(q.qdisc.qstats.backlog));
    opts = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (opts == core::ptr::null_mut())
    goto nla_put_failure;
    if (nla_put(skb, TCA_SFB_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;
    return nla_nest_end(skb, opts);
    nla_put_failure:
    nla_nest_cancel(skb, opts);
    return -EMSGSIZE;
    }
#[no_mangle]
unsafe extern "C" fn sfb_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> c_int {
    static int sfb_dump_stats(struct Qdisc *sch, struct gnet_dump *d)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    struct tc_sfb_xstats st = {
    .earlydrop = READ_ONCE(q.stats.earlydrop),
    .penaltydrop = READ_ONCE(q.stats.penaltydrop),
    .bucketdrop = READ_ONCE(q.stats.bucketdrop),
    .queuedrop = READ_ONCE(q.stats.queuedrop),
    .childdrop = READ_ONCE(q.stats.childdrop),
    .marked = READ_ONCE(q.stats.marked),
    };
    st.maxqlen = sfb_compute_qlen(&st.maxprob, &st.avgprob, q);
    return gnet_stats_copy_app(d, &st, sizeof(st));
    }
    static int sfb_dump_class(struct Qdisc *sch, unsigned long cl,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    return -ENOSYS;
    }
    static int sfb_graft(struct Qdisc *sch, unsigned long arg, struct Qdisc *new,
    struct Qdisc **old, struct netlink_ext_ack *extack)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    if (new == core::ptr::null_mut())
    new = &noop_qdisc;
// old = qdisc_replace(sch, new, &q->qdisc);
    return 0;
    }
    static struct Qdisc *sfb_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    return q.qdisc;
    }
#[no_mangle]
unsafe extern "C" fn sfb_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long sfb_find(struct Qdisc *sch, u32 classid)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn sfb_unbind(sch: *mut Qdisc, arg: c_ulong) {
    static void sfb_unbind(struct Qdisc *sch, unsigned long arg)
    {
    }
    static int sfb_change_class(struct Qdisc *sch, u32 classid, u32 parentid,
    struct nlattr **tca, unsigned long *arg,
    struct netlink_ext_ack *extack)
    {
    return -ENOSYS;
    }
    static int sfb_delete(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn sfb_walk(sch: *mut Qdisc, walker: *mut qdisc_walker) {
    static void sfb_walk(struct Qdisc *sch, struct qdisc_walker *walker)
    {
    if (!walker.stop) {
    tc_qdisc_stats_dump(sch, 1, walker);
    }
    }
    static struct tcf_block *sfb_tcf_block(struct Qdisc *sch, unsigned long cl,
    struct netlink_ext_ack *extack)
    {
    struct sfb_sched_data *q = qdisc_priv(sch);
    if (cl)
    return core::ptr::null_mut();
    return q.block;
    }
    static unsigned long sfb_bind(struct Qdisc *sch, unsigned long parent,
    u32 classid)
    {
    return 0;
    }
    static const struct Qdisc_class_ops sfb_class_ops = {
    .graft		=	sfb_graft,
    .leaf		=	sfb_leaf,
    .find		=	sfb_find,
    .change		=	sfb_change_class,
    .delete		=	sfb_delete,
    .walk		=	sfb_walk,
    .tcf_block	=	sfb_tcf_block,
    .bind_tcf	=	sfb_bind,
    .unbind_tcf	=	sfb_unbind,
    .dump		=	sfb_dump_class,
    };
    static struct Qdisc_ops sfb_qdisc_ops __read_mostly = {
    .id		=	"sfb",
    .priv_size	=	sizeof(struct sfb_sched_data),
    .cl_ops		=	&sfb_class_ops,
    .enqueue	=	sfb_enqueue,
    .dequeue	=	sfb_dequeue,
    .peek		=	sfb_peek,
    .init		=	sfb_init,
    .reset		=	sfb_reset,
    .destroy	=	sfb_destroy,
    .change		=	sfb_change,
    .dump		=	sfb_dump,
    .dump_stats	=	sfb_dump_stats,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("sfb");
#[no_mangle]
unsafe extern "C" fn sfb_module_init() -> int __init {
    static int __init sfb_module_init(void)
    {
    return register_qdisc(&sfb_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn sfb_module_exit() -> void __exit {
    static void __exit sfb_module_exit(void)
    {
    unregister_qdisc(&sfb_qdisc_ops);
    }
    module_init(sfb_module_init)
    module_exit(sfb_module_exit)
    MODULE_DESCRIPTION("Stochastic Fair Blue queue discipline");
    MODULE_AUTHOR("Juliusz Chroboczek");
    MODULE_AUTHOR("Eric Dumazet");
    MODULE_LICENSE("GPL");
