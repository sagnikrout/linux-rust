//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/i740_reg.h
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
// Authors:
// Kevin E. Martin <kevin@precisioninsight.com>
//
// I/O register offsets

pub const XRX: c_uint = 0x3D6;
pub const MRX: c_uint = 0x3D2;
// VGA Color Palette Registers
pub const DACMASK: c_uint = 0x3C6;
pub const DACSTATE: c_uint = 0x3C7;
pub const DACRX: c_uint = 0x3C7;
pub const DACWX: c_uint = 0x3C8;
pub const DACDATA: c_uint = 0x3C9;
// CRT Controller Registers (CRX)
pub const START_ADDR_HI: c_uint = 0x0C;
pub const START_ADDR_LO: c_uint = 0x0D;
pub const VERT_SYNC_END: c_uint = 0x11;
pub const EXT_VERT_TOTAL: c_uint = 0x30;
pub const EXT_VERT_DISPLAY: c_uint = 0x31;
pub const EXT_VERT_SYNC_START: c_uint = 0x32;
pub const EXT_VERT_BLANK_START: c_uint = 0x33;
pub const EXT_HORIZ_TOTAL: c_uint = 0x35;
pub const EXT_HORIZ_BLANK: c_uint = 0x39;
pub const EXT_START_ADDR: c_uint = 0x40;
pub const EXT_START_ADDR_ENABLE: c_uint = 0x80;
pub const EXT_OFFSET: c_uint = 0x41;
pub const EXT_START_ADDR_HI: c_uint = 0x42;
pub const INTERLACE_CNTL: c_uint = 0x70;
pub const INTERLACE_ENABLE: c_uint = 0x80;
pub const INTERLACE_DISABLE: c_uint = 0x00;
// Miscellaneous Output Register
pub const MSR_R: c_uint = 0x3CC;
pub const MSR_W: c_uint = 0x3C2;
pub const IO_ADDR_SELECT: c_uint = 0x01;
pub const MDA_BASE: c_uint = 0x3B0;
pub const CGA_BASE: c_uint = 0x3D0;
// System Configuration Extension Registers (XRX)
pub const IO_CTNL: c_uint = 0x09;
pub const EXTENDED_ATTR_CNTL: c_uint = 0x02;
pub const EXTENDED_CRTC_CNTL: c_uint = 0x01;
pub const ADDRESS_MAPPING: c_uint = 0x0A;
pub const PACKED_MODE_ENABLE: c_uint = 0x04;
pub const LINEAR_MODE_ENABLE: c_uint = 0x02;
pub const PAGE_MAPPING_ENABLE: c_uint = 0x01;
pub const BITBLT_CNTL: c_uint = 0x20;
pub const COLEXP_MODE: c_uint = 0x30;
pub const COLEXP_8BPP: c_uint = 0x00;
pub const COLEXP_16BPP: c_uint = 0x10;
pub const COLEXP_24BPP: c_uint = 0x20;
pub const COLEXP_RESERVED: c_uint = 0x30;
pub const CHIP_RESET: c_uint = 0x02;
pub const BITBLT_STATUS: c_uint = 0x01;
pub const DISPLAY_CNTL: c_uint = 0x40;
pub const VGA_WRAP_MODE: c_uint = 0x02;
pub const VGA_WRAP_AT_256KB: c_uint = 0x00;
pub const VGA_NO_WRAP: c_uint = 0x02;
pub const GUI_MODE: c_uint = 0x01;
pub const STANDARD_VGA_MODE: c_uint = 0x00;
pub const HIRES_MODE: c_uint = 0x01;
pub const DRAM_ROW_TYPE: c_uint = 0x50;
pub const DRAM_ROW_0: c_uint = 0x07;
pub const DRAM_ROW_0_SDRAM: c_uint = 0x00;
pub const DRAM_ROW_0_EMPTY: c_uint = 0x07;
pub const DRAM_ROW_1: c_uint = 0x38;
pub const DRAM_ROW_1_SDRAM: c_uint = 0x00;
pub const DRAM_ROW_1_EMPTY: c_uint = 0x38;
pub const DRAM_ROW_CNTL_LO: c_uint = 0x51;
pub const DRAM_CAS_LATENCY: c_uint = 0x10;
pub const DRAM_RAS_TIMING: c_uint = 0x08;
pub const DRAM_RAS_PRECHARGE: c_uint = 0x04;
pub const DRAM_ROW_CNTL_HI: c_uint = 0x52;
pub const DRAM_EXT_CNTL: c_uint = 0x53;
pub const DRAM_REFRESH_RATE: c_uint = 0x03;
pub const DRAM_REFRESH_DISABLE: c_uint = 0x00;
pub const DRAM_REFRESH_60HZ: c_uint = 0x01;
pub const DRAM_REFRESH_FAST_TEST: c_uint = 0x02;
pub const DRAM_REFRESH_RESERVED: c_uint = 0x03;
pub const DRAM_TIMING: c_uint = 0x54;
pub const DRAM_ROW_BNDRY_0: c_uint = 0x55;
pub const DRAM_ROW_BNDRY_1: c_uint = 0x56;
pub const DPMS_SYNC_SELECT: c_uint = 0x61;
pub const VSYNC_CNTL: c_uint = 0x08;
pub const VSYNC_ON: c_uint = 0x00;
pub const VSYNC_OFF: c_uint = 0x08;
pub const HSYNC_CNTL: c_uint = 0x02;
pub const HSYNC_ON: c_uint = 0x00;
pub const HSYNC_OFF: c_uint = 0x02;
pub const PIXPIPE_CONFIG_0: c_uint = 0x80;
pub const DAC_8_BIT: c_uint = 0x80;
pub const DAC_6_BIT: c_uint = 0x00;
pub const HW_CURSOR_ENABLE: c_uint = 0x10;
pub const EXTENDED_PALETTE: c_uint = 0x01;
pub const PIXPIPE_CONFIG_1: c_uint = 0x81;
pub const DISPLAY_COLOR_MODE: c_uint = 0x0F;
pub const DISPLAY_VGA_MODE: c_uint = 0x00;
pub const DISPLAY_8BPP_MODE: c_uint = 0x02;
pub const DISPLAY_15BPP_MODE: c_uint = 0x04;
pub const DISPLAY_16BPP_MODE: c_uint = 0x05;
pub const DISPLAY_24BPP_MODE: c_uint = 0x06;
pub const DISPLAY_32BPP_MODE: c_uint = 0x07;
pub const PIXPIPE_CONFIG_2: c_uint = 0x82;
pub const DISPLAY_GAMMA_ENABLE: c_uint = 0x08;
pub const DISPLAY_GAMMA_DISABLE: c_uint = 0x00;
pub const OVERLAY_GAMMA_ENABLE: c_uint = 0x04;
pub const OVERLAY_GAMMA_DISABLE: c_uint = 0x00;
pub const CURSOR_CONTROL: c_uint = 0xA0;
pub const CURSOR_ORIGIN_SCREEN: c_uint = 0x00;
pub const CURSOR_ORIGIN_DISPLAY: c_uint = 0x10;
pub const CURSOR_MODE: c_uint = 0x07;
pub const CURSOR_MODE_DISABLE: c_uint = 0x00;
pub const CURSOR_MODE_32_4C_AX: c_uint = 0x01;
pub const CURSOR_MODE_128_2C: c_uint = 0x02;
pub const CURSOR_MODE_128_1C: c_uint = 0x03;
pub const CURSOR_MODE_64_3C: c_uint = 0x04;
pub const CURSOR_MODE_64_4C_AX: c_uint = 0x05;
pub const CURSOR_MODE_64_4C: c_uint = 0x06;
pub const CURSOR_MODE_RESERVED: c_uint = 0x07;
pub const CURSOR_BASEADDR_LO: c_uint = 0xA2;
pub const CURSOR_BASEADDR_HI: c_uint = 0xA3;
pub const CURSOR_X_LO: c_uint = 0xA4;
pub const CURSOR_X_HI: c_uint = 0xA5;
pub const CURSOR_X_POS: c_uint = 0x00;
pub const CURSOR_X_NEG: c_uint = 0x80;
pub const CURSOR_Y_LO: c_uint = 0xA6;
pub const CURSOR_Y_HI: c_uint = 0xA7;
pub const CURSOR_Y_POS: c_uint = 0x00;
pub const CURSOR_Y_NEG: c_uint = 0x80;
pub const VCLK2_VCO_M: c_uint = 0xC8;
pub const VCLK2_VCO_N: c_uint = 0xC9;
pub const VCLK2_VCO_MN_MSBS: c_uint = 0xCA;
pub const VCO_N_MSBS: c_uint = 0x30;
pub const VCO_M_MSBS: c_uint = 0x03;
pub const VCLK2_VCO_DIV_SEL: c_uint = 0xCB;
pub const POST_DIV_SELECT: c_uint = 0x70;
pub const POST_DIV_1: c_uint = 0x00;
pub const POST_DIV_2: c_uint = 0x10;
pub const POST_DIV_4: c_uint = 0x20;
pub const POST_DIV_8: c_uint = 0x30;
pub const POST_DIV_16: c_uint = 0x40;
pub const POST_DIV_32: c_uint = 0x50;
pub const VCO_LOOP_DIV_BY_4M: c_uint = 0x00;
pub const VCO_LOOP_DIV_BY_16M: c_uint = 0x04;
pub const REF_CLK_DIV_BY_5: c_uint = 0x02;
pub const REF_DIV_4: c_uint = 0x00;
pub const REF_DIV_1: c_uint = 0x01;
pub const PLL_CNTL: c_uint = 0xCE;
pub const PLL_MEMCLK_SEL: c_uint = 0x03;
pub const PLL_MEMCLK__66667KHZ: c_uint = 0x00;
pub const PLL_MEMCLK__75000KHZ: c_uint = 0x01;
pub const PLL_MEMCLK__88889KHZ: c_uint = 0x02;
pub const PLL_MEMCLK_100000KHZ: c_uint = 0x03;
// Multimedia Extension Registers (MRX)
pub const ACQ_CNTL_1: c_uint = 0x02;
pub const ACQ_CNTL_2: c_uint = 0x03;
pub const FRAME_CAP_MODE: c_uint = 0x01;
pub const CONT_CAP_MODE: c_uint = 0x00;
pub const SINGLE_CAP_MODE: c_uint = 0x01;
pub const ACQ_CNTL_3: c_uint = 0x04;
pub const COL_KEY_CNTL_1: c_uint = 0x3C;
pub const BLANK_DISP_OVERLAY: c_uint = 0x20;
// FIFOs
pub const LP_FIFO: c_uint = 0x1000;
pub const HP_FIFO: c_uint = 0x2000;
pub const INSTPNT: c_uint = 0x3040;
pub const LP_FIFO_COUNT: c_uint = 0x3040;
pub const HP_FIFO_COUNT: c_uint = 0x3041;
// FIFO Commands
pub const CLIENT: c_uint = 0xE0000000;
pub const CLIENT_2D: c_uint = 0x60000000;
// Command Parser Mode Register
pub const COMPARS: c_uint = 0x3038;
pub const TWO_D_INST_DISABLE: c_uint = 0x08;
pub const THREE_D_INST_DISABLE: c_uint = 0x04;
pub const STATE_VAR_UPDATE_DISABLE: c_uint = 0x02;
pub const PAL_STIP_DISABLE: c_uint = 0x01;
// Interrupt Control Registers
pub const IER: c_uint = 0x3030;
pub const IIR: c_uint = 0x3032;
pub const IMR: c_uint = 0x3034;
pub const ISR: c_uint = 0x3036;
pub const VMIINTB_EVENT: c_uint = 0x2000;
pub const GPIO4_INT: c_uint = 0x1000;
pub const DISP_FLIP_EVENT: c_uint = 0x0800;
pub const DVD_PORT_DMA: c_uint = 0x0400;
pub const DISP_VBLANK: c_uint = 0x0200;
pub const FIFO_EMPTY_DMA_DONE: c_uint = 0x0100;
pub const INST_PARSER_ERROR: c_uint = 0x0080;
pub const USER_DEFINED: c_uint = 0x0040;
pub const BREAKPOINT: c_uint = 0x0020;
pub const DISP_HORIZ_COUNT: c_uint = 0x0010;
pub const DISP_VSYNC: c_uint = 0x0008;
pub const CAPTURE_HORIZ_COUNT: c_uint = 0x0004;
pub const CAPTURE_VSYNC: c_uint = 0x0002;
pub const THREE_D_PIPE_FLUSHED: c_uint = 0x0001;
// FIFO Watermark and Burst Length Control Register
pub const FWATER_BLC: c_uint = 0x00006000;
pub const LMI_BURST_LENGTH: c_uint = 0x7F000000;
pub const LMI_FIFO_WATERMARK: c_uint = 0x003F0000;
pub const AGP_BURST_LENGTH: c_uint = 0x00007F00;
pub const AGP_FIFO_WATERMARK: c_uint = 0x0000003F;
// BitBLT Registers
pub const SRC_DST_PITCH: c_uint = 0x00040000;
pub const DST_PITCH: c_uint = 0x1FFF0000;
pub const SRC_PITCH: c_uint = 0x00001FFF;
pub const COLEXP_BG_COLOR: c_uint = 0x00040004;
pub const COLEXP_FG_COLOR: c_uint = 0x00040008;
pub const MONO_SRC_CNTL: c_uint = 0x0004000C;
pub const MONO_USE_COLEXP: c_uint = 0x00000000;
pub const MONO_USE_SRCEXP: c_uint = 0x08000000;
pub const MONO_DATA_ALIGN: c_uint = 0x07000000;
pub const MONO_BIT_ALIGN: c_uint = 0x01000000;
pub const MONO_BYTE_ALIGN: c_uint = 0x02000000;
pub const MONO_WORD_ALIGN: c_uint = 0x03000000;
pub const MONO_DWORD_ALIGN: c_uint = 0x04000000;
pub const MONO_QWORD_ALIGN: c_uint = 0x05000000;
pub const MONO_SRC_INIT_DSCRD: c_uint = 0x003F0000;
pub const MONO_SRC_RIGHT_CLIP: c_uint = 0x00003F00;
pub const MONO_SRC_LEFT_CLIP: c_uint = 0x0000003F;
pub const BITBLT_CONTROL: c_uint = 0x00040010;
pub const BLTR_STATUS: c_uint = 0x80000000;
pub const DYN_DEPTH: c_uint = 0x03000000;
pub const DYN_DEPTH_8BPP: c_uint = 0x00000000;
pub const DYN_DEPTH_16BPP: c_uint = 0x01000000;
pub const DYN_DEPTH_24BPP: c_uint = 0x02000000;
pub const DYN_DEPTH_32BPP: c_uint = 0x03000000	/* Unimplemented on the i740 */;
pub const DYN_DEPTH_ENABLE: c_uint = 0x00800000;
pub const PAT_VERT_ALIGN: c_uint = 0x00700000;
pub const SOLID_PAT_SELECT: c_uint = 0x00080000;
pub const PAT_IS_IN_COLOR: c_uint = 0x00000000;
pub const PAT_IS_MONO: c_uint = 0x00040000;
pub const MONO_PAT_TRANSP: c_uint = 0x00020000;
pub const COLOR_TRANSP_ROP: c_uint = 0x00000000;
pub const COLOR_TRANSP_DST: c_uint = 0x00008000;
pub const COLOR_TRANSP_EQ: c_uint = 0x00000000;
pub const COLOR_TRANSP_NOT_EQ: c_uint = 0x00010000;
pub const COLOR_TRANSP_ENABLE: c_uint = 0x00004000;
pub const MONO_SRC_TRANSP: c_uint = 0x00002000;
pub const SRC_IS_IN_COLOR: c_uint = 0x00000000;
pub const SRC_IS_MONO: c_uint = 0x00001000;
pub const SRC_USE_SRC_ADDR: c_uint = 0x00000000;
pub const SRC_USE_BLTDATA: c_uint = 0x00000400;
pub const BLT_TOP_TO_BOT: c_uint = 0x00000000;
pub const BLT_BOT_TO_TOP: c_uint = 0x00000200;
pub const BLT_LEFT_TO_RIGHT: c_uint = 0x00000000;
pub const BLT_RIGHT_TO_LEFT: c_uint = 0x00000100;
pub const BLT_ROP: c_uint = 0x000000FF;
pub const BLT_PAT_ADDR: c_uint = 0x00040014;
pub const BLT_SRC_ADDR: c_uint = 0x00040018;
pub const BLT_DST_ADDR: c_uint = 0x0004001C;
pub const BLT_DST_H_W: c_uint = 0x00040020;
pub const BLT_DST_HEIGHT: c_uint = 0x1FFF0000;
pub const BLT_DST_WIDTH: c_uint = 0x00001FFF;
pub const SRCEXP_BG_COLOR: c_uint = 0x00040024;
pub const SRCEXP_FG_COLOR: c_uint = 0x00040028;
pub const BLTDATA: c_uint = 0x00050000;
