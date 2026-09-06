//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdca/sdca_hid.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//

#[no_mangle]
unsafe extern "C" fn sdwhid_parse(hid: *mut hid_device) -> c_int {
    static int sdwhid_parse(struct hid_device *hid)
    {
    struct sdca_function_data *function = hid.driver_data;
    unsigned int rsize;
    int ret;
    rsize = le16_to_cpu(function.hid.desc.rpt_desc.wDescriptorLength);
    if (!rsize || rsize > HID_MAX_DESCRIPTOR_SIZE) {
    dev_err(&hid.dev, "invalid size of report descriptor (%u)\n", rsize);
    return -EINVAL;
    }
    ret = hid_parse_report(hid, function.hid.report_desc, rsize);
    if (!ret)
    return 0;
    dev_err(&hid.dev, "parsing report descriptor failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdwhid_start(hid: *mut hid_device) -> c_int {
    static int sdwhid_start(struct hid_device *hid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdwhid_stop(hid: *mut hid_device) {
    static void sdwhid_stop(struct hid_device *hid)
    {
    }
    static int sdwhid_raw_request(struct hid_device *hid, unsigned char reportnum,
    __u8 *buf, size_t len, unsigned char rtype, int reqtype)
    {
    switch (reqtype) {
    case HID_REQ_GET_REPORT:
// not implemented yet
    return 0;
    case HID_REQ_SET_REPORT:
// not implemented yet
    return 0;
    default:
    return -EIO;
    }
    }
#[no_mangle]
unsafe extern "C" fn sdwhid_open(hid: *mut hid_device) -> c_int {
    static int sdwhid_open(struct hid_device *hid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdwhid_close(hid: *mut hid_device) {
    static void sdwhid_close(struct hid_device *hid)
    {
    }
    static const struct hid_ll_driver sdw_hid_driver = {
    .parse = sdwhid_parse,
    .start = sdwhid_start,
    .stop = sdwhid_stop,
    .open = sdwhid_open,
    .close = sdwhid_close,
    .raw_request = sdwhid_raw_request,
    };
//
// sdca_add_hid_device - create a new SDCA HID device
// @interrupt: Pointer to the SDCA interrupt information structure.
//
// Return: Zero on success, and a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_add_hid_device(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_add_hid_device(struct sdca_interrupt *interrupt)
    {
    struct device *dev = interrupt.dev;
    struct sdca_function_data *function = interrupt.function;
    struct hid_device *hid;
    int ret;
    hid = hid_allocate_device();
    if (IS_ERR(hid))
    return PTR_ERR(hid);
    hid.ll_driver = &sdw_hid_driver;
    hid.dev.parent = dev;
    hid.bus = BUS_SDW;
    hid.version = le16_to_cpu(function.hid.desc.bcdHID);
    strscpy(hid.phys, dev_name(dev));
    snprintf(hid.name, sizeof(hid.name), "SDCA %s:%02x",
    function.desc.name, function.desc.adr);
    hid.driver_data = function;
    ret = hid_add_device(hid);
    if (ret && ret != -ENODEV) {
    dev_err(dev, "can't add hid device: %d\n", ret);
    hid_destroy_device(hid);
    return ret;
    }
    interrupt.priv = hid;
    return 0;
    }
    EXPORT_SYMBOL_NS(sdca_add_hid_device, "SND_SOC_SDCA");
//
// sdca_destroy_hid_device - destroy the HID device
// @interrupt: Pointer to the SDCA interrupt information structure.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_destroy_hid_device(interrupt: *mut sdca_interrupt) {
    void sdca_destroy_hid_device(struct sdca_interrupt *interrupt)
    {
    struct hid_device *hid = interrupt.priv;
    hid_destroy_device(hid);
    }
    EXPORT_SYMBOL_NS(sdca_destroy_hid_device, "SND_SOC_SDCA");
//
// sdca_hid_process_report - read a HID event from the device and report
// @interrupt: Pointer to the SDCA interrupt information structure.
//
// Return: Zero on success, and a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_hid_process_report(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_hid_process_report(struct sdca_interrupt *interrupt)
    {
    struct device *dev = interrupt.dev;
    struct hid_device *hid = interrupt.priv;
    void *val __free(kfree) = core::ptr::null_mut();
    int len, ret;
    ret = sdca_ump_get_owner_host(dev, interrupt.function_regmap,
    interrupt.function, interrupt.entity,
    interrupt.control);
    if (ret)
    return ret;
    len = sdca_ump_read_message(dev, interrupt.device_regmap,
    interrupt.function_regmap,
    interrupt.function, interrupt.entity,
    SDCA_CTL_HIDE_HIDTX_MESSAGEOFFSET,
    SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH, &val);
    if (len < 0)
    return len;
    ret = sdca_ump_set_owner_device(dev, interrupt.function_regmap,
    interrupt.function, interrupt.entity,
    interrupt.control);
    if (ret)
    return ret;
    ret = hid_input_report(hid, HID_INPUT_REPORT, val, len, true);
    if (ret < 0) {
    dev_err(dev, "failed to report hid event: %d\n", ret);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(sdca_hid_process_report, "SND_SOC_SDCA");
