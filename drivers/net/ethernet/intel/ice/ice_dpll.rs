//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_dpll.h
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
// Copyright (C) 2022, Intel Corporation.

pub const ICE_DPLL_RCLK_NUM_MAX: c_int = 4;
pub const ICE_DPLL_TXCLK_NUM_MAX: c_int = 2;
pub const E825_EXT_EREF_PIN_IDX: c_int = 0;
pub const E825_EXT_SYNCE_PIN_IDX: c_int = 1;
pub const ICE_CGU_R10: c_uint = 0x28;

pub const ICE_CGU_R11: c_uint = 0x2C;

pub const ICE_CGU_BYPASS_MUX_OFFSET_E825C: c_int = 3;
//
// enum ice_dpll_pin_sw - enumerate ice software pin indices:
// @ICE_DPLL_PIN_SW_1_IDX: index of first SW pin
// @ICE_DPLL_PIN_SW_2_IDX: index of second SW pin
// @ICE_DPLL_PIN_SW_NUM: number of SW pins in pair
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_dpll_pin_sw {
    ICE_DPLL_PIN_SW_1_IDX,
    ICE_DPLL_PIN_SW_2_IDX,
    ICE_DPLL_PIN_SW_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dpll_pin_work {
    pub work: work_struct,
    pub action: c_ulong,
    pub pin: *mut ice_dpll_pin,
}

// ice_dpll_pin - store info about pins
// @pin: dpll pin structure
// @pf: pointer to pf, which has registered the dpll_pin
// @tracker: reference count tracker
// @idx: ice pin private idx
// @num_parents: hols number of parent pins
// @parent_idx: hold indexes of parent pins
// @flags: pin flags returned from HW
// @state: state of a pin
// @prop: pin properties
// @freq: current frequency of a pin
// @phase_adjust: current phase adjust value
// @phase_offset: monitored phase offset value
// @ref_sync: store id of reference sync pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dpll_pin {
    pub pin: *mut dpll_pin,
    pub pf: *mut ice_pf,
    pub tracker: dpll_tracker,
    pub fwnode: *mut fwnode_handle,
    pub nb: notifier_block,
    pub idx: u8,
    pub num_parents: u8,
    pub parent_idx: [u8; ICE_DPLL_RCLK_NUM_MAX],
    pub flags: [u8; ICE_DPLL_RCLK_NUM_MAX],
    pub state: [u8; ICE_DPLL_RCLK_NUM_MAX],
    pub prop: dpll_pin_properties,
    pub freq: u32,
    pub phase_adjust: i32,
    pub input: *mut ice_dpll_pin,
    pub output: *mut ice_dpll_pin,
    pub direction: dpll_pin_direction,
    pub phase_offset: i64,
    pub status: u8,
    pub ref_sync: u8,
    pub active: bool,
    pub hidden: bool,
    pub tx_ref_src: ice_e825c_ref_clk,
}

// ice_dpll - store info required for DPLL control
// @dpll: pointer to dpll dev
// @pf: pointer to pf, which has registered the dpll_device
// @tracker: reference count tracker
// @dpll_idx: index of dpll on the NIC
// @input_idx: currently selected input index
// @prev_input_idx: previously selected input index
// @ref_state: state of dpll reference signals
// @eec_mode: eec_mode dpll is configured for
// @phase_offset: phase offset of active pin vs dpll signal
// @prev_phase_offset: previous phase offset of active pin vs dpll signal
// @input_prio: priorities of each input
// @dpll_state: current dpll sync state
// @prev_dpll_state: last dpll sync state
// @phase_offset_monitor_period: period for phase offset monitor read frequency
// @active_input: pointer to active input pin
// @prev_input: pointer to previous active input pin
// @ops: holds the registered ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dpll {
    pub dpll: *mut dpll_device,
    pub pf: *mut ice_pf,
    pub tracker: dpll_tracker,
    pub dpll_idx: u8,
    pub input_idx: u8,
    pub prev_input_idx: u8,
    pub ref_state: u8,
    pub eec_mode: u8,
    pub phase_offset: i64,
    pub prev_phase_offset: i64,
    pub input_prio: *mut u8,
    pub dpll_state: dpll_lock_status,
    pub prev_dpll_state: dpll_lock_status,
    pub mode: dpll_mode,
    pub phase_offset_monitor_period: u32,
    pub active_input: *mut dpll_pin,
    pub prev_input: *mut dpll_pin,
    pub ops: *const dpll_device_ops,
}

