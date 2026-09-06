//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/nand-qpic-common.h
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
// QCOM QPIC common APIs header file
//
// Copyright (c) 2023 Qualcomm Inc.
// Authors:	Md sadre Alam	<quic_mdalam@quicinc.com>
//

// NANDc reg offsets
pub const NAND_FLASH_CMD: c_uint = 0x00;
pub const NAND_ADDR0: c_uint = 0x04;
pub const NAND_ADDR1: c_uint = 0x08;
pub const NAND_FLASH_CHIP_SELECT: c_uint = 0x0c;
pub const NAND_EXEC_CMD: c_uint = 0x10;
pub const NAND_FLASH_STATUS: c_uint = 0x14;
pub const NAND_BUFFER_STATUS: c_uint = 0x18;
pub const NAND_DEV0_CFG0: c_uint = 0x20;
pub const NAND_DEV0_CFG1: c_uint = 0x24;
pub const NAND_DEV0_ECC_CFG: c_uint = 0x28;
pub const NAND_AUTO_STATUS_EN: c_uint = 0x2c;
pub const NAND_DEV1_CFG0: c_uint = 0x30;
pub const NAND_DEV1_CFG1: c_uint = 0x34;
pub const NAND_READ_ID: c_uint = 0x40;
pub const NAND_READ_STATUS: c_uint = 0x44;
pub const NAND_DEV_CMD0: c_uint = 0xa0;
pub const NAND_DEV_CMD1: c_uint = 0xa4;
pub const NAND_DEV_CMD2: c_uint = 0xa8;
pub const NAND_DEV_CMD_VLD: c_uint = 0xac;
pub const SFLASHC_BURST_CFG: c_uint = 0xe0;
pub const NAND_ERASED_CW_DETECT_CFG: c_uint = 0xe8;
pub const NAND_ERASED_CW_DETECT_STATUS: c_uint = 0xec;
pub const NAND_EBI2_ECC_BUF_CFG: c_uint = 0xf0;
pub const FLASH_BUF_ACC: c_uint = 0x100;
pub const NAND_CTRL: c_uint = 0xf00;
pub const NAND_VERSION: c_uint = 0xf08;
pub const NAND_READ_LOCATION_0: c_uint = 0xf20;
pub const NAND_READ_LOCATION_1: c_uint = 0xf24;
pub const NAND_READ_LOCATION_2: c_uint = 0xf28;
pub const NAND_READ_LOCATION_3: c_uint = 0xf2c;
pub const NAND_READ_LOCATION_LAST_CW_0: c_uint = 0xf40;
pub const NAND_READ_LOCATION_LAST_CW_1: c_uint = 0xf44;
pub const NAND_READ_LOCATION_LAST_CW_2: c_uint = 0xf48;
pub const NAND_READ_LOCATION_LAST_CW_3: c_uint = 0xf4c;
// dummy register offsets, used by qcom_write_reg_dma
pub const NAND_DEV_CMD1_RESTORE: c_uint = 0xdead;
pub const NAND_DEV_CMD_VLD_RESTORE: c_uint = 0xbeef;
// NAND_FLASH_CMD bits

// NAND_FLASH_CHIP_SELECT bits
pub const NAND_DEV_SEL: c_int = 0;

// NAND_FLASH_STATUS bits

// NAND_BUFFER_STATUS bits

pub const BS_CORRECTABLE_ERR_MSK: c_uint = 0x1f;
// NAND_DEVn_CFG0 bits

// NAND_DEVn_CFG0 bits

// NAND_DEV0_ECC_CFG bits

pub const ECC_MODE_4BIT: c_int = 0;
pub const ECC_MODE_8BIT: c_int = 1;

// NAND_DEV_CMD1 bits

// NAND_DEV_CMD_VLD bits

// NAND_EBI2_ECC_BUF_CFG bits

// NAND_ERASED_CW_DETECT_CFG bits
pub const ERASED_CW_ECC_MASK: c_int = 1;
pub const AUTO_DETECT_RES: c_int = 0;

// NAND_ERASED_CW_DETECT_STATUS bits

// NAND_READ_LOCATION_n bits

// Version Mask
pub const NAND_VERSION_MAJOR_MASK: c_uint = 0xf0000000;
pub const NAND_VERSION_MAJOR_SHIFT: c_int = 28;
pub const NAND_VERSION_MINOR_MASK: c_uint = 0x0fff0000;
pub const NAND_VERSION_MINOR_SHIFT: c_int = 16;
// NAND OP_CMDs
pub const OP_PAGE_READ: c_uint = 0x2;
pub const OP_PAGE_READ_WITH_ECC: c_uint = 0x3;
pub const OP_PAGE_READ_WITH_ECC_SPARE: c_uint = 0x4;
pub const OP_PAGE_READ_ONFI_READ: c_uint = 0x5;
pub const OP_PROGRAM_PAGE: c_uint = 0x6;
pub const OP_PAGE_PROGRAM_WITH_ECC: c_uint = 0x7;
pub const OP_PROGRAM_PAGE_SPARE: c_uint = 0x9;
pub const OP_BLOCK_ERASE: c_uint = 0xa;
pub const OP_CHECK_STATUS: c_uint = 0xc;
pub const OP_FETCH_ID: c_uint = 0xb;
pub const OP_RESET_DEVICE: c_uint = 0xd;
// Default Value for NAND_DEV_CMD_VLD

