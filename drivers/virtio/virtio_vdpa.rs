//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_vdpa.c
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
// VIRTIO based driver for vDPA device
//
// Copyright (c) 2020, Red Hat. All rights reserved.
// Author: Jason Wang <jasowang@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vdpa_device {
    pub vdev: virtio_device,
    pub vdpa: *mut vdpa_device,
    pub features: u64,
}

    static inline struct virtio_vdpa_device *
    to_virtio_vdpa_device(struct virtio_device *dev)
    {
    return container_of(dev, struct virtio_vdpa_device, vdev);
    }
    static struct vdpa_device *vd_get_vdpa(struct virtio_device *vdev)
    {
    return to_virtio_vdpa_device(vdev).vdpa;
    }
    static void virtio_vdpa_get(struct virtio_device *vdev, unsigned int offset,
    void *buf, unsigned int len)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    vdpa_get_config(vdpa, offset, buf, len);
    }
    static void virtio_vdpa_set(struct virtio_device *vdev, unsigned int offset,
    const void *buf, unsigned int len)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    vdpa_set_config(vdpa, offset, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_generation(vdev: *mut virtio_device) -> u32 {
    static u32 virtio_vdpa_generation(struct virtio_device *vdev)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    if (ops.get_generation)
    return ops.get_generation(vdpa);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_get_status(vdev: *mut virtio_device) -> u8 {
    static u8 virtio_vdpa_get_status(struct virtio_device *vdev)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    return ops.get_status(vdpa);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_set_status(vdev: *mut virtio_device, status: u8) {
    static void virtio_vdpa_set_status(struct virtio_device *vdev, u8 status)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    vdpa_set_status(vdpa, status);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_reset(vdev: *mut virtio_device) {
    static void virtio_vdpa_reset(struct virtio_device *vdev)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    vdpa_reset(vdpa, 0);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_notify(vq: *mut virtqueue) -> bool {
    static bool virtio_vdpa_notify(struct virtqueue *vq)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vq.vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    ops.kick_vq(vdpa, vq.index);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_notify_with_data(vq: *mut virtqueue) -> bool {
    static bool virtio_vdpa_notify_with_data(struct virtqueue *vq)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vq.vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    let mut data: u32 = vring_notification_data(vq);
    ops.kick_vq_with_data(vdpa, data);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_config_cb(private: *mut c_void) -> irqreturn_t {
    static irqreturn_t virtio_vdpa_config_cb(void *private)
    {
    struct virtio_vdpa_device *vd_dev = private;
    virtio_config_changed(&vd_dev.vdev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_virtqueue_cb(private: *mut c_void) -> irqreturn_t {
    static irqreturn_t virtio_vdpa_virtqueue_cb(void *private)
    {
    struct virtqueue *vq = private;
    return vring_interrupt(0, vq);
    }
    static struct virtqueue *
    virtio_vdpa_setup_vq(struct virtio_device *vdev, unsigned int index,
    void (*callback)(struct virtqueue *vq),
    const char *name, bool ctx)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    bool (*notify)(struct virtqueue *vq) = virtio_vdpa_notify;
    struct vdpa_callback cb;
    struct virtqueue *vq;
    u64 desc_addr, driver_addr, device_addr;
    let mut map: union virtio_map = {0};
// Assume split virtqueue, switch to packed if necessary
    let mut state: vdpa_vq_state = {0};
    u32 align, max_num, min_num = 1;
    let mut may_reduce_num: bool = true;
    int err;
    if (!name)
    return core::ptr::null_mut();
    if (index >= vdpa.nvqs)
    return ERR_PTR(-ENOENT);
// We cannot accept VIRTIO_F_NOTIFICATION_DATA without kick_vq_with_data
    if (__virtio_test_bit(vdev, VIRTIO_F_NOTIFICATION_DATA)) {
    if (ops.kick_vq_with_data)
    notify = virtio_vdpa_notify_with_data;
    else
    __virtio_clear_bit(vdev, VIRTIO_F_NOTIFICATION_DATA);
    }
// Queue shouldn't already be set up.
    if (ops.get_vq_ready(vdpa, index))
    return ERR_PTR(-ENOENT);
    if (ops.get_vq_size)
    max_num = ops.get_vq_size(vdpa, index);
    else
    max_num = ops.get_vq_num_max(vdpa);
    if (max_num == 0) {
    err = -ENOENT;
    goto error_new_virtqueue;
    }
    if (ops.get_vq_num_min)
    min_num = ops.get_vq_num_min(vdpa);
    may_reduce_num = (max_num != min_num);
// Create the vring
    align = ops.get_vq_align(vdpa);
    if (ops.get_vq_map)
    map = ops.get_vq_map(vdpa, index);
    else
    map = vdpa_get_map(vdpa);
    vq = vring_create_virtqueue_map(index, max_num, align, vdev,
    true, may_reduce_num, ctx,
    notify, callback, name, map);
    if (!vq) {
    err = -ENOMEM;
    goto error_new_virtqueue;
    }
    if (index == 0)
    vdev.vmap = map;
    vq.num_max = max_num;
// Setup virtqueue callback
    cb.callback = callback ? virtio_vdpa_virtqueue_cb : core::ptr::null_mut();
    cb.private = vq;
    cb.trigger = core::ptr::null_mut();
    ops.set_vq_cb(vdpa, index, &cb);
    ops.set_vq_num(vdpa, index, virtqueue_get_vring_size(vq));
    desc_addr = virtqueue_get_desc_addr(vq);
    driver_addr = virtqueue_get_avail_addr(vq);
    device_addr = virtqueue_get_used_addr(vq);
    if (ops.set_vq_address(vdpa, index,
    desc_addr, driver_addr,
    device_addr)) {
    err = -EINVAL;
    goto err_vq;
    }
// reset virtqueue state index
    if (virtio_has_feature(vdev, VIRTIO_F_RING_PACKED)) {
    struct vdpa_vq_state_packed *s = &state.packed;
    s.last_avail_counter = 1;
    s.last_avail_idx = 0;
    s.last_used_counter = 1;
    s.last_used_idx = 0;
    }
    err = ops.set_vq_state(vdpa, index, &state);
    if (err)
    goto err_vq;
    ops.set_vq_ready(vdpa, index, 1);
    return vq;
    err_vq:
    vring_del_virtqueue(vq);
    error_new_virtqueue:
    ops.set_vq_ready(vdpa, index, 0);
// VDPA driver should make sure vq is stopeed here
    WARN_ON(ops.get_vq_ready(vdpa, index));
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_del_vq(vq: *mut virtqueue) {
    static void virtio_vdpa_del_vq(struct virtqueue *vq)
    {
    struct virtio_vdpa_device *vd_dev = to_virtio_vdpa_device(vq.vdev);
    struct vdpa_device *vdpa = vd_dev.vdpa;
    const struct vdpa_config_ops *ops = vdpa.config;
    let mut index: c_uint = vq.index;
// Select and deactivate the queue (best effort)
    ops.set_vq_ready(vdpa, index, 0);
    vring_del_virtqueue(vq);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_del_vqs(vdev: *mut virtio_device) {
    static void virtio_vdpa_del_vqs(struct virtio_device *vdev)
    {
    struct virtqueue *vq, *n;
    list_for_each_entry_safe(vq, n, &vdev.vqs, list)
    virtio_vdpa_del_vq(vq);
    }
#[no_mangle]
unsafe extern "C" fn default_calc_sets(affd: *mut irq_affinity, affvecs: c_uint) {
    static void default_calc_sets(struct irq_affinity *affd, unsigned int affvecs)
    {
    affd.nr_sets = 1;
    affd.set_size[0] = affvecs;
    }
    static struct cpumask *
    create_affinity_masks(unsigned int nvecs, struct irq_affinity *affd)
    {
    let mut affvecs: c_uint = 0, curvec, usedvecs, i;
    struct cpumask *masks = core::ptr::null_mut();
    if (nvecs > affd.pre_vectors + affd.post_vectors)
    affvecs = nvecs - affd.pre_vectors - affd.post_vectors;
    if (!affd.calc_sets)
    affd.calc_sets = default_calc_sets;
    affd.calc_sets(affd, affvecs);
    if (!affvecs)
    return core::ptr::null_mut();
    masks = kzalloc_objs(*masks, nvecs);
    if (!masks)
    return core::ptr::null_mut();
// Fill out vectors at the beginning that don't need affinity
    for (curvec = 0; curvec < affd.pre_vectors; curvec++)
    cpumask_setall(&masks[curvec]);
    for (i = 0, usedvecs = 0; i < affd.nr_sets; i++) {
    let mut this_vecs: c_uint = affd.set_size[i];
    unsigned int nr_masks;
    int j;
    struct cpumask *result = group_cpus_evenly(this_vecs, &nr_masks);
    if (!result) {
    kfree(masks);
    return core::ptr::null_mut();
    }
    for (j = 0; j < nr_masks; j++)
    cpumask_copy(&masks[curvec + j], &result[j]);
    kfree(result);
    curvec += nr_masks;
    usedvecs += nr_masks;
    }
// Fill out vectors at the end that don't need affinity
    if (usedvecs >= affvecs)
    curvec = affd.pre_vectors + affvecs;
    else
    curvec = affd.pre_vectors + usedvecs;
    for (; curvec < nvecs; curvec++)
    cpumask_setall(&masks[curvec]);
    return masks;
    }
    static int virtio_vdpa_find_vqs(struct virtio_device *vdev, unsigned int nvqs,
    struct virtqueue *vqs[],
    struct virtqueue_info vqs_info[],
    struct irq_affinity *desc)
    {
    struct virtio_vdpa_device *vd_dev = to_virtio_vdpa_device(vdev);
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    struct cpumask *masks;
    struct vdpa_callback cb;
    let mut has_affinity: bool = desc && ops.set_vq_affinity;
    int i, err, queue_idx = 0;
    if (has_affinity) {
    masks = create_affinity_masks(nvqs, desc);
    if (!masks)
    return -ENOMEM;
    }
    for (i = 0; i < nvqs; ++i) {
    struct virtqueue_info *vqi = &vqs_info[i];
    if (!vqi.name) {
    vqs[i] = core::ptr::null_mut();
    continue;
    }
    vqs[i] = virtio_vdpa_setup_vq(vdev, queue_idx++, vqi.callback,
    vqi.name, vqi.ctx);
    if (IS_ERR(vqs[i])) {
    err = PTR_ERR(vqs[i]);
    goto err_setup_vq;
    }
    if (has_affinity)
    ops.set_vq_affinity(vdpa, i, &masks[i]);
    }
    cb.callback = virtio_vdpa_config_cb;
    cb.private = vd_dev;
    ops.set_config_cb(vdpa, &cb);
    if (has_affinity)
    kfree(masks);
    return 0;
    err_setup_vq:
    virtio_vdpa_del_vqs(vdev);
    if (has_affinity)
    kfree(masks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_get_features(vdev: *mut virtio_device) -> u64 {
    static u64 virtio_vdpa_get_features(struct virtio_device *vdev)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    return ops.get_device_features(vdpa);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_finalize_features(vdev: *mut virtio_device) -> c_int {
    static int virtio_vdpa_finalize_features(struct virtio_device *vdev)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
// Give virtio_ring a chance to accept features.
    vring_transport_features(vdev);
    return vdpa_set_features(vdpa, vdev.features);
    }
    static const char *virtio_vdpa_bus_name(struct virtio_device *vdev)
    {
    struct virtio_vdpa_device *vd_dev = to_virtio_vdpa_device(vdev);
    struct vdpa_device *vdpa = vd_dev.vdpa;
    return dev_name(&vdpa.dev);
    }
    static int virtio_vdpa_set_vq_affinity(struct virtqueue *vq,
    const struct cpumask *cpu_mask)
    {
    struct virtio_vdpa_device *vd_dev = to_virtio_vdpa_device(vq.vdev);
    struct vdpa_device *vdpa = vd_dev.vdpa;
    const struct vdpa_config_ops *ops = vdpa.config;
    let mut index: c_uint = vq.index;
    if (ops.set_vq_affinity)
    return ops.set_vq_affinity(vdpa, index, cpu_mask);
    return 0;
    }
    static const struct cpumask *
    virtio_vdpa_get_vq_affinity(struct virtio_device *vdev, int index)
    {
    struct vdpa_device *vdpa = vd_get_vdpa(vdev);
    const struct vdpa_config_ops *ops = vdpa.config;
    if (ops.get_vq_affinity)
    return ops.get_vq_affinity(vdpa, index);
    return core::ptr::null_mut();
    }
    static const struct virtio_config_ops virtio_vdpa_config_ops = {
    .get		= virtio_vdpa_get,
    .set		= virtio_vdpa_set,
    .generation	= virtio_vdpa_generation,
    .get_status	= virtio_vdpa_get_status,
    .set_status	= virtio_vdpa_set_status,
    .reset		= virtio_vdpa_reset,
    .find_vqs	= virtio_vdpa_find_vqs,
    .del_vqs	= virtio_vdpa_del_vqs,
    .get_features	= virtio_vdpa_get_features,
    .finalize_features = virtio_vdpa_finalize_features,
    .bus_name	= virtio_vdpa_bus_name,
    .set_vq_affinity = virtio_vdpa_set_vq_affinity,
    .get_vq_affinity = virtio_vdpa_get_vq_affinity,
    };
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_release_dev(_d: *mut device) {
    static void virtio_vdpa_release_dev(struct device *_d)
    {
    struct virtio_device *vdev =
    container_of(_d, struct virtio_device, dev);
    struct virtio_vdpa_device *vd_dev =
    container_of(vdev, struct virtio_vdpa_device, vdev);
    kfree(vd_dev);
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_probe(vdpa: *mut vdpa_device) -> c_int {
    static int virtio_vdpa_probe(struct vdpa_device *vdpa)
    {
    const struct vdpa_config_ops *ops = vdpa.config;
    struct virtio_vdpa_device *vd_dev, *reg_dev = core::ptr::null_mut();
    let mut ret: c_int = -EINVAL;
    vd_dev = kzalloc_obj(*vd_dev);
    if (!vd_dev)
    return -ENOMEM;
    vd_dev.vdev.dev.parent = vdpa.map ? &vdpa.dev :
    vdpa_get_map(vdpa).dma_dev;
    vd_dev.vdev.dev.release = virtio_vdpa_release_dev;
    vd_dev.vdev.config = &virtio_vdpa_config_ops;
    vd_dev.vdev.map = vdpa.map;
    vd_dev.vdpa = vdpa;
    vd_dev.vdev.id.device = ops.get_device_id(vdpa);
    if (vd_dev.vdev.id.device == 0)
    goto err;
    vd_dev.vdev.id.vendor = ops.get_vendor_id(vdpa);
    ret = register_virtio_device(&vd_dev.vdev);
    reg_dev = vd_dev;
    if (ret)
    goto err;
    vdpa_set_drvdata(vdpa, vd_dev);
    return 0;
    err:
    if (reg_dev)
    put_device(&vd_dev.vdev.dev);
    else
    kfree(vd_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtio_vdpa_remove(vdpa: *mut vdpa_device) {
    static void virtio_vdpa_remove(struct vdpa_device *vdpa)
    {
    struct virtio_vdpa_device *vd_dev = vdpa_get_drvdata(vdpa);
    unregister_virtio_device(&vd_dev.vdev);
    }
    static struct vdpa_driver virtio_vdpa_driver = {
    .driver = {
    .name	= "virtio_vdpa",
    },
    .probe	= virtio_vdpa_probe,
    .remove = virtio_vdpa_remove,
    };
    module_vdpa_driver(virtio_vdpa_driver);
    MODULE_VERSION(MOD_VERSION);
    MODULE_LICENSE(MOD_LICENSE);
    MODULE_AUTHOR(MOD_AUTHOR);
    MODULE_DESCRIPTION(MOD_DESC);
