//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/virtio-rng.c
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
//
// Randomness driver for virtio
// Copyright (C) 2007, 2008 Rusty Russell IBM Corporation
//

    static DEFINE_IDA(rng_index_ida);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtrng_info {
    pub hwrng: hwrng,
    pub vq: *mut virtqueue,
    pub name: [c_char; 25],
    pub index: c_int,
    pub hwrng_register_done: bool,
    pub hwrng_removed: bool,
// data transfer
    pub have_data: completion,
    pub data_avail: c_uint,
    pub data_idx: c_uint,
// minimal size returned by rng_buffer_size()
    pub data: [u8; 32],    pub data: [u8; SMP_CACHE_BYTES],
}

#[no_mangle]
unsafe extern "C" fn random_recv_done(vq: *mut virtqueue) {
    static void random_recv_done(struct virtqueue *vq)
    {
    struct virtrng_info *vi = vq.vdev.priv;
    unsigned int len;
// We can get spurious callbacks, e.g. shared IRQs + virtio_pci.
    if (!virtqueue_get_buf(vi.vq, &len))
    return;
    smp_store_release(&vi.data_avail, len);
    complete(&vi.have_data);
    }
#[no_mangle]
unsafe extern "C" fn request_entropy(vi: *mut virtrng_info) {
    static void request_entropy(struct virtrng_info *vi)
    {
    struct scatterlist sg;
    reinit_completion(&vi.have_data);
    vi.data_idx = 0;
    sg_init_one(&sg, vi.data, sizeof(vi.data));
// There should always be room for one buffer.
    virtqueue_add_inbuf(vi.vq, &sg, 1, vi.data, GFP_KERNEL);
    virtqueue_kick(vi.vq);
    }
    static unsigned int copy_data(struct virtrng_info *vi, void *buf,
    unsigned int size)
    {
    unsigned int idx, avail;
//
// vi->data_avail was set from the device-reported used.len and
// vi->data_idx was advanced by previous copy_data() calls.  A
// malicious or buggy virtio-rng backend can drive either past
// sizeof(vi->data).  Clamp at point of use and harden the index
// with array_index_nospec() so the memcpy() below cannot be
// steered into adjacent slab memory, including under
// speculation.
//
    avail = min_t(unsigned int, vi.data_avail, sizeof(vi.data));
    if (vi.data_idx >= avail) {
    vi.data_avail = 0;
    request_entropy(vi);
    return 0;
    }
    size = min_t(unsigned int, size, avail - vi.data_idx);
    idx = array_index_nospec(vi.data_idx, sizeof(vi.data));
    memcpy(buf, vi.data + idx, size);
    vi.data_idx += size;
    vi.data_avail -= size;
    if (vi.data_avail == 0)
    request_entropy(vi);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn virtio_read(rng: *mut hwrng, buf: *mut c_void, size: usize, wait: bool) -> c_int {
    static int virtio_read(struct hwrng *rng, void *buf, size_t size, bool wait)
    {
    int ret;
    struct virtrng_info *vi = (struct virtrng_info *)rng.priv;
    unsigned int chunk;
    size_t read;
    if (vi.hwrng_removed)
    return -ENODEV;
    read = 0;
// copy available data
    if (smp_load_acquire(&vi.data_avail)) {
    chunk = copy_data(vi, buf, size);
    size -= chunk;
    read += chunk;
    }
    if (!wait)
    return read;
// We have already copied available entropy,
// so either size is 0 or data_avail is 0
//
    while (size != 0) {
// data_avail is 0 but a request is pending
    ret = wait_for_completion_killable(&vi.have_data);
    if (ret < 0)
    return ret;
// if vi->data_avail is 0, we have been interrupted
// by a cleanup, but buffer stays in the queue
//
    if (vi.data_avail == 0)
    return read;
    chunk = copy_data(vi, buf + read, size);
    size -= chunk;
    read += chunk;
    }
    return read;
    }
#[no_mangle]
unsafe extern "C" fn virtio_cleanup(rng: *mut hwrng) {
    static void virtio_cleanup(struct hwrng *rng)
    {
    struct virtrng_info *vi = (struct virtrng_info *)rng.priv;
    complete(&vi.have_data);
    }
#[no_mangle]
unsafe extern "C" fn probe_common(vdev: *mut virtio_device) -> c_int {
    static int probe_common(struct virtio_device *vdev)
    {
    int err, index;
    struct virtrng_info *vi = core::ptr::null_mut();
    vi = kzalloc_obj(struct virtrng_info);
    if (!vi)
    return -ENOMEM;
    vi.index = index = ida_alloc(&rng_index_ida, GFP_KERNEL);
    if (index < 0) {
    err = index;
    goto err_ida;
    }
    sprintf(vi.name, "virtio_rng.%d", index);
    init_completion(&vi.have_data);
    vi.hwrng = (struct hwrng) {
    .read = virtio_read,
    .cleanup = virtio_cleanup,
    .priv = (unsigned long)vi,
    .name = vi.name,
    };
    vdev.priv = vi;
// We expect a single virtqueue.
    vi.vq = virtio_find_single_vq(vdev, random_recv_done, "input");
    if (IS_ERR(vi.vq)) {
    err = PTR_ERR(vi.vq);
    goto err_find;
    }
    virtio_device_ready(vdev);
// we always have a pending entropy request
    request_entropy(vi);
    return 0;
    err_find:
    ida_free(&rng_index_ida, index);
    err_ida:
    kfree(vi);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn remove_common(vdev: *mut virtio_device) {
    static void remove_common(struct virtio_device *vdev)
    {
    struct virtrng_info *vi = vdev.priv;
    vi.hwrng_removed = true;
    vi.data_avail = 0;
    vi.data_idx = 0;
    complete(&vi.have_data);
    if (vi.hwrng_register_done)
    hwrng_unregister(&vi.hwrng);
    virtio_reset_device(vdev);
    vdev.config.del_vqs(vdev);
    ida_free(&rng_index_ida, vi.index);
    kfree(vi);
    }
#[no_mangle]
unsafe extern "C" fn virtrng_probe(vdev: *mut virtio_device) -> c_int {
    static int virtrng_probe(struct virtio_device *vdev)
    {
    return probe_common(vdev);
    }
#[no_mangle]
unsafe extern "C" fn virtrng_remove(vdev: *mut virtio_device) {
    static void virtrng_remove(struct virtio_device *vdev)
    {
    remove_common(vdev);
    }
#[no_mangle]
unsafe extern "C" fn virtrng_scan(vdev: *mut virtio_device) {
    static void virtrng_scan(struct virtio_device *vdev)
    {
    struct virtrng_info *vi = vdev.priv;
    int err;
    err = hwrng_register(&vi.hwrng);
    if (!err)
    vi.hwrng_register_done = true;
    }
#[no_mangle]
unsafe extern "C" fn virtrng_freeze(vdev: *mut virtio_device) -> c_int {
    static int virtrng_freeze(struct virtio_device *vdev)
    {
    remove_common(vdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtrng_restore(vdev: *mut virtio_device) -> c_int {
    static int virtrng_restore(struct virtio_device *vdev)
    {
    int err;
    err = probe_common(vdev);
    if (!err) {
    struct virtrng_info *vi = vdev.priv;
//
// Set hwrng_removed to ensure that virtio_read()
// does not block waiting for data before the
// registration is complete.
//
    vi.hwrng_removed = true;
    err = hwrng_register(&vi.hwrng);
    if (!err) {
    vi.hwrng_register_done = true;
    vi.hwrng_removed = false;
    }
    }
    return err;
    }
    static const struct virtio_device_id id_table[] = {
    { VIRTIO_ID_RNG, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    static struct virtio_driver virtio_rng_driver = {
    .driver.name =	KBUILD_MODNAME,
    .id_table =	id_table,
    .probe =	virtrng_probe,
    .remove =	virtrng_remove,
    .scan =		virtrng_scan,
    .freeze =	pm_sleep_ptr(virtrng_freeze),
    .restore =	pm_sleep_ptr(virtrng_restore),
    };
    module_virtio_driver(virtio_rng_driver);
    MODULE_DEVICE_TABLE(virtio, id_table);
    MODULE_DESCRIPTION("Virtio random number driver");
    MODULE_LICENSE("GPL");
