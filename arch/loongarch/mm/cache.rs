//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/cache.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 1994 - 2003, 06, 07 by Ralf Baechle (ralf@linux-mips.org)
// Copyright (C) 2007 MIPS Technologies, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn cache_error_setup() {
    void cache_error_setup(void)
    {
    extern char __weak except_vec_cex;
    set_merr_handler(0x0, &except_vec_cex, 0x80);
    }
#[no_mangle]
unsafe extern "C" fn flush_cache_leaf(leaf: c_uint) {
    static void flush_cache_leaf(unsigned int leaf)
    {
    int i, j, nr_nodes;
    let mut addr: u64 = CSR_DMW0_BASE;
    struct cache_desc *cdesc = current_cpu_data.cache_leaves + leaf;
    nr_nodes = cache_private(cdesc) ? 1 : loongson_sysconf.nr_nodes;
    do {
    for (i = 0; i < cdesc.sets; i++) {
    for (j = 0; j < cdesc.ways; j++) {
    flush_cache_line(leaf, addr);
    addr++;
    }
    addr -= cdesc.ways;
    addr += cdesc.linesz;
    }
    addr += (1ULL << NODE_ADDRSPACE_SHIFT);
    } while (--nr_nodes > 0);
    }
#[no_mangle]
pub unsafe extern "C" fn __flush_cache_all() -> asmlinkage __visible void {
    asmlinkage __visible void __flush_cache_all(void)
    {
    int leaf;
    struct cache_desc *cdesc = current_cpu_data.cache_leaves;
    let mut cache_present: c_uint = current_cpu_data.cache_leaves_present;
    leaf = cache_present - 1;
    if (cache_inclusive(cdesc + leaf)) {
    flush_cache_leaf(leaf);
    return;
    }
    for (leaf = 0; leaf < cache_present; leaf++)
    flush_cache_leaf(leaf);
    }

    do {											\
    unsigned int cfg1;								\
    \
    cfg1 = read_cpucfg(LOONGARCH_CPUCFG17 + leaf);					\
    if (level == 1)	{								\
    cdesc.flags |= CACHE_PRIVATE;						\
    } else {									\
    if (cfg0 & LXIUPRIV)							\
    cdesc.flags |= CACHE_PRIVATE;					\
    if (cfg0 & LXIUINCL)							\
    cdesc.flags |= CACHE_INCLUSIVE;				\
    }										\
    cdesc.level = level;								\
    cdesc.flags |= CACHE_PRESENT;							\
    cdesc.ways = ((cfg1 & CPUCFG_CACHE_WAYS_M) >> CPUCFG_CACHE_WAYS) + 1;		\
    cdesc.sets = 1 << ((cfg1 & CPUCFG_CACHE_SETS_M) >> CPUCFG_CACHE_SETS);		\
    cdesc.linesz = 1 << ((cfg1 & CPUCFG_CACHE_LSIZE_M) >> CPUCFG_CACHE_LSIZE);	\
    cdesc++; leaf++;								\
    } while (0)
#[no_mangle]
pub unsafe extern "C" fn cpu_cache_init() {
    void cpu_cache_init(void)
    {
    let mut leaf: c_uint = 0, level = 1;
    let mut config: c_uint = read_cpucfg(LOONGARCH_CPUCFG16);
    struct cache_desc *cdesc = current_cpu_data.cache_leaves;
    if (config & L1IUPRE) {
    if (config & L1IUUNIFY)
    cdesc.type = CACHE_TYPE_UNIFIED;
    else
    cdesc.type = CACHE_TYPE_INST;
    populate_cache_properties(config, cdesc, level, leaf);
    }
    if (config & L1DPRE) {
    cdesc.type = CACHE_TYPE_DATA;
    populate_cache_properties(config, cdesc, level, leaf);
    }
    config = config >> 3;
    for (level = 2; level <= CACHE_LEVEL_MAX; level++) {
    if (!config)
    break;
    if (config & LXIUPRE) {
    if (config & LXIUUNIFY)
    cdesc.type = CACHE_TYPE_UNIFIED;
    else
    cdesc.type = CACHE_TYPE_INST;
    populate_cache_properties(config, cdesc, level, leaf);
    }
    if (config & LXDPRE) {
    cdesc.type = CACHE_TYPE_DATA;
    populate_cache_properties(config, cdesc, level, leaf);
    }
    config = config >> 7;
    }
    BUG_ON(leaf > CACHE_LEAVES_MAX);
    current_cpu_data.cache_leaves_present = leaf;
    current_cpu_data.options |= LOONGARCH_CPU_PREFETCH;
    }
    static const pgprot_t protection_map[16] = {
    [VM_NONE]					= __pgprot(_CACHE_CC | _PAGE_USER |
    _PAGE_NO_EXEC | _PAGE_NO_READ |
    (_PAGE_PROTNONE ? : _PAGE_PRESENT)),
    [VM_READ]					= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC),
    [VM_WRITE]					= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC),
    [VM_WRITE | VM_READ]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC),
    [VM_EXEC]					= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_EXEC | VM_READ]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_EXEC | VM_WRITE]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_EXEC | VM_WRITE | VM_READ]			= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_SHARED]					= __pgprot(_CACHE_CC | _PAGE_USER |
    _PAGE_NO_EXEC | _PAGE_NO_READ |
    (_PAGE_PROTNONE ? : _PAGE_PRESENT)),
    [VM_SHARED | VM_READ]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC),
    [VM_SHARED | VM_WRITE]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC | _PAGE_WRITE),
    [VM_SHARED | VM_WRITE | VM_READ]		= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_NO_EXEC | _PAGE_WRITE),
    [VM_SHARED | VM_EXEC]				= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_SHARED | VM_EXEC | VM_READ]			= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT),
    [VM_SHARED | VM_EXEC | VM_WRITE]		= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_WRITE),
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ]	= __pgprot(_CACHE_CC | _PAGE_VALID |
    _PAGE_USER | _PAGE_PRESENT |
    _PAGE_WRITE)
    };
    DECLARE_VM_GET_PAGE_PROT
