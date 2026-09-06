//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/cio.h
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
//
// Common interface for I/O on S/390
//

pub const LPM_ANYPATH: c_uint = 0xff;
pub const __MAX_CSSID: c_int = 0;
pub const __MAX_SUBCHANNEL: c_int = 65535;
pub const __MAX_SSID: c_int = 3;

pub const CCW_MAX_BYTE_COUNT: c_int = 65535;
//
// struct ccw1 - channel command word
// @cmd_code: command code
// @flags: flags, like IDA addressing, etc.
// @count: byte count
// @cda: data address
//
// The ccw is the basic structure to build channel programs that perform
// operations with the device or the control unit. Only Format-1 channel
// command words are supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw1 {
    pub cmd_code: __u8,
    pub flags: __u8,
    pub count: __u16,
    pub cda: dma32_t,
    pub ((packed,aligned(8))): } __attribute__,
//
// struct ccw0 - channel command word
// @cmd_code: command code
// @cda: data address
// @flags: flags, like IDA addressing, etc.
// @reserved: will be ignored
// @count: byte count
//
// The format-0 ccw structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw0 {
    pub cmd_code: __u8,
    pub 24: __u32 cda :,
    pub flags: __u8,
    pub reserved: __u8,
    pub count: __u16,
    pub __aligned(8): } __packed,
pub const CCW_FLAG_DC: c_uint = 0x80;
pub const CCW_FLAG_CC: c_uint = 0x40;
pub const CCW_FLAG_SLI: c_uint = 0x20;
pub const CCW_FLAG_SKIP: c_uint = 0x10;
pub const CCW_FLAG_PCI: c_uint = 0x08;
pub const CCW_FLAG_IDA: c_uint = 0x04;
pub const CCW_FLAG_SUSPEND: c_uint = 0x02;
pub const CCW_CMD_READ_IPL: c_uint = 0x02;
pub const CCW_CMD_NOOP: c_uint = 0x03;
pub const CCW_CMD_BASIC_SENSE: c_uint = 0x04;
pub const CCW_CMD_TIC: c_uint = 0x08;
pub const CCW_CMD_STLCK: c_uint = 0x14;
pub const CCW_CMD_SENSE_PGID: c_uint = 0x34;
pub const CCW_CMD_SUSPEND_RECONN: c_uint = 0x5B;
pub const CCW_CMD_RDC: c_uint = 0x64;
pub const CCW_CMD_RELEASE: c_uint = 0x94;
pub const CCW_CMD_SET_PGID: c_uint = 0xAF;
pub const CCW_CMD_SENSE_ID: c_uint = 0xE4;
pub const CCW_CMD_DCTL: c_uint = 0xF3;
pub const SENSE_MAX_COUNT: c_uint = 0x20;
//
// struct erw - extended report word
// @res0: reserved
// @auth: authorization check
// @pvrf: path-verification-required flag
// @cpt: channel-path timeout
// @fsavf: failing storage address validity flag
// @cons: concurrent sense
// @scavf: secondary ccw address validity flag
// @fsaf: failing storage address format
// @scnt: sense count, if @cons == %1
// @res16: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erw {
    pub 3: __u32 res0 :,
    pub 1: __u32 auth :,
    pub 1: __u32 pvrf :,
    pub 1: __u32 cpt :,
    pub 1: __u32 fsavf :,
    pub 1: __u32 cons :,
    pub 1: __u32 scavf :,
    pub 1: __u32 fsaf :,
    pub 6: __u32 scnt :,
    pub 16: __u32 res16 :,
// C attribute field omitted
//
// struct erw_eadm - EADM Subchannel extended report word
// @b: aob error
// @r: arsb error
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erw_eadm {
    pub 16: __u32 :,
    pub 1: __u32 b :,
    pub 1: __u32 r :,
    pub 14: __u32 :,
    pub __packed: },
//
// struct sublog - subchannel logout area
// @res0: reserved
// @esf: extended status flags
// @lpum: last path used mask
// @arep: ancillary report
// @fvf: field-validity flags
// @sacc: storage access code
// @termc: termination code
// @devsc: device-status check
// @serr: secondary error
// @ioerr: i/o-error alert
// @seqc: sequence code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sublog {
    pub 1: __u32 res0 :,
    pub 7: __u32 esf :,
    pub 8: __u32 lpum :,
    pub 1: __u32 arep :,
    pub 5: __u32 fvf :,
    pub 2: __u32 sacc :,
    pub 2: __u32 termc :,
    pub 1: __u32 devsc :,
    pub 1: __u32 serr :,
    pub 1: __u32 ioerr :,
    pub 3: __u32 seqc :,
// C attribute field omitted
//
// struct esw0 - Format 0 Extended Status Word (ESW)
// @sublog: subchannel logout
// @erw: extended report word
// @faddr: failing storage address
// @saddr: secondary ccw address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw0 {
    pub sublog: sublog,
    pub erw: erw,
    pub faddr: [dma32_t; 2],
    pub saddr: dma32_t,
