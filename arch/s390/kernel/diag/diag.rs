//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/diag/diag.c
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
// Implementation of s390 diagnose codes
//
// Copyright IBM Corp. 2007
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_stat {
    pub counter: [c_uint; NR_DIAG_STAT],
}

    static DEFINE_PER_CPU(struct diag_stat, diag_stat);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_desc {
    pub code: c_int,
    pub name: *mut c_char,
}

    static const struct diag_desc diag_map[NR_DIAG_STAT] = {
    [DIAG_STAT_X008] = { .code = 0x008, .name = "Console Function" },
    [DIAG_STAT_X00C] = { .code = 0x00c, .name = "Pseudo Timer" },
    [DIAG_STAT_X010] = { .code = 0x010, .name = "Release Pages" },
    [DIAG_STAT_X014] = { .code = 0x014, .name = "Spool File Services" },
    [DIAG_STAT_X044] = { .code = 0x044, .name = "Voluntary Timeslice End" },
    [DIAG_STAT_X064] = { .code = 0x064, .name = "NSS Manipulation" },
    [DIAG_STAT_X08C] = { .code = 0x08c, .name = "Access 3270 Display Device Information" },
    [DIAG_STAT_X09C] = { .code = 0x09c, .name = "Relinquish Timeslice" },
    [DIAG_STAT_X0DC] = { .code = 0x0dc, .name = "Appldata Control" },
    [DIAG_STAT_X204] = { .code = 0x204, .name = "Logical-CPU Utilization" },
    [DIAG_STAT_X210] = { .code = 0x210, .name = "Device Information" },
    [DIAG_STAT_X224] = { .code = 0x224, .name = "EBCDIC-Name Table" },
    [DIAG_STAT_X250] = { .code = 0x250, .name = "Block I/O" },
    [DIAG_STAT_X258] = { .code = 0x258, .name = "Page-Reference Services" },
    [DIAG_STAT_X26C] = { .code = 0x26c, .name = "Certain System Information" },
    [DIAG_STAT_X288] = { .code = 0x288, .name = "Time Bomb" },
    [DIAG_STAT_X2C4] = { .code = 0x2c4, .name = "FTP Services" },
    [DIAG_STAT_X2FC] = { .code = 0x2fc, .name = "Guest Performance Data" },
    [DIAG_STAT_X304] = { .code = 0x304, .name = "Partition-Resource Service" },
    [DIAG_STAT_X308] = { .code = 0x308, .name = "List-Directed IPL" },
    [DIAG_STAT_X310] = { .code = 0x310, .name = "Memory Topology Information" },
    [DIAG_STAT_X318] = { .code = 0x318, .name = "CP Name and Version Codes" },
    [DIAG_STAT_X320] = { .code = 0x320, .name = "Certificate Store" },
    [DIAG_STAT_X324] = { .code = 0x324, .name = "Power Information Block" },
    [DIAG_STAT_X49C] = { .code = 0x49c, .name = "Warning-Track Interruption" },
    [DIAG_STAT_X500] = { .code = 0x500, .name = "Virtio Service" },
    };
    struct diag_ops __amode31_ref diag_amode31_ops = {
    .diag210 = _diag210_amode31,
    .diag26c = _diag26c_amode31,
    .diag14 = _diag14_amode31,
    .diag0c = _diag0c_amode31,
    .diag8c = _diag8c_amode31,
    .diag308_reset = _diag308_reset_amode31
    };
    static struct diag210 _diag210_tmp_amode31 __section(".amode31.data");
    struct diag210 __amode31_ref *__diag210_tmp_amode31 = &_diag210_tmp_amode31;
    static struct diag8c _diag8c_tmp_amode31 __section(".amode31.data");
    static struct diag8c __amode31_ref *__diag8c_tmp_amode31 = &_diag8c_tmp_amode31;
