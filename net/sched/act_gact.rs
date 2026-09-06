//! Automatically rewritten from C to Rust
//! Source: net/sched/act_gact.c
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
// net/sched/act_gact.c		Generic actions
//
// copyright 	Jamal Hadi Salim (2002-4)
//

    static struct tc_action_ops act_gact_ops;

#[no_mangle]
unsafe extern "C" fn gact_net_rand(gact: *mut tcf_gact) -> c_int {
    static int gact_net_rand(struct tcf_gact *gact)
    {
    smp_rmb(); /* coupled with smp_wmb() in tcf_gact_init() */
    if (get_random_u32_below(gact.tcfg_pval))
    return gact.tcf_action;
    return gact.tcfg_paction;
    }
#[no_mangle]
unsafe extern "C" fn gact_determ(gact: *mut tcf_gact) -> c_int {
    static int gact_determ(struct tcf_gact *gact)
    {
    let mut pack: u32 = atomic_inc_return(&gact.packets);
    smp_rmb(); /* coupled with smp_wmb() in tcf_gact_init() */
    if (pack % gact.tcfg_pval)
    return gact.tcf_action;
    return gact.tcfg_paction;
    }
    typedef int (*g_rand)(struct tcf_gact *gact);
    static g_rand gact_rand[MAX_RAND] = { core::ptr::null_mut(), gact_net_rand, gact_determ };

    static const struct nla_policy gact_policy[TCA_GACT_MAX + 1] = {
    [TCA_GACT_PARMS]	= { .len = sizeof(struct tc_gact) },
    [TCA_GACT_PROB]		= { .len = sizeof(struct tc_gact_p) },
    };
    static int tcf_gact_init(struct net *net, struct nlattr *nla,
    struct nlattr *est, struct tc_action **a,
    struct tcf_proto *tp, u32 flags,
    struct netlink_ext_ack *extack)
    {
    struct tc_action_net *tn = net_generic(net, act_gact_ops.net_id);
    let mut bind: bool = flags & TCA_ACT_FLAGS_BIND;
    struct nlattr *tb[TCA_GACT_MAX + 1];
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tc_gact *parm;
    struct tcf_gact *gact;
    let mut ret: c_int = 0;
    u32 index;
    int err;

    struct tc_gact_p *p_parm = core::ptr::null_mut();

    if (nla == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_GACT_MAX, nla, gact_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_GACT_PARMS] == core::ptr::null_mut())
    return -EINVAL;
    parm = nla_data(tb[TCA_GACT_PARMS]);
    index = parm.index;

    if (tb[TCA_GACT_PROB] != core::ptr::null_mut())
    return -EOPNOTSUPP;

    if (tb[TCA_GACT_PROB]) {
    p_parm = nla_data(tb[TCA_GACT_PROB]);
    if (p_parm.ptype >= MAX_RAND)
    return -EINVAL;
    if (!tcf_action_valid(p_parm.paction)) {
    NL_SET_ERR_MSG(extack,
    "invalid fallback control action");
    return -EINVAL;
    }
    if (TC_ACT_EXT_CMP(p_parm.paction, TC_ACT_GOTO_CHAIN)) {
    NL_SET_ERR_MSG(extack,
    "goto chain not allowed on fallback");
    return -EINVAL;
    }
    }

    err = tcf_idr_check_alloc(tn, &index, a, bind);
    if (!err) {
    ret = tcf_idr_create_from_flags(tn, index, est, a,
    &act_gact_ops, bind, flags);
    if (ret) {
    tcf_idr_cleanup(tn, index);
    return ret;
    }
    ret = ACT_P_CREATED;
    } else if (err > 0) {
    if (bind)/* dont override defaults */
    return ACT_P_BOUND;
    if (!(flags & TCA_ACT_FLAGS_REPLACE)) {
    tcf_idr_release(*a, bind);
    return -EEXIST;
    }
    } else {
    return err;
    }
    err = tcf_action_check_ctrlact(parm.action, tp, &goto_ch, extack);
    if (err < 0)
    goto release_idr;
    gact = to_gact(*a);
    spin_lock_bh(&gact.tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, parm.action, goto_ch);

    if (p_parm) {
    gact.tcfg_paction = p_parm.paction;
    gact.tcfg_pval    = max_t(u16, 1, p_parm.pval);
// Make sure tcfg_pval is written before tcfg_ptype
// coupled with smp_rmb() in gact_net_rand() & gact_determ()
//
    smp_wmb();
    gact.tcfg_ptype   = p_parm.ptype;
    }

    spin_unlock_bh(&gact.tcf_lock);
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    return ret;
    release_idr:
    tcf_idr_release(*a, bind);
    return err;
    }
    TC_INDIRECT_SCOPE int tcf_gact_act(struct sk_buff *skb,
    const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_gact *gact = to_gact(a);
    let mut action: c_int = READ_ONCE(gact.tcf_action);

    {
    let mut ptype: u32 = READ_ONCE(gact.tcfg_ptype);
    if (ptype)
    action = gact_rand[ptype](gact);
    }

    tcf_action_update_bstats(&gact.common, skb);
    if (action == TC_ACT_SHOT)
    tcf_action_inc_drop_qstats(&gact.common);
    tcf_lastuse_update(&gact.tcf_tm);
    return action;
    }
    static void tcf_gact_stats_update(struct tc_action *a, u64 bytes, u64 packets,
    u64 drops, u64 lastuse, bool hw)
    {
    struct tcf_gact *gact = to_gact(a);
    let mut action: c_int = READ_ONCE(gact.tcf_action);
    struct tcf_t *tm = &gact.tcf_tm;
    tcf_action_update_stats(a, bytes, packets,
    action == TC_ACT_SHOT ? packets : drops, hw);
    tm.lastuse = max_t(u64, tm.lastuse, lastuse);
    }
    static int tcf_gact_dump(struct sk_buff *skb, struct tc_action *a,
    int bind, int ref)
    {
    unsigned char *b = skb_tail_pointer(skb);
    struct tcf_gact *gact = to_gact(a);
    struct tc_gact opt = {
    .index   = gact.tcf_index,
    .refcnt  = refcount_read(&gact.tcf_refcnt) - ref,
    .bindcnt = atomic_read(&gact.tcf_bindcnt) - bind,
    };
    struct tcf_t t;
    spin_lock_bh(&gact.tcf_lock);
    opt.action = gact.tcf_action;
    if (nla_put(skb, TCA_GACT_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;

    if (gact.tcfg_ptype) {
    struct tc_gact_p p_opt = {
    .paction = gact.tcfg_paction,
    .pval    = gact.tcfg_pval,
    .ptype   = gact.tcfg_ptype,
    };
    if (nla_put(skb, TCA_GACT_PROB, sizeof(p_opt), &p_opt))
    goto nla_put_failure;
    }

    tcf_tm_dump(&t, &gact.tcf_tm);
    if (nla_put_64bit(skb, TCA_GACT_TM, sizeof(t), &t, TCA_GACT_PAD))
    goto nla_put_failure;
    spin_unlock_bh(&gact.tcf_lock);
    return skb.len;
    nla_put_failure:
    spin_unlock_bh(&gact.tcf_lock);
    nlmsg_trim(skb, b);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn tcf_gact_get_fill_size(act: *const tc_action) -> usize {
    static size_t tcf_gact_get_fill_size(const struct tc_action *act)
    {
    size_t sz = nla_total_size(sizeof(struct tc_gact)); /* TCA_GACT_PARMS */

    if (to_gact(act).tcfg_ptype)
// TCA_GACT_PROB
    sz += nla_total_size(sizeof(struct tc_gact_p));

    return sz;
    }
    static int tcf_gact_offload_act_setup(struct tc_action *act, void *entry_data,
    u32 *index_inc, bool bind,
    struct netlink_ext_ack *extack)
    {
    if (bind) {
    struct flow_action_entry *entry = entry_data;
    if (is_tcf_gact_ok(act)) {
    entry.id = FLOW_ACTION_ACCEPT;
    } else if (is_tcf_gact_shot(act)) {
    entry.id = FLOW_ACTION_DROP;
    } else if (is_tcf_gact_trap(act)) {
    entry.id = FLOW_ACTION_TRAP;
    } else if (is_tcf_gact_goto_chain(act)) {
    entry.id = FLOW_ACTION_GOTO;
    entry.chain_index = tcf_gact_goto_chain_index(act);
    } else if (is_tcf_gact_continue(act)) {
    NL_SET_ERR_MSG_MOD(extack, "Offload of \"continue\" action is not supported");
    return -EOPNOTSUPP;
    } else if (is_tcf_gact_reclassify(act)) {
    NL_SET_ERR_MSG_MOD(extack, "Offload of \"reclassify\" action is not supported");
    return -EOPNOTSUPP;
    } else if (is_tcf_gact_pipe(act)) {
    NL_SET_ERR_MSG_MOD(extack, "Offload of \"pipe\" action is not supported");
    return -EOPNOTSUPP;
    } else {
    NL_SET_ERR_MSG_MOD(extack, "Unsupported generic action offload");
    return -EOPNOTSUPP;
    }
// index_inc = 1;
    } else {
    struct flow_offload_action *fl_action = entry_data;
    if (is_tcf_gact_ok(act))
    fl_action.id = FLOW_ACTION_ACCEPT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_gact_shot(act)) -> else {
    else if (is_tcf_gact_shot(act))
    fl_action.id = FLOW_ACTION_DROP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_gact_trap(act)) -> else {
    else if (is_tcf_gact_trap(act))
    fl_action.id = FLOW_ACTION_TRAP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_tcf_gact_goto_chain(act)) -> else {
    else if (is_tcf_gact_goto_chain(act))
    fl_action.id = FLOW_ACTION_GOTO;
    else
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static struct tc_action_ops act_gact_ops = {
    .kind		=	"gact",
    .id		=	TCA_ID_GACT,
    .owner		=	THIS_MODULE,
    .act		=	tcf_gact_act,
    .stats_update	=	tcf_gact_stats_update,
    .dump		=	tcf_gact_dump,
    .init		=	tcf_gact_init,
    .get_fill_size	=	tcf_gact_get_fill_size,
    .offload_act_setup =	tcf_gact_offload_act_setup,
    .size		=	sizeof(struct tcf_gact),
    };
    MODULE_ALIAS_NET_ACT("gact");
#[no_mangle]
unsafe extern "C" fn gact_init_net(net: *mut net) -> __net_init int {
    static __net_init int gact_init_net(struct net *net)
    {
    struct tc_action_net *tn = net_generic(net, act_gact_ops.net_id);
    return tc_action_net_init(net, tn, &act_gact_ops);
    }
#[no_mangle]
unsafe extern "C" fn gact_exit_net(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit gact_exit_net(struct list_head *net_list)
    {
    tc_action_net_exit(net_list, act_gact_ops.net_id);
    }
    static struct pernet_operations gact_net_ops = {
    .init = gact_init_net,
    .exit_batch = gact_exit_net,
    .id   = &act_gact_ops.net_id,
    .size = sizeof(struct tc_action_net),
    };
    MODULE_AUTHOR("Jamal Hadi Salim(2002-4)");
    MODULE_DESCRIPTION("Generic Classifier actions");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn gact_init_module() -> int __init {
    static int __init gact_init_module(void)
    {

    pr_info("GACT probability on\n");

    pr_info("GACT probability NOT on\n");

    return tcf_register_action(&act_gact_ops, &gact_net_ops);
    }
#[no_mangle]
unsafe extern "C" fn gact_cleanup_module() -> void __exit {
    static void __exit gact_cleanup_module(void)
    {
    tcf_unregister_action(&act_gact_ops, &gact_net_ops);
    }
    module_init(gact_init_module);
    module_exit(gact_cleanup_module);
