//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/scmi_protocol.h
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
// SCMI Message Protocol driver header
//
// Copyright (C) 2018-2021 ARM Ltd.
//

pub const SCMI_MAX_STR_SIZE: c_int = 64;
pub const SCMI_SHORT_NAME_MAX_SIZE: c_int = 16;
//
// struct scmi_base_info - version information structure
//
// @major_ver: Major ABI version. Change here implies risk of backward
// compatibility break.
// @minor_ver: Minor ABI version. Change here implies new feature addition,
// or compatible change in ABI.
// @num_protocols: Number of protocols that are implemented, excluding the
// base protocol.
// @num_agents: Number of agents in the system.
// @impl_ver: A vendor-specific implementation version.
// @vendor_id: A vendor identifier(Null terminated ASCII string)
// @sub_vendor_id: A sub-vendor identifier(Null terminated ASCII string)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_base_info {
    pub major_ver: u16,
    pub minor_ver: u16,
    pub num_protocols: u8,
    pub num_agents: u8,
    pub impl_ver: u32,
    pub vendor_id: [c_char; SCMI_SHORT_NAME_MAX_SIZE],
    pub sub_vendor_id: [c_char; SCMI_SHORT_NAME_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_rates {
    pub rate_discrete: bool,
    pub num_rates: c_uint,
    pub rates: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_info {
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub enable_latency: c_uint,
    pub rate_changed_notifications: bool,
    pub rate_change_requested_notifications: bool,
    pub state_ctrl_forbidden: bool,
    pub rate_ctrl_forbidden: bool,
    pub parent_ctrl_forbidden: bool,
    pub extended_config: bool,
    pub min_rate: u64,
    pub max_rate: u64,
    pub num_parents: c_int,
    pub parents: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_power_scale {
    SCMI_POWER_BOGOWATTS,
    SCMI_POWER_MILLIWATTS,
    SCMI_POWER_MICROWATTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_clock_oem_config {
    SCMI_CLOCK_CFG_DUTY_CYCLE = 0x1,
    SCMI_CLOCK_CFG_PHASE,
    SCMI_CLOCK_CFG_OEM_START = 0x80,
    SCMI_CLOCK_CFG_OEM_END = 0xFF,
}

//
// struct scmi_clk_proto_ops - represents the various operations provided
// by SCMI Clock Protocol
//
// @count_get: get the count of clocks provided by SCMI
// @info_get: get the information of the specified clock
// @rate_get: request the current clock rate of a clock
// @rate_set: set the clock rate of a clock
// @determine_rate: determine the effective rate that can be supported by a
// clock calculating the closest allowed rate.
// Note that @rate is an input/output parameter used both to
// describe the requested rate and report the closest match
// @all_rates_get: get the list of all available rates for the specified clock.
// @enable: enables the specified clock
// @disable: disables the specified clock
// @state_get: get the status of the specified clock
// @config_oem_get: get the value of an OEM specific clock config
// @config_oem_set: set the value of an OEM specific clock config
// @parent_get: get the parent id of a clk
// @parent_set: set the parent of a clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clk_proto_ops {
    pub ph): *const *const int (count_get)(struct scmi_protocol_handle,
    pub clk_id): *const *const (struct scmi_protocol_handle ph, u32,
    pub rate): *mut u64,
    pub rate): u64,
    pub rate): *mut c_ulong,
    pub clk_id): *const *const (struct scmi_protocol_handle ph, u32,
    pub atomic): bool,
    pub atomic): bool,
    pub atomic): *mut *mut bool enabled, bool,
    pub atomic): *mut *mut *mut u32 oem_val, u32 attributes, bool,
    pub atomic): u32 oem_val, bool,
    pub parent_id): *const *const *const int (parent_get)(struct scmi_protocol_handle ph, u32 clk_id, u32,
    pub parent_id): *const *const *const int (parent_set)(struct scmi_protocol_handle ph, u32 clk_id, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_perf_domain_info {
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub set_perf: bool,
}

//
// struct scmi_perf_proto_ops - represents the various operations provided
// by SCMI Performance Protocol
//
// @num_domains_get: gets the number of supported performance domains
// @info_get: get the information of a performance domain
// @limits_set: sets limits on the performance level of a domain
// @limits_get: gets limits on the performance level of a domain
// @level_set: sets the performance level of a domain
// @level_get: gets the performance level of a domain
// @transition_latency_get: gets the DVFS transition latency for a given device
// @rate_limit_get: gets the minimum time (us) required between successive
// requests
// @device_opps_add: adds all the OPPs for a given device
// @freq_set: sets the frequency for a given device using sustained frequency
// to sustained performance level mapping
// @freq_get: gets the frequency for a given device using sustained frequency
// to sustained performance level mapping
// @est_power_get: gets the estimated power cost for a given performance domain
// at a given frequency
// @fast_switch_possible: indicates if fast DVFS switching is possible or not
// for a given device
// @fast_switch_rate_limit: gets the minimum time (us) required between
// successive fast_switching requests
// @power_scale_get: indicates if the power values provided are in milliWatts
// or in some other (abstract) scale
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_perf_proto_ops {
    pub ph): *const *const int (num_domains_get)(struct scmi_protocol_handle,
    pub domain): *const *const (struct scmi_protocol_handle ph, u32,
    pub min_perf): u32 max_perf, u32,
    pub min_perf): *mut *mut u32 max_perf, u32,
    pub poll): u32 level, bool,
    pub poll): *mut *mut u32 level, bool,
    pub domain): u32,
    pub rate_limit): *mut u32 domain, u32,
    pub domain): *mut *mut device dev, u32,
    pub poll): unsigned long rate, bool,
    pub poll): *mut *mut unsigned long rate, bool,
    pub power): *mut *mut unsigned long rate, unsigned long,
    pub domain): u32,
    pub rate_limit): *mut u32 domain, u32,
    pub ph): *const *const scmi_power_scale (power_scale_get)(struct scmi_protocol_handle,
}

