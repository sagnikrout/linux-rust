//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/wcc/core.c
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

pub const BRCMF_WCC_E_LAST: c_int = 213;
    static int brcmf_wcc_set_sae_pwd(struct brcmf_if *ifp,
    struct cfg80211_crypto_settings *crypto)
    {
    return brcmf_set_wsec(ifp, crypto.sae_pwd, crypto.sae_pwd_len,
    BRCMF_WSEC_PASSPHRASE);
    }
#[no_mangle]
unsafe extern "C" fn brcmf_wcc_alloc_fweh_info(drvr: *mut brcmf_pub) -> c_int {
    static int brcmf_wcc_alloc_fweh_info(struct brcmf_pub *drvr)
    {
    struct brcmf_fweh_info *fweh;
    fweh = kzalloc_flex(*fweh, evt_handler, BRCMF_WCC_E_LAST);
    if (!fweh)
    return -ENOMEM;
    fweh.num_event_codes = BRCMF_WCC_E_LAST;
    drvr.fweh = fweh;
    return 0;
    }
    const struct brcmf_fwvid_ops brcmf_wcc_ops = {
    .set_sae_password = brcmf_wcc_set_sae_pwd,
    .alloc_fweh_info = brcmf_wcc_alloc_fweh_info,
    };
