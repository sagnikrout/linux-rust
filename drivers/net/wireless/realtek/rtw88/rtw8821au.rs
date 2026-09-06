//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8821au.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2024  Realtek Corporation
//

    static const struct usb_device_id rtw_8821au_id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x0811, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x0820, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x0821, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x8822, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x0823, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xa811, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0411, 0x0242, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Buffalo */
    { USB_DEVICE_AND_INTERFACE_INFO(0x0411, 0x029b, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Buffalo */
    { USB_DEVICE_AND_INTERFACE_INFO(0x04bb, 0x0953, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* I-O DATA */
    { USB_DEVICE_AND_INTERFACE_INFO(0x056e, 0x4007, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* ELECOM */
    { USB_DEVICE_AND_INTERFACE_INFO(0x056e, 0x400e, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* ELECOM */
    { USB_DEVICE_AND_INTERFACE_INFO(0x056e, 0x400f, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* ELECOM */
    { USB_DEVICE_AND_INTERFACE_INFO(0x0846, 0x9052, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Netgear */
    { USB_DEVICE_AND_INTERFACE_INFO(0x0e66, 0x0023, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* HAWKING */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2001, 0x3314, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* D-Link */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2001, 0x3318, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* D-Link */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2019, 0xab32, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Planex */
    { USB_DEVICE_AND_INTERFACE_INFO(0x20f4, 0x804b, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* TRENDnet */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2357, 0x011e, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* TP Link */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2357, 0x011f, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* TP Link */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2357, 0x0120, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* TP Link */
    { USB_DEVICE_AND_INTERFACE_INFO(0x3823, 0x6249, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Obihai */
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xa811, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Edimax */
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xa812, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Edimax */
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xa813, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Edimax */
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xb611, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8821a_hw_spec) }, /* Edimax */
    {},
    };
    MODULE_DEVICE_TABLE(usb, rtw_8821au_id_table);
    static struct usb_driver rtw_8821au_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtw_8821au_id_table,
    .probe = rtw_usb_probe,
    .disconnect = rtw_usb_disconnect,
    };
    module_usb_driver(rtw_8821au_driver);
    MODULE_AUTHOR("Bitterblue Smith <rtl8821cerfe2@gmail.com>");
    MODULE_DESCRIPTION("Realtek 802.11ac wireless 8821au/8811au driver");
    MODULE_LICENSE("Dual BSD/GPL");
