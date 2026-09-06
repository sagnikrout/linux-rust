//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arm/fas216.h
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
// linux/drivers/acorn/scsi/fas216.h
//
// Copyright (C) 1997-2000 Russell King
//
// FAS216 generic driver
//

// FAS register definitions
// transfer count low

// transfer count medium

// fifo data

// command

pub const CMD_NOP: c_uint = 0x00;
pub const CMD_FLUSHFIFO: c_uint = 0x01;
pub const CMD_RESETCHIP: c_uint = 0x02;
pub const CMD_RESETSCSI: c_uint = 0x03;
pub const CMD_TRANSFERINFO: c_uint = 0x10;
pub const CMD_INITCMDCOMPLETE: c_uint = 0x11;
pub const CMD_MSGACCEPTED: c_uint = 0x12;
pub const CMD_PADBYTES: c_uint = 0x18;
pub const CMD_SETATN: c_uint = 0x1a;
pub const CMD_RSETATN: c_uint = 0x1b;
pub const CMD_SELECTWOATN: c_uint = 0x41;
pub const CMD_SELECTATN: c_uint = 0x42;
pub const CMD_SELECTATNSTOP: c_uint = 0x43;
pub const CMD_ENABLESEL: c_uint = 0x44;
pub const CMD_DISABLESEL: c_uint = 0x45;
pub const CMD_SELECTATN3: c_uint = 0x46;
pub const CMD_RESEL3: c_uint = 0x47;
pub const CMD_WITHDMA: c_uint = 0x80;
// status register (read)

// bus ID for select / reselect

// Interrupt status register (read)

// Timeout register (write)

// Sequence step register (read)

pub const IS_BITS: c_uint = 0x07;
pub const IS_SELARB: c_uint = 0x00				/* Select & Arb ok	*/;
pub const IS_MSGBYTESENT: c_uint = 0x01				/* One byte message sent*/;
pub const IS_NOTCOMMAND: c_uint = 0x02				/* Not in command state	*/;
pub const IS_EARLYPHASE: c_uint = 0x03				/* Early phase change	*/;
pub const IS_COMPLETE: c_uint = 0x04				/* Command ok		*/;
pub const IS_SOF: c_uint = 0x08				/* Sync off flag	*/;
// Transfer period step (write)

// Synchronous Offset (write)

// Fifo state register (read)

pub const CFIS_CF: c_uint = 0x1f				/* Num bytes in FIFO	*/;
pub const CFIS_IS: c_uint = 0xe0				/* Step			*/;
// config register 1

// Clock conversion factor (read)

pub const CLKF_F37MHZ: c_uint = 0x00				/* 35.01 - 40 MHz		*/;
pub const CLKF_F10MHZ: c_uint = 0x02				/* 10 MHz			*/;
pub const CLKF_F12MHZ: c_uint = 0x03				/* 10.01 - 15 MHz		*/;
pub const CLKF_F17MHZ: c_uint = 0x04				/* 15.01 - 20 MHz		*/;
pub const CLKF_F22MHZ: c_uint = 0x05				/* 20.01 - 25 MHz		*/;
pub const CLKF_F27MHZ: c_uint = 0x06				/* 25.01 - 30 MHz		*/;
pub const CLKF_F32MHZ: c_uint = 0x07				/* 30.01 - 35 MHz		*/;
// Chip test register (write)

pub const TEST_FTM: c_uint = 0x01				/* Force target mode		*/;
pub const TEST_FIM: c_uint = 0x02				/* Force initiator mode		*/;
pub const TEST_FHI: c_uint = 0x04				/* Force high impedance mode	*/;
// Configuration register 2 (read/write)

// Configuration register 3 (read/write)

// High transfer count (read/write)

// ID register (read only)

// Data alignment

pub const MAGIC: c_uint = 0x441296bdUL;
pub const NR_MSGS: c_int = 8;

//
// Error recovery
//
// driver information
// statistics information
// configuration information
// queue handling
// per-device info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fas216_device {
    pub /: *mut *mut unsigned char disconnect_ok:1; / device can disconnect,
    pub /: *mut *mut unsigned char parity_enabled:1; / parity checking enabled,
    pub /: *mut *mut unsigned char parity_check:1; / need to check parity checking,
    pub /: *mut *mut *mut unsigned char period; / sync xfer period in (4ns),
    pub /: *mut *mut unsigned char stp; / synchronous transfer period,
    pub /: *mut *mut unsigned char sof; / synchronous offset register,
    pub /: *mut *mut unsigned char wide_xfer; / currently negociated wide transfer,
    pub /: *mut *mut neg_t sync_state; / synchronous transfer mode,
    pub /: *mut *mut neg_t wide_state; / wide transfer mode,
    pub device: [}; 8],
    pub /: *mut *mut unsigned long busyluns[64/sizeof(unsigned long)];/ array of bits indicating LUNs busy,
