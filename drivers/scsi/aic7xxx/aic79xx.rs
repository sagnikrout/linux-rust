//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic79xx.h
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
// Core definitions and data structures shareable across OS platforms.
//
// Copyright (c) 1994-2002 Justin T. Gibbs.
// Copyright (c) 2000-2002 Adaptec Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id: //depot/aic7xxx/aic7xxx/aic79xx.h#109 $
//
// $FreeBSD$
//
// Register Definitions

// Forward Declarations
// Useful Macros

pub const TRUE: c_int = 1;

pub const FALSE: c_int = 0;

pub const ALL_TARGETS_MASK: c_uint = 0xFFFF;

pub const SCB_LIST_NULL: c_uint = 0xFF00;

pub const QOUTFIFO_ENTRY_VALID: c_uint = 0x80;

//
// TCLs have the following format: TTTTLLLLLLLL
//

pub const AHD_TMODE_ENABLE: c_int = 0;

pub const AHD_NEVER_COL_IDX: c_uint = 0xFFFF;
// Driver Constants
//
// The maximum number of supported targets.
//
pub const AHD_NUM_TARGETS: c_int = 16;
//
// The maximum number of supported luns.
// The identify message only supports 64 luns in non-packetized transfers.
// You can have 2^64 luns when information unit transfers are enabled,
// but until we see a need to support that many, we support 256.
//
pub const AHD_NUM_LUNS_NONPKT: c_int = 64;
pub const AHD_NUM_LUNS: c_int = 256;
//
// The maximum transfer per S/G segment.
//
pub const AHD_MAXTRANSFER_SIZE: c_uint = 0x00ffffff	/* limited by 24bit counter */;
//
// The maximum amount of SCB storage in hardware on a controller.
// This value represents an upper bound.  Due to software design,
// we may not be able to use this number.
//
pub const AHD_SCB_MAX: c_int = 512;
//
// The maximum number of concurrent transactions supported per driver instance.
// Sequencer Control Blocks (SCBs) store per-transaction information.
//

//
// Define the size of our QIN and QOUT FIFOs.  They must be a power of 2
// in size and accommodate as many transactions as can be queued concurrently.
//

//
// The maximum amount of SCB storage we allocate in host memory.
//

//
// Ring Buffer of incoming target commands.
// We allocate 256 to simplify the logic in the sequencer
// by using the natural wrap point of an 8bit counter.
//
pub const AHD_TMODE_CMDS: c_int = 256;
// Reset line assertion time in us
pub const AHD_BUSRESET_DELAY: c_int = 25;
// Chip Characteristics/Operating Settings
//
// Chip Type
// The chip order is from least sophisticated to most sophisticated.
//
// Features available in each chip type.
//
// Bugs in the silicon that we work around in software.
//
// Rev A hardware fails to update LAST/CURR/NEXTSCB
// correctly in certain packetized selection cases.
//
// The wrong SCB is accessed to check the abort pending bit.
// Packetized bitbucket crosses packet boundaries.
// The selection timer runs twice as long as its setting.
// The Non-LQ CRC error status is delayed until phase change.
// The chip must be reset for all outgoing bus resets.
// Some PCIX fields must be saved and restored across chip reset.
// MMAPIO is not functional in PCI-X mode.
// Reads to SCBRAM fail to reset the discard timer.
// Bug workarounds that can be disabled on non-PCIX busses.
//
// LQOSTOP0 status set even for forced selections with ATN
// to perform non-packetized message delivery.
//
// FIFO auto-flush does not always trigger.
// The CLRLQO registers are not self-clearing.
// The PACKETIZED status bit refers to the previous connection.
// "Short Luns" are not placed into outgoing LQ packets correctly.
//
// Only the FIFO allocated to the non-packetized connection may
// be in use during a non-packetzied connection.
//
// Writing to a DFF SCBPTR register may fail if concurent with
// a hardware write to the other DFF SCBPTR register.  This is
// not currently a concern in our sequencer since all chips with
// this bug have the AHD_NONPACKFIFO_BUG and all writes of concern
// occur in non-packetized connections.
//
// SGHADDR updates are slow.
//
// Changing the MODE_PTR coincident with an interrupt that
// switches to a different mode will cause the interrupt to
// be in the mode written outside of interrupt context.
//
// Non-packetized busfree revision does not work.
//
// Paced transfers are indicated with a non-standard PPR
// option bit in the neg table, 160MHz is indicated by
// sync factor 0x7, and the offset if off by a factor of 2.
//
// LQOOVERRUN false positives.
//
// Controller write to INTSTAT will lose to a host
// write to CLRINT.
//
// The GEM318 violates the SCSI spec by not waiting
// the mandated bus settle delay between phase changes
// in some situations.  Some aic79xx chip revs. are more
// strict in this regard and will treat REQ assertions
// that fall within the bus settle delay window as
// glitches.  This flag tells the firmware to tolerate
// early REQ assertions.
//
// The LED does not stay on long enough in packetized modes.
//
// Configuration specific settings.
// The driver determines these settings by probing the
// chip/controller's configuration.
//
// For cards without an seeprom
// or a BIOS to initialize the chip's
// SRAM, we use the default target
// settings.
//
// Allow initiator operations on
// this controller.
//
// Allow target operations on this
// controller.
//
// Internal 50pin connector
// sits behind an aic3860
//
// Hardware  SCB Definition
//
// The driver keeps up to MAX_SCB scb structures per card in memory.  The SCB
// consists of a "hardware SCB" mirroring the fields available on the card
// and additional information the kernel stores for each transaction.
//
// To minimize space utilization, a portion of the hardware scb stores
// different data during different portions of a SCSI transaction.
// As initialized by the host driver for the initiator role, this area
// contains the SCSI cdb (or a pointer to the  cdb) to be executed.  After
// the cdb has been presented to the target, this area serves to store
// residual transfer information and the SCSI status byte.
// For the target role, the contents of this area do not change, but
// still serve a different purpose than for the initiator role.  See
// struct target_data for details.
//
// Status information embedded in the shared poriton of
// an SCB after passing the cdb to the target.  The kernel
// driver will only read this data for transactions that
// complete abnormally.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initiator_status {
    pub /: *mut *mut uint32_t residual_datacnt; / Residual in the current S/G seg,
    pub /: *mut *mut uint32_t residual_sgptr; / The next S/G for this transfer,
    pub /: *mut *mut uint8_t scsi_status; / Standard SCSI status byte,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_status {
    pub /: *mut *mut uint32_t residual_datacnt; / Residual in the current S/G seg,
    pub /: *mut *mut uint32_t residual_sgptr; / The next S/G for this transfer,
    pub /: *mut *mut uint8_t scsi_status; / SCSI status to give to initiator,
    pub /: *mut *mut uint8_t target_phases; / Bitmap of phases to execute,
    pub /: *mut *mut uint8_t data_phase; / Data-In or Data-Out,
    pub /: *mut *mut uint8_t initiator_tag; / Initiator's transaction tag,
}

