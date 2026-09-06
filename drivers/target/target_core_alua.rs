//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_alua.h
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
// INQUIRY response data, TPGS Field
//
// from spc4r17 section 6.4.2 Table 135
//
pub const TPGS_NO_ALUA: c_uint = 0x00;
pub const TPGS_IMPLICIT_ALUA: c_uint = 0x10;
pub const TPGS_EXPLICIT_ALUA: c_uint = 0x20;
//
// ASYMMETRIC ACCESS STATE field
//
// from spc4r36j section 6.37 Table 307
//
pub const ALUA_ACCESS_STATE_ACTIVE_OPTIMIZED: c_uint = 0x0;
pub const ALUA_ACCESS_STATE_ACTIVE_NON_OPTIMIZED: c_uint = 0x1;
pub const ALUA_ACCESS_STATE_STANDBY: c_uint = 0x2;
pub const ALUA_ACCESS_STATE_UNAVAILABLE: c_uint = 0x3;
pub const ALUA_ACCESS_STATE_LBA_DEPENDENT: c_uint = 0x4;
pub const ALUA_ACCESS_STATE_OFFLINE: c_uint = 0xe;
pub const ALUA_ACCESS_STATE_TRANSITION: c_uint = 0xf;
//
// from spc4r36j section 6.37 Table 306
//
pub const ALUA_T_SUP: c_uint = 0x80;
pub const ALUA_O_SUP: c_uint = 0x40;
pub const ALUA_LBD_SUP: c_uint = 0x10;
pub const ALUA_U_SUP: c_uint = 0x08;
pub const ALUA_S_SUP: c_uint = 0x04;
pub const ALUA_AN_SUP: c_uint = 0x02;
pub const ALUA_AO_SUP: c_uint = 0x01;
//
// REPORT_TARGET_PORT_GROUP STATUS CODE
//
// from spc4r17 section 6.27 Table 246
//
pub const ALUA_STATUS_NONE: c_uint = 0x00;
pub const ALUA_STATUS_ALTERED_BY_EXPLICIT_STPG: c_uint = 0x01;
pub const ALUA_STATUS_ALTERED_BY_IMPLICIT_ALUA: c_uint = 0x02;
//
// From spc4r17, Table D.1: ASC and ASCQ Assignement
//
pub const ASCQ_04H_ALUA_STATE_TRANSITION: c_uint = 0x0a;
pub const ASCQ_04H_ALUA_TG_PT_STANDBY: c_uint = 0x0b;
pub const ASCQ_04H_ALUA_TG_PT_UNAVAILABLE: c_uint = 0x0c;
pub const ASCQ_04H_ALUA_OFFLINE: c_uint = 0x12;
//
// Used as the default for Active/NonOptimized delay (in milliseconds)
// This can also be changed via configfs on a per target port group basis..
//
pub const ALUA_DEFAULT_NONOP_DELAY_MSECS: c_int = 100;

//
// Used for implicit and explicit ALUA transitional delay, that is disabled
// by default, and is intended to be used for debugging client side ALUA code.
//
pub const ALUA_DEFAULT_TRANS_DELAY_MSECS: c_int = 0;

//
// Used for the recommended application client implicit transition timeout
// in seconds, returned by the REPORT_TARGET_PORT_GROUPS w/ extended header.
//
pub const ALUA_DEFAULT_IMPLICIT_TRANS_SECS: c_int = 0;
pub const ALUA_MAX_IMPLICIT_TRANS_SECS: c_int = 255;
// Used by core_alua_update_tpg_(primary,secondary)_metadata
pub const ALUA_MD_BUF_LEN: c_int = 1024;
extern "C" {
    pub fn target_emulate_report_target_port_groups(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_emulate_set_target_port_groups(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_emulate_report_referrals(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn core_alua_check_nonop_delay(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn core_alua_allocate_lba_map_mem(: *mut t10_alua_lba_map, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn core_alua_free_lba_map(: *mut list_head);
}
extern "C" {
    pub fn core_alua_set_lu_gp_id(: *mut t10_alua_lu_gp, _arg: u16) -> c_int;
}
extern "C" {
    pub fn core_alua_free_lu_gp(: *mut t10_alua_lu_gp);
}
extern "C" {
    pub fn core_alua_free_lu_gp_mem(: *mut se_device);
}
extern "C" {
    pub fn core_alua_put_lu_gp_from_name(: *mut t10_alua_lu_gp);
}
extern "C" {
    pub fn core_alua_drop_lu_gp_dev(: *mut se_device);
}
extern "C" {
    pub fn core_alua_set_tg_pt_gp_id(: *mut t10_alua_tg_pt_gp, _arg: u16) -> c_int;
}
extern "C" {
    pub fn core_alua_free_tg_pt_gp(: *mut t10_alua_tg_pt_gp);
}
extern "C" {
    pub fn target_detach_tg_pt_gp(: *mut se_lun);
}
extern "C" {
    pub fn target_attach_tg_pt_gp(: *mut se_lun, : *mut t10_alua_tg_pt_gp);
}
extern "C" {
    pub fn core_alua_show_tg_pt_gp_info(: *mut se_lun, : *mut c_char) -> isize;
}
extern "C" {
    pub fn core_alua_show_access_type(: *mut t10_alua_tg_pt_gp, : *mut c_char) -> isize;
}
extern "C" {
    pub fn core_alua_show_offline_bit(: *mut se_lun, : *mut c_char) -> isize;
}
extern "C" {
    pub fn core_alua_show_secondary_status(: *mut se_lun, : *mut c_char) -> isize;
}
extern "C" {
    pub fn core_setup_alua(: *mut se_device) -> c_int;
}
extern "C" {
    pub fn target_alua_state_check(cmd: *mut se_cmd) -> sense_reason_t;
}
