//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/rawnand.h
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
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
// Steven J. Hill <sjhill@realitydiluted.com>
// Thomas Gleixner <tglx@kernel.org>
//
// Info:
// Contains standard defines and IDs for NAND flash devices
//
// Changelog:
// See git changelog.
//

// The maximum number of NAND chips in an array
pub const NAND_MAX_CHIPS: c_int = 8;
//
// Constants for hardware specific CLE/ALE/NCE function
//
// These are bits which can be or'ed to set/clear multiple
// bits in one go.
//
// Select the chip by setting nCE to low
pub const NAND_NCE: c_uint = 0x01;
// Select the command latch by setting CLE to high
pub const NAND_CLE: c_uint = 0x02;
// Select the address latch by setting ALE to high
pub const NAND_ALE: c_uint = 0x04;

pub const NAND_CTRL_CHANGE: c_uint = 0x80;
//
// Standard NAND flash commands
//
pub const NAND_CMD_READ0: c_int = 0;
pub const NAND_CMD_READ1: c_int = 1;
pub const NAND_CMD_RNDOUT: c_int = 5;
pub const NAND_CMD_PAGEPROG: c_uint = 0x10;
pub const NAND_CMD_READOOB: c_uint = 0x50;
pub const NAND_CMD_ERASE1: c_uint = 0x60;
pub const NAND_CMD_STATUS: c_uint = 0x70;
pub const NAND_CMD_SEQIN: c_uint = 0x80;
pub const NAND_CMD_RNDIN: c_uint = 0x85;
pub const NAND_CMD_READID: c_uint = 0x90;
pub const NAND_CMD_ERASE2: c_uint = 0xd0;
pub const NAND_CMD_PARAM: c_uint = 0xec;
pub const NAND_CMD_GET_FEATURES: c_uint = 0xee;
pub const NAND_CMD_SET_FEATURES: c_uint = 0xef;
pub const NAND_CMD_RESET: c_uint = 0xff;
// Extended commands for large page devices
pub const NAND_CMD_READSTART: c_uint = 0x30;
pub const NAND_CMD_READCACHESEQ: c_uint = 0x31;
pub const NAND_CMD_READCACHEEND: c_uint = 0x3f;
pub const NAND_CMD_RNDOUTSTART: c_uint = 0xE0;
pub const NAND_CMD_CACHEDPROG: c_uint = 0x15;

// Status bits
pub const NAND_STATUS_FAIL: c_uint = 0x01;
pub const NAND_STATUS_FAIL_N1: c_uint = 0x02;
pub const NAND_STATUS_TRUE_READY: c_uint = 0x20;
pub const NAND_STATUS_READY: c_uint = 0x40;
pub const NAND_STATUS_WP: c_uint = 0x80;

//
// Constants for Hardware ECC
//
// Reset Hardware ECC for read
pub const NAND_ECC_READ: c_int = 0;
// Reset Hardware ECC for write
pub const NAND_ECC_WRITE: c_int = 1;
// Enable Hardware ECC before syndrome is read back from flash
pub const NAND_ECC_READSYN: c_int = 2;
//
// Enable generic NAND 'page erased' check. This check is only done when
// ecc.correct() returns -EBADMSG.
// Set this flag if your implementation does not fix bitflips in erased
// pages and you want to rely on the default implementation.
//

//
// Option constants for bizarre disfunctionality and real
// features.
//
// Buswidth is 16 bit

//
// When using software implementation of Hamming, we can specify which byte
// ordering should be used.
//

// Chip has cache program function

// Options valid for Samsung large page devices

//
// Chip requires ready check on read (for auto-incremented sequential read).
// True only for small page devices; large page devices do not support
// autoincrement.
//

// Chip does not allow subpage writes

// Device is one of 'new' xD cards that expose fake nand command set

// Device behaves just like nand, but is readonly

// Device supports subpage reads

// Macros to identify the above

//
// Some MLC NANDs need data scrambling to limit bitflips caused by repeated
// patterns.
//

// Device needs 3rd row address cycle

// Non chip related options
// This option skips the bbt scan during initialization.

// Chip may not exist, so silence any errors in scan

//
// Autodetect nand buswidth with readid/onfi.
// This suppose the driver will configure the hardware in 8 bits mode
// when calling nand_scan_ident, and update its configuration
// before calling nand_scan_tail.
//

//
// This option could be defined by controller drivers to protect against
// kmap'ed, vmalloc'ed highmem buffers being passed from upper layers
//

//
// In case your controller is implementing ->legacy.cmd_ctrl() and is relying
// on the default ->cmdfunc() implementation, you may want to let the core
// handle the tCCS delay which is required when a column change (RNDIN or
// RNDOUT) is requested.
// If your controller already takes care of this delay, you don't need to set
// this flag.
//

//
// Whether the NAND chip is a boot medium. Drivers might use this information
// to select ECC algorithms supported by the boot ROM or similar restrictions.
//

//
// Do not try to tweak the timings at runtime. This is needed when the
// controller initializes the timings on itself or when it relies on
// configuration done by the bootloader.
//

//
// There are different places where the manufacturer stores the factory bad
// block markers.
//
// Position within the block: Each of these pages needs to be checked for a
// bad block marking pattern.
//

//
// Some controllers with pipelined ECC engines override the BBM marker with
// data or ECC bytes, thus making bad block detection through bad block marker
// impossible. Let's flag those chips so the core knows it shouldn't check the
// BBM and consider all blocks good.
//

