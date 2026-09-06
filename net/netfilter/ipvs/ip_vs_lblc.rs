//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_lblc.c
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
// IPVS:        Locality-Based Least-Connection scheduling module
//
// Authors:     Wensong Zhang <wensong@gnuchina.org>
//
// Changes:
// Martin Hamilton         :    fixed the terrible locking bugs
// *lock(tbl->lock) ==> *lock(&tbl->lock)
// Wensong Zhang           :    fixed the uninitialized tbl->lock bug
// Wensong Zhang           :    added doing full expiration check to
// collect stale entries of 24+ hours when
// no partial expire check in a half hour
// Julian Anastasov        :    replaced del_timer call with del_timer_sync
// to avoid the possible race between timer
// handler and del_timer thread in SMP
//
// The lblc algorithm is as follows (pseudo code):
//
// if cachenode[dest_ip] is null then
// n, cachenode[dest_ip] <- {weighted least-conn node};
// else
// n <- cachenode[dest_ip];
// if (n is dead) OR
// (n.conns>n.weight AND
// there is a node m with m.conns<m.weight/2) then
// n, cachenode[dest_ip] <- {weighted least-conn node};
//
// return n;
//
// Thanks must go to Wenzhuo Zhang for talking WCCP to me and pushing
// me to write this module.
//

// for sysctl

//
// It is for garbage collection of stale IPVS lblc entries,
// when the table is full.
//

//
// It is for full expiration check.
// When there is no partial expiration check (garbage collection)
// in a half hour, do a full expiration check to collect stale
// entries that haven't been touched for a day.
//
pub const COUNT_FOR_FULL_EXPIRATION: c_int = 30;
//
// for IPVS lblc entry hash table
//

pub const CONFIG_IP_VS_LBLC_TAB_BITS: c_int = 10;

//
// IPVS lblc entry represents an association between destination
// IP address and its destination server
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_lblc_entry {
    pub list: hlist_node,
    pub /: *mut *mut int af; / address family,
    pub /: *mut *mut union nf_inet_addr addr; / destination IP address,
    pub /: *mut *mut *mut ip_vs_dest dest; / real server (cache),
    pub /: *mut *mut unsigned long lastuse; / last used time,
    pub rcu_head: rcu_head,
}

//
// IPVS lblc hash table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_lblc_table {
    pub rcu_head: rcu_head,
    pub /: *mut *mut hlist_head bucket[IP_VS_LBLC_TAB_SIZE]; / hash bucket,
    pub /: *mut *mut timer_list periodic_timer; / collect stale entries,
    pub /: *mut *mut *mut ip_vs_service svc; / pointer back to service,
    pub /: *mut *mut atomic_t entries; / number of entries,
    pub /: *mut *mut int max_size; / maximum size of entries,
    pub /: *mut *mut int rover; / rover for expire check,
    pub /: *mut *mut int counter; / counter for no expire,
    pub dead: bool,
}

//
// IPVS LBLC sysctl table
//

    static struct ctl_table vs_vars_table[] = {
    {
    .procname	= "lblc_expiration",
    .data		= core::ptr::null_mut(),
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_jiffies,
    },
    };

