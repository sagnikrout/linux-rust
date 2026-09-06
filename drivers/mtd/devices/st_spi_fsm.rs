//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/devices/st_spi_fsm.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// st_spi_fsm.c	- ST Fast Sequence Mode (FSM) Serial Flash Controller
//
// Author: Angus Clark <angus.clark@st.com>
//
// Copyright (C) 2010-2014 STMicroelectronics Limited
//
// JEDEC probe based on drivers/mtd/devices/m25p80.c
//

//
// FSM SPI Controller Registers
//
pub const SPI_CLOCKDIV: c_uint = 0x0010;
pub const SPI_MODESELECT: c_uint = 0x0018;
pub const SPI_CONFIGDATA: c_uint = 0x0020;
pub const SPI_STA_MODE_CHANGE: c_uint = 0x0028;
pub const SPI_FAST_SEQ_TRANSFER_SIZE: c_uint = 0x0100;
pub const SPI_FAST_SEQ_ADD1: c_uint = 0x0104;
pub const SPI_FAST_SEQ_ADD2: c_uint = 0x0108;
pub const SPI_FAST_SEQ_ADD_CFG: c_uint = 0x010c;
pub const SPI_FAST_SEQ_OPC1: c_uint = 0x0110;
pub const SPI_FAST_SEQ_OPC2: c_uint = 0x0114;
pub const SPI_FAST_SEQ_OPC3: c_uint = 0x0118;
pub const SPI_FAST_SEQ_OPC4: c_uint = 0x011c;
pub const SPI_FAST_SEQ_OPC5: c_uint = 0x0120;
pub const SPI_MODE_BITS: c_uint = 0x0124;
pub const SPI_DUMMY_BITS: c_uint = 0x0128;
pub const SPI_FAST_SEQ_FLASH_STA_DATA: c_uint = 0x012c;
pub const SPI_FAST_SEQ_1: c_uint = 0x0130;
pub const SPI_FAST_SEQ_2: c_uint = 0x0134;
pub const SPI_FAST_SEQ_3: c_uint = 0x0138;
pub const SPI_FAST_SEQ_4: c_uint = 0x013c;
pub const SPI_FAST_SEQ_CFG: c_uint = 0x0140;
pub const SPI_FAST_SEQ_STA: c_uint = 0x0144;
pub const SPI_QUAD_BOOT_SEQ_INIT_1: c_uint = 0x0148;
pub const SPI_QUAD_BOOT_SEQ_INIT_2: c_uint = 0x014c;
pub const SPI_QUAD_BOOT_READ_SEQ_1: c_uint = 0x0150;
pub const SPI_QUAD_BOOT_READ_SEQ_2: c_uint = 0x0154;
pub const SPI_PROGRAM_ERASE_TIME: c_uint = 0x0158;
pub const SPI_MULT_PAGE_REPEAT_SEQ_1: c_uint = 0x015c;
pub const SPI_MULT_PAGE_REPEAT_SEQ_2: c_uint = 0x0160;
pub const SPI_STATUS_WR_TIME_REG: c_uint = 0x0164;
pub const SPI_FAST_SEQ_DATA_REG: c_uint = 0x0300;
//
// Register: SPI_MODESELECT
//
pub const SPI_MODESELECT_CONTIG: c_uint = 0x01;
pub const SPI_MODESELECT_FASTREAD: c_uint = 0x02;
pub const SPI_MODESELECT_DUALIO: c_uint = 0x04;
pub const SPI_MODESELECT_FSM: c_uint = 0x08;
pub const SPI_MODESELECT_QUADBOOT: c_uint = 0x10;
//
// Register: SPI_CONFIGDATA
//
pub const SPI_CFG_DEVICE_ST: c_uint = 0x1;
pub const SPI_CFG_DEVICE_ATMEL: c_uint = 0x4;

//
// Register: SPI_FAST_SEQ_TRANSFER_SIZE
//

//
// Register: SPI_FAST_SEQ_ADD_CFG
//

//
// Register: SPI_FAST_SEQ_n
//

//
// Register: SPI_FAST_SEQ_CFG
//

//
// Register: SPI_MODE_BITS
//

//
// Register: SPI_DUMMY_BITS
//

//
// Register: SPI_FAST_SEQ_FLASH_STA_DATA
//

//
// FSM SPI Instruction Opcodes
//
pub const STFSM_OPC_CMD: c_uint = 0x1;
pub const STFSM_OPC_ADD: c_uint = 0x2;
pub const STFSM_OPC_STA: c_uint = 0x3;
pub const STFSM_OPC_MODE: c_uint = 0x4;
pub const STFSM_OPC_DUMMY: c_uint = 0x5;
pub const STFSM_OPC_DATA: c_uint = 0x6;
pub const STFSM_OPC_WAIT: c_uint = 0x7;
pub const STFSM_OPC_JUMP: c_uint = 0x8;
pub const STFSM_OPC_GOTO: c_uint = 0x9;
pub const STFSM_OPC_STOP: c_uint = 0xF;
//
// FSM SPI Instructions (== opcode + operand).
//

