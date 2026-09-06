//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/leapraid/leapraid.h
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
// Copyright (C) 2026 LeapIO Tech Inc.
//
// LeapRAID storage and RAID controller driver.
//
// Doorbell register definitions.
pub const LEAPRAID_DB_RESET: c_uint = 0x00000000;
pub const LEAPRAID_DB_READY: c_uint = 0x10000000;
pub const LEAPRAID_DB_OPERATIONAL: c_uint = 0x20000000;
pub const LEAPRAID_DB_FAULT: c_uint = 0x40000000;
pub const LEAPRAID_DB_MASK: c_uint = 0xF0000000;
pub const LEAPRAID_DB_OVER_TEMPERATURE: c_uint = 0x2810;
pub const LEAPRAID_DB_USED: c_uint = 0x08000000;
pub const LEAPRAID_DB_DATA_MASK: c_uint = 0x0000FFFF;
pub const LEAPRAID_DB_FUNC_SHIFT: c_int = 24;
pub const LEAPRAID_DB_ADD_DWORDS_SHIFT: c_int = 16;
// Maximum number of retries waiting for doorbell to become ready.
pub const LEAPRAID_DB_RETRY_COUNT_MAX: c_int = 10;
// Maximum number of retries waiting for doorbell to become operational.
pub const LEAPRAID_DB_WAIT_OP_SHORT: c_int = 10;
pub const LEAPRAID_DB_WAIT_OP_LONG: c_int = 200;
// Maximum number of retries waiting for host to end recovery.
pub const LEAPRAID_WAIT_SHOST_RECOVERY: c_int = 400;
// Diagnostic register definitions.
pub const LEAPRAID_DIAG_WRITE_ENABLE: c_uint = 0x00000080;
pub const LEAPRAID_DIAG_RESET: c_uint = 0x00000004;
// Interrupt status register definitions.
pub const LEAPRAID_HOST2ADAPTER_DB_STATUS: c_uint = 0x80000000;
pub const LEAPRAID_ADAPTER2HOST_DB_STATUS: c_uint = 0x00000001;
// The number of debug register.
pub const LEAPRAID_DEBUGLOG_SZ_MAX: c_int = 16;
// Reply post host register definitions.
pub const REP_POST_HOST_IDX_REG_CNT: c_int = 16;
pub const LEAPRAID_RPHI_MSIX_IDX_SHIFT: c_int = 24;
// Virtual PHY flags.
pub const LEAPRAID_SAS_PHYINFO_VPHY: c_uint = 0x00001000;
// Linux driver init firmware.
pub const LEAPRAID_WHOINIT_LINUX_DRIVER: c_uint = 0x04;
// Reply descriptor post queue array mode.
pub const LEAPRAID_ADAPTER_INIT_MSGFLG_RDPQ_ARRAY_MODE: c_uint = 0x01;
// Request description flags.
pub const LEAPRAID_REQ_DESC_FLG_SCSI_IO: c_uint = 0x00;
pub const LEAPRAID_REQ_DESC_FLG_HPR: c_uint = 0x06;
pub const LEAPRAID_REQ_DESC_FLG_DFLT_TYPE: c_uint = 0x08;
// Reply description flags.
pub const LEAPRAID_RPY_DESC_FLG_TYPE_MASK: c_uint = 0x0F;
pub const LEAPRAID_RPY_DESC_FLG_SCSI_IO_SUCCESS: c_uint = 0x00;
pub const LEAPRAID_RPY_DESC_FLG_ADDRESS_REPLY: c_uint = 0x01;
pub const LEAPRAID_RPY_DESC_FLG_FP_SCSI_IO_SUCCESS: c_uint = 0x06;
pub const LEAPRAID_RPY_DESC_FLG_UNUSED: c_uint = 0x0F;
// Request and reply messages share the same set of function codes.
//
// Note: SCSIIO (SCSI I/O) represents SCSI I/O operations and is used
// consistently throughout the driver for all SCSI command handling.
//
pub const LEAPRAID_FUNC_SCSIIO: c_uint = 0x00;
pub const LEAPRAID_FUNC_SCSI_TMF: c_uint = 0x01;
pub const LEAPRAID_FUNC_ADAPTER_INIT: c_uint = 0x02;
pub const LEAPRAID_FUNC_GET_ADAPTER_FEATURES: c_uint = 0x03;
pub const LEAPRAID_FUNC_CONFIG_OP: c_uint = 0x04;
pub const LEAPRAID_FUNC_SCAN_DEV: c_uint = 0x06;
pub const LEAPRAID_FUNC_EVENT_NOTIFY: c_uint = 0x07;
pub const LEAPRAID_FUNC_FW_DOWNLOAD: c_uint = 0x09;
pub const LEAPRAID_FUNC_FW_UPLOAD: c_uint = 0x12;
pub const LEAPRAID_FUNC_SCSIIO_RAID_PASSTHROUGH: c_uint = 0x16;
pub const LEAPRAID_FUNC_SCSI_ENC_PROCESSOR: c_uint = 0x18;
pub const LEAPRAID_FUNC_SMP_PASSTHROUGH: c_uint = 0x1A;
pub const LEAPRAID_FUNC_SAS_IO_UNIT_CTRL: c_uint = 0x1B;
pub const LEAPRAID_FUNC_SCSIIO_SATA_PASSTHROUGH: c_uint = 0x1C;
pub const LEAPRAID_FUNC_ADAPTER_UNIT_RESET: c_uint = 0x40;
pub const LEAPRAID_FUNC_HANDSHAKE: c_uint = 0x42;
pub const LEAPRAID_FUNC_LOGBUF_INIT: c_uint = 0x57;
// Adapter status values.
pub const LEAPRAID_ADAPTER_STATUS_MASK: c_uint = 0x7FFF;
pub const LEAPRAID_ADAPTER_STATUS_SUCCESS: c_uint = 0x0000;
pub const LEAPRAID_ADAPTER_STATUS_BUSY: c_uint = 0x0002;
pub const LEAPRAID_ADAPTER_STATUS_INTERNAL_ERROR: c_uint = 0x0004;
pub const LEAPRAID_ADAPTER_STATUS_INSUFFICIENT_RESOURCES: c_uint = 0x0006;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_INVALID_ACTION: c_uint = 0x0020;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_INVALID_TYPE: c_uint = 0x0021;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_INVALID_PAGE: c_uint = 0x0022;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_INVALID_DATA: c_uint = 0x0023;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_NO_DEFAULTS: c_uint = 0x0024;
pub const LEAPRAID_ADAPTER_STATUS_CONFIG_CANT_COMMIT: c_uint = 0x0025;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_RECOVERED_ERROR: c_uint = 0x0040;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_DEVICE_NOT_THERE: c_uint = 0x0043;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_DATA_OVERRUN: c_uint = 0x0044;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_DATA_UNDERRUN: c_uint = 0x0045;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_IO_DATA_ERROR: c_uint = 0x0046;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_PROTOCOL_ERROR: c_uint = 0x0047;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_TASK_TERMINATED: c_uint = 0x0048;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_RESIDUAL_MISMATCH: c_uint = 0x0049;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_TASK_MGMT_FAILED: c_uint = 0x004A;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_ADAPTER_TERMINATED: c_uint = 0x004B;
pub const LEAPRAID_ADAPTER_STATUS_SCSI_EXT_TERMINATED: c_uint = 0x004C;
// SGE flags.
pub const LEAPRAID_SGE_FLG_LAST_ONE: c_uint = 0x80;
pub const LEAPRAID_SGE_FLG_EOB: c_uint = 0x40;
pub const LEAPRAID_SGE_FLG_EOL: c_uint = 0x01;
pub const LEAPRAID_SGE_FLG_SHIFT: c_int = 24;
pub const LEAPRAID_SGE_FLG_SIMPLE_ONE: c_uint = 0x10;
pub const LEAPRAID_SGE_FLG_SYSTEM_ADDR: c_uint = 0x00;
pub const LEAPRAID_SGE_FLG_H2C: c_uint = 0x04;
pub const LEAPRAID_SGE_FLG_32: c_uint = 0x00;
pub const LEAPRAID_SGE_FLG_64: c_uint = 0x02;
pub const LEAPRAID_IEEE_SGE_FLG_EOL: c_uint = 0x40;
pub const LEAPRAID_IEEE_SGE_FLG_SIMPLE_ONE: c_uint = 0x00;
pub const LEAPRAID_IEEE_SGE_FLG_CHAIN_ONE: c_uint = 0x80;
pub const LEAPRAID_IEEE_SGE_FLG_SYSTEM_ADDR: c_uint = 0x00;

