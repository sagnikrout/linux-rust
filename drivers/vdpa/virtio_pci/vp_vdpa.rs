//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/virtio_pci/vp_vdpa.c
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
// vDPA bridge driver for modern virtio-pci device
//
// Copyright (c) 2020, Red Hat Inc. All rights reserved.
// Author: Jason Wang <jasowang@redhat.com>
//
// Based on virtio_pci_modern.c.
//

pub const VP_VDPA_QUEUE_MAX: c_int = 256;

pub const VP_VDPA_NAME_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_vring {
    pub notify: *mut void __iomem,
    pub msix_name: [c_char; VP_VDPA_NAME_SIZE],
    pub cb: vdpa_callback,
    pub notify_pa: resource_size_t,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_vdpa {
    pub vdpa: vdpa_device,
    pub mdev: *mut virtio_pci_modern_device,
    pub vring: *mut vp_vring,
    pub config_cb: vdpa_callback,
    pub device_features: u64,
    pub msix_name: [c_char; VP_VDPA_NAME_SIZE],
    pub config_irq: c_int,
    pub queues: c_int,
    pub vectors: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_vdpa_mgmtdev {
    pub mgtdev: vdpa_mgmt_dev,
    pub mdev: *mut virtio_pci_modern_device,
    pub vp_vdpa: *mut vp_vdpa,
}

    static struct vp_vdpa *vdpa_to_vp(struct vdpa_device *vdpa)
    {
    return container_of(vdpa, struct vp_vdpa, vdpa);
    }
    static struct virtio_pci_modern_device *vdpa_to_mdev(struct vdpa_device *vdpa)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    return vp_vdpa.mdev;
    }
    static struct virtio_pci_modern_device *vp_vdpa_to_mdev(struct vp_vdpa *vp_vdpa)
    {
    return vp_vdpa.mdev;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_device_features(vdpa: *mut vdpa_device) -> u64 {
    static u64 vp_vdpa_get_device_features(struct vdpa_device *vdpa)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    return vp_vdpa.device_features;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_set_driver_features(vdpa: *mut vdpa_device, features: u64) -> c_int {
    static int vp_vdpa_set_driver_features(struct vdpa_device *vdpa, u64 features)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    vp_modern_set_features(mdev, features);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_driver_features(vdpa: *mut vdpa_device) -> u64 {
    static u64 vp_vdpa_get_driver_features(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return vp_modern_get_driver_features(mdev);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_status(vdpa: *mut vdpa_device) -> u8 {
    static u8 vp_vdpa_get_status(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return vp_modern_get_status(mdev);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vq_irq(vdpa: *mut vdpa_device, idx: u16) -> c_int {
    static int vp_vdpa_get_vq_irq(struct vdpa_device *vdpa, u16 idx)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    let mut irq: c_int = vp_vdpa.vring[idx].irq;
    if (irq == VIRTIO_MSI_NO_VECTOR)
    return -EINVAL;
    return irq;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_free_irq(vp_vdpa: *mut vp_vdpa) {
    static void vp_vdpa_free_irq(struct vp_vdpa *vp_vdpa)
    {
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    struct pci_dev *pdev = mdev.pci_dev;
    int i;
    for (i = 0; i < vp_vdpa.queues; i++) {
    if (vp_vdpa.vring[i].irq != VIRTIO_MSI_NO_VECTOR) {
    vp_modern_queue_vector(mdev, i, VIRTIO_MSI_NO_VECTOR);
    devm_free_irq(&pdev.dev, vp_vdpa.vring[i].irq,
    &vp_vdpa.vring[i]);
    vp_vdpa.vring[i].irq = VIRTIO_MSI_NO_VECTOR;
    }
    }
    if (vp_vdpa.config_irq != VIRTIO_MSI_NO_VECTOR) {
    vp_modern_config_vector(mdev, VIRTIO_MSI_NO_VECTOR);
    devm_free_irq(&pdev.dev, vp_vdpa.config_irq, vp_vdpa);
    vp_vdpa.config_irq = VIRTIO_MSI_NO_VECTOR;
    }
    if (vp_vdpa.vectors) {
    pci_free_irq_vectors(pdev);
    vp_vdpa.vectors = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_vq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t vp_vdpa_vq_handler(int irq, void *arg)
    {
    struct vp_vring *vring = arg;
    if (vring.cb.callback)
    return vring.cb.callback(vring.cb.private);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_config_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t vp_vdpa_config_handler(int irq, void *arg)
    {
    struct vp_vdpa *vp_vdpa = arg;
    if (vp_vdpa.config_cb.callback)
    return vp_vdpa.config_cb.callback(vp_vdpa.config_cb.private);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_request_irq(vp_vdpa: *mut vp_vdpa) -> c_int {
    static int vp_vdpa_request_irq(struct vp_vdpa *vp_vdpa)
    {
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    struct pci_dev *pdev = mdev.pci_dev;
    int i, ret, irq;
    let mut queues: c_int = vp_vdpa.queues;
    let mut vectors: c_int = 1;
    let mut msix_vec: c_int = 0;
    for (i = 0; i < queues; i++) {
    if (vp_vdpa.vring[i].cb.callback)
    vectors++;
    }
    ret = pci_alloc_irq_vectors(pdev, vectors, vectors, PCI_IRQ_MSIX);
    if (ret != vectors) {
    dev_err(&pdev.dev,
    "vp_vdpa: fail to allocate irq vectors want %d but %d\n",
    vectors, ret);
    return ret;
    }
    vp_vdpa.vectors = vectors;
    for (i = 0; i < queues; i++) {
    if (!vp_vdpa.vring[i].cb.callback)
    continue;
    snprintf(vp_vdpa.vring[i].msix_name, VP_VDPA_NAME_SIZE,
    "vp-vdpa[%s]-%d\n", pci_name(pdev), i);
    irq = pci_irq_vector(pdev, msix_vec);
    ret = devm_request_irq(&pdev.dev, irq,
    vp_vdpa_vq_handler,
    0, vp_vdpa.vring[i].msix_name,
    &vp_vdpa.vring[i]);
    if (ret)
    goto err;
    vp_modern_queue_vector(mdev, i, msix_vec);
    vp_vdpa.vring[i].irq = irq;
    msix_vec++;
    }
    snprintf(vp_vdpa.msix_name, VP_VDPA_NAME_SIZE, "vp-vdpa[%s]-config\n",
    pci_name(pdev));
    irq = pci_irq_vector(pdev, msix_vec);
    ret = devm_request_irq(&pdev.dev, irq,	vp_vdpa_config_handler, 0,
    vp_vdpa.msix_name, vp_vdpa);
    if (ret)
    goto err;
    vp_modern_config_vector(mdev, msix_vec);
    vp_vdpa.config_irq = irq;
    return 0;
    err:
    vp_vdpa_free_irq(vp_vdpa);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_set_status(vdpa: *mut vdpa_device, status: u8) {
    static void vp_vdpa_set_status(struct vdpa_device *vdpa, u8 status)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    let mut s: u8 = vp_vdpa_get_status(vdpa);
    if (status & VIRTIO_CONFIG_S_DRIVER_OK &&
    !(s & VIRTIO_CONFIG_S_DRIVER_OK)) {
    if (vp_vdpa_request_irq(vp_vdpa)) {
    WARN_ON(1);
    return;
    }
    }
    vp_modern_set_status(mdev, status);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_reset(vdpa: *mut vdpa_device) -> c_int {
    static int vp_vdpa_reset(struct vdpa_device *vdpa)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    let mut s: u8 = vp_vdpa_get_status(vdpa);
    vp_modern_set_status(mdev, 0);
    if (s & VIRTIO_CONFIG_S_DRIVER_OK)
    vp_vdpa_free_irq(vp_vdpa);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vq_num_max(vdpa: *mut vdpa_device) -> u16 {
    static u16 vp_vdpa_get_vq_num_max(struct vdpa_device *vdpa)
    {
    return VP_VDPA_QUEUE_MAX;
    }
    static int vp_vdpa_get_vq_state(struct vdpa_device *vdpa, u16 qid,
    struct vdpa_vq_state *state)
    {
// Note that this is not supported by virtio specification, so
// we return -EOPNOTSUPP here. This means we can't support live
// migration, vhost device start/stop.
//
    return -EOPNOTSUPP;
    }
    static int vp_vdpa_set_vq_state_split(struct vdpa_device *vdpa,
    const struct vdpa_vq_state *state)
    {
    const struct vdpa_vq_state_split *split = &state.split;
    if (split.avail_index == 0)
    return 0;
    return -EOPNOTSUPP;
    }
    static int vp_vdpa_set_vq_state_packed(struct vdpa_device *vdpa,
    const struct vdpa_vq_state *state)
    {
    const struct vdpa_vq_state_packed *packed = &state.packed;
    if (packed.last_avail_counter == 1 &&
    packed.last_avail_idx == 0 &&
    packed.last_used_counter == 1 &&
    packed.last_used_idx == 0)
    return 0;
    return -EOPNOTSUPP;
    }
    static int vp_vdpa_set_vq_state(struct vdpa_device *vdpa, u16 qid,
    const struct vdpa_vq_state *state)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
// Note that this is not supported by virtio specification.
// But if the state is by chance equal to the device initial
// state, we can let it go.
//
    if ((vp_modern_get_status(mdev) & VIRTIO_CONFIG_S_FEATURES_OK) &&
    !vp_modern_get_queue_enable(mdev, qid)) {
    if (vp_modern_get_driver_features(mdev) &
    BIT_ULL(VIRTIO_F_RING_PACKED))
    return vp_vdpa_set_vq_state_packed(vdpa, state);
    else
    return vp_vdpa_set_vq_state_split(vdpa,	state);
    }
    return -EOPNOTSUPP;
    }
    static void vp_vdpa_set_vq_cb(struct vdpa_device *vdpa, u16 qid,
    struct vdpa_callback *cb)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    vp_vdpa.vring[qid].cb = *cb;
    }
    static void vp_vdpa_set_vq_ready(struct vdpa_device *vdpa,
    u16 qid, bool ready)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    vp_modern_set_queue_enable(mdev, qid, ready);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vq_ready(vdpa: *mut vdpa_device, qid: u16) -> bool {
    static bool vp_vdpa_get_vq_ready(struct vdpa_device *vdpa, u16 qid)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return vp_modern_get_queue_enable(mdev, qid);
    }
    static void vp_vdpa_set_vq_num(struct vdpa_device *vdpa, u16 qid,
    u32 num)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    vp_modern_set_queue_size(mdev, qid, num);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vq_size(vdpa: *mut vdpa_device, qid: u16) -> u16 {
    static u16 vp_vdpa_get_vq_size(struct vdpa_device *vdpa, u16 qid)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return vp_modern_get_queue_size(mdev, qid);
    }
    static int vp_vdpa_set_vq_address(struct vdpa_device *vdpa, u16 qid,
    u64 desc_area, u64 driver_area,
    u64 device_area)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    vp_modern_queue_address(mdev, qid, desc_area,
    driver_area, device_area);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_kick_vq(vdpa: *mut vdpa_device, qid: u16) {
    static void vp_vdpa_kick_vq(struct vdpa_device *vdpa, u16 qid)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    vp_iowrite16(qid, vp_vdpa.vring[qid].notify);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_kick_vq_with_data(vdpa: *mut vdpa_device, data: u32) {
    static void vp_vdpa_kick_vq_with_data(struct vdpa_device *vdpa, u32 data)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    let mut qid: u16 = data & 0xFFFF;
    vp_iowrite32(data, vp_vdpa.vring[qid].notify);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_generation(vdpa: *mut vdpa_device) -> u32 {
    static u32 vp_vdpa_get_generation(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return vp_modern_generation(mdev);
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_device_id(vdpa: *mut vdpa_device) -> u32 {
    static u32 vp_vdpa_get_device_id(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return mdev.id.device;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vendor_id(vdpa: *mut vdpa_device) -> u32 {
    static u32 vp_vdpa_get_vendor_id(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return mdev.id.vendor;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_vq_align(vdpa: *mut vdpa_device) -> u32 {
    static u32 vp_vdpa_get_vq_align(struct vdpa_device *vdpa)
    {
    return PAGE_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_get_config_size(vdpa: *mut vdpa_device) -> usize {
    static size_t vp_vdpa_get_config_size(struct vdpa_device *vdpa)
    {
    struct virtio_pci_modern_device *mdev = vdpa_to_mdev(vdpa);
    return mdev.device_len;
    }
    static void vp_vdpa_get_config(struct vdpa_device *vdpa,
    unsigned int offset,
    void *buf, unsigned int len)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    u8 old, new;
    u8 *p;
    int i;
    do {
    old = vp_ioread8(&mdev.common.config_generation);
    p = buf;
    for (i = 0; i < len; i++)
// p++ = vp_ioread8(mdev->device + offset + i);
    new = vp_ioread8(&mdev.common.config_generation);
    } while (old != new);
    }
    static void vp_vdpa_set_config(struct vdpa_device *vdpa,
    unsigned int offset, const void *buf,
    unsigned int len)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    const u8 *p = buf;
    int i;
    for (i = 0; i < len; i++)
    vp_iowrite8(*p++, mdev.device + offset + i);
    }
    static void vp_vdpa_set_config_cb(struct vdpa_device *vdpa,
    struct vdpa_callback *cb)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    vp_vdpa.config_cb = *cb;
    }
    static struct vdpa_notification_area
    vp_vdpa_get_vq_notification(struct vdpa_device *vdpa, u16 qid)
    {
    struct vp_vdpa *vp_vdpa = vdpa_to_vp(vdpa);
    struct virtio_pci_modern_device *mdev = vp_vdpa_to_mdev(vp_vdpa);
    struct vdpa_notification_area notify;
    notify.addr = vp_vdpa.vring[qid].notify_pa;
    notify.size = mdev.notify_offset_multiplier;
    return notify;
    }
    static const struct vdpa_config_ops vp_vdpa_ops = {
    .get_device_features = vp_vdpa_get_device_features,
    .set_driver_features = vp_vdpa_set_driver_features,
    .get_driver_features = vp_vdpa_get_driver_features,
    .get_status	= vp_vdpa_get_status,
    .set_status	= vp_vdpa_set_status,
    .reset		= vp_vdpa_reset,
    .get_vq_num_max	= vp_vdpa_get_vq_num_max,
    .get_vq_state	= vp_vdpa_get_vq_state,
    .get_vq_notification = vp_vdpa_get_vq_notification,
    .set_vq_state	= vp_vdpa_set_vq_state,
    .set_vq_cb	= vp_vdpa_set_vq_cb,
    .set_vq_ready	= vp_vdpa_set_vq_ready,
    .get_vq_ready	= vp_vdpa_get_vq_ready,
    .set_vq_num	= vp_vdpa_set_vq_num,
    .get_vq_size	= vp_vdpa_get_vq_size,
    .set_vq_address	= vp_vdpa_set_vq_address,
    .kick_vq	= vp_vdpa_kick_vq,
    .kick_vq_with_data      = vp_vdpa_kick_vq_with_data,
    .get_generation	= vp_vdpa_get_generation,
    .get_device_id	= vp_vdpa_get_device_id,
    .get_vendor_id	= vp_vdpa_get_vendor_id,
    .get_vq_align	= vp_vdpa_get_vq_align,
    .get_config_size = vp_vdpa_get_config_size,
    .get_config	= vp_vdpa_get_config,
    .set_config	= vp_vdpa_set_config,
    .set_config_cb  = vp_vdpa_set_config_cb,
    .get_vq_irq	= vp_vdpa_get_vq_irq,
    };
#[no_mangle]
unsafe extern "C" fn vp_vdpa_free_irq_vectors(data: *mut c_void) {
    static void vp_vdpa_free_irq_vectors(void *data)
    {
    pci_free_irq_vectors(data);
    }
    static int vp_vdpa_dev_add(struct vdpa_mgmt_dev *v_mdev, const char *name,
    const struct vdpa_dev_set_config *add_config)
    {
    struct vp_vdpa_mgmtdev *vp_vdpa_mgtdev =
    container_of(v_mdev, struct vp_vdpa_mgmtdev, mgtdev);
    struct virtio_pci_modern_device *mdev = vp_vdpa_mgtdev.mdev;
    struct pci_dev *pdev = mdev.pci_dev;
    struct device *dev = &pdev.dev;
    struct vp_vdpa *vp_vdpa = core::ptr::null_mut();
    u64 device_features;
    int ret, i;
    vp_vdpa = vdpa_alloc_device(struct vp_vdpa, vdpa,
    dev, &vp_vdpa_ops, core::ptr::null_mut(),
    1, 1, name, false);
    if (IS_ERR(vp_vdpa)) {
    dev_err(dev, "vp_vdpa: Failed to allocate vDPA structure\n");
    return PTR_ERR(vp_vdpa);
    }
    vp_vdpa_mgtdev.vp_vdpa = vp_vdpa;
    vp_vdpa.vdpa.vmap.dma_dev = &pdev.dev;
    vp_vdpa.queues = vp_modern_get_num_queues(mdev);
    vp_vdpa.mdev = mdev;
    device_features = vp_modern_get_features(mdev);
    if (add_config.mask & BIT_ULL(VDPA_ATTR_DEV_FEATURES)) {
    if (add_config.device_features & ~device_features) {
    ret = -EINVAL;
    dev_err(&pdev.dev, "Try to provision features "
    "that are not supported by the device: "
    "device_features 0x%llx provisioned 0x%llx\n",
    device_features, add_config.device_features);
    goto err;
    }
    device_features = add_config.device_features;
    }
    vp_vdpa.device_features = device_features;
    ret = devm_add_action_or_reset(dev, vp_vdpa_free_irq_vectors, pdev);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed for adding devres for freeing irq vectors\n");
    goto err;
    }
    vp_vdpa.vring = devm_kcalloc(&pdev.dev, vp_vdpa.queues,
    sizeof(*vp_vdpa.vring),
    GFP_KERNEL);
    if (!vp_vdpa.vring) {
    ret = -ENOMEM;
    dev_err(&pdev.dev, "Fail to allocate virtqueues\n");
    goto err;
    }
    for (i = 0; i < vp_vdpa.queues; i++) {
    vp_vdpa.vring[i].irq = VIRTIO_MSI_NO_VECTOR;
    vp_vdpa.vring[i].notify =
    vp_modern_map_vq_notify(mdev, i,
    &vp_vdpa.vring[i].notify_pa);
    if (!vp_vdpa.vring[i].notify) {
    ret = -EINVAL;
    dev_warn(&pdev.dev, "Fail to map vq notify %d\n", i);
    goto err;
    }
    }
    vp_vdpa.config_irq = VIRTIO_MSI_NO_VECTOR;
    vp_vdpa.vdpa.mdev = &vp_vdpa_mgtdev.mgtdev;
    ret = _vdpa_register_device(&vp_vdpa.vdpa, vp_vdpa.queues);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register to vdpa bus\n");
    goto err;
    }
    return 0;
    err:
    put_device(&vp_vdpa.vdpa.dev);
    return ret;
    }
    static void vp_vdpa_dev_del(struct vdpa_mgmt_dev *v_mdev,
    struct vdpa_device *dev)
    {
    struct vp_vdpa_mgmtdev *vp_vdpa_mgtdev =
    container_of(v_mdev, struct vp_vdpa_mgmtdev, mgtdev);
    struct vp_vdpa *vp_vdpa = vp_vdpa_mgtdev.vp_vdpa;
    _vdpa_unregister_device(&vp_vdpa.vdpa);
    vp_vdpa_mgtdev.vp_vdpa = core::ptr::null_mut();
    }
    static const struct vdpa_mgmtdev_ops vp_vdpa_mdev_ops = {
    .dev_add = vp_vdpa_dev_add,
    .dev_del = vp_vdpa_dev_del,
    };
#[no_mangle]
unsafe extern "C" fn vp_vdpa_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int vp_vdpa_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct vp_vdpa_mgmtdev *vp_vdpa_mgtdev = core::ptr::null_mut();
    struct vdpa_mgmt_dev *mgtdev;
    struct device *dev = &pdev.dev;
    struct virtio_pci_modern_device *mdev = core::ptr::null_mut();
    struct virtio_device_id *mdev_id = core::ptr::null_mut();
    int err;
    vp_vdpa_mgtdev = kzalloc_obj(*vp_vdpa_mgtdev);
    if (!vp_vdpa_mgtdev)
    return -ENOMEM;
    mgtdev = &vp_vdpa_mgtdev.mgtdev;
    mgtdev.ops = &vp_vdpa_mdev_ops;
    mgtdev.device = dev;
    mdev = kzalloc_obj(struct virtio_pci_modern_device);
    if (!mdev) {
    err = -ENOMEM;
    goto mdev_err;
    }
//
// id_table should be a null terminated array, so allocate one additional
// entry here, see vdpa_mgmtdev_get_classes().
//
    mdev_id = kzalloc_objs(struct virtio_device_id, 2);
    if (!mdev_id) {
    err = -ENOMEM;
    goto mdev_id_err;
    }
    vp_vdpa_mgtdev.mdev = mdev;
    mdev.pci_dev = pdev;
    err = pcim_enable_device(pdev);
    if (err) {
    goto probe_err;
    }
    err = vp_modern_probe(mdev);
    if (err) {
    dev_err(&pdev.dev, "Failed to probe modern PCI device\n");
    goto probe_err;
    }
    mdev_id[0].device = mdev.id.device;
    mdev_id[0].vendor = mdev.id.vendor;
    mgtdev.id_table = mdev_id;
    mgtdev.max_supported_vqs = vp_modern_get_num_queues(mdev);
    mgtdev.supported_features = vp_modern_get_features(mdev);
    mgtdev.config_attr_mask = (1 << VDPA_ATTR_DEV_FEATURES);
    pci_set_master(pdev);
    pci_set_drvdata(pdev, vp_vdpa_mgtdev);
    err = vdpa_mgmtdev_register(mgtdev);
    if (err) {
    dev_err(&pdev.dev, "Failed to register vdpa mgmtdev device\n");
    goto register_err;
    }
    return 0;
    register_err:
    vp_modern_remove(vp_vdpa_mgtdev.mdev);
    probe_err:
    kfree(mdev_id);
    mdev_id_err:
    kfree(mdev);
    mdev_err:
    kfree(vp_vdpa_mgtdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vp_vdpa_remove(pdev: *mut pci_dev) {
    static void vp_vdpa_remove(struct pci_dev *pdev)
    {
    struct vp_vdpa_mgmtdev *vp_vdpa_mgtdev = pci_get_drvdata(pdev);
    struct virtio_pci_modern_device *mdev = core::ptr::null_mut();
    mdev = vp_vdpa_mgtdev.mdev;
    vdpa_mgmtdev_unregister(&vp_vdpa_mgtdev.mgtdev);
    vp_modern_remove(mdev);
    kfree(vp_vdpa_mgtdev.mgtdev.id_table);
    kfree(mdev);
    kfree(vp_vdpa_mgtdev);
    }
    static struct pci_driver vp_vdpa_driver = {
    .name		= "vp-vdpa",
    .id_table	= core::ptr::null_mut(), /* only dynamic ids */
    .probe		= vp_vdpa_probe,
    .remove		= vp_vdpa_remove,
    };
    module_pci_driver(vp_vdpa_driver);
    MODULE_AUTHOR("Jason Wang <jasowang@redhat.com>");
    MODULE_DESCRIPTION("vp-vdpa");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1");
