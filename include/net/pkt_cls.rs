//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/pkt_cls.h
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

// TC action not accessible from user space

// Basic packet classifier frontend definitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_walker {
    pub stop: c_int,
    pub skip: c_int,
    pub count: c_int,
    pub nonempty: bool,
    pub cookie: c_ulong,
    pub ): *mut *mut *mut *mut int (fn)(struct tcf_proto , void node, struct tcf_walker,
}

extern "C" {
    pub fn register_tcf_proto_ops(ops: *mut tcf_proto_ops) -> c_int;
}
extern "C" {
    pub fn unregister_tcf_proto_ops(ops: *mut tcf_proto_ops);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_block_ext_info {
    pub binder_type: flow_block_binder_type,
    pub chain_head_change: *mut tcf_chain_head_change_t,
    pub chain_head_change_priv: *mut c_void,
    pub block_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_qevent {
    pub block: *mut tcf_block,
    pub info: tcf_block_ext_info,
    pub filter_chain: *mut tcf_proto __rcu,
}

extern "C" {
    pub fn tcf_queue_work(rwork: *mut rcu_work, func: work_func_t) -> bool;
}

extern "C" {
    pub fn tcf_chain_put_by_act(chain: *mut tcf_chain);
}
extern "C" {
    pub fn tcf_block_netif_keep_dst(block: *mut tcf_block);
}
extern "C" {
    pub fn tcf_block_put(block: *mut tcf_block);
}

// TC_ACT_REDIRECT from qdisc filter chains is not supported.
// Use BPF via tcx or mirred redirect instead.
//
extern "C" {
    pub fn xchg(_arg: clp, _arg: cl) -> return;
}
// Check q as it is not set for shared blocks. In that case,
// setting class is not supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_exts {

    pub /: *mut *mut __u32 type; / for backward compat(TCA_OLD_COMPAT),
    pub nr_actions: c_int,
    pub actions: *mut tc_action,
    pub net: *mut net,
    pub ns_tracker: netns_tracker,
    pub miss_cookie_node: *mut tcf_exts_miss_cookie_node,

// Map to export classifier specific extension TLV types to the
// generic extensions API. Unsupported extensions must be set to 0.
//
    pub action: c_int,
    pub police: c_int,
}

extern "C" {
    pub fn tcf_exts_init_ex(_arg: exts, _arg: net, _arg: action, _arg: police, _arg: NULL, _arg: 0, _arg: false) -> return;
}

// Return false if the netns is being destroyed in cleanup_net(). Callers
// need to do cleanup synchronously in this case, otherwise may race with
// tc_action_net_exit(). Return true for other cases.
//

//
// tcf_exts_has_actions - check if at least one action is present
// @exts: tc filter extensions handle
//
// Returns: true if at least one action is present.
//

//
// tcf_exts_exec - execute tc filter extensions
// @skb: socket buffer
// @exts: tc filter extensions handle
// @res: desired result
//
// Executes all configured extensions. Returns TC_ACT_OK on a normal execution,
// a negative number if the filter must be considered unmatched or
// a positive action code (TC_ACT_*) which must be returned to the
// underlying layer.
//

extern "C" {
    pub fn tcf_action_exec(_arg: skb, _arg: exts->actions, _arg: exts->nr_actions, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_exts_destroy(exts: *mut tcf_exts);
}
extern "C" {
    pub fn tcf_exts_change(dst: *mut tcf_exts, src: *mut tcf_exts);
}
extern "C" {
    pub fn tcf_exts_dump(skb: *mut sk_buff, exts: *mut tcf_exts) -> c_int;
}
extern "C" {
    pub fn tcf_exts_terse_dump(skb: *mut sk_buff, exts: *mut tcf_exts) -> c_int;
}
extern "C" {
    pub fn tcf_exts_dump_stats(skb: *mut sk_buff, exts: *mut tcf_exts) -> c_int;
}
//
// struct tcf_pkt_info - packet information
//
// @ptr: start of the pkt data
// @nexthdr: offset of the next header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_pkt_info {
    pub ptr: *mut *mut c_uchar,
    pub nexthdr: c_int,
}

//
// struct tcf_ematch - extended match (ematch)
//
// @matchid: identifier to allow userspace to reidentify a match
// @flags: flags specifying attributes and the relation to other matches
// @ops: the operations lookup table of the corresponding ematch module
// @datalen: length of the ematch specific configuration data
// @data: ematch specific data
// @net: the network namespace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch {
    pub ops: *mut *mut tcf_ematch_ops,
    pub data: c_ulong,
    pub datalen: c_uint,
    pub matchid: u16,
    pub flags: u16,
    pub net: *mut net,
}

