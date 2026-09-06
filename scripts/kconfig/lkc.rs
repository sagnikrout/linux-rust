//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/lkc.h
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
// Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
//

extern "C" {
    pub fn zconfdump(out: *mut FILE);
}
extern "C" {
    pub fn zconf_starthelp();
}
extern "C" {
    pub fn zconf_initscan(name: *const c_char);
}
extern "C" {
    pub fn zconf_nextfile(name: *const c_char);
}
// confdata.c
// confdata.c and expr.c
// util.c
// lexer.l
extern "C" {
    pub fn yylex() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gstr {
    pub len: usize,
    pub s: *mut c_char,
//
// when max_width is not zero long lines in string s (if any) get
// wrapped not to exceed the max_width value
//
    pub max_width: c_int,
}

extern "C" {
    pub fn str_new() -> gstr;
}
extern "C" {
    pub fn str_free(gs: *mut gstr);
}
extern "C" {
    pub fn str_append(gs: *mut gstr, s: *const c_char);
}
extern "C" {
    pub fn str_printf(gs: *mut gstr, fmt: *const c_char, ...);
}
// menu.c

extern "C" {
    pub fn _menu_init();
}
extern "C" {
    pub fn menu_warn(menu: *const menu, fmt: *const c_char, ...);
}
extern "C" {
    pub fn menu_end_menu();
}
extern "C" {
    pub fn menu_add_entry(sym: *mut symbol, type: menu_type);
}
extern "C" {
    pub fn menu_add_dep(dep: *mut expr, cond: *mut expr);
}
extern "C" {
    pub fn menu_add_visibility(dep: *mut expr);
}
extern "C" {
    pub fn menu_add_expr(type: prop_type, expr: *mut expr, dep: *mut expr);
}
extern "C" {
    pub fn menu_add_symbol(type: prop_type, sym: *mut symbol, dep: *mut expr);
}
extern "C" {
    pub fn menu_finalize();
}
extern "C" {
    pub fn menu_set_type(type: c_int);
}
extern "C" {
    pub fn menu_is_empty(menu: *mut menu) -> bool;
}
extern "C" {
    pub fn menu_is_visible(menu: *mut menu) -> bool;
}
extern "C" {
    pub fn menu_has_prompt(menu: *const menu) -> bool;
}
extern "C" {
    pub fn get_jump_key_char() -> c_int;
}
extern "C" {
    pub fn get_relations_str(sym_arr: *mut symbol, head: *mut list_head) -> gstr;
}
extern "C" {
    pub fn menu_get_ext_help(menu: *mut menu, help: *mut gstr);
}
extern "C" {
    pub fn menu_dump();
}
// symbol.c
extern "C" {
    pub fn sym_clear_all_valid();
}
// A choice is a symbol with no name
extern "C" {
    pub fn sym_is_choice_value(sym: *const symbol) -> bool;
}

