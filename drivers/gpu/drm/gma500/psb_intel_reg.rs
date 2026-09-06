//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/psb_intel_reg.h
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
// Copyright (c) 2009, Intel Corporation.
//
// GPIO regs
//
pub const GPIOA: c_uint = 0x5010;
pub const GPIOB: c_uint = 0x5014;
pub const GPIOC: c_uint = 0x5018;
pub const GPIOD: c_uint = 0x501c;
pub const GPIOE: c_uint = 0x5020;
pub const GPIOF: c_uint = 0x5024;
pub const GPIOG: c_uint = 0x5028;
pub const GPIOH: c_uint = 0x502c;

pub const GMBUS0: c_uint = 0x5100 /* clock/port select */;

pub const GMBUS_PORT_DISABLED: c_int = 0;
pub const GMBUS_PORT_SSC: c_int = 1;
pub const GMBUS_PORT_VGADDC: c_int = 2;
pub const GMBUS_PORT_PANEL: c_int = 3;

// 6 reserved

pub const GMBUS_NUM_PORTS: c_int = 8;
pub const GMBUS1: c_uint = 0x5104 /* command/status */;

pub const GMBUS_BYTE_COUNT_SHIFT: c_int = 16;
pub const GMBUS_SLAVE_INDEX_SHIFT: c_int = 8;
pub const GMBUS_SLAVE_ADDR_SHIFT: c_int = 1;

pub const GMBUS2: c_uint = 0x5108 /* status */;

pub const GMBUS3: c_uint = 0x510c /* data buffer bytes 3-0 */;
pub const GMBUS4: c_uint = 0x5110 /* interrupt mask (Pineview+) */;

pub const GMBUS5: c_uint = 0x5120 /* byte index */;

pub const BLC_PWM_CTL: c_uint = 0x61254;
pub const BLC_PWM_CTL2: c_uint = 0x61250;

pub const BLC_PWM_CTL_C: c_uint = 0x62254;
pub const BLC_PWM_CTL2_C: c_uint = 0x62250;

//
// This is the most significant 15 bits of the number of backlight cycles in a
// complete cycle of the modulated backlight control.
//
// The actual value is this field multiplied by two.
//

//
// This is the number of cycles out of the backlight modulation cycle for which
// the backlight is on.
//
// This field must be no greater than the number of cycles in the complete
// backlight modulation cycle.
//

pub const I915_GCFGC: c_uint = 0xf0;

pub const I855_HPLLCC: c_uint = 0xc0;

// I830 CRTC registers
pub const HTOTAL_A: c_uint = 0x60000;
pub const HBLANK_A: c_uint = 0x60004;
pub const HSYNC_A: c_uint = 0x60008;
pub const VTOTAL_A: c_uint = 0x6000c;
pub const VBLANK_A: c_uint = 0x60010;
pub const VSYNC_A: c_uint = 0x60014;
pub const PIPEASRC: c_uint = 0x6001c;
pub const BCLRPAT_A: c_uint = 0x60020;
pub const VSYNCSHIFT_A: c_uint = 0x60028;
pub const HTOTAL_B: c_uint = 0x61000;
pub const HBLANK_B: c_uint = 0x61004;
pub const HSYNC_B: c_uint = 0x61008;
pub const VTOTAL_B: c_uint = 0x6100c;
pub const VBLANK_B: c_uint = 0x61010;
pub const VSYNC_B: c_uint = 0x61014;
pub const PIPEBSRC: c_uint = 0x6101c;
pub const BCLRPAT_B: c_uint = 0x61020;
pub const VSYNCSHIFT_B: c_uint = 0x61028;
pub const HTOTAL_C: c_uint = 0x62000;
pub const HBLANK_C: c_uint = 0x62004;
pub const HSYNC_C: c_uint = 0x62008;
pub const VTOTAL_C: c_uint = 0x6200c;
pub const VBLANK_C: c_uint = 0x62010;
pub const VSYNC_C: c_uint = 0x62014;
pub const PIPECSRC: c_uint = 0x6201c;
pub const BCLRPAT_C: c_uint = 0x62020;
pub const VSYNCSHIFT_C: c_uint = 0x62028;
pub const PP_STATUS: c_uint = 0x61200;

//
// Indicates that all dependencies of the panel are on:
//
// - PLL enabled
// - pipe enabled
// - LVDS/DVOB/DVOC on
//

pub const PP_SEQUENCE_MASK: c_uint = 0x30000000;

pub const PP_SEQUENCE_STATE_MASK: c_uint = 0x0000000f;
pub const PP_CONTROL: c_uint = 0x61204;

// Poulsbo/Oaktrail
pub const LVDSPP_ON: c_uint = 0x61208;
pub const LVDSPP_OFF: c_uint = 0x6120c;
pub const PP_CYCLE: c_uint = 0x61210;
// Cedartrail
pub const PP_ON_DELAYS: c_uint = 0x61208		/* Cedartrail */;

pub const PANEL_POWER_UP_DELAY_SHIFT: c_int = 16;

pub const PANEL_LIGHT_ON_DELAY_SHIFT: c_int = 0;
pub const PP_OFF_DELAYS: c_uint = 0x6120c		/* Cedartrail */;

pub const PANEL_POWER_DOWN_DELAY_SHIFT: c_int = 16;

pub const PANEL_LIGHT_OFF_DELAY_SHIFT: c_int = 0;
pub const PP_DIVISOR: c_uint = 0x61210		/* Cedartrail */;

pub const PP_REFERENCE_DIVIDER_SHIFT: c_int = 8;