// S25FLxxxS commands
pub const S25FL_CMD_WRITE4_1_1_4: c_uint = 0x34;
pub const S25FL_CMD_SE4: c_uint = 0xdc;
pub const S25FL_CMD_CLSR: c_uint = 0x30;
pub const S25FL_CMD_DYBWR: c_uint = 0xe1;
pub const S25FL_CMD_DYBRD: c_uint = 0xe0;
pub const S25FL_CMD_WRITE4: c_uint = 0x12    /* Note, opcode clashes with;
// 'SPINOR_OP_WRITE_1_4_4'
// as found on N25Qxxx devices!
// Status register
pub const FLASH_STATUS_BUSY: c_uint = 0x01;
pub const FLASH_STATUS_WEL: c_uint = 0x02;
pub const FLASH_STATUS_BP0: c_uint = 0x04;
pub const FLASH_STATUS_BP1: c_uint = 0x08;
pub const FLASH_STATUS_BP2: c_uint = 0x10;
pub const FLASH_STATUS_SRWP0: c_uint = 0x80;
pub const FLASH_STATUS_TIMEOUT: c_uint = 0xff;
// S25FL Error Flags
pub const S25FL_STATUS_E_ERR: c_uint = 0x20;
pub const S25FL_STATUS_P_ERR: c_uint = 0x40;
pub const N25Q_CMD_WRVCR: c_uint = 0x81;
pub const N25Q_CMD_RDVCR: c_uint = 0x85;
pub const N25Q_CMD_RDVECR: c_uint = 0x65;
pub const N25Q_CMD_RDNVCR: c_uint = 0xb5;
pub const N25Q_CMD_WRNVCR: c_uint = 0xb1;

//
// Flags to tweak operation of default read/write/erase routines
//
pub const CFG_READ_TOGGLE_32BIT_ADDR: c_uint = 0x00000001;
pub const CFG_WRITE_TOGGLE_32BIT_ADDR: c_uint = 0x00000002;
pub const CFG_ERASESEC_TOGGLE_32BIT_ADDR: c_uint = 0x00000008;
pub const CFG_S25FL_CHECK_ERROR_FLAGS: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stfsm_seq {
    pub data_size: u32,
    pub addr1: u32,
    pub addr2: u32,
    pub addr_cfg: u32,
    pub seq_opc: [u32; 5],
    pub mode: u32,
    pub dummy: u32,
    pub status: u32,
    pub seq: [u8; 16],
    pub seq_cfg: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stfsm {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub mtd: mtd_info,
    pub lock: mutex,
    pub info: *mut flash_info,
    pub clk: *mut clk,
    pub configuration: u32,
    pub fifo_dir_delay: u32,
    pub booted_from_spi: bool,
    pub reset_signal: bool,
    pub reset_por: bool,
    pub stfsm_seq_read: stfsm_seq,
    pub stfsm_seq_write: stfsm_seq,
    pub stfsm_seq_en_32bit_addr: stfsm_seq,
}

// Parameters to configure a READ or WRITE FSM sequence
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_rw_config {
    pub /: *mut *mut uint32_t flags; / flags to support config,
    pub /: *mut *mut uint8_t cmd; / FLASH command,
    pub /: *mut *mut int write; / Write Sequence,
    pub /: *mut *mut uint8_t addr_pads; / No. of addr pads (MODE & DUMMY),
    pub /: *mut *mut uint8_t data_pads; / No. of data pads,
    pub /: *mut *mut uint8_t mode_data; / MODE data,
    pub /: *mut *mut uint8_t mode_cycles; / No. of MODE cycles,
    pub /: *mut *mut uint8_t dummy_cycles; / No. of DUMMY cycles,
}

// SPI Flash Device Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info {
    pub name: *mut c_char,
//
// JEDEC id zero means "no ID" (most older chips); otherwise it has
// a high byte of zero plus three data bytes: the manufacturer id,
// then a two byte device id.
//
    pub jedec_id: u32,
    pub ext_id: u16,
//
// The size listed here is what works with SPINOR_OP_SE, which isn't
// necessarily called a "sector" by the vendor.
//
    pub sector_size: unsigned,
    pub n_sectors: u16,
    pub flags: u32,
//
// Note, where FAST_READ is supported, freq_max specifies the
// FAST_READ frequency, not the READ frequency.
//
    pub max_freq: u32,
    pub ): *mut *mut int (config)(struct stfsm,
}

    static int stfsm_n25q_config(struct stfsm *fsm);
    static int stfsm_mx25_config(struct stfsm *fsm);
    static int stfsm_s25fl_config(struct stfsm *fsm);
    static int stfsm_w25q_config(struct stfsm *fsm);
    static struct flash_info flash_types[] = {
//
// ST Microelectronics/Numonyx --
// (newer production versions may have feature updates
// (eg faster operating frequency)
//

    { "m25p40",  0x202013, 0,  64 * 1024,   8, M25P_FLAG, 25, core::ptr::null_mut() },
    { "m25p80",  0x202014, 0,  64 * 1024,  16, M25P_FLAG, 25, core::ptr::null_mut() },
    { "m25p16",  0x202015, 0,  64 * 1024,  32, M25P_FLAG, 25, core::ptr::null_mut() },
    { "m25p32",  0x202016, 0,  64 * 1024,  64, M25P_FLAG, 50, core::ptr::null_mut() },
    { "m25p64",  0x202017, 0,  64 * 1024, 128, M25P_FLAG, 50, core::ptr::null_mut() },
    { "m25p128", 0x202018, 0, 256 * 1024,  64, M25P_FLAG, 50, core::ptr::null_mut() },

    FLASH_FLAG_READ_FAST        |	\
    FLASH_FLAG_READ_1_1_2       |	\
    FLASH_FLAG_WRITE_1_1_2)
    { "m25px32", 0x207116, 0,  64 * 1024,  64, M25PX_FLAG, 75, core::ptr::null_mut() },
    { "m25px64", 0x207117, 0,  64 * 1024, 128, M25PX_FLAG, 75, core::ptr::null_mut() },
// Macronix MX25xxx
// - Support for 'FLASH_FLAG_WRITE_1_4_4' is omitted for devices
// where operating frequency must be reduced.
//

    FLASH_FLAG_READ_FAST         |	\
    FLASH_FLAG_READ_1_1_2        |	\
    FLASH_FLAG_READ_1_2_2        |	\
    FLASH_FLAG_READ_1_1_4        |	\
    FLASH_FLAG_SE_4K             |	\
    FLASH_FLAG_SE_32K)
    { "mx25l3255e",  0xc29e16, 0, 64 * 1024, 64,
    (MX25_FLAG | FLASH_FLAG_WRITE_1_4_4), 86,
    stfsm_mx25_config},
    { "mx25l25635e", 0xc22019, 0, 64*1024, 512,
    (MX25_FLAG | FLASH_FLAG_32BIT_ADDR | FLASH_FLAG_RESET), 70,
    stfsm_mx25_config },
    { "mx25l25655e", 0xc22619, 0, 64*1024, 512,
    (MX25_FLAG | FLASH_FLAG_32BIT_ADDR | FLASH_FLAG_RESET), 70,
    stfsm_mx25_config},

    FLASH_FLAG_READ_FAST         |	\
    FLASH_FLAG_READ_1_1_2        |	\
    FLASH_FLAG_READ_1_2_2        |	\
    FLASH_FLAG_READ_1_1_4        |	\
    FLASH_FLAG_READ_1_4_4        |	\
    FLASH_FLAG_WRITE_1_1_2       |	\
    FLASH_FLAG_WRITE_1_2_2       |	\
    FLASH_FLAG_WRITE_1_1_4       |	\
    FLASH_FLAG_WRITE_1_4_4)
    { "n25q128", 0x20ba18, 0, 64 * 1024,  256, N25Q_FLAG, 108,
    stfsm_n25q_config },
    { "n25q256", 0x20ba19, 0, 64 * 1024,  512,
    N25Q_FLAG | FLASH_FLAG_32BIT_ADDR, 108, stfsm_n25q_config },
//
// Spansion S25FLxxxP
// - 256KiB and 64KiB sector variants (identified by ext. JEDEC)
//

    FLASH_FLAG_READ_1_1_2   |	\
    FLASH_FLAG_READ_1_2_2   |	\
    FLASH_FLAG_READ_1_1_4   |	\
    FLASH_FLAG_READ_1_4_4   |	\
    FLASH_FLAG_WRITE_1_1_4  |	\
    FLASH_FLAG_READ_FAST)
    { "s25fl032p",  0x010215, 0x4d00,  64 * 1024,  64, S25FLXXXP_FLAG, 80,
    stfsm_s25fl_config},
    { "s25fl129p0", 0x012018, 0x4d00, 256 * 1024,  64, S25FLXXXP_FLAG, 80,
    stfsm_s25fl_config },
    { "s25fl129p1", 0x012018, 0x4d01,  64 * 1024, 256, S25FLXXXP_FLAG, 80,
    stfsm_s25fl_config },
//
// Spansion S25FLxxxS
// - 256KiB and 64KiB sector variants (identified by ext. JEDEC)
// - RESET# signal supported by die but not bristled out on all
// package types.  The package type is a function of board design,
// so this information is captured in the board's flags.
// - Supports 'DYB' sector protection. Depending on variant, sectors
// may default to locked state on power-on.
//

    FLASH_FLAG_RESET        |	\
    FLASH_FLAG_DYB_LOCKING)
    { "s25fl128s0", 0x012018, 0x0300,  256 * 1024, 64, S25FLXXXS_FLAG, 80,
    stfsm_s25fl_config },
    { "s25fl128s1", 0x012018, 0x0301,  64 * 1024, 256, S25FLXXXS_FLAG, 80,
    stfsm_s25fl_config },
    { "s25fl256s0", 0x010219, 0x4d00, 256 * 1024, 128,
    S25FLXXXS_FLAG | FLASH_FLAG_32BIT_ADDR, 80, stfsm_s25fl_config },
    { "s25fl256s1", 0x010219, 0x4d01,  64 * 1024, 512,
    S25FLXXXS_FLAG | FLASH_FLAG_32BIT_ADDR, 80, stfsm_s25fl_config },
// Winbond -- w25x "blocks" are 64K, "sectors" are 4KiB

    FLASH_FLAG_READ_FAST         |	\
    FLASH_FLAG_READ_1_1_2        |	\
    FLASH_FLAG_WRITE_1_1_2)
    { "w25x40",  0xef3013, 0,  64 * 1024,   8, W25X_FLAG, 75, core::ptr::null_mut() },
    { "w25x80",  0xef3014, 0,  64 * 1024,  16, W25X_FLAG, 75, core::ptr::null_mut() },
    { "w25x16",  0xef3015, 0,  64 * 1024,  32, W25X_FLAG, 75, core::ptr::null_mut() },
    { "w25x32",  0xef3016, 0,  64 * 1024,  64, W25X_FLAG, 75, core::ptr::null_mut() },
    { "w25x64",  0xef3017, 0,  64 * 1024, 128, W25X_FLAG, 75, core::ptr::null_mut() },
// Winbond -- w25q "blocks" are 64K, "sectors" are 4KiB

    FLASH_FLAG_READ_FAST         |	\
    FLASH_FLAG_READ_1_1_2        |	\
    FLASH_FLAG_READ_1_2_2        |	\
    FLASH_FLAG_READ_1_1_4        |	\
    FLASH_FLAG_READ_1_4_4        |	\
    FLASH_FLAG_WRITE_1_1_4)
    { "w25q80",  0xef4014, 0,  64 * 1024,  16, W25Q_FLAG, 80,
    stfsm_w25q_config },
    { "w25q16",  0xef4015, 0,  64 * 1024,  32, W25Q_FLAG, 80,
    stfsm_w25q_config },
    { "w25q32",  0xef4016, 0,  64 * 1024,  64, W25Q_FLAG, 80,
    stfsm_w25q_config },
    { "w25q64",  0xef4017, 0,  64 * 1024, 128, W25Q_FLAG, 80,
    stfsm_w25q_config },
// Sentinel
    { core::ptr::null_mut(), 0x000000, 0, 0, 0, 0, 0, core::ptr::null_mut() },
    };
//
// FSM message sequence configurations:
//
// All configs are presented in order of preference
//
// Default READ configurations, in order of preference
    static struct seq_rw_config default_read_configs[] = {
    {FLASH_FLAG_READ_1_4_4, SPINOR_OP_READ_1_4_4,	0, 4, 4, 0x00, 2, 4},
    {FLASH_FLAG_READ_1_1_4, SPINOR_OP_READ_1_1_4,	0, 1, 4, 0x00, 4, 0},
    {FLASH_FLAG_READ_1_2_2, SPINOR_OP_READ_1_2_2,	0, 2, 2, 0x00, 4, 0},
    {FLASH_FLAG_READ_1_1_2, SPINOR_OP_READ_1_1_2,	0, 1, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_FAST,	SPINOR_OP_READ_FAST,	0, 1, 1, 0x00, 0, 8},
    {FLASH_FLAG_READ_WRITE, SPINOR_OP_READ,		0, 1, 1, 0x00, 0, 0},
    {0x00,			0,			0, 0, 0, 0x00, 0, 0},
    };
// Default WRITE configurations
    static struct seq_rw_config default_write_configs[] = {
    {FLASH_FLAG_WRITE_1_4_4, SPINOR_OP_WRITE_1_4_4, 1, 4, 4, 0x00, 0, 0},
    {FLASH_FLAG_WRITE_1_1_4, SPINOR_OP_WRITE_1_1_4, 1, 1, 4, 0x00, 0, 0},
    {FLASH_FLAG_WRITE_1_2_2, SPINOR_OP_WRITE_1_2_2, 1, 2, 2, 0x00, 0, 0},
    {FLASH_FLAG_WRITE_1_1_2, SPINOR_OP_WRITE_1_1_2, 1, 1, 2, 0x00, 0, 0},
    {FLASH_FLAG_READ_WRITE,  SPINOR_OP_WRITE,       1, 1, 1, 0x00, 0, 0},
    {0x00,			 0,			0, 0, 0, 0x00, 0, 0},
    };
//
// [N25Qxxx] Configuration
//

pub const N25Q_VCR_WRAP_CONT: c_uint = 0x3;
// N25Q 3-byte Address READ configurations
// - 'FAST' variants configured for 8 dummy cycles.
//
// Note, the number of dummy cycles used for 'FAST' READ operations is
// configurable and would normally be tuned according to the READ command and
// operating frequency.  However, this applies universally to all 'FAST' READ
// commands, including those used by the SPIBoot controller, and remains in
// force until the device is power-cycled.  Since the SPIBoot controller is
// hard-wired to use 8 dummy cycles, we must configure the device to also use 8
// cycles.
//
    static struct seq_rw_config n25q_read3_configs[] = {
    {FLASH_FLAG_READ_1_4_4, SPINOR_OP_READ_1_4_4,	0, 4, 4, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_1_4, SPINOR_OP_READ_1_1_4,	0, 1, 4, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_2_2, SPINOR_OP_READ_1_2_2,	0, 2, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_1_2, SPINOR_OP_READ_1_1_2,	0, 1, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_FAST,	SPINOR_OP_READ_FAST,	0, 1, 1, 0x00, 0, 8},
    {FLASH_FLAG_READ_WRITE, SPINOR_OP_READ,	        0, 1, 1, 0x00, 0, 0},
    {0x00,			0,			0, 0, 0, 0x00, 0, 0},
    };
// N25Q 4-byte Address READ configurations
// - use special 4-byte address READ commands (reduces overheads, and
// reduces risk of hitting watchdog reset issues).
// - 'FAST' variants configured for 8 dummy cycles (see note above.)
//
    static struct seq_rw_config n25q_read4_configs[] = {
    {FLASH_FLAG_READ_1_4_4, SPINOR_OP_READ_1_4_4_4B, 0, 4, 4, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_1_4, SPINOR_OP_READ_1_1_4_4B, 0, 1, 4, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_2_2, SPINOR_OP_READ_1_2_2_4B, 0, 2, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_1_2, SPINOR_OP_READ_1_1_2_4B, 0, 1, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_FAST,	SPINOR_OP_READ_FAST_4B,  0, 1, 1, 0x00, 0, 8},
    {FLASH_FLAG_READ_WRITE, SPINOR_OP_READ_4B,       0, 1, 1, 0x00, 0, 0},
    {0x00,			0,                       0, 0, 0, 0x00, 0, 0},
    };
//
// [MX25xxx] Configuration
//

#[no_mangle]
unsafe extern "C" fn stfsm_mx25_en_32bit_addr_seq(seq: *mut stfsm_seq) -> c_int {
    static int stfsm_mx25_en_32bit_addr_seq(struct stfsm_seq *seq)
    {
    seq.seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_EN4B) |
    SEQ_OPC_CSDEASSERT);
    seq.seq[0] = STFSM_INST_CMD1;
    seq.seq[1] = STFSM_INST_WAIT;
    seq.seq[2] = STFSM_INST_STOP;
    seq.seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_ERASE |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ);
    return 0;
    }
//
// [S25FLxxx] Configuration
//

//
// S25FLxxxS devices provide three ways of supporting 32-bit addressing: Bank
// Register, Extended Address Modes, and a 32-bit address command set.  The
// 32-bit address command set is used here, since it avoids any problems with
// entering a state that is incompatible with the SPIBoot Controller.
//
    static struct seq_rw_config stfsm_s25fl_read4_configs[] = {
    {FLASH_FLAG_READ_1_4_4,  SPINOR_OP_READ_1_4_4_4B,  0, 4, 4, 0x00, 2, 4},
    {FLASH_FLAG_READ_1_1_4,  SPINOR_OP_READ_1_1_4_4B,  0, 1, 4, 0x00, 0, 8},
    {FLASH_FLAG_READ_1_2_2,  SPINOR_OP_READ_1_2_2_4B,  0, 2, 2, 0x00, 4, 0},
    {FLASH_FLAG_READ_1_1_2,  SPINOR_OP_READ_1_1_2_4B,  0, 1, 2, 0x00, 0, 8},
    {FLASH_FLAG_READ_FAST,   SPINOR_OP_READ_FAST_4B,   0, 1, 1, 0x00, 0, 8},
    {FLASH_FLAG_READ_WRITE,  SPINOR_OP_READ_4B,        0, 1, 1, 0x00, 0, 0},
    {0x00,                   0,                        0, 0, 0, 0x00, 0, 0},
    };
    static struct seq_rw_config stfsm_s25fl_write4_configs[] = {
    {FLASH_FLAG_WRITE_1_1_4, S25FL_CMD_WRITE4_1_1_4, 1, 1, 4, 0x00, 0, 0},
    {FLASH_FLAG_READ_WRITE,  S25FL_CMD_WRITE4,       1, 1, 1, 0x00, 0, 0},
    {0x00,                   0,                      0, 0, 0, 0x00, 0, 0},
    };
//
// [W25Qxxx] Configuration
//

    static struct stfsm_seq stfsm_seq_read_jedec = {
    .data_size = TRANSFER_SIZE(8),
    .seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_RDID)),
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_DATA_READ,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    static struct stfsm_seq stfsm_seq_read_status_fifo = {
    .data_size = TRANSFER_SIZE(4),
    .seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_RDSR)),
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_DATA_READ,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    static struct stfsm_seq stfsm_seq_erase_sector = {
// 'addr_cfg' configured during initialisation
    .seq_opc = {
    (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) | SEQ_OPC_CSDEASSERT),
    (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_SE)),
    },
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_CMD2,
    STFSM_INST_ADD1,
    STFSM_INST_ADD2,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    static struct stfsm_seq stfsm_seq_erase_chip = {
    .seq_opc = {
    (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) | SEQ_OPC_CSDEASSERT),
    (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_CHIP_ERASE) | SEQ_OPC_CSDEASSERT),
    },
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_CMD2,
    STFSM_INST_WAIT,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_ERASE |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    static struct stfsm_seq stfsm_seq_write_status = {
    .seq_opc[0] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) | SEQ_OPC_CSDEASSERT),
    .seq_opc[1] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WRSR)),
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_CMD2,
    STFSM_INST_STA_WR1,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
// Dummy sequence to read one byte of data from flash into the FIFO
    static const struct stfsm_seq stfsm_seq_load_fifo_byte = {
    .data_size = TRANSFER_SIZE(1),
    .seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_RDID)),
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_DATA_READ,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
#[no_mangle]
unsafe extern "C" fn stfsm_n25q_en_32bit_addr_seq(seq: *mut stfsm_seq) -> c_int {
    static int stfsm_n25q_en_32bit_addr_seq(struct stfsm_seq *seq)
    {
    seq.seq_opc[0] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_EN4B));
    seq.seq_opc[1] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) |
    SEQ_OPC_CSDEASSERT);
    seq.seq[0] = STFSM_INST_CMD2;
    seq.seq[1] = STFSM_INST_CMD1;
    seq.seq[2] = STFSM_INST_WAIT;
    seq.seq[3] = STFSM_INST_STOP;
    seq.seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_ERASE |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn stfsm_is_idle(fsm: *mut stfsm) -> c_int {
    static inline int stfsm_is_idle(struct stfsm *fsm)
    {
    return readl(fsm.base + SPI_FAST_SEQ_STA) & 0x10;
    }
#[no_mangle]
pub unsafe extern "C" fn stfsm_fifo_available(fsm: *mut stfsm) -> u32 {
    static inline uint32_t stfsm_fifo_available(struct stfsm *fsm)
    {
    return (readl(fsm.base + SPI_FAST_SEQ_STA) >> 5) & 0x7f;
    }
    static inline void stfsm_load_seq(struct stfsm *fsm,
    const struct stfsm_seq *seq)
    {
    void __iomem *dst = fsm.base + SPI_FAST_SEQ_TRANSFER_SIZE;
    const uint32_t *src = (const uint32_t *)seq;
    let mut words: c_int = sizeof(*seq) / sizeof(*src);
    BUG_ON(!stfsm_is_idle(fsm));
    while (words--) {
    writel(*src, dst);
    src++;
    dst += 4;
    }
    }
#[no_mangle]
unsafe extern "C" fn stfsm_wait_seq(fsm: *mut stfsm) {
    static void stfsm_wait_seq(struct stfsm *fsm)
    {
    unsigned long deadline;
    let mut timeout: c_int = 0;
    deadline = jiffies + msecs_to_jiffies(STFSM_MAX_WAIT_SEQ_MS);
    while (!timeout) {
    if (time_after_eq(jiffies, deadline))
    timeout = 1;
    if (stfsm_is_idle(fsm))
    return;
    cond_resched();
    }
    dev_err(fsm.dev, "timeout on sequence completion\n");
    }
#[no_mangle]
unsafe extern "C" fn stfsm_read_fifo(fsm: *mut stfsm, buf: *mut u32, size: u32) {
    static void stfsm_read_fifo(struct stfsm *fsm, uint32_t *buf, uint32_t size)
    {
    let mut remaining: u32 = size >> 2;
    uint32_t avail;
    uint32_t words;
    dev_dbg(fsm.dev, "Reading %d bytes from FIFO\n", size);
    BUG_ON((((uintptr_t)buf) & 0x3) || (size & 0x3));
    while (remaining) {
    for (;;) {
    avail = stfsm_fifo_available(fsm);
    if (avail)
    break;
    udelay(1);
    }
    words = min(avail, remaining);
    remaining -= words;
    readsl(fsm.base + SPI_FAST_SEQ_DATA_REG, buf, words);
    buf += words;
    }
    }
//
// Clear the data FIFO
//
// Typically, this is only required during driver initialisation, where no
// assumptions can be made regarding the state of the FIFO.
//
// The process of clearing the FIFO is complicated by fact that while it is
// possible for the FIFO to contain an arbitrary number of bytes [1], the
// SPI_FAST_SEQ_STA register only reports the number of complete 32-bit words
// present.  Furthermore, data can only be drained from the FIFO by reading
// complete 32-bit words.
//
// With this in mind, a two stage process is used to the clear the FIFO:
//
// 1. Read any complete 32-bit words from the FIFO, as reported by the
// SPI_FAST_SEQ_STA register.
//
// 2. Mop up any remaining bytes.  At this point, it is not known if there
// are 0, 1, 2, or 3 bytes in the FIFO.  To handle all cases, a dummy FSM
// sequence is used to load one byte at a time, until a complete 32-bit
// word is formed; at most, 4 bytes will need to be loaded.
//
// [1] It is theoretically possible for the FIFO to contain an arbitrary number
// of bits.  However, since there are no known use-cases that leave
// incomplete bytes in the FIFO, only words and bytes are considered here.
//
#[no_mangle]
unsafe extern "C" fn stfsm_clear_fifo(fsm: *mut stfsm) {
    static void stfsm_clear_fifo(struct stfsm *fsm)
    {
    const struct stfsm_seq *seq = &stfsm_seq_load_fifo_byte;
    uint32_t words, i;
// 1. Clear any 32-bit words
    words = stfsm_fifo_available(fsm);
    if (words) {
    for (i = 0; i < words; i++)
    readl(fsm.base + SPI_FAST_SEQ_DATA_REG);
    dev_dbg(fsm.dev, "cleared %d words from FIFO\n", words);
    }
//
// 2. Clear any remaining bytes
// - Load the FIFO, one byte at a time, until a complete 32-bit word
// is available.
//
    for (i = 0, words = 0; i < 4 && !words; i++) {
    stfsm_load_seq(fsm, seq);
    stfsm_wait_seq(fsm);
    words = stfsm_fifo_available(fsm);
    }
// - A single word must be available now
    if (words != 1) {
    dev_err(fsm.dev, "failed to clear bytes from the data FIFO\n");
    return;
    }
// - Read the 32-bit word
    readl(fsm.base + SPI_FAST_SEQ_DATA_REG);
    dev_dbg(fsm.dev, "cleared %d byte(s) from the data FIFO\n", 4 - i);
    }
    static int stfsm_write_fifo(struct stfsm *fsm, const uint32_t *buf,
    uint32_t size)
    {
    let mut words: u32 = size >> 2;
    dev_dbg(fsm.dev, "writing %d bytes to FIFO\n", size);
    BUG_ON((((uintptr_t)buf) & 0x3) || (size & 0x3));
    writesl(fsm.base + SPI_FAST_SEQ_DATA_REG, buf, words);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_enter_32bit_addr(fsm: *mut stfsm, enter: c_int) -> c_int {
    static int stfsm_enter_32bit_addr(struct stfsm *fsm, int enter)
    {
    struct stfsm_seq *seq = &fsm.stfsm_seq_en_32bit_addr;
    let mut cmd: u32 = enter ? SPINOR_OP_EN4B : SPINOR_OP_EX4B;
    seq.seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(cmd) |
    SEQ_OPC_CSDEASSERT);
    stfsm_load_seq(fsm, seq);
    stfsm_wait_seq(fsm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_wait_busy(fsm: *mut stfsm) -> u8 {
    static uint8_t stfsm_wait_busy(struct stfsm *fsm)
    {
    struct stfsm_seq *seq = &stfsm_seq_read_status_fifo;
    unsigned long deadline;
    uint32_t status;
    let mut timeout: c_int = 0;
// Use RDRS1
    seq.seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_RDSR));
// Load read_status sequence
    stfsm_load_seq(fsm, seq);
//
// Repeat until busy bit is deasserted, or timeout, or error (S25FLxxxS)
//
    deadline = jiffies + FLASH_MAX_BUSY_WAIT;
    while (!timeout) {
    if (time_after_eq(jiffies, deadline))
    timeout = 1;
    stfsm_wait_seq(fsm);
    stfsm_read_fifo(fsm, &status, 4);
    if ((status & FLASH_STATUS_BUSY) == 0)
    return 0;
    if ((fsm.configuration & CFG_S25FL_CHECK_ERROR_FLAGS) &&
    ((status & S25FL_STATUS_P_ERR) ||
    (status & S25FL_STATUS_E_ERR)))
    return (uint8_t)(status & 0xff);
    if (!timeout)
// Restart
    writel(seq.seq_cfg, fsm.base + SPI_FAST_SEQ_CFG);
    cond_resched();
    }
    dev_err(fsm.dev, "timeout on wait_busy\n");
    return FLASH_STATUS_TIMEOUT;
    }
    static int stfsm_read_status(struct stfsm *fsm, uint8_t cmd,
    uint8_t *data, int bytes)
    {
    struct stfsm_seq *seq = &stfsm_seq_read_status_fifo;
    uint32_t tmp;
    uint8_t *t = (uint8_t *)&tmp;
    int i;
    dev_dbg(fsm.dev, "read 'status' register [0x%02x], %d byte(s)\n",
    cmd, bytes);
    BUG_ON(bytes != 1 && bytes != 2);
    seq.seq_opc[0] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(cmd));
    stfsm_load_seq(fsm, seq);
    stfsm_read_fifo(fsm, &tmp, 4);
    for (i = 0; i < bytes; i++)
    data[i] = t[i];
    stfsm_wait_seq(fsm);
    return 0;
    }
    static int stfsm_write_status(struct stfsm *fsm, uint8_t cmd,
    uint16_t data, int bytes, int wait_busy)
    {
    struct stfsm_seq *seq = &stfsm_seq_write_status;
    dev_dbg(fsm.dev,
    "write 'status' register [0x%02x], %d byte(s), 0x%04x\n"
    " %s wait-busy\n", cmd, bytes, data, wait_busy ? "with" : "no");
    BUG_ON(bytes != 1 && bytes != 2);
    seq.seq_opc[1] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(cmd));
    seq.status = (uint32_t)data | STA_PADS_1 | STA_CSDEASSERT;
    seq.seq[2] = (bytes == 1) ? STFSM_INST_STA_WR1 : STFSM_INST_STA_WR1_2;
    stfsm_load_seq(fsm, seq);
    stfsm_wait_seq(fsm);
    if (wait_busy)
    stfsm_wait_busy(fsm);
    return 0;
    }
//
// SoC reset on 'boot-from-spi' systems
//
// Certain modes of operation cause the Flash device to enter a particular state
// for a period of time (e.g. 'Erase Sector', 'Quad Enable', and 'Enter 32-bit
// Addr' commands).  On boot-from-spi systems, it is important to consider what
// happens if a warm reset occurs during this period.  The SPIBoot controller
// assumes that Flash device is in its default reset state, 24-bit address mode,
// and ready to accept commands.  This can be achieved using some form of
// on-board logic/controller to force a device POR in response to a SoC-level
// reset or by making use of the device reset signal if available (limited
// number of devices only).
//
// Failure to take such precautions can cause problems following a warm reset.
// For some operations (e.g. ERASE), there is little that can be done.  For
// other modes of operation (e.g. 32-bit addressing), options are often
// available that can help minimise the window in which a reset could cause a
// problem.
//
#[no_mangle]
unsafe extern "C" fn stfsm_can_handle_soc_reset(fsm: *mut stfsm) -> bool {
    static bool stfsm_can_handle_soc_reset(struct stfsm *fsm)
    {
// Reset signal is available on the board and supported by the device
    if (fsm.reset_signal && fsm.info.flags & FLASH_FLAG_RESET)
    return true;
// Board-level logic forces a power-on-reset
    if (fsm.reset_por)
    return true;
// Reset is not properly handled and may result in failure to reboot
    return false;
    }
// Configure 'addr_cfg' according to addressing mode
    static void stfsm_prepare_erasesec_seq(struct stfsm *fsm,
    struct stfsm_seq *seq)
    {
    let mut addr1_cycles: c_int = fsm.info.flags & FLASH_FLAG_32BIT_ADDR ? 16 : 8;
    seq.addr_cfg = (ADR_CFG_CYCLES_ADD1(addr1_cycles) |
    ADR_CFG_PADS_1_ADD1 |
    ADR_CFG_CYCLES_ADD2(16) |
    ADR_CFG_PADS_1_ADD2 |
    ADR_CFG_CSDEASSERT_ADD2);
    }
// Search for preferred configuration based on available flags
    static struct seq_rw_config *
    stfsm_search_seq_rw_configs(struct stfsm *fsm,
    struct seq_rw_config cfgs[])
    {
    struct seq_rw_config *config;
    let mut flags: c_int = fsm.info.flags;
    for (config = cfgs; config.cmd != 0; config++)
    if ((config.flags & flags) == config.flags)
    return config;
    return core::ptr::null_mut();
    }
// Prepare a READ/WRITE sequence according to configuration parameters
    static void stfsm_prepare_rw_seq(struct stfsm *fsm,
    struct stfsm_seq *seq,
    struct seq_rw_config *cfg)
    {
    int addr1_cycles, addr2_cycles;
    let mut i: c_int = 0;
    memset(seq, 0, sizeof(*seq));
// Add READ/WRITE OPC
    seq.seq_opc[i++] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(cfg.cmd));
// Add WREN OPC for a WRITE sequence
    if (cfg.write)
    seq.seq_opc[i++] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) |
    SEQ_OPC_CSDEASSERT);
// Address configuration (24 or 32-bit addresses)
    addr1_cycles  = (fsm.info.flags & FLASH_FLAG_32BIT_ADDR) ? 16 : 8;
    addr1_cycles /= cfg.addr_pads;
    addr2_cycles  = 16 / cfg.addr_pads;
    seq.addr_cfg = ((addr1_cycles & 0x3f) << 0 |	/* ADD1 cycles */
    (cfg.addr_pads - 1) << 6 |	/* ADD1 pads */
    (addr2_cycles & 0x3f) << 16 |	/* ADD2 cycles */
    ((cfg.addr_pads - 1) << 22));	/* ADD2 pads */
