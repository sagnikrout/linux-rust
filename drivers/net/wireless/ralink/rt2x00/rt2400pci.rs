//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2400pci.h
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
pub const RF2420: c_uint = 0x0000;
pub const RF2421: c_uint = 0x0001;
//
// Signal information.
// Default offset is required for RSSI <-> dBm conversion.
//
pub const DEFAULT_RSSI_OFFSET: c_int = 100;
//
// Register layout information.
//
pub const CSR_REG_BASE: c_uint = 0x0000;
pub const CSR_REG_SIZE: c_uint = 0x014c;
pub const EEPROM_BASE: c_uint = 0x0000;
pub const EEPROM_SIZE: c_uint = 0x0100;
pub const BBP_BASE: c_uint = 0x0000;
pub const BBP_SIZE: c_uint = 0x0020;
pub const RF_BASE: c_uint = 0x0004;
pub const RF_SIZE: c_uint = 0x000c;
//
// Number of TX queues.
//
pub const NUM_TX_QUEUES: c_int = 2;
//
// Control/Status Registers(CSR).
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// CSR0: ASIC revision number.
//
pub const CSR0: c_uint = 0x0000;

//
// CSR1: System control register.
// SOFT_RESET: Software reset, 1: reset, 0: normal.
// BBP_RESET: Hardware reset, 1: reset, 0, release.
// HOST_READY: Host ready after initialization.
//
pub const CSR1: c_uint = 0x0004;

//
// CSR2: System admin status register (invalid).
//
pub const CSR2: c_uint = 0x0008;
//
// CSR3: STA MAC address register 0.
//
pub const CSR3: c_uint = 0x000c;

//
// CSR4: STA MAC address register 1.
//
pub const CSR4: c_uint = 0x0010;

//
// CSR5: BSSID register 0.
//
pub const CSR5: c_uint = 0x0014;

//
// CSR6: BSSID register 1.
//
pub const CSR6: c_uint = 0x0018;

//
// CSR7: Interrupt source register.
// Write 1 to clear interrupt.
// TBCN_EXPIRE: Beacon timer expired interrupt.
// TWAKE_EXPIRE: Wakeup timer expired interrupt.
// TATIMW_EXPIRE: Timer of atim window expired interrupt.
// TXDONE_TXRING: Tx ring transmit done interrupt.
// TXDONE_ATIMRING: Atim ring transmit done interrupt.
// TXDONE_PRIORING: Priority ring transmit done interrupt.
// RXDONE: Receive done interrupt.
//
pub const CSR7: c_uint = 0x001c;

//
// CSR8: Interrupt mask register.
// Write 1 to mask interrupt.
// TBCN_EXPIRE: Beacon timer expired interrupt.
// TWAKE_EXPIRE: Wakeup timer expired interrupt.
// TATIMW_EXPIRE: Timer of atim window expired interrupt.
// TXDONE_TXRING: Tx ring transmit done interrupt.
// TXDONE_ATIMRING: Atim ring transmit done interrupt.
// TXDONE_PRIORING: Priority ring transmit done interrupt.
// RXDONE: Receive done interrupt.
//
pub const CSR8: c_uint = 0x0020;

//
// CSR9: Maximum frame length register.
// MAX_FRAME_UNIT: Maximum frame length in 128b unit, default: 12.
//
pub const CSR9: c_uint = 0x0024;

//
// CSR11: Back-off control register.
// CWMIN: CWmin. Default cwmin is 31 (2^5 - 1).
// CWMAX: CWmax. Default cwmax is 1023 (2^10 - 1).
// SLOT_TIME: Slot time, default is 20us for 802.11b.
// LONG_RETRY: Long retry count.
// SHORT_RETRY: Short retry count.
//
pub const CSR11: c_uint = 0x002c;

//
// CSR12: Synchronization configuration register 0.
// All units in 1/16 TU.
// BEACON_INTERVAL: Beacon interval, default is 100 TU.
// CFPMAX_DURATION: Cfp maximum duration, default is 100 TU.
//
pub const CSR12: c_uint = 0x0030;

//
// CSR13: Synchronization configuration register 1.
// All units in 1/16 TU.
// ATIMW_DURATION: Atim window duration.
// CFP_PERIOD: Cfp period, default is 0 TU.
//
pub const CSR13: c_uint = 0x0034;

//
// CSR14: Synchronization control register.
// TSF_COUNT: Enable tsf auto counting.
// TSF_SYNC: Tsf sync, 0: disable, 1: infra, 2: ad-hoc/master mode.
// TBCN: Enable tbcn with reload value.
// TCFP: Enable tcfp & cfp / cp switching.
// TATIMW: Enable tatimw & atim window switching.
// BEACON_GEN: Enable beacon generator.
// CFP_COUNT_PRELOAD: Cfp count preload value.
// TBCM_PRELOAD: Tbcn preload value in units of 64us.
//
pub const CSR14: c_uint = 0x0038;

