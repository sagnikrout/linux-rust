//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/spi-nor/core.h
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
// Copyright (C) 2005, Intec Automation Inc.
// Copyright (C) 2014, Freescale Semiconductor, Inc.
//

pub const SPI_NOR_MAX_ID_LEN: c_int = 6;
//
// 256 bytes is a sane default for most older flashes. Newer flashes will
// have the page size defined within their SFDP tables.
//
pub const SPI_NOR_DEFAULT_PAGE_SIZE: c_int = 256;
pub const SPI_NOR_DEFAULT_N_BANKS: c_int = 1;

// Standard SPI NOR flash operations.

// Keep these in sync with the list in debugfs.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_nor_option_flags {
    SNOR_F_HAS_SR_TB	= BIT(0),
    SNOR_F_NO_OP_CHIP_ERASE	= BIT(1),
    SNOR_F_BROKEN_RESET	= BIT(2),
    SNOR_F_4B_OPCODES	= BIT(3),
    SNOR_F_HAS_4BAIT	= BIT(4),
    SNOR_F_HAS_LOCK		= BIT(5),
    SNOR_F_HAS_16BIT_SR	= BIT(6),
    SNOR_F_NO_READ_CR	= BIT(7),
    SNOR_F_HAS_SR_TB_BIT6	= BIT(8),
    SNOR_F_HAS_4BIT_BP      = BIT(9),
    SNOR_F_HAS_SR_BP3_BIT6  = BIT(10),
    SNOR_F_IO_MODE_EN_VOLATILE = BIT(11),
    SNOR_F_SOFT_RESET	= BIT(12),
    SNOR_F_SWP_IS_VOLATILE	= BIT(13),
    SNOR_F_RWW		= BIT(14),
    SNOR_F_ECC		= BIT(15),
    SNOR_F_NO_WP		= BIT(16),
    SNOR_F_SWAP16		= BIT(17),
    SNOR_F_HAS_SR2_CMP_BIT6	= BIT(18),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_read_command {
    pub num_mode_clocks: u8,
    pub num_wait_states: u8,
    pub opcode: u8,
    pub proto: spi_nor_protocol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_pp_command {
    pub opcode: u8,
    pub proto: spi_nor_protocol,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_nor_read_command_index {
    SNOR_CMD_READ,
    SNOR_CMD_READ_FAST,
    SNOR_CMD_READ_1_1_1_DTR,

// Dual SPI
    SNOR_CMD_READ_1_1_2,
    SNOR_CMD_READ_1_2_2,
    SNOR_CMD_READ_2_2_2,
    SNOR_CMD_READ_1_2_2_DTR,

// Quad SPI
    SNOR_CMD_READ_1_1_4,
    SNOR_CMD_READ_1_4_4,
    SNOR_CMD_READ_4_4_4,
    SNOR_CMD_READ_1_4_4_DTR,

// Octal SPI
    SNOR_CMD_READ_1_1_8,
    SNOR_CMD_READ_1_8_8,
    SNOR_CMD_READ_8_8_8,
    SNOR_CMD_READ_1_8_8_DTR,
    SNOR_CMD_READ_8_8_8_DTR,

    SNOR_CMD_READ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_nor_pp_command_index {
    SNOR_CMD_PP,

// Quad SPI
    SNOR_CMD_PP_1_1_4,
    SNOR_CMD_PP_1_4_4,
    SNOR_CMD_PP_4_4_4,

// Octal SPI
    SNOR_CMD_PP_1_1_8,
    SNOR_CMD_PP_1_8_8,
    SNOR_CMD_PP_8_8_8,
    SNOR_CMD_PP_8_8_8_DTR,

    SNOR_CMD_PP_MAX
}

//
// struct spi_nor_erase_type - Structure to describe a SPI NOR erase type
// @size:		the size of the sector/block erased by the erase type.
// JEDEC JESD216B imposes erase sizes to be a power of 2.
// @size_shift:		@size is a power of 2, the shift is stored in
// @size_shift.
// @size_mask:		the size mask based on @size_shift.
// @opcode:		the SPI command op code to erase the sector/block.
// @idx:		Erase Type index as sorted in the Basic Flash Parameter
// Table. It will be used to synchronize the supported
// Erase Types with the ones identified in the SFDP
// optional tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_erase_type {
    pub size: u32,
    pub size_shift: u32,
    pub size_mask: u32,
    pub opcode: u8,
    pub idx: u8,
}

//
// struct spi_nor_erase_command - Used for non-uniform erases
// The structure is used to describe a list of erase commands to be executed
// once we validate that the erase can be performed. The elements in the list
// are run-length encoded.
// @list:		for inclusion into the list of erase commands.
// @count:		how many times the same erase command should be
// consecutively used.
// @size:		the size of the sector/block erased by the command.
// @opcode:		the SPI command op code to erase the sector/block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_erase_command {
    pub list: list_head,
    pub count: u32,
    pub size: u32,
    pub opcode: u8,
}

//
// struct spi_nor_erase_region - Structure to describe a SPI NOR erase region
// @offset:		the offset in the data array of erase region start.
// @size:		the size of the region in bytes.
// @erase_mask:		bitmask to indicate all the supported erase commands
// inside this region. The erase types are sorted in
// ascending order with the smallest Erase Type size being
// at BIT(0).
// @overlaid:		determine if this region is overlaid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_erase_region {
    pub offset: u64,
    pub size: u64,
    pub erase_mask: u8,
    pub overlaid: bool,
}