// NAND_CTRL bits

//
// the NAND controller performs reads/writes with ECC in 516 byte chunks.
// the driver calls the chunks 'step' or 'codeword' interchangeably
//
pub const NANDC_STEP_SIZE: c_int = 512;
//
// the largest page size we support is 8K, this will have 16 steps/codewords
// of 512 bytes each
//

// we read at most 3 registers per codeword scan

// ECC modes supported by the controller

//
// Returns the actual register address for all NAND_DEV_ registers
// (i.e. NAND_DEV_CMD0, NAND_DEV_CMD1, NAND_DEV_CMD2 and NAND_DEV_CMD_VLD)
//

// Returns the dma address for reg read buffer

pub const QPIC_PER_CW_CMD_ELEMENTS: c_int = 32;
pub const QPIC_PER_CW_CMD_SGL: c_int = 32;
pub const QPIC_PER_CW_DATA_SGL: c_int = 8;

//
// Flags used in DMA descriptor preparation helper functions
// (i.e. qcom_read_reg_dma/qcom_write_reg_dma/qcom_read_data_dma/qcom_write_data_dma)
//
// Don't set the EOT in current tx BAM sgl

// Set the NWD flag in current BAM sgl

// Finish writing in the current BAM sgl and start writing in another BAM sgl

//
// Erased codeword status is being used two times in single transfer so this
// flag will determine the current value of erased codeword status register
//

pub const MAX_ADDRESS_CYCLE: c_int = 5;
//
// This data type corresponds to the BAM transaction which will be used for all
// NAND transfers.
// @bam_ce - the array of BAM command elements
// @cmd_sgl - sgl for NAND BAM command pipe
// @data_sgl - sgl for NAND BAM consumer/producer pipe
// @last_data_desc - last DMA desc in data channel (tx/rx).
// @last_cmd_desc - last DMA desc in command channel.
// @txn_done - completion for NAND transfer.
// @bam_ce_nitems - the number of elements in the @bam_ce array
// @cmd_sgl_nitems - the number of elements in the @cmd_sgl array
// @data_sgl_nitems - the number of elements in the @data_sgl array
// @bam_ce_pos - the index in bam_ce which is available for next sgl
// @bam_ce_start - the index in bam_ce which marks the start position ce
// for current sgl. It will be used for size calculation
// for current sgl
// @cmd_sgl_pos - current index in command sgl.
// @cmd_sgl_start - start index in command sgl.
// @tx_sgl_pos - current index in data sgl for tx.
// @tx_sgl_start - start index in data sgl for tx.
// @rx_sgl_pos - current index in data sgl for rx.
// @rx_sgl_start - start index in data sgl for rx.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bam_transaction {
    pub bam_ce: *mut bam_cmd_element,
    pub cmd_sgl: *mut scatterlist,
    pub data_sgl: *mut scatterlist,
    pub last_data_desc: *mut dma_async_tx_descriptor,
    pub last_cmd_desc: *mut dma_async_tx_descriptor,
    pub txn_done: completion,
    pub bam_ce_nitems: c_uint,
    pub cmd_sgl_nitems: c_uint,
    pub data_sgl_nitems: c_uint,
    pub bam_ce_pos: u32,
    pub bam_ce_start: u32,
    pub cmd_sgl_pos: u32,
    pub cmd_sgl_start: u32,
    pub tx_sgl_pos: u32,
    pub tx_sgl_start: u32,
    pub rx_sgl_pos: u32,
    pub rx_sgl_start: u32,
}

//
// This data type corresponds to the nand dma descriptor
// @dma_desc - low level DMA engine descriptor
// @list - list for desc_info
//
// @adm_sgl - sgl which will be used for single sgl dma descriptor. Only used by
// ADM
// @bam_sgl - sgl which will be used for dma descriptor. Only used by BAM
// @sgl_cnt - number of SGL in bam_sgl. Only used by BAM
// @dir - DMA transfer direction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_info {
    pub dma_desc: *mut dma_async_tx_descriptor,
    pub node: list_head,
    pub adm_sgl: scatterlist,
    pub bam_sgl: *mut scatterlist,
    pub sgl_cnt: c_int,
}

