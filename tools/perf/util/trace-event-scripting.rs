//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/trace-event-scripting.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// trace-event-scripting.  Scripting engine common and initialization code.
//
// Copyright (C) 2009-2010 Tom Zanussi <tzanussi@gmail.com>
//

    let mut scripting_max_stack: c_uint = PERF_MAX_STACK_DEPTH;
    struct scripting_context *scripting_context;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct script_spec {
    pub node: list_head,
    pub ops: *mut scripting_ops,
    pub spec: [c_char; ],
}

    static LIST_HEAD(script_specs);
    static struct script_spec *script_spec__new(const char *spec,
    struct scripting_ops *ops)
    {
    struct script_spec *s = malloc(sizeof(*s) + strlen(spec) + 1);
    if (s != core::ptr::null_mut()) {
    strcpy(s.spec, spec);
    s.ops = ops;
    }
    return s;
    }
#[no_mangle]
unsafe extern "C" fn script_spec__add(s: *mut script_spec) {
    static void script_spec__add(struct script_spec *s)
    {
    list_add_tail(&s.node, &script_specs);
    }
    static struct script_spec *script_spec__find(const char *spec)
    {
    struct script_spec *s;
    list_for_each_entry(s, &script_specs, node)
    if (strcasecmp(s.spec, spec) == 0)
    return s;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn script_spec_register(spec: *const c_char, ops: *mut scripting_ops) -> c_int {
    static int script_spec_register(const char *spec, struct scripting_ops *ops)
    {
    struct script_spec *s;
    s = script_spec__find(spec);
    if (s)
    return -1;
    s = script_spec__new(spec, ops);
    if (!s)
    return -1;
    script_spec__add(s);
    return 0;
    }
    struct scripting_ops *script_spec__lookup(const char *spec)
    {
    struct script_spec *s = script_spec__find(spec);
    if (!s)
    return core::ptr::null_mut();
    return s.ops;
    }
#[no_mangle]
pub unsafe extern "C" fn script_spec__for_each(ops: *mut *mut int (cb)(struct scripting_ops, spec): *const c_char) -> c_int {
    int script_spec__for_each(int (*cb)(struct scripting_ops *ops, const char *spec))
    {
    struct script_spec *s;
    let mut ret: c_int = 0;
    list_for_each_entry(s, &script_specs, node) {
    ret = cb(s.ops, s.spec);
    if (ret)
    break;
    }
    return ret;
    }
    void scripting_context__update(struct scripting_context *c,
    union perf_event *event,
    struct perf_sample *sample,
    struct addr_location *al,
    struct addr_location *addr_al)
    {

    const struct tep_event *tp_format = evsel__tp_format(sample.evsel);
    c.pevent = tp_format ? tp_format.tep : core::ptr::null_mut();

    c.pevent = core::ptr::null_mut();

    c.event_data = sample.raw_data;
    c.event = event;
    c.sample = sample;
    c.al = al;
    c.addr_al = addr_al;
    }
#[no_mangle]
unsafe extern "C" fn flush_script_unsupported() -> c_int {
    static int flush_script_unsupported(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stop_script_unsupported() -> c_int {
    static int stop_script_unsupported(void)
    {
    return 0;
    }
    static void process_event_unsupported(union perf_event *event __maybe_unused,
    struct perf_sample *sample __maybe_unused,
    struct addr_location *al __maybe_unused,
    struct addr_location *addr_al __maybe_unused)
    {
    }
#[no_mangle]
unsafe extern "C" fn print_python_unsupported_msg() {
    static void print_python_unsupported_msg(void)
    {
    fprintf(stderr, "Python scripting not supported."
    "  Install libpython and rebuild perf to enable it.\n"
    "For example:\n  # apt-get install python-dev (ubuntu)"
    "\n  # yum install python-devel (Fedora)"
    "\n  etc.\n");
    }
    static int python_start_script_unsupported(const char *script __maybe_unused,
    int argc __maybe_unused,
    const char **argv __maybe_unused,
    struct perf_session *session __maybe_unused)
    {
    print_python_unsupported_msg();
    return -1;
    }
    static int python_generate_script_unsupported(struct tep_handle *pevent
    __maybe_unused,
    const char *outfile
    __maybe_unused)
    {
    print_python_unsupported_msg();
    return -1;
    }
    struct scripting_ops python_scripting_unsupported_ops = {
    .name = "Python",
    .dirname = "python",
    .start_script = python_start_script_unsupported,
    .flush_script = flush_script_unsupported,
    .stop_script = stop_script_unsupported,
    .process_event = process_event_unsupported,
    .generate_script = python_generate_script_unsupported,
    };
#[no_mangle]
unsafe extern "C" fn register_python_scripting(scripting_ops: *mut scripting_ops) {
    static void register_python_scripting(struct scripting_ops *scripting_ops)
    {
    if (scripting_context == core::ptr::null_mut())
    scripting_context = malloc(sizeof(*scripting_context));
    if (scripting_context == core::ptr::null_mut() ||
    script_spec_register("Python", scripting_ops) ||
    script_spec_register("py", scripting_ops)) {
    pr_err("Error registering Python script extension: disabling it\n");
    zfree(&scripting_context);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn setup_python_scripting() {
    void setup_python_scripting(void)
    {
    register_python_scripting(&python_scripting_unsupported_ops);
    }

    extern struct scripting_ops python_scripting_ops;
#[no_mangle]
pub unsafe extern "C" fn setup_python_scripting() {
    void setup_python_scripting(void)
    {
    register_python_scripting(&python_scripting_ops);
    }

#[no_mangle]
unsafe extern "C" fn print_perl_unsupported_msg() {
    static void print_perl_unsupported_msg(void)
    {
    fprintf(stderr, "Perl scripting not supported."
    "  Install libperl and rebuild perf to enable it.\n"
    "For example:\n  # apt-get install libperl-dev (ubuntu)"
    "\n  # yum install 'perl(ExtUtils::Embed)' (Fedora)"
    "\n  etc.\n");
    }
    static int perl_start_script_unsupported(const char *script __maybe_unused,
    int argc __maybe_unused,
    const char **argv __maybe_unused,
    struct perf_session *session __maybe_unused)
    {
    print_perl_unsupported_msg();
    return -1;
    }
    static int perl_generate_script_unsupported(struct tep_handle *pevent
    __maybe_unused,
    const char *outfile __maybe_unused)
    {
    print_perl_unsupported_msg();
    return -1;
    }
    struct scripting_ops perl_scripting_unsupported_ops = {
    .name = "Perl",
    .dirname = "perl",
    .start_script = perl_start_script_unsupported,
    .flush_script = flush_script_unsupported,
    .stop_script = stop_script_unsupported,
    .process_event = process_event_unsupported,
    .generate_script = perl_generate_script_unsupported,
    };
#[no_mangle]
unsafe extern "C" fn register_perl_scripting(scripting_ops: *mut scripting_ops) {
    static void register_perl_scripting(struct scripting_ops *scripting_ops)
    {
    if (scripting_context == core::ptr::null_mut())
    scripting_context = malloc(sizeof(*scripting_context));
    if (scripting_context == core::ptr::null_mut() ||
    script_spec_register("Perl", scripting_ops) ||
    script_spec_register("pl", scripting_ops)) {
    pr_err("Error registering Perl script extension: disabling it\n");
    zfree(&scripting_context);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn setup_perl_scripting() {
    void setup_perl_scripting(void)
    {
    register_perl_scripting(&perl_scripting_unsupported_ops);
    }

    extern struct scripting_ops perl_scripting_ops;
#[no_mangle]
pub unsafe extern "C" fn setup_perl_scripting() {
    void setup_perl_scripting(void)
    {
    register_perl_scripting(&perl_scripting_ops);
    }

    static const struct {
    u32 flags;
    const char *name;
    } sample_flags[] = {
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL, "call"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN, "return"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CONDITIONAL, "jcc"},
    {PERF_IP_FLAG_BRANCH, "jmp"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_INTERRUPT, "int"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT, "iret"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET, "syscall"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_SYSCALLRET, "sysret"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_ASYNC, "async"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC |	PERF_IP_FLAG_INTERRUPT,
    "hw int"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TX_ABORT, "tx abrt"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_BEGIN, "tr strt"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_END, "tr end"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMENTRY, "vmentry"},
    {PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMEXIT, "vmexit"},
    {0, core::ptr::null_mut()}
    };
    static const struct {
    u32 flags;
    const char *name;
    } branch_events[] = {
    {PERF_IP_FLAG_BRANCH_MISS, "miss"},
    {PERF_IP_FLAG_NOT_TAKEN, "not_taken"},
    {0, core::ptr::null_mut()}
    };
#[no_mangle]
unsafe extern "C" fn sample_flags_to_name(flags: u32, str: *mut c_char, size: usize) -> c_int {
    static int sample_flags_to_name(u32 flags, char *str, size_t size)
    {
    int i;
    const char *prefix;
    let mut pos: c_int = 0, ret, ev_idx = 0;
    let mut xf: u32 = flags & PERF_ADDITIONAL_STATE_MASK;
    u32 types, events;
    char xs[16] = { 0 };
// Clear additional state bits
    flags &= ~PERF_ADDITIONAL_STATE_MASK;
    if (flags & PERF_IP_FLAG_TRACE_BEGIN)
    prefix = "tr strt ";
#[no_mangle]
pub unsafe extern "C" fn if(PERF_IP_FLAG_TRACE_END: flags &) -> else {
    else if (flags & PERF_IP_FLAG_TRACE_END)
    prefix = "tr end  ";
    else
    prefix = "";
    ret = snprintf(str + pos, size - pos, "%s", prefix);
    if (ret < 0)
    return ret;
    pos += ret;
    flags &= ~(PERF_IP_FLAG_TRACE_BEGIN | PERF_IP_FLAG_TRACE_END);
    types = flags & ~PERF_IP_FLAG_BRANCH_EVENT_MASK;
    for (i = 0; sample_flags[i].name; i++) {
    if (sample_flags[i].flags != types)
    continue;
    ret = snprintf(str + pos, size - pos, "%s", sample_flags[i].name);
    if (ret < 0)
    return ret;
    pos += ret;
    break;
    }
    events = flags & PERF_IP_FLAG_BRANCH_EVENT_MASK;
    for (i = 0; branch_events[i].name; i++) {
    if (!(branch_events[i].flags & events))
    continue;
    ret = snprintf(str + pos, size - pos, !ev_idx ? "/%s" : ",%s",
    branch_events[i].name);
    if (ret < 0)
    return ret;
    pos += ret;
    ev_idx++;
    }
// Add an end character '/' for events
    if (ev_idx) {
    ret = snprintf(str + pos, size - pos, "/");
    if (ret < 0)
    return ret;
    pos += ret;
    }
    if (!xf)
    return pos;
    snprintf(xs, sizeof(xs), "(%s%s%s)",
    flags & PERF_IP_FLAG_IN_TX ? "x" : "",
    flags & PERF_IP_FLAG_INTR_DISABLE ? "D" : "",
    flags & PERF_IP_FLAG_INTR_TOGGLE ? "t" : "");
// Right align the string if its length is less than the limit
    if ((pos + strlen(xs)) < SAMPLE_FLAGS_STR_ALIGNED_SIZE)
    ret = snprintf(str + pos, size - pos, "%*s",
    (int)(SAMPLE_FLAGS_STR_ALIGNED_SIZE - ret), xs);
    else
    ret = snprintf(str + pos, size - pos, " %s", xs);
    if (ret < 0)
    return ret;
    return pos + ret;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_sample__sprintf_flags(flags: u32, str: *mut c_char, sz: usize) -> c_int {
    int perf_sample__sprintf_flags(u32 flags, char *str, size_t sz)
    {
    const char *chars = PERF_IP_FLAG_CHARS;
    let mut n: usize = strlen(PERF_IP_FLAG_CHARS);
    size_t i, pos = 0;
    int ret;
    ret = sample_flags_to_name(flags, str, sz);
    if (ret > 0)
    return ret;
    for (i = 0; i < n; i++, flags >>= 1) {
    if ((flags & 1) && pos < sz)
    str[pos++] = chars[i];
    }
    for (; i < 32; i++, flags >>= 1) {
    if ((flags & 1) && pos < sz)
    str[pos++] = '?';
    }
    if (pos < sz)
    str[pos] = 0;
    return pos;
    }
