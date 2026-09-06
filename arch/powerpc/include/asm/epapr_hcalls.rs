//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/epapr_hcalls.h
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
// ePAPR hcall interface
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
//
// Author: Timur Tabi <timur@freescale.com>
//
// This file is provided under a dual BSD/GPL license.  When using or
// redistributing this file, you may do so under either license.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// A "hypercall" is an "sc 1" instruction.  This header file provides C
// wrapper functions for the ePAPR hypervisor interface.  It is inteded
// for use by Linux device drivers and other operating systems.
//
// The hypercalls are implemented as inline assembly, rather than assembly
// language functions in a .S file, for optimization.  It allows
// the caller to issue the hypercall instruction directly, improving both
// performance and memory footprint.
//

//
// Hypercall register clobber list
//
// These macros are used to define the list of clobbered registers during a
// hypercall.  Technically, registers r0 and r3-r12 are always clobbered,
// but the gcc inline assembly syntax does not allow us to specify registers
// on the clobber list that are also on the input/output list.  Therefore,
// the lists of clobbered registers depends on the number of register
// parameters ("+r" and "=r") passed to the hypercall.
//
// Each assembly block should use one of the HCALL_CLOBBERSx macros.  As a
// general rule, 'x' is the number of parameters passed to the assembly
// block *except* for r11.
//
// If you're not sure, just use the smallest value of 'x' that does not
// generate a compilation error.  Because these are static inline functions,
// the compiler will only check the clobber list for a function if you
// compile code that calls that function.
//
// r3 and r11 are not included in any clobbers list because they are always
// listed as output registers.
//
// XER, CTR, and LR are currently listed as clobbers because it's uncertain
// whether they will be clobbered.
//
// Note that r11 can be used as an output parameter.
//
// The "memory" clobber is only necessary for hcalls where the Hypervisor
// will read or write guest memory. However, we add it to all hcalls because
// the impact is minimal, and we want to ensure that it's present for the
// hcalls that need it.
//
// List of common clobbered registers.  Do not use this macro.

extern "C" {
    pub fn epapr_paravirt_early_init() -> int __init;
}

//
// We use "uintptr_t" to define a register because it's guaranteed to be a
// 32-bit integer on a 32-bit platform, and a 64-bit integer on a 64-bit
// platform.
//
// All registers are either input/output or output only.  Registers that are
// initialized before making the hypercall are input/output.  All
// input/output registers are represented with "+r".  Output-only registers
// are represented with "=r".  Do not specify any unused registers.  The
// clobber list will tell the compiler that the hypercall modifies those
// registers, which is good enough.
//
// ev_int_set_config - configure the specified interrupt
// @interrupt: the interrupt number
// @config: configuration for this interrupt
// @priority: interrupt priority
// @destination: destination CPU number
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
//
// ev_int_get_config - return the config of the specified interrupt
// @interrupt: the interrupt number
// @config: returned configuration for this interrupt
// @priority: returned interrupt priority
// @destination: returned destination CPU number
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
// config = r4;
// priority = r5;
// destination = r6;
//
// ev_int_set_mask - sets the mask for the specified interrupt source
// @interrupt: the interrupt number
// @mask: 0=enable interrupts, 1=disable interrupts
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
//
// ev_int_get_mask - returns the mask for the specified interrupt source
// @interrupt: the interrupt number
// @mask: returned mask for this interrupt (0=enabled, 1=disabled)
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
// mask = r4;
//
// ev_int_eoi - signal the end of interrupt processing
// @interrupt: the interrupt number
//
// This function signals the end of processing for the specified
// interrupt, which must be the interrupt currently in service. By
// definition, this is also the highest-priority interrupt.
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
//
// ev_byte_channel_send - send characters to a byte stream
// @handle: byte stream handle
// @count: (input) num of chars to send, (output) num chars sent
// @buffer: pointer to a 16-byte buffer
//
// @buffer must be at least 16 bytes long, because all 16 bytes will be
// read from memory into registers, even if count < 16.
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
extern "C" {
    pub fn __asm__(_arg: "r7") -> register uintptr_t r7;
}
extern "C" {
    pub fn __asm__(_arg: "r8") -> register uintptr_t r8;
}
// count = r4;
//
// ev_byte_channel_receive - fetch characters from a byte channel
// @handle: byte channel handle
// @count: (input) max num of chars to receive, (output) num chars received
// @buffer: pointer to a 16-byte buffer
//
// The size of @buffer must be at least 16 bytes, even if you request fewer
// than 16 characters, because we always write 16 bytes to @buffer.  This is
// for performance reasons.
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
extern "C" {
    pub fn __asm__(_arg: "r6") -> register uintptr_t r6;
}
extern "C" {
    pub fn __asm__(_arg: "r7") -> register uintptr_t r7;
}
extern "C" {
    pub fn __asm__(_arg: "r8") -> register uintptr_t r8;
}
// count = r4;
//
// ev_byte_channel_poll - returns the status of the byte channel buffers
// @handle: byte channel handle
// @rx_count: returned count of bytes in receive queue
// @tx_count: returned count of free space in transmit queue
//
// This function reports the amount of data in the receive queue (i.e. the
// number of bytes you can read), and the amount of free space in the transmit
// queue (i.e. the number of bytes you can write).
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
extern "C" {
    pub fn __asm__(_arg: "r5") -> register uintptr_t r5;
}
// rx_count = r4;
// tx_count = r5;
//
// ev_int_iack - acknowledge an interrupt
// @handle: handle to the target interrupt controller
// @vector: returned interrupt vector
//
// If handle is zero, the function returns the next interrupt source
// number to be handled irrespective of the hierarchy or cascading
// of interrupt controllers. If non-zero, specifies a handle to the
// interrupt controller that is the target of the acknowledge.
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
extern "C" {
    pub fn __asm__(_arg: "r4") -> register uintptr_t r4;
}
// vector = r4;
//
// ev_doorbell_send - send a doorbell to another partition
// @handle: doorbell send handle
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}
//
// ev_idle -- wait for next interrupt on this core
//
// Returns 0 for success, or an error code.
//
extern "C" {
    pub fn __asm__(_arg: "r11") -> register uintptr_t r11;
}
extern "C" {
    pub fn __asm__(_arg: "r3") -> register uintptr_t r3;
}

extern "C" {
    pub fn asm(_arg: "r0") -> register unsigned long r0;
}
extern "C" {
    pub fn asm(_arg: "r12") -> register unsigned long r12;
}

// r2 = out[0];
extern "C" {
    pub fn epapr_hypercall(_arg: in, _arg: out, _arg: nr) -> return;
}
extern "C" {
    pub fn epapr_hypercall(_arg: in, _arg: out, _arg: nr) -> return;
}
extern "C" {
    pub fn epapr_hypercall(_arg: in, _arg: out, _arg: nr) -> return;
}
extern "C" {
    pub fn epapr_hypercall(_arg: in, _arg: out, _arg: nr) -> return;
}
extern "C" {
    pub fn epapr_hypercall(_arg: in, _arg: out, _arg: nr) -> return;
}

