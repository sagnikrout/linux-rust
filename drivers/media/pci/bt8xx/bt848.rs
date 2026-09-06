//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/bt848.h
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

pub const PCI_VENDOR_ID_BROOKTREE: c_uint = 0x109e;

pub const PCI_DEVICE_ID_BT848: c_uint = 0x350;

pub const PCI_DEVICE_ID_BT849: c_uint = 0x351;

pub const PCI_DEVICE_ID_FUSION879: c_uint = 0x36c;

pub const PCI_DEVICE_ID_BT878: c_uint = 0x36e;

pub const PCI_DEVICE_ID_BT879: c_uint = 0x36f;

// Brooktree 848 registers
pub const BT848_DSTATUS: c_uint = 0x000;

pub const BT848_IFORM: c_uint = 0x004;

pub const BT848_IFORM_NTSC: c_int = 1;
pub const BT848_IFORM_NTSC_J: c_int = 2;
pub const BT848_IFORM_PAL_BDGHI: c_int = 3;
pub const BT848_IFORM_PAL_M: c_int = 4;
pub const BT848_IFORM_PAL_N: c_int = 5;
pub const BT848_IFORM_SECAM: c_int = 6;
pub const BT848_IFORM_PAL_NC: c_int = 7;
pub const BT848_IFORM_AUTO: c_int = 0;
pub const BT848_IFORM_NORM: c_int = 7;
pub const BT848_TDEC: c_uint = 0x008;

pub const BT848_E_CROP: c_uint = 0x00C;
pub const BT848_O_CROP: c_uint = 0x08C;
pub const BT848_E_VDELAY_LO: c_uint = 0x010;
pub const BT848_O_VDELAY_LO: c_uint = 0x090;
pub const BT848_E_VACTIVE_LO: c_uint = 0x014;
pub const BT848_O_VACTIVE_LO: c_uint = 0x094;
pub const BT848_E_HDELAY_LO: c_uint = 0x018;
pub const BT848_O_HDELAY_LO: c_uint = 0x098;
pub const BT848_E_HACTIVE_LO: c_uint = 0x01C;
pub const BT848_O_HACTIVE_LO: c_uint = 0x09C;
pub const BT848_E_HSCALE_HI: c_uint = 0x020;
pub const BT848_O_HSCALE_HI: c_uint = 0x0A0;
pub const BT848_E_HSCALE_LO: c_uint = 0x024;
pub const BT848_O_HSCALE_LO: c_uint = 0x0A4;
pub const BT848_BRIGHT: c_uint = 0x028;
pub const BT848_E_CONTROL: c_uint = 0x02C;
pub const BT848_O_CONTROL: c_uint = 0x0AC;

pub const BT848_CONTRAST_LO: c_uint = 0x030;
pub const BT848_SAT_U_LO: c_uint = 0x034;
pub const BT848_SAT_V_LO: c_uint = 0x038;
pub const BT848_HUE: c_uint = 0x03C;
pub const BT848_E_SCLOOP: c_uint = 0x040;
pub const BT848_O_SCLOOP: c_uint = 0x0C0;

pub const BT848_OFORM: c_uint = 0x048;

pub const BT848_E_VSCALE_HI: c_uint = 0x04C;
pub const BT848_O_VSCALE_HI: c_uint = 0x0CC;

pub const BT848_VSCALE_HI: c_int = 15;
pub const BT848_E_VSCALE_LO: c_uint = 0x050;
pub const BT848_O_VSCALE_LO: c_uint = 0x0D0;
pub const BT848_TEST: c_uint = 0x054;
pub const BT848_ADELAY: c_uint = 0x060;
pub const BT848_BDELAY: c_uint = 0x064;
pub const BT848_ADC: c_uint = 0x068;

pub const BT848_WC_UP: c_uint = 0x044;
pub const BT848_WC_DOWN: c_uint = 0x078;
pub const BT848_E_VTC: c_uint = 0x06C;
pub const BT848_O_VTC: c_uint = 0x0EC;

pub const BT848_VTC_VFILT_2TAP: c_int = 0;
pub const BT848_VTC_VFILT_3TAP: c_int = 1;
pub const BT848_VTC_VFILT_4TAP: c_int = 2;
pub const BT848_VTC_VFILT_5TAP: c_int = 3;
pub const BT848_SRESET: c_uint = 0x07C;
pub const BT848_COLOR_FMT: c_uint = 0x0D4;

