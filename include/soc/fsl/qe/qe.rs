//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/qe.h
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
// Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors: 	Shlomi Gridish <gridish@freescale.com>
// Li Yang <leoli@freescale.com>
//
// Description:
// QUICC Engine (QE) external definitions and structure.
//

pub const QE_NUM_OF_BRGS: c_int = 16;
pub const QE_NUM_OF_PORTS: c_int = 1024;
// Clocks and BRGs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qe_clock {
    QE_CLK_NONE = 0,
    QE_BRG1,		/* Baud Rate Generator 1 */
    QE_BRG2,		/* Baud Rate Generator 2 */
    QE_BRG3,		/* Baud Rate Generator 3 */
    QE_BRG4,		/* Baud Rate Generator 4 */
    QE_BRG5,		/* Baud Rate Generator 5 */
    QE_BRG6,		/* Baud Rate Generator 6 */
    QE_BRG7,		/* Baud Rate Generator 7 */
    QE_BRG8,		/* Baud Rate Generator 8 */
    QE_BRG9,		/* Baud Rate Generator 9 */
    QE_BRG10,		/* Baud Rate Generator 10 */
    QE_BRG11,		/* Baud Rate Generator 11 */
    QE_BRG12,		/* Baud Rate Generator 12 */
    QE_BRG13,		/* Baud Rate Generator 13 */
    QE_BRG14,		/* Baud Rate Generator 14 */
    QE_BRG15,		/* Baud Rate Generator 15 */
    QE_BRG16,		/* Baud Rate Generator 16 */
    QE_CLK1,		/* Clock 1 */
    QE_CLK2,		/* Clock 2 */
    QE_CLK3,		/* Clock 3 */
    QE_CLK4,		/* Clock 4 */
    QE_CLK5,		/* Clock 5 */
    QE_CLK6,		/* Clock 6 */
    QE_CLK7,		/* Clock 7 */
    QE_CLK8,		/* Clock 8 */
    QE_CLK9,		/* Clock 9 */
    QE_CLK10,		/* Clock 10 */
    QE_CLK11,		/* Clock 11 */
    QE_CLK12,		/* Clock 12 */
    QE_CLK13,		/* Clock 13 */
    QE_CLK14,		/* Clock 14 */
    QE_CLK15,		/* Clock 15 */
    QE_CLK16,		/* Clock 16 */
    QE_CLK17,		/* Clock 17 */
    QE_CLK18,		/* Clock 18 */
    QE_CLK19,		/* Clock 19 */
    QE_CLK20,		/* Clock 20 */
    QE_CLK21,		/* Clock 21 */
    QE_CLK22,		/* Clock 22 */
    QE_CLK23,		/* Clock 23 */
    QE_CLK24,		/* Clock 24 */
    QE_RSYNC_PIN,		/* RSYNC from pin */
    QE_TSYNC_PIN,		/* TSYNC from pin */
    QE_CLK_DUMMY
}

// Export QE common operations

extern "C" {
    pub fn qe_reset();
}

extern "C" {
    pub fn cpm_muram_init() -> c_int;
}

extern "C" {
    pub fn cpm_muram_alloc(size: c_ulong, align: c_ulong) -> i32;
}
extern "C" {
    pub fn cpm_muram_free(offset: i32);
}
extern "C" {
    pub fn cpm_muram_alloc_fixed(offset: c_ulong, size: c_ulong) -> i32;
}
extern "C" {
    pub fn cpm_muram_offset(addr: *const void __iomem) -> c_ulong;
}
extern "C" {
    pub fn cpm_muram_dma(addr: *mut void __iomem) -> dma_addr_t;
}
extern "C" {
    pub fn cpm_muram_free_addr(addr: *const void __iomem);
}

// QE PIO
pub const QE_PIO_PINS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_pio_regs {
    pub /: *mut *mut __be32 cpodr; / Open drain register,
    pub /: *mut *mut __be32 cpdata; / Data register,
    pub /: *mut *mut __be32 cpdir1; / Direction register,
    pub /: *mut *mut __be32 cpdir2; / Direction register,
    pub /: *mut *mut __be32 cppar1; / Pin assignment register,
    pub /: *mut *mut __be32 cppar2; / Pin assignment register,
    pub pad: [u8; 8],
}

pub const QE_PIO_DIR_IN: c_int = 2;
pub const QE_PIO_DIR_OUT: c_int = 1;

