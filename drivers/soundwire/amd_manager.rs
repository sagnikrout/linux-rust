//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soundwire/amd_manager.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright (C) 2023-24 Advanced Micro Devices, Inc. All rights reserved.
//

pub const SDW_MANAGER_REG_OFFSET: c_uint = 0xc00;
pub const AMD_SDW_DEFAULT_ROWS: c_int = 50;
pub const AMD_SDW_DEFAULT_COLUMNS: c_int = 10;
pub const ACP_PAD_PULLDOWN_CTRL: c_uint = 0x0001448;
pub const ACP_SW_PAD_KEEPER_EN: c_uint = 0x0001454;
pub const ACP_SW0_WAKE_EN: c_uint = 0x0001458;
pub const ACP_EXTERNAL_INTR_CNTL0: c_uint = 0x0001a04;
pub const ACP_EXTERNAL_INTR_STAT0: c_uint = 0x0001a0c;

pub const ACP_SW_EN: c_uint = 0x0003000;
pub const ACP_SW_EN_STATUS: c_uint = 0x0003004;
pub const ACP_SW_FRAMESIZE: c_uint = 0x0003008;
pub const ACP_SW_SSP_COUNTER: c_uint = 0x000300c;
pub const ACP_SW_AUDIO0_TX_EN: c_uint = 0x0003010;
pub const ACP_SW_AUDIO0_TX_EN_STATUS: c_uint = 0x0003014;
pub const ACP_SW_AUDIO0_TX_FRAME_FORMAT: c_uint = 0x0003018;
pub const ACP_SW_AUDIO0_TX_SAMPLEINTERVAL: c_uint = 0x000301c;
pub const ACP_SW_AUDIO0_TX_HCTRL_DP0: c_uint = 0x0003020;
pub const ACP_SW_AUDIO0_TX_HCTRL_DP1: c_uint = 0x0003024;
pub const ACP_SW_AUDIO0_TX_HCTRL_DP2: c_uint = 0x0003028;
pub const ACP_SW_AUDIO0_TX_HCTRL_DP3: c_uint = 0x000302c;
pub const ACP_SW_AUDIO0_TX_OFFSET_DP0: c_uint = 0x0003030;
pub const ACP_SW_AUDIO0_TX_OFFSET_DP1: c_uint = 0x0003034;
pub const ACP_SW_AUDIO0_TX_OFFSET_DP2: c_uint = 0x0003038;
pub const ACP_SW_AUDIO0_TX_OFFSET_DP3: c_uint = 0x000303c;
pub const ACP_SW_AUDIO0_TX_CHANNEL_ENABLE_DP0: c_uint = 0x0003040;
pub const ACP_SW_AUDIO0_TX_CHANNEL_ENABLE_DP1: c_uint = 0x0003044;
pub const ACP_SW_AUDIO0_TX_CHANNEL_ENABLE_DP2: c_uint = 0x0003048;
pub const ACP_SW_AUDIO0_TX_CHANNEL_ENABLE_DP3: c_uint = 0x000304c;
pub const ACP_SW_AUDIO1_TX_EN: c_uint = 0x0003050;
pub const ACP_SW_AUDIO1_TX_EN_STATUS: c_uint = 0x0003054;
pub const ACP_SW_AUDIO1_TX_FRAME_FORMAT: c_uint = 0x0003058;
pub const ACP_SW_AUDIO1_TX_SAMPLEINTERVAL: c_uint = 0x000305c;
pub const ACP_SW_AUDIO1_TX_HCTRL: c_uint = 0x0003060;
pub const ACP_SW_AUDIO1_TX_OFFSET: c_uint = 0x0003064;
pub const ACP_SW_AUDIO1_TX_CHANNEL_ENABLE_DP0: c_uint = 0x0003068;
pub const ACP_SW_AUDIO2_TX_EN: c_uint = 0x000306c;
pub const ACP_SW_AUDIO2_TX_EN_STATUS: c_uint = 0x0003070;
pub const ACP_SW_AUDIO2_TX_FRAME_FORMAT: c_uint = 0x0003074;
pub const ACP_SW_AUDIO2_TX_SAMPLEINTERVAL: c_uint = 0x0003078;
pub const ACP_SW_AUDIO2_TX_HCTRL: c_uint = 0x000307c;
pub const ACP_SW_AUDIO2_TX_OFFSET: c_uint = 0x0003080;
pub const ACP_SW_AUDIO2_TX_CHANNEL_ENABLE_DP0: c_uint = 0x0003084;
pub const ACP_SW_AUDIO0_RX_EN: c_uint = 0x0003088;
pub const ACP_SW_AUDIO0_RX_EN_STATUS: c_uint = 0x000308c;
pub const ACP_SW_AUDIO0_RX_FRAME_FORMAT: c_uint = 0x0003090;
pub const ACP_SW_AUDIO0_RX_SAMPLEINTERVAL: c_uint = 0x0003094;
pub const ACP_SW_AUDIO0_RX_HCTRL_DP0: c_uint = 0x0003098;
pub const ACP_SW_AUDIO0_RX_HCTRL_DP1: c_uint = 0x000309c;
pub const ACP_SW_AUDIO0_RX_HCTRL_DP2: c_uint = 0x0003100;
pub const ACP_SW_AUDIO0_RX_HCTRL_DP3: c_uint = 0x0003104;
pub const ACP_SW_AUDIO0_RX_OFFSET_DP0: c_uint = 0x0003108;
pub const ACP_SW_AUDIO0_RX_OFFSET_DP1: c_uint = 0x000310c;
pub const ACP_SW_AUDIO0_RX_OFFSET_DP2: c_uint = 0x0003110;
pub const ACP_SW_AUDIO0_RX_OFFSET_DP3: c_uint = 0x0003114;
pub const ACP_SW_AUDIO0_RX_CHANNEL_ENABLE_DP0: c_uint = 0x0003118;
pub const ACP_SW_AUDIO0_RX_CHANNEL_ENABLE_DP1: c_uint = 0x000311c;
pub const ACP_SW_AUDIO0_RX_CHANNEL_ENABLE_DP2: c_uint = 0x0003120;
pub const ACP_SW_AUDIO0_RX_CHANNEL_ENABLE_DP3: c_uint = 0x0003124;
pub const ACP_SW_AUDIO1_RX_EN: c_uint = 0x0003128;
pub const ACP_SW_AUDIO1_RX_EN_STATUS: c_uint = 0x000312c;
pub const ACP_SW_AUDIO1_RX_FRAME_FORMAT: c_uint = 0x0003130;
pub const ACP_SW_AUDIO1_RX_SAMPLEINTERVAL: c_uint = 0x0003134;
pub const ACP_SW_AUDIO1_RX_HCTRL: c_uint = 0x0003138;
pub const ACP_SW_AUDIO1_RX_OFFSET: c_uint = 0x000313c;
pub const ACP_SW_AUDIO1_RX_CHANNEL_ENABLE_DP0: c_uint = 0x0003140;
pub const ACP_SW_AUDIO2_RX_EN: c_uint = 0x0003144;
pub const ACP_SW_AUDIO2_RX_EN_STATUS: c_uint = 0x0003148;
pub const ACP_SW_AUDIO2_RX_FRAME_FORMAT: c_uint = 0x000314c;
pub const ACP_SW_AUDIO2_RX_SAMPLEINTERVAL: c_uint = 0x0003150;
pub const ACP_SW_AUDIO2_RX_HCTRL: c_uint = 0x0003154;
pub const ACP_SW_AUDIO2_RX_OFFSET: c_uint = 0x0003158;
pub const ACP_SW_AUDIO2_RX_CHANNEL_ENABLE_DP0: c_uint = 0x000315c;
pub const ACP_SW_BPT_PORT_EN: c_uint = 0x0003160;
pub const ACP_SW_BPT_PORT_EN_STATUS: c_uint = 0x0003164;
pub const ACP_SW_BPT_PORT_FRAME_FORMAT: c_uint = 0x0003168;
pub const ACP_SW_BPT_PORT_SAMPLEINTERVAL: c_uint = 0x000316c;
pub const ACP_SW_BPT_PORT_HCTRL: c_uint = 0x0003170;
pub const ACP_SW_BPT_PORT_OFFSET: c_uint = 0x0003174;
pub const ACP_SW_BPT_PORT_CHANNEL_ENABLE: c_uint = 0x0003178;
pub const ACP_SW_BPT_PORT_FIRST_BYTE_ADDR: c_uint = 0x000317c;
pub const ACP_SW_CLK_RESUME_CTRL: c_uint = 0x0003180;
pub const ACP_SW_CLK_RESUME_DELAY_CNTR: c_uint = 0x0003184;
pub const ACP_SW_BUS_RESET_CTRL: c_uint = 0x0003188;
pub const ACP_SW_PRBS_ERR_STATUS: c_uint = 0x000318c;
pub const ACP_SW_IMM_CMD_UPPER_WORD: c_uint = 0x0003230;
pub const ACP_SW_IMM_CMD_LOWER_QWORD: c_uint = 0x0003234;
pub const ACP_SW_IMM_RESP_UPPER_WORD: c_uint = 0x0003238;
pub const ACP_SW_IMM_RESP_LOWER_QWORD: c_uint = 0x000323c;
pub const ACP_SW_IMM_CMD_STS: c_uint = 0x0003240;
pub const ACP_SW_BRA_BASE_ADDRESS: c_uint = 0x0003244;
pub const ACP_SW_BRA_TRANSFER_SIZE: c_uint = 0x0003248;
pub const ACP_SW_BRA_DMA_BUSY: c_uint = 0x000324c;
pub const ACP_SW_BRA_RESP: c_uint = 0x0003250;
pub const ACP_SW_BRA_RESP_FRAME_ADDR: c_uint = 0x0003254;
pub const ACP_SW_BRA_CURRENT_TRANSFER_SIZE: c_uint = 0x0003258;
pub const ACP_SW_STATE_CHANGE_STATUS_0TO7: c_uint = 0x000325c;
pub const ACP_SW_STATE_CHANGE_STATUS_8TO11: c_uint = 0x0003260;
pub const ACP_SW_STATE_CHANGE_STATUS_MASK_0TO7: c_uint = 0x0003264;
pub const ACP_SW_STATE_CHANGE_STATUS_MASK_8TO11: c_uint = 0x0003268;
pub const ACP_SW_CLK_FREQUENCY_CTRL: c_uint = 0x000326c;
pub const ACP_SW_ERROR_INTR_MASK: c_uint = 0x0003270;
pub const ACP_SW_PHY_TEST_MODE_DATA_OFF: c_uint = 0x0003274;
pub const ACP_DELAY_US: c_int = 10;
pub const AMD_SDW_TIMEOUT: c_int = 1000;
pub const AMD_SDW_DEFAULT_CLK_FREQ: c_int = 12000000;

