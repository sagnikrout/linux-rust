//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/mmp/hw/mmp_ctrl.h
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
// drivers/video/mmp/hw/mmp_ctrl.h
//
// Copyright (C) 2012 Marvell Technology Group Ltd.
// Authors:  Guoqing Li <ligq@marvell.com>
// Lisa Du <cldu@marvell.com>
// Zhou Zhu <zzhu3@marvell.com>
//

// ------------< LCD register >------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_regs {
// TV patch register for MMP2
// 32 bit		TV Video Frame0 Y Starting Address

// 32 bit		TV Video Frame0 U Starting Address

// 32 bit		TV Video Frame0 V Starting Address

// 32 bit		TV Video Frame0 Command Starting Address

// 32 bit		TV Video Frame1 Y Starting Address Register

// 32 bit		TV Video Frame1 U Starting Address Register

// 32 bit		TV Video Frame1 V Starting Address Register

// 32 bit		TV Video Frame1 Command Starting Address Register

// 32 bit		TV Video Y andC Line Length(Pitch)Register

// 32 bit		TV Video U andV Line Length(Pitch)Register

// 32 bit	  TV Video Starting Point on Screen Register

// 32 bit		TV Video Source Size Register

// 32 bit	  TV Video Destination Size (After Zooming)Register

    pub v_y0: u32,
    pub v_u0: u32,
    pub v_v0: u32,
    pub v_c0: u32,
    pub v_y1: u32,
    pub v_u1: u32,
    pub v_v1: u32,
    pub v_c1: u32,
    pub /: *mut *mut u32 v_pitch_yc; / Video Y and C Line Length (Pitch),
    pub /: *mut *mut u32 v_pitch_uv; / Video U and V Line Length (Pitch),
    pub /: *mut *mut u32 v_start; / Video Starting Point on Screen,
    pub /: *mut *mut u32 v_size; / Video Source Size,
    pub /: *mut *mut u32 v_size_z; / Video Destination Size (After Zooming),
// 32 bit	   TV Graphic Frame 0 Starting Address Register

// 32 bit	  TV Graphic Frame 1 Starting Address Register

// 32 bit		TV Graphic Line Length(Pitch)Register

// 32 bit		TV Graphic Starting Point on Screen Register

// 32 bit		TV Graphic Source Size Register

// 32 bit		TV Graphic Destination size (after Zooming)Register

    pub /: *mut *mut u32 g_0; / Graphic Frame 0/1 Starting Address,
    pub g_1: u32,
    pub /: *mut *mut u32 g_pitch; / Graphic Line Length (Pitch),
    pub /: *mut *mut u32 g_start; / Graphic Starting Point on Screen,
    pub /: *mut *mut u32 g_size; / Graphic Source Size,
    pub /: *mut *mut u32 g_size_z; / Graphic Destination Size (After Zooming),
// 32 bit	  TV Hardware Cursor Starting Point on screen Register

// 32 bit		TV Hardware Cursor Size Register

    pub /: *mut *mut u32 hc_start; / Hardware Cursor,
    pub /: *mut *mut u32 hc_size; / Hardware Cursor,
// 32 bit		TV Total Screen Size Register

// 32 bit		TV Screen Active Size Register

// 32 bit		TV Screen Horizontal Porch Register

// 32 bit		TV Screen Vertical Porch Register

    pub /: *mut *mut u32 screen_size; / Screen Total Size,
    pub /: *mut *mut u32 screen_active; / Screen Active Size,
    pub /: *mut *mut u32 screen_h_porch; / Screen Horizontal Porch,
    pub /: *mut *mut u32 screen_v_porch; / Screen Vertical Porch,
// 32 bit		TV Screen Blank Color Register

// 32 bit		TV Hardware Cursor Color1 Register

// 32 bit		TV Hardware Cursor Color2 Register

    pub /: *mut *mut u32 blank_color; / Screen Blank Color,
    pub /: *mut *mut u32 hc_Alpha_color1; / Hardware Cursor Color1,
    pub /: *mut *mut u32 hc_Alpha_color2; / Hardware Cursor Color2,
// 32 bit		TV Video Y Color Key Control

// 32 bit		TV Video U Color Key Control

// 32 bit		TV Video V Color Key Control

    pub /: *mut *mut u32 v_colorkey_y; / Video Y Color Key Control,
    pub /: *mut *mut u32 v_colorkey_u; / Video U Color Key Control,
    pub /: *mut *mut u32 v_colorkey_v; / Video V Color Key Control,
// 32 bit		TV VSYNC PulsePixel Edge Control Register

    pub /: *mut *mut u32 vsync_ctrl; / VSYNC PulsePixel Edge Control,
}

// 32 bit		TV Path DMA Control 0

// 32 bit		TV Path DMA Control 1

// 32 bit		TV Path Video Contrast

// 32 bit		TV Path Video Saturation

// 32 bit		TV Path Video Hue Adjust

// 32 bit TV Path TVIF Control	Register

// 32 bit TV Path I/O Pad Control

// 32 bit TV Path Cloc	Divider

// dither configure

// dither table data was fixed by video bpp of input and output

// Video Frame 0&1 start address registers
pub const LCD_SPU_DMA_START_ADDR_Y0: c_uint = 0x00C0;
pub const LCD_SPU_DMA_START_ADDR_U0: c_uint = 0x00C4;
pub const LCD_SPU_DMA_START_ADDR_V0: c_uint = 0x00C8;
pub const LCD_CFG_DMA_START_ADDR_0: c_uint = 0x00CC /* Cmd address */;
pub const LCD_SPU_DMA_START_ADDR_Y1: c_uint = 0x00D0;
pub const LCD_SPU_DMA_START_ADDR_U1: c_uint = 0x00D4;
pub const LCD_SPU_DMA_START_ADDR_V1: c_uint = 0x00D8;
pub const LCD_CFG_DMA_START_ADDR_1: c_uint = 0x00DC /* Cmd address */;
// YC & UV Pitch
pub const LCD_SPU_DMA_PITCH_YC: c_uint = 0x00E0;

