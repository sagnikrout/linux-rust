//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/kexec_handover_debugfs.c
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
//
// kexec_handover_debugfs.c - kexec handover debugfs interfaces
// Copyright (C) 2023 Alexander Graf <graf@amazon.com>
// Copyright (C) 2025 Microsoft Corporation, Mike Rapoport <rppt@kernel.org>
// Copyright (C) 2025 Google LLC, Changyuan Lyu <changyuanl@google.com>
// Copyright (C) 2025 Google LLC, Pasha Tatashin <pasha.tatashin@soleen.com>
//

    static struct dentry *debugfs_root;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_debugfs {
    pub list: list_head,
    pub wrapper: debugfs_blob_wrapper,
    pub file: *mut dentry,
}

    static int __kho_debugfs_blob_add(struct list_head *list, struct dentry *dir,
    const char *name, const void *blob,
    size_t size)
    {
    struct fdt_debugfs *f;
    struct dentry *file;
    f = kmalloc_obj(*f);
    if (!f)
    return -ENOMEM;
    f.wrapper.data = (void *)blob;
    f.wrapper.size = size;
    file = debugfs_create_blob(name, 0400, dir, &f.wrapper);
    if (IS_ERR(file)) {
    kfree(f);
    return PTR_ERR(file);
    }
    f.file = file;
    list_add(&f.list, list);
    return 0;
    }
    int kho_debugfs_blob_add(struct kho_debugfs *dbg, const char *name,
    const void *blob, size_t size, bool root)
    {
    struct dentry *dir;
    if (root)
    dir = dbg.dir;
    else
    dir = dbg.sub_fdt_dir;
    return __kho_debugfs_blob_add(&dbg.fdt_list, dir, name, blob, size);
    }
#[no_mangle]
pub unsafe extern "C" fn kho_debugfs_blob_remove(dbg: *mut kho_debugfs, blob: *mut c_void) {
    void kho_debugfs_blob_remove(struct kho_debugfs *dbg, void *blob)
    {
    struct fdt_debugfs *ff;
    list_for_each_entry(ff, &dbg.fdt_list, list) {
    if (ff.wrapper.data == blob) {
    debugfs_remove(ff.file);
    list_del(&ff.list);
    kfree(ff);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn scratch_phys_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int scratch_phys_show(struct seq_file *m, void *v)
    {
    for (int i = 0; i < kho_scratch_cnt; i++)
    seq_printf(m, "0x%llx\n", kho_scratch[i].addr);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(scratch_phys);
#[no_mangle]
unsafe extern "C" fn scratch_len_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int scratch_len_show(struct seq_file *m, void *v)
    {
    for (int i = 0; i < kho_scratch_cnt; i++)
    seq_printf(m, "0x%llx\n", kho_scratch[i].size);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(scratch_len);
#[no_mangle]
pub unsafe extern "C" fn kho_in_debugfs_init(dbg: *mut kho_debugfs, fdt: *const c_void) -> __init void {
    __init void kho_in_debugfs_init(struct kho_debugfs *dbg, const void *fdt)
    {
    struct dentry *dir, *sub_fdt_dir;
    int err, child;
    INIT_LIST_HEAD(&dbg.fdt_list);
    dir = debugfs_create_dir("in", debugfs_root);
    if (IS_ERR(dir)) {
    err = PTR_ERR(dir);
    goto err_out;
    }
    sub_fdt_dir = debugfs_create_dir("sub_fdts", dir);
    if (IS_ERR(sub_fdt_dir)) {
    err = PTR_ERR(sub_fdt_dir);
    goto err_rmdir;
    }
    err = __kho_debugfs_blob_add(&dbg.fdt_list, dir, "fdt", fdt,
    fdt_totalsize(fdt));
    if (err)
    goto err_rmdir;
    fdt_for_each_subnode(child, fdt, 0) {
    let mut len: c_int = 0;
    const char *name = fdt_get_name(fdt, child, core::ptr::null_mut());
    const u64 *blob_phys;
    const u64 *blob_size;
    void *blob;
    blob_phys = fdt_getprop(fdt, child,
    KHO_SUB_TREE_PROP_NAME, &len);
    if (!blob_phys)
    continue;
    if (len != sizeof(*blob_phys)) {
    pr_warn("node %s prop %s has invalid length: %d\n",
    name, KHO_SUB_TREE_PROP_NAME, len);
    continue;
    }
    blob_size = fdt_getprop(fdt, child,
    KHO_SUB_TREE_SIZE_PROP_NAME, &len);
    if (!blob_size || len != sizeof(*blob_size)) {
    pr_warn("node %s missing or invalid %s property\n",
    name, KHO_SUB_TREE_SIZE_PROP_NAME);
    continue;
    }
    blob = phys_to_virt(*blob_phys);
    err = __kho_debugfs_blob_add(&dbg.fdt_list, sub_fdt_dir, name,
    blob, *blob_size);
    if (err) {
    pr_warn("failed to add blob %s to debugfs: %pe\n",
    name, ERR_PTR(err));
    continue;
    }
    }
    dbg.dir = dir;
    dbg.sub_fdt_dir = sub_fdt_dir;
    return;
    err_rmdir:
    debugfs_remove_recursive(dir);
    err_out:
//
// Failure to create /sys/kernel/debug/kho/in does not prevent
// reviving state from KHO and setting up KHO for the next
// kexec.
//
    if (err) {
    pr_err("failed exposing handover FDT in debugfs: %pe\n",
    ERR_PTR(err));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kho_out_debugfs_init(dbg: *mut kho_debugfs) -> __init int {
    __init int kho_out_debugfs_init(struct kho_debugfs *dbg)
    {
    struct dentry *dir, *f, *sub_fdt_dir;
    INIT_LIST_HEAD(&dbg.fdt_list);
    dir = debugfs_create_dir("out", debugfs_root);
    if (IS_ERR(dir))
    return -ENOMEM;
    sub_fdt_dir = debugfs_create_dir("sub_fdts", dir);
    if (IS_ERR(sub_fdt_dir))
    goto err_rmdir;
    f = debugfs_create_file("scratch_phys", 0400, dir, core::ptr::null_mut(),
    &scratch_phys_fops);
    if (IS_ERR(f))
    goto err_rmdir;
    f = debugfs_create_file("scratch_len", 0400, dir, core::ptr::null_mut(),
    &scratch_len_fops);
    if (IS_ERR(f))
    goto err_rmdir;
    dbg.dir = dir;
    dbg.sub_fdt_dir = sub_fdt_dir;
    return 0;
    err_rmdir:
    debugfs_remove_recursive(dir);
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_debugfs_init() -> __init int {
    __init int kho_debugfs_init(void)
    {
    debugfs_root = debugfs_create_dir("kho", core::ptr::null_mut());
    if (IS_ERR(debugfs_root))
    return -ENOENT;
    return 0;
    }
