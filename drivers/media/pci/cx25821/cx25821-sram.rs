//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821-sram.h
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
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
//
// #define RX_SRAM_START_SIZE        = 0;  //  Start of reserved SRAM

// #define RX_SRAM_POOL_START_SIZE   = 0;  //  Start of usable RX SRAM for buffers

pub const MBIF_IQ_SIZE: c_int = 64;

// #define RX_SRAM_POOL_FREE_SIZE    = 16; //  Start of available RX SRAM
// #define RX_SRAM_END_SIZE          = 0;  //  End of RX SRAM
// #define TX_SRAM_POOL_START_SIZE   = 0;  //  Start of transmit pool SRAM
// #define MSI_DATA_SIZE             = 64; //  Reserved (MSI Data, RISC working stora

// #define TX_SRAM_POOL_FREE_SIZE    = 704;    //  Start of available TX SRAM
// #define TX_SRAM_END_SIZE          = 0;      //  End of TX SRAM
// Receive SRAM
pub const RX_SRAM_START: c_uint = 0x10000;
pub const VID_A_DOWN_CMDS: c_uint = 0x10000;
pub const VID_B_DOWN_CMDS: c_uint = 0x10050;
pub const VID_C_DOWN_CMDS: c_uint = 0x100A0;
pub const VID_D_DOWN_CMDS: c_uint = 0x100F0;
pub const VID_E_DOWN_CMDS: c_uint = 0x10140;
pub const VID_F_DOWN_CMDS: c_uint = 0x10190;
pub const VID_G_DOWN_CMDS: c_uint = 0x101E0;
pub const VID_H_DOWN_CMDS: c_uint = 0x10230;
pub const VID_A_UP_CMDS: c_uint = 0x10280;
pub const VID_B_UP_CMDS: c_uint = 0x102D0;
pub const VID_C_UP_CMDS: c_uint = 0x10320;
pub const VID_D_UP_CMDS: c_uint = 0x10370;
pub const VID_E_UP_CMDS: c_uint = 0x103C0;
pub const VID_F_UP_CMDS: c_uint = 0x10410;
pub const VID_I_UP_CMDS: c_uint = 0x10460;
pub const VID_J_UP_CMDS: c_uint = 0x104B0;
pub const AUD_A_DOWN_CMDS: c_uint = 0x10500;
pub const AUD_B_DOWN_CMDS: c_uint = 0x10550;
pub const AUD_C_DOWN_CMDS: c_uint = 0x105A0;
pub const AUD_D_DOWN_CMDS: c_uint = 0x105F0;
pub const AUD_A_UP_CMDS: c_uint = 0x10640;
pub const AUD_B_UP_CMDS: c_uint = 0x10690;
pub const AUD_C_UP_CMDS: c_uint = 0x106E0;
pub const AUD_E_UP_CMDS: c_uint = 0x10730;
pub const MBIF_A_DOWN_CMDS: c_uint = 0x10780;
pub const MBIF_B_DOWN_CMDS: c_uint = 0x107D0;
pub const DMA_SCRATCH_PAD: c_uint = 0x10820	/* Scratch pad area from 0x10820 to 0x10B40 */;
// #define RX_SRAM_POOL_START        = 0x105B0;
pub const VID_A_IQ: c_uint = 0x11000;
pub const VID_B_IQ: c_uint = 0x11040;
pub const VID_C_IQ: c_uint = 0x11080;
pub const VID_D_IQ: c_uint = 0x110C0;
pub const VID_E_IQ: c_uint = 0x11100;
pub const VID_F_IQ: c_uint = 0x11140;
pub const VID_G_IQ: c_uint = 0x11180;
pub const VID_H_IQ: c_uint = 0x111C0;
pub const VID_I_IQ: c_uint = 0x11200;
pub const VID_J_IQ: c_uint = 0x11240;
pub const AUD_A_IQ: c_uint = 0x11280;
pub const AUD_B_IQ: c_uint = 0x112C0;
pub const AUD_C_IQ: c_uint = 0x11300;
pub const AUD_D_IQ: c_uint = 0x11340;
pub const AUD_E_IQ: c_uint = 0x11380;
pub const MBIF_A_IQ: c_uint = 0x11000;
pub const MBIF_B_IQ: c_uint = 0x110C0;
pub const VID_A_CDT: c_uint = 0x10C00;
pub const VID_B_CDT: c_uint = 0x10C40;
pub const VID_C_CDT: c_uint = 0x10C80;
pub const VID_D_CDT: c_uint = 0x10CC0;
pub const VID_E_CDT: c_uint = 0x10D00;
pub const VID_F_CDT: c_uint = 0x10D40;
pub const VID_G_CDT: c_uint = 0x10D80;
pub const VID_H_CDT: c_uint = 0x10DC0;
pub const VID_I_CDT: c_uint = 0x10E00;
pub const VID_J_CDT: c_uint = 0x10E40;
pub const AUD_A_CDT: c_uint = 0x10E80;
pub const AUD_B_CDT: c_uint = 0x10EB0;
pub const AUD_C_CDT: c_uint = 0x10EE0;
pub const AUD_D_CDT: c_uint = 0x10F10;
pub const AUD_E_CDT: c_uint = 0x10F40;
pub const MBIF_A_CDT: c_uint = 0x10C00;
pub const MBIF_B_CDT: c_uint = 0x10CC0;
// Cluster Buffer for RX
pub const VID_A_UP_CLUSTER_1: c_uint = 0x11400;
pub const VID_A_UP_CLUSTER_2: c_uint = 0x119A0;
pub const VID_A_UP_CLUSTER_3: c_uint = 0x11F40;
pub const VID_A_UP_CLUSTER_4: c_uint = 0x124E0;
pub const VID_B_UP_CLUSTER_1: c_uint = 0x12A80;
pub const VID_B_UP_CLUSTER_2: c_uint = 0x13020;
pub const VID_B_UP_CLUSTER_3: c_uint = 0x135C0;
pub const VID_B_UP_CLUSTER_4: c_uint = 0x13B60;
pub const VID_C_UP_CLUSTER_1: c_uint = 0x14100;
pub const VID_C_UP_CLUSTER_2: c_uint = 0x146A0;
pub const VID_C_UP_CLUSTER_3: c_uint = 0x14C40;
pub const VID_C_UP_CLUSTER_4: c_uint = 0x151E0;
pub const VID_D_UP_CLUSTER_1: c_uint = 0x15780;
pub const VID_D_UP_CLUSTER_2: c_uint = 0x15D20;
pub const VID_D_UP_CLUSTER_3: c_uint = 0x162C0;
pub const VID_D_UP_CLUSTER_4: c_uint = 0x16860;
pub const VID_E_UP_CLUSTER_1: c_uint = 0x16E00;
pub const VID_E_UP_CLUSTER_2: c_uint = 0x173A0;
pub const VID_E_UP_CLUSTER_3: c_uint = 0x17940;
pub const VID_E_UP_CLUSTER_4: c_uint = 0x17EE0;
pub const VID_F_UP_CLUSTER_1: c_uint = 0x18480;
pub const VID_F_UP_CLUSTER_2: c_uint = 0x18A20;
pub const VID_F_UP_CLUSTER_3: c_uint = 0x18FC0;
pub const VID_F_UP_CLUSTER_4: c_uint = 0x19560;
pub const VID_I_UP_CLUSTER_1: c_uint = 0x19B00;
pub const VID_I_UP_CLUSTER_2: c_uint = 0x1A0A0;
pub const VID_I_UP_CLUSTER_3: c_uint = 0x1A640;
pub const VID_I_UP_CLUSTER_4: c_uint = 0x1ABE0;
pub const VID_J_UP_CLUSTER_1: c_uint = 0x1B180;
pub const VID_J_UP_CLUSTER_2: c_uint = 0x1B720;
pub const VID_J_UP_CLUSTER_3: c_uint = 0x1BCC0;
pub const VID_J_UP_CLUSTER_4: c_uint = 0x1C260;
pub const AUD_A_UP_CLUSTER_1: c_uint = 0x1C800;
pub const AUD_A_UP_CLUSTER_2: c_uint = 0x1C880;
pub const AUD_A_UP_CLUSTER_3: c_uint = 0x1C900;
pub const AUD_B_UP_CLUSTER_1: c_uint = 0x1C980;
pub const AUD_B_UP_CLUSTER_2: c_uint = 0x1CA00;
pub const AUD_B_UP_CLUSTER_3: c_uint = 0x1CA80;
pub const AUD_C_UP_CLUSTER_1: c_uint = 0x1CB00;
pub const AUD_C_UP_CLUSTER_2: c_uint = 0x1CB80;
pub const AUD_C_UP_CLUSTER_3: c_uint = 0x1CC00;
pub const AUD_E_UP_CLUSTER_1: c_uint = 0x1CC80;
pub const AUD_E_UP_CLUSTER_2: c_uint = 0x1CD00;
pub const AUD_E_UP_CLUSTER_3: c_uint = 0x1CD80;
pub const RX_SRAM_POOL_FREE: c_uint = 0x1CE00;
pub const RX_SRAM_END: c_uint = 0x1D000;
// Free Receive SRAM    144 Bytes
// Transmit SRAM
pub const TX_SRAM_POOL_START: c_uint = 0x00000;
pub const VID_A_DOWN_CLUSTER_1: c_uint = 0x00040;
pub const VID_A_DOWN_CLUSTER_2: c_uint = 0x005E0;
pub const VID_A_DOWN_CLUSTER_3: c_uint = 0x00B80;
pub const VID_A_DOWN_CLUSTER_4: c_uint = 0x01120;
pub const VID_B_DOWN_CLUSTER_1: c_uint = 0x016C0;
pub const VID_B_DOWN_CLUSTER_2: c_uint = 0x01C60;
pub const VID_B_DOWN_CLUSTER_3: c_uint = 0x02200;
pub const VID_B_DOWN_CLUSTER_4: c_uint = 0x027A0;
pub const VID_C_DOWN_CLUSTER_1: c_uint = 0x02D40;
pub const VID_C_DOWN_CLUSTER_2: c_uint = 0x032E0;
pub const VID_C_DOWN_CLUSTER_3: c_uint = 0x03880;
pub const VID_C_DOWN_CLUSTER_4: c_uint = 0x03E20;
pub const VID_D_DOWN_CLUSTER_1: c_uint = 0x043C0;
pub const VID_D_DOWN_CLUSTER_2: c_uint = 0x04960;
pub const VID_D_DOWN_CLUSTER_3: c_uint = 0x04F00;
pub const VID_D_DOWN_CLUSTER_4: c_uint = 0x054A0;
pub const VID_E_DOWN_CLUSTER_1: c_uint = 0x05a40;
pub const VID_E_DOWN_CLUSTER_2: c_uint = 0x05FE0;
pub const VID_E_DOWN_CLUSTER_3: c_uint = 0x06580;
pub const VID_E_DOWN_CLUSTER_4: c_uint = 0x06B20;
pub const VID_F_DOWN_CLUSTER_1: c_uint = 0x070C0;
pub const VID_F_DOWN_CLUSTER_2: c_uint = 0x07660;
pub const VID_F_DOWN_CLUSTER_3: c_uint = 0x07C00;
pub const VID_F_DOWN_CLUSTER_4: c_uint = 0x081A0;
pub const VID_G_DOWN_CLUSTER_1: c_uint = 0x08740;
pub const VID_G_DOWN_CLUSTER_2: c_uint = 0x08CE0;
pub const VID_G_DOWN_CLUSTER_3: c_uint = 0x09280;
pub const VID_G_DOWN_CLUSTER_4: c_uint = 0x09820;
pub const VID_H_DOWN_CLUSTER_1: c_uint = 0x09DC0;
pub const VID_H_DOWN_CLUSTER_2: c_uint = 0x0A360;
pub const VID_H_DOWN_CLUSTER_3: c_uint = 0x0A900;
pub const VID_H_DOWN_CLUSTER_4: c_uint = 0x0AEA0;
pub const AUD_A_DOWN_CLUSTER_1: c_uint = 0x0B500;
pub const AUD_A_DOWN_CLUSTER_2: c_uint = 0x0B580;
pub const AUD_A_DOWN_CLUSTER_3: c_uint = 0x0B600;
pub const AUD_B_DOWN_CLUSTER_1: c_uint = 0x0B680;
pub const AUD_B_DOWN_CLUSTER_2: c_uint = 0x0B700;
pub const AUD_B_DOWN_CLUSTER_3: c_uint = 0x0B780;
pub const AUD_C_DOWN_CLUSTER_1: c_uint = 0x0B800;
pub const AUD_C_DOWN_CLUSTER_2: c_uint = 0x0B880;
pub const AUD_C_DOWN_CLUSTER_3: c_uint = 0x0B900;
pub const AUD_D_DOWN_CLUSTER_1: c_uint = 0x0B980;
pub const AUD_D_DOWN_CLUSTER_2: c_uint = 0x0BA00;
pub const AUD_D_DOWN_CLUSTER_3: c_uint = 0x0BA80;
pub const TX_SRAM_POOL_FREE: c_uint = 0x0BB00;
pub const TX_SRAM_END: c_uint = 0x0C000;

