//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/wd719x.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wd719x_sglist {
    pub ptr: __le32,
    pub length: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wd719x_card_type {
    WD719X_TYPE_UNKNOWN = 0,
    WD719X_TYPE_7193,
    WD719X_TYPE_7197,
    WD719X_TYPE_7296,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wd719x_regs {
    pub /: *mut *mut __le32 all; / All Status at once,
    pub /: *mut *mut u8 OPC; / Opcode register,
    pub /: *mut *mut u8 SCSI; / SCSI Errors,
    pub /: *mut *mut u8 SUE; / Spider unique Errors,
    pub /: *mut *mut u8 INT; / Interrupt Status,
    pub bytes: },
}

// Spider Command Block (SCB)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wd719x_scb {
    pub /: *mut *mut __le32 Int_SCB; / 00-03 Internal SCB link pointer (must be cleared),
    pub /: *mut *mut u8 SCB_opcode; / 04 SCB Command opcode,
    pub /: *mut *mut u8 CDB_tag; / 05 SCSI Tag byte for CDB queues (0 if untagged),
    pub /: *mut *mut u8 lun; / 06 SCSI LUN,
    pub /: *mut *mut u8 devid; / 07 SCSI Device ID,
    pub /: *mut *mut u8 CDB[16]; / 08-23 SCSI CDB (16 bytes as defined by ANSI spec.,
    pub /: *mut *mut __le32 data_p; / 24-27 Data transfer address (or SG list address),
    pub /: *mut *mut __le32 data_length; / 28-31 Data transfer Length (or SG list length),
    pub /: *mut *mut __le32 CDB_link; / 32-35 SCSI CDB Link Ptr,
    pub /: *mut *mut __le32 sense_buf; / 36-39 Auto request sense buffer address,
    pub /: *mut *mut u8 sense_buf_length;/ 40 Auto request sense transfer length,
    pub /: *mut *mut u8 reserved; / 41 reserved,
    pub /: *mut *mut u8 SCB_options; / 42 SCB-options,
    pub /: *mut *mut u8 SCB_tag_msg; / 43 Tagged messages options,
// Not filled in by host
    pub /: *mut *mut __le32 req_ptr; / 44-47 Ptr to Host Request returned on interrupt,
    pub /: *mut *mut u8 host_opcode; / 48 Host Command Opcode (same as AMR_00),
    pub /: *mut *mut u8 scsi_stat; / 49 SCSI Status returned,
    pub /: *mut *mut u8 ret_error; / 50 SPIDER Unique Error Code returned (SUE),
    pub /: *mut *mut u8 int_stat; / 51 Message u8 / Interrupt Status byte returned,
    pub /: *mut *mut __le32 transferred; / 52-55 Bytes Transferred,
    pub /: *mut *mut u8 last_trans[3]; / 56-58 Bytes Transferred in last session,
    pub /: *mut *mut u8 length; / 59 SCSI Messages Length (1-8),
    pub /: *mut *mut u8 sync_offset; / 60 Synchronous offset,
    pub /: *mut *mut u8 sync_rate; / 61 Synchronous rate,
    pub /: *mut *mut u8 flags[2]; / 62-63 SCB specific flags (local to each thread),
// everything below is for driver use (not used by card)
    pub /: *mut *mut dma_addr_t phys; / bus address of the SCB,
    pub dma_handle: dma_addr_t,
    pub /: *mut *mut *mut scsi_cmnd cmd; / a copy of the pointer we were passed,
    pub list: list_head,
    pub /: *mut *mut wd719x_sglist sg_list[WD719X_SG] __aligned(8); / SG list,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wd719x {
    pub /: *mut *mut *mut Scsi_Host sh; / pointer to host structure,
    pub pdev: *mut pci_dev,
    pub base: *mut void __iomem,
    pub /: *mut *mut wd719x_card_type type; / type of card,
    pub /: *mut *mut *mut void fw_virt; / firmware buffer CPU address,
    pub /: *mut *mut dma_addr_t fw_phys; / firmware buffer bus address,
    pub /: *mut *mut size_t fw_size; / firmware buffer size,
    pub /: *mut *mut *mut wd719x_host_param params; / host parameters (EEPROM),
    pub /: *mut *mut dma_addr_t params_phys; / host parameters bus address,
    pub /: *mut *mut *mut void hash_virt; / hash table CPU address,
    pub /: *mut *mut dma_addr_t hash_phys; / hash table bus address,
    pub active_scbs: list_head,
}

// timeout delays in microsecs
pub const WD719X_WAIT_FOR_CMD_READY: c_int = 500;
pub const WD719X_WAIT_FOR_RISC: c_int = 2000;
pub const WD719X_WAIT_FOR_SCSI_RESET: c_int = 3000000;
// All commands except 0x00 generate an interrupt
pub const WD719X_CMD_READY: c_uint = 0x00 /* Command register ready (or noop) */;
pub const WD719X_CMD_INIT_RISC: c_uint = 0x01 /* Initialize RISC */;
// 0x02 is reserved
pub const WD719X_CMD_BUSRESET: c_uint = 0x03 /* Assert SCSI bus reset */;
pub const WD719X_CMD_READ_FIRMVER: c_uint = 0x04 /* Read the Firmware Revision */;
pub const WD719X_CMD_ECHO_BYTES: c_uint = 0x05 /* Echo command bytes (DW) */;
// 0x06 is reserved
// 0x07 is reserved
pub const WD719X_CMD_GET_PARAM: c_uint = 0x08 /* Get programmable parameters */;
pub const WD719X_CMD_SET_PARAM: c_uint = 0x09 /* Set programmable parameters */;
pub const WD719X_CMD_SLEEP: c_uint = 0x0a /* Put SPIDER to sleep */;
pub const WD719X_CMD_READ_INIT: c_uint = 0x0b /* Read initialization parameters */;
pub const WD719X_CMD_RESTORE_INIT: c_uint = 0x0c /* Restore initialization parameters */;
// 0x0d is reserved
// 0x0e is reserved
// 0x0f is reserved
pub const WD719X_CMD_ABORT_TAG: c_uint = 0x10 /* Send Abort tag message to target */;
pub const WD719X_CMD_ABORT: c_uint = 0x11 /* Send Abort message to target */;
pub const WD719X_CMD_RESET: c_uint = 0x12 /* Send Reset message to target */;
pub const WD719X_CMD_INIT_SCAM: c_uint = 0x13 /* Initiate SCAM */;
pub const WD719X_CMD_GET_SYNC: c_uint = 0x14 /* Get synchronous rates */;
pub const WD719X_CMD_SET_SYNC: c_uint = 0x15 /* Set synchronous rates */;
pub const WD719X_CMD_GET_WIDTH: c_uint = 0x16 /* Get SCSI bus width */;
pub const WD719X_CMD_SET_WIDTH: c_uint = 0x17 /* Set SCSI bus width */;
pub const WD719X_CMD_GET_TAGS: c_uint = 0x18 /* Get tag flags */;
pub const WD719X_CMD_SET_TAGS: c_uint = 0x19 /* Set tag flags */;
pub const WD719X_CMD_GET_PARAM2: c_uint = 0x1a /* Get programmable params (format 2) */;
pub const WD719X_CMD_SET_PARAM2: c_uint = 0x1b /* Set programmable params (format 2) */;
// Commands with request pointers (mailbox)
pub const WD719X_CMD_PROCESS_SCB: c_uint = 0x80 /* Process SCSI Control Block (SCB) */;
// No interrupt generated on acceptance of SCB pointer
// interrupt status defines
pub const WD719X_INT_NONE: c_uint = 0x00 /* No interrupt pending */;
pub const WD719X_INT_NOERRORS: c_uint = 0x01 /* Command completed with no errors */;
pub const WD719X_INT_LINKNOERRORS: c_uint = 0x02 /* link cmd completed with no errors */;
pub const WD719X_INT_LINKNOSTATUS: c_uint = 0x03 /* link cmd completed with no flag set */;
pub const WD719X_INT_ERRORSLOGGED: c_uint = 0x04 /* cmd completed with errors logged */;
pub const WD719X_INT_SPIDERFAILED: c_uint = 0x05 /* cmd failed without valid SCSI status */;
pub const WD719X_INT_BADINT: c_uint = 0x80 /* unsolicited interrupt */;
pub const WD719X_INT_PIOREADY: c_uint = 0xf0 /* data ready for PIO output */;
// Spider Unique Error Codes (SUE)
pub const WD719X_SUE_NOERRORS: c_uint = 0x00 /* No errors detected by SPIDER */;
pub const WD719X_SUE_REJECTED: c_uint = 0x01 /* Command Rejected (bad opcode/param) */;
pub const WD719X_SUE_SCBQFULL: c_uint = 0x02 /* SCB queue full */;
// 0x03 is reserved
pub const WD719X_SUE_TERM: c_uint = 0x04 /* Host terminated SCB via primative cmd */;
pub const WD719X_SUE_CHAN1PAR: c_uint = 0x05 /* PCI Channel 1 parity error occurred */;
pub const WD719X_SUE_CHAN1ABORT: c_uint = 0x06 /* PCI Channel 1 system abort occurred */;
pub const WD719X_SUE_CHAN23PAR: c_uint = 0x07 /* PCI Channel 2/3 parity error occurred */;
pub const WD719X_SUE_CHAN23ABORT: c_uint = 0x08 /* PCI Channel 2/3 system abort occurred */;
pub const WD719X_SUE_TIMEOUT: c_uint = 0x10 /* Selection/reselection timeout */;
pub const WD719X_SUE_RESET: c_uint = 0x11 /* SCSI bus reset occurred */;
pub const WD719X_SUE_BUSERROR: c_uint = 0x12 /* SCSI bus error */;
pub const WD719X_SUE_WRONGWAY: c_uint = 0x13 /* Wrong data transfer dir set by target */;
pub const WD719X_SUE_BADPHASE: c_uint = 0x14 /* SCSI phase illegal or unexpected */;
pub const WD719X_SUE_TOOLONG: c_uint = 0x15 /* target requested too much data */;
pub const WD719X_SUE_BUSFREE: c_uint = 0x16 /* Unexpected SCSI bus free */;
pub const WD719X_SUE_ARSDONE: c_uint = 0x17 /* Auto request sense executed */;
pub const WD719X_SUE_IGNORED: c_uint = 0x18 /* SCSI message was ignored by target */;
pub const WD719X_SUE_WRONGTAGS: c_uint = 0x19 /* Tagged SCB & tags off (or vice versa) */;
pub const WD719X_SUE_BADTAGS: c_uint = 0x1a /* Wrong tag message type for target */;
pub const WD719X_SUE_NOSCAMID: c_uint = 0x1b /* No SCAM soft ID available */;
// code sizes
pub const WD719X_HASH_TABLE_SIZE: c_int = 4096;
// Advanced Mode Registers
// Regs 0x00..0x1f are for Advanced Mode of the card (RISC is running).
pub const WD719X_AMR_COMMAND: c_uint = 0x00;
pub const WD719X_AMR_CMD_PARAM: c_uint = 0x01;
pub const WD719X_AMR_CMD_PARAM_2: c_uint = 0x02;
pub const WD719X_AMR_CMD_PARAM_3: c_uint = 0x03;
pub const WD719X_AMR_SCB_IN: c_uint = 0x04;
pub const WD719X_AMR_BIOS_SHARE_INT: c_uint = 0x0f;
pub const WD719X_AMR_SCB_OUT: c_uint = 0x18;
pub const WD719X_AMR_OP_CODE: c_uint = 0x1c;
pub const WD719X_AMR_SCSI_STATUS: c_uint = 0x1d;
pub const WD719X_AMR_SCB_ERROR: c_uint = 0x1e;
pub const WD719X_AMR_INT_STATUS: c_uint = 0x1f;
pub const WD719X_DISABLE_INT: c_uint = 0x80;
// SCB flags
pub const WD719X_SCB_FLAGS_CHECK_DIRECTION: c_uint = 0x01;
pub const WD719X_SCB_FLAGS_PCI_TO_SCSI: c_uint = 0x02;
pub const WD719X_SCB_FLAGS_AUTO_REQUEST_SENSE: c_uint = 0x10;
pub const WD719X_SCB_FLAGS_DO_SCATTER_GATHER: c_uint = 0x20;
pub const WD719X_SCB_FLAGS_NO_DISCONNECT: c_uint = 0x40;
// PCI Registers used for reset, initial code download
// Regs 0x20..0x3f are for Normal (DOS) mode (RISC is asleep).
pub const WD719X_PCI_GPIO_CONTROL: c_uint = 0x3C;
pub const WD719X_PCI_GPIO_DATA: c_uint = 0x3D;
pub const WD719X_PCI_PORT_RESET: c_uint = 0x3E;
pub const WD719X_PCI_MODE_SELECT: c_uint = 0x3F;
pub const WD719X_PCI_EXTERNAL_ADDR: c_uint = 0x60;
pub const WD719X_PCI_INTERNAL_ADDR: c_uint = 0x64;
pub const WD719X_PCI_DMA_TRANSFER_SIZE: c_uint = 0x66;
pub const WD719X_PCI_CHANNEL2_3CMD: c_uint = 0x68;
pub const WD719X_PCI_CHANNEL2_3STATUS: c_uint = 0x69;
pub const WD719X_GPIO_ID_BITS: c_uint = 0x0a;
pub const WD719X_PRAM_BASE_ADDR: c_uint = 0x00;
// codes written to or read from the card
pub const WD719X_PCI_RESET: c_uint = 0x01;
pub const WD719X_ENABLE_ADVANCE_MODE: c_uint = 0x01;
pub const WD719X_START_CHANNEL2_3DMA: c_uint = 0x17;
pub const WD719X_START_CHANNEL2_3DONE: c_uint = 0x01;
pub const WD719X_START_CHANNEL2_3ABORT: c_uint = 0x20;
// 33C296 GPIO bits for EEPROM pins

// EEPROM contents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wd719x_eeprom_header {
    pub sig1: u8,
    pub sig2: u8,
    pub version: u8,
    pub checksum: u8,
    pub cfg_offset: u8,
    pub cfg_size: u8,
    pub setup_offset: u8,
    pub setup_size: u8,
    pub __packed: },
