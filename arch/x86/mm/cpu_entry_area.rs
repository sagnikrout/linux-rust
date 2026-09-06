//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/cpu_entry_area.c
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

    static DEFINE_PER_CPU_PAGE_ALIGNED(struct entry_stack_page, entry_stack_storage);

    static DEFINE_PER_CPU_PAGE_ALIGNED(struct exception_stacks, exception_stacks);
    DEFINE_PER_CPU(struct cea_exception_stacks*, cea_exception_stacks);
    static DEFINE_PER_CPU_READ_MOSTLY(unsigned long, _cea_offset);
#[no_mangle]
unsafe extern "C" fn cea_offset(cpu: c_uint) -> __always_inline unsigned int {
    static __always_inline unsigned int cea_offset(unsigned int cpu)
    {
    return per_cpu(_cea_offset, cpu);
    }
#[no_mangle]
unsafe extern "C" fn init_cea_offsets() -> __init void {
    static __init void init_cea_offsets(void)
    {
    unsigned int max_cea;
    unsigned int i, j;
    if (!kaslr_enabled()) {
    for_each_possible_cpu(i)
    per_cpu(_cea_offset, i) = i;
    return;
    }
    max_cea = (CPU_ENTRY_AREA_MAP_SIZE - PAGE_SIZE) / CPU_ENTRY_AREA_SIZE;
// O(sodding terrible)
    for_each_possible_cpu(i) {
    unsigned int cea;
    again:
    cea = get_random_u32_below(max_cea);
    for_each_possible_cpu(j) {
    if (cea_offset(j) == cea)
    goto again;
    if (i == j)
    break;
    }
    per_cpu(_cea_offset, i) = cea;
    }
    }

    DECLARE_PER_CPU_PAGE_ALIGNED(struct doublefault_stack, doublefault_stack);
#[no_mangle]
unsafe extern "C" fn cea_offset(cpu: c_uint) -> __always_inline unsigned int {
    static __always_inline unsigned int cea_offset(unsigned int cpu)
    {
    return cpu;
    }
    static inline void init_cea_offsets(void) { }

// Is called from entry code, so must be noinstr
    noinstr struct cpu_entry_area *get_cpu_entry_area(int cpu)
    {
    let mut va: c_ulong = CPU_ENTRY_AREA_PER_CPU + cea_offset(cpu) * CPU_ENTRY_AREA_SIZE;
    BUILD_BUG_ON(sizeof(struct cpu_entry_area) % PAGE_SIZE != 0);
    return (struct cpu_entry_area *) va;
    }
    EXPORT_SYMBOL(get_cpu_entry_area);
#[no_mangle]
pub unsafe extern "C" fn cea_set_pte(cea_vaddr: *mut c_void, pa: phys_addr_t, flags: pgprot_t) {
    void cea_set_pte(void *cea_vaddr, phys_addr_t pa, pgprot_t flags)
    {
    let mut va: c_ulong = (unsigned long) cea_vaddr;
    let mut pte: pte_t = pfn_pte(pa >> PAGE_SHIFT, flags);
//
// The cpu_entry_area is shared between the user and kernel
// page tables.  All of its ptes can safely be global.
// _PAGE_GLOBAL gets reused to help indicate PROT_NONE for
// non-present PTEs, so be careful not to set it in that
// case to avoid confusion.
//
    if (boot_cpu_has(X86_FEATURE_PGE) &&
    (pgprot_val(flags) & _PAGE_PRESENT))
    pte = pte_set_flags(pte, _PAGE_GLOBAL);
    set_pte_vaddr(va, pte);
    }
    static void __init
    cea_map_percpu_pages(void *cea_vaddr, void *ptr, int pages, pgprot_t prot)
    {
    for ( ; pages; pages--, cea_vaddr+= PAGE_SIZE, ptr += PAGE_SIZE)
    cea_set_pte(cea_vaddr, per_cpu_ptr_to_phys(ptr), prot);
    }
#[no_mangle]
unsafe extern "C" fn percpu_setup_debug_store(cpu: c_uint) -> void __init {
    static void __init percpu_setup_debug_store(unsigned int cpu)
    {

    unsigned int npages;
    void *cea;
    if (boot_cpu_data.x86_vendor != X86_VENDOR_INTEL)
    return;
    cea = &get_cpu_entry_area(cpu).cpu_debug_store;
    npages = sizeof(struct debug_store) / PAGE_SIZE;
    BUILD_BUG_ON(sizeof(struct debug_store) % PAGE_SIZE != 0);
    cea_map_percpu_pages(cea, &per_cpu(cpu_debug_store, cpu), npages,
    PAGE_KERNEL);
    cea = &get_cpu_entry_area(cpu).cpu_debug_buffers;
//
// Force the population of PMDs for not yet allocated per cpu
// memory like debug store buffers.
//
    npages = sizeof(struct debug_store_buffers) / PAGE_SIZE;
    for (; npages; npages--, cea += PAGE_SIZE)
    cea_set_pte(cea, 0, PAGE_NONE);

    }

    npages = sizeof(estacks.name## _stack) / PAGE_SIZE;		\
    cea_map_percpu_pages(cea.estacks.name## _stack,		\
    estacks.name## _stack, npages, PAGE_KERNEL);	\
    } while (0)
#[no_mangle]
unsafe extern "C" fn percpu_setup_exception_stacks(cpu: c_uint) -> void __init {
    static void __init percpu_setup_exception_stacks(unsigned int cpu)
    {
    struct exception_stacks *estacks = per_cpu_ptr(&exception_stacks, cpu);
    struct cpu_entry_area *cea = get_cpu_entry_area(cpu);
    unsigned int npages;
    BUILD_BUG_ON(sizeof(exception_stacks) % PAGE_SIZE != 0);
    per_cpu(cea_exception_stacks, cpu) = &cea.estacks;
//
// The exceptions stack mappings in the per cpu area are protected
// by guard pages so each stack must be mapped separately. DB2 is
// not mapped; it just exists to catch triple nesting of #DB.
//
    cea_map_stack(DF);
    cea_map_stack(NMI);
    cea_map_stack(DB);
    cea_map_stack(MCE);
    if (IS_ENABLED(CONFIG_AMD_MEM_ENCRYPT)) {
    if (cc_platform_has(CC_ATTR_GUEST_STATE_ENCRYPT)) {
    cea_map_stack(VC);
    cea_map_stack(VC2);
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn percpu_setup_exception_stacks(cpu: c_uint) -> void __init {
    static void __init percpu_setup_exception_stacks(unsigned int cpu)
    {
    struct cpu_entry_area *cea = get_cpu_entry_area(cpu);
    cea_map_percpu_pages(&cea.doublefault_stack,
    &per_cpu(doublefault_stack, cpu), 1, PAGE_KERNEL);
    }

// Setup the fixmap mappings only once per-processor
#[no_mangle]
unsafe extern "C" fn setup_cpu_entry_area(cpu: c_uint) -> void __init {
    static void __init setup_cpu_entry_area(unsigned int cpu)
    {
    struct cpu_entry_area *cea = get_cpu_entry_area(cpu);

// On 64-bit systems, we use a read-only fixmap GDT and TSS.
    let mut gdt_prot: pgprot_t = PAGE_KERNEL_RO;
    let mut tss_prot: pgprot_t = PAGE_KERNEL_RO;

//
// On 32-bit systems, the GDT cannot be read-only because
// our double fault handler uses a task gate, and entering through
// a task gate needs to change an available TSS to busy.  If the
// GDT is read-only, that will triple fault.  The TSS cannot be
// read-only because the CPU writes to it on task switches.
//
    let mut gdt_prot: pgprot_t = PAGE_KERNEL;
    let mut tss_prot: pgprot_t = PAGE_KERNEL;

    kasan_populate_shadow_for_vaddr(cea, CPU_ENTRY_AREA_SIZE,
    early_cpu_to_node(cpu));
    cea_set_pte(&cea.gdt, get_cpu_gdt_paddr(cpu), gdt_prot);
    cea_map_percpu_pages(&cea.entry_stack_page,
    per_cpu_ptr(&entry_stack_storage, cpu), 1,
    PAGE_KERNEL);
//
// The Intel SDM says (Volume 3, 7.2.1):
//
// Avoid placing a page boundary in the part of the TSS that the
// processor reads during a task switch (the first 104 bytes). The
// processor may not correctly perform address translations if a
// boundary occurs in this area. During a task switch, the processor
// reads and writes into the first 104 bytes of each TSS (using
// contiguous physical addresses beginning with the physical address
// of the first byte of the TSS). So, after TSS access begins, if
// part of the 104 bytes is not physically contiguous, the processor
// will access incorrect information without generating a page-fault
// exception.
//
// There are also a lot of errata involving the TSS spanning a page
// boundary.  Assert that we're not doing that.
//
    BUILD_BUG_ON((offsetof(struct tss_struct, x86_tss) ^
    offsetofend(struct tss_struct, x86_tss)) & PAGE_MASK);
    BUILD_BUG_ON(sizeof(struct tss_struct) % PAGE_SIZE != 0);
//
// VMX changes the host TR limit to 0x67 after a VM exit. This is
// okay, since 0x67 covers the size of struct x86_hw_tss. Make sure
// that this is correct.
//
    BUILD_BUG_ON(offsetof(struct tss_struct, x86_tss) != 0);
    BUILD_BUG_ON(sizeof(struct x86_hw_tss) != 0x68);
    cea_map_percpu_pages(&cea.tss, &per_cpu(cpu_tss_rw, cpu),
    sizeof(struct tss_struct) / PAGE_SIZE, tss_prot);

    per_cpu(cpu_entry_area, cpu) = cea;

    percpu_setup_exception_stacks(cpu);
    percpu_setup_debug_store(cpu);
    }
#[no_mangle]
unsafe extern "C" fn setup_cpu_entry_area_ptes() -> __init void {
    static __init void setup_cpu_entry_area_ptes(void)
    {

    unsigned long start, end;
// The +1 is for the readonly IDT:
    BUILD_BUG_ON((CPU_ENTRY_AREA_PAGES+1)*PAGE_SIZE != CPU_ENTRY_AREA_MAP_SIZE);
    BUG_ON(CPU_ENTRY_AREA_BASE & ~PMD_MASK);
    start = CPU_ENTRY_AREA_BASE;
    end = start + CPU_ENTRY_AREA_MAP_SIZE;
// Careful here: start + PMD_SIZE might wrap around
    for (; start < end && start >= CPU_ENTRY_AREA_BASE; start += PMD_SIZE)
    populate_extra_pte(start);

    }
#[no_mangle]
pub unsafe extern "C" fn setup_cpu_entry_areas() -> void __init {
    void __init setup_cpu_entry_areas(void)
    {
    unsigned int cpu;
    init_cea_offsets();
    setup_cpu_entry_area_ptes();
    for_each_possible_cpu(cpu)
    setup_cpu_entry_area(cpu);
//
// This is the last essential update to swapper_pgdir which needs
// to be synchronized to initial_page_table on 32bit.
//
    sync_initial_page_table();
    }
