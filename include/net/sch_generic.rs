//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sch_generic.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_rate_table {
    pub rate: tc_ratespec,
    pub data: [u32; 256],
    pub next: *mut qdisc_rate_table,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdisc_state_t {
    __QDISC_STATE_SCHED,
    __QDISC_STATE_DEACTIVATED,
    __QDISC_STATE_MISSED,
    __QDISC_STATE_DRAINING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_size_table {
    pub rcu: rcu_head,
    pub list: list_head,
    pub szopts: tc_sizespec,
    pub refcnt: c_int,
    pub data: [u16; ],
}

// similar to sk_buff_head, but skb->prev pointer is undefined.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_skb_head {
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub qlen: __u32,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qdisc {
    pub to_free): *mut sk_buff,
    pub sch): *mut *mut *mut sk_buff  (dequeue)(Qdisc,
    pub flags: c_uint,
pub const TCQ_F_BUILTIN: c_int = 1;
pub const TCQ_F_INGRESS: c_int = 2;
pub const TCQ_F_CAN_BYPASS: c_int = 4;
pub const TCQ_F_MQROOT: c_int = 8;
pub const TCQ_F_ONETXQUEUE: c_uint = 0x10 /* dequeue_skb() can assume all skbs are for;
// q->dev_queue : It can test
// netif_xmit_frozen_or_stopped() before
// dequeueing next packet.
// Its true for MQ/MQPRIO slaves, or non
// multiqueue device.
//

pub const TCQ_F_CPUSTATS: c_uint = 0x20 /* run using percpu statistics */;
pub const TCQ_F_NOPARENT: c_uint = 0x40 /* root of its hierarchy :;
// qdisc_tree_reduce_backlog() should stop.
//
pub const TCQ_F_INVISIBLE: c_uint = 0x80 /* invisible by default in dump */;
pub const TCQ_F_NOLOCK: c_uint = 0x100 /* qdisc does not require locking */;
pub const TCQ_F_OFFLOADED: c_uint = 0x200 /* qdisc is offloaded to HW */;
pub const TCQ_F_DEQUEUE_DROPS: c_uint = 0x400 /* ->dequeue() can drop packets in q->to_free */;
    pub limit: u32,
    pub ops: *const Qdisc_ops,
    pub stab: *mut qdisc_size_table __rcu,
    pub hash: hlist_node,
    pub handle: u32,
    pub parent: u32,
    pub depth: c_int,
    pub dev_queue: *mut netdev_queue,
    pub rate_est: *mut net_rate_estimator __rcu,
    pub cpu_bstats: *mut gnet_stats_basic_sync __percpu,
    pub cpu_qstats: *mut gnet_stats_queue __percpu,
    pub pad: c_int,
    pub refcnt: refcount_t,
// Cache line potentially dirtied in dequeue() or __netif_reschedule().
    pub ____cacheline_aligned: __cacheline_group_begin(Qdisc_read_mostly),
    pub gso_skb: sk_buff_head,
    pub next_sched: *mut Qdisc,
    pub skb_bad_txq: sk_buff_head,
// Fields dirtied in dequeue() fast path.
    pub ____cacheline_aligned: __cacheline_group_begin(Qdisc_write),
    pub q: qdisc_skb_head,
    pub state: c_ulong,
    pub bstats: gnet_stats_basic_sync,
    pub /: *mut *mut bool running; / must be written under qdisc spinlock,
// Note : we only change qstats.backlog in fast path.
    pub qstats: gnet_stats_queue,
    pub to_free: *mut sk_buff,
    pub ____cacheline_aligned_in_smp: atomic_long_t defer_count,
    pub defer_list: llist_head,
    pub seqlock: spinlock_t,
    pub rcu: rcu_head,
    pub dev_tracker: netdevice_tracker,
    pub root_lock_key: lock_class_key,
// private data
    pub ____cacheline_aligned: long privdata[],
}

