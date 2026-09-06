//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pci_insn.h
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

// Load/Store status codes
pub const ZPCI_PCI_ST_FUNC_NOT_ENABLED: c_int = 4;
pub const ZPCI_PCI_ST_FUNC_IN_ERR: c_int = 8;
pub const ZPCI_PCI_ST_BLOCKED: c_int = 12;
pub const ZPCI_PCI_ST_INSUF_RES: c_int = 16;
pub const ZPCI_PCI_ST_INVAL_AS: c_int = 20;
pub const ZPCI_PCI_ST_FUNC_ALREADY_ENABLED: c_int = 24;
pub const ZPCI_PCI_ST_DMA_AS_NOT_ENABLED: c_int = 28;
pub const ZPCI_PCI_ST_2ND_OP_IN_INV_AS: c_int = 36;
pub const ZPCI_PCI_ST_FUNC_NOT_AVAIL: c_int = 40;
pub const ZPCI_PCI_ST_ALREADY_IN_RQ_STATE: c_int = 44;
// PCI instruction condition codes
pub const ZPCI_CC_OK: c_int = 0;
pub const ZPCI_CC_ERR: c_int = 1;
pub const ZPCI_CC_BUSY: c_int = 2;
pub const ZPCI_CC_INVAL_HANDLE: c_int = 3;
// Load/Store address space identifiers
pub const ZPCI_PCIAS_MEMIO_0: c_int = 0;
pub const ZPCI_PCIAS_MEMIO_1: c_int = 1;
pub const ZPCI_PCIAS_MEMIO_2: c_int = 2;
pub const ZPCI_PCIAS_MEMIO_3: c_int = 3;
pub const ZPCI_PCIAS_MEMIO_4: c_int = 4;
pub const ZPCI_PCIAS_MEMIO_5: c_int = 5;
pub const ZPCI_PCIAS_CFGSPC: c_int = 15;
// Modify PCI Function Controls
pub const ZPCI_MOD_FC_REG_INT: c_int = 2;
pub const ZPCI_MOD_FC_DEREG_INT: c_int = 3;
pub const ZPCI_MOD_FC_REG_IOAT: c_int = 4;
pub const ZPCI_MOD_FC_DEREG_IOAT: c_int = 5;
pub const ZPCI_MOD_FC_REREG_IOAT: c_int = 6;
pub const ZPCI_MOD_FC_RESET_ERROR: c_int = 7;
pub const ZPCI_MOD_FC_RESET_BLOCK: c_int = 9;
pub const ZPCI_MOD_FC_SET_MEASURE: c_int = 10;
pub const ZPCI_MOD_FC_REG_INT_D: c_int = 16;
pub const ZPCI_MOD_FC_DEREG_INT_D: c_int = 17;
// FIB function controls
pub const ZPCI_FIB_FC_ENABLED: c_uint = 0x80;
pub const ZPCI_FIB_FC_ERROR: c_uint = 0x40;
pub const ZPCI_FIB_FC_LS_BLOCKED: c_uint = 0x20;
pub const ZPCI_FIB_FC_DMAAS_REG: c_uint = 0x10;
// FIB function controls
pub const ZPCI_FIB_FC_ENABLED: c_uint = 0x80;
pub const ZPCI_FIB_FC_ERROR: c_uint = 0x40;
pub const ZPCI_FIB_FC_LS_BLOCKED: c_uint = 0x20;
pub const ZPCI_FIB_FC_DMAAS_REG: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fib_fmt0 {
    pub 1: u32 :,
    pub /: *mut *mut u32 isc : 3; / Interrupt subclass,
    pub /: *mut *mut u32 noi : 12; / Number of interrupts,
    pub 2: u32 :,
    pub /: *mut *mut u32 aibvo : 6; / Adapter interrupt bit vector offset,
    pub /: *mut *mut u32 sum : 1; / Adapter int summary bit enabled,
    pub 1: u32 :,
    pub /: *mut *mut u32 aisbo : 6; / Adapter int summary bit offset,
    pub 32: u32 :,
    pub /: *mut *mut u64 aibv; / Adapter int bit vector address,
    pub /: *mut *mut u64 aisb; / Adapter int summary bit address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fib_fmt1 {
    pub 4: u32 :,
    pub 12: u32 noi :,
    pub 16: u32 :,
    pub 16: u32 dibvo :,
    pub 16: u32 :,
    pub 64: u64 :,
    pub 64: u64 :,
}

// Function Information Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fib {
    pub /: *mut *mut u32 fmt : 8; / format,
    pub 24: u32 :,
    pub 32: u32 :,
    pub /: *mut *mut u8 fc; / function controls,
    pub 56: u64 :,
    pub /: *mut *mut u64 pba; / PCI base address,
    pub /: *mut *mut u64 pal; / PCI address limit,
    pub /: *mut *mut u64 iota; / I/O Translation Anchor,
    pub fmt0: zpci_fib_fmt0,
    pub fmt1: zpci_fib_fmt1,
}

// Set Interruption Controls Operation Controls
pub const SIC_IRQ_MODE_ALL: c_int = 0;
pub const SIC_IRQ_MODE_SINGLE: c_int = 1;
pub const SIC_SET_AENI_CONTROLS: c_int = 2;
pub const SIC_IRQ_MODE_DIRECT: c_int = 4;
pub const SIC_IRQ_MODE_D_ALL: c_int = 16;
pub const SIC_IRQ_MODE_D_SINGLE: c_int = 17;
pub const SIC_IRQ_MODE_SET_CPU: c_int = 18;
// directed interruption information block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_diib {
    pub 1: u32 :,
    pub 3: u32 isc :,
    pub 28: u32 :,
    pub 16: u16 :,
    pub nr_cpus: u16,
    pub disb_addr: u64,
    pub 64: u64 :,
    pub 64: u64 :,
    pub __aligned(8): } __packed,
// cpu directed interruption information block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_cdiib {
    pub 64: u64 :,
    pub dibv_addr: u64,
    pub 64: u64 :,
    pub 64: u64 :,
    pub 64: u64 :,
    pub __aligned(8): } __packed,
// adapter interruption parameters block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_aipb {
    pub faisb: u64,
    pub gait: u64,
    pub 13: u16 :,
    pub 3: u16 afi :,
    pub 32: u32 :,
    pub faal: u16,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union zpci_sic_iib {
    pub diib: zpci_diib,
    pub cdiib: zpci_cdiib,
    pub aipb: zpci_aipb,
}

extern "C" {
    pub fn zpci_mod_fc(req: u64, fib: *mut zpci_fib, status: *mut u8) -> u8;
}
extern "C" {
    pub fn zpci_refresh_trans(fn: u64, addr: u64, range: u64) -> c_int;
}
extern "C" {
    pub fn __zpci_load(data: *mut u64, req: u64, offset: u64) -> c_int;
}
extern "C" {
    pub fn zpci_load(data: *mut u64, addr: *const volatile void __iomem, len: c_ulong) -> c_int;
}
extern "C" {
    pub fn __zpci_store(data: u64, req: u64, offset: u64) -> c_int;
}
extern "C" {
    pub fn zpci_store(addr: *const volatile void __iomem, data: u64, len: c_ulong) -> c_int;
}
extern "C" {
    pub fn __zpci_store_block(data: *const u64, req: u64, offset: u64) -> c_int;
}
extern "C" {
    pub fn zpci_barrier();
}
extern "C" {
    pub fn zpci_set_irq_ctrl(ctl: u16, isc: u8, iib: *mut zpci_sic_iib) -> c_int;
}