pub const PANEL_POWER_CYCLE_DELAY_SHIFT: c_int = 0;
pub const PFIT_CONTROL: c_uint = 0x61230;

pub const PFIT_PIPE_SHIFT: c_int = 29;

pub const PFIT_PGM_RATIOS: c_uint = 0x61234;
pub const PFIT_VERT_SCALE_MASK: c_uint = 0xfff00000;
pub const PFIT_HORIZ_SCALE_MASK: c_uint = 0x0000fff0;
pub const PFIT_AUTO_RATIOS: c_uint = 0x61238;
pub const DPLL_A: c_uint = 0x06014;
pub const DPLL_B: c_uint = 0x06018;

pub const DPLL_P2_CLOCK_DIV_MASK: c_uint = 0x03000000	/* i915 */;
pub const DPLL_FPA0h1_P1_POST_DIV_MASK: c_uint = 0x00ff0000	/* i915 */;

//
// The i830 generation, in DAC/serial mode, defines p1 as two plus this
// bitfield, or just 2 if PLL_P1_DIVIDE_BY_TWO is set.
//

//
// The i830 generation, in LVDS mode, defines P1 as the bit number set within
// this field (only one bit may be set).
//
pub const DPLL_FPA01_P1_POST_DIV_MASK_I830_LVDS: c_uint = 0x003f0000;
pub const DPLL_FPA01_P1_POST_DIV_SHIFT: c_int = 16;

// in DVO non-gang

// TVCLKIN

pub const PLL_LOAD_PULSE_PHASE_SHIFT: c_int = 9;
//
// Parallel to Serial Load Pulse phase selection.
// Selects the phase for the 10X DPLL clock for the PCIe
// digital display port. The range is 4 to 13; 10 or more
// is just a flip delay. The default is 6
//

//
// SDVO multiplier for 945G/GM. Not used on 965.
//
// DPLL_MD_UDI_MULTIPLIER_MASK
//
pub const SDVO_MULTIPLIER_MASK: c_uint = 0x000000ff;
pub const SDVO_MULTIPLIER_SHIFT_HIRES: c_int = 4;
pub const SDVO_MULTIPLIER_SHIFT_VGA: c_int = 0;
//
// PLL_MD
//
// Pipe A SDVO/UDI clock multiplier/divider register for G965.
pub const DPLL_A_MD: c_uint = 0x0601c;
// Pipe B SDVO/UDI clock multiplier/divider register for G965.
pub const DPLL_B_MD: c_uint = 0x06020;
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
pub const DPLL_TEST: c_uint = 0x606c;

pub const ADPA: c_uint = 0x61100;

pub const ADPA_DAC_DISABLE: c_int = 0;

pub const ADPA_PIPE_A_SELECT: c_int = 0;

pub const ADPA_SETS_HVPOLARITY: c_int = 0;

pub const ADPA_VSYNC_CNTL_ENABLE: c_int = 0;

pub const ADPA_HSYNC_CNTL_ENABLE: c_int = 0;

pub const ADPA_VSYNC_ACTIVE_LOW: c_int = 0;

pub const ADPA_HSYNC_ACTIVE_LOW: c_int = 0;
pub const FPA0: c_uint = 0x06040;
pub const FPA1: c_uint = 0x06044;
pub const FPB0: c_uint = 0x06048;
pub const FPB1: c_uint = 0x0604c;
pub const FP_N_DIV_MASK: c_uint = 0x003f0000;
pub const FP_N_DIV_SHIFT: c_int = 16;
pub const FP_M1_DIV_MASK: c_uint = 0x00003f00;
pub const FP_M1_DIV_SHIFT: c_int = 8;
pub const FP_M2_DIV_MASK: c_uint = 0x0000003f;
pub const FP_M2_DIV_SHIFT: c_int = 0;
pub const PORT_HOTPLUG_EN: c_uint = 0x61110;

// CDV..

pub const CRT_HOTPLUG_DETECT_MASK: c_uint = 0x000000F8;
pub const PORT_HOTPLUG_STAT: c_uint = 0x61114;

pub const SDVOB: c_uint = 0x61140;
pub const SDVOC: c_uint = 0x61160;

//
// 915G/GM SDVO pixel multiplier.
//
// Programmed value is multiplier - 1, up to 5x.
//
// DPLL_MD_UDI_MULTIPLIER_MASK
//

pub const SDVO_PORT_MULTIPLY_SHIFT: c_int = 23;

// Bits to be preserved when writing

//
// This register controls the LVDS output enable, pipe selection, and data
// format selection.
//
// All of the clock/data pairs are force powered down by power sequencing.
//
pub const LVDS: c_uint = 0x61180;
//
// Enables the LVDS port.  This bit must be set before DPLLs are enabled, as
// the DPLL semantics change when the LVDS is assigned to that pipe.
//

// Selects pipe B for LVDS data.  Must be set on pre-965.

// Turns on border drawing to allow centered display.

//
// Enables the A0-A2 data pairs and CLKA, containing 18 bits of color data per
// pixel.
//

//
// Controls the A3 data pair, which contains the additional LSBs for 24 bit
// mode.  Only enabled if LVDS_A0A2_CLKA_POWER_UP also indicates it should be
// on.
//

//
// Controls the CLKB pair.  This should only be set when LVDS_B0B3_POWER_UP
// is set.
//

//
// Controls the B0-B3 data pairs.  This must be set to match the DPLL p2
// setting for whether we are in dual-channel mode.  The B3 pair will
// additionally only be powered up when LVDS_A3_POWER_UP is set.
//

pub const PIPEACONF: c_uint = 0x70008;

pub const PIPEACONF_DISABLE: c_int = 0;

