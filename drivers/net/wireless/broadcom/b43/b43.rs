//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/b43.h
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

// MMIO offsets
pub const B43_MMIO_DMA0_REASON: c_uint = 0x20;
pub const B43_MMIO_DMA0_IRQ_MASK: c_uint = 0x24;
pub const B43_MMIO_DMA1_REASON: c_uint = 0x28;
pub const B43_MMIO_DMA1_IRQ_MASK: c_uint = 0x2C;
pub const B43_MMIO_DMA2_REASON: c_uint = 0x30;
pub const B43_MMIO_DMA2_IRQ_MASK: c_uint = 0x34;
pub const B43_MMIO_DMA3_REASON: c_uint = 0x38;
pub const B43_MMIO_DMA3_IRQ_MASK: c_uint = 0x3C;
pub const B43_MMIO_DMA4_REASON: c_uint = 0x40;
pub const B43_MMIO_DMA4_IRQ_MASK: c_uint = 0x44;
pub const B43_MMIO_DMA5_REASON: c_uint = 0x48;
pub const B43_MMIO_DMA5_IRQ_MASK: c_uint = 0x4C;
pub const B43_MMIO_MACCTL: c_uint = 0x120	/* MAC control */;
pub const B43_MMIO_MACCMD: c_uint = 0x124	/* MAC command */;
pub const B43_MMIO_GEN_IRQ_REASON: c_uint = 0x128;
pub const B43_MMIO_GEN_IRQ_MASK: c_uint = 0x12C;
pub const B43_MMIO_RAM_CONTROL: c_uint = 0x130;
pub const B43_MMIO_RAM_DATA: c_uint = 0x134;
pub const B43_MMIO_PS_STATUS: c_uint = 0x140;
pub const B43_MMIO_RADIO_HWENABLED_HI: c_uint = 0x158;
pub const B43_MMIO_MAC_HW_CAP: c_uint = 0x15C	/* MAC capabilities (corerev >= 13) */;
pub const B43_MMIO_SHM_CONTROL: c_uint = 0x160;
pub const B43_MMIO_SHM_DATA: c_uint = 0x164;
pub const B43_MMIO_SHM_DATA_UNALIGNED: c_uint = 0x166;
pub const B43_MMIO_XMITSTAT_0: c_uint = 0x170;
pub const B43_MMIO_XMITSTAT_1: c_uint = 0x174;
pub const B43_MMIO_REV3PLUS_TSF_LOW: c_uint = 0x180	/* core rev >= 3 only */;
pub const B43_MMIO_REV3PLUS_TSF_HIGH: c_uint = 0x184	/* core rev >= 3 only */;
pub const B43_MMIO_TSF_CFP_REP: c_uint = 0x188;
pub const B43_MMIO_TSF_CFP_START: c_uint = 0x18C;
pub const B43_MMIO_TSF_CFP_MAXDUR: c_uint = 0x190;
// 32-bit DMA
pub const B43_MMIO_DMA32_BASE0: c_uint = 0x200;
pub const B43_MMIO_DMA32_BASE1: c_uint = 0x220;
pub const B43_MMIO_DMA32_BASE2: c_uint = 0x240;
pub const B43_MMIO_DMA32_BASE3: c_uint = 0x260;
pub const B43_MMIO_DMA32_BASE4: c_uint = 0x280;
pub const B43_MMIO_DMA32_BASE5: c_uint = 0x2A0;
// 64-bit DMA
pub const B43_MMIO_DMA64_BASE0: c_uint = 0x200;
pub const B43_MMIO_DMA64_BASE1: c_uint = 0x240;
pub const B43_MMIO_DMA64_BASE2: c_uint = 0x280;
pub const B43_MMIO_DMA64_BASE3: c_uint = 0x2C0;
pub const B43_MMIO_DMA64_BASE4: c_uint = 0x300;
pub const B43_MMIO_DMA64_BASE5: c_uint = 0x340;
// PIO on core rev < 11
pub const B43_MMIO_PIO_BASE0: c_uint = 0x300;
pub const B43_MMIO_PIO_BASE1: c_uint = 0x310;
pub const B43_MMIO_PIO_BASE2: c_uint = 0x320;
pub const B43_MMIO_PIO_BASE3: c_uint = 0x330;
pub const B43_MMIO_PIO_BASE4: c_uint = 0x340;
pub const B43_MMIO_PIO_BASE5: c_uint = 0x350;
pub const B43_MMIO_PIO_BASE6: c_uint = 0x360;
pub const B43_MMIO_PIO_BASE7: c_uint = 0x370;
// PIO on core rev >= 11
pub const B43_MMIO_PIO11_BASE0: c_uint = 0x200;
pub const B43_MMIO_PIO11_BASE1: c_uint = 0x240;
pub const B43_MMIO_PIO11_BASE2: c_uint = 0x280;
pub const B43_MMIO_PIO11_BASE3: c_uint = 0x2C0;
pub const B43_MMIO_PIO11_BASE4: c_uint = 0x300;
pub const B43_MMIO_PIO11_BASE5: c_uint = 0x340;
pub const B43_MMIO_RADIO24_CONTROL: c_uint = 0x3D8	/* core rev >= 24 only */;
pub const B43_MMIO_RADIO24_DATA: c_uint = 0x3DA	/* core rev >= 24 only */;
pub const B43_MMIO_PHY_VER: c_uint = 0x3E0;
pub const B43_MMIO_PHY_RADIO: c_uint = 0x3E2;
pub const B43_MMIO_PHY0: c_uint = 0x3E6;
pub const B43_MMIO_ANTENNA: c_uint = 0x3E8;
pub const B43_MMIO_CHANNEL: c_uint = 0x3F0;
pub const B43_MMIO_CHANNEL_EXT: c_uint = 0x3F4;
pub const B43_MMIO_RADIO_CONTROL: c_uint = 0x3F6;
pub const B43_MMIO_RADIO_DATA_HIGH: c_uint = 0x3F8;
pub const B43_MMIO_RADIO_DATA_LOW: c_uint = 0x3FA;
pub const B43_MMIO_PHY_CONTROL: c_uint = 0x3FC;
pub const B43_MMIO_PHY_DATA: c_uint = 0x3FE;
pub const B43_MMIO_MACFILTER_CONTROL: c_uint = 0x420;
pub const B43_MMIO_MACFILTER_DATA: c_uint = 0x422;
pub const B43_MMIO_RCMTA_COUNT: c_uint = 0x43C;
pub const B43_MMIO_PSM_PHY_HDR: c_uint = 0x492;
pub const B43_MMIO_RADIO_HWENABLED_LO: c_uint = 0x49A;
pub const B43_MMIO_GPIO_CONTROL: c_uint = 0x49C;
pub const B43_MMIO_GPIO_MASK: c_uint = 0x49E;
pub const B43_MMIO_TXE0_CTL: c_uint = 0x500;
pub const B43_MMIO_TXE0_AUX: c_uint = 0x502;
pub const B43_MMIO_TXE0_TS_LOC: c_uint = 0x504;
pub const B43_MMIO_TXE0_TIME_OUT: c_uint = 0x506;
pub const B43_MMIO_TXE0_WM_0: c_uint = 0x508;
pub const B43_MMIO_TXE0_WM_1: c_uint = 0x50A;
pub const B43_MMIO_TXE0_PHYCTL: c_uint = 0x50C;
pub const B43_MMIO_TXE0_STATUS: c_uint = 0x50E;
pub const B43_MMIO_TXE0_MMPLCP0: c_uint = 0x510;
pub const B43_MMIO_TXE0_MMPLCP1: c_uint = 0x512;
pub const B43_MMIO_TXE0_PHYCTL1: c_uint = 0x514;
pub const B43_MMIO_XMTFIFODEF: c_uint = 0x520;
pub const B43_MMIO_XMTFIFO_FRAME_CNT: c_uint = 0x522	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFO_BYTE_CNT: c_uint = 0x524	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFO_HEAD: c_uint = 0x526	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFO_RD_PTR: c_uint = 0x528	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFO_WR_PTR: c_uint = 0x52A	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFODEF1: c_uint = 0x52C	/* core rev>= 16 only */;
pub const B43_MMIO_XMTFIFOCMD: c_uint = 0x540;
pub const B43_MMIO_XMTFIFOFLUSH: c_uint = 0x542;
pub const B43_MMIO_XMTFIFOTHRESH: c_uint = 0x544;
pub const B43_MMIO_XMTFIFORDY: c_uint = 0x546;
pub const B43_MMIO_XMTFIFOPRIRDY: c_uint = 0x548;
pub const B43_MMIO_XMTFIFORQPRI: c_uint = 0x54A;
pub const B43_MMIO_XMTTPLATETXPTR: c_uint = 0x54C;
pub const B43_MMIO_XMTTPLATEPTR: c_uint = 0x550;
pub const B43_MMIO_SMPL_CLCT_STRPTR: c_uint = 0x552	/* core rev>= 22 only */;
pub const B43_MMIO_SMPL_CLCT_STPPTR: c_uint = 0x554	/* core rev>= 22 only */;
pub const B43_MMIO_SMPL_CLCT_CURPTR: c_uint = 0x556	/* core rev>= 22 only */;
pub const B43_MMIO_XMTTPLATEDATALO: c_uint = 0x560;
pub const B43_MMIO_XMTTPLATEDATAHI: c_uint = 0x562;
pub const B43_MMIO_XMTSEL: c_uint = 0x568;
pub const B43_MMIO_XMTTXCNT: c_uint = 0x56A;
pub const B43_MMIO_XMTTXSHMADDR: c_uint = 0x56C;
pub const B43_MMIO_TSF_CFP_START_LOW: c_uint = 0x604;
pub const B43_MMIO_TSF_CFP_START_HIGH: c_uint = 0x606;
pub const B43_MMIO_TSF_CFP_PRETBTT: c_uint = 0x612;
pub const B43_MMIO_TSF_CLK_FRAC_LOW: c_uint = 0x62E;
pub const B43_MMIO_TSF_CLK_FRAC_HIGH: c_uint = 0x630;
pub const B43_MMIO_TSF_0: c_uint = 0x632	/* core rev < 3 only */;
pub const B43_MMIO_TSF_1: c_uint = 0x634	/* core rev < 3 only */;
pub const B43_MMIO_TSF_2: c_uint = 0x636	/* core rev < 3 only */;
pub const B43_MMIO_TSF_3: c_uint = 0x638	/* core rev < 3 only */;
pub const B43_MMIO_RNG: c_uint = 0x65A;
pub const B43_MMIO_IFSSLOT: c_uint = 0x684	/* Interframe slot time */;
pub const B43_MMIO_IFSCTL: c_uint = 0x688	/* Interframe space control */;
pub const B43_MMIO_IFSSTAT: c_uint = 0x690;
pub const B43_MMIO_IFSMEDBUSYCTL: c_uint = 0x692;
pub const B43_MMIO_IFTXDUR: c_uint = 0x694;
pub const B43_MMIO_IFSCTL_USE_EDCF: c_uint = 0x0004;
pub const B43_MMIO_POWERUP_DELAY: c_uint = 0x6A8;
pub const B43_MMIO_BTCOEX_CTL: c_uint = 0x6B4 /* Bluetooth Coexistence Control */;
pub const B43_MMIO_BTCOEX_STAT: c_uint = 0x6B6 /* Bluetooth Coexistence Status */;
pub const B43_MMIO_BTCOEX_TXCTL: c_uint = 0x6B8 /* Bluetooth Coexistence Transmit Control */;
pub const B43_MMIO_WEPCTL: c_uint = 0x7C0;
// SPROM boardflags_lo values
pub const B43_BFL_BTCOEXIST: c_uint = 0x0001	/* implements Bluetooth coexistance */;
pub const B43_BFL_PACTRL: c_uint = 0x0002	/* GPIO 9 controlling the PA */;
pub const B43_BFL_AIRLINEMODE: c_uint = 0x0004	/* implements GPIO 13 radio disable indication */;
pub const B43_BFL_RSSI: c_uint = 0x0008	/* software calculates nrssi slope. */;
pub const B43_BFL_ENETSPI: c_uint = 0x0010	/* has ephy roboswitch spi */;
pub const B43_BFL_XTAL_NOSLOW: c_uint = 0x0020	/* no slow clock available */;
pub const B43_BFL_CCKHIPWR: c_uint = 0x0040	/* can do high power CCK transmission */;
pub const B43_BFL_ENETADM: c_uint = 0x0080	/* has ADMtek switch */;
pub const B43_BFL_ENETVLAN: c_uint = 0x0100	/* can do vlan */;
pub const B43_BFL_AFTERBURNER: c_uint = 0x0200	/* supports Afterburner mode */;
pub const B43_BFL_NOPCI: c_uint = 0x0400	/* leaves PCI floating */;
pub const B43_BFL_FEM: c_uint = 0x0800	/* supports the Front End Module */;
pub const B43_BFL_EXTLNA: c_uint = 0x1000	/* has an external LNA */;
pub const B43_BFL_HGPA: c_uint = 0x2000	/* had high gain PA */;
pub const B43_BFL_BTCMOD: c_uint = 0x4000	/* BFL_BTCOEXIST is given in alternate GPIOs */;
pub const B43_BFL_ALTIQ: c_uint = 0x8000	/* alternate I/Q settings */;
// SPROM boardflags_hi values
pub const B43_BFH_NOPA: c_uint = 0x0001	/* has no PA */;
pub const B43_BFH_RSSIINV: c_uint = 0x0002	/* RSSI uses positive slope (not TSSI) */;
pub const B43_BFH_PAREF: c_uint = 0x0004	/* uses the PARef LDO */;
pub const B43_BFH_3TSWITCH: c_uint = 0x0008	/* uses a triple throw switch shared;
// with bluetooth
pub const B43_BFH_PHASESHIFT: c_uint = 0x0010	/* can support phase shifter */;
pub const B43_BFH_BUCKBOOST: c_uint = 0x0020	/* has buck/booster */;
pub const B43_BFH_FEM_BT: c_uint = 0x0040	/* has FEM and switch to share antenna;
// with bluetooth
pub const B43_BFH_NOCBUCK: c_uint = 0x0080;
pub const B43_BFH_PALDO: c_uint = 0x0200;
pub const B43_BFH_EXTLNA_5GHZ: c_uint = 0x1000	/* has an external LNA (5GHz mode) */;
// SPROM boardflags2_lo values
pub const B43_BFL2_RXBB_INT_REG_DIS: c_uint = 0x0001	/* external RX BB regulator present */;
pub const B43_BFL2_APLL_WAR: c_uint = 0x0002	/* alternative A-band PLL settings implemented */;
pub const B43_BFL2_TXPWRCTRL_EN: c_uint = 0x0004	/* permits enabling TX Power Control */;
pub const B43_BFL2_2X4_DIV: c_uint = 0x0008	/* 2x4 diversity switch */;
pub const B43_BFL2_5G_PWRGAIN: c_uint = 0x0010	/* supports 5G band power gain */;
pub const B43_BFL2_PCIEWAR_OVR: c_uint = 0x0020	/* overrides ASPM and Clkreq settings */;
pub const B43_BFL2_CAESERS_BRD: c_uint = 0x0040	/* is Caesers board (unused) */;
pub const B43_BFL2_BTC3WIRE: c_uint = 0x0080	/* used 3-wire bluetooth coexist */;
pub const B43_BFL2_SKWRKFEM_BRD: c_uint = 0x0100	/* 4321mcm93 uses Skyworks FEM */;
pub const B43_BFL2_SPUR_WAR: c_uint = 0x0200	/* has a workaround for clock-harmonic spurs */;
pub const B43_BFL2_GPLL_WAR: c_uint = 0x0400	/* altenative G-band PLL settings implemented */;
pub const B43_BFL2_SINGLEANT_CCK: c_uint = 0x1000;
pub const B43_BFL2_2G_SPUR_WAR: c_uint = 0x2000;
// SPROM boardflags2_hi values
pub const B43_BFH2_GPLL_WAR2: c_uint = 0x0001;
pub const B43_BFH2_IPALVLSHIFT_3P3: c_uint = 0x0002;
pub const B43_BFH2_INTERNDET_TXIQCAL: c_uint = 0x0004;
pub const B43_BFH2_XTALBUFOUTEN: c_uint = 0x0008;
// GPIO register offset, in both ChipCommon and PCI core.
pub const B43_GPIO_CONTROL: c_uint = 0x6c;
// SHM Routing
// SHM Routing modifiers
pub const B43_SHM_AUTOINC_R: c_uint = 0x0200	/* Auto-increment address on read */;
pub const B43_SHM_AUTOINC_W: c_uint = 0x0100	/* Auto-increment address on write */;

