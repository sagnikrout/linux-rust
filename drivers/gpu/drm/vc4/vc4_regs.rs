//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vc4/vc4_regs.h
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
// Copyright © 2014-2015 Broadcom
//

// Using the GNU statement expression extension

pub const V3D_IDENT0: c_uint = 0x00000;

pub const V3D_IDENT1: c_uint = 0x00004;
// Multiples of 1kb

pub const V3D_IDENT2: c_uint = 0x00008;
pub const V3D_SCRATCH: c_uint = 0x00010;
pub const V3D_L2CACTL: c_uint = 0x00020;

pub const V3D_SLCACTL: c_uint = 0x00024;

pub const V3D_INTCTL: c_uint = 0x00030;
pub const V3D_INTENA: c_uint = 0x00034;
pub const V3D_INTDIS: c_uint = 0x00038;

pub const V3D_CT0CS: c_uint = 0x00100;
pub const V3D_CT1CS: c_uint = 0x00104;

pub const V3D_CT0EA: c_uint = 0x00108;
pub const V3D_CT1EA: c_uint = 0x0010c;

pub const V3D_CT0CA: c_uint = 0x00110;
pub const V3D_CT1CA: c_uint = 0x00114;

pub const V3D_CT00RA0: c_uint = 0x00118;
pub const V3D_CT01RA0: c_uint = 0x0011c;

pub const V3D_CT0LC: c_uint = 0x00120;
pub const V3D_CT1LC: c_uint = 0x00124;

pub const V3D_CT0PC: c_uint = 0x00128;
pub const V3D_CT1PC: c_uint = 0x0012c;

pub const V3D_PCS: c_uint = 0x00130;

pub const V3D_BFC: c_uint = 0x00134;
pub const V3D_RFC: c_uint = 0x00138;
pub const V3D_BPCA: c_uint = 0x00300;
pub const V3D_BPCS: c_uint = 0x00304;
pub const V3D_BPOA: c_uint = 0x00308;
pub const V3D_BPOS: c_uint = 0x0030c;
pub const V3D_BXCF: c_uint = 0x00310;
pub const V3D_SQRSV0: c_uint = 0x00410;
pub const V3D_SQRSV1: c_uint = 0x00414;
pub const V3D_SQCNTL: c_uint = 0x00418;
pub const V3D_SRQPC: c_uint = 0x00430;
pub const V3D_SRQUA: c_uint = 0x00434;
pub const V3D_SRQUL: c_uint = 0x00438;
pub const V3D_SRQCS: c_uint = 0x0043c;
pub const V3D_VPACNTL: c_uint = 0x00500;
pub const V3D_VPMBASE: c_uint = 0x00504;
pub const V3D_PCTRC: c_uint = 0x00670;
pub const V3D_PCTRE: c_uint = 0x00674;

pub const V3D_DBGE: c_uint = 0x00f00;
pub const V3D_FDBGO: c_uint = 0x00f04;
pub const V3D_FDBGB: c_uint = 0x00f08;
pub const V3D_FDBGR: c_uint = 0x00f0c;
pub const V3D_FDBGS: c_uint = 0x00f10;
pub const V3D_ERRSTAT: c_uint = 0x00f20;
pub const PV_CONTROL: c_uint = 0x00;

pub const PV_V_CONTROL: c_uint = 0x04;

pub const PV_VSYNCD_EVEN: c_uint = 0x08;
pub const PV_HORZA: c_uint = 0x0c;

pub const PV_HORZB: c_uint = 0x10;

pub const PV_VERTA: c_uint = 0x14;

pub const PV_VERTB: c_uint = 0x18;

pub const PV_VERTA_EVEN: c_uint = 0x1c;
pub const PV_VERTB_EVEN: c_uint = 0x20;
pub const PV_INTEN: c_uint = 0x24;
pub const PV_INTSTAT: c_uint = 0x28;

pub const PV_STAT: c_uint = 0x2c;
pub const PV_HACT_ACT: c_uint = 0x30;
pub const PV_MUX_CFG: c_uint = 0x34;

pub const PV_PIPE_INIT_CTRL: c_uint = 0x94;

pub const SCALER_CHANNELS_COUNT: c_int = 3;
pub const SCALER_DISPCTRL: c_uint = 0x00000000;
// Global register for clock gating the HVS

// Enables Display 0 short line and underrun contribution to
// SCALER_DISPSTAT_IRQDISP0.  Note that short frame contributions are
// always enabled.
//

// Enables Display 0 end-of-line-N contribution to
// SCALER_DISPSTAT_IRQDISP0
//

// Enables Display 0 EOF contribution to SCALER_DISPSTAT_IRQDISP0

