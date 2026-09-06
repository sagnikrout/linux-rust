//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qlogicfas408.h
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
// to be used by qlogicfas and qlogic_cs
// ----------------------------------------------------------------
// Configuration
// Set the following to max out the speed of the PIO PseudoDMA transfers,
pub const QL_TURBO_PDMA: c_int = 1;
// This should be 1 to enable parity detection
pub const QL_ENABLE_PARITY: c_int = 1;
// This will reset all devices when the driver is initialized (during bootup).
pub const QL_RESET_AT_START: c_int = 0;
// crystal frequency in megahertz (for offset 5 and 9)
pub const XTALFREQ: c_int = 40;
//
// DANGER! modify these at your own risk
// SLOWCABLE can usually be reset to zero if you have a clean setup and
//
// config register 1 (offset 8) options
// This needs to be set to 1 if your cabling is long or noisy
pub const SLOWCABLE: c_int = 1;
//
// offset 0xc
// This will set fast (10Mhz) synchronous timing when set to 1
pub const FASTSCSI: c_int = 0;
// This when set to 1 will set a faster sync transfer rate

//
// offset 6
// This is the sync transfer divisor, XTALFREQ/X will be the maximum

//
// offset 7
// This is the count of how many synchronous transfers can take place
pub const SYNCOFFST: c_int = 0;
// for the curious, bits 7&6 control the deassertion delay in 1/2 cycles
// ----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlogicfas408_priv {
    pub /: *mut *mut int qbase; / Port,
    pub /: *mut *mut int qinitid; / initiator ID,
    pub /: *mut *mut int qabort; / Flag to cause an abort,
    pub /: *mut *mut int qlirq; / IRQ being used,
    pub /: *mut *mut int int_type; / type of irq, 2 for ISA board, 0 for PCMCIA,
    pub /: *mut *mut char qinfo[80]; / description,
    pub /: *mut *mut *mut scsi_cmnd qlcmd; / current command being processed,
    pub /: *mut *mut *mut Scsi_Host shost; / pointer back to host,
    pub /: *mut *mut *mut qlogicfas408_priv next; / next private struct,
}

// The qlogic card uses two register maps - These macros select which one

// following is watchdog timeout in microseconds
pub const WATCHDOG: c_int = 5000000;
// ----------------------------------------------------------------
// the following will set the monitor border color (useful to find

extern "C" {
    pub fn qlogicfas408_ihandl(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qlogicfas408_abort(cmd: *mut *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn qlogicfas408_host_reset(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn qlogicfas408_get_chip_type(qbase: c_int, int_type: c_int) -> c_int;
}
extern "C" {
    pub fn qlogicfas408_setup(qbase: c_int, id: c_int, int_type: c_int);
}
extern "C" {
    pub fn qlogicfas408_detect(qbase: c_int, int_type: c_int) -> c_int;
}
extern "C" {
    pub fn qlogicfas408_disable_ints(priv: *mut qlogicfas408_priv);
}
