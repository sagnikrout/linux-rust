//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_fw.h
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
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//
pub const MAX_PRST_DEV_DB_ENTRIES: c_int = 64;

pub const MAX_DEV_DB_ENTRIES: c_int = 512;
pub const MAX_DEV_DB_ENTRIES_40XX: c_int = 256;
//
// ISP 4010 I/O Register Set Structure and Definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_ctrl_stat_regs {
    pub /: *mut *mut __le32 ext_hw_conf; / 0x50 R/W,
    pub /: *mut *mut __le32 rsrvd0; / 0x54,
    pub /: *mut *mut __le32 port_ctrl; / 0x58,
    pub /: *mut *mut __le32 port_status; / 0x5c,
    pub /: *mut *mut __le32 rsrvd1[32]; / 0x60-0xdf,
    pub /: *mut *mut __le32 gp_out; / 0xe0,
    pub /: *mut *mut __le32 gp_in; / 0xe4,
    pub /: *mut *mut __le32 rsrvd2[5]; / 0xe8-0xfb,
    pub /: *mut *mut __le32 port_err_status; / 0xfc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_mem_cfg_regs {
    pub /: *mut *mut __le32 rsrvd0[12]; / 0x50-0x79,
    pub /: *mut *mut __le32 req_q_out; / 0x80,
    pub /: *mut *mut __le32 rsrvd1[31]; / 0x84-0xFF,
}

//
// ISP 82xx I/O Register Set structure definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_82xx {
    pub /: *mut *mut __le32 req_q_out; / 0x0000 (R): Request Queue out-Pointer.,
    pub /: *mut *mut *mut __le32 reserve1[63]; / Request Queue out-Pointer. (64  4),
    pub /: *mut *mut __le32 rsp_q_in; / 0x0100 (R/W): Response Queue In-Pointer.,
    pub /: *mut *mut __le32 reserve2[63]; / Response Queue In-Pointer.,
    pub /: *mut *mut __le32 rsp_q_out; / 0x0200 (R/W): Response Queue Out-Pointer.,
    pub /: *mut *mut __le32 reserve3[63]; / Response Queue Out-Pointer.,
    pub /: *mut *mut __le32 mailbox_in[8]; / 0x0300 (R/W): Mail box In registers,
    pub reserve4: [__le32; 24],
    pub /: *mut *mut __le32 hint; / 0x0380 (R/W): Host interrupt register,
    pub reserve5: [__le32; 31],
    pub /: *mut *mut __le32 mailbox_out[8]; / 0x0400 (R): Mail box Out registers,
    pub reserve6: [__le32; 56],
    pub /: *mut *mut __le32 host_status; / Offset 0x500 (R): host status,

    pub /: *mut *mut __le32 host_int; / Offset 0x0504 (R/W): Interrupt status.,

}

// ISP 83xx I/O Register Set structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_83xx {
    pub /: *mut *mut __le32 mailbox_in[16]; / 0x0000,
    pub /: *mut *mut __le32 reserve1[496]; / 0x0040,
    pub /: *mut *mut __le32 mailbox_out[16]; / 0x0800,
    pub reserve2: [__le32; 496],
    pub /: *mut *mut __le32 mbox_int; / 0x1000,
    pub reserve3: [__le32; 63],
    pub /: *mut *mut __le32 req_q_out; / 0x1100,
    pub reserve4: [__le32; 63],
    pub /: *mut *mut __le32 rsp_q_in; / 0x1200,
    pub reserve5: [__le32; 1919],
    pub /: *mut *mut __le32 req_q_in; / 0x3000,
    pub reserve6: [__le32; 3],
    pub /: *mut *mut __le32 iocb_int_mask; / 0x3010,
    pub reserve7: [__le32; 3],
    pub /: *mut *mut __le32 rsp_q_out; / 0x3020,
    pub reserve8: [__le32; 3],
    pub /: *mut *mut __le32 anonymousbuff; / 0x3030,
    pub /: *mut *mut __le32 mb_int_mask; / 0x3034,
    pub /: *mut *mut __le32 host_intr; / 0x3038 - Host Interrupt Register,
    pub /: *mut *mut __le32 risc_intr; / 0x303C - RISC Interrupt Register,
    pub reserve9: [__le32; 544],
    pub /: *mut *mut __le32 leg_int_ptr; / 0x38C0 - Legacy Interrupt Pointer Register,
    pub /: *mut *mut __le32 leg_int_trig; / 0x38C4 - Legacy Interrupt Trigger Control,
    pub /: *mut *mut __le32 leg_int_mask; / 0x38C8 - Legacy Interrupt Mask Register,
}

// remote register set (access via PCI memory read/write)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_reg {
pub const MBOX_REG_COUNT: c_int = 8;
    pub mailbox: [__le32; MBOX_REG_COUNT],
    pub /: *mut *mut __le32 flash_address; / 0x20,
    pub flash_data: __le32,
    pub ctrl_status: __le32,
    pub nvram: __le32,
    pub /: *mut *mut __le32 reserved1[2]; / 0x30,
// C attribute field omitted
    pub intr_mask: __le32,
    pub /: *mut *mut __le32 nvram; / 0x30,
    pub semaphore: __le32,
// C attribute field omitted
    pub u1: },
    pub /: *mut *mut __le32 req_q_in; / SCSI Request Queue Producer Index,
    pub /: *mut *mut __le32 rsp_q_out; / SCSI Completion Queue Consumer Index,
    pub /: *mut *mut __le32 reserved2[4]; / 0x40,
    pub /: *mut *mut __le32 ext_hw_conf; / 0x50,
    pub flow_ctrl: __le32,
    pub port_ctrl: __le32,
    pub port_status: __le32,
    pub /: *mut *mut __le32 reserved3[8]; / 0x60,
    pub /: *mut *mut __le32 req_q_out; / 0x80,
    pub /: *mut *mut __le32 reserved4[23]; / 0x84,
    pub /: *mut *mut __le32 gp_out; / 0xe0,
    pub gp_in: __le32,
    pub reserved5: [__le32; 5],
    pub /: *mut *mut __le32 port_err_status; / 0xfc,
// C attribute field omitted
    pub p0: port_ctrl_stat_regs,
    pub p1: host_mem_cfg_regs,
}

// Semaphore Defines for 4010
pub const QL4010_DRVR_SEM_BITS: c_uint = 0x00000030;
pub const QL4010_GPIO_SEM_BITS: c_uint = 0x000000c0;
pub const QL4010_SDRAM_SEM_BITS: c_uint = 0x00000300;
pub const QL4010_PHY_SEM_BITS: c_uint = 0x00000c00;
pub const QL4010_NVRAM_SEM_BITS: c_uint = 0x00003000;
pub const QL4010_FLASH_SEM_BITS: c_uint = 0x0000c000;
pub const QL4010_DRVR_SEM_MASK: c_uint = 0x00300000;
pub const QL4010_GPIO_SEM_MASK: c_uint = 0x00c00000;
pub const QL4010_SDRAM_SEM_MASK: c_uint = 0x03000000;
pub const QL4010_PHY_SEM_MASK: c_uint = 0x0c000000;
pub const QL4010_NVRAM_SEM_MASK: c_uint = 0x30000000;
pub const QL4010_FLASH_SEM_MASK: c_uint = 0xc0000000;
// Semaphore Defines for 4022
pub const QL4022_RESOURCE_MASK_BASE_CODE: c_uint = 0x7;
pub const QL4022_RESOURCE_BITS_BASE_CODE: c_uint = 0x4;

// nvram address for 4032
pub const NVRAM_PORT0_BOOT_MODE: c_uint = 0x03b1;
pub const NVRAM_PORT0_BOOT_PRI_TGT: c_uint = 0x03b2;
pub const NVRAM_PORT0_BOOT_SEC_TGT: c_uint = 0x03bb;
pub const NVRAM_PORT1_BOOT_MODE: c_uint = 0x07b1;
pub const NVRAM_PORT1_BOOT_PRI_TGT: c_uint = 0x07b2;
pub const NVRAM_PORT1_BOOT_SEC_TGT: c_uint = 0x07bb;
// Page # defines for 4022

// Register Mask - sets corresponding mask bits in the upper word
// ctrl_status definitions
pub const CSR_SCSI_PAGE_SELECT: c_uint = 0x00000003;
pub const CSR_SCSI_INTR_ENABLE: c_uint = 0x00000004	/* 4010 */;
pub const CSR_SCSI_RESET_INTR: c_uint = 0x00000008;
pub const CSR_SCSI_COMPLETION_INTR: c_uint = 0x00000010;
pub const CSR_SCSI_PROCESSOR_INTR: c_uint = 0x00000020;
pub const CSR_INTR_RISC: c_uint = 0x00000040;
pub const CSR_BOOT_ENABLE: c_uint = 0x00000080;
pub const CSR_NET_PAGE_SELECT: c_uint = 0x00000300	/* 4010 */;
pub const CSR_FUNC_NUM: c_uint = 0x00000700	/* 4022 */;
pub const CSR_NET_RESET_INTR: c_uint = 0x00000800	/* 4010 */;
pub const CSR_FORCE_SOFT_RESET: c_uint = 0x00002000	/* 4022 */;
pub const CSR_FATAL_ERROR: c_uint = 0x00004000;
pub const CSR_SOFT_RESET: c_uint = 0x00008000;

pub const ISP_CONTROL_FN0_SCSI: c_uint = 0x0500;
pub const ISP_CONTROL_FN1_SCSI: c_uint = 0x0700;

// ISP InterruptMask definitions
pub const IMR_SCSI_INTR_ENABLE: c_uint = 0x00000004	/* 4022 */;
// ISP 4022 nvram definitions
pub const NVR_WRITE_ENABLE: c_uint = 0x00000010	/* 4022 */;
pub const QL4010_NVRAM_SIZE: c_uint = 0x200;
pub const QL40X2_NVRAM_SIZE: c_uint = 0x800;
// ISP port_status definitions
// ISP Semaphore definitions
// ISP General Purpose Output definitions
pub const GPOR_TOPCAT_RESET: c_uint = 0x00000004;
// shadow registers (DMA'd from HA to system memory.  read only)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_regs {
// SCSI Request Queue Consumer Index
    pub /: *mut *mut __le32 req_q_out; / 0 x0 R,
// SCSI Completion Queue Producer Index
    pub /: *mut *mut __le32 rsp_q_in; / 4 x4 R,
}