pub const LCD_SPU_DMA_PITCH_UV: c_uint = 0x00E4;

// Video Starting Point on Screen Register
pub const LCD_SPUT_DMA_OVSA_HPXL_VLN: c_uint = 0x00E8;

// Video Size Register
pub const LCD_SPU_DMA_HPXL_VLN: c_uint = 0x00EC;

// Video Size After zooming Register
pub const LCD_SPU_DZM_HPXL_VLN: c_uint = 0x00F0;

// Graphic Frame 0&1 Starting Address Register
pub const LCD_CFG_GRA_START_ADDR0: c_uint = 0x00F4;
pub const LCD_CFG_GRA_START_ADDR1: c_uint = 0x00F8;
// Graphic Frame Pitch
pub const LCD_CFG_GRA_PITCH: c_uint = 0x00FC;
// Graphic Starting Point on Screen Register
pub const LCD_SPU_GRA_OVSA_HPXL_VLN: c_uint = 0x0100;

// Graphic Size Register
pub const LCD_SPU_GRA_HPXL_VLN: c_uint = 0x0104;

// Graphic Size after Zooming Register
pub const LCD_SPU_GZM_HPXL_VLN: c_uint = 0x0108;

// HW Cursor Starting Point on Screen Register
pub const LCD_SPU_HWC_OVSA_HPXL_VLN: c_uint = 0x010C;

// HW Cursor Size
pub const LCD_SPU_HWC_HPXL_VLN: c_uint = 0x0110;

// Total Screen Size Register
pub const LCD_SPUT_V_H_TOTAL: c_uint = 0x0114;

// Total Screen Active Size Register
pub const LCD_SPU_V_H_ACTIVE: c_uint = 0x0118;

// Screen H&V Porch Register
pub const LCD_SPU_H_PORCH: c_uint = 0x011C;

pub const LCD_SPU_V_PORCH: c_uint = 0x0120;

// Screen Blank Color Register
pub const LCD_SPU_BLANKCOLOR: c_uint = 0x0124;
pub const CFG_BLANKCOLOR_MASK: c_uint = 0x00FFFFFF;
pub const CFG_BLANKCOLOR_R_MASK: c_uint = 0x000000FF;
pub const CFG_BLANKCOLOR_G_MASK: c_uint = 0x0000FF00;
pub const CFG_BLANKCOLOR_B_MASK: c_uint = 0x00FF0000;
// HW Cursor Color 1&2 Register
pub const LCD_SPU_ALPHA_COLOR1: c_uint = 0x0128;
pub const CFG_HWC_COLOR1: c_uint = 0x00FFFFFF;

pub const CFG_HWC_COLOR1_R_MASK: c_uint = 0x000000FF;
pub const CFG_HWC_COLOR1_G_MASK: c_uint = 0x0000FF00;
pub const CFG_HWC_COLOR1_B_MASK: c_uint = 0x00FF0000;
pub const LCD_SPU_ALPHA_COLOR2: c_uint = 0x012C;
pub const CFG_HWC_COLOR2: c_uint = 0x00FFFFFF;
pub const CFG_HWC_COLOR2_R_MASK: c_uint = 0x000000FF;
pub const CFG_HWC_COLOR2_G_MASK: c_uint = 0x0000FF00;
pub const CFG_HWC_COLOR2_B_MASK: c_uint = 0x00FF0000;
// Video YUV Color Key Control
pub const LCD_SPU_COLORKEY_Y: c_uint = 0x0130;

pub const CFG_CKEY_Y2_MASK: c_uint = 0xFF000000;

pub const CFG_CKEY_Y1_MASK: c_uint = 0x00FF0000;

pub const CFG_CKEY_Y_MASK: c_uint = 0x0000FF00;

pub const CFG_ALPHA_Y_MASK: c_uint = 0x000000FF;
pub const LCD_SPU_COLORKEY_U: c_uint = 0x0134;

pub const CFG_CKEY_U2_MASK: c_uint = 0xFF000000;

pub const CFG_CKEY_U1_MASK: c_uint = 0x00FF0000;

pub const CFG_CKEY_U_MASK: c_uint = 0x0000FF00;

pub const CFG_ALPHA_U_MASK: c_uint = 0x000000FF;
pub const LCD_SPU_COLORKEY_V: c_uint = 0x0138;

pub const CFG_CKEY_V2_MASK: c_uint = 0xFF000000;

pub const CFG_CKEY_V1_MASK: c_uint = 0x00FF0000;

pub const CFG_CKEY_V_MASK: c_uint = 0x0000FF00;

pub const CFG_ALPHA_V_MASK: c_uint = 0x000000FF;
// Graphics/Video DMA color key enable bits in LCD_TV_CTRL1
pub const CFG_CKEY_GRA: c_uint = 0x2;
pub const CFG_CKEY_DMA: c_uint = 0x1;
// Interlace mode enable bits in LCD_TV_CTRL1

pub const LCD_PN_SEPXLCNT: c_uint = 0x013c /* MMP2 */;
// SPI Read Data Register
pub const LCD_SPU_SPI_RXDATA: c_uint = 0x0140;
// Smart Panel Read Data Register
pub const LCD_SPU_ISA_RSDATA: c_uint = 0x0144;
pub const ISA_RXDATA_16BIT_1_DATA_MASK: c_uint = 0x000000FF;
pub const ISA_RXDATA_16BIT_2_DATA_MASK: c_uint = 0x0000FF00;
pub const ISA_RXDATA_16BIT_3_DATA_MASK: c_uint = 0x00FF0000;
pub const ISA_RXDATA_16BIT_4_DATA_MASK: c_uint = 0xFF000000;
pub const ISA_RXDATA_32BIT_1_DATA_MASK: c_uint = 0x00FFFFFF;

// HWC SRAM Read Data Register
pub const LCD_SPU_HWC_RDDAT: c_uint = 0x0158;
// Gamma Table SRAM Read Data Register
pub const LCD_SPU_GAMMA_RDDAT: c_uint = 0x015c;
pub const CFG_GAMMA_RDDAT_MASK: c_uint = 0x000000FF;
// Palette Table SRAM Read Data Register
pub const LCD_SPU_PALETTE_RDDAT: c_uint = 0x0160;
pub const CFG_PALETTE_RDDAT_MASK: c_uint = 0x00FFFFFF;

