//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/si476x-core.h
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
// include/media/si476x-core.h -- Common definitions for si476x core
// device
//
// Copyright (C) 2012 Innovative Converged Devices(ICD)
// Copyright (C) 2013 Andrey Smirnov
//
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//

// Command Timeouts
pub const SI476X_DEFAULT_TIMEOUT: c_int = 100000;
pub const SI476X_TIMEOUT_TUNE: c_int = 700000;
pub const SI476X_TIMEOUT_POWER_UP: c_int = 330000;
pub const SI476X_STATUS_POLL_US: c_int = 0;
// -------------------- si476x-i2c.c -----------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_freq_supported_chips {
    SI476X_CHIP_SI4761 = 1,
    SI476X_CHIP_SI4764,
    SI476X_CHIP_SI4768,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_part_revisions {
    SI476X_REVISION_A10 = 0,
    SI476X_REVISION_A20 = 1,
    SI476X_REVISION_A30 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_mfd_cells {
    SI476X_RADIO_CELL = 0,
    SI476X_CODEC_CELL,
    SI476X_MFD_CELLS,
}

//
// enum si476x_power_state - possible power state of the si476x
// device.
//
// @SI476X_POWER_DOWN: In this state all regulators are turned off
// and the reset line is pulled low. The device is completely
// inactive.
// @SI476X_POWER_UP_FULL: In this state all the power regulators are
// turned on, reset line pulled high, IRQ line is enabled(polling is
// active for polling use scenario) and device is turned on with
// POWER_UP command. The device is ready to be used.
// @SI476X_POWER_INCONSISTENT: This state indicates that previous
// power down was inconsistent, meaning some of the regulators were
// not turned down and thus use of the device, without power-cycling
// is impossible.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_power_state {
    SI476X_POWER_DOWN		= 0,
    SI476X_POWER_UP_FULL		= 1,
    SI476X_POWER_INCONSISTENT	= 2,
}

//
// struct si476x_core - internal data structure representing the
// underlying "core" device which all the MFD cell-devices use.
//
// @client: Actual I2C client used to transfer commands to the chip.
// @regmap: Regmap for accessing the device registers
// @chip_id: Last digit of the chip model(E.g. "1" for SI4761)
// @cells: MFD cell devices created by this driver.
// @cmd_lock: Mutex used to serialize all the requests to the core
// device. This filed should not be used directly. Instead
// si476x_core_lock()/si476x_core_unlock() should be used to get
// exclusive access to the "core" device.
// @users: Active users counter(Used by the radio cell)
// @rds_read_queue: Wait queue used to wait for RDS data.
// @rds_fifo: FIFO in which all the RDS data received from the chip is
// placed.
// @rds_fifo_drainer: Worker that drains on-chip RDS FIFO.
// @rds_drainer_is_working: Flag used for launching only one instance
// of the @rds_fifo_drainer.
// @rds_drainer_status_lock: Lock used to guard access to the
// @rds_drainer_is_working variable.
// @command: Wait queue for wainting on the command comapletion.
// @cts: Clear To Send flag set upon receiving first status with CTS
// set.
// @tuning: Wait queue used for wainting for tune/seek comand
// completion.
// @stc: Similar to @cts, but for the STC bit of the status value.
// @power_up_parameters: Parameters used as argument for POWER_UP
// command when the device is started.
// @power_state: Current power state of the device.
// @supplies: Structure containing handles to all power supplies used
// by the device (NULL ones are ignored).
// @reset: GPIO connected to the RSTB pin of the chip.
// @pinmux: Chip's configurable pins configuration.
// @diversity_mode: Chips role when functioning in diversity mode.
// @is_alive: Chip is initialized and active.
// @status_monitor: Polling worker used in polling use case scenarion
// (when IRQ is not avalible).
// @revision: Chip's running firmware revision number(Used for correct
// command set support).
// @rds_fifo_depth: RDS FIFO size: 20 for IRQ mode or 5 for polling mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_core {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub chip_id: c_int,
    pub cells: [mfd_cell; SI476X_MFD_CELLS],
    pub /: *mut *mut mutex cmd_lock; / for serializing fm radio operations,
    pub users: core::sync::atomic::AtomicI32,
    pub rds_read_queue: wait_queue_head_t,
    pub rds_fifo: kfifo,
    pub rds_fifo_drainer: work_struct,
    pub rds_drainer_is_working: bool,
    pub rds_drainer_status_lock: mutex,
    pub command: wait_queue_head_t,
    pub cts: core::sync::atomic::AtomicI32,
    pub tuning: wait_queue_head_t,
    pub stc: core::sync::atomic::AtomicI32,
    pub power_up_parameters: si476x_power_up_args,
    pub power_state: si476x_power_state,
    pub supplies: [regulator_bulk_data; 4],
    pub reset: *mut gpio_desc,
    pub pinmux: si476x_pinmux,
    pub diversity_mode: si476x_phase_diversity_mode,
    pub is_alive: core::sync::atomic::AtomicI32,
    pub status_monitor: delayed_work,

    pub revision: c_int,
    pub rds_fifo_depth: c_int,
}

