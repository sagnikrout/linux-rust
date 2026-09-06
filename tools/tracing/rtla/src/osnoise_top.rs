//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/src/osnoise_top.c
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
// Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
//
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_top_cpu {
    pub sum_runtime: c_ulonglong,
    pub sum_noise: c_ulonglong,
    pub max_noise: c_ulonglong,
    pub max_sample: c_ulonglong,
    pub hw_count: c_ulonglong,
    pub nmi_count: c_ulonglong,
    pub irq_count: c_ulonglong,
    pub softirq_count: c_ulonglong,
    pub thread_count: c_ulonglong,
    pub sum_cycles: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_top_data {
    pub cpu_data: *mut osnoise_top_cpu,
}

//
// osnoise_free_top - free runtime data
//
#[no_mangle]
unsafe extern "C" fn osnoise_free_top(data: *mut osnoise_top_data) {
    static void osnoise_free_top(struct osnoise_top_data *data)
    {
    free(data.cpu_data);
    free(data);
    }
#[no_mangle]
unsafe extern "C" fn osnoise_free_top_tool(tool: *mut osnoise_tool) {
    static void osnoise_free_top_tool(struct osnoise_tool *tool)
    {
    osnoise_free_top(tool.data);
    }
//
// osnoise_alloc_histogram - alloc runtime data
//
    static struct osnoise_top_data *osnoise_alloc_top(void)
    {
    struct osnoise_top_data *data;
    data = calloc(1, sizeof(*data));
    if (!data)
    return core::ptr::null_mut();
// one set of histograms per CPU
    data.cpu_data = calloc(1, sizeof(*data.cpu_data) * nr_cpus);
    if (!data.cpu_data)
    goto cleanup;
    return data;
    cleanup:
    osnoise_free_top(data);
    return core::ptr::null_mut();
    }
//
// osnoise_top_handler - this is the handler for osnoise tracer events
//
    static int
    osnoise_top_handler(struct trace_seq *s, struct tep_record *record,
    struct tep_event *event, void *context)
    {
    struct trace_instance *trace = context;
    struct osnoise_tool *tool;
    unsigned long long val;
    struct osnoise_top_cpu *cpu_data;
    struct osnoise_top_data *data;
    let mut cpu: c_int = record.cpu;
    tool = container_of(trace, struct osnoise_tool, trace);
    data = tool.data;
    cpu_data = &data.cpu_data[cpu];
    cpu_data.sum_cycles++;
    tep_get_field_val(s, event, "runtime", record, &val, 1);
    update_sum(&cpu_data.sum_runtime, &val);
    tep_get_field_val(s, event, "noise", record, &val, 1);
    update_max(&cpu_data.max_noise, &val);
    update_sum(&cpu_data.sum_noise, &val);
    tep_get_field_val(s, event, "max_sample", record, &val, 1);
    update_max(&cpu_data.max_sample, &val);
    tep_get_field_val(s, event, "hw_count", record, &val, 1);
    update_sum(&cpu_data.hw_count, &val);
    tep_get_field_val(s, event, "nmi_count", record, &val, 1);
    update_sum(&cpu_data.nmi_count, &val);
    tep_get_field_val(s, event, "irq_count", record, &val, 1);
    update_sum(&cpu_data.irq_count, &val);
    tep_get_field_val(s, event, "softirq_count", record, &val, 1);
    update_sum(&cpu_data.softirq_count, &val);
    tep_get_field_val(s, event, "thread_count", record, &val, 1);
    update_sum(&cpu_data.thread_count, &val);
    return 0;
    }
//
// osnoise_top_header - print the header of the tool output
//
#[no_mangle]
unsafe extern "C" fn osnoise_top_header(top: *mut osnoise_tool) {
    static void osnoise_top_header(struct osnoise_tool *top)
    {
    struct osnoise_params *params = to_osnoise_params(top.params);
    struct trace_seq *s = top.trace.seq;
    let mut pretty: bool = params.common.pretty_output;
    char duration[26];
    get_duration(top.start_time, duration, sizeof(duration));
    if (pretty)
    trace_seq_printf(s, "\033[2;37;40m");
    trace_seq_printf(s, "                                          ");
    if (params.mode == MODE_OSNOISE) {
    trace_seq_printf(s, "Operating System Noise");
    trace_seq_printf(s, "                                       ");
    } else if (params.mode == MODE_HWNOISE) {
    trace_seq_printf(s, "Hardware-related Noise");
    }
    trace_seq_printf(s, "                                   ");
    if (pretty)
    trace_seq_printf(s, "\033[0;0;0m");
    trace_seq_printf(s, "\n");
    trace_seq_printf(s, "duration: %9s | time is in us\n", duration);
    if (pretty)
    trace_seq_printf(s, "\033[2;30;47m");
    trace_seq_printf(s, "CPU Period       Runtime ");
    trace_seq_printf(s, "       Noise ");
    trace_seq_printf(s, " %% CPU Aval ");
    trace_seq_printf(s, "  Max Noise   Max Single ");
    trace_seq_printf(s, "         HW          NMI");
    if (params.mode == MODE_HWNOISE)
    goto eol;
    trace_seq_printf(s, "          IRQ      Softirq       Thread");
    eol:
    if (pretty)
    trace_seq_printf(s, "\033[0;0;0m");
    trace_seq_printf(s, "\n");
    }
//
// clear_terminal - clears the output terminal
//
#[no_mangle]
unsafe extern "C" fn clear_terminal(seq: *mut trace_seq) {
    static void clear_terminal(struct trace_seq *seq)
    {
    if (!config_debug)
    trace_seq_printf(seq, "\033c");
    }
//
// osnoise_top_print - prints the output of a given CPU
//
#[no_mangle]
unsafe extern "C" fn osnoise_top_print(tool: *mut osnoise_tool, cpu: c_int) {
    static void osnoise_top_print(struct osnoise_tool *tool, int cpu)
    {
    struct osnoise_params *params = to_osnoise_params(tool.params);
    struct trace_seq *s = tool.trace.seq;
    struct osnoise_top_cpu *cpu_data;
    struct osnoise_top_data *data;
    int percentage;
    int decimal;
    data = tool.data;
    cpu_data = &data.cpu_data[cpu];
    if (!cpu_data.sum_runtime)
    return;
    percentage = ((cpu_data.sum_runtime - cpu_data.sum_noise) * 10000000)
    / cpu_data.sum_runtime;
    decimal = percentage % 100000;
    percentage = percentage / 100000;
    trace_seq_printf(s, "%3d #%-6d %12llu ", cpu, cpu_data.sum_cycles, cpu_data.sum_runtime);
    trace_seq_printf(s, "%12llu ", cpu_data.sum_noise);
    trace_seq_printf(s, "  %3d.%05d", percentage, decimal);
    trace_seq_printf(s, "%12llu %12llu", cpu_data.max_noise, cpu_data.max_sample);
    trace_seq_printf(s, "%12llu ", cpu_data.hw_count);
    trace_seq_printf(s, "%12llu ", cpu_data.nmi_count);
    if (params.mode == MODE_HWNOISE) {
    trace_seq_printf(s, "\n");
    return;
    }
    trace_seq_printf(s, "%12llu ", cpu_data.irq_count);
    trace_seq_printf(s, "%12llu ", cpu_data.softirq_count);
    trace_seq_printf(s, "%12llu\n", cpu_data.thread_count);
    }
//
// osnoise_print_stats - print data for all cpus
//
    static void
    osnoise_print_stats(struct osnoise_tool *top)
    {
    struct osnoise_params *params = to_osnoise_params(top.params);
    struct trace_instance *trace = &top.trace;
    int i;
    if (!params.common.quiet)
    clear_terminal(trace.seq);
    osnoise_top_header(top);
    for_each_monitored_cpu(i, &params.common) {
    osnoise_top_print(top, i);
    }
    trace_seq_do_printf(trace.seq);
    trace_seq_reset(trace.seq);
    osnoise_report_missed_events(top);
    }
//
// osnoise_top_apply_config - apply the top configs to the initialized tool
//
    static int
    osnoise_top_apply_config(struct osnoise_tool *tool)
    {
    struct osnoise_params *params = to_osnoise_params(tool.params);
    int retval;
    retval = osnoise_apply_config(tool, params);
    if (retval)
    goto out_err;
    if (params.mode == MODE_HWNOISE) {
    retval = osnoise_set_irq_disable(tool.context, 1);
    if (retval) {
    err_msg("Failed to set OSNOISE_IRQ_DISABLE option\n");
    goto out_err;
    }
    }
    if (isatty(STDOUT_FILENO) && !params.common.quiet)
    params.common.pretty_output = 1;
    return 0;
    out_err:
    return -1;
    }
//
// osnoise_init_top - initialize a osnoise top tool with parameters
//
    struct osnoise_tool *osnoise_init_top(struct common_params *params)
    {
    struct osnoise_tool *tool;
    tool = osnoise_init_tool("osnoise_top");
    if (!tool)
    return core::ptr::null_mut();
    tool.data = osnoise_alloc_top();
    if (!tool.data) {
    osnoise_destroy_tool(tool);
    return core::ptr::null_mut();
    }
    tep_register_event_handler(tool.trace.tep, -1, "ftrace", "osnoise",
    osnoise_top_handler, core::ptr::null_mut());
    return tool;
    }
    struct tool_ops osnoise_top_ops = {
    .tracer = "osnoise",
    .comm_prefix = "osnoise/",
    .parse_args = osnoise_top_parse_args,
    .init_tool = osnoise_init_top,
    .apply_config = osnoise_top_apply_config,
    .enable = osnoise_enable,
    .main = top_main_loop,
    .print_stats = osnoise_print_stats,
    .free = osnoise_free_top_tool,
    };
