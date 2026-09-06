//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla1280.h
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
// QLOGIC LINUX SOFTWARE
//
// QLogic ISP1280 (Ultra2) /12160 (Ultra3) SCSI driver
// Copyright (C) 2000 Qlogic Corporation
// (www.qlogic.com)
//
// Data bit definitions.
//
pub const BIT_0: c_uint = 0x1;
pub const BIT_1: c_uint = 0x2;
pub const BIT_2: c_uint = 0x4;
pub const BIT_3: c_uint = 0x8;
pub const BIT_4: c_uint = 0x10;
pub const BIT_5: c_uint = 0x20;
pub const BIT_6: c_uint = 0x40;
pub const BIT_7: c_uint = 0x80;
pub const BIT_8: c_uint = 0x100;
pub const BIT_9: c_uint = 0x200;
pub const BIT_10: c_uint = 0x400;
pub const BIT_11: c_uint = 0x800;
pub const BIT_12: c_uint = 0x1000;
pub const BIT_13: c_uint = 0x2000;
pub const BIT_14: c_uint = 0x4000;
pub const BIT_15: c_uint = 0x8000;
pub const BIT_16: c_uint = 0x10000;
pub const BIT_17: c_uint = 0x20000;
pub const BIT_18: c_uint = 0x40000;
pub const BIT_19: c_uint = 0x80000;
pub const BIT_20: c_uint = 0x100000;
pub const BIT_21: c_uint = 0x200000;
pub const BIT_22: c_uint = 0x400000;
pub const BIT_23: c_uint = 0x800000;
pub const BIT_24: c_uint = 0x1000000;
pub const BIT_25: c_uint = 0x2000000;
pub const BIT_26: c_uint = 0x4000000;
pub const BIT_27: c_uint = 0x8000000;
pub const BIT_28: c_uint = 0x10000000;
pub const BIT_29: c_uint = 0x20000000;
pub const BIT_30: c_uint = 0x40000000;
pub const BIT_31: c_uint = 0x80000000;

//
// Host adapter default definitions.
//

pub const MAX_B_BITS: c_int = 1;

//
// Watchdog time quantum
//

// Command retry count (0-65535)
pub const COMMAND_RETRY_COUNT: c_int = 255;
// Maximum outstanding commands in ISP queues
pub const MAX_OUTSTANDING_COMMANDS: c_int = 512;

// ISP request and response entry counts (37-65535)

//
// SCSI Request Block structure (sp) that occurs after each struct scsi_cmnd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srb {
    pub /: *mut *mut list_head list; / (8/16) LU queue,
    pub /: *mut *mut *mut scsi_cmnd cmd; / (4/8) SCSI command block,
// NOTE: the sp->cmd will be NULL when this completion is
// called, so you should know the scsi_cmnd when using this
    pub wait: *mut completion,
    pub /: *mut *mut dma_addr_t saved_dma_handle; / for unmap of single transfers,
    pub /: *mut *mut uint8_t flags; / (1) Status flags.,
    pub /: *mut *mut uint8_t dir; / direction of transfer,
}

//
// SRB flag definitions
//

//
// ISP I/O Register Set structure definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg {
    pub /: *mut *mut uint16_t id_l; / ID low,
    pub /: *mut *mut uint16_t id_h; / ID high,
    pub /: *mut *mut uint16_t cfg_0; / Configuration 0,
pub const ISP_CFG0_HWMSK: c_uint = 0x000f	/* Hardware revision mask */;

    pub /: *mut *mut uint16_t cfg_1; / Configuration 1,

    pub /: *mut *mut uint16_t ictrl; / Interface control,

    pub /: *mut *mut uint16_t istatus; / Interface status,

    pub /: *mut *mut uint16_t semaphore; / Semaphore,
    pub /: *mut *mut uint16_t nvram; / NVRAM register.,
pub const NV_DESELECT: c_int = 0;

    pub /: *mut *mut uint16_t flash_data; / Flash BIOS data,
    pub /: *mut *mut uint16_t flash_address; / Flash BIOS address,
    pub unused_1: [u16; 0x06],
