//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/nand.h
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
// Copyright 2017 - Free Electrons
//
// Authors:
// Boris Brezillon <boris.brezillon@free-electrons.com>
// Peter Pan <peterpandong@micron.com>
//

//
// struct nand_memory_organization - Memory organization structure
// @bits_per_cell: number of bits per NAND cell
// @pagesize: page size
// @oobsize: OOB area size
// @pages_per_eraseblock: number of pages per eraseblock
// @eraseblocks_per_lun: number of eraseblocks per LUN (Logical Unit Number)
// @max_bad_eraseblocks_per_lun: maximum number of bad eraseblocks per LUN
// @planes_per_lun: number of planes per LUN
// @luns_per_target: number of LUN per target (target is a synonym for die)
// @ntargets: total number of targets exposed by the NAND device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_memory_organization {
    pub bits_per_cell: c_uint,
    pub pagesize: c_uint,
    pub oobsize: c_uint,
    pub pages_per_eraseblock: c_uint,
    pub eraseblocks_per_lun: c_uint,
    pub max_bad_eraseblocks_per_lun: c_uint,
    pub planes_per_lun: c_uint,
    pub luns_per_target: c_uint,
    pub ntargets: c_uint,
}

//
// struct nand_row_converter - Information needed to convert an absolute offset
// into a row address
// @lun_addr_shift: position of the LUN identifier in the row address
// @eraseblock_addr_shift: position of the eraseblock identifier in the row
// address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_row_converter {
    pub lun_addr_shift: c_uint,
    pub eraseblock_addr_shift: c_uint,
}

//
// struct nand_pos - NAND position object
// @target: the NAND target/die
// @lun: the LUN identifier
// @plane: the plane within the LUN
// @eraseblock: the eraseblock within the LUN
// @page: the page within the LUN
//
// These information are usually used by specific sub-layers to select the
// appropriate target/die and generate a row address to pass to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_pos {
    pub target: c_uint,
    pub lun: c_uint,
    pub plane: c_uint,
    pub eraseblock: c_uint,
    pub page: c_uint,
}

//
// enum nand_page_io_req_type - Direction of an I/O request
// @NAND_PAGE_READ: from the chip, to the controller
// @NAND_PAGE_WRITE: from the controller, to the chip
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_page_io_req_type {
    NAND_PAGE_READ = 0,
    NAND_PAGE_WRITE,
}

//
// struct nand_page_io_req - NAND I/O request object
// @type: the type of page I/O: read or write
// @pos: the position this I/O request is targeting
// @dataoffs: the offset within the page
// @datalen: number of data bytes to read from/write to this page
// @databuf: buffer to store data in or get data from
// @ooboffs: the OOB offset within the page
// @ooblen: the number of OOB bytes to read from/write to this page
// @oobbuf: buffer to store OOB data in or get OOB data from
// @mode: one of the %MTD_OPS_XXX mode
// @continuous: no need to start over the operation at the end of each page, the
// NAND device will automatically prepare the next one
//
// This object is used to pass per-page I/O requests to NAND sub-layers. This
// way all useful information are already formatted in a useful way and
// specific NAND layers can focus on translating these information into
// specific commands/operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_page_io_req {
    pub type: nand_page_io_req_type,
    pub pos: nand_pos,
    pub dataoffs: c_uint,
    pub datalen: c_uint,
    pub out: *const c_void,
    pub in: *mut c_void,
    pub databuf: },
    pub ooboffs: c_uint,
    pub ooblen: c_uint,
    pub out: *const c_void,
    pub in: *mut c_void,
    pub oobbuf: },
    pub mode: c_int,
    pub continuous: bool,
}

//
// enum nand_ecc_engine_type - NAND ECC engine type
// @NAND_ECC_ENGINE_TYPE_INVALID: Invalid value
// @NAND_ECC_ENGINE_TYPE_NONE: No ECC correction
// @NAND_ECC_ENGINE_TYPE_SOFT: Software ECC correction
// @NAND_ECC_ENGINE_TYPE_ON_HOST: On host hardware ECC correction
// @NAND_ECC_ENGINE_TYPE_ON_DIE: On chip hardware ECC correction
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_ecc_engine_type {
    NAND_ECC_ENGINE_TYPE_INVALID,
    NAND_ECC_ENGINE_TYPE_NONE,
    NAND_ECC_ENGINE_TYPE_SOFT,
    NAND_ECC_ENGINE_TYPE_ON_HOST,
    NAND_ECC_ENGINE_TYPE_ON_DIE,
}

