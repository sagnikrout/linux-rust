//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gt_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

pub const VLV_GUNIT_BASE: c_uint = 0x180000;
//
// The perf control registers are technically multicast registers, but the
// driver never needs to read/write them directly; we only use them to build
// lists of registers (where they're mixed in with other non-MCR registers)
// and then operate on the offset directly.  For now we'll just define them
// as non-multicast so we can place them on the same list, but we may want
// to try to come up with a better way to handle heterogeneous lists of
// registers in the future.
//

// MTL workpoint reg to get core C state and actual freq of 3D, SAMedia

pub const MTL_CC0: c_uint = 0x0;
pub const MTL_CC6: c_uint = 0x3;

// RPM unit config (Gen8+)

// RCP unit config (Gen8+)

pub const RC6_CTX_BASE_MASK: c_uint = 0xFFFFFFF0;

//
// On GEN4, only the render ring INSTDONE exists and has a different
// layout than the GEN7+ version.
// The GEN2 counterpart of this register is GEN2_INSTDONE.
//

pub const HWS_ADDRESS_MASK: c_uint = 0xfffff000;
pub const HWS_START_ADDRESS_SHIFT: c_int = 4;

// GM45+ chicken bits -- debug workaround bits that may be required
// for various sorts of correct behavior.  The top 16 bits of each are
// the enables for writing to the corresponding low bit.
//

// Disables pipelining of read flushes past the SF-WIZ interface.
// Required on all Ironlake steppings according to the B-Spec, but the
// particular danger of not doing so is not specified.
//

// HSW only
pub const HSW_SELECTIVE_READ_ADDRESSING_SHIFT: c_int = 2;

pub const HSW_SELECTIVE_WRITE_ADDRESS_SHIFT: c_int = 4;

// HSW+

// Gen8
pub const GEN8_SELECTIVE_WRITE_ADDRESS_SHIFT: c_int = 4;

pub const GEN8_SELECTIVE_WRITE_ADDRESS_SHIFT: c_int = 4;

pub const GEN8_SELECTIVE_READ_SUBSLICE_SELECT_SHIFT: c_int = 9;

pub const GEN8_SELECTIVE_READ_SLICE_SELECT_SHIFT: c_int = 11;

// chicken reg for WaConextSwitchWithConcurrentTLBInvalidate

// WaClearTdlStateAckDirtyBits

//
// Logical Context regs
//
// Notes on SNB/IVB/VLV context size:
// - Power context is saved elsewhere (LLC or stolen)
// - Ring/execlist context is saved on SNB, not on IVB
// - Extended context size already includes render context size
// - We always need to follow the extended context size.
// SNB BSpec has comments indicating that we should use the
// render context size instead if execlists are disabled, but
// based on empirical testing that's just nonsense.
// - Pipelined/VF state is saved on SNB/IVB respectively
// - GT1 size just indicates how much of render context
// doesn't need saving on GT1
//

pub const GAMW_ECO_ENABLE_64K_IPS_FIELD: c_uint = 0xF;

pub const _RING_FAULT_REG_RCS: c_uint = 0x4094;
pub const _RING_FAULT_REG_VCS: c_uint = 0x4194;
pub const _RING_FAULT_REG_BCS: c_uint = 0x4294;
pub const _RING_FAULT_REG_VECS: c_uint = 0x4394;

pub const XEHP_TILE_LMEM_RANGE_SHIFT: c_int = 8;

pub const XEHP_CCS_BASE_SHIFT: c_int = 8;

// gamt regs

pub const GEN8_L3_LRA_1_GPGPU_DEFAULT_VALUE_BDW: c_uint = 0x67F1427F /* max/min for LRA1/2 */;
pub const GEN8_L3_LRA_1_GPGPU_DEFAULT_VALUE_CHV: c_uint = 0x5FF101FF /* max/min for LRA1/2 */;
pub const GEN9_L3_LRA_1_GPGPU_DEFAULT_VALUE_SKL: c_uint = 0x67F1427F /*    "        " */;
pub const GEN9_L3_LRA_1_GPGPU_DEFAULT_VALUE_BXT: c_uint = 0x5FF101FF /*    "        " */;

// There are the 4 64-bit counter registers, one for each stream output

// GEN7 chicken

// GEN8 chicken

pub const GEN6_MBC_SNPCR_SHIFT: c_int = 21;

// WaCatErrorRejectionIssue

// Fuse readout registers for GT

pub const GEN10_L3BANK_PAIR_COUNT: c_int = 4;
pub const GEN10_L3BANK_MASK: c_uint = 0x0F;
// on Xe_HP the same fuses indicates mslices instead of L3 banks
pub const GEN12_MAX_MSLICES: c_int = 4;

