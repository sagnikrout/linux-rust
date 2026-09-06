//! Automatically rewritten from C to Rust
//! Source: drivers/dma/bestcomm/bcom_gen_bd_rx_task.c
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
// Bestcomm GenBD RX task microcode
//
// Copyright (C) 2006 AppSpec Computer Technologies Corp.
// Jeff Gibbons <jeff.gibbons@appspec.com>
// Copyright (c) 2004 Freescale Semiconductor, Inc.
//
// Based on BestCommAPI-2.2/code_dma/image_rtos1/dma_image.hex
// on Tue Mar 4 10:14:12 2006 GMT
//

//
// The header consists of the following fields:
// u32	magic;
// u8	desc_size;
// u8	var_size;
// u8	inc_size;
// u8	first_var;
// u8	reserved[8];
//
// The size fields contain the number of 32-bit words.
//
    u32 bcom_gen_bd_rx_task[] = {
// header
    0x4243544b,
    0x0d020409,
    0x00000000,
    0x00000000,
// Task descriptors
    0x808220da, /* LCD: idx0 = var1, idx1 = var4; idx1 <= var3; idx0 += inc3, idx1 += inc2 */
    0x13e01010, /*   DRD1A: var4 = var2; FN=0 MORE init=31 WS=0 RS=0 */
    0xb880025b, /*   LCD: idx2 = *idx1, idx3 = var0; idx2 < var9; idx2 += inc3, idx3 += inc3 */
    0x10001308, /*     DRD1A: var4 = idx1; FN=0 MORE init=0 WS=0 RS=0 */
    0x60140002, /*     DRD2A: EU0=0 EU1=0 EU2=0 EU3=2 EXT init=0 WS=2 RS=2 */
    0x0cccfcca, /*     DRD2B1: *idx3 = EU3(); EU3(*idx3,var10)  */
    0xd9190240, /*   LCDEXT: idx2 = idx2; idx2 > var9; idx2 += inc0 */
    0xb8c5e009, /*   LCD: idx3 = *(idx1 + var00000015); ; idx3 += inc1 */
    0x07fecf80, /*     DRD1A: *idx3 = *idx0; FN=0 INT init=31 WS=3 RS=3 */
    0x99190024, /*   LCD: idx2 = idx2; idx2 once var0; idx2 += inc4 */
    0x60000005, /*     DRD2A: EU0=0 EU1=0 EU2=0 EU3=5 EXT init=0 WS=0 RS=0 */
    0x0c4cf889, /*     DRD2B1: *idx1 = EU3(); EU3(idx2,var9)  */
    0x000001f8, /*   NOP */
// VAR[9]-VAR[10]
    0x40000000,
    0x7fff7fff,
// INC[0]-INC[3]
    0x40000000,
    0xe0000000,
    0xa0000008,
    0x20000000,
    };
