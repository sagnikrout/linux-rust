//! Automatically rewritten from C to Rust
//! Source: net/ipv4/inetpeer.c
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
//
// INETPEER - A storage for permanent information about peers
//
// Authors:	Andrey V. Savochkin <saw@msu.ru>
//

//
// Theory of operations.
// We keep one entry for each peer IP address.  The nodes contains long-living
// information about the peer which doesn't depend on routes.
//
// Nodes are removed only when reference counter goes to 0.
// When it's happened the node may be removed when a sufficient amount of
// time has been passed since its last use.  The less-recently-used entry can
// also be removed if the pool is overloaded i.e. if the total amount of
// entries is greater-or-equal than the threshold.
//
// Node pool is organised as an RB tree.
// Such an implementation has been chosen not just for fun.  It's a way to
// prevent easy and efficient DoS attacks by creating hash collisions.  A huge
// amount of long living nodes in a single hash slot would significantly delay
// lookups performed with disabled BHs.
//
// Serialisation issues.
// 1.  Nodes may appear in the tree only with the pool lock held.
// 2.  Nodes may disappear from the tree only with the pool lock held
// AND reference count being 0.
// 3.  Global variable peer_total is modified under the pool lock.
// 4.  struct inet_peer fields modification:
// rb_node: pool lock
// refcnt: atomically against modifications on other CPU;
// usually under some other lock to prevent node disappearing
// daddr: unchangeable
//
    static struct kmem_cache *peer_cachep __ro_after_init;
    static siphash_aligned_key_t inetpeer_hash_key __read_mostly;
