//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_regs.h
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
// Copyright © 2025 Intel Corporation

pub const _GEN7_PIPEA_DE_LOAD_SL: c_uint = 0x70068;
pub const _GEN7_PIPEB_DE_LOAD_SL: c_uint = 0x71068;

pub const _BXT_PHY_CTL_DDI_A: c_uint = 0x64C00;
pub const _BXT_PHY_CTL_DDI_B: c_uint = 0x64C10;
pub const _BXT_PHY_CTL_DDI_C: c_uint = 0x64C20;

pub const _PHY_CTL_FAMILY_DDI: c_uint = 0x64C90;
pub const _PHY_CTL_FAMILY_EDP: c_uint = 0x64C80;
pub const _PHY_CTL_FAMILY_DDI_C: c_uint = 0x64CA0;

// UAIMI scratch pad register 1

// SKL VccIO mask
pub const SKL_VCCIO_MASK: c_uint = 0x1;
// SKL balance leg register

// I_boost values

// Balance leg disable bits
pub const BALANCE_LEG_DISABLE_SHIFT: c_int = 23;

// Note that HBLANK events are reserved on bdw+

// Note that PIPEC is not a simple translation of PIPEA/PIPEB

pub const _MBUS_ABOX0_CTL: c_uint = 0x45038;
pub const _MBUS_ABOX1_CTL: c_uint = 0x45048;
pub const _MBUS_ABOX2_CTL: c_uint = 0x4504C;

//
// Clock control & power management
//
pub const _DPLL_A: c_uint = 0x6014;
pub const _DPLL_B: c_uint = 0x6018;
pub const _CHV_DPLL_C: c_uint = 0x6030;

pub const VGA0_PD_P1_SHIFT: c_int = 0;

pub const VGA1_PD_P1_SHIFT: c_int = 8;

pub const DPLL_P2_CLOCK_DIV_MASK: c_uint = 0x03000000 /* i915 */;
pub const DPLL_FPA01_P1_POST_DIV_MASK: c_uint = 0x00ff0000 /* i915 */;
pub const DPLL_FPA01_P1_POST_DIV_MASK_PINEVIEW: c_uint = 0x00ff8000 /* Pineview */;

pub const DPLL_FPA01_P1_POST_DIV_MASK_I830: c_uint = 0x001f0000;

// Required on all Ironlake and Sandybridge according to the B-Spec.

//
// This bit must be set on the 830 to prevent hangs when turning off the
// overlay scaler.
//

// Additional CHV pll/phy registers

pub const PHY_LDO_DELAY_0NS: c_uint = 0x0;
pub const PHY_LDO_DELAY_200NS: c_uint = 0x1;
pub const PHY_LDO_DELAY_600NS: c_uint = 0x2;

pub const PHY_CH_SU_PSR: c_uint = 0x1;
pub const PHY_CH_DEEP_PSR: c_uint = 0x7;

//
// The i830 generation, in LVDS mode, defines P1 as the bit number set within
// this field (only one bit may be set).
//
pub const DPLL_FPA01_P1_POST_DIV_MASK_I830_LVDS: c_uint = 0x003f0000;
pub const DPLL_FPA01_P1_POST_DIV_SHIFT: c_int = 16;
pub const DPLL_FPA01_P1_POST_DIV_SHIFT_PINEVIEW: c_int = 15;
// i830, required in DVO non-gang

pub const PLL_LOAD_PULSE_PHASE_SHIFT: c_int = 9;
// Ironlake

//
// Parallel to Serial Load Pulse phase selection.
// Selects the phase for the 10X DPLL clock for the PCIe
// digital display port. The range is 4 to 13; 10 or more
// is just a flip delay. The default is 6
//

//
// SDVO multiplier for 945G/GM. Not used on 965.
//
pub const SDVO_MULTIPLIER_MASK: c_uint = 0x000000ff;
pub const SDVO_MULTIPLIER_SHIFT_HIRES: c_int = 4;
pub const SDVO_MULTIPLIER_SHIFT_VGA: c_int = 0;
pub const _DPLL_A_MD: c_uint = 0x601c;
pub const _DPLL_B_MD: c_uint = 0x6020;
pub const _CHV_DPLL_C_MD: c_uint = 0x603c;

//
// UDI pixel divider, controlling how many pixels are stuffed into a packet.
//
// Value is pixels minus 1.  Must be set to 1 pixel for SDVO.
//
pub const DPLL_MD_UDI_DIVIDER_MASK: c_uint = 0x3f000000;
pub const DPLL_MD_UDI_DIVIDER_SHIFT: c_int = 24;
// UDI pixel divider for VGA, same as DPLL_MD_UDI_DIVIDER_MASK.
pub const DPLL_MD_VGA_UDI_DIVIDER_MASK: c_uint = 0x003f0000;
pub const DPLL_MD_VGA_UDI_DIVIDER_SHIFT: c_int = 16;
//
// SDVO/UDI pixel multiplier.
//
// SDVO requires that the bus clock rate be between 1 and 2 Ghz, and the bus
// clock rate is 10 times the DPLL clock.  At low resolution/refresh rate
// modes, the bus rate would be below the limits, so SDVO allows for stuffing
// dummy bytes in the datastream at an increased clock rate, with both sides of
// the link knowing how many bytes are fill.
//
// So, for a mode with a dotclock of 65Mhz, we would want to double the clock
// rate to 130Mhz to get a bus rate of 1.30Ghz.  The DPLL clock rate would be
// set to 130Mhz, and the SDVO multiplier set to 2x in this register and
// through an SDVO command.
//
// This register field has values of multiplication factor minus 1, with
// a maximum multiplier of 5 for SDVO.
//
pub const DPLL_MD_UDI_MULTIPLIER_MASK: c_uint = 0x00003f00;
pub const DPLL_MD_UDI_MULTIPLIER_SHIFT: c_int = 8;
//
// SDVO/UDI pixel multiplier for VGA, same as DPLL_MD_UDI_MULTIPLIER_MASK.
// This best be set to the default value (3) or the CRT won't work. No,
// I don't entirely understand what this does...
//
pub const DPLL_MD_VGA_UDI_MULTIPLIER_MASK: c_uint = 0x0000003f;
pub const DPLL_MD_VGA_UDI_MULTIPLIER_SHIFT: c_int = 0;

pub const _FPA0: c_uint = 0x6040;
pub const _FPA1: c_uint = 0x6044;
pub const _FPB0: c_uint = 0x6048;
pub const _FPB1: c_uint = 0x604c;

