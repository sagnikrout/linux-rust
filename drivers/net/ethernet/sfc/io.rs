//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/io.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
//

//
// NIC register I/O
//
// The EF10 architecture exposes very few registers to the host and
// most of them are only 32 bits wide.  The only exceptions are the MC
// doorbell register pair, which has its own latching, and
// TX_DESC_UPD.
//
// The TX_DESC_UPD DMA descriptor pointer is 128-bits but is a special
// case in the BIU to avoid the need for locking in the host:
//
// - It is write-only.
// - The semantics of writing to this register is such that
// replacing the low 96 bits with zero does not affect functionality.
// - If the host writes to the last dword address of the register
// (i.e. the high 32 bits) the underlying register will always be
// written.  If the collector and the current write together do not
// provide values for all 128 bits of the register, the low 96 bits
// will be written as zero.
//

pub const EFX_USE_QWORD_IO: c_int = 1;

// Hardware issue requires that only 64-bit naturally aligned writes
// are seen by hardware. Its not strictly necessary to restrict to
// x86_64 arch, but done for safety since unusual write combining behaviour
// can break PIO.
//

// PIO is a win only if write-combining is possible

pub const EFX_USE_PIO: c_int = 1;

// Write a normal 128-bit CSR, locking as appropriate.
extern "C" {
    pub fn __attribute__(_arg: (unused)) -> unsigned long flags;
}

// Write a 32-bit CSR or the last dword of a special 128-bit CSR
// No lock required
// Read a 128-bit CSR, locking as appropriate.
extern "C" {
    pub fn __attribute__(_arg: (unused)) -> unsigned long flags;
}
// Read a 32-bit CSR or SRAM
// Write a 128-bit CSR forming part of a table
// Read a 128-bit CSR forming part of a table
// default VI stride (step between per-VI registers) is 8K on EF10 and
// 64K on EF100
//
pub const EFX_DEFAULT_VI_STRIDE: c_uint = 0x2000;
pub const EF100_DEFAULT_VI_STRIDE: c_uint = 0x10000;
// Calculate offset to page-mapped register
// Write the whole of RX_DESC_UPD or TX_DESC_UPD

// Write a page-mapped 32-bit CSR (EVQ_RPTR, EVQ_TMR (EF10), or the
// high bits of RX_DESC_UPD or TX_DESC_UPD)
//

