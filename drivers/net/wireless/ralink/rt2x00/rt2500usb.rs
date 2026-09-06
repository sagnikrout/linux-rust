//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2500usb.h
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
pub const RF2522: c_uint = 0x0000;
pub const RF2523: c_uint = 0x0001;
pub const RF2524: c_uint = 0x0002;
pub const RF2525: c_uint = 0x0003;
pub const RF2525E: c_uint = 0x0005;
pub const RF5222: c_uint = 0x0010;
//
// RT2570 version
//
pub const RT2570_VERSION_B: c_int = 2;
pub const RT2570_VERSION_C: c_int = 3;
pub const RT2570_VERSION_D: c_int = 4;
//
// Signal information.
// Default offset is required for RSSI <-> dBm conversion.
//
pub const DEFAULT_RSSI_OFFSET: c_int = 120;
//
// Register layout information.
//
pub const CSR_REG_BASE: c_uint = 0x0400;
pub const CSR_REG_SIZE: c_uint = 0x0100;
pub const EEPROM_BASE: c_uint = 0x0000;
pub const EEPROM_SIZE: c_uint = 0x006e;
pub const BBP_BASE: c_uint = 0x0000;
pub const BBP_SIZE: c_uint = 0x0060;
pub const RF_BASE: c_uint = 0x0004;
pub const RF_SIZE: c_uint = 0x0010;
//
// Number of TX queues.
//
pub const NUM_TX_QUEUES: c_int = 2;
//
// Control/Status Registers(CSR).
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// MAC_CSR0: ASIC revision number.
//
pub const MAC_CSR0: c_uint = 0x0400;
//
// MAC_CSR1: System control.
// SOFT_RESET: Software reset, 1: reset, 0: normal.
// BBP_RESET: Hardware reset, 1: reset, 0, release.
// HOST_READY: Host ready after initialization.
//
pub const MAC_CSR1: c_uint = 0x0402;

//
// MAC_CSR2: STA MAC register 0.
//
pub const MAC_CSR2: c_uint = 0x0404;

//
// MAC_CSR3: STA MAC register 1.
//
pub const MAC_CSR3: c_uint = 0x0406;

//
// MAC_CSR4: STA MAC register 2.
//

//
// MAC_CSR5: BSSID register 0.
//
pub const MAC_CSR5: c_uint = 0x040a;

//
// MAC_CSR6: BSSID register 1.
//
pub const MAC_CSR6: c_uint = 0x040c;

//
// MAC_CSR7: BSSID register 2.
//
pub const MAC_CSR7: c_uint = 0x040e;

//
// MAC_CSR8: Max frame length.
//
pub const MAC_CSR8: c_uint = 0x0410;

//
// Misc MAC_CSR registers.
// MAC_CSR9: Timer control.
// MAC_CSR10: Slot time.
// MAC_CSR11: SIFS.
// MAC_CSR12: EIFS.
// MAC_CSR13: Power mode0.
// MAC_CSR14: Power mode1.
// MAC_CSR15: Power saving transition0
// MAC_CSR16: Power saving transition1
//
pub const MAC_CSR9: c_uint = 0x0412;
pub const MAC_CSR10: c_uint = 0x0414;
pub const MAC_CSR11: c_uint = 0x0416;
pub const MAC_CSR12: c_uint = 0x0418;
pub const MAC_CSR13: c_uint = 0x041a;
pub const MAC_CSR14: c_uint = 0x041c;
pub const MAC_CSR15: c_uint = 0x041e;
pub const MAC_CSR16: c_uint = 0x0420;
//
// MAC_CSR17: Manual power control / status register.
// Allowed state: 0 deep_sleep, 1: sleep, 2: standby, 3: awake.
// SET_STATE: Set state. Write 1 to trigger, self cleared.
// BBP_DESIRE_STATE: BBP desired state.
// RF_DESIRE_STATE: RF desired state.
// BBP_CURRENT_STATE: BBP current state.
// RF_CURRENT_STATE: RF current state.
// PUT_TO_SLEEP: Put to sleep. Write 1 to trigger, self cleared.
//
pub const MAC_CSR17: c_uint = 0x0422;

//
// MAC_CSR18: Wakeup timer register.
// DELAY_AFTER_BEACON: Delay after Tbcn expired in units of 1/16 TU.
// BEACONS_BEFORE_WAKEUP: Number of beacon before wakeup.
// AUTO_WAKE: Enable auto wakeup / sleep mechanism.
//
pub const MAC_CSR18: c_uint = 0x0424;

//
// MAC_CSR19: GPIO control register.
// MAC_CSR19_VALx: GPIO value
// MAC_CSR19_DIRx: GPIO direction: 0 = input; 1 = output
//
pub const MAC_CSR19: c_uint = 0x0426;

