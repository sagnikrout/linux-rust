//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/loongson/lsdc_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

//
// PIXEL PLL Reference clock
//
pub const LSDC_PLL_REF_CLK_KHZ: c_int = 100000;
//
// Those PLL registers are relative to LSxxxxx_CFG_REG_BASE. xxxxx = 7A1000,
// 7A2000, 2K2000, 2K1000 etc.
//
// LS7A1000
pub const LS7A1000_PIXPLL0_REG: c_uint = 0x04B0;
pub const LS7A1000_PIXPLL1_REG: c_uint = 0x04C0;
// The DC, GPU, Graphic Memory Controller share the single gfxpll
pub const LS7A1000_PLL_GFX_REG: c_uint = 0x0490;
pub const LS7A1000_CONF_REG_BASE: c_uint = 0x10010000;
// LS7A2000
pub const LS7A2000_PIXPLL0_REG: c_uint = 0x04B0;
pub const LS7A2000_PIXPLL1_REG: c_uint = 0x04C0;
// The DC, GPU, Graphic Memory Controller share the single gfxpll
pub const LS7A2000_PLL_GFX_REG: c_uint = 0x0490;
pub const LS7A2000_CONF_REG_BASE: c_uint = 0x10010000;
// For LSDC_CRTCx_CFG_REG

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsdc_pixel_format {
    LSDC_PF_NONE = 0,
    LSDC_PF_XRGB444 = 1,    /* [12 bits] */
    LSDC_PF_XRGB555 = 2,    /* [15 bits] */
    LSDC_PF_XRGB565 = 3,    /* RGB [16 bits] */
    LSDC_PF_XRGB8888 = 4,   /* XRGB [32 bits] */
}

//
// Each crtc has two set fb address registers usable, FB_REG_IN_USING bit of
// LSDC_CRTCx_CFG_REG indicate which fb address register is in using by the
// CRTC currently. CFG_PAGE_FLIP is used to trigger the switch, the switching
// will be finished at the very next vblank. Trigger it again if you want to
// switch back.
//
// If FB0_ADDR_REG is in using, we write the address to FB0_ADDR_REG,
// if FB1_ADDR_REG is in using, we write the address to FB1_ADDR_REG.
//

// Indicate witch fb addr reg is in using, currently. read only

// The DC get soft reset if this bit changed from "1" to "0", active low

// If this bit is set, it say that the CRTC stop working anymore, anchored.

//
// The DMA step of the DC in LS7A2000/LS2K2000 is configurable,
// setting those bits on ls7a1000 platform make no effect.
//

pub const CFG_DMA_STEP_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsdc_dma_steps {
    LSDC_DMA_STEP_256_BYTES = 0,
    LSDC_DMA_STEP_128_BYTES = 1,
    LSDC_DMA_STEP_64_BYTES = 2,
    LSDC_DMA_STEP_32_BYTES = 3,
}

// For LSDC_CRTCx_HSYNC_REG

pub const HSYNC_END_SHIFT: c_int = 16;

pub const HSYNC_START_SHIFT: c_int = 0;
// For LSDC_CRTCx_VSYNC_REG

pub const VSYNC_END_SHIFT: c_int = 16;