// Cell info constants
pub const NAND_CI_CHIPNR_MSK: c_uint = 0x03;
pub const NAND_CI_CELLTYPE_MSK: c_uint = 0x0C;
pub const NAND_CI_CELLTYPE_SHIFT: c_int = 2;
// Position within the OOB data of the page
pub const NAND_BBM_POS_SMALL: c_int = 5;
pub const NAND_BBM_POS_LARGE: c_int = 0;
//
// struct nand_parameters - NAND generic parameters from the parameter page
// @model: Model name
// @supports_set_get_features: The NAND chip supports setting/getting features
// @supports_read_cache: The NAND chip supports read cache operations
// @set_feature_list: Bitmap of features that can be set
// @get_feature_list: Bitmap of features that can be get
// @onfi: ONFI specific parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_parameters {
// Generic parameters
    pub model: *const c_char,
    pub supports_set_get_features: bool,
    pub supports_read_cache: bool,
    pub ONFI_FEATURE_NUMBER): DECLARE_BITMAP(set_feature_list,,
    pub ONFI_FEATURE_NUMBER): DECLARE_BITMAP(get_feature_list,,
// ONFI parameters
    pub onfi: *mut onfi_params,
}

// The maximum expected count of bytes in the NAND ID sequence
pub const NAND_MAX_ID_LEN: c_int = 8;
//
// struct nand_id - NAND id structure
// @data: buffer containing the id bytes.
// @len: ID length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_id {
    pub data: [u8; NAND_MAX_ID_LEN],
    pub len: c_int,
}

//
// struct nand_ecc_step_info - ECC step information of ECC engine
// @stepsize: data bytes per ECC step
// @strengths: array of supported strengths
// @nstrengths: number of supported strengths
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_step_info {
    pub stepsize: c_int,
    pub strengths: *const c_int,
    pub nstrengths: c_int,
}

//
// struct nand_ecc_caps - capability of ECC engine
// @stepinfos: array of ECC step information
// @nstepinfos: number of ECC step information
// @calc_ecc_bytes: driver's hook to calculate ECC bytes per step
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_caps {
    pub stepinfos: *const nand_ecc_step_info,
    pub nstepinfos: c_int,
    pub strength): *mut *mut int (calc_ecc_bytes)(int step_size, int,
}

// a shorthand to generate struct nand_ecc_caps with only one ECC stepsize

//
// struct nand_ecc_ctrl - Control structure for ECC
// @engine_type: ECC engine type
// @placement:	OOB bytes placement
// @algo:	ECC algorithm
// @steps:	number of ECC steps per page
// @size:	data bytes per ECC step
// @bytes:	ECC bytes per step
// @strength:	max number of correctible bits per ECC step
// @total:	total number of ECC bytes per page
// @prepad:	padding information for syndrome based ECC generators
// @postpad:	padding information for syndrome based ECC generators
// @options:	ECC specific options (see NAND_ECC_XXX flags defined above)
// @calc_buf:	buffer for calculated ECC, size is oobsize.
// @code_buf:	buffer for ECC read from flash, size is oobsize.
// @hwctl:	function to control hardware ECC generator. Must only
// be provided if an hardware ECC is available
// @calculate:	function for ECC calculation or readback from ECC hardware
// @correct:	function for ECC correction, matching to ECC generator (sw/hw).
// Should return a positive number representing the number of
// corrected bitflips, -EBADMSG if the number of bitflips exceed
// ECC strength, or any other error code if the error is not
// directly related to correction.
// If -EBADMSG is returned the input buffers should be left
// untouched.
// @read_page_raw:	function to read a raw page without ECC. This function
// should hide the specific layout used by the ECC
// controller and always return contiguous in-band and
// out-of-band data even if they're not stored
// contiguously on the NAND chip (e.g.
// NAND_ECC_PLACEMENT_INTERLEAVED interleaves in-band and
// out-of-band data).
// @write_page_raw:	function to write a raw page without ECC. This function
// should hide the specific layout used by the ECC
// controller and consider the passed data as contiguous
// in-band and out-of-band data. ECC controller is
// responsible for doing the appropriate transformations
// to adapt to its specific layout (e.g.
// NAND_ECC_PLACEMENT_INTERLEAVED interleaves in-band and
// out-of-band data).
// @read_page:	function to read a page according to the ECC generator
// requirements; returns maximum number of bitflips corrected in
// any single ECC step, -EIO hw error
// @read_subpage:	function to read parts of the page covered by ECC;
// returns same as read_page()
// @write_subpage:	function to write parts of the page covered by ECC.
// @write_page:	function to write a page according to the ECC generator
// requirements.
// @write_oob_raw:	function to write chip OOB data without ECC
// @read_oob_raw:	function to read chip OOB data without ECC
// @read_oob:	function to read chip OOB data
// @write_oob:	function to write chip OOB data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_ctrl {
    pub engine_type: nand_ecc_engine_type,
    pub placement: nand_ecc_placement,
    pub algo: nand_ecc_algo,
    pub steps: c_int,
    pub size: c_int,
    pub bytes: c_int,
    pub total: c_int,
    pub strength: c_int,
    pub prepad: c_int,
    pub postpad: c_int,
    pub options: c_uint,
    pub calc_buf: *mut u8,
    pub code_buf: *mut u8,
    pub mode): *mut *mut *mut void (hwctl)(struct nand_chip chip, int,
    pub ecc_code): *mut u8,
    pub calc_ecc): *mut u8,
    pub page): int oob_required, int,
    pub page): int oob_required, int,
    pub page): int oob_required, int,
    pub page): *mut *mut uint32_t len, uint8_t buf, int,
    pub page): int oob_required, int,
    pub page): int oob_required, int,
    pub page): *mut *mut *mut int (write_oob_raw)(struct nand_chip chip, int,
    pub page): *mut *mut *mut int (read_oob_raw)(struct nand_chip chip, int,
    pub page): *mut *mut *mut int (read_oob)(struct nand_chip chip, int,
    pub page): *mut *mut *mut int (write_oob)(struct nand_chip chip, int,
}