pub const SNOR_ERASE_TYPE_MAX: c_int = 4;
//
// struct spi_nor_erase_map - Structure to describe the SPI NOR erase map
// @regions:		array of erase regions. The regions are consecutive in
// address space. Walking through the regions is done
// incrementally.
// @uniform_region:	a pre-allocated erase region for SPI NOR with a uniform
// sector size (legacy implementation).
// @erase_type:		an array of erase types shared by all the regions.
// The erase types are sorted in ascending order, with the
// smallest Erase Type size being the first member in the
// erase_type array.
// @n_regions:		number of erase regions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_erase_map {
    pub regions: *mut spi_nor_erase_region,
    pub uniform_region: spi_nor_erase_region,
    pub erase_type: [spi_nor_erase_type; SNOR_ERASE_TYPE_MAX],
    pub n_regions: c_uint,
}

//
// struct spi_nor_locking_ops - SPI NOR locking methods
// @lock:	lock a region of the SPI NOR, never locks more than what is
// requested, ie. may lock less.
// @unlock:	unlock a region of the SPI NOR, may unlock more than what is
// requested.
// @is_locked:	check if a region of the SPI NOR is completely locked, returns
// false otherwise. This feedback may be misleading because users
// may get an "unlocked" status even though a subpart of the region
// is effectively locked.
//
// If in doubt during development, check-out the debugfs output which tries to
// be more user friendly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_locking_ops {
    pub len): *mut *mut *mut int (lock)(struct spi_nor nor, loff_t ofs, u64,
    pub len): *mut *mut *mut int (unlock)(struct spi_nor nor, loff_t ofs, u64,
    pub len): *mut *mut *mut int (is_locked)(struct spi_nor nor, loff_t ofs, u64,
}

//
// struct spi_nor_otp_organization - Structure to describe the SPI NOR OTP regions
// @len:	size of one OTP region in bytes.
// @base:	start address of the OTP area.
// @offset:	offset between consecutive OTP regions if there are more
// than one.
// @n_regions:	number of individual OTP regions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_otp_organization {
    pub len: usize,
    pub base: loff_t,
    pub offset: loff_t,
    pub n_regions: c_uint,
}

//
// struct spi_nor_otp_ops - SPI NOR OTP methods
// @read:	read from the SPI NOR OTP area.
// @write:	write to the SPI NOR OTP area.
// @lock:	lock an OTP region.
// @erase:	erase an OTP region.
// @is_locked:	check if an OTP region of the SPI NOR is locked.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_otp_ops {
    pub buf): *mut *mut *mut int (read)(struct spi_nor nor, loff_t addr, size_t len, u8,
    pub buf): *const u8,
    pub region): *mut *mut *mut int (lock)(struct spi_nor nor, unsigned int,
    pub addr): *mut *mut *mut int (erase)(struct spi_nor nor, loff_t,
    pub region): *mut *mut *mut int (is_locked)(struct spi_nor nor, unsigned int,
}

//
// struct spi_nor_otp - SPI NOR OTP grouping structure
// @org:	OTP region organization
// @ops:	OTP access ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_otp {
    pub org: *const spi_nor_otp_organization,
    pub ops: *const spi_nor_otp_ops,
}

