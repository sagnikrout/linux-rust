//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2800.h
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
// RF chip defines.
//
// RF2820 2.4G 2T3R
// RF2850 2.4G/5G 2T3R
// RF2720 2.4G 1T2R
// RF2750 2.4G/5G 1T2R
// RF3020 2.4G 1T1R
// RF2020 2.4G B/G
// RF3021 2.4G 1T2R
// RF3022 2.4G 2T2R
// RF3052 2.4G/5G 2T2R
// RF2853 2.4G/5G 3T3R
// RF3320 2.4G 1T1R(RT3350/RT3370/RT3390)
// RF3322 2.4G 2T2R(RT3352/RT3371/RT3372/RT3391/RT3392)
// RF3053 2.4G/5G 3T3R(RT3563/RT3573/RT3593)
// RF3853 2.4G/5G 3T3R(RT3883/RT3662)
// RF5592 2.4G/5G 2T2R
// RF3070 2.4G 1T1R
// RF5360 2.4G 1T1R
// RF5362 2.4G 1T1R
// RF5370 2.4G 1T1R
// RF5390 2.4G 1T1R
//
pub const RF2820: c_uint = 0x0001;
pub const RF2850: c_uint = 0x0002;
pub const RF2720: c_uint = 0x0003;
pub const RF2750: c_uint = 0x0004;
pub const RF3020: c_uint = 0x0005;
pub const RF2020: c_uint = 0x0006;
pub const RF3021: c_uint = 0x0007;
pub const RF3022: c_uint = 0x0008;
pub const RF3052: c_uint = 0x0009;
pub const RF2853: c_uint = 0x000a;
pub const RF3320: c_uint = 0x000b;
pub const RF3322: c_uint = 0x000c;
pub const RF3053: c_uint = 0x000d;
pub const RF5592: c_uint = 0x000f;
pub const RF3070: c_uint = 0x3070;
pub const RF3290: c_uint = 0x3290;
pub const RF3853: c_uint = 0x3853;
pub const RF5350: c_uint = 0x5350;
pub const RF5360: c_uint = 0x5360;
pub const RF5362: c_uint = 0x5362;
pub const RF5370: c_uint = 0x5370;
pub const RF5372: c_uint = 0x5372;
pub const RF5390: c_uint = 0x5390;
pub const RF5392: c_uint = 0x5392;
pub const RF7620: c_uint = 0x7620;
//
// Chipset revisions.
//
pub const REV_RT2860C: c_uint = 0x0100;
pub const REV_RT2860D: c_uint = 0x0101;
pub const REV_RT2872E: c_uint = 0x0200;
pub const REV_RT3070E: c_uint = 0x0200;
pub const REV_RT3070F: c_uint = 0x0201;
pub const REV_RT3071E: c_uint = 0x0211;
pub const REV_RT3090E: c_uint = 0x0211;
pub const REV_RT3390E: c_uint = 0x0211;
pub const REV_RT3593E: c_uint = 0x0211;
pub const REV_RT5390F: c_uint = 0x0502;
pub const REV_RT5370G: c_uint = 0x0503;
pub const REV_RT5390R: c_uint = 0x1502;
pub const REV_RT5592C: c_uint = 0x0221;
pub const DEFAULT_RSSI_OFFSET: c_int = 120;
//
// Register layout information.
//
pub const CSR_REG_BASE: c_uint = 0x1000;
pub const CSR_REG_SIZE: c_uint = 0x0800;
pub const EEPROM_BASE: c_uint = 0x0000;
pub const EEPROM_SIZE: c_uint = 0x0200;
pub const BBP_BASE: c_uint = 0x0000;
pub const BBP_SIZE: c_uint = 0x00ff;
pub const RF_BASE: c_uint = 0x0004;
pub const RF_SIZE: c_uint = 0x0010;
pub const RFCSR_BASE: c_uint = 0x0000;
pub const RFCSR_SIZE: c_uint = 0x0040;
//
// Number of TX queues.
//
pub const NUM_TX_QUEUES: c_int = 4;
//
// Registers.
//
// MAC_CSR0_3290: MAC_CSR0 for RT3290 to identity MAC version number.
//
pub const MAC_CSR0_3290: c_uint = 0x0000;
//
// E2PROM_CSR: PCI EEPROM control register.
// RELOAD: Write 1 to reload eeprom content.
// TYPE: 0: 93c46, 1:93c66.
// LOAD_STATUS: 1:loading, 0:done.
//
pub const E2PROM_CSR: c_uint = 0x0004;

//
// CMB_CTRL_CFG
//
pub const CMB_CTRL: c_uint = 0x0020;

//
// EFUSE_CSR_3290: RT3290 EEPROM
//
pub const EFUSE_CTRL_3290: c_uint = 0x0024;
//
// EFUSE_DATA3 of 3290
//
pub const EFUSE_DATA3_3290: c_uint = 0x0028;
//
// EFUSE_DATA2 of 3290
//
pub const EFUSE_DATA2_3290: c_uint = 0x002c;
//
// EFUSE_DATA1 of 3290
//
pub const EFUSE_DATA1_3290: c_uint = 0x0030;
//
// EFUSE_DATA0 of 3290
//
pub const EFUSE_DATA0_3290: c_uint = 0x0034;
//
// OSC_CTRL_CFG
// Ring oscillator configuration
//
pub const OSC_CTRL: c_uint = 0x0038;

//
// COEX_CFG_0
//
pub const COEX_CFG0: c_uint = 0x0040;

//
// COEX_CFG_1
//
pub const COEX_CFG1: c_uint = 0x0044;
//
// COEX_CFG_2
//
pub const COEX_CFG2: c_uint = 0x0048;

//
// PLL_CTRL_CFG
// PLL configuration register
//
pub const PLL_CTRL: c_uint = 0x0050;

//
// WLAN_CTRL_CFG
// RT3290 wlan configuration
//
pub const WLAN_FUN_CTRL: c_uint = 0x0080;

//
// AUX_CTRL: Aux/PCI-E related configuration
//
pub const AUX_CTRL: c_uint = 0x10c;

//
// OPT_14: Unknown register used by rt3xxx devices.
//
pub const OPT_14_CSR: c_uint = 0x0114;

//
// INT_SOURCE_CSR: Interrupt source register.
// Write one to clear corresponding bit.
// TX_FIFO_STATUS: FIFO Statistics is full, sw should read TX_STA_FIFO
//
pub const INT_SOURCE_CSR: c_uint = 0x0200;

//
// INT_MASK_CSR: Interrupt MASK register. 1: the interrupt is mask OFF.
//
pub const INT_MASK_CSR: c_uint = 0x0204;

//
// WPDMA_GLO_CFG
//
pub const WPDMA_GLO_CFG: c_uint = 0x0208;

//
// WPDMA_RST_IDX
//
pub const WPDMA_RST_IDX: c_uint = 0x020c;

//
// DELAY_INT_CFG
//
pub const DELAY_INT_CFG: c_uint = 0x0210;

//
// WMM_AIFSN_CFG: Aifsn for each EDCA AC
// AIFSN0: AC_VO
// AIFSN1: AC_VI
// AIFSN2: AC_BE
// AIFSN3: AC_BK
//
pub const WMM_AIFSN_CFG: c_uint = 0x0214;

//
// WMM_CWMIN_CSR: CWmin for each EDCA AC
// CWMIN0: AC_VO
// CWMIN1: AC_VI
// CWMIN2: AC_BE
// CWMIN3: AC_BK
//
pub const WMM_CWMIN_CFG: c_uint = 0x0218;

//
// WMM_CWMAX_CSR: CWmax for each EDCA AC
// CWMAX0: AC_VO
// CWMAX1: AC_VI
// CWMAX2: AC_BE
// CWMAX3: AC_BK
//
pub const WMM_CWMAX_CFG: c_uint = 0x021c;

//
// AC_TXOP0: AC_VO/AC_VI TXOP register
// AC0TXOP: AC_VO in unit of 32us
// AC1TXOP: AC_VI in unit of 32us
//
pub const WMM_TXOP0_CFG: c_uint = 0x0220;

//
// AC_TXOP1: AC_BE/AC_BK TXOP register
// AC2TXOP: AC_BE in unit of 32us
// AC3TXOP: AC_BK in unit of 32us
//
pub const WMM_TXOP1_CFG: c_uint = 0x0224;