pub const HSW_F1_EU_DIS_10EUS: c_int = 0;
pub const HSW_F1_EU_DIS_8EUS: c_int = 1;
pub const HSW_F1_EU_DIS_6EUS: c_int = 2;

pub const GEN10_EU_DIS_SS_MASK: c_uint = 0xff;

// GEN11 changed all bit defs except for FULL & RENDER

pub const GEN9_SW_REQ_UNSLICE_RATIO_SHIFT: c_int = 23;

pub const GEN6_RPSWCTL_SHIFT: c_int = 9;

pub const GEN6_RP_EI_MASK: c_uint = 0xffffff;

pub const RC_SW_TARGET_STATE_SHIFT: c_int = 16;

// GPM unit config (Gen9+)

// GPM MSG_IDLE

pub const MSG_IDLE_FW_SHIFT: c_int = 9;

pub const VLV_B0_WA_L3SQCREG1_VALUE: c_uint = 0x00D30000;

pub const GEN7_WA_FOR_GEN7_L3_CONTROL: c_uint = 0x3C47FF8C;

// MOCS (Memory Object Control State) registers

pub const LNCFCMOCS_REG_COUNT: c_int = 32;

pub const GEN7_WA_L3_CHICKEN_MODE: c_uint = 0x20000000;

pub const GEN7_L3LOG_SIZE: c_uint = 0x80;

//
// Note that on CHV the following has an off-by-one error wrt. to BSpec.
// Using the formula in BSpec leads to a hang, while the formula here works
// fine and matches the formulas for all other platforms. A BSpec change
// request has been filed to clarify this.
//

pub const __GEN9_RCS0_MOCS0: c_uint = 0xc800;

pub const __GEN9_VCS0_MOCS0: c_uint = 0xc900;

pub const __GEN9_VCS1_MOCS0: c_uint = 0xca00;

pub const __GEN9_VECS0_MOCS0: c_uint = 0xcb00;

pub const __GEN9_BCS0_MOCS0: c_uint = 0xcc00;

// see GEN8_FAULT_TLB_DATA0/1

// see GEN8_RING_FAULT_REG

//
// We have both ENABLE and DISABLE defines below using the same bit because the
// meaning depends on the target platform. There are no platform prefix for them
// because different steppings of DG2 pick one or the other semantics.
//

pub const __GEN11_VCS2_MOCS0: c_uint = 0x10000;

pub const PXVFREQ_PX_MASK: c_uint = 0x7f000000;
pub const PXVFREQ_PX_SHIFT: c_int = 24;

pub const VIDFREQ_P0_MASK: c_uint = 0x1f000000;
pub const VIDFREQ_P0_SHIFT: c_int = 24;
pub const VIDFREQ_P0_CSCLK_MASK: c_uint = 0x00f00000;
pub const VIDFREQ_P0_CSCLK_SHIFT: c_int = 20;
pub const VIDFREQ_P0_CRCLK_MASK: c_uint = 0x000f0000;
pub const VIDFREQ_P0_CRCLK_SHIFT: c_int = 16;
pub const VIDFREQ_P1_MASK: c_uint = 0x00001f00;
pub const VIDFREQ_P1_SHIFT: c_int = 8;
pub const VIDFREQ_P1_CSCLK_MASK: c_uint = 0x000000f0;
pub const VIDFREQ_P1_CSCLK_SHIFT: c_int = 4;
pub const VIDFREQ_P1_CRCLK_MASK: c_uint = 0x0000000f;

pub const INTTOEXT_MAP3_SHIFT: c_int = 24;

pub const INTTOEXT_MAP2_SHIFT: c_int = 16;

pub const INTTOEXT_MAP1_SHIFT: c_int = 8;

pub const INTTOEXT_MAP0_SHIFT: c_int = 0;

pub const MEMCTL_CMD_MASK: c_uint = 0xe000;
pub const MEMCTL_CMD_SHIFT: c_int = 13;
pub const MEMCTL_CMD_RCLK_OFF: c_int = 0;
pub const MEMCTL_CMD_RCLK_ON: c_int = 1;
pub const MEMCTL_CMD_CHFREQ: c_int = 2;
pub const MEMCTL_CMD_CHVID: c_int = 3;
pub const MEMCTL_CMD_VMMOFF: c_int = 4;
pub const MEMCTL_CMD_VMMON: c_int = 5;

pub const MEMCTL_FREQ_MASK: c_uint = 0x0f00 /* jitter, from 0-15 */;
pub const MEMCTL_FREQ_SHIFT: c_int = 8;

pub const MEMCTL_TGT_VID_MASK: c_uint = 0x007f;

