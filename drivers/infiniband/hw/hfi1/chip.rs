//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/chip.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2020 Intel Corporation.
//
// This file contains all of the defines that is specific to the HFI chip
//
// sizes

pub const NUM_INTERRUPT_SOURCES: c_int = 768;
pub const RXE_NUM_CONTEXTS: c_int = 160;
pub const RXE_PER_CONTEXT_SIZE: c_uint = 0x1000	/* 4k */;
pub const RXE_NUM_TID_FLOWS: c_int = 32;
pub const RXE_NUM_DATA_VL: c_int = 8;
pub const TXE_NUM_CONTEXTS: c_int = 160;
pub const TXE_NUM_SDMA_ENGINES: c_int = 16;
pub const NUM_CONTEXTS_PER_SET: c_int = 8;
pub const VL_ARB_HIGH_PRIO_TABLE_SIZE: c_int = 16;
pub const VL_ARB_LOW_PRIO_TABLE_SIZE: c_int = 16;
pub const VL_ARB_TABLE_SIZE: c_int = 16;
pub const TXE_NUM_32_BIT_COUNTER: c_int = 7;
pub const TXE_NUM_64_BIT_COUNTER: c_int = 30;
pub const TXE_NUM_DATA_VL: c_int = 8;

pub const PIO_CMASK: c_uint = 0x7ff	/* counter mask for free and fill counters */;

//
// Virtual? Allocation Unit, defined as AU = 8*2^vAU, 64 bytes, AU is fixed
// at 64 bytes for all generation one devices
//
pub const CM_VAU: c_int = 3;
// HFI link credit count, AKA receive buffer depth (RBUF_DEPTH)
pub const CM_GLOBAL_CREDITS: c_uint = 0x880;
// Number of PKey entries in the HW
pub const MAX_PKEY_VALUES: c_int = 16;

// PBC flags

// PbcInsertHcrc field settings
pub const PBC_IHCRC_LKDETH: c_uint = 0x0	/* insert @ local KDETH offset */;
pub const PBC_IHCRC_GKDETH: c_uint = 0x1	/* insert @ global KDETH offset */;
pub const PBC_IHCRC_NONE: c_uint = 0x2	/* no HCRC inserted */;
// PBC fields
pub const PBC_STATIC_RATE_CONTROL_COUNT_SHIFT: c_int = 32;
pub const PBC_STATIC_RATE_CONTROL_COUNT_MASK: c_uint = 0xffffull;

pub const PBC_INSERT_HCRC_SHIFT: c_int = 26;
pub const PBC_INSERT_HCRC_MASK: c_uint = 0x3ull;

pub const PBC_VL_SHIFT: c_int = 12;
pub const PBC_VL_MASK: c_uint = 0xfull;

pub const PBC_LENGTH_DWS_SHIFT: c_int = 0;
pub const PBC_LENGTH_DWS_MASK: c_uint = 0xfffull;

// Credit Return Fields
pub const CR_COUNTER_SHIFT: c_int = 0;
pub const CR_COUNTER_MASK: c_uint = 0x7ffull;

pub const CR_STATUS_SHIFT: c_int = 11;
pub const CR_STATUS_MASK: c_uint = 0x1ull;

pub const CR_CREDIT_RETURN_DUE_TO_PBC_SHIFT: c_int = 12;
pub const CR_CREDIT_RETURN_DUE_TO_PBC_MASK: c_uint = 0x1ull;

pub const CR_CREDIT_RETURN_DUE_TO_THRESHOLD_SHIFT: c_int = 13;
pub const CR_CREDIT_RETURN_DUE_TO_THRESHOLD_MASK: c_uint = 0x1ull;

pub const CR_CREDIT_RETURN_DUE_TO_ERR_SHIFT: c_int = 14;
pub const CR_CREDIT_RETURN_DUE_TO_ERR_MASK: c_uint = 0x1ull;

pub const CR_CREDIT_RETURN_DUE_TO_FORCE_SHIFT: c_int = 15;
pub const CR_CREDIT_RETURN_DUE_TO_FORCE_MASK: c_uint = 0x1ull;

// Specific IRQ sources
pub const CCE_ERR_INT: c_int = 0;
pub const RXE_ERR_INT: c_int = 1;
pub const MISC_ERR_INT: c_int = 2;
pub const PIO_ERR_INT: c_int = 4;
pub const SDMA_ERR_INT: c_int = 5;
pub const EGRESS_ERR_INT: c_int = 6;
pub const TXE_ERR_INT: c_int = 7;
pub const PBC_INT: c_int = 240;
pub const GPIO_ASSERT_INT: c_int = 241;
pub const QSFP1_INT: c_int = 242;
pub const QSFP2_INT: c_int = 243;
pub const TCRIT_INT: c_int = 244;
// interrupt source ranges

