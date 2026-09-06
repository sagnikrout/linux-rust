//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_regs.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//
pub const MT_ASIC_VERSION: c_uint = 0x0000;
pub const MT76XX_REV_E3: c_uint = 0x22;
pub const MT76XX_REV_E4: c_uint = 0x33;
pub const MT_CMB_CTRL: c_uint = 0x0020;

pub const MT_EFUSE_CTRL: c_uint = 0x0024;

pub const MT_EFUSE_DATA_BASE: c_uint = 0x0028;

pub const MT_COEXCFG0: c_uint = 0x0040;

pub const MT_WLAN_FUN_CTRL: c_uint = 0x0080;

pub const MT_COEXCFG3: c_uint = 0x004c;
pub const MT_LDO_CTRL_0: c_uint = 0x006c;
pub const MT_LDO_CTRL_1: c_uint = 0x0070;

// MT76x0
pub const MT_CSR_EE_CFG1: c_uint = 0x0104;
pub const MT_XO_CTRL0: c_uint = 0x0100;
pub const MT_XO_CTRL1: c_uint = 0x0104;
pub const MT_XO_CTRL2: c_uint = 0x0108;
pub const MT_XO_CTRL3: c_uint = 0x010c;
pub const MT_XO_CTRL4: c_uint = 0x0110;
pub const MT_XO_CTRL5: c_uint = 0x0114;

pub const MT_XO_CTRL6: c_uint = 0x0118;

pub const MT_XO_CTRL7: c_uint = 0x011c;
pub const MT_IOCFG_6: c_uint = 0x0124;
pub const MT_USB_U3DMA_CFG: c_uint = 0x9018;

pub const MT_WLAN_MTC_CTRL: c_uint = 0x10148;

pub const MT_INT_SOURCE_CSR: c_uint = 0x0200;
pub const MT_INT_MASK_CSR: c_uint = 0x0204;

pub const MT_WPDMA_GLO_CFG: c_uint = 0x0208;

pub const MT_WPDMA_RST_IDX: c_uint = 0x020c;
pub const MT_WPDMA_DELAY_INT_CFG: c_uint = 0x0210;
pub const MT_WMM_AIFSN: c_uint = 0x0214;

pub const MT_WMM_CWMIN: c_uint = 0x0218;

pub const MT_WMM_CWMAX: c_uint = 0x021c;

pub const MT_WMM_TXOP_BASE: c_uint = 0x0220;

pub const MT_WMM_CTRL: c_uint = 0x0230 /* MT76x0 */;
pub const MT_FCE_DMA_ADDR: c_uint = 0x0230;
pub const MT_FCE_DMA_LEN: c_uint = 0x0234;
pub const MT_USB_DMA_CFG: c_uint = 0x0238;
pub const MT_TSO_CTRL: c_uint = 0x0250;
pub const MT_HEADER_TRANS_CTRL_REG: c_uint = 0x0260;
pub const MT_US_CYC_CFG: c_uint = 0x02a4;

pub const MT_TX_RING_BASE: c_uint = 0x0300;
pub const MT_RX_RING_BASE: c_uint = 0x03c0;
pub const MT_TX_HW_QUEUE_MCU: c_int = 8;
pub const MT_TX_HW_QUEUE_MGMT: c_int = 9;
pub const MT_PBF_SYS_CTRL: c_uint = 0x0400;

pub const MT_PBF_CFG: c_uint = 0x0404;

pub const MT_PBF_TX_MAX_PCNT: c_uint = 0x0408;
pub const MT_PBF_RX_MAX_PCNT: c_uint = 0x040c;
pub const MT_BCN_OFFSET_BASE: c_uint = 0x041c;

pub const MT_RXQ_STA: c_uint = 0x0430;
pub const MT_TXQ_STA: c_uint = 0x0434;
pub const MT_RF_CSR_CFG: c_uint = 0x0500;

pub const MT_RF_BYPASS_0: c_uint = 0x0504;
pub const MT_RF_BYPASS_1: c_uint = 0x0508;
pub const MT_RF_SETTING_0: c_uint = 0x050c;
pub const MT_RF_MISC: c_uint = 0x0518;
pub const MT_RF_DATA_WRITE: c_uint = 0x0524;
pub const MT_RF_CTRL: c_uint = 0x0528;

pub const MT_RF_DATA_READ: c_uint = 0x052c;
pub const MT_COM_REG0: c_uint = 0x0730;
pub const MT_COM_REG1: c_uint = 0x0734;
pub const MT_COM_REG2: c_uint = 0x0738;
pub const MT_COM_REG3: c_uint = 0x073C;
pub const MT_LED_CTRL: c_uint = 0x0770;