// C attribute field omitted
//
// struct esw1 - Format 1 Extended Status Word (ESW)
// @zero0: reserved zeros
// @lpum: last path used mask
// @zero16: reserved zeros
// @erw: extended report word
// @zeros: three fullwords of zeros
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw1 {
    pub zero0: __u8,
    pub lpum: __u8,
    pub zero16: __u16,
    pub erw: erw,
    pub zeros: [__u32; 3],
// C attribute field omitted
//
// struct esw2 - Format 2 Extended Status Word (ESW)
// @zero0: reserved zeros
// @lpum: last path used mask
// @dcti: device-connect-time interval
// @erw: extended report word
// @zeros: three fullwords of zeros
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw2 {
    pub zero0: __u8,
    pub lpum: __u8,
    pub dcti: __u16,
    pub erw: erw,
    pub zeros: [__u32; 3],
// C attribute field omitted
//
// struct esw3 - Format 3 Extended Status Word (ESW)
// @zero0: reserved zeros
// @lpum: last path used mask
// @res: reserved
// @erw: extended report word
// @zeros: three fullwords of zeros
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw3 {
    pub zero0: __u8,
    pub lpum: __u8,
    pub res: __u16,
    pub erw: erw,
    pub zeros: [__u32; 3],
// C attribute field omitted
//
// struct esw_eadm - EADM Subchannel Extended Status Word (ESW)
// @sublog: subchannel logout
// @erw: extended report word
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw_eadm {
    pub sublog: __u32,
    pub erw: erw_eadm,
    pub 32: __u32 :,
    pub 32: __u32 :,
    pub 32: __u32 :,
    pub __packed: },
//
// struct irb - interruption response block
// @scsw: subchannel status word
// @esw: extended status word
// @ecw: extended control word
//
// The irb that is handed to the device driver when an interrupt occurs. For
// solicited interrupts, the common I/O layer already performs checks whether
// a field is valid; a field not being valid is always passed as %0.
// If a unit check occurred, @ecw may contain sense data; this is retrieved
// by the common I/O layer itself if the device doesn't support concurrent
// sense (so that the device driver never needs to perform basic sense itself).
// For unsolicited interrupts, the irb is passed as-is (expect for sense data,
// if applicable).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irb {
    pub scsw: scsw,
    pub esw0: esw0,
    pub esw1: esw1,
    pub esw2: esw2,
    pub esw3: esw3,
    pub eadm: esw_eadm,
    pub esw: },
    pub ecw: [__u8; 32],
    pub ((packed,aligned(4))): } __attribute__,
//
// struct ciw - command information word  (CIW) layout
// @et: entry type
// @reserved: reserved bits
// @ct: command type
// @cmd: command code
// @count: command count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ciw {
    pub 2: __u32 et :,
    pub 2: __u32 reserved :,
    pub 4: __u32 ct :,
    pub 8: __u32 cmd :,
    pub 16: __u32 count :,
// C attribute field omitted
pub const CIW_TYPE_RCD: c_uint = 0x0    	/* read configuration data */;
pub const CIW_TYPE_SII: c_uint = 0x1    	/* set interface identifier */;
pub const CIW_TYPE_RNI: c_uint = 0x2    	/* read node identifier */;
//
// Node Descriptor as defined in SA22-7204, "Common I/O-Device Commands"
//
pub const ND_VALIDITY_VALID: c_int = 0;
pub const ND_VALIDITY_OUTDATED: c_int = 1;
pub const ND_VALIDITY_INVALID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_descriptor {
// Flags.
    pub validity:3: u32,
    pub reserved:5: u32,
    pub __packed: },
    pub byte0: u8,
    pub __packed: },
// Node parameters.
    pub params:24: u32,
// Node ID.
    pub type: [c_char; 6],
    pub model: [c_char; 3],
    pub manufacturer: [c_char; 3],
    pub plant: [c_char; 2],
    pub seq: [c_char; 12],
    pub tag: u16,
    pub __packed: },
//
// Flags used as input parameters for do_IO()
//
pub const DOIO_ALLOW_SUSPEND: c_uint = 0x0001 /* allow for channel prog. suspend */;
pub const DOIO_DENY_PREFETCH: c_uint = 0x0002 /* don't allow for CCW prefetch */;
pub const DOIO_SUPPRESS_INTER: c_uint = 0x0004 /* suppress intermediate inter. */;
// ... for suspended CCWs
// Device or subchannel gone.
pub const CIO_GONE: c_uint = 0x0001;
// No path to device.
pub const CIO_NO_PATH: c_uint = 0x0002;
// Device has appeared.
pub const CIO_OPER: c_uint = 0x0004;
// Sick revalidation of device.
pub const CIO_REVALIDATE: c_uint = 0x0008;
// Device did not respond in time.
pub const CIO_BOXED: c_uint = 0x0010;
//
// struct ccw_dev_id - unique identifier for ccw devices
// @ssid: subchannel set id
// @devno: device number
//
// This structure is not directly based on any hardware structure. The
// hardware identifies a device by its device number and its subchannel,
// which is in turn identified by its id. In order to get a unique identifier
// for ccw devices across subchannel sets, @struct ccw_dev_id has been
// introduced.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_dev_id {
    pub ssid: u8,
    pub devno: u16,
}

//
// ccw_dev_id_is_equal() - compare two ccw_dev_ids
// @dev_id1: a ccw_dev_id
// @dev_id2: another ccw_dev_id
// Returns:
// %1 if the two structures are equal field-by-field,
// %0 if not.
// Context:
// any
//
// pathmask_to_pos() - find the position of the left-most bit in a pathmask
// @mask: pathmask with at least one bit set
//
extern "C" {
    pub fn css_schedule_reprobe();
}
extern "C" {
    pub fn cio_dma_free(cpu_addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn cio_gp_dma_free(gp_dma: *mut gen_pool, cpu_addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn cio_gp_dma_destroy(gp_dma: *mut gen_pool, dma_dev: *mut device);
}
// Function from drivers/s390/cio/chsc.c
extern "C" {
    pub fn chsc_sstpc(page: *mut c_void, op: c_uint, ctrl: u16, clock_delta: *mut c_long) -> c_int;
}
extern "C" {
    pub fn chsc_sstpi(page: *mut c_void, result: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn chsc_stzi(page: *mut c_void, result: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn chsc_sgib(origin: u32) -> c_int;
}
extern "C" {
    pub fn chsc_scud(cu: u16, esm: *mut u64, esm_valid: *mut u8) -> c_int;
}