//
// MAC_CSR20: LED control register.
// ACTIVITY: 0: idle, 1: active.
// LINK: 0: linkoff, 1: linkup.
// ACTIVITY_POLARITY: 0: active low, 1: active high.
//
pub const MAC_CSR20: c_uint = 0x0428;

//
// MAC_CSR21: LED control register.
// ON_PERIOD: On period, default 70ms.
// OFF_PERIOD: Off period, default 30ms.
//
pub const MAC_CSR21: c_uint = 0x042a;

//
// MAC_CSR22: Collision window control register.
//
pub const MAC_CSR22: c_uint = 0x042c;
//
// Transmit related CSRs.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// TXRX_CSR0: Security control register.
//
pub const TXRX_CSR0: c_uint = 0x0440;

//
// TXRX_CSR1: TX configuration.
// ACK_TIMEOUT: ACK Timeout in unit of 1-us.
// TSF_OFFSET: TSF offset in MAC header.
// AUTO_SEQUENCE: Let ASIC control frame sequence number.
//
pub const TXRX_CSR1: c_uint = 0x0442;

//
// TXRX_CSR2: RX control.
// DISABLE_RX: Disable rx engine.
// DROP_CRC: Drop crc error.
// DROP_PHYSICAL: Drop physical error.
// DROP_CONTROL: Drop control frame.
// DROP_NOT_TO_ME: Drop not to me unicast frame.
// DROP_TODS: Drop frame tods bit is true.
// DROP_VERSION_ERROR: Drop version error frame.
// DROP_MCAST: Drop multicast frames.
// DROP_BCAST: Drop broadcast frames.
//
pub const TXRX_CSR2: c_uint = 0x0444;

//
// RX BBP ID registers
// TXRX_CSR3: CCK RX BBP ID.
// TXRX_CSR4: OFDM RX BBP ID.
//
pub const TXRX_CSR3: c_uint = 0x0446;
pub const TXRX_CSR4: c_uint = 0x0448;
//
// TXRX_CSR5: CCK TX BBP ID0.
//
pub const TXRX_CSR5: c_uint = 0x044a;

//
// TXRX_CSR6: CCK TX BBP ID1.
//
pub const TXRX_CSR6: c_uint = 0x044c;

//
// TXRX_CSR7: OFDM TX BBP ID0.
//
pub const TXRX_CSR7: c_uint = 0x044e;

//
// TXRX_CSR8: OFDM TX BBP ID1.
//
pub const TXRX_CSR8: c_uint = 0x0450;

//
// TXRX_CSR9: TX ACK time-out.
//
pub const TXRX_CSR9: c_uint = 0x0452;
//
// TXRX_CSR10: Auto responder control.
//
pub const TXRX_CSR10: c_uint = 0x0454;

//
// TXRX_CSR11: Auto responder basic rate.
//
pub const TXRX_CSR11: c_uint = 0x0456;
//
// ACK/CTS time registers.
//
pub const TXRX_CSR12: c_uint = 0x0458;
pub const TXRX_CSR13: c_uint = 0x045a;
pub const TXRX_CSR14: c_uint = 0x045c;
pub const TXRX_CSR15: c_uint = 0x045e;
pub const TXRX_CSR16: c_uint = 0x0460;
pub const TXRX_CSR17: c_uint = 0x0462;
//
// TXRX_CSR18: Synchronization control register.
//
pub const TXRX_CSR18: c_uint = 0x0464;

//
// TXRX_CSR19: Synchronization control register.
// TSF_COUNT: Enable TSF auto counting.
// TSF_SYNC: Tsf sync, 0: disable, 1: infra, 2: ad-hoc/master mode.
// TBCN: Enable Tbcn with reload value.
// BEACON_GEN: Enable beacon generator.
//
pub const TXRX_CSR19: c_uint = 0x0466;

//
// TXRX_CSR20: Tx BEACON offset time control register.
// OFFSET: In units of usec.
// BCN_EXPECT_WINDOW: Default: 2^CWmin
//
pub const TXRX_CSR20: c_uint = 0x0468;