//
// struct spi_nor_flash_parameter - SPI NOR flash parameters and settings.
// Includes legacy flash parameters and settings that can be overwritten
// by the spi_nor_fixups hooks, or dynamically when parsing the JESD216
// Serial Flash Discoverable Parameters (SFDP) tables.
//
// @bank_size:		the flash memory bank density in bytes.
// @size:		the total flash memory density in bytes.
// @writesize		Minimal writable flash unit size. Defaults to 1. Set to
// ECC unit size for ECC-ed flashes.
// @page_size:		the page size of the SPI NOR flash memory.
// @addr_nbytes:	number of address bytes to send.
// @addr_mode_nbytes:	number of address bytes of current address mode. Useful
// when the flash operates with 4B opcodes but needs the
// internal address mode for opcodes that don't have a 4B
// opcode correspondent.
// @rdsr_dummy:		dummy cycles needed for Read Status Register command
// in octal DTR mode.
// @rdsr_addr_nbytes:	dummy address bytes needed for Read Status Register
// command in octal DTR mode.
// @n_banks:		number of banks.
// @n_dice:		number of dice in the flash memory.
// @die_erase_opcode:	die erase opcode. Defaults to SPINOR_OP_CHIP_ERASE.
// @vreg_offset:	volatile register offset for each die.
// @hwcaps:		describes the read and page program hardware
// capabilities.
// @reads:		read capabilities ordered by priority: the higher index
// in the array, the higher priority.
// @page_programs:	page program capabilities ordered by priority: the
// higher index in the array, the higher priority.
// @erase_map:		the erase map parsed from the SFDP Sector Map Parameter
// Table.
// @otp:		SPI NOR OTP info.
// @set_octal_dtr:	enables or disables SPI NOR octal DTR mode.
// @quad_enable:	enables SPI NOR quad mode.
// @set_4byte_addr_mode: puts the SPI NOR in 4 byte addressing mode.
// @ready:		(optional) flashes might use a different mechanism
// than reading the status register to indicate they
// are ready for a new command
// @locking_ops:	SPI NOR locking methods.
// @priv:		flash's private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_flash_parameter {
    pub bank_size: u64,
    pub size: u64,
    pub writesize: u32,
    pub page_size: u32,
    pub addr_nbytes: u8,
    pub addr_mode_nbytes: u8,
    pub rdsr_dummy: u8,
    pub rdsr_addr_nbytes: u8,
    pub n_banks: u8,
    pub n_dice: u8,
    pub die_erase_opcode: u8,
    pub vreg_offset: *mut u32,
    pub hwcaps: spi_nor_hwcaps,
    pub reads: [spi_nor_read_command; SNOR_CMD_READ_MAX],
    pub page_programs: [spi_nor_pp_command; SNOR_CMD_PP_MAX],
    pub erase_map: spi_nor_erase_map,
    pub otp: spi_nor_otp,
    pub enable): *mut *mut *mut int (set_octal_dtr)(struct spi_nor nor, bool,
    pub nor): *mut *mut int (quad_enable)(struct spi_nor,
    pub enable): *mut *mut *mut int (set_4byte_addr_mode)(struct spi_nor nor, bool,
    pub nor): *mut *mut int (ready)(struct spi_nor,
    pub locking_ops: *const spi_nor_locking_ops,
    pub priv: *mut c_void,
}

//
// struct spi_nor_fixups - SPI NOR fixup hooks
// @default_init: called after default flash parameters init. Used to tweak
// flash parameters when information provided by the flash_info
// table is incomplete or wrong.
// @post_bfpt: called after the BFPT table has been parsed
// @smpt_read_dummy: called during SMPT table is being parsed. Used to fix the
// number of dummy cycles in read register ops.
// @smpt_map_id: called after map ID in SMPT table has been determined for the
// case the map ID is wrong and needs to be fixed.
// @post_sfdp: called after SFDP has been parsed (is not called for SPI NORs
// that do not support RDSFDP). Typically used to tweak various
// parameters that could not be extracted by other means (i.e.
// when information provided by the SFDP/flash_info tables are
// incomplete or wrong).
// @late_init: used to initialize flash parameters that are not declared in the
// JESD216 SFDP standard, or where SFDP tables not defined at all.
// Will replace the default_init() hook.
//
// Those hooks can be used to tweak the SPI NOR configuration when the SFDP
// table is broken or not available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_fixups {
    pub nor): *mut *mut void (default_init)(struct spi_nor,
    pub bfpt): *const sfdp_bfpt,
    pub read_dummy): *const *const *const void (smpt_read_dummy)(struct spi_nor nor, u8,
    pub map_id): *const *const *const void (smpt_map_id)(struct spi_nor nor, u8,
    pub nor): *mut *mut int (post_sfdp)(struct spi_nor,
    pub nor): *mut *mut int (late_init)(struct spi_nor,
}

