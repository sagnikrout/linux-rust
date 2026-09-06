//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/lib/ftrace-tcp.c
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

    static const char *trace_event_names[__MAX_TRACE_EVENTS] = {
// TCP_HASH_EVENT
    "tcp_hash_bad_header",
    "tcp_hash_md5_required",
    "tcp_hash_md5_unexpected",
    "tcp_hash_md5_mismatch",
    "tcp_hash_ao_required",
// TCP_AO_EVENT
    "tcp_ao_handshake_failure",
    "tcp_ao_wrong_maclen",
    "tcp_ao_mismatch",
    "tcp_ao_key_not_found",
    "tcp_ao_rnext_request",
// TCP_AO_EVENT_SK
    "tcp_ao_synack_no_key",
// TCP_AO_EVENT_SNE
    "tcp_ao_snd_sne_update",
    "tcp_ao_rcv_sne_update"
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expected_trace_point {
// required
    pub type: enum trace_events,
    pub family: c_int,
    pub src: union tcp_addr,
    pub dst: union tcp_addr,
// optional
    pub src_port: c_int,
    pub dst_port: c_int,
    pub L3index: c_int,
    pub fin: c_int,
    pub syn: c_int,
    pub rst: c_int,
    pub psh: c_int,
    pub ack: c_int,
    pub keyid: c_int,
    pub rnext: c_int,
    pub maclen: c_int,
    pub sne: c_int,
    pub matched: usize,
}

    static struct expected_trace_point *exp_tps;
    static size_t exp_tps_nr;
    static size_t exp_tps_size;
    let mut exp_tps_mutex: static pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
    int __trace_event_expect(enum trace_events type, int family,
    union tcp_addr src, union tcp_addr dst,
    int src_port, int dst_port, int L3index,
    int fin, int syn, int rst, int psh, int ack,
    int keyid, int rnext, int maclen, int sne)
    {
    struct expected_trace_point new_tp = {
    .type           = type,
    .family         = family,
    .src            = src,
    .dst            = dst,
    .src_port       = src_port,
    .dst_port       = dst_port,
    .L3index        = L3index,
    .fin            = fin,
    .syn            = syn,
    .rst            = rst,
    .psh            = psh,
    .ack            = ack,
    .keyid          = keyid,
    .rnext          = rnext,
    .maclen         = maclen,
    .sne            = sne,
    .matched        = 0,
    };
    let mut ret: c_int = 0;
    if (!kernel_config_has(KCONFIG_FTRACE))
    return 0;
    pthread_mutex_lock(&exp_tps_mutex);
    if (exp_tps_nr == exp_tps_size) {
    struct expected_trace_point *tmp;
    if (exp_tps_size == 0)
    exp_tps_size = 10;
    else
    exp_tps_size = exp_tps_size * 1.6;
    tmp = reallocarray(exp_tps, exp_tps_size, sizeof(exp_tps[0]));
    if (!tmp) {
    ret = -ENOMEM;
    goto out;
    }
    exp_tps = tmp;
    }
    exp_tps[exp_tps_nr] = new_tp;
    exp_tps_nr++;
    out:
    pthread_mutex_unlock(&exp_tps_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_expected_events() {
    static void free_expected_events(void)
    {
// We're from the process destructor - not taking the mutex
    exp_tps_size = 0;
    exp_tps = core::ptr::null_mut();
    free(exp_tps);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_point {
    pub family: c_int,
    pub src: union tcp_addr,
    pub dst: union tcp_addr,
    pub src_port: c_uint,
    pub dst_port: c_uint,
    pub L3index: c_int,
    unsigned int fin:1,
    syn:1,
    rst:1,
    psh:1,
    pub keyid: c_uint,
    pub rnext: c_uint,
    pub maclen: c_uint,
    pub sne: c_uint,
}

#[no_mangle]
unsafe extern "C" fn lookup_expected_event(event_type: c_int, e: *mut trace_point) -> bool {
    static bool lookup_expected_event(int event_type, struct trace_point *e)
    {
    size_t i;
    pthread_mutex_lock(&exp_tps_mutex);
    for (i = 0; i < exp_tps_nr; i++) {
    struct expected_trace_point *p = &exp_tps[i];
    size_t sk_size;
    if (p.type != event_type)
    continue;
    if (p.family != e.family)
    continue;
    if (p.family == AF_INET)
    sk_size = sizeof(p.src.a4);
    else
    sk_size = sizeof(p.src.a6);
    if (memcmp(&p.src, &e.src, sk_size))
    continue;
    if (memcmp(&p.dst, &e.dst, sk_size))
    continue;
    if (p.src_port >= 0 && p.src_port != e.src_port)
    continue;
    if (p.dst_port >= 0 && p.dst_port != e.dst_port)
    continue;
    if (p.L3index >= 0 && p.L3index != e.L3index)
    continue;
    if (p.fin >= 0 && p.fin != e.fin)
    continue;
    if (p.syn >= 0 && p.syn != e.syn)
    continue;
    if (p.rst >= 0 && p.rst != e.rst)
    continue;
    if (p.psh >= 0 && p.psh != e.psh)
    continue;
    if (p.ack >= 0 && p.ack != e.ack)
    continue;
    if (p.keyid >= 0 && p.keyid != e.keyid)
    continue;
    if (p.rnext >= 0 && p.rnext != e.rnext)
    continue;
    if (p.maclen >= 0 && p.maclen != e.maclen)
    continue;
    if (p.sne >= 0 && p.sne != e.sne)
    continue;
    p.matched++;
    pthread_mutex_unlock(&exp_tps_mutex);
    return true;
    }
    pthread_mutex_unlock(&exp_tps_mutex);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn check_event_type(line: *const c_char) -> c_int {
    static int check_event_type(const char *line)
    {
    size_t i;
//
// This should have been a set or hashmap, but it's a selftest,
// so... KISS.
//
    for (i = 0; i < __MAX_TRACE_EVENTS; i++) {
    if (!strncmp(trace_event_names[i], line, strlen(trace_event_names[i])))
    return i;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn event_has_flags(event: enum trace_events) -> bool {
    static bool event_has_flags(enum trace_events event)
    {
    switch (event) {
    case TCP_HASH_BAD_HEADER:
    case TCP_HASH_MD5_REQUIRED:
    case TCP_HASH_MD5_UNEXPECTED:
    case TCP_HASH_MD5_MISMATCH:
    case TCP_HASH_AO_REQUIRED:
    case TCP_AO_HANDSHAKE_FAILURE:
    case TCP_AO_WRONG_MACLEN:
    case TCP_AO_MISMATCH:
    case TCP_AO_KEY_NOT_FOUND:
    case TCP_AO_RNEXT_REQUEST:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn tracer_ip_split(family: c_int, src: *mut c_char, addr: *mut c_char, port: *mut c_char) -> c_int {
    static int tracer_ip_split(int family, char *src, char **addr, char **port)
    {
    char *p;
    if (family == AF_INET) {
// fomat is <addr>:port, i.e.: 10.0.254.1:7015
// addr = src;
    p = strchr(src, ':');
    if (!p) {
    test_print("Couldn't parse trace event addr:port %s", src);
    return -EINVAL;
    }
// p++ = '\0';
// port = p;
    return 0;
    }
    if (family != AF_INET6)
    return -EAFNOSUPPORT;
// format is [<addr>]:port, i.e.: [2001:db8:254::1]:7013
// addr = strchr(src, '[');
    p = strchr(src, ']');
    if (!p || !*addr) {
    test_print("Couldn't parse trace event [addr]:port %s", src);
    return -EINVAL;
    }
// addr = *addr + 1;      /* '['
// p++ = '\0';            /* ']'
    if (*p != ':') {
    test_print("Couldn't parse trace event :port %s", p);
    return -EINVAL;
    }
// p++ = '\0';            /* ':'
// port = p;
    return 0;
    }
    static int tracer_scan_address(int family, char *src,
    union tcp_addr *dst, unsigned int *port)
    {
    char *addr, *port_str;
    int ret;
    ret = tracer_ip_split(family, src, &addr, &port_str);
    if (ret)
    return ret;
    if (inet_pton(family, addr, dst) != 1) {
    test_print("Couldn't parse trace event addr %s", addr);
    return -EINVAL;
    }
    errno = 0;
// port = (unsigned int)strtoul(port_str, NULL, 10);
    if (errno != 0) {
    test_print("Couldn't parse trace event port %s", port_str);
    return -errno;
    }
    return 0;
    }
    static int tracer_scan_event(const char *line, enum trace_events event,
    struct trace_point *out)
    {
    char *src = core::ptr::null_mut(), *dst = core::ptr::null_mut(), *family = core::ptr::null_mut();
    char fin, syn, rst, psh, ack;
    int nr_matched, ret = 0;
    uint64_t netns_cookie;
    switch (event) {
    case TCP_HASH_BAD_HEADER:
    case TCP_HASH_MD5_REQUIRED:
    case TCP_HASH_MD5_UNEXPECTED:
    case TCP_HASH_MD5_MISMATCH:
    case TCP_HASH_AO_REQUIRED: {
    nr_matched = sscanf(line, "%*s net=%" PRIu64 " state%*s family=%ms src=%ms dest=%ms L3index=%d [%c%c%c%c%c]",
    &netns_cookie, &family,
    &src, &dst, &out.L3index,
    &fin, &syn, &rst, &psh, &ack);
    if (nr_matched != 10)
    test_print("Couldn't parse trace event, matched = %d/10",
    nr_matched);
    break;
    }
    case TCP_AO_HANDSHAKE_FAILURE:
    case TCP_AO_WRONG_MACLEN:
    case TCP_AO_MISMATCH:
    case TCP_AO_KEY_NOT_FOUND:
    case TCP_AO_RNEXT_REQUEST: {
    nr_matched = sscanf(line, "%*s net=%" PRIu64 " state%*s family=%ms src=%ms dest=%ms L3index=%d [%c%c%c%c%c] keyid=%u rnext=%u maclen=%u",
    &netns_cookie, &family,
    &src, &dst, &out.L3index,
    &fin, &syn, &rst, &psh, &ack,
    &out.keyid, &out.rnext, &out.maclen);
    if (nr_matched != 13)
    test_print("Couldn't parse trace event, matched = %d/13",
    nr_matched);
    break;
    }
    case TCP_AO_SYNACK_NO_KEY: {
    nr_matched = sscanf(line, "%*s net=%" PRIu64 " state%*s family=%ms src=%ms dest=%ms keyid=%u rnext=%u",
    &netns_cookie, &family,
    &src, &dst, &out.keyid, &out.rnext);
    if (nr_matched != 6)
    test_print("Couldn't parse trace event, matched = %d/6",
    nr_matched);
    break;
    }
    case TCP_AO_SND_SNE_UPDATE:
    case TCP_AO_RCV_SNE_UPDATE: {
    nr_matched = sscanf(line, "%*s net=%" PRIu64 " state%*s family=%ms src=%ms dest=%ms sne=%u",
    &netns_cookie, &family,
    &src, &dst, &out.sne);
    if (nr_matched != 5)
    test_print("Couldn't parse trace event, matched = %d/5",
    nr_matched);
    break;
    }
    default:
    return -1;
    }
    if (family) {
    if (!strcmp(family, "AF_INET")) {
    out.family = AF_INET;
    } else if (!strcmp(family, "AF_INET6")) {
    out.family = AF_INET6;
    } else {
    test_print("Couldn't parse trace event family %s", family);
    ret = -EINVAL;
    goto out_free;
    }
    }
    if (event_has_flags(event)) {
    out.fin = (fin == 'F');
    out.syn = (syn == 'S');
    out.rst = (rst == 'R');
    out.psh = (psh == 'P');
    out.ack = (ack == '.');
    if ((fin != 'F' && fin != ' ') ||
    (syn != 'S' && syn != ' ') ||
    (rst != 'R' && rst != ' ') ||
    (psh != 'P' && psh != ' ') ||
    (ack != '.' && ack != ' ')) {
    test_print("Couldn't parse trace event flags %c%c%c%c%c",
    fin, syn, rst, psh, ack);
    ret = -EINVAL;
    goto out_free;
    }
    }
    if (src && tracer_scan_address(out.family, src, &out.src, &out.src_port)) {
    ret = -EINVAL;
    goto out_free;
    }
    if (dst && tracer_scan_address(out.family, dst, &out.dst, &out.dst_port)) {
    ret = -EINVAL;
    goto out_free;
    }
    if (netns_cookie != ns_cookie1 && netns_cookie != ns_cookie2) {
    test_print("Net namespace filter for trace event didn't work: %" PRIu64 " != %" PRIu64 " OR %" PRIu64,
    netns_cookie, ns_cookie1, ns_cookie2);
    ret = -EINVAL;
    }
    out_free:
    free(src);
    free(dst);
    free(family);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aolib_tracer_process_event(line: *const c_char) -> enum ftracer_op {
    static enum ftracer_op aolib_tracer_process_event(const char *line)
    {
    let mut event_type: c_int = check_event_type(line);
    let mut tmp: trace_point = {};
    if (event_type < 0)
    return FTRACER_LINE_PRESERVE;
    if (tracer_scan_event(line, event_type, &tmp))
    return FTRACER_LINE_PRESERVE;
    return lookup_expected_event(event_type, &tmp) ?
    FTRACER_LINE_DISCARD : FTRACER_LINE_PRESERVE;
    }
#[no_mangle]
unsafe extern "C" fn dump_trace_event(e: *mut expected_trace_point) {
    static void dump_trace_event(struct expected_trace_point *e)
    {
    char src[INET6_ADDRSTRLEN], dst[INET6_ADDRSTRLEN];
    if (!inet_ntop(e.family, &e.src, src, INET6_ADDRSTRLEN))
    test_error("inet_ntop()");
    if (!inet_ntop(e.family, &e.dst, dst, INET6_ADDRSTRLEN))
    test_error("inet_ntop()");
    test_print("trace event filter %s [%s:%d => %s:%d, L3index %d, flags: %s%s%s%s%s, keyid: %d, rnext: %d, maclen: %d, sne: %d] = %zu",
    trace_event_names[e.type],
    src, e.src_port, dst, e.dst_port, e.L3index,
    e.fin ? "F" : "", e.syn ? "S" : "", e.rst ? "R" : "",
    e.psh ? "P" : "", e.ack ? "." : "",
    e.keyid, e.rnext, e.maclen, e.sne, e.matched);
    }
#[no_mangle]
unsafe extern "C" fn print_match_stats(unexpected_events: bool) {
    static void print_match_stats(bool unexpected_events)
    {
    size_t matches_per_type[__MAX_TRACE_EVENTS] = {};
    let mut expected_but_none: bool = false;
    size_t i, total_matched = 0;
    char *stat_line = core::ptr::null_mut();
    for (i = 0; i < exp_tps_nr; i++) {
    struct expected_trace_point *e = &exp_tps[i];
    total_matched += e.matched;
    matches_per_type[e.type] += e.matched;
    if (!e.matched)
    expected_but_none = true;
    }
    for (i = 0; i < __MAX_TRACE_EVENTS; i++) {
    if (!matches_per_type[i])
    continue;
    stat_line = test_sprintf("%s%s[%zu] ", stat_line ?: "",
    trace_event_names[i],
    matches_per_type[i]);
    if (!stat_line)
    test_error("test_sprintf()");
    }
    if (unexpected_events || expected_but_none) {
    for (i = 0; i < exp_tps_nr; i++)
    dump_trace_event(&exp_tps[i]);
    }
    if (unexpected_events)
    return;
    if (expected_but_none)
    test_fail("Some trace events were expected, but didn't occur");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: total_matched) -> else {
    else if (total_matched)
    test_ok("Trace events matched expectations: %zu %s",
    total_matched, stat_line);
    else
    test_ok("No unexpected trace events during the test run");
    }

    __test_print(__test_msg, fmt, ##__VA_ARGS__)
#[no_mangle]
unsafe extern "C" fn check_free_events(tracer: *mut test_ftracer) {
    static void check_free_events(struct test_ftracer *tracer)
    {
    const char **lines;
    size_t nr;
    if (!kernel_config_has(KCONFIG_FTRACE)) {
    test_skip("kernel config doesn't have ftrace - no checks");
    return;
    }
    nr = tracer_get_savedlines_nr(tracer);
    lines = tracer_get_savedlines(tracer);
    print_match_stats(!!nr);
    if (!nr)
    return;
    errno = 0;
    test_xfail("Trace events [%zu] were not expected:", nr);
    while (nr)
    dump_events("\t%s", lines[--nr]);
    }
#[no_mangle]
unsafe extern "C" fn setup_tcp_trace_events(tracer: *mut test_ftracer) -> c_int {
    static int setup_tcp_trace_events(struct test_ftracer *tracer)
    {
    char *filter;
    size_t i;
    int ret;
    filter = test_sprintf("net_cookie == %zu || net_cookie == %zu",
    ns_cookie1, ns_cookie2);
    if (!filter)
    return -ENOMEM;
    for (i = 0; i < __MAX_TRACE_EVENTS; i++) {
    char *event_name = test_sprintf("tcp/%s", trace_event_names[i]);
    if (!event_name) {
    ret = -ENOMEM;
    break;
    }
    ret = setup_trace_event(tracer, event_name, filter);
    free(event_name);
    if (ret)
    break;
    }
    free(filter);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aolib_tracer_destroy(tracer: *mut test_ftracer) {
    static void aolib_tracer_destroy(struct test_ftracer *tracer)
    {
    check_free_events(tracer);
    free_expected_events();
    }
#[no_mangle]
unsafe extern "C" fn aolib_tracer_expecting_more() -> bool {
    static bool aolib_tracer_expecting_more(void)
    {
    size_t i;
    for (i = 0; i < exp_tps_nr; i++)
    if (!exp_tps[i].matched)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_aolib_ftracer() -> c_int {
    int setup_aolib_ftracer(void)
    {
    struct test_ftracer *f;
    f = create_ftracer("aolib", aolib_tracer_process_event,
    aolib_tracer_destroy, aolib_tracer_expecting_more,
    DEFAULT_FTRACE_BUFFER_KB, DEFAULT_TRACER_LINES_ARR);
    if (!f)
    return -1;
    return setup_tcp_trace_events(f);
    }
