//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8822cu.c
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
// Copyright(c) 2018-2019  Realtek Corporation
//

    static const struct usb_device_id rtw_8822cu_id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xc82c, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xc812, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xc82e, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xd820, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xd82b, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) },
    { USB_DEVICE_AND_INTERFACE_INFO(0x13b1, 0x0043, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) }, /* Alpha - Alpha */
    { USB_DEVICE_AND_INTERFACE_INFO(0x2001, 0x3329, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8822c_hw_spec) }, /* D-Link AC13U rev. A1 */
    {},
    };
    MODULE_DEVICE_TABLE(usb, rtw_8822cu_id_table);
    static int rtw8822cu_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return rtw_usb_probe(intf, id);
    }
    static struct usb_driver rtw_8822cu_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtw_8822cu_id_table,
    .probe = rtw8822cu_probe,
    .disconnect = rtw_usb_disconnect,
    };
    module_usb_driver(rtw_8822cu_driver);
    MODULE_AUTHOR("Realtek Corporation");
    MODULE_DESCRIPTION("Realtek 802.11ac wireless 8822cu driver");
    MODULE_LICENSE("Dual BSD/GPL");
