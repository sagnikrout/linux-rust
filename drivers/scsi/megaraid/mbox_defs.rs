//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/mbox_defs.h
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
// Linux MegaRAID Unified device driver
//
// Copyright (c) 2003-2004  LSI Logic Corporation.
//
// FILE		: mbox_defs.h
//

//
// Commands and states for mailbox based controllers
//
pub const MBOXCMD_LREAD: c_uint = 0x01;
pub const MBOXCMD_LWRITE: c_uint = 0x02;
pub const MBOXCMD_PASSTHRU: c_uint = 0x03;
pub const MBOXCMD_ADPEXTINQ: c_uint = 0x04;
pub const MBOXCMD_ADAPTERINQ: c_uint = 0x05;
pub const MBOXCMD_LREAD64: c_uint = 0xA7;
pub const MBOXCMD_LWRITE64: c_uint = 0xA8;
pub const MBOXCMD_PASSTHRU64: c_uint = 0xC3;
pub const MBOXCMD_EXTPTHRU: c_uint = 0xE3;
pub const MAIN_MISC_OPCODE: c_uint = 0xA4;
pub const GET_MAX_SG_SUPPORT: c_uint = 0x01;
pub const SUPPORT_EXT_CDB: c_uint = 0x16;
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
pub const FC_DEL_LOGDRV: c_uint = 0xA4;
pub const OP_SUP_DEL_LOGDRV: c_uint = 0x2A;
pub const OP_GET_LDID_MAP: c_uint = 0x18;
pub const OP_DEL_LOGDRV: c_uint = 0x1C;
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
pub const GET_TARGET_ID: c_uint = 0x7D;
pub const CLUSTER_OP: c_uint = 0x70;
pub const GET_CLUSTER_MODE: c_uint = 0x02;
pub const CLUSTER_CMD: c_uint = 0x6E;
pub const RESERVE_LD: c_uint = 0x01;
pub const RELEASE_LD: c_uint = 0x02;
pub const RESET_RESERVATIONS: c_uint = 0x03;
pub const RESERVATION_STATUS: c_uint = 0x04;
pub const RESERVE_PD: c_uint = 0x05;
pub const RELEASE_PD: c_uint = 0x06;
//
// Module battery status
//
pub const BATTERY_MODULE_MISSING: c_uint = 0x01;
pub const BATTERY_LOW_VOLTAGE: c_uint = 0x02;
pub const BATTERY_TEMP_HIGH: c_uint = 0x04;
pub const BATTERY_PACK_MISSING: c_uint = 0x08;
pub const BATTERY_CHARGE_MASK: c_uint = 0x30;
pub const BATTERY_CHARGE_DONE: c_uint = 0x00;
pub const BATTERY_CHARGE_INPROG: c_uint = 0x10;
pub const BATTERY_CHARGE_FAIL: c_uint = 0x20;
pub const BATTERY_CYCLES_EXCEEDED: c_uint = 0x40;
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
pub const MAX_LOGICAL_DRIVES_8LD: c_int = 8;
pub const MAX_LOGICAL_DRIVES_40LD: c_int = 40;
pub const FC_MAX_PHYSICAL_DEVICES: c_int = 256;
pub const MAX_MBOX_CHANNELS: c_int = 5;
pub const MAX_MBOX_TARGET: c_int = 15;

