//! Automatically rewritten from C to Rust
//! Source: net/sched/act_skbedit.c
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
// Copyright (c) 2008, Intel Corporation.
//
// Author: Alexander Duyck <alexander.h.duyck@intel.com>
//

    static struct tc_action_ops act_skbedit_ops;
    static u16 tcf_skbedit_hash(struct tcf_skbedit_params *params,
    struct sk_buff *skb)
    {
    let mut queue_mapping: u16 = params.queue_mapping;
    if (params.flags & SKBEDIT_F_TXQ_SKBHASH) {
    let mut hash: u32 = skb_get_hash(skb);
    queue_mapping += hash % params.mapping_mod;
    }
    return netdev_cap_txqueue(skb.dev, queue_mapping);
    }
    TC_INDIRECT_SCOPE int tcf_skbedit_act(struct sk_buff *skb,
    const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_skbedit *d = to_skbedit(a);
    struct tcf_skbedit_params *params;
    tcf_lastuse_update(&d.tcf_tm);
    bstats_update(this_cpu_ptr(d.common.cpu_bstats), skb);
    params = rcu_dereference_bh(d.params);
    if (params.flags & SKBEDIT_F_PRIORITY)
    skb.priority = params.priority;
    if (params.flags & SKBEDIT_F_INHERITDSFIELD) {
    let mut wlen: c_int = skb_network_offset(skb);
    switch (skb_protocol(skb, true)) {
    case htons(ETH_P_IP):
    wlen += sizeof(struct iphdr);
    if (!pskb_may_pull(skb, wlen))
    goto err;
    skb.priority = ipv4_get_dsfield(ip_hdr(skb)) >> 2;
    break;
    case htons(ETH_P_IPV6):
    wlen += sizeof(struct ipv6hdr);
    if (!pskb_may_pull(skb, wlen))
    goto err;
    skb.priority = ipv6_get_dsfield(ipv6_hdr(skb)) >> 2;
    break;
    }
    }
    if (params.flags & SKBEDIT_F_QUEUE_MAPPING &&
    skb.dev.real_num_tx_queues > params.queue_mapping) {

    netdev_xmit_skip_txqueue(true);

    skb_set_queue_mapping(skb, tcf_skbedit_hash(params, skb));
    }
    if (params.flags & SKBEDIT_F_MARK) {
    skb.mark &= ~params.mask;
    skb.mark |= params.mark & params.mask;
    }
    if (params.flags & SKBEDIT_F_PTYPE)
    skb.pkt_type = params.ptype;
    return params.action;
    err:
    qstats_cpu_drop_inc(d.common.cpu_qstats);
    return TC_ACT_SHOT;
    }
    static void tcf_skbedit_stats_update(struct tc_action *a, u64 bytes,
    u64 packets, u64 drops,
    u64 lastuse, bool hw)
    {
    struct tcf_skbedit *d = to_skbedit(a);
    struct tcf_t *tm = &d.tcf_tm;
    tcf_action_update_stats(a, bytes, packets, drops, hw);
    tm.lastuse = max_t(u64, tm.lastuse, lastuse);
    }
    static const struct nla_policy skbedit_policy[TCA_SKBEDIT_MAX + 1] = {
    [TCA_SKBEDIT_PARMS]		= { .len = sizeof(struct tc_skbedit) },
    [TCA_SKBEDIT_PRIORITY]		= { .len = sizeof(u32) },
    [TCA_SKBEDIT_QUEUE_MAPPING]	= { .len = sizeof(u16) },
    [TCA_SKBEDIT_MARK]		= { .len = sizeof(u32) },
    [TCA_SKBEDIT_PTYPE]		= { .len = sizeof(u16) },
    [TCA_SKBEDIT_MASK]		= { .len = sizeof(u32) },
    [TCA_SKBEDIT_FLAGS]		= { .len = sizeof(u64) },
    [TCA_SKBEDIT_QUEUE_MAPPING_MAX]	= { .len = sizeof(u16) },
    };
    static int tcf_skbedit_init(struct net *net, struct nlattr *nla,
    struct nlattr *est, struct tc_action **a,
    struct tcf_proto *tp, u32 act_flags,
    struct netlink_ext_ack *extack)
    {
    struct tc_action_net *tn = net_generic(net, act_skbedit_ops.net_id);
    let mut bind: bool = act_flags & TCA_ACT_FLAGS_BIND;
    struct tcf_skbedit_params *params_new;
    struct nlattr *tb[TCA_SKBEDIT_MAX + 1];
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tc_skbedit *parm;
    struct tcf_skbedit *d;
    let mut flags: u32 = 0, *priority = core::ptr::null_mut(), *mark = core::ptr::null_mut(), *mask = core::ptr::null_mut();
    u16 *queue_mapping = core::ptr::null_mut(), *ptype = core::ptr::null_mut();
    let mut mapping_mod: u32 = 1;
    let mut exists: bool = false;
    let mut ret: c_int = 0, err;
    u32 index;
    if (nla == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_SKBEDIT_MAX, nla,
    skbedit_policy, core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_SKBEDIT_PARMS] == core::ptr::null_mut())
    return -EINVAL;
    if (tb[TCA_SKBEDIT_PRIORITY] != core::ptr::null_mut()) {
    flags |= SKBEDIT_F_PRIORITY;
    priority = nla_data(tb[TCA_SKBEDIT_PRIORITY]);
    }
    if (tb[TCA_SKBEDIT_QUEUE_MAPPING] != core::ptr::null_mut()) {
    if (is_tcf_skbedit_ingress(act_flags) &&
    !(act_flags & TCA_ACT_FLAGS_SKIP_SW)) {
    NL_SET_ERR_MSG_MOD(extack, "\"queue_mapping\" option on receive side is hardware only, use skip_sw");
    return -EOPNOTSUPP;
    }
    flags |= SKBEDIT_F_QUEUE_MAPPING;
    queue_mapping = nla_data(tb[TCA_SKBEDIT_QUEUE_MAPPING]);
    }
    if (tb[TCA_SKBEDIT_PTYPE] != core::ptr::null_mut()) {
    ptype = nla_data(tb[TCA_SKBEDIT_PTYPE]);
    if (!skb_pkt_type_ok(*ptype))
    return -EINVAL;
    flags |= SKBEDIT_F_PTYPE;
    }
    if (tb[TCA_SKBEDIT_MARK] != core::ptr::null_mut()) {
    flags |= SKBEDIT_F_MARK;
    mark = nla_data(tb[TCA_SKBEDIT_MARK]);
    }
    if (tb[TCA_SKBEDIT_MASK] != core::ptr::null_mut()) {
    flags |= SKBEDIT_F_MASK;
    mask = nla_data(tb[TCA_SKBEDIT_MASK]);
    }
    if (tb[TCA_SKBEDIT_FLAGS] != core::ptr::null_mut()) {
    u64 *pure_flags = nla_data(tb[TCA_SKBEDIT_FLAGS]);
    if (*pure_flags & SKBEDIT_F_TXQ_SKBHASH) {
    u16 *queue_mapping_max;
    if (!tb[TCA_SKBEDIT_QUEUE_MAPPING] ||
    !tb[TCA_SKBEDIT_QUEUE_MAPPING_MAX]) {
    NL_SET_ERR_MSG_MOD(extack, "Missing required range of queue_mapping.");
    return -EINVAL;
    }
    queue_mapping_max =
    nla_data(tb[TCA_SKBEDIT_QUEUE_MAPPING_MAX]);
    if (*queue_mapping_max < *queue_mapping) {
    NL_SET_ERR_MSG_MOD(extack, "The range of queue_mapping is invalid, max < min.");
    return -EINVAL;
    }
    mapping_mod = *queue_mapping_max - *queue_mapping + 1;
    if (mapping_mod > U16_MAX) {
    NL_SET_ERR_MSG_MOD(extack, "The range of queue_mapping is invalid.");
    return -EINVAL;
    }
    flags |= SKBEDIT_F_TXQ_SKBHASH;
    }
    if (*pure_flags & SKBEDIT_F_INHERITDSFIELD)
    flags |= SKBEDIT_F_INHERITDSFIELD;
    }
    parm = nla_data(tb[TCA_SKBEDIT_PARMS]);
    index = parm.index;
    err = tcf_idr_check_alloc(tn, &index, a, bind);
    if (err < 0)
    return err;
    exists = err;
    if (exists && bind)
    return ACT_P_BOUND;
    if (!flags) {
    if (exists)
    tcf_idr_release(*a, bind);
    else
    tcf_idr_cleanup(tn, index);
    return -EINVAL;
    }
    if (!exists) {
    ret = tcf_idr_create(tn, index, est, a,
    &act_skbedit_ops, bind, true, act_flags);
    if (ret) {
    tcf_idr_cleanup(tn, index);
    return ret;
    }
    d = to_skbedit(*a);
    ret = ACT_P_CREATED;
    } else {
    d = to_skbedit(*a);
    if (!(act_flags & TCA_ACT_FLAGS_REPLACE)) {
    tcf_idr_release(*a, bind);
    return -EEXIST;
    }
    }
    err = tcf_action_check_ctrlact(parm.action, tp, &goto_ch, extack);
    if (err < 0)
    goto release_idr;
    params_new = kzalloc_obj(*params_new);
    if (unlikely(!params_new)) {
    err = -ENOMEM;
    goto put_chain;
    }
    params_new.flags = flags;
    if (flags & SKBEDIT_F_PRIORITY)
    params_new.priority = *priority;
    if (flags & SKBEDIT_F_QUEUE_MAPPING) {
    params_new.queue_mapping = *queue_mapping;
    params_new.mapping_mod = mapping_mod;
    }
    if (flags & SKBEDIT_F_MARK)
    params_new.mark = *mark;
    if (flags & SKBEDIT_F_PTYPE)
    params_new.ptype = *ptype;
