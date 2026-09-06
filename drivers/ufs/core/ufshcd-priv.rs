//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/core/ufshcd-priv.h
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

extern "C" {
    pub fn ufshcd_enable_intr(hba: *mut ufs_hba, intrs: u32);
}
extern "C" {
    pub fn ufshcd_schedule_eh_work(hba: *mut ufs_hba);
}

extern "C" {
    pub fn ufs_hwmon_probe(hba: *mut ufs_hba, mask: u8);
}
extern "C" {
    pub fn ufs_hwmon_remove(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufs_hwmon_notify_event(hba: *mut ufs_hba, ee_mask: u8);
}

extern "C" {
    pub fn ufshcd_auto_hibern8_update(hba: *mut ufs_hba, ahit: u32);
}
extern "C" {
    pub fn ufshcd_mcq_init(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_mcq_disable(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_get_hba_mac(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_mcq_memory_alloc(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufshcd_cmd_inflight(cmd: *mut scsi_cmnd) -> bool;
}
extern "C" {
    pub fn ufshcd_mcq_sq_cleanup(hba: *mut ufs_hba, task_tag: c_int) -> c_int;
}
extern "C" {
    pub fn ufshcd_mcq_abort(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn ufshcd_mcq_read_mcqiacr(hba: *mut ufs_hba, i: c_int) -> u32;
}
extern "C" {
    pub fn ufshcd_mcq_write_mcqiacr(hba: *mut ufs_hba, val: u32, i: c_int);
}
extern "C" {
    pub fn ufshcd_try_to_abort_task(hba: *mut ufs_hba, tag: c_int) -> c_int;
}
extern "C" {
    pub fn ufshcd_release_scsi_cmd(hba: *mut ufs_hba, cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn ufshcd_pause_command_processing(hba: *mut ufs_hba, timeout_us: u64) -> c_int;
}
extern "C" {
    pub fn ufshcd_resume_command_processing(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_scale_clks(hba: *mut ufs_hba, freq: c_ulong, scale_up: bool) -> c_int;
}
//
// enum ufs_descr_fmt - UFS string descriptor format
// @SD_RAW: Raw UTF-16 format
// @SD_ASCII_STD: Convert to null-terminated ASCII string
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_descr_fmt {
    SD_RAW = 0,
    SD_ASCII_STD = 1,
}

extern "C" {
    pub fn ufshcd_read_string_desc(hba: *mut ufs_hba, desc_index: u8, buf: *mut u8, fmt: ufs_descr_fmt) -> c_int;
}
extern "C" {
    pub fn ufshcd_send_uic_cmd(hba: *mut ufs_hba, uic_cmd: *mut uic_command) -> c_int;
}
extern "C" {
    pub fn ufshcd_send_bsg_uic_cmd(hba: *mut ufs_hba, uic_cmd: *mut uic_command) -> c_int;
}
extern "C" {
    pub fn ufshcd_wb_toggle(hba: *mut ufs_hba, enable: bool) -> c_int;
}
extern "C" {
    pub fn ufshcd_uic_tx_eqtr(hba: *mut ufs_hba, gear: c_int) -> c_int;
}
extern "C" {
    pub fn ufshcd_apply_valid_tx_eq_settings(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_print_tx_eq_params(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_is_txeq_presets_used(hba: *mut ufs_hba) -> bool;
}
extern "C" {
    pub fn ufshcd_is_txeq_preset_selected(preshoot: u8, deemphasis: u8) -> bool;
}
extern "C" {
    pub fn ufshcd_retrain_tx_eq(hba: *mut ufs_hba, gear: u32) -> c_int;
}
extern "C" {
    pub fn ufshcd_retrieve_tx_eq_settings(hba: *mut ufs_hba);
}
extern "C" {
    pub fn ufshcd_store_tx_eq_settings(hba: *mut ufs_hba);
}
// Wrapper functions for safely calling variant operations
extern "C" {
    pub fn ufshcd_readl(_arg: hba, _arg: REG_UFS_VERSION) -> return;
}
//
// ufshcd_scsi_to_upiu_lun - maps scsi LUN to UPIU LUN
// @scsi_lun: scsi LUN id
//
// Return: UPIU LUN id
//
extern "C" {
    pub fn __ufshcd_write_ee_control(hba: *mut ufs_hba, ee_ctrl_mask: u32) -> c_int;
}
extern "C" {
    pub fn ufshcd_write_ee_control(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn pm_runtime_get_sync(_arg: &hba->ufs_device_wlun->sdev_gendev) -> return;
}
extern "C" {
    pub fn pm_runtime_get_if_active(_arg: &hba->ufs_device_wlun->sdev_gendev) -> return;
}
extern "C" {
    pub fn pm_runtime_put_sync(_arg: &hba->ufs_device_wlun->sdev_gendev) -> return;
}
extern "C" {
    pub fn pm_runtime_resume(_arg: &hba->ufs_device_wlun->sdev_gendev) -> return;
}
//
// ufs_is_valid_unit_desc_lun - checks if the given LUN has a unit descriptor
// @dev_info: pointer of instance of struct ufs_dev_info
// @lun: LU number to check
// @return: true if the lun has a matching unit descriptor, false otherwise
//
// Convert a block layer tag into a SCSI command pointer. This function is
// called once per I/O completion path and is also called from error paths.
//
extern "C" {
    pub fn blk_mq_rq_to_pdu(_arg: rq) -> return;
}

extern "C" {
    pub fn ufs_rpmb_probe(hba: *mut ufs_hba) -> c_int;
}
extern "C" {
    pub fn ufs_rpmb_remove(hba: *mut ufs_hba);
}

