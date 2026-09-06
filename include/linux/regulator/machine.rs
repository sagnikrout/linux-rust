//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/machine.h
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
// machine.h -- SoC Regulator support, machine/board driver API.
//
// Copyright (C) 2007, 2008 Wolfson Microelectronics PLC.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//
// Regulator Machine/Board Interface.
//

//
// Regulator operation constraint flags. These flags are used to enable
// certain regulator operations and can be OR'ed together.
//
// VOLTAGE:  Regulator output voltage can be changed by software on this
// board/machine.
// CURRENT:  Regulator output current can be changed by software on this
// board/machine.
// MODE:     Regulator operating mode can be changed by software on this
// board/machine.
// STATUS:   Regulator can be enabled and disabled.
// DRMS:     Dynamic Regulator Mode Switching is enabled for this regulator.
// BYPASS:   Regulator can be put into bypass mode
//
pub const REGULATOR_CHANGE_VOLTAGE: c_uint = 0x1;
pub const REGULATOR_CHANGE_CURRENT: c_uint = 0x2;
pub const REGULATOR_CHANGE_MODE: c_uint = 0x4;
pub const REGULATOR_CHANGE_STATUS: c_uint = 0x8;
pub const REGULATOR_CHANGE_DRMS: c_uint = 0x10;
pub const REGULATOR_CHANGE_BYPASS: c_uint = 0x20;
//
// operations in suspend mode
// DO_NOTHING_IN_SUSPEND - the default value
// DISABLE_IN_SUSPEND	- turn off regulator in suspend states
// ENABLE_IN_SUSPEND	- keep regulator on in suspend states
//
pub const DO_NOTHING_IN_SUSPEND: c_int = 0;
pub const DISABLE_IN_SUSPEND: c_int = 1;
pub const ENABLE_IN_SUSPEND: c_int = 2;
//
// Default time window (in milliseconds) following a critical under-voltage
// event during which less critical actions can be safely carried out by the
// system.
//
pub const REGULATOR_DEF_UV_LESS_CRITICAL_WINDOW_MS: c_int = 10;
// Regulator active discharge flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulator_active_discharge {
    REGULATOR_ACTIVE_DISCHARGE_DEFAULT,
    REGULATOR_ACTIVE_DISCHARGE_DISABLE,
    REGULATOR_ACTIVE_DISCHARGE_ENABLE,
}

//
// struct regulator_state - regulator state during low power system states
//
// This describes a regulators state during a system wide low power
// state.  One of enabled or disabled must be set for the
// configuration to be applied.
//
// @uV: Default operating voltage during suspend, it can be adjusted
// among <min_uV, max_uV>.
// @min_uV: Minimum suspend voltage may be set.
// @max_uV: Maximum suspend voltage may be set.
// @mode: Operating mode during suspend.
// @enabled: operations during suspend.
// - DO_NOTHING_IN_SUSPEND
// - DISABLE_IN_SUSPEND
// - ENABLE_IN_SUSPEND
// @changeable: Is this state can be switched between enabled/disabled,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_state {
    pub uV: c_int,
    pub min_uV: c_int,
    pub max_uV: c_int,
    pub mode: c_uint,
    pub enabled: c_int,
    pub changeable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notification_limit {
    pub prot: c_int,
    pub err: c_int,
    pub warn: c_int,
}

//
// struct regulation_constraints - regulator operating constraints.
//
// This struct describes regulator and board/machine specific constraints.
//
// @name: Descriptive name for the constraints, used for display purposes.
//
// @min_uV: Smallest voltage consumers may set.
// @max_uV: Largest voltage consumers may set.
// @uV_offset: Offset applied to voltages from consumer to compensate for
// voltage drops.
//
// @min_uA: Smallest current consumers may set.
// @max_uA: Largest current consumers may set.
// @ilim_uA: Maximum input current.
// @pw_budget_mW: Power budget for the regulator in mW.
// @system_load: Load that isn't captured by any consumer requests.
//
// @over_curr_limits:		Limits for acting on over current.
// @over_voltage_limits:	Limits for acting on over voltage.
// @under_voltage_limits:	Limits for acting on under voltage.
// @temp_limits:		Limits for acting on over temperature.
//
// @max_spread: Max possible spread between coupled regulators
// @max_uV_step: Max possible step change in voltage
// @valid_modes_mask: Mask of modes which may be configured by consumers.
// @valid_ops_mask: Operations which may be performed by consumers.
//
// @always_on: Set if the regulator should never be disabled.
// @boot_on: Set if the regulator is enabled when the system is initially
// started.  If the regulator is not enabled by the hardware or
// bootloader then it will be enabled when the constraints are
// applied.
// @apply_uV: Apply the voltage constraint when initialising.
// @ramp_disable: Disable ramp delay when initialising or when setting voltage.
// @soft_start: Enable soft start so that voltage ramps slowly.
// @pull_down: Enable pull down when regulator is disabled.
// @system_critical: Set if the regulator is critical to system stability or
// functionality.
// @over_current_protection: Auto disable on over current event.
//
// @over_current_detection: Configure over current limits.
// @over_voltage_detection: Configure over voltage limits.
// @under_voltage_detection: Configure under voltage limits.
// @over_temp_detection: Configure over temperature limits.
//
// @input_uV: Input voltage for regulator when supplied by another regulator.
//
// @state_disk: State for regulator when system is suspended in disk mode.
// @state_mem: State for regulator when system is suspended in mem mode.
// @state_standby: State for regulator when system is suspended in standby
// mode.
// @initial_state: Suspend state to set by default.
// @initial_mode: Mode to set at startup.
// @ramp_delay: Time to settle down after voltage change (unit: uV/us)
// @settling_time: Time to settle down after voltage change when voltage
// change is non-linear (unit: microseconds).
// @settling_time_up: Time to settle down after voltage increase when voltage
// change is non-linear (unit: microseconds).
// @settling_time_down : Time to settle down after voltage decrease when
// voltage change is non-linear (unit: microseconds).
// @active_discharge: Enable/disable active discharge. The enum
// regulator_active_discharge values are used for
// initialisation.
// @enable_time: Turn-on time of the rails (unit: microseconds)
// @uv_less_critical_window_ms: Specifies the time window (in milliseconds)
// following a critical under-voltage (UV) event
// during which less critical actions can be
// safely carried out by the system (for example
// logging). After this time window more critical
// actions should be done (for example prevent
// HW damage).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulation_constraints {
    pub name: *const c_char,
