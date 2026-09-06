//! Automatically rewritten from C to Rust
//! Source: drivers/thunderbolt/cap.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Thunderbolt driver - capabilities lookup
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

pub const CAP_OFFSET_MAX: c_uint = 0xff;
pub const VSE_CAP_OFFSET_MAX: c_uint = 0xffff;

#[no_mangle]
unsafe extern "C" fn tb_port_enable_tmu(port: *mut tb_port, enable: bool) -> c_int {
    static int tb_port_enable_tmu(struct tb_port *port, bool enable)
    {
    struct tb_switch *sw = port.sw;
    u32 value, offset;
    int ret;
//
// Legacy devices need to have TMU access enabled before port
// space can be fully accessed.
//
    if (tb_switch_is_light_ridge(sw))
    offset = 0x26;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tb_switch_is_eagle_ridge(sw)) -> else {
    else if (tb_switch_is_eagle_ridge(sw))
    offset = 0x2a;
    else
    return 0;
    ret = tb_sw_read(sw, &value, TB_CFG_SWITCH, offset, 1);
    if (ret)
    return ret;
    if (enable)
    value |= TMU_ACCESS_EN;
    else
    value &= ~TMU_ACCESS_EN;
    return tb_sw_write(sw, &value, TB_CFG_SWITCH, offset, 1);
    }
#[no_mangle]
unsafe extern "C" fn tb_port_dummy_read(port: *mut tb_port) {
    static void tb_port_dummy_read(struct tb_port *port)
    {
//
// When reading from next capability pointer location in port
// config space the read data is not cleared on LR. To avoid
// reading stale data on next read perform one dummy read after
// port capabilities are walked.
//
    if (tb_switch_is_light_ridge(port.sw)) {
    u32 dummy;
    tb_port_read(port, &dummy, TB_CFG_PORT, 0, 1);
    }
    }
//
// tb_port_next_cap() - Return next capability in the linked list
// @port: Port to find the capability for
// @offset: Previous capability offset (%0 for start)
//
// Finds dword offset of the next capability in port config space
// capability list. When passed %0 in @offset parameter, first entry
// will be returned, if it exists.
//
// Return:
// * Double word offset of the first or next capability - On success.
// * %0 - If no next capability is found.
// * Negative errno - Another error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn tb_port_next_cap(port: *mut tb_port, offset: c_uint) -> c_int {
    int tb_port_next_cap(struct tb_port *port, unsigned int offset)
    {
    struct tb_cap_any header;
    int ret;
    if (!offset)
    return port.config.first_cap_offset;
    ret = tb_port_read(port, &header, TB_CFG_PORT, offset, 1);
    if (ret)
    return ret;
    return header.basic.next;
    }
#[no_mangle]
unsafe extern "C" fn __tb_port_find_cap(port: *mut tb_port, cap: enum tb_port_cap) -> c_int {
    static int __tb_port_find_cap(struct tb_port *port, enum tb_port_cap cap)
    {
    let mut offset: c_int = 0;
    do {
    struct tb_cap_any header;
    int ret;
    offset = tb_port_next_cap(port, offset);
    if (offset < 0)
    return offset;
    ret = tb_port_read(port, &header, TB_CFG_PORT, offset, 1);
    if (ret)
    return ret;
    if (header.basic.cap == cap)
    return offset;
    } while (offset > 0);
    return -ENOENT;
    }
//
// tb_port_find_cap() - Find port capability
// @port: Port to find the capability for
// @cap: Capability to look
//
// Return:
// * Offset to the start of capability - On success.
// * %-ENOENT - If no such capability was found.
// * Negative errno - Another error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn tb_port_find_cap(port: *mut tb_port, cap: enum tb_port_cap) -> c_int {
    int tb_port_find_cap(struct tb_port *port, enum tb_port_cap cap)
    {
    int ret;
    ret = tb_port_enable_tmu(port, true);
    if (ret)
    return ret;
    ret = __tb_port_find_cap(port, cap);
    tb_port_dummy_read(port);
    tb_port_enable_tmu(port, false);
    return ret;
    }
//
// tb_switch_next_cap() - Return next capability in the linked list
// @sw: Switch to find the capability for
// @offset: Previous capability offset (%0 for start)
//
// Finds dword offset of the next capability in port config space
// capability list. When passed %0 in @offset parameter, first entry
// will be returned, if it exists.
//
// Return:
// * Double word offset of the first or next capability - On success.
// * %0 - If no next capability is found.
// * Negative errno - Another error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn tb_switch_next_cap(sw: *mut tb_switch, offset: c_uint) -> c_int {
    int tb_switch_next_cap(struct tb_switch *sw, unsigned int offset)
    {
    struct tb_cap_any header;
    int ret;
    if (!offset)
    return sw.config.first_cap_offset;
    ret = tb_sw_read(sw, &header, TB_CFG_SWITCH, offset, 2);
    if (ret)
    return ret;
    switch (header.basic.cap) {
    case TB_SWITCH_CAP_TMU:
    ret = header.basic.next;
    break;
    case TB_SWITCH_CAP_VSE:
    if (!header.extended_short.length)
    ret = header.extended_long.next;
    else
    ret = header.extended_short.next;
    break;
    default:
    tb_sw_dbg(sw, "unknown capability %#x at %#x\n",
    header.basic.cap, offset);
    ret = -EINVAL;
    break;
    }
    return ret >= VSE_CAP_OFFSET_MAX ? 0 : ret;
    }
//
// tb_switch_find_cap() - Find switch capability
// @sw: Switch to find the capability for
// @cap: Capability to look
//
// Return:
// * Offset to the start of capability - On success.
// * %-ENOENT - If no such capability was found.
// * Negative errno - Another error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn tb_switch_find_cap(sw: *mut tb_switch, cap: enum tb_switch_cap) -> c_int {
    int tb_switch_find_cap(struct tb_switch *sw, enum tb_switch_cap cap)
    {
    let mut offset: c_int = 0;
    do {
    struct tb_cap_any header;
    int ret;
    offset = tb_switch_next_cap(sw, offset);
    if (offset < 0)
    return offset;
    ret = tb_sw_read(sw, &header, TB_CFG_SWITCH, offset, 1);
    if (ret)
    return ret;
    if (header.basic.cap == cap)
    return offset;
    } while (offset);
    return -ENOENT;
    }
//
// tb_switch_find_vse_cap() - Find switch vendor specific capability
// @sw: Switch to find the capability for
// @vsec: Vendor specific capability to look
//
// This function enumerates vendor specific capabilities (VSEC) of a
// switch and returns offset when capability matching @vsec is found.
//
// Return:
// * Offset of capability - On success.
// * %-ENOENT - If capability was not found.
// * Negative errno - Another error occurred.
//
#[no_mangle]
pub unsafe extern "C" fn tb_switch_find_vse_cap(sw: *mut tb_switch, vsec: enum tb_switch_vse_cap) -> c_int {
    int tb_switch_find_vse_cap(struct tb_switch *sw, enum tb_switch_vse_cap vsec)
    {
    let mut offset: c_int = 0;
    do {
    struct tb_cap_any header;
    int ret;
    offset = tb_switch_next_cap(sw, offset);
    if (offset < 0)
    return offset;
    ret = tb_sw_read(sw, &header, TB_CFG_SWITCH, offset, 1);
    if (ret)
    return ret;
    if (header.extended_short.cap == TB_SWITCH_CAP_VSE &&
    header.extended_short.vsec_id == vsec)
    return offset;
    } while (offset);
    return -ENOENT;
    }