// Enables interrupt generation on the enabled EOF/EOLN/EISLUR
// bits and short frames..
//

// Enables interrupt generation on scaler profiler interrupt.

pub const SCALER_DISPSTAT: c_uint = 0x00000004;

// Set when the DISPEOLN line is done compositing.

// Set when VSTART is seen but there are still pixels in the current
// output line.
//

// Set when HSTART is seen but there are still pixels in the current
// output line.
//

// Set when the downstream tries to read from the display FIFO
// while it's empty.
//

// Set when the display mode changes from RUN to EOF

// Set on AXI invalid DMA ID error.

// Set on AXI slave read decode error

// Set on AXI slave write decode error

// Set when SCALER_DISPSTAT_DMA_ERROR is set, or
// SCALER_DISPSTAT_RESP_ERROR is not SCALER_DISPSTAT_RESP_OKAY.
//

// Set when any of the EOF/EOLN/ESFRAME/ESLINE bits are set and their
// corresponding interrupt bit is enabled in DISPCTRL.
//

// On read, the profiler interrupt.  On write, clear *all* interrupt bits.

pub const SCALER_DISPID: c_uint = 0x00000008;
pub const SCALER_DISPECTRL: c_uint = 0x0000000c;

pub const SCALER_DISPPROF: c_uint = 0x00000010;
pub const SCALER_DISPDITHER: c_uint = 0x00000014;

pub const SCALER_DISPEOLN: c_uint = 0x00000018;

pub const SCALER_DISPLIST0: c_uint = 0x00000020;
pub const SCALER_DISPLIST1: c_uint = 0x00000024;
pub const SCALER_DISPLIST2: c_uint = 0x00000028;
pub const SCALER_DISPLSTAT: c_uint = 0x0000002c;

pub const SCALER_DISPLACT0: c_uint = 0x00000030;
pub const SCALER_DISPLACT1: c_uint = 0x00000034;
pub const SCALER_DISPLACT2: c_uint = 0x00000038;

pub const SCALER_DISPCTRL0: c_uint = 0x00000040;

// Generates a single frame when VSTART is seen and stops at the last
// pixel read from the FIFO.
//

// Processes a single context in the dlist and then task switch,
// instead of an entire line.
//

// Set to have DISPSLAVE return 2 16bpp pixels and no status data.

// Turns on output to the DISPSLAVE register instead of the normal
// FIFO.
//

// Generates a single frame when VSTART is seen and stops at the last
// pixel read from the FIFO.
//

// Processes a single context in the dlist and then task switch,
// instead of an entire line.
//

pub const SCALER_DISPBKGND0: c_uint = 0x00000044;

// Enables filling the scaler line with the RGB value in the low 24
// bits before compositing.  Costs cycles, so should be skipped if
// opaque display planes will cover everything.
//

pub const SCALER_DISPSTAT0: c_uint = 0x00000048;

pub const SCALER_DISPBASE0: c_uint = 0x0000004c;
// Last pixel in the COB (display FIFO memory) allocated to this HVS
// channel.  Must be 4-pixel aligned (and thus 4 pixels less than the
// next COB base).
//

// First pixel in the COB (display FIFO memory) allocated to this HVS
// channel.  Must be 4-pixel aligned.
//

pub const SCALER_DISPCTRL1: c_uint = 0x00000050;
pub const SCALER_DISPBKGND1: c_uint = 0x00000054;

pub const SCALER_DISPSTAT1: c_uint = 0x00000058;

pub const SCALER_DISPBASE1: c_uint = 0x0000005c;

pub const SCALER_DISPCTRL2: c_uint = 0x00000060;

pub const SCALER_DISPBKGND2: c_uint = 0x00000064;
pub const SCALER_DISPSTAT2: c_uint = 0x00000068;

pub const SCALER_DISPBASE2: c_uint = 0x0000006c;
pub const SCALER_DISPALPHA2: c_uint = 0x00000070;
pub const SCALER_GAMADDR: c_uint = 0x00000078;

// Enables all gamma ramp SRAMs, not just those of CRTCs with gamma
// enabled.
//

pub const SCALER_OLEDOFFS: c_uint = 0x00000080;
// Clamps R to [16,235] and G/B to [16,240].

// Chooses which display FIFO the matrix applies to.

// Offsets are 8-bit 2s-complement.

// The coefficients are S0.9 fractions.
pub const SCALER_OLEDCOEF0: c_uint = 0x00000084;

pub const SCALER_OLEDCOEF1: c_uint = 0x00000088;

pub const SCALER_OLEDCOEF2: c_uint = 0x0000008c;