//
// struct scmi_power_proto_ops - represents the various operations provided
// by SCMI Power Protocol
//
// @num_domains_get: get the count of power domains provided by SCMI
// @name_get: gets the name of a power domain
// @state_set: sets the power state of a power domain
// @state_get: gets the power state of a power domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_proto_ops {
    pub ph): *const *const int (num_domains_get)(struct scmi_protocol_handle,
    pub domain): u32,
pub const SCMI_POWER_STATE_TYPE_SHIFT: c_int = 30;

    pub state): u32,
    pub state): *mut u32,
}

//
// struct scmi_sensor_reading  - represent a timestamped read
//
// Used by @reading_get_timestamped method.
//
// @value: The signed value sensor read.
// @timestamp: An unsigned timestamp for the sensor read, as provided by
// SCMI platform. Set to zero when not available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_reading {
    pub value: c_longlong,
    pub timestamp: c_ulonglong,
}

//
// struct scmi_range_attrs  - specifies a sensor or axis values' range
// @min_range: The minimum value which can be represented by the sensor/axis.
// @max_range: The maximum value which can be represented by the sensor/axis.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_range_attrs {
    pub min_range: c_longlong,
    pub max_range: c_longlong,
}

//
// struct scmi_sensor_axis_info  - describes one sensor axes
// @id: The axes ID.
// @type: Axes type. Chosen amongst one of @enum scmi_sensor_class.
// @scale: Power-of-10 multiplier applied to the axis unit.
// @name: NULL-terminated string representing axes name as advertised by
// SCMI platform.
// @extended_attrs: Flag to indicate the presence of additional extended
// attributes for this axes.
// @resolution: Extended attribute representing the resolution of the axes.
// Set to 0 if not reported by this axes.
// @exponent: Extended attribute representing the power-of-10 multiplier that
// is applied to the resolution field. Set to 0 if not reported by
// this axes.
// @attrs: Extended attributes representing minimum and maximum values
// measurable by this axes. Set to 0 if not reported by this sensor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_axis_info {
    pub id: c_uint,
    pub type: c_uint,
    pub scale: c_int,
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub extended_attrs: bool,
    pub resolution: c_uint,
    pub exponent: c_int,
    pub attrs: scmi_range_attrs,
}

//
// struct scmi_sensor_intervals_info  - describes number and type of available
// update intervals
// @segmented: Flag for segmented intervals' representation. When True there
// will be exactly 3 intervals in @desc, with each entry
// representing a member of a segment in this order:
// {lowest update interval, highest update interval, step size}
// @count: Number of intervals described in @desc.
// @desc: Array of @count interval descriptor bitmask represented as detailed in
// the SCMI specification: it can be accessed using the accompanying
// macros.
// @prealloc_pool: A minimal preallocated pool of desc entries used to avoid
// lesser-than-64-bytes dynamic allocation for small @count
// values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_intervals_info {
    pub segmented: bool,
    pub count: c_uint,