extern "C" {
    pub fn refcount_dec_if_one(_arg: &qdisc->refcnt) -> return;
}
// Intended to be used by unlocked users, when concurrent qdisc release is
// possible.
//
// For !TCQ_F_NOLOCK qdisc: callers must either call this within a qdisc
// root_lock section, or provide their own memory barriers -- ordering
// against qdisc_run_begin/end() atomic bit operations.
//
extern "C" {
    pub fn spin_is_locked(_arg: &qdisc->seqlock) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: qdisc->running) -> return;
}
extern "C" {
    pub fn nolock_qdisc_is_empty(_arg: qdisc) -> return;
}
// For !TCQ_F_NOLOCK qdisc, qdisc_run_begin/end() must be invoked with
// the qdisc root lock acquired.
//
// No need to insist if the MISSED flag was already set.
// Note that test_and_set_bit() also gives us memory ordering
// guarantees wrt potential earlier enqueue() and below
// spin_trylock(), both of which are necessary to prevent races
//
// Try to take the lock again to make sure that we will either
// grab it or the CPU that still has it will see MISSED set
// when testing it in qdisc_run_end()
//
extern "C" {
    pub fn spin_trylock(_arg: &qdisc->seqlock) -> return;
}
// spin_unlock() only has store-release semantic. The unlock
// and test_bit() ordering is a store-load ordering, so a full
// memory barrier is needed here.
//
extern "C" {
    pub fn netdev_queue_dql_avail(_arg: txq) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qdisc_class_ops {
    pub flags: c_uint,
// Child qdisc manipulation
    pub ): *mut *mut *mut *mut netdev_queue  (select_queue)(Qdisc , tcmsg,
    pub extack): *mut netlink_ext_ack,
    pub cl): *mut *mut *mut *mut Qdisc  (leaf)(Qdisc , unsigned long,
    pub long): *mut *mut *mut void (qlen_notify)(struct Qdisc , unsigned,
// Class manipulation routines
    pub classid): *mut *mut *mut unsigned long (find)(struct Qdisc , u32,
    pub ): *mut netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub arg): *mut *mut *mut *mut void (walk)(struct Qdisc , struct qdisc_walker,
// Filter manipulation
    pub extack): *mut netlink_ext_ack,
    pub classid): u32,
    pub long): *mut *mut *mut void (unbind_tcf)(struct Qdisc , unsigned,
// rtnetlink specific
    pub tcmsg*): *mut *mut sk_buff skb, struct,
    pub ): *mut gnet_dump,
}

// Qdisc_class_ops flag values
// Implements API that doesn't require rtnl lock
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdisc_class_ops_flags {
    QDISC_CLASS_OPS_DOIT_UNLOCKED = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qdisc_ops {
    pub next: *mut Qdisc_ops,
    pub cl_ops: *const Qdisc_class_ops,
    pub id: [c_char; IFNAMSIZ],
    pub priv_size: c_int,
    pub static_flags: c_uint,
    pub to_free): *mut sk_buff,
    pub ): *mut *mut *mut sk_buff  (dequeue)(Qdisc,
    pub ): *mut *mut *mut sk_buff  (peek)(Qdisc,
    pub extack): *mut netlink_ext_ack,
    pub ): *mut *mut void (reset)(struct Qdisc,
    pub ): *mut *mut void (destroy)(struct Qdisc,
    pub extack): *mut netlink_ext_ack,
    pub sch): *mut *mut void (attach)(struct Qdisc,
    pub int): *mut *mut *mut int (change_tx_queue_len)(struct Qdisc , unsigned,
    pub new_real_tx): c_uint,
    pub ): *mut *mut *mut int (dump)(struct Qdisc , struct sk_buff,
    pub ): *mut *mut *mut int (dump_stats)(struct Qdisc , struct gnet_dump,
    pub block_index): u32,
    pub block_index): u32,
    pub sch): *mut *mut u32 (ingress_block_get)(struct Qdisc,
    pub sch): *mut *mut u32 (egress_block_get)(struct Qdisc,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_result {
    pub class: c_ulong,
    pub classid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_proto_ops {
    pub head: list_head,
    pub kind: [c_char; IFNAMSIZ],
    pub ): *mut tcf_result,
    pub tcf_proto*): *mut *mut int (init)(struct,
    pub extack): *mut netlink_ext_ack,
    pub handle): *mut *mut *mut *mut void (get)(struct tcf_proto, u32,
    pub f): *mut *mut *mut void (put)(struct tcf_proto tp, void,
    pub ): *mut netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub tp): *mut *mut bool (delete_empty)(struct tcf_proto,
    pub rtnl_held): *mut *mut tcf_walker arg, bool,
    pub extack): *mut netlink_ext_ack,
    pub type_data): *mut c_void,
    pub type_data): *mut c_void,
    pub long): *mut *mut void , unsigned,
    pub extack): *mut netlink_ext_ack,
    pub tmplt_priv): *mut *mut void (tmplt_destroy)(void,
    pub cb_priv): *mut c_void,
    pub handle): u32,
// rtnetlink specific
    pub rtnl_held): *mut *mut tcmsg t, bool,
    pub tmplt_priv): *mut c_void,
    pub owner: *mut module,
    pub flags: c_int,
}