// cdma_* and ddma_* are 1040 only
    pub cdma_cfg: u16,

    pub cdma_ctrl: u16,
    pub cdma_status: u16,
    pub cdma_fifo_status: u16,
    pub cdma_count: u16,
    pub cdma_reserved: u16,
    pub cdma_address_count_0: u16,
    pub cdma_address_count_1: u16,
    pub cdma_address_count_2: u16,
    pub cdma_address_count_3: u16,
    pub unused_2: [u16; 0x06],
    pub ddma_cfg: u16,

    pub ddma_ctrl: u16,
    pub ddma_status: u16,
    pub ddma_fifo_status: u16,
    pub ddma_xfer_count_low: u16,
    pub ddma_xfer_count_high: u16,
    pub ddma_addr_count_0: u16,
    pub ddma_addr_count_1: u16,
    pub ddma_addr_count_2: u16,
    pub ddma_addr_count_3: u16,
    pub unused_3: [u16; 0x0e],
    pub /: *mut *mut uint16_t mailbox0; / Mailbox 0,
    pub /: *mut *mut uint16_t mailbox1; / Mailbox 1,
    pub /: *mut *mut uint16_t mailbox2; / Mailbox 2,
    pub /: *mut *mut uint16_t mailbox3; / Mailbox 3,
    pub /: *mut *mut uint16_t mailbox4; / Mailbox 4,
    pub /: *mut *mut uint16_t mailbox5; / Mailbox 5,
    pub /: *mut *mut uint16_t mailbox6; / Mailbox 6,
    pub /: *mut *mut uint16_t mailbox7; / Mailbox 7,
    pub /: *mut *mut uint16_t unused_4[0x20];/ 0x80-0xbf Gap,
    pub /: *mut *mut uint16_t host_cmd; / Host command and control,

    pub /: *mut *mut uint16_t unused_5[0x5]; / 0xc2-0xcb Gap,
    pub gpio_data: u16,
    pub gpio_enable: u16,
    pub /: *mut *mut uint16_t unused_6[0x11]; / d0-f0,
    pub /: *mut *mut uint16_t scsiControlPins; / f2,
}

pub const MAILBOX_REGISTER_COUNT: c_int = 8;
//
// ISP product identification definitions in mailboxes after reset.
//
pub const PROD_ID_1: c_uint = 0x4953;
pub const PROD_ID_2: c_uint = 0x0000;
pub const PROD_ID_2a: c_uint = 0x5020;
pub const PROD_ID_3: c_uint = 0x2020;
pub const PROD_ID_4: c_uint = 0x1;
//
// ISP host command and control register command definitions
//
pub const HC_RESET_RISC: c_uint = 0x1000	/* Reset RISC */;
pub const HC_PAUSE_RISC: c_uint = 0x2000	/* Pause RISC */;
pub const HC_RELEASE_RISC: c_uint = 0x3000	/* Release RISC from reset. */;
pub const HC_SET_HOST_INT: c_uint = 0x5000	/* Set host interrupt */;
pub const HC_CLR_HOST_INT: c_uint = 0x6000	/* Clear HOST interrupt */;
pub const HC_CLR_RISC_INT: c_uint = 0x7000	/* Clear RISC interrupt */;
pub const HC_DISABLE_BIOS: c_uint = 0x9000	/* Disable BIOS. */;
//
// ISP mailbox Self-Test status codes
//

//
// ISP mailbox command complete status codes
//
pub const MBS_CMD_CMP: c_uint = 0x4000	/* Command Complete. */;
pub const MBS_INV_CMD: c_uint = 0x4001	/* Invalid Command. */;
pub const MBS_HOST_INF_ERR: c_uint = 0x4002	/* Host Interface Error. */;
pub const MBS_TEST_FAILED: c_uint = 0x4003	/* Test Failed. */;
pub const MBS_CMD_ERR: c_uint = 0x4005	/* Command Error. */;
pub const MBS_CMD_PARAM_ERR: c_uint = 0x4006	/* Command Parameter Error. */;
//
// ISP mailbox asynchronous event status codes
//
pub const MBA_ASYNC_EVENT: c_uint = 0x8000	/* Asynchronous event. */;
pub const MBA_BUS_RESET: c_uint = 0x8001	/* SCSI Bus Reset. */;
pub const MBA_SYSTEM_ERR: c_uint = 0x8002	/* System Error. */;
pub const MBA_REQ_TRANSFER_ERR: c_uint = 0x8003	/* Request Transfer Error. */;
pub const MBA_RSP_TRANSFER_ERR: c_uint = 0x8004	/* Response Transfer Error. */;
pub const MBA_WAKEUP_THRES: c_uint = 0x8005	/* Request Queue Wake-up. */;
pub const MBA_TIMEOUT_RESET: c_uint = 0x8006	/* Execution Timeout Reset. */;
pub const MBA_DEVICE_RESET: c_uint = 0x8007	/* Bus Device Reset. */;
pub const MBA_BUS_MODE_CHANGE: c_uint = 0x800E	/* SCSI bus mode transition. */;
pub const MBA_SCSI_COMPLETION: c_uint = 0x8020	/* Completion response. */;
//
// ISP mailbox commands
//

