//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf_event_attr_fprintf.c
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
#[derive(Copy, Clone)]
pub struct bit_names {
    pub bit: c_int,
    pub name: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn __p_bits(buf: *mut c_char, size: usize, value: u64, bits: *mut bit_names) {
    static void __p_bits(char *buf, size_t size, u64 value, struct bit_names *bits)
    {
    let mut first_bit: bool = true;
    let mut i: c_int = 0;
    do {
    if (value & bits[i].bit) {
    buf += scnprintf(buf, size, "%s%s", first_bit ? "" : "|", bits[i].name);
    first_bit = false;
    }
    } while (bits[++i].name != core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn __p_sample_type(buf: *mut c_char, size: usize, value: u64) {
    static void __p_sample_type(char *buf, size_t size, u64 value)
    {

    struct bit_names bits[] = {
    bit_name(IP), bit_name(TID), bit_name(TIME), bit_name(ADDR),
    bit_name(READ), bit_name(CALLCHAIN), bit_name(ID), bit_name(CPU),
    bit_name(PERIOD), bit_name(STREAM_ID), bit_name(RAW),
    bit_name(BRANCH_STACK), bit_name(REGS_USER), bit_name(STACK_USER),
    bit_name(IDENTIFIER), bit_name(REGS_INTR), bit_name(DATA_SRC),
    bit_name(WEIGHT), bit_name(PHYS_ADDR), bit_name(AUX),
    bit_name(CGROUP), bit_name(DATA_PAGE_SIZE), bit_name(CODE_PAGE_SIZE),
    bit_name(WEIGHT_STRUCT),
    { .name = core::ptr::null_mut(), }
    };

    __p_bits(buf, size, value, bits);
    }
#[no_mangle]
unsafe extern "C" fn __p_branch_sample_type(buf: *mut c_char, size: usize, value: u64) {
    static void __p_branch_sample_type(char *buf, size_t size, u64 value)
    {

    struct bit_names bits[] = {
    bit_name(USER), bit_name(KERNEL), bit_name(HV), bit_name(ANY),
    bit_name(ANY_CALL), bit_name(ANY_RETURN), bit_name(IND_CALL),
    bit_name(ABORT_TX), bit_name(IN_TX), bit_name(NO_TX),
    bit_name(COND), bit_name(CALL_STACK), bit_name(IND_JUMP),
    bit_name(CALL), bit_name(NO_FLAGS), bit_name(NO_CYCLES),
    bit_name(TYPE_SAVE), bit_name(HW_INDEX), bit_name(PRIV_SAVE),
    bit_name(COUNTERS),
    { .name = core::ptr::null_mut(), }
    };

    __p_bits(buf, size, value, bits);
    }
#[no_mangle]
unsafe extern "C" fn __p_read_format(buf: *mut c_char, size: usize, value: u64) {
    static void __p_read_format(char *buf, size_t size, u64 value)
    {

    struct bit_names bits[] = {
    bit_name(TOTAL_TIME_ENABLED), bit_name(TOTAL_TIME_RUNNING),
    bit_name(ID), bit_name(GROUP), bit_name(LOST),
    { .name = core::ptr::null_mut(), }
    };

    __p_bits(buf, size, value, bits);
    }

    static const char *stringify_perf_type_id(struct perf_pmu *pmu, u32 type)
    {
    switch (type) {
    ENUM_ID_TO_STR_CASE(PERF_TYPE_HARDWARE)
    ENUM_ID_TO_STR_CASE(PERF_TYPE_SOFTWARE)
    ENUM_ID_TO_STR_CASE(PERF_TYPE_TRACEPOINT)
    ENUM_ID_TO_STR_CASE(PERF_TYPE_HW_CACHE)
    ENUM_ID_TO_STR_CASE(PERF_TYPE_BREAKPOINT)
    case PERF_TYPE_RAW:
    return pmu ? pmu.name : "PERF_TYPE_RAW";
    default:
    return pmu ? pmu.name : core::ptr::null_mut();
    }
    }
    static const char *stringify_perf_hw_id(u64 value)
    {
    switch (value & PERF_HW_EVENT_MASK) {
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CPU_CYCLES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_INSTRUCTIONS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_REFERENCES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_MISSES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_BRANCH_INSTRUCTIONS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_BRANCH_MISSES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_BUS_CYCLES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_STALLED_CYCLES_FRONTEND)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_STALLED_CYCLES_BACKEND)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_REF_CPU_CYCLES)
    default:
    return core::ptr::null_mut();
    }
    }
    static const char *stringify_perf_hw_cache_id(u64 value)
    {
    switch (value) {
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_L1D)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_L1I)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_LL)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_DTLB)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_ITLB)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_BPU)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_NODE)
    default:
    return core::ptr::null_mut();
    }
    }
    static const char *stringify_perf_hw_cache_op_id(u64 value)
    {
    switch (value) {
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_OP_READ)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_OP_WRITE)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_OP_PREFETCH)
    default:
    return core::ptr::null_mut();
    }
    }
    static const char *stringify_perf_hw_cache_op_result_id(u64 value)
    {
    switch (value) {
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_RESULT_ACCESS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_HW_CACHE_RESULT_MISS)
    default:
    return core::ptr::null_mut();
    }
    }
    static const char *stringify_perf_sw_id(u64 value)
    {
    switch (value) {
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_CPU_CLOCK)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_TASK_CLOCK)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_PAGE_FAULTS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_CONTEXT_SWITCHES)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_CPU_MIGRATIONS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_PAGE_FAULTS_MIN)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_PAGE_FAULTS_MAJ)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_ALIGNMENT_FAULTS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_EMULATION_FAULTS)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_DUMMY)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_BPF_OUTPUT)
    ENUM_ID_TO_STR_CASE(PERF_COUNT_SW_CGROUP_SWITCHES)
    default:
    return core::ptr::null_mut();
    }
    }

