//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/numa.c
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
// Author:  Xiang Gao <gaoxiang@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    int numa_off;
    cpumask_t cpus_on_node[MAX_NUMNODES];
    cpumask_t phys_cpus_on_node[MAX_NUMNODES];
    EXPORT_SYMBOL(cpus_on_node);
//
// apicid, cpu, node mappings
//
    s16 __cpuid_to_node[CONFIG_NR_CPUS] = {
    [0 ... CONFIG_NR_CPUS - 1] = NUMA_NO_NODE
    };
    EXPORT_SYMBOL(__cpuid_to_node);

    unsigned long __per_cpu_offset[NR_CPUS] __read_mostly;
    EXPORT_SYMBOL(__per_cpu_offset);
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_to_node(cpu: c_int) -> int __init {
    static int __init pcpu_cpu_to_node(int cpu)
    {
    return early_cpu_to_node(cpu);
    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> int __init {
    static int __init pcpu_cpu_distance(unsigned int from, unsigned int to)
    {
    if (early_cpu_to_node(from) == early_cpu_to_node(to))
    return LOCAL_DISTANCE;
    else
    return REMOTE_DISTANCE;
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_populate_pte(addr: c_ulong) -> void __init {
    void __init pcpu_populate_pte(unsigned long addr)
    {
    populate_kernel_pte(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas() -> void __init {
    void __init setup_per_cpu_areas(void)
    {
    unsigned long delta;
    unsigned int cpu;
    let mut rc: c_int = -EINVAL;
    if (pcpu_chosen_fc == PCPU_FC_AUTO) {
    if (nr_node_ids >= 8)
    pcpu_chosen_fc = PCPU_FC_PAGE;
    else
    pcpu_chosen_fc = PCPU_FC_EMBED;
    }
//
// Always reserve area for module percpu variables.  That's
// what the legacy allocator did.
//
    if (pcpu_chosen_fc != PCPU_FC_PAGE) {
    rc = pcpu_embed_first_chunk(PERCPU_MODULE_RESERVE,
    PERCPU_DYNAMIC_RESERVE, PMD_SIZE,
    pcpu_cpu_distance, pcpu_cpu_to_node);
    if (rc < 0)
    pr_warn("%s allocator failed (%d), falling back to page size\n",
    pcpu_fc_names[pcpu_chosen_fc], rc);
    }
    if (rc < 0)
    rc = pcpu_page_first_chunk(PERCPU_MODULE_RESERVE, pcpu_cpu_to_node);
    if (rc < 0)
    panic("cannot initialize percpu area (err=%d)", rc);
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu)
    __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu];
    }

//
// Get nodeid by logical cpu number.
// __cpuid_to_node maps phyical cpu id to node, so we
// should use cpu_logical_map(cpu) to index it.
//
// This routine is only used in early phase during
// booting, after setup_per_cpu_areas calling and numa_node
// initialization, cpu_to_node will be used instead.
//
#[no_mangle]
pub unsafe extern "C" fn early_cpu_to_node(cpu: c_int) -> c_int {
    int early_cpu_to_node(int cpu)
    {
    let mut physid: c_int = cpu_logical_map(cpu);
    if (physid < 0)
    return NUMA_NO_NODE;
    return __cpuid_to_node[physid];
    }
#[no_mangle]
pub unsafe extern "C" fn early_numa_add_cpu(cpuid: c_int, node: i16) -> void __init {
    void __init early_numa_add_cpu(int cpuid, s16 node)
    {
    let mut cpu: c_int = __cpu_number_map[cpuid];
    if (cpu < 0)
    return;
    cpumask_set_cpu(cpu, &cpus_on_node[node]);
    cpumask_set_cpu(cpuid, &phys_cpus_on_node[node]);
    }
#[no_mangle]
pub unsafe extern "C" fn numa_add_cpu(cpu: c_uint) {
    void numa_add_cpu(unsigned int cpu)
    {
    let mut nid: c_int = cpu_to_node(cpu);
    cpumask_set_cpu(cpu, &cpus_on_node[nid]);
    }
#[no_mangle]
pub unsafe extern "C" fn numa_remove_cpu(cpu: c_uint) {
    void numa_remove_cpu(unsigned int cpu)
    {
    let mut nid: c_int = cpu_to_node(cpu);
    cpumask_clear_cpu(cpu, &cpus_on_node[nid]);
    }
#[no_mangle]
unsafe extern "C" fn node_mem_init(node: c_uint) -> void __init {
    static void __init node_mem_init(unsigned int node)
    {
    unsigned long start_pfn, end_pfn;
    unsigned long node_addrspace_offset;
    node_addrspace_offset = nid_to_addrbase(node);
    pr_info("Node%d's addrspace_offset is 0x%lx\n",
    node, node_addrspace_offset);
    get_pfn_range_for_nid(node, &start_pfn, &end_pfn);
    pr_info("Node%d: start_pfn=0x%lx, end_pfn=0x%lx\n",
    node, start_pfn, end_pfn);
    alloc_node_data(node);
    }

    static unsigned long num_physpages;
#[no_mangle]
unsafe extern "C" fn info_node_memblock() -> void __init {
    static void __init info_node_memblock(void)
    {
    u32 mem_type;
    u64 mem_end, mem_start, mem_size;
    efi_memory_desc_t *md;
// Parse memory information and activate
    for_each_efi_memory_desc(md) {
    mem_type = md.type;
    mem_start = md.phys_addr;
    mem_size = md.num_pages << EFI_PAGE_SHIFT;
    mem_end = mem_start + mem_size;
    switch (mem_type) {
    case EFI_LOADER_CODE:
    case EFI_LOADER_DATA:
    case EFI_BOOT_SERVICES_CODE:
    case EFI_BOOT_SERVICES_DATA:
    case EFI_PERSISTENT_MEMORY:
    case EFI_CONVENTIONAL_MEMORY:
    num_physpages += (mem_size >> PAGE_SHIFT);
    pr_info("Node%d: mem_type:%d, mem_start:0x%llx, mem_size:0x%llx Bytes\n",
    (u32)pa_to_nid(mem_start), mem_type, mem_start, mem_size);
    pr_info("       start_pfn:0x%llx, end_pfn:0x%llx, num_physpages:0x%lx\n",
    mem_start >> PAGE_SHIFT, mem_end >> PAGE_SHIFT, num_physpages);
    break;
    case EFI_PAL_CODE:
    case EFI_UNUSABLE_MEMORY:
    case EFI_ACPI_RECLAIM_MEMORY:
    num_physpages += (mem_size >> PAGE_SHIFT);
    pr_info("Node%d: mem_type:%d, mem_start:0x%llx, mem_size:0x%llx Bytes\n",
    (u32)pa_to_nid(mem_start), mem_type, mem_start, mem_size);
    pr_info("       start_pfn:0x%llx, end_pfn:0x%llx, num_physpages:0x%lx\n",
    mem_start >> PAGE_SHIFT, mem_end >> PAGE_SHIFT, num_physpages);
    fallthrough;
    case EFI_RESERVED_TYPE:
    case EFI_RUNTIME_SERVICES_CODE:
    case EFI_RUNTIME_SERVICES_DATA:
    case EFI_MEMORY_MAPPED_IO:
    case EFI_MEMORY_MAPPED_IO_PORT_SPACE:
    pr_info("Resvd: mem_type:%d, mem_start:0x%llx, mem_size:0x%llx Bytes\n",
    mem_type, mem_start, mem_size);
    break;
    }
    }
    }
//
// fake_numa_init() - For Non-ACPI systems
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
unsafe extern "C" fn fake_numa_init() -> int __init {
    static int __init fake_numa_init(void)
    {
    let mut start: phys_addr_t = memblock_start_of_DRAM();
    let mut end: phys_addr_t = memblock_end_of_DRAM() - 1;
    pr_info("Faking a node at [mem %pap-%pap]\n", &start, &end);
    return numa_add_memblk(0, start, end + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn init_numa_memory() -> int __init {
    int __init init_numa_memory(void)
    {
    int i;
    int ret;
    int node;
    for (i = 0; i < NR_CPUS; i++)
    set_cpuid_to_node(i, NUMA_NO_NODE);
// Parse SRAT and SLIT if provided by firmware.
    if (!acpi_disabled)
    ret = numa_memblks_init(acpi_numa_init, false);
    else
    ret = numa_memblks_init(fake_numa_init, false);
    if (ret < 0)
    return ret;
    info_node_memblock();
    if (!memblock_validate_numa_coverage(SZ_1M))
    return -EINVAL;
    for_each_node_mask(node, node_possible_map) {
    node_mem_init(node);
    node_set_online(node);
    }
    max_pfn = PFN_DOWN(memblock_end_of_DRAM());
    max_low_pfn = min(PFN_DOWN(HIGHMEM_START), max_pfn);
    setup_nr_node_ids();
    loongson_sysconf.nr_nodes = nr_node_ids;
    loongson_sysconf.cores_per_node = cpumask_weight(&phys_cpus_on_node[0]);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn pcibus_to_node(bus: *mut pci_bus) -> c_int {
    int pcibus_to_node(struct pci_bus *bus)
    {
    return dev_to_node(&bus.dev);
    }
    EXPORT_SYMBOL(pcibus_to_node);
