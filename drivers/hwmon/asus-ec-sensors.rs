//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/asus-ec-sensors.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// HWMON driver for ASUS motherboards that publish some sensor values
// via the embedded controller registers.
//
// Copyright (C) 2021 Eugene Shalygin <eugene.shalygin@gmail.com>
// EC provides:
// - Chipset temperature
// - CPU temperature
// - Motherboard temperature
// - T_Sensor temperature
// - VRM temperature
// - Water In temperature
// - Water Out temperature
// - CPU Optional fan RPM
// - Chipset fan RPM
// - VRM Heat Sink fan RPM
// - Water Flow fan RPM
// - CPU current
// - CPU core voltage
//

    static char *mutex_path_override;
// Writing to this EC register switches EC bank
pub const ASUS_EC_BANK_REGISTER: c_uint = 0xff;
pub const SENSOR_LABEL_LEN: c_int = 16;
//
// Arbitrary set max. allowed bank number. Required for sorting banks and
// currently is overkill with just 2 banks used at max, but for the sake
// of alignment let's set it to a higher value.
//
pub const ASUS_EC_MAX_BANK: c_int = 3;
pub const ACPI_LOCK_DELAY_MS: c_int = 800;
// ACPI mutex for locking access to the EC for the firmware

pub const MAX_IDENTICAL_BOARD_VARIATIONS: c_int = 3;
// Moniker for the ACPI global lock (':' is not allowed in ASL identifiers)

    typedef union {
    u32 value;
    struct {
    u8 index;
    u8 bank;
    u8 size;
    u8 dummy;
    } components;
    } sensor_address;

    .value = (size << 16) + (bank << 8) + index                    \
    }
    static u32 hwmon_attributes[hwmon_max] = {
    [hwmon_chip] = HWMON_C_REGISTER_TZ,
    [hwmon_temp] = HWMON_T_INPUT | HWMON_T_LABEL,
    [hwmon_in] = HWMON_I_INPUT | HWMON_I_LABEL,
    [hwmon_curr] = HWMON_C_INPUT | HWMON_C_LABEL,
    [hwmon_fan] = HWMON_F_INPUT | HWMON_F_LABEL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_sensor_info {
    pub label: [c_char; SENSOR_LABEL_LEN],
    pub type: enum hwmon_sensor_types,
    pub addr: sensor_address,
}

    .label = sensor_label, .type = sensor_type,                    \
    .addr = MAKE_SENSOR_ADDRESS(size, bank, index),                \
    }
    enum ec_sensors {
// chipset temperature [℃]
    ec_sensor_temp_chipset,
// CPU temperature [℃]
    ec_sensor_temp_cpu,
// CPU package temperature [℃]
    ec_sensor_temp_cpu_package,
// motherboard temperature [℃]
    ec_sensor_temp_mb,
// "T_Sensor" temperature sensor reading [℃]
    ec_sensor_temp_t_sensor,
// like ec_sensor_temp_t_sensor, but at an alternate address [℃]
    ec_sensor_temp_t_sensor_alt1,
// VRM temperature [℃]
    ec_sensor_temp_vrm,
// VRM east (right) temperature [℃]
    ec_sensor_temp_vrme,
// VRM west (left) temperature [℃]
    ec_sensor_temp_vrmw,
// CPU Core voltage [mV]
    ec_sensor_in_cpu_core,
// CPU_Opt fan [RPM]
    ec_sensor_fan_cpu_opt,
// VRM heat sink fan [RPM]
    ec_sensor_fan_vrm_hs,
// VRM east (right) heat sink fan [RPM]
    ec_sensor_fan_vrme_hs,
// VRM west (left) heat sink fan [RPM]
    ec_sensor_fan_vrmw_hs,
// Chipset fan [RPM]
    ec_sensor_fan_chipset,
// Water flow sensor reading [RPM]
    ec_sensor_fan_water_flow,
// USB4 fan [RPM]
    ec_sensor_fan_usb4,
// M.2 fan [RPM]
    ec_sensor_fan_m2,
// CPU current [A]
    ec_sensor_curr_cpu,
// "Water_In" temperature sensor reading [℃]
    ec_sensor_temp_water_in,
// "Water_Out" temperature sensor reading [℃]
    ec_sensor_temp_water_out,
// "Water_Block_In" temperature sensor reading [℃]
    ec_sensor_temp_water_block_in,
// "Water_Block_Out" temperature sensor reading [℃]
    ec_sensor_temp_water_block_out,
// "T_sensor_2" temperature sensor reading [℃]
    ec_sensor_temp_t_sensor_2,
// "Extra_1" temperature sensor reading [℃]
    ec_sensor_temp_sensor_extra_1,
// "Extra_2" temperature sensor reading [℃]
    ec_sensor_temp_sensor_extra_2,
// "Extra_3" temperature sensor reading [℃]
    ec_sensor_temp_sensor_extra_3,
    };

//
// The values for temperature sensor readings without physical sensors connected.
// The value varies across generations and is seemingly defined by the EC chip
// used in the given board.
//
    static const s32 temperature_blank_values[] = {-62, -60, -40};
    static const s32 environment_temp_sensors =
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_T_SENSOR_ALT1 |
    SENSOR_TEMP_WATER_IN | SENSOR_TEMP_WATER_OUT |
    SENSOR_TEMP_WATER_BLOCK_IN | SENSOR_TEMP_WATER_BLOCK_OUT |
    SENSOR_TEMP_T_SENSOR_2 | SENSOR_TEMP_SENSOR_EXTRA_1 |
    SENSOR_TEMP_SENSOR_EXTRA_2 | SENSOR_TEMP_SENSOR_EXTRA_3;
    enum board_family {
    family_unknown,
    family_amd_400_series,
    family_amd_500_series,
    family_amd_600_series,
    family_amd_800_series,
    family_amd_trx_50,
    family_amd_wrx_90,
    family_intel_200_series,
    family_intel_300_series,
    family_intel_400_series,
    family_intel_600_series,
    family_intel_700_series
    };
