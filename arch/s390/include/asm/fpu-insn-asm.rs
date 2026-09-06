//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/fpu-insn-asm.h
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
// Support for Vector Instructions
//
// Assembler macros to generate .byte/.word code for particular
// vector instructions that are supported by recent binutils (>= 2.26) only.
//
// Copyright IBM Corp. 2015
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

// RXB - Compute most significant bit used vector registers
//
// @rxb:	Operand to store computed RXB value
// @v1:		Vector register designated operand whose MSB is stored in
// RXB bit 0 (instruction bit 36) and whose remaining bits
// are stored in instruction bits 8-11.
// @v2:		Vector register designated operand whose MSB is stored in
// RXB bit 1 (instruction bit 37) and whose remaining bits
// are stored in instruction bits 12-15.
// @v3:		Vector register designated operand whose MSB is stored in
// RXB bit 2 (instruction bit 38) and whose remaining bits
// are stored in instruction bits 16-19.
// @v4:		Vector register designated operand whose MSB is stored in
// RXB bit 3 (instruction bit 39) and whose remaining bits
// are stored in instruction bits 32-35.
//
// Note: In most vector instruction formats [1] V1, V2, V3, and V4 directly
// correspond to @v1, @v2, @v3, and @v4. But there are exceptions, such as but
// not limited to the vector instruction formats VRR-g, VRR-h, VRS-a, VRS-d,
// and VSI.
//
// [1] IBM z/Architecture Principles of Operation, chapter "Program
// Execution, section "Instructions", subsection "Instruction Formats".
//
// MRXB - Generate Element Size Control and RXB value
//
// @m:		Element size control
// @v1:		First vector register designated operand (for RXB)
// @v2:		Second vector register designated operand (for RXB)
// @v3:		Third vector register designated operand (for RXB)
// @v4:		Fourth vector register designated operand (for RXB)
//
// Note: For @v1, @v2, @v3, and @v4 also refer to the RXB macro
// description for further details.
//
// MRXBOPC - Generate Element Size Control, RXB, and final Opcode fields
//
// @m:		Element size control
// @opc:	Opcode
// @v1:		First vector register designated operand (for RXB)
// @v2:		Second vector register designated operand (for RXB)
// @v3:		Third vector register designated operand (for RXB)
// @v4:		Fourth vector register designated operand (for RXB)
//
// Note: For @v1, @v2, @v3, and @v4 also refer to the RXB macro
// description for further details.
//
// Vector support instructions
// VECTOR GENERATE BYTE MASK
// VECTOR LOAD VR ELEMENT FROM GR
// VECTOR LOAD REGISTER
// VECTOR LOAD
// VECTOR LOAD ELEMENT
// VECTOR LOAD ELEMENT IMMEDIATE
// VECTOR LOAD GR FROM VR ELEMENT
// VECTOR LOAD MULTIPLE
// VECTOR STORE
// VECTOR STORE BYTE REVERSED ELEMENTS
// VECTOR STORE MULTIPLE
// VECTOR PERMUTE
// VECTOR UNPACK LOGICAL LOW
// VECTOR PERMUTE DOUBLEWORD IMMEDIATE
// VECTOR REPLICATE
// VECTOR MERGE HIGH
// VECTOR MERGE LOW
// VECTOR LOAD WITH LENGTH
// VECTOR STORE WITH LENGTH
// Vector integer instructions
// VECTOR AND
// VECTOR CHECKSUM
// VECTOR EXCLUSIVE OR
// VECTOR GALOIS FIELD MULTIPLY SUM
// VECTOR GALOIS FIELD MULTIPLY SUM AND ACCUMULATE
// VECTOR SHIFT RIGHT LOGICAL BY BYTE
// VECTOR REPLICATE IMMEDIATE
// VECTOR ADD
// VECTOR ELEMENT SHIFT RIGHT ARITHMETIC
// VECTOR ELEMENT ROTATE LEFT LOGICAL
// VECTOR SHIFT LEFT DOUBLE BY BYTE

