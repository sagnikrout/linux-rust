//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/admtek/adm8211.h
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
// ADM8211 Registers
// CR32 (SIG) signature
pub const ADM8211_SIG1: c_uint = 0x82011317 /* ADM8211A */;
pub const ADM8211_SIG2: c_uint = 0x82111317 /* ADM8211B/ADM8211C */;

// CSR (Host Control and Status Registers)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_csr {
    pub /: *mut *mut __le32 PAR; / 0x00 CSR0,
    pub /: *mut *mut __le32 FRCTL; / 0x04 CSR0A,
    pub /: *mut *mut __le32 TDR; / 0x08 CSR1,
    pub /: *mut *mut __le32 WTDP; / 0x0C CSR1A,
    pub /: *mut *mut __le32 RDR; / 0x10 CSR2,
    pub /: *mut *mut __le32 WRDP; / 0x14 CSR2A,
    pub /: *mut *mut __le32 RDB; / 0x18 CSR3,
    pub /: *mut *mut __le32 TDBH; / 0x1C CSR3A,
    pub /: *mut *mut __le32 TDBD; / 0x20 CSR4,
    pub /: *mut *mut __le32 TDBP; / 0x24 CSR4A,
    pub /: *mut *mut __le32 STSR; / 0x28 CSR5,
    pub /: *mut *mut __le32 TDBB; / 0x2C CSR5A,
    pub /: *mut *mut __le32 NAR; / 0x30 CSR6,
    pub /: *mut *mut __le32 CSR6A; / reserved,
    pub /: *mut *mut __le32 IER; / 0x38 CSR7,
    pub /: *mut *mut __le32 TKIPSCEP; / 0x3C CSR7A,
    pub /: *mut *mut __le32 LPC; / 0x40 CSR8,
    pub /: *mut *mut __le32 CSR_TEST1; / 0x44 CSR8A,
    pub /: *mut *mut __le32 SPR; / 0x48 CSR9,
    pub /: *mut *mut __le32 CSR_TEST0; / 0x4C CSR9A,
    pub /: *mut *mut __le32 WCSR; / 0x50 CSR10,
    pub /: *mut *mut __le32 WPDR; / 0x54 CSR10A,
    pub /: *mut *mut __le32 GPTMR; / 0x58 CSR11,
    pub /: *mut *mut __le32 GPIO; / 0x5C CSR11A,
    pub /: *mut *mut __le32 BBPCTL; / 0x60 CSR12,
    pub /: *mut *mut __le32 SYNCTL; / 0x64 CSR12A,
    pub /: *mut *mut __le32 PLCPHD; / 0x68 CSR13,
    pub /: *mut *mut __le32 MMIWA; / 0x6C CSR13A,
    pub /: *mut *mut __le32 MMIRD0; / 0x70 CSR14,
    pub /: *mut *mut __le32 MMIRD1; / 0x74 CSR14A,
    pub /: *mut *mut __le32 TXBR; / 0x78 CSR15,
    pub /: *mut *mut __le32 SYNDATA; / 0x7C CSR15A,
    pub /: *mut *mut __le32 ALCS; / 0x80 CSR16,
    pub /: *mut *mut __le32 TOFS2; / 0x84 CSR17,
    pub /: *mut *mut __le32 CMDR; / 0x88 CSR18,
    pub /: *mut *mut __le32 PCIC; / 0x8C CSR19,
    pub /: *mut *mut __le32 PMCSR; / 0x90 CSR20,
    pub /: *mut *mut __le32 PAR0; / 0x94 CSR21,
    pub /: *mut *mut __le32 PAR1; / 0x98 CSR22,
    pub /: *mut *mut __le32 MAR0; / 0x9C CSR23,
    pub /: *mut *mut __le32 MAR1; / 0xA0 CSR24,
    pub /: *mut *mut __le32 ATIMDA0; / 0xA4 CSR25,
    pub /: *mut *mut __le32 ABDA1; / 0xA8 CSR26,
    pub /: *mut *mut __le32 BSSID0; / 0xAC CSR27,
    pub /: *mut *mut __le32 TXLMT; / 0xB0 CSR28,
    pub /: *mut *mut __le32 MIBCNT; / 0xB4 CSR29,
    pub /: *mut *mut __le32 BCNT; / 0xB8 CSR30,
    pub /: *mut *mut __le32 TSFTH; / 0xBC CSR31,
    pub /: *mut *mut __le32 TSC; / 0xC0 CSR32,
    pub /: *mut *mut __le32 SYNRF; / 0xC4 CSR33,
    pub /: *mut *mut __le32 BPLI; / 0xC8 CSR34,
    pub /: *mut *mut __le32 CAP0; / 0xCC CSR35,
    pub /: *mut *mut __le32 CAP1; / 0xD0 CSR36,
    pub /: *mut *mut __le32 RMD; / 0xD4 CSR37,
    pub /: *mut *mut __le32 CFPP; / 0xD8 CSR38,
    pub /: *mut *mut __le32 TOFS0; / 0xDC CSR39,
    pub /: *mut *mut __le32 TOFS1; / 0xE0 CSR40,
    pub /: *mut *mut __le32 IFST; / 0xE4 CSR41,
    pub /: *mut *mut __le32 RSPT; / 0xE8 CSR42,
    pub /: *mut *mut __le32 TSFTL; / 0xEC CSR43,
    pub /: *mut *mut __le32 WEPCTL; / 0xF0 CSR44,
    pub /: *mut *mut __le32 WESK; / 0xF4 CSR45,
    pub /: *mut *mut __le32 WEPCNT; / 0xF8 CSR46,
    pub /: *mut *mut __le32 MACTEST; / 0xFC CSR47,
    pub /: *mut *mut __le32 FER; / 0x100,
    pub /: *mut *mut __le32 FEMR; / 0x104,
    pub /: *mut *mut __le32 FPSR; / 0x108,
    pub /: *mut *mut __le32 FFER; / 0x10C,
    pub __packed: },