// I/O Pads Input Read Only Register
pub const LCD_SPU_IOPAD_IN: c_uint = 0x0178;
pub const CFG_IOPAD_IN_MASK: c_uint = 0x0FFFFFFF;

// Reserved Read Only Registers
pub const LCD_CFG_RDREG5F: c_uint = 0x017C;
pub const IRE_FRAME_CNT_MASK: c_uint = 0x000000C0;
pub const IPE_FRAME_CNT_MASK: c_uint = 0x00000030;
pub const GRA_FRAME_CNT_MASK: c_uint = 0x0000000C /* Graphic */;
pub const DMA_FRAME_CNT_MASK: c_uint = 0x00000003 /* Video */;

// SPI Control Register.
pub const LCD_SPU_SPI_CTRL: c_uint = 0x0180;

pub const CFG_SCLKCNT_MASK: c_uint = 0xFF000000;

pub const CFG_RXBITS_MASK: c_uint = 0x00FF0000;

pub const CFG_TXBITS_MASK: c_uint = 0x0000FF00;

pub const CFG_CLKINV_MASK: c_uint = 0x00000080;

pub const CFG_KEEPXFER_MASK: c_uint = 0x00000040;

pub const CFG_RXBITSTO0_MASK: c_uint = 0x00000020;

pub const CFG_TXBITSTO0_MASK: c_uint = 0x00000010;

pub const CFG_SPI_ENA_MASK: c_uint = 0x00000008;

pub const CFG_SPI_SEL_MASK: c_uint = 0x00000004;

pub const CFG_SPI_3W4WB_MASK: c_uint = 0x00000002;

pub const CFG_SPI_START_MASK: c_uint = 0x00000001;
// SPI Tx Data Register
pub const LCD_SPU_SPI_TXDATA: c_uint = 0x0184;
//
pub const LCD_SPU_SMPN_CTRL: c_uint = 0x0188;
// DMA Control 0 Register
pub const LCD_SPU_DMA_CTRL0: c_uint = 0x0190;

pub const CFG_NOBLENDING_MASK: c_uint = 0x80000000;

pub const CFG_GAMMA_ENA_MASK: c_uint = 0x40000000;

pub const CFG_CBSH_ENA_MASK: c_uint = 0x20000000;

pub const CFG_PALETTE_ENA_MASK: c_uint = 0x10000000;

pub const CFG_ARBFAST_ENA_MASK: c_uint = 0x08000000;

pub const CFG_HWC_1BITMOD_MASK: c_uint = 0x04000000;

pub const CFG_HWC_1BITENA_MASK: c_uint = 0x02000000;

pub const CFG_HWC_ENA_MASK: c_uint = 0x01000000;

pub const CFG_DMAFORMAT_MASK: c_uint = 0x00F00000;

pub const CFG_GRAFORMAT_MASK: c_uint = 0x000F0000;
// for graphic part

pub const CFG_GRA_FTOGGLE_MASK: c_uint = 0x00008000;

pub const CFG_GRA_HSMOOTH_MASK: c_uint = 0x00004000;

pub const CFG_GRA_TSTMODE_MASK: c_uint = 0x00002000;

pub const CFG_GRA_SWAPRB_MASK: c_uint = 0x00001000;

pub const CFG_GRA_SWAPUV_MASK: c_uint = 0x00000800;

pub const CFG_GRA_SWAPYU_MASK: c_uint = 0x00000400;
pub const CFG_GRA_SWAP_MASK: c_uint = 0x00001C00;

pub const CFG_YUV2RGB_GRA_MASK: c_uint = 0x00000200;

pub const CFG_GRA_ENA_MASK: c_uint = 0x00000100;

// for video part

pub const CFG_DMA_FTOGGLE_MASK: c_uint = 0x00000080;

pub const CFG_DMA_HSMOOTH_MASK: c_uint = 0x00000040;

pub const CFG_DMA_TSTMODE_MASK: c_uint = 0x00000020;

pub const CFG_DMA_SWAPRB_MASK: c_uint = 0x00000010;

pub const CFG_DMA_SWAPUV_MASK: c_uint = 0x00000008;

pub const CFG_DMA_SWAPYU_MASK: c_uint = 0x00000004;
pub const CFG_DMA_SWAP_MASK: c_uint = 0x0000001C;

pub const CFG_YUV2RGB_DMA_MASK: c_uint = 0x00000002;

pub const CFG_DMA_ENA_MASK: c_uint = 0x00000001;

// DMA Control 1 Register
pub const LCD_SPU_DMA_CTRL1: c_uint = 0x0194;

pub const CFG_FRAME_TRIG_MASK: c_uint = 0x80000000;

pub const CFG_VSYNC_TRIG_MASK: c_uint = 0x70000000;

pub const CFG_VSYNC_INV_MASK: c_uint = 0x08000000;

pub const CFG_COLOR_KEY_MASK: c_uint = 0x07000000;

pub const CFG_CARRY_MASK: c_uint = 0x00800000;

pub const CFG_LNBUF_ENA_MASK: c_uint = 0x00400000;

pub const CFG_GATED_ENA_MASK: c_uint = 0x00200000;

pub const CFG_PWRDN_ENA_MASK: c_uint = 0x00100000;

pub const CFG_DSCALE_MASK: c_uint = 0x000C0000;

pub const CFG_ALPHA_MODE_MASK: c_uint = 0x00030000;

pub const CFG_ALPHA_MASK: c_uint = 0x0000FF00;

pub const CFG_PXLCMD_MASK: c_uint = 0x000000FF;
// SRAM Control Register
pub const LCD_SPU_SRAM_CTRL: c_uint = 0x0198;

pub const CFG_SRAM_INIT_WR_RD_MASK: c_uint = 0x0000C000;

pub const CFG_SRAM_ADDR_LCDID_MASK: c_uint = 0x00000F00;