pub const SCMI_SENS_INTVL_SEGMENT_LOW: c_int = 0;
pub const SCMI_SENS_INTVL_SEGMENT_HIGH: c_int = 1;
pub const SCMI_SENS_INTVL_SEGMENT_STEP: c_int = 2;
    pub desc: *mut c_uint,

    pub \: int __signed_exp = FIELD_GET(GENMASK(4, 0), (x));,
    pub \: __signed_exp |= GENMASK(31, 5);,
    pub \: __signed_exp;,
pub const SCMI_MAX_PREALLOC_POOL: c_int = 16;
    pub prealloc_pool: [c_uint; SCMI_MAX_PREALLOC_POOL],
}

//
// struct scmi_sensor_info - represents information related to one of the
// available sensors.
// @id: Sensor ID.
// @type: Sensor type. Chosen amongst one of @enum scmi_sensor_class.
// @scale: Power-of-10 multiplier applied to the sensor unit.
// @num_trip_points: Number of maximum configurable trip points.
// @async: Flag for asynchronous read support.
// @update: Flag for continuouos update notification support.
// @timestamped: Flag for timestamped read support.
// @tstamp_scale: Power-of-10 multiplier applied to the sensor timestamps to
// represent it in seconds.
// @num_axis: Number of supported axis if any. Reported as 0 for scalar sensors.
// @axis: Pointer to an array of @num_axis descriptors.
// @intervals: Descriptor of available update intervals.
// @sensor_config: A bitmask reporting the current sensor configuration as
// detailed in the SCMI specification: it can accessed and
// modified through the accompanying macros.
// @name: NULL-terminated string representing sensor name as advertised by
// SCMI platform.
// @extended_scalar_attrs: Flag to indicate the presence of additional extended
// attributes for this sensor.
// @sensor_power: Extended attribute representing the average power
// consumed by the sensor in microwatts (uW) when it is active.
// Reported here only for scalar sensors.
// Set to 0 if not reported by this sensor.
// @resolution: Extended attribute representing the resolution of the sensor.
// Reported here only for scalar sensors.
// Set to 0 if not reported by this sensor.
// @exponent: Extended attribute representing the power-of-10 multiplier that is
// applied to the resolution field.
// Reported here only for scalar sensors.
// Set to 0 if not reported by this sensor.
// @scalar_attrs: Extended attributes representing minimum and maximum
// measurable values by this sensor.
// Reported here only for scalar sensors.
// Set to 0 if not reported by this sensor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_info {
    pub id: c_uint,
    pub type: c_uint,
    pub scale: c_int,
    pub num_trip_points: c_uint,
    pub async: bool,
    pub update: bool,
    pub timestamped: bool,
    pub tstamp_scale: c_int,
    pub num_axis: c_uint,
    pub axis: *mut scmi_sensor_axis_info,
    pub intervals: scmi_sensor_intervals_info,
    pub sensor_config: c_uint,

    pub \: FIELD_GET(SCMI_SENS_CFG_UPDATE_EXP_MASK, (x));,
    pub \: __signed_exp |= GENMASK(31, 5);,
    pub \: __signed_exp;,

pub const SCMI_SENS_CFG_ROUND_AUTO: c_int = 2;
pub const SCMI_SENS_CFG_ROUND_UP: c_int = 1;
pub const SCMI_SENS_CFG_ROUND_DOWN: c_int = 0;

pub const SCMI_SENS_CFG_TSTAMP_ENABLE: c_int = 1;
pub const SCMI_SENS_CFG_TSTAMP_DISABLE: c_int = 0;

pub const SCMI_SENS_CFG_SENSOR_ENABLE: c_int = 1;
pub const SCMI_SENS_CFG_SENSOR_DISABLE: c_int = 0;
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub extended_scalar_attrs: bool,
    pub sensor_power: c_uint,
    pub resolution: c_uint,
    pub exponent: c_int,
    pub scalar_attrs: scmi_range_attrs,
}

