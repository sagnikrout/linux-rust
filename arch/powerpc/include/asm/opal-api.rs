//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/opal-api.h
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
// OPAL API definitions.
//
// Copyright 2011-2015 IBM Corp.
//
// OPAL APIs
// Return codes
pub const OPAL_SUCCESS: c_int = 0;

// API Tokens (in r0)

pub const OPAL_TEST: c_int = 0;
pub const OPAL_CONSOLE_WRITE: c_int = 1;
pub const OPAL_CONSOLE_READ: c_int = 2;
pub const OPAL_RTC_READ: c_int = 3;
pub const OPAL_RTC_WRITE: c_int = 4;
pub const OPAL_CEC_POWER_DOWN: c_int = 5;
pub const OPAL_CEC_REBOOT: c_int = 6;
pub const OPAL_READ_NVRAM: c_int = 7;
pub const OPAL_WRITE_NVRAM: c_int = 8;
pub const OPAL_HANDLE_INTERRUPT: c_int = 9;
pub const OPAL_POLL_EVENTS: c_int = 10;
pub const OPAL_PCI_SET_HUB_TCE_MEMORY: c_int = 11;
pub const OPAL_PCI_SET_PHB_TCE_MEMORY: c_int = 12;
pub const OPAL_PCI_CONFIG_READ_BYTE: c_int = 13;
pub const OPAL_PCI_CONFIG_READ_HALF_WORD: c_int = 14;
pub const OPAL_PCI_CONFIG_READ_WORD: c_int = 15;
pub const OPAL_PCI_CONFIG_WRITE_BYTE: c_int = 16;
pub const OPAL_PCI_CONFIG_WRITE_HALF_WORD: c_int = 17;
pub const OPAL_PCI_CONFIG_WRITE_WORD: c_int = 18;
pub const OPAL_SET_XIVE: c_int = 19;
pub const OPAL_GET_XIVE: c_int = 20;

pub const OPAL_REGISTER_OPAL_EXCEPTION_HANDLER: c_int = 22;
pub const OPAL_PCI_EEH_FREEZE_STATUS: c_int = 23;
pub const OPAL_PCI_SHPC: c_int = 24;
pub const OPAL_CONSOLE_WRITE_BUFFER_SPACE: c_int = 25;
pub const OPAL_PCI_EEH_FREEZE_CLEAR: c_int = 26;
pub const OPAL_PCI_PHB_MMIO_ENABLE: c_int = 27;
pub const OPAL_PCI_SET_PHB_MEM_WINDOW: c_int = 28;
pub const OPAL_PCI_MAP_PE_MMIO_WINDOW: c_int = 29;
pub const OPAL_PCI_SET_PHB_TABLE_MEMORY: c_int = 30;
pub const OPAL_PCI_SET_PE: c_int = 31;
pub const OPAL_PCI_SET_PELTV: c_int = 32;
pub const OPAL_PCI_SET_MVE: c_int = 33;
pub const OPAL_PCI_SET_MVE_ENABLE: c_int = 34;
pub const OPAL_PCI_GET_XIVE_REISSUE: c_int = 35;
pub const OPAL_PCI_SET_XIVE_REISSUE: c_int = 36;
pub const OPAL_PCI_SET_XIVE_PE: c_int = 37;
pub const OPAL_GET_XIVE_SOURCE: c_int = 38;
pub const OPAL_GET_MSI_32: c_int = 39;
pub const OPAL_GET_MSI_64: c_int = 40;
pub const OPAL_START_CPU: c_int = 41;
pub const OPAL_QUERY_CPU_STATUS: c_int = 42;

pub const OPAL_PCI_MAP_PE_DMA_WINDOW: c_int = 44;
pub const OPAL_PCI_MAP_PE_DMA_WINDOW_REAL: c_int = 45;
pub const OPAL_PCI_RESET: c_int = 49;
pub const OPAL_PCI_GET_HUB_DIAG_DATA: c_int = 50;
pub const OPAL_PCI_GET_PHB_DIAG_DATA: c_int = 51;
pub const OPAL_PCI_FENCE_PHB: c_int = 52;
pub const OPAL_PCI_REINIT: c_int = 53;
pub const OPAL_PCI_MASK_PE_ERROR: c_int = 54;
pub const OPAL_SET_SLOT_LED_STATUS: c_int = 55;
pub const OPAL_GET_EPOW_STATUS: c_int = 56;
pub const OPAL_SET_SYSTEM_ATTENTION_LED: c_int = 57;
pub const OPAL_RESERVED1: c_int = 58;
pub const OPAL_RESERVED2: c_int = 59;
pub const OPAL_PCI_NEXT_ERROR: c_int = 60;
pub const OPAL_PCI_EEH_FREEZE_STATUS2: c_int = 61;
pub const OPAL_PCI_POLL: c_int = 62;
pub const OPAL_PCI_MSI_EOI: c_int = 63;
pub const OPAL_PCI_GET_PHB_DIAG_DATA2: c_int = 64;
pub const OPAL_XSCOM_READ: c_int = 65;
pub const OPAL_XSCOM_WRITE: c_int = 66;
pub const OPAL_LPC_READ: c_int = 67;
pub const OPAL_LPC_WRITE: c_int = 68;
pub const OPAL_RETURN_CPU: c_int = 69;
pub const OPAL_REINIT_CPUS: c_int = 70;
pub const OPAL_ELOG_READ: c_int = 71;
pub const OPAL_ELOG_WRITE: c_int = 72;
pub const OPAL_ELOG_ACK: c_int = 73;
pub const OPAL_ELOG_RESEND: c_int = 74;
pub const OPAL_ELOG_SIZE: c_int = 75;
pub const OPAL_FLASH_VALIDATE: c_int = 76;
pub const OPAL_FLASH_MANAGE: c_int = 77;
pub const OPAL_FLASH_UPDATE: c_int = 78;
pub const OPAL_RESYNC_TIMEBASE: c_int = 79;
pub const OPAL_CHECK_TOKEN: c_int = 80;
pub const OPAL_DUMP_INIT: c_int = 81;
pub const OPAL_DUMP_INFO: c_int = 82;
pub const OPAL_DUMP_READ: c_int = 83;
pub const OPAL_DUMP_ACK: c_int = 84;
pub const OPAL_GET_MSG: c_int = 85;
pub const OPAL_CHECK_ASYNC_COMPLETION: c_int = 86;
pub const OPAL_SYNC_HOST_REBOOT: c_int = 87;
pub const OPAL_SENSOR_READ: c_int = 88;
pub const OPAL_GET_PARAM: c_int = 89;
pub const OPAL_SET_PARAM: c_int = 90;
pub const OPAL_DUMP_RESEND: c_int = 91;

