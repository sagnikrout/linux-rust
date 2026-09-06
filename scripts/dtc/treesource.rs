//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/treesource.c
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
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
//

    extern FILE *yyin;
    extern int yyparse(void);
    extern YYLTYPE yylloc;
    struct dt_info *parser_output;
    bool treesource_error;
    struct dt_info *dt_from_source(const char *fname)
    {
    parser_output = core::ptr::null_mut();
    treesource_error = false;
    srcfile_push(fname);
    yyin = current_srcfile.f;
    yylloc.file = current_srcfile;
    if (yyparse() != 0)
    die("Unable to parse input tree\n");
    if (treesource_error)
    die("Syntax error parsing input tree\n");
    return parser_output;
    }
#[no_mangle]
unsafe extern "C" fn write_prefix(f: *mut FILE, level: c_int) {
    static void write_prefix(FILE *f, int level)
    {
    int i;
    for (i = 0; i < level; i++)
    fputc('\t', f);
    }
#[no_mangle]
unsafe extern "C" fn isstring(c: c_char) -> bool {
    static bool isstring(char c)
    {
    return (isprint((unsigned char)c)
    || (c == '\0')
    || strchr("\a\b\t\n\v\f\r", c));
    }
#[no_mangle]
unsafe extern "C" fn write_propval_string(f: *mut FILE, s: *const c_char, len: usize) {
    static void write_propval_string(FILE *f, const char *s, size_t len)
    {
    const char *end = s + len - 1;
    if (!len)
    return;
    assert(*end == '\0');
    fprintf(f, "\"");
    while (s < end) {
    let mut c: c_char = *s++;
    switch (c) {
    case '\a':
    fprintf(f, "\\a");
    break;
    case '\b':
    fprintf(f, "\\b");
    break;
    case '\t':
    fprintf(f, "\\t");
    break;
    case '\n':
    fprintf(f, "\\n");
    break;
    case '\v':
    fprintf(f, "\\v");
    break;
    case '\f':
    fprintf(f, "\\f");
    break;
    case '\r':
    fprintf(f, "\\r");
    break;
    case '\\':
    fprintf(f, "\\\\");
    break;
    case '\"':
    fprintf(f, "\\\"");
    break;
    case '\0':
    fprintf(f, "\\0");
    break;
    default:
    if (isprint((unsigned char)c))
    fprintf(f, "%c", c);
    else
    fprintf(f, "\\x%02"PRIx8, c);
    }
    }
    fprintf(f, "\"");
    }
#[no_mangle]
unsafe extern "C" fn write_propval_int(f: *mut FILE, p: *const c_char, len: usize, width: usize) {
    static void write_propval_int(FILE *f, const char *p, size_t len, size_t width)
    {
    const char *end = p + len;
    assert(len % width == 0);
    for (; p < end; p += width) {
    switch (width) {
    case 1:
    fprintf(f, "%02"PRIx8, *(const uint8_t*)p);
    break;
    case 2:
    fprintf(f, "0x%02"PRIx16, dtb_ld16(p));
    break;
    case 4:
    fprintf(f, "0x%02"PRIx32, dtb_ld32(p));
    break;
    case 8:
    fprintf(f, "0x%02"PRIx64, dtb_ld64(p));
    break;
    }
    if (p + width < end)
    fputc(' ', f);
    }
    }
    static const char *delim_start[] = {
    [TYPE_UINT8] = "[",
    [TYPE_UINT16] = "/bits/ 16 <",
    [TYPE_UINT32] = "<",
    [TYPE_UINT64] = "/bits/ 64 <",
    [TYPE_STRING] = "",
    };
    static const char *delim_end[] = {
    [TYPE_UINT8] = "]",
    [TYPE_UINT16] = ">",
    [TYPE_UINT32] = ">",
    [TYPE_UINT64] = ">",
    [TYPE_STRING] = "",
    };
//
// The invariants in the marker list are:
// - offsets are non-strictly monotonically increasing
// - for a single offset there is at most one type marker
// - for a single offset that has both a type marker and non-type markers, the
// type marker appears before the others.
//
    static struct marker **add_marker(struct marker **mi,
    enum markertype type, unsigned int offset, char *ref)
    {
    struct marker *nm;
    while (*mi && (*mi).offset < offset)
    mi = &(*mi).next;
    if (*mi && (*mi).offset == offset && is_type_marker((*mi).type)) {
    if (is_type_marker(type))
    return mi;
    mi = &(*mi).next;
    }
    if (*mi && (*mi).offset == offset && type == (*mi).type)
    return mi;
    nm = xmalloc(sizeof(*nm));
    nm.type = type;
    nm.offset = offset;
    nm.ref = ref;
    nm.next = *mi;
// mi = nm;
    return &nm.next;
    }
    void property_add_marker(struct property *prop,
    enum markertype type, unsigned int offset, char *ref)
    {
    add_marker(&prop.val.markers, type, offset, ref);
    }
#[no_mangle]
unsafe extern "C" fn add_string_markers(prop: *mut property, offset: c_uint, len: c_int) {
    static void add_string_markers(struct property *prop, unsigned int offset, int len)
    {
    int l;
    const char *p = prop.val.val + offset;
    struct marker **mi = &prop.val.markers;
    for (l = strlen(p) + 1; l < len; l += strlen(p + l) + 1)
    mi = add_marker(mi, TYPE_STRING, offset + l, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn add_phandle_marker(dti: *mut dt_info, prop: *mut property, offset: c_uint) {
    void add_phandle_marker(struct dt_info *dti, struct property *prop, unsigned int offset)
    {
    cell_t phandle;
    struct node *refn;
    char *ref;
    if (prop.val.len < offset + 4) {
    if (quiet < 1)
    fprintf(stderr,
    "Warning: property %s too short to contain a phandle at offset %u\n",
    prop.name, offset);
    return;
    }
    phandle = dtb_ld32(prop.val.val + offset);
    refn = get_node_by_phandle(dti.dt, phandle);
    if (!refn) {
    if (quiet < 1)
    fprintf(stderr,
    "Warning: node referenced by phandle 0x%x in property %s not found\n",
    phandle, prop.name);
    return;
    }
    if (refn.labels)
    ref = refn.labels.label;
    else
    ref = refn.fullpath;
    add_marker(&prop.val.markers, REF_PHANDLE, offset, ref);
    }
#[no_mangle]
unsafe extern "C" fn guess_value_type(prop: *mut property, offset: c_uint, len: c_int) -> enum markertype {
    static enum markertype guess_value_type(struct property *prop, unsigned int offset, int len)
    {
    const char *p = prop.val.val + offset;
    let mut nnotstring: c_int = 0, nnul = 0;
    int i;
    for (i = 0; i < len; i++) {
    if (! isstring(p[i]))
    nnotstring++;
    if (p[i] == '\0')
    nnul++;
    }
    if ((p[len-1] == '\0') && (nnotstring == 0) && (nnul <= len - nnul)) {
    if (nnul > 1)
    add_string_markers(prop, offset, len);
    return TYPE_STRING;
    } else if ((len % sizeof(cell_t)) == 0) {
    return TYPE_UINT32;
    }
    return TYPE_UINT8;
    }
#[no_mangle]
unsafe extern "C" fn guess_type_markers(prop: *mut property) {
    static void guess_type_markers(struct property *prop)
    {
    struct marker **m = &prop.val.markers;
    let mut offset: c_uint = 0;
    for (m = &prop.val.markers; *m; m = &((*m).next)) {
    if (is_type_marker((*m).type))
// assume the whole property is already marked
    return;
    if ((*m).offset > offset) {
    m = add_marker(m, guess_value_type(prop, offset, (*m).offset - offset),
    offset, core::ptr::null_mut());
    offset = (*m).offset;
    }
    if ((*m).type == REF_PHANDLE) {
    m = add_marker(m, TYPE_UINT32, offset, core::ptr::null_mut());
    offset += 4;
    }
    }
    if (offset < prop.val.len)
    add_marker(m, guess_value_type(prop, offset, prop.val.len - offset),
    offset, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn write_propval(f: *mut FILE, prop: *mut property) {
    static void write_propval(FILE *f, struct property *prop)
    {
    let mut len: usize = prop.val.len;
    struct marker *m;
    let mut emit_type: enum markertype = TYPE_NONE;
    char *srcstr;
    if (len == 0) {
    fprintf(f, ";");
    if (annotate) {
    srcstr = srcpos_string_first(prop.srcpos, annotate);
    if (srcstr) {
    fprintf(f, " /* %s */", srcstr);
    free(srcstr);
    }
    }
    fprintf(f, "\n");
    return;
    }
    fprintf(f, " =");
    guess_type_markers(prop);
    m = prop.val.markers;
    for_each_marker(m) {
    let mut chunk_len: usize = (m.next ? m.next.offset : len) - m.offset;
    let mut data_len: usize = type_marker_length(m) ? : len - m.offset;
    const char *p = &prop.val.val[m.offset];
    struct marker *m_phandle;
    if (is_type_marker(m.type)) {
    emit_type = m.type;
    fprintf(f, " %s", delim_start[emit_type]);
    } else if (m.type == LABEL)
    fprintf(f, " %s:", m.ref);
    if (emit_type == TYPE_NONE || chunk_len == 0)
    continue;
    switch(emit_type) {
    case TYPE_UINT16:
    write_propval_int(f, p, chunk_len, 2);
    break;
    case TYPE_UINT32:
    m_phandle = prop.val.markers;
    for_each_marker_of_type(m_phandle, REF_PHANDLE)
    if (m.offset == m_phandle.offset)
    break;
    if (m_phandle) {
    if (m_phandle.ref[0] == '/')
    fprintf(f, "&{%s}", m_phandle.ref);
    else
    fprintf(f, "&%s", m_phandle.ref);
    if (chunk_len > 4) {
    fputc(' ', f);
    write_propval_int(f, p + 4, chunk_len - 4, 4);
    }
    } else {
    write_propval_int(f, p, chunk_len, 4);
    }
    if (data_len > chunk_len)
    fputc(' ', f);
    break;
    case TYPE_UINT64:
    write_propval_int(f, p, chunk_len, 8);
    break;
    case TYPE_STRING:
    write_propval_string(f, p, chunk_len);
    break;
    default:
    write_propval_int(f, p, chunk_len, 1);
    }
    if (chunk_len == data_len) {
    let mut pos: usize = m.offset + chunk_len;
    fprintf(f, pos == len ? "%s" : "%s,",
    delim_end[emit_type] ? : "");
    emit_type = TYPE_NONE;
    }
    }
    fprintf(f, ";");
    if (annotate) {
    srcstr = srcpos_string_first(prop.srcpos, annotate);
    if (srcstr) {
    fprintf(f, " /* %s */", srcstr);
    free(srcstr);
    }
    }
    fprintf(f, "\n");
    }
#[no_mangle]
unsafe extern "C" fn write_tree_source_node(f: *mut FILE, tree: *mut node, level: c_int) {
    static void write_tree_source_node(FILE *f, struct node *tree, int level)
    {
    struct property *prop;
    struct node *child;
    struct label *l;
    char *srcstr;
    write_prefix(f, level);
    for_each_label(tree.labels, l)
    fprintf(f, "%s: ", l.label);
    if (tree.name && (*tree.name))
    fprintf(f, "%s {", tree.name);
    else
    fprintf(f, "/ {");
    if (annotate) {
    srcstr = srcpos_string_first(tree.srcpos, annotate);
    if (srcstr) {
    fprintf(f, " /* %s */", srcstr);
    free(srcstr);
    }
    }
    fprintf(f, "\n");
    for_each_property(tree, prop) {
    write_prefix(f, level+1);
    for_each_label(prop.labels, l)
    fprintf(f, "%s: ", l.label);
    fprintf(f, "%s", prop.name);
    write_propval(f, prop);
    }
    for_each_child(tree, child) {
    fprintf(f, "\n");
    write_tree_source_node(f, child, level+1);
    }
    write_prefix(f, level);
    fprintf(f, "};");
    if (annotate) {
    srcstr = srcpos_string_last(tree.srcpos, annotate);
    if (srcstr) {
    fprintf(f, " /* %s */", srcstr);
    free(srcstr);
    }
    }
    fprintf(f, "\n");
    }
#[no_mangle]
pub unsafe extern "C" fn dt_to_source(f: *mut FILE, dti: *mut dt_info) {
    void dt_to_source(FILE *f, struct dt_info *dti)
    {
    struct reserve_info *re;
    fprintf(f, "/dts-v1/;\n");
    if (dti.dtsflags & DTSF_PLUGIN)
    fprintf(f, "/plugin/;\n");
    fprintf(f, "\n");
    for (re = dti.reservelist; re; re = re.next) {
    struct label *l;
    for_each_label(re.labels, l)
    fprintf(f, "%s: ", l.label);
    fprintf(f, "/memreserve/\t0x%016llx 0x%016llx;\n",
    (unsigned long long)re.address,
    (unsigned long long)re.size);
    }
    write_tree_source_node(f, dti.dt, 0);
    }
