//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/spi-nor.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//

//
// Note on opcode nomenclature: some opcodes have a format like
// SPINOR_OP_FUNCTION{4,}_x_y_z. The numbers x, y, and z stand for the number
// of I/O lines used for the opcode, address, and data (respectively). The
// FUNCTION has an optional suffix of '4', to represent an opcode which
// requires a 4-byte (32-bit) address.
//
// Flash opcodes.
pub const SPINOR_OP_WRDI: c_uint = 0x04	/* Write disable */;
pub const SPINOR_OP_WREN: c_uint = 0x06	/* Write enable */;
pub const SPINOR_OP_RDSR: c_uint = 0x05	/* Read status register 1 */;
pub const SPINOR_OP_WRSR: c_uint = 0x01	/* Write status register 1 */;
pub const SPINOR_OP_RDSR2: c_uint = 0x3f	/* Read status register 2 */;
pub const SPINOR_OP_WRSR2: c_uint = 0x3e	/* Write status register 2 */;
pub const SPINOR_OP_READ: c_uint = 0x03	/* Read data bytes (low frequency) */;
pub const SPINOR_OP_READ_FAST: c_uint = 0x0b	/* Read data bytes (high frequency) */;
pub const SPINOR_OP_READ_1_1_2: c_uint = 0x3b	/* Read data bytes (Dual Output SPI) */;
pub const SPINOR_OP_READ_1_2_2: c_uint = 0xbb	/* Read data bytes (Dual I/O SPI) */;
pub const SPINOR_OP_READ_1_1_4: c_uint = 0x6b	/* Read data bytes (Quad Output SPI) */;
pub const SPINOR_OP_READ_1_4_4: c_uint = 0xeb	/* Read data bytes (Quad I/O SPI) */;
pub const SPINOR_OP_READ_1_1_8: c_uint = 0x8b	/* Read data bytes (Octal Output SPI) */;
pub const SPINOR_OP_READ_1_8_8: c_uint = 0xcb	/* Read data bytes (Octal I/O SPI) */;
pub const SPINOR_OP_PP: c_uint = 0x02	/* Page program (up to 256 bytes) */;
pub const SPINOR_OP_PP_1_1_4: c_uint = 0x32	/* Quad page program */;
pub const SPINOR_OP_PP_1_4_4: c_uint = 0x38	/* Quad page program */;
pub const SPINOR_OP_PP_1_1_8: c_uint = 0x82	/* Octal page program */;
pub const SPINOR_OP_PP_1_8_8: c_uint = 0xc2	/* Octal page program */;
pub const SPINOR_OP_BE_4K: c_uint = 0x20	/* Erase 4KiB block */;
pub const SPINOR_OP_BE_4K_PMC: c_uint = 0xd7	/* Erase 4KiB block on PMC chips */;
pub const SPINOR_OP_BE_32K: c_uint = 0x52	/* Erase 32KiB block */;
pub const SPINOR_OP_CHIP_ERASE: c_uint = 0xc7	/* Erase whole flash chip */;
pub const SPINOR_OP_SE: c_uint = 0xd8	/* Sector erase (usually 64KiB) */;
pub const SPINOR_OP_RDID: c_uint = 0x9f	/* Read JEDEC ID */;
pub const SPINOR_OP_RDSFDP: c_uint = 0x5a	/* Read SFDP */;
pub const SPINOR_OP_RDCR: c_uint = 0x35	/* Read configuration register */;
pub const SPINOR_OP_SRSTEN: c_uint = 0x66	/* Software Reset Enable */;
pub const SPINOR_OP_SRST: c_uint = 0x99	/* Software Reset */;
pub const SPINOR_OP_GBULK: c_uint = 0x98    /* Global Block Unlock */;
// 4-byte address opcodes - used on Spansion and some Macronix flashes.
pub const SPINOR_OP_READ_4B: c_uint = 0x13	/* Read data bytes (low frequency) */;
pub const SPINOR_OP_READ_FAST_4B: c_uint = 0x0c	/* Read data bytes (high frequency) */;
pub const SPINOR_OP_READ_1_1_2_4B: c_uint = 0x3c	/* Read data bytes (Dual Output SPI) */;
pub const SPINOR_OP_READ_1_2_2_4B: c_uint = 0xbc	/* Read data bytes (Dual I/O SPI) */;
pub const SPINOR_OP_READ_1_1_4_4B: c_uint = 0x6c	/* Read data bytes (Quad Output SPI) */;
pub const SPINOR_OP_READ_1_4_4_4B: c_uint = 0xec	/* Read data bytes (Quad I/O SPI) */;
pub const SPINOR_OP_READ_1_1_8_4B: c_uint = 0x7c	/* Read data bytes (Octal Output SPI) */;
pub const SPINOR_OP_READ_1_8_8_4B: c_uint = 0xcc	/* Read data bytes (Octal I/O SPI) */;
pub const SPINOR_OP_PP_4B: c_uint = 0x12	/* Page program (up to 256 bytes) */;
pub const SPINOR_OP_PP_1_1_4_4B: c_uint = 0x34	/* Quad page program */;
pub const SPINOR_OP_PP_1_4_4_4B: c_uint = 0x3e	/* Quad page program */;
pub const SPINOR_OP_PP_1_1_8_4B: c_uint = 0x84	/* Octal page program */;
pub const SPINOR_OP_PP_1_8_8_4B: c_uint = 0x8e	/* Octal page program */;
pub const SPINOR_OP_BE_4K_4B: c_uint = 0x21	/* Erase 4KiB block */;
pub const SPINOR_OP_BE_32K_4B: c_uint = 0x5c	/* Erase 32KiB block */;
pub const SPINOR_OP_SE_4B: c_uint = 0xdc	/* Sector erase (usually 64KiB) */;
// Double Transfer Rate opcodes - defined in JEDEC JESD216B.
pub const SPINOR_OP_READ_1_1_1_DTR: c_uint = 0x0d;
pub const SPINOR_OP_READ_1_2_2_DTR: c_uint = 0xbd;
pub const SPINOR_OP_READ_1_4_4_DTR: c_uint = 0xed;
pub const SPINOR_OP_READ_1_1_1_DTR_4B: c_uint = 0x0e;
pub const SPINOR_OP_READ_1_2_2_DTR_4B: c_uint = 0xbe;
pub const SPINOR_OP_READ_1_4_4_DTR_4B: c_uint = 0xee;
// Used for SST flashes only.
pub const SPINOR_OP_BP: c_uint = 0x02	/* Byte program */;
pub const SPINOR_OP_AAI_WP: c_uint = 0xad	/* Auto address increment word program */;
// Used for Macronix and Winbond flashes.
pub const SPINOR_OP_EN4B: c_uint = 0xb7	/* Enter 4-byte mode */;
pub const SPINOR_OP_EX4B: c_uint = 0xe9	/* Exit 4-byte mode */;
// Used for Spansion flashes only.
pub const SPINOR_OP_BRWR: c_uint = 0x17	/* Bank register write */;
// Used for Micron flashes only.
pub const SPINOR_OP_RD_EVCR: c_uint = 0x65    /* Read EVCR register */;
pub const SPINOR_OP_WD_EVCR: c_uint = 0x61    /* Write EVCR register */;
// Used for GigaDevices and Winbond flashes.
pub const SPINOR_OP_ESECR: c_uint = 0x44	/* Erase Security registers */;
pub const SPINOR_OP_PSECR: c_uint = 0x42	/* Program Security registers */;
pub const SPINOR_OP_RSECR: c_uint = 0x48	/* Read Security registers */;
// Status Register bits.