pub const PIPEACONF_SINGLE_WIDE: c_int = 0;
pub const PIPEACONF_PIPE_UNLOCKED: c_int = 0;

pub const PIPEACONF_PALETTE: c_int = 0;

pub const PIPEBCONF: c_uint = 0x71008;

pub const PIPEBCONF_DISABLE: c_int = 0;

pub const PIPEBCONF_DISABLE: c_int = 0;

pub const PIPEBCONF_PALETTE: c_int = 0;
pub const PIPECCONF: c_uint = 0x72008;
pub const PIPEBGCMAXRED: c_uint = 0x71010;
pub const PIPEBGCMAXGREEN: c_uint = 0x71014;
pub const PIPEBGCMAXBLUE: c_uint = 0x71018;
pub const PIPEASTAT: c_uint = 0x70024;
pub const PIPEBSTAT: c_uint = 0x71024;
pub const PIPECSTAT: c_uint = 0x72024;

pub const HISTOGRAM_INT_CONTROL: c_uint = 0x61268;

pub const HISTOGRAM_LOGIC_CONTROL: c_uint = 0x61260;
pub const PWM_CONTROL_LOGIC: c_uint = 0x61250;

pub const PWM_PHASEIN_VB_COUNT: c_uint = 0x00001f00;
pub const PWM_PHASEIN_INC: c_uint = 0x0000001f;

pub const DPST_YUV_LUMA_MODE: c_int = 0;
pub const PIPEAFRAMEHIGH: c_uint = 0x70040;
pub const PIPEAFRAMEPIXEL: c_uint = 0x70044;
pub const PIPEBFRAMEHIGH: c_uint = 0x71040;
pub const PIPEBFRAMEPIXEL: c_uint = 0x71044;
pub const PIPECFRAMEHIGH: c_uint = 0x72040;
pub const PIPECFRAMEPIXEL: c_uint = 0x72044;
pub const PIPE_FRAME_HIGH_MASK: c_uint = 0x0000ffff;
pub const PIPE_FRAME_HIGH_SHIFT: c_int = 0;
pub const PIPE_FRAME_LOW_MASK: c_uint = 0xff000000;
pub const PIPE_FRAME_LOW_SHIFT: c_int = 24;
pub const PIPE_PIXEL_MASK: c_uint = 0x00ffffff;
pub const PIPE_PIXEL_SHIFT: c_int = 0;
pub const FW_BLC_SELF: c_uint = 0x20e0;

pub const DSPARB: c_uint = 0x70030;
pub const DSPFW1: c_uint = 0x70034;
pub const DSP_FIFO_SR_WM_MASK: c_uint = 0xFF800000;
pub const DSP_FIFO_SR_WM_SHIFT: c_int = 23;
pub const CURSOR_B_FIFO_WM_MASK: c_uint = 0x003F0000;
pub const CURSOR_B_FIFO_WM_SHIFT: c_int = 16;
pub const DSPFW2: c_uint = 0x70038;
pub const CURSOR_A_FIFO_WM_MASK: c_uint = 0x3F00;
pub const CURSOR_A_FIFO_WM_SHIFT: c_int = 8;
pub const DSP_PLANE_C_FIFO_WM_MASK: c_uint = 0x7F;
pub const DSP_PLANE_C_FIFO_WM_SHIFT: c_int = 0;
pub const DSPFW3: c_uint = 0x7003c;
pub const DSPFW4: c_uint = 0x70050;
pub const DSPFW5: c_uint = 0x70054;
pub const DSP_PLANE_B_FIFO_WM1_SHIFT: c_int = 24;
pub const DSP_PLANE_A_FIFO_WM1_SHIFT: c_int = 16;
pub const CURSOR_B_FIFO_WM1_SHIFT: c_int = 8;
pub const CURSOR_FIFO_SR_WM1_SHIFT: c_int = 0;
pub const DSPFW6: c_uint = 0x70058;
pub const DSPCHICKENBIT: c_uint = 0x70400;
pub const DSPACNTR: c_uint = 0x70180;
pub const DSPBCNTR: c_uint = 0x71180;
pub const DSPCCNTR: c_uint = 0x72180;

pub const DISPLAY_PLANE_DISABLE: c_int = 0;

pub const DISPPLANE_GAMMA_DISABLE: c_int = 0;

pub const DISPPLANE_STEREO_DISABLE: c_int = 0;

pub const DISPPLANE_SEL_PIPE_POS: c_int = 24;
pub const DISPPLANE_SEL_PIPE_A: c_int = 0;

pub const DISPPLANE_SRC_KEY_DISABLE: c_int = 0;

pub const DISPPLANE_NO_LINE_DOUBLE: c_int = 0;
pub const DISPPLANE_STEREO_POLARITY_FIRST: c_int = 0;

// plane B only

pub const DISPPLANE_ALPHA_TRANS_DISABLE: c_int = 0;
pub const DISPPLANE_SPRITE_ABOVE_DISPLAYA: c_int = 0;

pub const DSPABASE: c_uint = 0x70184;
pub const DSPALINOFF: c_uint = 0x70184;
pub const DSPASTRIDE: c_uint = 0x70188;
pub const DSPBBASE: c_uint = 0x71184;

