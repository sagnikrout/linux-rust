//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/cacheflush.c
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
// flush_coherent_icache() - if a CPU has a coherent icache, flush it
// Return true if the cache was flushed, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn flush_coherent_icache() -> bool {
    static inline bool flush_coherent_icache(void)
    {
//
// For a snooping icache, we still need a dummy icbi to purge all the
// prefetched instructions from the ifetch buffers. We also need a sync
// before the icbi to order the actual stores to memory that might
// have modified instructions with the icbi.
//
    if (cpu_has_feature(CPU_FTR_COHERENT_ICACHE)) {
    mb(); /* sync */
    icbi((void *)PAGE_OFFSET);
    mb(); /* sync */
    isync();
    return true;
    }
    return false;
    }
//
// invalidate_icache_range() - Flush the icache by issuing icbi across an address range
// @start: the start address
// @stop: the stop address (exclusive)
//
#[no_mangle]
unsafe extern "C" fn invalidate_icache_range(start: c_ulong, stop: c_ulong) {
    static void invalidate_icache_range(unsigned long start, unsigned long stop)
    {
    let mut shift: c_ulong = l1_icache_shift();
    let mut bytes: c_ulong = l1_icache_bytes();
    char *addr = (char *)(start & ~(bytes - 1));
    let mut size: c_ulong = stop - (unsigned long)addr + (bytes - 1);
    unsigned long i;
    for (i = 0; i < size >> shift; i++, addr += bytes)
    icbi(addr);
    mb(); /* sync */
    isync();
    }
//
// flush_icache_range: Write any modified data cache blocks out to memory
// and invalidate the corresponding blocks in the instruction cache
//
// Generic code will call this after writing memory, before executing from it.
//
// @start: the start address
// @stop: the stop address (exclusive)
//
#[no_mangle]
pub unsafe extern "C" fn flush_icache_range(start: c_ulong, stop: c_ulong) {
    void flush_icache_range(unsigned long start, unsigned long stop)
    {
    if (flush_coherent_icache())
    return;
    clean_dcache_range(start, stop);
    if (IS_ENABLED(CONFIG_44x)) {
//
// Flash invalidate on 44x because we are passed kmapped
// addresses and this doesn't work for userspace pages due to
// the virtually tagged icache.
//
    iccci((void *)start);
    mb(); /* sync */
    isync();
    } else
    invalidate_icache_range(start, stop);
    }
    EXPORT_SYMBOL(flush_icache_range);

//
// flush_dcache_icache_phys() - Flush a page by its physical address
// @physaddr: the physical address of the page
//
#[no_mangle]
unsafe extern "C" fn flush_dcache_icache_phys(physaddr: c_ulong) {
    static void flush_dcache_icache_phys(unsigned long physaddr)
    {
    let mut bytes: c_ulong = l1_dcache_bytes();
    let mut nb: c_ulong = PAGE_SIZE / bytes;
    let mut addr: c_ulong = physaddr & PAGE_MASK;
    unsigned long msr, msr0;
    let mut loop1: c_ulong = addr, loop2 = addr;
    msr0 = mfmsr();
    msr = msr0 & ~MSR_DR;
//
// This must remain as ASM to prevent potential memory accesses
// while the data MMU is disabled
//
    asm volatile(
    "   mtctr %2;\n"
    "   mtmsr %3;\n"
    "   isync;\n"
    "0: dcbst   0, %0;\n"
    "   addi    %0, %0, %4;\n"
    "   bdnz    0b;\n"
    "   sync;\n"
    "   mtctr %2;\n"
    "1: icbi    0, %1;\n"
    "   addi    %1, %1, %4;\n"
    "   bdnz    1b;\n"
    "   sync;\n"
    "   mtmsr %5;\n"
    "   isync;\n"
    : "+&r" (loop1), "+&r" (loop2)
    : "r" (nb), "r" (msr), "i" (bytes), "r" (msr0)
    : "ctr", "memory");
    }
    NOKPROBE_SYMBOL(flush_dcache_icache_phys)

#[no_mangle]
unsafe extern "C" fn flush_dcache_icache_phys(physaddr: c_ulong) {
    static void flush_dcache_icache_phys(unsigned long physaddr)
    {
    }

//
// __flush_dcache_icache(): Flush a particular page from the data cache to RAM.
// Note: this is necessary because the instruction cache does *not
// snoop from the data cache.
//
// @p: the address of the page to flush
//
#[no_mangle]
unsafe extern "C" fn __flush_dcache_icache(p: *mut c_void) {
    static void __flush_dcache_icache(void *p)
    {
    let mut addr: c_ulong = (unsigned long)p & PAGE_MASK;
    clean_dcache_range(addr, addr + PAGE_SIZE);
//
// We don't flush the icache on 44x. Those have a virtual icache and we
// don't have access to the virtual address here (it's not the page
// vaddr but where it's mapped in user space). The flushing of the
// icache on these is handled elsewhere, when a change in the address
// space occurs, before returning to user space.
//
    if (mmu_has_feature(MMU_FTR_TYPE_44x))
    return;
    invalidate_icache_range(addr, addr + PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_dcache_icache_folio(folio: *mut folio) {
    void flush_dcache_icache_folio(struct folio *folio)
    {
    unsigned int i, nr = folio_nr_pages(folio);
    if (flush_coherent_icache())
    return;
    if (!folio_test_highmem(folio)) {
    void *addr = folio_address(folio);
    for (i = 0; i < nr; i++)
    __flush_dcache_icache(addr + i * PAGE_SIZE);
    } else if (IS_ENABLED(CONFIG_BOOKE) || sizeof(phys_addr_t) > sizeof(void *)) {
    for (i = 0; i < nr; i++) {
    void *start = kmap_local_folio(folio, i * PAGE_SIZE);
    __flush_dcache_icache(start);
    kunmap_local(start);
    }
    } else {
    let mut pfn: c_ulong = folio_pfn(folio);
    for (i = 0; i < nr; i++)
    flush_dcache_icache_phys((pfn + i) * PAGE_SIZE);
    }
    }
    EXPORT_SYMBOL(flush_dcache_icache_folio);
#[no_mangle]
pub unsafe extern "C" fn clear_user_page(page: *mut c_void, vaddr: c_ulong, pg: *mut page) {
    void clear_user_page(void *page, unsigned long vaddr, struct page *pg)
    {
    clear_page(page);
//
// We shouldn't have to do this, but some versions of glibc
// require it (ld.so assumes zero filled pages are icache clean)
// - Anton
//
    flush_dcache_page(pg);
    }
    EXPORT_SYMBOL(clear_user_page);
    void copy_user_page(void *vto, void *vfrom, unsigned long vaddr,
    struct page *pg)
    {
    copy_page(vto, vfrom);
//
// We should be able to use the following optimisation, however
// there are two problems.
// Firstly a bug in some versions of binutils meant PLT sections
// were not marked executable.
// Secondly the first word in the GOT section is blrl, used
// to establish the GOT address. Until recently the GOT was
// not marked executable.
// - Anton
//

    if (!vma.vm_file && ((vma.vm_flags & VM_EXEC) == 0))
    return;

    flush_dcache_page(pg);
    }
    void flush_icache_user_page(struct vm_area_struct *vma, struct page *page,
    unsigned long addr, int len)
    {
    void *maddr;
    maddr = kmap_local_page(page) + (addr & ~PAGE_MASK);
    flush_icache_range((unsigned long)maddr, (unsigned long)maddr + len);
    kunmap_local(maddr);
    }
