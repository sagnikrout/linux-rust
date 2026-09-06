//! Automatically rewritten from C to Rust
//! Source: net/sched/act_simple.c
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
// net/sched/act_simple.c	Simple example of an action
//
// Authors:	Jamal Hadi Salim (2005-8)
//

    static struct tc_action_ops act_simp_ops;
pub const SIMP_MAX_DATA: c_int = 32;
    TC_INDIRECT_SCOPE int tcf_simp_act(struct sk_buff *skb,
    const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_defact *d = to_defact(a);
    spin_lock(&d.tcf_lock);
    tcf_lastuse_update(&d.tcf_tm);
    bstats_update(&d.tcf_bstats, skb);
// print policy string followed by _ then packet count
// Example if this was the 3rd packet and the string was "hello"
// then it would look like "hello_3" (without quotes)
//
    pr_info("simple: %s_%llu\n",
    (char *)d.tcfd_defdata,
    u64_stats_read(&d.tcf_bstats.packets));
    spin_unlock(&d.tcf_lock);
    return d.tcf_action;
    }
#[no_mangle]
unsafe extern "C" fn tcf_simp_release(a: *mut tc_action) {
    static void tcf_simp_release(struct tc_action *a)
    {
    struct tcf_defact *d = to_defact(a);
    kfree(d.tcfd_defdata);
    }
#[no_mangle]
unsafe extern "C" fn alloc_defdata(d: *mut tcf_defact, defdata: *const nlattr) -> c_int {
    static int alloc_defdata(struct tcf_defact *d, const struct nlattr *defdata)
    {
    d.tcfd_defdata = kzalloc(SIMP_MAX_DATA, GFP_KERNEL);
    if (unlikely(!d.tcfd_defdata))
    return -ENOMEM;
    nla_strscpy(d.tcfd_defdata, defdata, SIMP_MAX_DATA);
    return 0;
    }
    static int reset_policy(struct tc_action *a, const struct nlattr *defdata,
    struct tc_defact *p, struct tcf_proto *tp,
    struct netlink_ext_ack *extack)
    {
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tcf_defact *d;
    int err;
    err = tcf_action_check_ctrlact(p.action, tp, &goto_ch, extack);
    if (err < 0)
    return err;
    d = to_defact(a);
    spin_lock_bh(&d.tcf_lock);
    goto_ch = tcf_action_set_ctrlact(a, p.action, goto_ch);
    nla_strscpy(d.tcfd_defdata, defdata, SIMP_MAX_DATA);
    spin_unlock_bh(&d.tcf_lock);
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    return 0;
    }
    static const struct nla_policy simple_policy[TCA_DEF_MAX + 1] = {
    [TCA_DEF_PARMS]	= { .len = sizeof(struct tc_defact) },
    [TCA_DEF_DATA]	= { .type = NLA_STRING, .len = SIMP_MAX_DATA },
    };
    static int tcf_simp_init(struct net *net, struct nlattr *nla,
    struct nlattr *est, struct tc_action **a,
    struct tcf_proto *tp, u32 flags,
    struct netlink_ext_ack *extack)
    {
    struct tc_action_net *tn = net_generic(net, act_simp_ops.net_id);
    let mut bind: bool = flags & TCA_ACT_FLAGS_BIND;
    struct nlattr *tb[TCA_DEF_MAX + 1];
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tc_defact *parm;
    struct tcf_defact *d;
    let mut exists: bool = false;
    let mut ret: c_int = 0, err;
    u32 index;
    if (nla == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_DEF_MAX, nla, simple_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_DEF_PARMS] == core::ptr::null_mut())
    return -EINVAL;
    parm = nla_data(tb[TCA_DEF_PARMS]);
    index = parm.index;
    err = tcf_idr_check_alloc(tn, &index, a, bind);
    if (err < 0)
    return err;
    exists = err;
    if (exists && bind)
    return ACT_P_BOUND;
    if (tb[TCA_DEF_DATA] == core::ptr::null_mut()) {
    if (exists)
    tcf_idr_release(*a, bind);
    else
    tcf_idr_cleanup(tn, index);
    return -EINVAL;
    }
    if (!exists) {
    ret = tcf_idr_create(tn, index, est, a,
    &act_simp_ops, bind, false, flags);
    if (ret) {
    tcf_idr_cleanup(tn, index);
    return ret;
    }
    d = to_defact(*a);
    err = tcf_action_check_ctrlact(parm.action, tp, &goto_ch,
    extack);
    if (err < 0)
    goto release_idr;
    err = alloc_defdata(d, tb[TCA_DEF_DATA]);
    if (err < 0)
    goto put_chain;
    tcf_action_set_ctrlact(*a, parm.action, goto_ch);
    ret = ACT_P_CREATED;
    } else {
    if (!(flags & TCA_ACT_FLAGS_REPLACE)) {
    err = -EEXIST;
    goto release_idr;
    }
    err = reset_policy(*a, tb[TCA_DEF_DATA], parm, tp, extack);
    if (err)
    goto release_idr;
    }
    return ret;
    put_chain:
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    release_idr:
    tcf_idr_release(*a, bind);
    return err;
    }
    static int tcf_simp_dump(struct sk_buff *skb, struct tc_action *a,
    int bind, int ref)
    {
    unsigned char *b = skb_tail_pointer(skb);
    struct tcf_defact *d = to_defact(a);
    struct tc_defact opt = {
    .index   = d.tcf_index,
    .refcnt  = refcount_read(&d.tcf_refcnt) - ref,
    .bindcnt = atomic_read(&d.tcf_bindcnt) - bind,
    };
    struct tcf_t t;
    spin_lock_bh(&d.tcf_lock);
    opt.action = d.tcf_action;
    if (nla_put(skb, TCA_DEF_PARMS, sizeof(opt), &opt) ||
    nla_put_string(skb, TCA_DEF_DATA, d.tcfd_defdata))
    goto nla_put_failure;
    tcf_tm_dump(&t, &d.tcf_tm);
    if (nla_put_64bit(skb, TCA_DEF_TM, sizeof(t), &t, TCA_DEF_PAD))
    goto nla_put_failure;
    spin_unlock_bh(&d.tcf_lock);
    return skb.len;
    nla_put_failure:
    spin_unlock_bh(&d.tcf_lock);
    nlmsg_trim(skb, b);
    return -1;
    }
    static struct tc_action_ops act_simp_ops = {
    .kind		=	"simple",
    .id		=	TCA_ID_SIMP,
    .owner		=	THIS_MODULE,
    .act		=	tcf_simp_act,
    .dump		=	tcf_simp_dump,
    .cleanup	=	tcf_simp_release,
    .init		=	tcf_simp_init,
    .size		=	sizeof(struct tcf_defact),
    };
    MODULE_ALIAS_NET_ACT("simple");
#[no_mangle]
unsafe extern "C" fn simp_init_net(net: *mut net) -> __net_init int {
    static __net_init int simp_init_net(struct net *net)
    {
    struct tc_action_net *tn = net_generic(net, act_simp_ops.net_id);
    return tc_action_net_init(net, tn, &act_simp_ops);
    }
#[no_mangle]
unsafe extern "C" fn simp_exit_net(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit simp_exit_net(struct list_head *net_list)
    {
    tc_action_net_exit(net_list, act_simp_ops.net_id);
    }
    static struct pernet_operations simp_net_ops = {
    .init = simp_init_net,
    .exit_batch = simp_exit_net,
    .id   = &act_simp_ops.net_id,
    .size = sizeof(struct tc_action_net),
    };
    MODULE_AUTHOR("Jamal Hadi Salim(2005)");
    MODULE_DESCRIPTION("Simple example action");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn simp_init_module() -> int __init {
    static int __init simp_init_module(void)
    {
    let mut ret: c_int = tcf_register_action(&act_simp_ops, &simp_net_ops);
    if (!ret)
    pr_info("Simple TC action Loaded\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn simp_cleanup_module() -> void __exit {
    static void __exit simp_cleanup_module(void)
    {
    tcf_unregister_action(&act_simp_ops, &simp_net_ops);
    }
    module_init(simp_init_module);
    module_exit(simp_cleanup_module);