// Classifiers setting TCF_PROTO_OPS_DOIT_UNLOCKED in tcf_proto_ops->flags
// are expected to implement tcf_proto_ops->delete_empty(), otherwise race
// conditions can occur when filters are inserted/deleted simultaneously.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcf_proto_ops_flags {
    TCF_PROTO_OPS_DOIT_UNLOCKED = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_proto {
// Fast access part
    pub next: *mut tcf_proto __rcu,
    pub root: *mut void __rcu,
// called under RCU BH lock
    pub ): *mut tcf_result,
    pub protocol: __be16,
// All the rest
    pub prio: u32,
    pub data: *mut c_void,
    pub ops: *const tcf_proto_ops,
    pub chain: *mut tcf_chain,
// Lock protects tcf_proto shared state and can be used by unlocked
// classifiers to protect their private data.
//
    pub lock: spinlock_t,
    pub deleting: bool,
    pub counted: bool,
    pub usesw: bool,
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
    pub destroy_ht_node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_skb_cb {
    pub pkt_len: c_uint,
    pub pkt_segs: u16,
    pub tc_classid: u16,
pub const QDISC_CB_PRIV_LEN: c_int = 20;
    pub data: [c_uchar; QDISC_CB_PRIV_LEN],
    pub slave_dev_queue_mapping: u16,
    pub post_ct:1: u8,
    pub post_ct_snat:1: u8,
    pub post_ct_dnat:1: u8,
}