pub const CFG_SRAM_ADDR_MASK: c_uint = 0x000000FF;
// SRAM Write Data Register
pub const LCD_SPU_SRAM_WRDAT: c_uint = 0x019C;
// SRAM RTC/WTC Control Register
pub const LCD_SPU_SRAM_PARA0: c_uint = 0x01A0;
// SRAM Power Down Control Register
pub const LCD_SPU_SRAM_PARA1: c_uint = 0x01A4;

pub const CFG_CSB_256x32_MASK: c_uint = 0x00008000;

pub const CFG_CSB_256x24_MASK: c_uint = 0x00004000;

pub const CFG_CSB_256x8_MASK: c_uint = 0x00002000;

pub const CFG_PDWN256x32_MASK: c_uint = 0x00000080;

pub const CFG_PDWN256x24_MASK: c_uint = 0x00000040;

pub const CFG_PDWN256x8_MASK: c_uint = 0x00000020;

pub const CFG_PDWN32x32_MASK: c_uint = 0x00000008;

pub const CFG_PDWN16x66_MASK: c_uint = 0x00000004;

pub const CFG_PDWN32x66_MASK: c_uint = 0x00000002;

pub const CFG_PDWN64x66_MASK: c_uint = 0x00000001;
// Smart or Dumb Panel Clock Divider
pub const LCD_CFG_SCLK_DIV: c_uint = 0x01A8;

pub const SCLK_SRC_SEL_MASK: c_uint = 0x80000000;

pub const CLK_FRACDIV_MASK: c_uint = 0x0FFF0000;

pub const DSI1_BITCLK_DIV_MASK: c_uint = 0x00000F00;

pub const CLK_INT_DIV_MASK: c_uint = 0x000000FF;
// Video Contrast Register
pub const LCD_SPU_CONTRAST: c_uint = 0x01AC;

pub const CFG_BRIGHTNESS_MASK: c_uint = 0xFFFF0000;

pub const CFG_CONTRAST_MASK: c_uint = 0x0000FFFF;
// Video Saturation Register
pub const LCD_SPU_SATURATION: c_uint = 0x01B0;

pub const CFG_C_MULTS_MASK: c_uint = 0xFFFF0000;

pub const CFG_SATURATION_MASK: c_uint = 0x0000FFFF;
// Video Hue Adjust Register
pub const LCD_SPU_CBSH_HUE: c_uint = 0x01B4;

pub const CFG_SIN0_MASK: c_uint = 0xFFFF0000;

pub const CFG_COS0_MASK: c_uint = 0x0000FFFF;
// Dump LCD Panel Control Register
pub const LCD_SPU_DUMB_CTRL: c_uint = 0x01B8;

pub const CFG_DUMBMODE_MASK: c_uint = 0xF0000000;

pub const CFG_INTFRBSWAP_MASK: c_uint = 0x0F000000;

pub const CFG_LCDGPIO_O_MASK: c_uint = 0x0FF00000;

pub const CFG_LCDGPIO_ENA_MASK: c_uint = 0x000FF000;

pub const CFG_BIAS_OUT_MASK: c_uint = 0x00000100;

pub const CFG_REVERSE_RGB_MASK: c_uint = 0x00000080;

pub const CFG_INV_COMPBLANK_MASK: c_uint = 0x00000040;

pub const CFG_INV_COMPSYNC_MASK: c_uint = 0x00000020;

pub const CFG_INV_HENA_MASK: c_uint = 0x00000010;

pub const CFG_INV_VSYNC_MASK: c_uint = 0x00000008;

pub const CFG_INV_HSYNC_MASK: c_uint = 0x00000004;

pub const CFG_INV_PCLK_MASK: c_uint = 0x00000002;

pub const CFG_DUMB_ENA_MASK: c_uint = 0x00000001;
// LCD I/O Pads Control Register
pub const SPU_IOPAD_CONTROL: c_uint = 0x01BC;

pub const CFG_GRA_VM_ENA_MASK: c_uint = 0x00008000;

pub const CFG_DMA_VM_ENA_MASK: c_uint = 0x00002000;

pub const CFG_CMD_VM_ENA_MASK: c_uint = 0x00001000;

pub const CFG_CSC_MASK: c_uint = 0x00000300;

pub const CFG_BOUNDARY_MASK: c_uint = 0x00000020;

pub const CFG_BURST_MASK: c_uint = 0x00000010;

pub const CFG_IOPADMODE_MASK: c_uint = 0x0000000F;
// LCD Interrupt Control Register
pub const SPU_IRQ_ENA: c_uint = 0x01C0;

pub const DMA_FRAME_IRQ0_ENA_MASK: c_uint = 0x80000000;

pub const DMA_FRAME_IRQ1_ENA_MASK: c_uint = 0x40000000;

pub const DMA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x20000000;

pub const AXI_BUS_ERROR_IRQ_ENA_MASK: c_uint = 0x10000000;

pub const GRA_FRAME_IRQ0_ENA_MASK: c_uint = 0x08000000;

pub const GRA_FRAME_IRQ1_ENA_MASK: c_uint = 0x04000000;

pub const GRA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x02000000;

pub const VSYNC_IRQ_ENA_MASK: c_uint = 0x00800000;

pub const DUMB_FRAMEDONE_ENA_MASK: c_uint = 0x00400000;

pub const TWC_FRAMEDONE_ENA_MASK: c_uint = 0x00200000;

pub const HWC_FRAMEDONE_ENA_MASK: c_uint = 0x00100000;

pub const SLV_IRQ_ENA_MASK: c_uint = 0x00080000;

pub const SPI_IRQ_ENA_MASK: c_uint = 0x00040000;

pub const PWRDN_IRQ_ENA_MASK: c_uint = 0x00020000;

pub const AXI_LATENCY_TOO_LONG_IRQ_ENA_MASK: c_uint = 0x00010000;

pub const CLEAN_SPU_IRQ_ISR_MASK: c_uint = 0x0000FFFF;

pub const TV_DMA_FRAME_IRQ0_ENA_MASK: c_uint = 0x00008000;

pub const TV_DMA_FRAME_IRQ1_ENA_MASK: c_uint = 0x00004000;

