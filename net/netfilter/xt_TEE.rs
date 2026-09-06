//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_TEE.c
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
// "TEE" target extension for Xtables
// Copyright © Sebastian Claßen, 2007
// Jan Engelhardt, 2007-2010
//
// based on ipt_ROUTE.c from Cédric de Launois
// <delaunois@info.ucl.be>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tee_priv {
    pub list: list_head,
    pub tginfo: *mut xt_tee_tginfo,
    pub oif: c_int,
}

    static unsigned int tee_net_id __read_mostly;
    static const union nf_inet_addr tee_zero_address;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_net {
    pub priv_list: list_head,
// lock protects the priv_list
    pub lock: mutex,
}

    static unsigned int
    tee_tg4(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tee_tginfo *info = par.targinfo;
    let mut oif: c_int = info.priv ? info.priv.oif : 0;
    nf_dup_ipv4(xt_net(par), skb, xt_hooknum(par), &info.gw.in, oif);
    return XT_CONTINUE;
    }

    static unsigned int
    tee_tg6(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tee_tginfo *info = par.targinfo;
    let mut oif: c_int = info.priv ? info.priv.oif : 0;
    nf_dup_ipv6(xt_net(par), skb, xt_hooknum(par), &info.gw.in6, oif);
    return XT_CONTINUE;
    }

    static int tee_netdev_event(struct notifier_block *this, unsigned long event,
    void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    struct net *net = dev_net(dev);
    struct tee_net *tn = net_generic(net, tee_net_id);
    struct xt_tee_priv *priv;
    mutex_lock(&tn.lock);
    list_for_each_entry(priv, &tn.priv_list, list) {
    switch (event) {
    case NETDEV_REGISTER:
    if (!strcmp(dev.name, priv.tginfo.oif))
    priv.oif = dev.ifindex;
    break;
    case NETDEV_UNREGISTER:
    if (dev.ifindex == priv.oif)
    priv.oif = -1;
    break;
    case NETDEV_CHANGENAME:
    if (!strcmp(dev.name, priv.tginfo.oif))
    priv.oif = dev.ifindex;
#[no_mangle]
pub unsafe extern "C" fn if(priv->oif: dev->ifindex ==) -> else {
    else if (dev.ifindex == priv.oif)
    priv.oif = -1;
    break;
    }
    }
    mutex_unlock(&tn.lock);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn tee_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int tee_tg_check(const struct xt_tgchk_param *par)
    {
    struct tee_net *tn = net_generic(par.net, tee_net_id);
    struct xt_tee_tginfo *info = par.targinfo;
    struct xt_tee_priv *priv;
// 0.0.0.0 and :: not allowed
    if (memcmp(&info.gw, &tee_zero_address,
    sizeof(tee_zero_address)) == 0)
    return -EINVAL;
    if (info.oif[0]) {
    struct net_device *dev;
    if (info.oif[sizeof(info.oif)-1] != '\0')
    return -EINVAL;
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    priv.tginfo  = info;
    priv.oif     = -1;
    info.priv    = priv;
    dev = dev_get_by_name(par.net, info.oif);
    if (dev) {
    priv.oif = dev.ifindex;
    dev_put(dev);
    }
    mutex_lock(&tn.lock);
    list_add(&priv.list, &tn.priv_list);
    mutex_unlock(&tn.lock);
    } else
    info.priv = core::ptr::null_mut();
    static_key_slow_inc(&xt_tee_enabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tee_tg_destroy(par: *const xt_tgdtor_param) {
    static void tee_tg_destroy(const struct xt_tgdtor_param *par)
    {
    struct tee_net *tn = net_generic(par.net, tee_net_id);
    struct xt_tee_tginfo *info = par.targinfo;
    if (info.priv) {
    mutex_lock(&tn.lock);
    list_del(&info.priv.list);
    mutex_unlock(&tn.lock);
    kfree(info.priv);
    }
    static_key_slow_dec(&xt_tee_enabled);
    }
    static struct xt_target tee_tg_reg[] __read_mostly = {
    {
    .name       = "TEE",
    .revision   = 1,
    .family     = NFPROTO_IPV4,
    .target     = tee_tg4,
    .targetsize = sizeof(struct xt_tee_tginfo),
    .usersize   = offsetof(struct xt_tee_tginfo, priv),
    .checkentry = tee_tg_check,
    .destroy    = tee_tg_destroy,
    .me         = THIS_MODULE,
    },

    {
    .name       = "TEE",
    .revision   = 1,
    .family     = NFPROTO_IPV6,
    .target     = tee_tg6,
    .targetsize = sizeof(struct xt_tee_tginfo),
    .usersize   = offsetof(struct xt_tee_tginfo, priv),
    .checkentry = tee_tg_check,
    .destroy    = tee_tg_destroy,
    .me         = THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn tee_net_init(net: *mut net) -> int __net_init {
    static int __net_init tee_net_init(struct net *net)
    {
    struct tee_net *tn = net_generic(net, tee_net_id);
    INIT_LIST_HEAD(&tn.priv_list);
    mutex_init(&tn.lock);
    return 0;
    }
    static struct pernet_operations tee_net_ops = {
    .init = tee_net_init,
    .id   = &tee_net_id,
    .size = sizeof(struct tee_net),
    };
    static struct notifier_block tee_netdev_notifier = {
    .notifier_call = tee_netdev_event,
    };
#[no_mangle]
unsafe extern "C" fn tee_tg_init() -> int __init {
    static int __init tee_tg_init(void)
    {
    int ret;
    ret = register_pernet_subsys(&tee_net_ops);
    if (ret < 0)
    return ret;
    ret = xt_register_targets(tee_tg_reg, ARRAY_SIZE(tee_tg_reg));
    if (ret < 0)
    goto cleanup_subsys;
    ret = register_netdevice_notifier(&tee_netdev_notifier);
    if (ret < 0)
    goto unregister_targets;
    return 0;
    unregister_targets:
    xt_unregister_targets(tee_tg_reg, ARRAY_SIZE(tee_tg_reg));
    cleanup_subsys:
    unregister_pernet_subsys(&tee_net_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tee_tg_exit() -> void __exit {
    static void __exit tee_tg_exit(void)
    {
    unregister_netdevice_notifier(&tee_netdev_notifier);
    xt_unregister_targets(tee_tg_reg, ARRAY_SIZE(tee_tg_reg));
    unregister_pernet_subsys(&tee_net_ops);
    }
    module_init(tee_tg_init);
    module_exit(tee_tg_exit);
    MODULE_AUTHOR("Sebastian Claßen <sebastian.classen@freenet.ag>");
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("Xtables: Reroute packet copy");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_TEE");
    MODULE_ALIAS("ip6t_TEE");
