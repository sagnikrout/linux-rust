//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/proc_powerpc.c
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
// Copyright (C) 2001 Mike Corrigan & Dave Engebretsen IBM Corporation
//

#[no_mangle]
unsafe extern "C" fn page_map_seek(file: *mut file, off: loff_t, whence: c_int) -> loff_t {
    static loff_t page_map_seek(struct file *file, loff_t off, int whence)
    {
    return fixed_size_llseek(file, off, whence, PAGE_SIZE);
    }
    static ssize_t page_map_read( struct file *file, char __user *buf, size_t nbytes,
    loff_t *ppos)
    {
    return simple_read_from_buffer(buf, nbytes, ppos,
    pde_data(file_inode(file)), PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn page_map_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int page_map_mmap( struct file *file, struct vm_area_struct *vma )
    {
    if ((vma.vm_end - vma.vm_start) > PAGE_SIZE)
    return -EINVAL;
    return remap_pfn_range(vma, vma.vm_start,
    __pa(pde_data(file_inode(file))) >> PAGE_SHIFT,
    PAGE_SIZE, vma.vm_page_prot);
    }
    static const struct proc_ops page_map_proc_ops = {
    .proc_lseek	= page_map_seek,
    .proc_read	= page_map_read,
    .proc_mmap	= page_map_mmap,
    };
    static union {
    struct systemcfg	data;
    u8			page[PAGE_SIZE];
    } systemcfg_data_store __page_aligned_data;
    struct systemcfg *systemcfg = &systemcfg_data_store.data;
#[no_mangle]
unsafe extern "C" fn proc_ppc64_init() -> int __init {
    static int __init proc_ppc64_init(void)
    {
    struct proc_dir_entry *pde;
    strscpy(systemcfg.eye_catcher, "SYSTEMCFG:PPC64");
    systemcfg.version.major = SYSTEMCFG_MAJOR;
    systemcfg.version.minor = SYSTEMCFG_MINOR;
    systemcfg.processor = mfspr(SPRN_PVR);
//
// Fake the old platform number for pSeries and add
// in LPAR bit if necessary
//
    systemcfg.platform = 0x100;
    if (firmware_has_feature(FW_FEATURE_LPAR))
    systemcfg.platform |= 1;
    systemcfg.physicalMemorySize = memblock_phys_mem_size();
    systemcfg.dcache_size = ppc64_caches.l1d.size;
    systemcfg.dcache_line_size = ppc64_caches.l1d.line_size;
    systemcfg.icache_size = ppc64_caches.l1i.size;
    systemcfg.icache_line_size = ppc64_caches.l1i.line_size;
    pde = proc_create_data("powerpc/systemcfg", S_IFREG | 0444, core::ptr::null_mut(),
    &page_map_proc_ops, systemcfg);
    if (!pde)
    return 1;
    proc_set_size(pde, PAGE_SIZE);
    return 0;
    }
    __initcall(proc_ppc64_init);

//
// Create the ppc64 and ppc64/rtas directories early. This allows us to
// assume that they have been previously created in drivers.
//
#[no_mangle]
unsafe extern "C" fn proc_ppc64_create() -> int __init {
    static int __init proc_ppc64_create(void)
    {
    struct proc_dir_entry *root;
    root = proc_mkdir("powerpc", core::ptr::null_mut());
    if (!root)
    return 1;

    if (!proc_symlink("ppc64", core::ptr::null_mut(), "powerpc"))
    pr_err("Failed to create link /proc/ppc64 . /proc/powerpc\n");

    if (!of_find_node_by_path("/rtas"))
    return 0;
    if (!proc_mkdir("rtas", root))
    return 1;
    if (!proc_symlink("rtas", core::ptr::null_mut(), "powerpc/rtas"))
    return 1;
    return 0;
    }
    core_initcall(proc_ppc64_create);
