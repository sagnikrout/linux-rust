//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic7xxx.h
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
// Copyright (c) 1994-2001 Justin T. Gibbs.
// Copyright (c) 2000-2001 Adaptec Inc.
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
// $Id: //depot/aic7xxx/aic7xxx/aic7xxx.h#85 $
//
// $FreeBSD$
//
// Register Definitions

// Forward Declarations
// Useful Macros

pub const TRUE: c_int = 1;

pub const FALSE: c_int = 0;

pub const ALL_TARGETS_MASK: c_uint = 0xFFFF;

pub const AHC_TMODE_ENABLE: c_int = 0;

// Driver Constants
//
// The maximum number of supported targets.
//
pub const AHC_NUM_TARGETS: c_int = 16;
//
// The maximum number of supported luns.
// The identify message only supports 64 luns in SPI3.
// You can have 2^64 luns when information unit transfers are enabled,
// but it is doubtful this driver will ever support IUTs.
//
pub const AHC_NUM_LUNS: c_int = 64;
//
// The maximum transfer per S/G segment.
//
pub const AHC_MAXTRANSFER_SIZE: c_uint = 0x00ffffff	/* limited by 24bit counter */;
//
// The maximum amount of SCB storage in hardware on a controller.
// This value represents an upper bound.  Controllers vary in the number
// they actually support.
//
pub const AHC_SCB_MAX: c_int = 255;
//
// The maximum number of concurrent transactions supported per driver instance.
// Sequencer Control Blocks (SCBs) store per-transaction information.  Although
// the space for SCBs on the host adapter varies by model, the driver will
// page the SCBs between host and controller memory as needed.  We are limited
// to 253 because:
// 1) The 8bit nature of the RISC engine holds us to an 8bit value.
// 2) We reserve one value, 255, to represent the invalid element.
// 3) Our input queue scheme requires one SCB to always be reserved
// in advance of queuing any SCBs.  This takes us down to 254.
// 4) To handle our output queue correctly on machines that only
// support 32bit stores, we must clear the array 4 bytes at a
// time.  To avoid colliding with a DMA write from the sequencer,
// we must be sure that 4 slots are empty when we write to clear
// the queue.  This reduces us to 253 SCBs: 1 that just completed
// and the known three additional empty slots in the queue that
// precede it.
//
pub const AHC_MAX_QUEUE: c_int = 253;
//
// The maximum amount of SCB storage we allocate in host memory.  This
// number should reflect the 1 additional SCB we require to handle our
// qinfifo mechanism.
//

//
// Ring Buffer of incoming target commands.
// We allocate 256 to simplify the logic in the sequencer
// by using the natural wrap point of an 8bit counter.
//
pub const AHC_TMODE_CMDS: c_int = 256;
// Reset line assertion time in us
pub const AHC_BUSRESET_DELAY: c_int = 25;
// Chip Characteristics/Operating Settings
//
// Chip Type
// The chip order is from least sophisticated to most sophisticated.
//
// Features available in each chip type.
//
// The real 7850 does not support Ultra modes, but there are
// several cards that use the generic 7850 PCI ID even though
// they are using an Ultra capable chip (7859/7860).  We start
// out with the AHC_ULTRA feature set and then check the DEVSTATUS
// register to determine if the capability is really present.
//
// Although we have space for both the initiator and
// target roles on ULTRA2 chips, we currently disable
// the initiator role to allow multi-scsi-id target mode
// configurations.  We can only respond on the same SCSI
// ID as our initiator role if we allow initiator operation.
// At some point, we should add a configuration knob to
// allow both roles to be loaded.
//
// Bugs in the silicon that we work around in software.
//
// On all chips prior to the U2 product line,
// the WIDEODD S/G segment feature does not
// work during scsi->HostBus transfers.
//
// On the aic7890/91 Rev 0 chips, the autoflush
// feature does not work.  A manual flush of
// the DMA FIFO is required.
//
// On many chips, cacheline streaming does not work.
//
// On the aic7896/97 chips, cacheline
// streaming must be enabled.
//
// PCI 2.1 Retry failure on non-empty data fifo.
//
// Controller does not handle cacheline residuals
// properly on S/G segments if PCI MWI instructions
// are allowed.
//
// An SCB upload using the SCB channel's
// auto array entry copy feature may
// corrupt data.  This appears to only
// occur on 66MHz systems.
//
// Configuration specific settings.
// The driver determines these settings by probing the
// chip/controller's configuration.
//
// The channel that should
// be probed first.
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
// The busy targets table is
// stored in SCB space rather
// than SRAM.
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
// complete abnormally (non-zero status byte).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_pkt {
    pub /: *mut *mut uint32_t residual_datacnt; / Residual in the current S/G seg,
    pub /: *mut *mut uint32_t residual_sg_ptr; / The next S/G for this transfer,
    pub /: *mut *mut uint8_t scsi_status; / Standard SCSI status byte,
}

