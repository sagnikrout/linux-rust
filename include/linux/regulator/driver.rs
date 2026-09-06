//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/driver.h
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
// driver.h -- SoC Regulator driver support.
//
// Copyright (C) 2007, 2008 Wolfson Microelectronics PLC.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//
// Regulator Driver Interface.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulator_status {
    REGULATOR_STATUS_OFF,
    REGULATOR_STATUS_ON,
    REGULATOR_STATUS_ERROR,
// fast/normal/idle/standby are flavors of "on"
    REGULATOR_STATUS_FAST,
    REGULATOR_STATUS_NORMAL,
    REGULATOR_STATUS_IDLE,
    REGULATOR_STATUS_STANDBY,
// The regulator is enabled but not regulating
    REGULATOR_STATUS_BYPASS,
// in case that any other status doesn't apply
    REGULATOR_STATUS_UNDEFINED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulator_detection_severity {
// Hardware shut down voltage outputs if condition is detected
    REGULATOR_SEVERITY_PROT,
// Hardware is probably damaged/inoperable
    REGULATOR_SEVERITY_ERR,
// Hardware is still recoverable but recovery action must be taken
    REGULATOR_SEVERITY_WARN,
}

// Initialize struct linear_range for regulators

// Initialize struct linear_range using voltages, not selectors