pub const MBC_DUMP_RAM_A64_ROM: c_uint = 0x0a	/* Dump RAM 64bit ROM version */;
pub const MBC_INIT_REQUEST_QUEUE: c_uint = 0x10	/* Initialize request queue */;
pub const MBC_INIT_RESPONSE_QUEUE: c_uint = 0x11	/* Initialize response queue */;
pub const MBC_EXECUTE_IOCB: c_uint = 0x12	/* Execute IOCB command */;
pub const MBC_ABORT_COMMAND: c_uint = 0x15	/* Abort IOCB command */;
pub const MBC_ABORT_DEVICE: c_uint = 0x16	/* Abort device (ID/LUN) */;
pub const MBC_ABORT_TARGET: c_uint = 0x17	/* Abort target (ID) */;
pub const MBC_BUS_RESET: c_uint = 0x18	/* SCSI bus reset */;
pub const MBC_GET_RETRY_COUNT: c_uint = 0x22	/* Get retry count and delay */;
pub const MBC_GET_TARGET_PARAMETERS: c_uint = 0x28	/* Get target parameters */;
pub const MBC_SET_INITIATOR_ID: c_uint = 0x30	/* Set initiator SCSI ID */;
pub const MBC_SET_SELECTION_TIMEOUT: c_uint = 0x31	/* Set selection timeout */;
pub const MBC_SET_RETRY_COUNT: c_uint = 0x32	/* Set retry count and delay */;
pub const MBC_SET_TAG_AGE_LIMIT: c_uint = 0x33	/* Set tag age limit */;
pub const MBC_SET_CLOCK_RATE: c_uint = 0x34	/* Set clock rate */;
pub const MBC_SET_ACTIVE_NEGATION: c_uint = 0x35	/* Set active negation state */;
pub const MBC_SET_ASYNC_DATA_SETUP: c_uint = 0x36	/* Set async data setup time */;
pub const MBC_SET_PCI_CONTROL: c_uint = 0x37	/* Set BUS control parameters */;
pub const MBC_SET_TARGET_PARAMETERS: c_uint = 0x38	/* Set target parameters */;
pub const MBC_SET_DEVICE_QUEUE: c_uint = 0x39	/* Set device queue parameters */;
pub const MBC_SET_RESET_DELAY_PARAMETERS: c_uint = 0x3A	/* Set reset delay parameters */;
pub const MBC_SET_SYSTEM_PARAMETER: c_uint = 0x45	/* Set system parameter word */;
pub const MBC_SET_FIRMWARE_FEATURES: c_uint = 0x4A	/* Set firmware feature word */;
pub const MBC_INIT_REQUEST_QUEUE_A64: c_uint = 0x52	/* Initialize request queue A64 */;
pub const MBC_INIT_RESPONSE_QUEUE_A64: c_uint = 0x53	/* Initialize response q A64 */;
pub const MBC_ENABLE_TARGET_MODE: c_uint = 0x55	/* Enable target mode */;
pub const MBC_SET_DATA_OVERRUN_RECOVERY: c_uint = 0x5A	/* Set data overrun recovery mode */;
//
// ISP Get/Set Target Parameters mailbox command control flags.
//

//
// NVRAM Command values.
//

