//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/mediatek/auxadc_thermal.c
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
// Copyright (c) 2015 MediaTek Inc.
// Author: Hanyi Wu <hanyi.wu@mediatek.com>
// Sascha Hauer <s.hauer@pengutronix.de>
// Dawei Chien <dawei.chien@mediatek.com>
// Louis Yu <louis.yu@mediatek.com>
//

// AUXADC Registers
pub const AUXADC_CON1_SET_V: c_uint = 0x008;
pub const AUXADC_CON1_CLR_V: c_uint = 0x00c;
pub const AUXADC_CON2_V: c_uint = 0x010;

pub const APMIXED_SYS_TS_CON0: c_uint = 0x600;
pub const APMIXED_SYS_TS_CON1: c_uint = 0x604;
// Thermal Controller Registers
pub const TEMP_MONCTL0: c_uint = 0x000;
pub const TEMP_MONCTL1: c_uint = 0x004;
pub const TEMP_MONCTL2: c_uint = 0x008;
pub const TEMP_MONIDET0: c_uint = 0x014;
pub const TEMP_MONIDET1: c_uint = 0x018;
pub const TEMP_MSRCTL0: c_uint = 0x038;
pub const TEMP_MSRCTL1: c_uint = 0x03c;
pub const TEMP_AHBPOLL: c_uint = 0x040;
pub const TEMP_AHBTO: c_uint = 0x044;
pub const TEMP_ADCPNP0: c_uint = 0x048;
pub const TEMP_ADCPNP1: c_uint = 0x04c;
pub const TEMP_ADCPNP2: c_uint = 0x050;
pub const TEMP_ADCPNP3: c_uint = 0x0b4;
pub const TEMP_ADCMUX: c_uint = 0x054;
pub const TEMP_ADCEN: c_uint = 0x060;
pub const TEMP_PNPMUXADDR: c_uint = 0x064;
pub const TEMP_ADCMUXADDR: c_uint = 0x068;
pub const TEMP_ADCENADDR: c_uint = 0x074;
pub const TEMP_ADCVALIDADDR: c_uint = 0x078;
pub const TEMP_ADCVOLTADDR: c_uint = 0x07c;
pub const TEMP_RDCTRL: c_uint = 0x080;
pub const TEMP_ADCVALIDMASK: c_uint = 0x084;
pub const TEMP_ADCVOLTAGESHIFT: c_uint = 0x088;
pub const TEMP_ADCWRITECTRL: c_uint = 0x08c;
pub const TEMP_MSR0: c_uint = 0x090;
pub const TEMP_MSR1: c_uint = 0x094;
pub const TEMP_MSR2: c_uint = 0x098;
pub const TEMP_MSR3: c_uint = 0x0B8;
pub const TEMP_SPARE0: c_uint = 0x0f0;
pub const TEMP_ADCPNP0_1: c_uint = 0x148;
pub const TEMP_ADCPNP1_1: c_uint = 0x14c;
pub const TEMP_ADCPNP2_1: c_uint = 0x150;
pub const TEMP_MSR0_1: c_uint = 0x190;
pub const TEMP_MSR1_1: c_uint = 0x194;
pub const TEMP_MSR2_1: c_uint = 0x198;
pub const TEMP_ADCPNP3_1: c_uint = 0x1b4;
pub const TEMP_MSR3_1: c_uint = 0x1B8;
pub const PTPCORESEL: c_uint = 0x400;

// MT8173 thermal sensors
pub const MT8173_TS1: c_int = 0;
pub const MT8173_TS2: c_int = 1;
pub const MT8173_TS3: c_int = 2;
pub const MT8173_TS4: c_int = 3;
pub const MT8173_TSABB: c_int = 4;
// AUXADC channel 11 is used for the temperature sensors
pub const MT8173_TEMP_AUXADC_CHANNEL: c_int = 11;
// The total number of temperature sensors in the MT8173
pub const MT8173_NUM_SENSORS: c_int = 5;
// The number of banks in the MT8173
pub const MT8173_NUM_ZONES: c_int = 4;
// The number of sensing points per bank
pub const MT8173_NUM_SENSORS_PER_ZONE: c_int = 4;
// The number of controller in the MT8173
pub const MT8173_NUM_CONTROLLER: c_int = 1;
// The calibration coefficient of sensor
pub const MT8173_CALIBRATION: c_int = 165;
// Valid temperatures range

pub const MT8173_TEMP_MAX: c_int = 150000;
//
// Layout of the fuses providing the calibration data
// These macros could be used for MT8183, MT8173, MT2701, and MT2712.
// MT8183 has 6 sensors and needs 6 VTS calibration data.
// MT8173 has 5 sensors and needs 5 VTS calibration data.
// MT2701 has 3 sensors and needs 3 VTS calibration data.
// MT2712 has 4 sensors and needs 4 VTS calibration data.
//

//
// Layout of the fuses providing the calibration data
// These macros could be used for MT7622.
//

//
// Layout of the fuses providing the calibration data
// These macros can be used for MT7981 and MT7986.
//

    enum {
    VTS1,
    VTS2,
    VTS3,
    VTS4,
    VTS5,
    VTSABB,
    MAX_NUM_VTS,
    };
    enum mtk_thermal_version {
    MTK_THERMAL_V1 = 1,
    MTK_THERMAL_V2,
    MTK_THERMAL_V3,
    };
