//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_netem.c
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
// net/sched/sch_netem.c	Network emulator
//
// Many of the algorithms and ideas for this came from
// NIST Net which is not copyrighted.
//
// Authors:	Stephen Hemminger <shemminger@osdl.org>
// Catalin(ux aka Dino) BOIE <catab at umbrella dot ro>
//

// Network Emulation Queuing algorithm.
    ====================================
    Sources: [1] Mark Carson, Darrin Santay, "NIST Net - A Linux-based
    Network Emulation Tool
    [2] Luigi Rizzo, DummyNet for FreeBSD
    ----------------------------------------------------------------
    This started out as a simple way to delay outgoing packets to
    test TCP but has grown to include most of the functionality
    of a full blown network emulator like NISTnet. It can delay
    packets and add random jitter (and correlation). The random
    distribution can be loaded from a table as well to provide
    normal, Pareto, or experimental curves. Packet loss,
    duplication, and reordering can also be emulated.
    This qdisc does not do classification that can be handled in
    layering other disciplines.  It does not need to do bandwidth
    control either since that can be handled by using token
    bucket or other rate control.
    Correlated Loss Generator models
    Added generation of correlated loss according to the
    "Gilbert-Elliot" model, a 4-state markov model.
    References:
    [1] NetemCLG Home http://netgroup.uniroma2.it/NetemCLG
    [2] S. Salsano, F. Ludovici, A. Ordine, "Definition of a general
    and intuitive loss model for packet networks and its implementation
    in the Netem module in the Linux kernel", available in [1]
    Authors: Stefano Salsano <stefano.salsano at uniroma2.it
    Fabio Ludovici <fabio.ludovici at yahoo.it>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disttable {
    pub size: u32,
    pub __counted_by(size): s16 table[],
}

// Loss models
    enum {
    CLG_RANDOM,
    CLG_4_STATES,
    CLG_GILB_ELL,
    };
// States in GE model
    enum {
    GOOD_STATE = 1,
    BAD_STATE,
    };
// States in 4 state model
    enum {
    TX_IN_GAP_PERIOD = 1,
    TX_IN_BURST_PERIOD,
    LOST_IN_GAP_PERIOD,
    LOST_IN_BURST_PERIOD,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netem_sched_data {
// Cacheline 0: tfifo state and per-packet enqueue/dequeue scalars.
    pub t_root: rb_root,
    pub t_head: *mut sk_buff,
    pub t_tail: *mut sk_buff,
    pub t_len: u32,
    pub counter: u32,
    pub latency: i64,
    pub jitter: i64,
    pub rate: u64,
    pub gap: u32,
    pub loss: u32,
// Cacheline 1: zero-check scalars and correlation states.
    pub duplicate: u32,
    pub reorder: u32,
    pub corrupt: u32,
    pub ecn: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crndstate {
    pub last: u32,
    pub rho: u32,
    pub corrupt_cor: } delay_cor, loss_cor, dup_cor, reorder_cor,,
    pub loss_model: u8,
// Cacheline 2: PRNG, distribution tables, slot dequeue state etc.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prng {
    pub seed: u64,
    pub prng_state: rnd_state,
    pub prng: },
    pub delay_dist: *mut disttable,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slotstate {
    pub slot_next: u64,
    pub packets_left: i32,
    pub bytes_left: i32,
    pub slot: },
    pub slot_dist: *mut disttable,
    pub qdisc: *mut Qdisc,
//
// Warm: rate-shaping parameters (only read when rate != 0) and
// configuration-only fields.  The fast path reads sch->limit, not
// q->limit.
//
    pub packet_overhead: i32,
    pub cell_size: u32,
    pub cell_size_reciprocal: reciprocal_value,
    pub cell_overhead: i32,
    pub limit: u32,
// Correlated Loss Generation models
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clgstate {
// 4-states and Gilbert-Elliot models
    pub /: *mut *mut u32 a1; / p13 for 4-states or p for GE,
    pub /: *mut *mut u32 a2; / p31 for 4-states or r for GE,
    pub /: *mut *mut u32 a3; / p32 for 4-states or h for GE,
    pub /: *mut *mut u32 a4; / p14 for 4-states or 1-k for GE,
    pub /: *mut *mut u32 a5; / p23 used only in 4-states,
// state of the Markov chain
    pub state: u8,
    pub clg: },
// Impairment counters
    pub delayed: u64,
    pub dropped: u64,
    pub corrupted: u64,
    pub duplicated: u64,
    pub ecn_marked: u64,
    pub reordered: u64,
    pub allocation_errors: u64,
// Cold tail: slot reschedule config and the watchdog timer.
    pub slot_config: tc_netem_slot,
    pub watchdog: qdisc_watchdog,
}

// Time stamp put into socket buffer control block
// Only valid when skbs are in our internal t(ime)fifo queue.
//
// As skb->rbnode uses same storage than skb->next, skb->prev and skb->tstamp,
// and skb->next & skb->prev are scratch space for a qdisc,
// we save skb->tstamp value in skb->cb[] before destroying it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netem_skb_cb {
    pub time_to_send: u64,
}

    static inline struct netem_skb_cb *netem_skb_cb(struct sk_buff *skb)
    {
// we assume we can use skb next/prev/tstamp as storage for rb_node
    qdisc_cb_private_validate(skb, sizeof(struct netem_skb_cb));
    return (struct netem_skb_cb *)qdisc_skb_cb(skb).data;
    }
