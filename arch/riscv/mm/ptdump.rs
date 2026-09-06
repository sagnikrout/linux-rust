//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/ptdump.c
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
// Copyright (C) 2019 SiFive
//

    ({						\
    if (m)					\
    seq_printf(m, fmt, ##args);	\
    })

    ({					\
    if (m)				\
    seq_puts(m, fmt);	\
    })
//
// The page dumper groups page table entries of the same type into a single
// description. It uses pg_state to track the range information while
// iterating over the pte entries. When the continuity is broken it then
// dumps out a description of the range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_state {
    pub ptdump: ptdump_state,
    pub seq: *mut seq_file,
    pub marker: *const addr_marker,
    pub start_address: c_ulong,
    pub start_pa: c_ulong,
    pub last_pa: c_ulong,
    pub level: c_int,
    pub current_prot: u64,
    pub check_wx: bool,
    pub wx_pages: c_ulong,
}

// Address marker
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_marker {
    pub start_address: c_ulong,
    pub name: *const c_char,
}

// Private information for debugfs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptd_mm_info {
    pub mm: *mut mm_struct,
    pub markers: *const addr_marker,
    pub base_addr: c_ulong,
    pub end: c_ulong,
}

    enum address_markers_idx {
    FIXMAP_START_NR,
    FIXMAP_END_NR,
    PCI_IO_START_NR,
    PCI_IO_END_NR,

    VMEMMAP_START_NR,
    VMEMMAP_END_NR,

    VMALLOC_START_NR,
    VMALLOC_END_NR,
    PAGE_OFFSET_NR,

    KASAN_SHADOW_START_NR,
    KASAN_SHADOW_END_NR,

    MODULES_MAPPING_NR,
    KERNEL_MAPPING_NR,

    END_OF_SPACE_NR
    };
    static struct addr_marker address_markers[] = {
    {0, "Fixmap start"},
    {0, "Fixmap end"},
    {0, "PCI I/O start"},
    {0, "PCI I/O end"},

    {0, "vmemmap start"},
    {0, "vmemmap end"},

    {0, "vmalloc() area"},
    {0, "vmalloc() end"},
    {0, "Linear mapping"},

    {0, "Kasan shadow start"},
    {0, "Kasan shadow end"},

    {0, "Modules/BPF mapping"},
    {0, "Kernel mapping"},

    {-1, core::ptr::null_mut()},
    };
    static struct ptd_mm_info kernel_ptd_info = {
    .mm		= &init_mm,
    .markers	= address_markers,
    .base_addr	= 0,
    .end		= ULONG_MAX,
    };

    static struct addr_marker efi_addr_markers[] = {
    { 0,		"UEFI runtime start" },
    { SZ_1G,	"UEFI runtime end" },
    { -1,		core::ptr::null_mut() }
    };
    static struct ptd_mm_info efi_ptd_info = {
    .mm		= &efi_mm,
    .markers	= efi_addr_markers,
    .base_addr	= 0,
    .end		= SZ_2G,
    };

// Page Table Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prot_bits {
    pub mask: u64,
    pub set: *const c_char,
    pub clear: *const c_char,
}

    static const struct prot_bits pte_bits[] = {
    {

    .mask = _PAGE_NAPOT,
    .set = "N",
    .clear = ".",
    }, {
    .mask = _PAGE_MTMASK_SVPBMT,
    .set = "MT(%s)",
    .clear = "  ..  ",
    }, {

    .mask = _PAGE_SOFT,
    .set = "RSW(%d)",
    .clear = "  ..  ",
    }, {
    .mask = _PAGE_DIRTY,
    .set = "D",
    .clear = ".",
    }, {
    .mask = _PAGE_ACCESSED,
    .set = "A",
    .clear = ".",
    }, {
    .mask = _PAGE_GLOBAL,
    .set = "G",
    .clear = ".",
    }, {
    .mask = _PAGE_USER,
    .set = "U",
    .clear = ".",
    }, {
    .mask = _PAGE_EXEC,
    .set = "X",
    .clear = ".",
    }, {
    .mask = _PAGE_WRITE,
    .set = "W",
    .clear = ".",
    }, {
    .mask = _PAGE_READ,
    .set = "R",
    .clear = ".",
    }, {
    .mask = _PAGE_PRESENT,
    .set = "V",
    .clear = ".",
    }
    };
// Page Level
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_level {
    pub name: *const c_char,
    pub mask: u64,
}

    static struct pg_level pg_level[] = {
    { /* pgd */
    .name = "PGD",
    }, { /* p4d */
    .name = (CONFIG_PGTABLE_LEVELS > 4) ? "P4D" : "PGD",
    }, { /* pud */
    .name = (CONFIG_PGTABLE_LEVELS > 3) ? "PUD" : "PGD",
    }, { /* pmd */
    .name = (CONFIG_PGTABLE_LEVELS > 2) ? "PMD" : "PGD",
    }, { /* pte */
    .name = "PTE",
    },
    };
