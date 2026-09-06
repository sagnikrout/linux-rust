//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid.h
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
// Driver features - change the values to enable or disable features in the
// driver.
//
// Command coalescing - This feature allows the driver to be able to combine
// two or more commands and issue as one command in order to boost I/O
// performance. Useful if the nature of the I/O is sequential. It is not very
// useful for random natured I/Os.
//
pub const MEGA_HAVE_COALESCING: c_int = 0;
//
// Clustering support - Set this flag if you are planning to use the
// clustering services provided by the megaraid controllers and planning to
// setup a cluster
//
pub const MEGA_HAVE_CLUSTERING: c_int = 1;
//
// Driver statistics - Set this flag if you are interested in statics about
// number of I/O completed on each logical drive and how many interrupts
// generated. If enabled, this information is available through /proc
// interface and through the private ioctl. Setting this flag has a
// performance penalty.
//
pub const MEGA_HAVE_STATS: c_int = 0;
//
// Enhanced /proc interface - This feature will allow you to have a more
// detailed /proc interface for megaraid driver. E.g., a real time update of
// the status of the logical drives, battery status, physical drives etc.
//
pub const MEGA_HAVE_ENH_PROC: c_int = 1;
pub const MAX_DEV_TYPE: c_int = 32;
pub const PCI_DEVICE_ID_DISCOVERY: c_uint = 0x000E;
pub const PCI_DEVICE_ID_PERC4_DI: c_uint = 0x000F;
pub const PCI_DEVICE_ID_PERC4_QC_VERDE: c_uint = 0x0407;
pub const HBA_SIGNATURE: c_uint = 0x3344;
pub const HBA_SIGNATURE_471: c_uint = 0xCCCC;
pub const HBA_SIGNATURE_64BIT: c_uint = 0x0299;

pub const DEFAULT_INITIATOR_ID: c_int = 7;

pub const MAX_COMMANDS: c_int = 126;

pub const MAX_CDB_LEN: c_int = 10;

pub const DEF_CMD_PER_LUN: c_int = 63;

pub const MAX_FIRMWARE_STATUS: c_int = 46;

pub const MAX_SECTORS_PER_IO: c_int = 128;
pub const MAX_LOGICAL_DRIVES_40LD: c_int = 40;
pub const FC_MAX_PHYSICAL_DEVICES: c_int = 256;
pub const MAX_LOGICAL_DRIVES_8LD: c_int = 8;
pub const MAX_CHANNELS: c_int = 5;
pub const MAX_TARGET: c_int = 15;

