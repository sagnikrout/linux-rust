//! Automatically rewritten from C to Rust
//! Source: drivers/base/cacheinfo.c
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
// cacheinfo support - processor cache information via sysfs
//
// Based on arch/x86/kernel/cpu/intel_cacheinfo.c
// Author: Sudeep Holla <sudeep.holla@arm.com>
//

// pointer to per cpu cacheinfo
    static DEFINE_PER_CPU(struct cpu_cacheinfo, ci_cpu_cacheinfo);

    (per_cpu_cacheinfo(cpu) + (idx))
// Set if no cache information is found in DT/ACPI.
    static bool use_arch_info;
    struct cpu_cacheinfo *get_cpu_cacheinfo(unsigned int cpu)
    {
    return ci_cacheinfo(cpu);
    }
    static inline bool cache_leaves_are_shared(struct cacheinfo *this_leaf,
    struct cacheinfo *sib_leaf)
    {
//
// For non DT/ACPI systems, assume unique level 1 caches,
// system-wide shared caches for all other levels.
//
    if (!(IS_ENABLED(CONFIG_OF) || IS_ENABLED(CONFIG_ACPI)) ||
    use_arch_info)
    return (this_leaf.level != 1) && (sib_leaf.level != 1);
    if ((sib_leaf.attributes & CACHE_ID) &&
    (this_leaf.attributes & CACHE_ID))
    return sib_leaf.id == this_leaf.id;
    return sib_leaf.fw_token == this_leaf.fw_token;
    }
