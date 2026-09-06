//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/cros_usbpd_notify.c
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
// Copyright 2020 Google LLC
//
// This driver serves as the receiver of cros_ec PD host events.
//

    static BLOCKING_NOTIFIER_HEAD(cros_usbpd_notifier_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_usbpd_notify_data {
    pub dev: *mut device,
    pub ec: *mut cros_ec_device,
    pub nb: notifier_block,
}

//
// cros_usbpd_register_notify - Register a notifier callback for PD events.
// @nb: Notifier block pointer to register
//
// On ACPI platforms this corresponds to host events on the ECPD
// "GOOG0003" ACPI device. On non-ACPI platforms this will filter mkbp events
// for USB PD events.
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn cros_usbpd_register_notify(nb: *mut notifier_block) -> c_int {
    int cros_usbpd_register_notify(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&cros_usbpd_notifier_list,
    nb);
    }
    EXPORT_SYMBOL_GPL(cros_usbpd_register_notify);
//
// cros_usbpd_unregister_notify - Unregister notifier callback for PD events.
// @nb: Notifier block pointer to unregister
//
// Unregister a notifier callback that was previously registered with
// cros_usbpd_register_notify().
//
#[no_mangle]
pub unsafe extern "C" fn cros_usbpd_unregister_notify(nb: *mut notifier_block) {
    void cros_usbpd_unregister_notify(struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&cros_usbpd_notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(cros_usbpd_unregister_notify);
    static void cros_usbpd_get_event_and_notify(struct device  *dev,
    struct cros_ec_device *ec_dev)
    {
    struct ec_response_host_event_status host_event_status;
    let mut event: u32 = 0;
    int ret;
//
// We still send a 0 event out to older devices which don't
// have the updated device heirarchy.
//
    if (!ec_dev) {
    dev_dbg(dev,
    "EC device inaccessible; sending 0 event status.\n");
    goto send_notify;
    }
// Check for PD host events on EC.
    ret = cros_ec_cmd(ec_dev, 0, EC_CMD_PD_HOST_EVENT_STATUS,
    core::ptr::null_mut(), 0, &host_event_status, sizeof(host_event_status));
    if (ret < 0) {
    dev_warn(dev, "Can't get host event status (err: %d)\n", ret);
    goto send_notify;
    }
    event = host_event_status.status;
    send_notify:
    blocking_notifier_call_chain(&cros_usbpd_notifier_list, event, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_acpi(device: acpi_handle, event: u32, data: *mut c_void) {
    static void cros_usbpd_notify_acpi(acpi_handle device, u32 event, void *data)
    {
    struct cros_usbpd_notify_data *pdnotify = data;
    cros_usbpd_get_event_and_notify(pdnotify.dev, pdnotify.ec);
    }
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_probe_acpi(pdev: *mut platform_device) -> c_int {
    static int cros_usbpd_notify_probe_acpi(struct platform_device *pdev)
    {
    struct cros_usbpd_notify_data *pdnotify;
    struct device *dev = &pdev.dev;
    struct acpi_device *adev, *parent_adev;
    struct cros_ec_device *ec_dev;
    struct fwnode_handle *parent_fwnode;
    acpi_status status;
    adev = ACPI_COMPANION(dev);
    pdnotify = devm_kzalloc(dev, sizeof(*pdnotify), GFP_KERNEL);
    if (!pdnotify)
    return -ENOMEM;
// Get the EC device pointer needed to talk to the EC.
    ec_dev = dev_get_drvdata(dev.parent);
    if (!ec_dev) {
//
// We continue even for older devices which don't have the
// correct device heirarchy, namely, GOOG0003 is a child
// of GOOG0004. If GOOG0003 is a child of GOOG0004 and we
// can't get a pointer to the Chrome EC device, defer the
// probe function.
//
    parent_fwnode = fwnode_get_parent(dev.fwnode);
    if (parent_fwnode) {
    parent_adev = to_acpi_device_node(parent_fwnode);
    if (parent_adev &&
    acpi_dev_hid_match(parent_adev, CREC_DRV_NAME)) {
    return -EPROBE_DEFER;
    }
    }
    dev_warn(dev, "Couldn't get Chrome EC device pointer.\n");
    }
    pdnotify.dev = dev;
    pdnotify.ec = ec_dev;
    status = acpi_install_notify_handler(adev.handle,
    ACPI_ALL_NOTIFY,
    cros_usbpd_notify_acpi,
    pdnotify);
    if (ACPI_FAILURE(status)) {
    dev_warn(dev, "Failed to register notify handler %08x\n",
    status);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_remove_acpi(pdev: *mut platform_device) {
    static void cros_usbpd_notify_remove_acpi(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct acpi_device *adev = ACPI_COMPANION(dev);
    acpi_remove_notify_handler(adev.handle, ACPI_ALL_NOTIFY,
    cros_usbpd_notify_acpi);
    }
    static const struct acpi_device_id cros_usbpd_notify_acpi_device_ids[] = {
    { ACPI_DRV_NAME, 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, cros_usbpd_notify_acpi_device_ids);
    static struct platform_driver cros_usbpd_notify_acpi_driver = {
    .driver = {
    .name = DRV_NAME_PLAT_ACPI,
    .acpi_match_table = cros_usbpd_notify_acpi_device_ids,
    },
    .probe = cros_usbpd_notify_probe_acpi,
    .remove = cros_usbpd_notify_remove_acpi,
    };

    static int cros_usbpd_notify_plat(struct notifier_block *nb,
    unsigned long queued_during_suspend,
    void *data)
    {
    struct cros_usbpd_notify_data *pdnotify = container_of(nb,
    struct cros_usbpd_notify_data, nb);
    struct cros_ec_device *ec_dev = (struct cros_ec_device *)data;
    let mut host_event: u32 = cros_ec_get_host_event(ec_dev);
    if (!host_event)
    return NOTIFY_DONE;
    if (host_event & (EC_HOST_EVENT_MASK(EC_HOST_EVENT_PD_MCU) |
    EC_HOST_EVENT_MASK(EC_HOST_EVENT_USB_MUX))) {
    cros_usbpd_get_event_and_notify(pdnotify.dev, ec_dev);
    return NOTIFY_OK;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_probe_plat(pdev: *mut platform_device) -> c_int {
    static int cros_usbpd_notify_probe_plat(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cros_ec_dev *ecdev = dev_get_drvdata(dev.parent);
    struct cros_usbpd_notify_data *pdnotify;
    int ret;
    pdnotify = devm_kzalloc(dev, sizeof(*pdnotify), GFP_KERNEL);
    if (!pdnotify)
    return -ENOMEM;
    pdnotify.dev = dev;
    pdnotify.ec = ecdev.ec_dev;
    pdnotify.nb.notifier_call = cros_usbpd_notify_plat;
    dev_set_drvdata(dev, pdnotify);
    ret = blocking_notifier_chain_register(&ecdev.ec_dev.event_notifier,
    &pdnotify.nb);
    if (ret < 0) {
    dev_err(dev, "Failed to register notifier\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_remove_plat(pdev: *mut platform_device) {
    static void cros_usbpd_notify_remove_plat(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cros_ec_dev *ecdev = dev_get_drvdata(dev.parent);
    struct cros_usbpd_notify_data *pdnotify =
    (struct cros_usbpd_notify_data *)dev_get_drvdata(dev);
    blocking_notifier_chain_unregister(&ecdev.ec_dev.event_notifier,
    &pdnotify.nb);
    }
    static const struct platform_device_id cros_usbpd_notify_id[] = {
    { .name = DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(platform, cros_usbpd_notify_id);
    static struct platform_driver cros_usbpd_notify_plat_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    .probe = cros_usbpd_notify_probe_plat,
    .remove = cros_usbpd_notify_remove_plat,
    .id_table = cros_usbpd_notify_id,
    };
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_init() -> int __init {
    static int __init cros_usbpd_notify_init(void)
    {
    int ret;
    ret = platform_driver_register(&cros_usbpd_notify_plat_driver);
    if (ret < 0)
    return ret;

    ret = platform_driver_register(&cros_usbpd_notify_acpi_driver);
    if (ret) {
    platform_driver_unregister(&cros_usbpd_notify_plat_driver);
    return ret;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_usbpd_notify_exit() -> void __exit {
    static void __exit cros_usbpd_notify_exit(void)
    {

    platform_driver_unregister(&cros_usbpd_notify_acpi_driver);

    platform_driver_unregister(&cros_usbpd_notify_plat_driver);
    }
    module_init(cros_usbpd_notify_init);
    module_exit(cros_usbpd_notify_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ChromeOS power delivery notifier device");
    MODULE_AUTHOR("Jon Flatley <jflat@chromium.org>");
