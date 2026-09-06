//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/wmi/sbl-fw-update.c
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
// Slim Bootloader(SBL) firmware update signaling driver
//
// Slim Bootloader is a small, open-source, non UEFI compliant, boot firmware
// optimized for running on certain Intel platforms.
//
// SBL exposes an ACPI-WMI device via /sys/bus/wmi/devices/<INTEL_WMI_SBL_GUID>.
// This driver further adds "firmware_update_request" device attribute.
// This attribute normally has a value of 0 and userspace can signal SBL
// to update firmware, on next reboot, by writing a value of 1.
//
// More details of SBL firmware update process is available at:
// https://slimbootloader.github.io/security/firmware-update.html
//

#[no_mangle]
unsafe extern "C" fn get_fwu_request(dev: *mut device, out: *mut u32) -> c_int {
    static int get_fwu_request(struct device *dev, u32 *out)
    {
    struct wmi_buffer buffer;
    __le32 *result;
    int ret;
    ret = wmidev_query_block(to_wmi_device(dev), 0, &buffer, sizeof(*result));
    if (ret < 0)
    return ret;
    result = buffer.data;
// out = le32_to_cpu(*result);
    kfree(result);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_fwu_request(dev: *mut device, in: u32) -> c_int {
    static int set_fwu_request(struct device *dev, u32 in)
    {
    let mut value: __le32 = cpu_to_le32(in);
    struct wmi_buffer buffer = {
    .length = sizeof(value),
    .data = &value,
    };
    return wmidev_set_block(to_wmi_device(dev), 0, &buffer);
    }
    static ssize_t firmware_update_request_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u32 val;
    int ret;
    ret = get_fwu_request(dev, &val);
    if (ret)
    return ret;
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t firmware_update_request_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    unsigned int val;
    int ret;
    ret = kstrtouint(buf, 0, &val);
    if (ret)
    return ret;
// May later be extended to support values other than 0 and 1
    if (val > 1)
    return -ERANGE;
    ret = set_fwu_request(dev, val);
    if (ret)
    return ret;
    return count;
    }
    static DEVICE_ATTR_RW(firmware_update_request);
    static struct attribute *firmware_update_attrs[] = {
    &dev_attr_firmware_update_request.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(firmware_update);
    static int intel_wmi_sbl_fw_update_probe(struct wmi_device *wdev,
    const void *context)
    {
    dev_info(&wdev.dev, "Slim Bootloader signaling driver attached\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_wmi_sbl_fw_update_remove(wdev: *mut wmi_device) {
    static void intel_wmi_sbl_fw_update_remove(struct wmi_device *wdev)
    {
    dev_info(&wdev.dev, "Slim Bootloader signaling driver removed\n");
    }
    static const struct wmi_device_id intel_wmi_sbl_id_table[] = {
    { .guid_string = INTEL_WMI_SBL_GUID },
    {}
    };
    MODULE_DEVICE_TABLE(wmi, intel_wmi_sbl_id_table);
    static struct wmi_driver intel_wmi_sbl_fw_update_driver = {
    .driver = {
    .name = "intel-wmi-sbl-fw-update",
    .dev_groups = firmware_update_groups,
    },
    .probe = intel_wmi_sbl_fw_update_probe,
    .remove = intel_wmi_sbl_fw_update_remove,
    .id_table = intel_wmi_sbl_id_table,
    .no_singleton = true,
    };
    module_wmi_driver(intel_wmi_sbl_fw_update_driver);
    MODULE_AUTHOR("Jithu Joseph <jithu.joseph@intel.com>");
    MODULE_DESCRIPTION("Slim Bootloader firmware update signaling driver");
    MODULE_LICENSE("GPL v2");