pub const LEAPRAID_SGE_OFFSET_SIZE: c_int = 4;
// The type of page.
pub const LEAPRAID_CFG_PT_IO_UNIT: c_uint = 0x00;
pub const LEAPRAID_CFG_PT_ADAPTER: c_uint = 0x01;
pub const LEAPRAID_CFG_PT_BIOS: c_uint = 0x02;
pub const LEAPRAID_CFG_PT_RAID_VOLUME: c_uint = 0x08;
pub const LEAPRAID_CFG_PT_MANUFACTURING: c_uint = 0x09;
pub const LEAPRAID_CFG_PT_RAID_PHYSDISK: c_uint = 0x0A;
pub const LEAPRAID_CFG_PT_EXTENDED: c_uint = 0x0F;
// The type of extended page.
pub const LEAPRAID_CFG_EXTPT_SAS_IO_UNIT: c_uint = 0x10;
pub const LEAPRAID_CFG_EXTPT_SAS_EXP: c_uint = 0x11;
pub const LEAPRAID_CFG_EXTPT_SAS_DEV: c_uint = 0x12;
pub const LEAPRAID_CFG_EXTPT_SAS_PHY: c_uint = 0x13;
pub const LEAPRAID_CFG_EXTPT_ENC: c_uint = 0x15;
pub const LEAPRAID_CFG_EXTPT_RAID_CONFIG: c_uint = 0x16;
// Config page address.
pub const LEAPRAID_SAS_CFG_PGAD_GET_NEXT_LOOP: c_uint = 0x00000000;
pub const LEAPRAID_SAS_ENC_CFG_PGAD_HDL: c_uint = 0x10000000;
pub const LEAPRAID_SAS_DEV_CFG_PGAD_HDL: c_uint = 0x20000000;
pub const LEAPRAID_SAS_EXP_CFG_PGAD_HDL_PHY_NUM: c_uint = 0x10000000;
pub const LEAPRAID_SAS_EXP_CFD_PGAD_HDL: c_uint = 0x20000000;
pub const LEAPRAID_SAS_EXP_CFG_PGAD_PHYNUM_SHIFT: c_int = 16;
pub const LEAPRAID_RAID_VOL_CFG_PGAD_HDL: c_uint = 0x10000000;
pub const LEAPRAID_SAS_PHY_CFG_PGAD_PHY_NUMBER: c_uint = 0x00000000;
pub const LEAPRAID_PHYSDISK_CFG_PGAD_PHYSDISKNUM: c_uint = 0x10000000;
// Config page operations.
pub const LEAPRAID_CFG_ACT_PAGE_HEADER: c_uint = 0x00;
pub const LEAPRAID_CFG_ACT_PAGE_READ_CUR: c_uint = 0x01;
pub const LEAPRAID_CFG_ACT_PAGE_WRITE_CUR: c_uint = 0x02;
// BIOS page number.
pub const LEAPRAID_CFG_PAGE_NUM_BIOS2: c_uint = 0x2;
pub const LEAPRAID_CFG_PAGE_NUM_BIOS3: c_uint = 0x3;
// Manufacturing page number.
pub const LEAPRAID_CFG_PAGE_NUM_MANU0: c_uint = 0x0;
// SAS device page number.
pub const LEAPRAID_CFG_PAGE_NUM_DEV0: c_uint = 0x0;
// SAS device page 0 flags.
pub const LEAPRAID_SAS_DEV_P0_FLG_ENC_LEVEL_VALID: c_uint = 0x0002;
pub const LEAPRAID_SAS_DEV_P0_FLG_DEV_PRESENT: c_uint = 0x0001;
pub const LEAPRAID_SAS_DEV_P0_CON_NAME_LEN: c_int = 4;
// SAS I/O unit page number.
pub const LEAPRAID_CFG_PAGE_NUM_IOUNIT0: c_uint = 0x0;
pub const LEAPRAID_CFG_PAGE_NUM_IOUNIT1: c_uint = 0x1;
// SAS expander page number.
pub const LEAPRAID_CFG_PAGE_NUM_EXP0: c_uint = 0x0;
pub const LEAPRAID_CFG_PAGE_NUM_EXP1: c_uint = 0x1;
// SAS enclosure page number.
pub const LEAPRAID_CFG_PAGE_NUM_ENC0: c_uint = 0x0;
// SAS PHY page number.
pub const LEAPRAID_CFG_PAGE_NUM_PHY0: c_uint = 0x0;
// RAID volume page number.
pub const LEAPRAID_CFG_PAGE_NUM_VOL0: c_uint = 0x0;
pub const LEAPRAID_CFG_PAGE_NUM_VOL1: c_uint = 0x1;
// Physical disk page number.
pub const LEAPRAID_CFG_PAGE_NUM_PD0: c_uint = 0x0;
pub const LEAPRAID_CFG_UNIT_SIZE: c_int = 4;
// Raid volume type and state.
pub const LEAPRAID_VOL_STATE_ONLINE: c_uint = 0x03;
pub const LEAPRAID_VOL_STATE_DEGRADED: c_uint = 0x04;
pub const LEAPRAID_VOL_STATE_OPTIMAL: c_uint = 0x05;
pub const LEAPRAID_VOL_TYPE_RAID0: c_uint = 0x00;
pub const LEAPRAID_VOL_TYPE_RAID1E: c_uint = 0x01;
pub const LEAPRAID_VOL_TYPE_RAID1: c_uint = 0x02;
pub const LEAPRAID_VOL_TYPE_RAID10: c_uint = 0x05;
pub const LEAPRAID_VOL_TYPE_UNKNOWN: c_uint = 0xFF;
// Raid volume element flags.
pub const LEAPRAID_RAIDCFG_P0_EFLG_MASK_ELEMENT_TYPE: c_uint = 0x000F;
pub const LEAPRAID_RAIDCFG_P0_EFLG_VOL_PHYS_DISK_ELEMENT: c_uint = 0x0001;
pub const LEAPRAID_RAIDCFG_P0_EFLG_HOT_SPARE_ELEMENT: c_uint = 0x0002;
pub const LEAPRAID_RAIDCFG_P0_EFLG_OCE_ELEMENT: c_uint = 0x0003;
// SAS negotiated link rates.
pub const LEAPRAID_SAS_NEG_LINK_RATE_MASK_PHYSICAL: c_uint = 0x0F;
pub const LEAPRAID_SAS_NEG_LINK_RATE_UNKNOWN_LINK_RATE: c_uint = 0x00;
pub const LEAPRAID_SAS_NEG_LINK_RATE_PHY_DISABLED: c_uint = 0x01;
pub const LEAPRAID_SAS_NEG_LINK_RATE_NEGOTIATION_FAILED: c_uint = 0x02;
pub const LEAPRAID_SAS_NEG_LINK_RATE_SATA_OOB_COMPLETE: c_uint = 0x03;
pub const LEAPRAID_SAS_NEG_LINK_RATE_PORT_SELECTOR: c_uint = 0x04;
pub const LEAPRAID_SAS_NEG_LINK_RATE_SMP_RESETTING: c_uint = 0x05;
pub const LEAPRAID_SAS_NEG_LINK_RATE_SHIFT: c_int = 4;
pub const LEAPRAID_SAS_NEG_LINK_RATE_1_5: c_uint = 0x08;
pub const LEAPRAID_SAS_NEG_LINK_RATE_3_0: c_uint = 0x09;
pub const LEAPRAID_SAS_NEG_LINK_RATE_6_0: c_uint = 0x0A;
pub const LEAPRAID_SAS_NEG_LINK_RATE_12_0: c_uint = 0x0B;
pub const LEAPRAID_SAS_PRATE_MIN_RATE_MASK: c_uint = 0x0F;
pub const LEAPRAID_SAS_HWRATE_MIN_RATE_MASK: c_uint = 0x0F;
//
// Control flags for a SCSI I/O Request.
//
pub const LEAPRAID_SCSIIO_CTRL_CDB_32BYTE: c_int = 4;
pub const LEAPRAID_SCSIIO_CTRL_CDB_LEN_SHIFT: c_int = 26;
pub const LEAPRAID_SCSIIO_CTRL_NODATATRANSFER: c_uint = 0x00000000;
pub const LEAPRAID_SCSIIO_CTRL_WRITE: c_uint = 0x01000000;
pub const LEAPRAID_SCSIIO_CTRL_READ: c_uint = 0x02000000;
pub const LEAPRAID_SCSIIO_CTRL_BIDIRECTIONAL: c_uint = 0x03000000;
pub const LEAPRAID_SCSIIO_CTRL_SIMPLEQ: c_uint = 0x00000000;
pub const LEAPRAID_SCSIIO_CTRL_ORDEREDQ: c_uint = 0x00000200;
pub const LEAPRAID_SCSIIO_CTRL_CMDPRI: c_uint = 0x00000800;
// SCSI state and status.
pub const LEAPRAID_SCSI_STATUS_BUSY: c_uint = 0x08;
pub const LEAPRAID_SCSI_STATUS_RESERVATION_CONFLICT: c_uint = 0x18;
pub const LEAPRAID_SCSI_STATUS_TASK_SET_FULL: c_uint = 0x28;
pub const LEAPRAID_SCSI_STATE_RESPONSE_INFO_VALID: c_uint = 0x10;
pub const LEAPRAID_SCSI_STATE_TERMINATED: c_uint = 0x08;
pub const LEAPRAID_SCSI_STATE_NO_SCSI_STATUS: c_uint = 0x04;
pub const LEAPRAID_SCSI_STATE_AUTOSENSE_FAILED: c_uint = 0x02;
pub const LEAPRAID_SCSI_STATE_AUTOSENSE_VALID: c_uint = 0x01;
// SCSI task management defines.
pub const LEAPRAID_TM_TASKTYPE_ABORT_TASK: c_uint = 0x01;
pub const LEAPRAID_TM_TASKTYPE_ABRT_TASK_SET: c_uint = 0x02;
pub const LEAPRAID_TM_TASKTYPE_TARGET_RESET: c_uint = 0x03;
pub const LEAPRAID_TM_TASKTYPE_LOGICAL_UNIT_RESET: c_uint = 0x05;
pub const LEAPRAID_TM_TASKTYPE_CLEAR_TASK_SET: c_uint = 0x06;
pub const LEAPRAID_TM_TASKTYPE_QUERY_TASK: c_uint = 0x07;
pub const LEAPRAID_TM_TASKTYPE_CLEAR_ACA: c_uint = 0x08;
pub const LEAPRAID_TM_TASKTYPE_QUERY_TASK_SET: c_uint = 0x09;
pub const LEAPRAID_TM_TASKTYPE_QUERY_ASYNC_EVENT: c_uint = 0x0A;
pub const LEAPRAID_TM_MSGFLAGS_LINK_RESET: c_uint = 0x00;
pub const LEAPRAID_TM_RSP_INVALID_FRAME: c_uint = 0x02;
// SCSI enclosure processor request defines.
pub const LEAPRAID_SEP_REQ_ACT_WRITE_STATUS: c_uint = 0x00;
pub const LEAPRAID_SEP_REQ_FLG_DEVHDL_ADDRESS: c_uint = 0x00;
pub const LEAPRAID_SEP_REQ_FLG_ENCLOSURE_SLOT_ADDRESS: c_uint = 0x01;
pub const LEAPRAID_SEP_REQ_SLOTSTATUS_PREDICTED_FAULT: c_uint = 0x00000040;
// The capabilities of the adapter.
pub const LEAPRAID_ADAPTER_FEATURES_CAP_ATOMIC_REQ: c_uint = 0x00080000;
pub const LEAPRAID_ADAPTER_FEATURES_CAP_INTEGRATED_RAID: c_uint = 0x00001000;
// Event code definitions for the firmware.
pub const LEAPRAID_EVT_SAS_DEV_STATUS_CHANGE: c_uint = 0x000F;
pub const LEAPRAID_EVT_SAS_TOPO_CHANGE_LIST: c_uint = 0x001C;
pub const LEAPRAID_EVT_SAS_ENCL_DEV_STATUS_CHANGE: c_uint = 0x001D;
pub const LEAPRAID_EVT_IR_CHANGE: c_uint = 0x0020;
pub const LEAPRAID_EVT_TURN_ON_PFA_LED: c_uint = 0xFFFC;
pub const LEAPRAID_EVT_SCAN_DEV_DONE: c_uint = 0xFFFD;
pub const LEAPRAID_EVT_REMOVE_DEAD_DEV: c_uint = 0xFFFF;
pub const LEAPRAID_MAX_EVENT_NUM: c_int = 128;
pub const LEAPRAID_EVT_SAS_DEV_STAT_RC_INTERNAL_DEV_RESET: c_uint = 0x08;
pub const LEAPRAID_EVT_SAS_DEV_STAT_RC_CMP_INTERNAL_DEV_RESET: c_uint = 0x0E;
// RAID configuration change event.
pub const LEAPRAID_EVT_IR_RC_VOLUME_ADD: c_uint = 0x01;
pub const LEAPRAID_EVT_IR_RC_VOLUME_DELETE: c_uint = 0x02;
pub const LEAPRAID_EVT_IR_RC_PD_HIDDEN_TO_ADD: c_uint = 0x03;
pub const LEAPRAID_EVT_IR_RC_PD_UNHIDDEN_TO_DELETE: c_uint = 0x04;
pub const LEAPRAID_EVT_IR_RC_PD_CREATED_TO_HIDE: c_uint = 0x05;
pub const LEAPRAID_EVT_IR_RC_PD_DELETED_TO_EXPOSE: c_uint = 0x06;
pub const LEAPRAID_EVT_IR_RC_VOLUME_HIDE: c_uint = 0x0A;
pub const LEAPRAID_EVT_IR_RC_VOLUME_UNHIDE: c_uint = 0x0B;
// SAS topology change event.
pub const LEAPRAID_EVT_SAS_TOPO_ES_NO_EXPANDER: c_uint = 0x00;
pub const LEAPRAID_EVT_SAS_TOPO_ES_ADDED: c_uint = 0x01;
pub const LEAPRAID_EVT_SAS_TOPO_ES_NOT_RESPONDING: c_uint = 0x02;
pub const LEAPRAID_EVT_SAS_TOPO_ES_RESPONDING: c_uint = 0x03;
pub const LEAPRAID_EVT_SAS_TOPO_RC_MASK: c_uint = 0x0F;
pub const LEAPRAID_EVT_SAS_TOPO_RC_TARG_ADDED: c_uint = 0x01;
pub const LEAPRAID_EVT_SAS_TOPO_RC_TARG_NOT_RESPONDING: c_uint = 0x02;
// Enclosure device status change event.
pub const LEAPRAID_EVT_SAS_ENCL_RC_ADDED: c_uint = 0x01;
pub const LEAPRAID_EVT_SAS_ENCL_RC_NOT_RESPONDING: c_uint = 0x02;
// Device type and identifiers.
pub const LEAPRAID_DEVTYP_SEP: c_uint = 0x00004000;
pub const LEAPRAID_DEVTYP_SSP_TGT: c_uint = 0x00000400;
pub const LEAPRAID_DEVTYP_STP_TGT: c_uint = 0x00000200;
pub const LEAPRAID_DEVTYP_SMP_TGT: c_uint = 0x00000100;
pub const LEAPRAID_DEVTYP_SATA_DEV: c_uint = 0x00000080;
pub const LEAPRAID_DEVTYP_SSP_INIT: c_uint = 0x00000040;
pub const LEAPRAID_DEVTYP_STP_INIT: c_uint = 0x00000020;
pub const LEAPRAID_DEVTYP_SMP_INIT: c_uint = 0x00000010;
pub const LEAPRAID_DEVTYP_SATA_HOST: c_uint = 0x00000008;
pub const LEAPRAID_DEVTYP_MASK_DEV_TYPE: c_uint = 0x00000007;
pub const LEAPRAID_DEVTYP_NO_DEV: c_uint = 0x00000000;
pub const LEAPRAID_DEVTYP_END_DEV: c_uint = 0x00000001;
pub const LEAPRAID_DEVTYP_EDGE_EXPANDER: c_uint = 0x00000002;
pub const LEAPRAID_DEVTYP_FANOUT_EXPANDER: c_uint = 0x00000003;
// SAS control operation.
pub const LEAPRAID_SAS_OP_PHY_LINK_RESET: c_uint = 0x06;
pub const LEAPRAID_SAS_OP_PHY_HARD_RESET: c_uint = 0x07;
pub const LEAPRAID_SAS_OP_SET_PARAMETER: c_uint = 0x0F;
// Boot device defines
pub const LEAPRAID_BOOTDEV_FORM_MASK: c_uint = 0x0F;
pub const LEAPRAID_BOOTDEV_FORM_NONE: c_uint = 0x00;
pub const LEAPRAID_BOOTDEV_FORM_SAS_WWID: c_uint = 0x05;
pub const LEAPRAID_BOOTDEV_FORM_ENC_SLOT: c_uint = 0x06;
pub const LEAPRAID_BOOTDEV_FORM_DEV_NAME: c_uint = 0x07;
//
// struct leapraid_reg_base - Register layout of the LeapRAID controller
//
// @db: Doorbell register used to signal commands or status to firmware.
// @ws: Write sequence register for synchronizing doorbell operations.
// @host_diag: Diagnostic register used for status or debug reporting.
// @r1: Reserved.
// @host_int_status: Interrupt status register reporting active interrupts.
// @host_int_mask: Interrupt mask register enabling or disabling sources.
// @r2: Reserved.
// @rep_msg_host_idx: Reply message index for the next available reply slot.
// @r3: Reserved.
// @debug_log: DebugLog registers for firmware debug and diagnostic output.
// @r4: Reserved.
// @atomic_req_desc_post: Atomic register for single descriptor posting.
// @adapter_log_buf_pos: Adapter log buffer write position.
// @host_log_buf_pos: Host log buffer write position.
// @r5: Reserved.
// @rep_post_reg_idx: Array of reply post index registers, one per queue.
// The number of entries is defined by
// REP_POST_HOST_IDX_REG_CNT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_reg_base {
    pub db: __le32,
    pub ws: __le32,
    pub host_diag: __le32,
    pub r1: [__le32; 9],
    pub host_int_status: __le32,
    pub host_int_mask: __le32,
    pub r2: [__le32; 4],
    pub rep_msg_host_idx: __le32,
    pub r3: [__le32; 13],
    pub debug_log: [__le32; LEAPRAID_DEBUGLOG_SZ_MAX],
    pub r4: [__le32; 2],
    pub atomic_req_desc_post: __le32,
    pub adapter_log_buf_pos: __le32,
    pub host_log_buf_pos: __le32,
    pub r5: [__le32; 142],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_post_reg_idx {
    pub idx: __le32,
    pub r1: __le32,
    pub r2: __le32,
    pub r3: __le32,
    pub rep_post_reg_idx: [}; REP_POST_HOST_IDX_REG_CNT],
    pub __packed: },
