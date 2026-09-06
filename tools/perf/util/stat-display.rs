//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/stat-display.c
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


pub const MGROUP_LEN: c_int = 50;
pub const METRIC_LEN: c_int = 38;
pub const EVNAME_LEN: c_int = 32;
pub const COUNTS_LEN: c_int = 18;
pub const INTERVAL_LEN: c_int = 16;
pub const CGROUP_LEN: c_int = 16;
pub const COMM_LEN: c_int = 16;
pub const PID_LEN: c_int = 7;
pub const CPUS_LEN: c_int = 4;
    static int aggr_header_lens[] = {
    [AGGR_CORE] 	= 18,
    [AGGR_CACHE]	= 22,
    [AGGR_CLUSTER]	= 20,
    [AGGR_DIE] 	= 12,
    [AGGR_SOCKET] 	= 6,
    [AGGR_NODE] 	= 6,
    [AGGR_NONE] 	= 6,
    [AGGR_THREAD] 	= 16,
    [AGGR_GLOBAL] 	= 0,
    };
    static const char *aggr_header_csv[] = {
    [AGGR_CORE]	=	"core,ctrs,",
    [AGGR_CACHE]	=	"cache,ctrs,",
    [AGGR_CLUSTER]	=	"cluster,ctrs,",
    [AGGR_DIE]	=	"die,ctrs,",
    [AGGR_SOCKET]	=	"socket,ctrs,",
    [AGGR_NONE]	=	"cpu,",
    [AGGR_THREAD]	=	"comm-pid,",
    [AGGR_NODE]	=	"node,",
    [AGGR_GLOBAL]	=	""
    };
    static const char *aggr_header_std[] = {
    [AGGR_CORE] 	= 	"core",
    [AGGR_CACHE] 	= 	"cache",
    [AGGR_CLUSTER]	= 	"cluster",
    [AGGR_DIE] 	= 	"die",
    [AGGR_SOCKET] 	= 	"socket",
    [AGGR_NONE] 	= 	"cpu",
    [AGGR_THREAD] 	= 	"comm-pid",
    [AGGR_NODE] 	= 	"node",
    [AGGR_GLOBAL] 	=	""
    };
    const char *metric_threshold_classify__color(enum metric_threshold_classify thresh)
    {
    const char * const colors[] = {
    "", /* unknown */
    PERF_COLOR_RED,     /* bad */
    PERF_COLOR_MAGENTA, /* nearly bad */
    PERF_COLOR_YELLOW,  /* less good */
    PERF_COLOR_GREEN,   /* good */
    };
    static_assert(ARRAY_SIZE(colors) - 1  == METRIC_THRESHOLD_GOOD, "missing enum value");
    return colors[thresh];
    }
    static const char *metric_threshold_classify__str(enum metric_threshold_classify thresh)
    {
    const char * const strs[] = {
    "unknown",
    "bad",
    "nearly bad",
    "less good",
    "good",
    };
    static_assert(ARRAY_SIZE(strs) - 1  == METRIC_THRESHOLD_GOOD, "missing enum value");
    return strs[thresh];
    }