// External hardware configuration register
#[repr(C)]
#[derive(Copy, Clone)]
pub union external_hw_config_reg {
// FIXME: Do we even need this?	 All values are
// referred to by 16 bit quantities.  Platform and
// endianess issues.
    pub bReserved0:1: __le32,
    pub bSDRAMProtectionMethod:2: __le32,
    pub bSDRAMBanks:1: __le32,
    pub bSDRAMChipWidth:1: __le32,
    pub bSDRAMChipSize:2: __le32,
    pub bParityDisable:1: __le32,
    pub bExternalMemoryType:1: __le32,
    pub bFlashBIOSWriteEnable:1: __le32,
    pub bFlashUpperBankSelect:1: __le32,
    pub bWriteBurst:2: __le32,
    pub bReserved1:3: __le32,
    pub bMask:16: __le32,
}

// 82XX Support  start
// 82xx Default FLT Addresses
pub const FA_FLASH_LAYOUT_ADDR_82: c_uint = 0xFC400;
pub const FA_FLASH_DESCR_ADDR_82: c_uint = 0xFC000;
pub const FA_BOOT_LOAD_ADDR_82: c_uint = 0x04000;
pub const FA_BOOT_CODE_ADDR_82: c_uint = 0x20000;
pub const FA_RISC_CODE_ADDR_82: c_uint = 0x40000;
pub const FA_GOLD_RISC_CODE_ADDR_82: c_uint = 0x80000;
pub const FA_FLASH_ISCSI_CHAP: c_uint = 0x540000;
pub const FA_FLASH_CHAP_SIZE: c_uint = 0xC0000;
pub const FA_FLASH_ISCSI_DDB: c_uint = 0x420000;
pub const FA_FLASH_DDB_SIZE: c_uint = 0x080000;
// Flash Description Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fdt_layout {
    pub sig: [u8; 4],
    pub version: u16,
    pub len: u16,
    pub checksum: u16,
    pub unused1: [u8; 2],
    pub model: [u8; 16],
    pub man_id: u16,
    pub id: u16,
    pub flags: u8,
    pub erase_cmd: u8,
    pub alt_erase_cmd: u8,
    pub wrt_enable_cmd: u8,
    pub wrt_enable_bits: u8,
    pub wrt_sts_reg_cmd: u8,
    pub unprotect_sec_cmd: u8,
    pub read_man_id_cmd: u8,
    pub block_size: u32,
    pub alt_block_size: u32,
    pub flash_size: u32,
    pub wrt_enable_data: u32,
    pub read_id_addr_len: u8,
    pub wrt_disable_bits: u8,
    pub read_dev_id_len: u8,
    pub chip_erase_cmd: u8,
    pub read_timeout: u16,
    pub protect_sec_cmd: u8,
    pub unused2: [u8; 65],
}

// Flash Layout Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_location {
    pub sig: [u8; 4],
    pub start_lo: u16,
    pub start_hi: u16,
    pub version: u8,
    pub unused: [u8; 5],
    pub checksum: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_header {
    pub version: u16,
    pub length: u16,
    pub checksum: u16,
    pub unused: u16,
}

// 82xx FLT Regions
pub const FLT_REG_FDT: c_uint = 0x1a;
pub const FLT_REG_FLT: c_uint = 0x1c;
pub const FLT_REG_BOOTLOAD_82: c_uint = 0x72;
pub const FLT_REG_FW_82: c_uint = 0x74;
pub const FLT_REG_FW_82_1: c_uint = 0x97;
pub const FLT_REG_GOLD_FW_82: c_uint = 0x75;
pub const FLT_REG_BOOT_CODE_82: c_uint = 0x78;
pub const FLT_REG_ISCSI_PARAM: c_uint = 0x65;
pub const FLT_REG_ISCSI_CHAP: c_uint = 0x63;
pub const FLT_REG_ISCSI_DDB: c_uint = 0x6A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_flt_region {
    pub code: u32,
    pub size: u32,
    pub start: u32,
    pub end: u32,
}

