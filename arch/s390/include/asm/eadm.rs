//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/eadm.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arqb {
    pub data: u64,
    pub fmt:4: u16,
    pub cmd_code: u16,
    pub msb_count: u16,
    pub reserved: [u32; 12],
    pub __packed: },
pub const ARQB_CMD_MOVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arsb {
    pub fmt:4: u16,
    pub ef: u8,
    pub ecbi: u8,
    pub fvf: u8,
    pub eqc: u8,
    pub fail_msb: u64,
    pub fail_aidaw: u64,
    pub fail_ms: u64,
    pub fail_scm: u64,
    pub reserved: [u32; 4],
    pub __packed: },
pub const EQC_WR_PROHIBIT: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msb {
    pub fmt:4: u8,
    pub oc:4: u8,
    pub flags: u8,
    pub bs:4: u16,
    pub blk_count: u32,
    pub data_addr: dma64_t,
    pub scm_addr: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aidaw {
    pub flags: u8,
    pub :24: u32,
    pub :32: u32,
    pub data_addr: dma64_t,
    pub __packed: },
pub const MSB_OC_CLEAR: c_int = 0;
pub const MSB_OC_READ: c_int = 1;
pub const MSB_OC_WRITE: c_int = 2;
pub const MSB_OC_RELEASE: c_int = 3;
pub const MSB_FLAG_BNM: c_uint = 0x80;
pub const MSB_FLAG_IDA: c_uint = 0x40;
pub const MSB_BS_4K: c_int = 0;
pub const MSB_BS_1M: c_int = 1;
pub const AOB_NR_MSB: c_int = 124;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aob {
    pub request: arqb,
    pub response: arsb,
    pub msb: [msb; AOB_NR_MSB],
    pub __aligned(PAGE_SIZE): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aob_rq_header {
    pub scmdev: *mut scm_device,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_device {
    pub address: u64,
    pub size: u64,
    pub nr_max_block: c_uint,
    pub dev: device,
    pub persistence:4: c_uint,
    pub oper_state:4: c_uint,
    pub data_state:4: c_uint,
    pub rank:4: c_uint,
    pub release:1: c_uint,
    pub res_id:8: c_uint,
    pub attrs: } __packed,
}

pub const OP_STATE_GOOD: c_int = 1;
pub const OP_STATE_TEMP_ERR: c_int = 2;
pub const OP_STATE_PERM_ERR: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scm_event {

    struct scm_driver {
    struct device_driver drv;
    int (*probe) (struct scm_device *scmdev);
    void (*remove) (struct scm_device *scmdev);
    void (*notify) (struct scm_device *scmdev, enum scm_event event);
    void (*handler) (struct scm_device *scmdev, void *data,
    blk_status_t error);
}

extern "C" {
    pub fn scm_driver_register(scmdrv: *mut scm_driver) -> c_int;
}
extern "C" {
    pub fn scm_driver_unregister(scmdrv: *mut scm_driver);
}
extern "C" {
    pub fn eadm_start_aob(aob: *mut aob) -> c_int;
}
extern "C" {
    pub fn scm_irq_handler(aob: *mut aob, error: blk_status_t);
}
