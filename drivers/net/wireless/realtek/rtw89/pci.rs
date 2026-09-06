//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/pci.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2020  Realtek Corporation
//

pub const MDIO_PG0_G1: c_int = 0;
pub const MDIO_PG1_G1: c_int = 1;
pub const MDIO_PG0_G2: c_int = 2;
pub const MDIO_PG1_G2: c_int = 3;
pub const RAC_CTRL_PPR: c_uint = 0x00;
pub const RAC_ANA03: c_uint = 0x03;

pub const RAC_ANA09: c_uint = 0x09;

pub const RAC_ANA0A: c_uint = 0x0A;

pub const RAC_ANA0B: c_uint = 0x0B;

pub const RAC_ANA0C: c_uint = 0x0C;

pub const RAC_ANA0D: c_uint = 0x0D;

pub const RAC_ANA10: c_uint = 0x10;

pub const ADDR_SEL_VAL: c_uint = 0x3C;
pub const ADDR_SEL_PINOUT_DIS_VAL: c_uint = 0x3C4;

pub const RAC_REG_REV2: c_uint = 0x1B;

pub const PCIE_DPHY_DLY_25US: c_uint = 0x1;
pub const RAC_ANA14: c_uint = 0x14;

pub const RAC_ANA19: c_uint = 0x19;

pub const RAC_REG_FLD_0: c_uint = 0x1D;

pub const PCIE_AUTOK_4: c_uint = 0x3;
pub const RAC_ANA1E: c_uint = 0x1E;
pub const RAC_ANA1E_G1_VAL: c_uint = 0x66EA;
pub const RAC_ANA1E_G2_VAL: c_uint = 0x6EEA;
pub const RAC_ANA1F: c_uint = 0x1F;

pub const RAC_ANA24: c_uint = 0x24;

pub const RAC_ANA26: c_uint = 0x26;

pub const RAC_ANA2E: c_uint = 0x2E;
pub const RAC_ANA2E_VAL: c_uint = 0xFFFE;
pub const RAC_CTRL_PPR_V1: c_uint = 0x30;

pub const RAC_SET_PPR_V1: c_uint = 0x31;
pub const RAC_ANA40: c_uint = 0x40;

pub const RAC_ANA41: c_uint = 0x41;

pub const R_AX_DBI_FLAG: c_uint = 0x1090;

pub const R_AX_DBI_WDATA: c_uint = 0x1094;
pub const R_AX_DBI_RDATA: c_uint = 0x1098;
pub const R_AX_MDIO_WDATA: c_uint = 0x10A4;
pub const R_AX_MDIO_RDATA: c_uint = 0x10A6;
pub const R_AX_PCIE_PS_CTRL_V1: c_uint = 0x3008;

pub const R_AX_PCIE_MIX_CFG_V1: c_uint = 0x300C;

pub const R_AX_L1_CLK_CTRL: c_uint = 0x3010;

pub const R_AX_PCIE_BG_CLR: c_uint = 0x303C;

pub const R_AX_PCIE_LAT_CTRL: c_uint = 0x3044;

pub const R_AX_PCIE_IO_RCY_M1: c_uint = 0x3100;

pub const R_AX_PCIE_WDT_TIMER_M1: c_uint = 0x3104;

pub const R_AX_PCIE_IO_RCY_M2: c_uint = 0x310C;

pub const R_AX_PCIE_WDT_TIMER_M2: c_uint = 0x3110;

pub const R_AX_PCIE_IO_RCY_E0: c_uint = 0x3118;

pub const R_AX_PCIE_WDT_TIMER_E0: c_uint = 0x311C;

pub const R_AX_PCIE_IO_RCY_S1: c_uint = 0x3124;

pub const R_AX_PCIE_WDT_TIMER_S1: c_uint = 0x3128;

pub const R_RAC_DIRECT_OFFSET_G1: c_uint = 0x3800;

pub const R_RAC_DIRECT_OFFSET_G2: c_uint = 0x3880;

pub const RAC_MULT: c_int = 2;
pub const R_RAC_DIRECT_OFFSET_BE_LANE0_G1: c_uint = 0x3800;
pub const R_RAC_DIRECT_OFFSET_BE_LANE1_G1: c_uint = 0x3880;
pub const R_RAC_DIRECT_OFFSET_BE_LANE0_G2: c_uint = 0x3900;
pub const R_RAC_DIRECT_OFFSET_BE_LANE1_G2: c_uint = 0x3980;
pub const RAC_DIRECT_OFFESET_L0_G1: c_uint = 0x3800;
pub const RAC_DIRECT_OFFESET_L1_G1: c_uint = 0x3900;
pub const RAC_DIRECT_OFFESET_L0_G2: c_uint = 0x3A00;
pub const RAC_DIRECT_OFFESET_L1_G2: c_uint = 0x3B00;
pub const RTW89_PCI_WR_RETRY_CNT: c_int = 20;
// Interrupts
pub const R_AX_HIMR0: c_uint = 0x01A0;

pub const R_AX_HISR0: c_uint = 0x01A4;
pub const R_AX_HIMR1: c_uint = 0x01A8;

pub const R_AX_HISR1: c_uint = 0x01AC;

pub const R_AX_MDIO_CFG: c_uint = 0x10A0;

pub const R_AX_PCIE_HIMR00: c_uint = 0x10B0;
pub const R_AX_HAXI_HIMR00: c_uint = 0x10B0;

pub const R_AX_PCIE_HISR00: c_uint = 0x10B4;
pub const R_AX_HAXI_HISR00: c_uint = 0x10B4;

pub const R_AX_HAXI_IDCT_MSK: c_uint = 0x10B8;

pub const R_AX_HAXI_IDCT: c_uint = 0x10BC;

pub const R_AX_HAXI_HIMR10: c_uint = 0x11E0;

pub const R_AX_PCIE_HIMR10: c_uint = 0x13B0;

pub const R_AX_PCIE_HISR10: c_uint = 0x13B4;

pub const R_AX_PCIE_HIMR00_V1: c_uint = 0x30B0;

pub const R_AX_PCIE_HISR00_V1: c_uint = 0x30B4;

pub const R_BE_PCIE_FRZ_CLK: c_uint = 0x3004;

pub const R_BE_PCIE_PS_CTRL: c_uint = 0x3008;