// Slave addresses for DMAing from HVS composition output to other
// devices.  The top bits are valid only in !FIFO32 mode.
//
pub const SCALER_DISPSLAVE0: c_uint = 0x000000c0;
pub const SCALER_DISPSLAVE1: c_uint = 0x000000c9;
pub const SCALER_DISPSLAVE2: c_uint = 0x000000d0;

// Set when the current line has been read and an HSTART is required.

// Set when the display FIFO is empty.

// Set when there is RGB data ready to read.

pub const SCALER_GAMDATA: c_uint = 0x000000e0;
pub const SCALER_DLIST_START: c_uint = 0x00002000;
pub const SCALER_DLIST_SIZE: c_uint = 0x00004000;
pub const SCALER5_DLIST_START: c_uint = 0x00004000;
pub const SCALER6_VERSION: c_uint = 0x00000000;

pub const SCALER6_CXM_SIZE: c_uint = 0x00000004;
pub const SCALER6_LBM_SIZE: c_uint = 0x00000008;
pub const SCALER6_UBM_SIZE: c_uint = 0x0000000c;
pub const SCALER6_COBA_SIZE: c_uint = 0x00000010;
pub const SCALER6_COB_SIZE: c_uint = 0x00000014;
pub const SCALER6_CONTROL: c_uint = 0x00000020;

pub const SCALER6_FETCHER_STATUS: c_uint = 0x00000024;
pub const SCALER6_FETCH_STATUS: c_uint = 0x00000028;
pub const SCALER6_HANDLE_ERROR: c_uint = 0x0000002c;
pub const SCALER6_DISP0_CTRL0: c_uint = 0x00000030;

pub const SCALER6_DISP0_CTRL1: c_uint = 0x00000034;

pub const SCALER6_DISP0_BGND: c_uint = 0x00000038;

pub const SCALER6_DISP0_LPTRS: c_uint = 0x0000003c;

pub const SCALER6_DISP0_COB: c_uint = 0x00000040;

pub const SCALER6_DISP0_STATUS: c_uint = 0x00000044;

pub const SCALER6_DISP0_DL: c_uint = 0x00000048;

