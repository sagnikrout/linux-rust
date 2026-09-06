//! Automatically rewritten from C to Rust
//! Source: net/core/lwtunnel.c
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
// lwtunnel	Infrastructure for light weight tunnels like mpls
//
// Authors:	Roopa Prabhu, <roopa@cumulusnetworks.com>
//

    DEFINE_STATIC_KEY_FALSE(nf_hooks_lwtunnel_enabled);
    EXPORT_SYMBOL_GPL(nf_hooks_lwtunnel_enabled);

    static const char *lwtunnel_encap_str(enum lwtunnel_encap_types encap_type)
    {
// Only lwt encaps implemented without using an interface for
// the encap need to return a string here.
//
    switch (encap_type) {
    case LWTUNNEL_ENCAP_MPLS:
    return "MPLS";
    case LWTUNNEL_ENCAP_ILA:
    return "ILA";
    case LWTUNNEL_ENCAP_SEG6:
    return "SEG6";
    case LWTUNNEL_ENCAP_BPF:
    return "BPF";
    case LWTUNNEL_ENCAP_SEG6_LOCAL:
    return "SEG6LOCAL";
    case LWTUNNEL_ENCAP_RPL:
    return "RPL";
    case LWTUNNEL_ENCAP_IOAM6:
    return "IOAM6";
    case LWTUNNEL_ENCAP_XFRM:
// module autoload not supported for encap type
    return core::ptr::null_mut();
    case LWTUNNEL_ENCAP_IP6:
    case LWTUNNEL_ENCAP_IP:
    case LWTUNNEL_ENCAP_NONE:
    case __LWTUNNEL_ENCAP_MAX:
// should not have got here
    WARN_ON(1);
    break;
    }
    return core::ptr::null_mut();
    }

    struct lwtunnel_state *lwtunnel_state_alloc(int encap_len)
    {
    struct lwtunnel_state *lws;
    lws = kzalloc(sizeof(*lws) + encap_len, GFP_ATOMIC);
    return lws;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_state_alloc);
    static const struct lwtunnel_encap_ops __rcu *
    lwtun_encaps[LWTUNNEL_ENCAP_MAX + 1] __read_mostly;
    int lwtunnel_encap_add_ops(const struct lwtunnel_encap_ops *ops,
    unsigned int num)
    {
    if (num > LWTUNNEL_ENCAP_MAX)
    return -ERANGE;
    return !cmpxchg((const struct lwtunnel_encap_ops **)
    &lwtun_encaps[num],
    core::ptr::null_mut(), ops) ? 0 : -1;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_encap_add_ops);
    int lwtunnel_encap_del_ops(const struct lwtunnel_encap_ops *ops,
    unsigned int encap_type)
    {
    int ret;
    if (encap_type == LWTUNNEL_ENCAP_NONE ||
    encap_type > LWTUNNEL_ENCAP_MAX)
    return -ERANGE;
    ret = (cmpxchg((const struct lwtunnel_encap_ops **)
    &lwtun_encaps[encap_type],
    ops, core::ptr::null_mut()) == ops) ? 0 : -1;
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_encap_del_ops);
    int lwtunnel_build_state(struct net *net, u16 encap_type,
    struct nlattr *encap, unsigned int family,
    const void *cfg, struct lwtunnel_state **lws,
    struct netlink_ext_ack *extack)
    {
    const struct lwtunnel_encap_ops *ops;
    let mut found: bool = false;
    let mut ret: c_int = -EINVAL;
    if (encap_type == LWTUNNEL_ENCAP_NONE ||
    encap_type > LWTUNNEL_ENCAP_MAX) {
    NL_SET_ERR_MSG_ATTR(extack, encap,
    "Unknown LWT encapsulation type");
    return ret;
    }
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[encap_type]);
    if (likely(ops && ops.build_state && try_module_get(ops.owner)))
    found = true;
    rcu_read_unlock();
    if (found) {
    ret = ops.build_state(net, encap, family, cfg, lws, extack);
    if (ret)
    module_put(ops.owner);
    } else {
// don't rely on -EOPNOTSUPP to detect match as build_state
// handlers could return it
//
    NL_SET_ERR_MSG_ATTR(extack, encap,
    "LWT encapsulation type not supported");
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_build_state);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_valid_encap_type(encap_type: u16, extack: *mut netlink_ext_ack) -> c_int {
    int lwtunnel_valid_encap_type(u16 encap_type, struct netlink_ext_ack *extack)
    {
    const struct lwtunnel_encap_ops *ops;
    let mut ret: c_int = -EINVAL;
    if (encap_type == LWTUNNEL_ENCAP_NONE ||
    encap_type > LWTUNNEL_ENCAP_MAX) {
    NL_SET_ERR_MSG(extack, "Unknown lwt encapsulation type");
    return ret;
    }
    ops = rcu_access_pointer(lwtun_encaps[encap_type]);

    if (!ops) {
    const char *encap_type_str = lwtunnel_encap_str(encap_type);
    if (encap_type_str) {
    request_module("rtnl-lwt-%s", encap_type_str);
    ops = rcu_access_pointer(lwtun_encaps[encap_type]);
    }
    }

    ret = ops ? 0 : -EOPNOTSUPP;
    if (ret < 0)
    NL_SET_ERR_MSG(extack, "lwt encapsulation type not supported");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_valid_encap_type);
    int lwtunnel_valid_encap_type_attr(struct nlattr *attr, int remaining,
    struct netlink_ext_ack *extack)
    {
    struct rtnexthop *rtnh = (struct rtnexthop *)attr;
    struct nlattr *nla_entype;
    struct nlattr *attrs;
    u16 encap_type;
    int attrlen;
    while (rtnh_ok(rtnh, remaining)) {
    attrlen = rtnh_attrlen(rtnh);
    if (attrlen > 0) {
    attrs = rtnh_attrs(rtnh);
    nla_entype = nla_find(attrs, attrlen, RTA_ENCAP_TYPE);
    if (nla_entype) {
    if (nla_len(nla_entype) < sizeof(u16)) {
    NL_SET_ERR_MSG(extack, "Invalid RTA_ENCAP_TYPE");
    return -EINVAL;
    }
    encap_type = nla_get_u16(nla_entype);
    if (lwtunnel_valid_encap_type(encap_type, extack))
    return -EOPNOTSUPP;
    }
    }
    rtnh = rtnh_next(rtnh, &remaining);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_valid_encap_type_attr);
#[no_mangle]
pub unsafe extern "C" fn lwtstate_free(lws: *mut lwtunnel_state) {
    void lwtstate_free(struct lwtunnel_state *lws)
    {
    const struct lwtunnel_encap_ops *ops = lwtun_encaps[lws.type];
    if (ops.destroy_state) {
    ops.destroy_state(lws);
    kfree_rcu(lws, rcu);
    } else {
    kfree(lws);
    }
    module_put(ops.owner);
    }
    EXPORT_SYMBOL_GPL(lwtstate_free);
    int lwtunnel_fill_encap(struct sk_buff *skb, struct lwtunnel_state *lwtstate,
    int encap_attr, int encap_type_attr)
    {
    const struct lwtunnel_encap_ops *ops;
    struct nlattr *nest;
    int ret;
    if (!lwtstate)
    return 0;
    if (lwtstate.type == LWTUNNEL_ENCAP_NONE ||
    lwtstate.type > LWTUNNEL_ENCAP_MAX)
    return 0;
    nest = nla_nest_start_noflag(skb, encap_attr);
    if (!nest)
    return -EMSGSIZE;
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[lwtstate.type]);
    if (likely(ops && ops.fill_encap))
    ret = ops.fill_encap(skb, lwtstate);
    rcu_read_unlock();
    if (ret)
    goto nla_put_failure;
    nla_nest_end(skb, nest);
    ret = nla_put_u16(skb, encap_type_attr, lwtstate.type);
    if (ret)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    nla_nest_cancel(skb, nest);
    return (ret == -EOPNOTSUPP ? 0 : ret);
    }
    EXPORT_SYMBOL_GPL(lwtunnel_fill_encap);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_get_encap_size(lwtstate: *mut lwtunnel_state) -> c_int {
    int lwtunnel_get_encap_size(struct lwtunnel_state *lwtstate)
    {
    const struct lwtunnel_encap_ops *ops;
    let mut ret: c_int = 0;
    if (!lwtstate)
    return 0;
    if (lwtstate.type == LWTUNNEL_ENCAP_NONE ||
    lwtstate.type > LWTUNNEL_ENCAP_MAX)
    return 0;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[lwtstate.type]);
    if (likely(ops && ops.get_encap_size))
    ret = nla_total_size(ops.get_encap_size(lwtstate));
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_get_encap_size);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_cmp_encap(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> c_int {
    int lwtunnel_cmp_encap(struct lwtunnel_state *a, struct lwtunnel_state *b)
    {
    const struct lwtunnel_encap_ops *ops;
    let mut ret: c_int = 0;
    if (!a && !b)
    return 0;
    if (!a || !b)
    return 1;
    if (a.type != b.type)
    return 1;
    if (a.type == LWTUNNEL_ENCAP_NONE ||
    a.type > LWTUNNEL_ENCAP_MAX)
    return 0;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[a.type]);
    if (likely(ops && ops.cmp_encap))
    ret = ops.cmp_encap(a, b);
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_cmp_encap);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int lwtunnel_output(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    const struct lwtunnel_encap_ops *ops;
    struct lwtunnel_state *lwtstate;
    struct dst_entry *dst;
    int ret;
    local_bh_disable();
    if (dev_xmit_recursion()) {
    net_crit_ratelimited("%s(): recursion limit reached on datapath\n",
    __func__);
    ret = -ENETDOWN;
    goto drop;
    }
    dst = skb_dst(skb);
    if (!dst) {
    ret = -EINVAL;
    goto drop;
    }
    lwtstate = dst.lwtstate;
    if (lwtstate.type == LWTUNNEL_ENCAP_NONE ||
    lwtstate.type > LWTUNNEL_ENCAP_MAX) {
    ret = 0;
    goto out;
    }
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[lwtstate.type]);
    if (likely(ops && ops.output)) {
// Encap pushes outer headers over the metadata; drop it.
    skb_metadata_clear(skb);
    dev_xmit_recursion_inc();
    ret = ops.output(net, sk, skb);
    dev_xmit_recursion_dec();
    }
    rcu_read_unlock();
    if (ret == -EOPNOTSUPP)
    goto drop;
    goto out;
    drop:
    kfree_skb(skb);
    out:
    local_bh_enable();
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_output);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_xmit(skb: *mut sk_buff) -> c_int {
    int lwtunnel_xmit(struct sk_buff *skb)
    {
    const struct lwtunnel_encap_ops *ops;
    struct lwtunnel_state *lwtstate;
    struct dst_entry *dst;
    int ret;
    local_bh_disable();
    if (dev_xmit_recursion()) {
    net_crit_ratelimited("%s(): recursion limit reached on datapath\n",
    __func__);
    ret = -ENETDOWN;
    goto drop;
    }
    dst = skb_dst(skb);
    if (!dst) {
    ret = -EINVAL;
    goto drop;
    }
    lwtstate = dst.lwtstate;
    if (lwtstate.type == LWTUNNEL_ENCAP_NONE ||
    lwtstate.type > LWTUNNEL_ENCAP_MAX) {
    ret = 0;
    goto out;
    }
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[lwtstate.type]);
    if (likely(ops && ops.xmit)) {
// Encap pushes outer headers over the metadata; drop it.
    skb_metadata_clear(skb);
    dev_xmit_recursion_inc();
    ret = ops.xmit(skb);
    dev_xmit_recursion_dec();
    }
    rcu_read_unlock();
    if (ret == -EOPNOTSUPP)
    goto drop;
    goto out;
    drop:
    kfree_skb(skb);
    out:
    local_bh_enable();
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_xmit);
#[no_mangle]
pub unsafe extern "C" fn lwtunnel_input(skb: *mut sk_buff) -> c_int {
    int lwtunnel_input(struct sk_buff *skb)
    {
    const struct lwtunnel_encap_ops *ops;
    struct lwtunnel_state *lwtstate;
    struct dst_entry *dst;
    int ret;
    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    if (dev_xmit_recursion()) {
    net_crit_ratelimited("%s(): recursion limit reached on datapath\n",
    __func__);
    ret = -ENETDOWN;
    goto drop;
    }
    dst = skb_dst(skb);
    if (!dst) {
    ret = -EINVAL;
    goto drop;
    }
    lwtstate = dst.lwtstate;
    if (lwtstate.type == LWTUNNEL_ENCAP_NONE ||
    lwtstate.type > LWTUNNEL_ENCAP_MAX)
    return 0;
    ret = -EOPNOTSUPP;
    rcu_read_lock();
    ops = rcu_dereference(lwtun_encaps[lwtstate.type]);
    if (likely(ops && ops.input)) {
// Encap pushes outer headers over the metadata; drop it.
    skb_metadata_clear(skb);
    dev_xmit_recursion_inc();
    ret = ops.input(skb);
    dev_xmit_recursion_dec();
    }
    rcu_read_unlock();
    if (ret == -EOPNOTSUPP)
    goto drop;
    return ret;
    drop:
    kfree_skb(skb);
    return ret;
    }
    EXPORT_SYMBOL_GPL(lwtunnel_input);