//
// enum nand_ecc_placement - NAND ECC bytes placement
// @NAND_ECC_PLACEMENT_UNKNOWN: The actual position of the ECC bytes is unknown
// @NAND_ECC_PLACEMENT_OOB: The ECC bytes are located in the OOB area
// @NAND_ECC_PLACEMENT_INTERLEAVED: Syndrome layout, there are ECC bytes
// interleaved with regular data in the main
// area
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_ecc_placement {
    NAND_ECC_PLACEMENT_UNKNOWN,
    NAND_ECC_PLACEMENT_OOB,
    NAND_ECC_PLACEMENT_INTERLEAVED,
}

//
// enum nand_ecc_algo - NAND ECC algorithm
// @NAND_ECC_ALGO_UNKNOWN: Unknown algorithm
// @NAND_ECC_ALGO_HAMMING: Hamming algorithm
// @NAND_ECC_ALGO_BCH: Bose-Chaudhuri-Hocquenghem algorithm
// @NAND_ECC_ALGO_RS: Reed-Solomon algorithm
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_ecc_algo {
    NAND_ECC_ALGO_UNKNOWN,
    NAND_ECC_ALGO_HAMMING,
    NAND_ECC_ALGO_BCH,
    NAND_ECC_ALGO_RS,
}

//
// struct nand_ecc_props - NAND ECC properties
// @engine_type: ECC engine type
// @placement: OOB placement (if relevant)
// @algo: ECC algorithm (if relevant)
// @strength: ECC strength
// @step_size: Number of bytes per step
// @flags: Misc properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_props {
    pub engine_type: nand_ecc_engine_type,
    pub placement: nand_ecc_placement,
    pub algo: nand_ecc_algo,
    pub strength: c_uint,
    pub step_size: c_uint,
    pub flags: c_uint,
}

// NAND ECC misc flags

//
// struct nand_bbt - bad block table object
// @cache: in memory BBT cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_bbt {
    pub cache: *mut c_ulong,
}

//
// struct nand_ops - NAND operations
// @erase: erase a specific block. No need to check if the block is bad before
// erasing, this has been taken care of by the generic NAND layer
// @markbad: mark a specific block bad. No need to check if the block is
// already marked bad, this has been taken care of by the generic
// NAND layer. This method should just write the BBM (Bad Block
// Marker) so that future call to struct_nand_ops->isbad() return
// true
// @isbad: check whether a block is bad or not. This method should just read
// the BBM and return whether the block is bad or not based on what it
// reads
//
// These are all low level operations that should be implemented by specialized
// NAND layers (SPI NAND, raw NAND, ...).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ops {
    pub pos): *const *const *const int (erase)(struct nand_device nand, struct nand_pos,
    pub pos): *const *const *const int (markbad)(struct nand_device nand, struct nand_pos,
    pub pos): *const *const *const bool (isbad)(struct nand_device nand, struct nand_pos,
}

//
// struct nand_ecc_context - Context for the ECC engine
// @conf: basic ECC engine parameters
// @nsteps: number of ECC steps
// @total: total number of bytes used for storing ECC codes, this is used by
// generic OOB layouts
// @priv: ECC engine driver private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_context {
    pub conf: nand_ecc_props,
    pub nsteps: c_uint,
    pub total: c_uint,
    pub priv: *mut c_void,
}

//
// struct nand_ecc_engine_ops - ECC engine operations
// @init_ctx: given a desired user configuration for the pointed NAND device,
// requests the ECC engine driver to setup a configuration with
// values it supports.
// @cleanup_ctx: clean the context initialized by @init_ctx.
// @prepare_io_req: is called before reading/writing a page to prepare the I/O
// request to be performed with ECC correction.
// @finish_io_req: is called after reading/writing a page to terminate the I/O
// request and ensure proper ECC correction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_engine_ops {
    pub nand): *mut *mut int (init_ctx)(struct nand_device,
    pub nand): *mut *mut void (cleanup_ctx)(struct nand_device,
    pub req): *mut nand_page_io_req,
    pub req): *mut nand_page_io_req,
}

