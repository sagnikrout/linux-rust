//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/geni-se.h
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
// Copyright (c) 2017-2018, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// enum geni_se_xfer_mode: Transfer modes supported by Serial Engines
//
// @GENI_SE_INVALID: Invalid mode
// @GENI_SE_FIFO: FIFO mode. Data is transferred with SE FIFO
// by programmed IO method
// @GENI_SE_DMA: Serial Engine DMA mode. Data is transferred
// with SE by DMAengine internal to SE
// @GENI_GPI_DMA: GPI DMA mode. Data is transferred using a DMAengine
// configured by a firmware residing on a GSI engine. This DMA name is
// interchangeably used as GSI or GPI which seem to imply the same DMAengine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum geni_se_xfer_mode {
    GENI_SE_INVALID,
    GENI_SE_FIFO,
    GENI_SE_DMA,
    GENI_GPI_DMA,
}

// Protocols supported by GENI Serial Engines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum geni_se_protocol_type {
    GENI_SE_NONE,
    GENI_SE_SPI,
    GENI_SE_UART,
    GENI_SE_I2C,
    GENI_SE_I3C,
    GENI_SE_SPI_SLAVE,
    GENI_SE_INVALID_PROTO = 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum geni_icc_path_index {
    GENI_TO_CORE,
    CPU_TO_GENI,
    GENI_TO_DDR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geni_icc_path {
    pub path: *mut icc_path,
    pub avg_bw: c_uint,
}

//
// struct geni_se - GENI Serial Engine
// @base:		Base Address of the Serial Engine's register block
// @dev:		Pointer to the Serial Engine device
// @wrapper:		Pointer to the parent QUP Wrapper core
// @clk:		Handle to the core serial engine clock
// @core_clk:		Auxiliary clock, which may be required by a protocol
// @num_clk_levels:	Number of valid clock levels in clk_perf_tbl
// @clk_perf_tbl:	Table of clock frequency input to serial engine clock
// @icc_paths:		Array of ICC paths for SE
// @pd_list:		Power domain list for managing power domains
// @has_opp:		Indicates if OPP is supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geni_se {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub wrapper: *mut geni_wrapper,
    pub clk: *mut clk,
    pub core_clk: *mut clk,
    pub num_clk_levels: c_uint,
    pub clk_perf_tbl: *mut c_ulong,
    pub icc_paths: [geni_icc_path; 3],
    pub pd_list: *mut dev_pm_domain_list,
    pub has_opp: bool,
}