// Data/Sequence configuration
    seq.seq_cfg = ((cfg.data_pads - 1) << 16 |
    SEQ_CFG_STARTSEQ |
    SEQ_CFG_CSDEASSERT);
    if (!cfg.write)
    seq.seq_cfg |= SEQ_CFG_READNOTWRITE;
// Mode configuration (no. of pads taken from addr cfg)
    seq.mode = ((cfg.mode_data & 0xff) << 0 |	/* data */
    (cfg.mode_cycles & 0x3f) << 16 |	/* cycles */
    (cfg.addr_pads - 1) << 22);	/* pads */
// Dummy configuration (no. of pads taken from addr cfg)
    seq.dummy = ((cfg.dummy_cycles & 0x3f) << 16 |	/* cycles */
    (cfg.addr_pads - 1) << 22);		/* pads */
// Instruction sequence
    i = 0;
    if (cfg.write)
    seq.seq[i++] = STFSM_INST_CMD2;
    seq.seq[i++] = STFSM_INST_CMD1;
    seq.seq[i++] = STFSM_INST_ADD1;
    seq.seq[i++] = STFSM_INST_ADD2;
    if (cfg.mode_cycles)
    seq.seq[i++] = STFSM_INST_MODE;
    if (cfg.dummy_cycles)
    seq.seq[i++] = STFSM_INST_DUMMY;
    seq.seq[i++] =
    cfg.write ? STFSM_INST_DATA_WRITE : STFSM_INST_DATA_READ;
    seq.seq[i++] = STFSM_INST_STOP;
    }
    static int stfsm_search_prepare_rw_seq(struct stfsm *fsm,
    struct stfsm_seq *seq,
    struct seq_rw_config *cfgs)
    {
    struct seq_rw_config *config;
    config = stfsm_search_seq_rw_configs(fsm, cfgs);
    if (!config) {
    dev_err(fsm.dev, "failed to find suitable config\n");
    return -EINVAL;
    }
    stfsm_prepare_rw_seq(fsm, seq, config);
    return 0;
    }
