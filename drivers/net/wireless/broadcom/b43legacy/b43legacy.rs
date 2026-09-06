//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/b43legacy.h
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

// Macro flag: #define B43legacy_H_

pub const B43legacy_IRQWAIT_MAX_RETRIES: c_int = 20;
// MMIO offsets
pub const B43legacy_MMIO_DMA0_REASON: c_uint = 0x20;
pub const B43legacy_MMIO_DMA0_IRQ_MASK: c_uint = 0x24;
pub const B43legacy_MMIO_DMA1_REASON: c_uint = 0x28;
pub const B43legacy_MMIO_DMA1_IRQ_MASK: c_uint = 0x2C;
pub const B43legacy_MMIO_DMA2_REASON: c_uint = 0x30;
pub const B43legacy_MMIO_DMA2_IRQ_MASK: c_uint = 0x34;
pub const B43legacy_MMIO_DMA3_REASON: c_uint = 0x38;
pub const B43legacy_MMIO_DMA3_IRQ_MASK: c_uint = 0x3C;
pub const B43legacy_MMIO_DMA4_REASON: c_uint = 0x40;
pub const B43legacy_MMIO_DMA4_IRQ_MASK: c_uint = 0x44;
pub const B43legacy_MMIO_DMA5_REASON: c_uint = 0x48;
pub const B43legacy_MMIO_DMA5_IRQ_MASK: c_uint = 0x4C;
pub const B43legacy_MMIO_MACCTL: c_uint = 0x120	/* MAC control */;
pub const B43legacy_MMIO_MACCMD: c_uint = 0x124	/* MAC command */;
pub const B43legacy_MMIO_GEN_IRQ_REASON: c_uint = 0x128;
pub const B43legacy_MMIO_GEN_IRQ_MASK: c_uint = 0x12C;
pub const B43legacy_MMIO_RAM_CONTROL: c_uint = 0x130;
pub const B43legacy_MMIO_RAM_DATA: c_uint = 0x134;
pub const B43legacy_MMIO_PS_STATUS: c_uint = 0x140;
pub const B43legacy_MMIO_RADIO_HWENABLED_HI: c_uint = 0x158;
pub const B43legacy_MMIO_SHM_CONTROL: c_uint = 0x160;
pub const B43legacy_MMIO_SHM_DATA: c_uint = 0x164;
pub const B43legacy_MMIO_SHM_DATA_UNALIGNED: c_uint = 0x166;
pub const B43legacy_MMIO_XMITSTAT_0: c_uint = 0x170;
pub const B43legacy_MMIO_XMITSTAT_1: c_uint = 0x174;
pub const B43legacy_MMIO_REV3PLUS_TSF_LOW: c_uint = 0x180 /* core rev >= 3 only */;
pub const B43legacy_MMIO_REV3PLUS_TSF_HIGH: c_uint = 0x184 /* core rev >= 3 only */;
pub const B43legacy_MMIO_TSF_CFP_REP: c_uint = 0x188;
pub const B43legacy_MMIO_TSF_CFP_START: c_uint = 0x18C;
// 32-bit DMA
pub const B43legacy_MMIO_DMA32_BASE0: c_uint = 0x200;
pub const B43legacy_MMIO_DMA32_BASE1: c_uint = 0x220;
pub const B43legacy_MMIO_DMA32_BASE2: c_uint = 0x240;
pub const B43legacy_MMIO_DMA32_BASE3: c_uint = 0x260;
pub const B43legacy_MMIO_DMA32_BASE4: c_uint = 0x280;
pub const B43legacy_MMIO_DMA32_BASE5: c_uint = 0x2A0;
// 64-bit DMA
pub const B43legacy_MMIO_DMA64_BASE0: c_uint = 0x200;
pub const B43legacy_MMIO_DMA64_BASE1: c_uint = 0x240;
pub const B43legacy_MMIO_DMA64_BASE2: c_uint = 0x280;
pub const B43legacy_MMIO_DMA64_BASE3: c_uint = 0x2C0;
pub const B43legacy_MMIO_DMA64_BASE4: c_uint = 0x300;
pub const B43legacy_MMIO_DMA64_BASE5: c_uint = 0x340;
// PIO
pub const B43legacy_MMIO_PIO1_BASE: c_uint = 0x300;
pub const B43legacy_MMIO_PIO2_BASE: c_uint = 0x310;
pub const B43legacy_MMIO_PIO3_BASE: c_uint = 0x320;
pub const B43legacy_MMIO_PIO4_BASE: c_uint = 0x330;
pub const B43legacy_MMIO_PHY_VER: c_uint = 0x3E0;
pub const B43legacy_MMIO_PHY_RADIO: c_uint = 0x3E2;
pub const B43legacy_MMIO_PHY0: c_uint = 0x3E6;
pub const B43legacy_MMIO_ANTENNA: c_uint = 0x3E8;
pub const B43legacy_MMIO_CHANNEL: c_uint = 0x3F0;
pub const B43legacy_MMIO_CHANNEL_EXT: c_uint = 0x3F4;
pub const B43legacy_MMIO_RADIO_CONTROL: c_uint = 0x3F6;
pub const B43legacy_MMIO_RADIO_DATA_HIGH: c_uint = 0x3F8;
pub const B43legacy_MMIO_RADIO_DATA_LOW: c_uint = 0x3FA;
pub const B43legacy_MMIO_PHY_CONTROL: c_uint = 0x3FC;
pub const B43legacy_MMIO_PHY_DATA: c_uint = 0x3FE;
pub const B43legacy_MMIO_MACFILTER_CONTROL: c_uint = 0x420;
pub const B43legacy_MMIO_MACFILTER_DATA: c_uint = 0x422;
pub const B43legacy_MMIO_RCMTA_COUNT: c_uint = 0x43C /* Receive Match Transmitter Addr */;
pub const B43legacy_MMIO_RADIO_HWENABLED_LO: c_uint = 0x49A;
pub const B43legacy_MMIO_GPIO_CONTROL: c_uint = 0x49C;
pub const B43legacy_MMIO_GPIO_MASK: c_uint = 0x49E;
pub const B43legacy_MMIO_TSF_CFP_PRETBTT: c_uint = 0x612;
pub const B43legacy_MMIO_TSF_0: c_uint = 0x632 /* core rev < 3 only */;
pub const B43legacy_MMIO_TSF_1: c_uint = 0x634 /* core rev < 3 only */;
pub const B43legacy_MMIO_TSF_2: c_uint = 0x636 /* core rev < 3 only */;
pub const B43legacy_MMIO_TSF_3: c_uint = 0x638 /* core rev < 3 only */;
pub const B43legacy_MMIO_RNG: c_uint = 0x65A;
pub const B43legacy_MMIO_POWERUP_DELAY: c_uint = 0x6A8;
// SPROM boardflags_lo values
pub const B43legacy_BFL_PACTRL: c_uint = 0x0002;
pub const B43legacy_BFL_RSSI: c_uint = 0x0008;
pub const B43legacy_BFL_EXTLNA: c_uint = 0x1000;
// GPIO register offset, in both ChipCommon and PCI core.
pub const B43legacy_GPIO_CONTROL: c_uint = 0x6c;
// SHM Routing
pub const B43legacy_SHM_SHARED: c_uint = 0x0001;
pub const B43legacy_SHM_WIRELESS: c_uint = 0x0002;
pub const B43legacy_SHM_HW: c_uint = 0x0004;
pub const B43legacy_SHM_UCODE: c_uint = 0x0300;
// SHM Routing modifiers
pub const B43legacy_SHM_AUTOINC_R: c_uint = 0x0200 /* Read Auto-increment */;
pub const B43legacy_SHM_AUTOINC_W: c_uint = 0x0100 /* Write Auto-increment */;