#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_rcu_free(head: *mut rcu_head) {
    static void ip_vs_lblc_rcu_free(struct rcu_head *head)
    {
    struct ip_vs_lblc_entry *en = container_of(head,
    struct ip_vs_lblc_entry,
    rcu_head);
    ip_vs_dest_put_and_free(en.dest);
    kfree(en);
    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_lblc_del(en: *mut ip_vs_lblc_entry) {
    static inline void ip_vs_lblc_del(struct ip_vs_lblc_entry *en)
    {
    hlist_del_rcu(&en.list);
    call_rcu(&en.rcu_head, ip_vs_lblc_rcu_free);
    }
//
// Returns hash value for IPVS LBLC entry
//
    static inline unsigned int
    ip_vs_lblc_hashkey(int af, const union nf_inet_addr *addr)
    {
    let mut addr_fold: __be32 = addr.ip;

    if (af == AF_INET6)
    addr_fold = addr.ip6[0]^addr.ip6[1]^
    addr.ip6[2]^addr.ip6[3];

    return hash_32(ntohl(addr_fold), IP_VS_LBLC_TAB_BITS);
    }
//
// Hash an entry in the ip_vs_lblc_table.
// returns bool success.
//
    static void
    ip_vs_lblc_hash(struct ip_vs_lblc_table *tbl, struct ip_vs_lblc_entry *en)
    {
    let mut hash: c_uint = ip_vs_lblc_hashkey(en.af, &en.addr);
    hlist_add_head_rcu(&en.list, &tbl.bucket[hash]);
    atomic_inc(&tbl.entries);
    }
// Get ip_vs_lblc_entry associated with supplied parameters.
    static inline struct ip_vs_lblc_entry *
    ip_vs_lblc_get(int af, struct ip_vs_lblc_table *tbl,
    const union nf_inet_addr *addr)
    {
    let mut hash: c_uint = ip_vs_lblc_hashkey(af, addr);
    struct ip_vs_lblc_entry *en;
    hlist_for_each_entry_rcu(en, &tbl.bucket[hash], list)
    if (ip_vs_addr_equal(af, &en.addr, addr))
    return en;
    return core::ptr::null_mut();
    }
//
// Create or update an ip_vs_lblc_entry, which is a mapping of a destination IP
// address to a server. Called under spin lock.
//
    static inline struct ip_vs_lblc_entry *
    ip_vs_lblc_new(struct ip_vs_lblc_table *tbl, const union nf_inet_addr *daddr,
    u16 af, struct ip_vs_dest *dest)
    {
    struct ip_vs_lblc_entry *en;
    en = ip_vs_lblc_get(af, tbl, daddr);
    if (en) {
    if (en.dest == dest)
    return en;
    ip_vs_lblc_del(en);
    }
    en = kmalloc_obj(*en, GFP_ATOMIC);
    if (!en)
    return core::ptr::null_mut();
    en.af = af;
    ip_vs_addr_copy(af, &en.addr, daddr);
    en.lastuse = jiffies;
    ip_vs_dest_hold(dest);
    en.dest = dest;
    ip_vs_lblc_hash(tbl, en);
    return en;
    }
//
// Flush all the entries of the specified table.
//
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_flush(svc: *mut ip_vs_service) {
    static void ip_vs_lblc_flush(struct ip_vs_service *svc)
    {
    struct ip_vs_lblc_table *tbl = svc.sched_data;
    struct ip_vs_lblc_entry *en;
    struct hlist_node *next;
    int i;
    spin_lock_bh(&svc.sched_lock);
    tbl.dead = true;
    for (i = 0; i < IP_VS_LBLC_TAB_SIZE; i++) {
    hlist_for_each_entry_safe(en, next, &tbl.bucket[i], list) {
    ip_vs_lblc_del(en);
    atomic_dec(&tbl.entries);
    }
    }
    spin_unlock_bh(&svc.sched_lock);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_lblc_expiration(svc: *mut ip_vs_service) -> c_int {
    static int sysctl_lblc_expiration(struct ip_vs_service *svc)
    {

    return svc.ipvs.sysctl_lblc_expiration;

    return DEFAULT_EXPIRATION;

    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_lblc_full_check(svc: *mut ip_vs_service) {
    static inline void ip_vs_lblc_full_check(struct ip_vs_service *svc)
    {
    struct ip_vs_lblc_table *tbl = svc.sched_data;
    struct ip_vs_lblc_entry *en;
    struct hlist_node *next;
    let mut now: c_ulong = jiffies;
    int i, j;
    for (i = 0, j = tbl.rover; i < IP_VS_LBLC_TAB_SIZE; i++) {
    j = (j + 1) & IP_VS_LBLC_TAB_MASK;
    spin_lock(&svc.sched_lock);
    hlist_for_each_entry_safe(en, next, &tbl.bucket[j], list) {
    if (time_before(now,
    en.lastuse +
    sysctl_lblc_expiration(svc)))
    continue;
    ip_vs_lblc_del(en);
    atomic_dec(&tbl.entries);
    }
    spin_unlock(&svc.sched_lock);
    }
    tbl.rover = j;
    }
//
// Periodical timer handler for IPVS lblc table
// It is used to collect stale entries when the number of entries
// exceeds the maximum size of the table.
//
// Fixme: we probably need more complicated algorithm to collect
// entries that have not been used for a long time even
// if the number of entries doesn't exceed the maximum size
// of the table.
// The full expiration check is for this purpose now.
//
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_check_expire(t: *mut timer_list) {
    static void ip_vs_lblc_check_expire(struct timer_list *t)
    {
    struct ip_vs_lblc_table *tbl = timer_container_of(tbl, t,
    periodic_timer);
    struct ip_vs_service *svc = tbl.svc;
    let mut now: c_ulong = jiffies;
    int goal;
    int i, j;
    struct ip_vs_lblc_entry *en;
    struct hlist_node *next;
    if ((tbl.counter % COUNT_FOR_FULL_EXPIRATION) == 0) {
// do full expiration check
    ip_vs_lblc_full_check(svc);
    tbl.counter = 1;
    goto out;
    }
    if (atomic_read(&tbl.entries) <= tbl.max_size) {
    tbl.counter++;
    goto out;
    }
    goal = (atomic_read(&tbl.entries) - tbl.max_size)*4/3;
    if (goal > tbl.max_size/2)
    goal = tbl.max_size/2;
    for (i = 0, j = tbl.rover; i < IP_VS_LBLC_TAB_SIZE; i++) {
    j = (j + 1) & IP_VS_LBLC_TAB_MASK;
    spin_lock(&svc.sched_lock);
    hlist_for_each_entry_safe(en, next, &tbl.bucket[j], list) {
    if (time_before(now, en.lastuse + ENTRY_TIMEOUT))
    continue;
    ip_vs_lblc_del(en);
    atomic_dec(&tbl.entries);
    goal--;
    }
    spin_unlock(&svc.sched_lock);
    if (goal <= 0)
    break;
    }
    tbl.rover = j;
    out:
    mod_timer(&tbl.periodic_timer, jiffies + CHECK_EXPIRE_INTERVAL);
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_init_svc(svc: *mut ip_vs_service) -> c_int {
    static int ip_vs_lblc_init_svc(struct ip_vs_service *svc)
    {
    int i;
    struct ip_vs_lblc_table *tbl;
//
// Allocate the ip_vs_lblc_table for this service
//
    tbl = kmalloc_obj(*tbl);
    if (tbl == core::ptr::null_mut())
    return -ENOMEM;
    svc.sched_data = tbl;
    IP_VS_DBG(6, "LBLC hash table (memory=%zdbytes) allocated for "
    "current service\n", sizeof(*tbl));
//
// Initialize the hash buckets
//
    for (i = 0; i < IP_VS_LBLC_TAB_SIZE; i++) {
    INIT_HLIST_HEAD(&tbl.bucket[i]);
    }
    tbl.max_size = IP_VS_LBLC_TAB_SIZE*16;
    tbl.rover = 0;
    tbl.counter = 1;
    tbl.dead = false;
    tbl.svc = svc;
    atomic_set(&tbl.entries, 0);
//
// Hook periodic timer for garbage collection
//
    timer_setup(&tbl.periodic_timer, ip_vs_lblc_check_expire, 0);
    mod_timer(&tbl.periodic_timer, jiffies + CHECK_EXPIRE_INTERVAL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_done_svc(svc: *mut ip_vs_service) {
    static void ip_vs_lblc_done_svc(struct ip_vs_service *svc)
    {
    struct ip_vs_lblc_table *tbl = svc.sched_data;
// remove periodic timer
    timer_shutdown_sync(&tbl.periodic_timer);
// got to clean up table entries here
    ip_vs_lblc_flush(svc);
// release the table itself
    kfree_rcu(tbl, rcu_head);
    IP_VS_DBG(6, "LBLC hash table (memory=%zdbytes) released\n",
    sizeof(*tbl));
    }
    static inline struct ip_vs_dest *
    __ip_vs_lblc_schedule(struct ip_vs_service *svc)
    {
    struct ip_vs_dest *dest, *least;
    int loh, doh;
//
// We use the following formula to estimate the load:
// (dest overhead) / dest->weight
//
// Remember -- no floats in kernel mode!!!
// The comparison of h1*w2 > h2*w1 is equivalent to that of
// h1/w1 > h2/w2
// if every weight is larger than zero.
//
// The server with weight=0 is quiesced and will not receive any
// new connection.
//
    list_for_each_entry_rcu(dest, &svc.destinations, n_list) {
    if (dest.flags & IP_VS_DEST_F_OVERLOAD)
    continue;
    if (atomic_read(&dest.weight) > 0) {
    least = dest;
    loh = ip_vs_dest_conn_overhead(least);
    goto nextstage;
    }
    }
    return core::ptr::null_mut();
//
// Find the destination with the least load.
//
    nextstage:
    list_for_each_entry_continue_rcu(dest, &svc.destinations, n_list) {
    if (dest.flags & IP_VS_DEST_F_OVERLOAD)
    continue;
    doh = ip_vs_dest_conn_overhead(dest);
    if ((__s64)loh * atomic_read(&dest.weight) >
    (__s64)doh * atomic_read(&least.weight)) {
    least = dest;
    loh = doh;
    }
    }
    IP_VS_DBG_BUF(6, "LBLC: server %s:%d "
    "activeconns %d refcnt %d weight %d overhead %d\n",
    IP_VS_DBG_ADDR(least.af, &least.addr),
    ntohs(least.port),
    atomic_read(&least.activeconns),
    refcount_read(&least.refcnt),
    atomic_read(&least.weight), loh);
    return least;
    }
//
// If this destination server is overloaded and there is a less loaded
// server, then return true.
//
    static inline int
    is_overloaded(struct ip_vs_dest *dest, struct ip_vs_service *svc)
    {
    if (atomic_read(&dest.activeconns) > atomic_read(&dest.weight)) {
    struct ip_vs_dest *d;
    list_for_each_entry_rcu(d, &svc.destinations, n_list) {
    if (atomic_read(&d.activeconns)*2
    < atomic_read(&d.weight)) {
    return 1;
    }
    }
    }
    return 0;
    }
//
// Locality-Based (weighted) Least-Connection scheduling
//
    static struct ip_vs_dest *
    ip_vs_lblc_schedule(struct ip_vs_service *svc, const struct sk_buff *skb,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_lblc_table *tbl = svc.sched_data;
    struct ip_vs_dest *dest = core::ptr::null_mut();
    struct ip_vs_lblc_entry *en;
    IP_VS_DBG(6, "%s(): Scheduling...\n", __func__);
// First look in our cache
    en = ip_vs_lblc_get(svc.af, tbl, &iph.daddr);
    if (en) {
// We only hold a read lock, but this is atomic
    en.lastuse = jiffies;
//
// If the destination is not available, i.e. it's in the trash,
// we must ignore it, as it may be removed from under our feet,
// if someone drops our reference count. Our caller only makes
// sure that destinations, that are not in the trash, are not
// moved to the trash, while we are scheduling. But anyone can
// free up entries from the trash at any time.
//
    dest = en.dest;
    if ((dest.cflags & IP_VS_DEST_CF_AVAILABLE) &&
    atomic_read(&dest.weight) > 0 && !is_overloaded(dest, svc))
    goto out;
    }
// No cache entry or it is invalid, time to schedule
    dest = __ip_vs_lblc_schedule(svc);
    if (!dest) {
    ip_vs_scheduler_err(svc, "no destination available");
    return core::ptr::null_mut();
    }
// If we fail to create a cache entry, we'll just use the valid dest
    spin_lock_bh(&svc.sched_lock);
    if (!tbl.dead)
    ip_vs_lblc_new(tbl, &iph.daddr, svc.af, dest);
    spin_unlock_bh(&svc.sched_lock);
    out:
    IP_VS_DBG_BUF(6, "LBLC: destination IP address %s -. server %s:%d\n",
    IP_VS_DBG_ADDR(svc.af, &iph.daddr),
    IP_VS_DBG_ADDR(dest.af, &dest.addr), ntohs(dest.port));
    return dest;
    }
//
// IPVS LBLC Scheduler structure
//
    static struct ip_vs_scheduler ip_vs_lblc_scheduler = {
    .name =			"lblc",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_lblc_scheduler.n_list),
    .init_service =		ip_vs_lblc_init_svc,
    .done_service =		ip_vs_lblc_done_svc,
    .schedule =		ip_vs_lblc_schedule,
    };
//
// per netns init.
//

#[no_mangle]
unsafe extern "C" fn __ip_vs_lblc_init(net: *mut net) -> int __net_init {
    static int __net_init __ip_vs_lblc_init(struct net *net)
    {
    struct netns_ipvs *ipvs = net_ipvs(net);
    let mut vars_table_size: usize = ARRAY_SIZE(vs_vars_table);
    if (!ipvs)
    return -ENOENT;
    if (!net_eq(net, &init_net)) {
    ipvs.lblc_ctl_table = kmemdup(vs_vars_table,
    sizeof(vs_vars_table),
    GFP_KERNEL);
    if (ipvs.lblc_ctl_table == core::ptr::null_mut())
    return -ENOMEM;
// Don't export sysctls to unprivileged users
    if (net.user_ns != &init_user_ns)
    vars_table_size = 0;
    } else
    ipvs.lblc_ctl_table = vs_vars_table;
    ipvs.sysctl_lblc_expiration = DEFAULT_EXPIRATION;
    ipvs.lblc_ctl_table[0].data = &ipvs.sysctl_lblc_expiration;
    ipvs.lblc_ctl_header = register_net_sysctl_sz(net, "net/ipv4/vs",
    ipvs.lblc_ctl_table,
    vars_table_size);
    if (!ipvs.lblc_ctl_header) {
    if (!net_eq(net, &init_net))
    kfree(ipvs.lblc_ctl_table);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __ip_vs_lblc_exit(net: *mut net) -> void __net_exit {
    static void __net_exit __ip_vs_lblc_exit(struct net *net)
    {
    struct netns_ipvs *ipvs = net_ipvs(net);
    unregister_net_sysctl_table(ipvs.lblc_ctl_header);
    if (!net_eq(net, &init_net))
    kfree(ipvs.lblc_ctl_table);
    }

    static int __net_init __ip_vs_lblc_init(struct net *net) { return 0; }
    static void __net_exit __ip_vs_lblc_exit(struct net *net) { }

    static struct pernet_operations ip_vs_lblc_ops = {
    .init = __ip_vs_lblc_init,
    .exit = __ip_vs_lblc_exit,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_init() -> int __init {
    static int __init ip_vs_lblc_init(void)
    {
    int ret;
    ret = register_pernet_subsys(&ip_vs_lblc_ops);
    if (ret)
    return ret;
    ret = register_ip_vs_scheduler(&ip_vs_lblc_scheduler);
    if (ret)
    unregister_pernet_subsys(&ip_vs_lblc_ops);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_lblc_cleanup() -> void __exit {
    static void __exit ip_vs_lblc_cleanup(void)
    {
    unregister_ip_vs_scheduler(&ip_vs_lblc_scheduler);
    unregister_pernet_subsys(&ip_vs_lblc_ops);
    rcu_barrier();
    }
    module_init(ip_vs_lblc_init);
    module_exit(ip_vs_lblc_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs locality-based least-connection scheduler");
