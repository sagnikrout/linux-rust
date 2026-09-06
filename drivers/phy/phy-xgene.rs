//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-xgene.c
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
// AppliedMicro X-Gene Multi-purpose PHY driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Author: Loc Ho <lho@apm.com>
// Tuan Phan <tphan@apm.com>
// Suman Tripathi <stripathi@apm.com>
//
// The APM X-Gene PHY consists of two PLL clock macro's (CMU) and lanes.
// The first PLL clock macro is used for internal reference clock. The second
// PLL clock macro is used to generate the clock for the PHY. This driver
// configures the first PLL CMU, the second PLL CMU, and programs the PHY to
// operate according to the mode of operation. The first PLL CMU is only
// required if internal clock is enabled.
//
// Logical Layer Out Of HW module units:
//
// -----------------
// | Internal      |    |------|
// | Ref PLL CMU   |----|      |     -------------    ---------
// ------------ ----    | MUX  |-----|PHY PLL CMU|----| Serdes|
// |      |     |           |    ---------
// External Clock ------|      |     -------------
// |------|
//
// The Ref PLL CMU CSR (Configuration System Registers) is accessed
// indirectly from the SDS offset at 0x2000. It is only required for
// internal reference clock.
// The PHY PLL CMU CSR is accessed indirectly from the SDS offset at 0x0000.
// The Serdes CSR is accessed indirectly from the SDS offset at 0x0400.
//
// The Ref PLL CMU can be located within the same PHY IP or outside the PHY IP
// due to shared Ref PLL CMU. For PHY with Ref PLL CMU shared with another IP,
// it is located outside the PHY IP. This is the case for the PHY located
// at 0x1f23a000 (SATA Port 4/5). For such PHY, another resource is required
// to located the SDS/Ref PLL CMU module and its clock for that IP enabled.
//
// Currently, this driver only supports Gen3 SATA mode with external clock.
//

// Max 2 lanes per a PHY unit
pub const MAX_LANE: c_int = 2;
// Register offset inside the PHY
pub const SERDES_PLL_INDIRECT_OFFSET: c_uint = 0x0000;
pub const SERDES_PLL_REF_INDIRECT_OFFSET: c_uint = 0x2000;
pub const SERDES_INDIRECT_OFFSET: c_uint = 0x0400;
pub const SERDES_LANE_STRIDE: c_uint = 0x0200;
// Some default Serdes parameters

pub const SATA_SPD_SEL_GEN3: c_uint = 0x7;
pub const SATA_SPD_SEL_GEN2: c_uint = 0x3;
pub const SATA_SPD_SEL_GEN1: c_uint = 0x1;
pub const SSC_DISABLE: c_int = 0;
pub const SSC_ENABLE: c_int = 1;
pub const FBDIV_VAL_50M: c_uint = 0x77;
pub const REFDIV_VAL_50M: c_uint = 0x1;
pub const FBDIV_VAL_100M: c_uint = 0x3B;
pub const REFDIV_VAL_100M: c_uint = 0x0;
// SATA Clock/Reset CSR
pub const SATACLKENREG: c_uint = 0x00000000;
pub const SATA0_CORE_CLKEN: c_uint = 0x00000002;
pub const SATA1_CORE_CLKEN: c_uint = 0x00000004;
pub const SATASRESETREG: c_uint = 0x00000004;
pub const SATA_MEM_RESET_MASK: c_uint = 0x00000020;

pub const SATA_SDS_RESET_MASK: c_uint = 0x00000004;
pub const SATA_CSR_RESET_MASK: c_uint = 0x00000001;
pub const SATA_CORE_RESET_MASK: c_uint = 0x00000002;
pub const SATA_PMCLK_RESET_MASK: c_uint = 0x00000010;
pub const SATA_PCLK_RESET_MASK: c_uint = 0x00000008;
// SDS CSR used for PHY Indirect access
pub const SATA_ENET_SDS_PCS_CTL0: c_uint = 0x00000000;

    (((dst) & ~0x00070000) | (((u32) (src) << 16) & 0x00070000))

    (((dst) & ~0x00e00000) | (((u32) (src) << 21) & 0x00e00000))
pub const SATA_ENET_SDS_CTL0: c_uint = 0x0000000c;

    (((dst) & ~0x00007fff) | (((u32) (src)) & 0x00007fff))
pub const SATA_ENET_SDS_CTL1: c_uint = 0x00000010;

    (((dst) & ~0x0000000f) | (((u32) (src)) & 0x0000000f))
pub const SATA_ENET_SDS_RST_CTL: c_uint = 0x00000024;
pub const SATA_ENET_SDS_IND_CMD_REG: c_uint = 0x0000003c;
pub const CFG_IND_WR_CMD_MASK: c_uint = 0x00000001;
pub const CFG_IND_RD_CMD_MASK: c_uint = 0x00000002;
pub const CFG_IND_CMD_DONE_MASK: c_uint = 0x00000004;

    (((dst) & ~0x003ffff0) | (((u32) (src) << 4) & 0x003ffff0))
pub const SATA_ENET_SDS_IND_RDATA_REG: c_uint = 0x00000040;
pub const SATA_ENET_SDS_IND_WDATA_REG: c_uint = 0x00000044;
pub const SATA_ENET_CLK_MACRO_REG: c_uint = 0x0000004c;

    (((dst) & ~0x00000001) | (((u32) (src)) & 0x00000001))

    (((dst) & ~0x001ff000) | (((u32) (src) << 12) & 0x001ff000))

    (((dst) & ~0x00000f80) | (((u32) (src) << 7) & 0x00000f80))

// PLL Clock Macro Unit (CMU) CSR accessing from SDS indirectly
pub const CMU_REG0: c_uint = 0x00000;
pub const CMU_REG0_PLL_REF_SEL_MASK: c_uint = 0x00002000;

    (((dst) & ~0x00002000) | (((u32) (src) << 13) & 0x00002000))
pub const CMU_REG0_PDOWN_MASK: c_uint = 0x00004000;

    (((dst) & ~0x000000e0) | (((u32) (src) << 5) & 0x000000e0))
pub const CMU_REG1: c_uint = 0x00002;

    (((dst) & ~0x00003c00) | (((u32) (src) << 10) & 0x00003c00))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))

    (((dst) & ~0x000003e0) | (((u32) (src) << 5) & 0x000003e0))
pub const CMU_REG1_REFCLK_CMOS_SEL_MASK: c_uint = 0x00000001;

    (((dst) & ~0x00000001) | (((u32) (src) << 0) & 0x00000001))
pub const CMU_REG2: c_uint = 0x00004;

    (((dst) & ~0x0000c000) | (((u32) (src) << 14) & 0x0000c000))

    (((dst) & ~0x0000001e) | (((u32) (src) << 1) & 0x0000001e))

    (((dst) & ~0x00003fe0) | (((u32) (src) << 5) & 0x00003fe0))
pub const CMU_REG3: c_uint = 0x00006;

    (((dst) & ~0x0000000f) | (((u32) (src) << 0) & 0x0000000f))

    (((dst) & ~0x000003f0) | (((u32) (src) << 4) & 0x000003f0))

    (((dst) & ~0x0000fc00) | (((u32) (src) << 10) & 0x0000fc00))
pub const CMU_REG4: c_uint = 0x00008;
pub const CMU_REG5: c_uint = 0x0000a;

    (((dst) & ~0x0000c000) | (((u32) (src) << 14) & 0x0000c000))

    (((dst) & ~0x0000000e) | (((u32) (src) << 1) & 0x0000000e))

    (((dst) & ~0x00003000) | (((u32) (src) << 12) & 0x00003000))
pub const CMU_REG5_PLL_RESETB_MASK: c_uint = 0x00000001;
pub const CMU_REG6: c_uint = 0x0000c;

    (((dst) & ~0x00000600) | (((u32) (src) << 9) & 0x00000600))

    (((dst) & ~0x00000004) | (((u32) (src) << 2) & 0x00000004))
pub const CMU_REG7: c_uint = 0x0000e;

pub const CMU_REG8: c_uint = 0x00010;
pub const CMU_REG9: c_uint = 0x00012;
pub const CMU_REG9_WORD_LEN_8BIT: c_uint = 0x000;
pub const CMU_REG9_WORD_LEN_10BIT: c_uint = 0x001;
pub const CMU_REG9_WORD_LEN_16BIT: c_uint = 0x002;
pub const CMU_REG9_WORD_LEN_20BIT: c_uint = 0x003;
pub const CMU_REG9_WORD_LEN_32BIT: c_uint = 0x004;
pub const CMU_REG9_WORD_LEN_40BIT: c_uint = 0x005;
pub const CMU_REG9_WORD_LEN_64BIT: c_uint = 0x006;
pub const CMU_REG9_WORD_LEN_66BIT: c_uint = 0x007;

    (((dst) & ~0x00000380) | (((u32) (src) << 7) & 0x00000380))

    (((dst) & ~0x00000070) | (((u32) (src) << 4) & 0x00000070))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))

    (((dst) & ~0x00000004) | (((u32) (src) << 2) & 0x00000004))

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))
pub const CMU_REG10: c_uint = 0x00014;

    (((dst) & ~0x00000001) | (((u32) (src) << 0) & 0x00000001))
pub const CMU_REG11: c_uint = 0x00016;
pub const CMU_REG12: c_uint = 0x00018;

    (((dst) & ~0x000000f0) | (((u32) (src) << 4) & 0x000000f0))
pub const CMU_REG13: c_uint = 0x0001a;
pub const CMU_REG14: c_uint = 0x0001c;
pub const CMU_REG15: c_uint = 0x0001e;
pub const CMU_REG16: c_uint = 0x00020;
pub const CMU_REG16_PVT_DN_MAN_ENA_MASK: c_uint = 0x00000001;
pub const CMU_REG16_PVT_UP_MAN_ENA_MASK: c_uint = 0x00000002;

    (((dst) & ~0x0000001c) | (((u32) (src) << 2) & 0x0000001c))

    (((dst) & ~0x00000040) | (((u32) (src) << 6) & 0x00000040))

    (((dst) & ~0x00000020) | (((u32) (src) << 5) & 0x00000020))
