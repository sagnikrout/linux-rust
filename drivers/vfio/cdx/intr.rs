//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/cdx/intr.c
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
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

#[no_mangle]
unsafe extern "C" fn vfio_cdx_msihandler(irq_no: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfio_cdx_msihandler(int irq_no, void *arg)
    {
    struct eventfd_ctx *trigger = arg;
    eventfd_signal(trigger);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vfio_cdx_msi_enable(vdev: *mut vfio_cdx_device, nvec: c_int) -> c_int {
    static int vfio_cdx_msi_enable(struct vfio_cdx_device *vdev, int nvec)
    {
    struct cdx_device *cdx_dev = to_cdx_device(vdev.vdev.dev);
    struct device *dev = vdev.vdev.dev;
    int msi_idx, ret;
    vdev.cdx_irqs = kzalloc_objs(struct vfio_cdx_irq, nvec);
    if (!vdev.cdx_irqs)
    return -ENOMEM;
    ret = cdx_enable_msi(cdx_dev);
    if (ret)
    goto err_free;
// Allocate cdx MSIs
    ret = msi_domain_alloc_irqs(dev, MSI_DEFAULT_DOMAIN, nvec);
    if (ret)
    goto err_disable;
    for (msi_idx = 0; msi_idx < nvec; msi_idx++)
    vdev.cdx_irqs[msi_idx].irq_no = msi_get_virq(dev, msi_idx);
    vdev.msi_count = nvec;
    return 0;
    err_disable:
    cdx_disable_msi(cdx_dev);
    err_free:
    kfree(vdev.cdx_irqs);
    vdev.cdx_irqs = core::ptr::null_mut();
    return ret;
    }
    static int vfio_cdx_msi_set_vector_signal(struct vfio_cdx_device *vdev,
    int vector, int fd)
    {
    struct eventfd_ctx *trigger;
    int irq_no, ret;
    if (vector < 0 || vector >= vdev.msi_count)
    return -EINVAL;
    irq_no = vdev.cdx_irqs[vector].irq_no;
    if (vdev.cdx_irqs[vector].trigger) {
    free_irq(irq_no, vdev.cdx_irqs[vector].trigger);
    kfree(vdev.cdx_irqs[vector].name);
    eventfd_ctx_put(vdev.cdx_irqs[vector].trigger);
    vdev.cdx_irqs[vector].trigger = core::ptr::null_mut();
    }
    if (fd < 0)
    return 0;
    vdev.cdx_irqs[vector].name = kasprintf(GFP_KERNEL, "vfio-msi[%d](%s)",
    vector, dev_name(vdev.vdev.dev));
    if (!vdev.cdx_irqs[vector].name)
    return -ENOMEM;
    trigger = eventfd_ctx_fdget(fd);
    if (IS_ERR(trigger)) {
    kfree(vdev.cdx_irqs[vector].name);
    return PTR_ERR(trigger);
    }
    ret = request_irq(irq_no, vfio_cdx_msihandler, 0,
    vdev.cdx_irqs[vector].name, trigger);
    if (ret) {
    kfree(vdev.cdx_irqs[vector].name);
    eventfd_ctx_put(trigger);
    return ret;
    }
    vdev.cdx_irqs[vector].trigger = trigger;
    return 0;
    }
    static int vfio_cdx_msi_set_block(struct vfio_cdx_device *vdev,
    unsigned int start, unsigned int count,
    int32_t *fds)
    {
    int i, j, ret = 0;
    if (start >= vdev.msi_count || start + count > vdev.msi_count)
    return -EINVAL;
    for (i = 0, j = start; i < count && !ret; i++, j++) {
    let mut fd: c_int = fds ? fds[i] : -1;
    ret = vfio_cdx_msi_set_vector_signal(vdev, j, fd);
    }
    if (ret) {
    for (--j; j >= (int)start; j--)
    vfio_cdx_msi_set_vector_signal(vdev, j, -1);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vfio_cdx_msi_disable(vdev: *mut vfio_cdx_device) {
    static void vfio_cdx_msi_disable(struct vfio_cdx_device *vdev)
    {
    struct cdx_device *cdx_dev = to_cdx_device(vdev.vdev.dev);
    struct device *dev = vdev.vdev.dev;
    vfio_cdx_msi_set_block(vdev, 0, vdev.msi_count, core::ptr::null_mut());
    if (!vdev.cdx_irqs)
    return;
    msi_domain_free_irqs_all(dev, MSI_DEFAULT_DOMAIN);
    cdx_disable_msi(cdx_dev);
    kfree(vdev.cdx_irqs);
    vdev.cdx_irqs = core::ptr::null_mut();
    vdev.msi_count = 0;
    }
    static int vfio_cdx_set_msi_trigger(struct vfio_cdx_device *vdev,
    unsigned int index, unsigned int start,
    unsigned int count, u32 flags,
    void *data)
    {
    struct cdx_device *cdx_dev = to_cdx_device(vdev.vdev.dev);
    int i;
    if (start + count > cdx_dev.num_msi)
    return -EINVAL;
    guard(mutex)(&vdev.cdx_irqs_lock);
    if (!count && (flags & VFIO_IRQ_SET_DATA_NONE)) {
    vfio_cdx_msi_disable(vdev);
    return 0;
    }
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    s32 *fds = data;
    int ret;
    if (vdev.cdx_irqs)
    return vfio_cdx_msi_set_block(vdev, start, count,
    fds);
    ret = vfio_cdx_msi_enable(vdev, cdx_dev.num_msi);
    if (ret)
    return ret;
    ret = vfio_cdx_msi_set_block(vdev, start, count, fds);
    if (ret)
    vfio_cdx_msi_disable(vdev);
    return ret;
    }
    if (!vdev.cdx_irqs)
    return -EINVAL;
    for (i = start; i < start + count; i++) {
    if (!vdev.cdx_irqs[i].trigger)
    continue;
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    eventfd_signal(vdev.cdx_irqs[i].trigger);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    u8 *bools = data;
    if (bools[i - start])
    eventfd_signal(vdev.cdx_irqs[i].trigger);
    }
    }
    return 0;
    }
    int vfio_cdx_set_irqs_ioctl(struct vfio_cdx_device *vdev,
    u32 flags, unsigned int index,
    unsigned int start, unsigned int count,
    void *data)
    {
    if (flags & VFIO_IRQ_SET_ACTION_TRIGGER)
    return vfio_cdx_set_msi_trigger(vdev, index, start,
    count, flags, data);
    else
    return -EINVAL;
    }
// Free All IRQs for the given device
#[no_mangle]
pub unsafe extern "C" fn vfio_cdx_irqs_cleanup(vdev: *mut vfio_cdx_device) {
    void vfio_cdx_irqs_cleanup(struct vfio_cdx_device *vdev)
    {
    vfio_cdx_set_msi_trigger(vdev, 0, 0, 0, VFIO_IRQ_SET_DATA_NONE, core::ptr::null_mut());
    }
