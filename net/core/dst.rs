//! Automatically rewritten from C to Rust
//! Source: net/core/dst.c
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
//
// net/core/dst.c	Protocol independent destination cache.
//
// Authors:		Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

#[no_mangle]
pub unsafe extern "C" fn dst_discard_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int dst_discard_out(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    kfree_skb(skb);
    return 0;
    }
    EXPORT_SYMBOL(dst_discard_out);
    const struct dst_metrics dst_default_metrics = {
// This initializer is needed to force linker to place this variable
// into const section. Otherwise it might end into bss section.
// We really want to avoid false sharing on this variable, and catch
// any writes on it.
//
    .refcnt = REFCOUNT_INIT(1),
    };
    EXPORT_SYMBOL(dst_default_metrics);
    void dst_init(struct dst_entry *dst, struct dst_ops *ops,
    struct net_device *dev, int initial_obsolete,
    unsigned short flags)
    {
    dst.dev = dev;
    netdev_hold(dev, &dst.dev_tracker, GFP_ATOMIC);
    dst.ops = ops;
    dst_init_metrics(dst, dst_default_metrics.metrics, true);
    dst.expires = 0UL;

    dst.xfrm = core::ptr::null_mut();

    dst.input = dst_discard;
    dst.output = dst_discard_out;
    dst.error = 0;
    dst.obsolete = initial_obsolete;
    dst.header_len = 0;
    dst.trailer_len = 0;

    dst.tclassid = 0;

    dst.lwtstate = core::ptr::null_mut();
    rcuref_init(&dst.__rcuref, 1);
    INIT_LIST_HEAD(&dst.rt_uncached);
    dst.rt_uncached_list = core::ptr::null_mut();
    dst.__use = 0;
    dst.lastuse = jiffies;
    dst.flags = flags;
    if (!(flags & DST_NOCOUNT))
    dst_entries_add(ops, 1);
    }
    EXPORT_SYMBOL(dst_init);
    void *dst_alloc(struct dst_ops *ops, struct net_device *dev,
    int initial_obsolete, unsigned short flags)
    {
    struct dst_entry *dst;
    if (ops.gc &&
    !(flags & DST_NOCOUNT) &&
    dst_entries_get_fast(ops) > ops.gc_thresh)
    ops.gc(ops);
    dst = kmem_cache_alloc(ops.kmem_cachep, GFP_ATOMIC);
    if (!dst)
    return core::ptr::null_mut();
    dst_init(dst, ops, dev, initial_obsolete, flags);
    return dst;
    }
    EXPORT_SYMBOL(dst_alloc);
