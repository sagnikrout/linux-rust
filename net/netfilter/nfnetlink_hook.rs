//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nfnetlink_hook.c
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
// Copyright (c) 2021 Red Hat GmbH
//
// Author: Florian Westphal <fw@strlen.de>
//

    static const struct nla_policy nfnl_hook_nla_policy[NFNLA_HOOK_MAX + 1] = {
    [NFNLA_HOOK_HOOKNUM]	= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFNLA_HOOK_PRIORITY]	= { .type = NLA_U32 },
    [NFNLA_HOOK_DEV]	= { .type = NLA_STRING,
    .len = IFNAMSIZ - 1 },
    [NFNLA_HOOK_FUNCTION_NAME] = { .type = NLA_NUL_STRING,
    .len = KSYM_NAME_LEN, },
    [NFNLA_HOOK_MODULE_NAME] = { .type = NLA_NUL_STRING,
    .len = MODULE_NAME_LEN, },
    [NFNLA_HOOK_CHAIN_INFO] = { .type = NLA_NESTED, },
    };
    static int nf_netlink_dump_start_rcu(struct sock *nlsk, struct sk_buff *skb,
    const struct nlmsghdr *nlh,
    struct netlink_dump_control *c)
    {
    int err;
    if (!try_module_get(THIS_MODULE))
    return -EINVAL;
    rcu_read_unlock();
    err = netlink_dump_start(nlsk, skb, nlh, c);
    rcu_read_lock();
    module_put(THIS_MODULE);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfnl_dump_hook_data {
    pub devname: [c_char; IFNAMSIZ],
    pub headv: c_ulong,
    pub hook: u8,
}

    static struct nlattr *nfnl_start_info_type(struct sk_buff *nlskb, enum nfnl_hook_chaintype t)
    {
    struct nlattr *nest = nla_nest_start(nlskb, NFNLA_HOOK_CHAIN_INFO);
    int ret;
    if (!nest)
    return core::ptr::null_mut();
    ret = nla_put_be32(nlskb, NFNLA_HOOK_INFO_TYPE, htonl(t));
    if (ret == 0)
    return nest;
    nla_nest_cancel(nlskb, nest);
    return core::ptr::null_mut();
    }
    static int nfnl_hook_put_bpf_prog_info(struct sk_buff *nlskb,
    const struct nfnl_dump_hook_data *ctx,
    unsigned int seq,
    const struct bpf_prog *prog)
    {
    struct nlattr *nest, *nest2;
    int ret;
    if (!IS_ENABLED(CONFIG_NETFILTER_BPF_LINK))
    return 0;
    if (WARN_ON_ONCE(!prog))
    return 0;
    nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_BPF);
    if (!nest)
    return -EMSGSIZE;
    nest2 = nla_nest_start(nlskb, NFNLA_HOOK_INFO_DESC);
    if (!nest2)
    goto cancel_nest;
    ret = nla_put_be32(nlskb, NFNLA_HOOK_BPF_ID, htonl(prog.aux.id));
    if (ret)
    goto cancel_nest;
    nla_nest_end(nlskb, nest2);
    nla_nest_end(nlskb, nest);
    return 0;
    cancel_nest:
    nla_nest_cancel(nlskb, nest);
    return -EMSGSIZE;
    }
    static int nfnl_hook_put_nft_info_desc(struct sk_buff *nlskb, const char *tname,
    const char *name, u8 family)
    {
    struct nlattr *nest;
    nest = nla_nest_start(nlskb, NFNLA_HOOK_INFO_DESC);
    if (!nest ||
    nla_put_string(nlskb, NFNLA_CHAIN_TABLE, tname) ||
    nla_put_string(nlskb, NFNLA_CHAIN_NAME, name) ||
    nla_put_u8(nlskb, NFNLA_CHAIN_FAMILY, family)) {
    nla_nest_cancel(nlskb, nest);
    return -EMSGSIZE;
    }
    nla_nest_end(nlskb, nest);
    return 0;
    }
    static int nfnl_hook_put_nft_chain_info(struct sk_buff *nlskb,
    const struct nfnl_dump_hook_data *ctx,
    unsigned int seq,
    struct nft_chain *chain)
    {
    struct net *net = sock_net(nlskb.sk);
    struct nlattr *nest;
    let mut ret: c_int = 0;
    if (WARN_ON_ONCE(!chain))
    return 0;
    if (!nft_is_active(net, chain))
    return 0;
    nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_NFTABLES);
    if (!nest)
    return -EMSGSIZE;
    ret = nfnl_hook_put_nft_info_desc(nlskb, chain.table.name,
    chain.name, chain.table.family);
    if (ret) {
    nla_nest_cancel(nlskb, nest);
    return ret;
    }
    nla_nest_end(nlskb, nest);
    return 0;
    }
    static int nfnl_hook_put_nft_ft_info(struct sk_buff *nlskb,
    const struct nfnl_dump_hook_data *ctx,
    unsigned int seq,
    struct nf_flowtable *nf_ft)
    {
    struct nft_flowtable *ft =
    container_of(nf_ft, struct nft_flowtable, data);
    struct net *net = sock_net(nlskb.sk);
    struct nlattr *nest;
    let mut ret: c_int = 0;
    if (WARN_ON_ONCE(!nf_ft))
    return 0;
    if (!nft_is_active(net, ft))
    return 0;
    nest = nfnl_start_info_type(nlskb, NFNL_HOOK_TYPE_NFT_FLOWTABLE);
    if (!nest)
    return -EMSGSIZE;
    ret = nfnl_hook_put_nft_info_desc(nlskb, ft.table.name,
    ft.name, ft.table.family);
    if (ret) {
    nla_nest_cancel(nlskb, nest);
    return ret;
    }
    nla_nest_end(nlskb, nest);
    return 0;
    }
    static int nfnl_hook_dump_one(struct sk_buff *nlskb,
    const struct nfnl_dump_hook_data *ctx,
    const struct nf_hook_ops *ops, int priority,
    int family, unsigned int seq)
    {
    let mut event: u16 = nfnl_msg_type(NFNL_SUBSYS_HOOK, NFNL_MSG_HOOK_GET);
    let mut portid: c_uint = NETLINK_CB(nlskb).portid;
    struct nlmsghdr *nlh;
    let mut ret: c_int = -EMSGSIZE;
    u32 hooknum;

    char sym[KSYM_SYMBOL_LEN];
    char *module_name;

    nlh = nfnl_msg_put(nlskb, portid, seq, event,
    NLM_F_MULTI, family, NFNETLINK_V0, 0);
    if (!nlh)
    goto nla_put_failure;

    ret = snprintf(sym, sizeof(sym), "%ps", ops.hook);
    if (ret >= sizeof(sym)) {
    ret = -EINVAL;
    goto nla_put_failure;
    }
    module_name = strstr(sym, " [");
    if (module_name) {
    char *end;
// module_name = '\0';
    module_name += 2;
    end = strchr(module_name, ']');
    if (end) {
// end = 0;
    ret = nla_put_string(nlskb, NFNLA_HOOK_MODULE_NAME, module_name);
    if (ret)
    goto nla_put_failure;
    }
    }
    ret = nla_put_string(nlskb, NFNLA_HOOK_FUNCTION_NAME, sym);
    if (ret)
    goto nla_put_failure;

    if (ops.pf == NFPROTO_INET && ops.hooknum == NF_INET_INGRESS)
    hooknum = NF_NETDEV_INGRESS;
    else
    hooknum = ops.hooknum;
    ret = nla_put_be32(nlskb, NFNLA_HOOK_HOOKNUM, htonl(hooknum));
    if (ret)
    goto nla_put_failure;
    ret = nla_put_be32(nlskb, NFNLA_HOOK_PRIORITY, htonl(priority));
    if (ret)
    goto nla_put_failure;
    switch (ops.hook_ops_type) {
    case NF_HOOK_OP_NF_TABLES:
    ret = nfnl_hook_put_nft_chain_info(nlskb, ctx, seq, ops.priv);
    break;
    case NF_HOOK_OP_BPF:
    ret = nfnl_hook_put_bpf_prog_info(nlskb, ctx, seq, ops.priv);
    break;
    case NF_HOOK_OP_NFT_FT:
    ret = nfnl_hook_put_nft_ft_info(nlskb, ctx, seq, ops.priv);
    break;
    case NF_HOOK_OP_UNDEFINED:
    break;
    default:
    WARN_ON_ONCE(1);
    break;
    }
    if (ret)
    goto nla_put_failure;
    nlmsg_end(nlskb, nlh);
    return 0;
    nla_put_failure:
    nlmsg_trim(nlskb, nlh);
    return ret;
    }
    static const struct nf_hook_entries *
    nfnl_hook_entries_head(u8 pf, unsigned int hook, struct net *net, const char *dev)
    {
    const struct nf_hook_entries *hook_head = core::ptr::null_mut();

    struct net_device *netdev;

    switch (pf) {
    case NFPROTO_IPV4:
    if (hook >= ARRAY_SIZE(net.nf.hooks_ipv4))
    return ERR_PTR(-EINVAL);
    hook_head = rcu_dereference(net.nf.hooks_ipv4[hook]);
    break;
    case NFPROTO_IPV6:
    if (hook >= ARRAY_SIZE(net.nf.hooks_ipv6))
    return ERR_PTR(-EINVAL);
    hook_head = rcu_dereference(net.nf.hooks_ipv6[hook]);
    break;
    case NFPROTO_ARP:

    if (hook >= ARRAY_SIZE(net.nf.hooks_arp))
    return ERR_PTR(-EINVAL);
    hook_head = rcu_dereference(net.nf.hooks_arp[hook]);

    break;
    case NFPROTO_BRIDGE:

    if (hook >= ARRAY_SIZE(net.nf.hooks_bridge))
    return ERR_PTR(-EINVAL);
    hook_head = rcu_dereference(net.nf.hooks_bridge[hook]);

    break;

    case NFPROTO_NETDEV:
    if (hook >= NF_NETDEV_NUMHOOKS)
    return ERR_PTR(-EOPNOTSUPP);
    if (!dev)
    return ERR_PTR(-ENODEV);
    netdev = dev_get_by_name_rcu(net, dev);
    if (!netdev)
    return ERR_PTR(-ENODEV);

    if (hook == NF_NETDEV_INGRESS)
    return rcu_dereference(netdev.nf_hooks_ingress);

    if (hook == NF_NETDEV_EGRESS)
    return rcu_dereference(netdev.nf_hooks_egress);

    fallthrough;

    default:
    return ERR_PTR(-EPROTONOSUPPORT);
    }
    return hook_head;
    }
    static int nfnl_hook_dump_nat(struct sk_buff *nlskb,
    const struct nfnl_dump_hook_data *ctx,
    const struct nf_hook_ops *ops,
    int family, unsigned int seq)
    {
    struct nf_nat_lookup_hook_priv *priv = ops.priv;
    struct nf_hook_entries *e = rcu_dereference(priv.entries);
    struct nf_hook_ops **nat_ops;
    int i, err;
    if (!e)
    return 0;
    nat_ops = nf_hook_entries_get_hook_ops(e);
    for (i = 0; i < e.num_hook_entries; i++) {
    err = nfnl_hook_dump_one(nlskb, ctx, nat_ops[i],
    ops.priority, family, seq);
    if (err)
    return err;
    }
    return 0;
    }
    static int nfnl_hook_dump(struct sk_buff *nlskb,
    struct netlink_callback *cb)
    {
    struct nfgenmsg *nfmsg = nlmsg_data(cb.nlh);
    struct nfnl_dump_hook_data *ctx = cb.data;
    int err, family = nfmsg.nfgen_family;
    struct net *net = sock_net(nlskb.sk);
    struct nf_hook_ops * const *ops;
    const struct nf_hook_entries *e;
    let mut i: c_uint = cb.args[0];
    rcu_read_lock();
    e = nfnl_hook_entries_head(family, ctx.hook, net, ctx.devname);
    if (!e)
    goto done;
    if (IS_ERR(e)) {
    cb.seq++;
    goto done;
    }
    if ((unsigned long)e != ctx.headv || i >= e.num_hook_entries)
    cb.seq++;
    ops = nf_hook_entries_get_hook_ops(e);
    for (; i < e.num_hook_entries; i++) {
    if (ops[i].hook_ops_type == NF_HOOK_OP_NAT)
    err = nfnl_hook_dump_nat(nlskb, ctx, ops[i], family,
    cb.nlh.nlmsg_seq);
    else
    err = nfnl_hook_dump_one(nlskb, ctx, ops[i],
    ops[i].priority, family,
    cb.nlh.nlmsg_seq);
    if (err)
    break;
    }
    done:
    nl_dump_check_consistent(cb, nlmsg_hdr(nlskb));
    rcu_read_unlock();
    cb.args[0] = i;
    return nlskb.len;
    }
