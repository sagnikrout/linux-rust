//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/alx/reg.h
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


//
// Copyright (c) 2013 Johannes Berg <johannes@sipsolutions.net>
//
// This file is free software: you may copy, redistribute and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 2 of the License, or (at your
// option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
// Copyright (c) 2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
pub const ALX_DEV_ID_AR8161: c_uint = 0x1091;
pub const ALX_DEV_ID_E2200: c_uint = 0xe091;
pub const ALX_DEV_ID_E2400: c_uint = 0xe0a1;
pub const ALX_DEV_ID_E2500: c_uint = 0xe0b1;
pub const ALX_DEV_ID_AR8162: c_uint = 0x1090;
pub const ALX_DEV_ID_AR8171: c_uint = 0x10A1;
pub const ALX_DEV_ID_AR8172: c_uint = 0x10A0;
// rev definition,
// bit(0): with xD support
// bit(1): with Card Reader function
// bit(7:2): real revision
//
pub const ALX_PCI_REVID_SHIFT: c_int = 3;
pub const ALX_REV_A0: c_int = 0;
pub const ALX_REV_A1: c_int = 1;
pub const ALX_REV_B0: c_int = 2;
pub const ALX_REV_C0: c_int = 3;
pub const ALX_DEV_CTRL: c_uint = 0x0060;
pub const ALX_DEV_CTRL_MAXRRS_MIN: c_int = 2;
pub const ALX_MSIX_MASK: c_uint = 0x0090;
pub const ALX_UE_SVRT: c_uint = 0x010C;

// eeprom & flash load register
pub const ALX_EFLD: c_uint = 0x0204;

// eFuse load register
pub const ALX_SLD: c_uint = 0x0218;

pub const ALX_SLD_MAX_TO: c_int = 100;
pub const ALX_PDLL_TRNS1: c_uint = 0x1104;

pub const ALX_PMCTRL: c_uint = 0x12F8;

// bit30: L0s/L1 controlled by MAC based on throughput(setting in 15A0)

pub const ALX_PMCTRL_LCKDET_TIMER_MASK: c_uint = 0xF;
pub const ALX_PMCTRL_LCKDET_TIMER_SHIFT: c_int = 24;
pub const ALX_PMCTRL_LCKDET_TIMER_DEF: c_uint = 0xC;
// bit[23:20] if pm_request_l1 time > @, then enter L0s not L1
pub const ALX_PMCTRL_L1REQ_TO_MASK: c_uint = 0xF;
pub const ALX_PMCTRL_L1REQ_TO_SHIFT: c_int = 20;
pub const ALX_PMCTRL_L1REG_TO_DEF: c_uint = 0xF;

pub const ALX_PMCTRL_L1_TIMER_MASK: c_uint = 0x7;
pub const ALX_PMCTRL_L1_TIMER_SHIFT: c_int = 16;
pub const ALX_PMCTRL_L1_TIMER_16US: c_int = 4;

// bit13: enable pcie clk switch in L1 state

// bit6: power down serdes RX

//
// following registers are mapped only to memory space
//
pub const ALX_MASTER: c_uint = 0x1400;
// bit12: 1:alwys select pclk from serdes, not sw to 25M

// bit11: irq moduration for rx

// bit10: irq moduration for tx/rx

// bit5: wakeup without pcie clk

// bit0: MAC & DMA reset

pub const ALX_DMA_MAC_RST_TO: c_int = 50;
pub const ALX_IRQ_MODU_TIMER: c_uint = 0x1408;
pub const ALX_IRQ_MODU_TIMER1_MASK: c_uint = 0xFFFF;
pub const ALX_IRQ_MODU_TIMER1_SHIFT: c_int = 0;
pub const ALX_PHY_CTRL: c_uint = 0x140C;

// bit14: affect MAC & PHY, go to low power sts

// bit13: 1:pll always ON, 0:can switch in lpw

// bit0: out of dsp RST state

pub const ALX_PHY_CTRL_DSPRST_TO: c_int = 80;

pub const ALX_MAC_STS: c_uint = 0x1410;

pub const ALX_MDIO: c_uint = 0x1414;