pub const NV_DELAY_COUNT: c_int = 10;
//
// QLogic ISP1280/ISP12160 NVRAM structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram {
    pub /: *mut *mut uint8_t id0; / 0,
    pub /: *mut *mut uint8_t id1; / 1,
    pub /: *mut *mut uint8_t id2; / 2,
    pub /: *mut *mut uint8_t id3; / 3,
    pub /: *mut *mut uint8_t version; / 4,
    pub bios_configuration_mode:2: u8,
    pub bios_disable:1: u8,
    pub selectable_scsi_boot_enable:1: u8,
    pub cd_rom_boot_enable:1: u8,
    pub disable_loading_risc_code:1: u8,
    pub enable_64bit_addressing:1: u8,
    pub unused_7:1: u8,
    pub /: *mut *mut } cntr_flags_1; / 5,
    pub boot_lun_number:5: u8,
    pub scsi_bus_number:1: u8,
    pub unused_6:1: u8,
    pub unused_7:1: u8,
    pub /: *mut *mut } cntr_flags_2l; / 7,
    pub boot_target_number:4: u8,
    pub unused_12:1: u8,
    pub unused_13:1: u8,
    pub unused_14:1: u8,
    pub unused_15:1: u8,
    pub /: *mut *mut } cntr_flags_2h; / 8,
    pub /: *mut *mut uint16_t unused_8; / 8, 9,
    pub /: *mut *mut uint16_t unused_10; / 10, 11,
    pub /: *mut *mut uint16_t unused_12; / 12, 13,
    pub /: *mut *mut uint16_t unused_14; / 14, 15,
    pub reserved:2: u8,
    pub burst_enable:1: u8,
    pub reserved_1:1: u8,
    pub fifo_threshold:4: u8,
    pub /: *mut *mut } isp_config; / 16,
// Termination
// 0 = Disable, 1 = high only, 3 = Auto term
//
    pub scsi_bus_1_control:2: u8,
    pub scsi_bus_0_control:2: u8,
    pub unused_0:1: u8,
    pub unused_1:1: u8,
    pub unused_2:1: u8,
    pub auto_term_support:1: u8,
    pub /: *mut *mut } termination; / 17,
    pub /: *mut *mut uint16_t isp_parameter; / 18, 19,
    pub w: u16,
    pub enable_fast_posting:1: u16,
    pub report_lvd_bus_transition:1: u16,
    pub unused_2:1: u16,
    pub unused_3:1: u16,
    pub disable_iosbs_with_bus_reset_status:1: u16,
    pub disable_synchronous_backoff:1: u16,
    pub unused_6:1: u16,
    pub synchronous_backoff_reporting:1: u16,
    pub disable_reselection_fairness:1: u16,
    pub unused_9:1: u16,
    pub unused_10:1: u16,
    pub unused_11:1: u16,
    pub unused_12:1: u16,
    pub unused_13:1: u16,
    pub unused_14:1: u16,
    pub unused_15:1: u16,
    pub f: },
    pub /: *mut *mut } firmware_feature; / 20, 21,
    pub /: *mut *mut uint16_t unused_22; / 22, 23,
    pub initiator_id:4: u8,
    pub scsi_reset_disable:1: u8,
    pub scsi_bus_size:1: u8,
    pub scsi_bus_type:1: u8,
    pub unused_7:1: u8,
    pub /: *mut *mut } config_1; / 24,
    pub /: *mut *mut uint8_t bus_reset_delay; / 25,
    pub /: *mut *mut uint8_t retry_count; / 26,
    pub /: *mut *mut uint8_t retry_delay; / 27,
    pub async_data_setup_time:4: u8,
    pub req_ack_active_negation:1: u8,
    pub data_line_active_negation:1: u8,
    pub unused_6:1: u8,
    pub unused_7:1: u8,
    pub /: *mut *mut } config_2; / 28,
    pub /: *mut *mut uint8_t unused_29; / 29,
    pub /: *mut *mut uint16_t selection_timeout; / 30, 31,
    pub /: *mut *mut uint16_t max_queue_depth; / 32, 33,
    pub /: *mut *mut uint16_t unused_34; / 34, 35,
    pub /: *mut *mut uint16_t unused_36; / 36, 37,
    pub /: *mut *mut uint16_t unused_38; / 38, 39,
    pub renegotiate_on_error:1: u8,
    pub stop_queue_on_check:1: u8,
    pub auto_request_sense:1: u8,
    pub tag_queuing:1: u8,
    pub enable_sync:1: u8,
    pub enable_wide:1: u8,
    pub parity_checking:1: u8,
    pub disconnect_allowed:1: u8,
    pub /: *mut *mut } parameter; / 40,
    pub /: *mut *mut uint8_t execution_throttle; / 41,
    pub /: *mut *mut uint8_t sync_period; / 42,
    pub flags_43: u8,
    pub sync_offset:4: u8,
    pub device_enable:1: u8,
    pub lun_disable:1: u8,
    pub unused_6:1: u8,
    pub unused_7:1: u8,
    pub flags1x80: },
    pub sync_offset:5: u8,
    pub device_enable:1: u8,
    pub unused_6:1: u8,
    pub unused_7:1: u8,
    pub flags1x160: },
    pub flags: },
    pub unused_44: u8,
    pub ppr_options:4: u8,
    pub ppr_bus_width:2: u8,
    pub unused_8:1: u8,
    pub enable_ppr:1: u8,
    pub /: *mut *mut } flags; / 44,
    pub ppr_1x160: },
    pub /: *mut *mut uint8_t unused_45; / 45,
    pub target: [}; MAX_TARGETS],
    pub bus: [}; MAX_BUSES],
    pub /: *mut *mut uint16_t unused_248; / 248, 249,
    pub /: *mut *mut uint16_t subsystem_id[2]; / 250, 251, 252, 253,
    pub unused_254: u8,
    pub system_id_pointer: u8,
    pub sysid_1x160: },
    pub /: *mut *mut uint8_t chksum; / 255,
}

