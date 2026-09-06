//! Automatically rewritten from C to Rust
//! Source: net/sched/sch_etf.c
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


// SPDX-License-Identifier: GPL-2.0
// net/sched/sch_etf.c  Earliest TxTime First queueing discipline.
//
// Authors:	Jesus Sanchez-Palencia <jesus.sanchez-palencia@intel.com>
// Vinicius Costa Gomes <vinicius.gomes@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etf_sched_data {
    pub offload: bool,
    pub deadline_mode: bool,
    pub skip_sock_check: bool,
    pub clockid: c_int,
    pub queue: c_int,
    pub /: *mut *mut s32 delta; / in ns,
    pub /: *mut *mut ktime_t last; / The txtime of the last skb sent to the netdevice.,
    pub head: rb_root_cached,
    pub watchdog: qdisc_watchdog,
    pub (*get_time)(void): *mut ktime_t,
}

    static const struct nla_policy etf_policy[TCA_ETF_MAX + 1] = {
    [TCA_ETF_PARMS]	= { .len = sizeof(struct tc_etf_qopt) },
    };
    static inline int validate_input_params(struct tc_etf_qopt *qopt,
    struct netlink_ext_ack *extack)
    {
// Check if params comply to the following rules:
// * Clockid and delta must be valid.
//
// * Dynamic clockids are not supported.
//
// * Delta must be a positive integer.
//
// Also note that for the HW offload case, we must
// expect that system clocks have been synchronized to PHC.
//
    if (qopt.clockid < 0) {
    NL_SET_ERR_MSG(extack, "Dynamic clockids are not supported");
    return -ENOTSUPP;
    }
    if (qopt.clockid != CLOCK_TAI) {
    NL_SET_ERR_MSG(extack, "Invalid clockid. CLOCK_TAI must be used");
    return -EINVAL;
    }
    if (qopt.delta < 0) {
    NL_SET_ERR_MSG(extack, "Delta must be positive");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_packet_valid(sch: *mut Qdisc, nskb: *mut sk_buff) -> bool {
    static bool is_packet_valid(struct Qdisc *sch, struct sk_buff *nskb)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    let mut txtime: ktime_t = nskb.tstamp;
    struct sock *sk = nskb.sk;
    ktime_t now;
    if (q.skip_sock_check)
    goto skip;
    if (!sk || !sk_fullsock(sk))
    return false;
    if (!sock_flag(sk, SOCK_TXTIME))
    return false;
// We don't perform crosstimestamping.
// Drop if packet's clockid differs from qdisc's.
//
    if (sk.sk_clockid != q.clockid)
    return false;
    if (sk.sk_txtime_deadline_mode != q.deadline_mode)
    return false;
    skip:
    now = q.get_time();
    if (ktime_before(txtime, now) || ktime_before(txtime, q.last))
    return false;
    return true;
    }
    static struct sk_buff *etf_peek_timesortedlist(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct rb_node *p;
    p = rb_first_cached(&q.head);
    if (!p)
    return core::ptr::null_mut();
    return rb_to_skb(p);
    }
#[no_mangle]
unsafe extern "C" fn reset_watchdog(sch: *mut Qdisc) {
    static void reset_watchdog(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb = etf_peek_timesortedlist(sch);
    ktime_t next;
    if (!skb) {
    qdisc_watchdog_cancel(&q.watchdog);
    return;
    }
    next = ktime_sub_ns(skb.tstamp, q.delta);
    qdisc_watchdog_schedule_ns(&q.watchdog, ktime_to_ns(next));
    }
#[no_mangle]
unsafe extern "C" fn report_sock_error(skb: *mut sk_buff, err: u32, code: u8) {
    static void report_sock_error(struct sk_buff *skb, u32 err, u8 code)
    {
    struct sock_exterr_skb *serr;
    struct sk_buff *clone;
    let mut txtime: ktime_t = skb.tstamp;
    struct sock *sk = skb.sk;
    if (!sk || !sk_fullsock(sk) || !(sk.sk_txtime_report_errors))
    return;
    clone = skb_clone(skb, GFP_ATOMIC);
    if (!clone)
    return;
    serr = SKB_EXT_ERR(clone);
    serr.ee.ee_errno = err;
    serr.ee.ee_origin = SO_EE_ORIGIN_TXTIME;
    serr.ee.ee_type = 0;
    serr.ee.ee_code = code;
    serr.ee.ee_pad = 0;
    serr.ee.ee_data = (txtime >> 32); /* high part of tstamp */
    serr.ee.ee_info = txtime; /* low part of tstamp */
    if (sock_queue_err_skb(sk, clone))
    kfree_skb(clone);
    }
    static int etf_enqueue_timesortedlist(struct sk_buff *nskb, struct Qdisc *sch,
    struct sk_buff **to_free)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct rb_node **p = &q.head.rb_root.rb_node, *parent = core::ptr::null_mut();
    let mut txtime: ktime_t = nskb.tstamp;
    let mut leftmost: bool = true;
    if (!is_packet_valid(sch, nskb)) {
    report_sock_error(nskb, EINVAL,
    SO_EE_CODE_TXTIME_INVALID_PARAM);
    return qdisc_drop(nskb, sch, to_free);
    }
    while (*p) {
    struct sk_buff *skb;
    parent = *p;
    skb = rb_to_skb(parent);
    if (ktime_compare(txtime, skb.tstamp) >= 0) {
    p = &parent.rb_right;
    leftmost = false;
    } else {
    p = &parent.rb_left;
    }
    }
    rb_link_node(&nskb.rbnode, parent, p);
    rb_insert_color_cached(&nskb.rbnode, &q.head, leftmost);
    qdisc_qstats_backlog_inc(sch, nskb);
    qdisc_qlen_inc(sch);
// Now we may need to re-arm the qdisc watchdog for the next packet.
    reset_watchdog(sch);
    return NET_XMIT_SUCCESS;
    }
    static void timesortedlist_drop(struct Qdisc *sch, struct sk_buff *skb,
    ktime_t now)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct sk_buff *to_free = core::ptr::null_mut();
    struct sk_buff *tmp = core::ptr::null_mut();
    skb_rbtree_walk_from_safe(skb, tmp) {
    if (ktime_after(skb.tstamp, now))
    break;
    rb_erase_cached(&skb.rbnode, &q.head);
// The rbnode field in the skb re-uses these fields, now that
// we are done with the rbnode, reset them.
//
    skb.next = core::ptr::null_mut();
    skb.prev = core::ptr::null_mut();
    skb.dev = qdisc_dev(sch);
    report_sock_error(skb, ECANCELED, SO_EE_CODE_TXTIME_MISSED);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_drop(skb, sch, &to_free);
    qdisc_qstats_overlimit(sch);
    qdisc_qlen_dec(sch);
    }
    kfree_skb_list(to_free);
    }
#[no_mangle]
unsafe extern "C" fn timesortedlist_remove(sch: *mut Qdisc, skb: *mut sk_buff) {
    static void timesortedlist_remove(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    rb_erase_cached(&skb.rbnode, &q.head);
// The rbnode field in the skb re-uses these fields, now that
// we are done with the rbnode, reset them.
//
    skb.next = core::ptr::null_mut();
    skb.prev = core::ptr::null_mut();
    skb.dev = qdisc_dev(sch);
    qdisc_qstats_backlog_dec(sch, skb);
    qdisc_bstats_update(sch, skb);
    q.last = skb.tstamp;
    qdisc_qlen_dec(sch);
    }
    static struct sk_buff *etf_dequeue_timesortedlist(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct sk_buff *skb;
    ktime_t now, next;
    skb = etf_peek_timesortedlist(sch);
    if (!skb)
    return core::ptr::null_mut();
    now = q.get_time();
// Drop if packet has expired while in queue.
    if (ktime_before(skb.tstamp, now)) {
    timesortedlist_drop(sch, skb, now);
    skb = core::ptr::null_mut();
    goto out;
    }
// When in deadline mode, dequeue as soon as possible and change the
// txtime from deadline to (now + delta).
//
    if (q.deadline_mode) {
    timesortedlist_remove(sch, skb);
    skb.tstamp = now;
    goto out;
    }
    next = ktime_sub_ns(skb.tstamp, q.delta);
// Dequeue only if now is within the [txtime - delta, txtime] range.
    if (ktime_after(now, next))
    timesortedlist_remove(sch, skb);
    else
    skb = core::ptr::null_mut();
    out:
// Now we may need to re-arm the qdisc watchdog for the next packet.
    reset_watchdog(sch);
    return skb;
    }
    static void etf_disable_offload(struct net_device *dev,
    struct etf_sched_data *q)
    {
    let mut etf: tc_etf_qopt_offload = { };
    const struct net_device_ops *ops;
    int err;
    if (!q.offload)
    return;
    ops = dev.netdev_ops;
    if (!ops.ndo_setup_tc)
    return;
    etf.queue = q.queue;
    etf.enable = 0;
    err = ops.ndo_setup_tc(dev, TC_SETUP_QDISC_ETF, &etf);
    if (err < 0)
    pr_warn("Couldn't disable ETF offload for queue %d\n",
    etf.queue);
    }
    static int etf_enable_offload(struct net_device *dev, struct etf_sched_data *q,
    struct netlink_ext_ack *extack)
    {
    const struct net_device_ops *ops = dev.netdev_ops;
    let mut etf: tc_etf_qopt_offload = { };
    int err;
    if (!ops.ndo_setup_tc) {
    NL_SET_ERR_MSG(extack, "Specified device does not support ETF offload");
    return -EOPNOTSUPP;
    }
    etf.queue = q.queue;
    etf.enable = 1;
    err = ops.ndo_setup_tc(dev, TC_SETUP_QDISC_ETF, &etf);
    if (err < 0) {
    NL_SET_ERR_MSG(extack, "Specified device failed to setup ETF hardware offload");
    return err;
    }
    return 0;
    }
    static int etf_init(struct Qdisc *sch, struct nlattr *opt,
    struct netlink_ext_ack *extack)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct net_device *dev = qdisc_dev(sch);
    struct nlattr *tb[TCA_ETF_MAX + 1];
    struct tc_etf_qopt *qopt;
    int err;
    if (!opt) {
    NL_SET_ERR_MSG(extack,
    "Missing ETF qdisc options which are mandatory");
    return -EINVAL;
    }
    err = nla_parse_nested_deprecated(tb, TCA_ETF_MAX, opt, etf_policy,
    extack);
    if (err < 0)
    return err;
    if (!tb[TCA_ETF_PARMS]) {
    NL_SET_ERR_MSG(extack, "Missing mandatory ETF parameters");
    return -EINVAL;
    }
    qopt = nla_data(tb[TCA_ETF_PARMS]);
    pr_debug("delta %d clockid %d offload %s deadline %s\n",
    qopt.delta, qopt.clockid,
    OFFLOAD_IS_ON(qopt) ? "on" : "off",
    DEADLINE_MODE_IS_ON(qopt) ? "on" : "off");
    err = validate_input_params(qopt, extack);
    if (err < 0)
    return err;
    q.queue = sch.dev_queue - netdev_get_tx_queue(dev, 0);
    if (OFFLOAD_IS_ON(qopt)) {
    err = etf_enable_offload(dev, q, extack);
    if (err < 0)
    return err;
    }
// Everything went OK, save the parameters used.
    q.delta = qopt.delta;
    q.clockid = qopt.clockid;
    q.offload = OFFLOAD_IS_ON(qopt);
    q.deadline_mode = DEADLINE_MODE_IS_ON(qopt);
    q.skip_sock_check = SKIP_SOCK_CHECK_IS_SET(qopt);
    switch (q.clockid) {
    case CLOCK_REALTIME:
    q.get_time = ktime_get_real;
    break;
    case CLOCK_MONOTONIC:
    q.get_time = ktime_get;
    break;
    case CLOCK_BOOTTIME:
    q.get_time = ktime_get_boottime;
    break;
    case CLOCK_TAI:
    q.get_time = ktime_get_clocktai;
    break;
    default:
    NL_SET_ERR_MSG(extack, "Clockid is not supported");
    return -ENOTSUPP;
    }
    qdisc_watchdog_init_clockid(&q.watchdog, sch, q.clockid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timesortedlist_clear(sch: *mut Qdisc) {
    static void timesortedlist_clear(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct rb_node *p = rb_first_cached(&q.head);
    while (p) {
    struct sk_buff *skb = rb_to_skb(p);
    p = rb_next(p);
    rb_erase_cached(&skb.rbnode, &q.head);
    rtnl_kfree_skbs(skb, skb);
    qdisc_qlen_dec(sch);
    }
    }
#[no_mangle]
unsafe extern "C" fn etf_reset(sch: *mut Qdisc) {
    static void etf_reset(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
// Only cancel watchdog if it's been initialized.
    if (q.watchdog.qdisc == sch)
    qdisc_watchdog_cancel(&q.watchdog);
// No matter which mode we are on, it's safe to clear both lists.
    timesortedlist_clear(sch);
    __qdisc_reset_queue(&sch.q);
    q.last = 0;
    }
#[no_mangle]
unsafe extern "C" fn etf_destroy(sch: *mut Qdisc) {
    static void etf_destroy(struct Qdisc *sch)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    struct net_device *dev = qdisc_dev(sch);
// Only cancel watchdog if it's been initialized.
    if (q.watchdog.qdisc == sch)
    qdisc_watchdog_cancel(&q.watchdog);
    etf_disable_offload(dev, q);
    }
#[no_mangle]
unsafe extern "C" fn etf_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    static int etf_dump(struct Qdisc *sch, struct sk_buff *skb)
    {
    struct etf_sched_data *q = qdisc_priv(sch);
    let mut opt: tc_etf_qopt = { };
    struct nlattr *nest;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if (!nest)
    goto nla_put_failure;
    opt.delta = READ_ONCE(q.delta);
    opt.clockid = READ_ONCE(q.clockid);
    if (READ_ONCE(q.offload))
    opt.flags |= TC_ETF_OFFLOAD_ON;
    if (READ_ONCE(q.deadline_mode))
    opt.flags |= TC_ETF_DEADLINE_MODE_ON;
    if (READ_ONCE(q.skip_sock_check))
    opt.flags |= TC_ETF_SKIP_SOCK_CHECK;
    if (nla_put(skb, TCA_ETF_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;
    return nla_nest_end(skb, nest);
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return -1;
    }
    static struct Qdisc_ops etf_qdisc_ops __read_mostly = {
    .id		=	"etf",
    .priv_size	=	sizeof(struct etf_sched_data),
    .enqueue	=	etf_enqueue_timesortedlist,
    .dequeue	=	etf_dequeue_timesortedlist,
    .peek		=	etf_peek_timesortedlist,
    .init		=	etf_init,
    .reset		=	etf_reset,
    .destroy	=	etf_destroy,
    .dump		=	etf_dump,
    .owner		=	THIS_MODULE,
    };
    MODULE_ALIAS_NET_SCH("etf");
#[no_mangle]
unsafe extern "C" fn etf_module_init() -> int __init {
    static int __init etf_module_init(void)
    {
    return register_qdisc(&etf_qdisc_ops);
    }
#[no_mangle]
unsafe extern "C" fn etf_module_exit() -> void __exit {
    static void __exit etf_module_exit(void)
    {
    unregister_qdisc(&etf_qdisc_ops);
    }
    module_init(etf_module_init)
    module_exit(etf_module_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Earliest TxTime First (ETF) qdisc");
