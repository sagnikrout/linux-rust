//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/qdio.h
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
// Copyright IBM Corp. 2000, 2008
// Author(s): Utz Bacher <utz.bacher@de.ibm.com>
// Jan Glauber <jang@linux.vnet.ibm.com>
//

// only use 4 queues to save some cachelines
pub const QDIO_MAX_QUEUES_PER_IRQ: c_int = 4;
pub const QDIO_MAX_BUFFERS_PER_Q: c_int = 128;

pub const QDIO_MAX_ELEMENTS_PER_BUFFER: c_int = 16;
pub const QDIO_QETH_QFMT: c_int = 0;
pub const QDIO_ZFCP_QFMT: c_int = 1;
pub const QDIO_IQDIO_QFMT: c_int = 2;
//
// struct qdesfmt0 - queue descriptor, format 0
// @sliba: absolute address of storage list information block
// @sla: absolute address of storage list
// @slsba: absolute address of storage list state block
// @akey: access key for SLIB
// @bkey: access key for SL
// @ckey: access key for SBALs
// @dkey: access key for SLSB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdesfmt0 {
    pub sliba: dma64_t,
    pub sla: dma64_t,
    pub slsba: dma64_t,
    pub 32: u32 :,
    pub 4: u32 akey :,
    pub 4: u32 bkey :,
    pub 4: u32 ckey :,
    pub 4: u32 dkey :,
    pub 16: u32 :,
// C attribute field omitted
pub const QDR_AC_MULTI_BUFFER_ENABLE: c_uint = 0x01;
//
// struct qdr - queue description record (QDR)
// @qfmt: queue format
// @ac: adapter characteristics
// @iqdcnt: input queue descriptor count
// @oqdcnt: output queue descriptor count
// @iqdsz: input queue descriptor size
// @oqdsz: output queue descriptor size
// @qiba: absolute address of queue information block
// @qkey: queue information block key
// @qdf0: queue descriptions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdr {
    pub 8: u32 qfmt :,
    pub 16: u32 :,
    pub 8: u32 ac :,
    pub 8: u32 :,
    pub 8: u32 iqdcnt :,
    pub 8: u32 :,
    pub 8: u32 oqdcnt :,
    pub 8: u32 :,
    pub 8: u32 iqdsz :,
    pub 8: u32 :,
    pub 8: u32 oqdsz :,
// private:
    pub res: [u32; 9],
// public:
    pub qiba: dma64_t,
    pub 32: u32 :,
    pub 4: u32 qkey :,
    pub 28: u32 :,
    pub qdf0: [qdesfmt0; 126],
    pub __aligned(PAGE_SIZE): } __packed,
pub const QIB_AC_OUTBOUND_PCI_SUPPORTED: c_uint = 0x40;
pub const QIB_RFLAGS_ENABLE_QEBSM: c_uint = 0x80;
pub const QIB_RFLAGS_ENABLE_DATA_DIV: c_uint = 0x02;
//
// struct qib - queue information block (QIB)
// @qfmt: queue format
// @pfmt: implementation dependent parameter format
// @rflags: QEBSM
// @ac: adapter characteristics
// @isliba: logical address of first input SLIB
// @osliba: logical address of first output SLIB
// @ebcnam: adapter identifier in EBCDIC
// @parm: implementation dependent parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qib {
    pub 8: u32 qfmt :,
    pub 8: u32 pfmt :,
    pub 8: u32 rflags :,
    pub 8: u32 ac :,
    pub 32: u32 :,
    pub isliba: u64,
    pub osliba: u64,
    pub 32: u32 :,
    pub 32: u32 :,
    pub ebcnam: [u8; 8],
// private:
    pub res: [u8; 88],
// public:
    pub parm: [u8; 128],
// C attribute field omitted
//
// struct slibe - storage list information block element (SLIBE)
// @parms: implementation dependent parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slibe {
    pub parms: u64,
}