pub const TV_DMA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x00002000;

pub const TVSYNC_IRQ_ENA_MASK: c_uint = 0x00001000;

pub const TV_FRAME_IRQ0_ENA_MASK: c_uint = 0x00000800;

pub const TV_FRAME_IRQ1_ENA_MASK: c_uint = 0x00000400;

pub const TV_GRA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x00000200;

pub const TV_FRAMEDONE_ENA_MASK: c_uint = 0x00000100;
// FIXME - JUST GUESS

pub const PN2_DMA_FRAME_IRQ0_ENA_MASK: c_uint = 0x00000080;

pub const PN2_DMA_FRAME_IRQ1_ENA_MASK: c_uint = 0x00000040;

pub const PN2_DMA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x00000020;

pub const PN2_GRA_FRAME_IRQ0_ENA_MASK: c_uint = 0x00000008;

pub const PN2_GRA_FRAME_IRQ1_ENA_MASK: c_uint = 0x04000004;

pub const PN2_GRA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x00000002;

pub const PN2_SYNC_IRQ_ENA_MASK: c_uint = 0x00000001;

// error indications

// LCD Interrupt Status Register
pub const SPU_IRQ_ISR: c_uint = 0x01C4;

pub const DMA_FRAME_IRQ0_MASK: c_uint = 0x80000000;

pub const DMA_FRAME_IRQ1_MASK: c_uint = 0x40000000;

pub const DMA_FF_UNDERFLOW_MASK: c_uint = 0x20000000;

pub const AXI_BUS_ERROR_IRQ_MASK: c_uint = 0x10000000;

pub const GRA_FRAME_IRQ0_MASK: c_uint = 0x08000000;

pub const GRA_FRAME_IRQ1_MASK: c_uint = 0x04000000;

pub const GRA_FF_UNDERFLOW_MASK: c_uint = 0x02000000;

pub const VSYNC_IRQ_MASK: c_uint = 0x00800000;

pub const DUMB_FRAMEDONE_MASK: c_uint = 0x00400000;

pub const TWC_FRAMEDONE_MASK: c_uint = 0x00200000;

pub const HWC_FRAMEDONE_MASK: c_uint = 0x00100000;

pub const SLV_IRQ_MASK: c_uint = 0x00080000;

pub const SPI_IRQ_MASK: c_uint = 0x00040000;

pub const PWRDN_IRQ_MASK: c_uint = 0x00020000;

pub const AXI_LATENCY_TOO_LONGR_IRQ_MASK: c_uint = 0x00010000;

pub const TV_DMA_FRAME_IRQ0_MASK: c_uint = 0x00008000;

pub const TV_DMA_FRAME_IRQ1_MASK: c_uint = 0x00004000;

pub const TV_DMA_FF_UNDERFLOW_MASK: c_uint = 0x00002000;

pub const TVSYNC_IRQ_MASK: c_uint = 0x00001000;

pub const TV_FRAME_IRQ0_MASK: c_uint = 0x00000800;

pub const TV_FRAME_IRQ1_MASK: c_uint = 0x00000400;

pub const TV_GRA_FF_UNDERFLOW_MASK: c_uint = 0x00000200;

pub const PN2_DMA_FRAME_IRQ0_MASK: c_uint = 0x00000080;

pub const PN2_DMA_FRAME_IRQ1_MASK: c_uint = 0x00000040;

pub const PN2_DMA_FF_UNDERFLOW_MASK: c_uint = 0x00000020;

pub const PN2_GRA_FRAME_IRQ0_MASK: c_uint = 0x00000008;

pub const PN2_GRA_FRAME_IRQ1_MASK: c_uint = 0x04000004;

pub const PN2_GRA_FF_UNDERFLOW_MASK: c_uint = 0x00000002;

pub const PN2_SYNC_IRQ_MASK: c_uint = 0x00000001;
// LCD FIFO Depth register
pub const LCD_FIFO_DEPTH: c_uint = 0x01c8;

pub const VIDEO_FIFO_MASK: c_uint = 0x00000003;

pub const GRAPHIC_FIFO_MASK: c_uint = 0x0000000c;
// read-only
pub const DMA_FRAME_IRQ0_LEVEL_MASK: c_uint = 0x00008000;
pub const DMA_FRAME_IRQ1_LEVEL_MASK: c_uint = 0x00004000;
pub const DMA_FRAME_CNT_ISR_MASK: c_uint = 0x00003000;
pub const GRA_FRAME_IRQ0_LEVEL_MASK: c_uint = 0x00000800;
pub const GRA_FRAME_IRQ1_LEVEL_MASK: c_uint = 0x00000400;
pub const GRA_FRAME_CNT_ISR_MASK: c_uint = 0x00000300;
pub const VSYNC_IRQ_LEVEL_MASK: c_uint = 0x00000080;
pub const DUMB_FRAMEDONE_LEVEL_MASK: c_uint = 0x00000040;
pub const TWC_FRAMEDONE_LEVEL_MASK: c_uint = 0x00000020;
pub const HWC_FRAMEDONE_LEVEL_MASK: c_uint = 0x00000010;
pub const SLV_FF_EMPTY_MASK: c_uint = 0x00000008;
pub const DMA_FF_ALLEMPTY_MASK: c_uint = 0x00000004;
pub const GRA_FF_ALLEMPTY_MASK: c_uint = 0x00000002;
pub const PWRDN_IRQ_LEVEL_MASK: c_uint = 0x00000001;
// 32 bit LCD Interrupt Reset Status

// 32 bit Panel Path Graphic Partial Display Horizontal Control Register

// 32 bit Panel Path Graphic Partial Display Vertical Control Register

// 32 bit TV Path Graphic Partial Display	  Horizontal Control Register

// 32 bit TV Path Graphic Partial Display Vertical Control Register

// 32 bit LCD Global Control Register

// 32 bit LCD SQU Line Buffer Control Register 1

// 32 bit LCD SQU Line Buffer Control Register 2

// 32 bit LCD Mixed Overlay Control Register