//
// Target mode version of the shared data SCB segment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_data {
    pub /: *mut *mut uint32_t residual_datacnt; / Residual in the current S/G seg,
    pub /: *mut *mut uint32_t residual_sg_ptr; / The next S/G for this transfer,
    pub /: *mut *mut uint8_t scsi_status; / SCSI status to give to initiator,
    pub /: *mut *mut uint8_t target_phases; / Bitmap of phases to execute,
    pub /: *mut *mut uint8_t data_phase; / Data-In or Data-Out,
    pub /: *mut *mut uint8_t initiator_tag; / Initiator's transaction tag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hardware_scb {
// 0*/	union {
//
// If the cdb is 12 bytes or less, we embed it directly
// in the SCB.  For longer cdbs, we embed the address
// of the cdb payload as seen by the chip and a DMA
// is used to pull it in.
//
    pub cdb: [u8; 12],
    pub cdb_ptr: u32,
    pub status: status_pkt,
    pub tdata: target_data,
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
// sequencer determines that there is a residual in the tranfer, it
// will set the SG_RESID_VALID flag in sgptr and dma the scb back into
// host memory.  To sumarize:
//
// Sequencer:
// o A residual has occurred if SG_FULL_RESID is set in sgptr,
// or residual_sgptr does not have SG_LIST_NULL set.
//
// o We are transferring the last segment if residual_datacnt has
// the SG_LAST_SEG flag set.
//
// Host:
// o A residual has occurred if a completed scb has the
// SG_RESID_VALID flag set.
//
// o residual_sgptr and sgptr refer to the "next" sg entry
// and so may point beyond the last valid sg entry for the
// transfer.
//
// 12*/	uint32_t dataptr;
// 16*/	uint32_t datacnt;
// Byte 3 (numbered from 0) of
// the datacnt is really the
// 4th byte in that data address.
//
// 20*/	uint32_t sgptr;
pub const SG_PTR_MASK: c_uint = 0xFFFFFFF8;
// 24*/	uint8_t  control;	/* See SCB_CONTROL in aic7xxx.reg for details
// 25*/	uint8_t  scsiid;	/* what to load in the SCSIID register
// 26*/	uint8_t  lun;
// 27*/	uint8_t  tag;
// Index into our kernel SCB array.
// Also used as the tag for tagged I/O
//
// 28*/	uint8_t  cdb_len;
// 29*/	uint8_t  scsirate;		/* Value for SCSIRATE register
// 30*/	uint8_t  scsioffset;		/* Value for SCSIOFFSET register
// 31*/	uint8_t  next;
// Used for threading SCBs in the
// "Waiting for Selection" and
// "Disconnected SCB" lists down
// in the sequencer.
//
// 32*/	uint8_t  cdb32[32];
// CDB storage for cdbs of size
// 13->32.  We store them here
// because hardware scbs are
// allocated from DMA safe
// memory so we are guaranteed
// the controller can access
// this data.
//
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
pub struct ahc_dma_seg {
    pub addr: u32,
    pub len: u32,
pub const AHC_DMA_LAST_SEG: c_uint = 0x80000000;
pub const AHC_SG_HIGH_ADDR_MASK: c_uint = 0x7F000000;
pub const AHC_SG_LEN_MASK: c_uint = 0x00FFFFFF;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_map_node {
    pub sg_dmamap: bus_dmamap_t,
    pub sg_physaddr: dma_addr_t,
    pub sg_vaddr: *mut *mut ahc_dma_seg,
    pub links: SLIST_ENTRY(sg_map_node),
}

//
// The current state of this SCB.
//
// Another device was active
// during the first timeout for
// this SCB so we gave ourselves
// an additional timeout period
// in case it was hogging the
// bus.
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
    pub tqe: TAILQ_ENTRY(scb),
    pub links: },
    pub pending_links: LIST_ENTRY(scb),
    pub io_ctx: ahc_io_ctx_t,
    pub ahc_softc: *mut ahc_softc,
    pub flags: scb_flag,
    pub platform_data: *mut scb_platform_data,
    pub sg_map: *mut sg_map_node,
    pub sg_list: *mut ahc_dma_seg,
    pub sg_list_phys: dma_addr_t,
    pub /: *mut *mut u_int sg_count;/ How full ahc_dma_seg is,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_data {
    pub /*: *mut SLIST_HEAD(, scb) free_scbs;,
// Pool of SCBs ready to be assigned
// commands to execute.
//
    pub /*: *mut *mut scb scbindex[256];,
// Mapping from tag to SCB.
// As tag identifiers are an
// 8bit value, we provide space
// for all possible tag values.
// Any lookups to entries at or
// above AHC_SCB_MAX_ALLOC will
// always fail.
//
    pub /: *mut *mut *mut hardware_scb hscbs; / Array of hardware SCBs,
    pub /: *mut *mut *mut scb scbarray; / Array of kernel SCBs,
    pub /: *mut *mut *mut scsi_sense_data sense; / Per SCB sense data,
//
// "Bus" addresses of our data structures.
//
    pub /: *mut *mut bus_dma_tag_t hscb_dmat; / dmat for our hardware SCB array,
    pub hscb_dmamap: bus_dmamap_t,
    pub hscb_busaddr: dma_addr_t,
    pub sense_dmat: bus_dma_tag_t,
    pub sense_dmamap: bus_dmamap_t,
    pub sense_busaddr: dma_addr_t,
    pub /: *mut *mut bus_dma_tag_t sg_dmat; / dmat for our sg segments,
    pub sg_maps: SLIST_HEAD(, sg_map_node),
    pub numscbs: u8,
    pub /: *mut *mut uint8_t maxhscbs; / Number of SCBs on the card,
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
pub const AHC_TMODE_EVENT_BUFFER_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_tmode_event {
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
pub struct ahc_tmode_lstate {
    pub path: *mut cam_path,
    pub accept_tios: ccb_hdr_slist,
    pub immed_notifies: ccb_hdr_slist,
    pub event_buffer: [ahc_tmode_event; AHC_TMODE_EVENT_BUFFER_SIZE],
    pub event_r_idx: u8,
    pub event_w_idx: u8,
}

// Transfer Negotiation Datastructures
pub const AHC_TRANS_CUR: c_uint = 0x01	/* Modify current neogtiation status */;
pub const AHC_TRANS_ACTIVE: c_uint = 0x03	/* Assume this target is on the bus */;
pub const AHC_TRANS_GOAL: c_uint = 0x04	/* Modify negotiation goal */;
pub const AHC_TRANS_USER: c_uint = 0x08	/* Modify user negotiation settings */;
pub const AHC_WIDTH_UNKNOWN: c_uint = 0xFF;
pub const AHC_PERIOD_UNKNOWN: c_uint = 0xFF;
pub const AHC_OFFSET_UNKNOWN: c_uint = 0xFF;
pub const AHC_PPR_OPTS_UNKNOWN: c_uint = 0xFF;
//
// Transfer Negotiation Information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_transinfo {
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
pub struct ahc_initiator_tinfo {
    pub /: *mut *mut uint8_t scsirate; / Computed value for SCSIRATE reg,
    pub curr: ahc_transinfo,
    pub goal: ahc_transinfo,
    pub user: ahc_transinfo,
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
pub struct ahc_tmode_tstate {
    pub enabled_luns: [*mut *mut ahc_tmode_lstate; AHC_NUM_LUNS],
    pub transinfo: [ahc_initiator_tinfo; AHC_NUM_TARGETS],
//
// Per initiator state bitmasks.
//
    pub /: *mut *mut uint16_t auto_negotiate;/ Auto Negotiation Required,
    pub /: *mut *mut uint16_t ultraenb; / Using ultra sync rate,
    pub /: *mut *mut uint16_t discenable; / Disconnection allowed,
    pub /: *mut *mut uint16_t tagenable; / Tagged Queuing allowed,
}

//
// Data structure for our table of allowed synchronous transfer rates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_syncrate {
    pub /: *mut *mut u_int sxfr_u2; / Value of the SXFR parameter for Ultra2+ Chips,
    pub /: *mut *mut u_int sxfr; / Value of the SXFR parameter for <= Ultra Chips,
pub const ULTRA_SXFR: c_uint = 0x100	/* Rate Requires Ultra Mode set */;
pub const ST_SXFR: c_uint = 0x010	/* Rate Single Transition Only */;
pub const DT_SXFR: c_uint = 0x040	/* Rate Double Transition Only */;
    pub /: *mut *mut uint8_t period; / Period to send to SCSI target,
    pub rate: *const c_char,
}

// Safe and valid period for async negotiations.
pub const AHC_ASYNC_XFER_PERIOD: c_uint = 0x45;
pub const AHC_ULTRA2_XFER_PERIOD: c_uint = 0x0a;
//
// Indexes into our table of syncronous transfer rates.
//
pub const AHC_SYNCRATE_DT: c_int = 0;
pub const AHC_SYNCRATE_ULTRA2: c_int = 1;
pub const AHC_SYNCRATE_ULTRA: c_int = 3;
pub const AHC_SYNCRATE_FAST: c_int = 6;

pub const AHC_SYNCRATE_MIN: c_int = 13;
// Lookup Tables
//
// Phase -> name and message out response
// to parity errors in each phase table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_phase_table_entry {
    pub phase: u8,
    pub /: *mut *mut uint8_t mesg_out; / Message response to parity errors,
    pub phasemsg: *mut c_char,
}

