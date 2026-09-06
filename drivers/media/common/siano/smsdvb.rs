//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/common/siano/smsdvb.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smsdvb_client_t {
    pub entry: list_head,
    pub coredev: *mut smscore_device_t,
    pub smsclient: *mut smscore_client_t,
    pub adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub frontend: dvb_frontend,
    pub fe_status: fe_status,
    pub tune_done: completion,
    pub stats_done: completion,
    pub last_per: c_int,
    pub legacy_per: int legacy_ber,,
    pub event_fe_state: c_int,
    pub event_unc_state: c_int,
    pub get_stats_jiffies: c_ulong,
    pub feed_users: c_int,
    pub has_tuned: bool,
// stats debugfs data
    pub debugfs: *mut dentry,
    pub debug_data: *mut smsdvb_debugfs,
    pub prt_dvb_stats: sms_prt_dvb_stats_t,
    pub prt_isdb_stats: sms_prt_isdb_stats_t,
    pub prt_isdb_stats_ex: sms_prt_isdb_stats_ex_t,
}

//
// This struct is a mix of struct sms_rx_stats_ex and
// struct sms_srvm_signal_status.
// It was obtained by comparing the way it was filled by the original code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RECEPTION_STATISTICS_PER_SLICES_S {
    pub result: u32,
    pub snr: u32,
    pub in_band_power: i32,
    pub ts_packets: u32,
    pub ets_packets: u32,
    pub constellation: u32,
    pub hp_code: u32,
    pub tps_srv_ind_lp: u32,
    pub tps_srv_ind_hp: u32,
    pub cell_id: u32,
    pub reason: u32,
    pub request_id: u32,
    pub /: *mut *mut u32 modem_state; / from SMSHOSTLIB_DVB_MODEM_STATE_ET,
    pub /: *mut *mut u32 ber; / Post Viterbi BER [1E-5],
    pub /: *mut *mut s32 RSSI; / dBm,
    pub /: *mut *mut s32 carrier_offset; / Carrier Offset in bin/1024,
    pub /: *mut *mut u32 is_rf_locked; / 0 - not locked, 1 - locked,
    pub /: *mut *mut u32 is_demod_locked; / 0 - not locked, 1 - locked,
    pub /: *mut *mut u32 ber_bit_count; / Total number of SYNC bits.,
    pub /: *mut *mut u32 ber_error_count; / Number of erroneous SYNC bits.,
    pub /: *mut *mut s32 MRC_SNR; / dB,
    pub /: *mut *mut s32 mrc_in_band_pwr; / In band power in dBM,
    pub /: *mut *mut s32 MRC_RSSI; / dBm,
}

// From smsdvb-debugfs.c

extern "C" {
    pub fn smsdvb_debugfs_create(client: *mut smsdvb_client_t) -> c_int;
}
extern "C" {
    pub fn smsdvb_debugfs_release(client: *mut smsdvb_client_t);
}
extern "C" {
    pub fn smsdvb_debugfs_register();
}
extern "C" {
    pub fn smsdvb_debugfs_unregister();
}