pub const DSPBSTRIDE: c_uint = 0x71188;
pub const DSPCBASE: c_uint = 0x72184;
pub const DSPCLINOFF: c_uint = 0x72184;
pub const DSPCSTRIDE: c_uint = 0x72188;
pub const DSPAKEYVAL: c_uint = 0x70194;
pub const DSPAKEYMASK: c_uint = 0x70198;
pub const DSPAPOS: c_uint = 0x7018C	/* reserved */;
pub const DSPASIZE: c_uint = 0x70190;
pub const DSPBPOS: c_uint = 0x7118C;
pub const DSPBSIZE: c_uint = 0x71190;
pub const DSPCPOS: c_uint = 0x7218C;
pub const DSPCSIZE: c_uint = 0x72190;
pub const DSPASURF: c_uint = 0x7019C;
pub const DSPATILEOFF: c_uint = 0x701A4;
pub const DSPBSURF: c_uint = 0x7119C;
pub const DSPBTILEOFF: c_uint = 0x711A4;
pub const DSPCSURF: c_uint = 0x7219C;
pub const DSPCTILEOFF: c_uint = 0x721A4;
pub const DSPCKEYMAXVAL: c_uint = 0x721A0;
pub const DSPCKEYMINVAL: c_uint = 0x72194;
pub const DSPCKEYMSK: c_uint = 0x72198;
pub const VGACNTRL: c_uint = 0x71400;

//
// Overlay registers
//
pub const OV_C_OFFSET: c_uint = 0x08000;
pub const OV_OVADD: c_uint = 0x30000;
pub const OV_DOVASTA: c_uint = 0x30008;

pub const OV_OGAMC5: c_uint = 0x30010;
pub const OV_OGAMC4: c_uint = 0x30014;
pub const OV_OGAMC3: c_uint = 0x30018;
pub const OV_OGAMC2: c_uint = 0x3001C;
pub const OV_OGAMC1: c_uint = 0x30020;
pub const OV_OGAMC0: c_uint = 0x30024;
pub const OVC_OVADD: c_uint = 0x38000;
pub const OVC_DOVCSTA: c_uint = 0x38008;
pub const OVC_OGAMC5: c_uint = 0x38010;
pub const OVC_OGAMC4: c_uint = 0x38014;
pub const OVC_OGAMC3: c_uint = 0x38018;
pub const OVC_OGAMC2: c_uint = 0x3801C;
pub const OVC_OGAMC1: c_uint = 0x38020;
pub const OVC_OGAMC0: c_uint = 0x38024;
//
// Some BIOS scratch area registers.  The 845 (and 830?) store the amount
// of video memory available to the BIOS in SWF1.
//
pub const SWF0: c_uint = 0x71410;
pub const SWF1: c_uint = 0x71414;
pub const SWF2: c_uint = 0x71418;
pub const SWF3: c_uint = 0x7141c;
pub const SWF4: c_uint = 0x71420;
pub const SWF5: c_uint = 0x71424;
pub const SWF6: c_uint = 0x71428;
//
// 855 scratch registers.
//
pub const SWF00: c_uint = 0x70410;
pub const SWF01: c_uint = 0x70414;
pub const SWF02: c_uint = 0x70418;
pub const SWF03: c_uint = 0x7041c;
pub const SWF04: c_uint = 0x70420;
pub const SWF05: c_uint = 0x70424;
pub const SWF06: c_uint = 0x70428;

pub const SWF30: c_uint = 0x72414;
pub const SWF31: c_uint = 0x72418;
pub const SWF32: c_uint = 0x7241c;
//
// Palette registers
//
pub const PALETTE_A: c_uint = 0x0a000;
pub const PALETTE_B: c_uint = 0x0a800;
pub const PALETTE_C: c_uint = 0x0ac00;
// Cursor A & B regs
pub const CURACNTR: c_uint = 0x70080;
pub const CURSOR_MODE_DISABLE: c_uint = 0x00;
pub const CURSOR_MODE_64_32B_AX: c_uint = 0x07;

pub const CURABASE: c_uint = 0x70084;
pub const CURAPOS: c_uint = 0x70088;
pub const CURSOR_POS_MASK: c_uint = 0x007FF;
pub const CURSOR_POS_SIGN: c_uint = 0x8000;
pub const CURSOR_X_SHIFT: c_int = 0;
pub const CURSOR_Y_SHIFT: c_int = 16;
pub const CURBCNTR: c_uint = 0x700c0;
pub const CURBBASE: c_uint = 0x700c4;
pub const CURBPOS: c_uint = 0x700c8;
pub const CURCCNTR: c_uint = 0x700e0;
pub const CURCBASE: c_uint = 0x700e4;
pub const CURCPOS: c_uint = 0x700e8;
//
// Interrupt Registers
//
pub const IER: c_uint = 0x020a0;
pub const IIR: c_uint = 0x020a4;
pub const IMR: c_uint = 0x020a8;
pub const ISR: c_uint = 0x020ac;
//
// MOORESTOWN delta registers
//
pub const MRST_DPLL_A: c_uint = 0x0f014;

pub const MRST_FPA0: c_uint = 0x0f040;
pub const MRST_FPA1: c_uint = 0x0f044;
pub const MRST_PERF_MODE: c_uint = 0x020f4;
//
// MEDFIELD HDMI registers
//
pub const HDMIPHYMISCCTL: c_uint = 0x61134;
pub const HDMI_PHY_POWER_DOWN: c_uint = 0x7f;
pub const HDMIB_CONTROL: c_uint = 0x61140;

// #define LVDS			0x61180

pub const MIPI: c_uint = 0x61190;
pub const MIPI_C: c_uint = 0x62190;

// Turns on border drawing to allow centered display.

pub const MIPIA_3LANE_MIPIC_1LANE: c_uint = 0x1;
pub const MIPIA_2LANE_MIPIC_2LANE: c_uint = 0x2;

pub const MIPI_TE_COUNT: c_uint = 0x61194;
// #define PP_CONTROL	0x61204

// #define PFIT_CONTROL	0x61230

