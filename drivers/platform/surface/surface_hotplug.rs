//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface_hotplug.c
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
// Surface Book (2 and later) hot-plug driver.
//
// Surface Book devices (can) have a hot-pluggable discrete GPU (dGPU). This
// driver is responsible for out-of-band hot-plug event signaling on these
// devices. It is specifically required when the hot-plug device is in D3cold
// and can thus not generate PCIe hot-plug events itself.
//
// Event signaling is handled via ACPI, which will generate the appropriate
// device-check notifications to be picked up by the PCIe hot-plug driver.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

    let mut shps_base_presence_int: static struct acpi_gpio_params = { 0, 0, false };
    let mut shps_base_presence: static struct acpi_gpio_params = { 1, 0, false };
    let mut shps_device_power_int: static struct acpi_gpio_params = { 2, 0, false };
    let mut shps_device_power: static struct acpi_gpio_params = { 3, 0, false };
    let mut shps_device_presence_int: static struct acpi_gpio_params = { 4, 0, false };
    let mut shps_device_presence: static struct acpi_gpio_params = { 5, 0, false };
    static const struct acpi_gpio_mapping shps_acpi_gpios[] = {
    { "base_presence-int-gpio",   &shps_base_presence_int,   1 },
    { "base_presence-gpio",       &shps_base_presence,       1 },
    { "device_power-int-gpio",    &shps_device_power_int,    1 },
    { "device_power-gpio",        &shps_device_power,        1 },
    { "device_presence-int-gpio", &shps_device_presence_int, 1 },
    { "device_presence-gpio",     &shps_device_presence,     1 },
    { },
    };
// 5515a847-ed55-4b27-8352-cd320e10360a
    static const guid_t shps_dsm_guid =
    GUID_INIT(0x5515a847, 0xed55, 0x4b27, 0x83, 0x52, 0xcd, 0x32, 0x0e, 0x10, 0x36, 0x0a);
