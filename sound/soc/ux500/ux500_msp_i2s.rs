//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ux500/ux500_msp_i2s.h
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
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>,
// for ST-Ericsson.
//

pub const MSP_INPUT_FREQ_APB: c_int = 48000000;
// Stereo mode. Used for APB data accesses as 16 bits accesses (mono),
// 32 bits accesses (stereo).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_stereo_mode {
    MSP_MONO,
    MSP_STEREO
}

// Direction (Transmit/Receive mode)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_direction {
    MSP_TX = 1,
    MSP_RX = 2
}

// Transmit and receive configuration register
pub const MSP_BIG_ENDIAN: c_uint = 0x00000000;
pub const MSP_LITTLE_ENDIAN: c_uint = 0x00001000;
pub const MSP_UNEXPECTED_FS_ABORT: c_uint = 0x00000000;
pub const MSP_UNEXPECTED_FS_IGNORE: c_uint = 0x00008000;
pub const MSP_NON_MODE_BIT_MASK: c_uint = 0x00009000;
// Global configuration register
pub const RX_ENABLE: c_uint = 0x00000001;
pub const RX_FIFO_ENABLE: c_uint = 0x00000002;
pub const RX_SYNC_SRG: c_uint = 0x00000010;
pub const RX_CLK_POL_RISING: c_uint = 0x00000020;
pub const RX_CLK_SEL_SRG: c_uint = 0x00000040;
pub const TX_ENABLE: c_uint = 0x00000100;
pub const TX_FIFO_ENABLE: c_uint = 0x00000200;
pub const TX_SYNC_SRG_PROG: c_uint = 0x00001800;
pub const TX_SYNC_SRG_AUTO: c_uint = 0x00001000;
pub const TX_CLK_POL_RISING: c_uint = 0x00002000;
pub const TX_CLK_SEL_SRG: c_uint = 0x00004000;
pub const TX_EXTRA_DELAY_ENABLE: c_uint = 0x00008000;
pub const SRG_ENABLE: c_uint = 0x00010000;
pub const FRAME_GEN_ENABLE: c_uint = 0x00100000;
pub const SRG_CLK_SEL_APB: c_uint = 0x00000000;
pub const RX_FIFO_SYNC_HI: c_uint = 0x00000000;
pub const TX_FIFO_SYNC_HI: c_uint = 0x00000000;
pub const SPI_CLK_MODE_NORMAL: c_uint = 0x00000000;

pub const MSP_DR: c_uint = 0x00;
pub const MSP_GCR: c_uint = 0x04;
pub const MSP_TCF: c_uint = 0x08;
pub const MSP_RCF: c_uint = 0x0c;
pub const MSP_SRG: c_uint = 0x10;
pub const MSP_FLR: c_uint = 0x14;
pub const MSP_DMACR: c_uint = 0x18;
pub const MSP_IMSC: c_uint = 0x20;
pub const MSP_RIS: c_uint = 0x24;
pub const MSP_MIS: c_uint = 0x28;
pub const MSP_ICR: c_uint = 0x2c;
pub const MSP_MCR: c_uint = 0x30;
pub const MSP_RCV: c_uint = 0x34;
pub const MSP_RCM: c_uint = 0x38;
pub const MSP_TCE0: c_uint = 0x40;
pub const MSP_TCE1: c_uint = 0x44;
pub const MSP_TCE2: c_uint = 0x48;
pub const MSP_TCE3: c_uint = 0x4c;
pub const MSP_RCE0: c_uint = 0x60;
pub const MSP_RCE1: c_uint = 0x64;
pub const MSP_RCE2: c_uint = 0x68;
pub const MSP_RCE3: c_uint = 0x6c;
pub const MSP_IODLY: c_uint = 0x70;
pub const MSP_ITCR: c_uint = 0x80;
pub const MSP_ITIP: c_uint = 0x84;
pub const MSP_ITOP: c_uint = 0x88;
pub const MSP_TSTDR: c_uint = 0x8c;
pub const MSP_PID0: c_uint = 0xfe0;
pub const MSP_PID1: c_uint = 0xfe4;
pub const MSP_PID2: c_uint = 0xfe8;
pub const MSP_PID3: c_uint = 0xfec;
pub const MSP_CID0: c_uint = 0xff0;
pub const MSP_CID1: c_uint = 0xff4;
pub const MSP_CID2: c_uint = 0xff8;
pub const MSP_CID3: c_uint = 0xffc;
// Protocol dependant parameters list

