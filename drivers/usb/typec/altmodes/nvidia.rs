//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/altmodes/nvidia.c
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
//
// Copyright (C) 2019 NVIDIA Corporation. All rights reserved.
//
// NVIDIA USB Type-C Alt Mode Driver
//

#[no_mangle]
unsafe extern "C" fn nvidia_altmode_probe(alt: *mut typec_altmode) -> c_int {
    static int nvidia_altmode_probe(struct typec_altmode *alt)
    {
    if (alt.svid == USB_TYPEC_NVIDIA_VLINK_SID)
    return dp_altmode_probe(alt);
    else
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn nvidia_altmode_remove(alt: *mut typec_altmode) {
    static void nvidia_altmode_remove(struct typec_altmode *alt)
    {
    if (alt.svid == USB_TYPEC_NVIDIA_VLINK_SID)
    dp_altmode_remove(alt);
    }
    static const struct typec_device_id nvidia_typec_id[] = {
    { USB_TYPEC_NVIDIA_VLINK_SID },
    { },
    };
    MODULE_DEVICE_TABLE(typec, nvidia_typec_id);
    static struct typec_altmode_driver nvidia_altmode_driver = {
    .id_table = nvidia_typec_id,
    .probe = nvidia_altmode_probe,
    .remove = nvidia_altmode_remove,
    .driver = {
    .name = "typec_nvidia",
    },
    };
    module_typec_altmode_driver(nvidia_altmode_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("NVIDIA USB Type-C Alt Mode Driver");