pub const AMD_SDW_MASTER_SUSPEND_DELAY_MS: c_int = 2000;

pub const AMD_SDW_IMM_RES_VALID: c_int = 1;
pub const AMD_SDW_IMM_CMD_BUSY: c_int = 2;
pub const AMD_SDW_ENABLE: c_int = 1;
pub const AMD_SDW_DISABLE: c_int = 0;
pub const AMD_SDW_BUS_RESET_CLEAR_REQ: c_int = 0;
pub const AMD_SDW_BUS_RESET_REQ: c_int = 1;
pub const AMD_SDW_BUS_RESET_DONE: c_int = 2;
pub const AMD_SDW_BUS_BASE_FREQ: c_int = 24000000;
pub const AMD_SDW0_EXT_INTR_MASK: c_uint = 0x200000;
pub const AMD_SDW1_EXT_INTR_MASK: c_int = 4;
pub const AMD_SDW_IRQ_MASK_0TO7: c_uint = 0x77777777;
pub const AMD_SDW_IRQ_MASK_8TO11: c_uint = 0x000c7777;
pub const AMD_SDW_IRQ_ERROR_MASK: c_uint = 0xff;
pub const AMD_SDW_MAX_FREQ_NUM: c_int = 1;
pub const AMD_ACP63_SDW0_MAX_TX_PORTS: c_int = 3;
pub const AMD_ACP63_SDW0_MAX_RX_PORTS: c_int = 3;
pub const AMD_ACP63_SDW1_MAX_TX_PORTS: c_int = 1;
pub const AMD_ACP63_SDW1_MAX_RX_PORTS: c_int = 1;
pub const AMD_ACP70_SDW_MAX_TX_PORTS: c_int = 3;
pub const AMD_ACP70_SDW_MAX_RX_PORTS: c_int = 3;
pub const AMD_ACP63_SDW0_MAX_DAI: c_int = 6;
pub const AMD_ACP63_SDW1_MAX_DAI: c_int = 2;
pub const AMD_ACP70_SDW_MAX_DAI: c_int = 6;
pub const AMD_SDW_SLAVE_0_ATTACHED: c_int = 5;
pub const AMD_SDW_SSP_COUNTER_VAL: c_int = 3;