//
// struct tcf_ematch_tree - ematch tree handle
//
// @hdr: ematch tree header supplied by userspace
// @matches: array of ematches
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch_tree {
    pub hdr: tcf_ematch_tree_hdr,
    pub matches: *mut *mut tcf_ematch,
}

//
// struct tcf_ematch_ops - ematch module operations
//
// @kind: identifier (kind) of this ematch module
// @datalen: length of expected configuration data (optional)
// @change: called during validation (optional)
// @match: called during ematch tree evaluation, must return 1/0
// @destroy: called during destroyage (optional)
// @dump: called during dumping process (optional)
// @owner: owner, must be set to THIS_MODULE
// @link: link to previous/next ematch module (internal use)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch_ops {
    pub kind: c_int,
    pub datalen: c_int,
    pub ): *mut int, struct tcf_ematch,
    pub ): *mut tcf_pkt_info,
    pub ): *mut *mut void (destroy)(struct tcf_ematch,
    pub ): *mut *mut *mut int (dump)(struct sk_buff , struct tcf_ematch,
    pub owner: *mut module,
    pub link: list_head,
}

extern "C" {
    pub fn tcf_em_register(: *mut tcf_ematch_ops) -> c_int;
}
extern "C" {
    pub fn tcf_em_unregister(: *mut tcf_ematch_ops);
}
extern "C" {
    pub fn tcf_em_tree_destroy(: *mut tcf_ematch_tree);
}
extern "C" {
    pub fn tcf_em_tree_dump(: *mut sk_buff, : *mut tcf_ematch_tree, _arg: c_int) -> c_int;
}
//
// tcf_em_tree_match - evaluate an ematch tree
//
// @skb: socket buffer of the packet in question
// @tree: ematch tree to be used for evaluation
// @info: packet information examined by classifier
//
// This function matches @skb against the ematch tree in @tree by going
// through all ematches respecting their logic relations returning
// as soon as the result is obvious.
//
// Returns: 1 if the ematch tree as-one matches, no ematches are configured
// or ematch is not enabled in the kernel, otherwise 0 is returned.
//
extern "C" {
    pub fn __tcf_em_tree_match(_arg: skb, _arg: tree, _arg: info) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ematch_tree {
}

extern "C" {
    pub fn skb_mac_header(_arg: skb) -> return;
}
extern "C" {
    pub fn skb_network_header(_arg: skb) -> return;
}
extern "C" {
    pub fn skb_transport_header(_arg: skb) -> return;
}
extern "C" {
    pub fn tc_cleanup_offload_action(flow_action: *mut flow_action);
}
extern "C" {
    pub fn tcf_exts_num_actions(exts: *mut tcf_exts) -> c_uint;
}

extern "C" {
    pub fn tcf_qevent_destroy(qe: *mut tcf_qevent, sch: *mut Qdisc);
}
extern "C" {
    pub fn tcf_qevent_dump(skb: *mut sk_buff, attr_name: c_int, qe: *mut tcf_qevent) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cls_u32_knode {
    pub exts: *mut tcf_exts,
    pub res: *mut tcf_result,
    pub sel: *mut tc_u32_sel,
    pub handle: u32,
    pub val: u32,
    pub mask: u32,
    pub link_handle: u32,
    pub fshift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cls_u32_hnode {
    pub handle: u32,
    pub prio: u32,
    pub divisor: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_clsu32_command {
    TC_CLSU32_NEW_KNODE,
    TC_CLSU32_REPLACE_KNODE,
    TC_CLSU32_DELETE_KNODE,
    TC_CLSU32_NEW_HNODE,
    TC_CLSU32_REPLACE_HNODE,
    TC_CLSU32_DELETE_HNODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cls_u32_offload {
    pub common: flow_cls_common_offload,
// knode values
    pub command: tc_clsu32_command,
    pub knode: tc_cls_u32_knode,
    pub hnode: tc_cls_u32_hnode,
}

// SKIP_HW and SKIP_SW are mutually exclusive flags.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_matchall_command {
    TC_CLSMATCHALL_REPLACE,
    TC_CLSMATCHALL_DESTROY,
    TC_CLSMATCHALL_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cls_matchall_offload {
    pub common: flow_cls_common_offload,
    pub command: tc_matchall_command,
    pub rule: *mut flow_rule,
    pub stats: flow_stats,
    pub use_act_stats: bool,
    pub cookie: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_clsbpf_command {
    TC_CLSBPF_OFFLOAD,
    TC_CLSBPF_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cls_bpf_offload {
    pub common: flow_cls_common_offload,
    pub command: tc_clsbpf_command,
    pub exts: *mut tcf_exts,
    pub prog: *mut bpf_prog,
    pub oldprog: *mut bpf_prog,
    pub name: *const c_char,
    pub exts_integrated: bool,
}

// This structure holds cookie structure that is passed from user
// to the kernel for actions and classifiers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cookie {
    pub data: *mut u8,
    pub len: u32,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_qopt_offload_stats {
    pub bstats: *mut gnet_stats_basic_sync,
    pub qstats: *mut gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_mq_command {
    TC_MQ_CREATE,
    TC_MQ_DESTROY,
    TC_MQ_STATS,
    TC_MQ_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_mq_opt_offload_graft_params {
    pub queue: c_ulong,
    pub child_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_mq_qopt_offload {
    pub command: tc_mq_command,
    pub handle: u32,
    pub stats: tc_qopt_offload_stats,
    pub graft_params: tc_mq_opt_offload_graft_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_htb_command {
// Root
    TC_HTB_CREATE, /* Initialize HTB offload. */
    TC_HTB_DESTROY, /* Destroy HTB offload. */

// Classes
// Allocate qid and create leaf.
    TC_HTB_LEAF_ALLOC_QUEUE,
// Convert leaf to inner, preserve and return qid, create new leaf.
    TC_HTB_LEAF_TO_INNER,
// Delete leaf, while siblings remain.
    TC_HTB_LEAF_DEL,
// Delete leaf, convert parent to leaf, preserving qid.
    TC_HTB_LEAF_DEL_LAST,
// TC_HTB_LEAF_DEL_LAST, but delete driver data on hardware errors.
    TC_HTB_LEAF_DEL_LAST_FORCE,
// Modify parameters of a node.
    TC_HTB_NODE_MODIFY,

// Class qdisc
    TC_HTB_LEAF_QUERY_QUEUE, /* Query qid by classid. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_htb_qopt_offload {
    pub extack: *mut netlink_ext_ack,
    pub command: tc_htb_command,
    pub parent_classid: u32,
    pub classid: u16,
    pub qid: u16,
    pub quantum: u32,
    pub rate: u64,
    pub ceil: u64,
    pub prio: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_red_command {
    TC_RED_REPLACE,
    TC_RED_DESTROY,
    TC_RED_STATS,
    TC_RED_XSTATS,
    TC_RED_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_red_qopt_offload_params {
    pub min: u32,
    pub max: u32,
    pub probability: u32,
    pub limit: u32,
    pub is_ecn: bool,
    pub is_harddrop: bool,
    pub is_nodrop: bool,
    pub qstats: *mut gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_red_qopt_offload {
    pub command: tc_red_command,
    pub handle: u32,
    pub parent: u32,
    pub set: tc_red_qopt_offload_params,
    pub stats: tc_qopt_offload_stats,
    pub xstats: *mut red_stats,
    pub child_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_gred_command {
    TC_GRED_REPLACE,
    TC_GRED_DESTROY,
    TC_GRED_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_vq_qopt_offload_params {
    pub present: bool,
    pub limit: u32,
    pub prio: u32,
    pub min: u32,
    pub max: u32,
    pub is_ecn: bool,
    pub is_harddrop: bool,
    pub probability: u32,
// Only need backlog, see struct tc_prio_qopt_offload_params
    pub backlog: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_qopt_offload_params {
    pub grio_on: bool,
    pub wred_on: bool,
    pub dp_cnt: c_uint,
    pub dp_def: c_uint,
    pub qstats: *mut gnet_stats_queue,
    pub tab: [tc_gred_vq_qopt_offload_params; MAX_DPs],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_qopt_offload_stats {
    pub bstats: [gnet_stats_basic_sync; MAX_DPs],
    pub qstats: [gnet_stats_queue; MAX_DPs],
    pub xstats: [*mut red_stats; MAX_DPs],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_gred_qopt_offload {
    pub command: tc_gred_command,
    pub handle: u32,
    pub parent: u32,
    pub set: tc_gred_qopt_offload_params,
    pub stats: tc_gred_qopt_offload_stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_prio_command {
    TC_PRIO_REPLACE,
    TC_PRIO_DESTROY,
    TC_PRIO_STATS,
    TC_PRIO_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_prio_qopt_offload_params {
    pub bands: c_int,
    pub 1]: u8 priomap[TC_PRIO_MAX +,
// At the point of un-offloading the Qdisc, the reported backlog and
// qlen need to be reduced by the portion that is in HW.
//
    pub qstats: *mut gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_prio_qopt_offload_graft_params {
    pub band: u8,
    pub child_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_prio_qopt_offload {
    pub command: tc_prio_command,
    pub handle: u32,
    pub parent: u32,
    pub replace_params: tc_prio_qopt_offload_params,
    pub stats: tc_qopt_offload_stats,
    pub graft_params: tc_prio_qopt_offload_graft_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_root_command {
    TC_ROOT_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_root_qopt_offload {
    pub command: tc_root_command,
    pub handle: u32,
    pub ingress: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_ets_command {
    TC_ETS_REPLACE,
    TC_ETS_DESTROY,
    TC_ETS_STATS,
    TC_ETS_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_ets_qopt_offload_replace_params {
    pub bands: c_uint,
    pub 1]: u8 priomap[TC_PRIO_MAX +,
    pub /: *mut *mut unsigned int quanta[TCQ_ETS_MAX_BANDS]; / 0 for strict bands.,
    pub weights: [c_uint; TCQ_ETS_MAX_BANDS],
    pub qstats: *mut gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_ets_qopt_offload_graft_params {
    pub band: u8,
    pub child_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_ets_qopt_offload {
    pub command: tc_ets_command,
    pub handle: u32,
    pub parent: u32,
    pub replace_params: tc_ets_qopt_offload_replace_params,
    pub stats: tc_qopt_offload_stats,
    pub graft_params: tc_ets_qopt_offload_graft_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_tbf_command {
    TC_TBF_REPLACE,
    TC_TBF_DESTROY,
    TC_TBF_STATS,
    TC_TBF_GRAFT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_tbf_qopt_offload_replace_params {
    pub rate: psched_ratecfg,
    pub max_size: u32,
    pub qstats: *mut gnet_stats_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_tbf_qopt_offload {
    pub extack: *mut netlink_ext_ack,
    pub command: tc_tbf_command,
    pub handle: u32,
    pub parent: u32,
    pub replace_params: tc_tbf_qopt_offload_replace_params,
    pub stats: tc_qopt_offload_stats,
    pub child_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_fifo_command {
    TC_FIFO_REPLACE,
    TC_FIFO_DESTROY,
    TC_FIFO_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_fifo_qopt_offload {
    pub command: tc_fifo_command,
    pub handle: u32,
    pub parent: u32,
    pub stats: tc_qopt_offload_stats,
}

extern "C" {
    pub fn tc_skb_ext_tc_enable();
}
extern "C" {
    pub fn tc_skb_ext_tc_disable();
}