//
// GPIO_CTRL:
// GPIO_CTRL_VALx: GPIO value
// GPIO_CTRL_DIRx: GPIO direction: 0 = output; 1 = input
//
pub const GPIO_CTRL: c_uint = 0x0228;

//
// MCU_CMD_CFG
//
pub const MCU_CMD_CFG: c_uint = 0x022c;
//
// AC_VO register offsets
//
pub const TX_BASE_PTR0: c_uint = 0x0230;
pub const TX_MAX_CNT0: c_uint = 0x0234;
pub const TX_CTX_IDX0: c_uint = 0x0238;
pub const TX_DTX_IDX0: c_uint = 0x023c;
//
// AC_VI register offsets
//
pub const TX_BASE_PTR1: c_uint = 0x0240;
pub const TX_MAX_CNT1: c_uint = 0x0244;
pub const TX_CTX_IDX1: c_uint = 0x0248;
pub const TX_DTX_IDX1: c_uint = 0x024c;
//
// AC_BE register offsets
//
pub const TX_BASE_PTR2: c_uint = 0x0250;
pub const TX_MAX_CNT2: c_uint = 0x0254;
pub const TX_CTX_IDX2: c_uint = 0x0258;
pub const TX_DTX_IDX2: c_uint = 0x025c;
//
// AC_BK register offsets
//
pub const TX_BASE_PTR3: c_uint = 0x0260;
pub const TX_MAX_CNT3: c_uint = 0x0264;
pub const TX_CTX_IDX3: c_uint = 0x0268;
pub const TX_DTX_IDX3: c_uint = 0x026c;
//
// HCCA register offsets
//
pub const TX_BASE_PTR4: c_uint = 0x0270;
pub const TX_MAX_CNT4: c_uint = 0x0274;
pub const TX_CTX_IDX4: c_uint = 0x0278;
pub const TX_DTX_IDX4: c_uint = 0x027c;
//
// MGMT register offsets
//
pub const TX_BASE_PTR5: c_uint = 0x0280;
pub const TX_MAX_CNT5: c_uint = 0x0284;
pub const TX_CTX_IDX5: c_uint = 0x0288;
pub const TX_DTX_IDX5: c_uint = 0x028c;
//
// RX register offsets
//
pub const RX_BASE_PTR: c_uint = 0x0290;
pub const RX_MAX_CNT: c_uint = 0x0294;
pub const RX_CRX_IDX: c_uint = 0x0298;
pub const RX_DRX_IDX: c_uint = 0x029c;
//
// USB_DMA_CFG
// RX_BULK_AGG_TIMEOUT: Rx Bulk Aggregation TimeOut in unit of 33ns.
// RX_BULK_AGG_LIMIT: Rx Bulk Aggregation Limit in unit of 256 bytes.
// PHY_CLEAR: phy watch dog enable.
// TX_CLEAR: Clear USB DMA TX path.
// TXOP_HALT: Halt TXOP count down when TX buffer is full.
// RX_BULK_AGG_EN: Enable Rx Bulk Aggregation.
// RX_BULK_EN: Enable USB DMA Rx.
// TX_BULK_EN: Enable USB DMA Tx.
// EP_OUT_VALID: OUT endpoint data valid.
// RX_BUSY: USB DMA RX FSM busy.
// TX_BUSY: USB DMA TX FSM busy.
//
pub const USB_DMA_CFG: c_uint = 0x02a0;

//
// US_CYC_CNT
// BT_MODE_EN: Bluetooth mode enable
// CLOCK CYCLE: Clock cycle count in 1us.
// PCI:0x21, PCIE:0x7d, USB:0x1e
//
pub const US_CYC_CNT: c_uint = 0x02a4;

//
// PBF_SYS_CTRL
// HOST_RAM_WRITE: enable Host program ram write selection
//
pub const PBF_SYS_CTRL: c_uint = 0x0400;

//
// HOST-MCU shared memory
//
pub const HOST_CMD_CSR: c_uint = 0x0404;

//
// PBF registers
// Most are for debug. Driver doesn't touch PBF register.
//
pub const PBF_CFG: c_uint = 0x0408;
pub const PBF_MAX_PCNT: c_uint = 0x040c;
pub const PBF_CTRL: c_uint = 0x0410;
pub const PBF_INT_STA: c_uint = 0x0414;
pub const PBF_INT_ENA: c_uint = 0x0418;
//
// BCN_OFFSET0:
//
pub const BCN_OFFSET0: c_uint = 0x042c;

//
// BCN_OFFSET1:
//
pub const BCN_OFFSET1: c_uint = 0x0430;

//
// TXRXQ_PCNT: PBF register
// PCNT_TX0Q: Page count for TX hardware queue 0
// PCNT_TX1Q: Page count for TX hardware queue 1
// PCNT_TX2Q: Page count for TX hardware queue 2
// PCNT_RX0Q: Page count for RX hardware queue
//
pub const TXRXQ_PCNT: c_uint = 0x0438;

//
// PBF register
// Debug. Driver doesn't touch PBF register.
//
pub const PBF_DBG: c_uint = 0x043c;
//
// RF registers
//
pub const RF_CSR_CFG: c_uint = 0x0500;

//
// MT7620 RF registers (reversed order)
//

// undocumented registers for calibration of new MAC
pub const RF_CONTROL0: c_uint = 0x0518;
pub const RF_BYPASS0: c_uint = 0x051c;
pub const RF_CONTROL1: c_uint = 0x0520;
pub const RF_BYPASS1: c_uint = 0x0524;
pub const RF_CONTROL2: c_uint = 0x0528;
pub const RF_BYPASS2: c_uint = 0x052c;
pub const RF_CONTROL3: c_uint = 0x0530;
pub const RF_BYPASS3: c_uint = 0x0534;
//
// EFUSE_CSR: RT30x0 EEPROM
//
pub const EFUSE_CTRL: c_uint = 0x0580;

//
// EFUSE_DATA0
//
pub const EFUSE_DATA0: c_uint = 0x0590;
//
// EFUSE_DATA1
//
pub const EFUSE_DATA1: c_uint = 0x0594;
//
// EFUSE_DATA2
//
pub const EFUSE_DATA2: c_uint = 0x0598;
//
// EFUSE_DATA3
//
pub const EFUSE_DATA3: c_uint = 0x059c;
//
// LDO_CFG0
//
pub const LDO_CFG0: c_uint = 0x05d4;

//
// GPIO_SWITCH
//
pub const GPIO_SWITCH: c_uint = 0x05dc;

//
// FIXME: where the DEBUG_INDEX name come from?
//
pub const MAC_DEBUG_INDEX: c_uint = 0x05e8;

//
// MAC Control/Status Registers(CSR).
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// MAC_CSR0: ASIC revision number.
// ASIC_REV: 0
// ASIC_VER: 2860 or 2870
//
pub const MAC_CSR0: c_uint = 0x1000;

//
// MAC_SYS_CTRL:
//
pub const MAC_SYS_CTRL: c_uint = 0x1004;

//
// MAC_ADDR_DW0: STA MAC register 0
//
pub const MAC_ADDR_DW0: c_uint = 0x1008;

//
// MAC_ADDR_DW1: STA MAC register 1
// UNICAST_TO_ME_MASK:
// Used to mask off bits from byte 5 of the MAC address
// to determine the UNICAST_TO_ME bit for RX frames.
// The full mask is complemented by BSS_ID_MASK:
// MASK = BSS_ID_MASK & UNICAST_TO_ME_MASK
//
pub const MAC_ADDR_DW1: c_uint = 0x100c;

//
// MAC_BSSID_DW0: BSSID register 0
//
pub const MAC_BSSID_DW0: c_uint = 0x1010;

//
// MAC_BSSID_DW1: BSSID register 1
// BSS_ID_MASK:
// 0: 1-BSSID mode (BSS index = 0)
// 1: 2-BSSID mode (BSS index: Byte5, bit 0)
// 2: 4-BSSID mode (BSS index: byte5, bit 0 - 1)
// 3: 8-BSSID mode (BSS index: byte5, bit 0 - 2)
// This mask is used to mask off bits 0, 1 and 2 of byte 5 of the
// BSSID. This will make sure that those bits will be ignored
// when determining the MY_BSS of RX frames.
//
pub const MAC_BSSID_DW1: c_uint = 0x1014;

//
// MAX_LEN_CFG: Maximum frame length register.
// MAX_MPDU: rt2860b max 16k bytes
// MAX_PSDU: Maximum PSDU length
// (power factor) 0:2^13, 1:2^14, 2:2^15, 3:2^16
//
pub const MAX_LEN_CFG: c_uint = 0x1018;

