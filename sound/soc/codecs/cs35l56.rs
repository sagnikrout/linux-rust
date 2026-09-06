//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l56.h
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
// Driver for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS35L56_SDW_GEN_INT_STAT_1: c_uint = 0xc0;
pub const CS35L56_SDW_GEN_INT_MASK_1: c_uint = 0xc1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_private {
    pub /: *mut *mut wm_adsp dsp; / must be first member,
    pub base: cs35l56_base,
    pub dsp_work: work_struct,
    pub dsp_wq: *mut workqueue_struct,
    pub component: *mut snd_soc_component,
    pub supplies: [regulator_bulk_data; CS35L56_NUM_BULK_SUPPLIES],
    pub sdw_peripheral: *mut sdw_slave,
    pub sdw_bus_regmap: *mut regmap,
    pub fallback_fw_suffix: *const c_char,
    pub soft_resetting: bool,
    pub sdw_attached: bool,
    pub init_completion: completion,
    pub speaker_id: c_int,
    pub rx_mask: u32,
    pub tx_mask: u32,
    pub asp_slot_width: u8,
    pub asp_slot_count: u8,
    pub tdm_mode: bool,
    pub sysclk_set: bool,
    pub sdw_link_num: u8,
    pub sdw_unique_id: u8,
    pub ambient_ctl_value: u8,
}

extern "C" {
    pub fn container_of(_arg: cs35l56_base, cs35l56_private: struct, _arg: base) -> return;
}
extern "C" {
    pub fn cs35l56_mask_soundwire_interrupts(cs35l56: *mut cs35l56_private);
}
extern "C" {
    pub fn cs35l56_unmask_soundwire_interrupts(cs35l56: *mut cs35l56_private);
}
extern "C" {
    pub fn cs35l56_system_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_system_suspend_late(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_system_suspend_no_irq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_system_resume_no_irq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_system_resume_early(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_system_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs35l56_irq_request(cs35l56_base: *mut cs35l56_base, irq: c_int) -> c_int;
}
extern "C" {
    pub fn cs35l56_common_probe(cs35l56: *mut cs35l56_private, irq: c_int) -> c_int;
}
extern "C" {
    pub fn cs35l56_init(cs35l56: *mut cs35l56_private) -> c_int;
}
extern "C" {
    pub fn cs35l56_remove(cs35l56: *mut cs35l56_private);
}

extern "C" {
    pub fn cs35l56_set_fw_suffix(cs35l56: *mut cs35l56_private) -> c_int;
}
extern "C" {
    pub fn cs35l56_set_fw_name(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn cs35l56_process_xu_properties(cs35l56: *mut cs35l56_private) -> c_int;
}
extern "C" {
    pub fn cs35l56_get_firmware_uid(cs35l56: *mut cs35l56_private) -> c_int;
}