pub const ALX_MDIO_CLK_SEL_MASK: c_uint = 0x7;
pub const ALX_MDIO_CLK_SEL_SHIFT: c_int = 24;
pub const ALX_MDIO_CLK_SEL_25MD4: c_int = 0;
pub const ALX_MDIO_CLK_SEL_25MD128: c_int = 7;

// bit21: 1:read,0:write

pub const ALX_MDIO_REG_MASK: c_uint = 0x1F;
pub const ALX_MDIO_REG_SHIFT: c_int = 16;
pub const ALX_MDIO_DATA_MASK: c_uint = 0xFFFF;
pub const ALX_MDIO_DATA_SHIFT: c_int = 0;
pub const ALX_MDIO_MAX_AC_TO: c_int = 120;
pub const ALX_MDIO_EXTN: c_uint = 0x1448;
pub const ALX_MDIO_EXTN_DEVAD_MASK: c_uint = 0x1F;
pub const ALX_MDIO_EXTN_DEVAD_SHIFT: c_int = 16;
pub const ALX_MDIO_EXTN_REG_MASK: c_uint = 0xFFFF;
pub const ALX_MDIO_EXTN_REG_SHIFT: c_int = 0;
pub const ALX_SERDES: c_uint = 0x1424;

pub const ALX_LPI_CTRL: c_uint = 0x1440;

// for B0+, bit[13..] for C0+
pub const ALX_HRTBT_EXT_CTRL: c_uint = 0x1AD0;
pub const L1F_HRTBT_EXT_CTRL_PERIOD_HIGH_MASK: c_uint = 0x3F;
pub const L1F_HRTBT_EXT_CTRL_PERIOD_HIGH_SHIFT: c_int = 24;

pub const ALX_HRTBT_EXT_CTRL_FRAG_LEN_MASK: c_uint = 0xFF;
pub const ALX_HRTBT_EXT_CTRL_FRAG_LEN_SHIFT: c_int = 4;

pub const ALX_HRTBT_REM_IPV4_ADDR: c_uint = 0x1AD4;
pub const ALX_HRTBT_HOST_IPV4_ADDR: c_uint = 0x1478;
pub const ALX_HRTBT_REM_IPV6_ADDR3: c_uint = 0x1AD8;
pub const ALX_HRTBT_REM_IPV6_ADDR2: c_uint = 0x1ADC;
pub const ALX_HRTBT_REM_IPV6_ADDR1: c_uint = 0x1AE0;
pub const ALX_HRTBT_REM_IPV6_ADDR0: c_uint = 0x1AE4;
// 1B8C ~ 1B94 for C0+
pub const ALX_SWOI_ACER_CTRL: c_uint = 0x1B8C;

pub const ALX_SWOI_ORIG_ACK_NAK_PKT_LEN_SHIFT: c_int = 12;

pub const ALX_SWOI_ORIG_ACK_ADDR_SHIFT: c_int = 0;
pub const ALX_SWOI_IOAC_CTRL_2: c_uint = 0x1B90;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_FRAG_LEN_MASK: c_uint = 0xFF;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_FRAG_LEN_SHIFT: c_int = 24;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_PKT_LEN_MASK: c_uint = 0xFFF;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_PKT_LEN_SHIFT: c_int = 12;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_HDR_ADDR_MASK: c_uint = 0xFFF;
pub const ALX_SWOI_IOAC_CTRL_2_SWOI_1_HDR_ADDR_SHIFT: c_int = 0;
pub const ALX_SWOI_IOAC_CTRL_3: c_uint = 0x1B94;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_FRAG_LEN_MASK: c_uint = 0xFF;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_FRAG_LEN_SHIFT: c_int = 24;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_PKT_LEN_MASK: c_uint = 0xFFF;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_PKT_LEN_SHIFT: c_int = 12;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_HDR_ADDR_MASK: c_uint = 0xFFF;
pub const ALX_SWOI_IOAC_CTRL_3_SWOI_2_HDR_ADDR_SHIFT: c_int = 0;
// for B0
pub const ALX_IDLE_DECISN_TIMER: c_uint = 0x1474;
// 1ms
pub const ALX_IDLE_DECISN_TIMER_DEF: c_uint = 0x400;
pub const ALX_MAC_CTRL: c_uint = 0x1480;

// bit29: 1:legacy(hi5b), 0:marvl(lo5b)