//
// struct nand_sdr_timings - SDR NAND chip timings
//
// This struct defines the timing requirements of a SDR NAND chip.
// These information can be found in every NAND datasheets and the timings
// meaning are described in the ONFI specifications:
// https://media-www.micron.com/-/media/client/onfi/specs/onfi_3_1_spec.pdf
// (chapter 4.15 Timing Parameters)
//
// All these timings are expressed in picoseconds.
//
// @tBERS_max: Block erase time
// @tCCS_min: Change column setup time
// @tPROG_max: Page program time
// @tR_max: Page read time
// @tALH_min: ALE hold time
// @tADL_min: ALE to data loading time
// @tALS_min: ALE setup time
// @tAR_min: ALE to RE# delay
// @tCEA_max: CE# access time
// @tCEH_min: CE# high hold time
// @tCH_min:  CE# hold time
// @tCHZ_max: CE# high to output hi-Z
// @tCLH_min: CLE hold time
// @tCLR_min: CLE to RE# delay
// @tCLS_min: CLE setup time
// @tCOH_min: CE# high to output hold
// @tCS_min: CE# setup time
// @tDH_min: Data hold time
// @tDS_min: Data setup time
// @tFEAT_max: Busy time for Set Features and Get Features
// @tIR_min: Output hi-Z to RE# low
// @tITC_max: Interface and Timing Mode Change time
// @tRC_min: RE# cycle time
// @tREA_max: RE# access time
// @tREH_min: RE# high hold time
// @tRHOH_min: RE# high to output hold
// @tRHW_min: RE# high to WE# low
// @tRHZ_max: RE# high to output hi-Z
// @tRLOH_min: RE# low to output hold
// @tRP_min: RE# pulse width
// @tRR_min: Ready to RE# low (data only)
// @tRST_max: Device reset time, measured from the falling edge of R/B# to the
// rising edge of R/B#.
// @tWB_max: WE# high to SR[6] low
// @tWC_min: WE# cycle time
// @tWH_min: WE# high hold time
// @tWHR_min: WE# high to RE# low
// @tWP_min: WE# pulse width
// @tWW_min: WP# transition to WE# low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_sdr_timings {
    pub tBERS_max: u64,
    pub tCCS_min: u32,
    pub tPROG_max: u64,
    pub tR_max: u64,
    pub tALH_min: u32,
    pub tADL_min: u32,
    pub tALS_min: u32,
    pub tAR_min: u32,
    pub tCEA_max: u32,
    pub tCEH_min: u32,
    pub tCH_min: u32,
    pub tCHZ_max: u32,
    pub tCLH_min: u32,
    pub tCLR_min: u32,
    pub tCLS_min: u32,
    pub tCOH_min: u32,
    pub tCS_min: u32,
    pub tDH_min: u32,
    pub tDS_min: u32,
    pub tFEAT_max: u32,
    pub tIR_min: u32,
    pub tITC_max: u32,
    pub tRC_min: u32,
    pub tREA_max: u32,
    pub tREH_min: u32,
    pub tRHOH_min: u32,
    pub tRHW_min: u32,
    pub tRHZ_max: u32,
    pub tRLOH_min: u32,
    pub tRP_min: u32,
    pub tRR_min: u32,
    pub tRST_max: u64,
    pub tWB_max: u32,
    pub tWC_min: u32,
    pub tWH_min: u32,
    pub tWHR_min: u32,
    pub tWP_min: u32,
    pub tWW_min: u32,
}

