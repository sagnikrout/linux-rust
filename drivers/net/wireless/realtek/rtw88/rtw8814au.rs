//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8814au.c
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
// Copyright(c) 2025  Realtek Corporation
//

    static const struct usb_device_id rtw_8814au_id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0x8813, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x056e, 0x400b, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x056e, 0x400d, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0846, 0x9054, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0b05, 0x1817, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0b05, 0x1852, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0b05, 0x1853, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x0e66, 0x0026, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x2001, 0x331a, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x20f4, 0x809a, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x20f4, 0x809b, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x2357, 0x0106, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xa834, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xa833, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8814a_hw_spec) },
    {},
    };
    MODULE_DEVICE_TABLE(usb, rtw_8814au_id_table);
    static struct usb_driver rtw_8814au_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtw_8814au_id_table,
    .probe = rtw_usb_probe,
    .disconnect = rtw_usb_disconnect,
    };
    module_usb_driver(rtw_8814au_driver);
    MODULE_AUTHOR("Bitterblue Smith <rtl8821cerfe2@gmail.com>");
    MODULE_DESCRIPTION("Realtek 802.11ac wireless 8814au driver");
    MODULE_LICENSE("Dual BSD/GPL");
