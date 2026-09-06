//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_pci_legacy.c
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
// Virtio PCI driver - legacy device support
//
// This module allows virtio devices to be used over a virtual PCI device.
// This can be used with QEMU based VMMs like KVM or Xen.
//
// Copyright IBM Corp. 2007
// Copyright Red Hat, Inc. 2014
//
// Authors:
// Anthony Liguori  <aliguori@us.ibm.com>
// Rusty Russell <rusty@rustcorp.com.au>
// Michael S. Tsirkin <mst@redhat.com>
//

// virtio config->get_features() implementation
#[no_mangle]
unsafe extern "C" fn vp_get_features(vdev: *mut virtio_device) -> u64 {
    static u64 vp_get_features(struct virtio_device *vdev)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
// When someone needs more than 32 feature bits, we'll need to
// steal a bit to indicate that the rest are somewhere else.
    return vp_legacy_get_features(&vp_dev.ldev);
    }
// virtio config->finalize_features() implementation
#[no_mangle]
unsafe extern "C" fn vp_finalize_features(vdev: *mut virtio_device) -> c_int {
    static int vp_finalize_features(struct virtio_device *vdev)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
// Give virtio_ring a chance to accept features.
    vring_transport_features(vdev);
// Make sure we don't have any features > 32 bits!
    BUG_ON((u32)vdev.features != vdev.features);
// We only support 32 feature bits.
    vp_legacy_set_features(&vp_dev.ldev, vdev.features);
    return 0;
    }
// virtio config->get() implementation
    static void vp_get(struct virtio_device *vdev, unsigned int offset,
    void *buf, unsigned int len)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
    void __iomem *ioaddr = vp_dev.ldev.ioaddr +
    VIRTIO_PCI_CONFIG_OFF(vp_dev.msix_enabled) +
    offset;
    u8 *ptr = buf;
    int i;
    for (i = 0; i < len; i++)
    ptr[i] = ioread8(ioaddr + i);
    }
// the config->set() implementation.  it's symmetric to the config->get()
// implementation
    static void vp_set(struct virtio_device *vdev, unsigned int offset,
    const void *buf, unsigned int len)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
    void __iomem *ioaddr = vp_dev.ldev.ioaddr +
    VIRTIO_PCI_CONFIG_OFF(vp_dev.msix_enabled) +
    offset;
    const u8 *ptr = buf;
    int i;
    for (i = 0; i < len; i++)
    iowrite8(ptr[i], ioaddr + i);
    }
// config->{get,set}_status() implementations
#[no_mangle]
unsafe extern "C" fn vp_get_status(vdev: *mut virtio_device) -> u8 {
    static u8 vp_get_status(struct virtio_device *vdev)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
    return vp_legacy_get_status(&vp_dev.ldev);
    }
#[no_mangle]
unsafe extern "C" fn vp_set_status(vdev: *mut virtio_device, status: u8) {
    static void vp_set_status(struct virtio_device *vdev, u8 status)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
// We should never be setting status to 0.
    BUG_ON(status == 0);
    vp_legacy_set_status(&vp_dev.ldev, status);
    }
