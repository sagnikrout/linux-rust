//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/samsung/exynos-regs-pmu.h
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
// Copyright (c) 2010-2015 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Exynos - Power management unit definition
//
// Notice:
// This is not a list of all Exynos Power Management Unit SFRs.
// There are too many of them, not mentioning subtle differences
// between SoCs. For now, put here only the used registers.
//

pub const S5P_CENTRAL_SEQ_CONFIGURATION: c_uint = 0x0200;

pub const S5P_CENTRAL_SEQ_OPTION: c_uint = 0x0208;

pub const EXYNOS_SWRESET: c_uint = 0x0400;
pub const S5P_WAKEUP_STAT: c_uint = 0x0600;
// Value for EXYNOS_EINT_WAKEUP_MASK disabling all external wakeup interrupts
pub const EXYNOS_EINT_WAKEUP_MASK_DISABLED: c_uint = 0xffffffff;
pub const EXYNOS_EINT_WAKEUP_MASK: c_uint = 0x0604;
pub const S5P_WAKEUP_MASK: c_uint = 0x0608;
pub const S5P_WAKEUP_MASK2: c_uint = 0x0614;
// MIPI_PHYn_CONTROL, valid for Exynos3250, Exynos4, Exynos5250 and Exynos5433

// Phy enable bit, common for all phy registers, not only MIPI

// USB PHY enable bit, valid for Exynos7870

pub const S5P_INFORM0: c_uint = 0x0800;
pub const S5P_INFORM1: c_uint = 0x0804;
pub const S5P_INFORM5: c_uint = 0x0814;
pub const S5P_INFORM6: c_uint = 0x0818;
pub const S5P_INFORM7: c_uint = 0x081C;
pub const S5P_PMU_SPARE2: c_uint = 0x0908;
pub const S5P_PMU_SPARE3: c_uint = 0x090C;
pub const EXYNOS_IROM_DATA2: c_uint = 0x0988;
pub const S5P_ARM_CORE0_LOWPWR: c_uint = 0x1000;
pub const S5P_DIS_IRQ_CORE0: c_uint = 0x1004;
pub const S5P_DIS_IRQ_CENTRAL0: c_uint = 0x1008;
pub const S5P_ARM_CORE1_LOWPWR: c_uint = 0x1010;
pub const S5P_DIS_IRQ_CORE1: c_uint = 0x1014;
pub const S5P_DIS_IRQ_CENTRAL1: c_uint = 0x1018;
pub const S5P_ARM_COMMON_LOWPWR: c_uint = 0x1080;
pub const S5P_L2_0_LOWPWR: c_uint = 0x10C0;
pub const S5P_L2_1_LOWPWR: c_uint = 0x10C4;
pub const S5P_CMU_ACLKSTOP_LOWPWR: c_uint = 0x1100;
pub const S5P_CMU_SCLKSTOP_LOWPWR: c_uint = 0x1104;
pub const S5P_CMU_RESET_LOWPWR: c_uint = 0x110C;
pub const S5P_APLL_SYSCLK_LOWPWR: c_uint = 0x1120;
pub const S5P_MPLL_SYSCLK_LOWPWR: c_uint = 0x1124;
pub const S5P_VPLL_SYSCLK_LOWPWR: c_uint = 0x1128;
pub const S5P_EPLL_SYSCLK_LOWPWR: c_uint = 0x112C;
pub const S5P_CMU_CLKSTOP_GPS_ALIVE_LOWPWR: c_uint = 0x1138;
pub const S5P_CMU_RESET_GPSALIVE_LOWPWR: c_uint = 0x113C;
pub const S5P_CMU_CLKSTOP_CAM_LOWPWR: c_uint = 0x1140;
pub const S5P_CMU_CLKSTOP_TV_LOWPWR: c_uint = 0x1144;
pub const S5P_CMU_CLKSTOP_MFC_LOWPWR: c_uint = 0x1148;
pub const S5P_CMU_CLKSTOP_G3D_LOWPWR: c_uint = 0x114C;
pub const S5P_CMU_CLKSTOP_LCD0_LOWPWR: c_uint = 0x1150;
pub const S5P_CMU_CLKSTOP_MAUDIO_LOWPWR: c_uint = 0x1158;
pub const S5P_CMU_CLKSTOP_GPS_LOWPWR: c_uint = 0x115C;
pub const S5P_CMU_RESET_CAM_LOWPWR: c_uint = 0x1160;
pub const S5P_CMU_RESET_TV_LOWPWR: c_uint = 0x1164;
pub const S5P_CMU_RESET_MFC_LOWPWR: c_uint = 0x1168;
pub const S5P_CMU_RESET_G3D_LOWPWR: c_uint = 0x116C;
pub const S5P_CMU_RESET_LCD0_LOWPWR: c_uint = 0x1170;
pub const S5P_CMU_RESET_MAUDIO_LOWPWR: c_uint = 0x1178;
pub const S5P_CMU_RESET_GPS_LOWPWR: c_uint = 0x117C;
pub const S5P_TOP_BUS_LOWPWR: c_uint = 0x1180;
pub const S5P_TOP_RETENTION_LOWPWR: c_uint = 0x1184;
pub const S5P_TOP_PWR_LOWPWR: c_uint = 0x1188;
pub const S5P_LOGIC_RESET_LOWPWR: c_uint = 0x11A0;
pub const S5P_ONENAND_MEM_LOWPWR: c_uint = 0x11C0;
pub const S5P_G2D_ACP_MEM_LOWPWR: c_uint = 0x11C8;
pub const S5P_USBOTG_MEM_LOWPWR: c_uint = 0x11CC;
pub const S5P_HSMMC_MEM_LOWPWR: c_uint = 0x11D0;
pub const S5P_CSSYS_MEM_LOWPWR: c_uint = 0x11D4;
pub const S5P_SECSS_MEM_LOWPWR: c_uint = 0x11D8;
pub const S5P_PAD_RETENTION_DRAM_LOWPWR: c_uint = 0x1200;
pub const S5P_PAD_RETENTION_MAUDIO_LOWPWR: c_uint = 0x1204;
pub const S5P_PAD_RETENTION_GPIO_LOWPWR: c_uint = 0x1220;
pub const S5P_PAD_RETENTION_UART_LOWPWR: c_uint = 0x1224;
pub const S5P_PAD_RETENTION_MMCA_LOWPWR: c_uint = 0x1228;
pub const S5P_PAD_RETENTION_MMCB_LOWPWR: c_uint = 0x122C;
pub const S5P_PAD_RETENTION_EBIA_LOWPWR: c_uint = 0x1230;
pub const S5P_PAD_RETENTION_EBIB_LOWPWR: c_uint = 0x1234;
pub const S5P_PAD_RETENTION_ISOLATION_LOWPWR: c_uint = 0x1240;
pub const S5P_PAD_RETENTION_ALV_SEL_LOWPWR: c_uint = 0x1260;
pub const S5P_XUSBXTI_LOWPWR: c_uint = 0x1280;
pub const S5P_XXTI_LOWPWR: c_uint = 0x1284;
pub const S5P_EXT_REGULATOR_LOWPWR: c_uint = 0x12C0;
pub const S5P_GPIO_MODE_LOWPWR: c_uint = 0x1300;
pub const S5P_GPIO_MODE_MAUDIO_LOWPWR: c_uint = 0x1340;
pub const S5P_CAM_LOWPWR: c_uint = 0x1380;
pub const S5P_TV_LOWPWR: c_uint = 0x1384;
pub const S5P_MFC_LOWPWR: c_uint = 0x1388;
pub const S5P_G3D_LOWPWR: c_uint = 0x138C;
pub const S5P_LCD0_LOWPWR: c_uint = 0x1390;
pub const S5P_MAUDIO_LOWPWR: c_uint = 0x1398;
pub const S5P_GPS_LOWPWR: c_uint = 0x139C;
pub const S5P_GPS_ALIVE_LOWPWR: c_uint = 0x13A0;
pub const EXYNOS_ARM_CORE0_CONFIGURATION: c_uint = 0x2000;

pub const EXYNOS_ARM_COMMON_CONFIGURATION: c_uint = 0x2500;

pub const EXYNOS_ARM_L2_CONFIGURATION: c_uint = 0x2600;

pub const S5P_PAD_RET_MAUDIO_OPTION: c_uint = 0x3028;
pub const S5P_PAD_RET_MMC2_OPTION: c_uint = 0x30c8;
pub const S5P_PAD_RET_GPIO_OPTION: c_uint = 0x3108;
pub const S5P_PAD_RET_UART_OPTION: c_uint = 0x3128;
pub const S5P_PAD_RET_MMCA_OPTION: c_uint = 0x3148;
pub const S5P_PAD_RET_MMCB_OPTION: c_uint = 0x3168;
pub const S5P_PAD_RET_EBIA_OPTION: c_uint = 0x3188;
pub const S5P_PAD_RET_EBIB_OPTION: c_uint = 0x31A8;
pub const S5P_PAD_RET_SPI_OPTION: c_uint = 0x31c8;
pub const S5P_PS_HOLD_CONTROL: c_uint = 0x330C;