// Misc SHM_SHARED offsets
pub const B43legacy_SHM_SH_WLCOREREV: c_uint = 0x0016 /* 802.11 core revision */;
pub const B43legacy_SHM_SH_HOSTFLO: c_uint = 0x005E /* Hostflags ucode opts (low) */;
pub const B43legacy_SHM_SH_HOSTFHI: c_uint = 0x0060 /* Hostflags ucode opts (high) */;
// SHM_SHARED crypto engine
pub const B43legacy_SHM_SH_KEYIDXBLOCK: c_uint = 0x05D4 /* Key index/algorithm block */;
// SHM_SHARED beacon/AP variables
pub const B43legacy_SHM_SH_DTIMP: c_uint = 0x0012 /* DTIM period */;
pub const B43legacy_SHM_SH_BTL0: c_uint = 0x0018 /* Beacon template length 0 */;
pub const B43legacy_SHM_SH_BTL1: c_uint = 0x001A /* Beacon template length 1 */;
pub const B43legacy_SHM_SH_BTSFOFF: c_uint = 0x001C /* Beacon TSF offset */;
pub const B43legacy_SHM_SH_TIMPOS: c_uint = 0x001E /* TIM position in beacon */;
pub const B43legacy_SHM_SH_BEACPHYCTL: c_uint = 0x0054 /* Beacon PHY TX control word */;
// SHM_SHARED ACK/CTS control
pub const B43legacy_SHM_SH_ACKCTSPHYCTL: c_uint = 0x0022 /* ACK/CTS PHY control word */;
// SHM_SHARED probe response variables
pub const B43legacy_SHM_SH_PRTLEN: c_uint = 0x004A /* Probe Response template length */;
pub const B43legacy_SHM_SH_PRMAXTIME: c_uint = 0x0074 /* Probe Response max time */;
pub const B43legacy_SHM_SH_PRPHYCTL: c_uint = 0x0188 /* Probe Resp PHY TX control */;
// SHM_SHARED rate tables
pub const B43legacy_SHM_SH_OFDMDIRECT: c_uint = 0x0480 /* Pointer to OFDM direct map */;
pub const B43legacy_SHM_SH_OFDMBASIC: c_uint = 0x04A0 /* Pointer to OFDM basic rate map */;
pub const B43legacy_SHM_SH_CCKDIRECT: c_uint = 0x04C0 /* Pointer to CCK direct map */;
pub const B43legacy_SHM_SH_CCKBASIC: c_uint = 0x04E0 /* Pointer to CCK basic rate map */;
// SHM_SHARED microcode soft registers
pub const B43legacy_SHM_SH_UCODEREV: c_uint = 0x0000 /* Microcode revision */;
pub const B43legacy_SHM_SH_UCODEPATCH: c_uint = 0x0002 /* Microcode patchlevel */;
pub const B43legacy_SHM_SH_UCODEDATE: c_uint = 0x0004 /* Microcode date */;
pub const B43legacy_SHM_SH_UCODETIME: c_uint = 0x0006 /* Microcode time */;
pub const B43legacy_SHM_SH_SPUWKUP: c_uint = 0x0094 /* pre-wakeup for synth PU in us */;
pub const B43legacy_SHM_SH_PRETBTT: c_uint = 0x0096 /* pre-TBTT in us */;
pub const B43legacy_UCODEFLAGS_OFFSET: c_uint = 0x005E;
// Hardware Radio Enable masks