// Serial EEPROM Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seeprom_config {
//
// Per SCSI ID Configuration Flags
//
    pub /: *mut *mut uint16_t device_flags[16]; / words 0-15,
pub const CFXFER: c_uint = 0x0007	/* synchronous transfer rate */;
pub const CFSYNCH: c_uint = 0x0008	/* enable synchronous transfer */;
pub const CFDISC: c_uint = 0x0010	/* enable disconnection */;
pub const CFWIDEB: c_uint = 0x0020	/* wide bus device */;
pub const CFSYNCHISULTRA: c_uint = 0x0040	/* CFSYNCH is an ultra offset (2940AU)*/;
pub const CFSYNCSINGLE: c_uint = 0x0080	/* Single-Transition signalling */;
pub const CFSTART: c_uint = 0x0100	/* send start unit SCSI command */;
pub const CFINCBIOS: c_uint = 0x0200	/* include in BIOS scan */;
pub const CFRNFOUND: c_uint = 0x0400	/* report even if not found */;
pub const CFMULTILUNDEV: c_uint = 0x0800	/* Probe multiple luns in BIOS scan */;
pub const CFWBCACHEENB: c_uint = 0x4000	/* Enable W-Behind Cache on disks */;
pub const CFWBCACHENOP: c_uint = 0xc000	/* Don't touch W-Behind Cache */;
//
// BIOS Control Bits
//
    pub /: *mut *mut uint16_t bios_control; / word 16,
