//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dim.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2019 Mellanox Technologies.

// Number of DIM profiles and period mode.
pub const NET_DIM_PARAMS_NUM_PROFILES: c_int = 5;
pub const NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE: c_int = 256;
pub const NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE: c_int = 128;
pub const NET_DIM_DEF_PROFILE_CQE: c_int = 1;
pub const NET_DIM_DEF_PROFILE_EQE: c_int = 1;
//
// Number of events between DIM iterations.
// Causes a moderation of the algorithm run.
//
pub const DIM_NEVENTS: c_int = 64;
//
// Is a difference between values justifies taking an action.
// We consider 10% difference as significant.
//

//
// Calculate the gap between two values.
// Take wrap-around and variable size into consideration.
//

//
// struct dim_cq_moder - Structure for CQ moderation values.
// Used for communications between DIM and its consumer.
//
// @usec: CQ timer suggestion (by DIM)
// @pkts: CQ packet counter suggestion (by DIM)
// @comps: Completion counter
// @cq_period_mode: CQ period count mode (from CQE/EQE)
// @rcu: for asynchronous kfree_rcu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dim_cq_moder {
    pub usec: u16,
    pub pkts: u16,
    pub comps: u16,
    pub cq_period_mode: u8,
    pub rcu: rcu_head,
}

//
// struct dim_irq_moder - Structure for irq moderation information.
// Used to collect irq moderation related information.
//
// @profile_flags: DIM_PROFILE_
// @coal_flags: DIM_COALESCE_* for Rx and Tx
// @dim_rx_mode: Rx DIM period count mode: CQE or EQE
// @dim_tx_mode: Tx DIM period count mode: CQE or EQE
// @rx_profile: DIM profile list for Rx
// @tx_profile: DIM profile list for Tx
// @rx_dim_work: Rx DIM worker scheduled by net_dim()
// @tx_dim_work: Tx DIM worker scheduled by net_dim()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dim_irq_moder {
    pub profile_flags: u8,
    pub coal_flags: u8,
    pub dim_rx_mode: u8,
    pub dim_tx_mode: u8,
    pub rx_profile: *mut dim_cq_moder __rcu,
    pub tx_profile: *mut dim_cq_moder __rcu,
    pub work): *mut *mut void (rx_dim_work)(struct work_struct,
    pub work): *mut *mut void (tx_dim_work)(struct work_struct,
}

//
// struct dim_sample - Structure for DIM sample data.
// Used for communications between DIM and its consumer.
//
// @time: Sample timestamp
// @pkt_ctr: Number of packets
// @byte_ctr: Number of bytes
// @event_ctr: Number of events
// @comp_ctr: Current completion counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dim_sample {
    pub time: ktime_t,
    pub pkt_ctr: u32,
    pub byte_ctr: u32,
    pub event_ctr: u16,
    pub comp_ctr: u32,
}

//
// struct dim_stats - Structure for DIM stats.
// Used for holding current measured rates.
//
// @ppms: Packets per msec
// @bpms: Bytes per msec
// @epms: Events per msec
// @cpms: Completions per msec
// @cpe_ratio: Ratio of completions to events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dim_stats {
    pub /: *mut *mut int ppms; / packets per msec,
    pub /: *mut *mut int bpms; / bytes per msec,
    pub /: *mut *mut int epms; / events per msec,
    pub /: *mut *mut int cpms; / completions per msec,
    pub /: *mut *mut int cpe_ratio; / ratio of completions to events,
}

//
// struct dim - Main structure for dynamic interrupt moderation (DIM).
// Used for holding all information about a specific DIM instance.
//
// @state: Algorithm state (see below)
// @prev_stats: Measured rates from previous iteration (for comparison)
// @start_sample: Sampled data at start of current iteration
// @measuring_sample: A &dim_sample that is used to update the current events
// @work: Work to perform on action required
// @priv: A pointer to the struct that points to dim
// @profile_ix: Current moderation profile
// @mode: CQ period count mode
// @tune_state: Algorithm tuning state (see below)
// @steps_right: Number of steps taken towards higher moderation
// @steps_left: Number of steps taken towards lower moderation
// @tired: Parking depth counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dim {
    pub state: u8,
    pub prev_stats: dim_stats,
    pub start_sample: dim_sample,
    pub measuring_sample: dim_sample,
    pub work: work_struct,
    pub priv: *mut c_void,
    pub profile_ix: u8,
    pub mode: u8,
    pub tune_state: u8,
    pub steps_right: u8,
    pub steps_left: u8,
    pub tired: u8,
}

