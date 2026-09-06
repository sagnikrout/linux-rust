//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw88/rtw8821cs.c
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
// Copyright(c) Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

    static const struct sdio_device_id rtw_8821cs_id_table[] =  {
    {
    SDIO_DEVICE(SDIO_VENDOR_ID_REALTEK,
    SDIO_DEVICE_ID_REALTEK_RTW8821CS),
    .driver_data = (kernel_ulong_t)&rtw8821c_hw_spec,
    },
    {}
    };
    MODULE_DEVICE_TABLE(sdio, rtw_8821cs_id_table);
    static struct sdio_driver rtw_8821cs_driver = {
    .name = KBUILD_MODNAME,
    .probe = rtw_sdio_probe,
    .remove = rtw_sdio_remove,
    .shutdown = rtw_sdio_shutdown,
    .id_table = rtw_8821cs_id_table,
    .drv = {
    .pm = &rtw_sdio_pm_ops,
    }
    };
    module_sdio_driver(rtw_8821cs_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Realtek 802.11ac wireless 8821cs driver");
    MODULE_LICENSE("Dual BSD/GPL");