pub const ALX_MAC_CTRL_SPEED_MASK: c_uint = 0x3;
pub const ALX_MAC_CTRL_SPEED_SHIFT: c_int = 20;
pub const ALX_MAC_CTRL_SPEED_10_100: c_int = 1;
pub const ALX_MAC_CTRL_SPEED_1000: c_int = 2;

pub const ALX_MAC_CTRL_PRMBLEN_MASK: c_uint = 0xF;
pub const ALX_MAC_CTRL_PRMBLEN_SHIFT: c_int = 10;

pub const ALX_STAD0: c_uint = 0x1488;
pub const ALX_STAD1: c_uint = 0x148C;
pub const ALX_HASH_TBL0: c_uint = 0x1490;
pub const ALX_HASH_TBL1: c_uint = 0x1494;
pub const ALX_MTU: c_uint = 0x149C;
pub const ALX_MTU_JUMBO_TH: c_int = 1514;
pub const ALX_MTU_STD_ALGN: c_int = 1536;
pub const ALX_SRAM5: c_uint = 0x1524;
pub const ALX_SRAM_RXF_LEN_MASK: c_uint = 0xFFF;
pub const ALX_SRAM_RXF_LEN_SHIFT: c_int = 0;

pub const ALX_SRAM9: c_uint = 0x1534;

pub const ALX_RX_BASE_ADDR_HI: c_uint = 0x1540;
pub const ALX_TX_BASE_ADDR_HI: c_uint = 0x1544;
pub const ALX_RFD_ADDR_LO: c_uint = 0x1550;
pub const ALX_RFD_RING_SZ: c_uint = 0x1560;
pub const ALX_RFD_BUF_SZ: c_uint = 0x1564;
pub const ALX_RRD_ADDR_LO: c_uint = 0x1568;
pub const ALX_RRD_RING_SZ: c_uint = 0x1578;
// pri3: highest, pri0: lowest
pub const ALX_TPD_PRI3_ADDR_LO: c_uint = 0x14E4;
pub const ALX_TPD_PRI2_ADDR_LO: c_uint = 0x14E0;
pub const ALX_TPD_PRI1_ADDR_LO: c_uint = 0x157C;
pub const ALX_TPD_PRI0_ADDR_LO: c_uint = 0x1580;
// producer index is 16bit
pub const ALX_TPD_PRI3_PIDX: c_uint = 0x1618;
pub const ALX_TPD_PRI2_PIDX: c_uint = 0x161A;
pub const ALX_TPD_PRI1_PIDX: c_uint = 0x15F0;
pub const ALX_TPD_PRI0_PIDX: c_uint = 0x15F2;
// consumer index is 16bit
pub const ALX_TPD_PRI3_CIDX: c_uint = 0x161C;
pub const ALX_TPD_PRI2_CIDX: c_uint = 0x161E;
pub const ALX_TPD_PRI1_CIDX: c_uint = 0x15F4;
pub const ALX_TPD_PRI0_CIDX: c_uint = 0x15F6;
pub const ALX_TPD_RING_SZ: c_uint = 0x1584;
pub const ALX_TXQ0: c_uint = 0x1590;
pub const ALX_TXQ0_TXF_BURST_PREF_MASK: c_uint = 0xFFFF;
pub const ALX_TXQ0_TXF_BURST_PREF_SHIFT: c_int = 16;
pub const ALX_TXQ_TXF_BURST_PREF_DEF: c_uint = 0x200;

pub const ALX_TXQ0_TPD_BURSTPREF_MASK: c_uint = 0xF;
pub const ALX_TXQ0_TPD_BURSTPREF_SHIFT: c_int = 0;
pub const ALX_TXQ_TPD_BURSTPREF_DEF: c_int = 5;
pub const ALX_TXQ1: c_uint = 0x1594;
// bit11:  drop large packet, len > (rfd buf)

pub const ALX_RXQ0: c_uint = 0x15A0;

