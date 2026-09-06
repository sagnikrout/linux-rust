//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/ti-soc-thermal/ti-bandgap.h
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
// OMAP4 Bandgap temperature sensor driver
//
// Copyright (C) 2011 Texas Instruments Incorporated - http://www.ti.com
// Contact:
// Eduardo Valentin <eduardo.valentin@ti.com>
//

//
// DOC: bandgap driver data structure
// ==================================
//
// +----------+----------------+
// | struct temp_sensor_regval |
// +---------------------------+
// * (Array of)
// |
// +-------------------+   +-----------------+
// | struct ti_bandgap |-->| struct device * |
// +----------+--------+   +-----------------+
// |
// V
// +------------------------+
// | struct ti_bandgap_data |
// +------------------------+
// |
// * (Array of)
// +------------+------------------------------------------------------+
// | +----------+------------+   +-------------------------+           |
// | | struct ti_temp_sensor |-->| struct temp_sensor_data |           |
// | +-----------------------+   +------------+------------+           |
// |            |                                                      |
// |            +                                                      |
// |            V                                                      |
// | +----------+-------------------+                                  |
// | | struct temp_sensor_registers |                                  |
// | +------------------------------+                                  |
// |                                                                   |
// +-------------------------------------------------------------------+
//
// Above is a simple diagram describing how the data structure below
// are organized. For each bandgap device there should be a ti_bandgap_data
// containing the device instance configuration, as well as, an array of
// sensors, representing every sensor instance present in this bandgap.
//
// struct temp_sensor_registers - descriptor to access registers and bitfields
// @temp_sensor_ctrl: TEMP_SENSOR_CTRL register offset
// @bgap_tempsoff_mask: mask to temp_sensor_ctrl.tempsoff
// @bgap_soc_mask: mask to temp_sensor_ctrl.soc
// @bgap_eocz_mask: mask to temp_sensor_ctrl.eocz
// @bgap_dtemp_mask: mask to temp_sensor_ctrl.dtemp
// @bgap_mask_ctrl: BANDGAP_MASK_CTRL register offset
// @mask_hot_mask: mask to bandgap_mask_ctrl.mask_hot
// @mask_cold_mask: mask to bandgap_mask_ctrl.mask_cold
// @mask_counter_delay_mask: mask to bandgap_mask_ctrl.mask_counter_delay
// @mask_freeze_mask: mask to bandgap_mask_ctrl.mask_free
// @bgap_mode_ctrl: BANDGAP_MODE_CTRL register offset
// @mode_ctrl_mask: mask to bandgap_mode_ctrl.mode_ctrl
// @bgap_counter: BANDGAP_COUNTER register offset
// @counter_mask: mask to bandgap_counter.counter
// @bgap_threshold: BANDGAP_THRESHOLD register offset (TALERT thresholds)
// @threshold_thot_mask: mask to bandgap_threhold.thot
// @threshold_tcold_mask: mask to bandgap_threhold.tcold
// @tshut_threshold: TSHUT_THRESHOLD register offset (TSHUT thresholds)
// @tshut_hot_mask: mask to tshut_threhold.thot
// @tshut_cold_mask: mask to tshut_threhold.thot
// @bgap_status: BANDGAP_STATUS register offset
// @status_hot_mask: mask to bandgap_status.hot
// @status_cold_mask: mask to bandgap_status.cold
// @ctrl_dtemp_1: CTRL_DTEMP1 register offset
// @ctrl_dtemp_2: CTRL_DTEMP2 register offset
// @bgap_efuse: BANDGAP_EFUSE register offset
//
// The register offsets and bitfields might change across
// OMAP and variants versions. Hence this struct serves as a
// descriptor map on how to access the registers and the bitfields.
//
// This descriptor contains registers of all versions of bandgap chips.
// Not all versions will use all registers, depending on the available
// features. Please read TRMs for descriptive explanation on each bitfield.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_sensor_registers {
    pub temp_sensor_ctrl: u32,
    pub bgap_tempsoff_mask: u32,
    pub bgap_soc_mask: u32,
    pub bgap_eocz_mask: u32,
    pub bgap_dtemp_mask: u32,
    pub bgap_mask_ctrl: u32,
    pub mask_hot_mask: u32,
    pub mask_cold_mask: u32,
    pub mask_counter_delay_mask: u32,
    pub mask_freeze_mask: u32,
    pub bgap_mode_ctrl: u32,
    pub mode_ctrl_mask: u32,
    pub bgap_counter: u32,
    pub counter_mask: u32,
    pub bgap_threshold: u32,
    pub threshold_thot_mask: u32,
    pub threshold_tcold_mask: u32,
    pub tshut_threshold: u32,
    pub tshut_hot_mask: u32,
    pub tshut_cold_mask: u32,
    pub bgap_status: u32,
    pub status_hot_mask: u32,
    pub status_cold_mask: u32,
    pub ctrl_dtemp_1: u32,
    pub ctrl_dtemp_2: u32,
    pub bgap_efuse: u32,
}

