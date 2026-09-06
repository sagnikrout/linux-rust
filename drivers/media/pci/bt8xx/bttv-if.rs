//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/bt8xx/bttv-if.c
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
    bttv-if.c  --  old gpio interface to other kernel modules
    don't use in new code, will go away in 2.7
    have a look at bttv-gpio.c instead.
    bttv - Bt848 frame grabber driver
    Copyright (C) 1996,97,98 Ralph  Metzler (rjkm@thp.uni-koeln.de)
    & Marcus Metzler (mocm@thp.uni-koeln.de)
    (c) 1999-2003 Gerd Knorr <kraxel@bytesex.org>
//

    EXPORT_SYMBOL(bttv_get_pcidev);
    EXPORT_SYMBOL(bttv_gpio_enable);
    EXPORT_SYMBOL(bttv_read_gpio);
    EXPORT_SYMBOL(bttv_write_gpio);
// -----------------------------------------------------------------------
// Exported functions - for other modules which want to access the
// gpio ports (IR for example)
// see bttv.h for comments
#[no_mangle]
pub unsafe extern "C" fn bttv_get_pcidev(card: c_uint) -> *mut pci_dev {
    struct pci_dev* bttv_get_pcidev(unsigned int card)
    {
    if (card >= bttv_num)
    return core::ptr::null_mut();
    if (!bttvs[card])
    return core::ptr::null_mut();
    return bttvs[card].c.pci;
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_gpio_enable(card: c_uint, mask: c_ulong, data: c_ulong) -> c_int {
    int bttv_gpio_enable(unsigned int card, unsigned long mask, unsigned long data)
    {
    struct bttv *btv;
    if (card >= bttv_num) {
    return -EINVAL;
    }
    btv = bttvs[card];
    if (!btv)
    return -ENODEV;
    gpio_inout(mask,data);
    if (bttv_gpio)
    bttv_gpio_tracking(btv,"extern enable");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_read_gpio(card: c_uint, data: *mut c_ulong) -> c_int {
    int bttv_read_gpio(unsigned int card, unsigned long *data)
    {
    struct bttv *btv;
    if (card >= bttv_num) {
    return -EINVAL;
    }
    btv = bttvs[card];
    if (!btv)
    return -ENODEV;
    if(btv.shutdown) {
    return -ENODEV;
    }
// prior setting BT848_GPIO_REG_INP is (probably) not needed
    because we set direct input on init */
// data = gpio_read();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_write_gpio(card: c_uint, mask: c_ulong, data: c_ulong) -> c_int {
    int bttv_write_gpio(unsigned int card, unsigned long mask, unsigned long data)
    {
    struct bttv *btv;
    if (card >= bttv_num) {
    return -EINVAL;
    }
    btv = bttvs[card];
    if (!btv)
    return -ENODEV;
// prior setting BT848_GPIO_REG_INP is (probably) not needed
    because direct input is set on init */
    gpio_bits(mask,data);
    if (bttv_gpio)
    bttv_gpio_tracking(btv,"extern write");
    return 0;
    }