//
// defined for Configure Dumb Mode
// DUMB LCD Panel bit[31:28]
//
pub const DUMB16_RGB565_0: c_uint = 0x0;
pub const DUMB16_RGB565_1: c_uint = 0x1;
pub const DUMB18_RGB666_0: c_uint = 0x2;
pub const DUMB18_RGB666_1: c_uint = 0x3;
pub const DUMB12_RGB444_0: c_uint = 0x4;
pub const DUMB12_RGB444_1: c_uint = 0x5;
pub const DUMB24_RGB888_0: c_uint = 0x6;
pub const DUMB_BLANK: c_uint = 0x7;
//
// defined for Configure I/O Pin Allocation Mode
// LCD LCD I/O Pads control register bit[3:0]
//
pub const IOPAD_DUMB24: c_uint = 0x0;
pub const IOPAD_DUMB18SPI: c_uint = 0x1;
pub const IOPAD_DUMB18GPIO: c_uint = 0x2;
pub const IOPAD_DUMB16SPI: c_uint = 0x3;
pub const IOPAD_DUMB16GPIO: c_uint = 0x4;
pub const IOPAD_DUMB12: c_uint = 0x5;
pub const IOPAD_SMART18SPI: c_uint = 0x6;
pub const IOPAD_SMART16SPI: c_uint = 0x7;
pub const IOPAD_SMART8BOTH: c_uint = 0x8;
pub const IOPAD_DUMB18_SMART8: c_uint = 0x9;
pub const IOPAD_DUMB16_SMART8SPI: c_uint = 0xa;
pub const IOPAD_DUMB16_SMART8GPIO: c_uint = 0xb;
pub const IOPAD_DUMB16_DUMB16: c_uint = 0xc;
pub const IOPAD_SMART8_SMART8: c_uint = 0xc;
//
// defined for indicating boundary and cycle burst length
//

// SRAM ID
pub const SRAMID_GAMMA_YR: c_uint = 0x0;
pub const SRAMID_GAMMA_UG: c_uint = 0x1;
pub const SRAMID_GAMMA_VB: c_uint = 0x2;
pub const SRAMID_PALATTE: c_uint = 0x3;
pub const SRAMID_HWC: c_uint = 0xf;
// SRAM INIT Read/Write
pub const SRAMID_INIT_READ: c_uint = 0x0;
pub const SRAMID_INIT_WRITE: c_uint = 0x2;
pub const SRAMID_INIT_DEFAULT: c_uint = 0x3;
//
// defined VSYNC selection mode for DMA control 1 register
// DMA1 bit[30:28]
//
pub const VMODE_SMPN: c_uint = 0x0;
pub const VMODE_SMPNIRQ: c_uint = 0x1;
pub const VMODE_DUMB: c_uint = 0x2;
pub const VMODE_IPE: c_uint = 0x3;
pub const VMODE_IRE: c_uint = 0x4;
//
// defined Configure Alpha and Alpha mode for DMA control 1 register
// DMA1 bit[15:08](alpha) / bit[17:16](alpha mode)
//
// ALPHA mode
pub const MODE_ALPHA_DMA: c_uint = 0x0;
pub const MODE_ALPHA_GRA: c_uint = 0x1;
pub const MODE_ALPHA_CFG: c_uint = 0x2;
// alpha value
pub const ALPHA_NOGRAPHIC: c_uint = 0xFF	  /* all video, no graphic */;
pub const ALPHA_NOVIDEO: c_uint = 0x00	  /* all graphic, no video */;
pub const ALPHA_GRAPHNVIDEO: c_uint = 0x0F	  /* Selects graphic & video */;
//
// defined Pixel Command for DMA control 1 register
// DMA1 bit[07:00]
//
pub const PIXEL_CMD: c_uint = 0x81;
// DSI
// DSI1 - 4 Lane Controller base
pub const DSI1_REGS_PHYSICAL_BASE: c_uint = 0xD420B800;
// DSI2 - 3 Lane Controller base
pub const DSI2_REGS_PHYSICAL_BASE: c_uint = 0xD420BA00;
// DSI Controller Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_lcd_regs {
pub const DSI_LCD1_CTRL_0: c_uint = 0x100   /* DSI Active Panel 1 Control register 0 */;
pub const DSI_LCD1_CTRL_1: c_uint = 0x104   /* DSI Active Panel 1 Control register 1 */;
    pub ctrl0: u32,
    pub ctrl1: u32,
    pub reserved1: [u32; 2],
