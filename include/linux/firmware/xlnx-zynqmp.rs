//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/xlnx-zynqmp.h
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
//
// Xilinx Zynq MPSoC Firmware layer
//
// Copyright (C) 2014-2021 Xilinx
// Copyright (C) 2022 - 2026 Advanced Micro Devices, Inc.
//
// Michal Simek <michal.simek@amd.com>
// Davorin Mista <davorin.mista@aggios.com>
// Jolly Shah <jollys@xilinx.com>
// Rajan Vaja <rajanv@xilinx.com>
//

pub const ZYNQMP_PM_VERSION_MAJOR: c_int = 1;
pub const ZYNQMP_PM_VERSION_MINOR: c_int = 0;

pub const ZYNQMP_TZ_VERSION_MAJOR: c_int = 1;
pub const ZYNQMP_TZ_VERSION_MINOR: c_int = 0;

// SMC SIP service Call Function Identifier Prefix
pub const PM_SIP_SVC: c_uint = 0xC2000000;
// SMC function ID to get SiP SVC version

// SiP Service Calls version numbers

// Fixed ID for FW specific APIs

// PM API versions
pub const PM_API_VERSION_1: c_int = 1;
pub const PM_API_VERSION_2: c_int = 2;
pub const PM_API_VERSION_3: c_int = 3;
pub const PM_PINCTRL_PARAM_SET_VERSION: c_int = 2;
// Family codes
pub const PM_ZYNQMP_FAMILY_CODE: c_uint = 0x1 /* ZynqMP family code */;
pub const PM_VERSAL_FAMILY_CODE: c_uint = 0x2 /* Versal family code */;
pub const PM_VERSAL_NET_FAMILY_CODE: c_uint = 0x3 /* Versal NET family code */;

// Firmware feature check version mask
pub const FIRMWARE_VERSION_MASK: c_uint = 0xFFFFU;
// ATF only commands
pub const TF_A_CLEAR_PM_STATE: c_uint = 0xa05;
pub const TF_A_PM_REGISTER_SGI: c_uint = 0xa04;
pub const PM_GET_TRUSTZONE_VERSION: c_uint = 0xa03;
pub const PM_SET_SUSPEND_MODE: c_uint = 0xa02;
pub const GET_CALLBACK_DATA: c_uint = 0xa01;
// Number of 32bits values in payload

// Number of 64bits arguments for SMC call

// Number of 32bits arguments for SMC call

// Number of arguments for a callback
pub const CB_ARG_CNT: c_int = 4;
// Payload size (consists of callback API ID + arguments)

// Node capabilities
pub const ZYNQMP_PM_CAPABILITY_ACCESS: c_uint = 0x1U;
pub const ZYNQMP_PM_CAPABILITY_CONTEXT: c_uint = 0x2U;
pub const ZYNQMP_PM_CAPABILITY_WAKEUP: c_uint = 0x4U;
pub const ZYNQMP_PM_CAPABILITY_UNUSABLE: c_uint = 0x8U;
// Loader commands
pub const PM_LOAD_PDI: c_uint = 0x701;
pub const PDI_SRC_DDR: c_uint = 0xF;
//
// Firmware FPGA Manager flags
// XILINX_ZYNQMP_PM_FPGA_FULL:	FPGA full reconfiguration
// XILINX_ZYNQMP_PM_FPGA_PARTIAL: FPGA partial reconfiguration
//
pub const XILINX_ZYNQMP_PM_FPGA_FULL: c_uint = 0x0U;

// FPGA Status Reg

//
// Node IDs for the Error Events.
//

// ZynqMP SD tap delay tuning
pub const SD_ITAPDLY: c_uint = 0xFF180314;
pub const SD_OTAPDLYSEL: c_uint = 0xFF180318;
//
// XPM_EVENT_ERROR_MASK_DDRMC_CR: Error event mask for DDRMC MC Correctable ECC Error.
//

//
// XPM_EVENT_ERROR_MASK_DDRMC_NCR: Error event mask for DDRMC MC Non-Correctable ECC Error.
//

