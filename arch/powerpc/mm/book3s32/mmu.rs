//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s32/mmu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This file contains the routines for handling the MMU on those
// PowerPC implementations where the MMU substantially follows the
// architecture specification.  This includes the 6xx, 7xx, 7xxx,
// and 8260 implementations but excludes the 8xx and 4xx.
// -- paulus
//
// Derived from arch/ppc/mm/init.c:
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

    u8 __initdata early_hash[SZ_256K] __aligned(SZ_256K) = {0};
    static struct hash_pte __initdata *Hash = (struct hash_pte *)early_hash;
    static unsigned long __initdata Hash_size, Hash_mask;
    static unsigned int __initdata hash_mb, hash_mb2;
    unsigned long __initdata _SDR1;
    struct ppc_bat BATS[8][2];	/* 8 pairs of IBAT, DBAT */
    static struct batrange {	/* stores address ranges mapped by BATs */
    unsigned long start;
    unsigned long limit;
    phys_addr_t phys;
    } bat_addrs[8];

    unsigned long mmu_hash_lock;

//
// Return PA for this VA if it is mapped by a BAT, or 0
//
#[no_mangle]
pub unsafe extern "C" fn v_block_mapped(va: c_ulong) -> phys_addr_t {
    phys_addr_t v_block_mapped(unsigned long va)
    {
    int b;
    for (b = 0; b < ARRAY_SIZE(bat_addrs); ++b)
    if (va >= bat_addrs[b].start && va < bat_addrs[b].limit)
    return bat_addrs[b].phys + (va - bat_addrs[b].start);
    return 0;
    }
//
// Return VA for a given PA or 0 if not mapped
//
#[no_mangle]
pub unsafe extern "C" fn p_block_mapped(pa: phys_addr_t) -> c_ulong {
    unsigned long p_block_mapped(phys_addr_t pa)
    {
    int b;
    for (b = 0; b < ARRAY_SIZE(bat_addrs); ++b)
    if (pa >= bat_addrs[b].phys
    && pa < (bat_addrs[b].limit-bat_addrs[b].start)
    +bat_addrs[b].phys)
    return bat_addrs[b].start+(pa-bat_addrs[b].phys);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn find_free_bat() -> int __init {
    int __init find_free_bat(void)
    {
    int b;
    let mut n: c_int = mmu_has_feature(MMU_FTR_USE_HIGH_BATS) ? 8 : 4;
    for (b = 0; b < n; b++) {
    struct ppc_bat *bat = BATS[b];
    if (!(bat[1].batu & 3))
    return b;
    }
    return -1;
    }
//
// This function calculates the size of the larger block usable to map the
// beginning of an area based on the start address and size of that area:
// - max block size is 256 on 6xx.
// - base address must be aligned to the block size. So the maximum block size
// is identified by the lowest bit set to 1 in the base address (for instance
// if base is 0x16000000, max size is 0x02000000).
// - block size has to be a power of two. This is calculated by finding the
// highest bit set to 1.
//
#[no_mangle]
pub unsafe extern "C" fn bat_block_size(base: c_ulong, top: c_ulong) -> c_uint {
    unsigned int bat_block_size(unsigned long base, unsigned long top)
    {
    let mut max_size: c_uint = SZ_256M;
    let mut base_shift: c_uint = (ffs(base) - 1) & 31;
    let mut block_shift: c_uint = (fls(top - base) - 1) & 31;
    return min3(max_size, 1U << base_shift, 1U << block_shift);
    }
//
// Set up one of the IBAT (block address translation) register pairs.
// The parameters are not checked; in particular size must be a power
// of 2 between 128k and 256M.
//
    static void setibat(int index, unsigned long virt, phys_addr_t phys,
    unsigned int size, pgprot_t prot)
    {
    let mut bl: c_uint = (size >> 17) - 1;
    int wimgxpp;
    struct ppc_bat *bat = BATS[index];
    let mut flags: c_ulong = pgprot_val(prot);
    if (!cpu_has_feature(CPU_FTR_NEED_COHERENT))
    flags &= ~_PAGE_COHERENT;
    wimgxpp = (flags & _PAGE_COHERENT) | (_PAGE_EXEC ? BPP_RX : BPP_XX);
    bat[0].batu = virt | (bl << 2) | 2; /* Vs=1, Vp=0 */
    bat[0].batl = BAT_PHYS_ADDR(phys) | wimgxpp;
    if (!is_kernel_addr(virt))
    bat[0].batu |= 1;	/* Vp = 1 */
    }
#[no_mangle]
unsafe extern "C" fn clearibat(index: c_int) {
    static void clearibat(int index)
    {
    struct ppc_bat *bat = BATS[index];
    bat[0].batu = 0;
    bat[0].batl = 0;
    }
#[no_mangle]
unsafe extern "C" fn __mmu_mapin_ram(base: c_ulong, top: c_ulong) -> unsigned long __init {
    static unsigned long __init __mmu_mapin_ram(unsigned long base, unsigned long top)
    {
    int idx;
    while ((idx = find_free_bat()) != -1 && base != top) {
    let mut size: c_uint = bat_block_size(base, top);
    if (size < 128 << 10)
    break;
    setbat(idx, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X);
    base += size;
    }
    return base;
    }
#[no_mangle]
pub unsafe extern "C" fn mmu_mapin_ram(base: c_ulong, top: c_ulong) -> unsigned long __init {
    unsigned long __init mmu_mapin_ram(unsigned long base, unsigned long top)
    {
    unsigned long done;
    let mut border: c_ulong = (unsigned long)__srwx_boundary - PAGE_OFFSET;
    unsigned long size;
    size = roundup_pow_of_two((unsigned long)_einittext - PAGE_OFFSET);
    setibat(0, PAGE_OFFSET, 0, size, PAGE_KERNEL_X);
    if (debug_pagealloc_enabled_or_kfence()) {
    pr_debug_once("Read-Write memory mapped without BATs\n");
    if (base >= border)
    return base;
    if (top >= border)
    top = border;
    }
    if (!strict_kernel_rwx_enabled() || base >= border || top <= border)
    return __mmu_mapin_ram(base, top);
    done = __mmu_mapin_ram(base, border);
    if (done != border)
    return done;
    return __mmu_mapin_ram(border, top);
    }
#[no_mangle]
unsafe extern "C" fn is_module_segment(addr: c_ulong) -> bool {
    static bool is_module_segment(unsigned long addr)
    {
    if (!IS_ENABLED(CONFIG_EXECMEM))
    return false;
    if (addr < ALIGN_DOWN(MODULES_VADDR, SZ_256M))
    return false;
    if (addr > ALIGN(MODULES_END, SZ_256M) - 1)
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn mmu_mark_initmem_nx() -> c_int {
    int mmu_mark_initmem_nx(void)
    {
    let mut nb: c_int = mmu_has_feature(MMU_FTR_USE_HIGH_BATS) ? 8 : 4;
    int i;
    let mut base: c_ulong = (unsigned long)_stext - PAGE_OFFSET;
    let mut top: c_ulong = ALIGN((unsigned long)_etext - PAGE_OFFSET, SZ_128K);
    let mut border: c_ulong = (unsigned long)__init_begin - PAGE_OFFSET;
    unsigned long size;
    for (i = 0; i < nb - 1 && base < top;) {
    size = bat_block_size(base, top);
    setibat(i++, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X);
    base += size;
    }
    if (base < top) {
    size = bat_block_size(base, top);
    if ((top - base) > size) {
    size <<= 1;
    if (strict_kernel_rwx_enabled() && base + size > border)
    pr_warn("Some RW data is getting mapped X. "
    "Adjust CONFIG_DATA_SHIFT to avoid that.\n");
    }
    setibat(i++, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X);
    base += size;
    }
    for (; i < nb; i++)
    clearibat(i);
    update_bats();
    for (i = ALIGN(TASK_SIZE, SZ_256M) >> 28; i < 16; i++) {
// Do not set NX on VM space for modules
    if (is_module_segment(i << 28))
    continue;
    mtsr(mfsr(i << 28) | 0x10000000, i << 28);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mmu_mark_rodata_ro() -> c_int {
    int mmu_mark_rodata_ro(void)
    {
    let mut nb: c_int = mmu_has_feature(MMU_FTR_USE_HIGH_BATS) ? 8 : 4;
    int i;
    for (i = 0; i < nb; i++) {
    struct ppc_bat *bat = BATS[i];
    if (bat_addrs[i].start < (unsigned long)__end_rodata)
    bat[1].batl = (bat[1].batl & ~BPP_RW) | BPP_RX;
    }
    update_bats();
    return 0;
    }
//
// Set up one of the D BAT (block address translation) register pairs.
// The parameters are not checked; in particular size must be a power
// of 2 between 128k and 256M.
//
    void __init setbat(int index, unsigned long virt, phys_addr_t phys,
    unsigned int size, pgprot_t prot)
    {
    unsigned int bl;
    int wimgxpp;
    struct ppc_bat *bat;
    let mut flags: c_ulong = pgprot_val(prot);
    if (index == -1)
    index = find_free_bat();
    if (index == -1) {
    pr_err("%s: no BAT available for mapping 0x%llx\n", __func__,
    (unsigned long long)phys);
    return;
    }
    bat = BATS[index];
    if ((flags & _PAGE_NO_CACHE) ||
    (cpu_has_feature(CPU_FTR_NEED_COHERENT) == 0))
    flags &= ~_PAGE_COHERENT;
    bl = (size >> 17) - 1;
// Do DBAT first
    wimgxpp = flags & (_PAGE_WRITETHRU | _PAGE_NO_CACHE
    | _PAGE_COHERENT | _PAGE_GUARDED);
    wimgxpp |= (flags & _PAGE_WRITE) ? BPP_RW : BPP_RX;
    bat[1].batu = virt | (bl << 2) | 2; /* Vs=1, Vp=0 */
    bat[1].batl = BAT_PHYS_ADDR(phys) | wimgxpp;
    if (!is_kernel_addr(virt))
    bat[1].batu |= 1; 	/* Vp = 1 */
    if (flags & _PAGE_GUARDED) {
// G bit must be zero in IBATs
    flags &= ~_PAGE_EXEC;
    }
    bat_addrs[index].start = virt;
    bat_addrs[index].limit = virt + ((bl + 1) << 17) - 1;
    bat_addrs[index].phys = phys;
    }
//
// Preload a translation in the hash table
//
#[no_mangle]
unsafe extern "C" fn hash_preload(mm: *mut mm_struct, ea: c_ulong) {
    static void hash_preload(struct mm_struct *mm, unsigned long ea)
    {
    pmd_t *pmd;
    if (!mmu_has_feature(MMU_FTR_HPTE_TABLE))
    return;
    pmd = pmd_off(mm, ea);
    if (!pmd_none(*pmd))
    add_hash_page(mm.context.id, ea, pmd_val(*pmd));
    }
//
// This is called at the end of handling a user page fault, when the
// fault has been handled by updating a PTE in the linux page tables.
// We use it to preload an HPTE into the hash table corresponding to
// the updated linux PTE.
//
// This must always be called with the pte lock held.
//
    void __update_mmu_cache(struct vm_area_struct *vma, unsigned long address,
    pte_t *ptep)
    {
//
// We don't need to worry about _PAGE_PRESENT here because we are
// called with either mm->page_table_lock held or ptl lock held
//
// We only want HPTEs for linux PTEs that have _PAGE_ACCESSED set
    if (!pte_young(*ptep) || address >= TASK_SIZE)
    return;
// We have to test for regs NULL since init will get here first thing at boot
    if (!current.thread.regs)
    return;
// We also avoid filling the hash if not coming from a fault
    if (TRAP(current.thread.regs) != 0x300 && TRAP(current.thread.regs) != 0x400)
    return;
    hash_preload(vma.vm_mm, address);
    }
//
// Initialize the hash table and patch the instructions in hashtable.S.
//
#[no_mangle]
pub unsafe extern "C" fn MMU_init_hw() -> void __init {
    void __init MMU_init_hw(void)
    {
    unsigned int n_hpteg, lg_n_hpteg;
    if (!mmu_has_feature(MMU_FTR_HPTE_TABLE))
    return;
    if ( ppc_md.progress ) ppc_md.progress("hash:enter", 0x105);

//
// Allow 1 HPTE (1/8 HPTEG) for each page of memory.
// This is less than the recommended amount, but then
// Linux ain't AIX.
//
    n_hpteg = total_memory / (PAGE_SIZE * 8);
    if (n_hpteg < MIN_N_HPTEG)
    n_hpteg = MIN_N_HPTEG;
    lg_n_hpteg = __ilog2(n_hpteg);
    if (n_hpteg & (n_hpteg - 1)) {
    ++lg_n_hpteg;		/* round up if not power of 2 */
    n_hpteg = 1 << lg_n_hpteg;
    }
    Hash_size = n_hpteg << LG_HPTEG_SIZE;
//
// Find some memory for the hash table.
//
    if ( ppc_md.progress ) ppc_md.progress("hash:find piece", 0x322);
    Hash = memblock_alloc_or_panic(Hash_size, Hash_size);
    _SDR1 = __pa(Hash) | SDR1_LOW_BITS;
    pr_info("Total memory = %lldMB; using %ldkB for hash table\n",
    (unsigned long long)(total_memory >> 20), Hash_size >> 10);
    Hash_mask = n_hpteg - 1;
    hash_mb2 = hash_mb = 32 - LG_HPTEG_SIZE - lg_n_hpteg;
    if (lg_n_hpteg > 16)
    hash_mb2 = 16 - LG_HPTEG_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn MMU_init_hw_patch() -> void __init {
    void __init MMU_init_hw_patch(void)
    {
    let mut hmask: c_uint = Hash_mask >> (16 - LG_HPTEG_SIZE);
    let mut hash: c_uint = (unsigned int)Hash - PAGE_OFFSET;
    if (!mmu_has_feature(MMU_FTR_HPTE_TABLE))
    return;
    if (ppc_md.progress)
    ppc_md.progress("hash:patch", 0x345);
    if (ppc_md.progress)
    ppc_md.progress("hash:done", 0x205);
// WARNING: Make sure nothing can trigger a KASAN check past this point
//
// Patch up the instructions in hashtable.S:create_hpte
//
    modify_instruction_site(&patch__hash_page_A0, 0xffff, hash >> 16);
    modify_instruction_site(&patch__hash_page_A1, 0x7c0, hash_mb << 6);
    modify_instruction_site(&patch__hash_page_A2, 0x7c0, hash_mb2 << 6);
    modify_instruction_site(&patch__hash_page_B, 0xffff, hmask);
    modify_instruction_site(&patch__hash_page_C, 0xffff, hmask);
//
// Patch up the instructions in hashtable.S:flush_hash_page
//
    modify_instruction_site(&patch__flush_hash_A0, 0xffff, hash >> 16);
    modify_instruction_site(&patch__flush_hash_A1, 0x7c0, hash_mb << 6);
    modify_instruction_site(&patch__flush_hash_A2, 0x7c0, hash_mb2 << 6);
    modify_instruction_site(&patch__flush_hash_B, 0xffff, hmask);
    }
    void setup_initial_memory_limit(phys_addr_t first_memblock_base,
    phys_addr_t first_memblock_size)
    {
// We don't currently support the first MEMBLOCK not mapping 0
// physical on those processors
//
    BUG_ON(first_memblock_base != 0);
    memblock_set_current_limit(min_t(u64, first_memblock_size, SZ_256M));
    }
#[no_mangle]
pub unsafe extern "C" fn print_system_hash_info() -> void __init {
    void __init print_system_hash_info(void)
    {
    pr_info("Hash_size         = 0x%lx\n", Hash_size);
    if (Hash_mask)
    pr_info("Hash_mask         = 0x%lx\n", Hash_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn early_init_mmu() -> void __init {
    void __init early_init_mmu(void)
    {
    }