pub const AMD_SDW_STAT_MAX_RETRY_COUNT: c_int = 100;
pub const AMD_SDW0_PAD_PULLDOWN_CTRL_ENABLE_MASK: c_uint = 0x7f9f;
pub const AMD_SDW1_PAD_PULLDOWN_CTRL_ENABLE_MASK: c_uint = 0x7ffa;
pub const AMD_SDW0_PAD_PULLDOWN_CTRL_DISABLE_MASK: c_uint = 0x60;
pub const AMD_SDW1_PAD_PULLDOWN_CTRL_DISABLE_MASK: c_int = 5;
pub const AMD_SDW0_PAD_KEEPER_EN_MASK: c_int = 1;
pub const AMD_SDW1_PAD_KEEPER_EN_MASK: c_uint = 0x10;
pub const AMD_SDW0_PAD_KEEPER_DISABLE_MASK: c_uint = 0x1e;
pub const AMD_SDW1_PAD_KEEPER_DISABLE_MASK: c_uint = 0xf;

pub const AMD_SDW_CLK_STOP_DONE: c_int = 1;
pub const AMD_SDW_CLK_RESUME_REQ: c_int = 2;
pub const AMD_SDW_CLK_RESUME_DONE: c_int = 3;

pub const AMD_SDW_DEVICE_STATE: c_uint = 0x1430;