pub const R_BE_PCIE_MIX_CFG: c_uint = 0x300C;

pub const R_BE_L1_CLK_CTRL: c_uint = 0x3010;

pub const R_BE_PCIE_LAT_CTRL: c_uint = 0x3044;

pub const R_BE_PCIE_HIMR0: c_uint = 0x30B0;

pub const R_BE_PCIE_HISR: c_uint = 0x30B4;

pub const R_BE_PCIE_DMA_IMR_0_V1: c_uint = 0x30B8;

pub const R_BE_PCIE_DMA_ISR: c_uint = 0x30BC;

pub const R_BE_HAXI_HIMR00: c_uint = 0xB0B0;

pub const R_BE_HAXI_HISR00: c_uint = 0xB0B4;

// TX/RX
pub const R_AX_DRV_FW_HSK_0: c_uint = 0x01B0;
pub const R_AX_DRV_FW_HSK_1: c_uint = 0x01B4;
pub const R_AX_DRV_FW_HSK_2: c_uint = 0x01B8;
pub const R_AX_DRV_FW_HSK_3: c_uint = 0x01BC;
pub const R_AX_DRV_FW_HSK_4: c_uint = 0x01C0;
pub const R_AX_DRV_FW_HSK_5: c_uint = 0x01C4;
pub const R_AX_DRV_FW_HSK_6: c_uint = 0x01C8;
pub const R_AX_DRV_FW_HSK_7: c_uint = 0x01CC;
pub const R_AX_RXQ_RXBD_IDX: c_uint = 0x1050;
pub const R_AX_RPQ_RXBD_IDX: c_uint = 0x1054;
pub const R_AX_ACH0_TXBD_IDX: c_uint = 0x1058;
pub const R_AX_ACH1_TXBD_IDX: c_uint = 0x105C;
pub const R_AX_ACH2_TXBD_IDX: c_uint = 0x1060;
pub const R_AX_ACH3_TXBD_IDX: c_uint = 0x1064;
pub const R_AX_ACH4_TXBD_IDX: c_uint = 0x1068;
pub const R_AX_ACH5_TXBD_IDX: c_uint = 0x106C;
pub const R_AX_ACH6_TXBD_IDX: c_uint = 0x1070;
pub const R_AX_ACH7_TXBD_IDX: c_uint = 0x1074;
pub const R_AX_CH8_TXBD_IDX: c_uint = 0x1078 /* Management Queue band 0 */;
pub const R_AX_CH9_TXBD_IDX: c_uint = 0x107C /* HI Queue band 0 */;
pub const R_AX_CH10_TXBD_IDX: c_uint = 0x137C /* Management Queue band 1 */;
pub const R_AX_CH11_TXBD_IDX: c_uint = 0x1380 /* HI Queue band 1 */;
pub const R_AX_CH12_TXBD_IDX: c_uint = 0x1080 /* FWCMD Queue */;
pub const R_AX_CH10_TXBD_IDX_V1: c_uint = 0x11D0;
pub const R_AX_CH11_TXBD_IDX_V1: c_uint = 0x11D4;
pub const R_AX_RXQ_RXBD_IDX_V1: c_uint = 0x1218;
pub const R_AX_RPQ_RXBD_IDX_V1: c_uint = 0x121C;

pub const R_AX_ACH0_TXBD_DESA_L: c_uint = 0x1110;
pub const R_AX_ACH0_TXBD_DESA_H: c_uint = 0x1114;
pub const R_AX_ACH1_TXBD_DESA_L: c_uint = 0x1118;
pub const R_AX_ACH1_TXBD_DESA_H: c_uint = 0x111C;
pub const R_AX_ACH2_TXBD_DESA_L: c_uint = 0x1120;
pub const R_AX_ACH2_TXBD_DESA_H: c_uint = 0x1124;
pub const R_AX_ACH3_TXBD_DESA_L: c_uint = 0x1128;
pub const R_AX_ACH3_TXBD_DESA_H: c_uint = 0x112C;
pub const R_AX_ACH4_TXBD_DESA_L: c_uint = 0x1130;
pub const R_AX_ACH4_TXBD_DESA_H: c_uint = 0x1134;
pub const R_AX_ACH5_TXBD_DESA_L: c_uint = 0x1138;
pub const R_AX_ACH5_TXBD_DESA_H: c_uint = 0x113C;
pub const R_AX_ACH6_TXBD_DESA_L: c_uint = 0x1140;
pub const R_AX_ACH6_TXBD_DESA_H: c_uint = 0x1144;
pub const R_AX_ACH7_TXBD_DESA_L: c_uint = 0x1148;
pub const R_AX_ACH7_TXBD_DESA_H: c_uint = 0x114C;
pub const R_AX_CH8_TXBD_DESA_L: c_uint = 0x1150;
pub const R_AX_CH8_TXBD_DESA_H: c_uint = 0x1154;
pub const R_AX_CH9_TXBD_DESA_L: c_uint = 0x1158;
pub const R_AX_CH9_TXBD_DESA_H: c_uint = 0x115C;
pub const R_AX_CH10_TXBD_DESA_L: c_uint = 0x1358;
pub const R_AX_CH10_TXBD_DESA_H: c_uint = 0x135C;
pub const R_AX_CH11_TXBD_DESA_L: c_uint = 0x1360;
pub const R_AX_CH11_TXBD_DESA_H: c_uint = 0x1364;
pub const R_AX_CH12_TXBD_DESA_L: c_uint = 0x1160;
pub const R_AX_CH12_TXBD_DESA_H: c_uint = 0x1164;
pub const R_AX_RXQ_RXBD_DESA_L: c_uint = 0x1100;
pub const R_AX_RXQ_RXBD_DESA_H: c_uint = 0x1104;
pub const R_AX_RPQ_RXBD_DESA_L: c_uint = 0x1108;
pub const R_AX_RPQ_RXBD_DESA_H: c_uint = 0x110C;
pub const R_AX_RXQ_RXBD_DESA_L_V1: c_uint = 0x1220;
pub const R_AX_RXQ_RXBD_DESA_H_V1: c_uint = 0x1224;
pub const R_AX_RPQ_RXBD_DESA_L_V1: c_uint = 0x1228;
pub const R_AX_RPQ_RXBD_DESA_H_V1: c_uint = 0x122C;
pub const R_AX_ACH0_TXBD_DESA_L_V1: c_uint = 0x1230;
pub const R_AX_ACH0_TXBD_DESA_H_V1: c_uint = 0x1234;
pub const R_AX_ACH1_TXBD_DESA_L_V1: c_uint = 0x1238;
pub const R_AX_ACH1_TXBD_DESA_H_V1: c_uint = 0x123C;
pub const R_AX_ACH2_TXBD_DESA_L_V1: c_uint = 0x1240;
pub const R_AX_ACH2_TXBD_DESA_H_V1: c_uint = 0x1244;
pub const R_AX_ACH3_TXBD_DESA_L_V1: c_uint = 0x1248;
pub const R_AX_ACH3_TXBD_DESA_H_V1: c_uint = 0x124C;
pub const R_AX_ACH4_TXBD_DESA_L_V1: c_uint = 0x1250;
pub const R_AX_ACH4_TXBD_DESA_H_V1: c_uint = 0x1254;
pub const R_AX_ACH5_TXBD_DESA_L_V1: c_uint = 0x1258;
pub const R_AX_ACH5_TXBD_DESA_H_V1: c_uint = 0x125C;
pub const R_AX_ACH6_TXBD_DESA_L_V1: c_uint = 0x1260;
pub const R_AX_ACH6_TXBD_DESA_H_V1: c_uint = 0x1264;
pub const R_AX_ACH7_TXBD_DESA_L_V1: c_uint = 0x1268;
pub const R_AX_ACH7_TXBD_DESA_H_V1: c_uint = 0x126C;
pub const R_AX_CH8_TXBD_DESA_L_V1: c_uint = 0x1270;
pub const R_AX_CH8_TXBD_DESA_H_V1: c_uint = 0x1274;
pub const R_AX_CH9_TXBD_DESA_L_V1: c_uint = 0x1278;
pub const R_AX_CH9_TXBD_DESA_H_V1: c_uint = 0x127C;
pub const R_AX_CH12_TXBD_DESA_L_V1: c_uint = 0x1280;
pub const R_AX_CH12_TXBD_DESA_H_V1: c_uint = 0x1284;
pub const R_AX_CH10_TXBD_DESA_L_V1: c_uint = 0x1458;
pub const R_AX_CH10_TXBD_DESA_H_V1: c_uint = 0x145C;
pub const R_AX_CH11_TXBD_DESA_L_V1: c_uint = 0x1460;
pub const R_AX_CH11_TXBD_DESA_H_V1: c_uint = 0x1464;