//
// ISP queue - command entry structure definition.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 handle; / System handle.,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub /: *mut *mut uint8_t target; / SCSI ID,
    pub /: *mut *mut __le16 cdb_len; / SCSI command length.,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub reserved: __le16,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut uint8_t scsi_cdb[MAX_CMDSZ]; / SCSI command words.,
    pub /: *mut *mut __le32 dseg_0_address; / Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address; / Data segment 1 address.,
    pub /: *mut *mut __le32 dseg_1_length; / Data segment 1 length.,
    pub /: *mut *mut __le32 dseg_2_address; / Data segment 2 address.,
    pub /: *mut *mut __le32 dseg_2_length; / Data segment 2 length.,
    pub /: *mut *mut __le32 dseg_3_address; / Data segment 3 address.,
    pub /: *mut *mut __le32 dseg_3_length; / Data segment 3 length.,
}

//
// ISP queue - continuation entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cont_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 reserved; / Reserved,
    pub /: *mut *mut __le32 dseg_0_address; / Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address; / Data segment 1 address.,
    pub /: *mut *mut __le32 dseg_1_length; / Data segment 1 length.,
    pub /: *mut *mut __le32 dseg_2_address; / Data segment 2 address.,
    pub /: *mut *mut __le32 dseg_2_length; / Data segment 2 length.,
    pub /: *mut *mut __le32 dseg_3_address; / Data segment 3 address.,
    pub /: *mut *mut __le32 dseg_3_length; / Data segment 3 length.,
    pub /: *mut *mut __le32 dseg_4_address; / Data segment 4 address.,
    pub /: *mut *mut __le32 dseg_4_length; / Data segment 4 length.,
    pub /: *mut *mut __le32 dseg_5_address; / Data segment 5 address.,
    pub /: *mut *mut __le32 dseg_5_length; / Data segment 5 length.,
    pub /: *mut *mut __le32 dseg_6_address; / Data segment 6 address.,
    pub /: *mut *mut __le32 dseg_6_length; / Data segment 6 length.,
}

//
// ISP queue - status entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct response {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,

    pub /: *mut *mut __le32 handle; / System handle.,
    pub /: *mut *mut __le16 scsi_status; / SCSI status.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 state_flags; / State flags.,

    pub /: *mut *mut __le16 status_flags; / Status flags.,
    pub /: *mut *mut __le16 time; / Time.,
    pub /: *mut *mut __le16 req_sense_length;/ Request sense data length.,
    pub /: *mut *mut __le32 residual_length; / Residual transfer length.,
    pub reserved: [__le16; 4],
    pub /: *mut *mut uint8_t req_sense_data[32]; / Request sense data.,
}

//
// ISP queue - marker entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrk_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub /: *mut *mut uint8_t target; / SCSI ID,
    pub /: *mut *mut uint8_t modifier; / Modifier (7-0).,
    pub reserved_1: [u8; 53],
}

//
// ISP queue - extended command entry structure definition.
//
// Unused by the driver!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecmd_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub /: *mut *mut uint8_t target; / SCSI ID,
    pub /: *mut *mut __le16 cdb_len; / SCSI command length.,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub reserved: __le16,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut uint8_t scsi_cdb[88]; / SCSI command words.,
}

//
// ISP queue - 64-Bit addressing, command entry structure definition.
//