//
// struct nand_nvddr_timings - NV-DDR NAND chip timings
//
// This struct defines the timing requirements of a NV-DDR NAND data interface.
// These information can be found in every NAND datasheets and the timings
// meaning are described in the ONFI specifications:
// https://media-www.micron.com/-/media/client/onfi/specs/onfi_4_1_gold.pdf
// (chapter 4.18.2 NV-DDR)
//
// All these timings are expressed in picoseconds.
//
// @tBERS_max: Block erase time
// @tCCS_min: Change column setup time
// @tPROG_max: Page program time
// @tR_max: Page read time
// @tAC_min: Access window of DQ[7:0] from CLK
// @tAC_max: Access window of DQ[7:0] from CLK
// @tADL_min: ALE to data loading time
// @tCAD_min: Command, Address, Data delay
// @tCAH_min: Command/Address DQ hold time
// @tCALH_min: W/R_n, CLE and ALE hold time
// @tCALS_min: W/R_n, CLE and ALE setup time
// @tCAS_min: Command/address DQ setup time
// @tCEH_min: CE# high hold time
// @tCH_min:  CE# hold time
// @tCK_min: Average clock cycle time
// @tCS_min: CE# setup time
// @tDH_min: Data hold time
// @tDQSCK_min: Start of the access window of DQS from CLK
// @tDQSCK_max: End of the access window of DQS from CLK
// @tDQSD_min: Min W/R_n low to DQS/DQ driven by device
// @tDQSD_max: Max W/R_n low to DQS/DQ driven by device
// @tDQSHZ_max: W/R_n high to DQS/DQ tri-state by device
// @tDQSQ_max: DQS-DQ skew, DQS to last DQ valid, per access
// @tDS_min: Data setup time
// @tDSC_min: DQS cycle time
// @tFEAT_max: Busy time for Set Features and Get Features
// @tITC_max: Interface and Timing Mode Change time
// @tQHS_max: Data hold skew factor
// @tRHW_min: Data output cycle to command, address, or data input cycle
// @tRR_min: Ready to RE# low (data only)
// @tRST_max: Device reset time, measured from the falling edge of R/B# to the
// rising edge of R/B#.
// @tWB_max: WE# high to SR[6] low
// @tWHR_min: WE# high to RE# low
// @tWRCK_min: W/R_n low to data output cycle
// @tWW_min: WP# transition to WE# low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_nvddr_timings {
    pub tBERS_max: u64,
    pub tCCS_min: u32,
    pub tPROG_max: u64,
    pub tR_max: u64,
    pub tAC_min: u32,
    pub tAC_max: u32,
    pub tADL_min: u32,
    pub tCAD_min: u32,
    pub tCAH_min: u32,
    pub tCALH_min: u32,
    pub tCALS_min: u32,
    pub tCAS_min: u32,
    pub tCEH_min: u32,
    pub tCH_min: u32,
    pub tCK_min: u32,
    pub tCS_min: u32,
    pub tDH_min: u32,
    pub tDQSCK_min: u32,
    pub tDQSCK_max: u32,
    pub tDQSD_min: u32,
    pub tDQSD_max: u32,
    pub tDQSHZ_max: u32,
    pub tDQSQ_max: u32,
    pub tDS_min: u32,
    pub tDSC_min: u32,
    pub tFEAT_max: u32,
    pub tITC_max: u32,
    pub tQHS_max: u32,
    pub tRHW_min: u32,
    pub tRR_min: u32,
    pub tRST_max: u32,
    pub tWB_max: u32,
    pub tWHR_min: u32,
    pub tWRCK_min: u32,
    pub tWW_min: u32,
}

//
// While timings related to the data interface itself are mostly different
// between SDR and NV-DDR, timings related to the internal chip behavior are
// common. IOW, the following entries which describe the internal delays have
// the same definition and are shared in both SDR and NV-DDR timing structures:
// - tADL_min
// - tBERS_max
// - tCCS_min
// - tFEAT_max
// - tPROG_max
// - tR_max
// - tRR_min
// - tRST_max
// - tWB_max
//
// The below macros return the value of a given timing, no matter the interface.
//

//
// enum nand_interface_type - NAND interface type
// @NAND_SDR_IFACE:	Single Data Rate interface
// @NAND_NVDDR_IFACE:	Double Data Rate interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_interface_type {
    NAND_SDR_IFACE,
    NAND_NVDDR_IFACE,
}

//
// struct nand_interface_config - NAND interface timing
// @type:	 type of the timing
// @timings:	 The timing information
// @timings.mode: Timing mode as defined in the specification
// @timings.sdr: Use it when @type is %NAND_SDR_IFACE.
// @timings.nvddr: Use it when @type is %NAND_NVDDR_IFACE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_interface_config {
    pub type: nand_interface_type,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_timings {
    pub mode: c_uint,
    pub sdr: nand_sdr_timings,
    pub nvddr: nand_nvddr_timings,
}

//
// nand_interface_is_sdr - get the interface type
// @conf:	The data interface
//
// nand_interface_is_nvddr - get the interface type
// @conf:	The data interface
//
// nand_get_sdr_timings - get SDR timing from data interface
// @conf:	The data interface
//
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
//
// nand_get_nvddr_timings - get NV-DDR timing from data interface
// @conf:	The data interface
//
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
//
// struct nand_op_cmd_instr - Definition of a command instruction
// @opcode: the command to issue in one cycle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_cmd_instr {
    pub opcode: u8,
}

//
// struct nand_op_addr_instr - Definition of an address instruction
// @naddrs: length of the @addrs array
// @addrs: array containing the address cycles to issue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_addr_instr {
    pub naddrs: c_uint,
    pub addrs: *const u8,
}

//
// struct nand_op_data_instr - Definition of a data instruction
// @len: number of data bytes to move
// @buf: buffer to fill
// @buf.in: buffer to fill when reading from the NAND chip
// @buf.out: buffer to read from when writing to the NAND chip
// @force_8bit: force 8-bit access
//
// Please note that "in" and "out" are inverted from the ONFI specification
// and are from the controller perspective, so a "in" is a read from the NAND
// chip while a "out" is a write to the NAND chip.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_data_instr {
    pub len: c_uint,
    pub in: *mut c_void,
    pub out: *const c_void,
    pub buf: },
    pub force_8bit: bool,
}

//
// struct nand_op_waitrdy_instr - Definition of a wait ready instruction
// @timeout_ms: maximum delay while waiting for the ready/busy pin in ms
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_waitrdy_instr {
    pub timeout_ms: c_uint,
}

//
// enum nand_op_instr_type - Definition of all instruction types
// @NAND_OP_CMD_INSTR: command instruction
// @NAND_OP_ADDR_INSTR: address instruction
// @NAND_OP_DATA_IN_INSTR: data in instruction
// @NAND_OP_DATA_OUT_INSTR: data out instruction
// @NAND_OP_WAITRDY_INSTR: wait ready instruction
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nand_op_instr_type {
    NAND_OP_CMD_INSTR,
    NAND_OP_ADDR_INSTR,
    NAND_OP_DATA_IN_INSTR,
    NAND_OP_DATA_OUT_INSTR,
    NAND_OP_WAITRDY_INSTR,
}