#[no_mangle]
unsafe extern "C" fn nfnl_hook_dump_start(cb: *mut netlink_callback) -> c_int {
    static int nfnl_hook_dump_start(struct netlink_callback *cb)
    {
    const struct nfgenmsg *nfmsg = nlmsg_data(cb.nlh);
    const struct nlattr * const *nla = cb.data;
    struct nfnl_dump_hook_data *ctx = core::ptr::null_mut();
    struct net *net = sock_net(cb.skb.sk);
    let mut family: u8 = nfmsg.nfgen_family;
    char name[IFNAMSIZ] = "";
    const void *head;
    u32 hooknum;
    hooknum = ntohl(nla_get_be32(nla[NFNLA_HOOK_HOOKNUM]));
    if (hooknum > 255)
    return -EINVAL;
    if (family == NFPROTO_NETDEV) {
    if (!nla[NFNLA_HOOK_DEV])
    return -EINVAL;
    nla_strscpy(name, nla[NFNLA_HOOK_DEV], sizeof(name));
    }
    rcu_read_lock();
// Not dereferenced; for consistency check only
    head = nfnl_hook_entries_head(family, hooknum, net, name);
    rcu_read_unlock();
    if (head && IS_ERR(head))
    return PTR_ERR(head);
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return -ENOMEM;
    strscpy(ctx.devname, name, sizeof(ctx.devname));
    ctx.headv = (unsigned long)head;
    ctx.hook = hooknum;
    cb.seq = 1;
    cb.data = ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfnl_hook_dump_stop(cb: *mut netlink_callback) -> c_int {
    static int nfnl_hook_dump_stop(struct netlink_callback *cb)
    {
    kfree(cb.data);
    return 0;
    }
    static int nfnl_hook_get(struct sk_buff *skb,
    const struct nfnl_info *info,
    const struct nlattr * const nla[])
    {
    if (!nla[NFNLA_HOOK_HOOKNUM])
    return -EINVAL;
    if (info.nlh.nlmsg_flags & NLM_F_DUMP) {
    struct netlink_dump_control c = {
    .start = nfnl_hook_dump_start,
    .done = nfnl_hook_dump_stop,
    .dump = nfnl_hook_dump,
    .module = THIS_MODULE,
    .data = (void *)nla,
    };
    return nf_netlink_dump_start_rcu(info.sk, skb, info.nlh, &c);
    }
    return -EOPNOTSUPP;
    }
    static const struct nfnl_callback nfnl_hook_cb[NFNL_MSG_HOOK_MAX] = {
    [NFNL_MSG_HOOK_GET] = {
    .call		= nfnl_hook_get,
    .type		= NFNL_CB_RCU,
    .attr_count	= NFNLA_HOOK_MAX,
    .policy		= nfnl_hook_nla_policy
    },
    };
    static const struct nfnetlink_subsystem nfhook_subsys = {
    .name				= "nfhook",
    .subsys_id			= NFNL_SUBSYS_HOOK,
    .cb_count			= NFNL_MSG_HOOK_MAX,
    .cb				= nfnl_hook_cb,
    };
    MODULE_ALIAS_NFNL_SUBSYS(NFNL_SUBSYS_HOOK);
#[no_mangle]
unsafe extern "C" fn nfnetlink_hook_init() -> int __init {
    static int __init nfnetlink_hook_init(void)
    {
    return nfnetlink_subsys_register(&nfhook_subsys);
    }
#[no_mangle]
unsafe extern "C" fn nfnetlink_hook_exit() -> void __exit {
    static void __exit nfnetlink_hook_exit(void)
    {
    nfnetlink_subsys_unregister(&nfhook_subsys);
    }
    module_init(nfnetlink_hook_init);
    module_exit(nfnetlink_hook_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("nfnetlink_hook: list registered netfilter hooks");