//
// Mailbox Commands Structures and Definitions
//
// Mailbox command definitions
pub const MBOX_CMD_ABOUT_FW: c_uint = 0x0009;
pub const MBOX_CMD_PING: c_uint = 0x000B;
pub const PING_IPV6_PROTOCOL_ENABLE: c_uint = 0x1;
pub const PING_IPV6_LINKLOCAL_ADDR: c_uint = 0x4;
pub const PING_IPV6_ADDR0: c_uint = 0x8;
pub const PING_IPV6_ADDR1: c_uint = 0xC;
pub const MBOX_CMD_ENABLE_INTRS: c_uint = 0x0010;
pub const INTR_DISABLE: c_int = 0;
pub const INTR_ENABLE: c_int = 1;
pub const MBOX_CMD_STOP_FW: c_uint = 0x0014;
pub const MBOX_CMD_ABORT_TASK: c_uint = 0x0015;
pub const MBOX_CMD_LUN_RESET: c_uint = 0x0016;
pub const MBOX_CMD_TARGET_WARM_RESET: c_uint = 0x0017;
pub const MBOX_CMD_GET_MANAGEMENT_DATA: c_uint = 0x001E;
pub const MBOX_CMD_GET_FW_STATUS: c_uint = 0x001F;
pub const MBOX_CMD_SET_ISNS_SERVICE: c_uint = 0x0021;
pub const ISNS_DISABLE: c_int = 0;
pub const ISNS_ENABLE: c_int = 1;
pub const MBOX_CMD_COPY_FLASH: c_uint = 0x0024;
pub const MBOX_CMD_WRITE_FLASH: c_uint = 0x0025;
pub const MBOX_CMD_READ_FLASH: c_uint = 0x0026;
pub const MBOX_CMD_CLEAR_DATABASE_ENTRY: c_uint = 0x0031;
pub const MBOX_CMD_CONN_OPEN: c_uint = 0x0074;
pub const MBOX_CMD_CONN_CLOSE_SESS_LOGOUT: c_uint = 0x0056;
pub const DDB_NOT_LOGGED_IN: c_uint = 0x09;
pub const LOGOUT_OPTION_CLOSE_SESSION: c_uint = 0x0002;
pub const LOGOUT_OPTION_RELOGIN: c_uint = 0x0004;
pub const LOGOUT_OPTION_FREE_DDB: c_uint = 0x0008;
pub const MBOX_CMD_SET_PARAM: c_uint = 0x0059;
pub const SET_DRVR_VERSION: c_uint = 0x200;
pub const MAX_DRVR_VER_LEN: c_int = 24;
pub const MBOX_CMD_EXECUTE_IOCB_A64: c_uint = 0x005A;
pub const MBOX_CMD_INITIALIZE_FIRMWARE: c_uint = 0x0060;
pub const MBOX_CMD_GET_INIT_FW_CTRL_BLOCK: c_uint = 0x0061;
pub const MBOX_CMD_REQUEST_DATABASE_ENTRY: c_uint = 0x0062;
pub const MBOX_CMD_SET_DATABASE_ENTRY: c_uint = 0x0063;
pub const MBOX_CMD_GET_DATABASE_ENTRY: c_uint = 0x0064;
pub const DDB_DS_UNASSIGNED: c_uint = 0x00;
pub const DDB_DS_NO_CONNECTION_ACTIVE: c_uint = 0x01;
pub const DDB_DS_DISCOVERY: c_uint = 0x02;
pub const DDB_DS_SESSION_ACTIVE: c_uint = 0x04;
pub const DDB_DS_SESSION_FAILED: c_uint = 0x06;
pub const DDB_DS_LOGIN_IN_PROCESS: c_uint = 0x07;
pub const MBOX_CMD_GET_FW_STATE: c_uint = 0x0069;
pub const MBOX_CMD_GET_INIT_FW_CTRL_BLOCK_DEFAULTS: c_uint = 0x006A;
pub const MBOX_CMD_DIAG_TEST: c_uint = 0x0075;
pub const MBOX_CMD_GET_SYS_INFO: c_uint = 0x0078;
pub const MBOX_CMD_GET_NVRAM: c_uint = 0x0078	/* For 40xx */;
pub const MBOX_CMD_SET_NVRAM: c_uint = 0x0079	/* For 40xx */;
pub const MBOX_CMD_RESTORE_FACTORY_DEFAULTS: c_uint = 0x0087;
pub const MBOX_CMD_SET_ACB: c_uint = 0x0088;
pub const MBOX_CMD_GET_ACB: c_uint = 0x0089;
pub const MBOX_CMD_DISABLE_ACB: c_uint = 0x008A;
pub const MBOX_CMD_GET_IPV6_NEIGHBOR_CACHE: c_uint = 0x008B;
pub const MBOX_CMD_GET_IPV6_DEST_CACHE: c_uint = 0x008C;
pub const MBOX_CMD_GET_IPV6_DEF_ROUTER_LIST: c_uint = 0x008D;
pub const MBOX_CMD_GET_IPV6_LCL_PREFIX_LIST: c_uint = 0x008E;
pub const MBOX_CMD_SET_IPV6_NEIGHBOR_CACHE: c_uint = 0x0090;
pub const MBOX_CMD_GET_IP_ADDR_STATE: c_uint = 0x0091;
pub const MBOX_CMD_SEND_IPV6_ROUTER_SOL: c_uint = 0x0092;
pub const MBOX_CMD_GET_DB_ENTRY_CURRENT_IP_ADDR: c_uint = 0x0093;
pub const MBOX_CMD_SET_PORT_CONFIG: c_uint = 0x0122;
pub const MBOX_CMD_GET_PORT_CONFIG: c_uint = 0x0123;
pub const MBOX_CMD_SET_LED_CONFIG: c_uint = 0x0125;
pub const MBOX_CMD_GET_LED_CONFIG: c_uint = 0x0126;
pub const MBOX_CMD_MINIDUMP: c_uint = 0x0129;
// Port Config
pub const ENABLE_INTERNAL_LOOPBACK: c_uint = 0x04;
pub const ENABLE_EXTERNAL_LOOPBACK: c_uint = 0x08;
pub const ENABLE_DCBX: c_uint = 0x10;
// Minidump subcommand
pub const MINIDUMP_GET_SIZE_SUBCOMMAND: c_uint = 0x00;
pub const MINIDUMP_GET_TMPLT_SUBCOMMAND: c_uint = 0x01;
// Mailbox 1
pub const FW_STATE_READY: c_uint = 0x0000;
pub const FW_STATE_CONFIG_WAIT: c_uint = 0x0001;
pub const FW_STATE_WAIT_AUTOCONNECT: c_uint = 0x0002;
pub const FW_STATE_ERROR: c_uint = 0x0004;
pub const FW_STATE_CONFIGURING_IP: c_uint = 0x0008;
// Mailbox 3
pub const FW_ADDSTATE_OPTICAL_MEDIA: c_uint = 0x0001;
pub const FW_ADDSTATE_DHCPv4_ENABLED: c_uint = 0x0002;
pub const FW_ADDSTATE_DHCPv4_LEASE_ACQUIRED: c_uint = 0x0004;
pub const FW_ADDSTATE_DHCPv4_LEASE_EXPIRED: c_uint = 0x0008;
pub const FW_ADDSTATE_LINK_UP: c_uint = 0x0010;
pub const FW_ADDSTATE_ISNS_SVC_ENABLED: c_uint = 0x0020;
pub const FW_ADDSTATE_LINK_SPEED_10MBPS: c_uint = 0x0100;
pub const FW_ADDSTATE_LINK_SPEED_100MBPS: c_uint = 0x0200;
pub const FW_ADDSTATE_LINK_SPEED_1GBPS: c_uint = 0x0400;
pub const FW_ADDSTATE_LINK_SPEED_10GBPS: c_uint = 0x0800;
pub const MBOX_CMD_GET_DATABASE_ENTRY_DEFAULTS: c_uint = 0x006B;
pub const IPV6_DEFAULT_DDB_ENTRY: c_uint = 0x0001;
pub const MBOX_CMD_CONN_OPEN_SESS_LOGIN: c_uint = 0x0074;
pub const MBOX_CMD_GET_CRASH_RECORD: c_uint = 0x0076	/* 4010 only */;
pub const MBOX_CMD_GET_CONN_EVENT_LOG: c_uint = 0x0077;
pub const MBOX_CMD_IDC_ACK: c_uint = 0x0101;
pub const MBOX_CMD_IDC_TIME_EXTEND: c_uint = 0x0102;
pub const MBOX_CMD_PORT_RESET: c_uint = 0x0120;
pub const MBOX_CMD_SET_PORT_CONFIG: c_uint = 0x0122;
// Mailbox status definitions
pub const MBOX_COMPLETION_STATUS: c_int = 4;
pub const MBOX_STS_BUSY: c_uint = 0x0007;
pub const MBOX_STS_INTERMEDIATE_COMPLETION: c_uint = 0x1000;
pub const MBOX_STS_COMMAND_COMPLETE: c_uint = 0x4000;
pub const MBOX_STS_COMMAND_ERROR: c_uint = 0x4005;
pub const MBOX_ASYNC_EVENT_STATUS: c_int = 8;
pub const MBOX_ASTS_SYSTEM_ERROR: c_uint = 0x8002;
pub const MBOX_ASTS_REQUEST_TRANSFER_ERROR: c_uint = 0x8003;
pub const MBOX_ASTS_RESPONSE_TRANSFER_ERROR: c_uint = 0x8004;
pub const MBOX_ASTS_PROTOCOL_STATISTIC_ALARM: c_uint = 0x8005;
pub const MBOX_ASTS_SCSI_COMMAND_PDU_REJECTED: c_uint = 0x8006;
pub const MBOX_ASTS_LINK_UP: c_uint = 0x8010;
pub const MBOX_ASTS_LINK_DOWN: c_uint = 0x8011;
pub const MBOX_ASTS_DATABASE_CHANGED: c_uint = 0x8014;
pub const MBOX_ASTS_UNSOLICITED_PDU_RECEIVED: c_uint = 0x8015;
pub const MBOX_ASTS_SELF_TEST_FAILED: c_uint = 0x8016;
pub const MBOX_ASTS_LOGIN_FAILED: c_uint = 0x8017;
pub const MBOX_ASTS_DNS: c_uint = 0x8018;
pub const MBOX_ASTS_HEARTBEAT: c_uint = 0x8019;
pub const MBOX_ASTS_NVRAM_INVALID: c_uint = 0x801A;
pub const MBOX_ASTS_MAC_ADDRESS_CHANGED: c_uint = 0x801B;
pub const MBOX_ASTS_IP_ADDRESS_CHANGED: c_uint = 0x801C;
pub const MBOX_ASTS_DHCP_LEASE_EXPIRED: c_uint = 0x801D;
pub const MBOX_ASTS_DHCP_LEASE_ACQUIRED: c_uint = 0x801F;
pub const MBOX_ASTS_ISNS_UNSOLICITED_PDU_RECEIVED: c_uint = 0x8021;
pub const MBOX_ASTS_DUPLICATE_IP: c_uint = 0x8025;
pub const MBOX_ASTS_ARP_COMPLETE: c_uint = 0x8026;
pub const MBOX_ASTS_SUBNET_STATE_CHANGE: c_uint = 0x8027;
pub const MBOX_ASTS_RESPONSE_QUEUE_FULL: c_uint = 0x8028;
pub const MBOX_ASTS_IP_ADDR_STATE_CHANGED: c_uint = 0x8029;
pub const MBOX_ASTS_IPV6_DEFAULT_ROUTER_CHANGED: c_uint = 0x802A;
pub const MBOX_ASTS_IPV6_LINK_MTU_CHANGE: c_uint = 0x802B;
pub const MBOX_ASTS_IPV6_AUTO_PREFIX_IGNORED: c_uint = 0x802C;
pub const MBOX_ASTS_IPV6_ND_LOCAL_PREFIX_IGNORED: c_uint = 0x802D;
pub const MBOX_ASTS_ICMPV6_ERROR_MSG_RCVD: c_uint = 0x802E;
pub const MBOX_ASTS_INITIALIZATION_FAILED: c_uint = 0x8031;
pub const MBOX_ASTS_SYSTEM_WARNING_EVENT: c_uint = 0x8036;
pub const MBOX_ASTS_IDC_COMPLETE: c_uint = 0x8100;
pub const MBOX_ASTS_IDC_REQUEST_NOTIFICATION: c_uint = 0x8101;
pub const MBOX_ASTS_IDC_TIME_EXTEND_NOTIFICATION: c_uint = 0x8102;
pub const MBOX_ASTS_DCBX_CONF_CHANGE: c_uint = 0x8110;
pub const MBOX_ASTS_TXSCVR_INSERTED: c_uint = 0x8130;
pub const MBOX_ASTS_TXSCVR_REMOVED: c_uint = 0x8131;
pub const ISNS_EVENT_DATA_RECEIVED: c_uint = 0x0000;
pub const ISNS_EVENT_CONNECTION_OPENED: c_uint = 0x0001;
pub const ISNS_EVENT_CONNECTION_FAILED: c_uint = 0x0002;
pub const MBOX_ASTS_IPSEC_SYSTEM_FATAL_ERROR: c_uint = 0x8022;
pub const MBOX_ASTS_SUBNET_STATE_CHANGE: c_uint = 0x8027;
// ACB Configuration Defines
pub const ACB_CONFIG_DISABLE: c_uint = 0x00;
pub const ACB_CONFIG_SET: c_uint = 0x01;
// ACB/IP Address State Defines
pub const IP_ADDRSTATE_UNCONFIGURED: c_int = 0;
pub const IP_ADDRSTATE_INVALID: c_int = 1;
pub const IP_ADDRSTATE_ACQUIRING: c_int = 2;
pub const IP_ADDRSTATE_TENTATIVE: c_int = 3;
pub const IP_ADDRSTATE_DEPRICATED: c_int = 4;
pub const IP_ADDRSTATE_PREFERRED: c_int = 5;
pub const IP_ADDRSTATE_DISABLING: c_int = 6;
// FLASH offsets
pub const FLASH_SEGMENT_IFCB: c_uint = 0x04000000;
pub const FLASH_OPT_RMW_HOLD: c_int = 0;
pub const FLASH_OPT_RMW_INIT: c_int = 1;
pub const FLASH_OPT_COMMIT: c_int = 2;
pub const FLASH_OPT_RMW_COMMIT: c_int = 3;
// generic defines to enable/disable params
pub const QL4_PARAM_DISABLE: c_int = 0;
pub const QL4_PARAM_ENABLE: c_int = 1;
//
// Host Adapter Initialization Control Block (from host)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_ctrl_blk {
    pub /: *mut *mut uint8_t version; / 00,
pub const IFCB_VER_MIN: c_uint = 0x01;
pub const IFCB_VER_MAX: c_uint = 0x02;
    pub /: *mut *mut uint8_t control; / 01,
pub const CTRLOPT_NEW_CONN_DISABLE: c_uint = 0x0002;
    pub /: *mut *mut uint16_t fw_options; / 02-03,
pub const FWOPT_HEARTBEAT_ENABLE: c_uint = 0x1000;
pub const FWOPT_SESSION_MODE: c_uint = 0x0040;
pub const FWOPT_INITIATOR_MODE: c_uint = 0x0020;
pub const FWOPT_TARGET_MODE: c_uint = 0x0010;
pub const FWOPT_ENABLE_CRBDB: c_uint = 0x8000;
    pub /: *mut *mut uint16_t exec_throttle; / 04-05,
    pub /: *mut *mut uint8_t zio_count; / 06,
    pub /: *mut *mut uint8_t res0; / 07,
    pub /: *mut *mut uint16_t eth_mtu_size; / 08-09,
    pub /: *mut *mut uint16_t add_fw_options; / 0A-0B,
pub const ADFWOPT_SERIALIZE_TASK_MGMT: c_uint = 0x0400;
pub const ADFWOPT_AUTOCONN_DISABLE: c_uint = 0x0002;
    pub /: *mut *mut uint8_t hb_interval; / 0C,
    pub /: *mut *mut uint8_t inst_num; / 0D,
    pub /: *mut *mut uint16_t res1; / 0E-0F,
    pub /: *mut *mut uint16_t rqq_consumer_idx; / 10-11,
    pub /: *mut *mut uint16_t compq_producer_idx; / 12-13,
    pub /: *mut *mut uint16_t rqq_len; / 14-15,
    pub /: *mut *mut uint16_t compq_len; / 16-17,
    pub /: *mut *mut uint32_t rqq_addr_lo; / 18-1B,
    pub /: *mut *mut uint32_t rqq_addr_hi; / 1C-1F,
    pub /: *mut *mut uint32_t compq_addr_lo; / 20-23,
    pub /: *mut *mut uint32_t compq_addr_hi; / 24-27,
    pub /: *mut *mut uint32_t shdwreg_addr_lo; / 28-2B,
    pub /: *mut *mut uint32_t shdwreg_addr_hi; / 2C-2F,
    pub /: *mut *mut uint16_t iscsi_opts; / 30-31,