//
// struct leapraid_atomic_req_desc - Atomic request descriptor
//
// @flg: Descriptor flag indicating the type of request (e.g. SCSI I/O).
// @msix_idx: MSI-X vector index used for interrupt routing.
// @taskid: Unique task identifier associated with this request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_atomic_req_desc {
    pub flg: u8,
    pub msix_idx: u8,
    pub taskid: __le16,
}

//
// union leapraid_rep_desc_union - Unified reply descriptor format
//
// @dflt_rep: Default reply descriptor containing basic completion info.
// @dflt_rep.rep_flg: Reply flag indicating reply type or status.
// @dflt_rep.msix_idx: MSI-X index for interrupt routing.
// @dflt_rep.taskid: Task identifier matching the submitted request.
// @r1: Reserved.
//
// @addr_rep: Address reply descriptor used when firmware returns a
// memory address associated with the reply.
// @addr_rep.rep_flg: Reply flag indicating reply type or status.
// @addr_rep.msix_idx: MSI-X index for interrupt routing.
// @addr_rep.taskid: Task identifier matching the submitted request.
// @addr_rep.rep_frame_addr: Physical address of the reply frame.
//
// @words: Raw 64-bit representation of the reply descriptor.
// @u: Alternative access using 32-bit low/high words.
// @u.low: Lower 32 bits of the descriptor.
// @u.high: Upper 32 bits of the descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_rep_desc_union {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_desc {
    pub rep_flg: u8,
    pub msix_idx: u8,
    pub taskid: __le16,
    pub r1: [u8; 4],
    pub dflt_rep: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_add_rep_desc {
    pub rep_flg: u8,
    pub msix_idx: u8,
    pub taskid: __le16,
    pub rep_frame_addr: __le32,
    pub addr_rep: },
    pub words: __le64,
    pub low: u32,
    pub high: u32,
    pub u: },
    pub __aligned(4): } __packed,
