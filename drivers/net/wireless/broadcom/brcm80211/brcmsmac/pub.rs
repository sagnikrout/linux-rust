//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/pub.h
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


//
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// phy types

// bw

// a large TX Power as an init value to factor out of min() calculations,
// keep low enough to fit in an s8, units are .25 dBm
//

// rate related definitions
pub const BRCMS_RATE_FLAG: c_uint = 0x80	/* Flag to indicate it is a basic rate */;
pub const BRCMS_RATE_MASK: c_uint = 0x7f	/* Rate value mask w/o basic rate flag */;
// legacy rx Antenna diversity for SISO rates

// default antdiv setting

// legacy rx Antenna diversity for SISO rates
// Tx on antenna 0, "legacy term Main"
pub const ANT_TX_FORCE_0: c_int = 0;
// Tx on antenna 1, "legacy term Aux"
pub const ANT_TX_FORCE_1: c_int = 1;
// Tx on phy's last good Rx antenna
pub const ANT_TX_LAST_RX: c_int = 3;
// driver's default tx antenna setting
pub const ANT_TX_DEF: c_int = 3;
// Tx Chain values
// def bitmap of txchain
pub const TXCHAIN_DEF: c_uint = 0x1;
// default bitmap of tx chains for nphy
pub const TXCHAIN_DEF_NPHY: c_uint = 0x3;
// default bitmap of tx chains for nphy
pub const TXCHAIN_DEF_HTPHY: c_uint = 0x7;
// def bitmap of rxchain
pub const RXCHAIN_DEF: c_uint = 0x1;
// default bitmap of rx chains for nphy
pub const RXCHAIN_DEF_NPHY: c_uint = 0x3;
// default bitmap of rx chains for nphy
pub const RXCHAIN_DEF_HTPHY: c_uint = 0x7;
// no antenna switch
pub const ANTSWITCH_NONE: c_int = 0;
// antenna switch on 4321CB2, 2of3
pub const ANTSWITCH_TYPE_1: c_int = 1;
// antenna switch on 4321MPCI, 2of3
pub const ANTSWITCH_TYPE_2: c_int = 2;
// antenna switch on 4322, 2of3
pub const ANTSWITCH_TYPE_3: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_rateset {
// # rates in this set
    pub count: u32,
// rates in 500kbps units w/hi bit set if basic
    pub rates: [u8; WL_NUMRATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_c_rateset {
    pub /: *mut *mut uint count; / number of rates in rates[],
// rates in 500kbps units w/hi bit set if basic
    pub rates: [u8; BRCMS_NUMRATES],
    pub /: *mut *mut u8 htphy_membership; / HT PHY Membership,
    pub /: *mut *mut u8 mcs[MCSSET_LEN]; / supported mcs index bit map,
}

// All the HT-specific default advertised capabilities (including AMPDU)
// should be grouped here at one place
//

// wlc internal bss_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_bss_info {
    pub /: *mut *mut u8 BSSID[ETH_ALEN]; / network BSSID,
    pub /: *mut *mut u16 flags; / flags for internal attributes,
    pub /: *mut *mut u8 SSID_len; / the length of SSID,
    pub /: *mut *mut u8 SSID[32]; / SSID string,
    pub /: *mut *mut s16 RSSI; / receive signal strength (in dBm),
    pub /: *mut *mut s16 SNR; / receive signal SNR in dB,
    pub /: *mut *mut u16 beacon_period; / units are Kusec,
    pub /: *mut *mut u16 chanspec; / Channel num, bw, ctrl_sb and band,
    pub /: *mut *mut brcms_c_rateset rateset; / supported rates,
}

//
// Public portion of common driver state structure.
// The wlc handle points at this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_pub {
    pub wlc: *mut brcms_c_info,
    pub ieee_hw: *mut ieee80211_hw,
    pub global_ampdu: *mut scb_ampdu,
    pub mac80211_state: c_uint,
    pub /: *mut *mut uint unit; / device instance number,
    pub /: *mut *mut uint corerev; / core revision,
    pub /: *mut *mut *mut si_pub sih; / SI handle (cookie for siutils calls),
    pub /: *mut *mut bool up; / interface up and running,
    pub /: *mut *mut bool hw_off; / HW is off,
    pub /: *mut *mut bool hw_up; / one time hw up/down,
    pub /: *mut *mut bool _piomode; / true if pio mode,
    pub /: *mut *mut uint _nbands; / # bands supported,
    pub /: *mut *mut uint now; / # elapsed seconds,
    pub /: *mut *mut bool delayed_down; / down delayed,
    pub /: *mut *mut bool associated; / true:part of [I]BSS, false: not,
// (union of stas_associated, aps_associated)
    pub /: *mut *mut bool _ampdu; / ampdu enabled or not,
    pub /: *mut *mut u8 _n_enab; / bitmap of 11N + HT support,
    pub /: *mut *mut u8 cur_etheraddr[ETH_ALEN]; / our local ethernet address,
    pub /: *mut *mut u32 radio_disabled; / bit vector for radio disabled reasons,
    pub /: *mut *mut u16 boardrev; / version # of particular board,
    pub /: *mut *mut u8 sromrev; / version # of the srom,
    pub /: *mut *mut char srom_ccode[BRCM_CNTRY_BUF_SZ]; / Country Code in SROM,
    pub /: *mut *mut u32 boardflags; / Board specific flags from srom,
    pub /: *mut *mut u32 boardflags2; / More board flags if sromrev >= 4,
    pub /: *mut *mut bool phy_11ncapable; / the PHY/HW is capable of 802.11N,
    pub /: *mut *mut *mut wl_cnt _cnt; / low-level counters in driver,
    pub dbgfs_dir: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlc_par_id {
    IOV_MPC = 1,
    IOV_RTSTHRESH,
    IOV_QTXPOWER,
    IOV_BCN_LI_BCN		/* Beacon listen interval in # of beacons */
}

//
// Feature-related macros to optimize out code
//
pub const ENAB_1x1: c_uint = 0x01;
pub const ENAB_2x2: c_uint = 0x02;
pub const ENAB_3x3: c_uint = 0x04;
pub const ENAB_4x4: c_uint = 0x08;

// WL11N Support
pub const AMPDU_AGG_HOST: c_int = 1;
// network protection config

//
// 54g modes (basic bits may still be overridden)
//
// GMODE_LEGACY_B
// Rateset: 1b, 2b, 5.5, 11
// Preamble: Long
// Shortslot: Off
// GMODE_AUTO
// Rateset: 1b, 2b, 5.5b, 11b, 18, 24, 36, 54
// Extended Rateset: 6, 9, 12, 48
// Preamble: Long
// Shortslot: Auto
// GMODE_ONLY
// Rateset: 1b, 2b, 5.5b, 11b, 18, 24b, 36, 54
// Extended Rateset: 6b, 9, 12b, 48
// Preamble: Short required
// Shortslot: Auto
// GMODE_B_DEFERRED
// Rateset: 1b, 2b, 5.5b, 11b, 18, 24, 36, 54
// Extended Rateset: 6, 9, 12, 48
// Preamble: Long
// Shortslot: On
// GMODE_PERFORMANCE
// Rateset: 1b, 2b, 5.5b, 6b, 9, 11b, 12b, 18, 24b, 36, 48, 54
// Preamble: Short required
// Shortslot: On and required
// GMODE_LRS
// Rateset: 1b, 2b, 5.5b, 11b
// Extended Rateset: 6, 9, 12, 18, 24, 36, 48, 54
// Preamble: Long
// Shortslot: Auto
//
pub const GMODE_LEGACY_B: c_int = 0;
pub const GMODE_AUTO: c_int = 1;
pub const GMODE_ONLY: c_int = 2;
pub const GMODE_B_DEFERRED: c_int = 3;
pub const GMODE_PERFORMANCE: c_int = 4;
pub const GMODE_LRS: c_int = 5;
pub const GMODE_MAX: c_int = 6;
// MCS values greater than this enable multiple streams
pub const HIGHEST_SINGLE_STREAM_MCS: c_int = 7;

// max number of antenna configurations
pub const ANT_SELCFG_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_antselcfg {
    pub /: *mut *mut u8 ant_config[ANT_SELCFG_MAX]; / antenna configuration,
    pub /: *mut *mut u8 num_antcfg; / number of available antenna configurations,
}

// common functions for every port
extern "C" {
    pub fn brcms_c_detach(wlc: *mut brcms_c_info) -> c_uint;
}
extern "C" {
    pub fn brcms_c_up(wlc: *mut brcms_c_info) -> c_int;
}
extern "C" {
    pub fn brcms_c_down(wlc: *mut brcms_c_info) -> c_uint;
}
extern "C" {
    pub fn brcms_c_chipmatch(core: *mut bcma_device) -> bool;
}
extern "C" {
    pub fn brcms_c_init(wlc: *mut brcms_c_info, mute_tx: bool);
}
extern "C" {
    pub fn brcms_c_reset(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_intrson(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_intrsoff(wlc: *mut brcms_c_info) -> u32;
}
extern "C" {
    pub fn brcms_c_intrsrestore(wlc: *mut brcms_c_info, macintmask: u32);
}
extern "C" {
    pub fn brcms_c_intrsupd(wlc: *mut brcms_c_info) -> bool;
}
extern "C" {
    pub fn brcms_c_isr(wlc: *mut brcms_c_info) -> bool;
}
extern "C" {
    pub fn brcms_c_dpc(wlc: *mut brcms_c_info, bounded: bool) -> bool;
}
extern "C" {
    pub fn brcms_c_aggregatable(wlc: *mut brcms_c_info, tid: u8) -> bool;
}
extern "C" {
    pub fn brcms_c_protection_upd(wlc: *mut brcms_c_info, idx: c_uint, val: c_int);
}
extern "C" {
    pub fn brcms_c_get_header_len() -> c_int;
}
extern "C" {
    pub fn brcms_c_suspend_mac_and_wait(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_enable_mac(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_associate_upd(wlc: *mut brcms_c_info, state: bool);
}
extern "C" {
    pub fn brcms_c_scan_start(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_scan_stop(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_get_curband(wlc: *mut brcms_c_info) -> c_int;
}
extern "C" {
    pub fn brcms_c_set_channel(wlc: *mut brcms_c_info, channel: u16) -> c_int;
}
extern "C" {
    pub fn brcms_c_set_rate_limit(wlc: *mut brcms_c_info, srl: u16, lrl: u16) -> c_int;
}
extern "C" {
    pub fn brcms_c_set_rateset(wlc: *mut brcms_c_info, rs: *mut brcm_rateset) -> c_int;
}
extern "C" {
    pub fn brcms_c_set_beacon_period(wlc: *mut brcms_c_info, period: u16) -> c_int;
}
extern "C" {
    pub fn brcms_c_get_phy_type(wlc: *mut brcms_c_info, phyidx: c_int) -> u16;
}
extern "C" {
    pub fn brcms_c_set_beacon_listen_interval(wlc: *mut brcms_c_info, interval: u8);
}
extern "C" {
    pub fn brcms_c_tsf_get(wlc: *mut brcms_c_info) -> u64;
}
extern "C" {
    pub fn brcms_c_tsf_set(wlc: *mut brcms_c_info, tsf: u64);
}
extern "C" {
    pub fn brcms_c_set_tx_power(wlc: *mut brcms_c_info, txpwr: c_int) -> c_int;
}
extern "C" {
    pub fn brcms_c_get_tx_power(wlc: *mut brcms_c_info) -> c_int;
}
extern "C" {
    pub fn brcms_c_check_radio_disabled(wlc: *mut brcms_c_info) -> bool;
}
extern "C" {
    pub fn brcms_c_mute(wlc: *mut brcms_c_info, on: bool);
}
extern "C" {
    pub fn brcms_c_tx_flush_completed(wlc: *mut brcms_c_info) -> bool;
}
extern "C" {
    pub fn brcms_c_start_station(wlc: *mut brcms_c_info, addr: *mut u8);
}
extern "C" {
    pub fn brcms_c_start_adhoc(wlc: *mut brcms_c_info, addr: *mut u8);
}
extern "C" {
    pub fn brcms_c_update_beacon(wlc: *mut brcms_c_info);
}
extern "C" {
    pub fn brcms_c_enable_probe_resp(wlc: *mut brcms_c_info, enable: bool);
}
extern "C" {
    pub fn brcms_c_set_ssid(wlc: *mut brcms_c_info, ssid: *mut u8, ssid_len: usize);
}
