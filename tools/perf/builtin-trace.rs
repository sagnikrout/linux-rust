//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-trace.c
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


//
// builtin-trace.c
//
// Builtin 'trace' command:
//
// Display a continuously updated trace of any workload, CPU, specific PID,
// system wide, etc.  Default format is loosely strace like, but any other
// event may be specified using --event.
//
// Copyright (C) 2012, 2013, 2014, 2015 Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//
// Initially based on the 'trace' prototype by Thomas Gleixner:
//
// http://lwn.net/Articles/415728/ ("Announcing a new utility: 'trace'")
//

pub const RAW_SYSCALL_ARGS_NUM: c_int = 6;
//
// strtoul: Go from a string to a value, i.e. for msr: MSR_FS_BASE to 0xc0000100
//
// We have to explicitely mark the direction of the flow of data, if from the
// kernel to user space or the other way around, since the BPF collector we
// have so far copies only from user to kernel space, mark the arguments that
// go that direction, so that we don´t end up collecting the previous contents
// for syscall args that goes from kernel to user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_arg_fmt {
    pub arg): *mut *mut *mut size_t (scnprintf)(char bf, size_t size, struct syscall_arg,
    pub val): *mut *mut *mut *mut bool (strtoul)(char bf, size_t size, struct syscall_arg arg, u64,
    pub val): *mut *mut *mut unsigned long (mask_val)(struct syscall_arg arg, unsigned long,
    pub parm: *mut c_void,
    pub name: *const c_char,
    pub arrays: u16 nr_entries; // for,
    pub from_user: bool,
    pub show_zero: bool,

    pub type: *const btf_type,
    pub /: *mut *mut int type_id; / used in btf_dump,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_fmt {
    pub name: *const c_char,
    pub alias: *const c_char,
    struct {
    const char *sys_enter,
// sys_exit;
    pub bpf_prog_name: },
    pub arg: [syscall_arg_fmt; RAW_SYSCALL_ARGS_NUM],
    pub nr_args: u8,
    pub errpid: bool,
    pub timeout: bool,
    pub hexret: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace {
    pub host_env: perf_env,
    pub tool: perf_tool,
    struct {
// Sorted sycall numbers used by the trace.
    pub table: *mut syscall,
// Size of table.
    pub table_size: usize,
    struct {
    struct evsel *sys_enter,
// sys_exit,
// bpf_output;
    pub events: },
    pub syscalls: },

    pub btf: *mut btf,

    pub opts: record_opts,
    pub evlist: *mut evlist,
    pub host: *mut machine,
    pub current: *mut thread,
    pub cgroup: *mut cgroup,
    pub base_time: u64,
    pub output: *mut FILE,
    pub nr_events: c_ulong,
    pub nr_events_printed: c_ulong,
    pub max_events: c_ulong,
    pub evswitch: evswitch,
    pub ev_qualifier: *mut strlist,
    struct {
    pub nr: usize,
    pub entries: *mut c_int,
    pub ev_qualifier_ids: },
    struct {
    pub nr: usize,
    pub entries: *mut pid_t,
    pub map: *mut bpf_map,
    pub filter_pids: },
//
// TODO: The map is from an ID (aka system call number) to struct
// syscall_stats. If there is >1 e_machine, such as i386 and x86-64
// processes, then the stats here will gather wrong the statistics for
// the non EM_HOST system calls. A fix would be to add the e_machine
// into the key, but this would make the code inconsistent with the
// per-thread version.
//
    pub syscall_stats: *mut hashmap,
    pub duration_filter: double,
    pub runtime_ms: double,
    pub pfmin: unsigned long pfmaj,,
    struct {
    u64		vfs_getname,
    pub stats: },
    pub max_stack: c_uint,
    pub min_stack: c_uint,
    pub summary_mode: enum trace_summary_mode,
    pub max_summary: c_int,
    pub raw_augmented_syscalls_args_size: c_int,
    pub raw_augmented_syscalls: bool,
    pub fd_path_disabled: bool,
    pub sort_events: bool,
    pub not_ev_qualifier: bool,
    pub live: bool,
    pub full_time: bool,
    pub sched: bool,
    pub multiple_threads: bool,
    pub summary: bool,
    pub summary_only: bool,
    pub errno_summary: bool,
    pub failure_only: bool,
    pub show_comm: bool,
    pub print_sample: bool,
    pub show_tool_stats: bool,
    pub trace_syscalls: bool,
    pub libtraceevent_print: bool,
    pub kernel_syscallchains: bool,
    pub args_alignment: i16,
    pub show_tstamp: bool,
    pub show_cpu: bool,
    pub show_duration: bool,
    pub show_zeros: bool,
    pub show_arg_names: bool,
    pub show_string_prefix: bool,
    pub force: bool,
    pub vfs_getname: bool,
    pub force_btf: bool,
    pub bitmask_list: bool,
    pub summary_bpf: bool,
    pub trace_pgfaults: c_int,
    pub perfconfig_events: *mut c_char,
    struct {
    pub data: ordered_events,
    pub last: u64,
    pub oe: },
    pub uid_str: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn trace__show_zeros(trace: *const trace) -> bool {
    bool trace__show_zeros(const struct trace *trace)
    {
    return trace.show_zeros;
    }
    struct machine *trace__host(const struct trace *trace)
    {
    return trace.host;
    }
#[no_mangle]
unsafe extern "C" fn trace__load_vmlinux_btf(__maybe_unused: *mut *mut trace trace) {
    static void trace__load_vmlinux_btf(struct trace *trace __maybe_unused)
    {

    if (trace.btf != core::ptr::null_mut())
    return;
    trace.btf = btf__load_vmlinux_btf();
    if (verbose > 0) {
    fprintf(trace.output, trace.btf ? "vmlinux BTF loaded\n" :
    "Failed to load vmlinux BTF\n");
    }

    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_field {
    pub offset: c_int,
    union {
    pub sample): *mut *mut *mut u64 (integer)(struct tp_field field, struct perf_sample,
    pub sample): *mut *mut *mut *mut void (pointer)(struct tp_field field, struct perf_sample,
}

    };

    static u64 tp_field__u##bits(struct tp_field *field, struct perf_sample *sample) \
    { \
    u##bits value; \
    memcpy(&value, sample.raw_data + field.offset, sizeof(value)); \
    return value;  \
    }
    TP_UINT_FIELD(8);
    TP_UINT_FIELD(16);
    TP_UINT_FIELD(32);
    TP_UINT_FIELD(64);

    static u64 tp_field__swapped_u##bits(struct tp_field *field, struct perf_sample *sample) \
    { \
    u##bits value; \
    memcpy(&value, sample.raw_data + field.offset, sizeof(value)); \
    return bswap_##bits(value);\
    }
    TP_UINT_FIELD__SWAPPED(16);
    TP_UINT_FIELD__SWAPPED(32);
    TP_UINT_FIELD__SWAPPED(64);
#[no_mangle]
unsafe extern "C" fn __tp_field__init_uint(field: *mut tp_field, size: c_int, offset: c_int, needs_swap: bool) -> c_int {
    static int __tp_field__init_uint(struct tp_field *field, int size, int offset, bool needs_swap)
    {
    field.offset = offset;
    switch (size) {
    case 1:
    field.integer = tp_field__u8;
    break;
    case 2:
    field.integer = needs_swap ? tp_field__swapped_u16 : tp_field__u16;
    break;
    case 4:
    field.integer = needs_swap ? tp_field__swapped_u32 : tp_field__u32;
    break;
    case 8:
    field.integer = needs_swap ? tp_field__swapped_u64 : tp_field__u64;
    break;
    default:
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tp_field__init_uint(field: *mut tp_field, format_field: *mut tep_format_field, needs_swap: bool) -> c_int {
    static int tp_field__init_uint(struct tp_field *field, struct tep_format_field *format_field, bool needs_swap)
    {
    return __tp_field__init_uint(field, format_field.size, format_field.offset, needs_swap);
    }
    static void *tp_field__ptr(struct tp_field *field, struct perf_sample *sample)
    {
    return sample.raw_data + field.offset;
    }
#[no_mangle]
unsafe extern "C" fn __tp_field__init_ptr(field: *mut tp_field, offset: c_int) -> c_int {
    static int __tp_field__init_ptr(struct tp_field *field, int offset)
    {
    field.offset = offset;
    field.pointer = tp_field__ptr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tp_field__init_ptr(field: *mut tp_field, format_field: *mut tep_format_field) -> c_int {
    static int tp_field__init_ptr(struct tp_field *field, struct tep_format_field *format_field)
    {
    return __tp_field__init_ptr(field, format_field.offset);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_tp {
    pub id: tp_field,
    union {
    pub ret: tp_field args,,
}

    };
//
// The evsel->priv as used by 'perf trace'
// sc:	for raw_syscalls:sys_{enter,exit} and syscalls:sys_{enter,exit}_SYSCALLNAME
// fmt: for all the other tracepoints
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evsel_trace {
    pub sc: syscall_tp,
    pub fmt: *mut syscall_arg_fmt,
}

    static struct evsel_trace *evsel_trace__new(void)
    {
    return zalloc(sizeof(struct evsel_trace));
    }
#[no_mangle]
unsafe extern "C" fn evsel_trace__delete(et: *mut evsel_trace) {
    static void evsel_trace__delete(struct evsel_trace *et)
    {
    if (et == core::ptr::null_mut())
    return;
    zfree(&et.fmt);
    free(et);
    }
//
// Used with raw_syscalls:sys_{enter,exit} and with the
// syscalls:sys_{enter,exit}_SYSCALL tracepoints
//
    static inline struct syscall_tp *__evsel__syscall_tp(struct evsel *evsel)
    {
    struct evsel_trace *et = evsel.priv;
    return &et.sc;
    }
    static struct syscall_tp *evsel__syscall_tp(struct evsel *evsel)
    {
    if (evsel.priv == core::ptr::null_mut()) {
    evsel.priv = evsel_trace__new();
    if (evsel.priv == core::ptr::null_mut())
    return core::ptr::null_mut();
    }
    return __evsel__syscall_tp(evsel);
    }
//
// Used with all the other tracepoints.
//
    static inline struct syscall_arg_fmt *__evsel__syscall_arg_fmt(struct evsel *evsel)
    {
    struct evsel_trace *et = evsel.priv;
    return et.fmt;
    }
    static struct syscall_arg_fmt *evsel__syscall_arg_fmt(struct evsel *evsel)
    {
    struct evsel_trace *et = evsel.priv;
    if (evsel.priv == core::ptr::null_mut()) {
    et = evsel.priv = evsel_trace__new();
    if (et == core::ptr::null_mut())
    return core::ptr::null_mut();
    }
    if (et.fmt == core::ptr::null_mut()) {
    const struct tep_event *tp_format = evsel__tp_format(evsel);
    if (tp_format == core::ptr::null_mut())
    goto out_delete;
    et.fmt = calloc(tp_format.format.nr_fields, sizeof(struct syscall_arg_fmt));
    if (et.fmt == core::ptr::null_mut())
    goto out_delete;
    }
    return __evsel__syscall_arg_fmt(evsel);
    out_delete:
    evsel_trace__delete(evsel.priv);
    evsel.priv = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_tp_uint_field(evsel: *mut evsel, field: *mut tp_field, name: *const c_char) -> c_int {
    static int evsel__init_tp_uint_field(struct evsel *evsel, struct tp_field *field, const char *name)
    {
    struct tep_format_field *format_field = evsel__field(evsel, name);
    if (format_field == core::ptr::null_mut())
    return -1;
    return tp_field__init_uint(field, format_field, evsel.needs_swap);
    }

    ({ struct syscall_tp *sc = __evsel__syscall_tp(evsel);\
    evsel__init_tp_uint_field(evsel, &sc.name, #name); })
#[no_mangle]
unsafe extern "C" fn evsel__init_tp_ptr_field(evsel: *mut evsel, field: *mut tp_field, name: *const c_char) -> c_int {
    static int evsel__init_tp_ptr_field(struct evsel *evsel, struct tp_field *field, const char *name)
    {
    struct tep_format_field *format_field = evsel__field(evsel, name);
    if (format_field == core::ptr::null_mut())
    return -1;
    return tp_field__init_ptr(field, format_field);
    }

    ({ struct syscall_tp *sc = __evsel__syscall_tp(evsel);\
    evsel__init_tp_ptr_field(evsel, &sc.name, #name); })
#[no_mangle]
unsafe extern "C" fn evsel__put_and_free_priv(evsel: *mut evsel) {
    static void evsel__put_and_free_priv(struct evsel *evsel)
    {
    zfree(&evsel.priv);
    evsel__put(evsel);
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_syscall_tp(evsel: *mut evsel) -> c_int {
    static int evsel__init_syscall_tp(struct evsel *evsel)
    {
    struct syscall_tp *sc = evsel__syscall_tp(evsel);
    if (sc != core::ptr::null_mut()) {
    if (evsel__init_tp_uint_field(evsel, &sc.id, "__syscall_nr") &&
    evsel__init_tp_uint_field(evsel, &sc.id, "nr"))
    return -ENOENT;
    return 0;
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_augmented_syscall_tp(evsel: *mut evsel, tp: *mut evsel) -> c_int {
    static int evsel__init_augmented_syscall_tp(struct evsel *evsel, struct evsel *tp)
    {
    struct syscall_tp *sc = evsel__syscall_tp(evsel);
    if (sc != core::ptr::null_mut()) {
    struct tep_format_field *syscall_id = evsel__field(tp, "id");
    if (syscall_id == core::ptr::null_mut())
    syscall_id = evsel__field(tp, "__syscall_nr");
    if (syscall_id == core::ptr::null_mut() ||
    __tp_field__init_uint(&sc.id, syscall_id.size, syscall_id.offset, evsel.needs_swap))
    return -EINVAL;
    return 0;
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_augmented_syscall_tp_args(evsel: *mut evsel) -> c_int {
    static int evsel__init_augmented_syscall_tp_args(struct evsel *evsel)
    {
    struct syscall_tp *sc = __evsel__syscall_tp(evsel);
    return __tp_field__init_ptr(&sc.args, sc.id.offset + sizeof(u64));
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_augmented_syscall_tp_ret(evsel: *mut evsel) -> c_int {
    static int evsel__init_augmented_syscall_tp_ret(struct evsel *evsel)
    {
    struct syscall_tp *sc = __evsel__syscall_tp(evsel);
    return __tp_field__init_uint(&sc.ret, sizeof(u64), sc.id.offset + sizeof(u64), evsel.needs_swap);
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_raw_syscall_tp(evsel: *mut evsel, handler: *mut c_void) -> c_int {
    static int evsel__init_raw_syscall_tp(struct evsel *evsel, void *handler)
    {
    if (evsel__syscall_tp(evsel) != core::ptr::null_mut()) {
    if (perf_evsel__init_sc_tp_uint_field(evsel, id))
    return -ENOENT;
    evsel.handler = handler;
    return 0;
    }
    return -ENOMEM;
    }
    static struct evsel *perf_evsel__raw_syscall_newtp(const char *direction, void *handler)
    {
    struct evsel *evsel = evsel__newtp("raw_syscalls", direction);
// older kernel (e.g., RHEL6) use syscalls:{enter,exit}
    if (IS_ERR(evsel))
    evsel = evsel__newtp("syscalls", direction);
    if (IS_ERR(evsel))
    return core::ptr::null_mut();
    if (evsel__init_raw_syscall_tp(evsel, handler))
    goto out_delete;
    return evsel;
    out_delete:
    evsel__put_and_free_priv(evsel);
    return core::ptr::null_mut();
    }

    ({ struct syscall_tp *fields = __evsel__syscall_tp(sample.evsel); \
    fields.name.integer(&fields.name, sample); })

    ({ struct syscall_tp *fields = __evsel__syscall_tp(sample.evsel); \
    fields.name.pointer(&fields.name, sample); })
#[no_mangle]
pub unsafe extern "C" fn strarray__scnprintf_suffix(sa: *mut strarray, bf: *mut c_char, size: usize, intfmt: *const c_char, show_suffix: bool, val: c_int) -> usize {
    size_t strarray__scnprintf_suffix(struct strarray *sa, char *bf, size_t size, const char *intfmt, bool show_suffix, int val)
    {
    let mut idx: c_int = val - sa.offset;
    if (idx < 0 || idx >= sa.nr_entries || sa.entries[idx] == core::ptr::null_mut()) {
    let mut printed: usize = scnprintf(bf, size, intfmt, val);
    if (show_suffix)
    printed += scnprintf(bf + printed, size - printed, " /* %s??? */", sa.prefix);
    return printed;
    }
    return scnprintf(bf, size, "%s%s", sa.entries[idx], show_suffix ? sa.prefix : "");
    }
#[no_mangle]
pub unsafe extern "C" fn strarray__scnprintf(sa: *mut strarray, bf: *mut c_char, size: usize, intfmt: *const c_char, show_prefix: bool, val: c_int) -> usize {
    size_t strarray__scnprintf(struct strarray *sa, char *bf, size_t size, const char *intfmt, bool show_prefix, int val)
    {
    let mut idx: c_int = val - sa.offset;
    if (idx < 0 || idx >= sa.nr_entries || sa.entries[idx] == core::ptr::null_mut()) {
    let mut printed: usize = scnprintf(bf, size, intfmt, val);
    if (show_prefix)
    printed += scnprintf(bf + printed, size - printed, " /* %s??? */", sa.prefix);
    return printed;
    }
    return scnprintf(bf, size, "%s%s", show_prefix ? sa.prefix : "", sa.entries[idx]);
    }
    static size_t __syscall_arg__scnprintf_strarray(char *bf, size_t size,
    const char *intfmt,
    struct syscall_arg *arg)
    {
    return strarray__scnprintf(arg.parm, bf, size, intfmt, arg.show_string_prefix, arg.val);
    }
    static size_t syscall_arg__scnprintf_strarray(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    return __syscall_arg__scnprintf_strarray(bf, size, "%d", arg);
    }

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__strtoul_strarray(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool {
    bool syscall_arg__strtoul_strarray(char *bf, size_t size, struct syscall_arg *arg, u64 *ret)
    {
    return strarray__strtoul(arg.parm, bf, size, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__strtoul_strarray_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool {
    bool syscall_arg__strtoul_strarray_flags(char *bf, size_t size, struct syscall_arg *arg, u64 *ret)
    {
    return strarray__strtoul_flags(arg.parm, bf, size, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__strtoul_strarrays(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool {
    bool syscall_arg__strtoul_strarrays(char *bf, size_t size, struct syscall_arg *arg, u64 *ret)
    {
    return strarrays__strtoul(arg.parm, bf, size, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_strarray_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_strarray_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    return strarray__scnprintf_flags(arg.parm, bf, size, arg.show_string_prefix, arg.val);
    }
#[no_mangle]
pub unsafe extern "C" fn strarrays__scnprintf(sas: *mut strarrays, bf: *mut c_char, size: usize, intfmt: *const c_char, show_prefix: bool, val: c_int) -> usize {
    size_t strarrays__scnprintf(struct strarrays *sas, char *bf, size_t size, const char *intfmt, bool show_prefix, int val)
    {
    size_t printed;
    int i;
    for (i = 0; i < sas.nr_entries; ++i) {
    struct strarray *sa = sas.entries[i];
    let mut idx: c_int = val - sa.offset;
    if (idx >= 0 && idx < sa.nr_entries) {
    if (sa.entries[idx] == core::ptr::null_mut())
    break;
    return scnprintf(bf, size, "%s%s", show_prefix ? sa.prefix : "", sa.entries[idx]);
    }
    }
    printed = scnprintf(bf, size, intfmt, val);
    if (show_prefix)
    printed += scnprintf(bf + printed, size - printed, " /* %s??? */", sas.entries[0].prefix);
    return printed;
    }
#[no_mangle]
pub unsafe extern "C" fn strarray__strtoul(sa: *mut strarray, bf: *mut c_char, size: usize, ret: *mut u64) -> bool {
    bool strarray__strtoul(struct strarray *sa, char *bf, size_t size, u64 *ret)
    {
    int i;
    for (i = 0; i < sa.nr_entries; ++i) {
    if (sa.entries[i] && strncmp(sa.entries[i], bf, size) == 0 && sa.entries[i][size] == '\0') {
// ret = sa->offset + i;
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn strarray__strtoul_flags(sa: *mut strarray, bf: *mut c_char, size: usize, ret: *mut u64) -> bool {
    bool strarray__strtoul_flags(struct strarray *sa, char *bf, size_t size, u64 *ret)
    {
    let mut val: u64 = 0;
    char *tok = bf, *sep, *end;
// ret = 0;
    while (size != 0) {
    let mut toklen: c_int = size;
    sep = memchr(tok, '|', size);
    if (sep != core::ptr::null_mut()) {
    size -= sep - tok + 1;
    end = sep - 1;
    while (end > tok && isspace(*end))
    --end;
    toklen = end - tok + 1;
    }
    while (isspace(*tok))
    ++tok;
    if (isalpha(*tok) || *tok == '_') {
    if (!strarray__strtoul(sa, tok, toklen, &val))
    return false;
    } else
    val = strtoul(tok, core::ptr::null_mut(), 0);
// ret |= (1 << (val - 1));
    if (sep == core::ptr::null_mut())
    break;
    tok = sep + 1;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn strarrays__strtoul(sas: *mut strarrays, bf: *mut c_char, size: usize, ret: *mut u64) -> bool {
    bool strarrays__strtoul(struct strarrays *sas, char *bf, size_t size, u64 *ret)
    {
    int i;
    for (i = 0; i < sas.nr_entries; ++i) {
    struct strarray *sa = sas.entries[i];
    if (strarray__strtoul(sa, bf, size, ret))
    return true;
    }
    return false;
    }
    size_t syscall_arg__scnprintf_strarrays(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    return strarrays__scnprintf(arg.parm, bf, size, "%d", arg.show_string_prefix, arg.val);
    }

    static size_t syscall_arg__scnprintf_fd_at(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut fd: c_int = arg.val;
    const char *prefix = "AT_FD";
    if (fd == AT_FDCWD)
    return scnprintf(bf, size, "%s%s", arg.show_string_prefix ? prefix : "", "CWD");
    return syscall_arg__scnprintf_fd(bf, size, arg);
    }

    static size_t syscall_arg__scnprintf_close_fd(char *bf, size_t size,
    struct syscall_arg *arg);

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_hex(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_hex(char *bf, size_t size, struct syscall_arg *arg)
    {
    return scnprintf(bf, size, "%#lx", arg.val);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_ptr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_ptr(char *bf, size_t size, struct syscall_arg *arg)
    {
    if (arg.val == 0)
    return scnprintf(bf, size, "core::ptr::null_mut()");
    return syscall_arg__scnprintf_hex(bf, size, arg);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_int(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_int(char *bf, size_t size, struct syscall_arg *arg)
    {
    return scnprintf(bf, size, "%d", arg.val);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_long(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_long(char *bf, size_t size, struct syscall_arg *arg)
    {
    return scnprintf(bf, size, "%ld", arg.val);
    }
#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_char_array(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    static size_t syscall_arg__scnprintf_char_array(char *bf, size_t size, struct syscall_arg *arg)
    {
// XXX Hey, maybe for sched:sched_switch prev/next comm fields we can
// fill missing comms using thread__set_comm()...
// here or in a special syscall_arg__scnprintf_pid_sched_tp...
    return scnprintf(bf, size, "\"%-.*s\"", arg.fmt.nr_entries ?: arg.len, arg.val);
    }

    static const char *bpf_cmd[] = {
    "MAP_CREATE", "MAP_LOOKUP_ELEM", "MAP_UPDATE_ELEM", "MAP_DELETE_ELEM",
    "MAP_GET_NEXT_KEY", "PROG_LOAD", "OBJ_PIN", "OBJ_GET", "PROG_ATTACH",
    "PROG_DETACH", "PROG_TEST_RUN", "PROG_GET_NEXT_ID", "MAP_GET_NEXT_ID",
    "PROG_GET_FD_BY_ID", "MAP_GET_FD_BY_ID", "OBJ_GET_INFO_BY_FD",
    "PROG_QUERY", "RAW_TRACEPOINT_OPEN", "BTF_LOAD", "BTF_GET_FD_BY_ID",
    "TASK_FD_QUERY", "MAP_LOOKUP_AND_DELETE_ELEM", "MAP_FREEZE",
    "BTF_GET_NEXT_ID", "MAP_LOOKUP_BATCH", "MAP_LOOKUP_AND_DELETE_BATCH",
    "MAP_UPDATE_BATCH", "MAP_DELETE_BATCH", "LINK_CREATE", "LINK_UPDATE",
    "LINK_GET_FD_BY_ID", "LINK_GET_NEXT_ID", "ENABLE_STATS", "ITER_CREATE",
    "LINK_DETACH", "PROG_BIND_MAP",
    };
    static DEFINE_STRARRAY(bpf_cmd, "BPF_");
    static const char *epoll_ctl_ops[] = { "ADD", "DEL", "MOD", };
    static DEFINE_STRARRAY_OFFSET(epoll_ctl_ops, "EPOLL_CTL_", 1);
    static const char *itimers[] = { "REAL", "VIRTUAL", "PROF", };
    static DEFINE_STRARRAY(itimers, "ITIMER_");
    static const char *keyctl_options[] = {
    "GET_KEYRING_ID", "JOIN_SESSION_KEYRING", "UPDATE", "REVOKE", "CHOWN",
    "SETPERM", "DESCRIBE", "CLEAR", "LINK", "UNLINK", "SEARCH", "READ",
    "INSTANTIATE", "NEGATE", "SET_REQKEY_KEYRING", "SET_TIMEOUT",
    "ASSUME_AUTHORITY", "GET_SECURITY", "SESSION_TO_PARENT", "REJECT",
    "INSTANTIATE_IOV", "INVALIDATE", "GET_PERSISTENT",
    };
    static DEFINE_STRARRAY(keyctl_options, "KEYCTL_");
    static const char *whences[] = { "SET", "CUR", "END",

    "DATA",

    "HOLE",

    };
    static DEFINE_STRARRAY(whences, "SEEK_");
    static const char *fcntl_cmds[] = {
    "DUPFD", "GETFD", "SETFD", "GETFL", "SETFL", "GETLK", "SETLK",
    "SETLKW", "SETOWN", "GETOWN", "SETSIG", "GETSIG", "GETLK64",
    "SETLK64", "SETLKW64", "SETOWN_EX", "GETOWN_EX",
    "GETOWNER_UIDS",
    };
    static DEFINE_STRARRAY(fcntl_cmds, "F_");
    static const char *fcntl_linux_specific_cmds[] = {
    "SETLEASE", "GETLEASE", "NOTIFY", "DUPFD_QUERY", [5] = "CANCELLK", "DUPFD_CLOEXEC",
    "SETPIPE_SZ", "GETPIPE_SZ", "ADD_SEALS", "GET_SEALS",
    "GET_RW_HINT", "SET_RW_HINT", "GET_FILE_RW_HINT", "SET_FILE_RW_HINT",
    };
    static DEFINE_STRARRAY_OFFSET(fcntl_linux_specific_cmds, "F_", F_LINUX_SPECIFIC_BASE);
    static struct strarray *fcntl_cmds_arrays[] = {
    &strarray__fcntl_cmds,
    &strarray__fcntl_linux_specific_cmds,
    };
    static DEFINE_STRARRAYS(fcntl_cmds_arrays);
    static const char *rlimit_resources[] = {
    "CPU", "FSIZE", "DATA", "STACK", "CORE", "RSS", "NPROC", "NOFILE",
    "MEMLOCK", "AS", "LOCKS", "SIGPENDING", "MSGQUEUE", "NICE", "RTPRIO",
    "RTTIME",
    };
    static DEFINE_STRARRAY(rlimit_resources, "RLIMIT_");
    static const char *sighow[] = { "BLOCK", "UNBLOCK", "SETMASK", };
    static DEFINE_STRARRAY(sighow, "SIG_");
    static const char *clockid[] = {
    "REALTIME", "MONOTONIC", "PROCESS_CPUTIME_ID", "THREAD_CPUTIME_ID",
    "MONOTONIC_RAW", "REALTIME_COARSE", "MONOTONIC_COARSE", "BOOTTIME",
    "REALTIME_ALARM", "BOOTTIME_ALARM", "SGI_CYCLE", "TAI"
    };
    static DEFINE_STRARRAY(clockid, "CLOCK_");
    static size_t syscall_arg__scnprintf_access_mode(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *suffix = "_OK";
    let mut printed: usize = 0;
    let mut mode: c_int = arg.val;
    if (mode == F_OK) /* 0 */
    return scnprintf(bf, size, "F%s", show_prefix ? suffix : "");

    if (mode & n##_OK) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s", #n, show_prefix ? suffix : ""); \
    mode &= ~n##_OK; \
    }
    P_MODE(R);
    P_MODE(W);
    P_MODE(X);

    if (mode)
    printed += scnprintf(bf + printed, size - printed, "|%#x", mode);
    return printed;
    }

    static size_t syscall_arg__scnprintf_filename(char *bf, size_t size,
    struct syscall_arg *arg);

// 'argname' is just documentational at this point, to remove the previous comment with that info

    { .scnprintf	= SCA_FILENAME, \
    .from_user	= true, }
    static size_t syscall_arg__scnprintf_buf(char *bf, size_t size, struct syscall_arg *arg);

    static size_t syscall_arg__scnprintf_pipe_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "O_";
    let mut printed: c_int = 0, flags = arg.val;

    if (flags & O_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~O_##n; \
    }
    P_FLAG(CLOEXEC);
    P_FLAG(NONBLOCK);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }

pub const GRND_NONBLOCK: c_uint = 0x0001;

pub const GRND_RANDOM: c_uint = 0x0002;

    static size_t syscall_arg__scnprintf_getrandom_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "GRND_";
    let mut printed: c_int = 0, flags = arg.val;

    if (flags & GRND_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~GRND_##n; \
    }
    P_FLAG(RANDOM);
    P_FLAG(NONBLOCK);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }

#[no_mangle]
unsafe extern "C" fn syscall_arg_fmt__cache_btf_enum(arg_fmt: *mut syscall_arg_fmt, btf: *mut btf, type: *mut c_char) {
    static void syscall_arg_fmt__cache_btf_enum(struct syscall_arg_fmt *arg_fmt, struct btf *btf, char *type)
    {
    int id;
    type = strstr(type, "enum ");
    if (type == core::ptr::null_mut())
    return;
    type += 5; // skip "enum " to get the enumeration name
    id = btf__find_by_name(btf, type);
    if (id < 0)
    return;
    arg_fmt.type = btf__type_by_id(btf, id);
    }
#[no_mangle]
unsafe extern "C" fn syscall_arg__strtoul_btf_enum(bf: *mut c_char, size: usize, arg: *mut syscall_arg, val: *mut u64) -> bool {
    static bool syscall_arg__strtoul_btf_enum(char *bf, size_t size, struct syscall_arg *arg, u64 *val)
    {
    const struct btf_type *bt = arg.fmt.type;
    struct btf *btf = arg.trace.btf;
    struct btf_enum *be = btf_enum(bt);
    for (u32 i = 0; i < btf_vlen(bt); ++i, ++be) {
    const char *name = btf__name_by_offset(btf, be.name_off);
    let mut max_len: c_int = max(size, strlen(name));
    if (strncmp(name, bf, max_len) == 0) {
// val = be->val;
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn syscall_arg__strtoul_btf_type(bf: *mut c_char, size: usize, arg: *mut syscall_arg, val: *mut u64) -> bool {
    static bool syscall_arg__strtoul_btf_type(char *bf, size_t size, struct syscall_arg *arg, u64 *val)
    {
    const struct btf_type *bt;
    char *type = arg.type_name;
    struct btf *btf;
    trace__load_vmlinux_btf(arg.trace);
    btf = arg.trace.btf;
    if (btf == core::ptr::null_mut())
    return false;
    if (arg.fmt.type == core::ptr::null_mut()) {
// See if this is an enum
    syscall_arg_fmt__cache_btf_enum(arg.fmt, btf, type);
    }
// Now let's see if we have a BTF type resolved
    bt = arg.fmt.type;
    if (bt == core::ptr::null_mut())
    return false;
// If it is an enum:
    if (btf_is_enum(arg.fmt.type))
    return syscall_arg__strtoul_btf_enum(bf, size, arg, val);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn btf_enum_scnprintf(type: *const btf_type, btf: *mut btf, bf: *mut c_char, size: usize, val: c_int) -> usize {
    static size_t btf_enum_scnprintf(const struct btf_type *type, struct btf *btf, char *bf, size_t size, int val)
    {
    struct btf_enum *be = btf_enum(type);
    let mut nr_entries: c_uint = btf_vlen(type);
    for (unsigned int i = 0; i < nr_entries; ++i, ++be) {
    if (be.val == val) {
    return scnprintf(bf, size, "%s",
    btf__name_by_offset(btf, be.name_off));
    }
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_btf_dump_snprintf_ctx {
    pub bf: *mut c_char,
    pub size: size_t printed,,
}

#[no_mangle]
unsafe extern "C" fn trace__btf_dump_snprintf(vctx: *mut c_void, fmt: *const c_char, args: va_list) {
    static void trace__btf_dump_snprintf(void *vctx, const char *fmt, va_list args)
    {
    struct trace_btf_dump_snprintf_ctx *ctx = vctx;
    ctx.printed += vscnprintf(ctx.bf + ctx.printed, ctx.size - ctx.printed, fmt, args);
    }
#[no_mangle]
unsafe extern "C" fn btf_struct_scnprintf(type: *const btf_type, btf: *mut btf, bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    static size_t btf_struct_scnprintf(const struct btf_type *type, struct btf *btf, char *bf, size_t size, struct syscall_arg *arg)
    {
    struct trace_btf_dump_snprintf_ctx ctx = {
    .bf   = bf,
    .size = size,
    };
    struct augmented_arg *augmented_arg = arg.augmented.args;
    let mut type_id: c_int = arg.fmt.type_id, consumed;
    struct btf_dump *btf_dump;
    LIBBPF_OPTS(btf_dump_opts, dump_opts);
    LIBBPF_OPTS(btf_dump_type_data_opts, dump_data_opts);
    if (arg == core::ptr::null_mut() || arg.augmented.args == core::ptr::null_mut())
    return 0;
    dump_data_opts.compact	  = true;
    dump_data_opts.skip_names = !arg.trace.show_arg_names;
    btf_dump = btf_dump__new(btf, trace__btf_dump_snprintf, &ctx, &dump_opts);
    if (btf_dump == core::ptr::null_mut())
    return 0;
// pretty print the struct data here
    if (btf_dump__dump_type_data(btf_dump, type_id, arg.augmented.args.value, type.size, &dump_data_opts) == 0)
    return 0;
    consumed = sizeof(*augmented_arg) + augmented_arg.size;
    arg.augmented.args = ((void *)arg.augmented.args) + consumed;
    arg.augmented.size -= consumed;
    btf_dump__free(btf_dump);
    return ctx.printed;
    }
    static size_t trace__btf_scnprintf(struct trace *trace, struct syscall_arg *arg, char *bf,
    size_t size, int val, char *type)
    {
    struct syscall_arg_fmt *arg_fmt = arg.fmt;
    if (trace.btf == core::ptr::null_mut())
    return 0;
    if (arg_fmt.type == core::ptr::null_mut()) {
// Check if this is an enum and if we have the BTF type for it.
    syscall_arg_fmt__cache_btf_enum(arg_fmt, trace.btf, type);
    }
// Did we manage to find a BTF type for the syscall/tracepoint argument?
    if (arg_fmt.type == core::ptr::null_mut())
    return 0;
    if (btf_is_enum(arg_fmt.type))
    return btf_enum_scnprintf(arg_fmt.type, trace.btf, bf, size, val);
#[no_mangle]
pub unsafe extern "C" fn if(btf_is_union(arg_fmt->type): btf_is_struct(arg_fmt->type) ||) -> else {
    else if (btf_is_struct(arg_fmt.type) || btf_is_union(arg_fmt.type))
    return btf_struct_scnprintf(arg_fmt.type, trace.btf, bf, size, arg);
    return 0;
    }

    static size_t trace__btf_scnprintf(struct trace *trace __maybe_unused, struct syscall_arg *arg __maybe_unused,
    char *bf __maybe_unused, size_t size __maybe_unused, int val __maybe_unused,
    char *type __maybe_unused)
    {
    return 0;
    }
    static bool syscall_arg__strtoul_btf_type(char *bf __maybe_unused, size_t size __maybe_unused,
    struct syscall_arg *arg __maybe_unused, u64 *val __maybe_unused)
    {
    return false;
    }

    { .scnprintf	= SCA_STRARRAY, \
    .strtoul	= STUL_STRARRAY, \
    .parm	= &strarray__##array, \
    .show_zero	= true, }

    { .scnprintf	= SCA_STRARRAY_FLAGS, \
    .strtoul	= STUL_STRARRAY_FLAGS, \
    .parm	= &strarray__##array, \
    .show_zero	= true, }
    static const struct syscall_fmt syscall_fmts[] = {
    { .name	    = "access",
    .arg = { [1] = { .scnprintf = SCA_ACCMODE,  /* mode */ }, }, },
    { .name	    = "arch_prctl",
    .arg = { [0] = { .scnprintf = SCA_X86_ARCH_PRCTL_CODE, /* code */ },
    [1] = { .scnprintf = SCA_PTR, /* arg2 */ }, }, },
    { .name	    = "bind",
    .arg = { [0] = { .scnprintf = SCA_INT, /* fd */ },
    [1] = SCA_SOCKADDR_FROM_USER(umyaddr),
    [2] = { .scnprintf = SCA_INT, /* addrlen */ }, }, },
    { .name	    = "bpf",
    .arg = { [0] = STRARRAY(cmd, bpf_cmd),
    [1] = { .from_user = true /* attr */, }, } },
    { .name	    = "brk",	    .hexret = true,
    .arg = { [0] = { .scnprintf = SCA_PTR, /* brk */ }, }, },
    { .name     = "clock_gettime",
    .arg = { [0] = STRARRAY(clk_id, clockid), }, },
    { .name	    = "clock_nanosleep",
    .arg = { [2] = SCA_TIMESPEC_FROM_USER(req), }, },
    { .name	    = "clone",	    .errpid = true, .nr_args = 5,
    .arg = { [0] = { .name = "flags",	    .scnprintf = SCA_CLONE_FLAGS, },
    [1] = { .name = "child_stack",   .scnprintf = SCA_HEX, },
    [2] = { .name = "parent_tidptr", .scnprintf = SCA_HEX, },
    [3] = { .name = "child_tidptr",  .scnprintf = SCA_HEX, },
    [4] = { .name = "tls",	    .scnprintf = SCA_HEX, }, }, },
    { .name	    = "close",
    .arg = { [0] = { .scnprintf = SCA_CLOSE_FD, /* fd */ }, }, },
    { .name	    = "connect",
    .arg = { [0] = { .scnprintf = SCA_INT, /* fd */ },
    [1] = SCA_SOCKADDR_FROM_USER(servaddr),
    [2] = { .scnprintf = SCA_INT, /* addrlen */ }, }, },
    { .name	    = "epoll_ctl",
    .arg = { [1] = STRARRAY(op, epoll_ctl_ops), }, },
    { .name	    = "eventfd2",
    .arg = { [1] = { .scnprintf = SCA_EFD_FLAGS, /* flags */ }, }, },
    { .name     = "faccessat",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	  /* dirfd */ },
    [1] = SCA_FILENAME_FROM_USER(pathname),
    [2] = { .scnprintf = SCA_ACCMODE,	  /* mode */ }, }, },
    { .name     = "faccessat2",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	  /* dirfd */ },
    [1] = SCA_FILENAME_FROM_USER(pathname),
    [2] = { .scnprintf = SCA_ACCMODE,	  /* mode */ },
    [3] = { .scnprintf = SCA_FACCESSAT2_FLAGS, /* flags */ }, }, },
    { .name	    = "fchmodat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "fchownat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "fcntl",
    .arg = { [1] = { .scnprintf = SCA_FCNTL_CMD,  /* cmd */
    .strtoul   = STUL_STRARRAYS,
    .parm      = &strarrays__fcntl_cmds_arrays,
    .show_zero = true, },
    [2] = { .scnprintf =  SCA_FCNTL_ARG, /* arg */ }, }, },
    { .name	    = "flock",
    .arg = { [1] = { .scnprintf = SCA_FLOCK, /* cmd */ }, }, },
    { .name     = "fsconfig",
    .arg = { [1] = STRARRAY(cmd, fsconfig_cmds), }, },
    { .name     = "fsmount",
    .arg = { [1] = { .scnprintf = SCA_FSMOUNT_FLAGS, /* fsmount_flags */
    .strtoul   = STUL_STRARRAYS,
    .show_zero = true, },
    [2] = { .scnprintf = SCA_FSMOUNT_ATTR_FLAGS, /* attr_flags */ }, }, },
    { .name     = "fspick",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	  /* dfd */ },
    [1] = SCA_FILENAME_FROM_USER(path),
    [2] = { .scnprintf = SCA_FSPICK_FLAGS, /* flags */ }, }, },
    { .name	    = "fstat", .alias = "newfstat", },
    { .name	    = "futex",
    .arg = { [1] = { .scnprintf = SCA_FUTEX_OP, /* op */ },
    [5] = { .scnprintf = SCA_FUTEX_VAL3, /* val3 */ }, }, },
    { .name	    = "futimesat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "getitimer",
    .arg = { [0] = STRARRAY(which, itimers), }, },
    { .name	    = "getpid",	    .errpid = true, },
    { .name	    = "getpgid",    .errpid = true, },
    { .name	    = "getppid",    .errpid = true, },
    { .name	    = "getrandom",
    .arg = { [2] = { .scnprintf = SCA_GETRANDOM_FLAGS, /* flags */ }, }, },
    { .name	    = "getrlimit",
    .arg = { [0] = STRARRAY(resource, rlimit_resources), }, },
    { .name	    = "getsockopt",
    .arg = { [1] = STRARRAY(level, socket_level), }, },
    { .name	    = "gettid",	    .errpid = true, },
    { .name	    = "ioctl",
    .arg = {

//
// FIXME: Make this available to all arches.
//
    [1] = { .scnprintf = SCA_IOCTL_CMD, /* cmd */ },
    [2] = { .scnprintf = SCA_HEX, /* arg */ }, }, },

    [2] = { .scnprintf = SCA_HEX, /* arg */ }, }, },

    { .name	    = "kcmp",	    .nr_args = 5,
    .arg = { [0] = { .name = "pid1",	.scnprintf = SCA_PID, },
    [1] = { .name = "pid2",	.scnprintf = SCA_PID, },
    [2] = { .name = "type",	.scnprintf = SCA_KCMP_TYPE, },
    [3] = { .name = "idx1",	.scnprintf = SCA_KCMP_IDX, },
    [4] = { .name = "idx2",	.scnprintf = SCA_KCMP_IDX, }, }, },
    { .name	    = "keyctl",
    .arg = { [0] = STRARRAY(option, keyctl_options), }, },
    { .name	    = "kill",
    .arg = { [1] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name	    = "linkat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "lseek",
    .arg = { [2] = STRARRAY(whence, whences), }, },
    { .name	    = "lstat", .alias = "newlstat", },
    { .name     = "madvise",
    .arg = { [0] = { .scnprintf = SCA_HEX,      /* start */ },
    [2] = { .scnprintf = SCA_MADV_BHV, /* behavior */ }, }, },
    { .name	    = "mkdirat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "mknodat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* fd */ }, }, },
    { .name	    = "mmap",	    .hexret = true,
// The standard mmap maps to old_mmap on s390x

    .alias = "old_mmap",

    .arg = { [2] = { .scnprintf = SCA_MMAP_PROT, .show_zero = true, /* prot */ },
    [3] = { .scnprintf = SCA_MMAP_FLAGS,	/* flags */
    .strtoul   = STUL_STRARRAY_FLAGS,
    .parm      = &strarray__mmap_flags, },
    [5] = { .scnprintf = SCA_HEX,	/* offset */ }, }, },
    { .name	    = "mount",
    .arg = { [0] = SCA_FILENAME_FROM_USER(devname),
    [3] = { .scnprintf = SCA_MOUNT_FLAGS, /* flags */
    .mask_val  = SCAMV_MOUNT_FLAGS, /* flags */ }, }, },
    { .name	    = "move_mount",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	/* from_dfd */ },
    [1] = SCA_FILENAME_FROM_USER(pathname),
    [2] = { .scnprintf = SCA_FDAT,	/* to_dfd */ },
    [3] = SCA_FILENAME_FROM_USER(pathname),
    [4] = { .scnprintf = SCA_MOVE_MOUNT_FLAGS, /* flags */ }, }, },
    { .name	    = "mprotect",
    .arg = { [0] = { .scnprintf = SCA_HEX,	/* start */ },
    [2] = { .scnprintf = SCA_MMAP_PROT, .show_zero = true, /* prot */ }, }, },
    { .name	    = "mq_unlink",
    .arg = { [0] = SCA_FILENAME_FROM_USER(u_name), }, },
    { .name	    = "mremap",	    .hexret = true,
    .arg = { [3] = { .scnprintf = SCA_MREMAP_FLAGS, /* flags */ }, }, },
    { .name	    = "name_to_handle_at",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* dfd */ }, }, },
    { .name	    = "nanosleep",
    .arg = { [0] = SCA_TIMESPEC_FROM_USER(req), }, },
    { .name	    = "newfstatat", .alias = "fstatat",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	  /* dirfd */ },
    [1] = SCA_FILENAME_FROM_USER(pathname),
    [3] = { .scnprintf = SCA_FS_AT_FLAGS, /* flags */ }, }, },
    { .name	    = "open",
    .arg = { [1] = { .scnprintf = SCA_OPEN_FLAGS, /* flags */ }, }, },
    { .name	    = "open_by_handle_at",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	/* dfd */ },
    [2] = { .scnprintf = SCA_OPEN_FLAGS, /* flags */ }, }, },
    { .name	    = "openat",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	/* dfd */ },
    [2] = { .scnprintf = SCA_OPEN_FLAGS, /* flags */ }, }, },
    { .name	    = "perf_event_open",
    .arg = { [0] = SCA_PERF_ATTR_FROM_USER(attr),
    [2] = { .scnprintf = SCA_INT,	/* cpu */ },
    [3] = { .scnprintf = SCA_FD,		/* group_fd */ },
    [4] = { .scnprintf = SCA_PERF_FLAGS, /* flags */ }, }, },
    { .name	    = "pipe2",
    .arg = { [1] = { .scnprintf = SCA_PIPE_FLAGS, /* flags */ }, }, },
    { .name	    = "pkey_alloc",
    .arg = { [1] = { .scnprintf = SCA_PKEY_ALLOC_ACCESS_RIGHTS,	/* access_rights */ }, }, },
    { .name	    = "pkey_free",
    .arg = { [0] = { .scnprintf = SCA_INT,	/* key */ }, }, },
    { .name	    = "pkey_mprotect",
    .arg = { [0] = { .scnprintf = SCA_HEX,	/* start */ },
    [2] = { .scnprintf = SCA_MMAP_PROT, .show_zero = true, /* prot */ },
    [3] = { .scnprintf = SCA_INT,	/* pkey */ }, }, },
    { .name	    = "poll", .timeout = true, },
    { .name	    = "ppoll", .timeout = true, },
    { .name	    = "prctl",
    .arg = { [0] = { .scnprintf = SCA_PRCTL_OPTION, /* option */
    .strtoul   = STUL_STRARRAY,
    .parm      = &strarray__prctl_options, },
    [1] = { .scnprintf = SCA_PRCTL_ARG2, /* arg2 */ },
    [2] = { .scnprintf = SCA_PRCTL_ARG3, /* arg3 */ }, }, },
    { .name	    = "pread", .alias = "pread64", },
    { .name	    = "preadv", .alias = "pread", },
    { .name	    = "prlimit64",
    .arg = { [1] = STRARRAY(resource, rlimit_resources),
    [2] = { .from_user = true /* new_rlim */, }, }, },
    { .name	    = "pwrite", .alias = "pwrite64", },
    { .name	    = "readlinkat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* dfd */ }, }, },
    { .name	    = "recvfrom",
    .arg = { [3] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ }, }, },
    { .name	    = "recvmmsg",
    .arg = { [3] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ }, }, },
    { .name	    = "recvmsg",
    .arg = { [2] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ }, }, },
    { .name	    = "renameat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* olddirfd */ },
    [2] = { .scnprintf = SCA_FDAT, /* newdirfd */ }, }, },
    { .name	    = "renameat2",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* olddirfd */ },
    [2] = { .scnprintf = SCA_FDAT, /* newdirfd */ },
    [4] = { .scnprintf = SCA_RENAMEAT2_FLAGS, /* flags */ }, }, },
    { .name	    = "rseq",
    .arg = { [0] = { .from_user = true /* rseq */, }, }, },
    { .name	    = "rt_sigaction",
    .arg = { [0] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name	    = "rt_sigprocmask",
    .arg = { [0] = STRARRAY(how, sighow), }, },
    { .name	    = "rt_sigqueueinfo",
    .arg = { [1] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name	    = "rt_tgsigqueueinfo",
    .arg = { [2] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name	    = "sched_setscheduler",
    .arg = { [1] = { .scnprintf = SCA_SCHED_POLICY, /* policy */ }, }, },
    { .name	    = "seccomp",
    .arg = { [0] = { .scnprintf = SCA_SECCOMP_OP,	   /* op */ },
    [1] = { .scnprintf = SCA_SECCOMP_FLAGS, /* flags */ }, }, },
    { .name	    = "select", .timeout = true, },
    { .name	    = "sendfile", .alias = "sendfile64", },
    { .name	    = "sendmmsg",
    .arg = { [3] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ }, }, },
    { .name	    = "sendmsg",
    .arg = { [2] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ }, }, },
    { .name	    = "sendto",
    .arg = { [3] = { .scnprintf = SCA_MSG_FLAGS, /* flags */ },
    [4] = SCA_SOCKADDR_FROM_USER(addr), }, },
    { .name	    = "set_robust_list",
    .arg = { [0] = { .from_user = true /* head */, }, }, },
    { .name	    = "set_tid_address", .errpid = true, },
    { .name	    = "setitimer",
    .arg = { [0] = STRARRAY(which, itimers), }, },
    { .name	    = "setrlimit",
    .arg = { [0] = STRARRAY(resource, rlimit_resources),
    [1] = { .from_user = true /* rlim */, }, }, },
    { .name	    = "setsockopt",
    .arg = { [1] = STRARRAY(level, socket_level), }, },
    { .name	    = "socket",
    .arg = { [0] = STRARRAY(family, socket_families),
    [1] = { .scnprintf = SCA_SK_TYPE, /* type */ },
    [2] = { .scnprintf = SCA_SK_PROTO, /* protocol */ }, }, },
    { .name	    = "socketpair",
    .arg = { [0] = STRARRAY(family, socket_families),
    [1] = { .scnprintf = SCA_SK_TYPE, /* type */ },
    [2] = { .scnprintf = SCA_SK_PROTO, /* protocol */ }, }, },
    { .name	    = "stat", .alias = "newstat", },
    { .name	    = "statx",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	 /* fdat */ },
    [2] = { .scnprintf = SCA_FS_AT_FLAGS, /* flags */ } ,
    [3] = { .scnprintf = SCA_STATX_MASK,	 /* mask */ }, }, },
    { .name	    = "swapoff",
    .arg = { [0] = SCA_FILENAME_FROM_USER(specialfile), }, },
    { .name	    = "swapon",
    .arg = { [0] = SCA_FILENAME_FROM_USER(specialfile), }, },
    { .name	    = "symlinkat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* dfd */ }, }, },
    { .name	    = "sync_file_range",
    .arg = { [3] = { .scnprintf = SCA_SYNC_FILE_RANGE_FLAGS, /* flags */ }, }, },
    { .name	    = "tgkill",
    .arg = { [2] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name	    = "tkill",
    .arg = { [1] = { .scnprintf = SCA_SIGNUM, /* sig */ }, }, },
    { .name     = "umount2", .alias = "umount",
    .arg = { [0] = SCA_FILENAME_FROM_USER(name), }, },
    { .name	    = "uname", .alias = "newuname", },
    { .name	    = "unlinkat",
    .arg = { [0] = { .scnprintf = SCA_FDAT,	  /* dfd */ },
    [1] = SCA_FILENAME_FROM_USER(pathname),
    [2] = { .scnprintf = SCA_FS_AT_FLAGS,  /* flags */ }, }, },
    { .name	    = "utimensat",
    .arg = { [0] = { .scnprintf = SCA_FDAT, /* dirfd */ }, }, },
    { .name	    = "wait4",	    .errpid = true,
    .arg = { [2] = { .scnprintf = SCA_WAITID_OPTIONS, /* options */ }, }, },
    { .name	    = "waitid",	    .errpid = true,
    .arg = { [3] = { .scnprintf = SCA_WAITID_OPTIONS, /* options */ }, }, },
    { .name	    = "write",
    .arg = { [1] = { .scnprintf = SCA_BUF /* buf */, .from_user = true, }, }, },
    };
#[no_mangle]
unsafe extern "C" fn syscall_fmt__cmp(name: *const c_void, fmtp: *const c_void) -> c_int {
    static int syscall_fmt__cmp(const void *name, const void *fmtp)
    {
    const struct syscall_fmt *fmt = fmtp;
    return strcmp(name, fmt.name);
    }
    static const struct syscall_fmt *__syscall_fmt__find(const struct syscall_fmt *fmts,
    const int nmemb,
    const char *name)
    {
    return bsearch(name, fmts, nmemb, sizeof(struct syscall_fmt), syscall_fmt__cmp);
    }
    static const struct syscall_fmt *syscall_fmt__find(const char *name)
    {
    let mut nmemb: c_int = ARRAY_SIZE(syscall_fmts);
    return __syscall_fmt__find(syscall_fmts, nmemb, name);
    }
    static const struct syscall_fmt *__syscall_fmt__find_by_alias(const struct syscall_fmt *fmts,
    const int nmemb, const char *alias)
    {
    int i;
    for (i = 0; i < nmemb; ++i) {
    if (fmts[i].alias && strcmp(fmts[i].alias, alias) == 0)
    return &fmts[i];
    }
    return core::ptr::null_mut();
    }
    static const struct syscall_fmt *syscall_fmt__find_by_alias(const char *alias)
    {
    let mut nmemb: c_int = ARRAY_SIZE(syscall_fmts);
    return __syscall_fmt__find_by_alias(syscall_fmts, nmemb, alias);
    }
//
// struct syscall
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall {
// @e_machine: The ELF machine associated with the entry.
    pub e_machine: c_int,
// @id: id value from the tracepoint, the system call number.
    pub id: c_int,
    pub tp_format: *mut tep_event,
    pub nr_args: c_int,
//
// @args_size: sum of the sizes of the syscall arguments, anything
// after that is augmented stuff: pathname for openat, etc.
//
    pub args_size: c_int,
    struct {
    struct bpf_program *sys_enter,
// sys_exit;
    pub bpf_prog: },
// @is_exit: is this "exit" or "exit_group"?
    pub is_exit: bool,
//
// @is_open: is this "open" or "openat"? To associate the fd returned in
// sys_exit with the pathname in sys_enter.
//
    pub is_open: bool,
//
// @nonexistent: Name lookup failed. Just a hole in the syscall table,
// syscall id not allocated.
//
    pub nonexistent: bool,
    pub use_btf: bool,
    pub args: *mut tep_format_field,
    pub name: *const c_char,
    pub fmt: *const syscall_fmt,
    pub arg_fmt: *mut syscall_arg_fmt,
}

//
// We need to have this 'calculated' boolean because in some cases we really
// don't know what is the duration of a syscall, for instance, when we start
// a session and some threads are waiting for a syscall to finish, say 'poll',
// in which case all we can do is to print "( ? ) for duration and for the
// start timestamp.
//
#[no_mangle]
unsafe extern "C" fn fprintf_duration(t: c_ulong, calculated: bool, fp: *mut FILE) -> usize {
    static size_t fprintf_duration(unsigned long t, bool calculated, FILE *fp)
    {
    let mut duration: double = (double)t / NSEC_PER_MSEC;
    let mut printed: usize = fprintf(fp, "(");
    if (!calculated)
    printed += fprintf(fp, "         ");
#[no_mangle]
pub unsafe extern "C" fn if(1.0: duration >=) -> else {
    else if (duration >= 1.0)
    printed += color_fprintf(fp, PERF_COLOR_RED, "%6.3f ms", duration);
#[no_mangle]
pub unsafe extern "C" fn if(0.01: duration >=) -> else {
    else if (duration >= 0.01)
    printed += color_fprintf(fp, PERF_COLOR_YELLOW, "%6.3f ms", duration);
    else
    printed += color_fprintf(fp, PERF_COLOR_NORMAL, "%6.3f ms", duration);
    return printed + fprintf(fp, "): ");
    }
//
// filename.ptr: The filename char pointer that will be vfs_getname'd
// filename.entry_str_pos: Where to insert the string translated from
// filename.ptr by the vfs_getname tracepoint/kprobe.
// ret_scnprintf: syscall args may set this to a different syscall return
// formatter, for instance, fcntl may return fds, file flags, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_trace {
    pub entry_time: u64,
    pub entry_cpu: u32,
    pub entry_pending: bool,
    pub nr_events: c_ulong,
    pub pfmin: unsigned long pfmaj,,
    pub entry_str: *mut c_char,
    pub runtime_ms: double,
    pub arg): *mut *mut *mut size_t (ret_scnprintf)(char bf, size_t size, struct syscall_arg,
    struct {
    pub ptr: c_ulong,
    pub entry_str_pos: short int,
    pub pending_open: bool,
    pub namelen: c_uint,
    pub name: *mut c_char,
    pub filename: },
    struct {
    pub max: c_int,
    pub table: *mut file,
    pub files: },
    pub syscall_stats: *mut hashmap,
}

#[no_mangle]
unsafe extern "C" fn syscall_id_hash(key: c_long, __maybe_unused: *mut *mut void ctx) -> usize {
    static size_t syscall_id_hash(long key, void *ctx __maybe_unused)
    {
    return key;
    }
#[no_mangle]
unsafe extern "C" fn syscall_id_equal(key1: c_long, key2: c_long, __maybe_unused: *mut *mut void ctx) -> bool {
    static bool syscall_id_equal(long key1, long key2, void *ctx __maybe_unused)
    {
    let mut key1: return = = key2;
    }
    static struct hashmap *alloc_syscall_stats(void)
    {
    struct hashmap *result = hashmap__new(syscall_id_hash, syscall_id_equal, core::ptr::null_mut());
    return IS_ERR(result) ? core::ptr::null_mut() : result;
    }
#[no_mangle]
unsafe extern "C" fn delete_syscall_stats(syscall_stats: *mut hashmap) {
    static void delete_syscall_stats(struct hashmap *syscall_stats)
    {
    struct hashmap_entry *pos;
    size_t bkt;
    if (!syscall_stats)
    return;
    hashmap__for_each_entry(syscall_stats, pos, bkt)
    zfree(&pos.pvalue);
    hashmap__free(syscall_stats);
    }
    static struct thread_trace *thread_trace__new(struct trace *trace)
    {
    struct thread_trace *ttrace =  zalloc(sizeof(struct thread_trace));
    if (ttrace) {
    ttrace.files.max = -1;
    if (trace.summary) {
    ttrace.syscall_stats = alloc_syscall_stats();
    if (!ttrace.syscall_stats)
    zfree(&ttrace);
    }
    }
    return ttrace;
    }
    static void thread_trace__free_files(struct thread_trace *ttrace);
#[no_mangle]
unsafe extern "C" fn thread_trace__delete(pttrace: *mut c_void) {
    static void thread_trace__delete(void *pttrace)
    {
    struct thread_trace *ttrace = pttrace;
    if (!ttrace)
    return;
    delete_syscall_stats(ttrace.syscall_stats);
    ttrace.syscall_stats = core::ptr::null_mut();
    thread_trace__free_files(ttrace);
    zfree(&ttrace.entry_str);
    free(ttrace);
    }
    static struct thread_trace *thread__trace(struct thread *thread, struct trace *trace)
    {
    struct thread_trace *ttrace;
    if (thread == core::ptr::null_mut())
    goto fail;
    if (thread__priv(thread) == core::ptr::null_mut())
    thread__set_priv(thread, thread_trace__new(trace));
    if (thread__priv(thread) == core::ptr::null_mut())
    goto fail;
    ttrace = thread__priv(thread);
    ++ttrace.nr_events;
    return ttrace;
    fail:
    color_fprintf(trace.output, PERF_COLOR_RED,
    "WARNING: not enough memory, dropping samples!\n");
    return core::ptr::null_mut();
    }
    void syscall_arg__set_ret_scnprintf(struct syscall_arg *arg,
    size_t (*ret_scnprintf)(char *bf, size_t size, struct syscall_arg *arg))
    {
    struct thread_trace *ttrace = thread__priv(arg.thread);
    ttrace.ret_scnprintf = ret_scnprintf;
    }

    let mut trace__entry_str_size: static size_t = 2048;
#[no_mangle]
unsafe extern "C" fn thread_trace__free_files(ttrace: *mut thread_trace) {
    static void thread_trace__free_files(struct thread_trace *ttrace)
    {
    for (int i = 0; i <= ttrace.files.max; ++i) {
    struct file *file = ttrace.files.table + i;
    zfree(&file.pathname);
    }
    zfree(&ttrace.files.table);
    ttrace.files.max  = -1;
    }
    static struct file *thread_trace__files_entry(struct thread_trace *ttrace, int fd)
    {
    if (fd < 0)
    return core::ptr::null_mut();
    if (fd > ttrace.files.max) {
    struct file *nfiles = realloc(ttrace.files.table, (fd + 1) * sizeof(struct file));
    if (nfiles == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (ttrace.files.max != -1) {
    memset(nfiles + ttrace.files.max + 1, 0,
    (fd - ttrace.files.max) * sizeof(struct file));
    } else {
    memset(nfiles, 0, (fd + 1) * sizeof(struct file));
    }
    ttrace.files.table = nfiles;
    ttrace.files.max   = fd;
    }
    return ttrace.files.table + fd;
    }
    struct file *thread__files_entry(struct thread *thread, int fd)
    {
    return thread_trace__files_entry(thread__priv(thread), fd);
    }
#[no_mangle]
unsafe extern "C" fn trace__set_fd_pathname(thread: *mut thread, fd: c_int, pathname: *const c_char) -> c_int {
    static int trace__set_fd_pathname(struct thread *thread, int fd, const char *pathname)
    {
    struct thread_trace *ttrace = thread__priv(thread);
    struct file *file = thread_trace__files_entry(ttrace, fd);
    if (file != core::ptr::null_mut()) {
    struct stat st;
    if (stat(pathname, &st) == 0)
    file.dev_maj = major(st.st_rdev);
    file.pathname = strdup(pathname);
    if (file.pathname)
    return 0;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn thread__read_fd_path(thread: *mut thread, fd: c_int) -> c_int {
    static int thread__read_fd_path(struct thread *thread, int fd)
    {
    char linkname[PATH_MAX], pathname[PATH_MAX];
    struct stat st;
    int ret;
    if (thread__pid(thread) == thread__tid(thread)) {
    scnprintf(linkname, sizeof(linkname),
    "/proc/%d/fd/%d", thread__pid(thread), fd);
    } else {
    scnprintf(linkname, sizeof(linkname),
    "/proc/%d/task/%d/fd/%d",
    thread__pid(thread), thread__tid(thread), fd);
    }
    if (lstat(linkname, &st) < 0 || st.st_size + 1 > (off_t)sizeof(pathname))
    return -1;
    ret = readlink(linkname, pathname, sizeof(pathname));
    if (ret < 0 || ret > st.st_size)
    return -1;
    pathname[ret] = '\0';
    return trace__set_fd_pathname(thread, fd, pathname);
    }
    static const char *thread__fd_path(struct thread *thread, int fd,
    struct trace *trace)
    {
    struct thread_trace *ttrace = thread__priv(thread);
    if (ttrace == core::ptr::null_mut() || trace.fd_path_disabled)
    return core::ptr::null_mut();
    if (fd < 0)
    return core::ptr::null_mut();
    if ((fd > ttrace.files.max || ttrace.files.table[fd].pathname == core::ptr::null_mut())) {
    if (!trace.live)
    return core::ptr::null_mut();
    ++trace.stats.proc_getname;
    if (thread__read_fd_path(thread, fd))
    return core::ptr::null_mut();
    }
    return ttrace.files.table[fd].pathname;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_fd(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_fd(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut fd: c_int = arg.val;
    let mut printed: usize = scnprintf(bf, size, "%d", fd);
    const char *path = thread__fd_path(arg.thread, fd, arg.trace);
    if (path)
    printed += scnprintf(bf + printed, size - printed, "<%s>", path);
    return printed;
    }
#[no_mangle]
pub unsafe extern "C" fn pid__scnprintf_fd(trace: *mut trace, pid: pid_t, fd: c_int, bf: *mut c_char, size: usize) -> usize {
    size_t pid__scnprintf_fd(struct trace *trace, pid_t pid, int fd, char *bf, size_t size)
    {
    let mut printed: usize = scnprintf(bf, size, "%d", fd);
    struct thread *thread = machine__find_thread(trace.host, pid, pid);
    if (thread) {
    const char *path = thread__fd_path(thread, fd, trace);
    if (path)
    printed += scnprintf(bf + printed, size - printed, "<%s>", path);
    thread__put(thread);
    }
    return printed;
    }
    static size_t syscall_arg__scnprintf_close_fd(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut fd: c_int = arg.val;
    let mut printed: usize = syscall_arg__scnprintf_fd(bf, size, arg);
    struct thread_trace *ttrace = thread__priv(arg.thread);
    if (ttrace && fd >= 0 && fd <= ttrace.files.max)
    zfree(&ttrace.files.table[fd].pathname);
    return printed;
    }
    static void thread__set_filename_pos(struct thread *thread, const char *bf,
    unsigned long ptr)
    {
    struct thread_trace *ttrace = thread__priv(thread);
    ttrace.filename.ptr = ptr;
    ttrace.filename.entry_str_pos = bf - ttrace.entry_str;
    }
#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_augmented_string(arg: *mut syscall_arg, bf: *mut c_char, size: usize) -> usize {
    static size_t syscall_arg__scnprintf_augmented_string(struct syscall_arg *arg, char *bf, size_t size)
    {
    struct augmented_arg *augmented_arg = arg.augmented.args;
    let mut printed: usize = scnprintf(bf, size, "\"%.*s\"", augmented_arg.size, augmented_arg.value);
//
// So that the next arg with a payload can consume its augmented arg, i.e. for rename* syscalls
// we would have two strings, each prefixed by its size.
//
    let mut consumed: c_int = sizeof(*augmented_arg) + augmented_arg.size;
    arg.augmented.args = ((void *)arg.augmented.args) + consumed;
    arg.augmented.size -= consumed;
    return printed;
    }
    static size_t syscall_arg__scnprintf_filename(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut ptr: c_ulong = arg.val;
    if (arg.augmented.args)
    return syscall_arg__scnprintf_augmented_string(arg, bf, size);
    if (!arg.trace.vfs_getname)
    return scnprintf(bf, size, "%#x", ptr);
    thread__set_filename_pos(arg.thread, bf, ptr);
    return 0;
    }
pub const MAX_CONTROL_CHAR: c_int = 31;
pub const MAX_ASCII: c_int = 127;
#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_buf(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    static size_t syscall_arg__scnprintf_buf(char *bf, size_t size, struct syscall_arg *arg)
    {
    struct augmented_arg *augmented_arg = arg.augmented.args;
    unsigned char *orig = (unsigned char *)augmented_arg.value;
    let mut printed: usize = 0;
    int consumed;
    if (augmented_arg == core::ptr::null_mut())
    return 0;
    for (int j = 0; j < augmented_arg.size; ++j) {
    let mut control_char: bool = orig[j] <= MAX_CONTROL_CHAR || orig[j] >= MAX_ASCII;
// print control characters (0~31 and 127), and non-ascii characters in \(digits)
    printed += scnprintf(bf + printed, size - printed, control_char ? "\\%d" : "%c", (int)orig[j]);
    }
    consumed = sizeof(*augmented_arg) + augmented_arg.size;
    arg.augmented.args = ((void *)arg.augmented.args) + consumed;
    arg.augmented.size -= consumed;
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn trace__filter_duration(trace: *mut trace, t: double) -> bool {
    static bool trace__filter_duration(struct trace *trace, double t)
    {
    return t < (trace.duration_filter * NSEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn __trace__fprintf_tstamp(trace: *mut trace, tstamp: u64, fp: *mut FILE) -> usize {
    static size_t __trace__fprintf_tstamp(struct trace *trace, u64 tstamp, FILE *fp)
    {
    let mut ts: double = (double)(tstamp - trace.base_time) / NSEC_PER_MSEC;
    return fprintf(fp, "%10.3f ", ts);
    }
//
// We're handling tstamp=0 as an undefined tstamp, i.e. like when we are
// using ttrace->entry_time for a thread that receives a sys_exit without
// first having received a sys_enter ("poll" issued before tracing session
// starts, lost sys_enter exit due to ring buffer overflow).
//
#[no_mangle]
unsafe extern "C" fn trace__fprintf_tstamp(trace: *mut trace, tstamp: u64, fp: *mut FILE) -> usize {
    static size_t trace__fprintf_tstamp(struct trace *trace, u64 tstamp, FILE *fp)
    {
    if (tstamp > 0)
    return __trace__fprintf_tstamp(trace, tstamp, fp);
    return fprintf(fp, "         ? ");
    }
//
// trace__fprintf_cpu - Print the CPU ID to a given file stream
// @cpu: The CPU ID to print
// @fp: The file stream to write to
//
// Formats and prints the specified CPU ID enclosed in brackets
// (e.g., "[003] ") to the provided file pointer. It is used to
// align and display the CPU ID consistently within the trace output.
//
// Return: The number of characters printed.
//
#[no_mangle]
unsafe extern "C" fn trace__fprintf_cpu(cpu: u32, fp: *mut FILE) -> usize {
    static size_t trace__fprintf_cpu(u32 cpu, FILE *fp)
    {
    let mut printed: usize = 0;
    if (cpu != (u32)-1)
    printed += fprintf(fp, "[%03u] ", cpu);
    return printed;
    }
    let mut workload_pid: static pid_t = -1;
    let mut done: static volatile sig_atomic_t = false;
    let mut interrupted: static volatile sig_atomic_t = false;
#[no_mangle]
unsafe extern "C" fn sighandler_interrupt(__maybe_unused: int sig) {
    static void sighandler_interrupt(int sig __maybe_unused)
    {
    done = interrupted = true;
    }
    static void sighandler_chld(int sig __maybe_unused, siginfo_t *info,
    void *context __maybe_unused)
    {
    if (info.si_pid == workload_pid)
    done = true;
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_comm_tid(trace: *mut trace, thread: *mut thread, fp: *mut FILE) -> usize {
    static size_t trace__fprintf_comm_tid(struct trace *trace, struct thread *thread, FILE *fp)
    {
    let mut printed: usize = 0;
    if (trace.multiple_threads) {
    if (trace.show_comm)
    printed += fprintf(fp, "%.14s/", thread__comm_str(thread));
    printed += fprintf(fp, "%d ", thread__tid(thread));
    }
    return printed;
    }
    static size_t trace__fprintf_entry_head(struct trace *trace, struct thread *thread,
    u64 duration, bool duration_calculated,
    u64 tstamp, u32 cpu, FILE *fp)
    {
    let mut printed: usize = 0;
    if (trace.show_tstamp)
    printed = trace__fprintf_tstamp(trace, tstamp, fp);
    if (trace.show_cpu && cpu != (u32)-1)
    printed += trace__fprintf_cpu(cpu, fp);
    if (trace.show_duration)
    printed += fprintf_duration(duration, duration_calculated, fp);
    return printed + trace__fprintf_comm_tid(trace, thread, fp);
    }
    static int trace__process_event(struct trace *trace, struct machine *machine,
    union perf_event *event, struct perf_sample *sample)
    {
    let mut ret: c_int = 0;
    switch (event.header.type) {
    case PERF_RECORD_LOST:
    color_fprintf(trace.output, PERF_COLOR_RED,
    "LOST %" PRIu64 " events!\n", (u64)event.lost.lost);
    ret = machine__process_lost_event(machine, event, sample);
    break;
    default:
    ret = machine__process_event(machine, event, sample);
    break;
    }
    return ret;
    }
    static int trace__tool_process(const struct perf_tool *tool,
    union perf_event *event,
    struct perf_sample *sample,
    struct machine *machine)
    {
    struct trace *trace = container_of(tool, struct trace, tool);
    return trace__process_event(trace, machine, event, sample);
    }
    static char *trace__machine__resolve_kernel_addr(void *vmachine, unsigned long long *addrp, char **modp)
    {
    struct machine *machine = vmachine;
    if (machine.kptr_restrict_warned)
    return core::ptr::null_mut();
    if (symbol_conf.kptr_restrict) {
    pr_warning("Kernel address maps (/proc/{kallsyms,modules}) are restricted.\n\n"
    "Check /proc/sys/kernel/kptr_restrict and /proc/sys/kernel/perf_event_paranoid.\n\n"
    "Kernel samples will not be resolved.\n");
    machine.kptr_restrict_warned = true;
    return core::ptr::null_mut();
    }
    return machine__resolve_kernel_addr(vmachine, addrp, modp);
    }
    static int trace__symbols_init(struct trace *trace, int argc, const char **argv,
    struct evlist *evlist)
    {
    let mut err: c_int = symbol__init(core::ptr::null_mut());
    if (err)
    return err;
    perf_env__init(&trace.host_env);
    err = perf_env__set_cmdline(&trace.host_env, argc, argv);
    if (err)
    goto out;
    trace.host = machine__new_host(&trace.host_env);
    if (trace.host == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out;
    }
    thread__set_priv_destructor(thread_trace__delete);
    err = trace_event__register_resolver(trace.host, trace__machine__resolve_kernel_addr);
    if (err < 0)
    goto out;
    if (trace.summary_only && trace.summary_mode != SUMMARY__BY_THREAD)
    goto out;
    err = __machine__synthesize_threads(trace.host, &trace.tool, &trace.opts.target,
    evlist__core(evlist).threads, trace__tool_process,
// needs_mmap=*/callchain_param.enabled &&
    !trace.summary_only,
// mmap_data=*/false,
// nr_threads_synthesize=*/1);
    out:
    if (err) {
    perf_env__exit(&trace.host_env);
    symbol__exit();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__symbols__exit(trace: *mut trace) {
    static void trace__symbols__exit(struct trace *trace)
    {
    machine__exit(trace.host);
    trace.host = core::ptr::null_mut();
    perf_env__exit(&trace.host_env);
    symbol__exit();
    }
#[no_mangle]
unsafe extern "C" fn syscall__alloc_arg_fmts(sc: *mut syscall, nr_args: c_int) -> c_int {
    static int syscall__alloc_arg_fmts(struct syscall *sc, int nr_args)
    {
    int idx;
    if (nr_args == RAW_SYSCALL_ARGS_NUM && sc.fmt && sc.fmt.nr_args != 0)
    nr_args = sc.fmt.nr_args;
    sc.arg_fmt = calloc(nr_args, sizeof(*sc.arg_fmt));
    if (sc.arg_fmt == core::ptr::null_mut())
    return -1;
    for (idx = 0; idx < nr_args; ++idx) {
    if (sc.fmt)
    sc.arg_fmt[idx] = sc.fmt.arg[idx];
    }
    sc.nr_args = nr_args;
    return 0;
    }
    static const struct syscall_arg_fmt syscall_arg_fmts__by_name[] = {
    { .name = "msr",	.scnprintf = SCA_X86_MSR,	  .strtoul = STUL_X86_MSR,	   },
    { .name = "vector",	.scnprintf = SCA_X86_IRQ_VECTORS, .strtoul = STUL_X86_IRQ_VECTORS, },
    };
#[no_mangle]
unsafe extern "C" fn syscall_arg_fmt__cmp(name: *const c_void, fmtp: *const c_void) -> c_int {
    static int syscall_arg_fmt__cmp(const void *name, const void *fmtp)
    {
    const struct syscall_arg_fmt *fmt = fmtp;
    return strcmp(name, fmt.name);
    }
    static const struct syscall_arg_fmt *
    __syscall_arg_fmt__find_by_name(const struct syscall_arg_fmt *fmts, const int nmemb,
    const char *name)
    {
    return bsearch(name, fmts, nmemb, sizeof(struct syscall_arg_fmt), syscall_arg_fmt__cmp);
    }
    static const struct syscall_arg_fmt *syscall_arg_fmt__find_by_name(const char *name)
    {
    let mut nmemb: c_int = ARRAY_SIZE(syscall_arg_fmts__by_name);
    return __syscall_arg_fmt__find_by_name(syscall_arg_fmts__by_name, nmemb, name);
    }
//
// v6.19 kernel added new fields to read userspace memory for event tracing.
// But it's not used by perf and confuses the syscall parameters.
//
#[no_mangle]
unsafe extern "C" fn is_internal_field(field: *mut tep_format_field) -> bool {
    static bool is_internal_field(struct tep_format_field *field)
    {
    return !strcmp(field.type, "__data_loc char[]");
    }
#[no_mangle]
unsafe extern "C" fn field_has_hex_fmt(field: *mut tep_format_field, len: c_int) -> bool {
    static bool field_has_hex_fmt(struct tep_format_field *field, int len)
    {
    const char *fmt, *pos, *end = core::ptr::null_mut();
    if (!field || !field.event || !field.event.print_fmt.format)
    return false;
    fmt = field.event.print_fmt.format;
// NB: Limit scanning strictly to the quoted printf format string
    if (*fmt == '"') {
    const char *p = ++fmt;
    while (*p) {
    if (*p == '\\' && p[1] != '\0') {
// NB: Skip escaped character
    p += 2;
    } else if (*p == '"') {
    end = p;
    break;
    } else {
    p++;
    }
    }
    } else {
    end = strchr(fmt, ',');
    }
    for (pos = strstr(fmt, field.name); pos && (!end || pos < end);
    pos = strstr(pos + 1, field.name)) {
    if (pos == fmt || !(isalnum(pos[-1]) || pos[-1] == '_')) {
    const char *after = pos + len;
    if (*after == '=' && (strstarts(after + 1, "0x") ||
    strstarts(after + 1, "%#") ||
    strstarts(after + 1, "%p")))
    return true;
    }
    }
    return false;
    }
    static struct tep_format_field *
    syscall_arg_fmt__init_array(struct syscall_arg_fmt *arg, struct tep_format_field *field,
    bool *use_btf)
    {
    struct tep_format_field *last_field = core::ptr::null_mut();
    int len;
    for (; field; field = field.next, ++arg) {
// assume it's the last argument
    if (is_internal_field(field))
    continue;
    last_field = field;
    if (arg.scnprintf)
    continue;
    len = strlen(field.name);
// As far as heuristics (or intention) goes this seems to hold true, and makes sense!
    if ((field.flags & TEP_FIELD_IS_POINTER) && strstarts(field.type, "const "))
    arg.from_user = true;
    if (strcmp(field.type, "const char *") == 0 &&
    ((len >= 4 && strcmp(field.name + len - 4, "name") == 0) ||
    strstr(field.name, "path") != core::ptr::null_mut())) {
    arg.scnprintf = SCA_FILENAME;
    } else if ((field.flags & TEP_FIELD_IS_POINTER) || strstr(field.name, "addr") ||
    field_has_hex_fmt(field, len))
    arg.scnprintf = SCA_PTR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(field->type, 0: "pid_t") ==) -> else {
    else if (strcmp(field.type, "pid_t") == 0)
    arg.scnprintf = SCA_PID;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(field->type, 0: "umode_t") ==) -> else {
    else if (strcmp(field.type, "umode_t") == 0)
    arg.scnprintf = SCA_MODE_T;
#[no_mangle]
pub unsafe extern "C" fn if(strstr(field->type: (field->flags & TEP_FIELD_IS_ARRAY) &&, _arg: "char")) -> else {
    arg.scnprintf = SCA_CHAR_ARRAY;
    arg.nr_entries = field.arraylen;
    } else if ((strcmp(field.type, "int") == 0 ||
    strcmp(field.type, "unsigned int") == 0 ||
    strcmp(field.type, "long") == 0) &&
    len >= 2 && strcmp(field.name + len - 2, "fd") == 0) {
//
// /sys/kernel/tracing/events/syscalls/sys_enter
// grep -E 'field:.*fd;' .../format|sed -r 's/.*field:([a-z ]+) [a-z_]*fd.+/\1/g'|sort|uniq -c
// 65 int
// 23 unsigned int
// 7 unsigned long
//
    arg.scnprintf = SCA_FD;
    } else if (strstr(field.type, "enum") && use_btf != core::ptr::null_mut()) {
// use_btf = true;
    arg.strtoul = STUL_BTF_TYPE;
    } else {
    const struct syscall_arg_fmt *fmt =
    syscall_arg_fmt__find_by_name(field.name);
    if (fmt) {
    arg.scnprintf = fmt.scnprintf;
    arg.strtoul   = fmt.strtoul;
    }
    }
    }
    return last_field;
    }
#[no_mangle]
unsafe extern "C" fn syscall__set_arg_fmts(sc: *mut syscall) -> c_int {
    static int syscall__set_arg_fmts(struct syscall *sc)
    {
    struct tep_format_field *last_field = syscall_arg_fmt__init_array(sc.arg_fmt, sc.args,
    &sc.use_btf);
    if (last_field)
    sc.args_size = last_field.offset + last_field.size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn syscall__read_info(sc: *mut syscall, trace: *mut trace) -> c_int {
    static int syscall__read_info(struct syscall *sc, struct trace *trace)
    {
    char tp_name[128];
    const char *name;
    struct tep_format_field *field;
    int err;
    if (sc.nonexistent)
    return -EEXIST;
    if (sc.name) {
// Info already read.
    return 0;
    }
    name = syscalltbl__name(sc.e_machine, sc.id);
    if (name == core::ptr::null_mut()) {
    sc.nonexistent = true;
    return -EEXIST;
    }
    sc.name = name;
    sc.fmt  = syscall_fmt__find(sc.name);
    snprintf(tp_name, sizeof(tp_name), "sys_enter_%s", sc.name);
    sc.tp_format = trace_event__tp_format("syscalls", tp_name);
    if (IS_ERR(sc.tp_format) && sc.fmt && sc.fmt.alias) {
    snprintf(tp_name, sizeof(tp_name), "sys_enter_%s", sc.fmt.alias);
    sc.tp_format = trace_event__tp_format("syscalls", tp_name);
    }
//
// Fails to read trace point format via sysfs node, so the trace point
// doesn't exist.  Set the 'nonexistent' flag as true.
//
    if (IS_ERR(sc.tp_format)) {
    sc.nonexistent = true;
    err = PTR_ERR(sc.tp_format);
    sc.tp_format = core::ptr::null_mut();
    return err;
    }
//
// The tracepoint format contains __syscall_nr field, so it's one more
// than the actual number of syscall arguments.
//
    if (syscall__alloc_arg_fmts(sc, sc.tp_format.format.nr_fields - 1))
    return -ENOMEM;
    sc.args = sc.tp_format.format.fields;
//
// We need to check and discard the first variable '__syscall_nr'
// or 'nr' that mean the syscall number. It is needless here.
// So drop '__syscall_nr' or 'nr' field but does not exist on older kernels.
//
    if (sc.args && (!strcmp(sc.args.name, "__syscall_nr") || !strcmp(sc.args.name, "nr"))) {
    sc.args = sc.args.next;
    --sc.nr_args;
    }
    field = sc.args;
    while (field) {
    if (is_internal_field(field))
    --sc.nr_args;
    field = field.next;
    }
    sc.is_exit = !strcmp(name, "exit_group") || !strcmp(name, "exit");
    sc.is_open = !strcmp(name, "open") || !strcmp(name, "openat");
    err = syscall__set_arg_fmts(sc);
// after calling syscall__set_arg_fmts() we'll know whether use_btf is true
    if (sc.use_btf)
    trace__load_vmlinux_btf(trace);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn evsel__init_tp_arg_scnprintf(evsel: *mut evsel, use_btf: *mut bool) -> c_int {
    static int evsel__init_tp_arg_scnprintf(struct evsel *evsel, bool *use_btf)
    {
    struct syscall_arg_fmt *fmt = evsel__syscall_arg_fmt(evsel);
    if (fmt != core::ptr::null_mut()) {
    const struct tep_event *tp_format = evsel__tp_format(evsel);
    if (tp_format) {
    syscall_arg_fmt__init_array(fmt, tp_format.format.fields, use_btf);
    return 0;
    }
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn intcmp(a: *const c_void, b: *const c_void) -> c_int {
    static int intcmp(const void *a, const void *b)
    {
    const int *one = a, *another = b;
    return *one - *another;
    }
#[no_mangle]
unsafe extern "C" fn trace__validate_ev_qualifier(trace: *mut trace) -> c_int {
    static int trace__validate_ev_qualifier(struct trace *trace)
    {
    let mut err: c_int = 0;
    let mut printed_invalid_prefix: bool = false;
    struct str_node *pos;
    let mut nr_used: usize = 0, nr_allocated = strlist__nr_entries(trace.ev_qualifier);
    trace.ev_qualifier_ids.entries = calloc(nr_allocated, sizeof(trace.ev_qualifier_ids.entries[0]));
    if (trace.ev_qualifier_ids.entries == core::ptr::null_mut()) {
    fputs("Error:\tNot enough memory for allocating events qualifier ids\n",
    trace.output);
    err = -EINVAL;
    goto out;
    }
    strlist__for_each_entry(pos, trace.ev_qualifier) {
    const char *sc = pos.s;
//
// TODO: Assume more than the validation/warnings are all for
// the same binary type as perf.
//
    let mut id: c_int = syscalltbl__id(EM_HOST, sc), match_next = -1;
    if (id < 0) {
    id = syscalltbl__strglobmatch_first(EM_HOST, sc, &match_next);
    if (id >= 0)
    goto matches;
    if (!printed_invalid_prefix) {
    pr_debug("Skipping unknown syscalls: ");
    printed_invalid_prefix = true;
    } else {
    pr_debug(", ");
    }
    pr_debug("%s", sc);
    continue;
    }
    matches:
    trace.ev_qualifier_ids.entries[nr_used++] = id;
    if (match_next == -1)
    continue;
    while (1) {
    id = syscalltbl__strglobmatch_next(EM_HOST, sc, &match_next);
    if (id < 0)
    break;
    if (nr_allocated == nr_used) {
    void *entries;
    nr_allocated += 8;
    entries = realloc(trace.ev_qualifier_ids.entries,
    nr_allocated * sizeof(trace.ev_qualifier_ids.entries[0]));
    if (entries == core::ptr::null_mut()) {
    err = -ENOMEM;
    fputs("\nError:\t Not enough memory for parsing\n", trace.output);
    goto out_free;
    }
    trace.ev_qualifier_ids.entries = entries;
    }
    trace.ev_qualifier_ids.entries[nr_used++] = id;
    }
    }
    trace.ev_qualifier_ids.nr = nr_used;
    qsort(trace.ev_qualifier_ids.entries, nr_used, sizeof(int), intcmp);
    out:
    if (printed_invalid_prefix)
    pr_debug("\n");
    return err;
    out_free:
    zfree(&trace.ev_qualifier_ids.entries);
    trace.ev_qualifier_ids.nr = 0;
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn trace__syscall_enabled(trace: *mut trace, id: c_int) -> __maybe_unused bool {
    static __maybe_unused bool trace__syscall_enabled(struct trace *trace, int id)
    {
    bool in_ev_qualifier;
    if (trace.ev_qualifier_ids.nr == 0)
    return true;
    in_ev_qualifier = bsearch(&id, trace.ev_qualifier_ids.entries,
    trace.ev_qualifier_ids.nr, sizeof(int), intcmp) != core::ptr::null_mut();
    if (in_ev_qualifier)
    return !trace.not_ev_qualifier;
    return trace.not_ev_qualifier;
    }
//
// args is to be interpreted as a series of longs but we need to handle
// 8-byte unaligned accesses. args points to raw_data within the event
// and raw_data is guaranteed to be 8-byte unaligned because it is
// preceded by raw_size which is a u32. So we need to copy args to a temp
// variable to read it. Most notably this avoids extended load instructions
// on unaligned addresses
//
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__val(arg: *mut syscall_arg, idx: u8) -> c_ulong {
    unsigned long syscall_arg__val(struct syscall_arg *arg, u8 idx)
    {
    unsigned long val;
    unsigned char *p = arg.args + sizeof(unsigned long) * idx;
    memcpy(&val, p, sizeof(val));
    return val;
    }
    static size_t syscall__scnprintf_name(struct syscall *sc, char *bf, size_t size,
    struct syscall_arg *arg)
    {
    if (sc.arg_fmt && sc.arg_fmt[arg.idx].name)
    return scnprintf(bf, size, "%s: ", sc.arg_fmt[arg.idx].name);
    return scnprintf(bf, size, "arg%d: ", arg.idx);
    }
//
// Check if the value is in fact zero, i.e. mask whatever needs masking, such
// as mount 'flags' argument that needs ignoring some magic flag, see comment
// in tools/perf/trace/beauty/mount_flags.c
//
#[no_mangle]
unsafe extern "C" fn syscall_arg_fmt__mask_val(fmt: *mut syscall_arg_fmt, arg: *mut syscall_arg, val: c_ulong) -> c_ulong {
    static unsigned long syscall_arg_fmt__mask_val(struct syscall_arg_fmt *fmt, struct syscall_arg *arg, unsigned long val)
    {
    if (fmt && fmt.mask_val)
    return fmt.mask_val(arg, val);
    return val;
    }
    static size_t syscall_arg_fmt__scnprintf_val(struct syscall_arg_fmt *fmt, char *bf, size_t size,
    struct syscall_arg *arg, unsigned long val)
    {
    if (fmt && fmt.scnprintf) {
    arg.val = val;
    if (fmt.parm)
    arg.parm = fmt.parm;
    return fmt.scnprintf(bf, size, arg);
    }
    return scnprintf(bf, size, "%ld", val);
    }
    static size_t syscall__scnprintf_args(struct syscall *sc, char *bf, size_t size,
    unsigned char *args, void *augmented_args, int augmented_args_size,
    struct trace *trace, struct thread *thread)
    {
    let mut printed: usize = 0, btf_printed;
    unsigned long val;
    let mut bit: u8 = 1;
    struct syscall_arg arg = {
    .args	= args,
    .augmented = {
    .size = augmented_args_size,
    .args = augmented_args,
    },
    .idx	= 0,
    .mask	= 0,
    .trace  = trace,
    .thread = thread,
    .show_string_prefix = trace.show_string_prefix,
    };
    struct thread_trace *ttrace = thread__priv(thread);
    void *default_scnprintf;
//
// Things like fcntl will set this in its 'cmd' formatter to pick the
// right formatter for the return value (an fd? file flags?), which is
// not needed for syscalls that always return a given type, say an fd.
//
    ttrace.ret_scnprintf = core::ptr::null_mut();
    if (sc.args != core::ptr::null_mut()) {
    struct tep_format_field *field;
    for (field = sc.args; field;
    field = field.next, ++arg.idx, bit <<= 1) {
    if (arg.mask & bit)
    continue;
    arg.fmt = &sc.arg_fmt[arg.idx];
    val = syscall_arg__val(&arg, arg.idx);
//
// Some syscall args need some mask, most don't and
// return val untouched.
//
    val = syscall_arg_fmt__mask_val(&sc.arg_fmt[arg.idx], &arg, val);
//
// Suppress this argument if its value is zero and show_zero
// property isn't set.
//
// If it has a BTF type, then override the zero suppression knob
// as the common case is for zero in an enum to have an associated entry.
//
    if (val == 0 && !trace.show_zeros &&
    !(sc.arg_fmt && sc.arg_fmt[arg.idx].show_zero) &&
    !(sc.arg_fmt && sc.arg_fmt[arg.idx].strtoul == STUL_BTF_TYPE))
    continue;
    printed += scnprintf(bf + printed, size - printed, "%s", printed ? ", " : "");
    if (trace.show_arg_names)
    printed += scnprintf(bf + printed, size - printed, "%s: ", field.name);
    default_scnprintf = sc.arg_fmt[arg.idx].scnprintf;
    if (trace.force_btf || default_scnprintf == core::ptr::null_mut() || default_scnprintf == SCA_PTR) {
    btf_printed = trace__btf_scnprintf(trace, &arg, bf + printed,
    size - printed, val, field.type);
    if (btf_printed) {
    printed += btf_printed;
    continue;
    }
    }
    printed += syscall_arg_fmt__scnprintf_val(&sc.arg_fmt[arg.idx],
    bf + printed, size - printed, &arg, val);
    }
    } else if (IS_ERR(sc.tp_format)) {
//
// If we managed to read the tracepoint /format file, then we
// may end up not having any args, like with gettid(), so only
// print the raw args when we didn't manage to read it.
//
    while (arg.idx < sc.nr_args) {
    if (arg.mask & bit)
    goto next_arg;
    val = syscall_arg__val(&arg, arg.idx);
    if (printed)
    printed += scnprintf(bf + printed, size - printed, ", ");
    printed += syscall__scnprintf_name(sc, bf + printed, size - printed, &arg);
    printed += syscall_arg_fmt__scnprintf_val(&sc.arg_fmt[arg.idx], bf + printed, size - printed, &arg, val);
    next_arg:
    ++arg.idx;
    bit <<= 1;
    }
    }
    return printed;
    }
    static struct syscall *syscall__new(int e_machine, int id)
    {
    struct syscall *sc = zalloc(sizeof(*sc));
    if (!sc)
    return core::ptr::null_mut();
    sc.e_machine = e_machine;
    sc.id = id;
    return sc;
    }
#[no_mangle]
unsafe extern "C" fn syscall__delete(sc: *mut syscall) {
    static void syscall__delete(struct syscall *sc)
    {
    if (!sc)
    return;
    free(sc.arg_fmt);
    free(sc);
    }
#[no_mangle]
unsafe extern "C" fn syscall__bsearch_cmp(key: *const c_void, entry: *const c_void) -> c_int {
    static int syscall__bsearch_cmp(const void *key, const void *entry)
    {
    const struct syscall *a = key, *b = *((const struct syscall **)entry);
    if (a.e_machine != b.e_machine)
    return a.e_machine - b.e_machine;
    return a.id - b.id;
    }
#[no_mangle]
unsafe extern "C" fn syscall__cmp(va: *const c_void, vb: *const c_void) -> c_int {
    static int syscall__cmp(const void *va, const void *vb)
    {
    const struct syscall *a = *((const struct syscall **)va);
    const struct syscall *b = *((const struct syscall **)vb);
    if (a.e_machine != b.e_machine)
    return a.e_machine - b.e_machine;
    return a.id - b.id;
    }
    static struct syscall *trace__find_syscall(struct trace *trace, int e_machine, int id)
    {
    struct syscall key = {
    .e_machine = e_machine,
    .id = id,
    };
    struct syscall *sc, **tmp;
    if (trace.syscalls.table) {
    struct syscall **sc_entry = bsearch(&key, trace.syscalls.table,
    trace.syscalls.table_size,
    sizeof(trace.syscalls.table[0]),
    syscall__bsearch_cmp);
    if (sc_entry)
    return *sc_entry;
    }
    sc = syscall__new(e_machine, id);
    if (!sc)
    return core::ptr::null_mut();
    tmp = reallocarray(trace.syscalls.table, trace.syscalls.table_size + 1,
    sizeof(trace.syscalls.table[0]));
    if (!tmp) {
    syscall__delete(sc);
    return core::ptr::null_mut();
    }
    trace.syscalls.table = tmp;
    trace.syscalls.table[trace.syscalls.table_size++] = sc;
    qsort(trace.syscalls.table, trace.syscalls.table_size, sizeof(trace.syscalls.table[0]),
    syscall__cmp);
    return sc;
    }
    typedef int (*tracepoint_handler)(struct trace *trace,
    union perf_event *event,
    struct perf_sample *sample);
    static struct syscall *trace__syscall_info(struct trace *trace, struct evsel *evsel,
    int e_machine, int id)
    {
    struct syscall *sc;
    let mut err: c_int = 0;
    if (id < 0) {
//
// XXX: Noticed on x86_64, reproduced as far back as 3.0.36, haven't tried
// before that, leaving at a higher verbosity level till that is
// explained. Reproduced with plain ftrace with:
//
// echo 1 > /t/events/raw_syscalls/sys_exit/enable
// grep "NR -1 " /t/trace_pipe
//
// After generating some load on the machine.
//
    if (verbose > 1) {
    static u64 n;
    fprintf(trace.output, "Invalid syscall %d id, skipping (%s, %" PRIu64 ") ...\n",
    id, evsel__name(evsel), ++n);
    }
    return core::ptr::null_mut();
    }
    err = -EINVAL;
    sc = trace__find_syscall(trace, e_machine, id);
    if (sc)
    err = syscall__read_info(sc, trace);
    if (err && verbose > 0) {
    errno = -err;
    fprintf(trace.output, "Problems reading syscall %d: %m", id);
    if (sc && sc.name)
    fprintf(trace.output, " (%s)", sc.name);
    fputs(" information\n", trace.output);
    }
    return err ? core::ptr::null_mut() : sc;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_stats {
    pub stats: stats,
    pub nr_failures: u64,
    pub max_errno: c_int,
    pub errnos: *mut u32,
}

    static void thread__update_stats(struct thread *thread, struct thread_trace *ttrace,
    int id, struct perf_sample *sample, long err,
    struct trace *trace)
    {
    struct hashmap *syscall_stats = ttrace.syscall_stats;
    struct syscall_stats *stats = core::ptr::null_mut();
    let mut duration: u64 = 0;
    if (trace.summary_bpf)
    return;
    if (trace.summary_mode == SUMMARY__BY_TOTAL)
    syscall_stats = trace.syscall_stats;
    if (!hashmap__find(syscall_stats, id, &stats)) {
    stats = zalloc(sizeof(*stats));
    if (stats == core::ptr::null_mut())
    return;
    init_stats(&stats.stats);
    if (hashmap__add(syscall_stats, id, stats) < 0) {
    free(stats);
    return;
    }
    }
    if (ttrace.entry_time && sample.time > ttrace.entry_time)
    duration = sample.time - ttrace.entry_time;
    update_stats(&stats.stats, duration);
    if (err < 0) {
    ++stats.nr_failures;
    if (!trace.errno_summary)
    return;
    err = -err;
    if (err > stats.max_errno) {
    u32 *new_errnos = realloc(stats.errnos, err * sizeof(u32));
    if (new_errnos) {
    memset(new_errnos + stats.max_errno, 0, (err - stats.max_errno) * sizeof(u32));
    } else {
    pr_debug("Not enough memory for errno stats for thread \"%s\"(%d/%d), results will be incomplete\n",
    thread__comm_str(thread), thread__pid(thread),
    thread__tid(thread));
    return;
    }
    stats.errnos = new_errnos;
    stats.max_errno = err;
    }
    ++stats.errnos[err - 1];
    }
    }
#[no_mangle]
unsafe extern "C" fn trace__printf_interrupted_entry(trace: *mut trace) -> c_int {
    static int trace__printf_interrupted_entry(struct trace *trace)
    {
    struct thread_trace *ttrace;
    size_t printed;
    int len;
    if (trace.failure_only || trace.current == core::ptr::null_mut())
    return 0;
    ttrace = thread__priv(trace.current);
    if (!ttrace.entry_pending)
    return 0;
    printed = trace__fprintf_entry_head(trace, trace.current, 0, false,
    ttrace.entry_time, ttrace.entry_cpu,
    trace.output);
    printed += len = fprintf(trace.output, "%s)", ttrace.entry_str);
    if (len < trace.args_alignment - 4)
    printed += fprintf(trace.output, "%-*s", trace.args_alignment - 4 - len, " ");
    printed += fprintf(trace.output, " ...\n");
    ttrace.entry_pending = false;
    ++trace.nr_events_printed;
    return printed;
    }
    static int trace__fprintf_sample(struct trace *trace, struct perf_sample *sample,
    struct thread *thread)
    {
    let mut printed: c_int = 0;
    if (trace.print_sample) {
    let mut ts: double = (double)sample.time / NSEC_PER_MSEC;
    printed += fprintf(trace.output, "%22s %10.3f %s %d/%d [%d]\n",
    evsel__name(sample.evsel), ts,
    thread__comm_str(thread),
    sample.pid, sample.tid, sample.cpu);
    }
    return printed;
    }
    static void *syscall__augmented_args(struct syscall *sc, struct perf_sample *sample, int *augmented_args_size, int raw_augmented_args_size)
    {
//
// For now with BPF raw_augmented we hook into raw_syscalls:sys_enter
// and there we get all 6 syscall args plus the tracepoint common fields
// that gets calculated at the start and the syscall_nr (another long).
// So we check if that is the case and if so don't look after the
// sc->args_size but always after the full raw_syscalls:sys_enter payload,
// which is fixed.
//
// We'll revisit this later to pass s->args_size to the BPF augmenter
// (now tools/perf/examples/bpf/augmented_raw_syscalls.c, so that it
// copies only what we need for each syscall, like what happens when we
// use syscalls:sys_enter_NAME, so that we reduce the kernel/userspace
// traffic to just what is needed for each syscall.
//
    let mut args_size: c_int = raw_augmented_args_size ?: sc.args_size;
// augmented_args_size = sample->raw_size - args_size;
    if (*augmented_args_size > 0) {
    static uintptr_t argbuf[1024]; /* assuming single-threaded */
    if ((size_t)(*augmented_args_size) > sizeof(argbuf))
    return core::ptr::null_mut();
//
// The perf ring-buffer is 8-byte aligned but sample->raw_data
// is not because it's preceded by u32 size.  Later, beautifier
// will use the augmented args with stricter alignments like in
// some struct.  To make sure it's aligned, let's copy the args
// into a static buffer as it's single-threaded for now.
//
    memcpy(argbuf, sample.raw_data + args_size, *augmented_args_size);
    return argbuf;
    }
    return core::ptr::null_mut();
    }
    static int trace__sys_enter(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    struct evsel *evsel = sample.evsel;
    char *msg;
    void *args;
    let mut printed: c_int = 0;
    struct thread *thread;
    let mut id: c_int = perf_evsel__sc_tp_uint(id, sample), err = -1;
    let mut augmented_args_size: c_int = 0, e_machine;
    void *augmented_args = core::ptr::null_mut();
    struct syscall *sc;
    struct thread_trace *ttrace;
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    e_machine = thread__e_machine(thread, trace.host, /*e_flags=*/core::ptr::null_mut());
    sc = trace__syscall_info(trace, evsel, e_machine, id);
    if (sc == core::ptr::null_mut())
    goto out_put;
    ttrace = thread__trace(thread, trace);
    if (ttrace == core::ptr::null_mut())
    goto out_put;
    trace__fprintf_sample(trace, sample, thread);
    args = perf_evsel__sc_tp_ptr(args, sample);
    if (ttrace.entry_str == core::ptr::null_mut()) {
    ttrace.entry_str = malloc(trace__entry_str_size);
    if (!ttrace.entry_str)
    goto out_put;
    }
    if (!(trace.duration_filter || trace.summary_only || trace.min_stack))
    trace__printf_interrupted_entry(trace);
//
// If this is raw_syscalls.sys_enter, then it always comes with the 6 possible
// arguments, even if the syscall being handled, say "openat", uses only 4 arguments
// this breaks syscall__augmented_args() check for augmented args, as we calculate
// syscall->args_size using each syscalls:sys_enter_NAME tracefs format file,
// so when handling, say the openat syscall, we end up getting 6 args for the
// raw_syscalls:sys_enter event, when we expected just 4, we end up mistakenly
// thinking that the extra 2 u64 args are the augmented filename, so just check
// here and avoid using augmented syscalls when the evsel is the raw_syscalls one.
//
    if (evsel != trace.syscalls.events.sys_enter)
    augmented_args = syscall__augmented_args(sc, sample, &augmented_args_size, trace.raw_augmented_syscalls_args_size);
    ttrace.entry_time = sample.time;
    ttrace.entry_cpu = sample.cpu;
    msg = ttrace.entry_str;
    printed += scnprintf(msg + printed, trace__entry_str_size - printed, "%s(", sc.name);
    printed += syscall__scnprintf_args(sc, msg + printed, trace__entry_str_size - printed,
    args, augmented_args, augmented_args_size, trace, thread);
    if (sc.is_exit) {
    if (!(trace.duration_filter || trace.summary_only || trace.failure_only || trace.min_stack)) {
    let mut alignment: c_int = 0;
    trace__fprintf_entry_head(trace, thread, 0, false,
    ttrace.entry_time,
    sample.cpu, trace.output);
    printed = fprintf(trace.output, "%s)", ttrace.entry_str);
    if (trace.args_alignment > printed)
    alignment = trace.args_alignment - printed;
    fprintf(trace.output, "%*s= ?\n", alignment, " ");
    }
    } else {
    ttrace.entry_pending = true;
// See trace__vfs_getname & trace__sys_exit
    ttrace.filename.pending_open = false;
    }
    if (trace.current != thread) {
    thread__put(trace.current);
    trace.current = thread__get(thread);
    }
    err = 0;
    out_put:
    thread__put(thread);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_sys_enter(trace: *mut trace, sample: *mut perf_sample) -> c_int {
    static int trace__fprintf_sys_enter(struct trace *trace, struct perf_sample *sample)
    {
    struct thread_trace *ttrace;
    struct thread *thread;
    let mut id: c_int = perf_evsel__sc_tp_uint(id, sample), err = -1;
    struct syscall *sc;
    char msg[1024];
    void *args, *augmented_args = core::ptr::null_mut();
    int augmented_args_size, e_machine;
    let mut printed: usize = 0;
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    e_machine = thread__e_machine(thread, trace.host, /*e_flags=*/core::ptr::null_mut());
    sc = trace__syscall_info(trace, sample.evsel, e_machine, id);
    if (sc == core::ptr::null_mut())
    goto out_put;
    ttrace = thread__trace(thread, trace);
//
// We need to get ttrace just to make sure it is there when syscall__scnprintf_args()
// and the rest of the beautifiers accessing it via struct syscall_arg touches it.
//
    if (ttrace == core::ptr::null_mut())
    goto out_put;
    args = perf_evsel__sc_tp_ptr(args, sample);
    augmented_args = syscall__augmented_args(sc, sample, &augmented_args_size, trace.raw_augmented_syscalls_args_size);
    printed += syscall__scnprintf_args(sc, msg, sizeof(msg), args, augmented_args, augmented_args_size, trace, thread);
    fprintf(trace.output, "%.*s", (int)printed, msg);
    err = 0;
    out_put:
    thread__put(thread);
    return err;
    }
    static int trace__resolve_callchain(struct trace *trace,
    struct perf_sample *sample,
    struct callchain_cursor *cursor)
    {
    struct evsel *evsel = sample.evsel;
    struct addr_location al;
    int max_stack = evsel.core.attr.sample_max_stack ?
    evsel.core.attr.sample_max_stack :
    trace.max_stack;
    let mut err: c_int = -1;
    addr_location__init(&al);
    if (machine__resolve(trace.host, &al, sample) < 0)
    goto out;
    err = thread__resolve_callchain(al.thread, cursor, sample, core::ptr::null_mut(), core::ptr::null_mut(), max_stack);
    out:
    addr_location__exit(&al);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_callchain(trace: *mut trace, sample: *mut perf_sample) -> c_int {
    static int trace__fprintf_callchain(struct trace *trace, struct perf_sample *sample)
    {
// TODO: user-configurable print_opts
    const unsigned int print_opts = EVSEL__PRINT_SYM |
    EVSEL__PRINT_DSO |
    EVSEL__PRINT_UNKNOWN_AS_ADDR;
    return sample__fprintf_callchain(sample, 38, print_opts, get_tls_callchain_cursor(), symbol_conf.bt_stop_list, trace.output);
    }
    static int trace__sys_exit(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    struct evsel *evsel = sample.evsel;
    long ret;
    let mut duration: u64 = 0;
    let mut duration_calculated: bool = false;
    struct thread *thread;
    let mut id: c_int = perf_evsel__sc_tp_uint(id, sample), err = -1, callchain_ret = 0, printed = 0;
    let mut alignment: c_int = trace.args_alignment, e_machine;
    struct syscall *sc;
    struct thread_trace *ttrace;
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    e_machine = thread__e_machine(thread, trace.host, /*e_flags=*/core::ptr::null_mut());
    sc = trace__syscall_info(trace, evsel, e_machine, id);
    if (sc == core::ptr::null_mut())
    goto out_put;
    ttrace = thread__trace(thread, trace);
    if (ttrace == core::ptr::null_mut())
    goto out_put;
    trace__fprintf_sample(trace, sample, thread);
    ret = perf_evsel__sc_tp_uint(ret, sample);
    if (trace.summary)
    thread__update_stats(thread, ttrace, id, sample, ret, trace);
    if (!trace.fd_path_disabled && sc.is_open && ret >= 0 && ttrace.filename.pending_open) {
    trace__set_fd_pathname(thread, ret, ttrace.filename.name);
    ttrace.filename.pending_open = false;
    ++trace.stats.vfs_getname;
    }
    if (ttrace.entry_time && sample.time >= ttrace.entry_time) {
    duration = sample.time - ttrace.entry_time;
    if (trace__filter_duration(trace, duration))
    goto out;
    duration_calculated = true;
    } else if (trace.duration_filter)
    goto out;
    if (sample.callchain) {
    struct callchain_cursor *cursor = get_tls_callchain_cursor();
    callchain_ret = trace__resolve_callchain(trace, sample, cursor);
    if (callchain_ret == 0) {
    if (cursor.nr < trace.min_stack)
    goto out;
    callchain_ret = 1;
    }
    }
    if (trace.summary_only || (ret >= 0 && trace.failure_only))
    goto out;
    trace__fprintf_entry_head(trace, thread, duration,
    duration_calculated, ttrace.entry_time,
    sample.cpu, trace.output);
    if (ttrace.entry_pending) {
    printed = fprintf(trace.output, "%s", ttrace.entry_str);
    } else {
    printed += fprintf(trace.output, " ... [");
    color_fprintf(trace.output, PERF_COLOR_YELLOW, "continued");
    printed += 9;
    printed += fprintf(trace.output, "]: %s()", sc.name);
    }
    printed++; /* the closing ')' */
    if (alignment > printed)
    alignment -= printed;
    else
    alignment = 0;
    fprintf(trace.output, ")%*s= ", alignment, " ");
    if (sc.fmt == core::ptr::null_mut()) {
    if (ret < 0)
    goto errno_print;
    signed_print:
    fprintf(trace.output, "%ld", ret);
    } else if (ret < 0) {
    errno_print: {
    char bf[STRERR_BUFSIZE];
    const char *emsg = str_error_r(-ret, bf, sizeof(bf));
    const char *e = perf_env__arch_strerrno(e_machine, err);
    fprintf(trace.output, "-1 %s (%s)", e, emsg);
    }
    } else if (ret == 0 && sc.fmt.timeout)
    fprintf(trace.output, "0 (Timeout)");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ttrace->ret_scnprintf) -> else {
    char bf[1024];
    struct syscall_arg arg = {
    .val	= ret,
    .thread	= thread,
    .trace	= trace,
    };
    ttrace.ret_scnprintf(bf, sizeof(bf), &arg);
    ttrace.ret_scnprintf = core::ptr::null_mut();
    fprintf(trace.output, "%s", bf);
    } else if (sc.fmt.hexret)
    fprintf(trace.output, "%#lx", ret);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sc->fmt->errpid) -> else {
    struct thread *child = machine__find_thread(trace.host, ret, ret);
    fprintf(trace.output, "%ld", ret);
    if (child != core::ptr::null_mut()) {
    if (thread__comm_set(child))
    fprintf(trace.output, " (%s)", thread__comm_str(child));
    thread__put(child);
    }
    } else
    goto signed_print;
    fputc('\n', trace.output);
//
// We only consider an 'event' for the sake of --max-events a non-filtered
// sys_enter + sys_exit and other tracepoint events.
//
    if (++trace.nr_events_printed == trace.max_events && trace.max_events != ULONG_MAX)
    interrupted = true;
    if (callchain_ret > 0)
    trace__fprintf_callchain(trace, sample);
#[no_mangle]
pub unsafe extern "C" fn if(0: callchain_ret <) -> else {
    else if (callchain_ret < 0)
    pr_err("Problem processing %s callchain, skipping...\n", evsel__name(evsel));
    out:
    ttrace.entry_pending = false;
    err = 0;
    out_put:
    thread__put(thread);
    return err;
    }
    static int trace__vfs_getname(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    struct thread *thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    struct thread_trace *ttrace;
    size_t filename_len, entry_str_len, to_move;
    ssize_t remaining_space;
    char *pos;
    const char *filename = perf_sample__strval(sample, "pathname");
    if (!thread)
    goto out;
    ttrace = thread__priv(thread);
    if (!ttrace)
    goto out_put;
    filename_len = strlen(filename);
    if (filename_len == 0)
    goto out_put;
    if (ttrace.filename.namelen < filename_len) {
    char *f = realloc(ttrace.filename.name, filename_len + 1);
    if (f == core::ptr::null_mut())
    goto out_put;
    ttrace.filename.namelen = filename_len;
    ttrace.filename.name = f;
    }
    strcpy(ttrace.filename.name, filename);
    ttrace.filename.pending_open = true;
    if (!ttrace.filename.ptr)
    goto out_put;
    entry_str_len = strlen(ttrace.entry_str);
    remaining_space = trace__entry_str_size - entry_str_len - 1; /* \0 */
    if (remaining_space <= 0)
    goto out_put;
    if (filename_len > (size_t)remaining_space) {
    filename += filename_len - remaining_space;
    filename_len = remaining_space;
    }
    to_move = entry_str_len - ttrace.filename.entry_str_pos + 1; /* \0 */
    pos = ttrace.entry_str + ttrace.filename.entry_str_pos;
    memmove(pos + filename_len, pos, to_move);
    memcpy(pos, filename, filename_len);
    ttrace.filename.ptr = 0;
    ttrace.filename.entry_str_pos = 0;
    out_put:
    thread__put(thread);
    out:
    return 0;
    }
    static int trace__sched_stat_runtime(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    let mut runtime: u64 = perf_sample__intval(sample, "runtime");
    let mut runtime_ms: double = (double)runtime / NSEC_PER_MSEC;
    struct thread *thread = machine__findnew_thread(trace.host,
    sample.pid,
    sample.tid);
    struct thread_trace *ttrace = thread__trace(thread, trace);
    if (ttrace == core::ptr::null_mut())
    goto out_dump;
    ttrace.runtime_ms += runtime_ms;
    trace.runtime_ms += runtime_ms;
    out_put:
    thread__put(thread);
    return 0;
    out_dump:
    fprintf(trace.output, "%s: comm=%s,pid=%u,runtime=%" PRIu64 ",vruntime=%" PRIu64 ")\n",
    sample.evsel.name,
    perf_sample__strval(sample, "comm"),
    (pid_t)perf_sample__intval(sample, "pid"),
    runtime,
    perf_sample__intval(sample, "vruntime"));
    goto out_put;
    }
    static int bpf_output__printer(enum binary_printer_ops op,
    unsigned int val, void *extra __maybe_unused, FILE *fp)
    {
    let mut ch: c_uchar = (unsigned char)val;
    switch (op) {
    case BINARY_PRINT_CHAR_DATA:
    return fprintf(fp, "%c", isprint(ch) ? ch : '.');
    case BINARY_PRINT_DATA_BEGIN:
    case BINARY_PRINT_LINE_BEGIN:
    case BINARY_PRINT_ADDR:
    case BINARY_PRINT_NUM_DATA:
    case BINARY_PRINT_NUM_PAD:
    case BINARY_PRINT_SEP:
    case BINARY_PRINT_CHAR_PAD:
    case BINARY_PRINT_LINE_END:
    case BINARY_PRINT_DATA_END:
    default:
    break;
    }
    return 0;
    }
    static void bpf_output__fprintf(struct trace *trace,
    struct perf_sample *sample)
    {
    binary__fprintf(sample.raw_data, sample.raw_size, 8,
    bpf_output__printer, core::ptr::null_mut(), trace.output);
    ++trace.nr_events_printed;
    }
#[no_mangle]
unsafe extern "C" fn bitmap_byte(mask: *const c_ulong, byte_idx: c_int) -> c_uchar {
    static unsigned char bitmap_byte(const unsigned long *mask, int byte_idx)
    {
    let mut b_val: c_uchar = 0;
    int bit_in_byte;
    for (bit_in_byte = 0; bit_in_byte < 8; bit_in_byte++) {
    let mut b_idx: c_int = byte_idx * 8 + bit_in_byte;
    let mut host_w_idx: c_int = b_idx / BITS_PER_LONG;
    let mut host_bit_in_word: c_int = b_idx % BITS_PER_LONG;
    if (mask[host_w_idx] & (1UL << host_bit_in_word))
    b_val |= (1 << bit_in_byte);
    }
    return b_val;
    }
#[no_mangle]
unsafe extern "C" fn trace__field_is_ip(name: *const c_char) -> bool {
    static bool trace__field_is_ip(const char *name)
    {
    return !strcmp(name, "__probe_ip") ||
    !strcmp(name, "caller_ip") ||
    !strcmp(name, "call_site");
    }
    static size_t trace__fprintf_tp_fields(struct trace *trace, struct perf_sample *sample,
    struct thread *thread, void *augmented_args, int augmented_args_size)
    {
    struct evsel *evsel = sample.evsel;
    char bf[2048];
    let mut size: usize = sizeof(bf);
    const struct tep_event *tp_format = evsel__tp_format(evsel);
    struct tep_format_field *field = tp_format ? tp_format.format.fields : core::ptr::null_mut();
    struct syscall_arg_fmt *arg = __evsel__syscall_arg_fmt(evsel);
    let mut printed: usize = 0, btf_printed;
    unsigned long val;
    let mut bit: u8 = 1;
    bool is_probe_ip;
    struct syscall_arg syscall_arg = {
    .augmented = {
    .size = augmented_args_size,
    .args = augmented_args,
    },
    .idx	= 0,
    .mask	= 0,
    .trace  = trace,
    .thread = thread,
    .show_string_prefix = trace.show_string_prefix,
    };
    for (; field && arg; field = field.next, ++syscall_arg.idx, bit <<= 1, ++arg) {
    if (syscall_arg.mask & bit)
    continue;
    syscall_arg.len = 0;
    syscall_arg.fmt = arg;
    if (field.flags & TEP_FIELD_IS_ARRAY) {
    void *ptr = format_field__get_raw_data(field, sample,
    evsel.needs_swap,
    &syscall_arg.len);
    if (!ptr) {
    pr_err("Problem processing %s field, skipping...\n", field.name);
    continue;
    }
    val = (uintptr_t)ptr;
    } else if ((field.flags & TEP_FIELD_IS_DYNAMIC) &&
    strstr(field.type, "cpumask")) {
    unsigned long *mask = format_field__get_cpumask(field, sample,
    evsel.needs_swap,
    &syscall_arg.len);
    if (!mask) {
    pr_err("Problem processing %s field, skipping...\n", field.name);
    continue;
    }
    printed += scnprintf(bf + printed, size - printed, "%s", printed ? ", " : "");
    if (trace.show_arg_names)
    printed += scnprintf(bf + printed, size - printed, "%s: ", field.name);
    if (syscall_arg.len == 0) {
    printed += scnprintf(bf + printed, size - printed, "0");
    } else if (trace.bitmask_list) {
    printed += bitmap_scnprintf(mask, syscall_arg.len * 8,
    bf + printed, size - printed);
    } else {
    int i;
    let mut skip_zero: bool = true;
    printed += scnprintf(bf + printed, size - printed, "0x");
// Print bytes from most significant to least significant
    for (i = syscall_arg.len - 1; i >= 0; i--) {
    let mut b_val: c_uchar = bitmap_byte(mask, i);
    if (skip_zero && b_val == 0 && i > 0)
    continue;
    if (skip_zero) {
    printed += scnprintf(bf + printed, size - printed, "%x", b_val);
    skip_zero = false;
    } else {
    printed += scnprintf(bf + printed, size - printed, "%02x", b_val);
    }
    }
    }
    free(mask);
    continue;
    } else
    val = format_field__intval(field, sample, evsel.needs_swap);
//
// Some syscall args need some mask, most don't and
// return val untouched.
//
    val = syscall_arg_fmt__mask_val(arg, &syscall_arg, val);
// Suppress this argument if its value is zero and show_zero property isn't set.
    if (val == 0 && !trace.show_zeros && !arg.show_zero && arg.strtoul != STUL_BTF_TYPE)
    continue;
//
// __probe_ip is implicitly added to bare dynamic probes.
// Suppress it by default to avoid cluttering the output.
// If verbose mode is enabled, ensure it is formatted as a
// hexadecimal memory address rather than a signed integer.
//
// caller_ip and call_site are also expected to be instruction
// pointers and should always be represented in hexadecimal.
//
    is_probe_ip = evsel__is_probe(evsel) && !strcmp(field.name, "__probe_ip");
    if (is_probe_ip || trace__field_is_ip(field.name)) {
    if (is_probe_ip && !verbose)
    continue;
    printed += scnprintf(bf + printed, size - printed,
    "%s", printed ? ", " : "");
    if (trace.show_arg_names)
    printed += scnprintf(bf + printed, size - printed,
    "%s: ", field.name);
    printed += scnprintf(bf + printed, size - printed, "%#016llx",
    (unsigned long long)val);
    continue;
    }
    printed += scnprintf(bf + printed, size - printed, "%s", printed ? ", " : "");
    if (trace.show_arg_names)
    printed += scnprintf(bf + printed, size - printed, "%s: ", field.name);
    btf_printed = trace__btf_scnprintf(trace, &syscall_arg, bf + printed, size - printed, val, field.type);
    if (btf_printed) {
    printed += btf_printed;
    continue;
    }
    printed += syscall_arg_fmt__scnprintf_val(arg, bf + printed, size - printed, &syscall_arg, val);
    }
    return fprintf(trace.output, "%.*s", (int)printed, bf);
    }
    static int trace__event_handler(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    struct evsel *evsel = sample.evsel;
    struct thread *thread;
    let mut callchain_ret: c_int = 0;
    if (evsel.nr_events_printed >= evsel.max_events)
    return 0;
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    if (sample.callchain) {
    struct callchain_cursor *cursor = get_tls_callchain_cursor();
    callchain_ret = trace__resolve_callchain(trace, sample, cursor);
    if (callchain_ret == 0) {
    if (cursor.nr < trace.min_stack)
    goto out;
    callchain_ret = 1;
    }
    }
    trace__printf_interrupted_entry(trace);
    trace__fprintf_tstamp(trace, sample.time, trace.output);
    if (trace.show_cpu)
    trace__fprintf_cpu(sample.cpu, trace.output);
    if (trace.trace_syscalls && trace.show_duration)
    fprintf(trace.output, "(         ): ");
    if (thread)
    trace__fprintf_comm_tid(trace, thread, trace.output);
    if (evsel == trace.syscalls.events.bpf_output) {
    let mut id: c_int = perf_evsel__sc_tp_uint(id, sample);
    int e_machine = thread
    ? thread__e_machine(thread, trace.host, /*e_flags=*/core::ptr::null_mut())
    : EM_HOST;
    struct syscall *sc = trace__syscall_info(trace, evsel, e_machine, id);
    if (sc) {
    fprintf(trace.output, "%s(", sc.name);
    trace__fprintf_sys_enter(trace, sample);
    fputc(')', trace.output);
    goto newline;
    }
//
// XXX: Not having the associated syscall info or not finding/adding
// the thread should never happen, but if it does...
// fall thru and print it as a bpf_output event.
//
    }
    fprintf(trace.output, "%s(", evsel.name);
    if (evsel__is_bpf_output(evsel)) {
    bpf_output__fprintf(trace, sample);
    } else {
    const struct tep_event *tp_format = evsel__tp_format(evsel);
    if (tp_format && (strncmp(tp_format.name, "sys_enter_", 10) ||
    trace__fprintf_sys_enter(trace, sample))) {
    if (trace.libtraceevent_print) {
    event_format__fprintf(tp_format, sample.cpu,
    sample.raw_data, sample.raw_size,
    trace.output);
    } else {
    trace__fprintf_tp_fields(trace, sample, thread, core::ptr::null_mut(), 0);
    }
    }
    }
    newline:
    fprintf(trace.output, ")\n");
    if (callchain_ret > 0)
    trace__fprintf_callchain(trace, sample);
#[no_mangle]
pub unsafe extern "C" fn if(0: callchain_ret <) -> else {
    else if (callchain_ret < 0)
    pr_err("Problem processing %s callchain, skipping...\n", evsel__name(evsel));
    ++trace.nr_events_printed;
    if (evsel.max_events != ULONG_MAX && ++evsel.nr_events_printed == evsel.max_events) {
    evsel__disable(evsel);
    evsel__close(evsel);
    }
    out:
    thread__put(thread);
    return 0;
    }
    static void print_location(FILE *f, struct perf_sample *sample,
    struct addr_location *al,
    bool print_dso, bool print_sym)
    {
    if ((verbose > 0 || print_dso) && al.map)
    fprintf(f, "%s@", dso__long_name(map__dso(al.map)));
    if ((verbose > 0 || print_sym) && al.sym)
    fprintf(f, "%s+0x%" PRIx64, al.sym.name,
    al.addr - al.sym.start);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: al->map) -> else {
    else if (al.map)
    fprintf(f, "0x%" PRIx64, al.addr);
    else
    fprintf(f, "0x%" PRIx64, sample.addr);
    }
    static int trace__pgfault(struct trace *trace,
    union perf_event *event __maybe_unused,
    struct perf_sample *sample)
    {
    struct thread *thread;
    struct addr_location al;
    let mut map_type: c_char = 'd';
    struct thread_trace *ttrace;
    let mut err: c_int = -1;
    let mut callchain_ret: c_int = 0;
    addr_location__init(&al);
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    if (sample.callchain) {
    struct callchain_cursor *cursor = get_tls_callchain_cursor();
    callchain_ret = trace__resolve_callchain(trace, sample, cursor);
    if (callchain_ret == 0) {
    if (cursor.nr < trace.min_stack)
    goto out_put;
    callchain_ret = 1;
    }
    }
    ttrace = thread__trace(thread, trace);
    if (ttrace == core::ptr::null_mut())
    goto out_put;
    if (sample.evsel.core.attr.config == PERF_COUNT_SW_PAGE_FAULTS_MAJ) {
    ttrace.pfmaj++;
    trace.pfmaj++;
    } else {
    ttrace.pfmin++;
    trace.pfmin++;
    }
    if (trace.summary_only)
    goto out;
    thread__find_symbol(thread, sample.cpumode, sample.ip, &al);
    trace__fprintf_entry_head(trace, thread, 0, true, sample.time,
    sample.cpu, trace.output);
    fprintf(trace.output, "%sfault [",
    sample.evsel.core.attr.config == PERF_COUNT_SW_PAGE_FAULTS_MAJ ?
    "maj" : "min");
    print_location(trace.output, sample, &al, false, true);
    fprintf(trace.output, "] => ");
    thread__find_symbol(thread, sample.cpumode, sample.addr, &al);
    if (!al.map) {
    thread__find_symbol(thread, sample.cpumode, sample.addr, &al);
    if (al.map)
    map_type = 'x';
    else
    map_type = '?';
    }
    print_location(trace.output, sample, &al, true, false);
    fprintf(trace.output, " (%c%c)\n", map_type, al.level);
    if (callchain_ret > 0)
    trace__fprintf_callchain(trace, sample);
#[no_mangle]
pub unsafe extern "C" fn if(0: callchain_ret <) -> else {
    else if (callchain_ret < 0)
    pr_err("Problem processing %s callchain, skipping...\n",
    evsel__name(sample.evsel));
    ++trace.nr_events_printed;
    out:
    err = 0;
    out_put:
    thread__put(thread);
    addr_location__exit(&al);
    return err;
    }
    static void trace__set_base_time(struct trace *trace,
    struct perf_sample *sample)
    {
//
// BPF events were not setting PERF_SAMPLE_TIME, so be more robust
// and don't use sample->time unconditionally, we may end up having
// some other event in the future without PERF_SAMPLE_TIME for good
// reason, i.e. we may not be interested in its timestamps, just in
// it taking place, picking some piece of information when it
// appears in our event stream (vfs_getname comes to mind).
//
    if (trace.base_time == 0 && !trace.full_time &&
    (sample.evsel.core.attr.sample_type & PERF_SAMPLE_TIME))
    trace.base_time = sample.time;
    }
    static int trace__process_sample(const struct perf_tool *tool,
    union perf_event *event,
    struct perf_sample *sample,
    struct machine *machine __maybe_unused)
    {
    struct trace *trace = container_of(tool, struct trace, tool);
    struct evsel *evsel = sample.evsel;
    struct thread *thread;
    let mut err: c_int = 0;
    let mut handler: tracepoint_handler = evsel.handler;
    thread = machine__findnew_thread(trace.host, sample.pid, sample.tid);
    if (thread && thread__is_filtered(thread))
    goto out;
    trace__set_base_time(trace, sample);
    if (handler) {
    ++trace.nr_events;
    handler(trace, event, sample);
    }
    out:
    thread__put(thread);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__record(trace: *mut trace, argc: c_int, argv: *const c_char) -> c_int {
    static int trace__record(struct trace *trace, int argc, const char **argv)
    {
    unsigned int rec_argc, i, j;
    const char **rec_argv;
    const char * const record_args[] = {
    "record",
    "-R",
    "-m", "1024",
    "-c", "1",
    };
    let mut pid: pid_t = getpid();
    char *filter = asprintf__tp_filter_pids(1, &pid);
    const char * const sc_args[] = { "-e", };
    let mut sc_args_nr: c_uint = ARRAY_SIZE(sc_args);
    const char * const majpf_args[] = { "-e", "major-faults" };
    let mut majpf_args_nr: c_uint = ARRAY_SIZE(majpf_args);
    const char * const minpf_args[] = { "-e", "minor-faults" };
    let mut minpf_args_nr: c_uint = ARRAY_SIZE(minpf_args);
    let mut err: c_int = -1;
// +3 is for the event string below and the pid filter
    rec_argc = ARRAY_SIZE(record_args) + sc_args_nr + 3 +
    majpf_args_nr + minpf_args_nr + argc;
    rec_argv = calloc(rec_argc + 1, sizeof(char *));
    if (rec_argv == core::ptr::null_mut() || filter == core::ptr::null_mut())
    goto out_free;
    j = 0;
    for (i = 0; i < ARRAY_SIZE(record_args); i++)
    rec_argv[j++] = record_args[i];
    if (trace.trace_syscalls) {
    for (i = 0; i < sc_args_nr; i++)
    rec_argv[j++] = sc_args[i];
// event string may be different for older kernels - e.g., RHEL6
    if (is_valid_tracepoint("raw_syscalls:sys_enter"))
    rec_argv[j++] = "raw_syscalls:sys_enter,raw_syscalls:sys_exit";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_valid_tracepoint("syscalls:sys_enter")) -> else {
    else if (is_valid_tracepoint("syscalls:sys_enter"))
    rec_argv[j++] = "syscalls:sys_enter,syscalls:sys_exit";
    else {
    pr_err("Neither raw_syscalls nor syscalls events exist.\n");
    goto out_free;
    }
    }
    rec_argv[j++] = "--filter";
    rec_argv[j++] = filter;
    if (trace.trace_pgfaults & TRACE_PFMAJ)
    for (i = 0; i < majpf_args_nr; i++)
    rec_argv[j++] = majpf_args[i];
    if (trace.trace_pgfaults & TRACE_PFMIN)
    for (i = 0; i < minpf_args_nr; i++)
    rec_argv[j++] = minpf_args[i];
    for (i = 0; i < (unsigned int)argc; i++)
    rec_argv[j++] = argv[i];
    err = cmd_record(j, rec_argv);
    out_free:
    free(filter);
    free(rec_argv);
    return err;
    }
    static size_t trace__fprintf_thread_summary(struct trace *trace, FILE *fp);
    static size_t trace__fprintf_total_summary(struct trace *trace, FILE *fp);
#[no_mangle]
unsafe extern "C" fn evlist__add_vfs_getname(evlist: *mut evlist) -> bool {
    static bool evlist__add_vfs_getname(struct evlist *evlist)
    {
    let mut found: bool = false;
    struct evsel *evsel, *tmp;
    struct parse_events_error err;
    int ret;
    parse_events_error__init(&err);
    ret = parse_events(evlist, "probe:vfs_getname*", &err);
    parse_events_error__exit(&err);
    if (ret)
    return false;
    evlist__for_each_entry_safe(evlist, evsel, tmp) {
    if (!strstarts(evsel__name(evsel), "probe:vfs_getname"))
    continue;
    if (evsel__field(evsel, "pathname")) {
    evsel.handler = trace__vfs_getname;
    found = true;
    continue;
    }
    list_del_init(&evsel.core.node);
    evsel.evlist = core::ptr::null_mut();
    evsel__put(evsel);
    }
    return found;
    }
    static struct evsel *evsel__new_pgfault(u64 config)
    {
    struct evsel *evsel;
    struct perf_event_attr attr = {
    .type = PERF_TYPE_SOFTWARE,
    .mmap_data = 1,
    };
    attr.config = config;
    attr.sample_period = 1;
    event_attr_init(&attr);
    evsel = evsel__new(&attr);
    if (evsel)
    evsel.handler = trace__pgfault;
    return evsel;
    }
#[no_mangle]
unsafe extern "C" fn evlist__free_syscall_tp_fields(evlist: *mut evlist) {
    static void evlist__free_syscall_tp_fields(struct evlist *evlist)
    {
    struct evsel *evsel;
    evlist__for_each_entry(evlist, evsel) {
    evsel_trace__delete(evsel.priv);
    evsel.priv = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn trace__handle_event(trace: *mut trace, event: *mut union perf_event, sample: *mut perf_sample) {
    static void trace__handle_event(struct trace *trace, union perf_event *event, struct perf_sample *sample)
    {
    let mut type: u32 = event.header.type;
    if (type != PERF_RECORD_SAMPLE) {
    trace__process_event(trace, trace.host, event, sample);
    return;
    }
    if (sample.evsel == core::ptr::null_mut()) {
    sample.evsel = evlist__id2evsel(trace.evlist, sample.id);
    if (sample.evsel)
    evsel__get(sample.evsel);
    }
    if (sample.evsel == core::ptr::null_mut()) {
    fprintf(trace.output, "Unknown tp ID %" PRIu64 ", skipping...\n", sample.id);
    return;
    }
    if (evswitch__discard(&trace.evswitch, sample.evsel))
    return;
    trace__set_base_time(trace, sample);
    if (sample.evsel.core.attr.type == PERF_TYPE_TRACEPOINT &&
    sample.raw_data == core::ptr::null_mut()) {
    fprintf(trace.output, "%s sample with no payload for tid: %d, cpu %d, raw_size=%d, skipping...\n",
    evsel__name(sample.evsel), sample.tid,
    sample.cpu, sample.raw_size);
    } else {
    let mut handler: tracepoint_handler = sample.evsel.handler;
    handler(trace, event, sample);
    }
    if (trace.nr_events_printed >= trace.max_events && trace.max_events != ULONG_MAX)
    interrupted = true;
    }
#[no_mangle]
unsafe extern "C" fn trace__add_syscall_newtp(trace: *mut trace) -> c_int {
    static int trace__add_syscall_newtp(struct trace *trace)
    {
    let mut ret: c_int = -1;
    struct evlist *evlist = trace.evlist;
    struct evsel *sys_enter, *sys_exit;
    sys_enter = perf_evsel__raw_syscall_newtp("sys_enter", trace__sys_enter);
    if (sys_enter == core::ptr::null_mut())
    goto out;
    if (perf_evsel__init_sc_tp_ptr_field(sys_enter, args))
    goto out_delete_sys_enter;
    sys_exit = perf_evsel__raw_syscall_newtp("sys_exit", trace__sys_exit);
    if (sys_exit == core::ptr::null_mut())
    goto out_delete_sys_enter;
    if (perf_evsel__init_sc_tp_uint_field(sys_exit, ret))
    goto out_delete_sys_exit;
    evsel__config_callchain(sys_enter, &trace.opts, &callchain_param);
    evsel__config_callchain(sys_exit, &trace.opts, &callchain_param);
    evlist__add(evlist, sys_enter);
    evlist__add(evlist, sys_exit);
    if (callchain_param.enabled && !trace.kernel_syscallchains) {
//
// We're interested only in the user space callchain
// leading to the syscall, allow overriding that for
// debugging reasons using --kernel_syscall_callchains
//
    sys_exit.core.attr.exclude_callchain_kernel = 1;
    }
    trace.syscalls.events.sys_enter = sys_enter;
    trace.syscalls.events.sys_exit  = sys_exit;
    ret = 0;
    out:
    return ret;
    out_delete_sys_exit:
    evsel__put_and_free_priv(sys_exit);
    out_delete_sys_enter:
    evsel__put_and_free_priv(sys_enter);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn trace__set_ev_qualifier_tp_filter(trace: *mut trace) -> c_int {
    static int trace__set_ev_qualifier_tp_filter(struct trace *trace)
    {
    let mut err: c_int = -1;
    struct evsel *sys_exit;
    char *filter = asprintf_expr_inout_ints("id", !trace.not_ev_qualifier,
    trace.ev_qualifier_ids.nr,
    trace.ev_qualifier_ids.entries);
    if (filter == core::ptr::null_mut())
    goto out_enomem;
    if (!evsel__append_tp_filter(trace.syscalls.events.sys_enter, filter)) {
    sys_exit = trace.syscalls.events.sys_exit;
    err = evsel__append_tp_filter(sys_exit, filter);
    }
    free(filter);
    out:
    return err;
    out_enomem:
    errno = ENOMEM;
    goto out;
    }

    static struct bpf_program *unaugmented_prog;
#[no_mangle]
unsafe extern "C" fn syscall_arg_fmt__cache_btf_struct(arg_fmt: *mut syscall_arg_fmt, btf: *mut btf, type: *mut c_char) -> c_int {
    static int syscall_arg_fmt__cache_btf_struct(struct syscall_arg_fmt *arg_fmt, struct btf *btf, char *type)
    {
    int id;
    if (arg_fmt.type != core::ptr::null_mut())
    return -1;
    id = btf__find_by_name(btf, type);
    if (id < 0)
    return -1;
    arg_fmt.type    = btf__type_by_id(btf, id);
    arg_fmt.type_id = id;
    return 0;
    }
    static struct bpf_program *trace__find_syscall_bpf_prog(struct trace *trace __maybe_unused,
    struct syscall *sc,
    const char *prog_name, const char *type)
    {
    struct bpf_program *prog;
    if (prog_name == core::ptr::null_mut()) {
    char default_prog_name[256];
    scnprintf(default_prog_name, sizeof(default_prog_name), "tp/syscalls/sys_%s_%s", type, sc.name);
    prog = augmented_syscalls__find_by_title(default_prog_name);
    if (prog != core::ptr::null_mut())
    goto out_found;
    if (sc.fmt && sc.fmt.alias) {
    scnprintf(default_prog_name, sizeof(default_prog_name), "tp/syscalls/sys_%s_%s", type, sc.fmt.alias);
    prog = augmented_syscalls__find_by_title(default_prog_name);
    if (prog != core::ptr::null_mut())
    goto out_found;
    }
    goto out_unaugmented;
    }
    prog = augmented_syscalls__find_by_title(prog_name);
    if (prog != core::ptr::null_mut()) {
    out_found:
    return prog;
    }
    pr_debug("Couldn't find BPF prog \"%s\" to associate with syscalls:sys_%s_%s, not augmenting it\n",
    prog_name, type, sc.name);
    out_unaugmented:
    return unaugmented_prog;
    }
#[no_mangle]
unsafe extern "C" fn trace__init_syscall_bpf_progs(trace: *mut trace, e_machine: c_int, id: c_int) {
    static void trace__init_syscall_bpf_progs(struct trace *trace, int e_machine, int id)
    {
    struct syscall *sc = trace__syscall_info(trace, core::ptr::null_mut(), e_machine, id);
    if (sc == core::ptr::null_mut())
    return;
    sc.bpf_prog.sys_enter = trace__find_syscall_bpf_prog(trace, sc, sc.fmt ? sc.fmt.bpf_prog_name.sys_enter : core::ptr::null_mut(), "enter");
    sc.bpf_prog.sys_exit  = trace__find_syscall_bpf_prog(trace, sc, sc.fmt ? sc.fmt.bpf_prog_name.sys_exit  : core::ptr::null_mut(),  "exit");
    }
#[no_mangle]
unsafe extern "C" fn trace__bpf_prog_sys_enter_fd(trace: *mut trace, e_machine: c_int, id: c_int) -> c_int {
    static int trace__bpf_prog_sys_enter_fd(struct trace *trace, int e_machine, int id)
    {
    struct syscall *sc = trace__syscall_info(trace, core::ptr::null_mut(), e_machine, id);
    return sc ? bpf_program__fd(sc.bpf_prog.sys_enter) : bpf_program__fd(unaugmented_prog);
    }
#[no_mangle]
unsafe extern "C" fn trace__bpf_prog_sys_exit_fd(trace: *mut trace, e_machine: c_int, id: c_int) -> c_int {
    static int trace__bpf_prog_sys_exit_fd(struct trace *trace, int e_machine, int id)
    {
    struct syscall *sc = trace__syscall_info(trace, core::ptr::null_mut(), e_machine, id);
    return sc ? bpf_program__fd(sc.bpf_prog.sys_exit) : bpf_program__fd(unaugmented_prog);
    }
#[no_mangle]
unsafe extern "C" fn trace__bpf_sys_enter_beauty_map(trace: *mut trace, e_machine: c_int, key: c_int, beauty_array: *mut c_uint) -> c_int {
    static int trace__bpf_sys_enter_beauty_map(struct trace *trace, int e_machine, int key, unsigned int *beauty_array)
    {
    struct tep_format_field *field;
    struct syscall *sc = trace__syscall_info(trace, core::ptr::null_mut(), e_machine, key);
    const struct btf_type *bt;
    char *struct_offset, *tmp, name[32];
    let mut can_augment: bool = false;
    int i, cnt;
    if (sc == core::ptr::null_mut())
    return -1;
    trace__load_vmlinux_btf(trace);
    if (trace.btf == core::ptr::null_mut())
    return -1;
    for (i = 0, field = sc.args; field; ++i, field = field.next) {
// XXX We're only collecting pointer payloads _from_ user space
    if (!sc.arg_fmt[i].from_user)
    continue;
    struct_offset = strstr(field.type, "struct ");
    if (struct_offset == core::ptr::null_mut())
    struct_offset = strstr(field.type, "union ");
    else
    struct_offset++; // "union" is shorter
    if (field.flags & TEP_FIELD_IS_POINTER && struct_offset) { /* struct or union (think BPF's attr arg) */
    struct_offset += 6;
// for 'struct foo *', we only want 'foo'
    for (tmp = struct_offset, cnt = 0; *tmp != ' ' && *tmp != '\0'; ++tmp, ++cnt) {
    }
    strncpy(name, struct_offset, cnt);
    name[cnt] = '\0';
// cache struct's btf_type and type_id
    if (syscall_arg_fmt__cache_btf_struct(&sc.arg_fmt[i], trace.btf, name))
    continue;
    bt = sc.arg_fmt[i].type;
    beauty_array[i] = bt.size;
    can_augment = true;
    } else if (field.flags & TEP_FIELD_IS_POINTER && /* string */
    strcmp(field.type, "const char *") == 0 &&
    (strstr(field.name, "name") ||
    strstr(field.name, "path") ||
    strstr(field.name, "file") ||
    strstr(field.name, "root") ||
    strstr(field.name, "key") ||
    strstr(field.name, "special") ||
    strstr(field.name, "type") ||
    strstr(field.name, "description"))) {
    beauty_array[i] = 1;
    can_augment = true;
    } else if (field.flags & TEP_FIELD_IS_POINTER && /* buffer */
    strstr(field.type, "char *") &&
    (strstr(field.name, "buf") ||
    strstr(field.name, "val") ||
    strstr(field.name, "msg"))) {
    int j;
    struct tep_format_field *field_tmp;
// find the size of the buffer that appears in pairs with buf
    for (j = 0, field_tmp = sc.args; field_tmp; ++j, field_tmp = field_tmp.next) {
    if (!(field_tmp.flags & TEP_FIELD_IS_POINTER) && /* only integers */
    (strstr(field_tmp.name, "count") ||
    strstr(field_tmp.name, "siz") ||  /* size, bufsiz */
    (strstr(field_tmp.name, "len") && strcmp(field_tmp.name, "filename")))) {
// filename's got 'len' in it, we don't want that
    beauty_array[i] = -(j + 1);
    can_augment = true;
    break;
    }
    }
    }
    }
    if (can_augment)
    return 0;
    return -1;
    }
    static struct bpf_program *trace__find_usable_bpf_prog_entry(struct trace *trace,
    struct syscall *sc)
    {
    struct tep_format_field *field, *candidate_field;
//
// We're only interested in syscalls that have a pointer:
//
    for (field = sc.args; field; field = field.next) {
    if (field.flags & TEP_FIELD_IS_POINTER)
    goto try_to_find_pair;
    }
    return core::ptr::null_mut();
    try_to_find_pair:
    for (int i = 0, num_idx = syscalltbl__num_idx(sc.e_machine); i < num_idx; ++i) {
    let mut id: c_int = syscalltbl__id_at_idx(sc.e_machine, i);
    struct syscall *pair = trace__syscall_info(trace, core::ptr::null_mut(), sc.e_machine, id);
    struct bpf_program *pair_prog;
    let mut is_candidate: bool = false;
    if (pair == core::ptr::null_mut() || pair.id == sc.id ||
    pair.bpf_prog.sys_enter == unaugmented_prog)
    continue;
    for (field = sc.args, candidate_field = pair.args;
    field && candidate_field; field = field.next, candidate_field = candidate_field.next) {
    bool is_pointer = field.flags & TEP_FIELD_IS_POINTER,
    candidate_is_pointer = candidate_field.flags & TEP_FIELD_IS_POINTER;
    if (is_pointer) {
    if (!candidate_is_pointer) {
// The candidate just doesn't copies our pointer arg, might copy other pointers we want.
    continue;
    }
    } else {
    if (candidate_is_pointer) {
// The candidate might copy a pointer we don't have, skip it.
    goto next_candidate;
    }
    continue;
    }
    if (strcmp(field.type, candidate_field.type))
    goto next_candidate;
//
// This is limited in the BPF program but sys_write
// uses "const char *" for its "buf" arg so we need to
// use some heuristic that is kinda future proof...
//
    if (strcmp(field.type, "const char *") == 0 &&
    !(strstr(field.name, "name") ||
    strstr(field.name, "path") ||
    strstr(field.name, "file") ||
    strstr(field.name, "root") ||
    strstr(field.name, "description")))
    goto next_candidate;
    is_candidate = true;
    }
    if (!is_candidate)
    goto next_candidate;
//
// Check if the tentative pair syscall augmenter has more pointers, if it has,
// then it may be collecting that and we then can't use it, as it would collect
// more than what is common to the two syscalls.
//
    if (candidate_field) {
    for (candidate_field = candidate_field.next; candidate_field; candidate_field = candidate_field.next)
    if (candidate_field.flags & TEP_FIELD_IS_POINTER)
    goto next_candidate;
    }
    pair_prog = pair.bpf_prog.sys_enter;
//
// If the pair isn't enabled, then its bpf_prog.sys_enter will not
// have been searched for, so search it here and if it returns the
// unaugmented one, then ignore it, otherwise we'll reuse that BPF
// program for a filtered syscall on a non-filtered one.
//
// For instance, we have "!syscalls:sys_enter_renameat" and that is
// useful for "renameat2".
//
    if (pair_prog == core::ptr::null_mut()) {
    pair_prog = trace__find_syscall_bpf_prog(trace, pair, pair.fmt ? pair.fmt.bpf_prog_name.sys_enter : core::ptr::null_mut(), "enter");
    if (pair_prog == unaugmented_prog)
    goto next_candidate;
    }
    pr_debug("Reusing \"%s\" BPF sys_enter augmenter for \"%s\"\n", pair.name,
    sc.name);
    return pair_prog;
    next_candidate:
    continue;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace__init_syscalls_bpf_prog_array_maps(trace: *mut trace, e_machine: c_int) -> c_int {
    static int trace__init_syscalls_bpf_prog_array_maps(struct trace *trace, int e_machine)
    {
    int map_enter_fd;
    int map_exit_fd;
    int beauty_map_fd;
    let mut err: c_int = 0;
    unsigned int beauty_array[6];
    if (augmented_syscalls__get_map_fds(&map_enter_fd, &map_exit_fd, &beauty_map_fd) < 0)
    return -1;
    unaugmented_prog = augmented_syscalls__unaugmented();
    for (int i = 0, num_idx = syscalltbl__num_idx(e_machine); i < num_idx; ++i) {
    int prog_fd, key = syscalltbl__id_at_idx(e_machine, i);
    if (!trace__syscall_enabled(trace, key))
    continue;
    trace__init_syscall_bpf_progs(trace, e_machine, key);
// It'll get at least the "!raw_syscalls:unaugmented"
    prog_fd = trace__bpf_prog_sys_enter_fd(trace, e_machine, key);
    err = bpf_map_update_elem(map_enter_fd, &key, &prog_fd, BPF_ANY);
    if (err)
    break;
    prog_fd = trace__bpf_prog_sys_exit_fd(trace, e_machine, key);
    err = bpf_map_update_elem(map_exit_fd, &key, &prog_fd, BPF_ANY);
    if (err)
    break;
// use beauty_map to tell BPF how many bytes to collect, set beauty_map's value here
    memset(beauty_array, 0, sizeof(beauty_array));
    err = trace__bpf_sys_enter_beauty_map(trace, e_machine, key, (unsigned int *)beauty_array);
    if (err)
    continue;
    err = bpf_map_update_elem(beauty_map_fd, &key, beauty_array, BPF_ANY);
    if (err)
    break;
    }
//
// Now lets do a second pass looking for enabled syscalls without
// an augmenter that have a signature that is a superset of another
// syscall with an augmenter so that we can auto-reuse it.
//
// I.e. if we have an augmenter for the "open" syscall that has
// this signature:
//
// int open(const char *pathname, int flags, mode_t mode);
//
// I.e. that will collect just the first string argument, then we
// can reuse it for the 'creat' syscall, that has this signature:
//
// int creat(const char *pathname, mode_t mode);
//
// and for:
//
// int stat(const char *pathname, struct stat *statbuf);
// int lstat(const char *pathname, struct stat *statbuf);
//
// Because the 'open' augmenter will collect the first arg as a string,
// and leave alone all the other args, which already helps with
// beautifying 'stat' and 'lstat''s pathname arg.
//
// Then, in time, when 'stat' gets an augmenter that collects both
// first and second arg (this one on the raw_syscalls:sys_exit prog
// array tail call, then that one will be used.
//
    for (int i = 0, num_idx = syscalltbl__num_idx(e_machine); i < num_idx; ++i) {
    let mut key: c_int = syscalltbl__id_at_idx(e_machine, i);
    struct syscall *sc = trace__syscall_info(trace, core::ptr::null_mut(), e_machine, key);
    struct bpf_program *pair_prog;
    int prog_fd;
    if (sc == core::ptr::null_mut() || sc.bpf_prog.sys_enter == core::ptr::null_mut())
    continue;
//
// For now we're just reusing the sys_enter prog, and if it
// already has an augmenter, we don't need to find one.
//
    if (sc.bpf_prog.sys_enter != unaugmented_prog)
    continue;
//
// Look at all the other syscalls for one that has a signature
// that is close enough that we can share:
//
    pair_prog = trace__find_usable_bpf_prog_entry(trace, sc);
    if (pair_prog == core::ptr::null_mut())
    continue;
    sc.bpf_prog.sys_enter = pair_prog;
//
// Update the BPF_MAP_TYPE_PROG_SHARED for raw_syscalls:sys_enter
// with the fd for the program we're reusing:
//
    prog_fd = bpf_program__fd(sc.bpf_prog.sys_enter);
    err = bpf_map_update_elem(map_enter_fd, &key, &prog_fd, BPF_ANY);
    if (err)
    break;
    }
    return err;
    }

    static int trace__init_syscalls_bpf_prog_array_maps(struct trace *trace __maybe_unused,
    int e_machine __maybe_unused)
    {
    return -1;
    }

#[no_mangle]
unsafe extern "C" fn trace__set_ev_qualifier_filter(trace: *mut trace) -> c_int {
    static int trace__set_ev_qualifier_filter(struct trace *trace)
    {
    if (trace.syscalls.events.sys_enter)
    return trace__set_ev_qualifier_tp_filter(trace);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace__set_filter_loop_pids(trace: *mut trace) -> c_int {
    static int trace__set_filter_loop_pids(struct trace *trace)
    {
    let mut nr: c_uint = 1, err;
    pid_t pids[32] = {
    getpid(),
    };
    struct thread *thread = machine__find_thread(trace.host, pids[0], pids[0]);
    while (thread && nr < ARRAY_SIZE(pids)) {
    struct thread *parent = machine__find_thread(trace.host,
    thread__ppid(thread),
    thread__ppid(thread));
    if (parent == core::ptr::null_mut())
    break;
    if (!strcmp(thread__comm_str(parent), "sshd") ||
    strstarts(thread__comm_str(parent), "gnome-terminal")) {
    pids[nr++] = thread__tid(parent);
    thread__put(parent);
    break;
    }
    thread__put(thread);
    thread = parent;
    }
    thread__put(thread);
    err = evlist__append_tp_filter_pids(trace.evlist, nr, pids);
    if (!err)
    err = augmented_syscalls__set_filter_pids(nr, pids);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__set_filter_pids(trace: *mut trace) -> c_int {
    static int trace__set_filter_pids(struct trace *trace)
    {
    let mut err: c_int = 0;
//
// Better not use !target__has_task() here because we need to cover the
// case where no threads were specified in the command line, but a
// workload was, and in that case we will fill in the thread_map when
// we fork the workload in evlist__prepare_workload.
//
    if (trace.filter_pids.nr > 0) {
    err = evlist__append_tp_filter_pids(trace.evlist, trace.filter_pids.nr,
    trace.filter_pids.entries);
    if (!err) {
    err = augmented_syscalls__set_filter_pids(trace.filter_pids.nr,
    trace.filter_pids.entries);
    }
    } else if (perf_thread_map__pid(evlist__core(trace.evlist).threads, 0) == -1) {
    err = trace__set_filter_loop_pids(trace);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __trace__deliver_event(trace: *mut trace, event: *mut union perf_event) -> c_int {
    static int __trace__deliver_event(struct trace *trace, union perf_event *event)
    {
    struct evlist *evlist = trace.evlist;
    struct perf_sample sample;
    int err;
    perf_sample__init(&sample, /*all=*/false);
    err = evlist__parse_sample(evlist, event, &sample);
    if (err)
    fprintf(trace.output, "Can't parse sample, err = %d, skipping...\n", err);
    else
    trace__handle_event(trace, event, &sample);
    perf_sample__exit(&sample);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __trace__flush_events(trace: *mut trace) -> c_int {
    static int __trace__flush_events(struct trace *trace)
    {
    let mut first: u64 = ordered_events__first_time(&trace.oe.data);
    let mut flush: u64 = trace.oe.last - NSEC_PER_SEC;
// Is there some thing to flush..
    if (first && first < flush)
    return ordered_events__flush_time(&trace.oe.data, flush);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace__flush_events(trace: *mut trace) -> c_int {
    static int trace__flush_events(struct trace *trace)
    {
    return !trace.sort_events ? 0 : __trace__flush_events(trace);
    }
#[no_mangle]
unsafe extern "C" fn trace__deliver_event(trace: *mut trace, event: *mut union perf_event) -> c_int {
    static int trace__deliver_event(struct trace *trace, union perf_event *event)
    {
    int err;
    if (!trace.sort_events)
    return __trace__deliver_event(trace, event);
    err = evlist__parse_sample_timestamp(trace.evlist, event, &trace.oe.last);
    if (err && err != -1)
    return err;
    err = ordered_events__queue(&trace.oe.data, event, trace.oe.last, 0, core::ptr::null_mut());
    if (err)
    return err;
    return trace__flush_events(trace);
    }
    static int ordered_events__deliver_event(struct ordered_events *oe,
    struct ordered_event *event)
    {
    struct trace *trace = container_of(oe, struct trace, oe.data);
    return __trace__deliver_event(trace, event.event);
    }
    static struct syscall_arg_fmt *evsel__find_syscall_arg_fmt_by_name(struct evsel *evsel, char *arg,
    char **type)
    {
    struct syscall_arg_fmt *fmt = __evsel__syscall_arg_fmt(evsel);
    const struct tep_event *tp_format;
    if (!fmt)
    return core::ptr::null_mut();
    tp_format = evsel__tp_format(evsel);
    if (!tp_format)
    return core::ptr::null_mut();
    for (const struct tep_format_field *field = tp_format.format.fields; field;
    field = field.next, ++fmt) {
    if (strcmp(field.name, arg) == 0) {
// type = field->type;
    return fmt;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace__expand_filter(trace: *mut trace, evsel: *mut evsel) -> c_int {
    static int trace__expand_filter(struct trace *trace, struct evsel *evsel)
    {
    char *tok, *left = evsel.filter, *new_filter = evsel.filter;
    while ((tok = strpbrk(left, "=<>!")) != core::ptr::null_mut()) {
    char *right = tok + 1, *right_end;
    if (*right == '=')
    ++right;
    while (isspace(*right))
    ++right;
    if (*right == '\0')
    break;
    while (!isalpha(*left))
    if (++left == tok) {
//
// Bail out, can't find the name of the argument that is being
// used in the filter, let it try to set this filter, will fail later.
//
    return 0;
    }
    right_end = right + 1;
    while (isalnum(*right_end) || *right_end == '_' || *right_end == '|')
    ++right_end;
    if (isalpha(*right)) {
    struct syscall_arg_fmt *fmt;
    int left_size = tok - left,
    right_size = right_end - right;
    char arg[128], *type;
    while (isspace(left[left_size - 1]))
    --left_size;
    scnprintf(arg, sizeof(arg), "%.*s", left_size, left);
    fmt = evsel__find_syscall_arg_fmt_by_name(evsel, arg, &type);
    if (fmt == core::ptr::null_mut()) {
    pr_err("\"%s\" not found in \"%s\", can't set filter \"%s\"\n",
    arg, evsel.name, evsel.filter);
    return -1;
    }
    pr_debug2("trying to expand \"%s\" \"%.*s\" \"%.*s\" . ",
    arg, (int)(right - tok), tok, right_size, right);
    if (fmt.strtoul) {
    u64 val;
    struct syscall_arg syscall_arg = {
    .trace = trace,
    .fmt   = fmt,
    .type_name = type,
    .parm = fmt.parm,
    };
    if (fmt.strtoul(right, right_size, &syscall_arg, &val)) {
    char *n, expansion[19];
    let mut expansion_lenght: c_int = scnprintf(expansion, sizeof(expansion), "%#" PRIx64, val);
    let mut expansion_offset: c_int = right - new_filter;
    pr_debug("%s", expansion);
    if (asprintf(&n, "%.*s%s%s", expansion_offset, new_filter, expansion, right_end) < 0) {
    pr_debug(" out of memory!\n");
    free(new_filter);
    return -1;
    }
    if (new_filter != evsel.filter)
    free(new_filter);
    left = n + expansion_offset + expansion_lenght;
    new_filter = n;
    } else {
    pr_err("\"%.*s\" not found for \"%s\" in \"%s\", can't set filter \"%s\"\n",
    right_size, right, arg, evsel.name, evsel.filter);
    return -1;
    }
    } else {
    pr_err("No resolver (strtoul) for \"%s\" in \"%s\", can't set filter \"%s\"\n",
    arg, evsel.name, evsel.filter);
    return -1;
    }
    pr_debug("\n");
    } else {
    left = right_end;
    }
    }
    if (new_filter != evsel.filter) {
    pr_debug("New filter for %s: %s\n", evsel.name, new_filter);
    evsel__set_filter(evsel, new_filter);
    free(new_filter);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace__expand_filters(trace: *mut trace, err_evsel: *mut evsel) -> c_int {
    static int trace__expand_filters(struct trace *trace, struct evsel **err_evsel)
    {
    struct evlist *evlist = trace.evlist;
    struct evsel *evsel;
    evlist__for_each_entry(evlist, evsel) {
    if (evsel.filter == core::ptr::null_mut())
    continue;
    if (trace__expand_filter(trace, evsel)) {
// err_evsel = evsel;
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace__run(trace: *mut trace, argc: c_int, argv: *const c_char) -> c_int {
    static int trace__run(struct trace *trace, int argc, const char **argv)
    {
    struct evlist *evlist = trace.evlist;
    struct evsel *evsel, *pgfault_maj = core::ptr::null_mut(), *pgfault_min = core::ptr::null_mut();
    let mut err: c_int = -1, i;
    unsigned long before;
    let mut forks: bool = argc > 0;
    let mut draining: bool = false;
    trace.live = true;
    if (trace.summary_bpf) {
    if (trace_prepare_bpf_summary(trace.summary_mode) < 0)
    goto out_put_evlist;
    if (trace.summary_only)
    goto create_maps;
    }
    if (!trace.raw_augmented_syscalls) {
    if (trace.trace_syscalls && trace__add_syscall_newtp(trace))
    goto out_error_raw_syscalls;
    if (trace.trace_syscalls)
    trace.vfs_getname = evlist__add_vfs_getname(evlist);
    }
    if ((trace.trace_pgfaults & TRACE_PFMAJ)) {
    pgfault_maj = evsel__new_pgfault(PERF_COUNT_SW_PAGE_FAULTS_MAJ);
    if (pgfault_maj == core::ptr::null_mut())
    goto out_error_mem;
    evsel__config_callchain(pgfault_maj, &trace.opts, &callchain_param);
    evlist__add(evlist, pgfault_maj);
    }
    if ((trace.trace_pgfaults & TRACE_PFMIN)) {
    pgfault_min = evsel__new_pgfault(PERF_COUNT_SW_PAGE_FAULTS_MIN);
    if (pgfault_min == core::ptr::null_mut())
    goto out_error_mem;
    evsel__config_callchain(pgfault_min, &trace.opts, &callchain_param);
    evlist__add(evlist, pgfault_min);
    }
// Enable ignoring missing threads when -p option is defined.
    trace.opts.ignore_missing_thread = trace.opts.target.pid;
    if (trace.sched &&
    evlist__add_newtp(evlist, "sched", "sched_stat_runtime", trace__sched_stat_runtime))
    goto out_error_sched_stat_runtime;
//
// If a global cgroup was set, apply it to all the events without an
// explicit cgroup. I.e.:
//
// trace -G A -e sched:*switch
//
// Will set all raw_syscalls:sys_{enter,exit}, pgfault, vfs_getname, etc
// _and_ sched:sched_switch to the 'A' cgroup, while:
//
// trace -e sched:*switch -G A
//
// will only set the sched:sched_switch event to the 'A' cgroup, all the
// other events (raw_syscalls:sys_{enter,exit}, etc are left "without"
// a cgroup (on the root cgroup, sys wide, etc).
//
// Multiple cgroups:
//
// trace -G A -e sched:*switch -G B
//
// the syscall ones go to the 'A' cgroup, the sched:sched_switch goes
// to the 'B' cgroup.
//
// evlist__set_default_cgroup() grabs a reference of the passed cgroup
// only for the evsels still without a cgroup, i.e. evsel->cgroup == NULL.
//
    if (trace.cgroup)
    evlist__set_default_cgroup(trace.evlist, trace.cgroup);
    create_maps:
    err = evlist__create_maps(evlist, &trace.opts.target);
    if (err < 0) {
    fprintf(trace.output, "Problems parsing the target to trace, check your options!\n");
    goto out_put_evlist;
    }
    err = trace__symbols_init(trace, argc, argv, evlist);
    if (err < 0) {
    fprintf(trace.output, "Problems initializing symbol libraries!\n");
    goto out_put_evlist;
    }
    if (trace.summary_mode == SUMMARY__BY_TOTAL && !trace.summary_bpf) {
    trace.syscall_stats = alloc_syscall_stats();
    if (!trace.syscall_stats)
    goto out_put_evlist;
    }
    evlist__config(evlist, &trace.opts, &callchain_param);
    if (forks) {
    err = evlist__prepare_workload(evlist, &trace.opts.target, argv, false, core::ptr::null_mut());
    if (err < 0) {
    fprintf(trace.output, "Couldn't run the workload!\n");
    goto out_put_evlist;
    }
    workload_pid = evlist__workload_pid(evlist);
    }
    err = evlist__open(evlist);
    if (err < 0)
    goto out_error_open;
    augmented_syscalls__setup_bpf_output();
    err = trace__set_filter_pids(trace);
    if (err < 0)
    goto out_error_mem;
//
// TODO: Initialize for all host binary machine types, not just
// those matching the perf binary.
//
    trace__init_syscalls_bpf_prog_array_maps(trace, EM_HOST);
    if (trace.ev_qualifier_ids.nr > 0) {
    err = trace__set_ev_qualifier_filter(trace);
    if (err < 0)
    goto out_errno;
    if (trace.syscalls.events.sys_exit) {
    pr_debug("event qualifier tracepoint filter: %s\n",
    trace.syscalls.events.sys_exit.filter);
    }
    }
//
// If the "close" syscall is not traced, then we will not have the
// opportunity to, in syscall_arg__scnprintf_close_fd() invalidate the
// fd->pathname table and were ending up showing the last value set by
// syscalls opening a pathname and associating it with a descriptor or
// reading it from /proc/pid/fd/ in cases where that doesn't make
// sense.
//
// So just disable this beautifier (SCA_FD, SCA_FDAT) when 'close' is
// not in use.
//
// TODO: support for more than just perf binary machine type close.
    trace.fd_path_disabled = !trace__syscall_enabled(trace, syscalltbl__id(EM_HOST, "close"));
    err = trace__expand_filters(trace, &evsel);
    if (err)
    goto out_put_evlist;
    err = evlist__apply_filters(evlist, &evsel, &trace.opts.target);
    if (err < 0)
    goto out_error_apply_filters;
    if (!trace.summary_only || !trace.summary_bpf) {
    err = evlist__do_mmap(evlist, trace.opts.mmap_pages);
    if (err < 0)
    goto out_error_mmap;
    }
    if (!target__none(&trace.opts.target) && !trace.opts.target.initial_delay)
    evlist__enable(evlist);
    if (forks)
    evlist__start_workload(evlist);
    if (trace.opts.target.initial_delay) {
    usleep(trace.opts.target.initial_delay * 1000);
    evlist__enable(evlist);
    }
    if (trace.summary_bpf)
    trace_start_bpf_summary();
    trace.multiple_threads = perf_thread_map__pid(evlist__core(evlist).threads, 0) == -1 ||
    perf_thread_map__nr(evlist__core(evlist).threads) > 1 ||
    evlist__first(evlist).core.attr.inherit;
//
// Now that we already used evsel->core.attr to ask the kernel to setup the
// events, lets reuse evsel->core.attr.sample_max_stack as the limit in
// trace__resolve_callchain(), allowing per-event max-stack settings
// to override an explicitly set --max-stack global setting.
//
    evlist__for_each_entry(evlist, evsel) {
    if (evsel__has_callchain(evsel) &&
    evsel.core.attr.sample_max_stack == 0)
    evsel.core.attr.sample_max_stack = trace.max_stack;
    }
    again:
    before = trace.nr_events;
    for (i = 0; i < evlist__core(evlist).nr_mmaps; i++) {
    union perf_event *event;
    struct mmap *md;
    md = &evlist__mmap(evlist)[i];
    if (perf_mmap__read_init(&md.core) < 0)
    continue;
    while ((event = perf_mmap__read_event(&md.core)) != core::ptr::null_mut()) {
    ++trace.nr_events;
    err = trace__deliver_event(trace, event);
    if (err)
    goto out_disable;
    perf_mmap__consume(&md.core);
    if (interrupted)
    goto out_disable;
    if (done && !draining) {
    evlist__disable(evlist);
    draining = true;
    }
    }
    perf_mmap__read_done(&md.core);
    }
    if (trace.nr_events == before) {
    let mut timeout: c_int = done ? 100 : -1;
    if (!draining && evlist__poll(evlist, timeout) > 0) {
    if (evlist__filter_pollfd(evlist, POLLERR | POLLHUP | POLLNVAL) == 0)
    draining = true;
    goto again;
    } else {
    if (trace__flush_events(trace))
    goto out_disable;
    }
    } else {
    goto again;
    }
    out_disable:
    thread__zput(trace.current);
    evlist__disable(evlist);
    if (trace.summary_bpf)
    trace_end_bpf_summary();
    if (trace.sort_events)
    ordered_events__flush(&trace.oe.data, OE_FLUSH__FINAL);
    if (!err) {
    if (trace.summary) {
    if (trace.summary_bpf)
    trace_print_bpf_summary(trace.output, trace.max_summary);
#[no_mangle]
pub unsafe extern "C" fn if(SUMMARY__BY_TOTAL: trace->summary_mode ==) -> else {
    else if (trace.summary_mode == SUMMARY__BY_TOTAL)
    trace__fprintf_total_summary(trace, trace.output);
    else
    trace__fprintf_thread_summary(trace, trace.output);
    }
    if (trace.show_tool_stats) {
    fprintf(trace.output, "Stats:\n "
    " vfs_getname : %" PRIu64 "\n"
    " proc_getname: %" PRIu64 "\n",
    trace.stats.vfs_getname,
    trace.stats.proc_getname);
    }
    }
    out_put_evlist:
    trace_cleanup_bpf_summary();
    delete_syscall_stats(trace.syscall_stats);
    trace__symbols__exit(trace);
    evlist__free_syscall_tp_fields(evlist);
    evlist__put(evlist);
    cgroup__put(trace.cgroup);
    trace.evlist = core::ptr::null_mut();
    trace.live = false;
    return err;
    {
    char errbuf[BUFSIZ];
    out_error_sched_stat_runtime:
    tracing_path__strerror_open_tp(errno, errbuf, sizeof(errbuf), "sched", "sched_stat_runtime");
    goto out_error;
    out_error_raw_syscalls:
    tracing_path__strerror_open_tp(errno, errbuf, sizeof(errbuf), "raw_syscalls", "sys_(enter|exit)");
    goto out_error;
    out_error_mmap:
    evlist__strerror_mmap(evlist, errno, errbuf, sizeof(errbuf));
    goto out_error;
    out_error_open:
    evlist__strerror_open(evlist, errno, errbuf, sizeof(errbuf));
    out_error:
    fprintf(trace.output, "%s\n", errbuf);
    goto out_put_evlist;
    out_error_apply_filters:
    fprintf(trace.output,
    "Failed to set filter \"%s\" on event %s: %m\n",
    evsel.filter, evsel__name(evsel));
    goto out_put_evlist;
    }
    out_error_mem:
    fprintf(trace.output, "Not enough memory to run!\n");
    goto out_put_evlist;
    out_errno:
    fprintf(trace.output, "%m\n");
    goto out_put_evlist;
    }
#[no_mangle]
unsafe extern "C" fn trace__replay(trace: *mut trace) -> c_int {
    static int trace__replay(struct trace *trace)
    {
    const struct evsel_str_handler handlers[] = {
    { "probe:vfs_getname",	     trace__vfs_getname, },
    };
    struct perf_data data = {
    .path  = input_name,
    .mode  = PERF_DATA_MODE_READ,
    .force = trace.force,
    };
    struct perf_session *session;
    struct evsel *evsel;
    let mut err: c_int = -1;
    perf_tool__init(&trace.tool, /*ordered_events=*/true);
    trace.tool.sample	  = trace__process_sample;
    trace.tool.mmap	  = perf_event__process_mmap;
    trace.tool.mmap2	  = perf_event__process_mmap2;
    trace.tool.comm	  = perf_event__process_comm;
    trace.tool.exit	  = perf_event__process_exit;
    trace.tool.fork	  = perf_event__process_fork;
    trace.tool.attr	  = perf_event__process_attr;
    trace.tool.tracing_data  = perf_event__process_tracing_data;
    trace.tool.build_id	  = perf_event__process_build_id;
    trace.tool.namespaces	  = perf_event__process_namespaces;
    trace.tool.ordered_events = true;
    trace.tool.ordering_requires_timestamps = true;
// add tid to output
    trace.multiple_threads = true;
    session = perf_session__new(&data, &trace.tool);
    if (IS_ERR(session))
    return PTR_ERR(session);
    if (trace.opts.target.pid)
    symbol_conf.pid_list_str = strdup(trace.opts.target.pid);
    if (trace.opts.target.tid)
    symbol_conf.tid_list_str = strdup(trace.opts.target.tid);
    if (symbol__init(perf_session__env(session)) < 0)
    goto out;
    trace.host = &session.machines.host;
    err = perf_session__set_tracepoints_handlers(session, handlers);
    if (err)
    goto out;
    evsel = evlist__find_tracepoint_by_name(session.evlist, "raw_syscalls:sys_enter");
    trace.syscalls.events.sys_enter = evsel;
// older kernels have syscalls tp versus raw_syscalls
    if (evsel == core::ptr::null_mut())
    evsel = evlist__find_tracepoint_by_name(session.evlist, "syscalls:sys_enter");
    if (evsel &&
    (evsel__init_raw_syscall_tp(evsel, trace__sys_enter) < 0 ||
    perf_evsel__init_sc_tp_ptr_field(evsel, args))) {
    pr_err("Error during initialize raw_syscalls:sys_enter event\n");
    goto out;
    }
    evsel = evlist__find_tracepoint_by_name(session.evlist, "raw_syscalls:sys_exit");
    trace.syscalls.events.sys_exit = evsel;
    if (evsel == core::ptr::null_mut())
    evsel = evlist__find_tracepoint_by_name(session.evlist, "syscalls:sys_exit");
    if (evsel &&
    (evsel__init_raw_syscall_tp(evsel, trace__sys_exit) < 0 ||
    perf_evsel__init_sc_tp_uint_field(evsel, ret))) {
    pr_err("Error during initialize raw_syscalls:sys_exit event\n");
    goto out;
    }
    evlist__for_each_entry(session.evlist, evsel) {
    if (evsel.core.attr.type == PERF_TYPE_SOFTWARE &&
    (evsel.core.attr.config == PERF_COUNT_SW_PAGE_FAULTS_MAJ ||
    evsel.core.attr.config == PERF_COUNT_SW_PAGE_FAULTS_MIN ||
    evsel.core.attr.config == PERF_COUNT_SW_PAGE_FAULTS))
    evsel.handler = trace__pgfault;
    }
    if (trace.summary_mode == SUMMARY__BY_TOTAL) {
    trace.syscall_stats = alloc_syscall_stats();
    if (!trace.syscall_stats)
    goto out;
    }
    setup_pager();
    err = perf_session__process_events(session);
    if (err)
    pr_err("Failed to process events, error %d", err);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: trace->summary) -> else {
    else if (trace.summary)
    trace__fprintf_thread_summary(trace, trace.output);
    out:
    delete_syscall_stats(trace.syscall_stats);
    perf_session__delete(session);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_summary_header(fp: *mut FILE) -> usize {
    static size_t trace__fprintf_summary_header(FILE *fp)
    {
    size_t printed;
    printed  = fprintf(fp, "\n Summary of events:\n\n");
    return printed;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_entry {
    pub stats: *mut syscall_stats,
    pub msecs: double,
    pub syscall: c_int,
}

#[no_mangle]
unsafe extern "C" fn entry_cmp(e1: *const c_void, e2: *const c_void) -> c_int {
    static int entry_cmp(const void *e1, const void *e2)
    {
    const struct syscall_entry *entry1 = e1;
    const struct syscall_entry *entry2 = e2;
    return entry1.msecs > entry2.msecs ? -1 : 1;
    }
    static struct syscall_entry *syscall__sort_stats(struct hashmap *syscall_stats)
    {
    struct syscall_entry *entry;
    struct hashmap_entry *pos;
    unsigned bkt, i, nr;
    nr = syscall_stats.sz;
    entry = malloc(nr * sizeof(*entry));
    if (entry == core::ptr::null_mut())
    return core::ptr::null_mut();
    i = 0;
    hashmap__for_each_entry(syscall_stats, pos, bkt) {
    struct syscall_stats *ss = pos.pvalue;
    struct stats *st = &ss.stats;
    entry[i].stats = ss;
    entry[i].msecs = (u64)st.n * (avg_stats(st) / NSEC_PER_MSEC);
    entry[i].syscall = pos.key;
    i++;
    }
    assert(i == nr);
    qsort(entry, nr, sizeof(*entry), entry_cmp);
    return entry;
    }
    static size_t syscall__dump_stats(struct trace *trace, int e_machine, FILE *fp,
    struct hashmap *syscall_stats)
    {
    let mut printed: usize = 0;
    let mut lines: c_int = 0;
    struct syscall *sc;
    struct syscall_entry *entries;
    entries = syscall__sort_stats(syscall_stats);
    if (entries == core::ptr::null_mut())
    return 0;
    printed += fprintf(fp, "\n");
    printed += fprintf(fp, "   syscall            calls  errors  total       min       avg       max       stddev\n");
    printed += fprintf(fp, "                                     (msec)    (msec)    (msec)    (msec)        (%%)\n");
    printed += fprintf(fp, "   --------------- --------  ------ -------- --------- --------- ---------     ------\n");
    for (size_t i = 0; i < syscall_stats.sz; i++) {
    struct syscall_entry *entry = &entries[i];
    struct syscall_stats *stats = entry.stats;
    if (stats) {
    let mut min: double = (double)(stats.stats.min) / NSEC_PER_MSEC;
    let mut max: double = (double)(stats.stats.max) / NSEC_PER_MSEC;
    let mut avg: double = avg_stats(&stats.stats);
    double pct;
    let mut n: u64 = (u64)stats.stats.n;
    pct = avg ? 100.0 * stddev_stats(&stats.stats) / avg : 0.0;
    avg /= NSEC_PER_MSEC;
    sc = trace__syscall_info(trace, /*evsel=*/core::ptr::null_mut(), e_machine, entry.syscall);
    if (!sc)
    continue;
    printed += fprintf(fp, "   %-15s", sc.name);
    printed += fprintf(fp, " %8" PRIu64 " %6" PRIu64 " %9.3f %9.3f %9.3f",
    n, stats.nr_failures, entry.msecs, min, avg);
    printed += fprintf(fp, " %9.3f %9.2f%%\n", max, pct);
    if (trace.errno_summary && stats.nr_failures) {
    int e;
    for (e = 0; e < stats.max_errno; ++e) {
    if (stats.errnos[e] != 0)
    fprintf(fp, "\t\t\t\t%s: %d\n",
    perf_env__arch_strerrno(e_machine, e + 1),
    stats.errnos[e]);
    }
    }
    lines++;
    }
    if (trace.max_summary && trace.max_summary <= lines)
    break;
    }
    free(entries);
    printed += fprintf(fp, "\n\n");
    return printed;
    }
    static size_t thread__dump_stats(struct thread_trace *ttrace,
    struct trace *trace, int e_machine, FILE *fp)
    {
    return syscall__dump_stats(trace, e_machine, fp, ttrace.syscall_stats);
    }
#[no_mangle]
unsafe extern "C" fn system__dump_stats(trace: *mut trace, e_machine: c_int, fp: *mut FILE) -> usize {
    static size_t system__dump_stats(struct trace *trace, int e_machine, FILE *fp)
    {
    return syscall__dump_stats(trace, e_machine, fp, trace.syscall_stats);
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_thread(fp: *mut FILE, thread: *mut thread, trace: *mut trace) -> usize {
    static size_t trace__fprintf_thread(FILE *fp, struct thread *thread, struct trace *trace)
    {
    let mut printed: usize = 0;
    struct thread_trace *ttrace = thread__priv(thread);
    let mut e_machine: c_int = thread__e_machine(thread, trace.host, /*e_flags=*/core::ptr::null_mut());
    double ratio;
    if (ttrace == core::ptr::null_mut())
    return 0;
    ratio = (double)ttrace.nr_events / trace.nr_events * 100.0;
    printed += fprintf(fp, " %s (%d), ", thread__comm_str(thread), thread__tid(thread));
    printed += fprintf(fp, "%lu events, ", ttrace.nr_events);
    printed += fprintf(fp, "%.1f%%", ratio);
    if (ttrace.pfmaj)
    printed += fprintf(fp, ", %lu majfaults", ttrace.pfmaj);
    if (ttrace.pfmin)
    printed += fprintf(fp, ", %lu minfaults", ttrace.pfmin);
    if (trace.sched)
    printed += fprintf(fp, ", %.3f msec\n", ttrace.runtime_ms);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: fputc('\n', EOF: fp) !=) -> else {
    else if (fputc('\n', fp) != EOF)
    ++printed;
    printed += thread__dump_stats(ttrace, trace, e_machine, fp);
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn thread__nr_events(ttrace: *mut thread_trace) -> c_ulong {
    static unsigned long thread__nr_events(struct thread_trace *ttrace)
    {
    return ttrace ? ttrace.nr_events : 0;
    }
    static int trace_nr_events_cmp(void *priv __maybe_unused,
    const struct list_head *la,
    const struct list_head *lb)
    {
    struct thread_list *a = list_entry(la, struct thread_list, list);
    struct thread_list *b = list_entry(lb, struct thread_list, list);
    let mut a_nr_events: c_ulong = thread__nr_events(thread__priv(a.thread));
    let mut b_nr_events: c_ulong = thread__nr_events(thread__priv(b.thread));
    if (a_nr_events != b_nr_events)
    return a_nr_events < b_nr_events ? -1 : 1;
// Identical number of threads, place smaller tids first.
#[no_mangle]
pub unsafe extern "C" fn thread__tid(thread__tid(b->thread: a->thread) <) -> return {
    return thread__tid(a.thread) < thread__tid(b.thread)
    ? -1
    : (thread__tid(a.thread) > thread__tid(b.thread) ? 1 : 0);
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_thread_summary(trace: *mut trace, fp: *mut FILE) -> usize {
    static size_t trace__fprintf_thread_summary(struct trace *trace, FILE *fp)
    {
    let mut printed: usize = trace__fprintf_summary_header(fp);
    LIST_HEAD(threads);
    if (machine__thread_list(trace.host, &threads) == 0) {
    struct thread_list *pos;
    list_sort(core::ptr::null_mut(), &threads, trace_nr_events_cmp);
    list_for_each_entry(pos, &threads, list)
    printed += trace__fprintf_thread(fp, pos.thread, trace);
    }
    thread_list__delete(&threads);
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn trace__fprintf_total_summary(trace: *mut trace, fp: *mut FILE) -> usize {
    static size_t trace__fprintf_total_summary(struct trace *trace, FILE *fp)
    {
    let mut printed: usize = trace__fprintf_summary_header(fp);
    printed += fprintf(fp, " total, ");
    printed += fprintf(fp, "%lu events", trace.nr_events);
    if (trace.pfmaj)
    printed += fprintf(fp, ", %lu majfaults", trace.pfmaj);
    if (trace.pfmin)
    printed += fprintf(fp, ", %lu minfaults", trace.pfmin);
    if (trace.sched)
    printed += fprintf(fp, ", %.3f msec\n", trace.runtime_ms);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: fputc('\n', EOF: fp) !=) -> else {
    else if (fputc('\n', fp) != EOF)
    ++printed;
// TODO: get all system e_machines.
    printed += system__dump_stats(trace, EM_HOST, fp);
    return printed;
    }
    static int trace__set_duration(const struct option *opt, const char *str,
    int unset __maybe_unused)
    {
    struct trace *trace = opt.value;
    trace.duration_filter = atof(str);
    return 0;
    }
    static int trace__set_filter_pids_from_option(const struct option *opt, const char *str,
    int unset __maybe_unused)
    {
    let mut ret: c_int = -1;
    size_t i;
    struct trace *trace = opt.value;
//
// FIXME: introduce a intarray class, plain parse csv and create a
// { int nr, int entries[] } struct...
//
    struct intlist *list = intlist__new(str);
    if (list == core::ptr::null_mut())
    return -1;
    i = trace.filter_pids.nr = intlist__nr_entries(list) + 1;
    trace.filter_pids.entries = calloc(i, sizeof(pid_t));
    if (trace.filter_pids.entries == core::ptr::null_mut())
    goto out;
    trace.filter_pids.entries[0] = getpid();
    for (i = 1; i < trace.filter_pids.nr; ++i)
    trace.filter_pids.entries[i] = intlist__entry(list, i - 1).i;
    intlist__delete(list);
    ret = 0;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace__open_output(trace: *mut trace, filename: *const c_char) -> c_int {
    static int trace__open_output(struct trace *trace, const char *filename)
    {
    struct stat st;
    if (!stat(filename, &st) && st.st_size) {
    char oldname[PATH_MAX];
    scnprintf(oldname, sizeof(oldname), "%s.old", filename);
    unlink(oldname);
    rename(filename, oldname);
    }
    trace.output = fopen(filename, "w");
    return trace.output == core::ptr::null_mut() ? -errno : 0;
    }
    static int parse_pagefaults(const struct option *opt, const char *str,
    int unset __maybe_unused)
    {
    int *trace_pgfaults = opt.value;
    if (strcmp(str, "all") == 0)
// trace_pgfaults |= TRACE_PFMAJ | TRACE_PFMIN;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(str, 0: "maj") ==) -> else {
    else if (strcmp(str, "maj") == 0)
// trace_pgfaults |= TRACE_PFMAJ;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(str, 0: "min") ==) -> else {
    else if (strcmp(str, "min") == 0)
// trace_pgfaults |= TRACE_PFMIN;
    else
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn evlist__set_default_evsel_handler(evlist: *mut evlist, handler: *mut c_void) {
    static void evlist__set_default_evsel_handler(struct evlist *evlist, void *handler)
    {
    struct evsel *evsel;
    evlist__for_each_entry(evlist, evsel) {
    if (evsel.handler == core::ptr::null_mut())
    evsel.handler = handler;
    }
    }
#[no_mangle]
unsafe extern "C" fn evsel__set_syscall_arg_fmt(evsel: *mut evsel, name: *const c_char) {
    static void evsel__set_syscall_arg_fmt(struct evsel *evsel, const char *name)
    {
    struct syscall_arg_fmt *fmt = evsel__syscall_arg_fmt(evsel);
    if (fmt) {
    const struct syscall_fmt *scfmt = syscall_fmt__find(name);
    if (scfmt) {
    const struct tep_event *tp_format = evsel__tp_format(evsel);
    if (tp_format) {
    let mut skip: c_int = 0;
    if (strcmp(tp_format.format.fields.name, "__syscall_nr") == 0 ||
    strcmp(tp_format.format.fields.name, "nr") == 0)
    ++skip;
    memcpy(fmt + skip, scfmt.arg,
    (tp_format.format.nr_fields - skip) * sizeof(*fmt));
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn evlist__set_syscall_tp_fields(evlist: *mut evlist, use_btf: *mut bool) -> c_int {
    static int evlist__set_syscall_tp_fields(struct evlist *evlist, bool *use_btf)
    {
    struct evsel *evsel;
    evlist__for_each_entry(evlist, evsel) {
    const struct tep_event *tp_format;
    if (evsel.priv)
    continue;
    tp_format = evsel__tp_format(evsel);
    if (!tp_format)
    continue;
    if (strcmp(tp_format.system, "syscalls")) {
    evsel__init_tp_arg_scnprintf(evsel, use_btf);
    continue;
    }
    if (evsel__init_syscall_tp(evsel))
    return -1;
    if (!strncmp(tp_format.name, "sys_enter_", 10)) {
    struct syscall_tp *sc = __evsel__syscall_tp(evsel);
    if (__tp_field__init_ptr(&sc.args, sc.id.offset + sizeof(u64)))
    return -1;
    evsel__set_syscall_arg_fmt(evsel,
    tp_format.name + sizeof("sys_enter_") - 1);
    } else if (!strncmp(tp_format.name, "sys_exit_", 9)) {
    struct syscall_tp *sc = __evsel__syscall_tp(evsel);
    if (__tp_field__init_uint(&sc.ret, sizeof(u64),
    sc.id.offset + sizeof(u64),
    evsel.needs_swap))
    return -1;
    evsel__set_syscall_arg_fmt(evsel,
    tp_format.name + sizeof("sys_exit_") - 1);
    }
    }
    return 0;
    }
//
// XXX: Hackish, just splitting the combined -e+--event (syscalls
// (raw_syscalls:{sys_{enter,exit}} + events (tracepoints, HW, SW, etc) to use
// existing facilities unchanged (trace->ev_qualifier + parse_options()).
//
// It'd be better to introduce a parse_options() variant that would return a
// list with the terms it didn't match to an event...
//
    static int trace__parse_events_option(const struct option *opt, const char *str,
    int unset __maybe_unused)
    {
    struct trace *trace = (struct trace *)opt.value;
    const char *s;
    char *strd, *sep = core::ptr::null_mut(), *lists[2] = { core::ptr::null_mut(), core::ptr::null_mut(), };
    let mut len: c_int = strlen(str) + 1, err = -1, list, idx;
    char *strace_groups_dir = system_path(STRACE_GROUPS_DIR);
    char group_name[PATH_MAX];
    const struct syscall_fmt *fmt;
    if (strace_groups_dir == core::ptr::null_mut())
    return -1;
    s = strd = strdup(str);
    if (strd == core::ptr::null_mut())
    return -1;
    if (*s == '!') {
    ++s;
    trace.not_ev_qualifier = true;
    }
    while (1) {
    if ((sep = strchr((char *)s, ',')) != core::ptr::null_mut())
// sep = '\0';
    list = 0;
// TODO: support for more than just perf binary machine type syscalls.
    if (syscalltbl__id(EM_HOST, s) >= 0 ||
    syscalltbl__strglobmatch_first(EM_HOST, s, &idx) >= 0) {
    list = 1;
    goto do_concat;
    }
    fmt = syscall_fmt__find_by_alias(s);
    if (fmt != core::ptr::null_mut()) {
    list = 1;
    s = fmt.name;
    } else {
    path__join(group_name, sizeof(group_name), strace_groups_dir, s);
    if (access(group_name, R_OK) == 0)
    list = 1;
    }
    do_concat:
    if (lists[list]) {
    sprintf(lists[list] + strlen(lists[list]), ",%s", s);
    } else {
    lists[list] = malloc(len);
    if (lists[list] == core::ptr::null_mut())
    goto out;
    strcpy(lists[list], s);
    }
    if (!sep)
    break;
// sep = ',';
    s = sep + 1;
    }
    if (lists[1] != core::ptr::null_mut()) {
    struct strlist_config slist_config = {
    .dirname = strace_groups_dir,
    };
    trace.ev_qualifier = strlist__new(lists[1], &slist_config);
    if (trace.ev_qualifier == core::ptr::null_mut()) {
    fputs("Not enough memory to parse event qualifier", trace.output);
    goto out;
    }
    if (trace__validate_ev_qualifier(trace))
    goto out;
    trace.trace_syscalls = true;
    }
    err = 0;
    if (lists[0]) {
    struct parse_events_option_args parse_events_option_args = {
    .evlistp = &trace.evlist,
    };
    struct option o = {
    .value = &parse_events_option_args,
    };
    err = parse_events_option(&o, lists[0], 0);
    }
    out:
    free(strace_groups_dir);
    free(lists[0]);
    free(lists[1]);
    free(strd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__parse_cgroups(opt: *const option, str: *const c_char, unset: c_int) -> c_int {
    static int trace__parse_cgroups(const struct option *opt, const char *str, int unset)
    {
    struct trace *trace = opt.value;
    if (!list_empty(&evlist__core(trace.evlist).entries)) {
    struct option o = {
    .value = &trace.evlist,
    };
    return parse_cgroups(&o, str, unset);
    }
    trace.cgroup = evlist__findnew_cgroup(trace.evlist, str);
    return 0;
    }
    static int trace__parse_summary_mode(const struct option *opt, const char *str,
    int unset __maybe_unused)
    {
    struct trace *trace = opt.value;
    if (!strcmp(str, "thread")) {
    trace.summary_mode = SUMMARY__BY_THREAD;
    } else if (!strcmp(str, "total")) {
    trace.summary_mode = SUMMARY__BY_TOTAL;
    } else if (!strcmp(str, "cgroup")) {
    trace.summary_mode = SUMMARY__BY_CGROUP;
    } else {
    pr_err("Unknown summary mode: %s\n", str);
    return -1;
    }
    return 0;
    }
    static int trace_parse_callchain_opt(const struct option *opt,
    const char *arg,
    int unset)
    {
    return record_opts__parse_callchain(opt.value, &callchain_param, arg, unset);
    }
#[no_mangle]
unsafe extern "C" fn trace__config(var: *const c_char, value: *const c_char, arg: *mut c_void) -> c_int {
    static int trace__config(const char *var, const char *value, void *arg)
    {
    struct trace *trace = arg;
    let mut err: c_int = 0;
    if (!strcmp(var, "trace.add_events")) {
    trace.perfconfig_events = strdup(value);
    if (trace.perfconfig_events == core::ptr::null_mut()) {
    pr_err("Not enough memory for %s\n", "trace.add_events");
    return -1;
    }
    } else if (!strcmp(var, "trace.show_timestamp")) {
    trace.show_tstamp = perf_config_bool(var, value);
    } else if (!strcmp(var, "trace.show_duration")) {
    trace.show_duration = perf_config_bool(var, value);
    } else if (!strcmp(var, "trace.show_arg_names")) {
    trace.show_arg_names = perf_config_bool(var, value);
    if (!trace.show_arg_names)
    trace.show_zeros = true;
    } else if (!strcmp(var, "trace.show_zeros")) {
    let mut new_show_zeros: bool = perf_config_bool(var, value);
    if (!trace.show_arg_names && !new_show_zeros) {
    pr_warning("trace.show_zeros has to be set when trace.show_arg_names=no\n");
    goto out;
    }
    trace.show_zeros = new_show_zeros;
    } else if (!strcmp(var, "trace.show_prefix")) {
    trace.show_string_prefix = perf_config_bool(var, value);
    } else if (!strcmp(var, "trace.no_inherit")) {
    trace.opts.no_inherit = perf_config_bool(var, value);
    } else if (!strcmp(var, "trace.args_alignment")) {
    let mut args_alignment: c_int = 0;
    if (perf_config_int(&args_alignment, var, value) == 0)
    trace.args_alignment = args_alignment;
    } else if (!strcmp(var, "trace.tracepoint_beautifiers")) {
    if (strcasecmp(value, "libtraceevent") == 0)
    trace.libtraceevent_print = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcasecmp(value, 0: "libbeauty") ==) -> else {
    else if (strcasecmp(value, "libbeauty") == 0)
    trace.libtraceevent_print = false;
    }
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn trace__exit(trace: *mut trace) {
    static void trace__exit(struct trace *trace)
    {
    thread__zput(trace.current);
    strlist__delete(trace.ev_qualifier);
    zfree(&trace.ev_qualifier_ids.entries);
    if (trace.syscalls.table) {
    for (size_t i = 0; i < trace.syscalls.table_size; i++)
    syscall__delete(trace.syscalls.table[i]);
    zfree(&trace.syscalls.table);
    }
    zfree(&trace.perfconfig_events);
    evlist__put(trace.evlist);
    trace.evlist = core::ptr::null_mut();
    ordered_events__free(&trace.oe.data);

    btf__free(trace.btf);
    trace.btf = core::ptr::null_mut();

    }
#[no_mangle]
pub unsafe extern "C" fn cmd_trace(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_trace(int argc, const char **argv)
    {
    const char *trace_usage[] = {
    "perf trace [<options>] [<command>]",
    "perf trace [<options>] -- <command> [<options>]",
    "perf trace record [<options>] [<command>]",
    "perf trace record [<options>] -- <command> [<options>]",
    core::ptr::null_mut()
    };
    struct trace trace = {
    .opts = {
    .target = {
    .uses_mmap = true,
    },
    .user_freq     = UINT_MAX,
    .user_interval = ULLONG_MAX,
    .no_buffering  = true,
    .mmap_pages    = UINT_MAX,
    },
    .output = stderr,
    .show_comm = true,
    .show_tstamp = true,
    .show_duration = true,
    .show_arg_names = true,
    .args_alignment = 70,
    .trace_syscalls = false,
    .kernel_syscallchains = false,
    .max_stack = UINT_MAX,
    .max_events = ULONG_MAX,
    };
    const char *output_name = core::ptr::null_mut();
    const struct option trace_options[] = {
    OPT_CALLBACK('e', "event", &trace, "event",
    "event/syscall selector. use 'perf list' to list available events",
    trace__parse_events_option),
    OPT_CALLBACK(0, "filter", &trace.evlist, "filter",
    "event filter", parse_filter),
    OPT_BOOLEAN(0, "comm", &trace.show_comm,
    "show the thread COMM next to its id"),
    OPT_BOOLEAN(0, "tool_stats", &trace.show_tool_stats, "show tool stats"),
    OPT_CALLBACK(0, "expr", &trace, "expr", "list of syscalls/events to trace",
    trace__parse_events_option),
    OPT_STRING('o', "output", &output_name, "file", "output file name"),
    OPT_STRING('i', "input", &input_name, "file", "Analyze events in file"),
    OPT_STRING('p', "pid", &trace.opts.target.pid, "pid",
    "trace events on existing process id"),
    OPT_STRING('t', "tid", &trace.opts.target.tid, "tid",
    "trace events on existing thread id"),
    OPT_CALLBACK(0, "filter-pids", &trace, "CSV list of pids",
    "pids to filter (by the kernel)", trace__set_filter_pids_from_option),
    OPT_BOOLEAN('a', "all-cpus", &trace.opts.target.system_wide,
    "system-wide collection from all CPUs"),
    OPT_STRING('C', "cpu", &trace.opts.target.cpu_list, "cpu",
    "list of cpus to monitor"),
    OPT_BOOLEAN(0, "no-inherit", &trace.opts.no_inherit,
    "child tasks do not inherit counters"),
    OPT_CALLBACK('m', "mmap-pages", &trace.opts.mmap_pages, "pages",
    "number of mmap data pages", evlist__parse_mmap_pages),
    OPT_STRING('u', "uid", &trace.uid_str, "user", "user to profile"),
    OPT_BOOLEAN(0, "show-cpu", &trace.show_cpu, "show cpu id"),
    OPT_CALLBACK(0, "duration", &trace, "float",
    "show only events with duration > N.M ms",
    trace__set_duration),
    OPT_BOOLEAN(0, "sched", &trace.sched, "show blocking scheduler events"),
    OPT_INCR('v', "verbose", &verbose, "be more verbose"),
    OPT_BOOLEAN('T', "time", &trace.full_time,
    "Show full timestamp, not time relative to first start"),
    OPT_BOOLEAN(0, "failure", &trace.failure_only,
    "Show only syscalls that failed"),
    OPT_BOOLEAN('s', "summary", &trace.summary_only,
    "Show only syscall summary with statistics"),
    OPT_BOOLEAN('S', "with-summary", &trace.summary,
    "Show all syscalls and summary with statistics"),
    OPT_BOOLEAN(0, "errno-summary", &trace.errno_summary,
    "Show errno stats per syscall, use with -s or -S"),
    OPT_CALLBACK(0, "summary-mode", &trace, "mode",
    "How to show summary: select thread (default), total or cgroup",
    trace__parse_summary_mode),
    OPT_CALLBACK_DEFAULT('F', "pf", &trace.trace_pgfaults, "all|maj|min",
    "Trace pagefaults", parse_pagefaults, "maj"),
    OPT_BOOLEAN(0, "syscalls", &trace.trace_syscalls, "Trace syscalls"),
    OPT_BOOLEAN('f', "force", &trace.force, "don't complain, do it"),
    OPT_CALLBACK(0, "call-graph", &trace.opts,
    "record_mode[,record_size]", record_callchain_help,
    &trace_parse_callchain_opt),
    OPT_BOOLEAN(0, "libtraceevent_print", &trace.libtraceevent_print,
    "Use libtraceevent to print the tracepoint arguments."),
    OPT_BOOLEAN(0, "kernel-syscall-graph", &trace.kernel_syscallchains,
    "Show the kernel callchains on the syscall exit path"),
    OPT_ULONG(0, "max-events", &trace.max_events,
    "Set the maximum number of events to print, exit after that is reached. "),
    OPT_UINTEGER(0, "min-stack", &trace.min_stack,
    "Set the minimum stack depth when parsing the callchain, "
    "anything below the specified depth will be ignored."),
    OPT_UINTEGER(0, "max-stack", &trace.max_stack,
    "Set the maximum stack depth when parsing the callchain, "
    "anything beyond the specified depth will be ignored. "
    "Default: kernel.perf_event_max_stack or " __stringify(PERF_MAX_STACK_DEPTH)),
    OPT_BOOLEAN(0, "sort-events", &trace.sort_events,
    "Sort batch of events before processing, use if getting out of order events"),
    OPT_BOOLEAN(0, "print-sample", &trace.print_sample,
    "print the PERF_RECORD_SAMPLE PERF_SAMPLE_ info, for debugging"),
    OPT_UINTEGER(0, "proc-map-timeout", &proc_map_timeout,
    "per thread proc mmap processing timeout in ms"),
    OPT_CALLBACK('G', "cgroup", &trace, "name", "monitor event in cgroup name only",
    trace__parse_cgroups),
    OPT_INTEGER('D', "delay", &trace.opts.target.initial_delay,
    "ms to wait before starting measurement after program "
    "start"),
    OPT_BOOLEAN(0, "force-btf", &trace.force_btf, "Prefer btf_dump general pretty printer"
    "to customized ones"),
    OPT_BOOLEAN(0, "bitmask-list", &trace.bitmask_list, "Show bitmask as a human-readable list"),
    OPT_BOOLEAN(0, "bpf-summary", &trace.summary_bpf, "Summary syscall stats in BPF"),
    OPT_INTEGER(0, "max-summary", &trace.max_summary,
    "Max number of entries in the summary."),
    OPTS_EVSWITCH(&trace.evswitch),
    OPT_END()
    };
    let mut max_stack_user_set: bool __maybe_unused = true;
    let mut mmap_pages_user_set: bool = true;
    struct evsel *evsel;
    const char * const trace_subcommands[] = { "record", core::ptr::null_mut() };
    let mut err: c_int = -1;
    char bf[BUFSIZ];
    struct sigaction sigchld_act;
    signal(SIGSEGV, sighandler_dump_stack);
    signal(SIGFPE, sighandler_dump_stack);
    signal(SIGINT, sighandler_interrupt);
    memset(&sigchld_act, 0, sizeof(sigchld_act));
    sigchld_act.sa_flags = SA_SIGINFO;
    sigchld_act.sa_sigaction = sighandler_chld;
    sigaction(SIGCHLD, &sigchld_act, core::ptr::null_mut());
    ordered_events__init(&trace.oe.data, ordered_events__deliver_event, &trace);
    ordered_events__set_copy_on_queue(&trace.oe.data, true);
    trace.evlist = evlist__new();
    if (trace.evlist == core::ptr::null_mut()) {
    pr_err("Not enough memory to run!\n");
    err = -ENOMEM;
    goto out;
    }
//
// Parsing .perfconfig may entail creating a BPF event, that may need
// to create BPF maps, so bump RLIM_MEMLOCK as the default 64K setting
// is too small. This affects just this process, not touching the
// global setting. If it fails we'll get something in 'perf trace -v'
// to help diagnose the problem.
//
    rlimit__bump_memlock();
    err = perf_config(trace__config, &trace);
    if (err)
    goto out;
    argc = parse_options_subcommand(argc, argv, trace_options, trace_subcommands,
    trace_usage, PARSE_OPT_STOP_AT_NON_OPTION);
//
// Here we already passed thru trace__parse_events_option() and it has
// already figured out if -e syscall_name, if not but if --event
// foo:bar was used, the user is interested _just_ in those, say,
// tracepoint events, not in the strace-like syscall-name-based mode.
//
// This is important because we need to check if strace-like mode is
// needed to decided if we should filter out the eBPF
// __augmented_syscalls__ code, if it is in the mix, say, via
// .perfconfig trace.add_events, and filter those out.
//
    if (!trace.trace_syscalls && !trace.trace_pgfaults &&
    evlist__nr_entries(trace.evlist) == 0 /* Was --events used? */) {
    trace.trace_syscalls = true;
    }
//
// Now that we have --verbose figured out, lets see if we need to parse
// events from .perfconfig, so that if those events fail parsing, say some
// BPF program fails, then we'll be able to use --verbose to see what went
// wrong in more detail.
//
    if (trace.perfconfig_events != core::ptr::null_mut()) {
    struct parse_events_error parse_err;
    parse_events_error__init(&parse_err);
    err = parse_events(trace.evlist, trace.perfconfig_events, &parse_err);
    if (err)
    parse_events_error__print(&parse_err, trace.perfconfig_events);
    parse_events_error__exit(&parse_err);
    if (err)
    goto out;
    }
    if (trace.show_cpu)
    trace.opts.sample_cpu = true;
    if ((nr_cgroups || trace.cgroup) && !trace.opts.target.system_wide) {
    usage_with_options_msg(trace_usage, trace_options,
    "cgroup monitoring only available in system-wide mode");
    }
    if (!trace.trace_syscalls)
    goto skip_augmentation;
    if ((argc >= 1) && (strcmp(argv[0], "record") == 0)) {
    pr_debug("Syscall augmentation fails with record, disabling augmentation");
    goto skip_augmentation;
    }
    if (trace.summary_bpf) {
    if (!trace.opts.target.system_wide) {
// TODO: Add filters in the BPF to support other targets.
    pr_err("Error: --bpf-summary only works for system-wide mode.\n");
    goto out;
    }
    if (trace.summary_only)
    goto skip_augmentation;
    }
    err = augmented_syscalls__prepare();
    if (err < 0)
    goto skip_augmentation;
    trace__add_syscall_newtp(&trace);
    err = augmented_syscalls__create_bpf_output(trace.evlist);
    if (err == 0)
    trace.syscalls.events.bpf_output = evlist__last(trace.evlist);
    skip_augmentation:
    err = -1;
    if (trace.trace_pgfaults) {
    trace.opts.sample_address = true;
    trace.opts.sample_time = true;
    }
    if (trace.opts.mmap_pages == UINT_MAX)
    mmap_pages_user_set = false;
    if (trace.max_stack == UINT_MAX) {
    trace.max_stack = input_name ? PERF_MAX_STACK_DEPTH : sysctl__max_stack();
    max_stack_user_set = false;
    }

    if ((trace.min_stack || max_stack_user_set) && !callchain_param.enabled) {
    record_opts__parse_callchain(&trace.opts, &callchain_param, "dwarf", false);
    }

    if (callchain_param.enabled) {
    if (!mmap_pages_user_set && geteuid() == 0)
    trace.opts.mmap_pages = perf_event_mlock_kb_in_pages() * 4;
    symbol_conf.use_callchain = true;
    }
    if (evlist__nr_entries(trace.evlist) > 0) {
    let mut use_btf: bool = false;
    evlist__set_default_evsel_handler(trace.evlist, trace__event_handler);
    if (evlist__set_syscall_tp_fields(trace.evlist, &use_btf)) {
    perror("failed to set syscalls:* tracepoint fields");
    goto out;
    }
    if (use_btf)
    trace__load_vmlinux_btf(&trace);
    }
//
// If we are augmenting syscalls, then combine what we put in the
// __augmented_syscalls__ BPF map with what is in the
// syscalls:sys_exit_FOO tracepoints, i.e. just like we do without BPF,
// combining raw_syscalls:sys_enter with raw_syscalls:sys_exit.
//
// We'll switch to look at two BPF maps, one for sys_enter and the
// other for sys_exit when we start augmenting the sys_exit paths with
// buffers that are being copied from kernel to userspace, think 'read'
// syscall.
//
    if (trace.syscalls.events.bpf_output) {
    evlist__for_each_entry(trace.evlist, evsel) {
    let mut raw_syscalls_sys_exit: bool = evsel__name_is(evsel, "raw_syscalls:sys_exit");
    if (raw_syscalls_sys_exit) {
    trace.raw_augmented_syscalls = true;
    goto init_augmented_syscall_tp;
    }
    if (trace.syscalls.events.bpf_output.priv == core::ptr::null_mut() &&
    strstr(evsel__name(evsel), "syscalls:sys_enter")) {
    struct evsel *augmented = trace.syscalls.events.bpf_output;
    if (evsel__init_augmented_syscall_tp(augmented, evsel) ||
    evsel__init_augmented_syscall_tp_args(augmented))
    goto out;
//
// Augmented is __augmented_syscalls__ BPF_OUTPUT event
// Above we made sure we can get from the payload the tp fields
// that we get from syscalls:sys_enter tracefs format file.
//
    augmented.handler = trace__sys_enter;
//
// Now we do the same for the *syscalls:sys_enter event so that
// if we handle it directly, i.e. if the BPF prog returns 0 so
// as not to filter it, then we'll handle it just like we would
// for the BPF_OUTPUT one:
//
    if (evsel__init_augmented_syscall_tp(evsel, evsel) ||
    evsel__init_augmented_syscall_tp_args(evsel))
    goto out;
    evsel.handler = trace__sys_enter;
    }
    if (strstarts(evsel__name(evsel), "syscalls:sys_exit_")) {
    struct syscall_tp *sc;
    init_augmented_syscall_tp:
    if (evsel__init_augmented_syscall_tp(evsel, evsel))
    goto out;
    sc = __evsel__syscall_tp(evsel);
//
// For now with BPF raw_augmented we hook into
// raw_syscalls:sys_enter and there we get all
// 6 syscall args plus the tracepoint common
// fields and the syscall_nr (another long).
// So we check if that is the case and if so
// don't look after the sc->args_size but
// always after the full raw_syscalls:sys_enter
// payload, which is fixed.
//
// We'll revisit this later to pass
// s->args_size to the BPF augmenter (now
// tools/perf/examples/bpf/augmented_raw_syscalls.c,
// so that it copies only what we need for each
// syscall, like what happens when we use
// syscalls:sys_enter_NAME, so that we reduce
// the kernel/userspace traffic to just what is
// needed for each syscall.
//
    if (trace.raw_augmented_syscalls)
    trace.raw_augmented_syscalls_args_size = (6 + 1) * sizeof(long) + sc.id.offset;
    evsel__init_augmented_syscall_tp_ret(evsel);
    evsel.handler = trace__sys_exit;
    }
    }
    }
    if ((argc >= 1) && (strcmp(argv[0], "record") == 0)) {
    err = trace__record(&trace, argc-1, &argv[1]);
    goto out;
    }
// Using just --errno-summary will trigger --summary
    if (trace.errno_summary && !trace.summary && !trace.summary_only)
    trace.summary_only = true;
// summary_only implies summary option, but don't overwrite summary if set
    if (trace.summary_only)
    trace.summary = trace.summary_only;
// Keep exited threads, otherwise information might be lost for summary
    if (trace.summary) {
    symbol_conf.keep_exited_threads = true;
    if (trace.summary_mode == SUMMARY__NONE)
    trace.summary_mode = SUMMARY__BY_THREAD;
    if (!trace.summary_bpf && trace.summary_mode == SUMMARY__BY_CGROUP) {
    pr_err("Error: --summary-mode=cgroup only works with --bpf-summary\n");
    err = -EINVAL;
    goto out;
    }
    }
    if (output_name != core::ptr::null_mut()) {
    err = trace__open_output(&trace, output_name);
    if (err < 0) {
    perror("failed to create output file");
    goto out;
    }
    }
    err = evswitch__init(&trace.evswitch, trace.evlist, stderr);
    if (err)
    goto out_close;
    err = target__validate(&trace.opts.target);
    if (err) {
    target__strerror(&trace.opts.target, err, bf, sizeof(bf));
    fprintf(trace.output, "%s", bf);
    goto out_close;
    }
    if (trace.uid_str) {
    let mut uid: uid_t = parse_uid(trace.uid_str);
    if (uid == UINT_MAX) {
    ui__error("Invalid User: %s", trace.uid_str);
    err = -EINVAL;
    goto out_close;
    }
    err = parse_uid_filter(trace.evlist, uid);
    if (err)
    goto out_close;
    trace.opts.target.system_wide = true;
    }
    if (!argc && target__none(&trace.opts.target))
    trace.opts.target.system_wide = true;
    if (input_name)
    err = trace__replay(&trace);
    else
    err = trace__run(&trace, argc, argv);
    out_close:
    if (output_name != core::ptr::null_mut())
    fclose(trace.output);
    out:
    trace__exit(&trace);
    augmented_syscalls__cleanup();
    return err;
    }