pub const S5P_CAM_OPTION: c_uint = 0x3C08;
pub const S5P_MFC_OPTION: c_uint = 0x3C48;
pub const S5P_G3D_OPTION: c_uint = 0x3C68;
pub const S5P_LCD0_OPTION: c_uint = 0x3C88;
pub const S5P_LCD1_OPTION: c_uint = 0x3CA8;

pub const S5P_CORE_LOCAL_PWR_EN: c_uint = 0x3;

// Only for S5Pv210
pub const S5PV210_EINT_WAKEUP_MASK: c_uint = 0xC004;
// Only for Exynos2200
pub const EXYNOS2200_PHY_CTRL_USB20: c_uint = 0x72C;
// Only for Exynos4210
pub const S5P_CMU_CLKSTOP_LCD1_LOWPWR: c_uint = 0x1154;
pub const S5P_CMU_RESET_LCD1_LOWPWR: c_uint = 0x1174;
pub const S5P_MODIMIF_MEM_LOWPWR: c_uint = 0x11C4;
pub const S5P_PCIE_MEM_LOWPWR: c_uint = 0x11E0;
pub const S5P_SATA_MEM_LOWPWR: c_uint = 0x11E4;
pub const S5P_LCD1_LOWPWR: c_uint = 0x1394;
// Only for Exynos4x12
pub const S5P_ISP_ARM_LOWPWR: c_uint = 0x1050;
pub const S5P_DIS_IRQ_ISP_ARM_LOCAL_LOWPWR: c_uint = 0x1054;
pub const S5P_DIS_IRQ_ISP_ARM_CENTRAL_LOWPWR: c_uint = 0x1058;
pub const S5P_CMU_ACLKSTOP_COREBLK_LOWPWR: c_uint = 0x1110;
pub const S5P_CMU_SCLKSTOP_COREBLK_LOWPWR: c_uint = 0x1114;
pub const S5P_CMU_RESET_COREBLK_LOWPWR: c_uint = 0x111C;
pub const S5P_MPLLUSER_SYSCLK_LOWPWR: c_uint = 0x1130;
pub const S5P_CMU_CLKSTOP_ISP_LOWPWR: c_uint = 0x1154;
pub const S5P_CMU_RESET_ISP_LOWPWR: c_uint = 0x1174;
pub const S5P_TOP_BUS_COREBLK_LOWPWR: c_uint = 0x1190;
pub const S5P_TOP_RETENTION_COREBLK_LOWPWR: c_uint = 0x1194;
pub const S5P_TOP_PWR_COREBLK_LOWPWR: c_uint = 0x1198;
pub const S5P_OSCCLK_GATE_LOWPWR: c_uint = 0x11A4;
pub const S5P_LOGIC_RESET_COREBLK_LOWPWR: c_uint = 0x11B0;
pub const S5P_OSCCLK_GATE_COREBLK_LOWPWR: c_uint = 0x11B4;
pub const S5P_HSI_MEM_LOWPWR: c_uint = 0x11C4;
pub const S5P_ROTATOR_MEM_LOWPWR: c_uint = 0x11DC;
pub const S5P_PAD_RETENTION_GPIO_COREBLK_LOWPWR: c_uint = 0x123C;
pub const S5P_PAD_ISOLATION_COREBLK_LOWPWR: c_uint = 0x1250;
pub const S5P_GPIO_MODE_COREBLK_LOWPWR: c_uint = 0x1320;
pub const S5P_TOP_ASB_RESET_LOWPWR: c_uint = 0x1344;
pub const S5P_TOP_ASB_ISOLATION_LOWPWR: c_uint = 0x1348;
pub const S5P_ISP_LOWPWR: c_uint = 0x1394;
pub const S5P_DRAM_FREQ_DOWN_LOWPWR: c_uint = 0x13B0;
pub const S5P_DDRPHY_DLLOFF_LOWPWR: c_uint = 0x13B4;
pub const S5P_CMU_SYSCLK_ISP_LOWPWR: c_uint = 0x13B8;
pub const S5P_CMU_SYSCLK_GPS_LOWPWR: c_uint = 0x13BC;
pub const S5P_LPDDR_PHY_DLL_LOCK_LOWPWR: c_uint = 0x13C0;
pub const S5P_ARM_L2_0_OPTION: c_uint = 0x2608;
pub const S5P_ARM_L2_1_OPTION: c_uint = 0x2628;
pub const S5P_ONENAND_MEM_OPTION: c_uint = 0x2E08;
pub const S5P_HSI_MEM_OPTION: c_uint = 0x2E28;
pub const S5P_G2D_ACP_MEM_OPTION: c_uint = 0x2E48;
pub const S5P_USBOTG_MEM_OPTION: c_uint = 0x2E68;
pub const S5P_HSMMC_MEM_OPTION: c_uint = 0x2E88;
pub const S5P_CSSYS_MEM_OPTION: c_uint = 0x2EA8;
pub const S5P_SECSS_MEM_OPTION: c_uint = 0x2EC8;
pub const S5P_ROTATOR_MEM_OPTION: c_uint = 0x2F48;
// Only for Exynos4412
pub const S5P_ARM_CORE2_LOWPWR: c_uint = 0x1020;
pub const S5P_DIS_IRQ_CORE2: c_uint = 0x1024;
pub const S5P_DIS_IRQ_CENTRAL2: c_uint = 0x1028;
pub const S5P_ARM_CORE3_LOWPWR: c_uint = 0x1030;
pub const S5P_DIS_IRQ_CORE3: c_uint = 0x1034;
pub const S5P_DIS_IRQ_CENTRAL3: c_uint = 0x1038;
// Only for Exynos3XXX
pub const EXYNOS3_ARM_CORE0_SYS_PWR_REG: c_uint = 0x1000;
pub const EXYNOS3_DIS_IRQ_ARM_CORE0_LOCAL_SYS_PWR_REG: c_uint = 0x1004;
pub const EXYNOS3_DIS_IRQ_ARM_CORE0_CENTRAL_SYS_PWR_REG: c_uint = 0x1008;
pub const EXYNOS3_ARM_CORE1_SYS_PWR_REG: c_uint = 0x1010;
pub const EXYNOS3_DIS_IRQ_ARM_CORE1_LOCAL_SYS_PWR_REG: c_uint = 0x1014;
pub const EXYNOS3_DIS_IRQ_ARM_CORE1_CENTRAL_SYS_PWR_REG: c_uint = 0x1018;
pub const EXYNOS3_ISP_ARM_SYS_PWR_REG: c_uint = 0x1050;
pub const EXYNOS3_DIS_IRQ_ISP_ARM_LOCAL_SYS_PWR_REG: c_uint = 0x1054;
pub const EXYNOS3_DIS_IRQ_ISP_ARM_CENTRAL_SYS_PWR_REG: c_uint = 0x1058;
pub const EXYNOS3_ARM_COMMON_SYS_PWR_REG: c_uint = 0x1080;
pub const EXYNOS3_ARM_L2_SYS_PWR_REG: c_uint = 0x10C0;
pub const EXYNOS3_CMU_ACLKSTOP_SYS_PWR_REG: c_uint = 0x1100;
pub const EXYNOS3_CMU_SCLKSTOP_SYS_PWR_REG: c_uint = 0x1104;
pub const EXYNOS3_CMU_RESET_SYS_PWR_REG: c_uint = 0x110C;
pub const EXYNOS3_CMU_ACLKSTOP_COREBLK_SYS_PWR_REG: c_uint = 0x1110;
pub const EXYNOS3_CMU_SCLKSTOP_COREBLK_SYS_PWR_REG: c_uint = 0x1114;
pub const EXYNOS3_CMU_RESET_COREBLK_SYS_PWR_REG: c_uint = 0x111C;
pub const EXYNOS3_APLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1120;
pub const EXYNOS3_MPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1124;
pub const EXYNOS3_VPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1128;
pub const EXYNOS3_EPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x112C;
pub const EXYNOS3_MPLLUSER_SYSCLK_SYS_PWR_REG: c_uint = 0x1130;
pub const EXYNOS3_BPLLUSER_SYSCLK_SYS_PWR_REG: c_uint = 0x1134;
pub const EXYNOS3_EPLLUSER_SYSCLK_SYS_PWR_REG: c_uint = 0x1138;
pub const EXYNOS3_CMU_CLKSTOP_CAM_SYS_PWR_REG: c_uint = 0x1140;
pub const EXYNOS3_CMU_CLKSTOP_MFC_SYS_PWR_REG: c_uint = 0x1148;
pub const EXYNOS3_CMU_CLKSTOP_G3D_SYS_PWR_REG: c_uint = 0x114C;
pub const EXYNOS3_CMU_CLKSTOP_LCD0_SYS_PWR_REG: c_uint = 0x1150;
pub const EXYNOS3_CMU_CLKSTOP_ISP_SYS_PWR_REG: c_uint = 0x1154;
pub const EXYNOS3_CMU_CLKSTOP_MAUDIO_SYS_PWR_REG: c_uint = 0x1158;
pub const EXYNOS3_CMU_RESET_CAM_SYS_PWR_REG: c_uint = 0x1160;
pub const EXYNOS3_CMU_RESET_MFC_SYS_PWR_REG: c_uint = 0x1168;
pub const EXYNOS3_CMU_RESET_G3D_SYS_PWR_REG: c_uint = 0x116C;
pub const EXYNOS3_CMU_RESET_LCD0_SYS_PWR_REG: c_uint = 0x1170;
pub const EXYNOS3_CMU_RESET_ISP_SYS_PWR_REG: c_uint = 0x1174;
pub const EXYNOS3_CMU_RESET_MAUDIO_SYS_PWR_REG: c_uint = 0x1178;
pub const EXYNOS3_TOP_BUS_SYS_PWR_REG: c_uint = 0x1180;
pub const EXYNOS3_TOP_RETENTION_SYS_PWR_REG: c_uint = 0x1184;
pub const EXYNOS3_TOP_PWR_SYS_PWR_REG: c_uint = 0x1188;
pub const EXYNOS3_TOP_BUS_COREBLK_SYS_PWR_REG: c_uint = 0x1190;
pub const EXYNOS3_TOP_RETENTION_COREBLK_SYS_PWR_REG: c_uint = 0x1194;
pub const EXYNOS3_TOP_PWR_COREBLK_SYS_PWR_REG: c_uint = 0x1198;
pub const EXYNOS3_LOGIC_RESET_SYS_PWR_REG: c_uint = 0x11A0;
pub const EXYNOS3_OSCCLK_GATE_SYS_PWR_REG: c_uint = 0x11A4;
pub const EXYNOS3_LOGIC_RESET_COREBLK_SYS_PWR_REG: c_uint = 0x11B0;
pub const EXYNOS3_OSCCLK_GATE_COREBLK_SYS_PWR_REG: c_uint = 0x11B4;
pub const EXYNOS3_PAD_RETENTION_DRAM_SYS_PWR_REG: c_uint = 0x1200;
pub const EXYNOS3_PAD_RETENTION_MAUDIO_SYS_PWR_REG: c_uint = 0x1204;
pub const EXYNOS3_PAD_RETENTION_SPI_SYS_PWR_REG: c_uint = 0x1208;
pub const EXYNOS3_PAD_RETENTION_MMC2_SYS_PWR_REG: c_uint = 0x1218;
pub const EXYNOS3_PAD_RETENTION_GPIO_SYS_PWR_REG: c_uint = 0x1220;
pub const EXYNOS3_PAD_RETENTION_UART_SYS_PWR_REG: c_uint = 0x1224;
pub const EXYNOS3_PAD_RETENTION_MMC0_SYS_PWR_REG: c_uint = 0x1228;
pub const EXYNOS3_PAD_RETENTION_MMC1_SYS_PWR_REG: c_uint = 0x122C;
pub const EXYNOS3_PAD_RETENTION_EBIA_SYS_PWR_REG: c_uint = 0x1230;
pub const EXYNOS3_PAD_RETENTION_EBIB_SYS_PWR_REG: c_uint = 0x1234;
pub const EXYNOS3_PAD_RETENTION_JTAG_SYS_PWR_REG: c_uint = 0x1238;
pub const EXYNOS3_PAD_ISOLATION_SYS_PWR_REG: c_uint = 0x1240;
pub const EXYNOS3_PAD_ALV_SEL_SYS_PWR_REG: c_uint = 0x1260;
pub const EXYNOS3_XUSBXTI_SYS_PWR_REG: c_uint = 0x1280;
pub const EXYNOS3_XXTI_SYS_PWR_REG: c_uint = 0x1284;
pub const EXYNOS3_EXT_REGULATOR_SYS_PWR_REG: c_uint = 0x12C0;
pub const EXYNOS3_EXT_REGULATOR_COREBLK_SYS_PWR_REG: c_uint = 0x12C4;
pub const EXYNOS3_GPIO_MODE_SYS_PWR_REG: c_uint = 0x1300;
pub const EXYNOS3_GPIO_MODE_MAUDIO_SYS_PWR_REG: c_uint = 0x1340;
pub const EXYNOS3_TOP_ASB_RESET_SYS_PWR_REG: c_uint = 0x1344;
pub const EXYNOS3_TOP_ASB_ISOLATION_SYS_PWR_REG: c_uint = 0x1348;
pub const EXYNOS3_TOP_ASB_RESET_COREBLK_SYS_PWR_REG: c_uint = 0x1350;
pub const EXYNOS3_TOP_ASB_ISOLATION_COREBLK_SYS_PWR_REG: c_uint = 0x1354;
pub const EXYNOS3_CAM_SYS_PWR_REG: c_uint = 0x1380;
pub const EXYNOS3_MFC_SYS_PWR_REG: c_uint = 0x1388;
pub const EXYNOS3_G3D_SYS_PWR_REG: c_uint = 0x138C;
pub const EXYNOS3_LCD0_SYS_PWR_REG: c_uint = 0x1390;
pub const EXYNOS3_ISP_SYS_PWR_REG: c_uint = 0x1394;
pub const EXYNOS3_MAUDIO_SYS_PWR_REG: c_uint = 0x1398;
pub const EXYNOS3_DRAM_FREQ_DOWN_SYS_PWR_REG: c_uint = 0x13B0;
pub const EXYNOS3_DDRPHY_DLLOFF_SYS_PWR_REG: c_uint = 0x13B4;
pub const EXYNOS3_CMU_SYSCLK_ISP_SYS_PWR_REG: c_uint = 0x13B8;
pub const EXYNOS3_LPDDR_PHY_DLL_LOCK_SYS_PWR_REG: c_uint = 0x13C0;
pub const EXYNOS3_BPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x13C4;
pub const EXYNOS3_UPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x13C8;
pub const EXYNOS3_ARM_CORE0_OPTION: c_uint = 0x2008;

