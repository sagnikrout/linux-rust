//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/udbg_scc.c
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
// udbg for zilog scc ports as found on Apple PowerMacs
//
// Copyright (C) 2001-2005 PPC 64 Team, IBM Corp
//

    extern u8 real_readb(volatile u8 __iomem  *addr);
    extern void real_writeb(u8 data, volatile u8 __iomem *addr);
pub const SCC_TXRDY: c_int = 4;
pub const SCC_RXRDY: c_int = 1;
    static volatile u8 __iomem *sccc;
    static volatile u8 __iomem *sccd;
#[no_mangle]
unsafe extern "C" fn udbg_scc_putc(c: c_char) {
    static void udbg_scc_putc(char c)
    {
    if (sccc) {
    while ((in_8(sccc) & SCC_TXRDY) == 0)
    ;
    out_8(sccd,  c);
    if (c == '\n')
    udbg_scc_putc('\r');
    }
    }
#[no_mangle]
unsafe extern "C" fn udbg_scc_getc_poll() -> c_int {
    static int udbg_scc_getc_poll(void)
    {
    if (sccc) {
    if ((in_8(sccc) & SCC_RXRDY) != 0)
    return in_8(sccd);
    else
    return -1;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn udbg_scc_getc() -> c_int {
    static int udbg_scc_getc(void)
    {
    if (sccc) {
    while ((in_8(sccc) & SCC_RXRDY) == 0)
    ;
    return in_8(sccd);
    }
    return -1;
    }
    static unsigned char scc_inittab[] = {
    13, 0,		/* set baud rate divisor */
    12, 0,
    14, 1,		/* baud rate gen enable, src=rtxc */
    11, 0x50,		/* clocks = br gen */
    5,  0xea,		/* tx 8 bits, assert DTR & RTS */
    4,  0x46,		/* x16 clock, 1 stop */
    3,  0xc1,		/* rx enable, 8 bits */
    };
#[no_mangle]
pub unsafe extern "C" fn udbg_scc_init(force_scc: c_int) -> void __init {
    void __init udbg_scc_init(int force_scc)
    {
    const u32 *reg;
    unsigned long addr;
    struct device_node *stdout = core::ptr::null_mut(), *escc = core::ptr::null_mut(), *macio = core::ptr::null_mut();
    struct device_node *ch, *ch_def = core::ptr::null_mut(), *ch_a = core::ptr::null_mut();
    const char *path;
    int i;
    escc = of_find_node_by_name(core::ptr::null_mut(), "escc");
    if (escc == core::ptr::null_mut())
    goto bail;
    macio = of_get_parent(escc);
    if (macio == core::ptr::null_mut())
    goto bail;
    path = of_get_property(of_chosen, "linux,stdout-path", core::ptr::null_mut());
    if (path != core::ptr::null_mut())
    stdout = of_find_node_by_path(path);
    for_each_child_of_node(escc, ch) {
    if (ch == stdout) {
    of_node_put(ch_def);
    ch_def = of_node_get(ch);
    }
    if (of_node_name_eq(ch, "ch-a")) {
    of_node_put(ch_a);
    ch_a = of_node_get(ch);
    }
    }
    if (ch_def == core::ptr::null_mut() && !force_scc)
    goto bail;
    ch = ch_def ? ch_def : ch_a;
// Get address within mac-io ASIC
    reg = of_get_property(escc, "reg", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    goto bail;
    addr = reg[0];
// Get address of mac-io PCI itself
    reg = of_get_property(macio, "assigned-addresses", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    goto bail;
    addr += reg[2];
// Lock the serial port
    pmac_call_feature(PMAC_FTR_SCC_ENABLE, ch,
    PMAC_SCC_ASYNC | PMAC_SCC_FLAG_XMON, 1);
    if (ch == ch_a)
    addr += 0x20;
    sccc = ioremap(addr & PAGE_MASK, PAGE_SIZE) ;
    sccc += addr & ~PAGE_MASK;
    sccd = sccc + 0x10;
    mb();
    for (i = 20000; i != 0; --i)
    in_8(sccc);
    out_8(sccc, 0x09);		/* reset A or B side */
    out_8(sccc, 0xc0);
// If SCC was the OF output port, read the BRG value, else
// Setup for 38400 or 57600 8N1 depending on the machine
//
    if (ch_def != core::ptr::null_mut()) {
    out_8(sccc, 13);
    scc_inittab[1] = in_8(sccc);
    out_8(sccc, 12);
    scc_inittab[3] = in_8(sccc);
    } else if (of_machine_is_compatible("RackMac1,1")
    || of_machine_is_compatible("RackMac1,2")
    || of_machine_is_compatible("MacRISC4")) {
// Xserves and G5s default to 57600
    scc_inittab[1] = 0;
    scc_inittab[3] = 0;
    } else {
// Others default to 38400
    scc_inittab[1] = 0;
    scc_inittab[3] = 1;
    }
    for (i = 0; i < sizeof(scc_inittab); ++i)
    out_8(sccc, scc_inittab[i]);
    udbg_putc = udbg_scc_putc;
    udbg_getc = udbg_scc_getc;
    udbg_getc_poll = udbg_scc_getc_poll;
    udbg_puts("Hello World !\n");
    bail:
    of_node_put(macio);
    of_node_put(escc);
    of_node_put(stdout);
    of_node_put(ch_def);
    of_node_put(ch_a);
    }

#[no_mangle]
unsafe extern "C" fn udbg_real_scc_putc(c: c_char) {
    static void udbg_real_scc_putc(char c)
    {
    while ((real_readb(sccc) & SCC_TXRDY) == 0)
    ;
    real_writeb(c, sccd);
    if (c == '\n')
    udbg_real_scc_putc('\r');
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_pmac_realmode() -> void __init {
    void __init udbg_init_pmac_realmode(void)
    {
    sccc = (volatile u8 __iomem *)0x80013020ul;
    sccd = (volatile u8 __iomem *)0x80013030ul;
    udbg_putc = udbg_real_scc_putc;
    udbg_getc = core::ptr::null_mut();
    udbg_getc_poll = core::ptr::null_mut();
    }
