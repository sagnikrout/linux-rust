//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_eh.h
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

extern "C" {
    pub fn scsi_eh_flush_done_q(done_q: *mut list_head);
}
extern "C" {
    pub fn scsi_report_bus_reset(: *mut Scsi_Host, _arg: c_int);
}
extern "C" {
    pub fn scsi_report_device_reset(: *mut Scsi_Host, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn scsi_block_when_processing_errors(: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn scsi_check_sense(: *mut scsi_cmnd) -> scsi_disposition;
}
extern "C" {
    pub fn scsi_ioctl_reset(: *mut scsi_device, : *mut int __user) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_eh_save {
// saved state
    pub result: c_int,
    pub resid_len: c_uint,
    pub eh_eflags: c_int,
    pub data_direction: dma_data_direction,
    pub underflow: unsigned,
    pub cmd_len: c_uchar,
    pub prot_op: c_uchar,
    pub cmnd: [c_uchar; 32],
    pub sdb: scsi_data_buffer,
    pub sense_sgl: scatterlist,
// struct request fields

    pub rq_crypt_ctx: *mut bio_crypt_ctx,
    pub rq_crypt_keyslot: *mut blk_crypto_keyslot,

}
