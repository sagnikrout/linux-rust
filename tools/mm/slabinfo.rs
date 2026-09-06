//! Automatically rewritten from C to Rust
//! Source: tools/mm/slabinfo.c
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
// Slabinfo: Tool to get reports about slabs
//
// (C) 2007 sgi, Christoph Lameter
// (C) 2011 Linux Foundation, Christoph Lameter
//
// Compile with:
//
// gcc -o slabinfo slabinfo.c
//

pub const MAX_SLABS: c_int = 2000;
pub const MAX_ALIASES: c_int = 500;
pub const MAX_NODES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slabinfo {
    pub name: *mut c_char,
    pub alias: c_int,
    pub refs: c_int,
    pub destroy_by_rcu: int aliases, align, cache_dma, cpu_slabs,,
    pub objs_per_slab: unsigned int hwcache_align, object_size,,
    pub trace: unsigned int sanity_checks, slab_size, store_user,,
    pub red_zone: int order, poison, reclaim_account,,
    pub total_objects: unsigned long partial, objects, slabs, objects_partial,,
    pub alloc_slowpath: unsigned long alloc_fastpath,,
    pub free_slowpath: unsigned long free_fastpath,,
    pub free_remove_partial: unsigned long free_frozen, free_add_partial,,
    pub alloc_refill: unsigned long alloc_from_partial, alloc_slab, free_slab,,
    pub deactivate_empty: unsigned long cpuslab_flush, deactivate_full,,
    pub deactivate_to_tail: unsigned long deactivate_to_head,,
    pub order_fallback: unsigned long deactivate_remote_frees,,
    pub cmpxchg_double_fail: unsigned long cmpxchg_double_cpu_fail,,
    pub deactivate_bypass: unsigned long alloc_node_mismatch,,
    pub cpu_partial_free: unsigned long cpu_partial_alloc,,
    pub numa: [c_int; MAX_NODES],
    pub numa_partial: [c_int; MAX_NODES],
    pub slabinfo: [}; MAX_SLABS],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aliasinfo {
    pub name: *mut c_char,
    pub ref: *mut c_char,
    pub slab: *mut slabinfo,
    pub aliasinfo: [}; MAX_ALIASES],
    pub slabs: c_int,
    pub actual_slabs: c_int,
    pub aliases: c_int,
    pub alias_targets: c_int,
    pub highest_node: c_int,
    pub buffer: [c_char; 4096],
    pub show_empty: c_int,
    pub show_report: c_int,
    pub show_alias: c_int,
    pub show_slab: c_int,
    pub 1: int skip_zero =,
    pub show_numa: c_int,
    pub show_track: c_int,
    pub show_first_alias: c_int,
    pub validate: c_int,
    pub shrink: c_int,
    pub show_inverted: c_int,
    pub show_single_ref: c_int,
    pub show_totals: c_int,
    pub sort_size: c_int,
    pub sort_active: c_int,
    pub set_debug: c_int,
    pub show_ops: c_int,
    pub sort_partial: c_int,
    pub show_activity: c_int,
    pub -1: int output_lines =,
    pub sort_loss: c_int,
    pub extended_totals: c_int,
    pub show_bytes: c_int,
    pub unreclaim_only: c_int,
// Debug options
    pub sanity: c_int,
    pub redzone: c_int,
    pub poison: c_int,
    pub tracking: c_int,
    pub tracing: c_int,
    pub page_size: c_int,
    pub pattern: regex_t,