pub const EXYNOS3_ARM_COMMON_OPTION: c_uint = 0x2408;
pub const EXYNOS3_ARM_L2_OPTION: c_uint = 0x2608;
pub const EXYNOS3_TOP_PWR_OPTION: c_uint = 0x2C48;
pub const EXYNOS3_CORE_TOP_PWR_OPTION: c_uint = 0x2CA8;
pub const EXYNOS3_XUSBXTI_DURATION: c_uint = 0x341C;
pub const EXYNOS3_XXTI_DURATION: c_uint = 0x343C;
pub const EXYNOS3_EXT_REGULATOR_DURATION: c_uint = 0x361C;
pub const EXYNOS3_EXT_REGULATOR_COREBLK_DURATION: c_uint = 0x363C;
pub const XUSBXTI_DURATION: c_uint = 0x00000BB8;

pub const EXT_REGULATOR_DURATION: c_uint = 0x00001D4C;

// for XXX_OPTION

// For Exynos5
pub const EXYNOS5_AUTO_WDTRESET_DISABLE: c_uint = 0x0408;
pub const EXYNOS5_MASK_WDTRESET_REQUEST: c_uint = 0x040C;
pub const EXYNOS5_USBDRD_PHY_CONTROL: c_uint = 0x0704;
pub const EXYNOS5_DPTX_PHY_CONTROL: c_uint = 0x0720;

