//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/firewire/firedtv.h
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
// FireDTV driver (formerly known as FireSAT)
//
// Copyright (C) 2004 Andreas Monitzer <andy@monitzer.com>
// Copyright (C) 2008 Henrik Kurelid <henrik@kurelid.se>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct firedtv_tuner_status {
    pub active_system:8: unsigned,
    pub searching:1: unsigned,
    pub moving:1: unsigned,
    pub no_rf:1: unsigned,
    pub input:1: unsigned,
    pub selected_antenna:7: unsigned,
    pub ber:32: unsigned,
    pub signal_strength:8: unsigned,
    pub raster_frequency:2: unsigned,
    pub rf_frequency:22: unsigned,
    pub man_dep_info_length:8: unsigned,
    pub front_end_error:1: unsigned,
    pub antenna_error:1: unsigned,
    pub front_end_power_status:1: unsigned,
    pub power_supply:1: unsigned,
    pub carrier_noise_ratio:16: unsigned,
    pub power_supply_voltage:8: unsigned,
    pub antenna_voltage:8: unsigned,
    pub firewire_bus_voltage:8: unsigned,
    pub ca_mmi:1: unsigned,
    pub ca_pmt_reply:1: unsigned,
    pub ca_date_time_request:1: unsigned,
    pub ca_application_info:1: unsigned,
    pub ca_module_present_status:1: unsigned,
    pub ca_dvb_flag:1: unsigned,
    pub ca_error_flag:1: unsigned,
    pub ca_initialization_status:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum model_type {
    FIREDTV_UNKNOWN = 0,
    FIREDTV_DVB_S   = 1,
    FIREDTV_DVB_C   = 2,
    FIREDTV_DVB_T   = 3,
    FIREDTV_DVB_S2  = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct firedtv {
    pub device: *mut device,
    pub list: list_head,
    pub adapter: dvb_adapter,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub frontend: dmx_frontend,
    pub dvbnet: dvb_net,
    pub fe: dvb_frontend,
    pub cadev: *mut dvb_device,
    pub ca_last_command: c_int,
    pub ca_time_interval: c_int,
    pub avc_mutex: mutex,
    pub avc_wait: wait_queue_head_t,
    pub avc_reply_received: bool,
    pub remote_ctrl_work: work_struct,
    pub remote_ctrl_dev: *mut input_dev,
    pub type: model_type,
    pub subunit: c_char,
    pub isochannel: i8,
    pub ir_context: *mut fdtv_ir_context,
    pub voltage: fe_sec_voltage,
    pub tone: fe_sec_tone_mode,
    pub demux_mutex: mutex,
    pub channel_active: c_ulong,
    pub channel_pid: [u16; 16],
    pub avc_data_length: c_int,
    pub avc_data: [u8; 512],
}

// firedtv-avc.c
extern "C" {
    pub fn avc_recv(fdtv: *mut firedtv, data: *mut c_void, length: usize) -> c_int;
}
extern "C" {
    pub fn avc_tuner_status(fdtv: *mut firedtv, stat: *mut firedtv_tuner_status) -> c_int;
}
extern "C" {
    pub fn avc_tuner_dsd(fdtv: *mut firedtv, params: *mut dtv_frontend_properties) -> c_int;
}
extern "C" {
    pub fn avc_tuner_set_pids(fdtv: *mut firedtv, pidc: c_uchar, pid[]: u16) -> c_int;
}
extern "C" {
    pub fn avc_tuner_get_ts(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn avc_identify_subunit(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn avc_remote_ctrl_work(work: *mut work_struct);
}
extern "C" {
    pub fn avc_register_remote_control(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn avc_ca_reset(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn avc_ca_pmt(fdtv: *mut firedtv, app_info: *mut c_char, length: c_int) -> c_int;
}
extern "C" {
    pub fn avc_ca_get_time_date(fdtv: *mut firedtv, interval: *mut c_int) -> c_int;
}
extern "C" {
    pub fn avc_ca_enter_menu(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn avc_ca_get_mmi(fdtv: *mut firedtv, mmi_object: *mut c_char, len: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn cmp_establish_pp_connection(fdtv: *mut firedtv, plug: c_int, channel: c_int) -> c_int;
}
extern "C" {
    pub fn cmp_break_pp_connection(fdtv: *mut firedtv, plug: c_int, channel: c_int);
}
// firedtv-ci.c
extern "C" {
    pub fn fdtv_ca_register(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn fdtv_ca_release(fdtv: *mut firedtv);
}
// firedtv-dvb.c
extern "C" {
    pub fn fdtv_start_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int;
}
extern "C" {
    pub fn fdtv_stop_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int;
}
extern "C" {
    pub fn fdtv_dvb_register(fdtv: *mut firedtv, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn fdtv_dvb_unregister(fdtv: *mut firedtv);
}
// firedtv-fe.c
extern "C" {
    pub fn fdtv_frontend_init(fdtv: *mut firedtv, name: *const c_char);
}
// firedtv-fw.c
extern "C" {
    pub fn fdtv_lock(fdtv: *mut firedtv, addr: u64, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn fdtv_read(fdtv: *mut firedtv, addr: u64, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn fdtv_write(fdtv: *mut firedtv, addr: u64, data: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn fdtv_start_iso(fdtv: *mut firedtv) -> c_int;
}
extern "C" {
    pub fn fdtv_stop_iso(fdtv: *mut firedtv);
}
// firedtv-rc.c

extern "C" {
    pub fn fdtv_register_rc(fdtv: *mut firedtv, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn fdtv_unregister_rc(fdtv: *mut firedtv);
}
extern "C" {
    pub fn fdtv_handle_rc(fdtv: *mut firedtv, code: c_uint);
}

