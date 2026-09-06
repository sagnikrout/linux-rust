//! Automatically rewritten from C to Rust
//! Source: drivers/edac/debugfs.c
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

    static struct dentry *edac_debugfs;
#[no_mangle]
pub unsafe extern "C" fn edac_debugfs_init() -> void __init {
    void __init edac_debugfs_init(void)
    {
    edac_debugfs = debugfs_create_dir("edac", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn edac_debugfs_exit() {
    void edac_debugfs_exit(void)
    {
    debugfs_remove_recursive(edac_debugfs);
    }
#[no_mangle]
pub unsafe extern "C" fn edac_create_debugfs_nodes(mci: *mut mem_ctl_info) {
    void edac_create_debugfs_nodes(struct mem_ctl_info *mci)
    {
    mci.debugfs = debugfs_create_dir(mci.dev.kobj.name, edac_debugfs);
    }
// Create a toplevel dir under EDAC's debugfs hierarchy
    struct dentry *edac_debugfs_create_dir(const char *dirname)
    {
    if (!edac_debugfs)
    return core::ptr::null_mut();
    return debugfs_create_dir(dirname, edac_debugfs);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_dir);
// Create a toplevel dir under EDAC's debugfs hierarchy with parent @parent
    struct dentry *
    edac_debugfs_create_dir_at(const char *dirname, struct dentry *parent)
    {
    return debugfs_create_dir(dirname, parent);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_dir_at);
//
// Create a file under EDAC's hierarchy or a sub-hierarchy:
//
// @name: file name
// @mode: file permissions
// @parent: parent dentry. If NULL, it becomes the toplevel EDAC dir
// @data: private data of caller
// @fops: file operations of this file
//
    struct dentry *
    edac_debugfs_create_file(const char *name, umode_t mode, struct dentry *parent,
    void *data, const struct file_operations *fops)
    {
    if (!parent)
    parent = edac_debugfs;
    return debugfs_create_file(name, mode, parent, data, fops);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_file);
// Wrapper for debugfs_create_x8()
    void edac_debugfs_create_x8(const char *name, umode_t mode,
    struct dentry *parent, u8 *value)
    {
    if (!parent)
    parent = edac_debugfs;
    debugfs_create_x8(name, mode, parent, value);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_x8);
// Wrapper for debugfs_create_x16()
    void edac_debugfs_create_x16(const char *name, umode_t mode,
    struct dentry *parent, u16 *value)
    {
    if (!parent)
    parent = edac_debugfs;
    debugfs_create_x16(name, mode, parent, value);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_x16);
// Wrapper for debugfs_create_x32()
    void edac_debugfs_create_x32(const char *name, umode_t mode,
    struct dentry *parent, u32 *value)
    {
    if (!parent)
    parent = edac_debugfs;
    debugfs_create_x32(name, mode, parent, value);
    }
    EXPORT_SYMBOL_GPL(edac_debugfs_create_x32);
