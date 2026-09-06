//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda-mlink.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022-2023 Intel Corporation
//
// enum hda_bus_ml_link_type - mlink link type, used by SOF link DMA
// allocator constraints (see struct sof_intel_hda_dev).
//
// @HDA_BUS_ML_LINK_HDA:  non-alt link, i.e. HDA codec or iDisp
// @HDA_BUS_ML_LINK_SDW:  alt link, SoundWire
// @HDA_BUS_ML_LINK_UAOL: alt link, USB Audio Offload
// @HDA_BUS_ML_LINK_OTHER: alt link, SSP or DMIC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hda_bus_ml_link_type {
    HDA_BUS_ML_LINK_HDA,
    HDA_BUS_ML_LINK_SDW,
    HDA_BUS_ML_LINK_UAOL,
    HDA_BUS_ML_LINK_OTHER,
}

extern "C" {
    pub fn hda_bus_ml_init(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn hda_bus_ml_free(bus: *mut hdac_bus);
}
extern "C" {
    pub fn hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_enable_interrupt_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool);
}
extern "C" {
    pub fn hdac_bus_eml_enable_interrupt(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool);
}
extern "C" {
    pub fn hdac_bus_eml_check_interrupt(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool;
}
extern "C" {
    pub fn hdac_bus_eml_set_syncprd_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, syncprd: u32) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_set_syncprd_unlocked(bus: *mut hdac_bus, syncprd: u32) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_wait_syncpu_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_wait_syncpu_unlocked(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sync_arm_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int);
}
extern "C" {
    pub fn hdac_bus_eml_sdw_sync_arm_unlocked(bus: *mut hdac_bus, sublink: c_int);
}
extern "C" {
    pub fn hdac_bus_eml_sync_go_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_sync_go_unlocked(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_check_cmdsync_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_check_cmdsync_unlocked(bus: *mut hdac_bus) -> bool;
}
extern "C" {
    pub fn hdac_bus_eml_power_up(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_power_up_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_power_down(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_power_down_unlocked(bus: *mut hdac_bus, alt: bool, elid: c_int, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_power_up_unlocked(bus: *mut hdac_bus, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_power_down_unlocked(bus: *mut hdac_bus, sublink: c_int) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_get_lsdiid_unlocked(bus: *mut hdac_bus, sublink: c_int, lsdiid: *mut u16) -> c_int;
}
extern "C" {
    pub fn hdac_bus_eml_sdw_set_lsdiid(bus: *mut hdac_bus, sublink: c_int, dev_num: c_int) -> c_int;
}
extern "C" {
    pub fn hda_bus_ml_reset_losidv(bus: *mut hdac_bus);
}
extern "C" {
    pub fn hda_bus_ml_resume(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn hda_bus_ml_suspend(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn hda_bus_ml_link_get_type(hlink: *mut hdac_ext_link) -> hda_bus_ml_link_type;
}
extern "C" {
    pub fn hdac_bus_eml_enable_offload(bus: *mut hdac_bus, alt: bool, elid: c_int, enable: bool);
}
// microphone privacy specific function supported by ACE3+ architecture
extern "C" {
    pub fn hdac_bus_eml_is_mic_privacy_changed(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool;
}
extern "C" {
    pub fn hdac_bus_eml_get_mic_privacy_state(bus: *mut hdac_bus, alt: bool, elid: c_int) -> bool;
}