pub const R_AX_RXQ_RXBD_NUM: c_uint = 0x1020;
pub const R_AX_RPQ_RXBD_NUM: c_uint = 0x1022;
pub const R_AX_ACH0_TXBD_NUM: c_uint = 0x1024;
pub const R_AX_ACH1_TXBD_NUM: c_uint = 0x1026;
pub const R_AX_ACH2_TXBD_NUM: c_uint = 0x1028;
pub const R_AX_ACH3_TXBD_NUM: c_uint = 0x102A;
pub const R_AX_ACH4_TXBD_NUM: c_uint = 0x102C;
pub const R_AX_ACH5_TXBD_NUM: c_uint = 0x102E;
pub const R_AX_ACH6_TXBD_NUM: c_uint = 0x1030;
pub const R_AX_ACH7_TXBD_NUM: c_uint = 0x1032;
pub const R_AX_CH8_TXBD_NUM: c_uint = 0x1034;
pub const R_AX_CH9_TXBD_NUM: c_uint = 0x1036;
pub const R_AX_CH10_TXBD_NUM: c_uint = 0x1338;
pub const R_AX_CH11_TXBD_NUM: c_uint = 0x133A;
pub const R_AX_CH12_TXBD_NUM: c_uint = 0x1038;
pub const R_AX_RXQ_RXBD_NUM_V1: c_uint = 0x1210;
pub const R_AX_RPQ_RXBD_NUM_V1: c_uint = 0x1212;
pub const R_AX_CH10_TXBD_NUM_V1: c_uint = 0x1438;
pub const R_AX_CH11_TXBD_NUM_V1: c_uint = 0x143A;
pub const R_AX_ACH0_BDRAM_CTRL: c_uint = 0x1200;
pub const R_AX_ACH1_BDRAM_CTRL: c_uint = 0x1204;
pub const R_AX_ACH2_BDRAM_CTRL: c_uint = 0x1208;
pub const R_AX_ACH3_BDRAM_CTRL: c_uint = 0x120C;
pub const R_AX_ACH4_BDRAM_CTRL: c_uint = 0x1210;
pub const R_AX_ACH5_BDRAM_CTRL: c_uint = 0x1214;
pub const R_AX_ACH6_BDRAM_CTRL: c_uint = 0x1218;
pub const R_AX_ACH7_BDRAM_CTRL: c_uint = 0x121C;
pub const R_AX_CH8_BDRAM_CTRL: c_uint = 0x1220;
pub const R_AX_CH9_BDRAM_CTRL: c_uint = 0x1224;
pub const R_AX_CH10_BDRAM_CTRL: c_uint = 0x1320;
pub const R_AX_CH11_BDRAM_CTRL: c_uint = 0x1324;
pub const R_AX_CH12_BDRAM_CTRL: c_uint = 0x1228;
pub const R_AX_ACH0_BDRAM_CTRL_V1: c_uint = 0x1300;
pub const R_AX_ACH1_BDRAM_CTRL_V1: c_uint = 0x1304;
pub const R_AX_ACH2_BDRAM_CTRL_V1: c_uint = 0x1308;
pub const R_AX_ACH3_BDRAM_CTRL_V1: c_uint = 0x130C;
pub const R_AX_ACH4_BDRAM_CTRL_V1: c_uint = 0x1310;
pub const R_AX_ACH5_BDRAM_CTRL_V1: c_uint = 0x1314;
pub const R_AX_ACH6_BDRAM_CTRL_V1: c_uint = 0x1318;
pub const R_AX_ACH7_BDRAM_CTRL_V1: c_uint = 0x131C;
pub const R_AX_CH8_BDRAM_CTRL_V1: c_uint = 0x1320;
pub const R_AX_CH9_BDRAM_CTRL_V1: c_uint = 0x1324;
pub const R_AX_CH12_BDRAM_CTRL_V1: c_uint = 0x1328;
pub const R_AX_CH10_BDRAM_CTRL_V1: c_uint = 0x1420;
pub const R_AX_CH11_BDRAM_CTRL_V1: c_uint = 0x1424;

