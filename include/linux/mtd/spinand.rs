//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/spinand.h
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
// Copyright (c) 2016-2017 Micron Technology, Inc.
//
// Authors:
// Peter Pan <peterpandong@micron.com>
//

//
// Standard SPI NAND flash operations
//

//
// Octal DDR SPI NAND flash operations
//

// feature register
pub const REG_BLOCK_LOCK: c_uint = 0xa0;
pub const BL_ALL_UNLOCKED: c_uint = 0x00;
// configuration register
pub const REG_CFG: c_uint = 0xb0;

// status register
pub const REG_STATUS: c_uint = 0xc0;

pub const SPINAND_MAX_ID_LEN: c_int = 6;
//
// For erase, write and read operation, we got the following timings :
// tBERS (erase) 1ms to 4ms
// tPROG 300us to 400us
// tREAD 25us to 100us
// In order to minimize latency, the min value is divided by 4 for the
// initial delay, and dividing by 20 for the poll delay.
// For reset, 5us/10us/500us if the device is respectively
// reading/programming/erasing when the RESET occurs. Since we always
// issue a RESET when the device is IDLE, 5us is selected for both initial
// and poll delay.
//
pub const SPINAND_READ_INITIAL_DELAY_US: c_int = 6;
pub const SPINAND_READ_POLL_DELAY_US: c_int = 5;
pub const SPINAND_RESET_INITIAL_DELAY_US: c_int = 5;
pub const SPINAND_RESET_POLL_DELAY_US: c_int = 5;
pub const SPINAND_WRITE_INITIAL_DELAY_US: c_int = 75;
pub const SPINAND_WRITE_POLL_DELAY_US: c_int = 15;
pub const SPINAND_ERASE_INITIAL_DELAY_US: c_int = 250;
pub const SPINAND_ERASE_POLL_DELAY_US: c_int = 50;
pub const SPINAND_WAITRDY_TIMEOUT_MS: c_int = 400;
//
// struct spinand_id - SPI NAND id structure
// @data: buffer containing the id bytes. Currently 6 bytes large, but can
// be extended if required
// @len: ID length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_id {
    pub data: [u8; SPINAND_MAX_ID_LEN],
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spinand_readid_method {
    SPINAND_READID_METHOD_OPCODE,
    SPINAND_READID_METHOD_OPCODE_ADDR,
    SPINAND_READID_METHOD_OPCODE_DUMMY,
}

//
// struct spinand_devid - SPI NAND device id structure
// @id: device id of current chip
// @len: number of bytes in device id
// @method: method to read chip id
// There are 3 possible variants:
// SPINAND_READID_METHOD_OPCODE: chip id is returned immediately
// after read_id opcode.
// SPINAND_READID_METHOD_OPCODE_ADDR: chip id is returned after
// read_id opcode + 1-byte address.
// SPINAND_READID_METHOD_OPCODE_DUMMY: chip id is returned after
// read_id opcode + 1 dummy byte.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_devid {
    pub id: *const u8,
    pub len: u8,
    pub method: spinand_readid_method,
}

//
// struct manufacurer_ops - SPI NAND manufacturer specific operations
// @init: initialize a SPI NAND device
// @cleanup: cleanup a SPI NAND device
//
// Each SPI NAND manufacturer driver should implement this interface so that
// NAND chips coming from this vendor can be initialized properly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_manufacturer_ops {
    pub spinand): *mut *mut int (init)(struct spinand_device,
    pub spinand): *mut *mut void (cleanup)(struct spinand_device,
}

//
// struct spinand_manufacturer - SPI NAND manufacturer instance
// @id: manufacturer ID
// @name: manufacturer name
// @devid_len: number of bytes in device ID
// @chips: supported SPI NANDs under current manufacturer
// @nchips: number of SPI NANDs available in chips array
// @ops: manufacturer operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_manufacturer {
    pub id: u8,
    pub name: *mut c_char,
    pub chips: *const spinand_info,
    pub nchips: usize,
    pub ops: *const spinand_manufacturer_ops,
}

