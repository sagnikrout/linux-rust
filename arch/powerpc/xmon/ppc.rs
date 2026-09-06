//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/xmon/ppc.h
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


// ppc.h -- Header file for PowerPC opcode table

pub type ppc_cpu_t = u64;
// The opcode table is an array of struct powerpc_opcode.
// The opcode name.
// The opcode itself.  Those bits which will be filled in with
// The opcode mask.  This is used by the disassembler.  This is a
// One bit flags for the opcode.  These are used to indicate which
// An array of operand codes.  Each code is an index into the
// The table itself is sorted by major opcode number, and is otherwise
// Values defined for the flags field of a struct powerpc_opcode.
// Opcode is defined for the PowerPC architecture.
pub const PPC_OPCODE_PPC: c_int = 1;
// Opcode is defined for the POWER (RS/6000) architecture.
pub const PPC_OPCODE_POWER: c_int = 2;
// Opcode is defined for the POWER2 (Rios 2) architecture.
pub const PPC_OPCODE_POWER2: c_int = 4;
// Opcode is supported by the Motorola PowerPC 601 processor.  The 601
pub const PPC_OPCODE_601: c_int = 8;
// Opcode is supported in both the Power and PowerPC architectures
pub const PPC_OPCODE_COMMON: c_uint = 0x10;
// Opcode is supported for any Power or PowerPC platform (this is
pub const PPC_OPCODE_ANY: c_uint = 0x20;
// Opcode is only defined on 64 bit architectures.
pub const PPC_OPCODE_64: c_uint = 0x40;
// Opcode is supported as part of the 64-bit bridge.
pub const PPC_OPCODE_64_BRIDGE: c_uint = 0x80;
// Opcode is supported by Altivec Vector Unit
pub const PPC_OPCODE_ALTIVEC: c_uint = 0x100;
// Opcode is supported by PowerPC 403 processor.
pub const PPC_OPCODE_403: c_uint = 0x200;
// Opcode is supported by PowerPC BookE processor.
pub const PPC_OPCODE_BOOKE: c_uint = 0x400;
// Opcode is supported by PowerPC 440 processor.
pub const PPC_OPCODE_440: c_uint = 0x800;
// Opcode is only supported by Power4 architecture.
pub const PPC_OPCODE_POWER4: c_uint = 0x1000;
// Opcode is only supported by Power7 architecture.
pub const PPC_OPCODE_POWER7: c_uint = 0x2000;
// Opcode is only supported by e500x2 Core.
pub const PPC_OPCODE_SPE: c_uint = 0x4000;
// Opcode is supported by e500x2 Integer select APU.
pub const PPC_OPCODE_ISEL: c_uint = 0x8000;
// Opcode is an e500 SPE floating point instruction.
pub const PPC_OPCODE_EFS: c_uint = 0x10000;
// Opcode is supported by branch locking APU.
pub const PPC_OPCODE_BRLOCK: c_uint = 0x20000;
// Opcode is supported by performance monitor APU.
pub const PPC_OPCODE_PMR: c_uint = 0x40000;
// Opcode is supported by cache locking APU.
pub const PPC_OPCODE_CACHELCK: c_uint = 0x80000;
// Opcode is supported by machine check APU.
pub const PPC_OPCODE_RFMCI: c_uint = 0x100000;
// Opcode is only supported by Power5 architecture.
pub const PPC_OPCODE_POWER5: c_uint = 0x200000;
// Opcode is supported by PowerPC e300 family.
pub const PPC_OPCODE_E300: c_uint = 0x400000;
// Opcode is only supported by Power6 architecture.
pub const PPC_OPCODE_POWER6: c_uint = 0x800000;
// Opcode is only supported by PowerPC Cell family.
pub const PPC_OPCODE_CELL: c_uint = 0x1000000;
// Opcode is supported by CPUs with paired singles support.
pub const PPC_OPCODE_PPCPS: c_uint = 0x2000000;
// Opcode is supported by Power E500MC
pub const PPC_OPCODE_E500MC: c_uint = 0x4000000;
// Opcode is supported by PowerPC 405 processor.
pub const PPC_OPCODE_405: c_uint = 0x8000000;
// Opcode is supported by Vector-Scalar (VSX) Unit
pub const PPC_OPCODE_VSX: c_uint = 0x10000000;
// Opcode is supported by A2.
pub const PPC_OPCODE_A2: c_uint = 0x20000000;
// Opcode is supported by PowerPC 476 processor.
pub const PPC_OPCODE_476: c_uint = 0x40000000;
// Opcode is supported by AppliedMicro Titan core
pub const PPC_OPCODE_TITAN: c_uint = 0x80000000;
// Opcode which is supported by the e500 family
pub const PPC_OPCODE_E500: c_uint = 0x100000000ull;
// Opcode is supported by Extended Altivec Vector Unit
pub const PPC_OPCODE_ALTIVEC2: c_uint = 0x200000000ull;
// Opcode is supported by Power E6500
pub const PPC_OPCODE_E6500: c_uint = 0x400000000ull;
// Opcode is supported by Thread management APU
pub const PPC_OPCODE_TMR: c_uint = 0x800000000ull;
// Opcode which is supported by the VLE extension.
pub const PPC_OPCODE_VLE: c_uint = 0x1000000000ull;
// Opcode is only supported by Power8 architecture.
pub const PPC_OPCODE_POWER8: c_uint = 0x2000000000ull;
// Opcode which is supported by the Hardware Transactional Memory extension.
// Currently, this is the same as the POWER8 mask.  If another cpu comes out