// #define BLC_PWM_CTL		0x61254

// #define PIPEACONF 0x70008

// #define DSPACNTR		0x70180
pub const MRST_DSPABASE: c_uint = 0x7019c;
pub const MRST_DSPBBASE: c_uint = 0x7119c;
//
// Moorestown registers.
//
// MIPI IP registers
//
pub const MIPIC_REG_OFFSET: c_uint = 0x800;
pub const DEVICE_READY_REG: c_uint = 0xb000;

pub const EXIT_ULPS_DEV_READY: c_uint = 0x3;
pub const LP_OUTPUT_HOLD_RELEASE: c_uint = 0x810000;

pub const INTR_STAT_REG: c_uint = 0xb004;

pub const INTR_EN_REG: c_uint = 0xb008;
pub const DSI_FUNC_PRG_REG: c_uint = 0xb00c;
pub const DPI_CHANNEL_NUMBER_POS: c_uint = 0x03;
pub const DBI_CHANNEL_NUMBER_POS: c_uint = 0x05;
pub const FMT_DPI_POS: c_uint = 0x07;
pub const FMT_DBI_POS: c_uint = 0x0A;
pub const DBI_DATA_WIDTH_POS: c_uint = 0x0D;
// DPI PIXEL FORMATS
pub const RGB_565_FMT: c_uint = 0x01	/* RGB 565 FORMAT */;
pub const RGB_666_FMT: c_uint = 0x02	/* RGB 666 FORMAT */;
pub const LRGB_666_FMT: c_uint = 0x03	/* RGB LOOSELY PACKED;
// 666 FORMAT
//
pub const RGB_888_FMT: c_uint = 0x04	/* RGB 888 FORMAT */;
pub const VIRTUAL_CHANNEL_NUMBER_0: c_uint = 0x00	/* Virtual channel 0 */;
pub const VIRTUAL_CHANNEL_NUMBER_1: c_uint = 0x01	/* Virtual channel 1 */;
pub const VIRTUAL_CHANNEL_NUMBER_2: c_uint = 0x02	/* Virtual channel 2 */;
pub const VIRTUAL_CHANNEL_NUMBER_3: c_uint = 0x03	/* Virtual channel 3 */;
pub const DBI_NOT_SUPPORTED: c_uint = 0x00	/* command mode;
// is not supported
//
pub const DBI_DATA_WIDTH_16BIT: c_uint = 0x01	/* 16 bit data */;
pub const DBI_DATA_WIDTH_9BIT: c_uint = 0x02	/* 9 bit data */;
pub const DBI_DATA_WIDTH_8BIT: c_uint = 0x03	/* 8 bit data */;
pub const DBI_DATA_WIDTH_OPT1: c_uint = 0x04	/* option 1 */;
pub const DBI_DATA_WIDTH_OPT2: c_uint = 0x05	/* option 2 */;
pub const HS_TX_TIMEOUT_REG: c_uint = 0xb010;
pub const LP_RX_TIMEOUT_REG: c_uint = 0xb014;
pub const TURN_AROUND_TIMEOUT_REG: c_uint = 0xb018;
pub const DEVICE_RESET_REG: c_uint = 0xb01C;
pub const DPI_RESOLUTION_REG: c_uint = 0xb020;
pub const RES_V_POS: c_uint = 0x10;
pub const HORIZ_SYNC_PAD_COUNT_REG: c_uint = 0xb028;
pub const HORIZ_BACK_PORCH_COUNT_REG: c_uint = 0xb02C;
pub const HORIZ_FRONT_PORCH_COUNT_REG: c_uint = 0xb030;
pub const HORIZ_ACTIVE_AREA_COUNT_REG: c_uint = 0xb034;
pub const VERT_SYNC_PAD_COUNT_REG: c_uint = 0xb038;
pub const VERT_BACK_PORCH_COUNT_REG: c_uint = 0xb03c;
pub const VERT_FRONT_PORCH_COUNT_REG: c_uint = 0xb040;
pub const HIGH_LOW_SWITCH_COUNT_REG: c_uint = 0xb044;
pub const DPI_CONTROL_REG: c_uint = 0xb048;

pub const DPI_DATA_REG: c_uint = 0xb04c;
pub const DPI_BACK_LIGHT_ON_DATA: c_uint = 0x07;
pub const DPI_BACK_LIGHT_OFF_DATA: c_uint = 0x17;
pub const INIT_COUNT_REG: c_uint = 0xb050;
pub const MAX_RET_PAK_REG: c_uint = 0xb054;
pub const VIDEO_FMT_REG: c_uint = 0xb058;

pub const EOT_DISABLE_REG: c_uint = 0xb05c;

pub const LP_BYTECLK_REG: c_uint = 0xb060;
pub const LP_GEN_DATA_REG: c_uint = 0xb064;
pub const HS_GEN_DATA_REG: c_uint = 0xb068;
pub const LP_GEN_CTRL_REG: c_uint = 0xb06C;
pub const HS_GEN_CTRL_REG: c_uint = 0xb070;
pub const DCS_CHANNEL_NUMBER_POS: c_uint = 0x6;
pub const MCS_COMMANDS_POS: c_uint = 0x8;
pub const WORD_COUNTS_POS: c_uint = 0x8;
pub const MCS_PARAMETER_POS: c_uint = 0x10;
pub const GEN_FIFO_STAT_REG: c_uint = 0xb074;

pub const HS_LS_DBI_ENABLE_REG: c_uint = 0xb078;
pub const TXCLKESC_REG: c_uint = 0xb07c;
pub const DPHY_PARAM_REG: c_uint = 0xb080;
pub const DBI_BW_CTRL_REG: c_uint = 0xb084;
pub const CLK_LANE_SWT_REG: c_uint = 0xb088;
//
// MIPI Adapter registers
//
pub const MIPI_CONTROL_REG: c_uint = 0xb104;

