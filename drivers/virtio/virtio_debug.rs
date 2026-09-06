//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_debug.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    static struct dentry *virtio_debugfs_dir;
#[no_mangle]
unsafe extern "C" fn virtio_debug_device_features_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int virtio_debug_device_features_show(struct seq_file *s, void *data)
    {
    u64 device_features[VIRTIO_FEATURES_U64S];
    struct virtio_device *dev = s.private;
    unsigned int i;
    virtio_get_features(dev, device_features);
    for (i = 0; i < VIRTIO_FEATURES_BITS; i++) {
    if (virtio_features_test_bit(device_features, i))
    seq_printf(s, "%u\n", i);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(virtio_debug_device_features);
#[no_mangle]
unsafe extern "C" fn virtio_debug_filter_features_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int virtio_debug_filter_features_show(struct seq_file *s, void *data)
    {
    struct virtio_device *dev = s.private;
    unsigned int i;
    for (i = 0; i < VIRTIO_FEATURES_BITS; i++) {
    if (virtio_features_test_bit(dev.debugfs_filter_features, i))
    seq_printf(s, "%u\n", i);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(virtio_debug_filter_features);
#[no_mangle]
unsafe extern "C" fn virtio_debug_filter_features_clear(data: *mut c_void, val: u64) -> c_int {
    static int virtio_debug_filter_features_clear(void *data, u64 val)
    {
    struct virtio_device *dev = data;
    if (val == 1)
    virtio_features_zero(dev.debugfs_filter_features);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(virtio_debug_filter_features_clear_fops, core::ptr::null_mut(),
    virtio_debug_filter_features_clear, "%llu\n");
#[no_mangle]
unsafe extern "C" fn virtio_debug_filter_feature_add(data: *mut c_void, val: u64) -> c_int {
    static int virtio_debug_filter_feature_add(void *data, u64 val)
    {
    struct virtio_device *dev = data;
    if (val >= VIRTIO_FEATURES_BITS)
    return -EINVAL;
    virtio_features_set_bit(dev.debugfs_filter_features, val);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(virtio_debug_filter_feature_add_fops, core::ptr::null_mut(),
    virtio_debug_filter_feature_add, "%llu\n");
#[no_mangle]
unsafe extern "C" fn virtio_debug_filter_feature_del(data: *mut c_void, val: u64) -> c_int {
    static int virtio_debug_filter_feature_del(void *data, u64 val)
    {
    struct virtio_device *dev = data;
    if (val >= VIRTIO_FEATURES_BITS)
    return -EINVAL;
    virtio_features_clear_bit(dev.debugfs_filter_features, val);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(virtio_debug_filter_feature_del_fops, core::ptr::null_mut(),
    virtio_debug_filter_feature_del, "%llu\n");
#[no_mangle]
pub unsafe extern "C" fn virtio_debug_device_init(dev: *mut virtio_device) {
    void virtio_debug_device_init(struct virtio_device *dev)
    {
    dev.debugfs_dir = debugfs_create_dir(dev_name(&dev.dev),
    virtio_debugfs_dir);
    debugfs_create_file("device_features", 0400, dev.debugfs_dir, dev,
    &virtio_debug_device_features_fops);
    debugfs_create_file("filter_features", 0400, dev.debugfs_dir, dev,
    &virtio_debug_filter_features_fops);
    debugfs_create_file("filter_features_clear", 0200, dev.debugfs_dir, dev,
    &virtio_debug_filter_features_clear_fops);
    debugfs_create_file("filter_feature_add", 0200, dev.debugfs_dir, dev,
    &virtio_debug_filter_feature_add_fops);
    debugfs_create_file("filter_feature_del", 0200, dev.debugfs_dir, dev,
    &virtio_debug_filter_feature_del_fops);
    }
    EXPORT_SYMBOL_GPL(virtio_debug_device_init);
#[no_mangle]
pub unsafe extern "C" fn virtio_debug_device_filter_features(dev: *mut virtio_device) {
    void virtio_debug_device_filter_features(struct virtio_device *dev)
    {
    virtio_features_andnot(dev.features_array, dev.features_array,
    dev.debugfs_filter_features);
    }
    EXPORT_SYMBOL_GPL(virtio_debug_device_filter_features);
#[no_mangle]
pub unsafe extern "C" fn virtio_debug_device_exit(dev: *mut virtio_device) {
    void virtio_debug_device_exit(struct virtio_device *dev)
    {
    debugfs_remove_recursive(dev.debugfs_dir);
    }
    EXPORT_SYMBOL_GPL(virtio_debug_device_exit);
#[no_mangle]
pub unsafe extern "C" fn virtio_debug_init() {
    void virtio_debug_init(void)
    {
    virtio_debugfs_dir = debugfs_create_dir("virtio", core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(virtio_debug_init);
#[no_mangle]
pub unsafe extern "C" fn virtio_debug_exit() {
    void virtio_debug_exit(void)
    {
    debugfs_remove_recursive(virtio_debugfs_dir);
    }
    EXPORT_SYMBOL_GPL(virtio_debug_exit);