pub const CMU_REG17: c_uint = 0x00022;

    (((dst) & ~0x00007f00) | (((u32) (src) << 8) & 0x00007f00))

    (((dst) & ~0x000000e0) | (((u32) (src) << 5) & 0x000000e0))
pub const CMU_REG17_PVT_TERM_MAN_ENA_MASK: c_uint = 0x00008000;
pub const CMU_REG18: c_uint = 0x00024;
pub const CMU_REG19: c_uint = 0x00026;
pub const CMU_REG20: c_uint = 0x00028;
pub const CMU_REG21: c_uint = 0x0002a;
pub const CMU_REG22: c_uint = 0x0002c;
pub const CMU_REG23: c_uint = 0x0002e;
pub const CMU_REG24: c_uint = 0x00030;
pub const CMU_REG25: c_uint = 0x00032;
pub const CMU_REG26: c_uint = 0x00034;

    (((dst) & ~0x00000001) | (((u32) (src) << 0) & 0x00000001))
pub const CMU_REG27: c_uint = 0x00036;
pub const CMU_REG28: c_uint = 0x00038;
pub const CMU_REG29: c_uint = 0x0003a;
pub const CMU_REG30: c_uint = 0x0003c;

    (((dst) & ~0x00000006) | (((u32) (src) << 1) & 0x00000006))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))
pub const CMU_REG31: c_uint = 0x0003e;
pub const CMU_REG32: c_uint = 0x00040;
pub const CMU_REG32_FORCE_VCOCAL_START_MASK: c_uint = 0x00004000;

    (((dst) & ~0x00000006) | (((u32) (src) << 1) & 0x00000006))

    (((dst) & ~0x00000180) | (((u32) (src) << 7) & 0x00000180))
pub const CMU_REG33: c_uint = 0x00042;
pub const CMU_REG34: c_uint = 0x00044;

    (((dst) & ~0x0000000f) | (((u32) (src) << 0) & 0x0000000f))

    (((dst) & ~0x00000f00) | (((u32) (src) << 8) & 0x00000f00))

    (((dst) & ~0x000000f0) | (((u32) (src) << 4) & 0x000000f0))

    (((dst) & ~0x0000f000) | (((u32) (src) << 12) & 0x0000f000))
pub const CMU_REG35: c_uint = 0x00046;

    (((dst) & ~0x0000fe00) | (((u32) (src) << 9) & 0x0000fe00))
pub const CMU_REG36: c_uint = 0x00048;

    (((dst) & ~0x00000010) | (((u32) (src) << 4) & 0x00000010))

    (((dst) & ~0x0000ffc0) | (((u32) (src) << 6) & 0x0000ffc0))

    (((dst) & ~0x00000020) | (((u32) (src) << 5) & 0x00000020))
pub const CMU_REG37: c_uint = 0x0004a;
pub const CMU_REG38: c_uint = 0x0004c;
pub const CMU_REG39: c_uint = 0x0004e;
// PHY lane CSR accessing from SDS indirectly
pub const RXTX_REG0: c_uint = 0x000;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))

    (((dst) & ~0x000007c0) | (((u32) (src) << 6) & 0x000007c0))

    (((dst) & ~0x0000003e) | (((u32) (src) << 1) & 0x0000003e))
pub const RXTX_REG1: c_uint = 0x002;

    (((dst) & ~0x0000f000) | (((u32) (src) << 12) & 0x0000f000))

    (((dst) & ~0x00000f80) | (((u32) (src) << 7) & 0x00000f80))

    (((dst) & ~0x00000060) | (((u32) (src) << 5) & 0x00000060))

    (((dst) & ~0x00000006) | (((u32) (src) << 1) &  0x00000006))
pub const RXTX_REG2: c_uint = 0x004;

    (((dst) & ~0x00000100) | (((u32) (src) << 8) & 0x00000100))

    (((dst) & ~0x00000020) | (((u32) (src) << 5) & 0x00000020))

    (((dst) & ~0x000000c0) | (((u32) (src) << 6) & 0x000000c0))
pub const RXTX_REG4: c_uint = 0x008;
pub const RXTX_REG4_TX_LOOPBACK_BUF_EN_MASK: c_uint = 0x00000040;

    (((dst) & ~0x0000c000) | (((u32) (src) << 14) & 0x0000c000))

    (((dst) & ~0x00003800) | (((u32) (src) << 11) & 0x00003800))
pub const RXTX_REG5: c_uint = 0x00a;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))

    (((dst) & ~0x000007e0) | (((u32) (src) << 5) & 0x000007e0))

    (((dst) & ~0x0000001f) | (((u32) (src) << 0) & 0x0000001f))
pub const RXTX_REG6: c_uint = 0x00c;

    (((dst) & ~0x00000780) | (((u32) (src) << 7) & 0x00000780))

    (((dst) & ~0x00000040) | (((u32) (src) << 6) & 0x00000040))

    (((dst) & ~0x00000001) | (((u32) (src) << 0) & 0x00000001))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))
pub const RXTX_REG7: c_uint = 0x00e;
pub const RXTX_REG7_RESETB_RXD_MASK: c_uint = 0x00000100;
pub const RXTX_REG7_RESETB_RXA_MASK: c_uint = 0x00000080;

    (((dst) & ~0x00000040) | (((u32) (src) << 6) & 0x00000040))

    (((dst) & ~0x00003800) | (((u32) (src) << 11) & 0x00003800))
pub const RXTX_REG8: c_uint = 0x010;

    (((dst) & ~0x00004000) | (((u32) (src) << 14) & 0x00004000))

    (((dst) & ~0x00000800) | (((u32) (src) << 11) & 0x00000800))

    (((dst) & ~0x00000200) | (((u32) (src) << 9) & 0x00000200))

    (((dst) & ~0x000000f0) | (((u32) (src) << 4) & 0x000000f0))

    (((dst) & ~0x00000100) | (((u32) (src) << 8) & 0x00000100))
pub const RXTX_REG7: c_uint = 0x00e;

    (((dst) & ~0x00000100) | (((u32) (src) << 8) & 0x00000100))

    (((dst) & ~0x00000080) | (((u32) (src) << 7) & 0x00000080))
pub const RXTX_REG7_LOOP_BACK_ENA_CTLE_MASK: c_uint = 0x00004000;

    (((dst) & ~0x00004000) | (((u32) (src) << 14) & 0x00004000))
pub const RXTX_REG11: c_uint = 0x016;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))
pub const RXTX_REG12: c_uint = 0x018;

    (((dst) & ~0x00002000) | (((u32) (src) << 13) & 0x00002000))

    (((dst) & ~0x00000004) | (((u32) (src) << 2) & 0x00000004))
pub const RXTX_REG12_RX_DET_TERM_ENABLE_MASK: c_uint = 0x00000002;

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))
pub const RXTX_REG13: c_uint = 0x01a;
pub const RXTX_REG14: c_uint = 0x01c;

    (((dst) & ~0x0000003f) | (((u32) (src) << 0) & 0x0000003f))

    (((dst) & ~0x00000040) | (((u32) (src) << 6) & 0x00000040))
pub const RXTX_REG26: c_uint = 0x034;

    (((dst) & ~0x00003800) | (((u32) (src) << 11) & 0x00003800))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))
pub const RXTX_REG21: c_uint = 0x02a;

pub const RXTX_REG22: c_uint = 0x02c;

pub const RXTX_REG23: c_uint = 0x02e;

pub const RXTX_REG24: c_uint = 0x030;

pub const RXTX_REG27: c_uint = 0x036;
pub const RXTX_REG28: c_uint = 0x038;
pub const RXTX_REG31: c_uint = 0x03e;
pub const RXTX_REG38: c_uint = 0x04c;

    (((dst) & 0x0000fffe) | (((u32) (src) << 1) & 0x0000fffe))
pub const RXTX_REG39: c_uint = 0x04e;
pub const RXTX_REG40: c_uint = 0x050;
pub const RXTX_REG41: c_uint = 0x052;
pub const RXTX_REG42: c_uint = 0x054;
pub const RXTX_REG43: c_uint = 0x056;
pub const RXTX_REG44: c_uint = 0x058;
pub const RXTX_REG45: c_uint = 0x05a;
pub const RXTX_REG46: c_uint = 0x05c;
pub const RXTX_REG47: c_uint = 0x05e;
pub const RXTX_REG48: c_uint = 0x060;
pub const RXTX_REG49: c_uint = 0x062;
pub const RXTX_REG50: c_uint = 0x064;
pub const RXTX_REG51: c_uint = 0x066;
pub const RXTX_REG52: c_uint = 0x068;
pub const RXTX_REG53: c_uint = 0x06a;
pub const RXTX_REG54: c_uint = 0x06c;
pub const RXTX_REG55: c_uint = 0x06e;
pub const RXTX_REG61: c_uint = 0x07a;

    (((dst) & ~0x00000010) | (((u32) (src) << 4) & 0x00000010))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))

    (((dst) & ~0x000000c0) | (((u32) (src) << 6) & 0x000000c0))

    (((dst) & ~0x00003c00) | (((u32) (src) << 10) & 0x00003c00))
pub const RXTX_REG62: c_uint = 0x07c;

    (((dst) & ~0x00003800) | (((u32) (src) << 11) & 0x00003800))
pub const RXTX_REG81: c_uint = 0x0a2;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))

    (((dst) & ~0x000007c0) | (((u32) (src) << 6) & 0x000007c0))

    (((dst) & ~0x0000003e) | (((u32) (src) << 1) & 0x0000003e))
