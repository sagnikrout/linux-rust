//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt73usb.h
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
pub const RF5226: c_uint = 0x0001;
pub const RF2528: c_uint = 0x0002;
pub const RF5225: c_uint = 0x0003;
pub const RF2527: c_uint = 0x0004;
//
// Signal information.
// Default offset is required for RSSI <-> dBm conversion.
//
pub const DEFAULT_RSSI_OFFSET: c_int = 120;
//
// Register layout information.
//
pub const CSR_REG_BASE: c_uint = 0x3000;
pub const CSR_REG_SIZE: c_uint = 0x04b0;
pub const EEPROM_BASE: c_uint = 0x0000;
pub const EEPROM_SIZE: c_uint = 0x0100;
pub const BBP_BASE: c_uint = 0x0000;
pub const BBP_SIZE: c_uint = 0x0080;
pub const RF_BASE: c_uint = 0x0004;
pub const RF_SIZE: c_uint = 0x0010;
//
// Number of TX queues.
//
pub const NUM_TX_QUEUES: c_int = 4;
//
// USB registers.
//
// MCU_LEDCS: LED control for MCU Mailbox.
//

//
// 8051 firmware image.
//

pub const FIRMWARE_IMAGE_BASE: c_uint = 0x0800;
//
// Security key table memory.
// 16 entries 32-byte for shared key table
// 64 entries 32-byte for pairwise key table
// 64 entries 8-byte for pairwise ta key table
//
pub const SHARED_KEY_TABLE_BASE: c_uint = 0x1000;
pub const PAIRWISE_KEY_TABLE_BASE: c_uint = 0x1200;
pub const PAIRWISE_TA_TABLE_BASE: c_uint = 0x1a00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_key_entry {
    pub key: [u8; 16],
    pub tx_mic: [u8; 8],
    pub rx_mic: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_pairwise_ta_entry {
    pub address: [u8; 6],
    pub cipher: u8,
    pub reserved: u8,
    pub __packed: },
//
// Since NULL frame won't be that long (256 byte),
// We steal 16 tail bytes to save debugging settings.
//
pub const HW_DEBUG_SETTING_BASE: c_uint = 0x2bf0;
//
// On-chip BEACON frame space.
//
pub const HW_BEACON_BASE0: c_uint = 0x2400;
pub const HW_BEACON_BASE1: c_uint = 0x2500;
pub const HW_BEACON_BASE2: c_uint = 0x2600;
pub const HW_BEACON_BASE3: c_uint = 0x2700;

//
// MAC Control/Status Registers(CSR).
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// MAC_CSR0: ASIC revision number.
//
pub const MAC_CSR0: c_uint = 0x3000;

//
// MAC_CSR1: System control register.
// SOFT_RESET: Software reset bit, 1: reset, 0: normal.
// BBP_RESET: Hardware reset BBP.
// HOST_READY: Host is ready after initialization, 1: ready.
//
pub const MAC_CSR1: c_uint = 0x3004;

//
// MAC_CSR2: STA MAC register 0.
//
pub const MAC_CSR2: c_uint = 0x3008;

//
// MAC_CSR3: STA MAC register 1.
// UNICAST_TO_ME_MASK:
// Used to mask off bits from byte 5 of the MAC address
// to determine the UNICAST_TO_ME bit for RX frames.
// The full mask is complemented by BSS_ID_MASK:
// MASK = BSS_ID_MASK & UNICAST_TO_ME_MASK
//
pub const MAC_CSR3: c_uint = 0x300c;

//
// MAC_CSR4: BSSID register 0.
//
pub const MAC_CSR4: c_uint = 0x3010;

//
// MAC_CSR5: BSSID register 1.
// BSS_ID_MASK:
// This mask is used to mask off bits 0 and 1 of byte 5 of the
// BSSID. This will make sure that those bits will be ignored
// when determining the MY_BSS of RX frames.
// 0: 1-BSSID mode (BSS index = 0)
// 1: 2-BSSID mode (BSS index: Byte5, bit 0)
// 2: 2-BSSID mode (BSS index: byte5, bit 1)
// 3: 4-BSSID mode (BSS index: byte5, bit 0 - 1)
//
pub const MAC_CSR5: c_uint = 0x3014;

//
// MAC_CSR6: Maximum frame length register.
//
pub const MAC_CSR6: c_uint = 0x3018;

//
// MAC_CSR7: Reserved
//
pub const MAC_CSR7: c_uint = 0x301c;
//
// MAC_CSR8: SIFS/EIFS register.
// All units are in US.
//
pub const MAC_CSR8: c_uint = 0x3020;

//
// MAC_CSR9: Back-Off control register.
// SLOT_TIME: Slot time, default is 20us for 802.11BG.
// CWMIN: Bit for Cwmin. default Cwmin is 31 (2^5 - 1).
// CWMAX: Bit for Cwmax, default Cwmax is 1023 (2^10 - 1).
// CW_SELECT: 1: CWmin/Cwmax select from register, 0:select from TxD.
//
pub const MAC_CSR9: c_uint = 0x3024;

//
// MAC_CSR10: Power state configuration.
//
pub const MAC_CSR10: c_uint = 0x3028;
//
// MAC_CSR11: Power saving transition time register.
// DELAY_AFTER_TBCN: Delay after Tbcn expired in units of TU.
// TBCN_BEFORE_WAKEUP: Number of beacon before wakeup.
// WAKEUP_LATENCY: In unit of TU.
//
pub const MAC_CSR11: c_uint = 0x302c;

//
// MAC_CSR12: Manual power control / status register (merge CSR20 & PWRCSR1).
// CURRENT_STATE: 0:sleep, 1:awake.
// FORCE_WAKEUP: This has higher priority than PUT_TO_SLEEP.
// BBP_CURRENT_STATE: 0: BBP sleep, 1: BBP awake.
//
pub const MAC_CSR12: c_uint = 0x3030;

//
// MAC_CSR13: GPIO.
// MAC_CSR13_VALx: GPIO value
// MAC_CSR13_DIRx: GPIO direction: 0 = input; 1 = output
//
pub const MAC_CSR13: c_uint = 0x3034;

//
// MAC_CSR14: LED control register.
// ON_PERIOD: On period, default 70ms.
// OFF_PERIOD: Off period, default 30ms.
// HW_LED: HW TX activity, 1: normal OFF, 0: normal ON.
// SW_LED: s/w LED, 1: ON, 0: OFF.
// HW_LED_POLARITY: 0: active low, 1: active high.
//
pub const MAC_CSR14: c_uint = 0x3038;

//
// MAC_CSR15: NAV control.
//
pub const MAC_CSR15: c_uint = 0x303c;
//
// TXRX control registers.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// TXRX_CSR0: TX/RX configuration register.
// TSF_OFFSET: Default is 24.
// AUTO_TX_SEQ: 1: ASIC auto replace sequence nr in outgoing frame.
// DISABLE_RX: Disable Rx engine.
// DROP_CRC: Drop CRC error.
// DROP_PHYSICAL: Drop physical error.
// DROP_CONTROL: Drop control frame.
// DROP_NOT_TO_ME: Drop not to me unicast frame.
// DROP_TO_DS: Drop fram ToDs bit is true.
// DROP_VERSION_ERROR: Drop version error frame.
// DROP_MULTICAST: Drop multicast frames.
// DROP_BORADCAST: Drop broadcast frames.
// DROP_ACK_CTS: Drop received ACK and CTS.
//
pub const TXRX_CSR0: c_uint = 0x3040;

//
// TXRX_CSR1
//
pub const TXRX_CSR1: c_uint = 0x3044;

//
// TXRX_CSR2
//
pub const TXRX_CSR2: c_uint = 0x3048;

//
// TXRX_CSR3
//
pub const TXRX_CSR3: c_uint = 0x304c;

//
// TXRX_CSR4: Auto-Responder/Tx-retry register.
// AUTORESPOND_PREAMBLE: 0:long, 1:short preamble.
// OFDM_TX_RATE_DOWN: 1:enable.
// OFDM_TX_RATE_STEP: 0:1-step, 1: 2-step, 2:3-step, 3:4-step.
// OFDM_TX_FALLBACK_CCK: 0: Fallback to OFDM 6M only, 1: Fallback to CCK 1M,2M.
//
pub const TXRX_CSR4: c_uint = 0x3050;

//
// TXRX_CSR5
//
pub const TXRX_CSR5: c_uint = 0x3054;
//
// TXRX_CSR6: ACK/CTS payload consumed time
//
pub const TXRX_CSR6: c_uint = 0x3058;
//
// TXRX_CSR7: OFDM ACK/CTS payload consumed time for 6/9/12/18 mbps.
//
pub const TXRX_CSR7: c_uint = 0x305c;

//
// TXRX_CSR8: OFDM ACK/CTS payload consumed time for 24/36/48/54 mbps.
//
pub const TXRX_CSR8: c_uint = 0x3060;

//
// TXRX_CSR9: Synchronization control register.
// BEACON_INTERVAL: In unit of 1/16 TU.
// TSF_TICKING: Enable TSF auto counting.
// TSF_SYNC: Tsf sync, 0: disable, 1: infra, 2: ad-hoc/master mode.
// BEACON_GEN: Enable beacon generator.
//
pub const TXRX_CSR9: c_uint = 0x3064;

//
// TXRX_CSR10: BEACON alignment.
//
pub const TXRX_CSR10: c_uint = 0x3068;
//
// TXRX_CSR11: AES mask.
//
pub const TXRX_CSR11: c_uint = 0x306c;
//
// TXRX_CSR12: TSF low 32.
//
pub const TXRX_CSR12: c_uint = 0x3070;

//
// TXRX_CSR13: TSF high 32.
//
pub const TXRX_CSR13: c_uint = 0x3074;

//
// TXRX_CSR14: TBTT timer.
//
pub const TXRX_CSR14: c_uint = 0x3078;
//
// TXRX_CSR15: TKIP MIC priority byte "AND" mask.
//
pub const TXRX_CSR15: c_uint = 0x307c;
//
// PHY control registers.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// PHY_CSR0: RF/PS control.
//
pub const PHY_CSR0: c_uint = 0x3080;

//
// PHY_CSR1
//
pub const PHY_CSR1: c_uint = 0x3084;

//
// PHY_CSR2: Pre-TX BBP control.
//
pub const PHY_CSR2: c_uint = 0x3088;
//
// PHY_CSR3: BBP serial control register.
// VALUE: Register value to program into BBP.
// REG_NUM: Selected BBP register.
// READ_CONTROL: 0: Write BBP, 1: Read BBP.
// BUSY: 1: ASIC is busy execute BBP programming.
//
pub const PHY_CSR3: c_uint = 0x308c;

//
// PHY_CSR4: RF serial control register
// VALUE: Register value (include register id) serial out to RF/IF chip.
// NUMBER_OF_BITS: Number of bits used in RFRegValue (I:20, RFMD:22).
// IF_SELECT: 1: select IF to program, 0: select RF to program.
// PLL_LD: RF PLL_LD status.
// BUSY: 1: ASIC is busy execute RF programming.
//
pub const PHY_CSR4: c_uint = 0x3090;

//
// PHY_CSR5: RX to TX signal switch timing control.
//
pub const PHY_CSR5: c_uint = 0x3094;

//
// PHY_CSR6: TX to RX signal timing control.
//
pub const PHY_CSR6: c_uint = 0x3098;

//
// PHY_CSR7: TX DAC switching timing control.
//
pub const PHY_CSR7: c_uint = 0x309c;
//
// Security control register.
//
// SEC_CSR0: Shared key table control.
//
pub const SEC_CSR0: c_uint = 0x30a0;

//
// SEC_CSR1: Shared key table security mode register.
//
pub const SEC_CSR1: c_uint = 0x30a4;

//
// Pairwise key table valid bitmap registers.
// SEC_CSR2: pairwise key table valid bitmap 0.
// SEC_CSR3: pairwise key table valid bitmap 1.
//
pub const SEC_CSR2: c_uint = 0x30a8;
pub const SEC_CSR3: c_uint = 0x30ac;
//
// SEC_CSR4: Pairwise key table lookup control.
//
pub const SEC_CSR4: c_uint = 0x30b0;

//
// SEC_CSR5: shared key table security mode register.
//
pub const SEC_CSR5: c_uint = 0x30b4;

//
// STA control registers.
//
// STA_CSR0: RX PLCP error count & RX FCS error count.
//
pub const STA_CSR0: c_uint = 0x30c0;

//
// STA_CSR1: RX False CCA count & RX LONG frame count.
//
pub const STA_CSR1: c_uint = 0x30c4;

//
// STA_CSR2: TX Beacon count and RX FIFO overflow count.
//
pub const STA_CSR2: c_uint = 0x30c8;

//
// STA_CSR3: TX Beacon count.
//
pub const STA_CSR3: c_uint = 0x30cc;

//
// STA_CSR4: TX Retry count.
//
pub const STA_CSR4: c_uint = 0x30d0;

//
// STA_CSR5: TX Retry count.
//
pub const STA_CSR5: c_uint = 0x30d4;

//
// QOS control registers.
//
// QOS_CSR1: TXOP holder MAC address register.
//
pub const QOS_CSR1: c_uint = 0x30e4;

//
// QOS_CSR2: TXOP holder timeout register.
//
pub const QOS_CSR2: c_uint = 0x30e8;
//
// RX QOS-CFPOLL MAC address register.
// QOS_CSR3: RX QOS-CFPOLL MAC address 0.
// QOS_CSR4: RX QOS-CFPOLL MAC address 1.
//
pub const QOS_CSR3: c_uint = 0x30ec;
pub const QOS_CSR4: c_uint = 0x30f0;
//
// QOS_CSR5: "QosControl" field of the RX QOS-CFPOLL.
//
pub const QOS_CSR5: c_uint = 0x30f4;
//
// WMM Scheduler Register
//
// AIFSN_CSR: AIFSN for each EDCA AC.
// AIFSN0: For AC_VO.
// AIFSN1: For AC_VI.
// AIFSN2: For AC_BE.
// AIFSN3: For AC_BK.
//
pub const AIFSN_CSR: c_uint = 0x0400;

//
// CWMIN_CSR: CWmin for each EDCA AC.
// CWMIN0: For AC_VO.
// CWMIN1: For AC_VI.
// CWMIN2: For AC_BE.
// CWMIN3: For AC_BK.
//
pub const CWMIN_CSR: c_uint = 0x0404;

//
// CWMAX_CSR: CWmax for each EDCA AC.
// CWMAX0: For AC_VO.
// CWMAX1: For AC_VI.
// CWMAX2: For AC_BE.
// CWMAX3: For AC_BK.
//
pub const CWMAX_CSR: c_uint = 0x0408;

//
// AC_TXOP_CSR0: AC_VO/AC_VI TXOP register.
// AC0_TX_OP: For AC_VO, in unit of 32us.
// AC1_TX_OP: For AC_VI, in unit of 32us.
//
pub const AC_TXOP_CSR0: c_uint = 0x040c;

//
// AC_TXOP_CSR1: AC_BE/AC_BK TXOP register.
// AC2_TX_OP: For AC_BE, in unit of 32us.
// AC3_TX_OP: For AC_BK, in unit of 32us.
//
pub const AC_TXOP_CSR1: c_uint = 0x0410;

//
// BBP registers.
// The wordsize of the BBP is 8 bits.
//
// R2
//

//
// R3
//

//
// R4: RX antenna control
// FRAME_END: 1 - DPDT, 0 - SPDT (Only valid for 802.11G, RF2527 & RF2529)
//
// ANTENNA_CONTROL semantics (guessed):
// 0x1: Software controlled antenna switching (fixed or SW diversity)
// 0x2: Hardware diversity.
//

//
// R77
//

//
// RF registers
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
// HW MAC address.
//
pub const EEPROM_MAC_ADDR_0: c_uint = 0x0002;

pub const EEPROM_MAC_ADDR1: c_uint = 0x0003;

pub const EEPROM_MAC_ADDR_2: c_uint = 0x0004;

//
// EEPROM antenna.
// ANTENNA_NUM: Number of antennas.
// TX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// RX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// FRAME_TYPE: 0: DPDT , 1: SPDT , noted this bit is valid for g only.
// DYN_TXAGC: Dynamic TX AGC control.
// HARDWARE_RADIO: 1: Hardware controlled radio. Read GPIO0.
// RF_TYPE: Rf_type of this adapter.
//
pub const EEPROM_ANTENNA: c_uint = 0x0010;

//
// EEPROM NIC config.
// EXTERNAL_LNA: External LNA.
//
pub const EEPROM_NIC: c_uint = 0x0011;

//
// EEPROM geography.
// GEO_A: Default geographical setting for 5GHz band
// GEO: Default geographical setting.
//
pub const EEPROM_GEOGRAPHY: c_uint = 0x0012;

//
// EEPROM BBP.
//
pub const EEPROM_BBP_START: c_uint = 0x0013;
pub const EEPROM_BBP_SIZE: c_int = 16;

//
// EEPROM TXPOWER 802.11G
//
pub const EEPROM_TXPOWER_G_START: c_uint = 0x0023;
pub const EEPROM_TXPOWER_G_SIZE: c_int = 7;

//
// EEPROM Frequency
//
pub const EEPROM_FREQ: c_uint = 0x002f;

//
// EEPROM LED.
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
pub const EEPROM_LED: c_uint = 0x0030;

//
// EEPROM TXPOWER 802.11A
//
pub const EEPROM_TXPOWER_A_START: c_uint = 0x0031;
pub const EEPROM_TXPOWER_A_SIZE: c_int = 12;

//
// EEPROM RSSI offset 802.11BG
//
pub const EEPROM_RSSI_OFFSET_BG: c_uint = 0x004d;

//
// EEPROM RSSI offset 802.11A
//
pub const EEPROM_RSSI_OFFSET_A: c_uint = 0x004e;

//
// DMA descriptor defines.
//

//
// TX descriptor format for TX, PRIO and Beacon Ring.
//
// Word0
// BURST: Next frame belongs to same "burst" event.
// TKIP_MIC: ASIC appends TKIP MIC if TKIP is used.
// KEY_TABLE: Use per-client pairwise KEY table.
// KEY_INDEX:
// Key index (0~31) to the pairwise KEY table.
// 0~3 to shared KEY table 0 (BSS0).
// 4~7 to shared KEY table 1 (BSS1).
// 8~11 to shared KEY table 2 (BSS2).
// 12~15 to shared KEY table 3 (BSS3).
// BURST2: For backward compatibility, set to same value as BURST.
//

//
// Word1
// HOST_Q_ID: EDCA/HCCA queue ID.
// HW_SEQUENCE: MAC overwrites the frame sequence number.
// BUFFER_COUNT: Number of buffers in this TXD.
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
// Word5
// FRAME_OFFSET: Frame start offset inside ASIC TXFIFO (after TXINFO field).
// PACKET_ID: Driver assigned packet ID to categorize TXResult in interrupt.
// WAITING_DMA_DONE_INT: TXD been filled with data
// and waiting for TxDoneISR housekeeping.
//

//
// RX descriptor format for RX Ring.
//
// Word0
// CIPHER_ERROR: 1:ICV error, 2:MIC error, 3:invalid key.
// KEY_INDEX: Decryption key actually used.
//

//
// WORD1
// SIGNAL: RX raw data rate reported by BBP.
// RSSI: RSSI reported by BBP.
//

//
// Word2
// IV: Received IV of originally encrypted.
//

//
// Word3
// EIV: Received EIV of originally encrypted.
//

//
// Word4
// ICV: Received ICV of originally encrypted.
// NOTE: This is a guess, the official definition is "reserved"
//

//
// the above 20-byte is called RXINFO and will be DMAed to MAC RX block
// and passed to the HOST driver.
// The following fields are for DMA block and HOST usage only.
// Can't be touched by ASIC MAC block.
//
// Word5
//

//
// Macros for converting txpower from EEPROM to mac80211 value
// and from mac80211 value to register value.
//
pub const MIN_TXPOWER: c_int = 0;
pub const MAX_TXPOWER: c_int = 31;
pub const DEFAULT_TXPOWER: c_int = 24;