//
// struct nand_op_instr - Instruction object
// @type: the instruction type
// @ctx:  extra data associated to the instruction. You'll have to use the
// appropriate element depending on @type
// @ctx.cmd: use it if @type is %NAND_OP_CMD_INSTR
// @ctx.addr: use it if @type is %NAND_OP_ADDR_INSTR
// @ctx.data: use it if @type is %NAND_OP_DATA_IN_INSTR
// or %NAND_OP_DATA_OUT_INSTR
// @ctx.waitrdy: use it if @type is %NAND_OP_WAITRDY_INSTR
// @delay_ns: delay the controller should apply after the instruction has been
// issued on the bus. Most modern controllers have internal timings
// control logic, and in this case, the controller driver can ignore
// this field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_instr {
    pub type: nand_op_instr_type,
    pub cmd: nand_op_cmd_instr,
    pub addr: nand_op_addr_instr,
    pub data: nand_op_data_instr,
    pub waitrdy: nand_op_waitrdy_instr,
    pub ctx: },
    pub delay_ns: c_uint,
}

//
// Special handling must be done for the WAITRDY timeout parameter as it usually
// is either tPROG (after a prog), tR (before a read), tRST (during a reset) or
// tBERS (during an erase) which all of them are u64 values that cannot be
// divided by usual kernel macros and must be handled with the special
// DIV_ROUND_UP_ULL() macro.
//
// Cast to type of dividend is needed here to guarantee that the result won't
// be an unsigned long long when the dividend is an unsigned long (or smaller),
// which is what the compiler does when it sees ternary operator with 2
// different return types (picks the largest type to make sure there's no
// loss).
//

//
// struct nand_subop - a sub operation
// @cs: the CS line to select for this NAND sub-operation
// @instrs: array of instructions
// @ninstrs: length of the @instrs array
// @first_instr_start_off: offset to start from for the first instruction
// of the sub-operation
// @last_instr_end_off: offset to end at (excluded) for the last instruction
// of the sub-operation
//
// Both @first_instr_start_off and @last_instr_end_off only apply to data or
// address instructions.
//
// When an operation cannot be handled as is by the NAND controller, it will
// be split by the parser into sub-operations which will be passed to the
// controller driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_subop {
    pub cs: c_uint,
    pub instrs: *const nand_op_instr,
    pub ninstrs: c_uint,
    pub first_instr_start_off: c_uint,
    pub last_instr_end_off: c_uint,
}

//
// struct nand_op_parser_addr_constraints - Constraints for address instructions
// @maxcycles: maximum number of address cycles the controller can issue in a
// single step
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_parser_addr_constraints {
    pub maxcycles: c_uint,
}

//
// struct nand_op_parser_data_constraints - Constraints for data instructions
// @maxlen: maximum data length that the controller can handle in a single step
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_parser_data_constraints {
    pub maxlen: c_uint,
}

//
// struct nand_op_parser_pattern_elem - One element of a pattern
// @type: the instructuction type
// @optional: whether this element of the pattern is optional or mandatory
// @ctx: address or data constraint
// @ctx.addr: address constraint (number of cycles)
// @ctx.data: data constraint (data length)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_parser_pattern_elem {
    pub type: nand_op_instr_type,
    pub optional: bool,
    pub addr: nand_op_parser_addr_constraints,
    pub data: nand_op_parser_data_constraints,
    pub ctx: },
}

//
// struct nand_op_parser_pattern - NAND sub-operation pattern descriptor
// @elems: array of pattern elements
// @nelems: number of pattern elements in @elems array
// @exec: the function that will issue a sub-operation
//
// A pattern is a list of elements, each element reprensenting one instruction
// with its constraints. The pattern itself is used by the core to match NAND
// chip operation with NAND controller operations.
// Once a match between a NAND controller operation pattern and a NAND chip
// operation (or a sub-set of a NAND operation) is found, the pattern ->exec()
// hook is called so that the controller driver can issue the operation on the
// bus.
//
// Controller drivers should declare as many patterns as they support and pass
// this list of patterns (created with the help of the following macro) to
// the nand_op_parser_exec_op() helper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_parser_pattern {
    pub elems: *const nand_op_parser_pattern_elem,
    pub nelems: c_uint,
    pub subop): *const *const *const int (exec)(struct nand_chip chip, struct nand_subop,
}

//
// struct nand_op_parser - NAND controller operation parser descriptor
// @patterns: array of supported patterns
// @npatterns: length of the @patterns array
//
// The parser descriptor is just an array of supported patterns which will be
// iterated by nand_op_parser_exec_op() everytime it tries to execute an
// NAND operation (or tries to determine if a specific operation is supported).
//
// It is worth mentioning that patterns will be tested in their declaration
// order, and the first match will be taken, so it's important to order patterns
// appropriately so that simple/inefficient patterns are placed at the end of
// the list. Usually, this is where you put single instruction patterns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_op_parser {
    pub patterns: *const nand_op_parser_pattern,
    pub npatterns: c_uint,
}

//
// struct nand_operation - NAND operation descriptor
// @cs: the CS line to select for this NAND operation
// @deassert_wp: set to true when the operation requires the WP pin to be
// de-asserted (ERASE, PROG, ...)
// @instrs: array of instructions to execute
// @ninstrs: length of the @instrs array
//
// The actual operation structure that will be passed to chip->exec_op().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_operation {
    pub cs: c_uint,
    pub deassert_wp: bool,
    pub instrs: *const nand_op_instr,
    pub ninstrs: c_uint,
}

