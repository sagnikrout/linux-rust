//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/vmw_pvscsi.h
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


//
// VMware PVSCSI header file
//
// Copyright (C) 2008-2014, VMware, Inc. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; version 2 of the License and no later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//

pub const PVSCSI_MAX_NUM_SG_ENTRIES_PER_SEGMENT: c_int = 128;

pub const PCI_DEVICE_ID_VMWARE_PVSCSI: c_uint = 0x07C0;
//
// host adapter status/error codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HostBusAdapterStatus {
    BTSTAT_SUCCESS       = 0x00,  /* CCB complete normally with no errors */
    BTSTAT_LINKED_COMMAND_COMPLETED           = 0x0a,
    BTSTAT_LINKED_COMMAND_COMPLETED_WITH_FLAG = 0x0b,
    BTSTAT_DATA_UNDERRUN = 0x0c,
    BTSTAT_SELTIMEO      = 0x11,  /* SCSI selection timeout */
    BTSTAT_DATARUN       = 0x12,  /* data overrun/underrun */
    BTSTAT_BUSFREE       = 0x13,  /* unexpected bus free */
    BTSTAT_INVPHASE      = 0x14,  /* invalid bus phase or sequence
// requested by target
    BTSTAT_LUNMISMATCH   = 0x17,  /* linked CCB has different LUN from
// first CCB
    BTSTAT_INVPARAM      = 0x1a,  /* invalid parameter in CCB or segment
// list
    BTSTAT_SENSFAILED    = 0x1b,  /* auto request sense failed */
    BTSTAT_TAGREJECT     = 0x1c,  /* SCSI II tagged queueing message
// rejected by target
    BTSTAT_BADMSG        = 0x1d,  /* unsupported message received by the
// host adapter
    BTSTAT_HAHARDWARE    = 0x20,  /* host adapter hardware failed */
    BTSTAT_NORESPONSE    = 0x21,  /* target did not respond to SCSI ATN,
// sent a SCSI RST
    BTSTAT_SENTRST       = 0x22,  /* host adapter asserted a SCSI RST */
    BTSTAT_RECVRST       = 0x23,  /* other SCSI devices asserted a SCSI
// RST
    BTSTAT_DISCONNECT    = 0x24,  /* target device reconnected improperly
// (w/o tag)
    BTSTAT_BUSRESET      = 0x25,  /* host adapter issued BUS device reset */
    BTSTAT_ABORTQUEUE    = 0x26,  /* abort queue generated */
    BTSTAT_HASOFTWARE    = 0x27,  /* host adapter software error */
    BTSTAT_HATIMEOUT     = 0x30,  /* host adapter hardware timeout error */
    BTSTAT_SCSIPARITY    = 0x34,  /* SCSI parity error detected */
}

//
// SCSI device status values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ScsiDeviceStatus {
    SDSTAT_GOOD  = 0x00, /* No errors. */
    SDSTAT_CHECK = 0x02, /* Check condition. */
}

//
// Register offsets.
//
// These registers are accessible both via i/o space and mm i/o.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSIRegOffset {
    PVSCSI_REG_OFFSET_COMMAND        =    0x0,
    PVSCSI_REG_OFFSET_COMMAND_DATA   =    0x4,
    PVSCSI_REG_OFFSET_COMMAND_STATUS =    0x8,
    PVSCSI_REG_OFFSET_LAST_STS_0     =  0x100,
    PVSCSI_REG_OFFSET_LAST_STS_1     =  0x104,
    PVSCSI_REG_OFFSET_LAST_STS_2     =  0x108,
    PVSCSI_REG_OFFSET_LAST_STS_3     =  0x10c,
    PVSCSI_REG_OFFSET_INTR_STATUS    = 0x100c,
    PVSCSI_REG_OFFSET_INTR_MASK      = 0x2010,
    PVSCSI_REG_OFFSET_KICK_NON_RW_IO = 0x3014,
    PVSCSI_REG_OFFSET_DEBUG          = 0x3018,
    PVSCSI_REG_OFFSET_KICK_RW_IO     = 0x4018,
}