pub const MAX_ROW_SIZE_40LD: c_int = 32;
pub const MAX_ROW_SIZE_8LD: c_int = 8;
pub const SPAN_DEPTH_8_SPANS: c_int = 8;
pub const SPAN_DEPTH_4_SPANS: c_int = 4;
pub const MAX_REQ_SENSE_LEN: c_uint = 0x20;
//
// struct mbox_t - Driver and f/w handshake structure.
// @cmd		: firmware command
// @cmdid	: command id
// @numsectors	: number of sectors to be transferred
// @lba		: Logical Block Address on LD
// @xferaddr	: DMA address for data transfer
// @logdrv	: logical drive number
// @numsge	: number of scatter gather elements in sg list
// @resvd	: reserved
// @busy	: f/w busy, must wait to issue more commands.
// @numstatus	: number of commands completed.
// @status	: status of the commands completed
// @completed	: array of completed command ids.
// @poll	: poll and ack sequence
// @ack		: poll and ack sequence
//
// The central handshake structure between the driver and the firmware. This
// structure must be allocated by the driver and aligned at 8-byte boundary.
//
pub const MBOX_MAX_FIRMWARE_STATUS: c_int = 46;
//
// mbox64_t - 64-bit extension for the mailbox
// @segment_lo	: the low 32-bits of the address of the scatter-gather list
// @segment_hi	: the upper 32-bits of the address of the scatter-gather list
// @mbox	: 32-bit mailbox, whose xferadder field must be set to
// 0xFFFFFFFF
//
// This is the extension of the 32-bit mailbox to be able to perform DMA
// beyond 4GB address range.
//
// mailbox structure used for internal commands
//
// mraid_passthru_t - passthru structure to issue commands to physical devices
// @timeout		: command timeout, 0=6sec, 1=60sec, 2=10min, 3=3hr
// @ars			: set if ARS required after check condition
// @islogical		: set if command meant for logical devices
// @logdrv		: logical drive number if command for LD
// @channel		: Channel on which physical device is located
// @target		: SCSI target of the device
// @queuetag		: unused
// @queueaction		: unused
// @cdb			: SCSI CDB
// @cdblen		: length of the CDB
// @reqsenselen		: amount of request sense data to be returned
// @reqsensearea	: Sense information buffer
// @numsge		: number of scatter-gather elements in the sg list
// @scsistatus		: SCSI status of the command completed.
// @dataxferaddr	: DMA data transfer address
// @dataxferlen		: amount of the data to be transferred.
//
// mraid_epassthru_t - passthru structure to issue commands to physical devices
// @timeout		: command timeout, 0=6sec, 1=60sec, 2=10min, 3=3hr
// @ars			: set if ARS required after check condition
// @rsvd1		: reserved field
// @cd_rom		: (?)
// @rsvd2		: reserved field
// @islogical		: set if command meant for logical devices
// @logdrv		: logical drive number if command for LD
// @channel		: Channel on which physical device is located
// @target		: SCSI target of the device
// @queuetag		: unused
// @queueaction		: unused
// @cdblen		: length of the CDB
// @rsvd3		: reserved field
// @cdb			: SCSI CDB
// @numsge		: number of scatter-gather elements in the sg list
// @status		: SCSI status of the command completed.
// @reqsenselen		: amount of request sense data to be returned
// @reqsensearea	: Sense information buffer
// @rsvd4		: reserved field
// @dataxferaddr	: DMA data transfer address
// @dataxferlen		: amount of the data to be transferred.
//
// mraid_pinfo_t - product info, static information about the controller
// @data_size		: current size in bytes (not including resvd)
// @config_signature	: Current value is 0x00282008
// @fw_version		: Firmware version
// @bios_version	: version of the BIOS
// @product_name	: Name given to the controller
// @max_commands	: Maximum concurrent commands supported
// @nchannels		: Number of SCSI Channels detected
// @fc_loop_present	: Number of Fibre Loops detected
// @mem_type		: EDO, FPM, SDRAM etc
// @signature		:
// @dram_size		: In terms of MB
// @subsysid		: device PCI subsystem ID
// @subsysvid		: device PCI subsystem vendor ID
// @notify_counters	:
// @pad1k		: 135 + 889 resvd = 1024 total size
//
// This structures holds the information about the controller which is not
// expected to change dynamically.
//
// The current value of config signature is 0x00282008:
// 0x28 = MAX_LOGICAL_DRIVES,
// 0x20 = Number of stripes and
// 0x08 = Number of spans
//
// mraid_notify_t - the notification structure
// @global_counter		: Any change increments this counter
// @param_counter		: Indicates any params changed
// @param_id			: Param modified - defined below
// @param_val			: New val of last param modified
// @write_config_counter	: write config occurred
// @write_config_rsvd		:
// @ldrv_op_counter		: Indicates ldrv op started/completed
// @ldrv_opid			: ldrv num
// @ldrv_opcmd			: ldrv operation - defined below
// @ldrv_opstatus		: status of the operation
// @ldrv_state_counter		: Indicates change of ldrv state
// @ldrv_state_id		: ldrv num
// @ldrv_state_new		: New state
// @ldrv_state_old		: old state
// @pdrv_state_counter		: Indicates change of ldrv state
// @pdrv_state_id		: pdrv id
// @pdrv_state_new		: New state
// @pdrv_state_old		: old state
// @pdrv_fmt_counter		: Indicates pdrv format started/over
// @pdrv_fmt_id			: pdrv id
// @pdrv_fmt_val		: format started/over
// @pdrv_fmt_rsvd		:
// @targ_xfer_counter		: Indicates SCSI-2 Xfer rate change
// @targ_xfer_id		: pdrv Id
// @targ_xfer_val		: new Xfer params of last pdrv
// @targ_xfer_rsvd		:
// @fcloop_id_chg_counter	: Indicates loopid changed
// @fcloopid_pdrvid		: pdrv id
// @fcloop_id0			: loopid on fc loop 0
// @fcloop_id1			: loopid on fc loop 1
// @fcloop_state_counter	: Indicates loop state changed
// @fcloop_state0		: state of fc loop 0
// @fcloop_state1		: state of fc loop 1
// @fcloop_state_rsvd		:
//
// mraid_inquiry3_t - enquiry for device information
//
// @data_size		: current size in bytes (not including resvd)
// @notify		:
// @notify_rsvd		:
// @rebuild_rate	: rebuild rate (0% - 100%)
// @cache_flush_int	: cache flush interval in seconds
// @sense_alert		:
// @drive_insert_count	: drive insertion count
// @battery_status	:
// @num_ldrv		: no. of Log Drives configured
// @recon_state		: state of reconstruct
// @ldrv_op_status	: logdrv Status
// @ldrv_size		: size of each log drv
// @ldrv_prop		:
// @ldrv_state		: state of log drives
// @pdrv_state		: state of phys drvs.
// @pdrv_format		:
// @targ_xfer		: phys device transfer rate
// @pad1k		: 761 + 263reserved = 1024 bytes total size
//
pub const MAX_NOTIFY_SIZE: c_uint = 0x80;

