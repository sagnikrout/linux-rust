//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/hp_82335/hp82335.h
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
// copyright            : (C) 2002 by Frank Mori Hess
//

// struct which defines private_data for board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hp82335_priv {
    pub tms9914_priv: tms9914_priv,
    pub irq: c_uint,
    pub raw_iobase: c_ulong,
}

// size of io memory region used
// hp82335 register offsets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_read_regs {
    HPREG_CSR = 0x17f8,
    HPREG_STATUS = 0x1ffc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_write_regs {
    HPREG_INTR_CLEAR = 0x17f7,
    HPREG_CCR = HPREG_CSR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccr_bits {
    DMA_ENABLE = (1 << 0),   /* DMA enable                  */
    DMA_CHAN_SELECT = (1 << 1),   /* DMA channel select  O=3,1=2 */
    INTR_ENABLE = (1 << 2),   /* interrupt enable            */
    SYS_DISABLE = (1 << 3),   /* system controller disable   */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csr_bits {
    SWITCH6 = (1 << 0),   /* switch 6 position           */
    SWITCH5 = (1 << 1),   /* switch 5 position           */
    SYS_CONTROLLER = (1 << 2),   /* system controller bit       */
    DMA_ENABLE_STATUS = (1 << 4),   /* DMA enabled                 */
    DMA_CHAN_STATUS = (1 << 5),   /* DMA channel   0=3,1=2       */
    INTR_ENABLE_STATUS = (1 << 6),   /* Interrupt enable            */
    INTR_PENDING = (1 << 7),   /* Interrupt Pending           */
}