// CSR0 - PAR (PCI Address Register)

pub const ADM8211_PAR_PBL: c_uint = 0x00003f00;

pub const ADM8211_PAR_DSL: c_uint = 0x0000007c;

// CSR1 - FRCTL (Frame Control Register)

pub const ADM8211_FRCTL_AID: c_uint = 0x0000ffff;
pub const ADM8211_FRCTL_AID_ON: c_uint = 0x0000c000;
// CSR5 - STSR (Status Register)

// CSR6 - NAR (Network Access Register)

    pub ADM8211_NAR_ST));\: ~(ADM8211_NAR_SR |,
    pub \: ADM8211_CSR_READ(NAR);,
    pub \: msleep(20);,

    pub \: ADM8211_CSR_WRITE(NAR, priv->nar & ~ADM8211_NAR_SR);,
    pub \: ADM8211_CSR_READ(NAR);,
    pub \: mdelay(20);,

    pub \: ADM8211_CSR_WRITE(NAR, priv->nar);,
// CSR7 - IER (Interrupt Enable Register)

// CSR9 - SPR (Serial Port Register)

// CSR9A - CSR_TEST0

// CSR10 - WCSR (Wake-up Control/Status Register)

// CSR11A - GPIO

pub const ADM8211_CSR_GPIO_IN: c_uint = 0x0000003f;
// CSR12 - BBPCTL (BBP Control port)

pub const ADM8211_BBPCTL_TYPE: c_uint = 0x001c0000;

pub const ADM8211_BBPCTL_ADDR: c_uint = 0x0000ff00;
pub const ADM8211_BBPCTL_DATA: c_uint = 0x000000ff;
// CSR12A - SYNCTL (Synthesizer Control port)

// SYNCTL 21:0 Data (Si4126: 18-bit data, 4-bit address)
// CSR18 - CMDR (Command Register)

// CSR33 - SYNRF (SYNRF direct control)

// CSR44 - WEPCTL (WEP Control)

// CSR45 - WESK (Data Entry for Share/Individual Key)

// FER (Function Event Register)

// Si4126 RF Synthesizer - Control Registers
pub const SI4126_MAIN_CONF: c_int = 0;
pub const SI4126_PHASE_DET_GAIN: c_int = 1;
pub const SI4126_POWERDOWN: c_int = 2;

pub const SI4126_RF2_N_DIV: c_int = 4;
pub const SI4126_IF_N_DIV: c_int = 5;

pub const SI4126_RF2_R_DIV: c_int = 7;
pub const SI4126_IF_R_DIV: c_int = 8;
// Main Configuration

// Powerdown

// RF3000 BBP - Control Port Registers
// 0x00 - reserved
pub const RF3000_MODEM_CTRL__RX_STATUS: c_uint = 0x01;
pub const RF3000_CCA_CTRL: c_uint = 0x02;
pub const RF3000_DIVERSITY__RSSI: c_uint = 0x03;
pub const RF3000_RX_SIGNAL_FIELD: c_uint = 0x04;
pub const RF3000_RX_LEN_MSB: c_uint = 0x05;
pub const RF3000_RX_LEN_LSB: c_uint = 0x06;
pub const RF3000_RX_SERVICE_FIELD: c_uint = 0x07;
pub const RF3000_TX_VAR_GAIN__TX_LEN_EXT: c_uint = 0x11;
pub const RF3000_TX_LEN_MSB: c_uint = 0x12;
pub const RF3000_TX_LEN_LSB: c_uint = 0x13;
pub const RF3000_LOW_GAIN_CALIB: c_uint = 0x14;
pub const RF3000_HIGH_GAIN_CALIB: c_uint = 0x15;
// ADM8211 revisions
pub const ADM8211_REV_AB: c_uint = 0x11;
pub const ADM8211_REV_AF: c_uint = 0x15;
pub const ADM8211_REV_BA: c_uint = 0x20;
pub const ADM8211_REV_CA: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_desc {
    pub status: __le32,
    pub length: __le32,
    pub buffer1: __le32,
    pub buffer2: __le32,
}

// SRAM offsets

pub const ADM8211_SRAM_INDIV_KEY: c_uint = 0x0000;
pub const ADM8211_SRAM_A_SHARE_KEY: c_uint = 0x0160;
pub const ADM8211_SRAM_B_SHARE_KEY: c_uint = 0x00c0;
pub const ADM8211_SRAM_A_SSID: c_uint = 0x0180;
pub const ADM8211_SRAM_B_SSID: c_uint = 0x00d4;

pub const ADM8211_SRAM_A_SUPP_RATE: c_uint = 0x0191;
pub const ADM8211_SRAM_B_SUPP_RATE: c_uint = 0x00dd;

pub const ADM8211_SRAM_A_SIZE: c_uint = 0x0200;
pub const ADM8211_SRAM_B_SIZE: c_uint = 0x01c0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_rx_ring_info {
    pub skb: *mut sk_buff,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_tx_ring_info {
    pub skb: *mut sk_buff,
    pub mapping: dma_addr_t,
    pub hdrlen: usize,
}

pub const PLCP_SIGNAL_1M: c_uint = 0x0a;
pub const PLCP_SIGNAL_2M: c_uint = 0x14;
pub const PLCP_SIGNAL_5M5: c_uint = 0x37;
pub const PLCP_SIGNAL_11M: c_uint = 0x6e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_tx_hdr {
    pub da: [u8; 6],
    pub /: *mut *mut u8 signal; / PLCP signal / TX rate in 100 Kbps,
    pub service: u8,
    pub frame_body_size: __le16,
    pub frame_control: __le16,
    pub plcp_frag_tail_len: __le16,
    pub plcp_frag_head_len: __le16,
    pub dur_frag_tail: __le16,
    pub dur_frag_head: __le16,
    pub addr4: [u8; 6],
    pub header_control: __le16,
    pub frag: __le16,
    pub reserved_0: u8,
    pub retry_limit: u8,
    pub wep2key0: u32,
    pub wep2key1: u32,
    pub wep2key2: u32,
    pub wep2key3: u32,
    pub keyid: u8,
    pub huh??: u8 entry_control; //,
    pub reserved_1: u16,
    pub reserved_2: u32,
    pub __packed: },
pub const RX_COPY_BREAK: c_int = 128;
pub const RX_PKT_SIZE: c_int = 2500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_eeprom {
    pub /: *mut *mut __le16 signature; / 0x00,
    pub /: *mut *mut u8 major_version; / 0x02,
    pub /: *mut *mut u8 minor_version; / 0x03,
    pub /: *mut *mut u8 reserved_1[4]; / 0x04,
    pub /: *mut *mut u8 hwaddr[6]; / 0x08,
    pub /: *mut *mut u8 reserved_2[8]; / 0x1E,
    pub /: *mut *mut __le16 cr49; / 0x16,
    pub /: *mut *mut u8 cr03; / 0x18,
    pub /: *mut *mut u8 cr28; / 0x19,
    pub /: *mut *mut u8 cr29; / 0x1A,
    pub /: *mut *mut u8 country_code; / 0x1B,
// specific bbp types
pub const ADM8211_BBP_RFMD3000: c_uint = 0x00;
pub const ADM8211_BBP_RFMD3002: c_uint = 0x01;
pub const ADM8211_BBP_ADM8011: c_uint = 0x04;
    pub /: *mut *mut u8 specific_bbptype; / 0x1C,
    pub /: *mut *mut u8 specific_rftype; / 0x1D,
    pub /: *mut *mut u8 reserved_3[2]; / 0x1E,
    pub /: *mut *mut __le16 device_id; / 0x20,
    pub /: *mut *mut __le16 vendor_id; / 0x22,
    pub /: *mut *mut __le16 subsystem_id; / 0x24,
    pub /: *mut *mut __le16 subsystem_vendor_id; / 0x26,
    pub /: *mut *mut u8 maxlat; / 0x28,
    pub /: *mut *mut u8 mingnt; / 0x29,
    pub /: *mut *mut __le16 cis_pointer_low; / 0x2A,
    pub /: *mut *mut __le16 cis_pointer_high; / 0x2C,
    pub /: *mut *mut __le16 csr18; / 0x2E,
    pub /: *mut *mut u8 reserved_4[16]; / 0x30,
    pub /: *mut *mut u8 d1_pwrdara; / 0x40,
    pub /: *mut *mut u8 d0_pwrdara; / 0x41,
    pub /: *mut *mut u8 d3_pwrdara; / 0x42,
    pub /: *mut *mut u8 d2_pwrdara; / 0x43,
    pub /: *mut *mut u8 antenna_power[14]; / 0x44,
    pub /: *mut *mut __le16 cis_wordcnt; / 0x52,
    pub /: *mut *mut u8 tx_power[14]; / 0x54,
    pub /: *mut *mut u8 lpf_cutoff[14]; / 0x62,
    pub /: *mut *mut u8 lnags_threshold[14]; / 0x70,
    pub /: *mut *mut __le16 checksum; / 0x7E,
    pub /: *mut *mut u8 cis_data[]; / 0x80, 384 bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm8211_priv {
    pub pdev: *mut pci_dev,
    pub lock: spinlock_t,
    pub map: *mut adm8211_csr __iomem,
    pub rx_ring: *mut adm8211_desc,
    pub tx_ring: *mut adm8211_desc,
    pub rx_ring_dma: dma_addr_t,
    pub tx_ring_dma: dma_addr_t,
    pub rx_buffers: *mut adm8211_rx_ring_info,
    pub tx_buffers: *mut adm8211_tx_ring_info,
    pub tx_ring_size: unsigned int rx_ring_size,,
    pub cur_rx: unsigned int cur_tx, dirty_tx,,
    pub stats: ieee80211_low_level_stats,
    pub band: ieee80211_supported_band,
    pub channels: [ieee80211_channel; 14],
    pub mode: c_int,
    pub channel: c_int,
    pub bssid: [u8; ETH_ALEN],
    pub soft_rx_crc: u8,
    pub retry_limit: u8,
    pub ant_power: u8,
    pub tx_power: u8,
    pub lpf_cutoff: u8,
    pub lnags_threshold: u8,
    pub eeprom: *mut adm8211_eeprom,
    pub eeprom_len: usize,
    pub nar: u32,
pub const ADM8211_TYPE_INTERSIL: c_uint = 0x00;
pub const ADM8211_TYPE_RFMD: c_uint = 0x01;
pub const ADM8211_TYPE_MARVEL: c_uint = 0x02;
pub const ADM8211_TYPE_AIROHA: c_uint = 0x03;
pub const ADM8211_TYPE_ADMTEK: c_uint = 0x05;
    pub rf_type:3: c_uint,
    pub bbp_type:3: c_uint,
    pub specific_bbptype: u8,
    pub transceiver_type: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_chan_range {
    pub min: u8,
    pub max: u8,
}