//
// struct temp_sensor_data - The thresholds and limits for temperature sensors.
// @tshut_hot: temperature to trigger a thermal reset (initial value)
// @tshut_cold: temp to get the plat out of reset due to thermal (init val)
// @t_hot: temperature to trigger a thermal alert (high initial value)
// @t_cold: temperature to trigger a thermal alert (low initial value)
// @min_freq: sensor minimum clock rate
// @max_freq: sensor maximum clock rate
//
// This data structure will hold the required thresholds and temperature limits
// for a specific temperature sensor, like shutdown temperature, alert
// temperature, clock / rate used, ADC conversion limits and update intervals
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_sensor_data {
    pub tshut_hot: u32,
    pub tshut_cold: u32,
    pub t_hot: u32,
    pub t_cold: u32,
    pub min_freq: u32,
    pub max_freq: u32,
}

//
// struct temp_sensor_regval - temperature sensor register values and priv data
// @bg_mode_ctrl: temp sensor control register value
// @bg_ctrl: bandgap ctrl register value
// @bg_counter: bandgap counter value
// @bg_threshold: bandgap threshold register value
// @tshut_threshold: bandgap tshut register value
// @data: private data
//
// Data structure to save and restore bandgap register set context. Only
// required registers are shadowed, when needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_sensor_regval {
    pub bg_mode_ctrl: u32,
    pub bg_ctrl: u32,
    pub bg_counter: u32,
    pub bg_threshold: u32,
    pub tshut_threshold: u32,
    pub data: *mut c_void,
}

//
// struct ti_bandgap - bandgap device structure
// @dev: struct device pointer
// @base: io memory base address
// @conf: struct with bandgap configuration set (# sensors, conv_table, etc)
// @regval: temperature sensor register values
// @fclock: pointer to functional clock of temperature sensor
// @div_clk: pointer to divider clock of temperature sensor fclk
// @lock: spinlock for ti_bandgap structure
// @irq: MPU IRQ number for thermal alert
// @tshut_gpio: GPIO where Tshut signal is routed
// @clk_rate: Holds current clock rate
//
// The bandgap device structure representing the bandgap device instance.
// It holds most of the dynamic stuff. Configurations and sensor specific
// entries are inside the @conf structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_bandgap {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub conf: *const ti_bandgap_data,
    pub regval: *mut temp_sensor_regval,
    pub fclock: *mut clk,
    pub div_clk: *mut clk,
    pub /: *mut *mut spinlock_t lock; / shields this struct,
    pub irq: c_int,
    pub tshut_gpiod: *mut gpio_desc,
    pub clk_rate: u32,
    pub nb: notifier_block,
    pub is_suspended:1: c_uint,
}

//
// struct ti_temp_sensor - bandgap temperature sensor configuration data
// @ts_data: pointer to struct with thresholds, limits of temperature sensor
// @registers: pointer to the list of register offsets and bitfields
// @domain: the name of the domain where the sensor is located
// @slope_pcb: sensor gradient slope info for hotspot extrapolation equation
// with no external influence
// @constant_pcb: sensor gradient const info for hotspot extrapolation equation
// with no external influence
// @register_cooling: function to describe how this sensor is going to be cooled
// @unregister_cooling: function to release cooling data
//
// Data structure to describe a temperature sensor handled by a bandgap device.
// It should provide configuration details on this sensor, such as how to
// access the registers affecting this sensor, shadow register buffer, how to
// assess the gradient from hotspot, how to cooldown the domain when sensor
// reports too hot temperature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_temp_sensor {
    pub ts_data: *mut temp_sensor_data,
    pub registers: *mut temp_sensor_registers,
    pub domain: *mut c_char,