pub const SCALER6_DISP0_RUN: c_uint = 0x0000004c;
pub const SCALER6_DISP1_CTRL0: c_uint = 0x00000050;
pub const SCALER6_DISP1_CTRL1: c_uint = 0x00000054;
pub const SCALER6_DISP1_BGND: c_uint = 0x00000058;
pub const SCALER6_DISP1_LPTRS: c_uint = 0x0000005c;
pub const SCALER6_DISP1_COB: c_uint = 0x00000060;
pub const SCALER6_DISP1_STATUS: c_uint = 0x00000064;
pub const SCALER6_DISP1_DL: c_uint = 0x00000068;
pub const SCALER6_DISP1_RUN: c_uint = 0x0000006c;
pub const SCALER6_DISP2_CTRL0: c_uint = 0x00000070;
pub const SCALER6_DISP2_CTRL1: c_uint = 0x00000074;
pub const SCALER6_DISP2_BGND: c_uint = 0x00000078;
pub const SCALER6_DISP2_LPTRS: c_uint = 0x0000007c;
pub const SCALER6_DISP2_COB: c_uint = 0x00000080;
pub const SCALER6_DISP2_STATUS: c_uint = 0x00000084;
pub const SCALER6_DISP2_DL: c_uint = 0x00000088;
pub const SCALER6_DISP2_RUN: c_uint = 0x0000008c;
pub const SCALER6_EOLN: c_uint = 0x00000090;
pub const SCALER6_DL_STATUS: c_uint = 0x00000094;
pub const SCALER6_BFG_MISC: c_uint = 0x0000009c;
pub const SCALER6_QOS0: c_uint = 0x000000a0;
pub const SCALER6_PROF0: c_uint = 0x000000a4;
pub const SCALER6_QOS1: c_uint = 0x000000a8;
pub const SCALER6_PROF1: c_uint = 0x000000ac;
pub const SCALER6_QOS2: c_uint = 0x000000b0;
pub const SCALER6_PROF2: c_uint = 0x000000b4;
pub const SCALER6_PRI_MAP0: c_uint = 0x000000b8;
pub const SCALER6_PRI_MAP1: c_uint = 0x000000bc;
pub const SCALER6_HISTCTRL: c_uint = 0x000000c0;
pub const SCALER6_HISTBIN0: c_uint = 0x000000c4;
pub const SCALER6_HISTBIN1: c_uint = 0x000000c8;
pub const SCALER6_HISTBIN2: c_uint = 0x000000cc;
pub const SCALER6_HISTBIN3: c_uint = 0x000000d0;
pub const SCALER6_HISTBIN4: c_uint = 0x000000d4;
pub const SCALER6_HISTBIN5: c_uint = 0x000000d8;
pub const SCALER6_HISTBIN6: c_uint = 0x000000dc;
pub const SCALER6_HISTBIN7: c_uint = 0x000000e0;
pub const SCALER6_HDR_CFG_REMAP: c_uint = 0x000000f4;
pub const SCALER6_COL_SPACE: c_uint = 0x000000f8;
pub const SCALER6_HVS_ID: c_uint = 0x000000fc;
pub const SCALER6_CFC1: c_uint = 0x00000100;
pub const SCALER6_DISP_UPM_ISO0: c_uint = 0x00000200;
pub const SCALER6_DISP_UPM_ISO1: c_uint = 0x00000204;
pub const SCALER6_DISP_UPM_ISO2: c_uint = 0x00000208;
pub const SCALER6_DISP_LBM_ISO0: c_uint = 0x0000020c;
pub const SCALER6_DISP_LBM_ISO1: c_uint = 0x00000210;
pub const SCALER6_DISP_LBM_ISO2: c_uint = 0x00000214;
pub const SCALER6_DISP_COB_ISO0: c_uint = 0x00000218;
pub const SCALER6_DISP_COB_ISO1: c_uint = 0x0000021c;
pub const SCALER6_DISP_COB_ISO2: c_uint = 0x00000220;
pub const SCALER6_BAD_COB: c_uint = 0x00000224;
pub const SCALER6_BAD_LBM: c_uint = 0x00000228;
pub const SCALER6_BAD_UPM: c_uint = 0x0000022c;
pub const SCALER6_BAD_AXI: c_uint = 0x00000230;
pub const SCALER6D_VERSION: c_uint = 0x00000000;
pub const SCALER6D_CXM_SIZE: c_uint = 0x00000004;
pub const SCALER6D_LBM_SIZE: c_uint = 0x00000008;
pub const SCALER6D_UBM_SIZE: c_uint = 0x0000000c;
pub const SCALER6D_COBA_SIZE: c_uint = 0x00000010;
pub const SCALER6D_COB_SIZE: c_uint = 0x00000014;
pub const SCALER6D_CONTROL: c_uint = 0x00000020;
pub const SCALER6D_FETCHER_STATUS: c_uint = 0x00000024;
pub const SCALER6D_FETCH_STATUS: c_uint = 0x00000028;
pub const SCALER6D_HANDLE_ERROR: c_uint = 0x0000002c;
pub const SCALER6D_EOLN: c_uint = 0x00000030;
pub const SCALER6D_DL_STATUS: c_uint = 0x00000034;
pub const SCALER6D_PRI_MAP0: c_uint = 0x00000038;
pub const SCALER6D_PRI_MAP1: c_uint = 0x0000003c;
pub const SCALER6D_HISTCTRL: c_uint = 0x000000d0;
pub const SCALER6D_HISTBIN0: c_uint = 0x000000d4;
pub const SCALER6D_HISTBIN1: c_uint = 0x000000d8;
pub const SCALER6D_HISTBIN2: c_uint = 0x000000dc;
pub const SCALER6D_HISTBIN3: c_uint = 0x000000e0;
pub const SCALER6D_HISTBIN4: c_uint = 0x000000e4;
pub const SCALER6D_HISTBIN5: c_uint = 0x000000e8;
pub const SCALER6D_HISTBIN6: c_uint = 0x000000ec;
pub const SCALER6D_HISTBIN7: c_uint = 0x000000f0;
pub const SCALER6D_HVS_ID: c_uint = 0x000000fc;
pub const SCALER6D_DISP0_CTRL0: c_uint = 0x00000100;
pub const SCALER6D_DISP0_CTRL1: c_uint = 0x00000104;
pub const SCALER6D_DISP0_BGND: c_uint = 0x00000108;
pub const SCALER6D_DISP0_LPTRS: c_uint = 0x00000110;
pub const SCALER6D_DISP0_COB: c_uint = 0x00000114;
pub const SCALER6D_DISP0_STATUS: c_uint = 0x00000118;
pub const SCALER6D_DISP0_CTRL0: c_uint = 0x00000100;
pub const SCALER6D_DISP0_CTRL1: c_uint = 0x00000104;
pub const SCALER6D_DISP0_BGND0: c_uint = 0x00000108;
pub const SCALER6D_DISP0_BGND1: c_uint = 0x0000010c;
pub const SCALER6D_DISP0_LPTRS: c_uint = 0x00000110;
pub const SCALER6D_DISP0_COB: c_uint = 0x00000114;
pub const SCALER6D_DISP0_STATUS: c_uint = 0x00000118;
pub const SCALER6D_DISP0_DL: c_uint = 0x0000011c;
pub const SCALER6D_DISP0_RUN: c_uint = 0x00000120;
pub const SCALER6D_QOS0: c_uint = 0x00000124;
pub const SCALER6D_PROF0: c_uint = 0x00000128;
pub const SCALER6D_DISP1_CTRL0: c_uint = 0x00000140;
pub const SCALER6D_DISP1_CTRL1: c_uint = 0x00000144;
pub const SCALER6D_DISP1_BGND0: c_uint = 0x00000148;
pub const SCALER6D_DISP1_BGND1: c_uint = 0x0000014c;
pub const SCALER6D_DISP1_LPTRS: c_uint = 0x00000150;
pub const SCALER6D_DISP1_COB: c_uint = 0x00000154;
pub const SCALER6D_DISP1_STATUS: c_uint = 0x00000158;
pub const SCALER6D_DISP1_DL: c_uint = 0x0000015c;
pub const SCALER6D_DISP1_RUN: c_uint = 0x00000160;
pub const SCALER6D_QOS1: c_uint = 0x00000164;
pub const SCALER6D_PROF1: c_uint = 0x00000168;
pub const SCALER6D_DISP2_CTRL0: c_uint = 0x00000180;
pub const SCALER6D_DISP2_CTRL1: c_uint = 0x00000184;
pub const SCALER6D_DISP2_BGND0: c_uint = 0x00000188;
pub const SCALER6D_DISP2_BGND1: c_uint = 0x0000018c;
pub const SCALER6D_DISP2_LPTRS: c_uint = 0x00000190;
pub const SCALER6D_DISP2_COB: c_uint = 0x00000194;
pub const SCALER6D_DISP2_STATUS: c_uint = 0x00000198;
pub const SCALER6D_DISP2_DL: c_uint = 0x0000019c;
pub const SCALER6D_DISP2_RUN: c_uint = 0x000001a0;
pub const SCALER6D_QOS2: c_uint = 0x000001a4;
pub const SCALER6D_PROF2: c_uint = 0x000001a8;

