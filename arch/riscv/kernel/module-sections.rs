//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/module-sections.c
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
// Copyright (C) 2014-2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
//
// Copyright (C) 2018 Andes Technology Corporation <zong@andestech.com>
//

#[no_mangle]
pub unsafe extern "C" fn module_emit_got_entry(mod: *mut module, val: c_ulong) -> c_ulong {
    unsigned long module_emit_got_entry(struct module *mod, unsigned long val)
    {
    struct mod_section *got_sec = &mod.arch.got;
    let mut i: c_int = got_sec.num_entries;
    struct got_entry *got = get_got_entry(val, got_sec);
    if (got)
    return (unsigned long)got;
// There is no duplicate entry, create a new one
    got = (struct got_entry *)got_sec.shdr.sh_addr;
    got[i] = emit_got_entry(val);
    got_sec.num_entries++;
    BUG_ON(got_sec.num_entries > got_sec.max_entries);
    return (unsigned long)&got[i];
    }
#[no_mangle]
pub unsafe extern "C" fn module_emit_plt_entry(mod: *mut module, val: c_ulong) -> c_ulong {
    unsigned long module_emit_plt_entry(struct module *mod, unsigned long val)
    {
    struct mod_section *got_plt_sec = &mod.arch.got_plt;
    struct got_entry *got_plt;
    struct mod_section *plt_sec = &mod.arch.plt;
    struct plt_entry *plt = get_plt_entry(val, plt_sec, got_plt_sec);
    let mut i: c_int = plt_sec.num_entries;
    if (plt)
    return (unsigned long)plt;
// There is no duplicate entry, create a new one
    got_plt = (struct got_entry *)got_plt_sec.shdr.sh_addr;
    got_plt[i] = emit_got_entry(val);
    plt = (struct plt_entry *)plt_sec.shdr.sh_addr;
    plt[i] = emit_plt_entry(val,
    (unsigned long)&plt[i],
    (unsigned long)&got_plt[i]);
    plt_sec.num_entries++;
    got_plt_sec.num_entries++;
    BUG_ON(plt_sec.num_entries > plt_sec.max_entries);
    return (unsigned long)&plt[i];
    }
#[no_mangle]
unsafe extern "C" fn cmp_rela(a: *const c_void, b: *const c_void) -> c_int {
    static int cmp_rela(const void *a, const void *b)
    {
    const Elf_Rela *x = a, *y = b;
    int i;
// sort by type, symbol index and addend
    i = cmp_int(x.r_info, y.r_info);
    if (i == 0)
    i = cmp_int(x.r_addend, y.r_addend);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn duplicate_rela(rela: *const Elf_Rela, idx: c_int) -> bool {
    static bool duplicate_rela(const Elf_Rela *rela, int idx)
    {
//
// Entries are sorted by type, symbol index and addend. That means
// that, if a duplicate entry exists, it must be in the preceding slot.
//
    return idx > 0 && cmp_rela(rela + idx, rela + idx - 1) == 0;
    }
    static void count_max_entries(const Elf_Rela *relas, size_t num,
    unsigned int *plts, unsigned int *gots)
    {
    for (size_t i = 0; i < num; i++) {
    if (duplicate_rela(relas, i))
    continue;
    switch (ELF_R_TYPE(relas[i].r_info)) {
    case R_RISCV_CALL_PLT:
    case R_RISCV_PLT32:
    (*plts)++;
    break;
    case R_RISCV_GOT_HI20:
    (*gots)++;
    break;
    default:
    unreachable();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn rela_needs_plt_got_entry(rela: *const Elf_Rela) -> bool {
    static bool rela_needs_plt_got_entry(const Elf_Rela *rela)
    {
    switch (ELF_R_TYPE(rela.r_info)) {
    case R_RISCV_CALL_PLT:
    case R_RISCV_GOT_HI20:
    case R_RISCV_PLT32:
    return true;
    default:
    return false;
    }
    }
    int module_frob_arch_sections(Elf_Ehdr *ehdr, Elf_Shdr *sechdrs,
    char *secstrings, struct module *mod)
    {
    let mut num_scratch_relas: usize = 0;
    let mut num_plts: c_uint = 0;
    let mut num_gots: c_uint = 0;
    Elf_Rela *scratch = core::ptr::null_mut();
    Elf_Rela *new_scratch;
    let mut scratch_size: usize = 0;
    int i;
//
// Find the empty .got and .plt sections.
//
    for (i = 0; i < ehdr.e_shnum; i++) {
    if (!strcmp(secstrings + sechdrs[i].sh_name, ".plt"))
    mod.arch.plt.shdr = sechdrs + i;
#[no_mangle]
pub unsafe extern "C" fn if(sechdrs[i].sh_name: !strcmp(secstrings +, _arg: ".got")) -> else {
    else if (!strcmp(secstrings + sechdrs[i].sh_name, ".got"))
    mod.arch.got.shdr = sechdrs + i;
#[no_mangle]
pub unsafe extern "C" fn if(sechdrs[i].sh_name: !strcmp(secstrings +, _arg: ".got.plt")) -> else {
    else if (!strcmp(secstrings + sechdrs[i].sh_name, ".got.plt"))
    mod.arch.got_plt.shdr = sechdrs + i;
    }
    if (!mod.arch.plt.shdr) {
    pr_err("%s: module PLT section(s) missing\n", mod.name);
    return -ENOEXEC;
    }
    if (!mod.arch.got.shdr) {
    pr_err("%s: module GOT section(s) missing\n", mod.name);
    return -ENOEXEC;
    }
    if (!mod.arch.got_plt.shdr) {
    pr_err("%s: module GOT.PLT section(s) missing\n", mod.name);
    return -ENOEXEC;
    }
// Calculate the maximum number of entries
    for (i = 0; i < ehdr.e_shnum; i++) {
    let mut num_relas: usize = sechdrs[i].sh_size / sizeof(Elf_Rela);
    Elf_Rela *relas = (void *)ehdr + sechdrs[i].sh_offset;
    Elf_Shdr *dst_sec = sechdrs + sechdrs[i].sh_info;
    size_t scratch_size_needed;
    if (sechdrs[i].sh_type != SHT_RELA)
    continue;
// ignore relocations that operate on non-exec sections
    if (!(dst_sec.sh_flags & SHF_EXECINSTR))
    continue;
//
// apply_relocate_add() relies on HI20 and LO12 relocation pairs being
// close together, so sort a copy of the section to avoid interfering.
//
    scratch_size_needed = (num_scratch_relas + num_relas) * sizeof(*scratch);
    if (scratch_size_needed > scratch_size) {
    scratch_size = scratch_size_needed;
    new_scratch = kvrealloc(scratch, scratch_size, GFP_KERNEL);
    if (!new_scratch) {
    kvfree(scratch);
    return -ENOMEM;
    }
    scratch = new_scratch;
    }
    for (size_t j = 0; j < num_relas; j++)
    if (rela_needs_plt_got_entry(&relas[j]))
    scratch[num_scratch_relas++] = relas[j];
    }
    if (scratch) {
// sort the accumulated PLT/GOT relocations so duplicates are adjacent
    sort(scratch, num_scratch_relas, sizeof(*scratch), cmp_rela, core::ptr::null_mut());
    count_max_entries(scratch, num_scratch_relas, &num_plts, &num_gots);
    kvfree(scratch);
    }
    mod.arch.plt.shdr.sh_type = SHT_NOBITS;
    mod.arch.plt.shdr.sh_flags = SHF_EXECINSTR | SHF_ALLOC;
    mod.arch.plt.shdr.sh_addralign = L1_CACHE_BYTES;
    mod.arch.plt.shdr.sh_size = (num_plts + 1) * sizeof(struct plt_entry);
    mod.arch.plt.num_entries = 0;
    mod.arch.plt.max_entries = num_plts;
    mod.arch.got.shdr.sh_type = SHT_NOBITS;
    mod.arch.got.shdr.sh_flags = SHF_ALLOC;
    mod.arch.got.shdr.sh_addralign = L1_CACHE_BYTES;
    mod.arch.got.shdr.sh_size = (num_gots + 1) * sizeof(struct got_entry);
    mod.arch.got.num_entries = 0;
    mod.arch.got.max_entries = num_gots;
    mod.arch.got_plt.shdr.sh_type = SHT_NOBITS;
    mod.arch.got_plt.shdr.sh_flags = SHF_ALLOC;
    mod.arch.got_plt.shdr.sh_addralign = L1_CACHE_BYTES;
    mod.arch.got_plt.shdr.sh_size = (num_plts + 1) * sizeof(struct got_entry);
    mod.arch.got_plt.num_entries = 0;
    mod.arch.got_plt.max_entries = num_plts;
    return 0;
    }
