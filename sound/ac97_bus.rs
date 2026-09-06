//! Automatically rewritten from C to Rust
//! Source: sound/ac97_bus.c
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
// Linux driver model AC97 bus interface
//
// Author:	Nicolas Pitre
// Created:	Jan 14, 2005
// Copyright:	(C) MontaVista Software Inc.
//

//
// snd_ac97_check_id() - Reads and checks the vendor ID of the device
// @ac97: The AC97 device to check
// @id: The ID to compare to
// @id_mask: Mask that is applied to the device ID before comparing to @id
//
// If @id is 0 this function returns true if the read device vendor ID is
// a valid ID. If @id is non 0 this functions returns true if @id
// matches the read vendor ID. Otherwise the function returns false.
//
    static bool snd_ac97_check_id(struct snd_ac97 *ac97, unsigned int id,
    unsigned int id_mask)
    {
    ac97.id = ac97.bus.ops.read(ac97, AC97_VENDOR_ID1) << 16;
    ac97.id |= ac97.bus.ops.read(ac97, AC97_VENDOR_ID2);
    if (ac97.id == 0x0 || ac97.id == 0xffffffff)
    return false;
    if (id != 0 && id != (ac97.id & id_mask))
    return false;
    return true;
    }
//
// snd_ac97_reset() - Reset AC'97 device
// @ac97: The AC'97 device to reset
// @try_warm: Try a warm reset first
// @id: Expected device vendor ID
// @id_mask: Mask that is applied to the device ID before comparing to @id
//
// This function resets the AC'97 device. If @try_warm is true the function
// first performs a warm reset. If @try_warm is false the function issues
// cold reset followed by a warm reset. If @id is 0 any valid device ID
// will be accepted, otherwise only the ID that matches @id and @id_mask
// is accepted.
// Returns:
// * %1 - if warm reset is successful
// * %0 - if cold reset and warm reset is successful
// * %-ENODEV - if @id and @id_mask not matching
//
    int snd_ac97_reset(struct snd_ac97 *ac97, bool try_warm, unsigned int id,
    unsigned int id_mask)
    {
    const struct snd_ac97_bus_ops *ops = ac97.bus.ops;
    if (try_warm && ops.warm_reset) {
    ops.warm_reset(ac97);
    if (snd_ac97_check_id(ac97, id, id_mask))
    return 1;
    }
    if (ops.reset)
    ops.reset(ac97);
    if (ops.warm_reset)
    ops.warm_reset(ac97);
    if (snd_ac97_check_id(ac97, id, id_mask))
    return 0;
    return -ENODEV;
    }
    EXPORT_SYMBOL_GPL(snd_ac97_reset);
    const struct bus_type ac97_bus_type = {
    .name		= "ac97",
    };
    EXPORT_SYMBOL(ac97_bus_type);
#[no_mangle]
unsafe extern "C" fn ac97_bus_init() -> int __init {
    static int __init ac97_bus_init(void)
    {
    return bus_register(&ac97_bus_type);
    }
    subsys_initcall(ac97_bus_init);
#[no_mangle]
unsafe extern "C" fn ac97_bus_exit() -> void __exit {
    static void __exit ac97_bus_exit(void)
    {
    bus_unregister(&ac97_bus_type);
    }
    module_exit(ac97_bus_exit);
    MODULE_DESCRIPTION("Legacy AC97 bus interface");
    MODULE_LICENSE("GPL");
