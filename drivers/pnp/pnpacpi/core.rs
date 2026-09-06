//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/pnpacpi/core.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// pnpacpi -- PnP ACPI driver
//
// Copyright (c) 2004 Matthieu Castet <castet.matthieu@free.fr>
// Copyright (c) 2004 Li Shaohua <shaohua.li@intel.com>
//

    static int num;
//
// Compatible Device IDs
//

    if (!(('0' <= (c) && (c) <= '9') || ('A' <= (c) && (c) <= 'F'))) \
    return 0

    if (!('A' <= (c) && (c) <= 'Z')) \
    return 0
#[no_mangle]
unsafe extern "C" fn ispnpidacpi(id: *const c_char) -> int __init {
    static int __init ispnpidacpi(const char *id)
    {
    TEST_ALPHA(id[0]);
    TEST_ALPHA(id[1]);
    TEST_ALPHA(id[2]);
    TEST_HEX(id[3]);
    TEST_HEX(id[4]);
    TEST_HEX(id[5]);
    TEST_HEX(id[6]);
    if (id[7] != '\0')
    return 0;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_get_resources(dev: *mut pnp_dev) -> c_int {
    static int pnpacpi_get_resources(struct pnp_dev *dev)
    {
    pnp_dbg(&dev.dev, "get resources\n");
    return pnpacpi_parse_allocated_resource(dev);
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_set_resources(dev: *mut pnp_dev) -> c_int {
    static int pnpacpi_set_resources(struct pnp_dev *dev)
    {
    struct acpi_device *acpi_dev;
    acpi_handle handle;
    let mut ret: c_int = 0;
    pnp_dbg(&dev.dev, "set resources\n");
    acpi_dev = ACPI_COMPANION(&dev.dev);
    if (!acpi_dev) {
    dev_dbg(&dev.dev, "ACPI device not found in %s!\n", __func__);
    return -ENODEV;
    }
    if (WARN_ON_ONCE(acpi_dev != dev.data))
    dev.data = acpi_dev;
    handle = acpi_dev.handle;
    if (acpi_has_method(handle, METHOD_NAME__SRS)) {
    struct acpi_buffer buffer;
    ret = pnpacpi_build_resource_template(dev, &buffer);
    if (ret)
    return ret;
    ret = pnpacpi_encode_resources(dev, &buffer);
    if (!ret) {
    acpi_status status;
    status = acpi_set_current_resources(handle, &buffer);
    if (ACPI_FAILURE(status))
    ret = -EIO;
    }
    kfree(buffer.pointer);
    }
    if (!ret && acpi_device_power_manageable(acpi_dev))
    ret = acpi_device_set_power(acpi_dev, ACPI_STATE_D0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_disable_resources(dev: *mut pnp_dev) -> c_int {
    static int pnpacpi_disable_resources(struct pnp_dev *dev)
    {
    struct acpi_device *acpi_dev;
    acpi_status status;
    dev_dbg(&dev.dev, "disable resources\n");
    acpi_dev = ACPI_COMPANION(&dev.dev);
    if (!acpi_dev) {
    dev_dbg(&dev.dev, "ACPI device not found in %s!\n", __func__);
    return 0;
    }
// acpi_unregister_gsi(pnp_irq(dev, 0));
    if (acpi_device_power_manageable(acpi_dev))
    acpi_device_set_power(acpi_dev, ACPI_STATE_D3_COLD);
// continue even if acpi_device_set_power() fails
    status = acpi_evaluate_object(acpi_dev.handle, "_DIS", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status) && status != AE_NOT_FOUND)
    return -ENODEV;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pnpacpi_can_wakeup(dev: *mut pnp_dev) -> bool {
    static bool pnpacpi_can_wakeup(struct pnp_dev *dev)
    {
    struct acpi_device *acpi_dev = ACPI_COMPANION(&dev.dev);
    if (!acpi_dev) {
    dev_dbg(&dev.dev, "ACPI device not found in %s!\n", __func__);
    return false;
    }
    return acpi_bus_can_wakeup(acpi_dev.handle);
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_suspend(dev: *mut pnp_dev, state: pm_message_t) -> c_int {
    static int pnpacpi_suspend(struct pnp_dev *dev, pm_message_t state)
    {
    struct acpi_device *acpi_dev = ACPI_COMPANION(&dev.dev);
    let mut error: c_int = 0;
    if (!acpi_dev) {
    dev_dbg(&dev.dev, "ACPI device not found in %s!\n", __func__);
    return 0;
    }
    if (device_can_wakeup(&dev.dev)) {
    error = acpi_pm_set_device_wakeup(&dev.dev,
    device_may_wakeup(&dev.dev));
    if (error)
    return error;
    }
    if (acpi_device_power_manageable(acpi_dev)) {
    int power_state = acpi_pm_device_sleep_state(&dev.dev, core::ptr::null_mut(),
    ACPI_STATE_D3_COLD);
    if (power_state < 0)
    power_state = (state.event == PM_EVENT_ON) ?
    ACPI_STATE_D0 : ACPI_STATE_D3_COLD;
//
// acpi_device_set_power() can fail (keyboard port can't be
// powered-down?), and in any case, our return value is ignored
// by pnp_bus_suspend().  Hence we don't revert the wakeup
// setting if the set_power fails.
//
    error = acpi_device_set_power(acpi_dev, power_state);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_resume(dev: *mut pnp_dev) -> c_int {
    static int pnpacpi_resume(struct pnp_dev *dev)
    {
    struct acpi_device *acpi_dev = ACPI_COMPANION(&dev.dev);
    let mut error: c_int = 0;
    if (!acpi_dev) {
    dev_dbg(&dev.dev, "ACPI device not found in %s!\n", __func__);
    return -ENODEV;
    }
    if (device_may_wakeup(&dev.dev))
    acpi_pm_set_device_wakeup(&dev.dev, false);
    if (acpi_device_power_manageable(acpi_dev))
    error = acpi_device_set_power(acpi_dev, ACPI_STATE_D0);
    return error;
    }

    struct pnp_protocol pnpacpi_protocol = {
    .name	 = "Plug and Play ACPI",
    .get	 = pnpacpi_get_resources,
    .set	 = pnpacpi_set_resources,
    .disable = pnpacpi_disable_resources,

    .can_wakeup = pnpacpi_can_wakeup,
    .suspend = pnpacpi_suspend,
    .resume = pnpacpi_resume,

    };
    EXPORT_SYMBOL(pnpacpi_protocol);
#[no_mangle]
unsafe extern "C" fn pnpacpi_get_id(device: *mut acpi_device) -> *const char __init {
    static const char *__init pnpacpi_get_id(struct acpi_device *device)
    {
    struct acpi_hardware_id *id;
    list_for_each_entry(id, &device.pnp.ids, list) {
    if (ispnpidacpi(id.id))
    return id.id;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pnpacpi_add_device(device: *mut acpi_device) -> int __init {
    static int __init pnpacpi_add_device(struct acpi_device *device)
    {
    struct pnp_dev *dev;
    const char *pnpid;
    struct acpi_hardware_id *id;
    int error;
// Skip devices that are already bound
    if (device.physical_node_count)
    return 0;
//
// If a PnPacpi device is not present , the device
// driver should not be loaded.
//
    if (!acpi_has_method(device.handle, "_CRS"))
    return 0;
    pnpid = pnpacpi_get_id(device);
    if (!pnpid)
    return 0;
    if (!device.status.present)
    return 0;
    dev = pnp_alloc_dev(&pnpacpi_protocol, num, pnpid);
    if (!dev)
    return -ENOMEM;
    ACPI_COMPANION_SET(&dev.dev, device);
    dev.data = device;
// .enabled means the device can decode the resources
    dev.active = device.status.enabled;
    if (acpi_has_method(device.handle, "_SRS"))
    dev.capabilities |= PNP_CONFIGURABLE;
    dev.capabilities |= PNP_READ;
    if (device.flags.dynamic_status && (dev.capabilities & PNP_CONFIGURABLE))
    dev.capabilities |= PNP_WRITE;
    if (device.flags.removable)
    dev.capabilities |= PNP_REMOVABLE;
    if (acpi_has_method(device.handle, "_DIS"))
    dev.capabilities |= PNP_DISABLE;
    strscpy(dev.name, acpi_device_bid(device), sizeof(dev.name));
    if (dev.active)
    pnpacpi_parse_allocated_resource(dev);
    if (dev.capabilities & PNP_CONFIGURABLE)
    pnpacpi_parse_resource_option_data(dev);
    list_for_each_entry(id, &device.pnp.ids, list) {
    if (!strcmp(id.id, pnpid))
    continue;
    if (!ispnpidacpi(id.id))
    continue;
    pnp_add_id(dev, id.id);
    }
// clear out the damaged flags
    if (!dev.active)
    pnp_init_resources(dev);
    error = pnp_add_device(dev);
    if (error) {
    put_device(&dev.dev);
    return error;
    }
    num++;
    return 0;
    }
    static acpi_status __init pnpacpi_add_device_handler(acpi_handle handle,
    u32 lvl, void *context,
    void **rv)
    {
    struct acpi_device *device = acpi_fetch_acpi_dev(handle);
    if (!device)
    return AE_CTRL_DEPTH;
    if (acpi_is_pnp_device(device))
    pnpacpi_add_device(device);
    return AE_OK;
    }
    int pnpacpi_disabled __initdata;
#[no_mangle]
unsafe extern "C" fn pnpacpi_init() -> int __init {
    static int __init pnpacpi_init(void)
    {
    if (acpi_disabled || pnpacpi_disabled) {
    printk(KERN_INFO "pnp: PnP ACPI: disabled\n");
    return 0;
    }
    printk(KERN_INFO "pnp: PnP ACPI init\n");
    pnp_register_protocol(&pnpacpi_protocol);
    acpi_get_devices(core::ptr::null_mut(), pnpacpi_add_device_handler, core::ptr::null_mut(), core::ptr::null_mut());
    printk(KERN_INFO "pnp: PnP ACPI: found %d devices\n", num);
    pnp_platform_devices = 1;
    return 0;
    }
    fs_initcall(pnpacpi_init);
#[no_mangle]
unsafe extern "C" fn pnpacpi_setup(str: *mut c_char) -> int __init {
    static int __init pnpacpi_setup(char *str)
    {
    if (str == core::ptr::null_mut())
    return 1;
    if (!strncmp(str, "off", 3))
    pnpacpi_disabled = 1;
    return 1;
    }
    __setup("pnpacpi=", pnpacpi_setup);
