//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/sd_uhs2.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Header file for UHS-II packets, Host Controller registers and I/O
// accessors.
//
// Copyright (C) 2014 Intel Corp, All Rights Reserved.
//
// LINK Layer definition
//
// UHS2 Header:
// Refer to UHS-II Addendum Version 1.02 Figure 5-2, the format of CCMD Header is described below:
// bit [3:0]  : DID(Destination ID = Node ID of UHS2 card)
// bit [6:4]  : TYP(Packet Type)
// 000b: CCMD(Control command packet)
// 001b: DCMD(Data command packet)
// 010b: RES(Response packet)
// 011b: DATA(Data payload packet)
// 111b: MSG(Message packet)
// Others: Reserved
// bit [7]    : NP(Native Packet)
// bit [10:8] : TID(Transaction ID)
// bit [11]   : Reserved
// bit [15:12]: SID(Source ID 0: Node ID of Host)
//
// Broadcast CCMD issued by Host is represented as DID=SID=0.
//
// UHS2 Argument:
// Refer to UHS-II Addendum Version 1.02 Figure 6-5, the format of CCMD Argument is described below:
// bit [3:0]  : MSB of IOADR
// bit [5:4]  : PLEN(Payload Length)
// 00b: 0 byte
// 01b: 4 bytes
// 10b: 8 bytes
// 11b: 16 bytes
// bit [6]    : Reserved
// bit [7]    : R/W(Read/Write)
// 0: Control read command
// 1: Control write command
// bit [15:8] : LSB of IOADR
//
// I/O Address specifies the address of register in UHS-II I/O space accessed by CCMD.
// The unit of I/O Address is 4 Bytes. It is transmitted in MSB first, LSB last.
//
pub const UHS2_NATIVE_PACKET_POS: c_int = 7;

pub const UHS2_PACKET_TYPE_POS: c_int = 4;

pub const UHS2_DEST_ID_MASK: c_uint = 0x0F;
pub const UHS2_DEST_ID: c_uint = 0x1;
pub const UHS2_SRC_ID_POS: c_int = 12;
pub const UHS2_SRC_ID_MASK: c_uint = 0xF000;
pub const UHS2_TRANS_ID_POS: c_int = 8;
pub const UHS2_TRANS_ID_MASK: c_uint = 0x0700;
// UHS2 MSG
pub const UHS2_MSG_CTG_POS: c_int = 5;
pub const UHS2_MSG_CTG_LMSG: c_uint = 0x00;
pub const UHS2_MSG_CTG_INT: c_uint = 0x60;
pub const UHS2_MSG_CTG_AMSG: c_uint = 0x80;
pub const UHS2_MSG_CTG_FCREQ: c_uint = 0x00;
pub const UHS2_MSG_CTG_FCRDY: c_uint = 0x01;
pub const UHS2_MSG_CTG_STAT: c_uint = 0x02;
pub const UHS2_MSG_CODE_POS: c_int = 8;
pub const UHS2_MSG_CODE_FC_UNRECOVER_ERR: c_uint = 0x8;
pub const UHS2_MSG_CODE_STAT_UNRECOVER_ERR: c_uint = 0x8;
pub const UHS2_MSG_CODE_STAT_RECOVER_ERR: c_uint = 0x1;
// TRANS Layer definition
// Native packets
pub const UHS2_NATIVE_CMD_RW_POS: c_int = 7;

pub const UHS2_NATIVE_CMD_PLEN_POS: c_int = 4;

pub const UHS2_NATIVE_CCMD_GET_MIOADR_MASK: c_uint = 0xF00;
pub const UHS2_NATIVE_CCMD_MIOADR_MASK: c_uint = 0x0F;
pub const UHS2_NATIVE_CCMD_LIOADR_POS: c_int = 8;
pub const UHS2_NATIVE_CCMD_GET_LIOADR_MASK: c_uint = 0x0FF;

pub const UHS2_DEV_INIT_PAYLOAD_LEN: c_int = 1;
pub const UHS2_DEV_INIT_RESP_LEN: c_int = 6;
pub const UHS2_DEV_ENUM_PAYLOAD_LEN: c_int = 1;
pub const UHS2_DEV_ENUM_RESP_LEN: c_int = 8;
pub const UHS2_CFG_WRITE_PAYLOAD_LEN: c_int = 2;
pub const UHS2_CFG_WRITE_PHY_SET_RESP_LEN: c_int = 4;
pub const UHS2_CFG_WRITE_GENERIC_SET_RESP_LEN: c_int = 5;
pub const UHS2_GO_DORMANT_PAYLOAD_LEN: c_int = 1;
//
// UHS2 Argument:
// Refer to UHS-II Addendum Version 1.02 Figure 6-8, the format of DCMD Argument is described below:
// bit [3:0]  : Reserved
// bit [6:3]  : TMODE(Transfer Mode)
// bit 3: DAM(Data Access Mode)
// bit 4: TLUM(TLEN Unit Mode)
// bit 5: LM(Length Mode)
// bit 6: DM(Duplex Mode)
// bit [7]    : R/W(Read/Write)
// 0: Control read command
// 1: Control write command
// bit [15:8] : Reserved
//
// I/O Address specifies the address of register in UHS-II I/O space accessed by CCMD.
// The unit of I/O Address is 4 Bytes. It is transmitted in MSB first, LSB last.
//
pub const UHS2_DCMD_DM_POS: c_int = 6;

pub const UHS2_DCMD_LM_POS: c_int = 5;

pub const UHS2_DCMD_TLUM_POS: c_int = 4;

pub const UHS2_NATIVE_DCMD_DAM_POS: c_int = 3;

