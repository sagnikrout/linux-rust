//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xenfs/super.c
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
// xenfs.c - a filesystem for passing info between the a domain and
// the hypervisor.
//
// 2008-10-07  Alex Zeffertt    Replaced /proc/xen/xenbus with xenfs filesystem
// and /proc/xen compatibility mount point.
// Turned xenfs into a loadable module.
//

    MODULE_DESCRIPTION("Xen filesystem");
    MODULE_LICENSE("GPL");
    static ssize_t capabilities_read(struct file *file, char __user *buf,
    size_t size, loff_t *off)
    {
    char *tmp = "";
    if (xen_initial_domain())
    tmp = "control_d\n";
    return simple_read_from_buffer(buf, size, off, tmp, strlen(tmp));
    }
    static const struct file_operations capabilities_file_ops = {
    .read = capabilities_read,
    .llseek = default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn xenfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    static int xenfs_fill_super(struct super_block *sb, struct fs_context *fc)
    {
    static const struct tree_descr xenfs_files[] = {
    [2] = { "xenbus", &xen_xenbus_fops, S_IRUSR|S_IWUSR },
    { "capabilities", &capabilities_file_ops, S_IRUGO },
    { "privcmd", &xen_privcmd_fops, S_IRUSR|S_IWUSR },
    {""},
    };
    static const struct tree_descr xenfs_init_files[] = {
    [2] = { "xenbus", &xen_xenbus_fops, S_IRUSR|S_IWUSR },
    { "capabilities", &capabilities_file_ops, S_IRUGO },
    { "privcmd", &xen_privcmd_fops, S_IRUSR|S_IWUSR },
    { "xsd_kva", &xsd_kva_file_ops, S_IRUSR|S_IWUSR},
    { "xsd_port", &xsd_port_file_ops, S_IRUSR|S_IWUSR},

    { "xensyms", &xensyms_ops, S_IRUSR},

    {""},
    };
    return simple_fill_super(sb, XENFS_SUPER_MAGIC,
    xen_initial_domain() ? xenfs_init_files : xenfs_files);
    }
#[no_mangle]
unsafe extern "C" fn xenfs_get_tree(fc: *mut fs_context) -> c_int {
    static int xenfs_get_tree(struct fs_context *fc)
    {
    return get_tree_single(fc, xenfs_fill_super);
    }
    static const struct fs_context_operations xenfs_context_ops = {
    .get_tree	= xenfs_get_tree,
    };
#[no_mangle]
unsafe extern "C" fn xenfs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int xenfs_init_fs_context(struct fs_context *fc)
    {
    fc.ops = &xenfs_context_ops;
    return 0;
    }
    static struct file_system_type xenfs_type = {
    .owner =	THIS_MODULE,
    .name =		"xenfs",
    .init_fs_context = xenfs_init_fs_context,
    .kill_sb =	kill_anon_super,
    };
    MODULE_ALIAS_FS("xenfs");
#[no_mangle]
unsafe extern "C" fn xenfs_init() -> int __init {
    static int __init xenfs_init(void)
    {
    if (xen_domain())
    return register_filesystem(&xenfs_type);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xenfs_exit() -> void __exit {
    static void __exit xenfs_exit(void)
    {
    if (xen_domain())
    unregister_filesystem(&xenfs_type);
    }
    module_init(xenfs_init);
    module_exit(xenfs_exit);