pub const ISCSIOPTS_HEADER_DIGEST_EN: c_uint = 0x2000;
pub const ISCSIOPTS_DATA_DIGEST_EN: c_uint = 0x1000;
pub const ISCSIOPTS_IMMEDIATE_DATA_EN: c_uint = 0x0800;
pub const ISCSIOPTS_INITIAL_R2T_EN: c_uint = 0x0400;
pub const ISCSIOPTS_DATA_SEQ_INORDER_EN: c_uint = 0x0200;
pub const ISCSIOPTS_DATA_PDU_INORDER_EN: c_uint = 0x0100;
pub const ISCSIOPTS_CHAP_AUTH_EN: c_uint = 0x0080;
pub const ISCSIOPTS_SNACK_EN: c_uint = 0x0040;
pub const ISCSIOPTS_DISCOVERY_LOGOUT_EN: c_uint = 0x0020;
pub const ISCSIOPTS_BIDI_CHAP_EN: c_uint = 0x0010;
pub const ISCSIOPTS_DISCOVERY_AUTH_EN: c_uint = 0x0008;
pub const ISCSIOPTS_STRICT_LOGIN_COMP_EN: c_uint = 0x0004;
pub const ISCSIOPTS_ERL: c_uint = 0x0003;
    pub /: *mut *mut uint16_t ipv4_tcp_opts; / 32-33,
pub const TCPOPT_DELAYED_ACK_DISABLE: c_uint = 0x8000;
pub const TCPOPT_DHCP_ENABLE: c_uint = 0x0200;
pub const TCPOPT_DNS_SERVER_IP_EN: c_uint = 0x0100;
pub const TCPOPT_SLP_DA_INFO_EN: c_uint = 0x0080;
pub const TCPOPT_NAGLE_ALGO_DISABLE: c_uint = 0x0020;
pub const TCPOPT_WINDOW_SCALE_DISABLE: c_uint = 0x0010;
pub const TCPOPT_TIMER_SCALE: c_uint = 0x000E;
pub const TCPOPT_TIMESTAMP_ENABLE: c_uint = 0x0001;
    pub /: *mut *mut uint16_t ipv4_ip_opts; / 34-35,
pub const IPOPT_IPV4_PROTOCOL_ENABLE: c_uint = 0x8000;
pub const IPOPT_IPV4_TOS_EN: c_uint = 0x4000;
pub const IPOPT_VLAN_TAGGING_ENABLE: c_uint = 0x2000;
pub const IPOPT_GRAT_ARP_EN: c_uint = 0x1000;
pub const IPOPT_ALT_CID_EN: c_uint = 0x0800;
pub const IPOPT_REQ_VID_EN: c_uint = 0x0400;
pub const IPOPT_USE_VID_EN: c_uint = 0x0200;
pub const IPOPT_LEARN_IQN_EN: c_uint = 0x0100;
pub const IPOPT_FRAGMENTATION_DISABLE: c_uint = 0x0010;
pub const IPOPT_IN_FORWARD_EN: c_uint = 0x0008;
pub const IPOPT_ARP_REDIRECT_EN: c_uint = 0x0004;
    pub /: *mut *mut uint16_t iscsi_max_pdu_size; / 36-37,
    pub /: *mut *mut uint8_t ipv4_tos; / 38,
    pub /: *mut *mut uint8_t ipv4_ttl; / 39,
    pub /: *mut *mut uint8_t acb_version; / 3A,
