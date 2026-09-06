//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_reg.h
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


// Copyright 2003 Tungsten Graphics, Inc., Cedar Park, Texas.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
// IN NO EVENT SHALL TUNGSTEN GRAPHICS AND/OR ITS SUPPLIERS BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

//
// DOC: The i915 register macro definition style guide
//
// Follow the style described here for new macros, and while changing existing
// macros. Do **not** mass change existing definitions just to update the style.
//
// File Layout
// ~~~~~~~~~~~
//
// Keep helper macros near the top. For example, _PIPE() and friends.
//
// Prefix macros that generally should not be used outside of this file with
// underscore '_'. For example, _PIPE() and friends, single instances of
// registers that are defined solely for the use by function-like macros.
//
// Avoid using the underscore prefixed macros outside of this file. There are
// exceptions, but keep them to a minimum.
//
// There are two basic types of register definitions: Single registers and
// register groups. Register groups are registers which have two or more
// instances, for example one per pipe, port, transcoder, etc. Register groups
// should be defined using function-like macros.
//
// For single registers, define the register offset first, followed by register
// contents.
//
// For register groups, define the register instance offsets first, prefixed
// with underscore, followed by a function-like macro choosing the right
// instance based on the parameter, followed by register contents.
//
// Define the register contents (i.e. bit and bit field macros) from most
// significant to least significant bit. Indent the register content macros
// using two extra spaces between ``#define`` and the macro name.
//
// Define bit fields using ``REG_GENMASK(h, l)``. Define bit field contents
// using ``REG_FIELD_PREP(mask, value)``. This will define the values already
// shifted in place, so they can be directly OR'd together. For convenience,
// function-like macros may be used to define bit fields, but do note that the
// macros may be needed to read as well as write the register contents.
//
// Define bits using ``REG_BIT(N)``. Do **not** add ``_BIT`` suffix to the name.
//
// Group the register and its contents together without blank lines, separate
// from other registers and their contents with one blank line.
//
// Indent macro values from macro names using TABs. Align values vertically. Use
// braces in macro values as needed to avoid unintended precedence after macro
// substitution. Use spaces in macro values according to kernel coding
// style. Use lower case in hexadecimal values.
//
// Naming
// ~~~~~~
//
// Try to name registers according to the specs. If the register name changes in
// the specs from platform to another, stick to the original name.
//
// Try to reuse existing register macro definitions. Only add new macros for
// new register offsets, or when the register contents have changed enough to
// warrant a full redefinition.
//
// When a register macro changes for a new platform, prefix the new macro using
// the platform acronym or generation. For example, ``SKL_`` or ``GEN8_``. The
// prefix signifies the start platform/generation using the register.
//
// When a bit (field) macro changes or gets added for a new platform, while
// retaining the existing register macro, add a platform acronym or generation
// suffix to the name. For example, ``_SKL`` or ``_GEN8``.
//
// Examples
// ~~~~~~~~
//
// (Note that the values in the example are indented using spaces instead of
// TABs to avoid misalignment in generated documentation. Use TABs in the
// definitions.)::
//
// #define _FOO_A                      0xf000
// #define _FOO_B                      0xf001
// #define FOO(pipe)                   _MMIO_PIPE(pipe, _FOO_A, _FOO_B)
// #define   FOO_ENABLE                REG_BIT(31)
// #define   FOO_MODE_MASK             REG_GENMASK(19, 16)
// #define   FOO_MODE_BAR              REG_FIELD_PREP(FOO_MODE_MASK, 0)
// #define   FOO_MODE_BAZ              REG_FIELD_PREP(FOO_MODE_MASK, 1)
// #define   FOO_MODE_QUX_SNB          REG_FIELD_PREP(FOO_MODE_MASK, 2)
//
// #define BAR                         _MMIO(0xb000)
// #define GEN8_BAR                    _MMIO(0xb888)
//

//
// Reset registers
//

//
// IOSF sideband
//

pub const IOSF_DEVFN_SHIFT: c_int = 24;
pub const IOSF_OPCODE_SHIFT: c_int = 16;
pub const IOSF_PORT_SHIFT: c_int = 8;
pub const IOSF_BYTE_ENABLES_SHIFT: c_int = 4;
pub const IOSF_BAR_SHIFT: c_int = 1;

pub const IOSF_PORT_BUNIT: c_uint = 0x03;
pub const IOSF_PORT_PUNIT: c_uint = 0x04;
pub const IOSF_PORT_NC: c_uint = 0x11;
pub const IOSF_PORT_DPIO: c_uint = 0x12;
pub const IOSF_PORT_GPIO_NC: c_uint = 0x13;
pub const IOSF_PORT_CCK: c_uint = 0x14;
pub const IOSF_PORT_DPIO_2: c_uint = 0x1a;
pub const IOSF_PORT_FLISDSI: c_uint = 0x1b;
pub const IOSF_PORT_GPIO_SC: c_uint = 0x48;
pub const IOSF_PORT_GPIO_SUS: c_uint = 0xa8;
pub const IOSF_PORT_CCU: c_uint = 0xa9;
pub const CHV_IOSF_PORT_GPIO_N: c_uint = 0x13;
pub const CHV_IOSF_PORT_GPIO_SE: c_uint = 0x48;
pub const CHV_IOSF_PORT_GPIO_E: c_uint = 0xa8;
pub const CHV_IOSF_PORT_GPIO_SW: c_uint = 0xb2;

