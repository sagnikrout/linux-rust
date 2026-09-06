//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/pgtable.c
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

    let mut __ro_after_init: phys_addr_t physical_mask = (1ULL << __PHYSICAL_MASK_SHIFT) - 1;
    EXPORT_SYMBOL(physical_mask);
    SYM_PIC_ALIAS(physical_mask);

#[no_mangle]
pub unsafe extern "C" fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    pgtable_t pte_alloc_one(struct mm_struct *mm)
    {
    return __pte_alloc_one(mm, GFP_PGTABLE_USER);
    }
#[no_mangle]
pub unsafe extern "C" fn ___pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page) {
    void ___pte_free_tlb(struct mmu_gather *tlb, struct page *pte)
    {
    paravirt_release_pte(page_to_pfn(pte));
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
    }

#[no_mangle]
pub unsafe extern "C" fn ___pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t) {
    void ___pmd_free_tlb(struct mmu_gather *tlb, pmd_t *pmd)
    {
    paravirt_release_pmd(__pa(pmd) >> PAGE_SHIFT);
//
// NOTE! For PAE, any changes to the top page-directory-pointer-table
// entries need a full cr3 reload to flush.
//

    tlb.need_flush_all = 1;

    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pmd));
    }

#[no_mangle]
pub unsafe extern "C" fn ___pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t) {
    void ___pud_free_tlb(struct mmu_gather *tlb, pud_t *pud)
    {
    paravirt_release_pud(__pa(pud) >> PAGE_SHIFT);
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(pud));
    }