// If set, then multichannel, otherwise 2 channel.

// If set, then AUDIO_LAYOUT overrides audio_cea_mask

// When set, the CTS_PERIOD counts based on MAI bus sync pulse instead
// of pixel clock.
//

// When set, no CRP packets will be sent.

// If set, generates CTS values based on N, audio clock, and video
// clock.  N must be divisible by 128.
//

// Horizontal active pixels (hdisplay).

// Horizontal back porch (htotal - hsync_end).

// Horizontal sync pulse (hsync_end - hsync_start).

// Horizontal front porch (hsync_start - hdisplay).

// Vertical sync pulse (vsync_end - vsync_start).

// Vertical front porch (vsync_start - vdisplay).

// Vertical active lines (vdisplay).

// Vertical sync pulse offset (for interlaced)

// Vertical pack porch (vtotal - vsync_end).

// Set when the transmission has ended.

// If set, transmission was acked on the 1st or 2nd attempt (only one
// retry is attempted).  If in continuous mode, this means TX needs to
// be filled if !TX_EOM.
//

// Number of bytes received for the message.

// Sets continuous receive mode.  Generates interrupt after each 8
// bytes to signal that RX_DATA should be consumed, and at RX_EOM.
//
// If disabled, maximum 16 bytes will be received (including header),
// and interrupt at RX_EOM.  Later bytes will be acked but not put
// into the RX_DATA.
//

// Set this after a CEC interrupt.

// Starts a TX.  Will wait for appropriate idel time before CEC
// activity. Must be cleared in between transmits.
//

// Device's CEC address

// Divides off of HSM clock to generate CEC bit clock.
// With the current defaults the CEC bit clock is 40 kHz = 25 usec

// Set these fields to how many bit clock cycles get to that many
// microseconds.
//

// Debug: Current receive value on the CEC pad.

// Debug: Override CEC output to 0.

// Set when audio stream is received at a slower rate than the
// sampling period, so MAI fifo goes empty.  Write 1 to clear.
//

// If set, MAI bus generates SPDIF (bit 31) parity instead of passing
// through.
//

// Underflow error status bit, write 1 to clear.

// Overflow error status bit, write 1 to clear.

// Single-shot reset bit.  Read value is undefined.

// Divider from HDMI HSM clock to MAI serial clock.  Sampling period
// converges to N / (M + 1) cycles.
//

