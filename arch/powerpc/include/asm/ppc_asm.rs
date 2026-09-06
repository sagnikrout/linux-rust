//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ppc_asm.h
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
// Copyright (C) 1995-1999 Gary Thomas, Paul Mackerras, Cort Dougan.
//

//
// This expands to a sequence of operations with reg incrementing from
// start to end inclusive, of this form:
//
// op  reg, (offset + (width * reg))(base)
//
// Note that offset is not the offset of the first operation unless start
// is zero (or width is zero).
//
// This expands to a sequence of register clears for regs start to end
// inclusive, of the form:
//
// li rN, 0
//
// Macros for storing registers into and loading registers from
// exception frames.
//

// macros for handling user register sanitisation

// Macro flag: #define HANDLER_RESTORE_NVGPRS()

// Macro flag: #define SANITIZE_SYSCALL_GPRS()
// Macro flag: #define SANITIZE_GPR(n)

// Macro flag: #define SANITIZE_NVGPRS()
// Macro flag: #define SANITIZE_RESTORE_NVGPRS()

// Save the lower 32 VSRs in the thread VSR region

//
// b = base register for addressing, o = base offset from register of 1st EVR
// n = first EVR, s = scratch
//

// Macros to adjust thread priority for hardware multithreading

pub const ULONG_SIZE: c_int = 8;

pub const ULONG_SIZE: c_int = 4;

//
// Used to name C functions called from asm
//

//
// We use __powerpc64__ here because we want the compat VDSO to use the 32-bit
// version below in the else case of the ifdef.
//

pub const STACKFRAMESIZE: c_int = 256;

pub const STK_GOT: c_int = 24;
pub const STK_PARAM_AREA: c_int = 32;

pub const STK_GOT: c_int = 40;
pub const STK_PARAM_AREA: c_int = 48;

//
// __kprobes (the C annotation) puts the symbol into the .kprobes.text
// section, which gets emitted at the end of regular text.
//
// _ASM_NOKPROBE_SYMBOL and NOKPROBE_SYMBOL just adds the symbol to
// a blacklist. The former is for core kprobe functions/data, the
// latter is for those that incdentially must be excluded from probing
// and allows them to be linked at more optimal location within text.
//

// Macro flag: #define _ASM_NOKPROBE_SYMBOL(entry)

// Macro flag: #define FUNC_END(name)
//
// LOAD_REG_IMMEDIATE(rn, expr)
// Loads the value of the constant expression 'expr' into register 'rn'
// using immediate instructions only.  Use this when it's important not
// to reference other data (i.e. on ppc64 when the TOC pointer is not
// valid) and when 'expr' is a constant or absolute address.
//
// LOAD_REG_ADDR(rn, name)
// Loads the address of label 'name' into register 'rn'.  Use this when
// you don't particularly need immediate instructions only, but you need
// the whole address in one register (e.g. it's a structure address and
// you want to access various offsets within it).  On ppc32 this is
// identical to LOAD_REG_IMMEDIATE.
//
// LOAD_REG_ADDR_PIC(rn, name)
// Loads the address of label 'name' into register 'run'. Use this when
// the kernel doesn't run at the linked or relocated address. Please
// note that this macro will clobber the lr register.
//
// LOAD_REG_ADDRBASE(rn, name)
// ADDROFF(name)
// LOAD_REG_ADDRBASE loads part of the address of label 'name' into
// register 'rn'.  ADDROFF(name) returns the remainder of the address as
// a constant expression.  ADDROFF(name) is a signed expression < 16 bits
// in size, so is suitable for use directly as an offset in load and store
// instructions.  Use this when loading/storing a single word or less as:
// LOAD_REG_ADDRBASE(rX, name)
// ld	rY,ADDROFF(name)(rX)
//
// Be careful, this will clobber the lr register.

//
// This is used in register-constrained interrupt handlers. Not to be used
// by BOOK3S. ld complains with "got/toc optimization is not supported" if r2
// is not used for the TOC offset, so use @got(tocreg). If the interrupt
// handlers saved r2 instead, LOAD_REG_ADDR could be used.
//

pub const ADDROFF(name): c_int = 0;
// offsets for stack frame layout
pub const LRSAVE: c_int = 16;
//
// GCC stack frames follow a different pattern on 32 vs 64. This can be used
// to make asm frames be consistent with C.
//

// offsets for stack frame layout
pub const LRSAVE: c_int = 4;

// various errata or part fixups