pub const DSI_LCD1_TIMING_0: c_uint = 0x110   /* Timing register 0 */;
pub const DSI_LCD1_TIMING_1: c_uint = 0x114   /* Timing register 1 */;
pub const DSI_LCD1_TIMING_2: c_uint = 0x118   /* Timing register 2 */;
pub const DSI_LCD1_TIMING_3: c_uint = 0x11C   /* Timing register 3 */;
pub const DSI_LCD1_WC_0: c_uint = 0x120   /* Word Count register 0 */;
pub const DSI_LCD1_WC_1: c_uint = 0x124   /* Word Count register 1 */;
pub const DSI_LCD1_WC_2: c_uint = 0x128	 /* Word Count register 2 */;
    pub timing0: u32,
    pub timing1: u32,
    pub timing2: u32,
    pub timing3: u32,
    pub wc0: u32,
    pub wc1: u32,
    pub wc2: u32,
    pub reserved2: [u32; 1],
    pub slot_cnt0: u32,
    pub slot_cnt1: u32,
    pub reserved3: [u32; 2],
    pub status_0: u32,
    pub status_1: u32,
    pub status_2: u32,
    pub status_3: u32,
    pub status_4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_regs {
pub const DSI_CTRL_0: c_uint = 0x000   /* DSI control register 0 */;
pub const DSI_CTRL_1: c_uint = 0x004   /* DSI control register 1 */;
    pub ctrl0: u32,
    pub ctrl1: u32,
    pub reserved1: [u32; 2],
    pub irq_status: u32,
    pub irq_mask: u32,
    pub reserved2: [u32; 2],
pub const DSI_CPU_CMD_0: c_uint = 0x020   /* DSI CPU packet command register 0 */;
pub const DSI_CPU_CMD_1: c_uint = 0x024   /* DSU CPU Packet Command Register 1 */;
pub const DSI_CPU_CMD_3: c_uint = 0x02C   /* DSU CPU Packet Command Register 3 */;
pub const DSI_CPU_WDAT_0: c_uint = 0x030   /* DSI CUP */;
    pub cmd0: u32,
    pub cmd1: u32,
    pub cmd2: u32,
    pub cmd3: u32,
    pub dat0: u32,
    pub status0: u32,
    pub status1: u32,
    pub status2: u32,
    pub status3: u32,
    pub status4: u32,
    pub reserved3: [u32; 2],
    pub smt_cmd: u32,
    pub smt_ctrl0: u32,
    pub smt_ctrl1: u32,
    pub reserved4: [u32; 1],
    pub rx0_status: u32,
// Rx Packet Header - data from slave device
pub const DSI_RX_PKT_HDR_0: c_uint = 0x064;
    pub rx0_header: u32,
    pub rx1_status: u32,
    pub rx1_header: u32,
    pub rx_ctrl: u32,
    pub rx_ctrl1: u32,
    pub rx2_status: u32,
    pub rx2_header: u32,
    pub reserved5: [u32; 1],
    pub phy_ctrl1: u32,
pub const DSI_PHY_CTRL_2: c_uint = 0x088   /* DSI DPHI Control Register 2 */;
pub const DSI_PHY_CTRL_3: c_uint = 0x08C   /* DPHY Control Register 3 */;
    pub phy_ctrl2: u32,
    pub phy_ctrl3: u32,
    pub phy_status0: u32,
    pub phy_status1: u32,
    pub reserved6: [u32; 5],
    pub phy_status2: u32,
pub const DSI_PHY_RCOMP_0: c_uint = 0x0B0   /* DPHY Rcomp Control Register */;
    pub phy_rcomp0: u32,
    pub reserved7: [u32; 3],
pub const DSI_PHY_TIME_0: c_uint = 0x0C0   /* DPHY Timing Control Register 0 */;
pub const DSI_PHY_TIME_1: c_uint = 0x0C4   /* DPHY Timing Control Register 1 */;
pub const DSI_PHY_TIME_2: c_uint = 0x0C8   /* DPHY Timing Control Register 2 */;
pub const DSI_PHY_TIME_3: c_uint = 0x0CC   /* DPHY Timing Control Register 3 */;
pub const DSI_PHY_TIME_4: c_uint = 0x0D0   /* DPHY Timing Control Register 4 */;
pub const DSI_PHY_TIME_5: c_uint = 0x0D4   /* DPHY Timing Control Register 5 */;
    pub phy_timing0: u32,
    pub phy_timing1: u32,
    pub phy_timing2: u32,
    pub phy_timing3: u32,
    pub phy_code_0: u32,
    pub phy_code_1: u32,
    pub reserved8: [u32; 2],
    pub mem_ctrl: u32,
    pub tx_timer: u32,
    pub rx_timer: u32,
    pub turn_timer: u32,
    pub reserved9: [u32; 4],
pub const DSI_LCD1_CTRL_0: c_uint = 0x100   /* DSI Active Panel 1 Control register 0 */;
pub const DSI_LCD1_CTRL_1: c_uint = 0x104   /* DSI Active Panel 1 Control register 1 */;
pub const DSI_LCD1_TIMING_0: c_uint = 0x110   /* Timing register 0 */;
pub const DSI_LCD1_TIMING_1: c_uint = 0x114   /* Timing register 1 */;
pub const DSI_LCD1_TIMING_2: c_uint = 0x118   /* Timing register 2 */;
pub const DSI_LCD1_TIMING_3: c_uint = 0x11C   /* Timing register 3 */;
pub const DSI_LCD1_WC_0: c_uint = 0x120   /* Word Count register 0 */;
pub const DSI_LCD1_WC_1: c_uint = 0x124   /* Word Count register 1 */;
pub const DSI_LCD1_WC_2: c_uint = 0x128   /* Word Count register 2 */;
    pub lcd1: dsi_lcd_regs,
    pub reserved10: [u32; 11],
    pub lcd2: dsi_lcd_regs,
}

pub const DSI_LCD2_CTRL_0: c_uint = 0x180   /* DSI Active Panel 2 Control register 0 */;
pub const DSI_LCD2_CTRL_1: c_uint = 0x184   /* DSI Active Panel 2 Control register 1 */;
pub const DSI_LCD2_TIMING_0: c_uint = 0x190   /* Timing register 0 */;
pub const DSI_LCD2_TIMING_1: c_uint = 0x194   /* Timing register 1 */;
pub const DSI_LCD2_TIMING_2: c_uint = 0x198   /* Timing register 2 */;
pub const DSI_LCD2_TIMING_3: c_uint = 0x19C   /* Timing register 3 */;
pub const DSI_LCD2_WC_0: c_uint = 0x1A0   /* Word Count register 0 */;
pub const DSI_LCD2_WC_1: c_uint = 0x1A4   /* Word Count register 1 */;
pub const DSI_LCD2_WC_2: c_uint = 0x1A8	 /* Word Count register 2 */;
// DSI_CTRL_0		0x0000	DSI Control Register 0

// DSI_CTRL_1		0x0004	DSI Control Register 1

pub const DSI_CTRL_1_CFG_LCD2_VCH_NO_SHIFT: c_int = 2;

pub const DSI_CTRL_1_CFG_LCD1_VCH_NO_SHIFT: c_int = 0;
// DSI_LCD1_CTRL_1	0x0104	DSI Active Panel 1 Control Register 1
// LCD 1 Vsync Reset Enable

// LCD 1 2K Pixel Buffer Mode Enable

// Bit(s) DSI_LCD1_CTRL_1_RSRV_29_23 reserved
// Long Blanking Packet Enable

// Extra Long Blanking Packet Enable

// Front Porch Packet Enable

// hact Packet Enable

// Back Porch Packet Enable

// hse Packet Enable

// hsa Packet Enable

// All Item Enable after Pixel Data

