//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/virtio/virtio_crypto_mgr.c
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
// Management for virtio crypto devices (refer to adf_dev_mgr.c)
//
// Copyright 2016 HUAWEI TECHNOLOGIES CO., LTD.
//

    static LIST_HEAD(virtio_crypto_table);
    static uint32_t num_devices;
// The table_lock protects the above global list and num_devices
    static DEFINE_MUTEX(table_lock);
pub const VIRTIO_CRYPTO_MAX_DEVICES: c_int = 32;
//
// virtcrypto_devmgr_add_dev() - Add vcrypto_dev to the acceleration
// framework.
// @vcrypto_dev:  Pointer to virtio crypto device.
//
// Function adds virtio crypto device to the global list.
// To be used by virtio crypto device specific drivers.
//
// Return: 0 on success, error code othewise.
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_devmgr_add_dev(vcrypto_dev: *mut virtio_crypto) -> c_int {
    int virtcrypto_devmgr_add_dev(struct virtio_crypto *vcrypto_dev)
    {
    struct list_head *itr;
    mutex_lock(&table_lock);
    if (num_devices == VIRTIO_CRYPTO_MAX_DEVICES) {
    pr_info("virtio_crypto: only support up to %d devices\n",
    VIRTIO_CRYPTO_MAX_DEVICES);
    mutex_unlock(&table_lock);
    return -EFAULT;
    }
    list_for_each(itr, &virtio_crypto_table) {
    struct virtio_crypto *ptr =
    list_entry(itr, struct virtio_crypto, list);
    if (ptr == vcrypto_dev) {
    mutex_unlock(&table_lock);
    return -EEXIST;
    }
    }
    atomic_set(&vcrypto_dev.ref_count, 0);
    list_add_tail(&vcrypto_dev.list, &virtio_crypto_table);
    vcrypto_dev.dev_id = num_devices++;
    mutex_unlock(&table_lock);
    return 0;
    }
    struct list_head *virtcrypto_devmgr_get_head(void)
    {
    return &virtio_crypto_table;
    }
//
// virtcrypto_devmgr_rm_dev() - Remove vcrypto_dev from the acceleration
// framework.
// @vcrypto_dev:  Pointer to virtio crypto device.
//
// Function removes virtio crypto device from the acceleration framework.
// To be used by virtio crypto device specific drivers.
//
// Return: void
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_devmgr_rm_dev(vcrypto_dev: *mut virtio_crypto) {
    void virtcrypto_devmgr_rm_dev(struct virtio_crypto *vcrypto_dev)
    {
    mutex_lock(&table_lock);
    list_del(&vcrypto_dev.list);
    num_devices--;
    mutex_unlock(&table_lock);
    }
//
// virtcrypto_dev_get() - Increment vcrypto_dev reference count
// @vcrypto_dev: Pointer to virtio crypto device.
//
// Increment the vcrypto_dev refcount and if this is the first time
// incrementing it during this period the vcrypto_dev is in use,
// increment the module refcount too.
// To be used by virtio crypto device specific drivers.
//
// Return: 0 when successful, EFAULT when fail to bump module refcount
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_dev_get(vcrypto_dev: *mut virtio_crypto) -> c_int {
    int virtcrypto_dev_get(struct virtio_crypto *vcrypto_dev)
    {
    if (atomic_add_return(1, &vcrypto_dev.ref_count) == 1)
    if (!try_module_get(vcrypto_dev.owner))
    return -EFAULT;
    return 0;
    }
//
// virtcrypto_dev_put() - Decrement vcrypto_dev reference count
// @vcrypto_dev: Pointer to virtio crypto device.
//
// Decrement the vcrypto_dev refcount and if this is the last time
// decrementing it during this period the vcrypto_dev is in use,
// decrement the module refcount too.
// To be used by virtio crypto device specific drivers.
//
// Return: void
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_dev_put(vcrypto_dev: *mut virtio_crypto) {
    void virtcrypto_dev_put(struct virtio_crypto *vcrypto_dev)
    {
    if (atomic_sub_return(1, &vcrypto_dev.ref_count) == 0)
    module_put(vcrypto_dev.owner);
    }
//
// virtcrypto_dev_started() - Check whether device has started
// @vcrypto_dev: Pointer to virtio crypto device.
//
// To be used by virtio crypto device specific drivers.
//
// Return: 1 when the device has started, 0 otherwise
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_dev_started(vcrypto_dev: *mut virtio_crypto) -> c_int {
    int virtcrypto_dev_started(struct virtio_crypto *vcrypto_dev)
    {
    return (vcrypto_dev.status & VIRTIO_CRYPTO_S_HW_READY);
    }
//
// virtcrypto_get_dev_node() - Get vcrypto_dev on the node.
// @node:  Node id the driver works.
// @service: Crypto service that needs to be supported by the
// dev
// @algo: The algorithm number that needs to be supported by the
// dev
//
// Function returns the virtio crypto device used fewest on the node,
// and supports the given crypto service and algorithm.
//
// To be used by virtio crypto device specific drivers.
//
// Return: pointer to vcrypto_dev or NULL if not found.
//
    struct virtio_crypto *virtcrypto_get_dev_node(int node, uint32_t service,
    uint32_t algo)
    {
    struct virtio_crypto *vcrypto_dev = core::ptr::null_mut(), *tmp_dev;
    let mut best: c_ulong = ~0;
    unsigned long ctr;
    mutex_lock(&table_lock);
    list_for_each_entry(tmp_dev, virtcrypto_devmgr_get_head(), list) {
    if ((node == dev_to_node(&tmp_dev.vdev.dev) ||
    dev_to_node(&tmp_dev.vdev.dev) < 0) &&
    virtcrypto_dev_started(tmp_dev) &&
    virtcrypto_algo_is_supported(tmp_dev, service, algo)) {
    ctr = atomic_read(&tmp_dev.ref_count);
    if (best > ctr) {
    vcrypto_dev = tmp_dev;
    best = ctr;
    }
    }
    }
    if (!vcrypto_dev) {
    pr_info("virtio_crypto: Could not find a device on node %d\n",
    node);
// Get any started device
    list_for_each_entry(tmp_dev,
    virtcrypto_devmgr_get_head(), list) {
    if (virtcrypto_dev_started(tmp_dev) &&
    virtcrypto_algo_is_supported(tmp_dev,
    service, algo)) {
    vcrypto_dev = tmp_dev;
    break;
    }
    }
    }
    mutex_unlock(&table_lock);
    if (!vcrypto_dev)
    return core::ptr::null_mut();
    virtcrypto_dev_get(vcrypto_dev);
    return vcrypto_dev;
    }
//
// virtcrypto_dev_start() - Start virtio crypto device
// @vcrypto:    Pointer to virtio crypto device.
//
// Function notifies all the registered services that the virtio crypto device
// is ready to be used.
// To be used by virtio crypto device specific drivers.
//
// Return: 0 on success, EFAULT when fail to register algorithms
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_dev_start(vcrypto: *mut virtio_crypto) -> c_int {
    int virtcrypto_dev_start(struct virtio_crypto *vcrypto)
    {
    if (virtio_crypto_skcipher_algs_register(vcrypto)) {
    pr_err("virtio_crypto: Failed to register crypto skcipher algs\n");
    return -EFAULT;
    }
    if (virtio_crypto_akcipher_algs_register(vcrypto)) {
    pr_err("virtio_crypto: Failed to register crypto akcipher algs\n");
    virtio_crypto_skcipher_algs_unregister(vcrypto);
    return -EFAULT;
    }
    return 0;
    }
//
// virtcrypto_dev_stop() - Stop virtio crypto device
// @vcrypto:    Pointer to virtio crypto device.
//
// Function notifies all the registered services that the virtio crypto device
// shall no longer be used.
// To be used by virtio crypto device specific drivers.
//
// Return: void
//
#[no_mangle]
pub unsafe extern "C" fn virtcrypto_dev_stop(vcrypto: *mut virtio_crypto) {
    void virtcrypto_dev_stop(struct virtio_crypto *vcrypto)
    {
    virtio_crypto_skcipher_algs_unregister(vcrypto);
    virtio_crypto_akcipher_algs_unregister(vcrypto);
    }
//
// vcrypto_algo_is_supported()
// @vcrypto: Pointer to virtio crypto device.
// @service: The bit number for service validate.
// See VIRTIO_CRYPTO_SERVICE_
// @algo : The bit number for the algorithm to validate.
//
// Validate if the virtio crypto device supports a service and
// algo.
//
// Return true if device supports a service and algo.
//
    bool virtcrypto_algo_is_supported(struct virtio_crypto *vcrypto,
    uint32_t service,
    uint32_t algo)
    {
    let mut service_mask: u32 = 1u << service;
    let mut algo_mask: u32 = 0;
    let mut low: bool = true;
    if (algo > 31) {
    algo -= 32;
    low = false;
    }
    if (!(vcrypto.crypto_services & service_mask))
    return false;
    switch (service) {
    case VIRTIO_CRYPTO_SERVICE_CIPHER:
    if (low)
    algo_mask = vcrypto.cipher_algo_l;
    else
    algo_mask = vcrypto.cipher_algo_h;
    break;
    case VIRTIO_CRYPTO_SERVICE_HASH:
    algo_mask = vcrypto.hash_algo;
    break;
    case VIRTIO_CRYPTO_SERVICE_MAC:
    if (low)
    algo_mask = vcrypto.mac_algo_l;
    else
    algo_mask = vcrypto.mac_algo_h;
    break;
    case VIRTIO_CRYPTO_SERVICE_AEAD:
    algo_mask = vcrypto.aead_algo;
    break;
    case VIRTIO_CRYPTO_SERVICE_AKCIPHER:
    algo_mask = vcrypto.akcipher_algo;
    break;
    }
    if (!(algo_mask & (1u << algo)))
    return false;
    return true;
    }