//
// struct spi_nor_id - SPI NOR flash ID.
//
// @bytes: the bytes returned by the flash when issuing command 9F. Typically,
// the first byte is the manufacturer ID code (see JEP106) and the next
// two bytes are a flash part specific ID.
// @len:   the number of bytes of ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_id {
    pub bytes: *const u8,
    pub len: u8,
}

//
// struct flash_info - SPI NOR flash_info entry.
// @id:   pointer to struct spi_nor_id or NULL, which means "no ID" (mostly
// older chips).
// @name: (obsolete) the name of the flash. Do not set it for new additions.
// @size:           the size of the flash in bytes. The flash size is one
// property parsed by the SFDP. We use it as an indicator
// whether we need SFDP parsing for a particular flash.
// I.e. non-legacy flash entries in flash_info will have
// a size of zero iff SFDP should be used.
// @sector_size:    (optional) the size listed here is what works with
// SPINOR_OP_SE, which isn't necessarily called a "sector" by
// the vendor. Defaults to 64k.
// @n_banks:        (optional) the number of banks. Defaults to 1.
// @page_size:      (optional) the flash's page size. Defaults to 256.
// @addr_nbytes:    number of address bytes to send.
//
// @flags:          flags that indicate support that is not defined by the
// JESD216 standard in its SFDP tables. Flag meanings:
// SPI_NOR_HAS_LOCK:        flash supports lock/unlock via SR
// SPI_NOR_HAS_TB:          flash SR has Top/Bottom (TB) protect bit. Must be
// used with SPI_NOR_HAS_LOCK.
// SPI_NOR_TB_SR_BIT6:      Top/Bottom (TB) is bit 6 of status register.
// Must be used with SPI_NOR_HAS_TB.
// SPI_NOR_4BIT_BP:         flash SR has 4 bit fields (BP0-3) for block
// protection.
// SPI_NOR_BP3_SR_BIT6:     BP3 is bit 6 of status register. Must be used with
// SPI_NOR_4BIT_BP.
// SPI_NOR_SWP_IS_VOLATILE: flash has volatile software write protection bits.
// Usually these will power-up in a write-protected
// state.
// SPI_NOR_NO_ERASE:        no erase command needed.
// SPI_NOR_QUAD_PP:         flash supports Quad Input Page Program.
// SPI_NOR_RWW:             flash supports reads while write.
// SPI_NOR_HAS_CMP:         flash SR2 has complement (CMP) protect bit. Must
// be used with SPI_NOR_HAS_LOCK.
//
// @no_sfdp_flags:  flags that indicate support that can be discovered via SFDP.
// Used when SFDP tables are not defined in the flash. These
// flags are used together with the SPI_NOR_SKIP_SFDP flag.
// SPI_NOR_SKIP_SFDP:       skip parsing of SFDP tables.
// SECT_4K:                 SPINOR_OP_BE_4K works uniformly.
// SPI_NOR_DUAL_READ:       flash supports Dual Read.
// SPI_NOR_QUAD_READ:       flash supports Quad Read.
// SPI_NOR_OCTAL_READ:      flash supports Octal Read.
// SPI_NOR_OCTAL_DTR_READ:  flash supports octal DTR Read.
// SPI_NOR_OCTAL_DTR_PP:    flash supports Octal DTR Page Program.
//
// @fixup_flags:    flags that indicate support that can be discovered via SFDP
// ideally, but can not be discovered for this particular flash
// because the SFDP table that indicates this support is not
// defined by the flash. In case the table for this support is
// defined but has wrong values, one should instead use a
// post_sfdp() hook to set the SNOR_F equivalent flag.
//
// SPI_NOR_4B_OPCODES:      use dedicated 4byte address op codes to support
// memory size above 128Mib.
// SPI_NOR_IO_MODE_EN_VOLATILE: flash enables the best available I/O mode
// via a volatile bit.
// @mfr_flags:      manufacturer private flags. Used in the manufacturer fixup
// hooks to differentiate support between flashes of the same
// manufacturer.
// @otp_org:        flash's OTP organization.
// @fixups:         part specific fixup hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info {
    pub name: *mut c_char,
    pub id: *const spi_nor_id,
    pub size: usize,
    pub sector_size: unsigned,
    pub page_size: u16,
    pub n_banks: u8,
    pub addr_nbytes: u8,
    pub flags: u16,

    pub no_sfdp_flags: u8,

    pub fixup_flags: u8,

    pub mfr_flags: u8,
    pub otp: *const spi_nor_otp_organization,
    pub fixups: *const spi_nor_fixups,
}