pub const OPAL_PCI_SET_PHB_CAPI_MODE: c_int = 93;
pub const OPAL_DUMP_INFO2: c_int = 94;
pub const OPAL_WRITE_OPPANEL_ASYNC: c_int = 95;
pub const OPAL_PCI_ERR_INJECT: c_int = 96;
pub const OPAL_PCI_EEH_FREEZE_SET: c_int = 97;
pub const OPAL_HANDLE_HMI: c_int = 98;
pub const OPAL_CONFIG_CPU_IDLE_STATE: c_int = 99;
pub const OPAL_SLW_SET_REG: c_int = 100;
pub const OPAL_REGISTER_DUMP_REGION: c_int = 101;
pub const OPAL_UNREGISTER_DUMP_REGION: c_int = 102;
pub const OPAL_WRITE_TPO: c_int = 103;
pub const OPAL_READ_TPO: c_int = 104;
pub const OPAL_GET_DPO_STATUS: c_int = 105;

pub const OPAL_IPMI_SEND: c_int = 107;
pub const OPAL_IPMI_RECV: c_int = 108;
pub const OPAL_I2C_REQUEST: c_int = 109;
pub const OPAL_FLASH_READ: c_int = 110;
pub const OPAL_FLASH_WRITE: c_int = 111;
pub const OPAL_FLASH_ERASE: c_int = 112;
pub const OPAL_PRD_MSG: c_int = 113;
pub const OPAL_LEDS_GET_INDICATOR: c_int = 114;
pub const OPAL_LEDS_SET_INDICATOR: c_int = 115;
pub const OPAL_CEC_REBOOT2: c_int = 116;
pub const OPAL_CONSOLE_FLUSH: c_int = 117;
pub const OPAL_GET_DEVICE_TREE: c_int = 118;
pub const OPAL_PCI_GET_PRESENCE_STATE: c_int = 119;
pub const OPAL_PCI_GET_POWER_STATE: c_int = 120;
pub const OPAL_PCI_SET_POWER_STATE: c_int = 121;
pub const OPAL_INT_GET_XIRR: c_int = 122;
pub const OPAL_INT_SET_CPPR: c_int = 123;
pub const OPAL_INT_EOI: c_int = 124;
pub const OPAL_INT_SET_MFRR: c_int = 125;
pub const OPAL_PCI_TCE_KILL: c_int = 126;
pub const OPAL_NMMU_SET_PTCR: c_int = 127;
pub const OPAL_XIVE_RESET: c_int = 128;
pub const OPAL_XIVE_GET_IRQ_INFO: c_int = 129;
pub const OPAL_XIVE_GET_IRQ_CONFIG: c_int = 130;
pub const OPAL_XIVE_SET_IRQ_CONFIG: c_int = 131;
pub const OPAL_XIVE_GET_QUEUE_INFO: c_int = 132;
pub const OPAL_XIVE_SET_QUEUE_INFO: c_int = 133;
pub const OPAL_XIVE_DONATE_PAGE: c_int = 134;
pub const OPAL_XIVE_ALLOCATE_VP_BLOCK: c_int = 135;
pub const OPAL_XIVE_FREE_VP_BLOCK: c_int = 136;
pub const OPAL_XIVE_GET_VP_INFO: c_int = 137;
pub const OPAL_XIVE_SET_VP_INFO: c_int = 138;
pub const OPAL_XIVE_ALLOCATE_IRQ: c_int = 139;
pub const OPAL_XIVE_FREE_IRQ: c_int = 140;
pub const OPAL_XIVE_SYNC: c_int = 141;
pub const OPAL_XIVE_DUMP: c_int = 142;
pub const OPAL_XIVE_GET_QUEUE_STATE: c_int = 143;
pub const OPAL_XIVE_SET_QUEUE_STATE: c_int = 144;
pub const OPAL_SIGNAL_SYSTEM_RESET: c_int = 145;
pub const OPAL_NPU_INIT_CONTEXT: c_int = 146;
pub const OPAL_NPU_DESTROY_CONTEXT: c_int = 147;
pub const OPAL_NPU_MAP_LPAR: c_int = 148;
pub const OPAL_IMC_COUNTERS_INIT: c_int = 149;
pub const OPAL_IMC_COUNTERS_START: c_int = 150;
pub const OPAL_IMC_COUNTERS_STOP: c_int = 151;
pub const OPAL_GET_POWERCAP: c_int = 152;
pub const OPAL_SET_POWERCAP: c_int = 153;
pub const OPAL_GET_POWER_SHIFT_RATIO: c_int = 154;
pub const OPAL_SET_POWER_SHIFT_RATIO: c_int = 155;
pub const OPAL_SENSOR_GROUP_CLEAR: c_int = 156;
pub const OPAL_PCI_SET_P2P: c_int = 157;
pub const OPAL_QUIESCE: c_int = 158;
pub const OPAL_NPU_SPA_SETUP: c_int = 159;
pub const OPAL_NPU_SPA_CLEAR_CACHE: c_int = 160;
pub const OPAL_NPU_TL_SET: c_int = 161;
pub const OPAL_SENSOR_READ_U64: c_int = 162;
pub const OPAL_SENSOR_GROUP_ENABLE: c_int = 163;
pub const OPAL_PCI_GET_PBCQ_TUNNEL_BAR: c_int = 164;
pub const OPAL_PCI_SET_PBCQ_TUNNEL_BAR: c_int = 165;
pub const OPAL_HANDLE_HMI2: c_int = 166;
pub const OPAL_NX_COPROC_INIT: c_int = 167;
pub const OPAL_XIVE_GET_VP_STATE: c_int = 170;
pub const OPAL_MPIPL_UPDATE: c_int = 173;
pub const OPAL_MPIPL_REGISTER_TAG: c_int = 174;
pub const OPAL_MPIPL_QUERY_TAG: c_int = 175;
pub const OPAL_SECVAR_GET: c_int = 176;
pub const OPAL_SECVAR_GET_NEXT: c_int = 177;
pub const OPAL_SECVAR_ENQUEUE_UPDATE: c_int = 178;
pub const OPAL_LAST: c_int = 178;