// for hotspot extrapolation
    pub slope_pcb: c_int,
    pub constant_pcb: c_int,
    pub id): *mut *mut *mut int (register_cooling)(struct ti_bandgap bgp, int,
    pub id): *mut *mut *mut int (unregister_cooling)(struct ti_bandgap bgp, int,
}

//
// DOC: ti bandgap feature types
//
// TI_BANDGAP_FEATURE_TSHUT - used when the thermal shutdown signal output
// of a bandgap device instance is routed to the processor. This means
// the system must react and perform the shutdown by itself (handle an
// IRQ, for instance).
//
// TI_BANDGAP_FEATURE_TSHUT_CONFIG - used when the bandgap device has control
// over the thermal shutdown configuration. This means that the thermal
// shutdown thresholds are programmable, for instance.
//
// TI_BANDGAP_FEATURE_TALERT - used when the bandgap device instance outputs
// a signal representing violation of programmable alert thresholds.
//
// TI_BANDGAP_FEATURE_MODE_CONFIG - used when it is possible to choose which
// mode, continuous or one shot, the bandgap device instance will operate.
//
// TI_BANDGAP_FEATURE_COUNTER - used when the bandgap device instance allows
// programming the update interval of its internal state machine.
//
// TI_BANDGAP_FEATURE_POWER_SWITCH - used when the bandgap device allows
// itself to be switched on/off.
//
// TI_BANDGAP_FEATURE_CLK_CTRL - used when the clocks feeding the bandgap
// device are gateable or not.
//
// TI_BANDGAP_FEATURE_FREEZE_BIT - used when the bandgap device features
// a history buffer that its update can be freezed/unfreezed.
//
// TI_BANDGAP_FEATURE_COUNTER_DELAY - used when the bandgap device features
// a delay programming based on distinct values.
//
// TI_BANDGAP_FEATURE_HISTORY_BUFFER - used when the bandgap device features
// a history buffer of temperatures.
//
// TI_BANDGAP_FEATURE_ERRATA_814 - used to workaorund when the bandgap device
// has Errata 814
// TI_BANDGAP_FEATURE_UNRELIABLE - used when the sensor readings are too
// inaccurate.
// TI_BANDGAP_FEATURE_CONT_MODE_ONLY - used when single mode hangs the sensor
// TI_BANDGAP_HAS(b, f) - macro to check if a bandgap device is capable of a
// specific feature (above) or not. Return non-zero, if yes.
//

//
// struct ti_bandgap_data - ti bandgap data configuration structure
// @features: a bitwise flag set to describe the device features
// @conv_table: Pointer to ADC to temperature conversion table
// @adc_start_val: ADC conversion table starting value
// @adc_end_val: ADC conversion table ending value
// @fclock_name: clock name of the functional clock
// @div_ck_name: clock name of the clock divisor
// @sensor_count: count of temperature sensor within this bandgap device
// @report_temperature: callback to report thermal alert to thermal API
// @expose_sensor: callback to export sensor to thermal API
// @remove_sensor: callback to destroy sensor from thermal API
// @sensors: array of sensors present in this bandgap instance
//
// This is a data structure which should hold most of the static configuration
// of a bandgap device instance. It should describe which features this instance
// is capable of, the clock names to feed this device, the amount of sensors and
// their configuration representation, and how to export and unexport them to
// a thermal API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_bandgap_data {
    pub features: c_uint,
    pub conv_table: *const c_int,
    pub adc_start_val: u32,
    pub adc_end_val: u32,
    pub fclock_name: *mut c_char,
    pub div_ck_name: *mut c_char,
    pub sensor_count: c_int,
    pub id): *mut *mut *mut int (report_temperature)(struct ti_bandgap bgp, int,
    pub domain): *mut *mut *mut int (expose_sensor)(struct ti_bandgap bgp, int id, char,
    pub id): *mut *mut *mut int (remove_sensor)(struct ti_bandgap bgp, int,
// this needs to be at the end
    pub sensors: [ti_temp_sensor; ],
}

extern "C" {
    pub fn ti_bandgap_set_sensor_data(bgp: *mut ti_bandgap, id: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ti_bandgap_get_trend(bgp: *mut ti_bandgap, id: c_int, trend: *mut c_int) -> c_int;
}