pub const FP_N_DIV_MASK: c_uint = 0x003f0000;
pub const FP_N_PINEVIEW_DIV_MASK: c_uint = 0x00ff0000;
pub const FP_N_DIV_SHIFT: c_int = 16;
pub const FP_M1_DIV_MASK: c_uint = 0x00003f00;
pub const FP_M1_DIV_SHIFT: c_int = 8;
pub const FP_M2_DIV_MASK: c_uint = 0x0000003f;
pub const FP_M2_PINEVIEW_DIV_MASK: c_uint = 0x000000ff;
pub const FP_M2_DIV_SHIFT: c_int = 0;

pub const CDCLK_FREQ_SHIFT: c_int = 4;

pub const CZCLK_FREQ_MASK: c_uint = 0xf;

//
// Overlay regs
//

//
// GEN9 clock gating regs
//

pub const _CLKGATE_DIS_PSL_A: c_uint = 0x46520;
pub const _CLKGATE_DIS_PSL_B: c_uint = 0x46524;
pub const _CLKGATE_DIS_PSL_C: c_uint = 0x46528;

pub const _CLKGATE_DIS_PSL_EXT_A: c_uint = 0x4654C;
pub const _CLKGATE_DIS_PSL_EXT_B: c_uint = 0x46550;

//
// Display engine regs
//
// Pipe/transcoder A timing regs
pub const _TRANS_HTOTAL_A: c_uint = 0x60000;
pub const _TRANS_HTOTAL_B: c_uint = 0x61000;

pub const _TRANS_HBLANK_A: c_uint = 0x60004;
pub const _TRANS_HBLANK_B: c_uint = 0x61004;

pub const _TRANS_HSYNC_A: c_uint = 0x60008;
pub const _TRANS_HSYNC_B: c_uint = 0x61008;

pub const _TRANS_VTOTAL_A: c_uint = 0x6000c;
pub const _TRANS_VTOTAL_B: c_uint = 0x6100c;

pub const _TRANS_VBLANK_A: c_uint = 0x60010;
pub const _TRANS_VBLANK_B: c_uint = 0x61010;

pub const _TRANS_VSYNC_A: c_uint = 0x60014;
pub const _TRANS_VSYNC_B: c_uint = 0x61014;

pub const _PIPEASRC: c_uint = 0x6001c;
pub const _PIPEBSRC: c_uint = 0x6101c;

pub const _BCLRPAT_A: c_uint = 0x60020;
pub const _BCLRPAT_B: c_uint = 0x61020;

pub const _TRANS_VSYNCSHIFT_A: c_uint = 0x60028;
pub const _TRANS_VSYNCSHIFT_B: c_uint = 0x61028;

pub const _TRANS_MULT_A: c_uint = 0x6002c;
pub const _TRANS_MULT_B: c_uint = 0x6102c;

// Hotplug control (945+ only)

// must use period 64 on GM45 according to docs

// HDMI/DP bits are g4x+

// CRT/TV common between gen3+

// SDVO is different across gen3/4

//
// Bspec seems to be seriously misleaded about the SDVO hpd bits on i965g/gm,
// since reality corrobates that they're the same as on gen3. But keep these
// bits here (and the comment!) to help any other lost wanderers back onto the
// right tracks.
//

// SDVO and HDMI port control.
// The same register may be used for SDVO or HDMI
pub const _GEN3_SDVOB: c_uint = 0x61140;
pub const _GEN3_SDVOC: c_uint = 0x61160;

// Gen 3 SDVO bits:

pub const SDVO_PIPE_SEL_SHIFT: c_int = 30;

//
// 915G/GM SDVO pixel multiplier.
// Programmed value is multiplier - 1, up to 5x.
// \sa DPLL_MD_UDI_MULTIPLIER_MASK
//

pub const SDVO_PORT_MULTIPLY_SHIFT: c_int = 23;

// Bits to be preserved when writing

// Gen 4 SDVO/HDMI bits:

// VSYNC/HSYNC bits new with 965, default is to be set

// Gen 5 (IBX) SDVO/HDMI bits:

// Gen 6 (CPT) SDVO/HDMI bits:
pub const SDVO_PIPE_SEL_SHIFT_CPT: c_int = 29;

// CHV SDVO/HDMI bits:
pub const SDVO_PIPE_SEL_SHIFT_CHV: c_int = 24;

// Video Data Island Packet control

// Read the description of VIDEO_DIP_DATA (before Haswell) or VIDEO_DIP_ECC
// (Haswell and newer) to see which VIDEO_DIP_DATA byte corresponds to each byte
// of the infoframe structure specified by CEA-861.
pub const VIDEO_DIP_DATA_SIZE: c_int = 32;
pub const VIDEO_DIP_ASYNC_DATA_SIZE: c_int = 36;
pub const VIDEO_DIP_GMP_DATA_SIZE: c_int = 36;
pub const VIDEO_DIP_VSC_DATA_SIZE: c_int = 36;
pub const VIDEO_DIP_PPS_DATA_SIZE: c_int = 132;

// Pre HSW:

// HSW and later:

pub const VSC_SELECT_SHIFT: c_int = 25;

// ADL and later:

// Display Port

pub const DP_PIPE_SEL_SHIFT_CHV: c_int = 16;

//
// Computing GMCH M and N values for the Display Port link
//
// GMCH M/N = dot clock * bytes per pixel / ls_clk * # of lanes
//
// ls_clk (we assume) is the DP link clock (1.62 or 2.7 GHz)
//
// The GMCH value is used internally
//
// bytes_per_pixel is the number of bytes coming out of the plane,
// which is after the LUTs, so we want the bytes for our color format.
// For our current usage, this is always 3, one byte for R, G and B.
//
pub const _PIPEA_DATA_M_G4X: c_uint = 0x70050;
pub const _PIPEB_DATA_M_G4X: c_uint = 0x71050;

// Transfer unit size for display port - 1, default is 0x3f (for TU size 64)

pub const _PIPEA_DATA_N_G4X: c_uint = 0x70054;
pub const _PIPEB_DATA_N_G4X: c_uint = 0x71054;

//
// Computing Link M and N values for the Display Port link
//
// Link M / N = pixel_clock / ls_clk
//
// (the DP spec calls pixel_clock the 'strm_clk')
//
// The Link value is transmitted in the Main Stream
// Attributes and VB-ID.
//
pub const _PIPEA_LINK_M_G4X: c_uint = 0x70060;
pub const _PIPEB_LINK_M_G4X: c_uint = 0x71060;

pub const _PIPEA_LINK_N_G4X: c_uint = 0x70064;
pub const _PIPEB_LINK_N_G4X: c_uint = 0x71064;

// Pipe A
pub const _PIPEADSL: c_uint = 0x70000;

pub const _TRANSACONF: c_uint = 0x70008;

//
// ilk+: PF/D=progressive fetch/display, IF/D=interlaced fetch/display,
// DBL=power saving pixel doubling, PF-ID* requires panel fitter
//

