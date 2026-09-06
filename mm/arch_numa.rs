//! Automatically rewritten from C to Rust
//! Source: mm/arch_numa.c
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
// NUMA support, based on the x86 implementation.
//
// Copyright (C) 2015 Cavium Inc.
// Author: Ganapatrao Kulkarni <gkulkarni@cavium.com>
//

    static int cpu_to_node_map[NR_CPUS] = { [0 ... NR_CPUS-1] = NUMA_NO_NODE };
    bool numa_off;
#[no_mangle]
unsafe extern "C" fn numa_parse_early_param(opt: *mut c_char) -> __init int {
    static __init int numa_parse_early_param(char *opt)
    {
    if (!opt)
    return -EINVAL;
    if (str_has_prefix(opt, "off"))
    numa_off = true;
    if (!strncmp(opt, "fake=", 5))
    return numa_emu_cmdline(opt + 5);
    return 0;
    }
    early_param("numa", numa_parse_early_param);
    cpumask_var_t node_to_cpumask_map[MAX_NUMNODES];
    EXPORT_SYMBOL(node_to_cpumask_map);

//
// Returns a pointer to the bitmask of CPUs on Node 'node'.
//
    const struct cpumask *cpumask_of_node(int node)
    {
    if (node == NUMA_NO_NODE)
    return cpu_all_mask;
    if (WARN_ON(node < 0 || node >= nr_node_ids))
    return cpu_none_mask;
    if (WARN_ON(node_to_cpumask_map[node] == core::ptr::null_mut()))
    return cpu_online_mask;
    return node_to_cpumask_map[node];
    }
    EXPORT_SYMBOL(cpumask_of_node);

#[no_mangle]
unsafe extern "C" fn numa_update_cpu(cpu: c_uint, remove: bool) {
    static void numa_update_cpu(unsigned int cpu, bool remove)
    {
    let mut nid: c_int = cpu_to_node(cpu);
    if (nid == NUMA_NO_NODE)
    return;
    if (remove)
    cpumask_clear_cpu(cpu, node_to_cpumask_map[nid]);
    else
    cpumask_set_cpu(cpu, node_to_cpumask_map[nid]);
    }
#[no_mangle]
pub unsafe extern "C" fn numa_add_cpu(cpu: c_uint) {
    void numa_add_cpu(unsigned int cpu)
    {
    numa_update_cpu(cpu, false);
    }
#[no_mangle]
pub unsafe extern "C" fn numa_remove_cpu(cpu: c_uint) {
    void numa_remove_cpu(unsigned int cpu)
    {
    numa_update_cpu(cpu, true);
    }

#[no_mangle]
pub unsafe extern "C" fn numa_clear_node(cpu: c_uint) {
    void numa_clear_node(unsigned int cpu)
    {
    numa_remove_cpu(cpu);
    set_cpu_numa_node(cpu, NUMA_NO_NODE);
    }
//
// Allocate node_to_cpumask_map based on number of available nodes
// Requires node_possible_map to be valid.
//
// Note: cpumask_of_node() is not valid until after this is done.
// (Use CONFIG_DEBUG_PER_CPU_MAPS to check this.)
//
#[no_mangle]
unsafe extern "C" fn setup_node_to_cpumask_map() -> void __init {
    static void __init setup_node_to_cpumask_map(void)
    {
    int node;
// setup nr_node_ids if not done yet
    if (nr_node_ids == MAX_NUMNODES)
    setup_nr_node_ids();
//
// This check should never be true but it makes it clear to compilers
// that node_to_cpumask_map is bound by nr_node_ids, avoiding false
// positive fortify warnings when accessing node_to_cpumask_map in the
// for loop below.
//
    if (unlikely(nr_node_ids > MAX_NUMNODES)) {
    pr_err("nr_node_ids (%u) is larger than MAX_NUMNODES (%u)\n",
    nr_node_ids, MAX_NUMNODES);
    return;
    }
// allocate and clear the mapping
    for (node = 0; node < nr_node_ids; node++) {
    alloc_bootmem_cpumask_var(&node_to_cpumask_map[node]);
    cpumask_clear(node_to_cpumask_map[node]);
    }
// cpumask_of_node() will now work
    pr_debug("Node to cpumask map for %u nodes\n", nr_node_ids);
    }
//
// Set the cpu to node and mem mapping
//
#[no_mangle]
pub unsafe extern "C" fn numa_store_cpu_info(cpu: c_uint) {
    void numa_store_cpu_info(unsigned int cpu)
    {
    set_cpu_numa_node(cpu, cpu_to_node_map[cpu]);
    }
#[no_mangle]
pub unsafe extern "C" fn early_map_cpu_to_node(cpu: c_uint, nid: c_int) -> void __init {
    void __init early_map_cpu_to_node(unsigned int cpu, int nid)
    {
// fallback to node 0
    if (nid < 0 || nid >= MAX_NUMNODES || numa_off)
    nid = 0;
    cpu_to_node_map[cpu] = nid;
//
// We should set the numa node of cpu0 as soon as possible, because it
// has already been set up online before. cpu_to_node(0) will soon be
// called.
//
    if (!cpu)
    set_cpu_numa_node(cpu, nid);
    }

    unsigned long __per_cpu_offset[NR_CPUS] __read_mostly;
    EXPORT_SYMBOL(__per_cpu_offset);
#[no_mangle]
pub unsafe extern "C" fn early_cpu_to_node(cpu: c_int) -> c_int {
    int early_cpu_to_node(int cpu)
    {
    return cpu_to_node_map[cpu];
    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> int __init {
    static int __init pcpu_cpu_distance(unsigned int from, unsigned int to)
    {
    return node_distance(early_cpu_to_node(from), early_cpu_to_node(to));
    }
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas() -> void __init {
    void __init setup_per_cpu_areas(void)
    {
    unsigned long delta;
    unsigned int cpu;
    let mut rc: c_int = -EINVAL;
    if (pcpu_chosen_fc != PCPU_FC_PAGE) {
//
// Always reserve area for module percpu variables.  That's
// what the legacy allocator did.
//
    rc = pcpu_embed_first_chunk(PERCPU_MODULE_RESERVE,
    PERCPU_DYNAMIC_RESERVE, PAGE_SIZE,
    pcpu_cpu_distance,
    early_cpu_to_node);

    if (rc < 0)
    pr_warn("PERCPU: %s allocator failed (%d), falling back to page size\n",
    pcpu_fc_names[pcpu_chosen_fc], rc);

    }

    if (rc < 0)
    rc = pcpu_page_first_chunk(PERCPU_MODULE_RESERVE, early_cpu_to_node);

    if (rc < 0)
    panic("Failed to initialize percpu areas (err=%d).", rc);
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu)
    __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu];
    }

//
// Initialize NODE_DATA for a node on the local memory
//
#[no_mangle]
unsafe extern "C" fn setup_node_data(nid: c_int, start_pfn: u64, end_pfn: u64) -> void __init {
    static void __init setup_node_data(int nid, u64 start_pfn, u64 end_pfn)
    {
    if (start_pfn >= end_pfn)
    pr_info("Initmem setup node %d [<memory-less node>]\n", nid);
    alloc_node_data(nid);
    NODE_DATA(nid).node_id = nid;
    NODE_DATA(nid).node_start_pfn = start_pfn;
    NODE_DATA(nid).node_spanned_pages = end_pfn - start_pfn;
    }
#[no_mangle]
unsafe extern "C" fn numa_register_nodes() -> int __init {
    static int __init numa_register_nodes(void)
    {
    int nid;
// Check the validity of the memblock/node mapping
    if (!memblock_validate_numa_coverage(0))
    return -EINVAL;
// Finally register nodes.
    for_each_node_mask(nid, numa_nodes_parsed) {
    unsigned long start_pfn, end_pfn;
    get_pfn_range_for_nid(nid, &start_pfn, &end_pfn);
    setup_node_data(nid, start_pfn, end_pfn);
    node_set_online(nid);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn numa_init((*init_func)(void): *mut c_int) -> int __init {
    static int __init numa_init(int (*init_func)(void))
    {
    int ret;
    ret = numa_memblks_init(init_func, /* memblock_force_top_down */ false);
    if (ret < 0)
    goto out_free_distance;
    if (nodes_empty(numa_nodes_parsed)) {
    pr_info("No NUMA configuration found\n");
    ret = -EINVAL;
    goto out_free_distance;
    }
    ret = numa_register_nodes();
    if (ret < 0)
    goto out_free_distance;
    setup_node_to_cpumask_map();
    return 0;
    out_free_distance:
    numa_reset_distance();
    return ret;
    }
//
// dummy_numa_init() - Fallback dummy NUMA init
//
// Used if there's no underlying NUMA architecture, NUMA initialization
// fails, or NUMA is disabled on the command line.
//
// Must online at least one node (node 0) and add memory blocks that cover all
// allowed memory. It is unlikely that this function fails.
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
unsafe extern "C" fn dummy_numa_init() -> int __init {
    static int __init dummy_numa_init(void)
    {
    let mut start: phys_addr_t = memblock_start_of_DRAM();
    let mut end: phys_addr_t = memblock_end_of_DRAM() - 1;
    int ret;
    if (numa_off)
    pr_info("NUMA disabled\n"); /* Forced off on command line. */
    pr_info("Faking a node at [mem %pap-%pap]\n", &start, &end);
    ret = numa_add_memblk(0, start, end + 1);
    if (ret) {
    pr_err("NUMA init failed\n");
    return ret;
    }
    numa_off = true;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn arch_acpi_numa_init() -> int __init {
    static int __init arch_acpi_numa_init(void)
    {
    int ret;
    ret = acpi_numa_init();
    if (ret) {
    pr_debug("Failed to initialise from firmware\n");
    return ret;
    }
    return srat_disabled() ? -EINVAL : 0;
    }

#[no_mangle]
unsafe extern "C" fn arch_acpi_numa_init() -> int __init {
    static int __init arch_acpi_numa_init(void)
    {
    return -EOPNOTSUPP;
    }

//
// arch_numa_init() - Initialize NUMA
//
// Try each configured NUMA initialization method until one succeeds. The
// last fallback is dummy single node config encompassing whole memory.
//
#[no_mangle]
pub unsafe extern "C" fn arch_numa_init() -> void __init {
    void __init arch_numa_init(void)
    {
    if (!numa_off) {
    if (!acpi_disabled && !numa_init(arch_acpi_numa_init))
    return;
    if (acpi_disabled && !numa_init(of_numa_init))
    return;
    }
    numa_init(dummy_numa_init);
    }

    void __init numa_emu_update_cpu_to_node(int *emu_nid_to_phys,
    unsigned int nr_emu_nids)
    {
    int i, j;
//
// Transform cpu_to_node_map table to use emulated nids by
// reverse-mapping phys_nid.  The maps should always exist but fall
// back to zero just in case.
//
    for (i = 0; i < ARRAY_SIZE(cpu_to_node_map); i++) {
    if (cpu_to_node_map[i] == NUMA_NO_NODE)
    continue;
    for (j = 0; j < nr_emu_nids; j++)
    if (cpu_to_node_map[i] == emu_nid_to_phys[j])
    break;
    cpu_to_node_map[i] = j < nr_emu_nids ? j : 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn numa_emu_dma_end() -> u64 __init {
    u64 __init numa_emu_dma_end(void)
    {
    return memblock_start_of_DRAM() + SZ_4G;
    }
#[no_mangle]
pub unsafe extern "C" fn debug_cpumask_set_cpu(cpu: c_uint, node: c_int, enable: bool) {
    void debug_cpumask_set_cpu(unsigned int cpu, int node, bool enable)
    {
    struct cpumask *mask;
    if (node == NUMA_NO_NODE)
    return;
    mask = node_to_cpumask_map[node];
    if (!cpumask_available(mask)) {
    pr_err("node_to_cpumask_map[%i] core::ptr::null_mut()\n", node);
    dump_stack();
    return;
    }
    if (enable)
    cpumask_set_cpu(cpu, mask);
    else
    cpumask_clear_cpu(cpu, mask);
    pr_debug("%s cpu %d node %d: mask now %*pbl\n",
    enable ? "numa_add_cpu" : "numa_remove_cpu",
    cpu, node, cpumask_pr_args(mask));
    }
