//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-kmem.c
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

    static int	kmem_slab;
    static int	kmem_page;
    static long	kmem_page_size;
    static enum {
    KMEM_SLAB,
    KMEM_PAGE,
    } kmem_default = KMEM_SLAB;  /* for backward compatibility */
    struct alloc_stat;
    typedef int (*sort_fn_t)(void *, void *);
    static int			alloc_flag;
    static int			caller_flag;
    let mut alloc_lines: static int = -1;
    let mut caller_lines: static int = -1;
    static bool			raw_ip;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_stat {
    pub call_site: u64,
    pub ptr: u64,
    pub bytes_req: u64,
    pub bytes_alloc: u64,
    pub last_alloc: u64,
    pub hit: u32,
    pub pingpong: u32,
    pub alloc_cpu: c_short,
    pub node: rb_node,
}

    static struct rb_root root_alloc_stat;
    static struct rb_root root_alloc_sorted;
    static struct rb_root root_caller_stat;
    static struct rb_root root_caller_sorted;
    static unsigned long total_requested, total_allocated, total_freed;
    static unsigned long nr_allocs, nr_cross_allocs;
// filters for controlling start and stop of time of analysis
    static struct perf_time_interval ptime;
    static const char *time_str;
    static int insert_alloc_stat(unsigned long call_site, unsigned long ptr,
    int bytes_req, int bytes_alloc, int cpu)
    {
    struct rb_node **node = &root_alloc_stat.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct alloc_stat *data = core::ptr::null_mut();
    while (*node) {
    parent = *node;
    data = rb_entry(*node, struct alloc_stat, node);
    if (ptr > data.ptr)
    node = &(*node).rb_right;
#[no_mangle]
pub unsafe extern "C" fn if(data->ptr: ptr <) -> else {
    else if (ptr < data.ptr)
    node = &(*node).rb_left;
    else
    break;
    }
    if (data && data.ptr == ptr) {
    data.hit++;
    data.bytes_req += bytes_req;
    data.bytes_alloc += bytes_alloc;
    } else {
    data = malloc(sizeof(*data));
    if (!data) {
    pr_err("%s: malloc failed\n", __func__);
    return -1;
    }
    data.ptr = ptr;
    data.pingpong = 0;
    data.hit = 1;
    data.bytes_req = bytes_req;
    data.bytes_alloc = bytes_alloc;
    rb_link_node(&data.node, parent, node);
    rb_insert_color(&data.node, &root_alloc_stat);
    }
    data.call_site = call_site;
    data.alloc_cpu = cpu;
    data.last_alloc = bytes_alloc;
    return 0;
    }
    static int insert_caller_stat(unsigned long call_site,
    int bytes_req, int bytes_alloc)
    {
    struct rb_node **node = &root_caller_stat.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct alloc_stat *data = core::ptr::null_mut();
    while (*node) {
    parent = *node;
    data = rb_entry(*node, struct alloc_stat, node);
    if (call_site > data.call_site)
    node = &(*node).rb_right;
#[no_mangle]
pub unsafe extern "C" fn if(data->call_site: call_site <) -> else {
    else if (call_site < data.call_site)
    node = &(*node).rb_left;
    else
    break;
    }
    if (data && data.call_site == call_site) {
    data.hit++;
    data.bytes_req += bytes_req;
    data.bytes_alloc += bytes_alloc;
    } else {
    data = malloc(sizeof(*data));
    if (!data) {
    pr_err("%s: malloc failed\n", __func__);
    return -1;
    }
    data.call_site = call_site;
    data.pingpong = 0;
    data.hit = 1;
    data.bytes_req = bytes_req;
    data.bytes_alloc = bytes_alloc;
    rb_link_node(&data.node, parent, node);
    rb_insert_color(&data.node, &root_caller_stat);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn evsel__process_alloc_event(sample: *mut perf_sample) -> c_int {
    static int evsel__process_alloc_event(struct perf_sample *sample)
    {
    unsigned long ptr = perf_sample__intval(sample, "ptr"),
    call_site = perf_sample__intval(sample, "call_site");
    int bytes_req = perf_sample__intval(sample, "bytes_req"),
    bytes_alloc = perf_sample__intval(sample, "bytes_alloc");
    if (insert_alloc_stat(call_site, ptr, bytes_req, bytes_alloc, sample.cpu) ||
    insert_caller_stat(call_site, bytes_req, bytes_alloc))
    return -1;
    total_requested += bytes_req;
    total_allocated += bytes_alloc;
    nr_allocs++;
//
// Commit 11e9734bcb6a ("mm/slab_common: unify NUMA and UMA
// version of tracepoints") adds the field "node" into the
// tracepoints 'kmalloc' and 'kmem_cache_alloc'.
//
// The legacy tracepoints 'kmalloc_node' and 'kmem_cache_alloc_node'
// also contain the field "node".
//
// If the tracepoint contains the field "node" the tool stats the
// cross allocation.
//
    if (evsel__field(sample.evsel, "node")) {
    int node1, node2;
    node1 = cpu__get_node((struct perf_cpu){.cpu = sample.cpu});
    node2 = perf_sample__intval(sample, "node");
//
// If the field "node" is NUMA_NO_NODE (-1), we don't take it
// as a cross allocation.
//
    if ((node2 != NUMA_NO_NODE) && (node1 != node2))
    nr_cross_allocs++;
    }
    return 0;
    }
    static int ptr_cmp(void *, void *);
    static int slab_callsite_cmp(void *, void *);
    static struct alloc_stat *search_alloc_stat(unsigned long ptr,
    unsigned long call_site,
    struct rb_root *root,
    sort_fn_t sort_fn)
    {
    struct rb_node *node = root.rb_node;
    let mut key: alloc_stat = { .ptr = ptr, .call_site = call_site };
    while (node) {
    struct alloc_stat *data;
    int cmp;
    data = rb_entry(node, struct alloc_stat, node);
    cmp = sort_fn(&key, data);
    if (cmp < 0)
    node = node.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    node = node.rb_right;
    else
    return data;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn evsel__process_free_event(sample: *mut perf_sample) -> c_int {
    static int evsel__process_free_event(struct perf_sample *sample)
    {
    let mut ptr: c_ulong = perf_sample__intval(sample, "ptr");
    struct alloc_stat *s_alloc, *s_caller;
    s_alloc = search_alloc_stat(ptr, 0, &root_alloc_stat, ptr_cmp);
    if (!s_alloc)
    return 0;
    total_freed += s_alloc.last_alloc;
    if ((short)sample.cpu != s_alloc.alloc_cpu) {
    s_alloc.pingpong++;
    s_caller = search_alloc_stat(0, s_alloc.call_site,
    &root_caller_stat,
    slab_callsite_cmp);
    if (!s_caller)
    return -1;
    s_caller.pingpong++;
    }
    s_alloc.alloc_cpu = -1;
    return 0;
    }
    static u64 total_page_alloc_bytes;
    static u64 total_page_free_bytes;
    static u64 total_page_nomatch_bytes;
    static u64 total_page_fail_bytes;
    static unsigned long nr_page_allocs;
    static unsigned long nr_page_frees;
    static unsigned long nr_page_fails;
    static unsigned long nr_page_nomatch;
    static bool use_pfn;
    static bool live_page;
    static struct perf_session *kmem_session;
pub const MAX_MIGRATE_TYPES: c_int = 6;
pub const MAX_PAGE_ORDER: c_int = 11;
    static int order_stats[MAX_PAGE_ORDER][MAX_MIGRATE_TYPES];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_stat {
    pub node: rb_node,
    pub page: u64,
    pub callsite: u64,
    pub order: c_int,
    pub gfp_flags: unsigned,
    pub migrate_type: unsigned,
    pub alloc_bytes: u64,
    pub free_bytes: u64,
    pub nr_alloc: c_int,
    pub nr_free: c_int,
}

    static struct rb_root page_live_tree;
    static struct rb_root page_alloc_tree;
    static struct rb_root page_alloc_sorted;
    static struct rb_root page_caller_tree;
    static struct rb_root page_caller_sorted;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_func {
    pub start: u64,
    pub end: u64,
    pub name: *mut c_char,
}

    static int nr_alloc_funcs;
    static struct alloc_func *alloc_func_list;
#[no_mangle]
unsafe extern "C" fn funcmp(a: *const c_void, b: *const c_void) -> c_int {
    static int funcmp(const void *a, const void *b)
    {
    const struct alloc_func *fa = a;
    const struct alloc_func *fb = b;
    if (fa.start > fb.start)
    return 1;
    else
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn callcmp(a: *const c_void, b: *const c_void) -> c_int {
    static int callcmp(const void *a, const void *b)
    {
    const struct alloc_func *fa = a;
    const struct alloc_func *fb = b;
    if (fb.start <= fa.start && fa.end < fb.end)
    return 0;
    if (fa.start > fb.start)
    return 1;
    else
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn build_alloc_func_list() -> c_int {
    static int build_alloc_func_list(void)
    {
    int ret;
    struct map *kernel_map;
    struct symbol *sym;
    struct rb_node *node;
    struct alloc_func *func;
    struct machine *machine = &kmem_session.machines.host;
    regex_t alloc_func_regex;
    static const char pattern[] = "^_?_?(alloc|get_free|get_zeroed)_pages?";
    ret = regcomp(&alloc_func_regex, pattern, REG_EXTENDED);
    if (ret) {
    char err[BUFSIZ];
    regerror(ret, &alloc_func_regex, err, sizeof(err));
    pr_err("Invalid regex: %s\n%s", pattern, err);
    return -EINVAL;
    }
    kernel_map = machine__kernel_map(machine);
    if (map__load(kernel_map) < 0) {
    pr_err("cannot load kernel map\n");
    return -ENOENT;
    }
    map__for_each_symbol(kernel_map, sym, node) {
    if (regexec(&alloc_func_regex, sym.name, 0, core::ptr::null_mut(), 0))
    continue;
    func = realloc(alloc_func_list,
    (nr_alloc_funcs + 1) * sizeof(*func));
    if (func == core::ptr::null_mut())
    return -ENOMEM;
    pr_debug("alloc func: %s\n", sym.name);
    func[nr_alloc_funcs].start = sym.start;
    func[nr_alloc_funcs].end   = sym.end;
    func[nr_alloc_funcs].name  = sym.name;
    alloc_func_list = func;
    nr_alloc_funcs++;
    }
    qsort(alloc_func_list, nr_alloc_funcs, sizeof(*func), funcmp);
    regfree(&alloc_func_regex);
    return 0;
    }
//
// Find first non-memory allocation function from callchain.
// The allocation functions are in the 'alloc_func_list'.
//
#[no_mangle]
unsafe extern "C" fn find_callsite(sample: *mut perf_sample) -> u64 {
    static u64 find_callsite(struct perf_sample *sample)
    {
    struct addr_location al;
    struct machine *machine = &kmem_session.machines.host;
    struct callchain_cursor_node *node;
    struct callchain_cursor *cursor;
    let mut result: u64 = sample.ip;
    addr_location__init(&al);
    if (alloc_func_list == core::ptr::null_mut()) {
    if (build_alloc_func_list() < 0)
    goto out;
    }
    al.thread = machine__findnew_thread(machine, sample.pid, sample.tid);
    cursor = get_tls_callchain_cursor();
    if (cursor == core::ptr::null_mut())
    goto out;
    sample__resolve_callchain(sample, cursor, /*parent=*/core::ptr::null_mut(), &al, 16);
    callchain_cursor_commit(cursor);
    while (true) {
    struct alloc_func key, *caller;
    u64 addr;
    node = callchain_cursor_current(cursor);
    if (node == core::ptr::null_mut())
    break;
    key.start = key.end = node.ip;
    caller = bsearch(&key, alloc_func_list, nr_alloc_funcs,
    sizeof(key), callcmp);
    if (!caller) {
// found
    if (node.ms.map)
    addr = map__dso_unmap_ip(node.ms.map, node.ip);
    else
    addr = node.ip;
    result = addr;
    goto out;
    } else
    pr_debug3("skipping alloc function: %s\n", caller.name);
    callchain_cursor_advance(cursor);
    }
    pr_debug2("unknown callsite: %"PRIx64 "\n", sample.ip);
    out:
    addr_location__exit(&al);
    return result;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sort_dimension {
    pub name: [c_char; 20],
    pub cmp: sort_fn_t,
    pub list: list_head,
}

    static LIST_HEAD(page_alloc_sort_input);
    static LIST_HEAD(page_caller_sort_input);
    static struct page_stat *
    __page_stat__findnew_page(struct page_stat *pstat, bool create)
    {
    struct rb_node **node = &page_live_tree.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct page_stat *data;
    while (*node) {
    s64 cmp;
    parent = *node;
    data = rb_entry(*node, struct page_stat, node);
    cmp = data.page - pstat.page;
    if (cmp < 0)
    node = &parent.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    node = &parent.rb_right;
    else
    return data;
    }
    if (!create)
    return core::ptr::null_mut();
    data = zalloc(sizeof(*data));
    if (data != core::ptr::null_mut()) {
    data.page = pstat.page;
    data.order = pstat.order;
    data.gfp_flags = pstat.gfp_flags;
    data.migrate_type = pstat.migrate_type;
    rb_link_node(&data.node, parent, node);
    rb_insert_color(&data.node, &page_live_tree);
    }
    return data;
    }
    static struct page_stat *page_stat__find_page(struct page_stat *pstat)
    {
    return __page_stat__findnew_page(pstat, false);
    }
    static struct page_stat *page_stat__findnew_page(struct page_stat *pstat)
    {
    return __page_stat__findnew_page(pstat, true);
    }
    static struct page_stat *
    __page_stat__findnew_alloc(struct page_stat *pstat, bool create)
    {
    struct rb_node **node = &page_alloc_tree.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct page_stat *data;
    struct sort_dimension *sort;
    while (*node) {
    let mut cmp: c_int = 0;
    parent = *node;
    data = rb_entry(*node, struct page_stat, node);
    list_for_each_entry(sort, &page_alloc_sort_input, list) {
    cmp = sort.cmp(pstat, data);
    if (cmp)
    break;
    }
    if (cmp < 0)
    node = &parent.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    node = &parent.rb_right;
    else
    return data;
    }
    if (!create)
    return core::ptr::null_mut();
    data = zalloc(sizeof(*data));
    if (data != core::ptr::null_mut()) {
    data.page = pstat.page;
    data.order = pstat.order;
    data.gfp_flags = pstat.gfp_flags;
    data.migrate_type = pstat.migrate_type;
    rb_link_node(&data.node, parent, node);
    rb_insert_color(&data.node, &page_alloc_tree);
    }
    return data;
    }
    static struct page_stat *page_stat__find_alloc(struct page_stat *pstat)
    {
    return __page_stat__findnew_alloc(pstat, false);
    }
    static struct page_stat *page_stat__findnew_alloc(struct page_stat *pstat)
    {
    return __page_stat__findnew_alloc(pstat, true);
    }
    static struct page_stat *
    __page_stat__findnew_caller(struct page_stat *pstat, bool create)
    {
    struct rb_node **node = &page_caller_tree.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct page_stat *data;
    struct sort_dimension *sort;
    while (*node) {
    let mut cmp: c_int = 0;
    parent = *node;
    data = rb_entry(*node, struct page_stat, node);
    list_for_each_entry(sort, &page_caller_sort_input, list) {
    cmp = sort.cmp(pstat, data);
    if (cmp)
    break;
    }
    if (cmp < 0)
    node = &parent.rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: cmp >) -> else {
    else if (cmp > 0)
    node = &parent.rb_right;
    else
    return data;
    }
    if (!create)
    return core::ptr::null_mut();
    data = zalloc(sizeof(*data));
    if (data != core::ptr::null_mut()) {
    data.callsite = pstat.callsite;
    data.order = pstat.order;
    data.gfp_flags = pstat.gfp_flags;
    data.migrate_type = pstat.migrate_type;
    rb_link_node(&data.node, parent, node);
    rb_insert_color(&data.node, &page_caller_tree);
    }
    return data;
    }
    static struct page_stat *page_stat__find_caller(struct page_stat *pstat)
    {
    return __page_stat__findnew_caller(pstat, false);
    }
    static struct page_stat *page_stat__findnew_caller(struct page_stat *pstat)
    {
    return __page_stat__findnew_caller(pstat, true);
    }
#[no_mangle]
unsafe extern "C" fn valid_page(pfn_or_page: u64) -> bool {
    static bool valid_page(u64 pfn_or_page)
    {
    if (use_pfn && pfn_or_page == -1UL)
    return false;
    if (!use_pfn && pfn_or_page == 0)
    return false;
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfp_flag {
    pub flags: c_uint,
    pub compact_str: *mut c_char,
    pub human_readable: *mut c_char,
}

    static struct gfp_flag *gfps;
    static int nr_gfps;
#[no_mangle]
unsafe extern "C" fn gfpcmp(a: *const c_void, b: *const c_void) -> c_int {
    static int gfpcmp(const void *a, const void *b)
    {
    const struct gfp_flag *fa = a;
    const struct gfp_flag *fb = b;
    return fa.flags - fb.flags;
    }
// see include/trace/events/mmflags.h
    static const struct {
    const char *original;
    const char *compact;
    } gfp_compact_table[] = {
    { "GFP_TRANSHUGE",		"THP" },
    { "GFP_TRANSHUGE_LIGHT",	"THL" },
    { "GFP_HIGHUSER_MOVABLE",	"HUM" },
    { "GFP_HIGHUSER",		"HU" },
    { "GFP_USER",			"U" },
    { "GFP_KERNEL_ACCOUNT",		"KAC" },
    { "GFP_KERNEL",			"K" },
    { "GFP_NOFS",			"NF" },
    { "GFP_ATOMIC",			"A" },
    { "GFP_NOIO",			"NI" },
    { "GFP_NOWAIT",			"NW" },
    { "GFP_DMA",			"D" },
    { "__GFP_HIGHMEM",		"HM" },
    { "GFP_DMA32",			"D32" },
    { "__GFP_HIGH",			"H" },
    { "__GFP_IO",			"I" },
    { "__GFP_FS",			"F" },
    { "__GFP_NOWARN",		"NWR" },
    { "__GFP_RETRY_MAYFAIL",	"R" },
    { "__GFP_NOFAIL",		"NF" },
    { "__GFP_NORETRY",		"NR" },
    { "__GFP_COMP",			"C" },
    { "__GFP_ZERO",			"Z" },
    { "__GFP_NOMEMALLOC",		"NMA" },
    { "__GFP_MEMALLOC",		"MA" },
    { "__GFP_HARDWALL",		"HW" },
    { "__GFP_THISNODE",		"TN" },
    { "__GFP_RECLAIMABLE",		"RC" },
    { "__GFP_MOVABLE",		"M" },
    { "__GFP_ACCOUNT",		"AC" },
    { "__GFP_WRITE",		"WR" },
    { "__GFP_RECLAIM",		"R" },
    { "__GFP_DIRECT_RECLAIM",	"DR" },
    { "__GFP_KSWAPD_RECLAIM",	"KR" },
    };
    static size_t max_gfp_len;
    static char *compact_gfp_flags(char *gfp_flags)
    {
    char *orig_flags = strdup(gfp_flags);
    char *new_flags = core::ptr::null_mut();
    char *str, *pos = core::ptr::null_mut();
    let mut len: usize = 0;
    if (orig_flags == core::ptr::null_mut())
    return core::ptr::null_mut();
    str = strtok_r(orig_flags, "|", &pos);
    while (str) {
    size_t i;
    char *new;
    const char *cpt;
    for (i = 0; i < ARRAY_SIZE(gfp_compact_table); i++) {
    if (strcmp(gfp_compact_table[i].original, str))
    continue;
    cpt = gfp_compact_table[i].compact;
    new = realloc(new_flags, len + strlen(cpt) + 2);
    if (new == core::ptr::null_mut()) {
    free(new_flags);
    free(orig_flags);
    return core::ptr::null_mut();
    }
    new_flags = new;
    if (!len) {
    strcpy(new_flags, cpt);
    } else {
    strcat(new_flags, "|");
    strcat(new_flags, cpt);
    len++;
    }
    len += strlen(cpt);
    }
    str = strtok_r(core::ptr::null_mut(), "|", &pos);
    }
    if (max_gfp_len < len)
    max_gfp_len = len;
    free(orig_flags);
    return new_flags;
    }
    static char *compact_gfp_string(unsigned long gfp_flags)
    {
    struct gfp_flag key = {
    .flags = gfp_flags,
    };
    struct gfp_flag *gfp;
    gfp = bsearch(&key, gfps, nr_gfps, sizeof(*gfps), gfpcmp);
    if (gfp)
    return gfp.compact_str;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn parse_gfp_flags(sample: *mut perf_sample, gfp_flags: c_uint) -> c_int {
    static int parse_gfp_flags(struct perf_sample *sample, unsigned int gfp_flags)
    {
    struct tep_record record = {
    .cpu = sample.cpu,
    .data = sample.raw_data,
    .size = sample.raw_size,
    };
    struct trace_seq seq;
    char *str, *pos = core::ptr::null_mut();
    const struct tep_event *tp_format;
    if (nr_gfps) {
    struct gfp_flag key = {
    .flags = gfp_flags,
    };
    if (bsearch(&key, gfps, nr_gfps, sizeof(*gfps), gfpcmp))
    return 0;
    }
    trace_seq_init(&seq);
    tp_format = evsel__tp_format(sample.evsel);
    if (tp_format)
    tep_print_event(tp_format.tep, &seq, &record, "%s", TEP_PRINT_INFO);
    str = strtok_r(seq.buffer, " ", &pos);
    while (str) {
    if (!strncmp(str, "gfp_flags=", 10)) {
    struct gfp_flag *new;
    new = realloc(gfps, (nr_gfps + 1) * sizeof(*gfps));
    if (new == core::ptr::null_mut())
    goto err_out;
    gfps = new;
    new += nr_gfps;
    new.flags = gfp_flags;
    new.human_readable = strdup(str + 10);
    if (!new.human_readable)
    goto err_out;
    new.compact_str = compact_gfp_flags(str + 10);
    if (!new.compact_str) {
    free(new.human_readable);
    goto err_out;
    }
    nr_gfps++;
    qsort(gfps, nr_gfps, sizeof(*gfps), gfpcmp);
    }
    str = strtok_r(core::ptr::null_mut(), " ", &pos);
    }
    trace_seq_destroy(&seq);
    return 0;
    err_out:
    trace_seq_destroy(&seq);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn evsel__process_page_alloc_event(sample: *mut perf_sample) -> c_int {
    static int evsel__process_page_alloc_event(struct perf_sample *sample)
    {
    u64 page;
    let mut order: c_uint = perf_sample__intval(sample, "order");
    let mut gfp_flags: c_uint = perf_sample__intval(sample, "gfp_flags");
    let mut migrate_type: c_uint = perf_sample__intval(sample, "migratetype");
    let mut bytes: u64 = kmem_page_size << order;
    u64 callsite;
    struct page_stat *pstat;
    struct page_stat this = {
    .order = order,
    .gfp_flags = gfp_flags,
    .migrate_type = migrate_type,
    };
    if (order >= MAX_PAGE_ORDER) {
    pr_debug("Out-of-bounds order %u\n", order);
    return -1;
    }
    if (migrate_type >= MAX_MIGRATE_TYPES) {
    pr_debug("Out-of-bounds migratetype %u\n", migrate_type);
    return -1;
    }
    if (use_pfn)
    page = perf_sample__intval(sample, "pfn");
    else
    page = perf_sample__intval(sample, "page");
    nr_page_allocs++;
    total_page_alloc_bytes += bytes;
    if (!valid_page(page)) {
    nr_page_fails++;
    total_page_fail_bytes += bytes;
    return 0;
    }
    if (parse_gfp_flags(sample, gfp_flags) < 0)
    return -1;
    callsite = find_callsite(sample);
//
// This is to find the current page (with correct gfp flags and
// migrate type) at free event.
//
    this.page = page;
    pstat = page_stat__findnew_page(&this);
    if (pstat == core::ptr::null_mut())
    return -ENOMEM;
    pstat.nr_alloc++;
    pstat.alloc_bytes += bytes;
    pstat.callsite = callsite;
    if (!live_page) {
    pstat = page_stat__findnew_alloc(&this);
    if (pstat == core::ptr::null_mut())
    return -ENOMEM;
    pstat.nr_alloc++;
    pstat.alloc_bytes += bytes;
    pstat.callsite = callsite;
    }
    this.callsite = callsite;
    pstat = page_stat__findnew_caller(&this);
    if (pstat == core::ptr::null_mut())
    return -ENOMEM;
    pstat.nr_alloc++;
    pstat.alloc_bytes += bytes;
    order_stats[order][migrate_type]++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn evsel__process_page_free_event(sample: *mut perf_sample) -> c_int {
    static int evsel__process_page_free_event(struct perf_sample *sample)
    {
    u64 page;
    let mut order: c_uint = perf_sample__intval(sample, "order");
    let mut bytes: u64 = kmem_page_size << order;
    struct page_stat *pstat;
    struct page_stat this = {
    .order = order,
    };
    if (order >= MAX_PAGE_ORDER) {
    pr_debug("Out-of-bounds order %u\n", order);
    return -1;
    }
    if (use_pfn)
    page = perf_sample__intval(sample, "pfn");
    else
    page = perf_sample__intval(sample, "page");
    nr_page_frees++;
    total_page_free_bytes += bytes;
    this.page = page;
    pstat = page_stat__find_page(&this);
    if (pstat == core::ptr::null_mut()) {
    pr_debug2("missing free at page %"PRIx64" (order: %d)\n",
    page, order);
    nr_page_nomatch++;
    total_page_nomatch_bytes += bytes;
    return 0;
    }
    this.gfp_flags = pstat.gfp_flags;
    this.migrate_type = pstat.migrate_type;
    this.callsite = pstat.callsite;
    rb_erase(&pstat.node, &page_live_tree);
    free(pstat);
    if (live_page) {
    order_stats[this.order][this.migrate_type]--;
    } else {
    pstat = page_stat__find_alloc(&this);
    if (pstat == core::ptr::null_mut())
    return -ENOMEM;
    pstat.nr_free++;
    pstat.free_bytes += bytes;
    }
    pstat = page_stat__find_caller(&this);
    if (pstat == core::ptr::null_mut())
    return -ENOENT;
    pstat.nr_free++;
    pstat.free_bytes += bytes;
    if (live_page) {
    pstat.nr_alloc--;
    pstat.alloc_bytes -= bytes;
    if (pstat.nr_alloc == 0) {
    rb_erase(&pstat.node, &page_caller_tree);
    free(pstat);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn perf_kmem__skip_sample(sample: *mut perf_sample) -> bool {
    static bool perf_kmem__skip_sample(struct perf_sample *sample)
    {
// skip sample based on time?
    if (perf_time__skip_sample(&ptime, sample.time))
    return true;
    return false;
    }
    typedef int (*tracepoint_handler)(struct perf_sample *sample);
    static int process_sample_event(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample,
    struct machine *machine)
    {
    struct evsel *evsel = sample.evsel;
    let mut err: c_int = 0;
    struct thread *thread = machine__findnew_thread(machine, sample.pid,
    sample.tid);
    if (thread == core::ptr::null_mut()) {
    pr_debug("problem processing %s (%u) event at offset %#" PRIx64 ", skipping it.\n",
    perf_event__name(event.header.type), event.header.type,
    sample.file_offset);
    return -1;
    }
    if (perf_kmem__skip_sample(sample)) {
    thread__put(thread);
    return 0;
    }
    dump_printf(" ... thread: %s:%d\n", thread__comm_str(thread), thread__tid(thread));
    if (evsel.handler != core::ptr::null_mut()) {
    let mut f: tracepoint_handler = evsel.handler;
    err = f(sample);
    }
    thread__put(thread);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fragmentation(n_req: c_ulong, n_alloc: c_ulong) -> double {
    static double fragmentation(unsigned long n_req, unsigned long n_alloc)
    {
    if (n_alloc == 0)
    return 0.0;
    else
    return 100.0 - (100.0 * n_req / n_alloc);
    }
    static void __print_slab_result(struct rb_root *root,
    struct perf_session *session,
    int n_lines, int is_caller)
    {
    struct rb_node *next;
    struct machine *machine = &session.machines.host;
    printf("%.105s\n", graph_dotted_line);
    printf(" %-34s |",  is_caller ? "Callsite": "Alloc Ptr");
    printf(" Total_alloc/Per | Total_req/Per   | Hit      | Ping-pong | Frag\n");
    printf("%.105s\n", graph_dotted_line);
    next = rb_first(root);
    while (next && n_lines--) {
    struct alloc_stat *data = rb_entry(next, struct alloc_stat,
    node);
    struct symbol *sym = core::ptr::null_mut();
    struct map *map;
    char buf[BUFSIZ];
    u64 addr;
    if (is_caller) {
    addr = data.call_site;
    if (!raw_ip)
    sym = machine__find_kernel_symbol(machine, addr, &map);
    } else
    addr = data.ptr;
    if (sym != core::ptr::null_mut())
    snprintf(buf, sizeof(buf), "%s+%" PRIx64 "", sym.name,
    addr - map__unmap_ip(map, sym.start));
    else
    snprintf(buf, sizeof(buf), "%#" PRIx64 "", addr);
    printf(" %-34s |", buf);
    printf(" %9llu/%-5lu | %9llu/%-5lu | %8lu | %9lu | %6.3f%%\n",
    (unsigned long long)data.bytes_alloc,
    (unsigned long)data.bytes_alloc / data.hit,
    (unsigned long long)data.bytes_req,
    (unsigned long)data.bytes_req / data.hit,
    (unsigned long)data.hit,
    (unsigned long)data.pingpong,
    fragmentation(data.bytes_req, data.bytes_alloc));
    next = rb_next(next);
    }
    if (n_lines == -1)
    printf(" ...                                | ...             | ...             | ...      | ...       | ...   \n");
    printf("%.105s\n", graph_dotted_line);
    }
    static const char * const migrate_type_str[] = {
    "UNMOVABL",
    "RECLAIM",
    "MOVABLE",
    "RESERVED",
    "CMA/ISLT",
    "UNKNOWN",
    };
#[no_mangle]
unsafe extern "C" fn __print_page_alloc_result(session: *mut perf_session, n_lines: c_int) {
    static void __print_page_alloc_result(struct perf_session *session, int n_lines)
    {
    struct rb_node *next = rb_first(&page_alloc_sorted);
    struct machine *machine = &session.machines.host;
    const char *format;
    let mut gfp_len: c_int = max(strlen("GFP flags"), max_gfp_len);
    printf("\n%.105s\n", graph_dotted_line);
    printf(" %-16s | %5s alloc (KB) | Hits      | Order | Mig.type | %-*s | Callsite\n",
    use_pfn ? "PFN" : "Page", live_page ? "Live" : "Total",
    gfp_len, "GFP flags");
    printf("%.105s\n", graph_dotted_line);
    if (use_pfn)
    format = " %16llu | %'16llu | %'9d | %5d | %8s | %-*s | %s\n";
    else
    format = " %016llx | %'16llu | %'9d | %5d | %8s | %-*s | %s\n";
    while (next && n_lines--) {
    struct page_stat *data;
    struct symbol *sym;
    struct map *map;
    char buf[32];
    char *caller = buf;
    data = rb_entry(next, struct page_stat, node);
    sym = machine__find_kernel_symbol(machine, data.callsite, &map);
    if (sym)
    caller = sym.name;
    else
    scnprintf(buf, sizeof(buf), "%"PRIx64, data.callsite);
    printf(format, (unsigned long long)data.page,
    (unsigned long long)data.alloc_bytes / 1024,
    data.nr_alloc, data.order,
    migrate_type_str[data.migrate_type],
    gfp_len, compact_gfp_string(data.gfp_flags), caller);
    next = rb_next(next);
    }
    if (n_lines == -1) {
    printf(" ...              | ...              | ...       | ...   | ...      | %-*s | ...\n",
    gfp_len, "...");
    }
    printf("%.105s\n", graph_dotted_line);
    }
#[no_mangle]
unsafe extern "C" fn __print_page_caller_result(session: *mut perf_session, n_lines: c_int) {
    static void __print_page_caller_result(struct perf_session *session, int n_lines)
    {
    struct rb_node *next = rb_first(&page_caller_sorted);
    struct machine *machine = &session.machines.host;
    let mut gfp_len: c_int = max(strlen("GFP flags"), max_gfp_len);
    printf("\n%.105s\n", graph_dotted_line);
    printf(" %5s alloc (KB) | Hits      | Order | Mig.type | %-*s | Callsite\n",
    live_page ? "Live" : "Total", gfp_len, "GFP flags");
    printf("%.105s\n", graph_dotted_line);
    while (next && n_lines--) {
    struct page_stat *data;
    struct symbol *sym;
    struct map *map;
    char buf[32];
    char *caller = buf;
    data = rb_entry(next, struct page_stat, node);
    sym = machine__find_kernel_symbol(machine, data.callsite, &map);
    if (sym)
    caller = sym.name;
    else
    scnprintf(buf, sizeof(buf), "%"PRIx64, data.callsite);
    printf(" %'16llu | %'9d | %5d | %8s | %-*s | %s\n",
    (unsigned long long)data.alloc_bytes / 1024,
    data.nr_alloc, data.order,
    migrate_type_str[data.migrate_type],
    gfp_len, compact_gfp_string(data.gfp_flags), caller);
    next = rb_next(next);
    }
    if (n_lines == -1) {
    printf(" ...              | ...       | ...   | ...      | %-*s | ...\n",
    gfp_len, "...");
    }
    printf("%.105s\n", graph_dotted_line);
    }
#[no_mangle]
unsafe extern "C" fn print_gfp_flags() {
    static void print_gfp_flags(void)
    {
    int i;
    printf("#\n");
    printf("# GFP flags\n");
    printf("# ---------\n");
    for (i = 0; i < nr_gfps; i++) {
    printf("# %08x: %*s: %s\n", gfps[i].flags,
    (int) max_gfp_len, gfps[i].compact_str,
    gfps[i].human_readable);
    }
    }
#[no_mangle]
unsafe extern "C" fn print_slab_summary() {
    static void print_slab_summary(void)
    {
    printf("\nSUMMARY (SLAB allocator)");
    printf("\n========================\n");
    printf("Total bytes requested: %'lu\n", total_requested);
    printf("Total bytes allocated: %'lu\n", total_allocated);
    printf("Total bytes freed:     %'lu\n", total_freed);
    if (total_allocated > total_freed) {
    printf("Net total bytes allocated: %'lu\n",
    total_allocated - total_freed);
    }
    printf("Total bytes wasted on internal fragmentation: %'lu\n",
    total_allocated - total_requested);
    printf("Internal fragmentation: %f%%\n",
    fragmentation(total_requested, total_allocated));
    printf("Cross CPU allocations: %'lu/%'lu\n", nr_cross_allocs, nr_allocs);
    }
#[no_mangle]
unsafe extern "C" fn print_page_summary() {
    static void print_page_summary(void)
    {
    int o, m;
    let mut nr_alloc_freed: u64 = nr_page_frees - nr_page_nomatch;
    let mut total_alloc_freed_bytes: u64 = total_page_free_bytes - total_page_nomatch_bytes;
    printf("\nSUMMARY (page allocator)");
    printf("\n========================\n");
    printf("%-30s: %'16lu   [ %'16"PRIu64" KB ]\n", "Total allocation requests",
    nr_page_allocs, total_page_alloc_bytes / 1024);
    printf("%-30s: %'16lu   [ %'16"PRIu64" KB ]\n", "Total free requests",
    nr_page_frees, total_page_free_bytes / 1024);
    printf("\n");
    printf("%-30s: %'16"PRIu64"   [ %'16"PRIu64" KB ]\n", "Total alloc+freed requests",
    nr_alloc_freed, (total_alloc_freed_bytes) / 1024);
    printf("%-30s: %'16"PRIu64"   [ %'16"PRIu64" KB ]\n", "Total alloc-only requests",
    nr_page_allocs - nr_alloc_freed,
    (total_page_alloc_bytes - total_alloc_freed_bytes) / 1024);
    printf("%-30s: %'16lu   [ %'16"PRIu64" KB ]\n", "Total free-only requests",
    nr_page_nomatch, total_page_nomatch_bytes / 1024);
    printf("\n");
    printf("%-30s: %'16lu   [ %'16"PRIu64" KB ]\n", "Total allocation failures",
    nr_page_fails, total_page_fail_bytes / 1024);
    printf("\n");
    printf("%5s  %12s  %12s  %12s  %12s  %12s\n", "Order",  "Unmovable",
    "Reclaimable", "Movable", "Reserved", "CMA/Isolated");
    printf("%.5s  %.12s  %.12s  %.12s  %.12s  %.12s\n", graph_dotted_line,
    graph_dotted_line, graph_dotted_line, graph_dotted_line,
    graph_dotted_line, graph_dotted_line);
    for (o = 0; o < MAX_PAGE_ORDER; o++) {
    printf("%5d", o);
    for (m = 0; m < MAX_MIGRATE_TYPES - 1; m++) {
    if (order_stats[o][m])
    printf("  %'12d", order_stats[o][m]);
    else
    printf("  %12c", '.');
    }
    printf("\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn print_slab_result(session: *mut perf_session) {
    static void print_slab_result(struct perf_session *session)
    {
    if (caller_flag)
    __print_slab_result(&root_caller_sorted, session, caller_lines, 1);
    if (alloc_flag)
    __print_slab_result(&root_alloc_sorted, session, alloc_lines, 0);
    print_slab_summary();
    }
#[no_mangle]
unsafe extern "C" fn print_page_result(session: *mut perf_session) {
    static void print_page_result(struct perf_session *session)
    {
    if (caller_flag || alloc_flag)
    print_gfp_flags();
    if (caller_flag)
    __print_page_caller_result(session, caller_lines);
    if (alloc_flag)
    __print_page_alloc_result(session, alloc_lines);
    print_page_summary();
    }
#[no_mangle]
unsafe extern "C" fn print_result(session: *mut perf_session) {
    static void print_result(struct perf_session *session)
    {
    if (kmem_slab)
    print_slab_result(session);
    if (kmem_page)
    print_page_result(session);
    }
    static LIST_HEAD(slab_caller_sort);
    static LIST_HEAD(slab_alloc_sort);
    static LIST_HEAD(page_caller_sort);
    static LIST_HEAD(page_alloc_sort);
    static void sort_slab_insert(struct rb_root *root, struct alloc_stat *data,
    struct list_head *sort_list)
    {
    struct rb_node **new = &(root.rb_node);
    struct rb_node *parent = core::ptr::null_mut();
    struct sort_dimension *sort;
    while (*new) {
    struct alloc_stat *this;
    let mut cmp: c_int = 0;
    this = rb_entry(*new, struct alloc_stat, node);
    parent = *new;
    list_for_each_entry(sort, sort_list, list) {
    cmp = sort.cmp(data, this);
    if (cmp)
    break;
    }
    if (cmp > 0)
    new = &((*new).rb_left);
    else
    new = &((*new).rb_right);
    }
    rb_link_node(&data.node, parent, new);
    rb_insert_color(&data.node, root);
    }
    static void __sort_slab_result(struct rb_root *root, struct rb_root *root_sorted,
    struct list_head *sort_list)
    {
    struct rb_node *node;
    struct alloc_stat *data;
    for (;;) {
    node = rb_first(root);
    if (!node)
    break;
    rb_erase(node, root);
    data = rb_entry(node, struct alloc_stat, node);
    sort_slab_insert(root_sorted, data, sort_list);
    }
    }
    static void sort_page_insert(struct rb_root *root, struct page_stat *data,
    struct list_head *sort_list)
    {
    struct rb_node **new = &root.rb_node;
    struct rb_node *parent = core::ptr::null_mut();
    struct sort_dimension *sort;
    while (*new) {
    struct page_stat *this;
    let mut cmp: c_int = 0;
    this = rb_entry(*new, struct page_stat, node);
    parent = *new;
    list_for_each_entry(sort, sort_list, list) {
    cmp = sort.cmp(data, this);
    if (cmp)
    break;
    }
    if (cmp > 0)
    new = &parent.rb_left;
    else
    new = &parent.rb_right;
    }
    rb_link_node(&data.node, parent, new);
    rb_insert_color(&data.node, root);
    }
    static void __sort_page_result(struct rb_root *root, struct rb_root *root_sorted,
    struct list_head *sort_list)
    {
    struct rb_node *node;
    struct page_stat *data;
    for (;;) {
    node = rb_first(root);
    if (!node)
    break;
    rb_erase(node, root);
    data = rb_entry(node, struct page_stat, node);
    sort_page_insert(root_sorted, data, sort_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn sort_result() {
    static void sort_result(void)
    {
    if (kmem_slab) {
    __sort_slab_result(&root_alloc_stat, &root_alloc_sorted,
    &slab_alloc_sort);
    __sort_slab_result(&root_caller_stat, &root_caller_sorted,
    &slab_caller_sort);
    }
    if (kmem_page) {
    if (live_page)
    __sort_page_result(&page_live_tree, &page_alloc_sorted,
    &page_alloc_sort);
    else
    __sort_page_result(&page_alloc_tree, &page_alloc_sorted,
    &page_alloc_sort);
    __sort_page_result(&page_caller_tree, &page_caller_sorted,
    &page_caller_sort);
    }
    }
#[no_mangle]
unsafe extern "C" fn __cmd_kmem(session: *mut perf_session) -> c_int {
    static int __cmd_kmem(struct perf_session *session)
    {
    let mut err: c_int = -EINVAL;
    struct evsel *evsel;
    const struct evsel_str_handler kmem_tracepoints[] = {
// slab allocator
    { "kmem:kmalloc",		evsel__process_alloc_event, },
    { "kmem:kmem_cache_alloc",	evsel__process_alloc_event, },
    { "kmem:kmalloc_node",		evsel__process_alloc_event, },
    { "kmem:kmem_cache_alloc_node", evsel__process_alloc_event, },
    { "kmem:kfree",			evsel__process_free_event, },
    { "kmem:kmem_cache_free",	evsel__process_free_event, },
// page allocator
    { "kmem:mm_page_alloc",		evsel__process_page_alloc_event, },
    { "kmem:mm_page_free",		evsel__process_page_free_event, },
    };
    if (!perf_session__has_traces(session, "kmem record"))
    goto out;
    if (perf_session__set_tracepoints_handlers(session, kmem_tracepoints)) {
    pr_err("Initializing perf session tracepoint handlers failed\n");
    goto out;
    }
    evlist__for_each_entry(session.evlist, evsel) {
    if (evsel__name_is(evsel, "kmem:mm_page_alloc") &&
    evsel__field(evsel, "pfn")) {
    use_pfn = true;
    break;
    }
    }
    setup_pager();
    err = perf_session__process_events(session);
    if (err != 0) {
    pr_err("error during process events: %d\n", err);
    goto out;
    }
    sort_result();
    print_result(session);
    out:
    return err;
    }
// slab sort keys
#[no_mangle]
unsafe extern "C" fn ptr_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int ptr_cmp(void *a, void *b)
    {
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    if (l.ptr < r.ptr)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->ptr: l->ptr >) -> else {
    else if (l.ptr > r.ptr)
    return 1;
    return 0;
    }
    static struct sort_dimension ptr_sort_dimension = {
    .name	= "ptr",
    .cmp	= ptr_cmp,
    };
#[no_mangle]
unsafe extern "C" fn slab_callsite_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int slab_callsite_cmp(void *a, void *b)
    {
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    if (l.call_site < r.call_site)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->call_site: l->call_site >) -> else {
    else if (l.call_site > r.call_site)
    return 1;
    return 0;
    }
    static struct sort_dimension callsite_sort_dimension = {
    .name	= "callsite",
    .cmp	= slab_callsite_cmp,
    };
#[no_mangle]
unsafe extern "C" fn hit_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int hit_cmp(void *a, void *b)
    {
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    if (l.hit < r.hit)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->hit: l->hit >) -> else {
    else if (l.hit > r.hit)
    return 1;
    return 0;
    }
    static struct sort_dimension hit_sort_dimension = {
    .name	= "hit",
    .cmp	= hit_cmp,
    };
#[no_mangle]
unsafe extern "C" fn bytes_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int bytes_cmp(void *a, void *b)
    {
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    if (l.bytes_alloc < r.bytes_alloc)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->bytes_alloc: l->bytes_alloc >) -> else {
    else if (l.bytes_alloc > r.bytes_alloc)
    return 1;
    return 0;
    }
    static struct sort_dimension bytes_sort_dimension = {
    .name	= "bytes",
    .cmp	= bytes_cmp,
    };
#[no_mangle]
unsafe extern "C" fn frag_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int frag_cmp(void *a, void *b)
    {
    double x, y;
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    x = fragmentation(l.bytes_req, l.bytes_alloc);
    y = fragmentation(r.bytes_req, r.bytes_alloc);
    if (x < y)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(y: x >) -> else {
    else if (x > y)
    return 1;
    return 0;
    }
    static struct sort_dimension frag_sort_dimension = {
    .name	= "frag",
    .cmp	= frag_cmp,
    };
#[no_mangle]
unsafe extern "C" fn pingpong_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int pingpong_cmp(void *a, void *b)
    {
    struct alloc_stat *l = a;
    struct alloc_stat *r = b;
    if (l.pingpong < r.pingpong)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->pingpong: l->pingpong >) -> else {
    else if (l.pingpong > r.pingpong)
    return 1;
    return 0;
    }
    static struct sort_dimension pingpong_sort_dimension = {
    .name	= "pingpong",
    .cmp	= pingpong_cmp,
    };
// page sort keys
#[no_mangle]
unsafe extern "C" fn page_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int page_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
    if (l.page < r.page)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->page: l->page >) -> else {
    else if (l.page > r.page)
    return 1;
    return 0;
    }
    static struct sort_dimension page_sort_dimension = {
    .name	= "page",
    .cmp	= page_cmp,
    };
#[no_mangle]
unsafe extern "C" fn page_callsite_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int page_callsite_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
    if (l.callsite < r.callsite)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->callsite: l->callsite >) -> else {
    else if (l.callsite > r.callsite)
    return 1;
    return 0;
    }
    static struct sort_dimension page_callsite_sort_dimension = {
    .name	= "callsite",
    .cmp	= page_callsite_cmp,
    };
#[no_mangle]
unsafe extern "C" fn page_hit_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int page_hit_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
    if (l.nr_alloc < r.nr_alloc)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->nr_alloc: l->nr_alloc >) -> else {
    else if (l.nr_alloc > r.nr_alloc)
    return 1;
    return 0;
    }
    static struct sort_dimension page_hit_sort_dimension = {
    .name	= "hit",
    .cmp	= page_hit_cmp,
    };
#[no_mangle]
unsafe extern "C" fn page_bytes_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int page_bytes_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
    if (l.alloc_bytes < r.alloc_bytes)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->alloc_bytes: l->alloc_bytes >) -> else {
    else if (l.alloc_bytes > r.alloc_bytes)
    return 1;
    return 0;
    }
    static struct sort_dimension page_bytes_sort_dimension = {
    .name	= "bytes",
    .cmp	= page_bytes_cmp,
    };
#[no_mangle]
unsafe extern "C" fn page_order_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int page_order_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
    if (l.order < r.order)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->order: l->order >) -> else {
    else if (l.order > r.order)
    return 1;
    return 0;
    }
    static struct sort_dimension page_order_sort_dimension = {
    .name	= "order",
    .cmp	= page_order_cmp,
    };
#[no_mangle]
unsafe extern "C" fn migrate_type_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int migrate_type_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
// for internal use to find free'd page
    if (l.migrate_type == -1U)
    return 0;
    if (l.migrate_type < r.migrate_type)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->migrate_type: l->migrate_type >) -> else {
    else if (l.migrate_type > r.migrate_type)
    return 1;
    return 0;
    }
    static struct sort_dimension migrate_type_sort_dimension = {
    .name	= "migtype",
    .cmp	= migrate_type_cmp,
    };
#[no_mangle]
unsafe extern "C" fn gfp_flags_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    static int gfp_flags_cmp(void *a, void *b)
    {
    struct page_stat *l = a;
    struct page_stat *r = b;
// for internal use to find free'd page
    if (l.gfp_flags == -1U)
    return 0;
    if (l.gfp_flags < r.gfp_flags)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r->gfp_flags: l->gfp_flags >) -> else {
    else if (l.gfp_flags > r.gfp_flags)
    return 1;
    return 0;
    }
    static struct sort_dimension gfp_flags_sort_dimension = {
    .name	= "gfp",
    .cmp	= gfp_flags_cmp,
    };
    static struct sort_dimension *slab_sorts[] = {
    &ptr_sort_dimension,
    &callsite_sort_dimension,
    &hit_sort_dimension,
    &bytes_sort_dimension,
    &frag_sort_dimension,
    &pingpong_sort_dimension,
    };
    static struct sort_dimension *page_sorts[] = {
    &page_sort_dimension,
    &page_callsite_sort_dimension,
    &page_hit_sort_dimension,
    &page_bytes_sort_dimension,
    &page_order_sort_dimension,
    &migrate_type_sort_dimension,
    &gfp_flags_sort_dimension,
    };
#[no_mangle]
unsafe extern "C" fn slab_sort_dimension__add(tok: *const c_char, list: *mut list_head) -> c_int {
    static int slab_sort_dimension__add(const char *tok, struct list_head *list)
    {
    struct sort_dimension *sort;
    int i;
    for (i = 0; i < (int)ARRAY_SIZE(slab_sorts); i++) {
    if (!strcmp(slab_sorts[i].name, tok)) {
    sort = memdup(slab_sorts[i], sizeof(*slab_sorts[i]));
    if (!sort) {
    pr_err("%s: memdup failed\n", __func__);
    return -1;
    }
    list_add_tail(&sort.list, list);
    return 0;
    }
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn page_sort_dimension__add(tok: *const c_char, list: *mut list_head) -> c_int {
    static int page_sort_dimension__add(const char *tok, struct list_head *list)
    {
    struct sort_dimension *sort;
    int i;
    for (i = 0; i < (int)ARRAY_SIZE(page_sorts); i++) {
    if (!strcmp(page_sorts[i].name, tok)) {
    sort = memdup(page_sorts[i], sizeof(*page_sorts[i]));
    if (!sort) {
    pr_err("%s: memdup failed\n", __func__);
    return -1;
    }
    list_add_tail(&sort.list, list);
    return 0;
    }
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn setup_slab_sorting(sort_list: *mut list_head, arg: *const c_char) -> c_int {
    static int setup_slab_sorting(struct list_head *sort_list, const char *arg)
    {
    char *tok;
    char *str = strdup(arg);
    char *pos = str;
    if (!str) {
    pr_err("%s: strdup failed\n", __func__);
    return -1;
    }
    while (true) {
    tok = strsep(&pos, ",");
    if (!tok)
    break;
    if (slab_sort_dimension__add(tok, sort_list) < 0) {
    pr_err("Unknown slab --sort key: '%s'", tok);
    free(str);
    return -1;
    }
    }
    free(str);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_page_sorting(sort_list: *mut list_head, arg: *const c_char) -> c_int {
    static int setup_page_sorting(struct list_head *sort_list, const char *arg)
    {
    char *tok;
    char *str = strdup(arg);
    char *pos = str;
    if (!str) {
    pr_err("%s: strdup failed\n", __func__);
    return -1;
    }
    while (true) {
    tok = strsep(&pos, ",");
    if (!tok)
    break;
    if (page_sort_dimension__add(tok, sort_list) < 0) {
    pr_err("Unknown page --sort key: '%s'", tok);
    free(str);
    return -1;
    }
    }
    free(str);
    return 0;
    }
    static int parse_sort_opt(const struct option *opt __maybe_unused,
    const char *arg, int unset __maybe_unused)
    {
    if (!arg)
    return -1;
    if (kmem_page > kmem_slab ||
    (kmem_page == 0 && kmem_slab == 0 && kmem_default == KMEM_PAGE)) {
    if (caller_flag > alloc_flag)
    return setup_page_sorting(&page_caller_sort, arg);
    else
    return setup_page_sorting(&page_alloc_sort, arg);
    } else {
    if (caller_flag > alloc_flag)
    return setup_slab_sorting(&slab_caller_sort, arg);
    else
    return setup_slab_sorting(&slab_alloc_sort, arg);
    }
    return 0;
    }
    static int parse_caller_opt(const struct option *opt __maybe_unused,
    const char *arg __maybe_unused,
    int unset __maybe_unused)
    {
    caller_flag = (alloc_flag + 1);
    return 0;
    }
    static int parse_alloc_opt(const struct option *opt __maybe_unused,
    const char *arg __maybe_unused,
    int unset __maybe_unused)
    {
    alloc_flag = (caller_flag + 1);
    return 0;
    }
    static int parse_slab_opt(const struct option *opt __maybe_unused,
    const char *arg __maybe_unused,
    int unset __maybe_unused)
    {
    kmem_slab = (kmem_page + 1);
    return 0;
    }
    static int parse_page_opt(const struct option *opt __maybe_unused,
    const char *arg __maybe_unused,
    int unset __maybe_unused)
    {
    kmem_page = (kmem_slab + 1);
    return 0;
    }
    static int parse_line_opt(const struct option *opt __maybe_unused,
    const char *arg, int unset __maybe_unused)
    {
    int lines;
    if (!arg)
    return -1;
    lines = strtoul(arg, core::ptr::null_mut(), 10);
    if (caller_flag > alloc_flag)
    caller_lines = lines;
    else
    alloc_lines = lines;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_legacy_tp_is_exposed() -> bool {
    static bool slab_legacy_tp_is_exposed(void)
    {
//
// The tracepoints "kmem:kmalloc_node" and
// "kmem:kmem_cache_alloc_node" have been removed on the latest
// kernel, if the tracepoint "kmem:kmalloc_node" is existed it
// means the tool is running on an old kernel, we need to
// rollback to support these legacy tracepoints.
//
    return IS_ERR(trace_event__tp_format("kmem", "kmalloc_node")) ?
    false : true;
    }
#[no_mangle]
unsafe extern "C" fn __cmd_record(argc: c_int, argv: *const c_char) -> c_int {
    static int __cmd_record(int argc, const char **argv)
    {
    const char * const record_args[] = {
    "record", "-a", "-R", "-c", "1",
    };
    const char * const slab_events[] = {
    "-e", "kmem:kmalloc",
    "-e", "kmem:kfree",
    "-e", "kmem:kmem_cache_alloc",
    "-e", "kmem:kmem_cache_free",
    };
    const char * const slab_legacy_events[] = {
    "-e", "kmem:kmalloc_node",
    "-e", "kmem:kmem_cache_alloc_node",
    };
    const char * const page_events[] = {
    "-e", "kmem:mm_page_alloc",
    "-e", "kmem:mm_page_free",
    };
    unsigned int rec_argc, i, j;
    const char **rec_argv;
    let mut slab_legacy_tp_exposed: c_uint = slab_legacy_tp_is_exposed();
    rec_argc = ARRAY_SIZE(record_args) + argc - 1;
    if (kmem_slab) {
    rec_argc += ARRAY_SIZE(slab_events);
    if (slab_legacy_tp_exposed)
    rec_argc += ARRAY_SIZE(slab_legacy_events);
    }
    if (kmem_page)
    rec_argc += ARRAY_SIZE(page_events) + 1; /* for -g */
    rec_argv = calloc(rec_argc + 1, sizeof(char *));
    if (rec_argv == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(record_args); i++)
    rec_argv[i] = strdup(record_args[i]);
    if (kmem_slab) {
    for (j = 0; j < ARRAY_SIZE(slab_events); j++, i++)
    rec_argv[i] = strdup(slab_events[j]);
    if (slab_legacy_tp_exposed) {
    for (j = 0; j < ARRAY_SIZE(slab_legacy_events); j++, i++)
    rec_argv[i] = strdup(slab_legacy_events[j]);
    }
    }
    if (kmem_page) {
    rec_argv[i++] = strdup("-g");
    for (j = 0; j < ARRAY_SIZE(page_events); j++, i++)
    rec_argv[i] = strdup(page_events[j]);
    }
    for (j = 1; j < (unsigned int)argc; j++, i++)
    rec_argv[i] = argv[j];
    return cmd_record(i, rec_argv);
    }
#[no_mangle]
unsafe extern "C" fn kmem_config(var: *const c_char, value: *const c_char, __maybe_unused: *mut *mut void cb) -> c_int {
    static int kmem_config(const char *var, const char *value, void *cb __maybe_unused)
    {
    if (!strcmp(var, "kmem.default")) {
    if (!strcmp(value, "slab"))
    kmem_default = KMEM_SLAB;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(value, _arg: "page")) -> else {
    else if (!strcmp(value, "page"))
    kmem_default = KMEM_PAGE;
    else
    pr_err("invalid default value ('slab' or 'page' required): %s\n",
    value);
    return 0;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_kmem(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_kmem(int argc, const char **argv)
    {
    let mut default_slab_sort: *const char  const = "frag,hit,bytes";
    let mut default_page_sort: *const char  const = "bytes,hit";
    struct perf_data data = {
    .mode = PERF_DATA_MODE_READ,
    };
    const struct option kmem_options[] = {
    OPT_STRING('i', "input", &input_name, "file", "input file name"),
    OPT_INCR('v', "verbose", &verbose,
    "be more verbose (show symbol address, etc)"),
    OPT_CALLBACK_NOOPT(0, "caller", core::ptr::null_mut(), core::ptr::null_mut(),
    "show per-callsite statistics", parse_caller_opt),
    OPT_CALLBACK_NOOPT(0, "alloc", core::ptr::null_mut(), core::ptr::null_mut(),
    "show per-allocation statistics", parse_alloc_opt),
    OPT_CALLBACK('s', "sort", core::ptr::null_mut(), "key[,key2...]",
    "sort by keys: ptr, callsite, bytes, hit, pingpong, frag, "
    "page, order, migtype, gfp", parse_sort_opt),
    OPT_CALLBACK('l', "line", core::ptr::null_mut(), "num", "show n lines", parse_line_opt),
    OPT_BOOLEAN(0, "raw-ip", &raw_ip, "show raw ip instead of symbol"),
    OPT_BOOLEAN('f', "force", &data.force, "don't complain, do it"),
    OPT_CALLBACK_NOOPT(0, "slab", core::ptr::null_mut(), core::ptr::null_mut(), "Analyze slab allocator",
    parse_slab_opt),
    OPT_CALLBACK_NOOPT(0, "page", core::ptr::null_mut(), core::ptr::null_mut(), "Analyze page allocator",
    parse_page_opt),
    OPT_BOOLEAN(0, "live", &live_page, "Show live page stat"),
    OPT_STRING(0, "time", &time_str, "str",
    "Time span of interest (start,stop)"),
    OPT_END()
    };
    const char *const kmem_subcommands[] = { "record", "stat", core::ptr::null_mut() };
    const char *kmem_usage[] = {
    core::ptr::null_mut(),
    core::ptr::null_mut()
    };
    struct perf_session *session;
    struct perf_tool perf_kmem;
    static const char errmsg[] = "No %s allocation events found.  Have you run 'perf kmem record --%s'?\n";
    let mut ret: c_int = perf_config(kmem_config, core::ptr::null_mut());
    if (ret)
    return ret;
    argc = parse_options_subcommand(argc, argv, kmem_options,
    kmem_subcommands, kmem_usage,
    PARSE_OPT_STOP_AT_NON_OPTION);
    if (!argc)
    usage_with_options(kmem_usage, kmem_options);
    if (kmem_slab == 0 && kmem_page == 0) {
    if (kmem_default == KMEM_SLAB)
    kmem_slab = 1;
    else
    kmem_page = 1;
    }
    if (strlen(argv[0]) > 2 && strstarts("record", argv[0])) {
    symbol__init(core::ptr::null_mut());
    return __cmd_record(argc, argv);
    }
    data.path = input_name;
    perf_tool__init(&perf_kmem, /*ordered_events=*/true);
    perf_kmem.sample	= process_sample_event;
    perf_kmem.comm		= perf_event__process_comm;
    perf_kmem.mmap		= perf_event__process_mmap;
    perf_kmem.mmap2		= perf_event__process_mmap2;
    perf_kmem.namespaces	= perf_event__process_namespaces;
    kmem_session = session = perf_session__new(&data, &perf_kmem);
    if (IS_ERR(session))
    return PTR_ERR(session);
    ret = -1;
    if (kmem_slab) {
    if (!evlist__find_tracepoint_by_name(session.evlist, "kmem:kmalloc")) {
    pr_err(errmsg, "slab", "slab");
    goto out_delete;
    }
    }
    if (kmem_page) {
    struct evsel *evsel = evlist__find_tracepoint_by_name(session.evlist, "kmem:mm_page_alloc");
    const struct tep_event *tp_format = evsel ? evsel__tp_format(evsel) : core::ptr::null_mut();
    if (tp_format == core::ptr::null_mut()) {
    pr_err(errmsg, "page", "page");
    goto out_delete;
    }
    kmem_page_size = tep_get_page_size(tp_format.tep);
    symbol_conf.use_callchain = true;
    }
    symbol__init(perf_session__env(session));
    if (perf_time__parse_str(&ptime, time_str) != 0) {
    pr_err("Invalid time string\n");
    ret = -EINVAL;
    goto out_delete;
    }
    if (!strcmp(argv[0], "stat")) {
    setlocale(LC_ALL, "");
    if (cpu__setup_cpunode_map())
    goto out_delete;
    if (list_empty(&slab_caller_sort))
    setup_slab_sorting(&slab_caller_sort, default_slab_sort);
    if (list_empty(&slab_alloc_sort))
    setup_slab_sorting(&slab_alloc_sort, default_slab_sort);
    if (list_empty(&page_caller_sort))
    setup_page_sorting(&page_caller_sort, default_page_sort);
    if (list_empty(&page_alloc_sort))
    setup_page_sorting(&page_alloc_sort, default_page_sort);
    if (kmem_page) {
    setup_page_sorting(&page_alloc_sort_input,
    "page,order,migtype,gfp");
    setup_page_sorting(&page_caller_sort_input,
    "callsite,order,migtype,gfp");
    }
    ret = __cmd_kmem(session);
    } else
    usage_with_options(kmem_usage, kmem_options);
    out_delete:
    perf_session__delete(session);
// free usage string allocated by parse_options_subcommand
    free((void *)kmem_usage[0]);
    return ret;
    }