// meaning of other SR_* bits may differ between vendors

// Spansion/Cypress specific status bits

pub const SR_BP_SHIFT: c_int = 2;
// Enhanced Volatile Configuration Register bits

// Status Register 2 bits.

// Supported SPI protocols

pub const SNOR_PROTO_INST_SHIFT: c_int = 16;

pub const SNOR_PROTO_ADDR_SHIFT: c_int = 8;

pub const SNOR_PROTO_DATA_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_nor_protocol {
    SNOR_PROTO_1_1_1 = SNOR_PROTO_STR(1, 1, 1),
    SNOR_PROTO_1_1_2 = SNOR_PROTO_STR(1, 1, 2),
    SNOR_PROTO_1_1_4 = SNOR_PROTO_STR(1, 1, 4),
    SNOR_PROTO_1_1_8 = SNOR_PROTO_STR(1, 1, 8),
    SNOR_PROTO_1_2_2 = SNOR_PROTO_STR(1, 2, 2),
    SNOR_PROTO_1_4_4 = SNOR_PROTO_STR(1, 4, 4),
    SNOR_PROTO_1_8_8 = SNOR_PROTO_STR(1, 8, 8),
    SNOR_PROTO_2_2_2 = SNOR_PROTO_STR(2, 2, 2),
    SNOR_PROTO_4_4_4 = SNOR_PROTO_STR(4, 4, 4),
    SNOR_PROTO_8_8_8 = SNOR_PROTO_STR(8, 8, 8),

    SNOR_PROTO_1_1_1_DTR = SNOR_PROTO_DTR(1, 1, 1),
    SNOR_PROTO_1_2_2_DTR = SNOR_PROTO_DTR(1, 2, 2),
    SNOR_PROTO_1_4_4_DTR = SNOR_PROTO_DTR(1, 4, 4),
    SNOR_PROTO_1_8_8_DTR = SNOR_PROTO_DTR(1, 8, 8),
    SNOR_PROTO_8_8_8_DTR = SNOR_PROTO_DTR(8, 8, 8),
}