//
// struct nand_controller_ops - Controller operations
//
// @attach_chip: this method is called after the NAND detection phase after
// flash ID and MTD fields such as erase size, page size and OOB
// size have been set up. ECC requirements are available if
// provided by the NAND chip or device tree. Typically used to
// choose the appropriate ECC configuration and allocate
// associated resources.
// This hook is optional.
// @detach_chip: free all resources allocated/claimed in
// nand_controller_ops->attach_chip().
// This hook is optional.
// @exec_op:	 controller specific method to execute NAND operations.
// This method replaces chip->legacy.cmdfunc(),
// chip->legacy.{read,write}_{buf,byte,word}(),
// chip->legacy.dev_ready() and chip->legacy.waitfunc().
// @setup_interface: setup the data interface and timing. If chipnr is set to
// %NAND_DATA_IFACE_CHECK_ONLY this means the configuration
// should not be applied but only checked.
// This hook is optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_controller_ops {
    pub chip): *mut *mut int (attach_chip)(struct nand_chip,
    pub chip): *mut *mut void (detach_chip)(struct nand_chip,
    pub check_only): bool,
    pub conf): *const nand_interface_config,
}

//
// struct nand_controller - Structure used to describe a NAND controller
//
// @lock:		lock used to serialize accesses to the NAND controller
// @ops:		NAND controller operations.
// @supported_op:	NAND controller known-to-be-supported operations,
// only writable by the core after initial checking.
// @supported_op.data_only_read: The controller supports reading more data from
// the bus without restarting an entire read operation nor
// changing the column.
// @supported_op.cont_read: The controller supports sequential cache reads.
// @controller_wp:	the controller is in charge of handling the WP pin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_controller {
    pub lock: mutex,
    pub ops: *const nand_controller_ops,
    pub 1: unsigned int data_only_read:,
    pub 1: unsigned int cont_read:,
    pub supported_op: },
    pub controller_wp: bool,
}

//
// struct nand_legacy - NAND chip legacy fields/hooks
// @IO_ADDR_R: address to read the 8 I/O lines of the flash device
// @IO_ADDR_W: address to write the 8 I/O lines of the flash device
// @select_chip: select/deselect a specific target/die
// @read_byte: read one byte from the chip
// @write_byte: write a single byte to the chip on the low 8 I/O lines
// @write_buf: write data from the buffer to the chip
// @read_buf: read data from the chip into the buffer
// @cmd_ctrl: hardware specific function for controlling ALE/CLE/nCE. Also used
// to write command and address
// @cmdfunc: hardware specific function for writing commands to the chip.
// @dev_ready: hardware specific function for accessing device ready/busy line.
// If set to NULL no access to ready/busy is available and the
// ready/busy information is read from the chip status register.
// @waitfunc: hardware specific function for wait on ready.
// @block_bad: check if a block is bad, using OOB markers
// @block_markbad: mark a block bad
// @set_features: set the NAND chip features
// @get_features: get the NAND chip features
// @chip_delay: chip dependent delay for transferring data from array to read
// regs (tR).
// @dummy_controller: dummy controller implementation for drivers that can
// only control a single chip
//
// If you look at this structure you're already wrong. These fields/hooks are
// all deprecated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_legacy {
    pub IO_ADDR_R: *mut void __iomem,
    pub IO_ADDR_W: *mut void __iomem,
    pub cs): *mut *mut *mut void (select_chip)(struct nand_chip chip, int,
    pub chip): *mut *mut u8 (read_byte)(struct nand_chip,
    pub byte): *mut *mut *mut void (write_byte)(struct nand_chip chip, u8,
    pub len): *const *const *const *const void (write_buf)(struct nand_chip chip, u8 buf, int,
    pub len): *mut *mut *mut *mut void (read_buf)(struct nand_chip chip, u8 buf, int,
    pub ctrl): *mut *mut *mut void (cmd_ctrl)(struct nand_chip chip, int dat, unsigned int,
    pub page_addr): c_int,
    pub chip): *mut *mut int (dev_ready)(struct nand_chip,
    pub chip): *mut *mut int (waitfunc)(struct nand_chip,
    pub ofs): *mut *mut *mut int (block_bad)(struct nand_chip chip, loff_t,
    pub ofs): *mut *mut *mut int (block_markbad)(struct nand_chip chip, loff_t,
    pub subfeature_para): *mut u8,
    pub subfeature_para): *mut u8,
    pub chip_delay: c_int,
    pub dummy_controller: nand_controller,
}

//
// struct nand_chip_ops - NAND chip operations
// @suspend: Suspend operation
// @resume: Resume operation
// @lock_area: Lock operation
// @unlock_area: Unlock operation
// @setup_read_retry: Set the read-retry mode (mostly needed for MLC NANDs)
// @choose_interface_config: Choose the best interface configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_chip_ops {
    pub chip): *mut *mut int (suspend)(struct nand_chip,
    pub chip): *mut *mut void (resume)(struct nand_chip,
    pub len): *mut *mut *mut int (lock_area)(struct nand_chip chip, loff_t ofs, uint64_t,
    pub len): *mut *mut *mut int (unlock_area)(struct nand_chip chip, loff_t ofs, uint64_t,
    pub retry_mode): *mut *mut *mut int (setup_read_retry)(struct nand_chip chip, int,
    pub iface): *mut nand_interface_config,
}