pub const ALX_RXQ0_RSS_MODE_MASK: c_uint = 0x3;
pub const ALX_RXQ0_RSS_MODE_SHIFT: c_int = 26;
pub const ALX_RXQ0_RSS_MODE_DIS: c_int = 0;
pub const ALX_RXQ0_RSS_MODE_MQMI: c_int = 3;
pub const ALX_RXQ0_NUM_RFD_PREF_MASK: c_uint = 0x3F;
pub const ALX_RXQ0_NUM_RFD_PREF_SHIFT: c_int = 20;
pub const ALX_RXQ0_NUM_RFD_PREF_DEF: c_int = 8;
pub const ALX_RXQ0_IDT_TBL_SIZE_MASK: c_uint = 0x1FF;
pub const ALX_RXQ0_IDT_TBL_SIZE_SHIFT: c_int = 8;
pub const ALX_RXQ0_IDT_TBL_SIZE_DEF: c_uint = 0x100;
pub const ALX_RXQ0_IDT_TBL_SIZE_NORMAL: c_int = 128;

pub const ALX_RXQ0_RSS_HSTYP_MASK: c_uint = 0xF;
pub const ALX_RXQ0_RSS_HSTYP_SHIFT: c_int = 2;

pub const ALX_RXQ0_ASPM_THRESH_MASK: c_uint = 0x3;
pub const ALX_RXQ0_ASPM_THRESH_SHIFT: c_int = 0;
pub const ALX_RXQ0_ASPM_THRESH_100M: c_int = 3;
pub const ALX_RXQ2: c_uint = 0x15A8;
pub const ALX_RXQ2_RXF_XOFF_THRESH_MASK: c_uint = 0xFFF;
pub const ALX_RXQ2_RXF_XOFF_THRESH_SHIFT: c_int = 16;
pub const ALX_RXQ2_RXF_XON_THRESH_MASK: c_uint = 0xFFF;
pub const ALX_RXQ2_RXF_XON_THRESH_SHIFT: c_int = 0;
// Size = tx-packet(1522) + IPG(12) + SOF(8) + 64(Pause) + IPG(12) + SOF(8) +
// rx-packet(1522) + delay-of-link(64)
// = 3212.
//
pub const ALX_RXQ2_RXF_FLOW_CTRL_RSVD: c_int = 3212;
pub const ALX_DMA: c_uint = 0x15C0;
pub const ALX_DMA_RCHNL_SEL_MASK: c_uint = 0x3;
pub const ALX_DMA_RCHNL_SEL_SHIFT: c_int = 26;
pub const ALX_DMA_WDLY_CNT_MASK: c_uint = 0xF;
pub const ALX_DMA_WDLY_CNT_SHIFT: c_int = 16;
pub const ALX_DMA_WDLY_CNT_DEF: c_int = 4;
pub const ALX_DMA_RDLY_CNT_MASK: c_uint = 0x1F;
pub const ALX_DMA_RDLY_CNT_SHIFT: c_int = 11;
pub const ALX_DMA_RDLY_CNT_DEF: c_int = 15;
// bit10: 0:tpd with pri, 1: data

pub const ALX_DMA_RREQ_BLEN_MASK: c_uint = 0x7;
pub const ALX_DMA_RREQ_BLEN_SHIFT: c_int = 4;
pub const ALX_DMA_RORDER_MODE_MASK: c_uint = 0x7;
pub const ALX_DMA_RORDER_MODE_SHIFT: c_int = 0;
pub const ALX_DMA_RORDER_MODE_OUT: c_int = 4;
pub const ALX_WOL0: c_uint = 0x14A0;

pub const ALX_RFD_PIDX: c_uint = 0x15E0;
pub const ALX_RFD_CIDX: c_uint = 0x15F8;
// MIB
pub const ALX_MIB_BASE: c_uint = 0x1700;

pub const ALX_ISR: c_uint = 0x1600;

pub const ALX_IMR: c_uint = 0x1604;
// re-send assert msg if SW no response
pub const ALX_INT_RETRIG: c_uint = 0x1608;
// 40ms
pub const ALX_INT_RETRIG_TO: c_int = 20000;
pub const ALX_SMB_TIMER: c_uint = 0x15C4;
pub const ALX_TINT_TPD_THRSHLD: c_uint = 0x15C8;
pub const ALX_TINT_TIMER: c_uint = 0x15CC;
pub const ALX_CLK_GATE: c_uint = 0x1814;

// interop between drivers
pub const ALX_DRV: c_uint = 0x1804;

// bit23: adv Pause

