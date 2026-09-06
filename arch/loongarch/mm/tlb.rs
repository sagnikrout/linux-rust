//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/tlb.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_all() {
    void local_flush_tlb_all(void)
    {
    invtlb_all(INVTLB_CURRENT_ALL, 0, 0);
    }
    EXPORT_SYMBOL(local_flush_tlb_all);
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_user() {
    void local_flush_tlb_user(void)
    {
    invtlb_all(INVTLB_CURRENT_GFALSE, 0, 0);
    }
    EXPORT_SYMBOL(local_flush_tlb_user);
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_kernel() {
    void local_flush_tlb_kernel(void)
    {
    invtlb_all(INVTLB_CURRENT_GTRUE, 0, 0);
    }
    EXPORT_SYMBOL(local_flush_tlb_kernel);
//
// All entries common to a mm share an asid. To effectively flush
// these entries, we just bump the asid.
//
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_mm(mm: *mut mm_struct) {
    void local_flush_tlb_mm(struct mm_struct *mm)
    {
    int cpu;
    preempt_disable();
    cpu = smp_processor_id();
    if (asid_valid(mm, cpu))
    drop_mmu_context(mm, cpu);
    else
    cpumask_clear_cpu(cpu, mm_cpumask(mm));
    preempt_enable();
    }
    void local_flush_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    struct mm_struct *mm = vma.vm_mm;
    let mut cpu: c_int = smp_processor_id();
    if (asid_valid(mm, cpu)) {
    unsigned long size, flags;
    local_irq_save(flags);
    start = round_down(start, PAGE_SIZE << 1);
    end = round_up(end, PAGE_SIZE << 1);
    size = (end - start) >> (PAGE_SHIFT + 1);
    if (size <= (current_cpu_data.tlbsizestlbsets ?
    current_cpu_data.tlbsize / 8 :
    current_cpu_data.tlbsize / 2)) {
    let mut asid: c_int = cpu_asid(cpu, mm);
    while (start < end) {
    invtlb(INVTLB_ADDR_GFALSE_AND_ASID, asid, start);
    start += (PAGE_SIZE << 1);
    }
    } else {
    drop_mmu_context(mm, cpu);
    }
    local_irq_restore(flags);
    } else {
    cpumask_clear_cpu(cpu, mm_cpumask(mm));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    void local_flush_tlb_kernel_range(unsigned long start, unsigned long end)
    {
    unsigned long size, flags;
    local_irq_save(flags);
    size = (end - start + (PAGE_SIZE - 1)) >> PAGE_SHIFT;
    size = (size + 1) >> 1;
    if (size <= (current_cpu_data.tlbsizestlbsets ?
    current_cpu_data.tlbsize / 8 :
    current_cpu_data.tlbsize / 2)) {
    start &= (PAGE_MASK << 1);
    end += ((PAGE_SIZE << 1) - 1);
    end &= (PAGE_MASK << 1);
    while (start < end) {
    invtlb_addr(INVTLB_ADDR_GTRUE_OR_ASID, 0, start);
    start += (PAGE_SIZE << 1);
    }
    } else {
    local_flush_tlb_kernel();
    }
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong) {
    void local_flush_tlb_page(struct vm_area_struct *vma, unsigned long page)
    {
    let mut cpu: c_int = smp_processor_id();
    if (asid_valid(vma.vm_mm, cpu)) {
    int newpid;
    newpid = cpu_asid(cpu, vma.vm_mm);
    page &= (PAGE_MASK << 1);
    invtlb(INVTLB_ADDR_GFALSE_AND_ASID, newpid, page);
    } else {
    cpumask_clear_cpu(cpu, mm_cpumask(vma.vm_mm));
    }
    }
//
// This one is only used for pages with the global bit set so we don't care
// much about the ASID.
//
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_one(page: c_ulong) {
    void local_flush_tlb_one(unsigned long page)
    {
    page &= (PAGE_MASK << 1);
    invtlb_addr(INVTLB_ADDR_GTRUE_OR_ASID, 0, page);
    }
#[no_mangle]
unsafe extern "C" fn __update_hugetlb(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) {
    static void __update_hugetlb(struct vm_area_struct *vma, unsigned long address, pte_t *ptep)
    {

    int idx;
    unsigned long lo;
    unsigned long flags;
    local_irq_save(flags);
    address &= (PAGE_MASK << 1);
    write_csr_entryhi(address);
    tlb_probe();
    idx = read_csr_tlbidx();
    write_csr_pagesize(PS_HUGE_SIZE);
    lo = pmd_to_entrylo(pte_val(*ptep));
    write_csr_entrylo0(lo);
    write_csr_entrylo1(lo + (HPAGE_SIZE >> 1));
    if (idx < 0)
    tlb_write_random();
    else
    tlb_write_indexed();
    write_csr_pagesize(PS_DEFAULT_SIZE);
    local_irq_restore(flags);

    }
#[no_mangle]
pub unsafe extern "C" fn __update_tlb(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) {
    void __update_tlb(struct vm_area_struct *vma, unsigned long address, pte_t *ptep)
    {
    int idx;
    unsigned long flags;
    if (cpu_has_ptw)
    return;
//
// Handle debugger faulting in for debugee.
//
    if (current.active_mm != vma.vm_mm)
    return;
    if (pte_val(*ptep) & _PAGE_HUGE) {
    __update_hugetlb(vma, address, ptep);
    return;
    }
    local_irq_save(flags);
    if ((unsigned long)ptep & sizeof(pte_t))
    ptep--;
    address &= (PAGE_MASK << 1);
    write_csr_entryhi(address);
    tlb_probe();
    idx = read_csr_tlbidx();
    write_csr_pagesize(PS_DEFAULT_SIZE);
    write_csr_entrylo0(pte_val(*ptep++));
    write_csr_entrylo1(pte_val(*ptep));
    if (idx < 0)
    tlb_write_random();
    else
    tlb_write_indexed();
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn setup_ptwalker() -> void __no_sanitize_address {
    static void __no_sanitize_address setup_ptwalker(void)
    {
    unsigned long pwctl0, pwctl1;
    let mut pgd_i: c_ulong = 0, pgd_w = 0;
    let mut pud_i: c_ulong = 0, pud_w = 0;
    let mut pmd_i: c_ulong = 0, pmd_w = 0;
    let mut pte_i: c_ulong = 0, pte_w = 0;
    pgd_i = PGDIR_SHIFT;
    pgd_w = PAGE_SHIFT - 3;

    pud_i = PUD_SHIFT;
    pud_w = PAGE_SHIFT - 3;

    pmd_i = PMD_SHIFT;
    pmd_w = PAGE_SHIFT - 3;

    pte_i = PAGE_SHIFT;
    pte_w = PAGE_SHIFT - 3;
    pwctl0 = pte_i | pte_w << 5 | pmd_i << 10 | pmd_w << 15 | pud_i << 20 | pud_w << 25;
    pwctl1 = pgd_i | pgd_w << 6;
    if (cpu_has_ptw)
    pwctl1 |= CSR_PWCTL1_PTW;
    csr_write(pwctl0, LOONGARCH_CSR_PWCTL0);
    csr_write(pwctl1, LOONGARCH_CSR_PWCTL1);
    csr_write((long)swapper_pg_dir, LOONGARCH_CSR_PGDH);
    csr_write((long)invalid_pg_dir, LOONGARCH_CSR_PGDL);
    csr_write((long)smp_processor_id(), LOONGARCH_CSR_TMID);
    }
#[no_mangle]
unsafe extern "C" fn output_pgtable_bits_defines() {
    static void output_pgtable_bits_defines(void)
    {

    pr_debug("#define " fmt, ##__VA_ARGS__)
    pr_debug("#include <asm/asm.h>\n");
    pr_debug("#include <asm/regdef.h>\n");
    pr_debug("\n");
    pr_define("_PAGE_VALID_SHIFT %d\n", _PAGE_VALID_SHIFT);
    pr_define("_PAGE_DIRTY_SHIFT %d\n", _PAGE_DIRTY_SHIFT);
    pr_define("_PAGE_HUGE_SHIFT %d\n", _PAGE_HUGE_SHIFT);
    pr_define("_PAGE_GLOBAL_SHIFT %d\n", _PAGE_GLOBAL_SHIFT);
    pr_define("_PAGE_PRESENT_SHIFT %d\n", _PAGE_PRESENT_SHIFT);
    pr_define("_PAGE_WRITE_SHIFT %d\n", _PAGE_WRITE_SHIFT);

    pr_define("_PAGE_NO_READ_SHIFT %d\n", _PAGE_NO_READ_SHIFT);
    pr_define("_PAGE_NO_EXEC_SHIFT %d\n", _PAGE_NO_EXEC_SHIFT);

    pr_define("PFN_PTE_SHIFT %d\n", PFN_PTE_SHIFT);
    pr_debug("\n");
    }

    unsigned long pcpu_handlers[NR_CPUS];

#[no_mangle]
unsafe extern "C" fn setup_tlb_handler(cpu: c_int) {
    static void setup_tlb_handler(int cpu)
    {
    setup_ptwalker();
    local_flush_tlb_all();
    if (cpu_has_ptw) {
    exception_table[EXCCODE_TLBI] = handle_tlb_load_ptw;
    exception_table[EXCCODE_TLBL] = handle_tlb_load_ptw;
    exception_table[EXCCODE_TLBS] = handle_tlb_store_ptw;
    exception_table[EXCCODE_TLBM] = handle_tlb_modify_ptw;
    }
// The tlb handlers are generated only once
    if (cpu == 0) {
    memcpy((void *)tlbrentry, handle_tlb_refill, 0x80);
    local_flush_icache_range(tlbrentry, tlbrentry + 0x80);
    for (int i = EXCCODE_TLBL; i <= EXCCODE_TLBPE; i++)
    set_handler(i * VECSIZE, exception_table[i], VECSIZE);
    } else {
    int vec_sz __maybe_unused;
    void *addr __maybe_unused;
    struct page *page __maybe_unused;
// Avoid lockdep warning
    rcutree_report_cpu_starting(cpu);

    vec_sz = sizeof(exception_handlers);
    if (pcpu_handlers[cpu])
    return;
    page = alloc_pages_node(cpu_to_node(cpu), GFP_ATOMIC, get_order(vec_sz));
    if (!page)
    return;
    addr = page_address(page);
    pcpu_handlers[cpu] = (unsigned long)addr;
    memcpy((void *)addr, (void *)eentry, vec_sz);
    local_flush_icache_range((unsigned long)addr, (unsigned long)addr + vec_sz);
    csr_write64(pcpu_handlers[cpu], LOONGARCH_CSR_EENTRY);
    csr_write64(pcpu_handlers[cpu], LOONGARCH_CSR_MERRENTRY);
    csr_write64(pcpu_handlers[cpu] + 80*VECSIZE, LOONGARCH_CSR_TLBRENTRY);

    }
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_init(cpu: c_int) {
    void tlb_init(int cpu)
    {
    write_csr_pagesize(PS_DEFAULT_SIZE);
    write_csr_stlbpgsize(PS_DEFAULT_SIZE);
    write_csr_tlbrefill_pagesize(PS_DEFAULT_SIZE);
    setup_tlb_handler(cpu);
    output_pgtable_bits_defines();
    }
