//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/debug_pagetables.c
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

#[no_mangle]
unsafe extern "C" fn ptdump_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_show(struct seq_file *m, void *v)
    {
    ptdump_walk_pgd_level_debugfs(m, &init_mm, false);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump);
#[no_mangle]
unsafe extern "C" fn ptdump_curknl_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_curknl_show(struct seq_file *m, void *v)
    {
    if (current.mm.pgd)
    ptdump_walk_pgd_level_debugfs(m, current.mm, false);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump_curknl);

#[no_mangle]
unsafe extern "C" fn ptdump_curusr_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_curusr_show(struct seq_file *m, void *v)
    {
    if (current.mm.pgd)
    ptdump_walk_pgd_level_debugfs(m, current.mm, true);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump_curusr);

#[no_mangle]
unsafe extern "C" fn ptdump_efi_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_efi_show(struct seq_file *m, void *v)
    {
    if (efi_mm.pgd)
    ptdump_walk_pgd_level_debugfs(m, &efi_mm, false);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump_efi);

    static struct dentry *dir;
#[no_mangle]
unsafe extern "C" fn pt_dump_debug_init() -> int __init {
    static int __init pt_dump_debug_init(void)
    {
    dir = debugfs_create_dir("page_tables", core::ptr::null_mut());
    debugfs_create_file("kernel", 0400, dir, core::ptr::null_mut(), &ptdump_fops);
    debugfs_create_file("current_kernel", 0400, dir, core::ptr::null_mut(),
    &ptdump_curknl_fops);

    debugfs_create_file("current_user", 0400, dir, core::ptr::null_mut(),
    &ptdump_curusr_fops);

    debugfs_create_file("efi", 0400, dir, core::ptr::null_mut(), &ptdump_efi_fops);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pt_dump_debug_exit() -> void __exit {
    static void __exit pt_dump_debug_exit(void)
    {
    debugfs_remove_recursive(dir);
    }
    module_init(pt_dump_debug_init);
    module_exit(pt_dump_debug_exit);
    MODULE_AUTHOR("Arjan van de Ven <arjan@linux.intel.com>");
    MODULE_DESCRIPTION("Kernel debugging helper that dumps pagetables");