pub const CFSUPREM: c_uint = 0x0001	/* support all removeable drives */;
pub const CFSUPREMB: c_uint = 0x0002	/* support removeable boot drives */;
pub const CFBIOSEN: c_uint = 0x0004	/* BIOS enabled */;
pub const CFBIOS_BUSSCAN: c_uint = 0x0008	/* Have the BIOS Scan the Bus */;
pub const CFSM2DRV: c_uint = 0x0010	/* support more than two drives */;
pub const CFSTPWLEVEL: c_uint = 0x0010	/* Termination level control */;
pub const CF284XEXTEND: c_uint = 0x0020	/* extended translation (284x cards) */;
pub const CFCTRL_A: c_uint = 0x0020	/* BIOS displays Ctrl-A message */;
pub const CFTERM_MENU: c_uint = 0x0040	/* BIOS displays termination menu */;
pub const CFEXTEND: c_uint = 0x0080	/* extended translation enabled */;
pub const CFSCAMEN: c_uint = 0x0100	/* SCAM enable */;
pub const CFMSG_LEVEL: c_uint = 0x0600	/* BIOS Message Level */;
pub const CFMSG_VERBOSE: c_uint = 0x0000;
pub const CFMSG_SILENT: c_uint = 0x0200;
pub const CFMSG_DIAG: c_uint = 0x0400;
pub const CFBOOTCD: c_uint = 0x0800  /* Support Bootable CD-ROM */;
// UNUSED		0xff00
//
// Host Adapter Control Bits
//
    pub /: *mut *mut uint16_t adapter_control; / word 17,