//
// struct nand_manufacturer - NAND manufacturer structure
// @desc: The manufacturer description
// @priv: Private information for the manufacturer driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_manufacturer {
    pub desc: *const nand_manufacturer_desc,
    pub priv: *mut c_void,
}

//
// struct nand_secure_region - NAND secure region structure
// @offset: Offset of the start of the secure region
// @size: Size of the secure region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_secure_region {
    pub offset: u64,
    pub size: u64,
}

//
// struct nand_chip - NAND Private Flash Chip Data
// @base: Inherit from the generic NAND device
// @id: Holds NAND ID
// @parameters: Holds generic parameters under an easily readable form
// @manufacturer: Manufacturer information
// @ops: NAND chip operations
// @legacy: All legacy fields/hooks. If you develop a new driver, don't even try
// to use any of these fields/hooks, and if you're modifying an
// existing driver that is using those fields/hooks, you should
// consider reworking the driver and avoid using them.
// @options: Various chip options. They can partly be set to inform nand_scan
// about special functionality. See the defines for further
// explanation.
// @current_interface_config: The currently used NAND interface configuration
// @best_interface_config: The best NAND interface configuration which fits both
// the NAND chip and NAND controller constraints. If
// unset, the default reset interface configuration must
// be used.
// @bbt_erase_shift: Number of address bits in a bbt entry
// @bbt_options: Bad block table specific options. All options used here must
// come from bbm.h. By default, these options will be copied to
// the appropriate nand_bbt_descr's.
// @badblockpos: Bad block marker position in the oob area
// @badblockbits: Minimum number of set bits in a good block's bad block marker
// position; i.e., BBM = 11110111b is good when badblockbits = 7
// @bbt_td: Bad block table descriptor for flash lookup
// @bbt_md: Bad block table mirror descriptor
// @badblock_pattern: Bad block scan pattern used for initial bad block scan
// @bbt: Bad block table pointer
// @page_shift: Number of address bits in a page (column address bits)
// @phys_erase_shift: Number of address bits in a physical eraseblock
// @chip_shift: Number of address bits in one chip
// @pagemask: Page number mask = number of (pages / chip) - 1
// @subpagesize: Holds the subpagesize
// @data_buf: Buffer for data, size is (page size + oobsize)
// @oob_poi: pointer on the OOB area covered by data_buf
// @pagecache: Structure containing page cache related fields
// @pagecache.bitflips: Number of bitflips of the cached page
// @pagecache.page: Page number currently in the cache. -1 means no page is
// currently cached
// @buf_align: Minimum buffer alignment required by a platform
// @lock: Lock protecting the suspended field. Also used to serialize accesses
// to the NAND device
// @suspended: Set to 1 when the device is suspended, 0 when it's not
// @resume_wq: wait queue to sleep if rawnand is in suspended state.
// @cur_cs: Currently selected target. -1 means no target selected, otherwise we
// should always have cur_cs >= 0 && cur_cs < nanddev_ntargets().
// NAND Controller drivers should not modify this value, but they're
// allowed to read it.
// @read_retries: The number of read retry modes supported
// @secure_regions: Structure containing the secure regions info
// @nr_secure_regions: Number of secure regions
// @cont_read: Sequential page read internals
// @cont_read.ongoing: Whether a continuous read is ongoing or not
// @cont_read.first_page: Start of the continuous read operation
// @cont_read.pause_page: End of the current sequential cache read operation
// @cont_read.last_page: End of the continuous read operation
// @controller: The hardware controller	structure which is shared among multiple
// independent devices
// @ecc: The ECC controller structure
// @priv: Chip private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_chip {
    pub base: nand_device,
    pub id: nand_id,
    pub parameters: nand_parameters,
    pub manufacturer: nand_manufacturer,
    pub ops: nand_chip_ops,
    pub legacy: nand_legacy,
    pub options: c_uint,
// Data interface
    pub current_interface_config: *const nand_interface_config,
    pub best_interface_config: *mut nand_interface_config,
// Bad block information
    pub bbt_erase_shift: c_uint,
    pub bbt_options: c_uint,
    pub badblockpos: c_uint,
    pub badblockbits: c_uint,
    pub bbt_td: *mut nand_bbt_descr,
    pub bbt_md: *mut nand_bbt_descr,
    pub badblock_pattern: *mut nand_bbt_descr,
    pub bbt: *mut u8,
// Device internal layout
    pub page_shift: c_uint,
    pub phys_erase_shift: c_uint,
    pub chip_shift: c_uint,
    pub pagemask: c_uint,
    pub subpagesize: c_uint,
// Buffers
    pub data_buf: *mut u8,
    pub oob_poi: *mut u8,
    pub bitflips: c_uint,
    pub page: c_int,
    pub pagecache: },
    pub buf_align: c_ulong,
// Internals
    pub lock: mutex,
    pub 1: unsigned int suspended :,
    pub resume_wq: wait_queue_head_t,
    pub cur_cs: c_int,
    pub read_retries: c_int,
    pub secure_regions: *mut nand_secure_region,
    pub nr_secure_regions: u8,
    pub ongoing: bool,
    pub first_page: c_uint,
    pub pause_page: c_uint,
    pub last_page: c_uint,
    pub cont_read: },
// Externals
    pub controller: *mut nand_controller,
    pub ecc: nand_ecc_ctrl,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: mtd, nand_chip: struct, _arg: base.mtd) -> return;
}
extern "C" {
    pub fn mtd_get_of_node(_arg: nand_to_mtd(chip)) -> return;
}
//
// nand_get_interface_config - Retrieve the current interface configuration
// of a NAND chip
// @chip: The NAND chip
//
// A helper for defining older NAND chips where the second ID byte fully
// defined the chip, including the geometry (chip size, eraseblock size, page
// size). All these chips have 512 bytes NAND page size.
//

