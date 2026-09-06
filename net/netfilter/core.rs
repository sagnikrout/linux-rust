//! Automatically rewritten from C to Rust
//! Source: net/netfilter/core.c
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
// netfilter.c: look after the filters for various protocols.
// Heavily influenced by the old firewall.c by David Bonn and Alan Cox.
//
// Thanks to Rob `CmdrTaco' Malda for not influencing this code in any
// way.
//

    struct static_key nf_hooks_needed[NFPROTO_NUMPROTO][NF_MAX_HOOKS];
    EXPORT_SYMBOL(nf_hooks_needed);

    static DEFINE_MUTEX(nf_hook_mutex);
// max hooks per family/hooknum
pub const MAX_HOOK_COUNT: c_int = 1024;

    rcu_dereference_protected(e, lockdep_is_held(&nf_hook_mutex))
    static struct nf_hook_entries *allocate_hook_entries_size(u16 num)
    {
    struct nf_hook_entries *e;
    size_t alloc = sizeof(*e) +
    sizeof(struct nf_hook_entry) * num +
    sizeof(struct nf_hook_ops *) * num +
    sizeof(struct nf_hook_entries_rcu_head);
    if (num == 0)
    return core::ptr::null_mut();
    e = kvzalloc(alloc, GFP_KERNEL_ACCOUNT);
    if (e)
    e.num_hook_entries = num;
    return e;
    }