pub const IS_GENERAL_ERR_START: c_int = 0;
pub const IS_SDMAENG_ERR_START: c_int = 16;
pub const IS_SENDCTXT_ERR_START: c_int = 32;
pub const IS_SDMA_START: c_int = 192;
pub const IS_SDMA_PROGRESS_START: c_int = 208;
pub const IS_SDMA_IDLE_START: c_int = 224;
pub const IS_VARIOUS_START: c_int = 240;
pub const IS_DC_START: c_int = 248;
pub const IS_RCVAVAIL_START: c_int = 256;
pub const IS_RCVURGENT_START: c_int = 416;
pub const IS_SENDCREDIT_START: c_int = 576;
pub const IS_RESERVED_START: c_int = 736;
pub const IS_LAST_SOURCE: c_int = 767;
// derived interrupt source values
pub const IS_GENERAL_ERR_END: c_int = 7;
pub const IS_SDMAENG_ERR_END: c_int = 31;
pub const IS_SENDCTXT_ERR_END: c_int = 191;
pub const IS_SDMA_END: c_int = 207;
pub const IS_SDMA_PROGRESS_END: c_int = 223;
pub const IS_SDMA_IDLE_END: c_int = 239;
pub const IS_VARIOUS_END: c_int = 244;
pub const IS_DC_END: c_int = 255;
pub const IS_RCVAVAIL_END: c_int = 415;
pub const IS_RCVURGENT_END: c_int = 575;
pub const IS_SENDCREDIT_END: c_int = 735;

// DCC_CFG_PORT_CONFIG logical link states
pub const LSTATE_DOWN: c_uint = 0x1;
pub const LSTATE_INIT: c_uint = 0x2;
pub const LSTATE_ARMED: c_uint = 0x3;
pub const LSTATE_ACTIVE: c_uint = 0x4;
// DCC_CFG_RESET reset states

// 0x17

// DC8051_STS_CUR_STATE port values (physical link states)
pub const PLS_DISABLED: c_uint = 0x30;
pub const PLS_OFFLINE: c_uint = 0x90;
pub const PLS_OFFLINE_QUIET: c_uint = 0x90;
pub const PLS_OFFLINE_PLANNED_DOWN_INFORM: c_uint = 0x91;
pub const PLS_OFFLINE_READY_TO_QUIET_LT: c_uint = 0x92;
pub const PLS_OFFLINE_REPORT_FAILURE: c_uint = 0x93;
pub const PLS_OFFLINE_READY_TO_QUIET_BCC: c_uint = 0x94;
pub const PLS_OFFLINE_QUIET_DURATION: c_uint = 0x95;
pub const PLS_POLLING: c_uint = 0x20;
pub const PLS_POLLING_QUIET: c_uint = 0x20;
pub const PLS_POLLING_ACTIVE: c_uint = 0x21;
pub const PLS_CONFIGPHY: c_uint = 0x40;
pub const PLS_CONFIGPHY_DEBOUCE: c_uint = 0x40;
pub const PLS_CONFIGPHY_ESTCOMM: c_uint = 0x41;
pub const PLS_CONFIGPHY_ESTCOMM_TXRX_HUNT: c_uint = 0x42;
pub const PLS_CONFIGPHY_ESTCOMM_LOCAL_COMPLETE: c_uint = 0x43;
pub const PLS_CONFIGPHY_OPTEQ: c_uint = 0x44;
pub const PLS_CONFIGPHY_OPTEQ_OPTIMIZING: c_uint = 0x44;
pub const PLS_CONFIGPHY_OPTEQ_LOCAL_COMPLETE: c_uint = 0x45;
pub const PLS_CONFIGPHY_VERIFYCAP: c_uint = 0x46;
pub const PLS_CONFIGPHY_VERIFYCAP_EXCHANGE: c_uint = 0x46;
pub const PLS_CONFIGPHY_VERIFYCAP_LOCAL_COMPLETE: c_uint = 0x47;
pub const PLS_CONFIGLT: c_uint = 0x48;
pub const PLS_CONFIGLT_CONFIGURE: c_uint = 0x48;
pub const PLS_CONFIGLT_LINK_TRANSFER_ACTIVE: c_uint = 0x49;
pub const PLS_LINKUP: c_uint = 0x50;
pub const PLS_PHYTEST: c_uint = 0xB0;
pub const PLS_INTERNAL_SERDES_LOOPBACK: c_uint = 0xe1;
pub const PLS_QUICK_LINKUP: c_uint = 0xe2;
// DC_DC8051_CFG_HOST_CMD_0.REQ_TYPE - 8051 host commands
pub const HCMD_LOAD_CONFIG_DATA: c_uint = 0x01;
pub const HCMD_READ_CONFIG_DATA: c_uint = 0x02;
pub const HCMD_CHANGE_PHY_STATE: c_uint = 0x03;
pub const HCMD_SEND_LCB_IDLE_MSG: c_uint = 0x04;
pub const HCMD_MISC: c_uint = 0x05;
pub const HCMD_READ_LCB_IDLE_MSG: c_uint = 0x06;
pub const HCMD_READ_LCB_CSR: c_uint = 0x07;
pub const HCMD_WRITE_LCB_CSR: c_uint = 0x08;
pub const HCMD_INTERFACE_TEST: c_uint = 0xff;
// DC_DC8051_CFG_HOST_CMD_1.RETURN_CODE - 8051 host command return
pub const HCMD_SUCCESS: c_int = 2;
// DC_DC8051_DBG_ERR_INFO_SET_BY_8051.ERROR - error flags

