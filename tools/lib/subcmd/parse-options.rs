//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/subcmd/parse-options.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_opt_type {
// special types
    OPTION_END,
    OPTION_ARGUMENT,
    OPTION_GROUP,
// options with no arguments
    OPTION_BIT,
    OPTION_BOOLEAN,
    OPTION_INCR,
    OPTION_SET_UINT,
    OPTION_SET_PTR,
// options with arguments (usually)
    OPTION_STRING,
    OPTION_INTEGER,
    OPTION_LONG,
    OPTION_ULONG,
    OPTION_CALLBACK,
    OPTION_U64,
    OPTION_UINTEGER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_opt_flags {
    PARSE_OPT_KEEP_DASHDASH = 1,
    PARSE_OPT_STOP_AT_NON_OPTION = 2,
    PARSE_OPT_KEEP_ARGV0 = 4,
    PARSE_OPT_KEEP_UNKNOWN = 8,
    PARSE_OPT_NO_INTERNAL_HELP = 16,
    PARSE_OPT_OPTARG_ALLOW_NEXT = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_opt_option_flags {
    PARSE_OPT_OPTARG  = 1,
    PARSE_OPT_NOARG   = 2,
    PARSE_OPT_NONEG   = 4,
    PARSE_OPT_HIDDEN  = 8,
    PARSE_OPT_LASTARG_DEFAULT = 16,
    PARSE_OPT_DISABLED = 32,
    PARSE_OPT_EXCLUSIVE = 64,
    PARSE_OPT_NOEMPTY  = 128,
    PARSE_OPT_NOBUILD  = 256,
    PARSE_OPT_CANSKIP  = 512,
    PARSE_OPT_NOAUTONEG = 1024,
}

extern "C" {
    pub fn parse_opt_cb(: *const option, arg: *const c_char, unset: c_int) -> typedef int;
}
//
// `type`::
// holds the type of the option, you must have an OPTION_END last in your
// array.
//
// `short_name`::
// the character to use as a short option name, '\0' if none.
//
// `long_name`::
// the long option name, without the leading dashes, NULL if none.
//
// `value`::
// stores pointers to the values to be filled.
//
// `argh`::
// token to explain the kind of argument this option wants. Keep it
// homogeneous across the repository.
//
// `help`::
// the short help associated to what the option does.
// Must never be NULL (except for OPTION_END).
// OPTION_GROUP uses this pointer to store the group header.
//
// `flags`::
// mask of parse_opt_option_flags.
// PARSE_OPT_OPTARG: says that the argument is optional (not for BOOLEANs)
// PARSE_OPT_NOARG: says that this option takes no argument, for CALLBACKs
// PARSE_OPT_NONEG: says that this option cannot be negated
// PARSE_OPT_HIDDEN this option is skipped in the default usage, showed in
// the long one.
//
// `callback`::
// pointer to the callback to use for OPTION_CALLBACK.
//
// `defval`::
// default value to fill (*->value) with for PARSE_OPT_OPTARG.
// OPTION_{BIT,SET_UINT,SET_PTR} store the {mask,integer,pointer} to put in
// the value when met.
// CALLBACKS can use it like they want.
//
// `set`::
// whether an option was set by the user
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct option {
    pub type: parse_opt_type,
    pub short_name: c_int,
    pub long_name: *const c_char,
    pub value: *mut c_void,
    pub argh: *const c_char,
    pub help: *const c_char,
    pub build_opt: *const c_char,
    pub flags: c_int,
    pub callback: *mut parse_opt_cb,
    pub defval: intptr_t,
    pub set: *mut bool,
    pub data: *mut c_void,
    pub parent: *const option,
}

// parse_options() will filter out the processed options and leave the
// non-option argments in argv[].
// Returns the number of arguments left in argv[].
//
// NOTE: parse_options() and parse_options_subcommand() may call exit() in the
// case of an error (or for 'special' options like --list-cmds or --list-opts).
//
// ----- incremantal advanced APIs -----
//
// It's okay for the caller to consume argv/argc in the usual way.
// Other fields of that structure are private to parse-options and should not
// be modified in any way.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_opt_ctx_t {
    pub argv: *const c_char,
    pub out: *const c_char,
    pub cpidx: int argc,,
    pub opt: *const c_char,
    pub excl_opt: *const option,
    pub flags: c_int,
}

// ----- some often used options -----
extern "C" {
    pub fn parse_opt_abbrev_cb(: *const option, : *const c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn parse_opt_approxidate_cb(: *const option, : *const c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn parse_opt_verbosity_cb(: *const option, : *const c_char, _arg: c_int) -> c_int;
}

extern "C" {
    pub fn set_option_flag(opts: *mut option, sopt: c_int, lopt: *const c_char, flag: c_int);
}