// Common SE registers
pub const GENI_GENERAL_CFG: c_uint = 0x10;
pub const GENI_FORCE_DEFAULT_REG: c_uint = 0x20;
pub const GENI_OUTPUT_CTRL: c_uint = 0x24;
pub const SE_GENI_STATUS: c_uint = 0x40;
pub const GENI_SER_M_CLK_CFG: c_uint = 0x48;
pub const GENI_SER_S_CLK_CFG: c_uint = 0x4c;
pub const GENI_CLK_CTRL_RO: c_uint = 0x60;
pub const GENI_IF_DISABLE_RO: c_uint = 0x64;
pub const GENI_FW_REVISION_RO: c_uint = 0x68;
pub const GENI_FW_MULTILOCK_MSA_RO: c_uint = 0x74;
pub const SE_GENI_CLK_SEL: c_uint = 0x7c;
pub const SE_GENI_CFG_SEQ_START: c_uint = 0x84;
pub const SE_GENI_DMA_MODE_EN: c_uint = 0x258;
pub const SE_GENI_M_CMD0: c_uint = 0x600;
pub const SE_GENI_M_CMD_CTRL_REG: c_uint = 0x604;
pub const SE_GENI_M_IRQ_STATUS: c_uint = 0x610;
pub const SE_GENI_M_IRQ_EN: c_uint = 0x614;
pub const SE_GENI_M_IRQ_CLEAR: c_uint = 0x618;
pub const SE_GENI_M_IRQ_EN_SET: c_uint = 0x61c;
pub const SE_GENI_M_IRQ_EN_CLEAR: c_uint = 0x620;
pub const M_CMD_ERR_STATUS: c_uint = 0x624;
pub const M_FW_ERR_STATUS: c_uint = 0x628;
pub const SE_GENI_S_CMD0: c_uint = 0x630;
pub const SE_GENI_S_CMD_CTRL_REG: c_uint = 0x634;
pub const SE_GENI_S_IRQ_STATUS: c_uint = 0x640;
pub const SE_GENI_S_IRQ_EN: c_uint = 0x644;
pub const SE_GENI_S_IRQ_CLEAR: c_uint = 0x648;
pub const SE_GENI_S_IRQ_EN_SET: c_uint = 0x64c;
pub const SE_GENI_S_IRQ_EN_CLEAR: c_uint = 0x650;
pub const SE_GENI_TX_FIFOn: c_uint = 0x700;
pub const SE_GENI_RX_FIFOn: c_uint = 0x780;
pub const SE_GENI_TX_FIFO_STATUS: c_uint = 0x800;
pub const SE_GENI_RX_FIFO_STATUS: c_uint = 0x804;
pub const SE_GENI_TX_WATERMARK_REG: c_uint = 0x80c;
pub const SE_GENI_RX_WATERMARK_REG: c_uint = 0x810;
pub const SE_GENI_RX_RFR_WATERMARK_REG: c_uint = 0x814;
pub const SE_GENI_IOS: c_uint = 0x908;
pub const SE_GENI_M_GP_LENGTH: c_uint = 0x910;
pub const SE_GENI_S_GP_LENGTH: c_uint = 0x914;
// TX DMA registers
pub const SE_DMA_TX_PTR_L: c_uint = 0xc30;
pub const SE_DMA_TX_PTR_H: c_uint = 0xc34;
pub const SE_DMA_TX_ATTR: c_uint = 0xc38;
pub const SE_DMA_TX_LEN: c_uint = 0xc3c;
pub const SE_DMA_TX_IRQ_STAT: c_uint = 0xc40;
pub const SE_DMA_TX_IRQ_CLR: c_uint = 0xc44;
pub const SE_DMA_TX_IRQ_EN: c_uint = 0xc48;
pub const SE_DMA_TX_IRQ_EN_SET: c_uint = 0xc4c;
pub const SE_DMA_TX_IRQ_EN_CLR: c_uint = 0xc50;
pub const SE_DMA_TX_LEN_IN: c_uint = 0xc54;
pub const SE_DMA_TX_FSM_RST: c_uint = 0xc58;
pub const SE_DMA_TX_MAX_BURST: c_uint = 0xc5c;
// RX DMA registers
pub const SE_DMA_RX_PTR_L: c_uint = 0xd30;
pub const SE_DMA_RX_PTR_H: c_uint = 0xd34;
pub const SE_DMA_RX_ATTR: c_uint = 0xd38;
pub const SE_DMA_RX_LEN: c_uint = 0xd3c;
pub const SE_DMA_RX_IRQ_STAT: c_uint = 0xd40;
pub const SE_DMA_RX_IRQ_CLR: c_uint = 0xd44;
pub const SE_DMA_RX_IRQ_EN: c_uint = 0xd48;
pub const SE_DMA_RX_IRQ_EN_SET: c_uint = 0xd4c;
pub const SE_DMA_RX_IRQ_EN_CLR: c_uint = 0xd50;
pub const SE_DMA_RX_LEN_IN: c_uint = 0xd54;
pub const SE_DMA_RX_FSM_RST: c_uint = 0xd58;
pub const SE_DMA_RX_MAX_BURST: c_uint = 0xd5c;
// DMA general / debug registers
pub const SE_GSI_EVENT_EN: c_uint = 0xe18;
pub const SE_IRQ_EN: c_uint = 0xe1c;
pub const DMA_IF_EN_RO: c_uint = 0xe20;
pub const SE_HW_PARAM_0: c_uint = 0xe24;
pub const SE_HW_PARAM_1: c_uint = 0xe28;
pub const SE_HW_PARAM_2: c_uint = 0xe2c;
pub const DMA_GENERAL_CFG: c_uint = 0xe30;
pub const SE_DMA_QSB_TRANS_CFG: c_uint = 0xe38;
pub const SE_DMA_DEBUG_REG0: c_uint = 0xe40;
pub const SE_DMA_IF_EN: c_uint = 0x2004;
// GENI_FORCE_DEFAULT_REG fields