pub const UHS2_RES_NACK_POS: c_int = 7;

pub const UHS2_RES_ECODE_POS: c_int = 4;
pub const UHS2_RES_ECODE_MASK: c_uint = 0x7;
pub const UHS2_RES_ECODE_COND: c_int = 1;
pub const UHS2_RES_ECODE_ARG: c_int = 2;
pub const UHS2_RES_ECODE_GEN: c_int = 3;
// IOADR of device registers
pub const UHS2_IOADR_GENERIC_CAPS: c_uint = 0x00;
pub const UHS2_IOADR_PHY_CAPS: c_uint = 0x02;
pub const UHS2_IOADR_LINK_CAPS: c_uint = 0x04;
pub const UHS2_IOADR_RSV_CAPS: c_uint = 0x06;
pub const UHS2_IOADR_GENERIC_SETTINGS: c_uint = 0x08;
pub const UHS2_IOADR_PHY_SETTINGS: c_uint = 0x0A;
pub const UHS2_IOADR_LINK_SETTINGS: c_uint = 0x0C;
pub const UHS2_IOADR_PRESET: c_uint = 0x40;
// SD application packets
pub const UHS2_SD_CMD_INDEX_POS: c_int = 8;
pub const UHS2_SD_CMD_APP_POS: c_int = 14;

// UHS-II Device Registers
pub const UHS2_DEV_CONFIG_REG: c_uint = 0x000;
// General Caps and Settings registers

pub const UHS2_DEV_CONFIG_N_LANES_POS: c_int = 8;
pub const UHS2_DEV_CONFIG_N_LANES_MASK: c_uint = 0x3F;
pub const UHS2_DEV_CONFIG_2L_HD_FD: c_uint = 0x1;
pub const UHS2_DEV_CONFIG_2D1U_FD: c_uint = 0x2;
pub const UHS2_DEV_CONFIG_1D2U_FD: c_uint = 0x4;
pub const UHS2_DEV_CONFIG_2D2U_FD: c_uint = 0x8;
pub const UHS2_DEV_CONFIG_DADR_POS: c_int = 14;
pub const UHS2_DEV_CONFIG_DADR_MASK: c_uint = 0x1;
pub const UHS2_DEV_CONFIG_APP_POS: c_int = 16;
pub const UHS2_DEV_CONFIG_APP_MASK: c_uint = 0xFF;
pub const UHS2_DEV_CONFIG_APP_SD_MEM: c_uint = 0x1;

pub const UHS2_DEV_CONFIG_GEN_SET_N_LANES_POS: c_int = 8;
pub const UHS2_DEV_CONFIG_GEN_SET_2L_FD_HD: c_uint = 0x0;
pub const UHS2_DEV_CONFIG_GEN_SET_2D1U_FD: c_uint = 0x2;
pub const UHS2_DEV_CONFIG_GEN_SET_1D2U_FD: c_uint = 0x3;
pub const UHS2_DEV_CONFIG_GEN_SET_2D2U_FD: c_uint = 0x4;

// PHY Caps and Settings registers

pub const UHS2_DEV_CONFIG_PHY_MINOR_MASK: c_uint = 0xF;
pub const UHS2_DEV_CONFIG_PHY_MAJOR_POS: c_int = 4;
pub const UHS2_DEV_CONFIG_PHY_MAJOR_MASK: c_uint = 0x3;
pub const UHS2_DEV_CONFIG_CAN_HIBER_POS: c_int = 15;
pub const UHS2_DEV_CONFIG_CAN_HIBER_MASK: c_uint = 0x1;

pub const UHS2_DEV_CONFIG_N_LSS_SYN_MASK: c_uint = 0xF;
pub const UHS2_DEV_CONFIG_N_LSS_DIR_POS: c_int = 4;
pub const UHS2_DEV_CONFIG_N_LSS_DIR_MASK: c_uint = 0xF;

pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_POS: c_int = 6;
pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_A: c_uint = 0x0;
pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_B: c_uint = 0x1;
// LINK-TRAN Caps and Settings registers

pub const UHS2_DEV_CONFIG_LT_MINOR_MASK: c_uint = 0xF;
pub const UHS2_DEV_CONFIG_LT_MAJOR_POS: c_int = 4;
pub const UHS2_DEV_CONFIG_LT_MAJOR_MASK: c_uint = 0x3;
pub const UHS2_DEV_CONFIG_N_FCU_POS: c_int = 8;
pub const UHS2_DEV_CONFIG_N_FCU_MASK: c_uint = 0xFF;
pub const UHS2_DEV_CONFIG_DEV_TYPE_POS: c_int = 16;
pub const UHS2_DEV_CONFIG_DEV_TYPE_MASK: c_uint = 0x7;
pub const UHS2_DEV_CONFIG_MAX_BLK_LEN_POS: c_int = 20;
pub const UHS2_DEV_CONFIG_MAX_BLK_LEN_MASK: c_uint = 0xFFF;

pub const UHS2_DEV_CONFIG_N_DATA_GAP_MASK: c_uint = 0xFF;

pub const UHS2_DEV_CONFIG_LT_SET_MAX_BLK_LEN: c_uint = 0x200;
pub const UHS2_DEV_CONFIG_LT_SET_MAX_RETRY_POS: c_int = 16;
// Preset register

pub const UHS2_DEV_INT_REG: c_uint = 0x100;
pub const UHS2_DEV_STATUS_REG: c_uint = 0x180;
pub const UHS2_DEV_CMD_REG: c_uint = 0x200;

pub const UHS2_RCLK_MAX: c_int = 52000000;
pub const UHS2_RCLK_MIN: c_int = 26000000;
