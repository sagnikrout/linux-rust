//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/virqfd.c
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
// VFIO generic eventfd code for IRQFD support.
// Derived from drivers/vfio/pci/vfio_pci_intrs.c
//
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//

    static struct workqueue_struct *vfio_irqfd_cleanup_wq;
    static DEFINE_SPINLOCK(virqfd_lock);
#[no_mangle]
pub unsafe extern "C" fn vfio_virqfd_init() -> int __init {
    int __init vfio_virqfd_init(void)
    {
    vfio_irqfd_cleanup_wq =
    create_singlethread_workqueue("vfio-irqfd-cleanup");
    if (!vfio_irqfd_cleanup_wq)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_virqfd_exit() {
    void vfio_virqfd_exit(void)
    {
    destroy_workqueue(vfio_irqfd_cleanup_wq);
    }
#[no_mangle]
unsafe extern "C" fn virqfd_deactivate(virqfd: *mut virqfd) {
    static void virqfd_deactivate(struct virqfd *virqfd)
    {
    queue_work(vfio_irqfd_cleanup_wq, &virqfd.shutdown);
    }
#[no_mangle]
unsafe extern "C" fn virqfd_wakeup(wait: *mut wait_queue_entry_t, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int {
    static int virqfd_wakeup(wait_queue_entry_t *wait, unsigned mode, int sync, void *key)
    {
    struct virqfd *virqfd = container_of(wait, struct virqfd, wait);
    let mut flags: __poll_t = key_to_poll(key);
    if (flags & EPOLLIN) {
    u64 cnt;
    eventfd_ctx_do_read(virqfd.eventfd, &cnt);
// An event has been signaled, call function
    if ((!virqfd.handler ||
    virqfd.handler(virqfd.opaque, virqfd.data)) &&
    virqfd.thread)
    schedule_work(&virqfd.inject);
    }
    if (flags & EPOLLHUP) {
    unsigned long flags;
    spin_lock_irqsave(&virqfd_lock, flags);
//
// The eventfd is closing, if the virqfd has not yet been
// queued for release, as determined by testing whether the
// virqfd pointer to it is still valid, queue it now.  As
// with kvm irqfds, we know we won't race against the virqfd
// going away because we hold the lock to get here.
//
    if (*(virqfd.pvirqfd) == virqfd) {
// (virqfd->pvirqfd) = NULL;
    virqfd_deactivate(virqfd);
    }
    spin_unlock_irqrestore(&virqfd_lock, flags);
    }
    return 0;
    }
    static void virqfd_ptable_queue_proc(struct file *file,
    wait_queue_head_t *wqh, poll_table *pt)
    {
    struct virqfd *virqfd = container_of(pt, struct virqfd, pt);
    add_wait_queue(wqh, &virqfd.wait);
    }
#[no_mangle]
unsafe extern "C" fn virqfd_shutdown(work: *mut work_struct) {
    static void virqfd_shutdown(struct work_struct *work)
    {
    struct virqfd *virqfd = container_of(work, struct virqfd, shutdown);
    u64 cnt;
    eventfd_ctx_remove_wait_queue(virqfd.eventfd, &virqfd.wait, &cnt);
    flush_work(&virqfd.inject);
    eventfd_ctx_put(virqfd.eventfd);
    kfree(virqfd);
    }
#[no_mangle]
unsafe extern "C" fn virqfd_inject(work: *mut work_struct) {
    static void virqfd_inject(struct work_struct *work)
    {
    struct virqfd *virqfd = container_of(work, struct virqfd, inject);
    if (virqfd.thread)
    virqfd.thread(virqfd.opaque, virqfd.data);
    }
#[no_mangle]
unsafe extern "C" fn virqfd_flush_inject(work: *mut work_struct) {
    static void virqfd_flush_inject(struct work_struct *work)
    {
    struct virqfd *virqfd = container_of(work, struct virqfd, flush_inject);
    flush_work(&virqfd.inject);
    }
    int vfio_virqfd_enable(void *opaque,
    int (*handler)(void *, void *),
    void (*thread)(void *, void *),
    void *data, struct virqfd **pvirqfd, int fd)
    {
    struct eventfd_ctx *ctx;
    struct virqfd *virqfd;
    let mut ret: c_int = 0;
    __poll_t events;
    virqfd = kzalloc_obj(*virqfd, GFP_KERNEL_ACCOUNT);
    if (!virqfd)
    return -ENOMEM;
    virqfd.pvirqfd = pvirqfd;
    virqfd.opaque = opaque;
    virqfd.handler = handler;
    virqfd.thread = thread;
    virqfd.data = data;
    INIT_WORK(&virqfd.shutdown, virqfd_shutdown);
    INIT_WORK(&virqfd.inject, virqfd_inject);
    INIT_WORK(&virqfd.flush_inject, virqfd_flush_inject);
    CLASS(fd, irqfd)(fd);
    if (fd_empty(irqfd)) {
    ret = -EBADF;
    goto err_fd;
    }
    ctx = eventfd_ctx_fileget(fd_file(irqfd));
    if (IS_ERR(ctx)) {
    ret = PTR_ERR(ctx);
    goto err_fd;
    }
    virqfd.eventfd = ctx;
//
// virqfds can be released by closing the eventfd or directly
// through ioctl.  These are both done through a workqueue, so
// we update the pointer to the virqfd under lock to avoid
// pushing multiple jobs to release the same virqfd.
//
    spin_lock_irq(&virqfd_lock);
    if (*pvirqfd) {
    spin_unlock_irq(&virqfd_lock);
    ret = -EBUSY;
    goto err_busy;
    }
// pvirqfd = virqfd;
    spin_unlock_irq(&virqfd_lock);
//
// Install our own custom wake-up handling so we are notified via
// a callback whenever someone signals the underlying eventfd.
//
    init_waitqueue_func_entry(&virqfd.wait, virqfd_wakeup);
    init_poll_funcptr(&virqfd.pt, virqfd_ptable_queue_proc);
    events = vfs_poll(fd_file(irqfd), &virqfd.pt);
//
// Check if there was an event already pending on the eventfd
// before we registered and trigger it as if we didn't miss it.
//
    if (events & EPOLLIN) {
    if ((!handler || handler(opaque, data)) && thread)
    schedule_work(&virqfd.inject);
    }
    return 0;
    err_busy:
    eventfd_ctx_put(ctx);
    err_fd:
    kfree(virqfd);
    return ret;
    }
    EXPORT_SYMBOL_GPL(vfio_virqfd_enable);
#[no_mangle]
pub unsafe extern "C" fn vfio_virqfd_disable(pvirqfd: *mut virqfd) {
    void vfio_virqfd_disable(struct virqfd **pvirqfd)
    {
    unsigned long flags;
    spin_lock_irqsave(&virqfd_lock, flags);
    if (*pvirqfd) {
    virqfd_deactivate(*pvirqfd);
// pvirqfd = NULL;
    }
    spin_unlock_irqrestore(&virqfd_lock, flags);
//
// Block until we know all outstanding shutdown jobs have completed.
// Even if we don't queue the job, flush the wq to be sure it's
// been released.
//
    flush_workqueue(vfio_irqfd_cleanup_wq);
    }
    EXPORT_SYMBOL_GPL(vfio_virqfd_disable);
#[no_mangle]
pub unsafe extern "C" fn vfio_virqfd_flush_thread(pvirqfd: *mut virqfd) {
    void vfio_virqfd_flush_thread(struct virqfd **pvirqfd)
    {
    unsigned long flags;
    spin_lock_irqsave(&virqfd_lock, flags);
    if (*pvirqfd && (*pvirqfd).thread)
    queue_work(vfio_irqfd_cleanup_wq, &(*pvirqfd).flush_inject);
    spin_unlock_irqrestore(&virqfd_lock, flags);
    flush_workqueue(vfio_irqfd_cleanup_wq);
    }
    EXPORT_SYMBOL_GPL(vfio_virqfd_flush_thread);
