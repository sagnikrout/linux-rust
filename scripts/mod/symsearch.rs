//! Automatically rewritten from C to Rust
//! Source: scripts/mod/symsearch.c
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
// Helper functions for finding the symbol in an ELF which is "nearest"
// to a given address.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syminfo {
    pub symbol_index: c_uint,
    pub section_index: c_uint,
    pub addr: Elf_Addr,
}

//
// Container used to hold an entire binary search table.
// Entries in table are ascending, sorted first by section_index,
// then by addr, and last by symbol_index.  The sorting by
// symbol_index is used to ensure predictable behavior when
// multiple symbols are present with the same address; all
// symbols past the first are effectively ignored, by eliding
// them in symsearch_fixup().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symsearch {
    pub table_size: c_uint,
    pub table: [syminfo; ],
}

#[no_mangle]
unsafe extern "C" fn syminfo_compare(s1: *const c_void, s2: *const c_void) -> c_int {
    static int syminfo_compare(const void *s1, const void *s2)
    {
    const struct syminfo *sym1 = s1;
    const struct syminfo *sym2 = s2;
    if (sym1.section_index > sym2.section_index)
    return 1;
    if (sym1.section_index < sym2.section_index)
    return -1;
    if (sym1.addr > sym2.addr)
    return 1;
    if (sym1.addr < sym2.addr)
    return -1;
    if (sym1.symbol_index > sym2.symbol_index)
    return 1;
    if (sym1.symbol_index < sym2.symbol_index)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn symbol_count(elf: *mut elf_info) -> c_uint {
    static unsigned int symbol_count(struct elf_info *elf)
    {
    let mut result: c_uint = 0;
    for (Elf_Sym *sym = elf.symtab_start; sym < elf.symtab_stop; sym++) {
    if (is_valid_name(elf, sym))
    result++;
    }
    return result;
    }
//
// Populate the search array that we just allocated.
// Be slightly paranoid here.  The ELF file is mmap'd and could
// conceivably change between symbol_count() and symsearch_populate().
// If we notice any difference, bail out rather than potentially
// propagating errors or crashing.
//
    static void symsearch_populate(struct elf_info *elf,
    struct syminfo *table,
    unsigned int table_size)
    {
    let mut is_arm: bool = (elf.hdr.e_machine == EM_ARM);
    for (Elf_Sym *sym = elf.symtab_start; sym < elf.symtab_stop; sym++) {
    if (is_valid_name(elf, sym)) {
    if (table_size-- == 0)
    fatal("%s: size mismatch\n", __func__);
    table.symbol_index = sym - elf.symtab_start;
    table.section_index = get_secindex(elf, sym);
    table.addr = sym.st_value;
//
// For ARM Thumb instruction, the bit 0 of st_value is
// set if the symbol is STT_FUNC type. Mask it to get
// the address.
//
    if (is_arm && ELF_ST_TYPE(sym.st_info) == STT_FUNC)
    table.addr &= ~1;
    table++;
    }
    }
    if (table_size != 0)
    fatal("%s: size mismatch\n", __func__);
    }
//
// Do any fixups on the table after sorting.
// For now, this just finds adjacent entries which have
// the same section_index and addr, and it propagates
// the first symbol_index over the subsequent entries,
// so that only one symbol_index is seen for any given
// section_index and addr.  This ensures that whether
// we're looking at an address from "above" or "below"
// that we see the same symbol_index.
// This does leave some duplicate entries in the table;
// in practice, these are a small fraction of the
// total number of entries, and they are harmless to
// the binary search algorithm other than a few occasional
// unnecessary comparisons.
//
#[no_mangle]
unsafe extern "C" fn symsearch_fixup(table: *mut syminfo, table_size: c_uint) {
    static void symsearch_fixup(struct syminfo *table, unsigned int table_size)
    {
// Don't look at index 0, it will never change.
    for (unsigned int i = 1; i < table_size; i++) {
    if (table[i].addr == table[i - 1].addr &&
    table[i].section_index == table[i - 1].section_index) {
    table[i].symbol_index = table[i - 1].symbol_index;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn symsearch_init(elf: *mut elf_info) {
    void symsearch_init(struct elf_info *elf)
    {
    let mut table_size: c_uint = symbol_count(elf);
    elf.symsearch = xmalloc(sizeof(struct symsearch) +
    sizeof(struct syminfo) * table_size);
    elf.symsearch.table_size = table_size;
    symsearch_populate(elf, elf.symsearch.table, table_size);
    qsort(elf.symsearch.table, table_size,
    sizeof(struct syminfo), syminfo_compare);
    symsearch_fixup(elf.symsearch.table, table_size);
    }
#[no_mangle]
pub unsafe extern "C" fn symsearch_finish(elf: *mut elf_info) {
    void symsearch_finish(struct elf_info *elf)
    {
    free(elf.symsearch);
    elf.symsearch = core::ptr::null_mut();
    }
//
// Find the syminfo which is in secndx and "nearest" to addr.
// allow_negative: allow returning a symbol whose address is > addr.
// min_distance: ignore symbols which are further away than this.
//
// Returns a pointer into the symbol table for success.
// Returns NULL if no legal symbol is found within the requested range.
//
    Elf_Sym *symsearch_find_nearest(struct elf_info *elf, Elf_Addr addr,
    unsigned int secndx, bool allow_negative,
    Elf_Addr min_distance)
    {
    let mut hi: c_uint = elf.symsearch.table_size;
    let mut lo: c_uint = 0;
    struct syminfo *table = elf.symsearch.table;
    struct syminfo target;
    target.addr = addr;
    target.section_index = secndx;
    target.symbol_index = ~0;  /* compares greater than any actual index */
    while (hi > lo) {
    unsigned int mid = lo + (hi - lo) / 2;  /* Avoids overflow */
    if (syminfo_compare(&table[mid], &target) > 0)
    hi = mid;
    else
    lo = mid + 1;
    }
//
// table[hi], if it exists, is the first entry in the array which
// lies beyond target.  table[hi - 1], if it exists, is the last
// entry in the array which comes before target, including the
// case where it perfectly matches the section and the address.
//
// Note -- if the address we're looking up falls perfectly
// in the middle of two symbols, this is written to always
// prefer the symbol with the lower address.
//
    Elf_Sym *result = core::ptr::null_mut();
    if (allow_negative &&
    hi < elf.symsearch.table_size &&
    table[hi].section_index == secndx &&
    table[hi].addr - addr <= min_distance) {
    min_distance = table[hi].addr - addr;
    result = &elf.symtab_start[table[hi].symbol_index];
    }
    if (hi > 0 &&
    table[hi - 1].section_index == secndx &&
    addr - table[hi - 1].addr <= min_distance) {
    result = &elf.symtab_start[table[hi - 1].symbol_index];
    }
    return result;
    }