// SPI NAND manufacturers
//
// struct spinand_op_variants - SPI NAND operation variants
// @ops: the list of variants for a given operation
// @nops: the number of variants
//
// Some operations like read-from-cache/write-to-cache have several variants
// depending on the number of IO lines you use to transfer data or address
// cycles. This structure is a way to describe the different variants supported
// by a chip and let the core pick the best one based on the SPI mem controller
// capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_op_variants {
    pub ops: *const spi_mem_op,
    pub nops: c_uint,
}

//
// spinand_ecc_info - description of the on-die ECC implemented by a SPI NAND
// chip
// @get_status: get the ECC status. Should return a positive number encoding
// the number of corrected bitflips if correction was possible or
// -EBADMSG if there are uncorrectable errors. I can also return
// other negative error codes if the error is not caused by
// uncorrectable bitflips
// @ooblayout: the OOB layout used by the on-die ECC implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_ecc_info {
    pub status): *mut *mut *mut int (get_status)(struct spinand_device spinand, u8,
    pub ooblayout: *const mtd_ooblayout_ops,
}

// SPI NAND flags

//
// struct spinand_ondie_ecc_conf - private SPI-NAND on-die ECC engine structure
// @status: status of the last wait operation that will be used in case
// ->get_status() is not populated by the spinand device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_ondie_ecc_conf {
    pub status: u8,
}

//
// struct spinand_otp_layout - structure to describe the SPI NAND OTP area
// @npages: number of pages in the OTP
// @start_page: start page of the user/factory OTP area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_otp_layout {
    pub npages: c_uint,
    pub start_page: c_uint,
}

//
// struct spinand_fact_otp_ops - SPI NAND OTP methods for factory area
// @info: get the OTP area information
// @read: read from the SPI NAND OTP area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_fact_otp_ops {
    pub retlen): *mut *mut otp_info buf, size_t,
    pub buf): *mut *mut size_t retlen, u8,
}

//
// struct spinand_user_otp_ops - SPI NAND OTP methods for user area
// @info: get the OTP area information
// @lock: lock an OTP region
// @erase: erase an OTP region
// @read: read from the SPI NAND OTP area
// @write: write to the SPI NAND OTP area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_user_otp_ops {
    pub retlen): *mut *mut otp_info buf, size_t,
    pub len): *mut *mut *mut int (lock)(struct spinand_device spinand, loff_t from, size_t,
    pub len): *mut *mut *mut int (erase)(struct spinand_device spinand, loff_t from, size_t,
    pub buf): *mut *mut size_t retlen, u8,
    pub buf): *const *const size_t retlen, u8,
}

//
// struct spinand_fact_otp - SPI NAND OTP grouping structure for factory area
// @layout: OTP region layout
// @ops: OTP access ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_fact_otp {
    pub layout: spinand_otp_layout,
    pub ops: *const spinand_fact_otp_ops,
}

//
// struct spinand_user_otp - SPI NAND OTP grouping structure for user area
// @layout: OTP region layout
// @ops: OTP access ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_user_otp {
    pub layout: spinand_otp_layout,
    pub ops: *const spinand_user_otp_ops,
}

//
// enum spinand_bus_interface - SPI NAND bus interface types
// @SSDR: Bus configuration supporting all 1S-XX-XX operations, including dual and quad
// @ODTR: Bus configuration supporting only 8D-8D-8D operations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spinand_bus_interface {
    SSDR,
    ODTR,
}

