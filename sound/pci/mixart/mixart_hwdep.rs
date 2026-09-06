//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/mixart/mixart_hwdep.h
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
// Driver for Digigram miXart soundcards
//
// definitions and makros for basic card access
//
// Copyright (c) 2003 by Digigram <alsa@digigram.com>
//

// Daughter board Type
pub const DAUGHTER_TYPE_MASK: c_uint = 0x0F;
pub const DAUGHTER_VER_MASK: c_uint = 0xF0;

pub const MIXART_DAUGHTER_TYPE_NONE: c_uint = 0x00;
pub const MIXART_DAUGHTER_TYPE_COBRANET: c_uint = 0x08;
pub const MIXART_DAUGHTER_TYPE_AES: c_uint = 0x0E;

//
// -----------BAR 0 --------------------------------------------------------------------------------------------------------
//
pub const MIXART_PSEUDOREG: c_uint = 0x2000                    /* base address for pseudoregister */;

// perfmeter (available when elf loaded)

// motherboard xilinx loader info

// elf loader info

//
// after the elf code is loaded, and the flowtable info was passed to it,
// the driver polls on this address, until it shows 1 (presence) or 2 (absence)
// once it is non-zero, the daughter board type may be read
//

// Global info structure

// daughterboard xilinx loader info

//
pub const MIXART_FLOWTABLE_PTR: c_uint = 0x3000                    /* pointer to flow table */;
// mailbox addresses
// message DRV -> EMB
pub const MSG_INBOUND_POST_HEAD: c_uint = 0x010008	/* DRV posts MF + increment4 */;
pub const MSG_INBOUND_POST_TAIL: c_uint = 0x01000C	/* EMB gets MF + increment4 */;
// message EMB -> DRV
pub const MSG_OUTBOUND_POST_TAIL: c_uint = 0x01001C	/* DRV gets MF + increment4 */;
pub const MSG_OUTBOUND_POST_HEAD: c_uint = 0x010018	/* EMB posts MF + increment4 */;
// Get Free Frames
pub const MSG_INBOUND_FREE_TAIL: c_uint = 0x010004	/* DRV gets MFA + increment4 */;
pub const MSG_OUTBOUND_FREE_TAIL: c_uint = 0x010014	/* EMB gets MFA + increment4 */;
// Put Free Frames
pub const MSG_OUTBOUND_FREE_HEAD: c_uint = 0x010010	/* DRV puts MFA + increment4 */;
pub const MSG_INBOUND_FREE_HEAD: c_uint = 0x010000    /* EMB puts MFA + increment4 */;
// firmware addresses of the message fifos
pub const MSG_BOUND_STACK_SIZE: c_uint = 0x004000    /* size of each following stack */;
// posted messages
pub const MSG_OUTBOUND_POST_STACK: c_uint = 0x108000    /* stack of messages to the DRV */;
pub const MSG_INBOUND_POST_STACK: c_uint = 0x104000    /* stack of messages to the EMB */;
// available empty messages
pub const MSG_OUTBOUND_FREE_STACK: c_uint = 0x10C000    /* stack of free enveloped for EMB */;
pub const MSG_INBOUND_FREE_STACK: c_uint = 0x100000    /* stack of free enveloped for DRV */;
// defines for mailbox message frames
pub const MSG_FRAME_OFFSET: c_uint = 0x64;
pub const MSG_FRAME_SIZE: c_uint = 0x6400;
pub const MSG_FRAME_NUMBER: c_int = 32;

//
// -----------BAR 1 --------------------------------------------------------------------------------------------------------
//
// interrupt addresses and constants
pub const MIXART_PCI_OMIMR_OFFSET: c_uint = 0x34    /* outbound message interrupt mask register */;
pub const MIXART_PCI_OMISR_OFFSET: c_uint = 0x30    /* outbound message interrupt status register */;
pub const MIXART_PCI_ODBR_OFFSET: c_uint = 0x60    /* outbound doorbell register */;
pub const MIXART_BA1_BRUTAL_RESET_OFFSET: c_uint = 0x68    /* write 1 in LSBit to reset board */;
pub const MIXART_HOST_ALL_INTERRUPT_MASKED: c_uint = 0x02B   /* 0000 0010 1011 */;
pub const MIXART_ALLOW_OUTBOUND_DOORBELL: c_uint = 0x023   /* 0000 0010 0011 */;
pub const MIXART_OIDI: c_uint = 0x008   /* 0000 0000 1000 */;
extern "C" {
    pub fn snd_mixart_setup_firmware(mgr: *mut mixart_mgr) -> c_int;
}