// HVS display list information.
pub const HVS_BOOTLOADER_DLIST_END: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hvs_pixel_format {
// 8bpp
    HVS_PIXEL_FORMAT_RGB332 = 0,
// 16bpp
    HVS_PIXEL_FORMAT_RGBA4444 = 1,
    HVS_PIXEL_FORMAT_RGB555 = 2,
    HVS_PIXEL_FORMAT_RGBA5551 = 3,
    HVS_PIXEL_FORMAT_RGB565 = 4,
// 24bpp
    HVS_PIXEL_FORMAT_RGB888 = 5,
    HVS_PIXEL_FORMAT_RGBA6666 = 6,
// 32bpp
    HVS_PIXEL_FORMAT_RGBA8888 = 7,

    HVS_PIXEL_FORMAT_YCBCR_YUV420_3PLANE = 8,
    HVS_PIXEL_FORMAT_YCBCR_YUV420_2PLANE = 9,
    HVS_PIXEL_FORMAT_YCBCR_YUV422_3PLANE = 10,
    HVS_PIXEL_FORMAT_YCBCR_YUV422_2PLANE = 11,
    HVS_PIXEL_FORMAT_H264 = 12,
    HVS_PIXEL_FORMAT_PALETTE = 13,
    HVS_PIXEL_FORMAT_YUV444_RGB = 14,
    HVS_PIXEL_FORMAT_AYUV444_RGB = 15,
    HVS_PIXEL_FORMAT_RGBA1010102 = 16,
    HVS_PIXEL_FORMAT_YCBCR_10BIT = 17,
}

// Note: the LSB is the rightmost character shown.  Only valid for
// HVS_PIXEL_FORMAT_RGB8888, not RGB888.
//
// For modes 332, 4444, 555, 5551, 6666, 8888, 10:10:10:2
pub const HVS_PIXEL_ORDER_RGBA: c_int = 0;
pub const HVS_PIXEL_ORDER_BGRA: c_int = 1;
pub const HVS_PIXEL_ORDER_ARGB: c_int = 2;
pub const HVS_PIXEL_ORDER_ABGR: c_int = 3;
// For modes 666 and 888 (4 & 5)
pub const HVS_PIXEL_ORDER_XBRG: c_int = 0;
pub const HVS_PIXEL_ORDER_XRBG: c_int = 1;
pub const HVS_PIXEL_ORDER_XRGB: c_int = 2;
pub const HVS_PIXEL_ORDER_XBGR: c_int = 3;
// For YCbCr modes (8-12, and 17)
pub const HVS_PIXEL_ORDER_XYCBCR: c_int = 0;
pub const HVS_PIXEL_ORDER_XYCRCB: c_int = 1;
pub const HVS_PIXEL_ORDER_YXCBCR: c_int = 2;
pub const HVS_PIXEL_ORDER_YXCRCB: c_int = 3;

pub const SCALER_CTL0_SIZE_SHIFT: c_int = 24;

pub const SCALER_CTL0_TILING_SHIFT: c_int = 20;
pub const SCALER_CTL0_TILING_LINEAR: c_int = 0;
pub const SCALER_CTL0_TILING_64B: c_int = 1;
pub const SCALER_CTL0_TILING_128B: c_int = 2;
pub const SCALER_CTL0_TILING_256B_OR_T: c_int = 3;

pub const SCALER_CTL0_KEY_MODE_SHIFT: c_int = 17;
pub const SCALER_CTL0_KEY_DISABLED: c_int = 0;
pub const SCALER_CTL0_KEY_LUMA_OR_COMMON_RGB: c_int = 1;

pub const SCALER_CTL0_ORDER_SHIFT: c_int = 13;

pub const SCALER_CTL0_RGBA_EXPAND_SHIFT: c_int = 11;
pub const SCALER_CTL0_RGBA_EXPAND_ZERO: c_int = 0;
pub const SCALER_CTL0_RGBA_EXPAND_LSB: c_int = 1;
pub const SCALER_CTL0_RGBA_EXPAND_MSB: c_int = 2;
pub const SCALER_CTL0_RGBA_EXPAND_ROUND: c_int = 3;

pub const SCALER_CTL0_SCL1_SHIFT: c_int = 8;

pub const SCALER_CTL0_SCL0_SHIFT: c_int = 5;
pub const SCALER_CTL0_SCL_H_PPF_V_PPF: c_int = 0;
pub const SCALER_CTL0_SCL_H_TPZ_V_PPF: c_int = 1;
pub const SCALER_CTL0_SCL_H_PPF_V_TPZ: c_int = 2;
pub const SCALER_CTL0_SCL_H_TPZ_V_TPZ: c_int = 3;
pub const SCALER_CTL0_SCL_H_PPF_V_NONE: c_int = 4;
pub const SCALER_CTL0_SCL_H_NONE_V_PPF: c_int = 5;
pub const SCALER_CTL0_SCL_H_NONE_V_TPZ: c_int = 6;
pub const SCALER_CTL0_SCL_H_TPZ_V_NONE: c_int = 7;
// Set to indicate no scaling.