extern "C" {
    pub fn i2c_get_clientdata(_arg: client) -> return;
}
//
// si476x_core_lock() - lock the core device to get an exclusive access
// to it.
// @core: Core device structure
//
// si476x_core_unlock() - unlock the core device to relinquish an
// exclusive access to it.
// @core: Core device structure
//
// *_TUNE_FREQ family of commands accept frequency in multiples of
// Since the V4L2_TUNER_CAP_LOW flag is supplied, V4L2 subsystem
// mesures frequency in 62.5 Hz units
extern "C" {
    pub fn hz_to_si476x(_arg: core, _arg: v4l2_to_hz(freq)) -> return;
}
extern "C" {
    pub fn hz_to_v4l2(_arg: si476x_to_hz(core, _arg: freq)) -> return;
}
//
// struct si476x_func_info - structure containing result of the
// FUNC_INFO command.
//
// @firmware: Firmware version numbers.
// @firmware.major: Firmware major number.
// @firmware.minor[...]: Firmware minor numbers.
// @patch_id: Firmware patch level.
// @func: Mode tuner is working in.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_func_info {
    pub minor: [u8 major,; 2],
    pub firmware: },
    pub patch_id: u16,
    pub func: si476x_func,
}

//
// struct si476x_power_down_args - structure used to pass parameters
// to POWER_DOWN command
//
// @xosc: true - Power down, but leav oscillator running.
// false - Full power down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_power_down_args {
    pub xosc: bool,
}

//
// enum si476x_tunemode - enum representing possible tune modes for
// the chip.
// @SI476X_TM_VALIDATED_NORMAL_TUNE: Unconditionally stay on the new
// channel after tune, tune status is valid.
// @SI476X_TM_INVALIDATED_FAST_TUNE: Unconditionally stay in the new
// channel after tune, tune status invalid.
// @SI476X_TM_VALIDATED_AF_TUNE: Jump back to previous channel if
// metric thresholds are not met.
// @SI476X_TM_VALIDATED_AF_CHECK: Unconditionally jump back to the
// previous channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_tunemode {
    SI476X_TM_VALIDATED_NORMAL_TUNE = 0,
    SI476X_TM_INVALIDATED_FAST_TUNE = 1,
    SI476X_TM_VALIDATED_AF_TUNE     = 2,
    SI476X_TM_VALIDATED_AF_CHECK    = 3,
}

//
// enum si476x_smoothmetrics - enum containing the possible setting fo
// audio transitioning of the chip
// @SI476X_SM_INITIALIZE_AUDIO: Initialize audio state to match this
// new channel
// @SI476X_SM_TRANSITION_AUDIO: Transition audio state from previous
// channel values to the new values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_smoothmetrics {
    SI476X_SM_INITIALIZE_AUDIO = 0,
    SI476X_SM_TRANSITION_AUDIO = 1,
}