pub const R_AX_PCIE_INIT_CFG1: c_uint = 0x1000;

pub const R_AX_TXDMA_ADDR_H: c_uint = 0x10F0;
pub const R_AX_RXDMA_ADDR_H: c_uint = 0x10F4;
pub const R_AX_PCIE_DMA_STOP1: c_uint = 0x1010;

pub const R_AX_PCIE_DMA_STOP2: c_uint = 0x1310;

pub const R_AX_TXBD_RWPTR_CLR1: c_uint = 0x1014;

pub const R_AX_RXBD_RWPTR_CLR: c_uint = 0x1018;

pub const R_AX_TXBD_RWPTR_CLR2: c_uint = 0x1314;

pub const R_AX_PCIE_DMA_BUSY1: c_uint = 0x101C;

pub const R_AX_PCIE_DMA_BUSY2: c_uint = 0x131C;

pub const R_AX_WP_ADDR_H_SEL0_3: c_uint = 0x1334;
pub const R_AX_WP_ADDR_H_SEL4_7: c_uint = 0x1338;
pub const R_AX_WP_ADDR_H_SEL8_11: c_uint = 0x133C;
pub const R_AX_WP_ADDR_H_SEL12_15: c_uint = 0x1340;
pub const R_BE_CH0_TXBD_NUM_V1: c_uint = 0xB030;
pub const R_BE_CH1_TXBD_NUM_V1: c_uint = 0xB032;
pub const R_BE_CH2_TXBD_NUM_V1: c_uint = 0xB034;
pub const R_BE_CH3_TXBD_NUM_V1: c_uint = 0xB036;
pub const R_BE_CH4_TXBD_NUM_V1: c_uint = 0xB038;
pub const R_BE_CH5_TXBD_NUM_V1: c_uint = 0xB03A;
pub const R_BE_CH6_TXBD_NUM_V1: c_uint = 0xB03C;
pub const R_BE_CH7_TXBD_NUM_V1: c_uint = 0xB03E;
pub const R_BE_CH8_TXBD_NUM_V1: c_uint = 0xB040;
pub const R_BE_CH9_TXBD_NUM_V1: c_uint = 0xB042;
pub const R_BE_CH10_TXBD_NUM_V1: c_uint = 0xB044;
pub const R_BE_CH11_TXBD_NUM_V1: c_uint = 0xB046;
pub const R_BE_CH12_TXBD_NUM_V1: c_uint = 0xB048;
pub const R_BE_CH13_TXBD_NUM_V1: c_uint = 0xB04C;
pub const R_BE_CH14_TXBD_NUM_V1: c_uint = 0xB04E;
pub const R_BE_CH0_TXBD_CFG: c_uint = 0xB030;
pub const R_BE_CH2_TXBD_CFG: c_uint = 0xB034;
pub const R_BE_CH4_TXBD_CFG: c_uint = 0xB038;
pub const R_BE_CH6_TXBD_CFG: c_uint = 0xB03C;
pub const R_BE_CH8_TXBD_CFG: c_uint = 0xB040;
pub const R_BE_CH10_TXBD_CFG: c_uint = 0xB044;
pub const R_BE_CH12_TXBD_CFG: c_uint = 0xB048;

pub const R_BE_RXQ0_RXBD_NUM_V1: c_uint = 0xB050;
pub const R_BE_RPQ0_RXBD_NUM_V1: c_uint = 0xB052;
pub const R_BE_RX_CH0_RXBD_CONFIG: c_uint = 0xB050;
pub const R_BE_RX_CH1_RXBD_CONFIG: c_uint = 0xB052;

pub const R_BE_CH0_TXBD_IDX_V1: c_uint = 0xB100;
pub const R_BE_CH1_TXBD_IDX_V1: c_uint = 0xB104;
pub const R_BE_CH2_TXBD_IDX_V1: c_uint = 0xB108;
pub const R_BE_CH3_TXBD_IDX_V1: c_uint = 0xB10C;
pub const R_BE_CH4_TXBD_IDX_V1: c_uint = 0xB110;
pub const R_BE_CH5_TXBD_IDX_V1: c_uint = 0xB114;
pub const R_BE_CH6_TXBD_IDX_V1: c_uint = 0xB118;
pub const R_BE_CH7_TXBD_IDX_V1: c_uint = 0xB11C;
pub const R_BE_CH8_TXBD_IDX_V1: c_uint = 0xB120;
pub const R_BE_CH9_TXBD_IDX_V1: c_uint = 0xB124;
pub const R_BE_CH10_TXBD_IDX_V1: c_uint = 0xB128;
pub const R_BE_CH11_TXBD_IDX_V1: c_uint = 0xB12C;
pub const R_BE_CH12_TXBD_IDX_V1: c_uint = 0xB130;
pub const R_BE_CH13_TXBD_IDX_V1: c_uint = 0xB134;
pub const R_BE_CH14_TXBD_IDX_V1: c_uint = 0xB138;
pub const R_BE_RXQ0_RXBD_IDX_V1: c_uint = 0xB160;
pub const R_BE_RPQ0_RXBD_IDX_V1: c_uint = 0xB164;
pub const R_BE_CH0_TXBD_DESA_L_V1: c_uint = 0xB200;
pub const R_BE_CH0_TXBD_DESA_H_V1: c_uint = 0xB204;
pub const R_BE_CH1_TXBD_DESA_L_V1: c_uint = 0xB208;
pub const R_BE_CH1_TXBD_DESA_H_V1: c_uint = 0xB20C;
pub const R_BE_CH2_TXBD_DESA_L_V1: c_uint = 0xB210;
pub const R_BE_CH2_TXBD_DESA_H_V1: c_uint = 0xB214;
pub const R_BE_CH3_TXBD_DESA_L_V1: c_uint = 0xB218;
pub const R_BE_CH3_TXBD_DESA_H_V1: c_uint = 0xB21C;
pub const R_BE_CH4_TXBD_DESA_L_V1: c_uint = 0xB220;
pub const R_BE_CH4_TXBD_DESA_H_V1: c_uint = 0xB224;
pub const R_BE_CH5_TXBD_DESA_L_V1: c_uint = 0xB228;
pub const R_BE_CH5_TXBD_DESA_H_V1: c_uint = 0xB22C;
pub const R_BE_CH6_TXBD_DESA_L_V1: c_uint = 0xB230;
pub const R_BE_CH6_TXBD_DESA_H_V1: c_uint = 0xB234;
pub const R_BE_CH7_TXBD_DESA_L_V1: c_uint = 0xB238;
pub const R_BE_CH7_TXBD_DESA_H_V1: c_uint = 0xB23C;
pub const R_BE_CH8_TXBD_DESA_L_V1: c_uint = 0xB240;
pub const R_BE_CH8_TXBD_DESA_H_V1: c_uint = 0xB244;
pub const R_BE_CH9_TXBD_DESA_L_V1: c_uint = 0xB248;
pub const R_BE_CH9_TXBD_DESA_H_V1: c_uint = 0xB24C;
pub const R_BE_CH10_TXBD_DESA_L_V1: c_uint = 0xB250;
pub const R_BE_CH10_TXBD_DESA_H_V1: c_uint = 0xB254;
pub const R_BE_CH11_TXBD_DESA_L_V1: c_uint = 0xB258;
pub const R_BE_CH11_TXBD_DESA_H_V1: c_uint = 0xB25C;
pub const R_BE_CH12_TXBD_DESA_L_V1: c_uint = 0xB260;
pub const R_BE_CH12_TXBD_DESA_H_V1: c_uint = 0xB264;
pub const R_BE_CH13_TXBD_DESA_L_V1: c_uint = 0xB268;
pub const R_BE_CH13_TXBD_DESA_H_V1: c_uint = 0xB26C;
pub const R_BE_CH14_TXBD_DESA_L_V1: c_uint = 0xB270;
pub const R_BE_CH14_TXBD_DESA_H_V1: c_uint = 0xB274;
pub const R_BE_ACQ_TXBD_DESA_L: c_uint = 0xB200;