pub const RXTX_REG96: c_uint = 0x0c0;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))

    (((dst) & ~0x000007c0) | (((u32) (src) << 6) & 0x000007c0))

    (((dst) & ~0x0000003e) | (((u32) (src) << 1) & 0x0000003e))
pub const RXTX_REG99: c_uint = 0x0c6;

    (((dst) & ~0x0000f800) | (((u32) (src) << 11) & 0x0000f800))

    (((dst) & ~0x000007c0) | (((u32) (src) << 6) & 0x000007c0))

    (((dst) & ~0x0000003e) | (((u32) (src) << 1) & 0x0000003e))
pub const RXTX_REG102: c_uint = 0x0cc;

    (((dst) & ~0x00000060) | (((u32) (src) << 5) & 0x00000060))
pub const RXTX_REG114: c_uint = 0x0e4;
pub const RXTX_REG121: c_uint = 0x0f2;

pub const RXTX_REG125: c_uint = 0x0fa;

    (((dst) & ~0x0000fe00) | (((u32) (src) << 9) & 0x0000fe00))

    (((dst) & ~0x00000100) | (((u32) (src) << 8) & 0x00000100))

    (((dst) & ~0x00000080) | (((u32) (src) << 7) & 0x00000080))

    (((dst) & ~0x0000007c) | (((u32) (src) << 2) & 0x0000007c))

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))
pub const RXTX_REG127: c_uint = 0x0fe;
pub const RXTX_REG127_FORCE_SUM_CAL_START_MASK: c_uint = 0x00000002;
pub const RXTX_REG127_FORCE_LAT_CAL_START_MASK: c_uint = 0x00000004;

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))

    (((dst) & ~0x00000004) | (((u32) (src) << 2) & 0x00000004))

    (((dst) & ~0x00000008) | (((u32) (src) << 3) & 0x00000008))

    (((dst) & ~0x0000fc00) | (((u32) (src) << 10) & 0x0000fc00))

    (((dst) & ~0x000003f0) | (((u32) (src) << 4) & 0x000003f0))
pub const RXTX_REG128: c_uint = 0x100;

    (((dst) & ~0x0000000c) | (((u32) (src) << 2) & 0x0000000c))

    (((dst) & ~0x0000fc00) | (((u32) (src) << 10) & 0x0000fc00))

    (((dst) & ~0x000003f0) | (((u32) (src) << 4) & 0x000003f0))
pub const RXTX_REG129: c_uint = 0x102;

    (((dst) & ~0x0000fc00) | (((u32) (src) << 10) & 0x0000fc00))

    (((dst) & ~0x000003f0) | (((u32) (src) << 4) & 0x000003f0))
pub const RXTX_REG130: c_uint = 0x104;

    (((dst) & ~0x0000fc00) | (((u32) (src) << 10) & 0x0000fc00))

    (((dst) & ~0x000003f0) | (((u32) (src) << 4) & 0x000003f0))
pub const RXTX_REG145: c_uint = 0x122;

    (((dst) & ~0x00000001) | (((u32) (src) << 0) & 0x00000001))

    (((dst) & ~0x00000002) | (((u32) (src) << 1) & 0x00000002))

    (((dst) & ~0x0000c000) | (((u32) (src) << 14) & 0x0000c000))

    (((dst) & ~0x00000004) | (((u32) (src) << 2) & 0x00000004))