//
// Partial list from Distributed Management Task Force (DMTF) specification:
// DSP0249 (Platform Level Data Model specification)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_sensor_class {
    NONE = 0x0,
    UNSPEC = 0x1,
    TEMPERATURE_C = 0x2,
    TEMPERATURE_F = 0x3,
    TEMPERATURE_K = 0x4,
    VOLTAGE = 0x5,
    CURRENT = 0x6,
    POWER = 0x7,
    ENERGY = 0x8,
    CHARGE = 0x9,
    VOLTAMPERE = 0xA,
    NITS = 0xB,
    LUMENS = 0xC,
    LUX = 0xD,
    CANDELAS = 0xE,
    KPA = 0xF,
    PSI = 0x10,
    NEWTON = 0x11,
    CFM = 0x12,
    RPM = 0x13,
    HERTZ = 0x14,
    SECS = 0x15,
    MINS = 0x16,
    HOURS = 0x17,
    DAYS = 0x18,
    WEEKS = 0x19,
    MILS = 0x1A,
    INCHES = 0x1B,
    FEET = 0x1C,
    CUBIC_INCHES = 0x1D,
    CUBIC_FEET = 0x1E,
    METERS = 0x1F,
    CUBIC_CM = 0x20,
    CUBIC_METERS = 0x21,
    LITERS = 0x22,
    FLUID_OUNCES = 0x23,
    RADIANS = 0x24,
    STERADIANS = 0x25,
    REVOLUTIONS = 0x26,
    CYCLES = 0x27,
    GRAVITIES = 0x28,
    OUNCES = 0x29,
    POUNDS = 0x2A,
    FOOT_POUNDS = 0x2B,
    OUNCE_INCHES = 0x2C,
    GAUSS = 0x2D,
    GILBERTS = 0x2E,
    HENRIES = 0x2F,
    FARADS = 0x30,
    OHMS = 0x31,
    SIEMENS = 0x32,
    MOLES = 0x33,
    BECQUERELS = 0x34,
    PPM = 0x35,
    DECIBELS = 0x36,
    DBA = 0x37,
    DBC = 0x38,
    GRAYS = 0x39,
    SIEVERTS = 0x3A,
    COLOR_TEMP_K = 0x3B,
    BITS = 0x3C,
    BYTES = 0x3D,
    WORDS = 0x3E,
    DWORDS = 0x3F,
    QWORDS = 0x40,
    PERCENTAGE = 0x41,
    PASCALS = 0x42,
    COUNTS = 0x43,
    GRAMS = 0x44,
    NEWTON_METERS = 0x45,
    HITS = 0x46,
    MISSES = 0x47,
    RETRIES = 0x48,
    OVERRUNS = 0x49,
    UNDERRUNS = 0x4A,
    COLLISIONS = 0x4B,
    PACKETS = 0x4C,
    MESSAGES = 0x4D,
    CHARS = 0x4E,
    ERRORS = 0x4F,
    CORRECTED_ERRS = 0x50,
    UNCORRECTABLE_ERRS = 0x51,
    SQ_MILS = 0x52,
    SQ_INCHES = 0x53,
    SQ_FEET = 0x54,
    SQ_CM = 0x55,
    SQ_METERS = 0x56,
    RADIANS_SEC = 0x57,
    BPM = 0x58,
    METERS_SEC_SQUARED = 0x59,
    METERS_SEC = 0x5A,
    CUBIC_METERS_SEC = 0x5B,
    MM_MERCURY = 0x5C,
    RADIANS_SEC_SQUARED = 0x5D,
    OEM_UNIT = 0xFF
}

//
// struct scmi_sensor_proto_ops - represents the various operations provided
// by SCMI Sensor Protocol
//
// @count_get: get the count of sensors provided by SCMI
// @info_get: get the information of the specified sensor
// @trip_point_config: selects and configures a trip-point of interest
// @reading_get: gets the current value of the sensor
// @reading_get_timestamped: gets the current value and timestamp, when
// available, of the sensor. (as of v3.0 spec)
// Supports multi-axis sensors for sensors which
// supports it and if the @reading array size of
// @count entry equals the sensor num_axis
// @config_get: Get sensor current configuration
// @config_set: Set sensor current configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_proto_ops {
    pub ph): *const *const int (count_get)(struct scmi_protocol_handle,
    pub sensor_id): *const *const (struct scmi_protocol_handle ph, u32,
    pub trip_value): u32 sensor_id, u8 trip_id, u64,
    pub value): *mut u64,
    pub readings): *mut scmi_sensor_reading,
    pub sensor_config): *mut u32 sensor_id, u32,
    pub sensor_config): u32 sensor_id, u32,
}