pub const CFAUTOTERM: c_uint = 0x0001	/* Perform Auto termination */;
pub const CFULTRAEN: c_uint = 0x0002	/* Ultra SCSI speed enable */;
pub const CF284XSELTO: c_uint = 0x0003	/* Selection timeout (284x cards) */;
pub const CF284XFIFO: c_uint = 0x000C	/* FIFO Threshold (284x cards) */;
pub const CFSTERM: c_uint = 0x0004	/* SCSI low byte termination */;
pub const CFWSTERM: c_uint = 0x0008	/* SCSI high byte termination */;
pub const CFSPARITY: c_uint = 0x0010	/* SCSI parity */;
pub const CF284XSTERM: c_uint = 0x0020	/* SCSI low byte term (284x cards) */;
pub const CFMULTILUN: c_uint = 0x0020;
pub const CFRESETB: c_uint = 0x0040	/* reset SCSI bus at boot */;
pub const CFCLUSTERENB: c_uint = 0x0080	/* Cluster Enable */;
pub const CFBOOTCHAN: c_uint = 0x0300	/* probe this channel first */;
pub const CFBOOTCHANSHIFT: c_int = 8;
pub const CFSEAUTOTERM: c_uint = 0x0400	/* Ultra2 Perform secondary Auto Term*/;
pub const CFSELOWTERM: c_uint = 0x0800	/* Ultra2 secondary low term */;
pub const CFSEHIGHTERM: c_uint = 0x1000	/* Ultra2 secondary high term */;
pub const CFENABLEDV: c_uint = 0x4000	/* Perform Domain Validation*/;
//
// Bus Release Time, Host Adapter ID
//
    pub /: *mut *mut uint16_t brtime_id; / word 18,
pub const CFSCSIID: c_uint = 0x000f	/* host adapter SCSI ID */;
// UNUSED		0x00f0
pub const CFBRTIME: c_uint = 0xff00	/* bus release time */;
//
// Maximum targets
//
    pub /: *mut *mut uint16_t max_targets; / word 19,
pub const CFMAXTARG: c_uint = 0x00ff	/* maximum targets */;
pub const CFBOOTLUN: c_uint = 0x0f00	/* Lun to boot from */;
pub const CFBOOTID: c_uint = 0xf000	/* Target to boot from */;
    pub /: *mut *mut uint16_t res_1[10]; / words 20-29,
    pub /: *mut *mut uint16_t signature; / Signature == 0x250,
pub const CFSIGNATURE: c_uint = 0x250;
pub const CFSIGNATURE2: c_uint = 0x300;
    pub /: *mut *mut uint16_t checksum; / word 31,
}

