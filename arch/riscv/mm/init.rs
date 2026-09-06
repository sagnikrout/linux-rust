//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/init.c
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
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
// Copyright (C) 2020 FORTH-ICS/CARV
// Nick Kossifidis <mick@ics.forth.gr>
//

    DECLARE_BITMAP(new_valid_map_cpus, NR_CPUS);

    struct kernel_mapping kernel_map __ro_after_init;
    EXPORT_SYMBOL(kernel_map);

    let mut __ro_after_init: u64 satp_mode = SATP_MODE_57;

    let mut __ro_after_init: u64 satp_mode = SATP_MODE_32;

    EXPORT_SYMBOL(satp_mode);

    let mut __ro_after_init: bool pgtable_l4_enabled = true;
    let mut __ro_after_init: bool pgtable_l5_enabled = true;
    EXPORT_SYMBOL(pgtable_l4_enabled);
    EXPORT_SYMBOL(pgtable_l5_enabled);

    phys_addr_t phys_ram_base __ro_after_init;
    EXPORT_SYMBOL(phys_ram_base);

    PFN_PHYS(MAX_FOLIO_VMEMMAP_ALIGN / \
    sizeof(struct page)))
    unsigned long vmemmap_start_pfn __ro_after_init;
    EXPORT_SYMBOL(vmemmap_start_pfn);

    extern char _start[];
    void *_dtb_early_va __initdata;
    uintptr_t _dtb_early_pa __initdata;
    phys_addr_t dma32_phys_limit __initdata;
#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) -> void __init {
    void __init arch_zone_limits_init(unsigned long *max_zone_pfns)
    {

    max_zone_pfns[ZONE_DMA32] = PFN_DOWN(dma32_phys_limit);

    max_zone_pfns[ZONE_NORMAL] = max_low_pfn;
    }