pub const BT848_COLOR_FMT_E_RGB32: c_int = 0;
pub const BT848_COLOR_FMT_E_RGB24: c_int = 1;
pub const BT848_COLOR_FMT_E_RGB16: c_int = 2;
pub const BT848_COLOR_FMT_E_RGB15: c_int = 3;
pub const BT848_COLOR_FMT_E_YUY2: c_int = 4;
pub const BT848_COLOR_FMT_E_BtYUV: c_int = 5;
pub const BT848_COLOR_FMT_E_Y8: c_int = 6;
pub const BT848_COLOR_FMT_E_RGB8: c_int = 7;
pub const BT848_COLOR_FMT_E_YCrCb422: c_int = 8;
pub const BT848_COLOR_FMT_E_YCrCb411: c_int = 9;
pub const BT848_COLOR_FMT_E_RAW: c_int = 14;
pub const BT848_COLOR_FMT_RGB32: c_uint = 0x00;
pub const BT848_COLOR_FMT_RGB24: c_uint = 0x11;
pub const BT848_COLOR_FMT_RGB16: c_uint = 0x22;
pub const BT848_COLOR_FMT_RGB15: c_uint = 0x33;
pub const BT848_COLOR_FMT_YUY2: c_uint = 0x44;
pub const BT848_COLOR_FMT_BtYUV: c_uint = 0x55;
pub const BT848_COLOR_FMT_Y8: c_uint = 0x66;
pub const BT848_COLOR_FMT_RGB8: c_uint = 0x77;
pub const BT848_COLOR_FMT_YCrCb422: c_uint = 0x88;
pub const BT848_COLOR_FMT_YCrCb411: c_uint = 0x99;
pub const BT848_COLOR_FMT_RAW: c_uint = 0xee;
pub const BT848_VTOTAL_LO: c_uint = 0xB0;
pub const BT848_VTOTAL_HI: c_uint = 0xB4;
pub const BT848_COLOR_CTL: c_uint = 0x0D8;

pub const BT848_CAP_CTL: c_uint = 0x0DC;

pub const BT848_VBI_PACK_SIZE: c_uint = 0x0E0;
pub const BT848_VBI_PACK_DEL: c_uint = 0x0E4;
pub const BT848_VBI_PACK_DEL_VBI_HDELAY: c_uint = 0xfc;
pub const BT848_VBI_PACK_DEL_EXT_FRAME: c_int = 2;
pub const BT848_VBI_PACK_DEL_VBI_PKT_HI: c_int = 1;
pub const BT848_INT_STAT: c_uint = 0x100;
pub const BT848_INT_MASK: c_uint = 0x104;

pub const BT848_RISC_VIDEO: c_int = 1;
pub const BT848_RISC_TOP: c_int = 2;
pub const BT848_RISC_VBI: c_int = 4;

pub const BT848_GPIO_DMA_CTL: c_uint = 0x10C;

pub const BT848_I2C: c_uint = 0x110;

pub const BT848_RISC_STRT_ADD: c_uint = 0x114;
pub const BT848_GPIO_OUT_EN: c_uint = 0x118;
pub const BT848_GPIO_REG_INP: c_uint = 0x11C;
pub const BT848_RISC_COUNT: c_uint = 0x120;
pub const BT848_GPIO_DATA: c_uint = 0x200;
// Bt848 RISC commands
// only for the SYNC RISC command
pub const BT848_FIFO_STATUS_FM1: c_uint = 0x06;
pub const BT848_FIFO_STATUS_FM3: c_uint = 0x0e;
pub const BT848_FIFO_STATUS_SOL: c_uint = 0x02;
pub const BT848_FIFO_STATUS_EOL4: c_uint = 0x01;
pub const BT848_FIFO_STATUS_EOL3: c_uint = 0x0d;
pub const BT848_FIFO_STATUS_EOL2: c_uint = 0x09;
pub const BT848_FIFO_STATUS_EOL1: c_uint = 0x05;
pub const BT848_FIFO_STATUS_VRE: c_uint = 0x04;
pub const BT848_FIFO_STATUS_VRO: c_uint = 0x0c;
pub const BT848_FIFO_STATUS_PXV: c_uint = 0x00;

// WRITE and SKIP
// disable which bytes of each DWORD

pub const BT848_RISC_BYTE_NONE: c_int = 0;
// cause RISCI

// RISC command is last one in this line

// RISC command is first one in this line

// Bt848A and higher only !!
pub const BT848_TGLB: c_uint = 0x080;
pub const BT848_TGCTRL: c_uint = 0x084;
pub const BT848_FCAP: c_uint = 0x0E8;
pub const BT848_PLL_F_LO: c_uint = 0x0F0;
pub const BT848_PLL_F_HI: c_uint = 0x0F4;
pub const BT848_PLL_XCI: c_uint = 0x0F8;

pub const BT848_DVSIF: c_uint = 0x0FC;
// Bt878 register
pub const BT878_DEVCTRL: c_uint = 0x40;
pub const BT878_EN_TBFX: c_uint = 0x02;
pub const BT878_EN_VSFX: c_uint = 0x04;