// DC_DC8051_DBG_ERR_INFO_SET_BY_8051.HOST_MSG - host message flags

// DC_DC8051_CFG_EXT_DEV_1.REQ_TYPE - 8051 host requests
pub const HREQ_LOAD_CONFIG: c_uint = 0x01;
pub const HREQ_SAVE_CONFIG: c_uint = 0x02;
pub const HREQ_READ_CONFIG: c_uint = 0x03;
pub const HREQ_SET_TX_EQ_ABS: c_uint = 0x04;
pub const HREQ_SET_TX_EQ_REL: c_uint = 0x05;
pub const HREQ_ENABLE: c_uint = 0x06;
pub const HREQ_LCB_RESET: c_uint = 0x07;
pub const HREQ_CONFIG_DONE: c_uint = 0xfe;
pub const HREQ_INTERFACE_TEST: c_uint = 0xff;
// DC_DC8051_CFG_EXT_DEV_0.RETURN_CODE - 8051 host request return codes
pub const HREQ_INVALID: c_uint = 0x01;
pub const HREQ_SUCCESS: c_uint = 0x02;
pub const HREQ_NOT_SUPPORTED: c_uint = 0x03;
pub const HREQ_FEATURE_NOT_SUPPORTED: c_uint = 0x04 /* request specific feature */;
pub const HREQ_REQUEST_REJECTED: c_uint = 0xfe;
pub const HREQ_EXECUTION_ONGOING: c_uint = 0xff;
// MISC host command functions
pub const HCMD_MISC_REQUEST_LCB_ACCESS: c_uint = 0x1;
pub const HCMD_MISC_GRANT_LCB_ACCESS: c_uint = 0x2;
// idle flit message types
pub const IDLE_PHYSICAL_LINK_MGMT: c_uint = 0x1;
pub const IDLE_CRU: c_uint = 0x2;
pub const IDLE_SMA: c_uint = 0x3;
pub const IDLE_POWER_MGMT: c_uint = 0x4;
// idle flit message send fields (both send and read)
pub const IDLE_PAYLOAD_MASK: c_uint = 0xffffffffffull /* 40 bits */;
pub const IDLE_PAYLOAD_SHIFT: c_int = 8;
pub const IDLE_MSG_TYPE_MASK: c_uint = 0xf;
pub const IDLE_MSG_TYPE_SHIFT: c_int = 0;
// idle flit message read fields
pub const READ_IDLE_MSG_TYPE_MASK: c_uint = 0xf;
pub const READ_IDLE_MSG_TYPE_SHIFT: c_int = 0;
// SMA idle flit payload commands
pub const SMA_IDLE_ARM: c_int = 1;
pub const SMA_IDLE_ACTIVE: c_int = 2;
// DC_DC8051_CFG_MODE.GENERAL bits
pub const DISABLE_SELF_GUID_CHECK: c_uint = 0x2;
// Bad L2 frame error code
pub const BAD_L2_ERR: c_uint = 0x6;
//
// Eager buffer minimum and maximum sizes supported by the hardware.
// All power-of-two sizes in between are supported as well.
// MAX_EAGER_BUFFER_TOTAL is the maximum size of memory
// allocatable for Eager buffer to a single context. All others
// are limits for the RcvArray entries.
//

pub const HFI1_MIN_HDRQ_EGRBUF_CNT: c_int = 32;
pub const HFI1_MAX_HDRQ_EGRBUF_CNT: c_int = 16352;
//
// Receive expected base and count and eager base and count increment -
// the CSR fields hold multiples of this value.
//
pub const RCV_SHIFT: c_int = 3;

//
// Receive header queue entry increment - the CSR holds multiples of
// this value.
//
pub const HDRQ_SIZE_SHIFT: c_int = 5;

