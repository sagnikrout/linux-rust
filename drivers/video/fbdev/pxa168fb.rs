//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/pxa168fb.h
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


// SPDX-License-Identifier: GPL-2.0
// ------------< LCD register >------------
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
pub const GRA_FRAME_CNT_MASK: c_uint = 0x0000000C  /* Graphic */;
pub const DMA_FRAME_CNT_MASK: c_uint = 0x00000003  /* Video */;
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

pub const SCLK_SOURCE_SELECT_MASK: c_uint = 0x80000000;

pub const CLK_FRACDIV_MASK: c_uint = 0x0FFF0000;

pub const CLK_INT_DIV_MASK: c_uint = 0x0000FFFF;
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

pub const CFG_CMD_VM_ENA_MASK: c_uint = 0x00000800;

pub const CFG_CSC_MASK: c_uint = 0x00000300;

pub const CFG_AXICTRL_MASK: c_uint = 0x000000F0;

pub const CFG_IOPADMODE_MASK: c_uint = 0x0000000F;
// LCD Interrupt Control Register
pub const SPU_IRQ_ENA: c_uint = 0x01C0;

pub const DMA_FRAME_IRQ0_ENA_MASK: c_uint = 0x80000000;

pub const DMA_FRAME_IRQ1_ENA_MASK: c_uint = 0x40000000;

pub const DMA_FF_UNDERFLOW_ENA_MASK: c_uint = 0x20000000;

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

pub const ERR_IRQ_ENA_MASK: c_uint = 0x00010000;

pub const CLEAN_SPU_IRQ_ISR_MASK: c_uint = 0x0000FFFF;
// LCD Interrupt Status Register
pub const SPU_IRQ_ISR: c_uint = 0x01C4;

pub const DMA_FRAME_IRQ0_MASK: c_uint = 0x80000000;

pub const DMA_FRAME_IRQ1_MASK: c_uint = 0x40000000;

pub const DMA_FF_UNDERFLOW_MASK: c_uint = 0x20000000;

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

pub const ERR_IRQ_MASK: c_uint = 0x00010000;
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
//
// defined Video Memory Color format for DMA control 0 register
// DMA0 bit[23:20]
//
pub const VMODE_RGB565: c_uint = 0x0;
pub const VMODE_RGB1555: c_uint = 0x1;
pub const VMODE_RGB888PACKED: c_uint = 0x2;
pub const VMODE_RGB888UNPACKED: c_uint = 0x3;
pub const VMODE_RGBA888: c_uint = 0x4;
pub const VMODE_YUV422PACKED: c_uint = 0x5;
pub const VMODE_YUV422PLANAR: c_uint = 0x6;
pub const VMODE_YUV420PLANAR: c_uint = 0x7;
pub const VMODE_SMPNCMD: c_uint = 0x8;
pub const VMODE_PALETTE4BIT: c_uint = 0x9;
pub const VMODE_PALETTE8BIT: c_uint = 0xa;
pub const VMODE_RESERVED: c_uint = 0xb;
//
// defined Graphic Memory Color format for DMA control 0 register
// DMA0 bit[19:16]
//
pub const GMODE_RGB565: c_uint = 0x0;
pub const GMODE_RGB1555: c_uint = 0x1;
pub const GMODE_RGB888PACKED: c_uint = 0x2;
pub const GMODE_RGB888UNPACKED: c_uint = 0x3;
pub const GMODE_RGBA888: c_uint = 0x4;
pub const GMODE_YUV422PACKED: c_uint = 0x5;
pub const GMODE_YUV422PLANAR: c_uint = 0x6;
pub const GMODE_YUV420PLANAR: c_uint = 0x7;
pub const GMODE_SMPNCMD: c_uint = 0x8;
pub const GMODE_PALETTE4BIT: c_uint = 0x9;
pub const GMODE_PALETTE8BIT: c_uint = 0xa;
pub const GMODE_RESERVED: c_uint = 0xb;
//
// define for DMA control 1 register
//

pub const DMA1_VSYNC_MODE: c_int = 28;
pub const DMA1_VSYNC_INV: c_int = 27;
pub const DMA1_CKEY: c_int = 24;
pub const DMA1_CARRY: c_int = 23;
pub const DMA1_LNBUF_ENA: c_int = 22;
pub const DMA1_GATED_ENA: c_int = 21;
pub const DMA1_PWRDN_ENA: c_int = 20;
pub const DMA1_DSCALE: c_int = 18;
pub const DMA1_ALPHA_MODE: c_int = 16;
pub const DMA1_ALPHA: c_int = 08;
pub const DMA1_PXLCMD: c_int = 00;
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