extern "C" {
    pub fn par_io_init(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn par_io_of_config(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn par_io_data_set(port: u8, pin: u8, val: u8) -> c_int;
}

//
// Pin multiplexing functions.
//

extern "C" {
    pub fn qe_pin_free(qe_pin: *mut qe_pin);
}
extern "C" {
    pub fn qe_pin_set_gpio(qe_pin: *mut qe_pin);
}
extern "C" {
    pub fn qe_pin_set_dedicated(pin: *mut qe_pin);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

extern "C" {
    pub fn qe_issue_cmd(cmd: u32, device: u32, mcn_protocol: u8, cmd_input: u32) -> c_int;
}

// QE internal API
extern "C" {
    pub fn qe_clock_source(source: *const c_char) -> qe_clock;
}
extern "C" {
    pub fn qe_get_brg_clk() -> c_uint;
}
extern "C" {
    pub fn qe_setbrg(brg: qe_clock, rate: c_uint, multiplier: c_uint) -> c_int;
}
extern "C" {
    pub fn qe_get_snum() -> c_int;
}
extern "C" {
    pub fn qe_put_snum(snum: u8);
}
extern "C" {
    pub fn qe_get_num_of_risc() -> c_uint;
}
extern "C" {
    pub fn qe_get_num_of_snums() -> c_uint;
}
//
// MPC8568E reference manual says:
//
// "...power down sequence waits for all I/O interfaces to become idle.
// In some applications this may happen eventually without actively
// shutting down interfaces, but most likely, software will have to
// take steps to shut down the eTSEC, QUICC Engine Block, and PCI
// interfaces before issuing the command (either the write to the core
// MSR[WE] as described above or writing to POWMGTCSR) to put the
// device into sleep state."
//
// MPC8569E reference manual has a similar paragraph.
//

// we actually use cpm_muram implementation, define this for convenience

// Structure that defines QE firmware binary files.
//
// See Documentation/arch/powerpc/qe_firmware.rst for a description of these
// fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_firmware {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_header {
    pub /: *mut *mut __be32 length; / Length of the entire structure, in bytes,
    pub /: *mut *mut u8 magic[3]; / Set to { 'Q', 'E', 'F' },
    pub /: *mut *mut u8 version; / Version of this layout. First ver is '1',
    pub header: },
    pub /: *mut *mut u8 id[62]; / Null-terminated identifier string,
    pub /: *mut *mut u8 split; / 0 = shared I-RAM, 1 = split I-RAM,
    pub /: *mut *mut u8 count; / Number of microcode[] structures,
    pub /: *mut *mut __be16 model; / The SOC model,
    pub /: *mut *mut u8 major; / The SOC revision major,
    pub /: *mut *mut u8 minor; / The SOC revision minor,
// C attribute field omitted
    pub /: *mut *mut u8 padding[4]; / Reserved, for alignment,
    pub /: *mut *mut __be64 extended_modes; / Extended modes,
    pub /: *mut *mut __be32 vtraps[8]; / Virtual trap addresses,
    pub /: *mut *mut u8 reserved[4]; / Reserved, for future expansion,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_microcode {
    pub /: *mut *mut u8 id[32]; / Null-terminated identifier,
    pub /: *mut *mut __be32 traps[16]; / Trap addresses, 0 == ignore,
    pub /: *mut *mut __be32 eccr; / The value for the ECCR register,
    pub /: *mut *mut __be32 iram_offset; / Offset into I-RAM for the code,
    pub /: *mut *mut __be32 count; / Number of 32-bit words of the code,
    pub /: *mut *mut __be32 code_offset; / Offset of the actual microcode,
    pub /: *mut *mut u8 major; / The microcode version major,
    pub /: *mut *mut u8 minor; / The microcode version minor,
    pub /: *mut *mut u8 revision; / The microcode version revision,
    pub /: *mut *mut u8 padding; / Reserved, for alignment,
    pub /: *mut *mut u8 reserved[4]; / Reserved, for future expansion,
    pub microcode: [} __packed; ],
// All microcode binaries should be located here
// CRC32 should be located here, after the microcode binaries
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_firmware_info {
    pub /: *mut *mut char id[64]; / Firmware name,
    pub /: *mut *mut u32 vtraps[8]; / Virtual trap addresses,
    pub /: *mut *mut u64 extended_modes; / Extended modes,
}

// Upload a firmware to the QE
extern "C" {
    pub fn qe_upload_firmware(firmware: *const qe_firmware) -> c_int;
}

// Obtain information on the uploaded firmware
// QE USB
extern "C" {
    pub fn qe_usb_clock_set(clk: qe_clock, rate: c_int) -> c_int;
}
// Buffer descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_bd {
    pub status: __be16,
    pub length: __be16,
    pub buf: __be32,
// C attribute field omitted
pub const BD_STATUS_MASK: c_uint = 0xffff0000;
pub const BD_LENGTH_MASK: c_uint = 0x0000ffff;
// Alignment

pub const QE_ALIGNMENT_OF_BD: c_int = 8;
pub const QE_ALIGNMENT_OF_PRAM: c_int = 64;
// RISC allocation
pub const QE_RISC_ALLOCATION_RISC1: c_uint = 0x1  /* RISC 1 */;
pub const QE_RISC_ALLOCATION_RISC2: c_uint = 0x2  /* RISC 2 */;
pub const QE_RISC_ALLOCATION_RISC3: c_uint = 0x4  /* RISC 3 */;
pub const QE_RISC_ALLOCATION_RISC4: c_uint = 0x8  /* RISC 4 */;

// QE extended filtering Table Lookup Key Size
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qe_fltr_tbl_lookup_key_size {
    QE_FLTR_TABLE_LOOKUP_KEY_SIZE_8_BYTES
    = 0x3f,		/* LookupKey parsed by the Generate LookupKey
    CMD is truncated to 8 bytes */
    QE_FLTR_TABLE_LOOKUP_KEY_SIZE_16_BYTES
    = 0x5f,		/* LookupKey parsed by the Generate LookupKey
    CMD is truncated to 16 bytes */
}

// QE FLTR extended filtering Largest External Table Lookup Key Size
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qe_fltr_largest_external_tbl_lookup_key_size {
    QE_FLTR_LARGEST_EXTERNAL_TABLE_LOOKUP_KEY_SIZE_NONE
    = 0x0,/* not used */
    QE_FLTR_LARGEST_EXTERNAL_TABLE_LOOKUP_KEY_SIZE_8_BYTES
    = QE_FLTR_TABLE_LOOKUP_KEY_SIZE_8_BYTES,	/* 8 bytes */
    QE_FLTR_LARGEST_EXTERNAL_TABLE_LOOKUP_KEY_SIZE_16_BYTES
    = QE_FLTR_TABLE_LOOKUP_KEY_SIZE_16_BYTES,	/* 16 bytes */
}

// structure representing QE parameter RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_timer_tables {
    pub /: *mut *mut u16 tm_base; / QE timer table base adr,
    pub /: *mut *mut u16 tm_ptr; / QE timer table pointer,
    pub /: *mut *mut u16 r_tmr; / QE timer mode register,
    pub /: *mut *mut u16 r_tmv; / QE timer valid register,
    pub /: *mut *mut u32 tm_cmd; / QE timer cmd register,
    pub /: *mut *mut u32 tm_cnt; / QE timer internal cnt,
// C attribute field omitted
pub const QE_FLTR_TAD_SIZE: c_int = 8;
// QE extended filtering Termination Action Descriptor (TAD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_fltr_tad {
    pub serialized: [u8; QE_FLTR_TAD_SIZE],
// C attribute field omitted
// Communication Direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum comm_dir {
    COMM_DIR_NONE = 0,
    COMM_DIR_RX = 1,
    COMM_DIR_TX = 2,
    COMM_DIR_RX_AND_TX = 3
}

// QE CMXUCR Registers.
// There are two UCCs represented in each of the four CMXUCR registers.
// These values are for the UCC in the LSBs
//
pub const QE_CMXUCR_MII_ENET_MNG: c_uint = 0x00007000;
pub const QE_CMXUCR_MII_ENET_MNG_SHIFT: c_int = 12;
pub const QE_CMXUCR_GRANT: c_uint = 0x00008000;
pub const QE_CMXUCR_TSA: c_uint = 0x00004000;
pub const QE_CMXUCR_BKPT: c_uint = 0x00000100;
pub const QE_CMXUCR_TX_CLK_SRC_MASK: c_uint = 0x0000000F;
// QE CMXGCR Registers.
//
pub const QE_CMXGCR_MII_ENET_MNG: c_uint = 0x00007000;
pub const QE_CMXGCR_MII_ENET_MNG_SHIFT: c_int = 12;
pub const QE_CMXGCR_USBCS: c_uint = 0x0000000f;
pub const QE_CMXGCR_USBCS_CLK3: c_uint = 0x1;
pub const QE_CMXGCR_USBCS_CLK5: c_uint = 0x2;
pub const QE_CMXGCR_USBCS_CLK7: c_uint = 0x3;
pub const QE_CMXGCR_USBCS_CLK9: c_uint = 0x4;
pub const QE_CMXGCR_USBCS_CLK13: c_uint = 0x5;
pub const QE_CMXGCR_USBCS_CLK17: c_uint = 0x6;
pub const QE_CMXGCR_USBCS_CLK19: c_uint = 0x7;
pub const QE_CMXGCR_USBCS_CLK21: c_uint = 0x8;
pub const QE_CMXGCR_USBCS_BRG9: c_uint = 0x9;
pub const QE_CMXGCR_USBCS_BRG10: c_uint = 0xa;
// QE CECR Commands.
//
pub const QE_CR_FLG: c_uint = 0x00010000;
pub const QE_RESET: c_uint = 0x80000000;
pub const QE_INIT_TX_RX: c_uint = 0x00000000;
pub const QE_INIT_RX: c_uint = 0x00000001;
pub const QE_INIT_TX: c_uint = 0x00000002;
pub const QE_ENTER_HUNT_MODE: c_uint = 0x00000003;
pub const QE_STOP_TX: c_uint = 0x00000004;
pub const QE_GRACEFUL_STOP_TX: c_uint = 0x00000005;
pub const QE_RESTART_TX: c_uint = 0x00000006;
pub const QE_CLOSE_RX_BD: c_uint = 0x00000007;
pub const QE_SWITCH_COMMAND: c_uint = 0x00000007;
pub const QE_SET_GROUP_ADDRESS: c_uint = 0x00000008;
pub const QE_START_IDMA: c_uint = 0x00000009;
pub const QE_MCC_STOP_RX: c_uint = 0x00000009;
pub const QE_ATM_TRANSMIT: c_uint = 0x0000000a;
pub const QE_HPAC_CLEAR_ALL: c_uint = 0x0000000b;
pub const QE_GRACEFUL_STOP_RX: c_uint = 0x0000001a;
pub const QE_RESTART_RX: c_uint = 0x0000001b;
pub const QE_HPAC_SET_PRIORITY: c_uint = 0x0000010b;
pub const QE_HPAC_STOP_TX: c_uint = 0x0000020b;
pub const QE_HPAC_STOP_RX: c_uint = 0x0000030b;
pub const QE_HPAC_GRACEFUL_STOP_TX: c_uint = 0x0000040b;
pub const QE_HPAC_GRACEFUL_STOP_RX: c_uint = 0x0000050b;
pub const QE_HPAC_START_TX: c_uint = 0x0000060b;
pub const QE_HPAC_START_RX: c_uint = 0x0000070b;
pub const QE_USB_STOP_TX: c_uint = 0x0000000a;
pub const QE_USB_RESTART_TX: c_uint = 0x0000000c;
pub const QE_QMC_STOP_TX: c_uint = 0x0000000c;
pub const QE_QMC_STOP_RX: c_uint = 0x0000000d;
pub const QE_SS7_SU_FIL_RESET: c_uint = 0x0000000e;
pub const QE_PUSHSCHED: c_uint = 0x0000000f;
// jonathbr added from here down for 83xx
pub const QE_RESET_BCS: c_uint = 0x0000000a;
pub const QE_MCC_INIT_TX_RX_16: c_uint = 0x00000003;
pub const QE_MCC_STOP_TX: c_uint = 0x00000004;
pub const QE_MCC_INIT_TX_1: c_uint = 0x00000005;
pub const QE_MCC_INIT_RX_1: c_uint = 0x00000006;
pub const QE_MCC_RESET: c_uint = 0x00000007;
pub const QE_SET_TIMER: c_uint = 0x00000008;
pub const QE_RANDOM_NUMBER: c_uint = 0x0000000c;
pub const QE_ATM_MULTI_THREAD_INIT: c_uint = 0x00000011;
pub const QE_ASSIGN_PAGE: c_uint = 0x00000012;
pub const QE_ADD_REMOVE_HASH_ENTRY: c_uint = 0x00000013;
pub const QE_START_FLOW_CONTROL: c_uint = 0x00000014;
pub const QE_STOP_FLOW_CONTROL: c_uint = 0x00000015;
pub const QE_ASSIGN_PAGE_TO_DEVICE: c_uint = 0x00000016;
pub const QE_ASSIGN_RISC: c_uint = 0x00000010;
pub const QE_CR_MCN_NORMAL_SHIFT: c_int = 6;
pub const QE_CR_MCN_USB_SHIFT: c_int = 4;
pub const QE_CR_MCN_RISC_ASSIGN_SHIFT: c_int = 8;
pub const QE_CR_SNUM_SHIFT: c_int = 17;
// QE CECR Sub Block - sub block of QE command.
//
pub const QE_CR_SUBBLOCK_INVALID: c_uint = 0x00000000;
pub const QE_CR_SUBBLOCK_USB: c_uint = 0x03200000;
pub const QE_CR_SUBBLOCK_UCCFAST1: c_uint = 0x02000000;
pub const QE_CR_SUBBLOCK_UCCFAST2: c_uint = 0x02200000;
pub const QE_CR_SUBBLOCK_UCCFAST3: c_uint = 0x02400000;
pub const QE_CR_SUBBLOCK_UCCFAST4: c_uint = 0x02600000;
pub const QE_CR_SUBBLOCK_UCCFAST5: c_uint = 0x02800000;
pub const QE_CR_SUBBLOCK_UCCFAST6: c_uint = 0x02a00000;
pub const QE_CR_SUBBLOCK_UCCFAST7: c_uint = 0x02c00000;
pub const QE_CR_SUBBLOCK_UCCFAST8: c_uint = 0x02e00000;
pub const QE_CR_SUBBLOCK_UCCSLOW1: c_uint = 0x00000000;
pub const QE_CR_SUBBLOCK_UCCSLOW2: c_uint = 0x00200000;
pub const QE_CR_SUBBLOCK_UCCSLOW3: c_uint = 0x00400000;
pub const QE_CR_SUBBLOCK_UCCSLOW4: c_uint = 0x00600000;
pub const QE_CR_SUBBLOCK_UCCSLOW5: c_uint = 0x00800000;
pub const QE_CR_SUBBLOCK_UCCSLOW6: c_uint = 0x00a00000;
pub const QE_CR_SUBBLOCK_UCCSLOW7: c_uint = 0x00c00000;
pub const QE_CR_SUBBLOCK_UCCSLOW8: c_uint = 0x00e00000;
pub const QE_CR_SUBBLOCK_MCC1: c_uint = 0x03800000;
pub const QE_CR_SUBBLOCK_MCC2: c_uint = 0x03a00000;
pub const QE_CR_SUBBLOCK_MCC3: c_uint = 0x03000000;
pub const QE_CR_SUBBLOCK_IDMA1: c_uint = 0x02800000;
pub const QE_CR_SUBBLOCK_IDMA2: c_uint = 0x02a00000;
pub const QE_CR_SUBBLOCK_IDMA3: c_uint = 0x02c00000;
pub const QE_CR_SUBBLOCK_IDMA4: c_uint = 0x02e00000;
pub const QE_CR_SUBBLOCK_HPAC: c_uint = 0x01e00000;
pub const QE_CR_SUBBLOCK_SPI1: c_uint = 0x01400000;
pub const QE_CR_SUBBLOCK_SPI2: c_uint = 0x01600000;
pub const QE_CR_SUBBLOCK_RAND: c_uint = 0x01c00000;
pub const QE_CR_SUBBLOCK_TIMER: c_uint = 0x01e00000;
pub const QE_CR_SUBBLOCK_GENERAL: c_uint = 0x03c00000;
// QE CECR Protocol - For non-MCC, specifies mode for QE CECR command
pub const QE_CR_PROTOCOL_UNSPECIFIED: c_uint = 0x00	/* For all other protocols */;
pub const QE_CR_PROTOCOL_HDLC_TRANSPARENT: c_uint = 0x00;
pub const QE_CR_PROTOCOL_QMC: c_uint = 0x02;
pub const QE_CR_PROTOCOL_UART: c_uint = 0x04;
pub const QE_CR_PROTOCOL_ATM_POS: c_uint = 0x0A;
pub const QE_CR_PROTOCOL_ETHERNET: c_uint = 0x0C;
pub const QE_CR_PROTOCOL_L2_SWITCH: c_uint = 0x0D;
// BRG configuration register
pub const QE_BRGC_ENABLE: c_uint = 0x00010000;
pub const QE_BRGC_DIVISOR_SHIFT: c_int = 1;
pub const QE_BRGC_DIVISOR_MAX: c_uint = 0xFFF;
pub const QE_BRGC_DIV16: c_int = 1;
// QE Timers registers
pub const QE_GTCFR1_PCAS: c_uint = 0x80;
pub const QE_GTCFR1_STP2: c_uint = 0x20;
pub const QE_GTCFR1_RST2: c_uint = 0x10;
pub const QE_GTCFR1_GM2: c_uint = 0x08;
pub const QE_GTCFR1_GM1: c_uint = 0x04;
pub const QE_GTCFR1_STP1: c_uint = 0x02;
pub const QE_GTCFR1_RST1: c_uint = 0x01;
// SDMA registers
pub const QE_SDSR_BER1: c_uint = 0x02000000;
pub const QE_SDSR_BER2: c_uint = 0x01000000;
pub const QE_SDMR_GLB_1_MSK: c_uint = 0x80000000;
pub const QE_SDMR_ADR_SEL: c_uint = 0x20000000;
pub const QE_SDMR_BER1_MSK: c_uint = 0x02000000;
pub const QE_SDMR_BER2_MSK: c_uint = 0x01000000;
pub const QE_SDMR_EB1_MSK: c_uint = 0x00800000;
pub const QE_SDMR_ER1_MSK: c_uint = 0x00080000;
pub const QE_SDMR_ER2_MSK: c_uint = 0x00040000;
pub const QE_SDMR_CEN_MASK: c_uint = 0x0000E000;
pub const QE_SDMR_SBER_1: c_uint = 0x00000200;
pub const QE_SDMR_SBER_2: c_uint = 0x00000200;
pub const QE_SDMR_EB1_PR_MASK: c_uint = 0x000000C0;
pub const QE_SDMR_ER1_PR: c_uint = 0x00000008;
pub const QE_SDMR_CEN_SHIFT: c_int = 13;
pub const QE_SDMR_EB1_PR_SHIFT: c_int = 6;
pub const QE_SDTM_MSNUM_SHIFT: c_int = 24;
pub const QE_SDEBCR_BA_MASK: c_uint = 0x01FFFFFF;
// Communication Processor
pub const QE_CP_CERCR_MEE: c_uint = 0x8000	/* Multi-user RAM ECC enable */;
pub const QE_CP_CERCR_IEE: c_uint = 0x4000	/* Instruction RAM ECC enable */;
pub const QE_CP_CERCR_CIR: c_uint = 0x0800	/* Common instruction RAM */;
// I-RAM
pub const QE_IRAM_IADD_AIE: c_uint = 0x80000000	/* Auto Increment Enable */;
pub const QE_IRAM_IADD_BADDR: c_uint = 0x00080000	/* Base Address */;
pub const QE_IRAM_READY: c_uint = 0x80000000      /* Ready */;
// UPC
pub const UPGCR_PROTOCOL: c_uint = 0x80000000	/* protocol ul2 or pl2 */;
pub const UPGCR_TMS: c_uint = 0x40000000	/* Transmit master/slave mode */;
pub const UPGCR_RMS: c_uint = 0x20000000	/* Receive master/slave mode */;
pub const UPGCR_ADDR: c_uint = 0x10000000	/* Master MPHY Addr multiplexing */;
pub const UPGCR_DIAG: c_uint = 0x01000000	/* Diagnostic mode */;
// UCC GUEMR register
pub const UCC_GUEMR_MODE_MASK_RX: c_uint = 0x02;
pub const UCC_GUEMR_MODE_FAST_RX: c_uint = 0x02;
pub const UCC_GUEMR_MODE_SLOW_RX: c_uint = 0x00;
pub const UCC_GUEMR_MODE_MASK_TX: c_uint = 0x01;
pub const UCC_GUEMR_MODE_FAST_TX: c_uint = 0x01;
pub const UCC_GUEMR_MODE_SLOW_TX: c_uint = 0x00;

pub const UCC_GUEMR_SET_RESERVED3: c_uint = 0x10	/* Bit 3 in the guemr is reserved but;
// structure representing UCC SLOW parameter RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_slow_pram {
    pub /: *mut *mut __be16 rbase; / RX BD base address,
    pub /: *mut *mut __be16 tbase; / TX BD base address,
    pub /: *mut *mut u8 rbmr; / RX bus mode register (same as CPM's RFCR),
    pub /: *mut *mut u8 tbmr; / TX bus mode register (same as CPM's TFCR),
    pub /: *mut *mut __be16 mrblr; / Rx buffer length,
    pub /: *mut *mut __be32 rstate; / Rx internal state,
    pub /: *mut *mut __be32 rptr; / Rx internal data pointer,
    pub /: *mut *mut __be16 rbptr; / rb BD Pointer,
    pub /: *mut *mut __be16 rcount; / Rx internal byte count,
    pub /: *mut *mut __be32 rtemp; / Rx temp,
    pub /: *mut *mut __be32 tstate; / Tx internal state,
    pub /: *mut *mut __be32 tptr; / Tx internal data pointer,
    pub /: *mut *mut __be16 tbptr; / Tx BD pointer,
    pub /: *mut *mut __be16 tcount; / Tx byte count,
    pub /: *mut *mut __be32 ttemp; / Tx temp,
    pub /: *mut *mut __be32 rcrc; / temp receive CRC,
    pub /: *mut *mut __be32 tcrc; / temp transmit CRC,
// C attribute field omitted
// General UCC SLOW Mode Register (GUMRH & GUMRL)
pub const UCC_SLOW_GUMR_H_SAM_QMC: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_H_SAM_SATM: c_uint = 0x00008000;
pub const UCC_SLOW_GUMR_H_REVD: c_uint = 0x00002000;
pub const UCC_SLOW_GUMR_H_TRX: c_uint = 0x00001000;
pub const UCC_SLOW_GUMR_H_TTX: c_uint = 0x00000800;
pub const UCC_SLOW_GUMR_H_CDP: c_uint = 0x00000400;
pub const UCC_SLOW_GUMR_H_CTSP: c_uint = 0x00000200;
pub const UCC_SLOW_GUMR_H_CDS: c_uint = 0x00000100;
pub const UCC_SLOW_GUMR_H_CTSS: c_uint = 0x00000080;
pub const UCC_SLOW_GUMR_H_TFL: c_uint = 0x00000040;
pub const UCC_SLOW_GUMR_H_RFW: c_uint = 0x00000020;
pub const UCC_SLOW_GUMR_H_TXSY: c_uint = 0x00000010;
pub const UCC_SLOW_GUMR_H_4SYNC: c_uint = 0x00000004;
pub const UCC_SLOW_GUMR_H_8SYNC: c_uint = 0x00000008;
pub const UCC_SLOW_GUMR_H_16SYNC: c_uint = 0x0000000c;
pub const UCC_SLOW_GUMR_H_RTSM: c_uint = 0x00000002;
pub const UCC_SLOW_GUMR_H_RSYN: c_uint = 0x00000001;
pub const UCC_SLOW_GUMR_L_TCI: c_uint = 0x10000000;
pub const UCC_SLOW_GUMR_L_RINV: c_uint = 0x02000000;
pub const UCC_SLOW_GUMR_L_TINV: c_uint = 0x01000000;
pub const UCC_SLOW_GUMR_L_TEND: c_uint = 0x00040000;
pub const UCC_SLOW_GUMR_L_TDCR_MASK: c_uint = 0x00030000;
pub const UCC_SLOW_GUMR_L_TDCR_32: c_uint = 0x00030000;
pub const UCC_SLOW_GUMR_L_TDCR_16: c_uint = 0x00020000;
pub const UCC_SLOW_GUMR_L_TDCR_8: c_uint = 0x00010000;
pub const UCC_SLOW_GUMR_L_TDCR_1: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_L_RDCR_MASK: c_uint = 0x0000c000;
pub const UCC_SLOW_GUMR_L_RDCR_32: c_uint = 0x0000c000;
pub const UCC_SLOW_GUMR_L_RDCR_16: c_uint = 0x00008000;
pub const UCC_SLOW_GUMR_L_RDCR_8: c_uint = 0x00004000;
pub const UCC_SLOW_GUMR_L_RDCR_1: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_L_RENC_NRZI: c_uint = 0x00000800;
pub const UCC_SLOW_GUMR_L_RENC_NRZ: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_L_TENC_NRZI: c_uint = 0x00000100;
pub const UCC_SLOW_GUMR_L_TENC_NRZ: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_L_DIAG_MASK: c_uint = 0x000000c0;
pub const UCC_SLOW_GUMR_L_DIAG_LE: c_uint = 0x000000c0;
pub const UCC_SLOW_GUMR_L_DIAG_ECHO: c_uint = 0x00000080;
pub const UCC_SLOW_GUMR_L_DIAG_LOOP: c_uint = 0x00000040;
pub const UCC_SLOW_GUMR_L_DIAG_NORM: c_uint = 0x00000000;
pub const UCC_SLOW_GUMR_L_ENR: c_uint = 0x00000020;
pub const UCC_SLOW_GUMR_L_ENT: c_uint = 0x00000010;
pub const UCC_SLOW_GUMR_L_MODE_MASK: c_uint = 0x0000000F;
pub const UCC_SLOW_GUMR_L_MODE_BISYNC: c_uint = 0x00000008;
pub const UCC_SLOW_GUMR_L_MODE_AHDLC: c_uint = 0x00000006;
pub const UCC_SLOW_GUMR_L_MODE_UART: c_uint = 0x00000004;
pub const UCC_SLOW_GUMR_L_MODE_QMC: c_uint = 0x00000002;
// General UCC FAST Mode Register
pub const UCC_FAST_GUMR_LOOPBACK: c_uint = 0x40000000;
pub const UCC_FAST_GUMR_TCI: c_uint = 0x20000000;
pub const UCC_FAST_GUMR_TRX: c_uint = 0x10000000;
pub const UCC_FAST_GUMR_TTX: c_uint = 0x08000000;
pub const UCC_FAST_GUMR_CDP: c_uint = 0x04000000;
pub const UCC_FAST_GUMR_CTSP: c_uint = 0x02000000;
pub const UCC_FAST_GUMR_CDS: c_uint = 0x01000000;
pub const UCC_FAST_GUMR_CTSS: c_uint = 0x00800000;
pub const UCC_FAST_GUMR_TXSY: c_uint = 0x00020000;
pub const UCC_FAST_GUMR_RSYN: c_uint = 0x00010000;
pub const UCC_FAST_GUMR_SYNL_MASK: c_uint = 0x0000C000;
pub const UCC_FAST_GUMR_SYNL_16: c_uint = 0x0000C000;
pub const UCC_FAST_GUMR_SYNL_8: c_uint = 0x00008000;
pub const UCC_FAST_GUMR_SYNL_AUTO: c_uint = 0x00004000;
pub const UCC_FAST_GUMR_RTSM: c_uint = 0x00002000;
pub const UCC_FAST_GUMR_REVD: c_uint = 0x00000400;
pub const UCC_FAST_GUMR_ENR: c_uint = 0x00000020;
pub const UCC_FAST_GUMR_ENT: c_uint = 0x00000010;
// UART Slow UCC Event Register (UCCE)
pub const UCC_UART_UCCE_AB: c_uint = 0x0200;
pub const UCC_UART_UCCE_IDLE: c_uint = 0x0100;
pub const UCC_UART_UCCE_GRA: c_uint = 0x0080;
pub const UCC_UART_UCCE_BRKE: c_uint = 0x0040;
pub const UCC_UART_UCCE_BRKS: c_uint = 0x0020;
pub const UCC_UART_UCCE_CCR: c_uint = 0x0008;
pub const UCC_UART_UCCE_BSY: c_uint = 0x0004;
pub const UCC_UART_UCCE_TX: c_uint = 0x0002;
pub const UCC_UART_UCCE_RX: c_uint = 0x0001;
// HDLC Slow UCC Event Register (UCCE)
pub const UCC_HDLC_UCCE_GLR: c_uint = 0x1000;
pub const UCC_HDLC_UCCE_GLT: c_uint = 0x0800;
pub const UCC_HDLC_UCCE_IDLE: c_uint = 0x0100;
pub const UCC_HDLC_UCCE_BRKE: c_uint = 0x0040;
pub const UCC_HDLC_UCCE_BRKS: c_uint = 0x0020;
pub const UCC_HDLC_UCCE_TXE: c_uint = 0x0010;
pub const UCC_HDLC_UCCE_RXF: c_uint = 0x0008;
pub const UCC_HDLC_UCCE_BSY: c_uint = 0x0004;
pub const UCC_HDLC_UCCE_TXB: c_uint = 0x0002;
pub const UCC_HDLC_UCCE_RXB: c_uint = 0x0001;
// BISYNC Slow UCC Event Register (UCCE)
pub const UCC_BISYNC_UCCE_GRA: c_uint = 0x0080;
pub const UCC_BISYNC_UCCE_TXE: c_uint = 0x0010;
pub const UCC_BISYNC_UCCE_RCH: c_uint = 0x0008;
pub const UCC_BISYNC_UCCE_BSY: c_uint = 0x0004;
pub const UCC_BISYNC_UCCE_TXB: c_uint = 0x0002;
pub const UCC_BISYNC_UCCE_RXB: c_uint = 0x0001;
// Gigabit Ethernet Fast UCC Event Register (UCCE)
pub const UCC_GETH_UCCE_MPD: c_uint = 0x80000000;
pub const UCC_GETH_UCCE_SCAR: c_uint = 0x40000000;
pub const UCC_GETH_UCCE_GRA: c_uint = 0x20000000;
pub const UCC_GETH_UCCE_CBPR: c_uint = 0x10000000;
pub const UCC_GETH_UCCE_BSY: c_uint = 0x08000000;
pub const UCC_GETH_UCCE_RXC: c_uint = 0x04000000;
pub const UCC_GETH_UCCE_TXC: c_uint = 0x02000000;
pub const UCC_GETH_UCCE_TXE: c_uint = 0x01000000;
pub const UCC_GETH_UCCE_TXB7: c_uint = 0x00800000;
pub const UCC_GETH_UCCE_TXB6: c_uint = 0x00400000;
pub const UCC_GETH_UCCE_TXB5: c_uint = 0x00200000;
pub const UCC_GETH_UCCE_TXB4: c_uint = 0x00100000;
pub const UCC_GETH_UCCE_TXB3: c_uint = 0x00080000;
pub const UCC_GETH_UCCE_TXB2: c_uint = 0x00040000;
pub const UCC_GETH_UCCE_TXB1: c_uint = 0x00020000;
pub const UCC_GETH_UCCE_TXB0: c_uint = 0x00010000;
pub const UCC_GETH_UCCE_RXB7: c_uint = 0x00008000;
pub const UCC_GETH_UCCE_RXB6: c_uint = 0x00004000;
pub const UCC_GETH_UCCE_RXB5: c_uint = 0x00002000;
pub const UCC_GETH_UCCE_RXB4: c_uint = 0x00001000;
pub const UCC_GETH_UCCE_RXB3: c_uint = 0x00000800;
pub const UCC_GETH_UCCE_RXB2: c_uint = 0x00000400;
pub const UCC_GETH_UCCE_RXB1: c_uint = 0x00000200;
pub const UCC_GETH_UCCE_RXB0: c_uint = 0x00000100;
pub const UCC_GETH_UCCE_RXF7: c_uint = 0x00000080;
pub const UCC_GETH_UCCE_RXF6: c_uint = 0x00000040;
pub const UCC_GETH_UCCE_RXF5: c_uint = 0x00000020;
pub const UCC_GETH_UCCE_RXF4: c_uint = 0x00000010;
pub const UCC_GETH_UCCE_RXF3: c_uint = 0x00000008;
pub const UCC_GETH_UCCE_RXF2: c_uint = 0x00000004;
pub const UCC_GETH_UCCE_RXF1: c_uint = 0x00000002;
pub const UCC_GETH_UCCE_RXF0: c_uint = 0x00000001;
// UCC Protocol Specific Mode Register (UPSMR), when used for UART
pub const UCC_UART_UPSMR_FLC: c_uint = 0x8000;
pub const UCC_UART_UPSMR_SL: c_uint = 0x4000;
pub const UCC_UART_UPSMR_CL_MASK: c_uint = 0x3000;
pub const UCC_UART_UPSMR_CL_8: c_uint = 0x3000;
pub const UCC_UART_UPSMR_CL_7: c_uint = 0x2000;
pub const UCC_UART_UPSMR_CL_6: c_uint = 0x1000;
pub const UCC_UART_UPSMR_CL_5: c_uint = 0x0000;
pub const UCC_UART_UPSMR_UM_MASK: c_uint = 0x0c00;
pub const UCC_UART_UPSMR_UM_NORMAL: c_uint = 0x0000;
pub const UCC_UART_UPSMR_UM_MAN_MULTI: c_uint = 0x0400;
pub const UCC_UART_UPSMR_UM_AUTO_MULTI: c_uint = 0x0c00;
pub const UCC_UART_UPSMR_FRZ: c_uint = 0x0200;
pub const UCC_UART_UPSMR_RZS: c_uint = 0x0100;
pub const UCC_UART_UPSMR_SYN: c_uint = 0x0080;
pub const UCC_UART_UPSMR_DRT: c_uint = 0x0040;
pub const UCC_UART_UPSMR_PEN: c_uint = 0x0010;
pub const UCC_UART_UPSMR_RPM_MASK: c_uint = 0x000c;
pub const UCC_UART_UPSMR_RPM_ODD: c_uint = 0x0000;
pub const UCC_UART_UPSMR_RPM_LOW: c_uint = 0x0004;
pub const UCC_UART_UPSMR_RPM_EVEN: c_uint = 0x0008;
pub const UCC_UART_UPSMR_RPM_HIGH: c_uint = 0x000C;
pub const UCC_UART_UPSMR_TPM_MASK: c_uint = 0x0003;
pub const UCC_UART_UPSMR_TPM_ODD: c_uint = 0x0000;
pub const UCC_UART_UPSMR_TPM_LOW: c_uint = 0x0001;
pub const UCC_UART_UPSMR_TPM_EVEN: c_uint = 0x0002;
pub const UCC_UART_UPSMR_TPM_HIGH: c_uint = 0x0003;
// UCC Protocol Specific Mode Register (UPSMR), when used for Ethernet
pub const UCC_GETH_UPSMR_FTFE: c_uint = 0x80000000;
pub const UCC_GETH_UPSMR_PTPE: c_uint = 0x40000000;
pub const UCC_GETH_UPSMR_ECM: c_uint = 0x04000000;
pub const UCC_GETH_UPSMR_HSE: c_uint = 0x02000000;
pub const UCC_GETH_UPSMR_PRO: c_uint = 0x00400000;
pub const UCC_GETH_UPSMR_CAP: c_uint = 0x00200000;
pub const UCC_GETH_UPSMR_RSH: c_uint = 0x00100000;
pub const UCC_GETH_UPSMR_RPM: c_uint = 0x00080000;
pub const UCC_GETH_UPSMR_R10M: c_uint = 0x00040000;
pub const UCC_GETH_UPSMR_RLPB: c_uint = 0x00020000;
pub const UCC_GETH_UPSMR_TBIM: c_uint = 0x00010000;
pub const UCC_GETH_UPSMR_RES1: c_uint = 0x00002000;
pub const UCC_GETH_UPSMR_RMM: c_uint = 0x00001000;
pub const UCC_GETH_UPSMR_CAM: c_uint = 0x00000400;
pub const UCC_GETH_UPSMR_BRO: c_uint = 0x00000200;
pub const UCC_GETH_UPSMR_SMM: c_uint = 0x00000080;
pub const UCC_GETH_UPSMR_SGMM: c_uint = 0x00000020;
// UCC Protocol Specific Mode Register (UPSMR), when used for HDLC
pub const UCC_HDLC_UPSMR_RTE: c_uint = 0x02000000;
pub const UCC_HDLC_UPSMR_BUS: c_uint = 0x00200000;
pub const UCC_HDLC_UPSMR_CW8: c_uint = 0x00007000;
// UCC Transmit On Demand Register (UTODR)
pub const UCC_SLOW_TOD: c_uint = 0x8000;
pub const UCC_FAST_TOD: c_uint = 0x8000;
// UCC Bus Mode Register masks
// Not to be confused with the Bundle Mode Register
pub const UCC_BMR_GBL: c_uint = 0x20;
pub const UCC_BMR_BO_BE: c_uint = 0x10;
pub const UCC_BMR_CETM: c_uint = 0x04;
pub const UCC_BMR_DTB: c_uint = 0x02;
pub const UCC_BMR_BDB: c_uint = 0x01;
// Function code masks
pub const FC_GBL: c_uint = 0x20;
pub const FC_DTB_LCL: c_uint = 0x02;
pub const UCC_FAST_FUNCTION_CODE_GBL: c_uint = 0x20;
pub const UCC_FAST_FUNCTION_CODE_DTB_LCL: c_uint = 0x02;
pub const UCC_FAST_FUNCTION_CODE_BDB_LCL: c_uint = 0x01;

