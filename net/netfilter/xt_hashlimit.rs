//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_hashlimit.c
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
// xt_hashlimit - Netfilter module to limit the number of packets per time
// separately for each hashbucket (sourceip/sourceport/dstip/dstport)
//
// (C) 2003-2004 by Harald Welte <laforge@netfilter.org>
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
// Copyright © CC Computer Consultants GmbH, 2007 - 2008
//
// Development of this code was funded by Astaro AG, http://www.astaro.com
//

    XT_HASHLIMIT_HASH_SIP | XT_HASHLIMIT_HASH_SPT | \
    XT_HASHLIMIT_INVERT | XT_HASHLIMIT_BYTES |\
    XT_HASHLIMIT_RATE_MATCH)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_DESCRIPTION("Xtables: per hash-bucket rate-limit match");
    MODULE_ALIAS("ipt_hashlimit");
    MODULE_ALIAS("ip6t_hashlimit");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashlimit_net {
    pub htables: hlist_head,
    pub ipt_hashlimit: *mut proc_dir_entry,
    pub ip6t_hashlimit: *mut proc_dir_entry,
}

    static unsigned int hashlimit_net_id;
    static inline struct hashlimit_net *hashlimit_pernet(struct net *net)
    {
    return net_generic(net, hashlimit_net_id);
    }
// need to declare this at the top
    static const struct seq_operations dl_seq_ops_v2;
    static const struct seq_operations dl_seq_ops_v1;
    static const struct seq_operations dl_seq_ops;