// default behaviour is to use all the bits
    params_new.mask = 0xffffffff;
    if (flags & SKBEDIT_F_MASK)
    params_new.mask = *mask;
    params_new.action = parm.action;
    spin_lock_bh(&d.tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, parm.action, goto_ch);
    params_new = rcu_replace_pointer(d.params, params_new,
    lockdep_is_held(&d.tcf_lock));
    spin_unlock_bh(&d.tcf_lock);
    if (params_new)
    kfree_rcu(params_new, rcu);
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    return ret;
    put_chain:
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    release_idr:
    tcf_idr_release(*a, bind);
    return err;
    }
    static int tcf_skbedit_dump(struct sk_buff *skb, struct tc_action *a,
    int bind, int ref)
    {
    const struct tcf_skbedit *d = to_skbedit(a);
    unsigned char *b = skb_tail_pointer(skb);
    const struct tcf_skbedit_params *params;
    struct tc_skbedit opt = {
    .index   = d.tcf_index,
    .refcnt  = refcount_read(&d.tcf_refcnt) - ref,
    .bindcnt = atomic_read(&d.tcf_bindcnt) - bind,
    };
    let mut pure_flags: u64 = 0;
    struct tcf_t t;
    rcu_read_lock();
    params = rcu_dereference(d.params);
    opt.action = params.action;
    if (nla_put(skb, TCA_SKBEDIT_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;
    if ((params.flags & SKBEDIT_F_PRIORITY) &&
    nla_put_u32(skb, TCA_SKBEDIT_PRIORITY, params.priority))
    goto nla_put_failure;
    if ((params.flags & SKBEDIT_F_QUEUE_MAPPING) &&
    nla_put_u16(skb, TCA_SKBEDIT_QUEUE_MAPPING, params.queue_mapping))
    goto nla_put_failure;
    if ((params.flags & SKBEDIT_F_MARK) &&
    nla_put_u32(skb, TCA_SKBEDIT_MARK, params.mark))
    goto nla_put_failure;
    if ((params.flags & SKBEDIT_F_PTYPE) &&
    nla_put_u16(skb, TCA_SKBEDIT_PTYPE, params.ptype))
    goto nla_put_failure;
    if ((params.flags & SKBEDIT_F_MASK) &&
    nla_put_u32(skb, TCA_SKBEDIT_MASK, params.mask))
    goto nla_put_failure;
    if (params.flags & SKBEDIT_F_INHERITDSFIELD)
    pure_flags |= SKBEDIT_F_INHERITDSFIELD;
    if (params.flags & SKBEDIT_F_TXQ_SKBHASH) {
    if (nla_put_u16(skb, TCA_SKBEDIT_QUEUE_MAPPING_MAX,
    params.queue_mapping + params.mapping_mod - 1))
    goto nla_put_failure;
    pure_flags |= SKBEDIT_F_TXQ_SKBHASH;
    }
    if (pure_flags != 0 &&
    nla_put(skb, TCA_SKBEDIT_FLAGS, sizeof(pure_flags), &pure_flags))
    goto nla_put_failure;
    tcf_tm_dump(&t, &d.tcf_tm);
    if (nla_put_64bit(skb, TCA_SKBEDIT_TM, sizeof(t), &t, TCA_SKBEDIT_PAD))
    goto nla_put_failure;
    rcu_read_unlock();
    return skb.len;
    nla_put_failure:
    rcu_read_unlock();
    nlmsg_trim(skb, b);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn tcf_skbedit_cleanup(a: *mut tc_action) {
    static void tcf_skbedit_cleanup(struct tc_action *a)
    {
    struct tcf_skbedit *d = to_skbedit(a);
    struct tcf_skbedit_params *params;
    params = rcu_dereference_protected(d.params, 1);
    if (params)
    kfree_rcu(params, rcu);
    }
#[no_mangle]
unsafe extern "C" fn tcf_skbedit_get_fill_size(act: *const tc_action) -> usize {
    static size_t tcf_skbedit_get_fill_size(const struct tc_action *act)
    {
#[no_mangle]
pub unsafe extern "C" fn nla_total_size(tc_skbedit): sizeof(struct) -> return {
    return nla_total_size(sizeof(struct tc_skbedit))
    + nla_total_size(sizeof(u32)) /* TCA_SKBEDIT_PRIORITY */
    + nla_total_size(sizeof(u16)) /* TCA_SKBEDIT_QUEUE_MAPPING */
    + nla_total_size(sizeof(u16)) /* TCA_SKBEDIT_QUEUE_MAPPING_MAX */
    + nla_total_size(sizeof(u32)) /* TCA_SKBEDIT_MARK */
    + nla_total_size(sizeof(u16)) /* TCA_SKBEDIT_PTYPE */
    + nla_total_size(sizeof(u32)) /* TCA_SKBEDIT_MASK */
    + nla_total_size_64bit(sizeof(u64)); /* TCA_SKBEDIT_FLAGS */
    }
    static int tcf_skbedit_offload_act_setup(struct tc_action *act, void *entry_data,
    u32 *index_inc, bool bind,
    struct netlink_ext_ack *extack)
    {
    if (bind) {
    struct flow_action_entry *entry = entry_data;
    if (is_tcf_skbedit_mark(act)) {
    entry.id = FLOW_ACTION_MARK;
    entry.mark = tcf_skbedit_mark(act);
    } else if (is_tcf_skbedit_ptype(act)) {
    entry.id = FLOW_ACTION_PTYPE;
    entry.ptype = tcf_skbedit_ptype(act);
    } else if (is_tcf_skbedit_priority(act)) {
    entry.id = FLOW_ACTION_PRIORITY;
    entry.priority = tcf_skbedit_priority(act);
    } else if (is_tcf_skbedit_tx_queue_mapping(act)) {
    NL_SET_ERR_MSG_MOD(extack, "Offload not supported when \"queue_mapping\" option is used on transmit side");
    return -EOPNOTSUPP;
    } else if (is_tcf_skbedit_rx_queue_mapping(act)) {
    entry.id = FLOW_ACTION_RX_QUEUE_MAPPING;
    entry.rx_queue = tcf_skbedit_rx_queue_mapping(act);
    } else if (is_tcf_skbedit_inheritdsfield(act)) {
    NL_SET_ERR_MSG_MOD(extack, "Offload not supported when \"inheritdsfield\" option is used");
    return -EOPNOTSUPP;
    } else {
    NL_SET_ERR_MSG_MOD(extack, "Unsupported skbedit option offload");
    return -EOPNOTSUPP;
    }
// index_inc = 1;
    } else {
    struct flow_offload_action *fl_action = entry_data;
    if (is_tcf_skbedit_mark(act))
    fl_action.id = FLOW_ACTION_MARK;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_skbedit_ptype(act)) -> else {
    else if (is_tcf_skbedit_ptype(act))
    fl_action.id = FLOW_ACTION_PTYPE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_skbedit_priority(act)) -> else {
    else if (is_tcf_skbedit_priority(act))
    fl_action.id = FLOW_ACTION_PRIORITY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_skbedit_rx_queue_mapping(act)) -> else {
    else if (is_tcf_skbedit_rx_queue_mapping(act))
    fl_action.id = FLOW_ACTION_RX_QUEUE_MAPPING;
    else
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static struct tc_action_ops act_skbedit_ops = {
    .kind		=	"skbedit",
    .id		=	TCA_ID_SKBEDIT,
    .owner		=	THIS_MODULE,
    .act		=	tcf_skbedit_act,
    .stats_update	=	tcf_skbedit_stats_update,
    .dump		=	tcf_skbedit_dump,
    .init		=	tcf_skbedit_init,
    .cleanup	=	tcf_skbedit_cleanup,
    .get_fill_size	=	tcf_skbedit_get_fill_size,
    .offload_act_setup =	tcf_skbedit_offload_act_setup,
    .size		=	sizeof(struct tcf_skbedit),
    };
    MODULE_ALIAS_NET_ACT("skbedit");
#[no_mangle]
unsafe extern "C" fn skbedit_init_net(net: *mut net) -> __net_init int {
    static __net_init int skbedit_init_net(struct net *net)
    {
    struct tc_action_net *tn = net_generic(net, act_skbedit_ops.net_id);
    return tc_action_net_init(net, tn, &act_skbedit_ops);
    }
#[no_mangle]
unsafe extern "C" fn skbedit_exit_net(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit skbedit_exit_net(struct list_head *net_list)
    {
    tc_action_net_exit(net_list, act_skbedit_ops.net_id);
    }
    static struct pernet_operations skbedit_net_ops = {
    .init = skbedit_init_net,
    .exit_batch = skbedit_exit_net,
    .id   = &act_skbedit_ops.net_id,
    .size = sizeof(struct tc_action_net),
    };
    MODULE_AUTHOR("Alexander Duyck, <alexander.h.duyck@intel.com>");
    MODULE_DESCRIPTION("SKB Editing");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn skbedit_init_module() -> int __init {
    static int __init skbedit_init_module(void)
    {
    return tcf_register_action(&act_skbedit_ops, &skbedit_net_ops);
    }
#[no_mangle]
unsafe extern "C" fn skbedit_cleanup_module() -> void __exit {
    static void __exit skbedit_cleanup_module(void)
    {
    tcf_unregister_action(&act_skbedit_ops, &skbedit_net_ops);
    }
    module_init(skbedit_init_module);
    module_exit(skbedit_cleanup_module);