pub const ACB_NOT_SUPPORTED: c_uint = 0x00;
pub const ACB_SUPPORTED: c_uint = 0x02 /* Capable of ACB Version 2;
    pub /: *mut *mut uint8_t res2; / 3B,
    pub /: *mut *mut uint16_t def_timeout; / 3C-3D,
    pub /: *mut *mut uint16_t iscsi_fburst_len; / 3E-3F,
    pub /: *mut *mut uint16_t iscsi_def_time2wait; / 40-41,
    pub /: *mut *mut uint16_t iscsi_def_time2retain; / 42-43,
    pub /: *mut *mut uint16_t iscsi_max_outstnd_r2t; / 44-45,
    pub /: *mut *mut uint16_t conn_ka_timeout; / 46-47,
    pub /: *mut *mut uint16_t ipv4_port; / 48-49,
    pub /: *mut *mut uint16_t iscsi_max_burst_len; / 4A-4B,
    pub /: *mut *mut uint32_t res5; / 4C-4F,
    pub /: *mut *mut uint8_t ipv4_addr[4]; / 50-53,
    pub /: *mut *mut uint16_t ipv4_vlan_tag; / 54-55,
    pub /: *mut *mut uint8_t ipv4_addr_state; / 56,
    pub /: *mut *mut uint8_t ipv4_cacheid; / 57,
    pub /: *mut *mut uint8_t res6[8]; / 58-5F,
    pub /: *mut *mut uint8_t ipv4_subnet[4]; / 60-63,
    pub /: *mut *mut uint8_t res7[12]; / 64-6F,
    pub /: *mut *mut uint8_t ipv4_gw_addr[4]; / 70-73,
    pub /: *mut *mut uint8_t res8[0xc]; / 74-7F,
    pub /: *mut *mut uint8_t pri_dns_srvr_ip[4];/ 80-83,
    pub /: *mut *mut uint8_t sec_dns_srvr_ip[4];/ 84-87,
    pub /: *mut *mut uint16_t min_eph_port; / 88-89,
    pub /: *mut *mut uint16_t max_eph_port; / 8A-8B,
    pub /: *mut *mut uint8_t res9[4]; / 8C-8F,
    pub /: *mut *mut uint8_t iscsi_alias[32];/ 90-AF,
    pub /: *mut *mut uint8_t res9_1[0x16]; / B0-C5,
    pub /: *mut *mut uint16_t tgt_portal_grp;/ C6-C7,
    pub /: *mut *mut uint8_t abort_timer; / C8,
    pub /: *mut *mut uint8_t ipv4_tcp_wsf; / C9,
    pub /: *mut *mut uint8_t res10[6]; / CA-CF,
    pub /: *mut *mut uint8_t ipv4_sec_ip_addr[4]; / D0-D3,
    pub /: *mut *mut uint8_t ipv4_dhcp_vid_len; / D4,
    pub /: *mut *mut uint8_t ipv4_dhcp_vid[11]; / D5-DF,
    pub /: *mut *mut uint8_t res11[20]; / E0-F3,
    pub /: *mut *mut uint8_t ipv4_dhcp_alt_cid_len; / F4,
    pub /: *mut *mut uint8_t ipv4_dhcp_alt_cid[11]; / F5-FF,
    pub /: *mut *mut uint8_t iscsi_name[224]; / 100-1DF,
    pub /: *mut *mut uint8_t res12[32]; / 1E0-1FF,
    pub /: *mut *mut uint32_t cookie; / 200-203,
    pub /: *mut *mut uint16_t ipv6_port; / 204-205,
    pub /: *mut *mut uint16_t ipv6_opts; / 206-207,
pub const IPV6_OPT_IPV6_PROTOCOL_ENABLE: c_uint = 0x8000;
pub const IPV6_OPT_VLAN_TAGGING_ENABLE: c_uint = 0x2000;
pub const IPV6_OPT_GRAT_NEIGHBOR_ADV_EN: c_uint = 0x1000;
pub const IPV6_OPT_REDIRECT_EN: c_uint = 0x0004;
    pub /: *mut *mut uint16_t ipv6_addtl_opts; / 208-209,
pub const IPV6_ADDOPT_IGNORE_ICMP_ECHO_REQ: c_uint = 0x0040;
pub const IPV6_ADDOPT_MLD_EN: c_uint = 0x0004;
pub const IPV6_ADDOPT_NEIGHBOR_DISCOVERY_ADDR_ENABLE: c_uint = 0x0002 /* Pri ACB;
pub const IPV6_ADDOPT_AUTOCONFIG_LINK_LOCAL_ADDR: c_uint = 0x0001;
    pub /: *mut *mut uint16_t ipv6_tcp_opts; / 20A-20B,
pub const IPV6_TCPOPT_DELAYED_ACK_DISABLE: c_uint = 0x8000;
pub const IPV6_TCPOPT_NAGLE_ALGO_DISABLE: c_uint = 0x0020;
pub const IPV6_TCPOPT_WINDOW_SCALE_DISABLE: c_uint = 0x0010;
pub const IPV6_TCPOPT_TIMER_SCALE: c_uint = 0x000E;
pub const IPV6_TCPOPT_TIMESTAMP_EN: c_uint = 0x0001;
    pub /: *mut *mut uint8_t ipv6_tcp_wsf; / 20C,
    pub /: *mut *mut uint16_t ipv6_flow_lbl; / 20D-20F,
    pub /: *mut *mut uint8_t ipv6_dflt_rtr_addr[16]; / 210-21F,
    pub /: *mut *mut uint16_t ipv6_vlan_tag; / 220-221,
    pub /: *mut *mut uint8_t ipv6_lnk_lcl_addr_state;/ 222,
    pub /: *mut *mut uint8_t ipv6_addr0_state; / 223,
    pub /: *mut *mut uint8_t ipv6_addr1_state; / 224,
    pub /: *mut *mut uint8_t ipv6_dflt_rtr_state; / 225,
pub const IPV6_RTRSTATE_UNKNOWN: c_int = 0;
pub const IPV6_RTRSTATE_MANUAL: c_int = 1;
pub const IPV6_RTRSTATE_ADVERTISED: c_int = 3;
pub const IPV6_RTRSTATE_STALE: c_int = 4;
    pub /: *mut *mut uint8_t ipv6_traffic_class; / 226,
    pub /: *mut *mut uint8_t ipv6_hop_limit; / 227,
    pub /: *mut *mut uint8_t ipv6_if_id[8]; / 228-22F,
    pub /: *mut *mut uint8_t ipv6_addr0[16]; / 230-23F,
    pub /: *mut *mut uint8_t ipv6_addr1[16]; / 240-24F,
    pub /: *mut *mut uint32_t ipv6_nd_reach_time; / 250-253,
    pub /: *mut *mut uint32_t ipv6_nd_rexmit_timer; / 254-257,
    pub /: *mut *mut uint32_t ipv6_nd_stale_timeout; / 258-25B,
    pub /: *mut *mut uint8_t ipv6_dup_addr_detect_count; / 25C,
    pub /: *mut *mut uint8_t ipv6_cache_id; / 25D,
    pub /: *mut *mut uint8_t res13[18]; / 25E-26F,
    pub /: *mut *mut uint32_t ipv6_gw_advrt_mtu; / 270-273,
    pub /: *mut *mut uint8_t res14[140]; / 274-2FF,
}

// One IPv4, one IPv6 link local and 2 IPv6
//
pub const IP_STATE_MASK: c_uint = 0x0F000000;
pub const IP_STATE_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_fw_ctrl_blk {
    pub pri: addr_ctrl_blk,
// struct addr_ctrl_blk sec;
}

pub const PRIMARI_ACB: c_int = 0;
pub const SECONDARY_ACB: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_ctrl_blk_def {
    pub /: *mut *mut uint8_t reserved1[1]; / 00,
    pub /: *mut *mut uint8_t control; / 01,
    pub /: *mut *mut uint8_t reserved2[11]; / 02-0C,
    pub /: *mut *mut uint8_t inst_num; / 0D,
    pub /: *mut *mut uint8_t reserved3[34]; / 0E-2F,
    pub /: *mut *mut uint16_t iscsi_opts; / 30-31,
    pub /: *mut *mut uint16_t ipv4_tcp_opts; / 32-33,
    pub /: *mut *mut uint16_t ipv4_ip_opts; / 34-35,
    pub /: *mut *mut uint16_t iscsi_max_pdu_size; / 36-37,
    pub /: *mut *mut uint8_t ipv4_tos; / 38,
    pub /: *mut *mut uint8_t ipv4_ttl; / 39,
    pub /: *mut *mut uint8_t reserved4[2]; / 3A-3B,
    pub /: *mut *mut uint16_t def_timeout; / 3C-3D,
    pub /: *mut *mut uint16_t iscsi_fburst_len; / 3E-3F,
    pub /: *mut *mut uint8_t reserved5[4]; / 40-43,
    pub /: *mut *mut uint16_t iscsi_max_outstnd_r2t; / 44-45,
    pub /: *mut *mut uint8_t reserved6[2]; / 46-47,
    pub /: *mut *mut uint16_t ipv4_port; / 48-49,
    pub /: *mut *mut uint16_t iscsi_max_burst_len; / 4A-4B,
    pub /: *mut *mut uint8_t reserved7[4]; / 4C-4F,
    pub /: *mut *mut uint8_t ipv4_addr[4]; / 50-53,
    pub /: *mut *mut uint16_t ipv4_vlan_tag; / 54-55,
    pub /: *mut *mut uint8_t ipv4_addr_state; / 56,
    pub /: *mut *mut uint8_t ipv4_cacheid; / 57,
    pub /: *mut *mut uint8_t reserved8[8]; / 58-5F,
    pub /: *mut *mut uint8_t ipv4_subnet[4]; / 60-63,
    pub /: *mut *mut uint8_t reserved9[12]; / 64-6F,
    pub /: *mut *mut uint8_t ipv4_gw_addr[4]; / 70-73,
    pub /: *mut *mut uint8_t reserved10[84]; / 74-C7,
    pub /: *mut *mut uint8_t abort_timer; / C8,
    pub /: *mut *mut uint8_t ipv4_tcp_wsf; / C9,
    pub /: *mut *mut uint8_t reserved11[10]; / CA-D3,
    pub /: *mut *mut uint8_t ipv4_dhcp_vid_len; / D4,
    pub /: *mut *mut uint8_t ipv4_dhcp_vid[11]; / D5-DF,
    pub /: *mut *mut uint8_t reserved12[20]; / E0-F3,
    pub /: *mut *mut uint8_t ipv4_dhcp_alt_cid_len; / F4,
    pub /: *mut *mut uint8_t ipv4_dhcp_alt_cid[11]; / F5-FF,
    pub /: *mut *mut uint8_t iscsi_name[224]; / 100-1DF,
    pub /: *mut *mut uint8_t reserved13[32]; / 1E0-1FF,
    pub /: *mut *mut uint32_t cookie; / 200-203,
    pub /: *mut *mut uint16_t ipv6_port; / 204-205,
    pub /: *mut *mut uint16_t ipv6_opts; / 206-207,
    pub /: *mut *mut uint16_t ipv6_addtl_opts; / 208-209,
    pub /: *mut *mut uint16_t ipv6_tcp_opts; / 20A-20B,
    pub /: *mut *mut uint8_t ipv6_tcp_wsf; / 20C,
    pub /: *mut *mut uint16_t ipv6_flow_lbl; / 20D-20F,
    pub /: *mut *mut uint8_t ipv6_dflt_rtr_addr[16]; / 210-21F,
    pub /: *mut *mut uint16_t ipv6_vlan_tag; / 220-221,
    pub /: *mut *mut uint8_t ipv6_lnk_lcl_addr_state; / 222,
    pub /: *mut *mut uint8_t ipv6_addr0_state; / 223,
    pub /: *mut *mut uint8_t ipv6_addr1_state; / 224,
    pub /: *mut *mut uint8_t ipv6_dflt_rtr_state; / 225,
    pub /: *mut *mut uint8_t ipv6_traffic_class; / 226,
    pub /: *mut *mut uint8_t ipv6_hop_limit; / 227,
    pub /: *mut *mut uint8_t ipv6_if_id[8]; / 228-22F,
    pub /: *mut *mut uint8_t ipv6_addr0[16]; / 230-23F,
    pub /: *mut *mut uint8_t ipv6_addr1[16]; / 240-24F,
    pub /: *mut *mut uint32_t ipv6_nd_reach_time; / 250-253,
    pub /: *mut *mut uint32_t ipv6_nd_rexmit_timer; / 254-257,
    pub /: *mut *mut uint32_t ipv6_nd_stale_timeout; / 258-25B,
    pub /: *mut *mut uint8_t ipv6_dup_addr_detect_count; / 25C,
    pub /: *mut *mut uint8_t ipv6_cache_id; / 25D,
    pub /: *mut *mut uint8_t reserved14[18]; / 25E-26F,
    pub /: *mut *mut uint32_t ipv6_gw_advrt_mtu; / 270-273,
    pub /: *mut *mut uint8_t reserved15[140]; / 274-2FF,
}

//
pub const MAX_CHAP_ENTRIES_40XX: c_int = 128;
pub const MAX_CHAP_ENTRIES_82XX: c_int = 1024;
pub const MAX_RESRV_CHAP_IDX: c_int = 3;
pub const FLASH_CHAP_OFFSET: c_uint = 0x06000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_chap_table {
    pub link: u16,
    pub flags: u8,
    pub secret_len: u8,
pub const MIN_CHAP_SECRET_LEN: c_int = 12;
pub const MAX_CHAP_SECRET_LEN: c_int = 100;
    pub secret: [u8; MAX_CHAP_SECRET_LEN],
pub const MAX_CHAP_NAME_LEN: c_int = 256;
    pub name: [u8; MAX_CHAP_NAME_LEN],
    pub reserved: u16,
pub const CHAP_VALID_COOKIE: c_uint = 0x4092;
pub const CHAP_INVALID_COOKIE: c_uint = 0xFFEE;
    pub cookie: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_db_entry {
    pub /: *mut *mut uint16_t options; / 00-01,
pub const DDB_OPT_DISC_SESSION: c_uint = 0x10;
pub const DDB_OPT_TARGET: c_uint = 0x02 /* device is a target */;
pub const DDB_OPT_IPV6_DEVICE: c_uint = 0x100;
pub const DDB_OPT_AUTO_SENDTGTS_DISABLE: c_uint = 0x40;
pub const DDB_OPT_IPV6_NULL_LINK_LOCAL: c_uint = 0x800 /* post connection */;
pub const DDB_OPT_IPV6_FW_DEFINED_LINK_LOCAL: c_uint = 0x800 /* pre connection */;
pub const OPT_IS_FW_ASSIGNED_IPV6: c_int = 11;
pub const OPT_IPV6_DEVICE: c_int = 8;
pub const OPT_AUTO_SENDTGTS_DISABLE: c_int = 6;
pub const OPT_DISC_SESSION: c_int = 4;
pub const OPT_ENTRY_STATE: c_int = 3;
    pub /: *mut *mut uint16_t exec_throttle; / 02-03,
    pub /: *mut *mut uint16_t exec_count; / 04-05,
    pub /: *mut *mut uint16_t res0; / 06-07,
    pub /: *mut *mut uint16_t iscsi_options; / 08-09,
pub const ISCSIOPT_HEADER_DIGEST_EN: c_int = 13;
pub const ISCSIOPT_DATA_DIGEST_EN: c_int = 12;
pub const ISCSIOPT_IMMEDIATE_DATA_EN: c_int = 11;
pub const ISCSIOPT_INITIAL_R2T_EN: c_int = 10;
pub const ISCSIOPT_DATA_SEQ_IN_ORDER: c_int = 9;
pub const ISCSIOPT_DATA_PDU_IN_ORDER: c_int = 8;
pub const ISCSIOPT_CHAP_AUTH_EN: c_int = 7;
pub const ISCSIOPT_SNACK_REQ_EN: c_int = 6;
pub const ISCSIOPT_DISCOVERY_LOGOUT_EN: c_int = 5;
pub const ISCSIOPT_BIDI_CHAP_EN: c_int = 4;
pub const ISCSIOPT_DISCOVERY_AUTH_OPTIONAL: c_int = 3;
pub const ISCSIOPT_ERL1: c_int = 1;
pub const ISCSIOPT_ERL0: c_int = 0;
    pub /: *mut *mut uint16_t tcp_options; / 0A-0B,
pub const TCPOPT_TIMESTAMP_STAT: c_int = 6;
pub const TCPOPT_NAGLE_DISABLE: c_int = 5;
pub const TCPOPT_WSF_DISABLE: c_int = 4;
pub const TCPOPT_TIMER_SCALE3: c_int = 3;
pub const TCPOPT_TIMER_SCALE2: c_int = 2;
pub const TCPOPT_TIMER_SCALE1: c_int = 1;
pub const TCPOPT_TIMESTAMP_EN: c_int = 0;
    pub /: *mut *mut uint16_t ip_options; / 0C-0D,
