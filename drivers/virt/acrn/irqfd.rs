//! Automatically rewritten from C to Rust
//! Source: drivers/virt/acrn/irqfd.c
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
// ACRN HSM irqfd: use eventfd objects to inject virtual interrupts
//
// Copyright (C) 2020 Intel Corporation. All rights reserved.
//
// Authors:
// Shuo Liu <shuo.a.liu@intel.com>
// Yakui Zhao <yakui.zhao@intel.com>
//

// Cleanup work has been queued; set via test_and_set_bit().
pub const HSM_IRQFD_FLAG_SHUTDOWN: c_int = 0;
//
// struct hsm_irqfd - Properties of HSM irqfd
// @vm:		Associated VM pointer
// @wait:	Entry of wait-queue
// @shutdown:	Async shutdown work
// @eventfd:	Associated eventfd
// @list:	Entry within &acrn_vm.irqfds of irqfds of a VM
// @pt:		Structure for select/poll on the associated eventfd
// @msi:	MSI data
// @flags:	Internal lifecycle flags (HSM_IRQFD_FLAG_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsm_irqfd {
    pub vm: *mut acrn_vm,
    pub wait: wait_queue_entry_t,
    pub shutdown: work_struct,
    pub eventfd: *mut eventfd_ctx,
    pub list: list_head,
    pub pt: poll_table,
    pub msi: acrn_msi_entry,
    pub flags: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn acrn_irqfd_inject(irqfd: *mut hsm_irqfd) {
    static void acrn_irqfd_inject(struct hsm_irqfd *irqfd)
    {
    struct acrn_vm *vm = irqfd.vm;
    acrn_msi_inject(vm, irqfd.msi.msi_addr,
    irqfd.msi.msi_data);
    }
// Queue the cleanup work at most once. Safe from atomic context.
#[no_mangle]
unsafe extern "C" fn hsm_irqfd_queue_shutdown(irqfd: *mut hsm_irqfd) {
    static void hsm_irqfd_queue_shutdown(struct hsm_irqfd *irqfd)
    {
    if (!test_and_set_bit(HSM_IRQFD_FLAG_SHUTDOWN, &irqfd.flags))
    queue_work(irqfd.vm.irqfd_wq, &irqfd.shutdown);
    }
// Sole owner of @irqfd: unhook waitqueue, drop eventfd ref, free.
#[no_mangle]
unsafe extern "C" fn hsm_irqfd_shutdown_work(work: *mut work_struct) {
    static void hsm_irqfd_shutdown_work(struct work_struct *work)
    {
    struct hsm_irqfd *irqfd = container_of(work, struct hsm_irqfd,
    shutdown);
    struct acrn_vm *vm = irqfd.vm;
    u64 cnt;
    mutex_lock(&vm.irqfds_lock);
    if (!list_empty(&irqfd.list))
    list_del_init(&irqfd.list);
    mutex_unlock(&vm.irqfds_lock);
    eventfd_ctx_remove_wait_queue(irqfd.eventfd, &irqfd.wait, &cnt);
    eventfd_ctx_put(irqfd.eventfd);
    kfree(irqfd);
    }
// Called with wqh->lock held and interrupts disabled
    static int hsm_irqfd_wakeup(wait_queue_entry_t *wait, unsigned int mode,
    int sync, void *key)
    {
    let mut poll_bits: c_ulong = (unsigned long)key;
    struct hsm_irqfd *irqfd;
    irqfd = container_of(wait, struct hsm_irqfd, wait);
    if (poll_bits & POLLIN)
// An event has been signaled, inject an interrupt
    acrn_irqfd_inject(irqfd);
    if (poll_bits & POLLHUP)
// Defer teardown to the cleanup work; can't sleep here.
    hsm_irqfd_queue_shutdown(irqfd);
    return 0;
    }
    static void hsm_irqfd_poll_func(struct file *file, wait_queue_head_t *wqh,
    poll_table *pt)
    {
    struct hsm_irqfd *irqfd;
    irqfd = container_of(pt, struct hsm_irqfd, pt);
    add_wait_queue(wqh, &irqfd.wait);
    }
//
// Assign an eventfd to a VM and create a HSM irqfd associated with the
// eventfd. The properties of the HSM irqfd are built from a &struct
// acrn_irqfd.
//
#[no_mangle]
unsafe extern "C" fn acrn_irqfd_assign(vm: *mut acrn_vm, args: *mut acrn_irqfd) -> c_int {
    static int acrn_irqfd_assign(struct acrn_vm *vm, struct acrn_irqfd *args)
    {
    struct eventfd_ctx *eventfd = core::ptr::null_mut();
    struct hsm_irqfd *irqfd, *tmp;
    __poll_t events;
    let mut ret: c_int = 0;
    irqfd = kzalloc_obj(*irqfd);
    if (!irqfd)
    return -ENOMEM;
    irqfd.vm = vm;
    memcpy(&irqfd.msi, &args.msi, sizeof(args.msi));
    INIT_LIST_HEAD(&irqfd.list);
    INIT_WORK(&irqfd.shutdown, hsm_irqfd_shutdown_work);
    CLASS(fd, f)(args.fd);
    if (fd_empty(f)) {
    ret = -EBADF;
    goto out;
    }
    eventfd = eventfd_ctx_fileget(fd_file(f));
    if (IS_ERR(eventfd)) {
    ret = PTR_ERR(eventfd);
    goto out;
    }
    irqfd.eventfd = eventfd;
//
// Install custom wake-up handling to be notified whenever underlying
// eventfd is signaled.
//
    init_waitqueue_func_entry(&irqfd.wait, hsm_irqfd_wakeup);
    init_poll_funcptr(&irqfd.pt, hsm_irqfd_poll_func);
//
// Hold irqfds_lock across waitqueue install and list_add so the
// irqfd is not visible to deassign/deinit before its waitqueue
// entry is in place, and any racing EPOLLHUP cleanup work blocks
// on irqfds_lock until publication completes.
//
    mutex_lock(&vm.irqfds_lock);
    list_for_each_entry(tmp, &vm.irqfds, list) {
    if (irqfd.eventfd != tmp.eventfd)
    continue;
    ret = -EBUSY;
    mutex_unlock(&vm.irqfds_lock);
    goto fail;
    }
    events = vfs_poll(fd_file(f), &irqfd.pt);
    list_add_tail(&irqfd.list, &vm.irqfds);
    if (events & EPOLLIN)
    acrn_irqfd_inject(irqfd);
    mutex_unlock(&vm.irqfds_lock);
    return 0;
    fail:
    eventfd_ctx_put(eventfd);
    out:
    kfree(irqfd);
    return ret;
    }
    static int acrn_irqfd_deassign(struct acrn_vm *vm,
    struct acrn_irqfd *args)
    {
    struct hsm_irqfd *irqfd, *tmp;
    struct eventfd_ctx *eventfd;
    eventfd = eventfd_ctx_fdget(args.fd);
    if (IS_ERR(eventfd))
    return PTR_ERR(eventfd);
    mutex_lock(&vm.irqfds_lock);
    list_for_each_entry_safe(irqfd, tmp, &vm.irqfds, list) {
    if (irqfd.eventfd == eventfd) {
    list_del_init(&irqfd.list);
    hsm_irqfd_queue_shutdown(irqfd);
    break;
    }
    }
    mutex_unlock(&vm.irqfds_lock);
    eventfd_ctx_put(eventfd);
// Wait for cleanup work to finish so the eventfd is fully detached.
    flush_workqueue(vm.irqfd_wq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acrn_irqfd_config(vm: *mut acrn_vm, args: *mut acrn_irqfd) -> c_int {
    int acrn_irqfd_config(struct acrn_vm *vm, struct acrn_irqfd *args)
    {
    int ret;
    if (args.flags & ACRN_IRQFD_FLAG_DEASSIGN)
    ret = acrn_irqfd_deassign(vm, args);
    else
    ret = acrn_irqfd_assign(vm, args);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn acrn_irqfd_init(vm: *mut acrn_vm) -> c_int {
    int acrn_irqfd_init(struct acrn_vm *vm)
    {
    INIT_LIST_HEAD(&vm.irqfds);
    mutex_init(&vm.irqfds_lock);
    vm.irqfd_wq = alloc_workqueue("acrn_irqfd-%u", WQ_PERCPU, 0, vm.vmid);
    if (!vm.irqfd_wq)
    return -ENOMEM;
    dev_dbg(acrn_dev.this_device, "VM %u irqfd init.\n", vm.vmid);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acrn_irqfd_deinit(vm: *mut acrn_vm) {
    void acrn_irqfd_deinit(struct acrn_vm *vm)
    {
    struct hsm_irqfd *irqfd, *next;
    dev_dbg(acrn_dev.this_device, "VM %u irqfd deinit.\n", vm.vmid);
    mutex_lock(&vm.irqfds_lock);
    list_for_each_entry_safe(irqfd, next, &vm.irqfds, list) {
    list_del_init(&irqfd.list);
    hsm_irqfd_queue_shutdown(irqfd);
    }
    mutex_unlock(&vm.irqfds_lock);
// Drain all cleanup work before tearing the workqueue down.
    flush_workqueue(vm.irqfd_wq);
    destroy_workqueue(vm.irqfd_wq);
    }