//
// enum dim_cq_period_mode - Modes for CQ period count
//
// @DIM_CQ_PERIOD_MODE_START_FROM_EQE: Start counting from EQE
// @DIM_CQ_PERIOD_MODE_START_FROM_CQE: Start counting from CQE (implies timer reset)
// @DIM_CQ_PERIOD_NUM_MODES: Number of modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dim_cq_period_mode {
    DIM_CQ_PERIOD_MODE_START_FROM_EQE = 0x0,
    DIM_CQ_PERIOD_MODE_START_FROM_CQE = 0x1,
    DIM_CQ_PERIOD_NUM_MODES
}

//
// enum dim_state - DIM algorithm states
//
// These will determine if the algorithm is in a valid state to start an iteration.
//
// @DIM_START_MEASURE: This is the first iteration (also after applying a new profile)
// @DIM_MEASURE_IN_PROGRESS: Algorithm is already in progress - check if
// need to perform an action
// @DIM_APPLY_NEW_PROFILE: DIM consumer is currently applying a profile - no need to measure
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dim_state {
    DIM_START_MEASURE,
    DIM_MEASURE_IN_PROGRESS,
    DIM_APPLY_NEW_PROFILE,
}

//
// enum dim_tune_state - DIM algorithm tune states
//
// These will determine which action the algorithm should perform.
//
// @DIM_PARKING_ON_TOP: Algorithm found a local top point - exit on significant difference
// @DIM_PARKING_TIRED: Algorithm found a deep top point - don't exit if tired > 0
// @DIM_GOING_RIGHT: Algorithm is currently trying higher moderation levels
// @DIM_GOING_LEFT: Algorithm is currently trying lower moderation levels
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dim_tune_state {
    DIM_PARKING_ON_TOP,
    DIM_PARKING_TIRED,
    DIM_GOING_RIGHT,
    DIM_GOING_LEFT,
}

//
// enum dim_stats_state - DIM algorithm statistics states
//
// These will determine the verdict of current iteration.
//
// @DIM_STATS_WORSE: Current iteration shows worse performance than before
// @DIM_STATS_SAME:  Current iteration shows same performance than before
// @DIM_STATS_BETTER: Current iteration shows better performance than before
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dim_stats_state {
    DIM_STATS_WORSE,
    DIM_STATS_SAME,
    DIM_STATS_BETTER,
}

//
// enum dim_step_result - DIM algorithm step results
//
// These describe the result of a step.
//
// @DIM_STEPPED: Performed a regular step
// @DIM_TOO_TIRED: Same kind of step was done multiple times - should go to
// tired parking
// @DIM_ON_EDGE: Stepped to the most left/right profile
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dim_step_result {
    DIM_STEPPED,
    DIM_TOO_TIRED,
    DIM_ON_EDGE,
}