pub const EXYNOS5_ARM_CORE0_SYS_PWR_REG: c_uint = 0x1000;
pub const EXYNOS5_DIS_IRQ_ARM_CORE0_LOCAL_SYS_PWR_REG: c_uint = 0x1004;
pub const EXYNOS5_DIS_IRQ_ARM_CORE0_CENTRAL_SYS_PWR_REG: c_uint = 0x1008;
pub const EXYNOS5_ARM_CORE1_SYS_PWR_REG: c_uint = 0x1010;
pub const EXYNOS5_DIS_IRQ_ARM_CORE1_LOCAL_SYS_PWR_REG: c_uint = 0x1014;
pub const EXYNOS5_DIS_IRQ_ARM_CORE1_CENTRAL_SYS_PWR_REG: c_uint = 0x1018;
pub const EXYNOS5_FSYS_ARM_SYS_PWR_REG: c_uint = 0x1040;
pub const EXYNOS5_DIS_IRQ_FSYS_ARM_CENTRAL_SYS_PWR_REG: c_uint = 0x1048;
pub const EXYNOS5_ISP_ARM_SYS_PWR_REG: c_uint = 0x1050;
pub const EXYNOS5_DIS_IRQ_ISP_ARM_LOCAL_SYS_PWR_REG: c_uint = 0x1054;
pub const EXYNOS5_DIS_IRQ_ISP_ARM_CENTRAL_SYS_PWR_REG: c_uint = 0x1058;
pub const EXYNOS5_ARM_COMMON_SYS_PWR_REG: c_uint = 0x1080;
pub const EXYNOS5_ARM_L2_SYS_PWR_REG: c_uint = 0x10C0;
pub const EXYNOS5_CMU_ACLKSTOP_SYS_PWR_REG: c_uint = 0x1100;
pub const EXYNOS5_CMU_SCLKSTOP_SYS_PWR_REG: c_uint = 0x1104;
pub const EXYNOS5_CMU_RESET_SYS_PWR_REG: c_uint = 0x110C;
pub const EXYNOS5_CMU_ACLKSTOP_SYSMEM_SYS_PWR_REG: c_uint = 0x1120;
pub const EXYNOS5_CMU_SCLKSTOP_SYSMEM_SYS_PWR_REG: c_uint = 0x1124;
pub const EXYNOS5_CMU_RESET_SYSMEM_SYS_PWR_REG: c_uint = 0x112C;
pub const EXYNOS5_DRAM_FREQ_DOWN_SYS_PWR_REG: c_uint = 0x1130;
pub const EXYNOS5_DDRPHY_DLLOFF_SYS_PWR_REG: c_uint = 0x1134;
pub const EXYNOS5_DDRPHY_DLLLOCK_SYS_PWR_REG: c_uint = 0x1138;
pub const EXYNOS5_APLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1140;
pub const EXYNOS5_MPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1144;
pub const EXYNOS5_VPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1148;
pub const EXYNOS5_EPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x114C;
pub const EXYNOS5_BPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1150;
pub const EXYNOS5_CPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1154;
pub const EXYNOS5_MPLLUSER_SYSCLK_SYS_PWR_REG: c_uint = 0x1164;
pub const EXYNOS5_BPLLUSER_SYSCLK_SYS_PWR_REG: c_uint = 0x1170;
pub const EXYNOS5_TOP_BUS_SYS_PWR_REG: c_uint = 0x1180;
pub const EXYNOS5_TOP_RETENTION_SYS_PWR_REG: c_uint = 0x1184;
pub const EXYNOS5_TOP_PWR_SYS_PWR_REG: c_uint = 0x1188;
pub const EXYNOS5_TOP_BUS_SYSMEM_SYS_PWR_REG: c_uint = 0x1190;
pub const EXYNOS5_TOP_RETENTION_SYSMEM_SYS_PWR_REG: c_uint = 0x1194;
pub const EXYNOS5_TOP_PWR_SYSMEM_SYS_PWR_REG: c_uint = 0x1198;
pub const EXYNOS5_LOGIC_RESET_SYS_PWR_REG: c_uint = 0x11A0;
pub const EXYNOS5_OSCCLK_GATE_SYS_PWR_REG: c_uint = 0x11A4;
pub const EXYNOS5_LOGIC_RESET_SYSMEM_SYS_PWR_REG: c_uint = 0x11B0;
pub const EXYNOS5_OSCCLK_GATE_SYSMEM_SYS_PWR_REG: c_uint = 0x11B4;
pub const EXYNOS5_USBOTG_MEM_SYS_PWR_REG: c_uint = 0x11C0;
pub const EXYNOS5_G2D_MEM_SYS_PWR_REG: c_uint = 0x11C8;
pub const EXYNOS5_USBDRD_MEM_SYS_PWR_REG: c_uint = 0x11CC;
pub const EXYNOS5_SDMMC_MEM_SYS_PWR_REG: c_uint = 0x11D0;
pub const EXYNOS5_CSSYS_MEM_SYS_PWR_REG: c_uint = 0x11D4;
pub const EXYNOS5_SECSS_MEM_SYS_PWR_REG: c_uint = 0x11D8;
pub const EXYNOS5_ROTATOR_MEM_SYS_PWR_REG: c_uint = 0x11DC;
pub const EXYNOS5_INTRAM_MEM_SYS_PWR_REG: c_uint = 0x11E0;
pub const EXYNOS5_INTROM_MEM_SYS_PWR_REG: c_uint = 0x11E4;
pub const EXYNOS5_JPEG_MEM_SYS_PWR_REG: c_uint = 0x11E8;
pub const EXYNOS5_HSI_MEM_SYS_PWR_REG: c_uint = 0x11EC;
pub const EXYNOS5_MCUIOP_MEM_SYS_PWR_REG: c_uint = 0x11F4;
pub const EXYNOS5_SATA_MEM_SYS_PWR_REG: c_uint = 0x11FC;
pub const EXYNOS5_PAD_RETENTION_DRAM_SYS_PWR_REG: c_uint = 0x1200;
pub const EXYNOS5_PAD_RETENTION_MAU_SYS_PWR_REG: c_uint = 0x1204;
pub const EXYNOS5_PAD_RETENTION_GPIO_SYS_PWR_REG: c_uint = 0x1220;
pub const EXYNOS5_PAD_RETENTION_UART_SYS_PWR_REG: c_uint = 0x1224;
pub const EXYNOS5_PAD_RETENTION_MMCA_SYS_PWR_REG: c_uint = 0x1228;
pub const EXYNOS5_PAD_RETENTION_MMCB_SYS_PWR_REG: c_uint = 0x122C;
pub const EXYNOS5_PAD_RETENTION_EBIA_SYS_PWR_REG: c_uint = 0x1230;
pub const EXYNOS5_PAD_RETENTION_EBIB_SYS_PWR_REG: c_uint = 0x1234;
pub const EXYNOS5_PAD_RETENTION_SPI_SYS_PWR_REG: c_uint = 0x1238;
pub const EXYNOS5_PAD_RETENTION_GPIO_SYSMEM_SYS_PWR_REG: c_uint = 0x123C;
pub const EXYNOS5_PAD_ISOLATION_SYS_PWR_REG: c_uint = 0x1240;
pub const EXYNOS5_PAD_ISOLATION_SYSMEM_SYS_PWR_REG: c_uint = 0x1250;
pub const EXYNOS5_PAD_ALV_SEL_SYS_PWR_REG: c_uint = 0x1260;
pub const EXYNOS5_XUSBXTI_SYS_PWR_REG: c_uint = 0x1280;
pub const EXYNOS5_XXTI_SYS_PWR_REG: c_uint = 0x1284;
pub const EXYNOS5_EXT_REGULATOR_SYS_PWR_REG: c_uint = 0x12C0;
pub const EXYNOS5_GPIO_MODE_SYS_PWR_REG: c_uint = 0x1300;
pub const EXYNOS5_GPIO_MODE_SYSMEM_SYS_PWR_REG: c_uint = 0x1320;
pub const EXYNOS5_GPIO_MODE_MAU_SYS_PWR_REG: c_uint = 0x1340;
pub const EXYNOS5_TOP_ASB_RESET_SYS_PWR_REG: c_uint = 0x1344;
pub const EXYNOS5_TOP_ASB_ISOLATION_SYS_PWR_REG: c_uint = 0x1348;
pub const EXYNOS5_GSCL_SYS_PWR_REG: c_uint = 0x1400;
pub const EXYNOS5_ISP_SYS_PWR_REG: c_uint = 0x1404;
pub const EXYNOS5_MFC_SYS_PWR_REG: c_uint = 0x1408;
pub const EXYNOS5_G3D_SYS_PWR_REG: c_uint = 0x140C;
pub const EXYNOS5_DISP1_SYS_PWR_REG: c_uint = 0x1414;
pub const EXYNOS5_MAU_SYS_PWR_REG: c_uint = 0x1418;
pub const EXYNOS5_CMU_CLKSTOP_GSCL_SYS_PWR_REG: c_uint = 0x1480;
pub const EXYNOS5_CMU_CLKSTOP_ISP_SYS_PWR_REG: c_uint = 0x1484;
pub const EXYNOS5_CMU_CLKSTOP_MFC_SYS_PWR_REG: c_uint = 0x1488;
pub const EXYNOS5_CMU_CLKSTOP_G3D_SYS_PWR_REG: c_uint = 0x148C;
pub const EXYNOS5_CMU_CLKSTOP_DISP1_SYS_PWR_REG: c_uint = 0x1494;
pub const EXYNOS5_CMU_CLKSTOP_MAU_SYS_PWR_REG: c_uint = 0x1498;
pub const EXYNOS5_CMU_SYSCLK_GSCL_SYS_PWR_REG: c_uint = 0x14C0;
pub const EXYNOS5_CMU_SYSCLK_ISP_SYS_PWR_REG: c_uint = 0x14C4;
pub const EXYNOS5_CMU_SYSCLK_MFC_SYS_PWR_REG: c_uint = 0x14C8;
pub const EXYNOS5_CMU_SYSCLK_G3D_SYS_PWR_REG: c_uint = 0x14CC;
pub const EXYNOS5_CMU_SYSCLK_DISP1_SYS_PWR_REG: c_uint = 0x14D4;
pub const EXYNOS5_CMU_SYSCLK_MAU_SYS_PWR_REG: c_uint = 0x14D8;
pub const EXYNOS5_CMU_RESET_GSCL_SYS_PWR_REG: c_uint = 0x1580;
pub const EXYNOS5_CMU_RESET_ISP_SYS_PWR_REG: c_uint = 0x1584;
pub const EXYNOS5_CMU_RESET_MFC_SYS_PWR_REG: c_uint = 0x1588;
pub const EXYNOS5_CMU_RESET_G3D_SYS_PWR_REG: c_uint = 0x158C;
pub const EXYNOS5_CMU_RESET_DISP1_SYS_PWR_REG: c_uint = 0x1594;
pub const EXYNOS5_CMU_RESET_MAU_SYS_PWR_REG: c_uint = 0x1598;
pub const EXYNOS5_ARM_CORE0_OPTION: c_uint = 0x2008;
pub const EXYNOS5_ARM_CORE1_OPTION: c_uint = 0x2088;
pub const EXYNOS5_FSYS_ARM_OPTION: c_uint = 0x2208;
pub const EXYNOS5_ISP_ARM_OPTION: c_uint = 0x2288;
pub const EXYNOS5_ARM_COMMON_OPTION: c_uint = 0x2408;
pub const EXYNOS5_ARM_L2_OPTION: c_uint = 0x2608;
pub const EXYNOS5_TOP_PWR_OPTION: c_uint = 0x2C48;
pub const EXYNOS5_TOP_PWR_SYSMEM_OPTION: c_uint = 0x2CC8;
pub const EXYNOS5_JPEG_MEM_OPTION: c_uint = 0x2F48;
pub const EXYNOS5_GSCL_OPTION: c_uint = 0x4008;
pub const EXYNOS5_ISP_OPTION: c_uint = 0x4028;
pub const EXYNOS5_MFC_OPTION: c_uint = 0x4048;
pub const EXYNOS5_G3D_OPTION: c_uint = 0x4068;
pub const EXYNOS5_DISP1_OPTION: c_uint = 0x40A8;
pub const EXYNOS5_MAU_OPTION: c_uint = 0x40C8;