pub const RXTX_REG147: c_uint = 0x126;
pub const RXTX_REG148: c_uint = 0x128;
// Clock macro type
    enum cmu_type_t {
    REF_CMU = 0,	/* Clock macro is the internal reference clock */
    PHY_CMU = 1,	/* Clock macro is the PLL for the Serdes */
    };
    enum mux_type_t {
    MUX_SELECT_ATA = 0,	/* Switch the MUX to ATA */
    MUX_SELECT_SGMMII = 0,	/* Switch the MUX to SGMII */
    };
    enum clk_type_t {
    CLK_EXT_DIFF = 0,	/* External differential */
    CLK_INT_DIFF = 1,	/* Internal differential */
    CLK_INT_SING = 2,	/* Internal single ended */
    };
    enum xgene_phy_mode {
    MODE_SATA	= 0,	/* List them for simple reference */
    MODE_SGMII	= 1,
    MODE_PCIE	= 2,
    MODE_USB	= 3,
    MODE_XFI	= 4,
    MODE_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_sata_override_param {
    pub /: *mut *mut u32 speed[MAX_LANE]; / Index for override parameter per lane,
    pub /: *mut *mut u32 txspeed[3]; / Tx speed,
    pub /: *mut *mut *mut u32 txboostgain[MAX_LANE3]; / Tx freq boost and gain control,
    pub /: *mut *mut *mut u32 txeyetuning[MAX_LANE3]; / Tx eye tuning,
    pub /: *mut *mut *mut u32 txeyedirection[MAX_LANE3]; / Tx eye tuning direction,
    pub /: *mut *mut *mut u32 txamplitude[MAX_LANE3]; / Tx amplitude control,
    pub /: *mut *mut *mut u32 txprecursor_cn1[MAX_LANE3]; / Tx emphasis taps 1st pre-cursor,
    pub /: *mut *mut *mut u32 txprecursor_cn2[MAX_LANE3]; / Tx emphasis taps 2nd pre-cursor,
    pub /: *mut *mut *mut u32 txpostcursor_cp1[MAX_LANE3]; / Tx emphasis taps post-cursor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_phy_ctx {
    pub dev: *mut device,
    pub phy: *mut phy,
    pub /: *mut *mut enum xgene_phy_mode mode; / Mode of operation,
    pub /: *mut *mut enum clk_type_t clk_type; / Input clock selection,
    pub /: *mut *mut *mut void __iomem sds_base; / PHY CSR base addr,
    pub /: *mut *mut *mut clk clk; / Optional clock,
// Override Serdes parameters
    pub sata_param: xgene_sata_override_param,
}

//
// For chip earlier than A3 version, enable this flag.
// To enable, pass boot argument phy_xgene.preA3Chip=1
//
    static int preA3Chip;
    MODULE_PARM_DESC(preA3Chip, "Enable pre-A3 chip support (1=enable 0=disable)");
    module_param_named(preA3Chip, preA3Chip, int, 0444);
    static void sds_wr(void __iomem *csr_base, u32 indirect_cmd_reg,
    u32 indirect_data_reg, u32 addr, u32 data)
    {
    let mut deadline: c_ulong = jiffies + HZ;
    u32 val;
    u32 cmd;
    cmd = CFG_IND_WR_CMD_MASK | CFG_IND_CMD_DONE_MASK;
    cmd = CFG_IND_ADDR_SET(cmd, addr);
    writel(data, csr_base + indirect_data_reg);
    readl(csr_base + indirect_data_reg); /* Force a barrier */
    writel(cmd, csr_base + indirect_cmd_reg);
    readl(csr_base + indirect_cmd_reg); /* Force a barrier */
    do {
    val = readl(csr_base + indirect_cmd_reg);
    } while (!(val & CFG_IND_CMD_DONE_MASK) &&
    time_before(jiffies, deadline));
    if (!(val & CFG_IND_CMD_DONE_MASK))
    pr_err("SDS WR timeout at 0x%p offset 0x%08X value 0x%08X\n",
    csr_base + indirect_cmd_reg, addr, data);
    }
    static void sds_rd(void __iomem *csr_base, u32 indirect_cmd_reg,
    u32 indirect_data_reg, u32 addr, u32 *data)
    {
    let mut deadline: c_ulong = jiffies + HZ;
    u32 val;
    u32 cmd;
    cmd = CFG_IND_RD_CMD_MASK | CFG_IND_CMD_DONE_MASK;
    cmd = CFG_IND_ADDR_SET(cmd, addr);
    writel(cmd, csr_base + indirect_cmd_reg);
    readl(csr_base + indirect_cmd_reg); /* Force a barrier */
    do {
    val = readl(csr_base + indirect_cmd_reg);
    } while (!(val & CFG_IND_CMD_DONE_MASK) &&
    time_before(jiffies, deadline));
// data = readl(csr_base + indirect_data_reg);
    if (!(val & CFG_IND_CMD_DONE_MASK))
    pr_err("SDS WR timeout at 0x%p offset 0x%08X value 0x%08X\n",
    csr_base + indirect_cmd_reg, addr, *data);
    }
    static void cmu_wr(struct xgene_phy_ctx *ctx, enum cmu_type_t cmu_type,
    u32 reg, u32 data)
    {
    void __iomem *sds_base = ctx.sds_base;
    u32 val;
    if (cmu_type == REF_CMU)
    reg += SERDES_PLL_REF_INDIRECT_OFFSET;
    else
    reg += SERDES_PLL_INDIRECT_OFFSET;
    sds_wr(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_WDATA_REG, reg, data);
    sds_rd(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_RDATA_REG, reg, &val);
    pr_debug("CMU WR addr 0x%X value 0x%08X <. 0x%08X\n", reg, data, val);
    }
    static void cmu_rd(struct xgene_phy_ctx *ctx, enum cmu_type_t cmu_type,
    u32 reg, u32 *data)
    {
    void __iomem *sds_base = ctx.sds_base;
    if (cmu_type == REF_CMU)
    reg += SERDES_PLL_REF_INDIRECT_OFFSET;
    else
    reg += SERDES_PLL_INDIRECT_OFFSET;
    sds_rd(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_RDATA_REG, reg, data);
    pr_debug("CMU RD addr 0x%X value 0x%08X\n", reg, *data);
    }
    static void cmu_toggle1to0(struct xgene_phy_ctx *ctx, enum cmu_type_t cmu_type,
    u32 reg, u32 bits)
    {
    u32 val;
    cmu_rd(ctx, cmu_type, reg, &val);
    val |= bits;
    cmu_wr(ctx, cmu_type, reg, val);
    cmu_rd(ctx, cmu_type, reg, &val);
    val &= ~bits;
    cmu_wr(ctx, cmu_type, reg, val);
    }
    static void cmu_clrbits(struct xgene_phy_ctx *ctx, enum cmu_type_t cmu_type,
    u32 reg, u32 bits)
    {
    u32 val;
    cmu_rd(ctx, cmu_type, reg, &val);
    val &= ~bits;
    cmu_wr(ctx, cmu_type, reg, val);
    }
    static void cmu_setbits(struct xgene_phy_ctx *ctx, enum cmu_type_t cmu_type,
    u32 reg, u32 bits)
    {
    u32 val;
    cmu_rd(ctx, cmu_type, reg, &val);
    val |= bits;
    cmu_wr(ctx, cmu_type, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn serdes_wr(ctx: *mut xgene_phy_ctx, lane: c_int, reg: u32, data: u32) {
    static void serdes_wr(struct xgene_phy_ctx *ctx, int lane, u32 reg, u32 data)
    {
    void __iomem *sds_base = ctx.sds_base;
    u32 val;
    reg += SERDES_INDIRECT_OFFSET;
    reg += lane * SERDES_LANE_STRIDE;
    sds_wr(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_WDATA_REG, reg, data);
    sds_rd(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_RDATA_REG, reg, &val);
    pr_debug("SERDES WR addr 0x%X value 0x%08X <. 0x%08X\n", reg, data,
    val);
    }
#[no_mangle]
unsafe extern "C" fn serdes_rd(ctx: *mut xgene_phy_ctx, lane: c_int, reg: u32, data: *mut u32) {
    static void serdes_rd(struct xgene_phy_ctx *ctx, int lane, u32 reg, u32 *data)
    {
    void __iomem *sds_base = ctx.sds_base;
    reg += SERDES_INDIRECT_OFFSET;
    reg += lane * SERDES_LANE_STRIDE;
    sds_rd(sds_base, SATA_ENET_SDS_IND_CMD_REG,
    SATA_ENET_SDS_IND_RDATA_REG, reg, data);
    pr_debug("SERDES RD addr 0x%X value 0x%08X\n", reg, *data);
    }
    static void serdes_clrbits(struct xgene_phy_ctx *ctx, int lane, u32 reg,
    u32 bits)
    {
    u32 val;
    serdes_rd(ctx, lane, reg, &val);
    val &= ~bits;
    serdes_wr(ctx, lane, reg, val);
    }
    static void serdes_setbits(struct xgene_phy_ctx *ctx, int lane, u32 reg,
    u32 bits)
    {
    u32 val;
    serdes_rd(ctx, lane, reg, &val);
    val |= bits;
    serdes_wr(ctx, lane, reg, val);
    }
    static void xgene_phy_cfg_cmu_clk_type(struct xgene_phy_ctx *ctx,
    enum cmu_type_t cmu_type,
    enum clk_type_t clk_type)
    {
    u32 val;
// Set the reset sequence delay for TX ready assertion
    cmu_rd(ctx, cmu_type, CMU_REG12, &val);
    val = CMU_REG12_STATE_DELAY9_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG12, val);
// Set the programmable stage delays between various enable stages
    cmu_wr(ctx, cmu_type, CMU_REG13, 0x0222);
    cmu_wr(ctx, cmu_type, CMU_REG14, 0x2225);
// Configure clock type
    if (clk_type == CLK_EXT_DIFF) {
// Select external clock mux
    cmu_rd(ctx, cmu_type, CMU_REG0, &val);
    val = CMU_REG0_PLL_REF_SEL_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG0, val);
// Select CMOS as reference clock
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_REFCLK_CMOS_SEL_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
    dev_dbg(ctx.dev, "Set external reference clock\n");
    } else if (clk_type == CLK_INT_DIFF) {
// Select internal clock mux
    cmu_rd(ctx, cmu_type, CMU_REG0, &val);
    val = CMU_REG0_PLL_REF_SEL_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG0, val);
// Select CMOS as reference clock
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_REFCLK_CMOS_SEL_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
    dev_dbg(ctx.dev, "Set internal reference clock\n");
    } else if (clk_type == CLK_INT_SING) {
//
// NOTE: This clock type is NOT support for controller
// whose internal clock shared in the PCIe controller
//
// Select internal clock mux
//
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_REFCLK_CMOS_SEL_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
// Select CML as reference clock
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_REFCLK_CMOS_SEL_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
    dev_dbg(ctx.dev,
    "Set internal single ended reference clock\n");
    }
    }
    static void xgene_phy_sata_cfg_cmu_core(struct xgene_phy_ctx *ctx,
    enum cmu_type_t cmu_type,
    enum clk_type_t clk_type)
    {
    u32 val;
    int ref_100MHz;
    if (cmu_type == REF_CMU) {
// Set VCO calibration voltage threshold
    cmu_rd(ctx, cmu_type, CMU_REG34, &val);
    val = CMU_REG34_VCO_CAL_VTH_LO_MAX_SET(val, 0x7);
    val = CMU_REG34_VCO_CAL_VTH_HI_MAX_SET(val, 0xc);
    val = CMU_REG34_VCO_CAL_VTH_LO_MIN_SET(val, 0x3);
    val = CMU_REG34_VCO_CAL_VTH_HI_MIN_SET(val, 0x8);
    cmu_wr(ctx, cmu_type, CMU_REG34, val);
    }
// Set the VCO calibration counter
    cmu_rd(ctx, cmu_type, CMU_REG0, &val);
    if (cmu_type == REF_CMU || preA3Chip)
    val = CMU_REG0_CAL_COUNT_RESOL_SET(val, 0x4);
    else
    val = CMU_REG0_CAL_COUNT_RESOL_SET(val, 0x7);
    cmu_wr(ctx, cmu_type, CMU_REG0, val);
// Configure PLL for calibration
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_PLL_CP_SET(val, 0x1);
    if (cmu_type == REF_CMU || preA3Chip)
    val = CMU_REG1_PLL_CP_SEL_SET(val, 0x5);
    else
    val = CMU_REG1_PLL_CP_SEL_SET(val, 0x3);
    if (cmu_type == REF_CMU)
    val = CMU_REG1_PLL_MANUALCAL_SET(val, 0x0);
    else
    val = CMU_REG1_PLL_MANUALCAL_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
    if (cmu_type != REF_CMU)
    cmu_clrbits(ctx, cmu_type, CMU_REG5, CMU_REG5_PLL_RESETB_MASK);
// Configure the PLL for either 100MHz or 50MHz
    cmu_rd(ctx, cmu_type, CMU_REG2, &val);
    if (cmu_type == REF_CMU) {
    val = CMU_REG2_PLL_LFRES_SET(val, 0xa);
    ref_100MHz = 1;
    } else {
    val = CMU_REG2_PLL_LFRES_SET(val, 0x3);
    if (clk_type == CLK_EXT_DIFF)
    ref_100MHz = 0;
    else
    ref_100MHz = 1;
    }
    if (ref_100MHz) {
    val = CMU_REG2_PLL_FBDIV_SET(val, FBDIV_VAL_100M);
    val = CMU_REG2_PLL_REFDIV_SET(val, REFDIV_VAL_100M);
    } else {
    val = CMU_REG2_PLL_FBDIV_SET(val, FBDIV_VAL_50M);
    val = CMU_REG2_PLL_REFDIV_SET(val, REFDIV_VAL_50M);
    }
    cmu_wr(ctx, cmu_type, CMU_REG2, val);
// Configure the VCO
    cmu_rd(ctx, cmu_type, CMU_REG3, &val);
    if (cmu_type == REF_CMU) {
    val = CMU_REG3_VCOVARSEL_SET(val, 0x3);
    val = CMU_REG3_VCO_MOMSEL_INIT_SET(val, 0x10);
    } else {
    val = CMU_REG3_VCOVARSEL_SET(val, 0xF);
    if (preA3Chip)
    val = CMU_REG3_VCO_MOMSEL_INIT_SET(val, 0x15);
    else
    val = CMU_REG3_VCO_MOMSEL_INIT_SET(val, 0x1a);
    val = CMU_REG3_VCO_MANMOMSEL_SET(val, 0x15);
    }
    cmu_wr(ctx, cmu_type, CMU_REG3, val);
// Disable force PLL lock
    cmu_rd(ctx, cmu_type, CMU_REG26, &val);
    val = CMU_REG26_FORCE_PLL_LOCK_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG26, val);
// Setup PLL loop filter
    cmu_rd(ctx, cmu_type, CMU_REG5, &val);
    val = CMU_REG5_PLL_LFSMCAP_SET(val, 0x3);
    val = CMU_REG5_PLL_LFCAP_SET(val, 0x3);
    if (cmu_type == REF_CMU || !preA3Chip)
    val = CMU_REG5_PLL_LOCK_RESOLUTION_SET(val, 0x7);
    else
    val = CMU_REG5_PLL_LOCK_RESOLUTION_SET(val, 0x4);
    cmu_wr(ctx, cmu_type, CMU_REG5, val);
// Enable or disable manual calibration
    cmu_rd(ctx, cmu_type, CMU_REG6, &val);
    val = CMU_REG6_PLL_VREGTRIM_SET(val, preA3Chip ? 0x0 : 0x2);
    val = CMU_REG6_MAN_PVT_CAL_SET(val, preA3Chip ? 0x1 : 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG6, val);
// Configure lane for 20-bits
    if (cmu_type == PHY_CMU) {
    cmu_rd(ctx, cmu_type, CMU_REG9, &val);
    val = CMU_REG9_TX_WORD_MODE_CH1_SET(val,
    CMU_REG9_WORD_LEN_20BIT);
    val = CMU_REG9_TX_WORD_MODE_CH0_SET(val,
    CMU_REG9_WORD_LEN_20BIT);
    val = CMU_REG9_PLL_POST_DIVBY2_SET(val, 0x1);
    if (!preA3Chip) {
    val = CMU_REG9_VBG_BYPASSB_SET(val, 0x0);
    val = CMU_REG9_IGEN_BYPASS_SET(val , 0x0);
    }
    cmu_wr(ctx, cmu_type, CMU_REG9, val);
    if (!preA3Chip) {
    cmu_rd(ctx, cmu_type, CMU_REG10, &val);
    val = CMU_REG10_VREG_REFSEL_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG10, val);
    }
    }
    cmu_rd(ctx, cmu_type, CMU_REG16, &val);
    val = CMU_REG16_CALIBRATION_DONE_OVERRIDE_SET(val, 0x1);
    val = CMU_REG16_BYPASS_PLL_LOCK_SET(val, 0x1);
    if (cmu_type == REF_CMU || preA3Chip)
    val = CMU_REG16_VCOCAL_WAIT_BTW_CODE_SET(val, 0x4);
    else
    val = CMU_REG16_VCOCAL_WAIT_BTW_CODE_SET(val, 0x7);
    cmu_wr(ctx, cmu_type, CMU_REG16, val);
// Configure for SATA
    cmu_rd(ctx, cmu_type, CMU_REG30, &val);
    val = CMU_REG30_PCIE_MODE_SET(val, 0x0);
    val = CMU_REG30_LOCK_COUNT_SET(val, 0x3);
    cmu_wr(ctx, cmu_type, CMU_REG30, val);
// Disable state machine bypass
    cmu_wr(ctx, cmu_type, CMU_REG31, 0xF);
    cmu_rd(ctx, cmu_type, CMU_REG32, &val);
    val = CMU_REG32_PVT_CAL_WAIT_SEL_SET(val, 0x3);
    if (cmu_type == REF_CMU || preA3Chip)
    val = CMU_REG32_IREF_ADJ_SET(val, 0x3);
    else
    val = CMU_REG32_IREF_ADJ_SET(val, 0x1);
    cmu_wr(ctx, cmu_type, CMU_REG32, val);
// Set VCO calibration threshold
    if (cmu_type != REF_CMU && preA3Chip)
    cmu_wr(ctx, cmu_type, CMU_REG34, 0x8d27);
    else
    cmu_wr(ctx, cmu_type, CMU_REG34, 0x873c);
// Set CTLE Override and override waiting from state machine
    cmu_wr(ctx, cmu_type, CMU_REG37, 0xF00F);
    }
    static void xgene_phy_ssc_enable(struct xgene_phy_ctx *ctx,
    enum cmu_type_t cmu_type)
    {
    u32 val;
// Set SSC modulation value
    cmu_rd(ctx, cmu_type, CMU_REG35, &val);
    val = CMU_REG35_PLL_SSC_MOD_SET(val, 98);
    cmu_wr(ctx, cmu_type, CMU_REG35, val);
// Enable SSC, set vertical step and DSM value
    cmu_rd(ctx, cmu_type, CMU_REG36, &val);
    val = CMU_REG36_PLL_SSC_VSTEP_SET(val, 30);
    val = CMU_REG36_PLL_SSC_EN_SET(val, 1);
    val = CMU_REG36_PLL_SSC_DSMSEL_SET(val, 1);
    cmu_wr(ctx, cmu_type, CMU_REG36, val);
// Reset the PLL
    cmu_clrbits(ctx, cmu_type, CMU_REG5, CMU_REG5_PLL_RESETB_MASK);
    cmu_setbits(ctx, cmu_type, CMU_REG5, CMU_REG5_PLL_RESETB_MASK);
// Force VCO calibration to restart
    cmu_toggle1to0(ctx, cmu_type, CMU_REG32,
    CMU_REG32_FORCE_VCOCAL_START_MASK);
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_sata_cfg_lanes(ctx: *mut xgene_phy_ctx) {
    static void xgene_phy_sata_cfg_lanes(struct xgene_phy_ctx *ctx)
    {
    u32 val;
    u32 reg;
    int i;
    int lane;
    for (lane = 0; lane < MAX_LANE; lane++) {
    serdes_wr(ctx, lane, RXTX_REG147, 0x6);
// Set boost control for quarter, half, and full rate
    serdes_rd(ctx, lane, RXTX_REG0, &val);
    val = RXTX_REG0_CTLE_EQ_HR_SET(val, 0x10);
    val = RXTX_REG0_CTLE_EQ_QR_SET(val, 0x10);
    val = RXTX_REG0_CTLE_EQ_FR_SET(val, 0x10);
    serdes_wr(ctx, lane, RXTX_REG0, val);
// Set boost control value
    serdes_rd(ctx, lane, RXTX_REG1, &val);
    val = RXTX_REG1_RXACVCM_SET(val, 0x7);
    val = RXTX_REG1_CTLE_EQ_SET(val,
    ctx.sata_param.txboostgain[lane * 3 +
    ctx.sata_param.speed[lane]]);
    serdes_wr(ctx, lane, RXTX_REG1, val);
// Latch VTT value based on the termination to ground and
// enable TX FIFO
//
    serdes_rd(ctx, lane, RXTX_REG2, &val);
    val = RXTX_REG2_VTT_ENA_SET(val, 0x1);
    val = RXTX_REG2_VTT_SEL_SET(val, 0x1);
    val = RXTX_REG2_TX_FIFO_ENA_SET(val, 0x1);
    serdes_wr(ctx, lane, RXTX_REG2, val);
// Configure Tx for 20-bits
    serdes_rd(ctx, lane, RXTX_REG4, &val);
    val = RXTX_REG4_TX_WORD_MODE_SET(val, CMU_REG9_WORD_LEN_20BIT);
    serdes_wr(ctx, lane, RXTX_REG4, val);
    if (!preA3Chip) {
    serdes_rd(ctx, lane, RXTX_REG1, &val);
    val = RXTX_REG1_RXVREG1_SET(val, 0x2);
    val = RXTX_REG1_RXIREF_ADJ_SET(val, 0x2);
    serdes_wr(ctx, lane, RXTX_REG1, val);
    }
// Set pre-emphasis first 1 and 2, and post-emphasis values
    serdes_rd(ctx, lane, RXTX_REG5, &val);
    val = RXTX_REG5_TX_CN1_SET(val,
    ctx.sata_param.txprecursor_cn1[lane * 3 +
    ctx.sata_param.speed[lane]]);
    val = RXTX_REG5_TX_CP1_SET(val,
    ctx.sata_param.txpostcursor_cp1[lane * 3 +
    ctx.sata_param.speed[lane]]);
    val = RXTX_REG5_TX_CN2_SET(val,
    ctx.sata_param.txprecursor_cn2[lane * 3 +
    ctx.sata_param.speed[lane]]);
    serdes_wr(ctx, lane, RXTX_REG5, val);
// Set TX amplitude value
    serdes_rd(ctx, lane, RXTX_REG6, &val);
    val = RXTX_REG6_TXAMP_CNTL_SET(val,
    ctx.sata_param.txamplitude[lane * 3 +
    ctx.sata_param.speed[lane]]);
    val = RXTX_REG6_TXAMP_ENA_SET(val, 0x1);
    val = RXTX_REG6_TX_IDLE_SET(val, 0x0);
    val = RXTX_REG6_RX_BIST_RESYNC_SET(val, 0x0);
    val = RXTX_REG6_RX_BIST_ERRCNT_RD_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG6, val);
// Configure Rx for 20-bits
    serdes_rd(ctx, lane, RXTX_REG7, &val);
    val = RXTX_REG7_BIST_ENA_RX_SET(val, 0x0);
    val = RXTX_REG7_RX_WORD_MODE_SET(val, CMU_REG9_WORD_LEN_20BIT);
    serdes_wr(ctx, lane, RXTX_REG7, val);
// Set CDR and LOS values and enable Rx SSC
    serdes_rd(ctx, lane, RXTX_REG8, &val);
    val = RXTX_REG8_CDR_LOOP_ENA_SET(val, 0x1);
    val = RXTX_REG8_CDR_BYPASS_RXLOS_SET(val, 0x0);
    val = RXTX_REG8_SSC_ENABLE_SET(val, 0x1);
    val = RXTX_REG8_SD_DISABLE_SET(val, 0x0);
    val = RXTX_REG8_SD_VREF_SET(val, 0x4);
    serdes_wr(ctx, lane, RXTX_REG8, val);
// Set phase adjust upper/lower limits
    serdes_rd(ctx, lane, RXTX_REG11, &val);
    val = RXTX_REG11_PHASE_ADJUST_LIMIT_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG11, val);
// Enable Latch Off; disable SUMOS and Tx termination
    serdes_rd(ctx, lane, RXTX_REG12, &val);
    val = RXTX_REG12_LATCH_OFF_ENA_SET(val, 0x1);
    val = RXTX_REG12_SUMOS_ENABLE_SET(val, 0x0);
    val = RXTX_REG12_RX_DET_TERM_ENABLE_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG12, val);
// Set period error latch to 512T and enable BWL
    serdes_rd(ctx, lane, RXTX_REG26, &val);
    val = RXTX_REG26_PERIOD_ERROR_LATCH_SET(val, 0x0);
    val = RXTX_REG26_BLWC_ENA_SET(val, 0x1);
    serdes_wr(ctx, lane, RXTX_REG26, val);
    serdes_wr(ctx, lane, RXTX_REG28, 0x0);
// Set DFE loop preset value
    serdes_wr(ctx, lane, RXTX_REG31, 0x0);
// Set Eye Monitor counter width to 12-bit
    serdes_rd(ctx, lane, RXTX_REG61, &val);
    val = RXTX_REG61_ISCAN_INBERT_SET(val, 0x1);
    val = RXTX_REG61_LOADFREQ_SHIFT_SET(val, 0x0);
    val = RXTX_REG61_EYE_COUNT_WIDTH_SEL_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG61, val);
    serdes_rd(ctx, lane, RXTX_REG62, &val);
    val = RXTX_REG62_PERIOD_H1_QLATCH_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG62, val);