pub const WD719X_EE_SIG1: c_int = 0;
pub const WD719X_EE_SIG2: c_int = 1;
pub const WD719X_EE_VERSION: c_int = 2;
pub const WD719X_EE_CHECKSUM: c_int = 3;
pub const WD719X_EE_CFG_OFFSET: c_int = 4;
pub const WD719X_EE_CFG_SIZE: c_int = 5;
pub const WD719X_EE_SETUP_OFFSET: c_int = 6;
pub const WD719X_EE_SETUP_SIZE: c_int = 7;
pub const WD719X_EE_SCSI_ID_MASK: c_uint = 0xf;
// SPIDER Host Parameters Block (=EEPROM configuration block)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wd719x_host_param {
    pub /: *mut *mut u8 ch_1_th; / FIFO threshold,
    pub /: *mut *mut u8 scsi_conf; / SCSI configuration,
    pub /: *mut *mut u8 own_scsi_id; / controller SCSI ID,
    pub timeout*/: *mut *mut u8 sel_timeout; / selection,
    pub /: *mut *mut u8 sleep_timer; / seep timer,
    pub /: *mut *mut __le16 cdb_size;/ CDB size groups,
    pub /: *mut *mut __le16 tag_en; / Tag msg enables (ID 0-15),
    pub /: *mut *mut u8 scsi_pad; / SCSI pad control,
    pub /: *mut *mut __le32 wide; / WIDE msg options (ID 0-15),
    pub /: *mut *mut __le32 sync; / SYNC msg options (ID 0-15),
    pub /: *mut *mut u8 soft_mask; / soft error mask,
    pub /: *mut *mut u8 unsol_mask; / unsolicited error mask,
    pub __packed: },