pub const MT_LED_TX_BLINK_0: c_uint = 0x0774;
pub const MT_LED_TX_BLINK_1: c_uint = 0x0778;
pub const MT_LED_S0_BASE: c_uint = 0x077C;

pub const MT_LED_S1_BASE: c_uint = 0x0780;

pub const MT_FCE_PSE_CTRL: c_uint = 0x0800;
pub const MT_FCE_PARAMETERS: c_uint = 0x0804;
pub const MT_FCE_CSO: c_uint = 0x0808;
pub const MT_FCE_L2_STUFF: c_uint = 0x080c;

pub const MT_FCE_WLAN_FLOW_CONTROL1: c_uint = 0x0824;
pub const MT_TX_CPU_FROM_FCE_BASE_PTR: c_uint = 0x09a0;
pub const MT_TX_CPU_FROM_FCE_MAX_COUNT: c_uint = 0x09a4;
pub const MT_TX_CPU_FROM_FCE_CPU_DESC_IDX: c_uint = 0x09a8;
pub const MT_FCE_PDMA_GLOBAL_CONF: c_uint = 0x09c4;
pub const MT_FCE_SKIP_FS: c_uint = 0x0a6c;
pub const MT_PAUSE_ENABLE_CONTROL1: c_uint = 0x0a38;
pub const MT_MAC_CSR0: c_uint = 0x1000;
pub const MT_MAC_SYS_CTRL: c_uint = 0x1004;

pub const MT_MAC_ADDR_DW0: c_uint = 0x1008;
pub const MT_MAC_ADDR_DW1: c_uint = 0x100c;

pub const MT_MAC_BSSID_DW0: c_uint = 0x1010;
pub const MT_MAC_BSSID_DW1: c_uint = 0x1014;

pub const MT_MAX_LEN_CFG: c_uint = 0x1018;

pub const MT_LED_CFG: c_uint = 0x102c;
pub const MT_AMPDU_MAX_LEN_20M1S: c_uint = 0x1030;
pub const MT_AMPDU_MAX_LEN_20M2S: c_uint = 0x1034;
pub const MT_AMPDU_MAX_LEN_40M1S: c_uint = 0x1038;
pub const MT_AMPDU_MAX_LEN_40M2S: c_uint = 0x103c;
pub const MT_AMPDU_MAX_LEN: c_uint = 0x1040;
pub const MT_WCID_DROP_BASE: c_uint = 0x106c;

pub const MT_BCN_BYPASS_MASK: c_uint = 0x108c;
pub const MT_MAC_APC_BSSID_BASE: c_uint = 0x1090;

pub const MT_XIFS_TIME_CFG: c_uint = 0x1100;

pub const MT_BKOFF_SLOT_CFG: c_uint = 0x1104;

pub const MT_CH_TIME_CFG: c_uint = 0x110c;

pub const MT_PBF_LIFE_TIMER: c_uint = 0x1110;
pub const MT_BEACON_TIME_CFG: c_uint = 0x1114;

pub const MT_TBTT_SYNC_CFG: c_uint = 0x1118;
pub const MT_TSF_TIMER_DW0: c_uint = 0x111c;
pub const MT_TSF_TIMER_DW1: c_uint = 0x1120;
pub const MT_TBTT_TIMER: c_uint = 0x1124;

pub const MT_INT_TIMER_CFG: c_uint = 0x1128;

pub const MT_INT_TIMER_EN: c_uint = 0x112c;

pub const MT_CH_IDLE: c_uint = 0x1130;
pub const MT_CH_BUSY: c_uint = 0x1134;
pub const MT_EXT_CH_BUSY: c_uint = 0x1138;
pub const MT_ED_CCA_TIMER: c_uint = 0x1140;
pub const MT_MAC_STATUS: c_uint = 0x1200;

pub const MT_PWR_PIN_CFG: c_uint = 0x1204;
pub const MT_AUX_CLK_CFG: c_uint = 0x120c;
pub const MT_BB_PA_MODE_CFG0: c_uint = 0x1214;
pub const MT_BB_PA_MODE_CFG1: c_uint = 0x1218;
pub const MT_RF_PA_MODE_CFG0: c_uint = 0x121c;
pub const MT_RF_PA_MODE_CFG1: c_uint = 0x1220;
pub const MT_RF_PA_MODE_ADJ0: c_uint = 0x1228;
pub const MT_RF_PA_MODE_ADJ1: c_uint = 0x122c;
pub const MT_DACCLK_EN_DLY_CFG: c_uint = 0x1264;
pub const MT_EDCA_CFG_BASE: c_uint = 0x1300;