#[no_mangle]
pub unsafe extern "C" fn print_mlk(name: *mut c_char, b: c_ulong, t: c_ulong) {
    static inline void print_mlk(char *name, unsigned long b, unsigned long t)
    {
    pr_notice("%12s : 0x%08lx - 0x%08lx   (%4ld kB)\n", name, b, t,
    (((t) - (b)) >> LOG2_SZ_1K));
    }
#[no_mangle]
pub unsafe extern "C" fn print_mlm(name: *mut c_char, b: c_ulong, t: c_ulong) {
    static inline void print_mlm(char *name, unsigned long b, unsigned long t)
    {
    pr_notice("%12s : 0x%08lx - 0x%08lx   (%4ld MB)\n", name, b, t,
    (((t) - (b)) >> LOG2_SZ_1M));
    }
#[no_mangle]
pub unsafe extern "C" fn print_mlg(name: *mut c_char, b: c_ulong, t: c_ulong) {
    static inline void print_mlg(char *name, unsigned long b, unsigned long t)
    {
    pr_notice("%12s : 0x%08lx - 0x%08lx   (%4ld GB)\n", name, b, t,
    (((t) - (b)) >> LOG2_SZ_1G));
    }

#[no_mangle]
pub unsafe extern "C" fn print_mlt(name: *mut c_char, b: c_ulong, t: c_ulong) {
    static inline void print_mlt(char *name, unsigned long b, unsigned long t)
    {
    pr_notice("%12s : 0x%08lx - 0x%08lx   (%4ld TB)\n", name, b, t,
    (((t) - (b)) >> LOG2_SZ_1T));
    }

#[no_mangle]
pub unsafe extern "C" fn print_ml(name: *mut c_char, b: c_ulong, t: c_ulong) {
    static inline void print_ml(char *name, unsigned long b, unsigned long t)
    {
    let mut diff: c_ulong = t - b;
    if (IS_ENABLED(CONFIG_64BIT) && (diff >> LOG2_SZ_1T) >= 10)
    print_mlt(name, b, t);
#[no_mangle]
pub unsafe extern "C" fn if(10: (diff >> LOG2_SZ_1G) >=) -> else {
    else if ((diff >> LOG2_SZ_1G) >= 10)
    print_mlg(name, b, t);
#[no_mangle]
pub unsafe extern "C" fn if(10: (diff >> LOG2_SZ_1M) >=) -> else {
    else if ((diff >> LOG2_SZ_1M) >= 10)
    print_mlm(name, b, t);
    else
    print_mlk(name, b, t);
    }
#[no_mangle]
unsafe extern "C" fn print_vm_layout() -> void __init {
    static void __init print_vm_layout(void)
    {
    pr_notice("Virtual kernel memory layout:\n");
    print_ml("fixmap", (unsigned long)FIXADDR_START,
    (unsigned long)FIXADDR_TOP);
    print_ml("pci io", (unsigned long)PCI_IO_START,
    (unsigned long)PCI_IO_END);
    print_ml("vmemmap", (unsigned long)VMEMMAP_START,
    (unsigned long)VMEMMAP_END);
    print_ml("vmalloc", (unsigned long)VMALLOC_START,
    (unsigned long)VMALLOC_END);

    print_ml("modules", (unsigned long)MODULES_VADDR,
    (unsigned long)MODULES_END);

    print_ml("lowmem", (unsigned long)PAGE_OFFSET,
    (unsigned long)high_memory);
    if (IS_ENABLED(CONFIG_64BIT)) {

    print_ml("kasan", KASAN_SHADOW_START, KASAN_SHADOW_END);

    print_ml("kernel", (unsigned long)kernel_map.virt_addr,
    (unsigned long)ADDRESS_SPACE_END);
    }
    }

    static void print_vm_layout(void) { }

#[no_mangle]
pub unsafe extern "C" fn arch_mm_preinit() -> void __init {
    void __init arch_mm_preinit(void)
    {
    bool swiotlb = max_pfn > PFN_DOWN(dma32_phys_limit) &&
    memblock_start_of_DRAM() < dma32_phys_limit;
    let mut swiotlb_flags: c_uint = SWIOTLB_VERBOSE;

    BUG_ON(!mem_map);

    if (IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) && !swiotlb &&
    dma_cache_alignment != 1) {
//
// No 32-bit DMA bouncing needed (either all DRAM is within
// the 32-bit limit, or it all starts above it), but
// kmalloc() buffers whose sizes are not cache-line-aligned
// still require bouncing for non-coherent DMA.  Use
// SWIOTLB_ANY so that the buffer can be allocated from high
// memory when DRAM starts above dma32_phys_limit.  Allocate
// ~1 MB per 1 GB of RAM.
//
    unsigned long size =
    DIV_ROUND_UP(memblock_phys_mem_size(), 1024);
    swiotlb_adjust_size(min(swiotlb_size_or_default(), size));
    swiotlb = true;
    swiotlb_flags |= SWIOTLB_ANY;
    }
    swiotlb_init(swiotlb, swiotlb_flags);
    print_vm_layout();
    }
// Limit the memory size via mem.
    static phys_addr_t memory_limit;
#[no_mangle]
unsafe extern "C" fn early_mem(p: *mut c_char) -> int __init {
    static int __init early_mem(char *p)
    {
    u64 size;
    if (!p)
    return 1;
    size = memparse(p, &p) & PAGE_MASK;
    memory_limit = min_t(u64, size, memory_limit);
    pr_notice("Memory limited to %lldMB\n", (u64)memory_limit >> 20);
    return 0;
    }
    early_param("mem", early_mem);
#[no_mangle]
unsafe extern "C" fn setup_bootmem() -> void __init {
    static void __init setup_bootmem(void)
    {
    let mut vmlinux_end: phys_addr_t = __pa_symbol(&_end);
    phys_addr_t max_mapped_addr;
    phys_addr_t phys_ram_end, vmlinux_start;
    vmlinux_start = __pa_symbol(&_start);
    memblock_enforce_memory_limit(memory_limit);
//
// Make sure we align the reservation on PMD_SIZE since we will
// map the kernel in the linear mapping as read-only: we do not want
// any allocation to happen between _end and the next pmd aligned page.
//
    if (IS_ENABLED(CONFIG_64BIT) && IS_ENABLED(CONFIG_STRICT_KERNEL_RWX))
    vmlinux_end = (vmlinux_end + PMD_SIZE - 1) & PMD_MASK;
//
// Reserve from the start of the kernel to the end of the kernel
//
    memblock_reserve(vmlinux_start, vmlinux_end - vmlinux_start);
//
// Make sure we align the start of the memory on a PMD boundary so that
// at worst, we map the linear mapping with PMD mappings.
//
    phys_ram_base = memblock_start_of_DRAM() & PMD_MASK;

    vmemmap_start_pfn = round_down(phys_ram_base, VMEMMAP_ADDR_ALIGN) >> PAGE_SHIFT;

//
// In 64-bit, any use of __va/__pa before this point is wrong as we
// did not know the start of DRAM before.
//
    if (IS_ENABLED(CONFIG_64BIT) && IS_ENABLED(CONFIG_MMU))
    kernel_map.va_pa_offset = PAGE_OFFSET - phys_ram_base;
//
// The size of the linear page mapping may restrict the amount of
// usable RAM.
//
    if (IS_ENABLED(CONFIG_64BIT) && IS_ENABLED(CONFIG_MMU)) {
    max_mapped_addr = __pa(PAGE_OFFSET) + KERN_VIRT_SIZE;
    if (memblock_end_of_DRAM() > max_mapped_addr) {
    memblock_cap_memory_range(phys_ram_base,
    max_mapped_addr - phys_ram_base);
    pr_warn("Physical memory overflows the linear mapping size: region above %pa removed",
    &max_mapped_addr);
    }
    }
//
// Reserve physical address space that would be mapped to virtual
// addresses greater than (void *)(-PAGE_SIZE) because:
// - This memory would overlap with ERR_PTR
// - This memory belongs to high memory, which is not supported
//
// This is not applicable to 64-bit kernel, because virtual addresses
// after (void *)(-PAGE_SIZE) are not linearly mapped: they are
// occupied by kernel mapping. Also it is unrealistic for high memory
// to exist on 64-bit platforms.
//
    if (!IS_ENABLED(CONFIG_64BIT)) {
    max_mapped_addr = __va_to_pa_nodebug(-PAGE_SIZE);
    memblock_reserve(max_mapped_addr, (phys_addr_t)-max_mapped_addr);
    }
    phys_ram_end = memblock_end_of_DRAM();
    min_low_pfn = PFN_UP(phys_ram_base);
    max_low_pfn = max_pfn = PFN_DOWN(phys_ram_end);
    dma32_phys_limit = min(4UL * SZ_1G, (unsigned long)PFN_PHYS(max_low_pfn));
    reserve_initrd_mem();
//
// No allocation should be done before reserving the memory as defined
// in the device tree, otherwise the allocation could end up in a
// reserved region.
//
    early_init_fdt_scan_reserved_mem();
//
// If DTB is built in, no need to reserve its memblock.
// Otherwise, do reserve it but avoid using
// early_init_fdt_reserve_self() since __pa() does
// not work for DTB pointers that are fixmap addresses
//
    if (!IS_ENABLED(CONFIG_BUILTIN_DTB))
    memblock_reserve(dtb_early_pa, fdt_totalsize(dtb_early_va));
    }

    extern unsigned long __rela_dyn_start, __rela_dyn_end;
#[no_mangle]
unsafe extern "C" fn relocate_kernel() -> void __init {
    static void __init relocate_kernel(void)
    {
    Elf_Rela *rela = (Elf_Rela *)&__rela_dyn_start;
//
// This holds the offset between the linked virtual address and the
// relocated virtual address.
//
    let mut reloc_offset: uintptr_t = kernel_map.virt_addr - KERNEL_LINK_ADDR;
//
// This holds the offset between kernel linked virtual address and
// physical address.
//
    let mut va_kernel_link_pa_offset: uintptr_t = KERNEL_LINK_ADDR - kernel_map.phys_addr;
    for ( ; rela < (Elf_Rela *)&__rela_dyn_end; rela++) {
    let mut addr: Elf_Addr = (rela.r_offset - va_kernel_link_pa_offset);
    let mut relocated_addr: Elf_Addr = rela.r_addend;
    if (rela.r_info != R_RISCV_RELATIVE)
    continue;
//
// Make sure to not relocate vdso symbols like rt_sigreturn
// which are linked from the address 0 in vmlinux since
// vdso symbol addresses are actually used as an offset from
// mm->context.vdso in VDSO_OFFSET macro.
//
    if (relocated_addr >= KERNEL_LINK_ADDR)
    relocated_addr += reloc_offset;
// (Elf_Addr *)addr = relocated_addr;
    }
    }

    struct pt_alloc_ops pt_ops __meminitdata;
    pgd_t swapper_pg_dir[PTRS_PER_PGD] __page_aligned_bss;
    pgd_t trampoline_pg_dir[PTRS_PER_PGD] __page_aligned_bss;
    static pte_t fixmap_pte[PTRS_PER_PTE] __page_aligned_bss;
    pgd_t early_pg_dir[PTRS_PER_PGD] __initdata __aligned(PAGE_SIZE);
    static const pgprot_t protection_map[16] = {
    [VM_NONE]					= PAGE_NONE,
    [VM_READ]					= PAGE_READ,
    [VM_WRITE]					= PAGE_SHADOWSTACK,
    [VM_WRITE | VM_READ]				= PAGE_COPY,
    [VM_EXEC]					= PAGE_EXEC,
    [VM_EXEC | VM_READ]				= PAGE_READ_EXEC,
    [VM_EXEC | VM_WRITE]				= PAGE_COPY_EXEC,
    [VM_EXEC | VM_WRITE | VM_READ]			= PAGE_COPY_EXEC,
    [VM_SHARED]					= PAGE_NONE,
    [VM_SHARED | VM_READ]				= PAGE_READ,
    [VM_SHARED | VM_WRITE]				= PAGE_SHARED,
    [VM_SHARED | VM_WRITE | VM_READ]		= PAGE_SHARED,
    [VM_SHARED | VM_EXEC]				= PAGE_EXEC,
    [VM_SHARED | VM_EXEC | VM_READ]			= PAGE_READ_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE]		= PAGE_SHARED_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ]	= PAGE_SHARED_EXEC
    };
    DECLARE_VM_GET_PAGE_PROT
#[no_mangle]
pub unsafe extern "C" fn __set_fixmap(idx: enum fixed_addresses, phys: phys_addr_t, prot: pgprot_t) {
    void __set_fixmap(enum fixed_addresses idx, phys_addr_t phys, pgprot_t prot)
    {
    let mut addr: c_ulong = __fix_to_virt(idx);
    pte_t *ptep;
    BUG_ON(idx <= FIX_HOLE || idx >= __end_of_fixed_addresses);
    ptep = &fixmap_pte[pte_index(addr)];
    if (pgprot_val(prot))
    set_pte(ptep, pfn_pte(phys >> PAGE_SHIFT, prot));
    else
    pte_clear(&init_mm, addr, ptep);
    local_flush_tlb_page(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn get_pte_virt_early(pa: phys_addr_t) -> *mut pte_t __init {
    static inline pte_t *__init get_pte_virt_early(phys_addr_t pa)
    {
    return (pte_t *)((uintptr_t)pa);
    }
#[no_mangle]
pub unsafe extern "C" fn get_pte_virt_fixmap(pa: phys_addr_t) -> *mut pte_t __init {
    static inline pte_t *__init get_pte_virt_fixmap(phys_addr_t pa)
    {
    clear_fixmap(FIX_PTE);
    return (pte_t *)set_fixmap_offset(FIX_PTE, pa);
    }
#[no_mangle]
pub unsafe extern "C" fn get_pte_virt_late(pa: phys_addr_t) -> *mut pte_t __meminit {
    static inline pte_t *__meminit get_pte_virt_late(phys_addr_t pa)
    {
    return (pte_t *) __va(pa);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pte_early(va: uintptr_t) -> phys_addr_t __init {
    static inline phys_addr_t __init alloc_pte_early(uintptr_t va)
    {
//
// We only create PMD or PGD early mappings so we
// should never reach here with MMU disabled.
//
    BUG();
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pte_fixmap(va: uintptr_t) -> phys_addr_t __init {
    static inline phys_addr_t __init alloc_pte_fixmap(uintptr_t va)
    {
    return memblock_phys_alloc(PAGE_SIZE, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn alloc_pte_late(va: uintptr_t) -> phys_addr_t __meminit {
    static phys_addr_t __meminit alloc_pte_late(uintptr_t va)
    {
    struct ptdesc *ptdesc = pagetable_alloc(GFP_KERNEL, 0);
//
// We do not know which mm the PTE page is associated to at this point.
// Passing NULL to the ctor is the safe option, though it may result
// in unnecessary work (e.g. initialising the ptlock for init_mm).
//
    BUG_ON(!ptdesc || !pagetable_pte_ctor(core::ptr::null_mut(), ptdesc));
    return __pa((pte_t *)ptdesc_address(ptdesc));
    }
    static void __meminit create_pte_mapping(pte_t *ptep, uintptr_t va, phys_addr_t pa, phys_addr_t sz,
    pgprot_t prot)
    {
    let mut pte_idx: uintptr_t = pte_index(va);
    BUG_ON(sz != PAGE_SIZE);
    if (pte_none(ptep[pte_idx]))
    ptep[pte_idx] = pfn_pte(PFN_DOWN(pa), prot);
    }

    static pmd_t trampoline_pmd[PTRS_PER_PMD] __page_aligned_bss;
    static pmd_t fixmap_pmd[PTRS_PER_PMD] __page_aligned_bss;
    static pmd_t early_pmd[PTRS_PER_PMD] __initdata __aligned(PAGE_SIZE);
    static p4d_t trampoline_p4d[PTRS_PER_P4D] __page_aligned_bss;
    static p4d_t fixmap_p4d[PTRS_PER_P4D] __page_aligned_bss;
    static p4d_t early_p4d[PTRS_PER_P4D] __initdata __aligned(PAGE_SIZE);
    static pud_t trampoline_pud[PTRS_PER_PUD] __page_aligned_bss;
    static pud_t fixmap_pud[PTRS_PER_PUD] __page_aligned_bss;
    static pud_t early_pud[PTRS_PER_PUD] __initdata __aligned(PAGE_SIZE);
#[no_mangle]
unsafe extern "C" fn get_pmd_virt_early(pa: phys_addr_t) -> *mut pmd_t __init {
    static pmd_t *__init get_pmd_virt_early(phys_addr_t pa)
    {
// Before MMU is enabled
    return (pmd_t *)((uintptr_t)pa);
    }
#[no_mangle]
unsafe extern "C" fn get_pmd_virt_fixmap(pa: phys_addr_t) -> *mut pmd_t __init {
    static pmd_t *__init get_pmd_virt_fixmap(phys_addr_t pa)
    {
    clear_fixmap(FIX_PMD);
    return (pmd_t *)set_fixmap_offset(FIX_PMD, pa);
    }
#[no_mangle]
unsafe extern "C" fn get_pmd_virt_late(pa: phys_addr_t) -> *mut pmd_t __meminit {
    static pmd_t *__meminit get_pmd_virt_late(phys_addr_t pa)
    {
    return (pmd_t *) __va(pa);
    }
#[no_mangle]
unsafe extern "C" fn alloc_pmd_early(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_pmd_early(uintptr_t va)
    {
    BUG_ON((va - kernel_map.virt_addr) >> PUD_SHIFT);
    return (uintptr_t)early_pmd;
    }
#[no_mangle]
unsafe extern "C" fn alloc_pmd_fixmap(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_pmd_fixmap(uintptr_t va)
    {
    return memblock_phys_alloc(PAGE_SIZE, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn alloc_pmd_late(va: uintptr_t) -> phys_addr_t __meminit {
    static phys_addr_t __meminit alloc_pmd_late(uintptr_t va)
    {
    struct ptdesc *ptdesc = pagetable_alloc(GFP_KERNEL, 0);
// See comment in alloc_pte_late() regarding NULL passed the ctor
    BUG_ON(!ptdesc || !pagetable_pmd_ctor(core::ptr::null_mut(), ptdesc));
    return __pa((pmd_t *)ptdesc_address(ptdesc));
    }
    static void __meminit create_pmd_mapping(pmd_t *pmdp,
    uintptr_t va, phys_addr_t pa,
    phys_addr_t sz, pgprot_t prot)
    {
    pte_t *ptep;
    phys_addr_t pte_phys;
    let mut pmd_idx: uintptr_t = pmd_index(va);
    if (sz == PMD_SIZE) {
    if (pmd_none(pmdp[pmd_idx]))
    pmdp[pmd_idx] = pfn_pmd(PFN_DOWN(pa), prot);
    return;
    }
    if (pmd_none(pmdp[pmd_idx])) {
    pte_phys = pt_ops.alloc_pte(va);
    pmdp[pmd_idx] = pfn_pmd(PFN_DOWN(pte_phys), PAGE_TABLE);
    ptep = pt_ops.get_pte_virt(pte_phys);
    memset(ptep, 0, PAGE_SIZE);
    } else {
    pte_phys = PFN_PHYS(_pmd_pfn(pmdp[pmd_idx]));
    ptep = pt_ops.get_pte_virt(pte_phys);
    }
    create_pte_mapping(ptep, va, pa, sz, prot);
    }
#[no_mangle]
unsafe extern "C" fn get_pud_virt_early(pa: phys_addr_t) -> *mut pud_t __init {
    static pud_t *__init get_pud_virt_early(phys_addr_t pa)
    {
    return (pud_t *)((uintptr_t)pa);
    }
#[no_mangle]
unsafe extern "C" fn get_pud_virt_fixmap(pa: phys_addr_t) -> *mut pud_t __init {
    static pud_t *__init get_pud_virt_fixmap(phys_addr_t pa)
    {
    clear_fixmap(FIX_PUD);
    return (pud_t *)set_fixmap_offset(FIX_PUD, pa);
    }
#[no_mangle]
unsafe extern "C" fn get_pud_virt_late(pa: phys_addr_t) -> *mut pud_t __meminit {
    static pud_t *__meminit get_pud_virt_late(phys_addr_t pa)
    {
    return (pud_t *)__va(pa);
    }
#[no_mangle]
unsafe extern "C" fn alloc_pud_early(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_pud_early(uintptr_t va)
    {
// Only one PUD is available for early mapping
    BUG_ON((va - kernel_map.virt_addr) >> PGDIR_SHIFT);
    return (uintptr_t)early_pud;
    }
#[no_mangle]
unsafe extern "C" fn alloc_pud_fixmap(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_pud_fixmap(uintptr_t va)
    {
    return memblock_phys_alloc(PAGE_SIZE, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn alloc_pud_late(va: uintptr_t) -> phys_addr_t __meminit {
    static phys_addr_t __meminit alloc_pud_late(uintptr_t va)
    {
    struct ptdesc *ptdesc = pagetable_alloc(GFP_KERNEL, 0);
    BUG_ON(!ptdesc);
    pagetable_pud_ctor(ptdesc);
    return __pa((pud_t *)ptdesc_address(ptdesc));
    }
#[no_mangle]
unsafe extern "C" fn get_p4d_virt_early(pa: phys_addr_t) -> *mut p4d_t __init {
    static p4d_t *__init get_p4d_virt_early(phys_addr_t pa)
    {
    return (p4d_t *)((uintptr_t)pa);
    }
#[no_mangle]
unsafe extern "C" fn get_p4d_virt_fixmap(pa: phys_addr_t) -> *mut p4d_t __init {
    static p4d_t *__init get_p4d_virt_fixmap(phys_addr_t pa)
    {
    clear_fixmap(FIX_P4D);
    return (p4d_t *)set_fixmap_offset(FIX_P4D, pa);
    }
#[no_mangle]
unsafe extern "C" fn get_p4d_virt_late(pa: phys_addr_t) -> *mut p4d_t __meminit {
    static p4d_t *__meminit get_p4d_virt_late(phys_addr_t pa)
    {
    return (p4d_t *)__va(pa);
    }
#[no_mangle]
unsafe extern "C" fn alloc_p4d_early(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_p4d_early(uintptr_t va)
    {
// Only one P4D is available for early mapping
    BUG_ON((va - kernel_map.virt_addr) >> PGDIR_SHIFT);
    return (uintptr_t)early_p4d;
    }
#[no_mangle]
unsafe extern "C" fn alloc_p4d_fixmap(va: uintptr_t) -> phys_addr_t __init {
    static phys_addr_t __init alloc_p4d_fixmap(uintptr_t va)
    {
    return memblock_phys_alloc(PAGE_SIZE, PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn alloc_p4d_late(va: uintptr_t) -> phys_addr_t __meminit {
    static phys_addr_t __meminit alloc_p4d_late(uintptr_t va)
    {
    struct ptdesc *ptdesc = pagetable_alloc(GFP_KERNEL, 0);
    BUG_ON(!ptdesc);
    pagetable_p4d_ctor(ptdesc);
    return __pa((p4d_t *)ptdesc_address(ptdesc));
    }
    static void __meminit create_pud_mapping(pud_t *pudp, uintptr_t va, phys_addr_t pa, phys_addr_t sz,
    pgprot_t prot)
    {
    pmd_t *nextp;
    phys_addr_t next_phys;
    let mut pud_index: uintptr_t = pud_index(va);
    if (sz == PUD_SIZE) {
    if (pud_val(pudp[pud_index]) == 0)
    pudp[pud_index] = pfn_pud(PFN_DOWN(pa), prot);
    return;
    }
    if (pud_val(pudp[pud_index]) == 0) {
    next_phys = pt_ops.alloc_pmd(va);
    pudp[pud_index] = pfn_pud(PFN_DOWN(next_phys), PAGE_TABLE);
    nextp = pt_ops.get_pmd_virt(next_phys);
    memset(nextp, 0, PAGE_SIZE);
    } else {
    next_phys = PFN_PHYS(_pud_pfn(pudp[pud_index]));
    nextp = pt_ops.get_pmd_virt(next_phys);
    }
    create_pmd_mapping(nextp, va, pa, sz, prot);
    }
    static void __meminit create_p4d_mapping(p4d_t *p4dp, uintptr_t va, phys_addr_t pa, phys_addr_t sz,
    pgprot_t prot)
    {
    pud_t *nextp;
    phys_addr_t next_phys;
    let mut p4d_index: uintptr_t = p4d_index(va);
    if (sz == P4D_SIZE) {
    if (p4d_val(p4dp[p4d_index]) == 0)
    p4dp[p4d_index] = pfn_p4d(PFN_DOWN(pa), prot);
    return;
    }
    if (p4d_val(p4dp[p4d_index]) == 0) {
    next_phys = pt_ops.alloc_pud(va);
    p4dp[p4d_index] = pfn_p4d(PFN_DOWN(next_phys), PAGE_TABLE);
    nextp = pt_ops.get_pud_virt(next_phys);
    memset(nextp, 0, PAGE_SIZE);
    } else {
    next_phys = PFN_PHYS(_p4d_pfn(p4dp[p4d_index]));
    nextp = pt_ops.get_pud_virt(next_phys);
    }
    create_pud_mapping(nextp, va, pa, sz, prot);
    }

    pt_ops.alloc_p4d(__va) : (pgtable_l4_enabled ?		\
    pt_ops.alloc_pud(__va) : pt_ops.alloc_pmd(__va)))

    pt_ops.get_p4d_virt(__pa) : (pgd_next_t *)(pgtable_l4_enabled ?	\
    pt_ops.get_pud_virt(__pa) : (pud_t *)pt_ops.get_pmd_virt(__pa)))

    (pgtable_l5_enabled ?			\
    create_p4d_mapping(__nextp, __va, __pa, __sz, __prot) : \
    (pgtable_l4_enabled ?			\
    create_pud_mapping((pud_t *)__nextp, __va, __pa, __sz, __prot) :	\
    create_pmd_mapping((pmd_t *)__nextp, __va, __pa, __sz, __prot)))

    (uintptr_t)fixmap_p4d : (pgtable_l4_enabled ?		\
    (uintptr_t)fixmap_pud : (uintptr_t)fixmap_pmd))

    (uintptr_t)trampoline_p4d : (pgtable_l4_enabled ?	\
    (uintptr_t)trampoline_pud : (uintptr_t)trampoline_pmd))

    create_pte_mapping(__nextp, __va, __pa, __sz, __prot)

    void __meminit create_pgd_mapping(pgd_t *pgdp, uintptr_t va, phys_addr_t pa, phys_addr_t sz,
    pgprot_t prot)
    {
    pgd_next_t *nextp;
    phys_addr_t next_phys;
    let mut pgd_idx: uintptr_t = pgd_index(va);
    if (sz == PGDIR_SIZE) {
    if (pgd_val(pgdp[pgd_idx]) == 0)
    pgdp[pgd_idx] = pfn_pgd(PFN_DOWN(pa), prot);
    return;
    }
    if (pgd_val(pgdp[pgd_idx]) == 0) {
    next_phys = alloc_pgd_next(va);
    pgdp[pgd_idx] = pfn_pgd(PFN_DOWN(next_phys), PAGE_TABLE);
    nextp = get_pgd_next_virt(next_phys);
    memset(nextp, 0, PAGE_SIZE);
    } else {
    next_phys = PFN_PHYS(_pgd_pfn(pgdp[pgd_idx]));
    nextp = get_pgd_next_virt(next_phys);
    }
    create_pgd_next_mapping(nextp, va, pa, sz, prot);
    }
#[no_mangle]
unsafe extern "C" fn best_map_size(pa: phys_addr_t, va: uintptr_t, size: phys_addr_t) -> uintptr_t __meminit {
    static uintptr_t __meminit best_map_size(phys_addr_t pa, uintptr_t va, phys_addr_t size)
    {
    if (debug_pagealloc_enabled())
    return PAGE_SIZE;
    if (pgtable_l5_enabled &&
    !(pa & (P4D_SIZE - 1)) && !(va & (P4D_SIZE - 1)) && size >= P4D_SIZE)
    return P4D_SIZE;
    if (pgtable_l4_enabled &&
    !(pa & (PUD_SIZE - 1)) && !(va & (PUD_SIZE - 1)) && size >= PUD_SIZE)
    return PUD_SIZE;
    if (IS_ENABLED(CONFIG_64BIT) &&
    !(pa & (PMD_SIZE - 1)) && !(va & (PMD_SIZE - 1)) && size >= PMD_SIZE)
    return PMD_SIZE;
    return PAGE_SIZE;
    }

#[no_mangle]
unsafe extern "C" fn pgprot_from_va(va: uintptr_t) -> __meminit pgprot_t {
    static __meminit pgprot_t pgprot_from_va(uintptr_t va)
    {
    if (is_va_kernel_text(va))
    return PAGE_KERNEL_READ_EXEC;
//
// In 64-bit kernel, the kernel mapping is outside the linear mapping so
// we must protect its linear mapping alias from being executed and
// written.
// And rodata section is marked readonly in mark_rodata_ro.
//
    if (IS_ENABLED(CONFIG_64BIT) && is_va_kernel_lm_alias_text(va))
    return PAGE_KERNEL_READ;
    return PAGE_KERNEL;
    }
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    set_kernel_memory(__start_rodata, _data, set_memory_ro);
    if (IS_ENABLED(CONFIG_64BIT))
    set_kernel_memory(lm_alias(__start_rodata), lm_alias(_data),
    set_memory_ro);
    }

#[no_mangle]
unsafe extern "C" fn pgprot_from_va(va: uintptr_t) -> __meminit pgprot_t {
    static __meminit pgprot_t pgprot_from_va(uintptr_t va)
    {
    if (IS_ENABLED(CONFIG_64BIT) && !is_kernel_mapping(va))
    return PAGE_KERNEL;
    return PAGE_KERNEL_EXEC;
    }

    u64 __pi_set_satp_mode_from_cmdline(uintptr_t dtb_pa);
    u64 __pi_set_satp_mode_from_fdt(uintptr_t dtb_pa);
#[no_mangle]
unsafe extern "C" fn disable_pgtable_l5() -> void __init {
    static void __init disable_pgtable_l5(void)
    {
    pgtable_l5_enabled = false;
    kernel_map.page_offset = PAGE_OFFSET_L4;
    satp_mode = SATP_MODE_48;
    }
#[no_mangle]
unsafe extern "C" fn disable_pgtable_l4() -> void __init {
    static void __init disable_pgtable_l4(void)
    {
    pgtable_l4_enabled = false;
    kernel_map.page_offset = PAGE_OFFSET_L3;
    satp_mode = SATP_MODE_39;
    }
#[no_mangle]
unsafe extern "C" fn print_no4lvl(p: *mut c_char) -> int __init {
    static int __init print_no4lvl(char *p)
    {
    pr_info("Disabled 4-level and 5-level paging");
    return 0;
    }
    early_param("no4lvl", print_no4lvl);
#[no_mangle]
unsafe extern "C" fn print_no5lvl(p: *mut c_char) -> int __init {
    static int __init print_no5lvl(char *p)
    {
    pr_info("Disabled 5-level paging");
    return 0;
    }
    early_param("no5lvl", print_no5lvl);
#[no_mangle]
unsafe extern "C" fn set_mmap_rnd_bits_max() -> void __init {
    static void __init set_mmap_rnd_bits_max(void)
    {
    mmap_rnd_bits_max = MMAP_VA_BITS - PAGE_SHIFT - 3;
    }
#[no_mangle]
unsafe extern "C" fn is_vaddr_valid(va: c_ulong) -> bool __init {
    static bool __init is_vaddr_valid(unsigned long va)
    {
    let mut up: c_ulong = 0;
    switch (satp_mode) {
    case SATP_MODE_39:
    up = 1UL << 38;
    break;
    case SATP_MODE_48:
    up = 1UL << 47;
    break;
    case SATP_MODE_57:
    up = 1UL << 56;
    break;
    default:
    return false;
    }
    return (va < up) || (va >= (ULONG_MAX - up + 1));
    }
//
// There is a simple way to determine if 4-level is supported by the
// underlying hardware: establish 1:1 mapping in 4-level page table mode
// then read SATP to see if the configuration was taken into account
// meaning sv48 is supported.
// The maximum SATP mode is limited by both the command line and the "mmu-type"
// property in the device tree, since some platforms may hang if an unsupported
// SATP mode is attempted.
//
#[no_mangle]
unsafe extern "C" fn set_satp_mode(dtb_pa: uintptr_t) -> __init void {
    static __init void set_satp_mode(uintptr_t dtb_pa)
    {
    u64 identity_satp, hw_satp;
    let mut set_satp_mode_pmd: uintptr_t = ((unsigned long)set_satp_mode) & PMD_MASK;
    u64 satp_mode_limit = min_not_zero(__pi_set_satp_mode_from_cmdline(dtb_pa),
    __pi_set_satp_mode_from_fdt(dtb_pa));
    kernel_map.page_offset = PAGE_OFFSET_L5;
    if (satp_mode_limit == SATP_MODE_48) {
    disable_pgtable_l5();
    } else if (satp_mode_limit == SATP_MODE_39) {
    disable_pgtable_l5();
    disable_pgtable_l4();
    return;
    }
    create_p4d_mapping(early_p4d,
    set_satp_mode_pmd, (uintptr_t)early_pud,
    P4D_SIZE, PAGE_TABLE);
    create_pud_mapping(early_pud,
    set_satp_mode_pmd, (uintptr_t)early_pmd,
    PUD_SIZE, PAGE_TABLE);
// Handle the case where set_satp_mode straddles 2 PMDs
    create_pmd_mapping(early_pmd,
    set_satp_mode_pmd, set_satp_mode_pmd,
    PMD_SIZE, PAGE_KERNEL_EXEC);
    create_pmd_mapping(early_pmd,
    set_satp_mode_pmd + PMD_SIZE,
    set_satp_mode_pmd + PMD_SIZE,
    PMD_SIZE, PAGE_KERNEL_EXEC);
    retry:
    if (!is_vaddr_valid(set_satp_mode_pmd))
    goto out;
    create_pgd_mapping(early_pg_dir,
    set_satp_mode_pmd,
    pgtable_l5_enabled ?
    (uintptr_t)early_p4d : (uintptr_t)early_pud,
    PGDIR_SIZE, PAGE_TABLE);
    identity_satp = PFN_DOWN((uintptr_t)&early_pg_dir) | satp_mode;
    local_flush_tlb_all();
    csr_write(CSR_SATP, identity_satp);
    hw_satp = csr_swap(CSR_SATP, 0ULL);
    local_flush_tlb_all();
    if (hw_satp != identity_satp) {
    if (pgtable_l5_enabled) {
    disable_pgtable_l5();
    memset(early_pg_dir, 0, PAGE_SIZE);
    goto retry;
    }
    disable_pgtable_l4();
    }
    out:
    memset(early_pg_dir, 0, PAGE_SIZE);
    memset(early_p4d, 0, PAGE_SIZE);
    memset(early_pud, 0, PAGE_SIZE);
    memset(early_pmd, 0, PAGE_SIZE);
    }

//
// setup_vm() is called from head.S with MMU-off.
//
// Following requirements should be honoured for setup_vm() to work
// correctly:
// 1) It should use PC-relative addressing for accessing kernel symbols.
// To achieve this we always use GCC cmodel=medany.
// 2) The compiler instrumentation for FTRACE will not work for setup_vm()
// so disable compiler instrumentation when FTRACE is enabled.
//
// Currently, the above requirements are honoured by using custom CFLAGS
// for init.o in mm/Makefile.
//

#[no_mangle]
unsafe extern "C" fn create_kernel_page_table(pgdir: *mut pgd_t, early: bool) -> void __init {
    static void __init create_kernel_page_table(pgd_t *pgdir, bool early)
    {
    uintptr_t va, end_va;
    end_va = kernel_map.virt_addr + kernel_map.size;
    for (va = kernel_map.virt_addr; va < end_va; va += PMD_SIZE)
    create_pgd_mapping(pgdir, va,
    kernel_map.phys_addr + (va - kernel_map.virt_addr),
    PMD_SIZE,
    early ?
    PAGE_KERNEL_EXEC : pgprot_from_va(va));
    }
//
// Setup a 4MB mapping that encompasses the device tree: for 64-bit kernel,
// this means 2 PMD entries whereas for 32-bit kernel, this is only 1 PGDIR
// entry.
//
    static void __init create_fdt_early_page_table(uintptr_t fix_fdt_va,
    uintptr_t dtb_pa)
    {

    let mut pa: uintptr_t = dtb_pa & ~(PMD_SIZE - 1);
// Make sure the fdt fixmap address is always aligned on PMD size
    BUILD_BUG_ON(FIX_FDT % (PMD_SIZE / PAGE_SIZE));
// In 32-bit only, the fdt lies in its own PGD
    if (!IS_ENABLED(CONFIG_64BIT)) {
    create_pgd_mapping(early_pg_dir, fix_fdt_va,
    pa, MAX_FDT_SIZE, PAGE_KERNEL);
    } else {
    create_pmd_mapping(fixmap_pmd, fix_fdt_va,
    pa, PMD_SIZE, PAGE_KERNEL);
    create_pmd_mapping(fixmap_pmd, fix_fdt_va + PMD_SIZE,
    pa + PMD_SIZE, PMD_SIZE, PAGE_KERNEL);
    }
    dtb_early_va = (void *)fix_fdt_va + (dtb_pa & (PMD_SIZE - 1));

//
// For 64-bit kernel, __va can't be used since it would return a linear
// mapping address whereas dtb_early_va will be used before
// setup_vm_final installs the linear mapping. For 32-bit kernel, as the
// kernel is mapped in the linear mapping, that makes no difference.
//
    dtb_early_va = kernel_mapping_pa_to_va(dtb_pa);

    dtb_early_pa = dtb_pa;
    }
//
// MMU is not enabled, the page tables are allocated directly using
// early_pmd/pud/p4d and the address returned is the physical one.
//
#[no_mangle]
unsafe extern "C" fn pt_ops_set_early() -> void __init {
    static void __init pt_ops_set_early(void)
    {
    pt_ops.alloc_pte = alloc_pte_early;
    pt_ops.get_pte_virt = get_pte_virt_early;

    pt_ops.alloc_pmd = alloc_pmd_early;
    pt_ops.get_pmd_virt = get_pmd_virt_early;
    pt_ops.alloc_pud = alloc_pud_early;
    pt_ops.get_pud_virt = get_pud_virt_early;
    pt_ops.alloc_p4d = alloc_p4d_early;
    pt_ops.get_p4d_virt = get_p4d_virt_early;

    }
//
// MMU is enabled but page table setup is not complete yet.
// fixmap page table alloc functions must be used as a means to temporarily
// map the allocated physical pages since the linear mapping does not exist yet.
//
// Note that this is called with MMU disabled, hence kernel_mapping_pa_to_va,
// but it will be used as described above.
//
#[no_mangle]
unsafe extern "C" fn pt_ops_set_fixmap() -> void __init {
    static void __init pt_ops_set_fixmap(void)
    {
    pt_ops.alloc_pte = kernel_mapping_pa_to_va(alloc_pte_fixmap);
    pt_ops.get_pte_virt = kernel_mapping_pa_to_va(get_pte_virt_fixmap);

    pt_ops.alloc_pmd = kernel_mapping_pa_to_va(alloc_pmd_fixmap);
    pt_ops.get_pmd_virt = kernel_mapping_pa_to_va(get_pmd_virt_fixmap);
    pt_ops.alloc_pud = kernel_mapping_pa_to_va(alloc_pud_fixmap);
    pt_ops.get_pud_virt = kernel_mapping_pa_to_va(get_pud_virt_fixmap);
    pt_ops.alloc_p4d = kernel_mapping_pa_to_va(alloc_p4d_fixmap);
    pt_ops.get_p4d_virt = kernel_mapping_pa_to_va(get_p4d_virt_fixmap);

    }
//
// MMU is enabled and page table setup is complete, so from now, we can use
// generic page allocation functions to setup page table.
//
#[no_mangle]
unsafe extern "C" fn pt_ops_set_late() -> void __init {
    static void __init pt_ops_set_late(void)
    {
    pt_ops.alloc_pte = alloc_pte_late;
    pt_ops.get_pte_virt = get_pte_virt_late;

    pt_ops.alloc_pmd = alloc_pmd_late;
    pt_ops.get_pmd_virt = get_pmd_virt_late;
    pt_ops.alloc_pud = alloc_pud_late;
    pt_ops.get_pud_virt = get_pud_virt_late;
    pt_ops.alloc_p4d = alloc_p4d_late;
    pt_ops.get_p4d_virt = get_p4d_virt_late;

    }

    extern bool __init __pi_set_nokaslr_from_cmdline(uintptr_t dtb_pa);
    extern u64 __init __pi_get_kaslr_seed(uintptr_t dtb_pa);
    extern u64 __init __pi_get_kaslr_seed_zkr(const uintptr_t dtb_pa);
#[no_mangle]
unsafe extern "C" fn print_nokaslr(p: *mut c_char) -> int __init {
    static int __init print_nokaslr(char *p)
    {
    pr_info("Disabled KASLR");
    return 0;
    }
    early_param("nokaslr", print_nokaslr);

#[no_mangle]
pub unsafe extern "C" fn setup_vm(dtb_pa: uintptr_t) -> asmlinkage void __init {
    asmlinkage void __init setup_vm(uintptr_t dtb_pa)
    {
    pmd_t __maybe_unused fix_bmap_spmd, fix_bmap_epmd;

    if (!__pi_set_nokaslr_from_cmdline(dtb_pa)) {
    let mut kaslr_seed: u64 = __pi_get_kaslr_seed_zkr(dtb_pa);
    let mut kernel_size: u32 = (uintptr_t)(&_end) - (uintptr_t)(&_start);
    u32 nr_pos;
    if (kaslr_seed == 0)
    kaslr_seed = __pi_get_kaslr_seed(dtb_pa);
//
// Compute the number of positions available: we are limited
// by the early page table that only has one PUD and we must
// be aligned on PMD_SIZE.
//
    nr_pos = (PUD_SIZE - kernel_size) / PMD_SIZE;
    kernel_map.virt_offset = (kaslr_seed % nr_pos) * PMD_SIZE;
    }

    kernel_map.virt_addr = KERNEL_LINK_ADDR + kernel_map.virt_offset;
    kernel_map.phys_addr = (uintptr_t)(&_start);
    kernel_map.size = (uintptr_t)(&_end) - kernel_map.phys_addr;
    kernel_map.va_kernel_pa_offset = kernel_map.virt_addr - kernel_map.phys_addr;

    set_satp_mode(dtb_pa);
    set_mmap_rnd_bits_max();

//
// In 64-bit, we defer the setup of va_pa_offset to setup_bootmem,
// where we have the system memory layout: this allows us to align
// the physical and virtual mappings and then make use of PUD/P4D/PGD
// for the linear mapping. This is only possible because the kernel
// mapping lies outside the linear mapping.
// In 32-bit however, as the kernel resides in the linear mapping,
// setup_vm_final can not change the mapping established here,
// otherwise the same kernel addresses would get mapped to different
// physical addresses (if the start of dram is different from the
// kernel physical address start).
//
    kernel_map.va_pa_offset = IS_ENABLED(CONFIG_64BIT) ?
    0UL : PAGE_OFFSET - kernel_map.phys_addr;
    memory_limit = KERN_VIRT_SIZE;
// Sanity check alignment and size
    BUG_ON((PAGE_OFFSET % PGDIR_SIZE) != 0);
    BUG_ON((kernel_map.phys_addr % PMD_SIZE) != 0);

//
// The last 4K bytes of the addressable memory can not be mapped because
// of IS_ERR_VALUE macro.
//
    BUG_ON((kernel_map.virt_addr + kernel_map.size) > ADDRESS_SPACE_END - SZ_4K);

//
// Early page table uses only one PUD, which makes it possible
// to map PUD_SIZE aligned on PUD_SIZE: if the relocation offset
// makes the kernel cross over a PUD_SIZE boundary, raise a bug
// since a part of the kernel would not get mapped.
//
    if (IS_ENABLED(CONFIG_64BIT))
    BUG_ON(PUD_SIZE - (kernel_map.virt_addr & (PUD_SIZE - 1)) < kernel_map.size);
    relocate_kernel();

    apply_early_boot_alternatives();
    pt_ops_set_early();
// Setup early PGD for fixmap
    create_pgd_mapping(early_pg_dir, FIXADDR_START,
    fixmap_pgd_next, PGDIR_SIZE, PAGE_TABLE);

// Setup fixmap P4D and PUD
    if (pgtable_l5_enabled)
    create_p4d_mapping(fixmap_p4d, FIXADDR_START,
    (uintptr_t)fixmap_pud, P4D_SIZE, PAGE_TABLE);
// Setup fixmap PUD and PMD
    if (pgtable_l4_enabled)
    create_pud_mapping(fixmap_pud, FIXADDR_START,
    (uintptr_t)fixmap_pmd, PUD_SIZE, PAGE_TABLE);
    create_pmd_mapping(fixmap_pmd, FIXADDR_START,
    (uintptr_t)fixmap_pte, PMD_SIZE, PAGE_TABLE);
// Setup trampoline PGD and PMD
    create_pgd_mapping(trampoline_pg_dir, kernel_map.virt_addr,
    trampoline_pgd_next, PGDIR_SIZE, PAGE_TABLE);
    if (pgtable_l5_enabled)
    create_p4d_mapping(trampoline_p4d, kernel_map.virt_addr,
    (uintptr_t)trampoline_pud, P4D_SIZE, PAGE_TABLE);
    if (pgtable_l4_enabled)
    create_pud_mapping(trampoline_pud, kernel_map.virt_addr,
    (uintptr_t)trampoline_pmd, PUD_SIZE, PAGE_TABLE);
    create_pmd_mapping(trampoline_pmd, kernel_map.virt_addr,
    kernel_map.phys_addr, PMD_SIZE, PAGE_KERNEL_EXEC);

// Setup trampoline PGD
    create_pgd_mapping(trampoline_pg_dir, kernel_map.virt_addr,
    kernel_map.phys_addr, PGDIR_SIZE, PAGE_KERNEL_EXEC);

//
// Setup early PGD covering entire kernel which will allow
// us to reach paging_init(). We map all memory banks later
// in setup_vm_final() below.
//
    create_kernel_page_table(early_pg_dir, true);
// Setup early mapping for FDT early scan
    create_fdt_early_page_table(__fix_to_virt(FIX_FDT), dtb_pa);
//
// Bootime fixmap only can handle PMD_SIZE mapping. Thus, boot-ioremap
// range can not span multiple pmds.
//
    BUG_ON((__fix_to_virt(FIX_BTMAP_BEGIN) >> PMD_SHIFT)
    != (__fix_to_virt(FIX_BTMAP_END) >> PMD_SHIFT));

//
// Early ioremap fixmap is already created as it lies within first 2MB
// of fixmap region. We always map PMD_SIZE. Thus, both FIX_BTMAP_END
// FIX_BTMAP_BEGIN should lie in the same pmd. Verify that and warn
// the user if not.
//
    fix_bmap_spmd = fixmap_pmd[pmd_index(__fix_to_virt(FIX_BTMAP_BEGIN))];
    fix_bmap_epmd = fixmap_pmd[pmd_index(__fix_to_virt(FIX_BTMAP_END))];
    if (pmd_val(fix_bmap_spmd) != pmd_val(fix_bmap_epmd)) {
    WARN_ON(1);
    pr_warn("fixmap btmap start [%08lx] != end [%08lx]\n",
    pmd_val(fix_bmap_spmd), pmd_val(fix_bmap_epmd));
    pr_warn("fix_to_virt(FIX_BTMAP_BEGIN): %08lx\n",
    fix_to_virt(FIX_BTMAP_BEGIN));
    pr_warn("fix_to_virt(FIX_BTMAP_END):   %08lx\n",
    fix_to_virt(FIX_BTMAP_END));
    pr_warn("FIX_BTMAP_END:       %d\n", FIX_BTMAP_END);
    pr_warn("FIX_BTMAP_BEGIN:     %d\n", FIX_BTMAP_BEGIN);
    }

    pt_ops_set_fixmap();
    }
    static void __meminit create_linear_mapping_range(phys_addr_t start, phys_addr_t end,
    uintptr_t fixed_map_size, const pgprot_t *pgprot)
    {
    phys_addr_t pa;
    uintptr_t va, map_size;
    for (pa = start; pa < end; pa += map_size) {
    va = (uintptr_t)__va(pa);
    map_size = fixed_map_size ? fixed_map_size :
    best_map_size(pa, va, end - pa);
    create_pgd_mapping(swapper_pg_dir, va, pa, map_size,
    pgprot ? *pgprot : pgprot_from_va(va));
    }
    }
#[no_mangle]
unsafe extern "C" fn create_linear_mapping_page_table() -> void __init {
    static void __init create_linear_mapping_page_table(void)
    {
    phys_addr_t start, end;
    phys_addr_t kfence_pool __maybe_unused;
    u64 i;

    let mut ktext_start: phys_addr_t = __pa_symbol(_start);
    let mut ktext_size: phys_addr_t = __init_data_begin - _start;
    let mut krodata_start: phys_addr_t = __pa_symbol(__start_rodata);
    let mut krodata_size: phys_addr_t = _data - __start_rodata;
// Isolate kernel text and rodata so they don't get mapped with a PUD
    memblock_mark_nomap(ktext_start,  ktext_size);
    memblock_mark_nomap(krodata_start, krodata_size);

//
// kfence pool must be backed by PAGE_SIZE mappings, so allocate it
// before we setup the linear mapping so that we avoid using hugepages
// for this region.
//
    kfence_pool = memblock_phys_alloc(KFENCE_POOL_SIZE, PAGE_SIZE);
    BUG_ON(!kfence_pool);
    memblock_mark_nomap(kfence_pool, KFENCE_POOL_SIZE);
    __kfence_pool = __va(kfence_pool);

// Map all memory banks in the linear mapping
    for_each_mem_range(i, &start, &end) {
    if (start <= __pa(PAGE_OFFSET) &&
    __pa(PAGE_OFFSET) < end)
    start = __pa(PAGE_OFFSET);
    create_linear_mapping_range(start, end, 0, core::ptr::null_mut());
    }

    create_linear_mapping_range(ktext_start, ktext_start + ktext_size, 0, core::ptr::null_mut());
    create_linear_mapping_range(krodata_start, krodata_start + krodata_size, 0, core::ptr::null_mut());
    memblock_clear_nomap(ktext_start,  ktext_size);
    memblock_clear_nomap(krodata_start, krodata_size);

    create_linear_mapping_range(kfence_pool, kfence_pool + KFENCE_POOL_SIZE, PAGE_SIZE, core::ptr::null_mut());
    memblock_clear_nomap(kfence_pool, KFENCE_POOL_SIZE);

    }
#[no_mangle]
unsafe extern "C" fn setup_vm_final() -> void __init {
    static void __init setup_vm_final(void)
    {
// Setup swapper PGD for fixmap

//
// In 32-bit, the device tree lies in a pgd entry, so it must be copied
// directly in swapper_pg_dir in addition to the pgd entry that points
// to fixmap_pte.
//
    let mut idx: c_ulong = pgd_index(__fix_to_virt(FIX_FDT));
    set_pgd(&swapper_pg_dir[idx], early_pg_dir[idx]);

    create_pgd_mapping(swapper_pg_dir, FIXADDR_START,
    __pa_symbol(fixmap_pgd_next),
    PGDIR_SIZE, PAGE_TABLE);
// Map the linear mapping
    create_linear_mapping_page_table();
// Map the kernel
    if (IS_ENABLED(CONFIG_64BIT))
    create_kernel_page_table(swapper_pg_dir, false);

    kasan_swapper_init();

// Clear fixmap PTE and PMD mappings
    clear_fixmap(FIX_PTE);
    clear_fixmap(FIX_PMD);
    clear_fixmap(FIX_PUD);
    clear_fixmap(FIX_P4D);
// Move to swapper page table
    csr_write(CSR_SATP, PFN_DOWN(__pa_symbol(swapper_pg_dir)) | satp_mode);
    local_flush_tlb_all();
    pt_ops_set_late();
    }

#[no_mangle]
pub unsafe extern "C" fn setup_vm(dtb_pa: uintptr_t) -> asmlinkage void __init {
    asmlinkage void __init setup_vm(uintptr_t dtb_pa)
    {
    dtb_early_va = (void *)dtb_pa;
    dtb_early_pa = dtb_pa;

    kernel_map.virt_addr = (uintptr_t)_start;
    kernel_map.phys_addr = (uintptr_t)_start;
    relocate_kernel();

    }
#[no_mangle]
pub unsafe extern "C" fn setup_vm_final() {
    static inline void setup_vm_final(void)
    {
    }

//
// reserve_crashkernel() - reserves memory for crash kernel
//
// This function reserves memory area given in "crashkernel=" kernel command
// line parameter. The memory reserved is used by dump capture kernel when
// primary kernel is crashing.
//
#[no_mangle]
unsafe extern "C" fn arch_reserve_crashkernel() -> void __init {
    static void __init arch_reserve_crashkernel(void)
    {
    let mut low_size: c_ulonglong = 0, cma_size = 0;
    unsigned long long crash_base, crash_size;
    let mut high: bool = false;
    int ret;
    if (!IS_ENABLED(CONFIG_CRASH_RESERVE))
    return;
    ret = parse_crashkernel(boot_command_line, memblock_phys_mem_size(),
    &crash_size, &crash_base,
    &low_size, &cma_size, &high);
    if (ret)
    return;
    reserve_crashkernel_generic(crash_size, crash_base, low_size, high);
    reserve_crashkernel_cma(cma_size);
    }
#[no_mangle]
pub unsafe extern "C" fn paging_init() -> void __init {
    void __init paging_init(void)
    {
    setup_bootmem();
    setup_vm_final();
// Depend on that Linear Mapping is ready
    memblock_allow_resize();
    }
#[no_mangle]
pub unsafe extern "C" fn misc_mem_init() -> void __init {
    void __init misc_mem_init(void)
    {
    early_memtest(min_low_pfn << PAGE_SHIFT, max_low_pfn << PAGE_SHIFT);
    arch_numa_init();
    dma_contiguous_reserve(dma32_phys_limit);

// The entire VMEMMAP region has been populated. Flush TLB for this region
    local_flush_tlb_kernel_range(VMEMMAP_START, VMEMMAP_END);

    arch_reserve_crashkernel();
    memblock_dump_all();
    }

    int __meminit vmemmap_populate(unsigned long start, unsigned long end, int node,
    struct vmem_altmap *altmap)
    {
    WARN_ON((start < VMEMMAP_START) || (end > VMEMMAP_END));
//
// Note that SPARSEMEM_VMEMMAP is only selected for rv64 and that we
// can't use hugepage mappings for 2-level page table because in case of
// memory hotplug, we are not able to update all the page tables with
// the new PMDs.
//
    return vmemmap_populate_hugepages(start, end, node, altmap);
    }

//
// Pre-allocates page-table pages for a specific area in the kernel
// page-table. Only the level which needs to be synchronized between
// all page-tables is allocated because the synchronization can be
// expensive.
//
    static void __init preallocate_pgd_pages_range(unsigned long start, unsigned long end,
    const char *area)
    {
    unsigned long addr;
    const char *lvl;
    for (addr = start; addr < end && addr >= start; addr = ALIGN(addr + 1, PGDIR_SIZE)) {
    pgd_t *pgd = pgd_offset_k(addr);
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    lvl = "p4d";
    p4d = p4d_alloc(&init_mm, pgd, addr);
    if (!p4d)
    goto failed;
    if (pgtable_l5_enabled)
    continue;
    lvl = "pud";
    pud = pud_alloc(&init_mm, p4d, addr);
    if (!pud)
    goto failed;
    if (pgtable_l4_enabled)
    continue;
    lvl = "pmd";
    pmd = pmd_alloc(&init_mm, pud, addr);
    if (!pmd)
    goto failed;
    }
    return;
    failed:
//
// The pages have to be there now or they will be missing in
// process page-tables later.
//
    panic("Failed to pre-allocate %s pages for %s area\n", lvl, area);
    }

#[no_mangle]
pub unsafe extern "C" fn pgtable_cache_init() -> void __init {
    void __init pgtable_cache_init(void)
    {
    preallocate_pgd_pages_range(VMALLOC_START, VMALLOC_END, "vmalloc");
    if (IS_ENABLED(CONFIG_MODULES))
    preallocate_pgd_pages_range(MODULES_VADDR, MODULES_END, "bpf/modules");
    if (IS_ENABLED(CONFIG_MEMORY_HOTPLUG)) {
    preallocate_pgd_pages_range(VMEMMAP_START, VMEMMAP_END, "vmemmap");
    preallocate_pgd_pages_range(PAGE_OFFSET, PAGE_END, "direct map");
    if (IS_ENABLED(CONFIG_KASAN))
    preallocate_pgd_pages_range(KASAN_SHADOW_START, KASAN_SHADOW_END, "kasan");
    }
    }

    static struct execmem_info execmem_info __ro_after_init;
    struct execmem_info __init *execmem_arch_setup(void)
    {
    execmem_info = (struct execmem_info){
    .ranges = {
    [EXECMEM_DEFAULT] = {
    .start	= MODULES_VADDR,
    .end	= MODULES_END,
    .pgprot	= PAGE_KERNEL,
    .alignment = 1,
    },
    [EXECMEM_KPROBES] = {
    .start	= VMALLOC_START,
    .end	= VMALLOC_END,
    .pgprot	= PAGE_KERNEL_READ_EXEC,
    .alignment = 1,
    },
    [EXECMEM_BPF] = {
    .start	= BPF_JIT_REGION_START,
    .end	= BPF_JIT_REGION_END,
    .pgprot	= PAGE_KERNEL,
    .alignment = PAGE_SIZE,
    },
    },
    };
    return &execmem_info;
    }

#[no_mangle]
unsafe extern "C" fn free_pte_table(pte_start: *mut pte_t, pmd: *mut pmd_t) -> void __meminit {
    static void __meminit free_pte_table(pte_t *pte_start, pmd_t *pmd)
    {
    struct page *page = pmd_page(*pmd);
    struct ptdesc *ptdesc = page_ptdesc(page);
    pte_t *pte;
    int i;
    for (i = 0; i < PTRS_PER_PTE; i++) {
    pte = pte_start + i;
    if (!pte_none(*pte))
    return;
    }
    pagetable_dtor(ptdesc);
    if (PageReserved(page))
    free_reserved_page(page);
    else
    pagetable_free(ptdesc);
    pmd_clear(pmd);
    }
#[no_mangle]
unsafe extern "C" fn free_pmd_table(pmd_start: *mut pmd_t, pud: *mut pud_t, is_vmemmap: bool) -> void __meminit {
    static void __meminit free_pmd_table(pmd_t *pmd_start, pud_t *pud, bool is_vmemmap)
    {
    struct page *page = pud_page(*pud);
    struct ptdesc *ptdesc = page_ptdesc(page);
    pmd_t *pmd;
    int i;
    for (i = 0; i < PTRS_PER_PMD; i++) {
    pmd = pmd_start + i;
    if (!pmd_none(*pmd))
    return;
    }
    if (!is_vmemmap)
    pagetable_dtor(ptdesc);
    if (PageReserved(page))
    free_reserved_page(page);
    else
    pagetable_free(ptdesc);
    pud_clear(pud);
    }
#[no_mangle]
unsafe extern "C" fn free_pud_table(pud_start: *mut pud_t, p4d: *mut p4d_t) -> void __meminit {
    static void __meminit free_pud_table(pud_t *pud_start, p4d_t *p4d)
    {
    struct page *page = p4d_page(*p4d);
    pud_t *pud;
    int i;
    for (i = 0; i < PTRS_PER_PUD; i++) {
    pud = pud_start + i;
    if (!pud_none(*pud))
    return;
    }
    if (PageReserved(page))
    free_reserved_page(page);
    else
    __free_pages(page, 0);
    p4d_clear(p4d);
    }
    static void __meminit free_vmemmap_storage(struct page *page, size_t size,
    struct vmem_altmap *altmap)
    {
    let mut order: c_int = get_order(size);
    if (altmap) {
    vmem_altmap_free(altmap, size >> PAGE_SHIFT);
    return;
    }
    if (PageReserved(page)) {
    let mut nr_pages: c_uint = 1 << order;
    while (nr_pages--)
    free_reserved_page(page++);
    return;
    }
    __free_pages(page, order);
    }
    static void __meminit remove_pte_mapping(pte_t *pte_base, unsigned long addr, unsigned long end,
    bool is_vmemmap, struct vmem_altmap *altmap)
    {
    unsigned long next;
    pte_t *ptep, pte;
    for (; addr < end; addr = next) {
    next = (addr + PAGE_SIZE) & PAGE_MASK;
    if (next > end)
    next = end;
    ptep = pte_base + pte_index(addr);
    pte = ptep_get(ptep);
    if (!pte_present(*ptep))
    continue;
    pte_clear(&init_mm, addr, ptep);
    if (is_vmemmap)
    free_vmemmap_storage(pte_page(pte), PAGE_SIZE, altmap);
    }
    }
    static void __meminit remove_pmd_mapping(pmd_t *pmd_base, unsigned long addr, unsigned long end,
    bool is_vmemmap, struct vmem_altmap *altmap)
    {
    unsigned long next;
    pte_t *pte_base;
    pmd_t *pmdp, pmd;
    for (; addr < end; addr = next) {
    next = pmd_addr_end(addr, end);
    pmdp = pmd_base + pmd_index(addr);
    pmd = pmdp_get(pmdp);
    if (!pmd_present(pmd))
    continue;
    if (pmd_leaf(pmd)) {
    pmd_clear(pmdp);
    if (is_vmemmap)
    free_vmemmap_storage(pmd_page(pmd), PMD_SIZE, altmap);
    continue;
    }
    pte_base = (pte_t *)pmd_page_vaddr(*pmdp);
    remove_pte_mapping(pte_base, addr, next, is_vmemmap, altmap);
    free_pte_table(pte_base, pmdp);
    }
    }
    static void __meminit remove_pud_mapping(pud_t *pud_base, unsigned long addr, unsigned long end,
    bool is_vmemmap, struct vmem_altmap *altmap)
    {
    unsigned long next;
    pud_t *pudp, pud;
    pmd_t *pmd_base;
    for (; addr < end; addr = next) {
    next = pud_addr_end(addr, end);
    pudp = pgtable_l4_enabled ? pud_base + pud_index(addr) : pud_base;
    pud = pudp_get(pudp);
    if (!pud_present(pud))
    continue;
    if (pud_leaf(pud)) {
    if (pgtable_l4_enabled) {
    pud_clear(pudp);
    if (is_vmemmap)
    free_vmemmap_storage(pud_page(pud), PUD_SIZE, altmap);
    }
    continue;
    }
    pmd_base = pmd_offset(pudp, 0);
    remove_pmd_mapping(pmd_base, addr, next, is_vmemmap, altmap);
    if (pgtable_l4_enabled)
    free_pmd_table(pmd_base, pudp, is_vmemmap);
    }
    }
    static void __meminit remove_p4d_mapping(p4d_t *p4d_base, unsigned long addr, unsigned long end,
    bool is_vmemmap, struct vmem_altmap *altmap)
    {
    unsigned long next;
    p4d_t *p4dp, p4d;
    pud_t *pud_base;
    for (; addr < end; addr = next) {
    next = p4d_addr_end(addr, end);
    p4dp = pgtable_l5_enabled ? p4d_base + p4d_index(addr) : p4d_base;
    p4d = p4dp_get(p4dp);
    if (!p4d_present(p4d))
    continue;
    if (p4d_leaf(p4d)) {
    if (pgtable_l5_enabled) {
    p4d_clear(p4dp);
    if (is_vmemmap)
    free_vmemmap_storage(p4d_page(p4d), P4D_SIZE, altmap);
    }
    continue;
    }
    pud_base = pud_offset(p4dp, 0);
    remove_pud_mapping(pud_base, addr, next, is_vmemmap, altmap);
    if (pgtable_l5_enabled)
    free_pud_table(pud_base, p4dp);
    }
    }
    static void __meminit remove_pgd_mapping(unsigned long va, unsigned long end, bool is_vmemmap,
    struct vmem_altmap *altmap)
    {
    unsigned long addr, next;
    p4d_t *p4d_base;
    pgd_t *pgd;
    for (addr = va; addr < end; addr = next) {
    next = pgd_addr_end(addr, end);
    pgd = pgd_offset_k(addr);
    if (!pgd_present(*pgd))
    continue;
    if (pgd_leaf(*pgd))
    continue;
    p4d_base = p4d_offset(pgd, 0);
    remove_p4d_mapping(p4d_base, addr, next, is_vmemmap, altmap);
    }
    flush_tlb_all();
    }
#[no_mangle]
unsafe extern "C" fn remove_linear_mapping(start: phys_addr_t, size: u64) -> void __meminit {
    static void __meminit remove_linear_mapping(phys_addr_t start, u64 size)
    {
    let mut va: c_ulong = (unsigned long)__va(start);
    let mut end: c_ulong = (unsigned long)__va(start + size);
    remove_pgd_mapping(va, end, false, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_mappable_range() -> range {
    struct range arch_get_mappable_range(void)
    {
    struct range mhp_range;
    mhp_range.start = __pa(PAGE_OFFSET);
    mhp_range.end = __pa(PAGE_END - 1);
    return mhp_range;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_add_memory(nid: c_int, start: u64, size: u64, params: *mut mhp_params) -> int __ref {
    int __ref arch_add_memory(int nid, u64 start, u64 size, struct mhp_params *params)
    {
    let mut ret: c_int = 0;
    create_linear_mapping_range(start, start + size, 0, &params.pgprot);
    ret = __add_pages(nid, start >> PAGE_SHIFT, size >> PAGE_SHIFT, params);
    if (ret) {
    remove_linear_mapping(start, size);
    goto out;
    }
    max_pfn = PFN_UP(start + size);
    max_low_pfn = max_pfn;
    out:
    flush_tlb_all();
    return ret;
    }
    void __ref arch_remove_memory(u64 start, u64 size, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    __remove_pages(start >> PAGE_SHIFT, size >> PAGE_SHIFT, altmap, pgmap);
    remove_linear_mapping(start, size);
    flush_tlb_all();
    }
#[no_mangle]
pub unsafe extern "C" fn vmemmap_free(start: c_ulong, end: c_ulong, altmap: *mut vmem_altmap) -> void __ref {
    void __ref vmemmap_free(unsigned long start, unsigned long end, struct vmem_altmap *altmap)
    {
    remove_pgd_mapping(start, end, true, altmap);
    }