// Set BW select tap X for DFE loop
    for (i = 0; i < 9; i++) {
    reg = RXTX_REG81 + i * 2;
    serdes_rd(ctx, lane, reg, &val);
    val = RXTX_REG89_MU_TH7_SET(val, 0xe);
    val = RXTX_REG89_MU_TH8_SET(val, 0xe);
    val = RXTX_REG89_MU_TH9_SET(val, 0xe);
    serdes_wr(ctx, lane, reg, val);
    }
// Set BW select tap X for frequency adjust loop
    for (i = 0; i < 3; i++) {
    reg = RXTX_REG96 + i * 2;
    serdes_rd(ctx, lane, reg, &val);
    val = RXTX_REG96_MU_FREQ1_SET(val, 0x10);
    val = RXTX_REG96_MU_FREQ2_SET(val, 0x10);
    val = RXTX_REG96_MU_FREQ3_SET(val, 0x10);
    serdes_wr(ctx, lane, reg, val);
    }
// Set BW select tap X for phase adjust loop
    for (i = 0; i < 3; i++) {
    reg = RXTX_REG99 + i * 2;
    serdes_rd(ctx, lane, reg, &val);
    val = RXTX_REG99_MU_PHASE1_SET(val, 0x7);
    val = RXTX_REG99_MU_PHASE2_SET(val, 0x7);
    val = RXTX_REG99_MU_PHASE3_SET(val, 0x7);
    serdes_wr(ctx, lane, reg, val);
    }
    serdes_rd(ctx, lane, RXTX_REG102, &val);
    val = RXTX_REG102_FREQLOOP_LIMIT_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG102, val);
    serdes_wr(ctx, lane, RXTX_REG114, 0xffe0);
    serdes_rd(ctx, lane, RXTX_REG125, &val);
    val = RXTX_REG125_SIGN_PQ_SET(val,
    ctx.sata_param.txeyedirection[lane * 3 +
    ctx.sata_param.speed[lane]]);
    val = RXTX_REG125_PQ_REG_SET(val,
    ctx.sata_param.txeyetuning[lane * 3 +
    ctx.sata_param.speed[lane]]);
    val = RXTX_REG125_PHZ_MANUAL_SET(val, 0x1);
    serdes_wr(ctx, lane, RXTX_REG125, val);
    serdes_rd(ctx, lane, RXTX_REG127, &val);
    val = RXTX_REG127_LATCH_MAN_CAL_ENA_SET(val, 0x0);
    serdes_wr(ctx, lane, RXTX_REG127, val);
    serdes_rd(ctx, lane, RXTX_REG128, &val);
    val = RXTX_REG128_LATCH_CAL_WAIT_SEL_SET(val, 0x3);
    serdes_wr(ctx, lane, RXTX_REG128, val);
    serdes_rd(ctx, lane, RXTX_REG145, &val);
    val = RXTX_REG145_RXDFE_CONFIG_SET(val, 0x3);
    val = RXTX_REG145_TX_IDLE_SATA_SET(val, 0x0);
    if (preA3Chip) {
    val = RXTX_REG145_RXES_ENA_SET(val, 0x1);
    val = RXTX_REG145_RXVWES_LATENA_SET(val, 0x1);
    } else {
    val = RXTX_REG145_RXES_ENA_SET(val, 0x0);
    val = RXTX_REG145_RXVWES_LATENA_SET(val, 0x0);
    }
    serdes_wr(ctx, lane, RXTX_REG145, val);