// Macro flag: #define TLBSYNC

//
// This instruction is not implemented on the PPC 603 or 601; however, on
// the 403GCX and 405GP tlbia IS defined and tlbie is not.
// All of these instructions exist in the 8xx, they have magical powers,
// and they must be used.
//

// Macro flag: #define PPC440EP_ERR42

// The following stops all load and store data streams associated with stream
// ID (ie. streams created explicitly).  The embedded and server mnemonics for
// dcbt are different so this must only be used for server.
//

// setup read stream 0 */					\
// setup write stream 1 */					\
//
// toreal/fromreal/tophys/tovirt macros. 32-bit BookE makes them
// keep the address intact to be compatible with code shared with
// 32-bit classic.
//
// On the other hand, I find it useful to have them behave as expected
// by their name (ie always do the addition) on 64-bit BookE
//

// Macro flag: #define toreal(rd)
// Macro flag: #define fromreal(rd)
//
// We use addis to ensure compatibility with the "classic" ppc versions of
// these macros, which use rs = 0 to get the tophys offset in rd, rather than
// converting the address in r0, and so this version has to do that too
// (i.e. set register rd to 0 when rs == 0).
//

// Macro flag: #define fromreal(rd)

// The boring bits...
// Condition Register Bit Fields
pub const cr0: c_int = 0;
pub const cr1: c_int = 1;
pub const cr2: c_int = 2;
pub const cr3: c_int = 3;
pub const cr4: c_int = 4;
pub const cr5: c_int = 5;
pub const cr6: c_int = 6;
pub const cr7: c_int = 7;
//
// General Purpose Registers (GPRs)
//
// The lower case r0-r31 should be used in preference to the upper
// case R0-R31 as they provide more error checking in the assembler.
// Use R0-31 only when really nessesary.
//

