//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/of.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// USB of helper code
//

    static const char *const usbphy_modes[] = {
    [USBPHY_INTERFACE_MODE_UNKNOWN]	= "",
    [USBPHY_INTERFACE_MODE_UTMI]	= "utmi",
    [USBPHY_INTERFACE_MODE_UTMIW]	= "utmi_wide",
    [USBPHY_INTERFACE_MODE_ULPI]	= "ulpi",
    [USBPHY_INTERFACE_MODE_SERIAL]	= "serial",
    [USBPHY_INTERFACE_MODE_HSIC]	= "hsic",
    };
//
// of_usb_get_phy_mode - Get phy mode for given device_node
// @np:	Pointer to the given device_node
//
// The function gets phy interface string from property 'phy_type',
// and returns the corresponding enum usb_phy_interface
//
#[no_mangle]
pub unsafe extern "C" fn of_usb_get_phy_mode(np: *mut device_node) -> enum usb_phy_interface {
    enum usb_phy_interface of_usb_get_phy_mode(struct device_node *np)
    {
    const char *phy_type;
    int err, i;
    err = of_property_read_string(np, "phy_type", &phy_type);
    if (err < 0)
    return USBPHY_INTERFACE_MODE_UNKNOWN;
    for (i = 0; i < ARRAY_SIZE(usbphy_modes); i++)
    if (!strcmp(phy_type, usbphy_modes[i]))
    return i;
    return USBPHY_INTERFACE_MODE_UNKNOWN;
    }
    EXPORT_SYMBOL_GPL(of_usb_get_phy_mode);