//
// Virtual h/w commands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSICommands {
    PVSCSI_CMD_FIRST             = 0, /* has to be first */

    PVSCSI_CMD_ADAPTER_RESET     = 1,
    PVSCSI_CMD_ISSUE_SCSI        = 2,
    PVSCSI_CMD_SETUP_RINGS       = 3,
    PVSCSI_CMD_RESET_BUS         = 4,
    PVSCSI_CMD_RESET_DEVICE      = 5,
    PVSCSI_CMD_ABORT_CMD         = 6,
    PVSCSI_CMD_CONFIG            = 7,
    PVSCSI_CMD_SETUP_MSG_RING    = 8,
    PVSCSI_CMD_DEVICE_UNPLUG     = 9,
    PVSCSI_CMD_SETUP_REQCALLTHRESHOLD     = 10,

    PVSCSI_CMD_LAST              = 11  /* has to be last */
}

//
// Command descriptor for PVSCSI_CMD_RESET_DEVICE --
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescResetDevice {
    pub target: u32,
    pub lun: [u8; 8],
    pub __packed: },
//
// Command descriptor for PVSCSI_CMD_CONFIG --
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescConfigCmd {
    pub cmpAddr: u64,
    pub configPageAddress: u64,
    pub configPageNum: u32,
    pub _pad: u32,
    pub __packed: },