//
// struct leapraid_req - Generic request header
//
// @func_dep1: Function-dependent parameter (low 16 bits).
// @r1: Reserved.
// @func: Function code identifying the command type.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_req {
    pub func_dep1: __le16,
    pub r1: u8,
    pub func: u8,
    pub r2: [u8; 8],
}

//
// struct leapraid_rep - Generic reply header
//
// @r1: Reserved.
// @msg_len: Length of the reply message in bytes.
// @function: Function code corresponding to the request.
// @r2: Reserved.
// @adapter_status: Status code reported by the adapter.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep {
    pub r1: [u8; 2],
    pub msg_len: u8,
    pub function: u8,
    pub r2: [u8; 10],
    pub adapter_status: __le16,
    pub r3: [u8; 4],
}

//
// struct leapraid_sge_simple32 - 32-bit simple scatter-gather entry
//
// @flg_and_len: Combined field for flags and segment length.
// @addr: 32-bit physical address of the data buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sge_simple32 {
    pub flg_and_len: __le32,
    pub addr: __le32,
}

//
// struct leapraid_sge_simple64 - 64-bit simple scatter-gather entry
//
// @flg_and_len: Combined field for flags and segment length.
// @addr: 64-bit physical address of the data buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sge_simple64 {
    pub flg_and_len: __le32,
    pub addr: __le64,
    pub __aligned(4): } __packed,
//
// struct leapraid_sge_simple_union - Unified 32/64-bit SGE representation
//
// @flg_and_len: Combined field for flags and segment length.
// @u.addr32: 32-bit address field.
// @u.addr64: 64-bit address field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sge_simple_union {
    pub flg_and_len: __le32,
    pub addr32: __le32,
    pub addr64: __le64,
    pub u: } __packed __aligned(4),
    pub __aligned(4): } __packed,
//
// struct leapraid_sge_chain_union - Chained scatter-gather entry
//
// @len: Length of the chain descriptor.
// @next_chain_offset: Offset to the next SGE chain.
// @flg: Flags indicating chain or termination properties.
// @u.addr32: 32-bit physical address.
// @u.addr64: 64-bit physical address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sge_chain_union {
    pub len: __le16,
    pub next_chain_offset: u8,
    pub flg: u8,
    pub addr32: __le32,
    pub addr64: __le64,
    pub u: } __packed __aligned(4),
    pub __aligned(4): } __packed,
//
// struct leapraid_ieee_sge_simple32 - IEEE 32-bit simple SGE format
//
// @addr: 32-bit physical address of the data buffer.
// @flg_and_len: Combined field for flags and data length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_ieee_sge_simple32 {
    pub addr: __le32,
    pub flg_and_len: __le32,
}

//
// struct leapraid_ieee_sge_simple64 - IEEE 64-bit simple SGE format
//
// @addr: 64-bit physical address of the data buffer.
// @len: Length of the data segment.
// @r1: Reserved.
// @flg: Flags indicating transfer properties.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_ieee_sge_simple64 {
    pub addr: __le64,
    pub len: __le32,
    pub r1: [u8; 3],
    pub flg: u8,
    pub __aligned(4): } __packed,
//
// union leapraid_ieee_sge_simple_union - Unified IEEE SGE format
//
// @simple32: IEEE 32-bit simple SGE entry.
// @simple64: IEEE 64-bit simple SGE entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_ieee_sge_simple_union {
    pub simple32: leapraid_ieee_sge_simple32,
    pub simple64: leapraid_ieee_sge_simple64,
}

//
// union leapraid_ieee_sge_chain_union - Unified IEEE SGE chain format
//
// @chain32: IEEE 32-bit chain SGE entry.
// @chain64: IEEE 64-bit chain SGE entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_ieee_sge_chain_union {
    pub chain32: leapraid_ieee_sge_simple32,
    pub chain64: leapraid_ieee_sge_simple64,
}

//
// struct leapraid_chain64_ieee_sg - 64-bit IEEE chain SGE descriptor
//
// @addr: Physical address of the next chain segment.
// @len: Length of the current SGE.
// @r1: Reserved.
// @next_chain_offset: Offset to the next chain element.
// @flg: Flags that describe SGE attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_chain64_ieee_sg {
    pub addr: __le64,
    pub len: __le32,
    pub r1: [u8; 2],
    pub next_chain_offset: u8,
    pub flg: u8,
    pub __aligned(4): } __packed,
//
// union leapraid_ieee_sge_io_union - IEEE-style SGE union for I/O
//
// @ieee_simple: Simple IEEE SGE descriptor.
// @ieee_chain: IEEE chain SGE descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_ieee_sge_io_union {
    pub ieee_simple: leapraid_ieee_sge_simple64,
    pub ieee_chain: leapraid_chain64_ieee_sg,
}

//
// union leapraid_simple_sge_union - Union of simple SGE descriptors
//
// @leapraid_simple: LeapRAID simple SGE.
// @ieee_simple: IEEE-style simple SGE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_simple_sge_union {
    pub leapraid_simple: leapraid_sge_simple_union,
    pub ieee_simple: leapraid_ieee_sge_simple_union,
}

//
// union leapraid_sge_io_union - Combined SGE union for all I/O types
//
// @leapraid_simple: LeapRAID simple SGE format.
// @leapraid_chain: LeapRAID chain SGE format.
// @ieee_simple: IEEE simple SGE format.
// @ieee_chain: IEEE chain SGE format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_sge_io_union {
    pub leapraid_simple: leapraid_sge_simple_union,
    pub leapraid_chain: leapraid_sge_chain_union,
    pub ieee_simple: leapraid_ieee_sge_simple_union,
    pub ieee_chain: leapraid_ieee_sge_chain_union,
}

//
// struct leapraid_cfg_pg_header - Standard configuration page header
//
// @r1: Reserved.
// @page_len: Length of the page in 4-byte units.
// @page_num: Page number.
// @page_type: Page type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_cfg_pg_header {
    pub r1: u8,
    pub page_len: u8,
    pub page_num: u8,
    pub page_type: u8,
}

//
// struct leapraid_cfg_ext_pg_header - Extended configuration page header
//
// @r1: Reserved.
// @r2: Reserved.
// @page_num: Page number.
// @page_type: Page type.
// @ext_page_len: Extended page length.
// @ext_page_type: Extended page type.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_cfg_ext_pg_header {
    pub r1: u8,
    pub r2: u8,
    pub page_num: u8,
    pub page_type: u8,
    pub ext_page_len: __le16,
    pub ext_page_type: u8,
    pub r3: u8,
}

//
// struct leapraid_cfg_req - Configuration request message
//
// @action: Requested action type.
// @sgl_flag: SGL flag field.
// @chain_offset: Offset to next chain SGE.
// @func: Function code.
// @ext_page_len: Extended page length.
// @ext_page_type: Extended page type.
// @msg_flag: Message flags.
// @r1: Reserved.
// @header: Configuration page header.
// @page_addr: Address of the page buffer.
// @page_buf_sge: SGE describing the page buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_cfg_req {
    pub action: u8,
    pub sgl_flag: u8,
    pub chain_offset: u8,
    pub func: u8,
    pub ext_page_len: __le16,
    pub ext_page_type: u8,
    pub msg_flag: u8,
    pub r1: [u8; 12],
    pub header: leapraid_cfg_pg_header,
    pub page_addr: __le32,
    pub page_buf_sge: leapraid_sge_io_union,
}

//
// struct leapraid_cfg_rep - Configuration reply message
//
// @action: Action type from the request.
// @r1: Reserved.
// @msg_len: Message length in bytes.
// @func: Function code.
// @ext_page_len: Extended page length.
// @ext_page_type: Extended page type.
// @msg_flag: Message flags.
// @r2: Reserved.
// @adapter_status: Adapter status code.
// @r3: Reserved.
// @header: Configuration page header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_cfg_rep {
    pub action: u8,
    pub r1: u8,
    pub msg_len: u8,
    pub func: u8,
    pub ext_page_len: __le16,
    pub ext_page_type: u8,
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub adapter_status: __le16,
    pub r3: [u8; 4],
    pub header: leapraid_cfg_pg_header,
}

