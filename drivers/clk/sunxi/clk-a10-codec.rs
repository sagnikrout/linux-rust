//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-a10-codec.c
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
// Copyright 2013 Emilio López
//
// Emilio López <emilio@elopez.com.ar>
//

pub const SUN4I_CODEC_GATE: c_int = 31;
#[no_mangle]
unsafe extern "C" fn sun4i_codec_clk_setup(node: *mut device_node) -> void __init {
    static void __init sun4i_codec_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    const char *clk_name = node.name, *parent_name;
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg))
    return;
    of_property_read_string(node, "clock-output-names", &clk_name);
    parent_name = of_clk_get_parent_name(node, 0);
    clk = clk_register_gate(core::ptr::null_mut(), clk_name, parent_name,
    CLK_SET_RATE_PARENT, reg,
    SUN4I_CODEC_GATE, 0, core::ptr::null_mut());
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
    CLK_OF_DECLARE(sun4i_codec, "allwinner,sun4i-a10-codec-clk",
    sun4i_codec_clk_setup);
