//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/rtas.h
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
// Definitions for talking to the RTAS on CHRP machines.
//
// Copyright (C) 2001 Peter Bergner
// Copyright (C) 2001 PPC 64 Team, IBM Corp
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtas_function_index {
    RTAS_FNIDX__CHECK_EXCEPTION,
    RTAS_FNIDX__DISPLAY_CHARACTER,
    RTAS_FNIDX__EVENT_SCAN,
    RTAS_FNIDX__FREEZE_TIME_BASE,
    RTAS_FNIDX__GET_POWER_LEVEL,
    RTAS_FNIDX__GET_SENSOR_STATE,
    RTAS_FNIDX__GET_TERM_CHAR,
    RTAS_FNIDX__GET_TIME_OF_DAY,
    RTAS_FNIDX__IBM_ACTIVATE_FIRMWARE,
    RTAS_FNIDX__IBM_CBE_START_PTCAL,
    RTAS_FNIDX__IBM_CBE_STOP_PTCAL,
    RTAS_FNIDX__IBM_CHANGE_MSI,
    RTAS_FNIDX__IBM_CLOSE_ERRINJCT,
    RTAS_FNIDX__IBM_CONFIGURE_BRIDGE,
    RTAS_FNIDX__IBM_CONFIGURE_CONNECTOR,
    RTAS_FNIDX__IBM_CONFIGURE_KERNEL_DUMP,
    RTAS_FNIDX__IBM_CONFIGURE_PE,
    RTAS_FNIDX__IBM_CREATE_PE_DMA_WINDOW,
    RTAS_FNIDX__IBM_DISPLAY_MESSAGE,
    RTAS_FNIDX__IBM_ERRINJCT,
    RTAS_FNIDX__IBM_EXTI2C,
    RTAS_FNIDX__IBM_GET_CONFIG_ADDR_INFO,
    RTAS_FNIDX__IBM_GET_CONFIG_ADDR_INFO2,
    RTAS_FNIDX__IBM_GET_DYNAMIC_SENSOR_STATE,
    RTAS_FNIDX__IBM_GET_INDICES,
    RTAS_FNIDX__IBM_GET_RIO_TOPOLOGY,
    RTAS_FNIDX__IBM_GET_SYSTEM_PARAMETER,
    RTAS_FNIDX__IBM_GET_VPD,
    RTAS_FNIDX__IBM_GET_XIVE,
    RTAS_FNIDX__IBM_INT_OFF,
    RTAS_FNIDX__IBM_INT_ON,
    RTAS_FNIDX__IBM_IO_QUIESCE_ACK,
    RTAS_FNIDX__IBM_LPAR_PERFTOOLS,
    RTAS_FNIDX__IBM_MANAGE_FLASH_IMAGE,
    RTAS_FNIDX__IBM_MANAGE_STORAGE_PRESERVATION,
    RTAS_FNIDX__IBM_NMI_INTERLOCK,
    RTAS_FNIDX__IBM_NMI_REGISTER,
    RTAS_FNIDX__IBM_OPEN_ERRINJCT,
    RTAS_FNIDX__IBM_OPEN_SRIOV_ALLOW_UNFREEZE,
    RTAS_FNIDX__IBM_OPEN_SRIOV_MAP_PE_NUMBER,
    RTAS_FNIDX__IBM_OS_TERM,
    RTAS_FNIDX__IBM_PARTNER_CONTROL,
    RTAS_FNIDX__IBM_PHYSICAL_ATTESTATION,
    RTAS_FNIDX__IBM_PLATFORM_DUMP,
    RTAS_FNIDX__IBM_POWER_OFF_UPS,
    RTAS_FNIDX__IBM_QUERY_INTERRUPT_SOURCE_NUMBER,
    RTAS_FNIDX__IBM_QUERY_PE_DMA_WINDOW,
    RTAS_FNIDX__IBM_READ_PCI_CONFIG,
    RTAS_FNIDX__IBM_READ_SLOT_RESET_STATE,
    RTAS_FNIDX__IBM_READ_SLOT_RESET_STATE2,
    RTAS_FNIDX__IBM_RECEIVE_HVPIPE_MSG,
    RTAS_FNIDX__IBM_REMOVE_PE_DMA_WINDOW,
    RTAS_FNIDX__IBM_RESET_PE_DMA_WINDOW,
    RTAS_FNIDX__IBM_SCAN_LOG_DUMP,
    RTAS_FNIDX__IBM_SEND_HVPIPE_MSG,
    RTAS_FNIDX__IBM_SET_DYNAMIC_INDICATOR,
    RTAS_FNIDX__IBM_SET_EEH_OPTION,
    RTAS_FNIDX__IBM_SET_SLOT_RESET,
    RTAS_FNIDX__IBM_SET_SYSTEM_PARAMETER,
    RTAS_FNIDX__IBM_SET_XIVE,
    RTAS_FNIDX__IBM_SLOT_ERROR_DETAIL,
    RTAS_FNIDX__IBM_SUSPEND_ME,
    RTAS_FNIDX__IBM_TUNE_DMA_PARMS,
    RTAS_FNIDX__IBM_UPDATE_FLASH_64_AND_REBOOT,
    RTAS_FNIDX__IBM_UPDATE_NODES,
    RTAS_FNIDX__IBM_UPDATE_PROPERTIES,
    RTAS_FNIDX__IBM_VALIDATE_FLASH_IMAGE,
    RTAS_FNIDX__IBM_WRITE_PCI_CONFIG,
    RTAS_FNIDX__NVRAM_FETCH,
    RTAS_FNIDX__NVRAM_STORE,
    RTAS_FNIDX__POWER_OFF,
    RTAS_FNIDX__PUT_TERM_CHAR,
    RTAS_FNIDX__QUERY_CPU_STOPPED_STATE,
    RTAS_FNIDX__READ_PCI_CONFIG,
    RTAS_FNIDX__RTAS_LAST_ERROR,
    RTAS_FNIDX__SET_INDICATOR,
    RTAS_FNIDX__SET_POWER_LEVEL,
    RTAS_FNIDX__SET_TIME_FOR_POWER_ON,
    RTAS_FNIDX__SET_TIME_OF_DAY,
    RTAS_FNIDX__START_CPU,
    RTAS_FNIDX__STOP_SELF,
    RTAS_FNIDX__SYSTEM_REBOOT,
    RTAS_FNIDX__THAW_TIME_BASE,
    RTAS_FNIDX__WRITE_PCI_CONFIG,
}