pub const R_BE_ACQ_TXBD_DESA_H: c_uint = 0xB204;

pub const R_BE_NACQ_TXBD_DESA_L: c_uint = 0xB240;

pub const R_BE_NACQ_TXBD_DESA_H: c_uint = 0xB244;

pub const R_BE_RXQ0_RXBD_DESA_L_V1: c_uint = 0xB300;
pub const R_BE_RXQ0_RXBD_DESA_H_V1: c_uint = 0xB304;
pub const R_BE_RPQ0_RXBD_DESA_L_V1: c_uint = 0xB308;
pub const R_BE_RPQ0_RXBD_DESA_H_V1: c_uint = 0xB30C;
pub const R_BE_HOST0_RXBD_DESA_L: c_uint = 0xB300;

pub const R_BE_HOST0_RXBD_DESA_H: c_uint = 0xB304;

pub const R_BE_WP_ADDR_H_SEL0_3_V1: c_uint = 0xB420;
pub const R_BE_WP_ADDR_H_SEL4_7_V1: c_uint = 0xB424;
pub const R_BE_WP_ADDR_H_SEL8_11_V1: c_uint = 0xB428;
pub const R_BE_WP_ADDR_H_SEL12_15_V1: c_uint = 0xB42C;
// Configure
pub const R_AX_PCIE_INIT_CFG2: c_uint = 0x1004;

pub const R_AX_PCIE_PS_CTRL: c_uint = 0x1008;

pub const R_AX_INT_MIT_RX: c_uint = 0x10D4;

pub const AX_RXTIMER_UNIT_64US: c_int = 0;
pub const AX_RXTIMER_UNIT_128US: c_int = 1;
pub const AX_RXTIMER_UNIT_256US: c_int = 2;
pub const AX_RXTIMER_UNIT_512US: c_int = 3;

pub const R_AX_DBG_ERR_FLAG_V1: c_uint = 0x1104;
pub const R_AX_INT_MIT_RX_V1: c_uint = 0x1184;

pub const R_AX_DBG_ERR_FLAG: c_uint = 0x11C4;

pub const R_AX_TXBD_RWPTR_CLR2_V1: c_uint = 0x11C4;

pub const R_AX_LBC_WATCHDOG: c_uint = 0x11D8;

pub const R_AX_RXBD_RWPTR_CLR_V1: c_uint = 0x1200;

pub const R_AX_HAXI_EXP_CTRL: c_uint = 0x1204;

pub const R_AX_PCIE_EXP_CTRL: c_uint = 0x13F0;

pub const R_AX_PCIE_RX_PREF_ADV: c_uint = 0x13F4;

pub const R_AX_PCIE_HRPWM_V1: c_uint = 0x30C0;
pub const R_AX_PCIE_CRPWM: c_uint = 0x30C4;
pub const R_AX_LBC_WATCHDOG_V1: c_uint = 0x30D8;
pub const R_BE_PCIE_HRPWM: c_uint = 0x30C0;
pub const R_BE_PCIE_CRPWM: c_uint = 0x30C4;
pub const R_BE_PCIE_HCI2FW_ISR: c_uint = 0x30CC;
pub const R_BE_L1_2_CTRL_HCILDO: c_uint = 0x3110;

pub const R_BE_PL1_DBG_INFO: c_uint = 0x3120;

pub const R_BE_PCIE_MIT0_TMR: c_uint = 0x3330;

pub const BE_MIT0_TMR_UNIT_1MS: c_int = 0;
pub const BE_MIT0_TMR_UNIT_2MS: c_int = 1;
pub const BE_MIT0_TMR_UNIT_4MS: c_int = 2;
pub const BE_MIT0_TMR_UNIT_8MS: c_int = 3;

pub const R_BE_PCIE_MIT0_CNT: c_uint = 0x3334;

pub const R_BE_PCIE_MIT_CH_EN: c_uint = 0x3338;

pub const R_BE_SER_PL1_CTRL: c_uint = 0x34A8;

pub const PCIE_SER_TIMER_UNIT: c_uint = 0x2;
pub const PCIE_SER_WOW_TIMER_UNIT: c_uint = 0x7;

pub const R_BE_REG_PL1_MASK: c_uint = 0x34B0;