// HostFlags. See b43legacy_hf_read/write()
pub const B43legacy_HF_SYMW: c_uint = 0x00000002 /* G-PHY SYM workaround */;
pub const B43legacy_HF_GDCW: c_uint = 0x00000020 /* G-PHY DV cancel filter */;
pub const B43legacy_HF_OFDMPABOOST: c_uint = 0x00000040 /* Enable PA boost OFDM */;
pub const B43legacy_HF_EDCF: c_uint = 0x00000100 /* on if WME/MAC suspended */;
// MacFilter offsets.
pub const B43legacy_MACFILTER_SELF: c_uint = 0x0000;
pub const B43legacy_MACFILTER_BSSID: c_uint = 0x0003;
pub const B43legacy_MACFILTER_MAC: c_uint = 0x0010;
// PHYVersioning
pub const B43legacy_PHYTYPE_B: c_uint = 0x01;
pub const B43legacy_PHYTYPE_G: c_uint = 0x02;
// PHYRegisters
pub const B43legacy_PHY_G_LO_CONTROL: c_uint = 0x0810;
pub const B43legacy_PHY_ILT_G_CTRL: c_uint = 0x0472;
pub const B43legacy_PHY_ILT_G_DATA1: c_uint = 0x0473;
pub const B43legacy_PHY_ILT_G_DATA2: c_uint = 0x0474;
pub const B43legacy_PHY_G_PCTL: c_uint = 0x0029;
pub const B43legacy_PHY_RADIO_BITFIELD: c_uint = 0x0401;
pub const B43legacy_PHY_G_CRS: c_uint = 0x0429;
pub const B43legacy_PHY_NRSSILT_CTRL: c_uint = 0x0803;
pub const B43legacy_PHY_NRSSILT_DATA: c_uint = 0x0804;
// RadioRegisters
pub const B43legacy_RADIOCTL_ID: c_uint = 0x01;
// MAC Control bitfield
pub const B43legacy_MACCTL_ENABLED: c_uint = 0x00000001 /* MAC Enabled */;
pub const B43legacy_MACCTL_PSM_RUN: c_uint = 0x00000002 /* Run Microcode */;
pub const B43legacy_MACCTL_PSM_JMP0: c_uint = 0x00000004 /* Microcode jump to 0 */;
pub const B43legacy_MACCTL_SHM_ENABLED: c_uint = 0x00000100 /* SHM Enabled */;
pub const B43legacy_MACCTL_IHR_ENABLED: c_uint = 0x00000400 /* IHR Region Enabled */;
pub const B43legacy_MACCTL_BE: c_uint = 0x00010000 /* Big Endian mode */;
pub const B43legacy_MACCTL_INFRA: c_uint = 0x00020000 /* Infrastructure mode */;
pub const B43legacy_MACCTL_AP: c_uint = 0x00040000 /* AccessPoint mode */;
pub const B43legacy_MACCTL_RADIOLOCK: c_uint = 0x00080000 /* Radio lock */;
pub const B43legacy_MACCTL_BEACPROMISC: c_uint = 0x00100000 /* Beacon Promiscuous */;
pub const B43legacy_MACCTL_KEEP_BADPLCP: c_uint = 0x00200000 /* Keep bad PLCP frames */;
pub const B43legacy_MACCTL_KEEP_CTL: c_uint = 0x00400000 /* Keep control frames */;
pub const B43legacy_MACCTL_KEEP_BAD: c_uint = 0x00800000 /* Keep bad frames (FCS) */;
pub const B43legacy_MACCTL_PROMISC: c_uint = 0x01000000 /* Promiscuous mode */;
pub const B43legacy_MACCTL_HWPS: c_uint = 0x02000000 /* Hardware Power Saving */;
pub const B43legacy_MACCTL_AWAKE: c_uint = 0x04000000 /* Device is awake */;
pub const B43legacy_MACCTL_TBTTHOLD: c_uint = 0x10000000 /* TBTT Hold */;
pub const B43legacy_MACCTL_GMODE: c_uint = 0x80000000 /* G Mode */;
// MAC Command bitfield
pub const B43legacy_MACCMD_BEACON0_VALID: c_uint = 0x00000001 /* Beacon 0 in template RAM is busy/valid */;
pub const B43legacy_MACCMD_BEACON1_VALID: c_uint = 0x00000002 /* Beacon 1 in template RAM is busy/valid */;
pub const B43legacy_MACCMD_DFQ_VALID: c_uint = 0x00000004 /* Directed frame queue valid (IBSS PS mode, ATIM) */;
pub const B43legacy_MACCMD_CCA: c_uint = 0x00000008 /* Clear channel assessment */;
pub const B43legacy_MACCMD_BGNOISE: c_uint = 0x00000010 /* Background noise */;
// 802.11 core specific TM State Low flags
pub const B43legacy_TMSLOW_GMODE: c_uint = 0x20000000 /* G Mode Enable */;
pub const B43legacy_TMSLOW_PLLREFSEL: c_uint = 0x00200000 /* PLL Freq Ref Select */;
pub const B43legacy_TMSLOW_MACPHYCLKEN: c_uint = 0x00100000 /* MAC PHY Clock Ctrl Enbl */;
pub const B43legacy_TMSLOW_PHYRESET: c_uint = 0x00080000 /* PHY Reset */;
pub const B43legacy_TMSLOW_PHYCLKEN: c_uint = 0x00040000 /* PHY Clock Enable */;
// 802.11 core specific TM State High flags
pub const B43legacy_TMSHIGH_FCLOCK: c_uint = 0x00040000 /* Fast Clock Available */;
pub const B43legacy_TMSHIGH_GPHY: c_uint = 0x00010000 /* G-PHY avail (rev >= 5) */;
pub const B43legacy_UCODEFLAG_AUTODIV: c_uint = 0x0001;
// Generic-Interrupt reasons.
pub const B43legacy_IRQ_MAC_SUSPENDED: c_uint = 0x00000001;
pub const B43legacy_IRQ_BEACON: c_uint = 0x00000002;
pub const B43legacy_IRQ_TBTT_INDI: c_uint = 0x00000004 /* Target Beacon Transmit Time */;
pub const B43legacy_IRQ_BEACON_TX_OK: c_uint = 0x00000008;
pub const B43legacy_IRQ_BEACON_CANCEL: c_uint = 0x00000010;
pub const B43legacy_IRQ_ATIM_END: c_uint = 0x00000020;
pub const B43legacy_IRQ_PMQ: c_uint = 0x00000040;
pub const B43legacy_IRQ_PIO_WORKAROUND: c_uint = 0x00000100;
pub const B43legacy_IRQ_MAC_TXERR: c_uint = 0x00000200;
pub const B43legacy_IRQ_PHY_TXERR: c_uint = 0x00000800;
pub const B43legacy_IRQ_PMEVENT: c_uint = 0x00001000;
pub const B43legacy_IRQ_TIMER0: c_uint = 0x00002000;
pub const B43legacy_IRQ_TIMER1: c_uint = 0x00004000;
pub const B43legacy_IRQ_DMA: c_uint = 0x00008000;
pub const B43legacy_IRQ_TXFIFO_FLUSH_OK: c_uint = 0x00010000;
pub const B43legacy_IRQ_CCA_MEASURE_OK: c_uint = 0x00020000;
pub const B43legacy_IRQ_NOISESAMPLE_OK: c_uint = 0x00040000;
pub const B43legacy_IRQ_UCODE_DEBUG: c_uint = 0x08000000;
pub const B43legacy_IRQ_RFKILL: c_uint = 0x10000000;
pub const B43legacy_IRQ_TX_OK: c_uint = 0x20000000;
pub const B43legacy_IRQ_PHY_G_CHANGED: c_uint = 0x40000000;
pub const B43legacy_IRQ_TIMEOUT: c_uint = 0x80000000;
pub const B43legacy_IRQ_ALL: c_uint = 0xFFFFFFFF;