//
// Freeze handling flags
//
pub const FREEZE_ABORT: c_uint = 0x01	/* do not do recovery */;
pub const FREEZE_SELF: c_uint = 0x02	/* initiate the freeze */;
pub const FREEZE_LINK_DOWN: c_uint = 0x04	/* link is down */;
//
// Chip implementation codes.
//
pub const ICODE_RTL_SILICON: c_uint = 0x00;
pub const ICODE_RTL_VCS_SIMULATION: c_uint = 0x01;
pub const ICODE_FPGA_EMULATION: c_uint = 0x02;
pub const ICODE_FUNCTIONAL_SIMULATOR: c_uint = 0x03;
//
// 8051 data memory size.
//
pub const DC8051_DATA_MEM_SIZE: c_uint = 0x1000;
//
// 8051 firmware registers
//
pub const NUM_GENERAL_FIELDS: c_uint = 0x17;
pub const NUM_LANE_FIELDS: c_uint = 0x8;
// 8051 general register Field IDs
pub const LINK_OPTIMIZATION_SETTINGS: c_uint = 0x00;
pub const LINK_TUNING_PARAMETERS: c_uint = 0x02;
pub const DC_HOST_COMM_SETTINGS: c_uint = 0x03;
pub const TX_SETTINGS: c_uint = 0x06;
pub const VERIFY_CAP_LOCAL_PHY: c_uint = 0x07;
pub const VERIFY_CAP_LOCAL_FABRIC: c_uint = 0x08;
pub const VERIFY_CAP_LOCAL_LINK_MODE: c_uint = 0x09;
pub const LOCAL_DEVICE_ID: c_uint = 0x0a;
pub const RESERVED_REGISTERS: c_uint = 0x0b;
pub const LOCAL_LNI_INFO: c_uint = 0x0c;
pub const REMOTE_LNI_INFO: c_uint = 0x0d;
pub const MISC_STATUS: c_uint = 0x0e;
pub const VERIFY_CAP_REMOTE_PHY: c_uint = 0x0f;
pub const VERIFY_CAP_REMOTE_FABRIC: c_uint = 0x10;
pub const VERIFY_CAP_REMOTE_LINK_WIDTH: c_uint = 0x11;
pub const LAST_LOCAL_STATE_COMPLETE: c_uint = 0x12;
pub const LAST_REMOTE_STATE_COMPLETE: c_uint = 0x13;
pub const LINK_QUALITY_INFO: c_uint = 0x14;
pub const REMOTE_DEVICE_ID: c_uint = 0x15;
pub const LINK_DOWN_REASON: c_uint = 0x16 /* first byte of offset 0x16 */;
pub const VERSION_PATCH: c_uint = 0x16 /* last byte of offset 0x16 */;
// 8051 lane specific register field IDs
pub const TX_EQ_SETTINGS: c_uint = 0x00;
pub const CHANNEL_LOSS_SETTINGS: c_uint = 0x05;
// Lane ID for general configuration registers
pub const GENERAL_CONFIG: c_int = 4;
// LINK_TUNING_PARAMETERS fields
pub const TUNING_METHOD_SHIFT: c_int = 24;
// LINK_OPTIMIZATION_SETTINGS fields
pub const ENABLE_EXT_DEV_CONFIG_SHIFT: c_int = 24;
// LOAD_DATA 8051 command shifts and fields
pub const LOAD_DATA_FIELD_ID_SHIFT: c_int = 40;
pub const LOAD_DATA_FIELD_ID_MASK: c_uint = 0xfull;
pub const LOAD_DATA_LANE_ID_SHIFT: c_int = 32;
pub const LOAD_DATA_LANE_ID_MASK: c_uint = 0xfull;
pub const LOAD_DATA_DATA_SHIFT: c_uint = 0x0;
pub const LOAD_DATA_DATA_MASK: c_uint = 0xffffffffull;
// READ_DATA 8051 command shifts and fields
pub const READ_DATA_FIELD_ID_SHIFT: c_int = 40;
pub const READ_DATA_FIELD_ID_MASK: c_uint = 0xffull;
pub const READ_DATA_LANE_ID_SHIFT: c_int = 32;
pub const READ_DATA_LANE_ID_MASK: c_uint = 0xffull;
pub const READ_DATA_DATA_SHIFT: c_uint = 0x0;
pub const READ_DATA_DATA_MASK: c_uint = 0xffffffffull;
// TX settings fields
pub const ENABLE_LANE_TX_SHIFT: c_int = 0;
pub const ENABLE_LANE_TX_MASK: c_uint = 0xff;
pub const TX_POLARITY_INVERSION_SHIFT: c_int = 8;
pub const TX_POLARITY_INVERSION_MASK: c_uint = 0xff;
pub const RX_POLARITY_INVERSION_SHIFT: c_int = 16;
pub const RX_POLARITY_INVERSION_MASK: c_uint = 0xff;
pub const MAX_RATE_SHIFT: c_int = 24;
pub const MAX_RATE_MASK: c_uint = 0xff;
// verify capability PHY fields
pub const CONTINIOUS_REMOTE_UPDATE_SUPPORT_SHIFT: c_uint = 0x4;
pub const CONTINIOUS_REMOTE_UPDATE_SUPPORT_MASK: c_uint = 0x1;
pub const POWER_MANAGEMENT_SHIFT: c_uint = 0x0;
pub const POWER_MANAGEMENT_MASK: c_uint = 0xf;
// 8051 lane register Field IDs
pub const SPICO_FW_VERSION: c_uint = 0x7	/* SPICO firmware version */;
// SPICO firmware version fields
pub const SPICO_ROM_VERSION_SHIFT: c_int = 0;
pub const SPICO_ROM_VERSION_MASK: c_uint = 0xffff;
pub const SPICO_ROM_PROD_ID_SHIFT: c_int = 16;
pub const SPICO_ROM_PROD_ID_MASK: c_uint = 0xffff;
// verify capability fabric fields
pub const VAU_SHIFT: c_int = 0;
pub const VAU_MASK: c_uint = 0x0007;
pub const Z_SHIFT: c_int = 3;
pub const Z_MASK: c_uint = 0x0001;
pub const VCU_SHIFT: c_int = 4;
pub const VCU_MASK: c_uint = 0x0007;
pub const VL15BUF_SHIFT: c_int = 8;
pub const VL15BUF_MASK: c_uint = 0x0fff;
pub const CRC_SIZES_SHIFT: c_int = 20;
pub const CRC_SIZES_MASK: c_uint = 0x7;
// verify capability local link width fields