#[no_mangle]
unsafe extern "C" fn print_running_std(config: *mut perf_stat_config, run: u64, ena: u64) {
    static void print_running_std(struct perf_stat_config *config, u64 run, u64 ena)
    {
    if (run != ena)
    fprintf(config.output, "  (%.2f%%)", 100.0 * run / ena);
    }
#[no_mangle]
unsafe extern "C" fn print_running_csv(config: *mut perf_stat_config, run: u64, ena: u64) {
    static void print_running_csv(struct perf_stat_config *config, u64 run, u64 ena)
    {
    let mut enabled_percent: double = 100;
    if (run != ena)
    enabled_percent = 100 * run / ena;
    fprintf(config.output, "%s%" PRIu64 "%s%.2f",
    config.csv_sep, run, config.csv_sep, enabled_percent);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outstate {
// Std mode: insert a newline before the next metric
    pub newline: bool,
// JSON mode: track need for comma for a previous field or not
    pub first: bool,
// Num CSV separators remaining to pad out when not all fields are printed
    pub csv_col_pad: c_int,
//
// The following don't track state across fields, but are here as a shortcut to
// pass data to the print functions. The alternative would be to update the
// function signatures of the entire print stack to pass them through.
//
// Place to output to
    pub fh: *const *const FILE,
// Lines are timestamped in --interval-print mode
    pub timestamp: [c_char; 64],
// Num items aggregated in current line. See struct perf_stat_aggr.nr
    pub aggr_nr: c_int,
// Core/socket/die etc ID for the current line
    pub id: aggr_cpu_id,
// Event for current line
    pub evsel: *mut evsel,
// Cgroup for current line
    pub cgrp: *mut cgroup,
}

    static const char *json_sep(struct outstate *os)
    {
    const char *sep = os.first ? "" : ", ";
    os.first = false;
    return sep;
    }

#[no_mangle]
unsafe extern "C" fn print_running_json(os: *mut outstate, run: u64, ena: u64) {
    static void print_running_json(struct outstate *os, u64 run, u64 ena)
    {
    let mut enabled_percent: double = 100;
    if (run != ena)
    enabled_percent = 100 * run / ena;
    json_out(os, "\"event-runtime\" : %" PRIu64 ", \"pcnt-running\" : %.2f",
    run, enabled_percent);
    }
    static void print_running(struct perf_stat_config *config, struct outstate *os,
    u64 run, u64 ena, bool before_metric)
    {
    if (config.json_output) {
    if (before_metric)
    print_running_json(os, run, ena);
    } else if (config.csv_output) {
    if (before_metric)
    print_running_csv(config, run, ena);
    } else {
    if (!before_metric)
    print_running_std(config, run, ena);
    }
    }
    static void print_noise_pct_std(struct perf_stat_config *config,
    double pct)
    {
    if (pct)
    fprintf(config.output, "  ( +-%6.2f%% )", pct);
    }
    static void print_noise_pct_csv(struct perf_stat_config *config,
    double pct)
    {
    fprintf(config.output, "%s%.2f%%", config.csv_sep, pct);
    }
    static void print_noise_pct_json(struct outstate *os,
    double pct)
    {
    json_out(os, "\"variance\" : %.2f", pct);
    }
    static void print_noise_pct(struct perf_stat_config *config, struct outstate *os,
    double total, double avg, bool before_metric)
    {
    let mut pct: double = rel_stddev_stats(total, avg);
    if (config.json_output) {
    if (before_metric)
    print_noise_pct_json(os, pct);
    } else if (config.csv_output) {
    if (before_metric)
    print_noise_pct_csv(config, pct);
    } else {
    if (!before_metric)
    print_noise_pct_std(config, pct);
    }
    }
    static void print_noise(struct perf_stat_config *config, struct outstate *os,
    struct evsel *evsel, double avg, bool before_metric)
    {
    struct perf_stat_evsel *ps;
    if (config.run_count == 1)
    return;
    ps = evsel.stats;
    print_noise_pct(config, os, stddev_stats(&ps.res_stats), avg, before_metric);
    }
#[no_mangle]
unsafe extern "C" fn print_cgroup_std(config: *mut perf_stat_config, cgrp_name: *const c_char) {
    static void print_cgroup_std(struct perf_stat_config *config, const char *cgrp_name)
    {
    fprintf(config.output, " %-*s", CGROUP_LEN, cgrp_name);
    }
#[no_mangle]
unsafe extern "C" fn print_cgroup_csv(config: *mut perf_stat_config, cgrp_name: *const c_char) {
    static void print_cgroup_csv(struct perf_stat_config *config, const char *cgrp_name)
    {
    fprintf(config.output, "%s%s", config.csv_sep, cgrp_name);
    }
#[no_mangle]
unsafe extern "C" fn print_cgroup_json(os: *mut outstate, cgrp_name: *const c_char) {
    static void print_cgroup_json(struct outstate *os, const char *cgrp_name)
    {
    json_out(os, "\"cgroup\" : \"%s\"", cgrp_name);
    }
    static void print_cgroup(struct perf_stat_config *config, struct outstate *os,
    struct cgroup *cgrp)
    {
    if (nr_cgroups || config.cgroup_list) {
    const char *cgrp_name = cgrp ? cgrp.name  : "";
    if (config.json_output)
    print_cgroup_json(os, cgrp_name);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    print_cgroup_csv(config, cgrp_name);
    else
    print_cgroup_std(config, cgrp_name);
    }
    }
    static void print_aggr_id_std(struct perf_stat_config *config,
    struct evsel *evsel, struct aggr_cpu_id id, int aggr_nr)
    {
    FILE *output = config.output;
    let mut idx: c_int = config.aggr_mode;
    char buf[128];
    switch (config.aggr_mode) {
    case AGGR_CORE:
    snprintf(buf, sizeof(buf), "S%d-D%d-C%d", id.socket, id.die, id.core);
    break;
    case AGGR_CACHE:
    snprintf(buf, sizeof(buf), "S%d-D%d-L%d-ID%d",
    id.socket, id.die, id.cache_lvl, id.cache);
    break;
    case AGGR_CLUSTER:
    snprintf(buf, sizeof(buf), "S%d-D%d-CLS%d", id.socket, id.die, id.cluster);
    break;
    case AGGR_DIE:
    snprintf(buf, sizeof(buf), "S%d-D%d", id.socket, id.die);
    break;
    case AGGR_SOCKET:
    snprintf(buf, sizeof(buf), "S%d", id.socket);
    break;
    case AGGR_NODE:
    snprintf(buf, sizeof(buf), "N%d", id.node);
    break;
    case AGGR_NONE:
    if (evsel.percore && !config.percore_show_thread) {
    snprintf(buf, sizeof(buf), "S%d-D%d-C%d ",
    id.socket, id.die, id.core);
    fprintf(output, "%-*s ",
    aggr_header_lens[AGGR_CORE], buf);
    } else if (id.cpu.cpu > -1) {
    fprintf(output, "CPU%-*d ",
    aggr_header_lens[AGGR_NONE] - 3, id.cpu.cpu);
    }
    return;
    case AGGR_THREAD:
    fprintf(output, "%*s-%-*d ",
    COMM_LEN, perf_thread_map__comm(evsel.core.threads, id.thread_idx),
    PID_LEN, perf_thread_map__pid(evsel.core.threads, id.thread_idx));
    return;
    case AGGR_GLOBAL:
    case AGGR_UNSET:
    case AGGR_MAX:
    default:
    return;
    }
    fprintf(output, "%-*s %*d ", aggr_header_lens[idx], buf, /*strlen("ctrs")*/ 4, aggr_nr);
    }
    static void print_aggr_id_csv(struct perf_stat_config *config,
    struct evsel *evsel, struct aggr_cpu_id id, int aggr_nr)
    {
    FILE *output = config.output;
    const char *sep = config.csv_sep;
    switch (config.aggr_mode) {
    case AGGR_CORE:
    fprintf(output, "S%d-D%d-C%d%s%d%s",
    id.socket, id.die, id.core, sep, aggr_nr, sep);
    break;
    case AGGR_CACHE:
    fprintf(config.output, "S%d-D%d-L%d-ID%d%s%d%s",
    id.socket, id.die, id.cache_lvl, id.cache, sep, aggr_nr, sep);
    break;
    case AGGR_CLUSTER:
    fprintf(config.output, "S%d-D%d-CLS%d%s%d%s",
    id.socket, id.die, id.cluster, sep, aggr_nr, sep);
    break;
    case AGGR_DIE:
    fprintf(output, "S%d-D%d%s%d%s",
    id.socket, id.die, sep, aggr_nr, sep);
    break;
    case AGGR_SOCKET:
    fprintf(output, "S%d%s%d%s",
    id.socket, sep, aggr_nr, sep);
    break;
    case AGGR_NODE:
    fprintf(output, "N%d%s%d%s",
    id.node, sep, aggr_nr, sep);
    break;
    case AGGR_NONE:
    if (evsel.percore && !config.percore_show_thread) {
    fprintf(output, "S%d-D%d-C%d%s",
    id.socket, id.die, id.core, sep);
    } else if (id.cpu.cpu > -1) {
    fprintf(output, "CPU%d%s",
    id.cpu.cpu, sep);
    }
    break;
    case AGGR_THREAD:
    fprintf(output, "%s-%d%s",
    perf_thread_map__comm(evsel.core.threads, id.thread_idx),
    perf_thread_map__pid(evsel.core.threads, id.thread_idx),
    sep);
    break;
    case AGGR_GLOBAL:
    case AGGR_UNSET:
    case AGGR_MAX:
    default:
    break;
    }
    }
    static void print_aggr_id_json(struct perf_stat_config *config, struct outstate *os,
    struct evsel *evsel, struct aggr_cpu_id id, int aggr_nr)
    {
    switch (config.aggr_mode) {
    case AGGR_CORE:
    json_out(os, "\"core\" : \"S%d-D%d-C%d\", \"counters\" : %d",
    id.socket, id.die, id.core, aggr_nr);
    break;
    case AGGR_CACHE:
    json_out(os, "\"cache\" : \"S%d-D%d-L%d-ID%d\", \"counters\" : %d",
    id.socket, id.die, id.cache_lvl, id.cache, aggr_nr);
    break;
    case AGGR_CLUSTER:
    json_out(os, "\"cluster\" : \"S%d-D%d-CLS%d\", \"counters\" : %d",
    id.socket, id.die, id.cluster, aggr_nr);
    break;
    case AGGR_DIE:
    json_out(os, "\"die\" : \"S%d-D%d\", \"counters\" : %d",
    id.socket, id.die, aggr_nr);
    break;
    case AGGR_SOCKET:
    json_out(os, "\"socket\" : \"S%d\", \"counters\" : %d",
    id.socket, aggr_nr);
    break;
    case AGGR_NODE:
    json_out(os, "\"node\" : \"N%d\", \"counters\" : %d",
    id.node, aggr_nr);
    break;
    case AGGR_NONE:
    if (evsel.percore && !config.percore_show_thread) {
    json_out(os, "\"core\" : \"S%d-D%d-C%d\"",
    id.socket, id.die, id.core);
    } else if (id.cpu.cpu > -1) {
    json_out(os, "\"cpu\" : \"%d\"",
    id.cpu.cpu);
    }
    break;
    case AGGR_THREAD:
    json_out(os, "\"thread\" : \"%s-%d\"",
    perf_thread_map__comm(evsel.core.threads, id.thread_idx),
    perf_thread_map__pid(evsel.core.threads, id.thread_idx));
    break;
    case AGGR_GLOBAL:
    case AGGR_UNSET:
    case AGGR_MAX:
    default:
    break;
    }
    }
    static void aggr_printout(struct perf_stat_config *config, struct outstate *os,
    struct evsel *evsel, struct aggr_cpu_id id, int aggr_nr)
    {
    if (config.json_output)
    print_aggr_id_json(config, os, evsel, id, aggr_nr);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    print_aggr_id_csv(config, evsel, id, aggr_nr);
    else
    print_aggr_id_std(config, evsel, id, aggr_nr);
    }
    static void new_line_std(struct perf_stat_config *config __maybe_unused,
    void *ctx)
    {
    struct outstate *os = ctx;
    os.newline = true;
    }
    static inline void __new_line_std_csv(struct perf_stat_config *config,
    struct outstate *os)
    {
    fputc('\n', os.fh);
    if (config.interval)
    fputs(os.timestamp, os.fh);
    aggr_printout(config, os, os.evsel, os.id, os.aggr_nr);
    }
#[no_mangle]
pub unsafe extern "C" fn __new_line_std(config: *mut perf_stat_config, os: *mut outstate) {
    static inline void __new_line_std(struct perf_stat_config *config, struct outstate *os)
    {
    fprintf(os.fh, "%*s", COUNTS_LEN + EVNAME_LEN + config.unit_width + 2, "");
    }
    static void do_new_line_std(struct perf_stat_config *config,
    struct outstate *os)
    {
    __new_line_std_csv(config, os);
    if (config.aggr_mode == AGGR_NONE)
    fprintf(os.fh, "        ");
    __new_line_std(config, os);
    }
    static void print_metric_std(struct perf_stat_config *config,
    void *ctx, enum metric_threshold_classify thresh,
    const char *fmt, const char *unit, double val)
    {
    struct outstate *os = ctx;
    FILE *out = os.fh;
    int n;
    let mut newline: bool = os.newline;
    const char *color = metric_threshold_classify__color(thresh);
    os.newline = false;
    if (unit == core::ptr::null_mut() || fmt == core::ptr::null_mut()) {
    fprintf(out, "%-*s", METRIC_LEN, "");
    return;
    }
    if (newline)
    do_new_line_std(config, os);
    n = fprintf(out, " # ");
    if (color)
    n += color_fprintf(out, color, fmt, val);
    else
    n += fprintf(out, fmt, val);
    fprintf(out, " %-*s", METRIC_LEN - n - 1, unit);
    }
#[no_mangle]
unsafe extern "C" fn new_line_csv(config: *mut perf_stat_config, ctx: *mut c_void) {
    static void new_line_csv(struct perf_stat_config *config, void *ctx)
    {
    struct outstate *os = ctx;
    int i;
    __new_line_std_csv(config, os);
    for (i = 0; i < os.csv_col_pad; i++)
    fputs(config.csv_sep, os.fh);
    }
    static void print_metric_csv(struct perf_stat_config *config __maybe_unused,
    void *ctx,
    enum metric_threshold_classify thresh __maybe_unused,
    const char *fmt, const char *unit, double val)
    {
    struct outstate *os = ctx;
    FILE *out = os.fh;
    char buf[64], *vals, *ends;
    if (unit == core::ptr::null_mut() || fmt == core::ptr::null_mut()) {
    fprintf(out, "%s%s", config.csv_sep, config.csv_sep);
    return;
    }
    snprintf(buf, sizeof(buf), fmt, val);
    ends = vals = skip_spaces(buf);
    while (isdigit(*ends) || *ends == '.')
    ends++;
// ends = 0;
    fprintf(out, "%s%s%s%s", config.csv_sep, vals, config.csv_sep, skip_spaces(unit));
    }
    static void print_metric_json(struct perf_stat_config *config __maybe_unused,
    void *ctx,
    enum metric_threshold_classify thresh,
    const char *fmt __maybe_unused,
    const char *unit, double val)
    {
    struct outstate *os = ctx;
    FILE *out = os.fh;
    if (unit) {
    json_out(os, "\"metric-value\" : \"%f\", \"metric-unit\" : \"%s\"", val, unit);
    if (thresh != METRIC_THRESHOLD_UNKNOWN) {
    json_out(os, "\"metric-threshold\" : \"%s\"",
    metric_threshold_classify__str(thresh));
    }
    }
    if (!config.metric_only)
    fprintf(out, "}");
    }
#[no_mangle]
unsafe extern "C" fn new_line_json(config: *mut perf_stat_config, ctx: *mut c_void) {
    static void new_line_json(struct perf_stat_config *config, void *ctx)
    {
    struct outstate *os = ctx;
    fputs("\n{", os.fh);
    os.first = true;
    if (config.interval)
    json_out(os, "%s", os.timestamp);
    aggr_printout(config, os, os.evsel, os.id, os.aggr_nr);
    }
    static void print_metricgroup_header_json(struct perf_stat_config *config,
    void *ctx,
    const char *metricgroup_name)
    {
    if (!metricgroup_name)
    return;
    json_out((struct outstate *) ctx, "\"metricgroup\" : \"%s\"}", metricgroup_name);
    new_line_json(config, ctx);
    }
    static void print_metricgroup_header_csv(struct perf_stat_config *config,
    void *ctx,
    const char *metricgroup_name)
    {
    struct outstate *os = ctx;
    int i;
    if (!metricgroup_name) {
// Leave space for running and enabling
    for (i = 0; i < os.csv_col_pad - 2; i++)
    fputs(config.csv_sep, os.fh);
    return;
    }
    for (i = 0; i < os.csv_col_pad; i++)
    fputs(config.csv_sep, os.fh);
    fprintf(config.output, "%s", metricgroup_name);
    new_line_csv(config, ctx);
    }
    static void print_metricgroup_header_std(struct perf_stat_config *config,
    void *ctx,
    const char *metricgroup_name)
    {
    struct outstate *os = ctx;
    if (!metricgroup_name) {
    __new_line_std(config, os);
    return;
    }
    fprintf(config.output, " %*s", config.metric_only_len, metricgroup_name);
    }
    static void print_metric_only(struct perf_stat_config *config,
    void *ctx, enum metric_threshold_classify thresh,
    const char *fmt, const char *unit, double val)
    {
    struct outstate *os = ctx;
    FILE *out = os.fh;
    char str[1024];
    unsigned mlen;
    const char *color = metric_threshold_classify__color(thresh);
    int olen;
    if (!unit) {
    os.first = false;
    return;
    }
    mlen = max_t(unsigned, strlen(unit), config.metric_only_len);
    olen = snprintf(str, sizeof(str), fmt ?: "", val);
    color_snprintf(str, sizeof(str), color ?: "", fmt ?: "", val);
    fprintf(out, "%*s%s", max_t(int, mlen - olen, 1), "", str);
    os.first = false;
    }
    static void print_metric_only_csv(struct perf_stat_config *config __maybe_unused,
    void *ctx,
    enum metric_threshold_classify thresh __maybe_unused,
    const char *fmt,
    const char *unit __maybe_unused, double val)
    {
    struct outstate *os = ctx;
    FILE *out = os.fh;
    char buf[64], *vals, *ends;
    if (!unit)
    return;
    snprintf(buf, sizeof(buf), fmt ?: "", val);
    ends = vals = skip_spaces(buf);
    while (isdigit(*ends) || *ends == '.')
    ends++;
// ends = 0;
    fprintf(out, "%s%s", vals, config.csv_sep);
    os.first = false;
    }
    static void print_metric_only_json(struct perf_stat_config *config __maybe_unused,
    void *ctx,
    enum metric_threshold_classify thresh __maybe_unused,
    const char *fmt,
    const char *unit, double val)
    {
    struct outstate *os = ctx;
    char buf[64], *ends;
    const char *vals;
    if (!unit || !unit[0])
    return;
    snprintf(buf, sizeof(buf), fmt ?: "", val);
    vals = ends = skip_spaces(buf);
    while (isdigit(*ends) || *ends == '.')
    ends++;
// ends = 0;
    if (!vals[0])
    vals = "none";
    json_out(os, "\"%s\" : \"%s\"", unit, vals);
    }
    static void print_metric_header(struct perf_stat_config *config,
    void *ctx,
    enum metric_threshold_classify thresh __maybe_unused,
    const char *fmt __maybe_unused,
    const char *unit, double val __maybe_unused)
    {
    struct outstate *os = ctx;
// In case of iostat, print metric header for first root port only
    if (config.iostat_run &&
    os.evsel.priv != evlist__selected(os.evsel.evlist).priv)
    return;
    if (os.evsel.cgrp != os.cgrp)
    return;
    if (!unit)
    return;
    if (config.json_output)
    return;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    fprintf(os.fh, "%s%s", unit, config.csv_sep);
    else
    fprintf(os.fh, "%*s ", config.metric_only_len, unit);
    }
    static void print_counter_value_std(struct perf_stat_config *config,
    struct evsel *evsel, double avg, bool ok)
    {
    FILE *output = config.output;
    let mut sc: double = evsel.scale;
    const char *fmt;
    const char *bad_count = evsel.supported ? CNTR_NOT_COUNTED : CNTR_NOT_SUPPORTED;
    if (config.big_num)
    fmt = floor(sc) != sc ? "%'*.2f " : "%'*.0f ";
    else
    fmt = floor(sc) != sc ? "%*.2f " : "%*.0f ";
    if (ok)
    fprintf(output, fmt, COUNTS_LEN, avg);
    else
    fprintf(output, "%*s ", COUNTS_LEN, bad_count);
    if (evsel.unit)
    fprintf(output, "%-*s ", config.unit_width, evsel.unit);
    fprintf(output, "%-*s", EVNAME_LEN, evsel__name(evsel));
    }
    static void print_counter_value_csv(struct perf_stat_config *config,
    struct evsel *evsel, double avg, bool ok)
    {
    FILE *output = config.output;
    let mut sc: double = evsel.scale;
    const char *sep = config.csv_sep;
    const char *fmt = floor(sc) != sc ? "%.2f%s" : "%.0f%s";
    const char *bad_count = evsel.supported ? CNTR_NOT_COUNTED : CNTR_NOT_SUPPORTED;
    if (ok)
    fprintf(output, fmt, avg, sep);
    else
    fprintf(output, "%s%s", bad_count, sep);
    if (evsel.unit)
    fprintf(output, "%s%s", evsel.unit, sep);
    fprintf(output, "%s", evsel__name(evsel));
    }
    static void print_counter_value_json(struct outstate *os,
    struct evsel *evsel, double avg, bool ok)
    {
    const char *bad_count = evsel.supported ? CNTR_NOT_COUNTED : CNTR_NOT_SUPPORTED;
    if (ok)
    json_out(os, "\"counter-value\" : \"%f\"", avg);
    else
    json_out(os, "\"counter-value\" : \"%s\"", bad_count);
    if (evsel.unit)
    json_out(os, "\"unit\" : \"%s\"", evsel.unit);
    json_out(os, "\"event\" : \"%s\"", evsel__name(evsel));
    }
    static void print_counter_value(struct perf_stat_config *config, struct outstate *os,
    struct evsel *evsel, double avg, bool ok)
    {
    if (config.json_output)
    print_counter_value_json(os, evsel, avg, ok);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    print_counter_value_csv(config, evsel, avg, ok);
    else
    print_counter_value_std(config, evsel, avg, ok);
    }
    static void abs_printout(struct perf_stat_config *config,
    struct outstate *os,
    struct aggr_cpu_id id, int aggr_nr,
    struct evsel *evsel, double avg, bool ok)
    {
    aggr_printout(config, os, evsel, id, aggr_nr);
    print_counter_value(config, os, evsel, avg, ok);
    print_cgroup(config, os, evsel.cgrp);
    }
#[no_mangle]
unsafe extern "C" fn evlist__has_hybrid_pmus(evlist: *mut evlist) -> bool {
    static bool evlist__has_hybrid_pmus(struct evlist *evlist)
    {
    struct evsel *evsel;
    struct perf_pmu *last_core_pmu = core::ptr::null_mut();
    if (perf_pmus__num_core_pmus() == 1)
    return false;
    evlist__for_each_entry(evlist, evsel) {
    if (evsel.core.is_pmu_core) {
    struct perf_pmu *pmu = evsel__find_pmu(evsel);
    if (pmu == last_core_pmu)
    continue;
    if (last_core_pmu == core::ptr::null_mut()) {
    last_core_pmu = pmu;
    continue;
    }
// A distinct core PMU.
    return true;
    }
    }
    return false;
    }
    static void printout(struct perf_stat_config *config, struct outstate *os,
    double uval, u64 run, u64 ena, double noise, int aggr_idx)
    {
    struct perf_stat_output_ctx out;
    print_metric_t pm;
    new_line_t nl;
    print_metricgroup_header_t pmh;
    let mut ok: bool = true;
    struct evsel *counter = os.evsel;
    if (config.csv_output) {
    pm = config.metric_only ? print_metric_only_csv : print_metric_csv;
    nl = config.metric_only ? core::ptr::null_mut() : new_line_csv;
    pmh = print_metricgroup_header_csv;
    os.csv_col_pad = 4 + (counter.cgrp ? 1 : 0);
    } else if (config.json_output) {
    pm = config.metric_only ? print_metric_only_json : print_metric_json;
    nl = config.metric_only ? core::ptr::null_mut() : new_line_json;
    pmh = print_metricgroup_header_json;
    } else {
    pm = config.metric_only ? print_metric_only : print_metric_std;
    nl = config.metric_only ? core::ptr::null_mut() : new_line_std;
    pmh = print_metricgroup_header_std;
    }
    if (run == 0 || ena == 0 || counter.counts.scaled == -1) {
    ok = false;
    if (counter.supported) {
    if (!evlist__has_hybrid_pmus(counter.evlist) &&
    counter.pmu && counter.pmu.is_core)
    config.print_free_counters_hint = 1;
    }
    }
    out.print_metric = pm;
    out.new_line = nl;
    out.print_metricgroup_header = pmh;
    out.ctx = os;
    out.force_header = false;
    if (!config.metric_only && (!counter.default_metricgroup || counter.default_show_events)) {
    abs_printout(config, os, os.id, os.aggr_nr, counter, uval, ok);
    print_noise(config, os, counter, noise, /*before_metric=*/true);
    print_running(config, os, run, ena, /*before_metric=*/true);
    }
    if (!config.metric_only && counter.default_metricgroup &&
    !counter.default_show_events) {
    void *from = core::ptr::null_mut();
    aggr_printout(config, os, os.evsel, os.id, os.aggr_nr);
// Print out all the metricgroup with the same metric event.
    do {
    let mut num: c_int = 0;
// Print out the new line for the next new metricgroup.
    if (from) {
    if (config.json_output)
    new_line_json(config, (void *)os);
    else
    __new_line_std_csv(config, os);
    }
    print_noise(config, os, counter, noise,
// before_metric=*/true);
    print_running(config, os, run, ena,
// before_metric=*/true);
    from = perf_stat__print_shadow_stats_metricgroup(
    config, counter, aggr_idx, &num, from, &out);
    } while (from != core::ptr::null_mut());
    } else {
    perf_stat__print_shadow_stats(config, counter, aggr_idx, &out);
    }
    if (!config.metric_only) {
    print_noise(config, os, counter, noise, /*before_metric=*/false);
    print_running(config, os, run, ena, /*before_metric=*/false);
    }
    }
//
// should_skip_zero_count() - Check if the event should print 0 values.
// @config: The perf stat configuration (including aggregation mode).
// @counter: The evsel with its associated cpumap.
// @id: The aggregation id that is being queried.
//
// Due to mismatch between the event cpumap or thread-map and the
// aggregation mode, sometimes it'd iterate the counter with the map
// which does not contain any values.
//
// For example, uncore events have dedicated CPUs to manage them,
// result for other CPUs should be zero and skipped.
//
// Return: %true if the value should NOT be printed, %false if the value
// needs to be printed like "<not counted>" or "<not supported>".
//
    static bool should_skip_zero_counter(struct perf_stat_config *config,
    struct evsel *counter,
    const struct aggr_cpu_id *id)
    {
    struct perf_cpu cpu;
    unsigned int idx;
//
// Skip unsupported default events when not verbose. (default events
// are all marked 'skippable').
//
    if (verbose == 0 && counter.skippable && !counter.supported)
    return true;
// Metric only counts won't be displayed but the metric wants to be computed.
    if (config.metric_only)
    return false;
    if (config.hide_zero && counter.supported)
    return true;
//
// Skip value 0 when enabling --per-thread globally,
// otherwise it will have too many 0 output.
//
    if (config.aggr_mode == AGGR_THREAD && config.system_wide)
    return true;
//
// In per-thread mode the aggr_map and aggr_get_id functions may be
// NULL, assume all 0 values should be output in that case.
//
    if (!config.aggr_map || !config.aggr_get_id)
    return false;
//
// Tool events may be gathered on all logical CPUs, for example
// system_time, but for many the first index is the only one used, for
// example num_cores. Don't skip for the first index.
//
    if (evsel__is_tool(counter)) {
    struct aggr_cpu_id own_id =
    config.aggr_get_id(config, (struct perf_cpu){ .cpu = 0 });
    return !aggr_cpu_id__equal(id, &own_id);
    }
//
// Skip value 0 when the counter's cpumask doesn't match the given aggr
// id.
//
    perf_cpu_map__for_each_cpu(cpu, idx, counter.core.cpus) {
    let mut own_id: aggr_cpu_id = config.aggr_get_id(config, cpu);
    if (aggr_cpu_id__equal(id, &own_id))
    return false;
    }
    return true;
    }
    static void print_counter_aggrdata(struct perf_stat_config *config,
    struct evsel *counter, int aggr_idx,
    struct outstate *os)
    {
    FILE *output = config.output;
    u64 ena, run, val;
    double uval;
    struct perf_stat_evsel *ps = counter.stats;
    struct perf_stat_aggr *aggr = &ps.aggr[aggr_idx];
    let mut id: aggr_cpu_id = config.aggr_map.map[aggr_idx];
    let mut avg: double = aggr.counts.val;
    let mut metric_only: bool = config.metric_only;
    os.id = id;
    os.aggr_nr = aggr.nr;
    os.evsel = counter;
// Skip already merged uncore/hybrid events
    if (config.aggr_mode != AGGR_NONE) {
    if (evsel__is_hybrid(counter)) {
    if (config.hybrid_merge && counter.first_wildcard_match != core::ptr::null_mut())
    return;
    } else {
    if (counter.first_wildcard_match != core::ptr::null_mut())
    return;
    }
    }
    val = aggr.counts.val;
    ena = aggr.counts.ena;
    run = aggr.counts.run;
    if (perf_stat__skip_metric_event(counter))
    return;
    if (val == 0 && should_skip_zero_counter(config, counter, &id))
    return;
    if (!metric_only) {
    if (config.json_output) {
    os.first = true;
    fputc('{', output);
    }
    if (config.interval) {
    if (config.json_output)
    json_out(os, "%s", os.timestamp);
    else
    fprintf(output, "%s", os.timestamp);
    } else if (config.summary && config.csv_output &&
    !config.no_csv_summary)
    fprintf(output, "%s%s", "summary", config.csv_sep);
    }
    uval = val * counter.scale;
    printout(config, os, uval, run, ena, avg, aggr_idx);
    if (!metric_only)
    fputc('\n', output);
    }
    static void print_metric_begin(struct perf_stat_config *config,
    struct evlist *evlist,
    struct outstate *os, int aggr_idx)
    {
    struct perf_stat_aggr *aggr;
    struct aggr_cpu_id id;
    struct evsel *evsel;
    os.first = true;
    if (!config.metric_only)
    return;
    if (config.json_output)
    fputc('{', config.output);
    if (config.interval) {
    if (config.json_output)
    json_out(os, "%s", os.timestamp);
    else
    fprintf(config.output, "%s", os.timestamp);
    }
    evsel = evlist__first(evlist);
    id = config.aggr_map.map[aggr_idx];
    aggr = &evsel.stats.aggr[aggr_idx];
    aggr_printout(config, os, evsel, id, aggr.nr);
    print_cgroup(config, os, os.cgrp ? : evsel.cgrp);
    }
#[no_mangle]
unsafe extern "C" fn print_metric_end(config: *mut perf_stat_config, os: *mut outstate) {
    static void print_metric_end(struct perf_stat_config *config, struct outstate *os)
    {
    FILE *output = config.output;
    if (!config.metric_only)
    return;
    if (config.json_output) {
    if (os.first)
    fputs("\"metric-value\" : \"none\"", output);
    fputc('}', output);
    }
    fputc('\n', output);
    }
    static void print_aggr(struct perf_stat_config *config,
    struct evlist *evlist,
    struct outstate *os)
    {
    struct evsel *counter;
    int aggr_idx;
    if (!config.aggr_map || !config.aggr_get_id)
    return;
//
// With metric_only everything is on a single line.
// Without each counter has its own line.
//
    cpu_aggr_map__for_each_idx(aggr_idx, config.aggr_map) {
    print_metric_begin(config, evlist, os, aggr_idx);
    evlist__for_each_entry(evlist, counter) {
    print_counter_aggrdata(config, counter, aggr_idx, os);
    }
    print_metric_end(config, os);
    }
    }
    static void print_aggr_cgroup(struct perf_stat_config *config,
    struct evlist *evlist,
    struct outstate *os)
    {
    struct evsel *counter, *evsel;
    int aggr_idx;
    if (!config.aggr_map || !config.aggr_get_id)
    return;
    evlist__for_each_entry(evlist, evsel) {
    if (os.cgrp == evsel.cgrp)
    continue;
    os.cgrp = evsel.cgrp;
    cpu_aggr_map__for_each_idx(aggr_idx, config.aggr_map) {
    print_metric_begin(config, evlist, os, aggr_idx);
    evlist__for_each_entry(evlist, counter) {
    if (counter.cgrp != os.cgrp)
    continue;
    print_counter_aggrdata(config, counter, aggr_idx, os);
    }
    print_metric_end(config, os);
    }
    }
    }
    static void print_counter(struct perf_stat_config *config,
    struct evsel *counter, struct outstate *os)
    {
    int aggr_idx;
// AGGR_THREAD doesn't have config->aggr_get_id
    if (!config.aggr_map)
    return;
    cpu_aggr_map__for_each_idx(aggr_idx, config.aggr_map) {
    print_counter_aggrdata(config, counter, aggr_idx, os);
    }
    }
    static void print_no_aggr_metric(struct perf_stat_config *config,
    struct evlist *evlist,
    struct outstate *os)
    {
    unsigned int all_idx;
    struct perf_cpu cpu;
    perf_cpu_map__for_each_cpu(cpu, all_idx, evlist__core(evlist).user_requested_cpus) {
    struct evsel *counter;
    let mut first: bool = true;
    evlist__for_each_entry(evlist, counter) {
    u64 ena, run, val;
    double uval;
    struct perf_stat_evsel *ps = counter.stats;
    let mut aggr_idx: c_int = 0;
    if (!perf_cpu_map__has(evsel__cpus(counter), cpu))
    continue;
    cpu_aggr_map__for_each_idx(aggr_idx, config.aggr_map) {
    if (config.aggr_map.map[aggr_idx].cpu.cpu == cpu.cpu)
    break;
    }
    os.evsel = counter;
    os.id = aggr_cpu_id__cpu(cpu, /*data=*/core::ptr::null_mut());
    if (first) {
    print_metric_begin(config, evlist, os, aggr_idx);
    first = false;
    }
    val = ps.aggr[aggr_idx].counts.val;
    ena = ps.aggr[aggr_idx].counts.ena;
    run = ps.aggr[aggr_idx].counts.run;
    uval = val * counter.scale;
    printout(config, os, uval, run, ena, 1.0, aggr_idx);
    }
    if (!first)
    print_metric_end(config, os);
    }
    }
    static void print_metric_headers_std(struct perf_stat_config *config,
    bool no_indent)
    {
    fputc(' ', config.output);
    if (!no_indent) {
    let mut len: c_int = aggr_header_lens[config.aggr_mode];
    if (nr_cgroups || config.cgroup_list)
    len += CGROUP_LEN + 1;
    fprintf(config.output, "%*s", len, "");
    }
    }
    static void print_metric_headers_csv(struct perf_stat_config *config,
    bool no_indent __maybe_unused)
    {
    const char *p;
    if (config.interval)
    fprintf(config.output, "time%s", config.csv_sep);
    if (config.iostat_run)
    return;
    p = aggr_header_csv[config.aggr_mode];
    while (*p) {
    if (*p == ',')
    fputs(config.csv_sep, config.output);
    else
    fputc(*p, config.output);
    p++;
    }
    }
    static void print_metric_headers_json(struct perf_stat_config *config __maybe_unused,
    bool no_indent __maybe_unused)
    {
    }
    static void print_metric_headers(struct perf_stat_config *config,
    struct evlist *evlist, bool no_indent)
    {
    struct evsel *counter;
    struct outstate os = {
    .fh = config.output
    };
    struct perf_stat_output_ctx out = {
    .ctx = &os,
    .print_metric = print_metric_header,
    .new_line = core::ptr::null_mut(),
    .force_header = true,
    };
    if (config.json_output)
    print_metric_headers_json(config, no_indent);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    print_metric_headers_csv(config, no_indent);
    else
    print_metric_headers_std(config, no_indent);
    if (config.iostat_run)
    iostat_print_header_prefix(config);
    if (config.cgroup_list)
    os.cgrp = evlist__first(evlist).cgrp;
// Print metrics headers only
    evlist__for_each_entry(evlist, counter) {
    if (!config.iostat_run &&
    config.aggr_mode != AGGR_NONE && counter.metric_leader != counter)
    continue;
    os.evsel = counter;
    perf_stat__print_shadow_stats(config, counter, /*aggr_idx=*/0, &out);
    }
    if (!config.json_output)
    fputc('\n', config.output);
    }
    static void prepare_timestamp(struct perf_stat_config *config,
    struct outstate *os, struct timespec *ts)
    {
    if (config.iostat_run)
    return;
    if (config.json_output)
    scnprintf(os.timestamp, sizeof(os.timestamp), "\"interval\" : %lu.%09lu",
    (unsigned long) ts.tv_sec, ts.tv_nsec);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    scnprintf(os.timestamp, sizeof(os.timestamp), "%lu.%09lu%s",
    (unsigned long) ts.tv_sec, ts.tv_nsec, config.csv_sep);
    else
    scnprintf(os.timestamp, sizeof(os.timestamp), "%6lu.%09lu ",
    (unsigned long) ts.tv_sec, ts.tv_nsec);
    }
    static void print_header_interval_std(struct perf_stat_config *config,
    struct target *_target __maybe_unused,
    struct evlist *evlist,
    int argc __maybe_unused,
    const char **argv __maybe_unused)
    {
    FILE *output = config.output;
    switch (config.aggr_mode) {
    case AGGR_NODE:
    case AGGR_SOCKET:
    case AGGR_DIE:
    case AGGR_CLUSTER:
    case AGGR_CACHE:
    case AGGR_CORE:
    fprintf(output, "#%*s %-*s ctrs",
    INTERVAL_LEN - 1, "time",
    aggr_header_lens[config.aggr_mode],
    aggr_header_std[config.aggr_mode]);
    break;
    case AGGR_NONE:
    fprintf(output, "#%*s %-*s",
    INTERVAL_LEN - 1, "time",
    aggr_header_lens[config.aggr_mode],
    aggr_header_std[config.aggr_mode]);
    break;
    case AGGR_THREAD:
    fprintf(output, "#%*s %*s-%-*s",
    INTERVAL_LEN - 1, "time",
    COMM_LEN, "comm", PID_LEN, "pid");
    break;
    case AGGR_GLOBAL:
    default:
    if (!config.iostat_run)
    fprintf(output, "#%*s",
    INTERVAL_LEN - 1, "time");
    case AGGR_UNSET:
    case AGGR_MAX:
    break;
    }
    if (config.metric_only)
    print_metric_headers(config, evlist, true);
    else
    fprintf(output, " %*s %*s events\n",
    COUNTS_LEN, "counts", config.unit_width, "unit");
    }
    static void print_header_std(struct perf_stat_config *config,
    struct target *_target, struct evlist *evlist,
    int argc, const char **argv)
    {
    FILE *output = config.output;
    int i;
    fprintf(output, "\n");
    fprintf(output, " Performance counter stats for ");
    if (_target.bpf_str)
    fprintf(output, "\'BPF program(s) %s", _target.bpf_str);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: _target->system_wide) -> else {
    else if (_target.system_wide)
    fprintf(output, "\'system wide");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: _target->cpu_list) -> else {
    else if (_target.cpu_list)
    fprintf(output, "\'CPU(s) %s", _target.cpu_list);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !target__has_task(_target)) -> else {
    fprintf(output, "\'%s", argv ? argv[0] : "pipe");
    for (i = 1; argv && (i < argc); i++)
    fprintf(output, " %s", argv[i]);
    } else if (_target.pid)
    fprintf(output, "process id \'%s", _target.pid);
    else
    fprintf(output, "thread id \'%s", _target.tid);
    fprintf(output, "\'");
    if (config.run_count > 1)
    fprintf(output, " (%d runs)", config.run_count);
    fprintf(output, ":\n\n");
    if (config.metric_only)
    print_metric_headers(config, evlist, false);
    }
    static void print_header_csv(struct perf_stat_config *config,
    struct target *_target __maybe_unused,
    struct evlist *evlist,
    int argc __maybe_unused,
    const char **argv __maybe_unused)
    {
    if (config.metric_only)
    print_metric_headers(config, evlist, true);
    }
    static void print_header_json(struct perf_stat_config *config,
    struct target *_target __maybe_unused,
    struct evlist *evlist,
    int argc __maybe_unused,
    const char **argv __maybe_unused)
    {
    if (config.metric_only)
    print_metric_headers(config, evlist, true);
    }
    static void print_header(struct perf_stat_config *config,
    struct target *_target,
    struct evlist *evlist,
    int argc, const char **argv)
    {
    static int num_print_iv;
    fflush(stdout);
    if (config.interval_clear)
    puts(CONSOLE_CLEAR);
    if (num_print_iv == 0 || config.interval_clear) {
    if (config.json_output)
    print_header_json(config, _target, evlist, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->csv_output) -> else {
    else if (config.csv_output)
    print_header_csv(config, _target, evlist, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: config->interval) -> else {
    else if (config.interval)
    print_header_interval_std(config, _target, evlist, argc, argv);
    else
    print_header_std(config, _target, evlist, argc, argv);
    }
    if (num_print_iv++ == 25)
    num_print_iv = 0;
    }
#[no_mangle]
unsafe extern "C" fn print_table(config: *mut perf_stat_config, output: *mut FILE, avg: double) {
    static void print_table(struct perf_stat_config *config, FILE *output, double avg)
    {
    char tmp[64];
    int idx, indent = 0;
    scnprintf(tmp, 64, " %17.9f", avg);
    while (tmp[indent] == ' ')
    indent++;
    fprintf(output, "%*s# Table of individual measurements:\n", indent, "");
    for (idx = 0; idx < config.run_count; idx++) {
    let mut run: double = (double) config.walltime_run[idx] / NSEC_PER_SEC;
    int h, n = 1 + abs((int) (100.0 * (run - avg)/run) / 5);
    fprintf(output, " %17.9f (%+.9f) ", run, run - avg);
    for (h = 0; h < n; h++)
    fprintf(output, "#");
    fprintf(output, "\n");
    }
    fprintf(output, "\n%*s# Final result:\n", indent, "");
    }
#[no_mangle]
unsafe extern "C" fn timeval2double(t: *mut timeval) -> double {
    static double timeval2double(struct timeval *t)
    {
    return t.tv_sec + (double) t.tv_usec/USEC_PER_SEC;
    }
#[no_mangle]
unsafe extern "C" fn print_footer(config: *mut perf_stat_config) {
    static void print_footer(struct perf_stat_config *config)
    {
    let mut avg: double = avg_stats(config.walltime_nsecs_stats) / NSEC_PER_SEC;
    FILE *output = config.output;
    if (config.interval || config.csv_output || config.json_output)
    return;
    if (!config.null_run)
    fprintf(output, "\n");
    if (config.run_count == 1) {
    fprintf(output, " %17.9f seconds time elapsed", avg);
    if (config.ru_display) {
    let mut ru_utime: double = timeval2double(&config.ru_data.ru_utime);
    let mut ru_stime: double = timeval2double(&config.ru_data.ru_stime);
    fprintf(output, "\n\n");
    fprintf(output, " %17.9f seconds user\n", ru_utime);
    fprintf(output, " %17.9f seconds sys\n", ru_stime);
    }
    } else {
    let mut sd: double = stddev_stats(config.walltime_nsecs_stats) / NSEC_PER_SEC;
    if (config.walltime_run_table)
    print_table(config, output, avg);
    fprintf(output, " %17.9f +- %.9f seconds time elapsed", avg, sd);
    print_noise_pct(config, core::ptr::null_mut(), sd, avg, /*before_metric=*/false);
    }
    fprintf(output, "\n\n");
    if (config.print_free_counters_hint && sysctl__nmi_watchdog_enabled())
    fprintf(output,
    "Some events weren't counted. Try disabling the NMI watchdog:\n"
    "	echo 0 > /proc/sys/kernel/nmi_watchdog\n"
    "	perf stat ...\n"
    "	echo 1 > /proc/sys/kernel/nmi_watchdog\n");
    }
    static void print_percore(struct perf_stat_config *config,
    struct evsel *counter, struct outstate *os)
    {
    let mut metric_only: bool = config.metric_only;
    FILE *output = config.output;
    struct cpu_aggr_map *core_map;
    int aggr_idx, core_map_len = 0;
    if (!config.aggr_map || !config.aggr_get_id)
    return;
    if (config.percore_show_thread)
    return print_counter(config, counter, os);
//
// core_map will hold the aggr_cpu_id for the cores that have been
// printed so that each core is printed just once.
//
    core_map = cpu_aggr_map__empty_new(config.aggr_map.nr);
    if (core_map == core::ptr::null_mut()) {
    fprintf(output, "Cannot allocate per-core aggr map for display\n");
    return;
    }
    cpu_aggr_map__for_each_idx(aggr_idx, config.aggr_map) {
    let mut curr_cpu: perf_cpu = config.aggr_map.map[aggr_idx].cpu;
    let mut core_id: aggr_cpu_id = aggr_cpu_id__core(curr_cpu, core::ptr::null_mut());
    let mut found: bool = false;
    for (int i = 0; i < core_map_len; i++) {
    if (aggr_cpu_id__equal(&core_map.map[i], &core_id)) {
    found = true;
    break;
    }
    }
    if (found)
    continue;
    print_counter_aggrdata(config, counter, aggr_idx, os);
    core_map.map[core_map_len++] = core_id;
    }
    free(core_map);
    if (metric_only)
    fputc('\n', output);
    }
    static void print_cgroup_counter(struct perf_stat_config *config, struct evlist *evlist,
    struct outstate *os)
    {
    struct evsel *counter;
    evlist__for_each_entry(evlist, counter) {
    if (os.cgrp != counter.cgrp) {
    if (os.cgrp != core::ptr::null_mut())
    print_metric_end(config, os);
    os.cgrp = counter.cgrp;
    print_metric_begin(config, evlist, os, /*aggr_idx=*/0);
    }
    print_counter(config, counter, os);
    }
    if (os.cgrp)
    print_metric_end(config, os);
    }
    void evlist__print_counters(struct evlist *evlist, struct perf_stat_config *config,
    struct target *_target, struct timespec *ts,
    int argc, const char **argv)
    {
    let mut metric_only: bool = config.metric_only;
    struct evsel *counter;
    struct outstate os = {
    .fh = config.output,
    .first = true,
    };
    evlist__uniquify_evsel_names(evlist, config);
    if (config.iostat_run)
    evlist__set_selected(evlist, evlist__first(evlist));
    if (config.interval)
    prepare_timestamp(config, &os, ts);
    print_header(config, _target, evlist, argc, argv);
    switch (config.aggr_mode) {
    case AGGR_CORE:
    case AGGR_CACHE:
    case AGGR_CLUSTER:
    case AGGR_DIE:
    case AGGR_SOCKET:
    case AGGR_NODE:
    if (config.cgroup_list)
    print_aggr_cgroup(config, evlist, &os);
    else
    print_aggr(config, evlist, &os);
    break;
    case AGGR_THREAD:
    case AGGR_GLOBAL:
    if (config.iostat_run) {
    iostat_print_counters(evlist, config, ts, os.timestamp,
    (iostat_print_counter_t)print_counter, &os);
    } else if (config.cgroup_list) {
    print_cgroup_counter(config, evlist, &os);
    } else {
    print_metric_begin(config, evlist, &os, /*aggr_idx=*/0);
    evlist__for_each_entry(evlist, counter) {
    print_counter(config, counter, &os);
    }
    print_metric_end(config, &os);
    }
    break;
    case AGGR_NONE:
    if (metric_only)
    print_no_aggr_metric(config, evlist, &os);
    else {
    evlist__for_each_entry(evlist, counter) {
    if (counter.percore)
    print_percore(config, counter, &os);
    else
    print_counter(config, counter, &os);
    }
    }
    break;
    case AGGR_MAX:
    case AGGR_UNSET:
    default:
    break;
    }
    print_footer(config);
    fflush(config.output);
    }
