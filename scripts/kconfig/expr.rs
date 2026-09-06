//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/expr.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum expr_type {
    E_NONE, E_OR, E_AND, E_NOT,
    E_EQUAL, E_UNEQUAL, E_LTH, E_LEQ, E_GTH, E_GEQ,
    E_SYMBOL, E_RANGE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union expr_data {
    pub expr: *const *const expr,
    pub sym: *const *const symbol,
    pub _initdata: *mut c_void,
}

//
// struct expr - expression
//
// @node:  link node for the hash table
// @type:  expression type
// @val: calculated tristate value
// @val_is_valid: indicate whether the value is valid
// @left:  left node
// @right: right node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr {
    pub node: hlist_node,
    pub type: expr_type,
    pub val: tristate,
    pub val_is_valid: bool,
    pub right: expr_data left,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr_value {
    pub expr: *mut expr,
    pub tri: tristate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol_value {
    pub val: *mut c_void,
    pub tri: tristate,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_type {
    S_UNKNOWN, S_BOOLEAN, S_TRISTATE, S_INT, S_HEX, S_STRING
}

// enum values are used as index to symbol.def[]
//
// Represents a configuration symbol.
//
// Choices are represented as a special kind of symbol with null name.
//
// @choice_link: linked to menu::choice_members
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol {
// link node for the hash table
    pub node: hlist_node,
// The name of the symbol, e.g. "FOO" for 'config FOO'
    pub name: *mut c_char,
// S_BOOLEAN, S_TRISTATE, ...
    pub type: symbol_type,
//
// The calculated value of the symbol. The SYMBOL_VALID bit is set in
// 'flags' when this is up to date. Note that this value might differ
// from the user value set in e.g. a .config file, due to visibility.
//
    pub curr: symbol_value,
//
// Values for the symbol provided from outside. def[S_DEF_USER] holds
// the .config value.
//
    pub def: [symbol_value; S_DEF_COUNT],
//
// An upper bound on the tristate value the user can set for the symbol
// if it is a boolean or tristate. Calculated from prompt dependencies,
// which also inherit dependencies from enclosing menus, choices, and
// ifs. If 'n', the user value will be ignored.
//
// Symbols lacking prompts always have visibility 'n'.
//
    pub visible: tristate,
// config entries associated with this symbol
    pub menus: list_head,
    pub choice_link: list_head,
// SYMBOL_* flags
    pub flags: c_int,
// List of properties. See prop_type.
    pub prop: *mut property,
// Dependencies from enclosing menus, choices, and ifs
    pub dir_dep: expr_value,
// Reverse dependencies through being selected by other symbols
    pub rev_dep: expr_value,
//
// "Weak" reverse dependencies through being implied by other symbols
//
    pub implied: expr_value,
}

pub const SYMBOL_CONST: c_uint = 0x0001  /* symbol is const */;
pub const SYMBOL_CHECK: c_uint = 0x0008  /* used during dependency checking */;
pub const SYMBOL_VALID: c_uint = 0x0080  /* set when symbol.curr is calculated */;
pub const SYMBOL_TRANS: c_uint = 0x0100  /* symbol is transitional only (not visible)*/;
pub const SYMBOL_WRITE: c_uint = 0x0200  /* write symbol to file (KCONFIG_CONFIG) */;
pub const SYMBOL_WRITTEN: c_uint = 0x0800  /* track info to avoid double-write to .config */;
pub const SYMBOL_CHECKED: c_uint = 0x2000  /* used during dependency checking */;
pub const SYMBOL_WARNED: c_uint = 0x8000  /* warning has been issued */;
// Set when symbol.def[] is used
pub const SYMBOL_DEF: c_uint = 0x10000  /* First bit of SYMBOL_DEF */;
pub const SYMBOL_DEF_USER: c_uint = 0x10000  /* symbol.def[S_DEF_USER] is valid */;
pub const SYMBOL_DEF_AUTO: c_uint = 0x20000  /* symbol.def[S_DEF_AUTO] is valid */;
pub const SYMBOL_DEF3: c_uint = 0x40000  /* symbol.def[S_DEF_3] is valid */;
pub const SYMBOL_DEF4: c_uint = 0x80000  /* symbol.def[S_DEF_4] is valid */;
pub const SYMBOL_MAXLENGTH: c_int = 256;
// A property represents the config options that can be associated
// with a config "symbol".
// Sample:
// config FOO
// default y
// prompt "foo prompt"
// select BAR
// config BAZ
// int "BAZ Value"
// range 1..255
//
// Please, also check parser.y:print_symbol() when modifying the
// list of property types!
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prop_type {
    P_UNKNOWN,
    P_PROMPT,   /* prompt "foo prompt" or "BAZ Value" */
    P_COMMENT,  /* text associated with a comment */
    P_MENU,     /* prompt associated with a menu or menuconfig symbol */
    P_DEFAULT,  /* default y */
    P_SELECT,   /* select BAR */
    P_IMPLY,    /* imply BAR */
    P_RANGE,    /* range 7..100 (for a symbol) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct property {
    pub /: *mut *mut *mut property next; / next property - null if last,
    pub /: *mut *mut prop_type type; / type of property,
    pub /: *const *const *const char text; / the prompt value - P_PROMPT, P_MENU, P_COMMENT,
    pub visible: expr_value,
    pub /: *mut *mut *mut expr expr; / the optional conditional part of the property,
    pub with: *mut *mut *mut menu menu; / the menu the property are associated,
// valid for: P_SELECT, P_RANGE,
// P_PROMPT, P_DEFAULT, P_MENU, P_COMMENT
    pub /: *const *const *const char filename; / what file was this property defined,
    pub /: *mut *mut int lineno; / what lineno was this property defined,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum menu_type {
    M_CHOICE,  // "choice"
    M_COMMENT, // "comment"
    M_IF,      // "if"
    M_MENU,    // "mainmenu", "menu", "menuconfig"
    M_NORMAL,  // others, i.e., "config"
}

//
// Represents a node in the menu tree, as seen in e.g. menuconfig (though used
// for all front ends). Each symbol, menu, etc. defined in the Kconfig files
// gets a node. A symbol defined in multiple locations gets one node at each
// location.
//
// @type: type of the menu entry
// @choice_members: list of choice members with priority.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menu {
    pub type: menu_type,
// The next menu node at the same level
    pub next: *mut menu,
// The parent menu node, corresponding to e.g. a menu or choice
    pub parent: *mut menu,
// The first child menu node, for e.g. menus and choices
    pub list: *mut menu,
//
// The symbol associated with the menu node. Choices are implemented as
// a special kind of symbol. NULL for menus, comments, and ifs.
//
    pub sym: *mut symbol,
    pub /: *mut *mut list_head link; / link to symbol::menus,
    pub choice_members: list_head,
//
// The prompt associated with the node. This holds the prompt for a
// symbol as well as the text for a menu or comment, along with the
// type (P_PROMPT, P_MENU, etc.)
//
    pub prompt: *mut property,
//
// 'visible if' dependencies. If more than one is given, they will be
// ANDed together.
//
    pub visibility: *mut expr,
//
// Ordinary dependencies from e.g. 'depends on' and 'if', ANDed
// together
//
    pub dep: *mut expr,
// MENU_* flags
    pub flags: c_uint,
// Any help text associated with the node
    pub help: *mut c_char,
// The location where the menu node appears in the Kconfig files
    pub filename: *const c_char,
    pub lineno: c_int,
// For use by front ends that need to store auxiliary data
    pub data: *mut c_void,
}

//
// Set on a menu node when the corresponding symbol changes state in some way.
// Can be checked by front ends.
//
pub const MENU_CHANGED: c_uint = 0x0001;
pub const MENU_ROOT: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jump_key {
    pub entries: list_head,
    pub offset: usize,
    pub target: *mut menu,
}

extern "C" {
    pub fn expr_eliminate_eq(ep1: *mut expr, ep2: *mut expr);
}
extern "C" {
    pub fn expr_eq(e1: *mut expr, e2: *mut expr) -> bool;
}
extern "C" {
    pub fn expr_calc_value(e: *mut expr) -> tristate;
}
extern "C" {
    pub fn expr_contains_symbol(dep: *mut expr, sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn expr_contains_symbol_negated(dep: *mut expr, sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn expr_depends_symbol(dep: *mut expr, sym: *mut symbol) -> bool;
}
extern "C" {
    pub fn expr_fprint(e: *mut expr, out: *mut FILE);
}
extern "C" {
    pub fn expr_gstr_print(e: *const expr, gs: *mut gstr);
}

