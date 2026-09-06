//! Automatically rewritten from C to Rust
//! Source: net/sched/act_nat.c
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
// Stateless NAT actions
//
// Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
//

    static struct tc_action_ops act_nat_ops;
    static const struct nla_policy nat_policy[TCA_NAT_MAX + 1] = {
    [TCA_NAT_PARMS]	= { .len = sizeof(struct tc_nat) },
    };
    static int tcf_nat_init(struct net *net, struct nlattr *nla, struct nlattr *est,
    struct tc_action **a, struct tcf_proto *tp,
    u32 flags, struct netlink_ext_ack *extack)
    {
    struct tc_action_net *tn = net_generic(net, act_nat_ops.net_id);
    let mut bind: bool = flags & TCA_ACT_FLAGS_BIND;
    struct tcf_nat_parms *nparm, *oparm;
    struct nlattr *tb[TCA_NAT_MAX + 1];
    struct tcf_chain *goto_ch = core::ptr::null_mut();
    struct tc_nat *parm;
    let mut ret: c_int = 0, err;
    struct tcf_nat *p;
    u32 index;
    if (nla == core::ptr::null_mut())
    return -EINVAL;
    err = nla_parse_nested_deprecated(tb, TCA_NAT_MAX, nla, nat_policy,
    core::ptr::null_mut());
    if (err < 0)
    return err;
    if (tb[TCA_NAT_PARMS] == core::ptr::null_mut())
    return -EINVAL;
    parm = nla_data(tb[TCA_NAT_PARMS]);
    index = parm.index;
    err = tcf_idr_check_alloc(tn, &index, a, bind);
    if (!err) {
    ret = tcf_idr_create_from_flags(tn, index, est, a, &act_nat_ops,
    bind, flags);
    if (ret) {
    tcf_idr_cleanup(tn, index);
    return ret;
    }
    ret = ACT_P_CREATED;
    } else if (err > 0) {
    if (bind)
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
    nparm = kzalloc_obj(*nparm);
    if (!nparm) {
    err = -ENOMEM;
    goto release_idr;
    }
    nparm.old_addr = parm.old_addr;
    nparm.new_addr = parm.new_addr;
    nparm.mask = parm.mask;
    nparm.flags = parm.flags;
    nparm.action = parm.action;
    p = to_tcf_nat(*a);
    spin_lock_bh(&p.tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, parm.action, goto_ch);
    oparm = rcu_replace_pointer(p.parms, nparm, lockdep_is_held(&p.tcf_lock));
    spin_unlock_bh(&p.tcf_lock);
    if (goto_ch)
    tcf_chain_put_by_act(goto_ch);
    if (oparm)
    kfree_rcu(oparm, rcu);
    return ret;
    release_idr:
    tcf_idr_release(*a, bind);
    return err;
    }
    TC_INDIRECT_SCOPE int tcf_nat_act(struct sk_buff *skb,
    const struct tc_action *a,
    struct tcf_result *res)
    {
    struct tcf_nat *p = to_tcf_nat(a);
    struct tcf_nat_parms *parms;
    struct iphdr *iph;
    __be32 old_addr;
    __be32 new_addr;
    __be32 mask;
    __be32 addr;
    int egress;
    int action;
    int ihl;
    int noff;
    tcf_lastuse_update(&p.tcf_tm);
    tcf_action_update_bstats(&p.common, skb);
    parms = rcu_dereference_bh(p.parms);
    action = parms.action;
    if (unlikely(action == TC_ACT_SHOT))
    goto drop;
    old_addr = parms.old_addr;
    new_addr = parms.new_addr;
    mask = parms.mask;
    egress = parms.flags & TCA_NAT_FLAG_EGRESS;
    noff = skb_network_offset(skb);
    if (!pskb_may_pull(skb, sizeof(*iph) + noff))
    goto drop;
    iph = ip_hdr(skb);
    if (egress)
    addr = iph.saddr;
    else
    addr = iph.daddr;
    if (!((old_addr ^ addr) & mask)) {
    if (skb_try_make_writable(skb, sizeof(*iph) + noff))
    goto drop;
    new_addr &= mask;
    new_addr |= addr & ~mask;
// Rewrite IP header
    iph = ip_hdr(skb);
    if (egress)
    iph.saddr = new_addr;
    else
    iph.daddr = new_addr;
    csum_replace4(&iph.check, addr, new_addr);
    } else if ((iph.frag_off & htons(IP_OFFSET)) ||
    iph.protocol != IPPROTO_ICMP) {
    goto out;
    }
    ihl = iph.ihl * 4;
// It would be nice to share code with stateful NAT.
    switch (iph.frag_off & htons(IP_OFFSET) ? 0 : iph.protocol) {
    case IPPROTO_TCP:
    {
    struct tcphdr *tcph;
    if (!pskb_may_pull(skb, ihl + sizeof(*tcph) + noff) ||
    skb_try_make_writable(skb, ihl + sizeof(*tcph) + noff))
    goto drop;
    tcph = (void *)(skb_network_header(skb) + ihl);
    inet_proto_csum_replace4(&tcph.check, skb, addr, new_addr,
    true);
    break;
    }
    case IPPROTO_UDP:
    {
    struct udphdr *udph;
    if (!pskb_may_pull(skb, ihl + sizeof(*udph) + noff) ||
    skb_try_make_writable(skb, ihl + sizeof(*udph) + noff))
    goto drop;
    udph = (void *)(skb_network_header(skb) + ihl);
    if (udph.check || skb.ip_summed == CHECKSUM_PARTIAL) {
    inet_proto_csum_replace4(&udph.check, skb, addr,
    new_addr, true);
    if (!udph.check)
    udph.check = CSUM_MANGLED_0;
    }
    break;
    }
    case IPPROTO_ICMP:
    {
    struct icmphdr *icmph;
    if (!pskb_may_pull(skb, ihl + sizeof(*icmph) + noff))
    goto drop;
    icmph = (void *)(skb_network_header(skb) + ihl);
    if (!icmp_is_err(icmph.type))
    break;
    if (!pskb_may_pull(skb, ihl + sizeof(*icmph) + sizeof(*iph) +
    noff))
    goto drop;
    icmph = (void *)(skb_network_header(skb) + ihl);
    iph = (void *)(icmph + 1);
    if (egress)
    addr = iph.daddr;
    else
    addr = iph.saddr;
    if ((old_addr ^ addr) & mask)
    break;
    if (skb_try_make_writable(skb, ihl + sizeof(*icmph) +
    sizeof(*iph) + noff))
    goto drop;
    icmph = (void *)(skb_network_header(skb) + ihl);
    iph = (void *)(icmph + 1);
    new_addr &= mask;
    new_addr |= addr & ~mask;
// XXX Fix up the inner checksums.
    if (egress)
    iph.daddr = new_addr;
    else
    iph.saddr = new_addr;
    inet_proto_csum_replace4(&icmph.checksum, skb, addr, new_addr,
    false);
    break;
    }
    default:
    break;
    }
    out:
    return action;
    drop:
    tcf_action_inc_drop_qstats(&p.common);
    return TC_ACT_SHOT;
    }
    static int tcf_nat_dump(struct sk_buff *skb, struct tc_action *a,
    int bind, int ref)
    {
    unsigned char *b = skb_tail_pointer(skb);
    const struct tcf_nat *p = to_tcf_nat(a);
    const struct tcf_nat_parms *parms;
    struct tc_nat opt = {
    .index    = p.tcf_index,
    .refcnt   = refcount_read(&p.tcf_refcnt) - ref,
    .bindcnt  = atomic_read(&p.tcf_bindcnt) - bind,
    };
    struct tcf_t t;
    rcu_read_lock();
    parms = rcu_dereference(p.parms);
    opt.action = parms.action;
    opt.old_addr = parms.old_addr;
    opt.new_addr = parms.new_addr;
    opt.mask = parms.mask;
    opt.flags = parms.flags;
    if (nla_put(skb, TCA_NAT_PARMS, sizeof(opt), &opt))
    goto nla_put_failure;
    tcf_tm_dump(&t, &p.tcf_tm);
    if (nla_put_64bit(skb, TCA_NAT_TM, sizeof(t), &t, TCA_NAT_PAD))
    goto nla_put_failure;
    rcu_read_unlock();
    return skb.len;
    nla_put_failure:
    rcu_read_unlock();
    nlmsg_trim(skb, b);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn tcf_nat_cleanup(a: *mut tc_action) {
    static void tcf_nat_cleanup(struct tc_action *a)
    {
    struct tcf_nat *p = to_tcf_nat(a);
    struct tcf_nat_parms *parms;
    parms = rcu_dereference_protected(p.parms, 1);
    if (parms)
    kfree_rcu(parms, rcu);
    }
    static struct tc_action_ops act_nat_ops = {
    .kind		=	"nat",
    .id		=	TCA_ID_NAT,
    .owner		=	THIS_MODULE,
    .act		=	tcf_nat_act,
    .dump		=	tcf_nat_dump,
    .init		=	tcf_nat_init,
    .cleanup	=	tcf_nat_cleanup,
    .size		=	sizeof(struct tcf_nat),
    };
    MODULE_ALIAS_NET_ACT("nat");
#[no_mangle]
unsafe extern "C" fn nat_init_net(net: *mut net) -> __net_init int {
    static __net_init int nat_init_net(struct net *net)
    {
    struct tc_action_net *tn = net_generic(net, act_nat_ops.net_id);
    return tc_action_net_init(net, tn, &act_nat_ops);
    }
#[no_mangle]
unsafe extern "C" fn nat_exit_net(net_list: *mut list_head) -> void __net_exit {
    static void __net_exit nat_exit_net(struct list_head *net_list)
    {
    tc_action_net_exit(net_list, act_nat_ops.net_id);
    }
    static struct pernet_operations nat_net_ops = {
    .init = nat_init_net,
    .exit_batch = nat_exit_net,
    .id   = &act_nat_ops.net_id,
    .size = sizeof(struct tc_action_net),
    };
    MODULE_DESCRIPTION("Stateless NAT actions");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn nat_init_module() -> int __init {
    static int __init nat_init_module(void)
    {
    return tcf_register_action(&act_nat_ops, &nat_net_ops);
    }
#[no_mangle]
unsafe extern "C" fn nat_cleanup_module() -> void __exit {
    static void __exit nat_cleanup_module(void)
    {
    tcf_unregister_action(&act_nat_ops, &nat_net_ops);
    }
    module_init(nat_init_module);
    module_exit(nat_cleanup_module);