// Extra Long Packet Enable after Pixel Data

// Bit(s) DSI_LCD1_CTRL_1_RSRV_13_11 reserved
// Turn Around Bus at Last h Line

// Go to Low Power Every Frame

// Go to Low Power Every Line

// Bit(s) DSI_LCD1_CTRL_1_RSRV_7_4 reserved
// DSI Transmission Mode for LCD 1
pub const DSI_LCD1_CTRL_1_CFG_L1_BURST_MODE_SHIFT: c_int = 2;

// LCD 1 Input Data RGB Mode for LCD 1
pub const DSI_LCD2_CTRL_1_CFG_L1_RGB_TYPE_SHIFT: c_int = 0;

// DSI_PHY_CTRL_2		0x0088	DPHY Control Register 2
// Bit(s) DSI_PHY_CTRL_2_RSRV_31_12 reserved
// DPHY LP Receiver Enable

pub const DSI_PHY_CTRL_2_CFG_CSR_LANE_RESC_EN_SHIFT: c_int = 8;
// DPHY Data Lane Enable

pub const DSI_PHY_CTRL_2_CFG_CSR_LANE_EN_SHIFT: c_int = 4;
// DPHY Bus Turn Around

pub const DSI_PHY_CTRL_2_CFG_CSR_LANE_TURN_SHIFT: c_int = 0;
// DSI_CPU_CMD_1		0x0024	DSI CPU Packet Command Register 1
// Bit(s) DSI_CPU_CMD_1_RSRV_31_24 reserved
// LPDT TX Enable

pub const DSI_CPU_CMD_1_CFG_TXLP_LPDT_SHIFT: c_int = 20;
// ULPS TX Enable

pub const DSI_CPU_CMD_1_CFG_TXLP_ULPS_SHIFT: c_int = 16;
// Low Power TX Trigger Code

pub const DSI_CPU_CMD_1_CFG_TXLP_TRIGGER_CODE_SHIFT: c_int = 0;
// DSI_PHY_TIME_0	0x00c0	DPHY Timing Control Register 0
// Length of HS Exit Period in tx_clk_esc Cycles

pub const DSI_PHY_TIME_0_CFG_CSR_TIME_HS_EXIT_SHIFT: c_int = 24;
// DPHY HS Trail Period Length

pub const DSI_PHY_TIME_0_CFG_CSR_TIME_HS_TRAIL_SHIFT: c_int = 16;
// DPHY HS Zero State Length

pub const DSI_PHY_TIME_0_CDG_CSR_TIME_HS_ZERO_SHIFT: c_int = 8;
// DPHY HS Prepare State Length

pub const DSI_PHY_TIME_0_CFG_CSR_TIME_HS_PREP_SHIFT: c_int = 0;
// DSI_PHY_TIME_1		0x00c4	DPHY Timing Control Register 1
// Time to Drive LP-00 by New Transmitter

pub const DSI_PHY_TIME_1_CFG_CSR_TIME_TA_GET_SHIFT: c_int = 24;
// Time to Drive LP-00 after Turn Request

pub const DSI_PHY_TIME_1_CFG_CSR_TIME_TA_GO_SHIFT: c_int = 16;
// DPHY HS Wakeup Period Length

pub const DSI_PHY_TIME_1_CFG_CSR_TIME_WAKEUP_SHIFT: c_int = 0;
// DSI_PHY_TIME_2		0x00c8	DPHY Timing Control Register 2
// DPHY CLK Exit Period Length

pub const DSI_PHY_TIME_2_CFG_CSR_TIME_CK_EXIT_SHIFT: c_int = 24;
// DPHY CLK Trail Period Length

pub const DSI_PHY_TIME_2_CFG_CSR_TIME_CK_TRAIL_SHIFT: c_int = 16;
// DPHY CLK Zero State Length

pub const DSI_PHY_TIME_2_CFG_CSR_TIME_CK_ZERO_SHIFT: c_int = 8;
// DPHY CLK LP Length

pub const DSI_PHY_TIME_2_CFG_CSR_TIME_CK_LPX_SHIFT: c_int = 0;
// DSI_PHY_TIME_3		0x00cc	DPHY Timing Control Register 3
// Bit(s) DSI_PHY_TIME_3_RSRV_31_16 reserved
// DPHY LP Length

pub const DSI_PHY_TIME_3_CFG_CSR_TIME_LPX_SHIFT: c_int = 8;
// DPHY HS req to rdy Length

pub const DSI_PHY_TIME_3_CFG_CSR_TIME_REQRDY_SHIFT: c_int = 0;

// LVDS
// LVDS_PHY_CTRL
pub const LVDS_PHY_CTL: c_uint = 0x2A4;

pub const LVDS_PHY_CTL_EXT: c_uint = 0x2A8;
// LVDS_PHY_CTRL_EXT1

// LVDS_PHY_CTRL_EXT2

// LVDS_PHY_CTRL_EXT3

// LVDS_PHY_CTRL_EXT4

// LVDS_PHY_CTRL_EXT5

//
// mmp path describes part of mmp path related info:
// which is hiden in display driver and not exported to buffer driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmphw_path_plat {
    pub id: c_int,
    pub ctrl: *mut mmphw_ctrl,
    pub path: *mut mmp_path,
    pub path_config: u32,
    pub link_config: u32,
    pub dsi_rbswap: u32,
}

// mmp ctrl describes mmp controller related info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmphw_ctrl {
// platform related, get from config
    pub name: *const c_char,
    pub irq: c_int,
    pub reg_base: *mut void __iomem,
    pub clk: *mut clk,
// sys info
    pub dev: *mut device,
// state
    pub open_count: c_int,
    pub status: c_int,
    pub access_ok: mutex,
// pathes
    pub path_num: c_int,
    pub __counted_by(path_num): mmphw_path_plat path_plats[],
}

extern "C" {
    pub fn path_to_ctrl(_arg: overlay->path) -> return;
}
// path regs, for regs symmetrical for both pathes

extern "C" {
    pub fn lcd_spi_register(ctrl: *mut mmphw_ctrl) -> c_int;
}

