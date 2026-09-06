//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/cyw/module.c
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2022 Broadcom Corporation
//

#[no_mangle]
unsafe extern "C" fn brcmf_cyw_init() -> int __init {
    static int __init brcmf_cyw_init(void)
    {
    return brcmf_fwvid_register_vendor(BRCMF_FWVENDOR_CYW, THIS_MODULE,
    &brcmf_cyw_ops);
    }
#[no_mangle]
unsafe extern "C" fn brcmf_cyw_exit() -> void __exit {
    static void __exit brcmf_cyw_exit(void)
    {
    brcmf_fwvid_unregister_vendor(BRCMF_FWVENDOR_CYW, THIS_MODULE);
    }
    MODULE_DESCRIPTION("Broadcom FullMAC WLAN driver plugin for Cypress/Infineon chipsets");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_IMPORT_NS("BRCMFMAC");
    module_init(brcmf_cyw_init);
    module_exit(brcmf_cyw_exit);
