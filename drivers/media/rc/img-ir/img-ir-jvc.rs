//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/img-ir/img-ir-jvc.c
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
// ImgTec IR Decoder setup for JVC protocol.
//
// Copyright 2012-2014 Imagination Technologies Ltd.
//

// Convert JVC data to a scancode
    static int img_ir_jvc_scancode(int len, u64 raw, u64 enabled_protocols,
    struct img_ir_scancode_req *request)
    {
    unsigned int cust, data;
    if (len != 16)
    return -EINVAL;
    cust = (raw >> 0) & 0xff;
    data = (raw >> 8) & 0xff;
    request.protocol = RC_PROTO_JVC;
    request.scancode = cust << 8 | data;
    return IMG_IR_SCANCODE;
    }
// Convert JVC scancode to JVC data filter
    static int img_ir_jvc_filter(const struct rc_scancode_filter *in,
    struct img_ir_filter *out, u64 protocols)
    {
    unsigned int cust, data;
    unsigned int cust_m, data_m;
    cust   = (in.data >> 8) & 0xff;
    cust_m = (in.mask >> 8) & 0xff;
    data   = (in.data >> 0) & 0xff;
    data_m = (in.mask >> 0) & 0xff;
    out.data = cust   | data << 8;
    out.mask = cust_m | data_m << 8;
    return 0;
    }
//
// JVC decoder
// See also http://www.sbprojects.com/knowledge/ir/jvc.php
// http://support.jvc.com/consumer/support/documents/RemoteCodes.pdf
//
    struct img_ir_decoder img_ir_jvc = {
    .type = RC_PROTO_BIT_JVC,
    .control = {
    .decoden = 1,
    .code_type = IMG_IR_CODETYPE_PULSEDIST,
    },
// main timings
    .unit = 527500, /* 527.5 us */
    .timings = {
// leader symbol
    .ldr = {
    .pulse = { 16	/* 8.44 ms */ },
    .space = { 8	/* 4.22 ms */ },
    },
// 0 symbol
    .s00 = {
    .pulse = { 1	/* 527.5 us +-60 us */ },
    .space = { 1	/* 527.5 us */ },
    },
// 1 symbol
    .s01 = {
    .pulse = { 1	/* 527.5 us +-60 us */ },
    .space = { 3	/* 1.5825 ms +-40 us */ },
    },
// free time
    .ft = {
    .minlen = 16,
    .maxlen = 16,
    .ft_min = 10,	/* 5.275 ms */
    },
    },
// scancode logic
    .scancode = img_ir_jvc_scancode,
    .filter = img_ir_jvc_filter,
    };
