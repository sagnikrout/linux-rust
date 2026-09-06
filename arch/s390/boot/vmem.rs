//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/vmem.c
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

    struct ctlreg __bootdata_preserved(s390_invalid_asce);

    atomic_long_t __bootdata_preserved(direct_pages_count[PG_DIRECT_MAP_MAX]);

    enum populate_mode {
    POPULATE_NONE,
    POPULATE_DIRECT,
    POPULATE_LOWCORE,
    POPULATE_ABS_LOWCORE,
    POPULATE_IDENTITY,
    POPULATE_KERNEL,

// KASAN modes should be last and grouped together, see is_kasan_populate_mode()
    POPULATE_KASAN_MAP_SHADOW,
    POPULATE_KASAN_ZERO_SHADOW,
    POPULATE_KASAN_SHALLOW

    };

    static inline const char *get_populate_mode_name(enum populate_mode t)
    {
    switch (t) {
    POPULATE_MODE_NAME(NONE);
    POPULATE_MODE_NAME(DIRECT);
    POPULATE_MODE_NAME(LOWCORE);
    POPULATE_MODE_NAME(ABS_LOWCORE);
    POPULATE_MODE_NAME(IDENTITY);
    POPULATE_MODE_NAME(KERNEL);

    POPULATE_MODE_NAME(KASAN_MAP_SHADOW);
    POPULATE_MODE_NAME(KASAN_ZERO_SHADOW);
    POPULATE_MODE_NAME(KASAN_SHALLOW);

    default:
    return "UNKNOWN";
    }
    }
