//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/solo6x10/solo6x10-regs.h
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//

// Global 6010 system configuration
pub const SOLO_SYS_CFG: c_uint = 0x0000;
pub const SOLO_SYS_CFG_FOUT_EN: c_uint = 0x00000001;
pub const SOLO_SYS_CFG_PLL_BYPASS: c_uint = 0x00000002;
pub const SOLO_SYS_CFG_PLL_PWDN: c_uint = 0x00000004;

pub const SOLO_SYS_CFG_CLOCK_DIV: c_uint = 0x00080000;

pub const SOLO_SYS_CFG_SDRAM64BIT: c_uint = 0x40000000;
pub const SOLO_SYS_CFG_RESET: c_uint = 0x80000000;
pub const SOLO_DMA_CTRL: c_uint = 0x0004;

// 0=16/32MB, 1=32/64MB, 2=64/128MB, 3=128/256MB

// Some things we set in this are undocumented. Why Softlogic?!?!
pub const SOLO_DMA_CTRL1: c_uint = 0x0008;
pub const SOLO_SYS_VCLK: c_uint = 0x000C;

// 0=sys_clk/4, 1=sys_clk/2, 2=clk_in/2 of system input

pub const SOLO_IRQ_STAT: c_uint = 0x0010;
pub const SOLO_IRQ_MASK: c_uint = 0x0014;

pub const SOLO_CHIP_OPTION: c_uint = 0x001C;
pub const SOLO_CHIP_ID_MASK: c_uint = 0x00000007;
pub const SOLO_PLL_CONFIG: c_uint = 0x0020 /* 6110 Only */;
pub const SOLO_EEPROM_CTRL: c_uint = 0x0060;

pub const SOLO_PCI_ERR: c_uint = 0x0070;
pub const SOLO_PCI_ERR_FATAL: c_uint = 0x00000001;
pub const SOLO_PCI_ERR_PARITY: c_uint = 0x00000002;
pub const SOLO_PCI_ERR_TARGET: c_uint = 0x00000004;
pub const SOLO_PCI_ERR_TIMEOUT: c_uint = 0x00000008;
pub const SOLO_PCI_ERR_P2M: c_uint = 0x00000010;
pub const SOLO_PCI_ERR_ATA: c_uint = 0x00000020;
pub const SOLO_PCI_ERR_P2M_DESC: c_uint = 0x00000040;

pub const SOLO_P2M_BASE: c_uint = 0x0080;

// 0:r=[14:10] g=[9:5] b=[4:0], 1:r=[15:11] g=[10:5] b=[4:0]

// 0:512, 1:256, 2:128, 3:64, 4:32, 5:128(2page)

pub const SOLO_P2M_BURST_512: c_int = 0;
pub const SOLO_P2M_BURST_256: c_int = 1;
pub const SOLO_P2M_BURST_128: c_int = 2;
pub const SOLO_P2M_BURST_64: c_int = 3;
pub const SOLO_P2M_BURST_32: c_int = 4;

// 0:Y[0]<-0(OFF), 1:Y[0]<-1(ON), 2:Y[0]<-G[0], 3:Y[0]<-Bit[15]

pub const SOLO_VI_CH_SWITCH_0: c_uint = 0x0100;
pub const SOLO_VI_CH_SWITCH_1: c_uint = 0x0104;
pub const SOLO_VI_CH_SWITCH_2: c_uint = 0x0108;
pub const SOLO_VI_CH_ENA: c_uint = 0x010C;
pub const SOLO_VI_CH_FORMAT: c_uint = 0x0110;

pub const SOLO_VI_FMT_CFG: c_uint = 0x0114;

pub const SOLO_VI_PAGE_SW: c_uint = 0x0118;

pub const SOLO_VI_ACT_I_P: c_uint = 0x011C;
pub const SOLO_VI_ACT_I_S: c_uint = 0x0120;
pub const SOLO_VI_ACT_P: c_uint = 0x0124;

pub const SOLO_VI_STATUS0: c_uint = 0x0128;

pub const SOLO_VI_STATUS1: c_uint = 0x012C;
// XXX: Might be better off in kernel level disp.h

pub const SOLO_VI_PB_CONFIG: c_uint = 0x0130;

