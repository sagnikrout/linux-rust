//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/megaraid/megaraid_sas_debugfs.c
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


//
// Linux MegaRAID driver for SAS based RAID controllers
//
// Copyright (c) 2003-2018  LSI Corporation.
// Copyright (c) 2003-2018  Avago Technologies.
// Copyright (c) 2003-2018  Broadcom Inc.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// Authors: Broadcom Inc.
// Kashyap Desai <kashyap.desai@broadcom.com>
// Sumit Saxena <sumit.saxena@broadcom.com>
// Shivasharan S <shivasharan.srikanteshwara@broadcom.com>
//
// Send feedback to: megaraidlinux.pdl@broadcom.com
//

    struct dentry *megasas_debugfs_root;
    static ssize_t
    megasas_debugfs_read(struct file *filp, char __user *ubuf, size_t cnt,
    loff_t *ppos)
    {
    struct megasas_debugfs_buffer *debug = filp.private_data;
    if (!debug || !debug.buf)
    return 0;
    return simple_read_from_buffer(ubuf, cnt, ppos, debug.buf, debug.len);
    }
    static int
    megasas_debugfs_raidmap_open(struct inode *inode, struct file *file)
    {
    struct megasas_instance *instance = inode.i_private;
    struct megasas_debugfs_buffer *debug;
    struct fusion_context *fusion;
    fusion = instance.ctrl_context;
    debug = kzalloc_obj(struct megasas_debugfs_buffer);
    if (!debug)
    return -ENOMEM;
    debug.buf = (void *)fusion.ld_drv_map[(instance.map_id & 1)];
    debug.len = fusion.drv_map_sz;
    file.private_data = debug;
    return 0;
    }
    static int
    megasas_debugfs_release(struct inode *inode, struct file *file)
    {
    struct megasas_debug_buffer *debug = file.private_data;
    if (!debug)
    return 0;
    file.private_data = core::ptr::null_mut();
    kfree(debug);
    return 0;
    }
    static const struct file_operations megasas_debugfs_raidmap_fops = {
    .owner		= THIS_MODULE,
    .open           = megasas_debugfs_raidmap_open,
    .read           = megasas_debugfs_read,
    .release        = megasas_debugfs_release,
    };
//
// megasas_init_debugfs :	Create debugfs root for megaraid_sas driver
//
#[no_mangle]
pub unsafe extern "C" fn megasas_init_debugfs() {
    void megasas_init_debugfs(void)
    {
    megasas_debugfs_root = debugfs_create_dir("megaraid_sas", core::ptr::null_mut());
    if (!megasas_debugfs_root)
    pr_info("Cannot create debugfs root\n");
    }
//
// megasas_exit_debugfs :	Remove debugfs root for megaraid_sas driver
//
#[no_mangle]
pub unsafe extern "C" fn megasas_exit_debugfs() {
    void megasas_exit_debugfs(void)
    {
    debugfs_remove_recursive(megasas_debugfs_root);
    }
//
// megasas_setup_debugfs :	Setup debugfs per Fusion adapter
// instance:				Soft instance of adapter
//
    void
    megasas_setup_debugfs(struct megasas_instance *instance)
    {
    char name[64];
    struct fusion_context *fusion;
    fusion = instance.ctrl_context;
    if (fusion) {
    snprintf(name, sizeof(name),
    "scsi_host%d", instance.host.host_no);
    if (!instance.debugfs_root) {
    instance.debugfs_root =
    debugfs_create_dir(name, megasas_debugfs_root);
    if (!instance.debugfs_root) {
    dev_err(&instance.pdev.dev,
    "Cannot create per adapter debugfs directory\n");
    return;
    }
    }
    snprintf(name, sizeof(name), "raidmap_dump");
    instance.raidmap_dump =
    debugfs_create_file(name, S_IRUGO,
    instance.debugfs_root, instance,
    &megasas_debugfs_raidmap_fops);
    if (!instance.raidmap_dump) {
    dev_err(&instance.pdev.dev,
    "Cannot create raidmap debugfs file\n");
    debugfs_remove(instance.debugfs_root);
    return;
    }
    }
    }
//
// megasas_destroy_debugfs :	Destroy debugfs per Fusion adapter
// instance:					Soft instance of adapter
//
#[no_mangle]
pub unsafe extern "C" fn megasas_destroy_debugfs(instance: *mut megasas_instance) {
    void megasas_destroy_debugfs(struct megasas_instance *instance)
    {
    debugfs_remove_recursive(instance.debugfs_root);
    }

#[no_mangle]
pub unsafe extern "C" fn megasas_init_debugfs() {
    void megasas_init_debugfs(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn megasas_exit_debugfs() {
    void megasas_exit_debugfs(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn megasas_setup_debugfs(instance: *mut megasas_instance) {
    void megasas_setup_debugfs(struct megasas_instance *instance)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn megasas_destroy_debugfs(instance: *mut megasas_instance) {
    void megasas_destroy_debugfs(struct megasas_instance *instance)
    {
    }