// hash table crap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsthash_dst {
    union {
    struct {
    pub src: __be32,
    pub dst: __be32,
    pub ip: },

    struct {
    pub src: [__be32; 4],
    pub dst: [__be32; 4],
    pub ip6: },

}

    __be16 src_port;
    __be16 dst_port;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsthash_ent {
// static / read-only parts in the beginning
    pub node: hlist_node,
    pub dst: dsthash_dst,
// modified structure members in the end
    pub lock: spinlock_t,
    pub /: *mut *mut unsigned long expires; / precalculated expiry time,
    struct {
    pub /: *mut *mut unsigned long prev; / last modification,
    union {
    struct {
    pub credit: u_int64_t,
    pub credit_cap: u_int64_t,
    pub cost: u_int64_t,
}

    struct {
    u_int32_t interval, prev_window;
    u_int64_t current_rate;
    u_int64_t rate;
    int64_t burst;
    };
    };
    } rateinfo;
    struct rcu_head rcu;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_hashlimit_htable {
    pub /: *mut *mut hlist_node node; / global list of all htables,
    pub use: refcount_t,
    pub family: u_int8_t,
    pub rnd_initialized: bool,
    pub ratematch: bool,
    pub /: *mut *mut hashlimit_cfg3 cfg; / config,
// used internally
    pub /: *mut *mut spinlock_t lock; / lock for list_head,
    pub /: *mut *mut u_int32_t rnd; / random seed for hash,
    pub /: *mut *mut unsigned int count; / number entries in table,
    pub gc_work: delayed_work,
// seq_file stuff
    pub pde: *mut proc_dir_entry,
    pub name: *const c_char,
    pub net: *mut net,
    pub /: *mut *mut hlist_head hash[]; / hashtable itself,
}

    static int
    cfg_copy(struct hashlimit_cfg3 *to, const void *from, int revision)
    {
    if (revision == 1) {
    struct hashlimit_cfg1 *cfg = (struct hashlimit_cfg1 *)from;
    to.mode = cfg.mode;
    to.avg = cfg.avg;
    to.burst = cfg.burst;
    to.size = cfg.size;
    to.max = cfg.max;
    to.gc_interval = cfg.gc_interval;
    to.expire = cfg.expire;
    to.srcmask = cfg.srcmask;
    to.dstmask = cfg.dstmask;
    } else if (revision == 2) {
    struct hashlimit_cfg2 *cfg = (struct hashlimit_cfg2 *)from;
    to.mode = cfg.mode;
    to.avg = cfg.avg;
    to.burst = cfg.burst;
    to.size = cfg.size;
    to.max = cfg.max;
    to.gc_interval = cfg.gc_interval;
    to.expire = cfg.expire;
    to.srcmask = cfg.srcmask;
    to.dstmask = cfg.dstmask;
    } else if (revision == 3) {
    memcpy(to, from, sizeof(struct hashlimit_cfg3));
    } else {
    return -EINVAL;
    }
    return 0;
    }
    static DEFINE_MUTEX(hashlimit_mutex);	/* protects htables list */
    static struct kmem_cache *hashlimit_cachep __read_mostly;
    static inline bool dst_cmp(const struct dsthash_ent *ent,
    const struct dsthash_dst *b)
    {
    return !memcmp(&ent.dst, b, sizeof(ent.dst));
    }
    static u_int32_t
    hash_dst(const struct xt_hashlimit_htable *ht, const struct dsthash_dst *dst)
    {
    u_int32_t hash = jhash2((const u32 *)dst,
    sizeof(*dst)/sizeof(u32),
    ht.rnd);
//
// Instead of returning hash % ht->cfg.size (implying a divide)
// we return the high 32 bits of the (hash * ht->cfg.size) that will
// give results between [0 and cfg.size-1] and same hash distribution,
// but using a multiply, less expensive than a divide
//
    return reciprocal_scale(hash, ht.cfg.size);
    }
    static struct dsthash_ent *
    dsthash_find(const struct xt_hashlimit_htable *ht,
    const struct dsthash_dst *dst)
    {
    struct dsthash_ent *ent;
    let mut hash: u_int32_t = hash_dst(ht, dst);
    if (!hlist_empty(&ht.hash[hash])) {
    hlist_for_each_entry_rcu(ent, &ht.hash[hash], node)
    if (dst_cmp(ent, dst)) {
    spin_lock(&ent.lock);
    return ent;
    }
    }
    return core::ptr::null_mut();
    }
// allocate dsthash_ent, initialize dst, put in htable and lock it
    static struct dsthash_ent *
    dsthash_alloc_init(struct xt_hashlimit_htable *ht,
    const struct dsthash_dst *dst, bool *race)
    {
    struct dsthash_ent *ent;
    spin_lock(&ht.lock);
// Two or more packets may race to create the same entry in the
// hashtable, double check if this packet lost race.
//
    ent = dsthash_find(ht, dst);
    if (ent != core::ptr::null_mut()) {
    spin_unlock(&ht.lock);
// race = true;
    return ent;
    }
// initialize hash with random val at the time we allocate
// the first hashtable entry
    if (unlikely(!ht.rnd_initialized)) {
    get_random_bytes(&ht.rnd, sizeof(ht.rnd));
    ht.rnd_initialized = true;
    }
    if (ht.cfg.max && ht.count >= ht.cfg.max) {
// FIXME: do something. question is what..
    net_err_ratelimited("max count of %u reached\n", ht.cfg.max);
    ent = core::ptr::null_mut();
    } else
    ent = kmem_cache_alloc(hashlimit_cachep, GFP_ATOMIC);
    if (ent) {
    memcpy(&ent.dst, dst, sizeof(ent.dst));
    spin_lock_init(&ent.lock);
    spin_lock(&ent.lock);
    hlist_add_head_rcu(&ent.node, &ht.hash[hash_dst(ht, dst)]);
    ht.count++;
    }
    spin_unlock(&ht.lock);
    return ent;
    }
#[no_mangle]
unsafe extern "C" fn dsthash_free_rcu(head: *mut rcu_head) {
    static void dsthash_free_rcu(struct rcu_head *head)
    {
    struct dsthash_ent *ent = container_of(head, struct dsthash_ent, rcu);
    kmem_cache_free(hashlimit_cachep, ent);
    }
    static inline void
    dsthash_free(struct xt_hashlimit_htable *ht, struct dsthash_ent *ent)
    {
    hlist_del_rcu(&ent.node);
    call_rcu(&ent.rcu, dsthash_free_rcu);
    ht.count--;
    }
    static void htable_gc(struct work_struct *work);
    static int htable_create(struct net *net, struct hashlimit_cfg3 *cfg,
    const char *name, u_int8_t family,
    struct xt_hashlimit_htable **out_hinfo,
    int revision)
    {
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(net);
    struct xt_hashlimit_htable *hinfo;
    const struct seq_operations *ops;
    unsigned int size, i;
    let mut nr_pages: c_ulong = totalram_pages();
    int ret;
    if (cfg.size) {
    size = cfg.size;
    } else {
    size = (nr_pages << PAGE_SHIFT) / 16384 /
    sizeof(struct hlist_head);
    if (nr_pages > 1024 * 1024 * 1024 / PAGE_SIZE)
    size = 8192;
    if (size < 16)
    size = 16;
    }
    hinfo = kvmalloc_flex(*hinfo, hash, size);
    if (hinfo == core::ptr::null_mut())
    return -ENOMEM;
// out_hinfo = hinfo;
// copy match config into hashtable config
    ret = cfg_copy(&hinfo.cfg, (void *)cfg, 3);
    if (ret) {
    kvfree(hinfo);
    return ret;
    }
    hinfo.cfg.size = size;
    if (hinfo.cfg.max == 0)
    hinfo.cfg.max = 8 * hinfo.cfg.size;
#[no_mangle]
pub unsafe extern "C" fn if(hinfo->cfg.size: hinfo->cfg.max <) -> else {
    else if (hinfo.cfg.max < hinfo.cfg.size)
    hinfo.cfg.max = hinfo.cfg.size;
    for (i = 0; i < hinfo.cfg.size; i++)
    INIT_HLIST_HEAD(&hinfo.hash[i]);
    refcount_set(&hinfo.use, 1);
    hinfo.count = 0;
    hinfo.family = family;
    hinfo.rnd_initialized = false;
    hinfo.name = kstrdup(name, GFP_KERNEL);
    if (!hinfo.name) {
    kvfree(hinfo);
    return -ENOMEM;
    }
    hinfo.ratematch = !!(cfg.mode & XT_HASHLIMIT_RATE_MATCH);
    spin_lock_init(&hinfo.lock);
    switch (revision) {
    case 1:
    ops = &dl_seq_ops_v1;
    break;
    case 2:
    ops = &dl_seq_ops_v2;
    break;
    default:
    ops = &dl_seq_ops;
    }
    hinfo.pde = proc_create_seq_data(name, 0,
    (family == NFPROTO_IPV4) ?
    hashlimit_net.ipt_hashlimit : hashlimit_net.ip6t_hashlimit,
    ops, hinfo);
    if (hinfo.pde == core::ptr::null_mut()) {
    kfree(hinfo.name);
    kvfree(hinfo);
    return -ENOMEM;
    }
    hinfo.net = net;
    INIT_DEFERRABLE_WORK(&hinfo.gc_work, htable_gc);
    queue_delayed_work(system_power_efficient_wq, &hinfo.gc_work,
    msecs_to_jiffies(hinfo.cfg.gc_interval));
    hlist_add_head(&hinfo.node, &hashlimit_net.htables);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn htable_selective_cleanup(ht: *mut xt_hashlimit_htable, select_all: bool) {
    static void htable_selective_cleanup(struct xt_hashlimit_htable *ht, bool select_all)
    {
    unsigned int i;
    for (i = 0; i < ht.cfg.size; i++) {
    struct hlist_head *head = &ht.hash[i];
    struct dsthash_ent *dh;
    struct hlist_node *n;
    if (hlist_empty(head))
    continue;
    spin_lock_bh(&ht.lock);
    hlist_for_each_entry_safe(dh, n, head, node) {
    if (time_after_eq(jiffies, dh.expires) || select_all)
    dsthash_free(ht, dh);
    }
    spin_unlock_bh(&ht.lock);
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn htable_gc(work: *mut work_struct) {
    static void htable_gc(struct work_struct *work)
    {
    struct xt_hashlimit_htable *ht;
    ht = container_of(work, struct xt_hashlimit_htable, gc_work.work);
    htable_selective_cleanup(ht, false);
    queue_delayed_work(system_power_efficient_wq,
    &ht.gc_work, msecs_to_jiffies(ht.cfg.gc_interval));
    }
#[no_mangle]
unsafe extern "C" fn htable_remove_proc_entry(hinfo: *mut xt_hashlimit_htable) {
    static void htable_remove_proc_entry(struct xt_hashlimit_htable *hinfo)
    {
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(hinfo.net);
    struct proc_dir_entry *parent;
    if (hinfo.family == NFPROTO_IPV4)
    parent = hashlimit_net.ipt_hashlimit;
    else
    parent = hashlimit_net.ip6t_hashlimit;
    if (parent != core::ptr::null_mut())
    remove_proc_entry(hinfo.name, parent);
    }
    static struct xt_hashlimit_htable *htable_find_get(struct net *net,
    const char *name,
    u_int8_t family)
    {
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(net);
    struct xt_hashlimit_htable *hinfo;
    hlist_for_each_entry(hinfo, &hashlimit_net.htables, node) {
    if (!strcmp(name, hinfo.name) &&
    hinfo.family == family) {
    refcount_inc(&hinfo.use);
    return hinfo;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn htable_put(hinfo: *mut xt_hashlimit_htable) {
    static void htable_put(struct xt_hashlimit_htable *hinfo)
    {
    if (refcount_dec_and_mutex_lock(&hinfo.use, &hashlimit_mutex)) {
    hlist_del(&hinfo.node);
    htable_remove_proc_entry(hinfo);
    mutex_unlock(&hashlimit_mutex);
    cancel_delayed_work_sync(&hinfo.gc_work);
    htable_selective_cleanup(hinfo, true);
    kfree(hinfo.name);
    kvfree(hinfo);
    }
    }
// The algorithm used is the Simple Token Bucket Filter (TBF)
// see net/sched/sch_tbf.c in the linux source tree
//
// Rusty: This is my (non-mathematically-inclined) understanding of
    this algorithm.  The `average rate' in jiffies becomes your initial
    amount of credit `credit' and the most credit you can ever have
    `credit_cap'.  The `peak rate' becomes the cost of passing the
    test, `cost'.
    `prev' tracks the last packet hit: you gain one credit per jiffy.
    If you get credit balance more than this, the extra credit is
    discarded.  Every time the match passes, you lose `cost' credits;
    if you don't have that many, the test fails.
    See Alexey's formal explanation in net/sched/sch_tbf.c.
    To get the maximum range, we multiply by this factor (ie. you get N
    credits per jiffy).  We want to allow a rate as low as 1 per day
    (slowest userspace tool allows), which means
    CREDITS_PER_JIFFY*HZ*60*60*24 < 2^32 ie.
//

// Repeated shift and or gives us all 1s, final shift and add 1 gives
// us the power of 2 below the theoretical max, so GCC simply does a
// shift.

// in byte mode, the lowest possible rate is one packet/second.
// credit_cap is used as a counter that tells us how many times we can
// refill the "credits available" counter when it becomes empty.
//

#[no_mangle]
unsafe extern "C" fn xt_hashlimit_len_to_chunks(len: u32) -> u32 {
    static u32 xt_hashlimit_len_to_chunks(u32 len)
    {
    return (len >> XT_HASHLIMIT_BYTE_SHIFT) + 1;
    }
// Precision saver.
#[no_mangle]
unsafe extern "C" fn user2credits(user: u64, revision: c_int) -> u64 {
    static u64 user2credits(u64 user, int revision)
    {
    u64 scale = (revision == 1) ?
    XT_HASHLIMIT_SCALE : XT_HASHLIMIT_SCALE_v2;
    u64 cpj = (revision == 1) ?
    CREDITS_PER_JIFFY_v1 : CREDITS_PER_JIFFY;
// Avoid overflow: divide the constant operands first
    if (scale >= HZ * cpj)
    return div64_u64(user, div64_u64(scale, HZ * cpj));
    return user * div64_u64(HZ * cpj, scale);
    }
#[no_mangle]
unsafe extern "C" fn user2credits_byte(user: u32) -> u32 {
    static u32 user2credits_byte(u32 user)
    {
    let mut us: u64 = user;
    us *= HZ * CREDITS_PER_JIFFY_BYTES;
    return (u32) (us >> 32);
    }
#[no_mangle]
unsafe extern "C" fn user2rate(user: u64) -> u64 {
    static u64 user2rate(u64 user)
    {
    if (user != 0) {
    return div64_u64(XT_HASHLIMIT_SCALE_v2, user);
    } else {
    pr_info_ratelimited("invalid rate from userspace: %llu\n",
    user);
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn user2rate_bytes(user: u32) -> u64 {
    static u64 user2rate_bytes(u32 user)
    {
    u64 r;
    r = user ? U32_MAX / user : U32_MAX;
    return (r - 1) << XT_HASHLIMIT_BYTE_SHIFT;
    }
    static void rateinfo_recalc(struct dsthash_ent *dh, unsigned long now,
    u32 mode, int revision)
    {
    let mut delta: c_ulong = now - dh.rateinfo.prev;
    u64 cap, cpj;
    if (delta == 0)
    return;
    if (revision >= 3 && mode & XT_HASHLIMIT_RATE_MATCH) {
    let mut interval: u64 = dh.rateinfo.interval * HZ;
    if (delta < interval)
    return;
    dh.rateinfo.prev = now;
    dh.rateinfo.prev_window =
    ((dh.rateinfo.current_rate * interval) >
    (delta * dh.rateinfo.rate));
    dh.rateinfo.current_rate = 0;
    return;
    }
    dh.rateinfo.prev = now;
    if (mode & XT_HASHLIMIT_BYTES) {
    let mut tmp: u64 = dh.rateinfo.credit;
    dh.rateinfo.credit += CREDITS_PER_JIFFY_BYTES * delta;
    cap = CREDITS_PER_JIFFY_BYTES * HZ;
    if (tmp >= dh.rateinfo.credit) {/* overflow */
    dh.rateinfo.credit = cap;
    return;
    }
    } else {
    cpj = (revision == 1) ?
    CREDITS_PER_JIFFY_v1 : CREDITS_PER_JIFFY;
    dh.rateinfo.credit += delta * cpj;
    cap = dh.rateinfo.credit_cap;
    }
    if (dh.rateinfo.credit > cap)
    dh.rateinfo.credit = cap;
    }
    static void rateinfo_init(struct dsthash_ent *dh,
    struct xt_hashlimit_htable *hinfo, int revision)
    {
    dh.rateinfo.prev = jiffies;
    if (revision >= 3 && hinfo.cfg.mode & XT_HASHLIMIT_RATE_MATCH) {
    dh.rateinfo.prev_window = 0;
    dh.rateinfo.current_rate = 0;
    if (hinfo.cfg.mode & XT_HASHLIMIT_BYTES) {
    dh.rateinfo.rate =
    user2rate_bytes((u32)hinfo.cfg.avg);
    if (hinfo.cfg.burst)
    dh.rateinfo.burst =
    hinfo.cfg.burst * dh.rateinfo.rate;
    else
    dh.rateinfo.burst = dh.rateinfo.rate;
    } else {
    dh.rateinfo.rate = user2rate(hinfo.cfg.avg);
    dh.rateinfo.burst =
    hinfo.cfg.burst + dh.rateinfo.rate;
    }
    dh.rateinfo.interval = hinfo.cfg.interval;
    } else if (hinfo.cfg.mode & XT_HASHLIMIT_BYTES) {
    dh.rateinfo.credit = CREDITS_PER_JIFFY_BYTES * HZ;
    dh.rateinfo.cost = user2credits_byte(hinfo.cfg.avg);
    dh.rateinfo.credit_cap = hinfo.cfg.burst;
    } else {
    dh.rateinfo.credit = user2credits(hinfo.cfg.avg *
    hinfo.cfg.burst, revision);
    dh.rateinfo.cost = user2credits(hinfo.cfg.avg, revision);
    dh.rateinfo.credit_cap = dh.rateinfo.credit;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn maskl(a: __be32, l: c_uint) -> __be32 {
    static inline __be32 maskl(__be32 a, unsigned int l)
    {
    return l ? htonl(ntohl(a) & ~0 << (32 - l)) : 0;
    }

#[no_mangle]
unsafe extern "C" fn hashlimit_ipv6_mask(i: *mut __be32, p: c_uint) {
    static void hashlimit_ipv6_mask(__be32 *i, unsigned int p)
    {
    switch (p) {
    case 0 ... 31:
    i[0] = maskl(i[0], p);
    i[1] = i[2] = i[3] = 0;
    break;
    case 32 ... 63:
    i[1] = maskl(i[1], p - 32);
    i[2] = i[3] = 0;
    break;
    case 64 ... 95:
    i[2] = maskl(i[2], p - 64);
    i[3] = 0;
    break;
    case 96 ... 127:
    i[3] = maskl(i[3], p - 96);
    break;
    case 128:
    break;
    }
    }

    static int
    hashlimit_init_dst(const struct xt_hashlimit_htable *hinfo,
    struct dsthash_dst *dst,
    const struct sk_buff *skb, unsigned int protoff)
    {
    __be16 _ports[2], *ports;
    u8 nexthdr;
    int poff;
    memset(dst, 0, sizeof(*dst));
    switch (hinfo.family) {
    case NFPROTO_IPV4:
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_DIP)
    dst.ip.dst = maskl(ip_hdr(skb).daddr,
    hinfo.cfg.dstmask);
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_SIP)
    dst.ip.src = maskl(ip_hdr(skb).saddr,
    hinfo.cfg.srcmask);
    if (!(hinfo.cfg.mode &
    (XT_HASHLIMIT_HASH_DPT | XT_HASHLIMIT_HASH_SPT)))
    return 0;
    if (ntohs(ip_hdr(skb).frag_off) & IP_OFFSET)
    return -1;
    nexthdr = ip_hdr(skb).protocol;
    break;

    case NFPROTO_IPV6:
    {
    __be16 frag_off;
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_DIP) {
    memcpy(&dst.ip6.dst, &ipv6_hdr(skb).daddr,
    sizeof(dst.ip6.dst));
    hashlimit_ipv6_mask(dst.ip6.dst, hinfo.cfg.dstmask);
    }
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_SIP) {
    memcpy(&dst.ip6.src, &ipv6_hdr(skb).saddr,
    sizeof(dst.ip6.src));
    hashlimit_ipv6_mask(dst.ip6.src, hinfo.cfg.srcmask);
    }
    if (!(hinfo.cfg.mode &
    (XT_HASHLIMIT_HASH_DPT | XT_HASHLIMIT_HASH_SPT)))
    return 0;
    nexthdr = ipv6_hdr(skb).nexthdr;
    protoff = ipv6_skip_exthdr(skb, sizeof(struct ipv6hdr), &nexthdr, &frag_off);
    if ((int)protoff < 0 || ntohs(frag_off) & IP6_OFFSET)
    return -1;
    break;
    }

    default:
    BUG();
    return 0;
    }
    poff = proto_ports_offset(nexthdr);
    if (poff >= 0) {
    ports = skb_header_pointer(skb, protoff + poff, sizeof(_ports),
    &_ports);
    } else {
    _ports[0] = _ports[1] = 0;
    ports = _ports;
    }
    if (!ports)
    return -1;
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_SPT)
    dst.src_port = ports[0];
    if (hinfo.cfg.mode & XT_HASHLIMIT_HASH_DPT)
    dst.dst_port = ports[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_byte_cost(len: c_uint, dh: *mut dsthash_ent) -> u32 {
    static u32 hashlimit_byte_cost(unsigned int len, struct dsthash_ent *dh)
    {
    let mut tmp: u64 = xt_hashlimit_len_to_chunks(len);
    tmp = tmp * dh.rateinfo.cost;
    if (unlikely(tmp > CREDITS_PER_JIFFY_BYTES * HZ))
    tmp = CREDITS_PER_JIFFY_BYTES * HZ;
    if (dh.rateinfo.credit < tmp && dh.rateinfo.credit_cap) {
    dh.rateinfo.credit_cap--;
    dh.rateinfo.credit = CREDITS_PER_JIFFY_BYTES * HZ;
    }
    return (u32) tmp;
    }
    static bool
    hashlimit_mt_common(const struct sk_buff *skb, struct xt_action_param *par,
    struct xt_hashlimit_htable *hinfo,
    const struct hashlimit_cfg3 *cfg, int revision)
    {
    let mut now: c_ulong = jiffies;
    struct dsthash_ent *dh;
    struct dsthash_dst dst;
    let mut race: bool = false;
    u64 cost;
    if (hashlimit_init_dst(hinfo, &dst, skb, par.thoff) < 0)
    goto hotdrop;
    local_bh_disable();
    dh = dsthash_find(hinfo, &dst);
    if (dh == core::ptr::null_mut()) {
    dh = dsthash_alloc_init(hinfo, &dst, &race);
    if (dh == core::ptr::null_mut()) {
    local_bh_enable();
    goto hotdrop;
    } else if (race) {
// Already got an entry, update expiration timeout
    dh.expires = now + msecs_to_jiffies(hinfo.cfg.expire);
    rateinfo_recalc(dh, now, hinfo.cfg.mode, revision);
    } else {
    dh.expires = jiffies + msecs_to_jiffies(hinfo.cfg.expire);
    rateinfo_init(dh, hinfo, revision);
    }
    } else {
// update expiration timeout
    dh.expires = now + msecs_to_jiffies(hinfo.cfg.expire);
    rateinfo_recalc(dh, now, hinfo.cfg.mode, revision);
    }
    if (cfg.mode & XT_HASHLIMIT_RATE_MATCH) {
    cost = (cfg.mode & XT_HASHLIMIT_BYTES) ? skb.len : 1;
    dh.rateinfo.current_rate += cost;
    if (!dh.rateinfo.prev_window &&
    (dh.rateinfo.current_rate <= dh.rateinfo.burst)) {
    spin_unlock(&dh.lock);
    local_bh_enable();
    return !(cfg.mode & XT_HASHLIMIT_INVERT);
    } else {
    goto overlimit;
    }
    }
    if (cfg.mode & XT_HASHLIMIT_BYTES)
    cost = hashlimit_byte_cost(skb.len, dh);
    else
    cost = dh.rateinfo.cost;
    if (dh.rateinfo.credit >= cost) {
// below the limit
    dh.rateinfo.credit -= cost;
    spin_unlock(&dh.lock);
    local_bh_enable();
    return !(cfg.mode & XT_HASHLIMIT_INVERT);
    }
    overlimit:
    spin_unlock(&dh.lock);
    local_bh_enable();
// default match is underlimit - so over the limit, we need to invert
    return cfg.mode & XT_HASHLIMIT_INVERT;
    hotdrop:
    par.hotdrop = true;
    return false;
    }
    static bool
    hashlimit_mt_v1(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_hashlimit_mtinfo1 *info = par.matchinfo;
    struct xt_hashlimit_htable *hinfo = info.hinfo;
    let mut cfg: hashlimit_cfg3 = {};
    int ret;
    ret = cfg_copy(&cfg, (void *)&info.cfg, 1);
    if (ret)
    return ret;
    return hashlimit_mt_common(skb, par, hinfo, &cfg, 1);
    }
    static bool
    hashlimit_mt_v2(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_hashlimit_mtinfo2 *info = par.matchinfo;
    struct xt_hashlimit_htable *hinfo = info.hinfo;
    let mut cfg: hashlimit_cfg3 = {};
    int ret;
    ret = cfg_copy(&cfg, (void *)&info.cfg, 2);
    if (ret)
    return ret;
    return hashlimit_mt_common(skb, par, hinfo, &cfg, 2);
    }
    static bool
    hashlimit_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_hashlimit_mtinfo3 *info = par.matchinfo;
    struct xt_hashlimit_htable *hinfo = info.hinfo;
    return hashlimit_mt_common(skb, par, hinfo, &info.cfg, 3);
    }
pub const HASHLIMIT_MAX_SIZE: c_int = 1048576;
    static int hashlimit_mt_check_common(const struct xt_mtchk_param *par,
    struct xt_hashlimit_htable **hinfo,
    struct hashlimit_cfg3 *cfg,
    const char *name, int revision)
    {
    struct net *net = par.net;
    int ret;
    if (cfg.gc_interval == 0 || cfg.expire == 0)
    return -EINVAL;
    if (cfg.size > HASHLIMIT_MAX_SIZE) {
    cfg.size = HASHLIMIT_MAX_SIZE;
    pr_info_ratelimited("size too large, truncated to %u\n", cfg.size);
    }
    if (cfg.max > HASHLIMIT_MAX_SIZE) {
    cfg.max = HASHLIMIT_MAX_SIZE;
    pr_info_ratelimited("max too large, truncated to %u\n", cfg.max);
    }
    if (par.family == NFPROTO_IPV4) {
    if (cfg.srcmask > 32 || cfg.dstmask > 32)
    return -EINVAL;
    } else {
    if (cfg.srcmask > 128 || cfg.dstmask > 128)
    return -EINVAL;
    }
    if (cfg.mode & ~XT_HASHLIMIT_ALL) {
    pr_info_ratelimited("Unknown mode mask %X, kernel too old?\n",
    cfg.mode);
    return -EINVAL;
    }
// Check for overflow.
    if (cfg.mode & XT_HASHLIMIT_RATE_MATCH) {
    if (revision < 3)
    return -EINVAL;
    if (cfg.avg == 0 || cfg.avg > U32_MAX) {
    pr_info_ratelimited("invalid rate\n");
    return -ERANGE;
    }
    if (cfg.interval == 0) {
    pr_info_ratelimited("invalid interval\n");
    return -EINVAL;
    }
    } else if (cfg.mode & XT_HASHLIMIT_BYTES) {
    if (user2credits_byte(cfg.avg) == 0) {
    pr_info_ratelimited("overflow, rate too high: %llu\n",
    cfg.avg);
    return -EINVAL;
    }
    } else if (cfg.burst == 0 ||
    user2credits(cfg.avg * cfg.burst, revision) <
    user2credits(cfg.avg, revision)) {
    pr_info_ratelimited("overflow, try lower: %llu/%llu\n",
    cfg.avg, cfg.burst);
    return -ERANGE;
    }
    mutex_lock(&hashlimit_mutex);
// hinfo = htable_find_get(net, name, par->family);
    if (*hinfo == core::ptr::null_mut()) {
    ret = htable_create(net, cfg, name, par.family,
    hinfo, revision);
    if (ret < 0) {
    mutex_unlock(&hashlimit_mutex);
    return ret;
    }
    } else {
    if ((cfg.mode & XT_HASHLIMIT_RATE_MATCH &&
    !(*hinfo).ratematch) ||
    (!(cfg.mode & XT_HASHLIMIT_RATE_MATCH) &&
    (*hinfo).ratematch)) {
    mutex_unlock(&hashlimit_mutex);
    htable_put(*hinfo);
    return -EINVAL;
    }
    }
    mutex_unlock(&hashlimit_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_check_v1(par: *const xt_mtchk_param) -> c_int {
    static int hashlimit_mt_check_v1(const struct xt_mtchk_param *par)
    {
    struct xt_hashlimit_mtinfo1 *info = par.matchinfo;
    let mut cfg: hashlimit_cfg3 = {};
    int ret;
    ret = xt_check_proc_name(info.name, sizeof(info.name));
    if (ret)
    return ret;
    ret = cfg_copy(&cfg, (void *)&info.cfg, 1);
    if (ret)
    return ret;
    return hashlimit_mt_check_common(par, &info.hinfo,
    &cfg, info.name, 1);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_check_v2(par: *const xt_mtchk_param) -> c_int {
    static int hashlimit_mt_check_v2(const struct xt_mtchk_param *par)
    {
    struct xt_hashlimit_mtinfo2 *info = par.matchinfo;
    let mut cfg: hashlimit_cfg3 = {};
    int ret;
    ret = xt_check_proc_name(info.name, sizeof(info.name));
    if (ret)
    return ret;
    ret = cfg_copy(&cfg, (void *)&info.cfg, 2);
    if (ret)
    return ret;
    return hashlimit_mt_check_common(par, &info.hinfo,
    &cfg, info.name, 2);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int hashlimit_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_hashlimit_mtinfo3 *info = par.matchinfo;
    int ret;
    ret = xt_check_proc_name(info.name, sizeof(info.name));
    if (ret)
    return ret;
    return hashlimit_mt_check_common(par, &info.hinfo, &info.cfg,
    info.name, 3);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_destroy_v2(par: *const xt_mtdtor_param) {
    static void hashlimit_mt_destroy_v2(const struct xt_mtdtor_param *par)
    {
    const struct xt_hashlimit_mtinfo2 *info = par.matchinfo;
    htable_put(info.hinfo);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_destroy_v1(par: *const xt_mtdtor_param) {
    static void hashlimit_mt_destroy_v1(const struct xt_mtdtor_param *par)
    {
    const struct xt_hashlimit_mtinfo1 *info = par.matchinfo;
    htable_put(info.hinfo);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_destroy(par: *const xt_mtdtor_param) {
    static void hashlimit_mt_destroy(const struct xt_mtdtor_param *par)
    {
    const struct xt_hashlimit_mtinfo3 *info = par.matchinfo;
    htable_put(info.hinfo);
    }
    static struct xt_match hashlimit_mt_reg[] __read_mostly = {
    {
    .name           = "hashlimit",
    .revision       = 1,
    .family         = NFPROTO_IPV4,
    .match          = hashlimit_mt_v1,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo1),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo1, hinfo),
    .checkentry     = hashlimit_mt_check_v1,
    .destroy        = hashlimit_mt_destroy_v1,
    .me             = THIS_MODULE,
    },
    {
    .name           = "hashlimit",
    .revision       = 2,
    .family         = NFPROTO_IPV4,
    .match          = hashlimit_mt_v2,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo2),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo2, hinfo),
    .checkentry     = hashlimit_mt_check_v2,
    .destroy        = hashlimit_mt_destroy_v2,
    .me             = THIS_MODULE,
    },
    {
    .name           = "hashlimit",
    .revision       = 3,
    .family         = NFPROTO_IPV4,
    .match          = hashlimit_mt,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo3),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo3, hinfo),
    .checkentry     = hashlimit_mt_check,
    .destroy        = hashlimit_mt_destroy,
    .me             = THIS_MODULE,
    },

    {
    .name           = "hashlimit",
    .revision       = 1,
    .family         = NFPROTO_IPV6,
    .match          = hashlimit_mt_v1,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo1),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo1, hinfo),
    .checkentry     = hashlimit_mt_check_v1,
    .destroy        = hashlimit_mt_destroy_v1,
    .me             = THIS_MODULE,
    },
    {
    .name           = "hashlimit",
    .revision       = 2,
    .family         = NFPROTO_IPV6,
    .match          = hashlimit_mt_v2,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo2),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo2, hinfo),
    .checkentry     = hashlimit_mt_check_v2,
    .destroy        = hashlimit_mt_destroy_v2,
    .me             = THIS_MODULE,
    },
    {
    .name           = "hashlimit",
    .revision       = 3,
    .family         = NFPROTO_IPV6,
    .match          = hashlimit_mt,
    .matchsize      = sizeof(struct xt_hashlimit_mtinfo3),
    .usersize	= offsetof(struct xt_hashlimit_mtinfo3, hinfo),
    .checkentry     = hashlimit_mt_check,
    .destroy        = hashlimit_mt_destroy,
    .me             = THIS_MODULE,
    },

    };
// PROC stuff
    static void *dl_seq_start(struct seq_file *s, loff_t *pos)
    __acquires(htable.lock)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket;
    spin_lock_bh(&htable.lock);
    if (*pos >= htable.cfg.size)
    return core::ptr::null_mut();
    bucket = kmalloc(sizeof(unsigned int), GFP_ATOMIC);
    if (!bucket)
    return ERR_PTR(-ENOMEM);
// bucket = *pos;
    return bucket;
    }
    static void *dl_seq_next(struct seq_file *s, void *v, loff_t *pos)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket = v;
// pos = ++(*bucket);
    if (*pos >= htable.cfg.size) {
    kfree(v);
    return core::ptr::null_mut();
    }
    return bucket;
    }
#[no_mangle]
unsafe extern "C" fn dl_seq_stop(s: *mut seq_file, v: *mut c_void) {
    static void dl_seq_stop(struct seq_file *s, void *v)
    __releases(htable.lock)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket = v;
    if (!IS_ERR(bucket))
    kfree(bucket);
    spin_unlock_bh(&htable.lock);
    }
    static void dl_seq_print(struct dsthash_ent *ent, u_int8_t family,
    struct seq_file *s)
    {
    switch (family) {
    case NFPROTO_IPV4:
    seq_printf(s, "%ld %pI4:%u.%pI4:%u %llu %llu %llu\n",
    (long)(ent.expires - jiffies)/HZ,
    &ent.dst.ip.src,
    ntohs(ent.dst.src_port),
    &ent.dst.ip.dst,
    ntohs(ent.dst.dst_port),
    ent.rateinfo.credit, ent.rateinfo.credit_cap,
    ent.rateinfo.cost);
    break;

    case NFPROTO_IPV6:
    seq_printf(s, "%ld %pI6:%u.%pI6:%u %llu %llu %llu\n",
    (long)(ent.expires - jiffies)/HZ,
    &ent.dst.ip6.src,
    ntohs(ent.dst.src_port),
    &ent.dst.ip6.dst,
    ntohs(ent.dst.dst_port),
    ent.rateinfo.credit, ent.rateinfo.credit_cap,
    ent.rateinfo.cost);
    break;

    default:
    BUG();
    }
    }
    static int dl_seq_real_show_v2(struct dsthash_ent *ent, u_int8_t family,
    struct seq_file *s)
    {
    struct xt_hashlimit_htable *ht = pde_data(file_inode(s.file));
    spin_lock(&ent.lock);
// recalculate to show accurate numbers
    rateinfo_recalc(ent, jiffies, ht.cfg.mode, 2);
    dl_seq_print(ent, family, s);
    spin_unlock(&ent.lock);
    return seq_has_overflowed(s);
    }
    static int dl_seq_real_show_v1(struct dsthash_ent *ent, u_int8_t family,
    struct seq_file *s)
    {
    struct xt_hashlimit_htable *ht = pde_data(file_inode(s.file));
    spin_lock(&ent.lock);
// recalculate to show accurate numbers
    rateinfo_recalc(ent, jiffies, ht.cfg.mode, 1);
    dl_seq_print(ent, family, s);
    spin_unlock(&ent.lock);
    return seq_has_overflowed(s);
    }
    static int dl_seq_real_show(struct dsthash_ent *ent, u_int8_t family,
    struct seq_file *s)
    {
    struct xt_hashlimit_htable *ht = pde_data(file_inode(s.file));
    spin_lock(&ent.lock);
// recalculate to show accurate numbers
    rateinfo_recalc(ent, jiffies, ht.cfg.mode, 3);
    dl_seq_print(ent, family, s);
    spin_unlock(&ent.lock);
    return seq_has_overflowed(s);
    }
#[no_mangle]
unsafe extern "C" fn dl_seq_show_v2(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int dl_seq_show_v2(struct seq_file *s, void *v)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket = (unsigned int *)v;
    struct dsthash_ent *ent;
    if (!hlist_empty(&htable.hash[*bucket])) {
    hlist_for_each_entry(ent, &htable.hash[*bucket], node)
    if (dl_seq_real_show_v2(ent, htable.family, s))
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dl_seq_show_v1(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int dl_seq_show_v1(struct seq_file *s, void *v)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket = v;
    struct dsthash_ent *ent;
    if (!hlist_empty(&htable.hash[*bucket])) {
    hlist_for_each_entry(ent, &htable.hash[*bucket], node)
    if (dl_seq_real_show_v1(ent, htable.family, s))
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dl_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int dl_seq_show(struct seq_file *s, void *v)
    {
    struct xt_hashlimit_htable *htable = pde_data(file_inode(s.file));
    unsigned int *bucket = v;
    struct dsthash_ent *ent;
    if (!hlist_empty(&htable.hash[*bucket])) {
    hlist_for_each_entry(ent, &htable.hash[*bucket], node)
    if (dl_seq_real_show(ent, htable.family, s))
    return -1;
    }
    return 0;
    }
    static const struct seq_operations dl_seq_ops_v1 = {
    .start = dl_seq_start,
    .next  = dl_seq_next,
    .stop  = dl_seq_stop,
    .show  = dl_seq_show_v1
    };
    static const struct seq_operations dl_seq_ops_v2 = {
    .start = dl_seq_start,
    .next  = dl_seq_next,
    .stop  = dl_seq_stop,
    .show  = dl_seq_show_v2
    };
    static const struct seq_operations dl_seq_ops = {
    .start = dl_seq_start,
    .next  = dl_seq_next,
    .stop  = dl_seq_stop,
    .show  = dl_seq_show
    };
#[no_mangle]
unsafe extern "C" fn hashlimit_proc_net_init(net: *mut net) -> int __net_init {
    static int __net_init hashlimit_proc_net_init(struct net *net)
    {
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(net);
    hashlimit_net.ipt_hashlimit = proc_mkdir("ipt_hashlimit", net.proc_net);
    if (!hashlimit_net.ipt_hashlimit)
    return -ENOMEM;

    hashlimit_net.ip6t_hashlimit = proc_mkdir("ip6t_hashlimit", net.proc_net);
    if (!hashlimit_net.ip6t_hashlimit) {
    remove_proc_entry("ipt_hashlimit", net.proc_net);
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_proc_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit hashlimit_proc_net_exit(struct net *net)
    {
    struct xt_hashlimit_htable *hinfo;
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(net);
// hashlimit_net_exit() is called before hashlimit_mt_destroy().
// Make sure that the parent ipt_hashlimit and ip6t_hashlimit proc
// entries is empty before trying to remove it.
//
    mutex_lock(&hashlimit_mutex);
    hlist_for_each_entry(hinfo, &hashlimit_net.htables, node)
    htable_remove_proc_entry(hinfo);
    hashlimit_net.ipt_hashlimit = core::ptr::null_mut();
    hashlimit_net.ip6t_hashlimit = core::ptr::null_mut();
    mutex_unlock(&hashlimit_mutex);
    remove_proc_entry("ipt_hashlimit", net.proc_net);

    remove_proc_entry("ip6t_hashlimit", net.proc_net);

    }
#[no_mangle]
unsafe extern "C" fn hashlimit_net_init(net: *mut net) -> int __net_init {
    static int __net_init hashlimit_net_init(struct net *net)
    {
    struct hashlimit_net *hashlimit_net = hashlimit_pernet(net);
    INIT_HLIST_HEAD(&hashlimit_net.htables);
    return hashlimit_proc_net_init(net);
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_net_exit(net: *mut net) -> void __net_exit {
    static void __net_exit hashlimit_net_exit(struct net *net)
    {
    hashlimit_proc_net_exit(net);
    }
    static struct pernet_operations hashlimit_net_ops = {
    .init	= hashlimit_net_init,
    .exit	= hashlimit_net_exit,
    .id	= &hashlimit_net_id,
    .size	= sizeof(struct hashlimit_net),
    };
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_init() -> int __init {
    static int __init hashlimit_mt_init(void)
    {
    int err;
    err = register_pernet_subsys(&hashlimit_net_ops);
    if (err < 0)
    return err;
    err = xt_register_matches(hashlimit_mt_reg,
    ARRAY_SIZE(hashlimit_mt_reg));
    if (err < 0)
    goto err1;
    err = -ENOMEM;
    hashlimit_cachep = kmem_cache_create("xt_hashlimit",
    sizeof(struct dsthash_ent), 0, 0,
    core::ptr::null_mut());
    if (!hashlimit_cachep) {
    pr_warn("unable to create slab cache\n");
    goto err2;
    }
    return 0;
    err2:
    xt_unregister_matches(hashlimit_mt_reg, ARRAY_SIZE(hashlimit_mt_reg));
    err1:
    unregister_pernet_subsys(&hashlimit_net_ops);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hashlimit_mt_exit() -> void __exit {
    static void __exit hashlimit_mt_exit(void)
    {
    xt_unregister_matches(hashlimit_mt_reg, ARRAY_SIZE(hashlimit_mt_reg));
    unregister_pernet_subsys(&hashlimit_net_ops);
    rcu_barrier();
    kmem_cache_destroy(hashlimit_cachep);
    }
    module_init(hashlimit_mt_init);
    module_exit(hashlimit_mt_exit);