//
// Initiator mode SCB shared data area.
// If the embedded CDB is 12 bytes or less, we embed
// the sense buffer address in the SCB.  This allows
// us to retrieve sense information without interrupting
// the host in packetized mode.
//
pub type sense_addr_t = u32;
pub const MAX_CDB_LEN: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub union initiator_data {
    pub cdbptr: u64,
    pub cdblen: u8,
    pub cdb_from_host: },
    pub cdb: [u8; MAX_CDB_LEN],
    pub cdb: [u8; MAX_CDB_LEN_WITH_SENSE_ADDR],
    pub sense_addr: sense_addr_t,
    pub cdb_plus_saddr: },
}

//
// Target mode version of the shared data SCB segment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_data {
    pub spare: [u32; 2],
    pub /: *mut *mut uint8_t scsi_status; / SCSI status to give to initiator,
    pub /: *mut *mut uint8_t target_phases; / Bitmap of phases to execute,
    pub /: *mut *mut uint8_t data_phase; / Data-In or Data-Out,
    pub /: *mut *mut uint8_t initiator_tag; / Initiator's transaction tag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hardware_scb {
// 0*/	union {
    pub idata: initiator_data,
    pub tdata: target_data,
    pub istatus: initiator_status,
    pub tstatus: target_status,
    pub shared_data: },
//
// A word about residuals.
// The scb is presented to the sequencer with the dataptr and datacnt
// fields initialized to the contents of the first S/G element to
// transfer.  The sgptr field is initialized to the bus address for
// the S/G element that follows the first in the in core S/G array
// or'ed with the SG_FULL_RESID flag.  Sgptr may point to an invalid
// S/G entry for this transfer (single S/G element transfer with the
// first elements address and length preloaded in the dataptr/datacnt
// fields).  If no transfer is to occur, sgptr is set to SG_LIST_NULL.
// The SG_FULL_RESID flag ensures that the residual will be correctly
// noted even if no data transfers occur.  Once the data phase is entered,
// the residual sgptr and datacnt are loaded from the sgptr and the
// datacnt fields.  After each S/G element's dataptr and length are
// loaded into the hardware, the residual sgptr is advanced.  After
// each S/G element is expired, its datacnt field is checked to see
// if the LAST_SEG flag is set.  If so, SG_LIST_NULL is set in the
// residual sg ptr and the transfer is considered complete.  If the
// sequencer determines that there is a residual in the tranfer, or
// there is non-zero status, it will set the SG_STATUS_VALID flag in
// sgptr and dma the scb back into host memory.  To sumarize:
//
// Sequencer:
// o A residual has occurred if SG_FULL_RESID is set in sgptr,
// or residual_sgptr does not have SG_LIST_NULL set.
//
// o We are transferring the last segment if residual_datacnt has
// the SG_LAST_SEG flag set.
//
// Host:
// o A residual can only have occurred if a completed scb has the
// SG_STATUS_VALID flag set.  Inspection of the SCSI status field,
// the residual_datacnt, and the residual_sgptr field will tell
// for sure.
//
// o residual_sgptr and sgptr refer to the "next" sg entry
// and so may point beyond the last valid sg entry for the
// transfer.
//
pub const SG_PTR_MASK: c_uint = 0xFFFFFFF8;
// 16*/	uint16_t tag;		/* Reused by Sequencer.
// 18*/	uint8_t  control;	/* See SCB_CONTROL in aic79xx.reg for details
// 19*/	uint8_t	 scsiid;
// Selection out Id
// Our Id (bits 0-3) Their ID (bits 4-7)
//
// 20*/	uint8_t  lun;
// 21*/	uint8_t  task_attribute;
// 22*/	uint8_t  cdb_len;
// 23*/	uint8_t  task_management;
// 24*/	uint64_t dataptr;
// 32*/	uint32_t datacnt;	/* Byte 3 is spare.
// 36*/	uint32_t sgptr;
// 40*/	uint32_t hscb_busaddr;
// 44*/	uint32_t next_hscb_busaddr;
// Long lun field only downloaded for full 8 byte lun support
// 48*/  uint8_t	 pkt_long_lun[8];
// Fields below are not Downloaded (Sequencer may use for scratch)
// 56*/  uint8_t	 spare[8];
}

