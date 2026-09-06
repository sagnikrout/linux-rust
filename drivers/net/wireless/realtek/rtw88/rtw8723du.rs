//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8723du.c
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

    static const struct usb_device_id rtw_8723du_id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(RTW_USB_VENDOR_ID_REALTEK, 0xd723, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8723d_hw_spec) }, /* 8723DU 1*1 */
    { USB_DEVICE_AND_INTERFACE_INFO(0x7392, 0xd611, 0xff, 0xff, 0xff),
    .driver_info = (kernel_ulong_t)&(rtw8723d_hw_spec) }, /* Edimax EW-7611ULB V2 */
    { },
    };
    MODULE_DEVICE_TABLE(usb, rtw_8723du_id_table);
    static int rtw8723du_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return rtw_usb_probe(intf, id);
    }
    static struct usb_driver rtw_8723du_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtw_8723du_id_table,
    .probe = rtw8723du_probe,
    .disconnect = rtw_usb_disconnect,
    };
    module_usb_driver(rtw_8723du_driver);
    MODULE_AUTHOR("Hans Ulli Kroll <linux@ulli-kroll.de>");
    MODULE_DESCRIPTION("Realtek 802.11n wireless 8723du driver");
    MODULE_LICENSE("Dual BSD/GPL");
