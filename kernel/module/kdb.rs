//! Automatically rewritten from C to Rust
//! Source: kernel/module/kdb.c
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
// Module kdb support
//
// Copyright (C) 2010 Jason Wessel
//

//
// kdb_lsmod - This function implements the 'lsmod' command.  Lists
// currently loaded kernel modules.
// Mostly taken from userland lsmod.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_lsmod(argc: c_int, argv: *const c_char) -> c_int {
    int kdb_lsmod(int argc, const char **argv)
    {
    struct module *mod;
    if (argc != 0)
    return KDB_ARGCOUNT;
    kdb_printf("Module                  Size  modstruct     Used by\n");
    list_for_each_entry(mod, &modules, list) {
    if (mod.state == MODULE_STATE_UNFORMED)
    continue;
    kdb_printf("%-20s%8u", mod.name, mod.mem[MOD_TEXT].size);
    kdb_printf("/%8u", mod.mem[MOD_RODATA].size);
    kdb_printf("/%8u", mod.mem[MOD_RO_AFTER_INIT].size);
    kdb_printf("/%8u", mod.mem[MOD_DATA].size);
    kdb_printf("  0x%px ", (void *)mod);

    kdb_printf("%4d ", module_refcount(mod));

    if (mod.state == MODULE_STATE_GOING)
    kdb_printf(" (Unloading)");
#[no_mangle]
pub unsafe extern "C" fn if(MODULE_STATE_COMING: mod->state ==) -> else {
    else if (mod.state == MODULE_STATE_COMING)
    kdb_printf(" (Loading)");
    else
    kdb_printf(" (Live)");
    kdb_printf(" 0x%px", mod.mem[MOD_TEXT].base);
    kdb_printf("/0x%px", mod.mem[MOD_RODATA].base);
    kdb_printf("/0x%px", mod.mem[MOD_RO_AFTER_INIT].base);
    kdb_printf("/0x%px", mod.mem[MOD_DATA].base);

    {
    struct module_use *use;
    kdb_printf(" [ ");
    list_for_each_entry(use, &mod.source_list,
    source_list)
    kdb_printf("%s ", use.target.name);
    kdb_printf("]\n");
    }

    }
    return 0;
    }