// Message Buffer
// Software Configuration Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_aic7770_softc {
//
// Saved register state used for chip_init().
//
    pub busspd: u8,
    pub bustime: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_pci_softc {
//
// Saved register state used for chip_init().
//
    pub devconfig: u32,
    pub targcrccnt: u16,
    pub command: u8,
    pub csize_lattime: u8,
    pub optionmode: u8,
    pub crccontrol1: u8,
    pub dscommand0: u8,
    pub dspcistatus: u8,
    pub scbbaddr: u8,
    pub dff_thrsh: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ahc_bus_softc {
    pub aic7770_softc: ahc_aic7770_softc,
    pub pci_softc: ahc_pci_softc,
}

extern "C" {
    pub fn void(: *mut *mut ahc_bus_intr_t)(struct ahc_softc) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut ahc_bus_chip_init_t)(struct ahc_softc) -> typedef;
}
extern "C" {
    pub fn ahc_callback_t(: *mut c_void) -> typedef void;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_softc {
    pub tag: bus_space_tag_t,
    pub bsh: bus_space_handle_t,
    pub scb_data: *mut scb_data,
    pub next_queued_scb: *mut scb,
//
// SCBs that have been sent to the controller
//
    pub pending_scbs: BSD_LIST_HEAD(, scb),
//
// Counting lock for deferring the release of additional
// untagged transactions from the untagged_queues.  When
// the lock is decremented to 0, all queues in the
// untagged_queues array are run.
//
    pub untagged_queue_lock: u_int,
//
// Per-target queue of untagged-transactions.  The
// transaction at the head of the queue is the
// currently pending untagged transaction for the
// target.  The driver only allows a single untagged
// transaction per target.
//
    pub untagged_queues: [scb_tailq; AHC_NUM_TARGETS],
//
// Bus attachment specific data.
//
    pub bus_softc: ahc_bus_softc,
//
// Platform specific data.
//
    pub platform_data: *mut ahc_platform_data,
//
// Platform specific device information.
//
    pub dev_softc: ahc_dev_softc_t,
    pub dev: *mut device,
//
// Bus specific device information.
//
    pub bus_intr: ahc_bus_intr_t,
//
// Bus specific initialization required
// after a chip reset.
//
    pub bus_chip_init: ahc_bus_chip_init_t,
//
// Target mode related state kept on a per enabled lun basis.
// Targets that are not enabled will have null entries.
// As an initiator, we keep one target entry for our initiator
// ID to store our sync/wide transfer settings.
//
    pub enabled_targets: [*mut ahc_tmode_tstate; AHC_NUM_TARGETS],
//
// The black hole device responsible for handling requests for
// disabled luns on enabled targets.
//
    pub black_hole: *mut ahc_tmode_lstate,
//
// Device instance currently on the bus awaiting a continue TIO
// for a command that was not given the disconnect priveledge.
//
    pub pending_device: *mut ahc_tmode_lstate,
//
// Card characteristics
//
    pub chip: ahc_chip,
    pub features: ahc_feature,
    pub bugs: ahc_bug,
    pub flags: ahc_flag,
    pub seep_config: *mut seeprom_config,
// Values to store in the SEQCTL register for pause and unpause
    pub unpause: u8,
    pub pause: u8,
// Command Queues
    pub qoutfifonext: u8,
    pub qinfifonext: u8,
    pub qoutfifo: *mut u8,
    pub qinfifo: *mut u8,
// Critical Section Data
    pub critical_sections: *mut cs,
    pub num_critical_sections: u_int,
// Channel Names ('A', 'B', etc.)
    pub channel: c_char,
    pub channel_b: c_char,
// Initiator Bus ID
    pub our_id: u8,
    pub our_id_b: u8,
//
// PCI error detection.
//
    pub unsolicited_ints: c_int,
//
// Target incoming command FIFO.
//
    pub targetcmds: *mut target_cmd,
    pub tqinfifonext: u8,
//
// Cached copy of the sequencer control register.
//
    pub seqctl: u8,
//
// Incoming and outgoing message handling.
//
    pub send_msg_perror: u8,
    pub msg_type: ahc_msg_type,
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
    pub shared_data_dmamap: bus_dmamap_t,
    pub shared_data_busaddr: dma_addr_t,
//
// Bus address of the one byte buffer used to
// work-around a DMA bug for chips <= aic7880
// in target mode.
//
    pub dma_bug_buf: dma_addr_t,
// Number of enabled target mode device on this card
    pub enabled_luns: u_int,
// Initialization level of this data structure
    pub init_level: u_int,
// PCI cacheline size.
    pub pci_cachesize: u_int,
//
// Count of parity errors we have seen as a target.
// We auto-disable parity error checking after seeing
// AHC_PCI_TARGET_PERR_THRESH number of errors.
//
    pub pci_target_perr_count: u_int,
pub const AHC_PCI_TARGET_PERR_THRESH: c_int = 10;
// Maximum number of sequencer instructions supported.
    pub instruction_ram_size: u_int,
// Per-Unit descriptive information
    pub description: *const c_char,
    pub name: *mut c_char,
    pub unit: c_int,
// Selection Timer settings
    pub seltime: c_int,
    pub seltime_b: c_int,
    pub /: *mut *mut uint16_t user_discenable;/ Disconnection allowed,
    pub /: *mut *mut uint16_t user_tagenable;/ Tagged Queuing allowed,
}

// Active Device Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_devinfo {
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
    pub fn int(: *mut ahc_device_setup_t)(struct ahc_softc) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_pci_identity {
    pub full_id: u64,
    pub id_mask: u64,
    pub name: *const c_char,
    pub setup: *mut ahc_device_setup_t,
}

// VL/EISA Declarations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aic7770_identity {
    pub full_id: u32,
    pub id_mask: u32,
    pub name: *const c_char,
    pub setup: *mut ahc_device_setup_t,
}