#[no_mangle]
unsafe extern "C" fn is_kasan_populate_mode(mode: enum populate_mode) -> bool {
    static bool is_kasan_populate_mode(enum populate_mode mode)
    {

    return mode >= POPULATE_KASAN_MAP_SHADOW;

    return false;

    }
    static void pgtable_populate(unsigned long addr, unsigned long end, enum populate_mode mode);

    static pte_t pte_z;
#[no_mangle]
pub unsafe extern "C" fn kasan_populate(start: c_ulong, end: c_ulong, mode: enum populate_mode) {
    static inline void kasan_populate(unsigned long start, unsigned long end, enum populate_mode mode)
    {
    let mut sha_start: c_ulong = PAGE_ALIGN_DOWN(__sha(start));
    let mut sha_end: c_ulong = PAGE_ALIGN(__sha(end));
    boot_debug("%-17s 0x%016lx-0x%016lx >> 0x%016lx-0x%016lx\n", get_populate_mode_name(mode),
    start, end, sha_start, sha_end);
    pgtable_populate(sha_start, sha_end, mode);
    }
#[no_mangle]
unsafe extern "C" fn kasan_populate_shadow(kernel_start: c_ulong, kernel_end: c_ulong) {
    static void kasan_populate_shadow(unsigned long kernel_start, unsigned long kernel_end)
    {
    let mut pmd_z: pmd_t = __pmd(__pa(kasan_early_shadow_pte) | _SEGMENT_ENTRY);
    let mut pud_z: pud_t = __pud(__pa(kasan_early_shadow_pmd) | _REGION3_ENTRY);
    let mut p4d_z: p4d_t = __p4d(__pa(kasan_early_shadow_pud) | _REGION2_ENTRY);
    let mut memgap_start: c_ulong = 0;
    unsigned long start, end;
    int i;
    pte_z = __pte(__pa(kasan_early_shadow_page) | pgprot_val(PAGE_KERNEL_RO));
    crst_table_init((unsigned long *)kasan_early_shadow_p4d, p4d_val(p4d_z));
    crst_table_init((unsigned long *)kasan_early_shadow_pud, pud_val(pud_z));
    crst_table_init((unsigned long *)kasan_early_shadow_pmd, pmd_val(pmd_z));
    memset64((u64 *)kasan_early_shadow_pte, pte_val(pte_z), PTRS_PER_PTE);
    __arch_set_page_dat(kasan_early_shadow_p4d, 1UL << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pud, 1UL << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pmd, 1UL << CRST_ALLOC_ORDER);
    __arch_set_page_dat(kasan_early_shadow_pte, 1);
    for_each_physmem_usable_range(i, &start, &end) {
    kasan_populate((unsigned long)__identity_va(start),
    (unsigned long)__identity_va(end),
    POPULATE_KASAN_MAP_SHADOW);
    if (memgap_start && physmem_info.info_source == MEM_DETECT_DIAG260) {
    kasan_populate((unsigned long)__identity_va(memgap_start),
    (unsigned long)__identity_va(start),
    POPULATE_KASAN_ZERO_SHADOW);
    }
    memgap_start = end;
    }
    kasan_populate(kernel_start + TEXT_OFFSET, kernel_end, POPULATE_KASAN_MAP_SHADOW);
    kasan_populate(0, (unsigned long)__identity_va(0), POPULATE_KASAN_ZERO_SHADOW);
    kasan_populate(AMODE31_START, AMODE31_END, POPULATE_KASAN_ZERO_SHADOW);
// shallowly populate kasan shadow for vmalloc and modules
    kasan_populate(VMALLOC_START, MODULES_END, POPULATE_KASAN_SHALLOW);
// populate kasan shadow for untracked memory
    kasan_populate((unsigned long)__identity_va(ident_map_size), VMALLOC_START,
    POPULATE_KASAN_ZERO_SHADOW);
    kasan_populate(kernel_end, _REGION1_SIZE, POPULATE_KASAN_ZERO_SHADOW);
    }
    static bool kasan_pgd_populate_zero_shadow(pgd_t *pgd, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    if (mode == POPULATE_KASAN_ZERO_SHADOW &&
    IS_ALIGNED(addr, PGDIR_SIZE) && end - addr >= PGDIR_SIZE) {
    pgd_populate(&init_mm, pgd, kasan_early_shadow_p4d);
    return true;
    }
    return false;
    }
    static bool kasan_p4d_populate_zero_shadow(p4d_t *p4d, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    if (mode == POPULATE_KASAN_ZERO_SHADOW &&
    IS_ALIGNED(addr, P4D_SIZE) && end - addr >= P4D_SIZE) {
    p4d_populate(&init_mm, p4d, kasan_early_shadow_pud);
    return true;
    }
    return false;
    }
    static bool kasan_pud_populate_zero_shadow(pud_t *pud, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    if (mode == POPULATE_KASAN_ZERO_SHADOW &&
    IS_ALIGNED(addr, PUD_SIZE) && end - addr >= PUD_SIZE) {
    pud_populate(&init_mm, pud, kasan_early_shadow_pmd);
    return true;
    }
    return false;
    }
    static bool kasan_pmd_populate_zero_shadow(pmd_t *pmd, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    if (mode == POPULATE_KASAN_ZERO_SHADOW &&
    IS_ALIGNED(addr, PMD_SIZE) && end - addr >= PMD_SIZE) {
    pmd_populate(&init_mm, pmd, kasan_early_shadow_pte);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn kasan_pte_populate_zero_shadow(pte: *mut pte_t, mode: enum populate_mode) -> bool {
    static bool kasan_pte_populate_zero_shadow(pte_t *pte, enum populate_mode mode)
    {
    if (mode == POPULATE_KASAN_ZERO_SHADOW) {
    set_pte(pte, pte_z);
    return true;
    }
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_populate_shadow(kernel_start: c_ulong, kernel_end: c_ulong) {
    static inline void kasan_populate_shadow(unsigned long kernel_start, unsigned long kernel_end)
    {
    }
    static inline bool kasan_pgd_populate_zero_shadow(pgd_t *pgd, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    return false;
    }
    static inline bool kasan_p4d_populate_zero_shadow(p4d_t *p4d, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    return false;
    }
    static inline bool kasan_pud_populate_zero_shadow(pud_t *pud, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    return false;
    }
    static inline bool kasan_pmd_populate_zero_shadow(pmd_t *pmd, unsigned long addr,
    unsigned long end, enum populate_mode mode)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn kasan_pte_populate_zero_shadow(pte: *mut pte_t, mode: enum populate_mode) -> bool {
    static bool kasan_pte_populate_zero_shadow(pte_t *pte, enum populate_mode mode)
    {
    return false;
    }

//
// Mimic virt_to_kpte() in lack of init_mm symbol. Skip pmd NULL check though.
//
    static inline pte_t *__virt_to_kpte(unsigned long va)
    {
    return pte_offset_kernel(pmd_offset(pud_offset(p4d_offset(pgd_offset_k(va), va), va), va), va);
    }
    static void *boot_crst_alloc(unsigned long val)
    {
    let mut size: c_ulong = PAGE_SIZE << CRST_ALLOC_ORDER;
    unsigned long *table;
    table = (unsigned long *)physmem_alloc_or_die(RR_VMEM, size, size);
    crst_table_init(table, val);
    __arch_set_page_dat(table, 1UL << CRST_ALLOC_ORDER);
    return table;
    }
    static pte_t *boot_pte_alloc(void)
    {
    pte_t *pte;
    pte = (void *)physmem_alloc_or_die(RR_VMEM, PAGE_SIZE, PAGE_SIZE);
    __arch_set_page_dat(pte, 1);
    memset64((u64 *)pte, _PAGE_INVALID, PTRS_PER_PTE);
    return pte;
    }
    static unsigned long resolve_pa_may_alloc(unsigned long addr, unsigned long size,
    enum populate_mode mode)
    {
    switch (mode) {
    case POPULATE_NONE:
    return INVALID_PHYS_ADDR;
    case POPULATE_DIRECT:
    return addr;
    case POPULATE_LOWCORE:
    return __lowcore_pa(addr);
    case POPULATE_ABS_LOWCORE:
    return __abs_lowcore_pa(addr);
    case POPULATE_KERNEL:
    return __kernel_pa(addr);
    case POPULATE_IDENTITY:
    return __identity_pa(addr);

    case POPULATE_KASAN_MAP_SHADOW:
// Allow to fail large page allocations, this will fall back to 1mb/4k pages
    addr = physmem_alloc(RR_VMEM, size, size, size == PAGE_SIZE);
    if (addr) {
    memset((void *)addr, 0, size);
    return addr;
    }
    return INVALID_PHYS_ADDR;

    default:
    return INVALID_PHYS_ADDR;
    }
    }
#[no_mangle]
unsafe extern "C" fn large_page_mapping_allowed(mode: enum populate_mode) -> bool {
    static bool large_page_mapping_allowed(enum populate_mode mode)
    {
    switch (mode) {
    case POPULATE_DIRECT:
    case POPULATE_IDENTITY:
    case POPULATE_KERNEL:

    case POPULATE_KASAN_MAP_SHADOW:

    return true;
    default:
    return false;
    }
    }
    static unsigned long try_get_large_pud_pa(pud_t *pu_dir, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    unsigned long pa, size = end - addr;
    if (!cpu_has_edat2() || !large_page_mapping_allowed(mode) ||
    !IS_ALIGNED(addr, PUD_SIZE) || (size < PUD_SIZE))
    return INVALID_PHYS_ADDR;
    pa = resolve_pa_may_alloc(addr, size, mode);
    if (!IS_ALIGNED(pa, PUD_SIZE))
    return INVALID_PHYS_ADDR;
    return pa;
    }
    static unsigned long try_get_large_pmd_pa(pmd_t *pm_dir, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    unsigned long pa, size = end - addr;
    if (!cpu_has_edat1() || !large_page_mapping_allowed(mode) ||
    !IS_ALIGNED(addr, PMD_SIZE) || (size < PMD_SIZE))
    return INVALID_PHYS_ADDR;
    pa = resolve_pa_may_alloc(addr, size, mode);
    if (!IS_ALIGNED(pa, PMD_SIZE))
    return INVALID_PHYS_ADDR;
    return pa;
    }
    static void pgtable_pte_populate(pmd_t *pmd, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    let mut pages: c_ulong = 0;
    pte_t *pte, entry;
    pte = pte_offset_kernel(pmd, addr);
    for (; addr < end; addr += PAGE_SIZE, pte++) {
    if (pte_none(ptep_get(pte))) {
    if (kasan_pte_populate_zero_shadow(pte, mode))
    continue;
    entry = __pte(resolve_pa_may_alloc(addr, PAGE_SIZE, mode));
    entry = set_pte_bit(entry, PAGE_KERNEL);
    set_pte(pte, entry);
    pages++;
    }
    }
    if (mode == POPULATE_IDENTITY)
    update_page_count(PG_DIRECT_MAP_4K, pages);
    }
    static void pgtable_pmd_populate(pud_t *pud, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    unsigned long pa, next, pages = 0;
    pmd_t *pmd, entry, large_entry;
    pte_t *pte;
    pmd = pmd_offset(pud, addr);
    for (; addr < end; addr = next, pmd++) {
    next = pmd_addr_end(addr, end);
    entry = pmdp_get(pmd);
    if (pmd_none(entry)) {
    if (kasan_pmd_populate_zero_shadow(pmd, addr, next, mode))
    continue;
    pa = try_get_large_pmd_pa(pmd, addr, next, mode);
    if (pa != INVALID_PHYS_ADDR) {
    large_entry = __pmd(pa);
    large_entry = set_pmd_bit(large_entry, SEGMENT_KERNEL);
    set_pmd(pmd, large_entry);
    pages++;
    continue;
    }
    pte = boot_pte_alloc();
    pmd_populate(&init_mm, pmd, pte);
    } else if (pmd_leaf(entry)) {
    continue;
    }
    pgtable_pte_populate(pmd, addr, next, mode);
    }
    if (mode == POPULATE_IDENTITY)
    update_page_count(PG_DIRECT_MAP_1M, pages);
    }
    static void pgtable_pud_populate(p4d_t *p4d, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    unsigned long pa, next, pages = 0;
    pud_t *pud, entry, large_entry;
    pmd_t *pmd;
    pud = pud_offset(p4d, addr);
    for (; addr < end; addr = next, pud++) {
    next = pud_addr_end(addr, end);
    entry = pudp_get(pud);
    if (pud_none(entry)) {
    if (kasan_pud_populate_zero_shadow(pud, addr, next, mode))
    continue;
    pa = try_get_large_pud_pa(pud, addr, next, mode);
    if (pa != INVALID_PHYS_ADDR) {
    large_entry = __pud(pa);
    large_entry = set_pud_bit(large_entry, REGION3_KERNEL);
    set_pud(pud, large_entry);
    pages++;
    continue;
    }
    pmd = boot_crst_alloc(_SEGMENT_ENTRY_EMPTY);
    pud_populate(&init_mm, pud, pmd);
    } else if (pud_leaf(entry)) {
    continue;
    }
    pgtable_pmd_populate(pud, addr, next, mode);
    }
    if (mode == POPULATE_IDENTITY)
    update_page_count(PG_DIRECT_MAP_2G, pages);
    }
    static void pgtable_p4d_populate(pgd_t *pgd, unsigned long addr, unsigned long end,
    enum populate_mode mode)
    {
    unsigned long next;
    p4d_t *p4d;
    pud_t *pud;
    p4d = p4d_offset(pgd, addr);
    for (; addr < end; addr = next, p4d++) {
    next = p4d_addr_end(addr, end);
    if (p4d_none(p4dp_get(p4d))) {
    if (kasan_p4d_populate_zero_shadow(p4d, addr, next, mode))
    continue;
    pud = boot_crst_alloc(_REGION3_ENTRY_EMPTY);
    p4d_populate(&init_mm, p4d, pud);
    }
    pgtable_pud_populate(p4d, addr, next, mode);
    }
    }
#[no_mangle]
unsafe extern "C" fn pgtable_populate(addr: c_ulong, end: c_ulong, mode: enum populate_mode) {
    static void pgtable_populate(unsigned long addr, unsigned long end, enum populate_mode mode)
    {
    unsigned long next;
    pgd_t *pgd;
    p4d_t *p4d;
    if (!is_kasan_populate_mode(mode)) {
    boot_debug("%-17s 0x%016lx-0x%016lx . 0x%016lx-0x%016lx\n",
    get_populate_mode_name(mode), addr, end,
    resolve_pa_may_alloc(addr, 0, mode),
    resolve_pa_may_alloc(end - 1, 0, mode) + 1);
    }
    pgd = pgd_offset(&init_mm, addr);
    for (; addr < end; addr = next, pgd++) {
    next = pgd_addr_end(addr, end);
    if (pgd_none(pgdp_get(pgd))) {
    if (kasan_pgd_populate_zero_shadow(pgd, addr, next, mode))
    continue;
    p4d = boot_crst_alloc(_REGION2_ENTRY_EMPTY);
    pgd_populate(&init_mm, pgd, p4d);
    }

    if (mode == POPULATE_KASAN_SHALLOW)
    continue;

    pgtable_p4d_populate(pgd, addr, next, mode);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn setup_vmem(kernel_start: c_ulong, kernel_end: c_ulong, asce_limit: c_ulong) {
    void setup_vmem(unsigned long kernel_start, unsigned long kernel_end, unsigned long asce_limit)
    {
    let mut lowcore_address: c_ulong = 0;
    unsigned long start, end;
    unsigned long asce_type;
    unsigned long asce_bits;
    pgd_t *init_mm_pgd;
    int i;
//
// Mark whole memory as no-dat. This must be done before any
// page tables are allocated, or kernel image builtin pages
// are marked as dat tables.
//
    for_each_physmem_online_range(i, &start, &end)
    __arch_set_page_nodat((void *)start, (end - start) >> PAGE_SHIFT);
//
// init_mm->pgd contains virtual address of swapper_pg_dir.
// It is unusable at this stage since DAT is yet off. Swap
// it for physical address of swapper_pg_dir and restore
// the virtual address after all page tables are created.
//
    init_mm_pgd = init_mm.pgd;
    init_mm.pgd = (pgd_t *)swapper_pg_dir;
    if (asce_limit == _REGION1_SIZE) {
    asce_type = _REGION2_ENTRY_EMPTY;
    asce_bits = _ASCE_TYPE_REGION2 | _ASCE_TABLE_LENGTH;
    } else {
    asce_type = _REGION3_ENTRY_EMPTY;
    asce_bits = _ASCE_TYPE_REGION3 | _ASCE_TABLE_LENGTH;
    }
    s390_invalid_asce.val = invalid_pg_dir | _ASCE_TYPE_REGION3 | _ASCE_TABLE_LENGTH;
    crst_table_init((unsigned long *)swapper_pg_dir, asce_type);
    crst_table_init((unsigned long *)invalid_pg_dir, _REGION3_ENTRY_EMPTY);
    __arch_set_page_dat((void *)swapper_pg_dir, 1UL << CRST_ALLOC_ORDER);
    __arch_set_page_dat((void *)invalid_pg_dir, 1UL << CRST_ALLOC_ORDER);
    if (machine_has_relocated_lowcore())
    lowcore_address = LOWCORE_ALT_ADDRESS;
//
// To allow prefixing the lowcore must be mapped with 4KB pages.
// To prevent creation of a large page at address 0 first map
// the lowcore and create the identity mapping only afterwards.
//
    pgtable_populate(lowcore_address,
    lowcore_address + sizeof(struct lowcore),
    POPULATE_LOWCORE);
    for_each_physmem_usable_range(i, &start, &end) {
// Do not map lowcore with identity mapping
    if (!start)
    start = sizeof(struct lowcore);
    pgtable_populate((unsigned long)__identity_va(start),
    (unsigned long)__identity_va(end),
    POPULATE_IDENTITY);
    }
//
// [kernel_start..kernel_start + TEXT_OFFSET] region is never
// accessed as per the linker script:
//
// . = TEXT_OFFSET;
//
// Therefore, skip mapping TEXT_OFFSET bytes to prevent access to
// [__kaslr_offset_phys..__kaslr_offset_phys + TEXT_OFFSET] region.
//
    pgtable_populate(kernel_start + TEXT_OFFSET, kernel_end, POPULATE_KERNEL);
    pgtable_populate(AMODE31_START, AMODE31_END, POPULATE_DIRECT);
    pgtable_populate(__abs_lowcore, __abs_lowcore + sizeof(struct lowcore),
    POPULATE_ABS_LOWCORE);
    pgtable_populate(__memcpy_real_area, __memcpy_real_area + PAGE_SIZE,
    POPULATE_NONE);
    memcpy_real_ptep = __identity_va(__virt_to_kpte(__memcpy_real_area));
    kasan_populate_shadow(kernel_start, kernel_end);
    get_lowcore().kernel_asce.val = swapper_pg_dir | asce_bits;
    get_lowcore().user_asce = s390_invalid_asce;
    local_ctl_load(1, &get_lowcore().kernel_asce);
    local_ctl_load(7, &get_lowcore().user_asce);
    local_ctl_load(13, &get_lowcore().kernel_asce);
    init_mm.context.asce = get_lowcore().kernel_asce.val;
    init_mm.pgd = init_mm_pgd;
    }