// voltage output range (inclusive) - for voltage control
    pub min_uV: c_int,
    pub max_uV: c_int,
    pub uV_offset: c_int,
// current output range (inclusive) - for current control
    pub min_uA: c_int,
    pub max_uA: c_int,
    pub ilim_uA: c_int,
    pub pw_budget_mW: c_int,
    pub system_load: c_int,
// used for coupled regulators
    pub max_spread: *mut u32,
// used for changing voltage in steps
    pub max_uV_step: c_int,
// valid regulator operating modes for this machine
    pub valid_modes_mask: c_uint,
// valid operations for regulator on this machine
    pub valid_ops_mask: c_uint,
// regulator input voltage - only if supply is another regulator
    pub input_uV: c_int,
// regulator suspend states for global PMIC STANDBY/HIBERNATE
    pub state_disk: regulator_state,
    pub state_mem: regulator_state,
    pub state_standby: regulator_state,
    pub over_curr_limits: notification_limit,
    pub over_voltage_limits: notification_limit,
    pub under_voltage_limits: notification_limit,
    pub temp_limits: notification_limit,
    pub /: *mut *mut suspend_state_t initial_state; / suspend state to set at init,
// mode to set on startup
    pub initial_mode: c_uint,
    pub ramp_delay: c_uint,
    pub settling_time: c_uint,
    pub settling_time_up: c_uint,
    pub settling_time_down: c_uint,
    pub enable_time: c_uint,
    pub uv_less_critical_window_ms: c_uint,
    pub active_discharge: c_uint,
// constraint flags
    pub /: *mut *mut unsigned always_on:1; / regulator never off when system is on,
    pub /: *mut *mut unsigned boot_on:1; / bootloader/firmware enabled regulator,
    pub /: *mut *mut unsigned apply_uV:1; / apply uV constraint if min == max,
    pub /: *mut *mut unsigned ramp_disable:1; / disable ramp delay,
    pub /: *mut *mut unsigned soft_start:1; / ramp voltage slowly,
    pub /: *mut *mut unsigned pull_down:1; / pull down resistor when regulator off,
    pub /: *mut *mut unsigned system_critical:1; / critical to system stability,
    pub /: *mut *mut unsigned over_current_protection:1; / auto disable on over current,
    pub /: *mut *mut unsigned over_current_detection:1; / notify on over current,
    pub /: *mut *mut unsigned over_voltage_detection:1; / notify on over voltage,
    pub /: *mut *mut unsigned under_voltage_detection:1; / notify on under voltage,
    pub /: *mut *mut unsigned over_temp_detection:1; / notify on over temperature,
}

//
// struct regulator_consumer_supply - supply -> device mapping
//
// This maps a supply name to a device. Use of dev_name allows support for
// buses which make struct device available late such as I2C.
//
// @dev_name: Result of dev_name() for the consumer.
// @supply: Name for the supply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_consumer_supply {
    pub /: *const *const *const char dev_name; / dev_name() for consumer,
    pub /: *const *const *const char supply; / consumer supply - e.g. "vcc",
}

// Initialize struct regulator_consumer_supply

//
// struct regulator_init_data - regulator platform initialisation data.
//
// Initialisation constraints, our supply and consumers supplies.
//
// @supply_regulator: Parent regulator.  Specified using the regulator name
// as it appears in the name field in sysfs, which can
// be explicitly set using the constraints field 'name'.
//
// @constraints: Constraints.  These must be specified for the regulator to
// be usable.
// @num_consumer_supplies: Number of consumer device supplies.
// @consumer_supplies: Consumer device supply configuration.
// @driver_data: Data passed to regulator_init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_init_data {
    pub /: *const *const *const char supply_regulator; / or NULL for system supply,
    pub constraints: regulation_constraints,
    pub num_consumer_supplies: c_int,
    pub consumer_supplies: *mut regulator_consumer_supply,
// optional regulator machine specific data
    pub /: *mut *mut *mut void driver_data; / core does not touch this,
}

extern "C" {
    pub fn regulator_has_full_constraints();
}

