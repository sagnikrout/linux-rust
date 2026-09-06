//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/s1d13xxxfb.h
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


// include/video/s1d13xxxfb.h
//
// (c) 2004 Simtec Electronics
// (c) 2005 Thibaut VARENE <varenet@parisc-linux.org>
//
// Header file for Epson S1D13XXX driver code
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
pub const S1D_PALETTE_SIZE: c_int = 256;

// S1DREG_REV_CODE register = prod_id (6 bits) + revision (2 bits)
pub const S1D13505_PROD_ID: c_uint = 0x3	/* 000011 */;
pub const S1D13506_PROD_ID: c_uint = 0x4	/* 000100 */;
pub const S1D13806_PROD_ID: c_uint = 0x7	/* 000111 */;
// register definitions (tested on s1d13896)
pub const S1DREG_REV_CODE: c_uint = 0x0000	/* Prod + Rev Code Register */;
pub const S1DREG_MISC: c_uint = 0x0001	/* Miscellaneous Register */;
pub const S1DREG_GPIO_CNF0: c_uint = 0x0004	/* General IO Pins Configuration Register 0 */;
pub const S1DREG_GPIO_CNF1: c_uint = 0x0005	/* General IO Pins Configuration Register 1 */;
pub const S1DREG_GPIO_CTL0: c_uint = 0x0008	/* General IO Pins Control Register 0 */;
pub const S1DREG_GPIO_CTL1: c_uint = 0x0009	/* General IO Pins Control Register 1 */;
pub const S1DREG_CNF_STATUS: c_uint = 0x000C	/* Configuration Status Readback Register */;
pub const S1DREG_CLK_CNF: c_uint = 0x0010	/* Memory Clock Configuration Register */;
pub const S1DREG_LCD_CLK_CNF: c_uint = 0x0014	/* LCD Pixel Clock Configuration Register */;
pub const S1DREG_CRT_CLK_CNF: c_uint = 0x0018	/* CRT/TV Pixel Clock Configuration Register */;
pub const S1DREG_MPLUG_CLK_CNF: c_uint = 0x001C	/* MediaPlug Clock Configuration Register */;
pub const S1DREG_CPU2MEM_WST_SEL: c_uint = 0x001E	/* CPU To Memory Wait State Select Register */;
pub const S1DREG_MEM_CNF: c_uint = 0x0020	/* Memory Configuration Register */;
pub const S1DREG_SDRAM_REF_RATE: c_uint = 0x0021	/* SDRAM Refresh Rate Register */;
pub const S1DREG_SDRAM_TC0: c_uint = 0x002A	/* SDRAM Timing Control Register 0 */;
pub const S1DREG_SDRAM_TC1: c_uint = 0x002B	/* SDRAM Timing Control Register 1 */;
pub const S1DREG_PANEL_TYPE: c_uint = 0x0030	/* Panel Type Register */;
pub const S1DREG_MOD_RATE: c_uint = 0x0031	/* MOD Rate Register */;
pub const S1DREG_LCD_DISP_HWIDTH: c_uint = 0x0032	/* LCD Horizontal Display Width Register: ((val)+1)*8)=pix/line */;
pub const S1DREG_LCD_NDISP_HPER: c_uint = 0x0034	/* LCD Horizontal Non-Display Period Register: ((val)+1)*8)=NDpix/line */;
pub const S1DREG_TFT_FPLINE_START: c_uint = 0x0035	/* TFT FPLINE Start Position Register */;
pub const S1DREG_TFT_FPLINE_PWIDTH: c_uint = 0x0036	/* TFT FPLINE Pulse Width Register. */;
pub const S1DREG_LCD_DISP_VHEIGHT0: c_uint = 0x0038	/* LCD Vertical Display Height Register 0 */;
pub const S1DREG_LCD_DISP_VHEIGHT1: c_uint = 0x0039	/* LCD Vertical Display Height Register 1 */;
pub const S1DREG_LCD_NDISP_VPER: c_uint = 0x003A	/* LCD Vertical Non-Display Period Register: (val)+1=NDlines */;
pub const S1DREG_TFT_FPFRAME_START: c_uint = 0x003B	/* TFT FPFRAME Start Position Register */;
pub const S1DREG_TFT_FPFRAME_PWIDTH: c_uint = 0x003C	/* TFT FPFRAME Pulse Width Register */;
pub const S1DREG_LCD_DISP_MODE: c_uint = 0x0040	/* LCD Display Mode Register */;
pub const S1DREG_LCD_MISC: c_uint = 0x0041	/* LCD Miscellaneous Register */;
pub const S1DREG_LCD_DISP_START0: c_uint = 0x0042	/* LCD Display Start Address Register 0 */;
pub const S1DREG_LCD_DISP_START1: c_uint = 0x0043	/* LCD Display Start Address Register 1 */;
pub const S1DREG_LCD_DISP_START2: c_uint = 0x0044	/* LCD Display Start Address Register 2 */;
pub const S1DREG_LCD_MEM_OFF0: c_uint = 0x0046	/* LCD Memory Address Offset Register 0 */;
pub const S1DREG_LCD_MEM_OFF1: c_uint = 0x0047	/* LCD Memory Address Offset Register 1 */;
pub const S1DREG_LCD_PIX_PAN: c_uint = 0x0048	/* LCD Pixel Panning Register */;
pub const S1DREG_LCD_DISP_FIFO_HTC: c_uint = 0x004A	/* LCD Display FIFO High Threshold Control Register */;
pub const S1DREG_LCD_DISP_FIFO_LTC: c_uint = 0x004B	/* LCD Display FIFO Low Threshold Control Register */;
pub const S1DREG_CRT_DISP_HWIDTH: c_uint = 0x0050	/* CRT/TV Horizontal Display Width Register: ((val)+1)*8)=pix/line */;
pub const S1DREG_CRT_NDISP_HPER: c_uint = 0x0052	/* CRT/TV Horizontal Non-Display Period Register */;
pub const S1DREG_CRT_HRTC_START: c_uint = 0x0053	/* CRT/TV HRTC Start Position Register */;
pub const S1DREG_CRT_HRTC_PWIDTH: c_uint = 0x0054	/* CRT/TV HRTC Pulse Width Register */;
pub const S1DREG_CRT_DISP_VHEIGHT0: c_uint = 0x0056	/* CRT/TV Vertical Display Height Register 0 */;
pub const S1DREG_CRT_DISP_VHEIGHT1: c_uint = 0x0057	/* CRT/TV Vertical Display Height Register 1 */;
pub const S1DREG_CRT_NDISP_VPER: c_uint = 0x0058	/* CRT/TV Vertical Non-Display Period Register */;
pub const S1DREG_CRT_VRTC_START: c_uint = 0x0059	/* CRT/TV VRTC Start Position Register */;
pub const S1DREG_CRT_VRTC_PWIDTH: c_uint = 0x005A	/* CRT/TV VRTC Pulse Width Register */;
pub const S1DREG_TV_OUT_CTL: c_uint = 0x005B	/* TV Output Control Register */;
pub const S1DREG_CRT_DISP_MODE: c_uint = 0x0060	/* CRT/TV Display Mode Register */;
pub const S1DREG_CRT_DISP_START0: c_uint = 0x0062	/* CRT/TV Display Start Address Register 0 */;
pub const S1DREG_CRT_DISP_START1: c_uint = 0x0063	/* CRT/TV Display Start Address Register 1 */;
pub const S1DREG_CRT_DISP_START2: c_uint = 0x0064	/* CRT/TV Display Start Address Register 2 */;
pub const S1DREG_CRT_MEM_OFF0: c_uint = 0x0066	/* CRT/TV Memory Address Offset Register 0 */;
pub const S1DREG_CRT_MEM_OFF1: c_uint = 0x0067	/* CRT/TV Memory Address Offset Register 1 */;
pub const S1DREG_CRT_PIX_PAN: c_uint = 0x0068	/* CRT/TV Pixel Panning Register */;
pub const S1DREG_CRT_DISP_FIFO_HTC: c_uint = 0x006A	/* CRT/TV Display FIFO High Threshold Control Register */;
pub const S1DREG_CRT_DISP_FIFO_LTC: c_uint = 0x006B	/* CRT/TV Display FIFO Low Threshold Control Register */;
pub const S1DREG_LCD_CUR_CTL: c_uint = 0x0070	/* LCD Ink/Cursor Control Register */;
pub const S1DREG_LCD_CUR_START: c_uint = 0x0071	/* LCD Ink/Cursor Start Address Register */;
pub const S1DREG_LCD_CUR_XPOS0: c_uint = 0x0072	/* LCD Cursor X Position Register 0 */;
pub const S1DREG_LCD_CUR_XPOS1: c_uint = 0x0073	/* LCD Cursor X Position Register 1 */;
pub const S1DREG_LCD_CUR_YPOS0: c_uint = 0x0074	/* LCD Cursor Y Position Register 0 */;
pub const S1DREG_LCD_CUR_YPOS1: c_uint = 0x0075	/* LCD Cursor Y Position Register 1 */;
pub const S1DREG_LCD_CUR_BCTL0: c_uint = 0x0076	/* LCD Ink/Cursor Blue Color 0 Register */;
pub const S1DREG_LCD_CUR_GCTL0: c_uint = 0x0077	/* LCD Ink/Cursor Green Color 0 Register */;
pub const S1DREG_LCD_CUR_RCTL0: c_uint = 0x0078	/* LCD Ink/Cursor Red Color 0 Register */;
pub const S1DREG_LCD_CUR_BCTL1: c_uint = 0x007A	/* LCD Ink/Cursor Blue Color 1 Register */;
pub const S1DREG_LCD_CUR_GCTL1: c_uint = 0x007B	/* LCD Ink/Cursor Green Color 1 Register */;
pub const S1DREG_LCD_CUR_RCTL1: c_uint = 0x007C	/* LCD Ink/Cursor Red Color 1 Register */;
pub const S1DREG_LCD_CUR_FIFO_HTC: c_uint = 0x007E	/* LCD Ink/Cursor FIFO High Threshold Register */;
pub const S1DREG_CRT_CUR_CTL: c_uint = 0x0080	/* CRT/TV Ink/Cursor Control Register */;
pub const S1DREG_CRT_CUR_START: c_uint = 0x0081	/* CRT/TV Ink/Cursor Start Address Register */;
pub const S1DREG_CRT_CUR_XPOS0: c_uint = 0x0082	/* CRT/TV Cursor X Position Register 0 */;
pub const S1DREG_CRT_CUR_XPOS1: c_uint = 0x0083	/* CRT/TV Cursor X Position Register 1 */;
pub const S1DREG_CRT_CUR_YPOS0: c_uint = 0x0084	/* CRT/TV Cursor Y Position Register 0 */;
pub const S1DREG_CRT_CUR_YPOS1: c_uint = 0x0085	/* CRT/TV Cursor Y Position Register 1 */;
pub const S1DREG_CRT_CUR_BCTL0: c_uint = 0x0086	/* CRT/TV Ink/Cursor Blue Color 0 Register */;
pub const S1DREG_CRT_CUR_GCTL0: c_uint = 0x0087	/* CRT/TV Ink/Cursor Green Color 0 Register */;
pub const S1DREG_CRT_CUR_RCTL0: c_uint = 0x0088	/* CRT/TV Ink/Cursor Red Color 0 Register */;
pub const S1DREG_CRT_CUR_BCTL1: c_uint = 0x008A	/* CRT/TV Ink/Cursor Blue Color 1 Register */;
pub const S1DREG_CRT_CUR_GCTL1: c_uint = 0x008B	/* CRT/TV Ink/Cursor Green Color 1 Register */;
pub const S1DREG_CRT_CUR_RCTL1: c_uint = 0x008C	/* CRT/TV Ink/Cursor Red Color 1 Register */;
pub const S1DREG_CRT_CUR_FIFO_HTC: c_uint = 0x008E	/* CRT/TV Ink/Cursor FIFO High Threshold Register */;
pub const S1DREG_BBLT_CTL0: c_uint = 0x0100	/* BitBLT Control Register 0 */;
pub const S1DREG_BBLT_CTL1: c_uint = 0x0101	/* BitBLT Control Register 1 */;
pub const S1DREG_BBLT_CC_EXP: c_uint = 0x0102	/* BitBLT Code/Color Expansion Register */;
pub const S1DREG_BBLT_OP: c_uint = 0x0103	/* BitBLT Operation Register */;
pub const S1DREG_BBLT_SRC_START0: c_uint = 0x0104	/* BitBLT Source Start Address Register 0 */;
pub const S1DREG_BBLT_SRC_START1: c_uint = 0x0105	/* BitBLT Source Start Address Register 1 */;
pub const S1DREG_BBLT_SRC_START2: c_uint = 0x0106	/* BitBLT Source Start Address Register 2 */;
pub const S1DREG_BBLT_DST_START0: c_uint = 0x0108	/* BitBLT Destination Start Address Register 0 */;
pub const S1DREG_BBLT_DST_START1: c_uint = 0x0109	/* BitBLT Destination Start Address Register 1 */;
pub const S1DREG_BBLT_DST_START2: c_uint = 0x010A	/* BitBLT Destination Start Address Register 2 */;
pub const S1DREG_BBLT_MEM_OFF0: c_uint = 0x010C	/* BitBLT Memory Address Offset Register 0 */;
pub const S1DREG_BBLT_MEM_OFF1: c_uint = 0x010D	/* BitBLT Memory Address Offset Register 1 */;
pub const S1DREG_BBLT_WIDTH0: c_uint = 0x0110	/* BitBLT Width Register 0 */;
pub const S1DREG_BBLT_WIDTH1: c_uint = 0x0111	/* BitBLT Width Register 1 */;
pub const S1DREG_BBLT_HEIGHT0: c_uint = 0x0112	/* BitBLT Height Register 0 */;
pub const S1DREG_BBLT_HEIGHT1: c_uint = 0x0113	/* BitBLT Height Register 1 */;
pub const S1DREG_BBLT_BGC0: c_uint = 0x0114	/* BitBLT Background Color Register 0 */;
pub const S1DREG_BBLT_BGC1: c_uint = 0x0115	/* BitBLT Background Color Register 1 */;
pub const S1DREG_BBLT_FGC0: c_uint = 0x0118	/* BitBLT Foreground Color Register 0 */;
pub const S1DREG_BBLT_FGC1: c_uint = 0x0119	/* BitBLT Foreground Color Register 1 */;
pub const S1DREG_LKUP_MODE: c_uint = 0x01E0	/* Look-Up Table Mode Register */;
pub const S1DREG_LKUP_ADDR: c_uint = 0x01E2	/* Look-Up Table Address Register */;
pub const S1DREG_LKUP_DATA: c_uint = 0x01E4	/* Look-Up Table Data Register */;
pub const S1DREG_PS_CNF: c_uint = 0x01F0	/* Power Save Configuration Register */;
pub const S1DREG_PS_STATUS: c_uint = 0x01F1	/* Power Save Status Register */;
pub const S1DREG_CPU2MEM_WDOGT: c_uint = 0x01F4	/* CPU-to-Memory Access Watchdog Timer Register */;
pub const S1DREG_COM_DISP_MODE: c_uint = 0x01FC	/* Common Display Mode Register */;
pub const S1DREG_DELAYOFF: c_uint = 0xFFFE;
pub const S1DREG_DELAYON: c_uint = 0xFFFF;
pub const BBLT_SOLID_FILL: c_uint = 0x0c;
// Note: all above defines should go in separate header files
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1d13xxxfb_regval {
    pub addr: u16,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1d13xxxfb_par {
    pub regs: *mut void __iomem,
    pub display: c_uchar,
    pub prod_id: c_uchar,
    pub revision: c_uchar,
    pub pseudo_palette: [c_uint; 16],
    pub /: *mut *mut *mut void regs_save; / pm saves all registers here,
    pub /: *mut *mut *mut void disp_save; / pm saves entire screen here,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1d13xxxfb_pdata {
    pub initregs: *const s1d13xxxfb_regval,
    pub initregssize: c_uint,
    pub (*platform_init_video)(void): *mut c_void,

    pub (*platform_suspend_video)(void): *mut c_int,
    pub (*platform_resume_video)(void): *mut c_int,

}