#[no_mangle]
unsafe extern "C" fn __nf_hook_entries_free(h: *mut rcu_head) {
    static void __nf_hook_entries_free(struct rcu_head *h)
    {
    struct nf_hook_entries_rcu_head *head;
    head = container_of(h, struct nf_hook_entries_rcu_head, head);
    kvfree(head.allocation);
    }
#[no_mangle]
unsafe extern "C" fn nf_hook_entries_free(e: *mut nf_hook_entries) {
    static void nf_hook_entries_free(struct nf_hook_entries *e)
    {
    struct nf_hook_entries_rcu_head *head;
    struct nf_hook_ops **ops;
    unsigned int num;
    if (!e)
    return;
    num = e.num_hook_entries;
    ops = nf_hook_entries_get_hook_ops(e);
    head = (void *)&ops[num];
    head.allocation = e;
    call_rcu(&head.head, __nf_hook_entries_free);
    }
    static unsigned int accept_all(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    return NF_ACCEPT; /* ACCEPT makes nf_hook_slow call next hook */
    }
    static const struct nf_hook_ops dummy_ops = {
    .hook = accept_all,
    .priority = INT_MIN,
    };
    static struct nf_hook_entries *
    nf_hook_entries_grow(const struct nf_hook_entries *old,
    const struct nf_hook_ops *reg)
    {
    unsigned int i, alloc_entries, nhooks, old_entries;
    struct nf_hook_ops **orig_ops = core::ptr::null_mut();
    struct nf_hook_ops **new_ops;
    struct nf_hook_entries *new;
    let mut inserted: bool = false;
    alloc_entries = 1;
    old_entries = old ? old.num_hook_entries : 0;
    if (old) {
    orig_ops = nf_hook_entries_get_hook_ops(old);
    for (i = 0; i < old_entries; i++) {
    if (orig_ops[i] != &dummy_ops)
    alloc_entries++;
// Restrict BPF hook type to force a unique priority, not
// shared at attach time.
//
// This is mainly to avoid ordering issues between two
// different bpf programs, this doesn't prevent a normal
// hook at same priority as a bpf one (we don't want to
// prevent defrag, conntrack, iptables etc from attaching).
//
    if (reg.priority == orig_ops[i].priority &&
    reg.hook_ops_type == NF_HOOK_OP_BPF)
    return ERR_PTR(-EBUSY);
    }
    }
    if (alloc_entries > MAX_HOOK_COUNT)
    return ERR_PTR(-E2BIG);
    new = allocate_hook_entries_size(alloc_entries);
    if (!new)
    return ERR_PTR(-ENOMEM);
    new_ops = nf_hook_entries_get_hook_ops(new);
    i = 0;
    nhooks = 0;
    while (i < old_entries) {
    if (orig_ops[i] == &dummy_ops) {
    ++i;
    continue;
    }
    if (inserted || reg.priority > orig_ops[i].priority) {
    new_ops[nhooks] = (void *)orig_ops[i];
    new.hooks[nhooks] = old.hooks[i];
    i++;
    } else {
    new_ops[nhooks] = (void *)reg;
    new.hooks[nhooks].hook = reg.hook;
    new.hooks[nhooks].priv = reg.priv;
    inserted = true;
    }
    nhooks++;
    }
    if (!inserted) {
    new_ops[nhooks] = (void *)reg;
    new.hooks[nhooks].hook = reg.hook;
    new.hooks[nhooks].priv = reg.priv;
    }
    return new;
    }
#[no_mangle]
unsafe extern "C" fn hooks_validate(hooks: *const nf_hook_entries) {
    static void hooks_validate(const struct nf_hook_entries *hooks)
    {

    struct nf_hook_ops **orig_ops;
    let mut prio: c_int = INT_MIN;
    let mut i: usize = 0;
    orig_ops = nf_hook_entries_get_hook_ops(hooks);
    for (i = 0; i < hooks.num_hook_entries; i++) {
    if (orig_ops[i] == &dummy_ops)
    continue;
    WARN_ON(orig_ops[i].priority < prio);
    if (orig_ops[i].priority > prio)
    prio = orig_ops[i].priority;
    }

    }
    int nf_hook_entries_insert_raw(struct nf_hook_entries __rcu **pp,
    const struct nf_hook_ops *reg)
    {
    struct nf_hook_entries *new_hooks;
    struct nf_hook_entries *p;
    p = rcu_dereference_raw(*pp);
    new_hooks = nf_hook_entries_grow(p, reg);
    if (IS_ERR(new_hooks))
    return PTR_ERR(new_hooks);
    hooks_validate(new_hooks);
    rcu_assign_pointer(*pp, new_hooks);
    BUG_ON(p == new_hooks);
    nf_hook_entries_free(p);
    return 0;
    }
    EXPORT_SYMBOL_GPL(nf_hook_entries_insert_raw);
//
// __nf_hook_entries_try_shrink - try to shrink hook array
//
// @old -- current hook blob at @pp
// @pp -- location of hook blob
//
// Hook unregistration must always succeed, so to-be-removed hooks
// are replaced by a dummy one that will just move to next hook.
//
// This counts the current dummy hooks, attempts to allocate new blob,
// copies the live hooks, then replaces and discards old one.
//
// return values:
//
// Returns address to free, or NULL.
//
    static void *__nf_hook_entries_try_shrink(struct nf_hook_entries *old,
    struct nf_hook_entries __rcu **pp)
    {
    unsigned int i, j, skip = 0, hook_entries;
    struct nf_hook_entries *new = core::ptr::null_mut();
    struct nf_hook_ops **orig_ops;
    struct nf_hook_ops **new_ops;
    if (WARN_ON_ONCE(!old))
    return core::ptr::null_mut();
    orig_ops = nf_hook_entries_get_hook_ops(old);
    for (i = 0; i < old.num_hook_entries; i++) {
    if (orig_ops[i] == &dummy_ops)
    skip++;
    }
// if skip == hook_entries all hooks have been removed
    hook_entries = old.num_hook_entries;
    if (skip == hook_entries)
    goto out_assign;
    if (skip == 0)
    return core::ptr::null_mut();
    hook_entries -= skip;
    new = allocate_hook_entries_size(hook_entries);
    if (!new)
    return core::ptr::null_mut();
    new_ops = nf_hook_entries_get_hook_ops(new);
    for (i = 0, j = 0; i < old.num_hook_entries; i++) {
    if (orig_ops[i] == &dummy_ops)
    continue;
    new.hooks[j] = old.hooks[i];
    new_ops[j] = (void *)orig_ops[i];
    j++;
    }
    hooks_validate(new);
    out_assign:
    rcu_assign_pointer(*pp, new);
    return old;
    }
    static struct nf_hook_entries __rcu **
    nf_hook_entry_head(struct net *net, int pf, unsigned int hooknum,
    struct net_device *dev)
    {
    switch (pf) {
    case NFPROTO_NETDEV:
    break;

    case NFPROTO_ARP:
    if (WARN_ON_ONCE(ARRAY_SIZE(net.nf.hooks_arp) <= hooknum))
    return core::ptr::null_mut();
    return net.nf.hooks_arp + hooknum;

    case NFPROTO_BRIDGE:
    if (WARN_ON_ONCE(ARRAY_SIZE(net.nf.hooks_bridge) <= hooknum))
    return core::ptr::null_mut();
    return net.nf.hooks_bridge + hooknum;

    case NFPROTO_INET:
    if (WARN_ON_ONCE(hooknum != NF_INET_INGRESS))
    return core::ptr::null_mut();
    if (!dev || dev_net(dev) != net) {
    WARN_ON_ONCE(1);
    return core::ptr::null_mut();
    }
    return &dev.nf_hooks_ingress;

    case NFPROTO_IPV4:
    if (WARN_ON_ONCE(ARRAY_SIZE(net.nf.hooks_ipv4) <= hooknum))
    return core::ptr::null_mut();
    return net.nf.hooks_ipv4 + hooknum;
    case NFPROTO_IPV6:
    if (WARN_ON_ONCE(ARRAY_SIZE(net.nf.hooks_ipv6) <= hooknum))
    return core::ptr::null_mut();
    return net.nf.hooks_ipv6 + hooknum;
    default:
    WARN_ON_ONCE(1);
    return core::ptr::null_mut();
    }

    if (hooknum == NF_NETDEV_INGRESS) {
    if (dev && dev_net(dev) == net)
    return &dev.nf_hooks_ingress;
    }

    if (hooknum == NF_NETDEV_EGRESS) {
    if (dev && dev_net(dev) == net)
    return &dev.nf_hooks_egress;
    }

    WARN_ON_ONCE(1);
    return core::ptr::null_mut();
    }
    static int nf_ingress_check(struct net *net, const struct nf_hook_ops *reg,
    int hooknum)
    {

    if (reg.hooknum == hooknum)
    return -EOPNOTSUPP;

    if (reg.hooknum != hooknum ||
    !reg.dev || dev_net(reg.dev) != net)
    return -EINVAL;
    return 0;
    }
    static inline bool __maybe_unused nf_ingress_hook(const struct nf_hook_ops *reg,
    int pf)
    {
    if ((pf == NFPROTO_NETDEV && reg.hooknum == NF_NETDEV_INGRESS) ||
    (pf == NFPROTO_INET && reg.hooknum == NF_INET_INGRESS))
    return true;
    return false;
    }
    static inline bool __maybe_unused nf_egress_hook(const struct nf_hook_ops *reg,
    int pf)
    {
    let mut pf: return = = NFPROTO_NETDEV && reg.hooknum == NF_NETDEV_EGRESS;
    }
#[no_mangle]
unsafe extern "C" fn nf_static_key_inc(reg: *const nf_hook_ops, pf: c_int) {
    static void nf_static_key_inc(const struct nf_hook_ops *reg, int pf)
    {

    int hooknum;
    if (pf == NFPROTO_INET && reg.hooknum == NF_INET_INGRESS) {
    pf = NFPROTO_NETDEV;
    hooknum = NF_NETDEV_INGRESS;
    } else {
    hooknum = reg.hooknum;
    }
    static_key_slow_inc(&nf_hooks_needed[pf][hooknum]);

    }
#[no_mangle]
unsafe extern "C" fn nf_static_key_dec(reg: *const nf_hook_ops, pf: c_int) {
    static void nf_static_key_dec(const struct nf_hook_ops *reg, int pf)
    {

    int hooknum;
    if (pf == NFPROTO_INET && reg.hooknum == NF_INET_INGRESS) {
    pf = NFPROTO_NETDEV;
    hooknum = NF_NETDEV_INGRESS;
    } else {
    hooknum = reg.hooknum;
    }
    static_key_slow_dec(&nf_hooks_needed[pf][hooknum]);

    }
    static int __nf_register_net_hook(struct net *net, int pf,
    const struct nf_hook_ops *reg)
    {
    struct nf_hook_entries *p, *new_hooks;
    struct nf_hook_entries __rcu **pp;
    int err;
    switch (pf) {
    case NFPROTO_NETDEV:

    if (reg.hooknum == NF_NETDEV_INGRESS)
    return -EOPNOTSUPP;

    if (reg.hooknum == NF_NETDEV_EGRESS)
    return -EOPNOTSUPP;

    if ((reg.hooknum != NF_NETDEV_INGRESS &&
    reg.hooknum != NF_NETDEV_EGRESS) ||
    !reg.dev || dev_net(reg.dev) != net)
    return -EINVAL;
    break;
    case NFPROTO_INET:
    if (reg.hooknum != NF_INET_INGRESS)
    break;
    err = nf_ingress_check(net, reg, NF_INET_INGRESS);
    if (err < 0)
    return err;
    break;
    }
    pp = nf_hook_entry_head(net, pf, reg.hooknum, reg.dev);
    if (!pp)
    return -EINVAL;
    mutex_lock(&nf_hook_mutex);
    p = nf_entry_dereference(*pp);
    new_hooks = nf_hook_entries_grow(p, reg);
    if (!IS_ERR(new_hooks)) {
    hooks_validate(new_hooks);
    rcu_assign_pointer(*pp, new_hooks);
    }
    mutex_unlock(&nf_hook_mutex);
    if (IS_ERR(new_hooks))
    return PTR_ERR(new_hooks);

    if (nf_ingress_hook(reg, pf))
    net_inc_ingress_queue();

    if (nf_egress_hook(reg, pf))
    net_inc_egress_queue();

    nf_static_key_inc(reg, pf);
    BUG_ON(p == new_hooks);
    nf_hook_entries_free(p);
    return 0;
    }
//
// nf_remove_net_hook - remove a hook from blob
//
// @oldp: current address of hook blob
// @unreg: hook to unregister
//
// This cannot fail, hook unregistration must always succeed.
// Therefore replace the to-be-removed hook with a dummy hook.
//
    static bool nf_remove_net_hook(struct nf_hook_entries *old,
    const struct nf_hook_ops *unreg)
    {
    struct nf_hook_ops **orig_ops;
    unsigned int i;
    orig_ops = nf_hook_entries_get_hook_ops(old);
    for (i = 0; i < old.num_hook_entries; i++) {
    if (orig_ops[i] != unreg)
    continue;
    WRITE_ONCE(old.hooks[i].hook, accept_all);
    WRITE_ONCE(orig_ops[i], (void *)&dummy_ops);
    return true;
    }
    return false;
    }
    static void __nf_unregister_net_hook(struct net *net, int pf,
    const struct nf_hook_ops *reg)
    {
    struct nf_hook_entries __rcu **pp;
    struct nf_hook_entries *p;
    pp = nf_hook_entry_head(net, pf, reg.hooknum, reg.dev);
    if (!pp)
    return;
    mutex_lock(&nf_hook_mutex);
    p = nf_entry_dereference(*pp);
    if (WARN_ON_ONCE(!p)) {
    mutex_unlock(&nf_hook_mutex);
    return;
    }
    if (nf_remove_net_hook(p, reg)) {

    if (nf_ingress_hook(reg, pf))
    net_dec_ingress_queue();

    if (nf_egress_hook(reg, pf))
    net_dec_egress_queue();

    nf_static_key_dec(reg, pf);
    } else {
    WARN_ONCE(1, "hook not found, pf %d num %d", pf, reg.hooknum);
    }
    p = __nf_hook_entries_try_shrink(p, pp);
    mutex_unlock(&nf_hook_mutex);
    if (!p)
    return;
    nf_queue_nf_hook_drop(net);
    nf_hook_entries_free(p);
    }
#[no_mangle]
pub unsafe extern "C" fn nf_unregister_net_hook(net: *mut net, reg: *const nf_hook_ops) {
    void nf_unregister_net_hook(struct net *net, const struct nf_hook_ops *reg)
    {
    if (reg.pf == NFPROTO_INET) {
    if (reg.hooknum == NF_INET_INGRESS) {
    __nf_unregister_net_hook(net, NFPROTO_INET, reg);
    } else {
    __nf_unregister_net_hook(net, NFPROTO_IPV4, reg);
    __nf_unregister_net_hook(net, NFPROTO_IPV6, reg);
    }
    } else {
    __nf_unregister_net_hook(net, reg.pf, reg);
    }
    }
    EXPORT_SYMBOL(nf_unregister_net_hook);
    void nf_hook_entries_delete_raw(struct nf_hook_entries __rcu **pp,
    const struct nf_hook_ops *reg)
    {
    struct nf_hook_entries *p;
    p = rcu_dereference_raw(*pp);
    if (nf_remove_net_hook(p, reg)) {
    p = __nf_hook_entries_try_shrink(p, pp);
    nf_hook_entries_free(p);
    }
    }
    EXPORT_SYMBOL_GPL(nf_hook_entries_delete_raw);
#[no_mangle]
pub unsafe extern "C" fn nf_register_net_hook(net: *mut net, reg: *const nf_hook_ops) -> c_int {
    int nf_register_net_hook(struct net *net, const struct nf_hook_ops *reg)
    {
    int err;
    if (reg.pf == NFPROTO_INET) {
    if (reg.hooknum == NF_INET_INGRESS) {
    err = __nf_register_net_hook(net, NFPROTO_INET, reg);
    if (err < 0)
    return err;
    } else {
    err = __nf_register_net_hook(net, NFPROTO_IPV4, reg);
    if (err < 0)
    return err;
    err = __nf_register_net_hook(net, NFPROTO_IPV6, reg);
    if (err < 0) {
    __nf_unregister_net_hook(net, NFPROTO_IPV4, reg);
    return err;
    }
    }
    } else {
    err = __nf_register_net_hook(net, reg.pf, reg);
    if (err < 0)
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL(nf_register_net_hook);
    int nf_register_net_hooks(struct net *net, const struct nf_hook_ops *reg,
    unsigned int n)
    {
    unsigned int i;
    let mut err: c_int = 0;
    for (i = 0; i < n; i++) {
    err = nf_register_net_hook(net, &reg[i]);
    if (err)
    goto err;
    }
    return err;
    err:
    if (i > 0)
    nf_unregister_net_hooks(net, reg, i);
    return err;
    }
    EXPORT_SYMBOL(nf_register_net_hooks);
    void nf_unregister_net_hooks(struct net *net, const struct nf_hook_ops *reg,
    unsigned int hookcount)
    {
    unsigned int i;
    for (i = 0; i < hookcount; i++)
    nf_unregister_net_hook(net, &reg[i]);
    }
    EXPORT_SYMBOL(nf_unregister_net_hooks);
// Returns 1 if okfn() needs to be executed by the caller,
// -EPERM for NF_DROP, 0 otherwise.  Caller must hold rcu_read_lock.
    int nf_hook_slow(struct sk_buff *skb, struct nf_hook_state *state,
    const struct nf_hook_entries *e, unsigned int s)
    {
    unsigned int verdict;
    int ret;
    for (; s < e.num_hook_entries; s++) {
    verdict = nf_hook_entry_hookfn(&e.hooks[s], skb, state);
    switch (verdict & NF_VERDICT_MASK) {
    case NF_ACCEPT:
    break;
    case NF_DROP:
    kfree_skb_reason(skb,
    SKB_DROP_REASON_NETFILTER_DROP);
    ret = NF_DROP_GETERR(verdict);
    if (ret == 0)
    ret = -EPERM;
    return ret;
    case NF_QUEUE:
    ret = nf_queue(skb, state, s, verdict);
    if (ret == 1)
    continue;
    return ret;
    case NF_STOLEN:
    return NF_DROP_GETERR(verdict);
    default:
    WARN_ON_ONCE(1);
    return 0;
    }
    }
    return 1;
    }
    EXPORT_SYMBOL(nf_hook_slow);
    void nf_hook_slow_list(struct list_head *head, struct nf_hook_state *state,
    const struct nf_hook_entries *e)
    {
    struct sk_buff *skb, *next;
    LIST_HEAD(sublist);
    int ret;
    list_for_each_entry_safe(skb, next, head, list) {
    skb_list_del_init(skb);
    ret = nf_hook_slow(skb, state, e, 0);
    if (ret == 1)
    list_add_tail(&skb.list, &sublist);
    }
// Put passed packets back on main list
    list_splice(&sublist, head);
    }
    EXPORT_SYMBOL(nf_hook_slow_list);
// This needs to be compiled in any case to avoid dependencies between the
// nfnetlink_queue code and nf_conntrack.
//
    const struct nfnl_ct_hook __rcu *nfnl_ct_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nfnl_ct_hook);
    const struct nf_ct_hook __rcu *nf_ct_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_ct_hook);
    const struct nf_defrag_hook __rcu *nf_defrag_v4_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_defrag_v4_hook);
    const struct nf_defrag_hook __rcu *nf_defrag_v6_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_defrag_v6_hook);

    u8 nf_ctnetlink_has_listener;
    EXPORT_SYMBOL_GPL(nf_ctnetlink_has_listener);
    const struct nf_nat_hook __rcu *nf_nat_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_nat_hook);
// This does not belong here, but locally generated errors need it if connection
// tracking in use: without this, connection may not be in hash table, and hence
// manufactured ICMP or RST packets will not be associated with it.
//
#[no_mangle]
pub unsafe extern "C" fn nf_ct_attach(new: *mut sk_buff, skb: *const sk_buff) {
    void nf_ct_attach(struct sk_buff *new, const struct sk_buff *skb)
    {
    const struct nf_ct_hook *ct_hook;
    if (skb._nfct) {
    rcu_read_lock();
    ct_hook = rcu_dereference(nf_ct_hook);
    if (ct_hook)
    ct_hook.attach(new, skb);
    rcu_read_unlock();
    }
    }
    EXPORT_SYMBOL(nf_ct_attach);
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_destroy(nfct: *mut nf_conntrack) {
    void nf_conntrack_destroy(struct nf_conntrack *nfct)
    {
    const struct nf_ct_hook *ct_hook;
    rcu_read_lock();
    ct_hook = rcu_dereference(nf_ct_hook);
    if (ct_hook)
    ct_hook.destroy(nfct);
    rcu_read_unlock();
    WARN_ON(!ct_hook);
    }
    EXPORT_SYMBOL(nf_conntrack_destroy);
#[no_mangle]
pub unsafe extern "C" fn nf_ct_set_closing(nfct: *mut nf_conntrack) {
    void nf_ct_set_closing(struct nf_conntrack *nfct)
    {
    const struct nf_ct_hook *ct_hook;
    if (!nfct)
    return;
    rcu_read_lock();
    ct_hook = rcu_dereference(nf_ct_hook);
    if (ct_hook)
    ct_hook.set_closing(nfct);
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(nf_ct_set_closing);
    bool nf_ct_get_tuple_skb(struct nf_conntrack_tuple *dst_tuple,
    const struct sk_buff *skb)
    {
    const struct nf_ct_hook *ct_hook;
    let mut ret: bool = false;
    rcu_read_lock();
    ct_hook = rcu_dereference(nf_ct_hook);
    if (ct_hook)
    ret = ct_hook.get_tuple_skb(dst_tuple, skb);
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL(nf_ct_get_tuple_skb);
// Built-in default zone used e.g. by modules.
    const struct nf_conntrack_zone nf_ct_zone_dflt = {
    .id	= NF_CT_DEFAULT_ZONE_ID,
    .dir	= NF_CT_DEFAULT_ZONE_DIR,
    };
    EXPORT_SYMBOL_GPL(nf_ct_zone_dflt);

    static void __net_init
    __netfilter_net_init(struct nf_hook_entries __rcu **e, int max)
    {
    int h;
    for (h = 0; h < max; h++)
    RCU_INIT_POINTER(e[h], core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn netfilter_net_init(net: *mut net) -> int __net_init {
    static int __net_init netfilter_net_init(struct net *net)
    {
    __netfilter_net_init(net.nf.hooks_ipv4, ARRAY_SIZE(net.nf.hooks_ipv4));
    __netfilter_net_init(net.nf.hooks_ipv6, ARRAY_SIZE(net.nf.hooks_ipv6));

    __netfilter_net_init(net.nf.hooks_arp, ARRAY_SIZE(net.nf.hooks_arp));

    __netfilter_net_init(net.nf.hooks_bridge, ARRAY_SIZE(net.nf.hooks_bridge));

    net.nf.proc_netfilter = proc_net_mkdir(net, "netfilter",
    net.proc_net);
    if (!net.nf.proc_netfilter) {
    if (!net_eq(net, &init_net))
    pr_err("cannot create netfilter proc entry");
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netfilter_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit netfilter_net_exit(struct net *net)
    {
    remove_proc_entry("netfilter", net.proc_net);
    }
    static struct pernet_operations netfilter_net_ops = {
    .init = netfilter_net_init,
    .exit = netfilter_net_exit,
    };
#[no_mangle]
pub unsafe extern "C" fn netfilter_init() -> int __init {
    int __init netfilter_init(void)
    {
    int ret;
    ret = register_pernet_subsys(&netfilter_net_ops);
    if (ret < 0)
    goto err;

    ret = netfilter_lwtunnel_init();
    if (ret < 0)
    goto err_lwtunnel_pernet;

    ret = netfilter_log_init();
    if (ret < 0)
    goto err_log_pernet;
    return 0;
    err_log_pernet:

    netfilter_lwtunnel_fini();
    err_lwtunnel_pernet:

    unregister_pernet_subsys(&netfilter_net_ops);
    err:
    return ret;
    }