pub const MAX_ROW_SIZE_40LD: c_int = 32;
pub const MAX_ROW_SIZE_8LD: c_int = 8;
pub const MAX_SPAN_DEPTH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_out {
// 0x0 */ u8 cmd;
// 0x1 */ u8 cmdid;
// 0x2 */ u16 numsectors;
// 0x4 */ u32 lba;
// 0x8 */ u32 xferaddr;
// 0xC */ u8 logdrv;
// 0xD */ u8 numsgelements;
// 0xE */ u8 resvd;
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_in {
// 0xF */ volatile u8 busy;
// 0x10 */ volatile u8 numstatus;
// 0x11 */ volatile u8 status;
// 0x12 */ volatile u8 completed[MAX_FIRMWARE_STATUS];
    pub poll: volatile u8,
    pub ack: volatile u8,
// C attribute field omitted
    pub m_out: mbox_out,
    pub m_in: mbox_in,
// C attribute field omitted
    pub xfer_segment_lo: u32,
    pub xfer_segment_hi: u32,
    pub mbox: mbox_t,
// C attribute field omitted
//
// Passthru definitions
//
pub const MAX_REQ_SENSE_LEN: c_uint = 0x20;
    pub /: *mut *mut u8 timeout:3; / 0=6sec/1=60sec/2=10min/3=3hrs,
    pub ars:1: u8,
    pub reserved:3: u8,
    pub islogical:1: u8,
    pub /: *mut *mut u8 logdrv; / if islogical == 1,
    pub /: *mut *mut u8 channel; / if islogical == 0,
    pub /: *mut *mut u8 target; / if islogical == 0,
    pub /: *mut *mut u8 queuetag; / unused,
    pub /: *mut *mut u8 queueaction; / unused,
    pub cdb: [u8; MAX_CDB_LEN],
    pub cdblen: u8,
    pub reqsenselen: u8,
    pub reqsensearea: [u8; MAX_REQ_SENSE_LEN],
    pub numsgelements: u8,
    pub scsistatus: u8,
    pub dataxferaddr: u32,
    pub dataxferlen: u32,
// C attribute field omitted
//
// Extended passthru: support CDB > 10 bytes
//
    pub /: *mut *mut u8 timeout:3; / 0=6sec/1=60sec/2=10min/3=3hrs,
    pub ars:1: u8,
    pub rsvd1:1: u8,
    pub cd_rom:1: u8,
    pub rsvd2:1: u8,
    pub islogical:1: u8,
    pub /: *mut *mut u8 logdrv; / if islogical == 1,
    pub /: *mut *mut u8 channel; / if islogical == 0,
    pub /: *mut *mut u8 target; / if islogical == 0,
    pub /: *mut *mut u8 queuetag; / unused,
    pub /: *mut *mut u8 queueaction; / unused,
    pub cdblen: u8,
    pub rsvd3: u8,
    pub cdb: [u8; MAX_EXT_CDB_LEN],
    pub numsgelements: u8,
    pub status: u8,
    pub reqsenselen: u8,
    pub reqsensearea: [u8; MAX_REQ_SENSE_LEN],
    pub rsvd4: u8,
    pub dataxferaddr: u32,
    pub dataxferlen: u32,
// C attribute field omitted
    pub address: u64,
    pub length: u32,
// C attribute field omitted
    pub address: u32,
    pub length: u32,
// C attribute field omitted
// Queued command data
    pub idx: c_int,
    pub state: u32,
    pub list: list_head,
    pub raw_mbox: [u8; 66],
    pub dma_type: u32,
    pub dma_direction: u32,
    pub cmd: *mut scsi_cmnd,
    pub dma_h_bulkdata: dma_addr_t,
    pub dma_h_sgdata: dma_addr_t,
    pub sgl: *mut mega_sglist,
    pub sgl64: *mut mega_sgl64,
    pub sgl_dma_addr: dma_addr_t,
    pub pthru: *mut mega_passthru,
    pub pthru_dma_addr: dma_addr_t,
    pub epthru: *mut mega_ext_passthru,
    pub epthru_dma_addr: dma_addr_t,
    pub scb_t: },
//
// Flags to follow the scb as it transitions between various stages
//
pub const SCB_FREE: c_uint = 0x0000	/* on the free list */;
pub const SCB_ACTIVE: c_uint = 0x0001	/* off the free list */;
pub const SCB_PENDQ: c_uint = 0x0002	/* on the pending queue */;
pub const SCB_ISSUED: c_uint = 0x0004	/* issued - owner f/w */;
pub const SCB_ABORT: c_uint = 0x0008	/* Got an abort for this one */;
pub const SCB_RESET: c_uint = 0x0010	/* Got a reset for this one */;
//
// Utilities declare this strcture size as 1024 bytes. So more fields can
// be added in future.
//
    pub /: *mut *mut u32 data_size; / current size in bytes (not including resvd),
    pub config_signature: u32,
// Current value is 0x00282008
// 0x28=MAX_LOGICAL_DRIVES,
// 0x20=Number of stripes and
// 0x08=Number of spans
    pub /: *mut *mut u8 fw_version[16]; / printable ASCI string,
    pub /: *mut *mut u8 bios_version[16]; / printable ASCI string,
    pub /: *mut *mut u8 product_name[80]; / printable ASCI string,
    pub /: *mut *mut u8 max_commands; / Max. concurrent commands supported,
    pub /: *mut *mut u8 nchannels; / Number of SCSI Channels detected,
    pub /: *mut *mut u8 fc_loop_present; / Number of Fibre Loops detected,
    pub /: *mut *mut u8 mem_type; / EDO, FPM, SDRAM etc,
    pub signature: u32,
    pub /: *mut *mut u16 dram_size; / In terms of MB,
    pub subsysid: u16,
    pub subsysvid: u16,
    pub notify_counters: u8,
    pub /: *mut *mut u8 pad1k[889]; / 135 + 889 resvd = 1024 total size,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notify {
    pub /: *mut *mut u32 global_counter; / Any change increments this counter,
    pub /: *mut *mut u8 param_counter; / Indicates any params changed,
    pub /: *mut *mut u8 param_id; / Param modified - defined below,
    pub /: *mut *mut u16 param_val; / New val of last param modified,
    pub /: *mut *mut u8 write_config_counter; / write config occurred,
    pub write_config_rsvd: [u8; 3],
    pub /: *mut *mut u8 ldrv_op_counter; / Indicates ldrv op started/completed,
    pub /: *mut *mut u8 ldrv_opid; / ldrv num,
    pub /: *mut *mut u8 ldrv_opcmd; / ldrv operation - defined below,
    pub /: *mut *mut u8 ldrv_opstatus; / status of the operation,
    pub /: *mut *mut u8 ldrv_state_counter; / Indicates change of ldrv state,
    pub /: *mut *mut u8 ldrv_state_id; / ldrv num,
    pub /: *mut *mut u8 ldrv_state_new; / New state,
    pub /: *mut *mut u8 ldrv_state_old; / old state,
    pub /: *mut *mut u8 pdrv_state_counter; / Indicates change of ldrv state,
    pub /: *mut *mut u8 pdrv_state_id; / pdrv id,
    pub /: *mut *mut u8 pdrv_state_new; / New state,
    pub /: *mut *mut u8 pdrv_state_old; / old state,
    pub /: *mut *mut u8 pdrv_fmt_counter; / Indicates pdrv format started/over,
    pub /: *mut *mut u8 pdrv_fmt_id; / pdrv id,
    pub /: *mut *mut u8 pdrv_fmt_val; / format started/over,
    pub pdrv_fmt_rsvd: u8,
    pub /: *mut *mut u8 targ_xfer_counter; / Indicates SCSI-2 Xfer rate change,
    pub /: *mut *mut u8 targ_xfer_id; / pdrv Id,
    pub /: *mut *mut u8 targ_xfer_val; / new Xfer params of last pdrv,
    pub targ_xfer_rsvd: u8,
    pub /: *mut *mut u8 fcloop_id_chg_counter; / Indicates loopid changed,
    pub /: *mut *mut u8 fcloopid_pdrvid; / pdrv id,
    pub /: *mut *mut u8 fcloop_id0; / loopid on fc loop 0,
    pub /: *mut *mut u8 fcloop_id1; / loopid on fc loop 1,
    pub /: *mut *mut u8 fcloop_state_counter; / Indicates loop state changed,
    pub /: *mut *mut u8 fcloop_state0; / state of fc loop 0,
    pub /: *mut *mut u8 fcloop_state1; / state of fc loop 1,
    pub fcloop_state_rsvd: u8,
// C attribute field omitted
pub const MAX_NOTIFY_SIZE: c_uint = 0x80;

    pub /: *mut *mut u32 data_size; / current size in bytes (not including resvd),
    pub notify: notify,
    pub CUR_NOTIFY_SIZE]: u8 notify_rsvd[MAX_NOTIFY_SIZE -,
    pub /: *mut *mut u8 rebuild_rate; / Rebuild rate (0% - 100%),
    pub /: *mut *mut u8 cache_flush_interval; / In terms of Seconds,
    pub sense_alert: u8,
    pub /: *mut *mut u8 drive_insert_count; / drive insertion count,
    pub battery_status: u8,
    pub /: *mut *mut u8 num_ldrv; / No. of Log Drives configured,
    pub of: *mut *mut u8 recon_state[MAX_LOGICAL_DRIVES_40LD / 8]; / State,
    pub logdrv: *mut *mut u16 ldrv_op_status[MAX_LOGICAL_DRIVES_40LD / 8]; /,
    pub /: *mut *mut u32 ldrv_size[MAX_LOGICAL_DRIVES_40LD];/ Size of each log drv,
    pub ldrv_prop: [u8; MAX_LOGICAL_DRIVES_40LD],
    pub /: *mut *mut u8 ldrv_state[MAX_LOGICAL_DRIVES_40LD];/ State of log drives,
    pub /: *mut *mut u8 pdrv_state[FC_MAX_PHYSICAL_DEVICES];/ State of phys drvs.,
    pub 16]: u16 pdrv_format[FC_MAX_PHYSICAL_DEVICES /,
    pub /: *mut *mut u8 targ_xfer[80]; / phys device transfer rate,
    pub /: *mut *mut u8 pad1k[263]; / 761 + 263reserved = 1024 bytes total size,
// C attribute field omitted
// Structures
    pub /: *mut *mut u8 max_commands; / Max concurrent commands supported,
    pub /: *mut *mut u8 rebuild_rate; / Rebuild rate - 0% thru 100%,
    pub /: *mut *mut u8 max_targ_per_chan; / Max targ per channel,
    pub /: *mut *mut u8 nchannels; / Number of channels on HBA,
    pub /: *mut *mut u8 fw_version[4]; / Firmware version,
    pub /: *mut *mut u16 age_of_flash; / Number of times FW has been flashed,
    pub /: *mut *mut u8 chip_set_value; / Contents of 0xC0000832,
    pub /: *mut *mut u8 dram_size; / In MB,
    pub /: *mut *mut u8 cache_flush_interval; / in seconds,
    pub bios_version: [u8; 4],
    pub board_type: u8,
    pub sense_alert: u8,
    pub configuration: *mut *mut u8 write_config_count; / Increase with every,
    pub inserted: *mut *mut u8 drive_inserted_count; / Increase with every drive,
//
    pub /: *mut *mut u8 inserted_drive; / Channel:Id of inserted drive,
    pub /*: *mut u8 battery_status;,
// BIT 0: battery module missing
// BIT 1: VBAD
// BIT 2: temperature high
// BIT 3: battery pack missing
// BIT 4,5:
// 00 - charge complete
// 01 - fast charge in progress
// 10 - fast charge fail
// 11 - undefined
// Bit 6: counter > 1000
// Bit 7: Undefined
//
    pub dec_fault_bus_info: u8,
// C attribute field omitted
    pub /: *mut *mut u8 num_ldrv; / Number of logical drives configured,
    pub rsvd: [u8; 3],
    pub ldrv_size: [u32; MAX_LOGICAL_DRIVES_8LD],
    pub ldrv_prop: [u8; MAX_LOGICAL_DRIVES_8LD],
    pub ldrv_state: [u8; MAX_LOGICAL_DRIVES_8LD],
// C attribute field omitted
    pub pdrv_state: [u8; MAX_PHYSICAL_DRIVES],
    pub rsvd: u8,
// C attribute field omitted
// RAID inquiry: Mailbox command 0x05
    pub adapter_info: mega_adp_info,
    pub logdrv_info: mega_ldrv_info,
    pub pdrv_info: mega_pdrv_info,
// C attribute field omitted
// RAID extended inquiry: Mailbox command 0x04
    pub raid_inq: mraid_inquiry,
    pub phys_drv_format: [u16; MAX_CHANNELS],
    pub stack_attn: u8,
    pub modem_status: u8,
    pub rsvd: [u8; 2],
// C attribute field omitted
    pub channel: u8,
    pub target: u8,
// C attribute field omitted
    pub /: *mut *mut u32 start_blk; / starting block,
    pub /: *mut *mut u32 num_blks; / # of blocks,
    pub device: [adp_device; MAX_ROW_SIZE_40LD],
// C attribute field omitted
    pub /: *mut *mut u32 start_blk; / starting block,
    pub /: *mut *mut u32 num_blks; / # of blocks,
    pub device: [adp_device; MAX_ROW_SIZE_8LD],
// C attribute field omitted
    pub /: *mut *mut u8 span_depth; / Total # of spans,
    pub /: *mut *mut u8 level; / RAID level,
    pub read: *mut *mut u8 read_ahead; / read ahead, no read ahead, adaptive,
    pub /: *mut *mut u8 stripe_sz; / Encoded stripe size,
    pub /: *mut *mut u8 status; / Status of the logical drive,
    pub /: *mut *mut u8 write_mode; / write mode, write_through/write_back,
    pub /: *mut *mut u8 direct_io; / direct io or through cache,
    pub /: *mut *mut u8 row_size; / Number of stripes in a row,
// C attribute field omitted
    pub lparam: logdrv_param,
    pub span: [adp_span_40ld; MAX_SPAN_DEPTH],
// C attribute field omitted
    pub lparam: logdrv_param,
    pub span: [adp_span_8ld; MAX_SPAN_DEPTH],
// C attribute field omitted
    pub /: *mut *mut u8 type; / Type of the device,
    pub /: *mut *mut u8 cur_status; / current status of the device,
    pub /: *mut *mut u8 tag_depth; / Level of tagging,
    pub /: *mut *mut u8 sync_neg; / sync negotiation - ENABLE or DISABLE,
    pub byte: *mut *mut u32 size; / configurable size in terms of 512,
// C attribute field omitted
    pub /: *mut *mut u8 nlog_drives; / number of logical drives,
    pub resvd: [u8; 3],
    pub ldrv: [logdrv_40ld; MAX_LOGICAL_DRIVES_40LD],
    pub pdrv: [phys_drv; MAX_PHYSICAL_DRIVES],
// C attribute field omitted
    pub /: *mut *mut u8 nlog_drives; / number of logical drives,
    pub resvd: [u8; 3],
    pub ldrv: [logdrv_8ld; MAX_LOGICAL_DRIVES_8LD],
    pub pdrv: [phys_drv; MAX_PHYSICAL_DRIVES],
// C attribute field omitted
//
// User ioctl structure.
// This structure will be used for Traditional Method ioctl interface
// commands (0x80),Alternate Buffer Method (0x81) ioctl commands and the
// Driver ioctls.
// The Driver ioctl interface handles the commands at the driver level,
// without being sent to the card.
//
// system call imposed limit. Change accordingly
pub const IOCTL_MAX_DATALEN: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uioctl_t {
    pub inlen: u32,
    pub outlen: u32,
    pub fca: [u8; 16],
    pub opcode: u8,
    pub subopcode: u8,
    pub adapno: u16,

    pub buffer: *mut u8,
    pub pad: [u8; 4],
    pub buffer: *mut u8,

    pub length: u32,
// C attribute field omitted
// C attribute field omitted
    pub /: *mut *mut u8 mbox[18]; / 16 bytes + 2 status bytes,
    pub pthru: mega_passthru,

    pub /: *mut *mut *mut char __user data; / buffer <= 4096 for 0x80 commands,
    pub pad: [c_char; 4],
    pub data: *mut char __user,
// C attribute field omitted
//
// struct mcontroller is used to pass information about the controllers in the
// system. Its up to the application how to use the information. We are passing
// as much info about the cards as possible and useful. Before issuing the
// call to find information about the cards, the application needs to issue a
// ioctl first to find out the number of controllers in the system.
//
pub const MAX_CONTROLLERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcontroller {
    pub base: u64,
    pub irq: u8,
    pub numldrv: u8,
    pub pcibus: u8,
    pub pcidev: u16,
    pub pcifun: u8,
    pub pciid: u16,
    pub pcivendor: u16,
    pub pcislot: u8,
    pub uid: u32,
}

//
// mailbox structure used for internal commands
//
// Defines for Driver IOCTL interface
//

//
// Definition for the new ioctl interface (NIT)
//
// Vendor specific Group-7 commands
//
pub const VENDOR_SPECIFIC_COMMANDS: c_uint = 0xE0;

//
// The ioctl command. No other command shall be used for this interface
//

//
// Data direction flags
//
pub const UIOC_RD: c_uint = 0x00001;
pub const UIOC_WR: c_uint = 0x00002;
//
// ioctl opcodes
//
pub const MBOX_CMD: c_uint = 0x00000	/* DCMD or passthru command */;
pub const GET_DRIVER_VER: c_uint = 0x10000	/* Get driver version */;
pub const GET_N_ADAP: c_uint = 0x20000	/* Get number of adapters */;
pub const GET_ADAP_INFO: c_uint = 0x30000	/* Get information about a adapter */;
pub const GET_CAP: c_uint = 0x40000	/* Get ioctl capabilities */;
pub const GET_STATS: c_uint = 0x50000	/* Get statistics, including error info */;
//
// The ioctl structure.
// MBOX macro converts a nitioctl_t structure to megacmd_t pointer and
// MBOX_P macro converts a nitioctl_t pointer to megacmd_t pointer.
//

//
// I/O statistics for some applications like SNMP agent. The caller must
// provide the number of logical drives for which status should be reported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct private_bios_data {
    pub /*: *mut u8 geometry:4;,
// bits 0-3 - BIOS geometry
// 0x0001 - 1GB
// 0x0010 - 2GB
// 0x1000 - 8GB
// Others values are invalid
//
    pub /: *mut *mut u8 unused:4; / bits 4-7 are unused,
    pub /*: *mut u8 boot_drv;,
// logical drive set as boot drive
// 0..7 - for 8LD cards
// 0..39 - for 40LD cards
//
    pub rsvd: [u8; 12],
    pub /: *mut *mut u16 cksum; / 0-(sum of first 13 bytes of this structure),
// C attribute field omitted
//
// Mailbox and firmware commands and subopcodes used in this driver.
//
pub const MEGA_MBOXCMD_LREAD: c_uint = 0x01;
pub const MEGA_MBOXCMD_LWRITE: c_uint = 0x02;
pub const MEGA_MBOXCMD_PASSTHRU: c_uint = 0x03;
pub const MEGA_MBOXCMD_ADPEXTINQ: c_uint = 0x04;
pub const MEGA_MBOXCMD_ADAPTERINQ: c_uint = 0x05;
pub const MEGA_MBOXCMD_LREAD64: c_uint = 0xA7;
pub const MEGA_MBOXCMD_LWRITE64: c_uint = 0xA8;
pub const MEGA_MBOXCMD_PASSTHRU64: c_uint = 0xC3;
pub const MEGA_MBOXCMD_EXTPTHRU: c_uint = 0xE3;
pub const MAIN_MISC_OPCODE: c_uint = 0xA4	/* f/w misc opcode */;
pub const GET_MAX_SG_SUPPORT: c_uint = 0x01	/* get max sg len supported by f/w */;
pub const FC_NEW_CONFIG: c_uint = 0xA1;
pub const NC_SUBOP_PRODUCT_INFO: c_uint = 0x0E;
pub const NC_SUBOP_ENQUIRY3: c_uint = 0x0F;
pub const ENQ3_GET_SOLICITED_FULL: c_uint = 0x02;
pub const OP_DCMD_READ_CONFIG: c_uint = 0x04;
pub const NEW_READ_CONFIG_8LD: c_uint = 0x67;
pub const READ_CONFIG_8LD: c_uint = 0x07;
pub const FLUSH_ADAPTER: c_uint = 0x0A;
pub const FLUSH_SYSTEM: c_uint = 0xFE;
//
// Command for random deletion of logical drives
//
pub const FC_DEL_LOGDRV: c_uint = 0xA4	/* f/w command */;
pub const OP_SUP_DEL_LOGDRV: c_uint = 0x2A	/* is feature supported */;
pub const OP_GET_LDID_MAP: c_uint = 0x18	/* get ldid and logdrv number map */;
pub const OP_DEL_LOGDRV: c_uint = 0x1C	/* delete logical drive */;
//
// BIOS commands
//
pub const IS_BIOS_ENABLED: c_uint = 0x62;
pub const GET_BIOS: c_uint = 0x01;
pub const CHNL_CLASS: c_uint = 0xA9;
pub const GET_CHNL_CLASS: c_uint = 0x00;
pub const SET_CHNL_CLASS: c_uint = 0x01;
pub const CH_RAID: c_uint = 0x01;
pub const CH_SCSI: c_uint = 0x00;
pub const BIOS_PVT_DATA: c_uint = 0x40;
pub const GET_BIOS_PVT_DATA: c_uint = 0x00;
//
// Commands to support clustering
//
pub const MEGA_GET_TARGET_ID: c_uint = 0x7D;
pub const MEGA_CLUSTER_OP: c_uint = 0x70;
pub const MEGA_GET_CLUSTER_MODE: c_uint = 0x02;
pub const MEGA_CLUSTER_CMD: c_uint = 0x6E;
pub const MEGA_RESERVE_LD: c_uint = 0x01;
pub const MEGA_RELEASE_LD: c_uint = 0x02;
pub const MEGA_RESET_RESERVATIONS: c_uint = 0x03;
pub const MEGA_RESERVATION_STATUS: c_uint = 0x04;
pub const MEGA_RESERVE_PD: c_uint = 0x05;
pub const MEGA_RELEASE_PD: c_uint = 0x06;
//
// Module battery status
//
pub const MEGA_BATT_MODULE_MISSING: c_uint = 0x01;
pub const MEGA_BATT_LOW_VOLTAGE: c_uint = 0x02;
pub const MEGA_BATT_TEMP_HIGH: c_uint = 0x04;
pub const MEGA_BATT_PACK_MISSING: c_uint = 0x08;
pub const MEGA_BATT_CHARGE_MASK: c_uint = 0x30;
pub const MEGA_BATT_CHARGE_DONE: c_uint = 0x00;
pub const MEGA_BATT_CHARGE_INPROG: c_uint = 0x10;
pub const MEGA_BATT_CHARGE_FAIL: c_uint = 0x20;
pub const MEGA_BATT_CYCLES_EXCEEDED: c_uint = 0x40;
//
// Physical drive states.
//
pub const PDRV_UNCNF: c_int = 0;
pub const PDRV_ONLINE: c_int = 3;
pub const PDRV_FAILED: c_int = 4;
pub const PDRV_RBLD: c_int = 5;
pub const PDRV_HOTSPARE: c_int = 6;
//
// Raid logical drive states.
//
pub const RDRV_OFFLINE: c_int = 0;
pub const RDRV_DEGRADED: c_int = 1;
pub const RDRV_OPTIMAL: c_int = 2;
pub const RDRV_DELETED: c_int = 3;
//
// Read, write and cache policies
//
pub const NO_READ_AHEAD: c_int = 0;
pub const READ_AHEAD: c_int = 1;
pub const ADAP_READ_AHEAD: c_int = 2;
pub const WRMODE_WRITE_THRU: c_int = 0;
pub const WRMODE_WRITE_BACK: c_int = 1;
pub const CACHED_IO: c_int = 0;
pub const DIRECT_IO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct megaraid_cmd_priv {
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_cmd_and_priv {
    pub cmd: scsi_cmnd,
    pub priv: megaraid_cmd_priv,
}

// See also scsi_mq_setup_tags()
//
// Each controller's soft state
//
// mbox64 with mbox not aligned on 16-byte boundary

// Host adapter parameters

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mega_hbas {
    pub is_bios_enabled: c_int,
    pub hostdata_addr: *mut adapter_t,
}

//
// For state flag. Do not use LSB(8 bits) which are
// reserved for storing info about channels.
//
pub const IN_ABORT: c_uint = 0x80000000L;
pub const IN_RESET: c_uint = 0x40000000L;
pub const BOARD_MEMMAP: c_uint = 0x20000000L;
pub const BOARD_IOMAP: c_uint = 0x10000000L;
pub const BOARD_40LD: c_uint = 0x08000000L;
pub const BOARD_64BIT: c_uint = 0x04000000L;
pub const INTR_VALID: c_uint = 0x40;
pub const PCI_CONF_AMISIG: c_uint = 0xa0;
pub const PCI_CONF_AMISIG64: c_uint = 0xa4;
pub const MEGA_DMA_TYPE_NONE: c_uint = 0xFFFF;
pub const MEGA_BULK_DATA: c_uint = 0x0001;
pub const MEGA_SGLIST: c_uint = 0x0002;
//
// Parameters for the io-mapped controllers
//
// I/O Port offsets
pub const CMD_PORT: c_uint = 0x00;
pub const ACK_PORT: c_uint = 0x00;
pub const TOGGLE_PORT: c_uint = 0x01;
pub const INTR_PORT: c_uint = 0x0a;
pub const MBOX_BUSY_PORT: c_uint = 0x00;
pub const MBOX_PORT0: c_uint = 0x04;
pub const MBOX_PORT1: c_uint = 0x05;
pub const MBOX_PORT2: c_uint = 0x06;
pub const MBOX_PORT3: c_uint = 0x07;
pub const ENABLE_MBOX_REGION: c_uint = 0x0B;
// I/O Port Values
pub const ISSUE_BYTE: c_uint = 0x10;
pub const ACK_BYTE: c_uint = 0x08;
pub const ENABLE_INTR_BYTE: c_uint = 0xc0;
pub const DISABLE_INTR_BYTE: c_uint = 0x00;
pub const VALID_INTR_BYTE: c_uint = 0x40;
pub const MBOX_BUSY_BYTE: c_uint = 0x10;
pub const ENABLE_MBOX_BYTE: c_uint = 0x00;
// Setup some port macros here

//
// This is our SYSDEP area. All kernel specific detail should be placed here -
// as much as possible
//
// End of SYSDEP area
//
extern "C" {
    pub fn mega_query_adapter(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn issue_scb(: *mut adapter_t, : *mut scb_t) -> static int;
}
extern "C" {
    pub fn mega_setup_mailbox(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn __mega_runpendq(: *mut adapter_t) -> static void;
}
extern "C" {
    pub fn issue_scb_block(: *mut adapter_t, : *mut u_char) -> static int;
}
extern "C" {
    pub fn megaraid_isr_memmapped(_arg: c_int, : *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn megaraid_isr_iomapped(_arg: c_int, : *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn mega_free_scb(: *mut adapter_t, : *mut scb_t) -> static void;
}
extern "C" {
    pub fn megaraid_abort(: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn megaraid_reset(: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn megaraid_abort_and_reset(: *mut adapter_t, : *mut scsi_cmnd, _arg: c_int) -> static int;
}
extern "C" {
    pub fn __mega_busywait_mbox(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn mega_rundoneq(: *mut adapter_t) -> static void;
}
extern "C" {
    pub fn mega_cmd_done(: *mut adapter_t, []: u8, _arg: c_int, _arg: c_int) -> static void;
}
extern "C" {
    pub fn mega_free_sgl(adapter: *mut adapter_t);
}
extern "C" {
    pub fn megadev_open(: *mut inode, : *mut file) -> static int;
}
extern "C" {
    pub fn megadev_ioctl(: *mut file, int: unsigned, long: unsigned) -> static int;
}
extern "C" {
    pub fn mega_m_to_n(: *mut void __user, : *mut nitioctl_t) -> static int;
}
extern "C" {
    pub fn mega_n_to_m(: *mut void __user, : *mut megacmd_t) -> static int;
}
extern "C" {
    pub fn mega_init_scb(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn mega_is_bios_enabled(: *mut adapter_t) -> static int;
}

extern "C" {
    pub fn mega_create_proc_entry(_arg: c_int, : *mut proc_dir_entry) -> static void;
}
extern "C" {
    pub fn mega_adapinq(: *mut adapter_t, _arg: dma_addr_t) -> static int;
}
extern "C" {
    pub fn mega_internal_dev_inquiry(: *mut adapter_t, _arg: u8, _arg: u8, _arg: dma_addr_t) -> static int;
}

extern "C" {
    pub fn mega_support_ext_cdb(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn mega_enum_raid_scsi(: *mut adapter_t) -> static void;
}
extern "C" {
    pub fn mega_get_boot_drv(: *mut adapter_t) -> static void;
}
extern "C" {
    pub fn mega_support_random_del(: *mut adapter_t) -> static int;
}
extern "C" {
    pub fn mega_del_logdrv(: *mut adapter_t, _arg: c_int) -> static int;
}
extern "C" {
    pub fn mega_do_del_logdrv(: *mut adapter_t, _arg: c_int) -> static int;
}
extern "C" {
    pub fn mega_get_max_sgl(: *mut adapter_t) -> static void;
}
extern "C" {
    pub fn mega_internal_command(: *mut adapter_t, : *mut megacmd_t, : *mut mega_passthru) -> static int;
}
extern "C" {
    pub fn mega_support_cluster(: *mut adapter_t) -> static int;
}

// vi: set ts=8 sw=8 tw=78:
