//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/xive-regs.h
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
// Copyright 2016,2017 IBM Corporation.
//
// "magic" Event State Buffer (ESB) MMIO offsets.
//
// Each interrupt source has a 2-bit state machine called ESB
// which can be controlled by MMIO. It's made of 2 bits, P and
// Q. P indicates that an interrupt is pending (has been sent
// to a queue and is waiting for an EOI). Q indicates that the
// interrupt has been triggered while pending.
//
// This acts as a coalescing mechanism in order to guarantee
// that a given interrupt only occurs at most once in a queue.
//
// When doing an EOI, the Q bit will indicate if the interrupt
// needs to be re-triggered.
//
// The following offsets into the ESB MMIO allow to read or
// manipulate the PQ bits. They must be used with an 8-bytes
// load instruction. They all return the previous state of the
// interrupt (atomically).
//
// Additionally, some ESB pages support doing an EOI via a
// store at 0 and some ESBs support doing a trigger via a
// separate trigger page.
//
pub const XIVE_ESB_STORE_EOI: c_uint = 0x400 /* Store */;
pub const XIVE_ESB_LOAD_EOI: c_uint = 0x000 /* Load */;
pub const XIVE_ESB_GET: c_uint = 0x800 /* Load */;
pub const XIVE_ESB_SET_PQ_00: c_uint = 0xc00 /* Load */;
pub const XIVE_ESB_SET_PQ_01: c_uint = 0xd00 /* Load */;
pub const XIVE_ESB_SET_PQ_10: c_uint = 0xe00 /* Load */;
pub const XIVE_ESB_SET_PQ_11: c_uint = 0xf00 /* Load */;
//
// Load-after-store ordering
//
// Adding this offset to the load address will enforce
// load-after-store ordering. This is required to use StoreEOI.
//
pub const XIVE_ESB_LD_ST_MO: c_uint = 0x40 /* Load-after-store ordering */;
pub const XIVE_ESB_VAL_P: c_uint = 0x2;
pub const XIVE_ESB_VAL_Q: c_uint = 0x1;
pub const XIVE_ESB_INVALID: c_uint = 0xFF;
//
// Thread Management (aka "TM") registers
//
// TM register offsets
pub const TM_QW0_USER: c_uint = 0x000 /* All rings */;
pub const TM_QW1_OS: c_uint = 0x010 /* Ring 0..2 */;
pub const TM_QW2_HV_POOL: c_uint = 0x020 /* Ring 0..1 */;
pub const TM_QW3_HV_PHYS: c_uint = 0x030 /* Ring 0..1 */;
// Byte offsets inside a QW             QW0 QW1 QW2 QW3
pub const TM_NSR: c_uint = 0x0  /*  +   +   -   +  */;
pub const TM_CPPR: c_uint = 0x1  /*  -   +   -   +  */;
pub const TM_IPB: c_uint = 0x2  /*  -   +   +   +  */;
pub const TM_LSMFB: c_uint = 0x3  /*  -   +   +   +  */;
pub const TM_ACK_CNT: c_uint = 0x4  /*  -   +   -   -  */;
pub const TM_INC: c_uint = 0x5  /*  -   +   -   +  */;
pub const TM_AGE: c_uint = 0x6  /*  -   +   -   +  */;
pub const TM_PIPR: c_uint = 0x7  /*  -   +   -   +  */;
pub const TM_WORD0: c_uint = 0x0;
pub const TM_WORD1: c_uint = 0x4;
//
// QW word 2 contains the valid bit at the top and other fields
// depending on the QW.
//
pub const TM_WORD2: c_uint = 0x8;

//
// In addition to normal loads to "peek" and writes (only when invalid)
// using 4 and 8 bytes accesses, the above registers support these
// "special" byte operations:
//
// - Byte load from QW0[NSR] - User level NSR (EBB)
// - Byte store to QW0[NSR] - User level NSR (EBB)
// - Byte load/store to QW1[CPPR] and QW3[CPPR] - CPPR access
// - Byte load from QW3[TM_WORD2] - Read VT||00000||LP||LE on thrd 0
// otherwise VT||0000000
// - Byte store to QW3[TM_WORD2] - Set VT bit (and LP/LE if present)
//
// Then we have all these "special" CI ops at these offset that trigger
// all sorts of side effects:
//
pub const TM_SPC_ACK_EBB: c_uint = 0x800	/* Load8 ack EBB to reg*/;
pub const TM_SPC_ACK_OS_REG: c_uint = 0x810	/* Load16 ack OS irq to reg */;
pub const TM_SPC_PUSH_USR_CTX: c_uint = 0x808	/* Store32 Push/Validate user context */;
pub const TM_SPC_PULL_USR_CTX: c_uint = 0x808	/* Load32 Pull/Invalidate user context */;
pub const TM_SPC_SET_OS_PENDING: c_uint = 0x812	/* Store8 Set OS irq pending bit */;
pub const TM_SPC_PULL_OS_CTX: c_uint = 0x818	/* Load32/Load64 Pull/Invalidate OS context to reg */;
pub const TM_SPC_PULL_POOL_CTX: c_uint = 0x828	/* Load32/Load64 Pull/Invalidate Pool context to reg*/;
pub const TM_SPC_ACK_HV_REG: c_uint = 0x830	/* Load16 ack HV irq to reg */;
pub const TM_SPC_PULL_USR_CTX_OL: c_uint = 0xc08	/* Store8 Pull/Inval usr ctx to odd line */;
pub const TM_SPC_ACK_OS_EL: c_uint = 0xc10	/* Store8 ack OS irq to even line */;
pub const TM_SPC_ACK_HV_POOL_EL: c_uint = 0xc20	/* Store8 ack HV evt pool to even line */;
pub const TM_SPC_ACK_HV_EL: c_uint = 0xc30	/* Store8 ack HV irq to even line */;
// XXX more...
// NSR fields for the various QW ack types

pub const TM_QW3_NSR_HE_NONE: c_int = 0;
pub const TM_QW3_NSR_HE_POOL: c_int = 1;
pub const TM_QW3_NSR_HE_PHYS: c_int = 2;
pub const TM_QW3_NSR_HE_LSI: c_int = 3;

