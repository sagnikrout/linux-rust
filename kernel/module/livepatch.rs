//! Automatically rewritten from C to Rust
//! Source: kernel/module/livepatch.c
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
// Module livepatch support
//
// Copyright (C) 2016 Jessica Yu <jeyu@redhat.com>
//

//
// Persist ELF information about a module. Copy the ELF header,
// section header table, section string table, and symtab section
// index from info to mod->klp_info.
//
#[no_mangle]
pub unsafe extern "C" fn copy_module_elf(mod: *mut module, info: *mut load_info) -> c_int {
    int copy_module_elf(struct module *mod, struct load_info *info)
    {
    unsigned int size, symndx;
    int ret;
    size = sizeof(*mod.klp_info);
    mod.klp_info = kmalloc(size, GFP_KERNEL);
    if (!mod.klp_info)
    return -ENOMEM;
// ELF header
    size = sizeof(mod.klp_info.hdr);
    memcpy(&mod.klp_info.hdr, info.hdr, size);
// ELF section header table
    size = sizeof(*info.sechdrs) * info.hdr.e_shnum;
    mod.klp_info.sechdrs = kmemdup(info.sechdrs, size, GFP_KERNEL);
    if (!mod.klp_info.sechdrs) {
    ret = -ENOMEM;
    goto free_info;
    }
// ELF section name string table
    size = info.sechdrs[info.hdr.e_shstrndx].sh_size;
    mod.klp_info.secstrings = kmemdup(info.secstrings, size, GFP_KERNEL);
    if (!mod.klp_info.secstrings) {
    ret = -ENOMEM;
    goto free_sechdrs;
    }
// ELF symbol section index
    symndx = info.index.sym;
    mod.klp_info.symndx = symndx;
//
// For livepatch modules, core_kallsyms.symtab is a complete
// copy of the original symbol table. Adjust sh_addr to point
// to core_kallsyms.symtab since the copy of the symtab in module
// init memory is freed at the end of do_init_module().
//
    mod.klp_info.sechdrs[symndx].sh_addr = (unsigned long)mod.core_kallsyms.symtab;
    return 0;
    free_sechdrs:
    kfree(mod.klp_info.sechdrs);
    free_info:
    kfree(mod.klp_info);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn free_module_elf(mod: *mut module) {
    void free_module_elf(struct module *mod)
    {
    kfree(mod.klp_info.sechdrs);
    kfree(mod.klp_info.secstrings);
    kfree(mod.klp_info);
    }