//
// Set Rx LOS filter clock rate, sample rate, and threshold
// windows
//
    for (i = 0; i < 4; i++) {
    reg = RXTX_REG148 + i * 2;
    serdes_wr(ctx, lane, reg, 0xFFFF);
    }
    }
    }
    static int xgene_phy_cal_rdy_chk(struct xgene_phy_ctx *ctx,
    enum cmu_type_t cmu_type,
    enum clk_type_t clk_type)
    {
    void __iomem *csr_serdes = ctx.sds_base;
    int loop;
    u32 val;
// Release PHY main reset
    writel(0xdf, csr_serdes + SATA_ENET_SDS_RST_CTL);
    readl(csr_serdes + SATA_ENET_SDS_RST_CTL); /* Force a barrier */
    if (cmu_type != REF_CMU) {
    cmu_setbits(ctx, cmu_type, CMU_REG5, CMU_REG5_PLL_RESETB_MASK);
//
// As per PHY design spec, the PLL reset requires a minimum
// of 800us.
//
    usleep_range(800, 1000);
    cmu_rd(ctx, cmu_type, CMU_REG1, &val);
    val = CMU_REG1_PLL_MANUALCAL_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG1, val);
//
// As per PHY design spec, the PLL auto calibration requires
// a minimum of 800us.
//
    usleep_range(800, 1000);
    cmu_toggle1to0(ctx, cmu_type, CMU_REG32,
    CMU_REG32_FORCE_VCOCAL_START_MASK);
//
// As per PHY design spec, the PLL requires a minimum of
// 800us to settle.
//
    usleep_range(800, 1000);
    }
    if (!preA3Chip)
    goto skip_manual_cal;
//
// Configure the termination resister calibration
// The serial receive pins, RXP/RXN, have TERMination resistor
// that is required to be calibrated.
//
    cmu_rd(ctx, cmu_type, CMU_REG17, &val);
    val = CMU_REG17_PVT_CODE_R2A_SET(val, 0x12);
    val = CMU_REG17_RESERVED_7_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG17, val);
    cmu_toggle1to0(ctx, cmu_type, CMU_REG17,
    CMU_REG17_PVT_TERM_MAN_ENA_MASK);
//
// The serial transmit pins, TXP/TXN, have Pull-UP and Pull-DOWN
// resistors that are required to the calibrated.
// Configure the pull DOWN calibration
//
    cmu_rd(ctx, cmu_type, CMU_REG17, &val);
    val = CMU_REG17_PVT_CODE_R2A_SET(val, 0x29);
    val = CMU_REG17_RESERVED_7_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG17, val);
    cmu_toggle1to0(ctx, cmu_type, CMU_REG16,
    CMU_REG16_PVT_DN_MAN_ENA_MASK);
// Configure the pull UP calibration
    cmu_rd(ctx, cmu_type, CMU_REG17, &val);
    val = CMU_REG17_PVT_CODE_R2A_SET(val, 0x28);
    val = CMU_REG17_RESERVED_7_SET(val, 0x0);
    cmu_wr(ctx, cmu_type, CMU_REG17, val);
    cmu_toggle1to0(ctx, cmu_type, CMU_REG16,
    CMU_REG16_PVT_UP_MAN_ENA_MASK);
    skip_manual_cal:
// Poll the PLL calibration completion status for at least 1 ms
    loop = 100;
    do {
    cmu_rd(ctx, cmu_type, CMU_REG7, &val);
    if (CMU_REG7_PLL_CALIB_DONE_RD(val))
    break;
//
// As per PHY design spec, PLL calibration status requires
// a minimum of 10us to be updated.
//
    usleep_range(10, 100);
    } while (--loop > 0);
    cmu_rd(ctx, cmu_type, CMU_REG7, &val);
    dev_dbg(ctx.dev, "PLL calibration %s\n",
    CMU_REG7_PLL_CALIB_DONE_RD(val) ? "done" : "failed");
    if (CMU_REG7_VCO_CAL_FAIL_RD(val)) {
    dev_err(ctx.dev,
    "PLL calibration failed due to VCO failure\n");
    return -1;
    }
    dev_dbg(ctx.dev, "PLL calibration successful\n");
    cmu_rd(ctx, cmu_type, CMU_REG15, &val);
    dev_dbg(ctx.dev, "PHY Tx is %sready\n", val & 0x300 ? "" : "not ");
    return 0;
    }
    static void xgene_phy_pdwn_force_vco(struct xgene_phy_ctx *ctx,
    enum cmu_type_t cmu_type,
    enum clk_type_t clk_type)
    {
    u32 val;
    dev_dbg(ctx.dev, "Reset VCO and re-start again\n");
    if (cmu_type == PHY_CMU) {
    cmu_rd(ctx, cmu_type, CMU_REG16, &val);
    val = CMU_REG16_VCOCAL_WAIT_BTW_CODE_SET(val, 0x7);
    cmu_wr(ctx, cmu_type, CMU_REG16, val);
    }
    cmu_toggle1to0(ctx, cmu_type, CMU_REG0, CMU_REG0_PDOWN_MASK);
    cmu_toggle1to0(ctx, cmu_type, CMU_REG32,
    CMU_REG32_FORCE_VCOCAL_START_MASK);
    }
    static int xgene_phy_hw_init_sata(struct xgene_phy_ctx *ctx,
    enum clk_type_t clk_type, int ssc_enable)
    {
    void __iomem *sds_base = ctx.sds_base;
    u32 val;
    int i;
// Configure the PHY for operation
    dev_dbg(ctx.dev, "Reset PHY\n");
// Place PHY into reset
    writel(0x0, sds_base + SATA_ENET_SDS_RST_CTL);
    val = readl(sds_base + SATA_ENET_SDS_RST_CTL);	/* Force a barrier */
// Release PHY lane from reset (active high)
    writel(0x20, sds_base + SATA_ENET_SDS_RST_CTL);
    readl(sds_base + SATA_ENET_SDS_RST_CTL);	/* Force a barrier */
// Release all PHY module out of reset except PHY main reset
    writel(0xde, sds_base + SATA_ENET_SDS_RST_CTL);
    readl(sds_base + SATA_ENET_SDS_RST_CTL);	/* Force a barrier */
// Set the operation speed
    val = readl(sds_base + SATA_ENET_SDS_CTL1);
    val = CFG_I_SPD_SEL_CDR_OVR1_SET(val,
    ctx.sata_param.txspeed[ctx.sata_param.speed[0]]);
    writel(val, sds_base + SATA_ENET_SDS_CTL1);
    dev_dbg(ctx.dev, "Set the customer pin mode to SATA\n");
    val = readl(sds_base + SATA_ENET_SDS_CTL0);
    val = REGSPEC_CFG_I_CUSTOMER_PIN_MODE0_SET(val, 0x4421);
    writel(val, sds_base + SATA_ENET_SDS_CTL0);
// Configure the clock macro unit (CMU) clock type
    xgene_phy_cfg_cmu_clk_type(ctx, PHY_CMU, clk_type);
// Configure the clock macro
    xgene_phy_sata_cfg_cmu_core(ctx, PHY_CMU, clk_type);
// Enable SSC if enabled
    if (ssc_enable)
    xgene_phy_ssc_enable(ctx, PHY_CMU);
// Configure PHY lanes
    xgene_phy_sata_cfg_lanes(ctx);
// Set Rx/Tx 20-bit
    val = readl(sds_base + SATA_ENET_SDS_PCS_CTL0);
    val = REGSPEC_CFG_I_RX_WORDMODE0_SET(val, 0x3);
    val = REGSPEC_CFG_I_TX_WORDMODE0_SET(val, 0x3);
    writel(val, sds_base + SATA_ENET_SDS_PCS_CTL0);
// Start PLL calibration and try for three times
    i = 10;
    do {
    if (!xgene_phy_cal_rdy_chk(ctx, PHY_CMU, clk_type))
    break;
// If failed, toggle the VCO power signal and start again
    xgene_phy_pdwn_force_vco(ctx, PHY_CMU, clk_type);
    } while (--i > 0);
// Even on failure, allow to continue any way
    if (i <= 0)
    dev_err(ctx.dev, "PLL calibration failed\n");
    return 0;
    }
    static int xgene_phy_hw_initialize(struct xgene_phy_ctx *ctx,
    enum clk_type_t clk_type,
    int ssc_enable)
    {
    int rc;
    dev_dbg(ctx.dev, "PHY init clk type %d\n", clk_type);
    if (ctx.mode == MODE_SATA) {
    rc = xgene_phy_hw_init_sata(ctx, clk_type, ssc_enable);
    if (rc)
    return rc;
    } else {
    dev_err(ctx.dev, "Un-supported customer pin mode %d\n",
    ctx.mode);
    return -ENODEV;
    }
    return 0;
    }
