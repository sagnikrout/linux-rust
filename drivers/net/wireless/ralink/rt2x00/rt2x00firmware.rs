//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00firmware.c
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
    Copyright (C) 2004 - 2009 Ivo van Doorn <IvDoorn@gmail.com>
    Copyright (C) 2004 - 2009 Gertjan van Wingerde <gwingerde@gmail.com>
    <http://rt2x00.serialmonkey.com>
//
    Module: rt2x00lib
    Abstract: rt2x00 firmware loading routines.
//

#[no_mangle]
unsafe extern "C" fn rt2x00lib_request_firmware(rt2x00dev: *mut rt2x00_dev) -> c_int {
    static int rt2x00lib_request_firmware(struct rt2x00_dev *rt2x00dev)
    {
    struct device *device = wiphy_dev(rt2x00dev.hw.wiphy);
    const struct firmware *fw;
    char *fw_name;
    int retval;
//
// Read correct firmware from harddisk.
//
    fw_name = rt2x00dev.ops.lib.get_firmware_name(rt2x00dev);
    if (!fw_name) {
    rt2x00_err(rt2x00dev,
    "Invalid firmware filename\n"
    "Please file bug report to %s\n", DRV_PROJECT);
    return -EINVAL;
    }
    rt2x00_info(rt2x00dev, "Loading firmware file '%s'\n", fw_name);
    retval = request_firmware(&fw, fw_name, device);
    if (retval) {
    rt2x00_err(rt2x00dev, "Failed to request Firmware\n");
    return retval;
    }
    if (!fw || !fw.size || !fw.data) {
    rt2x00_err(rt2x00dev, "Failed to read Firmware\n");
    release_firmware(fw);
    return -ENOENT;
    }
    rt2x00_info(rt2x00dev, "Firmware detected - version: %d.%d\n",
    fw.data[fw.size - 4], fw.data[fw.size - 3]);
    snprintf(rt2x00dev.hw.wiphy.fw_version,
    sizeof(rt2x00dev.hw.wiphy.fw_version), "%d.%d",
    fw.data[fw.size - 4], fw.data[fw.size - 3]);
    retval = rt2x00dev.ops.lib.check_firmware(rt2x00dev, fw.data, fw.size);
    switch (retval) {
    case FW_OK:
    break;
    case FW_BAD_CRC:
    rt2x00_err(rt2x00dev, "Firmware checksum error\n");
    goto exit;
    case FW_BAD_LENGTH:
    rt2x00_err(rt2x00dev, "Invalid firmware file length (len=%zu)\n",
    fw.size);
    goto exit;
    case FW_BAD_VERSION:
    rt2x00_err(rt2x00dev, "Current firmware does not support detected chipset\n");
    goto exit;
    }
    rt2x00dev.fw = fw;
    return 0;
    exit:
    release_firmware(fw);
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn rt2x00lib_load_firmware(rt2x00dev: *mut rt2x00_dev) -> c_int {
    int rt2x00lib_load_firmware(struct rt2x00_dev *rt2x00dev)
    {
    int retval;
    if (!rt2x00_has_cap_flag(rt2x00dev, REQUIRE_FIRMWARE))
    return 0;
    if (!rt2x00dev.fw) {
    retval = rt2x00lib_request_firmware(rt2x00dev);
    if (retval)
    return retval;
    }
//
// Send firmware to the device.
//
    retval = rt2x00dev.ops.lib.load_firmware(rt2x00dev,
    rt2x00dev.fw.data,
    rt2x00dev.fw.size);
//
// When the firmware is uploaded to the hardware the LED
// association status might have been triggered, for correct
// LED handling it should now be reset.
//
    rt2x00leds_led_assoc(rt2x00dev, false);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn rt2x00lib_free_firmware(rt2x00dev: *mut rt2x00_dev) {
    void rt2x00lib_free_firmware(struct rt2x00_dev *rt2x00dev)
    {
    release_firmware(rt2x00dev.fw);
    rt2x00dev.fw = core::ptr::null_mut();
    }