//
// struct leapraid_boot_dev_format_sas_wwid - Boot device identified by wwid
//
// @sas_addr: SAS address of the device.
// @lun: Logical unit number.
// @r1: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_boot_dev_format_sas_wwid {
    pub sas_addr: __le64,
    pub lun: [u8; 8],
    pub r1: [u8; 8],
    pub __aligned(4): } __packed,
//
// struct leapraid_boot_dev_format_enc_slot - identified by enclosure
//
// @enc_lid: Enclosure logical ID.
// @r1: Reserved.
// @slot_num: Slot number in the enclosure.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_boot_dev_format_enc_slot {
    pub enc_lid: __le64,
    pub r1: [u8; 8],
    pub slot_num: __le16,
    pub r2: [u8; 6],
    pub __aligned(4): } __packed,
//
// struct leapraid_boot_dev_format_dev_name - Boot device by device name
//
// @dev_name: Device name identifier.
// @lun: Logical unit number.
// @r1: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_boot_dev_format_dev_name {
    pub dev_name: __le64,
    pub lun: [u8; 8],
    pub r1: [u8; 8],
    pub __aligned(4): } __packed,
//
// union leapraid_boot_dev_format - Boot device format union
//
// @sas_wwid: Format using SAS WWID and LUN.
// @enc_slot: Format using enclosure slot and ID.
// @dev_name: Format using device name and LUN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_boot_dev_format {
    pub sas_wwid: leapraid_boot_dev_format_sas_wwid,
    pub enc_slot: leapraid_boot_dev_format_enc_slot,
    pub dev_name: leapraid_boot_dev_format_dev_name,
}

//
// struct leapraid_manufacturing_p0 - Manufacturing configuration page 0
//
// @header: Configuration page header.
// @chip_name: Chip name as a 16-byte ASCII string.
// @chip_revision: Chip revision as an 8-byte ASCII string.
// @board_name: Board name as a 16-byte ASCII string.
// @board_assembly: Board assembly as a 16-byte ASCII string.
// @board_tracer_number: Board tracer number as a 16-byte ASCII string.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_manufacturing_p0 {
    pub header: leapraid_cfg_pg_header,
    pub chip_name: [u8; 16],
    pub chip_revision: [u8; 8],
    pub board_name: [u8; 16],
    pub board_assembly: [u8; 16],
    pub board_tracer_number: [u8; 16],
}

//
// struct leapraid_bios_page2 - BIOS configuration page 2
//
// @header: Configuration page header.
// @r1: Reserved.
// @requested_boot_dev_form: Format type of the requested boot device.
// @r2: Reserved.
// @requested_boot_dev: Boot device requested by BIOS or user.
// @requested_alt_boot_dev_form: Format of the alternate boot device.
// @r3: Reserved.
// @requested_alt_boot_dev: Alternate boot device requested.
// @current_boot_dev_form: Format type of the active boot device.
// @r4: Reserved.
// @current_boot_dev: Currently active boot device in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_bios_page2 {
    pub header: leapraid_cfg_pg_header,
    pub r1: [u8; 24],
    pub requested_boot_dev_form: u8,
    pub r2: [u8; 3],
    pub requested_boot_dev: leapraid_boot_dev_format,
    pub requested_alt_boot_dev_form: u8,
    pub r3: [u8; 3],
    pub requested_alt_boot_dev: leapraid_boot_dev_format,
    pub current_boot_dev_form: u8,
    pub r4: [u8; 3],
    pub current_boot_dev: leapraid_boot_dev_format,
}

//
// struct leapraid_bios_page3 - BIOS configuration page 3
//
// @header: Configuration page header.
// @r1: Reserved.
// @bios_version: BIOS firmware version number.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_bios_page3 {
    pub header: leapraid_cfg_pg_header,
    pub r1: [u8; 4],
    pub bios_version: __le32,
    pub r2: [u8; 84],
}

//
// struct leapraid_raidvol0_phys_disk - Physical disk in RAID volume
//
// @r1: Reserved.
// @phys_disk_num: Physical disk number within the RAID volume.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raidvol0_phys_disk {
    pub r1: [u8; 2],
    pub phys_disk_num: u8,
    pub r2: u8,
}

//
// struct leapraid_raidvol_p0 - RAID volume configuration page 0
//
// @header: Configuration page header.
// @dev_hdl: Device handle for the RAID volume.
// @volume_state: State of the RAID volume.
// @volume_type: RAID type.
// @r1: Reserved.
// @num_phys_disks: Number of physical disks in the volume.
// @r2: Reserved.
// @phys_disk: Array of physical disks in this volume.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raidvol_p0 {
    pub header: leapraid_cfg_pg_header,
    pub dev_hdl: __le16,
    pub volume_state: u8,
    pub volume_type: u8,
    pub r1: [u8; 28],
    pub num_phys_disks: u8,
    pub r2: [u8; 3],
    pub phys_disk: [leapraid_raidvol0_phys_disk; ],
}

//
// struct leapraid_raidvol_p1 - RAID volume configuration page 1
//
// @header: Configuration page header.
// @dev_hdl: Device handle of the RAID volume.
// @r1: Reserved.
// @wwid: World-wide identifier for the volume.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raidvol_p1 {
    pub header: leapraid_cfg_pg_header,
    pub dev_hdl: __le16,
    pub r1: [u8; 42],
    pub wwid: __le64,
    pub r2: [u8; 8],
    pub __aligned(4): } __packed,
//
// struct leapraid_raidpd_p0 - Physical disk configuration page 0
//
// @header: Configuration page header.
// @dev_hdl: Device handle of the physical disk.
// @r1: Reserved.
// @phys_disk_num: Physical disk number.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raidpd_p0 {
    pub header: leapraid_cfg_pg_header,
    pub dev_hdl: __le16,
    pub r1: u8,
    pub phys_disk_num: u8,
    pub r2: [u8; 112],
}

//
// struct leapraid_sas_io_unit0_phy_info - PHY info for SAS I/O unit
//
// @port: Port number the PHY belongs to.
// @port_flg: Flags describing port status.
// @phy_flg: Flags describing PHY status.
// @neg_link_rate: Negotiated link rate of the PHY.
// @controller_phy_dev_info: Controller PHY device info.
// @attached_dev_hdl: Handle of attached device.
// @controller_dev_hdl: Handle of the controller device.
// @r1: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_io_unit0_phy_info {
    pub port: u8,
    pub port_flg: u8,
    pub phy_flg: u8,
    pub neg_link_rate: u8,
    pub controller_phy_dev_info: __le32,
    pub attached_dev_hdl: __le16,
    pub controller_dev_hdl: __le16,
    pub r1: [u8; 8],
}

//
// struct leapraid_sas_io_unit_p0 - SAS I/O unit configuration page 0
//
// @header: Extended configuration page header.
// @r1: Reserved.
// @phy_num: Number of PHYs in this unit.
// @r2: Reserved.
// @phy_info: Array of PHY information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_io_unit_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub r1: [u8; 4],
    pub phy_num: u8,
    pub r2: [u8; 3],
    pub phy_info: [leapraid_sas_io_unit0_phy_info; ],
}

//
// struct leapraid_exp_p0 - SAS expander page 0
//
// @header: Extended page header.
// @physical_port: Physical port number.
// @r1: Reserved.
// @enc_hdl: Enclosure handle.
// @sas_address: SAS address of the expander.
// @r2: Reserved.
// @dev_hdl: Device handle of this expander.
// @parent_dev_hdl: Device handle of parent expander.
// @r3: Reserved.
// @phy_num: Number of PHYs.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_exp_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub physical_port: u8,
    pub r1: u8,
    pub enc_hdl: __le16,
    pub sas_address: __le64,
    pub r2: [u8; 4],
    pub dev_hdl: __le16,
    pub parent_dev_hdl: __le16,
    pub r3: [u8; 4],
    pub phy_num: u8,
    pub r4: [u8; 27],
    pub __aligned(4): } __packed,
//
// struct leapraid_exp_p1 - SAS expander page 1
//
// @header: Extended page header.
// @r1: Reserved.
// @p_link_rate: PHY link rate.
// @hw_link_rate: Hardware supported link rate.
// @attached_dev_hdl: Attached device handle.
// @r2: Reserved.
// @neg_link_rate: Negotiated link rate.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_exp_p1 {
    pub header: leapraid_cfg_ext_pg_header,
    pub r1: [u8; 8],
    pub p_link_rate: u8,
    pub hw_link_rate: u8,
    pub attached_dev_hdl: __le16,
    pub r2: [u8; 11],
    pub neg_link_rate: u8,
    pub r3: [u8; 12],
}

//
// struct leapraid_sas_dev_p0 - SAS device page 0
//
// @header: Extended configuration page header.
// @slot: Slot number.
// @enc_hdl: Enclosure handle.
// @sas_address: SAS address.
// @parent_dev_hdl: Parent device handle.
// @phy_num: Number of PHYs.
// @r1: Reserved.
// @dev_hdl: Device handle.
// @r2: Reserved.
// @dev_info: Device information.
// @flg: Flags.
// @physical_port: Physical port number.
// @max_port_connections: Maximum port connections.
// @dev_name: Device name.
// @port_groups: Number of port groups.
// @r3: Reserved.
// @enc_level: Enclosure level.
// @connector_name: Connector identifier.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_dev_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub slot: __le16,
    pub enc_hdl: __le16,
    pub sas_address: __le64,
    pub parent_dev_hdl: __le16,
    pub phy_num: u8,
    pub r1: u8,
    pub dev_hdl: __le16,
    pub r2: [u8; 2],
    pub dev_info: __le32,
    pub flg: __le16,
    pub physical_port: u8,
    pub max_port_connections: u8,
    pub dev_name: __le64,
    pub port_groups: u8,
    pub r3: [u8; 2],
    pub enc_level: u8,
    pub connector_name: [u8; LEAPRAID_SAS_DEV_P0_CON_NAME_LEN],
    pub r4: [u8; 4],
    pub __aligned(4): } __packed,