//
// ISP queue - 64-Bit addressing, continuation entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cont_a64_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const CONTINUE_A64_TYPE: c_uint = 0xA	/* Continuation A64 entry. */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 dseg_0_address[2]; / Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address[2]; / Data segment 1 address.,
    pub /: *mut *mut __le32 dseg_1_length; / Data segment 1 length.,
    pub /: *mut *mut __le32 dseg_2_address[2]; / Data segment 2 address.,
    pub /: *mut *mut __le32 dseg_2_length; / Data segment 2 length.,
    pub /: *mut *mut __le32 dseg_3_address[2]; / Data segment 3 address.,
    pub /: *mut *mut __le32 dseg_3_length; / Data segment 3 length.,
    pub /: *mut *mut __le32 dseg_4_address[2]; / Data segment 4 address.,
    pub /: *mut *mut __le32 dseg_4_length; / Data segment 4 length.,
}

//
// ISP queue - enable LUN entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elun_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const ENABLE_LUN_TYPE: c_uint = 0xB	/* Enable LUN entry. */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status not used.,
    pub reserved_2: __le32,
    pub /: *mut *mut __le16 lun; / Bit 15 is bus number.,
    pub reserved_4: __le16,
    pub option_flags: __le32,
    pub status: u8,
    pub reserved_5: u8,
    pub /: *mut *mut uint8_t command_count; / Number of ATIOs allocated.,
    pub /: *mut *mut uint8_t immed_notify_count; / Number of Immediate Notify,
// entries allocated.
    pub /: *mut *mut uint8_t group_6_length; / SCSI CDB length for group 6,
// commands (2-26).
    pub /: *mut *mut uint8_t group_7_length; / SCSI CDB length for group 7,
// commands (2-26).
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub reserved_6: [__le16; 20],
}

//
// ISP queue - modify LUN entry structure definition.
//
// Unused by the driver!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modify_lun_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const MODIFY_LUN_TYPE: c_uint = 0xC	/* Modify LUN entry. */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub reserved_3: u8,
    pub operators: u8,
    pub reserved_4: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub reserved_5: u8,
    pub /: *mut *mut uint8_t command_count; / Number of ATIOs allocated.,
    pub /: *mut *mut uint8_t immed_notify_count; / Number of Immediate Notify,
// entries allocated.
    pub reserved_6: __le16,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub reserved_7: [__le16; 20],
}

//
// ISP queue - immediate notify entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notify_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const IMMED_NOTIFY_TYPE: c_uint = 0xD	/* Immediate notify entry. */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub lun: u8,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub reserved_4: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
// entries allocated.
    pub seq_id: __le16,
    pub /: *mut *mut uint8_t scsi_msg[8]; / SCSI message not handled by ISP,
    pub reserved_5: [__le16; 8],
    pub sense_data: [u8; 18],
}

//
// ISP queue - notify acknowledge entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nack_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const NOTIFY_ACK_TYPE: c_uint = 0xE	/* Notify acknowledge entry. */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub lun: u8,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub event: u8,
    pub seq_id: __le16,
    pub reserved_4: [__le16; 22],
}

//
// ISP queue - Accept Target I/O (ATIO) entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atio_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub lun: u8,
    pub initiator_id: u8,
    pub cdb_len: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub scsi_status: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
    pub cdb: [u8; 26],
    pub sense_data: [u8; 18],
}

//
// ISP queue - Continue Target I/O (CTIO) entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub scsi_status: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
    pub transfer_length: __le32,
    pub residual: __le32,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le32 dseg_0_address; / Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address; / Data segment 1 address.,
    pub /: *mut *mut __le32 dseg_1_length; / Data segment 1 length.,
    pub /: *mut *mut __le32 dseg_2_address; / Data segment 2 address.,
    pub /: *mut *mut __le32 dseg_2_length; / Data segment 2 length.,
    pub /: *mut *mut __le32 dseg_3_address; / Data segment 3 address.,
    pub /: *mut *mut __le32 dseg_3_length; / Data segment 3 length.,
}

//
// ISP queue - CTIO returned entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_ret_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,

    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub scsi_status: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
    pub transfer_length: __le32,
    pub residual: __le32,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le32 dseg_0_address; / Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address; / Data segment 1 address.,
    pub /: *mut *mut __le16 dseg_1_length; / Data segment 1 length.,
    pub sense_data: [u8; 18],
}