// Misc SHM_SHARED offsets
pub const B43_SHM_SH_WLCOREREV: c_uint = 0x0016	/* 802.11 core revision */;
pub const B43_SHM_SH_PCTLWDPOS: c_uint = 0x0008;
pub const B43_SHM_SH_RXPADOFF: c_uint = 0x0034	/* RX Padding data offset (PIO only) */;
pub const B43_SHM_SH_FWCAPA: c_uint = 0x0042	/* Firmware capabilities (Opensource firmware only) */;
pub const B43_SHM_SH_PHYVER: c_uint = 0x0050	/* PHY version */;
pub const B43_SHM_SH_PHYTYPE: c_uint = 0x0052	/* PHY type */;
pub const B43_SHM_SH_ANTSWAP: c_uint = 0x005C	/* Antenna swap threshold */;
pub const B43_SHM_SH_HOSTF1: c_uint = 0x005E	/* Hostflags 1 for ucode options */;
pub const B43_SHM_SH_HOSTF2: c_uint = 0x0060	/* Hostflags 2 for ucode options */;
pub const B43_SHM_SH_HOSTF3: c_uint = 0x0062	/* Hostflags 3 for ucode options */;
pub const B43_SHM_SH_RFATT: c_uint = 0x0064	/* Current radio attenuation value */;
pub const B43_SHM_SH_RADAR: c_uint = 0x0066	/* Radar register */;
pub const B43_SHM_SH_PHYTXNOI: c_uint = 0x006E	/* PHY noise directly after TX (lower 8bit only) */;
pub const B43_SHM_SH_RFRXSP1: c_uint = 0x0072	/* RF RX SP Register 1 */;
pub const B43_SHM_SH_HOSTF4: c_uint = 0x0078	/* Hostflags 4 for ucode options */;
pub const B43_SHM_SH_CHAN: c_uint = 0x00A0	/* Current channel (low 8bit only) */;
pub const B43_SHM_SH_CHAN_5GHZ: c_uint = 0x0100	/* Bit set, if 5 Ghz channel */;
pub const B43_SHM_SH_CHAN_40MHZ: c_uint = 0x0200	/* Bit set, if 40 Mhz channel width */;
pub const B43_SHM_SH_MACHW_L: c_uint = 0x00C0	/* Location where the ucode expects the MAC capabilities */;
pub const B43_SHM_SH_MACHW_H: c_uint = 0x00C2	/* Location where the ucode expects the MAC capabilities */;
pub const B43_SHM_SH_HOSTF5: c_uint = 0x00D4	/* Hostflags 5 for ucode options */;
pub const B43_SHM_SH_BCMCFIFOID: c_uint = 0x0108	/* Last posted cookie to the bcast/mcast FIFO */;
// TSSI information
pub const B43_SHM_SH_TSSI_CCK: c_uint = 0x0058	/* TSSI for last 4 CCK frames (32bit) */;
pub const B43_SHM_SH_TSSI_OFDM_A: c_uint = 0x0068	/* TSSI for last 4 OFDM frames (32bit) */;
pub const B43_SHM_SH_TSSI_OFDM_G: c_uint = 0x0070	/* TSSI for last 4 OFDM frames (32bit) */;
pub const B43_TSSI_MAX: c_uint = 0x7F	/* Max value for one TSSI value */;
// SHM_SHARED TX FIFO variables
pub const B43_SHM_SH_SIZE01: c_uint = 0x0098	/* TX FIFO size for FIFO 0 (low) and 1 (high) */;
pub const B43_SHM_SH_SIZE23: c_uint = 0x009A	/* TX FIFO size for FIFO 2 and 3 */;
pub const B43_SHM_SH_SIZE45: c_uint = 0x009C	/* TX FIFO size for FIFO 4 and 5 */;
pub const B43_SHM_SH_SIZE67: c_uint = 0x009E	/* TX FIFO size for FIFO 6 and 7 */;
// SHM_SHARED background noise
pub const B43_SHM_SH_JSSI0: c_uint = 0x0088	/* Measure JSSI 0 */;
pub const B43_SHM_SH_JSSI1: c_uint = 0x008A	/* Measure JSSI 1 */;
pub const B43_SHM_SH_JSSIAUX: c_uint = 0x008C	/* Measure JSSI AUX */;
// SHM_SHARED crypto engine
pub const B43_SHM_SH_DEFAULTIV: c_uint = 0x003C	/* Default IV location */;
pub const B43_SHM_SH_NRRXTRANS: c_uint = 0x003E	/* # of soft RX transmitter addresses (max 8) */;
pub const B43_SHM_SH_KTP: c_uint = 0x0056	/* Key table pointer */;
pub const B43_SHM_SH_TKIPTSCTTAK: c_uint = 0x0318;
pub const B43_SHM_SH_KEYIDXBLOCK: c_uint = 0x05D4	/* Key index/algorithm block (v4 firmware) */;
pub const B43_SHM_SH_PSM: c_uint = 0x05F4	/* PSM transmitter address match block (rev < 5) */;
// SHM_SHARED WME variables
pub const B43_SHM_SH_EDCFSTAT: c_uint = 0x000E	/* EDCF status */;
pub const B43_SHM_SH_TXFCUR: c_uint = 0x0030	/* TXF current index */;
pub const B43_SHM_SH_EDCFQ: c_uint = 0x0240	/* EDCF Q info */;
// SHM_SHARED powersave mode related
pub const B43_SHM_SH_SLOTT: c_uint = 0x0010	/* Slot time */;
pub const B43_SHM_SH_DTIMPER: c_uint = 0x0012	/* DTIM period */;
pub const B43_SHM_SH_NOSLPZNATDTIM: c_uint = 0x004C	/* NOSLPZNAT DTIM */;
// SHM_SHARED beacon/AP variables
pub const B43_SHM_SH_BT_BASE0: c_uint = 0x0068	/* Beacon template base 0 */;
pub const B43_SHM_SH_BTL0: c_uint = 0x0018	/* Beacon template length 0 */;
pub const B43_SHM_SH_BT_BASE1: c_uint = 0x0468	/* Beacon template base 1 */;
pub const B43_SHM_SH_BTL1: c_uint = 0x001A	/* Beacon template length 1 */;
pub const B43_SHM_SH_BTSFOFF: c_uint = 0x001C	/* Beacon TSF offset */;
pub const B43_SHM_SH_TIMBPOS: c_uint = 0x001E	/* TIM B position in beacon */;
pub const B43_SHM_SH_DTIMP: c_uint = 0x0012	/* DTIP period */;
pub const B43_SHM_SH_MCASTCOOKIE: c_uint = 0x00A8	/* Last bcast/mcast frame ID */;
pub const B43_SHM_SH_SFFBLIM: c_uint = 0x0044	/* Short frame fallback retry limit */;
pub const B43_SHM_SH_LFFBLIM: c_uint = 0x0046	/* Long frame fallback retry limit */;
pub const B43_SHM_SH_BEACPHYCTL: c_uint = 0x0054	/* Beacon PHY TX control word (see PHY TX control) */;
pub const B43_SHM_SH_EXTNPHYCTL: c_uint = 0x00B0	/* Extended bytes for beacon PHY control (N) */;
pub const B43_SHM_SH_BCN_LI: c_uint = 0x00B6	/* beacon listen interval */;
// SHM_SHARED ACK/CTS control
pub const B43_SHM_SH_ACKCTSPHYCTL: c_uint = 0x0022	/* ACK/CTS PHY control word (see PHY TX control) */;
// SHM_SHARED probe response variables
pub const B43_SHM_SH_PRSSID: c_uint = 0x0160	/* Probe Response SSID */;
pub const B43_SHM_SH_PRSSIDLEN: c_uint = 0x0048	/* Probe Response SSID length */;
pub const B43_SHM_SH_PRTLEN: c_uint = 0x004A	/* Probe Response template length */;
pub const B43_SHM_SH_PRMAXTIME: c_uint = 0x0074	/* Probe Response max time */;
pub const B43_SHM_SH_PRPHYCTL: c_uint = 0x0188	/* Probe Response PHY TX control word */;
// SHM_SHARED rate tables
pub const B43_SHM_SH_OFDMDIRECT: c_uint = 0x01C0	/* Pointer to OFDM direct map */;
pub const B43_SHM_SH_OFDMBASIC: c_uint = 0x01E0	/* Pointer to OFDM basic rate map */;
pub const B43_SHM_SH_CCKDIRECT: c_uint = 0x0200	/* Pointer to CCK direct map */;
pub const B43_SHM_SH_CCKBASIC: c_uint = 0x0220	/* Pointer to CCK basic rate map */;
// SHM_SHARED microcode soft registers
pub const B43_SHM_SH_UCODEREV: c_uint = 0x0000	/* Microcode revision */;
pub const B43_SHM_SH_UCODEPATCH: c_uint = 0x0002	/* Microcode patchlevel */;
pub const B43_SHM_SH_UCODEDATE: c_uint = 0x0004	/* Microcode date */;
pub const B43_SHM_SH_UCODETIME: c_uint = 0x0006	/* Microcode time */;
pub const B43_SHM_SH_UCODESTAT: c_uint = 0x0040	/* Microcode debug status code */;
pub const B43_SHM_SH_UCODESTAT_INVALID: c_int = 0;
pub const B43_SHM_SH_UCODESTAT_INIT: c_int = 1;
pub const B43_SHM_SH_UCODESTAT_ACTIVE: c_int = 2;

