//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/parse-events.h
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
// Parse symbolic events/counts passed in as options:
//

// Arguments encoded in opt->value.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_option_args {
    pub evlistp: *mut evlist,
    pub pmu_filter: *const c_char,
    pub cputype_filter: bool,
}

extern "C" {
    pub fn parse_events_option(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn parse_events_option_new_evlist(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
// cputype_filter=*/false, err, /*fake_pmu=*/false,
// warn_if_reordered=*/true,
// fake_tp=*/false);
extern "C" {
    pub fn parse_event(evlist: *mut evlist, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn parse_filter(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn parse_uid_filter(evlist: *mut evlist, uid: uid_t) -> c_int;
}
extern "C" {
    pub fn exclude_perf(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_events__term_val_type {
    PARSE_EVENTS__TERM_TYPE_NUM,
    PARSE_EVENTS__TERM_TYPE_STR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_events__term_type {
    PARSE_EVENTS__TERM_TYPE_USER,
    PARSE_EVENTS__TERM_TYPE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_CONFIG1,
    PARSE_EVENTS__TERM_TYPE_CONFIG2,
    PARSE_EVENTS__TERM_TYPE_CONFIG3,
    PARSE_EVENTS__TERM_TYPE_CONFIG4,
    PARSE_EVENTS__TERM_TYPE_NAME,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_PERIOD,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_FREQ,
    PARSE_EVENTS__TERM_TYPE_BRANCH_SAMPLE_TYPE,
    PARSE_EVENTS__TERM_TYPE_TIME,
    PARSE_EVENTS__TERM_TYPE_CALLGRAPH,
    PARSE_EVENTS__TERM_TYPE_STACKSIZE,
    PARSE_EVENTS__TERM_TYPE_NOINHERIT,
    PARSE_EVENTS__TERM_TYPE_INHERIT,
    PARSE_EVENTS__TERM_TYPE_MAX_STACK,
    PARSE_EVENTS__TERM_TYPE_MAX_EVENTS,
    PARSE_EVENTS__TERM_TYPE_NOOVERWRITE,
    PARSE_EVENTS__TERM_TYPE_OVERWRITE,
    PARSE_EVENTS__TERM_TYPE_DRV_CFG,
    PARSE_EVENTS__TERM_TYPE_PERCORE,
    PARSE_EVENTS__TERM_TYPE_AUX_OUTPUT,
    PARSE_EVENTS__TERM_TYPE_AUX_ACTION,
    PARSE_EVENTS__TERM_TYPE_AUX_SAMPLE_SIZE,
    PARSE_EVENTS__TERM_TYPE_METRIC_ID,
    PARSE_EVENTS__TERM_TYPE_RAW,
    PARSE_EVENTS__TERM_TYPE_CPU,
    PARSE_EVENTS__TERM_TYPE_RATIO_TO_PREV,
    PARSE_EVENTS__TERM_TYPE_LEGACY_HARDWARE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_LEGACY_CACHE_CONFIG,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_term {
// @list: The term list the term is a part of.
    pub list: list_head,
//
// @config: The left-hand side of a term assignment, so the term
// "event=8" would have the config be "event"
//
    pub config: *const c_char,
//
// @val: The right-hand side of a term assignment that can either be a
// string or a number depending on type_val.
//
    pub str: *mut c_char,
    pub num: u64,
    pub val: },
// @type_val: The union variable in val to be used for the term.
    pub type_val: parse_events__term_val_type,
//
// @type_term: A predefined term type or PARSE_EVENTS__TERM_TYPE_USER
// when not inbuilt.
//
    pub type_term: parse_events__term_type,
//
// @err_term: The column index of the term from parsing, used during
// error output.
//
    pub err_term: c_int,
//
// @err_val: The column index of the val from parsing, used during error
// output.
//
    pub err_val: c_int,
// @used: Was the term used during parameterized-eval.
    pub used: bool,
//
// @weak: A term from the sysfs or json encoding of an event that
// shouldn't override terms coming from the command line.
//
    pub weak: bool,
//
// @no_value: Is there no value. If a numeric term has no value then the
// value is assumed to be 1. An event name also has no value.
//
    pub no_value: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_error {
// @list: The head of a list of errors.
    pub list: list_head,
}

// A wrapper around a list of terms for the sake of better type safety.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_terms {
    pub terms: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_state {
// The list parsed events are placed on.
    pub list: list_head,
// The updated index used by entries as they are added.
    pub idx: c_int,
// Error information.
    pub error: *mut parse_events_error,
// Holds returned terms for term parsing.
    pub terms: *mut parse_events_terms,
// Start token.
    pub stoken: c_int,
// Use the fake PMU marker for testing.
    pub fake_pmu: bool,
// Skip actual tracepoint processing for testing.
    pub fake_tp: bool,
// If non-null, when wildcard matching only match the given PMU.
    pub pmu_filter: *const c_char,
// If true, the pmu_filter was set by --cputype option.
    pub cputype_filter: bool,
// Should PE_LEGACY_NAME tokens be generated for config terms?
    pub match_legacy_cache_terms: bool,
// Were multiple PMUs scanned to find events?
    pub wild_card_pmus: bool,
}

extern "C" {
    pub fn parse_events__shrink_config_terms();
}
extern "C" {
    pub fn parse_events__is_hardcoded_term(term: *mut parse_events_term) -> c_int;
}
extern "C" {
    pub fn parse_events_term__delete(term: *mut parse_events_term);
}
extern "C" {
    pub fn parse_events_terms__delete(terms: *mut parse_events_terms);
}
extern "C" {
    pub fn parse_events_terms__init(terms: *mut parse_events_terms);
}
extern "C" {
    pub fn parse_events_terms__exit(terms: *mut parse_events_terms);
}
extern "C" {
    pub fn parse_events_terms(terms: *mut parse_events_terms, str: *const c_char) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_modifier {
    pub /: *mut *mut u8 precise; / Number of repeated 'p' for precision.,
    pub /: *mut *mut bool precise_max : 1; / 'P',
    pub /: *mut *mut bool non_idle : 1; / 'I',
    pub /: *mut *mut bool sample_read : 1; / 'S',
    pub /: *mut *mut bool pinned : 1; / 'D',
    pub /: *mut *mut bool exclusive : 1; / 'e',
    pub /: *mut *mut bool weak : 1; / 'W',
    pub /: *mut *mut bool bpf : 1; / 'b',
    pub /: *mut *mut bool user : 1; / 'u',
    pub /: *mut *mut bool kernel : 1; / 'k',
    pub /: *mut *mut bool hypervisor : 1; / 'h',
    pub /: *mut *mut bool guest : 1; / 'G',
    pub /: *mut *mut bool host : 1; / 'H',
    pub /: *mut *mut bool retire_lat : 1; / 'R',
    pub /: *mut *mut bool dont_regroup : 1; / 'X',
}

extern "C" {
    pub fn parse_events__set_default_name(list: *mut list_head, name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn parse_events__decode_legacy_cache(name: *const c_char, pmu_type: c_int, config: *mut __u64) -> c_int;
}
extern "C" {
    pub fn parse_events__set_leader(name: *mut c_char, list: *mut list_head);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_symbol {
    pub symbol: *const c_char,
    pub alias: *const c_char,
}

extern "C" {
    pub fn parse_events_error__init(err: *mut parse_events_error);
}
extern "C" {
    pub fn parse_events_error__exit(err: *mut parse_events_error);
}

//
// If the probe point starts with '%',
// or starts with "sdt_" and has a ':' but no '=',
// then it should be a SDT/cached probe point.
//

extern "C" {
    pub fn default_breakpoint_len() -> usize;
}