// Kernel SCB Definitions
//
// Some fields of the SCB are OS dependent.  Here we collect the
// definitions for elements that all OS platforms need to include
// in there SCB definition.
//
// Definition of a scatter/gather element as transferred to the controller.
// The aic7xxx chips only support a 24bit length.  We use the top byte of
// the length to store additional address bits and a flag to indicate
// that a given segment terminates the transfer.  This gives us an
// addressable range of 512GB on machines with 64bit PCI or with chips
// that can support dual address cycles on 32bit PCI busses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_dma_seg {
    pub addr: u32,
    pub len: u32,
pub const AHD_DMA_LAST_SEG: c_uint = 0x80000000;
pub const AHD_SG_HIGH_ADDR_MASK: c_uint = 0x7F000000;
pub const AHD_SG_LEN_MASK: c_uint = 0x00FFFFFF;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_dma64_seg {
    pub addr: u64,
    pub len: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_node {
    pub dmamap: bus_dmamap_t,
    pub physaddr: dma_addr_t,
    pub vaddr: *mut u8,
    pub links: SLIST_ENTRY(map_node),
}

//
// The current state of this SCB.
//
// We detected a parity or CRC
// error that has effected the
// payload of the command.  This
// flag is checked when normal
// status is returned to catch
// the case of a target not
// responding to our attempt
// to report the error.
//
// Another device was active
// during the first timeout for
// this SCB so we gave ourselves
// an additional timeout period
// in case it was hogging the
// bus.
//
// Be quiet about transmission type
// errors.  They are expected and we
// don't want to upset the user.  This
// flag is typically used during DV.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb {
    pub hscb: *mut hardware_scb,
    pub sle: SLIST_ENTRY(scb),
    pub le: LIST_ENTRY(scb),
    pub tqe: TAILQ_ENTRY(scb),
    pub links: },
    pub sle: SLIST_ENTRY(scb),
    pub le: LIST_ENTRY(scb),
    pub tqe: TAILQ_ENTRY(scb),
    pub links2: },

    pub col_scb: *mut scb,
    pub io_ctx: ahd_io_ctx_t,
    pub ahd_softc: *mut ahd_softc,
    pub flags: scb_flag,
    pub platform_data: *mut scb_platform_data,
    pub hscb_map: *mut map_node,
    pub sg_map: *mut map_node,
    pub sense_map: *mut map_node,
    pub sg_list: *mut c_void,
    pub sense_data: *mut u8,
    pub sg_list_busaddr: dma_addr_t,
    pub sense_busaddr: dma_addr_t,
    pub /: *mut *mut u_int sg_count;/ How full ahd_dma_seg is,
