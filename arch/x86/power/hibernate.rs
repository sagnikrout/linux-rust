//! Automatically rewritten from C to Rust
//! Source: arch/x86/power/hibernate.c
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
// Hibernation support for x86
//
// Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
// Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
// Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
//

//
// Address to jump to in the last phase of restore in order to get to the image
// kernel's text (this value is passed in the image header).
//
    unsigned long restore_jump_address __visible;
    unsigned long jump_address_phys;
//
// Value of the cr3 register from before the hibernation (this value is passed
// in the image header).
//
    unsigned long restore_cr3 __visible;
    unsigned long temp_pgt __visible;
    unsigned long relocated_restore_code __visible;
//
// pfn_is_nosave - check if given pfn is in the 'nosave' section
// @pfn: the page frame number to check.
//
#[no_mangle]
pub unsafe extern "C" fn pfn_is_nosave(pfn: c_ulong) -> c_int {
    int pfn_is_nosave(unsigned long pfn)
    {
    unsigned long nosave_begin_pfn;
    unsigned long nosave_end_pfn;
    nosave_begin_pfn = __pa_symbol(&__nosave_begin) >> PAGE_SHIFT;
    nosave_end_pfn = PAGE_ALIGN(__pa_symbol(&__nosave_end)) >> PAGE_SHIFT;
    return pfn >= nosave_begin_pfn && pfn < nosave_end_pfn;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct restore_data_record {
    pub jump_address: c_ulong,
    pub jump_address_phys: c_ulong,
    pub cr3: c_ulong,
    pub magic: c_ulong,
    pub e820_checksum: c_ulong,
}

//
// compute_e820_crc32 - calculate crc32 of a given e820 table
//
// @table: the e820 table to be calculated
//
// Return: the resulting checksum
//
#[no_mangle]
pub unsafe extern "C" fn compute_e820_crc32(table: *mut e820_table) -> u32 {
    static inline u32 compute_e820_crc32(struct e820_table *table)
    {
    int size = offsetof(struct e820_table, entries) +
    sizeof(struct e820_entry) * table.nr_entries;
    return ~crc32_le(~0, (unsigned char const *)table, size);
    }

pub const RESTORE_MAGIC: c_uint = 0x23456789ABCDEF02UL;

pub const RESTORE_MAGIC: c_uint = 0x12345679UL;

//
// arch_hibernation_header_save - populate the architecture specific part
// of a hibernation image header
// @addr: address where architecture specific header data will be saved.
// @max_size: maximum size of architecture specific data in hibernation header.
//
// Return: 0 on success, -EOVERFLOW if max_size is insufficient.
//
#[no_mangle]
pub unsafe extern "C" fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int {
    int arch_hibernation_header_save(void *addr, unsigned int max_size)
    {
    struct restore_data_record *rdr = addr;
    if (max_size < sizeof(struct restore_data_record))
    return -EOVERFLOW;
    rdr.magic = RESTORE_MAGIC;
    rdr.jump_address = (unsigned long)restore_registers;
    rdr.jump_address_phys = __pa_symbol(restore_registers);
//
// The restore code fixes up CR3 and CR4 in the following sequence:
//
// [in hibernation asm]
// 1. CR3 <= temporary page tables
// 2. CR4 <= mmu_cr4_features (from the kernel that restores us)
// 3. CR3 <= rdr->cr3
// 4. CR4 <= mmu_cr4_features (from us, i.e. the image kernel)
// [in restore_processor_state()]
// 5. CR4 <= saved CR4
// 6. CR3 <= saved CR3
//
// Our mmu_cr4_features has CR4.PCIDE=0, and toggling
// CR4.PCIDE while CR3's PCID bits are nonzero is illegal, so
// rdr->cr3 needs to point to valid page tables but must not
// have any of the PCID bits set.
//
    rdr.cr3 = restore_cr3 & ~CR3_PCID_MASK;
    rdr.e820_checksum = compute_e820_crc32(e820_table_firmware);
    return 0;
    }
//
// arch_hibernation_header_restore - read the architecture specific data
// from the hibernation image header
// @addr: address to read the data from
//
#[no_mangle]
pub unsafe extern "C" fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int {
    int arch_hibernation_header_restore(void *addr)
    {
    struct restore_data_record *rdr = addr;
    if (rdr.magic != RESTORE_MAGIC) {
    pr_crit("Unrecognized hibernate image header format!\n");
    return -EINVAL;
    }
    restore_jump_address = rdr.jump_address;
    jump_address_phys = rdr.jump_address_phys;
    restore_cr3 = rdr.cr3;
    if (rdr.e820_checksum != compute_e820_crc32(e820_table_firmware)) {
    pr_crit("Hibernate inconsistent memory map detected!\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn relocate_restore_code() -> c_int {
    int relocate_restore_code(void)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    relocated_restore_code = get_safe_page(GFP_ATOMIC);
    if (!relocated_restore_code)
    return -ENOMEM;
    __memcpy((void *)relocated_restore_code, core_restore_code, PAGE_SIZE);
// Make the page containing the relocated code executable
    pgd = (pgd_t *)__va(read_cr3_pa()) +
    pgd_index(relocated_restore_code);
    p4d = p4d_offset(pgd, relocated_restore_code);
    if (p4d_leaf(*p4d)) {
    set_p4d(p4d, __p4d(p4d_val(*p4d) & ~_PAGE_NX));
    goto out;
    }
    pud = pud_offset(p4d, relocated_restore_code);
    if (pud_leaf(*pud)) {
    set_pud(pud, __pud(pud_val(*pud) & ~_PAGE_NX));
    goto out;
    }
    pmd = pmd_offset(pud, relocated_restore_code);
    if (pmd_leaf(*pmd)) {
    set_pmd(pmd, __pmd(pmd_val(*pmd) & ~_PAGE_NX));
    goto out;
    }
    pte = pte_offset_kernel(pmd, relocated_restore_code);
    set_pte(pte, __pte(pte_val(*pte) & ~_PAGE_NX));
    out:
    __flush_tlb_all();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_resume_nosmt() -> c_int {
    int arch_resume_nosmt(void)
    {
    int ret;
//
// We reached this while coming out of hibernation. This means
// that SMT siblings are sleeping in hlt, as mwait is not safe
// against control transition during resume (see comment in
// hibernate_resume_nonboot_cpu_disable()).
//
// If the resumed kernel has SMT disabled, we have to take all the
// SMT siblings out of hlt, and offline them again so that they
// end up in mwait proper.
//
// Called with hotplug disabled.
//
    cpu_hotplug_enable();
    ret = arch_cpu_rescan_dead_smt_siblings();
    cpu_hotplug_disable();
    return ret;
    }