//
// struct regulator_ops - regulator operations.
//
// @enable: Configure the regulator as enabled.
// @disable: Configure the regulator as disabled.
// @is_enabled: Return 1 if the regulator is enabled, 0 if not.
// May also return negative errno.
//
// @set_voltage: Set the voltage for the regulator within the range specified.
// The driver should select the voltage closest to min_uV.
// @set_voltage_sel: Set the voltage for the regulator using the specified
// selector.
// @map_voltage: Convert a voltage into a selector
// @get_voltage: Return the currently configured voltage for the regulator;
// return -ENOTRECOVERABLE if regulator can't be read at
// bootup and hasn't been set yet.
// @get_voltage_sel: Return the currently configured voltage selector for the
// regulator; return -ENOTRECOVERABLE if regulator can't
// be read at bootup and hasn't been set yet.
// @list_voltage: Return one of the supported voltages, in microvolts; zero
// if the selector indicates a voltage that is unusable on this system;
// or negative errno.  Selectors range from zero to one less than
// regulator_desc.n_voltages.  Voltages may be reported in any order.
//
// @set_current_limit: Configure a limit for a current-limited regulator.
// The driver should select the current closest to max_uA.
// @get_current_limit: Get the configured limit for a current-limited regulator.
// @set_input_current_limit: Configure an input limit.
//
// @set_over_current_protection: Support enabling of and setting limits for over
// current situation detection. Detection can be configured for three
// levels of severity.
//
// - REGULATOR_SEVERITY_PROT should automatically shut down the regulator(s).
//
// - REGULATOR_SEVERITY_ERR should indicate that over-current situation is
// caused by an unrecoverable error but HW does not perform
// automatic shut down.
//
// - REGULATOR_SEVERITY_WARN should indicate situation where hardware is
// still believed to not be damaged but that a board sepcific
// recovery action is needed. If lim_uA is 0 the limit should not
// be changed but the detection should just be enabled/disabled as
// is requested.
//
// @set_over_voltage_protection: Support enabling of and setting limits for over
// voltage situation detection. Detection can be configured for same
// severities as over current protection. Units of uV.
// @set_under_voltage_protection: Support enabling of and setting limits for
// under voltage situation detection. Detection can be configured for same
// severities as over current protection. Units of uV.
// @set_thermal_protection: Support enabling of and setting limits for over
// temperature situation detection.Detection can be configured for same
// severities as over current protection. Units of degree Kelvin.
//
// @set_active_discharge: Set active discharge enable/disable of regulators.
//
// @set_mode: Set the configured operating mode for the regulator.
// @get_mode: Get the configured operating mode for the regulator.
// @get_error_flags: Get the current error(s) for the regulator.
// @get_status: Return actual (not as-configured) status of regulator, as a
// REGULATOR_STATUS value (or negative errno)
// @get_optimum_mode: Get the most efficient operating mode for the regulator
// when running with the specified parameters.
// @set_load: Set the load for the regulator.
//
// @set_bypass: Set the regulator in bypass mode.
// @get_bypass: Get the regulator bypass mode state.
//
// @enable_time: Time taken for the regulator voltage output voltage to
// stabilise after being enabled, in microseconds.
// @set_ramp_delay: Set the ramp delay for the regulator. The driver should
// select ramp delay equal to or less than(closest) ramp_delay.
// @set_voltage_time: Time taken for the regulator voltage output voltage
// to stabilise after being set to a new value, in microseconds.
// The function receives the from and to voltage as input, it
// should return the worst case.
// @set_voltage_time_sel: Time taken for the regulator voltage output voltage
// to stabilise after being set to a new value, in microseconds.
// The function receives the from and to voltage selector as
// input, it should return the worst case.
// @set_soft_start: Enable soft start for the regulator.
//
// @set_suspend_voltage: Set the voltage for the regulator when the system
// is suspended.
// @set_suspend_enable: Mark the regulator as enabled when the system is
// suspended.
// @set_suspend_disable: Mark the regulator as disabled when the system is
// suspended.
// @set_suspend_mode: Set the operating mode for the regulator when the
// system is suspended.
// @resume: Resume operation of suspended regulator.
// @set_pull_down: Configure the regulator to pull down when the regulator
// is disabled.
//
// This struct describes regulator operations which can be implemented by
// regulator chip drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_ops {
// enumerate supported voltages
    pub selector): *mut *mut *mut int (list_voltage) (struct regulator_dev , unsigned,
// get/set regulator voltage
    pub selector): *mut unsigned,
    pub max_uV): *mut *mut *mut int (map_voltage)(struct regulator_dev , int min_uV, int,
    pub selector): *mut *mut *mut int (set_voltage_sel) (struct regulator_dev , unsigned,
    pub ): *mut *mut int (get_voltage) (struct regulator_dev,
    pub ): *mut *mut int (get_voltage_sel) (struct regulator_dev,
// get/set regulator current
    pub max_uA): int min_uA, int,
    pub ): *mut *mut int (get_current_limit) (struct regulator_dev,
    pub lim_uA): *mut *mut *mut int (set_input_current_limit) (struct regulator_dev , int,
    pub enable): int severity, bool,
    pub enable): int severity, bool,
    pub enable): int severity, bool,
    pub enable): int severity, bool,
    pub enable): *mut *mut *mut int (set_active_discharge)(struct regulator_dev , bool,
// enable/disable regulator
    pub ): *mut *mut int (enable) (struct regulator_dev,
    pub ): *mut *mut int (disable) (struct regulator_dev,
    pub ): *mut *mut int (is_enabled) (struct regulator_dev,
// get/set regulator operating mode (defined in consumer.h)
    pub mode): *mut *mut *mut int (set_mode) (struct regulator_dev , unsigned int,
    pub ): *mut *mut unsigned int (get_mode) (struct regulator_dev,
// retrieve current error flags on the regulator
    pub flags): *mut *mut *mut int (get_error_flags)(struct regulator_dev , unsigned int,
// Time taken to enable or set voltage on the regulator
    pub ): *mut *mut int (enable_time) (struct regulator_dev,
    pub ramp_delay): *mut *mut *mut int (set_ramp_delay) (struct regulator_dev , int,
    pub new_uV): c_int,
    pub new_selector): c_uint,
    pub ): *mut *mut int (set_soft_start) (struct regulator_dev,
// report regulator status ... most other accessors report
// control inputs, this reports results of combining inputs
// from Linux (and other sources) with the actual load.
// returns REGULATOR_STATUS_* or negative errno.
//
    pub ): *mut *mut int (get_status)(struct regulator_dev,
// get most efficient regulator operating mode for load
    pub load_uA): int output_uV, int,
// set the load on the regulator
    pub load_uA): *mut *mut *mut int (set_load)(struct regulator_dev , int,
// control and report on bypass mode
    pub enable): *mut *mut *mut int (set_bypass)(struct regulator_dev dev, bool,
    pub enable): *mut *mut *mut int (get_bypass)(struct regulator_dev dev, bool,
// the operations below are for configuration of regulator state when
// its parent PMIC enters a global STANDBY/HIBERNATE state
// set regulator suspend voltage
    pub uV): *mut *mut *mut int (set_suspend_voltage) (struct regulator_dev , int,
// enable/disable regulator in suspend state
    pub ): *mut *mut int (set_suspend_enable) (struct regulator_dev,
    pub ): *mut *mut int (set_suspend_disable) (struct regulator_dev,
// set regulator suspend operating mode (defined in consumer.h)
    pub mode): *mut *mut *mut int (set_suspend_mode) (struct regulator_dev , unsigned int,
    pub rdev): *mut *mut int (resume)(struct regulator_dev,
    pub ): *mut *mut int (set_pull_down) (struct regulator_dev,
}

