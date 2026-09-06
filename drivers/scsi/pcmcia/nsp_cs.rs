//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pcmcia/nsp_cs.h
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


// =======================================================

// Macro flag: #define  __nsp_cs__
// for debugging
// #define NSP_DEBUG 9
//
// Macro flag: #define static
// Macro flag: #define inline
//
// Some useful macros...
//
// SCSI initiator must be ID 7
pub const NSP_INITIATOR_ID: c_int = 7;
pub const NSP_SELTIMEOUT: c_int = 200;
//
// register definitions
//
// ========================================================================
// base register
pub const IRQCONTROL: c_uint = 0x00  /* R */;

pub const IRQSTATUS: c_uint = 0x00  /* W */;

pub const IFSELECT: c_uint = 0x01 /* W */;

pub const FIFOSTATUS: c_uint = 0x01 /* R */;

pub const INDEXREG: c_uint = 0x02 /* R/W */;
pub const DATAREG: c_uint = 0x03 /* R/W */;
pub const FIFODATA: c_uint = 0x04 /* R/W */;
pub const FIFODATA1: c_uint = 0x05 /* R/W */;
pub const FIFODATA2: c_uint = 0x06 /* R/W */;
pub const FIFODATA3: c_uint = 0x07 /* R/W */;
// ====================================================================
// indexed register
pub const EXTBUSCTRL: c_uint = 0x10 /* R/W,deleted */;
pub const CLOCKDIV: c_uint = 0x11 /* R/W */;

pub const TERMPWRCTRL: c_uint = 0x13 /* R/W */;

pub const SCSIIRQMODE: c_uint = 0x15 /* R/W */;

pub const IRQPHASESENCE: c_uint = 0x16 /* R */;

pub const TIMERCOUNT: c_uint = 0x17 /* R/W */;
pub const SCSIBUSCTRL: c_uint = 0x18 /* R/W */;

pub const SCSIBUSMON: c_uint = 0x19 /* R */;
pub const SETARBIT: c_uint = 0x1A /* W */;

pub const ARBITSTATUS: c_uint = 0x1A /* R */;
// #  define ARBIT_GO        BIT(0)

pub const PARITYCTRL: c_uint = 0x1B  /* W */;
pub const PARITYSTATUS: c_uint = 0x1B  /* R */;
pub const COMMANDCTRL: c_uint = 0x1C  /* W */;

pub const RESELECTID: c_uint = 0x1C  /* R   */;
pub const COMMANDDATA: c_uint = 0x1D  /* R/W */;
pub const POINTERCLR: c_uint = 0x1E  /*   W */;

pub const TRANSFERCOUNT: c_uint = 0x1E  /* R   */;
pub const TRANSFERMODE: c_uint = 0x20  /* R/W */;

pub const SYNCREG: c_uint = 0x21 /* R/W */;

pub const SCSIDATALATCH: c_uint = 0x22 /*   W */;
pub const SCSIDATAIN: c_uint = 0x22 /* R   */;
pub const SCSIDATAWITHACK: c_uint = 0x23 /* R/W */;
pub const SCAMCONTROL: c_uint = 0x24 /*   W */;
pub const SCAMSTATUS: c_uint = 0x24 /* R   */;
pub const SCAMDATA: c_uint = 0x25 /* R/W */;
pub const OTHERCONTROL: c_uint = 0x26 /* R/W */;

pub const ACKWIDTH: c_uint = 0x27 /* R/W */;
pub const CLRTESTPNT: c_uint = 0x28 /*   W */;
pub const ACKCNTLD: c_uint = 0x29 /*   W */;
pub const REQCNTLD: c_uint = 0x2A /*   W */;
pub const HSTCNTLD: c_uint = 0x2B /*   W */;
pub const CHECKSUM: c_uint = 0x2C /* R/W */;
//
// Input status bit definitions.
//

//
// Useful Bus Monitor status combinations.
//

pub const BUSMON_BUS_FREE: c_int = 0;

// ====================================================================
// synchronous transfer negotiation data
pub const SYNC_NOT_YET: c_int = 0;
pub const SYNC_OK: c_int = 1;
pub const SYNC_NG: c_int = 2;
pub const NSP_MMIO_OFFSET: c_uint = 0x0800;
// int           CurrnetTarget;
pub const MSGBUF_SIZE: c_int = 20;
pub const N_TARGET: c_int = 8;

//
// Card service functions
extern "C" {
    pub fn nsp_cs_detach(p_dev: *mut pcmcia_device) -> static void;
}
extern "C" {
    pub fn nsp_cs_release(link: *mut pcmcia_device) -> static void;
}
extern "C" {
    pub fn nsp_cs_config(link: *mut pcmcia_device) -> static int;
}
// Linux SCSI subsystem specific functions
// Error handler
// static int nsp_eh_abort       (struct scsi_cmnd *SCpnt);
// static int nsp_eh_device_reset(struct scsi_cmnd *SCpnt);
extern "C" {
    pub fn nsp_eh_bus_reset(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_eh_host_reset(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_bus_reset(data: *mut nsp_hw_data) -> static int;
}
//
extern "C" {
    pub fn nsphw_init(data: *mut nsp_hw_data) -> static void;
}
extern "C" {
    pub fn nsphw_start_selection(SCpnt: *mut scsi_cmnd) -> static bool;
}
extern "C" {
    pub fn nsp_start_timer(SCpnt: *mut scsi_cmnd, time: c_int) -> static void;
}
extern "C" {
    pub fn nsp_fifo_count(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_pio_read(SCpnt: *mut scsi_cmnd) -> static void;
}
extern "C" {
    pub fn nsp_pio_write(SCpnt: *mut scsi_cmnd) -> static void;
}
extern "C" {
    pub fn nsp_nexus(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_scsi_done(SCpnt: *mut scsi_cmnd) -> static void;
}
extern "C" {
    pub fn nsp_analyze_sdtr(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_xfer(SCpnt: *mut scsi_cmnd, phase: c_int) -> static int;
}
extern "C" {
    pub fn nsp_dataphase_bypass(SCpnt: *mut scsi_cmnd) -> static int;
}
extern "C" {
    pub fn nsp_reselected(SCpnt: *mut scsi_cmnd) -> static void;
}
// Interrupt handler
// static irqreturn_t nspintr(int irq, void *dev_id);
// Debug

extern "C" {
    pub fn show_command(SCpnt: *mut scsi_cmnd) -> static void;
}
extern "C" {
    pub fn show_phase(SCpnt: *mut scsi_cmnd) -> static void;
}
extern "C" {
    pub fn show_busphase(stat: c_uchar) -> static void;
}
extern "C" {
    pub fn show_message(data: *mut nsp_hw_data) -> static void;
}

//
// SCSI phase
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _scsi_phase {
    PH_UNDETERMINED ,
    PH_ARBSTART     ,
    PH_SELSTART     ,
    PH_SELECTED     ,
    PH_COMMAND      ,
    PH_DATA         ,
    PH_STATUS       ,
    PH_MSG_IN       ,
    PH_MSG_OUT      ,
    PH_DISCONNECT   ,
    PH_RESELECT     ,
    PH_ABORT        ,
    PH_RESET
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _data_in_out {
    IO_UNKNOWN,
    IO_IN,
    IO_OUT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _burst_mode {
    BURST_IO8   = 0,
    BURST_IO32  = 1,
    BURST_MEM32 = 2,
}

// scatter-gather table

// end
