//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/common.h
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
// Copyright (c) 2014 Broadcom Corporation
//

pub const BRCMF_FW_ALTPATH_LEN: c_int = 256;
// Definitions for the module global and device specific settings are defined
// here. Two structs are used for them. brcmf_mp_global_t and brcmf_mp_device.
// The mp_global is instantiated once in a global struct and gets initialized
// by the common_attach function which should be called before any other
// (module) initiliazation takes place. The device specific settings is part
// of the drvr struct and should be initialized on every brcmf_attach.
//
// struct brcmf_mp_global_t - Global module parameters.
//
// @firmware_path: Alternative firmware path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_mp_global_t {
    pub firmware_path: [c_char; BRCMF_FW_ALTPATH_LEN],
}

//
// struct brcmf_mp_device - Device module parameters.
//
// @p2p_enable: Legacy P2P0 enable (old wpa_supplicant).
// @feature_disable: Feature_disable bitmask.
// @fcmode: FWS flow control.
// @roamoff: Firmware roaming off?
// @ignore_probe_fail: Ignore probe failure.
// @trivial_ccode_map: Assume firmware uses ISO3166 country codes with rev 0
// @country_codes: If available, pointer to struct for translating country codes
// @bus: Bus specific platform data. Only SDIO at the mmoment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_mp_device {
    pub p2p_enable: bool,
    pub feature_disable: c_uint,
    pub fcmode: c_int,
    pub roamoff: bool,
    pub iapp: bool,
    pub ignore_probe_fail: bool,
    pub trivial_ccode_map: bool,
    pub country_codes: *mut brcmfmac_pd_cc,
    pub board_type: *const c_char,
    pub mac: [c_uchar; ETH_ALEN],
    pub antenna_sku: *const c_char,
    pub cal_blob: *const c_void,
    pub cal_size: c_int,
    pub sdio: brcmfmac_sdio_pd,
    pub bus: },
}

extern "C" {
    pub fn brcmf_c_set_joinpref_default(ifp: *mut brcmf_if);
}
extern "C" {
    pub fn brcmf_release_module_param(module_param: *mut brcmf_mp_device);
}
// Sets dongle media info (drv_version, mac address).
extern "C" {
    pub fn brcmf_c_preinit_dcmds(ifp: *mut brcmf_if) -> c_int;
}
extern "C" {
    pub fn brcmf_c_set_cur_etheraddr(ifp: *mut brcmf_if, addr: *const u8) -> c_int;
}

extern "C" {
    pub fn brcmf_dmi_probe(settings: *mut brcmf_mp_device, chip: u32, chiprev: u32);
}

extern "C" {
    pub fn brcmf_map_prio_to_prec(cfg: *mut c_void, prio: u8) -> u8;
}
extern "C" {
    pub fn brcmf_map_prio_to_aci(cfg: *mut c_void, prio: u8) -> u8;
}
