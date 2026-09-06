//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/compressed/ident_map_64.c
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
// This code is used on x86_64 to create page table identity mappings on
// demand by building up a new set of page tables (or appending to the
// existing ones), and then switching over to them when ready.
//
// Copyright (C) 2015-2016  Yinghai Lu
// Copyright (C)      2016  Kees Cook
//
// No MITIGATION_PAGE_TABLE_ISOLATION support needed either:

// These actually do the work of building the kernel identity maps.

// Use the static base for this part of the boot process

// Macro flag: #define _SETUP

    extern unsigned long get_cmd_line_ptr(void);
// Used by PAGE_KERN* macros:
    let mut __read_mostly: pteval_t __default_kernel_pte_mask = ~0;
// Used to track our page table allocation area.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_pgt_data {
    pub pgt_buf: *mut c_uchar,
    pub pgt_buf_size: c_ulong,
    pub pgt_buf_offset: c_ulong,
}

//
// Allocates space for a page table entry, using struct alloc_pgt_data
// above. Besides the local callers, this is used as the allocation
// callback in mapping_info below.
//
    static void *alloc_pgt_page(void *context)
    {
    struct alloc_pgt_data *pages = (struct alloc_pgt_data *)context;
    unsigned char *entry;
// Validate there is space available for a new page.
    if (pages.pgt_buf_offset >= pages.pgt_buf_size) {
    debug_putstr("out of pgt_buf in " __FILE__ "!?\n");
    debug_putaddr(pages.pgt_buf_offset);
    debug_putaddr(pages.pgt_buf_size);
    return core::ptr::null_mut();
    }
// Consumed more tables than expected?
    if (pages.pgt_buf_offset == BOOT_PGT_SIZE_WARN) {
    debug_putstr("pgt_buf running low in " __FILE__ "\n");
    debug_putstr("Need to raise BOOT_PGT_SIZE?\n");
    debug_putaddr(pages.pgt_buf_offset);
    debug_putaddr(pages.pgt_buf_size);
    }
    entry = pages.pgt_buf + pages.pgt_buf_offset;
    pages.pgt_buf_offset += PAGE_SIZE;
    return entry;
    }
// Used to track our allocated page tables.
    static struct alloc_pgt_data pgt_data;
// The top level page table entry pointer.
    static unsigned long top_level_pgt;
    let mut physical_mask: phys_addr_t = (1ULL << __PHYSICAL_MASK_SHIFT) - 1;
//
// Mapping information structure passed to kernel_ident_mapping_init().
// Due to relocation, pointers must be assigned at run time not build time.
//
    static struct x86_mapping_info mapping_info;
//
// Adds the specified range to the identity mappings.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_add_identity_map(start: c_ulong, end: c_ulong) {
    void kernel_add_identity_map(unsigned long start, unsigned long end)
    {
    int ret;
// Align boundary to 2M.
    start = round_down(start, PMD_SIZE);
    end = round_up(end, PMD_SIZE);
    if (start >= end)
    return;
// Build the mapping.
    ret = kernel_ident_mapping_init(&mapping_info, (pgd_t *)top_level_pgt, start, end);
    if (ret)
    error("Error: kernel_ident_mapping_init() failed\n");
    }