//
// BBP_CSR_CFG: BBP serial control register
// VALUE: Register value to program into BBP
// REG_NUM: Selected BBP register
// READ_CONTROL: 0 write BBP, 1 read BBP
// BUSY: ASIC is busy executing BBP commands
// BBP_PAR_DUR: 0 4 MAC clocks, 1 8 MAC clocks
// BBP_RW_MODE: 0 serial, 1 parallel
//
pub const BBP_CSR_CFG: c_uint = 0x101c;

//
// RF_CSR_CFG0: RF control register
// REGID_AND_VALUE: Register value to program into RF
// BITWIDTH: Selected RF register
// STANDBYMODE: 0 high when standby, 1 low when standby
// SEL: 0 RF_LE0 activate, 1 RF_LE1 activate
// BUSY: ASIC is busy executing RF commands
//
pub const RF_CSR_CFG0: c_uint = 0x1020;

//
// RF_CSR_CFG1: RF control register
// REGID_AND_VALUE: Register value to program into RF
// RFGAP: Gap between BB_CONTROL_RF and RF_LE
// 0: 3 system clock cycle (37.5usec)
// 1: 5 system clock cycle (62.5usec)
//
pub const RF_CSR_CFG1: c_uint = 0x1024;

//
// RF_CSR_CFG2: RF control register
// VALUE: Register value to program into RF
//
pub const RF_CSR_CFG2: c_uint = 0x1028;

//
// LED_CFG: LED control
// ON_PERIOD: LED active time (ms) during TX (only used for LED mode 1)
// OFF_PERIOD: LED inactive time (ms) during TX (only used for LED mode 1)
// SLOW_BLINK_PERIOD: LED blink interval in seconds (only used for LED mode 2)
// color LED's:
// 0: off
// 1: blinking upon TX2
// 2: periodic slow blinking
// 3: always on
// LED polarity:
// 0: active low
// 1: active high
//
pub const LED_CFG: c_uint = 0x102c;

//
// AMPDU_MAX_LEN_20M1S: Per MCS max A-MPDU length, 20 MHz, MCS 0-7
// AMPDU_MAX_LEN_20M2S: Per MCS max A-MPDU length, 20 MHz, MCS 8-15
// AMPDU_MAX_LEN_40M1S: Per MCS max A-MPDU length, 40 MHz, MCS 0-7
// AMPDU_MAX_LEN_40M2S: Per MCS max A-MPDU length, 40 MHz, MCS 8-15
// Maximum A-MPDU length = 2^(AMPDU_MAX - 5) kilobytes
//
pub const AMPDU_MAX_LEN_20M1S: c_uint = 0x1030;
pub const AMPDU_MAX_LEN_20M2S: c_uint = 0x1034;
pub const AMPDU_MAX_LEN_40M1S: c_uint = 0x1038;
pub const AMPDU_MAX_LEN_40M2S: c_uint = 0x103C;
//
// AMPDU_BA_WINSIZE: Force BlockAck window size
// FORCE_WINSIZE_ENABLE:
// 0: Disable forcing of BlockAck window size
// 1: Enable forcing of BlockAck window size, overwrites values BlockAck
// window size values in the TXWI
// FORCE_WINSIZE: BlockAck window size
//
pub const AMPDU_BA_WINSIZE: c_uint = 0x1040;

//
// XIFS_TIME_CFG: MAC timing
// CCKM_SIFS_TIME: unit 1us. Applied after CCK RX/TX
// OFDM_SIFS_TIME: unit 1us. Applied after OFDM RX/TX
// OFDM_XIFS_TIME: unit 1us. Applied after OFDM RX
// when MAC doesn't reference BBP signal BBRXEND
// EIFS: unit 1us
// BB_RXEND_ENABLE: reference RXEND signal to begin XIFS defer
//
pub const XIFS_TIME_CFG: c_uint = 0x1100;

//
// BKOFF_SLOT_CFG:
//
pub const BKOFF_SLOT_CFG: c_uint = 0x1104;

//
// NAV_TIME_CFG:
//
pub const NAV_TIME_CFG: c_uint = 0x1108;

//
// CH_TIME_CFG: count as channel busy
// EIFS_BUSY: Count EIFS as channel busy
// NAV_BUSY: Count NAS as channel busy
// RX_BUSY: Count RX as channel busy
// TX_BUSY: Count TX as channel busy
// TMR_EN: Enable channel statistics timer
//
pub const CH_TIME_CFG: c_uint = 0x110c;

//
// PBF_LIFE_TIMER: TX/RX MPDU timestamp timer (free run) Unit: 1us
//
pub const PBF_LIFE_TIMER: c_uint = 0x1110;
//
// BCN_TIME_CFG:
// BEACON_INTERVAL: in unit of 1/16 TU
// TSF_TICKING: Enable TSF auto counting
// TSF_SYNC: Enable TSF sync, 00: disable, 01: infra mode, 10: ad-hoc mode
// BEACON_GEN: Enable beacon generator
//
pub const BCN_TIME_CFG: c_uint = 0x1114;

//
// TBTT_SYNC_CFG:
// BCN_AIFSN: Beacon AIFSN after TBTT interrupt in slots
// BCN_CWMIN: Beacon CWMin after TBTT interrupt in slots
//
pub const TBTT_SYNC_CFG: c_uint = 0x1118;

//
// TSF_TIMER_DW0: Local lsb TSF timer, read-only
//
pub const TSF_TIMER_DW0: c_uint = 0x111c;

//
// TSF_TIMER_DW1: Local msb TSF timer, read-only
//
pub const TSF_TIMER_DW1: c_uint = 0x1120;

//
// TBTT_TIMER: TImer remains till next TBTT, read-only
//
pub const TBTT_TIMER: c_uint = 0x1124;
//
// INT_TIMER_CFG: timer configuration
// PRE_TBTT_TIMER: leadtime to tbtt for pretbtt interrupt in units of 1/16 TU
// GP_TIMER: period of general purpose timer in units of 1/16 TU
//
pub const INT_TIMER_CFG: c_uint = 0x1128;

//
// INT_TIMER_EN: GP-timer and pre-tbtt Int enable
//
pub const INT_TIMER_EN: c_uint = 0x112c;

//
// CH_IDLE_STA: channel idle time (in us)
//
pub const CH_IDLE_STA: c_uint = 0x1130;
//
// CH_BUSY_STA: channel busy time on primary channel (in us)
//
pub const CH_BUSY_STA: c_uint = 0x1134;
//
// CH_BUSY_STA_SEC: channel busy time on secondary channel in HT40 mode (in us)
//
pub const CH_BUSY_STA_SEC: c_uint = 0x1138;
//
// MAC_STATUS_CFG:
// BBP_RF_BUSY: When set to 0, BBP and RF are stable.
// if 1 or higher one of the 2 registers is busy.
//
pub const MAC_STATUS_CFG: c_uint = 0x1200;

//
// PWR_PIN_CFG:
//
pub const PWR_PIN_CFG: c_uint = 0x1204;
//
// AUTOWAKEUP_CFG: Manual power control / status register
// TBCN_BEFORE_WAKE: ForceWake has high privilege than PutToSleep when both set
// AUTOWAKE: 0:sleep, 1:awake
//
pub const AUTOWAKEUP_CFG: c_uint = 0x1208;

//
// MIMO_PS_CFG: MIMO Power-save Configuration
//
pub const MIMO_PS_CFG: c_uint = 0x1210;

//
// EDCA_AC0_CFG:
//
pub const EDCA_AC0_CFG: c_uint = 0x1300;

//
// EDCA_AC1_CFG:
//
pub const EDCA_AC1_CFG: c_uint = 0x1304;

//
// EDCA_AC2_CFG:
//
pub const EDCA_AC2_CFG: c_uint = 0x1308;

//
// EDCA_AC3_CFG:
//
pub const EDCA_AC3_CFG: c_uint = 0x130c;

//
// EDCA_TID_AC_MAP:
//
pub const EDCA_TID_AC_MAP: c_uint = 0x1310;
//
// TX_PWR_CFG:
//

//
// TX_PWR_CFG_0:
//
pub const TX_PWR_CFG_0: c_uint = 0x1314;

// bits for 3T devices

// bits for new 2T devices

//
// TX_PWR_CFG_1:
//
pub const TX_PWR_CFG_1: c_uint = 0x1318;

