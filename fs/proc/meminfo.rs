//! Automatically rewritten from C to Rust
//! Source: fs/proc/meminfo.c
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

#[no_mangle]
pub unsafe extern "C" fn __attribute__(m: *mut (weak)) arch_report_meminfo(struct seq_file) {
    void __attribute__((weak)) arch_report_meminfo(struct seq_file *m)
    {
    }
#[no_mangle]
unsafe extern "C" fn show_val_kb(m: *mut seq_file, s: *const c_char, num: c_ulong) {
    static void show_val_kb(struct seq_file *m, const char *s, unsigned long num)
    {
    seq_put_decimal_ull_width(m, s, num << (PAGE_SHIFT - 10), 8);
    seq_write(m, " kB\n", 4);
    }
#[no_mangle]
unsafe extern "C" fn meminfo_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int meminfo_proc_show(struct seq_file *m, void *v)
    {
    struct sysinfo i;
    unsigned long committed;
    long cached;
    long available;
    unsigned long pages[NR_LRU_LISTS];
    unsigned long sreclaimable, sunreclaim;
    int lru;
    si_meminfo(&i);
    si_swapinfo(&i);
    committed = vm_memory_committed();
    cached = global_node_page_state(NR_FILE_PAGES) -
    total_swapcache_pages() - i.bufferram;
    if (cached < 0)
    cached = 0;
    for (lru = LRU_BASE; lru < NR_LRU_LISTS; lru++)
    pages[lru] = global_node_page_state(NR_LRU_BASE + lru);
    available = si_mem_available();
    sreclaimable = global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B);
    sunreclaim = global_node_page_state_pages(NR_SLAB_UNRECLAIMABLE_B);
    show_val_kb(m, "MemTotal:       ", i.totalram);
    show_val_kb(m, "MemFree:        ", i.freeram);
    show_val_kb(m, "MemAvailable:   ", available);
    show_val_kb(m, "Buffers:        ", i.bufferram);
    show_val_kb(m, "Cached:         ", cached);
    show_val_kb(m, "SwapCached:     ", total_swapcache_pages());
    show_val_kb(m, "Active:         ", pages[LRU_ACTIVE_ANON] +
    pages[LRU_ACTIVE_FILE]);
    show_val_kb(m, "Inactive:       ", pages[LRU_INACTIVE_ANON] +
    pages[LRU_INACTIVE_FILE]);
    show_val_kb(m, "Active(anon):   ", pages[LRU_ACTIVE_ANON]);
    show_val_kb(m, "Inactive(anon): ", pages[LRU_INACTIVE_ANON]);
    show_val_kb(m, "Active(file):   ", pages[LRU_ACTIVE_FILE]);
    show_val_kb(m, "Inactive(file): ", pages[LRU_INACTIVE_FILE]);
    show_val_kb(m, "Unevictable:    ", pages[LRU_UNEVICTABLE]);
    show_val_kb(m, "Mlocked:        ", global_zone_page_state(NR_MLOCK));

    show_val_kb(m, "HighTotal:      ", i.totalhigh);
    show_val_kb(m, "HighFree:       ", i.freehigh);
    show_val_kb(m, "LowTotal:       ", i.totalram - i.totalhigh);
    show_val_kb(m, "LowFree:        ", i.freeram - i.freehigh);

    show_val_kb(m, "MmapCopy:       ",
    (unsigned long)atomic_long_read(&mmap_pages_allocated));

    show_val_kb(m, "SwapTotal:      ", i.totalswap);
    show_val_kb(m, "SwapFree:       ", i.freeswap);

    show_val_kb(m, "Zswap:          ", zswap_total_pages());
    seq_printf(m,  "Zswapped:       %8lu kB\n",
    (unsigned long)atomic_long_read(&zswap_stored_pages) <<
    (PAGE_SHIFT - 10));

    show_val_kb(m, "Dirty:          ",
    global_node_page_state(NR_FILE_DIRTY));
    show_val_kb(m, "Writeback:      ",
    global_node_page_state(NR_WRITEBACK));
    show_val_kb(m, "AnonPages:      ",
    global_node_page_state(NR_ANON_MAPPED));
    show_val_kb(m, "Mapped:         ",
    global_node_page_state(NR_FILE_MAPPED));
    show_val_kb(m, "Shmem:          ", i.sharedram);
    show_val_kb(m, "KReclaimable:   ", sreclaimable +
    global_node_page_state(NR_KERNEL_MISC_RECLAIMABLE));
    show_val_kb(m, "Slab:           ", sreclaimable + sunreclaim);
    show_val_kb(m, "SReclaimable:   ", sreclaimable);
    show_val_kb(m, "SUnreclaim:     ", sunreclaim);
    seq_printf(m, "KernelStack:    %8lu kB\n",
    global_node_page_state(NR_KERNEL_STACK_KB));

    seq_printf(m, "ShadowCallStack:%8lu kB\n",
    global_node_page_state(NR_KERNEL_SCS_KB));

    show_val_kb(m, "PageTables:     ",
    global_node_page_state(NR_PAGETABLE));
    show_val_kb(m, "SecPageTables:  ",
    global_node_page_state(NR_SECONDARY_PAGETABLE));
    show_val_kb(m, "NFS_Unstable:   ", 0);
    show_val_kb(m, "Bounce:         ", 0);
    show_val_kb(m, "WritebackTmp:   ", 0);
    show_val_kb(m, "CommitLimit:    ", vm_commit_limit());
    show_val_kb(m, "Committed_AS:   ", committed);
    seq_printf(m, "VmallocTotal:   %8lu kB\n",
    (unsigned long)VMALLOC_TOTAL >> 10);
    show_val_kb(m, "VmallocUsed:    ",
    global_node_page_state(NR_VMALLOC));
    show_val_kb(m, "VmallocChunk:   ", 0ul);
    show_val_kb(m, "Percpu:         ", pcpu_nr_pages());
    memtest_report_meminfo(m);

    seq_printf(m, "HardwareCorrupted: %5lu kB\n",
    atomic_long_read(&num_poisoned_pages) << (PAGE_SHIFT - 10));

    show_val_kb(m, "AnonHugePages:  ",
    global_node_page_state(NR_ANON_THPS));
    show_val_kb(m, "ShmemHugePages: ",
    global_node_page_state(NR_SHMEM_THPS));
    show_val_kb(m, "ShmemPmdMapped: ",
    global_node_page_state(NR_SHMEM_PMDMAPPED));
    show_val_kb(m, "FileHugePages:  ",
    global_node_page_state(NR_FILE_THPS));
    show_val_kb(m, "FilePmdMapped:  ",
    global_node_page_state(NR_FILE_PMDMAPPED));

    show_val_kb(m, "CmaTotal:       ", totalcma_pages);
    show_val_kb(m, "CmaFree:        ",
    global_zone_page_state(NR_FREE_CMA_PAGES));

    show_val_kb(m, "Unaccepted:     ",
    global_zone_page_state(NR_UNACCEPTED));

    show_val_kb(m, "Balloon:        ",
    global_node_page_state(NR_BALLOON_PAGES));
    show_val_kb(m, "GPUActive:      ",
    global_node_page_state(NR_GPU_ACTIVE));
    show_val_kb(m, "GPUReclaim:     ",
    global_node_page_state(NR_GPU_RECLAIM));
    hugetlb_report_meminfo(m);
    arch_report_meminfo(m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_meminfo_init() -> int __init {
    static int __init proc_meminfo_init(void)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_single("meminfo", 0, core::ptr::null_mut(), meminfo_proc_show);
    pde_make_permanent(pde);
    return 0;
    }
    fs_initcall(proc_meminfo_init);