pub const AHC_EISA_SLOT_OFFSET: c_uint = 0xc00;
pub const AHC_EISA_IOSIZE: c_uint = 0x100;
// Function Declarations
//
// PCI Front End
extern "C" {
    pub fn ahc_pci_test_register_access(: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_pci_resume(ahc: *mut ahc_softc) -> void __maybe_unused;
}
// EISA/VL Front End
// SCB and SCB queue management
extern "C" {
    pub fn ahc_probe_scbs(: *mut ahc_softc) -> c_int;
}
// Initialization
extern "C" {
    pub fn ahc_softc_init(: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_controller_info(ahc: *mut ahc_softc, buf: *mut c_char);
}
extern "C" {
    pub fn ahc_chip_init(ahc: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_init(ahc: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_intr_enable(ahc: *mut ahc_softc, enable: c_int);
}
extern "C" {
    pub fn ahc_pause_and_flushwork(ahc: *mut ahc_softc);
}
extern "C" {
    pub fn ahc_suspend(ahc: *mut ahc_softc) -> int __maybe_unused;
}
extern "C" {
    pub fn ahc_resume(ahc: *mut ahc_softc) -> int __maybe_unused;
}
extern "C" {
    pub fn ahc_set_unit(: *mut ahc_softc, _arg: c_int);
}
extern "C" {
    pub fn ahc_set_name(: *mut ahc_softc, : *mut c_char);
}
extern "C" {
    pub fn ahc_free(ahc: *mut ahc_softc);
}
extern "C" {
    pub fn ahc_reset(ahc: *mut ahc_softc, reinit: c_int) -> c_int;
}
// Error Recovery
// Utility Functions
// Transfer Negotiation
//
// Negotiation types.  These are used to qualify if we should renegotiate
// even if our goal and current transport parameters are identical.
//
// Target Mode

pub const AHC_TMODE_ENABLE: c_int = 0;

// Debug

pub const AHC_SHOW_MISC: c_uint = 0x0001;
pub const AHC_SHOW_SENSE: c_uint = 0x0002;
pub const AHC_DUMP_SEEPROM: c_uint = 0x0004;
pub const AHC_SHOW_TERMCTL: c_uint = 0x0008;
pub const AHC_SHOW_MEMORY: c_uint = 0x0010;
pub const AHC_SHOW_MESSAGES: c_uint = 0x0020;
pub const AHC_SHOW_DV: c_uint = 0x0040;
pub const AHC_SHOW_SELTO: c_uint = 0x0080;
pub const AHC_SHOW_QFULL: c_uint = 0x0200;
pub const AHC_SHOW_QUEUE: c_uint = 0x0400;
pub const AHC_SHOW_TQIN: c_uint = 0x0800;
pub const AHC_SHOW_MASKED_ERRORS: c_uint = 0x1000;
pub const AHC_DEBUG_SEQUENCER: c_uint = 0x2000;

extern "C" {
    pub fn ahc_dump_card_state(ahc: *mut ahc_softc);
}
// SEEPROM
extern "C" {
    pub fn ahc_release_seeprom(sd: *mut seeprom_descriptor);
}