pub const AHD_MAX_LQ_CRC_ERRORS: c_int = 5;
    pub crc_retry_count: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_data {
//
// TAILQ of lists of free SCBs grouped by device
// collision domains.
//
    pub free_scbs: scb_tailq,
//
// Per-device lists of SCBs whose tag ID would collide
// with an already active tag on the device.
//
    pub AHD_NUM_LUNS_NONPKT]: *mut *mut scb_list free_scb_lists[AHD_NUM_TARGETS,
//
// SCBs that will not collide with any active device.
//
    pub any_dev_free_scb_list: scb_list,
//
// Mapping from tag to SCB.
//
    pub scbindex: [*mut scb; AHD_SCB_MAX],
//
// "Bus" addresses of our data structures.
//
    pub /: *mut *mut bus_dma_tag_t hscb_dmat; / dmat for our hardware SCB array,
    pub /: *mut *mut bus_dma_tag_t sg_dmat; / dmat for our sg segments,
    pub /: *mut *mut bus_dma_tag_t sense_dmat; / dmat for our sense buffers,
    pub hscb_maps: SLIST_HEAD(, map_node),
    pub sg_maps: SLIST_HEAD(, map_node),
    pub sense_maps: SLIST_HEAD(, map_node),
    pub /: *mut *mut int scbs_left; / unallocated scbs in head map_node,
    pub /: *mut *mut int sgs_left; / unallocated sgs in head map_node,
    pub /: *mut *mut int sense_left; / unallocated sense in head map_node,
    pub numscbs: u16,
    pub /: *mut *mut uint16_t maxhscbs; / Number of SCBs on the card,
    pub /*: *mut uint8_t init_level;,
// How far we've initialized
// this structure.
//
}

// Target Mode Definitions
//
// Connection descriptor for select-in requests in target mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_cmd {
    pub /: *mut *mut uint8_t scsiid; / Our ID and the initiator's ID,
    pub /: *mut *mut uint8_t identify; / Identify message,
    pub /*: *mut uint8_t bytes[22];,
// Bytes contains any additional message
// bytes terminated by 0xFF.  The remainder
// is the cdb to execute.
//
    pub /*: *mut uint8_t cmd_valid;,
// When a command is complete, the firmware
// will set cmd_valid to all bits set.
// After the host has seen the command,
// the bits are cleared.  This allows us
// to just peek at host memory to determine
// if more work is complete. cmd_valid is on
// an 8 byte boundary to simplify setting
// it on aic7880 hardware which only has
// limited direct access to the DMA FIFO.
//
    pub pad: [u8; 7],
}

//
// Number of events we can buffer up if we run out
// of immediate notify ccbs.
//
pub const AHD_TMODE_EVENT_BUFFER_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_tmode_event {
    pub initiator_id: u8,
    pub /: *mut *mut uint8_t event_type; / MSG type or EVENT_TYPE_BUS_RESET,
pub const EVENT_TYPE_BUS_RESET: c_uint = 0xFF;
    pub event_arg: u8,
}

//
// Per enabled lun target mode state.
// As this state is directly influenced by the host OS'es target mode
// environment, we let the OS module define it.  Forward declare the
// structure here so we can store arrays of them, etc. in OS neutral
// data structures.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_tmode_lstate {
    pub path: *mut cam_path,
    pub accept_tios: ccb_hdr_slist,
    pub immed_notifies: ccb_hdr_slist,
    pub event_buffer: [ahd_tmode_event; AHD_TMODE_EVENT_BUFFER_SIZE],
    pub event_r_idx: u8,
    pub event_w_idx: u8,
}

// Transfer Negotiation Datastructures
pub const AHD_TRANS_CUR: c_uint = 0x01	/* Modify current neogtiation status */;
pub const AHD_TRANS_ACTIVE: c_uint = 0x03	/* Assume this target is on the bus */;
pub const AHD_TRANS_GOAL: c_uint = 0x04	/* Modify negotiation goal */;
pub const AHD_TRANS_USER: c_uint = 0x08	/* Modify user negotiation settings */;
pub const AHD_PERIOD_10MHz: c_uint = 0x19;
pub const AHD_WIDTH_UNKNOWN: c_uint = 0xFF;
pub const AHD_PERIOD_UNKNOWN: c_uint = 0xFF;
pub const AHD_OFFSET_UNKNOWN: c_uint = 0xFF;
pub const AHD_PPR_OPTS_UNKNOWN: c_uint = 0xFF;
//
// Transfer Negotiation Information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_transinfo {
    pub /: *mut *mut uint8_t protocol_version; / SCSI Revision level,
    pub /: *mut *mut uint8_t transport_version; / SPI Revision level,
    pub /: *mut *mut uint8_t width; / Bus width,
    pub /: *mut *mut uint8_t period; / Sync rate factor,
    pub /: *mut *mut uint8_t offset; / Sync offset,
    pub /: *mut *mut uint8_t ppr_options; / Parallel Protocol Request options,
}

//
// Per-initiator current, goal and user transfer negotiation information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_initiator_tinfo {
    pub curr: ahd_transinfo,
    pub goal: ahd_transinfo,
    pub user: ahd_transinfo,
}

//
// Per enabled target ID state.
// Pointers to lun target state as well as sync/wide negotiation information
// for each initiator<->target mapping.  For the initiator role we pretend
// that we are the target and the targets are the initiators since the
// negotiation is the same regardless of role.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_tmode_tstate {
    pub enabled_luns: [*mut *mut ahd_tmode_lstate; AHD_NUM_LUNS],
    pub transinfo: [ahd_initiator_tinfo; AHD_NUM_TARGETS],
//
// Per initiator state bitmasks.
//
    pub /: *mut *mut uint16_t auto_negotiate;/ Auto Negotiation Required,
    pub /: *mut *mut uint16_t discenable; / Disconnection allowed,
    pub /: *mut *mut uint16_t tagenable; / Tagged Queuing allowed,
}

//
// Points of interest along the negotiated transfer scale.
//
pub const AHD_SYNCRATE_160: c_uint = 0x8;
pub const AHD_SYNCRATE_PACED: c_uint = 0x8;
pub const AHD_SYNCRATE_DT: c_uint = 0x9;
pub const AHD_SYNCRATE_ULTRA2: c_uint = 0xa;
pub const AHD_SYNCRATE_ULTRA: c_uint = 0xc;
pub const AHD_SYNCRATE_FAST: c_uint = 0x19;

pub const AHD_SYNCRATE_SYNC: c_uint = 0x32;
pub const AHD_SYNCRATE_MIN: c_uint = 0x60;
pub const AHD_SYNCRATE_ASYNC: c_uint = 0xFF;