pub const MEM_RSEXIT_MASK: c_uint = 0xc000;
pub const MEM_RSEXIT_SHIFT: c_int = 14;
pub const MEM_CONT_BUSY_MASK: c_uint = 0x3000;
pub const MEM_CONT_BUSY_SHIFT: c_int = 12;
pub const MEM_AVG_BUSY_MASK: c_uint = 0x0c00;
pub const MEM_AVG_BUSY_SHIFT: c_int = 10;
pub const MEM_EVAL_CHG_MASK: c_uint = 0x0300;
pub const MEM_EVAL_BUSY_SHIFT: c_int = 8;
pub const MEM_MON_IDLE_MASK: c_uint = 0x00c0;
pub const MEM_MON_IDLE_SHIFT: c_int = 6;
pub const MEM_UP_EVAL_MASK: c_uint = 0x0030;
pub const MEM_UP_EVAL_SHIFT: c_int = 4;
pub const MEM_DOWN_EVAL_MASK: c_uint = 0x000c;
pub const MEM_DOWN_EVAL_SHIFT: c_int = 2;
pub const MEM_SW_CMD_MASK: c_uint = 0x0003;
pub const MEM_INT_STEER_GFX: c_int = 0;
pub const MEM_INT_STEER_CMR: c_int = 1;
pub const MEM_INT_STEER_SMI: c_int = 2;
pub const MEM_INT_STEER_SCI: c_int = 3;

pub const MEMMODE_BOOST_FREQ_MASK: c_uint = 0x0f000000 /* jitter for boost, 0-15 */;
pub const MEMMODE_BOOST_FREQ_SHIFT: c_int = 24;
pub const MEMMODE_IDLE_MODE_MASK: c_uint = 0x00030000;
pub const MEMMODE_IDLE_MODE_SHIFT: c_int = 16;
pub const MEMMODE_IDLE_MODE_EVAL: c_int = 0;
pub const MEMMODE_IDLE_MODE_CONT: c_int = 1;

pub const MEMMODE_FSTART_MASK: c_uint = 0x00000f00 /* starting jitter, 0-15 */;
pub const MEMMODE_FSTART_SHIFT: c_int = 8;
pub const MEMMODE_FMAX_MASK: c_uint = 0x000000f0 /* max jitter, 0-15 */;
pub const MEMMODE_FMAX_SHIFT: c_int = 4;
pub const MEMMODE_FMIN_MASK: c_uint = 0x0000000f /* min jitter, 0-15 */;

pub const SWFREQ_MASK: c_uint = 0x0380 /* P0-7 */;
pub const SWFREQ_SHIFT: c_int = 7;
pub const TARVID_MASK: c_uint = 0x001f;

pub const MEMSTAT_VID_MASK: c_uint = 0x7f00;
pub const MEMSTAT_VID_SHIFT: c_int = 8;

pub const MEMSTAT_SRC_CTL_MASK: c_uint = 0x0003;
pub const MEMSTAT_SRC_CTL_CORE: c_int = 0;
pub const MEMSTAT_SRC_CTL_TRB: c_int = 1;
pub const MEMSTAT_SRC_CTL_THM: c_int = 2;
pub const MEMSTAT_SRC_CTL_STDBY: c_int = 3;

pub const ECR_CAP_MASK: c_uint = 0x0000001f /* Event range, 0-31 */;

pub const LCFUSE_HIV_MASK: c_uint = 0x000000ff;

//
// For Gen11 these are in the upper word of the GPM_WGBOXPERF
// registers. Shifting is handled on accessing the imr and ier.
//

pub const GEN7_GT_SCRATCH_REG_NUM: c_int = 8;

pub const GT_FIFO_FREE_ENTRIES_MASK: c_uint = 0x7f;
pub const GT_FIFO_NUM_RESERVED_ENTRIES: c_int = 20;

pub const GEN6_GT_THREAD_STATUS_CORE_MASK: c_uint = 0x7;

pub const GEN6_RCn_MASK: c_int = 7;
pub const GEN6_RC0: c_int = 0;
pub const GEN6_RC3: c_int = 2;
pub const GEN6_RC6: c_int = 3;
pub const GEN6_RC7: c_int = 4;

pub const GEN8_LSLICESTAT_MASK: c_uint = 0x7;

// irq instances for OTHER_CLASS
pub const OTHER_GUC_INSTANCE: c_int = 0;
pub const OTHER_GTPM_INSTANCE: c_int = 1;
pub const OTHER_GSC_HECI_2_INSTANCE: c_int = 3;
pub const OTHER_KCR_INSTANCE: c_int = 4;
pub const OTHER_GSC_INSTANCE: c_int = 6;
pub const OTHER_MEDIA_GUC_INSTANCE: c_int = 16;
pub const OTHER_MEDIA_GTPM_INSTANCE: c_int = 17;

//
// Standalone Media's non-engine GT registers are located at their regular GT
// offsets plus 0x380000.  This extra offset is stored inside the intel_uncore
// structure so that the existing code can be used for both GTs without
// modification.
//
pub const MTL_MEDIA_GSI_BASE: c_uint = 0x380000;