//
// struct leapraid_sas_phy_p0 - SAS PHY configuration page 0
//
// @header: Extended configuration page header.
// @r1: Reserved.
// @attached_dev_hdl: Handle of attached device.
// @r2: Reserved.
// @p_link_rate: PHY link rate.
// @hw_link_rate: Hardware supported link rate.
// @r3: Reserved.
// @phy_info: PHY information.
// @neg_link_rate: Negotiated link rate.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_phy_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub r1: [u8; 4],
    pub attached_dev_hdl: __le16,
    pub r2: [u8; 6],
    pub p_link_rate: u8,
    pub hw_link_rate: u8,
    pub r3: [u8; 2],
    pub phy_info: __le32,
    pub neg_link_rate: u8,
    pub r4: [u8; 3],
}

//
// struct leapraid_enc_p0 - SAS enclosure page 0
//
// @header: Extended configuration page header.
// @r1: Reserved.
// @enc_lid: Enclosure logical ID.
// @r2: Reserved.
// @enc_hdl: Enclosure handle.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_enc_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub r1: [u8; 4],
    pub enc_lid: __le64,
    pub r2: [u8; 2],
    pub enc_hdl: __le16,
    pub r3: [u8; 15],
    pub __aligned(4): } __packed,
//
// struct leapraid_raid_cfg_p0_element - RAID configuration element
//
// @element_flg: Element flags.
// @vol_dev_hdl: Volume device handle.
// @r1: Reserved.
// @phys_disk_dev_hdl: Physical disk device handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raid_cfg_p0_element {
    pub element_flg: __le16,
    pub vol_dev_hdl: __le16,
    pub r1: [u8; 2],
    pub phys_disk_dev_hdl: __le16,
}

//
// struct leapraid_raid_cfg_p0 - RAID configuration page 0
//
// @header: Extended configuration page header.
// @r1: Reserved.
// @cfg_num: Configuration number.
// @r2: Reserved.
// @elements_num: Number of RAID elements.
// @r3: Reserved.
// @cfg_element: Array of RAID elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raid_cfg_p0 {
    pub header: leapraid_cfg_ext_pg_header,
    pub r1: [u8; 3],
    pub cfg_num: u8,
    pub r2: [u8; 32],
    pub elements_num: u8,
    pub r3: [u8; 3],
    pub cfg_element: [leapraid_raid_cfg_p0_element; ],
}

//
// union leapraid_mpi_scsi_io_cdb_union - SCSI I/O CDB or simple SGE
//
// @cdb32: 32-byte SCSI command descriptor block.
// @sge: Simple SGE format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_mpi_scsi_io_cdb_union {
    pub cdb32: [u8; 32],
    pub sge: leapraid_sge_simple_union,
}

//
// struct leapraid_mpi_scsiio_req - MPI SCSI I/O request
//
// @dev_hdl: Device handle for the target.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r1: Reserved.
// @msg_flg: Message flags.
// @r2: Reserved.
// @sense_buffer_low_add: Lower 32-bit address of sense buffer.
// @dma_flag: DMA flags.
// @r3: Reserved.
// @sense_buffer_len: Sense buffer length.
// @r4: Reserved.
// @sgl_offset0..3: SGL offsets.
// @skip_count: Bytes to skip before transfer.
// @data_len: Length of data transfer.
// @bi_dir_data_len: Bi-directional transfer length.
// @io_flg: I/O flags.
// @eedp_flag: End-to-end data protection flags.
// @eedp_block_size: End-to-end data protection block size.
// @r5: Reserved.
// @secondary_ref_tag: Secondary reference tag.
// @secondary_app_tag: Secondary application tag.
// @app_tag_trans_mask: Application tag mask.
// @lun: Logical Unit Number.
// @ctrl: Control flags.
// @cdb: SCSI Command Descriptor Block or simple SGE.
// @sgl: Scatter-gather list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_mpi_scsiio_req {
    pub dev_hdl: __le16,
    pub chain_offset: u8,
    pub func: u8,
    pub r1: [u8; 3],
    pub msg_flg: u8,
    pub r2: [u8; 4],
    pub sense_buffer_low_add: __le32,
    pub dma_flag: u8,
    pub r3: u8,
    pub sense_buffer_len: u8,
    pub r4: u8,
    pub sgl_offset0: u8,
    pub sgl_offset1: u8,
    pub sgl_offset2: u8,
    pub sgl_offset3: u8,
    pub skip_count: __le32,
    pub data_len: __le32,
    pub bi_dir_data_len: __le32,
    pub io_flg: __le16,
    pub eedp_flag: __le16,
    pub eedp_block_size: __le16,
    pub r5: [u8; 2],
    pub secondary_ref_tag: __le32,
    pub secondary_app_tag: __le16,
    pub app_tag_trans_mask: __le16,
    pub lun: [u8; 8],
    pub ctrl: __le32,
    pub cdb: leapraid_mpi_scsi_io_cdb_union,
    pub sgl: leapraid_sge_io_union,
}

//
// union leapraid_scsi_io_cdb_union - SCSI I/O CDB or IEEE simple SGE
//
// @cdb32: 32-byte SCSI CDB.
// @sge: IEEE simple 64-bit SGE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union leapraid_scsi_io_cdb_union {
    pub cdb32: [u8; 32],
    pub sge: leapraid_ieee_sge_simple64,
}

//
// struct leapraid_scsiio_req - SCSI I/O request
//
// @dev_hdl: Device handle.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r1: Reserved.
// @msg_flg: Message flags.
// @r2: Reserved.
// @sense_buffer_low_add: Lower 32-bit address of sense buffer.
// @dma_flag: DMA flag.
// @r3: Reserved.
// @sense_buffer_len: Sense buffer length.
// @r4: Reserved.
// @sgl_offset0-3: SGL offsets.
// @skip_count: Bytes to skip before transfer.
// @data_len: Length of data transfer.
// @bi_dir_data_len: Bi-directional transfer length.
// @io_flg: I/O flags.
// @eedp_flag: End-to-end data protection flags.
// @eedp_block_size: End-to-end data protection block size.
// @r5: Reserved.
// @secondary_ref_tag: Secondary reference tag.
// @secondary_app_tag: Secondary application tag.
// @app_tag_trans_mask: Application tag mask.
// @lun: Logical Unit Number.
// @ctrl: Control flags.
// @cdb: SCSI Command Descriptor Block or simple SGE.
// @sgl: Scatter-gather list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scsiio_req {
    pub dev_hdl: __le16,
    pub chain_offset: u8,
    pub func: u8,
    pub r1: [u8; 3],
    pub msg_flg: u8,
    pub r2: [u8; 4],
    pub sense_buffer_low_add: __le32,
    pub dma_flag: u8,
    pub r3: u8,
    pub sense_buffer_len: u8,
    pub r4: u8,
    pub sgl_offset0: u8,
    pub sgl_offset1: u8,
    pub sgl_offset2: u8,
    pub sgl_offset3: u8,
    pub skip_count: __le32,
    pub data_len: __le32,
    pub bi_dir_data_len: __le32,
    pub io_flg: __le16,
    pub eedp_flag: __le16,
    pub eedp_block_size: __le16,
    pub r5: [u8; 2],
    pub secondary_ref_tag: __le32,
    pub secondary_app_tag: __le16,
    pub app_tag_trans_mask: __le16,
    pub lun: [u8; 8],
    pub ctrl: __le32,
    pub cdb: leapraid_scsi_io_cdb_union,
    pub sgl: leapraid_ieee_sge_io_union,
}

//
// struct leapraid_scsiio_rep - SCSI I/O response
//
// @dev_hdl: Device handle.
// @msg_len: Length of response message.
// @func: Function code.
// @r1: Reserved.
// @msg_flg: Message flags.
// @r2: Reserved.
// @scsi_status: SCSI status.
// @scsi_state: SCSI state.
// @adapter_status: Adapter status.
// @r3: Reserved.
// @transfer_count: Number of bytes transferred.
// @sense_count: Number of sense bytes.
// @resp_info: Additional response info.
// @task_tag: Task identifier.
// @scsi_status_qualifier: SCSI status qualifier.
// @bi_dir_trans_count: Bi-directional transfer count.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scsiio_rep {
    pub dev_hdl: __le16,
    pub msg_len: u8,
    pub func: u8,
    pub r1: [u8; 3],
    pub msg_flg: u8,
    pub r2: [u8; 4],
    pub scsi_status: u8,
    pub scsi_state: u8,
    pub adapter_status: __le16,
    pub r3: [u8; 4],
    pub transfer_count: __le32,
    pub sense_count: __le32,
    pub resp_info: __le32,
    pub task_tag: __le16,
    pub scsi_status_qualifier: __le16,
    pub bi_dir_trans_count: __le32,
    pub r4: [__le32; 3],
}