pub const MT_TX_PWR_CFG_0: c_uint = 0x1314;
pub const MT_TX_PWR_CFG_1: c_uint = 0x1318;
pub const MT_TX_PWR_CFG_2: c_uint = 0x131c;
pub const MT_TX_PWR_CFG_3: c_uint = 0x1320;
pub const MT_TX_PWR_CFG_4: c_uint = 0x1324;
pub const MT_TX_PIN_CFG: c_uint = 0x1328;

pub const MT_TX_BAND_CFG: c_uint = 0x132c;

pub const MT_HT_FBK_TO_LEGACY: c_uint = 0x1384;
pub const MT_TX_MPDU_ADJ_INT: c_uint = 0x1388;
pub const MT_TX_PWR_CFG_7: c_uint = 0x13d4;
pub const MT_TX_PWR_CFG_8: c_uint = 0x13d8;
pub const MT_TX_PWR_CFG_9: c_uint = 0x13dc;
pub const MT_TX_SW_CFG0: c_uint = 0x1330;
pub const MT_TX_SW_CFG1: c_uint = 0x1334;
pub const MT_TX_SW_CFG2: c_uint = 0x1338;
pub const MT_TXOP_CTRL_CFG: c_uint = 0x1340;

pub const MT_TX_RTS_CFG: c_uint = 0x1344;

pub const MT_TX_TIMEOUT_CFG: c_uint = 0x1348;

pub const MT_TX_RETRY_CFG: c_uint = 0x134c;
pub const MT_TX_LINK_CFG: c_uint = 0x1350;

pub const MT_VHT_HT_FBK_CFG0: c_uint = 0x1354;
pub const MT_VHT_HT_FBK_CFG1: c_uint = 0x1358;
pub const MT_LG_FBK_CFG0: c_uint = 0x135c;
pub const MT_LG_FBK_CFG1: c_uint = 0x1360;

pub const MT_CCK_PROT_CFG: c_uint = 0x1364;
pub const MT_OFDM_PROT_CFG: c_uint = 0x1368;
pub const MT_MM20_PROT_CFG: c_uint = 0x136c;
pub const MT_MM40_PROT_CFG: c_uint = 0x1370;
pub const MT_GF20_PROT_CFG: c_uint = 0x1374;
pub const MT_GF40_PROT_CFG: c_uint = 0x1378;

pub const MT_PROT_RATE_CCK_11: c_uint = 0x0003;
pub const MT_PROT_RATE_OFDM_6: c_uint = 0x2000;
pub const MT_PROT_RATE_OFDM_24: c_uint = 0x2004;
pub const MT_PROT_RATE_DUP_OFDM_24: c_uint = 0x2084;
pub const MT_PROT_RATE_SGI_OFDM_24: c_uint = 0x2104;

pub const MT_EXP_ACK_TIME: c_uint = 0x1380;
pub const MT_TX_PWR_CFG_0_EXT: c_uint = 0x1390;
pub const MT_TX_PWR_CFG_1_EXT: c_uint = 0x1394;
pub const MT_TX_FBK_LIMIT: c_uint = 0x1398;

pub const MT_TX0_RF_GAIN_CORR: c_uint = 0x13a0;
pub const MT_TX1_RF_GAIN_CORR: c_uint = 0x13a4;
pub const MT_TX0_RF_GAIN_ATTEN: c_uint = 0x13a8;
pub const MT_TX0_RF_GAIN_ATTEN: c_uint = 0x13a8 /* MT76x0 */;
pub const MT_TX_ALC_CFG_0: c_uint = 0x13b0;

pub const MT_TX_ALC_CFG_1: c_uint = 0x13b4;

pub const MT_TX_ALC_CFG_2: c_uint = 0x13a8;

pub const MT_TX_ALC_CFG_3: c_uint = 0x13ac;
pub const MT_TX_ALC_CFG_4: c_uint = 0x13c0;

pub const MT_TX0_BB_GAIN_ATTEN: c_uint = 0x13c0 /* MT76x0 */;
pub const MT_TX_ALC_VGA3: c_uint = 0x13c8;
pub const MT_TX_PROT_CFG6: c_uint = 0x13e0;
pub const MT_TX_PROT_CFG7: c_uint = 0x13e4;
pub const MT_TX_PROT_CFG8: c_uint = 0x13e8;
pub const MT_PIFS_TX_CFG: c_uint = 0x13ec;
pub const MT_RX_FILTR_CFG: c_uint = 0x1400;