//
// ISP queue - CTIO A64 entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_a64_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const CTIO_A64_TYPE: c_uint = 0xF	/* CTIO A64 entry */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub scsi_status: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
    pub transfer_length: __le32,
    pub residual: __le32,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub reserved_4: [__le32; 2],
    pub /: *mut *mut __le32 dseg_0_address[2];/ Data segment 0 address.,
    pub /: *mut *mut __le32 dseg_0_length; / Data segment 0 length.,
    pub /: *mut *mut __le32 dseg_1_address[2];/ Data segment 1 address.,
    pub /: *mut *mut __le32 dseg_1_length; / Data segment 1 length.,
}

//
// ISP queue - CTIO returned entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_a64_ret_entry {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const CTIO_A64_RET_TYPE: c_uint = 0xF	/* CTIO A64 returned entry */;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub reserved_1: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_2: __le32,
    pub /: *mut *mut uint8_t lun; / SCSI LUN,
    pub initiator_id: u8,
    pub reserved_3: u8,
    pub target_id: u8,
    pub option_flags: __le32,
    pub status: u8,
    pub scsi_status: u8,
    pub /: *mut *mut uint8_t tag_value; / Received queue tag message value,
    pub /: *mut *mut uint8_t tag_type; / Received queue tag message type,
    pub transfer_length: __le32,
    pub residual: __le32,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub reserved_4: [__le16; 7],
    pub sense_data: [u8; 18],
}

//
// ISP request and response queue entry sizes
//

//
// ISP status entry - completion status definitions.
//
pub const CS_COMPLETE: c_uint = 0x0	/* No errors */;
pub const CS_INCOMPLETE: c_uint = 0x1	/* Incomplete transfer of cmd. */;
pub const CS_DMA: c_uint = 0x2	/* A DMA direction error. */;
pub const CS_TRANSPORT: c_uint = 0x3	/* Transport error. */;
pub const CS_RESET: c_uint = 0x4	/* SCSI bus reset occurred */;
pub const CS_ABORTED: c_uint = 0x5	/* System aborted command. */;
pub const CS_TIMEOUT: c_uint = 0x6	/* Timeout error. */;
pub const CS_DATA_OVERRUN: c_uint = 0x7	/* Data overrun. */;
pub const CS_COMMAND_OVERRUN: c_uint = 0x8	/* Command Overrun. */;
pub const CS_STATUS_OVERRUN: c_uint = 0x9	/* Status Overrun. */;
pub const CS_BAD_MSG: c_uint = 0xA	/* Bad msg after status phase. */;
pub const CS_NO_MSG_OUT: c_uint = 0xB	/* No msg out after selection. */;
pub const CS_EXTENDED_ID: c_uint = 0xC	/* Extended ID failed. */;
pub const CS_IDE_MSG: c_uint = 0xD	/* Target rejected IDE msg. */;
pub const CS_ABORT_MSG: c_uint = 0xE	/* Target rejected abort msg. */;
pub const CS_REJECT_MSG: c_uint = 0xF	/* Target rejected reject msg. */;
pub const CS_NOP_MSG: c_uint = 0x10	/* Target rejected NOP msg. */;
pub const CS_PARITY_MSG: c_uint = 0x11	/* Target rejected parity msg. */;
pub const CS_DEV_RESET_MSG: c_uint = 0x12	/* Target rejected dev rst msg. */;
pub const CS_ID_MSG: c_uint = 0x13	/* Target rejected ID msg. */;
pub const CS_FREE: c_uint = 0x14	/* Unexpected bus free. */;
pub const CS_DATA_UNDERRUN: c_uint = 0x15	/* Data Underrun. */;
pub const CS_TRANACTION_1: c_uint = 0x18	/* Transaction error 1 */;
pub const CS_TRANACTION_2: c_uint = 0x19	/* Transaction error 2 */;
pub const CS_TRANACTION_3: c_uint = 0x1a	/* Transaction error 3 */;
pub const CS_INV_ENTRY_TYPE: c_uint = 0x1b	/* Invalid entry type */;
pub const CS_DEV_QUEUE_FULL: c_uint = 0x1c	/* Device queue full */;
pub const CS_PHASED_SKIPPED: c_uint = 0x1d	/* SCSI phase skipped */;
pub const CS_ARS_FAILED: c_uint = 0x1e	/* ARS failed */;
pub const CS_LVD_BUS_ERROR: c_uint = 0x21	/* LVD bus error */;
pub const CS_BAD_PAYLOAD: c_uint = 0x80	/* Driver defined */;
pub const CS_UNKNOWN: c_uint = 0x81	/* Driver defined */;
pub const CS_RETRY: c_uint = 0x82	/* Driver defined */;
//
// ISP target entries - Option flags bit definitions.
//

