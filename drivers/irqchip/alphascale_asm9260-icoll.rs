//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/alphascale_asm9260-icoll.h
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
// Copyright (C) 2014 Oleksij Rempel <linux@rempel-privat.de>
//
pub const ASM9260_NUM_IRQS: c_int = 64;
//
// this device provide 4 offsets for each register:
// 0x0 - plain read write mode
// 0x4 - set mode, OR logic.
// 0x8 - clr mode, XOR logic.
// 0xc - togle mode.
//
pub const ASM9260_HW_ICOLL_VECTOR: c_uint = 0x0000;
//
// bits 31:2
// This register presents the vector address for the interrupt currently
// active on the CPU IRQ input. Writing to this register notifies the
// interrupt collector that the interrupt service routine for the current
// interrupt has been entered.
// The exception trap should have a LDPC instruction from this address:
// LDPC ASM9260_HW_ICOLL_VECTOR_ADDR; IRQ exception at 0xffff0018
//
// The Interrupt Collector Level Acknowledge Register is used by software to
// indicate the completion of an interrupt on a specific level.
// This register is written at the very end of an interrupt service routine. If
// nesting is used then the CPU irq must be turned on before writing to this
// register to avoid a race condition in the CPU interrupt hardware.
//
pub const ASM9260_HW_ICOLL_LEVELACK: c_uint = 0x0010;

pub const ASM9260_HW_ICOLL_CTRL: c_uint = 0x0020;
//
// ASM9260_BM_CTRL_SFTRST and ASM9260_BM_CTRL_CLKGATE are not available on
// asm9260.
//

// disable interrupt level nesting

//
// Set this bit to one enable the RISC32-style read side effect associated with
// the vector address register. In this mode, interrupt in-service is signaled
// by the read of the ASM9260_HW_ICOLL_VECTOR register to acquire the interrupt
// vector address. Set this bit to zero for normal operation, in which the ISR
// signals in-service explicitly by means of a write to the
// ASM9260_HW_ICOLL_VECTOR register.
// 0 - Must Write to Vector register to go in-service.
// 1 - Go in-service as a read side effect
//

pub const ASM9260_HW_ICOLL_STAT_OFFSET: c_uint = 0x0030;
//
// bits 5:0
// Vector number of current interrupt. Multiply by 4 and add to vector base
// address to obtain the value in ASM9260_HW_ICOLL_VECTOR.
//
// RAW0 and RAW1 provides a read-only view of the raw interrupt request lines
// coming from various parts of the chip. Its purpose is to improve diagnostic
// observability.
//
pub const ASM9260_HW_ICOLL_RAW0: c_uint = 0x0040;
pub const ASM9260_HW_ICOLL_RAW1: c_uint = 0x0050;
pub const ASM9260_HW_ICOLL_INTERRUPT0: c_uint = 0x0060;

//
// WARNING: Modifying the priority of an enabled interrupt may result in
// undefined behavior.
//
pub const ASM9260_BM_INT_PRIORITY_MASK: c_uint = 0x3;

pub const ASM9260_HW_ICOLL_VBASE: c_uint = 0x0160;
//
// bits 31:2
// This bitfield holds the upper 30 bits of the base address of the vector
// table.
//
pub const ASM9260_HW_ICOLL_CLEAR0: c_uint = 0x01d0;
pub const ASM9260_HW_ICOLL_CLEAR1: c_uint = 0x01e0;

// Scratchpad
pub const ASM9260_HW_ICOLL_UNDEF_VECTOR: c_uint = 0x01f0;