pub const AMD_SDW_DEVICE_STATE_D0: c_int = 0;
pub const AMD_SDW_DEVICE_STATE_D3: c_int = 3;
pub const ACP_PME_EN: c_uint = 0x0001400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_manager_dp_reg {
    pub frame_fmt_reg: u32,
    pub sample_int_reg: u32,
    pub hctrl_dp0_reg: u32,
    pub offset_reg: u32,
    pub lane_ctrl_ch_en_reg: u32,
}

//
// SDW0 Manager instance registers  6 CPU DAI (3 TX & 3 RX Ports)
// whereas SDW1  Manager Instance registers 2 CPU DAI (one TX & one RX port)
// Below is the CPU DAI <->Manager port number mapping
// i.e SDW0 Pin0 -> port number 0 -> AUDIO0 TX
// SDW0 Pin1 -> Port number 1 -> AUDIO1 TX
// SDW0 Pin2 -> Port number 2 -> AUDIO2 TX
// SDW0 Pin3 -> port number 3 -> AUDIO0 RX
// SDW0 Pin4 -> Port number 4 -> AUDIO1 RX
// SDW0 Pin5 -> Port number 5 -> AUDIO2 RX
// Whereas for SDW1 instance
// SDW1 Pin0 -> port number 0 -> AUDIO1 TX
// SDW1 Pin1 -> Port number 1 -> AUDIO1 RX
// Same mapping should be used for programming DMA controller registers in SoundWire DMA driver.
// i.e if AUDIO0 TX channel is selected then we need to use AUDIO0 TX registers for DMA programming
// in SoundWire DMA driver.
//