//
// struct spi_nor_manufacturer - SPI NOR manufacturer object
// @name: manufacturer name
// @parts: array of parts supported by this manufacturer
// @nparts: number of entries in the parts array
// @fixups: hooks called at various points in time during spi_nor_scan()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_manufacturer {
    pub name: *const c_char,
    pub parts: *const flash_info,
    pub nparts: c_uint,
    pub fixups: *const spi_nor_fixups,
}

//
// struct sfdp - SFDP data
// @num_dwords: number of entries in the dwords array
// @dwords: array of double words of the SFDP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfdp {
    pub num_dwords: usize,
    pub dwords: *mut u32,
}

// Manufacturer drivers.
extern "C" {
    pub fn spi_nor_write_enable(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_write_disable(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_set_4byte_addr_mode_en4b_ex4b(nor: *mut spi_nor, enable: bool) -> c_int;
}
extern "C" {
    pub fn spi_nor_set_4byte_addr_mode_brwr(nor: *mut spi_nor, enable: bool) -> c_int;
}
extern "C" {
    pub fn spi_nor_set_4byte_addr_mode(nor: *mut spi_nor, enable: bool) -> c_int;
}
extern "C" {
    pub fn spi_nor_wait_till_ready(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_global_block_unlock(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_prep_and_lock(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_unlock_and_unprep(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_sr1_bit6_quad_enable(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_sr2_bit1_quad_enable(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_sr2_bit7_quad_enable(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_read_sr(nor: *mut spi_nor, sr: *mut u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_sr_ready(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_read_cr(nor: *mut spi_nor, cr: *mut u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_write_sr(nor: *mut spi_nor, sr: *const u8, len: usize) -> c_int;
}
extern "C" {
    pub fn spi_nor_write_sr_and_check(nor: *mut spi_nor, sr1: u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_write_16bit_cr_and_check(nor: *mut spi_nor, cr: u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_write_sr_cr_and_check(nor: *mut spi_nor, regs: *const u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_erase_sector(nor: *mut spi_nor, addr: u32) -> c_int;
}
extern "C" {
    pub fn spi_nor_otp_read_secr(nor: *mut spi_nor, addr: loff_t, len: usize, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn spi_nor_otp_erase_secr(nor: *mut spi_nor, addr: loff_t) -> c_int;
}
extern "C" {
    pub fn spi_nor_otp_lock_sr2(nor: *mut spi_nor, region: c_uint) -> c_int;
}
extern "C" {
    pub fn spi_nor_otp_is_locked_sr2(nor: *mut spi_nor, region: c_uint) -> c_int;
}
extern "C" {
    pub fn spi_nor_hwcaps_read2cmd(hwcaps: u32) -> c_int;
}
extern "C" {
    pub fn spi_nor_hwcaps_pp2cmd(hwcaps: u32) -> c_int;
}
extern "C" {
    pub fn spi_nor_convert_3to4_read(opcode: u8) -> u8;
}
extern "C" {
    pub fn spi_nor_mask_erase_type(erase: *mut spi_nor_erase_type);
}
extern "C" {
    pub fn spi_nor_init_default_locking_ops(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_has_default_locking_ops(nor: *mut spi_nor) -> bool;
}
extern "C" {
    pub fn spi_nor_try_unlock_all(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_cache_sr_lock_bits(nor: *mut spi_nor, sr: *mut u8);
}
extern "C" {
    pub fn spi_nor_set_mtd_locking_ops(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_set_mtd_otp_ops(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_check_sfdp_signature(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn spi_nor_parse_sfdp(nor: *mut spi_nor) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: mtd, spi_nor: struct, _arg: mtd) -> return;
}
//
// spi_nor_needs_sfdp() - returns true if SFDP parsing is used for this flash.
//
// Return: true if SFDP parsing is needed
//
// The flash size is one property parsed by the SFDP. We use it as an
// indicator whether we need SFDP parsing for a particular flash. I.e.
// non-legacy flash entries in flash_info will have a size of zero iff
// SFDP should be used.
//
extern "C" {
    pub fn spi_nor_get_min_prot_length_sr(nor: *mut spi_nor) -> u64;
}
extern "C" {
    pub fn spi_nor_get_locked_range_sr(nor: *mut spi_nor, sr: *const u8, ofs: *mut loff_t, len: *mut u64);
}
extern "C" {
    pub fn spi_nor_is_locked_sr(nor: *mut spi_nor, ofs: loff_t, len: u64, sr: *const u8) -> bool;
}

extern "C" {
    pub fn spi_nor_debugfs_register(nor: *mut spi_nor);
}
extern "C" {
    pub fn spi_nor_debugfs_shutdown();
}

