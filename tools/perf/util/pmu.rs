//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/pmu.h
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

pub const PERF_PMU_FORMAT_BITS: c_int = 64;
pub const MAX_PMU_NAME_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_caps {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmu_kind {
// A perf event syscall PMU.
    PERF_PMU_KIND_PE,
// A perf tool provided DRM PMU.
    PERF_PMU_KIND_DRM,
// A perf tool provided HWMON PMU.
    PERF_PMU_KIND_HWMON,
// Perf tool provided PMU for tool events like time.
    PERF_PMU_KIND_TOOL,
// A testing PMU kind.
    PERF_PMU_KIND_FAKE
}

//
// struct perf_pmu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu {
// @name: The name of the PMU such as "cpu".
    pub name: *const c_char,
//
// @alias_name: Optional alternate name for the PMU determined in
// architecture specific code.
//
    pub alias_name: *mut c_char,
//
// @id: Optional PMU identifier read from
// <sysfs>/bus/event_source/devices/<name>/identifier.
//
    pub id: *const c_char,
//
// @type: Perf event attributed type value, read from
// <sysfs>/bus/event_source/devices/<name>/type.
//
    pub type: __u32,
//
// @selectable: Can the PMU name be selected as if it were an event?
//
    pub selectable: bool,
//
// @is_core: Is the PMU the core CPU PMU? Determined by the name being
// "cpu" or by the presence of
// <sysfs>/bus/event_source/devices/<name>/cpus. There may be >1 core
// PMU on systems like Intel hybrid.
//
    pub is_core: bool,
//
// @is_uncore: Is the PMU not within the CPU core? Determined by the
// presence of <sysfs>/bus/event_source/devices/<name>/cpumask.
//
    pub is_uncore: bool,
//
// @auxtrace: Are events auxiliary events? Determined in architecture
// specific code.
//
    pub auxtrace: bool,
//
// @formats_checked: Only check PMU's formats are valid for
// perf_event_attr once.
//
    pub formats_checked: bool,
// @config_masks_present: Are there config format values?
    pub config_masks_present: bool,
// @config_masks_computed: Set when masks are lazily computed.
    pub config_masks_computed: bool,
//
// @max_precise: Number of levels of :ppp precision supported by the
// PMU, read from
// <sysfs>/bus/event_source/devices/<name>/caps/max_precise.
//
    pub max_precise: c_int,
//
// @perf_event_attr_init_default: Optional function to default
// initialize PMU specific parts of the perf_event_attr.
//
    pub attr): *mut perf_event_attr,
//
// @cpus: Empty or the contents of either of:
// <sysfs>/bus/event_source/devices/<name>/cpumask.
// <sysfs>/bus/event_source/devices/<cpu>/cpus.
//
    pub cpus: *mut perf_cpu_map,
//
// @format: Holds the contents of files read from
// <sysfs>/bus/event_source/devices/<name>/format/. The contents specify
// which event parameter changes what config, config1 or config2 bits.
//
    pub format: list_head,
//
// @aliases: List of struct perf_pmu_alias. Each alias corresponds to an
// event read from <sysfs>/bus/event_source/devices/<name>/events/ or
// from json events in pmu-events.c.
//
    pub aliases: *mut hashmap,
//
// @events_table: The events table for json events in pmu-events.c.
//
    pub events_table: *const pmu_events_table,
// @sysfs_aliases: Number of sysfs aliases loaded.
    pub sysfs_aliases: u32,
// @cpu_json_aliases: Number of json event aliases loaded specific to the CPUID.
    pub cpu_json_aliases: u32,
// @sys_json_aliases: Number of json event aliases loaded matching the PMU's identifier.
    pub sys_json_aliases: u32,
//
// @cpu_common_json_aliases: Number of json events that overlapped with sysfs when
// loading all sysfs events.
//
    pub cpu_common_json_aliases: u32,
// @sysfs_aliases_loaded: Are sysfs aliases loaded from disk?
    pub sysfs_aliases_loaded: bool,
//
// @cpu_aliases_added: Have all json events table entries for the PMU
// been added?
//
    pub cpu_aliases_added: bool,
// @caps_initialized: Has the list caps been initialized?
    pub caps_initialized: bool,
// @nr_caps: The length of the list caps.
    pub nr_caps: u32,
//
// @caps: Holds the contents of files read from
// <sysfs>/bus/event_source/devices/<name>/caps/.
//
// The contents are pairs of the filename with the value of its
// contents, for example, max_precise (see above) may have a value of 3.
//
    pub caps: list_head,
// @list: Element on pmus list in pmu.c.
    pub list: list_head,
//
// @config_masks: Derived from the PMU's format data, bits that are
// valid within the config value.
//
    pub config_masks: [__u64; PERF_PMU_FORMAT_VALUE_CONFIG_END],
