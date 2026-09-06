//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/spacemit/k3-syscon.h
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
// SpacemiT clock and reset driver definitions for the K3 SoC

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
pub const APBS_PLL4_SWCR1: c_uint = 0x130;
pub const APBS_PLL4_SWCR2: c_uint = 0x134;
pub const APBS_PLL4_SWCR3: c_uint = 0x138;
pub const APBS_PLL5_SWCR1: c_uint = 0x13c;
pub const APBS_PLL5_SWCR2: c_uint = 0x140;
pub const APBS_PLL5_SWCR3: c_uint = 0x144;
pub const APBS_PLL6_SWCR1: c_uint = 0x148;
pub const APBS_PLL6_SWCR2: c_uint = 0x14c;
pub const APBS_PLL6_SWCR3: c_uint = 0x150;
pub const APBS_PLL7_SWCR1: c_uint = 0x158;
pub const APBS_PLL7_SWCR2: c_uint = 0x15c;
pub const APBS_PLL7_SWCR3: c_uint = 0x160;
pub const APBS_PLL8_SWCR1: c_uint = 0x180;
pub const APBS_PLL8_SWCR2: c_uint = 0x184;
pub const APBS_PLL8_SWCR3: c_uint = 0x188;
// MPMU register offset
pub const MPMU_FCCR: c_uint = 0x0008;
pub const MPMU_POSR: c_uint = 0x0010;

