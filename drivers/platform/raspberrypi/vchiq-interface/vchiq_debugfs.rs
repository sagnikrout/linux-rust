//! Automatically rewritten from C to Rust
//! Source: drivers/platform/raspberrypi/vchiq-interface/vchiq_debugfs.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2014 Raspberry Pi (Trading) Ltd. All rights reserved.
// Copyright (c) 2010-2012 Broadcom. All rights reserved.
//

pub const DEBUGFS_WRITE_BUF_SIZE: c_int = 256;
// Global 'vchiq' debugfs and clients entry used by all instances
    static struct dentry *vchiq_dbg_dir;
    static struct dentry *vchiq_dbg_clients;
#[no_mangle]
unsafe extern "C" fn debugfs_usecount_show(f: *mut seq_file, offset: *mut c_void) -> c_int {
    static int debugfs_usecount_show(struct seq_file *f, void *offset)
    {
    struct vchiq_instance *instance = f.private;
    int use_count;
    use_count = vchiq_instance_get_use_count(instance);
    seq_printf(f, "%d\n", use_count);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(debugfs_usecount);
#[no_mangle]
unsafe extern "C" fn debugfs_trace_show(f: *mut seq_file, offset: *mut c_void) -> c_int {
    static int debugfs_trace_show(struct seq_file *f, void *offset)
    {
    struct vchiq_instance *instance = f.private;
    int trace;
    trace = vchiq_instance_get_trace(instance);
    seq_printf(f, "%s\n", trace ? "Y" : "N");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vchiq_dump_show(f: *mut seq_file, offset: *mut c_void) -> c_int {
    static int vchiq_dump_show(struct seq_file *f, void *offset)
    {
    struct vchiq_state *state = f.private;
    vchiq_dump_state(f, state);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(vchiq_dump);
#[no_mangle]
unsafe extern "C" fn debugfs_trace_open(inode: *mut inode, file: *mut file) -> c_int {
    static int debugfs_trace_open(struct inode *inode, struct file *file)
    {
    return single_open(file, debugfs_trace_show, inode.i_private);
    }
    static ssize_t debugfs_trace_write(struct file *file,
    const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    struct seq_file *f = (struct seq_file *)file.private_data;
    struct vchiq_instance *instance = f.private;
    char firstchar;
    if (copy_from_user(&firstchar, buffer, 1))
    return -EFAULT;
    switch (firstchar) {
    case 'Y':
    case 'y':
    case '1':
    vchiq_instance_set_trace(instance, 1);
    break;
    case 'N':
    case 'n':
    case '0':
    vchiq_instance_set_trace(instance, 0);
    break;
    default:
    break;
    }
// ppos += count;
    return count;
    }
    static const struct file_operations debugfs_trace_fops = {
    .owner		= THIS_MODULE,
    .open		= debugfs_trace_open,
    .write		= debugfs_trace_write,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .release	= single_release,
    };
// add an instance (process) to the debugfs entries
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_add_instance(instance: *mut vchiq_instance) {
    void vchiq_debugfs_add_instance(struct vchiq_instance *instance)
    {
    char pidstr[16];
    struct dentry *top;
    snprintf(pidstr, sizeof(pidstr), "%d",
    vchiq_instance_get_pid(instance));
    top = debugfs_create_dir(pidstr, vchiq_dbg_clients);
    debugfs_create_file("use_count", 0444, top, instance,
    &debugfs_usecount_fops);
    debugfs_create_file("trace", 0644, top, instance, &debugfs_trace_fops);
    vchiq_instance_get_debugfs_node(instance).dentry = top;
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_remove_instance(instance: *mut vchiq_instance) {
    void vchiq_debugfs_remove_instance(struct vchiq_instance *instance)
    {
    struct vchiq_debugfs_node *node =
    vchiq_instance_get_debugfs_node(instance);
    debugfs_remove_recursive(node.dentry);
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_init(state: *mut vchiq_state) {
    void vchiq_debugfs_init(struct vchiq_state *state)
    {
    vchiq_dbg_dir = debugfs_create_dir("vchiq", core::ptr::null_mut());
    vchiq_dbg_clients = debugfs_create_dir("clients", vchiq_dbg_dir);
    debugfs_create_file("state", S_IFREG | 0444, vchiq_dbg_dir, state,
    &vchiq_dump_fops);
    }
// remove all the debugfs entries
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_deinit() {
    void vchiq_debugfs_deinit(void)
    {
    debugfs_remove_recursive(vchiq_dbg_dir);
    }

#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_init(state: *mut vchiq_state) {
    void vchiq_debugfs_init(struct vchiq_state *state)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_deinit() {
    void vchiq_debugfs_deinit(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_add_instance(instance: *mut vchiq_instance) {
    void vchiq_debugfs_add_instance(struct vchiq_instance *instance)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_debugfs_remove_instance(instance: *mut vchiq_instance) {
    void vchiq_debugfs_remove_instance(struct vchiq_instance *instance)
    {
    }