// ice_dplls - store info required for CCU (clock controlling unit)
// @kworker: periodic worker
// @work: periodic work
// @wq: workqueue used to schedule DPLL-related deferred work
// @lock: protects DPLL configuration (see Locking below)
// @eec: pointer to EEC dpll dev
// @pps: pointer to PPS dpll dev
// @txc: pointer to TXC dpll dev
// @inputs: input pins pointer
// @outputs: output pins pointer
// @rclk: recovered pins pointer
// @txclks: TX clock reference pins pointer
// @num_inputs: number of input pins available on dpll
// @num_outputs: number of output pins available on dpll
// @cgu_state_acq_err_num: number of errors returned during periodic work
// @base_rclk_idx: idx of first pin used for clock revocery pins
// @clock_id: clock_id of dplls
// @input_phase_adj_max: max phase adjust value for an input pins
// @output_phase_adj_max: max phase adjust value for an output pins
// @periodic_counter: counter of periodic work executions
// @generic: true when generic DPLL ops are used
// @txclk_work: deferred TX reference clock switch worker
// @txclk_switch_requested: a TX ref clock switch is queued in @txclk_work
// @txclk_notify_rwsem: drains in-flight TXCLK notifications on teardown
//
// Locking:
// Acquisition order (top to bottom):
//
// txclk_notify_rwsem (read)
// -> pf->dplls.lock
// -> ctrl_pf->dplls.lock
//
// - @lock serializes all DPLL state mutations on this PF. When the
// controlling PF's lock must also be taken (e.g. updating the shared
// tx_refclks usage map), acquire pf->dplls.lock first, then
// ctrl_pf->dplls.lock. Skip the second acquire when pf == ctrl_pf
// to avoid recursive locking.
// - @txclk_notify_rwsem is held for read across
// ice_txclk_update_and_notify(), including the out-of-lock
// dpll_*_change_ntf() calls. ice_dpll_deinit() takes the write side
// standalone (not nested under any other lock) to drain in-flight
// readers before pins and the TXC DPLL device are freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dplls {
    pub kworker: *mut kthread_worker,
    pub work: kthread_delayed_work,
    pub wq: *mut workqueue_struct,
    pub lock: mutex,
    pub dpll_init: completion,
    pub eec: ice_dpll,
    pub pps: ice_dpll,
    pub txc: ice_dpll,
    pub inputs: *mut ice_dpll_pin,
    pub outputs: *mut ice_dpll_pin,
    pub sma: [ice_dpll_pin; ICE_DPLL_PIN_SW_NUM],
    pub ufl: [ice_dpll_pin; ICE_DPLL_PIN_SW_NUM],
    pub rclk: ice_dpll_pin,
    pub txclks: [ice_dpll_pin; ICE_DPLL_TXCLK_NUM_MAX],
    pub num_inputs: u8,
    pub num_outputs: u8,
    pub sma_data: u8,
    pub base_rclk_idx: u8,
    pub cgu_state_acq_err_num: c_int,
    pub clock_id: u64,
    pub input_phase_adj_max: i32,
    pub output_phase_adj_max: i32,
    pub periodic_counter: u32,
    pub generic: bool,
    pub txclk_work: work_struct,
    pub txclk_switch_requested: bool,
    pub txclk_notify_rwsem: rw_semaphore,
}

extern "C" {
    pub fn ice_dpll_init(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_dpll_deinit(pf: *mut ice_pf);
}