pub const MPMU_SUCCR: c_uint = 0x0014;
pub const MPMU_ISCCR0: c_uint = 0x0040;
pub const MPMU_ISCCR1: c_uint = 0x0044;
pub const MPMU_WDTPCR: c_uint = 0x0200;
pub const MPMU_RIPCCR: c_uint = 0x0210;
pub const MPMU_ACGR: c_uint = 0x1024;
pub const MPMU_APBCSCR: c_uint = 0x1050;
pub const MPMU_SUCCR_1: c_uint = 0x10b0;
pub const MPMU_I2S0_SYSCLK: c_uint = 0x1100;
pub const MPMU_I2S2_SYSCLK: c_uint = 0x1104;
pub const MPMU_I2S3_SYSCLK: c_uint = 0x1108;
pub const MPMU_I2S4_SYSCLK: c_uint = 0x110c;
pub const MPMU_I2S5_SYSCLK: c_uint = 0x1110;
pub const MPMU_I2S_SYSCLK_CTRL: c_uint = 0x1114;
// APBC register offset
pub const APBC_UART0_CLK_RST: c_uint = 0x00;
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
pub const APBC_TIMERS0_CLK_RST: c_uint = 0x34;
pub const APBC_TWSI2_CLK_RST: c_uint = 0x38;
pub const APBC_AIB_CLK_RST: c_uint = 0x3c;
pub const APBC_TWSI4_CLK_RST: c_uint = 0x40;
pub const APBC_TIMERS1_CLK_RST: c_uint = 0x44;
pub const APBC_ONEWIRE_CLK_RST: c_uint = 0x48;
pub const APBC_TWSI5_CLK_RST: c_uint = 0x4c;
pub const APBC_DRO_CLK_RST: c_uint = 0x58;
pub const APBC_IR0_CLK_RST: c_uint = 0x5c;
pub const APBC_IR1_CLK_RST: c_uint = 0x1c;
pub const APBC_TWSI6_CLK_RST: c_uint = 0x60;
pub const APBC_COUNTER_CLK_SEL: c_uint = 0x64;
pub const APBC_TSEN_CLK_RST: c_uint = 0x6c;
pub const APBC_UART4_CLK_RST: c_uint = 0x70;
pub const APBC_UART5_CLK_RST: c_uint = 0x74;
pub const APBC_UART6_CLK_RST: c_uint = 0x78;
pub const APBC_SSP3_CLK_RST: c_uint = 0x7c;
pub const APBC_SSPA0_CLK_RST: c_uint = 0x80;
pub const APBC_SSPA1_CLK_RST: c_uint = 0x84;
pub const APBC_SSPA2_CLK_RST: c_uint = 0x88;
pub const APBC_SSPA3_CLK_RST: c_uint = 0x8c;
pub const APBC_IPC_AP2AUD_CLK_RST: c_uint = 0x90;
pub const APBC_UART7_CLK_RST: c_uint = 0x94;
pub const APBC_UART8_CLK_RST: c_uint = 0x98;
pub const APBC_UART9_CLK_RST: c_uint = 0x9c;
pub const APBC_CAN0_CLK_RST: c_uint = 0xa0;
pub const APBC_CAN1_CLK_RST: c_uint = 0xa4;
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
pub const APBC_TIMERS2_CLK_RST: c_uint = 0x11c;
pub const APBC_TIMERS3_CLK_RST: c_uint = 0x120;
pub const APBC_TIMERS4_CLK_RST: c_uint = 0x124;
pub const APBC_TIMERS5_CLK_RST: c_uint = 0x128;
pub const APBC_TIMERS6_CLK_RST: c_uint = 0x12c;
pub const APBC_TIMERS7_CLK_RST: c_uint = 0x130;
pub const APBC_CAN2_CLK_RST: c_uint = 0x148;
pub const APBC_CAN3_CLK_RST: c_uint = 0x14c;
pub const APBC_CAN4_CLK_RST: c_uint = 0x150;
pub const APBC_UART10_CLK_RST: c_uint = 0x154;
pub const APBC_SSP0_CLK_RST: c_uint = 0x158;
pub const APBC_SSP1_CLK_RST: c_uint = 0x15c;
pub const APBC_SSPA4_CLK_RST: c_uint = 0x160;
pub const APBC_SSPA5_CLK_RST: c_uint = 0x164;
// APMU register offset
pub const APMU_CSI_CCIC2_CLK_RES_CTRL: c_uint = 0x024;
pub const APMU_ISP_CLK_RES_CTRL: c_uint = 0x038;
pub const APMU_PMU_CLK_GATE_CTRL: c_uint = 0x040;
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
pub const APMU_MCB_CLK_RES_CTRL: c_uint = 0x06c;
pub const APMU_VPU_CLK_RES_CTRL: c_uint = 0x0a4;
pub const APMU_DTC_CLK_RES_CTRL: c_uint = 0x0ac;
pub const APMU_GPU_CLK_RES_CTRL: c_uint = 0x0cc;
pub const APMU_SDH2_CLK_RES_CTRL: c_uint = 0x0e0;
pub const APMU_PMUA_MC_CTRL: c_uint = 0x0e8;
pub const APMU_PMU_CC2_AP: c_uint = 0x100;
pub const APMU_PMUA_EM_CLK_RES_CTRL: c_uint = 0x104;
pub const APMU_UCIE_CTRL: c_uint = 0x11c;
pub const APMU_RCPU_CLK_RES_CTRL: c_uint = 0x14c;
pub const APMU_TOP_DCLK_CTRL: c_uint = 0x158;
pub const APMU_LCD_EDP_CTRL: c_uint = 0x23c;
pub const APMU_UFS_CLK_RES_CTRL: c_uint = 0x268;
pub const APMU_LCD_CLK_RES_CTRL3: c_uint = 0x26c;
pub const APMU_LCD_CLK_RES_CTRL4: c_uint = 0x270;
pub const APMU_LCD_CLK_RES_CTRL5: c_uint = 0x274;
pub const APMU_CCI550_CLK_CTRL: c_uint = 0x300;
pub const APMU_ACLK_CLK_CTRL: c_uint = 0x388;
pub const APMU_CPU_C0_CLK_CTRL: c_uint = 0x38C;
pub const APMU_CPU_C1_CLK_CTRL: c_uint = 0x390;
pub const APMU_CPU_C2_CLK_CTRL: c_uint = 0x394;
pub const APMU_CPU_C3_CLK_CTRL: c_uint = 0x208;
pub const APMU_PCIE_CLK_RES_CTRL_A: c_uint = 0x1f0;
pub const APMU_PCIE_CLK_RES_CTRL_B: c_uint = 0x1d0;
pub const APMU_PCIE_CLK_RES_CTRL_C: c_uint = 0x1c8;
pub const APMU_PCIE_CLK_RES_CTRL_D: c_uint = 0x1e0;
pub const APMU_PCIE_CLK_RES_CTRL_E: c_uint = 0x1e8;
pub const APMU_EMAC0_CLK_RES_CTRL: c_uint = 0x3e4;
pub const APMU_EMAC1_CLK_RES_CTRL: c_uint = 0x3ec;
pub const APMU_EMAC2_CLK_RES_CTRL: c_uint = 0x248;
pub const APMU_ESPI_CLK_RES_CTRL: c_uint = 0x240;
pub const APMU_SNR_ISIM_VCLK_CTRL: c_uint = 0x3f8;
// DCIU register offsets
pub const DCIU_DMASYS_CLK_EN: c_uint = 0x234;
pub const DCIU_DMASYS_SDMA_CLK_EN: c_uint = 0x238;
pub const DCIU_C2_TCM_PIPE_CLK: c_uint = 0x244;
pub const DCIU_C3_TCM_PIPE_CLK: c_uint = 0x248;
pub const DCIU_DMASYS_S0_RSTN: c_uint = 0x204;
pub const DCIU_DMASYS_S1_RSTN: c_uint = 0x208;
pub const DCIU_DMASYS_A0_RSTN: c_uint = 0x20C;
pub const DCIU_DMASYS_A1_RSTN: c_uint = 0x210;
pub const DCIU_DMASYS_A2_RSTN: c_uint = 0x214;
pub const DCIU_DMASYS_A3_RSTN: c_uint = 0x218;
pub const DCIU_DMASYS_A4_RSTN: c_uint = 0x21C;
pub const DCIU_DMASYS_A5_RSTN: c_uint = 0x220;
pub const DCIU_DMASYS_A6_RSTN: c_uint = 0x224;
pub const DCIU_DMASYS_A7_RSTN: c_uint = 0x228;
pub const DCIU_DMASYS_RSTN: c_uint = 0x22C;
pub const DCIU_DMASYS_SDMA_RSTN: c_uint = 0x230;
// RCPU SYSCTRL register offsets
pub const RCPU_CAN_CLK_RST: c_uint = 0x4c;
pub const RCPU_CAN1_CLK_RST: c_uint = 0xF0;
pub const RCPU_CAN2_CLK_RST: c_uint = 0xF4;
pub const RCPU_CAN3_CLK_RST: c_uint = 0xF8;
pub const RCPU_CAN4_CLK_RST: c_uint = 0xFC;
pub const RCPU_IRC_CLK_RST: c_uint = 0x48;
pub const RCPU_IRC1_CLK_RST: c_uint = 0xEC;
pub const RCPU_GMAC_CLK_RST: c_uint = 0xE4;
pub const RCPU_ESPI_CLK_RST: c_uint = 0xDC;
pub const RCPU_AUDIO_I2S0_SYS_CLK_CTRL: c_uint = 0x70;
pub const RCPU_AUDIO_I2S1_SYS_CLK_CTRL: c_uint = 0x44;
// RCPU UARTCTRL register offsets
pub const RCPU1_UART0_CLK_RST: c_uint = 0x00;
pub const RCPU1_UART1_CLK_RST: c_uint = 0x04;
pub const RCPU1_UART2_CLK_RST: c_uint = 0x08;
pub const RCPU1_UART3_CLK_RST: c_uint = 0x0c;
pub const RCPU1_UART4_CLK_RST: c_uint = 0x10;
pub const RCPU1_UART5_CLK_RST: c_uint = 0x14;
// RCPU I2SCTRL register offsets
pub const RCPU2_AUDIO_I2S0_TX_RX_CLK_CTRL: c_uint = 0x60;
pub const RCPU2_AUDIO_I2S1_TX_RX_CLK_CTRL: c_uint = 0x64;
pub const RCPU2_AUDIO_I2S2_TX_RX_CLK_CTRL: c_uint = 0x68;
pub const RCPU2_AUDIO_I2S3_TX_RX_CLK_CTRL: c_uint = 0x6C;
pub const RCPU2_AUDIO_I2S2_SYS_CLK_CTRL: c_uint = 0x44;
pub const RCPU2_AUDIO_I2S3_SYS_CLK_CTRL: c_uint = 0x54;
// RCPU SPICTRL register offsets
pub const RCPU3_SSP0_CLK_RST: c_uint = 0x00;
pub const RCPU3_SSP1_CLK_RST: c_uint = 0x04;
pub const RCPU3_PWR_SSP_CLK_RST: c_uint = 0x08;
// RCPU I2CCTRL register offsets
pub const RCPU4_I2C0_CLK_RST: c_uint = 0x00;
pub const RCPU4_I2C1_CLK_RST: c_uint = 0x04;
pub const RCPU4_PWR_I2C_CLK_RST: c_uint = 0x08;
// RPMU register offsets
pub const RCPU5_AON_PER_CLK_RST_CTRL: c_uint = 0x2C;
pub const RCPU5_TIMER1_CLK_RST: c_uint = 0x4C;
pub const RCPU5_TIMER2_CLK_RST: c_uint = 0x70;
pub const RCPU5_TIMER3_CLK_RST: c_uint = 0x78;
pub const RCPU5_TIMER4_CLK_RST: c_uint = 0x7C;
pub const RCPU5_GPIO_AND_EDGE_CLK_RST: c_uint = 0x74;
pub const RCPU5_RCPU_BUS_CLK_CTRL: c_uint = 0xC0;
pub const RCPU5_RT24_CORE0_CLK_CTRL: c_uint = 0xC4;
pub const RCPU5_RT24_CORE1_CLK_CTRL: c_uint = 0xC8;
pub const RCPU5_RT24_CORE0_SW_RESET: c_uint = 0xCC;
pub const RCPU5_RT24_CORE1_SW_RESET: c_uint = 0xD0;
// RCPU PWMCTRL register offsets
pub const RCPU6_PWM0_CLK_RST: c_uint = 0x00;
pub const RCPU6_PWM1_CLK_RST: c_uint = 0x04;
pub const RCPU6_PWM2_CLK_RST: c_uint = 0x08;
pub const RCPU6_PWM3_CLK_RST: c_uint = 0x0c;
pub const RCPU6_PWM4_CLK_RST: c_uint = 0x10;
pub const RCPU6_PWM5_CLK_RST: c_uint = 0x14;
pub const RCPU6_PWM6_CLK_RST: c_uint = 0x18;
pub const RCPU6_PWM7_CLK_RST: c_uint = 0x1c;
pub const RCPU6_PWM8_CLK_RST: c_uint = 0x20;
pub const RCPU6_PWM9_CLK_RST: c_uint = 0x24;
// APBC2 SEC register offsets
pub const APBC2_UART1_CLK_RST: c_uint = 0x00;
pub const APBC2_SSP2_CLK_RST: c_uint = 0x04;
pub const APBC2_TWSI3_CLK_RST: c_uint = 0x08;
pub const APBC2_RTC_CLK_RST: c_uint = 0x0c;
pub const APBC2_TIMERS_CLK_RST: c_uint = 0x10;
pub const APBC2_GPIO_CLK_RST: c_uint = 0x1c;
