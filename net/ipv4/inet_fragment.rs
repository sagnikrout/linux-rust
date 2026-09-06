//! Automatically rewritten from C to Rust
//! Source: net/ipv4/inet_fragment.c
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
// inet fragments management
//
// Authors:	Pavel Emelyanov <xemul@openvz.org>
// Started as consolidation of ipv4/ip_fragment.c,
// ipv6/reassembly. and ipv6 nf conntrack reassembly
//

// Use skb->cb to track consecutive/adjacent fragments coming at
// the end of the queue. Nodes in the rb-tree queue will
// contain "runs" of one or more adjacent fragments.
//
// Invariants:
// - next_frag is NULL at the tail of a "run";
// - the head of a "run" has the sum of all fragment lengths in frag_run_len.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipfrag_skb_cb {
    union {
    pub h4: inet_skb_parm,
    pub h6: inet6_skb_parm,
}

    struct sk_buff		*next_frag;
    int			frag_run_len;
    int			ip_defrag_offset;
    };

#[no_mangle]
unsafe extern "C" fn fragcb_clear(skb: *mut sk_buff) {
    static void fragcb_clear(struct sk_buff *skb)
    {
    RB_CLEAR_NODE(&skb.rbnode);
    FRAG_CB(skb).next_frag = core::ptr::null_mut();
    FRAG_CB(skb).frag_run_len = skb.len;
    }
// Append skb to the last "run".
    static void fragrun_append_to_last(struct inet_frag_queue *q,
    struct sk_buff *skb)
    {
    fragcb_clear(skb);
    FRAG_CB(q.last_run_head).frag_run_len += skb.len;
    FRAG_CB(q.fragments_tail).next_frag = skb;
    q.fragments_tail = skb;
    }
// Create a new "run" with the skb.
#[no_mangle]
unsafe extern "C" fn fragrun_create(q: *mut inet_frag_queue, skb: *mut sk_buff) {
    static void fragrun_create(struct inet_frag_queue *q, struct sk_buff *skb)
    {
    BUILD_BUG_ON(sizeof(struct ipfrag_skb_cb) > sizeof(skb.cb));
    fragcb_clear(skb);
    if (q.last_run_head)
    rb_link_node(&skb.rbnode, &q.last_run_head.rbnode,
    &q.last_run_head.rbnode.rb_right);
    else
    rb_link_node(&skb.rbnode, core::ptr::null_mut(), &q.rb_fragments.rb_node);
    rb_insert_color(&skb.rbnode, &q.rb_fragments);
    q.fragments_tail = skb;
    q.last_run_head = skb;
    }
// Given the OR values of all fragments, apply RFC 3168 5.3 requirements
// Value : 0xff if frame should be dropped.
// 0 or INET_ECN_CE value, to be ORed in to final iph->tos field
//
    const u8 ip_frag_ecn_table[16] = {
// at least one fragment had CE, and others ECT_0 or ECT_1
    [IPFRAG_ECN_CE | IPFRAG_ECN_ECT_0]			= INET_ECN_CE,
    [IPFRAG_ECN_CE | IPFRAG_ECN_ECT_1]			= INET_ECN_CE,
    [IPFRAG_ECN_CE | IPFRAG_ECN_ECT_0 | IPFRAG_ECN_ECT_1]	= INET_ECN_CE,
// invalid combinations : drop frame
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_CE] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_ECT_0] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_ECT_1] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_ECT_0 | IPFRAG_ECN_ECT_1] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_CE | IPFRAG_ECN_ECT_0] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_CE | IPFRAG_ECN_ECT_1] = 0xff,
    [IPFRAG_ECN_NOT_ECT | IPFRAG_ECN_CE | IPFRAG_ECN_ECT_0 | IPFRAG_ECN_ECT_1] = 0xff,
    };
    EXPORT_SYMBOL(ip_frag_ecn_table);