//
// struct qaob - queue asynchronous operation block
// @res0: reserved parameters
// @res1: reserved parameter
// @res2: reserved parameter
// @res3: reserved parameter
// @aorc: asynchronous operation return code
// @flags: internal flags
// @cbtbs: control block type
// @sb_count: number of storage blocks
// @sba: storage block element addresses
// @dcount: size of storage block elements
// @user0: user definable value
// @res4: reserved parameter
// @user1: user definable value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaob {
    pub res0: [u64; 6],
    pub res1: u8,
    pub res2: u8,
    pub res3: u8,
    pub aorc: u8,
    pub flags: u8,
    pub cbtbs: u16,
    pub sb_count: u8,
    pub sba: [dma64_t; QDIO_MAX_ELEMENTS_PER_BUFFER],
    pub dcount: [u16; QDIO_MAX_ELEMENTS_PER_BUFFER],
    pub user0: u64,
    pub res4: [u64; 2],
    pub user1: [u8; 16],
// C attribute field omitted
//
// struct slib - storage list information block (SLIB)
// @nsliba: next SLIB address (if any)
// @sla: SL address
// @slsba: SLSB address
// @slibe: SLIB elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slib {
    pub nsliba: u64,
    pub sla: u64,
    pub slsba: u64,
// private:
    pub res: [u8; 1000],
// public:
    pub slibe: [slibe; QDIO_MAX_BUFFERS_PER_Q],
// C attribute field omitted
pub const SBAL_EFLAGS_LAST_ENTRY: c_uint = 0x40;
pub const SBAL_EFLAGS_CONTIGUOUS: c_uint = 0x20;
pub const SBAL_EFLAGS_FIRST_FRAG: c_uint = 0x04;
pub const SBAL_EFLAGS_MIDDLE_FRAG: c_uint = 0x08;
pub const SBAL_EFLAGS_LAST_FRAG: c_uint = 0x0c;
pub const SBAL_EFLAGS_MASK: c_uint = 0x6f;
pub const SBAL_SFLAGS0_PCI_REQ: c_uint = 0x40;
pub const SBAL_SFLAGS0_DATA_CONTINUATION: c_uint = 0x20;
// Awesome OpenFCP extensions
pub const SBAL_SFLAGS0_TYPE_STATUS: c_uint = 0x00;
pub const SBAL_SFLAGS0_TYPE_WRITE: c_uint = 0x08;
pub const SBAL_SFLAGS0_TYPE_READ: c_uint = 0x10;
pub const SBAL_SFLAGS0_TYPE_WRITE_READ: c_uint = 0x18;
pub const SBAL_SFLAGS0_MORE_SBALS: c_uint = 0x04;
pub const SBAL_SFLAGS0_COMMAND: c_uint = 0x02;
pub const SBAL_SFLAGS0_LAST_SBAL: c_uint = 0x00;

//
// struct qdio_buffer_element - SBAL entry
// @eflags: SBAL entry flags
// @scount: SBAL count
// @sflags: whole SBAL flags
// @length: length
// @addr: absolute data address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_buffer_element {
    pub eflags: u8,
// private:
    pub res1: u8,
// public:
    pub scount: u8,
    pub sflags: u8,
    pub length: u32,
    pub addr: dma64_t,
// C attribute field omitted
//
// struct qdio_buffer - storage block address list (SBAL)
// @element: SBAL entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_buffer {
    pub element: [qdio_buffer_element; QDIO_MAX_ELEMENTS_PER_BUFFER],
// C attribute field omitted
//
// struct sl_element - storage list entry
// @sbal: absolute SBAL address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl_element {
    pub sbal: dma64_t,
// C attribute field omitted
//
// struct sl - storage list (SL)
// @element: SL entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl {
    pub element: [sl_element; QDIO_MAX_BUFFERS_PER_Q],
// C attribute field omitted
//
// struct slsb - storage list state block (SLSB)
// @val: state per buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slsb {
    pub val: [u8; QDIO_MAX_BUFFERS_PER_Q],
