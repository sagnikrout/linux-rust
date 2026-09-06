//! Automatically rewritten from C to Rust
//! Source: net/wireless/ethtool.c
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

#[no_mangle]
pub unsafe extern "C" fn cfg80211_get_drvinfo(dev: *mut net_device, info: *mut ethtool_drvinfo) {
    void cfg80211_get_drvinfo(struct net_device *dev, struct ethtool_drvinfo *info)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct device *pdev = wiphy_dev(wdev.wiphy);
    if (pdev.driver)
    strscpy(info.driver, pdev.driver.name,
    sizeof(info.driver));
    else
    strscpy(info.driver, "N/A", sizeof(info.driver));
    strscpy(info.version, init_utsname().release, sizeof(info.version));
    if (wdev.wiphy.fw_version[0])
    strscpy(info.fw_version, wdev.wiphy.fw_version,
    sizeof(info.fw_version));
    else
    strscpy(info.fw_version, "N/A", sizeof(info.fw_version));
    strscpy(info.bus_info, dev_name(pdev),
    sizeof(info.bus_info));
    }
    EXPORT_SYMBOL(cfg80211_get_drvinfo);
