//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/cyber2000fb.h
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
// linux/drivers/video/cyber2000fb.h
//
// Copyright (C) 1998-2000 Russell King
//
// Integraphics Cyber2000 frame buffer device
//
// Internal CyberPro sizes and offsets.
//
pub const MMIO_OFFSET: c_uint = 0x00800000;
pub const MMIO_SIZE: c_uint = 0x000c0000;
pub const NR_PALETTE: c_int = 256;

extern "C" {
    pub fn printascii(: *const c_char);
}

pub const RAMDAC_RAMPWRDN: c_uint = 0x01;
pub const RAMDAC_DAC8BIT: c_uint = 0x02;
pub const RAMDAC_VREFEN: c_uint = 0x04;
pub const RAMDAC_BYPASS: c_uint = 0x10;
pub const RAMDAC_DACPWRDN: c_uint = 0x40;
pub const EXT_CRT_VRTOFL: c_uint = 0x11;
pub const EXT_CRT_VRTOFL_LINECOMP10: c_uint = 0x10;
pub const EXT_CRT_VRTOFL_INTERLACE: c_uint = 0x20;
pub const EXT_CRT_IRQ: c_uint = 0x12;
pub const EXT_CRT_IRQ_ENABLE: c_uint = 0x01;
pub const EXT_CRT_IRQ_ACT_HIGH: c_uint = 0x04;
pub const EXT_CRT_TEST: c_uint = 0x13;
pub const EXT_SYNC_CTL: c_uint = 0x16;
pub const EXT_SYNC_CTL_HS_NORMAL: c_uint = 0x00;
pub const EXT_SYNC_CTL_HS_0: c_uint = 0x01;
pub const EXT_SYNC_CTL_HS_1: c_uint = 0x02;
pub const EXT_SYNC_CTL_HS_HSVS: c_uint = 0x03;
pub const EXT_SYNC_CTL_VS_NORMAL: c_uint = 0x00;
pub const EXT_SYNC_CTL_VS_0: c_uint = 0x04;
pub const EXT_SYNC_CTL_VS_1: c_uint = 0x08;
pub const EXT_SYNC_CTL_VS_COMP: c_uint = 0x0c;
pub const EXT_BUS_CTL: c_uint = 0x30;
pub const EXT_BUS_CTL_LIN_1MB: c_uint = 0x00;
pub const EXT_BUS_CTL_LIN_2MB: c_uint = 0x01;
pub const EXT_BUS_CTL_LIN_4MB: c_uint = 0x02;
pub const EXT_BUS_CTL_ZEROWAIT: c_uint = 0x04;
pub const EXT_BUS_CTL_PCIBURST_WRITE: c_uint = 0x20;
pub const EXT_BUS_CTL_PCIBURST_READ: c_uint = 0x80	/* CyberPro 5000 only */;
pub const EXT_SEG_WRITE_PTR: c_uint = 0x31;
pub const EXT_SEG_READ_PTR: c_uint = 0x32;
pub const EXT_BIU_MISC: c_uint = 0x33;
pub const EXT_BIU_MISC_LIN_ENABLE: c_uint = 0x01;
pub const EXT_BIU_MISC_COP_ENABLE: c_uint = 0x04;
pub const EXT_BIU_MISC_COP_BFC: c_uint = 0x08;
pub const EXT_FUNC_CTL: c_uint = 0x3c;
pub const EXT_FUNC_CTL_EXTREGENBL: c_uint = 0x80	/* enable access to 0xbcxxx		*/;
pub const PCI_BM_CTL: c_uint = 0x3e;
pub const PCI_BM_CTL_ENABLE: c_uint = 0x01	/* enable bus-master			*/;
pub const PCI_BM_CTL_BURST: c_uint = 0x02	/* enable burst				*/;
pub const PCI_BM_CTL_BACK2BACK: c_uint = 0x04	/* enable back to back			*/;
pub const PCI_BM_CTL_DUMMY: c_uint = 0x08	/* insert dummy cycle			*/;
pub const X_V2_VID_MEM_START: c_uint = 0x40;
pub const X_V2_VID_SRC_WIDTH: c_uint = 0x43;
pub const X_V2_X_START: c_uint = 0x45;
pub const X_V2_X_END: c_uint = 0x47;
pub const X_V2_Y_START: c_uint = 0x49;
pub const X_V2_Y_END: c_uint = 0x4b;
pub const X_V2_VID_SRC_WIN_WIDTH: c_uint = 0x4d;
pub const Y_V2_DDA_X_INC: c_uint = 0x43;
pub const Y_V2_DDA_Y_INC: c_uint = 0x47;
pub const Y_V2_VID_FIFO_CTL: c_uint = 0x49;
pub const Y_V2_VID_FMT: c_uint = 0x4b;
pub const Y_V2_VID_DISP_CTL1: c_uint = 0x4c;
pub const Y_V2_VID_FIFO_CTL1: c_uint = 0x4d;
pub const J_X2_VID_MEM_START: c_uint = 0x40;
pub const J_X2_VID_SRC_WIDTH: c_uint = 0x43;
pub const J_X2_X_START: c_uint = 0x47;
pub const J_X2_X_END: c_uint = 0x49;
pub const J_X2_Y_START: c_uint = 0x4b;
pub const J_X2_Y_END: c_uint = 0x4d;
pub const J_X2_VID_SRC_WIN_WIDTH: c_uint = 0x4f;
pub const K_X2_DDA_X_INIT: c_uint = 0x40;
pub const K_X2_DDA_X_INC: c_uint = 0x42;
pub const K_X2_DDA_Y_INIT: c_uint = 0x44;
pub const K_X2_DDA_Y_INC: c_uint = 0x46;
pub const K_X2_VID_FMT: c_uint = 0x48;
pub const K_X2_VID_DISP_CTL1: c_uint = 0x49;
pub const K_CAP_X2_CTL1: c_uint = 0x49;
pub const CURS_H_START: c_uint = 0x50;
pub const CURS_H_PRESET: c_uint = 0x52;
pub const CURS_V_START: c_uint = 0x53;
pub const CURS_V_PRESET: c_uint = 0x55;
pub const CURS_CTL: c_uint = 0x56;
pub const EXT_ATTRIB_CTL: c_uint = 0x57;
pub const EXT_ATTRIB_CTL_EXT: c_uint = 0x01;
pub const EXT_OVERSCAN_RED: c_uint = 0x58;
pub const EXT_OVERSCAN_GREEN: c_uint = 0x59;
pub const EXT_OVERSCAN_BLUE: c_uint = 0x5a;
pub const CAP_X_START: c_uint = 0x60;
pub const CAP_X_END: c_uint = 0x62;
pub const CAP_Y_START: c_uint = 0x64;
pub const CAP_Y_END: c_uint = 0x66;
pub const CAP_DDA_X_INIT: c_uint = 0x68;
pub const CAP_DDA_X_INC: c_uint = 0x6a;
pub const CAP_DDA_Y_INIT: c_uint = 0x6c;
pub const CAP_DDA_Y_INC: c_uint = 0x6e;
pub const EXT_MEM_CTL0: c_uint = 0x70;
pub const EXT_MEM_CTL0_7CLK: c_uint = 0x01;
pub const EXT_MEM_CTL0_RAS_1: c_uint = 0x02;
pub const EXT_MEM_CTL0_RAS2CAS_1: c_uint = 0x04;
pub const EXT_MEM_CTL0_MULTCAS: c_uint = 0x08;
pub const EXT_MEM_CTL0_ASYM: c_uint = 0x10;
pub const EXT_MEM_CTL0_CAS1ON: c_uint = 0x20;
pub const EXT_MEM_CTL0_FIFOFLUSH: c_uint = 0x40;
pub const EXT_MEM_CTL0_SEQRESET: c_uint = 0x80;
pub const EXT_MEM_CTL1: c_uint = 0x71;
pub const EXT_MEM_CTL1_PAR: c_uint = 0x00;
pub const EXT_MEM_CTL1_SERPAR: c_uint = 0x01;
pub const EXT_MEM_CTL1_SER: c_uint = 0x03;
pub const EXT_MEM_CTL1_SYNC: c_uint = 0x04;
pub const EXT_MEM_CTL1_VRAM: c_uint = 0x08;
pub const EXT_MEM_CTL1_4K_REFRESH: c_uint = 0x10;
pub const EXT_MEM_CTL1_256Kx4: c_uint = 0x00;
pub const EXT_MEM_CTL1_512Kx8: c_uint = 0x40;
pub const EXT_MEM_CTL1_1Mx16: c_uint = 0x60;
pub const EXT_MEM_CTL2: c_uint = 0x72;
pub const MEM_CTL2_SIZE_1MB: c_uint = 0x00;
pub const MEM_CTL2_SIZE_2MB: c_uint = 0x01;
pub const MEM_CTL2_SIZE_4MB: c_uint = 0x02;
pub const MEM_CTL2_SIZE_MASK: c_uint = 0x03;
pub const MEM_CTL2_64BIT: c_uint = 0x04;
pub const EXT_HIDDEN_CTL1: c_uint = 0x73;
pub const EXT_FIFO_CTL: c_uint = 0x74;
pub const EXT_SEQ_MISC: c_uint = 0x77;
pub const EXT_SEQ_MISC_8: c_uint = 0x01;
pub const EXT_SEQ_MISC_16_RGB565: c_uint = 0x02;
pub const EXT_SEQ_MISC_32: c_uint = 0x03;
pub const EXT_SEQ_MISC_24_RGB888: c_uint = 0x04;
pub const EXT_SEQ_MISC_16_RGB555: c_uint = 0x06;
pub const EXT_SEQ_MISC_8_RGB332: c_uint = 0x09;
pub const EXT_SEQ_MISC_16_RGB444: c_uint = 0x0a;
pub const EXT_HIDDEN_CTL4: c_uint = 0x7a;
pub const CURS_MEM_START: c_uint = 0x7e		/* bits 23..12 */;
pub const CAP_PIP_X_START: c_uint = 0x80;
pub const CAP_PIP_X_END: c_uint = 0x82;
pub const CAP_PIP_Y_START: c_uint = 0x84;
pub const CAP_PIP_Y_END: c_uint = 0x86;
pub const EXT_CAP_CTL1: c_uint = 0x88;
pub const EXT_CAP_CTL2: c_uint = 0x89;
pub const EXT_CAP_CTL2_ODDFRAMEIRQ: c_uint = 0x01;
pub const EXT_CAP_CTL2_ANYFRAMEIRQ: c_uint = 0x02;
pub const BM_CTRL0: c_uint = 0x9c;
pub const BM_CTRL1: c_uint = 0x9d;
pub const EXT_CAP_MODE1: c_uint = 0xa4;
pub const EXT_CAP_MODE1_8BIT: c_uint = 0x01	/* enable 8bit capture mode		*/;
pub const EXT_CAP_MODE1_CCIR656: c_uint = 0x02	/* CCIR656 mode				*/;
pub const EXT_CAP_MODE1_IGNOREVGT: c_uint = 0x04	/* ignore VGT				*/;
pub const EXT_CAP_MODE1_ALTFIFO: c_uint = 0x10	/* use alternate FIFO for capture	*/;
pub const EXT_CAP_MODE1_SWAPUV: c_uint = 0x20	/* swap UV bytes			*/;
pub const EXT_CAP_MODE1_MIRRORY: c_uint = 0x40	/* mirror vertically			*/;
pub const EXT_CAP_MODE1_MIRRORX: c_uint = 0x80	/* mirror horizontally			*/;
pub const EXT_CAP_MODE2: c_uint = 0xa5;
pub const EXT_CAP_MODE2_CCIRINVOE: c_uint = 0x01;
pub const EXT_CAP_MODE2_CCIRINVVGT: c_uint = 0x02;
pub const EXT_CAP_MODE2_CCIRINVHGT: c_uint = 0x04;
pub const EXT_CAP_MODE2_CCIRINVDG: c_uint = 0x08;
pub const EXT_CAP_MODE2_DATEND: c_uint = 0x10;
pub const EXT_CAP_MODE2_CCIRDGH: c_uint = 0x20;
pub const EXT_CAP_MODE2_FIXSONY: c_uint = 0x40;
pub const EXT_CAP_MODE2_SYNCFREEZE: c_uint = 0x80;
pub const EXT_TV_CTL: c_uint = 0xae;
pub const EXT_DCLK_MULT: c_uint = 0xb0;
pub const EXT_DCLK_DIV: c_uint = 0xb1;
pub const EXT_DCLK_DIV_VFSEL: c_uint = 0x20;
pub const EXT_MCLK_MULT: c_uint = 0xb2;
pub const EXT_MCLK_DIV: c_uint = 0xb3;
pub const EXT_LATCH1: c_uint = 0xb5;
pub const EXT_LATCH1_VAFC_EN: c_uint = 0x01	/* enable VAFC				*/;
pub const EXT_FEATURE: c_uint = 0xb7;
pub const EXT_FEATURE_BUS_MASK: c_uint = 0x07	/* host bus mask			*/;
pub const EXT_FEATURE_BUS_PCI: c_uint = 0x00;
pub const EXT_FEATURE_BUS_VL_STD: c_uint = 0x04;
pub const EXT_FEATURE_BUS_VL_LINEAR: c_uint = 0x05;
pub const EXT_FEATURE_1682: c_uint = 0x20	/* IGS 1682 compatibility		*/;
pub const EXT_LATCH2: c_uint = 0xb6;
pub const EXT_LATCH2_I2C_CLKEN: c_uint = 0x10;
pub const EXT_LATCH2_I2C_CLK: c_uint = 0x20;
pub const EXT_LATCH2_I2C_DATEN: c_uint = 0x40;
pub const EXT_LATCH2_I2C_DAT: c_uint = 0x80;
pub const EXT_XT_CTL: c_uint = 0xbe;
pub const EXT_XT_CAP16: c_uint = 0x04;
pub const EXT_XT_LINEARFB: c_uint = 0x08;
pub const EXT_XT_PAL: c_uint = 0x10;
pub const EXT_MEM_START: c_uint = 0xc0		/* ext start address 21 bits		*/;
pub const HOR_PHASE_SHIFT: c_uint = 0xc2		/* high 3 bits				*/;
pub const EXT_SRC_WIDTH: c_uint = 0xc3		/* ext offset phase  10 bits		*/;
pub const EXT_SRC_HEIGHT: c_uint = 0xc4		/* high 6 bits				*/;
pub const EXT_X_START: c_uint = 0xc5		/* ext->screen, 16 bits			*/;
pub const EXT_X_END: c_uint = 0xc7		/* ext->screen, 16 bits			*/;
pub const EXT_Y_START: c_uint = 0xc9		/* ext->screen, 16 bits			*/;
pub const EXT_Y_END: c_uint = 0xcb		/* ext->screen, 16 bits			*/;
pub const EXT_SRC_WIN_WIDTH: c_uint = 0xcd		/* 8 bits				*/;
pub const EXT_COLOUR_COMPARE: c_uint = 0xce		/* 24 bits				*/;
pub const EXT_DDA_X_INIT: c_uint = 0xd1		/* ext->screen 16 bits			*/;
pub const EXT_DDA_X_INC: c_uint = 0xd3		/* ext->screen 16 bits			*/;
pub const EXT_DDA_Y_INIT: c_uint = 0xd5		/* ext->screen 16 bits			*/;
pub const EXT_DDA_Y_INC: c_uint = 0xd7		/* ext->screen 16 bits			*/;
pub const EXT_VID_FIFO_CTL: c_uint = 0xd9;
pub const EXT_VID_FMT: c_uint = 0xdb;
pub const EXT_VID_FMT_YUV422: c_uint = 0x00	/* formats - does this cause conversion? */;
pub const EXT_VID_FMT_RGB555: c_uint = 0x01;
pub const EXT_VID_FMT_RGB565: c_uint = 0x02;
pub const EXT_VID_FMT_RGB888_24: c_uint = 0x03;
pub const EXT_VID_FMT_RGB888_32: c_uint = 0x04;
pub const EXT_VID_FMT_RGB8: c_uint = 0x05;
pub const EXT_VID_FMT_RGB4444: c_uint = 0x06;
pub const EXT_VID_FMT_RGB8T: c_uint = 0x07;
pub const EXT_VID_FMT_DUP_PIX_ZOON: c_uint = 0x08	/* duplicate pixel zoom			*/;
pub const EXT_VID_FMT_MOD_3RD_PIX: c_uint = 0x20	/* modify 3rd duplicated pixel		*/;
pub const EXT_VID_FMT_DBL_H_PIX: c_uint = 0x40	/* double horiz pixels			*/;
pub const EXT_VID_FMT_YUV128: c_uint = 0x80	/* YUV data offset by 128		*/;
pub const EXT_VID_DISP_CTL1: c_uint = 0xdc;
pub const EXT_VID_DISP_CTL1_INTRAM: c_uint = 0x01	/* video pixels go to internal RAM	*/;
pub const EXT_VID_DISP_CTL1_IGNORE_CCOMP: c_uint = 0x02	/* ignore colour compare registers	*/;
pub const EXT_VID_DISP_CTL1_NOCLIP: c_uint = 0x04	/* do not clip to 16235,16240		*/;
pub const EXT_VID_DISP_CTL1_UV_AVG: c_uint = 0x08	/* U/V data is averaged			*/;
pub const EXT_VID_DISP_CTL1_Y128: c_uint = 0x10	/* Y data offset by 128 (if YUV128 set)	*/;
pub const EXT_VID_DISP_CTL1_VINTERPOL_OFF: c_uint = 0x20	/* disable vertical interpolation	*/;
pub const EXT_VID_DISP_CTL1_FULL_WIN: c_uint = 0x40	/* video out window full		*/;
pub const EXT_VID_DISP_CTL1_ENABLE_WINDOW: c_uint = 0x80	/* enable video window			*/;
pub const EXT_VID_FIFO_CTL1: c_uint = 0xdd;
pub const EXT_VID_FIFO_CTL1_OE_HIGH: c_uint = 0x02;
pub const EXT_VID_FIFO_CTL1_INTERLEAVE: c_uint = 0x04	/* enable interleaved memory read	*/;
pub const EXT_ROM_UCB4GH: c_uint = 0xe5;
pub const EXT_ROM_UCB4GH_FREEZE: c_uint = 0x02	/* capture frozen			*/;
pub const EXT_ROM_UCB4GH_ODDFRAME: c_uint = 0x04	/* 1 = odd frame captured		*/;
pub const EXT_ROM_UCB4GH_1HL: c_uint = 0x08	/* first horizonal line after VGT falling edge */;
pub const EXT_ROM_UCB4GH_ODD: c_uint = 0x10	/* odd frame indicator			*/;
pub const EXT_ROM_UCB4GH_INTSTAT: c_uint = 0x20	/* video interrupt			*/;
pub const VFAC_CTL1: c_uint = 0xe8;
pub const VFAC_CTL1_CAPTURE: c_uint = 0x01	/* capture enable (only when VSYNC high)*/;
pub const VFAC_CTL1_VFAC_ENABLE: c_uint = 0x02	/* vfac enable				*/;
pub const VFAC_CTL1_FREEZE_CAPTURE: c_uint = 0x04	/* freeze capture			*/;
pub const VFAC_CTL1_FREEZE_CAPTURE_SYNC: c_uint = 0x08	/* sync freeze capture			*/;
pub const VFAC_CTL1_VALIDFRAME_SRC: c_uint = 0x10	/* select valid frame source		*/;
pub const VFAC_CTL1_PHILIPS: c_uint = 0x40	/* select Philips mode			*/;
pub const VFAC_CTL1_MODVINTERPOLCLK: c_uint = 0x80	/* modify vertical interpolation clocl	*/;
pub const VFAC_CTL2: c_uint = 0xe9;
pub const VFAC_CTL2_INVERT_VIDDATAVALID: c_uint = 0x01	/* invert video data valid		*/;
pub const VFAC_CTL2_INVERT_GRAPHREADY: c_uint = 0x02	/* invert graphic ready output sig	*/;
pub const VFAC_CTL2_INVERT_DATACLK: c_uint = 0x04	/* invert data clock signal		*/;
pub const VFAC_CTL2_INVERT_HSYNC: c_uint = 0x08	/* invert hsync input			*/;
pub const VFAC_CTL2_INVERT_VSYNC: c_uint = 0x10	/* invert vsync input			*/;
pub const VFAC_CTL2_INVERT_FRAME: c_uint = 0x20	/* invert frame odd/even input		*/;
pub const VFAC_CTL2_INVERT_BLANK: c_uint = 0x40	/* invert blank output			*/;
pub const VFAC_CTL2_INVERT_OVSYNC: c_uint = 0x80	/* invert other vsync input		*/;
pub const VFAC_CTL3: c_uint = 0xea;
pub const VFAC_CTL3_CAP_LARGE_FIFO: c_uint = 0x01	/* large capture fifo			*/;
pub const VFAC_CTL3_CAP_INTERLACE: c_uint = 0x02	/* capture odd and even fields		*/;
pub const VFAC_CTL3_CAP_HOLD_4NS: c_uint = 0x00	/* hold capture data for 4ns		*/;
pub const VFAC_CTL3_CAP_HOLD_2NS: c_uint = 0x04	/* hold capture data for 2ns		*/;
pub const VFAC_CTL3_CAP_HOLD_6NS: c_uint = 0x08	/* hold capture data for 6ns		*/;
pub const VFAC_CTL3_CAP_HOLD_0NS: c_uint = 0x0c	/* hold capture data for 0ns		*/;
pub const VFAC_CTL3_CHROMAKEY: c_uint = 0x20	/* capture data will be chromakeyed	*/;
pub const VFAC_CTL3_CAP_IRQ: c_uint = 0x40	/* enable capture interrupt		*/;
pub const CAP_MEM_START: c_uint = 0xeb		/* 18 bits				*/;
pub const CAP_MAP_WIDTH: c_uint = 0xed		/* high 6 bits				*/;
pub const CAP_PITCH: c_uint = 0xee		/* 8 bits				*/;
pub const CAP_CTL_MISC: c_uint = 0xef;
pub const CAP_CTL_MISC_HDIV: c_uint = 0x01;
pub const CAP_CTL_MISC_HDIV4: c_uint = 0x02;
pub const CAP_CTL_MISC_ODDEVEN: c_uint = 0x04;
pub const CAP_CTL_MISC_HSYNCDIV2: c_uint = 0x08;
pub const CAP_CTL_MISC_SYNCTZHIGH: c_uint = 0x10;
pub const CAP_CTL_MISC_SYNCTZOR: c_uint = 0x20;
pub const CAP_CTL_MISC_DISPUSED: c_uint = 0x80;
pub const REG_BANK: c_uint = 0xfa;
pub const REG_BANK_X: c_uint = 0x00;
pub const REG_BANK_Y: c_uint = 0x01;
pub const REG_BANK_W: c_uint = 0x02;
pub const REG_BANK_T: c_uint = 0x03;
pub const REG_BANK_J: c_uint = 0x04;
pub const REG_BANK_K: c_uint = 0x05;
//
// Bus-master
//
pub const BM_VID_ADDR_LOW: c_uint = 0xbc040;
pub const BM_VID_ADDR_HIGH: c_uint = 0xbc044;
pub const BM_ADDRESS_LOW: c_uint = 0xbc080;
pub const BM_ADDRESS_HIGH: c_uint = 0xbc084;
pub const BM_LENGTH: c_uint = 0xbc088;
pub const BM_CONTROL: c_uint = 0xbc08c;
pub const BM_CONTROL_ENABLE: c_uint = 0x01	/* enable transfer			*/;
pub const BM_CONTROL_IRQEN: c_uint = 0x02	/* enable IRQ at end of transfer	*/;
pub const BM_CONTROL_INIT: c_uint = 0x04	/* initialise status & count		*/;
pub const BM_COUNT: c_uint = 0xbc090		/* read-only				*/;
//
// TV registers
//
pub const TV_VBLANK_EVEN_START: c_uint = 0xbe43c;
pub const TV_VBLANK_EVEN_END: c_uint = 0xbe440;
pub const TV_VBLANK_ODD_START: c_uint = 0xbe444;
pub const TV_VBLANK_ODD_END: c_uint = 0xbe448;
pub const TV_SYNC_YGAIN: c_uint = 0xbe44c;
pub const TV_UV_GAIN: c_uint = 0xbe450;
pub const TV_PED_UVDET: c_uint = 0xbe454;
pub const TV_UV_BURST_AMP: c_uint = 0xbe458;
pub const TV_HSYNC_START: c_uint = 0xbe45c;
pub const TV_HSYNC_END: c_uint = 0xbe460;
pub const TV_Y_DELAY1: c_uint = 0xbe464;
pub const TV_Y_DELAY2: c_uint = 0xbe468;
pub const TV_UV_DELAY1: c_uint = 0xbe46c;
pub const TV_BURST_START: c_uint = 0xbe470;
pub const TV_BURST_END: c_uint = 0xbe474;
pub const TV_HBLANK_START: c_uint = 0xbe478;
pub const TV_HBLANK_END: c_uint = 0xbe47c;
pub const TV_PED_EVEN_START: c_uint = 0xbe480;
pub const TV_PED_EVEN_END: c_uint = 0xbe484;
pub const TV_PED_ODD_START: c_uint = 0xbe488;
pub const TV_PED_ODD_END: c_uint = 0xbe48c;
pub const TV_VSYNC_EVEN_START: c_uint = 0xbe490;
pub const TV_VSYNC_EVEN_END: c_uint = 0xbe494;
pub const TV_VSYNC_ODD_START: c_uint = 0xbe498;
pub const TV_VSYNC_ODD_END: c_uint = 0xbe49c;
pub const TV_SCFL: c_uint = 0xbe4a0;
pub const TV_SCFH: c_uint = 0xbe4a4;
pub const TV_SCP: c_uint = 0xbe4a8;
pub const TV_DELAYBYPASS: c_uint = 0xbe4b4;
pub const TV_EQL_END: c_uint = 0xbe4bc;
pub const TV_SERR_START: c_uint = 0xbe4c0;
pub const TV_SERR_END: c_uint = 0xbe4c4;
pub const TV_CTL: c_uint = 0xbe4dc	/* reflects a previous register- MVFCLR, MVPCLR etc P241*/;
pub const TV_VSYNC_VGA_HS: c_uint = 0xbe4e8;
pub const TV_FLICK_XMIN: c_uint = 0xbe514;
pub const TV_FLICK_XMAX: c_uint = 0xbe518;
pub const TV_FLICK_YMIN: c_uint = 0xbe51c;
pub const TV_FLICK_YMAX: c_uint = 0xbe520;
//
// Graphics Co-processor
//
pub const CO_REG_CONTROL: c_uint = 0xbf011;
pub const CO_CTRL_BUSY: c_uint = 0x80;
pub const CO_CTRL_CMDFULL: c_uint = 0x04;
pub const CO_CTRL_FIFOEMPTY: c_uint = 0x02;
pub const CO_CTRL_READY: c_uint = 0x01;
pub const CO_REG_SRC_WIDTH: c_uint = 0xbf018;
pub const CO_REG_PIXFMT: c_uint = 0xbf01c;
pub const CO_PIXFMT_32BPP: c_uint = 0x03;
pub const CO_PIXFMT_24BPP: c_uint = 0x02;
pub const CO_PIXFMT_16BPP: c_uint = 0x01;
pub const CO_PIXFMT_8BPP: c_uint = 0x00;
pub const CO_REG_FGMIX: c_uint = 0xbf048;
pub const CO_FG_MIX_ZERO: c_uint = 0x00;
pub const CO_FG_MIX_SRC_AND_DST: c_uint = 0x01;
pub const CO_FG_MIX_SRC_AND_NDST: c_uint = 0x02;
pub const CO_FG_MIX_SRC: c_uint = 0x03;
pub const CO_FG_MIX_NSRC_AND_DST: c_uint = 0x04;
pub const CO_FG_MIX_DST: c_uint = 0x05;
pub const CO_FG_MIX_SRC_XOR_DST: c_uint = 0x06;
pub const CO_FG_MIX_SRC_OR_DST: c_uint = 0x07;
pub const CO_FG_MIX_NSRC_AND_NDST: c_uint = 0x08;
pub const CO_FG_MIX_SRC_XOR_NDST: c_uint = 0x09;
pub const CO_FG_MIX_NDST: c_uint = 0x0a;
pub const CO_FG_MIX_SRC_OR_NDST: c_uint = 0x0b;
pub const CO_FG_MIX_NSRC: c_uint = 0x0c;
pub const CO_FG_MIX_NSRC_OR_DST: c_uint = 0x0d;
pub const CO_FG_MIX_NSRC_OR_NDST: c_uint = 0x0e;
pub const CO_FG_MIX_ONES: c_uint = 0x0f;
pub const CO_REG_FGCOLOUR: c_uint = 0xbf058;
pub const CO_REG_BGCOLOUR: c_uint = 0xbf05c;
pub const CO_REG_PIXWIDTH: c_uint = 0xbf060;
pub const CO_REG_PIXHEIGHT: c_uint = 0xbf062;
pub const CO_REG_X_PHASE: c_uint = 0xbf078;
pub const CO_REG_CMD_L: c_uint = 0xbf07c;
pub const CO_CMD_L_PATTERN_FGCOL: c_uint = 0x8000;
pub const CO_CMD_L_INC_LEFT: c_uint = 0x0004;
pub const CO_CMD_L_INC_UP: c_uint = 0x0002;
pub const CO_REG_CMD_H: c_uint = 0xbf07e;
pub const CO_CMD_H_BGSRCMAP: c_uint = 0x8000	/* otherwise bg colour */;
pub const CO_CMD_H_FGSRCMAP: c_uint = 0x2000	/* otherwise fg colour */;
pub const CO_CMD_H_BLITTER: c_uint = 0x0800;
pub const CO_REG_SRC1_PTR: c_uint = 0xbf170;
pub const CO_REG_SRC2_PTR: c_uint = 0xbf174;
pub const CO_REG_DEST_PTR: c_uint = 0xbf178;
pub const CO_REG_DEST_WIDTH: c_uint = 0xbf218;
//
// Private structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyberpro_info {
    pub dev: *mut device,
    pub i2c: *mut i2c_adapter,
    pub regs: *mut unsigned char __iomem,
    pub fb: *mut char __iomem,
    pub dev_name: [c_char; 32],
    pub fb_size: c_uint,
    pub chip_id: c_uint,
    pub irq: c_uint,
//
// The following is a pointer to be passed into the
// functions below.  The modules outside the main
// cyber2000fb.c driver have no knowledge as to what
// is within this structure.
//
    pub info: *mut cfb_info,
}

pub const ID_IGA_1682: c_int = 0;
pub const ID_CYBERPRO_2000: c_int = 1;
pub const ID_CYBERPRO_2010: c_int = 2;
pub const ID_CYBERPRO_5000: c_int = 3;
//
// Note! Writing to the Cyber20x0 registers from an interrupt
// routine is definitely a bad idea atm.
//
extern "C" {
    pub fn cyber2000fb_enable_extregs(cfb: *mut cfb_info);
}
extern "C" {
    pub fn cyber2000fb_disable_extregs(cfb: *mut cfb_info);
}