//
// struct scmi_reset_proto_ops - represents the various operations provided
// by SCMI Reset Protocol
//
// @num_domains_get: get the count of reset domains provided by SCMI
// @name_get: gets the name of a reset domain
// @latency_get: gets the reset latency for the specified reset domain
// @reset: resets the specified reset domain
// @assert: explicitly assert reset signal of the specified reset domain
// @deassert: explicitly deassert reset signal of the specified reset domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_reset_proto_ops {
    pub ph): *const *const int (num_domains_get)(struct scmi_protocol_handle,
    pub domain): u32,
    pub domain): *const *const *const int (latency_get)(struct scmi_protocol_handle ph, u32,
    pub domain): *const *const *const int (reset)(struct scmi_protocol_handle ph, u32,
    pub domain): *const *const *const int (assert)(struct scmi_protocol_handle ph, u32,
    pub domain): *const *const *const int (deassert)(struct scmi_protocol_handle ph, u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_voltage_level_mode {
    SCMI_VOLTAGE_LEVEL_SET_AUTO,
    SCMI_VOLTAGE_LEVEL_SET_SYNC,
}

//
// struct scmi_voltage_info - describe one available SCMI Voltage Domain
//
// @id: the domain ID as advertised by the platform
// @segmented: defines the layout of the entries of array @levels_uv.
// - when True the entries are to be interpreted as triplets,
// each defining a segment representing a range of equally
// space voltages: <lowest_volts>, <highest_volt>, <step_uV>
// - when False the entries simply represent a single discrete
// supported voltage level
// @negative_volts_allowed: True if any of the entries of @levels_uv represent
// a negative voltage.
// @async_level_set: True when the voltage domain supports asynchronous level
// set commands.
// @name: name assigned to the Voltage Domain by platform
// @num_levels: number of total entries in @levels_uv.
// @levels_uv: array of entries describing the available voltage levels for
// this domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_voltage_info {
    pub id: c_uint,
    pub segmented: bool,
    pub negative_volts_allowed: bool,
    pub async_level_set: bool,
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub num_levels: c_uint,
pub const SCMI_VOLTAGE_SEGMENT_LOW: c_int = 0;
pub const SCMI_VOLTAGE_SEGMENT_HIGH: c_int = 1;
pub const SCMI_VOLTAGE_SEGMENT_STEP: c_int = 2;
    pub levels_uv: *mut c_int,
}

//
// struct scmi_voltage_proto_ops - represents the various operations provided
// by SCMI Voltage Protocol
//
// @num_domains_get: get the count of voltage domains provided by SCMI
// @info_get: get the information of the specified domain
// @config_set: set the config for the specified domain
// @config_get: get the config of the specified domain
// @level_set: set the voltage level for the specified domain
// @level_get: get the voltage level of the specified domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_voltage_proto_ops {
    pub ph): *const *const int (num_domains_get)(struct scmi_protocol_handle,
    pub domain_id): *const *const (struct scmi_protocol_handle ph, u32,
    pub config): u32,
pub const SCMI_VOLTAGE_ARCH_STATE_OFF: c_uint = 0x0;
pub const SCMI_VOLTAGE_ARCH_STATE_ON: c_uint = 0x7;
    pub config): *mut u32,
    pub volt_uV): scmi_voltage_level_mode mode, s32,
    pub volt_uV): *mut i32,
}

//
// struct scmi_powercap_info  - Describe one available Powercap domain
//
// @id: Domain ID as advertised by the platform.
// @notify_powercap_cap_change: CAP change notification support.
// @notify_powercap_measurement_change: MEASUREMENTS change notifications
// support.
// @async_powercap_cap_set: Asynchronous CAP set support.
// @powercap_cap_config: CAP configuration support.
// @powercap_monitoring: Monitoring (measurements) support.
// @powercap_pai_config: PAI configuration support.
// @powercap_scale_mw: Domain reports power data in milliwatt units.
// @powercap_scale_uw: Domain reports power data in microwatt units.
// Note that, when both @powercap_scale_mw and
// @powercap_scale_uw are set to false, the domain
// reports power data on an abstract linear scale.
// @name: name assigned to the Powercap Domain by platform.
// @min_pai: Minimum configurable PAI.
// @max_pai: Maximum configurable PAI.
// @pai_step: Step size between two consecutive PAI values.
// @min_power_cap: Minimum configurable CAP.
// @max_power_cap: Maximum configurable CAP.
// @power_cap_step: Step size between two consecutive CAP values.
// @sustainable_power: Maximum sustainable power consumption for this domain
// under normal conditions.
// @accuracy: The accuracy with which the power is measured and reported in
// integral multiples of 0.001 percent.
// @parent_id: Identifier of the containing parent power capping domain, or the
// value 0xFFFFFFFF if this powercap domain is a root domain not
// contained in any other domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_powercap_info {
    pub id: c_uint,
    pub notify_powercap_cap_change: bool,
    pub notify_powercap_measurement_change: bool,
    pub async_powercap_cap_set: bool,
    pub powercap_cap_config: bool,
    pub powercap_monitoring: bool,
    pub powercap_pai_config: bool,
    pub powercap_scale_mw: bool,
    pub powercap_scale_uw: bool,
    pub fastchannels: bool,
    pub name: [c_char; SCMI_MAX_STR_SIZE],
    pub min_pai: c_uint,
    pub max_pai: c_uint,
    pub pai_step: c_uint,
    pub min_power_cap: c_uint,
    pub max_power_cap: c_uint,
    pub power_cap_step: c_uint,
    pub sustainable_power: c_uint,
    pub accuracy: c_uint,