// Device tree flags
//
// Flags set in power-mgmt nodes in device tree describing
// idle states that are supported in the platform.
//
pub const OPAL_PM_TIMEBASE_STOP: c_uint = 0x00000002;
pub const OPAL_PM_LOSE_HYP_CONTEXT: c_uint = 0x00002000;
pub const OPAL_PM_LOSE_FULL_CONTEXT: c_uint = 0x00004000;
pub const OPAL_PM_NAP_ENABLED: c_uint = 0x00010000;
pub const OPAL_PM_SLEEP_ENABLED: c_uint = 0x00020000;
pub const OPAL_PM_WINKLE_ENABLED: c_uint = 0x00040000;
pub const OPAL_PM_SLEEP_ENABLED_ER1: c_uint = 0x00080000 /* with workaround */;
pub const OPAL_PM_STOP_INST_FAST: c_uint = 0x00100000;
pub const OPAL_PM_STOP_INST_DEEP: c_uint = 0x00200000;
//
// OPAL_CONFIG_CPU_IDLE_STATE parameters
//
pub const OPAL_CONFIG_IDLE_FASTSLEEP: c_int = 1;
pub const OPAL_CONFIG_IDLE_UNDO: c_int = 0;
pub const OPAL_CONFIG_IDLE_APPLY: c_int = 1;
// Other enums
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalFreezeState {
    OPAL_EEH_STOPPED_NOT_FROZEN = 0,
    OPAL_EEH_STOPPED_MMIO_FREEZE = 1,
    OPAL_EEH_STOPPED_DMA_FREEZE = 2,
    OPAL_EEH_STOPPED_MMIO_DMA_FREEZE = 3,
    OPAL_EEH_STOPPED_RESET = 4,
    OPAL_EEH_STOPPED_TEMP_UNAVAIL = 5,
    OPAL_EEH_STOPPED_PERM_UNAVAIL = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalEehFreezeActionToken {
    OPAL_EEH_ACTION_CLEAR_FREEZE_MMIO = 1,
    OPAL_EEH_ACTION_CLEAR_FREEZE_DMA = 2,
    OPAL_EEH_ACTION_CLEAR_FREEZE_ALL = 3,

    OPAL_EEH_ACTION_SET_FREEZE_MMIO = 1,
    OPAL_EEH_ACTION_SET_FREEZE_DMA  = 2,
    OPAL_EEH_ACTION_SET_FREEZE_ALL  = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciStatusToken {
    OPAL_EEH_NO_ERROR	= 0,
    OPAL_EEH_IOC_ERROR	= 1,
    OPAL_EEH_PHB_ERROR	= 2,
    OPAL_EEH_PE_ERROR	= 3,
    OPAL_EEH_PE_MMIO_ERROR	= 4,
    OPAL_EEH_PE_DMA_ERROR	= 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciErrorSeverity {
    OPAL_EEH_SEV_NO_ERROR	= 0,
    OPAL_EEH_SEV_IOC_DEAD	= 1,
    OPAL_EEH_SEV_PHB_DEAD	= 2,
    OPAL_EEH_SEV_PHB_FENCED	= 3,
    OPAL_EEH_SEV_PE_ER	= 4,
    OPAL_EEH_SEV_INF	= 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalErrinjectType {
    OPAL_ERR_INJECT_TYPE_IOA_BUS_ERR	= 0,
    OPAL_ERR_INJECT_TYPE_IOA_BUS_ERR64	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalErrinjectFunc {
// IOA bus specific errors
    OPAL_ERR_INJECT_FUNC_IOA_LD_MEM_ADDR	= 0,
    OPAL_ERR_INJECT_FUNC_IOA_LD_MEM_DATA	= 1,
    OPAL_ERR_INJECT_FUNC_IOA_LD_IO_ADDR	= 2,
    OPAL_ERR_INJECT_FUNC_IOA_LD_IO_DATA	= 3,
    OPAL_ERR_INJECT_FUNC_IOA_LD_CFG_ADDR	= 4,
    OPAL_ERR_INJECT_FUNC_IOA_LD_CFG_DATA	= 5,
    OPAL_ERR_INJECT_FUNC_IOA_ST_MEM_ADDR	= 6,
    OPAL_ERR_INJECT_FUNC_IOA_ST_MEM_DATA	= 7,
    OPAL_ERR_INJECT_FUNC_IOA_ST_IO_ADDR	= 8,
    OPAL_ERR_INJECT_FUNC_IOA_ST_IO_DATA	= 9,
    OPAL_ERR_INJECT_FUNC_IOA_ST_CFG_ADDR	= 10,
    OPAL_ERR_INJECT_FUNC_IOA_ST_CFG_DATA	= 11,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_RD_ADDR	= 12,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_RD_DATA	= 13,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_RD_MASTER	= 14,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_RD_TARGET	= 15,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_WR_ADDR	= 16,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_WR_DATA	= 17,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_WR_MASTER	= 18,
    OPAL_ERR_INJECT_FUNC_IOA_DMA_WR_TARGET	= 19,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMmioWindowType {
    OPAL_M32_WINDOW_TYPE = 1,
    OPAL_M64_WINDOW_TYPE = 2,
    OPAL_IO_WINDOW_TYPE  = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalExceptionHandler {
    OPAL_MACHINE_CHECK_HANDLER	    = 1,
    OPAL_HYPERVISOR_MAINTENANCE_HANDLER = 2,
    OPAL_SOFTPATCH_HANDLER		    = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPendingState {
    OPAL_EVENT_OPAL_INTERNAL   = 0x1,
    OPAL_EVENT_NVRAM	   = 0x2,
    OPAL_EVENT_RTC		   = 0x4,
    OPAL_EVENT_CONSOLE_OUTPUT  = 0x8,
    OPAL_EVENT_CONSOLE_INPUT   = 0x10,
    OPAL_EVENT_ERROR_LOG_AVAIL = 0x20,
    OPAL_EVENT_ERROR_LOG	   = 0x40,
    OPAL_EVENT_EPOW		   = 0x80,
    OPAL_EVENT_LED_STATUS	   = 0x100,
    OPAL_EVENT_PCI_ERROR	   = 0x200,
    OPAL_EVENT_DUMP_AVAIL	   = 0x400,
    OPAL_EVENT_MSG_PENDING	   = 0x800,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalThreadStatus {
    OPAL_THREAD_INACTIVE = 0x0,
    OPAL_THREAD_STARTED = 0x1,
    OPAL_THREAD_UNAVAILABLE = 0x2 /* opal-v3 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciBusCompare {
    OpalPciBusAny	= 0,	/* Any bus number match */
    OpalPciBus3Bits	= 2,	/* Match top 3 bits of bus number */
    OpalPciBus4Bits	= 3,	/* Match top 4 bits of bus number */
    OpalPciBus5Bits	= 4,	/* Match top 5 bits of bus number */
    OpalPciBus6Bits	= 5,	/* Match top 6 bits of bus number */
    OpalPciBus7Bits	= 6,	/* Match top 7 bits of bus number */
    OpalPciBusAll	= 7,	/* Match bus number exactly */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalDeviceCompare {
    OPAL_IGNORE_RID_DEVICE_NUMBER = 0,
    OPAL_COMPARE_RID_DEVICE_NUMBER = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalFuncCompare {
    OPAL_IGNORE_RID_FUNCTION_NUMBER = 0,
    OPAL_COMPARE_RID_FUNCTION_NUMBER = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPeAction {
    OPAL_UNMAP_PE = 0,
    OPAL_MAP_PE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPeltvAction {
    OPAL_REMOVE_PE_FROM_DOMAIN = 0,
    OPAL_ADD_PE_TO_DOMAIN = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMveEnableAction {
    OPAL_DISABLE_MVE = 0,
    OPAL_ENABLE_MVE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalM64Action {
    OPAL_DISABLE_M64 = 0,
    OPAL_ENABLE_M64_SPLIT = 1,
    OPAL_ENABLE_M64_NON_SPLIT = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciResetScope {
    OPAL_RESET_PHB_COMPLETE		= 1,
    OPAL_RESET_PCI_LINK		= 2,
    OPAL_RESET_PHB_ERROR		= 3,
    OPAL_RESET_PCI_HOT		= 4,
    OPAL_RESET_PCI_FUNDAMENTAL	= 5,
    OPAL_RESET_PCI_IODA_TABLE	= 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciReinitScope {
//
// Note: we chose values that do not overlap
// OpalPciResetScope as OPAL v2 used the same
// enum for both
//
    OPAL_REINIT_PCI_DEV = 1000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciResetState {
    OPAL_DEASSERT_RESET = 0,
    OPAL_ASSERT_RESET   = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciSlotPresence {
    OPAL_PCI_SLOT_EMPTY	= 0,
    OPAL_PCI_SLOT_PRESENT	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalPciSlotPower {
    OPAL_PCI_SLOT_POWER_OFF	= 0,
    OPAL_PCI_SLOT_POWER_ON	= 1,
    OPAL_PCI_SLOT_OFFLINE	= 2,
    OPAL_PCI_SLOT_ONLINE	= 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSlotLedType {
    OPAL_SLOT_LED_TYPE_ID = 0,	/* IDENTIFY LED */
    OPAL_SLOT_LED_TYPE_FAULT = 1,	/* FAULT LED */
    OPAL_SLOT_LED_TYPE_ATTN = 2,	/* System Attention LED */
    OPAL_SLOT_LED_TYPE_MAX = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSlotLedState {
    OPAL_SLOT_LED_STATE_OFF = 0,	/* LED is OFF */
    OPAL_SLOT_LED_STATE_ON = 1	/* LED is ON */
}

//
// Address cycle types for LPC accesses. These also correspond
// to the content of the first cell of the "reg" property for
// device nodes on the LPC bus
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalLPCAddressType {
    OPAL_LPC_MEM	= 0,
    OPAL_LPC_IO	= 1,
    OPAL_LPC_FW	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_msg_type {
    OPAL_MSG_ASYNC_COMP	= 0,	/* params[0] = token, params[1] = rc,
// additional params function-specific
//
    OPAL_MSG_MEM_ERR	= 1,
    OPAL_MSG_EPOW		= 2,
    OPAL_MSG_SHUTDOWN	= 3,	/* params[0] = 1 reboot, 0 shutdown */
    OPAL_MSG_HMI_EVT	= 4,
    OPAL_MSG_DPO		= 5,
    OPAL_MSG_PRD		= 6,
    OPAL_MSG_OCC		= 7,
    OPAL_MSG_PRD2		= 8,
    OPAL_MSG_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_msg {
    pub msg_type: __be32,
    pub reserved: __be32,
    pub params: [__be64; 8],
}

// System parameter permission
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSysparamPerm {
    OPAL_SYSPARAM_READ  = 0x1,
    OPAL_SYSPARAM_WRITE = 0x2,
    OPAL_SYSPARAM_RW    = (OPAL_SYSPARAM_READ | OPAL_SYSPARAM_WRITE),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_ipmi_msg {
    pub version: u8,
    pub netfn: u8,
    pub cmd: u8,
    pub data: [u8; ],
}

// FSP memory errors handling
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMemErr_Version {
    OpalMemErr_V1 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMemErrType {
    OPAL_MEM_ERR_TYPE_RESILIENCE	= 0,
    OPAL_MEM_ERR_TYPE_DYN_DALLOC,
}

// Memory Reilience error type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMemErr_ResilErrType {
    OPAL_MEM_RESILIENCE_CE		= 0,
    OPAL_MEM_RESILIENCE_UE,
    OPAL_MEM_RESILIENCE_UE_SCRUB,
}

// Dynamic Memory Deallocation type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalMemErr_DynErrType {
    OPAL_MEM_DYNAMIC_DEALLOC	= 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalMemoryErrorData {
    pub /: *mut *mut OpalMemErr_Version version:8; / 0x00,
    pub /: *mut *mut OpalMemErrType type:8; / 0x01,
    pub /: *mut *mut __be16 flags; / 0x02,
    pub /: *mut *mut uint8_t reserved_1[4]; / 0x04,
// Memory Resilience corrected/uncorrected error info
    pub resil_err_type:8: OpalMemErr_ResilErrType,
    pub reserved_1: [u8; 7],
    pub physical_address_start: __be64,
    pub physical_address_end: __be64,
    pub resilience: },
// Dynamic memory deallocation error info
    pub dyn_err_type:8: OpalMemErr_DynErrType,
    pub reserved_1: [u8; 7],
    pub physical_address_start: __be64,
    pub physical_address_end: __be64,
    pub dyn_dealloc: },
    pub u: },
}

// HMI interrupt event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_Version {
    OpalHMIEvt_V1 = 1,
    OpalHMIEvt_V2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_Severity {
    OpalHMI_SEV_NO_ERROR = 0,
    OpalHMI_SEV_WARNING = 1,
    OpalHMI_SEV_ERROR_SYNC = 2,
    OpalHMI_SEV_FATAL = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_Disposition {
    OpalHMI_DISPOSITION_RECOVERED = 0,
    OpalHMI_DISPOSITION_NOT_RECOVERED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_ErrType {
    OpalHMI_ERROR_MALFUNC_ALERT	= 0,
    OpalHMI_ERROR_PROC_RECOV_DONE,
    OpalHMI_ERROR_PROC_RECOV_DONE_AGAIN,
    OpalHMI_ERROR_PROC_RECOV_MASKED,
    OpalHMI_ERROR_TFAC,
    OpalHMI_ERROR_TFMR_PARITY,
    OpalHMI_ERROR_HA_OVERFLOW_WARN,
    OpalHMI_ERROR_XSCOM_FAIL,
    OpalHMI_ERROR_XSCOM_DONE,
    OpalHMI_ERROR_SCOM_FIR,
    OpalHMI_ERROR_DEBUG_TRIG_FIR,
    OpalHMI_ERROR_HYP_RESOURCE,
    OpalHMI_ERROR_CAPP_RECOVERY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_XstopType {
    CHECKSTOP_TYPE_UNKNOWN	=	0,
    CHECKSTOP_TYPE_CORE	=	1,
    CHECKSTOP_TYPE_NX	=	2,
    CHECKSTOP_TYPE_NPU	=	3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_CoreXstopReason {
    CORE_CHECKSTOP_IFU_REGFILE		= 0x00000001,
    CORE_CHECKSTOP_IFU_LOGIC		= 0x00000002,
    CORE_CHECKSTOP_PC_DURING_RECOV		= 0x00000004,
    CORE_CHECKSTOP_ISU_REGFILE		= 0x00000008,
    CORE_CHECKSTOP_ISU_LOGIC		= 0x00000010,
    CORE_CHECKSTOP_FXU_LOGIC		= 0x00000020,
    CORE_CHECKSTOP_VSU_LOGIC		= 0x00000040,
    CORE_CHECKSTOP_PC_RECOV_IN_MAINT_MODE	= 0x00000080,
    CORE_CHECKSTOP_LSU_REGFILE		= 0x00000100,
    CORE_CHECKSTOP_PC_FWD_PROGRESS		= 0x00000200,
    CORE_CHECKSTOP_LSU_LOGIC		= 0x00000400,
    CORE_CHECKSTOP_PC_LOGIC			= 0x00000800,
    CORE_CHECKSTOP_PC_HYP_RESOURCE		= 0x00001000,
    CORE_CHECKSTOP_PC_HANG_RECOV_FAILED	= 0x00002000,
    CORE_CHECKSTOP_PC_AMBI_HANG_DETECTED	= 0x00004000,
    CORE_CHECKSTOP_PC_DEBUG_TRIG_ERR_INJ	= 0x00008000,
    CORE_CHECKSTOP_PC_SPRD_HYP_ERR_INJ	= 0x00010000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalHMI_NestAccelXstopReason {
    NX_CHECKSTOP_SHM_INVAL_STATE_ERR	= 0x00000001,
    NX_CHECKSTOP_DMA_INVAL_STATE_ERR_1	= 0x00000002,
    NX_CHECKSTOP_DMA_INVAL_STATE_ERR_2	= 0x00000004,
    NX_CHECKSTOP_DMA_CH0_INVAL_STATE_ERR	= 0x00000008,
    NX_CHECKSTOP_DMA_CH1_INVAL_STATE_ERR	= 0x00000010,
    NX_CHECKSTOP_DMA_CH2_INVAL_STATE_ERR	= 0x00000020,
    NX_CHECKSTOP_DMA_CH3_INVAL_STATE_ERR	= 0x00000040,
    NX_CHECKSTOP_DMA_CH4_INVAL_STATE_ERR	= 0x00000080,
    NX_CHECKSTOP_DMA_CH5_INVAL_STATE_ERR	= 0x00000100,
    NX_CHECKSTOP_DMA_CH6_INVAL_STATE_ERR	= 0x00000200,
    NX_CHECKSTOP_DMA_CH7_INVAL_STATE_ERR	= 0x00000400,
    NX_CHECKSTOP_DMA_CRB_UE			= 0x00000800,
    NX_CHECKSTOP_DMA_CRB_SUE		= 0x00001000,
    NX_CHECKSTOP_PBI_ISN_UE			= 0x00002000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalHMIEvent {
    pub /: *mut *mut uint8_t version; / 0x00,
    pub /: *mut *mut uint8_t severity; / 0x01,
    pub /: *mut *mut uint8_t type; / 0x02,
    pub /: *mut *mut uint8_t disposition; / 0x03,
    pub /: *mut *mut uint8_t reserved_1[4]; / 0x04,
    pub hmer: __be64,
// TFMR register. Valid only for TFAC and TFMR_PARITY error type.
    pub tfmr: __be64,
// version 2 and later
//
// checkstop info (Core/NX).
// Valid for OpalHMI_ERROR_MALFUNC_ALERT.
//
    pub /: *mut *mut uint8_t xstop_type; / enum OpalHMI_XstopType,
    pub reserved_1: [u8; 3],
    pub xstop_reason: __be32,
    pub /: *mut *mut __be32 pir; / for CHECKSTOP_TYPE_CORE,
    pub /: *mut *mut __be32 chip_id; / for CHECKSTOP_TYPE_NX,
    pub u: },
    pub xstop_error: },
    pub u: },
}

// OPAL_HANDLE_HMI2 out_flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoP7IOCErrorData {
    pub type: __be16,
// GEM
    pub gemXfir: __be64,
    pub gemRfir: __be64,
    pub gemRirqfir: __be64,
    pub gemMask: __be64,
    pub gemRwof: __be64,
// LEM
    pub lemFir: __be64,
    pub lemErrMask: __be64,
    pub lemAction0: __be64,
    pub lemAction1: __be64,
    pub lemWof: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoP7IOCRgcErrorData {
    pub /: *mut *mut __be64 rgcStatus; / 3E1C10,
    pub /: *mut *mut __be64 rgcLdcp; / 3E1C18,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoP7IOCBiErrorData {
    pub /: *mut *mut __be64 biLdcp0; / 3C0100, 3C0118,
    pub /: *mut *mut __be64 biLdcp1; / 3C0108, 3C0120,
    pub /: *mut *mut __be64 biLdcp2; / 3C0110, 3C0128,
    pub /: *mut *mut __be64 biFenceStatus; / 3C0130, 3C0130,
    pub /: *mut *mut uint8_t biDownbound; / BI Downbound or Upbound,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoP7IOCCiErrorData {
    pub /: *mut *mut __be64 ciPortStatus; / 3Dn008,
    pub /: *mut *mut __be64 ciPortLdcp; / 3Dn010,
    pub /: *mut *mut uint8_t ciPort; / Index of CI port: 0/1,
}

//
// This structure defines the overlay which will be used to store PHB error
// data upon request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoPhbErrorCommon {
    pub version: __be32,
    pub ioType: __be32,
    pub len: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoP7IOCPhbErrorData {
    pub common: OpalIoPhbErrorCommon,
    pub brdgCtl: __be32,
// P7IOC utl regs
    pub portStatusReg: __be32,
    pub rootCmplxStatus: __be32,
    pub busAgentStatus: __be32,
// P7IOC cfg regs
    pub deviceStatus: __be32,
    pub slotStatus: __be32,
    pub linkStatus: __be32,
    pub devCmdStatus: __be32,
    pub devSecStatus: __be32,
// cfg AER regs
    pub rootErrorStatus: __be32,
    pub uncorrErrorStatus: __be32,
    pub corrErrorStatus: __be32,
    pub tlpHdr1: __be32,
    pub tlpHdr2: __be32,
    pub tlpHdr3: __be32,
    pub tlpHdr4: __be32,
    pub sourceId: __be32,
    pub rsv3: __be32,
// Record data about the call to allocate a buffer.
    pub errorClass: __be64,
    pub correlator: __be64,
// P7IOC MMIO Error Regs
    pub n120: __be64 p7iocPlssr; //,
    pub n110: __be64 p7iocCsr; //,
    pub nC00: __be64 lemFir; //,
    pub nC18: __be64 lemErrorMask; //,
    pub nC40: __be64 lemWOF; //,
    pub nC80: __be64 phbErrorStatus; //,
    pub nC88: __be64 phbFirstErrorStatus; //,
    pub nCC0: __be64 phbErrorLog0; //,
    pub nCC8: __be64 phbErrorLog1; //,
    pub nD00: __be64 mmioErrorStatus; //,
    pub nD08: __be64 mmioFirstErrorStatus; //,
    pub nD40: __be64 mmioErrorLog0; //,
    pub nD48: __be64 mmioErrorLog1; //,
    pub nD80: __be64 dma0ErrorStatus; //,
    pub nD88: __be64 dma0FirstErrorStatus; //,
    pub nDC0: __be64 dma0ErrorLog0; //,
    pub nDC8: __be64 dma0ErrorLog1; //,
    pub nE00: __be64 dma1ErrorStatus; //,
    pub nE08: __be64 dma1FirstErrorStatus; //,
    pub nE40: __be64 dma1ErrorLog0; //,
    pub nE48: __be64 dma1ErrorLog1; //,
    pub pestA: [__be64; OPAL_P7IOC_NUM_PEST_REGS],
    pub pestB: [__be64; OPAL_P7IOC_NUM_PEST_REGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoPhb3ErrorData {
    pub common: OpalIoPhbErrorCommon,
    pub brdgCtl: __be32,
// PHB3 UTL regs
    pub portStatusReg: __be32,
    pub rootCmplxStatus: __be32,
    pub busAgentStatus: __be32,
// PHB3 cfg regs
    pub deviceStatus: __be32,
    pub slotStatus: __be32,
    pub linkStatus: __be32,
    pub devCmdStatus: __be32,
    pub devSecStatus: __be32,
// cfg AER regs
    pub rootErrorStatus: __be32,
    pub uncorrErrorStatus: __be32,
    pub corrErrorStatus: __be32,
    pub tlpHdr1: __be32,
    pub tlpHdr2: __be32,
    pub tlpHdr3: __be32,
    pub tlpHdr4: __be32,
    pub sourceId: __be32,
    pub rsv3: __be32,
// Record data about the call to allocate a buffer
    pub errorClass: __be64,
    pub correlator: __be64,
// PHB3 MMIO Error Regs
    pub /: *mut *mut __be64 nFir; / 000,
    pub /: *mut *mut __be64 nFirMask; / 003,
    pub /: *mut *mut __be64 nFirWOF; / 008,
    pub /: *mut *mut __be64 phbPlssr; / 120,
    pub /: *mut *mut __be64 phbCsr; / 110,
    pub /: *mut *mut __be64 lemFir; / C00,
    pub /: *mut *mut __be64 lemErrorMask; / C18,
    pub /: *mut *mut __be64 lemWOF; / C40,
    pub /: *mut *mut __be64 phbErrorStatus; / C80,
    pub /: *mut *mut __be64 phbFirstErrorStatus; / C88,
    pub /: *mut *mut __be64 phbErrorLog0; / CC0,
    pub /: *mut *mut __be64 phbErrorLog1; / CC8,
    pub /: *mut *mut __be64 mmioErrorStatus; / D00,
    pub /: *mut *mut __be64 mmioFirstErrorStatus; / D08,
    pub /: *mut *mut __be64 mmioErrorLog0; / D40,
    pub /: *mut *mut __be64 mmioErrorLog1; / D48,
    pub /: *mut *mut __be64 dma0ErrorStatus; / D80,
    pub /: *mut *mut __be64 dma0FirstErrorStatus; / D88,
    pub /: *mut *mut __be64 dma0ErrorLog0; / DC0,
    pub /: *mut *mut __be64 dma0ErrorLog1; / DC8,
    pub /: *mut *mut __be64 dma1ErrorStatus; / E00,
    pub /: *mut *mut __be64 dma1FirstErrorStatus; / E08,
    pub /: *mut *mut __be64 dma1ErrorLog0; / E40,
    pub /: *mut *mut __be64 dma1ErrorLog1; / E48,
    pub pestA: [__be64; OPAL_PHB3_NUM_PEST_REGS],
    pub pestB: [__be64; OPAL_PHB3_NUM_PEST_REGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalIoPhb4ErrorData {
    pub common: OpalIoPhbErrorCommon,
    pub brdgCtl: __be32,
// PHB4 cfg regs
    pub deviceStatus: __be32,
    pub slotStatus: __be32,
    pub linkStatus: __be32,
    pub devCmdStatus: __be32,
    pub devSecStatus: __be32,
// cfg AER regs
    pub rootErrorStatus: __be32,
    pub uncorrErrorStatus: __be32,
    pub corrErrorStatus: __be32,
    pub tlpHdr1: __be32,
    pub tlpHdr2: __be32,
    pub tlpHdr3: __be32,
    pub tlpHdr4: __be32,
    pub sourceId: __be32,
// PHB4 ETU Error Regs
    pub /: *mut *mut __be64 nFir; / 000,
    pub /: *mut *mut __be64 nFirMask; / 003,
    pub /: *mut *mut __be64 nFirWOF; / 008,
    pub /: *mut *mut __be64 phbPlssr; / 120,
    pub /: *mut *mut __be64 phbCsr; / 110,
    pub /: *mut *mut __be64 lemFir; / C00,
    pub /: *mut *mut __be64 lemErrorMask; / C18,
    pub /: *mut *mut __be64 lemWOF; / C40,
    pub /: *mut *mut __be64 phbErrorStatus; / C80,
    pub /: *mut *mut __be64 phbFirstErrorStatus; / C88,
    pub /: *mut *mut __be64 phbErrorLog0; / CC0,
    pub /: *mut *mut __be64 phbErrorLog1; / CC8,
    pub /: *mut *mut __be64 phbTxeErrorStatus; / D00,
    pub /: *mut *mut __be64 phbTxeFirstErrorStatus; / D08,
    pub /: *mut *mut __be64 phbTxeErrorLog0; / D40,
    pub /: *mut *mut __be64 phbTxeErrorLog1; / D48,
    pub /: *mut *mut __be64 phbRxeArbErrorStatus; / D80,
    pub /: *mut *mut __be64 phbRxeArbFirstErrorStatus; / D88,
    pub /: *mut *mut __be64 phbRxeArbErrorLog0; / DC0,
    pub /: *mut *mut __be64 phbRxeArbErrorLog1; / DC8,
    pub /: *mut *mut __be64 phbRxeMrgErrorStatus; / E00,
    pub /: *mut *mut __be64 phbRxeMrgFirstErrorStatus; / E08,
    pub /: *mut *mut __be64 phbRxeMrgErrorLog0; / E40,
    pub /: *mut *mut __be64 phbRxeMrgErrorLog1; / E48,
    pub /: *mut *mut __be64 phbRxeTceErrorStatus; / E80,
    pub /: *mut *mut __be64 phbRxeTceFirstErrorStatus; / E88,
    pub /: *mut *mut __be64 phbRxeTceErrorLog0; / EC0,
    pub /: *mut *mut __be64 phbRxeTceErrorLog1; / EC8,
// PHB4 REGB Error Regs
    pub /: *mut *mut __be64 phbPblErrorStatus; / 1900,
    pub /: *mut *mut __be64 phbPblFirstErrorStatus; / 1908,
    pub /: *mut *mut __be64 phbPblErrorLog0; / 1940,
    pub /: *mut *mut __be64 phbPblErrorLog1; / 1948,
    pub /: *mut *mut __be64 phbPcieDlpErrorLog1; / 1AA0,
    pub /: *mut *mut __be64 phbPcieDlpErrorLog2; / 1AA8,
    pub /: *mut *mut __be64 phbPcieDlpErrorStatus; / 1AB0,
    pub /: *mut *mut __be64 phbRegbErrorStatus; / 1C00,
    pub /: *mut *mut __be64 phbRegbFirstErrorStatus; / 1C08,
    pub /: *mut *mut __be64 phbRegbErrorLog0; / 1C40,
    pub /: *mut *mut __be64 phbRegbErrorLog1; / 1C48,
    pub pestA: [__be64; OPAL_PHB4_NUM_PEST_REGS],
    pub pestB: [__be64; OPAL_PHB4_NUM_PEST_REGS],
}

// These two define the base MMU mode of the host on P9
//
// On P9 Nimbus DD2.0 and Cumlus (and later), KVM can still
// create hash guests in "radix" mode with care (full core
// switch only).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_prd_msg_type {
    OPAL_PRD_MSG_TYPE_INIT = 0,	/* HBRT --> OPAL */
    OPAL_PRD_MSG_TYPE_FINI,		/* HBRT/kernel --> OPAL */
    OPAL_PRD_MSG_TYPE_ATTN,		/* HBRT <-- OPAL */
    OPAL_PRD_MSG_TYPE_ATTN_ACK,	/* HBRT --> OPAL */
    OPAL_PRD_MSG_TYPE_OCC_ERROR,	/* HBRT <-- OPAL */
    OPAL_PRD_MSG_TYPE_OCC_RESET,	/* HBRT <-- OPAL */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_prd_msg_header {
    pub type: u8,
    pub pad: [u8; 1],
    pub size: __be16,
}

pub const OCC_RESET: c_int = 0;
pub const OCC_LOAD: c_int = 1;
pub const OCC_THROTTLE: c_int = 2;
pub const OCC_MAX_THROTTLE_STATUS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_occ_msg {
    pub type: __be64,
    pub chip: __be64,
    pub throttle_status: __be64,
}

//
// SG entries
//
// WARNING: The current implementation requires each entry
// to represent a block that is 4k aligned *and* each block
// size except the last one in the list to be as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_sg_entry {
    pub data: __be64,
    pub length: __be64,
}

//
// Candidate image SG list.
//
// length = VER | length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_sg_list {
    pub length: __be64,
    pub next: __be64,
    pub entry: [opal_sg_entry; ],
}

//
// Dump region ID range usable by the OS
//
pub const OPAL_DUMP_REGION_HOST_START: c_uint = 0x80;
pub const OPAL_DUMP_REGION_LOG_BUF: c_uint = 0x80;
pub const OPAL_DUMP_REGION_HOST_END: c_uint = 0xFF;
// CAPI modes for PHB
// OPAL I2C request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_i2c_request {
    pub type: u8,
pub const OPAL_I2C_RAW_READ: c_int = 0;
pub const OPAL_I2C_RAW_WRITE: c_int = 1;
pub const OPAL_I2C_SM_READ: c_int = 2;
pub const OPAL_I2C_SM_WRITE: c_int = 3;
    pub flags: u8,
pub const OPAL_I2C_ADDR_10: c_uint = 0x01	/* Not supported yet */;
    pub /: *mut *mut uint8_t subaddr_sz; / Max 4,
    pub reserved: u8,
    pub /: *mut *mut __be16 addr; / 7 or 10 bit address,
    pub reserved2: __be16,
    pub /: *mut *mut __be32 subaddr; / Sub-address if any,
    pub /: *mut *mut __be32 size; / Data size,
    pub /: *mut *mut __be64 buffer_ra; / Buffer real address,
}

//
// EPOW status sharing (OPAL and the host)
//
// The host will pass on OPAL, a buffer of length OPAL_SYSEPOW_MAX
// with individual elements being 16 bits wide to fetch the system
// wide EPOW status. Each element in the buffer will contain the
// EPOW status in its bit representation for a particular EPOW sub
// class as defined here. So multiple detailed EPOW status bits
// specific for any sub class can be represented in a single buffer
// element as its bit representation.
//
// System EPOW type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSysEpow {
    OPAL_SYSEPOW_POWER	= 0,	/* Power EPOW */
    OPAL_SYSEPOW_TEMP	= 1,	/* Temperature EPOW */
    OPAL_SYSEPOW_COOLING	= 2,	/* Cooling EPOW */
    OPAL_SYSEPOW_MAX	= 3,	/* Max EPOW categories */
}

// Power EPOW
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSysPower {
    OPAL_SYSPOWER_UPS	= 0x0001, /* System on UPS power */
    OPAL_SYSPOWER_CHNG	= 0x0002, /* System power config change */
    OPAL_SYSPOWER_FAIL	= 0x0004, /* System impending power failure */
    OPAL_SYSPOWER_INCL	= 0x0008, /* System incomplete power */
}

// Temperature EPOW
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSysTemp {
    OPAL_SYSTEMP_AMB	= 0x0001, /* System over ambient temperature */
    OPAL_SYSTEMP_INT	= 0x0002, /* System over internal temperature */
    OPAL_SYSTEMP_HMD	= 0x0004, /* System over ambient humidity */
}

// Cooling EPOW
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpalSysCooling {
    OPAL_SYSCOOL_INSF	= 0x0001, /* System insufficient cooling */
}

// Argument to OPAL_CEC_REBOOT2()
// Argument to OPAL_PCI_TCE_KILL
// The xive operation mode indicates the active "API" and
// corresponds to the "mode" parameter of the opal_xive_reset()
// call
//
// Flags for OPAL_XIVE_GET_IRQ_INFO
// Flags for OPAL_XIVE_GET/SET_QUEUE_INFO
// Flags for OPAL_XIVE_GET/SET_VP_INFO
// "Any chip" replacement for chip ID for allocation functions
// Xive sync options
// This bits are cumulative, arg is a girq
// Dump options
// "type" argument options for OPAL_IMC_COUNTERS_* calls
// PCI p2p descriptor
pub const OPAL_PCI_P2P_ENABLE: c_uint = 0x1;
pub const OPAL_PCI_P2P_LOAD: c_uint = 0x2;
pub const OPAL_PCI_P2P_STORE: c_uint = 0x4;
// MPIPL update operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_mpipl_ops {
    OPAL_MPIPL_ADD_RANGE			= 0,
    OPAL_MPIPL_REMOVE_RANGE			= 1,
    OPAL_MPIPL_REMOVE_ALL			= 2,
    OPAL_MPIPL_FREE_PRESERVED_MEMORY	= 3,
}

// Tag will point to various metadata area. Kernel will
// use tag to get metadata value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_mpipl_tags {
    OPAL_MPIPL_TAG_CPU	= 0,
    OPAL_MPIPL_TAG_OPAL	= 1,
    OPAL_MPIPL_TAG_KERNEL	= 2,
    OPAL_MPIPL_TAG_BOOT_MEM	= 3,
}

// Preserved memory details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_mpipl_region {
    pub src: __be64,
    pub dest: __be64,
    pub size: __be64,
}

// Structure version
pub const OPAL_MPIPL_VERSION: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_mpipl_fadump {
    pub version: u8,
    pub reserved: [u8; 7],
    pub /: *mut *mut __be32 crashing_pir; / OPAL crashing CPU PIR,
    pub cpu_data_version: __be32,
    pub cpu_data_size: __be32,
    pub region_cnt: __be32,
    pub region: [opal_mpipl_region; ],
    pub __packed: },

