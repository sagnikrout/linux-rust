//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/core.h
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

// Per-operation poll timeouts

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zl3073x_flags {
    ZL3073X_FLAG_REF_PHASE_COMP_32_BIT,
    ZL3073X_FLAG_DIE_TEMP_BIT,
    ZL3073X_FLAGS_NBITS /* must be last */
}

//
// struct zl3073x_chip_info - chip variant identification
// @id: chip ID
// @num_channels: number of DPLL channels supported by this variant
// @flags: chip variant flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_chip_info {
    pub id: u16,
    pub num_channels: u8,
    pub flags: c_ulong,
}

//
// struct zl3073x_dev - zl3073x device
// @dev: pointer to device
// @regmap: regmap to access device registers
// @info: detected chip info
// @multiop_lock: to serialize multiple register operations
// @tie_lock: to serialize TIE write operations
// @phase_step_lock: to serialize output phase step operations
// @ref: array of input references' invariants
// @out: array of outs' invariants
// @synth: array of synths' invariants
// @chan: array of DPLL channels' state
// @dplls: list of DPLLs
// @kworker: thread for periodic work
// @work: periodic work
// @clock_id: clock id of the device
// @out_step_time_mask: output step-time mask (device-global)
// @phase_avg_factor: phase offset measurement averaging factor
// @freq_monitor: is frequency monitor enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub info: *const zl3073x_chip_info,
    pub multiop_lock: mutex,
    pub tie_lock: mutex,
    pub phase_step_lock: mutex,
// Invariants
    pub ref: [zl3073x_ref; ZL3073X_NUM_REFS],
    pub out: [zl3073x_out; ZL3073X_NUM_OUTS],
    pub synth: [zl3073x_synth; ZL3073X_NUM_SYNTHS],
    pub chan: [zl3073x_chan; ZL3073X_MAX_CHANNELS],
// DPLL channels
    pub dplls: list_head,
// Monitor
    pub kworker: *mut kthread_worker,
    pub work: kthread_delayed_work,
// Per-chip parameters
    pub clock_id: u64,
    pub out_step_time_mask: u16,
    pub phase_avg_factor: u8,
    pub freq_monitor: bool,
}

extern "C" {
    pub fn zl3073x_dev_probe(zldev: *mut zl3073x_dev) -> c_int;
}
extern "C" {
    pub fn zl3073x_dev_start(zldev: *mut zl3073x_dev, full: bool) -> c_int;
}
extern "C" {
    pub fn zl3073x_dev_stop(zldev: *mut zl3073x_dev);
}
extern "C" {
    pub fn READ_ONCE(_arg: zldev->phase_avg_factor) -> return;
}
extern "C" {
    pub fn zl3073x_dev_phase_avg_factor_set(zldev: *mut zl3073x_dev, factor: u8) -> c_int;
}
//
// Registers operations
//
// struct zl3073x_hwreg_seq_item - HW register write sequence item
// @addr: HW register to be written
// @value: value to be written to HW register
// @mask: bitmask indicating bits to be updated
// @wait: number of ms to wait after register write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_hwreg_seq_item {
    pub addr: u32,
    pub value: u32,
    pub mask: u32,
    pub wait: u32,
}