//
// struct leapraid_scsi_tm_req - SCSI Task Management request
//
// @dev_hdl: Device handle.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r1: Reserved.
// @task_type: Task management function type.
// @r2: Reserved.
// @msg_flg: Message flags.
// @r3: Reserved.
// @lun: Logical Unit Number.
// @r4: Reserved.
// @task_mid: Task identifier.
// @r5: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scsi_tm_req {
    pub dev_hdl: __le16,
    pub chain_offset: u8,
    pub func: u8,
    pub r1: u8,
    pub task_type: u8,
    pub r2: u8,
    pub msg_flg: u8,
    pub r3: [u8; 4],
    pub lun: [u8; 8],
    pub r4: [u8; 28],
    pub task_mid: __le16,
    pub r5: [u8; 2],
}

//
// struct leapraid_scsi_tm_rep - SCSI Task Management response
//
// @dev_hdl: Device handle.
// @msg_len: Length of response message.
// @func: Function code.
// @resp_code: Response code.
// @task_type: Task management type.
// @r1: Reserved.
// @msg_flag: Message flags.
// @r2: Reserved.
// @adapter_status: Adapter status.
// @r3: Reserved.
// @termination_count: Count of terminated tasks.
// @response_info: Additional response info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scsi_tm_rep {
    pub dev_hdl: __le16,
    pub msg_len: u8,
    pub func: u8,
    pub resp_code: u8,
    pub task_type: u8,
    pub r1: u8,
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub adapter_status: __le16,
    pub r3: [u8; 4],
    pub termination_count: __le32,
    pub response_info: __le32,
}

//
// struct leapraid_sep_req - SEP (SCSI Enclosure Processor) request
//
// @dev_hdl: Device handle.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @act: Action to perform.
// @flg: Flags.
// @r1: Reserved.
// @msg_flag: Message flags.
// @r2: Reserved.
// @slot_status: Slot status.
// @r3: Reserved.
// @slot: Slot number.
// @enc_hdl: Enclosure handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sep_req {
    pub dev_hdl: __le16,
    pub chain_offset: u8,
    pub func: u8,
    pub act: u8,
    pub flg: u8,
    pub r1: u8,
    pub msg_flag: u8,
    pub r2: [u8; 4],
    pub slot_status: __le32,
    pub r3: [u8; 12],
    pub slot: __le16,
    pub enc_hdl: __le16,
}

//
// struct leapraid_sep_rep - SEP response
//
// @dev_hdl: Device handle.
// @msg_len: Message length.
// @func: Function code.
// @act: Action performed.
// @flg: Flags.
// @msg_flag: Message flags.
// @r1: Reserved.
// @adapter_status: Adapter status.
// @r2: Reserved.
// @slot_status: Slot status.
// @r3: Reserved.
// @slot: Slot number.
// @enc_hdl: Enclosure handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sep_rep {
    pub dev_hdl: __le16,
    pub msg_len: u8,
    pub func: u8,
    pub act: u8,
    pub flg: u8,
    pub r1: u8,
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub adapter_status: __le16,
    pub r3: [u8; 4],
    pub slot_status: __le32,
    pub r4: [u8; 4],
    pub slot: __le16,
    pub enc_hdl: __le16,
}

//
// struct leapraid_adapter_init_req - Adapter initialization request
//
// @who_init: Initiator of the initialization.
// @r1: Reserved.
// @chain_offset: Chain offset.
// @func: Function code.
// @r2: Reserved.
// @msg_flg: Message flags.
// @r3: Reserved.
// @msg_ver: Message version.
// @header_ver: Header version.
// @host_buf_addr: Host buffer address (non adapter-ref).
// @r4: Reserved.
// @host_buf_size: Host buffer size (non adapter-ref).
// @host_msix_vectors: Number of host MSI-X vectors.
// @r6: Reserved.
// @req_frame_size: Request frame size.
// @rep_desc_qd: Reply descriptor queue depth.
// @rep_msg_qd: Reply message queue depth.
// @sense_buffer_add_high: High 32-bit of sense buffer address.
// @rep_msg_dma_high: High 32-bit of reply message DMA address.
// @task_desc_base_addr: Base address of task descriptors.
// @rep_desc_q_arr_addr: Address of reply descriptor queue array.
// @rep_msg_addr_dma: Reply message DMA address.
// @time_stamp: Timestamp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_init_req {
    pub who_init: u8,
    pub r1: u8,
    pub chain_offset: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flg: u8,
    pub driver_ver: __le32,
    pub msg_ver: __le16,
    pub header_ver: __le16,
    pub host_buf_addr: __le32,
    pub r4: [u8; 2],
    pub host_buf_size: u8,
    pub host_msix_vectors: u8,
    pub r6: [u8; 2],
    pub req_frame_size: __le16,
    pub rep_desc_qd: __le16,
    pub rep_msg_qd: __le16,
    pub sense_buffer_add_high: __le32,
    pub rep_msg_dma_high: __le32,
    pub task_desc_base_addr: __le64,
    pub rep_desc_q_arr_addr: __le64,
    pub rep_msg_addr_dma: __le64,
    pub time_stamp: __le64,
    pub __aligned(4): } __packed,
//
// struct leapraid_rep_desc_q_arr - Reply descriptor queue array
//
// @rep_desc_base_addr: Base address of the reply descriptors.
// @r1: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_rep_desc_q_arr {
    pub rep_desc_base_addr: __le64,
    pub r1: __le64,
    pub __aligned(4): } __packed,
//
// struct leapraid_adapter_init_rep - Adapter initialization reply
//
// @who_init: Initiator of the initialization.
// @r1: Reserved.
// @msg_len: Length of reply message.
// @func: Function code.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
// @adapter_status: Adapter status.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_init_rep {
    pub who_init: u8,
    pub r1: u8,
    pub msg_len: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flag: u8,
    pub r3: [u8; 6],
    pub adapter_status: __le16,
    pub r4: [u8; 4],
}

//
// struct leapraid_adapter_log_req - Adapter log request
//
// @action: Action code.
// @type: Log type.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// r1: Reserved.
// @msg_flag: Message flags.
// r2: Reserved.
// @mbox: Mailbox for command-specific parameters.
// @sge: Scatter-gather entry for data buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_log_req {
    pub action: u8,
    pub type: u8,
    pub chain_offset: u8,
    pub func: u8,
    pub r1: [u8; 3],
    pub msg_flag: u8,
    pub r2: [u8; 4],
    pub b: [u8; 12],
    pub s: [__le16; 6],
    pub w: [__le32; 3],
    pub mbox: },
    pub sge: leapraid_sge_simple64,
    pub __aligned(4): } __packed,
//
// struct leapraid_adapter_log_rep - Adapter log reply
//
// @action: Action code echoed.
// @type: Log type echoed.
// @msg_len: Length of message.
// @func: Function code.
// @r1: Reserved.
// @msg_flag: Message flags.
// @r2: Reserved.
// @adapter_status: Status returned by adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_log_rep {
    pub action: u8,
    pub type: u8,
    pub msg_len: u8,
    pub func: u8,
    pub r1: [u8; 3],
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub adapter_status: __le16,
}

//
// struct leapraid_adapter_features_req - Request adapter features
//
// @r1: Reserved.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_features_req {
    pub r1: [u8; 2],
    pub chain_offset: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flag: u8,
    pub r3: [u8; 4],
}

//
// struct leapraid_adapter_features_rep - Adapter features reply
//
// @msg_ver: Message version.
// @msg_len: Length of reply message.
// @func: Function code.
// @header_ver: Header version.
// @r1: Reserved.
// @msg_flag: Message flags.
// @r2: Reserved.
// @adapter_status: Adapter status.
// @r3: Reserved.
// @who_init: Who initialized the adapter.
// @r4: Reserved.
// @max_msix_vectors: Max MSI-X vectors supported.
// @req_slot: Number of request slots.
// @r5: Reserved.
// @adapter_caps: Adapter capabilities.
// @fw_version: Firmware version.
// @sas_wide_max_qdepth: Max wide SAS queue depth.
// @sas_narrow_max_qdepth: Max narrow SAS queue depth.
// @r6: Reserved.
// @hp_slot: Number of high-priority slots.
// @r7: Reserved.
// @max_volumes: Maximum supported volumes.
// @max_dev_hdl: Maximum device handle.
// @r8: Reserved.
// @min_dev_hdl: Minimum device handle.
// @r9: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_adapter_features_rep {
    pub msg_ver: __le16,
    pub msg_len: u8,
    pub func: u8,
    pub header_ver: u16,
    pub r1: u8,
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub adapter_status: u16,
    pub r3: [u8; 4],
    pub sata_max_qdepth: u8,
    pub who_init: u8,
    pub r4: u8,
    pub max_msix_vectors: u8,
    pub req_slot: __le16,
    pub product_id: __le16,
    pub adapter_caps: __le32,
    pub fw_version: __le32,
    pub sas_wide_max_qdepth: __le16,
    pub sas_narrow_max_qdepth: __le16,
    pub r6: [u8; 10],
    pub hp_slot: __le16,
    pub r7: [u8; 3],
    pub max_volumes: u8,
    pub max_dev_hdl: __le16,
    pub r8: [u8; 2],
    pub min_dev_hdl: __le16,
    pub r9: [u8; 6],
}

//
// struct leapraid_scan_dev_req - Request to scan devices
//
// @r1: Reserved.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scan_dev_req {
    pub r1: [u8; 2],
    pub chain_offset: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flag: u8,
    pub r3: [u8; 4],
}