//
// mraid_adapinfo_t - information about the adapter
// @max_commands		: max concurrent commands supported
// @rebuild_rate		: rebuild rate - 0% thru 100%
// @max_targ_per_chan		: max targ per channel
// @nchannels			: number of channels on HBA
// @fw_version			: firmware version
// @age_of_flash		: number of times FW has been flashed
// @chip_set_value		: contents of 0xC0000832
// @dram_size			: in MB
// @cache_flush_interval	: in seconds
// @bios_version		:
// @board_type			:
// @sense_alert			:
// @write_config_count		: increase with every configuration change
// @drive_inserted_count	: increase with every drive inserted
// @inserted_drive		: channel:Id of inserted drive
// @battery_status		: bit 0: battery module missing
// bit 1: VBAD
// bit 2: temperature high
// bit 3: battery pack missing
// bit 4,5:
// 00 - charge complete
// 01 - fast charge in progress
// 10 - fast charge fail
// 11 - undefined
// bit 6: counter > 1000
// bit 7: Undefined
// @dec_fault_bus_info		:
//
// mraid_ldrv_info_t - information about the logical drives
// @nldrv	: Number of logical drives configured
// @rsvd	:
// @size	: size of each logical drive
// @prop	:
// @state	: state of each logical drive
//
// mraid_pdrv_info_t - information about the physical drives
// @pdrv_state	: state of each physical drive
//
// mraid_inquiry_t - RAID inquiry, mailbox command 0x05
// @mraid_adapinfo_t	: adapter information
// @mraid_ldrv_info_t	: logical drives information
// @mraid_pdrv_info_t	: physical drives information
//
// mraid_extinq_t - RAID extended inquiry, mailbox command 0x04
//
// @raid_inq		: raid inquiry
// @phys_drv_format	:
// @stack_attn		:
// @modem_status	:
// @rsvd		:
//
// adap_device_t - device information
// @channel	: channel fpor the device
// @target	: target ID of the device
//
// adap_span_40ld_t - 40LD span
// @start_blk	: starting block
// @num_blks	: number of blocks
//
// adap_span_8ld_t - 8LD span
// @start_blk	: starting block
// @num_blks	: number of blocks
//
// logdrv_param_t - logical drives parameters
//
// @span_depth	: total number of spans
// @level	: RAID level
// @read_ahead	: read ahead, no read ahead, adaptive read ahead
// @stripe_sz	: encoded stripe size
// @status	: status of the logical drive
// @write_mode	: write mode, write_through/write_back
// @direct_io	: direct io or through cache
// @row_size	: number of stripes in a row
//
// logdrv_40ld_t - logical drive definition for 40LD controllers
// @lparam	: logical drives parameters
// @span	: span
//
// logdrv_8ld_span8_t - logical drive definition for 8LD controllers
// @lparam	: logical drives parameters
// @span	: span
//
// 8-LD logical drive with up to 8 spans
//
// logdrv_8ld_span4_t - logical drive definition for 8LD controllers
// @lparam	: logical drives parameters
// @span	: span
//
// 8-LD logical drive with up to 4 spans
//
// phys_drive_t - physical device information
// @type	: Type of the device
// @cur_status	: current status of the device
// @tag_depth	: Level of tagging
// @sync_neg	: sync negotiation - ENABLE or DISABLE
// @size	: configurable size in terms of 512 byte
//
// disk_array_40ld_t - disk array for 40LD controllers
// @numldrv	: number of logical drives
// @resvd	:
// @ldrv	: logical drives information
// @pdrv	: physical drives information
//
// disk_array_8ld_span8_t - disk array for 8LD controllers
// @numldrv	: number of logical drives
// @resvd	:
// @ldrv	: logical drives information
// @pdrv	: physical drives information
//
// Disk array for 8LD logical drives with up to 8 spans
//
// disk_array_8ld_span4_t - disk array for 8LD controllers
// @numldrv	: number of logical drives
// @resvd	:
// @ldrv	: logical drives information
// @pdrv	: physical drives information
//
// Disk array for 8LD logical drives with up to 4 spans
//
// struct private_bios_data - bios private data for boot devices
// @geometry	: bits 0-3 - BIOS geometry, 0x0001 - 1GB, 0x0010 - 2GB,
// 0x1000 - 8GB, Others values are invalid
// @unused	: bits 4-7 are unused
// @boot_drv	: logical drive set as boot drive, 0..7 - for 8LD cards,
// 0..39 - for 40LD cards
// @cksum	: 0-(sum of first 13 bytes of this structure)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct private_bios_data {
    pub :4: uint8_t geometry,
    pub :4: uint8_t unused,
    pub boot_drv: u8,
    pub rsvd: [u8; 12],
    pub cksum: u16,
// C attribute field omitted
//
// mbox_sgl64 - 64-bit scatter list for mailbox based controllers
// @address	: address of the buffer
// @length	: data transfer length
//
    pub address: u64,
    pub length: u32,
// C attribute field omitted
//
// mbox_sgl32 - 32-bit scatter list for mailbox based controllers
// @address	: address of the buffer
// @length	: data transfer length
//
    pub address: u32,
    pub length: u32,
// C attribute field omitted
