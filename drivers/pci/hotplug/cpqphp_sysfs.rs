//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/cpqphp_sysfs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Compaq Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001,2003 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>
//

    static DEFINE_MUTEX(cpqphp_mutex);
#[no_mangle]
unsafe extern "C" fn show_ctrl(ctrl: *mut controller, buf: *mut c_char) -> c_int {
    static int show_ctrl(struct controller *ctrl, char *buf)
    {
    char *out = buf;
    int index;
    struct pci_resource *res;
    out += sprintf(buf, "Free resources: memory\n");
    index = 11;
    res = ctrl.mem_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "Free resources: prefetchable memory\n");
    index = 11;
    res = ctrl.p_mem_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "Free resources: IO\n");
    index = 11;
    res = ctrl.io_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "Free resources: bus numbers\n");
    index = 11;
    res = ctrl.bus_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    return out - buf;
    }
#[no_mangle]
unsafe extern "C" fn show_dev(ctrl: *mut controller, buf: *mut c_char) -> c_int {
    static int show_dev(struct controller *ctrl, char *buf)
    {
    char *out = buf;
    int index;
    struct pci_resource *res;
    struct pci_func *new_slot;
    struct slot *slot;
    slot = ctrl.slot;
    while (slot) {
    new_slot = cpqhp_slot_find(slot.bus, slot.device, 0);
    if (!new_slot)
    break;
    out += sprintf(out, "assigned resources: memory\n");
    index = 11;
    res = new_slot.mem_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "assigned resources: prefetchable memory\n");
    index = 11;
    res = new_slot.p_mem_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "assigned resources: IO\n");
    index = 11;
    res = new_slot.io_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    out += sprintf(out, "assigned resources: bus numbers\n");
    index = 11;
    res = new_slot.bus_head;
    while (res && index--) {
    out += sprintf(out, "start = %8.8x, length = %8.8x\n", res.base, res.length);
    res = res.next;
    }
    slot = slot.next;
    }
    return out - buf;
    }
#[no_mangle]
unsafe extern "C" fn spew_debug_info(ctrl: *mut controller, data: *mut c_char, size: c_int) -> c_int {
    static int spew_debug_info(struct controller *ctrl, char *data, int size)
    {
    int used;
    used = size - show_ctrl(ctrl, data);
    used = (size - used) - show_dev(ctrl, &data[used]);
    return used;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_dbg {
    pub size: c_int,
    pub data: *mut c_char,
}

#[no_mangle]
unsafe extern "C" fn open(inode: *mut inode, file: *mut file) -> c_int {
    static int open(struct inode *inode, struct file *file)
    {
    struct controller *ctrl = inode.i_private;
    struct ctrl_dbg *dbg;
    let mut retval: c_int = -ENOMEM;
    mutex_lock(&cpqphp_mutex);
    dbg = kmalloc_obj(*dbg);
    if (!dbg)
    goto exit;
    dbg.data = kmalloc(MAX_OUTPUT, GFP_KERNEL);
    if (!dbg.data) {
    kfree(dbg);
    goto exit;
    }
    dbg.size = spew_debug_info(ctrl, dbg.data, MAX_OUTPUT);
    file.private_data = dbg;
    retval = 0;
    exit:
    mutex_unlock(&cpqphp_mutex);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lseek(file: *mut file, off: loff_t, whence: c_int) -> loff_t {
    static loff_t lseek(struct file *file, loff_t off, int whence)
    {
    struct ctrl_dbg *dbg = file.private_data;
    return fixed_size_llseek(file, off, whence, dbg.size);
    }
    static ssize_t read(struct file *file, char __user *buf,
    size_t nbytes, loff_t *ppos)
    {
    struct ctrl_dbg *dbg = file.private_data;
    return simple_read_from_buffer(buf, nbytes, ppos, dbg.data, dbg.size);
    }
#[no_mangle]
unsafe extern "C" fn release(inode: *mut inode, file: *mut file) -> c_int {
    static int release(struct inode *inode, struct file *file)
    {
    struct ctrl_dbg *dbg = file.private_data;
    kfree(dbg.data);
    kfree(dbg);
    return 0;
    }
    static const struct file_operations debug_ops = {
    .owner = THIS_MODULE,
    .open = open,
    .llseek = lseek,
    .read = read,
    .release = release,
    };
    static struct dentry *root;
#[no_mangle]
pub unsafe extern "C" fn cpqhp_initialize_debugfs() {
    void cpqhp_initialize_debugfs(void)
    {
    if (!root)
    root = debugfs_create_dir("cpqhp", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn cpqhp_shutdown_debugfs() {
    void cpqhp_shutdown_debugfs(void)
    {
    debugfs_remove(root);
    }
#[no_mangle]
pub unsafe extern "C" fn cpqhp_create_debugfs_files(ctrl: *mut controller) {
    void cpqhp_create_debugfs_files(struct controller *ctrl)
    {
    ctrl.dentry = debugfs_create_file(dev_name(&ctrl.pci_dev.dev),
    S_IRUGO, root, ctrl, &debug_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn cpqhp_remove_debugfs_files(ctrl: *mut controller) {
    void cpqhp_remove_debugfs_files(struct controller *ctrl)
    {
    debugfs_remove(ctrl.dentry);
    ctrl.dentry = core::ptr::null_mut();
    }
