//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/53c700.h
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
// Driver for 53c700 and 53c700-66 chips from NCR and Symbios
//
// Copyright (C) 2001 by James.Bottomley@HansenPartnership.com
//

// Turn on for general debugging---too verbose for normal use

// Debug the tag queues, checking hash queue allocation and deallocation
// and search for duplicate tags

// The number of available command slots
pub const NCR_700_COMMAND_SLOTS_PER_HOST: c_int = 64;
// The maximum number of Scatter Gathers we allow
pub const NCR_700_SG_SEGMENTS: c_int = 32;
// The maximum number of luns (make this of the form 2^n)
pub const NCR_700_MAX_LUNS: c_int = 32;

// Maximum number of tags the driver ever allows per device
pub const NCR_700_MAX_TAGS: c_int = 16;
// Tag depth the driver starts out with (can be altered in sysfs)
pub const NCR_700_DEFAULT_TAGS: c_int = 4;
// This is the default number of commands per LUN in the untagged case.
// two is a good value because it means we can have one command active and
// one command fully prepared and waiting
//
pub const NCR_700_CMD_PER_LUN: c_int = 2;
// magic byte identifying an internally generated REQUEST_SENSE command
pub const NCR_700_INTERNAL_SENSE_MAGIC: c_uint = 0x42;
// These are the externally used routines
extern "C" {
    pub fn NCR_700_release(host: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn NCR_700_intr(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NCR_700_Host_State {
    NCR_700_HOST_BUSY,
    NCR_700_HOST_FREE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR_700_SG_List {
// The following is a script fragment to move the buffer onto the
// bus and then link the next fragment or return
pub const SCRIPT_MOVE_DATA_IN: c_uint = 0x09000000;
pub const SCRIPT_MOVE_DATA_OUT: c_uint = 0x08000000;
    pub ins: __u32,
    pub pAddr: __u32,
pub const SCRIPT_NOP: c_uint = 0x80000000;
pub const SCRIPT_RETURN: c_uint = 0x90080000;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR_700_Device_Parameters {
// space for creating a request sense command. Really, except
// for the annoying SCSI-2 requirement for LUN information in
// cmnd[1], this could be in static storage
    pub cmnd: [c_uchar; MAX_COMMAND_SIZE],
    pub depth: __u8,
    pub /: *mut *mut *mut scsi_cmnd current_cmnd; / currently active command,
}

// The SYNC negotiation sequence looks like:
//
// If DEV_NEGOTIATED_SYNC not set, tack and SDTR message on to the
// initial identify for the device and set DEV_BEGIN_SYNC_NEGOTIATION
// If we get an SDTR reply, work out the SXFER parameters, squirrel
// them away here, clear DEV_BEGIN_SYNC_NEGOTIATION and set
// DEV_NEGOTIATED_SYNC.  If we get a REJECT msg, squirrel
//
// 0:7	SXFER_REG negotiated value for this device
// 8:15 Current queue depth
// 16	negotiated SYNC flag
// 17 begin SYNC negotiation flag
// 18 device supports tag queueing

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NCR_700_tag_neg_state {
    NCR_700_START_TAG_NEGOTIATION = 0,
    NCR_700_DURING_TAG_NEGOTIATION = 1,
    NCR_700_FINISHED_TAG_NEGOTIATION = 2,
}

// clear the slot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR_700_command_slot {
    pub SG: [NCR_700_SG_List; NCR_700_SG_SEGMENTS+1],
    pub pSG: *mut NCR_700_SG_List,
pub const NCR_700_SLOT_MASK: c_uint = 0xFC;
pub const NCR_700_SLOT_MAGIC: c_uint = 0xb8;

    pub state: __u8,
pub const NCR_700_FLAG_AUTOSENSE: c_uint = 0x01;
    pub flags: __u8,
    pub /: *mut *mut __u8 pad1[2]; / Needed for m68k where min alignment is 2 bytes,
    pub tag: c_int,
    pub resume_offset: __u32,
    pub cmnd: *mut scsi_cmnd,
// The pci_mapped address of the actual command in cmnd
    pub pCmd: dma_addr_t,
    pub temp: __u32,
// if this command is a pci_single mapping, holds the dma address
// for later unmapping in the done routine
    pub dma_handle: dma_addr_t,
// historical remnant, now used to link free commands
    pub ITL_forw: *mut NCR_700_command_slot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR_700_Host_Parameters {
// These must be filled in by the calling driver
    pub /: *mut *mut int clock; / board clock speed in MHz,
    pub /: *mut *mut *mut void __iomem base; / the base for the port (copied to host),
    pub dev: *mut device,
    pub /: *mut *mut __u32 dmode_extra; / adjustable bus settings,
    pub /: *mut *mut __u32 dcntl_extra; / adjustable bus settings,
    pub /: *mut *mut __u32 ctest7_extra; / adjustable bus settings,
    pub /: *mut *mut __u32 differential:1; / if we are differential,

// This option is for HP only.  Set it if your chip is wired for
// little endian on this platform (which is big endian)
    pub force_le_on_be:1: __u32,

    pub /: *mut *mut __u32 chip710:1; / set if really a 710 not 700,
    pub /: *mut *mut __u32 burst_length:4; / set to 0 to disable 710 bursting,
    pub /: *mut *mut __u32 noncoherent:1; / needs to use non-coherent DMA,
// NOTHING BELOW HERE NEEDS ALTERING
    pub clock: *mut *mut __u32 fast:1; / if we can alter the SCSI bus,
    pub /: *mut *mut int sync_clock; / The speed of the SYNC core,
    pub /: *mut *mut *mut __u32 script; / pointer to script location,
    pub /: *mut *mut __u32 pScript; / physical mem addr of script,
    pub /: *mut *mut NCR_700_Host_State state; / protected by state lock,
    pub cmd: *mut scsi_cmnd,
// Note: pScript contains the single consistent block of
// memory.  All the msgin, msgout and status are allocated in
// this memory too (at separate cache lines).  TOTAL_MEM_SIZE
// represents the total size of this area
pub const MSG_ARRAY_SIZE: c_int = 8;

    pub msgout: *mut __u8,

    pub msgin: *mut __u8,

    pub status: *mut __u8,

    pub slots: *mut NCR_700_command_slot,

    pub saved_slot_position: c_int,
    pub /: *mut *mut int command_slot_count; / protected by state lock,
    pub tag_negotiated: __u8,
    pub rev: __u8,
    pub reselection_id: __u8,
    pub min_period: __u8,
// Free list, singly linked by ITL_forw elements
    pub free_list: *mut NCR_700_command_slot,
// Completion for waited for ops, like reset, abort or
// device reset.
//
// NOTE: relies on single threading in the error handler to
// have only one outstanding at once
    pub eh_complete: *mut completion,
}

//
// 53C700 Register Interface - the offset from the Selected base
// I/O address

pub const bE: c_int = 3;
pub const bSWAP: c_int = 0;

pub const bE: c_int = 0;
pub const bSWAP: c_int = 0;

pub const bEBus: c_int = 1;

pub const bEBus: c_int = 0;

// NOTE: These registers are in the LE register space only, the required byte
// swapping is done by the NCR_700_{read|write}[b] functions
pub const SCNTL0_REG: c_uint = 0x00;
pub const FULL_ARBITRATION: c_uint = 0xc0;
pub const PARITY: c_uint = 0x08;
pub const ENABLE_PARITY: c_uint = 0x04;
pub const AUTO_ATN: c_uint = 0x02;
pub const SCNTL1_REG: c_uint = 0x01;
pub const SLOW_BUS: c_uint = 0x80;
pub const ENABLE_SELECT: c_uint = 0x20;
pub const ASSERT_RST: c_uint = 0x08;
pub const ASSERT_EVEN_PARITY: c_uint = 0x04;
pub const SDID_REG: c_uint = 0x02;
pub const SIEN_REG: c_uint = 0x03;
pub const PHASE_MM_INT: c_uint = 0x80;
pub const FUNC_COMP_INT: c_uint = 0x40;
pub const SEL_TIMEOUT_INT: c_uint = 0x20;
pub const SELECT_INT: c_uint = 0x10;
pub const GROSS_ERR_INT: c_uint = 0x08;
pub const UX_DISC_INT: c_uint = 0x04;
pub const RST_INT: c_uint = 0x02;
pub const PAR_ERR_INT: c_uint = 0x01;
pub const SCID_REG: c_uint = 0x04;
pub const SXFER_REG: c_uint = 0x05;
pub const ASYNC_OPERATION: c_uint = 0x00;
pub const SODL_REG: c_uint = 0x06;
pub const SOCL_REG: c_uint = 0x07;
pub const SFBR_REG: c_uint = 0x08;
pub const SIDL_REG: c_uint = 0x09;
pub const SBDL_REG: c_uint = 0x0A;
pub const SBCL_REG: c_uint = 0x0B;
// read bits
pub const SBCL_IO: c_uint = 0x01;
// write bits
pub const SYNC_DIV_AS_ASYNC: c_uint = 0x00;
pub const SYNC_DIV_1_0: c_uint = 0x01;
pub const SYNC_DIV_1_5: c_uint = 0x02;
pub const SYNC_DIV_2_0: c_uint = 0x03;
pub const DSTAT_REG: c_uint = 0x0C;
pub const ILGL_INST_DETECTED: c_uint = 0x01;
pub const WATCH_DOG_INTERRUPT: c_uint = 0x02;
pub const SCRIPT_INT_RECEIVED: c_uint = 0x04;
pub const ABORTED: c_uint = 0x10;
pub const SSTAT0_REG: c_uint = 0x0D;
pub const PARITY_ERROR: c_uint = 0x01;
pub const SCSI_RESET_DETECTED: c_uint = 0x02;
pub const UNEXPECTED_DISCONNECT: c_uint = 0x04;
pub const SCSI_GROSS_ERROR: c_uint = 0x08;
pub const SELECTED: c_uint = 0x10;
pub const SELECTION_TIMEOUT: c_uint = 0x20;
pub const FUNCTION_COMPLETE: c_uint = 0x40;
pub const PHASE_MISMATCH: c_uint = 0x80;
pub const SSTAT1_REG: c_uint = 0x0E;
pub const SIDL_REG_FULL: c_uint = 0x80;
pub const SODR_REG_FULL: c_uint = 0x40;
pub const SODL_REG_FULL: c_uint = 0x20;
pub const SSTAT2_REG: c_uint = 0x0F;
pub const CTEST0_REG: c_uint = 0x14;
pub const BTB_TIMER_DISABLE: c_uint = 0x40;
pub const CTEST1_REG: c_uint = 0x15;
pub const CTEST2_REG: c_uint = 0x16;
pub const CTEST3_REG: c_uint = 0x17;
pub const CTEST4_REG: c_uint = 0x18;
pub const DISABLE_FIFO: c_uint = 0x00;
pub const SLBE: c_uint = 0x10;
pub const SFWR: c_uint = 0x08;
pub const BYTE_LANE0: c_uint = 0x04;
pub const BYTE_LANE1: c_uint = 0x05;
pub const BYTE_LANE2: c_uint = 0x06;
pub const BYTE_LANE3: c_uint = 0x07;
pub const SCSI_ZMODE: c_uint = 0x20;
pub const ZMODE: c_uint = 0x40;
pub const CTEST5_REG: c_uint = 0x19;
pub const MASTER_CONTROL: c_uint = 0x10;
pub const DMA_DIRECTION: c_uint = 0x08;
pub const CTEST7_REG: c_uint = 0x1B;
pub const BURST_DISABLE: c_uint = 0x80 /* 710 only */;
pub const SEL_TIMEOUT_DISABLE: c_uint = 0x10 /* 710 only */;
pub const DFP: c_uint = 0x08;
pub const EVP: c_uint = 0x04;
pub const CTEST7_TT1: c_uint = 0x02;
pub const DIFF: c_uint = 0x01;
pub const CTEST6_REG: c_uint = 0x1A;
pub const TEMP_REG: c_uint = 0x1C;
pub const DFIFO_REG: c_uint = 0x20;
pub const FLUSH_DMA_FIFO: c_uint = 0x80;
pub const CLR_FIFO: c_uint = 0x40;
pub const ISTAT_REG: c_uint = 0x21;
pub const ABORT_OPERATION: c_uint = 0x80;
pub const SOFTWARE_RESET_710: c_uint = 0x40;
pub const DMA_INT_PENDING: c_uint = 0x01;
pub const SCSI_INT_PENDING: c_uint = 0x02;
pub const CONNECTED: c_uint = 0x08;
pub const CTEST8_REG: c_uint = 0x22;
pub const LAST_DIS_ENBL: c_uint = 0x01;
pub const SHORTEN_FILTERING: c_uint = 0x04;
pub const ENABLE_ACTIVE_NEGATION: c_uint = 0x10;
pub const GENERATE_RECEIVE_PARITY: c_uint = 0x20;
pub const CLR_FIFO_710: c_uint = 0x04;
pub const FLUSH_DMA_FIFO_710: c_uint = 0x08;
pub const CTEST9_REG: c_uint = 0x23;
pub const DBC_REG: c_uint = 0x24;
pub const DCMD_REG: c_uint = 0x27;
pub const DNAD_REG: c_uint = 0x28;
pub const DIEN_REG: c_uint = 0x39;
pub const BUS_FAULT: c_uint = 0x20;
pub const ABORT_INT: c_uint = 0x10;
pub const INT_INST_INT: c_uint = 0x04;
pub const WD_INT: c_uint = 0x02;
pub const ILGL_INST_INT: c_uint = 0x01;
pub const DCNTL_REG: c_uint = 0x3B;
pub const SOFTWARE_RESET: c_uint = 0x01;
pub const COMPAT_700_MODE: c_uint = 0x01;
pub const SCRPTS_16BITS: c_uint = 0x20;
pub const EA_710: c_uint = 0x20;
pub const ASYNC_DIV_2_0: c_uint = 0x00;
pub const ASYNC_DIV_1_5: c_uint = 0x40;
pub const ASYNC_DIV_1_0: c_uint = 0x80;
pub const ASYNC_DIV_3_0: c_uint = 0xc0;
pub const DMODE_710_REG: c_uint = 0x38;
pub const DMODE_700_REG: c_uint = 0x34;
pub const BURST_LENGTH_1: c_uint = 0x00;
pub const BURST_LENGTH_2: c_uint = 0x40;
pub const BURST_LENGTH_4: c_uint = 0x80;
pub const BURST_LENGTH_8: c_uint = 0xC0;
pub const DMODE_FC1: c_uint = 0x10;
pub const DMODE_FC2: c_uint = 0x20;
pub const BW16: c_int = 32;
pub const MODE_286: c_int = 16;
pub const IO_XFER: c_int = 8;
pub const FIXED_ADDR: c_int = 4;
pub const DSP_REG: c_uint = 0x2C;
pub const DSPS_REG: c_uint = 0x30;
// Parameters to begin SDTR negotiations.  Empirically, I find that
// the 53c700-66 cannot handle an offset >8, so don't change this
pub const NCR_700_MAX_OFFSET: c_int = 8;
// Was hoping the max offset would be greater for the 710, but
// empirically it seems to be 8 also
pub const NCR_710_MAX_OFFSET: c_int = 8;
pub const NCR_700_MIN_XFERP: c_int = 1;
pub const NCR_710_MIN_XFERP: c_int = 0;

// Used for patching the SCSI ID in the SELECT instruction

extern "C" {
    pub fn ioread8((reg^bE): hostdata->base +) -> return;
}

// sanity check the register

// sanity check the register