pub const MIPI_DATA_ADDRESS_REG: c_uint = 0xb108;
pub const MIPI_DATA_LENGTH_REG: c_uint = 0xb10C;
pub const MIPI_COMMAND_ADDRESS_REG: c_uint = 0xb110;
pub const MIPI_COMMAND_LENGTH_REG: c_uint = 0xb114;
pub const MIPI_READ_DATA_RETURN_REG0: c_uint = 0xb118;
pub const MIPI_READ_DATA_RETURN_REG1: c_uint = 0xb11C;
pub const MIPI_READ_DATA_RETURN_REG2: c_uint = 0xb120;
pub const MIPI_READ_DATA_RETURN_REG3: c_uint = 0xb124;
pub const MIPI_READ_DATA_RETURN_REG4: c_uint = 0xb128;
pub const MIPI_READ_DATA_RETURN_REG5: c_uint = 0xb12C;
pub const MIPI_READ_DATA_RETURN_REG6: c_uint = 0xb130;
pub const MIPI_READ_DATA_RETURN_REG7: c_uint = 0xb134;
pub const MIPI_READ_DATA_VALID_REG: c_uint = 0xb138;
// DBI COMMANDS
pub const soft_reset: c_uint = 0x01;
//
// The display module performs a software reset.
// Registers are written with their SW Reset default values.
//
pub const get_power_mode: c_uint = 0x0a;
//
// The display module returns the current power mode
//
pub const get_address_mode: c_uint = 0x0b;
//
// The display module returns the current status.
//
pub const get_pixel_format: c_uint = 0x0c;
//
// This command gets the pixel format for the RGB image data
// used by the interface.
//
pub const get_display_mode: c_uint = 0x0d;
//
// The display module returns the Display Image Mode status.
//
pub const get_signal_mode: c_uint = 0x0e;
//
// The display module returns the Display Signal Mode.
//
pub const get_diagnostic_result: c_uint = 0x0f;
//
// The display module returns the self-diagnostic results following
// a Sleep Out command.
//
pub const enter_sleep_mode: c_uint = 0x10;
//
// This command causes the display module to enter the Sleep mode.
// In this mode, all unnecessary blocks inside the display module are
// disabled except interface communication. This is the lowest power
// mode the display module supports.
//
pub const exit_sleep_mode: c_uint = 0x11;
//
// This command causes the display module to exit Sleep mode.
// All blocks inside the display module are enabled.
//
pub const enter_partial_mode: c_uint = 0x12;
//
// This command causes the display module to enter the Partial Display
// Mode. The Partial Display Mode window is described by the
// set_partial_area command.
//
pub const enter_normal_mode: c_uint = 0x13;
//
// This command causes the display module to enter the Normal mode.
// Normal Mode is defined as Partial Display mode and Scroll mode are off
//
pub const exit_invert_mode: c_uint = 0x20;
//
// This command causes the display module to stop inverting the image
// data on the display device. The frame memory contents remain unchanged.
// No status bits are changed.
//
pub const enter_invert_mode: c_uint = 0x21;
//
// This command causes the display module to invert the image data only on
// the display device. The frame memory contents remain unchanged.
// No status bits are changed.
//
pub const set_gamma_curve: c_uint = 0x26;
//
// This command selects the desired gamma curve for the display device.
// Four fixed gamma curves are defined in section DCS spec.
//
pub const set_display_off: c_uint = 0x28;
// ************************************************************************* *\
pub const set_display_on: c_uint = 0x29;
// ************************************************************************* *\
pub const set_column_address: c_uint = 0x2a;
//
// This command defines the column extent of the frame memory accessed by
// the hostprocessor with the read_memory_continue and
// write_memory_continue commands.
// No status bits are changed.
//
pub const set_page_addr: c_uint = 0x2b;
//
// This command defines the page extent of the frame memory accessed by
// the host processor with the write_memory_continue and
// read_memory_continue command.
// No status bits are changed.
//
pub const write_mem_start: c_uint = 0x2c;
//
// This command transfers image data from the host processor to the
// display modules frame memory starting at the pixel location specified
// by preceding set_column_address and set_page_address commands.
//
pub const set_partial_area: c_uint = 0x30;
//
// This command defines the Partial Display mode s display area.
// There are two parameters associated with this command, the first
// defines the Start Row (SR) and the second the End Row (ER). SR and ER
// refer to the Frame Memory Line Pointer.
//
pub const set_scroll_area: c_uint = 0x33;
//
// This command defines the display modules Vertical Scrolling Area.
//
pub const set_tear_off: c_uint = 0x34;
//
// This command turns off the display modules Tearing Effect output
// signal on the TE signal line.
//
pub const set_tear_on: c_uint = 0x35;
//
// This command turns on the display modules Tearing Effect output signal
// on the TE signal line.
//
pub const set_address_mode: c_uint = 0x36;
//
// This command sets the data order for transfers from the host processor
// to display modules frame memory,bits B[7:5] and B3, and from the
// display modules frame memory to the display device, bits B[2:0] and B4.
//
pub const set_scroll_start: c_uint = 0x37;
//
// This command sets the start of the vertical scrolling area in the frame
// memory. The vertical scrolling area is fully defined when this command
// is used with the set_scroll_area command The set_scroll_start command
// has one parameter, the Vertical Scroll Pointer. The VSP defines the
// line in the frame memory that is written to the display device as the
// first line of the vertical scroll area.
//
pub const exit_idle_mode: c_uint = 0x38;
//
// This command causes the display module to exit Idle mode.
//
pub const enter_idle_mode: c_uint = 0x39;
//
// This command causes the display module to enter Idle Mode.
// In Idle Mode, color expression is reduced. Colors are shown on the
// display device using the MSB of each of the R, G and B color
// components in the frame memory
//
pub const set_pixel_format: c_uint = 0x3a;
//
// This command sets the pixel format for the RGB image data used by the
// interface.
// Bits D[6:4]  DPI Pixel Format Definition
// Bits D[2:0]  DBI Pixel Format Definition
// Bits D7 and D3 are not used.
//
pub const DCS_PIXEL_FORMAT_3bpp: c_uint = 0x1;
pub const DCS_PIXEL_FORMAT_8bpp: c_uint = 0x2;
pub const DCS_PIXEL_FORMAT_12bpp: c_uint = 0x3;
pub const DCS_PIXEL_FORMAT_16bpp: c_uint = 0x5;
pub const DCS_PIXEL_FORMAT_18bpp: c_uint = 0x6;
pub const DCS_PIXEL_FORMAT_24bpp: c_uint = 0x7;
pub const write_mem_cont: c_uint = 0x3c;
//
// This command transfers image data from the host processor to the
// display module's frame memory continuing from the pixel location
// following the previous write_memory_continue or write_memory_start
// command.
//
pub const set_tear_scanline: c_uint = 0x44;
//
// This command turns on the display modules Tearing Effect output signal
// on the TE signal line when the display module reaches line N.
//
pub const get_scanline: c_uint = 0x45;
//
// The display module returns the current scanline, N, used to update the
// display device. The total number of scanlines on a display device is
// defined as VSYNC + VBP + VACT + VFP.The first scanline is defined as
// the first line of V Sync and is denoted as Line 0.
// When in Sleep Mode, the value returned by get_scanline is undefined.
//
// MCS or Generic COMMANDS
// MCS/generic data type
pub const GEN_SHORT_WRITE_0: c_uint = 0x03  /* generic short write, no parameters */;
pub const GEN_SHORT_WRITE_1: c_uint = 0x13  /* generic short write, 1 parameters */;
pub const GEN_SHORT_WRITE_2: c_uint = 0x23  /* generic short write, 2 parameters */;
pub const GEN_READ_0: c_uint = 0x04  /* generic read, no parameters */;
pub const GEN_READ_1: c_uint = 0x14  /* generic read, 1 parameters */;
pub const GEN_READ_2: c_uint = 0x24  /* generic read, 2 parameters */;
pub const GEN_LONG_WRITE: c_uint = 0x29  /* generic long write */;
pub const MCS_SHORT_WRITE_0: c_uint = 0x05  /* MCS short write, no parameters */;
pub const MCS_SHORT_WRITE_1: c_uint = 0x15  /* MCS short write, 1 parameters */;
pub const MCS_READ: c_uint = 0x06  /* MCS read, no parameters */;
pub const MCS_LONG_WRITE: c_uint = 0x39  /* MCS long write */;
// MCS/generic commands
// TPO MCS
pub const write_display_profile: c_uint = 0x50;
pub const write_display_brightness: c_uint = 0x51;
pub const write_ctrl_display: c_uint = 0x53;
pub const write_ctrl_cabc: c_uint = 0x55;
pub const UI_IMAGE: c_uint = 0x01;
pub const STILL_IMAGE: c_uint = 0x02;
pub const MOVING_IMAGE: c_uint = 0x03;
pub const write_hysteresis: c_uint = 0x57;
pub const write_gamma_setting: c_uint = 0x58;
pub const write_cabc_min_bright: c_uint = 0x5e;
pub const write_kbbc_profile: c_uint = 0x60;
// TMD MCS
pub const tmd_write_display_brightness: c_uint = 0x8c;
//
// This command is used to control ambient light, panel backlight
// brightness and gamma settings.
//

