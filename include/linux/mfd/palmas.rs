//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/palmas.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TI Palmas
//
// Copyright 2011-2013 Texas Instruments Inc.
//
// Author: Graeme Gregory <gg@slimlogic.co.uk>
// Author: Ian Lartey <ian@slimlogic.co.uk>
//

pub const PALMAS_NUM_CLIENTS: c_int = 3;
// The ID_REVISION NUMBERS
pub const PALMAS_CHIP_OLD_ID: c_uint = 0x0000;
pub const PALMAS_CHIP_ID: c_uint = 0xC035;
pub const PALMAS_CHIP_CHARGER_ID: c_uint = 0xC036;

//
// Palmas PMIC feature types
//
// PALMAS_PMIC_FEATURE_SMPS10_BOOST - used when the PMIC provides SMPS10_BOOST
// regulator.
//
// PALMAS_PMIC_HAS(b, f) - macro to check if a bandgap device is capable of a
// specific feature (above) or not. Return non-zero, if yes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum palmas_usb_state {
    PALMAS_USB_STATE_DISCONNECT,
    PALMAS_USB_STATE_VBUS,
    PALMAS_USB_STATE_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas {
    pub dev: *mut device,
    pub i2c_clients: [*mut i2c_client; PALMAS_NUM_CLIENTS],
    pub regmap: [*mut regmap; PALMAS_NUM_CLIENTS],
// Stored chip id
    pub id: c_int,
    pub features: c_uint,
// IRQ Data
    pub irq: c_int,
    pub irq_mask: u32,
    pub irq_lock: mutex,
    pub irq_data: *mut regmap_irq_chip_data,
    pub pmic_ddata: *mut palmas_pmic_driver_data,
// Child Devices
    pub pmic: *mut palmas_pmic,
    pub gpadc: *mut palmas_gpadc,
    pub resource: *mut palmas_resource,
    pub usb: *mut palmas_usb,
// GPIO MUXing
    pub gpio_muxed: u8,
    pub led_muxed: u8,
    pub pwm_muxed: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_sleep_requestor_info {
    pub id: c_int,
    pub reg_offset: c_int,
    pub bit_pos: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_regs_info {
    pub name: *const c_char,
    pub sname: *const c_char,
    pub vsel_addr: u8,
    pub ctrl_addr: u8,
    pub tstep_addr: u8,
    pub sleep_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_pmic_driver_data {
    pub smps_start: c_int,
    pub smps_end: c_int,
    pub ldo_begin: c_int,
    pub ldo_end: c_int,
    pub max_reg: c_int,
    pub has_regen3: bool,
    pub palmas_regs_info: *mut palmas_regs_info,
    pub palmas_matches: *mut of_regulator_match,
    pub sleep_req_info: *mut palmas_sleep_requestor_info,
    pub config): regulator_config,
    pub config): regulator_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_gpadc_platform_data {
// Channel 3 current source is only enabled during conversion
    pub /: *mut *mut int ch3_current; / 0: off; 1: 10uA; 2: 400uA; 3: 800 uA,
// Channel 0 current source can be used for battery detection.
// If used for battery detection this will cause a permanent current
// consumption depending on current level set here.
//
    pub /: *mut *mut int ch0_current; / 0: off; 1: 5uA; 2: 15uA; 3: 20 uA,
    pub /: *mut *mut bool extended_delay; / use extended delay for conversion,
// default BAT_REMOVAL_DAT setting on device probe
    pub bat_removal: c_int,
// Sets the START_POLARITY bit in the RT_CTRL register
    pub start_polarity: c_int,
    pub auto_conversion_period_ms: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_reg_init {
// warm_rest controls the voltage levels after a warm reset
//
// 0: reload default values from OTP on warm reset
// 1: maintain voltage from VSEL on warm reset
//
    pub warm_reset: c_int,
// roof_floor controls whether the regulator uses the i2c style
// of DVS or uses the method where a GPIO or other control method is
// attached to the NSLEEP/ENABLE1/ENABLE2 pins
//
// For SMPS
//
// 0: i2c selection of voltage
// 1: pin selection of voltage.
//
// For LDO unused
//
    pub roof_floor: c_int,
// sleep_mode is the mode loaded to MODE_SLEEP bits as defined in
// the data sheet.
//
// For SMPS
//
// 0: Off
// 1: AUTO
// 2: ECO
// 3: Forced PWM
//
// For LDO
//
// 0: Off
// 1: On
//
    pub mode_sleep: c_int,
// voltage_sel is the bitfield loaded onto the SMPSX_VOLTAGE
// register. Set this is the default voltage set in OTP needs
// to be overridden.
//
    pub vsel: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum palmas_regulators {
// SMPS regulators
    PALMAS_REG_SMPS12,
    PALMAS_REG_SMPS123,
    PALMAS_REG_SMPS3,
    PALMAS_REG_SMPS45,
    PALMAS_REG_SMPS457,
    PALMAS_REG_SMPS6,
    PALMAS_REG_SMPS7,
    PALMAS_REG_SMPS8,
    PALMAS_REG_SMPS9,
    PALMAS_REG_SMPS10_OUT2,
    PALMAS_REG_SMPS10_OUT1,
// LDO regulators
    PALMAS_REG_LDO1,
    PALMAS_REG_LDO2,
    PALMAS_REG_LDO3,
    PALMAS_REG_LDO4,
    PALMAS_REG_LDO5,
    PALMAS_REG_LDO6,
    PALMAS_REG_LDO7,
    PALMAS_REG_LDO8,
    PALMAS_REG_LDO9,
    PALMAS_REG_LDOLN,
    PALMAS_REG_LDOUSB,
// External regulators
    PALMAS_REG_REGEN1,
    PALMAS_REG_REGEN2,
    PALMAS_REG_REGEN3,
    PALMAS_REG_SYSEN1,
    PALMAS_REG_SYSEN2,
// Total number of regulators
    PALMAS_NUM_REGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65917_regulators {
// SMPS regulators
    TPS65917_REG_SMPS1,
    TPS65917_REG_SMPS2,
    TPS65917_REG_SMPS3,
    TPS65917_REG_SMPS4,
    TPS65917_REG_SMPS5,
    TPS65917_REG_SMPS12,
// LDO regulators
    TPS65917_REG_LDO1,
    TPS65917_REG_LDO2,
    TPS65917_REG_LDO3,
    TPS65917_REG_LDO4,
    TPS65917_REG_LDO5,
    TPS65917_REG_REGEN1,
    TPS65917_REG_REGEN2,
    TPS65917_REG_REGEN3,

// Total number of regulators
    TPS65917_NUM_REGS,
}

// External controll signal name
//
// Palmas device resources can be controlled externally for
// enabling/disabling it rather than register write through i2c.
// Add the external controlled requestor ID for different resources.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum palmas_external_requestor_id {
    PALMAS_EXTERNAL_REQSTR_ID_REGEN1,
    PALMAS_EXTERNAL_REQSTR_ID_REGEN2,
    PALMAS_EXTERNAL_REQSTR_ID_SYSEN1,
    PALMAS_EXTERNAL_REQSTR_ID_SYSEN2,
    PALMAS_EXTERNAL_REQSTR_ID_CLK32KG,
    PALMAS_EXTERNAL_REQSTR_ID_CLK32KGAUDIO,
    PALMAS_EXTERNAL_REQSTR_ID_REGEN3,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS12,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS3,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS45,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS6,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS7,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS8,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS9,
    PALMAS_EXTERNAL_REQSTR_ID_SMPS10,
    PALMAS_EXTERNAL_REQSTR_ID_LDO1,
    PALMAS_EXTERNAL_REQSTR_ID_LDO2,
    PALMAS_EXTERNAL_REQSTR_ID_LDO3,
    PALMAS_EXTERNAL_REQSTR_ID_LDO4,
    PALMAS_EXTERNAL_REQSTR_ID_LDO5,
    PALMAS_EXTERNAL_REQSTR_ID_LDO6,
    PALMAS_EXTERNAL_REQSTR_ID_LDO7,
    PALMAS_EXTERNAL_REQSTR_ID_LDO8,
    PALMAS_EXTERNAL_REQSTR_ID_LDO9,
    PALMAS_EXTERNAL_REQSTR_ID_LDOLN,
    PALMAS_EXTERNAL_REQSTR_ID_LDOUSB,

// Last entry
    PALMAS_EXTERNAL_REQSTR_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65917_external_requestor_id {
    TPS65917_EXTERNAL_REQSTR_ID_REGEN1,
    TPS65917_EXTERNAL_REQSTR_ID_REGEN2,
    TPS65917_EXTERNAL_REQSTR_ID_REGEN3,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS1,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS2,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS3,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS4,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS5,
    TPS65917_EXTERNAL_REQSTR_ID_SMPS12,
    TPS65917_EXTERNAL_REQSTR_ID_LDO1,
    TPS65917_EXTERNAL_REQSTR_ID_LDO2,
    TPS65917_EXTERNAL_REQSTR_ID_LDO3,
    TPS65917_EXTERNAL_REQSTR_ID_LDO4,
    TPS65917_EXTERNAL_REQSTR_ID_LDO5,
// Last entry
    TPS65917_EXTERNAL_REQSTR_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_pmic_platform_data {
// An array of pointers to regulator init data indexed by regulator
// ID
//
    pub reg_data: [*mut regulator_init_data; PALMAS_NUM_REGS],
// An array of pointers to structures containing sleep mode and DVS
// configuration for regulators indexed by ID
//
    pub reg_init: [*mut palmas_reg_init; PALMAS_NUM_REGS],
// use LDO6 for vibrator control
    pub ldo6_vibrator: c_int,
// Enable tracking mode of LDO8
    pub enable_ldo8_tracking: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_usb_platform_data {
// Do we enable the wakeup comparator on probe
    pub wakeup: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_resource_platform_data {
    pub regen1_mode_sleep: c_int,
    pub regen2_mode_sleep: c_int,
    pub sysen1_mode_sleep: c_int,
    pub sysen2_mode_sleep: c_int,
// bitfield to be loaded to NSLEEP_RES_ASSIGN
    pub nsleep_res: u8,
// bitfield to be loaded to NSLEEP_SMPS_ASSIGN
    pub nsleep_smps: u8,
// bitfield to be loaded to NSLEEP_LDO_ASSIGN1
    pub nsleep_ldo1: u8,
// bitfield to be loaded to NSLEEP_LDO_ASSIGN2
    pub nsleep_ldo2: u8,
// bitfield to be loaded to ENABLE1_RES_ASSIGN
    pub enable1_res: u8,
// bitfield to be loaded to ENABLE1_SMPS_ASSIGN
    pub enable1_smps: u8,
// bitfield to be loaded to ENABLE1_LDO_ASSIGN1
    pub enable1_ldo1: u8,
// bitfield to be loaded to ENABLE1_LDO_ASSIGN2
    pub enable1_ldo2: u8,
// bitfield to be loaded to ENABLE2_RES_ASSIGN
    pub enable2_res: u8,
// bitfield to be loaded to ENABLE2_SMPS_ASSIGN
    pub enable2_smps: u8,
// bitfield to be loaded to ENABLE2_LDO_ASSIGN1
    pub enable2_ldo1: u8,
// bitfield to be loaded to ENABLE2_LDO_ASSIGN2
    pub enable2_ldo2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_clk_platform_data {
    pub clk32kg_mode_sleep: c_int,
    pub clk32kgaudio_mode_sleep: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_platform_data {
    pub irq_flags: c_int,
    pub gpio_base: c_int,
// bit value to be loaded to the POWER_CTRL register
    pub power_ctrl: u8,
//
// boolean to select if we want to configure muxing here
// then the two value to load into the registers if true
//
    pub mux_from_pdata: c_int,
    pub pad2: u8 pad1,,
    pub pm_off: bool,
    pub pmic_pdata: *mut palmas_pmic_platform_data,
    pub gpadc_pdata: *mut palmas_gpadc_platform_data,
    pub usb_pdata: *mut palmas_usb_platform_data,
    pub resource_pdata: *mut palmas_resource_platform_data,
    pub clk_pdata: *mut palmas_clk_platform_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_gpadc_calibration {
    pub gain: i32,
    pub gain_error: i32,
    pub offset_error: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_gpadc_result {
    pub raw_code: i32,
    pub corrected_code: i32,
    pub result: i32,
}

pub const PALMAS_MAX_CHANNELS: c_int = 16;
// Define the tps65917 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65917_irqs {
// INT1 registers
    TPS65917_RESERVED1,
    TPS65917_PWRON_IRQ,
    TPS65917_LONG_PRESS_KEY_IRQ,
    TPS65917_RESERVED2,
    TPS65917_PWRDOWN_IRQ,
    TPS65917_HOTDIE_IRQ,
    TPS65917_VSYS_MON_IRQ,
    TPS65917_RESERVED3,
// INT2 registers
    TPS65917_RESERVED4,
    TPS65917_OTP_ERROR_IRQ,
    TPS65917_WDT_IRQ,
    TPS65917_RESERVED5,
    TPS65917_RESET_IN_IRQ,
    TPS65917_FSD_IRQ,
    TPS65917_SHORT_IRQ,
    TPS65917_RESERVED6,
// INT3 registers
    TPS65917_GPADC_AUTO_0_IRQ,
    TPS65917_GPADC_AUTO_1_IRQ,
    TPS65917_GPADC_EOC_SW_IRQ,
    TPS65917_RESREVED6,
    TPS65917_RESERVED7,
    TPS65917_RESERVED8,
    TPS65917_RESERVED9,
    TPS65917_VBUS_IRQ,
// INT4 registers
    TPS65917_GPIO_0_IRQ,
    TPS65917_GPIO_1_IRQ,
    TPS65917_GPIO_2_IRQ,
    TPS65917_GPIO_3_IRQ,
    TPS65917_GPIO_4_IRQ,
    TPS65917_GPIO_5_IRQ,
    TPS65917_GPIO_6_IRQ,
    TPS65917_RESERVED10,
// Total Number IRQs
    TPS65917_NUM_IRQ,
}

// Define the palmas IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum palmas_irqs {
// INT1 registers
    PALMAS_CHARG_DET_N_VBUS_OVV_IRQ,
    PALMAS_PWRON_IRQ,
    PALMAS_LONG_PRESS_KEY_IRQ,
    PALMAS_RPWRON_IRQ,
    PALMAS_PWRDOWN_IRQ,
    PALMAS_HOTDIE_IRQ,
    PALMAS_VSYS_MON_IRQ,
    PALMAS_VBAT_MON_IRQ,
// INT2 registers
    PALMAS_RTC_ALARM_IRQ,
    PALMAS_RTC_TIMER_IRQ,
    PALMAS_WDT_IRQ,
    PALMAS_BATREMOVAL_IRQ,
    PALMAS_RESET_IN_IRQ,
    PALMAS_FBI_BB_IRQ,
    PALMAS_SHORT_IRQ,
    PALMAS_VAC_ACOK_IRQ,
// INT3 registers
    PALMAS_GPADC_AUTO_0_IRQ,
    PALMAS_GPADC_AUTO_1_IRQ,
    PALMAS_GPADC_EOC_SW_IRQ,
    PALMAS_GPADC_EOC_RT_IRQ,
    PALMAS_ID_OTG_IRQ,
    PALMAS_ID_IRQ,
    PALMAS_VBUS_OTG_IRQ,
    PALMAS_VBUS_IRQ,
// INT4 registers
    PALMAS_GPIO_0_IRQ,
    PALMAS_GPIO_1_IRQ,
    PALMAS_GPIO_2_IRQ,
    PALMAS_GPIO_3_IRQ,
    PALMAS_GPIO_4_IRQ,
    PALMAS_GPIO_5_IRQ,
    PALMAS_GPIO_6_IRQ,
    PALMAS_GPIO_7_IRQ,
// Total Number IRQs
    PALMAS_NUM_IRQ,
}

// Palmas GPADC Channels
// Palmas GPADC Channel0 Current Source
// Palmas GPADC Channel3 Current Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_pmic {
    pub palmas: *mut palmas,
    pub dev: *mut device,
    pub desc: [regulator_desc; PALMAS_NUM_REGS],
    pub mutex: mutex,
    pub smps123: c_int,
    pub smps457: c_int,
    pub smps12: c_int,
    pub range: [c_int; PALMAS_REG_SMPS10_OUT1],
    pub ramp_delay: [c_uint; PALMAS_REG_SMPS10_OUT1],
    pub current_reg_mode: [c_uint; PALMAS_REG_SMPS10_OUT1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_resource {
    pub palmas: *mut palmas,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct palmas_usb {
    pub palmas: *mut palmas,
    pub dev: *mut device,
    pub edev: *mut extcon_dev,
    pub id_otg_irq: c_int,
    pub id_irq: c_int,
    pub vbus_otg_irq: c_int,
    pub vbus_irq: c_int,
    pub gpio_id_irq: c_int,
    pub gpio_vbus_irq: c_int,
    pub id_gpiod: *mut gpio_desc,
    pub vbus_gpiod: *mut gpio_desc,
    pub sw_debounce_jiffies: c_ulong,
    pub wq_detectid: delayed_work,
    pub linkstat: palmas_usb_state,
    pub wakeup: c_int,
    pub enable_vbus_detection: bool,
    pub enable_id_detection: bool,
    pub enable_gpio_id_detection: bool,
    pub enable_gpio_vbus_detection: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_irq_events {
// Wakeup events from INT3
    PALMAS_USB_ID_WAKEPUP,
    PALMAS_USB_VBUS_WAKEUP,

// ID_OTG_EVENTS
    PALMAS_USB_ID_GND,
    N_PALMAS_USB_ID_GND,
    PALMAS_USB_ID_C,
    N_PALMAS_USB_ID_C,
    PALMAS_USB_ID_B,
    N_PALMAS_USB_ID_B,
    PALMAS_USB_ID_A,
    N_PALMAS_USB_ID_A,
    PALMAS_USB_ID_FLOAT,
    N_PALMAS_USB_ID_FLOAT,

// VBUS_OTG_EVENTS
    PALMAS_USB_VB_SESS_END,
    N_PALMAS_USB_VB_SESS_END,
    PALMAS_USB_VB_SESS_VLD,
    N_PALMAS_USB_VB_SESS_VLD,
    PALMAS_USB_VA_SESS_VLD,
    N_PALMAS_USB_VA_SESS_VLD,
    PALMAS_USB_VA_VBUS_VLD,
    N_PALMAS_USB_VA_VBUS_VLD,
    PALMAS_USB_VADP_SNS,
    N_PALMAS_USB_VADP_SNS,
    PALMAS_USB_VADP_PRB,
    N_PALMAS_USB_VADP_PRB,
    PALMAS_USB_VOTG_SESS_VLD,
    N_PALMAS_USB_VOTG_SESS_VLD,
}

// defines so we can store the mux settings

// helper macro to get correct slave number

// Base addresses of IP blocks in Palmas
pub const PALMAS_SMPS_DVS_BASE: c_uint = 0x020;
pub const PALMAS_RTC_BASE: c_uint = 0x100;
pub const PALMAS_VALIDITY_BASE: c_uint = 0x118;
pub const PALMAS_SMPS_BASE: c_uint = 0x120;
pub const PALMAS_LDO_BASE: c_uint = 0x150;
pub const PALMAS_DVFS_BASE: c_uint = 0x180;
pub const PALMAS_PMU_CONTROL_BASE: c_uint = 0x1A0;
pub const PALMAS_RESOURCE_BASE: c_uint = 0x1D4;
pub const PALMAS_PU_PD_OD_BASE: c_uint = 0x1F0;
pub const PALMAS_LED_BASE: c_uint = 0x200;
pub const PALMAS_INTERRUPT_BASE: c_uint = 0x210;
pub const PALMAS_USB_OTG_BASE: c_uint = 0x250;
pub const PALMAS_VIBRATOR_BASE: c_uint = 0x270;
pub const PALMAS_GPIO_BASE: c_uint = 0x280;
pub const PALMAS_USB_BASE: c_uint = 0x290;
pub const PALMAS_GPADC_BASE: c_uint = 0x2C0;
pub const PALMAS_TRIM_GPADC_BASE: c_uint = 0x3CD;
// Registers for function RTC
pub const PALMAS_SECONDS_REG: c_uint = 0x00;
pub const PALMAS_MINUTES_REG: c_uint = 0x01;
pub const PALMAS_HOURS_REG: c_uint = 0x02;
pub const PALMAS_DAYS_REG: c_uint = 0x03;
pub const PALMAS_MONTHS_REG: c_uint = 0x04;
pub const PALMAS_YEARS_REG: c_uint = 0x05;
pub const PALMAS_WEEKS_REG: c_uint = 0x06;
pub const PALMAS_ALARM_SECONDS_REG: c_uint = 0x08;
pub const PALMAS_ALARM_MINUTES_REG: c_uint = 0x09;
pub const PALMAS_ALARM_HOURS_REG: c_uint = 0x0A;
pub const PALMAS_ALARM_DAYS_REG: c_uint = 0x0B;
pub const PALMAS_ALARM_MONTHS_REG: c_uint = 0x0C;
pub const PALMAS_ALARM_YEARS_REG: c_uint = 0x0D;
pub const PALMAS_RTC_CTRL_REG: c_uint = 0x10;
pub const PALMAS_RTC_STATUS_REG: c_uint = 0x11;
pub const PALMAS_RTC_INTERRUPTS_REG: c_uint = 0x12;
pub const PALMAS_RTC_COMP_LSB_REG: c_uint = 0x13;
pub const PALMAS_RTC_COMP_MSB_REG: c_uint = 0x14;
pub const PALMAS_RTC_RES_PROG_REG: c_uint = 0x15;
pub const PALMAS_RTC_RESET_STATUS_REG: c_uint = 0x16;
// Bit definitions for SECONDS_REG
pub const PALMAS_SECONDS_REG_SEC1_MASK: c_uint = 0x70;
pub const PALMAS_SECONDS_REG_SEC1_SHIFT: c_uint = 0x04;
pub const PALMAS_SECONDS_REG_SEC0_MASK: c_uint = 0x0F;
pub const PALMAS_SECONDS_REG_SEC0_SHIFT: c_uint = 0x00;
// Bit definitions for MINUTES_REG
pub const PALMAS_MINUTES_REG_MIN1_MASK: c_uint = 0x70;
pub const PALMAS_MINUTES_REG_MIN1_SHIFT: c_uint = 0x04;
pub const PALMAS_MINUTES_REG_MIN0_MASK: c_uint = 0x0F;
pub const PALMAS_MINUTES_REG_MIN0_SHIFT: c_uint = 0x00;
// Bit definitions for HOURS_REG
pub const PALMAS_HOURS_REG_PM_NAM: c_uint = 0x80;
pub const PALMAS_HOURS_REG_PM_NAM_SHIFT: c_uint = 0x07;
pub const PALMAS_HOURS_REG_HOUR1_MASK: c_uint = 0x30;
pub const PALMAS_HOURS_REG_HOUR1_SHIFT: c_uint = 0x04;
pub const PALMAS_HOURS_REG_HOUR0_MASK: c_uint = 0x0F;
pub const PALMAS_HOURS_REG_HOUR0_SHIFT: c_uint = 0x00;
// Bit definitions for DAYS_REG
pub const PALMAS_DAYS_REG_DAY1_MASK: c_uint = 0x30;
pub const PALMAS_DAYS_REG_DAY1_SHIFT: c_uint = 0x04;
pub const PALMAS_DAYS_REG_DAY0_MASK: c_uint = 0x0F;
pub const PALMAS_DAYS_REG_DAY0_SHIFT: c_uint = 0x00;
// Bit definitions for MONTHS_REG
pub const PALMAS_MONTHS_REG_MONTH1: c_uint = 0x10;
pub const PALMAS_MONTHS_REG_MONTH1_SHIFT: c_uint = 0x04;
pub const PALMAS_MONTHS_REG_MONTH0_MASK: c_uint = 0x0F;
pub const PALMAS_MONTHS_REG_MONTH0_SHIFT: c_uint = 0x00;
// Bit definitions for YEARS_REG
pub const PALMAS_YEARS_REG_YEAR1_MASK: c_uint = 0xf0;
pub const PALMAS_YEARS_REG_YEAR1_SHIFT: c_uint = 0x04;
pub const PALMAS_YEARS_REG_YEAR0_MASK: c_uint = 0x0F;
pub const PALMAS_YEARS_REG_YEAR0_SHIFT: c_uint = 0x00;
// Bit definitions for WEEKS_REG
pub const PALMAS_WEEKS_REG_WEEK_MASK: c_uint = 0x07;
pub const PALMAS_WEEKS_REG_WEEK_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_SECONDS_REG
pub const PALMAS_ALARM_SECONDS_REG_ALARM_SEC1_MASK: c_uint = 0x70;
pub const PALMAS_ALARM_SECONDS_REG_ALARM_SEC1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_SECONDS_REG_ALARM_SEC0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_SECONDS_REG_ALARM_SEC0_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_MINUTES_REG
pub const PALMAS_ALARM_MINUTES_REG_ALARM_MIN1_MASK: c_uint = 0x70;
pub const PALMAS_ALARM_MINUTES_REG_ALARM_MIN1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_MINUTES_REG_ALARM_MIN0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_MINUTES_REG_ALARM_MIN0_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_HOURS_REG
pub const PALMAS_ALARM_HOURS_REG_ALARM_PM_NAM: c_uint = 0x80;
pub const PALMAS_ALARM_HOURS_REG_ALARM_PM_NAM_SHIFT: c_uint = 0x07;
pub const PALMAS_ALARM_HOURS_REG_ALARM_HOUR1_MASK: c_uint = 0x30;
pub const PALMAS_ALARM_HOURS_REG_ALARM_HOUR1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_HOURS_REG_ALARM_HOUR0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_HOURS_REG_ALARM_HOUR0_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_DAYS_REG
pub const PALMAS_ALARM_DAYS_REG_ALARM_DAY1_MASK: c_uint = 0x30;
pub const PALMAS_ALARM_DAYS_REG_ALARM_DAY1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_DAYS_REG_ALARM_DAY0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_DAYS_REG_ALARM_DAY0_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_MONTHS_REG
pub const PALMAS_ALARM_MONTHS_REG_ALARM_MONTH1: c_uint = 0x10;
pub const PALMAS_ALARM_MONTHS_REG_ALARM_MONTH1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_MONTHS_REG_ALARM_MONTH0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_MONTHS_REG_ALARM_MONTH0_SHIFT: c_uint = 0x00;
// Bit definitions for ALARM_YEARS_REG
pub const PALMAS_ALARM_YEARS_REG_ALARM_YEAR1_MASK: c_uint = 0xf0;
pub const PALMAS_ALARM_YEARS_REG_ALARM_YEAR1_SHIFT: c_uint = 0x04;
pub const PALMAS_ALARM_YEARS_REG_ALARM_YEAR0_MASK: c_uint = 0x0F;
pub const PALMAS_ALARM_YEARS_REG_ALARM_YEAR0_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_CTRL_REG
pub const PALMAS_RTC_CTRL_REG_RTC_V_OPT: c_uint = 0x80;
pub const PALMAS_RTC_CTRL_REG_RTC_V_OPT_SHIFT: c_uint = 0x07;
pub const PALMAS_RTC_CTRL_REG_GET_TIME: c_uint = 0x40;
pub const PALMAS_RTC_CTRL_REG_GET_TIME_SHIFT: c_uint = 0x06;
pub const PALMAS_RTC_CTRL_REG_SET_32_COUNTER: c_uint = 0x20;
pub const PALMAS_RTC_CTRL_REG_SET_32_COUNTER_SHIFT: c_uint = 0x05;
pub const PALMAS_RTC_CTRL_REG_TEST_MODE: c_uint = 0x10;
pub const PALMAS_RTC_CTRL_REG_TEST_MODE_SHIFT: c_uint = 0x04;
pub const PALMAS_RTC_CTRL_REG_MODE_12_24: c_uint = 0x08;
pub const PALMAS_RTC_CTRL_REG_MODE_12_24_SHIFT: c_uint = 0x03;
pub const PALMAS_RTC_CTRL_REG_AUTO_COMP: c_uint = 0x04;
pub const PALMAS_RTC_CTRL_REG_AUTO_COMP_SHIFT: c_uint = 0x02;
pub const PALMAS_RTC_CTRL_REG_ROUND_30S: c_uint = 0x02;
pub const PALMAS_RTC_CTRL_REG_ROUND_30S_SHIFT: c_uint = 0x01;
pub const PALMAS_RTC_CTRL_REG_STOP_RTC: c_uint = 0x01;
pub const PALMAS_RTC_CTRL_REG_STOP_RTC_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_STATUS_REG
pub const PALMAS_RTC_STATUS_REG_POWER_UP: c_uint = 0x80;
pub const PALMAS_RTC_STATUS_REG_POWER_UP_SHIFT: c_uint = 0x07;
pub const PALMAS_RTC_STATUS_REG_ALARM: c_uint = 0x40;
pub const PALMAS_RTC_STATUS_REG_ALARM_SHIFT: c_uint = 0x06;
pub const PALMAS_RTC_STATUS_REG_EVENT_1D: c_uint = 0x20;
pub const PALMAS_RTC_STATUS_REG_EVENT_1D_SHIFT: c_uint = 0x05;
pub const PALMAS_RTC_STATUS_REG_EVENT_1H: c_uint = 0x10;
pub const PALMAS_RTC_STATUS_REG_EVENT_1H_SHIFT: c_uint = 0x04;
pub const PALMAS_RTC_STATUS_REG_EVENT_1M: c_uint = 0x08;
pub const PALMAS_RTC_STATUS_REG_EVENT_1M_SHIFT: c_uint = 0x03;
pub const PALMAS_RTC_STATUS_REG_EVENT_1S: c_uint = 0x04;
pub const PALMAS_RTC_STATUS_REG_EVENT_1S_SHIFT: c_uint = 0x02;
pub const PALMAS_RTC_STATUS_REG_RUN: c_uint = 0x02;
pub const PALMAS_RTC_STATUS_REG_RUN_SHIFT: c_uint = 0x01;
// Bit definitions for RTC_INTERRUPTS_REG
pub const PALMAS_RTC_INTERRUPTS_REG_IT_SLEEP_MASK_EN: c_uint = 0x10;
pub const PALMAS_RTC_INTERRUPTS_REG_IT_SLEEP_MASK_EN_SHIFT: c_uint = 0x04;
pub const PALMAS_RTC_INTERRUPTS_REG_IT_ALARM: c_uint = 0x08;
pub const PALMAS_RTC_INTERRUPTS_REG_IT_ALARM_SHIFT: c_uint = 0x03;
pub const PALMAS_RTC_INTERRUPTS_REG_IT_TIMER: c_uint = 0x04;
pub const PALMAS_RTC_INTERRUPTS_REG_IT_TIMER_SHIFT: c_uint = 0x02;
pub const PALMAS_RTC_INTERRUPTS_REG_EVERY_MASK: c_uint = 0x03;
pub const PALMAS_RTC_INTERRUPTS_REG_EVERY_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_COMP_LSB_REG
pub const PALMAS_RTC_COMP_LSB_REG_RTC_COMP_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_RTC_COMP_LSB_REG_RTC_COMP_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_COMP_MSB_REG
pub const PALMAS_RTC_COMP_MSB_REG_RTC_COMP_MSB_MASK: c_uint = 0xFF;
pub const PALMAS_RTC_COMP_MSB_REG_RTC_COMP_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_RES_PROG_REG
pub const PALMAS_RTC_RES_PROG_REG_SW_RES_PROG_MASK: c_uint = 0x3F;
pub const PALMAS_RTC_RES_PROG_REG_SW_RES_PROG_SHIFT: c_uint = 0x00;
// Bit definitions for RTC_RESET_STATUS_REG
pub const PALMAS_RTC_RESET_STATUS_REG_RESET_STATUS: c_uint = 0x01;
pub const PALMAS_RTC_RESET_STATUS_REG_RESET_STATUS_SHIFT: c_uint = 0x00;
// Registers for function BACKUP
pub const PALMAS_BACKUP0: c_uint = 0x00;
pub const PALMAS_BACKUP1: c_uint = 0x01;
pub const PALMAS_BACKUP2: c_uint = 0x02;
pub const PALMAS_BACKUP3: c_uint = 0x03;
pub const PALMAS_BACKUP4: c_uint = 0x04;
pub const PALMAS_BACKUP5: c_uint = 0x05;
pub const PALMAS_BACKUP6: c_uint = 0x06;
pub const PALMAS_BACKUP7: c_uint = 0x07;
// Bit definitions for BACKUP0
pub const PALMAS_BACKUP0_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP0_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP1
pub const PALMAS_BACKUP1_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP1_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP2
pub const PALMAS_BACKUP2_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP2_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP3
pub const PALMAS_BACKUP3_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP3_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP4
pub const PALMAS_BACKUP4_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP4_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP5
pub const PALMAS_BACKUP5_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP5_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP6
pub const PALMAS_BACKUP6_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP6_BACKUP_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP7
pub const PALMAS_BACKUP7_BACKUP_MASK: c_uint = 0xFF;
pub const PALMAS_BACKUP7_BACKUP_SHIFT: c_uint = 0x00;
// Registers for function SMPS
pub const PALMAS_SMPS12_CTRL: c_uint = 0x00;
pub const PALMAS_SMPS12_TSTEP: c_uint = 0x01;
pub const PALMAS_SMPS12_FORCE: c_uint = 0x02;
pub const PALMAS_SMPS12_VOLTAGE: c_uint = 0x03;
pub const PALMAS_SMPS3_CTRL: c_uint = 0x04;
pub const PALMAS_SMPS3_VOLTAGE: c_uint = 0x07;
pub const PALMAS_SMPS45_CTRL: c_uint = 0x08;
pub const PALMAS_SMPS45_TSTEP: c_uint = 0x09;
pub const PALMAS_SMPS45_FORCE: c_uint = 0x0A;
pub const PALMAS_SMPS45_VOLTAGE: c_uint = 0x0B;
pub const PALMAS_SMPS6_CTRL: c_uint = 0x0C;
pub const PALMAS_SMPS6_TSTEP: c_uint = 0x0D;
pub const PALMAS_SMPS6_FORCE: c_uint = 0x0E;
pub const PALMAS_SMPS6_VOLTAGE: c_uint = 0x0F;
pub const PALMAS_SMPS7_CTRL: c_uint = 0x10;
pub const PALMAS_SMPS7_VOLTAGE: c_uint = 0x13;
pub const PALMAS_SMPS8_CTRL: c_uint = 0x14;
pub const PALMAS_SMPS8_TSTEP: c_uint = 0x15;
pub const PALMAS_SMPS8_FORCE: c_uint = 0x16;
pub const PALMAS_SMPS8_VOLTAGE: c_uint = 0x17;
pub const PALMAS_SMPS9_CTRL: c_uint = 0x18;
pub const PALMAS_SMPS9_VOLTAGE: c_uint = 0x1B;
pub const PALMAS_SMPS10_CTRL: c_uint = 0x1C;
pub const PALMAS_SMPS10_STATUS: c_uint = 0x1F;
pub const PALMAS_SMPS_CTRL: c_uint = 0x24;
pub const PALMAS_SMPS_PD_CTRL: c_uint = 0x25;
pub const PALMAS_SMPS_DITHER_EN: c_uint = 0x26;
pub const PALMAS_SMPS_THERMAL_EN: c_uint = 0x27;
pub const PALMAS_SMPS_THERMAL_STATUS: c_uint = 0x28;
pub const PALMAS_SMPS_SHORT_STATUS: c_uint = 0x29;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN: c_uint = 0x2A;
pub const PALMAS_SMPS_POWERGOOD_MASK1: c_uint = 0x2B;
pub const PALMAS_SMPS_POWERGOOD_MASK2: c_uint = 0x2C;
// Bit definitions for SMPS12_CTRL
pub const PALMAS_SMPS12_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS12_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS12_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const PALMAS_SMPS12_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS12_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS12_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS12_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS12_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS12_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS12_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS12_TSTEP
pub const PALMAS_SMPS12_TSTEP_TSTEP_MASK: c_uint = 0x03;
pub const PALMAS_SMPS12_TSTEP_TSTEP_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS12_FORCE
pub const PALMAS_SMPS12_FORCE_CMD: c_uint = 0x80;
pub const PALMAS_SMPS12_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS12_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS12_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS12_VOLTAGE
pub const PALMAS_SMPS12_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS12_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS12_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS12_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS3_CTRL
pub const PALMAS_SMPS3_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS3_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS3_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS3_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS3_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS3_VOLTAGE
pub const PALMAS_SMPS3_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS3_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS3_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS3_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS45_CTRL
pub const PALMAS_SMPS45_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS45_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS45_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const PALMAS_SMPS45_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS45_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS45_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS45_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS45_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS45_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS45_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS45_TSTEP
pub const PALMAS_SMPS45_TSTEP_TSTEP_MASK: c_uint = 0x03;
pub const PALMAS_SMPS45_TSTEP_TSTEP_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS45_FORCE
pub const PALMAS_SMPS45_FORCE_CMD: c_uint = 0x80;
pub const PALMAS_SMPS45_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS45_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS45_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS45_VOLTAGE
pub const PALMAS_SMPS45_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS45_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS45_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS45_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS6_CTRL
pub const PALMAS_SMPS6_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS6_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS6_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const PALMAS_SMPS6_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS6_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS6_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS6_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS6_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS6_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS6_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS6_TSTEP
pub const PALMAS_SMPS6_TSTEP_TSTEP_MASK: c_uint = 0x03;
pub const PALMAS_SMPS6_TSTEP_TSTEP_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS6_FORCE
pub const PALMAS_SMPS6_FORCE_CMD: c_uint = 0x80;
pub const PALMAS_SMPS6_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS6_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS6_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS6_VOLTAGE
pub const PALMAS_SMPS6_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS6_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS6_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS6_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS7_CTRL
pub const PALMAS_SMPS7_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS7_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS7_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS7_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS7_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS7_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS7_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS7_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS7_VOLTAGE
pub const PALMAS_SMPS7_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS7_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS7_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS7_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS8_CTRL
pub const PALMAS_SMPS8_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS8_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS8_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const PALMAS_SMPS8_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS8_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS8_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS8_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS8_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS8_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS8_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS8_TSTEP
pub const PALMAS_SMPS8_TSTEP_TSTEP_MASK: c_uint = 0x03;
pub const PALMAS_SMPS8_TSTEP_TSTEP_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS8_FORCE
pub const PALMAS_SMPS8_FORCE_CMD: c_uint = 0x80;
pub const PALMAS_SMPS8_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS8_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS8_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS8_VOLTAGE
pub const PALMAS_SMPS8_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS8_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS8_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS8_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS9_CTRL
pub const PALMAS_SMPS9_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_SMPS9_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS9_CTRL_STATUS_MASK: c_uint = 0x30;
pub const PALMAS_SMPS9_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS9_CTRL_MODE_SLEEP_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS9_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS9_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const PALMAS_SMPS9_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS9_VOLTAGE
pub const PALMAS_SMPS9_VOLTAGE_RANGE: c_uint = 0x80;
pub const PALMAS_SMPS9_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS9_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const PALMAS_SMPS9_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS10_CTRL
pub const PALMAS_SMPS10_CTRL_MODE_SLEEP_MASK: c_uint = 0xf0;
pub const PALMAS_SMPS10_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS10_CTRL_MODE_ACTIVE_MASK: c_uint = 0x0F;
pub const PALMAS_SMPS10_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS10_STATUS
pub const PALMAS_SMPS10_STATUS_STATUS_MASK: c_uint = 0x0F;
pub const PALMAS_SMPS10_STATUS_STATUS_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_CTRL
pub const PALMAS_SMPS_CTRL_SMPS45_SMPS457_EN: c_uint = 0x20;
pub const PALMAS_SMPS_CTRL_SMPS45_SMPS457_EN_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_CTRL_SMPS12_SMPS123_EN: c_uint = 0x10;
pub const PALMAS_SMPS_CTRL_SMPS12_SMPS123_EN_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS_CTRL_SMPS45_PHASE_CTRL_MASK: c_uint = 0x0c;
pub const PALMAS_SMPS_CTRL_SMPS45_PHASE_CTRL_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_CTRL_SMPS123_PHASE_CTRL_MASK: c_uint = 0x03;
pub const PALMAS_SMPS_CTRL_SMPS123_PHASE_CTRL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_PD_CTRL
pub const PALMAS_SMPS_PD_CTRL_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_PD_CTRL_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_PD_CTRL_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_PD_CTRL_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_PD_CTRL_SMPS7: c_uint = 0x10;
pub const PALMAS_SMPS_PD_CTRL_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS_PD_CTRL_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_PD_CTRL_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_PD_CTRL_SMPS45: c_uint = 0x04;
pub const PALMAS_SMPS_PD_CTRL_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_PD_CTRL_SMPS3: c_uint = 0x02;
pub const PALMAS_SMPS_PD_CTRL_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_SMPS_PD_CTRL_SMPS12: c_uint = 0x01;
pub const PALMAS_SMPS_PD_CTRL_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_THERMAL_EN
pub const PALMAS_SMPS_THERMAL_EN_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_THERMAL_EN_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_THERMAL_EN_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_THERMAL_EN_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_THERMAL_EN_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_THERMAL_EN_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_THERMAL_EN_SMPS457: c_uint = 0x04;
pub const PALMAS_SMPS_THERMAL_EN_SMPS457_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_THERMAL_EN_SMPS123: c_uint = 0x01;
pub const PALMAS_SMPS_THERMAL_EN_SMPS123_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_THERMAL_STATUS
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS457: c_uint = 0x04;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS457_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS123: c_uint = 0x01;
pub const PALMAS_SMPS_THERMAL_STATUS_SMPS123_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_SHORT_STATUS
pub const PALMAS_SMPS_SHORT_STATUS_SMPS10: c_uint = 0x80;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS10_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS7: c_uint = 0x10;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS45: c_uint = 0x04;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS3: c_uint = 0x02;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS12: c_uint = 0x01;
pub const PALMAS_SMPS_SHORT_STATUS_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_NEGATIVE_CURRENT_LIMIT_EN
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS7: c_uint = 0x10;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS45: c_uint = 0x04;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS3: c_uint = 0x02;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS12: c_uint = 0x01;
pub const PALMAS_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_POWERGOOD_MASK1
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS10: c_uint = 0x80;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS10_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS9: c_uint = 0x40;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS8: c_uint = 0x20;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS7: c_uint = 0x10;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS6: c_uint = 0x08;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS45: c_uint = 0x04;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS3: c_uint = 0x02;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS12: c_uint = 0x01;
pub const PALMAS_SMPS_POWERGOOD_MASK1_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_POWERGOOD_MASK2
pub const PALMAS_SMPS_POWERGOOD_MASK2_POWERGOOD_TYPE_SELECT: c_uint = 0x80;
pub const PALMAS_SMPS_POWERGOOD_MASK2_POWERGOOD_TYPE_SELECT_SHIFT: c_uint = 0x07;
pub const PALMAS_SMPS_POWERGOOD_MASK2_GPIO_7: c_uint = 0x04;
pub const PALMAS_SMPS_POWERGOOD_MASK2_GPIO_7_SHIFT: c_uint = 0x02;
pub const PALMAS_SMPS_POWERGOOD_MASK2_VBUS: c_uint = 0x02;
pub const PALMAS_SMPS_POWERGOOD_MASK2_VBUS_SHIFT: c_uint = 0x01;
pub const PALMAS_SMPS_POWERGOOD_MASK2_ACOK: c_uint = 0x01;
pub const PALMAS_SMPS_POWERGOOD_MASK2_ACOK_SHIFT: c_uint = 0x00;
// Registers for function LDO
pub const PALMAS_LDO1_CTRL: c_uint = 0x00;
pub const PALMAS_LDO1_VOLTAGE: c_uint = 0x01;
pub const PALMAS_LDO2_CTRL: c_uint = 0x02;
pub const PALMAS_LDO2_VOLTAGE: c_uint = 0x03;
pub const PALMAS_LDO3_CTRL: c_uint = 0x04;
pub const PALMAS_LDO3_VOLTAGE: c_uint = 0x05;
pub const PALMAS_LDO4_CTRL: c_uint = 0x06;
pub const PALMAS_LDO4_VOLTAGE: c_uint = 0x07;
pub const PALMAS_LDO5_CTRL: c_uint = 0x08;
pub const PALMAS_LDO5_VOLTAGE: c_uint = 0x09;
pub const PALMAS_LDO6_CTRL: c_uint = 0x0A;
pub const PALMAS_LDO6_VOLTAGE: c_uint = 0x0B;
pub const PALMAS_LDO7_CTRL: c_uint = 0x0C;
pub const PALMAS_LDO7_VOLTAGE: c_uint = 0x0D;
pub const PALMAS_LDO8_CTRL: c_uint = 0x0E;
pub const PALMAS_LDO8_VOLTAGE: c_uint = 0x0F;
pub const PALMAS_LDO9_CTRL: c_uint = 0x10;
pub const PALMAS_LDO9_VOLTAGE: c_uint = 0x11;
pub const PALMAS_LDOLN_CTRL: c_uint = 0x12;
pub const PALMAS_LDOLN_VOLTAGE: c_uint = 0x13;
pub const PALMAS_LDOUSB_CTRL: c_uint = 0x14;
pub const PALMAS_LDOUSB_VOLTAGE: c_uint = 0x15;
pub const PALMAS_LDO_CTRL: c_uint = 0x1A;
pub const PALMAS_LDO_PD_CTRL1: c_uint = 0x1B;
pub const PALMAS_LDO_PD_CTRL2: c_uint = 0x1C;
pub const PALMAS_LDO_SHORT_STATUS1: c_uint = 0x1D;
pub const PALMAS_LDO_SHORT_STATUS2: c_uint = 0x1E;
// Bit definitions for LDO1_CTRL
pub const PALMAS_LDO1_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO1_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO1_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO1_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO1_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO1_VOLTAGE
pub const PALMAS_LDO1_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO1_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO2_CTRL
pub const PALMAS_LDO2_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO2_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO2_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO2_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO2_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO2_VOLTAGE
pub const PALMAS_LDO2_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO2_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO3_CTRL
pub const PALMAS_LDO3_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO3_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO3_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO3_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO3_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO3_VOLTAGE
pub const PALMAS_LDO3_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO3_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO4_CTRL
pub const PALMAS_LDO4_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO4_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO4_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO4_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO4_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO4_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO4_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO4_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO4_VOLTAGE
pub const PALMAS_LDO4_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO4_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO5_CTRL
pub const PALMAS_LDO5_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO5_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO5_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO5_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO5_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO5_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO5_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO5_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO5_VOLTAGE
pub const PALMAS_LDO5_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO5_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO6_CTRL
pub const PALMAS_LDO6_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO6_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO6_CTRL_LDO_VIB_EN: c_uint = 0x40;
pub const PALMAS_LDO6_CTRL_LDO_VIB_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_LDO6_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO6_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO6_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO6_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO6_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO6_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO6_VOLTAGE
pub const PALMAS_LDO6_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO6_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO7_CTRL
pub const PALMAS_LDO7_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO7_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO7_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO7_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO7_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO7_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO7_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO7_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO7_VOLTAGE
pub const PALMAS_LDO7_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO7_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO8_CTRL
pub const PALMAS_LDO8_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO8_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO8_CTRL_LDO_TRACKING_EN: c_uint = 0x40;
pub const PALMAS_LDO8_CTRL_LDO_TRACKING_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_LDO8_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO8_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO8_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO8_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO8_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO8_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO8_VOLTAGE
pub const PALMAS_LDO8_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO8_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO9_CTRL
pub const PALMAS_LDO9_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDO9_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO9_CTRL_LDO_BYPASS_EN: c_uint = 0x40;
pub const PALMAS_LDO9_CTRL_LDO_BYPASS_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_LDO9_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDO9_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO9_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDO9_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO9_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDO9_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO9_VOLTAGE
pub const PALMAS_LDO9_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDO9_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDOLN_CTRL
pub const PALMAS_LDOLN_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDOLN_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDOLN_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDOLN_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDOLN_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDOLN_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDOLN_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDOLN_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDOLN_VOLTAGE
pub const PALMAS_LDOLN_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDOLN_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDOUSB_CTRL
pub const PALMAS_LDOUSB_CTRL_WR_S: c_uint = 0x80;
pub const PALMAS_LDOUSB_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const PALMAS_LDOUSB_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_LDOUSB_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_LDOUSB_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_LDOUSB_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_LDOUSB_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_LDOUSB_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDOUSB_VOLTAGE
pub const PALMAS_LDOUSB_VOLTAGE_VSEL_MASK: c_uint = 0x3F;
pub const PALMAS_LDOUSB_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_CTRL
pub const PALMAS_LDO_CTRL_LDOUSB_ON_VBUS_VSYS: c_uint = 0x01;
pub const PALMAS_LDO_CTRL_LDOUSB_ON_VBUS_VSYS_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_PD_CTRL1
pub const PALMAS_LDO_PD_CTRL1_LDO8: c_uint = 0x80;
pub const PALMAS_LDO_PD_CTRL1_LDO8_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO_PD_CTRL1_LDO7: c_uint = 0x40;
pub const PALMAS_LDO_PD_CTRL1_LDO7_SHIFT: c_uint = 0x06;
pub const PALMAS_LDO_PD_CTRL1_LDO6: c_uint = 0x20;
pub const PALMAS_LDO_PD_CTRL1_LDO6_SHIFT: c_uint = 0x05;
pub const PALMAS_LDO_PD_CTRL1_LDO5: c_uint = 0x10;
pub const PALMAS_LDO_PD_CTRL1_LDO5_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO_PD_CTRL1_LDO4: c_uint = 0x08;
pub const PALMAS_LDO_PD_CTRL1_LDO4_SHIFT: c_uint = 0x03;
pub const PALMAS_LDO_PD_CTRL1_LDO3: c_uint = 0x04;
pub const PALMAS_LDO_PD_CTRL1_LDO3_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO_PD_CTRL1_LDO2: c_uint = 0x02;
pub const PALMAS_LDO_PD_CTRL1_LDO2_SHIFT: c_uint = 0x01;
pub const PALMAS_LDO_PD_CTRL1_LDO1: c_uint = 0x01;
pub const PALMAS_LDO_PD_CTRL1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_PD_CTRL2
pub const PALMAS_LDO_PD_CTRL2_LDOUSB: c_uint = 0x04;
pub const PALMAS_LDO_PD_CTRL2_LDOUSB_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO_PD_CTRL2_LDOLN: c_uint = 0x02;
pub const PALMAS_LDO_PD_CTRL2_LDOLN_SHIFT: c_uint = 0x01;
pub const PALMAS_LDO_PD_CTRL2_LDO9: c_uint = 0x01;
pub const PALMAS_LDO_PD_CTRL2_LDO9_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_SHORT_STATUS1
pub const PALMAS_LDO_SHORT_STATUS1_LDO8: c_uint = 0x80;
pub const PALMAS_LDO_SHORT_STATUS1_LDO8_SHIFT: c_uint = 0x07;
pub const PALMAS_LDO_SHORT_STATUS1_LDO7: c_uint = 0x40;
pub const PALMAS_LDO_SHORT_STATUS1_LDO7_SHIFT: c_uint = 0x06;
pub const PALMAS_LDO_SHORT_STATUS1_LDO6: c_uint = 0x20;
pub const PALMAS_LDO_SHORT_STATUS1_LDO6_SHIFT: c_uint = 0x05;
pub const PALMAS_LDO_SHORT_STATUS1_LDO5: c_uint = 0x10;
pub const PALMAS_LDO_SHORT_STATUS1_LDO5_SHIFT: c_uint = 0x04;
pub const PALMAS_LDO_SHORT_STATUS1_LDO4: c_uint = 0x08;
pub const PALMAS_LDO_SHORT_STATUS1_LDO4_SHIFT: c_uint = 0x03;
pub const PALMAS_LDO_SHORT_STATUS1_LDO3: c_uint = 0x04;
pub const PALMAS_LDO_SHORT_STATUS1_LDO3_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO_SHORT_STATUS1_LDO2: c_uint = 0x02;
pub const PALMAS_LDO_SHORT_STATUS1_LDO2_SHIFT: c_uint = 0x01;
pub const PALMAS_LDO_SHORT_STATUS1_LDO1: c_uint = 0x01;
pub const PALMAS_LDO_SHORT_STATUS1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_SHORT_STATUS2
pub const PALMAS_LDO_SHORT_STATUS2_LDOVANA: c_uint = 0x08;
pub const PALMAS_LDO_SHORT_STATUS2_LDOVANA_SHIFT: c_uint = 0x03;
pub const PALMAS_LDO_SHORT_STATUS2_LDOUSB: c_uint = 0x04;
pub const PALMAS_LDO_SHORT_STATUS2_LDOUSB_SHIFT: c_uint = 0x02;
pub const PALMAS_LDO_SHORT_STATUS2_LDOLN: c_uint = 0x02;
pub const PALMAS_LDO_SHORT_STATUS2_LDOLN_SHIFT: c_uint = 0x01;
pub const PALMAS_LDO_SHORT_STATUS2_LDO9: c_uint = 0x01;
pub const PALMAS_LDO_SHORT_STATUS2_LDO9_SHIFT: c_uint = 0x00;
// Registers for function PMU_CONTROL
pub const PALMAS_DEV_CTRL: c_uint = 0x00;
pub const PALMAS_POWER_CTRL: c_uint = 0x01;
pub const PALMAS_VSYS_LO: c_uint = 0x02;
pub const PALMAS_VSYS_MON: c_uint = 0x03;
pub const PALMAS_VBAT_MON: c_uint = 0x04;
pub const PALMAS_WATCHDOG: c_uint = 0x05;
pub const PALMAS_BOOT_STATUS: c_uint = 0x06;
pub const PALMAS_BATTERY_BOUNCE: c_uint = 0x07;
pub const PALMAS_BACKUP_BATTERY_CTRL: c_uint = 0x08;
pub const PALMAS_LONG_PRESS_KEY: c_uint = 0x09;
pub const PALMAS_OSC_THERM_CTRL: c_uint = 0x0A;
pub const PALMAS_BATDEBOUNCING: c_uint = 0x0B;
pub const PALMAS_SWOFF_HWRST: c_uint = 0x0F;
pub const PALMAS_SWOFF_COLDRST: c_uint = 0x10;
pub const PALMAS_SWOFF_STATUS: c_uint = 0x11;
pub const PALMAS_PMU_CONFIG: c_uint = 0x12;
pub const PALMAS_SPARE: c_uint = 0x14;
pub const PALMAS_PMU_SECONDARY_INT: c_uint = 0x15;
pub const PALMAS_SW_REVISION: c_uint = 0x17;
pub const PALMAS_EXT_CHRG_CTRL: c_uint = 0x18;
pub const PALMAS_PMU_SECONDARY_INT2: c_uint = 0x19;
// Bit definitions for DEV_CTRL
pub const PALMAS_DEV_CTRL_DEV_STATUS_MASK: c_uint = 0x0c;
pub const PALMAS_DEV_CTRL_DEV_STATUS_SHIFT: c_uint = 0x02;
pub const PALMAS_DEV_CTRL_SW_RST: c_uint = 0x02;
pub const PALMAS_DEV_CTRL_SW_RST_SHIFT: c_uint = 0x01;
pub const PALMAS_DEV_CTRL_DEV_ON: c_uint = 0x01;
pub const PALMAS_DEV_CTRL_DEV_ON_SHIFT: c_uint = 0x00;
// Bit definitions for POWER_CTRL
pub const PALMAS_POWER_CTRL_ENABLE2_MASK: c_uint = 0x04;
pub const PALMAS_POWER_CTRL_ENABLE2_MASK_SHIFT: c_uint = 0x02;
pub const PALMAS_POWER_CTRL_ENABLE1_MASK: c_uint = 0x02;
pub const PALMAS_POWER_CTRL_ENABLE1_MASK_SHIFT: c_uint = 0x01;
pub const PALMAS_POWER_CTRL_NSLEEP_MASK: c_uint = 0x01;
pub const PALMAS_POWER_CTRL_NSLEEP_MASK_SHIFT: c_uint = 0x00;
// Bit definitions for VSYS_LO
pub const PALMAS_VSYS_LO_THRESHOLD_MASK: c_uint = 0x1F;
pub const PALMAS_VSYS_LO_THRESHOLD_SHIFT: c_uint = 0x00;
// Bit definitions for VSYS_MON
pub const PALMAS_VSYS_MON_ENABLE: c_uint = 0x80;
pub const PALMAS_VSYS_MON_ENABLE_SHIFT: c_uint = 0x07;
pub const PALMAS_VSYS_MON_THRESHOLD_MASK: c_uint = 0x3F;
pub const PALMAS_VSYS_MON_THRESHOLD_SHIFT: c_uint = 0x00;
// Bit definitions for VBAT_MON
pub const PALMAS_VBAT_MON_ENABLE: c_uint = 0x80;
pub const PALMAS_VBAT_MON_ENABLE_SHIFT: c_uint = 0x07;
pub const PALMAS_VBAT_MON_THRESHOLD_MASK: c_uint = 0x3F;
pub const PALMAS_VBAT_MON_THRESHOLD_SHIFT: c_uint = 0x00;
// Bit definitions for WATCHDOG
pub const PALMAS_WATCHDOG_LOCK: c_uint = 0x20;
pub const PALMAS_WATCHDOG_LOCK_SHIFT: c_uint = 0x05;
pub const PALMAS_WATCHDOG_ENABLE: c_uint = 0x10;
pub const PALMAS_WATCHDOG_ENABLE_SHIFT: c_uint = 0x04;
pub const PALMAS_WATCHDOG_MODE: c_uint = 0x08;
pub const PALMAS_WATCHDOG_MODE_SHIFT: c_uint = 0x03;
pub const PALMAS_WATCHDOG_TIMER_MASK: c_uint = 0x07;
pub const PALMAS_WATCHDOG_TIMER_SHIFT: c_uint = 0x00;
// Bit definitions for BOOT_STATUS
pub const PALMAS_BOOT_STATUS_BOOT1: c_uint = 0x02;
pub const PALMAS_BOOT_STATUS_BOOT1_SHIFT: c_uint = 0x01;
pub const PALMAS_BOOT_STATUS_BOOT0: c_uint = 0x01;
pub const PALMAS_BOOT_STATUS_BOOT0_SHIFT: c_uint = 0x00;
// Bit definitions for BATTERY_BOUNCE
pub const PALMAS_BATTERY_BOUNCE_BB_DELAY_MASK: c_uint = 0x3F;
pub const PALMAS_BATTERY_BOUNCE_BB_DELAY_SHIFT: c_uint = 0x00;
// Bit definitions for BACKUP_BATTERY_CTRL
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_18_15: c_uint = 0x80;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_18_15_SHIFT: c_uint = 0x07;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_EN_SLP: c_uint = 0x40;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_EN_SLP_SHIFT: c_uint = 0x06;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_EN_OFF: c_uint = 0x20;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_EN_OFF_SHIFT: c_uint = 0x05;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_PWEN: c_uint = 0x10;
pub const PALMAS_BACKUP_BATTERY_CTRL_VRTC_PWEN_SHIFT: c_uint = 0x04;
pub const PALMAS_BACKUP_BATTERY_CTRL_BBS_BBC_LOW_ICHRG: c_uint = 0x08;
pub const PALMAS_BACKUP_BATTERY_CTRL_BBS_BBC_LOW_ICHRG_SHIFT: c_uint = 0x03;
pub const PALMAS_BACKUP_BATTERY_CTRL_BB_SEL_MASK: c_uint = 0x06;
pub const PALMAS_BACKUP_BATTERY_CTRL_BB_SEL_SHIFT: c_uint = 0x01;
pub const PALMAS_BACKUP_BATTERY_CTRL_BB_CHG_EN: c_uint = 0x01;
pub const PALMAS_BACKUP_BATTERY_CTRL_BB_CHG_EN_SHIFT: c_uint = 0x00;
// Bit definitions for LONG_PRESS_KEY
pub const PALMAS_LONG_PRESS_KEY_LPK_LOCK: c_uint = 0x80;
pub const PALMAS_LONG_PRESS_KEY_LPK_LOCK_SHIFT: c_uint = 0x07;
pub const PALMAS_LONG_PRESS_KEY_LPK_INT_CLR: c_uint = 0x10;
pub const PALMAS_LONG_PRESS_KEY_LPK_INT_CLR_SHIFT: c_uint = 0x04;
pub const PALMAS_LONG_PRESS_KEY_LPK_TIME_MASK: c_uint = 0x0c;
pub const PALMAS_LONG_PRESS_KEY_LPK_TIME_SHIFT: c_uint = 0x02;
pub const PALMAS_LONG_PRESS_KEY_PWRON_DEBOUNCE_MASK: c_uint = 0x03;
pub const PALMAS_LONG_PRESS_KEY_PWRON_DEBOUNCE_SHIFT: c_uint = 0x00;
// Bit definitions for OSC_THERM_CTRL
pub const PALMAS_OSC_THERM_CTRL_VANA_ON_IN_SLEEP: c_uint = 0x80;
pub const PALMAS_OSC_THERM_CTRL_VANA_ON_IN_SLEEP_SHIFT: c_uint = 0x07;
pub const PALMAS_OSC_THERM_CTRL_INT_MASK_IN_SLEEP: c_uint = 0x40;
pub const PALMAS_OSC_THERM_CTRL_INT_MASK_IN_SLEEP_SHIFT: c_uint = 0x06;
pub const PALMAS_OSC_THERM_CTRL_RC15MHZ_ON_IN_SLEEP: c_uint = 0x20;
pub const PALMAS_OSC_THERM_CTRL_RC15MHZ_ON_IN_SLEEP_SHIFT: c_uint = 0x05;
pub const PALMAS_OSC_THERM_CTRL_THERM_OFF_IN_SLEEP: c_uint = 0x10;
pub const PALMAS_OSC_THERM_CTRL_THERM_OFF_IN_SLEEP_SHIFT: c_uint = 0x04;
pub const PALMAS_OSC_THERM_CTRL_THERM_HD_SEL_MASK: c_uint = 0x0c;
pub const PALMAS_OSC_THERM_CTRL_THERM_HD_SEL_SHIFT: c_uint = 0x02;
pub const PALMAS_OSC_THERM_CTRL_OSC_BYPASS: c_uint = 0x02;
pub const PALMAS_OSC_THERM_CTRL_OSC_BYPASS_SHIFT: c_uint = 0x01;
pub const PALMAS_OSC_THERM_CTRL_OSC_HPMODE: c_uint = 0x01;
pub const PALMAS_OSC_THERM_CTRL_OSC_HPMODE_SHIFT: c_uint = 0x00;
// Bit definitions for BATDEBOUNCING
pub const PALMAS_BATDEBOUNCING_BAT_DEB_BYPASS: c_uint = 0x80;
pub const PALMAS_BATDEBOUNCING_BAT_DEB_BYPASS_SHIFT: c_uint = 0x07;
pub const PALMAS_BATDEBOUNCING_BINS_DEB_MASK: c_uint = 0x78;
pub const PALMAS_BATDEBOUNCING_BINS_DEB_SHIFT: c_uint = 0x03;
pub const PALMAS_BATDEBOUNCING_BEXT_DEB_MASK: c_uint = 0x07;
pub const PALMAS_BATDEBOUNCING_BEXT_DEB_SHIFT: c_uint = 0x00;
// Bit definitions for SWOFF_HWRST
pub const PALMAS_SWOFF_HWRST_PWRON_LPK: c_uint = 0x80;
pub const PALMAS_SWOFF_HWRST_PWRON_LPK_SHIFT: c_uint = 0x07;
pub const PALMAS_SWOFF_HWRST_PWRDOWN: c_uint = 0x40;
pub const PALMAS_SWOFF_HWRST_PWRDOWN_SHIFT: c_uint = 0x06;
pub const PALMAS_SWOFF_HWRST_WTD: c_uint = 0x20;
pub const PALMAS_SWOFF_HWRST_WTD_SHIFT: c_uint = 0x05;
pub const PALMAS_SWOFF_HWRST_TSHUT: c_uint = 0x10;
pub const PALMAS_SWOFF_HWRST_TSHUT_SHIFT: c_uint = 0x04;
pub const PALMAS_SWOFF_HWRST_RESET_IN: c_uint = 0x08;
pub const PALMAS_SWOFF_HWRST_RESET_IN_SHIFT: c_uint = 0x03;
pub const PALMAS_SWOFF_HWRST_SW_RST: c_uint = 0x04;
pub const PALMAS_SWOFF_HWRST_SW_RST_SHIFT: c_uint = 0x02;
pub const PALMAS_SWOFF_HWRST_VSYS_LO: c_uint = 0x02;
pub const PALMAS_SWOFF_HWRST_VSYS_LO_SHIFT: c_uint = 0x01;
pub const PALMAS_SWOFF_HWRST_GPADC_SHUTDOWN: c_uint = 0x01;
pub const PALMAS_SWOFF_HWRST_GPADC_SHUTDOWN_SHIFT: c_uint = 0x00;
// Bit definitions for SWOFF_COLDRST
pub const PALMAS_SWOFF_COLDRST_PWRON_LPK: c_uint = 0x80;
pub const PALMAS_SWOFF_COLDRST_PWRON_LPK_SHIFT: c_uint = 0x07;
pub const PALMAS_SWOFF_COLDRST_PWRDOWN: c_uint = 0x40;
pub const PALMAS_SWOFF_COLDRST_PWRDOWN_SHIFT: c_uint = 0x06;
pub const PALMAS_SWOFF_COLDRST_WTD: c_uint = 0x20;
pub const PALMAS_SWOFF_COLDRST_WTD_SHIFT: c_uint = 0x05;
pub const PALMAS_SWOFF_COLDRST_TSHUT: c_uint = 0x10;
pub const PALMAS_SWOFF_COLDRST_TSHUT_SHIFT: c_uint = 0x04;
pub const PALMAS_SWOFF_COLDRST_RESET_IN: c_uint = 0x08;
pub const PALMAS_SWOFF_COLDRST_RESET_IN_SHIFT: c_uint = 0x03;
pub const PALMAS_SWOFF_COLDRST_SW_RST: c_uint = 0x04;
pub const PALMAS_SWOFF_COLDRST_SW_RST_SHIFT: c_uint = 0x02;
pub const PALMAS_SWOFF_COLDRST_VSYS_LO: c_uint = 0x02;
pub const PALMAS_SWOFF_COLDRST_VSYS_LO_SHIFT: c_uint = 0x01;
pub const PALMAS_SWOFF_COLDRST_GPADC_SHUTDOWN: c_uint = 0x01;
pub const PALMAS_SWOFF_COLDRST_GPADC_SHUTDOWN_SHIFT: c_uint = 0x00;
// Bit definitions for SWOFF_STATUS
pub const PALMAS_SWOFF_STATUS_PWRON_LPK: c_uint = 0x80;
pub const PALMAS_SWOFF_STATUS_PWRON_LPK_SHIFT: c_uint = 0x07;
pub const PALMAS_SWOFF_STATUS_PWRDOWN: c_uint = 0x40;
pub const PALMAS_SWOFF_STATUS_PWRDOWN_SHIFT: c_uint = 0x06;
pub const PALMAS_SWOFF_STATUS_WTD: c_uint = 0x20;
pub const PALMAS_SWOFF_STATUS_WTD_SHIFT: c_uint = 0x05;
pub const PALMAS_SWOFF_STATUS_TSHUT: c_uint = 0x10;
pub const PALMAS_SWOFF_STATUS_TSHUT_SHIFT: c_uint = 0x04;
pub const PALMAS_SWOFF_STATUS_RESET_IN: c_uint = 0x08;
pub const PALMAS_SWOFF_STATUS_RESET_IN_SHIFT: c_uint = 0x03;
pub const PALMAS_SWOFF_STATUS_SW_RST: c_uint = 0x04;
pub const PALMAS_SWOFF_STATUS_SW_RST_SHIFT: c_uint = 0x02;
pub const PALMAS_SWOFF_STATUS_VSYS_LO: c_uint = 0x02;
pub const PALMAS_SWOFF_STATUS_VSYS_LO_SHIFT: c_uint = 0x01;
pub const PALMAS_SWOFF_STATUS_GPADC_SHUTDOWN: c_uint = 0x01;
pub const PALMAS_SWOFF_STATUS_GPADC_SHUTDOWN_SHIFT: c_uint = 0x00;
// Bit definitions for PMU_CONFIG
pub const PALMAS_PMU_CONFIG_MULTI_CELL_EN: c_uint = 0x40;
pub const PALMAS_PMU_CONFIG_MULTI_CELL_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_PMU_CONFIG_SPARE_MASK: c_uint = 0x30;
pub const PALMAS_PMU_CONFIG_SPARE_SHIFT: c_uint = 0x04;
pub const PALMAS_PMU_CONFIG_SWOFF_DLY_MASK: c_uint = 0x0c;
pub const PALMAS_PMU_CONFIG_SWOFF_DLY_SHIFT: c_uint = 0x02;
pub const PALMAS_PMU_CONFIG_GATE_RESET_OUT: c_uint = 0x02;
pub const PALMAS_PMU_CONFIG_GATE_RESET_OUT_SHIFT: c_uint = 0x01;
pub const PALMAS_PMU_CONFIG_AUTODEVON: c_uint = 0x01;
pub const PALMAS_PMU_CONFIG_AUTODEVON_SHIFT: c_uint = 0x00;
// Bit definitions for SPARE
pub const PALMAS_SPARE_SPARE_MASK: c_uint = 0xf8;
pub const PALMAS_SPARE_SPARE_SHIFT: c_uint = 0x03;
pub const PALMAS_SPARE_REGEN3_OD: c_uint = 0x04;
pub const PALMAS_SPARE_REGEN3_OD_SHIFT: c_uint = 0x02;
pub const PALMAS_SPARE_REGEN2_OD: c_uint = 0x02;
pub const PALMAS_SPARE_REGEN2_OD_SHIFT: c_uint = 0x01;
pub const PALMAS_SPARE_REGEN1_OD: c_uint = 0x01;
pub const PALMAS_SPARE_REGEN1_OD_SHIFT: c_uint = 0x00;
// Bit definitions for PMU_SECONDARY_INT
pub const PALMAS_PMU_SECONDARY_INT_VBUS_OVV_INT_SRC: c_uint = 0x80;
pub const PALMAS_PMU_SECONDARY_INT_VBUS_OVV_INT_SRC_SHIFT: c_uint = 0x07;
pub const PALMAS_PMU_SECONDARY_INT_CHARG_DET_N_INT_SRC: c_uint = 0x40;
pub const PALMAS_PMU_SECONDARY_INT_CHARG_DET_N_INT_SRC_SHIFT: c_uint = 0x06;
pub const PALMAS_PMU_SECONDARY_INT_BB_INT_SRC: c_uint = 0x20;
pub const PALMAS_PMU_SECONDARY_INT_BB_INT_SRC_SHIFT: c_uint = 0x05;
pub const PALMAS_PMU_SECONDARY_INT_FBI_INT_SRC: c_uint = 0x10;
pub const PALMAS_PMU_SECONDARY_INT_FBI_INT_SRC_SHIFT: c_uint = 0x04;
pub const PALMAS_PMU_SECONDARY_INT_VBUS_OVV_MASK: c_uint = 0x08;
pub const PALMAS_PMU_SECONDARY_INT_VBUS_OVV_MASK_SHIFT: c_uint = 0x03;
pub const PALMAS_PMU_SECONDARY_INT_CHARG_DET_N_MASK: c_uint = 0x04;
pub const PALMAS_PMU_SECONDARY_INT_CHARG_DET_N_MASK_SHIFT: c_uint = 0x02;
pub const PALMAS_PMU_SECONDARY_INT_BB_MASK: c_uint = 0x02;
pub const PALMAS_PMU_SECONDARY_INT_BB_MASK_SHIFT: c_uint = 0x01;
pub const PALMAS_PMU_SECONDARY_INT_FBI_MASK: c_uint = 0x01;
pub const PALMAS_PMU_SECONDARY_INT_FBI_MASK_SHIFT: c_uint = 0x00;
// Bit definitions for SW_REVISION
pub const PALMAS_SW_REVISION_SW_REVISION_MASK: c_uint = 0xFF;
pub const PALMAS_SW_REVISION_SW_REVISION_SHIFT: c_uint = 0x00;
// Bit definitions for EXT_CHRG_CTRL
pub const PALMAS_EXT_CHRG_CTRL_VBUS_OVV_STATUS: c_uint = 0x80;
pub const PALMAS_EXT_CHRG_CTRL_VBUS_OVV_STATUS_SHIFT: c_uint = 0x07;
pub const PALMAS_EXT_CHRG_CTRL_CHARG_DET_N_STATUS: c_uint = 0x40;
pub const PALMAS_EXT_CHRG_CTRL_CHARG_DET_N_STATUS_SHIFT: c_uint = 0x06;
pub const PALMAS_EXT_CHRG_CTRL_VSYS_DEBOUNCE_DELAY: c_uint = 0x08;
pub const PALMAS_EXT_CHRG_CTRL_VSYS_DEBOUNCE_DELAY_SHIFT: c_uint = 0x03;
pub const PALMAS_EXT_CHRG_CTRL_CHRG_DET_N: c_uint = 0x04;
pub const PALMAS_EXT_CHRG_CTRL_CHRG_DET_N_SHIFT: c_uint = 0x02;
pub const PALMAS_EXT_CHRG_CTRL_AUTO_ACA_EN: c_uint = 0x02;
pub const PALMAS_EXT_CHRG_CTRL_AUTO_ACA_EN_SHIFT: c_uint = 0x01;
pub const PALMAS_EXT_CHRG_CTRL_AUTO_LDOUSB_EN: c_uint = 0x01;
pub const PALMAS_EXT_CHRG_CTRL_AUTO_LDOUSB_EN_SHIFT: c_uint = 0x00;
// Bit definitions for PMU_SECONDARY_INT2
pub const PALMAS_PMU_SECONDARY_INT2_DVFS2_INT_SRC: c_uint = 0x20;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS2_INT_SRC_SHIFT: c_uint = 0x05;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS1_INT_SRC: c_uint = 0x10;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS1_INT_SRC_SHIFT: c_uint = 0x04;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS2_MASK: c_uint = 0x02;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS2_MASK_SHIFT: c_uint = 0x01;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS1_MASK: c_uint = 0x01;
pub const PALMAS_PMU_SECONDARY_INT2_DVFS1_MASK_SHIFT: c_uint = 0x00;
// Registers for function RESOURCE
pub const PALMAS_CLK32KG_CTRL: c_uint = 0x00;
pub const PALMAS_CLK32KGAUDIO_CTRL: c_uint = 0x01;
pub const PALMAS_REGEN1_CTRL: c_uint = 0x02;
pub const PALMAS_REGEN2_CTRL: c_uint = 0x03;
pub const PALMAS_SYSEN1_CTRL: c_uint = 0x04;
pub const PALMAS_SYSEN2_CTRL: c_uint = 0x05;
pub const PALMAS_NSLEEP_RES_ASSIGN: c_uint = 0x06;
pub const PALMAS_NSLEEP_SMPS_ASSIGN: c_uint = 0x07;
pub const PALMAS_NSLEEP_LDO_ASSIGN1: c_uint = 0x08;
pub const PALMAS_NSLEEP_LDO_ASSIGN2: c_uint = 0x09;
pub const PALMAS_ENABLE1_RES_ASSIGN: c_uint = 0x0A;
pub const PALMAS_ENABLE1_SMPS_ASSIGN: c_uint = 0x0B;
pub const PALMAS_ENABLE1_LDO_ASSIGN1: c_uint = 0x0C;
pub const PALMAS_ENABLE1_LDO_ASSIGN2: c_uint = 0x0D;
pub const PALMAS_ENABLE2_RES_ASSIGN: c_uint = 0x0E;
pub const PALMAS_ENABLE2_SMPS_ASSIGN: c_uint = 0x0F;
pub const PALMAS_ENABLE2_LDO_ASSIGN1: c_uint = 0x10;
pub const PALMAS_ENABLE2_LDO_ASSIGN2: c_uint = 0x11;
pub const PALMAS_REGEN3_CTRL: c_uint = 0x12;
// Bit definitions for CLK32KG_CTRL
pub const PALMAS_CLK32KG_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_CLK32KG_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_CLK32KG_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_CLK32KG_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_CLK32KG_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_CLK32KG_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for CLK32KGAUDIO_CTRL
pub const PALMAS_CLK32KGAUDIO_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_CLK32KGAUDIO_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_CLK32KGAUDIO_CTRL_RESERVED3: c_uint = 0x08;
pub const PALMAS_CLK32KGAUDIO_CTRL_RESERVED3_SHIFT: c_uint = 0x03;
pub const PALMAS_CLK32KGAUDIO_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_CLK32KGAUDIO_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_CLK32KGAUDIO_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_CLK32KGAUDIO_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for REGEN1_CTRL
pub const PALMAS_REGEN1_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_REGEN1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_REGEN1_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_REGEN1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_REGEN1_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_REGEN1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for REGEN2_CTRL
pub const PALMAS_REGEN2_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_REGEN2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_REGEN2_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_REGEN2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_REGEN2_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_REGEN2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SYSEN1_CTRL
pub const PALMAS_SYSEN1_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_SYSEN1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SYSEN1_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_SYSEN1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SYSEN1_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_SYSEN1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SYSEN2_CTRL
pub const PALMAS_SYSEN2_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_SYSEN2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_SYSEN2_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_SYSEN2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_SYSEN2_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_SYSEN2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_RES_ASSIGN
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN3: c_uint = 0x40;
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x06;
pub const PALMAS_NSLEEP_RES_ASSIGN_CLK32KGAUDIO: c_uint = 0x20;
pub const PALMAS_NSLEEP_RES_ASSIGN_CLK32KGAUDIO_SHIFT: c_uint = 0x05;
pub const PALMAS_NSLEEP_RES_ASSIGN_CLK32KG: c_uint = 0x10;
pub const PALMAS_NSLEEP_RES_ASSIGN_CLK32KG_SHIFT: c_uint = 0x04;
pub const PALMAS_NSLEEP_RES_ASSIGN_SYSEN2: c_uint = 0x08;
pub const PALMAS_NSLEEP_RES_ASSIGN_SYSEN2_SHIFT: c_uint = 0x03;
pub const PALMAS_NSLEEP_RES_ASSIGN_SYSEN1: c_uint = 0x04;
pub const PALMAS_NSLEEP_RES_ASSIGN_SYSEN1_SHIFT: c_uint = 0x02;
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const PALMAS_NSLEEP_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_SMPS_ASSIGN
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS10: c_uint = 0x80;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS10_SHIFT: c_uint = 0x07;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS9: c_uint = 0x40;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS8: c_uint = 0x20;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS7: c_uint = 0x10;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS6: c_uint = 0x08;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS45: c_uint = 0x04;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS3: c_uint = 0x02;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS12: c_uint = 0x01;
pub const PALMAS_NSLEEP_SMPS_ASSIGN_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_LDO_ASSIGN1
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO8: c_uint = 0x80;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO8_SHIFT: c_uint = 0x07;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO7: c_uint = 0x40;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO7_SHIFT: c_uint = 0x06;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO6: c_uint = 0x20;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO6_SHIFT: c_uint = 0x05;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO5: c_uint = 0x10;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO5_SHIFT: c_uint = 0x04;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO4: c_uint = 0x08;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x03;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO3: c_uint = 0x04;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO3_SHIFT: c_uint = 0x02;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const PALMAS_NSLEEP_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_LDO_ASSIGN2
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDOUSB: c_uint = 0x04;
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDOUSB_SHIFT: c_uint = 0x02;
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDOLN: c_uint = 0x02;
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDOLN_SHIFT: c_uint = 0x01;
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDO9: c_uint = 0x01;
pub const PALMAS_NSLEEP_LDO_ASSIGN2_LDO9_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_RES_ASSIGN
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN3: c_uint = 0x40;
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE1_RES_ASSIGN_CLK32KGAUDIO: c_uint = 0x20;
pub const PALMAS_ENABLE1_RES_ASSIGN_CLK32KGAUDIO_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE1_RES_ASSIGN_CLK32KG: c_uint = 0x10;
pub const PALMAS_ENABLE1_RES_ASSIGN_CLK32KG_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE1_RES_ASSIGN_SYSEN2: c_uint = 0x08;
pub const PALMAS_ENABLE1_RES_ASSIGN_SYSEN2_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE1_RES_ASSIGN_SYSEN1: c_uint = 0x04;
pub const PALMAS_ENABLE1_RES_ASSIGN_SYSEN1_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const PALMAS_ENABLE1_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_SMPS_ASSIGN
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS10: c_uint = 0x80;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS10_SHIFT: c_uint = 0x07;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS9: c_uint = 0x40;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS8: c_uint = 0x20;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS7: c_uint = 0x10;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS6: c_uint = 0x08;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS45: c_uint = 0x04;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS3: c_uint = 0x02;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS12: c_uint = 0x01;
pub const PALMAS_ENABLE1_SMPS_ASSIGN_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_LDO_ASSIGN1
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO8: c_uint = 0x80;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO8_SHIFT: c_uint = 0x07;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO7: c_uint = 0x40;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO7_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO6: c_uint = 0x20;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO6_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO5: c_uint = 0x10;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO5_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO4: c_uint = 0x08;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO3: c_uint = 0x04;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO3_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const PALMAS_ENABLE1_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_LDO_ASSIGN2
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDOUSB: c_uint = 0x04;
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDOUSB_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDOLN: c_uint = 0x02;
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDOLN_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDO9: c_uint = 0x01;
pub const PALMAS_ENABLE1_LDO_ASSIGN2_LDO9_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_RES_ASSIGN
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN3: c_uint = 0x40;
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE2_RES_ASSIGN_CLK32KGAUDIO: c_uint = 0x20;
pub const PALMAS_ENABLE2_RES_ASSIGN_CLK32KGAUDIO_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE2_RES_ASSIGN_CLK32KG: c_uint = 0x10;
pub const PALMAS_ENABLE2_RES_ASSIGN_CLK32KG_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE2_RES_ASSIGN_SYSEN2: c_uint = 0x08;
pub const PALMAS_ENABLE2_RES_ASSIGN_SYSEN2_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE2_RES_ASSIGN_SYSEN1: c_uint = 0x04;
pub const PALMAS_ENABLE2_RES_ASSIGN_SYSEN1_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const PALMAS_ENABLE2_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_SMPS_ASSIGN
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS10: c_uint = 0x80;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS10_SHIFT: c_uint = 0x07;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS9: c_uint = 0x40;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS9_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS8: c_uint = 0x20;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS8_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS7: c_uint = 0x10;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS7_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS6: c_uint = 0x08;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS6_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS45: c_uint = 0x04;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS45_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS3: c_uint = 0x02;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS12: c_uint = 0x01;
pub const PALMAS_ENABLE2_SMPS_ASSIGN_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_LDO_ASSIGN1
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO8: c_uint = 0x80;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO8_SHIFT: c_uint = 0x07;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO7: c_uint = 0x40;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO7_SHIFT: c_uint = 0x06;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO6: c_uint = 0x20;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO6_SHIFT: c_uint = 0x05;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO5: c_uint = 0x10;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO5_SHIFT: c_uint = 0x04;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO4: c_uint = 0x08;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x03;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO3: c_uint = 0x04;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO3_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const PALMAS_ENABLE2_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_LDO_ASSIGN2
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDOUSB: c_uint = 0x04;
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDOUSB_SHIFT: c_uint = 0x02;
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDOLN: c_uint = 0x02;
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDOLN_SHIFT: c_uint = 0x01;
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDO9: c_uint = 0x01;
pub const PALMAS_ENABLE2_LDO_ASSIGN2_LDO9_SHIFT: c_uint = 0x00;
// Bit definitions for REGEN3_CTRL
pub const PALMAS_REGEN3_CTRL_STATUS: c_uint = 0x10;
pub const PALMAS_REGEN3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const PALMAS_REGEN3_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const PALMAS_REGEN3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const PALMAS_REGEN3_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const PALMAS_REGEN3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Registers for function PAD_CONTROL
pub const PALMAS_OD_OUTPUT_CTRL2: c_uint = 0x02;
pub const PALMAS_POLARITY_CTRL2: c_uint = 0x03;
pub const PALMAS_PU_PD_INPUT_CTRL1: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL2: c_uint = 0x05;
pub const PALMAS_PU_PD_INPUT_CTRL3: c_uint = 0x06;
pub const PALMAS_PU_PD_INPUT_CTRL5: c_uint = 0x07;
pub const PALMAS_OD_OUTPUT_CTRL: c_uint = 0x08;
pub const PALMAS_POLARITY_CTRL: c_uint = 0x09;
pub const PALMAS_PRIMARY_SECONDARY_PAD1: c_uint = 0x0A;
pub const PALMAS_PRIMARY_SECONDARY_PAD2: c_uint = 0x0B;
pub const PALMAS_I2C_SPI: c_uint = 0x0C;
pub const PALMAS_PU_PD_INPUT_CTRL4: c_uint = 0x0D;
pub const PALMAS_PRIMARY_SECONDARY_PAD3: c_uint = 0x0E;
pub const PALMAS_PRIMARY_SECONDARY_PAD4: c_uint = 0x0F;
// Bit definitions for PU_PD_INPUT_CTRL1
pub const PALMAS_PU_PD_INPUT_CTRL1_RESET_IN_PD: c_uint = 0x40;
pub const PALMAS_PU_PD_INPUT_CTRL1_RESET_IN_PD_SHIFT: c_uint = 0x06;
pub const PALMAS_PU_PD_INPUT_CTRL1_GPADC_START_PU: c_uint = 0x20;
pub const PALMAS_PU_PD_INPUT_CTRL1_GPADC_START_PU_SHIFT: c_uint = 0x05;
pub const PALMAS_PU_PD_INPUT_CTRL1_GPADC_START_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_INPUT_CTRL1_GPADC_START_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL1_PWRDOWN_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL1_PWRDOWN_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL1_NRESWARM_PU: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL1_NRESWARM_PU_SHIFT: c_uint = 0x01;
// Bit definitions for PU_PD_INPUT_CTRL2
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE2_PU: c_uint = 0x20;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE2_PU_SHIFT: c_uint = 0x05;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE2_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE2_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE1_PU: c_uint = 0x08;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE1_PU_SHIFT: c_uint = 0x03;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE1_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL2_ENABLE1_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL2_NSLEEP_PU: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL2_NSLEEP_PU_SHIFT: c_uint = 0x01;
pub const PALMAS_PU_PD_INPUT_CTRL2_NSLEEP_PD: c_uint = 0x01;
pub const PALMAS_PU_PD_INPUT_CTRL2_NSLEEP_PD_SHIFT: c_uint = 0x00;
// Bit definitions for PU_PD_INPUT_CTRL3
pub const PALMAS_PU_PD_INPUT_CTRL3_ACOK_PD: c_uint = 0x40;
pub const PALMAS_PU_PD_INPUT_CTRL3_ACOK_PD_SHIFT: c_uint = 0x06;
pub const PALMAS_PU_PD_INPUT_CTRL3_CHRG_DET_N_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_INPUT_CTRL3_CHRG_DET_N_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL3_POWERHOLD_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL3_POWERHOLD_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL3_MSECURE_PD: c_uint = 0x01;
pub const PALMAS_PU_PD_INPUT_CTRL3_MSECURE_PD_SHIFT: c_uint = 0x00;
// Bit definitions for OD_OUTPUT_CTRL
pub const PALMAS_OD_OUTPUT_CTRL_PWM_2_OD: c_uint = 0x80;
pub const PALMAS_OD_OUTPUT_CTRL_PWM_2_OD_SHIFT: c_uint = 0x07;
pub const PALMAS_OD_OUTPUT_CTRL_VBUSDET_OD: c_uint = 0x40;
pub const PALMAS_OD_OUTPUT_CTRL_VBUSDET_OD_SHIFT: c_uint = 0x06;
pub const PALMAS_OD_OUTPUT_CTRL_PWM_1_OD: c_uint = 0x20;
pub const PALMAS_OD_OUTPUT_CTRL_PWM_1_OD_SHIFT: c_uint = 0x05;
pub const PALMAS_OD_OUTPUT_CTRL_INT_OD: c_uint = 0x08;
pub const PALMAS_OD_OUTPUT_CTRL_INT_OD_SHIFT: c_uint = 0x03;
// Bit definitions for POLARITY_CTRL
pub const PALMAS_POLARITY_CTRL_INT_POLARITY: c_uint = 0x80;
pub const PALMAS_POLARITY_CTRL_INT_POLARITY_SHIFT: c_uint = 0x07;
pub const PALMAS_POLARITY_CTRL_ENABLE2_POLARITY: c_uint = 0x40;
pub const PALMAS_POLARITY_CTRL_ENABLE2_POLARITY_SHIFT: c_uint = 0x06;
pub const PALMAS_POLARITY_CTRL_ENABLE1_POLARITY: c_uint = 0x20;
pub const PALMAS_POLARITY_CTRL_ENABLE1_POLARITY_SHIFT: c_uint = 0x05;
pub const PALMAS_POLARITY_CTRL_NSLEEP_POLARITY: c_uint = 0x10;
pub const PALMAS_POLARITY_CTRL_NSLEEP_POLARITY_SHIFT: c_uint = 0x04;
pub const PALMAS_POLARITY_CTRL_RESET_IN_POLARITY: c_uint = 0x08;
pub const PALMAS_POLARITY_CTRL_RESET_IN_POLARITY_SHIFT: c_uint = 0x03;
pub const PALMAS_POLARITY_CTRL_GPIO_3_CHRG_DET_N_POLARITY: c_uint = 0x04;
pub const PALMAS_POLARITY_CTRL_GPIO_3_CHRG_DET_N_POLARITY_SHIFT: c_uint = 0x02;
pub const PALMAS_POLARITY_CTRL_POWERGOOD_USB_PSEL_POLARITY: c_uint = 0x02;
pub const PALMAS_POLARITY_CTRL_POWERGOOD_USB_PSEL_POLARITY_SHIFT: c_uint = 0x01;
pub const PALMAS_POLARITY_CTRL_PWRDOWN_POLARITY: c_uint = 0x01;
pub const PALMAS_POLARITY_CTRL_PWRDOWN_POLARITY_SHIFT: c_uint = 0x00;
// Bit definitions for PRIMARY_SECONDARY_PAD1
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_3: c_uint = 0x80;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_3_SHIFT: c_uint = 0x07;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_2_MASK: c_uint = 0x60;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_2_SHIFT: c_uint = 0x05;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_1_MASK: c_uint = 0x18;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_1_SHIFT: c_uint = 0x03;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_0: c_uint = 0x04;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_GPIO_0_SHIFT: c_uint = 0x02;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_VAC: c_uint = 0x02;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_VAC_SHIFT: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_POWERGOOD: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD1_POWERGOOD_SHIFT: c_uint = 0x00;
// Bit definitions for PRIMARY_SECONDARY_PAD2
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_7_MASK: c_uint = 0x30;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_7_SHIFT: c_uint = 0x04;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_6: c_uint = 0x08;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_6_SHIFT: c_uint = 0x03;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_5_MASK: c_uint = 0x06;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_5_SHIFT: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_4: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD2_GPIO_4_SHIFT: c_uint = 0x00;
// Bit definitions for I2C_SPI
pub const PALMAS_I2C_SPI_I2C2OTP_EN: c_uint = 0x80;
pub const PALMAS_I2C_SPI_I2C2OTP_EN_SHIFT: c_uint = 0x07;
pub const PALMAS_I2C_SPI_I2C2OTP_PAGESEL: c_uint = 0x40;
pub const PALMAS_I2C_SPI_I2C2OTP_PAGESEL_SHIFT: c_uint = 0x06;
pub const PALMAS_I2C_SPI_ID_I2C2: c_uint = 0x20;
pub const PALMAS_I2C_SPI_ID_I2C2_SHIFT: c_uint = 0x05;
pub const PALMAS_I2C_SPI_I2C_SPI: c_uint = 0x10;
pub const PALMAS_I2C_SPI_I2C_SPI_SHIFT: c_uint = 0x04;
pub const PALMAS_I2C_SPI_ID_I2C1_MASK: c_uint = 0x0F;
pub const PALMAS_I2C_SPI_ID_I2C1_SHIFT: c_uint = 0x00;
// Bit definitions for PU_PD_INPUT_CTRL4
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS2_DAT_PD: c_uint = 0x40;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS2_DAT_PD_SHIFT: c_uint = 0x06;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS2_CLK_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS2_CLK_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS1_DAT_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS1_DAT_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS1_CLK_PD: c_uint = 0x01;
pub const PALMAS_PU_PD_INPUT_CTRL4_DVFS1_CLK_PD_SHIFT: c_uint = 0x00;
// Bit definitions for PRIMARY_SECONDARY_PAD3
pub const PALMAS_PRIMARY_SECONDARY_PAD3_DVFS2: c_uint = 0x02;
pub const PALMAS_PRIMARY_SECONDARY_PAD3_DVFS2_SHIFT: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD3_DVFS1: c_uint = 0x01;
pub const PALMAS_PRIMARY_SECONDARY_PAD3_DVFS1_SHIFT: c_uint = 0x00;
// Registers for function LED_PWM
pub const PALMAS_LED_PERIOD_CTRL: c_uint = 0x00;
pub const PALMAS_LED_CTRL: c_uint = 0x01;
pub const PALMAS_PWM_CTRL1: c_uint = 0x02;
pub const PALMAS_PWM_CTRL2: c_uint = 0x03;
// Bit definitions for LED_PERIOD_CTRL
pub const PALMAS_LED_PERIOD_CTRL_LED_2_PERIOD_MASK: c_uint = 0x38;
pub const PALMAS_LED_PERIOD_CTRL_LED_2_PERIOD_SHIFT: c_uint = 0x03;
pub const PALMAS_LED_PERIOD_CTRL_LED_1_PERIOD_MASK: c_uint = 0x07;
pub const PALMAS_LED_PERIOD_CTRL_LED_1_PERIOD_SHIFT: c_uint = 0x00;
// Bit definitions for LED_CTRL
pub const PALMAS_LED_CTRL_LED_2_SEQ: c_uint = 0x20;
pub const PALMAS_LED_CTRL_LED_2_SEQ_SHIFT: c_uint = 0x05;
pub const PALMAS_LED_CTRL_LED_1_SEQ: c_uint = 0x10;
pub const PALMAS_LED_CTRL_LED_1_SEQ_SHIFT: c_uint = 0x04;
pub const PALMAS_LED_CTRL_LED_2_ON_TIME_MASK: c_uint = 0x0c;
pub const PALMAS_LED_CTRL_LED_2_ON_TIME_SHIFT: c_uint = 0x02;
pub const PALMAS_LED_CTRL_LED_1_ON_TIME_MASK: c_uint = 0x03;
pub const PALMAS_LED_CTRL_LED_1_ON_TIME_SHIFT: c_uint = 0x00;
// Bit definitions for PWM_CTRL1
pub const PALMAS_PWM_CTRL1_PWM_FREQ_EN: c_uint = 0x02;
pub const PALMAS_PWM_CTRL1_PWM_FREQ_EN_SHIFT: c_uint = 0x01;
pub const PALMAS_PWM_CTRL1_PWM_FREQ_SEL: c_uint = 0x01;
pub const PALMAS_PWM_CTRL1_PWM_FREQ_SEL_SHIFT: c_uint = 0x00;
// Bit definitions for PWM_CTRL2
pub const PALMAS_PWM_CTRL2_PWM_DUTY_SEL_MASK: c_uint = 0xFF;
pub const PALMAS_PWM_CTRL2_PWM_DUTY_SEL_SHIFT: c_uint = 0x00;
// Registers for function INTERRUPT
pub const PALMAS_INT1_STATUS: c_uint = 0x00;
pub const PALMAS_INT1_MASK: c_uint = 0x01;
pub const PALMAS_INT1_LINE_STATE: c_uint = 0x02;
pub const PALMAS_INT1_EDGE_DETECT1_RESERVED: c_uint = 0x03;
pub const PALMAS_INT1_EDGE_DETECT2_RESERVED: c_uint = 0x04;
pub const PALMAS_INT2_STATUS: c_uint = 0x05;
pub const PALMAS_INT2_MASK: c_uint = 0x06;
pub const PALMAS_INT2_LINE_STATE: c_uint = 0x07;
pub const PALMAS_INT2_EDGE_DETECT1_RESERVED: c_uint = 0x08;
pub const PALMAS_INT2_EDGE_DETECT2_RESERVED: c_uint = 0x09;
pub const PALMAS_INT3_STATUS: c_uint = 0x0A;
pub const PALMAS_INT3_MASK: c_uint = 0x0B;
pub const PALMAS_INT3_LINE_STATE: c_uint = 0x0C;
pub const PALMAS_INT3_EDGE_DETECT1_RESERVED: c_uint = 0x0D;
pub const PALMAS_INT3_EDGE_DETECT2_RESERVED: c_uint = 0x0E;
pub const PALMAS_INT4_STATUS: c_uint = 0x0F;
pub const PALMAS_INT4_MASK: c_uint = 0x10;
pub const PALMAS_INT4_LINE_STATE: c_uint = 0x11;
pub const PALMAS_INT4_EDGE_DETECT1: c_uint = 0x12;
pub const PALMAS_INT4_EDGE_DETECT2: c_uint = 0x13;
pub const PALMAS_INT_CTRL: c_uint = 0x14;
// Bit definitions for INT1_STATUS
pub const PALMAS_INT1_STATUS_VBAT_MON: c_uint = 0x80;
pub const PALMAS_INT1_STATUS_VBAT_MON_SHIFT: c_uint = 0x07;
pub const PALMAS_INT1_STATUS_VSYS_MON: c_uint = 0x40;
pub const PALMAS_INT1_STATUS_VSYS_MON_SHIFT: c_uint = 0x06;
pub const PALMAS_INT1_STATUS_HOTDIE: c_uint = 0x20;
pub const PALMAS_INT1_STATUS_HOTDIE_SHIFT: c_uint = 0x05;
pub const PALMAS_INT1_STATUS_PWRDOWN: c_uint = 0x10;
pub const PALMAS_INT1_STATUS_PWRDOWN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT1_STATUS_RPWRON: c_uint = 0x08;
pub const PALMAS_INT1_STATUS_RPWRON_SHIFT: c_uint = 0x03;
pub const PALMAS_INT1_STATUS_LONG_PRESS_KEY: c_uint = 0x04;
pub const PALMAS_INT1_STATUS_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const PALMAS_INT1_STATUS_PWRON: c_uint = 0x02;
pub const PALMAS_INT1_STATUS_PWRON_SHIFT: c_uint = 0x01;
pub const PALMAS_INT1_STATUS_CHARG_DET_N_VBUS_OVV: c_uint = 0x01;
pub const PALMAS_INT1_STATUS_CHARG_DET_N_VBUS_OVV_SHIFT: c_uint = 0x00;
// Bit definitions for INT1_MASK
pub const PALMAS_INT1_MASK_VBAT_MON: c_uint = 0x80;
pub const PALMAS_INT1_MASK_VBAT_MON_SHIFT: c_uint = 0x07;
pub const PALMAS_INT1_MASK_VSYS_MON: c_uint = 0x40;
pub const PALMAS_INT1_MASK_VSYS_MON_SHIFT: c_uint = 0x06;
pub const PALMAS_INT1_MASK_HOTDIE: c_uint = 0x20;
pub const PALMAS_INT1_MASK_HOTDIE_SHIFT: c_uint = 0x05;
pub const PALMAS_INT1_MASK_PWRDOWN: c_uint = 0x10;
pub const PALMAS_INT1_MASK_PWRDOWN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT1_MASK_RPWRON: c_uint = 0x08;
pub const PALMAS_INT1_MASK_RPWRON_SHIFT: c_uint = 0x03;
pub const PALMAS_INT1_MASK_LONG_PRESS_KEY: c_uint = 0x04;
pub const PALMAS_INT1_MASK_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const PALMAS_INT1_MASK_PWRON: c_uint = 0x02;
pub const PALMAS_INT1_MASK_PWRON_SHIFT: c_uint = 0x01;
pub const PALMAS_INT1_MASK_CHARG_DET_N_VBUS_OVV: c_uint = 0x01;
pub const PALMAS_INT1_MASK_CHARG_DET_N_VBUS_OVV_SHIFT: c_uint = 0x00;
// Bit definitions for INT1_LINE_STATE
pub const PALMAS_INT1_LINE_STATE_VBAT_MON: c_uint = 0x80;
pub const PALMAS_INT1_LINE_STATE_VBAT_MON_SHIFT: c_uint = 0x07;
pub const PALMAS_INT1_LINE_STATE_VSYS_MON: c_uint = 0x40;
pub const PALMAS_INT1_LINE_STATE_VSYS_MON_SHIFT: c_uint = 0x06;
pub const PALMAS_INT1_LINE_STATE_HOTDIE: c_uint = 0x20;
pub const PALMAS_INT1_LINE_STATE_HOTDIE_SHIFT: c_uint = 0x05;
pub const PALMAS_INT1_LINE_STATE_PWRDOWN: c_uint = 0x10;
pub const PALMAS_INT1_LINE_STATE_PWRDOWN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT1_LINE_STATE_RPWRON: c_uint = 0x08;
pub const PALMAS_INT1_LINE_STATE_RPWRON_SHIFT: c_uint = 0x03;
pub const PALMAS_INT1_LINE_STATE_LONG_PRESS_KEY: c_uint = 0x04;
pub const PALMAS_INT1_LINE_STATE_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const PALMAS_INT1_LINE_STATE_PWRON: c_uint = 0x02;
pub const PALMAS_INT1_LINE_STATE_PWRON_SHIFT: c_uint = 0x01;
pub const PALMAS_INT1_LINE_STATE_CHARG_DET_N_VBUS_OVV: c_uint = 0x01;
pub const PALMAS_INT1_LINE_STATE_CHARG_DET_N_VBUS_OVV_SHIFT: c_uint = 0x00;
// Bit definitions for INT2_STATUS
pub const PALMAS_INT2_STATUS_VAC_ACOK: c_uint = 0x80;
pub const PALMAS_INT2_STATUS_VAC_ACOK_SHIFT: c_uint = 0x07;
pub const PALMAS_INT2_STATUS_SHORT: c_uint = 0x40;
pub const PALMAS_INT2_STATUS_SHORT_SHIFT: c_uint = 0x06;
pub const PALMAS_INT2_STATUS_FBI_BB: c_uint = 0x20;
pub const PALMAS_INT2_STATUS_FBI_BB_SHIFT: c_uint = 0x05;
pub const PALMAS_INT2_STATUS_RESET_IN: c_uint = 0x10;
pub const PALMAS_INT2_STATUS_RESET_IN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT2_STATUS_BATREMOVAL: c_uint = 0x08;
pub const PALMAS_INT2_STATUS_BATREMOVAL_SHIFT: c_uint = 0x03;
pub const PALMAS_INT2_STATUS_WDT: c_uint = 0x04;
pub const PALMAS_INT2_STATUS_WDT_SHIFT: c_uint = 0x02;
pub const PALMAS_INT2_STATUS_RTC_TIMER: c_uint = 0x02;
pub const PALMAS_INT2_STATUS_RTC_TIMER_SHIFT: c_uint = 0x01;
pub const PALMAS_INT2_STATUS_RTC_ALARM: c_uint = 0x01;
pub const PALMAS_INT2_STATUS_RTC_ALARM_SHIFT: c_uint = 0x00;
// Bit definitions for INT2_MASK
pub const PALMAS_INT2_MASK_VAC_ACOK: c_uint = 0x80;
pub const PALMAS_INT2_MASK_VAC_ACOK_SHIFT: c_uint = 0x07;
pub const PALMAS_INT2_MASK_SHORT: c_uint = 0x40;
pub const PALMAS_INT2_MASK_SHORT_SHIFT: c_uint = 0x06;
pub const PALMAS_INT2_MASK_FBI_BB: c_uint = 0x20;
pub const PALMAS_INT2_MASK_FBI_BB_SHIFT: c_uint = 0x05;
pub const PALMAS_INT2_MASK_RESET_IN: c_uint = 0x10;
pub const PALMAS_INT2_MASK_RESET_IN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT2_MASK_BATREMOVAL: c_uint = 0x08;
pub const PALMAS_INT2_MASK_BATREMOVAL_SHIFT: c_uint = 0x03;
pub const PALMAS_INT2_MASK_WDT: c_uint = 0x04;
pub const PALMAS_INT2_MASK_WDT_SHIFT: c_uint = 0x02;
pub const PALMAS_INT2_MASK_RTC_TIMER: c_uint = 0x02;
pub const PALMAS_INT2_MASK_RTC_TIMER_SHIFT: c_uint = 0x01;
pub const PALMAS_INT2_MASK_RTC_ALARM: c_uint = 0x01;
pub const PALMAS_INT2_MASK_RTC_ALARM_SHIFT: c_uint = 0x00;
// Bit definitions for INT2_LINE_STATE
pub const PALMAS_INT2_LINE_STATE_VAC_ACOK: c_uint = 0x80;
pub const PALMAS_INT2_LINE_STATE_VAC_ACOK_SHIFT: c_uint = 0x07;
pub const PALMAS_INT2_LINE_STATE_SHORT: c_uint = 0x40;
pub const PALMAS_INT2_LINE_STATE_SHORT_SHIFT: c_uint = 0x06;
pub const PALMAS_INT2_LINE_STATE_FBI_BB: c_uint = 0x20;
pub const PALMAS_INT2_LINE_STATE_FBI_BB_SHIFT: c_uint = 0x05;
pub const PALMAS_INT2_LINE_STATE_RESET_IN: c_uint = 0x10;
pub const PALMAS_INT2_LINE_STATE_RESET_IN_SHIFT: c_uint = 0x04;
pub const PALMAS_INT2_LINE_STATE_BATREMOVAL: c_uint = 0x08;
pub const PALMAS_INT2_LINE_STATE_BATREMOVAL_SHIFT: c_uint = 0x03;
pub const PALMAS_INT2_LINE_STATE_WDT: c_uint = 0x04;
pub const PALMAS_INT2_LINE_STATE_WDT_SHIFT: c_uint = 0x02;
pub const PALMAS_INT2_LINE_STATE_RTC_TIMER: c_uint = 0x02;
pub const PALMAS_INT2_LINE_STATE_RTC_TIMER_SHIFT: c_uint = 0x01;
pub const PALMAS_INT2_LINE_STATE_RTC_ALARM: c_uint = 0x01;
pub const PALMAS_INT2_LINE_STATE_RTC_ALARM_SHIFT: c_uint = 0x00;
// Bit definitions for INT3_STATUS
pub const PALMAS_INT3_STATUS_VBUS: c_uint = 0x80;
pub const PALMAS_INT3_STATUS_VBUS_SHIFT: c_uint = 0x07;
pub const PALMAS_INT3_STATUS_VBUS_OTG: c_uint = 0x40;
pub const PALMAS_INT3_STATUS_VBUS_OTG_SHIFT: c_uint = 0x06;
pub const PALMAS_INT3_STATUS_ID: c_uint = 0x20;
pub const PALMAS_INT3_STATUS_ID_SHIFT: c_uint = 0x05;
pub const PALMAS_INT3_STATUS_ID_OTG: c_uint = 0x10;
pub const PALMAS_INT3_STATUS_ID_OTG_SHIFT: c_uint = 0x04;
pub const PALMAS_INT3_STATUS_GPADC_EOC_RT: c_uint = 0x08;
pub const PALMAS_INT3_STATUS_GPADC_EOC_RT_SHIFT: c_uint = 0x03;
pub const PALMAS_INT3_STATUS_GPADC_EOC_SW: c_uint = 0x04;
pub const PALMAS_INT3_STATUS_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const PALMAS_INT3_STATUS_GPADC_AUTO_1: c_uint = 0x02;
pub const PALMAS_INT3_STATUS_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT3_STATUS_GPADC_AUTO_0: c_uint = 0x01;
pub const PALMAS_INT3_STATUS_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT3_MASK
pub const PALMAS_INT3_MASK_VBUS: c_uint = 0x80;
pub const PALMAS_INT3_MASK_VBUS_SHIFT: c_uint = 0x07;
pub const PALMAS_INT3_MASK_VBUS_OTG: c_uint = 0x40;
pub const PALMAS_INT3_MASK_VBUS_OTG_SHIFT: c_uint = 0x06;
pub const PALMAS_INT3_MASK_ID: c_uint = 0x20;
pub const PALMAS_INT3_MASK_ID_SHIFT: c_uint = 0x05;
pub const PALMAS_INT3_MASK_ID_OTG: c_uint = 0x10;
pub const PALMAS_INT3_MASK_ID_OTG_SHIFT: c_uint = 0x04;
pub const PALMAS_INT3_MASK_GPADC_EOC_RT: c_uint = 0x08;
pub const PALMAS_INT3_MASK_GPADC_EOC_RT_SHIFT: c_uint = 0x03;
pub const PALMAS_INT3_MASK_GPADC_EOC_SW: c_uint = 0x04;
pub const PALMAS_INT3_MASK_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const PALMAS_INT3_MASK_GPADC_AUTO_1: c_uint = 0x02;
pub const PALMAS_INT3_MASK_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT3_MASK_GPADC_AUTO_0: c_uint = 0x01;
pub const PALMAS_INT3_MASK_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT3_LINE_STATE
pub const PALMAS_INT3_LINE_STATE_VBUS: c_uint = 0x80;
pub const PALMAS_INT3_LINE_STATE_VBUS_SHIFT: c_uint = 0x07;
pub const PALMAS_INT3_LINE_STATE_VBUS_OTG: c_uint = 0x40;
pub const PALMAS_INT3_LINE_STATE_VBUS_OTG_SHIFT: c_uint = 0x06;
pub const PALMAS_INT3_LINE_STATE_ID: c_uint = 0x20;
pub const PALMAS_INT3_LINE_STATE_ID_SHIFT: c_uint = 0x05;
pub const PALMAS_INT3_LINE_STATE_ID_OTG: c_uint = 0x10;
pub const PALMAS_INT3_LINE_STATE_ID_OTG_SHIFT: c_uint = 0x04;
pub const PALMAS_INT3_LINE_STATE_GPADC_EOC_RT: c_uint = 0x08;
pub const PALMAS_INT3_LINE_STATE_GPADC_EOC_RT_SHIFT: c_uint = 0x03;
pub const PALMAS_INT3_LINE_STATE_GPADC_EOC_SW: c_uint = 0x04;
pub const PALMAS_INT3_LINE_STATE_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const PALMAS_INT3_LINE_STATE_GPADC_AUTO_1: c_uint = 0x02;
pub const PALMAS_INT3_LINE_STATE_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT3_LINE_STATE_GPADC_AUTO_0: c_uint = 0x01;
pub const PALMAS_INT3_LINE_STATE_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_STATUS
pub const PALMAS_INT4_STATUS_GPIO_7: c_uint = 0x80;
pub const PALMAS_INT4_STATUS_GPIO_7_SHIFT: c_uint = 0x07;
pub const PALMAS_INT4_STATUS_GPIO_6: c_uint = 0x40;
pub const PALMAS_INT4_STATUS_GPIO_6_SHIFT: c_uint = 0x06;
pub const PALMAS_INT4_STATUS_GPIO_5: c_uint = 0x20;
pub const PALMAS_INT4_STATUS_GPIO_5_SHIFT: c_uint = 0x05;
pub const PALMAS_INT4_STATUS_GPIO_4: c_uint = 0x10;
pub const PALMAS_INT4_STATUS_GPIO_4_SHIFT: c_uint = 0x04;
pub const PALMAS_INT4_STATUS_GPIO_3: c_uint = 0x08;
pub const PALMAS_INT4_STATUS_GPIO_3_SHIFT: c_uint = 0x03;
pub const PALMAS_INT4_STATUS_GPIO_2: c_uint = 0x04;
pub const PALMAS_INT4_STATUS_GPIO_2_SHIFT: c_uint = 0x02;
pub const PALMAS_INT4_STATUS_GPIO_1: c_uint = 0x02;
pub const PALMAS_INT4_STATUS_GPIO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT4_STATUS_GPIO_0: c_uint = 0x01;
pub const PALMAS_INT4_STATUS_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_MASK
pub const PALMAS_INT4_MASK_GPIO_7: c_uint = 0x80;
pub const PALMAS_INT4_MASK_GPIO_7_SHIFT: c_uint = 0x07;
pub const PALMAS_INT4_MASK_GPIO_6: c_uint = 0x40;
pub const PALMAS_INT4_MASK_GPIO_6_SHIFT: c_uint = 0x06;
pub const PALMAS_INT4_MASK_GPIO_5: c_uint = 0x20;
pub const PALMAS_INT4_MASK_GPIO_5_SHIFT: c_uint = 0x05;
pub const PALMAS_INT4_MASK_GPIO_4: c_uint = 0x10;
pub const PALMAS_INT4_MASK_GPIO_4_SHIFT: c_uint = 0x04;
pub const PALMAS_INT4_MASK_GPIO_3: c_uint = 0x08;
pub const PALMAS_INT4_MASK_GPIO_3_SHIFT: c_uint = 0x03;
pub const PALMAS_INT4_MASK_GPIO_2: c_uint = 0x04;
pub const PALMAS_INT4_MASK_GPIO_2_SHIFT: c_uint = 0x02;
pub const PALMAS_INT4_MASK_GPIO_1: c_uint = 0x02;
pub const PALMAS_INT4_MASK_GPIO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT4_MASK_GPIO_0: c_uint = 0x01;
pub const PALMAS_INT4_MASK_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_LINE_STATE
pub const PALMAS_INT4_LINE_STATE_GPIO_7: c_uint = 0x80;
pub const PALMAS_INT4_LINE_STATE_GPIO_7_SHIFT: c_uint = 0x07;
pub const PALMAS_INT4_LINE_STATE_GPIO_6: c_uint = 0x40;
pub const PALMAS_INT4_LINE_STATE_GPIO_6_SHIFT: c_uint = 0x06;
pub const PALMAS_INT4_LINE_STATE_GPIO_5: c_uint = 0x20;
pub const PALMAS_INT4_LINE_STATE_GPIO_5_SHIFT: c_uint = 0x05;
pub const PALMAS_INT4_LINE_STATE_GPIO_4: c_uint = 0x10;
pub const PALMAS_INT4_LINE_STATE_GPIO_4_SHIFT: c_uint = 0x04;
pub const PALMAS_INT4_LINE_STATE_GPIO_3: c_uint = 0x08;
pub const PALMAS_INT4_LINE_STATE_GPIO_3_SHIFT: c_uint = 0x03;
pub const PALMAS_INT4_LINE_STATE_GPIO_2: c_uint = 0x04;
pub const PALMAS_INT4_LINE_STATE_GPIO_2_SHIFT: c_uint = 0x02;
pub const PALMAS_INT4_LINE_STATE_GPIO_1: c_uint = 0x02;
pub const PALMAS_INT4_LINE_STATE_GPIO_1_SHIFT: c_uint = 0x01;
pub const PALMAS_INT4_LINE_STATE_GPIO_0: c_uint = 0x01;
pub const PALMAS_INT4_LINE_STATE_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_EDGE_DETECT1
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_3_RISING: c_uint = 0x80;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_3_RISING_SHIFT: c_uint = 0x07;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_3_FALLING: c_uint = 0x40;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_3_FALLING_SHIFT: c_uint = 0x06;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_2_RISING: c_uint = 0x20;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_2_RISING_SHIFT: c_uint = 0x05;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_2_FALLING: c_uint = 0x10;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_2_FALLING_SHIFT: c_uint = 0x04;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_1_RISING: c_uint = 0x08;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_1_RISING_SHIFT: c_uint = 0x03;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_1_FALLING: c_uint = 0x04;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_1_FALLING_SHIFT: c_uint = 0x02;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_0_RISING: c_uint = 0x02;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_0_RISING_SHIFT: c_uint = 0x01;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_0_FALLING: c_uint = 0x01;
pub const PALMAS_INT4_EDGE_DETECT1_GPIO_0_FALLING_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_EDGE_DETECT2
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_7_RISING: c_uint = 0x80;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_7_RISING_SHIFT: c_uint = 0x07;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_7_FALLING: c_uint = 0x40;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_7_FALLING_SHIFT: c_uint = 0x06;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_6_RISING: c_uint = 0x20;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_6_RISING_SHIFT: c_uint = 0x05;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_6_FALLING: c_uint = 0x10;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_6_FALLING_SHIFT: c_uint = 0x04;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_5_RISING: c_uint = 0x08;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_5_RISING_SHIFT: c_uint = 0x03;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_5_FALLING: c_uint = 0x04;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_5_FALLING_SHIFT: c_uint = 0x02;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_4_RISING: c_uint = 0x02;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_4_RISING_SHIFT: c_uint = 0x01;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_4_FALLING: c_uint = 0x01;
pub const PALMAS_INT4_EDGE_DETECT2_GPIO_4_FALLING_SHIFT: c_uint = 0x00;
// Bit definitions for INT_CTRL
pub const PALMAS_INT_CTRL_INT_PENDING: c_uint = 0x04;
pub const PALMAS_INT_CTRL_INT_PENDING_SHIFT: c_uint = 0x02;
pub const PALMAS_INT_CTRL_INT_CLEAR: c_uint = 0x01;
pub const PALMAS_INT_CTRL_INT_CLEAR_SHIFT: c_uint = 0x00;
// Registers for function USB_OTG
pub const PALMAS_USB_WAKEUP: c_uint = 0x03;
pub const PALMAS_USB_VBUS_CTRL_SET: c_uint = 0x04;
pub const PALMAS_USB_VBUS_CTRL_CLR: c_uint = 0x05;
pub const PALMAS_USB_ID_CTRL_SET: c_uint = 0x06;
pub const PALMAS_USB_ID_CTRL_CLEAR: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_SRC: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_LATCH_SET: c_uint = 0x09;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR: c_uint = 0x0A;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET: c_uint = 0x0B;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR: c_uint = 0x0C;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET: c_uint = 0x0D;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR: c_uint = 0x0E;
pub const PALMAS_USB_ID_INT_SRC: c_uint = 0x0F;
pub const PALMAS_USB_ID_INT_LATCH_SET: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_LATCH_CLR: c_uint = 0x11;
pub const PALMAS_USB_ID_INT_EN_LO_SET: c_uint = 0x12;
pub const PALMAS_USB_ID_INT_EN_LO_CLR: c_uint = 0x13;
pub const PALMAS_USB_ID_INT_EN_HI_SET: c_uint = 0x14;
pub const PALMAS_USB_ID_INT_EN_HI_CLR: c_uint = 0x15;
pub const PALMAS_USB_OTG_ADP_CTRL: c_uint = 0x16;
pub const PALMAS_USB_OTG_ADP_HIGH: c_uint = 0x17;
pub const PALMAS_USB_OTG_ADP_LOW: c_uint = 0x18;
pub const PALMAS_USB_OTG_ADP_RISE: c_uint = 0x19;
pub const PALMAS_USB_OTG_REVISION: c_uint = 0x1A;
// Bit definitions for USB_WAKEUP
pub const PALMAS_USB_WAKEUP_ID_WK_UP_COMP: c_uint = 0x01;
pub const PALMAS_USB_WAKEUP_ID_WK_UP_COMP_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_CTRL_SET
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_CHRG_VSYS: c_uint = 0x80;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_CHRG_VSYS_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_DISCHRG: c_uint = 0x20;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_DISCHRG_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_IADP_SRC: c_uint = 0x10;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_IADP_SRC_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_IADP_SINK: c_uint = 0x08;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_IADP_SINK_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_ACT_COMP: c_uint = 0x04;
pub const PALMAS_USB_VBUS_CTRL_SET_VBUS_ACT_COMP_SHIFT: c_uint = 0x02;
// Bit definitions for USB_VBUS_CTRL_CLR
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_CHRG_VSYS: c_uint = 0x80;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_CHRG_VSYS_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_DISCHRG: c_uint = 0x20;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_DISCHRG_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_IADP_SRC: c_uint = 0x10;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_IADP_SRC_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_IADP_SINK: c_uint = 0x08;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_IADP_SINK_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_ACT_COMP: c_uint = 0x04;
pub const PALMAS_USB_VBUS_CTRL_CLR_VBUS_ACT_COMP_SHIFT: c_uint = 0x02;
// Bit definitions for USB_ID_CTRL_SET
pub const PALMAS_USB_ID_CTRL_SET_ID_PU_220K: c_uint = 0x80;
pub const PALMAS_USB_ID_CTRL_SET_ID_PU_220K_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_ID_CTRL_SET_ID_PU_100K: c_uint = 0x40;
pub const PALMAS_USB_ID_CTRL_SET_ID_PU_100K_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_ID_CTRL_SET_ID_GND_DRV: c_uint = 0x20;
pub const PALMAS_USB_ID_CTRL_SET_ID_GND_DRV_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_ID_CTRL_SET_ID_SRC_16U: c_uint = 0x10;
pub const PALMAS_USB_ID_CTRL_SET_ID_SRC_16U_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_CTRL_SET_ID_SRC_5U: c_uint = 0x08;
pub const PALMAS_USB_ID_CTRL_SET_ID_SRC_5U_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_CTRL_SET_ID_ACT_COMP: c_uint = 0x04;
pub const PALMAS_USB_ID_CTRL_SET_ID_ACT_COMP_SHIFT: c_uint = 0x02;
// Bit definitions for USB_ID_CTRL_CLEAR
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_PU_220K: c_uint = 0x80;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_PU_220K_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_PU_100K: c_uint = 0x40;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_PU_100K_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_GND_DRV: c_uint = 0x20;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_GND_DRV_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_SRC_16U: c_uint = 0x10;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_SRC_16U_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_SRC_5U: c_uint = 0x08;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_SRC_5U_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_ACT_COMP: c_uint = 0x04;
pub const PALMAS_USB_ID_CTRL_CLEAR_ID_ACT_COMP_SHIFT: c_uint = 0x02;
// Bit definitions for USB_VBUS_INT_SRC
pub const PALMAS_USB_VBUS_INT_SRC_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_SRC_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_SRC_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_SRC_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_SRC_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_SRC_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_SRC_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_SRC_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_SRC_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_SRC_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_SRC_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_SRC_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_SRC_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_SRC_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_LATCH_SET
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_ADP: c_uint = 0x10;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_ADP_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_LATCH_SET_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_LATCH_CLR
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_ADP: c_uint = 0x10;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_ADP_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_LATCH_CLR_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_EN_LO_SET
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_LO_SET_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_EN_LO_CLR
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_LO_CLR_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_EN_HI_SET
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_ADP: c_uint = 0x10;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_ADP_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_HI_SET_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_VBUS_INT_EN_HI_CLR
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VOTG_SESS_VLD: c_uint = 0x80;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VOTG_SESS_VLD_SHIFT: c_uint = 0x07;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VADP_PRB: c_uint = 0x40;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VADP_PRB_SHIFT: c_uint = 0x06;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VADP_SNS: c_uint = 0x20;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VADP_SNS_SHIFT: c_uint = 0x05;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_ADP: c_uint = 0x10;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_ADP_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VA_VBUS_VLD: c_uint = 0x08;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VA_VBUS_VLD_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VA_SESS_VLD: c_uint = 0x04;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VA_SESS_VLD_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VB_SESS_VLD: c_uint = 0x02;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VB_SESS_VLD_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VB_SESS_END: c_uint = 0x01;
pub const PALMAS_USB_VBUS_INT_EN_HI_CLR_VB_SESS_END_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_SRC
pub const PALMAS_USB_ID_INT_SRC_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_SRC_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_SRC_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_SRC_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_SRC_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_SRC_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_SRC_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_SRC_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_SRC_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_SRC_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_LATCH_SET
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_LATCH_SET_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_LATCH_CLR
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_LATCH_CLR_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_EN_LO_SET
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_LO_SET_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_EN_LO_CLR
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_LO_CLR_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_EN_HI_SET
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_HI_SET_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_ID_INT_EN_HI_CLR
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_FLOAT: c_uint = 0x10;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_FLOAT_SHIFT: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_A: c_uint = 0x08;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_A_SHIFT: c_uint = 0x03;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_B: c_uint = 0x04;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_B_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_C: c_uint = 0x02;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_C_SHIFT: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_GND: c_uint = 0x01;
pub const PALMAS_USB_ID_INT_EN_HI_CLR_ID_GND_SHIFT: c_uint = 0x00;
// Bit definitions for USB_OTG_ADP_CTRL
pub const PALMAS_USB_OTG_ADP_CTRL_ADP_EN: c_uint = 0x04;
pub const PALMAS_USB_OTG_ADP_CTRL_ADP_EN_SHIFT: c_uint = 0x02;
pub const PALMAS_USB_OTG_ADP_CTRL_ADP_MODE_MASK: c_uint = 0x03;
pub const PALMAS_USB_OTG_ADP_CTRL_ADP_MODE_SHIFT: c_uint = 0x00;
// Bit definitions for USB_OTG_ADP_HIGH
pub const PALMAS_USB_OTG_ADP_HIGH_T_ADP_HIGH_MASK: c_uint = 0xFF;
pub const PALMAS_USB_OTG_ADP_HIGH_T_ADP_HIGH_SHIFT: c_uint = 0x00;
// Bit definitions for USB_OTG_ADP_LOW
pub const PALMAS_USB_OTG_ADP_LOW_T_ADP_LOW_MASK: c_uint = 0xFF;
pub const PALMAS_USB_OTG_ADP_LOW_T_ADP_LOW_SHIFT: c_uint = 0x00;
// Bit definitions for USB_OTG_ADP_RISE
pub const PALMAS_USB_OTG_ADP_RISE_T_ADP_RISE_MASK: c_uint = 0xFF;
pub const PALMAS_USB_OTG_ADP_RISE_T_ADP_RISE_SHIFT: c_uint = 0x00;
// Bit definitions for USB_OTG_REVISION
pub const PALMAS_USB_OTG_REVISION_OTG_REV: c_uint = 0x01;
pub const PALMAS_USB_OTG_REVISION_OTG_REV_SHIFT: c_uint = 0x00;
// Registers for function VIBRATOR
pub const PALMAS_VIBRA_CTRL: c_uint = 0x00;
// Bit definitions for VIBRA_CTRL
pub const PALMAS_VIBRA_CTRL_PWM_DUTY_SEL_MASK: c_uint = 0x06;
pub const PALMAS_VIBRA_CTRL_PWM_DUTY_SEL_SHIFT: c_uint = 0x01;
pub const PALMAS_VIBRA_CTRL_PWM_FREQ_SEL: c_uint = 0x01;
pub const PALMAS_VIBRA_CTRL_PWM_FREQ_SEL_SHIFT: c_uint = 0x00;
// Registers for function GPIO
pub const PALMAS_GPIO_DATA_IN: c_uint = 0x00;
pub const PALMAS_GPIO_DATA_DIR: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_OUT: c_uint = 0x02;
pub const PALMAS_GPIO_DEBOUNCE_EN: c_uint = 0x03;
pub const PALMAS_GPIO_CLEAR_DATA_OUT: c_uint = 0x04;
pub const PALMAS_GPIO_SET_DATA_OUT: c_uint = 0x05;
pub const PALMAS_PU_PD_GPIO_CTRL1: c_uint = 0x06;
pub const PALMAS_PU_PD_GPIO_CTRL2: c_uint = 0x07;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL: c_uint = 0x08;
pub const PALMAS_GPIO_DATA_IN2: c_uint = 0x09;
pub const PALMAS_GPIO_DATA_DIR2: c_uint = 0x0A;
pub const PALMAS_GPIO_DATA_OUT2: c_uint = 0x0B;
pub const PALMAS_GPIO_DEBOUNCE_EN2: c_uint = 0x0C;
pub const PALMAS_GPIO_CLEAR_DATA_OUT2: c_uint = 0x0D;
pub const PALMAS_GPIO_SET_DATA_OUT2: c_uint = 0x0E;
pub const PALMAS_PU_PD_GPIO_CTRL3: c_uint = 0x0F;
pub const PALMAS_PU_PD_GPIO_CTRL4: c_uint = 0x10;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL2: c_uint = 0x11;
// Bit definitions for GPIO_DATA_IN
pub const PALMAS_GPIO_DATA_IN_GPIO_7_IN: c_uint = 0x80;
pub const PALMAS_GPIO_DATA_IN_GPIO_7_IN_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_DATA_IN_GPIO_6_IN: c_uint = 0x40;
pub const PALMAS_GPIO_DATA_IN_GPIO_6_IN_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_DATA_IN_GPIO_5_IN: c_uint = 0x20;
pub const PALMAS_GPIO_DATA_IN_GPIO_5_IN_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_DATA_IN_GPIO_4_IN: c_uint = 0x10;
pub const PALMAS_GPIO_DATA_IN_GPIO_4_IN_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_IN_GPIO_3_IN: c_uint = 0x08;
pub const PALMAS_GPIO_DATA_IN_GPIO_3_IN_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_DATA_IN_GPIO_2_IN: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_IN_GPIO_2_IN_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_IN_GPIO_1_IN: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_IN_GPIO_1_IN_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_IN_GPIO_0_IN: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_IN_GPIO_0_IN_SHIFT: c_uint = 0x00;
// Bit definitions for GPIO_DATA_DIR
pub const PALMAS_GPIO_DATA_DIR_GPIO_7_DIR: c_uint = 0x80;
pub const PALMAS_GPIO_DATA_DIR_GPIO_7_DIR_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_DATA_DIR_GPIO_6_DIR: c_uint = 0x40;
pub const PALMAS_GPIO_DATA_DIR_GPIO_6_DIR_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_DATA_DIR_GPIO_5_DIR: c_uint = 0x20;
pub const PALMAS_GPIO_DATA_DIR_GPIO_5_DIR_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_DATA_DIR_GPIO_4_DIR: c_uint = 0x10;
pub const PALMAS_GPIO_DATA_DIR_GPIO_4_DIR_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_DIR_GPIO_3_DIR: c_uint = 0x08;
pub const PALMAS_GPIO_DATA_DIR_GPIO_3_DIR_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_DATA_DIR_GPIO_2_DIR: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_DIR_GPIO_2_DIR_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_DIR_GPIO_1_DIR: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_DIR_GPIO_1_DIR_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_DIR_GPIO_0_DIR: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_DIR_GPIO_0_DIR_SHIFT: c_uint = 0x00;
// Bit definitions for GPIO_DATA_OUT
pub const PALMAS_GPIO_DATA_OUT_GPIO_7_OUT: c_uint = 0x80;
pub const PALMAS_GPIO_DATA_OUT_GPIO_7_OUT_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_DATA_OUT_GPIO_6_OUT: c_uint = 0x40;
pub const PALMAS_GPIO_DATA_OUT_GPIO_6_OUT_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_DATA_OUT_GPIO_5_OUT: c_uint = 0x20;
pub const PALMAS_GPIO_DATA_OUT_GPIO_5_OUT_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_DATA_OUT_GPIO_4_OUT: c_uint = 0x10;
pub const PALMAS_GPIO_DATA_OUT_GPIO_4_OUT_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_OUT_GPIO_3_OUT: c_uint = 0x08;
pub const PALMAS_GPIO_DATA_OUT_GPIO_3_OUT_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_DATA_OUT_GPIO_2_OUT: c_uint = 0x04;
pub const PALMAS_GPIO_DATA_OUT_GPIO_2_OUT_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_OUT_GPIO_1_OUT: c_uint = 0x02;
pub const PALMAS_GPIO_DATA_OUT_GPIO_1_OUT_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_OUT_GPIO_0_OUT: c_uint = 0x01;
pub const PALMAS_GPIO_DATA_OUT_GPIO_0_OUT_SHIFT: c_uint = 0x00;
// Bit definitions for GPIO_DEBOUNCE_EN
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_7_DEBOUNCE_EN: c_uint = 0x80;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_7_DEBOUNCE_EN_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_6_DEBOUNCE_EN: c_uint = 0x40;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_6_DEBOUNCE_EN_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_5_DEBOUNCE_EN: c_uint = 0x20;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_5_DEBOUNCE_EN_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_4_DEBOUNCE_EN: c_uint = 0x10;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_4_DEBOUNCE_EN_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_3_DEBOUNCE_EN: c_uint = 0x08;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_3_DEBOUNCE_EN_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_2_DEBOUNCE_EN: c_uint = 0x04;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_2_DEBOUNCE_EN_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_1_DEBOUNCE_EN: c_uint = 0x02;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_1_DEBOUNCE_EN_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_0_DEBOUNCE_EN: c_uint = 0x01;
pub const PALMAS_GPIO_DEBOUNCE_EN_GPIO_0_DEBOUNCE_EN_SHIFT: c_uint = 0x00;
// Bit definitions for GPIO_CLEAR_DATA_OUT
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_7_CLEAR_DATA_OUT: c_uint = 0x80;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_7_CLEAR_DATA_OUT_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_6_CLEAR_DATA_OUT: c_uint = 0x40;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_6_CLEAR_DATA_OUT_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_5_CLEAR_DATA_OUT: c_uint = 0x20;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_5_CLEAR_DATA_OUT_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_4_CLEAR_DATA_OUT: c_uint = 0x10;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_4_CLEAR_DATA_OUT_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_3_CLEAR_DATA_OUT: c_uint = 0x08;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_3_CLEAR_DATA_OUT_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_2_CLEAR_DATA_OUT: c_uint = 0x04;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_2_CLEAR_DATA_OUT_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_1_CLEAR_DATA_OUT: c_uint = 0x02;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_1_CLEAR_DATA_OUT_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_0_CLEAR_DATA_OUT: c_uint = 0x01;
pub const PALMAS_GPIO_CLEAR_DATA_OUT_GPIO_0_CLEAR_DATA_OUT_SHIFT: c_uint = 0x00;
// Bit definitions for GPIO_SET_DATA_OUT
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_7_SET_DATA_OUT: c_uint = 0x80;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_7_SET_DATA_OUT_SHIFT: c_uint = 0x07;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_6_SET_DATA_OUT: c_uint = 0x40;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_6_SET_DATA_OUT_SHIFT: c_uint = 0x06;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_5_SET_DATA_OUT: c_uint = 0x20;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_5_SET_DATA_OUT_SHIFT: c_uint = 0x05;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_4_SET_DATA_OUT: c_uint = 0x10;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_4_SET_DATA_OUT_SHIFT: c_uint = 0x04;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_3_SET_DATA_OUT: c_uint = 0x08;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_3_SET_DATA_OUT_SHIFT: c_uint = 0x03;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_2_SET_DATA_OUT: c_uint = 0x04;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_2_SET_DATA_OUT_SHIFT: c_uint = 0x02;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_1_SET_DATA_OUT: c_uint = 0x02;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_1_SET_DATA_OUT_SHIFT: c_uint = 0x01;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_0_SET_DATA_OUT: c_uint = 0x01;
pub const PALMAS_GPIO_SET_DATA_OUT_GPIO_0_SET_DATA_OUT_SHIFT: c_uint = 0x00;
// Bit definitions for PU_PD_GPIO_CTRL1
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_3_PD: c_uint = 0x40;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_3_PD_SHIFT: c_uint = 0x06;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_2_PU: c_uint = 0x20;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_2_PU_SHIFT: c_uint = 0x05;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_2_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_2_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_1_PU: c_uint = 0x08;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_1_PU_SHIFT: c_uint = 0x03;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_1_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_1_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_0_PD: c_uint = 0x01;
pub const PALMAS_PU_PD_GPIO_CTRL1_GPIO_0_PD_SHIFT: c_uint = 0x00;
// Bit definitions for PU_PD_GPIO_CTRL2
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_7_PD: c_uint = 0x40;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_7_PD_SHIFT: c_uint = 0x06;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_6_PU: c_uint = 0x20;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_6_PU_SHIFT: c_uint = 0x05;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_6_PD: c_uint = 0x10;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_6_PD_SHIFT: c_uint = 0x04;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_5_PU: c_uint = 0x08;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_5_PU_SHIFT: c_uint = 0x03;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_5_PD: c_uint = 0x04;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_5_PD_SHIFT: c_uint = 0x02;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_4_PU: c_uint = 0x02;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_4_PU_SHIFT: c_uint = 0x01;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_4_PD: c_uint = 0x01;
pub const PALMAS_PU_PD_GPIO_CTRL2_GPIO_4_PD_SHIFT: c_uint = 0x00;
// Bit definitions for OD_OUTPUT_GPIO_CTRL
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_5_OD: c_uint = 0x20;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_5_OD_SHIFT: c_uint = 0x05;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_2_OD: c_uint = 0x04;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_2_OD_SHIFT: c_uint = 0x02;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_1_OD: c_uint = 0x02;
pub const PALMAS_OD_OUTPUT_GPIO_CTRL_GPIO_1_OD_SHIFT: c_uint = 0x01;
// Registers for function GPADC
pub const PALMAS_GPADC_CTRL1: c_uint = 0x00;
pub const PALMAS_GPADC_CTRL2: c_uint = 0x01;
pub const PALMAS_GPADC_RT_CTRL: c_uint = 0x02;
pub const PALMAS_GPADC_AUTO_CTRL: c_uint = 0x03;
pub const PALMAS_GPADC_STATUS: c_uint = 0x04;
pub const PALMAS_GPADC_RT_SELECT: c_uint = 0x05;
pub const PALMAS_GPADC_RT_CONV0_LSB: c_uint = 0x06;
pub const PALMAS_GPADC_RT_CONV0_MSB: c_uint = 0x07;
pub const PALMAS_GPADC_AUTO_SELECT: c_uint = 0x08;
pub const PALMAS_GPADC_AUTO_CONV0_LSB: c_uint = 0x09;
pub const PALMAS_GPADC_AUTO_CONV0_MSB: c_uint = 0x0A;
pub const PALMAS_GPADC_AUTO_CONV1_LSB: c_uint = 0x0B;
pub const PALMAS_GPADC_AUTO_CONV1_MSB: c_uint = 0x0C;
pub const PALMAS_GPADC_SW_SELECT: c_uint = 0x0D;
pub const PALMAS_GPADC_SW_CONV0_LSB: c_uint = 0x0E;
pub const PALMAS_GPADC_SW_CONV0_MSB: c_uint = 0x0F;
pub const PALMAS_GPADC_THRES_CONV0_LSB: c_uint = 0x10;
pub const PALMAS_GPADC_THRES_CONV0_MSB: c_uint = 0x11;
pub const PALMAS_GPADC_THRES_CONV1_LSB: c_uint = 0x12;
pub const PALMAS_GPADC_THRES_CONV1_MSB: c_uint = 0x13;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN: c_uint = 0x14;
pub const PALMAS_GPADC_SMPS_VSEL_MONITORING: c_uint = 0x15;
// Bit definitions for GPADC_CTRL1
pub const PALMAS_GPADC_CTRL1_RESERVED_MASK: c_uint = 0xc0;
pub const PALMAS_GPADC_CTRL1_RESERVED_SHIFT: c_uint = 0x06;
pub const PALMAS_GPADC_CTRL1_CURRENT_SRC_CH3_MASK: c_uint = 0x30;
pub const PALMAS_GPADC_CTRL1_CURRENT_SRC_CH3_SHIFT: c_uint = 0x04;
pub const PALMAS_GPADC_CTRL1_CURRENT_SRC_CH0_MASK: c_uint = 0x0c;
pub const PALMAS_GPADC_CTRL1_CURRENT_SRC_CH0_SHIFT: c_uint = 0x02;
pub const PALMAS_GPADC_CTRL1_BAT_REMOVAL_DET: c_uint = 0x02;
pub const PALMAS_GPADC_CTRL1_BAT_REMOVAL_DET_SHIFT: c_uint = 0x01;
pub const PALMAS_GPADC_CTRL1_GPADC_FORCE: c_uint = 0x01;
pub const PALMAS_GPADC_CTRL1_GPADC_FORCE_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_CTRL2
pub const PALMAS_GPADC_CTRL2_RESERVED_MASK: c_uint = 0x06;
pub const PALMAS_GPADC_CTRL2_RESERVED_SHIFT: c_uint = 0x01;
// Bit definitions for GPADC_RT_CTRL
pub const PALMAS_GPADC_RT_CTRL_EXTEND_DELAY: c_uint = 0x02;
pub const PALMAS_GPADC_RT_CTRL_EXTEND_DELAY_SHIFT: c_uint = 0x01;
pub const PALMAS_GPADC_RT_CTRL_START_POLARITY: c_uint = 0x01;
pub const PALMAS_GPADC_RT_CTRL_START_POLARITY_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_CTRL
pub const PALMAS_GPADC_AUTO_CTRL_SHUTDOWN_CONV1: c_uint = 0x80;
pub const PALMAS_GPADC_AUTO_CTRL_SHUTDOWN_CONV1_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_AUTO_CTRL_SHUTDOWN_CONV0: c_uint = 0x40;
pub const PALMAS_GPADC_AUTO_CTRL_SHUTDOWN_CONV0_SHIFT: c_uint = 0x06;
pub const PALMAS_GPADC_AUTO_CTRL_AUTO_CONV1_EN: c_uint = 0x20;
pub const PALMAS_GPADC_AUTO_CTRL_AUTO_CONV1_EN_SHIFT: c_uint = 0x05;
pub const PALMAS_GPADC_AUTO_CTRL_AUTO_CONV0_EN: c_uint = 0x10;
pub const PALMAS_GPADC_AUTO_CTRL_AUTO_CONV0_EN_SHIFT: c_uint = 0x04;
pub const PALMAS_GPADC_AUTO_CTRL_COUNTER_CONV_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_AUTO_CTRL_COUNTER_CONV_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_STATUS
pub const PALMAS_GPADC_STATUS_GPADC_AVAILABLE: c_uint = 0x10;
pub const PALMAS_GPADC_STATUS_GPADC_AVAILABLE_SHIFT: c_uint = 0x04;
// Bit definitions for GPADC_RT_SELECT
pub const PALMAS_GPADC_RT_SELECT_RT_CONV_EN: c_uint = 0x80;
pub const PALMAS_GPADC_RT_SELECT_RT_CONV_EN_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_RT_SELECT_RT_CONV0_SEL_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_RT_SELECT_RT_CONV0_SEL_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_RT_CONV0_LSB
pub const PALMAS_GPADC_RT_CONV0_LSB_RT_CONV0_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_RT_CONV0_LSB_RT_CONV0_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_RT_CONV0_MSB
pub const PALMAS_GPADC_RT_CONV0_MSB_RT_CONV0_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_RT_CONV0_MSB_RT_CONV0_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_SELECT
pub const PALMAS_GPADC_AUTO_SELECT_AUTO_CONV1_SEL_MASK: c_uint = 0xF0;
pub const PALMAS_GPADC_AUTO_SELECT_AUTO_CONV1_SEL_SHIFT: c_uint = 0x04;
pub const PALMAS_GPADC_AUTO_SELECT_AUTO_CONV0_SEL_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_AUTO_SELECT_AUTO_CONV0_SEL_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_CONV0_LSB
pub const PALMAS_GPADC_AUTO_CONV0_LSB_AUTO_CONV0_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_AUTO_CONV0_LSB_AUTO_CONV0_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_CONV0_MSB
pub const PALMAS_GPADC_AUTO_CONV0_MSB_AUTO_CONV0_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_AUTO_CONV0_MSB_AUTO_CONV0_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_CONV1_LSB
pub const PALMAS_GPADC_AUTO_CONV1_LSB_AUTO_CONV1_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_AUTO_CONV1_LSB_AUTO_CONV1_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_AUTO_CONV1_MSB
pub const PALMAS_GPADC_AUTO_CONV1_MSB_AUTO_CONV1_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_AUTO_CONV1_MSB_AUTO_CONV1_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_SW_SELECT
pub const PALMAS_GPADC_SW_SELECT_SW_CONV_EN: c_uint = 0x80;
pub const PALMAS_GPADC_SW_SELECT_SW_CONV_EN_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_SW_SELECT_SW_START_CONV0: c_uint = 0x10;
pub const PALMAS_GPADC_SW_SELECT_SW_START_CONV0_SHIFT: c_uint = 0x04;
pub const PALMAS_GPADC_SW_SELECT_SW_CONV0_SEL_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_SW_SELECT_SW_CONV0_SEL_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_SW_CONV0_LSB
pub const PALMAS_GPADC_SW_CONV0_LSB_SW_CONV0_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_SW_CONV0_LSB_SW_CONV0_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_SW_CONV0_MSB
pub const PALMAS_GPADC_SW_CONV0_MSB_SW_CONV0_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_SW_CONV0_MSB_SW_CONV0_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_THRES_CONV0_LSB
pub const PALMAS_GPADC_THRES_CONV0_LSB_THRES_CONV0_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_THRES_CONV0_LSB_THRES_CONV0_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_THRES_CONV0_MSB
pub const PALMAS_GPADC_THRES_CONV0_MSB_THRES_CONV0_POL: c_uint = 0x80;
pub const PALMAS_GPADC_THRES_CONV0_MSB_THRES_CONV0_POL_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_THRES_CONV0_MSB_THRES_CONV0_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_THRES_CONV0_MSB_THRES_CONV0_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_THRES_CONV1_LSB
pub const PALMAS_GPADC_THRES_CONV1_LSB_THRES_CONV1_LSB_MASK: c_uint = 0xFF;
pub const PALMAS_GPADC_THRES_CONV1_LSB_THRES_CONV1_LSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_THRES_CONV1_MSB
pub const PALMAS_GPADC_THRES_CONV1_MSB_THRES_CONV1_POL: c_uint = 0x80;
pub const PALMAS_GPADC_THRES_CONV1_MSB_THRES_CONV1_POL_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_THRES_CONV1_MSB_THRES_CONV1_MSB_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_THRES_CONV1_MSB_THRES_CONV1_MSB_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_SMPS_ILMONITOR_EN
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_EN: c_uint = 0x20;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_EN_SHIFT: c_uint = 0x05;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_REXT: c_uint = 0x10;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_REXT_SHIFT: c_uint = 0x04;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_SEL_MASK: c_uint = 0x0F;
pub const PALMAS_GPADC_SMPS_ILMONITOR_EN_SMPS_ILMON_SEL_SHIFT: c_uint = 0x00;
// Bit definitions for GPADC_SMPS_VSEL_MONITORING
pub const PALMAS_GPADC_SMPS_VSEL_MONITORING_ACTIVE_PHASE: c_uint = 0x80;
pub const PALMAS_GPADC_SMPS_VSEL_MONITORING_ACTIVE_PHASE_SHIFT: c_uint = 0x07;
pub const PALMAS_GPADC_SMPS_VSEL_MONITORING_SMPS_VSEL_MONITORING_MASK: c_uint = 0x7F;
pub const PALMAS_GPADC_SMPS_VSEL_MONITORING_SMPS_VSEL_MONITORING_SHIFT: c_uint = 0x00;
// Registers for function GPADC
pub const PALMAS_GPADC_TRIM1: c_uint = 0x00;
pub const PALMAS_GPADC_TRIM2: c_uint = 0x01;
pub const PALMAS_GPADC_TRIM3: c_uint = 0x02;
pub const PALMAS_GPADC_TRIM4: c_uint = 0x03;
pub const PALMAS_GPADC_TRIM5: c_uint = 0x04;
pub const PALMAS_GPADC_TRIM6: c_uint = 0x05;
pub const PALMAS_GPADC_TRIM7: c_uint = 0x06;
pub const PALMAS_GPADC_TRIM8: c_uint = 0x07;
pub const PALMAS_GPADC_TRIM9: c_uint = 0x08;
pub const PALMAS_GPADC_TRIM10: c_uint = 0x09;
pub const PALMAS_GPADC_TRIM11: c_uint = 0x0A;
pub const PALMAS_GPADC_TRIM12: c_uint = 0x0B;
pub const PALMAS_GPADC_TRIM13: c_uint = 0x0C;
pub const PALMAS_GPADC_TRIM14: c_uint = 0x0D;
pub const PALMAS_GPADC_TRIM15: c_uint = 0x0E;
pub const PALMAS_GPADC_TRIM16: c_uint = 0x0F;
// TPS659038 regen2_ctrl offset iss different from palmas
pub const TPS659038_REGEN2_CTRL: c_uint = 0x12;
// TPS65917 Interrupt registers
// Registers for function INTERRUPT
pub const TPS65917_INT1_STATUS: c_uint = 0x00;
pub const TPS65917_INT1_MASK: c_uint = 0x01;
pub const TPS65917_INT1_LINE_STATE: c_uint = 0x02;
pub const TPS65917_INT2_STATUS: c_uint = 0x05;
pub const TPS65917_INT2_MASK: c_uint = 0x06;
pub const TPS65917_INT2_LINE_STATE: c_uint = 0x07;
pub const TPS65917_INT3_STATUS: c_uint = 0x0A;
pub const TPS65917_INT3_MASK: c_uint = 0x0B;
pub const TPS65917_INT3_LINE_STATE: c_uint = 0x0C;
pub const TPS65917_INT4_STATUS: c_uint = 0x0F;
pub const TPS65917_INT4_MASK: c_uint = 0x10;
pub const TPS65917_INT4_LINE_STATE: c_uint = 0x11;
pub const TPS65917_INT4_EDGE_DETECT1: c_uint = 0x12;
pub const TPS65917_INT4_EDGE_DETECT2: c_uint = 0x13;
pub const TPS65917_INT_CTRL: c_uint = 0x14;
// Bit definitions for INT1_STATUS
pub const TPS65917_INT1_STATUS_VSYS_MON: c_uint = 0x40;
pub const TPS65917_INT1_STATUS_VSYS_MON_SHIFT: c_uint = 0x06;
pub const TPS65917_INT1_STATUS_HOTDIE: c_uint = 0x20;
pub const TPS65917_INT1_STATUS_HOTDIE_SHIFT: c_uint = 0x05;
pub const TPS65917_INT1_STATUS_PWRDOWN: c_uint = 0x10;
pub const TPS65917_INT1_STATUS_PWRDOWN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT1_STATUS_LONG_PRESS_KEY: c_uint = 0x04;
pub const TPS65917_INT1_STATUS_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const TPS65917_INT1_STATUS_PWRON: c_uint = 0x02;
pub const TPS65917_INT1_STATUS_PWRON_SHIFT: c_uint = 0x01;
// Bit definitions for INT1_MASK
pub const TPS65917_INT1_MASK_VSYS_MON: c_uint = 0x40;
pub const TPS65917_INT1_MASK_VSYS_MON_SHIFT: c_uint = 0x06;
pub const TPS65917_INT1_MASK_HOTDIE: c_uint = 0x20;
pub const TPS65917_INT1_MASK_HOTDIE_SHIFT: c_uint = 0x05;
pub const TPS65917_INT1_MASK_PWRDOWN: c_uint = 0x10;
pub const TPS65917_INT1_MASK_PWRDOWN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT1_MASK_LONG_PRESS_KEY: c_uint = 0x04;
pub const TPS65917_INT1_MASK_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const TPS65917_INT1_MASK_PWRON: c_uint = 0x02;
pub const TPS65917_INT1_MASK_PWRON_SHIFT: c_uint = 0x01;
// Bit definitions for INT1_LINE_STATE
pub const TPS65917_INT1_LINE_STATE_VSYS_MON: c_uint = 0x40;
pub const TPS65917_INT1_LINE_STATE_VSYS_MON_SHIFT: c_uint = 0x06;
pub const TPS65917_INT1_LINE_STATE_HOTDIE: c_uint = 0x20;
pub const TPS65917_INT1_LINE_STATE_HOTDIE_SHIFT: c_uint = 0x05;
pub const TPS65917_INT1_LINE_STATE_PWRDOWN: c_uint = 0x10;
pub const TPS65917_INT1_LINE_STATE_PWRDOWN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT1_LINE_STATE_LONG_PRESS_KEY: c_uint = 0x04;
pub const TPS65917_INT1_LINE_STATE_LONG_PRESS_KEY_SHIFT: c_uint = 0x02;
pub const TPS65917_INT1_LINE_STATE_PWRON: c_uint = 0x02;
pub const TPS65917_INT1_LINE_STATE_PWRON_SHIFT: c_uint = 0x01;
// Bit definitions for INT2_STATUS
pub const TPS65917_INT2_STATUS_SHORT: c_uint = 0x40;
pub const TPS65917_INT2_STATUS_SHORT_SHIFT: c_uint = 0x06;
pub const TPS65917_INT2_STATUS_FSD: c_uint = 0x20;
pub const TPS65917_INT2_STATUS_FSD_SHIFT: c_uint = 0x05;
pub const TPS65917_INT2_STATUS_RESET_IN: c_uint = 0x10;
pub const TPS65917_INT2_STATUS_RESET_IN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT2_STATUS_WDT: c_uint = 0x04;
pub const TPS65917_INT2_STATUS_WDT_SHIFT: c_uint = 0x02;
pub const TPS65917_INT2_STATUS_OTP_ERROR: c_uint = 0x02;
pub const TPS65917_INT2_STATUS_OTP_ERROR_SHIFT: c_uint = 0x01;
// Bit definitions for INT2_MASK
pub const TPS65917_INT2_MASK_SHORT: c_uint = 0x40;
pub const TPS65917_INT2_MASK_SHORT_SHIFT: c_uint = 0x06;
pub const TPS65917_INT2_MASK_FSD: c_uint = 0x20;
pub const TPS65917_INT2_MASK_FSD_SHIFT: c_uint = 0x05;
pub const TPS65917_INT2_MASK_RESET_IN: c_uint = 0x10;
pub const TPS65917_INT2_MASK_RESET_IN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT2_MASK_WDT: c_uint = 0x04;
pub const TPS65917_INT2_MASK_WDT_SHIFT: c_uint = 0x02;
pub const TPS65917_INT2_MASK_OTP_ERROR_TIMER: c_uint = 0x02;
pub const TPS65917_INT2_MASK_OTP_ERROR_SHIFT: c_uint = 0x01;
// Bit definitions for INT2_LINE_STATE
pub const TPS65917_INT2_LINE_STATE_SHORT: c_uint = 0x40;
pub const TPS65917_INT2_LINE_STATE_SHORT_SHIFT: c_uint = 0x06;
pub const TPS65917_INT2_LINE_STATE_FSD: c_uint = 0x20;
pub const TPS65917_INT2_LINE_STATE_FSD_SHIFT: c_uint = 0x05;
pub const TPS65917_INT2_LINE_STATE_RESET_IN: c_uint = 0x10;
pub const TPS65917_INT2_LINE_STATE_RESET_IN_SHIFT: c_uint = 0x04;
pub const TPS65917_INT2_LINE_STATE_WDT: c_uint = 0x04;
pub const TPS65917_INT2_LINE_STATE_WDT_SHIFT: c_uint = 0x02;
pub const TPS65917_INT2_LINE_STATE_OTP_ERROR: c_uint = 0x02;
pub const TPS65917_INT2_LINE_STATE_OTP_ERROR_SHIFT: c_uint = 0x01;
// Bit definitions for INT3_STATUS
pub const TPS65917_INT3_STATUS_VBUS: c_uint = 0x80;
pub const TPS65917_INT3_STATUS_VBUS_SHIFT: c_uint = 0x07;
pub const TPS65917_INT3_STATUS_GPADC_EOC_SW: c_uint = 0x04;
pub const TPS65917_INT3_STATUS_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const TPS65917_INT3_STATUS_GPADC_AUTO_1: c_uint = 0x02;
pub const TPS65917_INT3_STATUS_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT3_STATUS_GPADC_AUTO_0: c_uint = 0x01;
pub const TPS65917_INT3_STATUS_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT3_MASK
pub const TPS65917_INT3_MASK_VBUS: c_uint = 0x80;
pub const TPS65917_INT3_MASK_VBUS_SHIFT: c_uint = 0x07;
pub const TPS65917_INT3_MASK_GPADC_EOC_SW: c_uint = 0x04;
pub const TPS65917_INT3_MASK_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const TPS65917_INT3_MASK_GPADC_AUTO_1: c_uint = 0x02;
pub const TPS65917_INT3_MASK_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT3_MASK_GPADC_AUTO_0: c_uint = 0x01;
pub const TPS65917_INT3_MASK_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT3_LINE_STATE
pub const TPS65917_INT3_LINE_STATE_VBUS: c_uint = 0x80;
pub const TPS65917_INT3_LINE_STATE_VBUS_SHIFT: c_uint = 0x07;
pub const TPS65917_INT3_LINE_STATE_GPADC_EOC_SW: c_uint = 0x04;
pub const TPS65917_INT3_LINE_STATE_GPADC_EOC_SW_SHIFT: c_uint = 0x02;
pub const TPS65917_INT3_LINE_STATE_GPADC_AUTO_1: c_uint = 0x02;
pub const TPS65917_INT3_LINE_STATE_GPADC_AUTO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT3_LINE_STATE_GPADC_AUTO_0: c_uint = 0x01;
pub const TPS65917_INT3_LINE_STATE_GPADC_AUTO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_STATUS
pub const TPS65917_INT4_STATUS_GPIO_6: c_uint = 0x40;
pub const TPS65917_INT4_STATUS_GPIO_6_SHIFT: c_uint = 0x06;
pub const TPS65917_INT4_STATUS_GPIO_5: c_uint = 0x20;
pub const TPS65917_INT4_STATUS_GPIO_5_SHIFT: c_uint = 0x05;
pub const TPS65917_INT4_STATUS_GPIO_4: c_uint = 0x10;
pub const TPS65917_INT4_STATUS_GPIO_4_SHIFT: c_uint = 0x04;
pub const TPS65917_INT4_STATUS_GPIO_3: c_uint = 0x08;
pub const TPS65917_INT4_STATUS_GPIO_3_SHIFT: c_uint = 0x03;
pub const TPS65917_INT4_STATUS_GPIO_2: c_uint = 0x04;
pub const TPS65917_INT4_STATUS_GPIO_2_SHIFT: c_uint = 0x02;
pub const TPS65917_INT4_STATUS_GPIO_1: c_uint = 0x02;
pub const TPS65917_INT4_STATUS_GPIO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT4_STATUS_GPIO_0: c_uint = 0x01;
pub const TPS65917_INT4_STATUS_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_MASK
pub const TPS65917_INT4_MASK_GPIO_6: c_uint = 0x40;
pub const TPS65917_INT4_MASK_GPIO_6_SHIFT: c_uint = 0x06;
pub const TPS65917_INT4_MASK_GPIO_5: c_uint = 0x20;
pub const TPS65917_INT4_MASK_GPIO_5_SHIFT: c_uint = 0x05;
pub const TPS65917_INT4_MASK_GPIO_4: c_uint = 0x10;
pub const TPS65917_INT4_MASK_GPIO_4_SHIFT: c_uint = 0x04;
pub const TPS65917_INT4_MASK_GPIO_3: c_uint = 0x08;
pub const TPS65917_INT4_MASK_GPIO_3_SHIFT: c_uint = 0x03;
pub const TPS65917_INT4_MASK_GPIO_2: c_uint = 0x04;
pub const TPS65917_INT4_MASK_GPIO_2_SHIFT: c_uint = 0x02;
pub const TPS65917_INT4_MASK_GPIO_1: c_uint = 0x02;
pub const TPS65917_INT4_MASK_GPIO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT4_MASK_GPIO_0: c_uint = 0x01;
pub const TPS65917_INT4_MASK_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_LINE_STATE
pub const TPS65917_INT4_LINE_STATE_GPIO_6: c_uint = 0x40;
pub const TPS65917_INT4_LINE_STATE_GPIO_6_SHIFT: c_uint = 0x06;
pub const TPS65917_INT4_LINE_STATE_GPIO_5: c_uint = 0x20;
pub const TPS65917_INT4_LINE_STATE_GPIO_5_SHIFT: c_uint = 0x05;
pub const TPS65917_INT4_LINE_STATE_GPIO_4: c_uint = 0x10;
pub const TPS65917_INT4_LINE_STATE_GPIO_4_SHIFT: c_uint = 0x04;
pub const TPS65917_INT4_LINE_STATE_GPIO_3: c_uint = 0x08;
pub const TPS65917_INT4_LINE_STATE_GPIO_3_SHIFT: c_uint = 0x03;
pub const TPS65917_INT4_LINE_STATE_GPIO_2: c_uint = 0x04;
pub const TPS65917_INT4_LINE_STATE_GPIO_2_SHIFT: c_uint = 0x02;
pub const TPS65917_INT4_LINE_STATE_GPIO_1: c_uint = 0x02;
pub const TPS65917_INT4_LINE_STATE_GPIO_1_SHIFT: c_uint = 0x01;
pub const TPS65917_INT4_LINE_STATE_GPIO_0: c_uint = 0x01;
pub const TPS65917_INT4_LINE_STATE_GPIO_0_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_EDGE_DETECT1
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_3_RISING: c_uint = 0x80;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_3_RISING_SHIFT: c_uint = 0x07;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_3_FALLING: c_uint = 0x40;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_3_FALLING_SHIFT: c_uint = 0x06;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_2_RISING: c_uint = 0x20;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_2_RISING_SHIFT: c_uint = 0x05;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_2_FALLING: c_uint = 0x10;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_2_FALLING_SHIFT: c_uint = 0x04;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_1_RISING: c_uint = 0x08;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_1_RISING_SHIFT: c_uint = 0x03;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_1_FALLING: c_uint = 0x04;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_1_FALLING_SHIFT: c_uint = 0x02;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_0_RISING: c_uint = 0x02;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_0_RISING_SHIFT: c_uint = 0x01;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_0_FALLING: c_uint = 0x01;
pub const TPS65917_INT4_EDGE_DETECT1_GPIO_0_FALLING_SHIFT: c_uint = 0x00;
// Bit definitions for INT4_EDGE_DETECT2
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_6_RISING: c_uint = 0x20;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_6_RISING_SHIFT: c_uint = 0x05;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_6_FALLING: c_uint = 0x10;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_6_FALLING_SHIFT: c_uint = 0x04;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_5_RISING: c_uint = 0x08;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_5_RISING_SHIFT: c_uint = 0x03;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_5_FALLING: c_uint = 0x04;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_5_FALLING_SHIFT: c_uint = 0x02;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_4_RISING: c_uint = 0x02;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_4_RISING_SHIFT: c_uint = 0x01;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_4_FALLING: c_uint = 0x01;
pub const TPS65917_INT4_EDGE_DETECT2_GPIO_4_FALLING_SHIFT: c_uint = 0x00;
// Bit definitions for INT_CTRL
pub const TPS65917_INT_CTRL_INT_PENDING: c_uint = 0x04;
pub const TPS65917_INT_CTRL_INT_PENDING_SHIFT: c_uint = 0x02;
pub const TPS65917_INT_CTRL_INT_CLEAR: c_uint = 0x01;
pub const TPS65917_INT_CTRL_INT_CLEAR_SHIFT: c_uint = 0x00;
// TPS65917 SMPS Registers
// Registers for function SMPS
pub const TPS65917_SMPS1_CTRL: c_uint = 0x00;
pub const TPS65917_SMPS1_FORCE: c_uint = 0x02;
pub const TPS65917_SMPS1_VOLTAGE: c_uint = 0x03;
pub const TPS65917_SMPS2_CTRL: c_uint = 0x04;
pub const TPS65917_SMPS2_FORCE: c_uint = 0x06;
pub const TPS65917_SMPS2_VOLTAGE: c_uint = 0x07;
pub const TPS65917_SMPS3_CTRL: c_uint = 0x0C;
pub const TPS65917_SMPS3_FORCE: c_uint = 0x0E;
pub const TPS65917_SMPS3_VOLTAGE: c_uint = 0x0F;
pub const TPS65917_SMPS4_CTRL: c_uint = 0x10;
pub const TPS65917_SMPS4_VOLTAGE: c_uint = 0x13;
pub const TPS65917_SMPS5_CTRL: c_uint = 0x18;
pub const TPS65917_SMPS5_VOLTAGE: c_uint = 0x1B;
pub const TPS65917_SMPS_CTRL: c_uint = 0x24;
pub const TPS65917_SMPS_PD_CTRL: c_uint = 0x25;
pub const TPS65917_SMPS_THERMAL_EN: c_uint = 0x27;
pub const TPS65917_SMPS_THERMAL_STATUS: c_uint = 0x28;
pub const TPS65917_SMPS_SHORT_STATUS: c_uint = 0x29;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN: c_uint = 0x2A;
pub const TPS65917_SMPS_POWERGOOD_MASK1: c_uint = 0x2B;
pub const TPS65917_SMPS_POWERGOOD_MASK2: c_uint = 0x2C;
// Bit definitions for SMPS1_CTRL
pub const TPS65917_SMPS1_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_SMPS1_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS1_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const TPS65917_SMPS1_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS1_CTRL_STATUS_MASK: c_uint = 0x30;
pub const TPS65917_SMPS1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS1_CTRL_MODE_SLEEP_MASK: c_uint = 0x0C;
pub const TPS65917_SMPS1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_SMPS1_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const TPS65917_SMPS1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS1_FORCE
pub const TPS65917_SMPS1_FORCE_CMD: c_uint = 0x80;
pub const TPS65917_SMPS1_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS1_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS1_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS1_VOLTAGE
pub const TPS65917_SMPS1_VOLTAGE_RANGE: c_uint = 0x80;
pub const TPS65917_SMPS1_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS1_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS1_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS2_CTRL
pub const TPS65917_SMPS2_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_SMPS2_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS2_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const TPS65917_SMPS2_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS2_CTRL_STATUS_MASK: c_uint = 0x30;
pub const TPS65917_SMPS2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS2_CTRL_MODE_SLEEP_MASK: c_uint = 0x0C;
pub const TPS65917_SMPS2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_SMPS2_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const TPS65917_SMPS2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS2_FORCE
pub const TPS65917_SMPS2_FORCE_CMD: c_uint = 0x80;
pub const TPS65917_SMPS2_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS2_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS2_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS2_VOLTAGE
pub const TPS65917_SMPS2_VOLTAGE_RANGE: c_uint = 0x80;
pub const TPS65917_SMPS2_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS2_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS2_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS3_CTRL
pub const TPS65917_SMPS3_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_SMPS3_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS3_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const TPS65917_SMPS3_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS3_CTRL_STATUS_MASK: c_uint = 0x30;
pub const TPS65917_SMPS3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS3_CTRL_MODE_SLEEP_MASK: c_uint = 0x0C;
pub const TPS65917_SMPS3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_SMPS3_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const TPS65917_SMPS3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS3_FORCE
pub const TPS65917_SMPS3_FORCE_CMD: c_uint = 0x80;
pub const TPS65917_SMPS3_FORCE_CMD_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS3_FORCE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS3_FORCE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS3_VOLTAGE
pub const TPS65917_SMPS3_VOLTAGE_RANGE: c_uint = 0x80;
pub const TPS65917_SMPS3_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS3_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS3_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS4_CTRL
pub const TPS65917_SMPS4_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_SMPS4_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS4_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const TPS65917_SMPS4_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS4_CTRL_STATUS_MASK: c_uint = 0x30;
pub const TPS65917_SMPS4_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS4_CTRL_MODE_SLEEP_MASK: c_uint = 0x0C;
pub const TPS65917_SMPS4_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_SMPS4_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const TPS65917_SMPS4_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS4_VOLTAGE
pub const TPS65917_SMPS4_VOLTAGE_RANGE: c_uint = 0x80;
pub const TPS65917_SMPS4_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS4_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS4_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS5_CTRL
pub const TPS65917_SMPS5_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_SMPS5_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS5_CTRL_ROOF_FLOOR_EN: c_uint = 0x40;
pub const TPS65917_SMPS5_CTRL_ROOF_FLOOR_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS5_CTRL_STATUS_MASK: c_uint = 0x30;
pub const TPS65917_SMPS5_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS5_CTRL_MODE_SLEEP_MASK: c_uint = 0x0C;
pub const TPS65917_SMPS5_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_SMPS5_CTRL_MODE_ACTIVE_MASK: c_uint = 0x03;
pub const TPS65917_SMPS5_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS5_VOLTAGE
pub const TPS65917_SMPS5_VOLTAGE_RANGE: c_uint = 0x80;
pub const TPS65917_SMPS5_VOLTAGE_RANGE_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS5_VOLTAGE_VSEL_MASK: c_uint = 0x7F;
pub const TPS65917_SMPS5_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_CTRL
pub const TPS65917_SMPS_CTRL_SMPS1_SMPS12_EN: c_uint = 0x10;
pub const TPS65917_SMPS_CTRL_SMPS1_SMPS12_EN_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_CTRL_SMPS12_PHASE_CTRL: c_uint = 0x03;
pub const TPS65917_SMPS_CTRL_SMPS12_PHASE_CTRL_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_PD_CTRL
pub const TPS65917_SMPS_PD_CTRL_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_PD_CTRL_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_PD_CTRL_SMPS4: c_uint = 0x10;
pub const TPS65917_SMPS_PD_CTRL_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_PD_CTRL_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_PD_CTRL_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_PD_CTRL_SMPS2: c_uint = 0x02;
pub const TPS65917_SMPS_PD_CTRL_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_SMPS_PD_CTRL_SMPS1: c_uint = 0x01;
pub const TPS65917_SMPS_PD_CTRL_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_THERMAL_EN
pub const TPS65917_SMPS_THERMAL_EN_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_THERMAL_EN_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_THERMAL_EN_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_THERMAL_EN_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_THERMAL_EN_SMPS12: c_uint = 0x01;
pub const TPS65917_SMPS_THERMAL_EN_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_THERMAL_STATUS
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS12: c_uint = 0x01;
pub const TPS65917_SMPS_THERMAL_STATUS_SMPS12_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_SHORT_STATUS
pub const TPS65917_SMPS_SHORT_STATUS_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS4: c_uint = 0x10;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS2: c_uint = 0x02;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS1: c_uint = 0x01;
pub const TPS65917_SMPS_SHORT_STATUS_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_NEGATIVE_CURRENT_LIMIT_EN
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS4: c_uint = 0x10;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS2: c_uint = 0x02;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS1: c_uint = 0x01;
pub const TPS65917_SMPS_NEGATIVE_CURRENT_LIMIT_EN_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_POWERGOOD_MASK1
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS5: c_uint = 0x40;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS4: c_uint = 0x10;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS3: c_uint = 0x08;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS2: c_uint = 0x02;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS1: c_uint = 0x01;
pub const TPS65917_SMPS_POWERGOOD_MASK1_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for SMPS_POWERGOOD_MASK2
pub const TPS65917_SMPS_POWERGOOD_MASK2_POWERGOOD_TYPE_SELECT: c_uint = 0x80;
pub const TPS65917_SMPS_POWERGOOD_MASK2_POWERGOOD_TYPE_SELECT_SHIFT: c_uint = 0x07;
pub const TPS65917_SMPS_POWERGOOD_MASK2_OVC_ALARM_SHIFT: c_uint = 0x10;
pub const TPS65917_SMPS_POWERGOOD_MASK2_OVC_ALARM: c_uint = 0x04;
// Bit definitions for SMPS_PLL_CTRL
pub const TPS65917_SMPS_PLL_CTRL_PLL_EN_PLL_BYPASS_SHIFT: c_uint = 0x08;
pub const TPS65917_SMPS_PLL_CTRL_PLL_PLL_EN_BYPASS: c_uint = 0x03;
pub const TPS65917_SMPS_PLL_CTRL_PLL_PLL_BYPASS_CLK_SHIFT: c_uint = 0x04;
pub const TPS65917_SMPS_PLL_CTRL_PLL_PLL_BYPASS_CLK: c_uint = 0x02;
// Registers for function LDO
pub const TPS65917_LDO1_CTRL: c_uint = 0x00;
pub const TPS65917_LDO1_VOLTAGE: c_uint = 0x01;
pub const TPS65917_LDO2_CTRL: c_uint = 0x02;
pub const TPS65917_LDO2_VOLTAGE: c_uint = 0x03;
pub const TPS65917_LDO3_CTRL: c_uint = 0x04;
pub const TPS65917_LDO3_VOLTAGE: c_uint = 0x05;
pub const TPS65917_LDO4_CTRL: c_uint = 0x0E;
pub const TPS65917_LDO4_VOLTAGE: c_uint = 0x0F;
pub const TPS65917_LDO5_CTRL: c_uint = 0x12;
pub const TPS65917_LDO5_VOLTAGE: c_uint = 0x13;
pub const TPS65917_LDO_PD_CTRL1: c_uint = 0x1B;
pub const TPS65917_LDO_PD_CTRL2: c_uint = 0x1C;
pub const TPS65917_LDO_SHORT_STATUS1: c_uint = 0x1D;
pub const TPS65917_LDO_SHORT_STATUS2: c_uint = 0x1E;
pub const TPS65917_LDO_PD_CTRL3: c_uint = 0x2D;
pub const TPS65917_LDO_SHORT_STATUS3: c_uint = 0x2E;
// Bit definitions for LDO1_CTRL
pub const TPS65917_LDO1_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_LDO1_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO1_CTRL_BYPASS_EN: c_uint = 0x40;
pub const TPS65917_LDO1_CTRL_BYPASS_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_LDO1_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_LDO1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_LDO1_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_LDO1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO1_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_LDO1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO1_VOLTAGE
pub const TPS65917_LDO1_VOLTAGE_VSEL_MASK: c_uint = 0x2F;
pub const TPS65917_LDO1_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO2_CTRL
pub const TPS65917_LDO2_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_LDO2_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO2_CTRL_BYPASS_EN: c_uint = 0x40;
pub const TPS65917_LDO2_CTRL_BYPASS_EN_SHIFT: c_uint = 0x06;
pub const TPS65917_LDO2_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_LDO2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_LDO2_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_LDO2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO2_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_LDO2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO2_VOLTAGE
pub const TPS65917_LDO2_VOLTAGE_VSEL_MASK: c_uint = 0x2F;
pub const TPS65917_LDO2_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO3_CTRL
pub const TPS65917_LDO3_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_LDO3_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO3_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_LDO3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_LDO3_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_LDO3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO3_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_LDO3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO3_VOLTAGE
pub const TPS65917_LDO3_VOLTAGE_VSEL_MASK: c_uint = 0x2F;
pub const TPS65917_LDO3_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO4_CTRL
pub const TPS65917_LDO4_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_LDO4_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO4_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_LDO4_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_LDO4_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_LDO4_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO4_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_LDO4_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO4_VOLTAGE
pub const TPS65917_LDO4_VOLTAGE_VSEL_MASK: c_uint = 0x2F;
pub const TPS65917_LDO4_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO5_CTRL
pub const TPS65917_LDO5_CTRL_WR_S: c_uint = 0x80;
pub const TPS65917_LDO5_CTRL_WR_S_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO5_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_LDO5_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_LDO5_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_LDO5_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO5_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_LDO5_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for LDO5_VOLTAGE
pub const TPS65917_LDO5_VOLTAGE_VSEL_MASK: c_uint = 0x2F;
pub const TPS65917_LDO5_VOLTAGE_VSEL_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_PD_CTRL1
pub const TPS65917_LDO_PD_CTRL1_LDO4: c_uint = 0x80;
pub const TPS65917_LDO_PD_CTRL1_LDO4_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO_PD_CTRL1_LDO2: c_uint = 0x02;
pub const TPS65917_LDO_PD_CTRL1_LDO2_SHIFT: c_uint = 0x01;
pub const TPS65917_LDO_PD_CTRL1_LDO1: c_uint = 0x01;
pub const TPS65917_LDO_PD_CTRL1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_PD_CTRL2
pub const TPS65917_LDO_PD_CTRL2_LDO3: c_uint = 0x04;
pub const TPS65917_LDO_PD_CTRL2_LDO3_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO_PD_CTRL2_LDO5: c_uint = 0x02;
pub const TPS65917_LDO_PD_CTRL2_LDO5_SHIFT: c_uint = 0x01;
// Bit definitions for LDO_PD_CTRL3
pub const TPS65917_LDO_PD_CTRL2_LDOVANA: c_uint = 0x80;
pub const TPS65917_LDO_PD_CTRL2_LDOVANA_SHIFT: c_uint = 0x07;
// Bit definitions for LDO_SHORT_STATUS1
pub const TPS65917_LDO_SHORT_STATUS1_LDO4: c_uint = 0x80;
pub const TPS65917_LDO_SHORT_STATUS1_LDO4_SHIFT: c_uint = 0x07;
pub const TPS65917_LDO_SHORT_STATUS1_LDO2: c_uint = 0x02;
pub const TPS65917_LDO_SHORT_STATUS1_LDO2_SHIFT: c_uint = 0x01;
pub const TPS65917_LDO_SHORT_STATUS1_LDO1: c_uint = 0x01;
pub const TPS65917_LDO_SHORT_STATUS1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for LDO_SHORT_STATUS2
pub const TPS65917_LDO_SHORT_STATUS2_LDO3: c_uint = 0x04;
pub const TPS65917_LDO_SHORT_STATUS2_LDO3_SHIFT: c_uint = 0x02;
pub const TPS65917_LDO_SHORT_STATUS2_LDO5: c_uint = 0x02;
pub const TPS65917_LDO_SHORT_STATUS2_LDO5_SHIFT: c_uint = 0x01;
// Bit definitions for LDO_SHORT_STATUS2
pub const TPS65917_LDO_SHORT_STATUS2_LDOVANA: c_uint = 0x80;
pub const TPS65917_LDO_SHORT_STATUS2_LDOVANA_SHIFT: c_uint = 0x07;
// Bit definitions for REGEN1_CTRL
pub const TPS65917_REGEN1_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_REGEN1_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_REGEN1_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_REGEN1_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_REGEN1_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_REGEN1_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for PLLEN_CTRL
pub const TPS65917_PLLEN_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_PLLEN_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_PLLEN_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_PLLEN_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_PLLEN_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_PLLEN_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for REGEN2_CTRL
pub const TPS65917_REGEN2_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_REGEN2_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_REGEN2_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_REGEN2_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_REGEN2_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_REGEN2_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_RES_ASSIGN
pub const TPS65917_NSLEEP_RES_ASSIGN_PLL_EN: c_uint = 0x08;
pub const TPS65917_NSLEEP_RES_ASSIGN_PLL_EN_SHIFT: c_uint = 0x03;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN3: c_uint = 0x04;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x02;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const TPS65917_NSLEEP_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_SMPS_ASSIGN
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS5: c_uint = 0x40;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS4: c_uint = 0x10;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS3: c_uint = 0x08;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS2: c_uint = 0x02;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS1: c_uint = 0x01;
pub const TPS65917_NSLEEP_SMPS_ASSIGN_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_LDO_ASSIGN1
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO4: c_uint = 0x80;
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x07;
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const TPS65917_NSLEEP_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for NSLEEP_LDO_ASSIGN2
pub const TPS65917_NSLEEP_LDO_ASSIGN2_LDO3: c_uint = 0x04;
pub const TPS65917_NSLEEP_LDO_ASSIGN2_LDO3_SHIFT: c_uint = 0x02;
pub const TPS65917_NSLEEP_LDO_ASSIGN2_LDO5: c_uint = 0x02;
pub const TPS65917_NSLEEP_LDO_ASSIGN2_LDO5_SHIFT: c_uint = 0x01;
// Bit definitions for ENABLE1_RES_ASSIGN
pub const TPS65917_ENABLE1_RES_ASSIGN_PLLEN: c_uint = 0x08;
pub const TPS65917_ENABLE1_RES_ASSIGN_PLLEN_SHIFT: c_uint = 0x03;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN3: c_uint = 0x04;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x02;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const TPS65917_ENABLE1_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_SMPS_ASSIGN
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS5: c_uint = 0x40;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS4: c_uint = 0x10;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS3: c_uint = 0x08;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS2: c_uint = 0x02;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS1: c_uint = 0x01;
pub const TPS65917_ENABLE1_SMPS_ASSIGN_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_LDO_ASSIGN1
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO4: c_uint = 0x80;
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x07;
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const TPS65917_ENABLE1_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE1_LDO_ASSIGN2
pub const TPS65917_ENABLE1_LDO_ASSIGN2_LDO3: c_uint = 0x04;
pub const TPS65917_ENABLE1_LDO_ASSIGN2_LDO3_SHIFT: c_uint = 0x02;
pub const TPS65917_ENABLE1_LDO_ASSIGN2_LDO5: c_uint = 0x02;
pub const TPS65917_ENABLE1_LDO_ASSIGN2_LDO5_SHIFT: c_uint = 0x01;
// Bit definitions for ENABLE2_RES_ASSIGN
pub const TPS65917_ENABLE2_RES_ASSIGN_PLLEN: c_uint = 0x08;
pub const TPS65917_ENABLE2_RES_ASSIGN_PLLEN_SHIFT: c_uint = 0x03;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN3: c_uint = 0x04;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN3_SHIFT: c_uint = 0x02;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN2: c_uint = 0x02;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN1: c_uint = 0x01;
pub const TPS65917_ENABLE2_RES_ASSIGN_REGEN1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_SMPS_ASSIGN
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS5: c_uint = 0x40;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS5_SHIFT: c_uint = 0x06;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS4: c_uint = 0x10;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS4_SHIFT: c_uint = 0x04;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS3: c_uint = 0x08;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS3_SHIFT: c_uint = 0x03;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS2: c_uint = 0x02;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS1: c_uint = 0x01;
pub const TPS65917_ENABLE2_SMPS_ASSIGN_SMPS1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_LDO_ASSIGN1
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO4: c_uint = 0x80;
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO4_SHIFT: c_uint = 0x07;
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO2: c_uint = 0x02;
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO2_SHIFT: c_uint = 0x01;
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO1: c_uint = 0x01;
pub const TPS65917_ENABLE2_LDO_ASSIGN1_LDO1_SHIFT: c_uint = 0x00;
// Bit definitions for ENABLE2_LDO_ASSIGN2
pub const TPS65917_ENABLE2_LDO_ASSIGN2_LDO3: c_uint = 0x04;
pub const TPS65917_ENABLE2_LDO_ASSIGN2_LDO3_SHIFT: c_uint = 0x02;
pub const TPS65917_ENABLE2_LDO_ASSIGN2_LDO5: c_uint = 0x02;
pub const TPS65917_ENABLE2_LDO_ASSIGN2_LDO5_SHIFT: c_uint = 0x01;
// Bit definitions for REGEN3_CTRL
pub const TPS65917_REGEN3_CTRL_STATUS: c_uint = 0x10;
pub const TPS65917_REGEN3_CTRL_STATUS_SHIFT: c_uint = 0x04;
pub const TPS65917_REGEN3_CTRL_MODE_SLEEP: c_uint = 0x04;
pub const TPS65917_REGEN3_CTRL_MODE_SLEEP_SHIFT: c_uint = 0x02;
pub const TPS65917_REGEN3_CTRL_MODE_ACTIVE: c_uint = 0x01;
pub const TPS65917_REGEN3_CTRL_MODE_ACTIVE_SHIFT: c_uint = 0x00;
// POWERHOLD Mask field for PRIMARY_SECONDARY_PAD2 register
pub const TPS65917_PRIMARY_SECONDARY_PAD2_GPIO_5_MASK: c_uint = 0xC;
// Registers for function RESOURCE
pub const TPS65917_REGEN1_CTRL: c_uint = 0x2;
pub const TPS65917_PLLEN_CTRL: c_uint = 0x3;
pub const TPS65917_NSLEEP_RES_ASSIGN: c_uint = 0x6;
pub const TPS65917_NSLEEP_SMPS_ASSIGN: c_uint = 0x7;
pub const TPS65917_NSLEEP_LDO_ASSIGN1: c_uint = 0x8;
pub const TPS65917_NSLEEP_LDO_ASSIGN2: c_uint = 0x9;
pub const TPS65917_ENABLE1_RES_ASSIGN: c_uint = 0xA;
pub const TPS65917_ENABLE1_SMPS_ASSIGN: c_uint = 0xB;
pub const TPS65917_ENABLE1_LDO_ASSIGN1: c_uint = 0xC;
pub const TPS65917_ENABLE1_LDO_ASSIGN2: c_uint = 0xD;
pub const TPS65917_ENABLE2_RES_ASSIGN: c_uint = 0xE;
pub const TPS65917_ENABLE2_SMPS_ASSIGN: c_uint = 0xF;
pub const TPS65917_ENABLE2_LDO_ASSIGN1: c_uint = 0x10;
pub const TPS65917_ENABLE2_LDO_ASSIGN2: c_uint = 0x11;
pub const TPS65917_REGEN2_CTRL: c_uint = 0x12;
pub const TPS65917_REGEN3_CTRL: c_uint = 0x13;
extern "C" {
    pub fn regmap_read(_arg: palmas->regmap[slave_id], _arg: addr, _arg: val) -> return;
}
extern "C" {
    pub fn regmap_write(_arg: palmas->regmap[slave_id], _arg: addr, _arg: value) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: palmas->regmap[slave_id], _arg: addr, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn regmap_irq_get_virq(_arg: palmas->irq_data, _arg: irq) -> return;
}