extern "C" {
    pub fn spi_nor_get_protocol_data_nbits(_arg: proto) -> return;
}
//
// struct spi_nor_hwcaps - Structure for describing the hardware capabilies
// supported by the SPI controller (bus master).
// @mask:		the bitmask listing all the supported hw capabilies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_hwcaps {
    pub mask: u32,
}

//
// (Fast) Read capabilities.
// MUST be ordered by priority: the higher bit position, the higher priority.
// As a matter of performances, it is relevant to use Octal SPI protocols first,
// then Quad SPI protocols before Dual SPI protocols, Fast Read and lastly
// (Slow) Read.
//

//
// Page Program capabilities.
// MUST be ordered by priority: the higher bit position, the higher priority.
// Like (Fast) Read capabilities, Octal/Quad SPI protocols are preferred to the
// legacy SPI 1-1-1 protocol.
// Note that Dual Page Programs are not supported because there is no existing
// JEDEC/SFDP standard to define them. Also at this moment no SPI flash memory
// implements such commands.
//

// Forward declaration that is used in 'struct spi_nor_controller_ops'
//
// struct spi_nor_controller_ops - SPI NOR controller driver specific
// operations.
// @prepare:		[OPTIONAL] do some preparations for the
// read/write/erase/lock/unlock operations.
// @unprepare:		[OPTIONAL] do some post work after the
// read/write/erase/lock/unlock operations.
// @read_reg:		read out the register.
// @write_reg:		write data to the register.
// @read:		read data from the SPI NOR.
// @write:		write data to the SPI NOR.
// @erase:		erase a sector of the SPI NOR at the offset @offs; if
// not provided by the driver, SPI NOR will send the erase
// opcode via write_reg().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_controller_ops {
    pub nor): *mut *mut int (prepare)(struct spi_nor,
    pub nor): *mut *mut void (unprepare)(struct spi_nor,
    pub len): *mut *mut *mut *mut int (read_reg)(struct spi_nor nor, u8 opcode, u8 buf, size_t,
    pub len): usize,
    pub buf): *mut *mut *mut ssize_t (read)(struct spi_nor nor, loff_t from, size_t len, u8,
    pub buf): *const u8,
    pub offs): *mut *mut *mut int (erase)(struct spi_nor nor, loff_t,
}

//
// enum spi_nor_cmd_ext - describes the command opcode extension in DTR mode
// @SPI_NOR_EXT_NONE: no extension. This is the default, and is used in Legacy
// SPI mode
// @SPI_NOR_EXT_REPEAT: the extension is same as the opcode
// @SPI_NOR_EXT_INVERT: the extension is the bitwise inverse of the opcode
// @SPI_NOR_EXT_HEX: the extension is any hex value. The command and opcode
// combine to form a 16-bit opcode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_nor_cmd_ext {
    SPI_NOR_EXT_NONE = 0,
    SPI_NOR_EXT_REPEAT,
    SPI_NOR_EXT_INVERT,
    SPI_NOR_EXT_HEX,
}