pub const RXEN_SHIFT: c_int = 0;
pub const RFFEN_SHIFT: c_int = 1;
pub const RFSPOL_SHIFT: c_int = 2;
pub const DCM_SHIFT: c_int = 3;
pub const RFSSEL_SHIFT: c_int = 4;
pub const RCKPOL_SHIFT: c_int = 5;
pub const RCKSEL_SHIFT: c_int = 6;
pub const LBM_SHIFT: c_int = 7;
pub const TXEN_SHIFT: c_int = 8;
pub const TFFEN_SHIFT: c_int = 9;
pub const TFSPOL_SHIFT: c_int = 10;
pub const TFSSEL_SHIFT: c_int = 11;
pub const TCKPOL_SHIFT: c_int = 13;
pub const TCKSEL_SHIFT: c_int = 14;
pub const TXDDL_SHIFT: c_int = 15;
pub const SGEN_SHIFT: c_int = 16;
pub const SCKPOL_SHIFT: c_int = 17;
pub const SCKSEL_SHIFT: c_int = 18;
pub const FGEN_SHIFT: c_int = 20;
pub const SPICKM_SHIFT: c_int = 21;
pub const TBSWAP_SHIFT: c_int = 28;

pub const P1ELEN_SHIFT: c_int = 0;
pub const P1FLEN_SHIFT: c_int = 3;
pub const DTYP_SHIFT: c_int = 10;
pub const ENDN_SHIFT: c_int = 12;
pub const DDLY_SHIFT: c_int = 13;
pub const FSIG_SHIFT: c_int = 15;
pub const P2ELEN_SHIFT: c_int = 16;
pub const P2FLEN_SHIFT: c_int = 19;
pub const P2SM_SHIFT: c_int = 26;
pub const P2EN_SHIFT: c_int = 27;
pub const FSYNC_SHIFT: c_int = 15;
pub const P1ELEN_MASK: c_uint = 0x00000007;
pub const P2ELEN_MASK: c_uint = 0x00070000;
pub const P1FLEN_MASK: c_uint = 0x00000378;
pub const P2FLEN_MASK: c_uint = 0x03780000;
pub const DDLY_MASK: c_uint = 0x00003000;
pub const DTYP_MASK: c_uint = 0x00000600;
pub const P2SM_MASK: c_uint = 0x04000000;
pub const P2EN_MASK: c_uint = 0x08000000;
pub const ENDN_MASK: c_uint = 0x00001000;
pub const TFSPOL_MASK: c_uint = 0x00000400;
pub const TBSWAP_MASK: c_uint = 0x30000000;
pub const COMPANDING_MODE_MASK: c_uint = 0x00000c00;
pub const FSYNC_MASK: c_uint = 0x00008000;

// Flag register

pub const RBUSY_SHIFT: c_int = 0;
pub const RFE_SHIFT: c_int = 1;
pub const RFU_SHIFT: c_int = 2;
pub const TBUSY_SHIFT: c_int = 3;
pub const TFE_SHIFT: c_int = 4;
pub const TFU_SHIFT: c_int = 5;
// Multichannel control register
pub const RMCEN_SHIFT: c_int = 0;
pub const RMCSF_SHIFT: c_int = 1;
pub const RCMPM_SHIFT: c_int = 3;
pub const TMCEN_SHIFT: c_int = 5;
pub const TNCSF_SHIFT: c_int = 6;
// Sample rate generator register
pub const SCKDIV_SHIFT: c_int = 0;
pub const FRWID_SHIFT: c_int = 10;
pub const FRPER_SHIFT: c_int = 16;
pub const SCK_DIV_MASK: c_uint = 0x0000003FF;

// DMA controller register

pub const RDMAE_SHIFT: c_int = 0;
pub const TDMAE_SHIFT: c_int = 1;
// Interrupt Register