//
// Opaque handle for client code to refer to RTAS functions. All valid
// function handles are build-time constants prefixed with RTAS_FN_.
//

// Memory set aside for sys_rtas to use with calls that need a work area.

//
// Common RTAS function return values, derived from the table "RTAS
// Status Word Values" in PAPR+ v2.13 7.2.8: "Return Codes". If a
// function can return a value in this table then generally it has the
// meaning listed here. More extended commentary in the documentation
// for rtas_call().
//
// RTAS functions may use negative and positive numbers not in this
// set for function-specific error and success conditions,
// respectively.
//

// statuses specific to ibm,suspend-me

// RTAS event classes
pub const RTAS_INTERNAL_ERROR: c_uint = 0x80000000 /* set bit 0 */;
pub const RTAS_EPOW_WARNING: c_uint = 0x40000000 /* set bit 1 */;
pub const RTAS_HOTPLUG_EVENTS: c_uint = 0x10000000 /* set bit 3 */;
pub const RTAS_IO_EVENTS: c_uint = 0x08000000 /* set bit 4 */;
pub const RTAS_HVPIPE_MSG_EVENTS: c_uint = 0x04000000 /* set bit 5 */;
pub const RTAS_EVENT_SCAN_ALL_EVENTS: c_uint = 0xffffffff;
// RTAS event severity
pub const RTAS_SEVERITY_FATAL: c_uint = 0x5;
pub const RTAS_SEVERITY_ERROR: c_uint = 0x4;
pub const RTAS_SEVERITY_ERROR_SYNC: c_uint = 0x3;
pub const RTAS_SEVERITY_WARNING: c_uint = 0x2;
pub const RTAS_SEVERITY_EVENT: c_uint = 0x1;
pub const RTAS_SEVERITY_NO_ERROR: c_uint = 0x0;
// RTAS event disposition
pub const RTAS_DISP_FULLY_RECOVERED: c_uint = 0x0;
pub const RTAS_DISP_LIMITED_RECOVERY: c_uint = 0x1;
pub const RTAS_DISP_NOT_RECOVERED: c_uint = 0x2;
// RTAS event initiator
pub const RTAS_INITIATOR_UNKNOWN: c_uint = 0x0;
pub const RTAS_INITIATOR_CPU: c_uint = 0x1;
pub const RTAS_INITIATOR_PCI: c_uint = 0x2;
pub const RTAS_INITIATOR_ISA: c_uint = 0x3;
pub const RTAS_INITIATOR_MEMORY: c_uint = 0x4;
pub const RTAS_INITIATOR_POWERMGM: c_uint = 0x5;
// RTAS event target
pub const RTAS_TARGET_UNKNOWN: c_uint = 0x0;
pub const RTAS_TARGET_CPU: c_uint = 0x1;
pub const RTAS_TARGET_PCI: c_uint = 0x2;
pub const RTAS_TARGET_ISA: c_uint = 0x3;
pub const RTAS_TARGET_MEMORY: c_uint = 0x4;
pub const RTAS_TARGET_POWERMGM: c_uint = 0x5;
// RTAS event type
pub const RTAS_TYPE_RETRY: c_uint = 0x01;
pub const RTAS_TYPE_TCE_ERR: c_uint = 0x02;
pub const RTAS_TYPE_INTERN_DEV_FAIL: c_uint = 0x03;
pub const RTAS_TYPE_TIMEOUT: c_uint = 0x04;
pub const RTAS_TYPE_DATA_PARITY: c_uint = 0x05;
pub const RTAS_TYPE_ADDR_PARITY: c_uint = 0x06;
pub const RTAS_TYPE_CACHE_PARITY: c_uint = 0x07;
pub const RTAS_TYPE_ADDR_INVALID: c_uint = 0x08;
pub const RTAS_TYPE_ECC_UNCORR: c_uint = 0x09;
pub const RTAS_TYPE_ECC_CORR: c_uint = 0x0a;
pub const RTAS_TYPE_EPOW: c_uint = 0x40;
pub const RTAS_TYPE_PLATFORM: c_uint = 0xE0;
pub const RTAS_TYPE_IO: c_uint = 0xE1;
pub const RTAS_TYPE_INFO: c_uint = 0xE2;
pub const RTAS_TYPE_DEALLOC: c_uint = 0xE3;
pub const RTAS_TYPE_DUMP: c_uint = 0xE4;
pub const RTAS_TYPE_HOTPLUG: c_uint = 0xE5;
pub const RTAS_TYPE_HVPIPE: c_uint = 0xE6;
// I don't add PowerMGM events right now, this is a different topic
pub const RTAS_TYPE_PMGM_POWER_SW_ON: c_uint = 0x60;
pub const RTAS_TYPE_PMGM_POWER_SW_OFF: c_uint = 0x61;
pub const RTAS_TYPE_PMGM_LID_OPEN: c_uint = 0x62;
pub const RTAS_TYPE_PMGM_LID_CLOSE: c_uint = 0x63;
pub const RTAS_TYPE_PMGM_SLEEP_BTN: c_uint = 0x64;
pub const RTAS_TYPE_PMGM_WAKE_BTN: c_uint = 0x65;
pub const RTAS_TYPE_PMGM_BATTERY_WARN: c_uint = 0x66;
pub const RTAS_TYPE_PMGM_BATTERY_CRIT: c_uint = 0x67;
pub const RTAS_TYPE_PMGM_SWITCH_TO_BAT: c_uint = 0x68;
pub const RTAS_TYPE_PMGM_SWITCH_TO_AC: c_uint = 0x69;
pub const RTAS_TYPE_PMGM_KBD_OR_MOUSE: c_uint = 0x6a;
pub const RTAS_TYPE_PMGM_ENCLOS_OPEN: c_uint = 0x6b;
pub const RTAS_TYPE_PMGM_ENCLOS_CLOSED: c_uint = 0x6c;
pub const RTAS_TYPE_PMGM_RING_INDICATE: c_uint = 0x6d;
pub const RTAS_TYPE_PMGM_LAN_ATTENTION: c_uint = 0x6e;
pub const RTAS_TYPE_PMGM_TIME_ALARM: c_uint = 0x6f;
pub const RTAS_TYPE_PMGM_CONFIG_CHANGE: c_uint = 0x70;
pub const RTAS_TYPE_PMGM_SERVICE_PROC: c_uint = 0x71;
// Platform Resource Reassignment Notification
pub const RTAS_TYPE_PRRN: c_uint = 0xA0;
// RTAS check-exception vector offset
pub const RTAS_VECTOR_EXTERNAL_INTERRUPT: c_uint = 0x500;