// init_crandom - initialize correlated random number generator
// Use entropy source for initial seed.
//
#[no_mangle]
unsafe extern "C" fn init_crandom(state: *mut crndstate, rho: c_ulong) {
    static void init_crandom(struct crndstate *state, unsigned long rho)
    {
    state.rho = rho;
    state.last = get_random_u32();
    }
// get_crandom - correlated random number generator
// Next number depends on last value.
// rho is scaled to avoid floating point.
//
#[no_mangle]
unsafe extern "C" fn get_crandom(state: *mut crndstate, p: *mut prng) -> u32 {
    static u32 get_crandom(struct crndstate *state, struct prng *p)
    {
    u64 value, rho;
    unsigned long answer;
    struct rnd_state *s = &p.prng_state;
    if (!state || state.rho == 0)	/* no correlation */
    return prandom_u32_state(s);
    value = prandom_u32_state(s);
    rho = (u64)state.rho + 1;
    answer = (value * ((1ull<<32) - rho) + state.last * rho) >> 32;
    state.last = answer;
    return answer;
    }
// loss_4state - 4-state model loss generator
// Generates losses according to the 4-state Markov chain adopted in
// the GI (General and Intuitive) loss model.
//
#[no_mangle]
unsafe extern "C" fn loss_4state(q: *mut netem_sched_data) -> bool {
    static bool loss_4state(struct netem_sched_data *q)
    {
    struct clgstate *clg = &q.clg;
    let mut rnd: u32 = prandom_u32_state(&q.prng.prng_state);
//
// Makes a comparison between rnd and the transition
// probabilities outgoing from the current state, then decides the
// next state and if the next packet has to be transmitted or lost.
// The four states correspond to:
// TX_IN_GAP_PERIOD => successfully transmitted packets within a gap period
// LOST_IN_GAP_PERIOD => isolated losses within a gap period
// LOST_IN_BURST_PERIOD => lost packets within a burst period
// TX_IN_BURST_PERIOD => successfully transmitted packets within a burst period
//
    switch (clg.state) {
    case TX_IN_GAP_PERIOD:
    if (rnd < clg.a4) {
    clg.state = LOST_IN_GAP_PERIOD;
    return true;
    } else if (rnd < clg.a1 + clg.a4) {
    clg.state = LOST_IN_BURST_PERIOD;
    return true;
    } else {
    clg.state = TX_IN_GAP_PERIOD;
    }
    break;
    case TX_IN_BURST_PERIOD:
    if (rnd < clg.a5) {
    clg.state = LOST_IN_BURST_PERIOD;
    return true;
    } else {
    clg.state = TX_IN_BURST_PERIOD;
    }
    break;
    case LOST_IN_BURST_PERIOD:
    if (rnd < clg.a3)
    clg.state = TX_IN_BURST_PERIOD;
#[no_mangle]
pub unsafe extern "C" fn if(clg->a3: rnd < clg->a2 +) -> else {
    clg.state = TX_IN_GAP_PERIOD;
    } else {
    clg.state = LOST_IN_BURST_PERIOD;
    return true;
    }
    break;
    case LOST_IN_GAP_PERIOD:
    clg.state = TX_IN_GAP_PERIOD;
    break;
    }
    return false;
    }
// loss_gilb_ell - Gilbert-Elliot model loss generator
// Generates losses according to the Gilbert-Elliot loss model or
// its special cases  (Gilbert or Simple Gilbert)
//
// Makes a comparison between random number and the transition
// probabilities outgoing from the current state, then decides the
// next state. A second random number is extracted and the comparison
// with the loss probability of the current state decides if the next
// packet will be transmitted or lost.
//
#[no_mangle]
unsafe extern "C" fn loss_gilb_ell(q: *mut netem_sched_data) -> bool {
    static bool loss_gilb_ell(struct netem_sched_data *q)
    {
    struct clgstate *clg = &q.clg;
    struct rnd_state *s = &q.prng.prng_state;
    switch (clg.state) {
    case GOOD_STATE:
    if (prandom_u32_state(s) < clg.a1)
    clg.state = BAD_STATE;
    if (prandom_u32_state(s) < clg.a4)
    return true;
    break;
    case BAD_STATE:
    if (prandom_u32_state(s) < clg.a2)
    clg.state = GOOD_STATE;
    if (prandom_u32_state(s) > clg.a3)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn loss_event(q: *mut netem_sched_data) -> bool {
    static bool loss_event(struct netem_sched_data *q)
    {
    switch (q.loss_model) {
    case CLG_RANDOM:
// Random packet drop 0 => none, ~0 => all
    return q.loss && q.loss >= get_crandom(&q.loss_cor, &q.prng);
    case CLG_4_STATES:
// 4state loss model algorithm (used also for GI model)
// Extracts a value from the markov 4 state loss generator,
// if it is 1 drops a packet and if needed writes the event in
// the kernel logs
//
    return loss_4state(q);
    case CLG_GILB_ELL:
// Gilbert-Elliot loss model algorithm
// Extracts a value from the Gilbert-Elliot loss generator,
// if it is 1 drops a packet and if needed writes the event in
// the kernel logs
//
    return loss_gilb_ell(q);
    }
    return false;	/* not reached */
    }
// tabledist - return a pseudo-randomly distributed value with mean mu and
// std deviation sigma.  Uses table lookup to approximate the desired
// distribution, and a uniformly-distributed pseudo-random source.
//
    static s64 tabledist(s64 mu, s32 sigma,
    struct crndstate *state,
    struct prng *prng,
    const struct disttable *dist)
    {
    s64 x;
    long t;
    u32 rnd;
    if (sigma == 0)
    return mu;
    rnd = get_crandom(state, prng);
// default uniform distribution
    if (dist == core::ptr::null_mut())
    return ((rnd % (2 * (u32)sigma)) + mu) - sigma;
    t = dist.table[rnd % dist.size];
    x = (sigma % NETEM_DIST_SCALE) * t;
    if (x >= 0)
    x += NETEM_DIST_SCALE/2;
    else
    x -= NETEM_DIST_SCALE/2;
    return  x / NETEM_DIST_SCALE + (sigma / NETEM_DIST_SCALE) * t + mu;
    }
#[no_mangle]
unsafe extern "C" fn packet_time_ns(len: u64, q: *const netem_sched_data) -> u64 {
    static u64 packet_time_ns(u64 len, const struct netem_sched_data *q)
    {
    len += q.packet_overhead;
    if (q.cell_size) {
    let mut cells: u32 = reciprocal_divide(len, q.cell_size_reciprocal);
    if (len > cells * q.cell_size)	/* extra cell needed for remainder */
    cells++;
    len = cells * (q.cell_size + q.cell_overhead);
    }
    return div64_u64(len * NSEC_PER_SEC, q.rate);
    }
#[no_mangle]
unsafe extern "C" fn tfifo_reset(sch: *mut Qdisc) {
    static void tfifo_reset(struct Qdisc *sch)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    struct rb_node *p = rb_first(&q.t_root);
    while (p) {
    struct sk_buff *skb = rb_to_skb(p);
    p = rb_next(p);
    rb_erase(&skb.rbnode, &q.t_root);
    rtnl_kfree_skbs(skb, skb);
    }
    rtnl_kfree_skbs(q.t_head, q.t_tail);
    q.t_head = core::ptr::null_mut();
    q.t_tail = core::ptr::null_mut();
    q.t_len = 0;
    }
#[no_mangle]
unsafe extern "C" fn tfifo_enqueue(nskb: *mut sk_buff, sch: *mut Qdisc) {
    static void tfifo_enqueue(struct sk_buff *nskb, struct Qdisc *sch)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    let mut tnext: u64 = netem_skb_cb(nskb).time_to_send;
    if (!q.t_tail || tnext >= netem_skb_cb(q.t_tail).time_to_send) {
    if (q.t_tail)
    q.t_tail.next = nskb;
    else
    q.t_head = nskb;
    q.t_tail = nskb;
    } else {
    struct rb_node **p = &q.t_root.rb_node, *parent = core::ptr::null_mut();
    while (*p) {
    struct sk_buff *skb;
    parent = *p;
    skb = rb_to_skb(parent);
    if (tnext >= netem_skb_cb(skb).time_to_send)
    p = &parent.rb_right;
    else
    p = &parent.rb_left;
    }
    rb_link_node(&nskb.rbnode, parent, p);
    rb_insert_color(&nskb.rbnode, &q.t_root);
    }
    q.t_len++;
    qdisc_qlen_inc(sch);
    }
// netem can't properly corrupt a megapacket (like we get from GSO), so instead
// when we statistically choose to corrupt one, we instead segment it, returning
// the first packet to be corrupted, and re-enqueue the remaining frames
//
    static struct sk_buff *netem_segment(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct sk_buff *segs;
    let mut features: netdev_features_t = netif_skb_features(skb);
    qdisc_skb_cb(skb).pkt_segs = 1;
    segs = skb_gso_segment(skb, features & ~NETIF_F_GSO_MASK);
    if (IS_ERR_OR_NULL(segs)) {
    qdisc_drop(skb, sch, to_free);
    return core::ptr::null_mut();
    }
    consume_skb(skb);
    return segs;
    }
//
// Insert one skb into qdisc.
// Note: parent depends on return value to account for queue length.
// NET_XMIT_DROP: queue length didn't change.
// NET_XMIT_SUCCESS: one skb was queued.
//
    static int netem_enqueue(struct sk_buff *skb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
// We don't fill cb now as skb_unshare() may invalidate it
    struct netem_skb_cb *cb;
    struct sk_buff *skb2 = core::ptr::null_mut();
    struct sk_buff *segs = core::ptr::null_mut();
    let mut prev_len: c_uint = qdisc_pkt_len(skb);
    let mut count: c_int = 1;
// Do not fool qdisc_drop_all()
    skb.prev = core::ptr::null_mut();
// Random duplication
    if (q.duplicate && skb.tc_depth == 0 &&
    q.duplicate >= get_crandom(&q.dup_cor, &q.prng)) {
    ++count;
    WRITE_ONCE(q.duplicated, q.duplicated + 1);
    }
// Drop packet?
    if (loss_event(q)) {
    if (q.ecn && INET_ECN_set_ce(skb)) {
    WRITE_ONCE(q.ecn_marked, q.ecn_marked + 1);
    } else {
    WRITE_ONCE(q.dropped, q.dropped + 1);
    --count;
    }
    }
    if (count == 0) {
    qdisc_qstats_drop(sch);
    __qdisc_drop(skb, to_free);
    return NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    }
// If a delay is expected, orphan the skb. (orphaning usually takes
// place at TX completion time, so _before_ the link transit delay)
//
    if (q.latency || q.jitter || q.rate)
    skb_orphan_partial(skb);
//
// If we need to duplicate packet, then clone it before
// original is modified.
//
    if (count > 1) {
    skb2 = skb_clone(skb, GFP_ATOMIC);
    if (!skb2)
    WRITE_ONCE(q.allocation_errors, q.allocation_errors + 1);
    }
//
// Randomized packet corruption.
// Make copy if needed since we are modifying
// If packet is going to be hardware checksummed, then
// do it now in software before we mangle it.
//
    if (q.corrupt && q.corrupt >= get_crandom(&q.corrupt_cor, &q.prng)) {
    if (skb_is_gso(skb)) {
    skb = netem_segment(skb, sch, to_free);
    if (!skb) {
    WRITE_ONCE(q.allocation_errors, q.allocation_errors + 1);
    goto finish_segs;
    }
    segs = skb.next;
    skb_mark_not_on_list(skb);
    qdisc_skb_cb(skb).pkt_len = skb.len;
    }
    skb = skb_unshare(skb, GFP_ATOMIC);
    if (unlikely(!skb)) {
    WRITE_ONCE(q.allocation_errors, q.allocation_errors + 1);
    qdisc_qstats_drop(sch);
    goto finish_segs;
    }
    if (skb_linearize(skb) ||
    (skb.ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb))) {
    WRITE_ONCE(q.allocation_errors, q.allocation_errors + 1);
    qdisc_drop(skb, sch, to_free);
    skb = core::ptr::null_mut();
    goto finish_segs;
    }
    if (skb.len) {
    let mut offset: u32 = get_random_u32_below(skb.len);
    skb.data[offset] ^= 1 << get_random_u32_below(8);
    WRITE_ONCE(q.corrupted, q.corrupted + 1);
    }
    }
    if (unlikely(sch.q.qlen >= sch.limit)) {
// re-link segs, so that qdisc_drop_all() frees them all
    skb.next = segs;
    qdisc_drop_all(skb, sch, to_free);
    if (skb2)
    __qdisc_drop(skb2, to_free);
    return NET_XMIT_DROP;
    }
//
// If doing duplication then re-insert at top of the
// qdisc tree, since parent queuer expects that only one
// skb will be queued.
//
    if (skb2) {
    struct Qdisc *rootq = qdisc_root_bh(sch);
    skb2.tc_depth++; /* prevent duplicating a dup... */
    rootq.enqueue(skb2, rootq, to_free);
    skb2 = core::ptr::null_mut();
    }
    qdisc_qstats_backlog_inc(sch, skb);
    cb = netem_skb_cb(skb);
    if (q.gap == 0 ||		/* not doing reordering */
    q.counter < q.gap - 1 ||	/* inside last reordering gap */
    q.reorder < get_crandom(&q.reorder_cor, &q.prng)) {
    u64 now;
    s64 delay;
    delay = tabledist(q.latency, q.jitter,
    &q.delay_cor, &q.prng, q.delay_dist);
    now = ktime_get_ns();
    if (q.rate) {
    struct netem_skb_cb *last = core::ptr::null_mut();
    if (sch.q.tail)
    last = netem_skb_cb(sch.q.tail);
    if (q.t_root.rb_node) {
    struct sk_buff *t_skb;
    struct netem_skb_cb *t_last;
    t_skb = skb_rb_last(&q.t_root);
    t_last = netem_skb_cb(t_skb);
    if (!last ||
    t_last.time_to_send > last.time_to_send)
    last = t_last;
    }
    if (q.t_tail) {
    struct netem_skb_cb *t_last =
    netem_skb_cb(q.t_tail);
    if (!last ||
    t_last.time_to_send > last.time_to_send)
    last = t_last;
    }
    if (last) {
//
// Last packet in queue is reference point (now),
// calculate this time bonus and subtract
// from delay.
//
    delay -= last.time_to_send - now;
    delay = max_t(s64, 0, delay);
    now = last.time_to_send;
    }
    delay += packet_time_ns(qdisc_pkt_len(skb), q);
    }
    cb.time_to_send = now + delay;
    ++q.counter;
    if (delay)
    WRITE_ONCE(q.delayed, q.delayed + 1);
    tfifo_enqueue(skb, sch);
    } else {
//
// Do re-ordering by putting one out of N packets at the front
// of the queue.
//
    WRITE_ONCE(q.reordered, q.reordered + 1);
    cb.time_to_send = ktime_get_ns();
    q.counter = 0;
    __qdisc_enqueue_head(skb, &sch.q);
    sch.qstats.requeues++;
    }
    finish_segs:
    if (skb2)
    __qdisc_drop(skb2, to_free);
    if (segs) {
    unsigned int len, last_len;
    int rc, nb;
    len = skb ? skb.len : 0;
    nb = skb ? 1 : 0;
    while (segs) {
    skb2 = segs.next;
    skb_mark_not_on_list(segs);
    qdisc_skb_cb(segs).pkt_len = segs.len;
    last_len = segs.len;
    rc = qdisc_enqueue(segs, sch, to_free);
    if (rc != NET_XMIT_SUCCESS) {
    if (net_xmit_drop_count(rc))
    qdisc_qstats_drop(sch);
    } else {
    nb++;
    len += last_len;
    }
    segs = skb2;
    }
// Parent qdiscs accounted for 1 skb of size @prev_len
    qdisc_tree_reduce_backlog(sch, -(nb - 1), -(len - prev_len));
    } else if (!skb) {
    return NET_XMIT_DROP;
    }
    return NET_XMIT_SUCCESS;
    }
// Delay the next round with a new future slot with a
// correct number of bytes and packets.
//
#[no_mangle]
unsafe extern "C" fn get_slot_next(q: *mut netem_sched_data, now: u64) {
    static void get_slot_next(struct netem_sched_data *q, u64 now)
    {
    s64 next_delay;
    if (!q.slot_dist)
    next_delay = q.slot_config.min_delay +
    mul_u64_u32_shr(q.slot_config.max_delay - q.slot_config.min_delay,
    get_random_u32(), 32);
    else
    next_delay = tabledist(q.slot_config.dist_delay,
    (s32)(q.slot_config.dist_jitter),
    core::ptr::null_mut(), &q.prng, q.slot_dist);
    q.slot.slot_next = now + next_delay;
    q.slot.packets_left = q.slot_config.max_packets;
    q.slot.bytes_left = q.slot_config.max_bytes;
    }
    static struct sk_buff *netem_peek(struct netem_sched_data *q)
    {
    struct sk_buff *skb = skb_rb_first(&q.t_root);
    u64 t1, t2;
    if (!skb)
    return q.t_head;
    if (!q.t_head)
    return skb;
    t1 = netem_skb_cb(skb).time_to_send;
    t2 = netem_skb_cb(q.t_head).time_to_send;
    if (t1 < t2)
    return skb;
    return q.t_head;
    }
#[no_mangle]
unsafe extern "C" fn netem_erase_head(q: *mut netem_sched_data, skb: *mut sk_buff) {
    static void netem_erase_head(struct netem_sched_data *q, struct sk_buff *skb)
    {
    if (skb == q.t_head) {
    q.t_head = skb.next;
    if (!q.t_head)
    q.t_tail = core::ptr::null_mut();
    } else {
    rb_erase(&skb.rbnode, &q.t_root);
    }
    }
    static struct sk_buff *netem_dequeue(struct Qdisc *sch)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb;
    tfifo_dequeue:
    skb = __qdisc_dequeue_head(&sch.q);
    if (skb) {
    deliver:
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
    return skb;
    }
    skb = netem_peek(q);
    if (skb) {
    u64 time_to_send;
    let mut now: u64 = ktime_get_ns();
// if more time remaining?
    time_to_send = netem_skb_cb(skb).time_to_send;
    if (q.slot.slot_next && q.slot.slot_next < time_to_send)
    get_slot_next(q, now);
    if (time_to_send <= now && q.slot.slot_next <= now) {
    netem_erase_head(q, skb);
    q.t_len--;
    skb.next = core::ptr::null_mut();
    skb.prev = core::ptr::null_mut();
// skb->dev shares skb->rbnode area,
// we need to restore its value.
//
    skb.dev = qdisc_dev(sch);
    if (q.slot.slot_next) {
    q.slot.packets_left--;
    q.slot.bytes_left -= qdisc_pkt_len(skb);
    if (q.slot.packets_left <= 0 ||
    q.slot.bytes_left <= 0)
    get_slot_next(q, now);
    }
    if (q.qdisc) {
    let mut pkt_len: c_uint = qdisc_pkt_len(skb);
    struct sk_buff *to_free = core::ptr::null_mut();
    int err;
    err = qdisc_enqueue(skb, q.qdisc, &to_free);
    kfree_skb_list(to_free);
    if (err != NET_XMIT_SUCCESS) {
    if (net_xmit_drop_count(err))
    qdisc_qstats_drop(sch);
    qstats_backlog_sub(sch, pkt_len);
    qdisc_qlen_dec(sch);
    qdisc_tree_reduce_backlog(sch, 1, pkt_len);
    }
    goto tfifo_dequeue;
    }
    qdisc_qlen_dec(sch);
    goto deliver;
    }
    if (q.qdisc) {
    skb = q.qdisc.ops.dequeue(q.qdisc);
    if (skb) {
    qdisc_qlen_dec(sch);
    goto deliver;
    }
    }
    qdisc_watchdog_schedule_ns(&q.watchdog,
    max(time_to_send,
    q.slot.slot_next));
    }
    if (q.qdisc) {
    skb = q.qdisc.ops.dequeue(q.qdisc);
    if (skb) {
    qdisc_qlen_dec(sch);
    goto deliver;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn netem_reset(sch: *mut Qdisc) {
    static void netem_reset(struct Qdisc *sch)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    qdisc_reset_queue(sch);
    tfifo_reset(sch);
    if (q.qdisc)
    qdisc_reset(q.qdisc);
    qdisc_watchdog_cancel(&q.watchdog);
    }
#[no_mangle]
unsafe extern "C" fn dist_free(d: *mut disttable) {
    static void dist_free(struct disttable *d)
    {
    kvfree(d);
    }
//
// Distribution data is a variable size payload containing
// signed 16 bit values.
//
#[no_mangle]
unsafe extern "C" fn get_dist_table(tbl: *mut disttable, attr: *const nlattr) -> c_int {
    static int get_dist_table(struct disttable **tbl, const struct nlattr *attr)
    {
    let mut n: usize = nla_len(attr)/sizeof(__s16);
    const __s16 *data = nla_data(attr);
    struct disttable *d;
    int i;
    if (!n || n > NETEM_DIST_MAX)
    return -EINVAL;
    d = kvmalloc_flex(*d, table, n);
    if (!d)
    return -ENOMEM;
    d.size = n;
    for (i = 0; i < n; i++)
    d.table[i] = data[i];
// tbl = d;
    return 0;
    }
    static int validate_time(const struct nlattr *attr, const char *name,
    struct netlink_ext_ack *extack)
    {
    if (nla_get_s64(attr) < 0) {
    NL_SET_ERR_MSG_ATTR_FMT(extack, attr, "negative %s", name);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn validate_slot(attr: *const nlattr, extack: *mut netlink_ext_ack) -> c_int {
    static int validate_slot(const struct nlattr *attr, struct netlink_ext_ack *extack)
    {
    const struct tc_netem_slot *c = nla_data(attr);
    if (c.min_delay < 0 || c.max_delay < 0) {
    NL_SET_ERR_MSG_ATTR(extack, attr, "negative slot delay");
    return -EINVAL;
    }
    if (c.min_delay > c.max_delay) {
    NL_SET_ERR_MSG_ATTR(extack, attr, "slot min delay greater than max delay");
    return -EINVAL;
    }
    if (c.dist_delay < 0 || c.dist_jitter < 0) {
    NL_SET_ERR_MSG_ATTR(extack, attr, "negative dist delay");
    return -EINVAL;
    }
    if (c.max_packets < 0 || c.max_bytes < 0) {
    NL_SET_ERR_MSG_ATTR(extack, attr, "negative slot limit");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_slot(q: *mut netem_sched_data, attr: *const nlattr) {
    static void get_slot(struct netem_sched_data *q, const struct nlattr *attr)
    {
    const struct tc_netem_slot *c = nla_data(attr);
    q.slot_config = *c;
    if (q.slot_config.max_packets == 0)
    q.slot_config.max_packets = INT_MAX;
    if (q.slot_config.max_bytes == 0)
    q.slot_config.max_bytes = INT_MAX;
// capping dist_jitter to the range acceptable by tabledist()
    q.slot_config.dist_jitter = min_t(__s64, INT_MAX, abs(q.slot_config.dist_jitter));
    q.slot.packets_left = q.slot_config.max_packets;
    q.slot.bytes_left = q.slot_config.max_bytes;
    if (q.slot_config.min_delay | q.slot_config.max_delay |
    q.slot_config.dist_jitter)
    q.slot.slot_next = ktime_get_ns();
    else
    q.slot.slot_next = 0;
    }
#[no_mangle]
unsafe extern "C" fn get_correlation(q: *mut netem_sched_data, attr: *const nlattr) {
    static void get_correlation(struct netem_sched_data *q, const struct nlattr *attr)
    {
    const struct tc_netem_corr *c = nla_data(attr);
    init_crandom(&q.delay_cor, c.delay_corr);
    init_crandom(&q.loss_cor, c.loss_corr);
    init_crandom(&q.dup_cor, c.dup_corr);
    }
#[no_mangle]
unsafe extern "C" fn get_reorder(q: *mut netem_sched_data, attr: *const nlattr) {
    static void get_reorder(struct netem_sched_data *q, const struct nlattr *attr)
    {
    const struct tc_netem_reorder *r = nla_data(attr);
    q.reorder = r.probability;
    init_crandom(&q.reorder_cor, r.correlation);
    }
#[no_mangle]
unsafe extern "C" fn get_corrupt(q: *mut netem_sched_data, attr: *const nlattr) {
    static void get_corrupt(struct netem_sched_data *q, const struct nlattr *attr)
    {
    const struct tc_netem_corrupt *r = nla_data(attr);
    q.corrupt = r.probability;
    init_crandom(&q.corrupt_cor, r.correlation);
    }
#[no_mangle]
unsafe extern "C" fn get_rate(q: *mut netem_sched_data, attr: *const nlattr) {
    static void get_rate(struct netem_sched_data *q, const struct nlattr *attr)
    {
    const struct tc_netem_rate *r = nla_data(attr);
    q.rate = r.rate;
    q.packet_overhead = r.packet_overhead;
    q.cell_size = r.cell_size;
    q.cell_overhead = r.cell_overhead;
    if (q.cell_size)
    q.cell_size_reciprocal = reciprocal_value(q.cell_size);
    else
    q.cell_size_reciprocal = (struct reciprocal_value) { 0 };
    }
    static int get_loss_clg(struct netem_sched_data *q, const struct nlattr *attr,
    struct netlink_ext_ack *extack)
    {
    const struct nlattr *la;
    int rem;
    nla_for_each_nested(la, attr, rem) {
    let mut type: u16 = nla_type(la);
    switch (type) {
    case NETEM_LOSS_GI: {
    const struct tc_netem_gimodel *gi = nla_data(la);
    if (nla_len(la) < sizeof(struct tc_netem_gimodel)) {
    NL_SET_ERR_MSG_ATTR(extack, la,
    "netem: incorrect gi model size");
    return -EINVAL;
    }
    q.loss_model = CLG_4_STATES;
    q.clg.state = TX_IN_GAP_PERIOD;
    q.clg.a1 = gi.p13;
    q.clg.a2 = gi.p31;
    q.clg.a3 = gi.p32;
    q.clg.a4 = gi.p14;
    q.clg.a5 = gi.p23;
    break;
    }
    case NETEM_LOSS_GE: {
    const struct tc_netem_gemodel *ge = nla_data(la);
    if (nla_len(la) < sizeof(struct tc_netem_gemodel)) {
    NL_SET_ERR_MSG_ATTR(extack, la,
    "netem: incorrect ge model size");
    return -EINVAL;
    }
    q.loss_model = CLG_GILB_ELL;
    q.clg.state = GOOD_STATE;
    q.clg.a1 = ge.p;
    q.clg.a2 = ge.r;
    q.clg.a3 = ge.h;
    q.clg.a4 = ge.k1;
    break;
    }
    default:
    NL_SET_ERR_MSG_ATTR_FMT(extack, la,
    "netem: unknown loss type %u", type);
    return -EINVAL;
    }
    }
    return 0;
    }
    static const struct nla_policy netem_policy[TCA_NETEM_MAX + 1] = {
    [TCA_NETEM_CORR]	= { .len = sizeof(struct tc_netem_corr) },
    [TCA_NETEM_REORDER]	= { .len = sizeof(struct tc_netem_reorder) },
    [TCA_NETEM_CORRUPT]	= { .len = sizeof(struct tc_netem_corrupt) },
    [TCA_NETEM_RATE]	= { .len = sizeof(struct tc_netem_rate) },
    [TCA_NETEM_LOSS]	= { .type = NLA_NESTED },
    [TCA_NETEM_ECN]		= { .type = NLA_U32 },
    [TCA_NETEM_RATE64]	= { .type = NLA_U64 },
    [TCA_NETEM_LATENCY64]	= { .type = NLA_S64 },
    [TCA_NETEM_JITTER64]	= { .type = NLA_S64 },
    [TCA_NETEM_SLOT]	= { .len = sizeof(struct tc_netem_slot) },
    [TCA_NETEM_PRNG_SEED]	= { .type = NLA_U64 },
    };
    static int parse_attr(struct nlattr *tb[], int maxtype, struct nlattr *nla,
    const struct nla_policy *policy, int len,
    struct netlink_ext_ack *extack)
    {
    let mut nested_len: c_int = nla_len(nla) - NLA_ALIGN(len);
    if (nested_len < 0) {
    NL_SET_ERR_MSG_FMT(extack, "netem: invalid attributes len %d < %d",
    nla_len(nla), NLA_ALIGN(len));
    return -EINVAL;
    }
    if (nested_len >= nla_attr_size(0))
    return nla_parse_deprecated(tb, maxtype,
    nla_data(nla) + NLA_ALIGN(len),
    nested_len, policy, extack);
    memset(tb, 0, sizeof(struct nlattr *) * (maxtype + 1));
    return 0;
    }
// Parse netlink message to set options
    static int netem_change(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    struct nlattr *tb[TCA_NETEM_MAX + 1];
    struct disttable *delay_dist = core::ptr::null_mut();
    struct disttable *slot_dist = core::ptr::null_mut();
    struct tc_netem_qopt *qopt;
    struct clgstate old_clg;
    let mut old_loss_model: c_int = CLG_RANDOM;
    int ret;
    qopt = nla_data(opt);
    ret = parse_attr(tb, TCA_NETEM_MAX, opt, netem_policy, sizeof(*qopt), extack);
    if (ret < 0)
    return ret;
    if (tb[TCA_NETEM_DELAY_DIST]) {
    ret = get_dist_table(&delay_dist, tb[TCA_NETEM_DELAY_DIST]);
    if (ret)
    goto table_free;
    }
    if (tb[TCA_NETEM_SLOT_DIST]) {
    ret = get_dist_table(&slot_dist, tb[TCA_NETEM_SLOT_DIST]);
    if (ret)
    goto table_free;
    }
    if (tb[TCA_NETEM_SLOT]) {
    ret = validate_slot(tb[TCA_NETEM_SLOT], extack);
    if (ret)
    goto table_free;
    }
    if (tb[TCA_NETEM_LATENCY64]) {
    ret = validate_time(tb[TCA_NETEM_LATENCY64], "latency", extack);
    if (ret)
    goto table_free;
    }
    if (tb[TCA_NETEM_JITTER64]) {
    ret = validate_time(tb[TCA_NETEM_JITTER64], "jitter", extack);
    if (ret)
    goto table_free;
    }
    sch_tree_lock(sch);
// backup q->clg and q->loss_model
    old_clg = q.clg;
    old_loss_model = q.loss_model;
    if (tb[TCA_NETEM_LOSS]) {
    ret = get_loss_clg(q, tb[TCA_NETEM_LOSS], extack);
    if (ret) {
    q.loss_model = old_loss_model;
    q.clg = old_clg;
    goto unlock;
    }
    } else {
    q.loss_model = CLG_RANDOM;
    }
    if (delay_dist)
    swap(q.delay_dist, delay_dist);
    if (slot_dist)
    swap(q.slot_dist, slot_dist);
    sch.limit = qopt.limit;
    q.latency = PSCHED_TICKS2NS(qopt.latency);
    q.jitter = PSCHED_TICKS2NS(qopt.jitter);
    q.limit = qopt.limit;
    q.gap = qopt.gap;
    q.counter = 0;
    q.loss = qopt.loss;
    q.duplicate = qopt.duplicate;
// for compatibility with earlier versions.
// if gap is set, need to assume 100% probability
//
    if (q.gap)
    q.reorder = ~0;
    if (tb[TCA_NETEM_CORR])
    get_correlation(q, tb[TCA_NETEM_CORR]);
    if (tb[TCA_NETEM_REORDER])
    get_reorder(q, tb[TCA_NETEM_REORDER]);
    if (tb[TCA_NETEM_CORRUPT])
    get_corrupt(q, tb[TCA_NETEM_CORRUPT]);
    if (tb[TCA_NETEM_RATE])
    get_rate(q, tb[TCA_NETEM_RATE]);
    if (tb[TCA_NETEM_RATE64])
    q.rate = max_t(u64, q.rate,
    nla_get_u64(tb[TCA_NETEM_RATE64]));
    if (tb[TCA_NETEM_LATENCY64])
    q.latency = nla_get_s64(tb[TCA_NETEM_LATENCY64]);
    if (tb[TCA_NETEM_JITTER64])
    q.jitter = nla_get_s64(tb[TCA_NETEM_JITTER64]);
    if (tb[TCA_NETEM_ECN])
    q.ecn = nla_get_u32(tb[TCA_NETEM_ECN]);
    if (tb[TCA_NETEM_SLOT])
    get_slot(q, tb[TCA_NETEM_SLOT]);
// capping jitter to the range acceptable by tabledist()
    q.jitter = min_t(s64, abs(q.jitter), INT_MAX);
    if (tb[TCA_NETEM_PRNG_SEED]) {
    q.prng.seed = nla_get_u64(tb[TCA_NETEM_PRNG_SEED]);
    prandom_seed_state(&q.prng.prng_state, q.prng.seed);
    }
    unlock:
    sch_tree_unlock(sch);
    table_free:
    dist_free(delay_dist);
    dist_free(slot_dist);
    return ret;
    }
    static int netem_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    qdisc_watchdog_init(&q.watchdog, sch);
    if (!opt)
    return -EINVAL;
    q.loss_model = CLG_RANDOM;
    q.prng.seed = get_random_u64();
    prandom_seed_state(&q.prng.prng_state, q.prng.seed);
    return netem_change(sch, opt, extack);
    }
#[no_mangle]
unsafe extern "C" fn netem_destroy(sch: *mut Qdisc) {
    static void netem_destroy(struct Qdisc *sch)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    qdisc_watchdog_cancel(&q.watchdog);
    if (q.qdisc)
    qdisc_put(q.qdisc);
    dist_free(q.delay_dist);
    dist_free(q.slot_dist);
    }
    static int dump_loss_model(const struct netem_sched_data *q,
    struct sk_buff *skb)
    {
    struct nlattr *nest;
    nest = nla_nest_start_noflag(skb, TCA_NETEM_LOSS);
    if (nest == core::ptr::null_mut())
    goto nla_put_failure;
    switch (q.loss_model) {
    case CLG_RANDOM:
// legacy loss model
    nla_nest_cancel(skb, nest);
    return 0;	/* no data */
    case CLG_4_STATES: {
    struct tc_netem_gimodel gi = {
    .p13 = q.clg.a1,
    .p31 = q.clg.a2,
    .p32 = q.clg.a3,
    .p14 = q.clg.a4,
    .p23 = q.clg.a5,
    };
    if (nla_put(skb, NETEM_LOSS_GI, sizeof(gi), &gi))
    goto nla_put_failure;
    break;
    }
    case CLG_GILB_ELL: {
    struct tc_netem_gemodel ge = {
    .p = q.clg.a1,
    .r = q.clg.a2,
    .h = q.clg.a3,
    .k1 = q.clg.a4,
    };
    if (nla_put(skb, NETEM_LOSS_GE, sizeof(ge), &ge))
    goto nla_put_failure;
    break;
    }
    }
    nla_nest_end(skb, nest);
    return 0;
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn netem_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int netem_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    const struct netem_sched_data *q = qdisc_priv(sch);
    struct nlattr *nla = (struct nlattr *) skb_tail_pointer(skb);
    struct tc_netem_qopt qopt;
    struct tc_netem_corr cor;
    struct tc_netem_reorder reorder;
    struct tc_netem_corrupt corrupt;
    struct tc_netem_rate rate;
    struct tc_netem_slot slot;
    qopt.latency = min_t(psched_time_t, PSCHED_NS2TICKS(q.latency),
    UINT_MAX);
    qopt.jitter = min_t(psched_time_t, PSCHED_NS2TICKS(q.jitter),
    UINT_MAX);
    qopt.limit = q.limit;
    qopt.loss = q.loss;
    qopt.gap = q.gap;
    qopt.duplicate = q.duplicate;
    if (nla_put(skb, TCA_OPTIONS, sizeof(qopt), &qopt))
    goto nla_put_failure;
    if (nla_put(skb, TCA_NETEM_LATENCY64, sizeof(q.latency), &q.latency))
    goto nla_put_failure;
    if (nla_put(skb, TCA_NETEM_JITTER64, sizeof(q.jitter), &q.jitter))
    goto nla_put_failure;
    cor.delay_corr = q.delay_cor.rho;
    cor.loss_corr = q.loss_cor.rho;
    cor.dup_corr = q.dup_cor.rho;
    if (nla_put(skb, TCA_NETEM_CORR, sizeof(cor), &cor))
    goto nla_put_failure;
    reorder.probability = q.reorder;
    reorder.correlation = q.reorder_cor.rho;
    if (nla_put(skb, TCA_NETEM_REORDER, sizeof(reorder), &reorder))
    goto nla_put_failure;
    corrupt.probability = q.corrupt;
    corrupt.correlation = q.corrupt_cor.rho;
    if (nla_put(skb, TCA_NETEM_CORRUPT, sizeof(corrupt), &corrupt))
    goto nla_put_failure;
    if (q.rate >= (1ULL << 32)) {
    if (nla_put_u64_64bit(skb, TCA_NETEM_RATE64, q.rate,
    TCA_NETEM_PAD))
    goto nla_put_failure;
    rate.rate = ~0U;
    } else {
    rate.rate = q.rate;
    }
    rate.packet_overhead = q.packet_overhead;
    rate.cell_size = q.cell_size;
    rate.cell_overhead = q.cell_overhead;
    if (nla_put(skb, TCA_NETEM_RATE, sizeof(rate), &rate))
    goto nla_put_failure;
    if (q.ecn && nla_put_u32(skb, TCA_NETEM_ECN, q.ecn))
    goto nla_put_failure;
    if (dump_loss_model(q, skb) != 0)
    goto nla_put_failure;
    if (q.slot_config.min_delay | q.slot_config.max_delay |
    q.slot_config.dist_jitter) {
    slot = q.slot_config;
    if (slot.max_packets == INT_MAX)
    slot.max_packets = 0;
    if (slot.max_bytes == INT_MAX)
    slot.max_bytes = 0;
    if (nla_put(skb, TCA_NETEM_SLOT, sizeof(slot), &slot))
    goto nla_put_failure;
    }
    if (nla_put_u64_64bit(skb, TCA_NETEM_PRNG_SEED, q.prng.seed,
    TCA_NETEM_PAD))
    goto nla_put_failure;
    return nla_nest_end(skb, nla);
    nla_put_failure:
    nlmsg_trim(skb, nla);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn netem_dump_stats(sch: *mut Qdisc, d: *mut gnet_dump) -> c_int {
    static int netem_dump_stats(struct Qdisc *sch, struct gnet_dump *d)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    struct tc_netem_xstats st = {
    .delayed    = READ_ONCE(q.delayed),
    .dropped    = READ_ONCE(q.dropped),
    .corrupted  = READ_ONCE(q.corrupted),
    .duplicated = READ_ONCE(q.duplicated),
    .reordered  = READ_ONCE(q.reordered),
    .ecn_marked = READ_ONCE(q.ecn_marked),
    .allocation_errors = READ_ONCE(q.allocation_errors),
    };
    return gnet_stats_copy_app(d, &st, sizeof(st));
    }
    static int netem_dump_class(struct Qdisc *sch, unsigned long cl,
    struct sk_buff *skb, struct tcmsg *tcm)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    if (cl != 1 || !q.qdisc) 	/* only one class */
    return -ENOENT;
    tcm.tcm_handle |= TC_H_MIN(1);
    tcm.tcm_info = q.qdisc.handle;
    return 0;
    }
    static int netem_graft(struct Qdisc *sch, unsigned long arg, struct Qdisc *new,
    struct Qdisc **old, struct netlink_ext_ack *extack)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
// old = qdisc_replace(sch, new, &q->qdisc);
    return 0;
    }
    static struct Qdisc *netem_leaf(struct Qdisc *sch, unsigned long arg)
    {
    struct netem_sched_data *q = qdisc_priv(sch);
    return q.qdisc;
    }
#[no_mangle]
unsafe extern "C" fn netem_find(sch: *mut Qdisc, classid: u32) -> c_ulong {
    static unsigned long netem_find(struct Qdisc *sch, u32 classid)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn netem_walk(sch: *mut Qdisc, walker: *mut qdisc_walker) {
    static void netem_walk(struct Qdisc *sch, struct qdisc_walker *walker)
    {
    if (!walker.stop) {
    if (!tc_qdisc_stats_dump(sch, 1, walker))
    return;
    }
    }
    static const struct Qdisc_class_ops netem_class_ops = {
    .graft		=	netem_graft,
    .leaf		=	netem_leaf,
    .find		=	netem_find,
    .walk		=	netem_walk,
    .dump		=	netem_dump_class,
    };
    static struct Qdisc_ops netem_qdisc_ops __read_mostly = {
    .id		=	"netem",
    .cl_ops		=	&netem_class_ops,
    .priv_size	=	sizeof(struct netem_sched_data),
    .enqueue	=	netem_enqueue,
    .dequeue	=	netem_dequeue,
    .peek		=	qdisc_peek_dequeued,
    .init		=	netem_init,
    .reset		=	netem_reset,
    .destroy	=	netem_destroy,
    .change		=	netem_change,
    .dump		=	netem_dump,
    .dump_stats	=	netem_dump_stats,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("netem");
#[no_mangle]
unsafe extern "C" fn netem_module_init() -> int __init {
    static int __init netem_module_init(void)
    {
    return register_qdisc(&netem_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn netem_module_exit() -> void __exit {
    static void __exit netem_module_exit(void)
    {
    unregister_qdisc(&netem_qdisc_ops);
    }
    module_init(netem_module_init)
    module_exit(netem_module_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Network characteristics emulator qdisc");