pub const SCALER_CTL0_PIXEL_FORMAT_SHIFT: c_int = 0;

pub const SCALER_POS0_FIXED_ALPHA_SHIFT: c_int = 24;

pub const SCALER_POS0_START_Y_SHIFT: c_int = 12;

pub const SCALER_POS0_START_X_SHIFT: c_int = 0;

pub const SCALER5_POS0_START_Y_SHIFT: c_int = 16;

pub const SCALER5_POS0_START_X_SHIFT: c_int = 0;

pub const SCALER5_CTL2_ALPHA_MODE_SHIFT: c_int = 30;
pub const SCALER5_CTL2_ALPHA_MODE_PIPELINE: c_int = 0;
pub const SCALER5_CTL2_ALPHA_MODE_FIXED: c_int = 1;
pub const SCALER5_CTL2_ALPHA_MODE_FIXED_NONZERO: c_int = 2;
pub const SCALER5_CTL2_ALPHA_MODE_FIXED_OVER_0x07: c_int = 3;

pub const SCALER5_CTL2_MAP_SEL_SHIFT: c_int = 17;

pub const SCALER5_CTL2_ALPHA_SHIFT: c_int = 4;

pub const SCALER_POS1_SCL_HEIGHT_SHIFT: c_int = 16;

pub const SCALER_POS1_SCL_WIDTH_SHIFT: c_int = 0;

pub const SCALER5_POS1_SCL_HEIGHT_SHIFT: c_int = 16;

pub const SCALER5_POS1_SCL_WIDTH_SHIFT: c_int = 0;

pub const SCALER_POS2_ALPHA_MODE_SHIFT: c_int = 30;
pub const SCALER_POS2_ALPHA_MODE_PIPELINE: c_int = 0;
pub const SCALER_POS2_ALPHA_MODE_FIXED: c_int = 1;
pub const SCALER_POS2_ALPHA_MODE_FIXED_NONZERO: c_int = 2;
pub const SCALER_POS2_ALPHA_MODE_FIXED_OVER_0x07: c_int = 3;

pub const SCALER_POS2_HEIGHT_SHIFT: c_int = 16;

pub const SCALER_POS2_WIDTH_SHIFT: c_int = 0;

pub const SCALER5_POS2_HEIGHT_SHIFT: c_int = 16;

pub const SCALER5_POS2_WIDTH_SHIFT: c_int = 0;
// Color Space Conversion words.  Some values are S2.8 signed
// integers, except that the 2 integer bits map as {0x0: 0, 0x1: 1,
// 0x2: 2, 0x3: -1}
//
// bottom 8 bits of S2.8 contribution of Cr to Blue

pub const SCALER_CSC0_COEF_CR_BLU_SHIFT: c_int = 24;
// Signed offset to apply to Y before CSC. (Y' = Y + YY_OFS)

pub const SCALER_CSC0_COEF_YY_OFS_SHIFT: c_int = 16;
// Signed offset to apply to CB before CSC (Cb' = Cb - 128 + CB_OFS).

pub const SCALER_CSC0_COEF_CB_OFS_SHIFT: c_int = 8;
// Signed offset to apply to CB before CSC (Cr' = Cr - 128 + CR_OFS).

pub const SCALER_CSC0_COEF_CR_OFS_SHIFT: c_int = 0;
pub const SCALER_CSC0_ITR_R_601_5: c_uint = 0x00f00000;
pub const SCALER_CSC0_ITR_R_709_3: c_uint = 0x00f00000;
pub const SCALER_CSC0_ITR_R_2020: c_uint = 0x00f00000;
pub const SCALER_CSC0_JPEG_JFIF: c_uint = 0x00000000;
pub const SCALER_CSC0_ITR_R_709_3_FR: c_uint = 0x00000000;
pub const SCALER_CSC0_ITR_R_2020_FR: c_uint = 0x00000000;
// S2.8 contribution of Cb to Green

pub const SCALER_CSC1_COEF_CB_GRN_SHIFT: c_int = 22;
// S2.8 contribution of Cr to Green

pub const SCALER_CSC1_COEF_CR_GRN_SHIFT: c_int = 12;
// S2.8 contribution of Y to all of RGB

pub const SCALER_CSC1_COEF_YY_ALL_SHIFT: c_int = 2;
// top 2 bits of S2.8 contribution of Cr to Blue

