//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/bestcomm/bestcomm_priv.h
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


//
// Private header for the MPC52xx processor BestComm driver
//
// By private, we mean that driver should not use it directly. It's meant
// to be used by the BestComm engine driver itself and by the intermediate
// layer between the core and the drivers.
//
// Copyright (C) 2006      Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2005      Varma Electronics Oy,
// ( by Andrey Volkov <avolkov@varma-el.com> )
// Copyright (C) 2003-2004 MontaVista, Software, Inc.
// ( by Dale Farnsworth <dfarnsworth@mvista.com> )
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// ========================================================================
// Engine related stuff
// ========================================================================
// Zones sizes and needed alignments
pub const BCOM_MAX_TASKS: c_int = 16;
pub const BCOM_MAX_VAR: c_int = 24;
pub const BCOM_MAX_INC: c_int = 8;
pub const BCOM_MAX_FDT: c_int = 64;
pub const BCOM_MAX_CTX: c_int = 20;

pub const BCOM_CTX_ALIGN: c_uint = 0x100;

pub const BCOM_VAR_ALIGN: c_uint = 0x80;

pub const BCOM_FDT_ALIGN: c_uint = 0x100;
//
// struct bcom_tdt - Task Descriptor Table Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_tdt {
    pub start: u32,
    pub stop: u32,
    pub var: u32,
    pub fdt: u32,
    pub /: *mut *mut u32 exec_status; / used internally by BestComm engine,
    pub /: *mut *mut u32 mvtp; / used internally by BestComm engine,
    pub context: u32,
    pub litbase: u32,
}

//
// struct bcom_engine
//
// This holds all info needed globaly to handle the engine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_engine {
    pub ofnode: *mut device_node,
    pub regs: *mut mpc52xx_sdma __iomem,
    pub regs_base: phys_addr_t,
    pub tdt: *mut bcom_tdt,
    pub ctx: *mut u32,
    pub var: *mut u32,
    pub fdt: *mut u32,
    pub lock: spinlock_t,
}

// ========================================================================
// Tasks related stuff
// ========================================================================
// Tasks image header
pub const BCOM_TASK_MAGIC: c_uint = 0x4243544B	/* 'BCTK' */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_task_header {
    pub magic: u32,
    pub /: *mut *mut u8 desc_size; / the size fields,
    pub /: *mut *mut u8 var_size; / are given in number,
    pub /: *mut *mut u8 inc_size; / of 32-bits words,
    pub first_var: u8,
    pub reserved: [u8; 8],
}

// Descriptors structure & co
pub const BCOM_DESC_NOP: c_uint = 0x000001f8;
pub const BCOM_LCD_MASK: c_uint = 0x80000000;
pub const BCOM_DRD_EXTENDED: c_uint = 0x40000000;
pub const BCOM_DRD_INITIATOR_SHIFT: c_int = 21;
// Tasks pragma

// 1=iter end

// task enable

// 0=frac(msb), 1=int(lsb)

// Looks like XLB speculative read generates XLB errors when a buffer
// is at the end of the physical memory. i.e. when accessing the
// lasts words, the engine tries to prefetch the next but there is no
// next ...
//

