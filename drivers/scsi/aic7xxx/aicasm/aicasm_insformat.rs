//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aicasm/aicasm_insformat.h
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
// Instruction formats for the sequencer program downloaded to
// Aic7xxx SCSI host adapters
//
// Copyright (c) 1997, 1998, 2000 Justin T. Gibbs.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id: //depot/aic7xxx/aic7xxx/aicasm/aicasm_insformat.h#12 $
//
// $FreeBSD$
//

// 8bit ALU logic operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format1 {

    pub 1: parity :,

    pub 8: immediate :,

}

// 8bit ALU shift/rotate operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format2 {

    pub 1: parity :,

    pub 8: shift_control :,

}

// 8bit branch control operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format3 {

    pub 1: parity :,

    pub 8: immediate :,

}

// 16bit ALU logic operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format4 {

    pub 1: parity :,

    pub 8: opcode_ext :,

}

// 16bit branch control operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format5 {

    pub 1: parity :,

    pub 8: opcode_ext :,

}

// Far branch operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_format6 {

    pub 1: parity :,

    pub 3: page :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ins_formats {
    pub format1: ins_format1,
    pub format2: ins_format2,
    pub format3: ins_format3,
    pub format4: ins_format4,
    pub format5: ins_format5,
    pub format6: ins_format6,
    pub bytes: [u8; 4],
    pub integer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct instruction {
    pub format: ins_formats,
    pub srcline: u_int,
    pub patch_label: *mut symbol,
    pub links: STAILQ_ENTRY(instruction),
}

pub const AIC_OP_OR: c_uint = 0x0;
pub const AIC_OP_AND: c_uint = 0x1;
pub const AIC_OP_XOR: c_uint = 0x2;
pub const AIC_OP_ADD: c_uint = 0x3;
pub const AIC_OP_ADC: c_uint = 0x4;
pub const AIC_OP_ROL: c_uint = 0x5;
pub const AIC_OP_BMOV: c_uint = 0x6;
pub const AIC_OP_MVI16: c_uint = 0x7;
pub const AIC_OP_JMP: c_uint = 0x8;
pub const AIC_OP_JC: c_uint = 0x9;
pub const AIC_OP_JNC: c_uint = 0xa;
pub const AIC_OP_CALL: c_uint = 0xb;
pub const AIC_OP_JNE: c_uint = 0xc;
pub const AIC_OP_JNZ: c_uint = 0xd;
pub const AIC_OP_JE: c_uint = 0xe;
pub const AIC_OP_JZ: c_uint = 0xf;
// Pseudo Ops
pub const AIC_OP_SHL: c_uint = 0x10;
pub const AIC_OP_SHR: c_uint = 0x20;
pub const AIC_OP_ROR: c_uint = 0x30;
// 16bit Ops. Low byte main opcode.  High byte extended opcode.
pub const AIC_OP_OR16: c_uint = 0x8005;
pub const AIC_OP_AND16: c_uint = 0x8105;
pub const AIC_OP_XOR16: c_uint = 0x8205;
pub const AIC_OP_ADD16: c_uint = 0x8305;
pub const AIC_OP_ADC16: c_uint = 0x8405;
pub const AIC_OP_JNE16: c_uint = 0x8805;
pub const AIC_OP_JNZ16: c_uint = 0x8905;
pub const AIC_OP_JE16: c_uint = 0x8C05;
pub const AIC_OP_JZ16: c_uint = 0x8B05;
pub const AIC_OP_JMP16: c_uint = 0x9005;
pub const AIC_OP_JC16: c_uint = 0x9105;
pub const AIC_OP_JNC16: c_uint = 0x9205;
pub const AIC_OP_CALL16: c_uint = 0x9305;
// Page extension is low three bits of second opcode byte.
pub const AIC_OP_JMPF: c_uint = 0xA005;
pub const AIC_OP_CALLF: c_uint = 0xB005;
pub const AIC_OP_JCF: c_uint = 0xC005;
pub const AIC_OP_JNCF: c_uint = 0xD005;
pub const AIC_OP_CMPXCHG: c_uint = 0xE005;
