//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/sja1000/sja1000.h
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
// sja1000.h -  Philips SJA1000 network device driver
//
// Copyright (c) 2003 Matthias Brukner, Trajet Gmbh, Rebenring 33,
// 38106 Braunschweig, GERMANY
//
// Copyright (c) 2002-2007 Volkswagen Group Electronic Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

// SJA1000 registers - manual section 6.4 (Pelican Mode)
pub const SJA1000_MOD: c_uint = 0x00;
pub const SJA1000_CMR: c_uint = 0x01;
pub const SJA1000_SR: c_uint = 0x02;
pub const SJA1000_IR: c_uint = 0x03;
pub const SJA1000_IER: c_uint = 0x04;
pub const SJA1000_ALC: c_uint = 0x0B;
pub const SJA1000_ECC: c_uint = 0x0C;
pub const SJA1000_EWL: c_uint = 0x0D;
pub const SJA1000_RXERR: c_uint = 0x0E;
pub const SJA1000_TXERR: c_uint = 0x0F;
pub const SJA1000_ACCC0: c_uint = 0x10;
pub const SJA1000_ACCC1: c_uint = 0x11;
pub const SJA1000_ACCC2: c_uint = 0x12;
pub const SJA1000_ACCC3: c_uint = 0x13;
pub const SJA1000_ACCM0: c_uint = 0x14;
pub const SJA1000_ACCM1: c_uint = 0x15;
pub const SJA1000_ACCM2: c_uint = 0x16;
pub const SJA1000_ACCM3: c_uint = 0x17;
pub const SJA1000_RMC: c_uint = 0x1D;
pub const SJA1000_RBSA: c_uint = 0x1E;
// Common registers - manual section 6.5
pub const SJA1000_BTR0: c_uint = 0x06;
pub const SJA1000_BTR1: c_uint = 0x07;
pub const SJA1000_OCR: c_uint = 0x08;
pub const SJA1000_CDR: c_uint = 0x1F;
pub const SJA1000_FI: c_uint = 0x10;
pub const SJA1000_SFF_BUF: c_uint = 0x13;
pub const SJA1000_EFF_BUF: c_uint = 0x15;
pub const SJA1000_FI_FF: c_uint = 0x80;
pub const SJA1000_FI_RTR: c_uint = 0x40;
pub const SJA1000_ID1: c_uint = 0x11;
pub const SJA1000_ID2: c_uint = 0x12;
pub const SJA1000_ID3: c_uint = 0x13;
pub const SJA1000_ID4: c_uint = 0x14;
pub const SJA1000_CAN_RAM: c_uint = 0x20;
// mode register
pub const MOD_RM: c_uint = 0x01;
pub const MOD_LOM: c_uint = 0x02;
pub const MOD_STM: c_uint = 0x04;
pub const MOD_AFM: c_uint = 0x08;
pub const MOD_SM: c_uint = 0x10;
// commands
pub const CMD_SRR: c_uint = 0x10;
pub const CMD_CDO: c_uint = 0x08;
pub const CMD_RRB: c_uint = 0x04;
pub const CMD_AT: c_uint = 0x02;
pub const CMD_TR: c_uint = 0x01;
// interrupt sources
pub const IRQ_BEI: c_uint = 0x80;
pub const IRQ_ALI: c_uint = 0x40;
pub const IRQ_EPI: c_uint = 0x20;
pub const IRQ_WUI: c_uint = 0x10;
pub const IRQ_DOI: c_uint = 0x08;
pub const IRQ_EI: c_uint = 0x04;
pub const IRQ_TI: c_uint = 0x02;
pub const IRQ_RI: c_uint = 0x01;
pub const IRQ_ALL: c_uint = 0xFF;
pub const IRQ_OFF: c_uint = 0x00;
// status register content
pub const SR_BS: c_uint = 0x80;
pub const SR_ES: c_uint = 0x40;
pub const SR_TS: c_uint = 0x20;
pub const SR_RS: c_uint = 0x10;
pub const SR_TCS: c_uint = 0x08;
pub const SR_TBS: c_uint = 0x04;
pub const SR_DOS: c_uint = 0x02;
pub const SR_RBS: c_uint = 0x01;

// ECC register
pub const ECC_SEG: c_uint = 0x1F;
pub const ECC_DIR: c_uint = 0x20;
pub const ECC_ERR: c_int = 6;
pub const ECC_BIT: c_uint = 0x00;
pub const ECC_FORM: c_uint = 0x40;
pub const ECC_STUFF: c_uint = 0x80;
pub const ECC_MASK: c_uint = 0xc0;
//
// Flags for sja1000priv.flags
//

//
// SJA1000 private data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1000_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub echo_skb: *mut sk_buff,
// the lower-layer is responsible for appropriate locking
    pub reg): *const *const *const u8 (read_reg) (struct sja1000_priv priv, int,
    pub val): *const *const *const void (write_reg) (struct sja1000_priv priv, int reg, u8,
    pub priv): *const *const void (pre_irq) (struct sja1000_priv,
    pub priv): *const *const void (post_irq) (struct sja1000_priv,
    pub /: *mut *mut *mut void priv; / for board-specific data,
    pub dev: *mut net_device,
    pub /: *mut *mut *mut void __iomem reg_base; / ioremap'ed address to registers,
    pub /: *mut *mut unsigned long irq_flags; / for request_irq(),
    pub /: *mut *mut spinlock_t cmdreg_lock; / lock for concurrent cmd register writes,
    pub /: *mut *mut u16 flags; / custom mode flags,
    pub /: *mut *mut u8 ocr; / output control register,
    pub /: *mut *mut u8 cdr; / clock divider register,
}

extern "C" {
    pub fn free_sja1000dev(dev: *mut net_device);
}
extern "C" {
    pub fn register_sja1000dev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_sja1000dev(dev: *mut net_device);
}
extern "C" {
    pub fn sja1000_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
