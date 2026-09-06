//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8814ae.c
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

    static const struct pci_device_id rtw_8814ae_id_table[] = {
    {
    PCI_DEVICE(PCI_VENDOR_ID_REALTEK, 0x8813),
    .driver_data = (kernel_ulong_t)&rtw8814a_hw_spec
    },
    {}
    };
    MODULE_DEVICE_TABLE(pci, rtw_8814ae_id_table);
    static struct pci_driver rtw_8814ae_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtw_8814ae_id_table,
    .probe = rtw_pci_probe,
    .remove = rtw_pci_remove,
    .driver.pm = &rtw_pm_ops,
    .shutdown = rtw_pci_shutdown,
    };
    module_pci_driver(rtw_8814ae_driver);
    MODULE_AUTHOR("Bitterblue Smith <rtl8821cerfe2@gmail.com>");
    MODULE_DESCRIPTION("Realtek 802.11ac wireless 8814ae driver");
    MODULE_LICENSE("Dual BSD/GPL");