// Prepare a READ/WRITE/ERASE 'default' sequences
#[no_mangle]
unsafe extern "C" fn stfsm_prepare_rwe_seqs_default(fsm: *mut stfsm) -> c_int {
    static int stfsm_prepare_rwe_seqs_default(struct stfsm *fsm)
    {
    let mut flags: u32 = fsm.info.flags;
    int ret;
// Configure 'READ' sequence
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_read,
    default_read_configs);
    if (ret) {
    dev_err(fsm.dev,
    "failed to prep READ sequence with flags [0x%08x]\n",
    flags);
    return ret;
    }
// Configure 'WRITE' sequence
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_write,
    default_write_configs);
    if (ret) {
    dev_err(fsm.dev,
    "failed to prep WRITE sequence with flags [0x%08x]\n",
    flags);
    return ret;
    }
// Configure 'ERASE_SECTOR' sequence
    stfsm_prepare_erasesec_seq(fsm, &stfsm_seq_erase_sector);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_mx25_config(fsm: *mut stfsm) -> c_int {
    static int stfsm_mx25_config(struct stfsm *fsm)
    {
    let mut flags: u32 = fsm.info.flags;
    uint32_t data_pads;
    uint8_t sta;
    int ret;
    bool soc_reset;
//
// Use default READ/WRITE sequences
//
    ret = stfsm_prepare_rwe_seqs_default(fsm);
    if (ret)
    return ret;
//
// Configure 32-bit Address Support
//
    if (flags & FLASH_FLAG_32BIT_ADDR) {
// Configure 'enter_32bitaddr' FSM sequence
    stfsm_mx25_en_32bit_addr_seq(&fsm.stfsm_seq_en_32bit_addr);
    soc_reset = stfsm_can_handle_soc_reset(fsm);
    if (soc_reset || !fsm.booted_from_spi)
// If we can handle SoC resets, we enable 32-bit address
// mode pervasively
    stfsm_enter_32bit_addr(fsm, 1);
    else
// Else, enable/disable 32-bit addressing before/after
// each operation
    fsm.configuration = (CFG_READ_TOGGLE_32BIT_ADDR |
    CFG_WRITE_TOGGLE_32BIT_ADDR |
    CFG_ERASESEC_TOGGLE_32BIT_ADDR);
    }
// Check status of 'QE' bit, update if required.
    stfsm_read_status(fsm, SPINOR_OP_RDSR, &sta, 1);
    data_pads = ((fsm.stfsm_seq_read.seq_cfg >> 16) & 0x3) + 1;
    if (data_pads == 4) {
    if (!(sta & MX25_STATUS_QE)) {
// Set 'QE'
    sta |= MX25_STATUS_QE;
    stfsm_write_status(fsm, SPINOR_OP_WRSR, sta, 1, 1);
    }
    } else {
    if (sta & MX25_STATUS_QE) {
// Clear 'QE'
    sta &= ~MX25_STATUS_QE;
    stfsm_write_status(fsm, SPINOR_OP_WRSR, sta, 1, 1);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_n25q_config(fsm: *mut stfsm) -> c_int {
    static int stfsm_n25q_config(struct stfsm *fsm)
    {
    let mut flags: u32 = fsm.info.flags;
    uint8_t vcr;
    let mut ret: c_int = 0;
    bool soc_reset;
// Configure 'READ' sequence
    if (flags & FLASH_FLAG_32BIT_ADDR)
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_read,
    n25q_read4_configs);
    else
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_read,
    n25q_read3_configs);
    if (ret) {
    dev_err(fsm.dev,
    "failed to prepare READ sequence with flags [0x%08x]\n",
    flags);
    return ret;
    }
// Configure 'WRITE' sequence (default configs)
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_write,
    default_write_configs);
    if (ret) {
    dev_err(fsm.dev,
    "preparing WRITE sequence using flags [0x%08x] failed\n",
    flags);
    return ret;
    }
// * Configure 'ERASE_SECTOR' sequence
    stfsm_prepare_erasesec_seq(fsm, &stfsm_seq_erase_sector);
// Configure 32-bit address support
    if (flags & FLASH_FLAG_32BIT_ADDR) {
    stfsm_n25q_en_32bit_addr_seq(&fsm.stfsm_seq_en_32bit_addr);
    soc_reset = stfsm_can_handle_soc_reset(fsm);
    if (soc_reset || !fsm.booted_from_spi) {
//
// If we can handle SoC resets, we enable 32-bit
// address mode pervasively
//
    stfsm_enter_32bit_addr(fsm, 1);
    } else {
//
// If not, enable/disable for WRITE and ERASE
// operations (READ uses special commands)
//
    fsm.configuration = (CFG_WRITE_TOGGLE_32BIT_ADDR |
    CFG_ERASESEC_TOGGLE_32BIT_ADDR);
    }
    }
//
// Configure device to use 8 dummy cycles
//
    vcr = (N25Q_VCR_DUMMY_CYCLES(8) | N25Q_VCR_XIP_DISABLED |
    N25Q_VCR_WRAP_CONT);
    stfsm_write_status(fsm, N25Q_CMD_WRVCR, vcr, 1, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_s25fl_prepare_erasesec_seq_32(seq: *mut stfsm_seq) {
    static void stfsm_s25fl_prepare_erasesec_seq_32(struct stfsm_seq *seq)
    {
    seq.seq_opc[1] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(S25FL_CMD_SE4));
    seq.addr_cfg = (ADR_CFG_CYCLES_ADD1(16) |
    ADR_CFG_PADS_1_ADD1 |
    ADR_CFG_CYCLES_ADD2(16) |
    ADR_CFG_PADS_1_ADD2 |
    ADR_CFG_CSDEASSERT_ADD2);
    }
#[no_mangle]
unsafe extern "C" fn stfsm_s25fl_read_dyb(fsm: *mut stfsm, offs: u32, dby: *mut u8) {
    static void stfsm_s25fl_read_dyb(struct stfsm *fsm, uint32_t offs, uint8_t *dby)
    {
    uint32_t tmp;
    struct stfsm_seq seq = {
    .data_size = TRANSFER_SIZE(4),
    .seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(S25FL_CMD_DYBRD)),
    .addr_cfg = (ADR_CFG_CYCLES_ADD1(16) |
    ADR_CFG_PADS_1_ADD1 |
    ADR_CFG_CYCLES_ADD2(16) |
    ADR_CFG_PADS_1_ADD2),
    .addr1 = (offs >> 16) & 0xffff,
    .addr2 = offs & 0xffff,
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_ADD1,
    STFSM_INST_ADD2,
    STFSM_INST_DATA_READ,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    stfsm_load_seq(fsm, &seq);
    stfsm_read_fifo(fsm, &tmp, 4);
// dby = (uint8_t)(tmp >> 24);
    stfsm_wait_seq(fsm);
    }
#[no_mangle]
unsafe extern "C" fn stfsm_s25fl_write_dyb(fsm: *mut stfsm, offs: u32, dby: u8) {
    static void stfsm_s25fl_write_dyb(struct stfsm *fsm, uint32_t offs, uint8_t dby)
    {
    struct stfsm_seq seq = {
    .seq_opc[0] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WREN) |
    SEQ_OPC_CSDEASSERT),
    .seq_opc[1] = (SEQ_OPC_PADS_1 | SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(S25FL_CMD_DYBWR)),
    .addr_cfg = (ADR_CFG_CYCLES_ADD1(16) |
    ADR_CFG_PADS_1_ADD1 |
    ADR_CFG_CYCLES_ADD2(16) |
    ADR_CFG_PADS_1_ADD2),
    .status = (uint32_t)dby | STA_PADS_1 | STA_CSDEASSERT,
    .addr1 = (offs >> 16) & 0xffff,
    .addr2 = offs & 0xffff,
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_CMD2,
    STFSM_INST_ADD1,
    STFSM_INST_ADD2,
    STFSM_INST_STA_WR1,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    stfsm_load_seq(fsm, &seq);
    stfsm_wait_seq(fsm);
    stfsm_wait_busy(fsm);
    }
#[no_mangle]
unsafe extern "C" fn stfsm_s25fl_clear_status_reg(fsm: *mut stfsm) -> c_int {
    static int stfsm_s25fl_clear_status_reg(struct stfsm *fsm)
    {
    struct stfsm_seq seq = {
    .seq_opc[0] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(S25FL_CMD_CLSR) |
    SEQ_OPC_CSDEASSERT),
    .seq_opc[1] = (SEQ_OPC_PADS_1 |
    SEQ_OPC_CYCLES(8) |
    SEQ_OPC_OPCODE(SPINOR_OP_WRDI) |
    SEQ_OPC_CSDEASSERT),
    .seq = {
    STFSM_INST_CMD1,
    STFSM_INST_CMD2,
    STFSM_INST_WAIT,
    STFSM_INST_STOP,
    },
    .seq_cfg = (SEQ_CFG_PADS_1 |
    SEQ_CFG_ERASE |
    SEQ_CFG_READNOTWRITE |
    SEQ_CFG_CSDEASSERT |
    SEQ_CFG_STARTSEQ),
    };
    stfsm_load_seq(fsm, &seq);
    stfsm_wait_seq(fsm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_s25fl_config(fsm: *mut stfsm) -> c_int {
    static int stfsm_s25fl_config(struct stfsm *fsm)
    {
    struct flash_info *info = fsm.info;
    let mut flags: u32 = info.flags;
    uint32_t data_pads;
    uint32_t offs;
    uint16_t sta_wr;
    uint8_t sr1, cr1, dyb;
    let mut update_sr: c_int = 0;
    int ret;
    if (flags & FLASH_FLAG_32BIT_ADDR) {
//
// Prepare Read/Write/Erase sequences according to S25FLxxx
// 32-bit address command set
//
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_read,
    stfsm_s25fl_read4_configs);
    if (ret)
    return ret;
    ret = stfsm_search_prepare_rw_seq(fsm, &fsm.stfsm_seq_write,
    stfsm_s25fl_write4_configs);
    if (ret)
    return ret;
    stfsm_s25fl_prepare_erasesec_seq_32(&stfsm_seq_erase_sector);
    } else {
// Use default configurations for 24-bit addressing
    ret = stfsm_prepare_rwe_seqs_default(fsm);
    if (ret)
    return ret;
    }
//
// For devices that support 'DYB' sector locking, check lock status and
// unlock sectors if necessary (some variants power-on with sectors
// locked by default)
//
    if (flags & FLASH_FLAG_DYB_LOCKING) {
    offs = 0;
    for (offs = 0; offs < info.sector_size * info.n_sectors;) {
    stfsm_s25fl_read_dyb(fsm, offs, &dyb);
    if (dyb == 0x00)
    stfsm_s25fl_write_dyb(fsm, offs, 0xff);
// Handle bottom/top 4KiB parameter sectors
    if ((offs < info.sector_size * 2) ||
    (offs >= (info.sector_size - info.n_sectors * 4)))
    offs += 0x1000;
    else
    offs += 0x10000;
    }
    }
// Check status of 'QE' bit, update if required.
    stfsm_read_status(fsm, SPINOR_OP_RDCR, &cr1, 1);
    data_pads = ((fsm.stfsm_seq_read.seq_cfg >> 16) & 0x3) + 1;
    if (data_pads == 4) {
    if (!(cr1 & STFSM_S25FL_CONFIG_QE)) {
// Set 'QE'
    cr1 |= STFSM_S25FL_CONFIG_QE;
    update_sr = 1;
    }
    } else {
    if (cr1 & STFSM_S25FL_CONFIG_QE) {
// Clear 'QE'
    cr1 &= ~STFSM_S25FL_CONFIG_QE;
    update_sr = 1;
    }
    }
    if (update_sr) {
    stfsm_read_status(fsm, SPINOR_OP_RDSR, &sr1, 1);
    sta_wr = ((uint16_t)cr1  << 8) | sr1;
    stfsm_write_status(fsm, SPINOR_OP_WRSR, sta_wr, 2, 1);
    }
//
// S25FLxxx devices support Program and Error error flags.
// Configure driver to check flags and clear if necessary.
//
    fsm.configuration |= CFG_S25FL_CHECK_ERROR_FLAGS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_w25q_config(fsm: *mut stfsm) -> c_int {
    static int stfsm_w25q_config(struct stfsm *fsm)
    {
    uint32_t data_pads;
    uint8_t sr1, sr2;
    uint16_t sr_wr;
    let mut update_sr: c_int = 0;
    int ret;
    ret = stfsm_prepare_rwe_seqs_default(fsm);
    if (ret)
    return ret;
// Check status of 'QE' bit, update if required.
    stfsm_read_status(fsm, SPINOR_OP_RDCR, &sr2, 1);
    data_pads = ((fsm.stfsm_seq_read.seq_cfg >> 16) & 0x3) + 1;
    if (data_pads == 4) {
    if (!(sr2 & W25Q_STATUS_QE)) {
// Set 'QE'
    sr2 |= W25Q_STATUS_QE;
    update_sr = 1;
    }
    } else {
    if (sr2 & W25Q_STATUS_QE) {
// Clear 'QE'
    sr2 &= ~W25Q_STATUS_QE;
    update_sr = 1;
    }
    }
    if (update_sr) {
// Write status register
    stfsm_read_status(fsm, SPINOR_OP_RDSR, &sr1, 1);
    sr_wr = ((uint16_t)sr2 << 8) | sr1;
    stfsm_write_status(fsm, SPINOR_OP_WRSR, sr_wr, 2, 1);
    }
    return 0;
    }
    static int stfsm_read(struct stfsm *fsm, uint8_t *buf, uint32_t size,
    uint32_t offset)
    {
    struct stfsm_seq *seq = &fsm.stfsm_seq_read;
    uint32_t data_pads;
    uint32_t read_mask;
    uint32_t size_ub;
    uint32_t size_lb;
    uint32_t size_mop;
    uint32_t tmp[4];
    uint32_t page_buf[FLASH_PAGESIZE_32];
    uint8_t *p;
    dev_dbg(fsm.dev, "reading %d bytes from 0x%08x\n", size, offset);
// Enter 32-bit address mode, if required
    if (fsm.configuration & CFG_READ_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 1);
// Must read in multiples of 32 cycles (or 32*pads/8 Bytes)
    data_pads = ((seq.seq_cfg >> 16) & 0x3) + 1;
    read_mask = (data_pads << 2) - 1;
// Handle non-aligned buf
    p = ((uintptr_t)buf & 0x3) ? (uint8_t *)page_buf : buf;
// Handle non-aligned size
    size_ub = (size + read_mask) & ~read_mask;
    size_lb = size & ~read_mask;
    size_mop = size & read_mask;
    seq.data_size = TRANSFER_SIZE(size_ub);
    seq.addr1 = (offset >> 16) & 0xffff;
    seq.addr2 = offset & 0xffff;
    stfsm_load_seq(fsm, seq);
    if (size_lb)
    stfsm_read_fifo(fsm, (uint32_t *)p, size_lb);
    if (size_mop) {
    stfsm_read_fifo(fsm, tmp, read_mask + 1);
    memcpy(p + size_lb, &tmp, size_mop);
    }
// Handle non-aligned buf
    if ((uintptr_t)buf & 0x3)
    memcpy(buf, page_buf, size);
// Wait for sequence to finish
    stfsm_wait_seq(fsm);
    stfsm_clear_fifo(fsm);
// Exit 32-bit address mode, if required
    if (fsm.configuration & CFG_READ_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 0);
    return 0;
    }
    static int stfsm_write(struct stfsm *fsm, const uint8_t *buf,
    uint32_t size, uint32_t offset)
    {
    struct stfsm_seq *seq = &fsm.stfsm_seq_write;
    uint32_t data_pads;
    uint32_t write_mask;
    uint32_t size_ub;
    uint32_t size_lb;
    uint32_t size_mop;
    uint32_t tmp[4];
    uint32_t i;
    uint32_t page_buf[FLASH_PAGESIZE_32];
    uint8_t *t = (uint8_t *)&tmp;
    const uint8_t *p;
    int ret;
    dev_dbg(fsm.dev, "writing %d bytes to 0x%08x\n", size, offset);
// Enter 32-bit address mode, if required
    if (fsm.configuration & CFG_WRITE_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 1);
// Must write in multiples of 32 cycles (or 32*pads/8 bytes)
    data_pads = ((seq.seq_cfg >> 16) & 0x3) + 1;
    write_mask = (data_pads << 2) - 1;
// Handle non-aligned buf
    if ((uintptr_t)buf & 0x3) {
    memcpy(page_buf, buf, size);
    p = (uint8_t *)page_buf;
    } else {
    p = buf;
    }
// Handle non-aligned size
    size_ub = (size + write_mask) & ~write_mask;
    size_lb = size & ~write_mask;
    size_mop = size & write_mask;
    seq.data_size = TRANSFER_SIZE(size_ub);
    seq.addr1 = (offset >> 16) & 0xffff;
    seq.addr2 = offset & 0xffff;
// Need to set FIFO to write mode, before writing data to FIFO (see
// GNBvb79594)
//
    writel(0x00040000, fsm.base + SPI_FAST_SEQ_CFG);
//
// Before writing data to the FIFO, apply a small delay to allow a
// potential change of FIFO direction to complete.
//
    if (fsm.fifo_dir_delay == 0)
    readl(fsm.base + SPI_FAST_SEQ_CFG);
    else
    udelay(fsm.fifo_dir_delay);
// Write data to FIFO, before starting sequence (see GNBvd79593)
    if (size_lb) {
    stfsm_write_fifo(fsm, (uint32_t *)p, size_lb);
    p += size_lb;
    }
// Handle non-aligned size
    if (size_mop) {
    memset(t, 0xff, write_mask + 1);	/* fill with 0xff's */
    for (i = 0; i < size_mop; i++)
    t[i] = *p++;
    stfsm_write_fifo(fsm, tmp, write_mask + 1);
    }
// Start sequence
    stfsm_load_seq(fsm, seq);
// Wait for sequence to finish
    stfsm_wait_seq(fsm);
// Wait for completion
    ret = stfsm_wait_busy(fsm);
    if (ret && fsm.configuration & CFG_S25FL_CHECK_ERROR_FLAGS)
    stfsm_s25fl_clear_status_reg(fsm);
// Exit 32-bit address mode, if required
    if (fsm.configuration & CFG_WRITE_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 0);
    return 0;
    }
//
// Read an address range from the flash chip. The address range
// may be any size provided it is within the physical boundaries.
//
    static int stfsm_mtd_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen, u_char *buf)
    {
    struct stfsm *fsm = dev_get_drvdata(mtd.dev.parent);
    uint32_t bytes;
    dev_dbg(fsm.dev, "%s from 0x%08x, len %zd\n",
    __func__, (u32)from, len);
    mutex_lock(&fsm.lock);
    while (len > 0) {
    bytes = min_t(size_t, len, FLASH_PAGESIZE);
    stfsm_read(fsm, buf, bytes, from);
    buf += bytes;
    from += bytes;
    len -= bytes;
// retlen += bytes;
    }
    mutex_unlock(&fsm.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_erase_sector(fsm: *mut stfsm, offset: u32) -> c_int {
    static int stfsm_erase_sector(struct stfsm *fsm, uint32_t offset)
    {
    struct stfsm_seq *seq = &stfsm_seq_erase_sector;
    int ret;
    dev_dbg(fsm.dev, "erasing sector at 0x%08x\n", offset);
// Enter 32-bit address mode, if required
    if (fsm.configuration & CFG_ERASESEC_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 1);
    seq.addr1 = (offset >> 16) & 0xffff;
    seq.addr2 = offset & 0xffff;
    stfsm_load_seq(fsm, seq);
    stfsm_wait_seq(fsm);
// Wait for completion
    ret = stfsm_wait_busy(fsm);
    if (ret && fsm.configuration & CFG_S25FL_CHECK_ERROR_FLAGS)
    stfsm_s25fl_clear_status_reg(fsm);
// Exit 32-bit address mode, if required
    if (fsm.configuration & CFG_ERASESEC_TOGGLE_32BIT_ADDR)
    stfsm_enter_32bit_addr(fsm, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_erase_chip(fsm: *mut stfsm) -> c_int {
    static int stfsm_erase_chip(struct stfsm *fsm)
    {
    const struct stfsm_seq *seq = &stfsm_seq_erase_chip;
    dev_dbg(fsm.dev, "erasing chip\n");
    stfsm_load_seq(fsm, seq);
    stfsm_wait_seq(fsm);
    return stfsm_wait_busy(fsm);
    }
//
// Write an address range to the flash chip.  Data must be written in
// FLASH_PAGESIZE chunks.  The address range may be any size provided
// it is within the physical boundaries.
//
    static int stfsm_mtd_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    struct stfsm *fsm = dev_get_drvdata(mtd.dev.parent);
    u32 page_offs;
    u32 bytes;
    uint8_t *b = (uint8_t *)buf;
    let mut ret: c_int = 0;
    dev_dbg(fsm.dev, "%s to 0x%08x, len %zd\n", __func__, (u32)to, len);
// Offset within page
    page_offs = to % FLASH_PAGESIZE;
    mutex_lock(&fsm.lock);
    while (len) {
// Write up to page boundary
    bytes = min_t(size_t, FLASH_PAGESIZE - page_offs, len);
    ret = stfsm_write(fsm, b, bytes, to);
    if (ret)
    goto out1;
    b += bytes;
    len -= bytes;
    to += bytes;
// We are now page-aligned
    page_offs = 0;
// retlen += bytes;
    }
    out1:
    mutex_unlock(&fsm.lock);
    return ret;
    }
//
// Erase an address range on the flash chip. The address range may extend
// one or more erase sectors.  Return an error is there is a problem erasing.
//
#[no_mangle]
unsafe extern "C" fn stfsm_mtd_erase(mtd: *mut mtd_info, instr: *mut erase_info) -> c_int {
    static int stfsm_mtd_erase(struct mtd_info *mtd, struct erase_info *instr)
    {
    struct stfsm *fsm = dev_get_drvdata(mtd.dev.parent);
    u32 addr, len;
    int ret;
    dev_dbg(fsm.dev, "%s at 0x%llx, len %lld\n", __func__,
    (long long)instr.addr, (long long)instr.len);
    addr = instr.addr;
    len = instr.len;
    mutex_lock(&fsm.lock);
// Whole-chip erase?
    if (len == mtd.size) {
    ret = stfsm_erase_chip(fsm);
    if (ret)
    goto out1;
    } else {
    while (len) {
    ret = stfsm_erase_sector(fsm, addr);
    if (ret)
    goto out1;
    addr += mtd.erasesize;
    len -= mtd.erasesize;
    }
    }
    mutex_unlock(&fsm.lock);
    return 0;
    out1:
    mutex_unlock(&fsm.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_read_jedec(fsm: *mut stfsm, jedec: *mut u8) {
    static void stfsm_read_jedec(struct stfsm *fsm, uint8_t *jedec)
    {
    const struct stfsm_seq *seq = &stfsm_seq_read_jedec;
    uint32_t tmp[2];
    stfsm_load_seq(fsm, seq);
    stfsm_read_fifo(fsm, tmp, 8);
    memcpy(jedec, tmp, 5);
    stfsm_wait_seq(fsm);
    }
    static struct flash_info *stfsm_jedec_probe(struct stfsm *fsm)
    {
    struct flash_info	*info;
    u16                     ext_jedec;
    u32			jedec;
    u8			id[5];
    stfsm_read_jedec(fsm, id);
    jedec     = id[0] << 16 | id[1] << 8 | id[2];
//
// JEDEC also defines an optional "extended device information"
// string for after vendor-specific data, after the three bytes
// we use here. Supporting some chips might require using it.
//
    ext_jedec = id[3] << 8  | id[4];
    dev_dbg(fsm.dev, "JEDEC =  0x%08x [%5ph]\n", jedec, id);
    for (info = flash_types; info.name; info++) {
    if (info.jedec_id == jedec) {
    if (info.ext_id && info.ext_id != ext_jedec)
    continue;
    return info;
    }
    }
    dev_err(fsm.dev, "Unrecognized JEDEC id %06x\n", jedec);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn stfsm_set_mode(fsm: *mut stfsm, mode: u32) -> c_int {
    static int stfsm_set_mode(struct stfsm *fsm, uint32_t mode)
    {
    int ret, timeout = 10;
// Wait for controller to accept mode change
    while (--timeout) {
    ret = readl(fsm.base + SPI_STA_MODE_CHANGE);
    if (ret & 0x1)
    break;
    udelay(1);
    }
    if (!timeout)
    return -EBUSY;
    writel(mode, fsm.base + SPI_MODESELECT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_set_freq(fsm: *mut stfsm, spi_freq: u32) {
    static void stfsm_set_freq(struct stfsm *fsm, uint32_t spi_freq)
    {
    uint32_t emi_freq;
    uint32_t clk_div;
    emi_freq = clk_get_rate(fsm.clk);
//
// Calculate clk_div - values between 2 and 128
// Multiple of 2, rounded up
//
    clk_div = 2 * DIV_ROUND_UP(emi_freq, 2 * spi_freq);
    if (clk_div < 2)
    clk_div = 2;
#[no_mangle]
pub unsafe extern "C" fn if(128: clk_div >) -> else {
    else if (clk_div > 128)
    clk_div = 128;
//
// Determine a suitable delay for the IP to complete a change of
// direction of the FIFO. The required delay is related to the clock
// divider used. The following heuristics are based on empirical tests,
// using a 100MHz EMI clock.
//
    if (clk_div <= 4)
    fsm.fifo_dir_delay = 0;
#[no_mangle]
pub unsafe extern "C" fn if(10: clk_div <=) -> else {
    else if (clk_div <= 10)
    fsm.fifo_dir_delay = 1;
    else
    fsm.fifo_dir_delay = DIV_ROUND_UP(clk_div, 10);
    dev_dbg(fsm.dev, "emi_clk = %uHZ, spi_freq = %uHZ, clk_div = %u\n",
    emi_freq, spi_freq, clk_div);
    writel(clk_div, fsm.base + SPI_CLOCKDIV);
    }
#[no_mangle]
unsafe extern "C" fn stfsm_init(fsm: *mut stfsm) -> c_int {
    static int stfsm_init(struct stfsm *fsm)
    {
    int ret;
// Perform a soft reset of the FSM controller
    writel(SEQ_CFG_SWRESET, fsm.base + SPI_FAST_SEQ_CFG);
    udelay(1);
    writel(0, fsm.base + SPI_FAST_SEQ_CFG);
// Set clock to 'safe' frequency initially
    stfsm_set_freq(fsm, STFSM_FLASH_SAFE_FREQ);
// Switch to FSM
    ret = stfsm_set_mode(fsm, SPI_MODESELECT_FSM);
    if (ret)
    return ret;
// Set timing parameters
    writel(SPI_CFG_DEVICE_ST            |
    SPI_CFG_DEFAULT_MIN_CS_HIGH  |
    SPI_CFG_DEFAULT_CS_SETUPHOLD |
    SPI_CFG_DEFAULT_DATA_HOLD,
    fsm.base + SPI_CONFIGDATA);
    writel(STFSM_DEFAULT_WR_TIME, fsm.base + SPI_STATUS_WR_TIME_REG);
//
// Set the FSM 'WAIT' delay to the minimum workable value.  Note, for
// our purposes, the WAIT instruction is used purely to achieve
// "sequence validity" rather than actually implement a delay.
//
    writel(0x00000001, fsm.base + SPI_PROGRAM_ERASE_TIME);
// Clear FIFO, just in case
    stfsm_clear_fifo(fsm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsm_fetch_platform_configs(pdev: *mut platform_device) {
    static void stfsm_fetch_platform_configs(struct platform_device *pdev)
    {
    struct stfsm *fsm = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    struct regmap *regmap;
    uint32_t boot_device_reg;
    uint32_t boot_device_spi;
    uint32_t boot_device;     /* Value we read from *boot_device_reg */
    int ret;
// Booting from SPI NOR Flash is the default
    fsm.booted_from_spi = true;
    regmap = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(regmap))
    goto boot_device_fail;
    fsm.reset_signal = of_property_read_bool(np, "st,reset-signal");
    fsm.reset_por = of_property_read_bool(np, "st,reset-por");
// Where in the syscon the boot device information lives
    ret = of_property_read_u32(np, "st,boot-device-reg", &boot_device_reg);
    if (ret)
    goto boot_device_fail;
// Boot device value when booted from SPI NOR
    ret = of_property_read_u32(np, "st,boot-device-spi", &boot_device_spi);
    if (ret)
    goto boot_device_fail;
    ret = regmap_read(regmap, boot_device_reg, &boot_device);
    if (ret)
    goto boot_device_fail;
    if (boot_device != boot_device_spi)
    fsm.booted_from_spi = false;
    return;
    boot_device_fail:
    dev_warn(&pdev.dev,
    "failed to fetch boot device, assuming boot from SPI\n");
    }
#[no_mangle]
unsafe extern "C" fn stfsm_probe(pdev: *mut platform_device) -> c_int {
    static int stfsm_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct flash_info *info;
    struct stfsm *fsm;
    int ret;
    if (!np) {
    dev_err(&pdev.dev, "No DT found\n");
    return -EINVAL;
    }
    fsm = devm_kzalloc(&pdev.dev, sizeof(*fsm), GFP_KERNEL);
    if (!fsm)
    return -ENOMEM;
    fsm.dev = &pdev.dev;
    platform_set_drvdata(pdev, fsm);
    fsm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(fsm.base))
    return PTR_ERR(fsm.base);
    fsm.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(fsm.clk)) {
    dev_err(fsm.dev, "Couldn't find EMI clock.\n");
    return PTR_ERR(fsm.clk);
    }
    mutex_init(&fsm.lock);
    ret = stfsm_init(fsm);
    if (ret) {
    dev_err(&pdev.dev, "Failed to initialise FSM Controller\n");
    return ret;
    }
    stfsm_fetch_platform_configs(pdev);
// Detect SPI FLASH device
    info = stfsm_jedec_probe(fsm);
    if (!info)
    return -ENODEV;
    fsm.info = info;
// Use device size to determine address width
    if (info.sector_size * info.n_sectors > 0x1000000)
    info.flags |= FLASH_FLAG_32BIT_ADDR;
//
// Configure READ/WRITE/ERASE sequences according to platform and
// device flags.
//
    if (info.config)
    ret = info.config(fsm);
    else
    ret = stfsm_prepare_rwe_seqs_default(fsm);
    if (ret)
    return ret;
    fsm.mtd.name		= info.name;
    fsm.mtd.dev.parent	= &pdev.dev;
    mtd_set_of_node(&fsm.mtd, np);
    fsm.mtd.type		= MTD_NORFLASH;
    fsm.mtd.writesize	= 4;
    fsm.mtd.writebufsize	= fsm.mtd.writesize;
    fsm.mtd.flags		= MTD_CAP_NORFLASH;
    fsm.mtd.size		= info.sector_size * info.n_sectors;
    fsm.mtd.erasesize	= info.sector_size;
    fsm.mtd._read  = stfsm_mtd_read;
    fsm.mtd._write = stfsm_mtd_write;
    fsm.mtd._erase = stfsm_mtd_erase;
    dev_info(&pdev.dev,
    "Found serial flash device: %s\n"
    " size = %llx (%lldMiB) erasesize = 0x%08x (%uKiB)\n",
    info.name,
    (long long)fsm.mtd.size, (long long)(fsm.mtd.size >> 20),
    fsm.mtd.erasesize, (fsm.mtd.erasesize >> 10));
    return mtd_device_register(&fsm.mtd, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn stfsm_remove(pdev: *mut platform_device) {
    static void stfsm_remove(struct platform_device *pdev)
    {
    struct stfsm *fsm = platform_get_drvdata(pdev);
    WARN_ON(mtd_device_unregister(&fsm.mtd));
    }
#[no_mangle]
unsafe extern "C" fn stfsmfsm_suspend(dev: *mut device) -> c_int {
    static int stfsmfsm_suspend(struct device *dev)
    {
    struct stfsm *fsm = dev_get_drvdata(dev);
    clk_disable_unprepare(fsm.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stfsmfsm_resume(dev: *mut device) -> c_int {
    static int stfsmfsm_resume(struct device *dev)
    {
    struct stfsm *fsm = dev_get_drvdata(dev);
    return clk_prepare_enable(fsm.clk);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(stfsm_pm_ops, stfsmfsm_suspend, stfsmfsm_resume);
    static const struct of_device_id stfsm_match[] = {
    { .compatible = "st,spi-fsm", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stfsm_match);
    static struct platform_driver stfsm_driver = {
    .probe		= stfsm_probe,
    .remove		= stfsm_remove,
    .driver		= {
    .name	= "st-spi-fsm",
    .of_match_table = stfsm_match,
    .pm     = pm_sleep_ptr(&stfsm_pm_ops),
    },
    };
    module_platform_driver(stfsm_driver);
    MODULE_AUTHOR("Angus Clark <angus.clark@st.com>");
    MODULE_DESCRIPTION("ST SPI FSM driver");
    MODULE_LICENSE("GPL");