//
// Regulators can either control voltage or current.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulator_type {
    REGULATOR_VOLTAGE,
    REGULATOR_CURRENT,
}

//
// struct regulator_desc - Static regulator descriptor
//
// Each regulator registered with the core is described with a
// structure of this type and a struct regulator_config.  This
// structure contains the non-varying parts of the regulator
// description.
//
// @name: Identifying name for the regulator.
// @supply_name: Identifying the regulator supply
// @of_match: Name used to identify regulator in DT.
// @of_match_full_name: A flag to indicate that the of_match string, if
// present, should be matched against the node full_name.
// @regulators_node: Name of node containing regulator definitions in DT.
// @of_parse_cb: Optional callback called only if of_match is present.
// Will be called for each regulator parsed from DT, during
// init_data parsing.
// The regulator_config passed as argument to the callback will
// be a copy of config passed to regulator_register, valid only
// for this particular call. Callback may freely change the
// config but it cannot store it for later usage.
// Callback should return 0 on success or negative ERRNO
// indicating failure.
// @init_cb: Optional callback called after the parsing of init_data.
// Allows the regulator to perform runtime init if necessary,
// such as synching the regulator and the parsed constraints.
// Callback should return 0 on success or negative ERRNO
// indicating failure.
// @id: Numerical identifier for the regulator.
// @ops: Regulator operations table.
// @irq: Interrupt number for the regulator.
// @type: Indicates if the regulator is a voltage or current regulator.
// @owner: Module providing the regulator, used for refcounting.
//
// @continuous_voltage_range: Indicates if the regulator can set any
// voltage within constrains range.
// @n_voltages: Number of selectors available for ops.list_voltage().
// @n_current_limits: Number of selectors available for current limits
//
// @min_uV: Voltage given by the lowest selector (if linear mapping)
// @uV_step: Voltage increase with each selector (if linear mapping)
// @linear_min_sel: Minimal selector for starting linear mapping
// @fixed_uV: Fixed voltage of rails.
// @ramp_delay: Time to settle down after voltage change (unit: uV/us)
// @min_dropout_uV: The minimum dropout voltage this regulator can handle
// @linear_ranges: A constant table of possible voltage ranges.
// @linear_range_selectors_bitfield: A constant table of voltage range
// selectors as bitfield values. If
// pickable ranges are used each range
// must have corresponding selector here.
// @n_linear_ranges: Number of entries in the @linear_ranges (and in
// linear_range_selectors_bitfield if used) table(s).
// @volt_table: Voltage mapping table (if table based mapping)
// @curr_table: Current limit mapping table (if table based mapping)
//
// @vsel_range_reg: Register for range selector when using pickable ranges
// and ``regulator_map_*_voltage_*_pickable`` functions.
// @vsel_range_mask: Mask for register bitfield used for range selector
// @range_applied_by_vsel: A flag to indicate that changes to vsel_range_reg
// are only effective after vsel_reg is written
// @vsel_reg: Register for selector when using ``regulator_map_*_voltage_*``
// @vsel_mask: Mask for register bitfield used for selector
// @vsel_step: Specify the resolution of selector stepping when setting
// voltage. If 0, then no stepping is done (requested selector is
// set directly), if >0 then the regulator API will ramp the
// voltage up/down gradually each time increasing/decreasing the
// selector by the specified step value.
// @csel_reg: Register for current limit selector using regmap set_current_limit
// @csel_mask: Mask for register bitfield used for current limit selector
// @apply_reg: Register for initiate voltage change on the output when
// using regulator_set_voltage_sel_regmap
// @apply_bit: Register bitfield used for initiate voltage change on the
// output when using regulator_set_voltage_sel_regmap
// @enable_reg: Register for control when using regmap enable/disable ops
// @enable_mask: Mask for control when using regmap enable/disable ops
// @enable_val: Enabling value for control when using regmap enable/disable ops
// @disable_val: Disabling value for control when using regmap enable/disable ops
// @enable_is_inverted: A flag to indicate set enable_mask bits to disable
// when using regulator_enable_regmap and friends APIs.
// @bypass_reg: Register for control when using regmap set_bypass
// @bypass_mask: Mask for control when using regmap set_bypass
// @bypass_val_on: Enabling value for control when using regmap set_bypass
// @bypass_val_off: Disabling value for control when using regmap set_bypass
// @active_discharge_off: Enabling value for control when using regmap
// set_active_discharge
// @active_discharge_on: Disabling value for control when using regmap
// set_active_discharge
// @active_discharge_mask: Mask for control when using regmap
// set_active_discharge
// @active_discharge_reg: Register for control when using regmap
// set_active_discharge
// @soft_start_reg: Register for control when using regmap set_soft_start
// @soft_start_mask: Mask for control when using regmap set_soft_start
// @soft_start_val_on: Enabling value for control when using regmap
// set_soft_start
// @pull_down_reg: Register for control when using regmap set_pull_down
// @pull_down_mask: Mask for control when using regmap set_pull_down
// @pull_down_val_on: Enabling value for control when using regmap
// set_pull_down
//
// @ramp_reg:		Register for controlling the regulator ramp-rate.
// @ramp_mask:		Bitmask for the ramp-rate control register.
// @ramp_delay_table:	Table for mapping the regulator ramp-rate values. Values
// should be given in units of V/S (uV/uS). See the
// regulator_set_ramp_delay_regmap().
// @n_ramp_values:	number of elements at @ramp_delay_table.
//
// @enable_time: Time taken for initial enable of regulator (in uS).
// @off_on_delay: guard time (in uS), before re-enabling a regulator
//
// @poll_enabled_time: The polling interval (in uS) to use while checking that
// the regulator was actually enabled. Max upto enable_time.
//
// @of_map_mode: Maps a hardware mode defined in a DeviceTree to a standard mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_desc {
    pub name: *const c_char,
    pub supply_name: *const c_char,
    pub of_match: *const c_char,
    pub of_match_full_name: bool,
    pub regulators_node: *const c_char,
    pub ): *mut regulator_config,
    pub ): *mut regulator_config,
    pub id: c_int,
    pub continuous_voltage_range:1: c_uint,
    pub n_voltages: unsigned,
    pub n_current_limits: c_uint,
    pub ops: *const regulator_ops,
    pub irq: c_int,
    pub type: regulator_type,
    pub owner: *mut module,
    pub min_uV: c_uint,
    pub uV_step: c_uint,
    pub linear_min_sel: c_uint,
    pub fixed_uV: c_int,
    pub ramp_delay: c_uint,
    pub min_dropout_uV: c_int,
    pub linear_ranges: *const linear_range,
    pub linear_range_selectors_bitfield: *const c_uint,
    pub n_linear_ranges: c_int,
    pub volt_table: *const c_uint,
    pub curr_table: *const c_uint,
    pub vsel_range_reg: c_uint,
    pub vsel_range_mask: c_uint,
    pub range_applied_by_vsel: bool,
    pub vsel_reg: c_uint,
    pub vsel_mask: c_uint,
    pub vsel_step: c_uint,
    pub csel_reg: c_uint,
    pub csel_mask: c_uint,
    pub apply_reg: c_uint,
    pub apply_bit: c_uint,
    pub enable_reg: c_uint,
    pub enable_mask: c_uint,
    pub enable_val: c_uint,
    pub disable_val: c_uint,
    pub enable_is_inverted: bool,
    pub bypass_reg: c_uint,
    pub bypass_mask: c_uint,
    pub bypass_val_on: c_uint,
    pub bypass_val_off: c_uint,
    pub active_discharge_on: c_uint,
    pub active_discharge_off: c_uint,
    pub active_discharge_mask: c_uint,
    pub active_discharge_reg: c_uint,
    pub soft_start_reg: c_uint,
    pub soft_start_mask: c_uint,
    pub soft_start_val_on: c_uint,
    pub pull_down_reg: c_uint,
    pub pull_down_mask: c_uint,
    pub pull_down_val_on: c_uint,
    pub ramp_reg: c_uint,
    pub ramp_mask: c_uint,
    pub ramp_delay_table: *const c_uint,
    pub n_ramp_values: c_uint,
    pub enable_time: c_uint,
    pub off_on_delay: c_uint,
    pub poll_enabled_time: c_uint,
    pub mode): *mut *mut unsigned int (of_map_mode)(unsigned int,
}

