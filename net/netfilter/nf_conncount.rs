//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conncount.c
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
// count the number of connections matching an arbitrary key.
//
// (C) 2017 Red Hat GmbH
// Author: Florian Westphal <fw@strlen.de>
//
// split from xt_connlimit.c:
// (c) 2000 Gerd Knorr <kraxel@bytesex.org>
// Nov 2002: Martin Bene <martin.bene@icomedias.com>:
// only ignore TIME_WAIT or gone connections
// (C) CC Computer Consultants GmbH, 2007
//

pub const CONNCOUNT_GC_MAX_NODES: c_int = 8;
pub const CONNCOUNT_GC_MAX_COLLECT: c_int = 64;
pub const MAX_KEYLEN: c_int = 5;
// we will save the tuples of all connections we care about
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_tuple {
    pub node: list_head,
    pub tuple: nf_conntrack_tuple,
    pub zone: nf_conntrack_zone,
    pub cpu: c_int,
    pub jiffies32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_rb {
    pub node: rb_node,
    pub list: nf_conncount_list,
    pub key: [u32; MAX_KEYLEN],
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_root {
    pub root: rb_root,
    pub lock: spinlock_t,
    pub count: seqcount_spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_data {
    pub keylen: c_uint,
    pub initval: u32,
    pub root: [nf_conncount_root; CONNCOUNT_SLOTS],
    pub net: *mut net,
    pub gc_work: work_struct,
    pub pending_trees: [c_ulong; BITS_TO_LONGS(CONNCOUNT_SLOTS)],
    pub gc_tree: c_uint,
}

    static struct kmem_cache *conncount_rb_cachep __read_mostly;
    static struct kmem_cache *conncount_conn_cachep __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn already_closed(conn: *const nf_conn) -> bool {
    static inline bool already_closed(const struct nf_conn *conn)
    {
    if (nf_ct_protonum(conn) == IPPROTO_TCP)
    return conn.proto.tcp.state == TCP_CONNTRACK_TIME_WAIT ||
    conn.proto.tcp.state == TCP_CONNTRACK_CLOSE;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn key_diff(a: *const u32, b: *const u32, klen: c_uint) -> c_int {
    static int key_diff(const u32 *a, const u32 *b, unsigned int klen)
    {
    return memcmp(a, b, klen * sizeof(u32));
    }
    static void conn_free(struct nf_conncount_list *list,
    struct nf_conncount_tuple *conn)
    {
    lockdep_assert_held(&list.list_lock);
    list.count--;
    list_del(&conn.node);
    kmem_cache_free(conncount_conn_cachep, conn);
    }
    static const struct nf_conntrack_tuple_hash *
    find_or_evict(struct net *net, struct nf_conncount_list *list,
    struct nf_conncount_tuple *conn)
    {
    const struct nf_conntrack_tuple_hash *found;
    unsigned long a, b;
    let mut cpu: c_int = raw_smp_processor_id();
    u32 age;
    found = nf_conntrack_find_get(net, &conn.zone, &conn.tuple);
    if (found)
    return found;
    b = conn.jiffies32;
    a = (u32)jiffies;
// conn might have been added just before by another cpu and
// might still be unconfirmed.  In this case, nf_conntrack_find()
// returns no result.  Thus only evict if this cpu added the
// stale entry or if the entry is older than two jiffies.
//
    age = a - b;
    if (conn.cpu == cpu || age >= 2) {
    conn_free(list, conn);
    return ERR_PTR(-ENOENT);
    }
    return ERR_PTR(-EAGAIN);
    }
    static bool get_ct_or_tuple_from_skb(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conn **ct,
    struct nf_conntrack_tuple *tuple,
    const struct nf_conntrack_zone **zone,
    bool *refcounted)
    {
    const struct nf_conntrack_tuple_hash *h;
    enum ip_conntrack_info ctinfo;
    struct nf_conn *found_ct;
    found_ct = nf_ct_get(skb, &ctinfo);
    if (found_ct && !nf_ct_is_template(found_ct)) {
// tuple = found_ct->tuplehash[IP_CT_DIR_ORIGINAL].tuple;
// zone = nf_ct_zone(found_ct);
// ct = found_ct;
    return true;
    }
    if (!nf_ct_get_tuplepr(skb, skb_network_offset(skb), l3num, net, tuple))
    return false;
    if (found_ct)
// zone = nf_ct_zone(found_ct);
    h = nf_conntrack_find_get(net, *zone, tuple);
    if (!h)
    return true;
    found_ct = nf_ct_tuplehash_to_ctrack(h);
// tuple = found_ct->tuplehash[IP_CT_DIR_ORIGINAL].tuple;
// zone = nf_ct_zone(found_ct);
// refcounted = true;
// ct = found_ct;
    return true;
    }
    static int __nf_conncount_add(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conncount_list *list)
    {
    const struct nf_conntrack_zone *zone = &nf_ct_zone_dflt;
    const struct nf_conntrack_tuple_hash *found;
    struct nf_conncount_tuple *conn, *conn_n;
    struct nf_conntrack_tuple tuple;
    struct nf_conn *ct = core::ptr::null_mut();
    struct nf_conn *found_ct;
    let mut collect: c_uint = 0;
    let mut refcounted: bool = false;
    let mut err: c_int = 0;
    if (!get_ct_or_tuple_from_skb(net, skb, l3num, &ct, &tuple, &zone, &refcounted))
    return -ENOENT;
    if (ct && nf_ct_is_confirmed(ct)) {
// Connection is confirmed but might still be in the setup phase.
// Only skip the tracking if it is fully assured. This guarantees
// that setup packets or retransmissions are properly counted and
// deduplicated.
//
    if (test_bit(IPS_ASSURED_BIT, &ct.status)) {
    err = -EEXIST;
    goto out_put;
    }
    goto check_connections;
    }
    if ((u32)jiffies == list.last_gc &&
    (list.count - list.last_gc_count) < CONNCOUNT_GC_MAX_COLLECT)
    goto add_new_node;
    check_connections:
// check the saved connections
    list_for_each_entry_safe(conn, conn_n, &list.head, node) {
    if (collect > CONNCOUNT_GC_MAX_COLLECT)
    break;
    found = find_or_evict(net, list, conn);
    if (IS_ERR(found)) {
// Not found, but might be about to be confirmed
    if (PTR_ERR(found) == -EAGAIN) {
    if (nf_ct_tuple_equal(&conn.tuple, &tuple) &&
    nf_ct_zone_id(&conn.zone, IP_CT_DIR_ORIGINAL) ==
    nf_ct_zone_id(zone, IP_CT_DIR_ORIGINAL))
    goto out_put; /* already exists */
    } else {
    collect++;
    }
    continue;
    }
    found_ct = nf_ct_tuplehash_to_ctrack(found);
    if (nf_ct_tuple_equal(&conn.tuple, &tuple) &&
    nf_ct_zone_equal(found_ct, zone, IP_CT_DIR_ORIGINAL)) {
//
// We should not see tuples twice unless someone hooks
// this into a table without "-p tcp --syn".
//
// Attempt to avoid a re-add in this case.
//
    nf_ct_put(found_ct);
    goto out_put;
    } else if (already_closed(found_ct)) {
//
// we do not care about connections which are
// closed already -> ditch it
//
    nf_ct_put(found_ct);
    conn_free(list, conn);
    collect++;
    continue;
    }
    nf_ct_put(found_ct);
    }
    list.last_gc = (u32)jiffies;
    list.last_gc_count = list.count;
    add_new_node:
    if (unlikely(list.count > INT_MAX)) {
    DEBUG_NET_WARN_ON_ONCE(1);
    err = -EOVERFLOW;
    goto out_put;
    }
    conn = kmem_cache_alloc(conncount_conn_cachep, GFP_ATOMIC);
    if (conn == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out_put;
    }
    conn.tuple = tuple;
    conn.zone = *zone;
    conn.cpu = raw_smp_processor_id();
    conn.jiffies32 = (u32)jiffies;
    list_add_tail(&conn.node, &list.head);
    list.count++;
    out_put:
    if (refcounted)
    nf_ct_put(ct);
    return err;
    }
    int nf_conncount_add_skb(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conncount_list *list)
    {
    int ret;
// check the saved connections
    spin_lock_bh(&list.list_lock);
    ret = __nf_conncount_add(net, skb, l3num, list);
    spin_unlock_bh(&list.list_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_conncount_add_skb);
#[no_mangle]
pub unsafe extern "C" fn nf_conncount_list_init(list: *mut nf_conncount_list) {
    void nf_conncount_list_init(struct nf_conncount_list *list)
    {
    spin_lock_init(&list.list_lock);
    INIT_LIST_HEAD(&list.head);
    list.count = 0;
    list.last_gc_count = 0;
    list.last_gc = (u32)jiffies;
    }
    EXPORT_SYMBOL_GPL(nf_conncount_list_init);
// Return true if the list is empty. Must be called with BH disabled.
    static bool __nf_conncount_gc_list(struct net *net,
    struct nf_conncount_list *list)
    {
    const struct nf_conntrack_tuple_hash *found;
    struct nf_conncount_tuple *conn, *conn_n;
    struct nf_conn *found_ct;
    let mut collected: c_uint = 0;
    let mut ret: bool = false;
// don't bother if we just did GC
    if ((u32)jiffies == READ_ONCE(list.last_gc))
    return false;
    list_for_each_entry_safe(conn, conn_n, &list.head, node) {
    found = find_or_evict(net, list, conn);
    if (IS_ERR(found)) {
    if (PTR_ERR(found) == -ENOENT)
    collected++;
    continue;
    }
    found_ct = nf_ct_tuplehash_to_ctrack(found);
    if (already_closed(found_ct)) {
//
// we do not care about connections which are
// closed already -> ditch it
//
    nf_ct_put(found_ct);
    conn_free(list, conn);
    collected++;
    continue;
    }
    nf_ct_put(found_ct);
    if (collected > CONNCOUNT_GC_MAX_COLLECT)
    break;
    }
    if (!list.count)
    ret = true;
    list.last_gc = (u32)jiffies;
    list.last_gc_count = list.count;
    return ret;
    }
    bool nf_conncount_gc_list(struct net *net,
    struct nf_conncount_list *list)
    {
    bool ret;
// don't bother if other cpu is already doing GC
    if (!spin_trylock_bh(&list.list_lock))
    return false;
    ret = __nf_conncount_gc_list(net, list);
    spin_unlock_bh(&list.list_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_conncount_gc_list);
#[no_mangle]
unsafe extern "C" fn __tree_nodes_free(h: *mut rcu_head) {
    static void __tree_nodes_free(struct rcu_head *h)
    {
    struct nf_conncount_rb *rbconn;
    rbconn = container_of(h, struct nf_conncount_rb, rcu_head);
    kmem_cache_free(conncount_rb_cachep, rbconn);
    }
    static void tree_nodes_free(struct nf_conncount_root *root,
    struct nf_conncount_rb *gc_nodes[],
    unsigned int gc_count)
    {
    struct nf_conncount_rb *rbconn;
    lockdep_assert_held(&root.lock);
    while (gc_count) {
    rbconn = gc_nodes[--gc_count];
    spin_lock(&rbconn.list.list_lock);
    if (!rbconn.list.count) {
    write_seqcount_begin(&root.count);
    rb_erase(&rbconn.node, &root.root);
    call_rcu(&rbconn.rcu_head, __tree_nodes_free);
    write_seqcount_end(&root.count);
    }
    spin_unlock(&rbconn.list.list_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn schedule_gc_worker(data: *mut nf_conncount_data, tree: c_int) {
    static void schedule_gc_worker(struct nf_conncount_data *data, int tree)
    {
    set_bit(tree, data.pending_trees);
    schedule_work(&data.gc_work);
    }
    static unsigned int
    insert_tree(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conncount_data *data,
    unsigned int hash,
    const u32 *key)
    {
    struct nf_conncount_root *root = &data.root[hash];
    struct nf_conncount_rb *gc_nodes[CONNCOUNT_GC_MAX_NODES];
    const struct nf_conntrack_zone *zone = &nf_ct_zone_dflt;
    let mut do_gc: bool = true, refcounted = false;
    let mut count: c_uint = 0, gc_count = 0;
    struct rb_node **rbnode, *parent;
    struct nf_conntrack_tuple tuple;
    struct nf_conncount_tuple *conn;
    struct nf_conncount_rb *rbconn;
    struct nf_conn *ct = core::ptr::null_mut();
    spin_lock_bh(&root.lock);
    restart:
    parent = core::ptr::null_mut();
    rbnode = &root.root.rb_node;
    while (*rbnode) {
    int diff;
    rbconn = rb_entry(*rbnode, struct nf_conncount_rb, node);
    parent = *rbnode;
    diff = key_diff(key, rbconn.key, data.keylen);
    if (diff < 0) {
    rbnode = &((*rbnode).rb_left);
    } else if (diff > 0) {
    rbnode = &((*rbnode).rb_right);
    } else {
    int ret;
    ret = nf_conncount_add_skb(net, skb, l3num, &rbconn.list);
    if (ret && ret != -EEXIST)
    count = 0; /* hotdrop */
    else
    count = rbconn.list.count;
    tree_nodes_free(root, gc_nodes, gc_count);
    goto out_unlock;
    }
    if (gc_count >= ARRAY_SIZE(gc_nodes))
    continue;
    if (do_gc && nf_conncount_gc_list(net, &rbconn.list))
    gc_nodes[gc_count++] = rbconn;
    }
    if (gc_count) {
    tree_nodes_free(root, gc_nodes, gc_count);
    schedule_gc_worker(data, hash);
    gc_count = 0;
    do_gc = false;
    goto restart;
    }
    if (get_ct_or_tuple_from_skb(net, skb, l3num, &ct, &tuple, &zone, &refcounted)) {
// expected case: match, insert new node
    rbconn = kmem_cache_alloc(conncount_rb_cachep, GFP_ATOMIC);
    if (rbconn == core::ptr::null_mut())
    goto out_unlock;
    conn = kmem_cache_alloc(conncount_conn_cachep, GFP_ATOMIC);
    if (conn == core::ptr::null_mut()) {
    kmem_cache_free(conncount_rb_cachep, rbconn);
    goto out_unlock;
    }
    conn.tuple = tuple;
    conn.zone = *zone;
    conn.cpu = raw_smp_processor_id();
    conn.jiffies32 = (u32)jiffies;
    memcpy(rbconn.key, key, sizeof(u32) * data.keylen);
    nf_conncount_list_init(&rbconn.list);
    list_add(&conn.node, &rbconn.list.head);
    count = 1;
    rbconn.list.count = count;
    write_seqcount_begin(&root.count);
    rb_link_node_rcu(&rbconn.node, parent, rbnode);
    rb_insert_color(&rbconn.node, &root.root);
    write_seqcount_end(&root.count);
    }
    out_unlock:
    if (refcounted)
    nf_ct_put(ct);
    spin_unlock_bh(&root.lock);
    return count;
    }
    static struct nf_conncount_rb *
    find_tree_node(struct nf_conncount_root *root, struct nf_conncount_data *data,
    const u32 *key)
    {
    let mut seq: c_uint = read_seqcount_begin(&root.count);
    struct rb_node *parent;
    parent = rcu_dereference_check(root.root.rb_node,
    lockdep_is_held(&root.lock));
    while (parent) {
    struct nf_conncount_rb *rbconn;
    int diff;
    rbconn = rb_entry(parent, struct nf_conncount_rb, node);
    diff = key_diff(key, rbconn.key, data.keylen);
    if (diff < 0)
    parent = rcu_dereference_check(parent.rb_left,
    lockdep_is_held(&root.lock));
#[no_mangle]
pub unsafe extern "C" fn if(0: diff >) -> else {
    else if (diff > 0)
    parent = rcu_dereference_check(parent.rb_right,
    lockdep_is_held(&root.lock));
    else
    return rbconn;
    if (read_seqcount_retry(&root.count, seq))
    return ERR_PTR(-EAGAIN);
    }
    if (read_seqcount_retry(&root.count, seq))
    return ERR_PTR(-EAGAIN);
    return ERR_PTR(-ENOENT);
    }
    static unsigned int
    count_tree(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conncount_data *data,
    const u32 *key)
    {
    struct nf_conncount_root *root;
    struct nf_conncount_rb *rbconn;
    unsigned int hash;
    int ret;
    hash = jhash2(key, data.keylen, data.initval) % CONNCOUNT_SLOTS;
    root = &data.root[hash];
    rbconn = find_tree_node(root, data, key);
    if (IS_ERR(rbconn)) {
    if (PTR_ERR(rbconn) == -EAGAIN) {
    spin_lock_bh(&root.lock);
    rbconn = find_tree_node(root, data, key);
    spin_unlock_bh(&root.lock);
    }
    if (PTR_ERR(rbconn) == -ENOENT) {
    if (!skb)
    return 0;
    return insert_tree(net, skb, l3num, data, hash, key);
    }
    DEBUG_NET_WARN_ON_ONCE(IS_ERR(rbconn));
    }
    DEBUG_NET_WARN_ON_ONCE(IS_ERR_OR_NULL(rbconn));
    if (IS_ERR_OR_NULL(rbconn))
    return 0;
    if (!skb) {
    nf_conncount_gc_list(net, &rbconn.list);
    return rbconn.list.count;
    }
    spin_lock_bh(&rbconn.list.list_lock);
// Node might be about to be free'd.
// We need to defer to insert_tree() in this case.
//
    if (rbconn.list.count == 0) {
    spin_unlock_bh(&rbconn.list.list_lock);
    return insert_tree(net, skb, l3num, data, hash, key);
    }
// same source network -> be counted!
    ret = __nf_conncount_add(net, skb, l3num, &rbconn.list);
    spin_unlock_bh(&rbconn.list.list_lock);
    if (ret && ret != -EEXIST)
    return 0; /* hotdrop */
// -EEXIST means add was skipped, update the list
    if (ret == -EEXIST)
    nf_conncount_gc_list(net, &rbconn.list);
    return rbconn.list.count;
    }
#[no_mangle]
unsafe extern "C" fn tree_gc_worker(work: *mut work_struct) {
    static void tree_gc_worker(struct work_struct *work)
    {
    struct nf_conncount_data *data = container_of(work, struct nf_conncount_data, gc_work);
    struct nf_conncount_rb *gc_nodes[CONNCOUNT_GC_MAX_NODES], *rbconn;
    unsigned int tree, next_tree, gc_count = 0;
    struct nf_conncount_root *root;
    struct rb_node *node;
    if (data.gc_tree == 0)
    data.gc_tree = find_first_bit(data.pending_trees, CONNCOUNT_SLOTS);
    tree = data.gc_tree % CONNCOUNT_SLOTS;
    root = &data.root[tree];
    spin_lock_bh(&root.lock);
    gc_count = 0;
    node = rb_first(&root.root);
    while (node != core::ptr::null_mut()) {
    u32 key[MAX_KEYLEN];
    bool drop_lock;
    rbconn = rb_entry(node, struct nf_conncount_rb, node);
    node = rb_next(node);
    if (nf_conncount_gc_list(data.net, &rbconn.list))
    gc_nodes[gc_count++] = rbconn;
    drop_lock = need_resched();
    if (drop_lock || gc_count >= ARRAY_SIZE(gc_nodes)) {
    tree_nodes_free(root, gc_nodes, gc_count);
    gc_count = 0;
    }
    if (!drop_lock || !node)
    continue;
    rbconn = rb_entry(node, struct nf_conncount_rb, node);
    memcpy(key, rbconn.key, sizeof(key));
    spin_unlock_bh(&root.lock);
    cond_resched();
    spin_lock_bh(&root.lock);
    rbconn = find_tree_node(root, data, key);
    if (IS_ERR_OR_NULL(rbconn)) /* rbconn was reaped */
    break;
    node = &rbconn.node;
    }
    tree_nodes_free(root, gc_nodes, gc_count);
    clear_bit(tree, data.pending_trees);
    next_tree = (tree + 1) % CONNCOUNT_SLOTS;
    next_tree = find_next_bit(data.pending_trees, CONNCOUNT_SLOTS, next_tree);
    if (next_tree < CONNCOUNT_SLOTS) {
    data.gc_tree = next_tree;
    schedule_work(work);
    } else {
    data.gc_tree = 0;
    }
    spin_unlock_bh(&root.lock);
    }
// Count and return number of conntrack entries in 'net' with particular 'key'.
// If 'skb' is not null, insert the corresponding tuple into the accounting
// data structure. Call with RCU read lock.
//
    unsigned int nf_conncount_count_skb(struct net *net,
    const struct sk_buff *skb,
    u16 l3num,
    struct nf_conncount_data *data,
    const u32 *key)
    {
    return count_tree(net, skb, l3num, data, key);
    }
    EXPORT_SYMBOL_GPL(nf_conncount_count_skb);
#[no_mangle]
unsafe extern "C" fn nf_conncount_root_init(r: *mut nf_conncount_root) {
    static void nf_conncount_root_init(struct nf_conncount_root *r)
    {
    r.root = RB_ROOT;
    spin_lock_init(&r.lock);
    seqcount_spinlock_init(&r.count, &r.lock);
    }
    struct nf_conncount_data *nf_conncount_init(struct net *net, unsigned int keylen)
    {
    struct nf_conncount_data *data;
    int i;
    if (keylen % sizeof(u32) ||
    keylen / sizeof(u32) > MAX_KEYLEN ||
    keylen == 0)
    return ERR_PTR(-EINVAL);
    data = kvzalloc_obj(*data);
    if (!data)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < ARRAY_SIZE(data.root); ++i)
    nf_conncount_root_init(&data.root[i]);
    data.keylen = keylen / sizeof(u32);
    data.net = net;
    data.initval = get_random_u32();
    INIT_WORK(&data.gc_work, tree_gc_worker);
    return data;
    }
    EXPORT_SYMBOL_GPL(nf_conncount_init);
#[no_mangle]
pub unsafe extern "C" fn nf_conncount_cache_free(list: *mut nf_conncount_list) {
    void nf_conncount_cache_free(struct nf_conncount_list *list)
    {
    struct nf_conncount_tuple *conn, *conn_n;
    list_for_each_entry_safe(conn, conn_n, &list.head, node)
    kmem_cache_free(conncount_conn_cachep, conn);
    }
    EXPORT_SYMBOL_GPL(nf_conncount_cache_free);
#[no_mangle]
unsafe extern "C" fn destroy_tree(r: *mut nf_conncount_root) {
    static void destroy_tree(struct nf_conncount_root *r)
    {
    struct nf_conncount_rb *rbconn;
    struct rb_node *node;
    while ((node = rb_first(&r.root)) != core::ptr::null_mut()) {
    rbconn = rb_entry(node, struct nf_conncount_rb, node);
    rb_erase(node, &r.root);
    nf_conncount_cache_free(&rbconn.list);
    kmem_cache_free(conncount_rb_cachep, rbconn);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn nf_conncount_destroy(net: *mut net, data: *mut nf_conncount_data) {
    void nf_conncount_destroy(struct net *net, struct nf_conncount_data *data)
    {
    unsigned int i;
    disable_work_sync(&data.gc_work);
    for (i = 0; i < ARRAY_SIZE(data.root); ++i)
    destroy_tree(&data.root[i]);
    kvfree(data);
    }
    EXPORT_SYMBOL_GPL(nf_conncount_destroy);
#[no_mangle]
unsafe extern "C" fn nf_conncount_modinit() -> int __init {
    static int __init nf_conncount_modinit(void)
    {
    conncount_conn_cachep = KMEM_CACHE(nf_conncount_tuple, 0);
    if (!conncount_conn_cachep)
    return -ENOMEM;
    conncount_rb_cachep = KMEM_CACHE(nf_conncount_rb, 0);
    if (!conncount_rb_cachep) {
    kmem_cache_destroy(conncount_conn_cachep);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nf_conncount_modexit() -> void __exit {
    static void __exit nf_conncount_modexit(void)
    {
    rcu_barrier();
    kmem_cache_destroy(conncount_conn_cachep);
    kmem_cache_destroy(conncount_rb_cachep);
    }
    module_init(nf_conncount_modinit);
    module_exit(nf_conncount_modexit);
    MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_DESCRIPTION("netfilter: count number of connections matching a key");
    MODULE_LICENSE("GPL");
