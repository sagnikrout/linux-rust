//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/mmc_ops.h
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
// linux/drivers/mmc/core/mmc_ops.h
//
// Copyright 2006-2007 Pierre Ossman
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmc_busy_cmd {
    MMC_BUSY_CMD6,
    MMC_BUSY_ERASE,
    MMC_BUSY_HPI,
    MMC_BUSY_EXTR_SINGLE,
    MMC_BUSY_IO,
}

extern "C" {
    pub fn mmc_select_card(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_deselect_cards(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_set_dsr(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn __mmc_go_idle(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_go_idle(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_send_op_cond(host: *mut mmc_host, ocr: u32, rocr: *mut u32) -> c_int;
}
extern "C" {
    pub fn mmc_set_relative_addr(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_send_csd(card: *mut mmc_card, csd: *mut u32) -> c_int;
}
extern "C" {
    pub fn __mmc_send_status(card: *mut mmc_card, status: *mut u32, retries: c_uint) -> c_int;
}
extern "C" {
    pub fn mmc_send_cid(host: *mut mmc_host, cid: *mut u32) -> c_int;
}
extern "C" {
    pub fn mmc_spi_read_ocr(host: *mut mmc_host, highcap: c_int, ocrp: *mut u32) -> c_int;
}
extern "C" {
    pub fn mmc_spi_set_crc(host: *mut mmc_host, use_crc: c_int) -> c_int;
}
extern "C" {
    pub fn mmc_bus_test(card: *mut mmc_card, bus_width: u8) -> c_int;
}
extern "C" {
    pub fn mmc_card_can_ext_csd(card: *mut mmc_card) -> bool;
}
extern "C" {
    pub fn mmc_switch_status(card: *mut mmc_card, crc_err_fatal: bool) -> c_int;
}
extern "C" {
    pub fn mmc_run_bkops(card: *mut mmc_card);
}
extern "C" {
    pub fn mmc_cmdq_enable(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_cmdq_disable(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_sanitize(card: *mut mmc_card, timeout_ms: c_uint) -> c_int;
}