#[no_mangle]
pub unsafe extern "C" fn inet_frags_init(f: *mut inet_frags) -> c_int {
    int inet_frags_init(struct inet_frags *f)
    {
    f.frags_cachep = kmem_cache_create(f.frags_cache_name, f.qsize, 0, 0,
    core::ptr::null_mut());
    if (!f.frags_cachep)
    return -ENOMEM;
    refcount_set(&f.refcnt, 1);
    init_completion(&f.completion);
    return 0;
    }
    EXPORT_SYMBOL(inet_frags_init);
#[no_mangle]
pub unsafe extern "C" fn inet_frags_fini(f: *mut inet_frags) {
    void inet_frags_fini(struct inet_frags *f)
    {
    if (refcount_dec_and_test(&f.refcnt))
    complete(&f.completion);
    wait_for_completion(&f.completion);
    kmem_cache_destroy(f.frags_cachep);
    f.frags_cachep = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(inet_frags_fini);
// called from rhashtable_free_and_destroy() at netns_frags dismantle
#[no_mangle]
unsafe extern "C" fn inet_frags_free_cb(ptr: *mut c_void, arg: *mut c_void) {
    static void inet_frags_free_cb(void *ptr, void *arg)
    {
    struct inet_frag_queue *fq = ptr;
    int count;
    count = timer_delete_sync(&fq.timer) ? 1 : 0;
    spin_lock_bh(&fq.lock);
    fq.flags |= INET_FRAG_DROP;
    if (!(fq.flags & INET_FRAG_COMPLETE)) {
    fq.flags |= INET_FRAG_COMPLETE;
    count++;
    } else if (fq.flags & INET_FRAG_HASH_DEAD) {
    count++;
    }
    spin_unlock_bh(&fq.lock);
    inet_frag_putn(fq, count);
    }
    static LLIST_HEAD(fqdir_free_list);
#[no_mangle]
unsafe extern "C" fn fqdir_free_fn(work: *mut work_struct) {
    static void fqdir_free_fn(struct work_struct *work)
    {
    struct llist_node *kill_list;
    struct fqdir *fqdir, *tmp;
    struct inet_frags *f;
// Atomically snapshot the list of fqdirs to free
    kill_list = llist_del_all(&fqdir_free_list);
// We need to make sure all ongoing call_rcu(..., inet_frag_destroy_rcu)
// have completed, since they need to dereference fqdir.
// Would it not be nice to have kfree_rcu_barrier() ? :)
//
    rcu_barrier();
    llist_for_each_entry_safe(fqdir, tmp, kill_list, free_list) {
    f = fqdir.f;
    if (refcount_dec_and_test(&f.refcnt))
    complete(&f.completion);
    kfree(fqdir);
    }
    }
    static DECLARE_DELAYED_WORK(fqdir_free_work, fqdir_free_fn);
#[no_mangle]
unsafe extern "C" fn fqdir_work_fn(work: *mut work_struct) {
    static void fqdir_work_fn(struct work_struct *work)
    {
    struct fqdir *fqdir = container_of(work, struct fqdir, destroy_work);
    rhashtable_free_and_destroy(&fqdir.rhashtable, inet_frags_free_cb, core::ptr::null_mut());
    if (llist_add(&fqdir.free_list, &fqdir_free_list))
    queue_delayed_work(system_percpu_wq, &fqdir_free_work, HZ);
    }
#[no_mangle]
pub unsafe extern "C" fn fqdir_init(fqdirp: *mut fqdir, f: *mut inet_frags, net: *mut net) -> c_int {
    int fqdir_init(struct fqdir **fqdirp, struct inet_frags *f, struct net *net)
    {
    struct fqdir *fqdir = kzalloc_obj(*fqdir);
    int res;
    if (!fqdir)
    return -ENOMEM;
    fqdir.f = f;
    fqdir.net = net;
    res = rhashtable_init(&fqdir.rhashtable, &fqdir.f.rhash_params);
    if (res < 0) {
    kfree(fqdir);
    return res;
    }
    refcount_inc(&f.refcnt);
// fqdirp = fqdir;
    return 0;
    }
    EXPORT_SYMBOL(fqdir_init);
    static struct workqueue_struct *inet_frag_wq;
#[no_mangle]
unsafe extern "C" fn inet_frag_wq_init() -> int __init {
    static int __init inet_frag_wq_init(void)
    {
    inet_frag_wq = create_workqueue("inet_frag_wq");
    if (!inet_frag_wq)
    panic("Could not create inet frag workq");
    return 0;
    }
    pure_initcall(inet_frag_wq_init);
#[no_mangle]
pub unsafe extern "C" fn fqdir_pre_exit(fqdir: *mut fqdir) {
    void fqdir_pre_exit(struct fqdir *fqdir)
    {
    struct inet_frag_queue *fq;
    struct rhashtable_iter hti;
// Prevent creation of new frags.
// Pairs with READ_ONCE() in inet_frag_find().
//
    WRITE_ONCE(fqdir.high_thresh, 0);
// Pairs with READ_ONCE() in inet_frag_kill(), ip_expire()
// and ip6frag_expire_frag_queue().
//
    WRITE_ONCE(fqdir.dead, true);
    rhashtable_walk_enter(&fqdir.rhashtable, &hti);
    rhashtable_walk_start(&hti);
    while ((fq = rhashtable_walk_next(&hti))) {
    if (IS_ERR(fq)) {
    if (PTR_ERR(fq) != -EAGAIN)
    break;
    continue;
    }
    spin_lock_bh(&fq.lock);
    if (!(fq.flags & INET_FRAG_COMPLETE))
    inet_frag_queue_flush(fq, 0);
    spin_unlock_bh(&fq.lock);
    }
    rhashtable_walk_stop(&hti);
    rhashtable_walk_exit(&hti);
    }
    EXPORT_SYMBOL(fqdir_pre_exit);
#[no_mangle]
pub unsafe extern "C" fn fqdir_exit(fqdir: *mut fqdir) {
    void fqdir_exit(struct fqdir *fqdir)
    {
    INIT_WORK(&fqdir.destroy_work, fqdir_work_fn);
    queue_work(inet_frag_wq, &fqdir.destroy_work);
    }
    EXPORT_SYMBOL(fqdir_exit);
#[no_mangle]
pub unsafe extern "C" fn inet_frag_kill(fq: *mut inet_frag_queue, refs: *mut c_int) {
    void inet_frag_kill(struct inet_frag_queue *fq, int *refs)
    {
    if (timer_delete(&fq.timer))
    (*refs)++;
    if (!(fq.flags & INET_FRAG_COMPLETE)) {
    struct fqdir *fqdir = fq.fqdir;
    fq.flags |= INET_FRAG_COMPLETE;
    rcu_read_lock();
// The RCU read lock provides a memory barrier
// guaranteeing that if fqdir->dead is false then
// the hash table destruction will not start until
// after we unlock.  Paired with fqdir_pre_exit().
//
    if (!READ_ONCE(fqdir.dead)) {
    rhashtable_remove_fast(&fqdir.rhashtable, &fq.node,
    fqdir.f.rhash_params);
    (*refs)++;
    } else {
    fq.flags |= INET_FRAG_HASH_DEAD;
    }
    rcu_read_unlock();
    }
    }
    EXPORT_SYMBOL(inet_frag_kill);
#[no_mangle]
unsafe extern "C" fn inet_frag_destroy_rcu(head: *mut rcu_head) {
    static void inet_frag_destroy_rcu(struct rcu_head *head)
    {
    struct inet_frag_queue *q = container_of(head, struct inet_frag_queue,
    rcu);
    struct inet_frags *f = q.fqdir.f;
    if (f.destructor)
    f.destructor(q);
    kmem_cache_free(f.frags_cachep, q);
    }
    static unsigned int
    inet_frag_rbtree_purge(struct rb_root *root, enum skb_drop_reason reason)
    {
    struct rb_node *p = rb_first(root);
    let mut sum: c_uint = 0;
    while (p) {
    struct sk_buff *skb = rb_entry(p, struct sk_buff, rbnode);
    p = rb_next(p);
    rb_erase(&skb.rbnode, root);
    while (skb) {
    struct sk_buff *next = FRAG_CB(skb).next_frag;
    sum += skb.truesize;
    kfree_skb_reason(skb, reason);
    skb = next;
    }
    }
    return sum;
    }
    void inet_frag_queue_flush(struct inet_frag_queue *q,
    enum skb_drop_reason reason)
    {
    unsigned int sum;
    reason = reason ?: SKB_DROP_REASON_FRAG_REASM_TIMEOUT;
    sum = inet_frag_rbtree_purge(&q.rb_fragments, reason);
    sub_frag_mem_limit(q.fqdir, sum);
    q.rb_fragments = RB_ROOT;
    q.fragments_tail = core::ptr::null_mut();
    q.last_run_head = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(inet_frag_queue_flush);
#[no_mangle]
pub unsafe extern "C" fn inet_frag_destroy(q: *mut inet_frag_queue) {
    void inet_frag_destroy(struct inet_frag_queue *q)
    {
    unsigned int sum, sum_truesize = 0;
    enum skb_drop_reason reason;
    struct inet_frags *f;
    struct fqdir *fqdir;
    WARN_ON(!(q.flags & INET_FRAG_COMPLETE));
    reason = (q.flags & INET_FRAG_DROP) ?
    SKB_DROP_REASON_FRAG_REASM_TIMEOUT :
    SKB_CONSUMED;
    WARN_ON(timer_delete(&q.timer) != 0);
// Release all fragment data.
    fqdir = q.fqdir;
    f = fqdir.f;
    sum_truesize = inet_frag_rbtree_purge(&q.rb_fragments, reason);
    sum = sum_truesize + f.qsize;
    call_rcu(&q.rcu, inet_frag_destroy_rcu);
    sub_frag_mem_limit(fqdir, sum);
    }
    EXPORT_SYMBOL(inet_frag_destroy);
    static struct inet_frag_queue *inet_frag_alloc(struct fqdir *fqdir,
    struct inet_frags *f,
    void *arg)
    {
    struct inet_frag_queue *q;
    q = kmem_cache_zalloc(f.frags_cachep, GFP_ATOMIC);
    if (!q)
    return core::ptr::null_mut();
    q.fqdir = fqdir;
    f.constructor(q, arg);
    add_frag_mem_limit(fqdir, f.qsize);
    timer_setup(&q.timer, f.frag_expire, 0);
    spin_lock_init(&q.lock);
// One reference for the timer, one for the hash table.
// We never take any extra references, only decrement this field.
//
    refcount_set(&q.refcnt, 2);
    return q;
    }
    static struct inet_frag_queue *inet_frag_create(struct fqdir *fqdir,
    void *arg,
    struct inet_frag_queue **prev)
    {
    struct inet_frags *f = fqdir.f;
    struct inet_frag_queue *q;
    q = inet_frag_alloc(fqdir, f, arg);
    if (!q) {
// prev = ERR_PTR(-ENOMEM);
    return core::ptr::null_mut();
    }
    spin_lock_bh(&q.lock);
// prev = rhashtable_lookup_get_insert_key(&fqdir->rhashtable, &q->key,
    &q.node, f.rhash_params);
    if (*prev) {
// We could not insert in the hash table,
// we need to cancel what inet_frag_alloc()
// anticipated.
//
    q.flags |= INET_FRAG_COMPLETE;
    spin_unlock_bh(&q.lock);
    inet_frag_putn(q, 2);
    return core::ptr::null_mut();
    }
    mod_timer(&q.timer, jiffies + fqdir.timeout);
    spin_unlock_bh(&q.lock);
    return q;
    }
    struct inet_frag_queue *inet_frag_find(struct fqdir *fqdir, void *key)
    {
// This pairs with WRITE_ONCE() in fqdir_pre_exit().
    let mut high_thresh: c_long = READ_ONCE(fqdir.high_thresh);
    struct inet_frag_queue *fq = core::ptr::null_mut(), *prev;
    if (!high_thresh || frag_mem_limit(fqdir) > high_thresh)
    return core::ptr::null_mut();
    prev = rhashtable_lookup(&fqdir.rhashtable, key, fqdir.f.rhash_params);
    if (!prev)
    fq = inet_frag_create(fqdir, key, &prev);
    if (!IS_ERR_OR_NULL(prev))
    fq = prev;
    return fq;
    }
    EXPORT_SYMBOL(inet_frag_find);
    int inet_frag_queue_insert(struct inet_frag_queue *q, struct sk_buff *skb,
    int offset, int end)
    {
    struct sk_buff *last = q.fragments_tail;
// An IP fragment is never a GSO packet, but an untrusted source
// (virtio_net_hdr) may have attached GSO metadata to it. Do not let
// that reach the reassembled skb, whose head keeps the first
// fragment's shinfo and whose frag_list is not GRO-shaped.
//
    skb_gso_reset(skb);
// RFC5722, Section 4, amended by Errata ID : 3089
// When reassembling an IPv6 datagram, if
// one or more its constituent fragments is determined to be an
// overlapping fragment, the entire datagram (and any constituent
// fragments) MUST be silently discarded.
//
// Duplicates, however, should be ignored (i.e. skb dropped, but the
// queue/fragments kept for later reassembly).
//
    if (!last)
    fragrun_create(q, skb);  /* First fragment. */
#[no_mangle]
pub unsafe extern "C" fn if(end: FRAG_CB(last)->ip_defrag_offset + last->len <) -> else {
// This is the common case: skb goes to the end.
// Detect and discard overlaps.
    if (offset < FRAG_CB(last).ip_defrag_offset + last.len)
    return IPFRAG_OVERLAP;
    if (offset == FRAG_CB(last).ip_defrag_offset + last.len)
    fragrun_append_to_last(q, skb);
    else
    fragrun_create(q, skb);
    } else {
// Binary search. Note that skb can become the first fragment,
// but not the last (covered above).
//
    struct rb_node **rbn, *parent;
    rbn = &q.rb_fragments.rb_node;
    do {
    struct sk_buff *curr;
    int curr_run_end;
    parent = *rbn;
    curr = rb_to_skb(parent);
    curr_run_end = FRAG_CB(curr).ip_defrag_offset +
    FRAG_CB(curr).frag_run_len;
    if (end <= FRAG_CB(curr).ip_defrag_offset)
    rbn = &parent.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(curr_run_end: offset >=) -> else {
    else if (offset >= curr_run_end)
    rbn = &parent.rb_right;
    else if (offset >= FRAG_CB(curr).ip_defrag_offset &&
    end <= curr_run_end)
    return IPFRAG_DUP;
    else
    return IPFRAG_OVERLAP;
    } while (*rbn);
// Here we have parent properly set, and rbn pointing to
// one of its NULL left/right children. Insert skb.
//
    fragcb_clear(skb);
    rb_link_node(&skb.rbnode, parent, rbn);
    rb_insert_color(&skb.rbnode, &q.rb_fragments);
    }
    FRAG_CB(skb).ip_defrag_offset = offset;
    if (offset)
    nf_reset_ct(skb);
    return IPFRAG_OK;
    }
    EXPORT_SYMBOL(inet_frag_queue_insert);
    void *inet_frag_reasm_prepare(struct inet_frag_queue *q, struct sk_buff *skb,
    struct sk_buff *parent)
    {
    struct sk_buff *fp, *head = skb_rb_first(&q.rb_fragments);
    void (*destructor)(struct sk_buff *);
    let mut orig_truesize: c_uint = 0;
    struct sk_buff **nextp = core::ptr::null_mut();
    struct sock *sk = skb.sk;
    int delta;
    if (sk && is_skb_wmem(skb)) {
// TX: skb->sk might have been passed as argument to
// dst->output and must remain valid until tx completes.
//
// Move sk to reassembled skb and fix up wmem accounting.
//
    orig_truesize = skb.truesize;
    destructor = skb.destructor;
    }
    if (head != skb) {
    fp = skb_clone(skb, GFP_ATOMIC);
    if (!fp) {
    head = skb;
    goto out_restore_sk;
    }
    if (RB_EMPTY_NODE(&skb.rbnode))
    FRAG_CB(parent).next_frag = fp;
    else
    rb_replace_node(&skb.rbnode, &fp.rbnode,
    &q.rb_fragments);
    if (q.fragments_tail == skb)
    q.fragments_tail = fp;
    if (orig_truesize) {
// prevent skb_morph from releasing sk
    skb.sk = core::ptr::null_mut();
    skb.destructor = core::ptr::null_mut();
    }
    skb_morph(skb, head);
    rb_replace_node(&head.rbnode, &skb.rbnode,
    &q.rb_fragments);
    consume_skb(head);
    head = skb;
    }
    WARN_ON(FRAG_CB(head).ip_defrag_offset != 0);
    delta = -head.truesize;
// Head of list must not be cloned.
    if (skb_unclone(head, GFP_ATOMIC))
    goto out_restore_sk;
    delta += head.truesize;
    if (delta)
    add_frag_mem_limit(q.fqdir, delta);
// If the first fragment is fragmented itself, we split
// it to two chunks: the first with data and paged part
// and the second, holding only fragments.
//
    if (skb_has_frag_list(head)) {
    struct sk_buff *clone;
    int i, plen = 0;
    clone = alloc_skb(0, GFP_ATOMIC);
    if (!clone)
    goto out_restore_sk;
    skb_shinfo(clone).frag_list = skb_shinfo(head).frag_list;
    skb_frag_list_init(head);
    for (i = 0; i < skb_shinfo(head).nr_frags; i++)
    plen += skb_frag_size(&skb_shinfo(head).frags[i]);
    clone.data_len = head.data_len - plen;
    clone.len = clone.data_len;
    head.truesize += clone.truesize;
    clone.csum = 0;
    clone.ip_summed = head.ip_summed;
    add_frag_mem_limit(q.fqdir, clone.truesize);
    skb_shinfo(head).frag_list = clone;
    nextp = &clone.next;
    } else {
    nextp = &skb_shinfo(head).frag_list;
    }
    out_restore_sk:
    if (orig_truesize) {
    let mut ts_delta: c_int = head.truesize - orig_truesize;
// if this reassembled skb is fragmented later,
// fraglist skbs will get skb->sk assigned from head->sk,
// and each frag skb will be released via sock_wfree.
//
// Update sk_wmem_alloc.
//
    head.sk = sk;
    head.destructor = destructor;
    refcount_add(ts_delta, &sk.sk_wmem_alloc);
    }
    return nextp;
    }
    EXPORT_SYMBOL(inet_frag_reasm_prepare);
    void inet_frag_reasm_finish(struct inet_frag_queue *q, struct sk_buff *head,
    void *reasm_data, bool try_coalesce)
    {
    struct sock *sk = is_skb_wmem(head) ? head.sk : core::ptr::null_mut();
    let mut head_truesize: c_uint = head.truesize;
    struct sk_buff **nextp = reasm_data;
    struct rb_node *rbn;
    struct sk_buff *fp;
    int sum_truesize;
    skb_push(head, head.data - skb_network_header(head));
// Traverse the tree in order, to build frag_list.
    fp = FRAG_CB(head).next_frag;
    rbn = rb_next(&head.rbnode);
    rb_erase(&head.rbnode, &q.rb_fragments);
    sum_truesize = head.truesize;
    while (rbn || fp) {
// fp points to the next sk_buff in the current run;
// rbn points to the next run.
//
// Go through the current run.
    while (fp) {
    struct sk_buff *next_frag = FRAG_CB(fp).next_frag;
    bool stolen;
    int delta;
    sum_truesize += fp.truesize;
    if (head.ip_summed != fp.ip_summed)
    head.ip_summed = CHECKSUM_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(CHECKSUM_COMPLETE: head->ip_summed ==) -> else {
    else if (head.ip_summed == CHECKSUM_COMPLETE)
    head.csum = csum_add(head.csum, fp.csum);
    if (try_coalesce && skb_try_coalesce(head, fp, &stolen,
    &delta)) {
    kfree_skb_partial(fp, stolen);
    } else {
    fp.prev = core::ptr::null_mut();
    memset(&fp.rbnode, 0, sizeof(fp.rbnode));
    fp.sk = core::ptr::null_mut();
    head.data_len += fp.len;
    head.len += fp.len;
    head.truesize += fp.truesize;
// nextp = fp;
    nextp = &fp.next;
    }
    fp = next_frag;
    }
// Move to the next run.
    if (rbn) {
    struct rb_node *rbnext = rb_next(rbn);
    fp = rb_to_skb(rbn);
    rb_erase(rbn, &q.rb_fragments);
    rbn = rbnext;
    }
    }
    sub_frag_mem_limit(q.fqdir, sum_truesize);
// nextp = NULL;
    skb_mark_not_on_list(head);
    head.prev = core::ptr::null_mut();
    head.tstamp = q.stamp;
    head.tstamp_type = q.tstamp_type;
    if (sk)
    refcount_add(sum_truesize - head_truesize, &sk.sk_wmem_alloc);
    }
    EXPORT_SYMBOL(inet_frag_reasm_finish);
    struct sk_buff *inet_frag_pull_head(struct inet_frag_queue *q)
    {
    struct sk_buff *head, *skb;
    head = skb_rb_first(&q.rb_fragments);
    if (!head)
    return core::ptr::null_mut();
    skb = FRAG_CB(head).next_frag;
    if (skb)
    rb_replace_node(&head.rbnode, &skb.rbnode,
    &q.rb_fragments);
    else
    rb_erase(&head.rbnode, &q.rb_fragments);
    memset(&head.rbnode, 0, sizeof(head.rbnode));
    barrier();
    if (head == q.fragments_tail)
    q.fragments_tail = core::ptr::null_mut();
    sub_frag_mem_limit(q.fqdir, head.truesize);
    return head;
    }
    EXPORT_SYMBOL(inet_frag_pull_head);
