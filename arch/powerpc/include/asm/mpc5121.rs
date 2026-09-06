//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpc5121.h
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
// MPC5121 Prototypes and definitions
//
// MPC512x Reset module registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_reset_module {
    pub /: *mut *mut u32 rcwlr; / Reset Configuration Word Low Register,
    pub /: *mut *mut u32 rcwhr; / Reset Configuration Word High Register,
    pub reserved1: u32,
    pub reserved2: u32,
    pub /: *mut *mut u32 rsr; / Reset Status Register,
    pub /: *mut *mut u32 rmr; / Reset Mode Register,
    pub /: *mut *mut u32 rpr; / Reset Protection Register,
    pub /: *mut *mut u32 rcr; / Reset Control Register,
    pub /: *mut *mut u32 rcer; / Reset Control Enable Register,
}

//
// Clock Control Module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_ccm {
    pub /: *mut *mut u32 spmr; / System PLL Mode Register,
    pub /: *mut *mut u32 sccr1; / System Clock Control Register 1,
    pub /: *mut *mut u32 sccr2; / System Clock Control Register 2,
    pub /: *mut *mut u32 scfr1; / System Clock Frequency Register 1,
    pub /: *mut *mut u32 scfr2; / System Clock Frequency Register 2,
    pub /: *mut *mut u32 scfr2s; / System Clock Frequency Shadow Register 2,
    pub /: *mut *mut u32 bcr; / Bread Crumb Register,
    pub /: *mut *mut u32 psc_ccr[12]; / PSC Clock Control Registers,
    pub /: *mut *mut u32 spccr; / SPDIF Clock Control Register,
    pub /: *mut *mut u32 cccr; / CFM Clock Control Register,
    pub /: *mut *mut u32 dccr; / DIU Clock Control Register,
    pub /: *mut *mut u32 mscan_ccr[4]; / MSCAN Clock Control Registers,
    pub /: *mut *mut u32 out_ccr[4]; / OUT CLK Configure Registers,
    pub /: *mut *mut u32 rsv0[2]; / Reserved,
    pub /: *mut *mut u32 scfr3; / System Clock Frequency Register 3,
    pub /: *mut *mut u32 rsv1[3]; / Reserved,
    pub /: *mut *mut u32 spll_lock_cnt; / System PLL Lock Counter,
    pub /: *mut *mut u8 res[0x6c]; / Reserved,
}

//
// LPC Module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_lpc {
    pub /: *mut *mut u32 cs_cfg[8]; / CS config,
    pub /: *mut *mut u32 cs_ctrl; / CS Control Register,
    pub /: *mut *mut u32 cs_status; / CS Status Register,
    pub /: *mut *mut u32 burst_ctrl; / CS Burst Control Register,
    pub /: *mut *mut u32 deadcycle_ctrl; / CS Deadcycle Control Register,
    pub /: *mut *mut u32 holdcycle_ctrl; / CS Holdcycle Control Register,
    pub /: *mut *mut u32 alt; / Address Latch Timing Register,
}

extern "C" {
    pub fn mpc512x_cs_config(cs: c_uint, val: u32) -> c_int;
}
//
// SCLPC Module (LPB FIFO)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_lpbfifo {
    pub /: *mut *mut u32 pkt_size; / SCLPC Packet Size Register,
    pub /: *mut *mut u32 start_addr; / SCLPC Start Address Register,
    pub /: *mut *mut u32 ctrl; / SCLPC Control Register,
    pub /: *mut *mut u32 enable; / SCLPC Enable Register,
    pub reserved1: u32,
    pub /: *mut *mut u32 status; / SCLPC Status Register,
    pub /: *mut *mut u32 bytes_done; / SCLPC Bytes Done Register,
    pub /: *mut *mut u32 emb_sc; / EMB Share Counter Register,
    pub /: *mut *mut u32 emb_pc; / EMB Pause Control Register,
    pub reserved2: [u32; 7],
    pub /: *mut *mut u32 data_word; / LPC RX/TX FIFO Data Word Register,
    pub /: *mut *mut u32 fifo_status; / LPC RX/TX FIFO Status Register,
    pub /: *mut *mut u32 fifo_ctrl; / LPC RX/TX FIFO Control Register,
    pub /: *mut *mut u32 fifo_alarm; / LPC RX/TX FIFO Alarm Register,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpb_dev_portsize {
    LPB_DEV_PORTSIZE_UNDEFINED = 0,
    LPB_DEV_PORTSIZE_1_BYTE = 1,
    LPB_DEV_PORTSIZE_2_BYTES = 2,
    LPB_DEV_PORTSIZE_4_BYTES = 4,
    LPB_DEV_PORTSIZE_8_BYTES = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpc512x_lpbfifo_req_dir {
    MPC512X_LPBFIFO_REQ_DIR_READ,
    MPC512X_LPBFIFO_REQ_DIR_WRITE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_lpbfifo_request {
    pub /: *mut *mut phys_addr_t dev_phys_addr; / physical address of some device on LPB,
    pub /: *mut *mut *mut void ram_virt_addr; / virtual address of some region in RAM,
    pub size: u32,
    pub portsize: lpb_dev_portsize,
    pub dir: mpc512x_lpbfifo_req_dir,
    pub ): *mut *mut void (callback)(struct mpc512x_lpbfifo_request,
}

extern "C" {
    pub fn mpc512x_lpbfifo_submit(req: *mut mpc512x_lpbfifo_request) -> c_int;
}