pub const B43_SHM_SH_MAXBFRAMES: c_uint = 0x0080	/* Maximum number of frames in a burst */;
pub const B43_SHM_SH_SPUWKUP: c_uint = 0x0094	/* pre-wakeup for synth PU in us */;
pub const B43_SHM_SH_PRETBTT: c_uint = 0x0096	/* pre-TBTT in us */;
// SHM_SHARED tx iq workarounds
pub const B43_SHM_SH_NPHY_TXIQW0: c_uint = 0x0700;
pub const B43_SHM_SH_NPHY_TXIQW1: c_uint = 0x0702;
pub const B43_SHM_SH_NPHY_TXIQW2: c_uint = 0x0704;
pub const B43_SHM_SH_NPHY_TXIQW3: c_uint = 0x0706;
// SHM_SHARED tx pwr ctrl
pub const B43_SHM_SH_NPHY_TXPWR_INDX0: c_uint = 0x0708;
pub const B43_SHM_SH_NPHY_TXPWR_INDX1: c_uint = 0x070E;
// SHM_SCRATCH offsets
pub const B43_SHM_SC_MINCONT: c_uint = 0x0003	/* Minimum contention window */;
pub const B43_SHM_SC_MAXCONT: c_uint = 0x0004	/* Maximum contention window */;
pub const B43_SHM_SC_CURCONT: c_uint = 0x0005	/* Current contention window */;
pub const B43_SHM_SC_SRLIMIT: c_uint = 0x0006	/* Short retry count limit */;
pub const B43_SHM_SC_LRLIMIT: c_uint = 0x0007	/* Long retry count limit */;
pub const B43_SHM_SC_DTIMC: c_uint = 0x0008	/* Current DTIM count */;
pub const B43_SHM_SC_BTL0LEN: c_uint = 0x0015	/* Beacon 0 template length */;
pub const B43_SHM_SC_BTL1LEN: c_uint = 0x0016	/* Beacon 1 template length */;
pub const B43_SHM_SC_SCFB: c_uint = 0x0017	/* Short frame transmit count threshold for rate fallback */;
pub const B43_SHM_SC_LCFB: c_uint = 0x0018	/* Long frame transmit count threshold for rate fallback */;
// Hardware Radio Enable masks

