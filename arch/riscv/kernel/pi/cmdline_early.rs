//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/pi/cmdline_early.c
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

    static char early_cmdline[COMMAND_LINE_SIZE];
    static char *get_early_cmdline(uintptr_t dtb_pa)
    {
    const char *fdt_cmdline;
    let mut fdt_cmdline_size: isize = 0;
    int chosen_node;
    if (!IS_ENABLED(CONFIG_CMDLINE_FORCE)) {
    chosen_node = fdt_path_offset((void *)dtb_pa, "/chosen");
    if (chosen_node >= 0) {
    fdt_cmdline = fdt_getprop((void *)dtb_pa, chosen_node,
    "bootargs", core::ptr::null_mut());
    if (fdt_cmdline) {
    fdt_cmdline_size = strscpy(early_cmdline, fdt_cmdline);
    if (fdt_cmdline_size < 0)
    return early_cmdline;
    }
    }
    }
    if (IS_ENABLED(CONFIG_CMDLINE_EXTEND) ||
    IS_ENABLED(CONFIG_CMDLINE_FORCE) ||
    fdt_cmdline_size == 0 /* CONFIG_CMDLINE_FALLBACK */)
    strscpy(early_cmdline + fdt_cmdline_size, CONFIG_CMDLINE,
    COMMAND_LINE_SIZE - fdt_cmdline_size);
    return early_cmdline;
    }
#[no_mangle]
unsafe extern "C" fn match_noXlvl(cmdline: *mut c_char) -> u64 {
    static u64 match_noXlvl(char *cmdline)
    {
    if (strstr(cmdline, "no4lvl"))
    return SATP_MODE_39;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(cmdline, _arg: "no5lvl")) -> else {
    else if (strstr(cmdline, "no5lvl"))
    return SATP_MODE_48;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_satp_mode_from_cmdline(dtb_pa: uintptr_t) -> u64 {
    u64 set_satp_mode_from_cmdline(uintptr_t dtb_pa)
    {
    char *cmdline = get_early_cmdline(dtb_pa);
    return match_noXlvl(cmdline);
    }
#[no_mangle]
unsafe extern "C" fn match_nokaslr(cmdline: *mut c_char) -> bool {
    static bool match_nokaslr(char *cmdline)
    {
    return strstr(cmdline, "nokaslr");
    }
#[no_mangle]
pub unsafe extern "C" fn set_nokaslr_from_cmdline(dtb_pa: uintptr_t) -> bool {
    bool set_nokaslr_from_cmdline(uintptr_t dtb_pa)
    {
    char *cmdline = get_early_cmdline(dtb_pa);
    return match_nokaslr(cmdline);
    }
