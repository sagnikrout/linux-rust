//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/debugfs.c
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
// Copyright (c) 2023, HiSilicon Ltd.
//

    static struct dentry *vfio_debugfs_root;
#[no_mangle]
unsafe extern "C" fn vfio_device_state_read(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int vfio_device_state_read(struct seq_file *seq, void *data)
    {
    struct device *vf_dev = seq.private;
    struct vfio_device *vdev = container_of(vf_dev,
    struct vfio_device, device);
    enum vfio_device_mig_state state;
    int ret;
    BUILD_BUG_ON(VFIO_DEVICE_STATE_NR !=
    VFIO_DEVICE_STATE_PRE_COPY_P2P + 1);
    ret = vdev.mig_ops.migration_get_state(vdev, &state);
    if (ret)
    return -EINVAL;
    switch (state) {
    case VFIO_DEVICE_STATE_ERROR:
    seq_puts(seq, "ERROR\n");
    break;
    case VFIO_DEVICE_STATE_STOP:
    seq_puts(seq, "STOP\n");
    break;
    case VFIO_DEVICE_STATE_RUNNING:
    seq_puts(seq, "RUNNING\n");
    break;
    case VFIO_DEVICE_STATE_STOP_COPY:
    seq_puts(seq, "STOP_COPY\n");
    break;
    case VFIO_DEVICE_STATE_RESUMING:
    seq_puts(seq, "RESUMING\n");
    break;
    case VFIO_DEVICE_STATE_RUNNING_P2P:
    seq_puts(seq, "RUNNING_P2P\n");
    break;
    case VFIO_DEVICE_STATE_PRE_COPY:
    seq_puts(seq, "PRE_COPY\n");
    break;
    case VFIO_DEVICE_STATE_PRE_COPY_P2P:
    seq_puts(seq, "PRE_COPY_P2P\n");
    break;
    default:
    seq_puts(seq, "Invalid\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfio_device_features_read(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int vfio_device_features_read(struct seq_file *seq, void *data)
    {
    struct device *vf_dev = seq.private;
    struct vfio_device *vdev = container_of(vf_dev, struct vfio_device, device);
    if (vdev.migration_flags & VFIO_MIGRATION_STOP_COPY)
    seq_puts(seq, "stop-copy\n");
    if (vdev.migration_flags & VFIO_MIGRATION_P2P)
    seq_puts(seq, "p2p\n");
    if (vdev.migration_flags & VFIO_MIGRATION_PRE_COPY)
    seq_puts(seq, "pre-copy\n");
    if (vdev.log_ops)
    seq_puts(seq, "dirty-tracking\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_device_debugfs_init(vdev: *mut vfio_device) {
    void vfio_device_debugfs_init(struct vfio_device *vdev)
    {
    struct device *dev = &vdev.device;
    vdev.debug_root = debugfs_create_dir(dev_name(vdev.dev),
    vfio_debugfs_root);
    if (vdev.mig_ops) {
    struct dentry *vfio_dev_migration = core::ptr::null_mut();
    vfio_dev_migration = debugfs_create_dir("migration",
    vdev.debug_root);
    debugfs_create_devm_seqfile(dev, "state", vfio_dev_migration,
    vfio_device_state_read);
    debugfs_create_devm_seqfile(dev, "features", vfio_dev_migration,
    vfio_device_features_read);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_device_debugfs_exit(vdev: *mut vfio_device) {
    void vfio_device_debugfs_exit(struct vfio_device *vdev)
    {
    debugfs_remove_recursive(vdev.debug_root);
    vdev.debug_root = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_debugfs_create_root() {
    void vfio_debugfs_create_root(void)
    {
    vfio_debugfs_root = debugfs_create_dir("vfio", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_debugfs_remove_root() {
    void vfio_debugfs_remove_root(void)
    {
    debugfs_remove_recursive(vfio_debugfs_root);
    vfio_debugfs_root = core::ptr::null_mut();
    }
