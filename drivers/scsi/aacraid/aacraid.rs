//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aacraid/aacraid.h
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
// Adaptec AAC series RAID controller driver
// (c) Copyright 2001 Red Hat Inc.	<alan@redhat.com>
//
// based on the old aacraid driver that is..
// Adaptec aacraid device driver for Linux.
//
// Copyright (c) 2000-2010 Adaptec, Inc.
// 2010-2015 PMC-Sierra, Inc. (aacraid@pmc-sierra.com)
// 2016-2017 Microsemi Corp. (aacraid@microsemi.com)
//
// Module Name:
// aacraid.h
//
// Abstract: Contains all routines for control of the aacraid driver
//

// eg: if (nblank(dprintk(x)))

// ------------------------------------------------------------------------------
// D E F I N E S
// ----------------------------------------------------------------------------

pub const AAC_PCI_MSI_ENABLE: c_uint = 0x8000;

pub const AAC_INT_ENABLE_TYPE1_INTX: c_uint = 0xfffffffb;
pub const AAC_INT_ENABLE_TYPE1_MSIX: c_uint = 0xfffffffa;
pub const AAC_INT_DISABLE_ALL: c_uint = 0xffffffff;
// Bit definitions in IOA->Host Interrupt Register

pub const PMC_GLOBAL_INT_BIT2: c_uint = 0x00000004;
pub const PMC_GLOBAL_INT_BIT0: c_uint = 0x00000001;

pub const MAXIMUM_NUM_CONTAINERS: c_int = 32;
pub const AAC_NUM_MGT_FIB: c_int = 8;

pub const AAC_MAX_LUN: c_int = 256;

// Macro flag: #define AAC_DEBUG_INSTRUMENT_AIF_DELETE
pub const AAC_MAX_NATIVE_TARGETS: c_int = 1024;
// Thor: 5 phys. buses: #0: empty, 1-4: 256 targets each
pub const AAC_MAX_BUSES: c_int = 5;
pub const AAC_MAX_TARGETS: c_int = 256;

pub const AAC_MAX_NATIVE_SIZE: c_int = 2048;
pub const FW_ERROR_BUFFER_SIZE: c_int = 512;
pub const AAC_SA_TIMEOUT: c_int = 180;
pub const AAC_ARC_TIMEOUT: c_int = 60;

// Thor AIF events