#[no_mangle]
pub unsafe extern "C" fn last_level_cache_is_valid(cpu: c_uint) -> bool {
    bool last_level_cache_is_valid(unsigned int cpu)
    {
    struct cacheinfo *llc;
    if (!cache_leaves(cpu) || !per_cpu_cacheinfo(cpu))
    return false;
    llc = per_cpu_cacheinfo_idx(cpu, cache_leaves(cpu) - 1);
    return (llc.attributes & CACHE_ID) || !!llc.fw_token;
    }
//
// Get the cacheinfo of the LLC associated with @cpu.
// Derived from update_per_cpu_data_slice_size_cpu().
//
    struct cacheinfo *get_cpu_cacheinfo_llc(unsigned int cpu)
    {
    struct cacheinfo *llc;
    if (!last_level_cache_is_valid(cpu))
    return core::ptr::null_mut();
    llc = per_cpu_cacheinfo_idx(cpu, cache_leaves(cpu) - 1);
    if (llc.type != CACHE_TYPE_DATA && llc.type != CACHE_TYPE_UNIFIED)
    return core::ptr::null_mut();
    return llc;
    }
#[no_mangle]
pub unsafe extern "C" fn last_level_cache_is_shared(cpu_x: c_uint, cpu_y: c_uint) -> bool {
    bool last_level_cache_is_shared(unsigned int cpu_x, unsigned int cpu_y)
    {
    struct cacheinfo *llc_x, *llc_y;
    if (!last_level_cache_is_valid(cpu_x) ||
    !last_level_cache_is_valid(cpu_y))
    return false;
    llc_x = per_cpu_cacheinfo_idx(cpu_x, cache_leaves(cpu_x) - 1);
    llc_y = per_cpu_cacheinfo_idx(cpu_y, cache_leaves(cpu_y) - 1);
    return cache_leaves_are_shared(llc_x, llc_y);
    }

    static bool of_check_cache_nodes(struct device_node *np);
// OF properties to query for a given cache type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_type_info {
    pub size_prop: *const c_char,
    pub line_size_props: [*const c_char; 2],
    pub nr_sets_prop: *const c_char,
}

    static const struct cache_type_info cache_type_info[] = {
    {
    .size_prop       = "cache-size",
    .line_size_props = { "cache-line-size",
    "cache-block-size", },
    .nr_sets_prop    = "cache-sets",
    }, {
    .size_prop       = "i-cache-size",
    .line_size_props = { "i-cache-line-size",
    "i-cache-block-size", },
    .nr_sets_prop    = "i-cache-sets",
    }, {
    .size_prop       = "d-cache-size",
    .line_size_props = { "d-cache-line-size",
    "d-cache-block-size", },
    .nr_sets_prop    = "d-cache-sets",
    },
    };
#[no_mangle]
pub unsafe extern "C" fn get_cacheinfo_idx(type: enum cache_type) -> c_int {
    static inline int get_cacheinfo_idx(enum cache_type type)
    {
    if (type == CACHE_TYPE_UNIFIED)
    return 0;
    return type;
    }
#[no_mangle]
unsafe extern "C" fn cache_size(this_leaf: *mut cacheinfo, np: *mut device_node) {
    static void cache_size(struct cacheinfo *this_leaf, struct device_node *np)
    {
    const char *propname;
    int ct_idx;
    ct_idx = get_cacheinfo_idx(this_leaf.type);
    propname = cache_type_info[ct_idx].size_prop;
    of_property_read_u32(np, propname, &this_leaf.size);
    }
// not cache_line_size() because that's a macro in include/linux/cache.h
    static void cache_get_line_size(struct cacheinfo *this_leaf,
    struct device_node *np)
    {
    int i, lim, ct_idx;
    ct_idx = get_cacheinfo_idx(this_leaf.type);
    lim = ARRAY_SIZE(cache_type_info[ct_idx].line_size_props);
    for (i = 0; i < lim; i++) {
    int ret;
    u32 line_size;
    const char *propname;
    propname = cache_type_info[ct_idx].line_size_props[i];
    ret = of_property_read_u32(np, propname, &line_size);
    if (!ret) {
    this_leaf.coherency_line_size = line_size;
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cache_nr_sets(this_leaf: *mut cacheinfo, np: *mut device_node) {
    static void cache_nr_sets(struct cacheinfo *this_leaf, struct device_node *np)
    {
    const char *propname;
    int ct_idx;
    ct_idx = get_cacheinfo_idx(this_leaf.type);
    propname = cache_type_info[ct_idx].nr_sets_prop;
    of_property_read_u32(np, propname, &this_leaf.number_of_sets);
    }
#[no_mangle]
unsafe extern "C" fn cache_associativity(this_leaf: *mut cacheinfo) {
    static void cache_associativity(struct cacheinfo *this_leaf)
    {
    let mut line_size: c_uint = this_leaf.coherency_line_size;
    let mut nr_sets: c_uint = this_leaf.number_of_sets;
    let mut size: c_uint = this_leaf.size;
//
// If the cache is fully associative, there is no need to
// check the other properties.
//
    if (!(nr_sets == 1) && (nr_sets > 0 && size > 0 && line_size > 0))
    this_leaf.ways_of_associativity = (size / nr_sets) / line_size;
    }
    static bool cache_node_is_unified(struct cacheinfo *this_leaf,
    struct device_node *np)
    {
    return of_property_read_bool(np, "cache-unified");
    }
    static bool match_cache_node(struct device_node *cpu,
    const struct device_node *cache_node)
    {
    struct device_node *prev, *cache = of_find_next_cache_node(cpu);
    while (cache) {
    if (cache == cache_node) {
    of_node_put(cache);
    return true;
    }
    prev = cache;
    cache = of_find_next_cache_node(cache);
    of_node_put(prev);
    }
    return false;
    }

    static void cache_of_set_id(struct cacheinfo *this_leaf,
    struct device_node *cache_node)
    {
    struct device_node *cpu;
    let mut min_id: u32 = ~0;
    for_each_of_cpu_node(cpu) {
    let mut id: u64 = of_get_cpu_hwid(cpu, 0);
    id = arch_compact_of_hwid(id);
    if (FIELD_GET(GENMASK_ULL(63, 32), id)) {
    of_node_put(cpu);
    return;
    }
    if (match_cache_node(cpu, cache_node))
    min_id = min(min_id, id);
    }
    if (min_id != ~0) {
    this_leaf.id = min_id;
    this_leaf.attributes |= CACHE_ID;
    }
    }
    static void cache_of_set_props(struct cacheinfo *this_leaf,
    struct device_node *np)
    {
//
// init_cache_level must setup the cache level correctly
// overriding the architecturally specified levels, so
// if type is NONE at this stage, it should be unified
//
    if (this_leaf.type == CACHE_TYPE_NOCACHE &&
    cache_node_is_unified(this_leaf, np))
    this_leaf.type = CACHE_TYPE_UNIFIED;
    cache_size(this_leaf, np);
    cache_get_line_size(this_leaf, np);
    cache_nr_sets(this_leaf, np);
    cache_associativity(this_leaf);
    cache_of_set_id(this_leaf, np);
    }
#[no_mangle]
unsafe extern "C" fn cache_setup_of_node(cpu: c_uint) -> c_int {
    static int cache_setup_of_node(unsigned int cpu)
    {
    struct cacheinfo *this_leaf;
    let mut index: c_uint = 0;
    struct device_node *np __free(device_node) = of_cpu_device_node_get(cpu);
    if (!np) {
    pr_err("Failed to find cpu%d device node\n", cpu);
    return -ENOENT;
    }
    if (!of_check_cache_nodes(np)) {
    return -ENOENT;
    }
    while (index < cache_leaves(cpu)) {
    this_leaf = per_cpu_cacheinfo_idx(cpu, index);
    if (this_leaf.level != 1) {
    struct device_node *prev __free(device_node) = np;
    np = of_find_next_cache_node(np);
    if (!np)
    break;
    }
    cache_of_set_props(this_leaf, np);
    this_leaf.fw_token = np;
    index++;
    }
    if (index != cache_leaves(cpu)) /* not all OF nodes populated */
    return -ENOENT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn of_check_cache_nodes(np: *mut device_node) -> bool {
    static bool of_check_cache_nodes(struct device_node *np)
    {
    if (of_property_present(np, "cache-size")   ||
    of_property_present(np, "i-cache-size") ||
    of_property_present(np, "d-cache-size") ||
    of_property_present(np, "cache-unified"))
    return true;
    struct device_node *next __free(device_node) = of_find_next_cache_node(np);
    if (next) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn of_count_cache_leaves(np: *mut device_node) -> c_int {
    static int of_count_cache_leaves(struct device_node *np)
    {
    let mut leaves: c_uint = 0;
    if (of_property_present(np, "cache-size"))
    ++leaves;
    if (of_property_present(np, "i-cache-size"))
    ++leaves;
    if (of_property_present(np, "d-cache-size"))
    ++leaves;
    if (!leaves) {
// The '[i-|d-|]cache-size' property is required, but
// if absent, fallback on the 'cache-unified' property.
//
    if (of_property_read_bool(np, "cache-unified"))
    return 1;
    else
    return 2;
    }
    return leaves;
    }
#[no_mangle]
pub unsafe extern "C" fn init_of_cache_level(cpu: c_uint) -> c_int {
    int init_of_cache_level(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct device_node *np __free(device_node) = of_cpu_device_node_get(cpu);
    let mut levels: c_uint = 0, leaves, level;
    if (!of_check_cache_nodes(np)) {
    return -ENOENT;
    }
    leaves = of_count_cache_leaves(np);
    if (leaves > 0)
    levels = 1;
    while (1) {
    struct device_node *prev __free(device_node) = np;
    np = of_find_next_cache_node(np);
    if (!np)
    break;
    if (!of_device_is_compatible(np, "cache"))
    return -EINVAL;
    if (of_property_read_u32(np, "cache-level", &level))
    return -EINVAL;
    if (level <= levels)
    return -EINVAL;
    leaves += of_count_cache_leaves(np);
    levels = level;
    }
    this_cpu_ci.num_levels = levels;
    this_cpu_ci.num_leaves = leaves;
    return 0;
    }

    static inline int cache_setup_of_node(unsigned int cpu) { return 0; }
    int init_of_cache_level(unsigned int cpu) { return 0; }

#[no_mangle]
pub unsafe extern "C" fn cache_setup_acpi(cpu: c_uint) -> int __weak {
    int __weak cache_setup_acpi(unsigned int cpu)
    {
    return -ENOTSUPP;
    }
    unsigned int coherency_max_size;
#[no_mangle]
unsafe extern "C" fn cache_setup_properties(cpu: c_uint) -> c_int {
    static int cache_setup_properties(unsigned int cpu)
    {
    let mut ret: c_int = 0;
    if (of_have_populated_dt())
    ret = cache_setup_of_node(cpu);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !acpi_disabled) -> else {
    else if (!acpi_disabled)
    ret = cache_setup_acpi(cpu);
//
// No DT/ACPI cache nodes; fall back to arch-derived topology (e.g.
// arm64 CLIDR_EL1) and clear the error to avoid a spurious warning.
//
    if (ret && use_arch_cache_info()) {
    use_arch_info = true;
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cache_shared_cpu_map_setup(cpu: c_uint) -> c_int {
    static int cache_shared_cpu_map_setup(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *this_leaf, *sib_leaf;
    unsigned int index, sib_index;
    let mut ret: c_int = 0;
    if (this_cpu_ci.cpu_map_populated)
    return 0;
//
// skip setting up cache properties if LLC is valid, just need
// to update the shared cpu_map if the cache attributes were
// populated early before all the cpus are brought online
//
    if (!last_level_cache_is_valid(cpu) && !use_arch_info) {
    ret = cache_setup_properties(cpu);
    if (ret)
    return ret;
    }
    for (index = 0; index < cache_leaves(cpu); index++) {
    unsigned int i;
    this_leaf = per_cpu_cacheinfo_idx(cpu, index);
    cpumask_set_cpu(cpu, &this_leaf.shared_cpu_map);
    for_each_online_cpu(i) {
    if (i == cpu || !per_cpu_cacheinfo(i))
    continue;/* skip if itself or no cacheinfo */
    for (sib_index = 0; sib_index < cache_leaves(i); sib_index++) {
    sib_leaf = per_cpu_cacheinfo_idx(i, sib_index);
//
// Comparing cache IDs only makes sense if the leaves
// belong to the same cache level of same type. Skip
// the check if level and type do not match.
//
    if (sib_leaf.level != this_leaf.level ||
    sib_leaf.type != this_leaf.type)
    continue;
    if (cache_leaves_are_shared(this_leaf, sib_leaf)) {
    cpumask_set_cpu(cpu, &sib_leaf.shared_cpu_map);
    cpumask_set_cpu(i, &this_leaf.shared_cpu_map);
    break;
    }
    }
    }
// record the maximum cache line size
    if (this_leaf.coherency_line_size > coherency_max_size)
    coherency_max_size = this_leaf.coherency_line_size;
    }
// shared_cpu_map is now populated for the cpu
    this_cpu_ci.cpu_map_populated = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cache_shared_cpu_map_remove(cpu: c_uint) {
    static void cache_shared_cpu_map_remove(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *this_leaf, *sib_leaf;
    unsigned int sibling, index, sib_index;
    for (index = 0; index < cache_leaves(cpu); index++) {
    this_leaf = per_cpu_cacheinfo_idx(cpu, index);
    for_each_cpu(sibling, &this_leaf.shared_cpu_map) {
    if (sibling == cpu || !per_cpu_cacheinfo(sibling))
    continue;/* skip if itself or no cacheinfo */
    for (sib_index = 0; sib_index < cache_leaves(sibling); sib_index++) {
    sib_leaf = per_cpu_cacheinfo_idx(sibling, sib_index);
//
// Comparing cache IDs only makes sense if the leaves
// belong to the same cache level of same type. Skip
// the check if level and type do not match.
//
    if (sib_leaf.level != this_leaf.level ||
    sib_leaf.type != this_leaf.type)
    continue;
    if (cache_leaves_are_shared(this_leaf, sib_leaf)) {
    cpumask_clear_cpu(cpu, &sib_leaf.shared_cpu_map);
    cpumask_clear_cpu(sibling, &this_leaf.shared_cpu_map);
    break;
    }
    }
    }
    }
// cpu is no longer populated in the shared map
    this_cpu_ci.cpu_map_populated = false;
    }
#[no_mangle]
unsafe extern "C" fn free_cache_attributes(cpu: c_uint) {
    static void free_cache_attributes(unsigned int cpu)
    {
    if (!per_cpu_cacheinfo(cpu))
    return;
    cache_shared_cpu_map_remove(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn early_cache_level(cpu: c_uint) -> int __weak {
    int __weak early_cache_level(unsigned int cpu)
    {
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn init_cache_level(cpu: c_uint) -> int __weak {
    int __weak init_cache_level(unsigned int cpu)
    {
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cache_leaves(cpu: c_uint) -> int __weak {
    int __weak populate_cache_leaves(unsigned int cpu)
    {
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_cache_info(cpu: c_int) -> c_int {
    static inline int allocate_cache_info(int cpu)
    {
    per_cpu_cacheinfo(cpu) = kzalloc_objs(struct cacheinfo,
    cache_leaves(cpu), GFP_ATOMIC);
    if (!per_cpu_cacheinfo(cpu)) {
    cache_leaves(cpu) = 0;
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fetch_cache_info(cpu: c_uint) -> c_int {
    int fetch_cache_info(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut levels: c_uint = 0, split_levels = 0;
    int ret;
    if (acpi_disabled) {
    ret = init_of_cache_level(cpu);
    } else {
    ret = acpi_get_cache_info(cpu, &levels, &split_levels);
    if (!ret) {
    this_cpu_ci.num_levels = levels;
//
// This assumes that:
// - there cannot be any split caches (data/instruction)
// above a unified cache
// - data/instruction caches come by pair
//
    this_cpu_ci.num_leaves = levels + split_levels;
    }
    }
    if (ret || !cache_leaves(cpu)) {
    ret = early_cache_level(cpu);
    if (ret)
    return ret;
    if (!cache_leaves(cpu))
    return -ENOENT;
    this_cpu_ci.early_ci_levels = true;
    }
    return allocate_cache_info(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn init_level_allocate_ci(cpu: c_uint) -> c_int {
    static inline int init_level_allocate_ci(unsigned int cpu)
    {
    let mut early_leaves: c_uint = cache_leaves(cpu);
// Since early initialization/allocation of the cacheinfo is allowed
// via fetch_cache_info() and this also gets called as CPU hotplug
// callbacks via cacheinfo_cpu_online, the init/alloc can be skipped
// as it will happen only once (the cacheinfo memory is never freed).
// Just populate the cacheinfo. However, if the cacheinfo has been
// allocated early through the arch-specific early_cache_level() call,
// there is a chance the info is wrong (this can happen on arm64). In
// that case, call init_cache_level() anyway to give the arch-specific
// code a chance to make things right.
//
    if (per_cpu_cacheinfo(cpu) && !ci_cacheinfo(cpu).early_ci_levels)
    return 0;
    if (init_cache_level(cpu) || !cache_leaves(cpu))
    return -ENOENT;
//
// Now that we have properly initialized the cache level info, make
// sure we don't try to do that again the next time we are called
// (e.g. as CPU hotplug callbacks).
//
    ci_cacheinfo(cpu).early_ci_levels = false;
//
// Some architectures (e.g., x86) do not use early initialization.
// Allocate memory now in such case.
//
    if (cache_leaves(cpu) <= early_leaves && per_cpu_cacheinfo(cpu))
    return 0;
    kfree(per_cpu_cacheinfo(cpu));
    return allocate_cache_info(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn detect_cache_attributes(cpu: c_uint) -> c_int {
    int detect_cache_attributes(unsigned int cpu)
    {
    int ret;
    ret = init_level_allocate_ci(cpu);
    if (ret)
    return ret;
//
// If LLC is valid the cache leaves were already populated so just go to
// update the cpu map.
//
    if (!last_level_cache_is_valid(cpu)) {
//
// populate_cache_leaves() may completely setup the cache leaves and
// shared_cpu_map or it may leave it partially setup.
//
    ret = populate_cache_leaves(cpu);
    if (ret)
    goto free_ci;
    }
//
// For systems using DT for cache hierarchy, fw_token
// and shared_cpu_map will be set up here only if they are
// not populated already
//
    ret = cache_shared_cpu_map_setup(cpu);
    if (ret) {
    pr_warn("Unable to detect cache hierarchy for CPU %d\n", cpu);
    goto free_ci;
    }
    return 0;
    free_ci:
    free_cache_attributes(cpu);
    return ret;
    }
// pointer to cpuX/cache device
    static DEFINE_PER_CPU(struct device *, ci_cache_dev);

    static cpumask_t cache_dev_map;
// pointer to array of devices for cpuX/cache/indexY
    static DEFINE_PER_CPU(struct device **, ci_index_dev);

    static ssize_t file_name##_show(struct device *dev,		\
    struct device_attribute *attr, char *buf)	\
    {								\
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);	\
    return sysfs_emit(buf, "%u\n", this_leaf.object);	\
    }
    show_one(id, id);
    show_one(level, level);
    show_one(coherency_line_size, coherency_line_size);
    show_one(number_of_sets, number_of_sets);
    show_one(physical_line_partition, physical_line_partition);
    show_one(ways_of_associativity, ways_of_associativity);
    static ssize_t size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%uK\n", this_leaf.size >> 10);
    }
    static ssize_t shared_cpu_map_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    const struct cpumask *mask = &this_leaf.shared_cpu_map;
    return sysfs_emit(buf, "%*pb\n", nr_cpu_ids, mask);
    }
    static ssize_t shared_cpu_list_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    const struct cpumask *mask = &this_leaf.shared_cpu_map;
    return sysfs_emit(buf, "%*pbl\n", nr_cpu_ids, mask);
    }
    static ssize_t type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    const char *output;
    switch (this_leaf.type) {
    case CACHE_TYPE_DATA:
    output = "Data";
    break;
    case CACHE_TYPE_INST:
    output = "Instruction";
    break;
    case CACHE_TYPE_UNIFIED:
    output = "Unified";
    break;
    default:
    return -EINVAL;
    }
    return sysfs_emit(buf, "%s\n", output);
    }
    static ssize_t allocation_policy_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    let mut ci_attr: c_uint = this_leaf.attributes;
    const char *output;
    if ((ci_attr & CACHE_READ_ALLOCATE) && (ci_attr & CACHE_WRITE_ALLOCATE))
    output = "ReadWriteAllocate";
#[no_mangle]
pub unsafe extern "C" fn if(CACHE_READ_ALLOCATE: ci_attr &) -> else {
    else if (ci_attr & CACHE_READ_ALLOCATE)
    output = "ReadAllocate";
#[no_mangle]
pub unsafe extern "C" fn if(CACHE_WRITE_ALLOCATE: ci_attr &) -> else {
    else if (ci_attr & CACHE_WRITE_ALLOCATE)
    output = "WriteAllocate";
    else
    return 0;
    return sysfs_emit(buf, "%s\n", output);
    }
    static ssize_t write_policy_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    let mut ci_attr: c_uint = this_leaf.attributes;
    let mut n: c_int = 0;
    if (ci_attr & CACHE_WRITE_THROUGH)
    n = sysfs_emit(buf, "WriteThrough\n");
#[no_mangle]
pub unsafe extern "C" fn if(CACHE_WRITE_BACK: ci_attr &) -> else {
    else if (ci_attr & CACHE_WRITE_BACK)
    n = sysfs_emit(buf, "WriteBack\n");
    return n;
    }
    static DEVICE_ATTR_RO(id);
    static DEVICE_ATTR_RO(level);
    static DEVICE_ATTR_RO(type);
    static DEVICE_ATTR_RO(coherency_line_size);
    static DEVICE_ATTR_RO(ways_of_associativity);
    static DEVICE_ATTR_RO(number_of_sets);
    static DEVICE_ATTR_RO(size);
    static DEVICE_ATTR_RO(allocation_policy);
    static DEVICE_ATTR_RO(write_policy);
    static DEVICE_ATTR_RO(shared_cpu_map);
    static DEVICE_ATTR_RO(shared_cpu_list);
    static DEVICE_ATTR_RO(physical_line_partition);
    static struct attribute *cache_default_attrs[] = {
    &dev_attr_id.attr,
    &dev_attr_type.attr,
    &dev_attr_level.attr,
    &dev_attr_shared_cpu_map.attr,
    &dev_attr_shared_cpu_list.attr,
    &dev_attr_coherency_line_size.attr,
    &dev_attr_ways_of_associativity.attr,
    &dev_attr_number_of_sets.attr,
    &dev_attr_size.attr,
    &dev_attr_allocation_policy.attr,
    &dev_attr_write_policy.attr,
    &dev_attr_physical_line_partition.attr,
    core::ptr::null_mut()
    };
    static umode_t
    cache_default_attrs_is_visible(struct kobject *kobj,
    struct attribute *attr, int unused)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct cacheinfo *this_leaf = dev_get_drvdata(dev);
    const struct cpumask *mask = &this_leaf.shared_cpu_map;
    let mut mode: umode_t = attr.mode;
    if ((attr == &dev_attr_id.attr) && (this_leaf.attributes & CACHE_ID))
    return mode;
    if ((attr == &dev_attr_type.attr) && this_leaf.type)
    return mode;
    if ((attr == &dev_attr_level.attr) && this_leaf.level)
    return mode;
    if ((attr == &dev_attr_shared_cpu_map.attr) && !cpumask_empty(mask))
    return mode;
    if ((attr == &dev_attr_shared_cpu_list.attr) && !cpumask_empty(mask))
    return mode;
    if ((attr == &dev_attr_coherency_line_size.attr) &&
    this_leaf.coherency_line_size)
    return mode;
    if ((attr == &dev_attr_ways_of_associativity.attr) &&
    this_leaf.size) /* allow 0 = full associativity */
    return mode;
    if ((attr == &dev_attr_number_of_sets.attr) &&
    this_leaf.number_of_sets)
    return mode;
    if ((attr == &dev_attr_size.attr) && this_leaf.size)
    return mode;
    if ((attr == &dev_attr_write_policy.attr) &&
    (this_leaf.attributes & CACHE_WRITE_POLICY_MASK))
    return mode;
    if ((attr == &dev_attr_allocation_policy.attr) &&
    (this_leaf.attributes & CACHE_ALLOCATE_POLICY_MASK))
    return mode;
    if ((attr == &dev_attr_physical_line_partition.attr) &&
    this_leaf.physical_line_partition)
    return mode;
    return 0;
    }
    static const struct attribute_group cache_default_group = {
    .attrs = cache_default_attrs,
    .is_visible = cache_default_attrs_is_visible,
    };
    static const struct attribute_group *cache_default_groups[] = {
    &cache_default_group,
    core::ptr::null_mut(),
    };
    static const struct attribute_group *cache_private_groups[] = {
    &cache_default_group,
    core::ptr::null_mut(), /* Place holder for private group */
    core::ptr::null_mut(),
    };
    const struct attribute_group *
#[no_mangle]
pub unsafe extern "C" fn cache_get_priv_group(this_leaf: *mut cacheinfo) -> __weak {
    __weak cache_get_priv_group(struct cacheinfo *this_leaf)
    {
    return core::ptr::null_mut();
    }
    static const struct attribute_group **
    cache_get_attribute_groups(struct cacheinfo *this_leaf)
    {
    const struct attribute_group *priv_group =
    cache_get_priv_group(this_leaf);
    if (!priv_group)
    return cache_default_groups;
    if (!cache_private_groups[1])
    cache_private_groups[1] = priv_group;
    return cache_private_groups;
    }
// Add/Remove cache interface for CPU device
#[no_mangle]
unsafe extern "C" fn cpu_cache_sysfs_exit(cpu: c_uint) {
    static void cpu_cache_sysfs_exit(unsigned int cpu)
    {
    int i;
    struct device *ci_dev;
    if (per_cpu_index_dev(cpu)) {
    for (i = 0; i < cache_leaves(cpu); i++) {
    ci_dev = per_cache_index_dev(cpu, i);
    if (!ci_dev)
    continue;
    device_unregister(ci_dev);
    }
    kfree(per_cpu_index_dev(cpu));
    per_cpu_index_dev(cpu) = core::ptr::null_mut();
    }
    device_unregister(per_cpu_cache_dev(cpu));
    per_cpu_cache_dev(cpu) = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cpu_cache_sysfs_init(cpu: c_uint) -> c_int {
    static int cpu_cache_sysfs_init(unsigned int cpu)
    {
    struct device *dev = get_cpu_device(cpu);
    if (per_cpu_cacheinfo(cpu) == core::ptr::null_mut())
    return -ENOENT;
    per_cpu_cache_dev(cpu) = cpu_device_create(dev, core::ptr::null_mut(), core::ptr::null_mut(), "cache");
    if (IS_ERR(per_cpu_cache_dev(cpu)))
    return PTR_ERR(per_cpu_cache_dev(cpu));
// Allocate all required memory
    per_cpu_index_dev(cpu) = kzalloc_objs(struct device *,
    cache_leaves(cpu));
    if (unlikely(per_cpu_index_dev(cpu) == core::ptr::null_mut()))
    goto err_out;
    return 0;
    err_out:
    cpu_cache_sysfs_exit(cpu);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn cache_add_dev(cpu: c_uint) -> c_int {
    static int cache_add_dev(unsigned int cpu)
    {
    unsigned int i;
    int rc;
    struct device *ci_dev, *parent;
    struct cacheinfo *this_leaf;
    const struct attribute_group **cache_groups;
    rc = cpu_cache_sysfs_init(cpu);
    if (unlikely(rc < 0))
    return rc;
    parent = per_cpu_cache_dev(cpu);
    for (i = 0; i < cache_leaves(cpu); i++) {
    this_leaf = per_cpu_cacheinfo_idx(cpu, i);
    if (this_leaf.disable_sysfs)
    continue;
    if (this_leaf.type == CACHE_TYPE_NOCACHE)
    break;
    cache_groups = cache_get_attribute_groups(this_leaf);
    ci_dev = cpu_device_create(parent, this_leaf, cache_groups,
    "index%1u", i);
    if (IS_ERR(ci_dev)) {
    rc = PTR_ERR(ci_dev);
    goto err;
    }
    per_cache_index_dev(cpu, i) = ci_dev;
    }
    cpumask_set_cpu(cpu, &cache_dev_map);
    return 0;
    err:
    cpu_cache_sysfs_exit(cpu);
    return rc;
    }
    static unsigned int cpu_map_shared_cache(bool online, unsigned int cpu,
    cpumask_t **map)
    {
    struct cacheinfo *llc, *sib_llc;
    unsigned int sibling;
    if (!last_level_cache_is_valid(cpu))
    return 0;
    llc = per_cpu_cacheinfo_idx(cpu, cache_leaves(cpu) - 1);
    if (llc.type != CACHE_TYPE_DATA && llc.type != CACHE_TYPE_UNIFIED)
    return 0;
    if (online) {
// map = &llc->shared_cpu_map;
    return cpumask_weight(*map);
    }
// shared_cpu_map of offlined CPU will be cleared, so use sibling map
    for_each_cpu(sibling, &llc.shared_cpu_map) {
    if (sibling == cpu || !last_level_cache_is_valid(sibling))
    continue;
    sib_llc = per_cpu_cacheinfo_idx(sibling, cache_leaves(sibling) - 1);
// map = &sib_llc->shared_cpu_map;
    return cpumask_weight(*map);
    }
    return 0;
    }
//
// Calculate the size of the per-CPU data cache slice.  This can be
// used to estimate the size of the data cache slice that can be used
// by one CPU under ideal circumstances.  UNIFIED caches are counted
// in addition to DATA caches.  So, please consider code cache usage
// when use the result.
//
// Because the cache inclusive/non-inclusive information isn't
// available, we just use the size of the per-CPU slice of LLC to make
// the result more predictable across architectures.
//
#[no_mangle]
unsafe extern "C" fn update_per_cpu_data_slice_size_cpu(cpu: c_uint) {
    static void update_per_cpu_data_slice_size_cpu(unsigned int cpu)
    {
    struct cpu_cacheinfo *ci;
    struct cacheinfo *llc;
    unsigned int nr_shared;
    if (!last_level_cache_is_valid(cpu))
    return;
    ci = ci_cacheinfo(cpu);
    llc = per_cpu_cacheinfo_idx(cpu, cache_leaves(cpu) - 1);
    if (llc.type != CACHE_TYPE_DATA && llc.type != CACHE_TYPE_UNIFIED)
    return;
    nr_shared = cpumask_weight(&llc.shared_cpu_map);
    if (nr_shared)
    ci.per_cpu_data_slice_size = llc.size / nr_shared;
    }
    static void update_per_cpu_data_slice_size(bool cpu_online, unsigned int cpu,
    cpumask_t *cpu_map)
    {
    unsigned int icpu;
    for_each_cpu(icpu, cpu_map) {
    if (!cpu_online && icpu == cpu)
    continue;
    update_per_cpu_data_slice_size_cpu(icpu);
    setup_pcp_cacheinfo(icpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn cacheinfo_cpu_online(cpu: c_uint) -> c_int {
    static int cacheinfo_cpu_online(unsigned int cpu)
    {
    let mut rc: c_int = detect_cache_attributes(cpu);
    cpumask_t *cpu_map;
    if (rc)
    return rc;
    rc = cache_add_dev(cpu);
    if (rc)
    goto err;
    if (cpu_map_shared_cache(true, cpu, &cpu_map))
    update_per_cpu_data_slice_size(true, cpu, cpu_map);
    sched_update_llc_bytes(cpu);
    return 0;
    err:
    free_cache_attributes(cpu);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cacheinfo_cpu_pre_down(cpu: c_uint) -> c_int {
    static int cacheinfo_cpu_pre_down(unsigned int cpu)
    {
    cpumask_t *cpu_map;
    unsigned int nr_shared;
    nr_shared = cpu_map_shared_cache(false, cpu, &cpu_map);
    if (cpumask_test_and_clear_cpu(cpu, &cache_dev_map))
    cpu_cache_sysfs_exit(cpu);
    free_cache_attributes(cpu);
    if (nr_shared > 1)
    update_per_cpu_data_slice_size(false, cpu, cpu_map);
    sched_update_llc_bytes(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cacheinfo_sysfs_init() -> int __init {
    static int __init cacheinfo_sysfs_init(void)
    {
    return cpuhp_setup_state(CPUHP_AP_BASE_CACHEINFO_ONLINE,
    "base/cacheinfo:online",
    cacheinfo_cpu_online, cacheinfo_cpu_pre_down);
    }
    device_initcall(cacheinfo_sysfs_init);