pub const VSYNC_START_SHIFT: c_int = 0;
// CRTC0
pub const LSDC_CRTC0_CFG_REG: c_uint = 0x1240;
pub const LSDC_CRTC0_FB0_ADDR_LO_REG: c_uint = 0x1260;
pub const LSDC_CRTC0_FB0_ADDR_HI_REG: c_uint = 0x15A0;
pub const LSDC_CRTC0_STRIDE_REG: c_uint = 0x1280;
pub const LSDC_CRTC0_FB_ORIGIN_REG: c_uint = 0x1300;
pub const LSDC_CRTC0_HDISPLAY_REG: c_uint = 0x1400;
pub const LSDC_CRTC0_HSYNC_REG: c_uint = 0x1420;
pub const LSDC_CRTC0_VDISPLAY_REG: c_uint = 0x1480;
pub const LSDC_CRTC0_VSYNC_REG: c_uint = 0x14A0;
pub const LSDC_CRTC0_GAMMA_INDEX_REG: c_uint = 0x14E0;
pub const LSDC_CRTC0_GAMMA_DATA_REG: c_uint = 0x1500;
pub const LSDC_CRTC0_FB1_ADDR_LO_REG: c_uint = 0x1580;
pub const LSDC_CRTC0_FB1_ADDR_HI_REG: c_uint = 0x15C0;
// CRTC1
pub const LSDC_CRTC1_CFG_REG: c_uint = 0x1250;
pub const LSDC_CRTC1_FB0_ADDR_LO_REG: c_uint = 0x1270;
pub const LSDC_CRTC1_FB0_ADDR_HI_REG: c_uint = 0x15B0;
pub const LSDC_CRTC1_STRIDE_REG: c_uint = 0x1290;
pub const LSDC_CRTC1_FB_ORIGIN_REG: c_uint = 0x1310;
pub const LSDC_CRTC1_HDISPLAY_REG: c_uint = 0x1410;
pub const LSDC_CRTC1_HSYNC_REG: c_uint = 0x1430;
pub const LSDC_CRTC1_VDISPLAY_REG: c_uint = 0x1490;
pub const LSDC_CRTC1_VSYNC_REG: c_uint = 0x14B0;
pub const LSDC_CRTC1_GAMMA_INDEX_REG: c_uint = 0x14F0;
pub const LSDC_CRTC1_GAMMA_DATA_REG: c_uint = 0x1510;
pub const LSDC_CRTC1_FB1_ADDR_LO_REG: c_uint = 0x1590;
pub const LSDC_CRTC1_FB1_ADDR_HI_REG: c_uint = 0x15D0;
// For LSDC_CRTCx_DVO_CONF_REG

// DVO0
pub const LSDC_CRTC0_DVO_CONF_REG: c_uint = 0x13C0;
// DVO1
pub const LSDC_CRTC1_DVO_CONF_REG: c_uint = 0x13D0;
//
// All of the DC variants has the hardware which record the scan position
// of the CRTC, [31:16] : current X position, [15:0] : current Y position
//
pub const LSDC_CRTC0_SCAN_POS_REG: c_uint = 0x14C0;
pub const LSDC_CRTC1_SCAN_POS_REG: c_uint = 0x14D0;
//
// LS7A2000 has Sync Deviation register.
//

pub const LSDC_CRTC0_SYNC_DEVIATION_REG: c_uint = 0x1B80;
pub const LSDC_CRTC1_SYNC_DEVIATION_REG: c_uint = 0x1B90;
//
// In gross, LSDC_CRTC1_XXX_REG - LSDC_CRTC0_XXX_REG = 0x10, but not all of
// the registers obey this rule, LSDC_CURSORx_XXX_REG just don't honor this.
// This is the root cause we can't untangle the code by manpulating offset
// of the register access simply. Our hardware engineers are lack experiance
// when they design this...
//
pub const CRTC_PIPE_OFFSET: c_uint = 0x10;
//
// There is only one hardware cursor unit in LS7A1000 and LS2K1000, let
// CFG_HW_CLONE_EN bit be "1" could eliminate this embarrassment, we made
// it on custom clone mode application. While LS7A2000 has two hardware
// cursor unit which is good enough.
//

pub const CURSOR_FORMAT_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsdc_cursor_format {
    CURSOR_FORMAT_DISABLE = 0,
    CURSOR_FORMAT_MONOCHROME = 1,   /* masked */
    CURSOR_FORMAT_ARGB8888 = 2,     /* A8R8G8B8 */
}

//
// LS7A1000 and LS2K1000 only support 32x32, LS2K2000 and LS7A2000 support
// 64x64, but it seems that setting this bit make no harms on LS7A1000, it
// just don't take effects.
//
pub const CURSOR_SIZE_SHIFT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsdc_cursor_size {
    CURSOR_SIZE_32X32 = 0,
    CURSOR_SIZE_64X64 = 1,
}

pub const CURSOR_LOCATION_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsdc_cursor_location {
    CURSOR_ON_CRTC0 = 0,
    CURSOR_ON_CRTC1 = 1,
}