//
// CSR15: Synchronization status register.
// CFP: ASIC is in contention-free period.
// ATIMW: ASIC is in ATIM window.
// BEACON_SENT: Beacon is send.
//
pub const CSR15: c_uint = 0x003c;

//
// CSR16: TSF timer register 0.
//
pub const CSR16: c_uint = 0x0040;

//
// CSR17: TSF timer register 1.
//
pub const CSR17: c_uint = 0x0044;

//
// CSR18: IFS timer register 0.
// SIFS: Sifs, default is 10 us.
// PIFS: Pifs, default is 30 us.
//
pub const CSR18: c_uint = 0x0048;

//
// CSR19: IFS timer register 1.
// DIFS: Difs, default is 50 us.
// EIFS: Eifs, default is 364 us.
//
pub const CSR19: c_uint = 0x004c;

//
// CSR20: Wakeup timer register.
// DELAY_AFTER_TBCN: Delay after tbcn expired in units of 1/16 TU.
// TBCN_BEFORE_WAKEUP: Number of beacon before wakeup.
// AUTOWAKE: Enable auto wakeup / sleep mechanism.
//
pub const CSR20: c_uint = 0x0050;

//
// CSR21: EEPROM control register.
// RELOAD: Write 1 to reload eeprom content.
// TYPE_93C46: 1: 93c46, 0:93c66.
//
pub const CSR21: c_uint = 0x0054;

//
// CSR22: CFP control register.
// CFP_DURATION_REMAIN: Cfp duration remain, in units of TU.
// RELOAD_CFP_DURATION: Write 1 to reload cfp duration remain.
//
pub const CSR22: c_uint = 0x0058;

//
// Transmit related CSRs.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// TXCSR0: TX Control Register.
// KICK_TX: Kick tx ring.
// KICK_ATIM: Kick atim ring.
// KICK_PRIO: Kick priority ring.
// ABORT: Abort all transmit related ring operation.
//
pub const TXCSR0: c_uint = 0x0060;

//
// TXCSR1: TX Configuration Register.
// ACK_TIMEOUT: Ack timeout, default = sifs + 2*slottime + acktime @ 1mbps.
// ACK_CONSUME_TIME: Ack consume time, default = sifs + acktime @ 1mbps.
// TSF_OFFSET: Insert tsf offset.
// AUTORESPONDER: Enable auto responder which include ack & cts.
//
pub const TXCSR1: c_uint = 0x0064;

//
// TXCSR2: Tx descriptor configuration register.
// TXD_SIZE: Tx descriptor size, default is 48.
// NUM_TXD: Number of tx entries in ring.
// NUM_ATIM: Number of atim entries in ring.
// NUM_PRIO: Number of priority entries in ring.
//
pub const TXCSR2: c_uint = 0x0068;

//
// TXCSR3: TX Ring Base address register.
//
pub const TXCSR3: c_uint = 0x006c;

//
// TXCSR4: TX Atim Ring Base address register.
//
pub const TXCSR4: c_uint = 0x0070;

//
// TXCSR5: TX Prio Ring Base address register.
//
pub const TXCSR5: c_uint = 0x0074;

//
// TXCSR6: Beacon Base address register.
//
pub const TXCSR6: c_uint = 0x0078;

//
// TXCSR7: Auto responder control register.
// AR_POWERMANAGEMENT: Auto responder power management bit.
//
pub const TXCSR7: c_uint = 0x007c;

//
// Receive related CSRs.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// RXCSR0: RX Control Register.
// DISABLE_RX: Disable rx engine.
// DROP_CRC: Drop crc error.
// DROP_PHYSICAL: Drop physical error.
// DROP_CONTROL: Drop control frame.
// DROP_NOT_TO_ME: Drop not to me unicast frame.
// DROP_TODS: Drop frame tods bit is true.
// DROP_VERSION_ERROR: Drop version error frame.
// PASS_CRC: Pass all packets with crc attached.
//
pub const RXCSR0: c_uint = 0x0080;

//
// RXCSR1: RX descriptor configuration register.
// RXD_SIZE: Rx descriptor size, default is 32b.
// NUM_RXD: Number of rx entries in ring.
//
pub const RXCSR1: c_uint = 0x0084;

//
// RXCSR2: RX Ring base address register.
//
pub const RXCSR2: c_uint = 0x0088;

//
// RXCSR3: BBP ID register for Rx operation.
// BBP_ID#: BBP register # id.
// BBP_ID#_VALID: BBP register # id is valid or not.
//
pub const RXCSR3: c_uint = 0x0090;

//
// RXCSR4: BBP ID register for Rx operation.
// BBP_ID#: BBP register # id.
// BBP_ID#_VALID: BBP register # id is valid or not.
//
pub const RXCSR4: c_uint = 0x0094;