//
// net_dim_init_irq_moder - collect information to initialize irq moderation
// @dev: target network device
// @profile_flags: Rx or Tx profile modification capability
// @coal_flags: irq moderation params flags
// @rx_mode: CQ period mode for Rx
// @tx_mode: CQ period mode for Tx
// @rx_dim_work: Rx worker called after dim decision
// @tx_dim_work: Tx worker called after dim decision
//
// Return: 0 on success or a negative error code.
//
// net_dim_free_irq_moder - free fields for irq moderation
// @dev: target network device
//
extern "C" {
    pub fn net_dim_free_irq_moder(dev: *mut net_device);
}
//
// net_dim_setting - initialize DIM's cq mode and schedule worker
// @dev: target network device
// @dim: DIM context
// @is_tx: true indicates the tx direction, false indicates the rx direction
//
extern "C" {
    pub fn net_dim_setting(dev: *mut net_device, dim: *mut dim, is_tx: bool);
}
//
// net_dim_work_cancel - synchronously cancel dim's worker
// @dim: DIM context
//
extern "C" {
    pub fn net_dim_work_cancel(dim: *mut dim);
}
//
// net_dim_get_rx_irq_moder - get DIM rx results based on profile_ix
// @dev: target network device
// @dim: DIM context
//
// Return: DIM irq moderation
//
// net_dim_get_tx_irq_moder - get DIM tx results based on profile_ix
// @dev: target network device
// @dim: DIM context
//
// Return: DIM irq moderation
//
// net_dim_set_rx_mode - set DIM rx cq mode
// @dev: target network device
// @rx_mode: target rx cq mode
//
extern "C" {
    pub fn net_dim_set_rx_mode(dev: *mut net_device, rx_mode: u8);
}
//
// net_dim_set_tx_mode - set DIM tx cq mode
// @dev: target network device
// @tx_mode: target tx cq mode
//
extern "C" {
    pub fn net_dim_set_tx_mode(dev: *mut net_device, tx_mode: u8);
}
//
// dim_on_top - check if current state is a good place to stop (top location)
// @dim: DIM context
//
// Check if current profile is a good place to park at.
// This will result in reducing the DIM checks frequency as we assume we
// shouldn't probably change profiles, unless traffic pattern wasn't changed.
//
extern "C" {
    pub fn dim_on_top(dim: *mut dim) -> bool;
}
//
// dim_turn - change profile altering direction
// @dim: DIM context
//
// Go left if we were going right and vice-versa.
// Do nothing if currently parking.
//
extern "C" {
    pub fn dim_turn(dim: *mut dim);
}
//
// dim_park_on_top - enter a parking state on a top location
// @dim: DIM context
//
// Enter parking state.
// Clear all movement history.
//
extern "C" {
    pub fn dim_park_on_top(dim: *mut dim);
}
//
// dim_park_tired - enter a tired parking state
// @dim: DIM context
//
// Enter parking state.
// Clear all movement history and cause DIM checks frequency to reduce.
//
extern "C" {
    pub fn dim_park_tired(dim: *mut dim);
}
//
// dim_calc_stats - calculate the difference between two samples
// @start: start sample
// @end: end sample
// @curr_stats: delta between samples
//
// Calculate the delta between two samples (in data rates).
// Takes into consideration counter wrap-around.
// Returned boolean indicates whether curr_stats are reliable.
//
// dim_update_sample - set a sample's fields with given values
// @event_ctr: number of events to set
// @packets: number of packets to set
// @bytes: number of bytes to set
// @s: DIM sample
//
// dim_update_sample_with_comps - set a sample's fields with given
// values including the completion parameter
// @event_ctr: number of events to set
// @packets: number of packets to set
// @bytes: number of bytes to set
// @comps: number of completions to set
// @s: DIM sample
//
// Net DIM
//
// net_dim_get_rx_moderation - provide a CQ moderation object for the given RX profile
// @cq_period_mode: CQ period mode
// @ix: Profile index
//
extern "C" {
    pub fn net_dim_get_rx_moderation(cq_period_mode: u8, ix: c_int) -> dim_cq_moder;
}
//
// net_dim_get_def_rx_moderation - provide the default RX moderation
// @cq_period_mode: CQ period mode
//
extern "C" {
    pub fn net_dim_get_def_rx_moderation(cq_period_mode: u8) -> dim_cq_moder;
}
//
// net_dim_get_tx_moderation - provide a CQ moderation object for the given TX profile
// @cq_period_mode: CQ period mode
// @ix: Profile index
//
extern "C" {
    pub fn net_dim_get_tx_moderation(cq_period_mode: u8, ix: c_int) -> dim_cq_moder;
}
//
// net_dim_get_def_tx_moderation - provide the default TX moderation
// @cq_period_mode: CQ period mode
//
extern "C" {
    pub fn net_dim_get_def_tx_moderation(cq_period_mode: u8) -> dim_cq_moder;
}
//
// net_dim - main DIM algorithm entry point
// @dim: DIM instance information
// @end_sample: Current data measurement
//
// Called by the consumer.
// This is the main logic of the algorithm, where data is processed in order
// to decide on next required action.
//
extern "C" {
    pub fn net_dim(dim: *mut dim, end_sample: *const dim_sample);
}
// RDMA DIM
//
// RDMA DIM profile:
// profile size must be of RDMA_DIM_PARAMS_NUM_PROFILES.
//
pub const RDMA_DIM_PARAMS_NUM_PROFILES: c_int = 9;
pub const RDMA_DIM_START_PROFILE: c_int = 0;
//
// rdma_dim - Runs the adaptive moderation.
// @dim: The moderation struct.
// @completions: The number of completions collected in this round.
//
// Each call to rdma_dim takes the latest amount of completions that
// have been collected and counts them as a new event.
// Once enough events have been collected the algorithm decides a new
// moderation level.
//
extern "C" {
    pub fn rdma_dim(dim: *mut dim, completions: u64);
}