//
// struct regulator_config - Dynamic regulator descriptor
//
// Each regulator registered with the core is described with a
// structure of this type and a struct regulator_desc.  This structure
// contains the runtime variable parts of the regulator description.
//
// @dev: struct device for the regulator
// @init_data: platform provided init data, passed through by driver
// @driver_data: private regulator data
// @of_node: OpenFirmware node to parse for device tree bindings (may be
// NULL).
// @regmap: regmap to use for core regmap helpers if dev_get_regmap() is
// insufficient.
// @ena_gpiod: GPIO controlling regulator enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_config {
    pub dev: *mut device,
    pub init_data: *const regulator_init_data,
    pub driver_data: *mut c_void,
    pub of_node: *mut device_node,
    pub regmap: *mut regmap,
    pub ena_gpiod: *mut gpio_desc,
}

//
// struct regulator_err_state - regulator error/notification status
//
// @rdev:		Regulator which status the struct indicates.
// @notifs:		Events which have occurred on the regulator.
// @errors:		Errors which are active on the regulator.
// @possible_errs:	Errors which can be signaled (by given IRQ).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_err_state {
    pub rdev: *mut regulator_dev,
    pub notifs: c_ulong,
    pub errors: c_ulong,
    pub possible_errs: c_int,
}

//
// struct regulator_irq_data - regulator error/notification status data
//
// @states:	Status structs for each of the associated regulators.
// @num_states:	Amount of associated regulators.
// @data:	Driver data pointer given at regulator_irq_desc.
// @opaque:	Value storage for IC driver. Core does not update this. ICs
// may want to store status register value here at map_event and
// compare contents at 'renable' callback to see if new problems
// have been added to status. If that is the case it may be
// desirable to return REGULATOR_ERROR_CLEARED and not
// REGULATOR_ERROR_ON to allow IRQ fire again and to generate
// notifications also for the new issues.
//
// This structure is passed to 'map_event' and 'renable' callbacks for
// reporting regulator status to core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_irq_data {
    pub states: *mut regulator_err_state,
    pub num_states: c_int,
    pub data: *mut c_void,
    pub opaque: c_long,
}