// HostFlags. See b43_hf_read/write()
pub const B43_HF_ANTDIVHELP: c_uint = 0x000000000001ULL /* ucode antenna div helper */;
pub const B43_HF_SYMW: c_uint = 0x000000000002ULL /* G-PHY SYM workaround */;
pub const B43_HF_RXPULLW: c_uint = 0x000000000004ULL /* RX pullup workaround */;
pub const B43_HF_CCKBOOST: c_uint = 0x000000000008ULL /* 4dB CCK power boost (exclusive with OFDM boost) */;
pub const B43_HF_BTCOEX: c_uint = 0x000000000010ULL /* Bluetooth coexistance */;
pub const B43_HF_GDCW: c_uint = 0x000000000020ULL /* G-PHY DC canceller filter bw workaround */;
pub const B43_HF_OFDMPABOOST: c_uint = 0x000000000040ULL /* Enable PA gain boost for OFDM */;
pub const B43_HF_ACPR: c_uint = 0x000000000080ULL /* Disable for Japan, channel 14 */;
pub const B43_HF_EDCF: c_uint = 0x000000000100ULL /* on if WME and MAC suspended */;
pub const B43_HF_TSSIRPSMW: c_uint = 0x000000000200ULL /* TSSI reset PSM ucode workaround */;
pub const B43_HF_20IN40IQW: c_uint = 0x000000000200ULL /* 20 in 40 MHz I/Q workaround (rev >= 13 only) */;
pub const B43_HF_DSCRQ: c_uint = 0x000000000400ULL /* Disable slow clock request in ucode */;
pub const B43_HF_ACIW: c_uint = 0x000000000800ULL /* ACI workaround: shift bits by 2 on PHY CRS */;
pub const B43_HF_2060W: c_uint = 0x000000001000ULL /* 2060 radio workaround */;
pub const B43_HF_RADARW: c_uint = 0x000000002000ULL /* Radar workaround */;
pub const B43_HF_USEDEFKEYS: c_uint = 0x000000004000ULL /* Enable use of default keys */;
pub const B43_HF_AFTERBURNER: c_uint = 0x000000008000ULL /* Afterburner enabled */;
pub const B43_HF_BT4PRIOCOEX: c_uint = 0x000000010000ULL /* Bluetooth 4-priority coexistance */;
pub const B43_HF_FWKUP: c_uint = 0x000000020000ULL /* Fast wake-up ucode */;
pub const B43_HF_VCORECALC: c_uint = 0x000000040000ULL /* Force VCO recalculation when powering up synthpu */;
pub const B43_HF_PCISCW: c_uint = 0x000000080000ULL /* PCI slow clock workaround */;
pub const B43_HF_4318TSSI: c_uint = 0x000000200000ULL /* 4318 TSSI */;
pub const B43_HF_FBCMCFIFO: c_uint = 0x000000400000ULL /* Flush bcast/mcast FIFO immediately */;
pub const B43_HF_HWPCTL: c_uint = 0x000000800000ULL /* Enable hardwarre power control */;
pub const B43_HF_BTCOEXALT: c_uint = 0x000001000000ULL /* Bluetooth coexistance in alternate pins */;
pub const B43_HF_TXBTCHECK: c_uint = 0x000002000000ULL /* Bluetooth check during transmission */;
pub const B43_HF_SKCFPUP: c_uint = 0x000004000000ULL /* Skip CFP update */;
pub const B43_HF_N40W: c_uint = 0x000008000000ULL /* N PHY 40 MHz workaround (rev >= 13 only) */;
pub const B43_HF_ANTSEL: c_uint = 0x000020000000ULL /* Antenna selection (for testing antenna div.) */;
pub const B43_HF_BT3COEXT: c_uint = 0x000020000000ULL /* Bluetooth 3-wire coexistence (rev >= 13 only) */;
pub const B43_HF_BTCANT: c_uint = 0x000040000000ULL /* Bluetooth coexistence (antenna mode) (rev >= 13 only) */;
pub const B43_HF_ANTSELEN: c_uint = 0x000100000000ULL /* Antenna selection enabled (rev >= 13 only) */;
pub const B43_HF_ANTSELMODE: c_uint = 0x000200000000ULL /* Antenna selection mode (rev >= 13 only) */;
pub const B43_HF_MLADVW: c_uint = 0x001000000000ULL /* N PHY ML ADV workaround (rev >= 13 only) */;
pub const B43_HF_PR45960W: c_uint = 0x080000000000ULL /* PR 45960 workaround (rev >= 13 only) */;
// Firmware capabilities field in SHM (Opensource firmware only)
pub const B43_FWCAPA_HWCRYPTO: c_uint = 0x0001;
pub const B43_FWCAPA_QOS: c_uint = 0x0002;
// MacFilter offsets.
pub const B43_MACFILTER_SELF: c_uint = 0x0000;
pub const B43_MACFILTER_BSSID: c_uint = 0x0003;
// PowerControl
pub const B43_PCTL_IN: c_uint = 0xB0;
pub const B43_PCTL_OUT: c_uint = 0xB4;
pub const B43_PCTL_OUTENABLE: c_uint = 0xB8;
pub const B43_PCTL_XTAL_POWERUP: c_uint = 0x40;
pub const B43_PCTL_PLL_POWERDOWN: c_uint = 0x80;
// PowerControl Clock Modes
pub const B43_PCTL_CLK_FAST: c_uint = 0x00;
pub const B43_PCTL_CLK_SLOW: c_uint = 0x01;
pub const B43_PCTL_CLK_DYNAMIC: c_uint = 0x02;
pub const B43_PCTL_FORCE_SLOW: c_uint = 0x0800;
pub const B43_PCTL_FORCE_PLL: c_uint = 0x1000;
pub const B43_PCTL_DYN_XTAL: c_uint = 0x2000;
// PHYVersioning
pub const B43_PHYTYPE_A: c_uint = 0x00;
pub const B43_PHYTYPE_B: c_uint = 0x01;
pub const B43_PHYTYPE_G: c_uint = 0x02;
pub const B43_PHYTYPE_N: c_uint = 0x04;
pub const B43_PHYTYPE_LP: c_uint = 0x05;
pub const B43_PHYTYPE_SSLPN: c_uint = 0x06;
pub const B43_PHYTYPE_HT: c_uint = 0x07;
pub const B43_PHYTYPE_LCN: c_uint = 0x08;
pub const B43_PHYTYPE_LCNXN: c_uint = 0x09;
pub const B43_PHYTYPE_LCN40: c_uint = 0x0a;
pub const B43_PHYTYPE_AC: c_uint = 0x0b;
// PHYRegisters
pub const B43_PHY_ILT_A_CTRL: c_uint = 0x0072;
pub const B43_PHY_ILT_A_DATA1: c_uint = 0x0073;
pub const B43_PHY_ILT_A_DATA2: c_uint = 0x0074;
pub const B43_PHY_G_LO_CONTROL: c_uint = 0x0810;
pub const B43_PHY_ILT_G_CTRL: c_uint = 0x0472;
pub const B43_PHY_ILT_G_DATA1: c_uint = 0x0473;
pub const B43_PHY_ILT_G_DATA2: c_uint = 0x0474;
pub const B43_PHY_A_PCTL: c_uint = 0x007B;
pub const B43_PHY_G_PCTL: c_uint = 0x0029;
pub const B43_PHY_A_CRS: c_uint = 0x0029;
pub const B43_PHY_RADIO_BITFIELD: c_uint = 0x0401;
pub const B43_PHY_G_CRS: c_uint = 0x0429;
pub const B43_PHY_NRSSILT_CTRL: c_uint = 0x0803;
pub const B43_PHY_NRSSILT_DATA: c_uint = 0x0804;
// RadioRegisters
pub const B43_RADIOCTL_ID: c_uint = 0x01;
// MAC Control bitfield
pub const B43_MACCTL_ENABLED: c_uint = 0x00000001	/* MAC Enabled */;
pub const B43_MACCTL_PSM_RUN: c_uint = 0x00000002	/* Run Microcode */;
pub const B43_MACCTL_PSM_JMP0: c_uint = 0x00000004	/* Microcode jump to 0 */;
pub const B43_MACCTL_SHM_ENABLED: c_uint = 0x00000100	/* SHM Enabled */;
pub const B43_MACCTL_SHM_UPPER: c_uint = 0x00000200	/* SHM Upper */;
pub const B43_MACCTL_IHR_ENABLED: c_uint = 0x00000400	/* IHR Region Enabled */;
pub const B43_MACCTL_PSM_DBG: c_uint = 0x00002000	/* Microcode debugging enabled */;
pub const B43_MACCTL_GPOUTSMSK: c_uint = 0x0000C000	/* GPOUT Select Mask */;
pub const B43_MACCTL_BE: c_uint = 0x00010000	/* Big Endian mode */;
pub const B43_MACCTL_INFRA: c_uint = 0x00020000	/* Infrastructure mode */;
pub const B43_MACCTL_AP: c_uint = 0x00040000	/* AccessPoint mode */;
pub const B43_MACCTL_RADIOLOCK: c_uint = 0x00080000	/* Radio lock */;
pub const B43_MACCTL_BEACPROMISC: c_uint = 0x00100000	/* Beacon Promiscuous */;
pub const B43_MACCTL_KEEP_BADPLCP: c_uint = 0x00200000	/* Keep frames with bad PLCP */;
pub const B43_MACCTL_PHY_LOCK: c_uint = 0x00200000;
pub const B43_MACCTL_KEEP_CTL: c_uint = 0x00400000	/* Keep control frames */;
pub const B43_MACCTL_KEEP_BAD: c_uint = 0x00800000	/* Keep bad frames (FCS) */;
pub const B43_MACCTL_PROMISC: c_uint = 0x01000000	/* Promiscuous mode */;
pub const B43_MACCTL_HWPS: c_uint = 0x02000000	/* Hardware Power Saving */;
pub const B43_MACCTL_AWAKE: c_uint = 0x04000000	/* Device is awake */;
pub const B43_MACCTL_CLOSEDNET: c_uint = 0x08000000	/* Closed net (no SSID bcast) */;
pub const B43_MACCTL_TBTTHOLD: c_uint = 0x10000000	/* TBTT Hold */;
pub const B43_MACCTL_DISCTXSTAT: c_uint = 0x20000000	/* Discard TX status */;
pub const B43_MACCTL_DISCPMQ: c_uint = 0x40000000	/* Discard Power Management Queue */;
pub const B43_MACCTL_GMODE: c_uint = 0x80000000	/* G Mode */;
// MAC Command bitfield
pub const B43_MACCMD_BEACON0_VALID: c_uint = 0x00000001	/* Beacon 0 in template RAM is busy/valid */;
pub const B43_MACCMD_BEACON1_VALID: c_uint = 0x00000002	/* Beacon 1 in template RAM is busy/valid */;
pub const B43_MACCMD_DFQ_VALID: c_uint = 0x00000004	/* Directed frame queue valid (IBSS PS mode, ATIM) */;
pub const B43_MACCMD_CCA: c_uint = 0x00000008	/* Clear channel assessment */;
pub const B43_MACCMD_BGNOISE: c_uint = 0x00000010	/* Background noise */;
// B43_MMIO_PSM_PHY_HDR bits
pub const B43_PSM_HDR_MAC_PHY_RESET: c_uint = 0x00000001;
pub const B43_PSM_HDR_MAC_PHY_CLOCK_EN: c_uint = 0x00000002;
pub const B43_PSM_HDR_MAC_PHY_FORCE_CLK: c_uint = 0x00000004;
// See BCMA_CLKCTLST_EXTRESREQ and BCMA_CLKCTLST_EXTRESST
pub const B43_BCMA_CLKCTLST_80211_PLL_REQ: c_uint = 0x00000100;
pub const B43_BCMA_CLKCTLST_PHY_PLL_REQ: c_uint = 0x00000200;
pub const B43_BCMA_CLKCTLST_80211_PLL_ST: c_uint = 0x01000000;
pub const B43_BCMA_CLKCTLST_PHY_PLL_ST: c_uint = 0x02000000;
// BCMA 802.11 core specific IO Control (BCMA_IOCTL) flags
pub const B43_BCMA_IOCTL_PHY_CLKEN: c_uint = 0x00000004	/* PHY Clock Enable */;
pub const B43_BCMA_IOCTL_PHY_RESET: c_uint = 0x00000008	/* PHY Reset */;
pub const B43_BCMA_IOCTL_MACPHYCLKEN: c_uint = 0x00000010	/* MAC PHY Clock Control Enable */;
pub const B43_BCMA_IOCTL_PLLREFSEL: c_uint = 0x00000020	/* PLL Frequency Reference Select */;
pub const B43_BCMA_IOCTL_PHY_BW: c_uint = 0x000000C0	/* PHY band width and clock speed mask (N-PHY+ only?) */;
pub const B43_BCMA_IOCTL_PHY_BW_10MHZ: c_uint = 0x00000000	/* 10 MHz bandwidth, 40 MHz PHY */;
pub const B43_BCMA_IOCTL_PHY_BW_20MHZ: c_uint = 0x00000040	/* 20 MHz bandwidth, 80 MHz PHY */;
pub const B43_BCMA_IOCTL_PHY_BW_40MHZ: c_uint = 0x00000080	/* 40 MHz bandwidth, 160 MHz PHY */;
pub const B43_BCMA_IOCTL_PHY_BW_80MHZ: c_uint = 0x000000C0	/* 80 MHz bandwidth */;
pub const B43_BCMA_IOCTL_DAC: c_uint = 0x00000300	/* Highspeed DAC mode control field */;
pub const B43_BCMA_IOCTL_GMODE: c_uint = 0x00002000	/* G Mode Enable */;
// BCMA 802.11 core specific IO status (BCMA_IOST) flags
pub const B43_BCMA_IOST_2G_PHY: c_uint = 0x00000001	/* 2.4G capable phy */;
pub const B43_BCMA_IOST_5G_PHY: c_uint = 0x00000002	/* 5G capable phy */;
pub const B43_BCMA_IOST_FASTCLKA: c_uint = 0x00000004	/* Fast Clock Available */;
pub const B43_BCMA_IOST_DUALB_PHY: c_uint = 0x00000008	/* Dualband phy */;
// 802.11 core specific TM State Low (SSB_TMSLOW) flags
pub const B43_TMSLOW_GMODE: c_uint = 0x20000000	/* G Mode Enable */;
pub const B43_TMSLOW_PHY_BANDWIDTH: c_uint = 0x00C00000	/* PHY band width and clock speed mask (N-PHY only) */;
pub const B43_TMSLOW_PHY_BANDWIDTH_10MHZ: c_uint = 0x00000000	/* 10 MHz bandwidth, 40 MHz PHY */;
pub const B43_TMSLOW_PHY_BANDWIDTH_20MHZ: c_uint = 0x00400000	/* 20 MHz bandwidth, 80 MHz PHY */;
pub const B43_TMSLOW_PHY_BANDWIDTH_40MHZ: c_uint = 0x00800000	/* 40 MHz bandwidth, 160 MHz PHY */;
pub const B43_TMSLOW_PLLREFSEL: c_uint = 0x00200000	/* PLL Frequency Reference Select (rev >= 5) */;
pub const B43_TMSLOW_MACPHYCLKEN: c_uint = 0x00100000	/* MAC PHY Clock Control Enable (rev >= 5) */;
pub const B43_TMSLOW_PHYRESET: c_uint = 0x00080000	/* PHY Reset */;
pub const B43_TMSLOW_PHYCLKEN: c_uint = 0x00040000	/* PHY Clock Enable */;
// 802.11 core specific TM State High (SSB_TMSHIGH) flags
pub const B43_TMSHIGH_DUALBAND_PHY: c_uint = 0x00080000	/* Dualband PHY available */;
pub const B43_TMSHIGH_FCLOCK: c_uint = 0x00040000	/* Fast Clock Available (rev >= 5) */;
pub const B43_TMSHIGH_HAVE_5GHZ_PHY: c_uint = 0x00020000	/* 5 GHz PHY available (rev >= 5) */;
pub const B43_TMSHIGH_HAVE_2GHZ_PHY: c_uint = 0x00010000	/* 2.4 GHz PHY available (rev >= 5) */;
// Generic-Interrupt reasons.
pub const B43_IRQ_MAC_SUSPENDED: c_uint = 0x00000001;
pub const B43_IRQ_BEACON: c_uint = 0x00000002;
pub const B43_IRQ_TBTT_INDI: c_uint = 0x00000004;
pub const B43_IRQ_BEACON_TX_OK: c_uint = 0x00000008;
pub const B43_IRQ_BEACON_CANCEL: c_uint = 0x00000010;
pub const B43_IRQ_ATIM_END: c_uint = 0x00000020;
pub const B43_IRQ_PMQ: c_uint = 0x00000040;
pub const B43_IRQ_PIO_WORKAROUND: c_uint = 0x00000100;
pub const B43_IRQ_MAC_TXERR: c_uint = 0x00000200;
pub const B43_IRQ_PHY_TXERR: c_uint = 0x00000800;
pub const B43_IRQ_PMEVENT: c_uint = 0x00001000;
pub const B43_IRQ_TIMER0: c_uint = 0x00002000;
pub const B43_IRQ_TIMER1: c_uint = 0x00004000;
pub const B43_IRQ_DMA: c_uint = 0x00008000;
pub const B43_IRQ_TXFIFO_FLUSH_OK: c_uint = 0x00010000;
pub const B43_IRQ_CCA_MEASURE_OK: c_uint = 0x00020000;
pub const B43_IRQ_NOISESAMPLE_OK: c_uint = 0x00040000;
pub const B43_IRQ_UCODE_DEBUG: c_uint = 0x08000000;
pub const B43_IRQ_RFKILL: c_uint = 0x10000000;
pub const B43_IRQ_TX_OK: c_uint = 0x20000000;
pub const B43_IRQ_PHY_G_CHANGED: c_uint = 0x40000000;
pub const B43_IRQ_TIMEOUT: c_uint = 0x80000000;
pub const B43_IRQ_ALL: c_uint = 0xFFFFFFFF;