pub const SCALER_CSC1_COEF_CR_BLU_SHIFT: c_int = 0;
pub const SCALER_CSC1_ITR_R_601_5: c_uint = 0xe73304a8;
pub const SCALER_CSC1_ITR_R_709_3: c_uint = 0xf27784a8;
pub const SCALER_CSC1_ITR_R_2020: c_uint = 0xf43594a8;
pub const SCALER_CSC1_JPEG_JFIF: c_uint = 0xea349400;
pub const SCALER_CSC1_ITR_R_709_3_FR: c_uint = 0xf4388400;
pub const SCALER_CSC1_ITR_R_2020_FR: c_uint = 0xf5b6d400;
// S2.8 contribution of Cb to Red

pub const SCALER_CSC2_COEF_CB_RED_SHIFT: c_int = 20;
// S2.8 contribution of Cr to Red

pub const SCALER_CSC2_COEF_CR_RED_SHIFT: c_int = 10;
// S2.8 contribution of Cb to Blue

pub const SCALER_CSC2_COEF_CB_BLU_SHIFT: c_int = 10;
pub const SCALER_CSC2_ITR_R_601_5: c_uint = 0x00066604;
pub const SCALER_CSC2_ITR_R_709_3: c_uint = 0x00072e1d;
pub const SCALER_CSC2_ITR_R_2020: c_uint = 0x0006b624;
pub const SCALER_CSC2_JPEG_JFIF: c_uint = 0x00059dc6;
pub const SCALER_CSC2_ITR_R_709_3_FR: c_uint = 0x00064ddb;
pub const SCALER_CSC2_ITR_R_2020_FR: c_uint = 0x0005e5e2;

pub const SCALER_TPZ0_SCALE_SHIFT: c_int = 8;

pub const SCALER_TPZ0_IPHASE_SHIFT: c_int = 0;

pub const SCALER_TPZ1_RECIP_SHIFT: c_int = 0;
// Skips interpolating coefficients to 64 phases, so just 8 are used.
// Required for nearest neighbor.
//

// Replaes the highest valued coefficient with one that makes all 4
// sum to unity.
//

pub const SCALER_PPF_SCALE_SHIFT: c_int = 8;

pub const SCALER_PPF_IPHASE_SHIFT: c_int = 0;

pub const SCALER_PPF_KERNEL_OFFSET_SHIFT: c_int = 0;

// PITCH0/1/2 fields for raster.

pub const SCALER_SRC_PITCH_SHIFT: c_int = 0;
// PITCH0/1/2 fields for tiled (SAND).

pub const SCALER_TILE_SKIP_0_SHIFT: c_int = 16;

pub const SCALER_TILE_HEIGHT_SHIFT: c_int = 0;
// Common PITCH0 fields

pub const SCALER_PITCH0_SINK_PIX_SHIFT: c_int = 26;
// PITCH0 fields for T-tiled.

pub const SCALER_PITCH0_TILE_WIDTH_L_SHIFT: c_int = 16;

// Y offset within a tile.

pub const SCALER_PITCH0_TILE_Y_OFFSET_SHIFT: c_int = 8;

pub const SCALER_PITCH0_TILE_WIDTH_R_SHIFT: c_int = 0;

pub const SCALER6_CTL0_ADDR_MODE_LINEAR: c_int = 0;
pub const SCALER6_CTL0_ADDR_MODE_128B: c_int = 1;
pub const SCALER6_CTL0_ADDR_MODE_256B: c_int = 2;
pub const SCALER6_CTL0_ADDR_MODE_MAP8: c_int = 3;
pub const SCALER6_CTL0_ADDR_MODE_UIF: c_int = 4;

pub const SCALER6_CTL0_ALPHA_MASK_NONE: c_int = 0;
pub const SCALER6D_CTL0_ALPHA_MASK_FIXED: c_int = 3;

pub const SCALER6_PTR0_UPM_BUFF_SIZE_16_LINES: c_int = 3;
pub const SCALER6_PTR0_UPM_BUFF_SIZE_8_LINES: c_int = 2;
pub const SCALER6_PTR0_UPM_BUFF_SIZE_4_LINES: c_int = 1;
pub const SCALER6_PTR0_UPM_BUFF_SIZE_2_LINES: c_int = 0;

pub const SCALER6_PTR2_ALPHA_BPP_1BPP: c_int = 1;
pub const SCALER6_PTR2_ALPHA_BPP_8BPP: c_int = 0;

pub const SCALER6_PTR2_ALPHA_ORDER_MSB_TO_LSB: c_int = 1;
pub const SCALER6_PTR2_ALPHA_ORDER_LSB_TO_MSB: c_int = 0;

