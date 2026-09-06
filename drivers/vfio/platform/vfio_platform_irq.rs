//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/platform/vfio_platform_irq.c
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
// VFIO platform devices interrupt handling
//
// Copyright (C) 2013 - Virtual Open Systems
// Author: Antonios Motakis <a.motakis@virtualopensystems.com>
//

#[no_mangle]
unsafe extern "C" fn vfio_platform_mask(irq_ctx: *mut vfio_platform_irq) {
    static void vfio_platform_mask(struct vfio_platform_irq *irq_ctx)
    {
    unsigned long flags;
    spin_lock_irqsave(&irq_ctx.lock, flags);
    if (!irq_ctx.masked) {
    disable_irq_nosync(irq_ctx.hwirq);
    irq_ctx.masked = true;
    }
    spin_unlock_irqrestore(&irq_ctx.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn vfio_platform_mask_handler(opaque: *mut c_void, unused: *mut c_void) -> c_int {
    static int vfio_platform_mask_handler(void *opaque, void *unused)
    {
    struct vfio_platform_irq *irq_ctx = opaque;
    vfio_platform_mask(irq_ctx);
    return 0;
    }
    static int vfio_platform_set_irq_mask(struct vfio_platform_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags,
    void *data)
    {
    if (start != 0 || count != 1)
    return -EINVAL;
    if (!(vdev.irqs[index].flags & VFIO_IRQ_INFO_MASKABLE))
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    let mut fd: i32 = *(int32_t *)data;
    if (fd >= 0)
    return vfio_virqfd_enable((void *) &vdev.irqs[index],
    vfio_platform_mask_handler,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &vdev.irqs[index].mask, fd);
    vfio_virqfd_disable(&vdev.irqs[index].mask);
    return 0;
    }
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    vfio_platform_mask(&vdev.irqs[index]);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut mask: u8 = *(uint8_t *)data;
    if (mask)
    vfio_platform_mask(&vdev.irqs[index]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfio_platform_unmask(irq_ctx: *mut vfio_platform_irq) {
    static void vfio_platform_unmask(struct vfio_platform_irq *irq_ctx)
    {
    unsigned long flags;
    spin_lock_irqsave(&irq_ctx.lock, flags);
    if (irq_ctx.masked) {
    enable_irq(irq_ctx.hwirq);
    irq_ctx.masked = false;
    }
    spin_unlock_irqrestore(&irq_ctx.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn vfio_platform_unmask_handler(opaque: *mut c_void, unused: *mut c_void) -> c_int {
    static int vfio_platform_unmask_handler(void *opaque, void *unused)
    {
    struct vfio_platform_irq *irq_ctx = opaque;
    vfio_platform_unmask(irq_ctx);
    return 0;
    }
    static int vfio_platform_set_irq_unmask(struct vfio_platform_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags,
    void *data)
    {
    if (start != 0 || count != 1)
    return -EINVAL;
    if (!(vdev.irqs[index].flags & VFIO_IRQ_INFO_MASKABLE))
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    let mut fd: i32 = *(int32_t *)data;
    if (fd >= 0)
    return vfio_virqfd_enable((void *) &vdev.irqs[index],
    vfio_platform_unmask_handler,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &vdev.irqs[index].unmask,
    fd);
    vfio_virqfd_disable(&vdev.irqs[index].unmask);
    return 0;
    }
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    vfio_platform_unmask(&vdev.irqs[index]);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut unmask: u8 = *(uint8_t *)data;
    if (unmask)
    vfio_platform_unmask(&vdev.irqs[index]);
    }
    return 0;
    }
//
// The trigger eventfd is guaranteed valid in the interrupt path
// and protected by the igate mutex when triggered via ioctl.
//
#[no_mangle]
unsafe extern "C" fn vfio_send_eventfd(irq_ctx: *mut vfio_platform_irq) {
    static void vfio_send_eventfd(struct vfio_platform_irq *irq_ctx)
    {
    if (likely(irq_ctx.trigger))
    eventfd_signal(irq_ctx.trigger);
    }
#[no_mangle]
unsafe extern "C" fn vfio_automasked_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfio_automasked_irq_handler(int irq, void *dev_id)
    {
    struct vfio_platform_irq *irq_ctx = dev_id;
    unsigned long flags;
    let mut ret: c_int = IRQ_NONE;
    spin_lock_irqsave(&irq_ctx.lock, flags);
    if (!irq_ctx.masked) {
    ret = IRQ_HANDLED;
// automask maskable interrupts
    disable_irq_nosync(irq_ctx.hwirq);
    irq_ctx.masked = true;
    }
    spin_unlock_irqrestore(&irq_ctx.lock, flags);
    if (ret == IRQ_HANDLED)
    vfio_send_eventfd(irq_ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vfio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfio_irq_handler(int irq, void *dev_id)
    {
    struct vfio_platform_irq *irq_ctx = dev_id;
    vfio_send_eventfd(irq_ctx);
    return IRQ_HANDLED;
    }
    static int vfio_set_trigger(struct vfio_platform_device *vdev, int index,
    int fd)
    {
    struct vfio_platform_irq *irq = &vdev.irqs[index];
    struct eventfd_ctx *trigger;
    if (irq.trigger) {
    disable_irq(irq.hwirq);
    eventfd_ctx_put(irq.trigger);
    irq.trigger = core::ptr::null_mut();
    }
    if (fd < 0) /* Disable only */
    return 0;
    trigger = eventfd_ctx_fdget(fd);
    if (IS_ERR(trigger))
    return PTR_ERR(trigger);
    irq.trigger = trigger;
//
// irq->masked effectively provides nested disables within the overall
// enable relative to trigger.  Specifically request_irq() is called
// with NO_AUTOEN, therefore the IRQ is initially disabled.  The user
// may only further disable the IRQ with a MASK operations because
// irq->masked is initially false.
//
    enable_irq(irq.hwirq);
    return 0;
    }
    static int vfio_platform_set_irq_trigger(struct vfio_platform_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags,
    void *data)
    {
    struct vfio_platform_irq *irq = &vdev.irqs[index];
    irq_handler_t handler;
    if (vdev.irqs[index].flags & VFIO_IRQ_INFO_AUTOMASKED)
    handler = vfio_automasked_irq_handler;
    else
    handler = vfio_irq_handler;
    if (!count && (flags & VFIO_IRQ_SET_DATA_NONE))
    return vfio_set_trigger(vdev, index, -1);
    if (start != 0 || count != 1)
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    let mut fd: i32 = *(int32_t *)data;
    return vfio_set_trigger(vdev, index, fd);
    }
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    handler(irq.hwirq, irq);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut trigger: u8 = *(uint8_t *)data;
    if (trigger)
    handler(irq.hwirq, irq);
    }
    return 0;
    }
    int vfio_platform_set_irqs_ioctl(struct vfio_platform_device *vdev,
    uint32_t flags, unsigned index, unsigned start,
    unsigned count, void *data)
    {
    int (*func)(struct vfio_platform_device *vdev, unsigned index,
    unsigned start, unsigned count, uint32_t flags,
    void *data) = core::ptr::null_mut();
//
// For compatibility, errors from request_irq() are local to the
// SET_IRQS path and reflected in the name pointer.  This allows,
// for example, polling mode fallback for an exclusive IRQ failure.
//
    if (IS_ERR(vdev.irqs[index].name))
    return PTR_ERR(vdev.irqs[index].name);
    switch (flags & VFIO_IRQ_SET_ACTION_TYPE_MASK) {
    case VFIO_IRQ_SET_ACTION_MASK:
    func = vfio_platform_set_irq_mask;
    break;
    case VFIO_IRQ_SET_ACTION_UNMASK:
    func = vfio_platform_set_irq_unmask;
    break;
    case VFIO_IRQ_SET_ACTION_TRIGGER:
    func = vfio_platform_set_irq_trigger;
    break;
    }
    if (!func)
    return -ENOTTY;
    return func(vdev, index, start, count, flags, data);
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_platform_irq_init(vdev: *mut vfio_platform_device) -> c_int {
    int vfio_platform_irq_init(struct vfio_platform_device *vdev)
    {
    let mut cnt: c_int = 0, i, ret = 0;
    while (vdev.get_irq(vdev, cnt) >= 0)
    cnt++;
    vdev.irqs = kzalloc_objs(struct vfio_platform_irq, cnt,
    GFP_KERNEL_ACCOUNT);
    if (!vdev.irqs)
    return -ENOMEM;
    for (i = 0; i < cnt; i++) {
    let mut hwirq: c_int = vdev.get_irq(vdev, i);
    let mut handler: irq_handler_t = vfio_irq_handler;
    if (hwirq < 0) {
    ret = -EINVAL;
    goto err;
    }
    spin_lock_init(&vdev.irqs[i].lock);
    vdev.irqs[i].flags = VFIO_IRQ_INFO_EVENTFD;
    if (irq_get_trigger_type(hwirq) & IRQ_TYPE_LEVEL_MASK) {
    vdev.irqs[i].flags |= VFIO_IRQ_INFO_MASKABLE
    | VFIO_IRQ_INFO_AUTOMASKED;
    handler = vfio_automasked_irq_handler;
    }
    vdev.irqs[i].count = 1;
    vdev.irqs[i].hwirq = hwirq;
    vdev.irqs[i].masked = false;
    vdev.irqs[i].name = kasprintf(GFP_KERNEL_ACCOUNT,
    "vfio-irq[%d](%s)", hwirq,
    vdev.name);
    if (!vdev.irqs[i].name) {
    ret = -ENOMEM;
    goto err;
    }
    ret = request_irq(hwirq, handler, IRQF_NO_AUTOEN,
    vdev.irqs[i].name, &vdev.irqs[i]);
    if (ret) {
    kfree(vdev.irqs[i].name);
    vdev.irqs[i].name = ERR_PTR(ret);
    }
    }
    vdev.num_irqs = cnt;
    return 0;
    err:
    for (--i; i >= 0; i--) {
    if (!IS_ERR(vdev.irqs[i].name)) {
    free_irq(vdev.irqs[i].hwirq, &vdev.irqs[i]);
    kfree(vdev.irqs[i].name);
    }
    }
    kfree(vdev.irqs);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_platform_irq_cleanup(vdev: *mut vfio_platform_device) {
    void vfio_platform_irq_cleanup(struct vfio_platform_device *vdev)
    {
    int i;
    for (i = 0; i < vdev.num_irqs; i++) {
    vfio_virqfd_disable(&vdev.irqs[i].mask);
    vfio_virqfd_disable(&vdev.irqs[i].unmask);
    if (!IS_ERR(vdev.irqs[i].name)) {
    free_irq(vdev.irqs[i].hwirq, &vdev.irqs[i]);
    if (vdev.irqs[i].trigger)
    eventfd_ctx_put(vdev.irqs[i].trigger);
    kfree(vdev.irqs[i].name);
    }
    }
    vdev.num_irqs = 0;
    kfree(vdev.irqs);
    }