// DPIO registers
pub const DPIO_DEVFN: c_int = 0;
//
// Fence registers
// [0-7]  @ 0x2000 gen2,gen3
// [8-15] @ 0x3000 945,g33,pnv
//
// [0-15] @ 0x3000 gen4,gen5
//
// [0-15] @ 0x100000 gen6,vlv,chv
// [0-31] @ 0x100000 gen7+
//

pub const I830_FENCE_START_MASK: c_uint = 0x07f80000;
pub const I830_FENCE_TILING_Y_SHIFT: c_int = 12;

pub const I830_FENCE_PITCH_SHIFT: c_int = 4;

pub const I915_FENCE_MAX_PITCH_VAL: c_int = 4;
pub const I830_FENCE_MAX_PITCH_VAL: c_int = 6;

pub const I915_FENCE_START_MASK: c_uint = 0x0ff00000;

pub const I965_FENCE_PITCH_SHIFT: c_int = 2;
pub const I965_FENCE_TILING_Y_SHIFT: c_int = 1;

pub const I965_FENCE_MAX_PITCH_VAL: c_uint = 0x0400;

pub const GEN6_FENCE_PITCH_SHIFT: c_int = 32;
pub const GEN7_FENCE_MAX_PITCH_VAL: c_uint = 0x0800;
// control register for cpu gtt access

//
// Instruction and interrupt control regs
//

pub const PGTBL_ADDRESS_LO_MASK: c_uint = 0xfffff000 /* bits [31:12] */;
pub const PGTBL_ADDRESS_HI_MASK: c_uint = 0x000000f0 /* bits [35:32] (gen4) */;

pub const RENDER_RING_BASE: c_uint = 0x02000;
pub const BSD_RING_BASE: c_uint = 0x04000;
pub const GEN6_BSD_RING_BASE: c_uint = 0x12000;
pub const GEN8_BSD2_RING_BASE: c_uint = 0x1c000;
pub const GEN11_BSD_RING_BASE: c_uint = 0x1c0000;
pub const GEN11_BSD2_RING_BASE: c_uint = 0x1c4000;
pub const GEN11_BSD3_RING_BASE: c_uint = 0x1d0000;
pub const GEN11_BSD4_RING_BASE: c_uint = 0x1d4000;
pub const XEHP_BSD5_RING_BASE: c_uint = 0x1e0000;
pub const XEHP_BSD6_RING_BASE: c_uint = 0x1e4000;
pub const XEHP_BSD7_RING_BASE: c_uint = 0x1f0000;
pub const XEHP_BSD8_RING_BASE: c_uint = 0x1f4000;
pub const VEBOX_RING_BASE: c_uint = 0x1a000;
pub const GEN11_VEBOX_RING_BASE: c_uint = 0x1c8000;
pub const GEN11_VEBOX2_RING_BASE: c_uint = 0x1d8000;
pub const XEHP_VEBOX3_RING_BASE: c_uint = 0x1e8000;
pub const XEHP_VEBOX4_RING_BASE: c_uint = 0x1f8000;
pub const MTL_GSC_RING_BASE: c_uint = 0x11a000;
pub const GEN12_COMPUTE0_RING_BASE: c_uint = 0x1a000;
pub const GEN12_COMPUTE1_RING_BASE: c_uint = 0x1c000;
pub const GEN12_COMPUTE2_RING_BASE: c_uint = 0x1e000;
pub const GEN12_COMPUTE3_RING_BASE: c_uint = 0x26000;
pub const BLT_RING_BASE: c_uint = 0x22000;
pub const XEHPC_BCS1_RING_BASE: c_uint = 0x3e0000;
pub const XEHPC_BCS2_RING_BASE: c_uint = 0x3e2000;
pub const XEHPC_BCS3_RING_BASE: c_uint = 0x3e4000;
pub const XEHPC_BCS4_RING_BASE: c_uint = 0x3e6000;
pub const XEHPC_BCS5_RING_BASE: c_uint = 0x3e8000;
pub const XEHPC_BCS6_RING_BASE: c_uint = 0x3ea000;
pub const XEHPC_BCS7_RING_BASE: c_uint = 0x3ec000;
pub const XEHPC_BCS8_RING_BASE: c_uint = 0x3ee000;
pub const DG1_GSC_HECI1_BASE: c_uint = 0x00258000;
pub const DG1_GSC_HECI2_BASE: c_uint = 0x00259000;
pub const DG2_GSC_HECI1_BASE: c_uint = 0x00373000;
pub const DG2_GSC_HECI2_BASE: c_uint = 0x00374000;
pub const MTL_GSC_HECI1_BASE: c_uint = 0x00116000;
pub const MTL_GSC_HECI2_BASE: c_uint = 0x00117000;

