//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/pci/vfio_pci_intrs.c
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
// VFIO PCI interrupt handling
//
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//
// Derived from original vfio:
// Copyright 2010 Cisco Systems, Inc.  All rights reserved.
// Author: Tom Lyon, pugs@cisco.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_irq_ctx {
    pub vdev: *mut vfio_pci_core_device,
    pub trigger: *mut eventfd_ctx,
    pub unmask: *mut virqfd,
    pub mask: *mut virqfd,
    pub name: *mut c_char,
    pub masked: bool,
    pub producer: irq_bypass_producer,
}

#[no_mangle]
unsafe extern "C" fn irq_is(vdev: *mut vfio_pci_core_device, type: c_int) -> bool {
    static bool irq_is(struct vfio_pci_core_device *vdev, int type)
    {
    return vdev.irq_type == type;
    }
#[no_mangle]
unsafe extern "C" fn is_intx(vdev: *mut vfio_pci_core_device) -> bool {
    static bool is_intx(struct vfio_pci_core_device *vdev)
    {
    return vdev.irq_type == VFIO_PCI_INTX_IRQ_INDEX;
    }
#[no_mangle]
unsafe extern "C" fn is_irq_none(vdev: *mut vfio_pci_core_device) -> bool {
    static bool is_irq_none(struct vfio_pci_core_device *vdev)
    {
    return !(vdev.irq_type == VFIO_PCI_INTX_IRQ_INDEX ||
    vdev.irq_type == VFIO_PCI_MSI_IRQ_INDEX ||
    vdev.irq_type == VFIO_PCI_MSIX_IRQ_INDEX);
    }
    static
    struct vfio_pci_irq_ctx *vfio_irq_ctx_get(struct vfio_pci_core_device *vdev,
    unsigned long index)
    {
    return xa_load(&vdev.ctx, index);
    }
    static void vfio_irq_ctx_free(struct vfio_pci_core_device *vdev,
    struct vfio_pci_irq_ctx *ctx, unsigned long index)
    {
    xa_erase(&vdev.ctx, index);
    kfree(ctx);
    }
    static struct vfio_pci_irq_ctx *
    vfio_irq_ctx_alloc(struct vfio_pci_core_device *vdev, unsigned long index)
    {
    struct vfio_pci_irq_ctx *ctx;
    int ret;
    ctx = kzalloc_obj(*ctx, GFP_KERNEL_ACCOUNT);
    if (!ctx)
    return core::ptr::null_mut();
    ret = xa_insert(&vdev.ctx, index, ctx, GFP_KERNEL_ACCOUNT);
    if (ret) {
    kfree(ctx);
    return core::ptr::null_mut();
    }
    return ctx;
    }
//
// INTx
//
#[no_mangle]
unsafe extern "C" fn vfio_send_intx_eventfd(opaque: *mut c_void, data: *mut c_void) {
    static void vfio_send_intx_eventfd(void *opaque, void *data)
    {
    struct vfio_pci_core_device *vdev = opaque;
    if (likely(is_intx(vdev) && !vdev.virq_disabled)) {
    struct vfio_pci_irq_ctx *ctx = data;
    struct eventfd_ctx *trigger = READ_ONCE(ctx.trigger);
    if (likely(trigger))
    eventfd_signal(trigger);
    }
    }
// Returns true if the INTx vfio_pci_irq_ctx.masked value is changed.
#[no_mangle]
unsafe extern "C" fn __vfio_pci_intx_mask(vdev: *mut vfio_pci_core_device) -> bool {
    static bool __vfio_pci_intx_mask(struct vfio_pci_core_device *vdev)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    unsigned long flags;
    let mut masked_changed: bool = false;
    lockdep_assert_held(&vdev.igate);
    spin_lock_irqsave(&vdev.irqlock, flags);
//
// Masking can come from interrupt, ioctl, or config space
// via INTx disable.  The latter means this can get called
// even when not using intx delivery.  In this case, just
// try to have the physical bit follow the virtual bit.
//
    if (unlikely(!is_intx(vdev))) {
    if (vdev.pci_2_3)
    pci_intx(pdev, 0);
    goto out_unlock;
    }
    ctx = vfio_irq_ctx_get(vdev, 0);
    if (WARN_ON_ONCE(!ctx))
    goto out_unlock;
    if (!ctx.masked) {
//
// Can't use check_and_mask here because we always want to
// mask, not just when something is pending.
//
    if (vdev.pci_2_3)
    pci_intx(pdev, 0);
    else
    disable_irq_nosync(pdev.irq);
    ctx.masked = true;
    masked_changed = true;
    }
    out_unlock:
    spin_unlock_irqrestore(&vdev.irqlock, flags);
    return masked_changed;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_pci_intx_mask(vdev: *mut vfio_pci_core_device) -> bool {
    bool vfio_pci_intx_mask(struct vfio_pci_core_device *vdev)
    {
    bool mask_changed;
    mutex_lock(&vdev.igate);
    mask_changed = __vfio_pci_intx_mask(vdev);
    mutex_unlock(&vdev.igate);
    return mask_changed;
    }
//
// If this is triggered by an eventfd, we can't call eventfd_signal
// or else we'll deadlock on the eventfd wait queue.  Return >0 when
// a signal is necessary, which can then be handled via a work queue
// or directly depending on the caller.
//
#[no_mangle]
unsafe extern "C" fn vfio_pci_intx_unmask_handler(opaque: *mut c_void, data: *mut c_void) -> c_int {
    static int vfio_pci_intx_unmask_handler(void *opaque, void *data)
    {
    struct vfio_pci_core_device *vdev = opaque;
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx = data;
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&vdev.irqlock, flags);
//
// Unmasking comes from ioctl or config, so again, have the
// physical bit follow the virtual even when not using INTx.
//
    if (unlikely(!is_intx(vdev))) {
    if (vdev.pci_2_3)
    pci_intx(pdev, 1);
    goto out_unlock;
    }
    if (ctx.masked && !vdev.virq_disabled) {
//
// A pending interrupt here would immediately trigger,
// but we can avoid that overhead by just re-sending
// the interrupt to the user.
//
    if (vdev.pci_2_3) {
    if (!pci_check_and_unmask_intx(pdev))
    ret = 1;
    } else
    enable_irq(pdev.irq);
    ctx.masked = (ret > 0);
    }
    out_unlock:
    spin_unlock_irqrestore(&vdev.irqlock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __vfio_pci_intx_unmask(vdev: *mut vfio_pci_core_device) {
    static void __vfio_pci_intx_unmask(struct vfio_pci_core_device *vdev)
    {
    struct vfio_pci_irq_ctx *ctx = vfio_irq_ctx_get(vdev, 0);
    lockdep_assert_held(&vdev.igate);
    if (vfio_pci_intx_unmask_handler(vdev, ctx) > 0)
    vfio_send_intx_eventfd(vdev, ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_pci_intx_unmask(vdev: *mut vfio_pci_core_device) {
    void vfio_pci_intx_unmask(struct vfio_pci_core_device *vdev)
    {
    mutex_lock(&vdev.igate);
    __vfio_pci_intx_unmask(vdev);
    mutex_unlock(&vdev.igate);
    }
#[no_mangle]
unsafe extern "C" fn vfio_intx_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfio_intx_handler(int irq, void *dev_id)
    {
    struct vfio_pci_irq_ctx *ctx = dev_id;
    struct vfio_pci_core_device *vdev = ctx.vdev;
    unsigned long flags;
    let mut ret: c_int = IRQ_NONE;
    spin_lock_irqsave(&vdev.irqlock, flags);
    if (!vdev.pci_2_3) {
    disable_irq_nosync(vdev.pdev.irq);
    ctx.masked = true;
    ret = IRQ_HANDLED;
    } else if (!ctx.masked &&  /* may be shared */
    pci_check_and_mask_intx(vdev.pdev)) {
    ctx.masked = true;
    ret = IRQ_HANDLED;
    }
    spin_unlock_irqrestore(&vdev.irqlock, flags);
    if (ret == IRQ_HANDLED)
    vfio_send_intx_eventfd(vdev, ctx);
    return ret;
    }
    static int vfio_intx_enable(struct vfio_pci_core_device *vdev,
    struct eventfd_ctx *trigger)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    unsigned long irqflags;
    char *name;
    int ret;
    if (!is_irq_none(vdev))
    return -EINVAL;
    if (!pdev.irq || pdev.irq == IRQ_NOTCONNECTED)
    return -ENODEV;
    name = kasprintf(GFP_KERNEL_ACCOUNT, "vfio-intx(%s)", pci_name(pdev));
    if (!name)
    return -ENOMEM;
    ctx = vfio_irq_ctx_alloc(vdev, 0);
    if (!ctx) {
    kfree(name);
    return -ENOMEM;
    }
    ctx.name = name;
    ctx.trigger = trigger;
    ctx.vdev = vdev;
//
// Fill the initial masked state based on virq_disabled.  After
// enable, changing the DisINTx bit in vconfig directly changes INTx
// masking.  igate prevents races during setup, once running masked
// is protected via irqlock.
//
// Devices supporting DisINTx also reflect the current mask state in
// the physical DisINTx bit, which is not affected during IRQ setup.
//
// Devices without DisINTx support require an exclusive interrupt.
// IRQ masking is performed at the IRQ chip.  Again, igate protects
// against races during setup and IRQ handlers and irqfds are not
// yet active, therefore masked is stable and can be used to
// conditionally auto-enable the IRQ.
//
// irq_type must be stable while the IRQ handler is registered,
// therefore it must be set before request_irq().
//
    ctx.masked = vdev.virq_disabled;
    if (vdev.pci_2_3) {
    pci_intx(pdev, !ctx.masked);
    irqflags = IRQF_SHARED;
    } else {
    irqflags = ctx.masked ? IRQF_NO_AUTOEN : 0;
    }
    vdev.irq_type = VFIO_PCI_INTX_IRQ_INDEX;
    if (!vdev.pci_2_3)
    irq_set_status_flags(pdev.irq, IRQ_DISABLE_UNLAZY);
    ret = request_irq(pdev.irq, vfio_intx_handler,
    irqflags, ctx.name, ctx);
    if (ret) {
    if (!vdev.pci_2_3)
    irq_clear_status_flags(pdev.irq, IRQ_DISABLE_UNLAZY);
    vdev.irq_type = VFIO_PCI_NUM_IRQS;
    kfree(name);
    vfio_irq_ctx_free(vdev, ctx, 0);
    return ret;
    }
    return 0;
    }
    static int vfio_intx_set_signal(struct vfio_pci_core_device *vdev,
    struct eventfd_ctx *trigger)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    struct eventfd_ctx *old;
    ctx = vfio_irq_ctx_get(vdev, 0);
    if (WARN_ON_ONCE(!ctx))
    return -EINVAL;
    old = ctx.trigger;
    WRITE_ONCE(ctx.trigger, trigger);
// Releasing an old ctx requires synchronizing in-flight users
    if (old) {
    synchronize_irq(pdev.irq);
    vfio_virqfd_flush_thread(&ctx.unmask);
    eventfd_ctx_put(old);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfio_intx_disable(vdev: *mut vfio_pci_core_device) {
    static void vfio_intx_disable(struct vfio_pci_core_device *vdev)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    ctx = vfio_irq_ctx_get(vdev, 0);
    WARN_ON_ONCE(!ctx);
    if (ctx) {
    vfio_virqfd_disable(&ctx.unmask);
    vfio_virqfd_disable(&ctx.mask);
    free_irq(pdev.irq, ctx);
    if (!vdev.pci_2_3)
    irq_clear_status_flags(pdev.irq, IRQ_DISABLE_UNLAZY);
    if (ctx.trigger)
    eventfd_ctx_put(ctx.trigger);
    kfree(ctx.name);
    vfio_irq_ctx_free(vdev, ctx, 0);
    }
    vdev.irq_type = VFIO_PCI_NUM_IRQS;
    }
//
// MSI/MSI-X
//
#[no_mangle]
unsafe extern "C" fn vfio_msihandler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfio_msihandler(int irq, void *arg)
    {
    struct eventfd_ctx *trigger = arg;
    eventfd_signal(trigger);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vfio_msi_enable(vdev: *mut vfio_pci_core_device, nvec: c_int, msix: bool) -> c_int {
    static int vfio_msi_enable(struct vfio_pci_core_device *vdev, int nvec, bool msix)
    {
    struct pci_dev *pdev = vdev.pdev;
    let mut flag: c_uint = msix ? PCI_IRQ_MSIX : PCI_IRQ_MSI;
    int ret;
    u16 cmd;
    if (!is_irq_none(vdev))
    return -EINVAL;
// return the number of supported vectors if we can't get all:
    cmd = vfio_pci_memory_lock_and_enable(vdev);
    ret = pci_alloc_irq_vectors(pdev, 1, nvec, flag);
    if (ret < nvec) {
    if (ret > 0)
    pci_free_irq_vectors(pdev);
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
    return ret;
    }
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
    vdev.irq_type = msix ? VFIO_PCI_MSIX_IRQ_INDEX :
    VFIO_PCI_MSI_IRQ_INDEX;
    if (!msix) {
//
// Compute the virtual hardware field for max msi vectors -
// it is the log base 2 of the number of vectors.
//
    vdev.msi_qmax = fls(nvec * 2 - 1) - 1;
    }
    return 0;
    }
//
// vfio_msi_alloc_irq() returns the Linux IRQ number of an MSI or MSI-X device
// interrupt vector. If a Linux IRQ number is not available then a new
// interrupt is allocated if dynamic MSI-X is supported.
//
// Where is vfio_msi_free_irq()? Allocated interrupts are maintained,
// essentially forming a cache that subsequent allocations can draw from.
// Interrupts are freed using pci_free_irq_vectors() when MSI/MSI-X is
// disabled.
//
    static int vfio_msi_alloc_irq(struct vfio_pci_core_device *vdev,
    unsigned int vector, bool msix)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct msi_map map;
    int irq;
    u16 cmd;
    irq = pci_irq_vector(pdev, vector);
    if (WARN_ON_ONCE(irq == 0))
    return -EINVAL;
    if (irq > 0 || !msix || !vdev.has_dyn_msix)
    return irq;
    cmd = vfio_pci_memory_lock_and_enable(vdev);
    map = pci_msix_alloc_irq_at(pdev, vector, core::ptr::null_mut());
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
    return map.index < 0 ? map.index : map.virq;
    }
    static int vfio_msi_set_vector_signal(struct vfio_pci_core_device *vdev,
    unsigned int vector, int fd, bool msix)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    struct eventfd_ctx *trigger;
    let mut irq: c_int = -EINVAL, ret;
    u16 cmd;
    ctx = vfio_irq_ctx_get(vdev, vector);
    if (ctx) {
    irq_bypass_unregister_producer(&ctx.producer);
    irq = pci_irq_vector(pdev, vector);
    cmd = vfio_pci_memory_lock_and_enable(vdev);
    free_irq(irq, ctx.trigger);
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
// Interrupt stays allocated, will be freed at MSI-X disable.
    kfree(ctx.name);
    eventfd_ctx_put(ctx.trigger);
    vfio_irq_ctx_free(vdev, ctx, vector);
    }
    if (fd < 0)
    return 0;
    if (irq == -EINVAL) {
// Interrupt stays allocated, will be freed at MSI-X disable.
    irq = vfio_msi_alloc_irq(vdev, vector, msix);
    if (irq < 0)
    return irq;
    }
    ctx = vfio_irq_ctx_alloc(vdev, vector);
    if (!ctx)
    return -ENOMEM;
    ctx.name = kasprintf(GFP_KERNEL_ACCOUNT, "vfio-msi%s[%d](%s)",
    msix ? "x" : "", vector, pci_name(pdev));
    if (!ctx.name) {
    ret = -ENOMEM;
    goto out_free_ctx;
    }
    trigger = eventfd_ctx_fdget(fd);
    if (IS_ERR(trigger)) {
    ret = PTR_ERR(trigger);
    goto out_free_name;
    }
//
// If the vector was previously allocated, refresh the on-device
// message data before enabling in case it had been cleared or
// corrupted (e.g. due to backdoor resets) since writing.
//
    cmd = vfio_pci_memory_lock_and_enable(vdev);
    if (msix) {
    struct msi_msg msg;
    get_cached_msi_msg(irq, &msg);
    pci_write_msi_msg(irq, &msg);
    }
    ret = request_irq(irq, vfio_msihandler, 0, ctx.name, trigger);
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
    if (ret)
    goto out_put_eventfd_ctx;
    ret = irq_bypass_register_producer(&ctx.producer, trigger, irq);
    if (unlikely(ret)) {
    dev_info(&pdev.dev,
    "irq bypass producer (eventfd %p) registration fails: %d\n",
    trigger, ret);
    }
    ctx.trigger = trigger;
    return 0;
    out_put_eventfd_ctx:
    eventfd_ctx_put(trigger);
    out_free_name:
    kfree(ctx.name);
    out_free_ctx:
    vfio_irq_ctx_free(vdev, ctx, vector);
    return ret;
    }
    static int vfio_msi_set_block(struct vfio_pci_core_device *vdev, unsigned start,
    unsigned count, int32_t *fds, bool msix)
    {
    unsigned int i, j;
    let mut ret: c_int = 0;
    for (i = 0, j = start; i < count && !ret; i++, j++) {
    let mut fd: c_int = fds ? fds[i] : -1;
    ret = vfio_msi_set_vector_signal(vdev, j, fd, msix);
    }
    if (ret) {
    for (i = start; i < j; i++)
    vfio_msi_set_vector_signal(vdev, i, -1, msix);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vfio_msi_disable(vdev: *mut vfio_pci_core_device, msix: bool) {
    static void vfio_msi_disable(struct vfio_pci_core_device *vdev, bool msix)
    {
    struct pci_dev *pdev = vdev.pdev;
    struct vfio_pci_irq_ctx *ctx;
    unsigned long i;
    u16 cmd;
    xa_for_each(&vdev.ctx, i, ctx) {
    vfio_virqfd_disable(&ctx.unmask);
    vfio_virqfd_disable(&ctx.mask);
    vfio_msi_set_vector_signal(vdev, i, -1, msix);
    }
    cmd = vfio_pci_memory_lock_and_enable(vdev);
    pci_free_irq_vectors(pdev);
    vfio_pci_memory_unlock_and_restore(vdev, cmd);
//
// Both disable paths above use pci_intx_for_msi() to clear DisINTx
// via their shutdown paths.  Restore for NoINTx devices.
//
    if (vdev.nointx)
    pci_intx(pdev, 0);
    vdev.irq_type = VFIO_PCI_NUM_IRQS;
    }
//
// IOCTL support
//
    static int vfio_pci_set_intx_unmask(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    if (!is_intx(vdev) || start != 0 || count != 1)
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    __vfio_pci_intx_unmask(vdev);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut unmask: u8 = *(uint8_t *)data;
    if (unmask)
    __vfio_pci_intx_unmask(vdev);
    } else if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    struct vfio_pci_irq_ctx *ctx = vfio_irq_ctx_get(vdev, 0);
    let mut fd: i32 = *(int32_t *)data;
    if (WARN_ON_ONCE(!ctx))
    return -EINVAL;
    if (fd >= 0)
    return vfio_virqfd_enable((void *) vdev,
    vfio_pci_intx_unmask_handler,
    vfio_send_intx_eventfd, ctx,
    &ctx.unmask, fd);
    vfio_virqfd_disable(&ctx.unmask);
    }
    return 0;
    }
    static int vfio_pci_set_intx_mask(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    if (!is_intx(vdev) || start != 0 || count != 1)
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    __vfio_pci_intx_mask(vdev);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut mask: u8 = *(uint8_t *)data;
    if (mask)
    __vfio_pci_intx_mask(vdev);
    } else if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    return -ENOTTY; /* XXX implement me */
    }
    return 0;
    }
    static int vfio_pci_set_intx_trigger(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    if (is_intx(vdev) && !count && (flags & VFIO_IRQ_SET_DATA_NONE)) {
    vfio_intx_disable(vdev);
    return 0;
    }
    if (!(is_intx(vdev) || is_irq_none(vdev)) || start != 0 || count != 1)
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    struct eventfd_ctx *trigger = core::ptr::null_mut();
    let mut fd: i32 = *(int32_t *)data;
    int ret;
    if (fd >= 0) {
    trigger = eventfd_ctx_fdget(fd);
    if (IS_ERR(trigger))
    return PTR_ERR(trigger);
    }
    if (is_intx(vdev))
    ret = vfio_intx_set_signal(vdev, trigger);
    else
    ret = vfio_intx_enable(vdev, trigger);
    if (ret && trigger)
    eventfd_ctx_put(trigger);
    return ret;
    }
    if (!is_intx(vdev))
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    vfio_send_intx_eventfd(vdev, vfio_irq_ctx_get(vdev, 0));
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    let mut trigger: u8 = *(uint8_t *)data;
    if (trigger)
    vfio_send_intx_eventfd(vdev, vfio_irq_ctx_get(vdev, 0));
    }
    return 0;
    }
    static int vfio_pci_set_msi_trigger(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    struct vfio_pci_irq_ctx *ctx;
    unsigned int i;
    let mut msix: bool = (index == VFIO_PCI_MSIX_IRQ_INDEX);
    if (irq_is(vdev, index) && !count && (flags & VFIO_IRQ_SET_DATA_NONE)) {
    vfio_msi_disable(vdev, msix);
    return 0;
    }
    if (!(irq_is(vdev, index) || is_irq_none(vdev)))
    return -EINVAL;
    if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    int32_t *fds = data;
    int ret;
    if (vdev.irq_type == index)
    return vfio_msi_set_block(vdev, start, count,
    fds, msix);
    ret = vfio_msi_enable(vdev, start + count, msix);
    if (ret)
    return ret;
    ret = vfio_msi_set_block(vdev, start, count, fds, msix);
    if (ret)
    vfio_msi_disable(vdev, msix);
    return ret;
    }
    if (!irq_is(vdev, index))
    return -EINVAL;
    for (i = start; i < start + count; i++) {
    ctx = vfio_irq_ctx_get(vdev, i);
    if (!ctx)
    continue;
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    eventfd_signal(ctx.trigger);
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    uint8_t *bools = data;
    if (bools[i - start])
    eventfd_signal(ctx.trigger);
    }
    }
    return 0;
    }
    static int vfio_pci_set_ctx_trigger_single(struct vfio_pci_core_device *vdev,
    struct vfio_pci_eventfd __rcu **peventfd,
    unsigned int count, uint32_t flags,
    void *data)
    {
// DATA_NONE/DATA_BOOL enables loopback testing
    if (flags & VFIO_IRQ_SET_DATA_NONE) {
    struct vfio_pci_eventfd *eventfd;
    eventfd = rcu_dereference_protected(*peventfd,
    lockdep_is_held(&vdev.igate));
    if (!eventfd)
    return -EINVAL;
    if (count) {
    eventfd_signal(eventfd.ctx);
    return 0;
    }
    return vfio_pci_eventfd_replace_locked(vdev, peventfd, core::ptr::null_mut());
    } else if (flags & VFIO_IRQ_SET_DATA_BOOL) {
    uint8_t trigger;
    if (!count)
    return -EINVAL;
    trigger = *(uint8_t *)data;
    if (trigger) {
    struct vfio_pci_eventfd *eventfd =
    rcu_dereference_protected(*peventfd,
    lockdep_is_held(&vdev.igate));
    if (eventfd)
    eventfd_signal(eventfd.ctx);
    }
    return 0;
    } else if (flags & VFIO_IRQ_SET_DATA_EVENTFD) {
    int32_t fd;
    if (!count)
    return -EINVAL;
    fd = *(int32_t *)data;
    if (fd == -1) {
    return vfio_pci_eventfd_replace_locked(vdev,
    peventfd, core::ptr::null_mut());
    } else if (fd >= 0) {
    struct eventfd_ctx *efdctx;
    int ret;
    efdctx = eventfd_ctx_fdget(fd);
    if (IS_ERR(efdctx))
    return PTR_ERR(efdctx);
    ret = vfio_pci_eventfd_replace_locked(vdev,
    peventfd, efdctx);
    if (ret)
    eventfd_ctx_put(efdctx);
    return ret;
    }
    }
    return -EINVAL;
    }
    static int vfio_pci_set_err_trigger(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    if (index != VFIO_PCI_ERR_IRQ_INDEX || start != 0 || count > 1)
    return -EINVAL;
    return vfio_pci_set_ctx_trigger_single(vdev, &vdev.err_trigger,
    count, flags, data);
    }
    static int vfio_pci_set_req_trigger(struct vfio_pci_core_device *vdev,
    unsigned index, unsigned start,
    unsigned count, uint32_t flags, void *data)
    {
    if (index != VFIO_PCI_REQ_IRQ_INDEX || start != 0 || count > 1)
    return -EINVAL;
    return vfio_pci_set_ctx_trigger_single(vdev, &vdev.req_trigger,
    count, flags, data);
    }
    int vfio_pci_set_irqs_ioctl(struct vfio_pci_core_device *vdev, uint32_t flags,
    unsigned index, unsigned start, unsigned count,
    void *data)
    {
    int (*func)(struct vfio_pci_core_device *vdev, unsigned index,
    unsigned start, unsigned count, uint32_t flags,
    void *data) = core::ptr::null_mut();
    switch (index) {
    case VFIO_PCI_INTX_IRQ_INDEX:
    switch (flags & VFIO_IRQ_SET_ACTION_TYPE_MASK) {
    case VFIO_IRQ_SET_ACTION_MASK:
    func = vfio_pci_set_intx_mask;
    break;
    case VFIO_IRQ_SET_ACTION_UNMASK:
    func = vfio_pci_set_intx_unmask;
    break;
    case VFIO_IRQ_SET_ACTION_TRIGGER:
    func = vfio_pci_set_intx_trigger;
    break;
    }
    break;
    case VFIO_PCI_MSI_IRQ_INDEX:
    case VFIO_PCI_MSIX_IRQ_INDEX:
    switch (flags & VFIO_IRQ_SET_ACTION_TYPE_MASK) {
    case VFIO_IRQ_SET_ACTION_MASK:
    case VFIO_IRQ_SET_ACTION_UNMASK:
// XXX Need masking support exported
    break;
    case VFIO_IRQ_SET_ACTION_TRIGGER:
    func = vfio_pci_set_msi_trigger;
    break;
    }
    break;
    case VFIO_PCI_ERR_IRQ_INDEX:
    switch (flags & VFIO_IRQ_SET_ACTION_TYPE_MASK) {
    case VFIO_IRQ_SET_ACTION_TRIGGER:
    func = vfio_pci_set_err_trigger;
    break;
    }
    break;
    case VFIO_PCI_REQ_IRQ_INDEX:
    switch (flags & VFIO_IRQ_SET_ACTION_TYPE_MASK) {
    case VFIO_IRQ_SET_ACTION_TRIGGER:
    func = vfio_pci_set_req_trigger;
    break;
    }
    break;
    }
    if (!func)
    return -ENOTTY;
    return func(vdev, index, start, count, flags, data);
    }
