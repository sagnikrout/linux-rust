//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/seeq/sgiseeq.h
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
// sgiseeq.h: Defines for the Seeq8003 ethernet controller.
//
// Copyright (C) 1996 David S. Miller (davem@davemloft.net)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgiseeq_wregs {
    pub multicase_high: [volatile unsigned int; 2],
    pub frame_gap: volatile unsigned int,
    pub control: volatile unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgiseeq_rregs {
    pub collision_tx: [volatile unsigned int; 2],
    pub collision_all: [volatile unsigned int; 2],
    pub _unused0: volatile unsigned int,
    pub rflags: volatile unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgiseeq_regs {
    pub eth_addr: [volatile unsigned int; 6],
    pub multicast_low: [volatile unsigned int; 6],
    pub wregs: sgiseeq_wregs,
    pub rregs: sgiseeq_rregs,
    pub rw: },
    pub rstat: volatile unsigned int,
    pub tstat: volatile unsigned int,
}

// Seeq8003 receive status register
pub const SEEQ_RSTAT_OVERF: c_uint = 0x001 /* Overflow */;
pub const SEEQ_RSTAT_CERROR: c_uint = 0x002 /* CRC error */;
pub const SEEQ_RSTAT_DERROR: c_uint = 0x004 /* Dribble error */;
pub const SEEQ_RSTAT_SFRAME: c_uint = 0x008 /* Short frame */;
pub const SEEQ_RSTAT_REOF: c_uint = 0x010 /* Received end of frame */;
pub const SEEQ_RSTAT_FIG: c_uint = 0x020 /* Frame is good */;
pub const SEEQ_RSTAT_TIMEO: c_uint = 0x040 /* Timeout, or late receive */;
pub const SEEQ_RSTAT_WHICH: c_uint = 0x080 /* Which status, 1=old 0=new */;
pub const SEEQ_RSTAT_LITTLE: c_uint = 0x100 /* DMA is done in little endian format */;
pub const SEEQ_RSTAT_SDMA: c_uint = 0x200 /* DMA has started */;
pub const SEEQ_RSTAT_ADMA: c_uint = 0x400 /* DMA is active */;
pub const SEEQ_RSTAT_ROVERF: c_uint = 0x800 /* Receive buffer overflow */;
// Seeq8003 receive command register
pub const SEEQ_RCMD_RDISAB: c_uint = 0x000 /* Disable receiver on the Seeq8003 */;
pub const SEEQ_RCMD_IOVERF: c_uint = 0x001 /* IRQ on buffer overflows */;
pub const SEEQ_RCMD_ICRC: c_uint = 0x002 /* IRQ on CRC errors */;
pub const SEEQ_RCMD_IDRIB: c_uint = 0x004 /* IRQ on dribble errors */;
pub const SEEQ_RCMD_ISHORT: c_uint = 0x008 /* IRQ on short frames */;
pub const SEEQ_RCMD_IEOF: c_uint = 0x010 /* IRQ on end of frame */;
pub const SEEQ_RCMD_IGOOD: c_uint = 0x020 /* IRQ on good frames */;
pub const SEEQ_RCMD_RANY: c_uint = 0x040 /* Receive any frame */;
pub const SEEQ_RCMD_RBCAST: c_uint = 0x080 /* Receive broadcasts */;
pub const SEEQ_RCMD_RBMCAST: c_uint = 0x0c0 /* Receive broadcasts/multicasts */;
// Seeq8003 transmit status register
pub const SEEQ_TSTAT_UFLOW: c_uint = 0x001 /* Transmit buffer underflow */;
pub const SEEQ_TSTAT_CLS: c_uint = 0x002 /* Collision detected */;
pub const SEEQ_TSTAT_R16: c_uint = 0x004 /* Did 16 retries to tx a frame */;
pub const SEEQ_TSTAT_PTRANS: c_uint = 0x008 /* Packet was transmitted ok */;
pub const SEEQ_TSTAT_LCLS: c_uint = 0x010 /* Late collision occurred */;
pub const SEEQ_TSTAT_WHICH: c_uint = 0x080 /* Which status, 1=old 0=new */;
pub const SEEQ_TSTAT_TLE: c_uint = 0x100 /* DMA is done in little endian format */;
pub const SEEQ_TSTAT_SDMA: c_uint = 0x200 /* DMA has started */;
pub const SEEQ_TSTAT_ADMA: c_uint = 0x400 /* DMA is active */;
// Seeq8003 transmit command register
pub const SEEQ_TCMD_RB0: c_uint = 0x00 /* Register bank zero w/station addr */;
pub const SEEQ_TCMD_IUF: c_uint = 0x01 /* IRQ on tx underflow */;
pub const SEEQ_TCMD_IC: c_uint = 0x02 /* IRQ on collisions */;
pub const SEEQ_TCMD_I16: c_uint = 0x04 /* IRQ after 16 failed attempts to tx frame */;
pub const SEEQ_TCMD_IPT: c_uint = 0x08 /* IRQ when packet successfully transmitted */;
pub const SEEQ_TCMD_RB1: c_uint = 0x20 /* Register bank one w/multi-cast low byte */;
pub const SEEQ_TCMD_RB2: c_uint = 0x40 /* Register bank two w/multi-cast high byte */;
// Seeq8003 control register
pub const SEEQ_CTRL_XCNT: c_uint = 0x01;
pub const SEEQ_CTRL_ACCNT: c_uint = 0x02;
pub const SEEQ_CTRL_SFLAG: c_uint = 0x04;
pub const SEEQ_CTRL_EMULTI: c_uint = 0x08;
pub const SEEQ_CTRL_ESHORT: c_uint = 0x10;
pub const SEEQ_CTRL_ENCARR: c_uint = 0x20;
// Seeq8003 control registers on the SGI Hollywood HPC.
pub const SEEQ_HPIO_P1BITS: c_uint = 0x00000001 /* cycles to stay in P1 phase for PIO */;
pub const SEEQ_HPIO_P2BITS: c_uint = 0x00000060 /* cycles to stay in P2 phase for PIO */;
pub const SEEQ_HPIO_P3BITS: c_uint = 0x00000100 /* cycles to stay in P3 phase for PIO */;
pub const SEEQ_HDMA_D1BITS: c_uint = 0x00000006 /* cycles to stay in D1 phase for DMA */;
pub const SEEQ_HDMA_D2BITS: c_uint = 0x00000020 /* cycles to stay in D2 phase for DMA */;
pub const SEEQ_HDMA_D3BITS: c_uint = 0x00000000 /* cycles to stay in D3 phase for DMA */;
pub const SEEQ_HDMA_TIMEO: c_uint = 0x00030000 /* cycles for DMA timeout */;
pub const SEEQ_HCTL_NORM: c_uint = 0x00000000 /* Normal operation mode */;
pub const SEEQ_HCTL_RESET: c_uint = 0x00000001 /* Reset Seeq8003 and HPC interface */;
pub const SEEQ_HCTL_IPEND: c_uint = 0x00000002 /* IRQ is pending for the chip */;
pub const SEEQ_HCTL_IPG: c_uint = 0x00001000 /* Inter-packet gap */;
pub const SEEQ_HCTL_RFIX: c_uint = 0x00002000 /* At rxdc, clear end-of-packet */;
pub const SEEQ_HCTL_EFIX: c_uint = 0x00004000 /* fixes intr status bit settings */;
pub const SEEQ_HCTL_IFIX: c_uint = 0x00008000 /* enable startup timeouts */;