pub const LINK_WIDTH_MASK: c_uint = 0xffff		/* also for remote link width */;
pub const LOCAL_FLAG_BITS_SHIFT: c_int = 16;
pub const LOCAL_FLAG_BITS_MASK: c_uint = 0xff;
pub const MISC_CONFIG_BITS_SHIFT: c_int = 24;
pub const MISC_CONFIG_BITS_MASK: c_uint = 0xff;
// verify capability remote link width fields
pub const REMOTE_TX_RATE_SHIFT: c_int = 16;
pub const REMOTE_TX_RATE_MASK: c_uint = 0xff;
// LOCAL_DEVICE_ID fields
pub const LOCAL_DEVICE_REV_SHIFT: c_int = 0;
pub const LOCAL_DEVICE_REV_MASK: c_uint = 0xff;
pub const LOCAL_DEVICE_ID_SHIFT: c_int = 8;
pub const LOCAL_DEVICE_ID_MASK: c_uint = 0xffff;
// REMOTE_DEVICE_ID fields
pub const REMOTE_DEVICE_REV_SHIFT: c_int = 0;
pub const REMOTE_DEVICE_REV_MASK: c_uint = 0xff;
pub const REMOTE_DEVICE_ID_SHIFT: c_int = 8;
pub const REMOTE_DEVICE_ID_MASK: c_uint = 0xffff;
// local LNI link width fields
pub const ENABLE_LANE_RX_SHIFT: c_int = 16;
pub const ENABLE_LANE_RX_MASK: c_uint = 0xff;
// mask, shift for reading 'mgmt_enabled' value from REMOTE_LNI_INFO field
pub const MGMT_ALLOWED_SHIFT: c_int = 23;
pub const MGMT_ALLOWED_MASK: c_uint = 0x1;
// mask, shift for 'link_quality' within LINK_QUALITY_INFO field
pub const LINK_QUALITY_SHIFT: c_int = 24;
pub const LINK_QUALITY_MASK: c_uint = 0x7;
//
// mask, shift for reading 'planned_down_remote_reason_code'
// from LINK_QUALITY_INFO field
//
pub const DOWN_REMOTE_REASON_SHIFT: c_int = 16;
pub const DOWN_REMOTE_REASON_MASK: c_uint = 0xff;
pub const HOST_INTERFACE_VERSION: c_int = 1;
pub const HOST_INTERFACE_VERSION_SHIFT: c_int = 16;
pub const HOST_INTERFACE_VERSION_MASK: c_uint = 0xff;
// verify capability PHY power management bits
pub const PWRM_BER_CONTROL: c_uint = 0x1;
pub const PWRM_BANDWIDTH_CONTROL: c_uint = 0x2;
// 8051 link down reasons
pub const LDR_LINK_TRANSFER_ACTIVE_LOW: c_uint = 0xa;
pub const LDR_RECEIVED_LINKDOWN_IDLE_MSG: c_uint = 0xb;
pub const LDR_RECEIVED_HOST_OFFLINE_REQ: c_uint = 0xc;
// verify capability fabric CRC size bits