// dma
    pub /: *mut *mut fasdmatype_t transfer_type; / current type of DMA transfer,
    pub min_dma): *mut *mut *mut *mut fasdmatype_t (setup) (struct Scsi_Host host, struct scsi_pointer SCp, fasdmadir_t direction, fasdmatype_t,
    pub transfer): *mut *mut *mut *mut void (pseudo)(struct Scsi_Host host, struct scsi_pointer SCp, fasdmadir_t direction, int,
    pub SCp): *mut *mut *mut void (stop) (struct Scsi_Host host, struct scsi_pointer,
    pub dma: },
// miscellaneous
    pub /: *mut *mut int internal_done; / flag to indicate request done,
    pub /: *mut *mut scsi_eh_save ses; / holds request sense restore info,
    pub magic_end: c_ulong,
    pub FAS216_Info: },
// driver-private data per SCSI command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fas216_cmd_priv {
//
// @scsi_pointer must be the first member. See also arm_scsi_pointer().
//
    pub scsi_pointer: scsi_pointer,
    pub cmd): *mut *mut void (scsi_done)(struct scsi_cmnd,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
// Function: int fas216_init (struct Scsi_Host *instance)
// Purpose : initialise FAS/NCR/AMD SCSI structures.
// Params  : instance - a driver-specific filled-out structure
// Returns : 0 on success
//
extern "C" {
    pub fn fas216_init(instance: *mut Scsi_Host) -> c_int;
}
// Function: int fas216_add (struct Scsi_Host *instance, struct device *dev)
// Purpose : initialise FAS/NCR/AMD SCSI ic.
// Params  : instance - a driver-specific filled-out structure
// Returns : 0 on success
//
extern "C" {
    pub fn fas216_add(instance: *mut Scsi_Host, dev: *mut device) -> c_int;
}
// Function: enum scsi_qc_status fas216_queue_command(struct Scsi_Host *h, struct scsi_cmnd *SCpnt)
// Purpose : queue a command for adapter to process.
// Params  : h - host adapter
// : SCpnt - Command to queue
// Returns : 0 - success, else error
//
// Function: enum scsi_qc_status fas216_noqueue_command(struct Scsi_Host *h,
// struct scsi_cmnd *SCpnt)
// Purpose : queue a command for adapter to process, and process it to completion.
// Params  : h - host adapter
// : SCpnt - Command to queue
// Returns : 0 - success, else error
//
// Function: irqreturn_t fas216_intr (FAS216_Info *info)
// Purpose : handle interrupts from the interface to progress a command
// Params  : info - interface to service
//
extern "C" {
    pub fn fas216_intr(info: *mut FAS216_Info) -> irqreturn_t;
}
extern "C" {
    pub fn fas216_remove(instance: *mut Scsi_Host);
}
// Function: void fas216_release (struct Scsi_Host *instance)
// Purpose : release all resources and put everything to bed for FAS/NCR/AMD SCSI ic.
// Params  : instance - a driver-specific filled-out structure
// Returns : 0 on success
//
extern "C" {
    pub fn fas216_release(instance: *mut Scsi_Host);
}
extern "C" {
    pub fn fas216_print_host(info: *mut FAS216_Info, m: *mut seq_file);
}
extern "C" {
    pub fn fas216_print_stats(info: *mut FAS216_Info, m: *mut seq_file);
}
extern "C" {
    pub fn fas216_print_devices(info: *mut FAS216_Info, m: *mut seq_file);
}
// Function: int fas216_eh_abort(struct scsi_cmnd *SCpnt)
// Purpose : abort this command
// Params  : SCpnt - command to abort
// Returns : FAILED if unable to abort
//
extern "C" {
    pub fn fas216_eh_abort(SCpnt: *mut scsi_cmnd) -> c_int;
}
// Function: int fas216_eh_device_reset(struct scsi_cmnd *SCpnt)
// Purpose : Reset the device associated with this command
// Params  : SCpnt - command specifing device to reset
// Returns : FAILED if unable to reset
//
extern "C" {
    pub fn fas216_eh_device_reset(SCpnt: *mut scsi_cmnd) -> c_int;
}
// Function: int fas216_eh_bus_reset(struct scsi_cmnd *SCpnt)
// Purpose : Reset the complete bus associated with this command
// Params  : SCpnt - command specifing bus to reset
// Returns : FAILED if unable to reset
//
extern "C" {
    pub fn fas216_eh_bus_reset(SCpnt: *mut scsi_cmnd) -> c_int;
}
// Function: int fas216_eh_host_reset(struct scsi_cmnd *SCpnt)
// Purpose : Reset the host associated with this command
// Params  : SCpnt - command specifing host to reset
// Returns : FAILED if unable to reset
//
extern "C" {
    pub fn fas216_eh_host_reset(SCpnt: *mut scsi_cmnd) -> c_int;
}