// Safe and valid period for async negotiations.
pub const AHD_ASYNC_XFER_PERIOD: c_uint = 0x44;
//
// In RevA, the synctable uses a 120MHz rate for the period
// factor 8 and 160MHz for the period factor 7.  The 120MHz
// rate never made it into the official SCSI spec, so we must
// compensate when setting the negotiation table for Rev A
// parts.
//
pub const AHD_SYNCRATE_REVA_120: c_uint = 0x8;
pub const AHD_SYNCRATE_REVA_160: c_uint = 0x7;
// Lookup Tables
//
// Phase -> name and message out response
// to parity errors in each phase table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_phase_table_entry {
    pub phase: u8,
    pub /: *mut *mut uint8_t mesg_out; / Message response to parity errors,
    pub phasemsg: *const c_char,
}

// Serial EEPROM Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seeprom_config {
//
// Per SCSI ID Configuration Flags
//
    pub /: *mut *mut uint16_t device_flags[16]; / words 0-15,
pub const CFXFER: c_uint = 0x003F	/* synchronous transfer rate */;
pub const CFXFER_ASYNC: c_uint = 0x3F;
pub const CFQAS: c_uint = 0x0040	/* Negotiate QAS */;
pub const CFPACKETIZED: c_uint = 0x0080	/* Negotiate Packetized Transfers */;
pub const CFSTART: c_uint = 0x0100	/* send start unit SCSI command */;
pub const CFINCBIOS: c_uint = 0x0200	/* include in BIOS scan */;
pub const CFDISC: c_uint = 0x0400	/* enable disconnection */;
pub const CFMULTILUNDEV: c_uint = 0x0800	/* Probe multiple luns in BIOS scan */;
pub const CFWIDEB: c_uint = 0x1000	/* wide bus device */;
pub const CFHOSTMANAGED: c_uint = 0x8000	/* Managed by a RAID controller */;
//
// BIOS Control Bits
//
    pub /: *mut *mut uint16_t bios_control; / word 16,
pub const CFSUPREM: c_uint = 0x0001	/* support all removeable drives */;
pub const CFSUPREMB: c_uint = 0x0002	/* support removeable boot drives */;
pub const CFBIOSSTATE: c_uint = 0x000C	/* BIOS Action State */;
pub const CFBS_DISABLED: c_uint = 0x00;
pub const CFBS_ENABLED: c_uint = 0x04;
pub const CFBS_DISABLED_SCAN: c_uint = 0x08;
pub const CFENABLEDV: c_uint = 0x0010	/* Perform Domain Validation */;
pub const CFCTRL_A: c_uint = 0x0020	/* BIOS displays Ctrl-A message */;
pub const CFSPARITY: c_uint = 0x0040	/* SCSI parity */;
pub const CFEXTEND: c_uint = 0x0080	/* extended translation enabled */;
pub const CFBOOTCD: c_uint = 0x0100  /* Support Bootable CD-ROM */;
pub const CFMSG_LEVEL: c_uint = 0x0600	/* BIOS Message Level */;
pub const CFMSG_VERBOSE: c_uint = 0x0000;
pub const CFMSG_SILENT: c_uint = 0x0200;
pub const CFMSG_DIAG: c_uint = 0x0400;
pub const CFRESETB: c_uint = 0x0800	/* reset SCSI bus at boot */;
// UNUSED		0xf000
//
// Host Adapter Control Bits
//
    pub /: *mut *mut uint16_t adapter_control; / word 17,
pub const CFAUTOTERM: c_uint = 0x0001	/* Perform Auto termination */;
pub const CFSTERM: c_uint = 0x0002	/* SCSI low byte termination */;
pub const CFWSTERM: c_uint = 0x0004	/* SCSI high byte termination */;
pub const CFSEAUTOTERM: c_uint = 0x0008	/* Ultra2 Perform secondary Auto Term*/;
pub const CFSELOWTERM: c_uint = 0x0010	/* Ultra2 secondary low term */;
pub const CFSEHIGHTERM: c_uint = 0x0020	/* Ultra2 secondary high term */;
pub const CFSTPWLEVEL: c_uint = 0x0040	/* Termination level control */;
pub const CFBIOSAUTOTERM: c_uint = 0x0080	/* Perform Auto termination */;
pub const CFTERM_MENU: c_uint = 0x0100	/* BIOS displays termination menu */;
pub const CFCLUSTERENB: c_uint = 0x8000	/* Cluster Enable */;
//
// Bus Release Time, Host Adapter ID
//
    pub /: *mut *mut uint16_t brtime_id; / word 18,
pub const CFSCSIID: c_uint = 0x000f	/* host adapter SCSI ID */;
// UNUSED		0x00f0
pub const CFBRTIME: c_uint = 0xff00	/* bus release time/PCI Latency Time */;
//
// Maximum targets
//
    pub /: *mut *mut uint16_t max_targets; / word 19,
