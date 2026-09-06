//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_init.h
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


// bnx2x_init.h: Qlogic Everest network driver.
// Structures and macroes needed during the initialization.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Eliezer Tamir
// Modified by: Vladislav Zolotarov
//
// Init operation types and structures
// Skip the following ops if all of the init modes don't match
// Skip the following ops if any of the init modes don't match
// Returns the index of start or end of a specific block stage in ops array

// structs for the various opcodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_op {
    pub op:8: u32,
    pub offset:24: u32,
    pub raw_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_read {
    pub op:8: u32,
    pub offset:24: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_write {
    pub op:8: u32,
    pub offset:24: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_arr_write {
    pub op:8: u32,
    pub offset:24: u32,

    pub data_len: u16,
    pub data_off: u16,

    pub data_off: u16,
    pub data_len: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_zero {
    pub op:8: u32,
    pub offset:24: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_if_mode {
    pub op:8: u32,
    pub cmd_offset:24: u32,
    pub mode_bit_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union init_op {
    pub read: op_read,
    pub write: op_write,
    pub arr_wr: op_arr_write,
    pub zero: op_zero,
    pub raw: raw_op,
    pub if_mode: op_if_mode,
}

// Init Phases
// Init Modes
// Init Blocks
// QM queue numbers
pub const BNX2X_ETH_Q: c_int = 0;
pub const BNX2X_TOE_Q: c_int = 3;
pub const BNX2X_TOE_ACK_Q: c_int = 6;
pub const BNX2X_ISCSI_Q: c_int = 9;
pub const BNX2X_ISCSI_ACK_Q: c_int = 11;
pub const BNX2X_FCOE_Q: c_int = 10;
// Vnics per mode
pub const BNX2X_PORT2_MODE_NUM_VNICS: c_int = 4;
pub const BNX2X_PORT4_MODE_NUM_VNICS: c_int = 2;
// COS offset for port1 in E3 B0 4port mode
pub const BNX2X_E3B0_PORT1_COS_OFFSET: c_int = 3;
// QM Register addresses
// Macro flag: #define BNX2X_Q_VOQ_REG_ADDR(pf_q_num)\

// Macro flag: #define BNX2X_Q_CMDQ_REG_ADDR(pf_q_num)\
// extracts the QM queue number for the specified port and vnic

// Maps the specified queue to the specified COS
// find current COS mapping
// check if queue->COS mapping has changed
// update parameters for 4port mode
// change queue mapping for each VNIC
// overwrite queue->VOQ mapping
// clear queue bit from current COS bit map
// set queue bit in new COS bit map
// set/clear queue bit in command-queue bit map
// (E2/E3A0 only, valid COS values are 0/1)
//
// Configures the QM according to the specified per-traffic-type COSes
// required only in backward compatible COS mode
// congestion management port init api description
// the api works as follows:
// the driver should pass the cmng_init_input struct, the port_init function
// will prepare the required internal ram structure which will be passed back
// to the driver (cmng_init) that will write it into the internal ram.
//
// IMPORTANT REMARKS:
// 1. the cmng_init struct does not represent the contiguous internal ram
// structure. the driver should use the XSTORM_CMNG_PERPORT_VARS_OFFSET
// offset in order to write the port sub struct and the
// PFID_FROM_PORT_AND_VNIC offset for writing the vnic sub struct (in other
// words - don't use memcpy!).
// 2. although the cmng_init struct is filled for the maximal vnic number
// possible, the driver should only write the valid vnics into the internal
// ram according to the appropriate port mode.
//
// CMNG constants, as derived from system spec calculations
// default MIN rate in case VNIC min rate is configured to zero- 100Mbps
pub const DEF_MIN_RATE: c_int = 100;
// resolution of the rate shaping timer - 400 usec
pub const RS_PERIODIC_TIMEOUT_USEC: c_int = 400;
// number of bytes in single QM arbitration cycle -
// coefficient for calculating the fairness timer
//
pub const QM_ARB_BYTES: c_int = 160000;
// resolution of Min algorithm 1:100
pub const MIN_RES: c_int = 100;
// how many bytes above threshold for
// the minimal credit of Min algorithm
//
pub const MIN_ABOVE_THRESH: c_int = 32768;
// Fairness algorithm integration time coefficient -
// for calculating the actual Tfair
//

// Memory of fairness algorithm - 2 cycles
pub const FAIR_MEM: c_int = 2;
pub const SAFC_TIMEOUT_USEC: c_int = 52;
pub const SDM_TICKS: c_int = 4;
// rate shaping per-port variables
// 100 micro seconds in SDM ticks = 25
// since each tick is 4 microSeconds
//
// this is the threshold below which no timer arming will occur.
// 1.25 coefficient is for the threshold to be a little bigger
// then the real time to compensate for timer in-accuracy
//
// rate shaping per-vnic variables
// global vnic counter
// maximal Mbps for this vnic
// the quota in each timer period - number of bytes
// transmitted in this period
//
// this is the resolution of the fairness timer
// fairness per-port variables
// for 10G it is 1000usec. for 1G it is 10000usec.
//
// this is the threshold below which we won't arm the timer anymore
// we multiply by 1e3/8 to get bytes/msec. We don't want the credits
// to pass a credit of the T_FAIR*FAIR_MEM (algorithm resolution)
//
// since each tick is 4 microSeconds
// calculate sum of weights
// global vnic counter
// fairness per-vnic variables
// this is the credit for each period of the fairness
// algorithm - number of bytes in T_FAIR (this vnic
// share of the port rate)
//
// Since cos and vnic shouldn't work together the rate
// to divide between the coses is the port rate.
//
// this is the credit for each period of
// the fairness algorithm - number of bytes
// in T_FAIR (this cos share of the vnic rate)
//
// in microSeconds
// Congestion management port init
// number of bytes transmitted in a rate of 10Gbps
// in one usec = 1.25KB.
//
// Returns the index of start or end of a specific block stage in ops array

//
// ILT management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilt_line {
    pub page_mapping: dma_addr_t,
    pub page: *mut c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilt_client_info {
    pub page_size: u32,
    pub start: u16,
    pub end: u16,
    pub client_num: u16,
    pub flags: u16,
pub const ILT_CLIENT_SKIP_INIT: c_uint = 0x1;
pub const ILT_CLIENT_SKIP_MEM: c_uint = 0x2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_ilt {
    pub start_line: u32,
    pub lines: *mut ilt_line,
    pub clients: [ilt_client_info; 4],
pub const ILT_CLIENT_CDU: c_int = 0;
pub const ILT_CLIENT_QM: c_int = 1;
pub const ILT_CLIENT_SRC: c_int = 2;
pub const ILT_CLIENT_TM: c_int = 3;
}

//
// SRC configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_ent {
    pub opaque: [u8; 56],
    pub next: u64,
}

//
// Parity configuration
//

// (name + suffix)
//
// bit 19 masked
// REG_WR(bp, PXP_REG_PXP_PRTY_MASK, 0x80000);
// bit 5,18,20-31
// REG_WR(bp, PXP2_REG_PXP2_PRTY_MASK_0, 0xfff40020);
// bit 5
// REG_WR(bp, PXP2_REG_PXP2_PRTY_MASK_1, 0x20);
// REG_WR(bp, HC_REG_HC_PRTY_MASK, 0x0);
// REG_WR(bp, MISC_REG_MISC_PRTY_MASK, 0x0);
// Block IGU, MISC, PXP and PXP2 parity errors as long as we don't
// want to handle "system kill" flow at the moment.
//
// [28] MCP Latched rom_parity
// [29] MCP Latched ump_rx_parity
// [30] MCP Latched ump_tx_parity
// [31] MCP Latched scpad_parity
//

// Below registers control the MCP parity attention output. When
// MISC_AEU_ENABLE_MCP_PRTY_BITS are set - attentions are
// enabled, when cleared - disabled.
//
// Disable MCP parity attentions
// Clear the parity error status registers.
// Clear SEM_FAST parities
// Check if there were parity attentions in MCP
// Clear parity attentions in MCP:
// [7]  clears Latched rom_parity
// [8]  clears Latched ump_rx_parity
// [9]  clears Latched ump_tx_parity
// [10] clears Latched scpad_parity (both ports)
//
// Enable MCP parity attentions