// misc status version fields
pub const STS_FM_VERSION_MINOR_SHIFT: c_int = 16;
pub const STS_FM_VERSION_MINOR_MASK: c_uint = 0xff;
pub const STS_FM_VERSION_MAJOR_SHIFT: c_int = 24;
pub const STS_FM_VERSION_MAJOR_MASK: c_uint = 0xff;
pub const STS_FM_VERSION_PATCH_SHIFT: c_int = 24;
pub const STS_FM_VERSION_PATCH_MASK: c_uint = 0xff;
// LCB_CFG_CRC_MODE TX_VAL and RX_VAL CRC mode values
pub const LCB_CRC_16B: c_uint = 0x0	/* 16b CRC */;
pub const LCB_CRC_14B: c_uint = 0x1	/* 14b CRC */;
pub const LCB_CRC_48B: c_uint = 0x2	/* 48b CRC */;
pub const LCB_CRC_12B_16B_PER_LANE: c_uint = 0x3	/* 12b-16b per lane CRC */;
//
// the following enum is (almost) a copy/paste of the definition
// in the OPA spec, section 20.2.2.6.8 (PortInfo)
//
// 48-bit overlapping LTP CRC mode (optional)
// 12 to 16 bit per lane LTP CRC mode (optional)
// timeouts

// cclock tick time, in picoseconds per tick: 1/speed * 10^12

//
// Mask of enabled MISC errors.  Do not enable the two RSA engine errors -
// see firmware.c:run_rsa() for details.
//

// valid values for the loopback module parameter

pub const LOOPBACK_SERDES: c_int = 1;
pub const LOOPBACK_LCB: c_int = 2;

