//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/keylargo.h
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

//
// keylargo.h: definitions for using the "KeyLargo" I/O controller chip.
//
// "Pangea" chipset has keylargo device-id 0x25 while core99
// has device-id 0x22. The rev. of the pangea one is 0, so we
// fake an artificial rev. in keylargo_rev by oring 0x100
//
pub const KL_PANGEA_REV: c_uint = 0x100;
// offset from base for feature control registers
pub const KEYLARGO_MBCR: c_uint = 0x34	/* KL Only, Media bay control/status */;
pub const KEYLARGO_FCR0: c_uint = 0x38;
pub const KEYLARGO_FCR1: c_uint = 0x3c;
pub const KEYLARGO_FCR2: c_uint = 0x40;
pub const KEYLARGO_FCR3: c_uint = 0x44;
pub const KEYLARGO_FCR4: c_uint = 0x48;
pub const KEYLARGO_FCR5: c_uint = 0x4c	/* Pangea only */;
// K2 additional FCRs
pub const K2_FCR6: c_uint = 0x34;
pub const K2_FCR7: c_uint = 0x30;
pub const K2_FCR8: c_uint = 0x2c;
pub const K2_FCR9: c_uint = 0x28;
pub const K2_FCR10: c_uint = 0x24;
// GPIO registers
pub const KEYLARGO_GPIO_LEVELS0: c_uint = 0x50;
pub const KEYLARGO_GPIO_LEVELS1: c_uint = 0x54;
pub const KEYLARGO_GPIO_EXTINT_0: c_uint = 0x58;
pub const KEYLARGO_GPIO_EXTINT_CNT: c_int = 18;
pub const KEYLARGO_GPIO_0: c_uint = 0x6A;
pub const KEYLARGO_GPIO_CNT: c_int = 17;
pub const KEYLARGO_GPIO_EXTINT_DUAL_EDGE: c_uint = 0x80;
pub const KEYLARGO_GPIO_OUTPUT_ENABLE: c_uint = 0x04;
pub const KEYLARGO_GPIO_OUTOUT_DATA: c_uint = 0x01;
pub const KEYLARGO_GPIO_INPUT_DATA: c_uint = 0x02;
// K2 does only extint GPIOs and does 51 of them
pub const K2_GPIO_EXTINT_0: c_uint = 0x58;
pub const K2_GPIO_EXTINT_CNT: c_int = 51;
// Specific GPIO regs

// Hrm... this one is only to be used on Pismo. It seems to also
// control the timebase enable on other machines. Still to be
// experimented... --BenH.
//

pub const KL_GPIO_EXTINT_CPU1_ASSERT: c_uint = 0x04;
pub const KL_GPIO_EXTINT_CPU1_RELEASE: c_uint = 0x38;

