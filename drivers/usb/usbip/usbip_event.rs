//! Automatically rewritten from C to Rust
//! Source: drivers/usb/usbip/usbip_event.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2003-2008 Takahiro Hirofuchi
// Copyright (C) 2015 Nobuo Iwata
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_event {
    pub node: list_head,
    pub ud: *mut usbip_device,
}

    static DEFINE_SPINLOCK(event_lock);
    static LIST_HEAD(event_list);
#[no_mangle]
unsafe extern "C" fn set_event(ud: *mut usbip_device, event: c_ulong) {
    static void set_event(struct usbip_device *ud, unsigned long event)
    {
    unsigned long flags;
    spin_lock_irqsave(&ud.lock, flags);
    ud.event |= event;
    spin_unlock_irqrestore(&ud.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn unset_event(ud: *mut usbip_device, event: c_ulong) {
    static void unset_event(struct usbip_device *ud, unsigned long event)
    {
    unsigned long flags;
    spin_lock_irqsave(&ud.lock, flags);
    ud.event &= ~event;
    spin_unlock_irqrestore(&ud.lock, flags);
    }
    static struct usbip_device *get_event(void)
    {
    struct usbip_event *ue = core::ptr::null_mut();
    struct usbip_device *ud = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&event_lock, flags);
    if (!list_empty(&event_list)) {
    ue = list_first_entry(&event_list, struct usbip_event, node);
    list_del(&ue.node);
    }
    spin_unlock_irqrestore(&event_lock, flags);
    if (ue) {
    ud = ue.ud;
    kfree(ue);
    }
    return ud;
    }
    static struct task_struct *worker_context;
#[no_mangle]
unsafe extern "C" fn event_handler(work: *mut work_struct) {
    static void event_handler(struct work_struct *work)
    {
    struct usbip_device *ud;
    if (worker_context == core::ptr::null_mut()) {
    worker_context = current;
    }
    while ((ud = get_event()) != core::ptr::null_mut()) {
    usbip_dbg_eh("pending event %lx\n", ud.event);
    mutex_lock(&ud.sysfs_lock);
//
// NOTE: shutdown must come first.
// Shutdown the device.
//
    if (ud.event & USBIP_EH_SHUTDOWN) {
    ud.eh_ops.shutdown(ud);
    unset_event(ud, USBIP_EH_SHUTDOWN);
    }
// Reset the device.
    if (ud.event & USBIP_EH_RESET) {
    ud.eh_ops.reset(ud);
    unset_event(ud, USBIP_EH_RESET);
    }
// Mark the device as unusable.
    if (ud.event & USBIP_EH_UNUSABLE) {
    ud.eh_ops.unusable(ud);
    unset_event(ud, USBIP_EH_UNUSABLE);
    }
    mutex_unlock(&ud.sysfs_lock);
    wake_up(&ud.eh_waitq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_start_eh(ud: *mut usbip_device) -> c_int {
    int usbip_start_eh(struct usbip_device *ud)
    {
    init_waitqueue_head(&ud.eh_waitq);
    ud.event = 0;
    return 0;
    }
    EXPORT_SYMBOL_GPL(usbip_start_eh);
#[no_mangle]
pub unsafe extern "C" fn usbip_stop_eh(ud: *mut usbip_device) {
    void usbip_stop_eh(struct usbip_device *ud)
    {
    let mut pending: c_ulong = ud.event & ~USBIP_EH_BYE;
    if (!(ud.event & USBIP_EH_BYE))
    usbip_dbg_eh("usbip_eh stopping but not removed\n");
    if (pending)
    usbip_dbg_eh("usbip_eh waiting completion %lx\n", pending);
    wait_event_interruptible(ud.eh_waitq, !(ud.event & ~USBIP_EH_BYE));
    usbip_dbg_eh("usbip_eh has stopped\n");
    }
    EXPORT_SYMBOL_GPL(usbip_stop_eh);

    static struct workqueue_struct *usbip_queue;
    static DECLARE_WORK(usbip_work, event_handler);
#[no_mangle]
pub unsafe extern "C" fn usbip_init_eh() -> c_int {
    int usbip_init_eh(void)
    {
    usbip_queue = create_singlethread_workqueue(WORK_QUEUE_NAME);
    if (usbip_queue == core::ptr::null_mut()) {
    pr_err("failed to create usbip_event\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_finish_eh() {
    void usbip_finish_eh(void)
    {
    destroy_workqueue(usbip_queue);
    usbip_queue = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_event_add(ud: *mut usbip_device, event: c_ulong) {
    void usbip_event_add(struct usbip_device *ud, unsigned long event)
    {
    struct usbip_event *ue;
    unsigned long flags;
    if (ud.event & USBIP_EH_BYE)
    return;
    set_event(ud, event);
    spin_lock_irqsave(&event_lock, flags);
    list_for_each_entry_reverse(ue, &event_list, node) {
    if (ue.ud == ud)
    goto out;
    }
    ue = kmalloc_obj(struct usbip_event, GFP_ATOMIC);
    if (ue == core::ptr::null_mut())
    goto out;
    ue.ud = ud;
    list_add_tail(&ue.node, &event_list);
    queue_work(usbip_queue, &usbip_work);
    out:
    spin_unlock_irqrestore(&event_lock, flags);
    }
    EXPORT_SYMBOL_GPL(usbip_event_add);
#[no_mangle]
pub unsafe extern "C" fn usbip_event_happened(ud: *mut usbip_device) -> c_int {
    int usbip_event_happened(struct usbip_device *ud)
    {
    let mut happened: c_int = 0;
    unsigned long flags;
    spin_lock_irqsave(&ud.lock, flags);
    if (ud.event != 0)
    happened = 1;
    spin_unlock_irqrestore(&ud.lock, flags);
    return happened;
    }
    EXPORT_SYMBOL_GPL(usbip_event_happened);
#[no_mangle]
pub unsafe extern "C" fn usbip_in_eh(task: *mut task_struct) -> c_int {
    int usbip_in_eh(struct task_struct *task)
    {
    if (task == worker_context)
    return 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(usbip_in_eh);
