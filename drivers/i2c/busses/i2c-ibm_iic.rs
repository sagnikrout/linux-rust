//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-ibm_iic.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/i2c/busses/i2c-ibm_iic.h
//
// Support for the IIC peripheral on IBM PPC 4xx
//
// Copyright (c) 2003 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// Based on original work by
// Ian DaSilva  <idasilva@mvista.com>
// Armin Kuster <akuster@mvista.com>
// Matt Porter  <mporter@mvista.com>
//
// Copyright 2000-2003 MontaVista Software Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iic_regs {
    pub mdbuf: u16,
    pub sbbuf: u16,
    pub lmadr: u8,
    pub hmadr: u8,
    pub cntl: u8,
    pub mdcntl: u8,
    pub sts: u8,
    pub extsts: u8,
    pub lsadr: u8,
    pub hsadr: u8,
    pub clkdiv: u8,
    pub intmsk: u8,
    pub xfrcnt: u8,
    pub xtcntlss: u8,
    pub directcntl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibm_iic_private {
    pub adap: i2c_adapter,
    pub vaddr: *mut volatile struct iic_regs __iomem,
    pub wq: wait_queue_head_t,
    pub idx: c_int,
    pub irq: c_int,
    pub fast_mode: c_int,
    pub clckdiv: u8,
}

// IICx_CNTL register
pub const CNTL_HMT: c_uint = 0x80;
pub const CNTL_AMD: c_uint = 0x40;
pub const CNTL_TCT_MASK: c_uint = 0x30;
pub const CNTL_TCT_SHIFT: c_int = 4;
pub const CNTL_RPST: c_uint = 0x08;
pub const CNTL_CHT: c_uint = 0x04;
pub const CNTL_RW: c_uint = 0x02;
pub const CNTL_PT: c_uint = 0x01;
// IICx_MDCNTL register
pub const MDCNTL_FSDB: c_uint = 0x80;
pub const MDCNTL_FMDB: c_uint = 0x40;
pub const MDCNTL_EGC: c_uint = 0x20;
pub const MDCNTL_FSM: c_uint = 0x10;
pub const MDCNTL_ESM: c_uint = 0x08;
pub const MDCNTL_EINT: c_uint = 0x04;
pub const MDCNTL_EUBS: c_uint = 0x02;
pub const MDCNTL_HSCL: c_uint = 0x01;
// IICx_STS register
pub const STS_SSS: c_uint = 0x80;
pub const STS_SLPR: c_uint = 0x40;
pub const STS_MDBS: c_uint = 0x20;
pub const STS_MDBF: c_uint = 0x10;
pub const STS_SCMP: c_uint = 0x08;
pub const STS_ERR: c_uint = 0x04;
pub const STS_IRQA: c_uint = 0x02;
pub const STS_PT: c_uint = 0x01;
// IICx_EXTSTS register
pub const EXTSTS_IRQP: c_uint = 0x80;
pub const EXTSTS_BCS_MASK: c_uint = 0x70;
pub const EXTSTS_BCS_FREE: c_uint = 0x40;
pub const EXTSTS_IRQD: c_uint = 0x08;
pub const EXTSTS_LA: c_uint = 0x04;
pub const EXTSTS_ICT: c_uint = 0x02;
pub const EXTSTS_XFRA: c_uint = 0x01;
// IICx_INTRMSK register
pub const INTRMSK_EIRC: c_uint = 0x80;
pub const INTRMSK_EIRS: c_uint = 0x40;
pub const INTRMSK_EIWC: c_uint = 0x20;
pub const INTRMSK_EIWS: c_uint = 0x10;
pub const INTRMSK_EIHE: c_uint = 0x08;
pub const INTRMSK_EIIC: c_uint = 0x04;
pub const INTRMSK_EITA: c_uint = 0x02;
pub const INTRMSK_EIMTC: c_uint = 0x01;
// IICx_XFRCNT register
pub const XFRCNT_MTC_MASK: c_uint = 0x07;
// IICx_XTCNTLSS register
pub const XTCNTLSS_SRC: c_uint = 0x80;
pub const XTCNTLSS_SRS: c_uint = 0x40;
pub const XTCNTLSS_SWC: c_uint = 0x20;
pub const XTCNTLSS_SWS: c_uint = 0x10;
pub const XTCNTLSS_SRST: c_uint = 0x01;
// IICx_DIRECTCNTL register
pub const DIRCNTL_SDAC: c_uint = 0x08;
pub const DIRCNTL_SCC: c_uint = 0x04;
pub const DIRCNTL_MSDA: c_uint = 0x02;
pub const DIRCNTL_MSC: c_uint = 0x01;
// Check if we really control the I2C bus and bus is free