pub const SOLO_VI_PB_RANGE_HV: c_uint = 0x0134;

pub const SOLO_VI_PB_ACT_H: c_uint = 0x0138;

pub const SOLO_VI_PB_ACT_V: c_uint = 0x013C;

pub const SOLO_VI_WIN_SW: c_uint = 0x0240;
pub const SOLO_VI_WIN_LIVE_AUTO_MUTE: c_uint = 0x0244;
pub const SOLO_VI_MOT_ADR: c_uint = 0x0260;

pub const SOLO_VI_MOT_CTRL: c_uint = 0x0264;

pub const SOLO_VI_MOT_CLEAR: c_uint = 0x0268;
pub const SOLO_VI_MOT_STATUS: c_uint = 0x026C;

pub const SOLO_VI_MOTION_BORDER: c_uint = 0x0270;
pub const SOLO_VI_MOTION_BAR: c_uint = 0x0274;

pub const SOLO_VO_FMT_ENC: c_uint = 0x0300;

pub const SOLO_VO_FMT_TYPE_NTSC: c_int = 0;

pub const SOLO_VO_ACT_H: c_uint = 0x0304;

pub const SOLO_VO_ACT_V: c_uint = 0x0308;

pub const SOLO_VO_RANGE_HV: c_uint = 0x030C;

pub const SOLO_VO_DISP_CTRL: c_uint = 0x0310;

pub const SOLO_VO_DISP_ERASE: c_uint = 0x0314;

pub const SOLO_VO_ZOOM_CTRL: c_uint = 0x0318;

pub const SOLO_VO_FREEZE_CTRL: c_uint = 0x031C;

pub const SOLO_VO_BKG_COLOR: c_uint = 0x0320;

pub const SOLO_VO_DEINTERLACE: c_uint = 0x0324;

pub const SOLO_VO_BORDER_LINE_COLOR: c_uint = 0x0330;
pub const SOLO_VO_BORDER_FILL_COLOR: c_uint = 0x0334;
pub const SOLO_VO_BORDER_LINE_MASK: c_uint = 0x0338;
pub const SOLO_VO_BORDER_FILL_MASK: c_uint = 0x033c;

pub const SOLO_VO_CELL_EXT_SET: c_uint = 0x0368;
pub const SOLO_VO_CELL_EXT_START: c_uint = 0x036c;
pub const SOLO_VO_CELL_EXT_STOP: c_uint = 0x0370;
pub const SOLO_VO_CELL_EXT_SET2: c_uint = 0x0374;
pub const SOLO_VO_CELL_EXT_START2: c_uint = 0x0378;
pub const SOLO_VO_CELL_EXT_STOP2: c_uint = 0x037c;

pub const SOLO_OSG_CONFIG: c_uint = 0x03E0;

pub const SOLO_OSG_ERASE: c_uint = 0x03E4;

pub const SOLO_VO_OSG_BLINK: c_uint = 0x03E8;

pub const SOLO_CAP_BASE: c_uint = 0x0400;

pub const SOLO_CAP_BTW: c_uint = 0x0404;

pub const SOLO_DIM_SCALE1: c_uint = 0x0408;
pub const SOLO_DIM_SCALE2: c_uint = 0x040C;
pub const SOLO_DIM_SCALE3: c_uint = 0x0410;
pub const SOLO_DIM_SCALE4: c_uint = 0x0414;
pub const SOLO_DIM_SCALE5: c_uint = 0x0418;

pub const SOLO_DIM_PROG: c_uint = 0x041C;
pub const SOLO_CAP_STATUS: c_uint = 0x0420;

pub const SOLO_VE_CFG0: c_uint = 0x0610;

pub const SOLO_VE_CFG1: c_uint = 0x0614;

pub const SOLO_VE_WMRK_POLY: c_uint = 0x061C;
pub const SOLO_VE_VMRK_INIT_KEY: c_uint = 0x0620;
pub const SOLO_VE_WMRK_STRL: c_uint = 0x0624;
pub const SOLO_VE_ENCRYP_POLY: c_uint = 0x0628;
pub const SOLO_VE_ENCRYP_INIT: c_uint = 0x062C;
pub const SOLO_VE_ATTR: c_uint = 0x0630;

