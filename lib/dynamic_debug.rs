//! Automatically rewritten from C to Rust
//! Source: lib/dynamic_debug.c
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
// lib/dynamic_debug.c
//
// make pr_debug()/dev_dbg() calls runtime configurable based upon their
// source module.
//
// Copyright (C) 2008 Jason Baron <jbaron@redhat.com>
// By Greg Banks <gnb@melbourne.sgi.com>
// Copyright (c) 2008 Silicon Graphics Inc.  All Rights Reserved.
// Copyright (C) 2011 Bart Van Assche.  All Rights Reserved.
// Copyright (C) 2013 Du, Changbin <changbin.du@gmail.com>
//

    extern struct _ddebug __start___dyndbg[];
    extern struct _ddebug __stop___dyndbg[];
    extern struct ddebug_class_map __start___dyndbg_classes[];
    extern struct ddebug_class_map __stop___dyndbg_classes[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddebug_table {
    pub maps: list_head link,,
    pub mod_name: *const c_char,
    pub num_ddebugs: c_uint,
    pub ddebugs: *mut _ddebug,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddebug_query {
    pub filename: *const c_char,
    pub module: *const c_char,
    pub function: *const c_char,
    pub format: *const c_char,
    pub class_string: *const c_char,
    pub last_lineno: unsigned int first_lineno,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddebug_iter {
    pub table: *mut ddebug_table,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flag_settings {
    pub flags: c_uint,
    pub mask: c_uint,
}

    static DEFINE_MUTEX(ddebug_lock);
    static LIST_HEAD(ddebug_tables);
    static int verbose;
    module_param(verbose, int, 0644);
    MODULE_PARM_DESC(verbose, " dynamic_debug/control processing "
    "( 0 = off (default), 1 = module add/rm, 2 = >control summary, 3 = parsing, 4 = per-site changes)");
// Return the path relative to source root
    static inline const char *trim_prefix(const char *path)
    {
    let mut skip: c_int = strlen(__FILE__) - strlen("lib/dynamic_debug.c");
    if (strncmp(path, __FILE__, skip))
    skip = 0; /* prefix mismatch, don't skip */
    return path + skip;
    }
    static const struct { unsigned flag:8; char opt_char; } opt_array[] = {
    { _DPRINTK_FLAGS_PRINT, 'p' },
    { _DPRINTK_FLAGS_INCL_MODNAME, 'm' },
    { _DPRINTK_FLAGS_INCL_FUNCNAME, 'f' },
    { _DPRINTK_FLAGS_INCL_SOURCENAME, 's' },
    { _DPRINTK_FLAGS_INCL_LINENO, 'l' },
    { _DPRINTK_FLAGS_INCL_TID, 't' },
    { _DPRINTK_FLAGS_INCL_STACK, 'd' },
    { _DPRINTK_FLAGS_NONE, '_' },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flagsbuf {
// format a string into buf[] which describes the _ddebug's flags
    static char *ddebug_describe_flags(unsigned int flags, struct flagsbuf *fb)
    {
    pub fb->buf: *mut *mut char p =,
    pub i: c_int,
    pub ++i): for (i = 0; i < ARRAY_SIZE(opt_array);,
    if (flags & opt_array[i].flag)
// p++ = opt_array[i].opt_char;
    if (p == fb.buf)
// p++ = '_';
// p = '\0';
    pub fb->buf: return,
    }

    do {								\
    if (verbose >= lvl)					\
    pub \: pr_info(fmt, ##__VA_ARGS__);,
    } while (0)

#[no_mangle]
unsafe extern "C" fn vpr_info_dq(query: *const ddebug_query, msg: *const c_char) {
    static void vpr_info_dq(const struct ddebug_query *query, const char *msg)
    {
// trim any trailing newlines
    pub 0: int fmtlen =,
    if (query.format) {
    pub strlen(query->format): fmtlen =,
    while (fmtlen && query.format[fmtlen - 1] == '\n')
    }
    v3pr_info("%s: func=\"%s\" file=\"%s\" module=\"%s\" format=\"%.*s\" lineno=%u-%u class=%s\n",
    msg,
    query.function ?: "",
    query.filename ?: "",
    query.module ?: "",
    fmtlen, query.format ?: "",
    pub query->class_string): query->first_lineno, query->last_lineno,,
    }
    static struct ddebug_class_map *ddebug_find_valid_class(struct ddebug_table const *dt,
    const char *class_string, int *class_id)
    {
    pub map: *mut ddebug_class_map,
    pub idx: c_int,
    list_for_each_entry(map, &dt.maps, link) {
    pub class_string): idx = match_string(map->class_names, map->length,,
    if (idx >= 0) {
// class_id = idx + map->base;
    pub map: return,
    }
    }
// class_id = -ENOENT;
    pub NULL: return,
    }

//
// Search the tables for _ddebug's which match the given `query' and
// apply the `flags' and `mask' to them.  Returns number of matching
// callsites, normally the same as number of changes.  If verbose,
// logs the changes.  Takes ddebug_lock.
//
    static int ddebug_change(const struct ddebug_query *query,
    struct flag_settings *modifiers)
    {
    pub i: c_int,
    pub dt: *mut ddebug_table,
    pub newflags: c_uint,
    pub 0: unsigned int nfound =,
    pub nbuf: flagsbuf fbuf,,
    pub NULL: *mut *mut ddebug_class_map map =,
    pub valid_class: int __outvar,
// search for matching ddebugs
    list_for_each_entry(dt, &ddebug_tables, link) {
// match against the module name
    if (query.module &&
    !match_wildcard(query.module, dt.mod_name))
    if (query.class_string) {
    pub &valid_class): map = ddebug_find_valid_class(dt, query->class_string,,
    if (!map)
    } else {
// constrain query, do not touch class'd callsites
    pub _DPRINTK_CLASS_DFLT: valid_class =,
    }
    pub {: for (i = 0; i < dt->num_ddebugs; i++),
    pub &dt->ddebugs[i]: *mut *mut _ddebug dp =,
// match site against query-class
    if (dp.class_id != valid_class)
// match against the source filename
    if (query.filename &&
    !match_wildcard(query.filename, dp.filename) &&
    !match_wildcard(query.filename,
    kbasename(dp.filename)) &&
    !match_wildcard(query.filename,
    trim_prefix(dp.filename)))
// match against the function
    if (query.function &&
    !match_wildcard(query.function, dp.function))
// match against the format
    if (query.format) {
    if (*query.format == '^') {
    pub p: *mut c_char,
// anchored search. match must be at beginning
    pub query->format+1): p = strstr(dp->format,,
    if (p != dp.format)
    } else if (!strstr(dp.format, query.format))
    }
// match against the line number range
    if (query.first_lineno &&
    dp.lineno < query.first_lineno)
    if (query.last_lineno &&
    dp.lineno > query.last_lineno)
    pub modifiers->flags: newflags = (dp->flags & modifiers->mask) |,
    if (newflags == dp.flags)

    if (dp.flags & _DPRINTK_FLAGS_PRINT) {
    if (!(newflags & _DPRINTK_FLAGS_PRINT))
    } else if (newflags & _DPRINTK_FLAGS_PRINT) {
    }

    v4pr_info("changed %s:%d [%s]%s %s => %s\n",
    trim_prefix(dp.filename), dp.lineno,
    dt.mod_name, dp.function,
    ddebug_describe_flags(dp.flags, &fbuf),
    pub &nbuf)): ddebug_describe_flags(newflags,,
    pub newflags: dp->flags =,
    }
    }
    if (!nfound && verbose)
    pub query\n"): pr_info("no matches for,
    pub nfound: return,
    }
//
// Split the buffer `buf' into space-separated words.
// Handles simple " and ' quoting, i.e. without nested,
// embedded or escaped \".  Return the number of words
// or <0 on error.
//
#[no_mangle]
unsafe extern "C" fn ddebug_tokenize(buf: *mut c_char, words[]: *mut c_char, maxwords: c_int) -> c_int {
    static int ddebug_tokenize(char *buf, char *words[], int maxwords)
    {
    pub 0: int nwords =,
    while (*buf) {
    pub end: *mut c_char,
// Skip leading whitespace
    pub skip_spaces(buf): buf =,
    if (!*buf)
    pub /: *mut *mut break; / oh, it was trailing whitespace,
    if (*buf == '#')
    pub /: *mut *mut break; / token starts comment, skip rest of line,
// find `end' of word, whitespace separated or quoted
    if (*buf == '"' || *buf == '\'') {
    pub buf++: *mut int quote =,
    pub end++): *mut *mut *mut for (end = buf; end && end != quote;,
    if (!*end) {
    pub buf): pr_err("unclosed quote: %s\n",,
    pub /: *mut *mut return -EINVAL; / unclosed quote,
    }
    } else {
    pub end++): *mut *mut *mut for (end = buf; end && !isspace(end);,
    if (end == buf) {
    pr_err("parse err after word:%d=%s\n", nwords,
    pub "<none>"): nwords ? words[nwords - 1] :,
    pub -EINVAL: return,
    }
    }
// `buf' is start of word, `end' is one past its end
    if (nwords == maxwords) {
    pub maxwords): pr_err("too many words, legal max <=%d\n",,
    pub /: *mut *mut return -EINVAL; / ran out of words[] before bytes,
    }
    if (*end)
// end++ = '\0';	/* terminate the word
    pub buf: words[nwords++] =,
    pub end: buf =,
    }
    if (verbose >= 3) {
    pub i: c_int,
    pub words:"): pr_info("split into,
    pub i++): for (i = 0; i < nwords;,
    pub words[i]): pr_cont(" \"%s\"",,
    }
    pub nwords: return,
    }
//
// Parse a single line number.  Note that the empty string ""
// is treated as a special case and converted to zero, which
// is later treated as a "don't care" value.
//
#[no_mangle]
pub unsafe extern "C" fn parse_lineno(str: *const c_char, val: *mut c_uint) -> c_int {
    static inline int parse_lineno(const char *str, unsigned int *val)
    {
    pub NULL): BUG_ON(str ==,
    if (*str == '\0') {
// val = 0;
    pub 0: return,
    }
    if (kstrtouint(str, 10, val) < 0) {
    pub str): pr_err("bad line-number: %s\n",,
    pub -EINVAL: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn parse_linerange(query: *mut ddebug_query, first: *const c_char) -> c_int {
    static int parse_linerange(struct ddebug_query *query, const char *first)
    {
    pub '-'): *mut *mut char last = strchr(first,,
    if (query.first_lineno || query.last_lineno) {
    pub 2x\n"): pr_err("match-spec: line used,
    pub -EINVAL: return,
    }
    if (last)
// last++ = '\0';
    if (parse_lineno(first, &query.first_lineno) < 0)
    pub -EINVAL: return,
    if (last) {
// range <first>-<last>
    if (parse_lineno(last, &query.last_lineno) < 0)
    pub -EINVAL: return,
// special case for last lineno not specified
    if (query.last_lineno == 0)
    pub UINT_MAX: query->last_lineno =,
    if (query.last_lineno < query.first_lineno) {
    pr_err("last-line:%d < 1st-line:%d\n",
    query.last_lineno,
    pub -EINVAL: return,
    }
    } else {
    pub query->first_lineno: query->last_lineno =,
    }
    v3pr_info("parsed line %d-%d\n", query.first_lineno,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn check_set(dest: *const c_char, src: *mut c_char, name: *mut c_char) -> c_int {
    static int check_set(const char **dest, char *src, char *name)
    {
    pub 0: int rc =,
    if (*dest) {
    pub -EINVAL: rc =,
    pr_err("match-spec:%s val:%s overridden by %s\n",
    pub src): *mut *mut name, dest,,
    }
// dest = src;
    pub rc: return,
    }
//
// Parse words[] as a ddebug query specification, which is a series
// of (keyword, value) pairs chosen from these possibilities:
//
// func <function-name>
// file <full-pathname>
// file <base-filename>
// module <module-name>
// format <escaped-string-to-find-in-format>
// line <lineno>
// line <first-lineno>-<last-lineno> // where either may be empty
//
// Only 1 of each type is allowed.
// Returns 0 on success, <0 on error.
//
    static int ddebug_parse_query(char *words[], int nwords,
    struct ddebug_query *query, const char *modname)
    {
    pub i: c_uint,
    pub 0: int rc =,
    pub fline: *mut c_char,
// check we have an even number of words
    if (nwords % 2 != 0) {
    pub <value>\n"): pr_err("expecting pairs of match-spec,
    pub -EINVAL: return,
    }
    pub {: for (i = 0; i < nwords; i += 2),
    pub words: [*mut *mut char keyword =; i],
    pub words: [*mut *mut char arg =; i+1],
    if (!strcmp(keyword, "func")) {
    pub "func"): rc = check_set(&query->function, arg,,
    } else if (!strcmp(keyword, "file")) {
    if (check_set(&query.filename, arg, "file"))
    pub -EINVAL: return,
// tail :$info is function or line-range
    pub ':'): fline = strchr(query->filename,,
    if (!fline)
// fline++ = '\0';
    if (isalpha(*fline) || *fline == '*' || *fline == '?') {
// take as function name
    if (check_set(&query.function, fline, "func"))
    pub -EINVAL: return,
    } else {
    if (parse_linerange(query, fline))
    pub -EINVAL: return,
    }
    } else if (!strcmp(keyword, "module")) {
    pub "module"): rc = check_set(&query->module, arg,,
    } else if (!strcmp(keyword, "format")) {
    string_unescape_inplace(arg, UNESCAPE_SPACE |
    UNESCAPE_OCTAL |
    pub "format"): rc = check_set(&query->format, arg,,
    } else if (!strcmp(keyword, "line")) {
    if (parse_linerange(query, arg))
    pub -EINVAL: return,
    } else if (!strcmp(keyword, "class")) {
    pub "class"): rc = check_set(&query->class_string, arg,,
    } else {
    pub keyword): pr_err("unknown keyword \"%s\"\n",,
    pub -EINVAL: return,
    }
    if (rc)
    pub rc: return,
    }
    if (!query.module && modname)
//
// support $modname.dyndbg=<multiple queries>, when
// not given in the query itself
//
    pub modname: query->module =,
    pub "parsed"): vpr_info_dq(query,,
    pub 0: return,
    }
//
// Parse `str' as a flags specification, format [-+=][p]+.
// Sets up *maskp and *flagsp to be used when changing the
// flags fields of matched _ddebug's.  Returns 0 on success
// or <0 on error.
//
#[no_mangle]
unsafe extern "C" fn ddebug_parse_flags(str: *const c_char, modifiers: *mut flag_settings) -> c_int {
    static int ddebug_parse_flags(const char *str, struct flag_settings *modifiers)
    {
    pub i: int op,,
    switch (*str) {
    case '+':
    case '-':
    case '=':
    pub str++: *mut op =,
    default:
    pub str): *mut *mut pr_err("bad flag-op %c, at start of %s\n", str,,
    pub -EINVAL: return,
    }
    pub op): v3pr_info("op='%c'\n",,
    pub {: *mut *mut for (; str ; ++str),
    pub {: for (i = ARRAY_SIZE(opt_array) - 1; i >= 0; i--),
    if (*str == opt_array[i].opt_char) {
    pub opt_array[i].flag: modifiers->flags |=,
    }
    }
    if (i < 0) {
    pub str): *mut pr_err("unknown flag '%c'\n",,
    pub -EINVAL: return,
    }
    }
    pub modifiers->flags): v3pr_info("flags=0x%x\n",,
// calculate final flags, mask based upon op
    switch (op) {
    case '=':
// modifiers->flags already set
    pub 0: modifiers->mask =,
    case '+':
    pub ~0U: modifiers->mask =,
    case '-':
    pub ~modifiers->flags: modifiers->mask =,
    pub 0: modifiers->flags =,
    }
    pub modifiers->mask): *mut *mut *mut v3pr_info("flagsp=0x%x maskp=0x%x\n", modifiers->flags,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ddebug_exec_query(query_string: *mut c_char, modname: *const c_char) -> c_int {
    static int ddebug_exec_query(char *query_string, const char *modname)
    {
    pub {}: flag_settings modifiers =,
    pub {}: ddebug_query query =,
pub const MAXWORDS: c_int = 9;
    pub nfound: int nwords,,
    pub words: [*mut c_char; MAXWORDS],
    pub MAXWORDS): nwords = ddebug_tokenize(query_string, words,,
    if (nwords <= 0) {
    pub failed\n"): pr_err("tokenize,
    pub -EINVAL: return,
    }
// check flags 1st (last arg) so query is pairs of spec,val
    if (ddebug_parse_flags(words[nwords-1], &modifiers)) {
    pub failed\n"): pr_err("flags parse,
    pub -EINVAL: return,
    }
    if (ddebug_parse_query(words, nwords-1, &query, modname)) {
    pub failed\n"): pr_err("query parse,
    pub -EINVAL: return,
    }
// actually go and implement the change
    pub &modifiers): nfound = ddebug_change(&query,,
    pub "no-match"): vpr_info_dq(&query, nfound ? "applied" :,
    pub nfound: return,
    }
// handle multiple queries in query string, continue on error, return
    last error or number of matching callsites.  Module name is either
    in param (for boot arg) or perhaps in query string.
//
#[no_mangle]
unsafe extern "C" fn ddebug_exec_queries(query: *mut c_char, modname: *const c_char) -> c_int {
    static int ddebug_exec_queries(char *query, const char *modname)
    {
    pub split: *mut c_char,
    pub 0: int i, errs = 0, exitcode = 0, rc, nfound =,
    pub {: for (i = 0; query; query = split),
    pub ";\n"): split = strpbrk(query,,
    if (split)
// split++ = '\0';
    pub skip_spaces(query): query =,
    if (!query || !*query || *query == '#')
    pub "*"): *mut vpr_info("query %d: \"%s\" mod:%s\n", i, query, modname ?:,
    pub modname): rc = ddebug_exec_query(query,,
    if (rc < 0) {
    pub rc: exitcode =,
    } else {
    pub rc: nfound +=,
    }
    }
    if (i)
    v2pr_info("processed %d queries, with %d matches, %d errs\n",
    pub errs): i, nfound,,
    if (exitcode)
    pub exitcode: return,
    pub nfound: return,
    }
// apply a new bitmap to the sys-knob's current bit-state
    static int ddebug_apply_class_bitmap(const struct ddebug_class_param *dcp,
    unsigned long *new_bits, unsigned long *old_bits)
    {
pub const QUERY_SIZE: c_int = 128;
    pub query: [c_char; QUERY_SIZE],
    pub dcp->map: *const *const ddebug_class_map map =,
    pub 0: int matches =,
    pub ct: int bi,,
    pub old_bits): *mut *mut v2pr_info("apply: 0x%lx to: 0x%lx\n", new_bits,,
    pub {: for (bi = 0; bi < map->length; bi++),
    if (test_bit(bi, new_bits) == test_bit(bi, old_bits))
    snprintf(query, QUERY_SIZE, "class %s %c%s", map.class_names[bi],
    pub dcp->flags): test_bit(bi, new_bits) ? '+' : '-',,
    pub NULL): ct = ddebug_exec_queries(query,,
    pub ct: matches +=,
    v2pr_info("bit_%d: %d matches on class: %s . 0x%lx\n", bi,
    pub new_bits): *mut ct, map->class_names[bi],,
    }
    pub matches: return,
    }
// stub to later conditionally add "$module." prefix where not already done

// accept comma-separated-list of [+-] classnames
#[no_mangle]
unsafe extern "C" fn param_set_dyndbg_classnames(instr: *const c_char, kp: *const kernel_param) -> c_int {
    static int param_set_dyndbg_classnames(const char *instr, const struct kernel_param *kp)
    {
    pub kp->arg: *const *const ddebug_class_param dcp =,
    pub dcp->map: *const *const ddebug_class_map map =,
    pub old_bits: unsigned long curr_bits,,
    pub tmp: *mut *mut *mut char cl_str, p,,
    pub 0: int cls_id, totct =,
    pub wanted: bool,
    pub GFP_KERNEL): cl_str = tmp = kstrdup_and_replace(instr, '\n', '\0',,
    if (!tmp)
    pub -ENOMEM: return,
// start with previously set state-bits, then modify
    pub dcp->bits: *mut curr_bits = old_bits =,
    pub curr_bits): vpr_info("\"%s\" > %s:0x%lx\n", cl_str, KP_NAME(kp),,
    pub {: for (; cl_str; cl_str = p),
    pub ','): p = strchr(cl_str,,
    if (p)
// p++ = '\0';
    if (*cl_str == '-') {
    pub false: wanted =,
    } else {
    pub true: wanted =,
    if (*cl_str == '+')
    }
    pub cl_str): cls_id = match_string(map->class_names, map->length,,
    if (cls_id < 0) {
    pub KP_NAME(kp)): pr_err("%s unknown to %s\n", cl_str,,
    }
// have one or more valid class_ids of one *_NAMES type
    switch (map.map_type) {
    case DD_CLASS_TYPE_DISJOINT_NAMES:
// the +/- pertains to a single bit
    if (test_bit(cls_id, &curr_bits) == wanted) {
    pub cl_str): v3pr_info("no change on %s\n",,
    }
    pub BIT(cls_id): curr_bits ^=,
    pub dcp->bits): totct += ddebug_apply_class_bitmap(dcp, &curr_bits,,
// dcp->bits = curr_bits;
    v2pr_info("%s: changed bit %d:%s\n", KP_NAME(kp), cls_id,
    case DD_CLASS_TYPE_LEVEL_NAMES:
// cls_id = N in 0..max. wanted +/- determines N or N-1
    pub CLASSMAP_BITMASK(*dcp->lvl): *mut old_bits =,
    pub )): curr_bits = CLASSMAP_BITMASK(cls_id + (wanted ? 1 : 0,
    pub &old_bits): totct += ddebug_apply_class_bitmap(dcp, &curr_bits,,
// dcp->lvl = (cls_id + (wanted ? 1 : 0));
    v2pr_info("%s: changed bit-%d: \"%s\" %lx.%lx\n", KP_NAME(kp), cls_id,
    pub curr_bits): map->class_names[cls_id], old_bits,,
    default:
    pub map->map_type): pr_err("illegal map-type value %d\n",,
    }
    }
    pub totct): vpr_info("total matches: %d\n",,
    pub 0: return,
    }
//
// param_set_dyndbg_classes - class FOO >control
// @instr: string echo>d to sysfs, input depends on map_type
// @kp:    kp->arg has state: bits/lvl, map, map_type
//
// Enable/disable prdbgs by their class, as given in the arguments to
// DECLARE_DYNDBG_CLASSMAP.  For LEVEL map-types, enforce relative
// levels by bitpos.
//
// Returns: 0 or <0 if error.
//
#[no_mangle]
pub unsafe extern "C" fn param_set_dyndbg_classes(instr: *const c_char, kp: *const kernel_param) -> c_int {
    int param_set_dyndbg_classes(const char *instr, const struct kernel_param *kp)
    {
    pub kp->arg: *const *const ddebug_class_param dcp =,
    pub dcp->map: *const *const ddebug_class_map map =,
    pub old_bits: unsigned long inrep, new_bits,,
    pub 0: int rc, totct =,
    switch (map.map_type) {
    case DD_CLASS_TYPE_DISJOINT_NAMES:
    case DD_CLASS_TYPE_LEVEL_NAMES:
// handle [+-]classnames list separately, we are done here
    pub kp): return param_set_dyndbg_classnames(instr,,
    case DD_CLASS_TYPE_DISJOINT_BITS:
    case DD_CLASS_TYPE_LEVEL_NUM:
// numeric input, accept and fall-thru
    pub &inrep): rc = kstrtoul(instr, 0,,
    if (rc) {
    pub KP_NAME(kp)): pr_err("expecting numeric input: %s > %s\n", instr,,
    pub -EINVAL: return,
    }
    default:
    pub map->map_type): pr_err("%s: bad map type: %d\n", KP_NAME(kp),,
    pub -EINVAL: return,
    }
// only _BITS,_NUM (numeric) map-types get here
    switch (map.map_type) {
    case DD_CLASS_TYPE_DISJOINT_BITS:
// expect bits. mask and warn if too many
    if (inrep & ~CLASSMAP_BITMASK(map.length)) {
    pr_warn("%s: input: 0x%lx exceeds mask: 0x%lx, masking\n",
    pub CLASSMAP_BITMASK(map->length)): KP_NAME(kp), inrep,,
    pub CLASSMAP_BITMASK(map->length): inrep &=,
    }
    pub KP_NAME(kp)): v2pr_info("bits:%lx > %s\n", inrep,,
    pub dcp->bits): totct += ddebug_apply_class_bitmap(dcp, &inrep,,
// dcp->bits = inrep;
    case DD_CLASS_TYPE_LEVEL_NUM:
// input is bitpos, of highest verbosity to be enabled
    if (inrep > map.length) {
    pr_warn("%s: level:%ld exceeds max:%d, clamping\n",
    pub map->length): KP_NAME(kp), inrep,,
    pub map->length: inrep =,
    }
    pub CLASSMAP_BITMASK(*dcp->lvl): *mut old_bits =,
    pub CLASSMAP_BITMASK(inrep): new_bits =,
    pub KP_NAME(kp)): v2pr_info("lvl:%ld bits:0x%lx > %s\n", inrep, new_bits,,
    pub &old_bits): totct += ddebug_apply_class_bitmap(dcp, &new_bits,,
// dcp->lvl = inrep;
    default:
    pub map->map_type): pr_warn("%s: bad map type: %d\n", KP_NAME(kp),,
    }
    pub totct): vpr_info("%s: total matches: %d\n", KP_NAME(kp),,
    pub 0: return,
    }
//
// param_get_dyndbg_classes - classes reader
// @buffer: string description of controlled bits -> classes
// @kp:     kp->arg has state: bits, map
//
// Reads last written state, underlying prdbg state may have been
// altered by direct >control.  Displays 0x for DISJOINT, 0-N for
// LEVEL Returns: #chars written or <0 on error
//
#[no_mangle]
pub unsafe extern "C" fn param_get_dyndbg_classes(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    int param_get_dyndbg_classes(char *buffer, const struct kernel_param *kp)
    {
    pub kp->arg: *const *const ddebug_class_param dcp =,
    pub dcp->map: *const *const ddebug_class_map map =,
    switch (map.map_type) {
    case DD_CLASS_TYPE_DISJOINT_NAMES:
    case DD_CLASS_TYPE_DISJOINT_BITS:
    pub dcp->bits): *mut return scnprintf(buffer, PAGE_SIZE, "0x%lx\n",,
    case DD_CLASS_TYPE_LEVEL_NAMES:
    case DD_CLASS_TYPE_LEVEL_NUM:
    pub dcp->lvl): *mut return scnprintf(buffer, PAGE_SIZE, "%d\n",,
    default:
    pub -1: return,
    }
    }
    const struct kernel_param_ops param_ops_dyndbg_classes = {
    .set = param_set_dyndbg_classes,
    .get = param_get_dyndbg_classes,
}

    EXPORT_SYMBOL(param_ops_dyndbg_classes);
pub const PREFIX_SIZE: c_int = 128;
#[no_mangle]
unsafe extern "C" fn remaining(wrote: c_int) -> c_int {
    static int remaining(int wrote)
    {
    if (PREFIX_SIZE - wrote > 0)
    return PREFIX_SIZE - wrote;
    return 0;
    }
    static char *__dynamic_emit_prefix(const struct _ddebug *desc, char *buf)
    {
    int pos_after_tid;
    let mut pos: c_int = 0;
    if (desc.flags & _DPRINTK_FLAGS_INCL_TID) {
    if (in_interrupt())
    pos += snprintf(buf + pos, remaining(pos), "<intr> ");
    else
    pos += snprintf(buf + pos, remaining(pos), "[%d] ",
    task_pid_vnr(current));
    }
    pos_after_tid = pos;
    if (desc.flags & _DPRINTK_FLAGS_INCL_MODNAME)
    pos += snprintf(buf + pos, remaining(pos), "%s:",
    desc.modname);
    if (desc.flags & _DPRINTK_FLAGS_INCL_FUNCNAME)
    pos += snprintf(buf + pos, remaining(pos), "%s:",
    desc.function);
    if (desc.flags & _DPRINTK_FLAGS_INCL_SOURCENAME)
    pos += snprintf(buf + pos, remaining(pos), "%s:",
    trim_prefix(desc.filename));
    if (desc.flags & _DPRINTK_FLAGS_INCL_LINENO)
    pos += snprintf(buf + pos, remaining(pos), "%d:",
    desc.lineno);
    if (pos - pos_after_tid)
    pos += snprintf(buf + pos, remaining(pos), " ");
    if (pos >= PREFIX_SIZE)
    buf[PREFIX_SIZE - 1] = '\0';
    return buf;
    }
    static inline char *dynamic_emit_prefix(struct _ddebug *desc, char *buf)
    {
    if (unlikely(desc.flags & _DPRINTK_FLAGS_INCL_ANY))
    return __dynamic_emit_prefix(desc, buf);
    return buf;
    }
#[no_mangle]
pub unsafe extern "C" fn __dynamic_pr_debug(descriptor: *mut _ddebug, fmt: *const c_char, ...) {
    void __dynamic_pr_debug(struct _ddebug *descriptor, const char *fmt, ...)
    {
    va_list args;
    struct va_format vaf;
    char buf[PREFIX_SIZE] = "";
    BUG_ON(!descriptor);
    BUG_ON(!fmt);
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    printk(KERN_DEBUG "%s%pV", dynamic_emit_prefix(descriptor, buf), &vaf);
    va_end(args);
    }
    EXPORT_SYMBOL(__dynamic_pr_debug);
    void __dynamic_dev_dbg(struct _ddebug *descriptor,
    const struct device *dev, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    BUG_ON(!descriptor);
    BUG_ON(!fmt);
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (!dev) {
    printk(KERN_DEBUG "(core::ptr::null_mut() device *): %pV", &vaf);
    } else {
    char buf[PREFIX_SIZE] = "";
    dev_printk_emit(LOGLEVEL_DEBUG, dev, "%s%s %s: %pV",
    dynamic_emit_prefix(descriptor, buf),
    dev_driver_string(dev), dev_name(dev),
    &vaf);
    }
    va_end(args);
    }
    EXPORT_SYMBOL(__dynamic_dev_dbg);

    void __dynamic_netdev_dbg(struct _ddebug *descriptor,
    const struct net_device *dev, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    BUG_ON(!descriptor);
    BUG_ON(!fmt);
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (dev && dev.dev.parent) {
    char buf[PREFIX_SIZE] = "";
    dev_printk_emit(LOGLEVEL_DEBUG, dev.dev.parent,
    "%s%s %s %s%s: %pV",
    dynamic_emit_prefix(descriptor, buf),
    dev_driver_string(dev.dev.parent),
    dev_name(dev.dev.parent),
    netdev_name(dev), netdev_reg_state(dev),
    &vaf);
    } else if (dev) {
    printk(KERN_DEBUG "%s%s: %pV", netdev_name(dev),
    netdev_reg_state(dev), &vaf);
    } else {
    printk(KERN_DEBUG "(core::ptr::null_mut() net_device): %pV", &vaf);
    }
    va_end(args);
    }
    EXPORT_SYMBOL(__dynamic_netdev_dbg);

    void __dynamic_ibdev_dbg(struct _ddebug *descriptor,
    const struct ib_device *ibdev, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (ibdev && ibdev.dev.parent) {
    char buf[PREFIX_SIZE] = "";
    dev_printk_emit(LOGLEVEL_DEBUG, ibdev.dev.parent,
    "%s%s %s %s: %pV",
    dynamic_emit_prefix(descriptor, buf),
    dev_driver_string(ibdev.dev.parent),
    dev_name(ibdev.dev.parent),
    dev_name(&ibdev.dev),
    &vaf);
    } else if (ibdev) {
    printk(KERN_DEBUG "%s: %pV", dev_name(&ibdev.dev), &vaf);
    } else {
    printk(KERN_DEBUG "(core::ptr::null_mut() ib_device): %pV", &vaf);
    }
    va_end(args);
    }
    EXPORT_SYMBOL(__dynamic_ibdev_dbg);

//
// Install a noop handler to make dyndbg look like a normal kernel cli param.
// This avoids warnings about dyndbg being an unknown cli param when supplied
// by a user.
//
#[no_mangle]
unsafe extern "C" fn dyndbg_setup(str: *mut c_char) -> __init int {
    static __init int dyndbg_setup(char *str)
    {
    return 1;
    }
    __setup("dyndbg=", dyndbg_setup);
//
// File_ops->write method for <debugfs>/dynamic_debug/control.  Gathers the
// command text from userspace, parses and executes it.
//
pub const USER_BUF_PAGE: c_int = 4096;
    static ssize_t ddebug_proc_write(struct file *file, const char __user *ubuf,
    size_t len, loff_t *offp)
    {
    char *tmpbuf;
    int ret;
    if (len == 0)
    return 0;
    if (len > USER_BUF_PAGE - 1) {
    pr_warn("expected <%d bytes into control\n", USER_BUF_PAGE);
    return -E2BIG;
    }
    tmpbuf = memdup_user_nul(ubuf, len);
    if (IS_ERR(tmpbuf))
    return PTR_ERR(tmpbuf);
    v2pr_info("read %zu bytes from userspace\n", len);
    ret = ddebug_exec_queries(tmpbuf, core::ptr::null_mut());
    kfree(tmpbuf);
    if (ret < 0)
    return ret;
// offp += len;
    return len;
    }
//
// Set the iterator to point to the first _ddebug object
// and return a pointer to that first object.  Returns
// NULL if there are no _ddebugs at all.
//
    static struct _ddebug *ddebug_iter_first(struct ddebug_iter *iter)
    {
    if (list_empty(&ddebug_tables)) {
    iter.table = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    iter.table = list_entry(ddebug_tables.next,
    struct ddebug_table, link);
    iter.idx = iter.table.num_ddebugs;
    return &iter.table.ddebugs[--iter.idx];
    }
//
// Advance the iterator to point to the next _ddebug
// object from the one the iterator currently points at,
// and returns a pointer to the new _ddebug.  Returns
// NULL if the iterator has seen all the _ddebugs.
//
    static struct _ddebug *ddebug_iter_next(struct ddebug_iter *iter)
    {
    if (iter.table == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (--iter.idx < 0) {
// iterate to next table
    if (list_is_last(&iter.table.link, &ddebug_tables)) {
    iter.table = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    iter.table = list_entry(iter.table.link.next,
    struct ddebug_table, link);
    iter.idx = iter.table.num_ddebugs;
    --iter.idx;
    }
    return &iter.table.ddebugs[iter.idx];
    }
//
// Seq_ops start method.  Called at the start of every
// read() call from userspace.  Takes the ddebug_lock and
// seeks the seq_file's iterator to the given position.
//
    static void *ddebug_proc_start(struct seq_file *m, loff_t *pos)
    {
    struct ddebug_iter *iter = m.private;
    struct _ddebug *dp;
    let mut n: c_int = *pos;
    mutex_lock(&ddebug_lock);
    if (!n)
    return SEQ_START_TOKEN;
    if (n < 0)
    return core::ptr::null_mut();
    dp = ddebug_iter_first(iter);
    while (dp != core::ptr::null_mut() && --n > 0)
    dp = ddebug_iter_next(iter);
    return dp;
    }
//
// Seq_ops next method.  Called several times within a read()
// call from userspace, with ddebug_lock held.  Walks to the
// next _ddebug object with a special case for the header line.
//
    static void *ddebug_proc_next(struct seq_file *m, void *p, loff_t *pos)
    {
    struct ddebug_iter *iter = m.private;
    struct _ddebug *dp;
    if (p == SEQ_START_TOKEN)
    dp = ddebug_iter_first(iter);
    else
    dp = ddebug_iter_next(iter);
    ++*pos;
    return dp;
    }

    (class_id >= map.base && class_id < map.base + map.length)
    static const char *ddebug_class_name(struct ddebug_iter *iter, struct _ddebug *dp)
    {
    struct ddebug_class_map *map;
    list_for_each_entry(map, &iter.table.maps, link)
    if (class_in_range(dp.class_id, map))
    return map.class_names[dp.class_id - map.base];
    return core::ptr::null_mut();
    }
//
// Seq_ops show method.  Called several times within a read()
// call from userspace, with ddebug_lock held.  Formats the
// current _ddebug as a single human-readable line, with a
// special case for the header line.
//
#[no_mangle]
unsafe extern "C" fn ddebug_proc_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int ddebug_proc_show(struct seq_file *m, void *p)
    {
    struct ddebug_iter *iter = m.private;
    struct _ddebug *dp = p;
    struct flagsbuf flags;
    char const *class;
    if (p == SEQ_START_TOKEN) {
    seq_puts(m,
    "# filename:lineno [module]function flags format\n");
    return 0;
    }
    seq_printf(m, "%s:%u [%s]%s =%s \"",
    trim_prefix(dp.filename), dp.lineno,
    iter.table.mod_name, dp.function,
    ddebug_describe_flags(dp.flags, &flags));
    seq_escape_str(m, dp.format, ESCAPE_SPACE, "\t\r\n\"");
    seq_putc(m, '"');
    if (dp.class_id != _DPRINTK_CLASS_DFLT) {
    class = ddebug_class_name(iter, dp);
    if (class)
    seq_printf(m, " class:%s", class);
    else
    seq_printf(m, " class unknown, _id:%d", dp.class_id);
    }
    seq_putc(m, '\n');
    return 0;
    }
//
// Seq_ops stop method.  Called at the end of each read()
// call from userspace.  Drops ddebug_lock.
//
#[no_mangle]
unsafe extern "C" fn ddebug_proc_stop(m: *mut seq_file, p: *mut c_void) {
    static void ddebug_proc_stop(struct seq_file *m, void *p)
    {
    mutex_unlock(&ddebug_lock);
    }
    static const struct seq_operations ddebug_proc_seqops = {
    .start = ddebug_proc_start,
    .next = ddebug_proc_next,
    .show = ddebug_proc_show,
    .stop = ddebug_proc_stop
    };
#[no_mangle]
unsafe extern "C" fn ddebug_proc_open(inode: *mut inode, file: *mut file) -> c_int {
    static int ddebug_proc_open(struct inode *inode, struct file *file)
    {
    return seq_open_private(file, &ddebug_proc_seqops,
    sizeof(struct ddebug_iter));
    }
    static const struct file_operations ddebug_proc_fops = {
    .owner = THIS_MODULE,
    .open = ddebug_proc_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = seq_release_private,
    .write = ddebug_proc_write
    };
    static const struct proc_ops proc_fops = {
    .proc_open = ddebug_proc_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = seq_release_private,
    .proc_write = ddebug_proc_write
    };
    static void ddebug_attach_module_classes(struct ddebug_table *dt,
    struct ddebug_class_map *classes,
    int num_classes)
    {
    struct ddebug_class_map *cm;
    int i, j, ct = 0;
    for (cm = classes, i = 0; i < num_classes; i++, cm++) {
    if (!strcmp(cm.mod_name, dt.mod_name)) {
    v2pr_info("class[%d]: module:%s base:%d len:%d ty:%d\n", i,
    cm.mod_name, cm.base, cm.length, cm.map_type);
    for (j = 0; j < cm.length; j++)
    v3pr_info(" %d: %d %s\n", j + cm.base, j,
    cm.class_names[j]);
    list_add(&cm.link, &dt.maps);
    ct++;
    }
    }
    if (ct)
    vpr_info("module:%s attached %d classes\n", dt.mod_name, ct);
    }
//
// Allocate a new ddebug_table for the given module
// and add it to the global list.
//
#[no_mangle]
unsafe extern "C" fn ddebug_add_module(di: *mut _ddebug_info, modname: *const c_char) -> c_int {
    static int ddebug_add_module(struct _ddebug_info *di, const char *modname)
    {
    struct ddebug_table *dt;
    v3pr_info("add-module: %s.%d sites\n", modname, di.num_descs);
    if (!di.num_descs) {
    v3pr_info(" skip %s\n", modname);
    return 0;
    }
    dt = kzalloc_obj(*dt);
    if (dt == core::ptr::null_mut()) {
    pr_err("error adding module: %s\n", modname);
    return -ENOMEM;
    }
//
// For built-in modules, name lives in .rodata and is
// immortal. For loaded modules, name points at the name[]
// member of struct module, which lives at least as long as
// this struct ddebug_table.
//
    dt.mod_name = modname;
    dt.ddebugs = di.descs;
    dt.num_ddebugs = di.num_descs;
    INIT_LIST_HEAD(&dt.link);
    INIT_LIST_HEAD(&dt.maps);
    if (di.classes && di.num_classes)
    ddebug_attach_module_classes(dt, di.classes, di.num_classes);
    mutex_lock(&ddebug_lock);
    list_add_tail(&dt.link, &ddebug_tables);
    mutex_unlock(&ddebug_lock);
    vpr_info("%3u debug prints in module %s\n", di.num_descs, modname);
    return 0;
    }
// helper for ddebug_dyndbg_(boot|module)_param_cb
    static int ddebug_dyndbg_param_cb(char *param, char *val,
    const char *modname, int on_err)
    {
    char *sep;
    sep = strchr(param, '.');
    if (sep) {
// needed only for ddebug_dyndbg_boot_param_cb
// sep = '\0';
    modname = param;
    param = sep + 1;
    }
    if (strcmp(param, "dyndbg"))
    return on_err; /* determined by caller */
    ddebug_exec_queries((val ? val : "+p"), modname);
    return 0; /* query failure shouldn't stop module load */
    }
// handle both dyndbg and $module.dyndbg params at boot
    static int ddebug_dyndbg_boot_param_cb(char *param, char *val,
    const char *unused, void *arg)
    {
    vpr_info("%s=\"%s\"\n", param, val);
    return ddebug_dyndbg_param_cb(param, val, core::ptr::null_mut(), 0);
    }
//
// modprobe foo finds foo.params in boot-args, strips "foo.", and
// passes them to load_module().  This callback gets unknown params,
// processes dyndbg params, rejects others.
//
#[no_mangle]
pub unsafe extern "C" fn ddebug_dyndbg_module_param_cb(param: *mut c_char, val: *mut c_char, module: *const c_char) -> c_int {
    int ddebug_dyndbg_module_param_cb(char *param, char *val, const char *module)
    {
    vpr_info("module: %s %s=\"%s\"\n", module, param, val);
    return ddebug_dyndbg_param_cb(param, val, module, -ENOENT);
    }
#[no_mangle]
unsafe extern "C" fn ddebug_table_free(dt: *mut ddebug_table) {
    static void ddebug_table_free(struct ddebug_table *dt)
    {
    list_del_init(&dt.link);
    kfree(dt);
    }

//
// Called in response to a module being unloaded.  Removes
// any ddebug_table's which point at the module.
//
#[no_mangle]
unsafe extern "C" fn ddebug_remove_module(mod_name: *const c_char) -> c_int {
    static int ddebug_remove_module(const char *mod_name)
    {
    struct ddebug_table *dt, *nextdt;
    let mut ret: c_int = -ENOENT;
    mutex_lock(&ddebug_lock);
    list_for_each_entry_safe(dt, nextdt, &ddebug_tables, link) {
    if (dt.mod_name == mod_name) {
    ddebug_table_free(dt);
    ret = 0;
    break;
    }
    }
    mutex_unlock(&ddebug_lock);
    if (!ret)
    v2pr_info("removed module \"%s\"\n", mod_name);
    return ret;
    }
    static int ddebug_module_notify(struct notifier_block *self, unsigned long val,
    void *data)
    {
    struct module *mod = data;
    let mut ret: c_int = 0;
    switch (val) {
    case MODULE_STATE_COMING:
    ret = ddebug_add_module(&mod.dyndbg_info, mod.name);
    if (ret)
    WARN(1, "Failed to allocate memory: dyndbg may not work properly.\n");
    break;
    case MODULE_STATE_GOING:
    ddebug_remove_module(mod.name);
    break;
    }
    return notifier_from_errno(ret);
    }
    static struct notifier_block ddebug_module_nb = {
    .notifier_call = ddebug_module_notify,
    .priority = 0, /* dynamic debug depends on jump label */
    };

#[no_mangle]
unsafe extern "C" fn ddebug_remove_all_tables() {
    static void ddebug_remove_all_tables(void)
    {
    mutex_lock(&ddebug_lock);
    while (!list_empty(&ddebug_tables)) {
    struct ddebug_table *dt = list_entry(ddebug_tables.next,
    struct ddebug_table,
    link);
    ddebug_table_free(dt);
    }
    mutex_unlock(&ddebug_lock);
    }
    static __initdata int ddebug_init_success;
#[no_mangle]
unsafe extern "C" fn dynamic_debug_init_control() -> int __init {
    static int __init dynamic_debug_init_control(void)
    {
    struct proc_dir_entry *procfs_dir;
    struct dentry *debugfs_dir;
    if (!ddebug_init_success)
    return -ENODEV;
// Create the control file in debugfs if it is enabled
    if (debugfs_initialized()) {
    debugfs_dir = debugfs_create_dir("dynamic_debug", core::ptr::null_mut());
    debugfs_create_file("control", 0644, debugfs_dir, core::ptr::null_mut(),
    &ddebug_proc_fops);
    }
// Also create the control file in procfs
    procfs_dir = proc_mkdir("dynamic_debug", core::ptr::null_mut());
    if (procfs_dir)
    proc_create("control", 0644, procfs_dir, &proc_fops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dynamic_debug_init() -> int __init {
    static int __init dynamic_debug_init(void)
    {
    struct _ddebug *iter, *iter_mod_start;
    int ret, i, mod_sites, mod_ct;
    const char *modname;
    char *cmdline;
    struct _ddebug_info di = {
    .descs = __start___dyndbg,
    .classes = __start___dyndbg_classes,
    .num_descs = __stop___dyndbg - __start___dyndbg,
    .num_classes = __stop___dyndbg_classes - __start___dyndbg_classes,
    };

    ret = register_module_notifier(&ddebug_module_nb);
    if (ret) {
    pr_warn("Failed to register dynamic debug module notifier\n");
    return ret;
    }

    if (&__start___dyndbg == &__stop___dyndbg) {
    if (IS_ENABLED(CONFIG_DYNAMIC_DEBUG)) {
    pr_warn("_ddebug table is empty in a CONFIG_DYNAMIC_DEBUG build\n");
    return 1;
    }
    pr_info("Ignore empty _ddebug table in a CONFIG_DYNAMIC_DEBUG_CORE build\n");
    ddebug_init_success = 1;
    return 0;
    }
    iter = iter_mod_start = __start___dyndbg;
    modname = iter.modname;
    i = mod_sites = mod_ct = 0;
    for (; iter < __stop___dyndbg; iter++, i++, mod_sites++) {
    if (strcmp(modname, iter.modname)) {
    mod_ct++;
    di.num_descs = mod_sites;
    di.descs = iter_mod_start;
    ret = ddebug_add_module(&di, modname);
    if (ret)
    goto out_err;
    mod_sites = 0;
    modname = iter.modname;
    iter_mod_start = iter;
    }
    }
    di.num_descs = mod_sites;
    di.descs = iter_mod_start;
    ret = ddebug_add_module(&di, modname);
    if (ret)
    goto out_err;
    ddebug_init_success = 1;
    vpr_info("%d prdebugs in %d modules, %d KiB in ddebug tables, %d kiB in __dyndbg section\n",
    i, mod_ct, (int)((mod_ct * sizeof(struct ddebug_table)) >> 10),
    (int)((i * sizeof(struct _ddebug)) >> 10));
    if (di.num_classes)
    v2pr_info("  %d builtin ddebug class-maps\n", di.num_classes);
// now that ddebug tables are loaded, process all boot args
// again to find and activate queries given in dyndbg params.
// While this has already been done for known boot params, it
// ignored the unknown ones (dyndbg in particular).  Reusing
// parse_args avoids ad-hoc parsing.  This will also attempt
// to activate queries for not-yet-loaded modules, which is
// slightly noisy if verbose, but harmless.
//
    cmdline = kstrdup(saved_command_line, GFP_KERNEL);
    parse_args("dyndbg params", cmdline, core::ptr::null_mut(),
    0, 0, 0, core::ptr::null_mut(), &ddebug_dyndbg_boot_param_cb);
    kfree(cmdline);
    return 0;
    out_err:
    ddebug_remove_all_tables();
    return 0;
    }
// Allow early initialization for boot messages via boot param
    early_initcall(dynamic_debug_init);
// Debugfs setup must be done later
    fs_initcall(dynamic_debug_init_control);