pub const TRANSCONF_PIXEL_COUNT_SCALING_X4: c_int = 1;
pub const _PIPEASTAT: c_uint = 0x70024;

pub const PIPESTAT_INT_ENABLE_MASK: c_uint = 0x7fff0000;
pub const PIPESTAT_INT_STATUS_MASK: c_uint = 0x0000ffff;
pub const _PIPE_ARB_CTL_A: c_uint = 0x70028 /* icl+ */;

pub const _PIPE_MISC_A: c_uint = 0x70030;
pub const _PIPE_MISC_B: c_uint = 0x71030;

//
// For Display < 13, Bits 5-7 of PIPE MISC represent DITHER BPC with
// valid values of: 6, 8, 10 BPC.
// ADLP+, the bits 5-7 represent PORT OUTPUT BPC with valid values of:
// 6, 8, 10, 12 BPC.
//

pub const _PIPE_MISC2_A: c_uint = 0x7002C;
pub const _PIPE_MISC2_B: c_uint = 0x7102C;

pub const _UNDERRUN_DBG1_A: c_uint = 0x70064;
pub const _UNDERRUN_DBG1_B: c_uint = 0x71064;

pub const _UNDERRUN_DBG2_A: c_uint = 0x70068;
pub const _UNDERRUN_DBG2_B: c_uint = 0x71068;

//
// The two pipe frame counter registers are not synchronized, so
// reading a stable value is somewhat tricky. The following code
// should work:
//
// do {
// high1 = ((INREG(PIPEAFRAMEHIGH) & PIPE_FRAME_HIGH_MASK) >>
// PIPE_FRAME_HIGH_SHIFT;
// low1 =  ((INREG(PIPEAFRAMEPIXEL) & PIPE_FRAME_LOW_MASK) >>
// PIPE_FRAME_LOW_SHIFT);
// high2 = ((INREG(PIPEAFRAMEHIGH) & PIPE_FRAME_HIGH_MASK) >>
// PIPE_FRAME_HIGH_SHIFT);
// } while (high1 != high2);
// frame = (high1 << 8) | low1;
//
pub const _PIPEAFRAMEHIGH: c_uint = 0x70040;

pub const PIPE_FRAME_HIGH_MASK: c_uint = 0x0000ffff;
pub const PIPE_FRAME_HIGH_SHIFT: c_int = 0;
pub const _PIPEAFRAMEPIXEL: c_uint = 0x70044;

pub const PIPE_FRAME_LOW_MASK: c_uint = 0xff000000;
pub const PIPE_FRAME_LOW_SHIFT: c_int = 24;
pub const PIPE_PIXEL_MASK: c_uint = 0x00ffffff;
pub const PIPE_PIXEL_SHIFT: c_int = 0;
// GM45+ just has to be different
pub const _PIPEA_FRMCOUNT_G4X: c_uint = 0x70040;

pub const _PIPEA_FLIPCOUNT_G4X: c_uint = 0x70044;

// CHV pipe B blender
pub const _CHV_BLEND_A: c_uint = 0x60a00;

pub const _CHV_CANVAS_A: c_uint = 0x60a04;

// Display/Sprite base address macros

//
// VBIOS flags
// gen2:
// [00:06] alm,mgm
// [10:16] all
// [30:32] alm,mgm
// gen3+:
// [00:0f] all
// [10:1f] all
// [30:32] all
//

// refresh rate hardware control

pub const RR_HW_LOW_POWER_FRAMES_MASK: c_uint = 0xff;
pub const RR_HW_HIGH_POWER_FRAMES_MASK: c_uint = 0xff00;
pub const _PIPEA_DATA_M1: c_uint = 0x60030;
pub const _PIPEB_DATA_M1: c_uint = 0x61030;

pub const _PIPEA_DATA_N1: c_uint = 0x60034;
pub const _PIPEB_DATA_N1: c_uint = 0x61034;

pub const _PIPEA_DATA_M2: c_uint = 0x60038;
pub const _PIPEB_DATA_M2: c_uint = 0x61038;

pub const _PIPEA_DATA_N2: c_uint = 0x6003c;
pub const _PIPEB_DATA_N2: c_uint = 0x6103c;

pub const _PIPEA_LINK_M1: c_uint = 0x60040;
pub const _PIPEB_LINK_M1: c_uint = 0x61040;

pub const _PIPEA_LINK_N1: c_uint = 0x60044;
pub const _PIPEB_LINK_N1: c_uint = 0x61044;

pub const _PIPEA_LINK_M2: c_uint = 0x60048;
pub const _PIPEB_LINK_M2: c_uint = 0x61048;

pub const _PIPEA_LINK_N2: c_uint = 0x6004c;
pub const _PIPEB_LINK_N2: c_uint = 0x6104c;

//
// Skylake scalers
//

pub const _PS_1A_CTRL: c_uint = 0x68180;
pub const _PS_2A_CTRL: c_uint = 0x68280;
pub const _PS_1B_CTRL: c_uint = 0x68980;
pub const _PS_2B_CTRL: c_uint = 0x68A80;
pub const _PS_1C_CTRL: c_uint = 0x69180;

pub const _PS_PWR_GATE_1A: c_uint = 0x68160;
pub const _PS_PWR_GATE_2A: c_uint = 0x68260;
pub const _PS_PWR_GATE_1B: c_uint = 0x68960;
pub const _PS_PWR_GATE_2B: c_uint = 0x68A60;
pub const _PS_PWR_GATE_1C: c_uint = 0x69160;

pub const _PS_WIN_POS_1A: c_uint = 0x68170;
pub const _PS_WIN_POS_2A: c_uint = 0x68270;
pub const _PS_WIN_POS_1B: c_uint = 0x68970;
pub const _PS_WIN_POS_2B: c_uint = 0x68A70;
pub const _PS_WIN_POS_1C: c_uint = 0x69170;

pub const _PS_WIN_SZ_1A: c_uint = 0x68174;
pub const _PS_WIN_SZ_2A: c_uint = 0x68274;
pub const _PS_WIN_SZ_1B: c_uint = 0x68974;
pub const _PS_WIN_SZ_2B: c_uint = 0x68A74;
pub const _PS_WIN_SZ_1C: c_uint = 0x69174;

pub const _PS_VSCALE_1A: c_uint = 0x68184;
pub const _PS_VSCALE_2A: c_uint = 0x68284;
pub const _PS_VSCALE_1B: c_uint = 0x68984;
pub const _PS_VSCALE_2B: c_uint = 0x68A84;
pub const _PS_VSCALE_1C: c_uint = 0x69184;

pub const _PS_HSCALE_1A: c_uint = 0x68190;
pub const _PS_HSCALE_2A: c_uint = 0x68290;
pub const _PS_HSCALE_1B: c_uint = 0x68990;
pub const _PS_HSCALE_2B: c_uint = 0x68A90;
pub const _PS_HSCALE_1C: c_uint = 0x69190;

