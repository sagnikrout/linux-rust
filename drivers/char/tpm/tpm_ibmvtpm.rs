//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm_ibmvtpm.h
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
// Copyright (C) 2012 IBM Corporation
//
// Author: Ashley Lai <ashleydlai@gmail.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//
// vTPM Message Format 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvtpm_crq {
    pub valid: u8,
    pub msg: u8,
    pub len: __be16,
    pub data: __be32,
    pub reserved: __be64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvtpm_crq_queue {
    pub crq_addr: *mut ibmvtpm_crq,
    pub index: u32,
    pub num_entry: u32,
    pub wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvtpm_dev {
    pub dev: *mut device,
    pub vdev: *mut vio_dev,
    pub crq_queue: ibmvtpm_crq_queue,
    pub crq_dma_handle: dma_addr_t,
    pub rtce_size: u32,
    pub rtce_buf: *mut void __iomem,
    pub rtce_dma_handle: dma_addr_t,
    pub rtce_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub res_len: u16,
    pub vtpm_version: u32,
    pub tpm_processing_cmd: u8,
}

// Initialize CRQ
pub const INIT_CRQ_CMD: c_uint = 0xC001000000000000LL /* Init cmd */;
pub const INIT_CRQ_COMP_CMD: c_uint = 0xC002000000000000LL /* Init complete cmd */;
pub const INIT_CRQ_RES: c_uint = 0x01	/* Init respond */;
pub const INIT_CRQ_COMP_RES: c_uint = 0x02	/* Init complete respond */;
pub const VALID_INIT_CRQ: c_uint = 0xC0	/* Valid command for init crq */;
// vTPM CRQ response is the message type | 0x80
pub const VTPM_MSG_RES: c_uint = 0x80;
pub const IBMVTPM_VALID_CMD: c_uint = 0x80;
// vTPM CRQ message types
pub const VTPM_GET_VERSION: c_uint = 0x01;

pub const VTPM_TPM_COMMAND: c_uint = 0x02;

pub const VTPM_GET_RTCE_BUFFER_SIZE: c_uint = 0x03;

pub const VTPM_PREPARE_TO_SUSPEND: c_uint = 0x04;