pub const SCMI_POWERCAP_ROOT_ZONE_ID: c_uint = 0xFFFFFFFFUL;
    pub parent_id: c_uint,
    pub fc_info: *mut scmi_fc_info,
}

//
// struct scmi_powercap_proto_ops - represents the various operations provided
// by SCMI Powercap Protocol
//
// @num_domains_get: get the count of powercap domains provided by SCMI.
// @info_get: get the information for the specified domain.
// @cap_get: get the current CAP value for the specified domain.
// On SCMI platforms supporting powercap zone disabling, this could
// report a zero value for a zone where powercapping is disabled.
// @cap_set: set the CAP value for the specified domain to the provided value;
// if the domain supports setting the CAP with an asynchronous command
// this request will finally trigger an asynchronous transfer, but, if
// @ignore_dresp here is set to true, this call will anyway return
// immediately without waiting for the related delayed response.
// Note that the powercap requested value must NOT be zero, even if
// the platform supports disabling a powercap by setting its cap to
// zero (since SCMI v3.2): there are dedicated operations that should
// be used for that. (@cap_enable_set/get)
// @cap_enable_set: enable or disable the powercapping on the specified domain,
// if supported by the SCMI platform implementation.
// Note that, by the SCMI specification, the platform can
// silently ignore our disable request and decide to enforce
// anyway some other powercap value requested by another agent
// on the system: for this reason @cap_get and @cap_enable_get
// will always report the final platform view of the powercaps.
// @cap_enable_get: get the current CAP enable status for the specified domain.
// @pai_get: get the current PAI value for the specified domain.
// @pai_set: set the PAI value for the specified domain to the provided value.
// @measurements_get: retrieve the current average power measurements for the
// specified domain and the related PAI upon which is
// calculated.
// @measurements_threshold_set: set the desired low and high power thresholds
// to be used when registering for notification
// of type POWERCAP_MEASUREMENTS_NOTIFY with this
// powercap domain.
// Note that this must be called at least once
// before registering any callback with the usual
// @scmi_notify_ops; moreover, in case this method
// is called with measurement notifications already
// enabled it will also trigger, transparently, a
// proper update of the power thresholds configured
// in the SCMI backend server.
// @measurements_threshold_get: get the currently configured low and high power
// thresholds used when registering callbacks for
// notification POWERCAP_MEASUREMENTS_NOTIFY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_powercap_proto_ops {
    pub ph): *const *const int (num_domains_get)(struct scmi_protocol_handle,
    pub domain_id): *const *const (struct scmi_protocol_handle ph, u32,
    pub power_cap): *mut u32,
    pub ignore_dresp): u32 power_cap, bool,
    pub enable): u32 domain_id, bool,
    pub enable): *mut u32 domain_id, bool,
    pub pai): *mut u32,
    pub pai): u32,
    pub pai): *mut *mut u32 domain_id, u32 average_power, u32,
    pub power_thresh_high): u32,
    pub power_thresh_high): *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_pinctrl_selector_type {
    PIN_TYPE = 0,
    GROUP_TYPE,
    FUNCTION_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_pinctrl_conf_type {
    SCMI_PIN_DEFAULT = 0,
    SCMI_PIN_BIAS_BUS_HOLD = 1,
    SCMI_PIN_BIAS_DISABLE = 2,
    SCMI_PIN_BIAS_HIGH_IMPEDANCE = 3,
    SCMI_PIN_BIAS_PULL_UP = 4,
    SCMI_PIN_BIAS_PULL_DEFAULT = 5,
    SCMI_PIN_BIAS_PULL_DOWN = 6,
    SCMI_PIN_DRIVE_OPEN_DRAIN = 7,
    SCMI_PIN_DRIVE_OPEN_SOURCE = 8,
    SCMI_PIN_DRIVE_PUSH_PULL = 9,
    SCMI_PIN_DRIVE_STRENGTH = 10,
    SCMI_PIN_INPUT_DEBOUNCE = 11,
    SCMI_PIN_INPUT_MODE = 12,
    SCMI_PIN_PULL_MODE = 13,
    SCMI_PIN_INPUT_VALUE = 14,
    SCMI_PIN_INPUT_SCHMITT = 15,
    SCMI_PIN_LOW_POWER_MODE = 16,
    SCMI_PIN_OUTPUT_MODE = 17,
    SCMI_PIN_OUTPUT_VALUE = 18,
    SCMI_PIN_POWER_SOURCE = 19,
    SCMI_PIN_SLEW_RATE = 20,
    SCMI_PIN_OEM_START = 192,
    SCMI_PIN_OEM_END = 255,
}

//
// struct scmi_pinctrl_proto_ops - represents the various operations provided
// by SCMI Pinctrl Protocol
//
// @count_get: returns count of the registered elements in given type
// @name_get: returns name by index of given type
// @group_pins_get: returns the set of pins, assigned to the specified group
// @function_groups_get: returns the set of groups, assigned to the specified
// function
// @mux_set: set muxing function for groups of pins
// @settings_get_one: returns one configuration parameter for pin or group
// specified by config_type
// @settings_get_all: returns all configuration parameters for pin or group
// @settings_conf: sets the configuration parameter for pin or group
// @pin_request: aquire pin before selecting mux setting
// @pin_free: frees pin, acquired by request_pin call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_pinctrl_proto_ops {
    pub type): scmi_pinctrl_selector_type,
    pub name): *const c_char,
    pub nr_pins): *mut c_uint,
    pub groups): *const c_uint,
    pub group): u32,
    pub config_value): *mut u32,
    pub config_values): *mut u32,
    pub config_value): *mut u32,
    pub pin): *const *const *const int (pin_request)(struct scmi_protocol_handle ph, u32,
    pub pin): *const *const *const int (pin_free)(struct scmi_protocol_handle ph, u32,
}