pub const EXYNOS5420_SWRESET_KFC_SEL: c_uint = 0x3;
// Only for Exynos5420

pub const EXYNOS5420_LPI_MASK: c_uint = 0x0004;
pub const EXYNOS5420_LPI_MASK1: c_uint = 0x0008;

pub const EXYNOS5420_ARM_INTR_SPREAD_ENABLE: c_uint = 0x0100;
pub const EXYNOS5420_ARM_INTR_SPREAD_USE_STANDBYWFI: c_uint = 0x0104;
pub const EXYNOS5420_UP_SCHEDULER: c_uint = 0x0120;
pub const SPREAD_ENABLE: c_uint = 0xF;
pub const SPREAD_USE_STANDWFI: c_uint = 0xF;

pub const EXYNOS5420_USBDRD1_PHY_CONTROL: c_uint = 0x0708;

pub const EXYNOS5420_DPTX_PHY_CONTROL: c_uint = 0x0728;
pub const EXYNOS5420_ARM_CORE2_SYS_PWR_REG: c_uint = 0x1020;
pub const EXYNOS5420_DIS_IRQ_ARM_CORE2_LOCAL_SYS_PWR_REG: c_uint = 0x1024;
pub const EXYNOS5420_DIS_IRQ_ARM_CORE2_CENTRAL_SYS_PWR_REG: c_uint = 0x1028;
pub const EXYNOS5420_ARM_CORE3_SYS_PWR_REG: c_uint = 0x1030;
pub const EXYNOS5420_DIS_IRQ_ARM_CORE3_LOCAL_SYS_PWR_REG: c_uint = 0x1034;
pub const EXYNOS5420_DIS_IRQ_ARM_CORE3_CENTRAL_SYS_PWR_REG: c_uint = 0x1038;
pub const EXYNOS5420_KFC_CORE0_SYS_PWR_REG: c_uint = 0x1040;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE0_LOCAL_SYS_PWR_REG: c_uint = 0x1044;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE0_CENTRAL_SYS_PWR_REG: c_uint = 0x1048;
pub const EXYNOS5420_KFC_CORE1_SYS_PWR_REG: c_uint = 0x1050;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE1_LOCAL_SYS_PWR_REG: c_uint = 0x1054;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE1_CENTRAL_SYS_PWR_REG: c_uint = 0x1058;
pub const EXYNOS5420_KFC_CORE2_SYS_PWR_REG: c_uint = 0x1060;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE2_LOCAL_SYS_PWR_REG: c_uint = 0x1064;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE2_CENTRAL_SYS_PWR_REG: c_uint = 0x1068;
pub const EXYNOS5420_KFC_CORE3_SYS_PWR_REG: c_uint = 0x1070;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE3_LOCAL_SYS_PWR_REG: c_uint = 0x1074;
pub const EXYNOS5420_DIS_IRQ_KFC_CORE3_CENTRAL_SYS_PWR_REG: c_uint = 0x1078;
pub const EXYNOS5420_ISP_ARM_SYS_PWR_REG: c_uint = 0x1090;
pub const EXYNOS5420_DIS_IRQ_ISP_ARM_LOCAL_SYS_PWR_REG: c_uint = 0x1094;
pub const EXYNOS5420_DIS_IRQ_ISP_ARM_CENTRAL_SYS_PWR_REG: c_uint = 0x1098;
pub const EXYNOS5420_ARM_COMMON_SYS_PWR_REG: c_uint = 0x10A0;
pub const EXYNOS5420_KFC_COMMON_SYS_PWR_REG: c_uint = 0x10B0;
pub const EXYNOS5420_KFC_L2_SYS_PWR_REG: c_uint = 0x10D0;
pub const EXYNOS5420_DPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1158;
pub const EXYNOS5420_IPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x115C;
pub const EXYNOS5420_KPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1160;
pub const EXYNOS5420_RPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1174;
pub const EXYNOS5420_SPLL_SYSCLK_SYS_PWR_REG: c_uint = 0x1178;
pub const EXYNOS5420_INTRAM_MEM_SYS_PWR_REG: c_uint = 0x11B8;
pub const EXYNOS5420_INTROM_MEM_SYS_PWR_REG: c_uint = 0x11BC;
pub const EXYNOS5420_PAD_RETENTION_JTAG_SYS_PWR_REG: c_uint = 0x1208;
pub const EXYNOS5420_PAD_RETENTION_DRAM_SYS_PWR_REG: c_uint = 0x1210;
pub const EXYNOS5420_PAD_RETENTION_UART_SYS_PWR_REG: c_uint = 0x1214;
pub const EXYNOS5420_PAD_RETENTION_MMC0_SYS_PWR_REG: c_uint = 0x1218;
pub const EXYNOS5420_PAD_RETENTION_MMC1_SYS_PWR_REG: c_uint = 0x121C;
pub const EXYNOS5420_PAD_RETENTION_MMC2_SYS_PWR_REG: c_uint = 0x1220;
pub const EXYNOS5420_PAD_RETENTION_HSI_SYS_PWR_REG: c_uint = 0x1224;
pub const EXYNOS5420_PAD_RETENTION_EBIA_SYS_PWR_REG: c_uint = 0x1228;
pub const EXYNOS5420_PAD_RETENTION_EBIB_SYS_PWR_REG: c_uint = 0x122C;
pub const EXYNOS5420_PAD_RETENTION_SPI_SYS_PWR_REG: c_uint = 0x1230;
pub const EXYNOS5420_PAD_RETENTION_DRAM_COREBLK_SYS_PWR_REG: c_uint = 0x1234;
pub const EXYNOS5420_DISP1_SYS_PWR_REG: c_uint = 0x1410;
pub const EXYNOS5420_MAU_SYS_PWR_REG: c_uint = 0x1414;
pub const EXYNOS5420_G2D_SYS_PWR_REG: c_uint = 0x1418;
pub const EXYNOS5420_MSC_SYS_PWR_REG: c_uint = 0x141C;
pub const EXYNOS5420_FSYS_SYS_PWR_REG: c_uint = 0x1420;
pub const EXYNOS5420_FSYS2_SYS_PWR_REG: c_uint = 0x1424;
pub const EXYNOS5420_PSGEN_SYS_PWR_REG: c_uint = 0x1428;
pub const EXYNOS5420_PERIC_SYS_PWR_REG: c_uint = 0x142C;
pub const EXYNOS5420_WCORE_SYS_PWR_REG: c_uint = 0x1430;
pub const EXYNOS5420_CMU_CLKSTOP_DISP1_SYS_PWR_REG: c_uint = 0x1490;
pub const EXYNOS5420_CMU_CLKSTOP_MAU_SYS_PWR_REG: c_uint = 0x1494;
pub const EXYNOS5420_CMU_CLKSTOP_G2D_SYS_PWR_REG: c_uint = 0x1498;
pub const EXYNOS5420_CMU_CLKSTOP_MSC_SYS_PWR_REG: c_uint = 0x149C;
pub const EXYNOS5420_CMU_CLKSTOP_FSYS_SYS_PWR_REG: c_uint = 0x14A0;
pub const EXYNOS5420_CMU_CLKSTOP_FSYS2_SYS_PWR_REG: c_uint = 0x14A4;
pub const EXYNOS5420_CMU_CLKSTOP_PSGEN_SYS_PWR_REG: c_uint = 0x14A8;
pub const EXYNOS5420_CMU_CLKSTOP_PERIC_SYS_PWR_REG: c_uint = 0x14AC;
pub const EXYNOS5420_CMU_CLKSTOP_WCORE_SYS_PWR_REG: c_uint = 0x14B0;
pub const EXYNOS5420_CMU_SYSCLK_TOPPWR_SYS_PWR_REG: c_uint = 0x14BC;
pub const EXYNOS5420_CMU_SYSCLK_DISP1_SYS_PWR_REG: c_uint = 0x14D0;
pub const EXYNOS5420_CMU_SYSCLK_MAU_SYS_PWR_REG: c_uint = 0x14D4;
pub const EXYNOS5420_CMU_SYSCLK_G2D_SYS_PWR_REG: c_uint = 0x14D8;
pub const EXYNOS5420_CMU_SYSCLK_MSC_SYS_PWR_REG: c_uint = 0x14DC;
pub const EXYNOS5420_CMU_SYSCLK_FSYS_SYS_PWR_REG: c_uint = 0x14E0;
pub const EXYNOS5420_CMU_SYSCLK_FSYS2_SYS_PWR_REG: c_uint = 0x14E4;
pub const EXYNOS5420_CMU_SYSCLK_PSGEN_SYS_PWR_REG: c_uint = 0x14E8;
pub const EXYNOS5420_CMU_SYSCLK_PERIC_SYS_PWR_REG: c_uint = 0x14EC;
pub const EXYNOS5420_CMU_SYSCLK_WCORE_SYS_PWR_REG: c_uint = 0x14F0;
pub const EXYNOS5420_CMU_SYSCLK_SYSMEM_TOPPWR_SYS_PWR_REG: c_uint = 0x14F4;
pub const EXYNOS5420_CMU_RESET_FSYS2_SYS_PWR_REG: c_uint = 0x1570;
pub const EXYNOS5420_CMU_RESET_PSGEN_SYS_PWR_REG: c_uint = 0x1574;
pub const EXYNOS5420_CMU_RESET_PERIC_SYS_PWR_REG: c_uint = 0x1578;
pub const EXYNOS5420_CMU_RESET_WCORE_SYS_PWR_REG: c_uint = 0x157C;
pub const EXYNOS5420_CMU_RESET_DISP1_SYS_PWR_REG: c_uint = 0x1590;
pub const EXYNOS5420_CMU_RESET_MAU_SYS_PWR_REG: c_uint = 0x1594;
pub const EXYNOS5420_CMU_RESET_G2D_SYS_PWR_REG: c_uint = 0x1598;
pub const EXYNOS5420_CMU_RESET_MSC_SYS_PWR_REG: c_uint = 0x159C;
pub const EXYNOS5420_CMU_RESET_FSYS_SYS_PWR_REG: c_uint = 0x15A0;
pub const EXYNOS5420_SFR_AXI_CGDIS1: c_uint = 0x15E4;
pub const EXYNOS5420_ARM_COMMON_OPTION: c_uint = 0x2508;
pub const EXYNOS5420_KFC_COMMON_OPTION: c_uint = 0x2588;
pub const EXYNOS5420_LOGIC_RESET_DURATION3: c_uint = 0x2D1C;
pub const EXYNOS5420_PAD_RET_GPIO_OPTION: c_uint = 0x30C8;
pub const EXYNOS5420_PAD_RET_UART_OPTION: c_uint = 0x30E8;
pub const EXYNOS5420_PAD_RET_MMCA_OPTION: c_uint = 0x3108;
pub const EXYNOS5420_PAD_RET_MMCB_OPTION: c_uint = 0x3128;
pub const EXYNOS5420_PAD_RET_MMCC_OPTION: c_uint = 0x3148;
pub const EXYNOS5420_PAD_RET_HSI_OPTION: c_uint = 0x3168;
pub const EXYNOS5420_PAD_RET_SPI_OPTION: c_uint = 0x31C8;
pub const EXYNOS5420_PAD_RET_DRAM_COREBLK_OPTION: c_uint = 0x31E8;
pub const EXYNOS_PAD_RET_DRAM_OPTION: c_uint = 0x3008;
pub const EXYNOS_PAD_RET_MAUDIO_OPTION: c_uint = 0x3028;
pub const EXYNOS_PAD_RET_JTAG_OPTION: c_uint = 0x3048;
pub const EXYNOS_PAD_RET_EBIA_OPTION: c_uint = 0x3188;
pub const EXYNOS_PAD_RET_EBIB_OPTION: c_uint = 0x31A8;
pub const EXYNOS5420_FSYS2_OPTION: c_uint = 0x4168;
pub const EXYNOS5420_PSGEN_OPTION: c_uint = 0x4188;