pub const CFMAXTARG: c_uint = 0x00ff	/* maximum targets */;
pub const CFBOOTLUN: c_uint = 0x0f00	/* Lun to boot from */;
pub const CFBOOTID: c_uint = 0xf000	/* Target to boot from */;
    pub /: *mut *mut uint16_t res_1[10]; / words 20-29,
    pub /: *mut *mut uint16_t signature; / BIOS Signature,
pub const CFSIGNATURE: c_uint = 0x400;
    pub /: *mut *mut uint16_t checksum; / word 31,
}

//
// Vital Product Data used during POST and by the BIOS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpd_config {
    pub bios_flags: u8,
pub const VPDMASTERBIOS: c_uint = 0x0001;
pub const VPDBOOTHOST: c_uint = 0x0002;
    pub reserved_1: [u8; 21],
    pub resource_type: u8,
    pub resource_len: [u8; 2],
    pub resource_data: [u8; 8],
    pub vpd_tag: u8,
    pub vpd_len: u16,
    pub vpd_keyword: [u8; 2],
    pub length: u8,
    pub revision: u8,
    pub device_flags: u8,
    pub termination_menus: [u8; 2],
    pub fifo_threshold: u8,
    pub end_tag: u8,
    pub vpd_checksum: u8,
    pub default_target_flags: u16,
    pub default_bios_flags: u16,
    pub default_ctrl_flags: u16,
    pub default_irq: u8,
    pub pci_lattime: u8,
    pub max_target: u8,
    pub boot_lun: u8,
    pub signature: u16,
    pub reserved_2: u8,
    pub checksum: u8,
    pub reserved_3: [u8; 4],
}

