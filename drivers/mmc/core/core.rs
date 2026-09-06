//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/core.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/drivers/mmc/core/core.h
//
// Copyright (C) 2003 Russell King, All Rights Reserved.
// Copyright 2007 Pierre Ossman
//

pub const MMC_CMD_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_bus_ops {
    pub ): *mut *mut void (remove)(struct mmc_host,
    pub ): *mut *mut void (detect)(struct mmc_host,
    pub ): *mut *mut int (pre_suspend)(struct mmc_host,
    pub ): *mut *mut int (suspend)(struct mmc_host,
    pub ): *mut *mut int (resume)(struct mmc_host,
    pub ): *mut *mut int (runtime_suspend)(struct mmc_host,
    pub ): *mut *mut int (runtime_resume)(struct mmc_host,
    pub ): *mut *mut int (alive)(struct mmc_host,
    pub ): *mut *mut int (shutdown)(struct mmc_host,
    pub ): *mut *mut int (hw_reset)(struct mmc_host,
    pub ): *mut *mut int (sw_reset)(struct mmc_host,
    pub ): *mut *mut bool (cache_enabled)(struct mmc_host,
    pub ): *mut *mut int (flush_cache)(struct mmc_host,
    pub host): *mut *mut int (handle_undervoltage)(struct mmc_host,
}

extern "C" {
    pub fn mmc_attach_bus(host: *mut mmc_host, ops: *const mmc_bus_ops);
}
extern "C" {
    pub fn mmc_detach_bus(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_init_erase(card: *mut mmc_card);
}
extern "C" {
    pub fn mmc_set_chip_select(host: *mut mmc_host, mode: c_int);
}
extern "C" {
    pub fn mmc_set_clock(host: *mut mmc_host, hz: c_uint);
}
extern "C" {
    pub fn mmc_set_bus_mode(host: *mut mmc_host, mode: c_uint);
}
extern "C" {
    pub fn mmc_set_bus_width(host: *mut mmc_host, width: c_uint);
}
extern "C" {
    pub fn mmc_select_voltage(host: *mut mmc_host, ocr: u32) -> u32;
}
extern "C" {
    pub fn mmc_set_uhs_voltage(host: *mut mmc_host, ocr: u32) -> c_int;
}
extern "C" {
    pub fn mmc_host_set_uhs_voltage(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_set_signal_voltage(host: *mut mmc_host, signal_voltage: c_int) -> c_int;
}
extern "C" {
    pub fn mmc_set_initial_signal_voltage(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_set_timing(host: *mut mmc_host, timing: c_uint);
}
extern "C" {
    pub fn mmc_set_driver_type(host: *mut mmc_host, drv_type: c_uint);
}
extern "C" {
    pub fn mmc_power_up(host: *mut mmc_host, ocr: u32);
}
extern "C" {
    pub fn mmc_power_off(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_power_cycle(host: *mut mmc_host, ocr: u32);
}
extern "C" {
    pub fn mmc_set_initial_state(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_vddrange_to_ocrmask(vdd_min: c_int, vdd_max: c_int) -> u32;
}
extern "C" {
    pub fn mmc_handle_undervoltage(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_regulator_register_undervoltage_notifier(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_regulator_unregister_undervoltage_notifier(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_undervoltage_workfn(work: *mut work_struct);
}
extern "C" {
    pub fn mmc_rescan(work: *mut work_struct);
}
extern "C" {
    pub fn mmc_start_host(host: *mut mmc_host);
}
extern "C" {
    pub fn __mmc_stop_host(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_stop_host(host: *mut mmc_host);
}
extern "C" {
    pub fn _mmc_detect_card_removed(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_detect_card_removed(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_attach_mmc(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_attach_sd(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_attach_sdio(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_attach_sd_uhs2(host: *mut mmc_host) -> c_int;
}
// Module parameters
// Debugfs information for hosts and cards

extern "C" {
    pub fn mmc_add_host_debugfs(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_remove_host_debugfs(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_add_card_debugfs(card: *mut mmc_card);
}
extern "C" {
    pub fn mmc_remove_card_debugfs(card: *mut mmc_card);
}

extern "C" {
    pub fn mmc_execute_tuning(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_hs200_to_hs400(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_hs400_to_hs200(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_wait_for_req_done(host: *mut mmc_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn mmc_is_req_done(host: *mut mmc_host, mrq: *mut mmc_request) -> bool;
}
extern "C" {
    pub fn mmc_start_request(host: *mut mmc_host, mrq: *mut mmc_request) -> c_int;
}
extern "C" {
    pub fn mmc_erase(card: *mut mmc_card, from: sector_t, nr: c_uint, arg: c_uint) -> c_int;
}
extern "C" {
    pub fn mmc_card_can_erase(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_card_can_trim(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_card_can_discard(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_card_can_sanitize(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_card_can_secure_erase_trim(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_card_can_cmd23(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_erase_group_aligned(card: *mut mmc_card, from: sector_t, nr: c_uint) -> c_int;
}
extern "C" {
    pub fn mmc_calc_max_discard(card: *mut mmc_card) -> c_uint;
}
extern "C" {
    pub fn mmc_set_blocklen(card: *mut mmc_card, blocklen: c_uint) -> c_int;
}
extern "C" {
    pub fn mmc_release_host(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_get_card(card: *mut mmc_card, ctx: *mut mmc_ctx);
}
extern "C" {
    pub fn mmc_put_card(card: *mut mmc_card, ctx: *mut mmc_ctx);
}
extern "C" {
    pub fn mmc_card_alternative_gpt_sector(card: *mut mmc_card, sector: *mut sector_t) -> c_int;
}
//
// mmc_claim_host - exclusively claim a host
// @host: mmc host to claim
//
// Claim a host for a set of operations.
//
extern "C" {
    pub fn mmc_cqe_start_req(host: *mut mmc_host, mrq: *mut mmc_request) -> c_int;
}
extern "C" {
    pub fn mmc_cqe_post_req(host: *mut mmc_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn mmc_cqe_recovery(host: *mut mmc_host) -> c_int;
}
//
// mmc_pre_req - Prepare for a new request
// @host: MMC host to prepare command
// @mrq: MMC request to prepare for
//
// mmc_pre_req() is called in prior to mmc_start_req() to let
// host prepare for the new request. Preparation of a request may be
// performed while another request is running on the host.
//
// mmc_post_req - Post process a completed request
// @host: MMC host to post process command
// @mrq: MMC request to post process for
// @err: Error, if non zero, clean up any resources made in pre_req
//
// Let the host post process a completed request. Post processing of
// a request may be performed while another request is running.
//
extern "C" {
    pub fn div_u64(_arg: dividend, _arg: divisor) -> return;
}
extern "C" {
    pub fn sector_div(_arg: dividend, _arg: divisor) -> return;
}