pub const DUR_WAIT_RESET: c_uint = 0xF;

// For Exynos5433

// For Exynos990

// For Exynos7870

// For Tensor GS101
// PMU ALIVE
pub const GS101_OM_STAT: c_uint = 0x0000;
pub const GS101_VERSION: c_uint = 0x0004;
pub const GS101_PORESET_CHECK: c_uint = 0x0008;
pub const GS101_OTP_STATUS: c_uint = 0x000c;
pub const GS101_SYSTEM_INFO: c_uint = 0x0010;

pub const GS101_INFORM0: c_uint = 0x0800;
pub const GS101_INFORM1: c_uint = 0x0804;
pub const GS101_INFORM2: c_uint = 0x0808;
pub const GS101_INFORM3: c_uint = 0x080c;

pub const GS101_PWR_HOLD_HW_TRIP: c_uint = 0x0820;
pub const GS101_PWR_HOLD_SW_TRIP: c_uint = 0x0824;

pub const GS101_INFORM4: c_uint = 0x0840;
pub const GS101_INFORM5: c_uint = 0x0844;
pub const GS101_INFORM6: c_uint = 0x0848;
pub const GS101_INFORM7: c_uint = 0x084c;
pub const GS101_INFORM8: c_uint = 0x0850;
pub const GS101_INFORM9: c_uint = 0x0854;
pub const GS101_INFORM10: c_uint = 0x0858;
pub const GS101_INFORM11: c_uint = 0x085c;

pub const GS101_IROM_INFORM: c_uint = 0x0880;

pub const GS101_IROM_PWRMODE: c_uint = 0x0990;

pub const GS101_CLUSTER0_OFFSET: c_uint = 0x1000;
pub const GS101_CLUSTER1_OFFSET: c_uint = 0x1300;
pub const GS101_CLUSTER2_OFFSET: c_uint = 0x1500;