extern "C" {
    pub fn be32_to_cpu(_arg: elog->extended_log_length) -> return;
}
pub const RTAS_V6EXT_LOG_FORMAT_EVENT_LOG: c_int = 14;

extern "C" {
    pub fn be32_to_cpu(_arg: ext_log->company_id) -> return;
}
// pSeries event log format
// Two bytes ASCII section IDs

extern "C" {
    pub fn be16_to_cpu(_arg: sect->id) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: sect->length) -> return;
}
pub const PSERIES_HP_ELOG_RESOURCE_CPU: c_int = 1;
pub const PSERIES_HP_ELOG_RESOURCE_MEM: c_int = 2;
pub const PSERIES_HP_ELOG_RESOURCE_SLOT: c_int = 3;
pub const PSERIES_HP_ELOG_RESOURCE_PHB: c_int = 4;
pub const PSERIES_HP_ELOG_RESOURCE_PMEM: c_int = 6;
pub const PSERIES_HP_ELOG_RESOURCE_DT: c_int = 7;
pub const PSERIES_HP_ELOG_ACTION_ADD: c_int = 1;
pub const PSERIES_HP_ELOG_ACTION_REMOVE: c_int = 2;
pub const PSERIES_HP_ELOG_ID_DRC_NAME: c_int = 1;
pub const PSERIES_HP_ELOG_ID_DRC_INDEX: c_int = 2;
pub const PSERIES_HP_ELOG_ID_DRC_COUNT: c_int = 3;
pub const PSERIES_HP_ELOG_ID_DRC_IC: c_int = 4;
//
// This can be set by the rtas_flash module so that it can get called
// as the absolutely last thing before the kernel terminates.
//
extern "C" {
    pub fn void(_arg: *mut rtas_flash_term_hook)(int) -> extern;
}
extern "C" {
    pub fn rtas_function_token(handle: rtas_fn_handle_t) -> i32;
}
extern "C" {
    pub fn rtas_token(service: *const c_char) -> c_int;
}
extern "C" {
    pub fn rtas_call(token: c_int, nargs: c_int, nret: c_int, outputs: *mut c_int, ...) -> c_int;
}
extern "C" {
    pub fn rtas_restart(cmd: *mut c_char) -> void __noreturn;
}
extern "C" {
    pub fn rtas_power_off();
}
extern "C" {
    pub fn rtas_halt() -> void __noreturn;
}
extern "C" {
    pub fn rtas_os_term(str: *mut c_char);
}
extern "C" {
    pub fn rtas_activate_firmware();
}
extern "C" {
    pub fn rtas_get_sensor(sensor: c_int, index: c_int, state: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtas_get_sensor_fast(sensor: c_int, index: c_int, state: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtas_get_power_level(powerdomain: c_int, level: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtas_set_power_level(powerdomain: c_int, level: c_int, setlevel: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtas_indicator_present(token: c_int, maxindex: *mut c_int) -> bool;
}
extern "C" {
    pub fn rtas_set_indicator(indicator: c_int, index: c_int, new_value: c_int) -> c_int;
}
extern "C" {
    pub fn rtas_set_indicator_fast(indicator: c_int, index: c_int, new_value: c_int) -> c_int;
}
extern "C" {
    pub fn rtas_progress(s: *mut c_char, hex: c_ushort);
}
extern "C" {
    pub fn rtas_ibm_suspend_me(fw_status: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtas_error_rc(rtas_rc: c_int) -> c_int;
}
extern "C" {
    pub fn rtas_get_boot_time() -> time64_t;
}
extern "C" {
    pub fn rtas_get_rtc_time(rtc_time: *mut rtc_time);
}
extern "C" {
    pub fn rtas_set_rtc_time(rtc_time: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn rtas_busy_delay_time(status: c_int) -> c_uint;
}
extern "C" {
    pub fn rtas_busy_delay(status: c_int) -> bool;
}
extern "C" {
    pub fn early_init_dt_scan_rtas(node: c_ulong, uname: *const c_char, depth: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pSeries_log_error(buf: *mut c_char, err_type: c_uint, fatal: c_int);
}

extern "C" {
    pub fn clobbering_unread_rtas_event() -> c_int;
}
extern "C" {
    pub fn rtas_syscall_dispatch_ibm_suspend_me(handle: u64) -> c_int;
}

extern "C" {
    pub fn rtas_cancel_event_scan();
}

// Error types logged.
pub const ERR_FLAG_ALREADY_LOGGED: c_uint = 0x0;
pub const ERR_FLAG_BOOT: c_uint = 0x1	/* log was pulled from NVRAM on boot */;
pub const ERR_TYPE_RTAS_LOG: c_uint = 0x2	/* from rtas event-scan */;
pub const ERR_TYPE_KERNEL_PANIC: c_uint = 0x4	/* from die()/panic() */;
pub const ERR_TYPE_KERNEL_PANIC_GZ: c_uint = 0x8	/* ditto, compressed */;
// All the types and not flags

pub const RTAS_ERROR_LOG_MAX: c_int = 2048;
//
// Return the firmware-specified size of the error log buffer
// for all rtas calls that require an error buffer argument.
// This includes 'check-exception' and 'rtas-last-error'.
//
extern "C" {
    pub fn rtas_get_error_log_max() -> c_int;
}
// Event Scan Parameters
pub const EVENT_SCAN_ALL_EVENTS: c_uint = 0xf0000000;
pub const SURVEILLANCE_TOKEN: c_int = 9000;

// Some RTAS ops require a data buffer and that buffer must be < 4G.
// Rather than having a memory allocator, just use this buffer
// (get the lock first), make the RTAS call.  Copy the data instead
// of holding the buffer for long.
//
pub const RTAS_DATA_BUF_SIZE: c_int = 4096;
// RMO buffer reserved for user-space RTAS use
pub const GLOBAL_INTERRUPT_QUEUE: c_int = 9005;
//
// rtas_config_addr - Format a busno, devfn and reg for RTAS.
// @busno: The bus number.
// @devfn: The device and function number as encoded by PCI_DEVFN().
// @reg: The register number.
//
// This function encodes the given busno, devfn and register number as
// required for RTAS calls that take a "config_addr" parameter.
// See PAPR requirement 7.3.4-1 for more info.
//
extern "C" {
    pub fn rtas_give_timebase();
}
extern "C" {
    pub fn rtas_take_timebase();
}

// Not the best place to put pSeries_coalesce_init, will be fixed when we
// move some of the rtas suspend-me stuff to pseries
extern "C" {
    pub fn pSeries_coalesce_init();
}
extern "C" {
    pub fn rtas_initialize();
}

extern "C" {
    pub fn read_24x7_sys_info();
}