// Device specific rate values.
// The actual values defined here are (rate_in_mbps * 2).
// Some code depends on this. Don't change it.
pub const B43legacy_CCK_RATE_1MB: c_int = 2;
pub const B43legacy_CCK_RATE_2MB: c_int = 4;
pub const B43legacy_CCK_RATE_5MB: c_int = 11;
pub const B43legacy_CCK_RATE_11MB: c_int = 22;
pub const B43legacy_OFDM_RATE_6MB: c_int = 12;
pub const B43legacy_OFDM_RATE_9MB: c_int = 18;
pub const B43legacy_OFDM_RATE_12MB: c_int = 24;
pub const B43legacy_OFDM_RATE_18MB: c_int = 36;
pub const B43legacy_OFDM_RATE_24MB: c_int = 48;
pub const B43legacy_OFDM_RATE_36MB: c_int = 72;
pub const B43legacy_OFDM_RATE_48MB: c_int = 96;
pub const B43legacy_OFDM_RATE_54MB: c_int = 108;
// Convert a b43legacy rate value to a rate in 100kbps

pub const B43legacy_DEFAULT_SHORT_RETRY_LIMIT: c_int = 7;
pub const B43legacy_DEFAULT_LONG_RETRY_LIMIT: c_int = 4;
pub const B43legacy_PHY_TX_BADNESS_LIMIT: c_int = 1000;
// Max size of a security key
pub const B43legacy_SEC_KEYSIZE: c_int = 16;
// Security algorithms.
// Core Information Registers
pub const B43legacy_CIR_BASE: c_uint = 0xf00;