// Node ID for all peripheral devices
pub const PM_DEV_ALL_PERIPH: c_uint = 0x18224FFFU;
// Node ID for all notifier callbacks
pub const PM_ALL_NOTIFIERS: c_uint = 0xFFFFFFFFU;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_module_id {
    PM_MODULE_ID = 0x0,
    XPM_MODULE_ID = 0x2,
    XSEM_MODULE_ID = 0x3,
    TF_A_MODULE_ID = 0xa,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_api_cb_id {
    PM_INIT_SUSPEND_CB = 30,
    PM_ACKNOWLEDGE_CB = 31,
    PM_NOTIFY_CB = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_api_id {
    PM_API_FEATURES = 0,
    PM_GET_API_VERSION = 1,
    PM_GET_NODE_STATUS = 3,
    PM_REGISTER_NOTIFIER = 5,
    PM_FORCE_POWERDOWN = 8,
    PM_REQUEST_WAKEUP = 10,
    PM_SYSTEM_SHUTDOWN = 12,
    PM_REQUEST_NODE = 13,
    PM_RELEASE_NODE = 14,
    PM_SET_REQUIREMENT = 15,
    PM_RESET_ASSERT = 17,
    PM_RESET_GET_STATUS = 18,
    PM_MMIO_WRITE = 19,
    PM_MMIO_READ = 20,
    PM_PM_INIT_FINALIZE = 21,
    PM_FPGA_LOAD = 22,
    PM_FPGA_GET_STATUS = 23,
    PM_GET_CHIPID = 24,
    PM_SECURE_SHA = 26,
    PM_PINCTRL_REQUEST = 28,
    PM_PINCTRL_RELEASE = 29,
    PM_PINCTRL_SET_FUNCTION = 31,
    PM_PINCTRL_CONFIG_PARAM_GET = 32,
    PM_PINCTRL_CONFIG_PARAM_SET = 33,
    PM_IOCTL = 34,
    PM_QUERY_DATA = 35,
    PM_CLOCK_ENABLE = 36,
    PM_CLOCK_DISABLE = 37,
    PM_CLOCK_GETSTATE = 38,
    PM_CLOCK_SETDIVIDER = 39,
    PM_CLOCK_GETDIVIDER = 40,
    PM_CLOCK_SETPARENT = 43,
    PM_CLOCK_GETPARENT = 44,
    PM_FPGA_READ = 46,
    PM_SECURE_AES = 47,
    PM_EFUSE_ACCESS = 53,
    PM_FEATURE_CHECK = 63,
}

// PMU-FW return status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_ret_status {
    XST_PM_SUCCESS = 0,
    XST_PM_INVALID_VERSION = 4,
    XST_PM_NO_FEATURE = 19,
    XST_PM_INVALID_CRC = 301,
    XST_PM_INTERNAL = 2000,
    XST_PM_CONFLICT = 2001,
    XST_PM_NO_ACCESS = 2002,
    XST_PM_INVALID_NODE = 2003,
    XST_PM_DOUBLE_REQ = 2004,
    XST_PM_ABORT_SUSPEND = 2005,
    XST_PM_MULT_USER = 2008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_ioctl_id {
    IOCTL_GET_RPU_OPER_MODE = 0,
    IOCTL_SET_RPU_OPER_MODE = 1,
    IOCTL_RPU_BOOT_ADDR_CONFIG = 2,
    IOCTL_TCM_COMB_CONFIG = 3,
    IOCTL_SET_TAPDELAY_BYPASS = 4,
    IOCTL_SD_DLL_RESET = 6,
    IOCTL_SET_SD_TAPDELAY = 7,
    IOCTL_SET_PLL_FRAC_MODE = 8,
    IOCTL_GET_PLL_FRAC_MODE = 9,
    IOCTL_SET_PLL_FRAC_DATA = 10,
    IOCTL_GET_PLL_FRAC_DATA = 11,
    IOCTL_WRITE_GGS = 12,
    IOCTL_READ_GGS = 13,
    IOCTL_WRITE_PGGS = 14,
    IOCTL_READ_PGGS = 15,
// Set healthy bit value
    IOCTL_SET_BOOT_HEALTH_STATUS = 17,
    IOCTL_OSPI_MUX_SELECT = 21,
// Register SGI to ATF
    IOCTL_REGISTER_SGI = 25,
// Runtime feature configuration
    IOCTL_SET_FEATURE_CONFIG = 26,
    IOCTL_GET_FEATURE_CONFIG = 27,
// IOCTL for Secure Read/Write Interface
    IOCTL_READ_REG = 28,
    IOCTL_MASK_WRITE_REG = 29,
// Dynamic SD/GEM configuration
    IOCTL_SET_SD_CONFIG = 30,
    IOCTL_SET_GEM_CONFIG = 31,
// IOCTL to get default/current QoS
    IOCTL_GET_QOS = 34,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_query_id {
    PM_QID_INVALID = 0,
    PM_QID_CLOCK_GET_NAME = 1,
    PM_QID_CLOCK_GET_TOPOLOGY = 2,
    PM_QID_CLOCK_GET_FIXEDFACTOR_PARAMS = 3,
    PM_QID_CLOCK_GET_PARENTS = 4,
    PM_QID_CLOCK_GET_ATTRIBUTES = 5,
    PM_QID_PINCTRL_GET_NUM_PINS = 6,
    PM_QID_PINCTRL_GET_NUM_FUNCTIONS = 7,
    PM_QID_PINCTRL_GET_NUM_FUNCTION_GROUPS = 8,
    PM_QID_PINCTRL_GET_FUNCTION_NAME = 9,
    PM_QID_PINCTRL_GET_FUNCTION_GROUPS = 10,
    PM_QID_PINCTRL_GET_PIN_GROUPS = 11,
    PM_QID_CLOCK_GET_NUM_CLOCKS = 12,
    PM_QID_CLOCK_GET_MAX_DIVISOR = 13,
    PM_QID_PINCTRL_GET_ATTRIBUTES = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_oper_mode {
    PM_RPU_MODE_LOCKSTEP = 0,
    PM_RPU_MODE_SPLIT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_boot_mem {
    PM_RPU_BOOTMEM_LOVEC = 0,
    PM_RPU_BOOTMEM_HIVEC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_tcm_comb {
    PM_RPU_TCM_SPLIT = 0,
    PM_RPU_TCM_COMB = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_reset_action {
    PM_RESET_ACTION_RELEASE = 0,
    PM_RESET_ACTION_ASSERT = 1,
    PM_RESET_ACTION_PULSE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_reset {
    ZYNQMP_PM_RESET_START = 1000,
    ZYNQMP_PM_RESET_PCIE_CFG = ZYNQMP_PM_RESET_START,
    ZYNQMP_PM_RESET_PCIE_BRIDGE = 1001,
    ZYNQMP_PM_RESET_PCIE_CTRL = 1002,
    ZYNQMP_PM_RESET_DP = 1003,
    ZYNQMP_PM_RESET_SWDT_CRF = 1004,
    ZYNQMP_PM_RESET_AFI_FM5 = 1005,
    ZYNQMP_PM_RESET_AFI_FM4 = 1006,
    ZYNQMP_PM_RESET_AFI_FM3 = 1007,
    ZYNQMP_PM_RESET_AFI_FM2 = 1008,
    ZYNQMP_PM_RESET_AFI_FM1 = 1009,
    ZYNQMP_PM_RESET_AFI_FM0 = 1010,
    ZYNQMP_PM_RESET_GDMA = 1011,
    ZYNQMP_PM_RESET_GPU_PP1 = 1012,
    ZYNQMP_PM_RESET_GPU_PP0 = 1013,
    ZYNQMP_PM_RESET_GPU = 1014,
    ZYNQMP_PM_RESET_GT = 1015,
    ZYNQMP_PM_RESET_SATA = 1016,
    ZYNQMP_PM_RESET_ACPU3_PWRON = 1017,
    ZYNQMP_PM_RESET_ACPU2_PWRON = 1018,
    ZYNQMP_PM_RESET_ACPU1_PWRON = 1019,
    ZYNQMP_PM_RESET_ACPU0_PWRON = 1020,
    ZYNQMP_PM_RESET_APU_L2 = 1021,
    ZYNQMP_PM_RESET_ACPU3 = 1022,
    ZYNQMP_PM_RESET_ACPU2 = 1023,
    ZYNQMP_PM_RESET_ACPU1 = 1024,
    ZYNQMP_PM_RESET_ACPU0 = 1025,
    ZYNQMP_PM_RESET_DDR = 1026,
    ZYNQMP_PM_RESET_APM_FPD = 1027,
    ZYNQMP_PM_RESET_SOFT = 1028,
    ZYNQMP_PM_RESET_GEM0 = 1029,
    ZYNQMP_PM_RESET_GEM1 = 1030,
    ZYNQMP_PM_RESET_GEM2 = 1031,
    ZYNQMP_PM_RESET_GEM3 = 1032,
    ZYNQMP_PM_RESET_QSPI = 1033,
    ZYNQMP_PM_RESET_UART0 = 1034,
    ZYNQMP_PM_RESET_UART1 = 1035,
    ZYNQMP_PM_RESET_SPI0 = 1036,
    ZYNQMP_PM_RESET_SPI1 = 1037,
    ZYNQMP_PM_RESET_SDIO0 = 1038,
    ZYNQMP_PM_RESET_SDIO1 = 1039,
    ZYNQMP_PM_RESET_CAN0 = 1040,
    ZYNQMP_PM_RESET_CAN1 = 1041,
    ZYNQMP_PM_RESET_I2C0 = 1042,
    ZYNQMP_PM_RESET_I2C1 = 1043,
    ZYNQMP_PM_RESET_TTC0 = 1044,
    ZYNQMP_PM_RESET_TTC1 = 1045,
    ZYNQMP_PM_RESET_TTC2 = 1046,
    ZYNQMP_PM_RESET_TTC3 = 1047,
    ZYNQMP_PM_RESET_SWDT_CRL = 1048,
    ZYNQMP_PM_RESET_NAND = 1049,
    ZYNQMP_PM_RESET_ADMA = 1050,
    ZYNQMP_PM_RESET_GPIO = 1051,
    ZYNQMP_PM_RESET_IOU_CC = 1052,
    ZYNQMP_PM_RESET_TIMESTAMP = 1053,
    ZYNQMP_PM_RESET_RPU_R50 = 1054,
    ZYNQMP_PM_RESET_RPU_R51 = 1055,
    ZYNQMP_PM_RESET_RPU_AMBA = 1056,
    ZYNQMP_PM_RESET_OCM = 1057,
    ZYNQMP_PM_RESET_RPU_PGE = 1058,
    ZYNQMP_PM_RESET_USB0_CORERESET = 1059,
    ZYNQMP_PM_RESET_USB1_CORERESET = 1060,
    ZYNQMP_PM_RESET_USB0_HIBERRESET = 1061,
    ZYNQMP_PM_RESET_USB1_HIBERRESET = 1062,
    ZYNQMP_PM_RESET_USB0_APB = 1063,
    ZYNQMP_PM_RESET_USB1_APB = 1064,
    ZYNQMP_PM_RESET_IPI = 1065,
    ZYNQMP_PM_RESET_APM_LPD = 1066,
    ZYNQMP_PM_RESET_RTC = 1067,
    ZYNQMP_PM_RESET_SYSMON = 1068,
    ZYNQMP_PM_RESET_AFI_FM6 = 1069,
    ZYNQMP_PM_RESET_LPD_SWDT = 1070,
    ZYNQMP_PM_RESET_FPD = 1071,
    ZYNQMP_PM_RESET_RPU_DBG1 = 1072,
    ZYNQMP_PM_RESET_RPU_DBG0 = 1073,
    ZYNQMP_PM_RESET_DBG_LPD = 1074,
    ZYNQMP_PM_RESET_DBG_FPD = 1075,
    ZYNQMP_PM_RESET_APLL = 1076,
    ZYNQMP_PM_RESET_DPLL = 1077,
    ZYNQMP_PM_RESET_VPLL = 1078,
    ZYNQMP_PM_RESET_IOPLL = 1079,
    ZYNQMP_PM_RESET_RPLL = 1080,
    ZYNQMP_PM_RESET_GPO3_PL_0 = 1081,
    ZYNQMP_PM_RESET_GPO3_PL_1 = 1082,
    ZYNQMP_PM_RESET_GPO3_PL_2 = 1083,
    ZYNQMP_PM_RESET_GPO3_PL_3 = 1084,
    ZYNQMP_PM_RESET_GPO3_PL_4 = 1085,
    ZYNQMP_PM_RESET_GPO3_PL_5 = 1086,
    ZYNQMP_PM_RESET_GPO3_PL_6 = 1087,
    ZYNQMP_PM_RESET_GPO3_PL_7 = 1088,
    ZYNQMP_PM_RESET_GPO3_PL_8 = 1089,
    ZYNQMP_PM_RESET_GPO3_PL_9 = 1090,
    ZYNQMP_PM_RESET_GPO3_PL_10 = 1091,
    ZYNQMP_PM_RESET_GPO3_PL_11 = 1092,
    ZYNQMP_PM_RESET_GPO3_PL_12 = 1093,
    ZYNQMP_PM_RESET_GPO3_PL_13 = 1094,
    ZYNQMP_PM_RESET_GPO3_PL_14 = 1095,
    ZYNQMP_PM_RESET_GPO3_PL_15 = 1096,
    ZYNQMP_PM_RESET_GPO3_PL_16 = 1097,
    ZYNQMP_PM_RESET_GPO3_PL_17 = 1098,
    ZYNQMP_PM_RESET_GPO3_PL_18 = 1099,
    ZYNQMP_PM_RESET_GPO3_PL_19 = 1100,
    ZYNQMP_PM_RESET_GPO3_PL_20 = 1101,
    ZYNQMP_PM_RESET_GPO3_PL_21 = 1102,
    ZYNQMP_PM_RESET_GPO3_PL_22 = 1103,
    ZYNQMP_PM_RESET_GPO3_PL_23 = 1104,
    ZYNQMP_PM_RESET_GPO3_PL_24 = 1105,
    ZYNQMP_PM_RESET_GPO3_PL_25 = 1106,
    ZYNQMP_PM_RESET_GPO3_PL_26 = 1107,
    ZYNQMP_PM_RESET_GPO3_PL_27 = 1108,
    ZYNQMP_PM_RESET_GPO3_PL_28 = 1109,
    ZYNQMP_PM_RESET_GPO3_PL_29 = 1110,
    ZYNQMP_PM_RESET_GPO3_PL_30 = 1111,
    ZYNQMP_PM_RESET_GPO3_PL_31 = 1112,
    ZYNQMP_PM_RESET_RPU_LS = 1113,
    ZYNQMP_PM_RESET_PS_ONLY = 1114,
    ZYNQMP_PM_RESET_PL = 1115,
    ZYNQMP_PM_RESET_PS_PL0 = 1116,
    ZYNQMP_PM_RESET_PS_PL1 = 1117,
    ZYNQMP_PM_RESET_PS_PL2 = 1118,
    ZYNQMP_PM_RESET_PS_PL3 = 1119,
    ZYNQMP_PM_RESET_END = ZYNQMP_PM_RESET_PS_PL3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_suspend_reason {
    SUSPEND_POWER_REQUEST = 201,
    SUSPEND_ALERT = 202,
    SUSPEND_SYSTEM_SHUTDOWN = 203,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_request_ack {
    ZYNQMP_PM_REQUEST_ACK_NO = 1,
    ZYNQMP_PM_REQUEST_ACK_BLOCKING = 2,
    ZYNQMP_PM_REQUEST_ACK_NON_BLOCKING = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_node_id {
    NODE_SD_0 = 39,
    NODE_SD_1 = 40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_type {
    PM_TAPDELAY_INPUT = 0,
    PM_TAPDELAY_OUTPUT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dll_reset_type {
    PM_DLL_RESET_ASSERT = 0,
    PM_DLL_RESET_RELEASE = 1,
    PM_DLL_RESET_PULSE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_config_param {
    PM_PINCTRL_CONFIG_SLEW_RATE = 0,
    PM_PINCTRL_CONFIG_BIAS_STATUS = 1,
    PM_PINCTRL_CONFIG_PULL_CTRL = 2,
    PM_PINCTRL_CONFIG_SCHMITT_CMOS = 3,
    PM_PINCTRL_CONFIG_DRIVE_STRENGTH = 4,
    PM_PINCTRL_CONFIG_VOLTAGE_STATUS = 5,
    PM_PINCTRL_CONFIG_TRI_STATE = 6,
    PM_PINCTRL_CONFIG_MAX = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_slew_rate {
    PM_PINCTRL_SLEW_RATE_FAST = 0,
    PM_PINCTRL_SLEW_RATE_SLOW = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_bias_status {
    PM_PINCTRL_BIAS_DISABLE = 0,
    PM_PINCTRL_BIAS_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_pull_ctrl {
    PM_PINCTRL_BIAS_PULL_DOWN = 0,
    PM_PINCTRL_BIAS_PULL_UP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_schmitt_cmos {
    PM_PINCTRL_INPUT_TYPE_CMOS = 0,
    PM_PINCTRL_INPUT_TYPE_SCHMITT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_drive_strength {
    PM_PINCTRL_DRIVE_STRENGTH_2MA = 0,
    PM_PINCTRL_DRIVE_STRENGTH_4MA = 1,
    PM_PINCTRL_DRIVE_STRENGTH_8MA = 2,
    PM_PINCTRL_DRIVE_STRENGTH_12MA = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_tri_state {
    PM_PINCTRL_TRI_STATE_DISABLE = 0,
    PM_PINCTRL_TRI_STATE_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_shutdown_type {
    ZYNQMP_PM_SHUTDOWN_TYPE_SHUTDOWN = 0,
    ZYNQMP_PM_SHUTDOWN_TYPE_RESET = 1,
    ZYNQMP_PM_SHUTDOWN_TYPE_SETSCOPE_ONLY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_shutdown_subtype {
    ZYNQMP_PM_SHUTDOWN_SUBTYPE_SUBSYSTEM = 0,
    ZYNQMP_PM_SHUTDOWN_SUBTYPE_PS_ONLY = 1,
    ZYNQMP_PM_SHUTDOWN_SUBTYPE_SYSTEM = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_signal_type {
    PM_TAPDELAY_NAND_DQS_IN = 0,
    PM_TAPDELAY_NAND_DQS_OUT = 1,
    PM_TAPDELAY_QSPI = 2,
    PM_TAPDELAY_MAX = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_bypass_ctrl {
    PM_TAPDELAY_BYPASS_DISABLE = 0,
    PM_TAPDELAY_BYPASS_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ospi_mux_select_type {
    PM_OSPI_MUX_SEL_DMA = 0,
    PM_OSPI_MUX_SEL_LINEAR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_feature_config_id {
    PM_FEATURE_INVALID = 0,
    PM_FEATURE_OVERTEMP_STATUS = 1,
    PM_FEATURE_OVERTEMP_VALUE = 2,
    PM_FEATURE_EXTWDT_STATUS = 3,
    PM_FEATURE_EXTWDT_VALUE = 4,
}

//
// enum pm_sd_config_type - PM SD configuration.
// @SD_CONFIG_EMMC_SEL: To set SD_EMMC_SEL in CTRL_REG_SD and SD_SLOTTYPE
// @SD_CONFIG_BASECLK: To set SD_BASECLK in SD_CONFIG_REG1
// @SD_CONFIG_8BIT: To set SD_8BIT in SD_CONFIG_REG2
// @SD_CONFIG_FIXED: To set fixed config registers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_sd_config_type {
    SD_CONFIG_EMMC_SEL = 1,
    SD_CONFIG_BASECLK = 2,
    SD_CONFIG_8BIT = 3,
    SD_CONFIG_FIXED = 4,
}

//
// enum pm_gem_config_type - PM GEM configuration.
// @GEM_CONFIG_SGMII_MODE: To set GEM_SGMII_MODE in GEM_CLK_CTRL register
// @GEM_CONFIG_FIXED: To set fixed config registers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_gem_config_type {
    GEM_CONFIG_SGMII_MODE = 1,
    GEM_CONFIG_FIXED = 2,
}

//
// enum pm_node_status - Device node status provided by xilpm fw
// @PM_NODE_UNUSED: Device is not used
// @PM_NODE_RUNNING: Device is power-on and out of reset
// @PM_NODE_HALT: Device is power-on but in the reset state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_node_status {
    PM_NODE_UNUSED = 0,
    PM_NODE_RUNNING = 1,
    PM_NODE_HALT = 12,
}

//
// struct zynqmp_pm_query_data - PM query data
// @qid:	query ID
// @arg1:	Argument 1 of query data
// @arg2:	Argument 2 of query data
// @arg3:	Argument 3 of query data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_pm_query_data {
    pub qid: u32,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32,
}

extern "C" {
    pub fn zynqmp_pm_invoke_fn(pm_api_id: u32, ret_payload: *mut u32, num_args: u32, ...) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_invoke_fw_fn(pm_api_id: u32, ret_payload: *mut u32, num_args: u32, ...) -> c_int;
}

extern "C" {
    pub fn zynqmp_pm_get_api_version(version: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_chipid(idcode: *mut u32, version: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_family_info(family: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_query_data(qdata: zynqmp_pm_query_data, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_enable(clock_id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_disable(clock_id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_getstate(clock_id: u32, state: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_setdivider(clock_id: u32, divider: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_getdivider(clock_id: u32, divider: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_setparent(clock_id: u32, parent_id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_clock_getparent(clock_id: u32, parent_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_pll_frac_mode(clk_id: u32, mode: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_pll_frac_mode(clk_id: u32, mode: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_pll_frac_data(clk_id: u32, data: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_pll_frac_data(clk_id: u32, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_sd_tapdelay(node_id: u32, type: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_sd_dll_reset(node_id: u32, type: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_ospi_mux_select(dev_id: u32, select: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_reset_get_status(reset: u32, status: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_bootmode_read(ps_mode: *mut u32) -> c_uint;
}
extern "C" {
    pub fn zynqmp_pm_bootmode_write(ps_mode: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_suspend_mode(mode: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_release_node(node: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_efuse_access(address: u64, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_fpga_load(address: u64, size: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_fpga_get_status(value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_fpga_get_config_status(value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_write_ggs(index: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_read_ggs(index: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_write_pggs(index: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_read_pggs(index: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_tapdelay_bypass(index: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_system_shutdown(type: u32, subtype: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_boot_health_status(value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_pinctrl_request(pin: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_pinctrl_release(pin: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_pinctrl_set_function(pin: u32, id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_load_pdi(src: u32, address: u64) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_feature(api_id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_is_function_supported(api_id: u32, id: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_feature_config(id: pm_feature_config_id, value: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_feature_config(id: pm_feature_config_id, payload: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_sec_read_reg(node_id: u32, offset: u32, ret_value: *mut u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_register_sgi(sgi_num: u32, reset: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_get_rpu_mode(node_id: u32, rpu_mode: *mut rpu_oper_mode) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_rpu_mode(node_id: u32, rpu_mode: rpu_oper_mode) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_tcm_config(node_id: u32, tcm_mode: rpu_tcm_comb) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_start_rpu(node: u32, bootaddr: u64) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_stop_rpu(node: u32) -> c_int;
}
extern "C" {
    pub fn zynqmp_pm_set_sd_config(node: u32, config: pm_sd_config_type, value: u32) -> c_int;
}