//
// Receiver Offset Calibration:
//
// Calibrate the receiver signal path offset in two steps - summar and
// latch calibrations
//
#[no_mangle]
unsafe extern "C" fn xgene_phy_force_lat_summer_cal(ctx: *mut xgene_phy_ctx, lane: c_int) {
    static void xgene_phy_force_lat_summer_cal(struct xgene_phy_ctx *ctx, int lane)
    {
    int i;
    static const struct {
    u32 reg;
    u32 val;
    } serdes_reg[] = {
    {RXTX_REG38, 0x0},
    {RXTX_REG39, 0xff00},
    {RXTX_REG40, 0xffff},
    {RXTX_REG41, 0xffff},
    {RXTX_REG42, 0xffff},
    {RXTX_REG43, 0xffff},
    {RXTX_REG44, 0xffff},
    {RXTX_REG45, 0xffff},
    {RXTX_REG46, 0xffff},
    {RXTX_REG47, 0xfffc},
    {RXTX_REG48, 0x0},
    {RXTX_REG49, 0x0},
    {RXTX_REG50, 0x0},
    {RXTX_REG51, 0x0},
    {RXTX_REG52, 0x0},
    {RXTX_REG53, 0x0},
    {RXTX_REG54, 0x0},
    {RXTX_REG55, 0x0},
    };
// Start SUMMER calibration
    serdes_setbits(ctx, lane, RXTX_REG127,
    RXTX_REG127_FORCE_SUM_CAL_START_MASK);
//
// As per PHY design spec, the Summer calibration requires a minimum
// of 100us to complete.
//
    usleep_range(100, 500);
    serdes_clrbits(ctx, lane, RXTX_REG127,
    RXTX_REG127_FORCE_SUM_CAL_START_MASK);
//
// As per PHY design spec, the auto calibration requires a minimum
// of 100us to complete.
//
    usleep_range(100, 500);
// Start latch calibration
    serdes_setbits(ctx, lane, RXTX_REG127,
    RXTX_REG127_FORCE_LAT_CAL_START_MASK);
//
// As per PHY design spec, the latch calibration requires a minimum
// of 100us to complete.
//
    usleep_range(100, 500);
    serdes_clrbits(ctx, lane, RXTX_REG127,
    RXTX_REG127_FORCE_LAT_CAL_START_MASK);
// Configure the PHY lane for calibration
    serdes_wr(ctx, lane, RXTX_REG28, 0x7);
    serdes_wr(ctx, lane, RXTX_REG31, 0x7e00);
    serdes_clrbits(ctx, lane, RXTX_REG4,
    RXTX_REG4_TX_LOOPBACK_BUF_EN_MASK);
    serdes_clrbits(ctx, lane, RXTX_REG7,
    RXTX_REG7_LOOP_BACK_ENA_CTLE_MASK);
    for (i = 0; i < ARRAY_SIZE(serdes_reg); i++)
    serdes_wr(ctx, lane, serdes_reg[i].reg,
    serdes_reg[i].val);
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_reset_rxd(ctx: *mut xgene_phy_ctx, lane: c_int) {
    static void xgene_phy_reset_rxd(struct xgene_phy_ctx *ctx, int lane)
    {
// Reset digital Rx
    serdes_clrbits(ctx, lane, RXTX_REG7, RXTX_REG7_RESETB_RXD_MASK);
// As per PHY design spec, the reset requires a minimum of 100us.
    usleep_range(100, 150);
    serdes_setbits(ctx, lane, RXTX_REG7, RXTX_REG7_RESETB_RXD_MASK);
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_get_avg(accum: c_int, samples: c_int) -> c_int {
    static int xgene_phy_get_avg(int accum, int samples)
    {
    return (accum + (samples / 2)) / samples;
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_gen_avg_val(ctx: *mut xgene_phy_ctx, lane: c_int) {
    static void xgene_phy_gen_avg_val(struct xgene_phy_ctx *ctx, int lane)
    {
    let mut max_loop: c_int = 10;
    let mut avg_loop: c_int = 0;
    let mut lat_do: c_int = 0, lat_xo = 0, lat_eo = 0, lat_so = 0;
    let mut lat_de: c_int = 0, lat_xe = 0, lat_ee = 0, lat_se = 0;
    let mut sum_cal: c_int = 0;
    int lat_do_itr, lat_xo_itr, lat_eo_itr, lat_so_itr;
    int lat_de_itr, lat_xe_itr, lat_ee_itr, lat_se_itr;
    int sum_cal_itr;
    int fail_even;
    int fail_odd;
    u32 val;
    dev_dbg(ctx.dev, "Generating avg calibration value for lane %d\n",
    lane);
// Enable RX Hi-Z termination
    serdes_setbits(ctx, lane, RXTX_REG12,
    RXTX_REG12_RX_DET_TERM_ENABLE_MASK);
// Turn off DFE
    serdes_wr(ctx, lane, RXTX_REG28, 0x0000);
// DFE Presets to zero
    serdes_wr(ctx, lane, RXTX_REG31, 0x0000);
//
// Receiver Offset Calibration:
// Calibrate the receiver signal path offset in two steps - summar
// and latch calibration.
// Runs the "Receiver Offset Calibration multiple times to determine
// the average value to use.
//
    while (avg_loop < max_loop) {
// Start the calibration
    xgene_phy_force_lat_summer_cal(ctx, lane);
    serdes_rd(ctx, lane, RXTX_REG21, &val);
    lat_do_itr = RXTX_REG21_DO_LATCH_CALOUT_RD(val);
    lat_xo_itr = RXTX_REG21_XO_LATCH_CALOUT_RD(val);
    fail_odd = RXTX_REG21_LATCH_CAL_FAIL_ODD_RD(val);
    serdes_rd(ctx, lane, RXTX_REG22, &val);
    lat_eo_itr = RXTX_REG22_EO_LATCH_CALOUT_RD(val);
    lat_so_itr = RXTX_REG22_SO_LATCH_CALOUT_RD(val);
    fail_even = RXTX_REG22_LATCH_CAL_FAIL_EVEN_RD(val);
    serdes_rd(ctx, lane, RXTX_REG23, &val);
    lat_de_itr = RXTX_REG23_DE_LATCH_CALOUT_RD(val);
    lat_xe_itr = RXTX_REG23_XE_LATCH_CALOUT_RD(val);
    serdes_rd(ctx, lane, RXTX_REG24, &val);
    lat_ee_itr = RXTX_REG24_EE_LATCH_CALOUT_RD(val);
    lat_se_itr = RXTX_REG24_SE_LATCH_CALOUT_RD(val);
    serdes_rd(ctx, lane, RXTX_REG121, &val);
    sum_cal_itr = RXTX_REG121_SUMOS_CAL_CODE_RD(val);
// Check for failure. If passed, sum them for averaging
    if ((fail_even == 0 || fail_even == 1) &&
    (fail_odd == 0 || fail_odd == 1)) {
    lat_do += lat_do_itr;
    lat_xo += lat_xo_itr;
    lat_eo += lat_eo_itr;
    lat_so += lat_so_itr;
    lat_de += lat_de_itr;
    lat_xe += lat_xe_itr;
    lat_ee += lat_ee_itr;
    lat_se += lat_se_itr;
    sum_cal += sum_cal_itr;
    dev_dbg(ctx.dev, "Iteration %d:\n", avg_loop);
    dev_dbg(ctx.dev, "DO 0x%x XO 0x%x EO 0x%x SO 0x%x\n",
    lat_do_itr, lat_xo_itr, lat_eo_itr,
    lat_so_itr);
    dev_dbg(ctx.dev, "DE 0x%x XE 0x%x EE 0x%x SE 0x%x\n",
    lat_de_itr, lat_xe_itr, lat_ee_itr,
    lat_se_itr);
    dev_dbg(ctx.dev, "SUM 0x%x\n", sum_cal_itr);
    ++avg_loop;
    } else {
    dev_err(ctx.dev,
    "Receiver calibration failed at %d loop\n",
    avg_loop);
    }
    xgene_phy_reset_rxd(ctx, lane);
    }
// Update latch manual calibration with average value
    serdes_rd(ctx, lane, RXTX_REG127, &val);
    val = RXTX_REG127_DO_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_do, max_loop));
    val = RXTX_REG127_XO_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_xo, max_loop));
    serdes_wr(ctx, lane, RXTX_REG127, val);
    serdes_rd(ctx, lane, RXTX_REG128, &val);
    val = RXTX_REG128_EO_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_eo, max_loop));
    val = RXTX_REG128_SO_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_so, max_loop));
    serdes_wr(ctx, lane, RXTX_REG128, val);
    serdes_rd(ctx, lane, RXTX_REG129, &val);
    val = RXTX_REG129_DE_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_de, max_loop));
    val = RXTX_REG129_XE_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_xe, max_loop));
    serdes_wr(ctx, lane, RXTX_REG129, val);
    serdes_rd(ctx, lane, RXTX_REG130, &val);
    val = RXTX_REG130_EE_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_ee, max_loop));
    val = RXTX_REG130_SE_LATCH_MANCAL_SET(val,
    xgene_phy_get_avg(lat_se, max_loop));
    serdes_wr(ctx, lane, RXTX_REG130, val);