// bit22: adv Asym Pause
pub const ALX_DRV_PHY_MASK: c_uint = 0xFF;
pub const ALX_DRV_PHY_SHIFT: c_int = 21;
pub const ALX_DRV_PHY_UNKNOWN: c_int = 0;
// flag of phy inited
pub const ALX_PHY_INITED: c_uint = 0x003F;
// reg 1830 ~ 186C for C0+, 16 bit map patterns and wake packet detection
pub const ALX_WOL_CTRL2: c_uint = 0x1830;

pub const ALX_WOL_CTRL3: c_uint = 0x1834;
pub const ALX_WOL_CTRL3_PTRN_ADDR_MASK: c_uint = 0xFFFFF;
pub const ALX_WOL_CTRL3_PTRN_ADDR_SHIFT: c_int = 0;
pub const ALX_WOL_CTRL4: c_uint = 0x1838;

pub const ALX_WOL_CTRL5: c_uint = 0x183C;
pub const ALX_WOL_CTRL5_PT3_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT3_LEN_SHIFT: c_int = 24;
pub const ALX_WOL_CTRL5_PT2_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT2_LEN_SHIFT: c_int = 16;
pub const ALX_WOL_CTRL5_PT1_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT1_LEN_SHIFT: c_int = 8;
pub const ALX_WOL_CTRL5_PT0_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT0_LEN_SHIFT: c_int = 0;
pub const ALX_WOL_CTRL6: c_uint = 0x1840;
pub const ALX_WOL_CTRL5_PT7_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT7_LEN_SHIFT: c_int = 24;
pub const ALX_WOL_CTRL5_PT6_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT6_LEN_SHIFT: c_int = 16;
pub const ALX_WOL_CTRL5_PT5_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT5_LEN_SHIFT: c_int = 8;
pub const ALX_WOL_CTRL5_PT4_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT4_LEN_SHIFT: c_int = 0;
pub const ALX_WOL_CTRL7: c_uint = 0x1844;
pub const ALX_WOL_CTRL5_PT11_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT11_LEN_SHIFT: c_int = 24;
pub const ALX_WOL_CTRL5_PT10_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT10_LEN_SHIFT: c_int = 16;
pub const ALX_WOL_CTRL5_PT9_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT9_LEN_SHIFT: c_int = 8;
pub const ALX_WOL_CTRL5_PT8_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT8_LEN_SHIFT: c_int = 0;
pub const ALX_WOL_CTRL8: c_uint = 0x1848;
pub const ALX_WOL_CTRL5_PT15_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT15_LEN_SHIFT: c_int = 24;
pub const ALX_WOL_CTRL5_PT14_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT14_LEN_SHIFT: c_int = 16;
pub const ALX_WOL_CTRL5_PT13_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT13_LEN_SHIFT: c_int = 8;
pub const ALX_WOL_CTRL5_PT12_LEN_MASK: c_uint = 0xFF;
pub const ALX_WOL_CTRL5_PT12_LEN_SHIFT: c_int = 0;
pub const ALX_ACER_FIXED_PTN0: c_uint = 0x1850;
pub const ALX_ACER_FIXED_PTN0_MASK: c_uint = 0xFFFFFFFF;
pub const ALX_ACER_FIXED_PTN0_SHIFT: c_int = 0;
pub const ALX_ACER_FIXED_PTN1: c_uint = 0x1854;
pub const ALX_ACER_FIXED_PTN1_MASK: c_uint = 0xFFFF;
pub const ALX_ACER_FIXED_PTN1_SHIFT: c_int = 0;
pub const ALX_ACER_RANDOM_NUM0: c_uint = 0x1858;
pub const ALX_ACER_RANDOM_NUM0_MASK: c_uint = 0xFFFFFFFF;
pub const ALX_ACER_RANDOM_NUM0_SHIFT: c_int = 0;
pub const ALX_ACER_RANDOM_NUM1: c_uint = 0x185C;
pub const ALX_ACER_RANDOM_NUM1_MASK: c_uint = 0xFFFFFFFF;
pub const ALX_ACER_RANDOM_NUM1_SHIFT: c_int = 0;
pub const ALX_ACER_RANDOM_NUM2: c_uint = 0x1860;
pub const ALX_ACER_RANDOM_NUM2_MASK: c_uint = 0xFFFFFFFF;
pub const ALX_ACER_RANDOM_NUM2_SHIFT: c_int = 0;
pub const ALX_ACER_RANDOM_NUM3: c_uint = 0x1864;
pub const ALX_ACER_RANDOM_NUM3_MASK: c_uint = 0xFFFFFFFF;
pub const ALX_ACER_RANDOM_NUM3_SHIFT: c_int = 0;
pub const ALX_ACER_MAGIC: c_uint = 0x1868;