//
// struct scmi_notify_ops  - represents notifications' operations provided by
// SCMI core
// @devm_event_notifier_register: Managed registration of a notifier_block for
// the requested event
// @devm_event_notifier_unregister: Managed unregistration of a notifier_block
// for the requested event
// @event_notifier_register: Register a notifier_block for the requested event
// @event_notifier_unregister: Unregister a notifier_block for the requested
// event
//
// A user can register/unregister its own notifier_block against the wanted
// platform instance regarding the desired event identified by the
// tuple: (proto_id, evt_id, src_id) using the provided register/unregister
// interface where:
//
// @sdev: The scmi_device to use when calling the devres managed ops devm_
// @handle: The handle identifying the platform instance to use, when not
// calling the managed ops devm_
// @proto_id: The protocol ID as in SCMI Specification
// @evt_id: The message ID of the desired event as in SCMI Specification
// @src_id: A pointer to the desired source ID if different sources are
// possible for the protocol (like domain_id, sensor_id...etc)
//
// @src_id can be provided as NULL if it simply does NOT make sense for
// the protocol at hand, OR if the user is explicitly interested in
// receiving notifications from ANY existent source associated to the
// specified proto_id / evt_id.
//
// Received notifications are finally delivered to the registered users,
// invoking the callback provided with the notifier_block *nb as follows:
//
// int user_cb(nb, evt_id, report)
//
// with:
//
// @nb: The notifier block provided by the user
// @evt_id: The message ID of the delivered event
// @report: A custom struct describing the specific event delivered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_notify_ops {
    pub nb): *mut notifier_block,
    pub nb): *mut notifier_block,
    pub nb): *mut notifier_block,
    pub nb): *mut notifier_block,
}