#[no_mangle]
unsafe extern "C" fn show_diag_stat(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_diag_stat(struct seq_file *m, void *v)
    {
    struct diag_stat *stat;
    let mut n: c_ulong = (unsigned long) v - 1;
    int cpu, prec, tmp;
    cpus_read_lock();
    if (n == 0) {
    seq_puts(m, "         ");
    for_each_online_cpu(cpu) {
    prec = 10;
    for (tmp = 10; cpu >= tmp; tmp *= 10)
    prec--;
    seq_printf(m, "%*s%d", prec, "CPU", cpu);
    }
    seq_putc(m, '\n');
    } else if (n <= NR_DIAG_STAT) {
    seq_printf(m, "diag %03x:", diag_map[n-1].code);
    for_each_online_cpu(cpu) {
    stat = &per_cpu(diag_stat, cpu);
    seq_printf(m, " %10u", stat.counter[n-1]);
    }
    seq_printf(m, "    %s\n", diag_map[n-1].name);
    }
    cpus_read_unlock();
    return 0;
    }
    static void *show_diag_stat_start(struct seq_file *m, loff_t *pos)
    {
    return *pos <= NR_DIAG_STAT ? (void *)((unsigned long) *pos + 1) : core::ptr::null_mut();
    }
    static void *show_diag_stat_next(struct seq_file *m, void *v, loff_t *pos)
    {
    ++*pos;
    return show_diag_stat_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn show_diag_stat_stop(m: *mut seq_file, v: *mut c_void) {
    static void show_diag_stat_stop(struct seq_file *m, void *v)
    {
    }
    static const struct seq_operations show_diag_stat_sops = {
    .start	= show_diag_stat_start,
    .next	= show_diag_stat_next,
    .stop	= show_diag_stat_stop,
    .show	= show_diag_stat,
    };
    DEFINE_SEQ_ATTRIBUTE(show_diag_stat);
#[no_mangle]
unsafe extern "C" fn show_diag_stat_init() -> int __init {
    static int __init show_diag_stat_init(void)
    {
    debugfs_create_file("diag_stat", 0400, core::ptr::null_mut(), core::ptr::null_mut(),
    &show_diag_stat_fops);
    return 0;
    }
    device_initcall(show_diag_stat_init);
#[no_mangle]
pub unsafe extern "C" fn diag_stat_inc(nr: enum diag_stat_enum) {
    void diag_stat_inc(enum diag_stat_enum nr)
    {
    this_cpu_inc(diag_stat.counter[nr]);
    trace_s390_diagnose(diag_map[nr].code);
    }
    EXPORT_SYMBOL(diag_stat_inc);
#[no_mangle]
pub unsafe extern "C" fn diag_stat_inc_norecursion(nr: enum diag_stat_enum) -> void notrace {
    void notrace diag_stat_inc_norecursion(enum diag_stat_enum nr)
    {
    this_cpu_inc(diag_stat.counter[nr]);
    trace_s390_diagnose_norecursion(diag_map[nr].code);
    }
    EXPORT_SYMBOL(diag_stat_inc_norecursion);
//
// Diagnose 0c: Pseudo Timer
//
#[no_mangle]
pub unsafe extern "C" fn diag0c(data: *mut hypfs_diag0c_entry) {
    void diag0c(struct hypfs_diag0c_entry *data)
    {
    diag_stat_inc(DIAG_STAT_X00C);
    diag_amode31_ops.diag0c(virt_to_phys(data));
    }
//
// Diagnose 14: Input spool file manipulation
//
// The subcode parameter determines the type of the first parameter rx.
// Currently used are the following 3 subcommands:
// 0x0:   Read the Next Spool File Buffer (Data Record)
// 0x28:  Position a Spool File to the Designated Record
// 0xfff: Retrieve Next File Descriptor
//
// For subcommands 0x0 and 0xfff, the value of the first parameter is
// a virtual address of a memory buffer and needs virtual to physical
// address translation. For other subcommands the rx parameter is not
// a virtual address.
//
#[no_mangle]
pub unsafe extern "C" fn diag14(rx: c_ulong, ry1: c_ulong, subcode: c_ulong) -> c_int {
    int diag14(unsigned long rx, unsigned long ry1, unsigned long subcode)
    {
    diag_stat_inc(DIAG_STAT_X014);
    switch (subcode) {
    case 0x0:
    case 0xfff:
    rx = virt_to_phys((void *)rx);
    break;
    default:
// Do nothing
    break;
    }
    return diag_amode31_ops.diag14(rx, ry1, subcode);
    }
    EXPORT_SYMBOL(diag14);
pub const DIAG204_BUSY_RC: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn __diag204(subcode: *mut c_ulong, size: c_ulong, addr: *mut c_void) -> c_int {
    static inline int __diag204(unsigned long *subcode, unsigned long size, void *addr)
    {
    let mut rp: union register_pair = { .even = *subcode, .odd = size };
    asm_inline volatile(
    "	diag	%[addr],%[rp],0x204\n"
    "0:	nopr	%%r7\n"
    EX_TABLE(0b,0b)
    : [rp] "+&d" (rp.pair) : [addr] "d" (addr) : "memory");
// subcode = rp.even;
    return rp.odd;
    }
//
// diag204() - Issue diagnose 204 call.
// @subcode: Subcode of diagnose 204 to be executed.
// @size: Size of area in pages which @area points to, if given.
// @addr: Vmalloc'ed memory area where the result is written to.
//
// Execute diagnose 204 with the given subcode and write the result to the
// memory area specified with @addr. For subcodes which do not write a
// result to memory both @size and @addr must be zero. If @addr is
// specified it must be page aligned and must have been allocated with
// vmalloc(). Conversion to real / physical addresses will be handled by
// this function if required.
//
#[no_mangle]
pub unsafe extern "C" fn diag204(subcode: c_ulong, size: c_ulong, addr: *mut c_void) -> c_int {
    int diag204(unsigned long subcode, unsigned long size, void *addr)
    {
    if (addr) {
    if (WARN_ON_ONCE(!is_vmalloc_addr(addr)))
    return -EINVAL;
    if (WARN_ON_ONCE(!IS_ALIGNED((unsigned long)addr, PAGE_SIZE)))
    return -EINVAL;
    }
    if ((subcode & DIAG204_SUBCODE_MASK) == DIAG204_SUBC_STIB4)
    addr = (void *)pfn_to_phys(vmalloc_to_pfn(addr));
    diag_stat_inc(DIAG_STAT_X204);
    size = __diag204(&subcode, size, addr);
    if (subcode == DIAG204_BUSY_RC)
    return -EBUSY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: subcode) -> else {
    else if (subcode)
    return -EOPNOTSUPP;
    return size;
    }
    EXPORT_SYMBOL(diag204);
//
// Diagnose 210: Get information about a virtual device
//
#[no_mangle]
pub unsafe extern "C" fn diag210(addr: *mut diag210) -> c_int {
    int diag210(struct diag210 *addr)
    {
    static DEFINE_SPINLOCK(diag210_lock);
    unsigned long flags;
    int ccode;
    spin_lock_irqsave(&diag210_lock, flags);
// __diag210_tmp_amode31 = *addr;
    diag_stat_inc(DIAG_STAT_X210);
    ccode = diag_amode31_ops.diag210(__diag210_tmp_amode31);
// addr = *__diag210_tmp_amode31;
    spin_unlock_irqrestore(&diag210_lock, flags);
    return ccode;
    }
    EXPORT_SYMBOL(diag210);
//
// Diagnose 8C: Access 3270 Display Device Information
//
#[no_mangle]
pub unsafe extern "C" fn diag8c(addr: *mut diag8c, devno: *mut ccw_dev_id) -> c_int {
    int diag8c(struct diag8c *addr, struct ccw_dev_id *devno)
    {
    static DEFINE_SPINLOCK(diag8c_lock);
    unsigned long flags;
    int ccode;
    spin_lock_irqsave(&diag8c_lock, flags);
    diag_stat_inc(DIAG_STAT_X08C);
    ccode = diag_amode31_ops.diag8c(__diag8c_tmp_amode31, devno, sizeof(*addr));
// addr = *__diag8c_tmp_amode31;
    spin_unlock_irqrestore(&diag8c_lock, flags);
    return ccode;
    }
    EXPORT_SYMBOL(diag8c);
#[no_mangle]
pub unsafe extern "C" fn diag224(ptr: *mut c_void) -> c_int {
    int diag224(void *ptr)
    {
    let mut addr: c_ulong = __pa(ptr);
    let mut rc: c_int = -EOPNOTSUPP;
    diag_stat_inc(DIAG_STAT_X224);
    asm_inline volatile("\n"
    "	diag	%[type],%[addr],0x224\n"
    "0:	lhi	%[rc],0\n"
    "1:\n"
    EX_TABLE(0b,1b)
    : [rc] "+d" (rc)
    , "=m" (*(struct { char buf[PAGE_SIZE]; } *)ptr)
    : [type] "d" (0), [addr] "d" (addr));
    return rc;
    }
    EXPORT_SYMBOL(diag224);
//
// Diagnose 26C: Access Certain System Information
//
#[no_mangle]
pub unsafe extern "C" fn diag26c(req: *mut c_void, resp: *mut c_void, subcode: enum diag26c_sc) -> c_int {
    int diag26c(void *req, void *resp, enum diag26c_sc subcode)
    {
    diag_stat_inc(DIAG_STAT_X26C);
    return diag_amode31_ops.diag26c(virt_to_phys(req), virt_to_phys(resp), subcode);
    }
    EXPORT_SYMBOL(diag26c);
#[no_mangle]
pub unsafe extern "C" fn diag49c(subcode: c_ulong) -> c_int {
    int diag49c(unsigned long subcode)
    {
    int cc;
    diag_stat_inc(DIAG_STAT_X49C);
    asm volatile(
    "	diag	%[subcode],0,0x49c\n"
    CC_IPM(cc)
    : CC_OUT(cc, cc)
    : [subcode] "d" (subcode)
    : CC_CLOBBER);
    return CC_TRANSFORM(cc);
    }
    EXPORT_SYMBOL(diag49c);