pub const IPOPT_FRAGMENT_DISABLE: c_int = 4;
    pub /: *mut *mut uint16_t iscsi_max_rcv_data_seg_len; / 0E-0F,
pub const BYTE_UNITS: c_int = 512;
    pub /: *mut *mut uint32_t res1; / 10-13,
    pub /: *mut *mut uint16_t iscsi_max_snd_data_seg_len; / 14-15,
    pub /: *mut *mut uint16_t iscsi_first_burst_len; / 16-17,
    pub /: *mut *mut uint16_t iscsi_def_time2wait; / 18-19,
    pub /: *mut *mut uint16_t iscsi_def_time2retain; / 1A-1B,
    pub /: *mut *mut uint16_t iscsi_max_outsnd_r2t; / 1C-1D,
    pub /: *mut *mut uint16_t ka_timeout; / 1E-1F,
    pub converted: *mut *mut uint8_t isid[6]; / 20-25 big-endian, must be,
// to little-endian
    pub /: *mut *mut uint16_t tsid; / 26-27,
    pub /: *mut *mut uint16_t port; / 28-29,
    pub /: *mut *mut uint16_t iscsi_max_burst_len; / 2A-2B,
    pub /: *mut *mut uint16_t def_timeout; / 2C-2D,
    pub /: *mut *mut uint16_t res2; / 2E-2F,
    pub /: *mut *mut uint8_t ip_addr[0x10]; / 30-3F,
    pub /: *mut *mut uint8_t iscsi_alias[0x20]; / 40-5F,
    pub /: *mut *mut uint8_t tgt_addr[0x20]; / 60-7F,
    pub /: *mut *mut uint16_t mss; / 80-81,
    pub /: *mut *mut uint16_t res3; / 82-83,
    pub /: *mut *mut uint16_t lcl_port; / 84-85,
    pub /: *mut *mut uint8_t ipv4_tos; / 86,
    pub /: *mut *mut uint16_t ipv6_flow_lbl; / 87-89,
    pub /: *mut *mut uint8_t res4[0x36]; / 8A-BF,
    pub a: *mut *mut uint8_t iscsi_name[0xE0]; / C0-19F : xxzzy Make this,
// pointer to a string so we
// don't have to reserve so
// much RAM
    pub /: *mut *mut uint8_t link_local_ipv6_addr[0x10]; / 1A0-1AF,
    pub /: *mut *mut uint8_t res5[0x10]; / 1B0-1BF,
pub const DDB_NO_LINK: c_uint = 0xFFFF;
pub const DDB_ISNS: c_uint = 0xFFFD;
    pub /: *mut *mut uint16_t ddb_link; / 1C0-1C1,
    pub /: *mut *mut uint16_t chap_tbl_idx; / 1C2-1C3,
    pub /: *mut *mut uint16_t tgt_portal_grp; / 1C4-1C5,
    pub /: *mut *mut uint8_t tcp_xmt_wsf; / 1C6,
    pub /: *mut *mut uint8_t tcp_rcv_wsf; / 1C7,
    pub /: *mut *mut uint32_t stat_sn; / 1C8-1CB,
    pub /: *mut *mut uint32_t exp_stat_sn; / 1CC-1CF,
    pub /: *mut *mut uint8_t res6[0x2b]; / 1D0-1FB,
pub const DDB_VALID_COOKIE: c_uint = 0x9034;
    pub /: *mut *mut uint16_t cookie; / 1FC-1FD,
    pub /: *mut *mut uint16_t len; / 1FE-1FF,
}

//
// Flash definitions
pub const FLASH_OFFSET_SYS_INFO: c_uint = 0x02000000;
pub const FLASH_DEFAULTBLOCKSIZE: c_uint = 0x20000;

// for EOF
// signature
pub const FLASH_RAW_ACCESS_ADDR: c_uint = 0x8e000000;
pub const BOOT_PARAM_OFFSET_PORT0: c_uint = 0x3b0;
pub const BOOT_PARAM_OFFSET_PORT1: c_uint = 0x7b0;
pub const FLASH_OFFSET_DB_INFO: c_uint = 0x05000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_info_phys_addr {
    pub /: *mut *mut uint8_t address[6]; / 00-05,
    pub /: *mut *mut uint8_t filler[2]; / 06-07,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_sys_info {
    pub /: *mut *mut uint32_t cookie; / 00-03,
    pub /: *mut *mut uint32_t physAddrCount; / 04-07,
    pub /: *mut *mut sys_info_phys_addr physAddr[4]; / 08-27,
    pub /: *mut *mut uint8_t vendorId[128]; / 28-A7,
    pub /: *mut *mut uint8_t productId[128]; / A8-127,
    pub /: *mut *mut uint32_t serialNumber; / 128-12B,
// PCI Configuration values
    pub /: *mut *mut uint32_t pciDeviceVendor; / 12C-12F,
    pub /: *mut *mut uint32_t pciDeviceId; / 130-133,
    pub /: *mut *mut uint32_t pciSubsysVendor; / 134-137,
    pub /: *mut *mut uint32_t pciSubsysId; / 138-13B,
// This validates version 1.
    pub /: *mut *mut uint32_t crumbs; / 13C-13F,
    pub /: *mut *mut uint32_t enterpriseNumber; / 140-143,
    pub /: *mut *mut uint32_t mtu; / 144-147,
    pub /: *mut *mut uint32_t reserved0; / 148-14b,
    pub /: *mut *mut uint32_t crumbs2; / 14c-14f,
    pub /: *mut *mut uint8_t acSerialNumber[16]; / 150-15f,
    pub /: *mut *mut uint32_t crumbs3; / 160-16f,
// Leave this last in the struct so it is declared invalid if
// any new items are added.
//
    pub /: *mut *mut uint32_t reserved1[39]; / 170-1ff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_sys_info {
    pub /: *mut *mut uint8_t board_id_str[16]; / 0-f Keep board ID string first,
// in this structure for GUI.
    pub /: *mut *mut uint16_t board_id; / 10-11 board ID code,
    pub /: *mut *mut uint16_t phys_port_cnt; / 12-13 number of physical network ports,
    pub /: *mut *mut uint16_t port_num; / 14-15 network port for this PCI function,
// (port 0 is first port)
    pub /: *mut *mut uint8_t mac_addr[6]; / 16-1b MAC address for this PCI function,
    pub /: *mut *mut uint32_t iscsi_pci_func_cnt; / 1c-1f number of iSCSI PCI functions,
    pub /: *mut *mut uint32_t pci_func; / 20-23 this PCI function,
    pub /: *mut *mut unsigned char serial_number[16]; / 24-33 serial number string,
    pub /: *mut *mut uint8_t reserved[12]; / 34-3f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct about_fw_info {
    pub /: *mut *mut uint16_t fw_major; / 00 - 01,
    pub /: *mut *mut uint16_t fw_minor; / 02 - 03,
    pub /: *mut *mut uint16_t fw_patch; / 04 - 05,
    pub /: *mut *mut uint16_t fw_build; / 06 - 07,
    pub /: *mut *mut uint8_t fw_build_date[16]; / 08 - 17 ASCII String,
    pub /: *mut *mut uint8_t fw_build_time[16]; / 18 - 27 ASCII String,
    pub /: *mut *mut uint8_t fw_build_user[16]; / 28 - 37 ASCII String,
    pub /: *mut *mut uint16_t fw_load_source; / 38 - 39,
// 1 = Flash Primary,
//
    pub /: *mut *mut uint8_t reserved1[6]; / 3A - 3F,
    pub /: *mut *mut uint16_t iscsi_major; / 40 - 41,
    pub /: *mut *mut uint16_t iscsi_minor; / 42 - 43,
    pub /: *mut *mut uint16_t bootload_major; / 44 - 45,
    pub /: *mut *mut uint16_t bootload_minor; / 46 - 47,
    pub /: *mut *mut uint16_t bootload_patch; / 48 - 49,
    pub /: *mut *mut uint16_t bootload_build; / 4A - 4B,
    pub /: *mut *mut uint8_t extended_timestamp[180];/ 4C - FF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crash_record {
    pub /: *mut *mut uint16_t fw_major_version; / 00 - 01,
    pub /: *mut *mut uint16_t fw_minor_version; / 02 - 03,
    pub /: *mut *mut uint16_t fw_patch_version; / 04 - 05,
    pub /: *mut *mut uint16_t fw_build_version; / 06 - 07,
    pub /: *mut *mut uint8_t build_date[16]; / 08 - 17,
    pub /: *mut *mut uint8_t build_time[16]; / 18 - 27,
    pub /: *mut *mut uint8_t build_user[16]; / 28 - 37,
    pub /: *mut *mut uint8_t card_serial_num[16]; / 38 - 47,
    pub /: *mut *mut uint32_t time_of_crash_in_secs; / 48 - 4B,
    pub /: *mut *mut uint32_t time_of_crash_in_ms; / 4C - 4F,
    pub /: *mut *mut uint16_t out_RISC_sd_num_frames; / 50 - 51,
    pub /: *mut *mut uint16_t OAP_sd_num_words; / 52 - 53,
    pub /: *mut *mut uint16_t IAP_sd_num_frames; / 54 - 55,
    pub /: *mut *mut uint16_t in_RISC_sd_num_words; / 56 - 57,
    pub /: *mut *mut uint8_t reserved1[28]; / 58 - 7F,
    pub /: *mut *mut uint8_t out_RISC_reg_dump[256]; / 80 -17F,
    pub /: *mut *mut uint8_t in_RISC_reg_dump[256]; /180 -27F,
    pub /: *mut *mut uint8_t in_out_RISC_stack_dump[]; /280 - ???,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conn_event_log_entry {
pub const MAX_CONN_EVENT_LOG_ENTRIES: c_int = 100;
    pub /: *mut *mut uint32_t timestamp_sec; / 00 - 03 seconds since boot,
    pub /: *mut *mut uint32_t timestamp_ms; / 04 - 07 milliseconds since boot,
    pub /: *mut *mut uint16_t device_index; / 08 - 09,
    pub /: *mut *mut uint16_t fw_conn_state; / 0A - 0B,
    pub /: *mut *mut uint8_t event_type; / 0C - 0C,
    pub /: *mut *mut uint8_t error_code; / 0D - 0D,
    pub /: *mut *mut uint16_t error_code_detail; / 0E - 0F,
    pub /: *mut *mut uint8_t num_consecutive_events; / 10 - 10,
    pub /: *mut *mut uint8_t rsvd[3]; / 11 - 13,
}

//
// IOCB Commands Structures and Definitions
//

// IOCB header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_header {
    pub entryType: u8,
pub const ET_STATUS: c_uint = 0x03;
pub const ET_MARKER: c_uint = 0x04;
pub const ET_CONT_T1: c_uint = 0x0A;
pub const ET_STATUS_CONTINUATION: c_uint = 0x10;
pub const ET_CMND_T3: c_uint = 0x19;
pub const ET_PASSTHRU0: c_uint = 0x3A;
pub const ET_PASSTHRU_STATUS: c_uint = 0x3C;
pub const ET_MBOX_CMD: c_uint = 0x38;
pub const ET_MBOX_STATUS: c_uint = 0x39;
    pub entryStatus: u8,
    pub systemDefined: u8,
pub const SD_ISCSI_PDU: c_uint = 0x01;
    pub entryCount: u8,
// SyetemDefined definition
}

// Generic queue entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_entry {
    pub data: [u8; 60],
    pub signature: u32,
}

// 64 bit addressing segment counts
pub const COMMAND_SEG_A64: c_int = 1;
pub const CONTINUE_SEG_A64: c_int = 5;
// 64 bit addressing segment definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_seg_a64 {
    pub addrLow: u32,
    pub addrHigh: u32,
    pub base: },
    pub count: u32,
}

// Command Type 3 entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_t3_entry {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint16_t target; / 08-09,
    pub /: *mut *mut uint16_t connection_id; / 0A-0B,
    pub /: *mut *mut uint8_t control_flags; / 0C,
// data direction  (bits 5-6)
pub const CF_WRITE: c_uint = 0x20;
pub const CF_READ: c_uint = 0x40;
pub const CF_NO_DATA: c_uint = 0x00;
// task attributes (bits 2-0)
pub const CF_HEAD_TAG: c_uint = 0x03;
pub const CF_ORDERED_TAG: c_uint = 0x02;
pub const CF_SIMPLE_TAG: c_uint = 0x01;
// STATE FLAGS FIELD IS A PLACE HOLDER. THE FW WILL SET BITS
// IN THIS FIELD AS THE COMMAND IS PROCESSED. WHEN THE IOCB IS
// CHANGED TO AN IOSB THIS FIELD WILL HAVE THE STATE FLAGS SET
// PROPERLY.
//
    pub /: *mut *mut uint8_t state_flags; / 0D,
    pub /: *mut *mut uint8_t cmdRefNum; / 0E,
    pub /: *mut *mut uint8_t reserved1; / 0F,
    pub /: *mut *mut uint8_t cdb[IOCB_MAX_CDB_LEN]; / 10-1F,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut uint32_t cmdSeqNum; / 28-2B,
    pub /: *mut *mut uint16_t timeout; / 2C-2D,
    pub /: *mut *mut uint16_t dataSegCnt; / 2E-2F,
    pub /: *mut *mut uint32_t ttlByteCnt; / 30-33,
    pub /: *mut *mut data_seg_a64 dataseg[COMMAND_SEG_A64]; / 34-3F,
}

