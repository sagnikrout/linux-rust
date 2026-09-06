//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/spacemit/k1-syscon.h
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
// SpacemiT clock and reset driver definitions for the K1 SoC

// APBS register offset
pub const APBS_PLL1_SWCR1: c_uint = 0x100;
pub const APBS_PLL1_SWCR2: c_uint = 0x104;
pub const APBS_PLL1_SWCR3: c_uint = 0x108;
pub const APBS_PLL2_SWCR1: c_uint = 0x118;
pub const APBS_PLL2_SWCR2: c_uint = 0x11c;
pub const APBS_PLL2_SWCR3: c_uint = 0x120;
pub const APBS_PLL3_SWCR1: c_uint = 0x124;
pub const APBS_PLL3_SWCR2: c_uint = 0x128;
pub const APBS_PLL3_SWCR3: c_uint = 0x12c;
// MPMU register offset
pub const MPMU_POSR: c_uint = 0x0010;
pub const MPMU_FCCR: c_uint = 0x0008;

pub const MPMU_SUCCR: c_uint = 0x0014;
pub const MPMU_ISCCR: c_uint = 0x0044;
pub const MPMU_WDTPCR: c_uint = 0x0200;
pub const MPMU_RIPCCR: c_uint = 0x0210;
pub const MPMU_ACGR: c_uint = 0x1024;
pub const MPMU_APBCSCR: c_uint = 0x1050;
pub const MPMU_SUCCR_1: c_uint = 0x10b0;
// APBC register offset
pub const APBC_UART1_CLK_RST: c_uint = 0x00;
pub const APBC_UART2_CLK_RST: c_uint = 0x04;
pub const APBC_GPIO_CLK_RST: c_uint = 0x08;
pub const APBC_PWM0_CLK_RST: c_uint = 0x0c;
pub const APBC_PWM1_CLK_RST: c_uint = 0x10;
pub const APBC_PWM2_CLK_RST: c_uint = 0x14;
pub const APBC_PWM3_CLK_RST: c_uint = 0x18;
pub const APBC_TWSI8_CLK_RST: c_uint = 0x20;
pub const APBC_UART3_CLK_RST: c_uint = 0x24;
pub const APBC_RTC_CLK_RST: c_uint = 0x28;
pub const APBC_TWSI0_CLK_RST: c_uint = 0x2c;
pub const APBC_TWSI1_CLK_RST: c_uint = 0x30;
pub const APBC_TIMERS1_CLK_RST: c_uint = 0x34;
pub const APBC_TWSI2_CLK_RST: c_uint = 0x38;
pub const APBC_AIB_CLK_RST: c_uint = 0x3c;
pub const APBC_TWSI4_CLK_RST: c_uint = 0x40;
pub const APBC_TIMERS2_CLK_RST: c_uint = 0x44;
pub const APBC_ONEWIRE_CLK_RST: c_uint = 0x48;
pub const APBC_TWSI5_CLK_RST: c_uint = 0x4c;
pub const APBC_DRO_CLK_RST: c_uint = 0x58;
pub const APBC_IR_CLK_RST: c_uint = 0x5c;
pub const APBC_TWSI6_CLK_RST: c_uint = 0x60;
pub const APBC_COUNTER_CLK_SEL: c_uint = 0x64;
pub const APBC_TWSI7_CLK_RST: c_uint = 0x68;
pub const APBC_TSEN_CLK_RST: c_uint = 0x6c;
pub const APBC_UART4_CLK_RST: c_uint = 0x70;
pub const APBC_UART5_CLK_RST: c_uint = 0x74;
pub const APBC_UART6_CLK_RST: c_uint = 0x78;
pub const APBC_SSP3_CLK_RST: c_uint = 0x7c;
pub const APBC_SSPA0_CLK_RST: c_uint = 0x80;
pub const APBC_SSPA1_CLK_RST: c_uint = 0x84;
pub const APBC_IPC_AP2AUD_CLK_RST: c_uint = 0x90;
pub const APBC_UART7_CLK_RST: c_uint = 0x94;
pub const APBC_UART8_CLK_RST: c_uint = 0x98;
pub const APBC_UART9_CLK_RST: c_uint = 0x9c;
pub const APBC_CAN0_CLK_RST: c_uint = 0xa0;
pub const APBC_PWM4_CLK_RST: c_uint = 0xa8;
pub const APBC_PWM5_CLK_RST: c_uint = 0xac;
pub const APBC_PWM6_CLK_RST: c_uint = 0xb0;
pub const APBC_PWM7_CLK_RST: c_uint = 0xb4;
pub const APBC_PWM8_CLK_RST: c_uint = 0xb8;
pub const APBC_PWM9_CLK_RST: c_uint = 0xbc;
pub const APBC_PWM10_CLK_RST: c_uint = 0xc0;
pub const APBC_PWM11_CLK_RST: c_uint = 0xc4;
pub const APBC_PWM12_CLK_RST: c_uint = 0xc8;
pub const APBC_PWM13_CLK_RST: c_uint = 0xcc;
pub const APBC_PWM14_CLK_RST: c_uint = 0xd0;
pub const APBC_PWM15_CLK_RST: c_uint = 0xd4;
pub const APBC_PWM16_CLK_RST: c_uint = 0xd8;
pub const APBC_PWM17_CLK_RST: c_uint = 0xdc;
pub const APBC_PWM18_CLK_RST: c_uint = 0xe0;
pub const APBC_PWM19_CLK_RST: c_uint = 0xe4;
// APMU register offset
pub const APMU_JPG_CLK_RES_CTRL: c_uint = 0x020;
pub const APMU_CSI_CCIC2_CLK_RES_CTRL: c_uint = 0x024;
pub const APMU_ISP_CLK_RES_CTRL: c_uint = 0x038;
pub const APMU_LCD_CLK_RES_CTRL1: c_uint = 0x044;
pub const APMU_LCD_SPI_CLK_RES_CTRL: c_uint = 0x048;
pub const APMU_LCD_CLK_RES_CTRL2: c_uint = 0x04c;
pub const APMU_CCIC_CLK_RES_CTRL: c_uint = 0x050;
pub const APMU_SDH0_CLK_RES_CTRL: c_uint = 0x054;
pub const APMU_SDH1_CLK_RES_CTRL: c_uint = 0x058;
pub const APMU_USB_CLK_RES_CTRL: c_uint = 0x05c;
pub const APMU_QSPI_CLK_RES_CTRL: c_uint = 0x060;
pub const APMU_DMA_CLK_RES_CTRL: c_uint = 0x064;
pub const APMU_AES_CLK_RES_CTRL: c_uint = 0x068;
pub const APMU_VPU_CLK_RES_CTRL: c_uint = 0x0a4;
pub const APMU_GPU_CLK_RES_CTRL: c_uint = 0x0cc;
pub const APMU_SDH2_CLK_RES_CTRL: c_uint = 0x0e0;
pub const APMU_PMUA_MC_CTRL: c_uint = 0x0e8;
pub const APMU_PMU_CC2_AP: c_uint = 0x100;
pub const APMU_PMUA_EM_CLK_RES_CTRL: c_uint = 0x104;
pub const APMU_AUDIO_CLK_RES_CTRL: c_uint = 0x14c;
pub const APMU_HDMI_CLK_RES_CTRL: c_uint = 0x1b8;
pub const APMU_CCI550_CLK_CTRL: c_uint = 0x300;
pub const APMU_ACLK_CLK_CTRL: c_uint = 0x388;
pub const APMU_CPU_C0_CLK_CTRL: c_uint = 0x38C;
pub const APMU_CPU_C1_CLK_CTRL: c_uint = 0x390;
pub const APMU_PCIE_CLK_RES_CTRL_0: c_uint = 0x3cc;
pub const APMU_PCIE_CLK_RES_CTRL_1: c_uint = 0x3d4;
pub const APMU_PCIE_CLK_RES_CTRL_2: c_uint = 0x3dc;
pub const APMU_EMAC0_CLK_RES_CTRL: c_uint = 0x3e4;
pub const APMU_EMAC1_CLK_RES_CTRL: c_uint = 0x3ec;
// RCPU register offsets
pub const RCPU_SSP0_CLK_RST: c_uint = 0x0028;
pub const RCPU_I2C0_CLK_RST: c_uint = 0x0030;
pub const RCPU_UART1_CLK_RST: c_uint = 0x003c;
pub const RCPU_CAN_CLK_RST: c_uint = 0x0048;
pub const RCPU_IR_CLK_RST: c_uint = 0x004c;
pub const RCPU_UART0_CLK_RST: c_uint = 0x00d8;
pub const AUDIO_HDMI_CLK_CTRL: c_uint = 0x2044;
// RCPU2 register offsets
pub const RCPU2_PWM0_CLK_RST: c_uint = 0x0000;
pub const RCPU2_PWM1_CLK_RST: c_uint = 0x0004;
pub const RCPU2_PWM2_CLK_RST: c_uint = 0x0008;
pub const RCPU2_PWM3_CLK_RST: c_uint = 0x000c;
pub const RCPU2_PWM4_CLK_RST: c_uint = 0x0010;
pub const RCPU2_PWM5_CLK_RST: c_uint = 0x0014;
pub const RCPU2_PWM6_CLK_RST: c_uint = 0x0018;
pub const RCPU2_PWM7_CLK_RST: c_uint = 0x001c;
pub const RCPU2_PWM8_CLK_RST: c_uint = 0x0020;
pub const RCPU2_PWM9_CLK_RST: c_uint = 0x0024;
// APBC2 register offsets
pub const APBC2_UART1_CLK_RST: c_uint = 0x0000;
pub const APBC2_SSP2_CLK_RST: c_uint = 0x0004;
pub const APBC2_TWSI3_CLK_RST: c_uint = 0x0008;
pub const APBC2_RTC_CLK_RST: c_uint = 0x000c;
pub const APBC2_TIMERS0_CLK_RST: c_uint = 0x0010;
pub const APBC2_KPC_CLK_RST: c_uint = 0x0014;
pub const APBC2_GPIO_CLK_RST: c_uint = 0x001c;