// sbtmstatehigh state flags
pub const B43legacy_SBTMSTATEHIGH_SERROR: c_uint = 0x00000001;
pub const B43legacy_SBTMSTATEHIGH_BUSY: c_uint = 0x00000004;
pub const B43legacy_SBTMSTATEHIGH_TIMEOUT: c_uint = 0x00000020;
pub const B43legacy_SBTMSTATEHIGH_G_PHY_AVAIL: c_uint = 0x00010000;
pub const B43legacy_SBTMSTATEHIGH_COREFLAGS: c_uint = 0x1FFF0000;
pub const B43legacy_SBTMSTATEHIGH_DMA64BIT: c_uint = 0x10000000;
pub const B43legacy_SBTMSTATEHIGH_GATEDCLK: c_uint = 0x20000000;
pub const B43legacy_SBTMSTATEHIGH_BISTFAILED: c_uint = 0x40000000;
pub const B43legacy_SBTMSTATEHIGH_BISTCOMPLETE: c_uint = 0x80000000;
// sbimstate flags
pub const B43legacy_SBIMSTATE_IB_ERROR: c_uint = 0x20000;
pub const B43legacy_SBIMSTATE_TIMEOUT: c_uint = 0x40000;

// This will evaluate the argument even if debugging is disabled.

// The firmware file header

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_fw_header {
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
pub const B43legacy_IV_OFFSET_MASK: c_uint = 0x7FFF;
pub const B43legacy_IV_32BIT: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_iv {
    pub offset_size: __be16,
    pub d16: __be16,
    pub d32: __be32,
    pub data: } __packed,
    pub __packed: },