extern "C" {
    pub fn tcf_chain_head_change_t(tp_head: *mut tcf_proto, priv: *mut c_void) -> typedef void;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_chain {
// Protects filter_chain.
    pub filter_chain_lock: mutex,
    pub filter_chain: *mut tcf_proto __rcu,
    pub list: list_head,
    pub block: *mut tcf_block,
    pub /: *mut *mut u32 index; / chain index,
    pub refcnt: c_uint,
    pub action_refcnt: c_uint,
    pub explicitly_created: bool,
    pub flushing: bool,
    pub tmplt_ops: *const tcf_proto_ops,
    pub tmplt_priv: *mut c_void,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_block {
    pub /: *mut *mut xarray ports; / datapath accessible,
// Lock protects tcf_block and lifetime-management data of chains
// attached to the block (refcnt, action_refcnt, explicitly_created).
//
    pub lock: mutex,
    pub chain_list: list_head,
    pub /: *mut *mut u32 index; / block index for shared blocks,
    pub /: *mut *mut u32 classid; / which class this block belongs to,
    pub refcnt: refcount_t,
    pub net: *mut net,
    pub q: *mut Qdisc,
    pub /: *mut *mut rw_semaphore cb_lock; / protects cb_list and offload counters,
    pub flow_block: flow_block,
    pub owner_list: list_head,
    pub keep_dst: bool,
    pub useswcnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t offloadcnt; / Number of oddloaded filters,
    pub /: *mut *mut unsigned int nooffloaddevcnt; / Number of devs unable to do offload,
    pub /: *mut *mut unsigned int lockeddevcnt; / Number of devs that require rtnl lock.,
    pub chain: *mut tcf_chain,
    pub filter_chain_list: list_head,
    pub chain0: },
    pub rcu: rcu_head,
    pub 7): DECLARE_HASHTABLE(proto_destroy_ht,,
    pub /: *mut *mut mutex proto_destroy_lock; / Lock for proto_destroy hashtable.,
}

extern "C" {
    pub fn lockdep_is_held(_arg: &chain->filter_chain_lock) -> return;
}
extern "C" {
    pub fn lockdep_is_held(_arg: &tp->lock) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: q->q.qlen) -> return;
}
extern "C" {
    pub fn rcu_dereference_bh(_arg: qdisc->dev_queue->qdisc) -> return;
}
extern "C" {
    pub fn rcu_dereference_rtnl(_arg: qdisc->dev_queue->qdisc_sleeping) -> return;
}
extern "C" {
    pub fn qdisc_lock(_arg: root) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qdisc_class_common {
    pub classid: u32,
    pub filter_cnt: c_uint,
    pub hnode: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qdisc_class_hash {
    pub hash: *mut hlist_head,
    pub hashsize: c_uint,
    pub hashmask: c_uint,
    pub hashelems: c_uint,
}

extern "C" {
    pub fn qdisc_class_hash_init(: *mut Qdisc_class_hash) -> c_int;
}
extern "C" {
    pub fn qdisc_class_hash_grow(: *mut Qdisc, : *mut Qdisc_class_hash);
}
extern "C" {
    pub fn qdisc_class_hash_destroy(: *mut Qdisc_class_hash);
}
extern "C" {
    pub fn dev_qdisc_change_tx_queue_len(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_init_scheduler(dev: *mut net_device);
}
extern "C" {
    pub fn dev_shutdown(dev: *mut net_device);
}
extern "C" {
    pub fn dev_activate(dev: *mut net_device);
}
extern "C" {
    pub fn dev_deactivate(dev: *mut net_device, reset_needed: bool);
}
extern "C" {
    pub fn dev_deactivate_many(head: *mut list_head, reset_needed: bool);
}
extern "C" {
    pub fn qdisc_reset(qdisc: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_destroy(qdisc: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_put(qdisc: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_put_unlocked(qdisc: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_tree_reduce_backlog(qdisc: *mut Qdisc, n: c_int, len: c_int);
}

extern "C" {
    pub fn qdisc_free(qdisc: *mut Qdisc);
}
extern "C" {
    pub fn skb_do_redirect(: *mut sk_buff) -> c_int;
}

// Reset all TX qdiscs greater than index of a device.
// Are all TX queues of the device empty?
// Are any of the TX qdiscs changing?
// "noqueue" qdisc identified by not having any enqueue, see noqueue_init()
// Is the device using the noop qdisc on all queues?
// additional qdisc xmit flags (NET_XMIT_MASK in linux/netdevice.h)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_xmit_qdisc_t {
    __NET_XMIT_STOLEN = 0x00010000,
    __NET_XMIT_BYPASS = 0x00020000,
}

extern "C" {
    pub fn gnet_stats_copy_queue(_arg: d, _arg: sch->cpu_qstats, _arg: &sch->qstats, _arg: qlen) -> return;
}
// qlen = qstats.qlen + qdisc_qlen_lockless(sch);
// backlog = qstats.backlog;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_skb_cb {
    pub qdisc_cb: qdisc_skb_cb,
    pub drop_reason: u32,
    pub /: *mut *mut u16 zone; / Only valid if qdisc_skb_cb(skb)->post_ct = true,
    pub mru: u16,
}

// TC classifier accessors - use enum skb_drop_reason
// Qdisc accessors - use enum qdisc_drop_reason
// Instead of calling kfree_skb() while root qdisc lock is held,
// queue the skb for future freeing at end of __dev_xmit_skb()
//
// to_free = skb;
// generic pseudo peek method for non-work-conserving qdisc
// we can reuse ->gso_skb because peek isn't called for root qdiscs
// it's still part of the queue
// use instead of qdisc->dequeue() for all qdiscs queried with ->peek()
//
// We do not know the backlog in bytes of this list, it
// is up to the caller to correct it
//
// pold = new;
extern "C" {
    pub fn qdisc_drop(_arg: skb, _arg: sch, _arg: to_free) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psched_ratecfg {
    pub /: *mut *mut u64 rate_bytes_ps; / bytes per second,
    pub mult: u32,
    pub overhead: u16,
    pub mpu: u16,
    pub linklayer: u8,
    pub shift: u8,
}

// legacy struct tc_ratespec has a 32bit @rate field
// Qdisc using 64bit rate should add new attributes
// in order to maintain compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psched_pktrate {
    pub /: *mut *mut u64 rate_pkts_ps; / packets per second,
    pub mult: u32,
    pub shift: u8,
}

extern "C" {
    pub fn psched_ppscfg_precompute(r: *mut psched_pktrate, pktrate64: u64);
}
// Mini Qdisc serves for specific needs of ingress/clsact Qdisc.
// The fast path only needs to access filter list and to update stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mini_Qdisc {
    pub filter_list: *mut tcf_proto,
    pub block: *mut tcf_block,
    pub cpu_bstats: *mut gnet_stats_basic_sync __percpu,
    pub cpu_qstats: *mut gnet_stats_queue __percpu,
    pub rcu_state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mini_Qdisc_pair {
    pub miniq1: mini_Qdisc,
    pub miniq2: mini_Qdisc,
    pub p_miniq: *mut mini_Qdisc __rcu,
}

extern "C" {
    pub fn mq_change_real_num_tx(sch: *mut Qdisc, new_real_tx: c_uint);
}
extern "C" {
    pub fn sch_frag_xmit_hook(skb: *mut sk_buff, skb): *mut *mut int (xmit)(struct sk_buff) -> c_int;
}
// Make sure qdisc is no longer in SCHED state.