pub const SOLO_VE_COMPT_MOT: c_uint = 0x0634 /* 6110 Only */;

pub const SOLO_VE_JPEG_QP_TBL: c_uint = 0x0670;
pub const SOLO_VE_JPEG_QP_CH_L: c_uint = 0x0674;
pub const SOLO_VE_JPEG_QP_CH_H: c_uint = 0x0678;
pub const SOLO_VE_JPEG_CFG: c_uint = 0x067C;
pub const SOLO_VE_JPEG_CTRL: c_uint = 0x0680;
pub const SOLO_VE_CODE_ENCRYPT: c_uint = 0x0684 /* 6110 Only */;
pub const SOLO_VE_JPEG_CFG1: c_uint = 0x0688 /* 6110 Only */;
pub const SOLO_VE_WMRK_ENABLE: c_uint = 0x068C /* 6110 Only */;
pub const SOLO_VE_OSD_CH: c_uint = 0x0690;
pub const SOLO_VE_OSD_BASE: c_uint = 0x0694;
pub const SOLO_VE_OSD_CLR: c_uint = 0x0698;
pub const SOLO_VE_OSD_OPT: c_uint = 0x069C;

pub const SOLO_VD_CFG0: c_uint = 0x0900;

pub const SOLO_VD_CFG1: c_uint = 0x0904;
pub const SOLO_VD_DEINTERLACE: c_uint = 0x0908;

pub const SOLO_VD_CODE_ADR: c_uint = 0x090C;
pub const SOLO_VD_CTRL: c_uint = 0x0910;

pub const SOLO_VD_STATUS0: c_uint = 0x0920;

pub const SOLO_VD_STATUS1: c_uint = 0x0924;
pub const SOLO_VD_IDX0: c_uint = 0x0930;

pub const SOLO_VD_IDX1: c_uint = 0x0934;

pub const SOLO_VD_IDX2: c_uint = 0x0938;

pub const SOLO_VD_IDX3: c_uint = 0x093C;

pub const SOLO_VD_IDX4: c_uint = 0x0940;

pub const SOLO_GPIO_CONFIG_0: c_uint = 0x0B00;
pub const SOLO_GPIO_CONFIG_1: c_uint = 0x0B04;
pub const SOLO_GPIO_DATA_OUT: c_uint = 0x0B08;
pub const SOLO_GPIO_DATA_IN: c_uint = 0x0B0C;
pub const SOLO_GPIO_INT_ACK_STA: c_uint = 0x0B10;
pub const SOLO_GPIO_INT_ENA: c_uint = 0x0B14;
pub const SOLO_GPIO_INT_CFG_0: c_uint = 0x0B18;
pub const SOLO_GPIO_INT_CFG_1: c_uint = 0x0B1C;
pub const SOLO_IIC_CFG: c_uint = 0x0B20;

pub const SOLO_IIC_CTRL: c_uint = 0x0B24;

pub const SOLO_IIC_TXD: c_uint = 0x0B28;
pub const SOLO_IIC_RXD: c_uint = 0x0B2C;
//
// UART REGISTER
//

pub const SOLO_UART_RX_BUFF_SIZE: c_int = 8;

pub const SOLO_UART_TX_BUFF_SIZE: c_int = 8;

pub const SOLO_TIMER_CLOCK_NUM: c_uint = 0x0be0;
pub const SOLO_TIMER_USEC: c_uint = 0x0be8;
pub const SOLO_TIMER_SEC: c_uint = 0x0bec;
pub const SOLO_TIMER_USEC_LSB: c_uint = 0x0d20 /* 6110 Only */;
pub const SOLO_AUDIO_CONTROL: c_uint = 0x0D00;

pub const SOLO_AUDIO_SAMPLE: c_uint = 0x0D04;

pub const SOLO_AUDIO_FDMA_INTR: c_uint = 0x0D08;

pub const SOLO_AUDIO_EVOL_0: c_uint = 0x0D0C;
pub const SOLO_AUDIO_EVOL_1: c_uint = 0x0D10;

pub const SOLO_AUDIO_STA: c_uint = 0x0D14;
//
// Watchdog configuration
//
pub const SOLO_WATCHDOG: c_uint = 0x0be4;