//
// enum nand_ecc_engine_integration - How the NAND ECC engine is integrated
// @NAND_ECC_ENGINE_INTEGRATION_INVALID: Invalid value
// @NAND_ECC_ENGINE_INTEGRATION_PIPELINED: Pipelined engine, performs on-the-fly
// correction, does not need to copy
// data around
// @NAND_ECC_ENGINE_INTEGRATION_EXTERNAL: External engine, needs to bring the
// data into its own area before use
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_ecc_engine_integration {
    NAND_ECC_ENGINE_INTEGRATION_INVALID,
    NAND_ECC_ENGINE_INTEGRATION_PIPELINED,
    NAND_ECC_ENGINE_INTEGRATION_EXTERNAL,
}

//
// struct nand_ecc_engine - ECC engine abstraction for NAND devices
// @dev: Host device
// @node: Private field for registration time
// @ops: ECC engine operations
// @integration: How the engine is integrated with the host
// (only relevant on %NAND_ECC_ENGINE_TYPE_ON_HOST engines)
// @priv: Private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_engine {
    pub dev: *mut device,
    pub node: list_head,
    pub ops: *const nand_ecc_engine_ops,
    pub integration: nand_ecc_engine_integration,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn of_get_nand_ecc_user_config(nand: *mut nand_device);
}
extern "C" {
    pub fn nand_ecc_init_ctx(nand: *mut nand_device) -> c_int;
}
extern "C" {
    pub fn nand_ecc_cleanup_ctx(nand: *mut nand_device);
}
extern "C" {
    pub fn nand_ecc_is_strong_enough(nand: *mut nand_device) -> bool;
}

extern "C" {
    pub fn nand_ecc_register_on_host_hw_engine(engine: *mut nand_ecc_engine) -> c_int;
}
extern "C" {
    pub fn nand_ecc_unregister_on_host_hw_engine(engine: *mut nand_ecc_engine) -> c_int;
}

extern "C" {
    pub fn nand_ecc_put_on_host_hw_engine(nand: *mut nand_device);
}

//
// struct nand_ecc_req_tweak_ctx - Help for automatically tweaking requests
// @orig_req: Pointer to the original IO request
// @nand: Related NAND device, to have access to its memory organization
// @page_buffer_size: Real size of the page buffer to use (can be set by the
// user before the tweaking mechanism initialization)
// @oob_buffer_size: Real size of the OOB buffer to use (can be set by the
// user before the tweaking mechanism initialization)
// @spare_databuf: Data bounce buffer
// @spare_oobbuf: OOB bounce buffer
// @bounce_data: Flag indicating a data bounce buffer is used
// @bounce_oob: Flag indicating an OOB bounce buffer is used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_req_tweak_ctx {
    pub orig_req: nand_page_io_req,
    pub nand: *mut nand_device,
    pub page_buffer_size: c_uint,
    pub oob_buffer_size: c_uint,
    pub spare_databuf: *mut c_void,
    pub spare_oobbuf: *mut c_void,
    pub bounce_data: bool,
    pub bounce_oob: bool,
}

extern "C" {
    pub fn nand_ecc_cleanup_req_tweaking(ctx: *mut nand_ecc_req_tweak_ctx);
}
//
// struct nand_ecc - Information relative to the ECC
// @defaults: Default values, depend on the underlying subsystem
// @requirements: ECC requirements from the NAND chip perspective
// @user_conf: User desires in terms of ECC parameters
// @ctx: ECC context for the ECC engine, derived from the device @requirements
// the @user_conf and the @defaults
// @ondie_engine: On-die ECC engine reference, if any
// @engine: ECC engine actually bound
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc {
    pub defaults: nand_ecc_props,
    pub requirements: nand_ecc_props,
    pub user_conf: nand_ecc_props,
    pub ctx: nand_ecc_context,
    pub ondie_engine: *mut nand_ecc_engine,
    pub engine: *mut nand_ecc_engine,
}