// Value pair to measure the LocalOscillator.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_lopair {
    pub low: i8,
    pub high: i8,
    pub used:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_phy {
// Possible PHYMODEs on this PHY
    pub possible_phymodes: u8,
// GMODE bit enabled in MACCTL?
    pub gmode: bool,
// Analog Type
    pub analog: u8,
// B43legacy_PHYTYPE_
    pub type: u8,
// PHY revision number.
    pub rev: u8,
    pub antenna_diversity: u16,
    pub savedpctlreg: u16,
// Radio versioning
    pub /: *mut *mut u16 radio_manuf; / Radio manufacturer,
    pub /: *mut *mut u16 radio_ver; / Radio version,
    pub calibrated:1: u8,
    pub /: *mut *mut u8 radio_rev; / Radio revision,
    pub /: *mut *mut bool dyn_tssi_tbl; / tssi2dbm is kmalloc()ed.,
// ACI (adjacent channel interference) flags.
    pub aci_enable: bool,
    pub aci_wlan_automatic: bool,
    pub aci_hw_rssi: bool,
// Radio switched on/off
    pub radio_on: bool,
// Values saved when turning the radio off.
// They are needed when turning it on again.
    pub valid: bool,
    pub rfover: u16,
    pub rfoverval: u16,
    pub radio_off_context: },
    pub minlowsig: [u16; 2],
    pub minlowsigpos: [u16; 2],
// LO Measurement Data.
// Use b43legacy_get_lopair() to get a value.
//
    pub _lo_pairs: *mut b43legacy_lopair,
// TSSI to dBm table in use
    pub tssi2dbm: *const i8,
// idle TSSI value
    pub idle_tssi: i8,
// Target idle TSSI
    pub tgt_idle_tssi: c_int,
// Current idle TSSI
    pub cur_idle_tssi: c_int,
// LocalOscillator control values.
    pub lo_control: *mut b43legacy_txpower_lo_control,
// Values from b43legacy_calc_loopback_gain()
    pub /: *mut *mut s16 max_lb_gain; / Maximum Loopback gain in hdB,
    pub /: *mut *mut s16 trsw_rx_gain; / TRSW RX gain in hdB,
    pub /: *mut *mut s16 lna_lod_gain; / LNA lod,
    pub /: *mut *mut s16 lna_gain; / LNA,
    pub /: *mut *mut s16 pga_gain; / PGA,
// Desired TX power level (in dBm). This is set by the user and
// adjusted in b43legacy_phy_xmitpower().
    pub power_level: u8,
// Values from b43legacy_calc_loopback_gain()
    pub loopback_gain: [u16; 2],