pub const ALX_ACER_MAGIC_RAN_LEN_MASK: c_uint = 0x1F;
pub const ALX_ACER_MAGIC_RAN_LEN_SHIFT: c_int = 5;
pub const ALX_ACER_MAGIC_FIX_LEN_MASK: c_uint = 0x1F;
pub const ALX_ACER_MAGIC_FIX_LEN_SHIFT: c_int = 0;
pub const ALX_ACER_TIMER: c_uint = 0x186C;

pub const ALX_ACER_TIMER_THRES_MASK: c_uint = 0x1FFFF;
pub const ALX_ACER_TIMER_THRES_SHIFT: c_int = 0;
pub const ALX_ACER_TIMER_THRES_DEF: c_int = 1;
// RSS definitions
pub const ALX_RSS_KEY0: c_uint = 0x14B0;
pub const ALX_RSS_KEY1: c_uint = 0x14B4;
pub const ALX_RSS_KEY2: c_uint = 0x14B8;
pub const ALX_RSS_KEY3: c_uint = 0x14BC;
pub const ALX_RSS_KEY4: c_uint = 0x14C0;
pub const ALX_RSS_KEY5: c_uint = 0x14C4;
pub const ALX_RSS_KEY6: c_uint = 0x14C8;
pub const ALX_RSS_KEY7: c_uint = 0x14CC;
pub const ALX_RSS_KEY8: c_uint = 0x14D0;
pub const ALX_RSS_KEY9: c_uint = 0x14D4;
pub const ALX_RSS_IDT_TBL0: c_uint = 0x1B00;
pub const ALX_MSI_MAP_TBL1: c_uint = 0x15D0;
pub const ALX_MSI_MAP_TBL1_TXQ1_SHIFT: c_int = 20;
pub const ALX_MSI_MAP_TBL1_TXQ0_SHIFT: c_int = 16;
pub const ALX_MSI_MAP_TBL1_RXQ3_SHIFT: c_int = 12;
pub const ALX_MSI_MAP_TBL1_RXQ2_SHIFT: c_int = 8;
pub const ALX_MSI_MAP_TBL1_RXQ1_SHIFT: c_int = 4;
pub const ALX_MSI_MAP_TBL1_RXQ0_SHIFT: c_int = 0;
pub const ALX_MSI_MAP_TBL2: c_uint = 0x15D8;
pub const ALX_MSI_MAP_TBL2_TXQ3_SHIFT: c_int = 20;
pub const ALX_MSI_MAP_TBL2_TXQ2_SHIFT: c_int = 16;
pub const ALX_MSI_MAP_TBL2_RXQ7_SHIFT: c_int = 12;
pub const ALX_MSI_MAP_TBL2_RXQ6_SHIFT: c_int = 8;
pub const ALX_MSI_MAP_TBL2_RXQ5_SHIFT: c_int = 4;
pub const ALX_MSI_MAP_TBL2_RXQ4_SHIFT: c_int = 0;
pub const ALX_MSI_ID_MAP: c_uint = 0x15D4;
pub const ALX_MSI_RETRANS_TIMER: c_uint = 0x1920;
// bit16: 1:line,0:standard

