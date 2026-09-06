//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/agdi.c
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
// This file implements handling of
// Arm Generic Diagnostic Dump and Reset Interface table (AGDI)
//
// Copyright (c) 2022, Ampere Computing LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agdi_data {
    pub /: *mut *mut unsigned char flags; / AGDI Signaling Mode,
    pub sdei_event: c_int,
    pub gsiv: c_uint,
    pub use_nmi: bool,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn agdi_sdei_handler(sdei_event: u32, regs: *mut pt_regs, arg: *mut c_void) -> c_int {
    static int agdi_sdei_handler(u32 sdei_event, struct pt_regs *regs, void *arg)
    {
    nmi_panic(regs, "Arm Generic Diagnostic Dump and Reset SDEI event issued");
    return 0;
    }
    static int agdi_sdei_probe(struct platform_device *pdev,
    struct agdi_data *adata)
    {
    int err;
    err = sdei_event_register(adata.sdei_event, agdi_sdei_handler, pdev);
    if (err) {
    dev_err(&pdev.dev, "Failed to register for SDEI event %d\n",
    adata.sdei_event);
    return err;
    }
    err = sdei_event_enable(adata.sdei_event);
    if (err)  {
    sdei_event_unregister(adata.sdei_event);
    dev_err(&pdev.dev, "Failed to enable event %d\n",
    adata.sdei_event);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn agdi_interrupt_handler_nmi(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t agdi_interrupt_handler_nmi(int irq, void *dev_id)
    {
    nmi_panic(core::ptr::null_mut(), "Arm Generic Diagnostic Dump and Reset NMI Interrupt event issued\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn agdi_interrupt_handler_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t agdi_interrupt_handler_irq(int irq, void *dev_id)
    {
    panic("Arm Generic Diagnostic Dump and Reset Interrupt event issued\n");
    return IRQ_HANDLED;
    }
    static int agdi_interrupt_probe(struct platform_device *pdev,
    struct agdi_data *adata)
    {
    unsigned long irq_flags;
    int ret;
    int irq;
    irq = acpi_register_gsi(core::ptr::null_mut(), adata.gsiv, ACPI_EDGE_SENSITIVE, ACPI_ACTIVE_HIGH);
    if (irq < 0) {
    dev_err(&pdev.dev, "cannot register GSI#%d (%d)\n", adata.gsiv, irq);
    return irq;
    }
    irq_flags = IRQF_PERCPU | IRQF_NOBALANCING | IRQF_NO_AUTOEN |
    IRQF_NO_THREAD;
// try NMI first
    ret = request_nmi(irq, &agdi_interrupt_handler_nmi, irq_flags,
    "agdi_interrupt_nmi", core::ptr::null_mut());
    if (!ret) {
    enable_nmi(irq);
    adata.irq = irq;
    adata.use_nmi = true;
    return 0;
    }
// Then try normal interrupt
    ret = request_irq(irq, &agdi_interrupt_handler_irq,
    irq_flags, "agdi_interrupt_irq", core::ptr::null_mut());
    if (ret) {
    dev_err(&pdev.dev, "cannot register IRQ %d\n", ret);
    acpi_unregister_gsi(adata.gsiv);
    return ret;
    }
    enable_irq(irq);
    adata.irq = irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn agdi_probe(pdev: *mut platform_device) -> c_int {
    static int agdi_probe(struct platform_device *pdev)
    {
    struct agdi_data *adata = dev_get_platdata(&pdev.dev);
    if (!adata)
    return -EINVAL;
    if (adata.flags & ACPI_AGDI_SIGNALING_MODE)
    return agdi_interrupt_probe(pdev, adata);
    else
    return agdi_sdei_probe(pdev, adata);
    }
    static void agdi_sdei_remove(struct platform_device *pdev,
    struct agdi_data *adata)
    {
    int err, i;
    err = sdei_event_disable(adata.sdei_event);
    if (err) {
    dev_err(&pdev.dev, "Failed to disable sdei-event #%d (%pe)\n",
    adata.sdei_event, ERR_PTR(err));
    return;
    }
    for (i = 0; i < 3; i++) {
    err = sdei_event_unregister(adata.sdei_event);
    if (err != -EINPROGRESS)
    break;
    schedule();
    }
    if (err)
    dev_err(&pdev.dev, "Failed to unregister sdei-event #%d (%pe)\n",
    adata.sdei_event, ERR_PTR(err));
    }
    static void agdi_interrupt_remove(struct platform_device *pdev,
    struct agdi_data *adata)
    {
    if (adata.irq == -1)
    return;
    if (adata.use_nmi)
    free_nmi(adata.irq, core::ptr::null_mut());
    else
    free_irq(adata.irq, core::ptr::null_mut());
    acpi_unregister_gsi(adata.gsiv);
    }
#[no_mangle]
unsafe extern "C" fn agdi_remove(pdev: *mut platform_device) {
    static void agdi_remove(struct platform_device *pdev)
    {
    struct agdi_data *adata = dev_get_platdata(&pdev.dev);
    if (adata.flags & ACPI_AGDI_SIGNALING_MODE)
    agdi_interrupt_remove(pdev, adata);
    else
    agdi_sdei_remove(pdev, adata);
    }
    static struct platform_driver agdi_driver = {
    .driver = {
    .name = "agdi",
    },
    .probe = agdi_probe,
    .remove = agdi_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_agdi_init() -> void __init {
    void __init acpi_agdi_init(void)
    {
    struct acpi_table_agdi *agdi_table;
    let mut pdata: agdi_data = { 0 };
    struct platform_device *pdev;
    acpi_status status;
    status = acpi_get_table(ACPI_SIG_AGDI, 0,
    (struct acpi_table_header **) &agdi_table);
    if (ACPI_FAILURE(status))
    return;
    if (agdi_table.flags & ACPI_AGDI_SIGNALING_MODE)
    pdata.gsiv = agdi_table.gsiv;
    else
    pdata.sdei_event = agdi_table.sdei_event;
    pdata.irq = -1;
    pdata.flags = agdi_table.flags;
    pdev = platform_device_register_data(core::ptr::null_mut(), "agdi", 0, &pdata, sizeof(pdata));
    if (IS_ERR(pdev))
    goto err_put_table;
    if (platform_driver_register(&agdi_driver))
    platform_device_unregister(pdev);
    err_put_table:
    acpi_put_table((struct acpi_table_header *)agdi_table);
    }
