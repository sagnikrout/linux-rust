//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/trace-event-parse.c
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
//
// Copyright (C) 2009, Steven Rostedt <srostedt@redhat.com>
//

    static int get_common_field(struct scripting_context *context,
    int *offset, int *size, const char *type)
    {
    struct tep_handle *pevent = context.pevent;
    struct tep_event *event;
    struct tep_format_field *field;
    if (!*size) {
    event = tep_get_first_event(pevent);
    if (!event)
    return 0;
    field = tep_find_common_field(event, type);
    if (!field)
    return 0;
// offset = field->offset;
// size = field->size;
    }
    return tep_read_number(pevent, context.event_data + *offset, *size);
    }
#[no_mangle]
pub unsafe extern "C" fn common_lock_depth(context: *mut scripting_context) -> c_int {
    int common_lock_depth(struct scripting_context *context)
    {
    static int offset;
    static int size;
    int ret;
    ret = get_common_field(context, &size, &offset,
    "common_lock_depth");
    if (ret < 0)
    return -1;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn common_flags(context: *mut scripting_context) -> c_int {
    int common_flags(struct scripting_context *context)
    {
    static int offset;
    static int size;
    int ret;
    ret = get_common_field(context, &size, &offset,
    "common_flags");
    if (ret < 0)
    return -1;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn common_pc(context: *mut scripting_context) -> c_int {
    int common_pc(struct scripting_context *context)
    {
    static int offset;
    static int size;
    int ret;
    ret = get_common_field(context, &size, &offset,
    "common_preempt_count");
    if (ret < 0)
    return -1;
    return ret;
    }
    unsigned long long
    raw_field_value(struct tep_event *event, const char *name, void *data)
    {
    struct tep_format_field *field;
    unsigned long long val;
    field = tep_find_any_field(event, name);
    if (!field)
    return 0ULL;
    tep_read_number_field(field, data, &val);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn read_size(event: *mut tep_event, ptr: *mut c_void, size: c_int) -> c_ulonglong {
    unsigned long long read_size(struct tep_event *event, void *ptr, int size)
    {
    return tep_read_number(event.tep, ptr, size);
    }
    void event_format__fprintf(const struct tep_event *event,
    int cpu, void *data, int size, FILE *fp)
    {
    struct tep_record record;
    struct trace_seq s;
    memset(&record, 0, sizeof(record));
    record.cpu = cpu;
    record.size = size;
    record.data = data;
    trace_seq_init(&s);
    tep_print_event(event.tep, &s, &record, "%s", TEP_PRINT_INFO);
    trace_seq_do_fprintf(&s, fp);
    trace_seq_destroy(&s);
    }
//
// prev_state is of size long, which is 32 bits on 32 bit architectures.
// As it needs to have the same bits for both 32 bit and 64 bit architectures
// we can just assume that the flags we care about will all be within
// the 32 bits.
//
pub const MAX_STATE_BITS: c_int = 32;
    static const char *convert_sym(struct tep_print_flag_sym *sym)
    {
    static char save_states[MAX_STATE_BITS + 1];
    memset(save_states, 0, sizeof(save_states));
// This is the flags for the prev_state_field, now make them into a string
    for (; sym; sym = sym.next) {
    let mut bitmask: c_long = strtoul(sym.value, core::ptr::null_mut(), 0);
    int i;
    for (i = 0; !(bitmask & 1); i++)
    bitmask >>= 1;
    if (i >= MAX_STATE_BITS)
    continue;
    save_states[i] = sym.str[0];
    }
    return save_states;
    }
    static struct tep_print_arg_field *
    find_arg_field(struct tep_format_field *prev_state_field, struct tep_print_arg *arg)
    {
    struct tep_print_arg_field *field;
    if (!arg)
    return core::ptr::null_mut();
    if (arg.type == TEP_PRINT_FIELD)
    return &arg.field;
    if (arg.type == TEP_PRINT_OP) {
    field = find_arg_field(prev_state_field, arg.op.left);
    if (field && field.field == prev_state_field)
    return field;
    field = find_arg_field(prev_state_field, arg.op.right);
    if (field && field.field == prev_state_field)
    return field;
    }
    return core::ptr::null_mut();
    }
    static struct tep_print_flag_sym *
    test_flags(struct tep_format_field *prev_state_field, struct tep_print_arg *arg)
    {
    struct tep_print_arg_field *field;
    field = find_arg_field(prev_state_field, arg.flags.field);
    if (!field)
    return core::ptr::null_mut();
    return arg.flags.flags;
    }
    static struct tep_print_flag_sym *
    search_op(struct tep_format_field *prev_state_field, struct tep_print_arg *arg)
    {
    struct tep_print_flag_sym *sym = core::ptr::null_mut();
    if (!arg)
    return core::ptr::null_mut();
    if (arg.type == TEP_PRINT_OP) {
    sym = search_op(prev_state_field, arg.op.left);
    if (sym)
    return sym;
    sym = search_op(prev_state_field, arg.op.right);
    if (sym)
    return sym;
    } else if (arg.type == TEP_PRINT_FLAGS) {
    sym = test_flags(prev_state_field, arg);
    }
    return sym;
    }
    const char *parse_task_states(struct tep_format_field *state_field)
    {
    struct tep_print_flag_sym *sym;
    struct tep_print_arg *arg;
    struct tep_event *event;
    event = state_field.event;
//
// Look at the event format fields, and search for where
// the prev_state is parsed via the format flags.
//
    for (arg = event.print_fmt.args; arg; arg = arg.next) {
//
// Currently, the __print_flags() for the prev_state
// is embedded in operations, so they too must be
// searched.
//
    sym = search_op(state_field, arg);
    if (sym)
    return convert_sym(sym);
    }
    return core::ptr::null_mut();
    }
    void parse_ftrace_printk(struct tep_handle *pevent,
    char *file, unsigned int size __maybe_unused)
    {
    unsigned long long addr;
    char *printk;
    char *line;
    char *next = core::ptr::null_mut();
    char *addr_str;
    char *fmt = core::ptr::null_mut();
    line = strtok_r(file, "\n", &next);
    while (line) {
    addr_str = strtok_r(line, ":", &fmt);
    if (!addr_str) {
    pr_warning("printk format with empty entry");
    break;
    }
    addr = strtoull(addr_str, core::ptr::null_mut(), 16);
// fmt still has a space, skip it
    printk = strdup(fmt+1);
    line = strtok_r(core::ptr::null_mut(), "\n", &next);
    tep_register_print_string(pevent, printk, addr);
    free(printk);
    }
    }
    void parse_saved_cmdline(struct tep_handle *pevent,
    char *file, unsigned int size __maybe_unused)
    {
    char comm[17]; /* Max comm length in the kernel is 16. */
    char *line;
    char *next = core::ptr::null_mut();
    int pid;
    line = strtok_r(file, "\n", &next);
    while (line) {
    if (sscanf(line, "%d %16s", &pid, comm) == 2)
    tep_register_comm(pevent, comm, pid);
    line = strtok_r(core::ptr::null_mut(), "\n", &next);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn parse_ftrace_file(pevent: *mut tep_handle, buf: *mut c_char, size: c_ulong) -> c_int {
    int parse_ftrace_file(struct tep_handle *pevent, char *buf, unsigned long size)
    {
    return tep_parse_event(pevent, buf, size, "ftrace");
    }
    int parse_event_file(struct tep_handle *pevent,
    char *buf, unsigned long size, char *sys)
    {
    return tep_parse_event(pevent, buf, size, sys);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flag {
    pub name: *const c_char,
    pub value: c_ulonglong,
}

    static const struct flag flags[] = {
    { "HI_SOFTIRQ", 0 },
    { "TIMER_SOFTIRQ", 1 },
    { "NET_TX_SOFTIRQ", 2 },
    { "NET_RX_SOFTIRQ", 3 },
    { "BLOCK_SOFTIRQ", 4 },
    { "IRQ_POLL_SOFTIRQ", 5 },
    { "TASKLET_SOFTIRQ", 6 },
    { "SCHED_SOFTIRQ", 7 },
    { "HRTIMER_SOFTIRQ", 8 },
    { "RCU_SOFTIRQ", 9 },
    { "HRTIMER_NORESTART", 0 },
    { "HRTIMER_RESTART", 1 },
    };
#[no_mangle]
pub unsafe extern "C" fn eval_flag(flag: *const c_char) -> c_ulonglong {
    unsigned long long eval_flag(const char *flag)
    {
    int i;
//
// Some flags in the format files do not get converted.
// If the flag is not numeric, see if it is something that
// we already know about.
//
    if (isdigit(flag[0]))
    return strtoull(flag, core::ptr::null_mut(), 0);
    for (i = 0; i < (int)(ARRAY_SIZE(flags)); i++)
    if (strcmp(flags[i].name, flag) == 0)
    return flags[i].value;
    return 0;
    }
