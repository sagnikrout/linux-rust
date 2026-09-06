//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/ultrasoc-smb.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Siemens System Memory Buffer driver.
// Copyright(c) 2022, HiSilicon Limited.
//

// Offset of SMB global registers
pub const SMB_GLB_CFG_REG: c_uint = 0x00;
pub const SMB_GLB_EN_REG: c_uint = 0x04;
pub const SMB_GLB_INT_REG: c_uint = 0x08;
// Offset of SMB logical buffer registers
pub const SMB_LB_CFG_LO_REG: c_uint = 0x40;
pub const SMB_LB_CFG_HI_REG: c_uint = 0x44;
pub const SMB_LB_INT_CTRL_REG: c_uint = 0x48;
pub const SMB_LB_INT_STS_REG: c_uint = 0x4c;
pub const SMB_LB_RD_ADDR_REG: c_uint = 0x5c;
pub const SMB_LB_WR_ADDR_REG: c_uint = 0x60;
pub const SMB_LB_PURGE_REG: c_uint = 0x64;
// Set global config register

// Set global interrupt control register

// Set logical buffer config register lower 32 bits

// Set logical buffer config register upper 32 bits

//
// Set logical buffer interrupt control register.
// The register control the validity of both real-time events and
// interrupts. When logical buffer status changes causes to issue
// an interrupt at the same time as it issues a real-time event.
// Real-time events are used in SMB driver, which needs to get the buffer
// status. Interrupts are used in debugger mode.
// SMB_LB_INT_CTRL_BUF_NOTE_MASK control which events flags or interrupts
// are valid.
//

// Set logical buffer interrupt status register

pub const SMB_REG_ADDR_RES: c_int = 0;
pub const SMB_BUF_ADDR_RES: c_int = 1;

//
// struct smb_data_buffer - Details of the buffer used by SMB
// @buf_base:	Memory mapped base address of SMB.
// @buf_hw_base:	SMB buffer start Physical base address, only used 32bits.
// @buf_size:	Size of the buffer.
// @data_size:	Size of the available trace data for SMB.
// @buf_rdptr:	Current read position (index) within the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_data_buffer {
    pub buf_base: *mut c_void,
    pub buf_hw_base: u32,
    pub buf_size: c_ulong,
    pub data_size: c_ulong,
    pub buf_rdptr: c_ulong,
}

//
// struct smb_drv_data - specifics associated to an SMB component
// @base:	Memory mapped base address for SMB component.
// @csdev:	Component vitals needed by the framework.
// @sdb:	Data buffer for SMB.
// @miscdev:	Specifics to handle "/dev/xyz.smb" entry.
// @spinlock:	Control data access to one at a time.
// @reading:	Synchronise user space access to SMB buffer.
// @pid:	Process ID of the process being monitored by the
// session that is using this component.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_drv_data {
    pub base: *mut void __iomem,
    pub csdev: *mut coresight_device,
    pub sdb: smb_data_buffer,
    pub miscdev: miscdevice,
    pub spinlock: raw_spinlock_t,
    pub reading: bool,
    pub pid: pid_t,
}