//
// ARCSR0: Auto Responder PLCP config register 0.
// ARCSR0_AR_BBP_DATA#: Auto responder BBP register # data.
// ARCSR0_AR_BBP_ID#: Auto responder BBP register # Id.
//
pub const ARCSR0: c_uint = 0x0098;

//
// ARCSR1: Auto Responder PLCP config register 1.
// ARCSR0_AR_BBP_DATA#: Auto responder BBP register # data.
// ARCSR0_AR_BBP_ID#: Auto responder BBP register # Id.
//
pub const ARCSR1: c_uint = 0x009c;

//
// Miscellaneous Registers.
// Some values are set in TU, whereas 1 TU == 1024 us.
//
// PCICSR: PCI control register.
// BIG_ENDIAN: 1: big endian, 0: little endian.
// RX_TRESHOLD: Rx threshold in dw to start pci access
// 0: 16dw (default), 1: 8dw, 2: 4dw, 3: 32dw.
// TX_TRESHOLD: Tx threshold in dw to start pci access
// 0: 0dw (default), 1: 1dw, 2: 4dw, 3: forward.
// BURST_LENTH: Pci burst length 0: 4dw (default, 1: 8dw, 2: 16dw, 3:32dw.
// ENABLE_CLK: Enable clk_run, pci clock can't going down to non-operational.
//
pub const PCICSR: c_uint = 0x008c;

//
// CNT0: FCS error count.
// FCS_ERROR: FCS error count, cleared when read.
//
pub const CNT0: c_uint = 0x00a0;

//
// Statistic Register.
// CNT1: PLCP error count.
// CNT2: Long error count.
// CNT3: CCA false alarm count.
// CNT4: Rx FIFO overflow count.
// CNT5: Tx FIFO underrun count.
//
pub const TIMECSR2: c_uint = 0x00a8;
pub const CNT1: c_uint = 0x00ac;
pub const CNT2: c_uint = 0x00b0;
pub const TIMECSR3: c_uint = 0x00b4;
pub const CNT3: c_uint = 0x00b8;
pub const CNT4: c_uint = 0x00bc;
pub const CNT5: c_uint = 0x00c0;
//
// Baseband Control Register.
//
// PWRCSR0: Power mode configuration register.
//
pub const PWRCSR0: c_uint = 0x00c4;
//
// Power state transition time registers.
//
pub const PSCSR0: c_uint = 0x00c8;
pub const PSCSR1: c_uint = 0x00cc;
pub const PSCSR2: c_uint = 0x00d0;
pub const PSCSR3: c_uint = 0x00d4;
//
// PWRCSR1: Manual power control / status register.
// Allowed state: 0 deep_sleep, 1: sleep, 2: standby, 3: awake.
// SET_STATE: Set state. Write 1 to trigger, self cleared.
// BBP_DESIRE_STATE: BBP desired state.
// RF_DESIRE_STATE: RF desired state.
// BBP_CURR_STATE: BBP current state.
// RF_CURR_STATE: RF current state.
// PUT_TO_SLEEP: Put to sleep. Write 1 to trigger, self cleared.
//
pub const PWRCSR1: c_uint = 0x00d8;

//
// TIMECSR: Timer control register.
// US_COUNT: 1 us timer count in units of clock cycles.
// US_64_COUNT: 64 us timer count in units of 1 us timer.
// BEACON_EXPECT: Beacon expect window.
//
pub const TIMECSR: c_uint = 0x00dc;

//
// MACCSR0: MAC configuration register 0.
//
pub const MACCSR0: c_uint = 0x00e0;
//
// MACCSR1: MAC configuration register 1.
// KICK_RX: Kick one-shot rx in one-shot rx mode.
// ONESHOT_RXMODE: Enable one-shot rx mode for debugging.
// BBPRX_RESET_MODE: Ralink bbp rx reset mode.
// AUTO_TXBBP: Auto tx logic access bbp control register.
// AUTO_RXBBP: Auto rx logic access bbp control register.
// LOOPBACK: Loopback mode. 0: normal, 1: internal, 2: external, 3:rsvd.
// INTERSIL_IF: Intersil if calibration pin.
//
pub const MACCSR1: c_uint = 0x00e4;

//
// RALINKCSR: Ralink Rx auto-reset BBCR.
// AR_BBP_DATA#: Auto reset BBP register # data.
// AR_BBP_ID#: Auto reset BBP register # id.
//
pub const RALINKCSR: c_uint = 0x00e8;

//
// BCNCSR: Beacon interval control register.
// CHANGE: Write one to change beacon interval.
// DELTATIME: The delta time value.
// NUM_BEACON: Number of beacon according to mode.
// MODE: Please refer to asic specs.
// PLUS: Plus or minus delta time value.
//
pub const BCNCSR: c_uint = 0x00ec;

