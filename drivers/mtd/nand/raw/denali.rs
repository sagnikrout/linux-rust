//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/denali.h
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
//
// NAND Flash Controller Device Driver
// Copyright (c) 2009 - 2010, Intel Corporation and its suppliers.
//

pub const DEVICE_RESET: c_uint = 0x0;

pub const TRANSFER_SPARE_REG: c_uint = 0x10;

pub const LOAD_WAIT_CNT: c_uint = 0x20;

pub const PROGRAM_WAIT_CNT: c_uint = 0x30;

pub const ERASE_WAIT_CNT: c_uint = 0x40;

pub const INT_MON_CYCCNT: c_uint = 0x50;

pub const RB_PIN_ENABLED: c_uint = 0x60;

pub const MULTIPLANE_OPERATION: c_uint = 0x70;

pub const MULTIPLANE_READ_ENABLE: c_uint = 0x80;

pub const COPYBACK_DISABLE: c_uint = 0x90;

pub const CACHE_WRITE_ENABLE: c_uint = 0xa0;

pub const CACHE_READ_ENABLE: c_uint = 0xb0;

pub const PREFETCH_MODE: c_uint = 0xc0;

pub const CHIP_ENABLE_DONT_CARE: c_uint = 0xd0;

pub const ECC_ENABLE: c_uint = 0xe0;

pub const GLOBAL_INT_ENABLE: c_uint = 0xf0;

pub const TWHR2_AND_WE_2_RE: c_uint = 0x100;

pub const TCWAW_AND_ADDR_2_DATA: c_uint = 0x110;
// The width of ADDR_2_DATA is 6 bit for old IP, 7 bit for new IP

pub const RE_2_WE: c_uint = 0x120;

pub const ACC_CLKS: c_uint = 0x130;

pub const NUMBER_OF_PLANES: c_uint = 0x140;

pub const PAGES_PER_BLOCK: c_uint = 0x150;

pub const DEVICE_WIDTH: c_uint = 0x160;

pub const DEVICE_MAIN_AREA_SIZE: c_uint = 0x170;

pub const DEVICE_SPARE_AREA_SIZE: c_uint = 0x180;

pub const TWO_ROW_ADDR_CYCLES: c_uint = 0x190;

pub const MULTIPLANE_ADDR_RESTRICT: c_uint = 0x1a0;

pub const ECC_CORRECTION: c_uint = 0x1b0;

pub const READ_MODE: c_uint = 0x1c0;

pub const WRITE_MODE: c_uint = 0x1d0;

pub const COPYBACK_MODE: c_uint = 0x1e0;

pub const RDWR_EN_LO_CNT: c_uint = 0x1f0;

pub const RDWR_EN_HI_CNT: c_uint = 0x200;

pub const MAX_RD_DELAY: c_uint = 0x210;

pub const CS_SETUP_CNT: c_uint = 0x220;

pub const SPARE_AREA_SKIP_BYTES: c_uint = 0x230;

pub const SPARE_AREA_MARKER: c_uint = 0x240;

pub const DEVICES_CONNECTED: c_uint = 0x250;

pub const DIE_MASK: c_uint = 0x260;

pub const FIRST_BLOCK_OF_NEXT_PLANE: c_uint = 0x270;

pub const WRITE_PROTECT: c_uint = 0x280;

pub const RE_2_RE: c_uint = 0x290;

pub const MANUFACTURER_ID: c_uint = 0x300;

pub const DEVICE_ID: c_uint = 0x310;

pub const DEVICE_PARAM_0: c_uint = 0x320;

pub const DEVICE_PARAM_1: c_uint = 0x330;

pub const DEVICE_PARAM_2: c_uint = 0x340;

pub const LOGICAL_PAGE_DATA_SIZE: c_uint = 0x350;

pub const LOGICAL_PAGE_SPARE_SIZE: c_uint = 0x360;

pub const REVISION: c_uint = 0x370;

pub const ONFI_DEVICE_FEATURES: c_uint = 0x380;

pub const ONFI_OPTIONAL_COMMANDS: c_uint = 0x390;

pub const ONFI_TIMING_MODE: c_uint = 0x3a0;

pub const ONFI_PGM_CACHE_TIMING_MODE: c_uint = 0x3b0;

pub const ONFI_DEVICE_NO_OF_LUNS: c_uint = 0x3c0;

pub const ONFI_DEVICE_NO_OF_BLOCKS_PER_LUN_L: c_uint = 0x3d0;

pub const ONFI_DEVICE_NO_OF_BLOCKS_PER_LUN_U: c_uint = 0x3e0;

pub const FEATURES: c_uint = 0x3f0;

pub const TRANSFER_MODE: c_uint = 0x400;

// bit[1:0] is used differently depending on IP version

pub const ECC_THRESHOLD: c_uint = 0x600;

pub const ECC_ERROR_BLOCK_ADDRESS: c_uint = 0x610;

pub const ECC_ERROR_PAGE_ADDRESS: c_uint = 0x620;

