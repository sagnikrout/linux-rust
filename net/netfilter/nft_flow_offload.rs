//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_flow_offload.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_flow_offload {
    pub flowtable: *mut nft_flowtable,
}

#[no_mangle]
unsafe extern "C" fn nft_flow_offload_skip(skb: *mut sk_buff, family: c_int) -> bool {
    static bool nft_flow_offload_skip(struct sk_buff *skb, int family)
    {
    if (skb_sec_path(skb))
    return true;
    if (family == NFPROTO_IPV4) {
    const struct ip_options *opt;
    opt = &(IPCB(skb).opt);
    if (unlikely(opt.optlen))
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn flow_offload_ct_tcp(ct: *mut nf_conn) {
    static void flow_offload_ct_tcp(struct nf_conn *ct)
    {
// conntrack will not see all packets, disable tcp window validation.
    spin_lock_bh(&ct.lock);
    ct.proto.tcp.seen[0].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
    ct.proto.tcp.seen[1].flags |= IP_CT_TCP_FLAG_BE_LIBERAL;
    spin_unlock_bh(&ct.lock);
    }
    static void nft_flow_offload_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_flow_offload *priv = nft_expr_priv(expr);
    struct nf_flowtable *flowtable = &priv.flowtable.data;
    struct tcphdr _tcph, *tcph = core::ptr::null_mut();
    let mut route: nf_flow_route = {};
    enum ip_conntrack_info ctinfo;
    struct flow_offload *flow;
    enum ip_conntrack_dir dir;
    struct nf_conn *ct;
    int ret;
    if (nft_flow_offload_skip(pkt.skb, nft_pf(pkt)))
    goto out;
    ct = nf_ct_get(pkt.skb, &ctinfo);
    if (!ct)
    goto out;
    switch (ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.dst.protonum) {
    case IPPROTO_TCP:
    tcph = skb_header_pointer(pkt.skb, nft_thoff(pkt),
    sizeof(_tcph), &_tcph);
    if (unlikely(!tcph || tcph.fin || tcph.rst ||
    !nf_conntrack_tcp_established(ct)))
    goto out;
    break;
    case IPPROTO_UDP:
    break;

    case IPPROTO_GRE: {
    struct nf_conntrack_tuple *tuple;
    if (ct.status & IPS_NAT_MASK)
    goto out;
    tuple = &ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple;
// No support for GRE v1
    if (tuple.src.u.gre.key || tuple.dst.u.gre.key)
    goto out;
    break;
    }

    default:
    goto out;
    }
    if (nf_ct_ext_exist(ct, NF_CT_EXT_HELPER) ||
    ct.status & (IPS_SEQ_ADJUST | IPS_NAT_CLASH))
    goto out;
    if (!nf_ct_is_confirmed(ct))
    goto out;
    if (test_and_set_bit(IPS_OFFLOAD_BIT, &ct.status))
    goto out;
    dir = CTINFO2DIR(ctinfo);
    if (nft_flow_route(pkt, ct, &route, dir, priv.flowtable) < 0)
    goto err_flow_route;
    flow = flow_offload_alloc(ct);
    if (!flow)
    goto err_flow_alloc;
    flow_offload_route_init(flow, &route);
    if (tcph)
    flow_offload_ct_tcp(ct);
    __set_bit(NF_FLOW_HW_BIDIRECTIONAL, &flow.flags);
    ret = flow_offload_add(flowtable, flow);
    if (ret < 0)
    goto err_flow_add;
    return;
    err_flow_add:
    flow_offload_free(flow);
    err_flow_alloc:
    dst_release(route.tuple[dir].dst);
    dst_release(route.tuple[!dir].dst);
    err_flow_route:
    clear_bit(IPS_OFFLOAD_BIT, &ct.status);
    out:
    regs.verdict.code = NFT_BREAK;
    }
    static int nft_flow_offload_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    let mut hook_mask: c_uint = (1 << NF_INET_FORWARD);
    if (ctx.family != NFPROTO_IPV4 &&
    ctx.family != NFPROTO_IPV6 &&
    ctx.family != NFPROTO_INET)
    return -EOPNOTSUPP;
    return nft_chain_validate_hooks(ctx.chain, hook_mask);
    }
    static const struct nla_policy nft_flow_offload_policy[NFTA_FLOW_MAX + 1] = {
    [NFTA_FLOW_TABLE_NAME]	= { .type = NLA_STRING,
    .len = NFT_NAME_MAXLEN - 1 },
    };
    static int nft_flow_offload_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_flow_offload *priv = nft_expr_priv(expr);
    let mut genmask: u8 = nft_genmask_next(ctx.net);
    struct nft_flowtable *flowtable;
    if (!tb[NFTA_FLOW_TABLE_NAME])
    return -EINVAL;
    flowtable = nft_flowtable_lookup(ctx.net, ctx.table,
    tb[NFTA_FLOW_TABLE_NAME], genmask);
    if (IS_ERR(flowtable))
    return PTR_ERR(flowtable);
    if (!nft_use_inc(&flowtable.use))
    return -EMFILE;
    priv.flowtable = flowtable;
    return nf_ct_netns_get(ctx.net, ctx.family);
    }
    static void nft_flow_offload_deactivate(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    enum nft_trans_phase phase)
    {
    struct nft_flow_offload *priv = nft_expr_priv(expr);
    nf_tables_deactivate_flowtable(ctx, priv.flowtable, phase);
    }
    static void nft_flow_offload_activate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_flow_offload *priv = nft_expr_priv(expr);
    nft_use_inc_restore(&priv.flowtable.use);
    }
    static void nft_flow_offload_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    nf_ct_netns_put(ctx.net, ctx.family);
    }
    static int nft_flow_offload_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_flow_offload *priv = nft_expr_priv(expr);
    if (nla_put_string(skb, NFTA_FLOW_TABLE_NAME, priv.flowtable.name))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static struct nft_expr_type nft_flow_offload_type;
    static const struct nft_expr_ops nft_flow_offload_ops = {
    .type		= &nft_flow_offload_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_flow_offload)),
    .eval		= nft_flow_offload_eval,
    .init		= nft_flow_offload_init,
    .activate	= nft_flow_offload_activate,
    .deactivate	= nft_flow_offload_deactivate,
    .destroy	= nft_flow_offload_destroy,
    .validate	= nft_flow_offload_validate,
    .dump		= nft_flow_offload_dump,
    };
    static struct nft_expr_type nft_flow_offload_type __read_mostly = {
    .name		= "flow_offload",
    .ops		= &nft_flow_offload_ops,
    .policy		= nft_flow_offload_policy,
    .maxattr	= NFTA_FLOW_MAX,
    .owner		= THIS_MODULE,
    };
    static int flow_offload_netdev_event(struct notifier_block *this,
    unsigned long event, void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    if (event != NETDEV_DOWN)
    return NOTIFY_DONE;
    nf_flow_table_cleanup(dev);
    return NOTIFY_DONE;
    }
    static struct notifier_block flow_offload_netdev_notifier = {
    .notifier_call	= flow_offload_netdev_event,
    };
#[no_mangle]
unsafe extern "C" fn nft_flow_offload_module_init() -> int __init {
    static int __init nft_flow_offload_module_init(void)
    {
    int err;
    err = register_netdevice_notifier(&flow_offload_netdev_notifier);
    if (err)
    goto err;
    err = nft_register_expr(&nft_flow_offload_type);
    if (err < 0)
    goto register_expr;
    return 0;
    register_expr:
    unregister_netdevice_notifier(&flow_offload_netdev_notifier);
    err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nft_flow_offload_module_exit() -> void __exit {
    static void __exit nft_flow_offload_module_exit(void)
    {
    nft_unregister_expr(&nft_flow_offload_type);
    unregister_netdevice_notifier(&flow_offload_netdev_notifier);
    }
    module_init(nft_flow_offload_module_init);
    module_exit(nft_flow_offload_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
    MODULE_ALIAS_NFT_EXPR("flow_offload");
    MODULE_DESCRIPTION("nftables hardware flow offload module");
