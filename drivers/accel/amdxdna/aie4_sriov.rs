//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie4_sriov.c
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

#[no_mangle]
unsafe extern "C" fn aie4_destroy_vfs(ndev: *mut amdxdna_dev_hdl) -> c_int {
    static int aie4_destroy_vfs(struct amdxdna_dev_hdl *ndev)
    {
    DECLARE_AIE_MSG(aie4_msg_destroy_vfs, AIE4_MSG_OP_DESTROY_VFS);
    int ret;
    ret = aie_send_mgmt_msg_wait(&ndev.aie, &msg);
    if (ret)
    XDNA_ERR(ndev.aie.xdna, "destroy vfs op failed: %d", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aie4_create_vfs(ndev: *mut amdxdna_dev_hdl, num_vfs: c_int) -> c_int {
    static int aie4_create_vfs(struct amdxdna_dev_hdl *ndev, int num_vfs)
    {
    DECLARE_AIE_MSG(aie4_msg_create_vfs, AIE4_MSG_OP_CREATE_VFS);
    int ret;
    req.vf_cnt = num_vfs;
    ret = aie_send_mgmt_msg_wait(&ndev.aie, &msg);
    if (ret)
    XDNA_ERR(ndev.aie.xdna, "create vfs op failed: %d", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aie4_sriov_stop(ndev: *mut amdxdna_dev_hdl) -> c_int {
    int aie4_sriov_stop(struct amdxdna_dev_hdl *ndev)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    struct pci_dev *pdev = to_pci_dev(xdna.ddev.dev);
    int ret;
    if (!pci_num_vf(pdev))
    return 0;
    ret = pci_vfs_assigned(pdev);
    if (ret) {
    XDNA_ERR(xdna, "VFs are still assigned to VMs");
    return -EPERM;
    }
    pci_disable_sriov(pdev);
    return aie4_destroy_vfs(ndev);
    }
#[no_mangle]
unsafe extern "C" fn aie4_sriov_start(ndev: *mut amdxdna_dev_hdl, num_vfs: c_int) -> c_int {
    static int aie4_sriov_start(struct amdxdna_dev_hdl *ndev, int num_vfs)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    struct pci_dev *pdev = to_pci_dev(xdna.ddev.dev);
    int ret;
    ret = aie4_create_vfs(ndev, num_vfs);
    if (ret)
    return ret;
    ret = pci_enable_sriov(pdev, num_vfs);
    if (ret) {
    XDNA_ERR(xdna, "configure VFs failed, ret: %d", ret);
    aie4_destroy_vfs(ndev);
    return ret;
    }
    return num_vfs;
    }
#[no_mangle]
pub unsafe extern "C" fn aie4_sriov_configure(xdna: *mut amdxdna_dev, num_vfs: c_int) -> c_int {
    int aie4_sriov_configure(struct amdxdna_dev *xdna, int num_vfs)
    {
    struct amdxdna_dev_hdl *ndev = xdna.dev_handle;
    drm_WARN_ON(&xdna.ddev, !mutex_is_locked(&xdna.dev_lock));
    return (num_vfs) ? aie4_sriov_start(ndev, num_vfs) : aie4_sriov_stop(ndev);
    }