//
// All the known sensors for ASUS EC controllers. These arrays have to be sorted
// by the full ((bank << 8) + index) register index (see asus_ec_block_read() as
// to why).
//
    static const struct ec_sensor_info sensors_family_amd_400[] = {
    [ec_sensor_temp_chipset] =
    EC_SENSOR("Chipset", hwmon_temp, 1, 0x00, 0x3a),
    [ec_sensor_temp_cpu] =
    EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x3b),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x3c),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_temp_vrm] =
    EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x3e),
    [ec_sensor_in_cpu_core] =
    EC_SENSOR("CPU Core", hwmon_in, 2, 0x00, 0xa2),
    [ec_sensor_fan_vrm_hs] =
    EC_SENSOR("VRM HS", hwmon_fan, 2, 0x00, 0xb2),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_fan_chipset] =
// no chipset fans in this generation
    EC_SENSOR("Chipset", hwmon_fan, 0, 0x00, 0x00),
    [ec_sensor_fan_water_flow] =
    EC_SENSOR("Water_Flow", hwmon_fan, 2, 0x00, 0xb4),
    [ec_sensor_curr_cpu] =
    EC_SENSOR("CPU", hwmon_curr, 1, 0x00, 0xf4),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x0b),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x0d),
    };
    static const struct ec_sensor_info sensors_family_amd_500[] = {
    [ec_sensor_temp_chipset] =
    EC_SENSOR("Chipset", hwmon_temp, 1, 0x00, 0x3a),
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x3b),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x3c),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_temp_vrm] = EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x3e),
    [ec_sensor_in_cpu_core] =
    EC_SENSOR("CPU Core", hwmon_in, 2, 0x00, 0xa2),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_vrm_hs] = EC_SENSOR("VRM HS", hwmon_fan, 2, 0x00, 0xb2),
    [ec_sensor_fan_chipset] =
    EC_SENSOR("Chipset", hwmon_fan, 2, 0x00, 0xb4),
    [ec_sensor_fan_water_flow] =
    EC_SENSOR("Water_Flow", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_curr_cpu] = EC_SENSOR("CPU", hwmon_curr, 1, 0x00, 0xf4),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x00),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x01),
    [ec_sensor_temp_water_block_in] =
    EC_SENSOR("Water_Block_In", hwmon_temp, 1, 0x01, 0x02),
    [ec_sensor_temp_water_block_out] =
    EC_SENSOR("Water_Block_Out", hwmon_temp, 1, 0x01, 0x03),
    [ec_sensor_temp_sensor_extra_1] =
    EC_SENSOR("Extra_1", hwmon_temp, 1, 0x01, 0x09),
    [ec_sensor_temp_t_sensor_2] =
    EC_SENSOR("T_sensor_2", hwmon_temp, 1, 0x01, 0x0a),
    [ec_sensor_temp_sensor_extra_2] =
    EC_SENSOR("Extra_2", hwmon_temp, 1, 0x01, 0x0b),
    [ec_sensor_temp_sensor_extra_3] =
    EC_SENSOR("Extra_3", hwmon_temp, 1, 0x01, 0x0c),
    };
    static const struct ec_sensor_info sensors_family_amd_600[] = {
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x30),
    [ec_sensor_temp_cpu_package] =
    EC_SENSOR("CPU Package", hwmon_temp, 1, 0x00, 0x31),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x32),
    [ec_sensor_temp_vrm] =
    EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x33),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x36),
    [ec_sensor_temp_t_sensor_alt1] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x37),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x00),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x01),
    };
    static const struct ec_sensor_info sensors_family_amd_800[] = {
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x30),
    [ec_sensor_temp_cpu_package] =
    EC_SENSOR("CPU Package", hwmon_temp, 1, 0x00, 0x31),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x32),
    [ec_sensor_temp_vrm] =
    EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x33),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x36),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    };
    static const struct ec_sensor_info sensors_family_amd_trx_50[] = {
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x30),
    [ec_sensor_temp_cpu_package] =
    EC_SENSOR("CPU Package", hwmon_temp, 1, 0x00, 0x31),
    [ec_sensor_temp_vrme] = EC_SENSOR("VRM_E", hwmon_temp, 1, 0x00, 0x33),
    [ec_sensor_temp_vrmw] = EC_SENSOR("VRM_W", hwmon_temp, 1, 0x00, 0x34),
    [ec_sensor_fan_cpu_opt] = EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_vrmw_hs] = EC_SENSOR("VRM_E HS", hwmon_fan, 2, 0x00, 0xb4),
    [ec_sensor_fan_vrme_hs] = EC_SENSOR("VRM_W HS", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x01, 0x04),
    };
    static const struct ec_sensor_info sensors_family_amd_wrx_90[] = {
    [ec_sensor_temp_cpu_package] =
    EC_SENSOR("CPU Package", hwmon_temp, 1, 0x00, 0x31),
    [ec_sensor_temp_vrme] = EC_SENSOR("VRM_E", hwmon_temp, 1, 0x00, 0x33),
    [ec_sensor_temp_vrmw] = EC_SENSOR("VRM_W", hwmon_temp, 1, 0x00, 0x34),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_vrmw_hs] =
    EC_SENSOR("VRMW HS", hwmon_fan, 2, 0x00, 0xb4),
    [ec_sensor_fan_usb4] = EC_SENSOR("USB4", hwmon_fan, 2, 0x00, 0xb6),
    [ec_sensor_fan_vrme_hs] =
    EC_SENSOR("VRME HS", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_fan_m2] = EC_SENSOR("M.2", hwmon_fan, 2, 0x00, 0xbe),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x01, 0x04),
    };
    static const struct ec_sensor_info sensors_family_intel_200[] = {
    [ec_sensor_temp_chipset] =
    EC_SENSOR("Chipset", hwmon_temp, 1, 0x00, 0x3a),
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x3b),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x3c),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xbc),
    };
    static const struct ec_sensor_info sensors_family_intel_300[] = {
    [ec_sensor_temp_chipset] =
    EC_SENSOR("Chipset", hwmon_temp, 1, 0x00, 0x3a),
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x3b),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x3c),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_temp_vrm] = EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x3e),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_vrm_hs] = EC_SENSOR("VRM HS", hwmon_fan, 2, 0x00, 0xb2),
    [ec_sensor_fan_water_flow] =
    EC_SENSOR("Water_Flow", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x00),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x01),
    };
    static const struct ec_sensor_info sensors_family_intel_400[] = {
    [ec_sensor_temp_chipset] =
    EC_SENSOR("Chipset", hwmon_temp, 1, 0x00, 0x3a),
    [ec_sensor_temp_cpu] = EC_SENSOR("CPU", hwmon_temp, 1, 0x00, 0x3b),
    [ec_sensor_temp_mb] =
    EC_SENSOR("Motherboard", hwmon_temp, 1, 0x00, 0x3c),
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_temp_vrm] = EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x3e),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_vrm_hs] = EC_SENSOR("VRM HS", hwmon_fan, 2, 0x00, 0xb2),
    };
    static const struct ec_sensor_info sensors_family_intel_600[] = {
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x00, 0x3d),
    [ec_sensor_temp_vrm] = EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x3e),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_water_flow] =
    EC_SENSOR("Water_Flow", hwmon_fan, 2, 0x00, 0xbe),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x00),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x01),
    [ec_sensor_temp_water_block_in] =
    EC_SENSOR("Water_Block_In", hwmon_temp, 1, 0x01, 0x02),
    };
    static const struct ec_sensor_info sensors_family_intel_700[] = {
    [ec_sensor_temp_t_sensor] =
    EC_SENSOR("T_Sensor", hwmon_temp, 1, 0x01, 0x09),
    [ec_sensor_temp_t_sensor_2] =
    EC_SENSOR("T_Sensor 2", hwmon_temp, 1, 0x01, 0x05),
    [ec_sensor_temp_vrm] = EC_SENSOR("VRM", hwmon_temp, 1, 0x00, 0x33),
    [ec_sensor_fan_cpu_opt] =
    EC_SENSOR("CPU_Opt", hwmon_fan, 2, 0x00, 0xb0),
    [ec_sensor_fan_water_flow] =
    EC_SENSOR("Water_Flow", hwmon_fan, 2, 0x00, 0xbc),
    [ec_sensor_temp_water_in] =
    EC_SENSOR("Water_In", hwmon_temp, 1, 0x01, 0x00),
    [ec_sensor_temp_water_out] =
    EC_SENSOR("Water_Out", hwmon_temp, 1, 0x01, 0x01),
    };