// The firmware register to fetch the debug-IRQ reason from.
pub const B43_DEBUGIRQ_REASON_REG: c_int = 63;
// Debug-IRQ reasons.

pub const B43_DEBUGIRQ_ACK: c_uint = 0xFFFF	/* The host writes that to ACK the IRQ */;
// The firmware register that contains the "marker" line.
pub const B43_MARKER_ID_REG: c_int = 2;
pub const B43_MARKER_LINE_REG: c_int = 3;
// The firmware register to fetch the panic reason from.
pub const B43_FWPANIC_REASON_REG: c_int = 3;
// Firmware panic reason codes

// The firmware register that contains the watchdog counter.
pub const B43_WATCHDOG_REG: c_int = 1;
// Device specific rate values.
// The actual values defined here are (rate_in_mbps * 2).
// Some code depends on this. Don't change it.
pub const B43_CCK_RATE_1MB: c_uint = 0x02;
pub const B43_CCK_RATE_2MB: c_uint = 0x04;
pub const B43_CCK_RATE_5MB: c_uint = 0x0B;
pub const B43_CCK_RATE_11MB: c_uint = 0x16;
pub const B43_OFDM_RATE_6MB: c_uint = 0x0C;
pub const B43_OFDM_RATE_9MB: c_uint = 0x12;
pub const B43_OFDM_RATE_12MB: c_uint = 0x18;
pub const B43_OFDM_RATE_18MB: c_uint = 0x24;
pub const B43_OFDM_RATE_24MB: c_uint = 0x30;
pub const B43_OFDM_RATE_36MB: c_uint = 0x48;
pub const B43_OFDM_RATE_48MB: c_uint = 0x60;
pub const B43_OFDM_RATE_54MB: c_uint = 0x6C;
// Convert a b43 rate value to a rate in 100kbps