pub const _PS_VPHASE_1A: c_uint = 0x68188;
pub const _PS_VPHASE_2A: c_uint = 0x68288;
pub const _PS_VPHASE_1B: c_uint = 0x68988;
pub const _PS_VPHASE_2B: c_uint = 0x68A88;
pub const _PS_VPHASE_1C: c_uint = 0x69188;

pub const _PS_HPHASE_1A: c_uint = 0x68194;
pub const _PS_HPHASE_2A: c_uint = 0x68294;
pub const _PS_HPHASE_1B: c_uint = 0x68994;
pub const _PS_HPHASE_2B: c_uint = 0x68A94;
pub const _PS_HPHASE_1C: c_uint = 0x69194;

pub const _PS_ECC_STAT_1A: c_uint = 0x681D0;
pub const _PS_ECC_STAT_2A: c_uint = 0x682D0;
pub const _PS_ECC_STAT_1B: c_uint = 0x689D0;
pub const _PS_ECC_STAT_2B: c_uint = 0x68AD0;
pub const _PS_ECC_STAT_1C: c_uint = 0x691D0;

pub const _PS_COEF_SET0_INDEX_1A: c_uint = 0x68198;
pub const _PS_COEF_SET0_INDEX_2A: c_uint = 0x68298;
pub const _PS_COEF_SET0_INDEX_1B: c_uint = 0x68998;
pub const _PS_COEF_SET0_INDEX_2B: c_uint = 0x68A98;

pub const _PS_COEF_SET0_DATA_1A: c_uint = 0x6819C;
pub const _PS_COEF_SET0_DATA_2A: c_uint = 0x6829C;
pub const _PS_COEF_SET0_DATA_1B: c_uint = 0x6899C;
pub const _PS_COEF_SET0_DATA_2B: c_uint = 0x68A9C;

// More Ivybridge lolz

// interrupts

// Display Internal Timeout Register

pub const _CHICKEN_PIPESL_1_A: c_uint = 0x420b0;
pub const _CHICKEN_PIPESL_1_B: c_uint = 0x420b4;

pub const _CHICKEN_TRANS_A: c_uint = 0x420c0;
pub const _CHICKEN_TRANS_B: c_uint = 0x420c4;
pub const _CHICKEN_TRANS_C: c_uint = 0x420c8;
pub const _CHICKEN_TRANS_EDP: c_uint = 0x420cc;
pub const _CHICKEN_TRANS_D: c_uint = 0x420d8;

pub const _MTL_CHICKEN_TRANS_A: c_uint = 0x604e0;
pub const _MTL_CHICKEN_TRANS_B: c_uint = 0x614e0;

pub const _BW_BUDDY0_CTL: c_uint = 0x45130;
pub const _BW_BUDDY1_CTL: c_uint = 0x45140;

pub const _BW_BUDDY0_PAGE_MASK: c_uint = 0x45134;
pub const _BW_BUDDY1_PAGE_MASK: c_uint = 0x45144;

pub const XE2LPD_DE_CAP_DSC_REMOVED: c_int = 1;

pub const XE2LPD_DE_CAP_SCALER_SINGLE: c_int = 1;

// GEN11 chicken
pub const _PIPEA_CHICKEN: c_uint = 0x70038;
pub const _PIPEB_CHICKEN: c_uint = 0x71038;
pub const _PIPEC_CHICKEN: c_uint = 0x72038;

pub const PCH_DISPLAY_BASE: c_uint = 0xc0000u;
// south display engine interrupt: IBX

// 18 reserved

// 12 reserved

// south display engine interrupt: CPT - CNP

pub const SDE_AUDIO_POWER_SHIFT_CPT: c_int = 29;

// south display engine interrupt: ICP/TGP/MTP

// PCH

// digital port hotplug

// This register is a reuse of PCH_PORT_HOTPLUG register. The
// functionality covered in PCH_PORT_HOTPLUG is split into
// SHOTPLUG_CTL_DDI and SHOTPLUG_CTL_TC.
//

pub const SHPD_FILTER_CNT_500_ADJ: c_uint = 0x001D9;
pub const SHPD_FILTER_CNT_250: c_uint = 0x000F8;
pub const _PCH_DPLL_A: c_uint = 0xc6014;
pub const _PCH_DPLL_B: c_uint = 0xc6018;

pub const _PCH_FPA0: c_uint = 0xc6040;
pub const _PCH_FPB0: c_uint = 0xc6048;

pub const _PCH_FPA1: c_uint = 0xc6044;
pub const _PCH_FPB1: c_uint = 0xc604c;

pub const DREF_CONTROL_MASK: c_uint = 0x7fc3;

pub const FDL_TP1_TIMER_SHIFT: c_int = 12;

pub const FDL_TP2_TIMER_SHIFT: c_int = 10;

pub const RAWCLK_FREQ_MASK: c_uint = 0x3ff;

pub const TRANS_DPLLA_SEL(pipe): c_int = 0;

// transcoder
pub const _PCH_TRANS_HTOTAL_A: c_uint = 0xe0000;
pub const _PCH_TRANS_HTOTAL_B: c_uint = 0xe1000;

pub const TRANS_HTOTAL_SHIFT: c_int = 16;
pub const TRANS_HACTIVE_SHIFT: c_int = 0;
pub const _PCH_TRANS_HBLANK_A: c_uint = 0xe0004;
pub const _PCH_TRANS_HBLANK_B: c_uint = 0xe1004;

pub const TRANS_HBLANK_END_SHIFT: c_int = 16;
pub const TRANS_HBLANK_START_SHIFT: c_int = 0;
pub const _PCH_TRANS_HSYNC_A: c_uint = 0xe0008;
pub const _PCH_TRANS_HSYNC_B: c_uint = 0xe1008;

pub const TRANS_HSYNC_END_SHIFT: c_int = 16;
pub const TRANS_HSYNC_START_SHIFT: c_int = 0;
pub const _PCH_TRANS_VTOTAL_A: c_uint = 0xe000c;
pub const _PCH_TRANS_VTOTAL_B: c_uint = 0xe100c;

pub const TRANS_VTOTAL_SHIFT: c_int = 16;
pub const TRANS_VACTIVE_SHIFT: c_int = 0;
pub const _PCH_TRANS_VBLANK_A: c_uint = 0xe0010;
pub const _PCH_TRANS_VBLANK_B: c_uint = 0xe1010;

pub const TRANS_VBLANK_END_SHIFT: c_int = 16;
pub const TRANS_VBLANK_START_SHIFT: c_int = 0;
pub const _PCH_TRANS_VSYNC_A: c_uint = 0xe0014;
pub const _PCH_TRANS_VSYNC_B: c_uint = 0xe1014;