//
// Bits in feature control register. Those bits different for K2 are
// listed separately
//
pub const KL_MBCR_MB0_PCI_ENABLE: c_uint = 0x00000800	/* exist ? */;
pub const KL_MBCR_MB0_IDE_ENABLE: c_uint = 0x00001000;
pub const KL_MBCR_MB0_FLOPPY_ENABLE: c_uint = 0x00002000	/* exist ? */;
pub const KL_MBCR_MB0_SOUND_ENABLE: c_uint = 0x00004000	/* hrm... */;
pub const KL_MBCR_MB0_DEV_MASK: c_uint = 0x00007800;
pub const KL_MBCR_MB0_DEV_POWER: c_uint = 0x00000400;
pub const KL_MBCR_MB0_DEV_RESET: c_uint = 0x00000200;
pub const KL_MBCR_MB0_ENABLE: c_uint = 0x00000100;
pub const KL_MBCR_MB1_PCI_ENABLE: c_uint = 0x08000000	/* exist ? */;
pub const KL_MBCR_MB1_IDE_ENABLE: c_uint = 0x10000000;
pub const KL_MBCR_MB1_FLOPPY_ENABLE: c_uint = 0x20000000	/* exist ? */;
pub const KL_MBCR_MB1_SOUND_ENABLE: c_uint = 0x40000000	/* hrm... */;
pub const KL_MBCR_MB1_DEV_MASK: c_uint = 0x78000000;
pub const KL_MBCR_MB1_DEV_POWER: c_uint = 0x04000000;
pub const KL_MBCR_MB1_DEV_RESET: c_uint = 0x02000000;
pub const KL_MBCR_MB1_ENABLE: c_uint = 0x01000000;
pub const KL0_SCC_B_INTF_ENABLE: c_uint = 0x00000001	/* (KL Only) */;
pub const KL0_SCC_A_INTF_ENABLE: c_uint = 0x00000002;
pub const KL0_SCC_SLOWPCLK: c_uint = 0x00000004;
pub const KL0_SCC_RESET: c_uint = 0x00000008;
pub const KL0_SCCA_ENABLE: c_uint = 0x00000010;
pub const KL0_SCCB_ENABLE: c_uint = 0x00000020;
pub const KL0_SCC_CELL_ENABLE: c_uint = 0x00000040;
pub const KL0_IRDA_HIGH_BAND: c_uint = 0x00000100	/* (KL Only) */;
pub const KL0_IRDA_SOURCE2_SEL: c_uint = 0x00000200	/* (KL Only) */;
pub const KL0_IRDA_SOURCE1_SEL: c_uint = 0x00000400	/* (KL Only) */;
pub const KL0_PG_USB0_PMI_ENABLE: c_uint = 0x00000400	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_RESET: c_uint = 0x00000800	/* (KL Only) */;
pub const KL0_PG_USB0_REF_SUSPEND_SEL: c_uint = 0x00000800	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_DEFAULT1: c_uint = 0x00001000	/* (KL Only) */;
pub const KL0_PG_USB0_REF_SUSPEND: c_uint = 0x00001000	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_DEFAULT0: c_uint = 0x00002000	/* (KL Only) */;
pub const KL0_PG_USB0_PAD_SUSPEND: c_uint = 0x00002000	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_FAST_CONNECT: c_uint = 0x00004000	/* (KL Only) */;
pub const KL0_PG_USB1_PMI_ENABLE: c_uint = 0x00004000	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_ENABLE: c_uint = 0x00008000	/* (KL Only) */;
pub const KL0_PG_USB1_REF_SUSPEND_SEL: c_uint = 0x00008000	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_CLK32_ENABLE: c_uint = 0x00010000	/* (KL Only) */;
pub const KL0_PG_USB1_REF_SUSPEND: c_uint = 0x00010000	/* (Pangea/Intrepid Only) */;
pub const KL0_IRDA_CLK19_ENABLE: c_uint = 0x00020000	/* (KL Only) */;
pub const KL0_PG_USB1_PAD_SUSPEND: c_uint = 0x00020000	/* (Pangea/Intrepid Only) */;
pub const KL0_USB0_PAD_SUSPEND0: c_uint = 0x00040000;
pub const KL0_USB0_PAD_SUSPEND1: c_uint = 0x00080000;
pub const KL0_USB0_CELL_ENABLE: c_uint = 0x00100000;
pub const KL0_USB1_PAD_SUSPEND0: c_uint = 0x00400000;
pub const KL0_USB1_PAD_SUSPEND1: c_uint = 0x00800000;
pub const KL0_USB1_CELL_ENABLE: c_uint = 0x01000000;
pub const KL0_USB_REF_SUSPEND: c_uint = 0x10000000	/* (KL Only) */;