// GENI_OUTPUT_CTRL fields

// GENI_STATUS fields

// GENI_SER_M_CLK_CFG/GENI_SER_S_CLK_CFG

pub const CLK_DIV_SHFT: c_int = 4;
// GENI_IF_DISABLE_RO fields

// GENI_FW_REVISION_RO fields

pub const FW_REV_PROTOCOL_SHFT: c_int = 8;
// GENI_CLK_SEL fields

// SE_GENI_CFG_SEQ_START fields

// SE_GENI_DMA_MODE_EN

// GENI_M_CMD0 fields

pub const M_OPCODE_SHFT: c_int = 27;

// GENI_M_CMD_CTRL_REG

// GENI_S_CMD0 fields

pub const S_OPCODE_SHFT: c_int = 27;

// GENI_S_CMD_CTRL_REG

// GENI_M_IRQ_EN fields

// GENI_S_IRQ_EN fields

// GENI_/TX/RX/RX_RFR/_WATERMARK_REG fields

// GENI_TX_FIFO_STATUS fields

// GENI_RX_FIFO_STATUS fields

pub const RX_LAST_BYTE_VALID_SHFT: c_int = 28;

// SE_GENI_IOS fields

// SE_GENI_M_GP_LENGTH and SE_GENI_S_GP_LENGTH fields

// SE_DMA_TX_IRQ_STAT Register fields

// SE_DMA_RX_IRQ_STAT Register fields

// SE_DMA_DEBUG_REG0 fields

// SE_HW_PARAM_0 fields

pub const TX_FIFO_WIDTH_SHFT: c_int = 24;
//
// For QUP HW Version >= 3.10 Tx fifo depth support is increased
// to 256bytes and corresponding bits are 16 to 23
//

pub const TX_FIFO_DEPTH_SHFT: c_int = 16;
// SE_HW_PARAM_1 fields

pub const RX_FIFO_WIDTH_SHFT: c_int = 24;
//
// For QUP HW Version >= 3.10 Rx fifo depth support is increased
// to 256bytes and corresponding bits are 16 to 23
//

pub const RX_FIFO_DEPTH_SHFT: c_int = 16;
// SE_HW_PARAM_2 fields

pub const HW_VER_MAJOR_SHFT: c_int = 28;

pub const HW_VER_MINOR_SHFT: c_int = 16;

// QUP SE VERSION value for major number 2 and minor number 5
pub const QUP_SE_VERSION_2_5: c_uint = 0x20050000;
//
// Define bandwidth thresholds that cause the underlying Core 2X interconnect
// clock to run at the named frequency. These baseline values are recommended
// by the hardware team, and are not dynamically scaled with GENI bandwidth
// beyond basic on/off.
//
pub const CORE_2X_19_2_MHZ: c_int = 960;
pub const CORE_2X_50_MHZ: c_int = 2500;
pub const CORE_2X_100_MHZ: c_int = 5000;
pub const CORE_2X_150_MHZ: c_int = 7500;
pub const CORE_2X_200_MHZ: c_int = 10000;
pub const CORE_2X_236_MHZ: c_int = 16383;