pub const ECC_ERROR_ADDRESS: c_uint = 0x630;

pub const ERR_CORRECTION_INFO: c_uint = 0x640;

pub const CFG_DATA_BLOCK_SIZE: c_uint = 0x6b0;
pub const CFG_LAST_DATA_BLOCK_SIZE: c_uint = 0x6c0;
pub const CFG_NUM_DATA_BLOCKS: c_uint = 0x6d0;
pub const CFG_META_DATA_SIZE: c_uint = 0x6e0;
pub const DMA_ENABLE: c_uint = 0x700;

pub const IGNORE_ECC_DONE: c_uint = 0x710;

pub const DMA_INTR: c_uint = 0x720;
pub const DMA_INTR_EN: c_uint = 0x730;

pub const TARGET_ERR_ADDR_LO: c_uint = 0x740;

pub const TARGET_ERR_ADDR_HI: c_uint = 0x750;

pub const CHNL_ACTIVE: c_uint = 0x760;

//
// struct denali_chip_sel - per-CS data of Denali NAND
//
// @bank:                  bank id of the controller this CS is connected to
// @hwhr2_and_we_2_re:     value of timing register HWHR2_AND_WE_2_RE
// @tcwaw_and_addr_2_data: value of timing register TCWAW_AND_ADDR_2_DATA
// @re_2_we:               value of timing register RE_2_WE
// @acc_clks:              value of timing register ACC_CLKS
// @rdwr_en_lo_cnt:        value of timing register RDWR_EN_LO_CNT
// @rdwr_en_hi_cnt:        value of timing register RDWR_EN_HI_CNT
// @cs_setup_cnt:          value of timing register CS_SETUP_CNT
// @re_2_re:               value of timing register RE_2_RE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct denali_chip_sel {
    pub bank: c_int,
    pub hwhr2_and_we_2_re: u32,
    pub tcwaw_and_addr_2_data: u32,
    pub re_2_we: u32,
    pub acc_clks: u32,
    pub rdwr_en_lo_cnt: u32,
    pub rdwr_en_hi_cnt: u32,
    pub cs_setup_cnt: u32,
    pub re_2_re: u32,
}

//
// struct denali_chip - per-chip data of Denali NAND
//
// @chip:  base NAND chip structure
// @node:  node to be used to associate this chip with the controller
// @nsels: the number of CS lines of this chip
// @sels:  the array of per-cs data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct denali_chip {
    pub chip: nand_chip,
    pub node: list_head,
    pub nsels: c_uint,
    pub __counted_by(nsels): denali_chip_sel sels[],
}

//
// struct denali_controller - Denali NAND controller data
//
// @controller:     base NAND controller structure
// @dev:            device
// @chips:          the list of chips attached to this controller
// @clk_rate:       frequency of core clock
// @clk_x_rate:     frequency of bus interface clock
// @reg:            base of Register Interface
// @host:           base of Host Data/Command interface
// @complete:       completion used to wait for interrupts
// @irq:            interrupt number
// @irq_mask:       interrupt bits the controller is waiting for
// @irq_status:     interrupt bits of events that have happened
// @irq_lock:       lock to protect @irq_mask and @irq_status
// @dma_avail:      set if DMA engine is available
// @devs_per_cs:    number of devices connected in parallel
// @oob_skip_bytes: number of bytes in OOB skipped by the ECC engine
// @active_bank:    active bank id
// @nbanks:         the number of banks supported by this controller
// @revision:       IP revision
// @caps:           controller capabilities that cannot be detected run-time
// @ecc_caps:       ECC engine capabilities
// @host_read:      callback for read access of Host Data/Command Interface
// @host_write:     callback for write access of Host Data/Command Interface
// @setup_dma:      callback for setup of the Data DMA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct denali_controller {
    pub controller: nand_controller,
    pub dev: *mut device,
    pub chips: list_head,
    pub clk_rate: c_ulong,
    pub clk_x_rate: c_ulong,
    pub reg: *mut void __iomem,
    pub host: *mut void __iomem,
    pub complete: completion,
    pub irq: c_int,
    pub irq_mask: u32,
    pub irq_status: u32,
    pub irq_lock: spinlock_t,
    pub dma_avail: bool,
    pub devs_per_cs: c_int,
    pub oob_skip_bytes: c_int,
    pub active_bank: c_int,
    pub nbanks: c_int,
    pub revision: c_uint,
    pub caps: c_uint,
    pub ecc_caps: *const nand_ecc_caps,
    pub addr): *mut *mut *mut u32 (host_read)(struct denali_controller denali, u32,
    pub data): u32,
    pub write): int page, bool,
}

extern "C" {
    pub fn denali_calc_ecc_bytes(step_size: c_int, strength: c_int) -> c_int;
}
extern "C" {
    pub fn denali_init(denali: *mut denali_controller) -> c_int;
}
extern "C" {
    pub fn denali_remove(denali: *mut denali_controller);
}