pub const R_BE_REG_PL1_ISR: c_uint = 0x34B4;
pub const B_PCIE_SER_ALL_ISR: c_uint = 0x7F;
pub const R_BE_RX_APPEND_MODE: c_uint = 0x8920;

pub const R_BE_TXBD_RWPTR_CLR1: c_uint = 0xB014;

pub const R_BE_RXBD_RWPTR_CLR1_V1: c_uint = 0xB018;

pub const R_BE_HAXI_DMA_BUSY1: c_uint = 0xB01C;

pub const R_BE_HAXI_EXP_CTRL_V1: c_uint = 0xB020;

pub const RTW89_PCI_TXBD_NUM_MAX: c_int = 256;
pub const RTW89_PCI_RXBD_NUM_MAX: c_int = 256;
pub const RTW89_PCI_TXWD_NUM_MAX: c_int = 512;
pub const RTW89_PCI_TXWD_PAGE_SIZE: c_int = 128;
pub const RTW89_PCI_ADDRINFO_MAX: c_int = 4;
// +40 for rtw89_rxdesc_long_v2; +4 for rtw89_pci_rxbd_info

pub const RTW89_PCI_POLL_BDRAM_RST_CNT: c_int = 100;
pub const RTW89_PCI_MULTITAG: c_int = 8;
// PCIE CFG register
pub const RTW89_PCIE_CAPABILITY_SPEED: c_uint = 0x7C;

pub const RTW89_PCIE_L1_STS_V1: c_uint = 0x80;

pub const RTW89_PCIE_GEN1_SPEED: c_uint = 0x01;
pub const RTW89_PCIE_GEN2_SPEED: c_uint = 0x02;
pub const RTW89_PCIE_PHY_RATE: c_uint = 0x82;

pub const RTW89_PCIE_LINK_CHANGE_SPEED: c_uint = 0xA0;
pub const RTW89_PCIE_L1SS_STS_V1: c_uint = 0x0168;

pub const RTW89_PCIE_ASPM_CTRL: c_uint = 0x070F;

pub const RTW89_PCIE_TIMER_CTRL: c_uint = 0x0718;

pub const RTW89_PCIE_L1_CTRL: c_uint = 0x0719;

pub const RTW89_PCIE_CLK_CTRL: c_uint = 0x0725;
pub const RTW89_PCIE_FTS: c_uint = 0x080C;

pub const RTW89_PCIE_RST_MSTATE: c_uint = 0x0B48;