//
// TXRX_CSR21
//
pub const TXRX_CSR21: c_uint = 0x046a;
//
// Encryption related CSRs.
//
// SEC_CSR0: Shared key 0, word 0
// SEC_CSR1: Shared key 0, word 1
// SEC_CSR2: Shared key 0, word 2
// SEC_CSR3: Shared key 0, word 3
// SEC_CSR4: Shared key 0, word 4
// SEC_CSR5: Shared key 0, word 5
// SEC_CSR6: Shared key 0, word 6
// SEC_CSR7: Shared key 0, word 7
//
pub const SEC_CSR0: c_uint = 0x0480;
pub const SEC_CSR1: c_uint = 0x0482;
pub const SEC_CSR2: c_uint = 0x0484;
pub const SEC_CSR3: c_uint = 0x0486;
pub const SEC_CSR4: c_uint = 0x0488;
pub const SEC_CSR5: c_uint = 0x048a;
pub const SEC_CSR6: c_uint = 0x048c;
pub const SEC_CSR7: c_uint = 0x048e;
//
// SEC_CSR8: Shared key 1, word 0
// SEC_CSR9: Shared key 1, word 1
// SEC_CSR10: Shared key 1, word 2
// SEC_CSR11: Shared key 1, word 3
// SEC_CSR12: Shared key 1, word 4
// SEC_CSR13: Shared key 1, word 5
// SEC_CSR14: Shared key 1, word 6
// SEC_CSR15: Shared key 1, word 7
//
pub const SEC_CSR8: c_uint = 0x0490;
pub const SEC_CSR9: c_uint = 0x0492;
pub const SEC_CSR10: c_uint = 0x0494;
pub const SEC_CSR11: c_uint = 0x0496;
pub const SEC_CSR12: c_uint = 0x0498;
pub const SEC_CSR13: c_uint = 0x049a;
pub const SEC_CSR14: c_uint = 0x049c;
pub const SEC_CSR15: c_uint = 0x049e;
//
// SEC_CSR16: Shared key 2, word 0
// SEC_CSR17: Shared key 2, word 1
// SEC_CSR18: Shared key 2, word 2
// SEC_CSR19: Shared key 2, word 3
// SEC_CSR20: Shared key 2, word 4
// SEC_CSR21: Shared key 2, word 5
// SEC_CSR22: Shared key 2, word 6
// SEC_CSR23: Shared key 2, word 7
//
pub const SEC_CSR16: c_uint = 0x04a0;
pub const SEC_CSR17: c_uint = 0x04a2;

pub const SEC_CSR19: c_uint = 0x04a6;
pub const SEC_CSR20: c_uint = 0x04a8;
pub const SEC_CSR21: c_uint = 0x04aa;
pub const SEC_CSR22: c_uint = 0x04ac;
pub const SEC_CSR23: c_uint = 0x04ae;
//
// SEC_CSR24: Shared key 3, word 0
// SEC_CSR25: Shared key 3, word 1
// SEC_CSR26: Shared key 3, word 2
// SEC_CSR27: Shared key 3, word 3
// SEC_CSR28: Shared key 3, word 4
// SEC_CSR29: Shared key 3, word 5
// SEC_CSR30: Shared key 3, word 6
// SEC_CSR31: Shared key 3, word 7
//
pub const SEC_CSR24: c_uint = 0x04b0;
pub const SEC_CSR25: c_uint = 0x04b2;
pub const SEC_CSR26: c_uint = 0x04b4;
pub const SEC_CSR27: c_uint = 0x04b6;
pub const SEC_CSR28: c_uint = 0x04b8;
pub const SEC_CSR29: c_uint = 0x04ba;
pub const SEC_CSR30: c_uint = 0x04bc;
pub const SEC_CSR31: c_uint = 0x04be;

//
// PHY control registers.
//
// PHY_CSR0: RF switching timing control.
//
pub const PHY_CSR0: c_uint = 0x04c0;
//
// PHY_CSR1: TX PA configuration.
//
pub const PHY_CSR1: c_uint = 0x04c2;
//
// MAC configuration registers.
//
// PHY_CSR2: TX MAC configuration.
// NOTE: Both register fields are complete dummy,
// documentation and legacy drivers are unclear un
// what this register means or what fields exists.
//
pub const PHY_CSR2: c_uint = 0x04c4;

//
// PHY_CSR3: RX MAC configuration.
//
pub const PHY_CSR3: c_uint = 0x04c6;
//
// PHY_CSR4: Interface configuration.
//
pub const PHY_CSR4: c_uint = 0x04c8;

//
// BBP pre-TX registers.
// PHY_CSR5: BBP pre-TX CCK.
//
pub const PHY_CSR5: c_uint = 0x04ca;

//
// BBP pre-TX registers.
// PHY_CSR6: BBP pre-TX OFDM.
//
pub const PHY_CSR6: c_uint = 0x04cc;

//
// PHY_CSR7: BBP access register 0.
// BBP_DATA: BBP data.
// BBP_REG_ID: BBP register ID.
// BBP_READ_CONTROL: 0: write, 1: read.
//
pub const PHY_CSR7: c_uint = 0x04ce;

//
// PHY_CSR8: BBP access register 1.
// BBP_BUSY: ASIC is busy execute BBP programming.
//
pub const PHY_CSR8: c_uint = 0x04d0;

//
// PHY_CSR9: RF access register.
// RF_VALUE: Register value + id to program into rf/if.
//
pub const PHY_CSR9: c_uint = 0x04d2;