pub const LSDC_CURSOR0_CFG_REG: c_uint = 0x1520;
pub const LSDC_CURSOR0_ADDR_LO_REG: c_uint = 0x1530;
pub const LSDC_CURSOR0_ADDR_HI_REG: c_uint = 0x15e0;
pub const LSDC_CURSOR0_POSITION_REG: c_uint = 0x1540  /* [31:16] Y, [15:0] X */;
pub const LSDC_CURSOR0_BG_COLOR_REG: c_uint = 0x1550  /* background color */;
pub const LSDC_CURSOR0_FG_COLOR_REG: c_uint = 0x1560  /* foreground color */;
pub const LSDC_CURSOR1_CFG_REG: c_uint = 0x1670;
pub const LSDC_CURSOR1_ADDR_LO_REG: c_uint = 0x1680;
pub const LSDC_CURSOR1_ADDR_HI_REG: c_uint = 0x16e0;
pub const LSDC_CURSOR1_POSITION_REG: c_uint = 0x1690  /* [31:16] Y, [15:0] X */;
pub const LSDC_CURSOR1_BG_COLOR_REG: c_uint = 0x16A0  /* background color */;
pub const LSDC_CURSOR1_FG_COLOR_REG: c_uint = 0x16B0  /* foreground color */;
//
// DC Interrupt Control Register, 32bit, Address Offset: 1570
//
// Bits 15:0 inidicate the interrupt status
// Bits 31:16 control enable interrupts corresponding to bit 15:0 or not
// Write 1 to enable, write 0 to disable
//
// RF: Read Finished
// IDBU: Internal Data Buffer Underflow
// IDBFU: Internal Data Buffer Fatal Underflow
// CBRF: Cursor Buffer Read Finished Flag, no use.
// FBRF0: CRTC-0 reading from its framebuffer finished.
// FBRF1: CRTC-1 reading from its framebuffer finished.
//
// +-------+--------------------------+-------+--------+--------+-------+
// | 31:27 |         26:16            | 15:11 |   10   |   9    |   8   |
// +-------+--------------------------+-------+--------+--------+-------+
// |  N/A  | Interrupt Enable Control |  N/A  | IDBFU0 | IDBFU1 | IDBU0 |
// +-------+--------------------------+-------+--------+--------+-------+
//
// +-------+-------+-------+------+--------+--------+--------+--------+
// |   7   |   6   |   5   |  4   |   3    |   2    |   1    |   0    |
// +-------+-------+-------+------+--------+--------+--------+--------+
// | IDBU1 | FBRF0 | FBRF1 | CRRF | HSYNC0 | VSYNC0 | HSYNC1 | VSYNC1 |
// +-------+-------+-------+------+--------+--------+--------+--------+
//
// unfortunately, CRTC0's interrupt is mess with CRTC1's interrupt in one
// register again.
//
pub const LSDC_INT_REG: c_uint = 0x1570;

//
// LS7A1000/LS7A2000 have 4 gpios which are used to emulated I2C.
// They are under control of the LS7A_DC_GPIO_DAT_REG and LS7A_DC_GPIO_DIR_REG
// register, Those GPIOs has no relationship whth the GPIO hardware on the
// bridge chip itself. Those offsets are relative to DC register base address
//
// LS2k1000 don't have those registers, they use hardware i2c or general GPIO
// emulated i2c from linux i2c subsystem.
//
// GPIO data register, address offset: 0x1650
// +---------------+-----------+-----------+
// | 7 | 6 | 5 | 4 |  3  |  2  |  1  |  0  |
// +---------------+-----------+-----------+
// |               |    DVO1   |    DVO0   |
// +      N/A      +-----------+-----------+
// |               | SCL | SDA | SCL | SDA |
// +---------------+-----------+-----------+
//
pub const LS7A_DC_GPIO_DAT_REG: c_uint = 0x1650;
//
// GPIO Input/Output direction control register, address offset: 0x1660
//
pub const LS7A_DC_GPIO_DIR_REG: c_uint = 0x1660;
//
// LS7A2000 has two built-in HDMI Encoder and one VGA encoder
//
// Number of continuous packets may be present
// in HDMI hblank and vblank zone, should >= 48
//
pub const LSDC_HDMI0_ZONE_REG: c_uint = 0x1700;
pub const LSDC_HDMI1_ZONE_REG: c_uint = 0x1710;
pub const HDMI_H_ZONE_IDLE_SHIFT: c_int = 0;
pub const HDMI_V_ZONE_IDLE_SHIFT: c_int = 16;
// HDMI Iterface Control Reg