pub const ALL_INT: c_uint = 0x000000ff;
// MSP test control register

pub const RMCEN_BIT: c_int = 0;
pub const RMCSF_BIT: c_int = 1;
pub const RCMPM_BIT: c_int = 3;
pub const TMCEN_BIT: c_int = 5;
pub const TNCSF_BIT: c_int = 6;
// Single or dual phase mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_phase_mode {
    MSP_SINGLE_PHASE,
    MSP_DUAL_PHASE
}

// Frame length
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_frame_length {
    MSP_FRAME_LEN_1 = 0,
    MSP_FRAME_LEN_2 = 1,
    MSP_FRAME_LEN_4 = 3,
    MSP_FRAME_LEN_8 = 7,
    MSP_FRAME_LEN_12 = 11,
    MSP_FRAME_LEN_16 = 15,
    MSP_FRAME_LEN_20 = 19,
    MSP_FRAME_LEN_32 = 31,
    MSP_FRAME_LEN_48 = 47,
    MSP_FRAME_LEN_64 = 63
}

// Element length
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_elem_length {
    MSP_ELEM_LEN_8 = 0,
    MSP_ELEM_LEN_10 = 1,
    MSP_ELEM_LEN_12 = 2,
    MSP_ELEM_LEN_14 = 3,
    MSP_ELEM_LEN_16 = 4,
    MSP_ELEM_LEN_20 = 5,
    MSP_ELEM_LEN_24 = 6,
    MSP_ELEM_LEN_32 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_data_xfer_width {
    MSP_DATA_TRANSFER_WIDTH_BYTE,
    MSP_DATA_TRANSFER_WIDTH_HALFWORD,
    MSP_DATA_TRANSFER_WIDTH_WORD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_frame_sync {
    MSP_FSYNC_UNIGNORE = 0,
    MSP_FSYNC_IGNORE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_phase2_start_mode {
    MSP_PHASE2_START_MODE_IMEDIATE,
    MSP_PHASE2_START_MODE_FSYNC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_btf {
    MSP_BTF_MS_BIT_FIRST = 0,
    MSP_BTF_LS_BIT_FIRST = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_fsync_pol {
    MSP_FSYNC_POL_ACT_HI = 0,
    MSP_FSYNC_POL_ACT_LO = 1
}

// Data delay (in bit clock cycles)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_delay {
    MSP_DELAY_0 = 0,
    MSP_DELAY_1 = 1,
    MSP_DELAY_2 = 2,
    MSP_DELAY_3 = 3
}

// Configurations of clocks (transmit, receive or sample rate generator)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_edge {
    MSP_FALLING_EDGE = 0,
    MSP_RISING_EDGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_hws {
    MSP_SWAP_NONE = 0,
    MSP_SWAP_BYTE_PER_WORD = 1,
    MSP_SWAP_BYTE_PER_HALF_WORD = 2,
    MSP_SWAP_HALF_WORD_PER_WORD = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_compress_mode {
    MSP_COMPRESS_MODE_LINEAR = 0,
    MSP_COMPRESS_MODE_MU_LAW = 2,
    MSP_COMPRESS_MODE_A_LAW = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_expand_mode {
    MSP_EXPAND_MODE_LINEAR = 0,
    MSP_EXPAND_MODE_LINEAR_SIGNED = 1,
    MSP_EXPAND_MODE_MU_LAW = 2,
    MSP_EXPAND_MODE_A_LAW = 3
}

pub const MSP_FRAME_PERIOD_IN_MONO_MODE: c_int = 256;
pub const MSP_FRAME_PERIOD_IN_STEREO_MODE: c_int = 32;
pub const MSP_FRAME_WIDTH_IN_STEREO_MODE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_protocol {
    MSP_I2S_PROTOCOL,
    MSP_PCM_PROTOCOL,
    MSP_PCM_COMPAND_PROTOCOL,
    MSP_INVALID_PROTOCOL
}

//
// No of registers to backup during
// suspend resume
//
pub const MAX_MSP_BACKUP_REGS: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2s_direction_t {
    MSP_DIR_TX = 0x01,
    MSP_DIR_RX = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_data_size {
    MSP_DATA_BITS_DEFAULT = -1,
    MSP_DATA_BITS_8 = 0x00,
    MSP_DATA_BITS_10,
    MSP_DATA_BITS_12,
    MSP_DATA_BITS_14,
    MSP_DATA_BITS_16,
    MSP_DATA_BITS_20,
    MSP_DATA_BITS_24,
    MSP_DATA_BITS_32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_state {
    MSP_STATE_IDLE = 0,
    MSP_STATE_CONFIGURED = 1,
    MSP_STATE_RUNNING = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp_rx_comparison_enable_mode {
    MSP_COMPARISON_DISABLED = 0,
    MSP_COMPARISON_NONEQUAL_ENABLED = 2,
    MSP_COMPARISON_EQUAL_ENABLED = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msp_multichannel_config {
    pub rx_multichannel_enable: bool,
    pub tx_multichannel_enable: bool,
    pub rx_comparison_enable_mode: msp_rx_comparison_enable_mode,
    pub padding: u8,
    pub comparison_value: u32,
    pub comparison_mask: u32,
    pub rx_channel_0_enable: u32,
    pub rx_channel_1_enable: u32,
    pub rx_channel_2_enable: u32,
    pub rx_channel_3_enable: u32,
    pub tx_channel_0_enable: u32,
    pub tx_channel_1_enable: u32,
    pub tx_channel_2_enable: u32,
    pub tx_channel_3_enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msp_protdesc {
    pub rx_phase_mode: u32,
    pub tx_phase_mode: u32,
    pub rx_phase2_start_mode: u32,
    pub tx_phase2_start_mode: u32,
    pub rx_byte_order: u32,
    pub tx_byte_order: u32,
    pub rx_frame_len_1: u32,
    pub rx_frame_len_2: u32,
    pub tx_frame_len_1: u32,
    pub tx_frame_len_2: u32,
    pub rx_elem_len_1: u32,
    pub rx_elem_len_2: u32,
    pub tx_elem_len_1: u32,
    pub tx_elem_len_2: u32,
    pub rx_data_delay: u32,
    pub tx_data_delay: u32,
    pub rx_clk_pol: u32,
    pub tx_clk_pol: u32,
    pub rx_fsync_pol: u32,
    pub tx_fsync_pol: u32,
    pub rx_half_word_swap: u32,
    pub tx_half_word_swap: u32,
    pub compression_mode: u32,
    pub expansion_mode: u32,
    pub frame_sync_ignore: u32,
    pub frame_period: u32,
    pub frame_width: u32,
    pub clocks_per_frame: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_msp_config {
    pub f_inputclk: c_uint,
    pub rx_clk_sel: c_uint,
    pub tx_clk_sel: c_uint,
    pub srg_clk_sel: c_uint,
    pub rx_fsync_pol: c_uint,
    pub tx_fsync_pol: c_uint,
    pub rx_fsync_sel: c_uint,
    pub tx_fsync_sel: c_uint,
    pub rx_fifo_config: c_uint,
    pub tx_fifo_config: c_uint,
    pub loopback_enable: c_uint,
    pub tx_data_enable: c_uint,
    pub default_protdesc: c_uint,
    pub protdesc: msp_protdesc,
    pub multichannel_configured: c_int,
    pub multichannel_config: msp_multichannel_config,
    pub direction: c_uint,
    pub protocol: c_uint,
    pub frame_freq: c_uint,
    pub data_size: msp_data_size,
    pub def_elem_len: c_uint,
    pub iodelay: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_msp {
    pub id: c_int,
    pub registers: *mut void __iomem,
    pub dev: *mut device,
    pub tx_rx_addr: dma_addr_t,
    pub msp_state: msp_state,
    pub def_elem_len: c_int,
    pub dir_busy: c_uint,
    pub loopback_enable: c_int,
    pub f_bitclk: c_uint,
}

extern "C" {
    pub fn ux500_msp_i2s_open(msp: *mut ux500_msp, config: *mut ux500_msp_config) -> c_int;
}