// Initiators number
pub const BCOM_INITIATOR_ALWAYS: c_int = 0;
pub const BCOM_INITIATOR_SCTMR_0: c_int = 1;
pub const BCOM_INITIATOR_SCTMR_1: c_int = 2;
pub const BCOM_INITIATOR_FEC_RX: c_int = 3;
pub const BCOM_INITIATOR_FEC_TX: c_int = 4;
pub const BCOM_INITIATOR_ATA_RX: c_int = 5;
pub const BCOM_INITIATOR_ATA_TX: c_int = 6;
pub const BCOM_INITIATOR_SCPCI_RX: c_int = 7;
pub const BCOM_INITIATOR_SCPCI_TX: c_int = 8;
pub const BCOM_INITIATOR_PSC3_RX: c_int = 9;
pub const BCOM_INITIATOR_PSC3_TX: c_int = 10;
pub const BCOM_INITIATOR_PSC2_RX: c_int = 11;
pub const BCOM_INITIATOR_PSC2_TX: c_int = 12;
pub const BCOM_INITIATOR_PSC1_RX: c_int = 13;
pub const BCOM_INITIATOR_PSC1_TX: c_int = 14;
pub const BCOM_INITIATOR_SCTMR_2: c_int = 15;
pub const BCOM_INITIATOR_SCLPC: c_int = 16;
pub const BCOM_INITIATOR_PSC5_RX: c_int = 17;
pub const BCOM_INITIATOR_PSC5_TX: c_int = 18;
pub const BCOM_INITIATOR_PSC4_RX: c_int = 19;
pub const BCOM_INITIATOR_PSC4_TX: c_int = 20;
pub const BCOM_INITIATOR_I2C2_RX: c_int = 21;
pub const BCOM_INITIATOR_I2C2_TX: c_int = 22;
pub const BCOM_INITIATOR_I2C1_RX: c_int = 23;
pub const BCOM_INITIATOR_I2C1_TX: c_int = 24;
pub const BCOM_INITIATOR_PSC6_RX: c_int = 25;
pub const BCOM_INITIATOR_PSC6_TX: c_int = 26;
pub const BCOM_INITIATOR_IRDA_RX: c_int = 25;
pub const BCOM_INITIATOR_IRDA_TX: c_int = 26;
pub const BCOM_INITIATOR_SCTMR_3: c_int = 27;
pub const BCOM_INITIATOR_SCTMR_4: c_int = 28;
pub const BCOM_INITIATOR_SCTMR_5: c_int = 29;
pub const BCOM_INITIATOR_SCTMR_6: c_int = 30;
pub const BCOM_INITIATOR_SCTMR_7: c_int = 31;
// Initiators priorities
pub const BCOM_IPR_ALWAYS: c_int = 7;
pub const BCOM_IPR_SCTMR_0: c_int = 2;
pub const BCOM_IPR_SCTMR_1: c_int = 2;
pub const BCOM_IPR_FEC_RX: c_int = 6;
pub const BCOM_IPR_FEC_TX: c_int = 5;
pub const BCOM_IPR_ATA_RX: c_int = 7;
pub const BCOM_IPR_ATA_TX: c_int = 7;
pub const BCOM_IPR_SCPCI_RX: c_int = 2;
pub const BCOM_IPR_SCPCI_TX: c_int = 2;
pub const BCOM_IPR_PSC3_RX: c_int = 2;
pub const BCOM_IPR_PSC3_TX: c_int = 2;
pub const BCOM_IPR_PSC2_RX: c_int = 2;
pub const BCOM_IPR_PSC2_TX: c_int = 2;
pub const BCOM_IPR_PSC1_RX: c_int = 2;
pub const BCOM_IPR_PSC1_TX: c_int = 2;
pub const BCOM_IPR_SCTMR_2: c_int = 2;
pub const BCOM_IPR_SCLPC: c_int = 2;
pub const BCOM_IPR_PSC5_RX: c_int = 2;
pub const BCOM_IPR_PSC5_TX: c_int = 2;
pub const BCOM_IPR_PSC4_RX: c_int = 2;
pub const BCOM_IPR_PSC4_TX: c_int = 2;
pub const BCOM_IPR_I2C2_RX: c_int = 2;
pub const BCOM_IPR_I2C2_TX: c_int = 2;
pub const BCOM_IPR_I2C1_RX: c_int = 2;
pub const BCOM_IPR_I2C1_TX: c_int = 2;
pub const BCOM_IPR_PSC6_RX: c_int = 2;
pub const BCOM_IPR_PSC6_TX: c_int = 2;
pub const BCOM_IPR_IRDA_RX: c_int = 2;
pub const BCOM_IPR_IRDA_TX: c_int = 2;
pub const BCOM_IPR_SCTMR_3: c_int = 2;
pub const BCOM_IPR_SCTMR_4: c_int = 2;
pub const BCOM_IPR_SCTMR_5: c_int = 2;
pub const BCOM_IPR_SCTMR_6: c_int = 2;
pub const BCOM_IPR_SCTMR_7: c_int = 2;
// ========================================================================
// API
// ========================================================================
extern "C" {
    pub fn bcom_task_free(tsk: *mut bcom_task);
}
extern "C" {
    pub fn bcom_load_image(task: c_int, task_image: *mut u32) -> c_int;
}
extern "C" {
    pub fn bcom_set_initiator(task: c_int, initiator: c_int);
}
pub const TASK_ENABLE: c_uint = 0x8000;
//
// bcom_disable_prefetch - Hook to disable bus prefetching
//
// ATA DMA and the original MPC5200 need this due to silicon bugs.  At the
// moment disabling prefetch is a one-way street.  There is no mechanism
// in place to turn prefetch back on after it has been disabled.  There is
// no reason it couldn't be done, it would just be more complex to implement.
//
extern "C" {
    pub fn bcom_sram_pa2va(_arg: bcom_eng->tdt[task].start) -> return;
}
extern "C" {
    pub fn bcom_sram_pa2va(_arg: bcom_eng->tdt[task].var) -> return;
}
// desc = (*desc & ~(0x1f << BCOM_DRD_INITIATOR_SHIFT)) |
// fdt = (*fdt & ~0xff) | pragma;
