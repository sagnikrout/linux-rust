//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/bulk.c
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


// SPDX-License-Identifier: GPL-2.0

//
// of_icc_bulk_get() - get interconnect paths
// @dev: the device requesting the path
// @num_paths: the number of icc_bulk_data
// @paths: the table with the paths we want to get
//
// Returns 0 on success or negative errno otherwise.
//
    int __must_check of_icc_bulk_get(struct device *dev, int num_paths,
    struct icc_bulk_data *paths)
    {
    int ret, i;
    for (i = 0; i < num_paths; i++) {
    paths[i].path = of_icc_get(dev, paths[i].name);
    if (IS_ERR(paths[i].path)) {
    ret = PTR_ERR(paths[i].path);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "of_icc_get() failed on path %s (%d)\n",
    paths[i].name, ret);
    paths[i].path = core::ptr::null_mut();
    goto err;
    }
    }
    return 0;
    err:
    icc_bulk_put(i, paths);
    return ret;
    }
    EXPORT_SYMBOL_GPL(of_icc_bulk_get);
//
// icc_bulk_put() - put a list of interconnect paths
// @num_paths: the number of icc_bulk_data
// @paths: the icc_bulk_data table with the paths being put
//
#[no_mangle]
pub unsafe extern "C" fn icc_bulk_put(num_paths: c_int, paths: *mut icc_bulk_data) {
    void icc_bulk_put(int num_paths, struct icc_bulk_data *paths)
    {
    while (--num_paths >= 0) {
    icc_put(paths[num_paths].path);
    paths[num_paths].path = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(icc_bulk_put);
//
// icc_bulk_set_bw() - set bandwidth to a set of paths
// @num_paths: the number of icc_bulk_data
// @paths: the icc_bulk_data table containing the paths and bandwidth
//
// Returns 0 on success or negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn icc_bulk_set_bw(num_paths: c_int, paths: *const icc_bulk_data) -> c_int {
    int icc_bulk_set_bw(int num_paths, const struct icc_bulk_data *paths)
    {
    let mut ret: c_int = 0;
    int i;
    for (i = 0; i < num_paths; i++) {
    ret = icc_set_bw(paths[i].path, paths[i].avg_bw, paths[i].peak_bw);
    if (ret) {
    pr_err("icc_set_bw() failed on path %s (%d)\n", paths[i].name, ret);
    return ret;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(icc_bulk_set_bw);
//
// icc_bulk_enable() - enable a previously disabled set of paths
// @num_paths: the number of icc_bulk_data
// @paths: the icc_bulk_data table containing the paths and bandwidth
//
// Returns 0 on success or negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn icc_bulk_enable(num_paths: c_int, paths: *const icc_bulk_data) -> c_int {
    int icc_bulk_enable(int num_paths, const struct icc_bulk_data *paths)
    {
    int ret, i;
    for (i = 0; i < num_paths; i++) {
    ret = icc_enable(paths[i].path);
    if (ret) {
    pr_err("icc_enable() failed on path %s (%d)\n", paths[i].name, ret);
    goto err;
    }
    }
    return 0;
    err:
    icc_bulk_disable(i, paths);
    return ret;
    }
    EXPORT_SYMBOL_GPL(icc_bulk_enable);
//
// icc_bulk_disable() - disable a set of interconnect paths
// @num_paths: the number of icc_bulk_data
// @paths: the icc_bulk_data table containing the paths and bandwidth
//
#[no_mangle]
pub unsafe extern "C" fn icc_bulk_disable(num_paths: c_int, paths: *const icc_bulk_data) {
    void icc_bulk_disable(int num_paths, const struct icc_bulk_data *paths)
    {
    while (--num_paths >= 0)
    icc_disable(paths[num_paths].path);
    }
    EXPORT_SYMBOL_GPL(icc_bulk_disable);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_bulk_devres {
    pub paths: *mut icc_bulk_data,
    pub num_paths: c_int,
}

#[no_mangle]
unsafe extern "C" fn devm_icc_bulk_release(dev: *mut device, res: *mut c_void) {
    static void devm_icc_bulk_release(struct device *dev, void *res)
    {
    struct icc_bulk_devres *devres = res;
    icc_bulk_put(devres.num_paths, devres.paths);
    }
//
// devm_of_icc_bulk_get() - resource managed of_icc_bulk_get
// @dev: the device requesting the path
// @num_paths: the number of icc_bulk_data
// @paths: the table with the paths we want to get
//
// Returns 0 on success or negative errno otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn devm_of_icc_bulk_get(dev: *mut device, num_paths: c_int, paths: *mut icc_bulk_data) -> c_int {
    int devm_of_icc_bulk_get(struct device *dev, int num_paths, struct icc_bulk_data *paths)
    {
    struct icc_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_icc_bulk_release, sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    ret = of_icc_bulk_get(dev, num_paths, paths);
    if (!ret) {
    devres.paths = paths;
    devres.num_paths = num_paths;
    devres_add(dev, devres);
    } else {
    devres_free(devres);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_of_icc_bulk_get);
