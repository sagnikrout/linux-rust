//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-scb.h
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
// cx18 System Control Block initialization
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

// NOTE: All ACK interrupts are in the SW2 register.  All non-ACK interrupts
pub const IRQ_APU_TO_CPU: c_uint = 0x00000001;
pub const IRQ_CPU_TO_APU_ACK: c_uint = 0x00000001;
pub const IRQ_HPU_TO_CPU: c_uint = 0x00000002;
pub const IRQ_CPU_TO_HPU_ACK: c_uint = 0x00000002;
pub const IRQ_PPU_TO_CPU: c_uint = 0x00000004;
pub const IRQ_CPU_TO_PPU_ACK: c_uint = 0x00000004;
pub const IRQ_EPU_TO_CPU: c_uint = 0x00000008;
pub const IRQ_CPU_TO_EPU_ACK: c_uint = 0x00000008;
pub const IRQ_CPU_TO_APU: c_uint = 0x00000010;
pub const IRQ_APU_TO_CPU_ACK: c_uint = 0x00000010;
pub const IRQ_HPU_TO_APU: c_uint = 0x00000020;
pub const IRQ_APU_TO_HPU_ACK: c_uint = 0x00000020;
pub const IRQ_PPU_TO_APU: c_uint = 0x00000040;
pub const IRQ_APU_TO_PPU_ACK: c_uint = 0x00000040;
pub const IRQ_EPU_TO_APU: c_uint = 0x00000080;
pub const IRQ_APU_TO_EPU_ACK: c_uint = 0x00000080;
pub const IRQ_CPU_TO_HPU: c_uint = 0x00000100;
pub const IRQ_HPU_TO_CPU_ACK: c_uint = 0x00000100;
pub const IRQ_APU_TO_HPU: c_uint = 0x00000200;
pub const IRQ_HPU_TO_APU_ACK: c_uint = 0x00000200;
pub const IRQ_PPU_TO_HPU: c_uint = 0x00000400;
pub const IRQ_HPU_TO_PPU_ACK: c_uint = 0x00000400;
pub const IRQ_EPU_TO_HPU: c_uint = 0x00000800;
pub const IRQ_HPU_TO_EPU_ACK: c_uint = 0x00000800;
pub const IRQ_CPU_TO_PPU: c_uint = 0x00001000;
pub const IRQ_PPU_TO_CPU_ACK: c_uint = 0x00001000;
pub const IRQ_APU_TO_PPU: c_uint = 0x00002000;
pub const IRQ_PPU_TO_APU_ACK: c_uint = 0x00002000;
pub const IRQ_HPU_TO_PPU: c_uint = 0x00004000;
pub const IRQ_PPU_TO_HPU_ACK: c_uint = 0x00004000;
pub const IRQ_EPU_TO_PPU: c_uint = 0x00008000;
pub const IRQ_PPU_TO_EPU_ACK: c_uint = 0x00008000;
pub const IRQ_CPU_TO_EPU: c_uint = 0x00010000;
pub const IRQ_EPU_TO_CPU_ACK: c_uint = 0x00010000;
pub const IRQ_APU_TO_EPU: c_uint = 0x00020000;
pub const IRQ_EPU_TO_APU_ACK: c_uint = 0x00020000;
pub const IRQ_HPU_TO_EPU: c_uint = 0x00040000;
pub const IRQ_EPU_TO_HPU_ACK: c_uint = 0x00040000;
pub const IRQ_PPU_TO_EPU: c_uint = 0x00080000;
pub const IRQ_EPU_TO_PPU_ACK: c_uint = 0x00080000;
pub const SCB_OFFSET: c_uint = 0xDC0000;
// If Firmware uses fixed memory map, it shall not allocate the area
pub const SCB_RESERVED_SIZE: c_uint = 0x10000;
// This structure is used by EPU to provide memory descriptors in its memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_mdl_ent {
    pub /: *mut *mut u32 paddr; / Physical address of a buffer segment,
    pub /: *mut *mut u32 length; / Length of the buffer segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_scb {
// These fields form the System Control Block which is used at boot time
// Offset where to find the Inter-Processor Communication data
    pub ipc_offset: u32,
    pub reserved01: [u32; 7],
// Offset where to find the start of the CPU code
    pub cpu_code_offset: u32,
    pub reserved02: [u32; 3],
// Offset where to find the start of the APU code
    pub apu_code_offset: u32,
    pub reserved03: [u32; 3],
// Offset where to find the start of the HPU code
    pub hpu_code_offset: u32,
    pub reserved04: [u32; 3],
// Offset where to find the start of the PPU code
    pub ppu_code_offset: u32,
    pub reserved05: [u32; 3],
// These fields form Inter-Processor Communication data which is used
// Fields for CPU:
// bit 0: 1/0 processor ready/not ready. Set other bits to 0.
    pub cpu_state: u32,
    pub reserved1: [u32; 7],
// Offset to the mailbox used for sending commands from APU to CPU
    pub apu2cpu_mb_offset: u32,
// Value to write to register SW1 register set (0xC7003100) after the
    pub apu2cpu_irq: u32,
// Value to write to register SW2 register set (0xC7003140) after the
    pub cpu2apu_irq_ack: u32,
    pub reserved2: [u32; 13],
    pub hpu2cpu_mb_offset: u32,
    pub hpu2cpu_irq: u32,
    pub cpu2hpu_irq_ack: u32,
    pub reserved3: [u32; 13],
    pub ppu2cpu_mb_offset: u32,
    pub ppu2cpu_irq: u32,
    pub cpu2ppu_irq_ack: u32,
    pub reserved4: [u32; 13],
    pub epu2cpu_mb_offset: u32,
    pub epu2cpu_irq: u32,
    pub cpu2epu_irq_ack: u32,
    pub reserved5: [u32; 13],
    pub reserved6: [u32; 8],
// Fields for APU:
    pub apu_state: u32,
    pub reserved11: [u32; 7],
    pub cpu2apu_mb_offset: u32,
    pub cpu2apu_irq: u32,
    pub apu2cpu_irq_ack: u32,
    pub reserved12: [u32; 13],
    pub hpu2apu_mb_offset: u32,
    pub hpu2apu_irq: u32,
    pub apu2hpu_irq_ack: u32,
    pub reserved13: [u32; 13],
    pub ppu2apu_mb_offset: u32,
    pub ppu2apu_irq: u32,
    pub apu2ppu_irq_ack: u32,
    pub reserved14: [u32; 13],
    pub epu2apu_mb_offset: u32,
    pub epu2apu_irq: u32,
    pub apu2epu_irq_ack: u32,
    pub reserved15: [u32; 13],
    pub reserved16: [u32; 8],
// Fields for HPU:
    pub hpu_state: u32,
    pub reserved21: [u32; 7],
    pub cpu2hpu_mb_offset: u32,
    pub cpu2hpu_irq: u32,
    pub hpu2cpu_irq_ack: u32,
    pub reserved22: [u32; 13],
    pub apu2hpu_mb_offset: u32,
    pub apu2hpu_irq: u32,
    pub hpu2apu_irq_ack: u32,
    pub reserved23: [u32; 13],
    pub ppu2hpu_mb_offset: u32,
    pub ppu2hpu_irq: u32,
    pub hpu2ppu_irq_ack: u32,
    pub reserved24: [u32; 13],
    pub epu2hpu_mb_offset: u32,
    pub epu2hpu_irq: u32,
    pub hpu2epu_irq_ack: u32,
    pub reserved25: [u32; 13],
    pub reserved26: [u32; 8],
// Fields for PPU:
    pub ppu_state: u32,
    pub reserved31: [u32; 7],
    pub cpu2ppu_mb_offset: u32,
    pub cpu2ppu_irq: u32,
    pub ppu2cpu_irq_ack: u32,
    pub reserved32: [u32; 13],
    pub apu2ppu_mb_offset: u32,
    pub apu2ppu_irq: u32,
    pub ppu2apu_irq_ack: u32,
    pub reserved33: [u32; 13],
    pub hpu2ppu_mb_offset: u32,
    pub hpu2ppu_irq: u32,
    pub ppu2hpu_irq_ack: u32,
    pub reserved34: [u32; 13],
    pub epu2ppu_mb_offset: u32,
    pub epu2ppu_irq: u32,
    pub ppu2epu_irq_ack: u32,
    pub reserved35: [u32; 13],
    pub reserved36: [u32; 8],
// Fields for EPU:
    pub epu_state: u32,
    pub reserved41: [u32; 7],
    pub cpu2epu_mb_offset: u32,
    pub cpu2epu_irq: u32,
    pub epu2cpu_irq_ack: u32,
    pub reserved42: [u32; 13],
    pub apu2epu_mb_offset: u32,
    pub apu2epu_irq: u32,
    pub epu2apu_irq_ack: u32,
    pub reserved43: [u32; 13],
    pub hpu2epu_mb_offset: u32,
    pub hpu2epu_irq: u32,
    pub epu2hpu_irq_ack: u32,
    pub reserved44: [u32; 13],
    pub ppu2epu_mb_offset: u32,
    pub ppu2epu_irq: u32,
    pub epu2ppu_irq_ack: u32,
    pub reserved45: [u32; 13],
    pub reserved46: [u32; 8],
    pub /: *mut *mut u32 semaphores[8]; / Semaphores,
    pub /: *mut *mut u32 reserved50[32]; / Reserved for future use,
    pub apu2cpu_mb: cx18_mailbox,
    pub hpu2cpu_mb: cx18_mailbox,
    pub ppu2cpu_mb: cx18_mailbox,
    pub epu2cpu_mb: cx18_mailbox,
    pub cpu2apu_mb: cx18_mailbox,
    pub hpu2apu_mb: cx18_mailbox,
    pub ppu2apu_mb: cx18_mailbox,
    pub epu2apu_mb: cx18_mailbox,
    pub cpu2hpu_mb: cx18_mailbox,
    pub apu2hpu_mb: cx18_mailbox,
    pub ppu2hpu_mb: cx18_mailbox,
    pub epu2hpu_mb: cx18_mailbox,
    pub cpu2ppu_mb: cx18_mailbox,
    pub apu2ppu_mb: cx18_mailbox,
    pub hpu2ppu_mb: cx18_mailbox,
    pub epu2ppu_mb: cx18_mailbox,
    pub cpu2epu_mb: cx18_mailbox,
    pub apu2epu_mb: cx18_mailbox,
    pub hpu2epu_mb: cx18_mailbox,
    pub ppu2epu_mb: cx18_mailbox,
    pub cpu_mdl_ack: [cx18_mdl_ack; CX18_MAX_STREAMS][CX18_MAX_MDL_ACKS],
    pub cpu_mdl: [cx18_mdl_ent; ],
}

extern "C" {
    pub fn cx18_init_scb(cx: *mut cx18);
}