//
// struct regulator_irq_desc - notification sender for IRQ based events.
//
// @name:	The visible name for the IRQ
// @fatal_cnt:	If this IRQ is used to signal HW damaging condition it may be
// best to shut-down regulator(s) or reboot the SOC if error
// handling is repeatedly failing. If fatal_cnt is given the IRQ
// handling is aborted if it fails for fatal_cnt times and die()
// callback (if populated) is called. If die() is not populated
// poweroff for the system is attempted in order to prevent any
// further damage.
// @reread_ms:	The time which is waited before attempting to re-read status
// at the worker if IC reading fails. Immediate re-read is done
// if time is not specified.
// @irq_off_ms:	The time which IRQ is kept disabled before re-evaluating the
// status for devices which keep IRQ disabled for duration of the
// error. If this is not given the IRQ is left enabled and renable
// is not called.
// @skip_off:	If set to true the IRQ handler will attempt to check if any of
// the associated regulators are enabled prior to taking other
// actions. If no regulators are enabled and this is set to true
// a spurious IRQ is assumed and IRQ_NONE is returned.
// @high_prio:	Boolean to indicate that high priority WQ should be used.
// @data:	Driver private data pointer which will be passed as such to
// the renable, map_event and die callbacks in regulator_irq_data.
// @die:	Protection callback. If IC status reading or recovery actions
// fail fatal_cnt times this callback is called or system is
// powered off. This callback should implement a final protection
// attempt like disabling the regulator. If protection succeeded
// die() may return 0. If anything else is returned the core
// assumes final protection failed and attempts to perform a
// poweroff as a last resort.
// @map_event:	Driver callback to map IRQ status into regulator devices with
// events / errors. NOTE: callback MUST initialize both the
// errors and notifs for all rdevs which it signals having
// active events as core does not clean the map data.
// REGULATOR_FAILED_RETRY can be returned to indicate that the
// status reading from IC failed. If this is repeated for
// fatal_cnt times the core will call die() callback or power-off
// the system as a last resort to protect the HW.
// @renable:	Optional callback to check status (if HW supports that) before
// re-enabling IRQ. If implemented this should clear the error
// flags so that errors fetched by regulator_get_error_flags()
// are updated. If callback is not implemented then errors are
// assumed to be cleared and IRQ is re-enabled.
// REGULATOR_FAILED_RETRY can be returned to
// indicate that the status reading from IC failed. If this is
// repeated for 'fatal_cnt' times the core will call die()
// callback or if die() is not populated then attempt to power-off
// the system as a last resort to protect the HW.
// Returning zero indicates that the problem in HW has been solved
// and IRQ will be re-enabled. Returning REGULATOR_ERROR_ON
// indicates the error condition is still active and keeps IRQ
// disabled. Please note that returning REGULATOR_ERROR_ON does
// not retrigger evaluating what events are active or resending
// notifications. If this is needed you probably want to return
// zero and allow IRQ to retrigger causing events to be
// re-evaluated and re-sent.
//
// This structure is used for registering regulator IRQ notification helper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_irq_desc {
    pub name: *const c_char,
    pub fatal_cnt: c_int,
    pub reread_ms: c_int,
    pub irq_off_ms: c_int,
    pub skip_off: bool,
    pub high_prio: bool,
    pub data: *mut c_void,
    pub rid): *mut *mut int (die)(struct regulator_irq_data,
    pub dev_mask): *mut c_ulong,
    pub rid): *mut *mut int (renable)(struct regulator_irq_data,
}