//
// A helper for defining newer chips which report their page size and
// eraseblock size via the extended ID bytes.
//
// The real difference between LEGACY_ID_NAND and EXTENDED_ID_NAND is that with
// EXTENDED_ID_NAND, manufacturers overloaded the same device ID so that the
// device ID now only represented a particular total chip size (and voltage,
// buswidth), and the page size, eraseblock size, and OOB size could vary while
// using the same device ID.
//

//
// struct nand_flash_dev - NAND Flash Device ID Structure
// @name: a human-readable name of the NAND chip
// @dev_id: the device ID (the second byte of the full chip ID array)
// @mfr_id: manufacturer ID part of the full chip ID array (refers the same
// memory address as ``id[0]``)
// @dev_id: device ID part of the full chip ID array (refers the same memory
// address as ``id[1]``)
// @id: full device ID array
// @pagesize: size of the NAND page in bytes; if 0, then the real page size (as
// well as the eraseblock size) is determined from the extended NAND
// chip ID array)
// @chipsize: total chip size in MiB
// @erasesize: eraseblock size in bytes (determined from the extended ID if 0)
// @options: stores various chip bit options
// @id_len: The valid length of the @id.
// @oobsize: OOB size
// @ecc: ECC correctability and step information from the datasheet.
// @ecc.strength_ds: The ECC correctability from the datasheet, same as the
// @ecc_strength_ds in nand_chip{}.
// @ecc.step_ds: The ECC step required by the @ecc.strength_ds, same as the
// @ecc_step_ds in nand_chip{}, also from the datasheet.
// For example, the "4bit ECC for each 512Byte" can be set with
// NAND_ECC_INFO(4, 512).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_flash_dev {
    pub name: *mut c_char,
    pub mfr_id: u8,
    pub dev_id: u8,
}

extern "C" {
    pub fn nand_create_bbt(chip: *mut nand_chip) -> c_int;
}
//
// Check if it is a SLC nand.
// The !nand_is_slc() can be used to check the MLC/TLC nand chips.
// We do not distinguish the MLC and TLC now.
//
// nand_opcode_8bits - Check if the opcode's address should be sent only on the
// lower 8 bits
// @command: opcode to check
//
extern "C" {
    pub fn rawnand_sw_hamming_init(chip: *mut nand_chip) -> c_int;
}
extern "C" {
    pub fn rawnand_sw_hamming_cleanup(chip: *mut nand_chip);
}
extern "C" {
    pub fn rawnand_sw_bch_init(chip: *mut nand_chip) -> c_int;
}
extern "C" {
    pub fn rawnand_sw_bch_cleanup(chip: *mut nand_chip);
}
// Default write_oob implementation
extern "C" {
    pub fn nand_write_oob_std(chip: *mut nand_chip, page: c_int) -> c_int;
}
// Default read_oob implementation
extern "C" {
    pub fn nand_read_oob_std(chip: *mut nand_chip, page: c_int) -> c_int;
}
// Stub used by drivers that do not support GET/SET FEATURES operations
// read_page_raw implementations
// write_page_raw implementations
// Reset and initialize a NAND device
extern "C" {
    pub fn nand_reset(chip: *mut nand_chip, chipnr: c_int) -> c_int;
}
// NAND operation helpers
extern "C" {
    pub fn nand_reset_op(chip: *mut nand_chip) -> c_int;
}
extern "C" {
    pub fn nand_status_op(chip: *mut nand_chip, status: *mut u8) -> c_int;
}
extern "C" {
    pub fn nand_exit_status_op(chip: *mut nand_chip) -> c_int;
}
extern "C" {
    pub fn nand_erase_op(chip: *mut nand_chip, eraseblock: c_uint) -> c_int;
}
extern "C" {
    pub fn nand_prog_page_end_op(chip: *mut nand_chip) -> c_int;
}
// Scan and identify a NAND device
extern "C" {
    pub fn nand_scan_with_ids(_arg: chip, _arg: max_chips, _arg: NULL) -> return;
}
// Internal helper for board drivers which need to override command function
extern "C" {
    pub fn nand_wait_ready(chip: *mut nand_chip);
}
//
// Free resources held by the NAND device, must be called on error after a
// sucessful nand_scan().
//
extern "C" {
    pub fn nand_cleanup(chip: *mut nand_chip);
}
//
// External helper for controller drivers that have to implement the WAITRDY
// instruction and have no physical pin to check it.
//
extern "C" {
    pub fn nand_soft_waitrdy(chip: *mut nand_chip, timeout_ms: c_ulong) -> c_int;
}
// Select/deselect a NAND target.
extern "C" {
    pub fn nand_select_target(chip: *mut nand_chip, cs: c_uint);
}
extern "C" {
    pub fn nand_deselect_target(chip: *mut nand_chip);
}
// Bitops
//
// nand_get_data_buf() - Get the internal page buffer
// @chip: NAND chip object
//
// Returns the pre-allocated page buffer after invalidating the cache. This
// function should be used by drivers that do not want to allocate their own
// bounce buffer and still need such a buffer for specific operations (most
// commonly when reading OOB data only).
//
// Be careful to never call this function in the write/write_oob path, because
// the core may have placed the data to be written out in this buffer.
//
// Return: pointer to the page cache buffer
//
// Parse the gpio-cs property