//
// struct spinand_info - Structure used to describe SPI NAND chips
// @model: model name
// @devid: device ID
// @flags: OR-ing of the SPINAND_XXX flags
// @memorg: memory organization
// @eccreq: ECC requirements
// @eccinfo: on-die ECC info
// @op_variants: operations variants
// @op_variants.read_cache: variants of the read-cache operation
// @op_variants.write_cache: variants of the write-cache operation
// @op_variants.update_cache: variants of the update-cache operation
// @op_variants.cont_read_cache: variants of the continuous read-cache operation
// @vendor_ops: vendor specific operations
// @select_target: function used to select a target/die. Required only for
// multi-die chips
// @configure_chip: Align the chip configuration with the core settings
// @set_cont_read: enable/disable continuous cached reads
// @fact_otp: SPI NAND factory OTP info.
// @user_otp: SPI NAND user OTP info.
// @read_retries: the number of read retry modes supported
// @set_read_retry: enable/disable read retry for data recovery
// @set_randomizer: enable/disable randomizer support
//
// Each SPI NAND manufacturer driver should have a spinand_info table
// describing all the chips supported by the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_info {
    pub model: *const c_char,
    pub devid: spinand_devid,
    pub flags: u32,
    pub memorg: nand_memory_organization,
    pub eccreq: nand_ecc_props,
    pub eccinfo: spinand_ecc_info,
    pub read_cache: *const spinand_op_variants,
    pub write_cache: *const spinand_op_variants,
    pub update_cache: *const spinand_op_variants,
    pub cont_read_cache: *const spinand_op_variants,
    pub op_variants: },
    pub vendor_ops: *const spinand_op_variants,
    pub target): c_uint,
    pub iface): spinand_bus_interface,
    pub enable): bool,
    pub fact_otp: spinand_fact_otp,
    pub user_otp: spinand_user_otp,
    pub read_retries: c_uint,
    pub read_retry): c_uint,
    pub enable): bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_dirmap {
    pub wdesc: *mut spi_mem_dirmap_desc,
    pub rdesc: *mut spi_mem_dirmap_desc,
}

//
// struct spinand_mem_ops - SPI NAND memory operations
// @reset: reset op template
// @readid: read ID op template
// @wr_en: write enable op template
// @wr_dis: write disable op template
// @set_feature: set feature op template
// @get_feature: get feature op template
// @blk_erase: blk erase op template
// @page_read: page read op template
// @prog_exec: prog exec op template
// @read_cache: read cache op template
// @write_cache: write cache op template
// @update_cache: update cache op template
// @cont_read_cache: continuous read cache op template (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_mem_ops {
    pub reset: spi_mem_op,
    pub readid: spi_mem_op,
    pub wr_en: spi_mem_op,
    pub wr_dis: spi_mem_op,
    pub set_feature: spi_mem_op,
    pub get_feature: spi_mem_op,
    pub blk_erase: spi_mem_op,
    pub page_read: spi_mem_op,
    pub prog_exec: spi_mem_op,
    pub read_cache: *const spi_mem_op,
    pub write_cache: *const spi_mem_op,
    pub update_cache: *const spi_mem_op,
    pub cont_read_cache: *const spi_mem_op,
}

//
// struct spinand_device - SPI NAND device instance
// @base: NAND device instance
// @spimem: pointer to the SPI mem object
// @lock: lock used to serialize accesses to the NAND
// @id: NAND ID as returned by READ_ID
// @flags: NAND flags
// @ssdr_op_templates: Templates for all single SDR SPI mem operations
// @odtr_op_templates: Templates for all octal DTR SPI mem operations
// @op_templates: Templates for all SPI mem operations
// @bus_iface: Current bus interface
// @select_target: select a specific target/die. Usually called before sending
// a command addressing a page or an eraseblock embedded in
// this die. Only required if your chip exposes several dies
// @cur_target: currently selected target/die
// @eccinfo: on-die ECC information
// @cfg_cache: config register cache. One entry per die
// @databuf: bounce buffer for data
// @oobbuf: bounce buffer for OOB data
// @scratchbuf: buffer used for everything but page accesses. This is needed
// because the spi-mem interface explicitly requests that buffers
// passed in spi_mem_op be DMA-able, so we can't based the bufs on
// the stack
// @manufacturer: SPI NAND manufacturer information
// @configure_chip: Align the chip configuration with the core settings
// @cont_read_possible: Field filled by the core once the whole system
// configuration is known to tell whether continuous reads are
// suitable to use or not in general with this chip/configuration.
// A per-transfer check must of course be done to ensure it is
// actually relevant to enable this feature.
// @set_cont_read: Enable/disable the continuous read feature
// @priv: manufacturer private data
// @fact_otp: SPI NAND factory OTP info.
// @user_otp: SPI NAND user OTP info.
// @read_retries: the number of read retry modes supported
// @set_read_retry: Enable/disable the read retry feature
// @set_randomizer: Enable/disable the randomizer feature
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinand_device {
    pub base: nand_device,
    pub spimem: *mut spi_mem,
    pub lock: mutex,
    pub id: spinand_id,
    pub flags: u32,
    pub ssdr_op_templates: spinand_mem_ops,
    pub odtr_op_templates: spinand_mem_ops,
    pub op_templates: *mut spinand_mem_ops,
    pub bus_iface: spinand_bus_interface,
    pub dirmaps: *mut spinand_dirmap,
    pub target): c_uint,
    pub cur_target: c_uint,
    pub eccinfo: spinand_ecc_info,
    pub cfg_cache: *mut u8,
    pub databuf: *mut u8,
    pub oobbuf: *mut u8,
    pub scratchbuf: *mut u8,
    pub manufacturer: *const spinand_manufacturer,
    pub priv: *mut c_void,
    pub iface): spinand_bus_interface,
    pub cont_read_possible: bool,
    pub enable): bool,
    pub enable): bool,
    pub fact_otp: *const spinand_fact_otp,
    pub user_otp: *const spinand_user_otp,
    pub read_retries: c_uint,
    pub retry_mode): c_uint,
}

