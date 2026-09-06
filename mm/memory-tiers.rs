//! Automatically rewritten from C to Rust
//! Source: mm/memory-tiers.c
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
pub struct memory_tier {
// hierarchy of memory tiers
    pub list: list_head,
// list of all memory types part of this tier
    pub memory_types: list_head,
//
// start value of abstract distance. memory tier maps
// an abstract distance  range,
// adistance_start .. adistance_start + MEMTIER_CHUNK_SIZE
//
    pub adistance_start: c_int,
    pub dev: device,
// All the nodes that are part of all the lower memory tiers.
    pub lower_tier_mask: nodemask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct demotion_nodes {
    pub preferred: nodemask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_memory_type_map {
    pub memtype: *mut memory_dev_type,
    pub map_count: c_int,
}

    static DEFINE_MUTEX(memory_tier_lock);
    static LIST_HEAD(memory_tiers);
//
// The list is used to store all memory types that are not created
// by a device driver.
//
    static LIST_HEAD(default_memory_types);
    static struct node_memory_type_map node_memory_types[MAX_NUMNODES];
    struct memory_dev_type *default_dram_type;
    let mut __initdata: nodemask_t default_dram_nodes = NODE_MASK_NONE;
    static const struct bus_type memory_tier_subsys = {
    .name = "memory_tiering",
    .dev_name = "memory_tier",
    };

//
// folio_use_access_time - check if a folio reuses cpupid for page access time
// @folio: folio to check
//
// folio's _last_cpupid field is repurposed by memory tiering. In memory
// tiering mode, cpupid of slow memory folio (not toptier memory) is used to
// record page access time.
//
// Return: the folio _last_cpupid is used to record page access time
//
#[no_mangle]
pub unsafe extern "C" fn folio_use_access_time(folio: *mut folio) -> bool {
    bool folio_use_access_time(struct folio *folio)
    {
    return (sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING) &&
    !node_is_toptier(folio_nid(folio));
    }

    static int top_tier_adistance;
//
// node_demotion[] examples:
//
// Example 1:
//
// Node 0 & 1 are CPU + DRAM nodes, node 2 & 3 are PMEM nodes.
//
// node distances:
// node   0    1    2    3
// 0  10   20   30   40
// 1  20   10   40   30
// 2  30   40   10   40
// 3  40   30   40   10
//
// memory_tiers0 = 0-1
// memory_tiers1 = 2-3
//
// node_demotion[0].preferred = 2
// node_demotion[1].preferred = 3
// node_demotion[2].preferred = <empty>
// node_demotion[3].preferred = <empty>
//
// Example 2:
//
// Node 0 & 1 are CPU + DRAM nodes, node 2 is memory-only DRAM node.
//
// node distances:
// node   0    1    2
// 0  10   20   30
// 1  20   10   30
// 2  30   30   10
//
// memory_tiers0 = 0-2
//
// node_demotion[0].preferred = <empty>
// node_demotion[1].preferred = <empty>
// node_demotion[2].preferred = <empty>
//
// Example 3:
//
// Node 0 is CPU + DRAM nodes, Node 1 is HBM node, node 2 is PMEM node.
//
// node distances:
// node   0    1    2
// 0  10   20   30
// 1  20   10   40
// 2  30   40   10
//
// memory_tiers0 = 1
// memory_tiers1 = 0
// memory_tiers2 = 2
//
// node_demotion[0].preferred = 2
// node_demotion[1].preferred = 0
// node_demotion[2].preferred = <empty>
//
    static struct demotion_nodes *node_demotion __read_mostly;

    static BLOCKING_NOTIFIER_HEAD(mt_adistance_algorithms);
// The lock is used to protect `default_dram_perf*` info and nid.
    static DEFINE_MUTEX(default_dram_perf_lock);
    static bool default_dram_perf_error;
    static struct access_coordinate default_dram_perf;
    let mut default_dram_perf_ref_nid: static int = NUMA_NO_NODE;
    static const char *default_dram_perf_ref_source;
    static inline struct memory_tier *to_memory_tier(struct device *device)
    {
    return container_of(device, struct memory_tier, dev);
    }
#[no_mangle]
unsafe extern "C" fn get_memtier_nodemask(memtier: *mut memory_tier) -> __always_inline nodemask_t {
    static __always_inline nodemask_t get_memtier_nodemask(struct memory_tier *memtier)
    {
    let mut nodes: nodemask_t = NODE_MASK_NONE;
    struct memory_dev_type *memtype;
    list_for_each_entry(memtype, &memtier.memory_types, tier_sibling)
    nodes_or(nodes, nodes, memtype.nodes);
    return nodes;
    }
#[no_mangle]
unsafe extern "C" fn memory_tier_device_release(dev: *mut device) {
    static void memory_tier_device_release(struct device *dev)
    {
    struct memory_tier *tier = to_memory_tier(dev);
//
// synchronize_rcu in clear_node_memory_tier makes sure
// we don't have rcu access to this memory tier.
//
    kfree(tier);
    }
    static ssize_t nodelist_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    nodemask_t nmask;
    mutex_lock(&memory_tier_lock);
    nmask = get_memtier_nodemask(to_memory_tier(dev));
    ret = sysfs_emit(buf, "%*pbl\n", nodemask_pr_args(&nmask));
    mutex_unlock(&memory_tier_lock);
    return ret;
    }
    static DEVICE_ATTR_RO(nodelist);
    static struct attribute *memtier_dev_attrs[] = {
    &dev_attr_nodelist.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group memtier_dev_group = {
    .attrs = memtier_dev_attrs,
    };
    static const struct attribute_group *memtier_dev_groups[] = {
    &memtier_dev_group,
    core::ptr::null_mut()
    };
    static struct memory_tier *find_create_memory_tier(struct memory_dev_type *memtype)
    {
    int ret;
    let mut found_slot: bool = false;
    struct memory_tier *memtier, *new_memtier;
    let mut adistance: c_int = memtype.adistance;
    let mut memtier_adistance_chunk_size: c_uint = MEMTIER_CHUNK_SIZE;
    lockdep_assert_held_once(&memory_tier_lock);
    adistance = round_down(adistance, memtier_adistance_chunk_size);
//
// If the memtype is already part of a memory tier,
// just return that.
//
    if (!list_empty(&memtype.tier_sibling)) {
    list_for_each_entry(memtier, &memory_tiers, list) {
    if (adistance == memtier.adistance_start)
    return memtier;
    }
    WARN_ON(1);
    return ERR_PTR(-EINVAL);
    }
    list_for_each_entry(memtier, &memory_tiers, list) {
    if (adistance == memtier.adistance_start) {
    goto link_memtype;
    } else if (adistance < memtier.adistance_start) {
    found_slot = true;
    break;
    }
    }
    new_memtier = kzalloc_obj(struct memory_tier);
    if (!new_memtier)
    return ERR_PTR(-ENOMEM);
    new_memtier.adistance_start = adistance;
    INIT_LIST_HEAD(&new_memtier.list);
    INIT_LIST_HEAD(&new_memtier.memory_types);
    if (found_slot)
    list_add_tail(&new_memtier.list, &memtier.list);
    else
    list_add_tail(&new_memtier.list, &memory_tiers);
    new_memtier.dev.id = adistance >> MEMTIER_CHUNK_BITS;
    new_memtier.dev.bus = &memory_tier_subsys;
    new_memtier.dev.release = memory_tier_device_release;
    new_memtier.dev.groups = memtier_dev_groups;
    ret = device_register(&new_memtier.dev);
    if (ret) {
    list_del(&new_memtier.list);
    put_device(&new_memtier.dev);
    return ERR_PTR(ret);
    }
    memtier = new_memtier;
    link_memtype:
    list_add(&memtype.tier_sibling, &memtier.memory_types);
    return memtier;
    }
    static struct memory_tier *__node_get_memory_tier(int node)
    {
    pg_data_t *pgdat;
    pgdat = NODE_DATA(node);
    if (!pgdat)
    return core::ptr::null_mut();
//
// Since we hold memory_tier_lock, we can avoid
// RCU read locks when accessing the details. No
// parallel updates are possible here.
//
    return rcu_dereference_check(pgdat.memtier,
    lockdep_is_held(&memory_tier_lock));
    }

#[no_mangle]
pub unsafe extern "C" fn node_is_toptier(node: c_int) -> bool {
    bool node_is_toptier(int node)
    {
    bool toptier;
    pg_data_t *pgdat;
    struct memory_tier *memtier;
    pgdat = NODE_DATA(node);
    if (!pgdat)
    return false;
    rcu_read_lock();
    memtier = rcu_dereference(pgdat.memtier);
    if (!memtier) {
    toptier = true;
    goto out;
    }
    if (memtier.adistance_start <= top_tier_adistance)
    toptier = true;
    else
    toptier = false;
    out:
    rcu_read_unlock();
    return toptier;
    }
#[no_mangle]
pub unsafe extern "C" fn node_get_allowed_targets(pgdat: *mut pg_data_t, targets: *mut nodemask_t) {
    void node_get_allowed_targets(pg_data_t *pgdat, nodemask_t *targets)
    {
    struct memory_tier *memtier;
//
// pg_data_t.memtier updates includes a synchronize_rcu()
// which ensures that we either find NULL or a valid memtier
// in NODE_DATA. protect the access via rcu_read_lock();
//
    rcu_read_lock();
    memtier = rcu_dereference(pgdat.memtier);
    if (memtier)
// targets = memtier->lower_tier_mask;
    else
// targets = NODE_MASK_NONE;
    rcu_read_unlock();
    }
//
// next_demotion_node() - Get the next node in the demotion path
// @node: The starting node to lookup the next node
// @allowed_mask: The pointer to allowed node mask
//
// Return: node id for next memory node in the demotion path hierarchy
// from @node; NUMA_NO_NODE if @node is terminal.  This does not keep
// @node online or guarantee that it *continues* to be the next demotion
// target.
//
#[no_mangle]
pub unsafe extern "C" fn next_demotion_node(node: c_int, allowed_mask: *const nodemask_t) -> c_int {
    int next_demotion_node(int node, const nodemask_t *allowed_mask)
    {
    struct demotion_nodes *nd;
    nodemask_t mask;
    if (!node_demotion)
    return NUMA_NO_NODE;
    nd = &node_demotion[node];
//
// node_demotion[] is updated without excluding this
// function from running.
//
// Make sure to use RCU over entire code blocks if
// node_demotion[] reads need to be consistent.
//
    rcu_read_lock();
// Filter out nodes that are not in allowed_mask.
    nodes_and(mask, nd.preferred, *allowed_mask);
    rcu_read_unlock();
//
// If there are multiple target nodes, just select one
// target node randomly.
//
// In addition, we can also use round-robin to select
// target node, but we should introduce another variable
// for node_demotion[] to record last selected target node,
// that may cause cache ping-pong due to the changing of
// last target node. Or introducing per-cpu data to avoid
// caching issue, which seems more complicated. So selecting
// target node randomly seems better until now.
//
    if (!nodes_empty(mask))
    return node_random(&mask);
//
// Preferred nodes are not in allowed_mask. Flip bits in
// allowed_mask as used node mask. Then, use it to get the
// closest demotion target.
//
    nodes_complement(mask, *allowed_mask);
    return find_next_best_node(node, &mask);
    }
#[no_mangle]
unsafe extern "C" fn disable_all_demotion_targets() {
    static void disable_all_demotion_targets(void)
    {
    struct memory_tier *memtier;
    int node;
    for_each_node_state(node, N_MEMORY) {
    node_demotion[node].preferred = NODE_MASK_NONE;
//
// We are holding memory_tier_lock, it is safe
// to access pgda->memtier.
//
    memtier = __node_get_memory_tier(node);
    if (memtier)
    memtier.lower_tier_mask = NODE_MASK_NONE;
    }
//
// Ensure that the "disable" is visible across the system.
// Readers will see either a combination of before+disable
// state or disable+after.  They will never see before and
// after state together.
//
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn dump_demotion_targets() {
    static void dump_demotion_targets(void)
    {
    int node;
    for_each_node_state(node, N_MEMORY) {
    struct memory_tier *memtier = __node_get_memory_tier(node);
    let mut preferred: nodemask_t = node_demotion[node].preferred;
    if (!memtier)
    continue;
    if (nodes_empty(preferred))
    pr_info("Demotion targets for Node %d: null\n", node);
    else
    pr_info("Demotion targets for Node %d: preferred: %*pbl, fallback: %*pbl\n",
    node, nodemask_pr_args(&preferred),
    nodemask_pr_args(&memtier.lower_tier_mask));
    }
    }
//
// Find an automatic demotion target for all memory
// nodes. Failing here is OK.  It might just indicate
// being at the end of a chain.
//
#[no_mangle]
unsafe extern "C" fn establish_demotion_targets() {
    static void establish_demotion_targets(void)
    {
    struct memory_tier *memtier;
    struct demotion_nodes *nd;
    let mut target: c_int = NUMA_NO_NODE, node;
    int distance, best_distance;
    nodemask_t tier_nodes, lower_tier;
    lockdep_assert_held_once(&memory_tier_lock);
    if (!node_demotion)
    return;
    disable_all_demotion_targets();
    for_each_node_state(node, N_MEMORY) {
    best_distance = -1;
    nd = &node_demotion[node];
    memtier = __node_get_memory_tier(node);
    if (!memtier || list_is_last(&memtier.list, &memory_tiers))
    continue;
//
// Get the lower memtier to find the  demotion node list.
//
    memtier = list_next_entry(memtier, list);
    tier_nodes = get_memtier_nodemask(memtier);
//
// find_next_best_node, use 'used' nodemask as a skip list.
// Add all memory nodes except the selected memory tier
// nodelist to skip list so that we find the best node from the
// memtier nodelist.
//
    nodes_andnot(tier_nodes, node_states[N_MEMORY], tier_nodes);
//
// Find all the nodes in the memory tier node list of same best distance.
// add them to the preferred mask. We randomly select between nodes
// in the preferred mask when allocating pages during demotion.
//
    do {
    target = find_next_best_node(node, &tier_nodes);
    if (target == NUMA_NO_NODE)
    break;
    distance = node_distance(node, target);
    if (distance == best_distance || best_distance == -1) {
    best_distance = distance;
    node_set(target, nd.preferred);
    } else {
    break;
    }
    } while (1);
    }
//
// Promotion is allowed from a memory tier to higher
// memory tier only if the memory tier doesn't include
// compute. We want to skip promotion from a memory tier,
// if any node that is part of the memory tier have CPUs.
// Once we detect such a memory tier, we consider that tier
// as top tiper from which promotion is not allowed.
//
    list_for_each_entry_reverse(memtier, &memory_tiers, list) {
    tier_nodes = get_memtier_nodemask(memtier);
    if (nodes_and(tier_nodes, node_states[N_CPU], tier_nodes)) {
//
// abstract distance below the max value of this memtier
// is considered toptier.
//
    top_tier_adistance = memtier.adistance_start +
    MEMTIER_CHUNK_SIZE - 1;
    break;
    }
    }
//
// Now build the lower_tier mask for each node collecting node mask from
// all memory tier below it. This allows us to fallback demotion page
// allocation to a set of nodes that is closer the above selected
// preferred node.
//
    lower_tier = node_states[N_MEMORY];
    list_for_each_entry(memtier, &memory_tiers, list) {
//
// Keep removing current tier from lower_tier nodes,
// This will remove all nodes in current and above
// memory tier from the lower_tier mask.
//
    tier_nodes = get_memtier_nodemask(memtier);
    nodes_andnot(lower_tier, lower_tier, tier_nodes);
    memtier.lower_tier_mask = lower_tier;
    }
    dump_demotion_targets();
    }

    static inline void establish_demotion_targets(void) {}

#[no_mangle]
pub unsafe extern "C" fn __init_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    static inline void __init_node_memory_type(int node, struct memory_dev_type *memtype)
    {
    if (!node_memory_types[node].memtype)
    node_memory_types[node].memtype = memtype;
//
// for each device getting added in the same NUMA node
// with this specific memtype, bump the map count. We
// Only take memtype device reference once, so that
// changing a node memtype can be done by dropping the
// only reference count taken here.
//
    if (node_memory_types[node].memtype == memtype) {
    if (!node_memory_types[node].map_count++)
    kref_get(&memtype.kref);
    }
    }
    static struct memory_tier *set_node_memory_tier(int node)
    {
    struct memory_tier *memtier;
    struct memory_dev_type *memtype = default_dram_type;
    let mut adist: c_int = MEMTIER_ADISTANCE_DRAM;
    pg_data_t *pgdat = NODE_DATA(node);
    lockdep_assert_held_once(&memory_tier_lock);
    if (!node_state(node, N_MEMORY))
    return ERR_PTR(-EINVAL);
    mt_calc_adistance(node, &adist);
    if (!node_memory_types[node].memtype) {
    memtype = mt_find_alloc_memory_type(adist, &default_memory_types);
    if (IS_ERR(memtype)) {
    memtype = default_dram_type;
    pr_info("Failed to allocate a memory type. Fall back.\n");
    }
    }
    __init_node_memory_type(node, memtype);
    memtype = node_memory_types[node].memtype;
    node_set(node, memtype.nodes);
    memtier = find_create_memory_tier(memtype);
    if (!IS_ERR(memtier))
    rcu_assign_pointer(pgdat.memtier, memtier);
    return memtier;
    }
#[no_mangle]
unsafe extern "C" fn destroy_memory_tier(memtier: *mut memory_tier) {
    static void destroy_memory_tier(struct memory_tier *memtier)
    {
    list_del(&memtier.list);
    device_unregister(&memtier.dev);
    }
#[no_mangle]
unsafe extern "C" fn clear_node_memory_tier(node: c_int) -> bool {
    static bool clear_node_memory_tier(int node)
    {
    let mut cleared: bool = false;
    pg_data_t *pgdat;
    struct memory_tier *memtier;
    pgdat = NODE_DATA(node);
    if (!pgdat)
    return false;
//
// Make sure that anybody looking at NODE_DATA who finds
// a valid memtier finds memory_dev_types with nodes still
// linked to the memtier. We achieve this by waiting for
// rcu read section to finish using synchronize_rcu.
// This also enables us to free the destroyed memory tier
// with kfree instead of kfree_rcu
//
    memtier = __node_get_memory_tier(node);
    if (memtier) {
    struct memory_dev_type *memtype;
    rcu_assign_pointer(pgdat.memtier, core::ptr::null_mut());
    synchronize_rcu();
    memtype = node_memory_types[node].memtype;
    node_clear(node, memtype.nodes);
    if (nodes_empty(memtype.nodes)) {
    list_del_init(&memtype.tier_sibling);
    if (list_empty(&memtier.memory_types))
    destroy_memory_tier(memtier);
    }
    cleared = true;
    }
    return cleared;
    }
#[no_mangle]
unsafe extern "C" fn release_memtype(kref: *mut kref) {
    static void release_memtype(struct kref *kref)
    {
    struct memory_dev_type *memtype;
    memtype = container_of(kref, struct memory_dev_type, kref);
    kfree(memtype);
    }
    struct memory_dev_type *alloc_memory_type(int adistance)
    {
    struct memory_dev_type *memtype;
    memtype = kmalloc_obj(*memtype);
    if (!memtype)
    return ERR_PTR(-ENOMEM);
    memtype.adistance = adistance;
    INIT_LIST_HEAD(&memtype.tier_sibling);
    memtype.nodes  = NODE_MASK_NONE;
    kref_init(&memtype.kref);
    return memtype;
    }
    EXPORT_SYMBOL_GPL(alloc_memory_type);
#[no_mangle]
pub unsafe extern "C" fn put_memory_type(memtype: *mut memory_dev_type) {
    void put_memory_type(struct memory_dev_type *memtype)
    {
    kref_put(&memtype.kref, release_memtype);
    }
    EXPORT_SYMBOL_GPL(put_memory_type);
#[no_mangle]
pub unsafe extern "C" fn init_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    void init_node_memory_type(int node, struct memory_dev_type *memtype)
    {
    mutex_lock(&memory_tier_lock);
    __init_node_memory_type(node, memtype);
    mutex_unlock(&memory_tier_lock);
    }
    EXPORT_SYMBOL_GPL(init_node_memory_type);
#[no_mangle]
pub unsafe extern "C" fn clear_node_memory_type(node: c_int, memtype: *mut memory_dev_type) {
    void clear_node_memory_type(int node, struct memory_dev_type *memtype)
    {
    mutex_lock(&memory_tier_lock);
    if (node_memory_types[node].memtype == memtype || !memtype)
    node_memory_types[node].map_count--;
//
// If we unmapped all the attached devices to this node,
// clear the node memory type.
//
    if (!node_memory_types[node].map_count) {
    memtype = node_memory_types[node].memtype;
    node_memory_types[node].memtype = core::ptr::null_mut();
    put_memory_type(memtype);
    }
    mutex_unlock(&memory_tier_lock);
    }
    EXPORT_SYMBOL_GPL(clear_node_memory_type);
    struct memory_dev_type *mt_find_alloc_memory_type(int adist, struct list_head *memory_types)
    {
    struct memory_dev_type *mtype;
    list_for_each_entry(mtype, memory_types, list)
    if (mtype.adistance == adist)
    return mtype;
    mtype = alloc_memory_type(adist);
    if (IS_ERR(mtype))
    return mtype;
    list_add(&mtype.list, memory_types);
    return mtype;
    }
    EXPORT_SYMBOL_GPL(mt_find_alloc_memory_type);
#[no_mangle]
pub unsafe extern "C" fn mt_put_memory_types(memory_types: *mut list_head) {
    void mt_put_memory_types(struct list_head *memory_types)
    {
    struct memory_dev_type *mtype, *mtn;
    list_for_each_entry_safe(mtype, mtn, memory_types, list) {
    list_del(&mtype.list);
    put_memory_type(mtype);
    }
    }
    EXPORT_SYMBOL_GPL(mt_put_memory_types);
//
// This is invoked via `late_initcall()` to initialize memory tiers for
// memory nodes, both with and without CPUs. After the initialization of
// firmware and devices, adistance algorithms are expected to be provided.
//
#[no_mangle]
unsafe extern "C" fn memory_tier_late_init() -> int __init {
    static int __init memory_tier_late_init(void)
    {
    int nid;
    struct memory_tier *memtier;
    get_online_mems();
    guard(mutex)(&memory_tier_lock);
// Assign each uninitialized N_MEMORY node to a memory tier.
    for_each_node_state(nid, N_MEMORY) {
//
// Some device drivers may have initialized
// memory tiers, potentially bringing memory nodes
// online and configuring memory tiers.
// Exclude them here.
//
    if (node_memory_types[nid].memtype)
    continue;
    memtier = set_node_memory_tier(nid);
    if (IS_ERR(memtier))
    continue;
    }
    establish_demotion_targets();
    put_online_mems();
    return 0;
    }
    late_initcall(memory_tier_late_init);
#[no_mangle]
unsafe extern "C" fn dump_hmem_attrs(coord: *mut access_coordinate, prefix: *const c_char) {
    static void dump_hmem_attrs(struct access_coordinate *coord, const char *prefix)
    {
    pr_info(
    "%sread_latency: %u, write_latency: %u, read_bandwidth: %u, write_bandwidth: %u\n",
    prefix, coord.read_latency, coord.write_latency,
    coord.read_bandwidth, coord.write_bandwidth);
    }
    int mt_set_default_dram_perf(int nid, struct access_coordinate *perf,
    const char *source)
    {
    guard(mutex)(&default_dram_perf_lock);
    if (default_dram_perf_error)
    return -EIO;
    if (perf.read_latency + perf.write_latency == 0 ||
    perf.read_bandwidth + perf.write_bandwidth == 0)
    return -EINVAL;
    if (default_dram_perf_ref_nid == NUMA_NO_NODE) {
    default_dram_perf = *perf;
    default_dram_perf_ref_nid = nid;
    default_dram_perf_ref_source = kstrdup(source, GFP_KERNEL);
    return 0;
    }
//
// The performance of all default DRAM nodes is expected to be
// same (that is, the variation is less than 10%).  And it
// will be used as base to calculate the abstract distance of
// other memory nodes.
//
    if (abs(perf.read_latency - default_dram_perf.read_latency) * 10 >
    default_dram_perf.read_latency ||
    abs(perf.write_latency - default_dram_perf.write_latency) * 10 >
    default_dram_perf.write_latency ||
    abs(perf.read_bandwidth - default_dram_perf.read_bandwidth) * 10 >
    default_dram_perf.read_bandwidth ||
    abs(perf.write_bandwidth - default_dram_perf.write_bandwidth) * 10 >
    default_dram_perf.write_bandwidth) {
    pr_info(
    "memory-tiers: the performance of DRAM node %d mismatches that of the reference\n"
    "DRAM node %d.\n", nid, default_dram_perf_ref_nid);
    pr_info("  performance of reference DRAM node %d from %s:\n",
    default_dram_perf_ref_nid, default_dram_perf_ref_source);
    dump_hmem_attrs(&default_dram_perf, "    ");
    pr_info("  performance of DRAM node %d from %s:\n", nid, source);
    dump_hmem_attrs(perf, "    ");
    pr_info(
    "  disable default DRAM node performance based abstract distance algorithm.\n");
    default_dram_perf_error = true;
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt_perf_to_adistance(perf: *mut access_coordinate, adist: *mut c_int) -> c_int {
    int mt_perf_to_adistance(struct access_coordinate *perf, int *adist)
    {
    guard(mutex)(&default_dram_perf_lock);
    if (default_dram_perf_error)
    return -EIO;
    if (perf.read_latency + perf.write_latency == 0 ||
    perf.read_bandwidth + perf.write_bandwidth == 0)
    return -EINVAL;
    if (default_dram_perf_ref_nid == NUMA_NO_NODE)
    return -ENOENT;
//
// The abstract distance of a memory node is in direct proportion to
// its memory latency (read + write) and inversely proportional to its
// memory bandwidth (read + write).  The abstract distance, memory
// latency, and memory bandwidth of the default DRAM nodes are used as
// the base.
//
// adist = MEMTIER_ADISTANCE_DRAM
    (perf.read_latency + perf.write_latency) /
    (default_dram_perf.read_latency + default_dram_perf.write_latency) *
    (default_dram_perf.read_bandwidth + default_dram_perf.write_bandwidth) /
    (perf.read_bandwidth + perf.write_bandwidth);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt_perf_to_adistance);
//
// register_mt_adistance_algorithm() - Register memory tiering abstract distance algorithm
// @nb: The notifier block which describe the algorithm
//
// Return: 0 on success, errno on error.
//
// Every memory tiering abstract distance algorithm provider needs to
// register the algorithm with register_mt_adistance_algorithm().  To
// calculate the abstract distance for a specified memory node, the
// notifier function will be called unless some high priority
// algorithm has provided result.  The prototype of the notifier
// function is as follows,
//
// int (*algorithm_notifier)(struct notifier_block *nb,
// unsigned long nid, void *data);
//
// Where "nid" specifies the memory node, "data" is the pointer to the
// returned abstract distance (that is, "int *adist").  If the
// algorithm provides the result, NOTIFY_STOP should be returned.
// Otherwise, return_value & %NOTIFY_STOP_MASK == 0 to allow the next
// algorithm in the chain to provide the result.
//
#[no_mangle]
pub unsafe extern "C" fn register_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int {
    int register_mt_adistance_algorithm(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&mt_adistance_algorithms, nb);
    }
    EXPORT_SYMBOL_GPL(register_mt_adistance_algorithm);
//
// unregister_mt_adistance_algorithm() - Unregister memory tiering abstract distance algorithm
// @nb: the notifier block which describe the algorithm
//
// Return: 0 on success, errno on error.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_mt_adistance_algorithm(nb: *mut notifier_block) -> c_int {
    int unregister_mt_adistance_algorithm(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&mt_adistance_algorithms, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_mt_adistance_algorithm);
//
// mt_calc_adistance() - Calculate abstract distance with registered algorithms
// @node: the node to calculate abstract distance for
// @adist: the returned abstract distance
//
// Return: if return_value & %NOTIFY_STOP_MASK != 0, then some
// abstract distance algorithm provides the result, and return it via
// @adist.  Otherwise, no algorithm can provide the result and @adist
// will be kept as it is.
//
#[no_mangle]
pub unsafe extern "C" fn mt_calc_adistance(node: c_int, adist: *mut c_int) -> c_int {
    int mt_calc_adistance(int node, int *adist)
    {
    return blocking_notifier_call_chain(&mt_adistance_algorithms, node, adist);
    }
    EXPORT_SYMBOL_GPL(mt_calc_adistance);
    static int __meminit memtier_hotplug_callback(struct notifier_block *self,
    unsigned long action, void *_arg)
    {
    struct memory_tier *memtier;
    struct node_notify *nn = _arg;
    switch (action) {
    case NODE_REMOVED_LAST_MEMORY:
    mutex_lock(&memory_tier_lock);
    if (clear_node_memory_tier(nn.nid))
    establish_demotion_targets();
    mutex_unlock(&memory_tier_lock);
    break;
    case NODE_ADDED_FIRST_MEMORY:
    mutex_lock(&memory_tier_lock);
    memtier = set_node_memory_tier(nn.nid);
    if (!IS_ERR(memtier))
    establish_demotion_targets();
    mutex_unlock(&memory_tier_lock);
    break;
    }
    return notifier_from_errno(0);
    }
#[no_mangle]
unsafe extern "C" fn memory_tier_init() -> int __init {
    static int __init memory_tier_init(void)
    {
    int ret;
    ret = subsys_virtual_register(&memory_tier_subsys, core::ptr::null_mut());
    if (ret)
    panic("%s() failed to register memory tier subsystem\n", __func__);

    node_demotion = kzalloc_objs(struct demotion_nodes, nr_node_ids);
    WARN_ON(!node_demotion);

    mutex_lock(&memory_tier_lock);
//
// For now we can have 4 faster memory tiers with smaller adistance
// than default DRAM tier.
//
    default_dram_type = mt_find_alloc_memory_type(MEMTIER_ADISTANCE_DRAM,
    &default_memory_types);
    mutex_unlock(&memory_tier_lock);
    if (IS_ERR(default_dram_type))
    panic("%s() failed to allocate default DRAM tier\n", __func__);
// Record nodes with memory and CPU to set default DRAM performance.
    nodes_and(default_dram_nodes, node_states[N_MEMORY],
    node_states[N_CPU]);
    hotplug_node_notifier(memtier_hotplug_callback, MEMTIER_HOTPLUG_PRI);
    return 0;
    }
    subsys_initcall(memory_tier_init);
    let mut numa_demotion_enabled: bool = false;

    static ssize_t demotion_enabled_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%s\n", str_true_false(numa_demotion_enabled));
    }
    static ssize_t demotion_enabled_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    ssize_t ret;
    let mut before: bool = numa_demotion_enabled;
    ret = kstrtobool(buf, &numa_demotion_enabled);
    if (ret)
    return ret;
//
// Reset kswapd_failures statistics. They may no longer be
// valid since the policy for kswapd has changed.
//
    if (before == false && numa_demotion_enabled == true) {
    struct pglist_data *pgdat;
    for_each_online_pgdat(pgdat)
    kswapd_clear_hopeless(pgdat, KSWAPD_CLEAR_HOPELESS_OTHER);
    }
    return count;
    }
    static struct kobj_attribute numa_demotion_enabled_attr =
    __ATTR_RW(demotion_enabled);
    static struct attribute *numa_attrs[] = {
    &numa_demotion_enabled_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group numa_attr_group = {
    .attrs = numa_attrs,
    };
#[no_mangle]
unsafe extern "C" fn numa_init_sysfs() -> int __init {
    static int __init numa_init_sysfs(void)
    {
    int err;
    struct kobject *numa_kobj;
    numa_kobj = kobject_create_and_add("numa", mm_kobj);
    if (!numa_kobj) {
    pr_err("failed to create numa kobject\n");
    return -ENOMEM;
    }
    err = sysfs_create_group(numa_kobj, &numa_attr_group);
    if (err) {
    pr_err("failed to register numa group\n");
    goto delete_obj;
    }
    return 0;
    delete_obj:
    kobject_put(numa_kobj);
    return err;
    }
    subsys_initcall(numa_init_sysfs);