//
// struct si476x_rds_status_report - the structure representing the
// response to 'FM_RD_STATUS' command
// @rdstpptyint: Traffic program flag(TP) and/or program type(PTY)
// code has changed.
// @rdspiint: Program identification(PI) code has changed.
// @rdssyncint: RDS synchronization has changed.
// @rdsfifoint: RDS was received and the RDS FIFO has at least
// 'FM_RDS_INTERRUPT_FIFO_COUNT' elements in it.
// @tpptyvalid: TP flag and PTY code are valid falg.
// @pivalid: PI code is valid flag.
// @rdssync: RDS is currently synchronized.
// @rdsfifolost: On or more RDS groups have been lost/discarded flag.
// @tp: Current channel's TP flag.
// @pty: Current channel's PTY code.
// @pi: Current channel's PI code.
// @rdsfifoused: Number of blocks remaining in the RDS FIFO (0 if empty).
// @ble:
// @rds: RDS data descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_rds_status_report {
    pub rdsfifoint: bool rdstpptyint, rdspiint, rdssyncint,,
    pub rdsfifolost: bool tpptyvalid, pivalid, rdssync,,
    pub tp: bool,
    pub pty: u8,
    pub pi: u16,
    pub rdsfifoused: u8,
    pub ble: [u8; 4],
    pub rds: [v4l2_rds_data; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_rsq_status_args {
    pub primary: bool,
    pub rsqack: bool,
    pub attune: bool,
    pub cancel: bool,
    pub stcack: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_injside {
    SI476X_INJSIDE_AUTO	= 0,
    SI476X_INJSIDE_LOW	= 1,
    SI476X_INJSIDE_HIGH	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si476x_tune_freq_args {
    pub zifsr: bool,
    pub hd: bool,
    pub injside: si476x_injside,
    pub freq: c_int,
    pub tunemode: si476x_tunemode,
    pub smoothmetrics: si476x_smoothmetrics,
    pub antcap: c_int,
}

extern "C" {
    pub fn si476x_core_stop(: *mut si476x_core, _arg: bool) -> c_int;
}
extern "C" {
    pub fn si476x_core_start(: *mut si476x_core, _arg: bool) -> c_int;
}
extern "C" {
    pub fn si476x_core_set_power_state(: *mut si476x_core, si476x_power_state: enum) -> c_int;
}
extern "C" {
    pub fn si476x_core_has_am(: *mut si476x_core) -> bool;
}
extern "C" {
    pub fn si476x_core_has_diversity(: *mut si476x_core) -> bool;
}
extern "C" {
    pub fn si476x_core_is_a_secondary_tuner(: *mut si476x_core) -> bool;
}
extern "C" {
    pub fn si476x_core_is_a_primary_tuner(: *mut si476x_core) -> bool;
}
extern "C" {
    pub fn si476x_core_is_in_am_receiver_mode(core: *mut si476x_core) -> bool;
}
extern "C" {
    pub fn si476x_core_is_powered_up(core: *mut si476x_core) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_i2c_type {
    SI476X_I2C_SEND,
    SI476X_I2C_RECV
}

// -------------------- si476x-cmd.c -----------------------
extern "C" {
    pub fn si476x_core_cmd_func_info(: *mut si476x_core, : *mut si476x_func_info) -> c_int;
}
extern "C" {
    pub fn si476x_core_cmd_set_property(: *mut si476x_core, _arg: u16, _arg: u16) -> c_int;
}
extern "C" {
    pub fn si476x_core_cmd_get_property(: *mut si476x_core, _arg: u16) -> c_int;
}
extern "C" {
    pub fn si476x_core_cmd_fm_seek_start(: *mut si476x_core, _arg: bool, _arg: bool) -> c_int;
}
extern "C" {
    pub fn si476x_core_cmd_am_seek_start(: *mut si476x_core, _arg: bool, _arg: bool) -> c_int;
}
extern "C" {
    pub fn si476x_core_cmd_fm_phase_div_status(: *mut si476x_core) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_power_grid_type {
    SI476X_POWER_GRID_50HZ = 0,
    SI476X_POWER_GRID_60HZ,
}

// Properties
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_interrupt_flags {
    SI476X_STCIEN = (1 << 0),
    SI476X_ACFIEN = (1 << 1),
    SI476X_RDSIEN = (1 << 2),
    SI476X_RSQIEN = (1 << 3),

    SI476X_ERRIEN = (1 << 6),
    SI476X_CTSIEN = (1 << 7),

    SI476X_STCREP = (1 << 8),
    SI476X_ACFREP = (1 << 9),
    SI476X_RDSREP = (1 << 10),
    SI476X_RSQREP = (1 << 11),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_rdsint_sources {
    SI476X_RDSTPPTY = (1 << 4),
    SI476X_RDSPI    = (1 << 3),
    SI476X_RDSSYNC	= (1 << 1),
    SI476X_RDSRECV	= (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_status_response_bits {
    SI476X_CTS	  = (1 << 7),
    SI476X_ERR	  = (1 << 6),
// Status response for WB receiver
    SI476X_WB_ASQ_INT = (1 << 4),
    SI476X_RSQ_INT    = (1 << 3),
// Status response for FM receiver
    SI476X_FM_RDS_INT = (1 << 2),
    SI476X_ACF_INT    = (1 << 1),
    SI476X_STC_INT    = (1 << 0),
}

// -------------------- si476x-prop.c -----------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_common_receiver_properties {
    SI476X_PROP_INT_CTL_ENABLE			= 0x0000,
    SI476X_PROP_DIGITAL_IO_INPUT_SAMPLE_RATE	= 0x0200,
    SI476X_PROP_DIGITAL_IO_INPUT_FORMAT		= 0x0201,
    SI476X_PROP_DIGITAL_IO_OUTPUT_SAMPLE_RATE	= 0x0202,
    SI476X_PROP_DIGITAL_IO_OUTPUT_FORMAT		= 0x0203,

    SI476X_PROP_SEEK_BAND_BOTTOM			= 0x1100,
    SI476X_PROP_SEEK_BAND_TOP			= 0x1101,
    SI476X_PROP_SEEK_FREQUENCY_SPACING		= 0x1102,

    SI476X_PROP_VALID_MAX_TUNE_ERROR		= 0x2000,
    SI476X_PROP_VALID_SNR_THRESHOLD			= 0x2003,
    SI476X_PROP_VALID_RSSI_THRESHOLD		= 0x2004,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_am_receiver_properties {
    SI476X_PROP_AUDIO_PWR_LINE_FILTER		= 0x0303,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_fm_receiver_properties {
    SI476X_PROP_AUDIO_DEEMPHASIS			= 0x0302,

    SI476X_PROP_FM_RDS_INTERRUPT_SOURCE		= 0x4000,
    SI476X_PROP_FM_RDS_INTERRUPT_FIFO_COUNT		= 0x4001,
    SI476X_PROP_FM_RDS_CONFIG			= 0x4002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_prop_audio_pwr_line_filter_bits {
    SI476X_PROP_PWR_HARMONICS_MASK	= 0x001f,
    SI476X_PROP_PWR_GRID_MASK	= 0x0100,
    SI476X_PROP_PWR_ENABLE_MASK	= 0x0200,
    SI476X_PROP_PWR_GRID_50HZ	= 0x0000,
    SI476X_PROP_PWR_GRID_60HZ	= 0x0100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_prop_fm_rds_config_bits {
    SI476X_PROP_RDSEN_MASK	= 0x1,
    SI476X_PROP_RDSEN	= 0x1,
}