#[no_mangle]
unsafe extern "C" fn dump_prot(st: *mut pg_state) {
    static void dump_prot(struct pg_state *st)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(pte_bits); i++) {
    char s[7];
    unsigned long val;
    val = st.current_prot & pte_bits[i].mask;
    if (val) {
    if (pte_bits[i].mask == _PAGE_SOFT)
    snprintf(s, sizeof(s), pte_bits[i].set, val >> 8);

#[no_mangle]
pub unsafe extern "C" fn if(_PAGE_MTMASK_SVPBMT: pte_bits[i].mask ==) -> else {
    if (val == _PAGE_NOCACHE_SVPBMT)
    snprintf(s, sizeof(s), pte_bits[i].set, "NC");
#[no_mangle]
pub unsafe extern "C" fn if(_PAGE_IO_SVPBMT: val ==) -> else {
    else if (val == _PAGE_IO_SVPBMT)
    snprintf(s, sizeof(s), pte_bits[i].set, "IO");
    else
    snprintf(s, sizeof(s), pte_bits[i].set, "??");
    }

    else
    strscpy(s, pte_bits[i].set);
    } else {
    strscpy(s, pte_bits[i].clear);
    }
    pt_dump_seq_printf(st.seq, " %s", s);
    }
    }

#[no_mangle]
unsafe extern "C" fn dump_addr(st: *mut pg_state, addr: c_ulong) {
    static void dump_addr(struct pg_state *st, unsigned long addr)
    {
    static const char units[] = "KMGTPE";
    const char *unit = units;
    unsigned long delta;
    pt_dump_seq_printf(st.seq, ADDR_FORMAT "-" ADDR_FORMAT "   ",
    st.start_address, addr);
    pt_dump_seq_printf(st.seq, " " ADDR_FORMAT " ", st.start_pa);
    delta = (addr - st.start_address) >> 10;
    while (!(delta & 1023) && unit[1]) {
    delta >>= 10;
    unit++;
    }
    pt_dump_seq_printf(st.seq, "%9lu%c %s", delta, *unit,
    pg_level[st.level].name);
    }
#[no_mangle]
unsafe extern "C" fn note_prot_wx(st: *mut pg_state, addr: c_ulong) {
    static void note_prot_wx(struct pg_state *st, unsigned long addr)
    {
    if (!st.check_wx)
    return;
    if ((st.current_prot & (_PAGE_WRITE | _PAGE_EXEC)) !=
    (_PAGE_WRITE | _PAGE_EXEC))
    return;
    WARN_ONCE(1, "riscv/mm: Found insecure W+X mapping at address %p/%pS\n",
    (void *)st.start_address, (void *)st.start_address);
    st.wx_pages += (addr - st.start_address) / PAGE_SIZE;
    }
    static void note_page(struct ptdump_state *pt_st, unsigned long addr,
    int level, u64 val)
    {
    struct pg_state *st = container_of(pt_st, struct pg_state, ptdump);
    let mut pa: u64 = PFN_PHYS(pte_pfn(__pte(val)));
    let mut prot: u64 = 0;
    if (level >= 0)
    prot = val & pg_level[level].mask;
    if (st.level == -1) {
    st.level = level;
    st.current_prot = prot;
    st.start_address = addr;
    st.start_pa = pa;
    st.last_pa = pa;
    pt_dump_seq_printf(st.seq, "---[ %s ]---\n", st.marker.name);
    } else if (prot != st.current_prot ||
    level != st.level || addr >= st.marker[1].start_address) {
    if (st.current_prot) {
    note_prot_wx(st, addr);
    dump_addr(st, addr);
    dump_prot(st);
    pt_dump_seq_puts(st.seq, "\n");
    }
    while (addr >= st.marker[1].start_address) {
    st.marker++;
    pt_dump_seq_printf(st.seq, "---[ %s ]---\n",
    st.marker.name);
    }
    st.start_address = addr;
    st.start_pa = pa;
    st.last_pa = pa;
    st.current_prot = prot;
    st.level = level;
    } else {
    st.last_pa = pa;
    }
    }
#[no_mangle]
unsafe extern "C" fn note_page_pte(pt_st: *mut ptdump_state, addr: c_ulong, pte: pte_t) {
    static void note_page_pte(struct ptdump_state *pt_st, unsigned long addr, pte_t pte)
    {
    note_page(pt_st, addr, 4, pte_val(pte));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pmd(pt_st: *mut ptdump_state, addr: c_ulong, pmd: pmd_t) {
    static void note_page_pmd(struct ptdump_state *pt_st, unsigned long addr, pmd_t pmd)
    {
    note_page(pt_st, addr, 3, pmd_val(pmd));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pud(pt_st: *mut ptdump_state, addr: c_ulong, pud: pud_t) {
    static void note_page_pud(struct ptdump_state *pt_st, unsigned long addr, pud_t pud)
    {
    note_page(pt_st, addr, 2, pud_val(pud));
    }
#[no_mangle]
unsafe extern "C" fn note_page_p4d(pt_st: *mut ptdump_state, addr: c_ulong, p4d: p4d_t) {
    static void note_page_p4d(struct ptdump_state *pt_st, unsigned long addr, p4d_t p4d)
    {
    note_page(pt_st, addr, 1, p4d_val(p4d));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pgd(pt_st: *mut ptdump_state, addr: c_ulong, pgd: pgd_t) {
    static void note_page_pgd(struct ptdump_state *pt_st, unsigned long addr, pgd_t pgd)
    {
    note_page(pt_st, addr, 0, pgd_val(pgd));
    }
#[no_mangle]
unsafe extern "C" fn note_page_flush(pt_st: *mut ptdump_state) {
    static void note_page_flush(struct ptdump_state *pt_st)
    {
    let mut pte_zero: pte_t = {0};
    note_page(pt_st, 0, -1, pte_val(pte_zero));
    }
#[no_mangle]
unsafe extern "C" fn ptdump_walk(s: *mut seq_file, pinfo: *mut ptd_mm_info) {
    static void ptdump_walk(struct seq_file *s, struct ptd_mm_info *pinfo)
    {
    struct pg_state st = {
    .seq = s,
    .marker = pinfo.markers,
    .level = -1,
    .ptdump = {
    .note_page_pte = note_page_pte,
    .note_page_pmd = note_page_pmd,
    .note_page_pud = note_page_pud,
    .note_page_p4d = note_page_p4d,
    .note_page_pgd = note_page_pgd,
    .note_page_flush = note_page_flush,
    .range = (struct ptdump_range[]) {
    {pinfo.base_addr, pinfo.end},
    {0, 0}
    }
    }
    };
    ptdump_walk_pgd(&st.ptdump, pinfo.mm, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn ptdump_check_wx() -> bool {
    bool ptdump_check_wx(void)
    {
    struct pg_state st = {
    .seq = core::ptr::null_mut(),
    .marker = (struct addr_marker[]) {
    {0, core::ptr::null_mut()},
    {-1, core::ptr::null_mut()},
    },
    .level = -1,
    .check_wx = true,
    .ptdump = {
    .note_page_pte = note_page_pte,
    .note_page_pmd = note_page_pmd,
    .note_page_pud = note_page_pud,
    .note_page_p4d = note_page_p4d,
    .note_page_pgd = note_page_pgd,
    .note_page_flush = note_page_flush,
    .range = (struct ptdump_range[]) {
    {KERN_VIRT_START, ULONG_MAX},
    {0, 0}
    }
    }
    };
    ptdump_walk_pgd(&st.ptdump, &init_mm, core::ptr::null_mut());
    if (st.wx_pages) {
    pr_warn("Checked W+X mappings: failed, %lu W+X pages found\n",
    st.wx_pages);
    return false;
    } else {
    pr_info("Checked W+X mappings: passed, no W+X pages found\n");
    return true;
    }
    }
#[no_mangle]
unsafe extern "C" fn ptdump_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_show(struct seq_file *m, void *v)
    {
    ptdump_walk(m, m.private);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump);
#[no_mangle]
unsafe extern "C" fn ptdump_init() -> int __init {
    static int __init ptdump_init(void)
    {
    unsigned int i, j;
    address_markers[FIXMAP_START_NR].start_address = FIXADDR_START;
    address_markers[FIXMAP_END_NR].start_address = FIXADDR_TOP;
    address_markers[PCI_IO_START_NR].start_address = PCI_IO_START;
    address_markers[PCI_IO_END_NR].start_address = PCI_IO_END;

    address_markers[VMEMMAP_START_NR].start_address = VMEMMAP_START;
    address_markers[VMEMMAP_END_NR].start_address = VMEMMAP_END;

    address_markers[VMALLOC_START_NR].start_address = VMALLOC_START;
    address_markers[VMALLOC_END_NR].start_address = VMALLOC_END;
    address_markers[PAGE_OFFSET_NR].start_address = PAGE_OFFSET;

    address_markers[KASAN_SHADOW_START_NR].start_address = KASAN_SHADOW_START;
    address_markers[KASAN_SHADOW_END_NR].start_address = KASAN_SHADOW_END;

    address_markers[MODULES_MAPPING_NR].start_address = MODULES_VADDR;
    address_markers[KERNEL_MAPPING_NR].start_address = kernel_map.virt_addr;

    kernel_ptd_info.base_addr = KERN_VIRT_START;
    pg_level[1].name = pgtable_l5_enabled ? "P4D" : "PGD";
    pg_level[2].name = pgtable_l4_enabled ? "PUD" : "PGD";
    for (i = 0; i < ARRAY_SIZE(pg_level); i++)
    for (j = 0; j < ARRAY_SIZE(pte_bits); j++)
    pg_level[i].mask |= pte_bits[j].mask;
    debugfs_create_file("kernel_page_tables", 0400, core::ptr::null_mut(), &kernel_ptd_info,
    &ptdump_fops);

    if (efi_enabled(EFI_RUNTIME_SERVICES))
    debugfs_create_file("efi_page_tables", 0400, core::ptr::null_mut(), &efi_ptd_info,
    &ptdump_fops);

    return 0;
    }
    device_initcall(ptdump_init);