//
// PHY_CSR10: RF access register.
// RF_VALUE: Register value + id to program into rf/if.
// RF_NUMBER_OF_BITS: Number of bits used in value (i:20, rfmd:22).
// RF_IF_SELECT: Chip to program: 0: rf, 1: if.
// RF_PLL_LD: Rf pll_ld status.
// RF_BUSY: 1: asic is busy execute rf programming.
//
pub const PHY_CSR10: c_uint = 0x04d4;

//
// STA_CSR0: FCS error count.
// FCS_ERROR: FCS error count, cleared when read.
//
pub const STA_CSR0: c_uint = 0x04e0;

//
// STA_CSR1: PLCP error count.
//
pub const STA_CSR1: c_uint = 0x04e2;
//
// STA_CSR2: LONG error count.
//
pub const STA_CSR2: c_uint = 0x04e4;
//
// STA_CSR3: CCA false alarm.
// FALSE_CCA_ERROR: False CCA error count, cleared when read.
//
pub const STA_CSR3: c_uint = 0x04e6;

//
// STA_CSR4: RX FIFO overflow.
//
pub const STA_CSR4: c_uint = 0x04e8;
//
// STA_CSR5: Beacon sent counter.
//
pub const STA_CSR5: c_uint = 0x04ea;
//
// Statistics registers
//
pub const STA_CSR6: c_uint = 0x04ec;
pub const STA_CSR7: c_uint = 0x04ee;
pub const STA_CSR8: c_uint = 0x04f0;
pub const STA_CSR9: c_uint = 0x04f2;
pub const STA_CSR10: c_uint = 0x04f4;
//
// BBP registers.
// The wordsize of the BBP is 8 bits.
//
// R2: TX antenna control
//

//
// R14: RX antenna control
//

//
// RF registers.
//
// RF 1
//

//
// RF 3
//

//
// EEPROM contents.
//
// HW MAC address.
//
pub const EEPROM_MAC_ADDR_0: c_uint = 0x0002;

pub const EEPROM_MAC_ADDR1: c_uint = 0x0003;

pub const EEPROM_MAC_ADDR_2: c_uint = 0x0004;

//
// EEPROM antenna.
// ANTENNA_NUM: Number of antenna's.
// TX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// RX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// LED_MODE: 0: default, 1: TX/RX activity, 2: Single (ignore link), 3: rsvd.
// DYN_TXAGC: Dynamic TX AGC control.
// HARDWARE_RADIO: 1: Hardware controlled radio. Read GPIO0.
// RF_TYPE: Rf_type of this adapter.
//
pub const EEPROM_ANTENNA: c_uint = 0x000b;

//
// EEPROM NIC config.
// CARDBUS_ACCEL: 0: enable, 1: disable.
// DYN_BBP_TUNE: 0: enable, 1: disable.
// CCK_TX_POWER: CCK TX power compensation.
//
pub const EEPROM_NIC: c_uint = 0x000c;

//
// EEPROM geography.
// GEO: Default geography setting for device.
//
pub const EEPROM_GEOGRAPHY: c_uint = 0x000d;

//
// EEPROM BBP.
//
pub const EEPROM_BBP_START: c_uint = 0x000e;
pub const EEPROM_BBP_SIZE: c_int = 16;

//
// EEPROM TXPOWER
//
pub const EEPROM_TXPOWER_START: c_uint = 0x001e;
pub const EEPROM_TXPOWER_SIZE: c_int = 7;

//
// EEPROM Tuning threshold
//
pub const EEPROM_BBPTUNE: c_uint = 0x0030;

//
// EEPROM BBP R24 Tuning.
//
pub const EEPROM_BBPTUNE_R24: c_uint = 0x0031;

//
// EEPROM BBP R25 Tuning.
//
pub const EEPROM_BBPTUNE_R25: c_uint = 0x0032;

//
// EEPROM BBP R24 Tuning.
//
pub const EEPROM_BBPTUNE_R61: c_uint = 0x0033;

//
// EEPROM BBP VGC Tuning.
//
pub const EEPROM_BBPTUNE_VGC: c_uint = 0x0034;

//
// EEPROM BBP R17 Tuning.
//
pub const EEPROM_BBPTUNE_R17: c_uint = 0x0035;

//
// RSSI <-> dBm offset calibration
//
pub const EEPROM_CALIBRATE_OFFSET: c_uint = 0x0036;

//
// DMA descriptor defines.
//

//
// TX descriptor format for TX, PRIO, ATIM and Beacon Ring.
//
// Word0
//

//
// Word1
//

//
// Word2: PLCP information
//

//
// Word3
//

//
// Word4
//

//
// RX descriptor format for RX Ring.
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
pub const MIN_TXPOWER: c_int = 0;
pub const MAX_TXPOWER: c_int = 31;
pub const DEFAULT_TXPOWER: c_int = 24;

