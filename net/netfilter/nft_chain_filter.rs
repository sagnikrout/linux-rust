//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_chain_filter.c
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

    static unsigned int nft_do_chain_ipv4(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    nft_set_pktinfo_ipv4(&pkt);
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_ipv4 = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_IPV4,
    .hook_mask	= (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_FORWARD) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING),
    .hooks		= {
    [NF_INET_LOCAL_IN]	= nft_do_chain_ipv4,
    [NF_INET_LOCAL_OUT]	= nft_do_chain_ipv4,
    [NF_INET_FORWARD]	= nft_do_chain_ipv4,
    [NF_INET_PRE_ROUTING]	= nft_do_chain_ipv4,
    [NF_INET_POST_ROUTING]	= nft_do_chain_ipv4,
    },
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_ipv4_init() {
    static void nft_chain_filter_ipv4_init(void)
    {
    nft_register_chain_type(&nft_chain_filter_ipv4);
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_ipv4_fini() {
    static void nft_chain_filter_ipv4_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_ipv4);
    }

    static inline void nft_chain_filter_ipv4_init(void) {}
    static inline void nft_chain_filter_ipv4_fini(void) {}

    static unsigned int nft_do_chain_arp(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    nft_set_pktinfo_unspec(&pkt);
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_arp = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_ARP,
    .owner		= THIS_MODULE,
    .hook_mask	= (1 << NF_ARP_IN) |
    (1 << NF_ARP_OUT),
    .hooks		= {
    [NF_ARP_IN]		= nft_do_chain_arp,
    [NF_ARP_OUT]		= nft_do_chain_arp,
    },
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_arp_init() {
    static void nft_chain_filter_arp_init(void)
    {
    nft_register_chain_type(&nft_chain_filter_arp);
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_arp_fini() {
    static void nft_chain_filter_arp_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_arp);
    }

    static inline void nft_chain_filter_arp_init(void) {}
    static inline void nft_chain_filter_arp_fini(void) {}

    static unsigned int nft_do_chain_ipv6(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    nft_set_pktinfo_ipv6(&pkt);
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_ipv6 = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_IPV6,
    .hook_mask	= (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_FORWARD) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING),
    .hooks		= {
    [NF_INET_LOCAL_IN]	= nft_do_chain_ipv6,
    [NF_INET_LOCAL_OUT]	= nft_do_chain_ipv6,
    [NF_INET_FORWARD]	= nft_do_chain_ipv6,
    [NF_INET_PRE_ROUTING]	= nft_do_chain_ipv6,
    [NF_INET_POST_ROUTING]	= nft_do_chain_ipv6,
    },
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_ipv6_init() {
    static void nft_chain_filter_ipv6_init(void)
    {
    nft_register_chain_type(&nft_chain_filter_ipv6);
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_ipv6_fini() {
    static void nft_chain_filter_ipv6_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_ipv6);
    }

    static inline void nft_chain_filter_ipv6_init(void) {}
    static inline void nft_chain_filter_ipv6_fini(void) {}

    static unsigned int nft_do_chain_inet(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    switch (state.pf) {
    case NFPROTO_IPV4:
    nft_set_pktinfo_ipv4(&pkt);
    break;
    case NFPROTO_IPV6:
    nft_set_pktinfo_ipv6(&pkt);
    break;
    default:
    break;
    }
    return nft_do_chain(&pkt, priv);
    }
    static unsigned int nft_do_chain_inet_ingress(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    let mut ingress_state: nf_hook_state = *state;
    struct nft_pktinfo pkt;
    switch (skb.protocol) {
    case htons(ETH_P_IP):
// Original hook is NFPROTO_NETDEV and NF_NETDEV_INGRESS.
    ingress_state.pf = NFPROTO_IPV4;
    ingress_state.hook = NF_INET_INGRESS;
    nft_set_pktinfo(&pkt, skb, &ingress_state);
    if (nft_set_pktinfo_ipv4_ingress(&pkt) < 0)
    return NF_DROP;
    break;
    case htons(ETH_P_IPV6):
    ingress_state.pf = NFPROTO_IPV6;
    ingress_state.hook = NF_INET_INGRESS;
    nft_set_pktinfo(&pkt, skb, &ingress_state);
    if (nft_set_pktinfo_ipv6_ingress(&pkt) < 0)
    return NF_DROP;
    break;
    default:
    return NF_ACCEPT;
    }
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_inet = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_INET,
    .hook_mask	= (1 << NF_INET_INGRESS) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_FORWARD) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING),
    .hooks		= {
    [NF_INET_INGRESS]	= nft_do_chain_inet_ingress,
    [NF_INET_LOCAL_IN]	= nft_do_chain_inet,
    [NF_INET_LOCAL_OUT]	= nft_do_chain_inet,
    [NF_INET_FORWARD]	= nft_do_chain_inet,
    [NF_INET_PRE_ROUTING]	= nft_do_chain_inet,
    [NF_INET_POST_ROUTING]	= nft_do_chain_inet,
    },
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_inet_init() {
    static void nft_chain_filter_inet_init(void)
    {
    nft_register_chain_type(&nft_chain_filter_inet);
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_inet_fini() {
    static void nft_chain_filter_inet_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_inet);
    }

    static inline void nft_chain_filter_inet_init(void) {}
    static inline void nft_chain_filter_inet_fini(void) {}

    static unsigned int
    nft_do_chain_bridge(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    switch (eth_hdr(skb).h_proto) {
    case htons(ETH_P_IP):
    nft_set_pktinfo_ipv4_validate(&pkt);
    break;
    case htons(ETH_P_IPV6):
    nft_set_pktinfo_ipv6_validate(&pkt);
    break;
    default:
    nft_set_pktinfo_unspec(&pkt);
    break;
    }
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_bridge = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_BRIDGE,
    .hook_mask	= (1 << NF_BR_PRE_ROUTING) |
    (1 << NF_BR_LOCAL_IN) |
    (1 << NF_BR_FORWARD) |
    (1 << NF_BR_LOCAL_OUT) |
    (1 << NF_BR_POST_ROUTING),
    .hooks		= {
    [NF_BR_PRE_ROUTING]	= nft_do_chain_bridge,
    [NF_BR_LOCAL_IN]	= nft_do_chain_bridge,
    [NF_BR_FORWARD]		= nft_do_chain_bridge,
    [NF_BR_LOCAL_OUT]	= nft_do_chain_bridge,
    [NF_BR_POST_ROUTING]	= nft_do_chain_bridge,
    },
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_bridge_init() {
    static void nft_chain_filter_bridge_init(void)
    {
    nft_register_chain_type(&nft_chain_filter_bridge);
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_bridge_fini() {
    static void nft_chain_filter_bridge_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_bridge);
    }

    static inline void nft_chain_filter_bridge_init(void) {}
    static inline void nft_chain_filter_bridge_fini(void) {}

    static unsigned int nft_do_chain_netdev(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    nft_set_pktinfo_ipv4_validate(&pkt);
    break;
    case htons(ETH_P_IPV6):
    nft_set_pktinfo_ipv6_validate(&pkt);
    break;
    default:
    nft_set_pktinfo_unspec(&pkt);
    break;
    }
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_filter_netdev = {
    .name		= "filter",
    .type		= NFT_CHAIN_T_DEFAULT,
    .family		= NFPROTO_NETDEV,
    .hook_mask	= (1 << NF_NETDEV_INGRESS) |
    (1 << NF_NETDEV_EGRESS),
    .hooks		= {
    [NF_NETDEV_INGRESS]	= nft_do_chain_netdev,
    [NF_NETDEV_EGRESS]	= nft_do_chain_netdev,
    },
    };
    static int nft_netdev_event(unsigned long event, struct net_device *dev,
    struct nft_base_chain *basechain, bool changename)
    {
    struct nft_table *table = basechain.chain.table;
    struct nf_hook_ops *ops;
    struct nft_hook *hook;
    bool match;
    list_for_each_entry(hook, &basechain.hook_list, list) {
    ops = nft_hook_find_ops(hook, dev);
    match = !strncmp(hook.ifname, dev.name, hook.ifnamelen);
    switch (event) {
    case NETDEV_UNREGISTER:
// NOP if not found or new name still matching
    if (!ops || (changename && match))
    continue;
    if (!(table.flags & NFT_TABLE_F_DORMANT))
    nf_unregister_net_hook(dev_net(dev), ops);
    list_del_rcu(&ops.list);
    kfree_rcu(ops, rcu);
    break;
    case NETDEV_REGISTER:
// NOP if not matching or already registered
    if (!match || ops)
    continue;
    ops = kmemdup(&basechain.ops,
    sizeof(struct nf_hook_ops),
    GFP_KERNEL_ACCOUNT);
    if (!ops)
    return 1;
    ops.dev = dev;
    if (!(table.flags & NFT_TABLE_F_DORMANT) &&
    nf_register_net_hook(dev_net(dev), ops)) {
    kfree(ops);
    return 1;
    }
    list_add_tail_rcu(&ops.list, &hook.ops_list);
    break;
    }
    break;
    }
    return 0;
    }
    static int __nf_tables_netdev_event(unsigned long event,
    struct net_device *dev,
    bool changename)
    {
    struct nft_base_chain *basechain;
    struct nftables_pernet *nft_net;
    struct nft_chain *chain;
    struct nft_table *table;
    nft_net = nft_pernet(dev_net(dev));
    list_for_each_entry(table, &nft_net.tables, list) {
    if (table.family != NFPROTO_NETDEV &&
    table.family != NFPROTO_INET)
    continue;
    list_for_each_entry(chain, &table.chains, list) {
    if (!nft_is_base_chain(chain))
    continue;
    basechain = nft_base_chain(chain);
    if (table.family == NFPROTO_INET &&
    basechain.ops.hooknum != NF_INET_INGRESS)
    continue;
    if (nft_netdev_event(event, dev, basechain, changename))
    return 1;
    }
    }
    return 0;
    }
    static int nf_tables_netdev_event(struct notifier_block *this,
    unsigned long event, void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    struct nftables_pernet *nft_net;
    let mut ret: c_int = NOTIFY_DONE;
    if (event != NETDEV_REGISTER &&
    event != NETDEV_UNREGISTER &&
    event != NETDEV_CHANGENAME)
    return NOTIFY_DONE;
    nft_net = nft_pernet(dev_net(dev));
    mutex_lock(&nft_net.commit_mutex);
    if (event == NETDEV_CHANGENAME) {
    if (__nf_tables_netdev_event(NETDEV_REGISTER, dev, true)) {
    ret = NOTIFY_BAD;
    goto out_unlock;
    }
    __nf_tables_netdev_event(NETDEV_UNREGISTER, dev, true);
    } else if (__nf_tables_netdev_event(event, dev, false)) {
    ret = NOTIFY_BAD;
    }
    out_unlock:
    mutex_unlock(&nft_net.commit_mutex);
    return ret;
    }
    static struct notifier_block nf_tables_netdev_notifier = {
    .notifier_call	= nf_tables_netdev_event,
    };
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_netdev_init() -> c_int {
    static int nft_chain_filter_netdev_init(void)
    {
    int err;
    nft_register_chain_type(&nft_chain_filter_netdev);
    err = register_netdevice_notifier(&nf_tables_netdev_notifier);
    if (err)
    goto err_register_netdevice_notifier;
    return 0;
    err_register_netdevice_notifier:
    nft_unregister_chain_type(&nft_chain_filter_netdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_filter_netdev_fini() {
    static void nft_chain_filter_netdev_fini(void)
    {
    nft_unregister_chain_type(&nft_chain_filter_netdev);
    unregister_netdevice_notifier(&nf_tables_netdev_notifier);
    }

    static inline int nft_chain_filter_netdev_init(void) { return 0; }
    static inline void nft_chain_filter_netdev_fini(void) {}

#[no_mangle]
pub unsafe extern "C" fn nft_chain_filter_init() -> int __init {
    int __init nft_chain_filter_init(void)
    {
    int err;
    err = nft_chain_filter_netdev_init();
    if (err < 0)
    return err;
    nft_chain_filter_ipv4_init();
    nft_chain_filter_ipv6_init();
    nft_chain_filter_arp_init();
    nft_chain_filter_inet_init();
    nft_chain_filter_bridge_init();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nft_chain_filter_fini() {
    void nft_chain_filter_fini(void)
    {
    nft_chain_filter_bridge_fini();
    nft_chain_filter_inet_fini();
    nft_chain_filter_arp_fini();
    nft_chain_filter_ipv6_fini();
    nft_chain_filter_ipv4_fini();
    nft_chain_filter_netdev_fini();
    }
