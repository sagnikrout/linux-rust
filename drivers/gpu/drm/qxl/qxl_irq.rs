//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/qxl/qxl_irq.c
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


//
// Copyright 2013 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Dave Airlie
// Alon Levy
//

#[no_mangle]
unsafe extern "C" fn qxl_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t qxl_irq_handler(int irq, void *arg)
    {
    struct drm_device *dev = (struct drm_device *) arg;
    struct qxl_device *qdev = to_qxl(dev);
    uint32_t pending;
    pending = xchg(&qdev.ram_header.int_pending, 0);
    if (!pending)
    return IRQ_NONE;
    atomic_inc(&qdev.irq_received);
    if (pending & QXL_INTERRUPT_DISPLAY) {
    atomic_inc(&qdev.irq_received_display);
    wake_up_all(&qdev.display_event);
    qxl_queue_garbage_collect(qdev, false);
    }
    if (pending & QXL_INTERRUPT_CURSOR) {
    atomic_inc(&qdev.irq_received_cursor);
    wake_up_all(&qdev.cursor_event);
    }
    if (pending & QXL_INTERRUPT_IO_CMD) {
    atomic_inc(&qdev.irq_received_io_cmd);
    wake_up_all(&qdev.io_cmd_event);
    }
    if (pending & QXL_INTERRUPT_ERROR) {
// TODO: log it, reset device (only way to exit this condition)
// (do it a certain number of times, afterwards admit defeat,
// to avoid endless loops).
//
    qdev.irq_received_error++;
    DRM_WARN("driver is in bug mode\n");
    }
    if (pending & QXL_INTERRUPT_CLIENT_MONITORS_CONFIG) {
    schedule_work(&qdev.client_monitors_config_work);
    }
    qdev.ram_header.int_mask = QXL_INTERRUPT_MASK;
    outb(0, qdev.io_base + QXL_IO_UPDATE_IRQ);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qxl_client_monitors_config_work_func(work: *mut work_struct) {
    static void qxl_client_monitors_config_work_func(struct work_struct *work)
    {
    struct qxl_device *qdev = container_of(work, struct qxl_device,
    client_monitors_config_work);
    qxl_display_read_client_monitors_config(qdev);
    }
#[no_mangle]
pub unsafe extern "C" fn qxl_irq_init(qdev: *mut qxl_device) -> c_int {
    int qxl_irq_init(struct qxl_device *qdev)
    {
    struct drm_device *ddev = &qdev.ddev;
    struct pci_dev *pdev = to_pci_dev(ddev.dev);
    int ret;
    init_waitqueue_head(&qdev.display_event);
    init_waitqueue_head(&qdev.cursor_event);
    init_waitqueue_head(&qdev.io_cmd_event);
    init_waitqueue_head(&qdev.release_event);
    INIT_WORK(&qdev.client_monitors_config_work,
    qxl_client_monitors_config_work_func);
    atomic_set(&qdev.irq_received, 0);
    atomic_set(&qdev.irq_received_display, 0);
    atomic_set(&qdev.irq_received_cursor, 0);
    atomic_set(&qdev.irq_received_io_cmd, 0);
    qdev.irq_received_error = 0;
    ret = request_irq(pdev.irq, qxl_irq_handler, IRQF_SHARED, ddev.driver.name, ddev);
    qdev.ram_header.int_mask = QXL_INTERRUPT_MASK;
    if (unlikely(ret != 0)) {
    DRM_ERROR("Failed installing irq: %d\n", ret);
    return 1;
    }
    return 0;
    }
