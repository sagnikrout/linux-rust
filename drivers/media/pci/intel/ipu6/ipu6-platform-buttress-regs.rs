//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-platform-buttress-regs.h
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
// Copyright (C) 2023--2024 Intel Corporation

// IS_WORKPOINT_REQ
pub const IPU6_BUTTRESS_REG_IS_FREQ_CTL: c_uint = 0x34;
// PS_WORKPOINT_REQ
pub const IPU6_BUTTRESS_REG_PS_FREQ_CTL: c_uint = 0x38;
// should be tuned for real silicon
pub const IPU6_IS_FREQ_CTL_DEFAULT_RATIO: c_uint = 0x08;
pub const IPU6SE_IS_FREQ_CTL_DEFAULT_RATIO: c_uint = 0x0a;
pub const IPU6_PS_FREQ_CTL_DEFAULT_RATIO: c_uint = 0x0d;
pub const IPU6_IS_FREQ_CTL_DEFAULT_QOS_FLOOR_RATIO: c_uint = 0x10;
pub const IPU6_PS_FREQ_CTL_DEFAULT_QOS_FLOOR_RATIO: c_uint = 0x0708;
pub const IPU6_BUTTRESS_PWR_STATE_IS_PWR_SHIFT: c_int = 3;

pub const IPU6_BUTTRESS_PWR_STATE_PS_PWR_SHIFT: c_int = 6;

pub const IPU6_BUTTRESS_PWR_STATE_DN_DONE: c_uint = 0x0;
pub const IPU6_BUTTRESS_PWR_STATE_UP_PROCESS: c_uint = 0x1;
pub const IPU6_BUTTRESS_PWR_STATE_DN_PROCESS: c_uint = 0x2;
pub const IPU6_BUTTRESS_PWR_STATE_UP_DONE: c_uint = 0x3;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_0: c_uint = 0x270;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_1: c_uint = 0x274;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_2: c_uint = 0x278;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_3: c_uint = 0x27c;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_4: c_uint = 0x280;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_5: c_uint = 0x284;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_6: c_uint = 0x288;
pub const IPU6_BUTTRESS_REG_FPGA_SUPPORT_7: c_uint = 0x28c;
pub const BUTTRESS_REG_WDT: c_uint = 0x8;
pub const BUTTRESS_REG_BTRS_CTRL: c_uint = 0xc;

pub const BUTTRESS_REG_FW_RESET_CTL: c_uint = 0x30;

pub const BUTTRESS_REG_IS_FREQ_CTL: c_uint = 0x34;
pub const BUTTRESS_REG_PS_FREQ_CTL: c_uint = 0x38;

pub const BUTTRESS_REG_PWR_STATE: c_uint = 0x5c;
pub const BUTTRESS_PWR_STATE_RESET: c_uint = 0x0;
pub const BUTTRESS_PWR_STATE_PWR_ON_DONE: c_uint = 0x1;
pub const BUTTRESS_PWR_STATE_PWR_RDY: c_uint = 0x3;
pub const BUTTRESS_PWR_STATE_PWR_IDLE: c_uint = 0x4;

pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_IDLE: c_uint = 0x0;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_PLL_CMP: c_uint = 0x1;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_CLKACK: c_uint = 0x2;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_PG_ACK: c_uint = 0x3;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_RST_ASSRT_CYCLES: c_uint = 0x4;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_STOP_CLK_CYCLES1: c_uint = 0x5;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_STOP_CLK_CYCLES2: c_uint = 0x6;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_RST_DEASSRT_CYCLES: c_uint = 0x7;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_FUSE_WR_CMP: c_uint = 0x8;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_BRK_POINT: c_uint = 0x9;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_IS_RDY: c_uint = 0xa;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_HALT_HALTED: c_uint = 0xb;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_RST_DURATION_CNT3: c_uint = 0xc;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_CLKACK_PD: c_uint = 0xd;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_PD_BRK_POINT: c_uint = 0xe;
pub const BUTTRESS_PWR_STATE_IS_PWR_FSM_WAIT_4_PD_PG_ACK0: c_uint = 0xf;

pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_IDLE: c_uint = 0x0;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PU_PLL_IP_RDY: c_uint = 0x1;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_RO_PRE_CNT_EXH: c_uint = 0x2;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PU_VGI_PWRGOOD: c_uint = 0x3;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_RO_POST_CNT_EXH: c_uint = 0x4;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WR_PLL_RATIO: c_uint = 0x5;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PU_PLL_CMP: c_uint = 0x6;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PU_CLKACK: c_uint = 0x7;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_RST_ASSRT_CYCLES: c_uint = 0x8;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_STOP_CLK_CYCLES1: c_uint = 0x9;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_STOP_CLK_CYCLES2: c_uint = 0xa;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_RST_DEASSRT_CYCLES: c_uint = 0xb;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_PU_BRK_PNT: c_uint = 0xc;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_FUSE_ACCPT: c_uint = 0xd;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_PS_PWR_UP: c_uint = 0xf;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_4_HALTED: c_uint = 0x10;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_RESET_CNT3: c_uint = 0x11;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PD_CLKACK: c_uint = 0x12;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_PD_OFF_IND: c_uint = 0x13;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_DVFS_PH4: c_uint = 0x14;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_DVFS_PLL_CMP: c_uint = 0x15;
pub const BUTTRESS_PWR_STATE_PS_PWR_FSM_WAIT_DVFS_CLKACK: c_uint = 0x16;
pub const BUTTRESS_REG_SECURITY_CTL: c_uint = 0x300;
pub const BUTTRESS_REG_SKU: c_uint = 0x314;
pub const BUTTRESS_REG_SECURITY_TOUCH: c_uint = 0x318;
pub const BUTTRESS_REG_CAMERA_MASK: c_uint = 0x84;

pub const BUTTRESS_REG_FW_SOURCE_BASE_LO: c_uint = 0x78;
pub const BUTTRESS_REG_FW_SOURCE_BASE_HI: c_uint = 0x7C;
pub const BUTTRESS_REG_FW_SOURCE_SIZE: c_uint = 0x80;
pub const BUTTRESS_REG_ISR_STATUS: c_uint = 0x90;
pub const BUTTRESS_REG_ISR_ENABLED_STATUS: c_uint = 0x94;
pub const BUTTRESS_REG_ISR_ENABLE: c_uint = 0x98;
pub const BUTTRESS_REG_ISR_CLEAR: c_uint = 0x9C;

pub const BUTTRESS_REG_IU2CSEDB0: c_uint = 0x100;

pub const BUTTRESS_IU2CSEDB0_IPC_CLIENT_ID_VAL: c_int = 2;
pub const BUTTRESS_REG_IU2CSEDATA0: c_uint = 0x104;
pub const BUTTRESS_IU2CSEDATA0_IPC_BOOT_LOAD: c_int = 1;
pub const BUTTRESS_IU2CSEDATA0_IPC_AUTH_RUN: c_int = 2;
pub const BUTTRESS_IU2CSEDATA0_IPC_AUTH_REPLACE: c_int = 3;
pub const BUTTRESS_IU2CSEDATA0_IPC_UPDATE_SECURE_TOUCH: c_int = 16;

pub const BUTTRESS_REG_IU2CSECSR: c_uint = 0x108;

pub const BUTTRESS_REG_CSE2IUDB0: c_uint = 0x304;
pub const BUTTRESS_REG_CSE2IUCSR: c_uint = 0x30C;
pub const BUTTRESS_REG_CSE2IUDATA0: c_uint = 0x308;
// 0x20 == NACK, 0xf == unknown command
pub const BUTTRESS_CSE2IUDATA0_IPC_NACK: c_uint = 0xf20;

pub const BUTTRESS_REG_ISH2IUCSR: c_uint = 0x50;
pub const BUTTRESS_REG_ISH2IUDB0: c_uint = 0x54;
pub const BUTTRESS_REG_ISH2IUDATA0: c_uint = 0x58;
pub const BUTTRESS_REG_IU2ISHDB0: c_uint = 0x10C;
pub const BUTTRESS_REG_IU2ISHDATA0: c_uint = 0x110;
pub const BUTTRESS_REG_IU2ISHDATA1: c_uint = 0x114;
pub const BUTTRESS_REG_IU2ISHCSR: c_uint = 0x118;
pub const BUTTRESS_REG_FABRIC_CMD: c_uint = 0x88;

pub const BUTTRESS_REG_TSW_CTL: c_uint = 0x120;

pub const BUTTRESS_REG_TSC_LO: c_uint = 0x164;
pub const BUTTRESS_REG_TSC_HI: c_uint = 0x168;