pub const TRANS_VSYNC_END_SHIFT: c_int = 16;
pub const TRANS_VSYNC_START_SHIFT: c_int = 0;
pub const _PCH_TRANS_VSYNCSHIFT_A: c_uint = 0xe0028;
pub const _PCH_TRANS_VSYNCSHIFT_B: c_uint = 0xe1028;

pub const _PCH_TRANSA_DATA_M1: c_uint = 0xe0030;
pub const _PCH_TRANSB_DATA_M1: c_uint = 0xe1030;

pub const _PCH_TRANSA_DATA_N1: c_uint = 0xe0034;
pub const _PCH_TRANSB_DATA_N1: c_uint = 0xe1034;

pub const _PCH_TRANSA_DATA_M2: c_uint = 0xe0038;
pub const _PCH_TRANSB_DATA_M2: c_uint = 0xe1038;

pub const _PCH_TRANSA_DATA_N2: c_uint = 0xe003c;
pub const _PCH_TRANSB_DATA_N2: c_uint = 0xe103c;

pub const _PCH_TRANSA_LINK_M1: c_uint = 0xe0040;
pub const _PCH_TRANSB_LINK_M1: c_uint = 0xe1040;

pub const _PCH_TRANSA_LINK_N1: c_uint = 0xe0044;
pub const _PCH_TRANSB_LINK_N1: c_uint = 0xe1044;

pub const _PCH_TRANSA_LINK_M2: c_uint = 0xe0048;
pub const _PCH_TRANSB_LINK_M2: c_uint = 0xe1048;

pub const _PCH_TRANSA_LINK_N2: c_uint = 0xe004c;
pub const _PCH_TRANSB_LINK_N2: c_uint = 0xe104c;

// Per-transcoder DIP controls (PCH)
pub const _VIDEO_DIP_CTL_A: c_uint = 0xe0200;
pub const _VIDEO_DIP_CTL_B: c_uint = 0xe1200;

pub const _VIDEO_DIP_DATA_A: c_uint = 0xe0208;
pub const _VIDEO_DIP_DATA_B: c_uint = 0xe1208;

pub const _VIDEO_DIP_GCP_A: c_uint = 0xe0210;
pub const _VIDEO_DIP_GCP_B: c_uint = 0xe1210;

// Per-transcoder DIP controls (VLV)
pub const _VLV_VIDEO_DIP_CTL_A: c_uint = 0x60200;
pub const _VLV_VIDEO_DIP_CTL_B: c_uint = 0x61170;
pub const _CHV_VIDEO_DIP_CTL_C: c_uint = 0x611f0;

pub const _VLV_VIDEO_DIP_DATA_A: c_uint = 0x60208;
pub const _VLV_VIDEO_DIP_DATA_B: c_uint = 0x61174;
pub const _CHV_VIDEO_DIP_DATA_C: c_uint = 0x611f4;

pub const _VLV_VIDEO_DIP_GDCP_PAYLOAD_A: c_uint = 0x60210;
pub const _VLV_VIDEO_DIP_GDCP_PAYLOAD_B: c_uint = 0x61178;
pub const _CHV_VIDEO_DIP_GDCP_PAYLOAD_C: c_uint = 0x611f8;

// Haswell DIP controls
pub const _HSW_VIDEO_DIP_CTL_A: c_uint = 0x60200;
pub const _HSW_VIDEO_DIP_CTL_B: c_uint = 0x61200;

pub const _HSW_VIDEO_DIP_AVI_DATA_A: c_uint = 0x60220;
pub const _HSW_VIDEO_DIP_AVI_DATA_B: c_uint = 0x61220;

pub const _HSW_VIDEO_DIP_VS_DATA_A: c_uint = 0x60260;
pub const _HSW_VIDEO_DIP_VS_DATA_B: c_uint = 0x61260;

pub const _HSW_VIDEO_DIP_SPD_DATA_A: c_uint = 0x602A0;
pub const _HSW_VIDEO_DIP_SPD_DATA_B: c_uint = 0x612A0;

pub const _HSW_VIDEO_DIP_GMP_DATA_A: c_uint = 0x602E0;
pub const _HSW_VIDEO_DIP_GMP_DATA_B: c_uint = 0x612E0;

pub const _HSW_VIDEO_DIP_VSC_DATA_A: c_uint = 0x60320;
pub const _HSW_VIDEO_DIP_VSC_DATA_B: c_uint = 0x61320;

// ADLP and later:
pub const _ADL_VIDEO_DIP_AS_DATA_A: c_uint = 0x60484;
pub const _ADL_VIDEO_DIP_AS_DATA_B: c_uint = 0x61484;

pub const _GLK_VIDEO_DIP_DRM_DATA_A: c_uint = 0x60440;
pub const _GLK_VIDEO_DIP_DRM_DATA_B: c_uint = 0x61440;

pub const _HSW_VIDEO_DIP_AVI_ECC_A: c_uint = 0x60240;
pub const _HSW_VIDEO_DIP_BVI_ECC_B: c_uint = 0x61240;
pub const _HSW_VIDEO_DIP_VS_ECC_A: c_uint = 0x60280;
pub const _HSW_VIDEO_DIP_VS_ECC_B: c_uint = 0x61280;
pub const _HSW_VIDEO_DIP_SPD_ECC_A: c_uint = 0x602C0;
pub const _HSW_VIDEO_DIP_SPD_ECC_B: c_uint = 0x612C0;
pub const _HSW_VIDEO_DIP_GMP_ECC_A: c_uint = 0x60300;
pub const _HSW_VIDEO_DIP_GMP_ECC_B: c_uint = 0x61300;
pub const _HSW_VIDEO_DIP_VSC_ECC_A: c_uint = 0x60344;
pub const _HSW_VIDEO_DIP_VSC_ECC_B: c_uint = 0x61344;
pub const _HSW_VIDEO_DIP_GCP_A: c_uint = 0x60210;
pub const _HSW_VIDEO_DIP_GCP_B: c_uint = 0x61210;

pub const _ICL_VIDEO_DIP_PPS_DATA_A: c_uint = 0x60350;
pub const _ICL_VIDEO_DIP_PPS_DATA_B: c_uint = 0x61350;

pub const _ICL_VIDEO_DIP_PPS_ECC_A: c_uint = 0x603D4;
pub const _ICL_VIDEO_DIP_PPS_ECC_B: c_uint = 0x613D4;

pub const _HSW_STEREO_3D_CTL_A: c_uint = 0x70020;
pub const _HSW_STEREO_3D_CTL_B: c_uint = 0x71020;

pub const _PCH_TRANSACONF: c_uint = 0xf0008;
pub const _PCH_TRANSBCONF: c_uint = 0xf1008;