#[no_mangle]
pub unsafe extern "C" fn ___p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t) {
    void ___p4d_free_tlb(struct mmu_gather *tlb, p4d_t *p4d)
    {
    paravirt_release_p4d(__pa(p4d) >> PAGE_SHIFT);
    tlb_remove_ptdesc(tlb, virt_to_ptdesc(p4d));
    }

#[no_mangle]
pub unsafe extern "C" fn pgd_list_add(pgd: *mut pgd_t) {
    static inline void pgd_list_add(pgd_t *pgd)
    {
    struct ptdesc *ptdesc = virt_to_ptdesc(pgd);
    list_add(&ptdesc.pt_list, &pgd_list);
    }
#[no_mangle]
pub unsafe extern "C" fn pgd_list_del(pgd: *mut pgd_t) {
    static inline void pgd_list_del(pgd_t *pgd)
    {
    struct ptdesc *ptdesc = virt_to_ptdesc(pgd);
    list_del(&ptdesc.pt_list);
    }
#[no_mangle]
unsafe extern "C" fn pgd_set_mm(pgd: *mut pgd_t, mm: *mut mm_struct) {
    static void pgd_set_mm(pgd_t *pgd, struct mm_struct *mm)
    {
    virt_to_ptdesc(pgd).pt_mm = mm;
    }
    struct mm_struct *pgd_page_get_mm(struct ptdesc *pt)
    {
    return pt.pt_mm;
    }
#[no_mangle]
unsafe extern "C" fn pgd_ctor(mm: *mut mm_struct, pgd: *mut pgd_t) {
    static void pgd_ctor(struct mm_struct *mm, pgd_t *pgd)
    {
// PAE preallocates all its PMDs.  No cloning needed.
    if (!IS_ENABLED(CONFIG_X86_PAE))
    clone_pgd_range(pgd + KERNEL_PGD_BOUNDARY,
    swapper_pg_dir + KERNEL_PGD_BOUNDARY,
    KERNEL_PGD_PTRS);
// List used to sync kernel mapping updates
    pgd_set_mm(pgd, mm);
    pgd_list_add(pgd);
    }
#[no_mangle]
unsafe extern "C" fn pgd_dtor(pgd: *mut pgd_t) {
    static void pgd_dtor(pgd_t *pgd)
    {
    spin_lock(&pgd_lock);
    pgd_list_del(pgd);
    spin_unlock(&pgd_lock);
    }

//
// In PAE mode, we need to do a cr3 reload (=tlb flush) when
// updating the top-level pagetable entries to guarantee the
// processor notices the update.  Since this is expensive, and
// all 4 top-level entries are used almost immediately in a
// new process's life, we just pre-populate them here.
//

//
// "USER_PMDS" are the PMDs for the user copy of the page tables when
// PTI is enabled. They do not exist when PTI is disabled.  Note that
// this is distinct from the user _portion_ of the kernel page tables
// which always exists.
//
// We allocate separate PMDs for the kernel part of the user page-table
// when PTI is enabled. We need them to map the per-process LDT into the
// user-space page-table.
//

    KERNEL_PGD_PTRS : 0)

#[no_mangle]
pub unsafe extern "C" fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmd: *mut pmd_t) {
    void pud_populate(struct mm_struct *mm, pud_t *pudp, pmd_t *pmd)
    {
    paravirt_alloc_pmd(mm, __pa(pmd) >> PAGE_SHIFT);
// Note: almost everything apart from _PAGE_PRESENT is
    reserved at the pmd (PDPT) level. */
    set_pud(pudp, __pud(__pa(pmd) | _PAGE_PRESENT));
//
// According to Intel App note "TLBs, Paging-Structure Caches,
// and Their Invalidation", April 2007, document 317080-001,
// section 8.1: in PAE mode we explicitly have to flush the
// TLB via cr3 if the top-level pgd is changed...
//
    flush_tlb_mm(mm);
    }

// No need to prepopulate any pagetable entries in non-PAE modes.
pub const PREALLOCATED_PMDS: c_int = 0;
pub const PREALLOCATED_USER_PMDS: c_int = 0;
pub const MAX_PREALLOCATED_USER_PMDS: c_int = 0;

#[no_mangle]
unsafe extern "C" fn free_pmds(mm: *mut mm_struct, pmds[]: *mut pmd_t, count: c_int) {
    static void free_pmds(struct mm_struct *mm, pmd_t *pmds[], int count)
    {
    int i;
    struct ptdesc *ptdesc;
    for (i = 0; i < count; i++)
    if (pmds[i]) {
    ptdesc = virt_to_ptdesc(pmds[i]);
    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
    mm_dec_nr_pmds(mm);
    }
    }
#[no_mangle]
unsafe extern "C" fn preallocate_pmds(mm: *mut mm_struct, pmds[]: *mut pmd_t, count: c_int) -> c_int {
    static int preallocate_pmds(struct mm_struct *mm, pmd_t *pmds[], int count)
    {
    int i;
    let mut failed: bool = false;
    let mut gfp: gfp_t = GFP_PGTABLE_USER;
    if (mm == &init_mm)
    gfp &= ~__GFP_ACCOUNT;
    gfp &= ~__GFP_HIGHMEM;
    for (i = 0; i < count; i++) {
    pmd_t *pmd = core::ptr::null_mut();
    struct ptdesc *ptdesc = pagetable_alloc(gfp, 0);
    if (!ptdesc)
    failed = true;
    if (ptdesc && !pagetable_pmd_ctor(mm, ptdesc)) {
    pagetable_free(ptdesc);
    ptdesc = core::ptr::null_mut();
    failed = true;
    }
    if (ptdesc) {
    mm_inc_nr_pmds(mm);
    pmd = ptdesc_address(ptdesc);
    }
    pmds[i] = pmd;
    }
    if (failed) {
    free_pmds(mm, pmds, count);
    return -ENOMEM;
    }
    return 0;
    }
//
// Mop up any pmd pages which may still be attached to the pgd.
// Normally they will be freed by munmap/exit_mmap, but any pmd we
// preallocate which never got a corresponding vma will need to be
// freed manually.
//
#[no_mangle]
unsafe extern "C" fn mop_up_one_pmd(mm: *mut mm_struct, pgdp: *mut pgd_t) {
    static void mop_up_one_pmd(struct mm_struct *mm, pgd_t *pgdp)
    {
    let mut pgd: pgd_t = *pgdp;
    if (pgd_val(pgd) != 0) {
    pmd_t *pmd = (pmd_t *)pgd_page_vaddr(pgd);
    pgd_clear(pgdp);
    paravirt_release_pmd(pgd_val(pgd) >> PAGE_SHIFT);
    pmd_free(mm, pmd);
    mm_dec_nr_pmds(mm);
    }
    }
#[no_mangle]
unsafe extern "C" fn pgd_mop_up_pmds(mm: *mut mm_struct, pgdp: *mut pgd_t) {
    static void pgd_mop_up_pmds(struct mm_struct *mm, pgd_t *pgdp)
    {
    int i;
    for (i = 0; i < PREALLOCATED_PMDS; i++)
    mop_up_one_pmd(mm, &pgdp[i]);

    if (!boot_cpu_has(X86_FEATURE_PTI))
    return;
    pgdp = kernel_to_user_pgdp(pgdp);
    for (i = 0; i < PREALLOCATED_USER_PMDS; i++)
    mop_up_one_pmd(mm, &pgdp[i + KERNEL_PGD_BOUNDARY]);

    }
#[no_mangle]
unsafe extern "C" fn pgd_prepopulate_pmd(mm: *mut mm_struct, pgd: *mut pgd_t, pmds[]: *mut pmd_t) {
    static void pgd_prepopulate_pmd(struct mm_struct *mm, pgd_t *pgd, pmd_t *pmds[])
    {
    p4d_t *p4d;
    pud_t *pud;
    int i;
    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    for (i = 0; i < PREALLOCATED_PMDS; i++, pud++) {
    pmd_t *pmd = pmds[i];
    if (i >= KERNEL_PGD_BOUNDARY)
    memcpy(pmd, (pmd_t *)pgd_page_vaddr(swapper_pg_dir[i]),
    sizeof(pmd_t) * PTRS_PER_PMD);
    pud_populate(mm, pud, pmd);
    }
    }

    static void pgd_prepopulate_user_pmd(struct mm_struct *mm,
    pgd_t *k_pgd, pmd_t *pmds[])
    {
    pgd_t *s_pgd = kernel_to_user_pgdp(swapper_pg_dir);
    pgd_t *u_pgd = kernel_to_user_pgdp(k_pgd);
    p4d_t *u_p4d;
    pud_t *u_pud;
    int i;
    u_p4d = p4d_offset(u_pgd, 0);
    u_pud = pud_offset(u_p4d, 0);
    s_pgd += KERNEL_PGD_BOUNDARY;
    u_pud += KERNEL_PGD_BOUNDARY;
    for (i = 0; i < PREALLOCATED_USER_PMDS; i++, u_pud++, s_pgd++) {
    pmd_t *pmd = pmds[i];
    memcpy(pmd, (pmd_t *)pgd_page_vaddr(*s_pgd),
    sizeof(pmd_t) * PTRS_PER_PMD);
    pud_populate(mm, u_pud, pmd);
    }
    }

    static void pgd_prepopulate_user_pmd(struct mm_struct *mm,
    pgd_t *k_pgd, pmd_t *pmds[])
    {
    }

    static inline pgd_t *_pgd_alloc(struct mm_struct *mm)
    {
//
// PTI and Xen need a whole page for the PAE PGD
// even though the hardware only needs 32 bytes.
//
// For simplicity, allocate a page for all users.
//
    return __pgd_alloc(mm, pgd_allocation_order());
    }
#[no_mangle]
pub unsafe extern "C" fn _pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    static inline void _pgd_free(struct mm_struct *mm, pgd_t *pgd)
    {
    __pgd_free(mm, pgd);
    }
    pgd_t *pgd_alloc(struct mm_struct *mm)
    {
    pgd_t *pgd;
    pmd_t *u_pmds[MAX_PREALLOCATED_USER_PMDS];
    pmd_t *pmds[PREALLOCATED_PMDS];
    pgd = _pgd_alloc(mm);
    if (pgd == core::ptr::null_mut())
    goto out;
    mm.pgd = pgd;
    if (sizeof(pmds) != 0 &&
    preallocate_pmds(mm, pmds, PREALLOCATED_PMDS) != 0)
    goto out_free_pgd;
    if (sizeof(u_pmds) != 0 &&
    preallocate_pmds(mm, u_pmds, PREALLOCATED_USER_PMDS) != 0)
    goto out_free_pmds;
    if (paravirt_pgd_alloc(mm) != 0)
    goto out_free_user_pmds;
//
// Make sure that pre-populating the pmds is atomic with
// respect to anything walking the pgd_list, so that they
// never see a partially populated pgd.
//
    spin_lock(&pgd_lock);
    pgd_ctor(mm, pgd);
    if (sizeof(pmds) != 0)
    pgd_prepopulate_pmd(mm, pgd, pmds);
    if (sizeof(u_pmds) != 0)
    pgd_prepopulate_user_pmd(mm, pgd, u_pmds);
    spin_unlock(&pgd_lock);
    return pgd;
    out_free_user_pmds:
    if (sizeof(u_pmds) != 0)
    free_pmds(mm, u_pmds, PREALLOCATED_USER_PMDS);
    out_free_pmds:
    if (sizeof(pmds) != 0)
    free_pmds(mm, pmds, PREALLOCATED_PMDS);
    out_free_pgd:
    _pgd_free(mm, pgd);
    out:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    void pgd_free(struct mm_struct *mm, pgd_t *pgd)
    {
    pgd_mop_up_pmds(mm, pgd);
    pgd_dtor(pgd);
    paravirt_pgd_free(mm, pgd);
    _pgd_free(mm, pgd);
    }
//
// Used to set accessed or dirty bits in the page table entries
// on other architectures. On x86, the accessed and dirty bits
// are tracked by hardware. However, do_wp_page calls this function
// to also make the pte writeable at the same time the dirty bit is
// set. In that case we do actually need to write the PTE.
//
    int ptep_set_access_flags(struct vm_area_struct *vma,
    unsigned long address, pte_t *ptep,
    pte_t entry, int dirty)
    {
    let mut changed: c_int = !pte_same(*ptep, entry);
    if (changed && dirty)
    set_pte(ptep, entry);
    return changed;
    }

    int pmdp_set_access_flags(struct vm_area_struct *vma,
    unsigned long address, pmd_t *pmdp,
    pmd_t entry, int dirty)
    {
    let mut changed: c_int = !pmd_same(*pmdp, entry);
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    if (changed && dirty) {
    set_pmd(pmdp, entry);
//
// We had a write-protection fault here and changed the pmd
// to to more permissive. No need to flush the TLB for that,
// #PF is architecturally guaranteed to do that and in the
// worst-case we'll generate a spurious fault.
//
    }
    return changed;
    }
    int pudp_set_access_flags(struct vm_area_struct *vma, unsigned long address,
    pud_t *pudp, pud_t entry, int dirty)
    {
    let mut changed: c_int = !pud_same(*pudp, entry);
    VM_BUG_ON(address & ~HPAGE_PUD_MASK);
    if (changed && dirty) {
    set_pud(pudp, entry);
//
// We had a write-protection fault here and changed the pud
// to to more permissive. No need to flush the TLB for that,
// #PF is architecturally guaranteed to do that and in the
// worst-case we'll generate a spurious fault.
//
    }
    return changed;
    }

    bool ptep_test_and_clear_young(struct vm_area_struct *vma,
    unsigned long addr, pte_t *ptep)
    {
    let mut ret: bool = false;
    if (pte_young(*ptep))
    ret = test_and_clear_bit(_PAGE_BIT_ACCESSED,
    (unsigned long *) &ptep.pte);
    return ret;
    }

    bool pmdp_test_and_clear_young(struct vm_area_struct *vma,
    unsigned long addr, pmd_t *pmdp)
    {
    let mut ret: bool = false;
    if (pmd_young(*pmdp))
    ret = test_and_clear_bit(_PAGE_BIT_ACCESSED,
    (unsigned long *)pmdp);
    return ret;
    }

    bool pudp_test_and_clear_young(struct vm_area_struct *vma,
    unsigned long addr, pud_t *pudp)
    {
    let mut ret: bool = false;
    if (pud_young(*pudp))
    ret = test_and_clear_bit(_PAGE_BIT_ACCESSED,
    (unsigned long *)pudp);
    return ret;
    }

    bool ptep_clear_flush_young(struct vm_area_struct *vma,
    unsigned long address, pte_t *ptep)
    {
//
// On x86 CPUs, clearing the accessed bit without a TLB flush
// doesn't cause data corruption. [ It could cause incorrect
// page aging and the (mistaken) reclaim of hot pages, but the
// chance of that should be relatively low. ]
//
// So as a performance optimization don't flush the TLB when
// clearing the accessed bit, it will eventually be flushed by
// a context switch or a VM operation anyway. [ In the rare
// event of it not getting flushed for a long time the delay
// shouldn't really matter because there's no real memory
// pressure for swapout to react to. ]
//
    return ptep_test_and_clear_young(vma, address, ptep);
    }

    bool pmdp_clear_flush_young(struct vm_area_struct *vma,
    unsigned long address, pmd_t *pmdp)
    {
    bool young;
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    young = pmdp_test_and_clear_young(vma, address, pmdp);
    if (young)
    flush_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    return young;
    }
    pmd_t pmdp_invalidate_ad(struct vm_area_struct *vma, unsigned long address,
    pmd_t *pmdp)
    {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
//
// No flush is necessary. Once an invalid PTE is established, the PTE's
// access and dirty bits cannot be updated.
//
    return pmdp_establish(vma, address, pmdp, pmd_mkinvalid(*pmdp));
    }

    defined(CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)
    pud_t pudp_invalidate(struct vm_area_struct *vma, unsigned long address,
    pud_t *pudp)
    {
    VM_WARN_ON_ONCE(!pud_present(*pudp));
    let mut old: pud_t = pudp_establish(vma, address, pudp, pud_mkinvalid(*pudp));
    flush_pud_tlb_range(vma, address, address + HPAGE_PUD_SIZE);
    return old;
    }

//
// reserve_top_address - Reserve a hole in the top of the kernel address space
// @reserve: Size of hole to reserve
//
// Can be used to relocate the fixmap area and poke a hole in the top
// of the kernel address space to make room for a hypervisor.
//
#[no_mangle]
pub unsafe extern "C" fn reserve_top_address(reserve: c_ulong) -> void __init {
    void __init reserve_top_address(unsigned long reserve)
    {

    BUG_ON(fixmaps_set > 0);
    __FIXADDR_TOP = round_down(-reserve, 1 << PMD_SHIFT) - PAGE_SIZE;
    printk(KERN_INFO "Reserving virtual address space above 0x%08lx (rounded to 0x%08lx)\n",
    -reserve, __FIXADDR_TOP + PAGE_SIZE);

    }
    int fixmaps_set;
#[no_mangle]
pub unsafe extern "C" fn __native_set_fixmap(idx: enum fixed_addresses, pte: pte_t) {
    void __native_set_fixmap(enum fixed_addresses idx, pte_t pte)
    {
    let mut address: c_ulong = __fix_to_virt(idx);

//
// Ensure that the static initial page tables are covering the
// fixmap completely.
//
    BUILD_BUG_ON(__end_of_permanent_fixed_addresses >
    (FIXMAP_PMD_NUM * PTRS_PER_PTE));

    if (idx >= __end_of_fixed_addresses) {
    BUG();
    return;
    }
    set_pte_vaddr(address, pte);
    fixmaps_set++;
    }
    void native_set_fixmap(unsigned /* enum fixed_addresses */ idx,
    phys_addr_t phys, pgprot_t flags)
    {
// Sanitize 'prot' against any unsupported bits:
    pgprot_val(flags) &= __default_kernel_pte_mask;
    __native_set_fixmap(idx, pfn_pte(phys >> PAGE_SHIFT, flags));
    }

//
// p4d_set_huge - Set up kernel P4D mapping
// @p4d: Pointer to the P4D entry
// @addr: Virtual address associated with the P4D entry
// @prot: Protection bits to use
//
// No 512GB pages yet -- always return 0
//
#[no_mangle]
pub unsafe extern "C" fn p4d_set_huge(p4d: *mut p4d_t, addr: phys_addr_t, prot: pgprot_t) -> c_int {
    int p4d_set_huge(p4d_t *p4d, phys_addr_t addr, pgprot_t prot)
    {
    return 0;
    }
//
// p4d_clear_huge - Clear kernel P4D mapping when it is set
// @p4d: Pointer to the P4D entry to clear
//
// No 512GB pages yet -- do nothing
//
#[no_mangle]
pub unsafe extern "C" fn p4d_clear_huge(p4d: *mut p4d_t) {
    void p4d_clear_huge(p4d_t *p4d)
    {
    }

//
// pud_set_huge - Set up kernel PUD mapping
// @pud: Pointer to the PUD entry
// @addr: Virtual address associated with the PUD entry
// @prot: Protection bits to use
//
// MTRRs can override PAT memory types with 4KiB granularity. Therefore, this
// function sets up a huge page only if the complete range has the same MTRR
// caching mode.
//
// Callers should try to decrease page size (1GB -> 2MB -> 4K) if the bigger
// page mapping attempt fails.
//
// Returns 1 on success and 0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn pud_set_huge(pud: *mut pud_t, addr: phys_addr_t, prot: pgprot_t) -> c_int {
    int pud_set_huge(pud_t *pud, phys_addr_t addr, pgprot_t prot)
    {
    u8 uniform;
    mtrr_type_lookup(addr, addr + PUD_SIZE, &uniform);
    if (!uniform)
    return 0;
// Bail out if we are we on a populated non-leaf entry:
    if (pud_present(*pud) && !pud_leaf(*pud))
    return 0;
    set_pte((pte_t *)pud, pfn_pte(
    (u64)addr >> PAGE_SHIFT,
    __pgprot(protval_4k_2_large(pgprot_val(prot)) | _PAGE_PSE)));
    return 1;
    }
//
// pmd_set_huge - Set up kernel PMD mapping
// @pmd: Pointer to the PMD entry
// @addr: Virtual address associated with the PMD entry
// @prot: Protection bits to use
//
// See text over pud_set_huge() above.
//
// Returns 1 on success and 0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn pmd_set_huge(pmd: *mut pmd_t, addr: phys_addr_t, prot: pgprot_t) -> c_int {
    int pmd_set_huge(pmd_t *pmd, phys_addr_t addr, pgprot_t prot)
    {
    u8 uniform;
    mtrr_type_lookup(addr, addr + PMD_SIZE, &uniform);
    if (!uniform) {
    pr_warn_once("%s: Cannot satisfy [mem %#010llx-%#010llx] with a huge-page mapping due to MTRR override.\n",
    __func__, addr, addr + PMD_SIZE);
    return 0;
    }
// Bail out if we are we on a populated non-leaf entry:
    if (pmd_present(*pmd) && !pmd_leaf(*pmd))
    return 0;
    set_pte((pte_t *)pmd, pfn_pte(
    (u64)addr >> PAGE_SHIFT,
    __pgprot(protval_4k_2_large(pgprot_val(prot)) | _PAGE_PSE)));
    return 1;
    }
//
// pud_clear_huge - Clear kernel PUD mapping when it is set
// @pud: Pointer to the PUD entry to clear.
//
// Returns 1 on success and 0 on failure (no PUD map is found).
//
#[no_mangle]
pub unsafe extern "C" fn pud_clear_huge(pud: *mut pud_t) -> c_int {
    int pud_clear_huge(pud_t *pud)
    {
    if (pud_leaf(*pud)) {
    pud_clear(pud);
    return 1;
    }
    return 0;
    }
//
// pmd_clear_huge - Clear kernel PMD mapping when it is set
// @pmd: Pointer to the PMD entry to clear.
//
// Returns 1 on success and 0 on failure (no PMD map is found).
//
#[no_mangle]
pub unsafe extern "C" fn pmd_clear_huge(pmd: *mut pmd_t) -> c_int {
    int pmd_clear_huge(pmd_t *pmd)
    {
    if (pmd_leaf(*pmd)) {
    pmd_clear(pmd);
    return 1;
    }
    return 0;
    }

//
// pud_free_pmd_page - Clear PUD entry and free PMD page
// @pud: Pointer to a PUD
// @addr: Virtual address associated with PUD
//
// Context: The PUD range has been unmapped and TLB purged.
// Return: 1 if clearing the entry succeeded. 0 otherwise.
//
// NOTE: Callers must allow a single page allocation.
//
#[no_mangle]
pub unsafe extern "C" fn pud_free_pmd_page(pud: *mut pud_t, addr: c_ulong) -> c_int {
    int pud_free_pmd_page(pud_t *pud, unsigned long addr)
    {
    pmd_t *pmd, *pmd_sv;
    struct ptdesc *pt;
    int i;
    pmd = pud_pgtable(*pud);
    pmd_sv = (pmd_t *)__get_free_page(GFP_KERNEL);
    if (!pmd_sv)
    return 0;
    for (i = 0; i < PTRS_PER_PMD; i++) {
    pmd_sv[i] = pmd[i];
    if (!pmd_none(pmd[i]))
    pmd_clear(&pmd[i]);
    }
    pud_clear(pud);
// INVLPG to clear all paging-structure caches
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE-1);
    for (i = 0; i < PTRS_PER_PMD; i++) {
    if (!pmd_none(pmd_sv[i])) {
    pt = page_ptdesc(pmd_page(pmd_sv[i]));
    pagetable_dtor_free(pt);
    }
    }
    free_page((unsigned long)pmd_sv);
    pmd_free(&init_mm, pmd);
    return 1;
    }
//
// pmd_free_pte_page - Clear PMD entry and free PTE page.
// @pmd: Pointer to the PMD
// @addr: Virtual address associated with PMD
//
// Context: The PMD range has been unmapped and TLB purged.
// Return: 1 if clearing the entry succeeded. 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pmd_free_pte_page(pmd: *mut pmd_t, addr: c_ulong) -> c_int {
    int pmd_free_pte_page(pmd_t *pmd, unsigned long addr)
    {
    struct ptdesc *pt;
    pt = page_ptdesc(pmd_page(*pmd));
    pmd_clear(pmd);
// INVLPG to clear all paging-structure caches
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE-1);
    pagetable_dtor_free(pt);
    return 1;
    }

//
// Disable free page handling on x86-PAE. This assures that ioremap()
// does not update sync'd PMD entries. See vmalloc_sync_one().
//
#[no_mangle]
pub unsafe extern "C" fn pmd_free_pte_page(pmd: *mut pmd_t, addr: c_ulong) -> c_int {
    int pmd_free_pte_page(pmd_t *pmd, unsigned long addr)
    {
    return pmd_none(*pmd);
    }

#[no_mangle]
pub unsafe extern "C" fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t {
    pte_t pte_mkwrite(pte_t pte, struct vm_area_struct *vma)
    {
    if (vma.vm_flags & VM_SHADOW_STACK)
    return pte_mkwrite_shstk(pte);
    pte = pte_mkwrite_novma(pte);
    return pte_clear_saveddirty(pte);
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t {
    pmd_t pmd_mkwrite(pmd_t pmd, struct vm_area_struct *vma)
    {
    if (vma.vm_flags & VM_SHADOW_STACK)
    return pmd_mkwrite_shstk(pmd);
    pmd = pmd_mkwrite_novma(pmd);
    return pmd_clear_saveddirty(pmd);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_check_zapped_pte(vma: *mut vm_area_struct, pte: pte_t) {
    void arch_check_zapped_pte(struct vm_area_struct *vma, pte_t pte)
    {
//
// Hardware before shadow stack can (rarely) set Dirty=1
// on a Write=0 PTE. So the below condition
// only indicates a software bug when shadow stack is
// supported by the HW. This checking is covered in
// pte_shstk().
//
    VM_WARN_ON_ONCE(!(vma.vm_flags & VM_SHADOW_STACK) &&
    pte_shstk(pte));
    }
#[no_mangle]
pub unsafe extern "C" fn arch_check_zapped_pmd(vma: *mut vm_area_struct, pmd: pmd_t) {
    void arch_check_zapped_pmd(struct vm_area_struct *vma, pmd_t pmd)
    {
// See note in arch_check_zapped_pte()
    VM_WARN_ON_ONCE(!(vma.vm_flags & VM_SHADOW_STACK) &&
    pmd_shstk(pmd));
    }
#[no_mangle]
pub unsafe extern "C" fn arch_check_zapped_pud(vma: *mut vm_area_struct, pud: pud_t) {
    void arch_check_zapped_pud(struct vm_area_struct *vma, pud_t pud)
    {
// See note in arch_check_zapped_pte()
    VM_WARN_ON_ONCE(!(vma.vm_flags & VM_SHADOW_STACK) && pud_shstk(pud));
    }
