//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/types.h
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
// This header file contains definition for global types
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ie_header {
    pub id: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ie_cf_param_set {
    pub header: ieee_ie_header,
    pub cfpcnt: u8,
    pub cfpperiod: u8,
    pub cfpmaxduration: __le16,
    pub cfpdurationremaining: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ie_ibss_param_set {
    pub header: ieee_ie_header,
    pub atimwindow: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ieee_ss_param_set {
    pub cf: ieee_ie_cf_param_set,
    pub ibss: ieee_ie_ibss_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ie_fh_param_set {
    pub header: ieee_ie_header,
    pub dwelltime: __le16,
    pub hopset: u8,
    pub hoppattern: u8,
    pub hopindex: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_ie_ds_param_set {
    pub header: ieee_ie_header,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ieee_phy_param_set {
    pub fh: ieee_ie_fh_param_set,
    pub ds: ieee_ie_ds_param_set,
    pub __packed: },
// TLV  type ID definition
pub const PROPRIETARY_TLV_BASE_ID: c_uint = 0x0100;
// Terminating TLV type
pub const MRVL_TERMINATE_TLV_ID: c_uint = 0xffff;
pub const TLV_TYPE_SSID: c_uint = 0x0000;
pub const TLV_TYPE_RATES: c_uint = 0x0001;
pub const TLV_TYPE_PHY_FH: c_uint = 0x0002;
pub const TLV_TYPE_PHY_DS: c_uint = 0x0003;
pub const TLV_TYPE_CF: c_uint = 0x0004;
pub const TLV_TYPE_IBSS: c_uint = 0x0006;
pub const TLV_TYPE_DOMAIN: c_uint = 0x0007;
pub const TLV_TYPE_POWER_CAPABILITY: c_uint = 0x0021;

// TLV related data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_header {
    pub type: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_data {
    pub header: mrvl_ie_header,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_rates_param_set {
    pub header: mrvl_ie_header,
    pub rates: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_ssid_param_set {
    pub header: mrvl_ie_header,
    pub ssid: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_wildcard_ssid_param_set {
    pub header: mrvl_ie_header,
    pub maxssidlength: u8,
    pub ssid: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chanscanmode {

    pub reserved_2_7:6: u8,
    pub disablechanfilt:1: u8,
    pub passivescan:1: u8,

    pub passivescan:1: u8,
    pub disablechanfilt:1: u8,
    pub reserved_2_7:6: u8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chanscanparamset {
    pub radiotype: u8,
    pub channumber: u8,
    pub chanscanmode: chanscanmode,
    pub minscantime: __le16,
    pub maxscantime: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_chanlist_param_set {
    pub header: mrvl_ie_header,
    pub chanscanparam: [chanscanparamset; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_cf_param_set {
    pub header: mrvl_ie_header,
    pub cfpcnt: u8,
    pub cfpperiod: u8,
    pub cfpmaxduration: __le16,
    pub cfpdurationremaining: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_ds_param_set {
    pub header: mrvl_ie_header,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_rsn_param_set {
    pub header: mrvl_ie_header,
    pub rsnie: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_tsf_timestamp {
    pub header: mrvl_ie_header,
    pub tsftable: [__le64; ],
    pub __packed: },
// v9 and later firmware only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_auth_type {
    pub header: mrvl_ie_header,
    pub auth: __le16,
    pub __packed: },
// Local Power capability
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_power_capability {
    pub header: mrvl_ie_header,
    pub minpower: i8,
    pub maxpower: i8,
    pub __packed: },
// used in CMD_802_11_SUBSCRIBE_EVENT for SNR, RSSI and Failure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_thresholds {
    pub header: mrvl_ie_header,
    pub value: u8,
    pub freq: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_beacons_missed {
    pub header: mrvl_ie_header,
    pub beaconmissed: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_num_probes {
    pub header: mrvl_ie_header,
    pub numprobes: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_bcast_probe {
    pub header: mrvl_ie_header,
    pub bcastprobe: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_num_ssid_probe {
    pub header: mrvl_ie_header,
    pub numssidprobe: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_pin {
    pub led: u8,
    pub pin: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_ledgpio {
    pub header: mrvl_ie_header,
    pub ledpin: [led_pin; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_bhv {
    pub firmwarestate: u8,
    pub led: u8,
    pub ledstate: u8,
    pub ledarg: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_ledbhv {
    pub header: mrvl_ie_header,
    pub ledbhv: [led_bhv; ],
    pub __packed: },
//
// Meant to be packed as the value member of a struct ieee80211_info_element.
// Note that the len member of the ieee80211_info_element varies depending on
// the mesh_id_len
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_meshie_val {
    pub oui: [u8; 3],
    pub type: u8,
    pub subtype: u8,
    pub version: u8,
    pub active_protocol_id: u8,
    pub active_metric_id: u8,
    pub mesh_capability: u8,
    pub mesh_id_len: u8,
    pub mesh_id: [u8; IEEE80211_MAX_SSID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_meshie {
    pub len: u8 id,,
    pub val: mrvl_meshie_val,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_mesh_defaults {
    pub bootflag: __le32,
    pub boottime: u8,
    pub reserved: u8,
    pub channel: __le16,
    pub meshie: mrvl_meshie,
    pub __packed: },
