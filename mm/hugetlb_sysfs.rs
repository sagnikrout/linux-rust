//! Automatically rewritten from C to Rust
//! Source: mm/hugetlb_sysfs.c
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
// HugeTLB sysfs interfaces.
// (C) Nadia Yvette Chambers, April 2004
//

    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)

    static struct kobj_attribute _name##_attr = __ATTR_WO(_name)

    static struct kobj_attribute _name##_attr = __ATTR_RW(_name)
    static struct kobject *hugepages_kobj;
    static struct kobject *hstate_kobjs[HUGE_MAX_HSTATE];
    static struct hstate *kobj_to_node_hstate(struct kobject *kobj, int *nidp);
    static struct hstate *kobj_to_hstate(struct kobject *kobj, int *nidp)
    {
    int i;
    for (i = 0; i < HUGE_MAX_HSTATE; i++)
    if (hstate_kobjs[i] == kobj) {
    if (nidp)
// nidp = NUMA_NO_NODE;
    return &hstates[i];
    }
    return kobj_to_node_hstate(kobj, nidp);
    }
    static ssize_t nr_hugepages_show_common(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h;
    unsigned long nr_huge_pages;
    int nid;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE)
    nr_huge_pages = h.nr_huge_pages;
    else
    nr_huge_pages = h.nr_huge_pages_node[nid];
    return sysfs_emit(buf, "%lu\n", nr_huge_pages);
    }
    static ssize_t nr_hugepages_store_common(bool obey_mempolicy,
    struct kobject *kobj, const char *buf,
    size_t len)
    {
    struct hstate *h;
    unsigned long count;
    int nid;
    int err;
    err = kstrtoul(buf, 10, &count);
    if (err)
    return err;
    h = kobj_to_hstate(kobj, &nid);
    return __nr_hugepages_store_common(obey_mempolicy, h, nid, count, len);
    }
    static ssize_t nr_hugepages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return nr_hugepages_show_common(kobj, attr, buf);
    }
    static ssize_t nr_hugepages_store(struct kobject *kobj,
    struct kobj_attribute *attr, const char *buf, size_t len)
    {
    return nr_hugepages_store_common(false, kobj, buf, len);
    }
    HSTATE_ATTR(nr_hugepages);

//
// hstate attribute for optionally mempolicy-based constraint on persistent
// huge page alloc/free.
//
    static ssize_t nr_hugepages_mempolicy_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *buf)
    {
    return nr_hugepages_show_common(kobj, attr, buf);
    }
    static ssize_t nr_hugepages_mempolicy_store(struct kobject *kobj,
    struct kobj_attribute *attr, const char *buf, size_t len)
    {
    return nr_hugepages_store_common(true, kobj, buf, len);
    }
    HSTATE_ATTR(nr_hugepages_mempolicy);

    static ssize_t nr_overcommit_hugepages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h = kobj_to_hstate(kobj, core::ptr::null_mut());
    return sysfs_emit(buf, "%lu\n", h.nr_overcommit_huge_pages);
    }
    static ssize_t nr_overcommit_hugepages_store(struct kobject *kobj,
    struct kobj_attribute *attr, const char *buf, size_t count)
    {
    int err;
    unsigned long input;
    struct hstate *h = kobj_to_hstate(kobj, core::ptr::null_mut());
    if (hstate_is_gigantic_no_runtime(h))
    return -EINVAL;
    err = kstrtoul(buf, 10, &input);
    if (err)
    return err;
    spin_lock_irq(&hugetlb_lock);
    h.nr_overcommit_huge_pages = input;
    spin_unlock_irq(&hugetlb_lock);
    return count;
    }
    HSTATE_ATTR(nr_overcommit_hugepages);
    static ssize_t free_hugepages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h;
    unsigned long free_huge_pages;
    int nid;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE)
    free_huge_pages = h.free_huge_pages;
    else
    free_huge_pages = h.free_huge_pages_node[nid];
    return sysfs_emit(buf, "%lu\n", free_huge_pages);
    }
    HSTATE_ATTR_RO(free_hugepages);
    static ssize_t resv_hugepages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h = kobj_to_hstate(kobj, core::ptr::null_mut());
    return sysfs_emit(buf, "%lu\n", h.resv_huge_pages);
    }
    HSTATE_ATTR_RO(resv_hugepages);
    static ssize_t surplus_hugepages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h;
    unsigned long surplus_huge_pages;
    int nid;
    h = kobj_to_hstate(kobj, &nid);
    if (nid == NUMA_NO_NODE)
    surplus_huge_pages = h.surplus_huge_pages;
    else
    surplus_huge_pages = h.surplus_huge_pages_node[nid];
    return sysfs_emit(buf, "%lu\n", surplus_huge_pages);
    }
    HSTATE_ATTR_RO(surplus_hugepages);
    static ssize_t demote_store(struct kobject *kobj,
    struct kobj_attribute *attr, const char *buf, size_t len)
    {
    unsigned long nr_demote;
    unsigned long nr_available;
    nodemask_t nodes_allowed, *n_mask;
    struct hstate *h;
    int err;
    int nid;
    err = kstrtoul(buf, 10, &nr_demote);
    if (err)
    return err;
    h = kobj_to_hstate(kobj, &nid);
    if (nid != NUMA_NO_NODE) {
    init_nodemask_of_node(&nodes_allowed, nid);
    n_mask = &nodes_allowed;
    } else {
    n_mask = &node_states[N_MEMORY];
    }
// Synchronize with other sysfs operations modifying huge pages
    mutex_lock(&h.resize_lock);
    spin_lock_irq(&hugetlb_lock);
    while (nr_demote) {
    long rc;
//
// Check for available pages to demote each time thorough the
// loop as demote_pool_huge_page will drop hugetlb_lock.
//
    if (nid != NUMA_NO_NODE)
    nr_available = h.free_huge_pages_node[nid];
    else
    nr_available = h.free_huge_pages;
    nr_available -= h.resv_huge_pages;
    if (!nr_available)
    break;
    rc = demote_pool_huge_page(h, n_mask, nr_demote);
    if (rc < 0) {
    err = rc;
    break;
    }
    nr_demote -= rc;
    }
    spin_unlock_irq(&hugetlb_lock);
    mutex_unlock(&h.resize_lock);
    if (err)
    return err;
    return len;
    }
    HSTATE_ATTR_WO(demote);
    static ssize_t demote_size_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct hstate *h = kobj_to_hstate(kobj, core::ptr::null_mut());
    let mut demote_size: c_ulong = (PAGE_SIZE << h.demote_order) / SZ_1K;
    return sysfs_emit(buf, "%lukB\n", demote_size);
    }
    static ssize_t demote_size_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct hstate *h, *demote_hstate;
    unsigned long demote_size;
    unsigned int demote_order;
    demote_size = (unsigned long)memparse(buf, core::ptr::null_mut());
    demote_hstate = size_to_hstate(demote_size);
    if (!demote_hstate)
    return -EINVAL;
    demote_order = demote_hstate.order;
    if (demote_order < HUGETLB_PAGE_ORDER)
    return -EINVAL;