pub const B43_DEFAULT_SHORT_RETRY_LIMIT: c_int = 7;
pub const B43_DEFAULT_LONG_RETRY_LIMIT: c_int = 4;
pub const B43_PHY_TX_BADNESS_LIMIT: c_int = 1000;
// Max size of a security key
pub const B43_SEC_KEYSIZE: c_int = 16;
// Max number of group keys
pub const B43_NR_GROUP_KEYS: c_int = 4;
// Max number of pairwise keys
pub const B43_NR_PAIRWISE_KEYS: c_int = 50;
// Security algorithms.
// The firmware file header

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_fw_header {
// File type
    pub type: u8,
// File format version
    pub ver: u8,
    pub __padding: [u8; 2],
// Size of the data. For ucode and PCM this is in bytes.
// For IV this is number-of-ivs.
    pub size: __be32,
    pub __packed: },
// Initial Value file format
pub const B43_IV_OFFSET_MASK: c_uint = 0x7FFF;
pub const B43_IV_32BIT: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_iv {
    pub offset_size: __be16,
    pub d16: __be16,
    pub d32: __be32,
    pub data: } __packed,
    pub __packed: },
// Data structures for DMA transmission, per 80211 core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dma {
    pub /: *mut *mut *mut b43_dmaring tx_ring_AC_BK; / Background,
    pub /: *mut *mut *mut b43_dmaring tx_ring_AC_BE; / Best Effort,
    pub /: *mut *mut *mut b43_dmaring tx_ring_AC_VI; / Video,
    pub /: *mut *mut *mut b43_dmaring tx_ring_AC_VO; / Voice,
    pub /: *mut *mut *mut b43_dmaring tx_ring_mcast; / Multicast,
    pub rx_ring: *mut b43_dmaring,
    pub /: *mut *mut u32 translation; / Routing bits,
    pub /: *mut *mut bool translation_in_low; / Should translation bit go into low addr?,
    pub /: *mut *mut bool parity; / Check for parity,
}