//
// struct nand_device - NAND device
// @mtd: MTD instance attached to the NAND device
// @memorg: memory layout
// @ecc: NAND ECC object attached to the NAND device
// @rowconv: position to row address converter
// @bbt: bad block table info
// @ops: NAND operations attached to the NAND device
//
// Generic NAND object. Specialized NAND layers (raw NAND, SPI NAND, OneNAND)
// should declare their own NAND object embedding a nand_device struct (that's
// how inheritance is done).
// struct_nand_device->memorg and struct_nand_device->ecc.requirements should
// be filled at device detection time to reflect the NAND device
// capabilities/requirements. Once this is done nanddev_init() can be called.
// It will take care of converting NAND information into MTD ones, which means
// the specialized NAND layers should never manually tweak
// struct_nand_device->mtd except for the ->_read/write() hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_device {
    pub mtd: mtd_info,
    pub memorg: nand_memory_organization,
    pub ecc: nand_ecc,
    pub rowconv: nand_row_converter,
    pub bbt: nand_bbt,
    pub ops: *const nand_ops,
}

//
// struct nand_io_iter - NAND I/O iterator
// @req: current I/O request
// @oobbytes_per_page: maximum number of OOB bytes per page
// @dataleft: remaining number of data bytes to read/write
// @oobleft: remaining number of OOB bytes to read/write
//
// Can be used by specialized NAND layers to iterate over all pages covered
// by an MTD I/O request, which should greatly simplifies the boiler-plate
// code needed to read/write data from/to a NAND device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_io_iter {
    pub req: nand_page_io_req,
    pub oobbytes_per_page: c_uint,
    pub dataleft: c_uint,
    pub oobleft: c_uint,
}