// demote order must be smaller than hstate order
    h = kobj_to_hstate(kobj, core::ptr::null_mut());
    if (demote_order >= h.order)
    return -EINVAL;
// resize_lock synchronizes access to demote size and writes
    mutex_lock(&h.resize_lock);
    h.demote_order = demote_order;
    mutex_unlock(&h.resize_lock);
    return count;
    }
    HSTATE_ATTR(demote_size);
    static struct attribute *hstate_attrs[] = {
    &nr_hugepages_attr.attr,
    &nr_overcommit_hugepages_attr.attr,
    &free_hugepages_attr.attr,
    &resv_hugepages_attr.attr,
    &surplus_hugepages_attr.attr,

    &nr_hugepages_mempolicy_attr.attr,

    core::ptr::null_mut(),
    };
    static const struct attribute_group hstate_attr_group = {
    .attrs = hstate_attrs,
    };
    static struct attribute *hstate_demote_attrs[] = {
    &demote_size_attr.attr,
    &demote_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group hstate_demote_attr_group = {
    .attrs = hstate_demote_attrs,
    };
    static int hugetlb_sysfs_add_hstate(struct hstate *h, struct kobject *parent,
    struct kobject **hstate_kobjs,
    const struct attribute_group *hstate_attr_group)
    {
    int retval;
    let mut hi: c_int = hstate_index(h);
    hstate_kobjs[hi] = kobject_create_and_add(h.name, parent);
    if (!hstate_kobjs[hi])
    return -ENOMEM;
    retval = sysfs_create_group(hstate_kobjs[hi], hstate_attr_group);
    if (retval) {
    kobject_put(hstate_kobjs[hi]);
    hstate_kobjs[hi] = core::ptr::null_mut();
    return retval;
    }
    if (h.demote_order) {
    retval = sysfs_create_group(hstate_kobjs[hi],
    &hstate_demote_attr_group);
    if (retval) {
    pr_warn("HugeTLB unable to create demote interfaces for %s\n", h.name);
    sysfs_remove_group(hstate_kobjs[hi], hstate_attr_group);
    kobject_put(hstate_kobjs[hi]);
    hstate_kobjs[hi] = core::ptr::null_mut();
    return retval;
    }
    }
    return 0;
    }

    static bool hugetlb_sysfs_initialized __ro_after_init;
//
// node_hstate/s - associate per node hstate attributes, via their kobjects,
// with node devices in node_devices[] using a parallel array.  The array
// index of a node device or _hstate == node id.
// This is here to avoid any static dependency of the node device driver, in
// the base kernel, on the hugetlb module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_hstate {
    pub hugepages_kobj: *mut kobject,
    pub hstate_kobjs: [*mut kobject; HUGE_MAX_HSTATE],
}

    static struct node_hstate node_hstates[MAX_NUMNODES];