// Flexport Logic
pub const FLXADDR_TERMCTL: c_uint = 0x0;
pub const FLX_TERMCTL_ENSECHIGH: c_uint = 0x8;
pub const FLX_TERMCTL_ENSECLOW: c_uint = 0x4;
pub const FLX_TERMCTL_ENPRIHIGH: c_uint = 0x2;
pub const FLX_TERMCTL_ENPRILOW: c_uint = 0x1;
pub const FLXADDR_ROMSTAT_CURSENSECTL: c_uint = 0x1;
pub const FLX_ROMSTAT_SEECFG: c_uint = 0xF0;
pub const FLX_ROMSTAT_EECFG: c_uint = 0x0F;
pub const FLX_ROMSTAT_SEE_93C66: c_uint = 0x00;
pub const FLX_ROMSTAT_SEE_NONE: c_uint = 0xF0;
pub const FLX_ROMSTAT_EE_512x8: c_uint = 0x0;
pub const FLX_ROMSTAT_EE_1MBx8: c_uint = 0x1;
pub const FLX_ROMSTAT_EE_2MBx8: c_uint = 0x2;
pub const FLX_ROMSTAT_EE_4MBx8: c_uint = 0x3;
pub const FLX_ROMSTAT_EE_16MBx8: c_uint = 0x4;
pub const CURSENSE_ENB: c_uint = 0x1;
pub const FLXADDR_FLEXSTAT: c_uint = 0x2;
pub const FLX_FSTAT_BUSY: c_uint = 0x1;
pub const FLXADDR_CURRENT_STAT: c_uint = 0x4;
pub const FLX_CSTAT_SEC_HIGH: c_uint = 0xC0;
pub const FLX_CSTAT_SEC_LOW: c_uint = 0x30;
pub const FLX_CSTAT_PRI_HIGH: c_uint = 0x0C;
pub const FLX_CSTAT_PRI_LOW: c_uint = 0x03;
pub const FLX_CSTAT_MASK: c_uint = 0x03;
pub const FLX_CSTAT_SHIFT: c_int = 2;
pub const FLX_CSTAT_OKAY: c_uint = 0x0;
pub const FLX_CSTAT_OVER: c_uint = 0x1;
pub const FLX_CSTAT_UNDER: c_uint = 0x2;
pub const FLX_CSTAT_INVALID: c_uint = 0x3;
extern "C" {
    pub fn ahd_verify_cksum(sc: *mut seeprom_config) -> c_int;
}
extern "C" {
    pub fn ahd_acquire_seeprom(ahd: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_release_seeprom(ahd: *mut ahd_softc);
}
// Message Buffer
// Software Configuration Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_suspend_channel_state {
    pub scsiseq: u8,
    pub sxfrctl0: u8,
    pub sxfrctl1: u8,
    pub simode0: u8,
    pub simode1: u8,
    pub seltimer: u8,
    pub seqctl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_suspend_pci_state {
    pub devconfig: u32,
    pub command: u8,
    pub csize_lattime: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_suspend_state {
    pub channel: [ahd_suspend_channel_state; 2],
    pub pci_state: ahd_suspend_pci_state,
    pub optionmode: u8,
    pub dscommand0: u8,
    pub dspcistatus: u8,
// hsmailbox
    pub crccontrol1: u8,
    pub scbbaddr: u8,
// Host and sequencer SCB counts
    pub dff_thrsh: u8,
    pub scratch_ram: *mut u8,
    pub btt: *mut u8,
}

extern "C" {
    pub fn void(: *mut *mut ahd_bus_intr_t)(struct ahd_softc) -> typedef;
}

pub type ahd_mode_state = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_softc {
    pub tags: [bus_space_tag_t; 2],
    pub bshs: [bus_space_handle_t; 2],
    pub scb_data: scb_data,
    pub next_queued_hscb: *mut hardware_scb,
    pub next_queued_hscb_map: *mut map_node,
//
// SCBs that have been sent to the controller
//
    pub pending_scbs: BSD_LIST_HEAD(, scb),
//
// Current register window mode information.
//
    pub dst_mode: ahd_mode,
    pub src_mode: ahd_mode,
//
// Saved register window mode information
// used for restore on next unpause.
//
    pub saved_dst_mode: ahd_mode,
    pub saved_src_mode: ahd_mode,
//
// Platform specific data.
//
    pub platform_data: *mut ahd_platform_data,
//
// Platform specific device information.
//
    pub dev_softc: ahd_dev_softc_t,
//
// Bus specific device information.
//
    pub bus_intr: ahd_bus_intr_t,
//
// Target mode related state kept on a per enabled lun basis.
// Targets that are not enabled will have null entries.
// As an initiator, we keep one target entry for our initiator
// ID to store our sync/wide transfer settings.
//
    pub enabled_targets: [*mut ahd_tmode_tstate; AHD_NUM_TARGETS],
//
// The black hole device responsible for handling requests for
// disabled luns on enabled targets.
//
    pub black_hole: *mut ahd_tmode_lstate,
//
// Device instance currently on the bus awaiting a continue TIO
// for a command that was not given the disconnect priveledge.
//
    pub pending_device: *mut ahd_tmode_lstate,
//
// Timer handles for timer driven callbacks.
//
    pub stat_timer: timer_list,
//
// Statistics.
//

pub const AHD_STAT_BUCKETS: c_int = 4;
    pub cmdcmplt_bucket: u_int,
    pub cmdcmplt_counts: [u32; AHD_STAT_BUCKETS],
    pub cmdcmplt_total: u32,
//
// Card characteristics
//
    pub chip: ahd_chip,
    pub features: ahd_feature,
    pub bugs: ahd_bug,
    pub flags: ahd_flag,
    pub seep_config: *mut seeprom_config,
// Command Queues
    pub qoutfifo: *mut ahd_completion,
    pub qoutfifonext: u16,
    pub qoutfifonext_valid_tag: u16,
    pub qinfifonext: u16,
    pub qinfifo: [u16; AHD_SCB_MAX],
//
// Our qfreeze count.  The sequencer compares
// this value with its own counter to determine
// whether to allow selections to occur.
//
    pub qfreeze_cnt: u16,
// Values to store in the SEQCTL register for pause and unpause
    pub unpause: u8,
    pub pause: u8,
// Critical Section Data
    pub critical_sections: *mut cs,
    pub num_critical_sections: u_int,
// Buffer for handling packetized bitbucket.
    pub overrun_buf: *mut u8,
// Links for chaining softcs
    pub links: TAILQ_ENTRY(ahd_softc),
// Channel Names ('A', 'B', etc.)
    pub channel: c_char,
// Initiator Bus ID
    pub our_id: u8,
//
// Target incoming command FIFO.
//
    pub targetcmds: *mut target_cmd,
    pub tqinfifonext: u8,
//
// Cached version of the hs_mailbox so we can avoid
// pausing the sequencer during mailbox updates.
//
    pub hs_mailbox: u8,
//
// Incoming and outgoing message handling.
//
    pub send_msg_perror: u8,
    pub msg_flags: ahd_msg_flags,
    pub msg_type: ahd_msg_type,
    pub /: *mut *mut uint8_t msgout_buf[12];/ Message we are sending,
    pub /: *mut *mut uint8_t msgin_buf[12];/ Message we are receiving,
    pub /: *mut *mut u_int msgout_len; / Length of message to send,
    pub /: *mut *mut u_int msgout_index; / Current index in msgout,
    pub /: *mut *mut u_int msgin_index; / Current index in msgin,
//
// Mapping information for data structures shared
// between the sequencer and kernel.
//
    pub parent_dmat: bus_dma_tag_t,
    pub shared_data_dmat: bus_dma_tag_t,
    pub shared_data_map: map_node,
// Information saved through suspend/resume cycles
    pub suspend_state: ahd_suspend_state,
// Number of enabled target mode device on this card
    pub enabled_luns: u_int,
// Initialization level of this data structure
    pub init_level: u_int,
// PCI cacheline size.
    pub pci_cachesize: u_int,
// IO Cell Parameters
    pub iocell_opts: [u8; AHD_NUM_PER_DEV_ANNEXCOLS],
    pub stack_size: u_int,
    pub saved_stack: *mut u16,
// Per-Unit descriptive information
    pub description: *const c_char,
    pub bus_description: *const c_char,
    pub name: *mut c_char,
    pub unit: c_int,
// Selection Timer settings
    pub seltime: c_int,
//
// Interrupt coalescing settings.
//

pub const AHD_INT_COALESCING_MAXCMDS_DEFAULT: c_int = 10;
pub const AHD_INT_COALESCING_MAXCMDS_MAX: c_int = 127;
pub const AHD_INT_COALESCING_MINCMDS_DEFAULT: c_int = 5;
pub const AHD_INT_COALESCING_MINCMDS_MAX: c_int = 127;
pub const AHD_INT_COALESCING_THRESHOLD_DEFAULT: c_int = 2000;
pub const AHD_INT_COALESCING_STOP_THRESHOLD_DEFAULT: c_int = 1000;
    pub int_coalescing_timer: u_int,
    pub int_coalescing_maxcmds: u_int,
    pub int_coalescing_mincmds: u_int,
    pub int_coalescing_threshold: u_int,
    pub int_coalescing_stop_threshold: u_int,
    pub /: *mut *mut uint16_t user_discenable;/ Disconnection allowed,
    pub /: *mut *mut uint16_t user_tagenable;/ Tagged Queuing allowed,
}

// IO Cell Configuration

// Active Device Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_devinfo {
    pub our_scsiid: c_int,
    pub target_offset: c_int,
    pub target_mask: u16,
    pub target: u_int,
    pub lun: u_int,
    pub channel: c_char,
    pub /*: *mut role_t role;,
// Only guaranteed to be correct if not
// in the busfree state.
//
}

// PCI Structures

extern "C" {
    pub fn int(: *mut ahd_device_setup_t)(struct ahd_softc) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahd_pci_identity {
    pub full_id: u64,
    pub id_mask: u64,
    pub name: *const c_char,
    pub setup: *mut ahd_device_setup_t,
}

// VL/EISA Declarations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aic7770_identity {
    pub full_id: u32,
    pub id_mask: u32,
    pub name: *const c_char,
    pub setup: *mut ahd_device_setup_t,
}

pub const AHD_EISA_SLOT_OFFSET: c_uint = 0xc00;
pub const AHD_EISA_IOSIZE: c_uint = 0x100;
// Function Declarations
//
// PCI Front End
extern "C" {
    pub fn ahd_pci_test_register_access(: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_pci_suspend(: *mut ahd_softc) -> void __maybe_unused;
}
extern "C" {
    pub fn ahd_pci_resume(: *mut ahd_softc) -> void __maybe_unused;
}
// SCB and SCB queue management
// Initialization
extern "C" {
    pub fn ahd_softc_init(: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_controller_info(ahd: *mut ahd_softc, buf: *mut c_char);
}
extern "C" {
    pub fn ahd_init(ahd: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_suspend(ahd: *mut ahd_softc) -> int __maybe_unused;
}
extern "C" {
    pub fn ahd_resume(ahd: *mut ahd_softc) -> void __maybe_unused;
}
extern "C" {
    pub fn ahd_default_config(ahd: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_intr_enable(ahd: *mut ahd_softc, enable: c_int);
}
extern "C" {
    pub fn ahd_pause_and_flushwork(ahd: *mut ahd_softc);
}
extern "C" {
    pub fn ahd_set_unit(: *mut ahd_softc, _arg: c_int);
}
extern "C" {
    pub fn ahd_set_name(: *mut ahd_softc, : *mut c_char);
}
extern "C" {
    pub fn ahd_free_scb(ahd: *mut ahd_softc, scb: *mut scb);
}
extern "C" {
    pub fn ahd_free(ahd: *mut ahd_softc);
}
extern "C" {
    pub fn ahd_reset(ahd: *mut ahd_softc, reinit: c_int) -> c_int;
}
// Error Recovery
// Utility Functions
// Transfer Negotiation
//
// Negotiation types.  These are used to qualify if we should renegotiate
// even if our goal and current transport parameters are identical.
//
// Target Mode

pub const AHD_TMODE_ENABLE: c_int = 0;

// Debug

pub const AHD_SHOW_MISC: c_uint = 0x00001;
pub const AHD_SHOW_SENSE: c_uint = 0x00002;
pub const AHD_SHOW_RECOVERY: c_uint = 0x00004;
pub const AHD_DUMP_SEEPROM: c_uint = 0x00008;
pub const AHD_SHOW_TERMCTL: c_uint = 0x00010;
pub const AHD_SHOW_MEMORY: c_uint = 0x00020;
pub const AHD_SHOW_MESSAGES: c_uint = 0x00040;
pub const AHD_SHOW_MODEPTR: c_uint = 0x00080;
pub const AHD_SHOW_SELTO: c_uint = 0x00100;
pub const AHD_SHOW_FIFOS: c_uint = 0x00200;
pub const AHD_SHOW_QFULL: c_uint = 0x00400;
pub const AHD_SHOW_DV: c_uint = 0x00800;
pub const AHD_SHOW_MASKED_ERRORS: c_uint = 0x01000;
pub const AHD_SHOW_QUEUE: c_uint = 0x02000;
pub const AHD_SHOW_TQIN: c_uint = 0x04000;
pub const AHD_SHOW_SG: c_uint = 0x08000;
pub const AHD_SHOW_INT_COALESCING: c_uint = 0x10000;
pub const AHD_DEBUG_SEQUENCER: c_uint = 0x20000;

extern "C" {
    pub fn ahd_dump_card_state(ahd: *mut ahd_softc);
}