// bits for 3T devices

// bits for new 2T devices

//
// TX_PWR_CFG_2:
//
pub const TX_PWR_CFG_2: c_uint = 0x131c;

// bits for 3T devices

// bits for new 2T devices

//
// TX_PWR_CFG_3:
//
pub const TX_PWR_CFG_3: c_uint = 0x1320;

// bits for 3T devices

// bits for new 2T devices

//
// TX_PWR_CFG_4:
//
pub const TX_PWR_CFG_4: c_uint = 0x1324;

// bits for 3T devices

// bits for new 2T devices

//
// TX_PIN_CFG:
//
pub const TX_PIN_CFG: c_uint = 0x1328;
pub const TX_PIN_CFG_PA_PE_DISABLE: c_uint = 0xfcfffff0;

//
// TX_BAND_CFG: 0x1 use upper 20MHz, 0x0 use lower 20MHz
//
pub const TX_BAND_CFG: c_uint = 0x132c;

//
// TX_SW_CFG0:
//
pub const TX_SW_CFG0: c_uint = 0x1330;
//
// TX_SW_CFG1:
//
pub const TX_SW_CFG1: c_uint = 0x1334;
//
// TX_SW_CFG2:
//
pub const TX_SW_CFG2: c_uint = 0x1338;
//
// TXOP_THRES_CFG:
//
pub const TXOP_THRES_CFG: c_uint = 0x133c;
//
// TXOP_CTRL_CFG:
// TIMEOUT_TRUN_EN: Enable/Disable TXOP timeout truncation
// AC_TRUN_EN: Enable/Disable truncation for AC change
// TXRATEGRP_TRUN_EN: Enable/Disable truncation for TX rate group change
// USER_MODE_TRUN_EN: Enable/Disable truncation for user TXOP mode
// MIMO_PS_TRUN_EN: Enable/Disable truncation for MIMO PS RTS/CTS
// RESERVED_TRUN_EN: Reserved
// LSIG_TXOP_EN: Enable/Disable L-SIG TXOP protection
// EXT_CCA_EN: Enable/Disable extension channel CCA reference (Defer 40Mhz
// transmissions if extension CCA is clear).
// EXT_CCA_DLY: Extension CCA signal delay time (unit: us)
// EXT_CWMIN: CwMin for extension channel backoff
// 0: Disabled
//
pub const TXOP_CTRL_CFG: c_uint = 0x1340;

//
// TX_RTS_CFG:
// RTS_THRES: unit:byte
// RTS_FBK_EN: enable rts rate fallback
//
pub const TX_RTS_CFG: c_uint = 0x1344;

//
// TX_TIMEOUT_CFG:
// MPDU_LIFETIME: expiration time = 2^(9+MPDU LIFE TIME) us
// RX_ACK_TIMEOUT: unit:slot. Used for TX procedure
// TX_OP_TIMEOUT: TXOP timeout value for TXOP truncation.
// it is recommended that:
// (SLOT_TIME) > (TX_OP_TIMEOUT) > (RX_ACK_TIMEOUT)
//
pub const TX_TIMEOUT_CFG: c_uint = 0x1348;

//
// TX_RTY_CFG:
// SHORT_RTY_LIMIT: short retry limit
// LONG_RTY_LIMIT: long retry limit
// LONG_RTY_THRE: Long retry threshoold
// NON_AGG_RTY_MODE: Non-Aggregate MPDU retry mode
// 0:expired by retry limit, 1: expired by mpdu life timer
// AGG_RTY_MODE: Aggregate MPDU retry mode
// 0:expired by retry limit, 1: expired by mpdu life timer
// TX_AUTO_FB_ENABLE: Tx retry PHY rate auto fallback enable
//
pub const TX_RTY_CFG: c_uint = 0x134c;

//
// TX_LINK_CFG:
// REMOTE_MFB_LIFETIME: remote MFB life time. unit: 32us
// MFB_ENABLE: TX apply remote MFB 1:enable
// REMOTE_UMFS_ENABLE: remote unsolicit  MFB enable
// 0: not apply remote remote unsolicit (MFS=7)
// TX_MRQ_EN: MCS request TX enable
// TX_RDG_EN: RDG TX enable
// TX_CF_ACK_EN: Piggyback CF-ACK enable
// REMOTE_MFB: remote MCS feedback
// REMOTE_MFS: remote MCS feedback sequence number
//
pub const TX_LINK_CFG: c_uint = 0x1350;

//
// HT_FBK_CFG0:
//
pub const HT_FBK_CFG0: c_uint = 0x1354;

//
// HT_FBK_CFG1:
//
pub const HT_FBK_CFG1: c_uint = 0x1358;

//
// LG_FBK_CFG0:
//
pub const LG_FBK_CFG0: c_uint = 0x135c;

//
// LG_FBK_CFG1:
//
pub const LG_FBK_CFG1: c_uint = 0x1360;

//
// CCK_PROT_CFG: CCK Protection
// PROTECT_RATE: Protection control frame rate for CCK TX(RTS/CTS/CFEnd)
// PROTECT_CTRL: Protection control frame type for CCK TX
// 0:none, 1:RTS/CTS, 2:CTS-to-self
// PROTECT_NAV_SHORT: TXOP protection type for CCK TX with short NAV
// PROTECT_NAV_LONG: TXOP protection type for CCK TX with long NAV
// TX_OP_ALLOW_CCK: CCK TXOP allowance, 0:disallow
// TX_OP_ALLOW_OFDM: CCK TXOP allowance, 0:disallow
// TX_OP_ALLOW_MM20: CCK TXOP allowance, 0:disallow
// TX_OP_ALLOW_MM40: CCK TXOP allowance, 0:disallow
// TX_OP_ALLOW_GF20: CCK TXOP allowance, 0:disallow
// TX_OP_ALLOW_GF40: CCK TXOP allowance, 0:disallow
// RTS_TH_EN: RTS threshold enable on CCK TX
//
pub const CCK_PROT_CFG: c_uint = 0x1364;

//
// OFDM_PROT_CFG: OFDM Protection
//
pub const OFDM_PROT_CFG: c_uint = 0x1368;

//
// MM20_PROT_CFG: MM20 Protection
//
pub const MM20_PROT_CFG: c_uint = 0x136c;

//
// MM40_PROT_CFG: MM40 Protection
//
pub const MM40_PROT_CFG: c_uint = 0x1370;

//
// GF20_PROT_CFG: GF20 Protection
//
pub const GF20_PROT_CFG: c_uint = 0x1374;

//
// GF40_PROT_CFG: GF40 Protection
//
pub const GF40_PROT_CFG: c_uint = 0x1378;

//
// EXP_CTS_TIME:
//
pub const EXP_CTS_TIME: c_uint = 0x137c;
//
// EXP_ACK_TIME:
//
pub const EXP_ACK_TIME: c_uint = 0x1380;
//
// HT_FBK_TO_LEGACY: Enable/Disable HT/RTS fallback to OFDM/CCK rate
// Not available for legacy SoCs
//
pub const HT_FBK_TO_LEGACY: c_uint = 0x1384;
// TX_PWR_CFG_5
pub const TX_PWR_CFG_5: c_uint = 0x1384;

// TX_PWR_CFG_6
pub const TX_PWR_CFG_6: c_uint = 0x1388;

// TX_PWR_CFG_0_EXT
pub const TX_PWR_CFG_0_EXT: c_uint = 0x1390;

// TX_PWR_CFG_1_EXT
pub const TX_PWR_CFG_1_EXT: c_uint = 0x1394;

// TX_PWR_CFG_2_EXT
pub const TX_PWR_CFG_2_EXT: c_uint = 0x1398;

// TX_PWR_CFG_3_EXT
pub const TX_PWR_CFG_3_EXT: c_uint = 0x139c;

// TX_PWR_CFG_4_EXT
pub const TX_PWR_CFG_4_EXT: c_uint = 0x13a0;

// TXn_RF_GAIN_CORRECT: RF Gain Correction for each RF_ALC[3:2]
// Unit: 0.1 dB, Range: -3.2 dB to 3.1 dB
//
pub const TX0_RF_GAIN_CORRECT: c_uint = 0x13a0;

pub const TX1_RF_GAIN_CORRECT: c_uint = 0x13a4;

// TXn_RF_GAIN_ATTEN: TXn RF Gain Attenuation Level
// Format: 7-bit, signed value
// Unit: 0.5 dB, Range: -20 dB to -5 dB
//
pub const TX0_RF_GAIN_ATTEN: c_uint = 0x13a8;