// Icelake PPS_DATA and _ECC DIP Registers.
// These are available for transcoders B,C and eDP.
// Adding the _A so as to reuse the _MMIO_TRANS2
// definition, with which it offsets to the right location.
//
pub const _TRANSA_CHICKEN1: c_uint = 0xf0060;
pub const _TRANSB_CHICKEN1: c_uint = 0xf1060;

pub const _TRANSA_CHICKEN2: c_uint = 0xf0064;
pub const _TRANSB_CHICKEN2: c_uint = 0xf1064;

// CPT
pub const _TRANS_DP_CTL_A: c_uint = 0xe0300;
pub const _TRANS_DP_CTL_B: c_uint = 0xe1300;
pub const _TRANS_DP_CTL_C: c_uint = 0xe2300;

pub const _TRANS_DP2_CTL_A: c_uint = 0x600a0;
pub const _TRANS_DP2_CTL_B: c_uint = 0x610a0;
pub const _TRANS_DP2_CTL_C: c_uint = 0x620a0;
pub const _TRANS_DP2_CTL_D: c_uint = 0x630a0;

pub const _TRANS_DP2_VFREQHIGH_A: c_uint = 0x600a4;
pub const _TRANS_DP2_VFREQHIGH_B: c_uint = 0x610a4;
pub const _TRANS_DP2_VFREQHIGH_C: c_uint = 0x620a4;
pub const _TRANS_DP2_VFREQHIGH_D: c_uint = 0x630a4;

pub const _TRANS_DP2_VFREQLOW_A: c_uint = 0x600a8;
pub const _TRANS_DP2_VFREQLOW_B: c_uint = 0x610a8;
pub const _TRANS_DP2_VFREQLOW_C: c_uint = 0x620a8;
pub const _TRANS_DP2_VFREQLOW_D: c_uint = 0x630a8;

pub const _DP_MIN_HBLANK_CTL_A: c_uint = 0x600ac;
pub const _DP_MIN_HBLANK_CTL_B: c_uint = 0x610ac;

// SNB eDP training params
// SNB A-stepping

// SNB B-stepping

// IVB

// legacy values

pub const PIXEL_OVERLAP_CNT_SHIFT: c_int = 30;
//
// HSW - ICL power wells
//
// Platforms have up to 3 power well control register sets, each set
// controlling up to 16 power wells via a request/status HW flag tuple:
// - main (HSW_PWR_WELL_CTL[1-4])
// - AUX  (ICL_PWR_WELL_CTL_AUX[1-4])
// - DDI  (ICL_PWR_WELL_CTL_DDI[1-4])
// Each control register set consists of up to 4 registers used by different
// sources that can request a power well to be enabled:
// - BIOS   (HSW_PWR_WELL_CTL1/ICL_PWR_WELL_CTL_AUX1/ICL_PWR_WELL_CTL_DDI1)
// - DRIVER (HSW_PWR_WELL_CTL2/ICL_PWR_WELL_CTL_AUX2/ICL_PWR_WELL_CTL_DDI2)
// - KVMR   (HSW_PWR_WELL_CTL3)   (only in the main register set)
// - DEBUG  (HSW_PWR_WELL_CTL4/ICL_PWR_WELL_CTL_AUX4/ICL_PWR_WELL_CTL_DDI4)
//

// HSW/BDW power well
pub const HSW_PW_CTL_IDX_GLOBAL: c_int = 15;
// SKL/BXT/GLK power wells
pub const SKL_PW_CTL_IDX_PW_2: c_int = 15;
pub const SKL_PW_CTL_IDX_PW_1: c_int = 14;
pub const GLK_PW_CTL_IDX_AUX_C: c_int = 10;
pub const GLK_PW_CTL_IDX_AUX_B: c_int = 9;
pub const GLK_PW_CTL_IDX_AUX_A: c_int = 8;
pub const SKL_PW_CTL_IDX_DDI_D: c_int = 4;
pub const SKL_PW_CTL_IDX_DDI_C: c_int = 3;
pub const SKL_PW_CTL_IDX_DDI_B: c_int = 2;
pub const SKL_PW_CTL_IDX_DDI_A_E: c_int = 1;
pub const GLK_PW_CTL_IDX_DDI_A: c_int = 1;
pub const SKL_PW_CTL_IDX_MISC_IO: c_int = 0;
// ICL/TGL - power wells
pub const TGL_PW_CTL_IDX_PW_5: c_int = 4;
pub const ICL_PW_CTL_IDX_PW_4: c_int = 3;
pub const ICL_PW_CTL_IDX_PW_3: c_int = 2;
pub const ICL_PW_CTL_IDX_PW_2: c_int = 1;
pub const ICL_PW_CTL_IDX_PW_1: c_int = 0;
// XE_LPD - power wells
pub const XELPD_PW_CTL_IDX_PW_D: c_int = 8;
pub const XELPD_PW_CTL_IDX_PW_C: c_int = 7;
pub const XELPD_PW_CTL_IDX_PW_B: c_int = 6;
pub const XELPD_PW_CTL_IDX_PW_A: c_int = 5;

pub const TGL_PW_CTL_IDX_AUX_TBT6: c_int = 14;
pub const TGL_PW_CTL_IDX_AUX_TBT5: c_int = 13;
pub const TGL_PW_CTL_IDX_AUX_TBT4: c_int = 12;
pub const ICL_PW_CTL_IDX_AUX_TBT4: c_int = 11;
pub const TGL_PW_CTL_IDX_AUX_TBT3: c_int = 11;
pub const ICL_PW_CTL_IDX_AUX_TBT3: c_int = 10;
pub const TGL_PW_CTL_IDX_AUX_TBT2: c_int = 10;
pub const ICL_PW_CTL_IDX_AUX_TBT2: c_int = 9;
pub const TGL_PW_CTL_IDX_AUX_TBT1: c_int = 9;
pub const ICL_PW_CTL_IDX_AUX_TBT1: c_int = 8;
pub const TGL_PW_CTL_IDX_AUX_TC6: c_int = 8;
pub const XELPD_PW_CTL_IDX_AUX_E: c_int = 8;
pub const TGL_PW_CTL_IDX_AUX_TC5: c_int = 7;
pub const XELPD_PW_CTL_IDX_AUX_D: c_int = 7;
pub const TGL_PW_CTL_IDX_AUX_TC4: c_int = 6;
pub const ICL_PW_CTL_IDX_AUX_F: c_int = 5;
pub const TGL_PW_CTL_IDX_AUX_TC3: c_int = 5;
pub const ICL_PW_CTL_IDX_AUX_E: c_int = 4;
pub const TGL_PW_CTL_IDX_AUX_TC2: c_int = 4;
pub const ICL_PW_CTL_IDX_AUX_D: c_int = 3;
pub const TGL_PW_CTL_IDX_AUX_TC1: c_int = 3;
pub const ICL_PW_CTL_IDX_AUX_C: c_int = 2;
pub const ICL_PW_CTL_IDX_AUX_B: c_int = 1;
pub const ICL_PW_CTL_IDX_AUX_A: c_int = 0;

