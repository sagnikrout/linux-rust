//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/ucsi/ucsi_acpi.c
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
// UCSI ACPI driver
//
// Copyright (C) 2017, Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

pub const UCSI_DSM_FUNC_WRITE: c_int = 1;
pub const UCSI_DSM_FUNC_READ: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_acpi {
    pub dev: *mut device,
    pub ucsi: *mut ucsi,
    pub base: *mut c_void,
    pub check_bogus_event: bool,
    pub guid: guid_t,
    pub cmd: u64,
}

#[no_mangle]
unsafe extern "C" fn ucsi_acpi_dsm(ua: *mut ucsi_acpi, func: c_int) -> c_int {
    static int ucsi_acpi_dsm(struct ucsi_acpi *ua, int func)
    {
    union acpi_object *obj;
    obj = acpi_evaluate_dsm(ACPI_HANDLE(ua.dev), &ua.guid, 1, func,
    core::ptr::null_mut());
    if (!obj) {
    dev_err(ua.dev, "%s: failed to evaluate _DSM %d\n",
    __func__, func);
    return -EIO;
    }
    ACPI_FREE(obj);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_read_version(ucsi: *mut ucsi, version: *mut u16) -> c_int {
    static int ucsi_acpi_read_version(struct ucsi *ucsi, u16 *version)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    int ret;
    ret = ucsi_acpi_dsm(ua, UCSI_DSM_FUNC_READ);
    if (ret)
    return ret;
    memcpy(version, ua.base + UCSI_VERSION, sizeof(*version));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_read_cci(ucsi: *mut ucsi, cci: *mut u32) -> c_int {
    static int ucsi_acpi_read_cci(struct ucsi *ucsi, u32 *cci)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    memcpy(cci, ua.base + UCSI_CCI, sizeof(*cci));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_poll_cci(ucsi: *mut ucsi, cci: *mut u32) -> c_int {
    static int ucsi_acpi_poll_cci(struct ucsi *ucsi, u32 *cci)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    int ret;
    ret = ucsi_acpi_dsm(ua, UCSI_DSM_FUNC_READ);
    if (ret)
    return ret;
    return ucsi_acpi_read_cci(ucsi, cci);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_read_message_in(ucsi: *mut ucsi, val: *mut c_void, val_len: usize) -> c_int {
    static int ucsi_acpi_read_message_in(struct ucsi *ucsi, void *val, size_t val_len)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    memcpy(val, ua.base + UCSI_MESSAGE_IN, val_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_write_message_out(ucsi: *mut ucsi, data: *mut c_void, data_len: usize) -> c_int {
    static int ucsi_acpi_write_message_out(struct ucsi *ucsi, void *data, size_t data_len)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    if (!data || !data_len)
    return -EINVAL;
    if (ucsi.version <= UCSI_VERSION_1_2)
    memcpy(ua.base + UCSI_MESSAGE_OUT, data, data_len);
    else
    memcpy(ua.base + UCSIv2_MESSAGE_OUT, data, data_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_async_control(ucsi: *mut ucsi, command: u64) -> c_int {
    static int ucsi_acpi_async_control(struct ucsi *ucsi, u64 command)
    {
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    memcpy(ua.base + UCSI_CONTROL, &command, sizeof(command));
    ua.cmd = command;
    return ucsi_acpi_dsm(ua, UCSI_DSM_FUNC_WRITE);
    }
    static const struct ucsi_operations ucsi_acpi_ops = {
    .read_version = ucsi_acpi_read_version,
    .read_cci = ucsi_acpi_read_cci,
    .poll_cci = ucsi_acpi_poll_cci,
    .read_message_in = ucsi_acpi_read_message_in,
    .write_message_out = ucsi_acpi_write_message_out,
    .sync_control = ucsi_sync_control_common,
    .async_control = ucsi_acpi_async_control
    };
    static int ucsi_gram_sync_control(struct ucsi *ucsi, u64 command, u32 *cci,
    void *val, size_t len, void *msg_out,
    size_t msg_out_size)
    {
    u16 bogus_change = UCSI_CONSTAT_POWER_LEVEL_CHANGE |
    UCSI_CONSTAT_PDOS_CHANGE;
    struct ucsi_acpi *ua = ucsi_get_drvdata(ucsi);
    int ret;
    ret = ucsi_sync_control_common(ucsi, command, cci, val, len,
    msg_out, msg_out_size);
    if (ret < 0)
    return ret;
    if (UCSI_COMMAND(ua.cmd) == UCSI_GET_PDOS &&
    ua.cmd & UCSI_GET_PDOS_PARTNER_PDO(1) &&
    ua.cmd & UCSI_GET_PDOS_SRC_PDOS)
    ua.check_bogus_event = true;
    if (UCSI_COMMAND(ua.cmd) == UCSI_GET_CONNECTOR_STATUS &&
    ua.check_bogus_event) {
// Clear the bogus change
    if (*(u16 *)val == bogus_change)
// (u16 *)val = 0;
    ua.check_bogus_event = false;
    }
    return ret;
    }
    static const struct ucsi_operations ucsi_gram_ops = {
    .read_version = ucsi_acpi_read_version,
    .read_cci = ucsi_acpi_read_cci,
    .poll_cci = ucsi_acpi_poll_cci,
    .read_message_in = ucsi_acpi_read_message_in,
    .sync_control = ucsi_gram_sync_control,
    .async_control = ucsi_acpi_async_control
    };
    static const struct dmi_system_id ucsi_acpi_quirks[] = {
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LG Electronics"),
    DMI_MATCH(DMI_PRODUCT_FAMILY, "LG gram PC"),
    DMI_MATCH(DMI_PRODUCT_NAME, "90Q"),
    },
    .driver_data = (void *)&ucsi_gram_ops,
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void ucsi_acpi_notify(acpi_handle handle, u32 event, void *data)
    {
    struct ucsi_acpi *ua = data;
    u32 cci;
    int ret;
    ret = ua.ucsi.ops.read_cci(ua.ucsi, &cci);
    if (ret)
    return;
    ucsi_notify_common(ua.ucsi, cci);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int ucsi_acpi_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&pdev.dev);
    const struct ucsi_operations *ops = &ucsi_acpi_ops;
    const struct dmi_system_id *id;
    struct ucsi_acpi *ua;
    struct resource *res;
    acpi_status status;
    int ret;
    if (adev.dep_unmet)
    return -EPROBE_DEFER;
    ua = devm_kzalloc(&pdev.dev, sizeof(*ua), GFP_KERNEL);
    if (!ua)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "missing memory resource\n");
    return -ENODEV;
    }
    ua.base = devm_memremap(&pdev.dev, res.start, resource_size(res), MEMREMAP_WB);
    if (IS_ERR(ua.base))
    return PTR_ERR(ua.base);
    ret = guid_parse(UCSI_DSM_UUID, &ua.guid);
    if (ret)
    return ret;
    ua.dev = &pdev.dev;
    id = dmi_first_match(ucsi_acpi_quirks);
    if (id)
    ops = id.driver_data;
    ua.ucsi = ucsi_create(&pdev.dev, ops);
    if (IS_ERR(ua.ucsi))
    return PTR_ERR(ua.ucsi);
    ucsi_set_drvdata(ua.ucsi, ua);
    status = acpi_install_notify_handler(ACPI_HANDLE(&pdev.dev),
    ACPI_DEVICE_NOTIFY,
    ucsi_acpi_notify, ua);
    if (ACPI_FAILURE(status)) {
    dev_err(&pdev.dev, "failed to install notify handler\n");
    ucsi_destroy(ua.ucsi);
    return -ENODEV;
    }
    ret = ucsi_register(ua.ucsi);
    if (ret) {
    acpi_remove_notify_handler(ACPI_HANDLE(&pdev.dev),
    ACPI_DEVICE_NOTIFY,
    ucsi_acpi_notify);
    ucsi_destroy(ua.ucsi);
    return ret;
    }
    platform_set_drvdata(pdev, ua);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_remove(pdev: *mut platform_device) {
    static void ucsi_acpi_remove(struct platform_device *pdev)
    {
    struct ucsi_acpi *ua = platform_get_drvdata(pdev);
    ucsi_unregister(ua.ucsi);
    ucsi_destroy(ua.ucsi);
    acpi_remove_notify_handler(ACPI_HANDLE(&pdev.dev), ACPI_DEVICE_NOTIFY,
    ucsi_acpi_notify);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_suspend(dev: *mut device) -> c_int {
    static int ucsi_acpi_suspend(struct device *dev)
    {
    struct ucsi_acpi *ua = dev_get_drvdata(dev);
    return ucsi_suspend(ua.ucsi);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_acpi_resume(dev: *mut device) -> c_int {
    static int ucsi_acpi_resume(struct device *dev)
    {
    struct ucsi_acpi *ua = dev_get_drvdata(dev);
    return ucsi_resume(ua.ucsi);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ucsi_acpi_pm_ops, ucsi_acpi_suspend,
    ucsi_acpi_resume);
    static const struct acpi_device_id ucsi_acpi_match[] = {
    { "PNP0CA0", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, ucsi_acpi_match);
    static struct platform_driver ucsi_acpi_platform_driver = {
    .driver = {
    .name = "ucsi_acpi",
    .pm = pm_ptr(&ucsi_acpi_pm_ops),
    .acpi_match_table = ACPI_PTR(ucsi_acpi_match),
    },
    .probe = ucsi_acpi_probe,
    .remove = ucsi_acpi_remove,
    };
    module_platform_driver(ucsi_acpi_platform_driver);
    MODULE_AUTHOR("Heikki Krogerus <heikki.krogerus@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("UCSI ACPI driver");
