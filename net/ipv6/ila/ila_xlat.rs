//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ila/ila_xlat.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_xlat_params {
    pub ip: ila_params,
    pub ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_map {
    pub xp: ila_xlat_params,
    pub node: rhash_head,
    pub next: *mut ila_map __rcu,
    pub rcu: rcu_head,
}

pub const MAX_LOCKS: c_int = 1024;
pub const LOCKS_PER_CPU: c_int = 10;
#[no_mangle]
unsafe extern "C" fn alloc_ila_locks(ilan: *mut ila_net) -> c_int {
    static int alloc_ila_locks(struct ila_net *ilan)
    {
    return alloc_bucket_spinlocks(&ilan.xlat.locks, &ilan.xlat.locks_mask,
    MAX_LOCKS, LOCKS_PER_CPU,
    GFP_KERNEL);
    }
    static u32 hashrnd __read_mostly;
#[no_mangle]
unsafe extern "C" fn __ila_hash_secret_init() -> __always_inline void {
    static __always_inline void __ila_hash_secret_init(void)
    {
    net_get_random_once(&hashrnd, sizeof(hashrnd));
    }
#[no_mangle]
pub unsafe extern "C" fn ila_locator_hash(loc: ila_locator) -> u32 {
    static inline u32 ila_locator_hash(struct ila_locator loc)
    {
    u32 *v = (u32 *)loc.v32;
    __ila_hash_secret_init();
    return jhash_2words(v[0], v[1], hashrnd);
    }
    static inline spinlock_t *ila_get_lock(struct ila_net *ilan,
    struct ila_locator loc)
    {
    return &ilan.xlat.locks[ila_locator_hash(loc) & ilan.xlat.locks_mask];
    }
    static inline int ila_cmp_wildcards(struct ila_map *ila,
    struct ila_addr *iaddr, int ifindex)
    {
    return (ila.xp.ifindex && ila.xp.ifindex != ifindex);
    }
    static inline int ila_cmp_params(struct ila_map *ila,
    struct ila_xlat_params *xp)
    {
    return (ila.xp.ifindex != xp.ifindex);
    }
    static int ila_cmpfn(struct rhashtable_compare_arg *arg,
    const void *obj)
    {
    const struct ila_map *ila = obj;
    return (ila.xp.ip.locator_match.v64 != *(__be64 *)arg.key);
    }
#[no_mangle]
pub unsafe extern "C" fn ila_order(ila: *mut ila_map) -> c_int {
    static inline int ila_order(struct ila_map *ila)
    {
    let mut score: c_int = 0;
    if (ila.xp.ifindex)
    score += 1 << 1;
    return score;
    }
    static const struct rhashtable_params rht_params = {
    .nelem_hint = 1024,
    .head_offset = offsetof(struct ila_map, node),
    .key_offset = offsetof(struct ila_map, xp.ip.locator_match),
    .key_len = sizeof(u64), /* identifier */
    .max_size = 1048576,
    .min_size = 256,
    .automatic_shrinking = true,
    .obj_cmpfn = ila_cmpfn,
    };
    static int parse_nl_config(struct genl_info *info,
    struct ila_xlat_params *xp)
    {
    memset(xp, 0, sizeof(*xp));
    if (info.attrs[ILA_ATTR_LOCATOR])
    xp.ip.locator.v64 = ( __be64)nla_get_u64(
    info.attrs[ILA_ATTR_LOCATOR]);
    if (info.attrs[ILA_ATTR_LOCATOR_MATCH])
    xp.ip.locator_match.v64 = ( __be64)nla_get_u64(
    info.attrs[ILA_ATTR_LOCATOR_MATCH]);
    xp.ip.csum_mode = nla_get_u8_default(info.attrs[ILA_ATTR_CSUM_MODE],
    ILA_CSUM_NO_ACTION);
    xp.ip.ident_type = nla_get_u8_default(info.attrs[ILA_ATTR_IDENT_TYPE],
    ILA_ATYPE_USE_FORMAT);
    if (info.attrs[ILA_ATTR_IFINDEX])
    xp.ifindex = nla_get_s32(info.attrs[ILA_ATTR_IFINDEX]);
    return 0;
    }
// Must be called with rcu readlock
    static inline struct ila_map *ila_lookup_wildcards(struct ila_addr *iaddr,
    int ifindex,
    struct ila_net *ilan)
    {
    struct ila_map *ila;
    ila = rhashtable_lookup_fast(&ilan.xlat.rhash_table, &iaddr.loc,
    rht_params);
    while (ila) {
    if (!ila_cmp_wildcards(ila, iaddr, ifindex))
    return ila;
    ila = rcu_access_pointer(ila.next);
    }
    return core::ptr::null_mut();
    }
// Must be called with rcu readlock
    static inline struct ila_map *ila_lookup_by_params(struct ila_xlat_params *xp,
    struct ila_net *ilan)
    {
    struct ila_map *ila;
    ila = rhashtable_lookup_fast(&ilan.xlat.rhash_table,
    &xp.ip.locator_match,
    rht_params);
    while (ila) {
    if (!ila_cmp_params(ila, xp))
    return ila;
    ila = rcu_access_pointer(ila.next);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ila_release(ila: *mut ila_map) {
    static inline void ila_release(struct ila_map *ila)
    {
    kfree_rcu(ila, rcu);
    }
#[no_mangle]
unsafe extern "C" fn ila_free_node(ila: *mut ila_map) {
    static void ila_free_node(struct ila_map *ila)
    {
    struct ila_map *next;
// Assume rcu_readlock held
    while (ila) {
    next = rcu_access_pointer(ila.next);
    ila_release(ila);
    ila = next;
    }
    }
#[no_mangle]
unsafe extern "C" fn ila_free_cb(ptr: *mut c_void, arg: *mut c_void) {
    static void ila_free_cb(void *ptr, void *arg)
    {
    ila_free_node((struct ila_map *)ptr);
    }
    static int ila_xlat_addr(struct sk_buff *skb, bool sir2ila);
    static unsigned int
    ila_nf_input(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    ila_xlat_addr(skb, false);
    return NF_ACCEPT;
    }
    static const struct nf_hook_ops ila_nf_hook_ops[] = {
    {
    .hook = ila_nf_input,
    .pf = NFPROTO_IPV6,
    .hooknum = NF_INET_PRE_ROUTING,
    .priority = -1,
    },
    };
    static DEFINE_MUTEX(ila_mutex);
#[no_mangle]
unsafe extern "C" fn ila_add_mapping(net: *mut net, xp: *mut ila_xlat_params) -> c_int {
    static int ila_add_mapping(struct net *net, struct ila_xlat_params *xp)
    {
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct ila_map *ila, *head;
    spinlock_t *lock = ila_get_lock(ilan, xp.ip.locator_match);
    let mut err: c_int = 0, order;
    if (!READ_ONCE(ilan.xlat.hooks_registered)) {
// We defer registering net hooks in the namespace until the
// first mapping is added.
//
    mutex_lock(&ila_mutex);
    if (!ilan.xlat.hooks_registered) {
    err = nf_register_net_hooks(net, ila_nf_hook_ops,
    ARRAY_SIZE(ila_nf_hook_ops));
    if (!err)
    WRITE_ONCE(ilan.xlat.hooks_registered, true);
    }
    mutex_unlock(&ila_mutex);
    if (err)
    return err;
    }
    ila = kzalloc_obj(*ila);
    if (!ila)
    return -ENOMEM;
    ila_init_saved_csum(&xp.ip);
    ila.xp = *xp;
    order = ila_order(ila);
    spin_lock(lock);
    head = rhashtable_lookup_fast(&ilan.xlat.rhash_table,
    &xp.ip.locator_match,
    rht_params);
    if (!head) {
// New entry for the rhash_table
    err = rhashtable_lookup_insert_fast(&ilan.xlat.rhash_table,
    &ila.node, rht_params);
    } else {
    struct ila_map *tila = head, *prev = core::ptr::null_mut();
    do {
    if (!ila_cmp_params(tila, xp)) {
    err = -EEXIST;
    goto out;
    }
    if (order > ila_order(tila))
    break;
    prev = tila;
    tila = rcu_dereference_protected(tila.next,
    lockdep_is_held(lock));
    } while (tila);
    if (prev) {
// Insert in sub list of head
    RCU_INIT_POINTER(ila.next, tila);
    rcu_assign_pointer(prev.next, ila);
    } else {
// Make this ila new head
    RCU_INIT_POINTER(ila.next, head);
    err = rhashtable_replace_fast(&ilan.xlat.rhash_table,
    &head.node,
    &ila.node, rht_params);
    if (err)
    goto out;
    }
    }
    out:
    spin_unlock(lock);
    if (err)
    kfree(ila);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ila_del_mapping(net: *mut net, xp: *mut ila_xlat_params) -> c_int {
    static int ila_del_mapping(struct net *net, struct ila_xlat_params *xp)
    {
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct ila_map *ila, *head, *prev;
    spinlock_t *lock = ila_get_lock(ilan, xp.ip.locator_match);
    let mut err: c_int = -ENOENT;
    spin_lock(lock);
    head = rhashtable_lookup_fast(&ilan.xlat.rhash_table,
    &xp.ip.locator_match, rht_params);
    ila = head;
    prev = core::ptr::null_mut();
    while (ila) {
    if (ila_cmp_params(ila, xp)) {
    prev = ila;
    ila = rcu_dereference_protected(ila.next,
    lockdep_is_held(lock));
    continue;
    }
    err = 0;
    if (prev) {
// Not head, just delete from list
    rcu_assign_pointer(prev.next, ila.next);
    } else {
// It is the head. If there is something in the
// sublist we need to make a new head.
//
    head = rcu_dereference_protected(ila.next,
    lockdep_is_held(lock));
    if (head) {
// Put first entry in the sublist into the
// table
//
    err = rhashtable_replace_fast(
    &ilan.xlat.rhash_table, &ila.node,
    &head.node, rht_params);
    if (err)
    goto out;
    } else {
// Entry no longer used
    err = rhashtable_remove_fast(
    &ilan.xlat.rhash_table,
    &ila.node, rht_params);
    }
    }
    ila_release(ila);
    break;
    }
    out:
    spin_unlock(lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_cmd_add_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ila_xlat_nl_cmd_add_mapping(struct sk_buff *skb, struct genl_info *info)
    {
    struct net *net = genl_info_net(info);
    struct ila_xlat_params p;
    int err;
    err = parse_nl_config(info, &p);
    if (err)
    return err;
    return ila_add_mapping(net, &p);
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_cmd_del_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ila_xlat_nl_cmd_del_mapping(struct sk_buff *skb, struct genl_info *info)
    {
    struct net *net = genl_info_net(info);
    struct ila_xlat_params xp;
    int err;
    err = parse_nl_config(info, &xp);
    if (err)
    return err;
    ila_del_mapping(net, &xp);
    return 0;
    }
    static inline spinlock_t *lock_from_ila_map(struct ila_net *ilan,
    struct ila_map *ila)
    {
    return ila_get_lock(ilan, ila.xp.ip.locator_match);
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_cmd_flush(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ila_xlat_nl_cmd_flush(struct sk_buff *skb, struct genl_info *info)
    {
    struct net *net = genl_info_net(info);
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct rhashtable_iter iter;
    struct ila_map *ila;
    spinlock_t *lock;
    let mut ret: c_int = 0;
    rhashtable_walk_enter(&ilan.xlat.rhash_table, &iter);
    rhashtable_walk_start(&iter);
    for (;;) {
    ila = rhashtable_walk_next(&iter);
    if (IS_ERR(ila)) {
    if (PTR_ERR(ila) == -EAGAIN)
    continue;
    ret = PTR_ERR(ila);
    goto done;
    } else if (!ila) {
    break;
    }
    lock = lock_from_ila_map(ilan, ila);
    spin_lock(lock);
    ret = rhashtable_remove_fast(&ilan.xlat.rhash_table,
    &ila.node, rht_params);
    if (!ret)
    ila_free_node(ila);
    spin_unlock(lock);
    if (ret)
    break;
    }
    done:
    rhashtable_walk_stop(&iter);
    rhashtable_walk_exit(&iter);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ila_fill_info(ila: *mut ila_map, msg: *mut sk_buff) -> c_int {
    static int ila_fill_info(struct ila_map *ila, struct sk_buff *msg)
    {
    if (nla_put_u64_64bit(msg, ILA_ATTR_LOCATOR,
    ( u64)ila.xp.ip.locator.v64,
    ILA_ATTR_PAD) ||
    nla_put_u64_64bit(msg, ILA_ATTR_LOCATOR_MATCH,
    ( u64)ila.xp.ip.locator_match.v64,
    ILA_ATTR_PAD) ||
    nla_put_s32(msg, ILA_ATTR_IFINDEX, ila.xp.ifindex) ||
    nla_put_u8(msg, ILA_ATTR_CSUM_MODE, ila.xp.ip.csum_mode) ||
    nla_put_u8(msg, ILA_ATTR_IDENT_TYPE, ila.xp.ip.ident_type))
    return -1;
    return 0;
    }
    static int ila_dump_info(struct ila_map *ila,
    u32 portid, u32 seq, u32 flags,
    struct sk_buff *skb, u8 cmd)
    {
    void *hdr;
    hdr = genlmsg_put(skb, portid, seq, &ila_nl_family, flags, cmd);
    if (!hdr)
    return -ENOMEM;
    if (ila_fill_info(ila, skb) < 0)
    goto nla_put_failure;
    genlmsg_end(skb, hdr);
    return 0;
    nla_put_failure:
    genlmsg_cancel(skb, hdr);
    return -EMSGSIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_cmd_get_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int ila_xlat_nl_cmd_get_mapping(struct sk_buff *skb, struct genl_info *info)
    {
    struct net *net = genl_info_net(info);
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct sk_buff *msg;
    struct ila_xlat_params xp;
    struct ila_map *ila;
    int ret;
    ret = parse_nl_config(info, &xp);
    if (ret)
    return ret;
    msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if (!msg)
    return -ENOMEM;
    rcu_read_lock();
    ret = -ESRCH;
    ila = ila_lookup_by_params(&xp, ilan);
    if (ila) {
    ret = ila_dump_info(ila,
    info.snd_portid,
    info.snd_seq, 0, msg,
    info.genlhdr.cmd);
    }
    rcu_read_unlock();
    if (ret < 0)
    goto out_free;
    return genlmsg_reply(msg, info);
    out_free:
    nlmsg_free(msg);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_dump_iter {
    pub rhiter: rhashtable_iter,
    pub skip: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_dump_start(cb: *mut netlink_callback) -> c_int {
    int ila_xlat_nl_dump_start(struct netlink_callback *cb)
    {
    struct net *net = sock_net(cb.skb.sk);
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct ila_dump_iter *iter;
    iter = kmalloc_obj(*iter);
    if (!iter)
    return -ENOMEM;
    rhashtable_walk_enter(&ilan.xlat.rhash_table, &iter.rhiter);
    iter.skip = 0;
    cb.args[0] = (long)iter;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_dump_done(cb: *mut netlink_callback) -> c_int {
    int ila_xlat_nl_dump_done(struct netlink_callback *cb)
    {
    struct ila_dump_iter *iter = (struct ila_dump_iter *)cb.args[0];
    rhashtable_walk_exit(&iter.rhiter);
    kfree(iter);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_nl_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int ila_xlat_nl_dump(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct ila_dump_iter *iter = (struct ila_dump_iter *)cb.args[0];
    struct rhashtable_iter *rhiter = &iter.rhiter;
    let mut skip: c_int = iter.skip;
    struct ila_map *ila;
    int ret;
    rhashtable_walk_start(rhiter);
// Get first entry
    ila = rhashtable_walk_peek(rhiter);
    if (ila && !IS_ERR(ila) && skip) {
// Skip over visited entries
    while (ila && skip) {
// Skip over any ila entries in this list that we
// have already dumped.
//
    ila = rcu_access_pointer(ila.next);
    skip--;
    }
    }
    skip = 0;
    for (;;) {
    if (IS_ERR(ila)) {
    ret = PTR_ERR(ila);
    if (ret == -EAGAIN) {
// Table has changed and iter has reset. Return
// -EAGAIN to the application even if we have
// written data to the skb. The application
// needs to deal with this.
//
    goto out_ret;
    } else {
    break;
    }
    } else if (!ila) {
    ret = 0;
    break;
    }
    while (ila) {
    ret =  ila_dump_info(ila, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, NLM_F_MULTI,
    skb, ILA_CMD_GET);
    if (ret)
    goto out;
    skip++;
    ila = rcu_access_pointer(ila.next);
    }
    skip = 0;
    ila = rhashtable_walk_next(rhiter);
    }
    out:
    iter.skip = skip;
    ret = (skb.len ? : ret);
    out_ret:
    rhashtable_walk_stop(rhiter);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_init_net(net: *mut net) -> c_int {
    int ila_xlat_init_net(struct net *net)
    {
    struct ila_net *ilan = net_generic(net, ila_net_id);
    int err;
    err = alloc_ila_locks(ilan);
    if (err)
    return err;
    err = rhashtable_init(&ilan.xlat.rhash_table, &rht_params);
    if (err) {
    free_bucket_spinlocks(ilan.xlat.locks);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_pre_exit_net(net: *mut net) {
    void ila_xlat_pre_exit_net(struct net *net)
    {
    struct ila_net *ilan = net_generic(net, ila_net_id);
    if (ilan.xlat.hooks_registered)
    nf_unregister_net_hooks(net, ila_nf_hook_ops,
    ARRAY_SIZE(ila_nf_hook_ops));
    }
#[no_mangle]
pub unsafe extern "C" fn ila_xlat_exit_net(net: *mut net) {
    void ila_xlat_exit_net(struct net *net)
    {
    struct ila_net *ilan = net_generic(net, ila_net_id);
    rhashtable_free_and_destroy(&ilan.xlat.rhash_table, ila_free_cb, core::ptr::null_mut());
    free_bucket_spinlocks(ilan.xlat.locks);
    }
#[no_mangle]
unsafe extern "C" fn ila_xlat_addr(skb: *mut sk_buff, sir2ila: bool) -> c_int {
    static int ila_xlat_addr(struct sk_buff *skb, bool sir2ila)
    {
    struct ila_map *ila;
    struct ipv6hdr *ip6h = ipv6_hdr(skb);
    struct net *net = dev_net(skb.dev);
    struct ila_net *ilan = net_generic(net, ila_net_id);
    struct ila_addr *iaddr = ila_a2i(&ip6h.daddr);
// Assumes skb contains a valid IPv6 header that is pulled
// No check here that ILA type in the mapping matches what is in the
// address. We assume that whatever sender gaves us can be translated.
// The checksum mode however is relevant.
//
    rcu_read_lock();
    ila = ila_lookup_wildcards(iaddr, skb.dev.ifindex, ilan);
    if (ila)
    ila_update_ipv6_locator(skb, &ila.xp.ip, sir2ila);
    rcu_read_unlock();
    return 0;
    }