//
// struct scmi_handle - Handle returned to ARM SCMI clients for usage.
//
// @dev: pointer to the SCMI device
// @version: pointer to the structure containing SCMI version information
// @devm_protocol_acquire: devres managed method to get hold of a protocol,
// causing its initialization and related resource
// accounting
// @devm_protocol_get: devres managed method to acquire a protocol and get specific
// operations and a dedicated protocol handler
// @devm_protocol_put: devres managed method to release a protocol
// @is_transport_atomic: method to check if the underlying transport for this
// instance handle is configured to support atomic
// transactions for commands.
// Some users of the SCMI stack in the upper layers could
// be interested to know if they can assume SCMI
// command transactions associated to this handle will
// never sleep and act accordingly.
// An optional atomic threshold value could be returned
// where configured.
// @notify_ops: pointer to set of notifications related operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_handle {
    pub dev: *mut device,
    pub version: *mut scmi_base_info,
    pub proto): u8,
    pub ph): *mut scmi_protocol_handle,
    pub proto): *mut *mut *mut void (devm_protocol_put)(struct scmi_device sdev, u8,
    pub atomic_threshold): *mut c_uint,
    pub notify_ops: *const scmi_notify_ops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_std_protocol {
    SCMI_PROTOCOL_BASE = 0x10,
    SCMI_PROTOCOL_POWER = 0x11,
    SCMI_PROTOCOL_SYSTEM = 0x12,
    SCMI_PROTOCOL_PERF = 0x13,
    SCMI_PROTOCOL_CLOCK = 0x14,
    SCMI_PROTOCOL_SENSOR = 0x15,
    SCMI_PROTOCOL_RESET = 0x16,
    SCMI_PROTOCOL_VOLTAGE = 0x17,
    SCMI_PROTOCOL_POWERCAP = 0x18,
    SCMI_PROTOCOL_PINCTRL = 0x19,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_system_events {
    SCMI_SYSTEM_SHUTDOWN,
    SCMI_SYSTEM_COLDRESET,
    SCMI_SYSTEM_WARMRESET,
    SCMI_SYSTEM_POWERUP,
    SCMI_SYSTEM_SUSPEND,
    SCMI_SYSTEM_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_device {
    pub id: u32,
    pub protocol_id: u8,
    pub name: *const c_char,
    pub dev: device,
    pub handle: *mut scmi_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_device_id {
    pub protocol_id: u8,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_driver {
    pub name: *const c_char,
    pub sdev): *mut *mut int (probe)(struct scmi_device,
    pub sdev): *mut *mut void (remove)(struct scmi_device,
    pub id_table: *const scmi_device_id,
    pub driver: device_driver,
}

extern "C" {
    pub fn scmi_driver_unregister(driver: *mut scmi_driver);
}

//
// module_scmi_driver() - Helper macro for registering a scmi driver
// @__scmi_driver: scmi_driver structure
//
// Helper macro for scmi drivers to set up proper module init / exit
// functions.  Replaces module_init() and module_exit() and keeps people from
// printing pointless things to the kernel log when their driver is loaded.
//

//
// module_scmi_protocol() - Helper macro for registering a scmi protocol
// @__scmi_protocol: scmi_protocol structure
//
// Helper macro for scmi drivers to set up proper module init / exit
// functions.  Replaces module_init() and module_exit() and keeps people from
// printing pointless things to the kernel log when their driver is loaded.
//

extern "C" {
    pub fn scmi_protocol_register(proto: *const scmi_protocol) -> c_int;
}
extern "C" {
    pub fn scmi_protocol_unregister(proto: *const scmi_protocol);
}
// SCMI Notification API - Custom Event Reports
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_notification_events {
    SCMI_EVENT_POWER_STATE_CHANGED = 0x0,
    SCMI_EVENT_CLOCK_RATE_CHANGED = 0x0,
    SCMI_EVENT_CLOCK_RATE_CHANGE_REQUESTED = 0x1,
    SCMI_EVENT_PERFORMANCE_LIMITS_CHANGED = 0x0,
    SCMI_EVENT_PERFORMANCE_LEVEL_CHANGED = 0x1,
    SCMI_EVENT_SENSOR_TRIP_POINT_EVENT = 0x0,
    SCMI_EVENT_SENSOR_UPDATE = 0x1,
    SCMI_EVENT_RESET_ISSUED = 0x0,
    SCMI_EVENT_BASE_ERROR_EVENT = 0x0,
    SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER = 0x0,
    SCMI_EVENT_POWERCAP_CAP_CHANGED = 0x0,
    SCMI_EVENT_POWERCAP_MEASUREMENTS_CHANGED = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_state_changed_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub power_state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_rate_notif_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub clock_id: c_uint,
    pub rate: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_system_power_state_notifier_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,

    pub flags: c_uint,
    pub system_state: c_uint,
    pub timeout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_perf_limits_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub range_max: c_uint,
    pub range_min: c_uint,
    pub range_max_freq: c_ulong,
    pub range_min_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_perf_level_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub performance_level: c_uint,
    pub performance_level_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_trip_point_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub sensor_id: c_uint,
    pub trip_point_desc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_sensor_update_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub sensor_id: c_uint,
    pub readings_count: c_uint,
    pub readings: [scmi_sensor_reading; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_reset_issued_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub reset_state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_base_error_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub fatal: bool,
    pub cmd_count: c_uint,
    pub reports: [c_ulonglong; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_powercap_cap_changed_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub power_cap: c_uint,
    pub pai: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_powercap_meas_changed_report {
    pub timestamp: ktime_t,
    pub agent_id: c_uint,
    pub domain_id: c_uint,
    pub power: c_uint,
}