// Locates and clears a region for a new top level page table.
#[no_mangle]
pub unsafe extern "C" fn initialize_identity_maps(rmode: *mut c_void) {
    void initialize_identity_maps(void *rmode)
    {
    unsigned long cmdline;
    struct setup_data *sd;
// Exclude the encryption mask from __PHYSICAL_MASK
    physical_mask &= ~sme_me_mask;
// Init mapping_info with run-time function/buffer pointers.
    mapping_info.alloc_pgt_page = alloc_pgt_page;
    mapping_info.context = &pgt_data;
    mapping_info.page_flag = __PAGE_KERNEL_LARGE_EXEC | sme_me_mask;
    mapping_info.kernpg_flag = _KERNPG_TABLE;
//
// It should be impossible for this not to already be true,
// but since calling this a second time would rewind the other
// counters, let's just make sure this is reset too.
//
    pgt_data.pgt_buf_offset = 0;
//
// If we came here via startup_32(), cr3 will be _pgtable already
// and we must append to the existing area instead of entirely
// overwriting it.
//
// With 5-level paging, we use '_pgtable' to allocate the p4d page table,
// the top-level page table is allocated separately.
//
// p4d_offset(top_level_pgt, 0) would cover both the 4- and 5-level
// cases. On 4-level paging it's equal to 'top_level_pgt'.
//
    top_level_pgt = read_cr3_pa();
    if (p4d_offset((pgd_t *)top_level_pgt, 0) == (p4d_t *)_pgtable) {
    pgt_data.pgt_buf = _pgtable + BOOT_INIT_PGT_SIZE;
    pgt_data.pgt_buf_size = BOOT_PGT_SIZE - BOOT_INIT_PGT_SIZE;
    memset(pgt_data.pgt_buf, 0, pgt_data.pgt_buf_size);
    } else {
    pgt_data.pgt_buf = _pgtable;
    pgt_data.pgt_buf_size = BOOT_PGT_SIZE;
    memset(pgt_data.pgt_buf, 0, pgt_data.pgt_buf_size);
    top_level_pgt = (unsigned long)alloc_pgt_page(&pgt_data);
    }
//
// New page-table is set up - map the kernel image, boot_params and the
// command line. The uncompressed kernel requires boot_params and the
// command line to be mapped in the identity mapping. Map them
// explicitly here in case the compressed kernel does not touch them,
// or does not touch all the pages covering them.
//
    kernel_add_identity_map((unsigned long)_head, (unsigned long)_end);
    boot_params_ptr = rmode;
    kernel_add_identity_map((unsigned long)boot_params_ptr,
    (unsigned long)(boot_params_ptr + 1));
    cmdline = get_cmd_line_ptr();
    kernel_add_identity_map(cmdline, cmdline + COMMAND_LINE_SIZE);
//
// Also map the setup_data entries passed via boot_params in case they
// need to be accessed by uncompressed kernel via the identity mapping.
//
    sd = (struct setup_data *)boot_params_ptr.hdr.setup_data;
    while (sd) {
    let mut sd_addr: c_ulong = (unsigned long)sd;
    kernel_add_identity_map(sd_addr, sd_addr + sizeof(*sd) + sd.len);
    sd = (struct setup_data *)sd.next;
    }
    sev_prep_identity_maps(top_level_pgt);
// Load the new page-table.
    write_cr3(top_level_pgt);
//
// Now that the required page table mappings are established and a
// GHCB can be used, check for SNP guest/HV feature compatibility.
//
    snp_check_features();
    }
    static pte_t *split_large_pmd(struct x86_mapping_info *info,
    pmd_t *pmdp, unsigned long __address)
    {
    unsigned long page_flags;
    unsigned long address;
    pte_t *pte;
    pmd_t pmd;
    int i;
    pte = (pte_t *)info.alloc_pgt_page(info.context);
    if (!pte)
    return core::ptr::null_mut();
    address     = __address & PMD_MASK;
// No large page - clear PSE flag
    page_flags  = info.page_flag & ~_PAGE_PSE;
// Populate the PTEs
    for (i = 0; i < PTRS_PER_PMD; i++) {
    set_pte(&pte[i], __pte(address | page_flags));
    address += PAGE_SIZE;
    }
//
// Ideally we need to clear the large PMD first and do a TLB
// flush before we write the new PMD. But the 2M range of the
// PMD might contain the code we execute and/or the stack
// we are on, so we can't do that. But that should be safe here
// because we are going from large to small mappings and we are
// also the only user of the page-table, so there is no chance
// of a TLB multihit.
//
    pmd = __pmd((unsigned long)pte | info.kernpg_flag);
    set_pmd(pmdp, pmd);
// Flush TLB to establish the new PMD
    write_cr3(top_level_pgt);
    return pte + pte_index(__address);
    }
#[no_mangle]
unsafe extern "C" fn clflush_page(address: c_ulong) {
    static void clflush_page(unsigned long address)
    {
    unsigned int flush_size;
    char *cl, *start, *end;
//
// Hardcode cl-size to 64 - CPUID can't be used here because that might
// cause another #VC exception and the GHCB is not ready to use yet.
//
    flush_size = 64;
    start      = (char *)(address & PAGE_MASK);
    end        = start + PAGE_SIZE;
//
// First make sure there are no pending writes on the cache-lines to
// flush.
//
    asm volatile("mfence" : : : "memory");
    for (cl = start; cl != end; cl += flush_size)
    clflush(cl);
    }
    static int set_clr_page_flags(struct x86_mapping_info *info,
    unsigned long address,
    pteval_t set, pteval_t clr)
    {
    pgd_t *pgdp = (pgd_t *)top_level_pgt;
    p4d_t *p4dp;
    pud_t *pudp;
    pmd_t *pmdp;
    pte_t *ptep, pte;
//
// First make sure there is a PMD mapping for 'address'.
// It should already exist, but keep things generic.
//
// To map the page just read from it and fault it in if there is no
// mapping yet. kernel_add_identity_map() can't be called here because
// that would unconditionally map the address on PMD level, destroying
// any PTE-level mappings that might already exist. Use assembly here
// so the access won't be optimized away.
//
    asm volatile("mov %[address], %%r9"
    :: [address] "g" (*(unsigned long *)address)
    : "r9", "memory");
//
// The page is mapped at least with PMD size - so skip checks and walk
// directly to the PMD.
//
    p4dp = p4d_offset(pgdp, address);
    pudp = pud_offset(p4dp, address);
    pmdp = pmd_offset(pudp, address);
    if (pmd_leaf(*pmdp))
    ptep = split_large_pmd(info, pmdp, address);
    else
    ptep = pte_offset_kernel(pmdp, address);
    if (!ptep)
    return -ENOMEM;
//
// Changing encryption attributes of a page requires to flush it from
// the caches.
//
    if ((set | clr) & _PAGE_ENC) {
    clflush_page(address);
//
// If the encryption attribute is being cleared, change the page state
// to shared in the RMP table.
//
    if (clr)
    snp_set_page_shared(__pa(address & PAGE_MASK));
    }
// Update PTE
    pte = *ptep;
    pte = pte_set_flags(pte, set);
    pte = pte_clear_flags(pte, clr);
    set_pte(ptep, pte);
//
// If the encryption attribute is being set, then change the page state to
// private in the RMP entry. The page state change must be done after the PTE
// is updated.
//
    if (set & _PAGE_ENC)
    snp_set_page_private(__pa(address & PAGE_MASK));
// Flush TLB after changing encryption attribute
    write_cr3(top_level_pgt);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_page_decrypted(address: c_ulong) -> c_int {
    int set_page_decrypted(unsigned long address)
    {
    return set_clr_page_flags(&mapping_info, address, 0, _PAGE_ENC);
    }
#[no_mangle]
pub unsafe extern "C" fn set_page_encrypted(address: c_ulong) -> c_int {
    int set_page_encrypted(unsigned long address)
    {
    return set_clr_page_flags(&mapping_info, address, _PAGE_ENC, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn set_page_non_present(address: c_ulong) -> c_int {
    int set_page_non_present(unsigned long address)
    {
    return set_clr_page_flags(&mapping_info, address, 0, _PAGE_PRESENT);
    }
    static void do_pf_error(const char *msg, unsigned long error_code,
    unsigned long address, unsigned long ip)
    {
    error_putstr(msg);
    error_putstr("\nError Code: ");
    error_puthex(error_code);
    error_putstr("\nCR2: 0x");
    error_puthex(address);
    error_putstr("\nRIP relative to _head: 0x");
    error_puthex(ip - (unsigned long)_head);
    error_putstr("\n");
    error("Stopping.\n");
    }
#[no_mangle]
pub unsafe extern "C" fn do_boot_page_fault(regs: *mut pt_regs, error_code: c_ulong) {
    void do_boot_page_fault(struct pt_regs *regs, unsigned long error_code)
    {
    let mut address: c_ulong = native_read_cr2();
    unsigned long end;
    bool ghcb_fault;
    ghcb_fault = sev_es_check_ghcb_fault(address);
    address   &= PMD_MASK;
    end        = address + PMD_SIZE;
//
// Check for unexpected error codes. Unexpected are:
// - Faults on present pages
// - User faults
// - Reserved bits set
//
    if (error_code & (X86_PF_PROT | X86_PF_USER | X86_PF_RSVD))
    do_pf_error("Unexpected page-fault:", error_code, address, regs.ip);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ghcb_fault) -> else {
    else if (ghcb_fault)
    do_pf_error("Page-fault on GHCB page:", error_code, address, regs.ip);
//
// Error code is sane - now identity map the 2M region around
// the faulting address.
//
    kernel_add_identity_map(address, end);
    }
#[no_mangle]
pub unsafe extern "C" fn do_boot_nmi_trap(regs: *mut pt_regs, error_code: c_ulong) {
    void do_boot_nmi_trap(struct pt_regs *regs, unsigned long error_code)
    {
    spurious_nmi_count++;
    }