//
// Command descriptor for PVSCSI_CMD_SETUP_REQCALLTHRESHOLD --
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescSetupReqCall {
    pub enable: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSIConfigPageType {
    PVSCSI_CONFIG_PAGE_CONTROLLER = 0x1958,
    PVSCSI_CONFIG_PAGE_PHY        = 0x1959,
    PVSCSI_CONFIG_PAGE_DEVICE     = 0x195a,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSIConfigPageAddressType {
    PVSCSI_CONFIG_CONTROLLER_ADDRESS = 0x2120,
    PVSCSI_CONFIG_BUSTARGET_ADDRESS  = 0x2121,
    PVSCSI_CONFIG_PHY_ADDRESS        = 0x2122,
}

//
// Command descriptor for PVSCSI_CMD_ABORT_CMD --
//
// - currently does not support specifying the LUN.
// - _pad should be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescAbortCmd {
    pub context: u64,
    pub target: u32,
    pub _pad: u32,
    pub __packed: },
//
// Command descriptor for PVSCSI_CMD_SETUP_RINGS --
//
// Notes:
// - reqRingNumPages and cmpRingNumPages need to be power of two.
// - reqRingNumPages and cmpRingNumPages need to be different from 0,
// - reqRingNumPages and cmpRingNumPages need to be inferior to
// PVSCSI_SETUP_RINGS_MAX_NUM_PAGES.
//
pub const PVSCSI_SETUP_RINGS_MAX_NUM_PAGES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescSetupRings {
    pub reqRingNumPages: u32,
    pub cmpRingNumPages: u32,
    pub ringsStatePPN: u64,
    pub reqRingPPNs: [u64; PVSCSI_SETUP_RINGS_MAX_NUM_PAGES],
    pub cmpRingPPNs: [u64; PVSCSI_SETUP_RINGS_MAX_NUM_PAGES],
    pub __packed: },
//
// Command descriptor for PVSCSI_CMD_SETUP_MSG_RING --
//
// Notes:
// - this command was not supported in the initial revision of the h/w
// interface. Before using it, you need to check that it is supported by
// writing PVSCSI_CMD_SETUP_MSG_RING to the 'command' register, then
// immediately after read the 'command status' register:
// * a value of -1 means that the cmd is NOT supported,
// * a value != -1 means that the cmd IS supported.
// If it's supported the 'command status' register should return:
// sizeof(PVSCSICmdDescSetupMsgRing) / sizeof(u32).
// - this command should be issued _after_ the usual SETUP_RINGS so that the
// RingsState page is already setup. If not, the command is a nop.
// - numPages needs to be a power of two,
// - numPages needs to be different from 0,
// - _pad should be zero.
//
pub const PVSCSI_SETUP_MSG_RING_MAX_NUM_PAGES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSICmdDescSetupMsgRing {
    pub numPages: u32,
    pub _pad: u32,
    pub ringPPNs: [u64; PVSCSI_SETUP_MSG_RING_MAX_NUM_PAGES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSIMsgType {
    PVSCSI_MSG_DEV_ADDED          = 0,
    PVSCSI_MSG_DEV_REMOVED        = 1,
    PVSCSI_MSG_LAST               = 2,
}

//
// Msg descriptor.
//
// sizeof(struct PVSCSIRingMsgDesc) == 128.
//
// - type is of type enum PVSCSIMsgType.
// - the content of args depend on the type of event being delivered.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIRingMsgDesc {
    pub type: u32,
    pub args: [u32; 31],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIMsgDescDevStatusChanged {
    pub /: *mut *mut u32 type; / PVSCSI_MSG_DEV _ADDED / _REMOVED,
    pub bus: u32,
    pub target: u32,
    pub lun: [u8; 8],
    pub pad: [u32; 27],
    pub __packed: },
//
// Rings state.
//
// - the fields:
// . msgProdIdx,
// . msgConsIdx,
// . msgNumEntriesLog2,
// .. are only used once the SETUP_MSG_RING cmd has been issued.
// - '_pad' helps to ensure that the msg related fields are on their own
// cache-line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIRingsState {
    pub reqProdIdx: u32,
    pub reqConsIdx: u32,
    pub reqNumEntriesLog2: u32,
    pub cmpProdIdx: u32,
    pub cmpConsIdx: u32,
    pub cmpNumEntriesLog2: u32,
    pub reqCallThreshold: u32,
    pub _pad: [u8; 100],
    pub msgProdIdx: u32,
    pub msgConsIdx: u32,
    pub msgNumEntriesLog2: u32,
    pub __packed: },
//
// Request descriptor.
//
// sizeof(RingReqDesc) = 128
//
// - context: is a unique identifier of a command. It could normally be any
// 64bit value, however we currently store it in the serialNumber variable
// of struct SCSI_Command, so we have the following restrictions due to the
// way this field is handled in the vmkernel storage stack:
// * this value can't be 0,
// * the upper 32bit need to be 0 since serialNumber is as a u32.
// Currently tracked as PR 292060.
// - dataLen: contains the total number of bytes that need to be transferred.
// - dataAddr:
// * if PVSCSI_FLAG_CMD_WITH_SG_LIST is set: dataAddr is the PA of the first
// s/g table segment, each s/g segment is entirely contained on a single
// page of physical memory,
// * if PVSCSI_FLAG_CMD_WITH_SG_LIST is NOT set, then dataAddr is the PA of
// the buffer used for the DMA transfer,
// - flags:
// * PVSCSI_FLAG_CMD_WITH_SG_LIST: see dataAddr above,
// * PVSCSI_FLAG_CMD_DIR_NONE: no DMA involved,
// * PVSCSI_FLAG_CMD_DIR_TOHOST: transfer from device to main memory,
// * PVSCSI_FLAG_CMD_DIR_TODEVICE: transfer from main memory to device,
// * PVSCSI_FLAG_CMD_OUT_OF_BAND_CDB: reserved to handle CDBs larger than
// 16bytes. To be specified.
// - vcpuHint: vcpuId of the processor that will be most likely waiting for the
// completion of the i/o. For guest OSes that use lowest priority message
// delivery mode (such as windows), we use this "hint" to deliver the
// completion action to the proper vcpu. For now, we can use the vcpuId of
// the processor that initiated the i/o as a likely candidate for the vcpu
// that will be waiting for the completion..
// - bus should be 0: we currently only support bus 0 for now.
// - unused should be zero'd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIRingReqDesc {
    pub context: u64,
    pub dataAddr: u64,
    pub dataLen: u64,
    pub senseAddr: u64,
    pub senseLen: u32,
    pub flags: u32,
    pub cdb: [u8; 16],
    pub cdbLen: u8,
    pub lun: [u8; 8],
    pub tag: u8,
    pub bus: u8,
    pub target: u8,
    pub vcpuHint: u16,
    pub unused: [u8; 58],
    pub __packed: },
//
// Scatter-gather list management.
//
// As described above, when PVSCSI_FLAG_CMD_WITH_SG_LIST is set in the
// RingReqDesc.flags, then RingReqDesc.dataAddr is the PA of the first s/g
// table segment.
//
// - each segment of the s/g table contain a succession of struct
// PVSCSISGElement.
// - each segment is entirely contained on a single physical page of memory.
// - a "chain" s/g element has the flag PVSCSI_SGE_FLAG_CHAIN_ELEMENT set in
// PVSCSISGElement.flags and in this case:
// * addr is the PA of the next s/g segment,
// * length is undefined, assumed to be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSISGElement {
    pub addr: u64,
    pub length: u32,
    pub flags: u32,
    pub __packed: },