pub const XELPD_PW_CTL_IDX_DDI_E: c_int = 8;
pub const TGL_PW_CTL_IDX_DDI_TC6: c_int = 8;
pub const XELPD_PW_CTL_IDX_DDI_D: c_int = 7;
pub const TGL_PW_CTL_IDX_DDI_TC5: c_int = 7;
pub const TGL_PW_CTL_IDX_DDI_TC4: c_int = 6;
pub const ICL_PW_CTL_IDX_DDI_F: c_int = 5;
pub const TGL_PW_CTL_IDX_DDI_TC3: c_int = 5;
pub const ICL_PW_CTL_IDX_DDI_E: c_int = 4;
pub const TGL_PW_CTL_IDX_DDI_TC2: c_int = 4;
pub const ICL_PW_CTL_IDX_DDI_D: c_int = 3;
pub const TGL_PW_CTL_IDX_DDI_TC1: c_int = 3;
pub const ICL_PW_CTL_IDX_DDI_C: c_int = 2;
pub const ICL_PW_CTL_IDX_DDI_B: c_int = 1;
pub const ICL_PW_CTL_IDX_DDI_A: c_int = 0;
// HSW - power well misc debug registers

// clock gating DSS DSC disable register

// SKL Fuse Status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skl_power_gate {
    SKL_PG0,
    SKL_PG1,
    SKL_PG2,
    ICL_PG3,
    ICL_PG4,
}

// Per-pipe DDI Function Control
pub const _TRANS_DDI_FUNC_CTL_A: c_uint = 0x60400;
pub const _TRANS_DDI_FUNC_CTL_B: c_uint = 0x61400;
pub const _TRANS_DDI_FUNC_CTL_C: c_uint = 0x62400;
pub const _TRANS_DDI_FUNC_CTL_D: c_uint = 0x63400;
pub const _TRANS_DDI_FUNC_CTL_EDP: c_uint = 0x6F400;
pub const _TRANS_DDI_FUNC_CTL_DSI0: c_uint = 0x6b400;
pub const _TRANS_DDI_FUNC_CTL_DSI1: c_uint = 0x6bc00;

// Those bits are ignored by pipe EDP since it can only connect to DDI A
pub const TRANS_DDI_PORT_SHIFT: c_int = 28;
pub const TGL_TRANS_DDI_PORT_SHIFT: c_int = 27;

pub const _TRANS_DDI_FUNC_CTL2_A: c_uint = 0x60404;
pub const _TRANS_DDI_FUNC_CTL2_B: c_uint = 0x61404;
pub const _TRANS_DDI_FUNC_CTL2_C: c_uint = 0x62404;
pub const _TRANS_DDI_FUNC_CTL2_EDP: c_uint = 0x6f404;
pub const _TRANS_DDI_FUNC_CTL2_DSI0: c_uint = 0x6b404;
pub const _TRANS_DDI_FUNC_CTL2_DSI1: c_uint = 0x6bc04;

// DisplayPort Transport Control
pub const _DP_TP_CTL_A: c_uint = 0x64040;
pub const _DP_TP_CTL_B: c_uint = 0x64140;
pub const _TGL_DP_TP_CTL_A: c_uint = 0x60540;

// DisplayPort Transport Status
pub const _DP_TP_STATUS_A: c_uint = 0x64044;
pub const _DP_TP_STATUS_B: c_uint = 0x64144;
pub const _TGL_DP_TP_STATUS_A: c_uint = 0x60544;

// DDI Buffer Control
pub const _DDI_BUF_CTL_A: c_uint = 0x64000;
pub const _DDI_BUF_CTL_B: c_uint = 0x64100;
// Known as DDI_CTL_DE in MTL+

pub const DDI_PORT_WIDTH_SHIFT: c_int = 1;

// DDI Buffer Translations
pub const _DDI_BUF_TRANS_A: c_uint = 0x64E00;
pub const _DDI_BUF_TRANS_B: c_uint = 0x64E60;

// DDI DP Compliance Control
pub const _DDI_DP_COMP_CTL_A: c_uint = 0x605F0;
pub const _DDI_DP_COMP_CTL_B: c_uint = 0x615F0;

// DDI DP Compliance Pattern
pub const _DDI_DP_COMP_PAT_A: c_uint = 0x605F4;
pub const _DDI_DP_COMP_PAT_B: c_uint = 0x615F4;

// LPT PIXCLK_GATE

// SPLL

// WRPLL
pub const _WRPLL_CTL1: c_uint = 0x46040;
pub const _WRPLL_CTL2: c_uint = 0x46060;

// WRPLL divider programming

pub const WRPLL_DIVIDER_POST_SHIFT: c_int = 8;

pub const WRPLL_DIVIDER_FB_SHIFT: c_int = 16;

// Port clock selection
pub const _PORT_CLK_SEL_A: c_uint = 0x46100;
pub const _PORT_CLK_SEL_B: c_uint = 0x46104;

// On ICL+ this is the same as PORT_CLK_SEL, but all bits change.

// Transcoder clock selection
pub const _TRANS_CLK_SEL_A: c_uint = 0x46140;
pub const _TRANS_CLK_SEL_B: c_uint = 0x46144;

// For each transcoder, we need to select the corresponding port clock

pub const _TRANSA_MSA_MISC: c_uint = 0x60410;
pub const _TRANSB_MSA_MISC: c_uint = 0x61410;
pub const _TRANSC_MSA_MISC: c_uint = 0x62410;
pub const _TRANS_EDP_MSA_MISC: c_uint = 0x6f410;

// See DP_MSA_MISC_* for the bit definitions
pub const _TRANS_A_SET_CONTEXT_LATENCY: c_uint = 0x6007C;
pub const _TRANS_B_SET_CONTEXT_LATENCY: c_uint = 0x6107C;
pub const _TRANS_C_SET_CONTEXT_LATENCY: c_uint = 0x6207C;
pub const _TRANS_D_SET_CONTEXT_LATENCY: c_uint = 0x6307C;

// LCPLL Control

//
// SKL Clocks
//
// CDCLK_CTL

// CDCLK_SQUASH_CTL

// LCPLL_CTL

// DPLL control1

pub const DPLL_CTRL1_LINK_RATE_2700: c_int = 0;
pub const DPLL_CTRL1_LINK_RATE_1350: c_int = 1;
pub const DPLL_CTRL1_LINK_RATE_810: c_int = 2;
pub const DPLL_CTRL1_LINK_RATE_1620: c_int = 3;
pub const DPLL_CTRL1_LINK_RATE_1080: c_int = 4;
pub const DPLL_CTRL1_LINK_RATE_2160: c_int = 5;
// DPLL control2

