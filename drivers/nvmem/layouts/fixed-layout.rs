//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/layouts/fixed-layout.c
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
// Copyright 2026 Bootlin
//
// Authors: Mathieu Dubois-Briand <mathieu.dubois-briand@bootlin.com>
//

#[no_mangle]
unsafe extern "C" fn fixed_layout_add_cells(layout: *mut nvmem_layout) -> c_int {
    static int fixed_layout_add_cells(struct nvmem_layout *layout)
    {
    struct device_node *np;
    int ret;
    np = of_nvmem_layout_get_container(layout.nvmem);
    if (!np)
    return -ENOENT;
    ret = nvmem_add_cells_from_dt(layout.nvmem, np);
    of_node_put(np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fixed_layout_probe(layout: *mut nvmem_layout) -> c_int {
    static int fixed_layout_probe(struct nvmem_layout *layout)
    {
    layout.add_cells = fixed_layout_add_cells;
    return nvmem_layout_register(layout);
    }
#[no_mangle]
unsafe extern "C" fn fixed_layout_remove(layout: *mut nvmem_layout) {
    static void fixed_layout_remove(struct nvmem_layout *layout)
    {
    nvmem_layout_unregister(layout);
    }
    static const struct of_device_id fixed_layout_of_match_table[] = {
    { .compatible = "fixed-layout", },
    {},
    };
    static struct nvmem_layout_driver fixed_layout_layout = {
    .driver = {
    .name = "fixed-layout",
    .of_match_table = fixed_layout_of_match_table,
    },
    .probe = fixed_layout_probe,
    .remove = fixed_layout_remove,
    };
    module_nvmem_layout_driver(fixed_layout_layout);
    MODULE_AUTHOR("Mathieu Dubois-Briand");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(of, fixed_layout_of_match_table);
    MODULE_DESCRIPTION("NVMEM fixed-layout driver");