// C attribute field omitted
// qdio adapter-characteristics-1 flag
pub const CHSC_AC1_INITIATE_INPUTQ: c_uint = 0x80;
pub const AC1_SIGA_INPUT_NEEDED: c_uint = 0x40	/* process input queues */;
pub const AC1_SIGA_OUTPUT_NEEDED: c_uint = 0x20	/* process output queues */;
pub const AC1_SIGA_SYNC_NEEDED: c_uint = 0x10	/* ask hypervisor to sync */;
pub const AC1_AUTOMATIC_SYNC_ON_THININT: c_uint = 0x08	/* set by hypervisor */;
pub const AC1_AUTOMATIC_SYNC_ON_OUT_PCI: c_uint = 0x04	/* set by hypervisor */;
pub const AC1_SC_QEBSM_AVAILABLE: c_uint = 0x02	/* available for subchannel */;
pub const AC1_SC_QEBSM_ENABLED: c_uint = 0x01	/* enabled for subchannel */;
pub const CHSC_AC2_MULTI_BUFFER_AVAILABLE: c_uint = 0x0080;
pub const CHSC_AC2_MULTI_BUFFER_ENABLED: c_uint = 0x0040;
pub const CHSC_AC2_DATA_DIV_AVAILABLE: c_uint = 0x0010;
pub const CHSC_AC2_SNIFFER_AVAILABLE: c_uint = 0x0008;
pub const CHSC_AC2_DATA_DIV_ENABLED: c_uint = 0x0002;
pub const CHSC_AC3_FORMAT2_CQ_AVAILABLE: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_ssqd_desc {
    pub flags: u8,
    pub sch: u16,
    pub qfmt: u8,
    pub parm: u8,
    pub qdioac1: u8,
    pub sch_class: u8,
    pub pcnt: u8,
    pub icnt: u8,
    pub ocnt: u8,
    pub mbccnt: u8,
    pub qdioac2: u16,
    pub sch_token: u64,
    pub mro: u8,
    pub mri: u8,
    pub qdioac3: u16,
    pub mmwc: u8,
// C attribute field omitted
// params are: ccw_device, qdio_error, queue_number,
    pub long): int, int, unsigned,
// qdio errors reported through the queue handlers:
pub const QDIO_ERROR_ACTIVATE: c_uint = 0x0001;
pub const QDIO_ERROR_GET_BUF_STATE: c_uint = 0x0002;
pub const QDIO_ERROR_SET_BUF_STATE: c_uint = 0x0004;
// extra info for completed SBALs:
pub const QDIO_ERROR_SLSB_STATE: c_uint = 0x0100;
pub const QDIO_ERROR_SLSB_PENDING: c_uint = 0x0200;
// for qdio_cleanup
pub const QDIO_FLAG_CLEANUP_USING_CLEAR: c_uint = 0x01;
pub const QDIO_FLAG_CLEANUP_USING_HALT: c_uint = 0x02;
//
// struct qdio_initialize - qdio initialization data
// @q_format: queue format
// @qdr_ac: feature flags to set
// @qib_param_field_format: format for qib_parm_field
// @qib_param_field: pointer to 128 bytes or NULL, if no param field
// @qib_rflags: rflags to set
// @no_input_qs: number of input queues
// @no_output_qs: number of output queues
// @input_handler: handler to be called for input queues, and device-wide errors
// @output_handler: handler to be called for output queues
// @irq_poll: Data IRQ polling handler
// @scan_threshold: # of in-use buffers that triggers scan on output queue
// @int_parm: interruption parameter
// @input_sbal_addr_array:  per-queue array, each element points to 128 SBALs
// @output_sbal_addr_array: per-queue array, each element points to 128 SBALs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdio_initialize {
    pub q_format: c_uchar,
    pub qdr_ac: c_uchar,
    pub qib_param_field_format: c_uint,
    pub qib_param_field: *mut c_uchar,
    pub qib_rflags: c_uchar,
    pub no_input_qs: c_uint,
    pub no_output_qs: c_uint,
    pub input_handler: *mut qdio_handler_t,
    pub output_handler: *mut qdio_handler_t,
    pub data): *mut *mut *mut void (irq_poll)(struct ccw_device cdev, unsigned long,
    pub int_parm: c_ulong,
    pub input_sbal_addr_array: *mut qdio_buffer,
    pub output_sbal_addr_array: *mut qdio_buffer,
}

extern "C" {
    pub fn qdio_alloc_buffers(buf: *mut qdio_buffer, count: c_uint) -> c_int;
}
extern "C" {
    pub fn qdio_free_buffers(buf: *mut qdio_buffer, count: c_uint);
}
extern "C" {
    pub fn qdio_reset_buffers(buf: *mut qdio_buffer, count: c_uint);
}
extern "C" {
    pub fn qdio_activate(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn qdio_start_irq(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn qdio_stop_irq(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn qdio_shutdown(: *mut ccw_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qdio_free(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn qdio_get_ssqd_desc(: *mut ccw_device, : *mut qdio_ssqd_desc) -> c_int;
}