pub const INTF_INTGRA_MINREF_V1: c_int = 90;
pub const INTF_INTGRA_HOSTREF_V1: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pcie_phy {
    PCIE_PHY_GEN1,
    PCIE_PHY_GEN2,
    PCIE_PHY_GEN1_UNDEFINE = 0x7F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pcie_l0sdly {
    PCIE_L0SDLY_1US = 0,
    PCIE_L0SDLY_2US = 1,
    PCIE_L0SDLY_3US = 2,
    PCIE_L0SDLY_4US = 3,
    PCIE_L0SDLY_5US = 4,
    PCIE_L0SDLY_6US = 5,
    PCIE_L0SDLY_7US = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pcie_l1dly {
    PCIE_L1DLY_16US = 4,
    PCIE_L1DLY_32US = 5,
    PCIE_L1DLY_64US = 6,
    PCIE_L1DLY_HW_INFI = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pcie_clkdly_hw {
    PCIE_CLKDLY_HW_0 = 0,
    PCIE_CLKDLY_HW_30US = 0x1,
    PCIE_CLKDLY_HW_50US = 0x2,
    PCIE_CLKDLY_HW_100US = 0x3,
    PCIE_CLKDLY_HW_150US = 0x4,
    PCIE_CLKDLY_HW_200US = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pcie_clkdly_hw_v1 {
    PCIE_CLKDLY_HW_V1_0 = 0,
    PCIE_CLKDLY_HW_V1_16US = 0x1,
    PCIE_CLKDLY_HW_V1_32US = 0x2,
    PCIE_CLKDLY_HW_V1_64US = 0x3,
    PCIE_CLKDLY_HW_V1_80US = 0x4,
    PCIE_CLKDLY_HW_V1_96US = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_bd_trunc_mode {
    MAC_AX_BD_NORM,
    MAC_AX_BD_TRUNC,
    MAC_AX_BD_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_rxbd_mode {
    MAC_AX_RXBD_PKT,
    MAC_AX_RXBD_SEP,
    MAC_AX_RXBD_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_tag_mode {
    MAC_AX_TAG_SGL,
    MAC_AX_TAG_MULTI,
    MAC_AX_TAG_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_tx_burst {
    MAC_AX_TX_BURST_16B = 0,
    MAC_AX_TX_BURST_32B = 1,
    MAC_AX_TX_BURST_64B = 2,
    MAC_AX_TX_BURST_V1_64B = 0,
    MAC_AX_TX_BURST_128B = 3,
    MAC_AX_TX_BURST_V1_128B = 1,
    MAC_AX_TX_BURST_256B = 4,
    MAC_AX_TX_BURST_V1_256B = 2,
    MAC_AX_TX_BURST_512B = 5,
    MAC_AX_TX_BURST_1024B = 6,
    MAC_AX_TX_BURST_2048B = 7,
    MAC_AX_TX_BURST_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_rx_burst {
    MAC_AX_RX_BURST_16B = 0,
    MAC_AX_RX_BURST_32B = 1,
    MAC_AX_RX_BURST_64B = 2,
    MAC_AX_RX_BURST_V1_64B = 0,
    MAC_AX_RX_BURST_128B = 3,
    MAC_AX_RX_BURST_V1_128B = 1,
    MAC_AX_RX_BURST_V1_256B = 0,
    MAC_AX_RX_BURST_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_wd_dma_intvl {
    MAC_AX_WD_DMA_INTVL_0S,
    MAC_AX_WD_DMA_INTVL_256NS,
    MAC_AX_WD_DMA_INTVL_512NS,
    MAC_AX_WD_DMA_INTVL_768NS,
    MAC_AX_WD_DMA_INTVL_1US,
    MAC_AX_WD_DMA_INTVL_1_5US,
    MAC_AX_WD_DMA_INTVL_2US,
    MAC_AX_WD_DMA_INTVL_4US,
    MAC_AX_WD_DMA_INTVL_8US,
    MAC_AX_WD_DMA_INTVL_16US,
    MAC_AX_WD_DMA_INTVL_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_multi_tag_num {
    MAC_AX_TAG_NUM_1,
    MAC_AX_TAG_NUM_2,
    MAC_AX_TAG_NUM_3,
    MAC_AX_TAG_NUM_4,
    MAC_AX_TAG_NUM_5,
    MAC_AX_TAG_NUM_6,
    MAC_AX_TAG_NUM_7,
    MAC_AX_TAG_NUM_8,
    MAC_AX_TAG_NUM_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_lbc_tmr {
    MAC_AX_LBC_TMR_8US = 0,
    MAC_AX_LBC_TMR_16US,
    MAC_AX_LBC_TMR_32US,
    MAC_AX_LBC_TMR_64US,
    MAC_AX_LBC_TMR_128US,
    MAC_AX_LBC_TMR_256US,
    MAC_AX_LBC_TMR_512US,
    MAC_AX_LBC_TMR_1MS,
    MAC_AX_LBC_TMR_2MS,
    MAC_AX_LBC_TMR_4MS,
    MAC_AX_LBC_TMR_8MS,
    MAC_AX_LBC_TMR_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_pcie_func_ctrl {
    MAC_AX_PCIE_DISABLE = 0,
    MAC_AX_PCIE_ENABLE = 1,
    MAC_AX_PCIE_DEFAULT = 0xFE,
    MAC_AX_PCIE_IGNORE = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_ax_io_rcy_tmr {
    MAC_AX_IO_RCY_ANA_TMR_2MS = 24000,
    MAC_AX_IO_RCY_ANA_TMR_4MS = 48000,
    MAC_AX_IO_RCY_ANA_TMR_6MS = 72000,
    MAC_AX_IO_RCY_ANA_TMR_DEF = 0xFE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pci_intr_mask_cfg {
    RTW89_PCI_INTR_MASK_RESET,
    RTW89_PCI_INTR_MASK_NORMAL,
    RTW89_PCI_INTR_MASK_LOW_POWER,
    RTW89_PCI_INTR_MASK_RECOVERY_START,
    RTW89_PCI_INTR_MASK_RECOVERY_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_bd_idx_addr {
    pub tx_bd_addrs: [u32; RTW89_TXCH_NUM],
    pub rx_bd_addrs: [u32; RTW89_RXCH_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_ch_dma_addr {
    pub /: *mut *mut u32 num; / also `offset` addr for group_bd_addr design,
    pub idx: u32,
    pub bdram: u32,
    pub desa_l: u32,
    pub desa_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_ch_dma_addr_set {
    pub tx: [rtw89_pci_ch_dma_addr; RTW89_TXCH_NUM],
    pub rx: [rtw89_pci_ch_dma_addr; RTW89_RXCH_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_bd_ram {
    pub start_idx: u8,
    pub max_num: u8,
    pub min_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_isr_def {
    pub isr_rdu: u32,
    pub isr_halt_c2h: u32,
    pub isr_wdt_timeout: u32,
    pub isr_sps_ocp: u32,
    pub isr_clear_rpq: rtw89_reg2_def,
    pub isr_clear_rxq: rtw89_reg2_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_gen_def {
    pub rtwdev): *mut *mut int (mac_pre_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (mac_pre_deinit)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (mac_post_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (clr_idx_all)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (rst_bdram)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (lv1rst_stop_dma)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (lv1rst_start_dma)(struct rtw89_dev,
    pub enable): *mut *mut *mut void (ctrl_txdma_ch)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (ctrl_txdma_fw_ch)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut int (poll_txdma_ch_idle)(struct rtw89_dev,
    pub enable): *mut *mut *mut void (aspm_set)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (clkreq_set)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (l1ss_set)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut void (disable_eq)(struct rtw89_dev,
    pub pwr_up): *mut *mut *mut void (power_wake)(struct rtw89_dev rtwdev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_ssid_quirk {
    pub vendor: c_ushort,
    pub device: c_ushort,
    pub subsystem_vendor: c_ushort,
    pub subsystem_device: c_ushort,
    pub custid: rtw89_custid,
    pub /: *mut *mut unsigned long bitmap; / bitmap of rtw89_quirks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rpp_info {
    pub seq: u16,
    pub qsel: u8,
    pub tx_status: u8,
    pub txch: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_info {
    pub gen_def: *const rtw89_pci_gen_def,
    pub isr_def: *const rtw89_pci_isr_def,
    pub txbd_trunc_mode: mac_ax_bd_trunc_mode,
    pub rxbd_trunc_mode: mac_ax_bd_trunc_mode,
    pub rxbd_mode: mac_ax_rxbd_mode,
    pub tag_mode: mac_ax_tag_mode,
    pub tx_burst: mac_ax_tx_burst,
    pub rx_burst: mac_ax_rx_burst,
    pub wd_dma_idle_intvl: mac_ax_wd_dma_intvl,
    pub wd_dma_act_intvl: mac_ax_wd_dma_intvl,
    pub multi_tag_num: mac_ax_multi_tag_num,
    pub lbc_en: mac_ax_pcie_func_ctrl,
    pub lbc_tmr: mac_ax_lbc_tmr,
    pub autok_en: mac_ax_pcie_func_ctrl,
    pub io_rcy_en: mac_ax_pcie_func_ctrl,
    pub io_rcy_tmr: mac_ax_io_rcy_tmr,
    pub rx_ring_eq_is_full: bool,
    pub check_rx_tag: bool,
    pub no_rxbd_fs: bool,
    pub group_bd_addr: bool,
    pub rpp_fmt_size: u32,
    pub init_cfg_reg: u32,
    pub txhci_en_bit: u32,
    pub rxhci_en_bit: u32,
    pub rxbd_mode_bit: u32,
    pub exp_ctrl_reg: u32,
    pub max_tag_num_mask: u32,
    pub rxbd_rwptr_clr_reg: u32,
    pub txbd_rwptr_clr2_reg: u32,
    pub dma_io_stop: rtw89_reg_def,
    pub dma_stop1: rtw89_reg_def,
    pub dma_stop2: rtw89_reg_def,
    pub dma_busy1: rtw89_reg_def,
    pub dma_busy2_reg: u32,
    pub dma_busy3_reg: u32,
    pub rpwm_addr: u32,
    pub cpwm_addr: u32,
    pub mit_addr: u32,
    pub wp_sel_addr: u32,
    pub tx_dma_ch_mask: u32,
    pub bd_idx_addr_low_power: *const rtw89_pci_bd_idx_addr,
    pub dma_addr_set: *const rtw89_pci_ch_dma_addr_set,
    pub (*bd_ram_table)[RTW89_TXCH_NUM]: *const rtw89_pci_bd_ram,
    pub en): *mut *mut *mut int (ltr_set)(struct rtw89_dev rtwdev, bool,
    pub add_info_nr): *mut dma_addr_t dma, u8,
    pub rpp_info): *mut rtw89_pci_rpp_info,
    pub rtwdev): *mut *mut void (config_intr_mask)(struct rtw89_dev,
    pub rtwpci): *mut *mut *mut void (enable_intr)(struct rtw89_dev rtwdev, struct rtw89_pci,
    pub rtwpci): *mut *mut *mut void (disable_intr)(struct rtw89_dev rtwdev, struct rtw89_pci,
    pub isrs): *mut rtw89_pci_isrs,
    pub ssid_quirks: *const rtw89_pci_ssid_quirk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_data {
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rx_info {
    pub dma: dma_addr_t,
    pub len:14: u32 fs:1, ls:1, tag:13,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_bd_32 {
    pub length: __le16,
    pub opt: __le16,

    pub dma: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_wp_info {
    pub seq0: __le16,
    pub seq1: __le16,
    pub seq2: __le16,
    pub seq3: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_addr_info_32 {
    pub length: __le16,
    pub option: __le16,
    pub dma: __le32,
    pub __packed: },
pub const RTW89_TXADDR_INFO_NR_V1: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_addr_info_32_v1 {
    pub length_opt: __le16,

    pub dma_low_lsb: __le16,
    pub dma_low_msb: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rpp_fmt {
    pub dword: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rpp_fmt_v1 {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rx_bd_32 {
    pub buf_size: __le16,
    pub opt: __le16,

    pub dma: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rxbd_info {
    pub dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_wd {
    pub list: list_head,
    pub queue: sk_buff_head,
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub len: u32,
    pub seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_dma_ring {
    pub head: *mut c_void,
    pub desc_size: u8,
    pub dma: dma_addr_t,
    pub addr: rtw89_pci_ch_dma_addr,
    pub len: u32,
    pub /: *mut *mut u32 wp; / host idx,
    pub /: *mut *mut u32 rp; / hw idx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_dma_pool {
    pub head: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_wd_ring {
    pub head: *mut c_void,
    pub dma: dma_addr_t,
    pub pages: [rtw89_pci_tx_wd; RTW89_PCI_TXWD_NUM_MAX],
    pub free_pages: list_head,
    pub page_size: u32,
    pub page_num: u32,
    pub curr_num: u32,
}

pub const RTW89_RX_TAG_MAX: c_uint = 0x1fff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_ring {
    pub wd_ring: rtw89_pci_tx_wd_ring,
    pub bd_ring: rtw89_pci_dma_ring,
    pub busy_pages: list_head,
    pub txch: u8,
    pub dma_enabled: bool,
    pub /: *mut *mut u16 tag; / range from 0x0001 ~ 0x1fff,
    pub tx_cnt: u64,
    pub tx_acked: u64,
    pub tx_retry_lmt: u64,
    pub tx_life_time: u64,
    pub tx_mac_id_drop: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_tx_rings {
    pub rings: [rtw89_pci_tx_ring; RTW89_TXCH_NUM],
    pub bd_pool: rtw89_pci_dma_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rx_ring {
    pub bd_ring: rtw89_pci_dma_ring,
    pub buf: [*mut sk_buff; RTW89_PCI_RXBD_NUM_MAX],
    pub buf_sz: u32,
    pub diliver_skb: *mut sk_buff,
    pub diliver_desc: rtw89_rx_desc_info,
    pub target_rx_tag:13: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_rx_rings {
    pub rings: [rtw89_pci_rx_ring; RTW89_RXCH_NUM],
    pub bd_pool: rtw89_pci_dma_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci_isrs {
    pub ind_isrs: u32,
    pub halt_c2h_isrs: u32,
    pub isrs: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pci {
    pub pdev: *mut pci_dev,
// protect HW irq related registers
    pub irq_lock: spinlock_t,
// protect TRX resources (exclude RXQ)
    pub trx_lock: spinlock_t,
    pub running: bool,
    pub low_power: bool,
    pub under_recovery: bool,
    pub enable_dac: bool,
    pub tx: rtw89_pci_tx_rings,
    pub rx: rtw89_pci_rx_rings,
    pub h2c_queue: sk_buff_head,
    pub h2c_release_queue: sk_buff_head,
    pub RTW89_TXCH_NUM): DECLARE_BITMAP(kick_map,,
    pub ind_intrs: u32,
    pub halt_c2h_intrs: u32,
    pub intrs: [u32; 2],
    pub mmap: *mut void __iomem,
}

extern "C" {
    pub fn rtw89_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn rtw89_pci_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn rtw89_pci_shutdown(pdev: *mut pci_dev);
}
extern "C" {
    pub fn rtw89_pci_basic_cfg(rtwdev: *mut rtw89_dev, resume: bool);
}
extern "C" {
    pub fn rtw89_pci_ops_reset(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_pci_ltr_set(rtwdev: *mut rtw89_dev, en: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_pci_ltr_set_v1(rtwdev: *mut rtw89_dev, en: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_pci_ltr_set_v2(rtwdev: *mut rtw89_dev, en: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_pci_ctrl_dma_all(rtwdev: *mut rtw89_dev, enable: bool);
}
extern "C" {
    pub fn rtw89_pci_config_intr_mask(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_pci_config_intr_mask_v1(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_pci_config_intr_mask_v2(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_pci_config_intr_mask_v3(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_pci_enable_intr(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_disable_intr(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_enable_intr_v1(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_disable_intr_v1(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_enable_intr_v2(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_disable_intr_v2(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_enable_intr_v3(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
extern "C" {
    pub fn rtw89_pci_disable_intr_v3(rtwdev: *mut rtw89_dev, rtwpci: *mut rtw89_pci);
}