pub const SHPS_DSM_REVISION: c_int = 1;
    enum shps_dsm_fn {
    SHPS_DSM_FN_PCI_NUM_ENTRIES	= 0x01,
    SHPS_DSM_FN_PCI_GET_ENTRIES	= 0x02,
    SHPS_DSM_FN_IRQ_BASE_PRESENCE	= 0x03,
    SHPS_DSM_FN_IRQ_DEVICE_POWER	= 0x04,
    SHPS_DSM_FN_IRQ_DEVICE_PRESENCE	= 0x05,
    };
    enum shps_irq_type {
// NOTE: Must be in order of enum shps_dsm_fn above.
    SHPS_IRQ_TYPE_BASE_PRESENCE	= 0,
    SHPS_IRQ_TYPE_DEVICE_POWER	= 1,
    SHPS_IRQ_TYPE_DEVICE_PRESENCE	= 2,
    SHPS_NUM_IRQS,
    };
    static const char *const shps_gpio_names[] = {
    [SHPS_IRQ_TYPE_BASE_PRESENCE]	= "base_presence",
    [SHPS_IRQ_TYPE_DEVICE_POWER]	= "device_power",
    [SHPS_IRQ_TYPE_DEVICE_PRESENCE]	= "device_presence",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shps_device {
    pub /: *mut *mut mutex lock[SHPS_NUM_IRQS]; / Protects update in shps_dsm_notify_irq(),
    pub gpio: [*mut gpio_desc; SHPS_NUM_IRQS],
    pub irq: [c_uint; SHPS_NUM_IRQS],
}

#[no_mangle]
unsafe extern "C" fn shps_dsm_fn_for_irq(type: enum shps_irq_type) -> enum shps_dsm_fn {
    static enum shps_dsm_fn shps_dsm_fn_for_irq(enum shps_irq_type type)
    {
    return SHPS_DSM_FN_IRQ_BASE_PRESENCE + type;
    }
#[no_mangle]
unsafe extern "C" fn shps_dsm_notify_irq(pdev: *mut platform_device, type: enum shps_irq_type) {
    static void shps_dsm_notify_irq(struct platform_device *pdev, enum shps_irq_type type)
    {
    struct shps_device *sdev = platform_get_drvdata(pdev);
    let mut handle: acpi_handle = ACPI_HANDLE(&pdev.dev);
    union acpi_object *result;
    union acpi_object param;
    int value;
    mutex_lock(&sdev.lock[type]);
    value = gpiod_get_value_cansleep(sdev.gpio[type]);
    if (value < 0) {
    mutex_unlock(&sdev.lock[type]);
    dev_err(&pdev.dev, "failed to get gpio: %d (irq=%d)\n", type, value);
    return;
    }
    dev_dbg(&pdev.dev, "IRQ notification via DSM (irq=%d, value=%d)\n", type, value);
    param.type = ACPI_TYPE_INTEGER;
    param.integer.value = value;
    result = acpi_evaluate_dsm_typed(handle, &shps_dsm_guid, SHPS_DSM_REVISION,
    shps_dsm_fn_for_irq(type), &param, ACPI_TYPE_BUFFER);
    if (!result) {
    dev_err(&pdev.dev, "IRQ notification via DSM failed (irq=%d, gpio=%d)\n",
    type, value);
    } else if (result.buffer.length != 1 || result.buffer.pointer[0] != 0) {
    dev_err(&pdev.dev,
    "IRQ notification via DSM failed: unexpected result value (irq=%d, gpio=%d)\n",
    type, value);
    }
    mutex_unlock(&sdev.lock[type]);
    ACPI_FREE(result);
    }
#[no_mangle]
unsafe extern "C" fn shps_handle_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t shps_handle_irq(int irq, void *data)
    {
    struct platform_device *pdev = data;
    struct shps_device *sdev = platform_get_drvdata(pdev);
    int type;
// Figure out which IRQ we're handling.
    for (type = 0; type < SHPS_NUM_IRQS; type++)
    if (irq == sdev.irq[type])
    break;
// We should have found our interrupt, if not: this is a bug.
    if (WARN(type >= SHPS_NUM_IRQS, "invalid IRQ number: %d\n", irq))
    return IRQ_HANDLED;
// Forward interrupt to ACPI via DSM.
    shps_dsm_notify_irq(pdev, type);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn shps_setup_irq(pdev: *mut platform_device, type: enum shps_irq_type) -> c_int {
    static int shps_setup_irq(struct platform_device *pdev, enum shps_irq_type type)
    {
    let mut flags: c_ulong = IRQF_ONESHOT | IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING;
    struct shps_device *sdev = platform_get_drvdata(pdev);
    struct gpio_desc *gpiod;
    let mut handle: acpi_handle = ACPI_HANDLE(&pdev.dev);
    const char *irq_name;
    let mut dsm: c_int = shps_dsm_fn_for_irq(type);
    int status, irq;
//
// Only set up interrupts that we actually need: The Surface Book 3
// does not have a DSM for base presence, so don't set up an interrupt
// for that.
//
    if (!acpi_check_dsm(handle, &shps_dsm_guid, SHPS_DSM_REVISION, BIT(dsm))) {
    dev_dbg(&pdev.dev, "IRQ notification via DSM not present (irq=%d)\n", type);
    return 0;
    }
    gpiod = devm_gpiod_get(&pdev.dev, shps_gpio_names[type], GPIOD_ASIS);
    if (IS_ERR(gpiod))
    return PTR_ERR(gpiod);
    irq = gpiod_to_irq(gpiod);
    if (irq < 0)
    return irq;
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "shps-irq-%d", type);
    if (!irq_name)
    return -ENOMEM;
    status = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(), shps_handle_irq,
    flags, irq_name, pdev);
    if (status)
    return status;
    dev_dbg(&pdev.dev, "set up irq %d as type %d\n", irq, type);
    sdev.gpio[type] = gpiod;
    sdev.irq[type] = irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface_hotplug_remove(pdev: *mut platform_device) {
    static void surface_hotplug_remove(struct platform_device *pdev)
    {
    struct shps_device *sdev = platform_get_drvdata(pdev);
    int i;
// Ensure that IRQs have been fully handled and won't trigger any more.
    for (i = 0; i < SHPS_NUM_IRQS; i++) {
    if (sdev.irq[i] != SHPS_IRQ_NOT_PRESENT)
    disable_irq(sdev.irq[i]);
    mutex_destroy(&sdev.lock[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn surface_hotplug_probe(pdev: *mut platform_device) -> c_int {
    static int surface_hotplug_probe(struct platform_device *pdev)
    {
    struct shps_device *sdev;
    int status, i;
//
// The MSHW0153 device is also present on the Surface Laptop 3,
// however that doesn't have a hot-pluggable PCIe device. It also
// doesn't have any GPIO interrupts/pins under the MSHW0153, so filter
// it out here.
//
    if (gpiod_count(&pdev.dev, core::ptr::null_mut()) < 0)
    return -ENODEV;
    status = devm_acpi_dev_add_driver_gpios(&pdev.dev, shps_acpi_gpios);
    if (status)
    return status;
    sdev = devm_kzalloc(&pdev.dev, sizeof(*sdev), GFP_KERNEL);
    if (!sdev)
    return -ENOMEM;
    platform_set_drvdata(pdev, sdev);
//
// Initialize IRQs so that we can safely call surface_hotplug_remove()
// on errors.
//
    for (i = 0; i < SHPS_NUM_IRQS; i++)
    sdev.irq[i] = SHPS_IRQ_NOT_PRESENT;
// Set up IRQs.
    for (i = 0; i < SHPS_NUM_IRQS; i++) {
    mutex_init(&sdev.lock[i]);
    status = shps_setup_irq(pdev, i);
    if (status) {
    dev_err(&pdev.dev, "failed to set up IRQ %d: %d\n", i, status);
    goto err;
    }
    }
// Ensure everything is up-to-date.
    for (i = 0; i < SHPS_NUM_IRQS; i++)
    if (sdev.irq[i] != SHPS_IRQ_NOT_PRESENT)
    shps_dsm_notify_irq(pdev, i);
    return 0;
    err:
    surface_hotplug_remove(pdev);
    return status;
    }
    static const struct acpi_device_id surface_hotplug_acpi_match[] = {
    { "MSHW0153", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, surface_hotplug_acpi_match);
    static struct platform_driver surface_hotplug_driver = {
    .probe = surface_hotplug_probe,
    .remove = surface_hotplug_remove,
    .driver = {
    .name = "surface_hotplug",
    .acpi_match_table = surface_hotplug_acpi_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(surface_hotplug_driver);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Surface Hot-Plug Signaling Driver for Surface Book Devices");
    MODULE_LICENSE("GPL");