#[no_mangle]
unsafe extern "C" fn vp_reset(vdev: *mut virtio_device) {
    static void vp_reset(struct virtio_device *vdev)
    {
    struct virtio_pci_device *vp_dev = to_vp_device(vdev);
// 0 status means a reset.
    vp_legacy_set_status(&vp_dev.ldev, 0);
// Flush out the status write, and flush in device writes,
// including MSi-X interrupts, if any.
    vp_legacy_get_status(&vp_dev.ldev);
// Flush pending VQ/configuration callbacks.
    vp_synchronize_vectors(vdev);
    }
#[no_mangle]
unsafe extern "C" fn vp_config_vector(vp_dev: *mut virtio_pci_device, vector: u16) -> u16 {
    static u16 vp_config_vector(struct virtio_pci_device *vp_dev, u16 vector)
    {
    return vp_legacy_config_vector(&vp_dev.ldev, vector);
    }
    static struct virtqueue *setup_vq(struct virtio_pci_device *vp_dev,
    struct virtio_pci_vq_info *info,
    unsigned int index,
    void (*callback)(struct virtqueue *vq),
    const char *name,
    bool ctx,
    u16 msix_vec)
    {
    struct virtqueue *vq;
    u16 num;
    int err;
    u64 q_pfn;
// Check if queue is either not available or already active.
    num = vp_legacy_get_queue_size(&vp_dev.ldev, index);
    if (!num || vp_legacy_get_queue_enable(&vp_dev.ldev, index))
    return ERR_PTR(-ENOENT);
    info.msix_vector = msix_vec;
// create the vring
    vq = vring_create_virtqueue(index, num,
    VIRTIO_PCI_VRING_ALIGN, &vp_dev.vdev,
    true, false, ctx,
    vp_notify, callback, name);
    if (!vq)
    return ERR_PTR(-ENOMEM);
    vq.num_max = num;
    q_pfn = virtqueue_get_desc_addr(vq) >> VIRTIO_PCI_QUEUE_ADDR_SHIFT;
    if (q_pfn >> 32) {
    dev_err(&vp_dev.pci_dev.dev,
    "platform bug: legacy virtio-pci must not be used with RAM above 0x%llxGB\n",
    0x1ULL << (32 + PAGE_SHIFT - 30));
    err = -E2BIG;
    goto out_del_vq;
    }
// activate the queue
    vp_legacy_set_queue_address(&vp_dev.ldev, index, q_pfn);
    vq.priv = (void  *)vp_dev.ldev.ioaddr + VIRTIO_PCI_QUEUE_NOTIFY;
    if (msix_vec != VIRTIO_MSI_NO_VECTOR) {
    msix_vec = vp_legacy_queue_vector(&vp_dev.ldev, index, msix_vec);
    if (msix_vec == VIRTIO_MSI_NO_VECTOR) {
    err = -EBUSY;
    goto out_deactivate;
    }
    }
    return vq;
    out_deactivate:
    vp_legacy_set_queue_address(&vp_dev.ldev, index, 0);
    out_del_vq:
    vring_del_virtqueue(vq);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn del_vq(info: *mut virtio_pci_vq_info) {
    static void del_vq(struct virtio_pci_vq_info *info)
    {
    struct virtqueue *vq = info.vq;
    struct virtio_pci_device *vp_dev = to_vp_device(vq.vdev);
    if (vp_dev.msix_enabled) {
    vp_legacy_queue_vector(&vp_dev.ldev, vq.index,
    VIRTIO_MSI_NO_VECTOR);
// Flush the write out to device
    ioread8(vp_dev.ldev.ioaddr + VIRTIO_PCI_ISR);
    }
// Select and deactivate the queue
    vp_legacy_set_queue_address(&vp_dev.ldev, vq.index, 0);
    vring_del_virtqueue(vq);
    }
    static const struct virtio_config_ops virtio_pci_config_ops = {
    .get		= vp_get,
    .set		= vp_set,
    .get_status	= vp_get_status,
    .set_status	= vp_set_status,
    .reset		= vp_reset,
    .find_vqs	= vp_find_vqs,
    .del_vqs	= vp_del_vqs,
    .synchronize_cbs = vp_synchronize_vectors,
    .get_features	= vp_get_features,
    .finalize_features = vp_finalize_features,
    .bus_name	= vp_bus_name,
    .set_vq_affinity = vp_set_vq_affinity,
    .get_vq_affinity = vp_get_vq_affinity,
    };
// the PCI probing function
#[no_mangle]
pub unsafe extern "C" fn virtio_pci_legacy_probe(vp_dev: *mut virtio_pci_device) -> c_int {
    int virtio_pci_legacy_probe(struct virtio_pci_device *vp_dev)
    {
    struct virtio_pci_legacy_device *ldev = &vp_dev.ldev;
    struct pci_dev *pci_dev = vp_dev.pci_dev;
    int rc;
    ldev.pci_dev = pci_dev;
    rc = vp_legacy_probe(ldev);
    if (rc)
    return rc;
    vp_dev.isr = ldev.isr;
    vp_dev.vdev.id = ldev.id;
    vp_dev.vdev.config = &virtio_pci_config_ops;
    vp_dev.config_vector = vp_config_vector;
    vp_dev.setup_vq = setup_vq;
    vp_dev.del_vq = del_vq;
    vp_dev.is_legacy = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_pci_legacy_remove(vp_dev: *mut virtio_pci_device) {
    void virtio_pci_legacy_remove(struct virtio_pci_device *vp_dev)
    {
    struct virtio_pci_legacy_device *ldev = &vp_dev.ldev;
    vp_legacy_remove(ldev);
    }
