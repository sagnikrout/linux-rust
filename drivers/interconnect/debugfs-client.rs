//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/debugfs-client.c
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
// Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// This can be dangerous, therefore don't provide any real compile time
// configuration option for this feature.
// People who want to use this will need to modify the source code directly.
//

    static LIST_HEAD(debugfs_paths);
    static DEFINE_MUTEX(debugfs_lock);
    static struct platform_device *pdev;
    static struct icc_path *cur_path;
    static char *src_node;
    static char *dst_node;
    static u32 avg_bw;
    static u32 peak_bw;
    static u32 tag;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_path {
    pub src: *const c_char,
    pub dst: *const c_char,
    pub path: *mut icc_path,
    pub list: list_head,
}

    static struct icc_path *get_path(const char *src, const char *dst)
    {
    struct debugfs_path *path;
    list_for_each_entry(path, &debugfs_paths, list) {
    if (!strcmp(path.src, src) && !strcmp(path.dst, dst))
    return path.path;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn icc_get_set(data: *mut c_void, val: u64) -> c_int {
    static int icc_get_set(void *data, u64 val)
    {
    struct debugfs_path *debugfs_path;
    char *src, *dst;
    let mut ret: c_int = 0;
    mutex_lock(&debugfs_lock);
    rcu_read_lock();
    src = rcu_dereference(src_node);
    dst = rcu_dereference(dst_node);
//
// If we've already looked up a path, then use the existing one instead
// of calling icc_get() again. This allows for updating previous BW
// votes when "get" is written to multiple times for multiple paths.
//
    cur_path = get_path(src, dst);
    if (cur_path) {
    rcu_read_unlock();
    goto out;
    }
    src = kstrdup(src, GFP_ATOMIC);
    dst = kstrdup(dst, GFP_ATOMIC);
    rcu_read_unlock();
    if (!src || !dst) {
    ret = -ENOMEM;
    goto err_free;
    }
    cur_path = icc_get(&pdev.dev, src, dst);
    if (IS_ERR(cur_path)) {
    ret = PTR_ERR(cur_path);
    goto err_free;
    }
    debugfs_path = kzalloc_obj(*debugfs_path);
    if (!debugfs_path) {
    ret = -ENOMEM;
    goto err_put;
    }
    debugfs_path.path = cur_path;
    debugfs_path.src = src;
    debugfs_path.dst = dst;
    list_add_tail(&debugfs_path.list, &debugfs_paths);
    goto out;
    err_put:
    icc_put(cur_path);
    err_free:
    kfree(src);
    kfree(dst);
    out:
    mutex_unlock(&debugfs_lock);
    return ret;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(icc_get_fops, core::ptr::null_mut(), icc_get_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn icc_commit_set(data: *mut c_void, val: u64) -> c_int {
    static int icc_commit_set(void *data, u64 val)
    {
    int ret;
    mutex_lock(&debugfs_lock);
    if (!cur_path) {
    ret = -EINVAL;
    goto out;
    }
    if (IS_ERR(cur_path)) {
    ret = PTR_ERR(cur_path);
    goto out;
    }
    icc_set_tag(cur_path, tag);
    ret = icc_set_bw(cur_path, avg_bw, peak_bw);
    out:
    mutex_unlock(&debugfs_lock);
    return ret;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(icc_commit_fops, core::ptr::null_mut(), icc_commit_set, "%llu\n");
#[no_mangle]
pub unsafe extern "C" fn icc_debugfs_client_init(icc_dir: *mut dentry) -> c_int {
    int icc_debugfs_client_init(struct dentry *icc_dir)
    {
    struct dentry *client_dir;
    int ret;
    pdev = platform_device_alloc("icc-debugfs-client", PLATFORM_DEVID_NONE);
    if (!pdev)
    return -ENOMEM;
    ret = platform_device_add(pdev);
    if (ret) {
    pr_err("%s: failed to add platform device: %d\n", __func__, ret);
    platform_device_put(pdev);
    return ret;
    }
    src_node = kstrdup("", GFP_KERNEL);
    dst_node = kstrdup("", GFP_KERNEL);
    if (!src_node || !dst_node) {
    kfree(dst_node);
    kfree(src_node);
    return -ENOMEM;
    }
    client_dir = debugfs_create_dir("test_client", icc_dir);
    debugfs_create_str("src_node", 0600, client_dir, &src_node);
    debugfs_create_str("dst_node", 0600, client_dir, &dst_node);
    debugfs_create_file("get", 0200, client_dir, core::ptr::null_mut(), &icc_get_fops);
    debugfs_create_u32("avg_bw", 0600, client_dir, &avg_bw);
    debugfs_create_u32("peak_bw", 0600, client_dir, &peak_bw);
    debugfs_create_u32("tag", 0600, client_dir, &tag);
    debugfs_create_file("commit", 0200, client_dir, core::ptr::null_mut(), &icc_commit_fops);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn icc_debugfs_client_init(icc_dir: *mut dentry) -> c_int {
    int icc_debugfs_client_init(struct dentry *icc_dir)
    {
    return 0;
    }