//
// Completion descriptor.
//
// sizeof(RingCmpDesc) = 32
//
// - context: identifier of the command. The same thing that was specified
// under "context" as part of struct RingReqDesc at initiation time,
// - dataLen: number of bytes transferred for the actual i/o operation,
// - senseLen: number of bytes written into the sense buffer,
// - hostStatus: adapter status,
// - scsiStatus: device status,
// - _pad should be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIRingCmpDesc {
    pub context: u64,
    pub dataLen: u64,
    pub senseLen: u32,
    pub hostStatus: u16,
    pub scsiStatus: u16,
    pub _pad: [u32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIConfigPageHeader {
    pub pageNum: u32,
    pub numDwords: u16,
    pub hostStatus: u16,
    pub scsiStatus: u16,
    pub reserved: [u16; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PVSCSIConfigPageController {
    pub header: PVSCSIConfigPageHeader,
    pub /: *mut *mut u64 nodeWWN; / Device name as defined in the SAS spec.,
    pub manufacturer: [u16; 64],
    pub serialNumber: [u16; 64],
    pub opromVersion: [u16; 32],
    pub hwVersion: [u16; 32],
    pub firmwareVersion: [u16; 32],
    pub numPhys: u32,
    pub useConsecutivePhyWWNs: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// Interrupt status / IRQ bits.
//

//
// Number of MSI-X vectors supported.
//
pub const PVSCSI_MAX_INTRS: c_int = 24;
//
// Misc constants for the rings.
//

pub const PVSCSI_MEM_SPACE_COMMAND_NUM_PAGES: c_int = 1;
pub const PVSCSI_MEM_SPACE_INTR_STATUS_NUM_PAGES: c_int = 1;
pub const PVSCSI_MEM_SPACE_MISC_NUM_PAGES: c_int = 2;
pub const PVSCSI_MEM_SPACE_KICK_IO_NUM_PAGES: c_int = 2;
pub const PVSCSI_MEM_SPACE_MSIX_NUM_PAGES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PVSCSIMemSpace {
    PVSCSI_MEM_SPACE_COMMAND_PAGE		= 0,
    PVSCSI_MEM_SPACE_INTR_STATUS_PAGE	= 1,
    PVSCSI_MEM_SPACE_MISC_PAGE		= 2,
    PVSCSI_MEM_SPACE_KICK_IO_PAGE		= 4,
    PVSCSI_MEM_SPACE_MSIX_TABLE_PAGE	= 6,
    PVSCSI_MEM_SPACE_MSIX_PBA_PAGE		= 7,
}