#[no_mangle]
unsafe extern "C" fn dst_destroy(dst: *mut dst_entry) {
    static void dst_destroy(struct dst_entry *dst)
    {
    struct dst_entry *child = core::ptr::null_mut();
    smp_rmb();

    if (dst.xfrm) {
    struct xfrm_dst *xdst = (struct xfrm_dst *) dst;
    child = xdst.child;
    }

    if (dst.ops.destroy)
    dst.ops.destroy(dst);
    netdev_put(dst.dev, &dst.dev_tracker);
    lwtstate_put(dst.lwtstate);
    if (dst.flags & DST_METADATA)
    metadata_dst_free((struct metadata_dst *)dst);
    else
    kmem_cache_free(dst.ops.kmem_cachep, dst);
    dst = child;
    if (dst)
    dst_release_immediate(dst);
    }
#[no_mangle]
unsafe extern "C" fn dst_destroy_rcu(head: *mut rcu_head) {
    static void dst_destroy_rcu(struct rcu_head *head)
    {
    struct dst_entry *dst = container_of(head, struct dst_entry, rcu_head);
    dst_destroy(dst);
    }
// Operations to mark dst as DEAD and clean up the net device referenced
// by dst:
// 1. put the dst under blackhole interface and discard all tx/rx packets
// on this route.
// 2. release the net_device
// This function should be called when removing routes from the fib tree
// in preparation for a NETDEV_DOWN/NETDEV_UNREGISTER event and also to
// make the next dst_ops->check() fail.
//
#[no_mangle]
pub unsafe extern "C" fn dst_dev_put(dst: *mut dst_entry) {
    void dst_dev_put(struct dst_entry *dst)
    {
    struct net_device *dev = dst.dev;
    WRITE_ONCE(dst.obsolete, DST_OBSOLETE_DEAD);
    if (dst.ops.ifdown)
    dst.ops.ifdown(dst, dev);
    WRITE_ONCE(dst.input, dst_discard);
    WRITE_ONCE(dst.output, dst_discard_out);
    rcu_assign_pointer(dst.dev_rcu, blackhole_netdev);
    netdev_ref_replace(dev, blackhole_netdev, &dst.dev_tracker,
    GFP_ATOMIC);
    }
    EXPORT_SYMBOL(dst_dev_put);
#[no_mangle]
unsafe extern "C" fn dst_count_dec(dst: *mut dst_entry) {
    static void dst_count_dec(struct dst_entry *dst)
    {
    if (!(dst.flags & DST_NOCOUNT))
    dst_entries_add(dst.ops, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn dst_release(dst: *mut dst_entry) {
    void dst_release(struct dst_entry *dst)
    {
    if (dst && rcuref_put(&dst.__rcuref)) {

    if (dst.flags & DST_METADATA) {
    struct metadata_dst *md_dst = (struct metadata_dst *)dst;
    if (md_dst.type == METADATA_IP_TUNNEL)
    dst_cache_reset_now(&md_dst.u.tun_info.dst_cache);
    }

    dst_count_dec(dst);
    call_rcu_hurry(&dst.rcu_head, dst_destroy_rcu);
    }
    }
    EXPORT_SYMBOL(dst_release);
#[no_mangle]
pub unsafe extern "C" fn dst_release_immediate(dst: *mut dst_entry) {
    void dst_release_immediate(struct dst_entry *dst)
    {
    if (dst && rcuref_put(&dst.__rcuref)) {
    dst_count_dec(dst);
    dst_destroy(dst);
    }
    }
    EXPORT_SYMBOL(dst_release_immediate);
    u32 *dst_cow_metrics_generic(struct dst_entry *dst, unsigned long old)
    {
    struct dst_metrics *p = kmalloc_obj(*p, GFP_ATOMIC);
    if (p) {
    struct dst_metrics *old_p = (struct dst_metrics *)__DST_METRICS_PTR(old);
    unsigned long prev, new;
    refcount_set(&p.refcnt, 1);
    memcpy(p.metrics, old_p.metrics, sizeof(p.metrics));
    new = (unsigned long) p;
    prev = cmpxchg(&dst._metrics, old, new);
    if (prev != old) {
    kfree(p);
    p = (struct dst_metrics *)__DST_METRICS_PTR(prev);
    if (prev & DST_METRICS_READ_ONLY)
    p = core::ptr::null_mut();
    } else if (prev & DST_METRICS_REFCOUNTED) {
    if (refcount_dec_and_test(&old_p.refcnt))
    kfree(old_p);
    }
    }
    BUILD_BUG_ON(offsetof(struct dst_metrics, metrics) != 0);
    return (u32 *)p;
    }
    EXPORT_SYMBOL(dst_cow_metrics_generic);
// Caller asserts that dst_metrics_read_only(dst) is false.
#[no_mangle]
pub unsafe extern "C" fn __dst_destroy_metrics_generic(dst: *mut dst_entry, old: c_ulong) {
    void __dst_destroy_metrics_generic(struct dst_entry *dst, unsigned long old)
    {
    unsigned long prev, new;
    new = ((unsigned long) &dst_default_metrics) | DST_METRICS_READ_ONLY;
    prev = cmpxchg(&dst._metrics, old, new);
    if (prev == old)
    kfree(__DST_METRICS_PTR(old));
    }
    EXPORT_SYMBOL(__dst_destroy_metrics_generic);
    struct dst_entry *dst_blackhole_check(struct dst_entry *dst, u32 cookie)
    {
    return core::ptr::null_mut();
    }
    u32 *dst_blackhole_cow_metrics(struct dst_entry *dst, unsigned long old)
    {
    return core::ptr::null_mut();
    }
    struct neighbour *dst_blackhole_neigh_lookup(const struct dst_entry *dst,
    struct sk_buff *skb,
    const void *daddr)
    {
    return core::ptr::null_mut();
    }
    void dst_blackhole_update_pmtu(struct dst_entry *dst, struct sock *sk,
    struct sk_buff *skb, u32 mtu,
    bool confirm_neigh)
    {
    }
    EXPORT_SYMBOL_GPL(dst_blackhole_update_pmtu);
    void dst_blackhole_redirect(struct dst_entry *dst, struct sock *sk,
    struct sk_buff *skb)
    {
    }
    EXPORT_SYMBOL_GPL(dst_blackhole_redirect);
#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_mtu(dst: *const dst_entry) -> c_uint {
    unsigned int dst_blackhole_mtu(const struct dst_entry *dst)
    {
    let mut mtu: c_uint = dst_metric_raw(dst, RTAX_MTU);
    return mtu ? : dst_dev(dst).mtu;
    }
    EXPORT_SYMBOL_GPL(dst_blackhole_mtu);
    static struct dst_ops dst_blackhole_ops = {
    .family		= AF_UNSPEC,
    .neigh_lookup	= dst_blackhole_neigh_lookup,
    .check		= dst_blackhole_check,
    .cow_metrics	= dst_blackhole_cow_metrics,
    .update_pmtu	= dst_blackhole_update_pmtu,
    .redirect	= dst_blackhole_redirect,
    .mtu		= dst_blackhole_mtu,
    };
    static void __metadata_dst_init(struct metadata_dst *md_dst,
    enum metadata_type type, u8 optslen)
    {
    struct dst_entry *dst;
    dst = &md_dst.dst;
    dst_init(dst, &dst_blackhole_ops, core::ptr::null_mut(), DST_OBSOLETE_NONE,
    DST_METADATA | DST_NOCOUNT);
    memset(dst + 1, 0, sizeof(*md_dst) + optslen - sizeof(*dst));
    md_dst.type = type;
    }
    struct metadata_dst *metadata_dst_alloc(u8 optslen, enum metadata_type type,
    gfp_t flags)
    {
    struct metadata_dst *md_dst;
    md_dst = kmalloc_flex(*md_dst, u.tun_info.options, optslen, flags);
    if (!md_dst)
    return core::ptr::null_mut();
    __metadata_dst_init(md_dst, type, optslen);
    return md_dst;
    }
    EXPORT_SYMBOL_GPL(metadata_dst_alloc);
#[no_mangle]
pub unsafe extern "C" fn metadata_dst_free(md_dst: *mut metadata_dst) {
    void metadata_dst_free(struct metadata_dst *md_dst)
    {

    if (md_dst.type == METADATA_IP_TUNNEL)
    dst_cache_destroy(&md_dst.u.tun_info.dst_cache);

    if (md_dst.type == METADATA_XFRM)
    dst_release(md_dst.u.xfrm_info.dst_orig);
    kfree(md_dst);
    }
    EXPORT_SYMBOL_GPL(metadata_dst_free);
    struct metadata_dst __percpu *
    metadata_dst_alloc_percpu(u8 optslen, enum metadata_type type, gfp_t flags)
    {
    int cpu;
    struct metadata_dst __percpu *md_dst;
    md_dst = __alloc_percpu_gfp(struct_size(md_dst, u.tun_info.options,
    optslen),
    __alignof__(struct metadata_dst), flags);
    if (!md_dst)
    return core::ptr::null_mut();
    for_each_possible_cpu(cpu)
    __metadata_dst_init(per_cpu_ptr(md_dst, cpu), type, optslen);
    return md_dst;
    }
    EXPORT_SYMBOL_GPL(metadata_dst_alloc_percpu);
#[no_mangle]
pub unsafe extern "C" fn metadata_dst_free_percpu(md_dst: *mut metadata_dst __percpu) {
    void metadata_dst_free_percpu(struct metadata_dst __percpu *md_dst)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    struct metadata_dst *one_md_dst = per_cpu_ptr(md_dst, cpu);

    if (one_md_dst.type == METADATA_IP_TUNNEL)
    dst_cache_destroy(&one_md_dst.u.tun_info.dst_cache);

    if (one_md_dst.type == METADATA_XFRM)
    dst_release(one_md_dst.u.xfrm_info.dst_orig);
    }
    free_percpu(md_dst);
    }
    EXPORT_SYMBOL_GPL(metadata_dst_free_percpu);
