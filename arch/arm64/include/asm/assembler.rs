//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/assembler.h
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
// Based on arch/arm/include/asm/assembler.h, arch/arm/mm/proc-macros.S
//
// Copyright (C) 1996-2000 Russell King
// Copyright (C) 2012 ARM Ltd.
//

//
// Provide a wxN alias for each wN register so what we can paste a xN
// reference after a 'w' to obtain the 32-bit version.
//
// Save/restore interrupts.
//
// call with daif masked
//
// RAS Error Synchronization barrier
//

//
// Value prediction barrier
//
// Clear Branch History instruction
//
// Speculation barrier
//
// NOP sequence
//
// Register aliases.
//
// Vector entry
//
// Select code when configured for BE.
//

// Macro flag: #define CPU_BE(code...)

//
// Select code when configured for LE.
//

// Macro flag: #define CPU_LE(code...)

//
// Define a macro that constructs a 64-bit value by concatenating two
// 32-bit registers. Note that on big endian systems the order of the
// registers is swapped.
//

//
// Pseudo-ops for PC-relative adr/ldr/str <reg>, <symbol> where
// <symbol> is within the range +/- 4 GB of the PC.
//
// @dst: destination register (64 bit wide)
// @sym: name of the symbol
//
// @dst: destination register (32 or 64 bit wide)
// @sym: name of the symbol
// @tmp: optional 64-bit scratch register to be used if <dst> is a
// 32-bit wide register, in which case it cannot be used to hold
// the address
//
// @src: source register (32 or 64 bit wide)
// @sym: name of the symbol
// @tmp: mandatory 64-bit scratch register to calculate the address
// while <src> needs to be preserved.
//
// @dst: destination register
//

//
// @dst: Result of per_cpu(sym, smp_processor_id()) (can be SP)
// @sym: The name of the per-cpu variable
// @tmp: scratch register
//
// @dst: Result of READ_ONCE(per_cpu(sym, smp_processor_id()))
// @sym: The name of the per-cpu variable
// @tmp: scratch register
//
// read_ctr - read CTR_EL0. If the system has mismatched register fields,
// provide the system wide safe value from arm64_ftr_reg_ctrel0.sys_val
//

//
// raw_dcache_line_size - get the minimum D-cache line size on this CPU
// from the CTR register.
//
// dcache_line_size - get the safe D-cache line size across all CPUs
//
// raw_icache_line_size - get the minimum I-cache line size on this CPU
// from the CTR register.
//
// icache_line_size - get the safe I-cache line size across all CPUs
//
// tcr_set_t0sz - update TCR.T0SZ so that we can load the ID map
//
// tcr_set_t1sz - update TCR.T1SZ
//
// tcr_compute_pa_size - set TCR.(I)PS to the highest supported
// ID_AA64MMFR0_EL1.PARange value
//
// tcr:		register with the TCR_ELx value to be updated
// pos:		IPS or PS bitfield position
// tmp{0,1}:	temporary registers
//
// Narrow PARange to fit the PS field in TCR_ELx