pub const TX1_RF_GAIN_ATTEN: c_uint = 0x13ac;

// TX_ALC_CFG_0: TX Automatic Level Control Configuration 0
// TX_ALC_LIMIT_n: TXn upper limit
// TX_ALC_CH_INIT_n: TXn channel initial transmission gain
// Unit: 0.5 dB, Range: 0 to 23.5 dB
//
pub const TX_ALC_CFG_0: c_uint = 0x13b0;

// TX_ALC_CFG_1: TX Automatic Level Control Configuration 1
// TX_TEMP_COMP:      TX Power Temperature Compensation
// Unit: 0.5 dB, Range: -10 dB to 10 dB
// TXn_GAIN_FINE:     TXn Gain Fine Adjustment
// Unit: 0.1 dB, Range: -0.8 dB to 0.7 dB
// RF_TOS_DLY:        Sets the RF_TOS_EN assertion delay after
// deassertion of PA_PE.
// Unit: 0.25 usec
// TXn_RF_GAIN_ATTEN: TXn RF gain attentuation selector
// RF_TOS_TIMEOUT:    time-out value for RF_TOS_ENABLE
// deassertion if RF_TOS_DONE is missing.
// Unit: 0.25 usec
// RF_TOS_ENABLE:     TX offset calibration enable
// ROS_BUSY_EN:       RX offset calibration busy enable
//
pub const TX_ALC_CFG_1: c_uint = 0x13b4;

// TXn_BB_GAIN_ATTEN: TXn RF Gain Attenuation Level
// Format: 5-bit signed values
// Unit: 0.5 dB, Range: -8 dB to 7 dB
//
pub const TX0_BB_GAIN_ATTEN: c_uint = 0x13c0;

pub const TX1_BB_GAIN_ATTEN: c_uint = 0x13c4;

// TX_ALC_VGA3: TX Automatic Level Correction Variable Gain Amplifier 3
pub const TX_ALC_VGA3: c_uint = 0x13c8;

// TX_PWR_CFG_7
pub const TX_PWR_CFG_7: c_uint = 0x13d4;

// bits for new 2T devices

// TX_PWR_CFG_8
pub const TX_PWR_CFG_8: c_uint = 0x13d8;

// bits for new 2T devices

// TX_PWR_CFG_9
pub const TX_PWR_CFG_9: c_uint = 0x13dc;

// bits for new 2T devices

//
// TX_TXBF_CFG:
//
pub const TX_TXBF_CFG_0: c_uint = 0x138c;
pub const TX_TXBF_CFG_1: c_uint = 0x13a4;
pub const TX_TXBF_CFG_2: c_uint = 0x13a8;
pub const TX_TXBF_CFG_3: c_uint = 0x13ac;
//
// TX_FBK_CFG_3S:
//
pub const TX_FBK_CFG_3S_0: c_uint = 0x13c4;
pub const TX_FBK_CFG_3S_1: c_uint = 0x13c8;
//
// RX_FILTER_CFG: RX configuration register.
//
pub const RX_FILTER_CFG: c_uint = 0x1400;

//
// AUTO_RSP_CFG:
// AUTORESPONDER: 0: disable, 1: enable
// BAC_ACK_POLICY: 0:long, 1:short preamble
// CTS_40_MMODE: Response CTS 40MHz duplicate mode
// CTS_40_MREF: Response CTS 40MHz duplicate mode
// AR_PREAMBLE: Auto responder preamble 0:long, 1:short preamble
// DUAL_CTS_EN: Power bit value in control frame
// ACK_CTS_PSM_BIT:Power bit value in control frame
//
pub const AUTO_RSP_CFG: c_uint = 0x1404;

//
// LEGACY_BASIC_RATE:
//
pub const LEGACY_BASIC_RATE: c_uint = 0x1408;
//
// HT_BASIC_RATE:
//
pub const HT_BASIC_RATE: c_uint = 0x140c;
//
// HT_CTRL_CFG:
//
pub const HT_CTRL_CFG: c_uint = 0x1410;
//
// SIFS_COST_CFG:
//
pub const SIFS_COST_CFG: c_uint = 0x1414;
//
// RX_PARSER_CFG:
// Set NAV for all received frames
//
pub const RX_PARSER_CFG: c_uint = 0x1418;
//
// TX_SEC_CNT0:
//
pub const TX_SEC_CNT0: c_uint = 0x1500;
//
// RX_SEC_CNT0:
//
pub const RX_SEC_CNT0: c_uint = 0x1504;
//
// CCMP_FC_MUTE:
//
pub const CCMP_FC_MUTE: c_uint = 0x1508;
//
// TXOP_HLDR_ADDR0:
//
pub const TXOP_HLDR_ADDR0: c_uint = 0x1600;
//
// TXOP_HLDR_ADDR1:
//
pub const TXOP_HLDR_ADDR1: c_uint = 0x1604;
//
// TXOP_HLDR_ET:
//
pub const TXOP_HLDR_ET: c_uint = 0x1608;
//
// QOS_CFPOLL_RA_DW0:
//
pub const QOS_CFPOLL_RA_DW0: c_uint = 0x160c;
//
// QOS_CFPOLL_RA_DW1:
//
pub const QOS_CFPOLL_RA_DW1: c_uint = 0x1610;
//
// QOS_CFPOLL_QC:
//
pub const QOS_CFPOLL_QC: c_uint = 0x1614;
//
// RX_STA_CNT0: RX PLCP error count & RX CRC error count
//
pub const RX_STA_CNT0: c_uint = 0x1700;

//
// RX_STA_CNT1: RX False CCA count & RX LONG frame count
//
pub const RX_STA_CNT1: c_uint = 0x1704;

//
// RX_STA_CNT2:
//
pub const RX_STA_CNT2: c_uint = 0x1708;

//
// TX_STA_CNT0: TX Beacon count
//
pub const TX_STA_CNT0: c_uint = 0x170c;

//
// TX_STA_CNT1: TX tx count
//
pub const TX_STA_CNT1: c_uint = 0x1710;

//
// TX_STA_CNT2: TX tx count
//
pub const TX_STA_CNT2: c_uint = 0x1714;

//
// TX_STA_FIFO: TX Result for specific PID status fifo register.
//
// This register is implemented as FIFO with 16 entries in the HW. Each
// register read fetches the next tx result. If the FIFO is full because
// it wasn't read fast enough after the according interrupt (TX_FIFO_STATUS)
// triggered, the hw seems to simply drop further tx results.
//
// VALID: 1: this tx result is valid
// 0: no valid tx result -> driver should stop reading
// PID_TYPE: The PID latched from the PID field in the TXWI, can be used
// to match a frame with its tx result (even though the PID is
// only 4 bits wide).
// PID_QUEUE: Part of PID_TYPE, this is the queue index number (0-3)
// PID_ENTRY: Part of PID_TYPE, this is the queue entry index number (1-3)
// This identification number is calculated by ((idx % 3) + 1).
// TX_SUCCESS: Indicates tx success (1) or failure (0)
// TX_AGGRE: Indicates if the frame was part of an aggregate (1) or not (0)
// TX_ACK_REQUIRED: Indicates if the frame needed to get ack'ed (1) or not (0)
// WCID: The wireless client ID.
// MCS: The tx rate used during the last transmission of this frame, be it
// successful or not.
// PHYMODE: The phymode used for the transmission.
//
pub const TX_STA_FIFO: c_uint = 0x1718;

//
// TX_AGG_CNT: Debug counter
//
pub const TX_AGG_CNT: c_uint = 0x171c;

//
// TX_AGG_CNT0:
//
pub const TX_AGG_CNT0: c_uint = 0x1720;

//
// TX_AGG_CNT1:
//
pub const TX_AGG_CNT1: c_uint = 0x1724;

//
// TX_AGG_CNT2:
//
pub const TX_AGG_CNT2: c_uint = 0x1728;

//
// TX_AGG_CNT3:
//
pub const TX_AGG_CNT3: c_uint = 0x172c;

//
// TX_AGG_CNT4:
//
pub const TX_AGG_CNT4: c_uint = 0x1730;

//
// TX_AGG_CNT5:
//
pub const TX_AGG_CNT5: c_uint = 0x1734;

//
// TX_AGG_CNT6:
//
pub const TX_AGG_CNT6: c_uint = 0x1738;