pub const HBA_MAX_SG_EMBEDDED: c_int = 28;
pub const HBA_MAX_SG_SEPARATE: c_int = 90;
pub const HBA_SENSE_DATA_LEN_MAX: c_int = 32;
pub const HBA_REQUEST_TAG_ERROR_FLAG: c_uint = 0x00000002;
pub const HBA_SGL_FLAGS_EXT: c_uint = 0x80000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_sgl {
    pub /: *mut *mut u32 addr_lo; / Lower 32-bits of SGL element address,
    pub /: *mut *mut u32 addr_hi; / Upper 32-bits of SGL element address,
    pub /: *mut *mut u32 len; / Length of SGL element in bytes,
    pub /: *mut *mut u32 flags; / SGL element flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_cmd_req {
    pub /: *mut *mut u8 iu_type; / HBA information unit type,
//
// byte1:
// [1:0] DIR - 0=No data, 0x1 = IN, 0x2 = OUT
// [2]   TYPE - 0=PCI, 1=DDR
// [3]   CRYPTO_ENABLE - 0=Crypto disabled, 1=Crypto enabled
//
    pub byte1: u8,
    pub /: *mut *mut u8 reply_qid; / Host reply queue to post response to,
    pub reserved1: u8,
    pub /: *mut *mut __le32 it_nexus; / Device handle for the request,
    pub /: *mut *mut __le32 request_id; / Sender context,
// Lower 32-bits of tweak value for crypto enabled IOs
    pub tweak_value_lo: __le32,
    pub /: *mut *mut u8 cdb[16]; / SCSI CDB of the command,
    pub /: *mut *mut u8 lun[8]; / SCSI LUN of the command,
// Total data length in bytes to be read/written (if any)
    pub data_length: __le32,
// [2:0] Task Attribute, [6:3] Command Priority
    pub attr_prio: u8,
// Number of SGL elements embedded in the HBA req
    pub emb_data_desc_count: u8,
    pub /: *mut *mut __le16 dek_index; / DEK index for crypto enabled IOs,
// Lower 32-bits of reserved error data target location on the host
    pub error_ptr_lo: __le32,
// Upper 32-bits of reserved error data target location on the host
    pub error_ptr_hi: __le32,
// Length of reserved error data area on the host in bytes
    pub error_length: __le32,
// Upper 32-bits of tweak value for crypto enabled IOs
    pub tweak_value_hi: __le32,
    pub /: *mut *mut aac_hba_sgl sge[HBA_MAX_SG_SEPARATE+2]; / SG list space,
//
// structure must not exceed
// AAC_MAX_NATIVE_SIZE-FW_ERROR_BUFFER_SIZE
//
}

// Task Management Functions (TMF)
pub const HBA_TMF_ABORT_TASK: c_uint = 0x01;
pub const HBA_TMF_LUN_RESET: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_tm_req {
    pub /: *mut *mut u8 iu_type; / HBA information unit type,
    pub /: *mut *mut u8 reply_qid; / Host reply queue to post response to,
    pub /: *mut *mut u8 tmf; / Task management function,
    pub reserved1: u8,
    pub /: *mut *mut __le32 it_nexus; / Device handle for the command,
    pub /: *mut *mut u8 lun[8]; / SCSI LUN,
// Used to hold sender context.
    pub /: *mut *mut __le32 request_id; / Sender context,
    pub reserved2: __le32,
// Request identifier of managed task
    pub /: *mut *mut __le32 managed_request_id; / Sender context being managed,
    pub reserved3: __le32,
// Lower 32-bits of reserved error data target location on the host
    pub error_ptr_lo: __le32,
// Upper 32-bits of reserved error data target location on the host
    pub error_ptr_hi: __le32,
// Length of reserved error data area on the host in bytes
    pub error_length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_reset_req {
    pub /: *mut *mut u8 iu_type; / HBA information unit type,
// 0 - reset specified device, 1 - reset all devices
    pub reset_type: u8,
    pub /: *mut *mut u8 reply_qid; / Host reply queue to post response to,
    pub reserved1: u8,
    pub /: *mut *mut __le32 it_nexus; / Device handle for the command,
    pub /: *mut *mut __le32 request_id; / Sender context,
// Lower 32-bits of reserved error data target location on the host
    pub error_ptr_lo: __le32,
// Upper 32-bits of reserved error data target location on the host
    pub error_ptr_hi: __le32,
// Length of reserved error data area on the host in bytes
    pub error_length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_resp {
    pub /: *mut *mut u8 iu_type; / HBA information unit type,
    pub reserved1: [u8; 3],
    pub /: *mut *mut __le32 request_identifier; / sender context,
    pub reserved2: __le32,
    pub /: *mut *mut u8 service_response; / SCSI service response,
    pub /: *mut *mut u8 status; / SCSI status,
    pub /: *mut *mut u8 datapres; / [1:0] - data present, [7:2] - reserved,
    pub /: *mut *mut u8 sense_response_data_len; / Sense/response data length,
    pub /: *mut *mut __le32 residual_count; / Residual data length in bytes,
// Sense/response data
    pub sense_response_buf: [u8; HBA_SENSE_DATA_LEN_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_native_hba {
    pub cmd: aac_hba_cmd_req,
    pub tmr: aac_hba_tm_req,
    pub cmd_bytes: [u8; AAC_MAX_NATIVE_SIZE-FW_ERROR_BUFFER_SIZE],
    pub cmd: },
    pub err: aac_hba_resp,
    pub resp_bytes: [u8; FW_ERROR_BUFFER_SIZE],
    pub resp: },
}

pub const CISS_REPORT_PHYSICAL_LUNS: c_uint = 0xc3;
pub const WRITE_HOST_WELLNESS: c_uint = 0xa5;
pub const CISS_IDENTIFY_PHYSICAL_DEVICE: c_uint = 0x15;
pub const BMIC_IN: c_uint = 0x26;
pub const BMIC_OUT: c_uint = 0x27;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_ciss_phys_luns_resp {
    pub /: *mut *mut u8 list_length[4]; / LUN list length (N-7, big endian),
    pub /: *mut *mut u8 resp_flag; / extended response_flag,
    pub reserved: [u8; 3],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ciss_lun {
    pub /: *mut *mut u8 tid[3]; / Target ID,
    pub /: *mut *mut u8 bus; / Bus, flag (bits 6,7),
    pub level3: [u8; 2],
    pub level2: [u8; 2],
    pub /: *mut *mut u8 node_ident[16]; / phys. node identifier,
    pub /: *mut *mut } lun[]; / List of phys. devices,
}

//
// Interrupts
//
pub const AAC_MAX_HRRQ: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_ciss_identify_pd {
    pub /: *mut *mut u8 scsi_bus; / SCSI Bus number on controller,
    pub /: *mut *mut u8 scsi_id; / SCSI ID on this bus,
    pub /: *mut *mut u16 block_size; / sector size in bytes,
    pub /: *mut *mut u32 total_blocks; / number for sectors on drive,
    pub /: *mut *mut u32 reserved_blocks; / controller reserved (RIS),
    pub /: *mut *mut u8 model[40]; / Physical Drive Model,
    pub /: *mut *mut u8 serial_number[40]; / Drive Serial Number,
    pub /: *mut *mut u8 firmware_revision[8]; / drive firmware revision,
    pub /: *mut *mut u8 scsi_inquiry_bits; / inquiry byte 7 bits,
    pub /: *mut *mut u8 compaq_drive_stamp; / 0 means drive not stamped,
    pub last_failure_reason: u8,
    pub flags: u8,
    pub more_flags: u8,
    pub /: *mut *mut u8 scsi_lun; / SCSI LUN for phys drive,
    pub yet_more_flags: u8,
    pub even_more_flags: u8,
    pub /: *mut *mut u32 spi_speed_rules; / SPI Speed :Ultra disable diagnose,
    pub /: *mut *mut u8 phys_connector[2]; / connector number on controller,
    pub /: *mut *mut u8 phys_box_on_bus; / phys enclosure this drive resides,
    pub /: *mut *mut u8 phys_bay_in_box; / phys drv bay this drive resides,
    pub /: *mut *mut u32 rpm; / Drive rotational speed in rpm,
    pub /: *mut *mut u8 device_type; / type of drive,
    pub /: *mut *mut u8 sata_version; / only valid when drive_type is SATA,
    pub big_total_block_count: u64,
    pub ris_starting_lba: u64,
    pub ris_size: u32,
    pub wwid: [u8; 20],
    pub controller_phy_map: [u8; 32],
    pub phy_count: u16,
    pub phy_connected_dev_type: [u8; 256],
    pub phy_to_drive_bay_num: [u8; 256],
    pub phy_to_attached_dev_index: [u16; 256],
    pub box_index: u8,
    pub spitfire_support: u8,
    pub extra_physical_drive_flags: u16,
    pub negotiated_link_rate: [u8; 256],
    pub phy_to_phy_map: [u8; 256],
    pub redundant_path_present_map: u8,
    pub redundant_path_failure_map: u8,
    pub active_path_number: u8,
    pub alternate_paths_phys_connector: [u16; 8],
    pub alternate_paths_phys_box_on_port: [u8; 8],
    pub multi_lun_device_lun_count: u8,
    pub minimum_good_fw_revision: [u8; 8],
    pub unique_inquiry_bytes: [u8; 20],
    pub current_temperature_degreesC: u8,
    pub temperature_threshold_degreesC: u8,
    pub max_temperature_degreesC: u8,
    pub /: *mut *mut *mut u8 logical_blocks_per_phys_block_exp; / phyblocksize = 512  2^exp,
    pub current_queue_depth_limit: u16,
    pub switch_name: [u8; 10],
    pub switch_port: u16,
    pub alternate_paths_switch_name: [u8; 40],
    pub alternate_paths_switch_port: [u8; 8],
    pub /: *mut *mut u16 power_on_hours; / valid only if gas gauge supported,
    pub /: *mut *mut u16 percent_endurance_used; / valid only if gas gauge supported.,
    pub drive_authentication: u8,
    pub smart_carrier_authentication: u8,
    pub smart_carrier_app_fw_version: u8,
    pub smart_carrier_bootloader_fw_version: u8,
    pub SanitizeSecureEraseSupport: u8,
    pub DriveKeyFlags: u8,
    pub encryption_key_name: [u8; 64],
    pub misc_drive_flags: u32,
    pub dek_index: u16,
    pub drive_encryption_flags: u16,
    pub sanitize_maximum_time: [u8; 6],
    pub connector_info_mode: u8,
    pub connector_info_number: [u8; 4],
    pub long_connector_name: [u8; 64],
    pub device_unique_identifier: [u8; 16],
    pub padto_2K: [u8; 17],
    pub __packed: },
//
// These macros convert from physical channels to virtual channels
//

pub const PMC_DEVICE_S6: c_uint = 0x28b;
pub const PMC_DEVICE_S7: c_uint = 0x28c;
pub const PMC_DEVICE_S8: c_uint = 0x28d;

//
// These macros are for keeping track of
// character device state.
//

// #define AAC_DETAILED_STATUS_INFO
    pub heads: c_int,
    pub sectors: c_int,
    pub cylinders: c_int,
}

//
// Firmware constants
//
pub const CT_NONE: c_int = 0;
pub const CT_OK: c_int = 218;

//
// Host side memory scatter gather list
// Used by the adapter for read, write, and readdirplus operations
// We have separate 32 and 64 bit version because even
// on 64 bit systems not all cards support the 64 bit version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgentry {
    pub /: *mut *mut __le32 addr; / 32-bit address.,
    pub /: *mut *mut __le32 count; / Length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sgentry {
    pub /: *mut *mut u32 addr; / 32-bit address.,
    pub /: *mut *mut u32 count; / Length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgentry64 {
    pub /: *mut *mut __le32 addr[2]; / 64-bit addr. 2 pieces for data alignment,
    pub /: *mut *mut __le32 count; / Length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sgentry64 {
    pub /: *mut *mut u32 addr[2]; / 64-bit addr. 2 pieces for data alignment,
    pub /: *mut *mut u32 count; / Length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgentryraw {
    pub /: *mut *mut __le32 next; / reserved for F/W use,
    pub /: *mut *mut __le32 prev; / reserved for F/W use,
    pub addr: [__le32; 2],
    pub count: __le32,
    pub /: *mut *mut __le32 flags; / reserved for F/W use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sgentryraw {
    pub /: *mut *mut u32 next; / reserved for F/W use,
    pub /: *mut *mut u32 prev; / reserved for F/W use,
    pub addr: [u32; 2],
    pub count: u32,
    pub /: *mut *mut u32 flags; / reserved for F/W use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_ieee1212 {
    pub addrLow: u32,
    pub addrHigh: u32,
    pub length: u32,
    pub flags: u32,
}

//
// SGMAP
//
// This is the SGMAP structure for all commands that use
// 32-bit addressing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgmap {
    pub count: __le32,
    pub sg: [sgentry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sgmap {
    pub count: u32,
    pub sg: [user_sgentry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgmap64 {
    pub count: __le32,
    pub sg: [sgentry64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sgmap64 {
    pub count: u32,
    pub sg: [user_sgentry64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgmapraw {
    pub count: __le32,
    pub sg: [sgentryraw; ],
}

// 2 = API
//
// unsigned	Month		:4;	// 1 - 12
// unsigned	Day		:6;	// 1 - 32
// unsigned	Hour		:6;	// 0 - 23
// unsigned	Minute		:6;	// 0 - 60
// unsigned	Second		:6;	// 0 - 60
//
// Define all the constants needed for the communication interface
//
// Define how many queue entries each queue will have and the total
// number of entries for the entire communication interface. Also define
// how many queues we support.
//
// This has to match the controller
//

pub const HOST_HIGH_CMD_ENTRIES: c_int = 4;
pub const HOST_NORM_CMD_ENTRIES: c_int = 8;
pub const ADAP_HIGH_CMD_ENTRIES: c_int = 4;
pub const ADAP_NORM_CMD_ENTRIES: c_int = 512;
pub const HOST_HIGH_RESP_ENTRIES: c_int = 4;
pub const HOST_NORM_RESP_ENTRIES: c_int = 512;
pub const ADAP_HIGH_RESP_ENTRIES: c_int = 4;
pub const ADAP_NORM_RESP_ENTRIES: c_int = 8;

//
// Set the queues on a 16 byte alignment
//
pub const QUEUE_ALIGNMENT: c_int = 16;
//
// The queue headers define the Communication Region queues. These
// are physically contiguous and accessible by both the adapter and the
// host. Even though all queue headers are in the same contiguous block
// they will be represented as individual units in the data structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_entry {
    pub /: *mut *mut __le32 size; / Size in bytes of Fib which this QE points to,
    pub /: *mut *mut __le32 addr; / Receiver address of the FIB,
}

//
// The adapter assumes the ProducerIndex and ConsumerIndex are grouped
// adjacently and in that order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_qhdr {
    pub access: *mut *mut __le64 header_addr;/ Address to hand the adapter to,
    pub /: *mut *mut *mut __le32 producer; / The producer index for this queue (host address),
    pub /: *mut *mut *mut __le32 consumer; / The consumer index for this queue (host address),
}

//
// Define all the events which the adapter would like to notify
// the host of.
//

pub const AdapNormRespNotFull: c_int = 5;
pub const AdapHighRespNotFull: c_int = 6;
pub const AdapNormCmdNotFull: c_int = 7;
pub const AdapHighCmdNotFull: c_int = 8;
pub const SynchCommandComplete: c_int = 9;
pub const AdapInternalError: c_uint = 0xfe    /* The adapter detected an internal error shutting down */;
//
// Define all the events the host wishes to notify the
// adapter of. The first four values much match the Qid the
// corresponding queue.
//
pub const AdapNormCmdQue: c_int = 2;
pub const AdapHighCmdQue: c_int = 3;
pub const AdapNormRespQue: c_int = 6;
pub const AdapHighRespQue: c_int = 7;
pub const HostShutdown: c_int = 8;
pub const HostPowerFail: c_int = 9;
pub const FatalCommError: c_int = 10;
pub const HostNormRespNotFull: c_int = 11;
pub const HostHighRespNotFull: c_int = 12;
pub const HostNormCmdNotFull: c_int = 13;
pub const HostHighCmdNotFull: c_int = 14;
pub const FastIo: c_int = 15;
pub const AdapPrintfDone: c_int = 16;
//
// Define all the queues that the adapter and host use to communicate
// Number them to match the physical queue layout.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aac_queue_types {
    HostNormCmdQueue = 0,	/* Adapter to host normal priority command traffic */
    HostHighCmdQueue,	/* Adapter to host high priority command traffic */
    AdapNormCmdQueue,	/* Host to adapter normal priority command traffic */
    AdapHighCmdQueue,	/* Host to adapter high priority command traffic */
    HostNormRespQueue,	/* Adapter to host normal priority response traffic */
    HostHighRespQueue,	/* Adapter to host high priority response traffic */
    AdapNormRespQueue,	/* Host to adapter normal priority response traffic */
    AdapHighRespQueue	/* Host to adapter high priority response traffic */
}

//
// Assign type values to the FSA communication data structures
//
pub const FIB_MAGIC: c_uint = 0x0001;
pub const FIB_MAGIC2: c_uint = 0x0004;
pub const FIB_MAGIC2_64: c_uint = 0x0005;
//
// Define the priority levels the FSA communication routines support.
//
pub const FsaNormal: c_int = 1;
// transport FIB header (PMC)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_fib_xporthdr {
    pub /: *mut *mut __le64 HostAddress; / FIB host address w/o xport header,
    pub /: *mut *mut __le32 Size; / FIB size excluding xport header,
    pub /: *mut *mut __le32 Handle; / driver handle to reference the FIB,
    pub Reserved: [__le64; 2],
}

pub const ALIGN32: c_int = 32;
//
// Define the FIB. The FIB is the where all the requested data and
// command information are put to the application on the FSA adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_fibhdr {
    pub /: *mut *mut __le32 XferState; / Current transfer state for this CCB,
    pub /: *mut *mut __le16 Command; / Routing information for the destination,
    pub /: *mut *mut u8 StructType; / Type FIB,
    pub /: *mut *mut u8 Unused; / Unused,
    pub /: *mut *mut __le16 Size; / Size of this FIB in bytes,
    pub sender: *mut *mut __le16 SenderSize; / Size of the FIB in the,
    pub /: *mut *mut __le32 SenderFibAddress; / Host defined data in the FIB,
    pub for: *mut *mut __le32 ReceiverFibAddress;/ Logical address of this FIB,
    pub /: *mut *mut __le32 SenderFibAddressHigh;/ upper 32bit of phys. FIB address,
    pub /: *mut *mut __le32 TimeStamp; / otherwise timestamp for FW internal use,
    pub u: },
    pub /: *mut *mut __le32 Handle; / FIB handle used for MSGU commnunication,
    pub /: *mut *mut u32 Previous; / FW internal use,
    pub /: *mut *mut u32 Next; / FW internal use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_fib {
    pub header: aac_fibhdr,
    pub data: u8 data[512-sizeof(struct aac_fibhdr)]; // Command specific,
}

//
// FIB commands
//
pub const TestCommandResponse: c_int = 1;
pub const TestAdapterCommand: c_int = 2;
//
// Lowlevel and comm commands
//
pub const LastTestCommand: c_int = 100;
pub const ReinitHostNormCommandQueue: c_int = 101;
pub const ReinitHostHighCommandQueue: c_int = 102;
pub const ReinitHostHighRespQueue: c_int = 103;
pub const ReinitHostNormRespQueue: c_int = 104;
pub const ReinitAdapNormCommandQueue: c_int = 105;
pub const ReinitAdapHighCommandQueue: c_int = 107;
pub const ReinitAdapHighRespQueue: c_int = 108;
pub const ReinitAdapNormRespQueue: c_int = 109;
pub const InterfaceShutdown: c_int = 110;
pub const DmaCommandFib: c_int = 120;
pub const StartProfile: c_int = 121;
pub const TermProfile: c_int = 122;
pub const SpeedTest: c_int = 123;
pub const TakeABreakPt: c_int = 124;
pub const RequestPerfData: c_int = 125;
pub const SetInterruptDefTimer: c_int = 126;
pub const SetInterruptDefCount: c_int = 127;
pub const GetInterruptDefStatus: c_int = 128;
pub const LastCommCommand: c_int = 129;
//
// Filesystem commands
//
pub const NuFileSystem: c_int = 300;
pub const UFS: c_int = 301;
pub const HostFileSystem: c_int = 302;
pub const LastFileSystemCommand: c_int = 303;
//
// Container Commands
//
pub const ContainerCommand: c_int = 500;
pub const ContainerCommand64: c_int = 501;
pub const ContainerRawIo: c_int = 502;
pub const ContainerRawIo2: c_int = 503;
//
// Scsi Port commands (scsi passthrough)
//
pub const ScsiPortCommand: c_int = 600;
pub const ScsiPortCommand64: c_int = 601;
//
// Misc house keeping and generic adapter initiated commands
//
pub const AifRequest: c_int = 700;
pub const CheckRevision: c_int = 701;
pub const FsaHostShutdown: c_int = 702;
pub const RequestAdapterInfo: c_int = 703;
pub const IsAdapterPaused: c_int = 704;
pub const SendHostTime: c_int = 705;
pub const RequestSupplementAdapterInfo: c_int = 706;
pub const LastMiscCommand: c_int = 707;
//
// Commands that will target the failover level on the FSA adapter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fib_xfer_state {
    HostOwned			= (1<<0),
    AdapterOwned			= (1<<1),
    FibInitialized			= (1<<2),
    FibEmpty			= (1<<3),
    AllocatedFromPool		= (1<<4),
    SentFromHost			= (1<<5),
    SentFromAdapter			= (1<<6),
    ResponseExpected		= (1<<7),
    NoResponseExpected		= (1<<8),
    AdapterProcessed		= (1<<9),
    HostProcessed			= (1<<10),
    HighPriority			= (1<<11),
    NormalPriority			= (1<<12),
    Async				= (1<<13),
    AsyncIo				= (1<<13),	// rpbfix: remove with new regime
    PageFileIo			= (1<<14),	// rpbfix: remove with new regime
    ShutdownRequest			= (1<<15),
    LazyWrite			= (1<<16),	// rpbfix: remove with new regime
    AdapterMicroFib			= (1<<17),
    BIOSFibPath			= (1<<18),
    FastResponseCapable		= (1<<19),
    ApiFib				= (1<<20),	/* Its an API Fib */
// PMC NEW COMM: There is no more AIF data pending
    NoMoreAifDataAvailable		= (1<<21)
}

//
// The following defines needs to be updated any time there is an
// incompatible change made to the aac_init structure.
//
pub const ADAPTER_INIT_STRUCT_REVISION: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _r7 {
    pub init_struct_revision: __le32,
    pub no_of_msix_vectors: __le32,
    pub fsrev: __le32,
    pub comm_header_address: __le32,
    pub fast_io_comm_area_address: __le32,
    pub adapter_fibs_physical_address: __le32,
    pub adapter_fibs_virtual_address: __le32,
    pub adapter_fibs_size: __le32,
    pub adapter_fib_align: __le32,
    pub printfbuf: __le32,
    pub printfbufsiz: __le32,
// number of 4k pages of host phys. mem.
    pub host_phys_mem_pages: __le32,
// number of seconds since 1970.
    pub host_elapsed_seconds: __le32,
// ADAPTER_INIT_STRUCT_REVISION_4 begins here
    pub /: *mut *mut __le32 init_flags; / flags for supported features,
pub const INITFLAGS_NEW_COMM_SUPPORTED: c_uint = 0x00000001;
pub const INITFLAGS_DRIVER_USES_UTC_TIME: c_uint = 0x00000010;
pub const INITFLAGS_DRIVER_SUPPORTS_PM: c_uint = 0x00000020;
pub const INITFLAGS_NEW_COMM_TYPE1_SUPPORTED: c_uint = 0x00000040;
pub const INITFLAGS_FAST_JBOD_SUPPORTED: c_uint = 0x00000080;
pub const INITFLAGS_NEW_COMM_TYPE2_SUPPORTED: c_uint = 0x00000100;
pub const INITFLAGS_DRIVER_SUPPORTS_HBA_MODE: c_uint = 0x00000400;
    pub /: *mut *mut __le32 max_io_commands; / max outstanding commands,
    pub /: *mut *mut __le32 max_io_size; / largest I/O command,
    pub /: *mut *mut __le32 max_fib_size; / largest FIB to adapter,
// ADAPTER_INIT_STRUCT_REVISION_5 begins here
    pub /: *mut *mut __le32 max_num_aif; / max number of aif,
// ADAPTER_INIT_STRUCT_REVISION_6 begins here
// Host RRQ (response queue) for SRC
    pub host_rrq_addr_low: __le32,
    pub host_rrq_addr_high: __le32,
    pub r7: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _r8 {
// ADAPTER_INIT_STRUCT_REVISION_8
    pub init_struct_revision: __le32,
    pub rr_queue_count: __le32,
    pub /: *mut *mut __le32 host_elapsed_seconds; / number of secs since 1970.,
    pub init_flags: __le32,
    pub /: *mut *mut __le32 max_io_size; / largest I/O command,
    pub /: *mut *mut __le32 max_num_aif; / max number of aif,
    pub reserved1: __le32,
    pub reserved2: __le32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _rrq {
    pub host_addr_low: __le32,
    pub host_addr_high: __le32,
    pub msix_id: __le16,
    pub element_count: __le16,
    pub comp_thresh: __le16,
    pub unused: __le16,
    pub /: *mut *mut } rrq[] __counted_by_le(rr_queue_count); / up to 64 RRQ addresses,
    pub r8: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aac_log_level {
    LOG_AAC_INIT			= 10,
    LOG_AAC_INFORMATIONAL		= 20,
    LOG_AAC_WARNING			= 30,
    LOG_AAC_LOW_ERROR		= 40,
    LOG_AAC_MEDIUM_ERROR		= 50,
    LOG_AAC_HIGH_ERROR		= 60,
    LOG_AAC_PANIC			= 70,
    LOG_AAC_DEBUG			= 80,
    LOG_AAC_WINDBG_PRINT		= 90
}

pub const FSAFS_NTC_GET_ADAPTER_FIB_CONTEXT: c_uint = 0x030b;
pub const FSAFS_NTC_FIB_CONTEXT: c_uint = 0x030c;
// Low level operations
// Transport operations
// Packet operations
// Administrative operations
//
// Define which interrupt handler needs to be installed
//
// Some adapter firmware needs communication memory
// below 2gig. This tells the init function to set the
// dma mask such that fib memory will be allocated where the
// adapter firmware can get to it.
//
pub const AAC_QUIRK_31BIT: c_uint = 0x0001;
//
// Some adapter firmware, when the raid card's cache is turned off, can not
// split up scatter gathers in order to deal with the limits of the
// underlying CHIM. This limit is 34 scatter gather elements.
//
pub const AAC_QUIRK_34SG: c_uint = 0x0002;
//
// This adapter is a slave (no Firmware)
//
pub const AAC_QUIRK_SLAVE: c_uint = 0x0004;
//
// This adapter is a master.
//
pub const AAC_QUIRK_MASTER: c_uint = 0x0008;
//
// Some adapter firmware perform poorly when it must split up scatter gathers
// in order to deal with the limits of the underlying CHIM. This limit in this
// class of adapters is 17 scatter gather elements.
//
pub const AAC_QUIRK_17SG: c_uint = 0x0010;
//
// Some adapter firmware does not support 64 bit scsi passthrough
// commands.
//
pub const AAC_QUIRK_SCSI_32: c_uint = 0x0020;
//
// SRC based adapters support the AifReqEvent functions
//
pub const AAC_QUIRK_SRC: c_uint = 0x0040;
//
// The adapter interface specs all queues to be located in the same
// physically contiguous block. The host structure that defines the
// commuication queues will assume they are each a separate physically
// contiguous memory region that will support them all being one big
// contiguous block.
// There is a command and response queue for each level and direction of
// commuication. These regions are accessed by both the host and adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_queue {
    pub /: *mut *mut u64 logical; /address we give the adapter,
    pub /: *mut *mut *mut aac_entry base; /system virtual address,
    pub headers*/: *mut *mut aac_qhdr headers; /producer,consumer q,
    pub /: *mut *mut u32 entries; /Number of queue entries,
    pub /: *mut *mut wait_queue_head_t qfull; /Event to wait on if q full,
    pub /: *mut *mut wait_queue_head_t cmdready; /Cmd ready from the adapter,
// This is only valid for adapter to host command queues.
    pub /: *mut *mut *mut spinlock_t lock; / Spinlock for this queue must take this lock before accessing the lock,
    pub /: *mut *mut spinlock_t lockdata; / Actual lock (used only on one side of the lock),
    pub /: *mut *mut list_head cmdq; / A queue of FIBs which need to be prcessed by the FS thread. This is,
// only valid for command queues which receive entries from the adapter.
// Number of entries on outstanding queue.
    pub numpending: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut aac_dev  dev; / Back pointer to adapter structure,
}

//
// Message queues. The order here is important, see also the
// queue type ordering
//
// SaP1 Message Unit Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_drawbridge_CSR {
// Offset	|  Name
    pub /: *mut *mut __le32 reserved[10]; / 00h-27h | Reserved,
    pub /: *mut *mut u8 LUT_Offset; / 28h | Lookup Table Offset,
    pub /: *mut *mut u8 reserved1[3]; / 29h-2bh | Reserved,
    pub /: *mut *mut __le32 LUT_Data; / 2ch | Looup Table Data,
    pub /: *mut *mut __le32 reserved2[26]; / 30h-97h | Reserved,
    pub /: *mut *mut __le16 PRICLEARIRQ; / 98h | Primary Clear Irq,
    pub /: *mut *mut __le16 SECCLEARIRQ; / 9ah | Secondary Clear Irq,
    pub /: *mut *mut __le16 PRISETIRQ; / 9ch | Primary Set Irq,
    pub /: *mut *mut __le16 SECSETIRQ; / 9eh | Secondary Set Irq,
    pub /: *mut *mut __le16 PRICLEARIRQMASK;/ a0h | Primary Clear Irq Mask,
    pub /: *mut *mut __le16 SECCLEARIRQMASK;/ a2h | Secondary Clear Irq Mask,
    pub /: *mut *mut __le16 PRISETIRQMASK; / a4h | Primary Set Irq Mask,
    pub /: *mut *mut __le16 SECSETIRQMASK; / a6h | Secondary Set Irq Mask,
    pub /: *mut *mut __le32 MAILBOX0; / a8h | Scratchpad 0,
    pub /: *mut *mut __le32 MAILBOX1; / ach | Scratchpad 1,
    pub /: *mut *mut __le32 MAILBOX2; / b0h | Scratchpad 2,
    pub /: *mut *mut __le32 MAILBOX3; / b4h | Scratchpad 3,
    pub /: *mut *mut __le32 MAILBOX4; / b8h | Scratchpad 4,
    pub /: *mut *mut __le32 MAILBOX5; / bch | Scratchpad 5,
    pub /: *mut *mut __le32 MAILBOX6; / c0h | Scratchpad 6,
    pub /: *mut *mut __le32 MAILBOX7; / c4h | Scratchpad 7,
    pub /: *mut *mut __le32 ROM_Setup_Data; / c8h | Rom Setup and Data,
    pub /: *mut *mut __le32 ROM_Control_Addr;/ cch | Rom Control and Address,
    pub /: *mut *mut __le32 reserved3[12]; / d0h-ffh | reserved,
    pub /: *mut *mut __le32 LUT[64]; / 100h-1ffh | Lookup Table Entries,
}

pub const DOORBELL_0: c_uint = 0x0001;
pub const DOORBELL_1: c_uint = 0x0002;
pub const DOORBELL_2: c_uint = 0x0004;
pub const DOORBELL_3: c_uint = 0x0008;
pub const DOORBELL_4: c_uint = 0x0010;
pub const DOORBELL_5: c_uint = 0x0020;
pub const DOORBELL_6: c_uint = 0x0040;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_registers {
    pub /: *mut *mut sa_drawbridge_CSR SaDbCSR; / 98h - c4h,
}

pub const SA_INIT_NUM_MSIXVECTORS: c_int = 1;

//
// Rx Message Unit Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_mu_registers {
// Local  | PCI*| Name
    pub /: *mut *mut __le32 ARSR; / 1300h | 00h | APIC Register Select Register,
    pub /: *mut *mut __le32 reserved0; / 1304h | 04h | Reserved,
    pub /: *mut *mut __le32 AWR; / 1308h | 08h | APIC Window Register,
    pub /: *mut *mut __le32 reserved1; / 130Ch | 0Ch | Reserved,
    pub /: *mut *mut __le32 IMRx[2]; / 1310h | 10h | Inbound Message Registers,
    pub /: *mut *mut __le32 OMRx[2]; / 1318h | 18h | Outbound Message Registers,
    pub /: *mut *mut __le32 IDR; / 1320h | 20h | Inbound Doorbell Register,
    pub Interrupt: *mut *mut __le32 IISR; / 1324h | 24h | Inbound,
    pub Interrupt: *mut *mut __le32 IIMR; / 1328h | 28h | Inbound,
    pub /: *mut *mut __le32 ODR; / 132Ch | 2Ch | Outbound Doorbell Register,
    pub Interrupt: *mut *mut __le32 OISR; / 1330h | 30h | Outbound,
    pub Interrupt: *mut *mut __le32 OIMR; / 1334h | 34h | Outbound,
    pub /: *mut *mut __le32 reserved2; / 1338h | 38h | Reserved,
    pub /: *mut *mut __le32 reserved3; / 133Ch | 3Ch | Reserved,
    pub /: *mut *mut __le32 InboundQueue;/ 1340h | 40h | Inbound Queue Port relative to firmware,
    pub /: *mut *mut __le32 OutboundQueue;/1344h | 44h | Outbound Queue Port relative to firmware,
// * Must access through ATU Inbound
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_inbound {
    pub Mailbox: [__le32; 8],
}

pub const INBOUNDDOORBELL_0: c_uint = 0x00000001;
pub const INBOUNDDOORBELL_1: c_uint = 0x00000002;
pub const INBOUNDDOORBELL_2: c_uint = 0x00000004;
pub const INBOUNDDOORBELL_3: c_uint = 0x00000008;
pub const INBOUNDDOORBELL_4: c_uint = 0x00000010;
pub const INBOUNDDOORBELL_5: c_uint = 0x00000020;
pub const INBOUNDDOORBELL_6: c_uint = 0x00000040;
pub const OUTBOUNDDOORBELL_0: c_uint = 0x00000001;
pub const OUTBOUNDDOORBELL_1: c_uint = 0x00000002;
pub const OUTBOUNDDOORBELL_2: c_uint = 0x00000004;
pub const OUTBOUNDDOORBELL_3: c_uint = 0x00000008;
pub const OUTBOUNDDOORBELL_4: c_uint = 0x00000010;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_registers {
    pub /: *mut *mut rx_mu_registers MUnit; / 1300h - 1347h,
    pub /: *mut *mut __le32 reserved1[2]; / 1348h - 134ch,
    pub IndexRegs: rx_inbound,
}

//
// Rkt Message Unit Registers (same as Rx, except a larger reserve region)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkt_registers {
    pub /: *mut *mut rkt_mu_registers MUnit; / 1300h - 1347h,
    pub /: *mut *mut __le32 reserved1[1006]; / 1348h - 22fch,
    pub /: *mut *mut rkt_inbound IndexRegs; / 2300h -,
}

//
// PMC SRC message unit registers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_mu_registers {
// PCI*| Name
    pub /: *mut *mut __le32 reserved0[6]; / 00h | Reserved,
    pub /: *mut *mut __le32 IOAR[2]; / 18h | IOA->host interrupt register,
    pub /: *mut *mut __le32 IDR; / 20h | Inbound Doorbell Register,
    pub /: *mut *mut __le32 IISR; / 24h | Inbound Int. Status Register,
    pub /: *mut *mut __le32 reserved1[3]; / 28h | Reserved,
    pub /: *mut *mut __le32 OIMR; / 34h | Outbound Int. Mask Register,
    pub /: *mut *mut __le32 reserved2[25]; / 38h | Reserved,
    pub /: *mut *mut __le32 ODR_R; / 9ch | Outbound Doorbell Read,
    pub /: *mut *mut __le32 ODR_C; / a0h | Outbound Doorbell Clear,
    pub /: *mut *mut __le32 reserved3[3]; / a4h | Reserved,
    pub /: *mut *mut __le32 SCR0; / b0h | Scratchpad 0,
    pub /: *mut *mut __le32 reserved4[2]; / b4h | Reserved,
    pub /: *mut *mut __le32 OMR; / bch | Outbound Message Register,
    pub /: *mut *mut __le32 IQ_L; / c0h | Inbound Queue (Low address),
    pub /: *mut *mut __le32 IQ_H; / c4h | Inbound Queue (High address),
    pub /: *mut *mut __le32 ODR_MSI; / c8h | MSI register for sync./AIF,
    pub /: *mut *mut __le32 reserved5; / cch | Reserved,
    pub /: *mut *mut __le32 IQN_L; / d0h | Inbound (native cmd) low,
    pub /: *mut *mut __le32 IQN_H; / d4h | Inbound (native cmd) high,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_registers {
    pub /: *mut *mut src_mu_registers MUnit; / 00h - cbh,
    pub /: *mut *mut __le32 reserved1[130786]; / d8h - 7fc5fh,
    pub /: *mut *mut src_inbound IndexRegs; / 7fc60h,
    pub tupelo: },
    pub /: *mut *mut __le32 reserved1[970]; / d8h - fffh,
    pub /: *mut *mut src_inbound IndexRegs; / 1000h,
    pub denali: },
    pub u: },
}

pub const SRC_ODR_SHIFT: c_int = 12;
pub const SRC_IDR_SHIFT: c_int = 9;
pub const SRC_MSI_READ_MASK: c_uint = 0x1000;
extern "C" {
    pub fn void(ctxt: *mut *mut fib_callback)(void, fibctx: *mut fib) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_fib_context {
    pub structure: s16 type; // used for verification of,
    pub size: i16,
    pub context: u32 unique; // unique value representing this,
    pub ulong: ulong jiffies; // used for cleanup - dmb changed to,
    pub list: list_head next; // used to link context's into a linked,
    pub arrive.: completion completion; // this is used to wait for the next fib to,
    pub WaitForSingleObject: int wait; // Set to true when thread is in,
    pub FibList: unsigned long count; // total number of FIBs on,
    pub hw_fibs: list_head fib_list; // this holds fibs and their attachd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sense_data {
    pub /: *mut *mut u8 error_code; / 70h (current errors), 71h(deferred errors),
    pub /: *mut *mut u8 valid:1; / A valid bit of one indicates that the information,
// field contains valid information as defined in the
// SCSI-2 Standard.
//
    pub /: *mut *mut u8 segment_number; / Only used for COPY, COMPARE, or COPY AND VERIFY Commands,
    pub /: *mut *mut u8 sense_key:4; / Sense Key,
    pub reserved:1: u8,
    pub /: *mut *mut u8 ILI:1; / Incorrect Length Indicator,
    pub /: *mut *mut u8 EOM:1; / End Of Medium - reserved for random access devices,
    pub /: *mut *mut u8 filemark:1; / Filemark - reserved for random access devices,
    pub unsigned: *mut *mut u8 information[4]; / for direct-access devices, contains the,
// logical block address or residue associated with
// the sense key
//
    pub /: *mut *mut u8 add_sense_len; / number of additional sense bytes to follow this field,
    pub /: *mut *mut u8 cmnd_info[4]; / not used,
    pub /: *mut *mut u8 ASC; / Additional Sense Code,
    pub /: *mut *mut u8 ASCQ; / Additional Sense Code Qualifier,
    pub /: *mut *mut u8 FRUC; / Field Replaceable Unit Code - not used,
    pub data: *mut *mut u8 bit_ptr:3; / indicates which byte of the CDB or parameter,
// was in error
//
    pub that: *mut *mut u8 BPV:1; / bit pointer valid (BPV): 1- indicates,
// the bit_ptr field has valid value
//
    pub reserved2:2: u8,
    pub CDB.: *mut *mut u8 CD:1; / command data bit: 1- illegal parameter in,
// 0- illegal parameter in data.
//
    pub SKSV:1: u8,
    pub /: *mut *mut u8 field_ptr[2]; / byte of the CDB or parameter data in error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsa_dev_info {
    pub last: u64,
    pub size: u64,
    pub type: u32,
    pub config_waiting_on: u32,
    pub config_waiting_stamp: c_ulong,
    pub queue_depth: u16,
    pub config_needed: u8,
    pub valid: u8,
    pub ro: u8,
    pub locked: u8,
    pub deleted: u8,
    pub devname: [c_char; 8],
    pub sense_data: sense_data,
    pub block_size: u32,
    pub identifier: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib {
    pub /: *mut *mut *mut void next; / this is used by the allocator,
    pub type: i16,
    pub size: i16,
//
// The Adapter that this I/O is destined for.
//
    pub dev: *mut aac_dev,
//
// This is the event the sendfib routine will wait on if the
// caller did not pass one and this is synch io.
//
    pub event_wait: completion,
    pub event_lock: spinlock_t,
    pub /: *mut *mut u32 done; / gets set to 1 when fib is complete,
    pub callback: fib_callback,
    pub callback_data: *mut c_void,
    pub ulong: u32 flags; // u32 dmb was,
//
// And for the internal issue/reply queues (we may be able
// to merge these two)
//
    pub fiblink: list_head,
    pub data: *mut c_void,
    pub vector_no: u32,
    pub /: *mut *mut *mut hw_fib hw_fib_va; / also used for native,
    pub hw_fib*/: *mut *mut dma_addr_t hw_fib_pa; / physical address of,
    pub /: *mut *mut dma_addr_t hw_sgl_pa; / extra sgl for native,
    pub /: *mut *mut dma_addr_t hw_error_pa; / error buffer for native,
    pub /: *mut *mut u32 hbacmd_size; / cmd size for native,
}

pub const AAC_INIT: c_int = 0;
pub const AAC_RESCAN: c_int = 1;
pub const AAC_DEVTYPE_RAID_MEMBER: c_int = 1;
pub const AAC_DEVTYPE_ARC_RAW: c_int = 2;
pub const AAC_DEVTYPE_NATIVE_RAW: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_map_info {
    pub /: *mut *mut __le32 rmw_nexus; / nexus for native HBA devices,
    pub /: *mut *mut u8 devtype; / device type,
    pub /: *mut *mut s8 reset_state; / 0 - no reset, 1..x -,
// after xth TM LUN reset
    pub qd_limit: u16,
    pub scan_counter: u32,
    pub safw_identify_resp: *mut aac_ciss_identify_pd,
}

//
// Adapter Information Block
//
// This is returned by the RequestAdapterInfo block
//
// StructExpansion == 1

// SupportedOptions2

// 4KB sector size

// 240 simple volume support

//
// Supports FIB dump sync command send prior to IOP_RESET
//

pub const AAC_SIS_VERSION_V3: c_int = 3;
pub const AAC_SIS_SLOT_UNKNOWN: c_uint = 0xFF;
pub const GetBusInfo: c_uint = 0x00000009;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_bus_info {
    pub /: *mut *mut __le32 Command; / VM_Ioctl,
    pub /: *mut *mut __le32 ObjType; / FT_DRIVE,
    pub /: *mut *mut __le32 MethodId; / 1 = SCSI Layer,
    pub /: *mut *mut __le32 ObjectId; / Handle,
    pub /: *mut *mut __le32 CtlCmd; / GetBusInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_bus_info_response {
    pub /: *mut *mut __le32 Status; / ST_OK,
    pub ObjType: __le32,
    pub /: *mut *mut __le32 MethodId; / unused,
    pub /: *mut *mut __le32 ObjectId; / unused,
    pub /: *mut *mut __le32 CtlCmd; / unused,
    pub ProbeComplete: __le32,
    pub BusCount: __le32,
    pub TargetsPerBus: __le32,
    pub InitiatorBusId: [u8; 10],
    pub BusValid: [u8; 10],
}

//
// Battery platforms
//

//
// cpu types
//

//
// Supported Options
//

pub const AAC_COMM_PRODUCER: c_int = 0;
pub const AAC_COMM_MESSAGE: c_int = 1;
pub const AAC_COMM_MESSAGE_TYPE1: c_int = 3;
pub const AAC_COMM_MESSAGE_TYPE2: c_int = 4;
pub const AAC_COMM_MESSAGE_TYPE3: c_int = 5;

// MSIX context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_msix_ctx {
    pub vector_no: c_int,
    pub dev: *mut aac_dev,
}

//
// negotiated FIB settings
//
// Map for 128 fib objects (64k)
//
// Fib Headers
//
// The user API will use an IOCTL to register itself to receive
// FIBs from the adapter.  The following list is used to keep
// track of all the threads that have requested these FIBs.  The
// mutex is used to synchronize access to all data associated
// with the adapter fibs.
//
// debug buffer
// mapped in region
//
// Holds initialization info
// to communicate with adapter
//
// response queue (if AAC_COMM_MESSAGE_TYPE1)
// index into rrq buffer
// pointer to buffer used for printf's from the adapter
//
// This lock will protect the two 32-bit
// writes to the Inbound Queue
//
// The following is the device specific extension.
//

//
// AIF thread states
//
// These are in adapter info but they are in the io flow so
// lets break them out so we don't have to do an AND to check them
//

//
// Define the command values
//
pub const Null: c_int = 0;
pub const GetAttributes: c_int = 1;
pub const SetAttributes: c_int = 2;
pub const Lookup: c_int = 3;
pub const ReadLink: c_int = 4;
pub const Read: c_int = 5;
pub const Write: c_int = 6;
pub const Create: c_int = 7;
pub const MakeDirectory: c_int = 8;
pub const SymbolicLink: c_int = 9;
pub const MakeNode: c_int = 10;
pub const Removex: c_int = 11;
pub const RemoveDirectoryx: c_int = 12;
pub const Rename: c_int = 13;
pub const Link: c_int = 14;
pub const ReadDirectory: c_int = 15;
pub const ReadDirectoryPlus: c_int = 16;
pub const FileSystemStatus: c_int = 17;
pub const FileSystemInfo: c_int = 18;
pub const PathConfigure: c_int = 19;
pub const Commit: c_int = 20;
pub const Mount: c_int = 21;
pub const UnMount: c_int = 22;
pub const Newfs: c_int = 23;
pub const FsCheck: c_int = 24;
pub const FsSync: c_int = 25;
pub const SimReadWrite: c_int = 26;
pub const SetFileSystemStatus: c_int = 27;
pub const BlockRead: c_int = 28;
pub const BlockWrite: c_int = 29;
pub const NvramIoctl: c_int = 30;
pub const FsSyncWait: c_int = 31;
pub const ClearArchiveBit: c_int = 32;
pub const SetAcl: c_int = 33;
pub const GetAcl: c_int = 34;
pub const AssignAcl: c_int = 35;

pub const MAX_FSACOMMAND_NUM: c_int = 38;
//
// Define the status returns. These are very unixlike although
// most are not in fact used
//
pub const ST_OK: c_int = 0;
pub const ST_PERM: c_int = 1;
pub const ST_NOENT: c_int = 2;
pub const ST_IO: c_int = 5;
pub const ST_NXIO: c_int = 6;
pub const ST_E2BIG: c_int = 7;
pub const ST_MEDERR: c_int = 8;
pub const ST_ACCES: c_int = 13;
pub const ST_EXIST: c_int = 17;
pub const ST_XDEV: c_int = 18;
pub const ST_NODEV: c_int = 19;
pub const ST_NOTDIR: c_int = 20;
pub const ST_ISDIR: c_int = 21;
pub const ST_INVAL: c_int = 22;
pub const ST_FBIG: c_int = 27;
pub const ST_NOSPC: c_int = 28;
pub const ST_ROFS: c_int = 30;
pub const ST_MLINK: c_int = 31;
pub const ST_WOULDBLOCK: c_int = 35;
pub const ST_NAMETOOLONG: c_int = 63;
pub const ST_NOTEMPTY: c_int = 66;
pub const ST_DQUOT: c_int = 69;
pub const ST_STALE: c_int = 70;
pub const ST_REMOTE: c_int = 71;
pub const ST_NOT_READY: c_int = 72;
pub const ST_BADHANDLE: c_int = 10001;
pub const ST_NOT_SYNC: c_int = 10002;
pub const ST_BAD_COOKIE: c_int = 10003;
pub const ST_NOTSUPP: c_int = 10004;
pub const ST_TOOSMALL: c_int = 10005;
pub const ST_SERVERFAULT: c_int = 10006;
pub const ST_BADTYPE: c_int = 10007;
pub const ST_JUKEBOX: c_int = 10008;
pub const ST_NOTMOUNTED: c_int = 10009;
pub const ST_MAINTMODE: c_int = 10010;
pub const ST_STALEACL: c_int = 10011;
//
// On writes how does the client want the data written.
//
pub const CACHE_CSTABLE: c_int = 1;
pub const CACHE_UNSTABLE: c_int = 2;
//
// Lets the client know at which level the data was committed on
// a write request
//
pub const CMFILE_SYNCH_NVRAM: c_int = 1;
pub const CMDATA_SYNCH_NVRAM: c_int = 2;
pub const CMFILE_SYNCH: c_int = 3;
pub const CMDATA_SYNCH: c_int = 4;
pub const CMUNSTABLE: c_int = 5;
pub const RIO_TYPE_WRITE: c_uint = 0x0000;
pub const RIO_TYPE_READ: c_uint = 0x0001;
pub const RIO_SUREWRITE: c_uint = 0x0008;
pub const RIO2_IO_TYPE: c_uint = 0x0003;
pub const RIO2_IO_TYPE_WRITE: c_uint = 0x0000;
pub const RIO2_IO_TYPE_READ: c_uint = 0x0001;
pub const RIO2_IO_TYPE_VERIFY: c_uint = 0x0002;
pub const RIO2_IO_ERROR: c_uint = 0x0004;
pub const RIO2_IO_SUREWRITE: c_uint = 0x0008;
pub const RIO2_SGL_CONFORMANT: c_uint = 0x0010;
pub const RIO2_SG_FORMAT: c_uint = 0xF000;
pub const RIO2_SG_FORMAT_ARC: c_uint = 0x0000;
pub const RIO2_SG_FORMAT_SRL: c_uint = 0x1000;
pub const RIO2_SG_FORMAT_IEEE1212: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_raw_io2 {
    pub blockLow: __le32,
    pub blockHigh: __le32,
    pub byteCount: __le32,
    pub cid: __le16,
    pub /: *mut *mut __le16 flags; / RIO2 flags,
    pub /: *mut *mut __le32 sgeFirstSize; / size of first sge el.,
    pub /: *mut *mut __le32 sgeNominalSize; / size of 2nd sge el. (if conformant),
    pub /: *mut *mut u8 sgeCnt; / only 8 bits required,
    pub /: *mut *mut u8 bpTotal; / reserved for F/W use,
    pub /: *mut *mut u8 bpComplete; / reserved for F/W use,
    pub /: *mut *mut u8 sgeFirstIndex; / reserved for F/W use,
    pub unused: [u8; 4],
    pub sge: [sge_ieee1212; ],
}

pub const CT_FLUSH_CACHE: c_int = 129;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_synchronize {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_FLUSH_CACHE,
    pub cid: __le32,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub /: *mut *mut *mut __le32 count; / sizeof(((struct aac_synchronize_reply )NULL)->data),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_synchronize_reply {
    pub dummy0: __le32,
    pub dummy1: __le32,
    pub /: *mut *mut __le32 status; / CT_OK,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub parm5: __le32,
    pub data: [u8; 16],
}

pub const CT_POWER_MANAGEMENT: c_int = 245;
pub const CT_PM_START_UNIT: c_int = 2;
pub const CT_PM_STOP_UNIT: c_int = 3;
pub const CT_PM_UNIT_IMMEDIATE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_power_management {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_POWER_MANAGEMENT,
    pub /: *mut *mut *mut __le32 sub; / CT_PM_,
    pub cid: __le32,
    pub /: *mut *mut *mut __le32 parm; / CT_PM_sub_,
}

pub const CT_PAUSE_IO: c_int = 65;
pub const CT_RELEASE_IO: c_int = 66;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_pause {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_PAUSE_IO,
    pub /: *mut *mut __le32 timeout; / 10ms ticks,
    pub min: __le32,
    pub noRescan: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub /: *mut *mut *mut __le32 count; / sizeof(((struct aac_pause_reply )NULL)->data),
}

//
// This and associated data structs are used by the
// ioctl caller and are in cpu order.
//
pub const AAC_SENSE_BUFFERSIZE: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_srb_unit {
    pub srb_reply: aac_srb_reply,
    pub srb: aac_srb,
}

//
// SRB Flags
//
pub const SRB_NoDataXfer: c_uint = 0x0000;
pub const SRB_DisableDisconnect: c_uint = 0x0004;
pub const SRB_DisableSynchTransfer: c_uint = 0x0008;
pub const SRB_BypassFrozenQueue: c_uint = 0x0010;
pub const SRB_DisableAutosense: c_uint = 0x0020;
pub const SRB_DataIn: c_uint = 0x0040;
pub const SRB_DataOut: c_uint = 0x0080;
//
// SRB Functions - set in aac_srb->function
//
pub const SRBF_ExecuteScsi: c_uint = 0x0000;
pub const SRBF_ClaimDevice: c_uint = 0x0001;
pub const SRBF_IO_Control: c_uint = 0x0002;
pub const SRBF_ReceiveEvent: c_uint = 0x0003;
pub const SRBF_ReleaseQueue: c_uint = 0x0004;
pub const SRBF_AttachDevice: c_uint = 0x0005;
pub const SRBF_ReleaseDevice: c_uint = 0x0006;
pub const SRBF_Shutdown: c_uint = 0x0007;
pub const SRBF_Flush: c_uint = 0x0008;
pub const SRBF_AbortCommand: c_uint = 0x0010;
pub const SRBF_ReleaseRecovery: c_uint = 0x0011;
pub const SRBF_ResetBus: c_uint = 0x0012;
pub const SRBF_ResetDevice: c_uint = 0x0013;
pub const SRBF_TerminateIO: c_uint = 0x0014;
pub const SRBF_FlushQueue: c_uint = 0x0015;
pub const SRBF_RemoveDevice: c_uint = 0x0016;
pub const SRBF_DomainValidation: c_uint = 0x0017;
//
// SRB SCSI Status - set in aac_srb->scsi_status
//
pub const SRB_STATUS_PENDING: c_uint = 0x00;
pub const SRB_STATUS_SUCCESS: c_uint = 0x01;
pub const SRB_STATUS_ABORTED: c_uint = 0x02;
pub const SRB_STATUS_ABORT_FAILED: c_uint = 0x03;
pub const SRB_STATUS_ERROR: c_uint = 0x04;
pub const SRB_STATUS_BUSY: c_uint = 0x05;
pub const SRB_STATUS_INVALID_REQUEST: c_uint = 0x06;
pub const SRB_STATUS_INVALID_PATH_ID: c_uint = 0x07;
pub const SRB_STATUS_NO_DEVICE: c_uint = 0x08;
pub const SRB_STATUS_TIMEOUT: c_uint = 0x09;
pub const SRB_STATUS_SELECTION_TIMEOUT: c_uint = 0x0A;
pub const SRB_STATUS_COMMAND_TIMEOUT: c_uint = 0x0B;
pub const SRB_STATUS_MESSAGE_REJECTED: c_uint = 0x0D;
pub const SRB_STATUS_BUS_RESET: c_uint = 0x0E;
pub const SRB_STATUS_PARITY_ERROR: c_uint = 0x0F;
pub const SRB_STATUS_REQUEST_SENSE_FAILED: c_uint = 0x10;
pub const SRB_STATUS_NO_HBA: c_uint = 0x11;
pub const SRB_STATUS_DATA_OVERRUN: c_uint = 0x12;
pub const SRB_STATUS_UNEXPECTED_BUS_FREE: c_uint = 0x13;
pub const SRB_STATUS_PHASE_SEQUENCE_FAILURE: c_uint = 0x14;
pub const SRB_STATUS_BAD_SRB_BLOCK_LENGTH: c_uint = 0x15;
pub const SRB_STATUS_REQUEST_FLUSHED: c_uint = 0x16;
pub const SRB_STATUS_DELAYED_RETRY: c_uint = 0x17;
pub const SRB_STATUS_INVALID_LUN: c_uint = 0x20;
pub const SRB_STATUS_INVALID_TARGET_ID: c_uint = 0x21;
pub const SRB_STATUS_BAD_FUNCTION: c_uint = 0x22;
pub const SRB_STATUS_ERROR_RECOVERY: c_uint = 0x23;
pub const SRB_STATUS_NOT_STARTED: c_uint = 0x24;
pub const SRB_STATUS_NOT_IN_USE: c_uint = 0x30;
pub const SRB_STATUS_FORCE_ABORT: c_uint = 0x31;
pub const SRB_STATUS_DOMAIN_VALIDATION_FAIL: c_uint = 0x32;
//
// Object-Server / Volume-Manager Dispatch Classes
//
pub const VM_Null: c_int = 0;
pub const VM_NameServe: c_int = 1;
pub const VM_ContainerConfig: c_int = 2;
pub const VM_Ioctl: c_int = 3;
pub const VM_FilesystemIoctl: c_int = 4;
pub const VM_CloseAll: c_int = 5;
pub const VM_CtBlockRead: c_int = 6;
pub const VM_CtBlockWrite: c_int = 7;

pub const VM_SliceBlockWrite: c_int = 9;

pub const VM_DriveBlockWrite: c_int = 11;

pub const VM_CtBlockVerify: c_int = 14;

pub const VM_CtBlockRead64: c_int = 16;
pub const VM_CtBlockWrite64: c_int = 17;
pub const VM_CtBlockVerify64: c_int = 18;
pub const VM_CtHostRead64: c_int = 19;
pub const VM_CtHostWrite64: c_int = 20;
pub const VM_DrvErrTblLog: c_int = 21;
pub const VM_NameServe64: c_int = 22;
pub const VM_NameServeAllBlk: c_int = 30;

//
// Descriptive information (eg, vital stats)
// that a content manager might report.  The
// FileArray filesystem component is one example
// of a content manager.  Raw mode might be
// another.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_fsinfo {
    pub /: *mut *mut __le32 fsTotalSize; / Consumed by fs, incl. metadata,
    pub fsBlockSize: __le32,
    pub fsFragSize: __le32,
    pub fsMaxExtendSize: __le32,
    pub fsSpaceUnits: __le32,
    pub fsMaxNumFiles: __le32,
    pub fsNumFreeFiles: __le32,
    pub fsInodeDensity: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_blockdevinfo {
    pub block_size: __le32,
    pub logical_phys_map: __le32,
    pub identifier: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aac_contentinfo {
    pub filesys: aac_fsinfo,
    pub bdevinfo: aac_blockdevinfo,
}

//
// Query for Container Configuration Status
//
pub const CT_GET_CONFIG_STATUS: c_int = 147;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_config_status {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_GET_CONFIG_STATUS,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub parm5: __le32,
    pub /: *mut *mut *mut __le32 count; / sizeof(((struct aac_get_config_status_resp )NULL)->data),
}

pub const CFACT_CONTINUE: c_int = 0;
pub const CFACT_PAUSE: c_int = 1;
pub const CFACT_ABORT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_config_status_resp {
    pub /: *mut *mut __le32 response; / ST_OK,
    pub dummy0: __le32,
    pub /: *mut *mut __le32 status; / CT_OK,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub parm5: __le32,
    pub /: *mut *mut __le32 action; / CFACT_CONTINUE, CFACT_PAUSE or CFACT_ABORT,
    pub flags: __le16,
    pub count: __le16,
    pub data: },
}

//
// Accept the configuration as-is
//
pub const CT_COMMIT_CONFIG: c_int = 152;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_commit_config {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_COMMIT_CONFIG,
}

//
// Query for Container Configuration Status
//
pub const CT_GET_CONTAINER_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_container_count {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_GET_CONTAINER_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_container_count_resp {
    pub /: *mut *mut __le32 response; / ST_OK,
    pub dummy0: __le32,
    pub MaxContainers: __le32,
    pub ContainerSwitchEntries: __le32,
    pub MaxPartitions: __le32,
    pub MaxSimpleVolumes: __le32,
}

//
// Query for "mountable" objects, ie, objects that are typically
// associated with a drive letter on the client (host) side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_mntent {
    pub oid: __le32,
    pub /: *mut *mut u8 name[16]; / if applicable,
    pub /: *mut *mut creation_info create_info; / if applicable,
    pub capacity: __le32,
    pub /: *mut *mut __le32 vol; / substrate structure,
    pub /: *mut *mut __le32 obj; / FT_FILESYS, etc.,
    pub mounting,: *mut *mut __le32 state; / unready for,
    pub content: *mut *mut aac_contentinfo fileinfo; / Info specific to,
    pub or: *mut *mut __le32 altoid; / != oid <==> snapshot,
    pub capacityhigh: __le32,
}

pub const FSCS_NOTCLEAN: c_uint = 0x0001  /* fsck is necessary before mounting */;
pub const FSCS_READONLY: c_uint = 0x0002	/* possible result of broken mirror */;
pub const FSCS_HIDDEN: c_uint = 0x0004	/* should be ignored - set during a clear */;
pub const FSCS_NOT_READY: c_uint = 0x0008	/* Array spinning up to fulfil request */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_query_mount {
    pub command: __le32,
    pub type: __le32,
    pub count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_mount {
    pub status: __le32,
    pub /: *mut *mut __le32 type; / should be same as that requested,
    pub count: __le32,
    pub mnt: [aac_mntent; 1],
}

pub const CT_READ_NAME: c_int = 130;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_name {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_READ_NAME,
    pub cid: __le32,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub /: *mut *mut *mut __le32 count; / sizeof(((struct aac_get_name_resp )NULL)->data),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_name_resp {
    pub dummy0: __le32,
    pub dummy1: __le32,
    pub /: *mut *mut __le32 status; / CT_OK,
    pub parm1: __le32,
    pub parm2: __le32,
    pub parm3: __le32,
    pub parm4: __le32,
    pub parm5: __le32,
    pub data: [u8; 17],
}

pub const CT_CID_TO_32BITS_UID: c_int = 165;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_serial {
    pub /: *mut *mut __le32 command; / VM_ContainerConfig,
    pub /: *mut *mut __le32 type; / CT_CID_TO_32BITS_UID,
    pub cid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_get_serial_resp {
    pub dummy0: __le32,
    pub dummy1: __le32,
    pub /: *mut *mut __le32 status; / CT_OK,
    pub uid: __le32,
}

//
// The following command is sent to shut down each container.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_close {
    pub command: __le32,
    pub cid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_delete_disk {
    pub disknum: u32,
    pub cnum: u32,
}

//
// Ugly - non Linux like ioctl coding for back compat.
//

//
// Define the method codes for how buffers are passed for I/O and FS
// controls
//
pub const METHOD_BUFFERED: c_int = 0;
pub const METHOD_NEITHER: c_int = 3;
//
// Filesystem ioctls
//

pub const FSACTL_DELETE_DISK: c_uint = 0x163;
pub const FSACTL_QUERY_DISK: c_uint = 0x173;

pub const FSACTL_GET_CONTAINERS: c_int = 2131;

// flags defined for IOP & HW SOFT RESET
pub const HW_IOP_RESET: c_uint = 0x01;
pub const HW_SOFT_RESET: c_uint = 0x02;

// HW Soft Reset register offset
pub const IBW_SWR_OFFSET: c_uint = 0x4000;
pub const SOFT_RESET_TIME: c_int = 60;
//
// If this value is set to 1 then interrupt moderation will occur
// in the base commuication support.
//
// Statistical counters in debug mode
//

//
// This is for management ioctl purpose only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_hba_info {
    pub driver_name: [u8; 50],
    pub adapter_number: u8,
    pub system_io_bus_number: u8,
    pub device_number: u8,
    pub function_number: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_vendor_id: u32,
    pub sub_system_id: u32,
    pub mapped_base_address_size: u32,
    pub base_physical_address_high_part: u32,
    pub base_physical_address_low_part: u32,
    pub max_command_size: u32,
    pub max_fib_size: u32,
    pub max_scatter_gather_from_os: u32,
    pub max_scatter_gather_to_fw: u32,
    pub max_outstanding_fibs: u32,
    pub queue_start_threshold: u32,
    pub queue_dump_threshold: u32,
    pub max_io_size_queued: u32,
    pub outstanding_io: u32,
    pub firmware_build_number: u32,
    pub bios_build_number: u32,
    pub driver_build_number: u32,
    pub serial_number_high_part: u32,
    pub serial_number_low_part: u32,
    pub supported_options: u32,
    pub feature_bits: u32,
    pub currentnumber_ports: u32,
    pub new_comm_interface:1: u8,
    pub new_commands_supported:1: u8,
    pub disable_passthrough:1: u8,
    pub expose_non_dasd:1: u8,
    pub queue_allowed:1: u8,
    pub bled_check_enabled:1: u8,
    pub reserved1:1: u8,
    pub reserted2:1: u8,
    pub reserved3: [u32; 10],
}

//
// The following macro is used when sending and receiving FIBs. It is
// only used for debugging.
//

// Macro flag: #define	FIB_COUNTER_INCREMENT(counter)

//
// Adapter direct commands
// Monitor/Kernel API
//
pub const BREAKPOINT_REQUEST: c_uint = 0x00000004;
pub const INIT_STRUCT_BASE_ADDRESS: c_uint = 0x00000005;
pub const READ_PERMANENT_PARAMETERS: c_uint = 0x0000000a;
pub const WRITE_PERMANENT_PARAMETERS: c_uint = 0x0000000b;
pub const HOST_CRASHING: c_uint = 0x0000000d;
pub const SEND_SYNCHRONOUS_FIB: c_uint = 0x0000000c;
pub const COMMAND_POST_RESULTS: c_uint = 0x00000014;
pub const GET_ADAPTER_PROPERTIES: c_uint = 0x00000019;
pub const GET_DRIVER_BUFFER_PROPERTIES: c_uint = 0x00000023;
pub const RCV_TEMP_READINGS: c_uint = 0x00000025;
pub const GET_COMM_PREFERRED_SETTINGS: c_uint = 0x00000026;
pub const IOP_RESET_FW_FIB_DUMP: c_uint = 0x00000034;
pub const DROP_IO: c_uint = 0x00000035;
pub const IOP_RESET: c_uint = 0x00001000;
pub const IOP_RESET_ALWAYS: c_uint = 0x00001001;
pub const RE_INIT_ADAPTER: c_uint = 0x000000ee;
pub const IOP_SRC_RESET_MASK: c_uint = 0x00000100;
//
// Adapter Status Register
//
// Phase Staus mailbox is 32bits:
// <31:16> = Phase Status
// <15:0>  = Phase
//
// The adapter reports is present state through the phase.  Only
// a single phase should be ever be set.  Each phase can have multiple
// phase status bits to provide more detailed information about the
// state of the board.  Care should be taken to ensure that any phase
// status bits that are set when changing the phase are also valid
// for the new phase or be cleared out.  Adapter software (monitor,
// iflash, kernel) is responsible for properly maintining the phase
// status mailbox when it is running.
//
// MONKER_API Phases
//
// Phases are bit oriented.  It is NOT valid  to have multiple bits set
//
pub const SELF_TEST_FAILED: c_uint = 0x00000004;
pub const MONITOR_PANIC: c_uint = 0x00000020;
pub const KERNEL_BOOTING: c_uint = 0x00000040;
pub const KERNEL_UP_AND_RUNNING: c_uint = 0x00000080;
pub const KERNEL_PANIC: c_uint = 0x00000100;
pub const FLASH_UPD_PENDING: c_uint = 0x00002000;
pub const FLASH_UPD_SUCCESS: c_uint = 0x00004000;
pub const FLASH_UPD_FAILED: c_uint = 0x00008000;
pub const INVALID_OMR: c_uint = 0xffffffff;

//
// Doorbell bit defines
//

// PMC specific outbound doorbell bits

//
// For FIB communication, we need all of the following things
// to send back to the user.
//

pub const EM_DRIVE_INSERTION: c_int = 31;
pub const EM_DRIVE_REMOVAL: c_int = 32;
pub const EM_SES_DRIVE_INSERTION: c_int = 33;
pub const EM_SES_DRIVE_REMOVAL: c_int = 26;

pub const AifBuCacheDataLoss: c_int = 10;
pub const AifBuCacheDataRecover: c_int = 11;

// PMC NEW COMM: Request the event data
pub const AifReqEvent: c_int = 200;

//
// Adapter Initiated FIB command structures. Start with the adapter
// initiated FIBs that really come from the adapter, and get responded
// to by the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_aifcmd {
    pub /: *mut *mut __le32 command; / Tell host what type of notify this is,
    pub /: *mut *mut __le32 seqnum; / To allow ordering of reports (if necessary),
    pub /: *mut *mut u8 data[]; / Undefined length (from kernel viewpoint),
}

//
// Convert capacity to cylinders
// accounting for the fact capacity could be a 64 bit value
//
extern "C" {
    pub fn aac_scan_host(dev: *mut aac_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aac_cmd_owner {
    AAC_OWNER_MIDLEVEL	= 0x101,
    AAC_OWNER_LOWLEVEL	= 0x102,
    AAC_OWNER_ERROR_HANDLER	= 0x103,
    AAC_OWNER_FIRMWARE	= 0x106,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aac_cmd_priv {
    pub ): *mut *mut int (callback)(struct scsi_cmnd,
    pub status: c_int,
    pub owner: aac_cmd_owner,
    pub sent_command: bool,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
extern "C" {
    pub fn aac_safw_rescan_worker(work: *mut work_struct);
}
extern "C" {
    pub fn aac_src_reinit_aif_worker(work: *mut work_struct);
}
extern "C" {
    pub fn aac_acquire_irq(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_free_irq(dev: *mut aac_dev);
}
extern "C" {
    pub fn aac_setup_safw_adapter(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_fib_vector_assign(dev: *mut aac_dev);
}
extern "C" {
    pub fn aac_fib_setup(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_fib_map_free(dev: *mut aac_dev);
}
extern "C" {
    pub fn aac_fib_free(context: *mut *mut fib);
}
extern "C" {
    pub fn aac_fib_init(context: *mut *mut fib);
}
extern "C" {
    pub fn aac_printf(dev: *mut aac_dev, val: u32);
}
extern "C" {
    pub fn aac_fib_send(command: u16, context: *mut *mut fib, size: c_ulong, priority: c_int, wait: c_int, reply: c_int, callback: fib_callback, ctxt: *mut c_void) -> c_int;
}
extern "C" {
    pub fn aac_consumer_get(dev: *mut *mut aac_dev, q: *mut *mut aac_queue, entry: *mut aac_entry) -> c_int;
}
extern "C" {
    pub fn aac_consumer_free(dev: *mut *mut aac_dev, q: *mut *mut aac_queue, qnum: u32);
}
extern "C" {
    pub fn aac_fib_complete(context: *mut *mut fib) -> c_int;
}
extern "C" {
    pub fn aac_hba_callback(context: *mut c_void, fibptr: *mut fib);
}

extern "C" {
    pub fn aac_src_access_devreg(dev: *mut aac_dev, mode: c_int);
}
extern "C" {
    pub fn aac_set_intx_mode(dev: *mut aac_dev);
}
extern "C" {
    pub fn aac_get_config_status(dev: *mut aac_dev, commit_flag: c_int) -> c_int;
}
extern "C" {
    pub fn aac_get_containers(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_scsi_cmd(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn aac_dev_ioctl(dev: *mut aac_dev, cmd: c_uint, arg: *mut void __user) -> c_int;
}

extern "C" {
    pub fn aac_get_serial_number(dev: *mut device, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn aac_do_ioctl(dev: *mut aac_dev, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn aac_rx_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_rkt_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_nark_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_sa_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_src_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_srcv_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_queue_get(dev: *mut *mut aac_dev, index: *mut *mut u32, qid: u32, hw_fib: *mut *mut hw_fib, wait: c_int, fibptr: *mut *mut fib, nonotify: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn aac_define_int_mode(dev: *mut aac_dev);
}
extern "C" {
    pub fn aac_response_normal(q: *mut *mut aac_queue) -> c_uint;
}
extern "C" {
    pub fn aac_command_normal(q: *mut *mut aac_queue) -> c_uint;
}
extern "C" {
    pub fn aac_reset_adapter(dev: *mut aac_dev, forced: c_int, reset_type: u8) -> c_int;
}
extern "C" {
    pub fn aac_command_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn aac_close_fib_context(dev: *mut *mut aac_dev, fibctx: *mut aac_fib_context) -> c_int;
}
extern "C" {
    pub fn aac_fib_adapter_complete(fibptr: *mut *mut fib, size: c_ushort) -> c_int;
}
extern "C" {
    pub fn aac_get_driver_ident(devtype: c_int) -> *mut aac_driver_ident;
}
extern "C" {
    pub fn aac_get_adapter_info(dev: *mut *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_send_shutdown(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_probe_container(dev: *mut aac_dev, cid: c_int) -> c_int;
}
extern "C" {
    pub fn _aac_rx_init(dev: *mut aac_dev) -> c_int;
}
extern "C" {
    pub fn aac_rx_select_comm(dev: *mut aac_dev, comm: c_int) -> c_int;
}
extern "C" {
    pub fn aac_rx_deliver_producer(fib: *mut *mut fib) -> c_int;
}
extern "C" {
    pub fn aac_reinit_aif(aac: *mut aac_dev, index: c_uint);
}
extern "C" {
    pub fn get_container_type(type: unsigned) -> *mut c_char;
}
