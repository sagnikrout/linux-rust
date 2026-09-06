//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/img-ir/img-ir-sharp.c
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
// ImgTec IR Decoder setup for Sharp protocol.
//
// Copyright 2012-2014 Imagination Technologies Ltd.
//

// Convert Sharp data to a scancode
    static int img_ir_sharp_scancode(int len, u64 raw, u64 enabled_protocols,
    struct img_ir_scancode_req *request)
    {
    unsigned int addr, cmd, exp, chk;
    if (len != 15)
    return -EINVAL;
    addr = (raw >>   0) & 0x1f;
    cmd  = (raw >>   5) & 0xff;
    exp  = (raw >>  13) &  0x1;
    chk  = (raw >>  14) &  0x1;
// validate data
    if (!exp)
    return -EINVAL;
    if (chk)
// probably the second half of the message
    return -EINVAL;
    request.protocol = RC_PROTO_SHARP;
    request.scancode = addr << 8 | cmd;
    return IMG_IR_SCANCODE;
    }
// Convert Sharp scancode to Sharp data filter
    static int img_ir_sharp_filter(const struct rc_scancode_filter *in,
    struct img_ir_filter *out, u64 protocols)
    {
    unsigned int addr, cmd, exp = 0, chk = 0;
    unsigned int addr_m, cmd_m, exp_m = 0, chk_m = 0;
    addr   = (in.data >> 8) & 0x1f;
    addr_m = (in.mask >> 8) & 0x1f;
    cmd    = (in.data >> 0) & 0xff;
    cmd_m  = (in.mask >> 0) & 0xff;
    if (cmd_m) {
// if filtering commands, we can only match the first part
    exp   = 1;
    exp_m = 1;
    chk   = 0;
    chk_m = 1;
    }
    out.data = addr        |
    cmd   <<  5 |
    exp   << 13 |
    chk   << 14;
    out.mask = addr_m      |
    cmd_m <<  5 |
    exp_m << 13 |
    chk_m << 14;
    return 0;
    }
//
// Sharp decoder
// See also http://www.sbprojects.com/knowledge/ir/sharp.php
//
    struct img_ir_decoder img_ir_sharp = {
    .type = RC_PROTO_BIT_SHARP,
    .control = {
    .decoden = 0,
    .decodend2 = 1,
    .code_type = IMG_IR_CODETYPE_PULSEDIST,
    .d1validsel = 1,
    },
// main timings
    .tolerance = 20,	/* 20% */
    .timings = {
// 0 symbol
    .s10 = {
    .pulse = { 320	/* 320 us */ },
    .space = { 680	/* 1 ms period */ },
    },
// 1 symbol
    .s11 = {
    .pulse = { 320	/* 320 us */ },
    .space = { 1680	/* 2 ms period */ },
    },
// free time
    .ft = {
    .minlen = 15,
    .maxlen = 15,
    .ft_min = 5000,	/* 5 ms */
    },
    },
// scancode logic
    .scancode = img_ir_sharp_scancode,
    .filter = img_ir_sharp_filter,
    };
