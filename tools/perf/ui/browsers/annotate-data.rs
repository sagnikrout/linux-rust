//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/browsers/annotate-data.c
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
pub struct browser_entry {
    pub node: list_head,
    pub data: *mut annotated_member,
    pub hists: *mut type_hist_entry,
    pub parent: *mut browser_entry,
    pub children: list_head,
    pub /: *mut *mut int indent; /indentation level, starts from 0,
    pub /: *mut *mut int nr_entries; / # of visible entries: self + descendents,
    pub /: *mut *mut bool folded; / only can be false when it has children,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct annotated_data_browser {
    pub b: ui_browser,
    pub entries: list_head,
    pub curr: *mut browser_entry,
    pub nr_events: c_int,
}

    static struct annotated_data_browser *get_browser(struct ui_browser *uib)
    {
    return container_of(uib, struct annotated_data_browser, b);
    }
    static void update_hist_entry(struct type_hist_entry *dst,
    struct type_hist_entry *src)
    {
    dst.nr_samples += src.nr_samples;
    dst.period += src.period;
    }
    static int get_member_overhead(struct annotated_data_type *adt,
    struct browser_entry *entry,
    struct evsel *leader)
    {
    struct annotated_member *member = entry.data;
    int i, k;
    for (i = 0; i < member.size; i++) {
    struct type_hist *h;
    struct evsel *evsel;
    let mut offset: c_int = member.offset + i;
    k = 0;
    for_each_group_evsel(evsel, leader) {
    if (symbol_conf.skip_empty &&
    evsel__hists(evsel).stats.nr_samples == 0)
    continue;
    h = adt.histograms[evsel.core.idx];
    update_hist_entry(&entry.hists[k++], &h.addr[offset]);
    }
    }
    return 0;
    }
    static int add_child_entries(struct annotated_data_browser *browser,
    struct browser_entry *parent,
    struct annotated_data_type *adt,
    struct annotated_member *member,
    struct evsel *evsel, int indent)
    {
    struct annotated_member *pos;
    struct browser_entry *entry;
    struct list_head *parent_list;
    entry = zalloc(sizeof(*entry));
    if (entry == core::ptr::null_mut())
    return -1;
    entry.hists = calloc(browser.nr_events, sizeof(*entry.hists));
    if (entry.hists == core::ptr::null_mut()) {
    free(entry);
    return -1;
    }
    entry.data = member;
    entry.parent = parent;
    entry.indent = indent;
    if (get_member_overhead(adt, entry, evsel) < 0) {
    free(entry);
    return -1;
    }
    INIT_LIST_HEAD(&entry.children);
    if (parent)
    parent_list = &parent.children;
    else
    parent_list = &browser.entries;
    list_add_tail(&entry.node, parent_list);
    list_for_each_entry(pos, &member.children, node) {
    int nr = add_child_entries(browser, entry, adt, pos, evsel,
    indent + 1);
    if (nr < 0)
    return nr;
    }
// add an entry for the closing bracket ("}")
    if (!list_empty(&member.children)) {
    struct browser_entry *bracket;
    bracket = zalloc(sizeof(*bracket));
    if (bracket == core::ptr::null_mut())
    return -1;
    bracket.indent = indent;
    bracket.parent = entry;
    bracket.folded = true;
    bracket.nr_entries = 1;
    INIT_LIST_HEAD(&bracket.children);
    list_add_tail(&bracket.node, &entry.children);
    }
// fold child entries by default
    entry.folded = true;
    entry.nr_entries = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn count_visible_entries(browser: *mut annotated_data_browser) -> u32 {
    static u32 count_visible_entries(struct annotated_data_browser *browser)
    {
    let mut nr: c_int = 0;
    struct browser_entry *entry;
    list_for_each_entry(entry, &browser.entries, node)
    nr += entry.nr_entries;
    return nr;
    }
#[no_mangle]
unsafe extern "C" fn annotated_data_browser__collect_entries(browser: *mut annotated_data_browser) -> c_int {
    static int annotated_data_browser__collect_entries(struct annotated_data_browser *browser)
    {
    struct hist_entry *he = browser.b.priv;
    struct annotated_data_type *adt = he.mem_type;
    struct evsel *evsel = hists_to_evsel(he.hists);
    INIT_LIST_HEAD(&browser.entries);
    add_child_entries(browser, /*parent=*/core::ptr::null_mut(), adt, &adt.self, evsel,
// indent=*/0);
    browser.b.entries = &browser.entries;
    browser.b.nr_entries = count_visible_entries(browser);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn annotated_data_browser__delete_entries(browser: *mut annotated_data_browser) {
    static void annotated_data_browser__delete_entries(struct annotated_data_browser *browser)
    {
    struct browser_entry *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, &browser.entries, node) {
    list_del_init(&pos.node);
    zfree(&pos.hists);
    free(pos);
    }
    }
    static struct browser_entry *get_first_child(struct browser_entry *entry)
    {
    if (list_empty(&entry.children))
    return core::ptr::null_mut();
    return list_first_entry(&entry.children, struct browser_entry, node);
    }
    static struct browser_entry *get_last_child(struct browser_entry *entry)
    {
    if (list_empty(&entry.children))
    return core::ptr::null_mut();
    return list_last_entry(&entry.children, struct browser_entry, node);
    }
#[no_mangle]
unsafe extern "C" fn is_first_child(entry: *mut browser_entry) -> bool {
    static bool is_first_child(struct browser_entry *entry)
    {
// This will be checked in a different way
    if (entry.parent == core::ptr::null_mut())
    return false;
    return get_first_child(entry.parent) == entry;
    }
#[no_mangle]
unsafe extern "C" fn is_last_child(entry: *mut browser_entry) -> bool {
    static bool is_last_child(struct browser_entry *entry)
    {
// This will be checked in a different way
    if (entry.parent == core::ptr::null_mut())
    return false;
    return get_last_child(entry.parent) == entry;
    }
    static struct browser_entry *browser__prev_entry(struct ui_browser *uib,
    struct browser_entry *entry)
    {
    struct annotated_data_browser *browser = get_browser(uib);
    struct browser_entry *first;
    first = list_first_entry(&browser.entries, struct browser_entry, node);
    while (entry != first) {
    if (is_first_child(entry))
    entry = entry.parent;
    else {
    entry = list_prev_entry(entry, node);
    while (!entry.folded)
    entry = get_last_child(entry);
    }
    if (!uib.filter || !uib.filter(uib, &entry.node))
    return entry;
    }
    return first;
    }
    static struct browser_entry *browser__next_entry(struct ui_browser *uib,
    struct browser_entry *entry)
    {
    struct annotated_data_browser *browser = get_browser(uib);
    struct browser_entry *last;
    last = list_last_entry(&browser.entries, struct browser_entry, node);
    while (!last.folded)
    last = get_last_child(last);
    while (entry != last) {
    if (!entry.folded)
    entry = get_first_child(entry);
    else {
    while (is_last_child(entry))
    entry = entry.parent;
    entry = list_next_entry(entry, node);
    }
    if (!uib.filter || !uib.filter(uib, &entry.node))
    return entry;
    }
    return last;
    }
#[no_mangle]
unsafe extern "C" fn browser__seek(uib: *mut ui_browser, offset: off_t, whence: c_int) {
    static void browser__seek(struct ui_browser *uib, off_t offset, int whence)
    {
    struct annotated_data_browser *browser = get_browser(uib);
    struct browser_entry *entry;
    if (uib.nr_entries == 0)
    return;
    switch (whence) {
    case SEEK_SET:
    entry = list_first_entry(&browser.entries, typeof(*entry), node);
    if (uib.filter && uib.filter(uib, &entry.node))
    entry = browser__next_entry(uib, entry);
    break;
    case SEEK_CUR:
    entry = list_entry(uib.top, typeof(*entry), node);
    break;
    case SEEK_END:
    entry = list_last_entry(&browser.entries, typeof(*entry), node);
    while (!entry.folded)
    entry = get_last_child(entry);
    if (uib.filter && uib.filter(uib, &entry.node))
    entry = browser__prev_entry(uib, entry);
    break;
    default:
    return;
    }
    assert(entry != core::ptr::null_mut());
    if (offset > 0) {
    while (offset-- != 0)
    entry = browser__next_entry(uib, entry);
    } else {
    while (offset++ != 0)
    entry = browser__prev_entry(uib, entry);
    }
    uib.top = &entry.node;
    }
#[no_mangle]
unsafe extern "C" fn browser__refresh(uib: *mut ui_browser) -> c_uint {
    static unsigned int browser__refresh(struct ui_browser *uib)
    {
    struct annotated_data_browser *browser = get_browser(uib);
    struct browser_entry *entry, *next;
    let mut row: c_int = 0;
    if (uib.top == core::ptr::null_mut() || uib.top == uib.entries)
    browser__seek(uib, SEEK_SET, 0);
    entry = list_entry(uib.top, typeof(*entry), node);
    while (true) {
    if (!uib.filter || !uib.filter(uib, &entry.node)) {
    ui_browser__gotorc(uib, row, 0);
    uib.write(uib, entry, row);
    if (uib.top_idx + row == uib.index)
    browser.curr = entry;
    if (++row == uib.rows)
    break;
    }
    next = browser__next_entry(uib, entry);
    if (next == entry)
    break;
    entry = next;
    }
    return row;
    }
#[no_mangle]
unsafe extern "C" fn browser__show(uib: *mut ui_browser) -> c_int {
    static int browser__show(struct ui_browser *uib)
    {
    struct hist_entry *he = uib.priv;
    struct annotated_data_type *adt = he.mem_type;
    struct annotated_data_browser *browser = get_browser(uib);
    const char *help = "Press 'h' for help on key bindings";
    char title[256];
    snprintf(title, sizeof(title), "Annotate type: '%s' (%d samples)",
    adt.self.type_name, he.stat.nr_events);
    if (ui_browser__show(uib, title, help) < 0)
    return -1;
// second line header
    ui_browser__gotorc_title(uib, 0, 0);
    ui_browser__set_color(uib, HE_COLORSET_ROOT);
    if (symbol_conf.show_total_period)
    strcpy(title, "Period");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: symbol_conf.show_nr_samples) -> else {
    else if (symbol_conf.show_nr_samples)
    strcpy(title, "Samples");
    else
    strcpy(title, "Percent");
    ui_browser__printf(uib, "%*s %10s %10s %10s  %s",
    2 + 11 * (browser.nr_events - 1), "",
    title, "Offset", "Size", "Field");
    ui_browser__write_nstring(uib, "", uib.width);
    return 0;
    }
    static void browser__write_overhead(struct ui_browser *uib,
    struct type_hist *total,
    struct type_hist_entry *hist, int row)
    {
    let mut period: u64 = hist.period;
    let mut percent: double = total.period ? (100.0 * period / total.period) : 0;
    let mut current: bool = ui_browser__is_current_entry(uib, row);
    let mut nr_samples: c_int = 0;
    ui_browser__set_percent_color(uib, percent, current);
    if (symbol_conf.show_total_period)
    ui_browser__printf(uib, " %10" PRIu64, period);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: symbol_conf.show_nr_samples) -> else {
    else if (symbol_conf.show_nr_samples)
    ui_browser__printf(uib, " %10d", nr_samples);
    else
    ui_browser__printf(uib, " %10.2f", percent);
    ui_browser__set_percent_color(uib, 0, current);
    }
#[no_mangle]
unsafe extern "C" fn browser__write(uib: *mut ui_browser, entry: *mut c_void, row: c_int) {
    static void browser__write(struct ui_browser *uib, void *entry, int row)
    {
    struct annotated_data_browser *browser = get_browser(uib);
    struct browser_entry *be = entry;
    struct annotated_member *member = be.data;
    struct hist_entry *he = uib.priv;
    struct annotated_data_type *adt = he.mem_type;
    struct evsel *leader = hists_to_evsel(he.hists);
    struct evsel *evsel;
    let mut idx: c_int = 0;
    let mut current: bool = ui_browser__is_current_entry(uib, row);
    if (member == core::ptr::null_mut()) {
// print the closing bracket
    ui_browser__set_percent_color(uib, 0, current);
    ui_browser__printf(uib, "%c ", NOCHLD_SIGN);
    ui_browser__write_nstring(uib, "", 11 * browser.nr_events);
    ui_browser__printf(uib, " %10s %10s  %*s};",
    "", "", be.indent * 4, "");
    ui_browser__write_nstring(uib, "", uib.width);
    return;
    }
    ui_browser__set_percent_color(uib, 0, current);
    if (!list_empty(&be.children))
    ui_browser__printf(uib, "%c ", be.folded ? FOLDED_SIGN : UNFOLD_SIGN);
    else
    ui_browser__printf(uib, "%c ", NOCHLD_SIGN);
// print the number
    for_each_group_evsel(evsel, leader) {
    struct type_hist *h = adt.histograms[evsel.core.idx];
    if (symbol_conf.skip_empty &&
    evsel__hists(evsel).stats.nr_samples == 0)
    continue;
    browser__write_overhead(uib, h, &be.hists[idx++], row);
    }
// print type info
    if (be.indent == 0 && !member.var_name) {
    ui_browser__printf(uib, " %#10x %#10x  %s%s",
    member.offset, member.size,
    member.type_name,
    list_empty(&member.children) || be.folded? ";" : " {");
    } else {
    ui_browser__printf(uib, " %#10x %#10x  %*s%s\t%s%s",
    member.offset, member.size,
    be.indent * 4, "", member.type_name,
    member.var_name ?: "",
    list_empty(&member.children) || be.folded ? ";" : " {");
    }
// fill the rest
    ui_browser__write_nstring(uib, "", uib.width);
    }
    static void annotated_data_browser__fold(struct annotated_data_browser *browser,
    struct browser_entry *entry,
    bool recursive)
    {
    struct browser_entry *child;
    if (list_empty(&entry.children))
    return;
    if (entry.folded && !recursive)
    return;
    if (recursive) {
    list_for_each_entry(child, &entry.children, node)
    annotated_data_browser__fold(browser, child, true);
    }
    entry.nr_entries = 1;
    entry.folded = true;
    }
    static void annotated_data_browser__unfold(struct annotated_data_browser *browser,
    struct browser_entry *entry,
    bool recursive)
    {
    struct browser_entry *child;
    int nr_entries;
    if (list_empty(&entry.children))
    return;
    if (!entry.folded && !recursive)
    return;
    nr_entries = 1; /* for self */
    list_for_each_entry(child, &entry.children, node) {
    if (recursive)
    annotated_data_browser__unfold(browser, child, true);
    nr_entries += child.nr_entries;
    }
    entry.nr_entries = nr_entries;
    entry.folded = false;
    }
    static void annotated_data_browser__toggle_fold(struct annotated_data_browser *browser,
    bool recursive)
    {
    struct browser_entry *curr = browser.curr;
    struct browser_entry *parent;
    parent = curr.parent;
    while (parent) {
    parent.nr_entries -= curr.nr_entries;
    parent = parent.parent;
    }
    browser.b.nr_entries -= curr.nr_entries;
    if (curr.folded)
    annotated_data_browser__unfold(browser, curr, recursive);
    else
    annotated_data_browser__fold(browser, curr, recursive);
    parent = curr.parent;
    while (parent) {
    parent.nr_entries += curr.nr_entries;
    parent = parent.parent;
    }
    browser.b.nr_entries += curr.nr_entries;
    assert(browser.b.nr_entries == count_visible_entries(browser));
    }
    static int annotated_data_browser__run(struct annotated_data_browser *browser,
    struct evsel *evsel __maybe_unused,
    struct hist_browser_timer *hbt)
    {
    let mut delay_secs: c_int = hbt ? hbt.refresh : 0;
    int key;
    if (browser__show(&browser.b) < 0)
    return -1;
    while (1) {
    key = ui_browser__run(&browser.b, delay_secs);
    switch (key) {
    case K_TIMER:
    if (hbt)
    hbt.timer(hbt.arg);
    continue;
    case K_F1:
    case 'h':
    ui_browser__help_window(&browser.b,
    "UP/DOWN/PGUP\n"
    "PGDN/SPACE    Navigate\n"
    "</>           Move to prev/next symbol\n"
    "e             Expand/Collapse current entry\n"
    "E             Expand/Collapse all children of the current\n"
    "q/ESC/CTRL+C  Exit\n\n");
    continue;
    case 'e':
    annotated_data_browser__toggle_fold(browser,
// recursive=*/false);
    break;
    case 'E':
    annotated_data_browser__toggle_fold(browser,
// recursive=*/true);
    break;
    case K_LEFT:
    case '<':
    case '>':
    case K_ESC:
    case 'q':
    case CTRL('c'):
    goto out;
    default:
    ui_browser__warn_unhandled_hotkey(&browser.b, key, delay_secs, ", use 'h'/F1 to see actions");
    continue;
    }
    }
    out:
    ui_browser__hide(&browser.b);
    return key;
    }
    int hist_entry__annotate_data_tui(struct hist_entry *he, struct evsel *evsel,
    struct hist_browser_timer *hbt)
    {
    struct annotated_data_browser browser = {
    .b = {
    .refresh = browser__refresh,
    .seek	 = browser__seek,
    .write	 = browser__write,
    .priv	 = he,
    .extra_title_lines = 1,
    },
    .nr_events = 1,
    };
    int ret;
    ui_helpline__push("Press ESC to exit");
    if (evsel__is_group_event(evsel)) {
    struct evsel *pos;
    let mut nr: c_int = 0;
    for_each_group_evsel(pos, evsel) {
    if (!symbol_conf.skip_empty ||
    evsel__hists(pos).stats.nr_samples)
    nr++;
    }
    browser.nr_events = nr;
    }
    ret = annotated_data_browser__collect_entries(&browser);
    if (ret < 0)
    goto out;
// To get the top and current entry
    browser__refresh(&browser.b);
// Show the first-level child entries by default
    annotated_data_browser__toggle_fold(&browser, /*recursive=*/false);
    ret = annotated_data_browser__run(&browser, evsel, hbt);
    out:
    annotated_data_browser__delete_entries(&browser);
    return ret;
    }
