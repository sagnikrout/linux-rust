//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_arm.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

//
// Because I'm terribly lazy and that repainting the whole of the KVM
// code with the proper names is a pain, use a helper to map the names
// inherited from AArch32 with the new fancy nomenclature. One day...
//

//
// The bits we set in HCR:
// TLOR:	Trap LORegion register accesses
// RW:		64bit by default, can be overridden for 32bit VMs
// TACR:	Trap ACTLR
// TSC:		Trap SMC
// TSW:		Trap cache operations by set/way
// TWE:		Trap WFE
// TWI:		Trap WFI
// TIDCP:	Trap L2CTLR/L2ECTLR
// BSU_IS:	Upgrade barriers to the inner shareable domain
// FB:		Force broadcast of all maintenance operations
// AMO:		Override CPSR.A and enable signaling with VA
// IMO:		Override CPSR.I and enable signaling with VI
// FMO:		Override CPSR.F and enable signaling with VF
// SWIO:	Turn set/way invalidates into set/way clean+invalidate
// PTW:		Take a stage2 fault if a stage1 walk steps in device memory
// TID3:	Trap EL1 reads of group 3 ID registers
// TID1:	Trap REVIDR_EL1, AIDR_EL1, and SMIDR_EL1
//

pub const MPAMHCR_HOST_FLAGS: c_int = 0;
// TCR_EL2 Registers bits

pub const TCR_EL2_PS_SHIFT: c_int = 16;

pub const TCR_EL2_T0SZ_MASK: c_uint = 0x3f;

//
// The VTCR_EL2 is configured per VM and is initialised in kvm_init_stage2_mmu.
//
// Note that when using 4K pages, we concatenate two first level page tables
// together. With 16K pages, we concatenate 16 first level page tables.
//
// VTCR_EL2:SL0 indicates the entry level for Stage2 translation.
// Interestingly, it depends on the page size.
// See D.10.2.121, VTCR_EL2, in ARM DDI 0487C.a
//
// -----------------------------------------
// | Entry level		|  4K  | 16K/64K |
// ------------------------------------------
// | Level: 0		|  2   |   -     |
// ------------------------------------------
// | Level: 1		|  1   |   2     |
// ------------------------------------------
// | Level: 2		|  0   |   1     |
// ------------------------------------------
// | Level: 3		|  -   |   0     |
// ------------------------------------------
//
// The table roughly translates to :
//
// SL0(PAGE_SIZE, Entry_level) = TGRAN_SL0_BASE - Entry_Level
//
// Where TGRAN_SL0_BASE is a magic number depending on the page size:
// TGRAN_SL0_BASE(4K) = 2
// TGRAN_SL0_BASE(16K) = 3
// TGRAN_SL0_BASE(64K) = 3
// provided we take care of ruling out the unsupported cases and
// Entry_Level = 4 - Number_of_levels.
//

//
// ARM VMSAv8-64 defines an algorithm for finding the translation table
// descriptors in section D4.2.8 in ARM DDI 0487C.a.
//
// The algorithm defines the expectations on the translation table
// addresses for each level, based on PAGE_SIZE, entry level
// and the translation table size (T0SZ). The variable "x" in the
// algorithm determines the alignment of a table base address at a given
// level and thus determines the alignment of VTTBR:BADDR for stage2
// page table entry level.
// Since the number of bits resolved at the entry level could vary
// depending on the T0SZ, the value of "x" is defined based on a
// Magic constant for a given PAGE_SIZE and Entry Level. The
// intermediate levels must be always aligned to the PAGE_SIZE (i.e,
// x = PAGE_SHIFT).
//
// The value of "x" for entry level is calculated as :
// x = Magic_N - T0SZ
//
// where Magic_N is an integer depending on the page size and the entry
// level of the page table as below:
//
// --------------------------------------------
// | Entry level		|  4K    16K   64K |
// --------------------------------------------
// | Level: 0 (4 levels)	| 28   |  -  |  -  |
// --------------------------------------------
// | Level: 1 (3 levels)	| 37   | 31  | 25  |
// --------------------------------------------
// | Level: 2 (2 levels)	| 46   | 42  | 38  |
// --------------------------------------------
// | Level: 3 (1 level)	| -    | 53  | 51  |
// --------------------------------------------
//
// We have a magic formula for the Magic_N below:
//
// Magic_N(PAGE_SIZE, Level) = 64 - ((PAGE_SHIFT - 3) * Number_of_levels)
//
// where Number_of_levels = (4 - Level). We are only interested in the
// value for Entry_Level for the stage2 page table.
//
// So, given that T0SZ = (64 - IPA_SHIFT), we can compute 'x' as follows:
//
// x = (64 - ((PAGE_SHIFT - 3) * Number_of_levels)) - (64 - IPA_SHIFT)
// = IPA_SHIFT - ((PAGE_SHIFT - 3) * Number of levels)
//
// Here is one way to explain the Magic Formula:
//
// x = log2(Size_of_Entry_Level_Table)
//
// Since, we can resolve (PAGE_SHIFT - 3) bits at each level, and another
// PAGE_SHIFT bits in the PTE, we have :
//
// Bits_Entry_level = IPA_SHIFT - ((PAGE_SHIFT - 3) * (n - 1) + PAGE_SHIFT)
// = IPA_SHIFT - (PAGE_SHIFT - 3) * n - 3
// where n = number of levels, and since each pointer is 8bytes, we have:
//
// x = Bits_Entry_Level + 3
// = IPA_SHIFT - (PAGE_SHIFT - 3) * n
//
// The only constraint here is that, we have to find the number of page table
// levels for a given IPA size (which we do, see stage2_pt_levels())
//

// Hyp System Trap Register

// Hyp Coprocessor Trap Register Shifts
pub const CPTR_EL2_TFP_SHIFT: c_int = 10;
// Hyp Coprocessor Trap Register

// Hyp Prefetch Fault Address Register (HPFAR/HDFAR)

//
// We have
// PAR	[PA_Shift - 1	: 12] = PA	[PA_Shift - 1 : 12]
// HPFAR	[PA_Shift - 9	: 4]  = FIPA	[PA_Shift - 1 : 12]
//
// Always assume 52 bit PA since at this point, we don't know how many PA bits
// the page table has been set up for. This should be safe since unused address
// bits in PAR are res0.
//

//
// ARMv8 Reset Values
//