// (data from target to initiator)

// (data from initiator to target)

//
// BUS parameters/settings structure - UNUSED
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_param {
    pub /: *mut *mut uint8_t id; / Host adapter SCSI id,
    pub /: *mut *mut uint8_t bus_reset_delay; / SCSI bus reset delay.,
    pub /: *mut *mut uint8_t failed_reset_count; / number of time reset failed,
    pub unused: u8,
    pub /: *mut *mut uint16_t device_enables; / Device enable bits.,
    pub /: *mut *mut uint16_t lun_disables; / LUN disable bits.,
    pub /: *mut *mut uint16_t qtag_enables; / Tag queue enables.,
    pub /: *mut *mut uint16_t hiwat; / High water mark per device.,
    pub reset_marker:1: u8,
    pub disable_scsi_reset:1: u8,
    pub /: *mut *mut uint8_t scsi_bus_dead:1; / SCSI Bus is Dead, when 5 back to back resets failed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_driver_setup {
    pub no_sync:1: u32,
    pub no_wide:1: u32,
    pub no_ppr:1: u32,
    pub no_nvram:1: u32,
    pub sync_mask: u16,
    pub wide_mask: u16,
    pub ppr_mask: u16,
}

//
// Linux Host Adapter structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_qla_host {
// Linux adapter configuration data
    pub /: *mut *mut *mut Scsi_Host host; / pointer to host data,
    pub next: *mut scsi_qla_host,
    pub /: *mut *mut *mut device_reg __iomem iobase; / Base Memory-mapped I/O address,
    pub /: *mut *mut *mut unsigned char __iomem mmpbase; / memory mapped address,
    pub host_no: c_ulong,
    pub pdev: *mut pci_dev,
    pub devnum: u8,
    pub revision: u8,
    pub ports: u8,
    pub actthreads: c_ulong,
    pub /: *mut *mut unsigned long isr_count; / Interrupt count,
    pub spurious_int: c_ulong,
// Outstandings ISP commands.
    pub outstanding_cmds: [*mut srb; MAX_OUTSTANDING_COMMANDS],
// BUS configuration data
    pub bus_settings: [bus_param; MAX_BUSES],
// Received ISP mailbox data.
    pub mailbox_out: [volatile uint16_t; MAILBOX_REGISTER_COUNT],
    pub /: *mut *mut dma_addr_t request_dma; / Physical Address,
    pub /: *mut *mut *mut request_t request_ring; / Base virtual address,
    pub /: *mut *mut *mut request_t request_ring_ptr; / Current address.,
    pub /: *mut *mut uint16_t req_ring_index; / Current index.,
    pub /: *mut *mut uint16_t req_q_cnt; / Number of available entries.,
    pub /: *mut *mut dma_addr_t response_dma; / Physical address.,
    pub /: *mut *mut *mut response response_ring; / Base virtual address,
    pub /: *mut *mut *mut response response_ring_ptr; / Current address.,
    pub /: *mut *mut uint16_t rsp_ring_index; / Current index.,
    pub /: *mut *mut list_head done_q; / Done queue,
    pub mailbox_wait: *mut completion,
    pub mailbox_timer: timer_list,
    pub /: *mut *mut uint32_t online:1; / 0,
    pub /: *mut *mut uint32_t reset_marker:1; / 1,
    pub /: *mut *mut uint32_t disable_host_adapter:1; / 2,
    pub /: *mut *mut uint32_t reset_active:1; / 3,
    pub /: *mut *mut uint32_t abort_isp_active:1; / 4,
    pub /: *mut *mut uint32_t disable_risc_code_load:1; / 5,
    pub flags: },
    pub nvram: nvram,
    pub nvram_valid: c_int,
// Firmware Info
    pub /: *mut *mut unsigned short fwstart; / start address for F/W,
    pub /: *mut *mut unsigned char fwver1; / F/W version first char,
    pub /: *mut *mut unsigned char fwver2; / F/W version second char,
    pub /: *mut *mut unsigned char fwver3; / F/W version third char,
}