pub const MT_AUTO_RSP_CFG: c_uint = 0x1404;

pub const MT_LEGACY_BASIC_RATE: c_uint = 0x1408;
pub const MT_HT_BASIC_RATE: c_uint = 0x140c;
pub const MT_HT_CTRL_CFG: c_uint = 0x1410;
pub const MT_RX_PARSER_CFG: c_uint = 0x1418;

pub const MT_EXT_CCA_CFG: c_uint = 0x141c;

pub const MT_TX_SW_CFG3: c_uint = 0x1478;
pub const MT_PN_PAD_MODE: c_uint = 0x150c;
pub const MT_TXOP_HLDR_ET: c_uint = 0x1608;

pub const MT_PROT_AUTO_TX_CFG: c_uint = 0x1648;

pub const MT_RX_STAT_0: c_uint = 0x1700;

pub const MT_RX_STAT_1: c_uint = 0x1704;

pub const MT_RX_STAT_2: c_uint = 0x1708;

pub const MT_TX_STA_0: c_uint = 0x170c;

pub const MT_TX_STA_1: c_uint = 0x1710;
pub const MT_TX_STA_2: c_uint = 0x1714;
pub const MT_TX_STAT_FIFO: c_uint = 0x1718;

pub const MT_TX_AGG_STAT: c_uint = 0x171c;
pub const MT_TX_AGG_CNT_BASE0: c_uint = 0x1720;
pub const MT_MPDU_DENSITY_CNT: c_uint = 0x1740;
pub const MT_TX_AGG_CNT_BASE1: c_uint = 0x174c;

pub const MT_TX_STAT_FIFO_EXT: c_uint = 0x1798;

pub const MT_WCID_TX_RATE_BASE: c_uint = 0x1c00;

pub const MT_BBP_CORE_BASE: c_uint = 0x2000;
pub const MT_BBP_IBI_BASE: c_uint = 0x2100;
pub const MT_BBP_AGC_BASE: c_uint = 0x2300;
pub const MT_BBP_TXC_BASE: c_uint = 0x2400;
pub const MT_BBP_RXC_BASE: c_uint = 0x2500;
pub const MT_BBP_TXO_BASE: c_uint = 0x2600;
pub const MT_BBP_TXBE_BASE: c_uint = 0x2700;
pub const MT_BBP_RXFE_BASE: c_uint = 0x2800;
pub const MT_BBP_RXO_BASE: c_uint = 0x2900;
pub const MT_BBP_DFS_BASE: c_uint = 0x2a00;
pub const MT_BBP_TR_BASE: c_uint = 0x2b00;
pub const MT_BBP_CAL_BASE: c_uint = 0x2c00;
pub const MT_BBP_DSC_BASE: c_uint = 0x2e00;
pub const MT_BBP_PFMU_BASE: c_uint = 0x2f00;

// AGC, R4/R5

// AGC, R6/R7

// AGC, R8/R9

pub const MT_WCID_ADDR_BASE: c_uint = 0x1800;

pub const MT_SRAM_BASE: c_uint = 0x4000;
pub const MT_WCID_KEY_BASE: c_uint = 0x8000;

pub const MT_WCID_IV_BASE: c_uint = 0xa000;

pub const MT_WCID_ATTR_BASE: c_uint = 0xa800;

pub const MT_SKEY_BASE_0: c_uint = 0xac00;
pub const MT_SKEY_BASE_1: c_uint = 0xb400;

pub const MT_SKEY_MODE_BASE_0: c_uint = 0xb000;
pub const MT_SKEY_MODE_BASE_1: c_uint = 0xb3f0;

pub const MT_BEACON_BASE: c_uint = 0xc000;
pub const MT_TEMP_SENSOR: c_uint = 0x1d000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wcid_addr {
    pub macaddr: [u8; 6],
    pub ba_mask: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wcid_key {
    pub key: [u8; 16],
    pub tx_mic: [u8; 8],
    pub rx_mic: [u8; 8],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x02_cipher_type {
    MT76X02_CIPHER_NONE,
    MT76X02_CIPHER_WEP40,
    MT76X02_CIPHER_WEP104,
    MT76X02_CIPHER_TKIP,
    MT76X02_CIPHER_AES_CCMP,
    MT76X02_CIPHER_CKIP40,
    MT76X02_CIPHER_CKIP104,
    MT76X02_CIPHER_CKIP128,
    MT76X02_CIPHER_WAPI,
}