// DCS Interface Pixel Formats
pub const DCS_PIXEL_FORMAT_3BPP: c_uint = 0x1;
pub const DCS_PIXEL_FORMAT_8BPP: c_uint = 0x2;
pub const DCS_PIXEL_FORMAT_12BPP: c_uint = 0x3;
pub const DCS_PIXEL_FORMAT_16BPP: c_uint = 0x5;
pub const DCS_PIXEL_FORMAT_18BPP: c_uint = 0x6;
pub const DCS_PIXEL_FORMAT_24BPP: c_uint = 0x7;
// ONE PARAMETER READ DATA
pub const addr_mode_data: c_uint = 0xfc;
pub const diag_res_data: c_uint = 0x00;
pub const disp_mode_data: c_uint = 0x23;
pub const pxl_fmt_data: c_uint = 0x77;
pub const pwr_mode_data: c_uint = 0x74;
pub const sig_mode_data: c_uint = 0x00;
// TWO PARAMETERS READ DATA
pub const scanline_data1: c_uint = 0xff;
pub const scanline_data2: c_uint = 0xff;
pub const NON_BURST_MODE_SYNC_PULSE: c_uint = 0x01	/* Non Burst Mode;
// with Sync Pulse
//
pub const NON_BURST_MODE_SYNC_EVENTS: c_uint = 0x02	/* Non Burst Mode;
// with Sync events
//
pub const BURST_MODE: c_uint = 0x03	/* Burst Mode */;
pub const DBI_COMMAND_BUFFER_SIZE: c_uint = 0x240   /* 0x32 */    /* 0x120 */;
// Allocate at least
// 0x100 Byte with 32
// byte alignment
//
pub const DBI_DATA_BUFFER_SIZE: c_uint = 0x120	/* Allocate at least;
// 0x100 Byte with 32
// byte alignment
//
pub const DBI_CB_TIME_OUT: c_uint = 0xFFFF;
pub const GEN_FB_TIME_OUT: c_int = 2000;
pub const SKU_83: c_uint = 0x01;
pub const SKU_100: c_uint = 0x02;
pub const SKU_100L: c_uint = 0x04;
pub const SKU_BYPASS: c_uint = 0x08;
// Some handy macros for playing with bitfields.

// PCI config space
pub const SB_PCKT: c_uint = 0x02100 /* cedarview */;