//
// Macro to perform a data cache maintenance for the interval
// [start, end) with dcache line size explicitly provided.
//
// op:		operation passed to dc instruction
// start:          starting virtual address of the region
// end:            end virtual address of the region
// linesz:		dcache line size
// fixup:		optional label to branch to on user fault
// Corrupts:       start, end, tmp
//
// Macro to perform a data cache maintenance for the interval
// [start, end) without waiting for completion
//
// op:		operation passed to dc instruction
// start:          starting virtual address of the region
// end:            end virtual address of the region
// fixup:		optional label to branch to on user fault
// Corrupts:       start, end, tmp1, tmp2
//
// Macro to perform a data cache maintenance for the interval
// [start, end) and wait for completion
//
// op:		operation passed to dc instruction
// domain:		domain used in dsb instruction
// start:          starting virtual address of the region
// end:            end virtual address of the region
// fixup:		optional label to branch to on user fault
// Corrupts:       start, end, tmp1, tmp2
//
// Macro to perform an instruction cache maintenance for the interval
// [start, end)
//
// start, end:	virtual addresses describing the region
// fixup:		optional label to branch to on user fault
// Corrupts:	tmp1, tmp2
//
// load_ttbr1 - install @pgtbl as a TTBR1 page table
// pgtbl preserved
// tmp1/tmp2 clobbered, either may overlap with pgtbl
//
// To prevent the possibility of old and new partial table walks being visible
// in the tlb, switch the ttbr to a zero page when we invalidate the old
// records. D4.7.1 'General TLB maintenance requirements' in ARM DDI 0487A.i
// Even switching to our copied tables will cause a changed output address at
// each stage of the walk.
//
// reset_pmuserenr_el0 - reset PMUSERENR_EL0 if PMUv3 present
//
// reset_amuserenr_el0 - reset AMUSERENR_EL0 if AMUv1 present
//
// copy_page - copy src to dest using temp registers t1-t8
//
// Annotate a function as being unsuitable for kprobes.
//

// Macro flag: #define NOKPROBE(x)

// Macro flag: #define EXPORT_SYMBOL_NOKASAN(name)

//
// Emit a 64-bit absolute little endian symbol reference in a way that
// ensures that it will be resolved at build time, even when building a
// PIE binary. This requires cooperation from the linker script, which
// must emit the lo32/hi32 halves individually.
//
// mov_q - move an immediate constant into a 64-bit register using
// between 2 and 4 movz/movk instructions (depending on the
// magnitude and sign of the operand)
//
// Return the current task_struct.
//
// If the kernel is built for 52-bit virtual addressing but the hardware only
// supports 48 bits, we cannot program the pgdir address into TTBR1 directly,
// but we have to add an offset so that the TTBR1 address corresponds with the
// pgdir entry that covers the lowest 48-bit addressable VA.
//
// Note that this trick is only used for LVA/64k pages - LPA2/4k pages uses an
// additional paging level, and on LPA2/16k pages, we would end up with a root
// level table with only 2 entries, which is suboptimal in terms of TLB
// utilization, so there we fall back to 47 bits of translation if LPA2 is not
// supported.
//
// orr is used as it can cover the immediate value (and is idempotent).
// ttbr: Value of ttbr to set, modified.
//

//
// Arrange a physical address in a TTBR register, taking care of 52-bit
// addresses.
//
// phys:	physical address, preserved
// ttbr:	returns the TTBR value
//

//
// tcr_clear_errata_bits - Clear TCR bits that trigger an errata on this CPU.
//

//
// Errata workaround prior to disable MMU. Insert an ISB immediately prior
// to executing the MSR that will change SCTLR_ELn[M] from a value of 1 to 0.
//

//
// frame_push - Push @regcount callee saved registers to the stack,
// starting at x19, as well as x29/x30, and set x29 to
// the new value of sp. Add @extra bytes of stack space
// for locals.
//
// frame_pop  - Pop the callee saved registers from the stack that were
// pushed in the most recent call to frame_push, as well
// as x29/x30 and any extra stack space that may have been
// allocated.
//
// Set SCTLR_ELx to the @reg value, and invalidate the local icache
// in the process. This is called when setting the MMU on.
//
// Invalidate the local I-cache so that any instructions fetched
// speculatively from the PoC are discarded, since they may have
// been dynamically patched at the PoU.
//
// Branch Target Identifier (BTI)
//
// This macro emits a program property note section identifying
// architecture features which require special handling, mainly for
// use in assembly files included in the VDSO.
//
pub const NT_GNU_PROPERTY_TYPE_0: c_int = 5;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: c_uint = 0xc0000000;

//
// This is described with an array of char in the Linux API
// spec but the text and all other usage (including binutils,
// clang and GCC) treat this as a 32 bit value so no swizzling
// is required for big endian.
//

// Save/restores x0-x3 to the stack

// Patched to NOP when not supported

