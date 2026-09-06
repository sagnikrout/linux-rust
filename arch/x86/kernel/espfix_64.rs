//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/espfix_64.c
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
// -----------------------------------------------------------------------
//
// Copyright 2014 Intel Corporation; author: H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// The IRET instruction, when returning to a 16-bit segment, only
// restores the bottom 16 bits of the user space stack pointer.  This
// causes some 16-bit software to break, but it also leaks kernel state
// to user space.
//
// This works around this by creating percpu "ministacks", each of which
// is mapped 2^16 times 64K apart.  When we detect that the return SS is
// on the LDT, we copy the IRET frame to the ministack and use the
// relevant alias to return to userspace.  The ministacks are mapped
// readonly, so if the IRET fault we promote #GP to #DF which is an IST
// vector and thus has its own stack; we then do the fixup in the #DF
// handler.
//
// This file sets up the ministacks and the related page tables.  The
// actual ministack invocation is in entry_64.S.
//

//
// Note: we only need 6*8 = 48 bytes for the espfix stack, but round
// it up to a cache line to avoid unnecessary sharing.
//

// There is address space for how many espfix pages?

// This contains the *bottom* address of the espfix stack
    DEFINE_PER_CPU_READ_MOSTLY(unsigned long, espfix_stack);
    DEFINE_PER_CPU_READ_MOSTLY(unsigned long, espfix_waddr);
// Initialization mutex - should this be a spinlock?
    static DEFINE_MUTEX(espfix_init_mutex);
// Page allocation bitmap - each page serves ESPFIX_STACKS_PER_PAGE CPUs

    static void *espfix_pages[ESPFIX_MAX_PAGES];
    static __page_aligned_bss pud_t espfix_pud_page[PTRS_PER_PUD]
    __aligned(PAGE_SIZE);
    static unsigned int page_random, slot_random;
//
// This returns the bottom address of the espfix stack for a specific CPU.
// The math allows for a non-power-of-two ESPFIX_STACK_SIZE, in which case
// we have to account for some amount of padding at the end of each page.
//
#[no_mangle]
pub unsafe extern "C" fn espfix_base_addr(cpu: c_uint) -> c_ulong {
    static inline unsigned long espfix_base_addr(unsigned int cpu)
    {
    unsigned long page, slot;
    unsigned long addr;
    page = (cpu / ESPFIX_STACKS_PER_PAGE) ^ page_random;
    slot = (cpu + slot_random) % ESPFIX_STACKS_PER_PAGE;
    addr = (page << PAGE_SHIFT) + (slot * ESPFIX_STACK_SIZE);
    addr = (addr & 0xffffUL) | ((addr & ~0xffffUL) << 16);
    addr += ESPFIX_BASE_ADDR;
    return addr;
    }

#[no_mangle]
unsafe extern "C" fn init_espfix_random() {
    static void init_espfix_random(void)
    {
    let mut rand: c_ulong = get_random_long();
    slot_random = rand % ESPFIX_STACKS_PER_PAGE;
    page_random = (rand / ESPFIX_STACKS_PER_PAGE)
    & (ESPFIX_PAGE_SPACE - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn init_espfix_bsp() -> void __init {
    void __init init_espfix_bsp(void)
    {
    pgd_t *pgd;
    p4d_t *p4d;
// FRED systems always restore the full value of %rsp
    if (cpu_feature_enabled(X86_FEATURE_FRED))
    return;
// Install the espfix pud into the kernel page directory
    pgd = &init_top_pgt[pgd_index(ESPFIX_BASE_ADDR)];
    p4d = p4d_alloc(&init_mm, pgd, ESPFIX_BASE_ADDR);
    p4d_populate(&init_mm, p4d, espfix_pud_page);
// Randomize the locations
    init_espfix_random();
// The rest is the same as for any other processor
    init_espfix_ap(0);
    }
#[no_mangle]
pub unsafe extern "C" fn init_espfix_ap(cpu: c_int) {
    void init_espfix_ap(int cpu)
    {
    unsigned int page;
    unsigned long addr;
    pud_t pud, *pud_p;
    pmd_t pmd, *pmd_p;
    pte_t pte, *pte_p;
    int n, node;
    void *stack_page;
    pteval_t ptemask;
// FRED systems always restore the full value of %rsp
    if (cpu_feature_enabled(X86_FEATURE_FRED))
    return;
// We only have to do this once...
    if (likely(per_cpu(espfix_stack, cpu)))
    return;		/* Already initialized */
    addr = espfix_base_addr(cpu);
    page = cpu/ESPFIX_STACKS_PER_PAGE;
// Did another CPU already set this up?
    stack_page = READ_ONCE(espfix_pages[page]);
    if (likely(stack_page))
    goto done;
    mutex_lock(&espfix_init_mutex);
// Did we race on the lock?
    stack_page = READ_ONCE(espfix_pages[page]);
    if (stack_page)
    goto unlock_done;
    node = cpu_to_node(cpu);
    ptemask = __supported_pte_mask;
    pud_p = &espfix_pud_page[pud_index(addr)];
    pud = *pud_p;
    if (!pud_present(pud)) {
    struct page *page = alloc_pages_node(node, PGALLOC_GFP, 0);
    pmd_p = (pmd_t *)page_address(page);
    pud = __pud(__pa(pmd_p) | (PGTABLE_PROT & ptemask));
    paravirt_alloc_pmd(&init_mm, __pa(pmd_p) >> PAGE_SHIFT);
    for (n = 0; n < ESPFIX_PUD_CLONES; n++)
    set_pud(&pud_p[n], pud);
    }
    pmd_p = pmd_offset(&pud, addr);
    pmd = *pmd_p;
    if (!pmd_present(pmd)) {
    struct page *page = alloc_pages_node(node, PGALLOC_GFP, 0);
    pte_p = (pte_t *)page_address(page);
    pmd = __pmd(__pa(pte_p) | (PGTABLE_PROT & ptemask));
    paravirt_alloc_pte(&init_mm, __pa(pte_p) >> PAGE_SHIFT);
    for (n = 0; n < ESPFIX_PMD_CLONES; n++)
    set_pmd(&pmd_p[n], pmd);
    }
    pte_p = pte_offset_kernel(&pmd, addr);
    stack_page = page_address(alloc_pages_node(node, GFP_KERNEL, 0));
//
// __PAGE_KERNEL_* includes _PAGE_GLOBAL, which we want since
// this is mapped to userspace.
//
    pte = __pte(__pa(stack_page) | ((__PAGE_KERNEL_RO | _PAGE_ENC) & ptemask));
    for (n = 0; n < ESPFIX_PTE_CLONES; n++)
    set_pte(&pte_p[n*PTE_STRIDE], pte);
// Job is done for this CPU and any CPU which shares this page
    WRITE_ONCE(espfix_pages[page], stack_page);
    unlock_done:
    mutex_unlock(&espfix_init_mutex);
    done:
    per_cpu(espfix_stack, cpu) = addr;
    per_cpu(espfix_waddr, cpu) = (unsigned long)stack_page
    + (addr & ~PAGE_MASK);
    }