extern "C" {
    pub fn zl3073x_read_u8(zldev: *mut zl3073x_dev, reg: c_uint, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_read_u16(zldev: *mut zl3073x_dev, reg: c_uint, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn zl3073x_read_u32(zldev: *mut zl3073x_dev, reg: c_uint, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn zl3073x_read_u48(zldev: *mut zl3073x_dev, reg: c_uint, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn zl3073x_write_u8(zldev: *mut zl3073x_dev, reg: c_uint, val: u8) -> c_int;
}
extern "C" {
    pub fn zl3073x_write_u16(zldev: *mut zl3073x_dev, reg: c_uint, val: u16) -> c_int;
}
extern "C" {
    pub fn zl3073x_write_u32(zldev: *mut zl3073x_dev, reg: c_uint, val: u32) -> c_int;
}
extern "C" {
    pub fn zl3073x_write_u48(zldev: *mut zl3073x_dev, reg: c_uint, val: u64) -> c_int;
}
extern "C" {
    pub fn zl3073x_read_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zl3073x_write_hwreg(zldev: *mut zl3073x_dev, addr: u32, value: u32) -> c_int;
}
//
// Misc operations
//
extern "C" {
    pub fn zl3073x_ref_phase_offsets_update(zldev: *mut zl3073x_dev, channel: c_int) -> c_int;
}
//
// zl3073x_dev_is_ref_phase_comp_32bit - check ref phase comp register size
// @zldev: pointer to zl3073x device
//
// Some chip IDs have a 32-bit wide ref_phase_offset_comp register instead
// of the default 48-bit.
//
// Return: true if the register is 32-bit, false if 48-bit
//
// P-pins ids are even while N-pins are odd
//
// zl3073x_input_pin_ref_get - get reference for given input pin
// @id: input pin id
//
// Return: reference id for the given input pin
//
// zl3073x_output_pin_out_get - get output for the given output pin
// @id: output pin id
//
// Return: output id for the given output pin
//
// Output pin pair shares the single output
//
// zl3073x_dev_ref_freq_get - get input reference frequency
// @zldev: pointer to zl3073x device
// @index: input reference index
//
// Return: frequency of given input reference
//
extern "C" {
    pub fn zl3073x_ref_freq_get(_arg: ref) -> return;
}
//
// zl3073x_dev_ref_is_diff - check if the given input reference is differential
// @zldev: pointer to zl3073x device
// @index: input reference index
//
// Return: true if reference is differential, false if reference is single-ended
//
extern "C" {
    pub fn zl3073x_ref_is_diff(_arg: ref) -> return;
}
//
// zl3073x_dev_ref_is_status_ok - check the given input reference status
// @zldev: pointer to zl3073x device
// @index: input reference index
//
// Return: true if the status is ok, false otherwise
//
extern "C" {
    pub fn zl3073x_ref_is_status_ok(_arg: ref) -> return;
}
//
// zl3073x_dev_synth_freq_get - get synth current freq
// @zldev: pointer to zl3073x device
// @index: synth index
//
// Return: frequency of given synthetizer
//
extern "C" {
    pub fn zl3073x_synth_freq_get(_arg: synth) -> return;
}
//
// zl3073x_dev_out_synth_get - get synth connected to given output
// @zldev: pointer to zl3073x device
// @index: output index
//
// Return: index of synth connected to given output.
//
extern "C" {
    pub fn zl3073x_out_synth_get(_arg: out) -> return;
}
//
// zl3073x_dev_out_is_enabled - check if the given output is enabled
// @zldev: pointer to zl3073x device
// @index: output index
//
// Return: true if the output is enabled, false otherwise
//
// Output is enabled only if associated synth is enabled
extern "C" {
    pub fn zl3073x_synth_is_enabled(zl3073x_out_is_enabled(out: synth) &&) -> return;
}
//
// zl3073x_dev_out_is_stepped - check if output is in step-time mask
// @zldev: pointer to zl3073x device
// @index: output index
//
// Return: true if output is affected by step-time operations
//
// zl3073x_dev_out_dpll_get - get DPLL ID the output is driven by
// @zldev: pointer to zl3073x device
// @index: output index
//
// Return: ID of DPLL the given output is driven by
//
// Get synthesizer connected to given output
// Return DPLL that drives the synth
extern "C" {
    pub fn zl3073x_synth_dpll_get(_arg: synth) -> return;
}
//
// zl3073x_dev_output_pin_freq_get - get output pin frequency
// @zldev: pointer to zl3073x device
// @id: output pin id
//
// Computes the output pin frequency based on the synth frequency, output
// divisor, and signal format. For N-div formats, N-pin frequency is
// additionally divided by esync_n_period.
//
// Return: frequency of the given output pin in Hz
//
// zl3073x_dev_out_is_diff - check if the given output is differential
// @zldev: pointer to zl3073x device
// @index: output index
//
// Return: true if output is differential, false if output is single-ended
//
extern "C" {
    pub fn zl3073x_out_is_diff(_arg: out) -> return;
}
//
// zl3073x_dev_output_pin_is_enabled - check if the given output pin is enabled
// @zldev: pointer to zl3073x device
// @id: output pin id
//
// Checks if the output of the given output pin is enabled and also that
// its signal format also enables the given pin.
//
// Return: true if output pin is enabled, false if output pin is disabled
//
// Check if the output is enabled - call _dev_ helper that
// additionally checks for attached synth enablement.
//
// Check signal format
// Both output pins are disabled by signal format
// Output is one single ended P-pin output
// Output is one single ended N-pin output
// For other format both pins are enabled