// MT2701 thermal sensors
pub const MT2701_TS1: c_int = 0;
pub const MT2701_TS2: c_int = 1;
pub const MT2701_TSABB: c_int = 2;
// AUXADC channel 11 is used for the temperature sensors
pub const MT2701_TEMP_AUXADC_CHANNEL: c_int = 11;
// The total number of temperature sensors in the MT2701
pub const MT2701_NUM_SENSORS: c_int = 3;
// The number of sensing points per bank
pub const MT2701_NUM_SENSORS_PER_ZONE: c_int = 3;
// The number of controller in the MT2701
pub const MT2701_NUM_CONTROLLER: c_int = 1;
// The calibration coefficient of sensor
pub const MT2701_CALIBRATION: c_int = 165;
// MT2712 thermal sensors
pub const MT2712_TS1: c_int = 0;
pub const MT2712_TS2: c_int = 1;
pub const MT2712_TS3: c_int = 2;
pub const MT2712_TS4: c_int = 3;
// AUXADC channel 11 is used for the temperature sensors
pub const MT2712_TEMP_AUXADC_CHANNEL: c_int = 11;
// The total number of temperature sensors in the MT2712
pub const MT2712_NUM_SENSORS: c_int = 4;
// The number of sensing points per bank
pub const MT2712_NUM_SENSORS_PER_ZONE: c_int = 4;
// The number of controller in the MT2712
pub const MT2712_NUM_CONTROLLER: c_int = 1;
// The calibration coefficient of sensor
pub const MT2712_CALIBRATION: c_int = 165;
pub const MT7622_TEMP_AUXADC_CHANNEL: c_int = 11;
pub const MT7622_NUM_SENSORS: c_int = 1;
pub const MT7622_NUM_ZONES: c_int = 1;
pub const MT7622_NUM_SENSORS_PER_ZONE: c_int = 1;
pub const MT7622_TS1: c_int = 0;
pub const MT7622_NUM_CONTROLLER: c_int = 1;
// The maximum number of banks
pub const MAX_NUM_ZONES: c_int = 8;
// The calibration coefficient of sensor
pub const MT7622_CALIBRATION: c_int = 165;
// MT8183 thermal sensors
pub const MT8183_TS1: c_int = 0;
pub const MT8183_TS2: c_int = 1;
pub const MT8183_TS3: c_int = 2;
pub const MT8183_TS4: c_int = 3;
pub const MT8183_TS5: c_int = 4;
pub const MT8183_TSABB: c_int = 5;
// AUXADC channel  is used for the temperature sensors
pub const MT8183_TEMP_AUXADC_CHANNEL: c_int = 11;
// The total number of temperature sensors in the MT8183
pub const MT8183_NUM_SENSORS: c_int = 6;
// The number of banks in the MT8183
pub const MT8183_NUM_ZONES: c_int = 1;
// The number of sensing points per bank
pub const MT8183_NUM_SENSORS_PER_ZONE: c_int = 6;
// The number of controller in the MT8183
pub const MT8183_NUM_CONTROLLER: c_int = 2;
// The calibration coefficient of sensor
pub const MT8183_CALIBRATION: c_int = 153;
// AUXADC channel 11 is used for the temperature sensors
pub const MT7986_TEMP_AUXADC_CHANNEL: c_int = 11;
// The total number of temperature sensors in the MT7986
pub const MT7986_NUM_SENSORS: c_int = 1;
// The number of banks in the MT7986
pub const MT7986_NUM_ZONES: c_int = 1;
// The number of sensing points per bank
pub const MT7986_NUM_SENSORS_PER_ZONE: c_int = 1;
// MT7986 thermal sensors
pub const MT7986_TS1: c_int = 0;
// The number of controller in the MT7986
pub const MT7986_NUM_CONTROLLER: c_int = 1;
// The calibration coefficient of sensor
pub const MT7986_CALIBRATION: c_int = 165;
// MT8365
pub const MT8365_TEMP_AUXADC_CHANNEL: c_int = 11;
pub const MT8365_CALIBRATION: c_int = 164;
pub const MT8365_NUM_CONTROLLER: c_int = 1;
pub const MT8365_NUM_BANKS: c_int = 1;
pub const MT8365_NUM_SENSORS: c_int = 3;
pub const MT8365_NUM_SENSORS_PER_ZONE: c_int = 3;
pub const MT8365_TS1: c_int = 0;
pub const MT8365_TS2: c_int = 1;
pub const MT8365_TS3: c_int = 2;
    struct mtk_thermal;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_bank_cfg {
    pub num_sensors: c_uint,
    pub sensors: *const c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_thermal_bank {
    pub mt: *mut mtk_thermal,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_thermal_data {
    pub num_banks: i32,
    pub num_sensors: i32,
    pub auxadc_channel: i32,
    pub vts_index: *const c_int,
    pub sensor_mux_values: *const c_int,
    pub msr: *const c_int,
    pub adcpnp: *const c_int,
    pub cali_val: c_int,
    pub num_controller: c_int,
    pub controller_offset: *const c_int,
    pub need_switch_bank: bool,
    pub bank_data: [thermal_bank_cfg; MAX_NUM_ZONES],
    pub version: enum mtk_thermal_version,
    pub apmixed_buffer_ctl_reg: u32,
    pub apmixed_buffer_ctl_mask: u32,
    pub apmixed_buffer_ctl_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_thermal {
    pub dev: *mut device,
    pub thermal_base: *mut void __iomem,
    pub clk_peri_therm: *mut clk,
    pub clk_auxadc: *mut clk,
// lock: for getting and putting banks
    pub lock: mutex,
// Calibration values
    pub adc_ge: i32,
    pub adc_oe: i32,
    pub degc_cali: i32,
    pub o_slope: i32,
    pub o_slope_sign: i32,
    pub vts: [i32; MAX_NUM_VTS],
    pub conf: *const mtk_thermal_data,
    pub banks: [mtk_thermal_bank; MAX_NUM_ZONES],
    pub raw): *mut *mut *mut int (raw_to_mcelsius)(struct mtk_thermal mt, int sensno, s32,
}

// MT8183 thermal sensor data
    static const int mt8183_bank_data[MT8183_NUM_SENSORS] = {
    MT8183_TS1, MT8183_TS2, MT8183_TS3, MT8183_TS4, MT8183_TS5, MT8183_TSABB
    };
    static const int mt8183_msr[MT8183_NUM_SENSORS_PER_ZONE] = {
    TEMP_MSR0_1, TEMP_MSR1_1, TEMP_MSR2_1, TEMP_MSR1, TEMP_MSR0, TEMP_MSR3_1
    };
    static const int mt8183_adcpnp[MT8183_NUM_SENSORS_PER_ZONE] = {
    TEMP_ADCPNP0_1, TEMP_ADCPNP1_1, TEMP_ADCPNP2_1,
    TEMP_ADCPNP1, TEMP_ADCPNP0, TEMP_ADCPNP3_1
    };
    static const int mt8183_mux_values[MT8183_NUM_SENSORS] = { 0, 1, 2, 3, 4, 0 };
    static const int mt8183_tc_offset[MT8183_NUM_CONTROLLER] = {0x0, 0x100};
    static const int mt8183_vts_index[MT8183_NUM_SENSORS] = {
    VTS1, VTS2, VTS3, VTS4, VTS5, VTSABB
    };
// MT8173 thermal sensor data
    static const int mt8173_bank_data[MT8173_NUM_ZONES][3] = {
    { MT8173_TS2, MT8173_TS3 },
    { MT8173_TS2, MT8173_TS4 },
    { MT8173_TS1, MT8173_TS2, MT8173_TSABB },
    { MT8173_TS2 },
    };
    static const int mt8173_msr[MT8173_NUM_SENSORS_PER_ZONE] = {
    TEMP_MSR0, TEMP_MSR1, TEMP_MSR2, TEMP_MSR3
    };
    static const int mt8173_adcpnp[MT8173_NUM_SENSORS_PER_ZONE] = {
    TEMP_ADCPNP0, TEMP_ADCPNP1, TEMP_ADCPNP2, TEMP_ADCPNP3
    };
    static const int mt8173_mux_values[MT8173_NUM_SENSORS] = { 0, 1, 2, 3, 16 };
    static const int mt8173_tc_offset[MT8173_NUM_CONTROLLER] = { 0x0, };
    static const int mt8173_vts_index[MT8173_NUM_SENSORS] = {
    VTS1, VTS2, VTS3, VTS4, VTSABB
    };
// MT2701 thermal sensor data
    static const int mt2701_bank_data[MT2701_NUM_SENSORS] = {
    MT2701_TS1, MT2701_TS2, MT2701_TSABB
    };
    static const int mt2701_msr[MT2701_NUM_SENSORS_PER_ZONE] = {
    TEMP_MSR0, TEMP_MSR1, TEMP_MSR2
    };
    static const int mt2701_adcpnp[MT2701_NUM_SENSORS_PER_ZONE] = {
    TEMP_ADCPNP0, TEMP_ADCPNP1, TEMP_ADCPNP2
    };
    static const int mt2701_mux_values[MT2701_NUM_SENSORS] = { 0, 1, 16 };
    static const int mt2701_tc_offset[MT2701_NUM_CONTROLLER] = { 0x0, };
    static const int mt2701_vts_index[MT2701_NUM_SENSORS] = {
    VTS1, VTS2, VTS3
    };
// MT2712 thermal sensor data
    static const int mt2712_bank_data[MT2712_NUM_SENSORS] = {
    MT2712_TS1, MT2712_TS2, MT2712_TS3, MT2712_TS4
    };
    static const int mt2712_msr[MT2712_NUM_SENSORS_PER_ZONE] = {
    TEMP_MSR0, TEMP_MSR1, TEMP_MSR2, TEMP_MSR3
    };
    static const int mt2712_adcpnp[MT2712_NUM_SENSORS_PER_ZONE] = {
    TEMP_ADCPNP0, TEMP_ADCPNP1, TEMP_ADCPNP2, TEMP_ADCPNP3
    };
    static const int mt2712_mux_values[MT2712_NUM_SENSORS] = { 0, 1, 2, 3 };
    static const int mt2712_tc_offset[MT2712_NUM_CONTROLLER] = { 0x0, };
    static const int mt2712_vts_index[MT2712_NUM_SENSORS] = {
    VTS1, VTS2, VTS3, VTS4
    };
// MT7622 thermal sensor data
    static const int mt7622_bank_data[MT7622_NUM_SENSORS] = { MT7622_TS1, };
    static const int mt7622_msr[MT7622_NUM_SENSORS_PER_ZONE] = { TEMP_MSR0, };
    static const int mt7622_adcpnp[MT7622_NUM_SENSORS_PER_ZONE] = { TEMP_ADCPNP0, };
    static const int mt7622_mux_values[MT7622_NUM_SENSORS] = { 0, };
    static const int mt7622_vts_index[MT7622_NUM_SENSORS] = { VTS1 };
    static const int mt7622_tc_offset[MT7622_NUM_CONTROLLER] = { 0x0, };
// MT7986 thermal sensor data
    static const int mt7986_bank_data[MT7986_NUM_SENSORS] = { MT7986_TS1, };
    static const int mt7986_msr[MT7986_NUM_SENSORS_PER_ZONE] = { TEMP_MSR0, };
    static const int mt7986_adcpnp[MT7986_NUM_SENSORS_PER_ZONE] = { TEMP_ADCPNP0, };
    static const int mt7986_mux_values[MT7986_NUM_SENSORS] = { 0, };
    static const int mt7986_vts_index[MT7986_NUM_SENSORS] = { VTS1 };
    static const int mt7986_tc_offset[MT7986_NUM_CONTROLLER] = { 0x0, };
// MT8365 thermal sensor data
    static const int mt8365_bank_data[MT8365_NUM_SENSORS] = {
    MT8365_TS1, MT8365_TS2, MT8365_TS3
    };
    static const int mt8365_msr[MT8365_NUM_SENSORS_PER_ZONE] = {
    TEMP_MSR0, TEMP_MSR1, TEMP_MSR2
    };
    static const int mt8365_adcpnp[MT8365_NUM_SENSORS_PER_ZONE] = {
    TEMP_ADCPNP0, TEMP_ADCPNP1, TEMP_ADCPNP2
    };
    static const int mt8365_mux_values[MT8365_NUM_SENSORS] = { 0, 1, 2 };
    static const int mt8365_tc_offset[MT8365_NUM_CONTROLLER] = { 0 };
    static const int mt8365_vts_index[MT8365_NUM_SENSORS] = { VTS1, VTS2, VTS3 };
//
// The MT8173 thermal controller has four banks. Each bank can read up to
// four temperature sensors simultaneously. The MT8173 has a total of 5
// temperature sensors. We use each bank to measure a certain area of the
// SoC. Since TS2 is located centrally in the SoC it is influenced by multiple
// areas, hence is used in different banks.
//
// The thermal core only gets the maximum temperature of all banks, so
// the bank concept wouldn't be necessary here. However, the SVS (Smart
// Voltage Scaling) unit makes its decisions based on the same bank
// data, and this indeed needs the temperatures of the individual banks
// for making better decisions.
//
    static const struct mtk_thermal_data mt8173_thermal_data = {
    .auxadc_channel = MT8173_TEMP_AUXADC_CHANNEL,
    .num_banks = MT8173_NUM_ZONES,
    .num_sensors = MT8173_NUM_SENSORS,
    .vts_index = mt8173_vts_index,
    .cali_val = MT8173_CALIBRATION,
    .num_controller = MT8173_NUM_CONTROLLER,
    .controller_offset = mt8173_tc_offset,
    .need_switch_bank = true,
    .bank_data = {
    {
    .num_sensors = 2,
    .sensors = mt8173_bank_data[0],
    }, {
    .num_sensors = 2,
    .sensors = mt8173_bank_data[1],
    }, {
    .num_sensors = 3,
    .sensors = mt8173_bank_data[2],
    }, {
    .num_sensors = 1,
    .sensors = mt8173_bank_data[3],
    },
    },
    .msr = mt8173_msr,
    .adcpnp = mt8173_adcpnp,
    .sensor_mux_values = mt8173_mux_values,
    .version = MTK_THERMAL_V1,
    };
//
// The MT2701 thermal controller has one bank, which can read up to
// three temperature sensors simultaneously. The MT2701 has a total of 3
// temperature sensors.
//
// The thermal core only gets the maximum temperature of this one bank,
// so the bank concept wouldn't be necessary here. However, the SVS (Smart
// Voltage Scaling) unit makes its decisions based on the same bank
// data.
//
    static const struct mtk_thermal_data mt2701_thermal_data = {
    .auxadc_channel = MT2701_TEMP_AUXADC_CHANNEL,
    .num_banks = 1,
    .num_sensors = MT2701_NUM_SENSORS,
    .vts_index = mt2701_vts_index,
    .cali_val = MT2701_CALIBRATION,
    .num_controller = MT2701_NUM_CONTROLLER,
    .controller_offset = mt2701_tc_offset,
    .need_switch_bank = true,
    .bank_data = {
    {
    .num_sensors = 3,
    .sensors = mt2701_bank_data,
    },
    },
    .msr = mt2701_msr,
    .adcpnp = mt2701_adcpnp,
    .sensor_mux_values = mt2701_mux_values,
    .version = MTK_THERMAL_V1,
    };
//
// The MT8365 thermal controller has one bank, which can read up to
// four temperature sensors simultaneously. The MT8365 has a total of 3
// temperature sensors.
//
// The thermal core only gets the maximum temperature of this one bank,
// so the bank concept wouldn't be necessary here. However, the SVS (Smart
// Voltage Scaling) unit makes its decisions based on the same bank
// data.
//
    static const struct mtk_thermal_data mt8365_thermal_data = {
    .auxadc_channel = MT8365_TEMP_AUXADC_CHANNEL,
    .num_banks = MT8365_NUM_BANKS,
    .num_sensors = MT8365_NUM_SENSORS,
    .vts_index = mt8365_vts_index,
    .cali_val = MT8365_CALIBRATION,
    .num_controller = MT8365_NUM_CONTROLLER,
    .controller_offset = mt8365_tc_offset,
    .need_switch_bank = false,
    .bank_data = {
    {
    .num_sensors = MT8365_NUM_SENSORS,
    .sensors = mt8365_bank_data
    },
    },
    .msr = mt8365_msr,
    .adcpnp = mt8365_adcpnp,
    .sensor_mux_values = mt8365_mux_values,
    .version = MTK_THERMAL_V1,
    .apmixed_buffer_ctl_reg = APMIXED_SYS_TS_CON0,
    .apmixed_buffer_ctl_mask = (u32) ~GENMASK(29, 28),
    .apmixed_buffer_ctl_set = 0,
    };
//
// The MT2712 thermal controller has one bank, which can read up to
// four temperature sensors simultaneously. The MT2712 has a total of 4
// temperature sensors.
//
// The thermal core only gets the maximum temperature of this one bank,
// so the bank concept wouldn't be necessary here. However, the SVS (Smart
// Voltage Scaling) unit makes its decisions based on the same bank
// data.
//
    static const struct mtk_thermal_data mt2712_thermal_data = {
    .auxadc_channel = MT2712_TEMP_AUXADC_CHANNEL,
    .num_banks = 1,
    .num_sensors = MT2712_NUM_SENSORS,
    .vts_index = mt2712_vts_index,
    .cali_val = MT2712_CALIBRATION,
    .num_controller = MT2712_NUM_CONTROLLER,
    .controller_offset = mt2712_tc_offset,
    .need_switch_bank = true,
    .bank_data = {
    {
    .num_sensors = 4,
    .sensors = mt2712_bank_data,
    },
    },
    .msr = mt2712_msr,
    .adcpnp = mt2712_adcpnp,
    .sensor_mux_values = mt2712_mux_values,
    .version = MTK_THERMAL_V1,
    };
//
// MT7622 have only one sensing point which uses AUXADC Channel 11 for raw data
// access.
//
    static const struct mtk_thermal_data mt7622_thermal_data = {
    .auxadc_channel = MT7622_TEMP_AUXADC_CHANNEL,
    .num_banks = MT7622_NUM_ZONES,
    .num_sensors = MT7622_NUM_SENSORS,
    .vts_index = mt7622_vts_index,
    .cali_val = MT7622_CALIBRATION,
    .num_controller = MT7622_NUM_CONTROLLER,
    .controller_offset = mt7622_tc_offset,
    .need_switch_bank = true,
    .bank_data = {
    {
    .num_sensors = 1,
    .sensors = mt7622_bank_data,
    },
    },
    .msr = mt7622_msr,
    .adcpnp = mt7622_adcpnp,
    .sensor_mux_values = mt7622_mux_values,
    .version = MTK_THERMAL_V2,
    .apmixed_buffer_ctl_reg = APMIXED_SYS_TS_CON1,
    .apmixed_buffer_ctl_mask = GENMASK(31, 6) | BIT(3),
    .apmixed_buffer_ctl_set = BIT(0),
    };
//
// The MT8183 thermal controller has one bank for the current SW framework.
// The MT8183 has a total of 6 temperature sensors.
// There are two thermal controller to control the six sensor.
// The first one bind 2 sensor, and the other bind 4 sensors.
// The thermal core only gets the maximum temperature of all sensor, so
// the bank concept wouldn't be necessary here. However, the SVS (Smart
// Voltage Scaling) unit makes its decisions based on the same bank
// data, and this indeed needs the temperatures of the individual banks
// for making better decisions.
//
    static const struct mtk_thermal_data mt8183_thermal_data = {
    .auxadc_channel = MT8183_TEMP_AUXADC_CHANNEL,
    .num_banks = MT8183_NUM_ZONES,
    .num_sensors = MT8183_NUM_SENSORS,
    .vts_index = mt8183_vts_index,
    .cali_val = MT8183_CALIBRATION,
    .num_controller = MT8183_NUM_CONTROLLER,
    .controller_offset = mt8183_tc_offset,
    .need_switch_bank = false,
    .bank_data = {
    {
    .num_sensors = 6,
    .sensors = mt8183_bank_data,
    },
    },
    .msr = mt8183_msr,
    .adcpnp = mt8183_adcpnp,
    .sensor_mux_values = mt8183_mux_values,
    .version = MTK_THERMAL_V1,
    };
//
// MT7986 uses AUXADC Channel 11 for raw data access.
//
    static const struct mtk_thermal_data mt7986_thermal_data = {
    .auxadc_channel = MT7986_TEMP_AUXADC_CHANNEL,
    .num_banks = MT7986_NUM_ZONES,
    .num_sensors = MT7986_NUM_SENSORS,
    .vts_index = mt7986_vts_index,
    .cali_val = MT7986_CALIBRATION,
    .num_controller = MT7986_NUM_CONTROLLER,
    .controller_offset = mt7986_tc_offset,
    .need_switch_bank = true,
    .bank_data = {
    {
    .num_sensors = 1,
    .sensors = mt7986_bank_data,
    },
    },
    .msr = mt7986_msr,
    .adcpnp = mt7986_adcpnp,
    .sensor_mux_values = mt7986_mux_values,
    .version = MTK_THERMAL_V3,
    .apmixed_buffer_ctl_reg = APMIXED_SYS_TS_CON1,
    .apmixed_buffer_ctl_mask = GENMASK(31, 6) | BIT(3),
    .apmixed_buffer_ctl_set = BIT(0),
    };
#[no_mangle]
unsafe extern "C" fn mtk_thermal_temp_is_valid(temp: c_int) -> bool {
    static bool mtk_thermal_temp_is_valid(int temp)
    {
    return (temp >= MT8173_TEMP_MIN) && (temp <= MT8173_TEMP_MAX);
    }
//
// raw_to_mcelsius_v1 - convert a raw ADC value to mcelsius
// @mt:	The thermal controller
// @sensno:	sensor number
// @raw:	raw ADC value
//
// This converts the raw ADC value to mcelsius using the SoC specific
// calibration constants
//
#[no_mangle]
unsafe extern "C" fn raw_to_mcelsius_v1(mt: *mut mtk_thermal, sensno: c_int, raw: i32) -> c_int {
    static int raw_to_mcelsius_v1(struct mtk_thermal *mt, int sensno, s32 raw)
    {
    s32 tmp;
    raw &= 0xfff;
    tmp = 203450520 << 3;
    tmp /= mt.conf.cali_val + mt.o_slope;
    tmp /= 10000 + mt.adc_ge;
    tmp *= raw - mt.vts[sensno] - 3350;
    tmp >>= 3;
    return mt.degc_cali * 500 - tmp;
    }
#[no_mangle]
unsafe extern "C" fn raw_to_mcelsius_v2(mt: *mut mtk_thermal, sensno: c_int, raw: i32) -> c_int {
    static int raw_to_mcelsius_v2(struct mtk_thermal *mt, int sensno, s32 raw)
    {
    s32 format_1;
    s32 format_2;
    s32 g_oe;
    s32 g_gain;
    s32 g_x_roomt;
    s32 tmp;
    if (raw == 0)
    return 0;
    raw &= 0xfff;
    g_gain = 10000 + (((mt.adc_ge - 512) * 10000) >> 12);
    g_oe = mt.adc_oe - 512;
    format_1 = mt.vts[VTS2] + 3105 - g_oe;
    format_2 = (mt.degc_cali * 10) >> 1;
    g_x_roomt = (((format_1 * 10000) >> 12) * 10000) / g_gain;
    tmp = (((((raw - g_oe) * 10000) >> 12) * 10000) / g_gain) - g_x_roomt;
    tmp = tmp * 10 * 100 / 11;
    if (mt.o_slope_sign == 0)
    tmp = tmp / (165 - mt.o_slope);
    else
    tmp = tmp / (165 + mt.o_slope);
    return (format_2 - tmp) * 100;
    }
#[no_mangle]
unsafe extern "C" fn raw_to_mcelsius_v3(mt: *mut mtk_thermal, sensno: c_int, raw: i32) -> c_int {
    static int raw_to_mcelsius_v3(struct mtk_thermal *mt, int sensno, s32 raw)
    {
    s32 tmp;
    if (raw == 0)
    return 0;
    raw &= 0xfff;
    tmp = 100000 * 15 / 16 * 10000;
    tmp /= 4096 - 512 + mt.adc_ge;
    tmp /= 1490;
    tmp *= raw - mt.vts[sensno] - 2900;
    return mt.degc_cali * 500 - tmp;
    }
//
// mtk_thermal_get_bank - get bank
// @bank:	The bank
//
// The bank registers are banked, we have to select a bank in the
// PTPCORESEL register to access it.
//
#[no_mangle]
unsafe extern "C" fn mtk_thermal_get_bank(bank: *mut mtk_thermal_bank) {
    static void mtk_thermal_get_bank(struct mtk_thermal_bank *bank)
    {
    struct mtk_thermal *mt = bank.mt;
    u32 val;
    if (mt.conf.need_switch_bank) {
    mutex_lock(&mt.lock);
    val = readl(mt.thermal_base + PTPCORESEL);
    val &= ~0xf;
    val |= bank.id;
    writel(val, mt.thermal_base + PTPCORESEL);
    }
    }
//
// mtk_thermal_put_bank - release bank
// @bank:	The bank
//
// release a bank previously taken with mtk_thermal_get_bank,
//
#[no_mangle]
unsafe extern "C" fn mtk_thermal_put_bank(bank: *mut mtk_thermal_bank) {
    static void mtk_thermal_put_bank(struct mtk_thermal_bank *bank)
    {
    struct mtk_thermal *mt = bank.mt;
    if (mt.conf.need_switch_bank)
    mutex_unlock(&mt.lock);
    }
//
// mtk_thermal_bank_temperature - get the temperature of a bank
// @bank:	The bank
//
// The temperature of a bank is considered the maximum temperature of
// the sensors associated to the bank.
//
#[no_mangle]
unsafe extern "C" fn mtk_thermal_bank_temperature(bank: *mut mtk_thermal_bank) -> c_int {
    static int mtk_thermal_bank_temperature(struct mtk_thermal_bank *bank)
    {
    struct mtk_thermal *mt = bank.mt;
    const struct mtk_thermal_data *conf = mt.conf;
    int i, temp = INT_MIN, max = INT_MIN;
    u32 raw;
    for (i = 0; i < conf.bank_data[bank.id].num_sensors; i++) {
    raw = readl(mt.thermal_base + conf.msr[i]);
    temp = mt.raw_to_mcelsius(
    mt, conf.bank_data[bank.id].sensors[i], raw);
//
// Depending on the filt/sen intervals and ADC polling time,
// we may need up to 60 milliseconds after initialization: this
// will result in the first reading containing an out of range
// temperature value.
// Validate the reading to both address the aforementioned issue
// and to eventually avoid bogus readings during runtime in the
// event that the AUXADC gets unstable due to high EMI, etc.
//
    if (!mtk_thermal_temp_is_valid(temp))
    temp = THERMAL_TEMP_INVALID;
    if (temp > max)
    max = temp;
    }
    return max;
    }
#[no_mangle]
unsafe extern "C" fn mtk_read_temp(tz: *mut thermal_zone_device, temperature: *mut c_int) -> c_int {
    static int mtk_read_temp(struct thermal_zone_device *tz, int *temperature)
    {
    struct mtk_thermal *mt = thermal_zone_device_priv(tz);
    int i;
    let mut tempmax: c_int = INT_MIN;
    for (i = 0; i < mt.conf.num_banks; i++) {
    struct mtk_thermal_bank *bank = &mt.banks[i];
    mtk_thermal_get_bank(bank);
    tempmax = max(tempmax, mtk_thermal_bank_temperature(bank));
    mtk_thermal_put_bank(bank);
    }
// temperature = tempmax;
    return 0;
    }
    static const struct thermal_zone_device_ops mtk_thermal_ops = {
    .get_temp = mtk_read_temp,
    };
    static void mtk_thermal_init_bank(struct mtk_thermal *mt, int num,
    u32 apmixed_phys_base, u32 auxadc_phys_base,
    int ctrl_id)
    {
    struct mtk_thermal_bank *bank = &mt.banks[num];
    const struct mtk_thermal_data *conf = mt.conf;
    int i;
    let mut offset: c_int = mt.conf.controller_offset[ctrl_id];
    void __iomem *controller_base = mt.thermal_base + offset;
    bank.id = num;
    bank.mt = mt;
    mtk_thermal_get_bank(bank);
// bus clock 66M counting unit is 12 * 15.15ns * 256 = 46.540us
    writel(TEMP_MONCTL1_PERIOD_UNIT(12), controller_base + TEMP_MONCTL1);
//
// filt interval is 1 * 46.540us = 46.54us,
// sen interval is 429 * 46.540us = 19.96ms
//
    writel(TEMP_MONCTL2_FILTER_INTERVAL(1) |
    TEMP_MONCTL2_SENSOR_INTERVAL(429),
    controller_base + TEMP_MONCTL2);
// poll is set to 10u
    writel(TEMP_AHBPOLL_ADC_POLL_INTERVAL(768),
    controller_base + TEMP_AHBPOLL);
// temperature sampling control, 1 sample
    writel(0x0, controller_base + TEMP_MSRCTL0);
// exceed this polling time, IRQ would be inserted
    writel(0xffffffff, controller_base + TEMP_AHBTO);
// number of interrupts per event, 1 is enough
    writel(0x0, controller_base + TEMP_MONIDET0);
    writel(0x0, controller_base + TEMP_MONIDET1);
//
// The MT8173 thermal controller does not have its own ADC. Instead it
// uses AHB bus accesses to control the AUXADC. To do this the thermal
// controller has to be programmed with the physical addresses of the
// AUXADC registers and with the various bit positions in the AUXADC.
// Also the thermal controller controls a mux in the APMIXEDSYS register
// space.
//
// this value will be stored to TEMP_PNPMUXADDR (TEMP_SPARE0)
// automatically by hw
//
    writel(BIT(conf.auxadc_channel), controller_base + TEMP_ADCMUX);
// AHB address for auxadc mux selection
    writel(auxadc_phys_base + AUXADC_CON1_CLR_V,
    controller_base + TEMP_ADCMUXADDR);
    if (mt.conf.version == MTK_THERMAL_V1) {
// AHB address for pnp sensor mux selection
    writel(apmixed_phys_base + APMIXED_SYS_TS_CON1,
    controller_base + TEMP_PNPMUXADDR);
    }
// AHB value for auxadc enable
    writel(BIT(conf.auxadc_channel), controller_base + TEMP_ADCEN);
// AHB address for auxadc enable (channel 0 immediate mode selected)
    writel(auxadc_phys_base + AUXADC_CON1_SET_V,
    controller_base + TEMP_ADCENADDR);
// AHB address for auxadc valid bit
    writel(auxadc_phys_base + AUXADC_DATA(conf.auxadc_channel),
    controller_base + TEMP_ADCVALIDADDR);
// AHB address for auxadc voltage output
    writel(auxadc_phys_base + AUXADC_DATA(conf.auxadc_channel),
    controller_base + TEMP_ADCVOLTADDR);
// read valid & voltage are at the same register
    writel(0x0, controller_base + TEMP_RDCTRL);
// indicate where the valid bit is
    writel(TEMP_ADCVALIDMASK_VALID_HIGH | TEMP_ADCVALIDMASK_VALID_POS(12),
    controller_base + TEMP_ADCVALIDMASK);
// no shift
    writel(0x0, controller_base + TEMP_ADCVOLTAGESHIFT);
// enable auxadc mux write transaction
    writel(TEMP_ADCWRITECTRL_ADC_MUX_WRITE,
    controller_base + TEMP_ADCWRITECTRL);
    for (i = 0; i < conf.bank_data[num].num_sensors; i++)
    writel(conf.sensor_mux_values[conf.bank_data[num].sensors[i]],
    mt.thermal_base + conf.adcpnp[i]);
    writel((1 << conf.bank_data[num].num_sensors) - 1,
    controller_base + TEMP_MONCTL0);
    writel(TEMP_ADCWRITECTRL_ADC_PNP_WRITE |
    TEMP_ADCWRITECTRL_ADC_MUX_WRITE,
    controller_base + TEMP_ADCWRITECTRL);
    mtk_thermal_put_bank(bank);
    }
#[no_mangle]
unsafe extern "C" fn of_get_phys_base(np: *mut device_node) -> u64 {
    static u64 of_get_phys_base(struct device_node *np)
    {
    struct resource res;
    if (of_address_to_resource(np, 0, &res))
    return OF_BAD_ADDR;
    return res.start;
    }
#[no_mangle]
unsafe extern "C" fn mtk_thermal_extract_efuse_v1(mt: *mut mtk_thermal, buf: *mut u32) -> c_int {
    static int mtk_thermal_extract_efuse_v1(struct mtk_thermal *mt, u32 *buf)
    {
    int i;
    if (!(buf[0] & CALIB_BUF0_VALID_V1))
    return -EINVAL;
    mt.adc_ge = CALIB_BUF1_ADC_GE_V1(buf[1]);
    for (i = 0; i < mt.conf.num_sensors; i++) {
    switch (mt.conf.vts_index[i]) {
    case VTS1:
    mt.vts[VTS1] = CALIB_BUF0_VTS_TS1_V1(buf[0]);
    break;
    case VTS2:
    mt.vts[VTS2] = CALIB_BUF0_VTS_TS2_V1(buf[0]);
    break;
    case VTS3:
    mt.vts[VTS3] = CALIB_BUF1_VTS_TS3_V1(buf[1]);
    break;
    case VTS4:
    mt.vts[VTS4] = CALIB_BUF2_VTS_TS4_V1(buf[2]);
    break;
    case VTS5:
    mt.vts[VTS5] = CALIB_BUF2_VTS_TS5_V1(buf[2]);
    break;
    case VTSABB:
    mt.vts[VTSABB] =
    CALIB_BUF2_VTS_TSABB_V1(buf[2]);
    break;
    default:
    break;
    }
    }
    mt.degc_cali = CALIB_BUF0_DEGC_CALI_V1(buf[0]);
    if (CALIB_BUF1_ID_V1(buf[1]) &
    CALIB_BUF0_O_SLOPE_SIGN_V1(buf[0]))
    mt.o_slope = -CALIB_BUF0_O_SLOPE_V1(buf[0]);
    else
    mt.o_slope = CALIB_BUF0_O_SLOPE_V1(buf[0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_thermal_extract_efuse_v2(mt: *mut mtk_thermal, buf: *mut u32) -> c_int {
    static int mtk_thermal_extract_efuse_v2(struct mtk_thermal *mt, u32 *buf)
    {
    if (!CALIB_BUF1_VALID_V2(buf[1]))
    return -EINVAL;
    mt.adc_oe = CALIB_BUF0_ADC_OE_V2(buf[0]);
    mt.adc_ge = CALIB_BUF0_ADC_GE_V2(buf[0]);
    mt.degc_cali = CALIB_BUF0_DEGC_CALI_V2(buf[0]);
    mt.o_slope = CALIB_BUF0_O_SLOPE_V2(buf[0]);
    mt.vts[VTS1] = CALIB_BUF1_VTS_TS1_V2(buf[1]);
    mt.vts[VTS2] = CALIB_BUF1_VTS_TS2_V2(buf[1]);
    mt.vts[VTSABB] = CALIB_BUF1_VTS_TSABB_V2(buf[1]);
    mt.o_slope_sign = CALIB_BUF1_O_SLOPE_SIGN_V2(buf[1]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_thermal_extract_efuse_v3(mt: *mut mtk_thermal, buf: *mut u32) -> c_int {
    static int mtk_thermal_extract_efuse_v3(struct mtk_thermal *mt, u32 *buf)
    {
    if (!CALIB_BUF1_VALID_V3(buf[1]))
    return -EINVAL;
    mt.adc_ge = CALIB_BUF0_ADC_GE_V3(buf[0]);
    mt.degc_cali = CALIB_BUF0_DEGC_CALI_V3(buf[0]);
    mt.o_slope = CALIB_BUF0_O_SLOPE_V3(buf[0]);
    mt.vts[VTS1] = CALIB_BUF1_VTS_TS1_V3(buf[1]);
    mt.vts[VTS2] = CALIB_BUF1_VTS_TS2_V3(buf[1]);
    mt.vts[VTSABB] = CALIB_BUF1_VTS_TSABB_V3(buf[1]);
    mt.o_slope_sign = CALIB_BUF1_O_SLOPE_SIGN_V3(buf[1]);
    if (CALIB_BUF1_ID_V3(buf[1]) == 0)
    mt.o_slope = 0;
    return 0;
    }
    static int mtk_thermal_get_calibration_data(struct device *dev,
    struct mtk_thermal *mt)
    {
    struct nvmem_cell *cell;
    u32 *buf;
    size_t len;
    int i, ret = 0;
// Start with default values
    mt.adc_ge = 512;
    mt.adc_oe = 512;
    for (i = 0; i < mt.conf.num_sensors; i++)
    mt.vts[i] = 260;
    mt.degc_cali = 40;
    mt.o_slope = 0;
    cell = nvmem_cell_get(dev, "calibration-data");
    if (IS_ERR(cell)) {
    if (PTR_ERR(cell) == -EPROBE_DEFER)
    return PTR_ERR(cell);
    return 0;
    }
    buf = (u32 *)nvmem_cell_read(cell, &len);
    nvmem_cell_put(cell);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    if (len < 3 * sizeof(u32)) {
    dev_warn(dev, "invalid calibration data\n");
    ret = -EINVAL;
    goto out;
    }
    switch (mt.conf.version) {
    case MTK_THERMAL_V1:
    ret = mtk_thermal_extract_efuse_v1(mt, buf);
    break;
    case MTK_THERMAL_V2:
    ret = mtk_thermal_extract_efuse_v2(mt, buf);
    break;
    case MTK_THERMAL_V3:
    ret = mtk_thermal_extract_efuse_v3(mt, buf);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    if (ret) {
    dev_info(dev, "Device not calibrated, using default calibration values\n");
    ret = 0;
    }
    out:
    kfree(buf);
    return ret;
    }
    static const struct of_device_id mtk_thermal_of_match[] = {
    {
    .compatible = "mediatek,mt8173-thermal",
    .data = (void *)&mt8173_thermal_data,
    },
    {
    .compatible = "mediatek,mt2701-thermal",
    .data = (void *)&mt2701_thermal_data,
    },
    {
    .compatible = "mediatek,mt2712-thermal",
    .data = (void *)&mt2712_thermal_data,
    },
    {
    .compatible = "mediatek,mt7622-thermal",
    .data = (void *)&mt7622_thermal_data,
    },
    {
    .compatible = "mediatek,mt7986-thermal",
    .data = (void *)&mt7986_thermal_data,
    },
    {
    .compatible = "mediatek,mt8183-thermal",
    .data = (void *)&mt8183_thermal_data,
    },
    {
    .compatible = "mediatek,mt8365-thermal",
    .data = (void *)&mt8365_thermal_data,
    }, {
    },
    };
    MODULE_DEVICE_TABLE(of, mtk_thermal_of_match);
    static void mtk_thermal_turn_on_buffer(struct mtk_thermal *mt,
    void __iomem *apmixed_base)
    {
    u32 tmp;
    if (!mt.conf.apmixed_buffer_ctl_reg)
    return;
    tmp = readl(apmixed_base + mt.conf.apmixed_buffer_ctl_reg);
    tmp &= mt.conf.apmixed_buffer_ctl_mask;
    tmp |= mt.conf.apmixed_buffer_ctl_set;
    writel(tmp, apmixed_base + mt.conf.apmixed_buffer_ctl_reg);
    udelay(200);
    }
    static void mtk_thermal_release_periodic_ts(struct mtk_thermal *mt,
    void __iomem *auxadc_base)
    {
    int tmp;
    writel(0x800, auxadc_base + AUXADC_CON1_SET_V);
    writel(0x1, mt.thermal_base + TEMP_MONCTL0);
    tmp = readl(mt.thermal_base + TEMP_MSRCTL1);
    writel((tmp & (~0x10e)), mt.thermal_base + TEMP_MSRCTL1);
    }
#[no_mangle]
unsafe extern "C" fn mtk_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_thermal_probe(struct platform_device *pdev)
    {
    int ret, i, ctrl_id;
    struct device_node *auxadc, *apmixedsys, *np = pdev.dev.of_node;
    struct mtk_thermal *mt;
    u64 auxadc_phys_base, apmixed_phys_base;
    struct thermal_zone_device *tzdev;
    void __iomem *apmixed_base, *auxadc_base;
    mt = devm_kzalloc(&pdev.dev, sizeof(*mt), GFP_KERNEL);
    if (!mt)
    return -ENOMEM;
    mt.conf = of_device_get_match_data(&pdev.dev);
    mt.thermal_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(mt.thermal_base))
    return PTR_ERR(mt.thermal_base);
    ret = mtk_thermal_get_calibration_data(&pdev.dev, mt);
    if (ret)
    return ret;
    mutex_init(&mt.lock);
    mt.dev = &pdev.dev;
    auxadc = of_parse_phandle(np, "mediatek,auxadc", 0);
    if (!auxadc) {
    dev_err(&pdev.dev, "missing auxadc node\n");
    return -ENODEV;
    }
    auxadc_base = of_iomap(auxadc, 0);
    auxadc_phys_base = of_get_phys_base(auxadc);
    of_node_put(auxadc);
    if (auxadc_phys_base == OF_BAD_ADDR) {
    dev_err(&pdev.dev, "Can't get auxadc phys address\n");
    return -EINVAL;
    }
    apmixedsys = of_parse_phandle(np, "mediatek,apmixedsys", 0);
    if (!apmixedsys) {
    dev_err(&pdev.dev, "missing apmixedsys node\n");
    return -ENODEV;
    }
    apmixed_base = of_iomap(apmixedsys, 0);
    apmixed_phys_base = of_get_phys_base(apmixedsys);
    of_node_put(apmixedsys);
    if (apmixed_phys_base == OF_BAD_ADDR) {
    dev_err(&pdev.dev, "Can't get auxadc phys address\n");
    return -EINVAL;
    }
    ret = device_reset_optional(&pdev.dev);
    if (ret)
    return ret;
    mt.clk_auxadc = devm_clk_get_enabled(&pdev.dev, "auxadc");
    if (IS_ERR(mt.clk_auxadc)) {
    ret = PTR_ERR(mt.clk_auxadc);
    dev_err(&pdev.dev, "Can't enable auxadc clk: %d\n", ret);
    return ret;
    }
    mt.clk_peri_therm = devm_clk_get_enabled(&pdev.dev, "therm");
    if (IS_ERR(mt.clk_peri_therm)) {
    ret = PTR_ERR(mt.clk_peri_therm);
    dev_err(&pdev.dev, "Can't enable peri clk: %d\n", ret);
    return ret;
    }
    mtk_thermal_turn_on_buffer(mt, apmixed_base);
    if (mt.conf.version != MTK_THERMAL_V1)
    mtk_thermal_release_periodic_ts(mt, auxadc_base);
    if (mt.conf.version == MTK_THERMAL_V1)
    mt.raw_to_mcelsius = raw_to_mcelsius_v1;
#[no_mangle]
pub unsafe extern "C" fn if(MTK_THERMAL_V2: mt->conf->version ==) -> else {
    else if (mt.conf.version == MTK_THERMAL_V2)
    mt.raw_to_mcelsius = raw_to_mcelsius_v2;
    else
    mt.raw_to_mcelsius = raw_to_mcelsius_v3;
    for (ctrl_id = 0; ctrl_id < mt.conf.num_controller ; ctrl_id++)
    for (i = 0; i < mt.conf.num_banks; i++)
    mtk_thermal_init_bank(mt, i, apmixed_phys_base,
    auxadc_phys_base, ctrl_id);
    tzdev = devm_thermal_of_zone_register(&pdev.dev, 0, mt,
    &mtk_thermal_ops);
    if (IS_ERR(tzdev))
    return PTR_ERR(tzdev);
    ret = devm_thermal_add_hwmon_sysfs(&pdev.dev, tzdev);
    if (ret)
    dev_warn(&pdev.dev, "error in thermal_add_hwmon_sysfs");
    return 0;
    }
    static struct platform_driver mtk_thermal_driver = {
    .probe = mtk_thermal_probe,
    .driver = {
    .name = "mtk-thermal",
    .of_match_table = mtk_thermal_of_match,
    },
    };
    module_platform_driver(mtk_thermal_driver);
    MODULE_AUTHOR("Michael Kao <michael.kao@mediatek.com>");
    MODULE_AUTHOR("Louis Yu <louis.yu@mediatek.com>");
    MODULE_AUTHOR("Dawei Chien <dawei.chien@mediatek.com>");
    MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
    MODULE_AUTHOR("Hanyi Wu <hanyi.wu@mediatek.com>");
    MODULE_DESCRIPTION("Mediatek thermal driver");
    MODULE_LICENSE("GPL v2");