pub const DSPCLK_GATE_D: c_uint = 0x6200;

pub const RAMCLK_GATE_D: c_uint = 0x6210;
// 32-bit value read/written from the DPIO reg.
pub const SB_DATA: c_uint = 0x02104 /* cedarview */;
// 32-bit address of the DPIO reg to be read/written.
pub const SB_ADDR: c_uint = 0x02108 /* cedarview */;
pub const DPIO_CFG: c_uint = 0x02110 /* cedarview */;

// reset is active low

// Cedarview sideband registers
pub const _SB_M_A: c_uint = 0x8008;
pub const _SB_M_B: c_uint = 0x8028;

pub const _SB_N_VCO_A: c_uint = 0x8014;
pub const _SB_N_VCO_B: c_uint = 0x8034;

pub const SB_N_VCO_SEL_SHIFT: c_int = 30;

pub const SB_N_DIVIDER_SHIFT: c_int = 26;

pub const SB_N_CB_TUNE_SHIFT: c_int = 24;
// the bit 14:13 is used to select between the different reference clock for Pipe A/B
pub const SB_REF_DPLLA: c_uint = 0x8010;
pub const SB_REF_DPLLB: c_uint = 0x8030;

// For the DPLL B, it will use the reference clk from DPLL A when using (2 << 13)
pub const _SB_REF_A: c_uint = 0x8018;
pub const _SB_REF_B: c_uint = 0x8038;

pub const _SB_P_A: c_uint = 0x801c;
pub const _SB_P_B: c_uint = 0x803c;

pub const SB_P2_DIVIDER_SHIFT: c_int = 30;

pub const SB_P1_DIVIDER_SHIFT: c_int = 12;
pub const PSB_LANE0: c_uint = 0x120;
pub const PSB_LANE1: c_uint = 0x220;
pub const PSB_LANE2: c_uint = 0x2320;
pub const PSB_LANE3: c_uint = 0x2420;

pub const DP_B: c_uint = 0x64100;
pub const DP_C: c_uint = 0x64200;

// Link training mode - select a suitable mode for each stage

pub const DP_LINK_TRAIN_SHIFT: c_int = 28;
// Signal voltages. These are mostly controlled by the other end

pub const DP_VOLTAGE_SHIFT: c_int = 25;
// Signal pre-emphasis levels, like voltages, the other end tells us what
// they want
//

pub const DP_PRE_EMPHASIS_SHIFT: c_int = 22;
// How many wires to use. I guess 3 was too hard

// Mystic DPCD version 1.1 special mode

// locked once port is enabled

// sends the clock on lane 15 of the PEG for debug

// limit RGB values to avoid confusing TVs

// Turn on the audio link

// vs and hs sync polarity

// A fantasy

// The aux channel provides a way to talk to the
// signal sink for DDC etc. Max packet size supported
// is 20 bytes in each direction, hence the 5 fixed
// data registers
//
pub const DPB_AUX_CH_CTL: c_uint = 0x64110;
pub const DPB_AUX_CH_DATA1: c_uint = 0x64114;
pub const DPB_AUX_CH_DATA2: c_uint = 0x64118;
pub const DPB_AUX_CH_DATA3: c_uint = 0x6411c;
pub const DPB_AUX_CH_DATA4: c_uint = 0x64120;
pub const DPB_AUX_CH_DATA5: c_uint = 0x64124;
pub const DPC_AUX_CH_CTL: c_uint = 0x64210;
pub const DPC_AUX_CH_DATA1: c_uint = 0x64214;
pub const DPC_AUX_CH_DATA2: c_uint = 0x64218;
pub const DPC_AUX_CH_DATA3: c_uint = 0x6421c;
pub const DPC_AUX_CH_DATA4: c_uint = 0x64220;
pub const DPC_AUX_CH_DATA5: c_uint = 0x64224;

pub const DP_AUX_CH_CTL_MESSAGE_SIZE_SHIFT: c_int = 20;

pub const DP_AUX_CH_CTL_PRECHARGE_2US_SHIFT: c_int = 16;

pub const DP_AUX_CH_CTL_BIT_CLOCK_2X_SHIFT: c_int = 0;
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
pub const _PIPEA_GMCH_DATA_M: c_uint = 0x70050;
pub const _PIPEB_GMCH_DATA_M: c_uint = 0x71050;
// Transfer unit size for display port - 1, default is 0x3f (for TU size 64)

pub const PIPE_GMCH_DATA_M_TU_SIZE_SHIFT: c_int = 25;

pub const _PIPEA_GMCH_DATA_N: c_uint = 0x70054;
pub const _PIPEB_GMCH_DATA_N: c_uint = 0x71054;

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
pub const _PIPEA_DP_LINK_M: c_uint = 0x70060;
pub const _PIPEB_DP_LINK_M: c_uint = 0x71060;

pub const _PIPEA_DP_LINK_N: c_uint = 0x70064;
pub const _PIPEB_DP_LINK_N: c_uint = 0x71064;