//
// The FWSTS register values are FW defined and can be different between
// HECI1 and HECI2
//
pub const HECI_FWSTS1: c_uint = 0xc40;

pub const HECI1_FWSTS1_CURRENT_STATE_RESET: c_int = 0;
pub const HECI1_FWSTS1_PROXY_STATE_NORMAL: c_int = 5;

pub const HECI_FWSTS2: c_uint = 0xc48;
pub const HECI_FWSTS3: c_uint = 0xc60;
pub const HECI_FWSTS4: c_uint = 0xc64;
pub const HECI_FWSTS5: c_uint = 0xc68;

pub const HECI_FWSTS6: c_uint = 0xc6c;
// the FWSTS regs are 1-based, so we use -base for index 0 to get an invalid reg

pub const GTT_CACHE_EN_ALL: c_uint = 0xF0007FFF;

// L3, CVS, ZTLB, RCC, CASC LRA min, max values

pub const GEN7_LRA_LIMITS_REG_NUM: c_int = 13;

pub const MM_BURST_LENGTH: c_uint = 0x00700000;
pub const MM_FIFO_WATERMARK: c_uint = 0x0001F000;
pub const LM_BURST_LENGTH: c_uint = 0x00000700;
pub const LM_FIFO_WATERMARK: c_uint = 0x0000001F;

//
// Make render/texture TLB fetches lower priority than associated data
// fetches. This is not turned on by default.
//

// Isoch request wait on GTT enable (Display A/B/C streams).
// Make isoch requests stall on the TLB update. May cause
// display underruns (test mode only)
//

// Block grant count for isoch requests when block count is
// set to a finite value.
//

// Enable render writes to complete in C2/C3/C4 power states.
// If this isn't enabled, render writes are prevented in low
// power states. That seems bad to me.
//

// This acknowledges an async flip immediately instead
// of waiting for 2TLB fetches.
//

// Enables non-sequential data reads through arbiter
//

// Disable FSB snooping of cacheable write cycles from binner/render
// command stream
//

// Arbiter time slice for non-isoch streams

// Low priority grace period page size

// Disable display A/B trickle feed

// Set display plane priority

// On modern GEN architectures interrupt control consists of two sets
// of registers. The first set pertains to the ring generating the
// interrupt. The second control is for the functional block generating the
// interrupt. These are PM, GT, DE, etc.
//
// Luckily *knocks on wood* all the ring interrupt bits match up with the
// GT interrupt bits, so we don't need to duplicate the defines.
//
// These defines should cover us well from SNB->HSW with minor exceptions
// it can also work on ILK.
//

// These are all the "old" interrupts

pub const GEN7_FF_SCHED_MASK: c_uint = 0x0077070;

// This bit must be unset on 855,865

// This bit must be set on 855,865.

// This bit must always be set on 965G/965GM

// This bit must always be set on 965G

pub const GT0_PERF_LIMIT_REASONS_MASK: c_uint = 0xde3;

pub const CLK_CTL2_CZCOUNT_30NS_SHIFT: c_int = 28;

pub const GEN8_RCS_IRQ_SHIFT: c_int = 0;
pub const GEN8_BCS_IRQ_SHIFT: c_int = 16;

pub const GEN8_VECS_IRQ_SHIFT: c_int = 0;
pub const GEN8_WD_IRQ_SHIFT: c_int = 16;

pub const EDRAM_ENABLED: c_uint = 0x1;

pub const GEN6_PCODE_FREQ_IA_RATIO_SHIFT: c_int = 8;
pub const GEN6_PCODE_FREQ_RING_RATIO_SHIFT: c_int = 16;

pub const STOLEN_ACCESS_ALLOWED: c_uint = 0x1;
// IVYBRIDGE DPF

// These are the 4 32-bit write offset registers for each stream
// output buffer.  It determines the offset from the
// 3DSTATE_SO_BUFFERs that the next streamed vertex output goes to.
//

pub const GEN9_TIMESTAMP_OVERRIDE_US_COUNTER_DIVIDER_SHIFT: c_int = 0;
pub const GEN9_TIMESTAMP_OVERRIDE_US_COUNTER_DIVIDER_MASK: c_uint = 0x3ff;
pub const GEN9_TIMESTAMP_OVERRIDE_US_COUNTER_DENOMINATOR_SHIFT: c_int = 12;

pub const MTL_MEDIA_GSI_BASE: c_uint = 0x380000;