//
// Forward declarations that are used internally by the core and manufacturer
// drivers.
//
// struct spi_nor - Structure for defining the SPI NOR layer
// @mtd:		an mtd_info structure
// @lock:		the lock for the read/write/erase/lock/unlock operations
// @rww:		Read-While-Write (RWW) sync lock
// @rww.wait:		wait queue for the RWW sync
// @rww.ongoing_io:	the bus is busy
// @rww.ongoing_rd:	a read is ongoing on the chip
// @rww.ongoing_pe:	a program/erase is ongoing on the chip
// @rww.used_banks:	bitmap of the banks in use
// @dev:		pointer to an SPI device or an SPI NOR controller device
// @spimem:		pointer to the SPI memory device
// @bouncebuf:		bounce buffer used when the buffer passed by the MTD
// layer is not DMA-able
// @bouncebuf_size:	size of the bounce buffer
// @id:			The flash's ID bytes. Always contains
// SPI_NOR_MAX_ID_LEN bytes.
// @info:		SPI NOR part JEDEC MFR ID and other info
// @manufacturer:	SPI NOR manufacturer
// @addr_nbytes:	number of address bytes
// @erase_opcode:	the opcode for erasing a sector
// @read_opcode:	the read opcode
// @read_dummy:		the dummy needed by the read operation
// @program_opcode:	the program opcode
// @sst_write_second:	used by the SST write operation
// @flags:		flag options for the current SPI NOR (SNOR_F_*)
// @cmd_ext_type:	the command opcode extension type for DTR mode.
// @read_proto:		the SPI protocol for read operations
// @write_proto:	the SPI protocol for write operations
// @reg_proto:		the SPI protocol for read_reg/write_reg/erase operations
// @sfdp:		the SFDP data of the flash
// @debugfs_root:	pointer to the debugfs directory
// @dfs_sr_cache:	Status Register cached value for debugfs use only
// @controller_ops:	SPI NOR controller driver specific operations.
// @params:		[FLASH-SPECIFIC] SPI NOR flash parameters and settings.
// The structure includes legacy flash parameters and
// settings that can be overwritten by the spi_nor_fixups
// hooks, or dynamically when parsing the SFDP tables.
// @dirmap:		pointers to struct spi_mem_dirmap_desc for reads/writes.
// @priv:		pointer to the private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor {
    pub mtd: mtd_info,
    pub lock: mutex,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_nor_rww {
    pub wait: wait_queue_head_t,
    pub ongoing_io: bool,
    pub ongoing_rd: bool,
    pub ongoing_pe: bool,
    pub used_banks: c_uint,
    pub rww: },
    pub dev: *mut device,
    pub spimem: *mut spi_mem,
    pub bouncebuf: *mut u8,
    pub bouncebuf_size: usize,
    pub id: *mut u8,
    pub info: *const flash_info,
    pub manufacturer: *const spi_nor_manufacturer,
    pub addr_nbytes: u8,
    pub erase_opcode: u8,
    pub read_opcode: u8,
    pub read_dummy: u8,
    pub program_opcode: u8,
    pub read_proto: spi_nor_protocol,
    pub write_proto: spi_nor_protocol,
    pub reg_proto: spi_nor_protocol,
    pub sst_write_second: bool,
    pub flags: u32,
    pub cmd_ext_type: spi_nor_cmd_ext,
    pub sfdp: *mut sfdp,
    pub debugfs_root: *mut dentry,
    pub dfs_sr_cache: [u8; 2],
    pub controller_ops: *const spi_nor_controller_ops,
    pub params: *mut spi_nor_flash_parameter,
    pub rdesc: *mut spi_mem_dirmap_desc,
    pub wdesc: *mut spi_mem_dirmap_desc,
    pub dirmap: },
    pub priv: *mut c_void,
}

extern "C" {
    pub fn mtd_get_of_node(_arg: &nor->mtd) -> return;
}
//
// spi_nor_scan() - scan the SPI NOR
// @nor:	the spi_nor structure
// @name:	the chip type name
// @hwcaps:	the hardware capabilities supported by the controller driver
//
// The drivers can use this function to scan the SPI NOR.
// In the scanning, it will try to get all the necessary information to
// fill the mtd_info{} and the spi_nor{}.
//
// The chip type name can be provided through the @name parameter.
//
// Return: 0 for success, others for failure.
//