//
// BBP / RF / IF Control Register.
//
// BBPCSR: BBP serial control register.
// VALUE: Register value to program into BBP.
// REGNUM: Selected BBP register.
// BUSY: 1: asic is busy execute BBP programming.
// WRITE_CONTROL: 1: write BBP, 0: read BBP.
//
pub const BBPCSR: c_uint = 0x00f0;

//
// RFCSR: RF serial control register.
// VALUE: Register value + id to program into rf/if.
// NUMBER_OF_BITS: Number of bits used in value (i:20, rfmd:22).
// IF_SELECT: Chip to program: 0: rf, 1: if.
// PLL_LD: Rf pll_ld status.
// BUSY: 1: asic is busy execute rf programming.
//
pub const RFCSR: c_uint = 0x00f4;

//
// LEDCSR: LED control register.
// ON_PERIOD: On period, default 70ms.
// OFF_PERIOD: Off period, default 30ms.
// LINK: 0: linkoff, 1: linkup.
// ACTIVITY: 0: idle, 1: active.
//
pub const LEDCSR: c_uint = 0x00f8;

//
// ASIC pointer information.
// RXPTR: Current RX ring address.
// TXPTR: Current Tx ring address.
// PRIPTR: Current Priority ring address.
// ATIMPTR: Current ATIM ring address.
//
pub const RXPTR: c_uint = 0x0100;
pub const TXPTR: c_uint = 0x0104;
pub const PRIPTR: c_uint = 0x0108;
pub const ATIMPTR: c_uint = 0x010c;
//
// GPIO and others.
//
// GPIOCSR: GPIO control register.
// GPIOCSR_VALx: Actual GPIO pin x value
// GPIOCSR_DIRx: GPIO direction: 0 = output; 1 = input
//
pub const GPIOCSR: c_uint = 0x0120;

//
// BBPPCSR: BBP Pin control register.
//
pub const BBPPCSR: c_uint = 0x0124;
//
// BCNCSR1: Tx BEACON offset time control register.
// PRELOAD: Beacon timer offset in units of usec.
//
pub const BCNCSR1: c_uint = 0x0130;

//
// MACCSR2: TX_PE to RX_PE turn-around time control register
// DELAY: RX_PE low width, in units of pci clock cycle.
//
pub const MACCSR2: c_uint = 0x0134;

//
// ARCSR2: 1 Mbps ACK/CTS PLCP.
//
pub const ARCSR2: c_uint = 0x013c;

//
// ARCSR3: 2 Mbps ACK/CTS PLCP.
//
pub const ARCSR3: c_uint = 0x0140;

//
// ARCSR4: 5.5 Mbps ACK/CTS PLCP.
//
pub const ARCSR4: c_uint = 0x0144;

//
// ARCSR5: 11 Mbps ACK/CTS PLCP.
//
pub const ARCSR5: c_uint = 0x0148;

//
// BBP registers.
// The wordsize of the BBP is 8 bits.
//
// R1: TX antenna control
//

//
// R4: RX antenna control
//

//
// RF registers
//
// RF 1
//

//
// RF 3
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
// ANTENNA_NUM: Number of antenna's.
// TX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// RX_DEFAULT: Default antenna 0: diversity, 1: A, 2: B.
// RF_TYPE: Rf_type of this adapter.
// LED_MODE: 0: default, 1: TX/RX activity,2: Single (ignore link), 3: rsvd.
// RX_AGCVGC: 0: disable, 1:enable BBP R13 tuning.
// HARDWARE_RADIO: 1: Hardware controlled radio. Read GPIO0.
//
pub const EEPROM_ANTENNA: c_uint = 0x0b;

//
// EEPROM BBP.
//
pub const EEPROM_BBP_START: c_uint = 0x0c;
pub const EEPROM_BBP_SIZE: c_int = 7;

//
// EEPROM TXPOWER
//
pub const EEPROM_TXPOWER_START: c_uint = 0x13;
pub const EEPROM_TXPOWER_SIZE: c_int = 7;

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
// Word2
//

//
// Word3 & 4: PLCP information
// The PLCP values should be treated as if they were BBP values.
//

//
// Word5
//

//
// Word6
//

//
// Word7
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
// Word4
//

//
// Word5 & 6 & 7: Reserved
//

//
// Macros for converting txpower from EEPROM to mac80211 value
// and from mac80211 value to register value.
// NOTE: Logics in rt2400pci for txpower are reversed
// compared to the other rt2x00 drivers. A higher txpower
// value means that the txpower must be lowered. This is
// important when converting the value coming from the
// mac80211 stack to the rt2400 acceptable value.
//
pub const MIN_TXPOWER: c_int = 31;
pub const MAX_TXPOWER: c_int = 62;
pub const DEFAULT_TXPOWER: c_int = 39;