// Data structures for PIO transmission, per 80211 core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_pio {
    pub /: *mut *mut *mut b43_pio_txqueue tx_queue_AC_BK; / Background,
    pub /: *mut *mut *mut b43_pio_txqueue tx_queue_AC_BE; / Best Effort,
    pub /: *mut *mut *mut b43_pio_txqueue tx_queue_AC_VI; / Video,
    pub /: *mut *mut *mut b43_pio_txqueue tx_queue_AC_VO; / Voice,
    pub /: *mut *mut *mut b43_pio_txqueue tx_queue_mcast; / Multicast,
    pub rx_queue: *mut b43_pio_rxqueue,
}

// Context information for a noise calculation (Link Quality).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_noise_calculation {
    pub calculation_running: bool,
    pub nr_samples: u8,
    pub samples: [i8; 8][4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_stats {
    pub link_noise: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_key {
// If keyconf is NULL, this key is disabled.
// keyconf is a cookie. Don't derefenrence it outside of the set_key
// path, because b43 doesn't own it.
    pub keyconf: *mut ieee80211_key_conf,
    pub algorithm: u8,
}

// SHM offsets to the QOS data structures for the 4 different queues.
pub const B43_QOS_QUEUE_NUM: c_int = 4;

// QOS parameter hardware data structure offsets.
pub const B43_NR_QOSPARAMS: c_int = 16;
// QOS parameters for a queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_qos_params {
// The QOS parameters
    pub p: ieee80211_tx_queue_params,
}

// The type of the firmware file.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_firmware_file_type {
    B43_FWTYPE_PROPRIETARY,
    B43_FWTYPE_OPENSOURCE,
    B43_NR_FWTYPES,
}