// Continuation Type 1 entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct continuation_t1_entry {
    pub hdr: qla4_header,
    pub dataseg: [data_seg_a64; CONTINUE_SEG_A64],
}

// Parameterize for 64 or 32 bits

// Marker entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_marker_entry {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t system_defined; / 04-07,
    pub /: *mut *mut uint16_t target; / 08-09,
    pub /: *mut *mut uint16_t modifier; / 0A-0B,
pub const MM_LUN_RESET: c_int = 0;
pub const MM_TGT_WARM_RESET: c_int = 1;
    pub /: *mut *mut uint16_t flags; / 0C-0D,
    pub /: *mut *mut uint16_t reserved1; / 0E-0F,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut uint64_t reserved2; / 18-1F,
    pub /: *mut *mut uint64_t reserved3; / 20-27,
    pub /: *mut *mut uint64_t reserved4; / 28-2F,
    pub /: *mut *mut uint64_t reserved5; / 30-37,
    pub /: *mut *mut uint64_t reserved6; / 38-3F,
}

// Status entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_entry {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint8_t scsiStatus; / 08,
    pub /: *mut *mut uint8_t iscsiFlags; / 09,
pub const ISCSI_FLAG_RESIDUAL_UNDER: c_uint = 0x02;
pub const ISCSI_FLAG_RESIDUAL_OVER: c_uint = 0x04;
    pub /: *mut *mut uint8_t iscsiResponse; / 0A,
    pub /: *mut *mut uint8_t completionStatus; / 0B,
pub const SCS_COMPLETE: c_uint = 0x00;
pub const SCS_INCOMPLETE: c_uint = 0x01;
pub const SCS_RESET_OCCURRED: c_uint = 0x04;
pub const SCS_ABORTED: c_uint = 0x05;
pub const SCS_TIMEOUT: c_uint = 0x06;
pub const SCS_DATA_OVERRUN: c_uint = 0x07;
pub const SCS_DATA_UNDERRUN: c_uint = 0x15;
pub const SCS_QUEUE_FULL: c_uint = 0x1C;
pub const SCS_DEVICE_UNAVAILABLE: c_uint = 0x28;
pub const SCS_DEVICE_LOGGED_OUT: c_uint = 0x29;
    pub /: *mut *mut uint8_t reserved1; / 0C,
// state_flags MUST be at the same location as state_flags in
// the Command_T3/4_Entry
    pub /: *mut *mut uint8_t state_flags; / 0D,
    pub /: *mut *mut uint16_t senseDataByteCnt; / 0E-0F,
    pub /: *mut *mut uint32_t residualByteCnt; / 10-13,
    pub /: *mut *mut uint32_t bidiResidualByteCnt; / 14-17,
    pub /: *mut *mut uint32_t expSeqNum; / 18-1B,
    pub /: *mut *mut uint32_t maxCmdSeqNum; / 1C-1F,
    pub /: *mut *mut uint8_t senseData[IOCB_MAX_SENSEDATA_LEN]; / 20-3F,
}

// Status Continuation entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_cont_entry {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint8_t ext_sense_data[IOCB_MAX_EXT_SENSEDATA_LEN]; / 04-63,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct passthru0 {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint16_t target; / 08-09,
    pub /: *mut *mut uint16_t connection_id; / 0A-0B,

    pub /: *mut *mut uint16_t control_flags; / 0C-0D,