// Floating Point Registers (FPRs)
pub const fr0: c_int = 0;
pub const fr1: c_int = 1;
pub const fr2: c_int = 2;
pub const fr3: c_int = 3;
pub const fr4: c_int = 4;
pub const fr5: c_int = 5;
pub const fr6: c_int = 6;
pub const fr7: c_int = 7;
pub const fr8: c_int = 8;
pub const fr9: c_int = 9;
pub const fr10: c_int = 10;
pub const fr11: c_int = 11;
pub const fr12: c_int = 12;
pub const fr13: c_int = 13;
pub const fr14: c_int = 14;
pub const fr15: c_int = 15;
pub const fr16: c_int = 16;
pub const fr17: c_int = 17;
pub const fr18: c_int = 18;
pub const fr19: c_int = 19;
pub const fr20: c_int = 20;
pub const fr21: c_int = 21;
pub const fr22: c_int = 22;
pub const fr23: c_int = 23;
pub const fr24: c_int = 24;
pub const fr25: c_int = 25;
pub const fr26: c_int = 26;
pub const fr27: c_int = 27;
pub const fr28: c_int = 28;
pub const fr29: c_int = 29;
pub const fr30: c_int = 30;
pub const fr31: c_int = 31;
// AltiVec Registers (VPRs)
pub const v0: c_int = 0;
pub const v1: c_int = 1;
pub const v2: c_int = 2;
pub const v3: c_int = 3;
pub const v4: c_int = 4;
pub const v5: c_int = 5;
pub const v6: c_int = 6;
pub const v7: c_int = 7;
pub const v8: c_int = 8;
pub const v9: c_int = 9;
pub const v10: c_int = 10;
pub const v11: c_int = 11;
pub const v12: c_int = 12;
pub const v13: c_int = 13;
pub const v14: c_int = 14;
pub const v15: c_int = 15;
pub const v16: c_int = 16;
pub const v17: c_int = 17;
pub const v18: c_int = 18;
pub const v19: c_int = 19;
pub const v20: c_int = 20;
pub const v21: c_int = 21;
pub const v22: c_int = 22;
pub const v23: c_int = 23;
pub const v24: c_int = 24;
pub const v25: c_int = 25;
pub const v26: c_int = 26;
pub const v27: c_int = 27;
pub const v28: c_int = 28;
pub const v29: c_int = 29;
pub const v30: c_int = 30;
pub const v31: c_int = 31;
// VSX Registers (VSRs)
pub const vs0: c_int = 0;
pub const vs1: c_int = 1;
pub const vs2: c_int = 2;
pub const vs3: c_int = 3;
pub const vs4: c_int = 4;
pub const vs5: c_int = 5;
pub const vs6: c_int = 6;
pub const vs7: c_int = 7;
pub const vs8: c_int = 8;
pub const vs9: c_int = 9;
pub const vs10: c_int = 10;
pub const vs11: c_int = 11;
pub const vs12: c_int = 12;
pub const vs13: c_int = 13;
pub const vs14: c_int = 14;
pub const vs15: c_int = 15;
pub const vs16: c_int = 16;
pub const vs17: c_int = 17;
pub const vs18: c_int = 18;
pub const vs19: c_int = 19;
pub const vs20: c_int = 20;
pub const vs21: c_int = 21;
pub const vs22: c_int = 22;
pub const vs23: c_int = 23;
pub const vs24: c_int = 24;
pub const vs25: c_int = 25;
pub const vs26: c_int = 26;
pub const vs27: c_int = 27;
pub const vs28: c_int = 28;
pub const vs29: c_int = 29;
pub const vs30: c_int = 30;
pub const vs31: c_int = 31;
pub const vs32: c_int = 32;
pub const vs33: c_int = 33;
pub const vs34: c_int = 34;
pub const vs35: c_int = 35;
pub const vs36: c_int = 36;
pub const vs37: c_int = 37;
pub const vs38: c_int = 38;
pub const vs39: c_int = 39;
pub const vs40: c_int = 40;
pub const vs41: c_int = 41;
pub const vs42: c_int = 42;
pub const vs43: c_int = 43;
pub const vs44: c_int = 44;
pub const vs45: c_int = 45;
pub const vs46: c_int = 46;
pub const vs47: c_int = 47;
pub const vs48: c_int = 48;
pub const vs49: c_int = 49;
pub const vs50: c_int = 50;
pub const vs51: c_int = 51;
pub const vs52: c_int = 52;
pub const vs53: c_int = 53;
pub const vs54: c_int = 54;
pub const vs55: c_int = 55;
pub const vs56: c_int = 56;
pub const vs57: c_int = 57;
pub const vs58: c_int = 58;
pub const vs59: c_int = 59;
pub const vs60: c_int = 60;
pub const vs61: c_int = 61;
pub const vs62: c_int = 62;
pub const vs63: c_int = 63;
// SPE Registers (EVPRs)
pub const evr0: c_int = 0;
pub const evr1: c_int = 1;
pub const evr2: c_int = 2;
pub const evr3: c_int = 3;
pub const evr4: c_int = 4;
pub const evr5: c_int = 5;
pub const evr6: c_int = 6;
pub const evr7: c_int = 7;
pub const evr8: c_int = 8;
pub const evr9: c_int = 9;
pub const evr10: c_int = 10;
pub const evr11: c_int = 11;
pub const evr12: c_int = 12;
pub const evr13: c_int = 13;
pub const evr14: c_int = 14;
pub const evr15: c_int = 15;
pub const evr16: c_int = 16;
pub const evr17: c_int = 17;
pub const evr18: c_int = 18;
pub const evr19: c_int = 19;
pub const evr20: c_int = 20;
pub const evr21: c_int = 21;
pub const evr22: c_int = 22;
pub const evr23: c_int = 23;
pub const evr24: c_int = 24;
pub const evr25: c_int = 25;
pub const evr26: c_int = 26;
pub const evr27: c_int = 27;
pub const evr28: c_int = 28;
pub const evr29: c_int = 29;
pub const evr30: c_int = 30;
pub const evr31: c_int = 31;

//
// Create an endian fixup trampoline
//
// This starts with a "tdi 0,0,0x48" instruction which is
// essentially a "trap never", and thus akin to a nop.
//
// The opcode for this instruction read with the wrong endian
// however results in a b . + 8
//
// So essentially we use that trick to execute the following
// trampoline in "reverse endian" if we are running with the
// MSR_LE bit set the "wrong" way for whatever endianness the
// kernel is built for.
//

// Macro flag: #define FIXUP_ENDIAN

//
// This version may be used in HV or non-HV context.
// MSR[EE] must be disabled.
//

//
// This version that may only be used with MSR[HV]=1
// - Does not clear MSR[RI], so more robust.
// - Slightly smaller and faster.
//

// Macro flag: #define BTB_FLUSH(reg)

pub const STACK_FRAME_PARAMS: c_int = 48;

pub const STACK_FRAME_PARAMS: c_int = 32;

pub const STACK_FRAME_PARAMS: c_int = 8;