extern "C" {
    pub fn spinand_fill_wr_en_op(spinand: *mut spinand_device) -> spi_mem_op;
}
extern "C" {
    pub fn spinand_fill_set_feature_op(spinand: *mut spinand_device, reg: u64, valptr: *const c_void) -> spi_mem_op;
}
extern "C" {
    pub fn spinand_fill_get_feature_op(spinand: *mut spinand_device, reg: u64, valptr: *mut c_void) -> spi_mem_op;
}
extern "C" {
    pub fn spinand_fill_prog_exec_op(spinand: *mut spinand_device, addr: u64) -> spi_mem_op;
}

//
// mtd_to_spinand() - Get the SPI NAND device attached to an MTD instance
// @mtd: MTD instance
//
// Return: the SPI NAND device attached to @mtd.
//
extern "C" {
    pub fn container_of(_arg: mtd_to_nanddev(mtd), spinand_device: struct, _arg: base) -> return;
}
//
// spinand_to_mtd() - Get the MTD device embedded in a SPI NAND device
// @spinand: SPI NAND device
//
// Return: the MTD device embedded in @spinand.
//
extern "C" {
    pub fn nanddev_to_mtd(_arg: &spinand->base) -> return;
}
//
// nand_to_spinand() - Get the SPI NAND device embedding an NAND object
// @nand: NAND object
//
// Return: the SPI NAND device embedding @nand.
//
extern "C" {
    pub fn container_of(_arg: nand, spinand_device: struct, _arg: base) -> return;
}
//
// spinand_to_nand() - Get the NAND device embedded in a SPI NAND object
// @spinand: SPI NAND device
//
// Return: the NAND device embedded in @spinand.
//
// spinand_set_of_node - Attach a DT node to a SPI NAND device
// @spinand: SPI NAND device
// @np: DT node
//
// Attach a DT node to a SPI NAND device.
//
extern "C" {
    pub fn spinand_op_is_odtr(op: *const spi_mem_op) -> bool;
}
extern "C" {
    pub fn spinand_upd_cfg(spinand: *mut spinand_device, mask: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn spinand_read_reg_op(spinand: *mut spinand_device, reg: u8, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn spinand_write_reg_op(spinand: *mut spinand_device, reg: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn spinand_write_enable_op(spinand: *mut spinand_device) -> c_int;
}
extern "C" {
    pub fn spinand_select_target(spinand: *mut spinand_device, target: c_uint) -> c_int;
}
extern "C" {
    pub fn spinand_otp_page_size(spinand: *mut spinand_device) -> usize;
}
extern "C" {
    pub fn spinand_fact_otp_size(spinand: *mut spinand_device) -> usize;
}
extern "C" {
    pub fn spinand_user_otp_size(spinand: *mut spinand_device) -> usize;
}
extern "C" {
    pub fn spinand_set_mtd_otp_ops(spinand: *mut spinand_device) -> c_int;
}