pub const PT_FLAG_ETHERNET_FRAME: c_uint = 0x8000;
pub const PT_FLAG_ISNS_PDU: c_uint = 0x8000;
pub const PT_FLAG_SEND_BUFFER: c_uint = 0x0200;
pub const PT_FLAG_WAIT_4_RESPONSE: c_uint = 0x0100;
pub const PT_FLAG_ISCSI_PDU: c_uint = 0x1000;
    pub /: *mut *mut uint16_t timeout; / 0E-0F,

    pub /: *mut *mut data_seg_a64 out_dsd; / 10-1B,
    pub /: *mut *mut uint32_t res1; / 1C-1F,
    pub /: *mut *mut data_seg_a64 in_dsd; / 20-2B,
    pub /: *mut *mut uint8_t res2[20]; / 2C-3F,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct passthru_status {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint16_t target; / 08-09,
    pub /: *mut *mut uint16_t connectionID; / 0A-0B,
    pub /: *mut *mut uint8_t completionStatus; / 0C,
pub const PASSTHRU_STATUS_COMPLETE: c_uint = 0x01;
    pub /: *mut *mut uint8_t residualFlags; / 0D,
    pub /: *mut *mut uint16_t timeout; / 0E-0F,
    pub /: *mut *mut uint16_t portNumber; / 10-11,
    pub /: *mut *mut uint8_t res1[10]; / 12-1B,
    pub /: *mut *mut uint32_t outResidual; / 1C-1F,
    pub /: *mut *mut uint8_t res2[12]; / 20-2B,
    pub /: *mut *mut uint32_t inResidual; / 2C-2F,
    pub /: *mut *mut uint8_t res4[16]; / 30-3F,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_cmd_iocb {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint32_t in_mbox[8]; / 08-25,
    pub /: *mut *mut uint32_t res1[6]; / 26-3F,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_status_iocb {
    pub /: *mut *mut qla4_header hdr; / 00-03,
    pub /: *mut *mut uint32_t handle; / 04-07,
    pub /: *mut *mut uint32_t out_mbox[8]; / 08-25,
    pub /: *mut *mut uint32_t res1[6]; / 26-3F,
}

//
// ISP queue - response queue entry definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct response {
    pub data: [u8; 60],
    pub signature: u32,
pub const RESPONSE_PROCESSED: c_uint = 0xDEADDEAD	/* Signature */;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_iscsi_stats {
    pub /: *mut *mut uint64_t mac_tx_frames; / 0000–0007,
    pub /: *mut *mut uint64_t mac_tx_bytes; / 0008–000F,
    pub /: *mut *mut uint64_t mac_tx_multicast_frames; / 0010–0017,
    pub /: *mut *mut uint64_t mac_tx_broadcast_frames; / 0018–001F,
    pub /: *mut *mut uint64_t mac_tx_pause_frames; / 0020–0027,
    pub /: *mut *mut uint64_t mac_tx_control_frames; / 0028–002F,
    pub /: *mut *mut uint64_t mac_tx_deferral; / 0030–0037,
    pub /: *mut *mut uint64_t mac_tx_excess_deferral; / 0038–003F,
    pub /: *mut *mut uint64_t mac_tx_late_collision; / 0040–0047,
    pub /: *mut *mut uint64_t mac_tx_abort; / 0048–004F,
    pub /: *mut *mut uint64_t mac_tx_single_collision; / 0050–0057,
    pub /: *mut *mut uint64_t mac_tx_multiple_collision; / 0058–005F,
    pub /: *mut *mut uint64_t mac_tx_collision; / 0060–0067,
    pub /: *mut *mut uint64_t mac_tx_frames_dropped; / 0068–006F,
    pub /: *mut *mut uint64_t mac_tx_jumbo_frames; / 0070–0077,
    pub /: *mut *mut uint64_t mac_rx_frames; / 0078–007F,
    pub /: *mut *mut uint64_t mac_rx_bytes; / 0080–0087,
    pub /: *mut *mut uint64_t mac_rx_unknown_control_frames; / 0088–008F,
    pub /: *mut *mut uint64_t mac_rx_pause_frames; / 0090–0097,
    pub /: *mut *mut uint64_t mac_rx_control_frames; / 0098–009F,
    pub /: *mut *mut uint64_t mac_rx_dribble; / 00A0–00A7,
    pub /: *mut *mut uint64_t mac_rx_frame_length_error; / 00A8–00AF,
    pub /: *mut *mut uint64_t mac_rx_jabber; / 00B0–00B7,
    pub /: *mut *mut uint64_t mac_rx_carrier_sense_error; / 00B8–00BF,
    pub /: *mut *mut uint64_t mac_rx_frame_discarded; / 00C0–00C7,
    pub /: *mut *mut uint64_t mac_rx_frames_dropped; / 00C8–00CF,
    pub /: *mut *mut uint64_t mac_crc_error; / 00D0–00D7,
    pub /: *mut *mut uint64_t mac_encoding_error; / 00D8–00DF,
    pub /: *mut *mut uint64_t mac_rx_length_error_large; / 00E0–00E7,
    pub /: *mut *mut uint64_t mac_rx_length_error_small; / 00E8–00EF,
    pub /: *mut *mut uint64_t mac_rx_multicast_frames; / 00F0–00F7,
    pub /: *mut *mut uint64_t mac_rx_broadcast_frames; / 00F8–00FF,
    pub /: *mut *mut uint64_t ip_tx_packets; / 0100–0107,
    pub /: *mut *mut uint64_t ip_tx_bytes; / 0108–010F,
    pub /: *mut *mut uint64_t ip_tx_fragments; / 0110–0117,
    pub /: *mut *mut uint64_t ip_rx_packets; / 0118–011F,
    pub /: *mut *mut uint64_t ip_rx_bytes; / 0120–0127,
    pub /: *mut *mut uint64_t ip_rx_fragments; / 0128–012F,
    pub /: *mut *mut uint64_t ip_datagram_reassembly; / 0130–0137,
    pub /: *mut *mut uint64_t ip_invalid_address_error; / 0138–013F,
    pub /: *mut *mut uint64_t ip_error_packets; / 0140–0147,
    pub /: *mut *mut uint64_t ip_fragrx_overlap; / 0148–014F,
    pub /: *mut *mut uint64_t ip_fragrx_outoforder; / 0150–0157,
    pub /: *mut *mut uint64_t ip_datagram_reassembly_timeout; / 0158–015F,
    pub /: *mut *mut uint64_t ipv6_tx_packets; / 0160–0167,
    pub /: *mut *mut uint64_t ipv6_tx_bytes; / 0168–016F,
    pub /: *mut *mut uint64_t ipv6_tx_fragments; / 0170–0177,
    pub /: *mut *mut uint64_t ipv6_rx_packets; / 0178–017F,
    pub /: *mut *mut uint64_t ipv6_rx_bytes; / 0180–0187,
    pub /: *mut *mut uint64_t ipv6_rx_fragments; / 0188–018F,
    pub /: *mut *mut uint64_t ipv6_datagram_reassembly; / 0190–0197,
    pub /: *mut *mut uint64_t ipv6_invalid_address_error; / 0198–019F,
    pub /: *mut *mut uint64_t ipv6_error_packets; / 01A0–01A7,
    pub /: *mut *mut uint64_t ipv6_fragrx_overlap; / 01A8–01AF,
    pub /: *mut *mut uint64_t ipv6_fragrx_outoforder; / 01B0–01B7,
    pub /: *mut *mut uint64_t ipv6_datagram_reassembly_timeout; / 01B8–01BF,
    pub /: *mut *mut uint64_t tcp_tx_segments; / 01C0–01C7,
    pub /: *mut *mut uint64_t tcp_tx_bytes; / 01C8–01CF,
    pub /: *mut *mut uint64_t tcp_rx_segments; / 01D0–01D7,
    pub /: *mut *mut uint64_t tcp_rx_byte; / 01D8–01DF,
    pub /: *mut *mut uint64_t tcp_duplicate_ack_retx; / 01E0–01E7,
    pub /: *mut *mut uint64_t tcp_retx_timer_expired; / 01E8–01EF,
    pub /: *mut *mut uint64_t tcp_rx_duplicate_ack; / 01F0–01F7,
    pub /: *mut *mut uint64_t tcp_rx_pure_ackr; / 01F8–01FF,
    pub /: *mut *mut uint64_t tcp_tx_delayed_ack; / 0200–0207,
    pub /: *mut *mut uint64_t tcp_tx_pure_ack; / 0208–020F,
    pub /: *mut *mut uint64_t tcp_rx_segment_error; / 0210–0217,
    pub /: *mut *mut uint64_t tcp_rx_segment_outoforder; / 0218–021F,
    pub /: *mut *mut uint64_t tcp_rx_window_probe; / 0220–0227,
    pub /: *mut *mut uint64_t tcp_rx_window_update; / 0228–022F,
    pub /: *mut *mut uint64_t tcp_tx_window_probe_persist; / 0230–0237,
    pub /: *mut *mut uint64_t ecc_error_correction; / 0238–023F,
    pub /: *mut *mut uint64_t iscsi_pdu_tx; / 0240-0247,
    pub /: *mut *mut uint64_t iscsi_data_bytes_tx; / 0248-024F,
    pub /: *mut *mut uint64_t iscsi_pdu_rx; / 0250-0257,
    pub /: *mut *mut uint64_t iscsi_data_bytes_rx; / 0258-025F,
    pub /: *mut *mut uint64_t iscsi_io_completed; / 0260-0267,
    pub /: *mut *mut uint64_t iscsi_unexpected_io_rx; / 0268-026F,
    pub /: *mut *mut uint64_t iscsi_format_error; / 0270-0277,
    pub /: *mut *mut uint64_t iscsi_hdr_digest_error; / 0278-027F,
    pub /: *mut *mut uint64_t iscsi_data_digest_error; / 0280-0287,
    pub /: *mut *mut uint64_t iscsi_sequence_error; / 0288-028F,
    pub /: *mut *mut uint32_t tx_cmd_pdu; / 0290-0293,
    pub /: *mut *mut uint32_t tx_resp_pdu; / 0294-0297,
    pub /: *mut *mut uint32_t rx_cmd_pdu; / 0298-029B,
    pub /: *mut *mut uint32_t rx_resp_pdu; / 029C-029F,
    pub /: *mut *mut uint64_t tx_data_octets; / 02A0-02A7,
    pub /: *mut *mut uint64_t rx_data_octets; / 02A8-02AF,
    pub /: *mut *mut uint32_t hdr_digest_err; / 02B0–02B3,
    pub /: *mut *mut uint32_t data_digest_err; / 02B4–02B7,
    pub /: *mut *mut uint32_t conn_timeout_err; / 02B8–02BB,
    pub /: *mut *mut uint32_t framing_err; / 02BC–02BF,
    pub /: *mut *mut uint32_t tx_nopout_pdus; / 02C0–02C3,
    pub /: *mut *mut uint32_t tx_scsi_cmd_pdus; / 02C4–02C7,
    pub /: *mut *mut uint32_t tx_tmf_cmd_pdus; / 02C8–02CB,
    pub /: *mut *mut uint32_t tx_login_cmd_pdus; / 02CC–02CF,
    pub /: *mut *mut uint32_t tx_text_cmd_pdus; / 02D0–02D3,
    pub /: *mut *mut uint32_t tx_scsi_write_pdus; / 02D4–02D7,
    pub /: *mut *mut uint32_t tx_logout_cmd_pdus; / 02D8–02DB,
    pub /: *mut *mut uint32_t tx_snack_req_pdus; / 02DC–02DF,
    pub /: *mut *mut uint32_t rx_nopin_pdus; / 02E0–02E3,
    pub /: *mut *mut uint32_t rx_scsi_resp_pdus; / 02E4–02E7,
    pub /: *mut *mut uint32_t rx_tmf_resp_pdus; / 02E8–02EB,
    pub /: *mut *mut uint32_t rx_login_resp_pdus; / 02EC–02EF,
    pub /: *mut *mut uint32_t rx_text_resp_pdus; / 02F0–02F3,
    pub /: *mut *mut uint32_t rx_scsi_read_pdus; / 02F4–02F7,
    pub /: *mut *mut uint32_t rx_logout_resp_pdus; / 02F8–02FB,
    pub /: *mut *mut uint32_t rx_r2t_pdus; / 02FC–02FF,
    pub /: *mut *mut uint32_t rx_async_pdus; / 0300–0303,
    pub /: *mut *mut uint32_t rx_reject_pdus; / 0304–0307,
    pub /: *mut *mut uint8_t reserved2[264]; / 0x0308 - 0x040F,
}

pub const QLA8XXX_DBG_STATE_ARRAY_LEN: c_int = 16;
pub const QLA8XXX_DBG_CAP_SIZE_ARRAY_LEN: c_int = 8;
pub const QLA8XXX_DBG_RSVD_ARRAY_LEN: c_int = 8;
pub const QLA83XX_DBG_OCM_WNDREG_ARRAY_LEN: c_int = 16;
pub const QLA83XX_SS_OCM_WNDREG_INDEX: c_int = 3;
pub const QLA83XX_SS_PCI_INDEX: c_int = 0;
pub const QLA8022_TEMPLATE_CAP_OFFSET: c_int = 172;
pub const QLA83XX_TEMPLATE_CAP_OFFSET: c_int = 268;
pub const QLA80XX_TEMPLATE_RESERVED_BITS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_8xxx_minidump_template_hdr {
    pub entry_type: u32,
    pub first_entry_offset: u32,
    pub size_of_template: u32,
    pub capture_debug_level: u32,
    pub num_of_entries: u32,
    pub version: u32,
    pub driver_timestamp: u32,
    pub checksum: u32,
    pub driver_capture_mask: u32,
    pub driver_info_word2: u32,
    pub driver_info_word3: u32,
    pub driver_info_word4: u32,
    pub saved_state_array: [u32; QLA8XXX_DBG_STATE_ARRAY_LEN],
    pub capture_size_array: [u32; QLA8XXX_DBG_CAP_SIZE_ARRAY_LEN],
    pub ocm_window_reg: [u32; QLA83XX_DBG_OCM_WNDREG_ARRAY_LEN],
    pub capabilities: [u32; QLA80XX_TEMPLATE_RESERVED_BITS],
}