//
// TX_AGG_CNT7:
//
pub const TX_AGG_CNT7: c_uint = 0x173c;

//
// MPDU_DENSITY_CNT:
// TX_ZERO_DEL: TX zero length delimiter count
// RX_ZERO_DEL: RX zero length delimiter count
//
pub const MPDU_DENSITY_CNT: c_uint = 0x1740;

//
// Security key table memory.
//
// The pairwise key table shares some memory with the beacon frame
// buffers 6 and 7. That basically means that when beacon 6 & 7
// are used we should only use the reduced pairwise key table which
// has a maximum of 222 entries.
//
// ---------------------------------------------
// |0x4000 | Pairwise Key   | Reduced Pairwise |
// |       | Table          | Key Table        |
// |       | Size: 256 * 32 | Size: 222 * 32   |
// |0x5BC0 |                |-------------------
// |       |                | Beacon 6         |
// |0x5DC0 |                |-------------------
// |       |                | Beacon 7         |
// |0x5FC0 |                |-------------------
// |0x5FFF |                |
// --------------------------
//
// MAC_WCID_BASE: 8-bytes (use only 6 bytes) * 256 entry
// PAIRWISE_KEY_TABLE_BASE: 32-byte * 256 entry
// MAC_IVEIV_TABLE_BASE: 8-byte * 256-entry
// MAC_WCID_ATTRIBUTE_BASE: 4-byte * 256-entry
// SHARED_KEY_TABLE_BASE: 32-byte * 16-entry
// SHARED_KEY_MODE_BASE: 4-byte * 16-entry
//
pub const MAC_WCID_BASE: c_uint = 0x1800;
pub const PAIRWISE_KEY_TABLE_BASE: c_uint = 0x4000;
pub const MAC_IVEIV_TABLE_BASE: c_uint = 0x6000;
pub const MAC_WCID_ATTRIBUTE_BASE: c_uint = 0x6800;
pub const SHARED_KEY_TABLE_BASE: c_uint = 0x6c00;
pub const SHARED_KEY_MODE_BASE: c_uint = 0x7000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_wcid_entry {
    pub mac: [u8; 6],
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_key_entry {
    pub key: [u8; 16],
    pub tx_mic: [u8; 8],
    pub rx_mic: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_iveiv_entry {
    pub iv: [u8; 8],
    pub __packed: },
//
// MAC_WCID_ATTRIBUTE:
//

//
// SHARED_KEY_MODE:
//

//
// HOST-MCU communication
//
// H2M_MAILBOX_CSR: Host-to-MCU Mailbox.
// CMD_TOKEN: Command id, 0xff disable status reporting.
//
pub const H2M_MAILBOX_CSR: c_uint = 0x7010;

//
// H2M_MAILBOX_CID:
// Free slots contain 0xff. MCU will store command's token to lowest free slot.
// If all slots are occupied status will be dropped.
//
pub const H2M_MAILBOX_CID: c_uint = 0x7014;

//
// H2M_MAILBOX_STATUS:
// Command status will be saved to same slot as command id.
//
pub const H2M_MAILBOX_STATUS: c_uint = 0x701c;
//
// H2M_INT_SRC:
//
pub const H2M_INT_SRC: c_uint = 0x7024;
//
// H2M_BBP_AGENT:
//
pub const H2M_BBP_AGENT: c_uint = 0x7028;
//
// MCU_LEDCS: LED control for MCU Mailbox.
//

//
// HW_CS_CTS_BASE:
// Carrier-sense CTS frame base address.
// It's where mac stores carrier-sense frame for carrier-sense function.
//
pub const HW_CS_CTS_BASE: c_uint = 0x7700;
//
// HW_DFS_CTS_BASE:
// DFS CTS frame base address. It's where mac stores CTS frame for DFS.
//
pub const HW_DFS_CTS_BASE: c_uint = 0x7780;
//
// TXRX control registers - base address 0x3000
//
// TXRX_CSR1:
// rt2860b  UNKNOWN reg use R/O Reg Addr 0x77d0 first..
//
pub const TXRX_CSR1: c_uint = 0x77d0;
//
// HW_DEBUG_SETTING_BASE:
// since NULL frame won't be that long (256 byte)
// We steal 16 tail bytes to save debugging settings
//
pub const HW_DEBUG_SETTING_BASE: c_uint = 0x77f0;
pub const HW_DEBUG_SETTING_BASE2: c_uint = 0x7770;
//
// HW_BEACON_BASE
// In order to support maximum 8 MBSS and its maximum length
// is 512 bytes for each beacon
// Three section discontinue memory segments will be used.
// 1. The original region for BCN 0~3
// 2. Extract memory from FCE table for BCN 4~5
// 3. Extract memory from Pair-wise key table for BCN 6~7
// It occupied those memory of wcid 238~253 for BCN 6
// and wcid 222~237 for BCN 7 (see Security key table memory
// for more info).
//
// IMPORTANT NOTE: Not sure why legacy driver does this,
// but HW_BEACON_BASE7 is 0x0200 bytes below HW_BEACON_BASE6.
//
pub const HW_BEACON_BASE0: c_uint = 0x7800;
pub const HW_BEACON_BASE1: c_uint = 0x7a00;
pub const HW_BEACON_BASE2: c_uint = 0x7c00;
pub const HW_BEACON_BASE3: c_uint = 0x7e00;
pub const HW_BEACON_BASE4: c_uint = 0x7200;
pub const HW_BEACON_BASE5: c_uint = 0x7400;
pub const HW_BEACON_BASE6: c_uint = 0x5dc0;
pub const HW_BEACON_BASE7: c_uint = 0x5bc0;

//
// BBP registers.
// The wordsize of the BBP is 8 bits.
//
// BBP 1: TX Antenna & Power Control
// POWER_CTRL:
// 0 - normal,
// 1 - drop tx power by 6dBm,
// 2 - drop tx power by 12dBm,
// 3 - increase tx power by 6dBm
//

//
// BBP 3: RX Antenna
//

//
// BBP 4: Bandwidth
//

// BBP27

//
// BBP 47: Bandwidth
//

//
// BBP 49
//

//
// BBP 105:
// - bit0: detect SIG on primary channel only (on 40MHz bandwidth)
// - bit1: FEQ (Feed Forward Compensation) for independend streams
// - bit2: MLD (Maximum Likehood Detection) for 2 streams (reserved on single
// stream)
// - bit4: channel estimation updates based on remodulation of
// L-SIG and HT-SIG symbols
//

//
// BBP 109
//

// BBP 110

//
// BBP 138: Unknown
//

//
// BBP 152: Rx Ant
//

//
// BBP 254: unknown
//

//
// RFCSR registers
// The wordsize of the RFCSR is 8 bits.
//
// RFCSR 1:
//

//
// RFCSR 2:
//

//
// RFCSR 3:
//

// Bits [7-4] for RF3320 (RT3370/RT3390), on other chipsets reserved

// Bits for RF3290/RF5360/RF5362/RF5370/RF5372/RF5390/RF5392

// Bits for RF3050

//
// RFCSR 4:
// VCOCAL_EN used by MT7620
//

//
// FRCSR 5:
//

//
// RFCSR 6:
//

// bits for RF3053

//
// RFCSR 7:
//

//
// RFCSR 9:
//

//
// RFCSR 11:
//

// bits for RF3053
// TODO: verify RFCSR11_MOD usage on other chips

//
// RFCSR 12:
//

//
// RFCSR 13:
//

//
// RFCSR 15:
//

//
// RFCSR 16:
//

//
// RFCSR 17:
//

// RFCSR 18

// RFCSR 19

//
// RFCSR 20:
//

//
// RFCSR 21:
//

//
// RFCSR 22:
//

//
// RFCSR 23:
//

//
// RFCSR 24:
//

//
// RFCSR 27:
//

//
// RFCSR 28:
//

//
// RFCSR 29:
//

//
// RFCSR 30:
//

//
// RFCSR 31:
//

// RFCSR 32 bits for RF3053

// RFCSR 36 bits for RF3053

//
// RFCSR 34:
//

//
// RFCSR 38:
//

//
// RFCSR 39:
//

//
// RFCSR 41:
//

//
// RFCSR 42:
//

//
// RFCSR 49:
//

// bits for RT3593

//
// RFCSR 50:
//

// bits for RT3593

// RFCSR 51
// bits for RT3593

//
// RF registers
//
// RF 2
//

//
// RF 3
//

//
// RF 4
//

//
// EEPROM content.
// The wordsize of the EEPROM is 16 bits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2800_eeprom_word {
    EEPROM_CHIP_ID = 0,
    EEPROM_VERSION,
    EEPROM_MAC_ADDR_0,
    EEPROM_MAC_ADDR_1,
    EEPROM_MAC_ADDR_2,
    EEPROM_NIC_CONF0,
    EEPROM_NIC_CONF1,
    EEPROM_FREQ,
    EEPROM_LED_AG_CONF,
    EEPROM_LED_ACT_CONF,
    EEPROM_LED_POLARITY,
    EEPROM_NIC_CONF2,
    EEPROM_LNA,
    EEPROM_RSSI_BG,
    EEPROM_RSSI_BG2,
    EEPROM_TXMIXER_GAIN_BG,
    EEPROM_RSSI_A,
    EEPROM_RSSI_A2,
    EEPROM_TXMIXER_GAIN_A,
    EEPROM_EIRP_MAX_TX_POWER,
    EEPROM_TXPOWER_DELTA,
    EEPROM_TXPOWER_BG1,
    EEPROM_TXPOWER_BG2,
    EEPROM_TSSI_BOUND_BG1,
    EEPROM_TSSI_BOUND_BG2,
    EEPROM_TSSI_BOUND_BG3,
    EEPROM_TSSI_BOUND_BG4,
    EEPROM_TSSI_BOUND_BG5,
    EEPROM_TXPOWER_A1,
    EEPROM_TXPOWER_A2,
    EEPROM_TXPOWER_INIT,
    EEPROM_TSSI_BOUND_A1,
    EEPROM_TSSI_BOUND_A2,
    EEPROM_TSSI_BOUND_A3,
    EEPROM_TSSI_BOUND_A4,
    EEPROM_TSSI_BOUND_A5,
    EEPROM_TXPOWER_BYRATE,
    EEPROM_BBP_START,

// IDs for extended EEPROM format used by three-chain devices
    EEPROM_EXT_LNA2,
    EEPROM_EXT_TXPOWER_BG3,
    EEPROM_EXT_TXPOWER_A3,

// New values must be added before this
    EEPROM_WORD_COUNT
}

//
// EEPROM Version
//

//
// HW MAC address.
//

//
// EEPROM NIC Configuration 0
// RXPATH: 1: 1R, 2: 2R, 3: 3R
// TXPATH: 1: 1T, 2: 2T, 3: 3T
// RF_TYPE: RFIC type
//

//
// EEPROM NIC Configuration 1
// HW_RADIO: 0: disable, 1: enable
// EXTERNAL_TX_ALC: 0: disable, 1: enable
// EXTERNAL_LNA_2G: 0: disable, 1: enable
// EXTERNAL_LNA_5G: 0: disable, 1: enable
// CARDBUS_ACCEL: 0: enable, 1: disable
// BW40M_SB_2G: 0: disable, 1: enable
// BW40M_SB_5G: 0: disable, 1: enable
// WPS_PBC: 0: disable, 1: enable
// BW40M_2G: 0: enable, 1: disable
// BW40M_5G: 0: enable, 1: disable
// BROADBAND_EXT_LNA: 0: disable, 1: enable
// ANT_DIVERSITY: 00: Disable, 01: Diversity,
// 10: Main antenna, 11: Aux antenna
// INTERNAL_TX_ALC: 0: disable, 1: enable
// BT_COEXIST: 0: disable, 1: enable
// DAC_TEST: 0: disable, 1: enable
// EXTERNAL_TX0_PA: 0: disable, 1: enable (only on RT3352)
// EXTERNAL_TX1_PA: 0: disable, 1: enable (only on RT3352)
//

//
// EEPROM frequency
//

//
// EEPROM LED
// POLARITY_RDY_G: Polarity RDY_G setting.
// POLARITY_RDY_A: Polarity RDY_A setting.
// POLARITY_ACT: Polarity ACT setting.
// POLARITY_GPIO_0: Polarity GPIO0 setting.
// POLARITY_GPIO_1: Polarity GPIO1 setting.
// POLARITY_GPIO_2: Polarity GPIO2 setting.
// POLARITY_GPIO_3: Polarity GPIO3 setting.
// POLARITY_GPIO_4: Polarity GPIO4 setting.
// LED_MODE: Led mode.
//

//
// EEPROM NIC Configuration 2
// RX_STREAM: 0: Reserved, 1: 1 Stream, 2: 2 Stream
// TX_STREAM: 0: Reserved, 1: 1 Stream, 2: 2 Stream
// CRYSTAL: 00: Reserved, 01: One crystal, 10: Two crystal, 11: Reserved
//

//
// EEPROM LNA
//

//
// EEPROM RSSI BG offset
//

//
// EEPROM RSSI BG2 offset
//

//
// EEPROM TXMIXER GAIN BG offset (note overlaps with EEPROM RSSI BG2).
//

//
// EEPROM RSSI A offset
//

//
// EEPROM RSSI A2 offset
//

//
// EEPROM TXMIXER GAIN A offset (note overlaps with EEPROM RSSI A2).
//

//
// EEPROM EIRP Maximum TX power values(unit: dbm)
//

//
// EEPROM TXpower delta: 20MHZ AND 40 MHZ use different power.
// This is delta in 40MHZ.
// VALUE: Tx Power dalta value, MAX=4(unit: dbm)
// TYPE: 1: Plus the delta value, 0: minus the delta value
// ENABLE: enable tx power compensation for 40BW
//

//
// EEPROM TXPOWER 802.11BG
//
pub const EEPROM_TXPOWER_BG_SIZE: c_int = 7;

//
// EEPROM temperature compensation boundaries 802.11BG
// MINUS4: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -4)
// MINUS3: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -3)
//

//
// EEPROM temperature compensation boundaries 802.11BG
// MINUS2: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -2)
// MINUS1: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -1)
//

//
// EEPROM temperature compensation boundaries 802.11BG
// REF: Reference TSSI value, no tx power changes needed
// PLUS1: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 1)
//

//
// EEPROM temperature compensation boundaries 802.11BG
// PLUS2: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 2)
// PLUS3: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 3)
//

//
// EEPROM temperature compensation boundaries 802.11BG
// PLUS4: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 4)
// AGC_STEP: Temperature compensation step.
//

//
// EEPROM TXPOWER 802.11A
//
pub const EEPROM_TXPOWER_A_SIZE: c_int = 6;

// EEPROM_TXPOWER_{A,G} fields for RT3593

//
// EEPROM temperature compensation boundaries 802.11A
// MINUS4: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -4)
// MINUS3: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -3)
//

//
// EEPROM temperature compensation boundaries 802.11A
// MINUS2: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -2)
// MINUS1: If the actual TSSI is below this boundary, tx power needs to be
// reduced by (agc_step * -1)
//

//
// EEPROM temperature compensation boundaries 802.11A
// REF: Reference TSSI value, no tx power changes needed
// PLUS1: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 1)
//

//
// EEPROM temperature compensation boundaries 802.11A
// PLUS2: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 2)
// PLUS3: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 3)
//

//
// EEPROM temperature compensation boundaries 802.11A
// PLUS4: If the actual TSSI is above this boundary, tx power needs to be
// increased by (agc_step * 4)
// AGC_STEP: Temperature compensation step.
//

//
// EEPROM TXPOWER by rate: tx power per tx rate for HT20 mode
//
pub const EEPROM_TXPOWER_BYRATE_SIZE: c_int = 9;

//
// EEPROM BBP.
//
pub const EEPROM_BBP_SIZE: c_int = 16;

// EEPROM_EXT_LNA2

//
// EEPROM IQ Calibration, unlike other entries those are byte addresses.
//
pub const EEPROM_IQ_GAIN_CAL_TX0_2G: c_uint = 0x130;
pub const EEPROM_IQ_PHASE_CAL_TX0_2G: c_uint = 0x131;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX0_2G: c_uint = 0x132;
pub const EEPROM_IQ_GAIN_CAL_TX1_2G: c_uint = 0x133;
pub const EEPROM_IQ_PHASE_CAL_TX1_2G: c_uint = 0x134;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX1_2G: c_uint = 0x135;
pub const EEPROM_IQ_GAIN_CAL_RX0_2G: c_uint = 0x136;
pub const EEPROM_IQ_PHASE_CAL_RX0_2G: c_uint = 0x137;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX0_2G: c_uint = 0x138;
pub const EEPROM_IQ_GAIN_CAL_RX1_2G: c_uint = 0x139;
pub const EEPROM_IQ_PHASE_CAL_RX1_2G: c_uint = 0x13A;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX1_2G: c_uint = 0x13B;
pub const EEPROM_RF_IQ_COMPENSATION_CONTROL: c_uint = 0x13C;
pub const EEPROM_RF_IQ_IMBALANCE_COMPENSATION_CONTROL: c_uint = 0x13D;
pub const EEPROM_IQ_GAIN_CAL_TX0_CH36_TO_CH64_5G: c_uint = 0x144;
pub const EEPROM_IQ_PHASE_CAL_TX0_CH36_TO_CH64_5G: c_uint = 0x145;

pub const EEPROM_IQ_PHASE_CAL_TX0_CH100_TO_CH138_5G: c_uint = 0x147;
pub const EEPROM_IQ_GAIN_CAL_TX0_CH140_TO_CH165_5G: c_uint = 0x148;
pub const EEPROM_IQ_PHASE_CAL_TX0_CH140_TO_CH165_5G: c_uint = 0x149;
pub const EEPROM_IQ_GAIN_CAL_TX1_CH36_TO_CH64_5G: c_uint = 0x14A;
pub const EEPROM_IQ_PHASE_CAL_TX1_CH36_TO_CH64_5G: c_uint = 0x14B;

pub const EEPROM_IQ_PHASE_CAL_TX1_CH100_TO_CH138_5G: c_uint = 0x14D;
pub const EEPROM_IQ_GAIN_CAL_TX1_CH140_TO_CH165_5G: c_uint = 0x14E;
pub const EEPROM_IQ_PHASE_CAL_TX1_CH140_TO_CH165_5G: c_uint = 0x14F;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX0_CH36_TO_CH64_5G: c_uint = 0x150;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX1_CH36_TO_CH64_5G: c_uint = 0x151;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX0_CH100_TO_CH138_5G: c_uint = 0x152;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX1_CH100_TO_CH138_5G: c_uint = 0x153;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX0_CH140_TO_CH165_5G: c_uint = 0x154;
pub const EEPROM_IQ_GROUPDELAY_CAL_TX1_CH140_TO_CH165_5G: c_uint = 0x155;
pub const EEPROM_IQ_GAIN_CAL_RX0_CH36_TO_CH64_5G: c_uint = 0x156;
pub const EEPROM_IQ_PHASE_CAL_RX0_CH36_TO_CH64_5G: c_uint = 0x157;

pub const EEPROM_IQ_PHASE_CAL_RX0_CH100_TO_CH138_5G: c_uint = 0x159;
pub const EEPROM_IQ_GAIN_CAL_RX0_CH140_TO_CH165_5G: c_uint = 0x15A;
pub const EEPROM_IQ_PHASE_CAL_RX0_CH140_TO_CH165_5G: c_uint = 0x15B;
pub const EEPROM_IQ_GAIN_CAL_RX1_CH36_TO_CH64_5G: c_uint = 0x15C;
pub const EEPROM_IQ_PHASE_CAL_RX1_CH36_TO_CH64_5G: c_uint = 0x15D;

pub const EEPROM_IQ_PHASE_CAL_RX1_CH100_TO_CH138_5G: c_uint = 0x15F;
pub const EEPROM_IQ_GAIN_CAL_RX1_CH140_TO_CH165_5G: c_uint = 0x160;
pub const EEPROM_IQ_PHASE_CAL_RX1_CH140_TO_CH165_5G: c_uint = 0x161;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX0_CH36_TO_CH64_5G: c_uint = 0x162;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX1_CH36_TO_CH64_5G: c_uint = 0x163;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX0_CH100_TO_CH138_5G: c_uint = 0x164;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX1_CH100_TO_CH138_5G: c_uint = 0x165;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX0_CH140_TO_CH165_5G: c_uint = 0x166;
pub const EEPROM_IQ_GROUPDELAY_CAL_RX1_CH140_TO_CH165_5G: c_uint = 0x167;
//
// MCU mailbox commands.
// MCU_SLEEP - go to power-save mode.
// arg1: 1: save as much power as possible, 0: save less power.
// status: 1: success, 2: already asleep,
// 3: maybe MAC is busy so can't finish this task.
// MCU_RADIO_OFF
// arg0: 0: do power-saving, NOT turn off radio.
//
pub const MCU_SLEEP: c_uint = 0x30;
pub const MCU_WAKEUP: c_uint = 0x31;
pub const MCU_RADIO_OFF: c_uint = 0x35;
pub const MCU_CURRENT: c_uint = 0x36;
pub const MCU_LED: c_uint = 0x50;
pub const MCU_LED_STRENGTH: c_uint = 0x51;
pub const MCU_LED_AG_CONF: c_uint = 0x52;
pub const MCU_LED_ACT_CONF: c_uint = 0x53;
pub const MCU_LED_LED_POLARITY: c_uint = 0x54;
pub const MCU_RADAR: c_uint = 0x60;
pub const MCU_BOOT_SIGNAL: c_uint = 0x72;

pub const MCU_FREQ_OFFSET: c_uint = 0x74;
pub const MCU_BBP_SIGNAL: c_uint = 0x80;
pub const MCU_POWER_SAVE: c_uint = 0x83;
pub const MCU_BAND_SELECT: c_uint = 0x91;
//
// MCU mailbox tokens
//
pub const TOKEN_SLEEP: c_int = 1;
pub const TOKEN_RADIO_OFF: c_int = 2;
pub const TOKEN_WAKEUP: c_int = 3;
//
// DMA descriptor defines.
//

//
// TX WI structure
//
// Word0
// FRAG: 1 To inform TKIP engine this is a fragment.
// MIMO_PS: The remote peer is in dynamic MIMO-PS mode
// TX_OP: 0:HT TXOP rule , 1:PIFS TX ,2:Backoff, 3:sifs
// BW: Channel bandwidth 0:20MHz, 1:40 MHz (for legacy rates this will
// duplicate the frame to both channels).
// STBC: 1: STBC support MCS =0-7, 2,3 : RESERVED
// AMPDU: 1: this frame is eligible for AMPDU aggregation, the hw will
// aggregate consecutive frames with the same RA and QoS TID. If
// a frame A with the same RA and QoS TID but AMPDU=0 is queued
// directly after a frame B with AMPDU=1, frame A might still
// get aggregated into the AMPDU started by frame B. So, setting
// AMPDU to 0 does _not_ necessarily mean the frame is sent as
// MPDU, it can still end up in an AMPDU if the previous frame
// was tagged as AMPDU.
//

//
// Word1
// ACK: 0: No Ack needed, 1: Ack needed
// NSEQ: 0: Don't assign hw sequence number, 1: Assign hw sequence number
// BW_WIN_SIZE: BA windows size of the recipient
// WIRELESS_CLI_ID: Client ID for WCID table access
// MPDU_TOTAL_BYTE_COUNT: Length of 802.11 frame
// PACKETID: Will be latched into the TX_STA_FIFO register once the according
// frame was processed. If multiple frames are aggregated together
// (AMPDU==1) the reported tx status will always contain the packet
// id of the first frame. 0: Don't report tx status for this frame.
// PACKETID_QUEUE: Part of PACKETID, This is the queue index (0-3)
// PACKETID_ENTRY: Part of PACKETID, THis is the queue entry index (1-3)
// This identification number is calculated by ((idx % 3) + 1).
// The (+1) is required to prevent PACKETID to become 0.
//

//
// Word2
//

//
// Word3
//

//
// RX WI structure
//
// Word0
//

//
// Word1
//

//
// Word2
//

//
// Word3
//

//
// Macros for converting txpower from EEPROM to mac80211 value
// and from mac80211 value to register value.
//
pub const MIN_G_TXPOWER: c_int = 0;

pub const MAX_G_TXPOWER: c_int = 31;
pub const MAX_A_TXPOWER: c_int = 15;
pub const DEFAULT_TXPOWER: c_int = 5;
pub const MIN_A_TXPOWER_3593: c_int = 0;
pub const MAX_A_TXPOWER_3593: c_int = 31;

//
// Board's maximun TX power limitation
//
pub const EIRP_MAX_TX_POWER_LIMIT: c_uint = 0x50;
//
// Number of TBTT intervals after which we have to adjust
// the hw beacon timer.
//
pub const BCN_TBTT_OFFSET: c_int = 64;
// Watchdog type mask