pub const ALX_MSI_RETRANS_TM_MASK: c_uint = 0xFFFF;
pub const ALX_MSI_RETRANS_TM_SHIFT: c_int = 0;
// CR DMA ctrl
// TX QoS
pub const ALX_WRR: c_uint = 0x1938;
pub const ALX_WRR_PRI_MASK: c_uint = 0x3;
pub const ALX_WRR_PRI_SHIFT: c_int = 29;
pub const ALX_WRR_PRI_RESTRICT_NONE: c_int = 3;
pub const ALX_WRR_PRI3_MASK: c_uint = 0x1F;
pub const ALX_WRR_PRI3_SHIFT: c_int = 24;
pub const ALX_WRR_PRI2_MASK: c_uint = 0x1F;
pub const ALX_WRR_PRI2_SHIFT: c_int = 16;
pub const ALX_WRR_PRI1_MASK: c_uint = 0x1F;
pub const ALX_WRR_PRI1_SHIFT: c_int = 8;
pub const ALX_WRR_PRI0_MASK: c_uint = 0x1F;
pub const ALX_WRR_PRI0_SHIFT: c_int = 0;
pub const ALX_HQTPD: c_uint = 0x193C;

pub const ALX_HQTPD_Q3_NUMPREF_MASK: c_uint = 0xF;
pub const ALX_HQTPD_Q3_NUMPREF_SHIFT: c_int = 8;
pub const ALX_HQTPD_Q2_NUMPREF_MASK: c_uint = 0xF;
pub const ALX_HQTPD_Q2_NUMPREF_SHIFT: c_int = 4;
pub const ALX_HQTPD_Q1_NUMPREF_MASK: c_uint = 0xF;
pub const ALX_HQTPD_Q1_NUMPREF_SHIFT: c_int = 0;
pub const ALX_MISC: c_uint = 0x19C0;
pub const ALX_MISC_PSW_OCP_MASK: c_uint = 0x7;
pub const ALX_MISC_PSW_OCP_SHIFT: c_int = 21;
pub const ALX_MISC_PSW_OCP_DEF: c_uint = 0x7;

pub const ALX_MSIC2: c_uint = 0x19C8;

pub const ALX_MISC3: c_uint = 0x19CC;
// bit1: 1:Software control 25M

// bit0: 25M switch to intnl OSC

// MSIX tbl in memory space
pub const ALX_MSIX_ENTRY_BASE: c_uint = 0x2000;
// PHY regs definition
// PHY Specific Status Register
pub const ALX_MII_GIGA_PSSR: c_uint = 0x11;
pub const ALX_GIGA_PSSR_SPD_DPLX_RESOLVED: c_uint = 0x0800;
pub const ALX_GIGA_PSSR_DPLX: c_uint = 0x2000;
pub const ALX_GIGA_PSSR_SPEED: c_uint = 0xC000;
pub const ALX_GIGA_PSSR_10MBS: c_uint = 0x0000;
pub const ALX_GIGA_PSSR_100MBS: c_uint = 0x4000;
pub const ALX_GIGA_PSSR_1000MBS: c_uint = 0x8000;
// PHY Interrupt Enable Register
pub const ALX_MII_IER: c_uint = 0x12;
pub const ALX_IER_LINK_UP: c_uint = 0x0400;
pub const ALX_IER_LINK_DOWN: c_uint = 0x0800;
// PHY Interrupt Status Register
pub const ALX_MII_ISR: c_uint = 0x13;
pub const ALX_MII_DBG_ADDR: c_uint = 0x1D;
pub const ALX_MII_DBG_DATA: c_uint = 0x1E;
// debug port
pub const ALX_MIIDBG_ANACTRL: c_uint = 0x00;
pub const ALX_ANACTRL_DEF: c_uint = 0x02EF;
pub const ALX_MIIDBG_SYSMODCTRL: c_uint = 0x04;
// en half bias
pub const ALX_SYSMODCTRL_IECHOADJ_DEF: c_uint = 0xBB8B;
pub const ALX_MIIDBG_SRDSYSMOD: c_uint = 0x05;
pub const ALX_SRDSYSMOD_DEEMP_EN: c_uint = 0x0040;
pub const ALX_SRDSYSMOD_DEF: c_uint = 0x2C46;
pub const ALX_MIIDBG_HIBNEG: c_uint = 0x0B;
pub const ALX_HIBNEG_PSHIB_EN: c_uint = 0x8000;
pub const ALX_HIBNEG_HIB_PSE: c_uint = 0x1000;
pub const ALX_HIBNEG_DEF: c_uint = 0xBC40;

