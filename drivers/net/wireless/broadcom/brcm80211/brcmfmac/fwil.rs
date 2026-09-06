//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/fwil.h
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
// Copyright (c) 2012 Broadcom Corporation
//

// Macro flag: #define _fwil_h_

//
// Dongle command codes that are interpreted by firmware
//
pub const BRCMF_C_GET_VERSION: c_int = 1;
pub const BRCMF_C_UP: c_int = 2;
pub const BRCMF_C_DOWN: c_int = 3;
pub const BRCMF_C_SET_PROMISC: c_int = 10;
pub const BRCMF_C_GET_RATE: c_int = 12;
pub const BRCMF_C_GET_INFRA: c_int = 19;
pub const BRCMF_C_SET_INFRA: c_int = 20;
pub const BRCMF_C_GET_AUTH: c_int = 21;
pub const BRCMF_C_SET_AUTH: c_int = 22;
pub const BRCMF_C_GET_BSSID: c_int = 23;
pub const BRCMF_C_GET_SSID: c_int = 25;
pub const BRCMF_C_SET_SSID: c_int = 26;
pub const BRCMF_C_TERMINATED: c_int = 28;
pub const BRCMF_C_GET_CHANNEL: c_int = 29;
pub const BRCMF_C_SET_CHANNEL: c_int = 30;
pub const BRCMF_C_GET_SRL: c_int = 31;
pub const BRCMF_C_SET_SRL: c_int = 32;
pub const BRCMF_C_GET_LRL: c_int = 33;
pub const BRCMF_C_SET_LRL: c_int = 34;
pub const BRCMF_C_GET_RADIO: c_int = 37;
pub const BRCMF_C_SET_RADIO: c_int = 38;
pub const BRCMF_C_GET_PHYTYPE: c_int = 39;
pub const BRCMF_C_SET_KEY: c_int = 45;
pub const BRCMF_C_GET_REGULATORY: c_int = 46;
pub const BRCMF_C_SET_REGULATORY: c_int = 47;
pub const BRCMF_C_SET_PASSIVE_SCAN: c_int = 49;
pub const BRCMF_C_SCAN: c_int = 50;
pub const BRCMF_C_SCAN_RESULTS: c_int = 51;
pub const BRCMF_C_DISASSOC: c_int = 52;
pub const BRCMF_C_REASSOC: c_int = 53;
pub const BRCMF_C_SET_ROAM_TRIGGER: c_int = 55;
pub const BRCMF_C_SET_ROAM_DELTA: c_int = 57;
pub const BRCMF_C_GET_BCNPRD: c_int = 75;
pub const BRCMF_C_SET_BCNPRD: c_int = 76;
pub const BRCMF_C_GET_DTIMPRD: c_int = 77;
pub const BRCMF_C_SET_DTIMPRD: c_int = 78;
pub const BRCMF_C_SET_COUNTRY: c_int = 84;
pub const BRCMF_C_GET_PM: c_int = 85;
pub const BRCMF_C_SET_PM: c_int = 86;
pub const BRCMF_C_GET_REVINFO: c_int = 98;
pub const BRCMF_C_GET_MONITOR: c_int = 107;
pub const BRCMF_C_SET_MONITOR: c_int = 108;
pub const BRCMF_C_GET_CURR_RATESET: c_int = 114;
pub const BRCMF_C_GET_AP: c_int = 117;
pub const BRCMF_C_SET_AP: c_int = 118;
pub const BRCMF_C_SET_SCB_AUTHORIZE: c_int = 121;
pub const BRCMF_C_SET_SCB_DEAUTHORIZE: c_int = 122;
pub const BRCMF_C_GET_RSSI: c_int = 127;
pub const BRCMF_C_GET_WSEC: c_int = 133;
pub const BRCMF_C_SET_WSEC: c_int = 134;
pub const BRCMF_C_GET_PHY_NOISE: c_int = 135;
pub const BRCMF_C_GET_BSS_INFO: c_int = 136;
pub const BRCMF_C_GET_GET_PKTCNTS: c_int = 137;
pub const BRCMF_C_GET_BANDLIST: c_int = 140;
pub const BRCMF_C_SET_SCB_TIMEOUT: c_int = 158;
pub const BRCMF_C_GET_ASSOCLIST: c_int = 159;
pub const BRCMF_C_GET_PHYLIST: c_int = 180;
pub const BRCMF_C_SET_SCAN_CHANNEL_TIME: c_int = 185;
pub const BRCMF_C_SET_SCAN_UNASSOC_TIME: c_int = 187;
pub const BRCMF_C_SCB_DEAUTHENTICATE_FOR_REASON: c_int = 201;
pub const BRCMF_C_SET_ASSOC_PREFER: c_int = 205;
pub const BRCMF_C_GET_VALID_CHANNELS: c_int = 217;
pub const BRCMF_C_SET_FAKEFRAG: c_int = 219;
pub const BRCMF_C_GET_KEY_PRIMARY: c_int = 235;
pub const BRCMF_C_SET_KEY_PRIMARY: c_int = 236;
pub const BRCMF_C_SET_SCAN_PASSIVE_TIME: c_int = 258;
pub const BRCMF_C_GET_VAR: c_int = 262;
pub const BRCMF_C_SET_VAR: c_int = 263;
pub const BRCMF_C_SET_WSEC_PMK: c_int = 268;
extern "C" {
    pub fn brcmf_fil_cmd_data_set(ifp: *mut brcmf_if, cmd: u32, data: *mut c_void, len: u32) -> i32;
}
extern "C" {
    pub fn brcmf_fil_cmd_data_get(ifp: *mut brcmf_if, cmd: u32, data: *mut c_void, len: u32) -> i32;
}
// data = le32_to_cpu(*(__le32 *)data);
// data_le = cpu_to_le32(*data);
extern "C" {
    pub fn brcmf_fil_cmd_int_get(_arg: ifp, _arg: cmd, _arg: data) -> return;
}
extern "C" {
    pub fn brcmf_fil_iovar_data_set(_arg: ifp, _arg: name, _arg: &data_le, _arg: sizeof(data_le)) -> return;
}
// data = le32_to_cpu(*(__le32 *)data);
// data_le = cpu_to_le32(*data);
extern "C" {
    pub fn brcmf_fil_iovar_int_get(_arg: ifp, _arg: name, _arg: data) -> return;
}
// data = le32_to_cpu(*(__le32 *)data);
// data_le = cpu_to_le32(*data);
extern "C" {
    pub fn brcmf_fil_bsscfg_int_get(_arg: ifp, _arg: name, _arg: data) -> return;
}
// data = le32_to_cpu(data_le);
extern "C" {
    pub fn brcmf_fil_xtlv_data_get(_arg: ifp, _arg: name, _arg: id, _arg: data, _arg: *mut sizeof(data)) -> return;
}
// data = le16_to_cpu(data_le);