//
// Preamble:
// Immediately preceding each video data period or data island period is the
// preamble. This is a sequence of eight identical control characters that
// indicate whether the upcoming data period is a video data period or is a
// data island. The values of CTL0, CTL1, CTL2, and CTL3 indicate the type of
// data period that follows.
//

pub const HDMI_VIDEO_PREAMBLE_SHIFT: c_int = 4;
// 1: hw i2c, 0: gpio emu i2c, shouldn't put in LSDC_HDMIx_INTF_CTRL_REG

pub const LSDC_HDMI0_INTF_CTRL_REG: c_uint = 0x1720;
pub const LSDC_HDMI1_INTF_CTRL_REG: c_uint = 0x1730;

pub const LSDC_HDMI0_PHY_CTRL_REG: c_uint = 0x1800;
pub const LSDC_HDMI1_PHY_CTRL_REG: c_uint = 0x1810;
// High level duration need > 1us

// Bypass the software configured values, using default source from somewhere

pub const HDMI_PLL_IDF_SHIFT: c_int = 1;

pub const HDMI_PLL_LF_SHIFT: c_int = 6;

pub const HDMI_PLL_ODF_SHIFT: c_int = 13;

pub const LSDC_HDMI0_PHY_PLL_REG: c_uint = 0x1820;
pub const LSDC_HDMI1_PHY_PLL_REG: c_uint = 0x1830;
// LS7A2000/LS2K2000 has hpd status reg, while the two hdmi's status
// located at the one register again.
//
pub const LSDC_HDMI_HPD_STATUS_REG: c_uint = 0x1BA0;

pub const LSDC_HDMI0_PHY_CAL_REG: c_uint = 0x18C0;
pub const LSDC_HDMI1_PHY_CAL_REG: c_uint = 0x18D0;
// AVI InfoFrame
pub const LSDC_HDMI0_AVI_CONTENT0: c_uint = 0x18E0;
pub const LSDC_HDMI1_AVI_CONTENT0: c_uint = 0x18D0;
pub const LSDC_HDMI0_AVI_CONTENT1: c_uint = 0x1900;
pub const LSDC_HDMI1_AVI_CONTENT1: c_uint = 0x1910;
pub const LSDC_HDMI0_AVI_CONTENT2: c_uint = 0x1920;
pub const LSDC_HDMI1_AVI_CONTENT2: c_uint = 0x1930;
pub const LSDC_HDMI0_AVI_CONTENT3: c_uint = 0x1940;
pub const LSDC_HDMI1_AVI_CONTENT3: c_uint = 0x1950;
// 1: enable avi infoframe packet, 0: disable avi infoframe packet

// 1: send one every two frame, 0: send one each frame

//
// 1: write 1 to flush avi reg content0 ~ content3 to the packet to be send,
// The hardware will clear this bit automatically.
//

pub const LSDC_HDMI0_AVI_INFO_CRTL_REG: c_uint = 0x1960;
pub const LSDC_HDMI1_AVI_INFO_CRTL_REG: c_uint = 0x1970;
//
// LS7A2000 has the hardware which count the number of vblank generated
//
pub const LSDC_CRTC0_VSYNC_COUNTER_REG: c_uint = 0x1A00;
pub const LSDC_CRTC1_VSYNC_COUNTER_REG: c_uint = 0x1A10;
//
// LS7A2000 has the audio hardware associate with the HDMI encoder.
//
pub const LSDC_HDMI0_AUDIO_PLL_LO_REG: c_uint = 0x1A20;
pub const LSDC_HDMI1_AUDIO_PLL_LO_REG: c_uint = 0x1A30;
pub const LSDC_HDMI0_AUDIO_PLL_HI_REG: c_uint = 0x1A40;
pub const LSDC_HDMI1_AUDIO_PLL_HI_REG: c_uint = 0x1A50;