//
// holds the current register values that we want to write. acts as a contiguous
// chunk of memory which we use to write the controller registers through DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nandc_regs {
    pub cmd: __le32,
    pub addr0: __le32,
    pub addr1: __le32,
    pub chip_sel: __le32,
    pub exec: __le32,
    pub cfg0: __le32,
    pub cfg1: __le32,
    pub ecc_bch_cfg: __le32,
    pub clrflashstatus: __le32,
    pub clrreadstatus: __le32,
    pub cmd1: __le32,
    pub vld: __le32,
    pub orig_cmd1: __le32,
    pub orig_vld: __le32,
    pub ecc_buf_cfg: __le32,
    pub read_location0: __le32,
    pub read_location1: __le32,
    pub read_location2: __le32,
    pub read_location3: __le32,
    pub read_location_last0: __le32,
    pub read_location_last1: __le32,
    pub read_location_last2: __le32,
    pub read_location_last3: __le32,
    pub spi_cfg: __le32,
    pub num_addr_cycle: __le32,
    pub busy_wait_cnt: __le32,
    pub flash_feature: __le32,
    pub erased_cw_detect_cfg_clr: __le32,
    pub erased_cw_detect_cfg_set: __le32,
}

//
// NAND controller data struct
//
// @dev:			parent device
//
// @base:			MMIO base
//
// @core_clk:			controller clock
// @aon_clk:			another controller clock
// @iomacro_clk:		io macro clock
//
// @regs:			a contiguous chunk of memory for DMA register
// writes. contains the register values to be
// written to controller
//
// @props:			properties of current NAND controller,
// initialized via DT match data
//
// @controller:			base controller structure
// @qspi:			qpic spi structure
// @host_list:			list containing all the chips attached to the
// controller
//
// @chan:			dma channel
// @cmd_crci:			ADM DMA CRCI for command flow control
// @data_crci:			ADM DMA CRCI for data flow control
//
// @desc_list:			DMA descriptor list (list of desc_infos)
//
// @data_buffer:		our local DMA buffer for page read/writes,
// used when we can't use the buffer provided
// by upper layers directly
// @reg_read_buf:		local buffer for reading back registers via DMA
//
// @base_phys:			physical base address of controller registers
// @base_dma:			dma base address of controller registers
// @reg_read_dma:		contains dma address for register read buffer
//
// @buf_size/count/start:	markers for chip->legacy.read_buf/write_buf
// functions
// @max_cwperpage:		maximum QPIC codewords required. calculated
// from all connected NAND devices pagesize
//
// @reg_read_pos:		marker for data read in reg_read_buf
//
// @cmd1/vld:			some fixed controller register values
//
// @exec_opwrite:		flag to select correct number of code word
// while reading status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_nand_controller {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub core_clk: *mut clk,
    pub aon_clk: *mut clk,
    pub regs: *mut nandc_regs,
    pub bam_txn: *mut bam_transaction,
    pub props: *const qcom_nandc_props,
    pub controller: nand_controller,
    pub qspi: *mut qpic_spi_nand,
    pub host_list: list_head,
// will be used only by QPIC for BAM DMA
    pub tx_chan: *mut dma_chan,
    pub rx_chan: *mut dma_chan,
    pub cmd_chan: *mut dma_chan,
}

// will be used only by EBI2 for ADM DMA
//
// This data type corresponds to the NAND controller properties which varies
// among different NAND controllers.
// @ecc_modes - ecc mode for NAND
// @dev_cmd_reg_start - NAND_DEV_CMD_* registers starting offset
// @supports_bam - whether NAND controller is using BAM
// @nandc_part_of_qpic - whether NAND controller is part of qpic IP
// @has_onfi_read_op - whether ONFI param page read command is supported
// @qpic_version2 - flag to indicate QPIC IP version 2
// @use_codeword_fixup - whether NAND has different layout for boot partitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_nandc_props {
    pub ecc_modes: u32,
    pub dev_cmd_reg_start: u32,
    pub bam_offset: u32,
    pub supports_bam: bool,
    pub nandc_part_of_qpic: bool,
    pub has_onfi_read_op: bool,
    pub qpic_version2: bool,
    pub use_codeword_fixup: bool,
}

extern "C" {
    pub fn qcom_free_bam_transaction(nandc: *mut qcom_nand_controller);
}
extern "C" {
    pub fn qcom_clear_bam_transaction(nandc: *mut qcom_nand_controller);
}
extern "C" {
    pub fn qcom_qpic_bam_dma_done(data: *mut c_void);
}
extern "C" {
    pub fn qcom_nandc_dev_to_mem(nandc: *mut qcom_nand_controller, is_cpu: bool);
}
extern "C" {
    pub fn qcom_submit_descs(nandc: *mut qcom_nand_controller) -> c_int;
}
extern "C" {
    pub fn qcom_clear_read_regs(nandc: *mut qcom_nand_controller);
}
extern "C" {
    pub fn qcom_nandc_unalloc(nandc: *mut qcom_nand_controller);
}
extern "C" {
    pub fn qcom_nandc_alloc(nandc: *mut qcom_nand_controller) -> c_int;
}
