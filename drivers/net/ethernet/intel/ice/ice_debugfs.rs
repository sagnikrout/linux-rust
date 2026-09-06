//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/ice/ice_debugfs.c
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
// Copyright (c) 2022, Intel Corporation.

    static struct dentry *ice_debugfs_root;
#[no_mangle]
pub unsafe extern "C" fn ice_debugfs_pf_init(pf: *mut ice_pf) -> c_int {
    int ice_debugfs_pf_init(struct ice_pf *pf)
    {
    const char *name = pci_name(pf.pdev);
    pf.ice_debugfs_pf = debugfs_create_dir(name, ice_debugfs_root);
    if (IS_ERR(pf.ice_debugfs_pf))
    return PTR_ERR(pf.ice_debugfs_pf);
    return 0;
    }
//
// ice_debugfs_pf_deinit - cleanup PF's debugfs
// @pf: pointer to the PF struct
//
#[no_mangle]
pub unsafe extern "C" fn ice_debugfs_pf_deinit(pf: *mut ice_pf) {
    void ice_debugfs_pf_deinit(struct ice_pf *pf)
    {
    debugfs_remove_recursive(pf.ice_debugfs_pf);
    pf.ice_debugfs_pf = core::ptr::null_mut();
    }
//
// ice_debugfs_init - create root directory for debugfs entries
//
#[no_mangle]
pub unsafe extern "C" fn ice_debugfs_init() {
    void ice_debugfs_init(void)
    {
    ice_debugfs_root = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    if (IS_ERR(ice_debugfs_root))
    pr_info("init of debugfs failed\n");
    }
//
// ice_debugfs_exit - remove debugfs entries
//
#[no_mangle]
pub unsafe extern "C" fn ice_debugfs_exit() {
    void ice_debugfs_exit(void)
    {
    debugfs_remove_recursive(ice_debugfs_root);
    ice_debugfs_root = core::ptr::null_mut();
    }