pub const KL1_USB2_PMI_ENABLE: c_uint = 0x00000001	/* Intrepid only */;
pub const KL1_AUDIO_SEL_22MCLK: c_uint = 0x00000002	/* KL/Pangea only */;
pub const KL1_USB2_REF_SUSPEND_SEL: c_uint = 0x00000002	/* Intrepid only */;
pub const KL1_USB2_REF_SUSPEND: c_uint = 0x00000004	/* Intrepid only */;
pub const KL1_AUDIO_CLK_ENABLE_BIT: c_uint = 0x00000008	/* KL/Pangea only */;
pub const KL1_USB2_PAD_SUSPEND_SEL: c_uint = 0x00000008	/* Intrepid only */;
pub const KL1_USB2_PAD_SUSPEND0: c_uint = 0x00000010	/* Intrepid only */;
pub const KL1_AUDIO_CLK_OUT_ENABLE: c_uint = 0x00000020	/* KL/Pangea only */;
pub const KL1_USB2_PAD_SUSPEND1: c_uint = 0x00000020	/* Intrepid only */;
pub const KL1_AUDIO_CELL_ENABLE: c_uint = 0x00000040	/* KL/Pangea only */;
pub const KL1_USB2_CELL_ENABLE: c_uint = 0x00000040	/* Intrepid only */;
pub const KL1_AUDIO_CHOOSE: c_uint = 0x00000080	/* KL/Pangea only */;
pub const KL1_I2S0_CHOOSE: c_uint = 0x00000200	/* KL Only */;
pub const KL1_I2S0_CELL_ENABLE: c_uint = 0x00000400;
pub const KL1_I2S0_CLK_ENABLE_BIT: c_uint = 0x00001000;
pub const KL1_I2S0_ENABLE: c_uint = 0x00002000;
pub const KL1_I2S1_CELL_ENABLE: c_uint = 0x00020000;
pub const KL1_I2S1_CLK_ENABLE_BIT: c_uint = 0x00080000;
pub const KL1_I2S1_ENABLE: c_uint = 0x00100000;
pub const KL1_EIDE0_ENABLE: c_uint = 0x00800000	/* KL/Intrepid Only */;
pub const KL1_EIDE0_RESET_N: c_uint = 0x01000000	/* KL/Intrepid Only */;
pub const KL1_EIDE1_ENABLE: c_uint = 0x04000000	/* KL Only */;
pub const KL1_EIDE1_RESET_N: c_uint = 0x08000000	/* KL Only */;
pub const KL1_UIDE_ENABLE: c_uint = 0x20000000	/* KL/Pangea Only */;
pub const KL1_UIDE_RESET_N: c_uint = 0x40000000	/* KL/Pangea Only */;
pub const KL2_IOBUS_ENABLE: c_uint = 0x00000002;
pub const KL2_SLEEP_STATE_BIT: c_uint = 0x00000100	/* KL Only */;
pub const KL2_PG_STOP_ALL_CLOCKS: c_uint = 0x00000100	/* Pangea Only */;
pub const KL2_MPIC_ENABLE: c_uint = 0x00020000;
pub const KL2_CARDSLOT_RESET: c_uint = 0x00040000	/* Pangea/Intrepid Only */;
pub const KL2_ALT_DATA_OUT: c_uint = 0x02000000	/* KL Only ??? */;
pub const KL2_MEM_IS_BIG: c_uint = 0x04000000;
pub const KL2_CARDSEL_16: c_uint = 0x08000000;
pub const KL3_SHUTDOWN_PLL_TOTAL: c_uint = 0x00000001	/* KL/Pangea only */;
pub const KL3_SHUTDOWN_PLLKW6: c_uint = 0x00000002	/* KL/Pangea only */;
pub const KL3_IT_SHUTDOWN_PLL3: c_uint = 0x00000002	/* Intrepid only */;
pub const KL3_SHUTDOWN_PLLKW4: c_uint = 0x00000004	/* KL/Pangea only */;
pub const KL3_IT_SHUTDOWN_PLL2: c_uint = 0x00000004	/* Intrepid only */;
pub const KL3_SHUTDOWN_PLLKW35: c_uint = 0x00000008	/* KL/Pangea only */;
pub const KL3_IT_SHUTDOWN_PLL1: c_uint = 0x00000008	/* Intrepid only */;
pub const KL3_SHUTDOWN_PLLKW12: c_uint = 0x00000010	/* KL Only */;
pub const KL3_IT_ENABLE_PLL3_SHUTDOWN: c_uint = 0x00000010	/* Intrepid only */;
pub const KL3_PLL_RESET: c_uint = 0x00000020	/* KL/Pangea only */;
pub const KL3_IT_ENABLE_PLL2_SHUTDOWN: c_uint = 0x00000020	/* Intrepid only */;
pub const KL3_IT_ENABLE_PLL1_SHUTDOWN: c_uint = 0x00000010	/* Intrepid only */;
pub const KL3_SHUTDOWN_PLL2X: c_uint = 0x00000080	/* KL Only */;
pub const KL3_CLK66_ENABLE: c_uint = 0x00000100	/* KL Only */;
pub const KL3_CLK49_ENABLE: c_uint = 0x00000200;
pub const KL3_CLK45_ENABLE: c_uint = 0x00000400;
pub const KL3_CLK31_ENABLE: c_uint = 0x00000800	/* KL/Pangea only */;
pub const KL3_TIMER_CLK18_ENABLE: c_uint = 0x00001000;
pub const KL3_I2S1_CLK18_ENABLE: c_uint = 0x00002000;
pub const KL3_I2S0_CLK18_ENABLE: c_uint = 0x00004000;
pub const KL3_VIA_CLK16_ENABLE: c_uint = 0x00008000	/* KL/Pangea only */;
pub const KL3_IT_VIA_CLK32_ENABLE: c_uint = 0x00008000	/* Intrepid only */;
pub const KL3_STOPPING33_ENABLED: c_uint = 0x00080000	/* KL Only */;
pub const KL3_PG_PLL_ENABLE_TEST: c_uint = 0x00080000	/* Pangea Only */;
// Intrepid USB bus 2, port 0,1

