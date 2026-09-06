//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptbase.h
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
// linux/drivers/message/fusion/mptbase.h
// High performance SCSI + LAN / Fibre Channel device drivers.
// For use with PCI chip/adapter(s):
// LSIFC9xx/LSI409xx Fibre Channel
// running LSI Fusion MPT (Message Passing Technology) firmware.
//
// Copyright (c) 1999-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//

// Macro flag: #define MPTBASE_H_INCLUDED
// {-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// Fusion MPT(linux) driver configurable stuff...
//
pub const MPT_MAX_ADAPTERS: c_int = 18;
pub const MPT_MAX_PROTOCOL_DRIVERS: c_int = 16;
pub const MPT_MAX_CALLBACKNAME_LEN: c_int = 49;

pub const MPT_MAX_FC_DEVICES: c_int = 255;
pub const MPT_MAX_SCSI_DEVICES: c_int = 16;
pub const MPT_LAST_LUN: c_int = 255;
pub const MPT_SENSE_BUFFER_ALLOC: c_int = 64;
// allow for 256 max sense alloc, but only 255 max request

pub const MPT_NAME_LENGTH: c_int = 32;
pub const MPT_KOBJ_NAME_LEN: c_int = 20;

// chg it to "driver/fusion" ?

pub const MPT_MAX_REQ_DEPTH: c_int = 1023;
pub const MPT_DEFAULT_REQ_DEPTH: c_int = 256;
pub const MPT_MIN_REQ_DEPTH: c_int = 128;

pub const MPT_DEFAULT_REPLY_DEPTH: c_int = 128;
pub const MPT_MIN_REPLY_DEPTH: c_int = 8;
pub const MPT_MAX_REPLIES_PER_ISR: c_int = 32;
pub const MPT_MAX_FRAME_SIZE: c_int = 128;
pub const MPT_DEFAULT_FRAME_SIZE: c_int = 128;
pub const MPT_REPLY_FRAME_SIZE: c_uint = 0x50  /* Must be a multiple of 8 */;
pub const MPT_SG_REQ_128_SCALE: c_int = 1;
pub const MPT_SG_REQ_96_SCALE: c_int = 2;
pub const MPT_SG_REQ_64_SCALE: c_int = 4;
pub const CAN_SLEEP: c_int = 1;
pub const NO_SLEEP: c_int = 0;
pub const MPT_COALESCING_TIMEOUT: c_uint = 0x10;
//
// SCSI transfer rate defines.
//
pub const MPT_ULTRA320: c_uint = 0x08;
pub const MPT_ULTRA160: c_uint = 0x09;
pub const MPT_ULTRA2: c_uint = 0x0A;
pub const MPT_ULTRA: c_uint = 0x0C;
pub const MPT_FAST: c_uint = 0x19;
pub const MPT_SCSI: c_uint = 0x32;
pub const MPT_ASYNC: c_uint = 0xFF;
pub const MPT_NARROW: c_int = 0;
pub const MPT_WIDE: c_int = 1;
pub const C0_1030: c_uint = 0x08;
pub const XL_929: c_uint = 0x01;
//
// Try to keep these at 2^N-1
//
pub const MPT_FC_CAN_QUEUE: c_int = 1024;
pub const MPT_SCSI_CAN_QUEUE: c_int = 127;
pub const MPT_SAS_CAN_QUEUE: c_int = 127;
//
// Set the MAX_SGE value based on user input.
//

pub const MPT_SCSI_SG_DEPTH: c_int = 16;

pub const MPT_SCSI_SG_DEPTH: c_int = 128;

pub const MPT_SCSI_SG_DEPTH: c_int = 40;

pub const MPT_SCSI_FC_SG_DEPTH: c_int = 16;

pub const MPT_SCSI_FC_SG_DEPTH: c_int = 256;

pub const MPT_SCSI_FC_SG_DEPTH: c_int = 40;

// debug print string length used for events and iocstatus

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// Attempt semi-consistent error & warning msgs across
// MPT drivers.  NOTE: Users of these macro defs must
// themselves define their own MYNAM.
//

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// ATTO UL4D associated structures and defines
//
pub const ATTOFLAG_DISC: c_uint = 0x0001;
pub const ATTOFLAG_TAGGED: c_uint = 0x0002;
pub const ATTOFLAG_WIDE_ENB: c_uint = 0x0008;
pub const ATTOFLAG_ID_ENB: c_uint = 0x0010;
pub const ATTOFLAG_LUN_ENB: c_uint = 0x0060;
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// MPT protocol driver defs...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt_pci_driver {
    pub dev): *mut *mut int (probe) (struct pci_dev,
    pub dev): *mut *mut void (remove) (struct pci_dev,
}