// TX Power control values.
// B/G PHY
// Current Radio Attenuation for TXpower recalculation.
    pub rfatt: u16,
// Current Baseband Attenuation for TXpower recalculation.
    pub bbatt: u16,
// Current TXpower control value for TXpower recalculation.
    pub txctl1: u16,
    pub txctl2: u16,
}

// A PHY
// Current Interference Mitigation mode
// Stack of saved values from the Interference Mitigation code.
// Each value in the stack is laid out as follows:
// bit 0-11:  offset
// bit 12-15: register ID
// bit 16-32: value
// register ID is: 0x1 PHY, 0x2 Radio, 0x3 ILT
//
pub const B43legacy_INTERFSTACK_SIZE: c_int = 26;
// Saved values from the NRSSI Slope calculation
// In memory nrssi lookup table.
// current channel
// PHY TX errors counter.

// Manual TX-power control enabled?
// PHY registers locked by b43legacy_phy_lock()?

// Data structures for DMA transmission, per 80211 core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dma {
    pub tx_ring0: *mut b43legacy_dmaring,
    pub tx_ring1: *mut b43legacy_dmaring,
    pub tx_ring2: *mut b43legacy_dmaring,
    pub tx_ring3: *mut b43legacy_dmaring,
    pub tx_ring4: *mut b43legacy_dmaring,
    pub tx_ring5: *mut b43legacy_dmaring,
    pub rx_ring0: *mut b43legacy_dmaring,
    pub /: *mut *mut *mut b43legacy_dmaring rx_ring3; / only on core.rev < 5,
    pub /: *mut *mut u32 translation; / Routing bits,
}

// Data structures for PIO transmission, per 80211 core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_pio {
    pub queue0: *mut b43legacy_pioqueue,
    pub queue1: *mut b43legacy_pioqueue,
    pub queue2: *mut b43legacy_pioqueue,
    pub queue3: *mut b43legacy_pioqueue,
}

// Context information for a noise calculation (Link Quality).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_noise_calculation {
    pub channel_at_start: u8,
    pub calculation_running: bool,
    pub nr_samples: u8,
    pub samples: [i8; 8][4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_stats {
    pub link_noise: u8,
// Store the last TX/RX times here for updating the leds.
    pub last_tx: c_ulong,
    pub last_rx: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_key {
    pub keyconf: *mut c_void,
    pub enabled: bool,
    pub algorithm: u8,
}

pub const B43legacy_QOS_QUEUE_NUM: c_int = 4;
// QOS parameters for a queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_qos_params {
// The QOS parameters
    pub p: ieee80211_tx_queue_params,
}

// Data structure for the WLAN parts (802.11 cores) of the b43legacy chip.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_wl {
// Pointer to the active wireless device on this chip
    pub current_dev: *mut b43legacy_wldev,
// Pointer to the ieee80211 hardware data structure
    pub hw: *mut ieee80211_hw,
    pub /: *mut *mut spinlock_t irq_lock; / locks IRQ,
    pub /: *mut *mut mutex mutex; / locks wireless core state,
    pub /: *mut *mut spinlock_t leds_lock; / lock for leds,
// firmware loading work
    pub firmware_load: work_struct,
// We can only have one operating interface (802.11 core)
// at a time. General information about this interface follows.
//
    pub vif: *mut ieee80211_vif,
// MAC address (can be NULL).
    pub mac_addr: [u8; ETH_ALEN],
// Current BSSID (can be NULL).
    pub bssid: [u8; ETH_ALEN],
// Interface type. (IEEE80211_IF_TYPE_XXX)
    pub if_type: c_int,
// Is the card operating in AP, STA or IBSS mode?
    pub operating: bool,
// filter flags
    pub filter_flags: c_uint,
// Stats about the wireless interface
    pub ieee_stats: ieee80211_low_level_stats,

    pub rng: hwrng,
    pub rng_initialized: u8,
    pub 1]: char rng_name[30 +,

// List of all wireless devices on this chip
    pub devlist: list_head,
    pub nr_devs: u8,
    pub radiotap_enabled: bool,
    pub radio_enabled: bool,
// The beacon we are currently using (AP or IBSS mode).
// This beacon stuff is protected by the irq_lock.
    pub current_beacon: *mut sk_buff,
    pub beacon0_uploaded: bool,
    pub beacon1_uploaded: bool,
    pub /: *mut *mut bool beacon_templates_virgin; / Never wrote the templates?,
    pub beacon_update_trigger: work_struct,
// The current QOS parameters for the 4 queues.
    pub qos_params: [b43legacy_qos_params; B43legacy_QOS_QUEUE_NUM],
// Packet transmit work
    pub tx_work: work_struct,
// Queue of packets to be transmitted.
    pub tx_queue: [sk_buff_head; B43legacy_QOS_QUEUE_NUM],
// Flag that implement the queues stopping.
    pub tx_queue_stopped: [bool; B43legacy_QOS_QUEUE_NUM],
}