pub const GS101_SUBBBLK_OFFSET_ALIVE: c_uint = 0x1800;
pub const GS101_SUBBBLK_OFFSET_AOC: c_uint = 0x1880;
pub const GS101_SUBBBLK_OFFSET_APM: c_uint = 0x1900;
pub const GS101_SUBBBLK_OFFSET_CMU: c_uint = 0x1980;
pub const GS101_SUBBBLK_OFFSET_BUS0: c_uint = 0x1a00;
pub const GS101_SUBBBLK_OFFSET_BUS1: c_uint = 0x1a80;
pub const GS101_SUBBBLK_OFFSET_BUS2: c_uint = 0x1b00;
pub const GS101_SUBBBLK_OFFSET_CORE: c_uint = 0x1b80;
pub const GS101_SUBBBLK_OFFSET_EH: c_uint = 0x1c00;
pub const GS101_SUBBBLK_OFFSET_CPUCL0: c_uint = 0x1c80;
pub const GS101_SUBBBLK_OFFSET_CPUCL1: c_uint = 0x1d00;
pub const GS101_SUBBBLK_OFFSET_CPUCL2: c_uint = 0x1d80;
pub const GS101_SUBBBLK_OFFSET_G3D: c_uint = 0x1e00;
pub const GS101_SUBBBLK_OFFSET_EMBEDDED_CPUCL0: c_uint = 0x1e80;
pub const GS101_SUBBBLK_OFFSET_EMBEDDED_G3D: c_uint = 0x2000;
pub const GS101_SUBBBLK_OFFSET_HSI0: c_uint = 0x2080;
pub const GS101_SUBBBLK_OFFSET_HSI1: c_uint = 0x2100;
pub const GS101_SUBBBLK_OFFSET_HSI2: c_uint = 0x2180;
pub const GS101_SUBBBLK_OFFSET_DPU: c_uint = 0x2200;
pub const GS101_SUBBBLK_OFFSET_DISP: c_uint = 0x2280;
pub const GS101_SUBBBLK_OFFSET_G2D: c_uint = 0x2300;
pub const GS101_SUBBBLK_OFFSET_MFC: c_uint = 0x2380;
pub const GS101_SUBBBLK_OFFSET_CSIS: c_uint = 0x2400;
pub const GS101_SUBBBLK_OFFSET_PDP: c_uint = 0x2480;
pub const GS101_SUBBBLK_OFFSET_DNS: c_uint = 0x2500;
pub const GS101_SUBBBLK_OFFSET_G3AA: c_uint = 0x2580;
pub const GS101_SUBBBLK_OFFSET_IPP: c_uint = 0x2600;
pub const GS101_SUBBBLK_OFFSET_ITP: c_uint = 0x2680;
pub const GS101_SUBBBLK_OFFSET_MCSC: c_uint = 0x2700;
pub const GS101_SUBBBLK_OFFSET_GDC: c_uint = 0x2780;
pub const GS101_SUBBBLK_OFFSET_TNR: c_uint = 0x2800;
pub const GS101_SUBBBLK_OFFSET_BO: c_uint = 0x2880;
pub const GS101_SUBBBLK_OFFSET_TPU: c_uint = 0x2900;
pub const GS101_SUBBBLK_OFFSET_MIF0: c_uint = 0x2980;
pub const GS101_SUBBBLK_OFFSET_MIF1: c_uint = 0x2a00;
pub const GS101_SUBBBLK_OFFSET_MIF2: c_uint = 0x2a80;
pub const GS101_SUBBBLK_OFFSET_MIF3: c_uint = 0x2b00;
pub const GS101_SUBBBLK_OFFSET_MISC: c_uint = 0x2b80;
pub const GS101_SUBBBLK_OFFSET_PERIC0: c_uint = 0x2c00;
pub const GS101_SUBBBLK_OFFSET_PERIC1: c_uint = 0x2c80;
pub const GS101_SUBBBLK_OFFSET_S2D: c_uint = 0x2d00;

pub const GS101_SUBBBLK_CPU_OFFSET_APM: c_uint = 0x3000;
pub const GS101_SUBBBLK_CPU_OFFSET_DBGCORE: c_uint = 0x3080;
pub const GS101_SUBBBLK_CPU_OFFSET_SSS: c_uint = 0x3100;

pub const GS101_MIF_CONFIGURATION: c_uint = 0x3800;
pub const GS101_MIF_STATUS: c_uint = 0x3804;
pub const GS101_MIF_STATES: c_uint = 0x3808;
pub const GS101_MIF_OPTION: c_uint = 0x380c;
pub const GS101_MIF_CTRL: c_uint = 0x3810;
pub const GS101_MIF_OUT: c_uint = 0x3820;
pub const GS101_MIF_IN: c_uint = 0x3824;
pub const GS101_MIF_INT_IN: c_uint = 0x3840;
pub const GS101_MIF_INT_EN: c_uint = 0x3844;
pub const GS101_MIF_INT_TYPE: c_uint = 0x3848;
pub const GS101_MIF_INT_DIR: c_uint = 0x384c;
pub const GS101_TOP_CONFIGURATION: c_uint = 0x3900;
pub const GS101_TOP_STATUS: c_uint = 0x3904;
pub const GS101_TOP_STATES: c_uint = 0x3908;
pub const GS101_TOP_OPTION: c_uint = 0x390c;
pub const GS101_TOP_OUT: c_uint = 0x3920;
pub const GS101_TOP_IN: c_uint = 0x3924;
pub const GS101_TOP_INT_IN: c_uint = 0x3940;
pub const GS101_TOP_INT_EN: c_uint = 0x3944;
pub const GS101_TOP_INT_TYPE: c_uint = 0x3948;
pub const GS101_TOP_INT_DIR: c_uint = 0x394c;
pub const GS101_WAKEUP_STAT: c_uint = 0x3950;
pub const GS101_WAKEUP2_STAT: c_uint = 0x3954;
pub const GS101_WAKEUP2_INT_IN: c_uint = 0x3960;
pub const GS101_WAKEUP2_INT_EN: c_uint = 0x3964;
pub const GS101_WAKEUP2_INT_TYPE: c_uint = 0x3968;
pub const GS101_WAKEUP2_INT_DIR: c_uint = 0x396c;
pub const GS101_SYSTEM_CONFIGURATION: c_uint = 0x3a00;
pub const GS101_SYSTEM_STATUS: c_uint = 0x3a04;
pub const GS101_SYSTEM_STATES: c_uint = 0x3a08;
pub const GS101_SYSTEM_OPTION: c_uint = 0x3a0c;
pub const GS101_SYSTEM_CTRL: c_uint = 0x3a10;
pub const GS101_SPARE_CTRL: c_uint = 0x3a14;
pub const GS101_USER_DEFINED_OUT: c_uint = 0x3a18;
pub const GS101_SYSTEM_OUT: c_uint = 0x3a20;
pub const GS101_SYSTEM_IN: c_uint = 0x3a24;
pub const GS101_SYSTEM_INT_IN: c_uint = 0x3a40;
pub const GS101_SYSTEM_INT_EN: c_uint = 0x3a44;
pub const GS101_SYSTEM_INT_TYPE: c_uint = 0x3a48;
pub const GS101_SYSTEM_INT_DIR: c_uint = 0x3a4c;
pub const GS101_EINT_INT_IN: c_uint = 0x3a50;
pub const GS101_EINT_INT_EN: c_uint = 0x3a54;
pub const GS101_EINT_INT_TYPE: c_uint = 0x3a58;
pub const GS101_EINT_INT_DIR: c_uint = 0x3a5c;
pub const GS101_EINT2_INT_IN: c_uint = 0x3a60;
pub const GS101_EINT2_INT_EN: c_uint = 0x3a64;
pub const GS101_EINT2_INT_TYPE: c_uint = 0x3a68;
pub const GS101_EINT2_INT_DIR: c_uint = 0x3a6c;
pub const GS101_EINT3_INT_IN: c_uint = 0x3a70;
pub const GS101_EINT3_INT_EN: c_uint = 0x3a74;
pub const GS101_EINT3_INT_TYPE: c_uint = 0x3a78;
pub const GS101_EINT3_INT_DIR: c_uint = 0x3a7c;
pub const GS101_EINT_WAKEUP_MASK: c_uint = 0x3a80;
pub const GS101_EINT_WAKEUP_MASK2: c_uint = 0x3a84;
pub const GS101_EINT_WAKEUP_MASK3: c_uint = 0x3a88;
pub const GS101_USER_DEFINED_INT_IN: c_uint = 0x3a90;
pub const GS101_USER_DEFINED_INT_EN: c_uint = 0x3a94;
pub const GS101_USER_DEFINED_INT_TYPE: c_uint = 0x3a98;
pub const GS101_USER_DEFINED_INT_DIR: c_uint = 0x3a9c;
pub const GS101_SCAN2DRAM_INT_IN: c_uint = 0x3aa0;
pub const GS101_SCAN2DRAM_INT_EN: c_uint = 0x3aa4;
pub const GS101_SCAN2DRAM_INT_TYPE: c_uint = 0x3aa8;
pub const GS101_SCAN2DRAM_INT_DIR: c_uint = 0x3aac;
pub const GS101_HCU_START: c_uint = 0x3ab0;
pub const GS101_CUSTOM_OUT: c_uint = 0x3ac0;
pub const GS101_CUSTOM_IN: c_uint = 0x3ac4;
pub const GS101_CUSTOM_INT_IN: c_uint = 0x3ad0;
pub const GS101_CUSTOM_INT_EN: c_uint = 0x3ad4;
pub const GS101_CUSTOM_INT_TYPE: c_uint = 0x3ad8;
pub const GS101_CUSTOM_INT_DIR: c_uint = 0x3adc;
pub const GS101_ACK_LAST_CPU: c_uint = 0x3afc;