// Context data for fetching firmware.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_request_fw_context {
// The device we are requesting the fw for.
    pub dev: *mut b43_wldev,
// a pointer to the firmware object
    pub blob: *const firmware,
// The type of firmware to request.
    pub req_type: b43_firmware_file_type,
// Error messages for each firmware type.
    pub errors: [c_char; B43_NR_FWTYPES][128],
// Temporary buffer for storing the firmware name.
    pub fwname: [c_char; 64],
// A fatal error occurred while requesting. Firmware request
// can not continue, as any other request will also fail.
    pub fatal_failure: c_int,
}

// In-memory representation of a cached microcode file.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_firmware_file {
    pub filename: *const c_char,
    pub data: *const firmware,
// Type of the firmware file name. Note that this does only indicate
// the type by the firmware name. NOT the file contents.
// If you want to check for proprietary vs opensource, use (struct b43_firmware)->opensource
// instead! The (struct b43_firmware)->opensource flag is derived from the actual firmware
// binary code, not just the filename.
//
    pub type: b43_firmware_file_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_firmware_hdr_format {
    B43_FW_HDR_598,
    B43_FW_HDR_410,
    B43_FW_HDR_351,
}

// Pointers to the firmware data and meta information about it.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_firmware {
// Microcode
    pub ucode: b43_firmware_file,
// PCM code
    pub pcm: b43_firmware_file,
// Initial MMIO values for the firmware
    pub initvals: b43_firmware_file,
// Initial MMIO values for the firmware, band-specific
    pub initvals_band: b43_firmware_file,
// Firmware revision
    pub rev: u16,
// Firmware patchlevel
    pub patch: u16,
// Format of header used by firmware
    pub hdr_format: b43_firmware_hdr_format,
// Set to true, if we are using an opensource firmware.
// Use this to check for proprietary vs opensource.
    pub opensource: bool,
// Set to true, if the core needs a PCM firmware, but
// we failed to load one. This is always false for
// core rev > 10, as these don't need PCM firmware.
    pub pcm_request_failed: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_band {
    B43_BAND_2G = 0,
    B43_BAND_5G_LO = 1,
    B43_BAND_5G_MI = 2,
    B43_BAND_5G_HI = 3,
}

// Device (802.11 core) initialization status.

// Data structure for one wireless device (802.11 core)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_wldev {
    pub dev: *mut b43_bus_dev,
    pub wl: *mut b43_wl,
// a completion event structure needed if this call is asynchronous
    pub fw_load_complete: completion,
// The device initialization status.
// Use b43_status() to query.
    pub __init_status: core::sync::atomic::AtomicI32,
    pub /: *mut *mut bool bad_frames_preempt; / Use "Bad Frames Preemption" (default off),
    pub /: *mut *mut bool dfq_valid; / Directed frame queue valid (IBSS PS mode, ATIM),
    pub /: *mut *mut bool radio_hw_enable; / saved state of radio hardware enabled state,
    pub /: *mut *mut bool qos_enabled; / TRUE, if QoS is used.,
    pub /: *mut *mut bool hwcrypto_enabled; / TRUE, if HW crypto acceleration is enabled.,
    pub /: *mut *mut bool use_pio; / TRUE if next init should use PIO,
// PHY/Radio device.
    pub phy: b43_phy,
// DMA engines.
    pub dma: b43_dma,
// PIO engines.
    pub pio: b43_pio,
}

// Use b43_using_pio_transfers() to check whether we are using
// DMA or PIO data transfers.
// Various statistics about the physical device.
// Reason code of the last interrupt.
// The currently active generic-interrupt mask.
// Link Quality calculation context.
// if > 0 MAC is suspended. if == 0 MAC is enabled.
// Periodic tasks
// encryption/decryption
// Firmware data
// Devicelist in struct b43_wl (all 802.11 cores)
// Debugging stuff follows.

// Data structure for the WLAN parts (802.11 cores) of the b43 chip.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_wl {
// Pointer to the active wireless device on this chip
    pub current_dev: *mut b43_wldev,
// Pointer to the ieee80211 hardware data structure
    pub hw: *mut ieee80211_hw,
// Global driver mutex. Every operation must run with this mutex locked.
    pub mutex: mutex,
// Hard-IRQ spinlock. This lock protects things used in the hard-IRQ
// handler, only. This basically is just the IRQ mask register.
    pub hardirq_lock: spinlock_t,
// Set this if we call ieee80211_register_hw() and check if we call
// ieee80211_unregister_hw().
    pub hw_registered: bool,
// We can only have one operating interface (802.11 core)
// at a time. General information about this interface follows.
//
    pub vif: *mut ieee80211_vif,
// The MAC address of the operating interface.
    pub mac_addr: [u8; ETH_ALEN],
// Current BSSID
    pub bssid: [u8; ETH_ALEN],
// Interface type. (NL80211_IFTYPE_XXX)
    pub if_type: c_int,
// Is the card operating in AP, STA or IBSS mode?
    pub operating: bool,
// filter flags
    pub filter_flags: c_uint,
// Stats about the wireless interface
    pub ieee_stats: ieee80211_low_level_stats,

    pub rng: hwrng,
    pub rng_initialized: bool,
    pub 1]: char rng_name[30 +,

    pub radiotap_enabled: bool,
    pub radio_enabled: bool,
// The beacon we are currently using (AP or IBSS mode).
    pub current_beacon: *mut sk_buff,
    pub beacon0_uploaded: bool,
    pub beacon1_uploaded: bool,
    pub /: *mut *mut bool beacon_templates_virgin; / Never wrote the templates?,
    pub beacon_update_trigger: work_struct,
    pub beacon_lock: spinlock_t,
// The current QOS parameters for the 4 queues.
    pub qos_params: [b43_qos_params; B43_QOS_QUEUE_NUM],
// Work for adjustment of the transmission power.
// This is scheduled when we determine that the actual TX output
// power doesn't match what we want.
    pub txpower_adjust_work: work_struct,
// Packet transmit work
    pub tx_work: work_struct,
// Queue of packets to be transmitted.
    pub tx_queue: [sk_buff_head; B43_QOS_QUEUE_NUM],
// Flag that implement the queues stopping.
    pub tx_queue_stopped: [bool; B43_QOS_QUEUE_NUM],
// firmware loading work
    pub firmware_load: work_struct,
// The device LEDs.
    pub leds: b43_leds,
// Kmalloc'ed scratch space for PIO TX/RX. Protected by wl->mutex.
    pub __attribute__((__aligned__(8))): u8 pio_scratchspace[118],
    pub __attribute__((__aligned__(8))): u8 pio_tailspace[4],
}

extern "C" {
    pub fn ssb_get_drvdata(_arg: ssb_dev) -> return;
}
// Is the device operating in a specified mode (NL80211_IFTYPE_XXX).
//
// b43_current_band - Returns the currently used band.
// Returns one of NL80211_BAND_2GHZ and NL80211_BAND_5GHZ.
//
// To optimize this check for flush_writes on BCM47XX_BCMA only.

// Message printing
// A WARN_ON variant that vanishes when b43 debugging is disabled.
// This _also_ evaluates the arg with debugging disabled.

// Convert an integer to a Q5.2 value

// Convert a Q5.2 value to an integer (precision loss!)

// Macros for printing a value in Q5.2 format