//
// Return values for regulator IRQ helpers.
//
// struct coupling_desc
//
// Describes coupling of regulators. Each regulator should have
// at least a pointer to itself in coupled_rdevs array.
// When a new coupled regulator is resolved, n_resolved is
// incremented.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coupling_desc {
    pub coupled_rdevs: *mut regulator_dev,
    pub coupler: *mut regulator_coupler,
    pub n_resolved: c_int,
    pub n_coupled: c_int,
}

//
// struct regulator_dev
//
// Voltage / Current regulator class device. One for each
// regulator.
//
// This should *not* be used directly by anything except the regulator
// core and notification injection (which should take the mutex and do
// no other direct access).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_dev {
    pub desc: *const regulator_desc,
    pub exclusive: c_int,
    pub use_count: u32,
    pub open_count: u32,
    pub bypass_count: u32,
// lists we belong to
    pub /: *mut *mut list_head list; / list of all regulators,
// lists we own
    pub /: *mut *mut list_head consumer_list; / consumers we supply,
    pub coupling_desc: coupling_desc,
    pub notifier: blocking_notifier_head,
    pub /: *mut *mut ww_mutex mutex; / consumer lock,
    pub mutex_owner: *mut task_struct,
    pub ref_cnt: c_int,
    pub owner: *mut module,
    pub dev: device,
    pub bdev: device,
    pub constraints: *mut regulation_constraints,
    pub /: *mut *mut *mut regulator supply; / for tree,
    pub supply_name: *const c_char,
    pub regmap: *mut regmap,
    pub disable_work: delayed_work,
    pub /: *mut *mut *mut void reg_data; / regulator_dev data,
    pub debugfs: *mut dentry,
    pub ena_pin: *mut regulator_enable_gpio,
    pub ena_gpio_state:1: c_uint,
    pub constraints_pending:1: c_uint,
    pub is_switch:1: c_uint,
// time when this regulator was disabled last time
    pub last_off: ktime_t,
    pub cached_err: c_int,
    pub use_cached_err: bool,
    pub err_lock: spinlock_t,
    pub pw_requested_mW: c_int,
// regulator notification forwarding
    pub supply_fwd_nb: notifier_block,
}

//
// Convert error flags to corresponding notifications.
//
// Can be used by drivers which use the notification helpers to
// find out correct notification flags based on the error flags. Drivers
// can avoid storing both supported notification and error flags which
// may save few bytes.
//
extern "C" {
    pub fn regulator_unregister(rdev: *mut regulator_dev);
}
extern "C" {
    pub fn regulator_irq_helper_cancel(handle: *mut c_void);
}
extern "C" {
    pub fn rdev_get_id(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_mode_to_status(int: unsigned) -> c_int;
}
extern "C" {
    pub fn regulator_get_voltage_sel_pickable_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_get_voltage_sel_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_set_voltage_sel_regmap(rdev: *mut regulator_dev, sel: unsigned) -> c_int;
}
extern "C" {
    pub fn regulator_is_enabled_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_enable_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_disable_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_set_bypass_regmap(rdev: *mut regulator_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn regulator_get_bypass_regmap(rdev: *mut regulator_dev, enable: *mut bool) -> c_int;
}
extern "C" {
    pub fn regulator_set_soft_start_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_set_pull_down_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_get_current_limit_regmap(rdev: *mut regulator_dev) -> c_int;
}
extern "C" {
    pub fn regulator_set_ramp_delay_regmap(rdev: *mut regulator_dev, ramp_delay: c_int) -> c_int;
}
extern "C" {
    pub fn regulator_sync_voltage_rdev(rdev: *mut regulator_dev) -> c_int;
}
//
// Helper functions intended to be used by regulator drivers prior registering
// their regulators.
//