//
// mtd_to_nanddev() - Get the NAND device attached to the MTD instance
// @mtd: MTD instance
//
// Return: the NAND device embedding @mtd.
//
extern "C" {
    pub fn container_of(_arg: mtd, nand_device: struct, _arg: mtd) -> return;
}
//
// nanddev_to_mtd() - Get the MTD device attached to a NAND device
// @nand: NAND device
//
// Return: the MTD device embedded in @nand.
//
// nanddev_bits_per_cell() - Get the number of bits per cell
// @nand: NAND device
//
// Return: the number of bits per cell.
//
// nanddev_page_size() - Get NAND page size
// @nand: NAND device
//
// Return: the page size.
//
// nanddev_per_page_oobsize() - Get NAND OOB size
// @nand: NAND device
//
// Return: the OOB size.
//
// nanddev_pages_per_eraseblock() - Get the number of pages per eraseblock
// @nand: NAND device
//
// Return: the number of pages per eraseblock.
//
// nanddev_pages_per_target() - Get the number of pages per target
// @nand: NAND device
//
// Return: the number of pages per target.
//
// nanddev_per_page_oobsize() - Get NAND erase block size
// @nand: NAND device
//
// Return: the eraseblock size.
//
// nanddev_eraseblocks_per_lun() - Get the number of eraseblocks per LUN
// @nand: NAND device
//
// Return: the number of eraseblocks per LUN.
//
// nanddev_eraseblocks_per_target() - Get the number of eraseblocks per target
// @nand: NAND device
//
// Return: the number of eraseblocks per target.
//
// nanddev_target_size() - Get the total size provided by a single target/die
// @nand: NAND device
//
// Return: the total size exposed by a single target/die in bytes.
//
// nanddev_ntarget() - Get the total of targets
// @nand: NAND device
//
// Return: the number of targets/dies exposed by @nand.
//
// nanddev_neraseblocks() - Get the total number of eraseblocks
// @nand: NAND device
//
// Return: the total number of eraseblocks exposed by @nand.
//
// nanddev_size() - Get NAND size
// @nand: NAND device
//
// Return: the total size (in bytes) exposed by @nand.
//
extern "C" {
    pub fn nanddev_target_size(nanddev_ntargets(nand: *mut *mut nand)) -> return;
}
//
// nanddev_get_memorg() - Extract memory organization info from a NAND device
// @nand: NAND device
//
// This can be used by the upper layer to fill the memorg info before calling
// nanddev_init().
//
// Return: the memorg object embedded in the NAND device.
//
// nanddev_get_ecc_conf() - Extract the ECC configuration from a NAND device
// @nand: NAND device
//
// nanddev_get_ecc_nsteps() - Extract the number of ECC steps
// @nand: NAND device
//
// nanddev_get_ecc_bytes_per_step() - Extract the number of ECC bytes per step
// @nand: NAND device
//
// nanddev_get_ecc_requirements() - Extract the ECC requirements from a NAND
// device
// @nand: NAND device
//
// nanddev_set_ecc_requirements() - Assign the ECC requirements of a NAND
// device
// @nand: NAND device
// @reqs: Requirements
//
extern "C" {
    pub fn nanddev_cleanup(nand: *mut nand_device);
}
//
// nanddev_register() - Register a NAND device
// @nand: NAND device
//
// Register a NAND device.
// This function is just a wrapper around mtd_device_register()
// registering the MTD device embedded in @nand.
//
// Return: 0 in case of success, a negative error code otherwise.
//
extern "C" {
    pub fn mtd_device_register(_arg: &nand->mtd, _arg: NULL, _arg: 0) -> return;
}
//
// nanddev_unregister() - Unregister a NAND device
// @nand: NAND device
//
// Unregister a NAND device.
// This function is just a wrapper around mtd_device_unregister()
// unregistering the MTD device embedded in @nand.
//
// Return: 0 in case of success, a negative error code otherwise.
//
extern "C" {
    pub fn mtd_device_unregister(_arg: &nand->mtd) -> return;
}
//
// nanddev_set_of_node() - Attach a DT node to a NAND device
// @nand: NAND device
// @np: DT node
//
// Attach a DT node to a NAND device.
//
// nanddev_get_of_node() - Retrieve the DT node attached to a NAND device
// @nand: NAND device
//
// Return: the DT node attached to @nand.
//
extern "C" {
    pub fn mtd_get_of_node(_arg: &nand->mtd) -> return;
}
//
// nanddev_offs_to_pos() - Convert an absolute NAND offset into a NAND position
// @nand: NAND device
// @offs: absolute NAND offset (usually passed by the MTD layer)
// @pos: a NAND position object to fill in
//
// Converts @offs into a nand_pos representation.
//
// Return: the offset within the NAND page pointed by @pos.
//
// nanddev_pos_cmp() - Compare two NAND positions
// @a: First NAND position
// @b: Second NAND position
//
// Compares two NAND positions.
//
// Return: -1 if @a < @b, 0 if @a == @b and 1 if @a > @b.
//
// nanddev_pos_to_offs() - Convert a NAND position into an absolute offset
// @nand: NAND device
// @pos: the NAND position to convert
//
// Converts @pos NAND position into an absolute offset.
//
// Return: the absolute offset. Note that @pos points to the beginning of a
// page, if one wants to point to a specific offset within this page
// the returned offset has to be adjusted manually.
//
// nanddev_pos_to_row() - Extract a row address from a NAND position
// @nand: NAND device
// @pos: the position to convert
//
// Converts a NAND position into a row address that can then be passed to the
// device.
//
// Return: the row address extracted from @pos.
//
// nanddev_pos_next_target() - Move a position to the next target/die
// @nand: NAND device
// @pos: the position to update
//
// Updates @pos to point to the start of the next target/die. Useful when you
// want to iterate over all targets/dies of a NAND device.
//
// nanddev_pos_next_lun() - Move a position to the next LUN
// @nand: NAND device
// @pos: the position to update
//
// Updates @pos to point to the start of the next LUN. Useful when you want to
// iterate over all LUNs of a NAND device.
//
extern "C" {
    pub fn nanddev_pos_next_target(_arg: nand, _arg: pos) -> return;
}
//
// nanddev_pos_next_eraseblock() - Move a position to the next eraseblock
// @nand: NAND device
// @pos: the position to update
//
// Updates @pos to point to the start of the next eraseblock. Useful when you
// want to iterate over all eraseblocks of a NAND device.
//
extern "C" {
    pub fn nanddev_pos_next_lun(_arg: nand, _arg: pos) -> return;
}
//
// nanddev_pos_next_page() - Move a position to the next page
// @nand: NAND device
// @pos: the position to update
//
// Updates @pos to point to the start of the next page. Useful when you want to
// iterate over all pages of a NAND device.
//
extern "C" {
    pub fn nanddev_pos_next_eraseblock(_arg: nand, _arg: pos) -> return;
}
//
// nand_io_page_iter_init - Initialize a NAND I/O iterator
// @nand: NAND device
// @offs: absolute offset
// @req: MTD request
// @iter: NAND I/O iterator
//
// Initializes a NAND iterator based on the information passed by the MTD
// layer for page jumps.
//
// nand_io_block_iter_init - Initialize a NAND I/O iterator
// @nand: NAND device
// @offs: absolute offset
// @req: MTD request
// @iter: NAND I/O iterator
//
// Initializes a NAND iterator based on the information passed by the MTD
// layer for block jumps (no OOB)
//
// In practice only reads may leverage this iterator.
//
// nand_io_iter_next_page - Move to the next page
// @nand: NAND device
// @iter: NAND I/O iterator
//
// Updates the @iter to point to the next page.
//
// nand_io_iter_next_block - Move to the next block
// @nand: NAND device
// @iter: NAND I/O iterator
//
// Updates the @iter to point to the next block.
// No OOB handling available.
//
// nand_io_iter_end - Should end iteration or not
// @nand: NAND device
// @iter: NAND I/O iterator
//
// Check whether @iter has reached the end of the NAND portion it was asked to
// iterate on or not.
//
// Return: true if @iter has reached the end of the iteration request, false
// otherwise.
//
// nand_io_for_each_page - Iterate over all NAND pages contained in an MTD I/O
// request
// @nand: NAND device
// @start: start address to read/write from
// @req: MTD I/O request
// @iter: NAND I/O iterator
//
// Should be used for iterating over pages that are contained in an MTD request.
//

