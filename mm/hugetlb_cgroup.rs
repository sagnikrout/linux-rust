//! Automatically rewritten from C to Rust
//! Source: mm/hugetlb_cgroup.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright IBM Corporation, 2012
// Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
//
// Cgroup v2
// Copyright (C) 2019 Red Hat, Inc.
// Author: Giuseppe Scrivano <gscrivan@redhat.com>
//

// Use t->m[0] to encode the offset

    static struct hugetlb_cgroup *root_h_cgroup __read_mostly;
    static struct cftype *dfl_files;
    static struct cftype *legacy_files;
    static inline struct page_counter *
    __hugetlb_cgroup_counter_from_cgroup(struct hugetlb_cgroup *h_cg, int idx,
    bool rsvd)
    {
    if (rsvd)
    return &h_cg.rsvd_hugepage[idx];
    return &h_cg.hugepage[idx];
    }
    static inline struct page_counter *
    hugetlb_cgroup_counter_from_cgroup(struct hugetlb_cgroup *h_cg, int idx)
    {
    return __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, false);
    }
    static inline struct page_counter *
    hugetlb_cgroup_counter_from_cgroup_rsvd(struct hugetlb_cgroup *h_cg, int idx)
    {
    return __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, true);
    }
    static inline
    struct hugetlb_cgroup *hugetlb_cgroup_from_css(struct cgroup_subsys_state *s)
    {
    return s ? container_of(s, struct hugetlb_cgroup, css) : core::ptr::null_mut();
    }
    static inline
    struct hugetlb_cgroup *hugetlb_cgroup_from_task(struct task_struct *task)
    {
    return hugetlb_cgroup_from_css(task_css(task, hugetlb_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_is_root(h_cg: *mut hugetlb_cgroup) -> bool {
    static inline bool hugetlb_cgroup_is_root(struct hugetlb_cgroup *h_cg)
    {
    return (h_cg == root_h_cgroup);
    }
    static inline struct hugetlb_cgroup *
    parent_hugetlb_cgroup(struct hugetlb_cgroup *h_cg)
    {
    return hugetlb_cgroup_from_css(h_cg.css.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_have_usage(h_cg: *mut hugetlb_cgroup) -> bool {
    static inline bool hugetlb_cgroup_have_usage(struct hugetlb_cgroup *h_cg)
    {
    struct hstate *h;
    for_each_hstate(h) {
    if (page_counter_read(
    hugetlb_cgroup_counter_from_cgroup(h_cg, hstate_index(h))))
    return true;
    }
    return false;
    }
    static void hugetlb_cgroup_init(struct hugetlb_cgroup *h_cgroup,
    struct hugetlb_cgroup *parent_h_cgroup)
    {
    int idx;
    for (idx = 0; idx < HUGE_MAX_HSTATE; idx++) {
    struct page_counter *fault, *fault_parent = core::ptr::null_mut();
    struct page_counter *rsvd, *rsvd_parent = core::ptr::null_mut();
    unsigned long limit;
    int ret;
    if (parent_h_cgroup) {
    fault_parent = hugetlb_cgroup_counter_from_cgroup(
    parent_h_cgroup, idx);
    rsvd_parent = hugetlb_cgroup_counter_from_cgroup_rsvd(
    parent_h_cgroup, idx);
    }
    fault = hugetlb_cgroup_counter_from_cgroup(h_cgroup, idx);
    rsvd = hugetlb_cgroup_counter_from_cgroup_rsvd(h_cgroup, idx);
    page_counter_init(fault, fault_parent, false);
    page_counter_init(rsvd, rsvd_parent, false);
    if (!cgroup_subsys_on_dfl(hugetlb_cgrp_subsys)) {
    fault.track_failcnt = true;
    rsvd.track_failcnt = true;
    }
    limit = round_down(PAGE_COUNTER_MAX,
    pages_per_huge_page(&hstates[idx]));
    ret = page_counter_set_max(fault, limit);
    VM_WARN_ON_ONCE(ret);
    ret = page_counter_set_max(rsvd, limit);
    VM_WARN_ON_ONCE(ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_free(h_cgroup: *mut hugetlb_cgroup) {
    static void hugetlb_cgroup_free(struct hugetlb_cgroup *h_cgroup)
    {
    int node;
    for_each_node(node)
    kfree(h_cgroup.nodeinfo[node]);
    kfree(h_cgroup);
    }
    static struct cgroup_subsys_state *
    hugetlb_cgroup_css_alloc(struct cgroup_subsys_state *parent_css)
    {
    struct hugetlb_cgroup *parent_h_cgroup = hugetlb_cgroup_from_css(parent_css);
    struct hugetlb_cgroup *h_cgroup;
    int node;
    h_cgroup = kzalloc_flex(*h_cgroup, nodeinfo, nr_node_ids);
    if (!h_cgroup)
    return ERR_PTR(-ENOMEM);
    if (!parent_h_cgroup)
    root_h_cgroup = h_cgroup;
//
// TODO: this routine can waste much memory for nodes which will
// never be onlined. It's better to use memory hotplug callback
// function.
//
    for_each_node(node) {
// Set node_to_alloc to NUMA_NO_NODE for offline nodes.
    int node_to_alloc =
    node_state(node, N_NORMAL_MEMORY) ? node : NUMA_NO_NODE;
    h_cgroup.nodeinfo[node] =
    kzalloc_node(sizeof(struct hugetlb_cgroup_per_node),
    GFP_KERNEL, node_to_alloc);
    if (!h_cgroup.nodeinfo[node])
    goto fail_alloc_nodeinfo;
    }
    hugetlb_cgroup_init(h_cgroup, parent_h_cgroup);
    return &h_cgroup.css;
    fail_alloc_nodeinfo:
    hugetlb_cgroup_free(h_cgroup);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_css_free(css: *mut cgroup_subsys_state) {
    static void hugetlb_cgroup_css_free(struct cgroup_subsys_state *css)
    {
    hugetlb_cgroup_free(hugetlb_cgroup_from_css(css));
    }
//
// Should be called with hugetlb_lock held.
// Since we are holding hugetlb_lock, pages cannot get moved from
// active list or uncharged from the cgroup, So no need to get
// page reference and test for page active here. This function
// cannot fail.
//
    static void hugetlb_cgroup_move_parent(int idx, struct hugetlb_cgroup *h_cg,
    struct folio *folio)
    {
    unsigned int nr_pages;
    struct page_counter *counter;
    struct hugetlb_cgroup *hcg;
    struct hugetlb_cgroup *parent = parent_hugetlb_cgroup(h_cg);
    hcg = hugetlb_cgroup_from_folio(folio);
//
// We can have pages in active list without any cgroup
// ie, hugepage with less than 3 pages. We can safely
// ignore those pages.
//
    if (!hcg || hcg != h_cg)
    goto out;
    nr_pages = folio_nr_pages(folio);
    if (!parent) {
    parent = root_h_cgroup;
// root has no limit
    page_counter_charge(&parent.hugepage[idx], nr_pages);
    }
    counter = &h_cg.hugepage[idx];
// Take the pages off the local counter
    page_counter_cancel(counter, nr_pages);
    set_hugetlb_cgroup(folio, parent);
    out:
    return;
    }
//
// Force the hugetlb cgroup to empty the hugetlb resources by moving them to
// the parent cgroup.
//
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_css_offline(css: *mut cgroup_subsys_state) {
    static void hugetlb_cgroup_css_offline(struct cgroup_subsys_state *css)
    {
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(css);
    struct hstate *h;
    struct folio *folio;
    do {
    for_each_hstate(h) {
    spin_lock_irq(&hugetlb_lock);
    list_for_each_entry(folio, &h.hugepage_activelist, lru)
    hugetlb_cgroup_move_parent(hstate_index(h), h_cg, folio);
    spin_unlock_irq(&hugetlb_lock);
    }
    cond_resched();
    } while (hugetlb_cgroup_have_usage(h_cg));
    }
    static inline void hugetlb_event(struct hugetlb_cgroup *hugetlb, int idx,
    enum hugetlb_memory_event event)
    {
    atomic_long_inc(&hugetlb.events_local[idx][event]);
    cgroup_file_notify(&hugetlb.events_local_file[idx]);
    do {
    atomic_long_inc(&hugetlb.events[idx][event]);
    cgroup_file_notify(&hugetlb.events_file[idx]);
    } while ((hugetlb = parent_hugetlb_cgroup(hugetlb)) &&
    !hugetlb_cgroup_is_root(hugetlb));
    }
    static int __hugetlb_cgroup_charge_cgroup(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup **ptr,
    bool rsvd)
    {
    let mut ret: c_int = 0;
    struct page_counter *counter;
    struct hugetlb_cgroup *h_cg = core::ptr::null_mut();
    if (hugetlb_cgroup_disabled())
    goto done;
    again:
    rcu_read_lock();
    h_cg = hugetlb_cgroup_from_task(current);
    if (!css_tryget(&h_cg.css)) {
    rcu_read_unlock();
    goto again;
    }
    rcu_read_unlock();
    if (!page_counter_try_charge(
    __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, rsvd),
    nr_pages, &counter)) {
    ret = -ENOMEM;
    hugetlb_event(h_cg, idx, HUGETLB_MAX);
    css_put(&h_cg.css);
    goto done;
    }
// Reservations take a reference to the css because they do not get
// reparented.
//
    if (!rsvd)
    css_put(&h_cg.css);
    done:
// ptr = h_cg;
    return ret;
    }
    int hugetlb_cgroup_charge_cgroup(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup **ptr)
    {
    return __hugetlb_cgroup_charge_cgroup(idx, nr_pages, ptr, false);
    }
    int hugetlb_cgroup_charge_cgroup_rsvd(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup **ptr)
    {
    return __hugetlb_cgroup_charge_cgroup(idx, nr_pages, ptr, true);
    }
// Should be called with hugetlb_lock held
    static void __hugetlb_cgroup_commit_charge(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg,
    struct folio *folio, bool rsvd)
    {
    if (hugetlb_cgroup_disabled() || !h_cg)
    return;
    lockdep_assert_held(&hugetlb_lock);
    __set_hugetlb_cgroup(folio, h_cg, rsvd);
    if (!rsvd) {
    unsigned long usage =
    h_cg.nodeinfo[folio_nid(folio)].usage[idx];
//
// This write is not atomic due to fetching usage and writing
// to it, but that's fine because we call this with
// hugetlb_lock held anyway.
//
    WRITE_ONCE(h_cg.nodeinfo[folio_nid(folio)].usage[idx],
    usage + nr_pages);
    }
    }
    void hugetlb_cgroup_commit_charge(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg,
    struct folio *folio)
    {
    __hugetlb_cgroup_commit_charge(idx, nr_pages, h_cg, folio, false);
    }
    void hugetlb_cgroup_commit_charge_rsvd(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg,
    struct folio *folio)
    {
    __hugetlb_cgroup_commit_charge(idx, nr_pages, h_cg, folio, true);
    }
//
// Should be called with hugetlb_lock held
//
    static void __hugetlb_cgroup_uncharge_folio(int idx, unsigned long nr_pages,
    struct folio *folio, bool rsvd)
    {
    struct hugetlb_cgroup *h_cg;
    if (hugetlb_cgroup_disabled())
    return;
    lockdep_assert_held(&hugetlb_lock);
    h_cg = __hugetlb_cgroup_from_folio(folio, rsvd);
    if (unlikely(!h_cg))
    return;
    __set_hugetlb_cgroup(folio, core::ptr::null_mut(), rsvd);
    page_counter_uncharge(__hugetlb_cgroup_counter_from_cgroup(h_cg, idx,
    rsvd),
    nr_pages);
    if (rsvd)
    css_put(&h_cg.css);
    else {
    unsigned long usage =
    h_cg.nodeinfo[folio_nid(folio)].usage[idx];
//
// This write is not atomic due to fetching usage and writing
// to it, but that's fine because we call this with
// hugetlb_lock held anyway.
//
    WRITE_ONCE(h_cg.nodeinfo[folio_nid(folio)].usage[idx],
    usage - nr_pages);
    }
    }
    void hugetlb_cgroup_uncharge_folio(int idx, unsigned long nr_pages,
    struct folio *folio)
    {
    __hugetlb_cgroup_uncharge_folio(idx, nr_pages, folio, false);
    }
    void hugetlb_cgroup_uncharge_folio_rsvd(int idx, unsigned long nr_pages,
    struct folio *folio)
    {
    __hugetlb_cgroup_uncharge_folio(idx, nr_pages, folio, true);
    }
    static void __hugetlb_cgroup_uncharge_cgroup(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg,
    bool rsvd)
    {
    if (hugetlb_cgroup_disabled() || !h_cg)
    return;
    page_counter_uncharge(__hugetlb_cgroup_counter_from_cgroup(h_cg, idx,
    rsvd),
    nr_pages);
    if (rsvd)
    css_put(&h_cg.css);
    }
    void hugetlb_cgroup_uncharge_cgroup(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg)
    {
    __hugetlb_cgroup_uncharge_cgroup(idx, nr_pages, h_cg, false);
    }
    void hugetlb_cgroup_uncharge_cgroup_rsvd(int idx, unsigned long nr_pages,
    struct hugetlb_cgroup *h_cg)
    {
    __hugetlb_cgroup_uncharge_cgroup(idx, nr_pages, h_cg, true);
    }
    void hugetlb_cgroup_uncharge_counter(struct resv_map *resv, unsigned long start,
    unsigned long end)
    {
    if (hugetlb_cgroup_disabled() || !resv || !resv.reservation_counter ||
    !resv.css)
    return;
    page_counter_uncharge(resv.reservation_counter,
    (end - start) * resv.pages_per_hpage);
    css_put(resv.css);
    }
    void hugetlb_cgroup_uncharge_file_region(struct resv_map *resv,
    struct file_region *rg,
    unsigned long nr_pages,
    bool region_del)
    {
    if (hugetlb_cgroup_disabled() || !resv || !rg || !nr_pages)
    return;
    if (rg.reservation_counter && resv.pages_per_hpage &&
    !resv.reservation_counter) {
    page_counter_uncharge(rg.reservation_counter,
    nr_pages * resv.pages_per_hpage);
//
// Only do css_put(rg->css) when we delete the entire region
// because one file_region must hold exactly one css reference.
//
    if (region_del)
    css_put(rg.css);
    }
    }
    enum {
    RES_USAGE,
    RES_RSVD_USAGE,
    RES_LIMIT,
    RES_RSVD_LIMIT,
    RES_MAX_USAGE,
    RES_RSVD_MAX_USAGE,
    RES_FAILCNT,
    RES_RSVD_FAILCNT,
    };
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_read_numa_stat(seq: *mut seq_file, dummy: *mut c_void) -> c_int {
    static int hugetlb_cgroup_read_numa_stat(struct seq_file *seq, void *dummy)
    {
    int nid;
    struct cftype *cft = seq_cft(seq);
    let mut idx: c_int = MEMFILE_IDX(cft.private);
    let mut legacy: bool = !cgroup_subsys_on_dfl(hugetlb_cgrp_subsys);
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(seq_css(seq));
    struct cgroup_subsys_state *css;
    unsigned long usage;
    if (legacy) {
// Add up usage across all nodes for the non-hierarchical total.
    usage = 0;
    for_each_node_state(nid, N_MEMORY)
    usage += READ_ONCE(h_cg.nodeinfo[nid].usage[idx]);
    seq_printf(seq, "total=%lu", usage * PAGE_SIZE);
// Simply print the per-node usage for the non-hierarchical total.
    for_each_node_state(nid, N_MEMORY)
    seq_printf(seq, " N%d=%lu", nid,
    READ_ONCE(h_cg.nodeinfo[nid].usage[idx]) *
    PAGE_SIZE);
    seq_putc(seq, '\n');
    }
//
// The hierarchical total is pretty much the value recorded by the
// counter, so use that.
//
    seq_printf(seq, "%stotal=%lu", legacy ? "hierarchical_" : "",
    page_counter_read(&h_cg.hugepage[idx]) * PAGE_SIZE);
//
// For each node, transverse the css tree to obtain the hierarchical
// node usage.
//
    for_each_node_state(nid, N_MEMORY) {
    usage = 0;
    rcu_read_lock();
    css_for_each_descendant_pre(css, &h_cg.css) {
    usage += READ_ONCE(hugetlb_cgroup_from_css(css)
    .nodeinfo[nid]
    .usage[idx]);
    }
    rcu_read_unlock();
    seq_printf(seq, " N%d=%lu", nid, usage * PAGE_SIZE);
    }
    seq_putc(seq, '\n');
    return 0;
    }
    static u64 hugetlb_cgroup_read_u64(struct cgroup_subsys_state *css,
    struct cftype *cft)
    {
    struct page_counter *counter;
    struct page_counter *rsvd_counter;
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(css);
    counter = &h_cg.hugepage[MEMFILE_IDX(cft.private)];
    rsvd_counter = &h_cg.rsvd_hugepage[MEMFILE_IDX(cft.private)];
    switch (MEMFILE_ATTR(cft.private)) {
    case RES_USAGE:
    return (u64)page_counter_read(counter) * PAGE_SIZE;
    case RES_RSVD_USAGE:
    return (u64)page_counter_read(rsvd_counter) * PAGE_SIZE;
    case RES_LIMIT:
    return (u64)counter.max * PAGE_SIZE;
    case RES_RSVD_LIMIT:
    return (u64)rsvd_counter.max * PAGE_SIZE;
    case RES_MAX_USAGE:
    return (u64)counter.watermark * PAGE_SIZE;
    case RES_RSVD_MAX_USAGE:
    return (u64)rsvd_counter.watermark * PAGE_SIZE;
    case RES_FAILCNT:
    return counter.failcnt;
    case RES_RSVD_FAILCNT:
    return rsvd_counter.failcnt;
    default:
    BUG();
    }
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_cgroup_read_u64_max(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int hugetlb_cgroup_read_u64_max(struct seq_file *seq, void *v)
    {
    int idx;
    u64 val;
    struct cftype *cft = seq_cft(seq);
    unsigned long limit;
    struct page_counter *counter;
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(seq_css(seq));
    idx = MEMFILE_IDX(cft.private);
    counter = &h_cg.hugepage[idx];
    limit = round_down(PAGE_COUNTER_MAX,
    pages_per_huge_page(&hstates[idx]));
    switch (MEMFILE_ATTR(cft.private)) {
    case RES_RSVD_USAGE:
    counter = &h_cg.rsvd_hugepage[idx];
    fallthrough;
    case RES_USAGE:
    val = (u64)page_counter_read(counter);
    seq_printf(seq, "%llu\n", val * PAGE_SIZE);
    break;
    case RES_RSVD_LIMIT:
    counter = &h_cg.rsvd_hugepage[idx];
    fallthrough;
    case RES_LIMIT:
    val = (u64)counter.max;
    if (val == limit)
    seq_puts(seq, "max\n");
    else
    seq_printf(seq, "%llu\n", val * PAGE_SIZE);
    break;
    default:
    BUG();
    }
    return 0;
    }
    static DEFINE_MUTEX(hugetlb_limit_mutex);
    static ssize_t hugetlb_cgroup_write(struct kernfs_open_file *of,
    char *buf, size_t nbytes, loff_t off,
    const char *max)
    {
    int ret, idx;
    unsigned long nr_pages;
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(of_css(of));
    let mut rsvd: bool = false;
    if (hugetlb_cgroup_is_root(h_cg)) /* Can't set limit on root */
    return -EINVAL;
    buf = strstrip(buf);
    ret = page_counter_memparse(buf, max, &nr_pages);
    if (ret)
    return ret;
    idx = MEMFILE_IDX(of_cft(of).private);
    nr_pages = round_down(nr_pages, pages_per_huge_page(&hstates[idx]));
    switch (MEMFILE_ATTR(of_cft(of).private)) {
    case RES_RSVD_LIMIT:
    rsvd = true;
    fallthrough;
    case RES_LIMIT:
    mutex_lock(&hugetlb_limit_mutex);
    ret = page_counter_set_max(
    __hugetlb_cgroup_counter_from_cgroup(h_cg, idx, rsvd),
    nr_pages);
    mutex_unlock(&hugetlb_limit_mutex);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret ?: nbytes;
    }
    static ssize_t hugetlb_cgroup_write_legacy(struct kernfs_open_file *of,
    char *buf, size_t nbytes, loff_t off)
    {
    return hugetlb_cgroup_write(of, buf, nbytes, off, "-1");
    }
    static ssize_t hugetlb_cgroup_write_dfl(struct kernfs_open_file *of,
    char *buf, size_t nbytes, loff_t off)
    {
    return hugetlb_cgroup_write(of, buf, nbytes, off, "max");
    }
    static ssize_t hugetlb_cgroup_reset(struct kernfs_open_file *of,
    char *buf, size_t nbytes, loff_t off)
    {
    let mut ret: c_int = 0;
    struct page_counter *counter, *rsvd_counter;
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(of_css(of));
    counter = &h_cg.hugepage[MEMFILE_IDX(of_cft(of).private)];
    rsvd_counter = &h_cg.rsvd_hugepage[MEMFILE_IDX(of_cft(of).private)];
    switch (MEMFILE_ATTR(of_cft(of).private)) {
    case RES_MAX_USAGE:
    page_counter_reset_watermark(counter);
    break;
    case RES_RSVD_MAX_USAGE:
    page_counter_reset_watermark(rsvd_counter);
    break;
    case RES_FAILCNT:
    counter.failcnt = 0;
    break;
    case RES_RSVD_FAILCNT:
    rsvd_counter.failcnt = 0;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret ?: nbytes;
    }
    static char *mem_fmt(char *buf, int size, unsigned long hsize)
    {
    if (hsize >= SZ_1G)
    snprintf(buf, size, "%luGB", hsize / SZ_1G);
#[no_mangle]
pub unsafe extern "C" fn if(SZ_1M: hsize >=) -> else {
    else if (hsize >= SZ_1M)
    snprintf(buf, size, "%luMB", hsize / SZ_1M);
    else
    snprintf(buf, size, "%luKB", hsize / SZ_1K);
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_events_show(seq: *mut seq_file, local: bool) -> c_int {
    static int __hugetlb_events_show(struct seq_file *seq, bool local)
    {
    int idx;
    long max;
    struct cftype *cft = seq_cft(seq);
    struct hugetlb_cgroup *h_cg = hugetlb_cgroup_from_css(seq_css(seq));
    idx = MEMFILE_IDX(cft.private);
    if (local)
    max = atomic_long_read(&h_cg.events_local[idx][HUGETLB_MAX]);
    else
    max = atomic_long_read(&h_cg.events[idx][HUGETLB_MAX]);
    seq_printf(seq, "max %lu\n", max);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_events_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int hugetlb_events_show(struct seq_file *seq, void *v)
    {
    return __hugetlb_events_show(seq, false);
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_events_local_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int hugetlb_events_local_show(struct seq_file *seq, void *v)
    {
    return __hugetlb_events_show(seq, true);
    }
    static struct cftype hugetlb_dfl_tmpl[] = {
    {
    .name = "max",
    .private = RES_LIMIT,
    .seq_show = hugetlb_cgroup_read_u64_max,
    .write = hugetlb_cgroup_write_dfl,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "rsvd.max",
    .private = RES_RSVD_LIMIT,
    .seq_show = hugetlb_cgroup_read_u64_max,
    .write = hugetlb_cgroup_write_dfl,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "current",
    .private = RES_USAGE,
    .seq_show = hugetlb_cgroup_read_u64_max,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "rsvd.current",
    .private = RES_RSVD_USAGE,
    .seq_show = hugetlb_cgroup_read_u64_max,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "events",
    .seq_show = hugetlb_events_show,
    .file_offset = MEMFILE_OFFSET(struct hugetlb_cgroup, events_file[0]),
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "events.local",
    .seq_show = hugetlb_events_local_show,
    .file_offset = MEMFILE_OFFSET(struct hugetlb_cgroup, events_local_file[0]),
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "numa_stat",
    .seq_show = hugetlb_cgroup_read_numa_stat,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
// don't need terminator here
    };
    static struct cftype hugetlb_legacy_tmpl[] = {
    {
    .name = "limit_in_bytes",
    .private = RES_LIMIT,
    .read_u64 = hugetlb_cgroup_read_u64,
    .write = hugetlb_cgroup_write_legacy,
    },
    {
    .name = "rsvd.limit_in_bytes",
    .private = RES_RSVD_LIMIT,
    .read_u64 = hugetlb_cgroup_read_u64,
    .write = hugetlb_cgroup_write_legacy,
    },
    {
    .name = "usage_in_bytes",
    .private = RES_USAGE,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "rsvd.usage_in_bytes",
    .private = RES_RSVD_USAGE,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "max_usage_in_bytes",
    .private = RES_MAX_USAGE,
    .write = hugetlb_cgroup_reset,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "rsvd.max_usage_in_bytes",
    .private = RES_RSVD_MAX_USAGE,
    .write = hugetlb_cgroup_reset,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "failcnt",
    .private = RES_FAILCNT,
    .write = hugetlb_cgroup_reset,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "rsvd.failcnt",
    .private = RES_RSVD_FAILCNT,
    .write = hugetlb_cgroup_reset,
    .read_u64 = hugetlb_cgroup_read_u64,
    },
    {
    .name = "numa_stat",
    .seq_show = hugetlb_cgroup_read_numa_stat,
    },
// don't need terminator here
    };
    static void __init
    hugetlb_cgroup_cfttypes_init(struct hstate *h, struct cftype *cft,
    struct cftype *tmpl, int tmpl_size)
    {
    char buf[32];
    int i, idx = hstate_index(h);
// format the size
    mem_fmt(buf, sizeof(buf), huge_page_size(h));
    for (i = 0; i < tmpl_size; cft++, tmpl++, i++) {
// cft = *tmpl;
// rebuild the name
    scnprintf(cft.name, MAX_CFTYPE_NAME, "%s.%s", buf, tmpl.name);
// rebuild the private
    cft.private = MEMFILE_PRIVATE(idx, tmpl.private);
// rebuild the file_offset
    if (tmpl.file_offset) {
    let mut offset: c_uint = tmpl.file_offset;
    cft.file_offset = MEMFILE_OFFSET0(offset) +
    MEMFILE_FIELD_SIZE(offset) * idx;
    }
    lockdep_register_key(&cft.lockdep_key);
    }
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_dfl_init(h: *mut hstate) -> void __init {
    static void __init __hugetlb_cgroup_file_dfl_init(struct hstate *h)
    {
    let mut idx: c_int = hstate_index(h);
    hugetlb_cgroup_cfttypes_init(h, dfl_files + idx * DFL_TMPL_SIZE,
    hugetlb_dfl_tmpl, DFL_TMPL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_legacy_init(h: *mut hstate) -> void __init {
    static void __init __hugetlb_cgroup_file_legacy_init(struct hstate *h)
    {
    let mut idx: c_int = hstate_index(h);
    hugetlb_cgroup_cfttypes_init(h, legacy_files + idx * LEGACY_TMPL_SIZE,
    hugetlb_legacy_tmpl, LEGACY_TMPL_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_init(h: *mut hstate) -> void __init {
    static void __init __hugetlb_cgroup_file_init(struct hstate *h)
    {
    __hugetlb_cgroup_file_dfl_init(h);
    __hugetlb_cgroup_file_legacy_init(h);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_pre_init() -> void __init {
    static void __init __hugetlb_cgroup_file_pre_init(void)
    {
    int cft_count;
    cft_count = hugetlb_max_hstate * DFL_TMPL_SIZE + 1; /* add terminator */
    dfl_files = kzalloc_objs(struct cftype, cft_count);
    BUG_ON(!dfl_files);
    cft_count = hugetlb_max_hstate * LEGACY_TMPL_SIZE + 1; /* add terminator */
    legacy_files = kzalloc_objs(struct cftype, cft_count);
    BUG_ON(!legacy_files);
    }
#[no_mangle]
unsafe extern "C" fn __hugetlb_cgroup_file_post_init() -> void __init {
    static void __init __hugetlb_cgroup_file_post_init(void)
    {
    WARN_ON(cgroup_add_dfl_cftypes(&hugetlb_cgrp_subsys,
    dfl_files));
    WARN_ON(cgroup_add_legacy_cftypes(&hugetlb_cgrp_subsys,
    legacy_files));
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_file_init() -> void __init {
    void __init hugetlb_cgroup_file_init(void)
    {
    struct hstate *h;
    __hugetlb_cgroup_file_pre_init();
    for_each_hstate(h)
    __hugetlb_cgroup_file_init(h);
    __hugetlb_cgroup_file_post_init();
    }
//
// hugetlb_lock will make sure a parallel cgroup rmdir won't happen
// when we migrate hugepages
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_cgroup_migrate(old_folio: *mut folio, new_folio: *mut folio) {
    void hugetlb_cgroup_migrate(struct folio *old_folio, struct folio *new_folio)
    {
    struct hugetlb_cgroup *h_cg;
    struct hugetlb_cgroup *h_cg_rsvd;
    struct hstate *h = folio_hstate(old_folio);
    if (hugetlb_cgroup_disabled())
    return;
    spin_lock_irq(&hugetlb_lock);
    h_cg = hugetlb_cgroup_from_folio(old_folio);
    h_cg_rsvd = hugetlb_cgroup_from_folio_rsvd(old_folio);
    set_hugetlb_cgroup(old_folio, core::ptr::null_mut());
    set_hugetlb_cgroup_rsvd(old_folio, core::ptr::null_mut());
// move the h_cg details to new cgroup
    set_hugetlb_cgroup(new_folio, h_cg);
    set_hugetlb_cgroup_rsvd(new_folio, h_cg_rsvd);
    list_move(&new_folio.lru, &h.hugepage_activelist);
    spin_unlock_irq(&hugetlb_lock);
    }
    static struct cftype hugetlb_files[] = {
    {} /* terminate */
    };
    struct cgroup_subsys hugetlb_cgrp_subsys = {
    .css_alloc	= hugetlb_cgroup_css_alloc,
    .css_offline	= hugetlb_cgroup_css_offline,
    .css_free	= hugetlb_cgroup_css_free,
    .dfl_cftypes	= hugetlb_files,
    .legacy_cftypes	= hugetlb_files,
    };
