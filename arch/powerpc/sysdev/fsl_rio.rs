//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/fsl_rio.h
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
// Freescale MPC85xx/MPC86xx RapidIO support
//
// Copyright 2009 Sysgo AG
// Thomas Moll <thomas.moll@sysgo.com>
// - fixed maintenance access routines, check for aligned access
//
// Copyright 2009 Integrated Device Technology, Inc.
// Alex Bounine <alexandre.bounine@idt.com>
// - Added Port-Write message handling
// - Added Machine Check exception handling
//
// Copyright (C) 2007, 2008, 2010, 2011 Freescale Semiconductor, Inc.
// Zhang Wei <wei.zhang@freescale.com>
// Lian Minghuan-B31939 <Minghuan.Lian@freescale.com>
// Liu Gang <Gang.Liu@freescale.com>
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

pub const RIO_MAINT_WIN_SIZE: c_uint = 0x400000;
pub const RIO_LTLEDCSR: c_uint = 0x0608;
pub const DOORBELL_ROWAR_EN: c_uint = 0x80000000;
pub const DOORBELL_ROWAR_TFLOWLV: c_uint = 0x08000000 /* highest priority level */;
pub const DOORBELL_ROWAR_PCI: c_uint = 0x02000000 /* PCI window */;
pub const DOORBELL_ROWAR_NREAD: c_uint = 0x00040000 /* NREAD */;
pub const DOORBELL_ROWAR_MAINTRD: c_uint = 0x00070000  /* maintenance read */;
pub const DOORBELL_ROWAR_RES: c_uint = 0x00002000 /* wrtpy: reserved */;
pub const DOORBELL_ROWAR_MAINTWD: c_uint = 0x00007000;
pub const DOORBELL_ROWAR_SIZE: c_uint = 0x0000000b /* window size is 4k */;
pub const RIO_ATMU_REGS_PORT1_OFFSET: c_uint = 0x10c00;
pub const RIO_ATMU_REGS_PORT2_OFFSET: c_uint = 0x10e00;
pub const RIO_S_DBELL_REGS_OFFSET: c_uint = 0x13400;
pub const RIO_S_PW_REGS_OFFSET: c_uint = 0x134e0;
pub const RIO_ATMU_REGS_DBELL_OFFSET: c_uint = 0x10C40;
pub const RIO_INB_ATMU_REGS_PORT1_OFFSET: c_uint = 0x10d60;
pub const RIO_INB_ATMU_REGS_PORT2_OFFSET: c_uint = 0x10f60;
pub const MAX_MSG_UNIT_NUM: c_int = 2;
pub const MAX_PORT_NUM: c_int = 4;
pub const RIO_INB_ATMU_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_atmu_regs {
    pub rowtar: u32,
    pub rowtear: u32,
    pub rowbar: u32,
    pub pad1: u32,
    pub rowar: u32,
    pub pad2: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_inb_atmu_regs {
    pub riwtar: u32,
    pub pad1: u32,
    pub riwbar: u32,
    pub pad2: u32,
    pub riwar: u32,
    pub pad3: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dbell_ring {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_port_write_msg {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
    pub msg_count: u32,
    pub err_count: u32,
    pub discard_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_rio_dbell {
    pub mport: [*mut rio_mport; MAX_PORT_NUM],
    pub dev: *mut device,
    pub dbell_regs: *mut rio_dbell_regs __iomem,
    pub dbell_ring: rio_dbell_ring,
    pub bellirq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_rio_pw {
    pub mport: [*mut rio_mport; MAX_PORT_NUM],
    pub dev: *mut device,
    pub pw_regs: *mut rio_pw_regs __iomem,
    pub port_write_msg: rio_port_write_msg,
    pub pwirq: c_int,
    pub pw_work: work_struct,
    pub pw_fifo: kfifo,
    pub pw_fifo_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_priv {
    pub dev: *mut device,
    pub regs_win: *mut void __iomem,
    pub atmu_regs: *mut rio_atmu_regs __iomem,
    pub maint_atmu_regs: *mut rio_atmu_regs __iomem,
    pub inb_atmu_regs: *mut rio_inb_atmu_regs __iomem,
    pub maint_win: *mut void __iomem,
    pub /: *mut *mut *mut void rmm_handle; / RapidIO message manager(unit) Handle,
}

extern "C" {
    pub fn fsl_rio_port_write_init(pw: *mut fsl_rio_pw) -> c_int;
}
extern "C" {
    pub fn fsl_rio_pw_enable(mport: *mut rio_mport, enable: c_int) -> c_int;
}
extern "C" {
    pub fn fsl_rio_port_error_handler(offset: c_int);
}
extern "C" {
    pub fn fsl_rio_doorbell_init(dbell: *mut fsl_rio_dbell) -> c_int;
}
extern "C" {
    pub fn fsl_close_outb_mbox(mport: *mut rio_mport, mbox: c_int);
}
extern "C" {
    pub fn fsl_close_inb_mbox(mport: *mut rio_mport, mbox: c_int);
}
extern "C" {
    pub fn fsl_add_inb_buffer(mport: *mut rio_mport, mbox: c_int, buf: *mut c_void) -> c_int;
}