#[no_mangle]
unsafe extern "C" fn inetpeer_addr_hash(a: *const inetpeer_addr) -> u64 {
    static u64 inetpeer_addr_hash(const struct inetpeer_addr *a)
    {
    net_get_random_once(&inetpeer_hash_key, sizeof(inetpeer_hash_key));
    if (a.family == AF_INET)
    return siphash_2u32(( u32)a.a4.addr, a.a4.vif,
    &inetpeer_hash_key);
    return siphash_4u32(( u32)a.a6.s6_addr32[0],
    ( u32)a.a6.s6_addr32[1],
    ( u32)a.a6.s6_addr32[2],
    ( u32)a.a6.s6_addr32[3],
    &inetpeer_hash_key);
    }
    static int inetpeer_entry_cmp(u64 dhash,
    const struct inetpeer_addr *daddr,
    const struct inet_peer *p)
    {
    if (dhash < p.hash)
    return -1;
    if (dhash > p.hash)
    return 1;
    return inetpeer_addr_cmp(daddr, &p.daddr);
    }
#[no_mangle]
pub unsafe extern "C" fn inet_peer_base_init(bp: *mut inet_peer_base) {
    void inet_peer_base_init(struct inet_peer_base *bp)
    {
    bp.rb_root = RB_ROOT;
    seqlock_init(&bp.lock);
    bp.total = 0;
    }
pub const PEER_MAX_GC: c_int = 32;
// Exported for sysctl_net_ipv4.
    int inet_peer_threshold __read_mostly;	/* start to throw entries more
// aggressively at this stage
    int inet_peer_minttl __read_mostly = 120 * HZ;	/* TTL under high load: 120 sec */
    int inet_peer_maxttl __read_mostly = 10 * 60 * HZ;	/* usual time to live: 10 min */
// Called from ip_output.c:ip_init
#[no_mangle]
pub unsafe extern "C" fn inet_initpeers() -> void __init {
    void __init inet_initpeers(void)
    {
    u64 nr_entries;
// 1% of physical memory
    nr_entries = div64_ul((u64)totalram_pages() << PAGE_SHIFT,
    100 * L1_CACHE_ALIGN(sizeof(struct inet_peer)));
    inet_peer_threshold = clamp_val(nr_entries, 4096, 65536 + 128);
    peer_cachep = KMEM_CACHE(inet_peer, SLAB_HWCACHE_ALIGN | SLAB_PANIC);
    }
// Called with rcu_read_lock() or base->lock held
    static struct inet_peer *lookup(const struct inetpeer_addr *daddr,
    u64 dhash,
    struct inet_peer_base *base,
    unsigned int seq,
    struct inet_peer *gc_stack[],
    unsigned int *gc_cnt,
    struct rb_node **parent_p,
    struct rb_node ***pp_p)
    {
    struct rb_node **pp, *parent, *next;
    struct inet_peer *p;
    u32 now;
    pp = &base.rb_root.rb_node;
    parent = core::ptr::null_mut();
    while (1) {
    int cmp;
    next = rcu_dereference_raw(*pp);
    if (!next)
    break;
    parent = next;
    p = rb_entry(parent, struct inet_peer, rb_node);
    cmp = inetpeer_entry_cmp(dhash, daddr, p);
    if (cmp == 0) {
    now = jiffies;
    if (READ_ONCE(p.dtime) != now)
    WRITE_ONCE(p.dtime, now);
    return p;
    }
    if (gc_stack) {
    if (*gc_cnt < PEER_MAX_GC)
    gc_stack[(*gc_cnt)++] = p;
    } else if (unlikely(read_seqretry(&base.lock, seq))) {
    break;
    }
    if (cmp == -1)
    pp = &next.rb_left;
    else
    pp = &next.rb_right;
    }
// parent_p = parent;
// pp_p = pp;
    return core::ptr::null_mut();
    }
// perform garbage collect on all items stacked during a lookup
    static void inet_peer_gc(struct inet_peer_base *base,
    struct inet_peer *gc_stack[],
    unsigned int gc_cnt)
    {
    int peer_threshold, peer_maxttl, peer_minttl;
    struct inet_peer *p;
    __u32 delta, ttl;
    int i;
    peer_threshold = READ_ONCE(inet_peer_threshold);
    peer_maxttl = READ_ONCE(inet_peer_maxttl);
    peer_minttl = READ_ONCE(inet_peer_minttl);
    if (base.total >= peer_threshold)
    ttl = 0; /* be aggressive */
    else
    ttl = peer_maxttl - (peer_maxttl - peer_minttl) / HZ *
    base.total / peer_threshold * HZ;
    for (i = 0; i < gc_cnt; i++) {
    p = gc_stack[i];
    delta = (__u32)jiffies - READ_ONCE(p.dtime);
    if (delta < ttl || !refcount_dec_if_one(&p.refcnt))
    gc_stack[i] = core::ptr::null_mut();
    }
    for (i = 0; i < gc_cnt; i++) {
    p = gc_stack[i];
    if (p) {
    rb_erase(&p.rb_node, &base.rb_root);
    base.total--;
    kfree_rcu(p, rcu);
    }
    }
    }
// Must be called under RCU : No refcount change is done here.
    struct inet_peer *inet_getpeer(struct inet_peer_base *base,
    const struct inetpeer_addr *daddr)
    {
    struct inet_peer *p, *gc_stack[PEER_MAX_GC];
    let mut dhash: u64 = inetpeer_addr_hash(daddr);
    struct rb_node **pp, *parent;
    unsigned int gc_cnt, seq;
// Attempt a lockless lookup first.
// Because of a concurrent writer, we might not find an existing entry.
//
    seq = read_seqbegin(&base.lock);
    p = lookup(daddr, dhash, base, seq, core::ptr::null_mut(), &gc_cnt, &parent, &pp);
// Make sure tree was not modified during our lookup.
    if (p && !read_seqretry(&base.lock, seq))
    return p;
// retry an exact lookup, taking the lock before.
// At least, nodes should be hot in our cache.
//
    parent = core::ptr::null_mut();
    write_seqlock_bh(&base.lock);
    gc_cnt = 0;
    p = lookup(daddr, dhash, base, seq, gc_stack, &gc_cnt, &parent, &pp);
    if (!p) {
    p = kmem_cache_alloc(peer_cachep, GFP_ATOMIC);
    if (p) {
    p.daddr = *daddr;
    p.hash = dhash;
    p.dtime = (__u32)jiffies;
    refcount_set(&p.refcnt, 1);
    atomic_set(&p.rid, 0);
    p.metrics[RTAX_LOCK-1] = INETPEER_METRICS_NEW;
    p.rate_tokens = 0;
    p.n_redirects = 0;
// 60*HZ is arbitrary, but chosen enough high so that the first
// calculation of tokens is at its maximum.
//
    p.rate_last = jiffies - 60*HZ;
    rb_link_node(&p.rb_node, parent, pp);
    rb_insert_color(&p.rb_node, &base.rb_root);
    base.total++;
    }
    }
    if (gc_cnt)
    inet_peer_gc(base, gc_stack, gc_cnt);
    write_sequnlock_bh(&base.lock);
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn inet_putpeer(p: *mut inet_peer) {
    void inet_putpeer(struct inet_peer *p)
    {
    if (refcount_dec_and_test(&p.refcnt))
    kfree_rcu(p, rcu);
    }
//
// Check transmit rate limitation for given message.
// The rate information is held in the inet_peer entries now.
// This function is generic and could be used for other purposes
// too. It uses a Token bucket filter as suggested by Alexey Kuznetsov.
//
// Note that the same inet_peer fields are modified by functions in
// route.c too, but these work for packet destinations while xrlim_allow
// works for icmp destinations. This means the rate limiting information
// for one "ip object" is shared - and these ICMPs are twice limited:
// by source and by destination.
//
// RFC 1812: 4.3.2.8 SHOULD be able to limit error message rate
// SHOULD allow setting of rate limits
//
// Shared between ICMPv4 and ICMPv6.
//
pub const XRLIM_BURST_FACTOR: c_int = 6;
#[no_mangle]
pub unsafe extern "C" fn inet_peer_xrlim_allow(peer: *mut inet_peer, timeout: c_int) -> bool {
    bool inet_peer_xrlim_allow(struct inet_peer *peer, int timeout)
    {
    unsigned long now, token, otoken, delta;
    let mut rc: bool = false;
    if (!peer)
    return true;
    token = otoken = READ_ONCE(peer.rate_tokens);
    now = jiffies;
    delta = now - READ_ONCE(peer.rate_last);
    if (delta) {
    WRITE_ONCE(peer.rate_last, now);
    token += delta;
    if (token > XRLIM_BURST_FACTOR * timeout)
    token = XRLIM_BURST_FACTOR * timeout;
    }
    if (token >= timeout) {
    token -= timeout;
    rc = true;
    }
    if (token != otoken)
    WRITE_ONCE(peer.rate_tokens, token);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn inetpeer_invalidate_tree(base: *mut inet_peer_base) {
    void inetpeer_invalidate_tree(struct inet_peer_base *base)
    {
    struct rb_node *p = rb_first(&base.rb_root);
    while (p) {
    struct inet_peer *peer = rb_entry(p, struct inet_peer, rb_node);
    p = rb_next(p);
    rb_erase(&peer.rb_node, &base.rb_root);
    inet_putpeer(peer);
    cond_resched();
    }
    base.total = 0;
    }