//
// MPT adapter / port / bus / device info structures...
//
// NOTE: When request frames are free, on the linkage structure
// contents are valid.  All other values are invalid.
// In particular, do NOT reply on offset [2]
// (in words) being the * message context.
// The message context must be reset (computed via base address
// + an offset) prior to issuing any command.
//
// NOTE2: On non-32-bit systems, where pointers are LARGE,
// using the linkage pointers destroys our sacred MsgContext
// field contents.  But we don't care anymore because these
// are now reset in mpt_put_msg_frame() just prior to sending
// a request off to the IOC.
//
// The following _MUST_ match the location of the
// MsgContext field in the MPT message headers.
//
// Remark: 32 bit identifier:
// 31-24: reserved
// 23-16: call back index
// 15-0 : request index
//
// We might want to view/access a frame as:
// 1) generic request header
// 2) SCSIIORequest
// 3) SCSIIOReply
// 4) MPIDefaultReply
// 5) frame tracker
//
pub const MPT_REQ_MSGFLAGS_DROPME: c_uint = 0x80;
//
// System interface register set
//
// NOTE: Use MPI_{DOORBELL,WRITESEQ,DIAG}_xxx defs in lsi/mpi.h
// in conjunction with SYSIF_REGS accesses!
//
// Dynamic Multi-Pathing specific stuff...
//
// VirtTarget negoFlags field
pub const MPT_TARGET_NO_NEGO_WIDE: c_uint = 0x01;
pub const MPT_TARGET_NO_NEGO_SYNC: c_uint = 0x02;
pub const MPT_TARGET_NO_NEGO_QAS: c_uint = 0x04;
pub const MPT_TAPE_NEGO_IDP: c_uint = 0x08;
//
// VirtDevice - FC LUN device or SCSI target device
//
// Fibre Channel (SCSI) target device and associated defines...
//
pub const MPT_TARGET_DEFAULT_DV_STATUS: c_uint = 0x00;
pub const MPT_TARGET_FLAGS_VALID_NEGO: c_uint = 0x01;
pub const MPT_TARGET_FLAGS_VALID_INQUIRY: c_uint = 0x02;
pub const MPT_TARGET_FLAGS_Q_YES: c_uint = 0x08;
pub const MPT_TARGET_FLAGS_VALID_56: c_uint = 0x10;
pub const MPT_TARGET_FLAGS_SAF_TE_ISSUED: c_uint = 0x20;
pub const MPT_TARGET_FLAGS_RAID_COMPONENT: c_uint = 0x40;
pub const MPT_TARGET_FLAGS_LED_ON: c_uint = 0x80;
//
// IOCTL structure and associated defines
//
pub const MPTCTL_RESET_OK: c_uint = 0x01	/* Issue Bus Reset */;
pub const MPT_MGMT_STATUS_RF_VALID: c_uint = 0x01	/* The Reply Frame is VALID */;
pub const MPT_MGMT_STATUS_COMMAND_GOOD: c_uint = 0x02	/* Command Status GOOD */;
pub const MPT_MGMT_STATUS_PENDING: c_uint = 0x04	/* command is pending */;
pub const MPT_MGMT_STATUS_DID_IOCRESET: c_uint = 0x08	/* IOC Reset occurred;
pub const MPT_MGMT_STATUS_SENSE_VALID: c_uint = 0x10	/* valid sense info */;
pub const MPT_MGMT_STATUS_TIMER_ACTIVE: c_uint = 0x20	/* obsolete */;
pub const MPT_MGMT_STATUS_FREE_MF: c_uint = 0x40	/* free the mf from;

//
// Event Structure and define
//

//
// CONFIGPARM status  defines
//

pub const MPT_CONFIG_ERROR: c_uint = 0x002F;
//
// Substructure to store SCSI specific configuration page data
//
// dvStatus defines:
pub const MPT_SCSICFG_USE_NVRAM: c_uint = 0x01	/* WriteSDP1 using NVRAM */;
pub const MPT_SCSICFG_ALL_IDS: c_uint = 0x02	/* WriteSDP1 to all IDS */;
// #define MPT_SCSICFG_BLK_NEGO		0x10	   WriteSDP1 with WDTR and SDTR disabled
// SAF-TE if Inquiry data length
// is too short to check for SAF-TE
//
// persistent table.
// 0 to disable
// automatic clearing.
//
// Inactive volume link list of raid component data
// @inactive_list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inactive_raid_component_info {
    pub list: list_head,
    pub /: *mut *mut u8 volumeID; / volume target id,
    pub /: *mut *mut u8 volumeBus; / volume channel,
    pub /: *mut *mut IOC_3_PHYS_DISK d; / phys disk info,
}

