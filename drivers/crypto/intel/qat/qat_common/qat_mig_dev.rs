//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/qat_mig_dev.c
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
// Copyright(c) 2024 Intel Corporation

    struct qat_mig_dev *qat_vfmig_create(struct pci_dev *pdev, int vf_id)
    {
    struct adf_accel_dev *accel_dev;
    struct qat_migdev_ops *ops;
    struct qat_mig_dev *mdev;
    accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if (!accel_dev)
    return ERR_PTR(-ENODEV);
    ops = GET_VFMIG_OPS(accel_dev);
    if (!ops || !ops.init || !ops.cleanup || !ops.reset || !ops.open ||
    !ops.close || !ops.suspend || !ops.resume || !ops.save_state ||
    !ops.load_state || !ops.save_setup || !ops.load_setup)
    return ERR_PTR(-EINVAL);
    mdev = kmalloc_obj(*mdev);
    if (!mdev)
    return ERR_PTR(-ENOMEM);
    mdev.vf_id = vf_id;
    mdev.parent_accel_dev = accel_dev;
    return mdev;
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_create);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_init(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_init(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).init(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_init);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_cleanup(mdev: *mut qat_mig_dev) {
    void qat_vfmig_cleanup(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).cleanup(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_cleanup);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_reset(mdev: *mut qat_mig_dev) {
    void qat_vfmig_reset(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).reset(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_reset);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_open(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_open(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).open(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_open);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_close(mdev: *mut qat_mig_dev) {
    void qat_vfmig_close(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    GET_VFMIG_OPS(accel_dev).close(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_close);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_suspend(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_suspend(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).suspend(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_suspend);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_resume(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_resume(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).resume(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_resume);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_save_state(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_save_state(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).save_state(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_save_state);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_save_setup(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_save_setup(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).save_setup(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_save_setup);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_load_state(mdev: *mut qat_mig_dev) -> c_int {
    int qat_vfmig_load_state(struct qat_mig_dev *mdev)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).load_state(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_load_state);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_load_setup(mdev: *mut qat_mig_dev, size: c_int) -> c_int {
    int qat_vfmig_load_setup(struct qat_mig_dev *mdev, int size)
    {
    struct adf_accel_dev *accel_dev = mdev.parent_accel_dev;
    return GET_VFMIG_OPS(accel_dev).load_setup(mdev, size);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_load_setup);
#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_destroy(mdev: *mut qat_mig_dev) {
    void qat_vfmig_destroy(struct qat_mig_dev *mdev)
    {
    kfree(mdev);
    }
    EXPORT_SYMBOL_GPL(qat_vfmig_destroy);
