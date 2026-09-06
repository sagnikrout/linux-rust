//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/ucaps.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved
//

    static DEFINE_MUTEX(ucaps_mutex);
    static struct ib_ucap *ucaps_list[RDMA_UCAP_MAX];
    static bool ucaps_class_is_registered;
    static dev_t ucaps_base_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ucap {
    pub cdev: cdev,
    pub dev: device,
    pub ref: kref,
}

    static const char *ucap_names[RDMA_UCAP_MAX] = {
    [RDMA_UCAP_MLX5_CTRL_LOCAL] = "mlx5_perm_ctrl_local",
    [RDMA_UCAP_MLX5_CTRL_OTHER_VHCA] = "mlx5_perm_ctrl_other_vhca"
    };
    static char *ucaps_devnode(const struct device *dev, umode_t *mode)
    {
    if (mode)
// mode = 0600;
    return kasprintf(GFP_KERNEL, "infiniband/%s", dev_name(dev));
    }
    static const struct class ucaps_class = {
    .name = "infiniband_ucaps",
    .devnode = ucaps_devnode,
    };
    static const struct file_operations ucaps_cdev_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    };
#[no_mangle]
unsafe extern "C" fn ib_cleanup_ucaps() -> __exit void {
    static __exit void ib_cleanup_ucaps(void)
    {
    mutex_lock(&ucaps_mutex);
    if (!ucaps_class_is_registered) {
    mutex_unlock(&ucaps_mutex);
    return;
    }
    for (int i = RDMA_UCAP_FIRST; i < RDMA_UCAP_MAX; i++)
    WARN_ON(ucaps_list[i]);
    class_unregister(&ucaps_class);
    ucaps_class_is_registered = false;
    unregister_chrdev_region(ucaps_base_dev, RDMA_UCAP_MAX);
    mutex_unlock(&ucaps_mutex);
    }
#[no_mangle]
unsafe extern "C" fn get_ucap_from_devt(devt: dev_t, idx_mask: *mut u64) -> c_int {
    static int get_ucap_from_devt(dev_t devt, u64 *idx_mask)
    {
    for (int type = RDMA_UCAP_FIRST; type < RDMA_UCAP_MAX; type++) {
    if (ucaps_list[type] && ucaps_list[type].dev.devt == devt) {
// idx_mask |= 1 << type;
    return 0;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn get_devt_from_fd(fd: c_uint, ret_dev: *mut dev_t) -> c_int {
    static int get_devt_from_fd(unsigned int fd, dev_t *ret_dev)
    {
    CLASS(fd, f)(fd);
    if (fd_empty(f) || fd_file(f).f_op != &ucaps_cdev_fops)
    return -EBADF;
// ret_dev = file_inode(fd_file(f))->i_rdev;
    return 0;
    }
//
// ib_ucaps_init - Initialization required before ucap creation.
//
// Return: 0 on success, or a negative errno value on failure
//
#[no_mangle]
unsafe extern "C" fn ib_ucaps_init() -> c_int {
    static int ib_ucaps_init(void)
    {
    let mut ret: c_int = 0;
    if (ucaps_class_is_registered)
    return ret;
    ret = class_register(&ucaps_class);
    if (ret)
    return ret;
    ret = alloc_chrdev_region(&ucaps_base_dev, 0, RDMA_UCAP_MAX,
    ucaps_class.name);
    if (ret < 0) {
    class_unregister(&ucaps_class);
    return ret;
    }
    ucaps_class_is_registered = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucap_dev_release(device: *mut device) {
    static void ucap_dev_release(struct device *device)
    {
    struct ib_ucap *ucap = container_of(device, struct ib_ucap, dev);
    kfree(ucap);
    }
//
// ib_create_ucap - Add a ucap character device
// @type: UCAP type
//
// Creates a ucap character device in the /dev/infiniband directory. By default,
// the device has root-only read-write access.
//
// A driver may call this multiple times with the same UCAP type. A reference
// count tracks creations and deletions.
//
// Return: 0 on success, or a negative errno value on failure
//
#[no_mangle]
pub unsafe extern "C" fn ib_create_ucap(type: enum rdma_user_cap) -> c_int {
    int ib_create_ucap(enum rdma_user_cap type)
    {
    struct ib_ucap *ucap;
    int ret;
    if (type >= RDMA_UCAP_MAX)
    return -EINVAL;
    mutex_lock(&ucaps_mutex);
    ret = ib_ucaps_init();
    if (ret)
    goto unlock;
    ucap = ucaps_list[type];
    if (ucap) {
    kref_get(&ucap.ref);
    mutex_unlock(&ucaps_mutex);
    return 0;
    }
    ucap = kzalloc_obj(*ucap);
    if (!ucap) {
    ret = -ENOMEM;
    goto unlock;
    }
    device_initialize(&ucap.dev);
    ucap.dev.class = &ucaps_class;
    ucap.dev.devt = MKDEV(MAJOR(ucaps_base_dev), type);
    ucap.dev.release = ucap_dev_release;
    ret = dev_set_name(&ucap.dev, "%s", ucap_names[type]);
    if (ret)
    goto err_device;
    cdev_init(&ucap.cdev, &ucaps_cdev_fops);
    ucap.cdev.owner = THIS_MODULE;
    ret = cdev_device_add(&ucap.cdev, &ucap.dev);
    if (ret)
    goto err_device;
    kref_init(&ucap.ref);
    ucaps_list[type] = ucap;
    mutex_unlock(&ucaps_mutex);
    return 0;
    err_device:
    put_device(&ucap.dev);
    unlock:
    mutex_unlock(&ucaps_mutex);
    return ret;
    }
    EXPORT_SYMBOL(ib_create_ucap);
#[no_mangle]
unsafe extern "C" fn ib_release_ucap(ref: *mut kref) {
    static void ib_release_ucap(struct kref *ref)
    {
    struct ib_ucap *ucap = container_of(ref, struct ib_ucap, ref);
    enum rdma_user_cap type;
    for (type = RDMA_UCAP_FIRST; type < RDMA_UCAP_MAX; type++) {
    if (ucaps_list[type] == ucap)
    break;
    }
    WARN_ON(type == RDMA_UCAP_MAX);
    ucaps_list[type] = core::ptr::null_mut();
    cdev_device_del(&ucap.cdev, &ucap.dev);
    put_device(&ucap.dev);
    }
//
// ib_remove_ucap - Remove a ucap character device
// @type: User cap type
//
// Removes the ucap character device according to type. The device is completely
// removed from the filesystem when its reference count reaches 0.
//
#[no_mangle]
pub unsafe extern "C" fn ib_remove_ucap(type: enum rdma_user_cap) {
    void ib_remove_ucap(enum rdma_user_cap type)
    {
    struct ib_ucap *ucap;
    mutex_lock(&ucaps_mutex);
    ucap = ucaps_list[type];
    if (WARN_ON(!ucap))
    goto end;
    kref_put(&ucap.ref, ib_release_ucap);
    end:
    mutex_unlock(&ucaps_mutex);
    }
    EXPORT_SYMBOL(ib_remove_ucap);
//
// ib_get_ucaps - Get bitmask of ucap types from file descriptors
// @fds: Array of file descriptors
// @fd_count: Number of file descriptors in the array
// @idx_mask: Bitmask to be updated based on the ucaps in the fd list
//
// Given an array of file descriptors, this function returns a bitmask of
// the ucaps where a bit is set if an FD for that ucap type was in the array.
//
// Return: 0 on success, or a negative errno value on failure
//
#[no_mangle]
pub unsafe extern "C" fn ib_get_ucaps(fds: *mut c_int, fd_count: c_int, idx_mask: *mut u64) -> c_int {
    int ib_get_ucaps(int *fds, int fd_count, uint64_t *idx_mask)
    {
    let mut ret: c_int = 0;
    dev_t dev;
// idx_mask = 0;
    mutex_lock(&ucaps_mutex);
    for (int i = 0; i < fd_count; i++) {
    ret = get_devt_from_fd(fds[i], &dev);
    if (ret)
    goto end;
    ret = get_ucap_from_devt(dev, idx_mask);
    if (ret)
    goto end;
    }
    end:
    mutex_unlock(&ucaps_mutex);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(ib_get_ucaps, "rdma_core");
    module_exit(ib_cleanup_ucaps);