// will ultimately hold fc_port_page0 also
pub const MPT_RPORT_INFO_FLAGS_REGISTERED: c_uint = 0x01	/* rport registered */;
pub const MPT_RPORT_INFO_FLAGS_MISSING: c_uint = 0x02	/* missing from DevPage0 scan */;
//
// data allocated for each fc rport device
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// MPT_SCSI_HOST defines - Used by the IOCTL and the SCSI drivers
// Private to the driver.
//

extern "C" {
    pub fn void(pAddr: *mut *mut MPT_ADD_SGE)(void, flagslength: u32, dma_addr: dma_addr_t) -> typedef;
}
extern "C" {
    pub fn void(ioc: *mut *mut MPT_SCHEDULE_TARGET_RESET)(void) -> typedef;
}
extern "C" {
    pub fn void(hd: *mut *mut MPT_FLUSH_RUNNING_CMDS)(MPT_SCSI_HOST) -> typedef;
}
//
// Adapter Structure - pci_dev specific. Maximum: MPT_MAX_ADAPTERS
//

// used in mpt_display_event_info

// Pool of buffers for chaining. ReqToChain
// and ChainToChain track index of chain buffers.
// ChainBuffer (DMA) virt/phys addresses.
// FreeChainQ (lock) locking mechanisms.
//
// We (host driver) get to manage our own RequestQueue!
// Pool of SCSI sense buffers for commands coming from
// the SCSI mid-layer.  We have one 256 byte sense buffer
// for each REQ entry.
//

//
// Description: errata_flag_1064
// If a PCIX read occurs within 1 or 2 cycles after the chip receives
// a split completion for a read data, an internal address pointer incorrectly
// increments by 32 bytes
//
// port_info object for the host
// driver forced bus resets count
// fw/external bus resets count
// cmd timeouts
//
// New return value convention:
// 1 = Ok to free associated request frame
// 0 = not Ok ...
//
extern "C" {
    pub fn int(ioc: *mut *mut MPT_CALLBACK)(MPT_ADAPTER, req: *mut MPT_FRAME_HDR, reply: *mut MPT_FRAME_HDR) -> typedef;
}
extern "C" {
    pub fn int(ioc: *mut *mut MPT_EVHANDLER)(MPT_ADAPTER, evReply: *mut EventNotificationReply_t) -> typedef;
}
extern "C" {
    pub fn int(ioc: *mut *mut MPT_RESETHANDLER)(MPT_ADAPTER, reset_phase: c_int) -> typedef;
}
// reset_phase defs
pub const MPT_IOC_PRE_RESET: c_int = 0;
pub const MPT_IOC_POST_RESET: c_int = 1;
pub const MPT_IOC_SETUP_RESET: c_int = 2;
//
// Invent MPT host event (super-set of MPI Events)
// Fitted to 1030's 64-byte [max] request frame size
//
pub const MPT_HOSTEVENT_IOC_BRINGUP: c_uint = 0x91;
pub const MPT_HOSTEVENT_IOC_RECOVER: c_uint = 0x92;
// Define the generic types based on the size
// of the dma_addr_t type.
//

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// Funky (private) macros...
//

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
pub const SCSI_STD_SENSE_BYTES: c_int = 18;
pub const SCSI_STD_INQUIRY_BYTES: c_int = 36;
pub const SCSI_MAX_INQUIRY_BYTES: c_int = 96;
//
// MPT_SCSI_HOST defines - Used by the IOCTL and the SCSI drivers
// Private to the driver.
//
// LOCAL structure and fields used when processing
// internally generated commands. These include:
// bus scan, dv and config requests.
//
// The TM_STATE variable is used to provide strict single threading of TM
// requests as well as communicate TM error conditions.
//

// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// More Dynamic Multi-Pathing stuff...
//
// Forward decl, a strange C thing, to prevent gcc compiler warnings
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// Generic structure passed to the base mpt_config function.
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// Public entry points...
//
extern "C" {
    pub fn mpt_attach(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn mpt_detach(pdev: *mut pci_dev);
}

extern "C" {
    pub fn mpt_suspend(pdev: *mut pci_dev, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn mpt_resume(pdev: *mut pci_dev) -> c_int;
}

extern "C" {
    pub fn mpt_deregister(cb_idx: u8);
}
extern "C" {
    pub fn mpt_event_register(cb_idx: u8, ev_cbfunc: MPT_EVHANDLER) -> c_int;
}
extern "C" {
    pub fn mpt_event_deregister(cb_idx: u8);
}
extern "C" {
    pub fn mpt_reset_register(cb_idx: u8, reset_func: MPT_RESETHANDLER) -> c_int;
}
extern "C" {
    pub fn mpt_reset_deregister(cb_idx: u8);
}
extern "C" {
    pub fn mpt_device_driver_register(dd_cbfunc: *mut *mut mpt_pci_driver, cb_idx: u8) -> c_int;
}
extern "C" {
    pub fn mpt_device_driver_deregister(cb_idx: u8);
}
extern "C" {
    pub fn mpt_free_msg_frame(ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR);
}
extern "C" {
    pub fn mpt_put_msg_frame(cb_idx: u8, ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR);
}
extern "C" {
    pub fn mpt_put_msg_frame_hi_pri(cb_idx: u8, ioc: *mut MPT_ADAPTER, mf: *mut MPT_FRAME_HDR);
}
extern "C" {
    pub fn mpt_send_handshake_request(cb_idx: u8, ioc: *mut MPT_ADAPTER, reqBytes: c_int, req: *mut u32, sleepFlag: c_int) -> c_int;
}
extern "C" {
    pub fn mpt_verify_adapter(iocid: c_int, iocpp: *mut MPT_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mpt_GetIocState(ioc: *mut MPT_ADAPTER, cooked: c_int) -> u32;
}
extern "C" {
    pub fn mpt_print_ioc_summary(ioc: *mut MPT_ADAPTER, buf: *mut c_char, size: *mut c_int, len: c_int, showlan: c_int);
}
extern "C" {
    pub fn mpt_HardResetHandler(ioc: *mut MPT_ADAPTER, sleepFlag: c_int) -> c_int;
}
extern "C" {
    pub fn mpt_Soft_Hard_ResetHandler(ioc: *mut MPT_ADAPTER, sleepFlag: c_int) -> c_int;
}
extern "C" {
    pub fn mpt_config(ioc: *mut MPT_ADAPTER, cfg: *mut CONFIGPARMS) -> c_int;
}
extern "C" {
    pub fn mpt_alloc_fw_memory(ioc: *mut MPT_ADAPTER, size: c_int) -> c_int;
}
extern "C" {
    pub fn mpt_free_fw_memory(ioc: *mut MPT_ADAPTER);
}
extern "C" {
    pub fn mpt_findImVolumes(ioc: *mut MPT_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mptbase_sas_persist_operation(ioc: *mut MPT_ADAPTER, persist_opcode: u8) -> c_int;
}
extern "C" {
    pub fn mpt_raid_phys_disk_pg0(ioc: *mut MPT_ADAPTER, phys_disk_num: u8, phys_disk: pRaidPhysDiskPage0_t) -> c_int;
}
extern "C" {
    pub fn mpt_set_taskmgmt_in_progress_flag(ioc: *mut MPT_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mpt_clear_taskmgmt_in_progress_flag(ioc: *mut MPT_ADAPTER);
}
extern "C" {
    pub fn mpt_halt_firmware(ioc: *mut MPT_ADAPTER) -> void __noreturn;
}
//
// Public data decl's...
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

//
// Shifted SGE Defines - Use in SGE with FlagsLength member.
// Otherwise, use MPI_xxx defines (refer to "lsi/mpi.h" header).
// Defaults: 32 bit SGE, SYSTEM_ADDRESS if direction bit is 0, read
//

// }-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