#[no_mangle]
unsafe extern "C" fn fatal(x: *const c_char, ...) {
    static void fatal(const char *x, ...)
    {
    pub ap: va_list,
    pub x): va_start(ap,,
    pub ap): vfprintf(stderr, x,,
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("slabinfo 4/15/2011. (c) 2007 sgi/(c) 2011 Linux Foundation.\n\n"
    "slabinfo [-aABDefhilLnoPrsStTUvXz1] [N=K] [-dafzput] [slab-regexp]\n"
    "-a|--aliases           Show aliases\n"
    "-A|--activity          Most active slabs first\n"
    "-B|--Bytes             Show size in bytes\n"
    "-D|--display-active    Switch line format to activity\n"
    "-e|--empty             Show empty slabs\n"
    "-f|--first-alias       Show first alias\n"
    "-h|--help              Show usage information\n"
    "-i|--inverted          Inverted list\n"
    "-l|--slabs             Show slabs\n"
    "-L|--Loss              Sort by loss\n"
    "-n|--numa              Show NUMA information\n"
    "-N|--lines=K           Show the first K slabs\n"
    "-o|--ops               Show kmem_cache_ops\n"
    "-P|--partial           Sort by number of partial slabs\n"
    "-r|--report            Detailed report on single slabs\n"
    "-s|--shrink            Shrink slabs\n"
    "-S|--Size              Sort by size\n"
    "-t|--tracking          Show alloc/free information\n"
    "-T|--Totals            Show summary information\n"
    "-U|--Unreclaim         Show unreclaimable slabs only\n"
    "-v|--validate          Validate slabs\n"
    "-X|--Xtotals           Show extended summary information\n"
    "-z|--zero              Include empty slabs\n"
    "-1|--1ref              Single reference\n"
    "\n"
    "-d  | --debug          Switch off all debug options\n"
    "-da | --debug=a        Switch on all debug options (--debug=FZPU)\n"
    "\n"
    "-d[afzput] | --debug=[afzput]\n"
    "    f | F              Sanity Checks (SLAB_CONSISTENCY_CHECKS)\n"
    "    z | Z              Redzoning\n"
    "    p | P              Poisoning\n"
    "    u | U              Tracking\n"
    "    t | T              Tracing\n"
    "\nSorting options (--Loss, --Size, --Partial) are mutually exclusive\n"
    }
#[no_mangle]
unsafe extern "C" fn read_obj(name: *const c_char) -> c_ulong {
    static unsigned long read_obj(const char *name)
    {
    pub len: usize,
    pub "r"): *mut *mut FILE f = fopen(name,,
    if (!f) {
    pub 0: buffer[0] =,
    if (errno == EACCES)
    pub strerror(errno)): fatal("%s, Try using superuser\n",,
    } else {
    if (!fgets(buffer, sizeof(buffer), f))
    pub 0: buffer[0] =,
    pub strlen(buffer): len =,
    if (len > 0 && buffer[len - 1] == '\n')
    pub 0: buffer[len - 1] =,
    }
    pub strlen(buffer): return,
    }
//
// Get the contents of an attribute
//
#[no_mangle]
unsafe extern "C" fn get_obj(name: *const c_char) -> c_ulong {
    static unsigned long get_obj(const char *name)
    {
    if (!read_obj(name))
    pub 0: return,
    pub atol(buffer): return,
    }
#[no_mangle]
unsafe extern "C" fn get_obj_and_str(name: *const c_char, x: *mut c_char) -> c_ulong {
    static unsigned long get_obj_and_str(const char *name, char **x)
    {
    pub 0: unsigned long result =,
    pub p: *mut c_char,
// x = NULL;
    if (!read_obj(name))
    pub 0: return,
    pub 10): result = strtoul(buffer, &p,,
    while (*p == ' ')
    if (*p)
// x = strdup(p);
    pub result: return,
    }
#[no_mangle]
unsafe extern "C" fn set_obj(s: *mut slabinfo, name: *const c_char, n: c_int) {
    static void set_obj(struct slabinfo *s, const char *name, int n)
    {
    pub x: [c_char; 100],
    pub f: *mut FILE,
    pub name): snprintf(x, 100, "%s/%s", s->name,,
    pub "w"): f = fopen(x,,
    if (!f)
    pub x): fatal("Cannot write to %s\n",,
    pub n): fprintf(f, "%d\n",,
    }
#[no_mangle]
unsafe extern "C" fn read_slab_obj(s: *mut slabinfo, name: *const c_char) -> c_ulong {
    static unsigned long read_slab_obj(struct slabinfo *s, const char *name)
    {
    pub x: [c_char; 100],
    pub f: *mut FILE,
    pub l: usize,
    pub name): snprintf(x, 100, "%s/%s", s->name,,
    pub "r"): f = fopen(x,,
    if (!f) {
    pub 0: buffer[0] =,
    pub 0: l =,
    } else {
    pub f): l = fread(buffer, 1, sizeof(buffer),,
    pub 0: buffer[l] =,
    }
    pub l: return,
    }
#[no_mangle]
unsafe extern "C" fn read_debug_slab_obj(s: *mut slabinfo, name: *const c_char) -> c_ulong {
    static unsigned long read_debug_slab_obj(struct slabinfo *s, const char *name)
    {
    pub x: [c_char; 128],
    pub f: *mut FILE,
    pub l: usize,
    pub name): snprintf(x, 128, "/sys/kernel/debug/slab/%s/%s", s->name,,
    pub "r"): f = fopen(x,,
    if (!f) {
    pub 0: buffer[0] =,
    pub 0: l =,
    } else {
    pub f): l = fread(buffer, 1, sizeof(buffer),,
    pub 0: buffer[l] =,
    }
    pub l: return,
    }
//
// Put a size string together
//
#[no_mangle]
unsafe extern "C" fn store_size(buffer: *mut c_char, value: c_ulong) -> c_int {
    static int store_size(char *buffer, unsigned long value)
    {
    pub 1: unsigned long divisor =,
    pub 0: char trailer =,
    pub n: c_int,
    if (!show_bytes) {
    if (value > 1000000000UL) {
    pub 100000000UL: divisor =,
    pub 'G': trailer =,
    } else if (value > 1000000UL) {
    pub 100000UL: divisor =,
    pub 'M': trailer =,
    } else if (value > 1000UL) {
    pub 100: divisor =,
    pub 'K': trailer =,
    }
    }
    pub divisor: value /=,
    pub "%ld",value): n = sprintf(buffer,,
    if (trailer) {
    pub trailer: buffer[n] =,
    pub 0: buffer[n] =,
    }
    if (divisor != 1) {
    pub 4): memmove(buffer + n - 2, buffer + n - 3,,
    pub '.': buffer[n-2] =,
    }
    pub n: return,
    }
#[no_mangle]
unsafe extern "C" fn decode_numa_list(numa: *mut c_int, t: *mut c_char) {
    static void decode_numa_list(int *numa, char *t)
    {
    pub node: c_int,
    pub nr: c_int,
    pub sizeof(int)): *mut *mut memset(numa, 0, MAX_NODES,
    if (!t)
    while (*t == 'N') {
    pub 10): node = strtoul(t, &t,,
    if (*t == '=') {
    pub 10): nr = strtoul(t, &t,,
    pub nr: numa[node] =,
    if (node > highest_node)
    pub node: highest_node =,
    }
    while (*t == ' ')
    }
    }
#[no_mangle]
unsafe extern "C" fn slab_validate(s: *mut slabinfo) {
    static void slab_validate(struct slabinfo *s)
    {
    if (strcmp(s.name, "*") == 0)
    pub 1): set_obj(s, "validate",,
    }
#[no_mangle]
unsafe extern "C" fn slab_shrink(s: *mut slabinfo) {
    static void slab_shrink(struct slabinfo *s)
    {
    if (strcmp(s.name, "*") == 0)
    pub 1): set_obj(s, "shrink",,
    }
    pub 0: int line =,
#[no_mangle]
unsafe extern "C" fn first_line() {
    static void first_line(void)
    {
    if (show_activity)
    printf("Name                   Objects      Alloc       Free"
    pub UL\n"): " %%Fast Fallb O CmpX,
    else
    printf("Name                   Objects Objsize           %s "
    "Slabs/Part/Cpu  O/S O %%Fr %%Ef Flg\n",
    pub "Space"): sort_loss ? " Loss" :,
    }
//
// Find the shortest alias of a slab
//
    static struct aliasinfo *find_one_alias(struct slabinfo *find)
    {
    pub a: *mut aliasinfo,
    pub NULL: *mut *mut aliasinfo best =,
    pub {: for(a = aliasinfo;a < aliasinfo + aliases; a++),
    if (a.slab == find &&
    (!best || strlen(best.name) < strlen(a.name))) {
    pub a: best =,
    if (strncmp(a.name,"kmall", 5) == 0)
    pub best: return,
    }
    }
    pub best: return,
    }
#[no_mangle]
unsafe extern "C" fn slab_size(s: *mut slabinfo) -> c_ulong {
    static unsigned long slab_size(struct slabinfo *s)
    {
    pub s->order): *mut *mut return s->slabs  (page_size <<,
    }
#[no_mangle]
unsafe extern "C" fn slab_activity(s: *mut slabinfo) -> c_ulong {
    static unsigned long slab_activity(struct slabinfo *s)
    {
    return 	s.alloc_fastpath + s.free_fastpath +
    pub s->free_slowpath: s->alloc_slowpath +,
    }
#[no_mangle]
unsafe extern "C" fn slab_waste(s: *mut slabinfo) -> c_ulong {
    static unsigned long slab_waste(struct slabinfo *s)
    {
    pub s->object_size: *mut *mut return slab_size(s) - s->objects,
    }
#[no_mangle]
unsafe extern "C" fn slab_numa(s: *mut slabinfo, mode: c_int) {
    static void slab_numa(struct slabinfo *s, int mode)
    {
    pub node: c_int,
    if (strcmp(s.name, "*") == 0)
    if (!highest_node) {
    pub s->name): printf("\n%s: No NUMA information available.\n",,
    }
    if (skip_zero && !s.slabs)
    if (!line) {
    pub "Slab"): printf("\n%-21s:", mode ? "NUMA nodes" :,
    pub node++): for(node = 0; node <= highest_node;,
    pub node): printf(" %4d",,
    pub node++): for(node = 0; node <= highest_node;,
    }
    pub s->name): printf("%-21s ", mode ? "All slabs" :,
    pub {: for(node = 0; node <= highest_node; node++),
    pub b: [c_char; 20],
    pub s->numa[node]): store_size(b,,
    pub b): printf(" %4s",,
    }
    if (mode) {
    pub slabs"): printf("%-21s ", "Partial,
    pub {: for(node = 0; node <= highest_node; node++),
    pub b: [c_char; 20],
    pub s->numa_partial[node]): store_size(b,,
    pub b): printf(" %4s",,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn show_tracking(s: *mut slabinfo) {
    static void show_tracking(struct slabinfo *s)
    {
    pub s->name): printf("\n%s: Kernel object allocation\n",,
    if (read_debug_slab_obj(s, "alloc_traces"))
    pub buffer): printf("%s",,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: read_slab_obj(s, _arg: "alloc_calls")) -> else {
    else if (read_slab_obj(s, "alloc_calls"))
    pub buffer): printf("%s",,
    else
    pub Data\n"): printf("No,
    pub s->name): printf("\n%s: Kernel object freeing\n",,
    if (read_debug_slab_obj(s, "free_traces"))
    pub buffer): printf("%s",,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: read_slab_obj(s, _arg: "free_calls")) -> else {
    else if (read_slab_obj(s, "free_calls"))
    pub buffer): printf("%s",,
    else
    pub Data\n"): printf("No,
    }
#[no_mangle]
unsafe extern "C" fn ops(s: *mut slabinfo) {
    static void ops(struct slabinfo *s)
    {
    if (strcmp(s.name, "*") == 0)
    if (read_slab_obj(s, "ops")) {
    pub s->name): printf("\n%s: kmem_cache operations\n",,
    pub buffer): printf("%s",,
    } else
    pub s->name): printf("\n%s has no kmem_cache operations\n",,
    }
    static const char *onoff(int x)
    {
    if (x)
    pub ": return "On,
    pub "Off": return,
    }
#[no_mangle]
unsafe extern "C" fn slab_stats(s: *mut slabinfo) {
    static void slab_stats(struct slabinfo *s)
    {
    pub total_alloc: c_ulong,
    pub total_free: c_ulong,
    pub total: c_ulong,
    if (!s.alloc_slab)
    pub s->alloc_slowpath: total_alloc = s->alloc_fastpath +,
    pub s->free_slowpath: total_free = s->free_fastpath +,
    if (!total_alloc)
    pub %%Fr\n"): printf("Slab Perf Counter Alloc Free %%Al,
    printf("Fastpath             %8lu %8lu %3lu %3lu\n",
    s.alloc_fastpath, s.free_fastpath,
    s.alloc_fastpath * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_fastpath  100 / total_free :,
    printf("Slowpath             %8lu %8lu %3lu %3lu\n",
    total_alloc - s.alloc_fastpath, s.free_slowpath,
    (total_alloc - s.alloc_fastpath) * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_slowpath  100 / total_free :,
    printf("Page Alloc           %8lu %8lu %3lu %3lu\n",
    s.alloc_slab, s.free_slab,
    s.alloc_slab * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_slab  100 / total_free :,
    printf("Add partial          %8lu %8lu %3lu %3lu\n",
    s.deactivate_to_head + s.deactivate_to_tail,
    s.free_add_partial,
    (s.deactivate_to_head + s.deactivate_to_tail) * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_add_partial  100 / total_free :,
    printf("Remove partial       %8lu %8lu %3lu %3lu\n",
    s.alloc_from_partial, s.free_remove_partial,
    s.alloc_from_partial * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_remove_partial  100 / total_free :,
    printf("Cpu partial list     %8lu %8lu %3lu %3lu\n",
    s.cpu_partial_alloc, s.cpu_partial_free,
    s.cpu_partial_alloc * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->cpu_partial_free  100 / total_free :,
    printf("RemoteObj/SlabFrozen %8lu %8lu %3lu %3lu\n",
    s.deactivate_remote_frees, s.free_frozen,
    s.deactivate_remote_frees * 100 / total_alloc,
    pub 0): *mut *mut total_free ? s->free_frozen  100 / total_free :,
    pub total_free): printf("Total %8lu %8lu\n\n", total_alloc,,
    if (s.cpuslab_flush)
    pub s->cpuslab_flush): printf("Flushes %8lu\n",,
    total = s.deactivate_full + s.deactivate_empty +
    pub s->deactivate_bypass: s->deactivate_to_head + s->deactivate_to_tail +,
    if (total) {
    pub %%\n"): printf("\nSlab Deactivation Occurrences,
    printf("Slab full                     %7lu  %3lu%%\n",
    pub total): *mut *mut s->deactivate_full, (s->deactivate_full  100) /,
    printf("Slab empty                    %7lu  %3lu%%\n",
    pub total): *mut *mut s->deactivate_empty, (s->deactivate_empty  100) /,
    printf("Moved to head of partial list %7lu  %3lu%%\n",
    pub total): *mut *mut s->deactivate_to_head, (s->deactivate_to_head  100) /,
    printf("Moved to tail of partial list %7lu  %3lu%%\n",
    pub total): *mut *mut s->deactivate_to_tail, (s->deactivate_to_tail  100) /,
    printf("Deactivation bypass           %7lu  %3lu%%\n",
    pub total): *mut *mut s->deactivate_bypass, (s->deactivate_bypass  100) /,
    printf("Refilled from foreign frees   %7lu  %3lu%%\n",
    pub total): *mut *mut s->alloc_refill, (s->alloc_refill  100) /,
    printf("Node mismatch                 %7lu  %3lu%%\n",
    pub total): *mut *mut s->alloc_node_mismatch, (s->alloc_node_mismatch  100) /,
    }
    if (s.cmpxchg_double_fail || s.cmpxchg_double_cpu_fail) {
    pub Looping\n------------------------\n"): printf("\nCmpxchg_double,
    printf("Locked Cmpxchg Double redos   %lu\nUnlocked Cmpxchg Double redos %lu\n",
    pub s->cmpxchg_double_cpu_fail): s->cmpxchg_double_fail,,
    }
    }
#[no_mangle]
unsafe extern "C" fn report(s: *mut slabinfo) {
    static void report(struct slabinfo *s)
    {
    if (strcmp(s.name, "*") == 0)
    printf("\nSlabcache: %-15s  Aliases: %2d Order : %2d Objects: %lu\n",
    pub s->objects): s->name, s->aliases, s->order,,
    if (s.hwcache_align)
    pub aligned\n"): *mut *mut *mut printf(" Hardware cacheline,
    if (s.cache_dma)
    pub zone\n"): *mut *mut *mut printf(" Memory is allocated in a special DMA,
    if (s.destroy_by_rcu)
    pub RCU\n"): *mut *mut *mut printf(" Slabs are destroyed via,
    if (s.reclaim_account)
    pub active\n"): *mut *mut *mut printf(" Reclaim accounting,
    pub Memory\n"): printf("\nSizes (bytes) Slabs Debug,
    printf("Object : %7d  Total  : %7ld   Sanity Checks : %s  Total: %7ld\n",
    s.object_size, s.slabs, onoff(s.sanity_checks),
    pub s->order)): *mut *mut s->slabs  (page_size <<,
    printf("SlabObj: %7d  Full   : %7ld   Redzoning     : %s  Used : %7ld\n",
    s.slab_size, s.slabs - s.partial - s.cpu_slabs,
    pub s->object_size): *mut *mut onoff(s->red_zone), s->objects,
    printf("SlabSiz: %7d  Partial: %7ld   Poisoning     : %s  Loss : %7ld\n",
    page_size << s.order, s.partial, onoff(s.poison),
    pub s->object_size): *mut *mut *mut s->slabs  (page_size << s->order) - s->objects,
    printf("Loss   : %7d  CpuSlab: %7d   Tracking      : %s  Lalig: %7ld\n",
    s.slab_size - s.object_size, s.cpu_slabs, onoff(s.store_user),
    pub s->objects): *mut *mut (s->slab_size - s->object_size),
    printf("Align  : %7d  Objects: %7d   Tracing       : %s  Lpadd: %7ld\n",
    s.align, s.objs_per_slab, onoff(s.trace),
    ((page_size << s.order) - s.objs_per_slab * s.slab_size) *
    pub 1): slab_numa(s,,
    }
#[no_mangle]
unsafe extern "C" fn slabcache(s: *mut slabinfo) {
    static void slabcache(struct slabinfo *s)
    {
    pub size_str: [c_char; 20],
    pub dist_str: [c_char; 40],
    pub flags: [c_char; 20],
    pub flags: *mut *mut char p =,
    if (strcmp(s.name, "*") == 0)
    if (unreclaim_only && s.reclaim_account)
    if (actual_slabs == 1) {
    }
    if (skip_zero && !show_empty && !s.slabs)
    if (show_empty && s.slabs)
    if (sort_loss == 0)
    pub slab_size(s)): store_size(size_str,,
    else
    pub slab_waste(s)): store_size(size_str,,
    snprintf(dist_str, 40, "%lu/%lu/%d", s.slabs - s.cpu_slabs,
    pub s->cpu_slabs): s->partial,,
    if (!line++)
    if (s.aliases)
// p++ = '*';
    if (s.cache_dma)
// p++ = 'd';
    if (s.hwcache_align)
// p++ = 'A';
    if (s.poison)
// p++ = 'P';
    if (s.reclaim_account)
// p++ = 'a';
    if (s.red_zone)
// p++ = 'Z';
    if (s.sanity_checks)
// p++ = 'F';
    if (s.store_user)
// p++ = 'U';
    if (s.trace)
// p++ = 'T';
// p = 0;
    if (show_activity) {
    pub total_alloc: c_ulong,
    pub total_free: c_ulong,
    pub s->alloc_slowpath: total_alloc = s->alloc_fastpath +,
    pub s->free_slowpath: total_free = s->free_fastpath +,
    printf("%-21s %8ld %10ld %10ld %3ld %3ld %5ld %1d %4ld %4ld\n",
    s.name, s.objects,
    total_alloc, total_free,
    total_alloc ? (s.alloc_fastpath * 100 / total_alloc) : 0,
    total_free ? (s.free_fastpath * 100 / total_free) : 0,
    s.order_fallback, s.order, s.cmpxchg_double_fail,
    } else {
    printf("%-21s %8ld %7d %15s %14s %4d %1d %3ld %3ld %s\n",
    s.name, s.objects, s.object_size, size_str, dist_str,
    s.objs_per_slab, s.order,
    s.slabs ? (s.partial * 100) / s.slabs : 100,
    s.slabs ? (s.objects * s.object_size * 100) /
    (s.slabs * (page_size << s.order)) : 100,
    }
    }
//
// Analyze debug options. Return false if something is amiss.
//
#[no_mangle]
unsafe extern "C" fn debug_opt_scan(opt: *mut c_char) -> c_int {
    static int debug_opt_scan(char *opt)
    {
    if (!opt || !opt[0] || strcmp(opt, "-") == 0)
    pub 1: return,
    if (strcasecmp(opt, "a") == 0) {
    pub 1: sanity =,
    pub 1: poison =,
    pub 1: redzone =,
    pub 1: tracking =,
    pub 1: return,
    }
    pub opt++): *mut *mut for ( ; opt;,
    switch (*opt) {
    case 'F' : case 'f':
    if (sanity)
    pub 0: return,
    pub 1: sanity =,
    case 'P' : case 'p':
    if (poison)
    pub 0: return,
    pub 1: poison =,
    case 'Z' : case 'z':
    if (redzone)
    pub 0: return,
    pub 1: redzone =,
    case 'U' : case 'u':
    if (tracking)
    pub 0: return,
    pub 1: tracking =,
    case 'T' : case 't':
    if (tracing)
    pub 0: return,
    pub 1: tracing =,
    default:
    pub 0: return,
    }
    pub 1: return,
    }
#[no_mangle]
unsafe extern "C" fn slab_empty(s: *mut slabinfo) -> c_int {
    static int slab_empty(struct slabinfo *s)
    {
    if (s.objects > 0)
    pub 0: return,
//
// We may still have slabs even if there are no objects. Shrinking will
// remove them.
//
    if (s.slabs != 0)
    pub 1): set_obj(s, "shrink",,
    pub 1: return,
    }
#[no_mangle]
unsafe extern "C" fn slab_debug(s: *mut slabinfo) {
    static void slab_debug(struct slabinfo *s)
    {
    if (strcmp(s.name, "*") == 0)
    if (sanity && !s.sanity_checks) {
    pub 1): set_obj(s, "sanity_checks",,
    }
    if (!sanity && s.sanity_checks) {
    if (slab_empty(s))
    pub 0): set_obj(s, "sanity_checks",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot disable sanity checks\n",,
    }
    if (redzone && !s.red_zone) {
    if (slab_empty(s))
    pub 1): set_obj(s, "red_zone",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot enable redzoning\n",,
    }
    if (!redzone && s.red_zone) {
    if (slab_empty(s))
    pub 0): set_obj(s, "red_zone",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot disable redzoning\n",,
    }
    if (poison && !s.poison) {
    if (slab_empty(s))
    pub 1): set_obj(s, "poison",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot enable poisoning\n",,
    }
    if (!poison && s.poison) {
    if (slab_empty(s))
    pub 0): set_obj(s, "poison",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot disable poisoning\n",,
    }
    if (tracking && !s.store_user) {
    if (slab_empty(s))
    pub 1): set_obj(s, "store_user",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot enable tracking\n",,
    }
    if (!tracking && s.store_user) {
    if (slab_empty(s))
    pub 0): set_obj(s, "store_user",,
    else
    pub s->name): fprintf(stderr, "%s not empty cannot disable tracking\n",,
    }
    if (tracing && !s.trace) {
    if (slabs == 1)
    pub 1): set_obj(s, "trace",,
    else
    pub s->name): fprintf(stderr, "%s can only enable trace for one slab at a time\n",,
    }
    if (!tracing && s.trace)
    pub 0): set_obj(s, "trace",,
    }
#[no_mangle]
unsafe extern "C" fn totals() {
    static void totals(void)
    {
    pub s: *mut slabinfo,
    pub 0: int used_slabs =,
    pub b4: [char b1[20], b2[20], b3[20],; 20],
    pub 63: unsigned long long max = 1ULL <<,
// Object size
    pub avg_objsize: unsigned long long min_objsize = max, max_objsize = 0,,
// Number of partial slabs in a slabcache
    unsigned long long min_partial = max, max_partial = 0,
    pub 0: avg_partial, total_partial =,
// Number of slabs in a slab cache
    unsigned long long min_slabs = max, max_slabs = 0,
    pub 0: avg_slabs, total_slabs =,
// Size of the whole slab
    unsigned long long min_size = max, max_size = 0,
    pub 0: avg_size, total_size =,
// Bytes used for object storage in a slab
    unsigned long long min_used = max, max_used = 0,
    pub 0: avg_used, total_used =,
// Waste: Bytes used for alignment and padding
    unsigned long long min_waste = max, max_waste = 0,
    pub 0: avg_waste, total_waste =,
// Number of objects in a slab
    unsigned long long min_objects = max, max_objects = 0,
    pub 0: avg_objects, total_objects =,
// Waste per object
    unsigned long long min_objwaste = max,
    max_objwaste = 0, avg_objwaste,
    pub 0: total_objwaste =,
// Memory per object
    unsigned long long min_memobj = max,
    max_memobj = 0, avg_memobj,
    pub 0: total_objsize =,
// Percentage of partial slabs per slab
    unsigned long min_ppart = 100, max_ppart = 0,
    pub 0: avg_ppart, total_ppart =,
// Number of objects in partial slabs
    unsigned long min_partobj = max, max_partobj = 0,
    pub 0: avg_partobj, total_partobj =,
// Percentage of partial objects of all objects in a slab
    unsigned long min_ppartobj = 100, max_ppartobj = 0,
    pub 0: avg_ppartobj, total_ppartobj =,
    pub {: for (s = slabinfo; s < slabinfo + slabs; s++),
    pub size: c_ulonglong,
    pub used: c_ulong,
    pub wasted: c_ulonglong,
    pub objwaste: c_ulonglong,
    pub percentage_partial_slabs: c_ulong,
    pub percentage_partial_objs: c_ulong,
    if (!s.slabs || !s.objects)
    pub slab_size(s): size =,
    pub s->object_size: *mut *mut used = s->objects,
    pub used: wasted = size -,
    pub s->object_size: objwaste = s->slab_size -,
    pub s->slabs: *mut *mut percentage_partial_slabs = s->partial  100 /,
    if (percentage_partial_slabs > 100)
    pub 100: percentage_partial_slabs =,
    percentage_partial_objs = s.objects_partial * 100
    pub s->objects: /,
    if (percentage_partial_objs > 100)
    pub 100: percentage_partial_objs =,
    if (s.object_size < min_objsize)
    pub s->object_size: min_objsize =,
    if (s.partial < min_partial)
    pub s->partial: min_partial =,
    if (s.slabs < min_slabs)
    pub s->slabs: min_slabs =,
    if (size < min_size)
    pub size: min_size =,
    if (wasted < min_waste)
    pub wasted: min_waste =,
    if (objwaste < min_objwaste)
    pub objwaste: min_objwaste =,
    if (s.objects < min_objects)
    pub s->objects: min_objects =,
    if (used < min_used)
    pub used: min_used =,
    if (s.objects_partial < min_partobj)
    pub s->objects_partial: min_partobj =,
    if (percentage_partial_slabs < min_ppart)
    pub percentage_partial_slabs: min_ppart =,
    if (percentage_partial_objs < min_ppartobj)
    pub percentage_partial_objs: min_ppartobj =,
    if (s.slab_size < min_memobj)
    pub s->slab_size: min_memobj =,
    if (s.object_size > max_objsize)
    pub s->object_size: max_objsize =,
    if (s.partial > max_partial)
    pub s->partial: max_partial =,
    if (s.slabs > max_slabs)
    pub s->slabs: max_slabs =,
    if (size > max_size)
    pub size: max_size =,
    if (wasted > max_waste)
    pub wasted: max_waste =,
    if (objwaste > max_objwaste)
    pub objwaste: max_objwaste =,
    if (s.objects > max_objects)
    pub s->objects: max_objects =,
    if (used > max_used)
    pub used: max_used =,
    if (s.objects_partial > max_partobj)
    pub s->objects_partial: max_partobj =,
    if (percentage_partial_slabs > max_ppart)
    pub percentage_partial_slabs: max_ppart =,
    if (percentage_partial_objs > max_ppartobj)
    pub percentage_partial_objs: max_ppartobj =,
    if (s.slab_size > max_memobj)
    pub s->slab_size: max_memobj =,
    pub s->partial: total_partial +=,
    pub s->slabs: total_slabs +=,
    pub size: total_size +=,
    pub wasted: total_waste +=,
    pub s->objects: total_objects +=,
    pub used: total_used +=,
    pub s->objects_partial: total_partobj +=,
    pub percentage_partial_slabs: total_ppart +=,
    pub percentage_partial_objs: total_ppartobj +=,
    pub objwaste: *mut *mut total_objwaste += s->objects,
    pub s->slab_size: *mut *mut total_objsize += s->objects,
    }
    if (!total_objects) {
    pub objects\n"): printf("No,
    }
    if (!used_slabs) {
    pub slabs\n"): printf("No,
    }
// Per slab averages
    pub used_slabs: avg_partial = total_partial /,
    pub used_slabs: avg_slabs = total_slabs /,
    pub used_slabs: avg_size = total_size /,
    pub used_slabs: avg_waste = total_waste /,
    pub used_slabs: avg_objects = total_objects /,
    pub used_slabs: avg_used = total_used /,
    pub used_slabs: avg_partobj = total_partobj /,
    pub used_slabs: avg_ppart = total_ppart /,
    pub used_slabs: avg_ppartobj = total_ppartobj /,
// Per object object sizes
    pub total_objects: avg_objsize = total_used /,
    pub total_objects: avg_objwaste = total_objwaste /,
    pub total_objects: *mut *mut avg_partobj = total_partobj  100 /,
    pub total_objects: avg_memobj = total_objsize /,
    pub Totals\n"): printf("Slabcache,
    printf("Slabcaches : %15d   Aliases  : %11d.%-3d  Active:    %3d\n",
    pub used_slabs): slabs, aliases, alias_targets,,
    pub total_waste): store_size(b1, total_size);store_size(b2,,
    pub total_used): *mut *mut store_size(b3, total_waste  100 /,
    pub b3): printf("Memory used: %15s # Loss : %15s MRatio:%6s%%\n", b1, b2,,
    pub total_partobj): store_size(b1, total_objects);store_size(b2,,
    pub total_objects): *mut *mut store_size(b3, total_partobj  100 /,
    pub b3): printf("# Objects : %15s # PartObj: %15s ORatio:%6s%%\n", b1, b2,,
    printf("Per Cache         Average              "
    pub Total\n"): "Min Max,
    printf("---------------------------------------"
    pub min_objects): store_size(b1, avg_objects);store_size(b2,,
    pub total_objects): store_size(b3, max_objects);store_size(b4,,
    printf("#Objects  %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_slabs): store_size(b1, avg_slabs);store_size(b2,,
    pub total_slabs): store_size(b3, max_slabs);store_size(b4,,
    printf("#Slabs    %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_partial): store_size(b1, avg_partial);store_size(b2,,
    pub total_partial): store_size(b3, max_partial);store_size(b4,,
    printf("#PartSlab %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_ppart): store_size(b1, avg_ppart);store_size(b2,,
    pub max_ppart): store_size(b3,,
    pub total_slabs): *mut *mut store_size(b4, total_partial  100 /,
    printf("%%PartSlab%15s%% %15s%% %15s%% %15s%%\n",
    pub b4): b1, b2, b3,,
    pub min_partobj): store_size(b1, avg_partobj);store_size(b2,,
    pub max_partobj): store_size(b3,,
    pub total_partobj): store_size(b4,,
    printf("PartObjs  %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_ppartobj): store_size(b1, avg_ppartobj);store_size(b2,,
    pub max_ppartobj): store_size(b3,,
    pub total_objects): *mut *mut store_size(b4, total_partobj  100 /,
    printf("%% PartObj%15s%% %15s%% %15s%% %15s%%\n",
    pub b4): b1, b2, b3,,
    pub min_size): store_size(b1, avg_size);store_size(b2,,
    pub total_size): store_size(b3, max_size);store_size(b4,,
    printf("Memory    %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_used): store_size(b1, avg_used);store_size(b2,,
    pub total_used): store_size(b3, max_used);store_size(b4,,
    printf("Used      %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    pub min_waste): store_size(b1, avg_waste);store_size(b2,,
    pub total_waste): store_size(b3, max_waste);store_size(b4,,
    printf("Loss      %15s  %15s  %15s  %15s\n",
    pub b4): b1, b2, b3,,
    printf("Per Object        Average              "
    pub Max\n"): "Min,
    printf("---------------------------------------"
    pub min_memobj): store_size(b1, avg_memobj);store_size(b2,,
    pub max_memobj): store_size(b3,,
    printf("Memory    %15s  %15s  %15s\n",
    pub b3): b1, b2,,
    pub min_objsize): store_size(b1, avg_objsize);store_size(b2,,
    pub max_objsize): store_size(b3,,
    printf("User      %15s  %15s  %15s\n",
    pub b3): b1, b2,,
    pub min_objwaste): store_size(b1, avg_objwaste);store_size(b2,,
    pub max_objwaste): store_size(b3,,
    printf("Loss      %15s  %15s  %15s\n",
    pub b3): b1, b2,,
    }
#[no_mangle]
unsafe extern "C" fn sort_slabs() {
    static void sort_slabs(void)
    {
    pub s1,*s2: *mut slabinfo,
    pub {: for (s1 = slabinfo; s1 < slabinfo + slabs; s1++),
    pub {: for (s2 = s1 + 1; s2 < slabinfo + slabs; s2++),
    pub result: c_int,
    if (sort_size) {
    if (slab_size(s1) == slab_size(s2))
    pub s2->name): result = strcasecmp(s1->name,,
    else
    pub slab_size(s2): result = slab_size(s1) <,
    } else if (sort_active) {
    if (slab_activity(s1) == slab_activity(s2))
    pub s2->name): result = strcasecmp(s1->name,,
    else
    pub slab_activity(s2): result = slab_activity(s1) <,
    } else if (sort_loss) {
    if (slab_waste(s1) == slab_waste(s2))
    pub s2->name): result = strcasecmp(s1->name,,
    else
    pub slab_waste(s2): result = slab_waste(s1) <,
    } else if (sort_partial) {
    if (s1.partial == s2.partial)
    pub s2->name): result = strcasecmp(s1->name,,
    else
    pub s2->partial: result = s1->partial <,
    } else
    pub s2->name): result = strcasecmp(s1->name,,
    if (show_inverted)
    pub -result: result =,
    if (result > 0) {
    pub t: slabinfo,
    pub slabinfo)): memcpy(&t, s1, sizeof(struct,
    pub slabinfo)): memcpy(s1, s2, sizeof(struct,
    pub slabinfo)): memcpy(s2, &t, sizeof(struct,
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sort_aliases() {
    static void sort_aliases(void)
    {
    pub a1,*a2: *mut aliasinfo,
    pub {: for (a1 = aliasinfo; a1 < aliasinfo + aliases; a1++),
    pub {: for (a2 = a1 + 1; a2 < aliasinfo + aliases; a2++),
    pub n2: *mut *mut char n1,,
    pub a1->name: n1 =,
    pub a2->name: n2 =,
    if (show_alias && !show_inverted) {
    pub a1->ref: n1 =,
    pub a2->ref: n2 =,
    }
    if (strcasecmp(n1, n2) > 0) {
    pub t: aliasinfo,
    pub aliasinfo)): memcpy(&t, a1, sizeof(struct,
    pub aliasinfo)): memcpy(a1, a2, sizeof(struct,
    pub aliasinfo)): memcpy(a2, &t, sizeof(struct,
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn link_slabs() {
    static void link_slabs(void)
    {
    pub a: *mut aliasinfo,
    pub s: *mut slabinfo,
    pub {: for (a = aliasinfo; a < aliasinfo + aliases; a++),
    pub s++): for (s = slabinfo; s < slabinfo + slabs;,
    if (strcmp(a.ref, s.name) == 0) {
    pub s: a->slab =,
    }
    if (s == slabinfo + slabs)
    pub a->ref): fatal("Unresolved alias %s\n",,
    }
    }
#[no_mangle]
unsafe extern "C" fn alias() {
    static void alias(void)
    {
    pub a: *mut aliasinfo,
    pub NULL: *mut *mut char active =,
    pub {: for(a = aliasinfo; a < aliasinfo + aliases; a++),
    if (!show_single_ref && a.slab.refs == 1)
    if (!show_inverted) {
    if (active) {
    if (strcmp(a.slab.name, active) == 0) {
    pub a->name): printf(" %s",,
    }
    }
    pub a->name): printf("\n%-12s <- %s", a->slab->name,,
    pub a->slab->name: active =,
    }
    else
    pub a->slab->name): printf("%-15s -> %s\n", a->name,,
    }
    if (active)
    }
#[no_mangle]
unsafe extern "C" fn rename_slabs() {
    static void rename_slabs(void)
    {
    pub s: *mut slabinfo,
    pub a: *mut aliasinfo,
    pub {: for (s = slabinfo; s < slabinfo + slabs; s++),
    if (*s.name != ':')
    if (s.refs > 1 && !show_first_alias)
    pub find_one_alias(s): a =,
    if (a)
    pub a->name: s->name =,
    else {
    pub "*": *mut s->name =,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn slab_mismatch(slab: *mut c_char) -> c_int {
    static int slab_mismatch(char *slab)
    {
    pub 0): return regexec(&pattern, slab, 0, NULL,,
    }
#[no_mangle]
unsafe extern "C" fn read_slab_dir() {
    static void read_slab_dir(void)
    {
    pub dir: *mut DIR,
    pub de: *mut dirent,
    pub slabinfo: *mut *mut slabinfo slab =,
    pub aliasinfo: *mut *mut aliasinfo alias =,
    pub p: *mut c_char,
    pub t: *mut c_char,
    pub count: c_int,
    if (chdir("/sys/kernel/slab") && chdir("/sys/slab"))
    pub active\n"): fatal("SYSFS support for SLUB not,
    pub opendir("."): dir =,
    while ((de = readdir(dir))) {
    if (de.d_name[0] == '.' ||
    (de.d_name[0] != ':' && slab_mismatch(de.d_name)))
    switch (de.d_type) {
    case DT_LNK:
    if (alias - aliasinfo == MAX_ALIASES)
    pub aliases\n"): fatal("Too many,
    pub strdup(de->d_name): alias->name =,
    pub sizeof(buffer)-1): count = readlink(de->d_name, buffer,,
    if (count < 0)
    pub de->d_name): fatal("Cannot read symlink %s\n",,
    pub 0: buffer[count] =,
    pub count: p = buffer +,
    while (p > buffer && p[-1] != '/')
    pub strdup(p): alias->ref =,
    case DT_DIR:
    if (slab - slabinfo == MAX_SLABS)
    pub slabs\n"): fatal("Too many,
    if (chdir(de.d_name))
    pub slab->name): fatal("Unable to access slab %s\n",,
    pub strdup(de->d_name): slab->name =,
    pub 0: slab->alias =,
    pub 0: slab->refs =,
    pub get_obj("aliases"): slab->aliases =,
    pub get_obj("align"): slab->align =,
    pub get_obj("cache_dma"): slab->cache_dma =,
    pub get_obj("cpu_slabs"): slab->cpu_slabs =,
    pub get_obj("destroy_by_rcu"): slab->destroy_by_rcu =,
    pub get_obj("hwcache_align"): slab->hwcache_align =,
    pub get_obj("object_size"): slab->object_size =,
    pub get_obj("objects"): slab->objects =,
    pub get_obj("objects_partial"): slab->objects_partial =,
    pub get_obj("total_objects"): slab->total_objects =,
    pub get_obj("objs_per_slab"): slab->objs_per_slab =,
    pub get_obj("order"): slab->order =,
    pub &t): slab->partial = get_obj_and_str("partial",,
    pub t): decode_numa_list(slab->numa_partial,,
    pub get_obj("poison"): slab->poison =,
    pub get_obj("reclaim_account"): slab->reclaim_account =,
    pub get_obj("red_zone"): slab->red_zone =,
    pub get_obj("sanity_checks"): slab->sanity_checks =,
    pub get_obj("slab_size"): slab->slab_size =,
    pub &t): slab->slabs = get_obj_and_str("slabs",,
    pub t): decode_numa_list(slab->numa,,
    pub get_obj("store_user"): slab->store_user =,
    pub get_obj("trace"): slab->trace =,
    pub get_obj("alloc_fastpath"): slab->alloc_fastpath =,
    pub get_obj("alloc_slowpath"): slab->alloc_slowpath =,
    pub get_obj("free_fastpath"): slab->free_fastpath =,
    pub get_obj("free_slowpath"): slab->free_slowpath =,
    pub get_obj("free_frozen"): slab->free_frozen=,
    pub get_obj("free_add_partial"): slab->free_add_partial =,
    pub get_obj("free_remove_partial"): slab->free_remove_partial =,
    pub get_obj("alloc_from_partial"): slab->alloc_from_partial =,
    pub get_obj("alloc_slab"): slab->alloc_slab =,
    pub get_obj("alloc_refill"): slab->alloc_refill =,
    pub get_obj("free_slab"): slab->free_slab =,
    pub get_obj("cpuslab_flush"): slab->cpuslab_flush =,
    pub get_obj("deactivate_full"): slab->deactivate_full =,
    pub get_obj("deactivate_empty"): slab->deactivate_empty =,
    pub get_obj("deactivate_to_head"): slab->deactivate_to_head =,
    pub get_obj("deactivate_to_tail"): slab->deactivate_to_tail =,
    pub get_obj("deactivate_remote_frees"): slab->deactivate_remote_frees =,
    pub get_obj("order_fallback"): slab->order_fallback =,
    pub get_obj("cmpxchg_double_cpu_fail"): slab->cmpxchg_double_cpu_fail =,
    pub get_obj("cmpxchg_double_fail"): slab->cmpxchg_double_fail =,
    pub get_obj("cpu_partial_alloc"): slab->cpu_partial_alloc =,
    pub get_obj("cpu_partial_free"): slab->cpu_partial_free =,
    pub get_obj("alloc_node_mismatch"): slab->alloc_node_mismatch =,
    pub get_obj("deactivate_bypass"): slab->deactivate_bypass =,
    if (chdir(".."))
    fatal("Unable to chdir from slab ../%s\n",
    if (slab.name[0] == ':')
    default :
    pub de->d_type): fatal("Unknown file type %lx\n",,
    }
    }
    pub slabinfo: slabs = slab -,
    pub slabs: actual_slabs =,
    pub aliasinfo: aliases = alias -,
    }
#[no_mangle]
unsafe extern "C" fn output_slabs() {
    static void output_slabs(void)
    {
    pub slab: *mut slabinfo,
    pub output_lines: int lines =,
    pub &&: for (slab = slabinfo; (slab < slabinfo + slabs),
    pub {: lines != 0; slab++),
    if (slab.alias)
    if (lines != -1)
    if (show_numa)
    pub 0): slab_numa(slab,,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: show_track) -> else {
    else if (show_track)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: validate) -> else {
    else if (validate)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: shrink) -> else {
    else if (shrink)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: set_debug) -> else {
    else if (set_debug)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: show_ops) -> else {
    else if (show_ops)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: show_slab) -> else {
    else if (show_slab)
#[no_mangle]
pub unsafe extern "C" fn if(_arg: show_report) -> else {
    else if (show_report)
    }
    }
    static void _xtotals(char *heading, char *underline,
    int loss, int size, int partial)
    {
    pub underline): printf("%s%s", heading,,
    pub 0: line =,
    pub loss: sort_loss =,
    pub size: sort_size =,
    pub partial: sort_partial =,
    }
#[no_mangle]
unsafe extern "C" fn xtotals() {
    static void xtotals(void)
    {
    pub underline: *mut *mut char heading,,
    pub size\n": heading = "\nSlabs sorted by,
    pub "--------------------\n": underline =,
    pub 0): _xtotals(heading, underline, 0, 1,,
    pub loss\n": heading = "\nSlabs sorted by,
    pub "--------------------\n": underline =,
    pub 0): _xtotals(heading, underline, 1, 0,,
    pub slabs\n": heading = "\nSlabs sorted by number of partial,
    pub "---------------------------------------\n": underline =,
    pub 1): _xtotals(heading, underline, 0, 0,,
    }
    struct option opts[] = {
    { "aliases", no_argument, core::ptr::null_mut(), 'a' },
    { "activity", no_argument, core::ptr::null_mut(), 'A' },
    { "Bytes", no_argument, core::ptr::null_mut(), 'B'},
    { "debug", optional_argument, core::ptr::null_mut(), 'd' },
    { "display-activity", no_argument, core::ptr::null_mut(), 'D' },
    { "empty", no_argument, core::ptr::null_mut(), 'e' },
    { "first-alias", no_argument, core::ptr::null_mut(), 'f' },
    { "help", no_argument, core::ptr::null_mut(), 'h' },
    { "inverted", no_argument, core::ptr::null_mut(), 'i'},
    { "slabs", no_argument, core::ptr::null_mut(), 'l' },
    { "Loss", no_argument, core::ptr::null_mut(), 'L'},
    { "numa", no_argument, core::ptr::null_mut(), 'n' },
    { "lines", required_argument, core::ptr::null_mut(), 'N'},
    { "ops", no_argument, core::ptr::null_mut(), 'o' },
    { "partial", no_argument, core::ptr::null_mut(), 'P'},
    { "report", no_argument, core::ptr::null_mut(), 'r' },
    { "shrink", no_argument, core::ptr::null_mut(), 's' },
    { "Size", no_argument, core::ptr::null_mut(), 'S'},
    { "tracking", no_argument, core::ptr::null_mut(), 't'},
    { "Totals", no_argument, core::ptr::null_mut(), 'T'},
    { "Unreclaim", no_argument, core::ptr::null_mut(), 'U'},
    { "validate", no_argument, core::ptr::null_mut(), 'v' },
    { "Xtotals", no_argument, core::ptr::null_mut(), 'X'},
    { "zero", no_argument, core::ptr::null_mut(), 'z' },
    { "1ref", no_argument, core::ptr::null_mut(), '1'},
    { core::ptr::null_mut(), 0, core::ptr::null_mut(), 0 }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int c;
    int err;
    char *pattern_source;
    page_size = getpagesize();
    while ((c = getopt_long(argc, argv, "aABd::DefhilLnN:oPrsStTUvXz1",
    opts, core::ptr::null_mut())) != -1)
    switch (c) {
    case 'a':
    show_alias = 1;
    break;
    case 'A':
    sort_active = 1;
    break;
    case 'B':
    show_bytes = 1;
    break;
    case 'd':
    set_debug = 1;
    if (!debug_opt_scan(optarg))
    fatal("Invalid debug option '%s'\n", optarg);
    break;
    case 'D':
    show_activity = 1;
    break;
    case 'e':
    show_empty = 1;
    break;
    case 'f':
    show_first_alias = 1;
    break;
    case 'h':
    usage();
    return 0;
    case 'i':
    show_inverted = 1;
    break;
    case 'l':
    show_slab = 1;
    break;
    case 'L':
    sort_loss = 1;
    break;
    case 'n':
    show_numa = 1;
    break;
    case 'N':
    if (optarg) {
    output_lines = atoi(optarg);
    if (output_lines < 1)
    output_lines = 1;
    }
    break;
    case 'o':
    show_ops = 1;
    break;
    case 'r':
    show_report = 1;
    break;
    case 'P':
    sort_partial = 1;
    break;
    case 's':
    shrink = 1;
    break;
    case 'S':
    sort_size = 1;
    break;
    case 't':
    show_track = 1;
    break;
    case 'T':
    show_totals = 1;
    break;
    case 'U':
    unreclaim_only = 1;
    break;
    case 'v':
    validate = 1;
    break;
    case 'X':
    if (output_lines == -1)
    output_lines = 1;
    extended_totals = 1;
    show_bytes = 1;
    break;
    case 'z':
    skip_zero = 0;
    break;
    case '1':
    show_single_ref = 1;
    break;
    default:
    fatal("%s: Invalid option '%c'\n", argv[0], optopt);
    }
    if (!show_slab && !show_alias && !show_track && !show_report
    && !validate && !shrink && !set_debug && !show_ops)
    show_slab = 1;
    if (argc > optind)
    pattern_source = argv[optind];
    else
    pattern_source = ".*";
    err = regcomp(&pattern, pattern_source, REG_ICASE|REG_NOSUB);
    if (err)
    fatal("%s: Invalid pattern '%s' code %d\n",
    argv[0], pattern_source, err);
    read_slab_dir();
    if (show_alias) {
    alias();
    } else if (extended_totals) {
    xtotals();
    } else if (show_totals) {
    totals();
    } else {
    link_slabs();
    rename_slabs();
    sort_slabs();
    output_slabs();
    }
    return 0;
    }