// Pointers to the firmware data and meta information about it.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_firmware {
// Microcode
    pub ucode: *const firmware,
// PCM code
    pub pcm: *const firmware,
// Initial MMIO values for the firmware
    pub initvals: *const firmware,
// Initial MMIO values for the firmware, band-specific
    pub initvals_band: *const firmware,
// Firmware revision
    pub rev: u16,
// Firmware patchlevel
    pub patch: u16,
}

// Device (802.11 core) initialization status.

// *** ---   HOW LOCKING WORKS IN B43legacy   ---
//
// You should always acquire both, wl->mutex and wl->irq_lock unless:
// - You don't need to acquire wl->irq_lock, if the interface is stopped.
// - You don't need to acquire wl->mutex in the IRQ handler, IRQ tasklet
// and packet TX path (and _ONLY_ there.)
//
// Data structure for one wireless device (802.11 core)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_wldev {
    pub dev: *mut ssb_device,
    pub wl: *mut b43legacy_wl,
// The device initialization status.
// Use b43legacy_status() to query.
    pub __init_status: core::sync::atomic::AtomicI32,
// Saved init status for handling suspend.
    pub suspend_init_status: c_int,
    pub /: *mut *mut bool __using_pio; / Using pio rather than dma.,
    pub /: *mut *mut bool bad_frames_preempt;/ Use "Bad Frames Preemption".,
    pub /: *mut *mut bool dfq_valid; / Directed frame queue valid (IBSS PS mode, ATIM).,
    pub /: *mut *mut bool short_preamble; / TRUE if using short preamble.,
    pub /: *mut *mut bool radio_hw_enable; / State of radio hardware enable bit.,
// PHY/Radio device.
    pub phy: b43legacy_phy,
// DMA engines.
    pub dma: b43legacy_dma,
// PIO engines.
    pub pio: b43legacy_pio,
}

// Various statistics about the physical device.
// The device LEDs.
// Reason code of the last interrupt.
// The currently active generic-interrupt mask.
// Link Quality calculation context.
// if > 0 MAC is suspended. if == 0 MAC is enabled.
// Interrupt Service Routine tasklet (bottom-half)
// Periodic tasks
// encryption/decryption
// Firmware data
// completion struct for firmware loading
// Devicelist in struct b43legacy_wl (all 802.11 cores)
// Debugging stuff follows.

// Helper function, which returns a boolean.
// TRUE, if PIO is used; FALSE, if DMA is used.
//

extern "C" {
    pub fn ssb_get_drvdata(_arg: ssb_dev) -> return;
}
// Is the device operating in a specified mode (IEEE80211_IF_TYPE_XXX).
extern "C" {
    pub fn ssb_read16(_arg: dev->dev, _arg: offset) -> return;
}
extern "C" {
    pub fn ssb_read32(_arg: dev->dev, _arg: offset) -> return;
}
// Message printing
extern "C" {
    pub fn b43legacyinfo(wl: *mut b43legacy_wl, fmt: *const c_char, ...);
}
extern "C" {
    pub fn b43legacyerr(wl: *mut b43legacy_wl, fmt: *const c_char, ...);
}
extern "C" {
    pub fn b43legacywarn(wl: *mut b43legacy_wl, fmt: *const c_char, ...);
}

extern "C" {
    pub fn b43legacydbg(wl: *mut b43legacy_wl, fmt: *const c_char, ...);
}

// Macros for printing a value in Q5.2 format

