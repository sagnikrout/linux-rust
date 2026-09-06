//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-gic-v3-prio.h
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
// GIC priorities from the view of the PMR/RPR.
//
// These values are chosen to be valid in either the absolute priority space or
// the NS view of the priority space. The value programmed into the distributor
// and ITS will be chosen at boot time such that these values appear in the
// PMR/RPR.
//
// GICV3_PRIO_UNMASKED is the PMR view of the priority to use to permit both
// IRQs and pseudo-NMIs.
//
// GICV3_PRIO_IRQ is the PMR view of the priority of regular interrupts. This
// can be written to the PMR to mask regular IRQs.
//
// GICV3_PRIO_NMI is the PMR view of the priority of pseudo-NMIs. This can be
// written to the PMR to mask pseudo-NMIs.
//
// On arm64 some code sections either automatically switch back to PSR.I or
// explicitly require to not use priority masking. If bit GICV3_PRIO_PSR_I_SET
// is included in the priority mask, it indicates that PSR.I should be set and
// interrupt disabling temporarily does not rely on IRQ priorities.
//
pub const GICV3_PRIO_UNMASKED: c_uint = 0xe0;
pub const GICV3_PRIO_IRQ: c_uint = 0xc0;
pub const GICV3_PRIO_NMI: c_uint = 0x80;