// DPLL Status

// DPLL cfg
pub const _DPLL1_CFGCR1: c_uint = 0x6C040;
pub const _DPLL2_CFGCR1: c_uint = 0x6C048;
pub const _DPLL3_CFGCR1: c_uint = 0x6C050;

pub const _DPLL1_CFGCR2: c_uint = 0x6C044;
pub const _DPLL2_CFGCR2: c_uint = 0x6C04C;
pub const _DPLL3_CFGCR2: c_uint = 0x6C054;

// ICL Clocks

//
// DG1 Clocks
// First registers controls the first A and B, while the second register
// controls the phy C and D. The bits on these registers are the
// same, but refer to different phys
//
pub const _DG1_DPCLKA_CFGCR0: c_uint = 0x164280;
pub const _DG1_DPCLKA1_CFGCR0: c_uint = 0x16C280;

// ADLS Clocks
pub const _ADLS_DPCLKA_CFGCR0: c_uint = 0x164280;
pub const _ADLS_DPCLKA_CFGCR1: c_uint = 0x1642BC;

// ADLS DPCLKA_CFGCR0 DDI mask

// ADLS DPCLKA_CFGCR1 DDI mask

// ICL PLL
pub const _DPLL0_ENABLE: c_uint = 0x46010;
pub const _DPLL1_ENABLE: c_uint = 0x46014;
pub const _ADLS_DPLL2_ENABLE: c_uint = 0x46018;
pub const _ADLS_DPLL3_ENABLE: c_uint = 0x46030;

pub const _DG2_PLL3_ENABLE: c_uint = 0x4601C;

pub const _MG_PLL1_ENABLE: c_uint = 0x46030;
pub const _MG_PLL2_ENABLE: c_uint = 0x46034;
pub const _MG_PLL3_ENABLE: c_uint = 0x46038;
pub const _MG_PLL4_ENABLE: c_uint = 0x4603C;
// Bits are the same as _DPLL0_ENABLE

// DG1 PLL

// ADL-P Type C PLL
pub const PORTTC1_PLL_ENABLE: c_uint = 0x46038;
pub const PORTTC2_PLL_ENABLE: c_uint = 0x46040;

pub const _ICL_DPLL0_CFGCR0: c_uint = 0x164000;
pub const _ICL_DPLL1_CFGCR0: c_uint = 0x164080;

pub const _ICL_DPLL0_CFGCR1: c_uint = 0x164004;
pub const _ICL_DPLL1_CFGCR1: c_uint = 0x164084;

pub const _TGL_DPLL0_CFGCR0: c_uint = 0x164284;
pub const _TGL_DPLL1_CFGCR0: c_uint = 0x16428C;
pub const _TGL_TBTPLL_CFGCR0: c_uint = 0x16429C;

pub const _TGL_DPLL0_DIV0: c_uint = 0x164B00;
pub const _TGL_DPLL1_DIV0: c_uint = 0x164C00;

pub const _TGL_DPLL0_CFGCR1: c_uint = 0x164288;
pub const _TGL_DPLL1_CFGCR1: c_uint = 0x164290;
pub const _TGL_TBTPLL_CFGCR1: c_uint = 0x1642A0;

pub const _DG1_DPLL2_CFGCR0: c_uint = 0x16C284;
pub const _DG1_DPLL3_CFGCR0: c_uint = 0x16C28C;

pub const _DG1_DPLL2_CFGCR1: c_uint = 0x16C288;
pub const _DG1_DPLL3_CFGCR1: c_uint = 0x16C290;

// For ADL-S DPLL4_CFGCR0/1 are used to control DPLL2
pub const _ADLS_DPLL4_CFGCR0: c_uint = 0x164294;
pub const _ADLS_DPLL3_CFGCR0: c_uint = 0x1642C0;

pub const _ADLS_DPLL4_CFGCR1: c_uint = 0x164298;
pub const _ADLS_DPLL3_CFGCR1: c_uint = 0x1642C4;

// BXT display engine PLL

pub const BXT_DE_PLL_RATIO_MASK: c_uint = 0xff;

pub const ICL_CDCLK_PLL_RATIO_MASK: c_uint = 0xff;
// GEN9 DC

pub const DC_STATE_DISABLE: c_int = 0;

// display version 20+

// display version 13+, except dg2

// Pipe WM_LINETIME - watermark line time
pub const _WM_LINETIME_A: c_uint = 0x45270;
pub const _WM_LINETIME_B: c_uint = 0x45274;

// SFUSE_STRAP

pub const FDIA_PHASE_SYNC_SHIFT_OVR: c_int = 19;
pub const FDIA_PHASE_SYNC_SHIFT_EN: c_int = 18;

// Gen4+ Timestamp and Pipe Frame time stamp registers

// g4x+, except vlv/chv!
pub const _PIPE_FRMTMSTMP_A: c_uint = 0x70048;
pub const _PIPE_FRMTMSTMP_B: c_uint = 0x71048;

// g4x+, except vlv/chv!
pub const _PIPE_FLIPTMSTMP_A: c_uint = 0x7004C;
pub const _PIPE_FLIPTMSTMP_B: c_uint = 0x7104C;

// tgl+
pub const _PIPE_FLIPDONETMSTMP_A: c_uint = 0x70054;
pub const _PIPE_FLIPDONETMSTMP_B: c_uint = 0x71054;

pub const _VLV_PIPE_MSA_MISC_A: c_uint = 0x70048;

pub const _ICL_PHY_MISC_A: c_uint = 0x64C00;
pub const _ICL_PHY_MISC_B: c_uint = 0x64C04;
pub const _DG2_PHY_MISC_TC1: c_uint = 0x64C14 /* TC1="PHY E" but offset as if "PHY F" */;

// See enum intel_tc_pin_assignment for the pin assignment field values.
pub const _TCSS_DDI_STATUS_1: c_uint = 0x161500;
pub const _TCSS_DDI_STATUS_2: c_uint = 0x161504;

// See enum intel_tc_pin_assignment for the pin assignment field values.

pub const _MTL_CLKGATE_DIS_TRANS_A: c_uint = 0x604E8;
pub const _MTL_CLKGATE_DIS_TRANS_B: c_uint = 0x614E8;

pub const _MTL_PIPE_CLKGATE_DIS2_A: c_uint = 0x60114;
pub const _MTL_PIPE_CLKGATE_DIS2_B: c_uint = 0x61114;

pub const MTL_MEM_SS_INFO_QGV_POINT_OFFSET: c_uint = 0x45710;