//
// struct leapraid_scan_dev_rep - Scan devices reply
//
// @r1: Reserved.
// @msg_len: Length of message.
// @func: Function code.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
// @adapter_status: Adapter status.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_scan_dev_rep {
    pub r1: [u8; 2],
    pub msg_len: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flag: u8,
    pub r3: [u8; 6],
    pub adapter_status: __le16,
    pub r4: [u8; 4],
}

//
// struct leapraid_evt_notify_req - Event notification request
//
// @r1: Reserved.
// @chain_offset: Offset for chained SGE.
// @func: Function code.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
// @evt_masks: Event masks to enable notifications.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_notify_req {
    pub r1: [u8; 2],
    pub chain_offset: u8,
    pub func: u8,
    pub r2: [u8; 3],
    pub msg_flag: u8,
    pub r3: [u8; 12],
    pub evt_masks: [__le32; 4],
    pub r4: [u8; 8],
}

//
// struct leapraid_evt_notify_rep - Event notification reply
//
// @evt_data_len: Length of event data.
// @msg_len: Length of message.
// @func: Function code.
// @r1: Reserved.
// @r2: Reserved.
// @msg_flag: Message flags.
// @r3: Reserved.
// @adapter_status: Adapter status.
// @r4: Reserved.
// @evt: Event code.
// @r5: Reserved.
// @evt_data: Event data array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_notify_rep {
    pub evt_data_len: __le16,
    pub msg_len: u8,
    pub func: u8,
    pub r1: [u8; 2],
    pub r2: u8,
    pub msg_flag: u8,
    pub r3: [u8; 6],
    pub adapter_status: __le16,
    pub r4: [u8; 4],
    pub evt: __le16,
    pub r5: [u8; 6],
    pub evt_data: [__le32; ],
}

//
// struct leapraid_evt_data_sas_dev_status_change - SAS device status change
//
// @task_tag: Task identifier.
// @reason_code: Reason for status change.
// @physical_port: Physical port number.
// @r1: Reserved.
// @dev_hdl: Device handle.
// @r2: Reserved.
// @sas_address: SAS address of device.
// @lun: Logical unit Number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_data_sas_dev_status_change {
    pub task_tag: __le16,
    pub reason_code: u8,
    pub physical_port: u8,
    pub r1: [u8; 2],
    pub dev_hdl: __le16,
    pub r2: [u8; 4],
    pub sas_address: __le64,
    pub lun: [u8; 8],
    pub __aligned(4): } __packed,
//
// struct leapraid_evt_data_ir_change - IR (Integrated RAID) change event data
//
// @r1: Reserved.
// @reason_code: Reason for IR change.
// @r2: Reserved.
// @vol_dev_hdl: Volume device handle.
// @phys_disk_dev_hdl: Physical disk device handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_data_ir_change {
    pub r1: u8,
    pub reason_code: u8,
    pub r2: [u8; 2],
    pub vol_dev_hdl: __le16,
    pub phys_disk_dev_hdl: __le16,
}

//
// struct leapraid_evt_data_sas_disc - SAS discovery event data
//
// @r1: Reserved.
// @reason_code: Reason for discovery event.
// @physical_port: Physical port number where event occurred.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_data_sas_disc {
    pub r1: u8,
    pub reason_code: u8,
    pub physical_port: u8,
    pub r2: [u8; 5],
}

//
// struct leapraid_evt_sas_topo_phy_entry - SAS topology PHY entry
//
// @attached_dev_hdl: Device handle attached to PHY.
// @link_rate: Current link rate.
// @phy_status: PHY status flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_sas_topo_phy_entry {
    pub attached_dev_hdl: __le16,
    pub link_rate: u8,
    pub phy_status: u8,
}

//
// struct leapraid_evt_data_sas_topo_change_list - SAS topology change list
//
// @encl_hdl: Enclosure handle.
// @exp_dev_hdl: Expander device handle.
// @num_phys: Number of PHYs in this entry.
// @r1: Reserved.
// @entry_num: Number of PHY elements.
// @start_phy_num: Start PHY number.
// @exp_status: Expander status.
// @physical_port: Physical port number.
// @phy: Array of SAS PHY entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_data_sas_topo_change_list {
    pub encl_hdl: __le16,
    pub exp_dev_hdl: __le16,
    pub num_phys: u8,
    pub r1: [u8; 3],
    pub entry_num: u8,
    pub start_phy_num: u8,
    pub exp_status: u8,
    pub physical_port: u8,
    pub phy: [leapraid_evt_sas_topo_phy_entry; ],
}

//
// struct leapraid_evt_data_sas_enc_dev_status_change -
// SAS enclosure device status
//
// @enc_hdl: Enclosure handle.
// @reason_code: Reason code for status change.
// @physical_port: Physical port number.
// @encl_logical_id: Enclosure logical ID.
// @num_slots: Number of slots in enclosure.
// @start_slot: First affected slot.
// @phy_bits: Bitmap of affected PHYs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_evt_data_sas_enc_dev_status_change {
    pub enc_hdl: __le16,
    pub reason_code: u8,
    pub physical_port: u8,
    pub encl_logical_id: __le64,
    pub num_slots: __le16,
    pub start_slot: __le16,
    pub phy_bits: __le32,
    pub __aligned(4): } __packed,
//
// struct leapraid_io_unit_ctrl_req - I/O unit control request
//
// @op: Operation code.
// @r1: Reserved.
// @chain_offset: SGE chain offset.
// @func: Function code.
// @dev_hdl: Device handle.
// @adapter_para: Adapter parameter selector.
// @msg_flag: Message flags.
// @r2: Reserved.
// @phy_num: PHY number.
// @r3: Reserved.
// @adapter_para_value: Value for adapter parameter.
// @adapter_para_value2: Optional second parameter value.
// @r4: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_io_unit_ctrl_req {
    pub op: u8,
    pub r1: u8,
    pub chain_offset: u8,
    pub func: u8,
    pub dev_hdl: __le16,
    pub adapter_para: u8,
    pub msg_flag: u8,
    pub r2: [u8; 6],
    pub phy_num: u8,
    pub r3: [u8; 17],
    pub adapter_para_value: __le32,
    pub adapter_para_value2: __le32,
    pub r4: [u8; 4],
}

//
// struct leapraid_io_unit_ctrl_rep - I/O unit control reply
//
// @op: Operation code echoed.
// @r1: Reserved.
// @func: Function code.
// @dev_hdl: Device handle.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_io_unit_ctrl_rep {
    pub op: u8,
    pub r1: [u8; 2],
    pub func: u8,
    pub dev_hdl: __le16,
    pub r2: [u8; 14],
}

//
// struct leapraid_raid_act_req - RAID action request
//
// @act: RAID action code.
// @r1: Reserved.
// @func: Function code.
// @r2: Reserved.
// @phys_disk_num: Number of physical disks involved.
// @r3: Reserved.
// @action_data_sge: SGE describing action-specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raid_act_req {
    pub act: u8,
    pub r1: [u8; 2],
    pub func: u8,
    pub r2: [u8; 2],
    pub phys_disk_num: u8,
    pub r3: [u8; 13],
    pub action_data_sge: leapraid_sge_simple_union,
}

//
// struct leapraid_raid_act_rep - RAID action reply
//
// @act: RAID action code echoed.
// @r1: Reserved.
// @func: Function code.
// @vol_dev_hdl: Volume device handle.
// @r2: Reserved
// @adapter_status: Status returned by adapter.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_raid_act_rep {
    pub act: u8,
    pub r1: [u8; 2],
    pub func: u8,
    pub vol_dev_hdl: __le16,
    pub r2: [u8; 8],
    pub adapter_status: __le16,
    pub r3: [u8; 76],
}

//
// struct leapraid_smp_passthrough_req - SMP passthrough request
//
// @passthrough_flg: Passthrough flags.
// @physical_port: Target PHY port.
// @r1: Reserved.
// @func: Function code.
// @req_data_len: Request data length.
// @r2: Reserved.
// @sas_address: SAS address of target device.
// @r3: Reserved.
// @sgl: Scatter-gather list describing request buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_smp_passthrough_req {
    pub passthrough_flg: u8,
    pub physical_port: u8,
    pub r1: u8,
    pub func: u8,
    pub req_data_len: __le16,
    pub r2: [u8; 10],
    pub sas_address: __le64,
    pub r3: [u8; 8],
    pub sgl: leapraid_simple_sge_union,
    pub __aligned(4): } __packed,
//
// struct leapraid_smp_passthrough_rep - SMP passthrough reply
//
// @passthrough_flg: Passthrough flags echoed.
// @physical_port: Target PHY port.
// @r1: Reserved.
// @func: Function code.
// @resp_data_len: Length of response data.
// @r2: Reserved.
// @adapter_status: Adapter status.
// @r3: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_smp_passthrough_rep {
    pub passthrough_flg: u8,
    pub physical_port: u8,
    pub r1: u8,
    pub func: u8,
    pub resp_data_len: __le16,
    pub r2: [u8; 8],
    pub adapter_status: __le16,
    pub r3: [u8; 12],
}

//
// struct leapraid_sas_io_unit_ctrl_req - SAS I/O unit control request
//
// @op: Operation code.
// @r1: Reserved.
// @func: Function code.
// @dev_hdl: Device handle.
// @r2: Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leapraid_sas_io_unit_ctrl_req {
    pub op: u8,
    pub r1: [u8; 2],
    pub func: u8,
    pub dev_hdl: __le16,
    pub r2: [u8; 38],
}