pub const ALX_MIIDBG_TST10BTCFG: c_uint = 0x12;
pub const ALX_TST10BTCFG_DEF: c_uint = 0x4C04;
pub const ALX_MIIDBG_AZ_ANADECT: c_uint = 0x15;
pub const ALX_AZ_ANADECT_DEF: c_uint = 0x3220;
pub const ALX_AZ_ANADECT_LONG: c_uint = 0x3210;
pub const ALX_MIIDBG_MSE16DB: c_uint = 0x18;
pub const ALX_MSE16DB_UP: c_uint = 0x05EA;
pub const ALX_MSE16DB_DOWN: c_uint = 0x02EA;
pub const ALX_MIIDBG_MSE20DB: c_uint = 0x1C;
pub const ALX_MSE20DB_TH_MASK: c_uint = 0x7F;
pub const ALX_MSE20DB_TH_SHIFT: c_int = 2;
pub const ALX_MSE20DB_TH_DEF: c_uint = 0x2E;
pub const ALX_MSE20DB_TH_HI: c_uint = 0x54;
pub const ALX_MIIDBG_AGC: c_uint = 0x23;
pub const ALX_AGC_2_VGA_MASK: c_uint = 0x3FU;
pub const ALX_AGC_2_VGA_SHIFT: c_int = 8;
pub const ALX_AGC_LONG1G_LIMT: c_int = 40;
pub const ALX_AGC_LONG100M_LIMT: c_int = 44;
pub const ALX_MIIDBG_LEGCYPS: c_uint = 0x29;
pub const ALX_LEGCYPS_EN: c_uint = 0x8000;
pub const ALX_LEGCYPS_DEF: c_uint = 0x129D;
pub const ALX_MIIDBG_TST100BTCFG: c_uint = 0x36;
pub const ALX_TST100BTCFG_DEF: c_uint = 0xE12C;
pub const ALX_MIIDBG_GREENCFG: c_uint = 0x3B;
pub const ALX_GREENCFG_DEF: c_uint = 0x7078;
pub const ALX_MIIDBG_GREENCFG2: c_uint = 0x3D;
pub const ALX_GREENCFG2_BP_GREEN: c_uint = 0x8000;
pub const ALX_GREENCFG2_GATE_DFSE_EN: c_uint = 0x0080;
// dev 3
pub const ALX_MIIEXT_PCS: c_int = 3;
pub const ALX_MIIEXT_CLDCTRL3: c_uint = 0x8003;
pub const ALX_CLDCTRL3_BP_CABLE1TH_DET_GT: c_uint = 0x8000;
pub const ALX_MIIEXT_CLDCTRL5: c_uint = 0x8005;
pub const ALX_CLDCTRL5_BP_VD_HLFBIAS: c_uint = 0x4000;
pub const ALX_MIIEXT_CLDCTRL6: c_uint = 0x8006;
pub const ALX_CLDCTRL6_CAB_LEN_MASK: c_uint = 0xFF;
pub const ALX_CLDCTRL6_CAB_LEN_SHIFT: c_int = 0;
pub const ALX_CLDCTRL6_CAB_LEN_SHORT1G: c_int = 116;
pub const ALX_CLDCTRL6_CAB_LEN_SHORT100M: c_int = 152;
pub const ALX_MIIEXT_VDRVBIAS: c_uint = 0x8062;
pub const ALX_VDRVBIAS_DEF: c_uint = 0x3;
// dev 7
pub const ALX_MIIEXT_ANEG: c_int = 7;
pub const ALX_MIIEXT_LOCAL_EEEADV: c_uint = 0x3C;
pub const ALX_LOCAL_EEEADV_1000BT: c_uint = 0x0004;
pub const ALX_LOCAL_EEEADV_100BT: c_uint = 0x0002;
pub const ALX_MIIEXT_AFE: c_uint = 0x801A;
pub const ALX_AFE_10BT_100M_TH: c_uint = 0x0040;
pub const ALX_MIIEXT_S3DIG10: c_uint = 0x8023;
// bit0: 1:bypass 10BT rx fifo, 0:original 10BT rx
pub const ALX_MIIEXT_S3DIG10_SL: c_uint = 0x0001;
pub const ALX_MIIEXT_S3DIG10_DEF: c_int = 0;
pub const ALX_MIIEXT_NLP78: c_uint = 0x8027;
pub const ALX_MIIEXT_NLP78_120M_DEF: c_uint = 0x8A05;