// set up bits in MISC_CONFIG_BITS
pub const LOOPBACK_SERDES_CONFIG_BIT_MASK_SHIFT: c_int = 0;
pub const EXT_CFG_LCB_RESET_SUPPORTED_SHIFT: c_int = 3;
// read and write hardware registers
extern "C" {
    pub fn read_csr(dd: *const hfi1_devdata, offset: u32) -> u64;
}
extern "C" {
    pub fn write_csr(dd: *const hfi1_devdata, offset: u32, value: u64);
}
//
// The *_kctxt_* flavor of the CSR read/write functions are for
// per-context or per-SDMA CSRs that are not mappable to user-space.
// Their spacing is not a PAGE_SIZE multiple.
//
// kernel per-context CSRs are separated by 0x100
extern "C" {
    pub fn read_csr(_arg: dd, ctxt): *mut *mut offset0 + (0x100) -> return;
}
// kernel per-context CSRs are separated by 0x100
extern "C" {
    pub fn read_lcb_csr(dd: *mut hfi1_devdata, offset: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn write_lcb_csr(dd: *mut hfi1_devdata, offset: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn get_csr_addr(_arg: dd, ctxt): *mut *mut offset0 + (0x100) -> return;
}
//
// The *_uctxt_* flavor of the CSR read/write functions are for
// per-context CSRs that are mappable to user space. All these CSRs
// are spaced by a PAGE_SIZE multiple in order to be mappable to
// different processes without exposing other contexts' CSRs
//
// user per-context CSRs are separated by 0x1000
extern "C" {
    pub fn read_csr(_arg: dd, ctxt): *mut *mut offset0 + (0x1000) -> return;
}
// user per-context CSRs are separated by 0x1000
extern "C" {
    pub fn read_csr(_arg: dd, _arg: RCV_CONTEXTS) -> return;
}
extern "C" {
    pub fn read_csr(_arg: dd, _arg: SEND_CONTEXTS) -> return;
}
extern "C" {
    pub fn read_csr(_arg: dd, _arg: SEND_DMA_ENGINES) -> return;
}
extern "C" {
    pub fn read_csr(_arg: dd, _arg: SEND_PIO_MEM_SIZE) -> return;
}
extern "C" {
    pub fn read_csr(_arg: dd, _arg: SEND_DMA_MEM_SIZE) -> return;
}
extern "C" {
    pub fn read_csr(_arg: dd, _arg: RCV_ARRAY_CNT) -> return;
}
extern "C" {
    pub fn encode_rcv_header_entry_size(size: u8) -> u8;
}
extern "C" {
    pub fn hfi1_validate_rcvhdrcnt(pdev: *mut pci_dev, thecnt: c_uint) -> c_int;
}
extern "C" {
    pub fn set_hdrq_regs(dd: *mut hfi1_devdata, ctxt: u8, entsize: u8, hdrcnt: u16);
}
// firmware.c
pub const SBUS_MASTER_BROADCAST: c_uint = 0xfd;

// SBus commands
pub const RESET_SBUS_RECEIVER: c_uint = 0x20;
pub const WRITE_SBUS_RECEIVER: c_uint = 0x21;
pub const READ_SBUS_RECEIVER: c_uint = 0x22;
extern "C" {
    pub fn set_sbus_fast_mode(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn clear_sbus_fast_mode(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_firmware_init(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn load_pcie_firmware(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn load_firmware(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn dispose_firmware();
}
extern "C" {
    pub fn acquire_hw_mutex(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn release_hw_mutex(dd: *mut hfi1_devdata);
}
//
// Bitmask of dynamic access for ASIC block chip resources.  Each HFI has its
// own range of bits for the resource so it can clear its own bits on
// starting and exiting.  If either HFI has the resource bit set, the
// resource is in use.  The separate bit ranges are:
// HFI0 bits  7:0
// HFI1 bits 15:8
//
pub const CR_SBUS: c_uint = 0x01	/* SBUS, THERM, and PCIE registers */;
pub const CR_EPROM: c_uint = 0x02	/* EEP, GPIO registers */;
pub const CR_I2C1: c_uint = 0x04	/* QSFP1_OE register */;
pub const CR_I2C2: c_uint = 0x08	/* QSFP2_OE register */;

//
// Bitmask of static ASIC states these are outside of the dynamic ASIC
// block chip resources above.  These are to be set once and never cleared.
// Must be holding the SBus dynamic flag when setting.
//
pub const CR_THERM_INIT: c_uint = 0x010000;
extern "C" {
    pub fn acquire_chip_resource(dd: *mut hfi1_devdata, resource: u32, mswait: u32) -> c_int;
}
extern "C" {
    pub fn release_chip_resource(dd: *mut hfi1_devdata, resource: u32);
}
extern "C" {
    pub fn init_chip_resources(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn finish_chip_resources(dd: *mut hfi1_devdata);
}
// ms wait time for access to an SBus resoure

// ms wait time for a qsfp (i2c) chain to become available

extern "C" {
    pub fn fabric_serdes_reset(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn read_8051_data(dd: *mut hfi1_devdata, addr: u32, len: u32, result: *mut u64) -> c_int;
}
// chip.c
extern "C" {
    pub fn write_host_interface_version(dd: *mut hfi1_devdata, version: u8) -> c_int;
}
extern "C" {
    pub fn read_guid(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn wait_fm_ready(dd: *mut hfi1_devdata, mstimeout: u32) -> c_int;
}
extern "C" {
    pub fn set_link_state(: *mut hfi1_pportdata, state: u32) -> c_int;
}
extern "C" {
    pub fn port_ltp_to_cap(port_ltp: c_int) -> c_int;
}
extern "C" {
    pub fn handle_verify_cap(work: *mut work_struct);
}
extern "C" {
    pub fn handle_freeze(work: *mut work_struct);
}
extern "C" {
    pub fn handle_link_up(work: *mut work_struct);
}
extern "C" {
    pub fn handle_link_down(work: *mut work_struct);
}
extern "C" {
    pub fn handle_link_downgrade(work: *mut work_struct);
}
extern "C" {
    pub fn handle_link_bounce(work: *mut work_struct);
}
extern "C" {
    pub fn handle_start_link(work: *mut work_struct);
}
extern "C" {
    pub fn handle_sma_message(work: *mut work_struct);
}
extern "C" {
    pub fn reset_qsfp(ppd: *mut hfi1_pportdata) -> c_int;
}
extern "C" {
    pub fn qsfp_event(work: *mut work_struct);
}
extern "C" {
    pub fn start_freeze_handling(ppd: *mut hfi1_pportdata, flags: c_int);
}
extern "C" {
    pub fn send_idle_sma(dd: *mut hfi1_devdata, message: u64) -> c_int;
}
extern "C" {
    pub fn load_8051_config(: *mut hfi1_devdata, _arg: u8, _arg: u8, _arg: u32) -> c_int;
}
extern "C" {
    pub fn read_8051_config(: *mut hfi1_devdata, _arg: u8, _arg: u8, : *mut u32) -> c_int;
}
extern "C" {
    pub fn start_link(ppd: *mut hfi1_pportdata) -> c_int;
}
extern "C" {
    pub fn bringup_serdes(ppd: *mut hfi1_pportdata) -> c_int;
}
extern "C" {
    pub fn set_intr_state(dd: *mut hfi1_devdata, enable: u32);
}
extern "C" {
    pub fn stop_drain_data_vls(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn open_fill_data_vls(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn ns_to_cclock(dd: *mut hfi1_devdata, ns: u32) -> u32;
}
extern "C" {
    pub fn cclock_to_ns(dd: *mut hfi1_devdata, cclock: u32) -> u32;
}
extern "C" {
    pub fn get_linkup_link_widths(ppd: *mut hfi1_pportdata);
}
extern "C" {
    pub fn read_ltp_rtt(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn clear_linkup_counters(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hdrqempty(rcd: *mut hfi1_ctxtdata) -> u32;
}
extern "C" {
    pub fn is_ax(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn is_bx(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn is_urg_masked(rcd: *mut hfi1_ctxtdata) -> bool;
}
extern "C" {
    pub fn read_physical_state(dd: *mut hfi1_devdata) -> u32;
}
extern "C" {
    pub fn chip_to_opa_pstate(dd: *mut hfi1_devdata, chip_pstate: u32) -> u32;
}
extern "C" {
    pub fn driver_pstate(ppd: *mut hfi1_pportdata) -> u32;
}
extern "C" {
    pub fn driver_lstate(ppd: *mut hfi1_pportdata) -> u32;
}
extern "C" {
    pub fn acquire_lcb_access(dd: *mut hfi1_devdata, sleep_ok: c_int) -> c_int;
}
extern "C" {
    pub fn release_lcb_access(dd: *mut hfi1_devdata, sleep_ok: c_int) -> c_int;
}

extern "C" {
    pub fn read_dev_cntr(dd: *mut hfi1_devdata, index: c_int, vl: c_int) -> u64;
}
extern "C" {
    pub fn write_dev_cntr(dd: *mut hfi1_devdata, index: c_int, vl: c_int, data: u64) -> u64;
}
extern "C" {
    pub fn read_port_cntr(ppd: *mut hfi1_pportdata, index: c_int, vl: c_int) -> u64;
}
extern "C" {
    pub fn write_port_cntr(ppd: *mut hfi1_pportdata, index: c_int, vl: c_int, data: u64) -> u64;
}
extern "C" {
    pub fn read_logical_state(dd: *mut hfi1_devdata) -> u32;
}
extern "C" {
    pub fn force_recv_intr(rcd: *mut hfi1_ctxtdata);
}
// Per VL indexes
// Per device counter indexes
// MISC_ERR_STATUS
// CceErrStatus
//
// A special counter that is the aggregate count
// of all the cce_err_status errors.  The remainder
// are actual bits in the CceErrStatus register.
//
// RcvErrStatus
// SendPioErrStatus
// SendDmaErrStatus
// SendEgressErrStatus
// SendErrStatus
// SendCtxtErrStatus
// SendDmaEngErrStatus
// Per port counter indexes
extern "C" {
    pub fn get_all_cpu_total(cntr: *mut u64 __percpu) -> u64;
}
extern "C" {
    pub fn hfi1_start_cleanup(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_clear_tids(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_init_ctxt(sc: *mut send_context);
}
extern "C" {
    pub fn hfi1_quiet_serdes(ppd: *mut hfi1_pportdata);
}
extern "C" {
    pub fn hfi1_read_cntrs(dd: *mut hfi1_devdata, namep: *mut c_char, cntrp: *mut u64) -> u32;
}
extern "C" {
    pub fn hfi1_read_portcntrs(ppd: *mut hfi1_pportdata, namep: *mut c_char, cntrp: *mut u64) -> u32;
}
extern "C" {
    pub fn hfi1_get_ib_cfg(ppd: *mut hfi1_pportdata, which: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_set_ib_cfg(ppd: *mut hfi1_pportdata, which: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn hfi1_clear_ctxt_jkey(dd: *mut hfi1_devdata, ctxt: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_clear_ctxt_pkey(dd: *mut hfi1_devdata, ctxt: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_read_link_quality(dd: *mut hfi1_devdata, link_quality: *mut u8);
}
extern "C" {
    pub fn general_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sdma_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn receive_context_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn receive_context_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn receive_context_interrupt_napi(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn set_intr_bits(dd: *mut hfi1_devdata, first: u16, last: u16, set: bool) -> c_int;
}
extern "C" {
    pub fn init_qsfp_int(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn remap_intr(dd: *mut hfi1_devdata, isrc: c_int, msix_intr: c_int);
}
extern "C" {
    pub fn remap_sdma_interrupts(dd: *mut hfi1_devdata, engine: c_int, msix_intr: c_int);
}
extern "C" {
    pub fn reset_interrupts(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_get_qp_map(dd: *mut hfi1_devdata, idx: u8) -> u8;
}
extern "C" {
    pub fn hfi1_init_aip_rsm(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_deinit_aip_rsm(dd: *mut hfi1_devdata);
}
//
// Interrupt source table.
//
// Each entry is an interrupt source "type".  It is ordered by increasing
// number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_table {
    pub /: *mut *mut int start; / interrupt source type start,
    pub /: *mut *mut int end; / interrupt source type end,
// routine that returns the name of the interrupt source
    pub source): *mut *mut *mut *mut char (is_name)(char name, size_t size, unsigned int,
// routine to call when receiving an interrupt
    pub source): *mut *mut *mut void (is_int)(struct hfi1_devdata dd, unsigned int,
}
