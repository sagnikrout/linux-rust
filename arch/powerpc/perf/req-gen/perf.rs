//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/req-gen/perf.h
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
// enumerate the request values as
// <NAME_UPPER>_<request name> = <request value>
//

//
// For each request:
// struct <NAME_LOWER>_<request name> {
// r_fields
// };
//

//
// Generate a check of the field offsets
// <NAME_LOWER>_assert_offsets_correct()
//

//
// Generate event attributes:
// PMU_EVENT_ATTR_STRING(<request name>_<field name>,
// <NAME_LOWER>_event_attr_<request name>_<field name>,
// "request=<request value>"
// "starting_index=<starting index type>"
// "counter_info_version=CURRENT_COUNTER_INFO_VERSION"
// "length=<f_size>"
// "offset=<f_offset>")
//
// TODO: counter_info_version may need to vary, we should interperate the
// value to some extent
//

//
// Define event attribute array
// static struct attribute *hv_gpci_event_attrs[] = {
// &<NAME_LOWER>_event_attr_<request name>_<field name>.attr,
// };
//

// Generate event list for platforms with counter_info_version 0x6 or below

//
// Based on getPerfCountInfo v1.018 documentation, some of the hv-gpci
// events were deprecated for platform firmware that supports
// counter_info_version 0x8 or above.
// Those deprecated events are still part of platform firmware that
// support counter_info_version 0x6 and below. As per the getPerfCountInfo
// v1.018 documentation there is no counter_info_version 0x7.
// Undefining macro ENABLE_EVENTS_COUNTERINFO_V6, to disable the addition of
// deprecated events in "hv_gpci_event_attrs" attribute group, for platforms
// that supports counter_info_version 0x8 or above.
//

// Generate event list for platforms with counter_info_version 0x8 or above

// cleanup