pub const GS101_HCU_SP: c_uint = 0x3b14;
pub const GS101_HCU_PC: c_uint = 0x3b18;
pub const GS101_PMU_RAM_CTRL: c_uint = 0x3b20;
pub const GS101_APM_HCU_CTRL: c_uint = 0x3b24;
pub const GS101_APM_NMI_ENABLE: c_uint = 0x3b30;
pub const GS101_DBGCORE_NMI_ENABLE: c_uint = 0x3b34;
pub const GS101_HCU_NMI_ENABLE: c_uint = 0x3b38;
pub const GS101_PWR_HOLD_WDT_ENABLE: c_uint = 0x3b3c;
pub const GS101_NMI_SRC_IN: c_uint = 0x3b40;
pub const GS101_RST_STAT: c_uint = 0x3b44;
pub const GS101_RST_STAT_PMU: c_uint = 0x3b48;
pub const GS101_HPM_INT_IN: c_uint = 0x3b60;
pub const GS101_HPM_INT_EN: c_uint = 0x3b64;
pub const GS101_HPM_INT_TYPE: c_uint = 0x3b68;
pub const GS101_HPM_INT_DIR: c_uint = 0x3b6c;
pub const GS101_S2D_AUTH: c_uint = 0x3b70;
pub const GS101_BOOT_STAT: c_uint = 0x3b74;
pub const GS101_PMLINK_OUT: c_uint = 0x3c00;
pub const GS101_PMLINK_AOC_OUT: c_uint = 0x3c04;
pub const GS101_PMLINK_AOC_CTRL: c_uint = 0x3c08;
pub const GS101_TCXO_BUF_CTRL: c_uint = 0x3c10;
pub const GS101_ADD_CTRL: c_uint = 0x3c14;
pub const GS101_HCU_TIMEOUT_RESET: c_uint = 0x3c20;
pub const GS101_HCU_TIMEOUT_SCAN2DRAM: c_uint = 0x3c24;

pub const GS101_PPC_CORE: c_uint = 0x3ca0;
pub const GS101_PPC_EH: c_uint = 0x3ca4;
pub const GS101_PPC_CPUCL1_0: c_uint = 0x3ca8;
pub const GS101_PPC_CPUCL1_1: c_uint = 0x3cac;
pub const GS101_EXT_REGULATOR_MIF_DURATION: c_uint = 0x3cb0;
pub const GS101_EXT_REGULATOR_TOP_DURATION: c_uint = 0x3cb4;
pub const GS101_EXT_REGULATOR_CPUCL2_DURATION: c_uint = 0x3cb8;
pub const GS101_EXT_REGULATOR_CPUCL1_DURATION: c_uint = 0x3cbc;
pub const GS101_EXT_REGULATOR_G3D_DURATION: c_uint = 0x3cc0;
pub const GS101_EXT_REGULATOR_TPU_DURATION: c_uint = 0x3cc4;
pub const GS101_TCXO_DURATION: c_uint = 0x3cc8;
pub const GS101_BURNIN_CTRL: c_uint = 0x3cd0;
pub const GS101_JTAG_DBG_DET: c_uint = 0x3cd4;
pub const GS101_MMC_CONWKUP_CTRL: c_uint = 0x3cd8;
pub const GS101_USBDPPHY0_USBDP_WAKEUP: c_uint = 0x3cdc;
pub const GS101_TMU_TOP_TRIP: c_uint = 0x3ce0;
pub const GS101_TMU_SUB_TRIP: c_uint = 0x3ce4;
pub const GS101_MEMORY_CEN: c_uint = 0x3d00;
pub const GS101_MEMORY_PGEN: c_uint = 0x3d04;
pub const GS101_MEMORY_RET: c_uint = 0x3d08;
pub const GS101_MEMORY_PGEN_FEEDBACK: c_uint = 0x3d0c;
pub const GS101_MEMORY_SMX: c_uint = 0x3d10;
pub const GS101_MEMORY_SMX_FEEDBACK: c_uint = 0x3d14;
pub const GS101_SLC_PCH_CHANNEL: c_uint = 0x3d20;
pub const GS101_SLC_PCH_CB: c_uint = 0x3d24;
pub const GS101_FORCE_NOMC: c_uint = 0x3d3c;
pub const GS101_FORCE_BOOST: c_uint = 0x3d4c;
pub const GS101_PMLINK_SLC_REQ: c_uint = 0x3d50;
pub const GS101_PMLINK_SLC_ACK: c_uint = 0x3d54;
pub const GS101_PMLINK_SLC_BUSY: c_uint = 0x3d58;
pub const GS101_BOOTSYNC_OUT: c_uint = 0x3d80;
pub const GS101_BOOTSYNC_IN: c_uint = 0x3d84;
pub const GS101_SCAN_READY_OUT: c_uint = 0x3d88;
pub const GS101_SCAN_READY_IN: c_uint = 0x3d8c;
pub const GS101_GSA_RESTORE: c_uint = 0x3d90;
pub const GS101_ALIVE_OTP_LATCH: c_uint = 0x3d94;
pub const GS101_DEBUG_OVERRIDE: c_uint = 0x3d98;
pub const GS101_WDT_OPTION: c_uint = 0x3d9c;
pub const GS101_AOC_WDT_CFG: c_uint = 0x3da0;
pub const GS101_CTRL_SECJTAG_ALIVE: c_uint = 0x3da4;
pub const GS101_CTRL_DIV_PLL_ALV_DIVLOW: c_uint = 0x3e00;
pub const GS101_CTRL_MUX_CLK_APM_REFSRC_AUTORESTORE: c_uint = 0x3e04;
pub const GS101_CTRL_MUX_CLK_APM_REFSRC: c_uint = 0x3e08;
pub const GS101_CTRL_MUX_CLK_APM_REF: c_uint = 0x3e0c;
pub const GS101_CTRL_MUX_PLL_ALV_DIV4: c_uint = 0x3e10;
pub const GS101_CTRL_PLL_ALV_DIV4: c_uint = 0x3e14;
pub const GS101_CTRL_OSCCLK_APMGSA: c_uint = 0x3e18;
pub const GS101_CTRL_BLK_AOC_CLKS: c_uint = 0x3e1c;
pub const GS101_CTRL_PLL_ALV_LOCK: c_uint = 0x3e20;
pub const GS101_CTRL_CLKDIV__CLKRTC: c_uint = 0x3e24;
pub const GS101_CTRL_SOC32K: c_uint = 0x3e30;
pub const GS101_CTRL_STM_PMU: c_uint = 0x3e34;
pub const GS101_CTRL_PMU_DEBUG: c_uint = 0x3e38;
pub const GS101_CTRL_DEBUG_UART: c_uint = 0x3e3c;
pub const GS101_CTRL_TCK: c_uint = 0x3e40;
pub const GS101_CTRL_SBU_SW_EN: c_uint = 0x3e44;
pub const GS101_PAD_CTRL_CLKOUT0: c_uint = 0x3e80;
pub const GS101_PAD_CTRL_CLKOUT1: c_uint = 0x3e84;
pub const GS101_PAD_CTRL_APM_24MOUT_0: c_uint = 0x3e88;
pub const GS101_PAD_CTRL_APM_24MOUT_1: c_uint = 0x3e8c;
pub const GS101_PAD_CTRL_IO_FORCE_RETENTION: c_uint = 0x3e90;
pub const GS101_PAD_CTRL_APACTIVE_n: c_uint = 0x3e94;
pub const GS101_PAD_CTRL_TCXO_ON: c_uint = 0x3e98;
pub const GS101_PAD_CTRL_PWR_HOLD: c_uint = 0x3e9c;
pub const GS101_PAD_CTRL_RESETO_n: c_uint = 0x3ea0;
pub const GS101_PAD_CTRL_WRESETO_n: c_uint = 0x3ea4;
pub const GS101_PHY_CTRL_USB20: c_uint = 0x3eb0;
pub const GS101_PHY_CTRL_USBDP: c_uint = 0x3eb4;
pub const GS101_PHY_CTRL_MIPI_DCPHY_M4M4: c_uint = 0x3eb8;
pub const GS101_PHY_CTRL_MIPI_DCPHY_S4S4S4S4: c_uint = 0x3ebc;
pub const GS101_PHY_CTRL_PCIE_GEN4_0: c_uint = 0x3ec0;
pub const GS101_PHY_CTRL_PCIE_GEN4_1: c_uint = 0x3ec4;
pub const GS101_PHY_CTRL_UFS: c_uint = 0x3ec8;
// PMU INTR GEN

// exynosautov920