#[no_mangle]
unsafe extern "C" fn print_id_unsigned(buf: *mut c_char, size: usize, value: u64, s: *const c_char) {
    static void print_id_unsigned(char *buf, size_t size, u64 value, const char *s)
    {
    if (s == core::ptr::null_mut())
    snprintf(buf, size, "%"PRIu64, value);
    else
    snprintf(buf, size, "%"PRIu64" (%s)", value, s);
    }
#[no_mangle]
unsafe extern "C" fn print_id_hex(buf: *mut c_char, size: usize, value: u64, s: *const c_char) {
    static void print_id_hex(char *buf, size_t size, u64 value, const char *s)
    {
    if (s == core::ptr::null_mut())
    snprintf(buf, size, "%#"PRIx64, value);
    else
    snprintf(buf, size, "%#"PRIx64" (%s)", value, s);
    }
#[no_mangle]
unsafe extern "C" fn __p_type_id(buf: *mut c_char, size: usize, pmu: *mut perf_pmu, type: u32) {
    static void __p_type_id(char *buf, size_t size, struct perf_pmu *pmu, u32 type)
    {
    print_id_unsigned(buf, size, type, stringify_perf_type_id(pmu, type));
    }
#[no_mangle]
unsafe extern "C" fn __p_config_hw_id(buf: *mut c_char, size: usize, pmu: *mut perf_pmu, config: u64) {
    static void __p_config_hw_id(char *buf, size_t size, struct perf_pmu *pmu, u64 config)
    {
    const char *name = stringify_perf_hw_id(config);
    if (name == core::ptr::null_mut()) {
    if (pmu == core::ptr::null_mut()) {
    snprintf(buf, size, "%#"PRIx64, config);
    } else {
    snprintf(buf, size, "%#"PRIx64" (%s/config=%#"PRIx64"/)", config, pmu.name,
    config);
    }
    } else {
    if (pmu == core::ptr::null_mut())
    snprintf(buf, size, "%#"PRIx64" (%s)", config, name);
    else
    snprintf(buf, size, "%#"PRIx64" (%s/%s/)", config, pmu.name, name);
    }
    }
#[no_mangle]
unsafe extern "C" fn __p_config_sw_id(buf: *mut c_char, size: usize, id: u64) {
    static void __p_config_sw_id(char *buf, size_t size, u64 id)
    {
    print_id_hex(buf, size, id, stringify_perf_sw_id(id));
    }
#[no_mangle]
unsafe extern "C" fn __p_config_hw_cache_id(buf: *mut c_char, size: usize, pmu: *mut perf_pmu, config: u64) {
    static void __p_config_hw_cache_id(char *buf, size_t size, struct perf_pmu *pmu, u64 config)
    {
    const char *hw_cache_str = stringify_perf_hw_cache_id(config & 0xff);
    const char *hw_cache_op_str =
    stringify_perf_hw_cache_op_id((config & 0xff00) >> 8);
    const char *hw_cache_op_result_str =
    stringify_perf_hw_cache_op_result_id((config & 0xff0000) >> 16);
    if (hw_cache_str == core::ptr::null_mut() || hw_cache_op_str == core::ptr::null_mut() || hw_cache_op_result_str == core::ptr::null_mut()) {
    if (pmu == core::ptr::null_mut()) {
    snprintf(buf, size, "%#"PRIx64, config);
    } else {
    snprintf(buf, size, "%#"PRIx64" (%s/config=%#"PRIx64"/)", config, pmu.name,
    config);
    }
    } else {
    if (pmu == core::ptr::null_mut()) {
    snprintf(buf, size, "%#"PRIx64" (%s | %s | %s)", config,
    hw_cache_op_result_str, hw_cache_op_str, hw_cache_str);
    } else {
    snprintf(buf, size, "%#"PRIx64" (%s/%s | %s | %s/)", config, pmu.name,
    hw_cache_op_result_str, hw_cache_op_str, hw_cache_str);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __p_config_tracepoint_id(buf: *mut c_char, size: usize, id: u64) {
    static void __p_config_tracepoint_id(char *buf, size_t size, u64 id)
    {
    char *str = tracepoint_id_to_name(id);
    print_id_hex(buf, size, id, str);
    free(str);
    }
#[no_mangle]
unsafe extern "C" fn __p_config_id(pmu: *mut perf_pmu, buf: *mut c_char, size: usize, type: u32, config: u64) {
    static void __p_config_id(struct perf_pmu *pmu, char *buf, size_t size, u32 type, u64 config)
    {
    switch (type) {
    case PERF_TYPE_HARDWARE:
    return __p_config_hw_id(buf, size, pmu, config);
    case PERF_TYPE_SOFTWARE:
    return __p_config_sw_id(buf, size, config);
    case PERF_TYPE_HW_CACHE:
    return __p_config_hw_cache_id(buf, size, pmu, config);
    case PERF_TYPE_TRACEPOINT:
    return __p_config_tracepoint_id(buf, size, config);
    case PERF_TYPE_RAW:
    case PERF_TYPE_BREAKPOINT:
    default:
    return print_id_hex(buf, size, config, perf_pmu__name_from_config(pmu, config));
    }
    }
pub const BUF_SIZE: c_int = 1024;

    do {									\
    if (attr_size >= offsetof(struct perf_event_attr, _f) +		\
    sizeof(attr._f) &&				\
    (_a || attr._f)) {						\
    _p(attr._f);						\
    ret += attr__fprintf(fp, _n, buf, priv);		\
    }								\
    } while (0)
// bitfield members share an offset; most are within PERF_ATTR_SIZE_VER0

    do {									\
    if (_a || attr._f) {						\
    _p(attr._f);						\
    ret += attr__fprintf(fp, _n, buf, priv);		\
    }								\
    } while (0)

    int perf_event_attr__fprintf(FILE *fp, struct perf_event_attr *attr,
    attr__fprintf_f attr__fprintf, void *priv)
    {
    struct perf_pmu *pmu = perf_pmus__find_by_type(attr.type);
//
// size == 0 means ABI0 — the producer didn't set attr.size.
// perf_event__fprintf_attr() may pass the raw mmap'd event
// before the local copy, so default to PERF_ATTR_SIZE_VER0
// (the ABI0 footprint) to avoid reading past the attr into
// the ID array that follows it in HEADER_ATTR events.
//
    let mut attr_size: u32 = attr.size ?: PERF_ATTR_SIZE_VER0;
    char buf[BUF_SIZE];
    let mut ret: c_int = 0;
//
// Cap to what we understand: all callers store the attr in a
// buffer of sizeof(*attr) bytes (perf.data read path copies
// min(attr.size, sizeof), BPF augmented path copies into a
// fixed-size value[] array).  A spoofed attr->size larger
// than sizeof would cause PRINT_ATTRn to read past the
// actual buffer.
//
    if (attr_size > sizeof(*attr))
    attr_size = sizeof(*attr);
    if (!pmu && attr_size >= offsetof(struct perf_event_attr, config) + sizeof(attr.config) &&
    (attr.type == PERF_TYPE_HARDWARE || attr.type == PERF_TYPE_HW_CACHE)) {
    let mut extended_type: u32 = attr.config >> PERF_PMU_TYPE_SHIFT;
    if (extended_type)
    pmu = perf_pmus__find_by_type(extended_type);
    }
    PRINT_ATTRn("type", type, p_type_id, true);
    PRINT_ATTRf(size, p_unsigned);
    PRINT_ATTRn("config", config, p_config_id, true);
    PRINT_ATTRn("{ sample_period, sample_freq }", sample_period, p_unsigned, false);
    PRINT_ATTRf(sample_type, p_sample_type);
    PRINT_ATTRf(read_format, p_read_format);
//
// All bitfields share a single __u64 right after read_format.
// BPF-captured attrs from perf trace may have a small size
// when the tracee passes a minimal struct, so skip the
// entire block when it's not covered.
//
    if (attr_size >= offsetof(struct perf_event_attr, wakeup_events)) {
    PRINT_ATTRf_bf(disabled, p_unsigned);
    PRINT_ATTRf_bf(inherit, p_unsigned);
    PRINT_ATTRf_bf(pinned, p_unsigned);
    PRINT_ATTRf_bf(exclusive, p_unsigned);
    PRINT_ATTRf_bf(exclude_user, p_unsigned);
    PRINT_ATTRf_bf(exclude_kernel, p_unsigned);
    PRINT_ATTRf_bf(exclude_hv, p_unsigned);
    PRINT_ATTRf_bf(exclude_idle, p_unsigned);
    PRINT_ATTRf_bf(mmap, p_unsigned);
    PRINT_ATTRf_bf(comm, p_unsigned);
    PRINT_ATTRf_bf(freq, p_unsigned);
    PRINT_ATTRf_bf(inherit_stat, p_unsigned);
    PRINT_ATTRf_bf(enable_on_exec, p_unsigned);
    PRINT_ATTRf_bf(task, p_unsigned);
    PRINT_ATTRf_bf(watermark, p_unsigned);
    PRINT_ATTRf_bf(precise_ip, p_unsigned);
    PRINT_ATTRf_bf(mmap_data, p_unsigned);
    PRINT_ATTRf_bf(sample_id_all, p_unsigned);
    PRINT_ATTRf_bf(exclude_host, p_unsigned);
    PRINT_ATTRf_bf(exclude_guest, p_unsigned);
    PRINT_ATTRf_bf(exclude_callchain_kernel, p_unsigned);
    PRINT_ATTRf_bf(exclude_callchain_user, p_unsigned);
    PRINT_ATTRf_bf(mmap2, p_unsigned);
    PRINT_ATTRf_bf(comm_exec, p_unsigned);
    PRINT_ATTRf_bf(use_clockid, p_unsigned);
    PRINT_ATTRf_bf(context_switch, p_unsigned);
    PRINT_ATTRf_bf(write_backward, p_unsigned);
    PRINT_ATTRf_bf(namespaces, p_unsigned);
    PRINT_ATTRf_bf(ksymbol, p_unsigned);
    PRINT_ATTRf_bf(bpf_event, p_unsigned);
    PRINT_ATTRf_bf(aux_output, p_unsigned);
    PRINT_ATTRf_bf(cgroup, p_unsigned);
    PRINT_ATTRf_bf(text_poke, p_unsigned);
    PRINT_ATTRf_bf(build_id, p_unsigned);
    PRINT_ATTRf_bf(inherit_thread, p_unsigned);
    PRINT_ATTRf_bf(remove_on_exec, p_unsigned);
    PRINT_ATTRf_bf(sigtrap, p_unsigned);
    PRINT_ATTRf_bf(defer_callchain, p_unsigned);
    PRINT_ATTRf_bf(defer_output, p_unsigned);
    }
    PRINT_ATTRn("{ wakeup_events, wakeup_watermark }", wakeup_events, p_unsigned, false);
    PRINT_ATTRf(bp_type, p_unsigned);
    PRINT_ATTRn("{ bp_addr, config1 }", bp_addr, p_hex, false);
    PRINT_ATTRn("{ bp_len, config2 }", bp_len, p_hex, false);
    PRINT_ATTRf(branch_sample_type, p_branch_sample_type);
    PRINT_ATTRf(sample_regs_user, p_hex);
    PRINT_ATTRf(sample_stack_user, p_unsigned);
    PRINT_ATTRf(clockid, p_signed);
    PRINT_ATTRf(sample_regs_intr, p_hex);
    PRINT_ATTRf(aux_watermark, p_unsigned);
    PRINT_ATTRf(sample_max_stack, p_unsigned);
    PRINT_ATTRf(aux_sample_size, p_unsigned);
    PRINT_ATTRf(sig_data, p_unsigned);
// aux_{start_paused,pause,resume} are at byte 116, past VER0
    if (attr_size >= offsetof(struct perf_event_attr, sig_data)) {
    PRINT_ATTRf_bf(aux_start_paused, p_unsigned);
    PRINT_ATTRf_bf(aux_pause, p_unsigned);
    PRINT_ATTRf_bf(aux_resume, p_unsigned);
    }
    return ret;
    }