extern "C" {
    pub fn geni_se_get_qup_hw_version(se: *mut geni_se) -> u32;
}
//
// geni_se_read_proto() - Read the protocol configured for a serial engine
// @se:		Pointer to the concerned serial engine.
//
// Return: Protocol value as configured in the serial engine.
//
// geni_se_setup_m_cmd() - Setup the primary sequencer
// @se:		Pointer to the concerned serial engine.
// @cmd:	Command/Operation to setup in the primary sequencer.
// @params:	Parameter for the sequencer command.
//
// This function is used to configure the primary sequencer with the
// command and its associated parameters.
//
// geni_se_setup_s_cmd() - Setup the secondary sequencer
// @se:		Pointer to the concerned serial engine.
// @cmd:	Command/Operation to setup in the secondary sequencer.
// @params:	Parameter for the sequencer command.
//
// This function is used to configure the secondary sequencer with the
// command and its associated parameters.
//
// geni_se_cancel_m_cmd() - Cancel the command configured in the primary
// sequencer
// @se:	Pointer to the concerned serial engine.
//
// This function is used to cancel the currently configured command in the
// primary sequencer.
//
// geni_se_cancel_s_cmd() - Cancel the command configured in the secondary
// sequencer
// @se:	Pointer to the concerned serial engine.
//
// This function is used to cancel the currently configured command in the
// secondary sequencer.
//
// geni_se_abort_m_cmd() - Abort the command configured in the primary sequencer
// @se:	Pointer to the concerned serial engine.
//
// This function is used to force abort the currently configured command in the
// primary sequencer.
//
// geni_se_abort_s_cmd() - Abort the command configured in the secondary
// sequencer
// @se:	Pointer to the concerned serial engine.
//
// This function is used to force abort the currently configured command in the
// secondary sequencer.
//
// geni_se_get_tx_fifo_depth() - Get the TX fifo depth of the serial engine
// based on QUP HW version
// @se: Pointer to the concerned serial engine.
//
// This function is used to get the depth i.e. number of elements in the
// TX fifo of the serial engine.
//
// Return: TX fifo depth in units of FIFO words.
//
// geni_se_get_tx_fifo_width() - Get the TX fifo width of the serial engine
// @se:	Pointer to the concerned serial engine.
//
// This function is used to get the width i.e. word size per element in the
// TX fifo of the serial engine.
//
// Return: TX fifo width in bits
//
// geni_se_get_rx_fifo_depth() - Get the RX fifo depth of the serial engine
// based on QUP HW version
// @se: Pointer to the concerned serial engine.
//
// This function is used to get the depth i.e. number of elements in the
// RX fifo of the serial engine.
//
// Return: RX fifo depth in units of FIFO words
//
extern "C" {
    pub fn geni_se_init(se: *mut geni_se, rx_wm: u32, rx_rfr: u32);
}
extern "C" {
    pub fn geni_se_select_mode(se: *mut geni_se, mode: geni_se_xfer_mode);
}
extern "C" {
    pub fn geni_se_resources_off(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_resources_on(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_clk_tbl_get(se: *mut geni_se, tbl: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn geni_se_tx_init_dma(se: *mut geni_se, iova: dma_addr_t, len: usize);
}
extern "C" {
    pub fn geni_se_rx_init_dma(se: *mut geni_se, iova: dma_addr_t, len: usize);
}
extern "C" {
    pub fn geni_se_tx_dma_unprep(se: *mut geni_se, iova: dma_addr_t, len: usize);
}
extern "C" {
    pub fn geni_se_rx_dma_unprep(se: *mut geni_se, iova: dma_addr_t, len: usize);
}
extern "C" {
    pub fn geni_icc_get(se: *mut geni_se, icc_ddr: *const c_char) -> c_int;
}
extern "C" {
    pub fn geni_icc_set_bw(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_icc_set_bw_ab(se: *mut geni_se, core_ab: u32, cfg_ab: u32, ddr_ab: u32) -> c_int;
}
extern "C" {
    pub fn geni_icc_set_tag(se: *mut geni_se, tag: u32);
}
extern "C" {
    pub fn geni_icc_enable(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_icc_disable(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_resources_init(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_resources_activate(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_resources_deactivate(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_load_se_firmware(se: *mut geni_se, protocol: geni_se_protocol_type) -> c_int;
}
extern "C" {
    pub fn geni_se_domain_attach(se: *mut geni_se) -> c_int;
}
extern "C" {
    pub fn geni_se_set_perf_level(se: *mut geni_se, level: c_ulong) -> c_int;
}
extern "C" {
    pub fn geni_se_set_perf_opp(se: *mut geni_se, clk_freq: c_ulong) -> c_int;
}