//
// @missing_features: Features to inhibit when events on this PMU are
// opened.
//
// @exclude_guest: Disables perf_event_attr exclude_guest and
// exclude_host.
//
    pub exclude_guest: bool,
//
// @checked: Are the missing features checked?
//
    pub checked: bool,
    pub missing_features: },
//
// @mem_events: List of the supported mem events
//
    pub mem_events: *mut perf_mem_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_info {
    pub unit: *const c_char,
    pub scale: double,
    pub retirement_latency_mean: double,
    pub retirement_latency_min: double,
    pub retirement_latency_max: double,
    pub per_pkg: bool,
    pub snapshot: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_event_info {
    pub pmu: *const perf_pmu,
    pub name: *const c_char,
    pub alias: *const *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub topic: *const c_char,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
    pub str: *const c_char,
    pub deprecated: bool,
}

//
// struct perf_pmu_format - Values from a format file read from
// <sysfs>/devices/cpu/format/ held in struct perf_pmu.
//
// For example, the contents of <sysfs>/devices/cpu/format/event may be
// "config:0-7" and will be represented here as name="event",
// value=PERF_PMU_FORMAT_VALUE_CONFIG and bits 0 to 7 will be set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_format {
// @list: Element on list within struct perf_pmu.
    pub list: list_head,
// @bits: Which config bits are set by this format value.
    pub PERF_PMU_FORMAT_BITS): DECLARE_BITMAP(bits,,
// @name: The modifier/file name.
    pub name: *mut c_char,
//
// @value : Which config value the format relates to. Supported values
// are from PERF_PMU_FORMAT_VALUE_CONFIG to
// PERF_PMU_FORMAT_VALUE_CONFIG_END.
//
    pub value: u16,
// @loaded: Has the contents been loaded/parsed.
    pub loaded: bool,
}

extern "C" {
    pub fn int(state: *mut *mut pmu_event_callback)(void, info: *mut pmu_event_info) -> typedef;
}
extern "C" {
    pub fn pmu_add_sys_aliases(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn perf_pmu__format_bits(pmu: *const perf_pmu, name: *const c_char) -> __u64;
}
extern "C" {
    pub fn perf_pmu__format_type(pmu: *const perf_pmu, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_pmu__find_event(pmu: *mut perf_pmu, event: *const c_char, state: *mut c_void, cb: pmu_event_callback) -> c_int;
}
extern "C" {
    pub fn perf_pmu_format__set_value(format: *mut c_void, config: c_int, bits: *mut c_ulong);
}
extern "C" {
    pub fn perf_pmu__has_format(pmu: *const perf_pmu, name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__for_each_format(pmu: *mut perf_pmu, state: *mut c_void, cb: pmu_format_callback) -> c_int;
}
extern "C" {
    pub fn perf_pmu__format_unpack(format: *mut c_ulong, config_val: u64) -> u64;
}
extern "C" {
    pub fn is_pmu_core(name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__supports_legacy_cache(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn perf_pmu__auto_merge_stats(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__num_events(pmu: *mut perf_pmu) -> usize;
}
extern "C" {
    pub fn perf_pmu__name_wildcard_match(pmu: *const perf_pmu, to_match: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__name_no_suffix_match(pmu: *const perf_pmu, to_match: *const c_char) -> bool;
}
//
// perf_pmu_is_software - is the PMU a software PMU as in it uses the
// perf_sw_context in the kernel?
//
extern "C" {
    pub fn perf_pmu__is_software(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn perf_pmu__benefits_from_affinity(pmu: *mut perf_pmu) -> bool;
}
extern "C" {
    pub fn perf_pmu__file_exists(pmu: *const perf_pmu, name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__test() -> c_int;
}
extern "C" {
    pub fn perf_pmu__arch_init(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn pmu_uncore_identifier_match(compat: *const c_char, id: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__convert_scale(scale: *const c_char, end: *mut c_char, sval: *mut double) -> c_int;
}
extern "C" {
    pub fn perf_pmu__caps_parse(pmu: *mut perf_pmu) -> c_int;
}
extern "C" {
    pub fn perf_pmu__warn_invalid_formats(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn perf_pmu__wildcard_match(pmu: *const perf_pmu, wildcard_to_match: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmu__event_source_devices_scnprintf(pathname: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn perf_pmu__event_source_devices_fd() -> c_int;
}
extern "C" {
    pub fn perf_pmu__pathname_fd(dirfd: c_int, pmu_name: *const c_char, filename: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn perf_pmu__init(pmu: *mut perf_pmu, type: __u32, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_pmu__delete(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn perf_pmu__is_fake(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn perf_pmu__reads_only_on_cpu_idx0(attr: *const perf_event_attr) -> bool;
}
