//! Automatically rewritten from C to Rust
//! Source: kernel/module/strict_rwx.c
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
// Module strict rwx
//
// Copyright (C) 2015 Rusty Russell
//

    static int module_set_memory(const struct module *mod, enum mod_mem_type type,
    int (*set_memory)(unsigned long start, int num_pages))
    {
    const struct module_memory *mod_mem = &mod.mem[type];
    if (!mod_mem.base)
    return 0;
    set_vm_flush_reset_perms(mod_mem.base);
    return set_memory((unsigned long)mod_mem.base, mod_mem.size >> PAGE_SHIFT);
    }
//
// Since some arches are moving towards PAGE_KERNEL module allocations instead
// of PAGE_KERNEL_EXEC, keep module_enable_x() independent of
// CONFIG_STRICT_MODULE_RWX because they are needed regardless of whether we
// are strict.
//
#[no_mangle]
pub unsafe extern "C" fn module_enable_text_rox(mod: *const module) -> c_int {
    int module_enable_text_rox(const struct module *mod)
    {
    for_class_mod_mem_type(type, text) {
    const struct module_memory *mem = &mod.mem[type];
    int ret;
    if (mem.is_rox)
    ret = execmem_restore_rox(mem.base, mem.size);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_STRICT_MODULE_RWX)) -> else {
    else if (IS_ENABLED(CONFIG_STRICT_MODULE_RWX))
    ret = module_set_memory(mod, type, set_memory_rox);
    else
    ret = module_set_memory(mod, type, set_memory_x);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn module_enable_rodata_ro(mod: *const module) -> c_int {
    int module_enable_rodata_ro(const struct module *mod)
    {
    int ret;
    if (!IS_ENABLED(CONFIG_STRICT_MODULE_RWX) || !rodata_enabled)
    return 0;
    ret = module_set_memory(mod, MOD_RODATA, set_memory_ro);
    if (ret)
    return ret;
    ret = module_set_memory(mod, MOD_INIT_RODATA, set_memory_ro);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn module_enable_rodata_ro_after_init(mod: *const module) -> c_int {
    int module_enable_rodata_ro_after_init(const struct module *mod)
    {
    if (!IS_ENABLED(CONFIG_STRICT_MODULE_RWX) || !rodata_enabled)
    return 0;
    return module_set_memory(mod, MOD_RO_AFTER_INIT, set_memory_ro);
    }
#[no_mangle]
pub unsafe extern "C" fn module_enable_data_nx(mod: *const module) -> c_int {
    int module_enable_data_nx(const struct module *mod)
    {
    if (!IS_ENABLED(CONFIG_STRICT_MODULE_RWX))
    return 0;
    for_class_mod_mem_type(type, data) {
    let mut ret: c_int = module_set_memory(mod, type, set_memory_nx);
    if (ret)
    return ret;
    }
    return 0;
    }
    int module_enforce_rwx_sections(const Elf_Ehdr *hdr, const Elf_Shdr *sechdrs,
    const char *secstrings,
    const struct module *mod)
    {
    let mut shf_wx: c_ulong = SHF_WRITE | SHF_EXECINSTR;
    int i;
    if (!IS_ENABLED(CONFIG_STRICT_MODULE_RWX))
    return 0;
    for (i = 0; i < hdr.e_shnum; i++) {
    if ((sechdrs[i].sh_flags & shf_wx) == shf_wx) {
    pr_err("%s: section %s (index %d) has invalid WRITE|EXEC flags\n",
    mod.name, secstrings + sechdrs[i].sh_name, i);
    return -ENOEXEC;
    }
    }
    return 0;
    }
    static const char *const ro_after_init[] = {
//
// Section .data..ro_after_init holds data explicitly annotated by
// __ro_after_init.
//
    ".data..ro_after_init",
//
// Section __jump_table holds data structures that are never modified,
// with the exception of entries that refer to code in the __init
// section, which are marked as such at module load time.
//
    "__jump_table",

//
// Section .static_call_sites holds data structures that need to be
// sorted and processed at module load time but are never modified
// afterwards.
//
    ".static_call_sites",

    };
    void module_mark_ro_after_init(const Elf_Ehdr *hdr, Elf_Shdr *sechdrs,
    const char *secstrings)
    {
    int i, j;
    for (i = 1; i < hdr.e_shnum; i++) {
    Elf_Shdr *shdr = &sechdrs[i];
    for (j = 0; j < ARRAY_SIZE(ro_after_init); j++) {
    if (strcmp(secstrings + shdr.sh_name,
    ro_after_init[j]) == 0) {
    shdr.sh_flags |= SHF_RO_AFTER_INIT;
    break;
    }
    }
    }
    }
