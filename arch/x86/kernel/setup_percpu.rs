//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/setup_percpu.c
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

    DEFINE_PER_CPU_CACHE_HOT(int, cpu_number);
    EXPORT_PER_CPU_SYMBOL(cpu_number);
    DEFINE_PER_CPU_CACHE_HOT(unsigned long, this_cpu_off);
    EXPORT_PER_CPU_SYMBOL(this_cpu_off);
    unsigned long __per_cpu_offset[NR_CPUS] __ro_after_init;
    EXPORT_SYMBOL(__per_cpu_offset);
//
// On x86_64 symbols referenced from code should be reachable using
// 32bit relocations.  Reserve space for static percpu variables in
// modules so that they are always served from the first chunk which
// is located at the percpu segment base.  On x86_32, anything can
// address anywhere.  No need to reserve space in the first chunk.
//

pub const PERCPU_FIRST_CHUNK_RESERVE: c_int = 0;

//
// pcpu_need_numa - determine percpu allocation needs to consider NUMA
//
// If NUMA is not configured or there is only one NUMA node available,
// there is no reason to consider NUMA.  This function determines
// whether percpu allocation should consider NUMA or not.
//
// RETURNS:
// true if NUMA should be considered; otherwise, false.
//
#[no_mangle]
unsafe extern "C" fn pcpu_need_numa() -> bool __init {
    static bool __init pcpu_need_numa(void)
    {

    pg_data_t *last = core::ptr::null_mut();
    unsigned int cpu;
    for_each_possible_cpu(cpu) {
    let mut node: c_int = early_cpu_to_node(cpu);
    if (node_online(node) && NODE_DATA(node) &&
    last && last != NODE_DATA(node))
    return true;
    last = NODE_DATA(node);
    }

    return false;
    }

#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> int __init {
    static int __init pcpu_cpu_distance(unsigned int from, unsigned int to)
    {

    if (early_cpu_to_node(from) == early_cpu_to_node(to))
    return LOCAL_DISTANCE;
    else
    return REMOTE_DISTANCE;

    return LOCAL_DISTANCE;

    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_to_node(cpu: c_int) -> int __init {
    static int __init pcpu_cpu_to_node(int cpu)
    {
    return early_cpu_to_node(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_populate_pte(addr: c_ulong) -> void __init {
    void __init pcpu_populate_pte(unsigned long addr)
    {
    populate_extra_pte(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_percpu_segment(cpu: c_int) {
    static inline void setup_percpu_segment(int cpu)
    {

    struct desc_struct d = GDT_ENTRY_INIT(DESC_DATA32,
    per_cpu_offset(cpu), 0xFFFFF);
    write_gdt_entry(get_cpu_gdt_rw(cpu), GDT_ENTRY_PERCPU, &d, DESCTYPE_S);

    }
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas() -> void __init {
    void __init setup_per_cpu_areas(void)
    {
    unsigned int cpu;
    unsigned long delta;
    int rc;
    pr_info("NR_CPUS:%d nr_cpumask_bits:%d nr_cpu_ids:%u nr_node_ids:%u\n",
    NR_CPUS, nr_cpumask_bits, nr_cpu_ids, nr_node_ids);
//
// Allocate percpu area.  Embedding allocator is our favorite;
// however, on NUMA configurations, it can result in very
// sparse unit mapping and vmalloc area isn't spacious enough
// on 32bit.  Use page in that case.
//

    if (pcpu_chosen_fc == PCPU_FC_AUTO && pcpu_need_numa())
    pcpu_chosen_fc = PCPU_FC_PAGE;

    rc = -EINVAL;
    if (pcpu_chosen_fc != PCPU_FC_PAGE) {
    const size_t dyn_size = PERCPU_MODULE_RESERVE +
    PERCPU_DYNAMIC_RESERVE - PERCPU_FIRST_CHUNK_RESERVE;
    size_t atom_size;
//
// On 64bit, use PMD_SIZE for atom_size so that embedded
// percpu areas are aligned to PMD.  This, in the future,
// can also allow using PMD mappings in vmalloc area.  Use
// PAGE_SIZE on 32bit as vmalloc space is highly contended
// and large vmalloc area allocs can easily fail.
//

    atom_size = PMD_SIZE;

    atom_size = PAGE_SIZE;

    rc = pcpu_embed_first_chunk(PERCPU_FIRST_CHUNK_RESERVE,
    dyn_size, atom_size,
    pcpu_cpu_distance,
    pcpu_cpu_to_node);
    if (rc < 0)
    pr_warn("%s allocator failed (%d), falling back to page size\n",
    pcpu_fc_names[pcpu_chosen_fc], rc);
    }
    if (rc < 0)
    rc = pcpu_page_first_chunk(PERCPU_FIRST_CHUNK_RESERVE,
    pcpu_cpu_to_node);
    if (rc < 0)
    panic("cannot initialize percpu area (err=%d)", rc);
// alrighty, percpu areas up and running
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu) {
    per_cpu_offset(cpu) = delta + pcpu_unit_offsets[cpu];
    per_cpu(this_cpu_off, cpu) = per_cpu_offset(cpu);
    per_cpu(cpu_number, cpu) = cpu;
    setup_percpu_segment(cpu);
//
// Copy data used in early init routines from the
// initial arrays to the per cpu data areas.  These
// arrays then become expendable and the *_early_ptr's
// are zeroed indicating that the static arrays are
// gone.
//

    per_cpu(x86_cpu_to_apicid, cpu) =
    early_per_cpu_map(x86_cpu_to_apicid, cpu);
    per_cpu(x86_cpu_to_acpiid, cpu) =
    early_per_cpu_map(x86_cpu_to_acpiid, cpu);

    per_cpu(x86_cpu_to_node_map, cpu) =
    early_per_cpu_map(x86_cpu_to_node_map, cpu);
//
// Ensure that the boot cpu numa_node is correct when the boot
// cpu is on a node that doesn't have memory installed.
// Also cpu_up() will call cpu_to_node() for APs when
// MEMORY_HOTPLUG is defined, before per_cpu(numa_node) is set
// up later with c_init aka intel_init/amd_init.
// So set them all (boot cpu and all APs).
//
    set_cpu_numa_node(cpu, early_cpu_to_node(cpu));

//
// Up to this point, the boot CPU has been using .init.data
// area.  Reload any changed state for the boot CPU.
//
    if (!cpu)
    switch_gdt_and_percpu_base(cpu);
    }
// indicate the early static arrays will soon be gone

    early_per_cpu_ptr(x86_cpu_to_apicid) = core::ptr::null_mut();
    early_per_cpu_ptr(x86_cpu_to_acpiid) = core::ptr::null_mut();

    early_per_cpu_ptr(x86_cpu_to_node_map) = core::ptr::null_mut();

// Setup node to cpumask map
    setup_node_to_cpumask_map();
// Setup cpu initialized, callin, callout masks
    setup_cpu_local_masks();
//
// Sync back kernel address range again.  We already did this in
// setup_arch(), but percpu data also needs to be available in
// the smpboot asm and arch_sync_kernel_mappings() doesn't sync to
// swapper_pg_dir on 32-bit. The per-cpu mappings need to be available
// there too.
//
// FIXME: Can the later sync in setup_cpu_entry_areas() replace
// this call?
//
    sync_initial_page_table();
    }