//
// A subset of global hstate attributes for node devices
//
    static struct attribute *per_node_hstate_attrs[] = {
    &nr_hugepages_attr.attr,
    &free_hugepages_attr.attr,
    &surplus_hugepages_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group per_node_hstate_attr_group = {
    .attrs = per_node_hstate_attrs,
    };
//
// kobj_to_node_hstate - lookup global hstate for node device hstate attr kobj.
// Returns node id via non-NULL nidp.
//
    static struct hstate *kobj_to_node_hstate(struct kobject *kobj, int *nidp)
    {
    int nid;
    for (nid = 0; nid < nr_node_ids; nid++) {
    struct node_hstate *nhs = &node_hstates[nid];
    int i;
    for (i = 0; i < HUGE_MAX_HSTATE; i++)
    if (nhs.hstate_kobjs[i] == kobj) {
    if (nidp)
// nidp = nid;
    return &hstates[i];
    }
    }
    BUG();
    return core::ptr::null_mut();
    }
//
// Unregister hstate attributes from a single node device.
// No-op if no hstate attributes attached.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_unregister_node(node: *mut node) {
    void hugetlb_unregister_node(struct node *node)
    {
    struct hstate *h;
    struct node_hstate *nhs = &node_hstates[node.dev.id];
    if (!nhs.hugepages_kobj)
    return;		/* no hstate attributes */
    for_each_hstate(h) {
    let mut idx: c_int = hstate_index(h);
    struct kobject *hstate_kobj = nhs.hstate_kobjs[idx];
    if (!hstate_kobj)
    continue;
    if (h.demote_order)
    sysfs_remove_group(hstate_kobj, &hstate_demote_attr_group);
    sysfs_remove_group(hstate_kobj, &per_node_hstate_attr_group);
    kobject_put(hstate_kobj);
    nhs.hstate_kobjs[idx] = core::ptr::null_mut();
    }
    kobject_put(nhs.hugepages_kobj);
    nhs.hugepages_kobj = core::ptr::null_mut();
    }
//
// Register hstate attributes for a single node device.
// No-op if attributes already registered.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_register_node(node: *mut node) {
    void hugetlb_register_node(struct node *node)
    {
    struct hstate *h;
    struct node_hstate *nhs = &node_hstates[node.dev.id];
    int err;
    if (!hugetlb_sysfs_initialized)
    return;
    if (nhs.hugepages_kobj)
    return;		/* already allocated */
    nhs.hugepages_kobj = kobject_create_and_add("hugepages",
    &node.dev.kobj);
    if (!nhs.hugepages_kobj)
    return;
    for_each_hstate(h) {
    err = hugetlb_sysfs_add_hstate(h, nhs.hugepages_kobj,
    nhs.hstate_kobjs,
    &per_node_hstate_attr_group);
    if (err) {
    pr_err("HugeTLB: Unable to add hstate %s for node %d\n",
    h.name, node.dev.id);
    hugetlb_unregister_node(node);
    break;
    }
    }
    }
//
// hugetlb init time:  register hstate attributes for all registered node
// devices of nodes that have memory.  All on-line nodes should have
// registered their associated device by this time.
//
#[no_mangle]
unsafe extern "C" fn hugetlb_register_all_nodes() -> void __init {
    static void __init hugetlb_register_all_nodes(void)
    {
    int nid;
    for_each_online_node(nid)
    hugetlb_register_node(node_devices[nid]);
    }

    static struct hstate *kobj_to_node_hstate(struct kobject *kobj, int *nidp)
    {
    BUG();
    if (nidp)
// nidp = -1;
    return core::ptr::null_mut();
    }
    static void hugetlb_register_all_nodes(void) { }

#[no_mangle]
pub unsafe extern "C" fn hugetlb_sysfs_init() -> void __init {
    void __init hugetlb_sysfs_init(void)
    {
    struct hstate *h;
    int err;
    hugepages_kobj = kobject_create_and_add("hugepages", mm_kobj);
    if (!hugepages_kobj)
    return;
    for_each_hstate(h) {
    err = hugetlb_sysfs_add_hstate(h, hugepages_kobj,
    hstate_kobjs, &hstate_attr_group);
    if (err)
    pr_err("HugeTLB: Unable to add hstate %s\n", h.name);
    }

    hugetlb_sysfs_initialized = true;

    hugetlb_register_all_nodes();
    }