// Opcode is supported by ppc750cl.
pub const PPC_OPCODE_750: c_uint = 0x4000000000ull;
// Opcode is supported by ppc7450.
pub const PPC_OPCODE_7450: c_uint = 0x8000000000ull;
// Opcode is supported by ppc821/850/860.
pub const PPC_OPCODE_860: c_uint = 0x10000000000ull;
// Opcode is only supported by Power9 architecture.
pub const PPC_OPCODE_POWER9: c_uint = 0x20000000000ull;
// Opcode is supported by Vector-Scalar (VSX) Unit from ISA 2.08.
pub const PPC_OPCODE_VSX3: c_uint = 0x40000000000ull;
// Opcode is supported by e200z4.
pub const PPC_OPCODE_E200Z4: c_uint = 0x80000000000ull;
// A macro to extract the major opcode from an instruction.

// A macro to determine if the instruction is a 2-byte VLE insn.

// A macro to extract the major opcode from a VLE instruction.

// A macro to convert a VLE opcode to a VLE opcode segment.

// The operands table is an array of struct powerpc_operand.
// A bitmask of bits in the operand.
// The shift operation to be applied to the operand.  No shift
// Insertion function.  This is used by the assembler.  To insert an
// Extraction function.  This is used by the disassembler.  To
// One bit syntax flags.
// Elements in the table are retrieved by indexing with values from
// Use with the shift field of a struct powerpc_operand to indicate

// Values defined for the flags field of a struct powerpc_operand.
// This operand takes signed values.

// This operand takes signed values, but also accepts a full positive

// This operand does not actually exist in the assembler input.  This

// The next operand should be wrapped in parentheses rather than
//

// This operand may use the symbolic names for the CR fields, which

// This operand names a register.  The disassembler uses this to print

// Like PPC_OPERAND_GPR, but don't print a leading 'r' for r0.

// This operand names a floating point register.  The disassembler

// This operand is a relative branch displacement.  The disassembler

// This operand is an absolute branch address.  The disassembler

// This operand is optional, and is zero if omitted.  This is used for

// This flag is only used with PPC_OPERAND_OPTIONAL.  If this operand

// This operand should be regarded as a negative number for the

// This operand names a vector unit register.  The disassembler

// This operand is for the DS field in a DS form instruction.

// This operand is for the DQ field in a DQ form instruction.

// Valid range of operand is 0..n rather than 0..n-1.

// Xilinx APU and FSL related operands

// This operand names a vector-scalar unit register.  The disassembler

// This is a CR FIELD that does not use symbolic names.

// This flag is only used with PPC_OPERAND_OPTIONAL.  If this operand

// This flag is only used with PPC_OPERAND_OPTIONAL.  The operand is

// The POWER and PowerPC assemblers use a few macros.  We keep them
// The macro name.
// The number of operands the macro takes.
// One bit flags for the opcode.  These are used to indicate which
// A format string to turn the macro into a normal instruction.