// Shortcuts for common combinations

    (SENSOR_TEMP_CHIPSET | SENSOR_TEMP_CPU | SENSOR_TEMP_MB)

    (SENSOR_TEMP_WATER_BLOCK_IN | SENSOR_TEMP_WATER_BLOCK_OUT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_board_info {
    pub sensors: c_ulong,
//
// Defines which mutex to use for guarding access to the state and the
// hardware. Can be either a full path to an AML mutex or the
// pseudo-path ACPI_GLOBAL_LOCK_PSEUDO_PATH to use the global ACPI lock,
// or left empty to use a regular mutex object, in which case access to
// the hardware is not guarded.
//
    pub mutex_path: *const c_char,
    pub family: enum board_family,
}

    static const struct ec_board_info board_info_crosshair_viii_dark_hero = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_SET_TEMP_WATER |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_WATER_FLOW |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_crosshair_viii_hero = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_SET_TEMP_WATER |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_CHIPSET |
    SENSOR_FAN_WATER_FLOW | SENSOR_CURR_CPU |
    SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_crosshair_viii_impact = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_CHIPSET | SENSOR_CURR_CPU |
    SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_crosshair_x670e_extreme = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_WATER_IN |
    SENSOR_TEMP_WATER_OUT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_crosshair_x670e_gene = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_crosshair_x670e_hero = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_SET_TEMP_WATER,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_crosshair_x870e_hero = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_maximus_vi_hero = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_SET_TEMP_WATER |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_WATER_FLOW,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_intel_300_series,
    };
    static const struct ec_board_info board_info_maximus_x_hero = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_LPCB_SIO1_MUT0,
    .family = family_intel_300_series,
    };
    static const struct ec_board_info board_info_maximus_xi_hero = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_SET_TEMP_WATER |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_WATER_FLOW,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_intel_300_series,
    };
    static const struct ec_board_info board_info_maximus_z690_formula = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_SET_TEMP_WATER | SENSOR_FAN_WATER_FLOW,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_intel_600_series,
    };
    static const struct ec_board_info board_info_maximus_z790_extreme = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_SET_TEMP_WATER | SENSOR_FAN_WATER_FLOW,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_intel_700_series,
    };
    static const struct ec_board_info board_info_maximus_z790_hero = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_SET_TEMP_WATER | SENSOR_FAN_WATER_FLOW |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_intel_700_series,
    };
    static const struct ec_board_info board_info_prime_x470_pro = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_400_series,
    };
    static const struct ec_board_info board_info_prime_x570_pro = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CHIPSET,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_prime_x670e_pro_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR_ALT1 | SENSOR_FAN_CPU_OPT,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_prime_z270_a = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_LPCB_SIO1_MUT0,
    .family = family_intel_200_series,
    };
    static const struct ec_board_info board_info_pro_art_b550_creator = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_pro_art_x570_creator_wifi = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_pro_art_x670E_creator_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_pro_art_x870E_creator_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_pro_art_z690_creator_wifi = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PC00_LPCB_SIO1_MUT0,
    .family = family_intel_600_series,
    };
    static const struct ec_board_info board_info_pro_ws_trx50_sage_wifi = {
// Board also has a nct6798
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE | SENSOR_TEMP_VRME |
    SENSOR_TEMP_VRMW | SENSOR_FAN_CPU_OPT | SENSOR_FAN_VRME_HS |
    SENSOR_FAN_VRMW_HS | SENSOR_TEMP_T_SENSOR,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_amd_trx_50,
    };
    static const struct ec_board_info board_info_pro_ws_wrx90e_sage_se = {
// Board also has a nct6798 with 7 more fans and temperatures
    .sensors = SENSOR_TEMP_CPU_PACKAGE | SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_USB4 | SENSOR_FAN_M2 |
    SENSOR_FAN_VRME_HS | SENSOR_FAN_VRMW_HS |
    SENSOR_TEMP_VRME | SENSOR_TEMP_VRMW,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_amd_wrx_90,
    };
    static const struct ec_board_info board_info_pro_ws_x570_ace = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CHIPSET |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_b550_e_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_b550_i_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_VRM_HS | SENSOR_CURR_CPU |
    SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_b650e_e_gaming = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_strix_b650e_i_gaming = {
    .sensors = SENSOR_TEMP_VRM | SENSOR_TEMP_T_SENSOR |
    SENSOR_SET_TEMP_CHIPSET_CPU_MB | SENSOR_IN_CPU_CORE,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_strix_b850_e_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_b850_i_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_x470_f_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CPU_OPT |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_400_series,
    };
    static const struct ec_board_info board_info_strix_x470_i_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_400_series,
    };
    static const struct ec_board_info board_info_strix_x570_e_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CHIPSET | SENSOR_CURR_CPU |
    SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_x570_e_gaming_wifi_ii = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_CURR_CPU |
    SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_x570_f_gaming = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB |
    SENSOR_TEMP_T_SENSOR | SENSOR_FAN_CHIPSET,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_x570_i_gaming = {
    .sensors = SENSOR_TEMP_CHIPSET | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_VRM_HS | SENSOR_FAN_CHIPSET |
    SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_amd_500_series,
    };
    static const struct ec_board_info board_info_strix_x670e_e_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB  | SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_strix_x670e_i_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_strix_x870_f_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM | SENSOR_TEMP_T_SENSOR,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_x870_i_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_x870e_e_gaming_wifi = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM | SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_x870e_h_gaming_wifi7 = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM | SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_800_series,
    };
    static const struct ec_board_info board_info_strix_z390_f_gaming = {
    .sensors = SENSOR_TEMP_CHIPSET | SENSOR_TEMP_VRM |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_intel_300_series,
    };
    static const struct ec_board_info board_info_strix_z490_f_gaming = {
    .sensors = SENSOR_TEMP_CHIPSET |
    SENSOR_TEMP_CPU |
    SENSOR_TEMP_MB |
    SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT |
    SENSOR_FAN_VRM_HS,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_ASMX,
    .family = family_intel_400_series,
    };
    static const struct ec_board_info board_info_strix_z690_a_gaming_wifi_d4 = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_intel_600_series,
    };
    static const struct ec_board_info board_info_strix_z690_e_gaming_wifi = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_RMTW_ASMX,
    .family = family_intel_600_series,
    };
    static const struct ec_board_info board_info_strix_z790_e_gaming_wifi_ii = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PC00_LPCB_SIO1_MUT0,
    .family = family_intel_700_series,
    };
    static const struct ec_board_info board_info_strix_z790_h_gaming_wifi = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PC00_LPCB_SIO1_MUT0,
    .family = family_intel_700_series,
    };
    static const struct ec_board_info board_info_strix_z790_i_gaming_wifi = {
    .sensors = SENSOR_TEMP_T_SENSOR | SENSOR_TEMP_T_SENSOR_2 |
    SENSOR_TEMP_VRM,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PC00_LPCB_SIO1_MUT0,
    .family = family_intel_700_series,
    };
    static const struct ec_board_info board_info_tuf_gaming_x670e_plus = {
    .sensors = SENSOR_TEMP_CPU | SENSOR_TEMP_CPU_PACKAGE |
    SENSOR_TEMP_MB | SENSOR_TEMP_VRM |
    SENSOR_TEMP_WATER_IN | SENSOR_TEMP_WATER_OUT |
    SENSOR_FAN_CPU_OPT,
    .mutex_path = ACPI_GLOBAL_LOCK_PSEUDO_PATH,
    .family = family_amd_600_series,
    };
    static const struct ec_board_info board_info_zenith_ii_extreme = {
    .sensors = SENSOR_SET_TEMP_CHIPSET_CPU_MB | SENSOR_TEMP_T_SENSOR |
    SENSOR_TEMP_VRM | SENSOR_SET_TEMP_WATER |
    SENSOR_FAN_CPU_OPT | SENSOR_FAN_CHIPSET | SENSOR_FAN_VRM_HS |
    SENSOR_FAN_WATER_FLOW | SENSOR_CURR_CPU | SENSOR_IN_CPU_CORE |
    SENSOR_SET_WATER_BLOCK |
    SENSOR_TEMP_T_SENSOR_2 | SENSOR_TEMP_SENSOR_EXTRA_1 |
    SENSOR_TEMP_SENSOR_EXTRA_2 | SENSOR_TEMP_SENSOR_EXTRA_3,
    .mutex_path = ASUS_HW_ACCESS_MUTEX_SB_PCI0_SBRG_SIO1_MUT0,
    .family = family_amd_500_series,
    };

    {                                                                      \
    .matches = {                                                   \
    DMI_EXACT_MATCH(DMI_BOARD_VENDOR,                      \
    "ASUSTeK COMPUTER INC."),              \
    DMI_EXACT_MATCH(DMI_BOARD_NAME, name),                 \
    },                                                             \
    .driver_data = (void *)board_info,                              \
    }
    static const struct dmi_system_id dmi_table[] = {
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("MAXIMUS VI HERO",
    &board_info_maximus_vi_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME X470-PRO",
    &board_info_prime_x470_pro),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME X570-PRO",
    &board_info_prime_x570_pro),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME X670E-PRO WIFI",
    &board_info_prime_x670e_pro_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME Z270-A",
    &board_info_prime_z270_a),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ProArt B550-CREATOR",
    &board_info_pro_art_b550_creator),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ProArt X570-CREATOR WIFI",
    &board_info_pro_art_x570_creator_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ProArt X670E-CREATOR WIFI",
    &board_info_pro_art_x670E_creator_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ProArt X870E-CREATOR WIFI",
    &board_info_pro_art_x870E_creator_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ProArt Z690-CREATOR WIFI",
    &board_info_pro_art_z690_creator_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("Pro WS TRX50-SAGE WIFI",
    &board_info_pro_ws_trx50_sage_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("Pro WS TRX50-SAGE WIFI A",
    &board_info_pro_ws_trx50_sage_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("Pro WS WRX90E-SAGE SE",
    &board_info_pro_ws_wrx90e_sage_se),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("Pro WS X570-ACE",
    &board_info_pro_ws_x570_ace),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VIII DARK HERO",
    &board_info_crosshair_viii_dark_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VIII FORMULA",
    &board_info_crosshair_viii_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VIII HERO",
    &board_info_crosshair_viii_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VIII HERO (WI-FI)",
    &board_info_crosshair_viii_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VIII IMPACT",
    &board_info_crosshair_viii_impact),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR X670E EXTREME",
    &board_info_crosshair_x670e_extreme),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR X670E GENE",
    &board_info_crosshair_x670e_gene),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR X670E HERO",
    &board_info_crosshair_x670e_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR X870E HERO",
    &board_info_crosshair_x870e_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS XI HERO",
    &board_info_maximus_xi_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS XI HERO (WI-FI)",
    &board_info_maximus_xi_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS X HERO",
    &board_info_maximus_x_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS Z690 FORMULA",
    &board_info_maximus_z690_formula),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS Z790 EXTREME",
    &board_info_maximus_z790_extreme),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG MAXIMUS Z790 HERO",
    &board_info_maximus_z790_hero),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B550-E GAMING",
    &board_info_strix_b550_e_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B550-I GAMING",
    &board_info_strix_b550_i_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B650E-E GAMING WIFI",
    &board_info_strix_b650e_e_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B650E-I GAMING WIFI",
    &board_info_strix_b650e_i_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B850-E GAMING WIFI",
    &board_info_strix_b850_e_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B850-I GAMING WIFI",
    &board_info_strix_b850_i_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X470-F GAMING",
    &board_info_strix_x470_f_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X470-I GAMING",
    &board_info_strix_x470_i_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X570-E GAMING",
    &board_info_strix_x570_e_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X570-E GAMING WIFI II",
    &board_info_strix_x570_e_gaming_wifi_ii),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X570-F GAMING",
    &board_info_strix_x570_f_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X570-I GAMING",
    &board_info_strix_x570_i_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X670E-E GAMING WIFI",
    &board_info_strix_x670e_e_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X670E-I GAMING WIFI",
    &board_info_strix_x670e_i_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X870-F GAMING WIFI",
    &board_info_strix_x870_f_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X870-I GAMING WIFI",
    &board_info_strix_x870_i_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X870E-E GAMING WIFI",
    &board_info_strix_x870e_e_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X870E-E GAMING WIFI7 R2",
    &board_info_strix_x870e_e_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X870E-H GAMING WIFI7",
    &board_info_strix_x870e_h_gaming_wifi7),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z390-E GAMING",
    &board_info_strix_z390_f_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z390-F GAMING",
    &board_info_strix_z390_f_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z490-F GAMING",
    &board_info_strix_z490_f_gaming),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z690-A GAMING WIFI D4",
    &board_info_strix_z690_a_gaming_wifi_d4),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z690-E GAMING WIFI",
    &board_info_strix_z690_e_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z790-E GAMING WIFI II",
    &board_info_strix_z790_e_gaming_wifi_ii),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z790-H GAMING WIFI",
    &board_info_strix_z790_h_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX Z790-I GAMING WIFI",
    &board_info_strix_z790_i_gaming_wifi),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG ZENITH II EXTREME",
    &board_info_zenith_ii_extreme),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG ZENITH II EXTREME ALPHA",
    &board_info_zenith_ii_extreme),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("TUF GAMING X670E-PLUS",
    &board_info_tuf_gaming_x670e_plus),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("TUF GAMING X670E-PLUS WIFI",
    &board_info_tuf_gaming_x670e_plus),
    {},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_sensor {
// this is ec_sensors enum value
    pub info_index: c_uint,
    pub cached_value: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_data {
    union {
    pub aml: acpi_handle,
// global lock handle
    pub glk: u32,
    pub mutex: },
    pub data): *mut *mut bool (lock)(struct lock_data,
    pub data): *mut *mut bool (unlock)(struct lock_data,
}

//
// The next function pairs implement options for locking access to the
// state and the EC
//
#[no_mangle]
unsafe extern "C" fn lock_via_acpi_mutex(data: *mut lock_data) -> bool {
    static bool lock_via_acpi_mutex(struct lock_data *data)
    {
//
// ASUS DSDT does not specify that access to the EC has to be guarded,
// but firmware does access it via ACPI
//
    return ACPI_SUCCESS(acpi_acquire_mutex(data.mutex.aml,
    core::ptr::null_mut(), ACPI_LOCK_DELAY_MS));
    }
#[no_mangle]
unsafe extern "C" fn unlock_acpi_mutex(data: *mut lock_data) -> bool {
    static bool unlock_acpi_mutex(struct lock_data *data)
    {
    return ACPI_SUCCESS(acpi_release_mutex(data.mutex.aml, core::ptr::null_mut()));
    }
#[no_mangle]
unsafe extern "C" fn lock_via_global_acpi_lock(data: *mut lock_data) -> bool {
    static bool lock_via_global_acpi_lock(struct lock_data *data)
    {
    return ACPI_SUCCESS(acpi_acquire_global_lock(ACPI_LOCK_DELAY_MS,
    &data.mutex.glk));
    }
#[no_mangle]
unsafe extern "C" fn unlock_global_acpi_lock(data: *mut lock_data) -> bool {
    static bool unlock_global_acpi_lock(struct lock_data *data)
    {
    return ACPI_SUCCESS(acpi_release_global_lock(data.mutex.glk));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_sensors_data {
    pub board_info: *const ec_board_info,
    pub sensors_info: *const ec_sensor_info,
    pub sensors: *mut ec_sensor,
// EC registers to read from
    pub registers: *mut u16,
    pub read_buffer: *mut u8,
// sorted list of unique register banks
    pub 1]: u8 banks[ASUS_EC_MAX_BANK +,
// in jiffies
    pub next_update: u64,
    pub lock_data: lock_data,
// number of board EC sensors
    pub nr_sensors: u8,
//
// number of EC registers to read
// (sensor might span more than 1 register)
//
    pub nr_registers: u8,
// number of unique register banks
    pub nr_banks: u8,
}

#[no_mangle]
unsafe extern "C" fn register_bank(reg: u16) -> u8 {
    static u8 register_bank(u16 reg)
    {
    return reg >> 8;
    }
#[no_mangle]
unsafe extern "C" fn register_index(reg: u16) -> u8 {
    static u8 register_index(u16 reg)
    {
    return reg & 0x00ff;
    }
#[no_mangle]
unsafe extern "C" fn is_sensor_data_signed(si: *const ec_sensor_info) -> bool {
    static bool is_sensor_data_signed(const struct ec_sensor_info *si)
    {
//
// guessed from WMI functions in DSDT code for boards
// of the X470 generation
//
    return si.type == hwmon_temp;
    }
    static const struct ec_sensor_info *
    get_sensor_info(const struct ec_sensors_data *state, int index)
    {
    return state.sensors_info + state.sensors[index].info_index;
    }
    static enum ec_sensors
    get_ec_sensor_type(const struct ec_sensors_data *state, int index)
    {
    return state.sensors[index].info_index;
    }
    static int find_ec_sensor_index(const struct ec_sensors_data *ec,
    enum hwmon_sensor_types type, int channel)
    {
    unsigned int i;
    for (i = 0; i < ec.nr_sensors; i++) {
    if (get_sensor_info(ec, i).type == type) {
    if (channel == 0)
    return i;
    channel--;
    }
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn bank_compare(a: *const c_void, b: *const c_void) -> c_int {
    static int bank_compare(const void *a, const void *b)
    {
    return *((const s8 *)a) - *((const s8 *)b);
    }
#[no_mangle]
unsafe extern "C" fn setup_sensor_data(ec: *mut ec_sensors_data) {
    static void setup_sensor_data(struct ec_sensors_data *ec)
    {
    struct ec_sensor *s = ec.sensors;
    bool bank_found;
    int i, j;
    u8 bank;
    ec.nr_banks = 0;
    ec.nr_registers = 0;
    for_each_set_bit(i, &ec.board_info.sensors,
    BITS_PER_TYPE(ec.board_info.sensors)) {
    s.info_index = i;
    s.cached_value = 0;
    ec.nr_registers +=
    ec.sensors_info[s.info_index].addr.components.size;
    bank_found = false;
    bank = ec.sensors_info[s.info_index].addr.components.bank;
    for (j = 0; j < ec.nr_banks; j++) {
    if (ec.banks[j] == bank) {
    bank_found = true;
    break;
    }
    }
    if (!bank_found) {
    ec.banks[ec.nr_banks++] = bank;
    }
    s++;
    }
    sort(ec.banks, ec.nr_banks, 1, bank_compare, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn fill_ec_registers(ec: *mut ec_sensors_data) {
    static void fill_ec_registers(struct ec_sensors_data *ec)
    {
    const struct ec_sensor_info *si;
    unsigned int i, j, register_idx = 0;
    for (i = 0; i < ec.nr_sensors; ++i) {
    si = get_sensor_info(ec, i);
    for (j = 0; j < si.addr.components.size; ++j, ++register_idx) {
    ec.registers[register_idx] =
    (si.addr.components.bank << 8) +
    si.addr.components.index + j;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_lock_data(dev: *mut device) -> c_int {
    static int setup_lock_data(struct device *dev)
    {
    const char *mutex_path;
    int status;
    struct ec_sensors_data *state = dev_get_drvdata(dev);
    mutex_path = mutex_path_override ?
    mutex_path_override : state.board_info.mutex_path;
    if (!mutex_path || !strlen(mutex_path)) {
    dev_err(dev, "Hardware access guard mutex name is empty");
    return -EINVAL;
    }
    if (!strcmp(mutex_path, ACPI_GLOBAL_LOCK_PSEUDO_PATH)) {
    state.lock_data.mutex.glk = 0;
    state.lock_data.lock = lock_via_global_acpi_lock;
    state.lock_data.unlock = unlock_global_acpi_lock;
    } else {
    status = acpi_get_handle(core::ptr::null_mut(), (acpi_string)mutex_path,
    &state.lock_data.mutex.aml);
    if (ACPI_FAILURE(status)) {
    dev_err(dev,
    "Failed to get hardware access guard AML mutex '%s': error %d",
    mutex_path, status);
    return -ENOENT;
    }
    state.lock_data.lock = lock_via_acpi_mutex;
    state.lock_data.unlock = unlock_acpi_mutex;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asus_ec_bank_switch(bank: u8, old: *mut u8) -> c_int {
    static int asus_ec_bank_switch(u8 bank, u8 *old)
    {
    let mut status: c_int = 0;
    if (old) {
    status = ec_read(ASUS_EC_BANK_REGISTER, old);
    }
    if (status || (old && (*old == bank)))
    return status;
    return ec_write(ASUS_EC_BANK_REGISTER, bank);
    }
    static int asus_ec_block_read(const struct device *dev,
    struct ec_sensors_data *ec)
    {
    int ireg, ibank, status;
    u8 bank, reg_bank, prev_bank;
    bank = 0;
    status = asus_ec_bank_switch(bank, &prev_bank);
    if (status) {
    dev_warn(dev, "EC bank switch failed");
    return status;
    }
    if (prev_bank) {
// oops... somebody else is working with the EC too
    dev_warn(dev,
    "Concurrent access to the ACPI EC detected.\nRace condition possible.");
    }
// read registers minimizing bank switches.
    for (ibank = 0; ibank < ec.nr_banks; ibank++) {
    if (bank != ec.banks[ibank]) {
    bank = ec.banks[ibank];
    if (asus_ec_bank_switch(bank, core::ptr::null_mut())) {
    dev_warn(dev, "EC bank switch to %d failed",
    bank);
    break;
    }
    }
    for (ireg = 0; ireg < ec.nr_registers; ireg++) {
    reg_bank = register_bank(ec.registers[ireg]);
    if (reg_bank != bank) {
    continue;
    }
    ec_read(register_index(ec.registers[ireg]),
    ec.read_buffer + ireg);
    }
    }
    status = asus_ec_bank_switch(prev_bank, core::ptr::null_mut());
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn get_sensor_value(si: *const ec_sensor_info, data: *mut u8) -> i32 {
    static inline s32 get_sensor_value(const struct ec_sensor_info *si, u8 *data)
    {
    if (is_sensor_data_signed(si)) {
    switch (si.addr.components.size) {
    case 1:
    return (s8)*data;
    case 2:
    return (s16)get_unaligned_be16(data);
    case 4:
    return (s32)get_unaligned_be32(data);
    default:
    return 0;
    }
    } else {
    switch (si.addr.components.size) {
    case 1:
    return *data;
    case 2:
    return get_unaligned_be16(data);
    case 4:
    return get_unaligned_be32(data);
    default:
    return 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn update_sensor_values(ec: *mut ec_sensors_data, data: *mut u8) {
    static void update_sensor_values(struct ec_sensors_data *ec, u8 *data)
    {
    const struct ec_sensor_info *si;
    struct ec_sensor *s, *sensor_end;
    sensor_end = ec.sensors + ec.nr_sensors;
    for (s = ec.sensors; s != sensor_end; s++) {
    si = ec.sensors_info + s.info_index;
    s.cached_value = get_sensor_value(si, data);
    data += si.addr.components.size;
    }
    }
    static int update_ec_sensors(const struct device *dev,
    struct ec_sensors_data *ec)
    {
    int status;
    if (!ec.lock_data.lock(&ec.lock_data)) {
    dev_warn(dev, "Failed to acquire mutex");
    return -EBUSY;
    }
    status = asus_ec_block_read(dev, ec);
    if (!status) {
    update_sensor_values(ec, ec.read_buffer);
    }
    if (!ec.lock_data.unlock(&ec.lock_data))
    dev_err(dev, "Failed to release mutex");
    return status;
    }
#[no_mangle]
unsafe extern "C" fn scale_sensor_value(value: i32, data_type: c_int) -> c_long {
    static long scale_sensor_value(s32 value, int data_type)
    {
    switch (data_type) {
    case hwmon_curr:
    case hwmon_temp:
    return value * MILLI;
    default:
    return value;
    }
    }
    static int get_cached_value_or_update(const struct device *dev,
    int sensor_index,
    struct ec_sensors_data *state, s32 *value)
    {
    if (time_after64(get_jiffies_64(), state.next_update)) {
    if (update_ec_sensors(dev, state)) {
    dev_err(dev, "update_ec_sensors() failure\n");
    return -EIO;
    }
    state.next_update = get_jiffies_64() + HZ;
    }
// value = state->sensors[sensor_index].cached_value;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_blank_temperature_value(value: i32) -> bool {
    static bool is_blank_temperature_value(s32 value)
    {
    size_t i;
    for (i = 0; i < ARRAY_SIZE(temperature_blank_values); ++i) {
    if (value == temperature_blank_values[i])
    return true;
    }
    return false;
    }
//
// Now follow the functions that implement the hwmon interface
//
    static int asus_ec_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    const struct ec_sensor_info *sensor_info;
    enum ec_sensors ec_sensor;
    int ret;
    let mut value: i32 = 0;
    struct ec_sensors_data *state = dev_get_drvdata(dev);
    let mut sidx: c_int = find_ec_sensor_index(state, type, channel);
    if (sidx < 0) {
    return sidx;
    }
    ret = get_cached_value_or_update(dev, sidx, state, &value);
    if (ret)
    return ret;
    sensor_info = get_sensor_info(state, sidx);
    if (sensor_info.type == hwmon_temp) {
    ec_sensor = get_ec_sensor_type(state, sidx);
    if ((environment_temp_sensors & BIT(ec_sensor)) &&
    is_blank_temperature_value(value))
    return -ENODATA;
    }
// val = scale_sensor_value(value, sensor_info->type);
    return 0;
    }
    static int asus_ec_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    struct ec_sensors_data *state = dev_get_drvdata(dev);
    let mut sensor_index: c_int = find_ec_sensor_index(state, type, channel);
    if (sensor_index < 0)
    return sensor_index;
// str = get_sensor_info(state, sensor_index)->label;
    return 0;
    }
    static umode_t asus_ec_hwmon_is_visible(const void *drvdata,
    enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    const struct ec_sensors_data *state = drvdata;
    return find_ec_sensor_index(state, type, channel) >= 0 ? S_IRUGO : 0;
    }
    static int
    asus_ec_hwmon_add_chan_info(struct hwmon_channel_info *asus_ec_hwmon_chan,
    struct device *dev, int num,
    enum hwmon_sensor_types type, u32 config)
    {
    int i;
    u32 *cfg = devm_kcalloc(dev, num + 1, sizeof(*cfg), GFP_KERNEL);
    if (!cfg)
    return -ENOMEM;
    asus_ec_hwmon_chan.type = type;
    asus_ec_hwmon_chan.config = cfg;
    for (i = 0; i < num; i++, cfg++)
// cfg = config;
    return 0;
    }
    static const struct hwmon_ops asus_ec_hwmon_ops = {
    .is_visible = asus_ec_hwmon_is_visible,
    .read = asus_ec_hwmon_read,
    .read_string = asus_ec_hwmon_read_string,
    };
    static struct hwmon_chip_info asus_ec_chip_info = {
    .ops = &asus_ec_hwmon_ops,
    };
    static const struct ec_board_info *get_board_info(void)
    {
    const struct dmi_system_id *dmi_entry;
    dmi_entry = dmi_first_match(dmi_table);
    return dmi_entry ? dmi_entry.driver_data : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn asus_ec_probe(pdev: *mut platform_device) -> c_int {
    static int asus_ec_probe(struct platform_device *pdev)
    {
    const struct hwmon_channel_info **ptr_asus_ec_ci;
    int nr_count[hwmon_max] = { 0 }, nr_types = 0;
    struct hwmon_channel_info *asus_ec_hwmon_chan;
    const struct ec_board_info *pboard_info;
    const struct hwmon_chip_info *chip_info;
    struct device *dev = &pdev.dev;
    struct ec_sensors_data *ec_data;
    const struct ec_sensor_info *si;
    enum hwmon_sensor_types type;
    struct device *hwdev;
    unsigned int i;
    int status;
    pboard_info = get_board_info();
    if (!pboard_info)
    return -ENODEV;
    ec_data = devm_kzalloc(dev, sizeof(struct ec_sensors_data),
    GFP_KERNEL);
    if (!ec_data)
    return -ENOMEM;
    ec_data.next_update = INITIAL_JIFFIES;
    dev_set_drvdata(dev, ec_data);
    ec_data.board_info = pboard_info;
    switch (ec_data.board_info.family) {
    case family_amd_400_series:
    ec_data.sensors_info = sensors_family_amd_400;
    break;
    case family_amd_500_series:
    ec_data.sensors_info = sensors_family_amd_500;
    break;
    case family_amd_600_series:
    ec_data.sensors_info = sensors_family_amd_600;
    break;
    case family_amd_800_series:
    ec_data.sensors_info = sensors_family_amd_800;
    break;
    case family_amd_trx_50:
    ec_data.sensors_info = sensors_family_amd_trx_50;
    break;
    case family_amd_wrx_90:
    ec_data.sensors_info = sensors_family_amd_wrx_90;
    break;
    case family_intel_200_series:
    ec_data.sensors_info = sensors_family_intel_200;
    break;
    case family_intel_300_series:
    ec_data.sensors_info = sensors_family_intel_300;
    break;
    case family_intel_400_series:
    ec_data.sensors_info = sensors_family_intel_400;
    break;
    case family_intel_600_series:
    ec_data.sensors_info = sensors_family_intel_600;
    break;
    case family_intel_700_series:
    ec_data.sensors_info = sensors_family_intel_700;
    break;
    default:
    dev_err(dev, "Unknown board family: %d",
    ec_data.board_info.family);
    return -EINVAL;
    }
    ec_data.nr_sensors = hweight_long(ec_data.board_info.sensors);
    ec_data.sensors = devm_kcalloc(dev, ec_data.nr_sensors,
    sizeof(struct ec_sensor), GFP_KERNEL);
    if (!ec_data.sensors)
    return -ENOMEM;
    status = setup_lock_data(dev);
    if (status) {
    dev_err(dev, "Failed to setup state/EC locking: %d", status);
    return status;
    }
    setup_sensor_data(ec_data);
    ec_data.registers = devm_kcalloc(dev, ec_data.nr_registers,
    sizeof(u16), GFP_KERNEL);
    ec_data.read_buffer = devm_kcalloc(dev, ec_data.nr_registers,
    sizeof(u8), GFP_KERNEL);
    if (!ec_data.registers || !ec_data.read_buffer)
    return -ENOMEM;
    fill_ec_registers(ec_data);
    for (i = 0; i < ec_data.nr_sensors; ++i) {
    si = get_sensor_info(ec_data, i);
    if (!nr_count[si.type])
    ++nr_types;
    ++nr_count[si.type];
    }
    if (nr_count[hwmon_temp])
    nr_count[hwmon_chip]++, nr_types++;
    asus_ec_hwmon_chan = devm_kcalloc(
    dev, nr_types, sizeof(*asus_ec_hwmon_chan), GFP_KERNEL);
    if (!asus_ec_hwmon_chan)
    return -ENOMEM;
    ptr_asus_ec_ci = devm_kcalloc(dev, nr_types + 1,
    sizeof(*ptr_asus_ec_ci), GFP_KERNEL);
    if (!ptr_asus_ec_ci)
    return -ENOMEM;
    asus_ec_chip_info.info = ptr_asus_ec_ci;
    chip_info = &asus_ec_chip_info;
    for (type = 0; type < hwmon_max; ++type) {
    if (!nr_count[type])
    continue;
    status = asus_ec_hwmon_add_chan_info(asus_ec_hwmon_chan, dev,
    nr_count[type], type,
    hwmon_attributes[type]);
    if (status)
    return status;
// ptr_asus_ec_ci++ = asus_ec_hwmon_chan++;
    }
    dev_info(dev, "board has %d EC sensors that span %d registers",
    ec_data.nr_sensors, ec_data.nr_registers);
    hwdev = devm_hwmon_device_register_with_info(dev, "asusec",
    ec_data, chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwdev);
    }
    MODULE_DEVICE_TABLE(dmi, dmi_table);
    static struct platform_driver asus_ec_sensors_platform_driver = {
    .driver = {
    .name	= "asus-ec-sensors",
    },
    .probe = asus_ec_probe,
    };
    static struct platform_device *asus_ec_sensors_platform_device;
#[no_mangle]
unsafe extern "C" fn asus_ec_init() -> int __init {
    static int __init asus_ec_init(void)
    {
    asus_ec_sensors_platform_device =
    platform_create_bundle(&asus_ec_sensors_platform_driver,
    asus_ec_probe, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    if (IS_ERR(asus_ec_sensors_platform_device))
    return PTR_ERR(asus_ec_sensors_platform_device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asus_ec_exit() -> void __exit {
    static void __exit asus_ec_exit(void)
    {
    platform_device_unregister(asus_ec_sensors_platform_device);
    platform_driver_unregister(&asus_ec_sensors_platform_driver);
    }
    module_init(asus_ec_init);
    module_exit(asus_ec_exit);
    module_param_named(mutex_path, mutex_path_override, charp, 0);
    MODULE_PARM_DESC(mutex_path,
    "Override ACPI mutex path used to guard access to hardware");
    MODULE_AUTHOR("Eugene Shalygin <eugene.shalygin@gmail.com>");
    MODULE_DESCRIPTION(
    "HWMON driver for sensors accessible via ACPI EC in ASUS motherboards");
    MODULE_LICENSE("GPL");