//
// nand_io_for_each_block - Iterate over all NAND pages contained in an MTD I/O
// request, one block at a time
// @nand: NAND device
// @start: start address to read/write from
// @req: MTD I/O request
// @iter: NAND I/O iterator
//
// Should be used for iterating over blocks that are contained in an MTD request.
//

extern "C" {
    pub fn nanddev_isbad(nand: *mut nand_device, pos: *const nand_pos) -> bool;
}
extern "C" {
    pub fn nanddev_isreserved(nand: *mut nand_device, pos: *const nand_pos) -> bool;
}
extern "C" {
    pub fn nanddev_markbad(nand: *mut nand_device, pos: *const nand_pos) -> c_int;
}
// ECC related functions
extern "C" {
    pub fn nanddev_ecc_engine_init(nand: *mut nand_device) -> c_int;
}
extern "C" {
    pub fn nanddev_ecc_engine_cleanup(nand: *mut nand_device);
}
// BBT related functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_bbt_block_status {
    NAND_BBT_BLOCK_STATUS_UNKNOWN,
    NAND_BBT_BLOCK_GOOD,
    NAND_BBT_BLOCK_WORN,
    NAND_BBT_BLOCK_RESERVED,
    NAND_BBT_BLOCK_FACTORY_BAD,
    NAND_BBT_BLOCK_NUM_STATUS,
}

extern "C" {
    pub fn nanddev_bbt_init(nand: *mut nand_device) -> c_int;
}
extern "C" {
    pub fn nanddev_bbt_cleanup(nand: *mut nand_device);
}
extern "C" {
    pub fn nanddev_bbt_update(nand: *mut nand_device) -> c_int;
}
extern "C" {
    pub fn nanddev_bbt_markbad(nand: *mut nand_device, block: c_uint) -> c_int;
}
//
// nanddev_bbt_pos_to_entry() - Convert a NAND position into a BBT entry
// @nand: NAND device
// @pos: the NAND position we want to get BBT entry for
//
// Return the BBT entry used to store information about the eraseblock pointed
// by @pos.
//
// Return: the BBT entry storing information about eraseblock pointed by @pos.
//
// nanddev_bbt_is_initialized() - Check if the BBT has been initialized
// @nand: NAND device
//
// Return: true if the BBT has been initialized, false otherwise.
//
// MTD -> NAND helper functions.
extern "C" {
    pub fn nanddev_mtd_erase(mtd: *mut mtd_info, einfo: *mut erase_info) -> c_int;
}
extern "C" {
    pub fn nanddev_mtd_max_bad_blocks(mtd: *mut mtd_info, offs: loff_t, len: usize) -> c_int;
}