// Update SUMMER calibration with average value
    serdes_rd(ctx, lane, RXTX_REG14, &val);
    val = RXTX_REG14_CLTE_LATCAL_MAN_PROG_SET(val,
    xgene_phy_get_avg(sum_cal, max_loop));
    serdes_wr(ctx, lane, RXTX_REG14, val);
    dev_dbg(ctx.dev, "Average Value:\n");
    dev_dbg(ctx.dev, "DO 0x%x XO 0x%x EO 0x%x SO 0x%x\n",
    xgene_phy_get_avg(lat_do, max_loop),
    xgene_phy_get_avg(lat_xo, max_loop),
    xgene_phy_get_avg(lat_eo, max_loop),
    xgene_phy_get_avg(lat_so, max_loop));
    dev_dbg(ctx.dev, "DE 0x%x XE 0x%x EE 0x%x SE 0x%x\n",
    xgene_phy_get_avg(lat_de, max_loop),
    xgene_phy_get_avg(lat_xe, max_loop),
    xgene_phy_get_avg(lat_ee, max_loop),
    xgene_phy_get_avg(lat_se, max_loop));
    dev_dbg(ctx.dev, "SUM 0x%x\n",
    xgene_phy_get_avg(sum_cal, max_loop));
    serdes_rd(ctx, lane, RXTX_REG14, &val);
    val = RXTX_REG14_CTLE_LATCAL_MAN_ENA_SET(val, 0x1);
    serdes_wr(ctx, lane, RXTX_REG14, val);
    dev_dbg(ctx.dev, "Enable Manual Summer calibration\n");
    serdes_rd(ctx, lane, RXTX_REG127, &val);
    val = RXTX_REG127_LATCH_MAN_CAL_ENA_SET(val, 0x1);
    dev_dbg(ctx.dev, "Enable Manual Latch calibration\n");
    serdes_wr(ctx, lane, RXTX_REG127, val);
// Disable RX Hi-Z termination
    serdes_rd(ctx, lane, RXTX_REG12, &val);
    val = RXTX_REG12_RX_DET_TERM_ENABLE_SET(val, 0);
    serdes_wr(ctx, lane, RXTX_REG12, val);
// Turn on DFE
    serdes_wr(ctx, lane, RXTX_REG28, 0x0007);
// Set DFE preset
    serdes_wr(ctx, lane, RXTX_REG31, 0x7e00);
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_hw_init(phy: *mut phy) -> c_int {
    static int xgene_phy_hw_init(struct phy *phy)
    {
    struct xgene_phy_ctx *ctx = phy_get_drvdata(phy);
    int rc;
    int i;
    rc = xgene_phy_hw_initialize(ctx, CLK_EXT_DIFF, SSC_DISABLE);
    if (rc) {
    dev_err(ctx.dev, "PHY initialize failed %d\n", rc);
    return rc;
    }
// Setup clock properly after PHY configuration
    if (!IS_ERR(ctx.clk)) {
// HW requires an toggle of the clock
    clk_prepare_enable(ctx.clk);
    clk_disable_unprepare(ctx.clk);
    clk_prepare_enable(ctx.clk);
    }
// Compute average value
    for (i = 0; i < MAX_LANE; i++)
    xgene_phy_gen_avg_val(ctx, i);
    dev_dbg(ctx.dev, "PHY initialized\n");
    return 0;
    }
    static const struct phy_ops xgene_phy_ops = {
    .init		= xgene_phy_hw_init,
    .owner		= THIS_MODULE,
    };
    static struct phy *xgene_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct xgene_phy_ctx *ctx = dev_get_drvdata(dev);
    if (args.args_count <= 0)
    return ERR_PTR(-EINVAL);
    if (args.args[0] >= MODE_MAX)
    return ERR_PTR(-EINVAL);
    ctx.mode = args.args[0];
    return ctx.phy;
    }
    static void xgene_phy_get_param(struct platform_device *pdev,
    const char *name, u32 *buffer,
    int count, u32 *default_val,
    u32 conv_factor)
    {
    int i;
    if (!of_property_read_u32_array(pdev.dev.of_node, name, buffer,
    count)) {
    for (i = 0; i < count; i++)
    buffer[i] /= conv_factor;
    return;
    }
// Does not exist, load default
    for (i = 0; i < count; i++)
    buffer[i] = default_val[i % 3];
    }
#[no_mangle]
unsafe extern "C" fn xgene_phy_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct xgene_phy_ctx *ctx;
    u32 default_spd[] = DEFAULT_SATA_SPD_SEL;
    u32 default_txboost_gain[] = DEFAULT_SATA_TXBOOST_GAIN;
    u32 default_txeye_direction[] = DEFAULT_SATA_TXEYEDIRECTION;
    u32 default_txeye_tuning[] = DEFAULT_SATA_TXEYETUNING;
    u32 default_txamp[] = DEFAULT_SATA_TXAMP;
    u32 default_txcn1[] = DEFAULT_SATA_TXCN1;
    u32 default_txcn2[] = DEFAULT_SATA_TXCN2;
    u32 default_txcp1[] = DEFAULT_SATA_TXCP1;
    int i;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.dev = &pdev.dev;
    ctx.sds_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.sds_base))
    return PTR_ERR(ctx.sds_base);
// Retrieve optional clock
    ctx.clk = clk_get(&pdev.dev, core::ptr::null_mut());
// Load override paramaters
    xgene_phy_get_param(pdev, "apm,tx-eye-tuning",
    ctx.sata_param.txeyetuning, 6, default_txeye_tuning, 1);
    xgene_phy_get_param(pdev, "apm,tx-eye-direction",
    ctx.sata_param.txeyedirection, 6, default_txeye_direction, 1);
    xgene_phy_get_param(pdev, "apm,tx-boost-gain",
    ctx.sata_param.txboostgain, 6, default_txboost_gain, 1);
    xgene_phy_get_param(pdev, "apm,tx-amplitude",
    ctx.sata_param.txamplitude, 6, default_txamp, 13300);
    xgene_phy_get_param(pdev, "apm,tx-pre-cursor1",
    ctx.sata_param.txprecursor_cn1, 6, default_txcn1, 18200);
    xgene_phy_get_param(pdev, "apm,tx-pre-cursor2",
    ctx.sata_param.txprecursor_cn2, 6, default_txcn2, 18200);
    xgene_phy_get_param(pdev, "apm,tx-post-cursor",
    ctx.sata_param.txpostcursor_cp1, 6, default_txcp1, 18200);
    xgene_phy_get_param(pdev, "apm,tx-speed",
    ctx.sata_param.txspeed, 3, default_spd, 1);
    for (i = 0; i < MAX_LANE; i++)
    ctx.sata_param.speed[i] = 2; /* Default to Gen3 */
    platform_set_drvdata(pdev, ctx);
    ctx.phy = devm_phy_create(ctx.dev, core::ptr::null_mut(), &xgene_phy_ops);
    if (IS_ERR(ctx.phy)) {
    dev_dbg(&pdev.dev, "Failed to create PHY\n");
    return PTR_ERR(ctx.phy);
    }
    phy_set_drvdata(ctx.phy, ctx);
    phy_provider = devm_of_phy_provider_register(ctx.dev, xgene_phy_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id xgene_phy_of_match[] = {
    {.compatible = "apm,xgene-phy",},
    {},
    };
    MODULE_DEVICE_TABLE(of, xgene_phy_of_match);
    static struct platform_driver xgene_phy_driver = {
    .probe = xgene_phy_probe,
    .driver = {
    .name = "xgene-phy",
    .of_match_table = xgene_phy_of_match,
    },
    };
    module_platform_driver(xgene_phy_driver);
    MODULE_DESCRIPTION("APM X-Gene Multi-Purpose PHY driver");
    MODULE_AUTHOR("Loc Ho <lho@apm.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION("0.1");