// Port 0,1 : bus 0, port 2,3 : bus 1

// Pangea and Intrepid only

pub const KL5_SCC_USE_CLK31: c_uint = 0x00000002	/* Pangea Only */;
pub const KL5_PWM_CLK32_EN: c_uint = 0x00000004;
pub const KL5_CLK3_68_EN: c_uint = 0x00000010;
pub const KL5_CLK32_EN: c_uint = 0x00000020;
// K2 definitions
pub const K2_FCR0_USB0_SWRESET: c_uint = 0x00200000;
pub const K2_FCR0_USB1_SWRESET: c_uint = 0x02000000;
pub const K2_FCR0_RING_PME_DISABLE: c_uint = 0x08000000;
pub const K2_FCR1_PCI1_BUS_RESET_N: c_uint = 0x00000010;
pub const K2_FCR1_PCI1_SLEEP_RESET_EN: c_uint = 0x00000020;
pub const K2_FCR1_I2S0_CELL_ENABLE: c_uint = 0x00000400;
pub const K2_FCR1_I2S0_RESET: c_uint = 0x00000800;
pub const K2_FCR1_I2S0_CLK_ENABLE_BIT: c_uint = 0x00001000;
pub const K2_FCR1_I2S0_ENABLE: c_uint = 0x00002000;
pub const K2_FCR1_PCI1_CLK_ENABLE: c_uint = 0x00004000;
pub const K2_FCR1_FW_CLK_ENABLE: c_uint = 0x00008000;
pub const K2_FCR1_FW_RESET_N: c_uint = 0x00010000;
pub const K2_FCR1_I2S1_CELL_ENABLE: c_uint = 0x00020000;
pub const K2_FCR1_I2S1_CLK_ENABLE_BIT: c_uint = 0x00080000;
pub const K2_FCR1_I2S1_ENABLE: c_uint = 0x00100000;
pub const K2_FCR1_GMAC_CLK_ENABLE: c_uint = 0x00400000;
pub const K2_FCR1_GMAC_POWER_DOWN: c_uint = 0x00800000;
pub const K2_FCR1_GMAC_RESET_N: c_uint = 0x01000000;
pub const K2_FCR1_SATA_CLK_ENABLE: c_uint = 0x02000000;
pub const K2_FCR1_SATA_POWER_DOWN: c_uint = 0x04000000;
pub const K2_FCR1_SATA_RESET_N: c_uint = 0x08000000;
pub const K2_FCR1_UATA_CLK_ENABLE: c_uint = 0x10000000;
pub const K2_FCR1_UATA_RESET_N: c_uint = 0x40000000;
pub const K2_FCR1_UATA_CHOOSE_CLK66: c_uint = 0x80000000;
// Shasta definitions
pub const SH_FCR1_I2S2_CELL_ENABLE: c_uint = 0x00000010;
pub const SH_FCR1_I2S2_CLK_ENABLE_BIT: c_uint = 0x00000040;
pub const SH_FCR1_I2S2_ENABLE: c_uint = 0x00000080;
pub const SH_FCR3_I2S2_CLK18_ENABLE: c_uint = 0x00008000;

