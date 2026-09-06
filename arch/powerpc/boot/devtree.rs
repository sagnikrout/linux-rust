//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/devtree.c
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
// devtree.c - convenience functions for device tree manipulation
// Copyright 2007 David Gibson, IBM Corporation.
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//
// Authors: David Gibson <david@gibson.dropbear.id.au>
// Scott Wood <scottwood@freescale.com>
//

#[no_mangle]
pub unsafe extern "C" fn dt_fixup_memory(start: u64, size: u64) {
    void dt_fixup_memory(u64 start, u64 size)
    {
    void *root, *memory;
    int naddr, nsize, i;
    u32 memreg[4];
    root = finddevice("/");
    if (getprop(root, "#address-cells", &naddr, sizeof(naddr)) < 0)
    naddr = 2;
    else
    naddr = be32_to_cpu(naddr);
    if (naddr < 1 || naddr > 2)
    fatal("Can't cope with #address-cells == %d in /\n\r", naddr);
    if (getprop(root, "#size-cells", &nsize, sizeof(nsize)) < 0)
    nsize = 1;
    else
    nsize = be32_to_cpu(nsize);
    if (nsize < 1 || nsize > 2)
    fatal("Can't cope with #size-cells == %d in /\n\r", nsize);
    i = 0;
    if (naddr == 2)
    memreg[i++] = cpu_to_be32(start >> 32);
    memreg[i++] = cpu_to_be32(start & 0xffffffff);
    if (nsize == 2)
    memreg[i++] = cpu_to_be32(size >> 32);
    memreg[i++] = cpu_to_be32(size & 0xffffffff);
    memory = finddevice("/memory");
    if (! memory) {
    memory = create_node(core::ptr::null_mut(), "memory");
    setprop_str(memory, "device_type", "memory");
    }
    printf("Memory <- <0x%x", be32_to_cpu(memreg[0]));
    for (i = 1; i < (naddr + nsize); i++)
    printf(" 0x%x", be32_to_cpu(memreg[i]));
    printf("> (%ldMB)\n\r", (unsigned long)(size >> 20));
    setprop(memory, "reg", memreg, (naddr + nsize)*sizeof(u32));
    }

#[no_mangle]
pub unsafe extern "C" fn dt_fixup_cpu_clocks(cpu: u32, tb: u32, bus: u32) {
    void dt_fixup_cpu_clocks(u32 cpu, u32 tb, u32 bus)
    {
    void *devp = core::ptr::null_mut();
    printf("CPU clock-frequency <- 0x%x (%dMHz)\n\r", cpu, MHZ(cpu));
    printf("CPU timebase-frequency <- 0x%x (%dMHz)\n\r", tb, MHZ(tb));
    if (bus > 0)
    printf("CPU bus-frequency <- 0x%x (%dMHz)\n\r", bus, MHZ(bus));
    while ((devp = find_node_by_devtype(devp, "cpu"))) {
    setprop_val(devp, "clock-frequency", cpu_to_be32(cpu));
    setprop_val(devp, "timebase-frequency", cpu_to_be32(tb));
    if (bus > 0)
    setprop_val(devp, "bus-frequency", cpu_to_be32(bus));
    }
    timebase_period_ns = 1000000000 / tb;
    }
#[no_mangle]
pub unsafe extern "C" fn dt_fixup_clock(path: *const c_char, freq: u32) {
    void dt_fixup_clock(const char *path, u32 freq)
    {
    void *devp = finddevice(path);
    if (devp) {
    printf("%s: clock-frequency <- %x (%dMHz)\n\r", path, freq, MHZ(freq));
    setprop_val(devp, "clock-frequency", cpu_to_be32(freq));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dt_fixup_mac_address_by_alias(alias: *const c_char, addr: *const u8) {
    void dt_fixup_mac_address_by_alias(const char *alias, const u8 *addr)
    {
    void *devp = find_node_by_alias(alias);
    if (devp) {
    printf("%s: local-mac-address <-"
    " %02x:%02x:%02x:%02x:%02x:%02x\n\r", alias,
    addr[0], addr[1], addr[2],
    addr[3], addr[4], addr[5]);
    setprop(devp, "local-mac-address", addr, 6);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dt_fixup_mac_address(index: u32, addr: *const u8) {
    void dt_fixup_mac_address(u32 index, const u8 *addr)
    {
    void *devp = find_node_by_prop_value(core::ptr::null_mut(), "linux,network-index",
    (void*)&index, sizeof(index));
    if (devp) {
    printf("ENET%d: local-mac-address <-"
    " %02x:%02x:%02x:%02x:%02x:%02x\n\r", index,
    addr[0], addr[1], addr[2],
    addr[3], addr[4], addr[5]);
    setprop(devp, "local-mac-address", addr, 6);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __dt_fixup_mac_addresses(startindex: u32, ...) {
    void __dt_fixup_mac_addresses(u32 startindex, ...)
    {
    va_list ap;
    let mut index: u32 = startindex;
    const u8 *addr;
    va_start(ap, startindex);
    while ((addr = va_arg(ap, const u8 *)))
    dt_fixup_mac_address(index++, addr);
    va_end(ap);
    }
pub const MAX_ADDR_CELLS: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn dt_get_reg_format(node: *mut c_void, naddr: *mut u32, nsize: *mut u32) {
    void dt_get_reg_format(void *node, u32 *naddr, u32 *nsize)
    {
    if (getprop(node, "#address-cells", naddr, 4) != 4)
// naddr = 2;
    else
// naddr = be32_to_cpu(*naddr);
    if (getprop(node, "#size-cells", nsize, 4) != 4)
// nsize = 1;
    else
// nsize = be32_to_cpu(*nsize);
    }
#[no_mangle]
unsafe extern "C" fn copy_val(dest: *mut u32, src: *mut u32, naddr: c_int) {
    static void copy_val(u32 *dest, u32 *src, int naddr)
    {
    let mut pad: c_int = MAX_ADDR_CELLS - naddr;
    memset(dest, 0, pad * 4);
    memcpy(dest + pad, src, naddr * 4);
    }
#[no_mangle]
unsafe extern "C" fn sub_reg(reg: *mut u32, sub: *mut u32) -> c_int {
    static int sub_reg(u32 *reg, u32 *sub)
    {
    int i, borrow = 0;
    for (i = MAX_ADDR_CELLS - 1; i >= 0; i--) {
    let mut prev_borrow: c_int = borrow;
    borrow = reg[i] < sub[i] + prev_borrow;
    reg[i] -= sub[i] + prev_borrow;
    }
    return !borrow;
    }
#[no_mangle]
unsafe extern "C" fn add_reg(reg: *mut u32, add: *mut u32, naddr: c_int) -> c_int {
    static int add_reg(u32 *reg, u32 *add, int naddr)
    {
    int i, carry = 0;
    for (i = MAX_ADDR_CELLS - 1; i >= MAX_ADDR_CELLS - naddr; i--) {
    let mut tmp: u64 = (u64)be32_to_cpu(reg[i]) + be32_to_cpu(add[i]) + carry;
    carry = tmp >> 32;
    reg[i] = cpu_to_be32((u32)tmp);
    }
    return !carry;
    }
// It is assumed that if the first byte of reg fits in a
// range, then the whole reg block fits.
//
#[no_mangle]
unsafe extern "C" fn compare_reg(reg: *mut u32, range: *mut u32, rangesize: *mut u32) -> c_int {
    static int compare_reg(u32 *reg, u32 *range, u32 *rangesize)
    {
    int i;
    u32 end;
    for (i = 0; i < MAX_ADDR_CELLS; i++) {
    if (be32_to_cpu(reg[i]) < be32_to_cpu(range[i]))
    return 0;
    if (be32_to_cpu(reg[i]) > be32_to_cpu(range[i]))
    break;
    }
    for (i = 0; i < MAX_ADDR_CELLS; i++) {
    end = be32_to_cpu(range[i]) + be32_to_cpu(rangesize[i]);
    if (be32_to_cpu(reg[i]) < end)
    break;
    if (be32_to_cpu(reg[i]) > end)
    return 0;
    }
    return reg[i] != end;
    }
// reg must be MAX_ADDR_CELLS
    static int find_range(u32 *reg, u32 *ranges, int nregaddr,
    int naddr, int nsize, int buflen)
    {
    let mut nrange: c_int = nregaddr + naddr + nsize;
    int i;
    for (i = 0; i + nrange <= buflen; i += nrange) {
    u32 range_addr[MAX_ADDR_CELLS];
    u32 range_size[MAX_ADDR_CELLS];
    copy_val(range_addr, ranges + i, nregaddr);
    copy_val(range_size, ranges + i + nregaddr + naddr, nsize);
    if (compare_reg(reg, range_addr, range_size))
    return i;
    }
    return -1;
    }
// Currently only generic buses without special encodings are supported.
// In particular, PCI is not supported.  Also, only the beginning of the
// reg block is tracked; size is ignored except in ranges.
//
    static u32 prop_buf[MAX_PROP_LEN / 4];
    static int dt_xlate(void *node, int res, int reglen, unsigned long *addr,
    unsigned long *size)
    {
    u32 last_addr[MAX_ADDR_CELLS];
    u32 this_addr[MAX_ADDR_CELLS];
    void *parent;
    u64 ret_addr, ret_size;
    u32 naddr, nsize, prev_naddr, prev_nsize;
    int buflen, offset;
    parent = get_parent(node);
    if (!parent)
    return 0;
    dt_get_reg_format(parent, &naddr, &nsize);
    if (nsize > 2)
    return 0;
    offset = (naddr + nsize) * res;
    if (reglen < offset + naddr + nsize ||
    MAX_PROP_LEN < (offset + naddr + nsize) * 4)
    return 0;
    copy_val(last_addr, prop_buf + offset, naddr);
    ret_size = be32_to_cpu(prop_buf[offset + naddr]);
    if (nsize == 2) {
    ret_size <<= 32;
    ret_size |= be32_to_cpu(prop_buf[offset + naddr + 1]);
    }
    for (;;) {
    prev_naddr = naddr;
    prev_nsize = nsize;
    node = parent;
    parent = get_parent(node);
    if (!parent)
    break;
    dt_get_reg_format(parent, &naddr, &nsize);
    buflen = getprop(node, "ranges", prop_buf,
    sizeof(prop_buf));
    if (buflen == 0)
    continue;
    if (buflen < 0 || buflen > sizeof(prop_buf))
    return 0;
    offset = find_range(last_addr, prop_buf, prev_naddr,
    naddr, prev_nsize, buflen / 4);
    if (offset < 0)
    return 0;
    copy_val(this_addr, prop_buf + offset, prev_naddr);
    if (!sub_reg(last_addr, this_addr))
    return 0;
    copy_val(this_addr, prop_buf + offset + prev_naddr, naddr);
    if (!add_reg(last_addr, this_addr, naddr))
    return 0;
    }
    if (naddr > 2)
    return 0;
    ret_addr = ((u64)be32_to_cpu(last_addr[2]) << 32) | be32_to_cpu(last_addr[3]);
    if (sizeof(void *) == 4 &&
    (ret_addr >= 0x100000000ULL || ret_size > 0x100000000ULL ||
    ret_addr + ret_size > 0x100000000ULL))
    return 0;
// addr = ret_addr;
    if (size)
// size = ret_size;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn dt_xlate_reg(node: *mut c_void, res: c_int, addr: *mut c_ulong, size: *mut c_ulong) -> c_int {
    int dt_xlate_reg(void *node, int res, unsigned long *addr, unsigned long *size)
    {
    int reglen;
    reglen = getprop(node, "reg", prop_buf, sizeof(prop_buf)) / 4;
    return dt_xlate(node, res, reglen, addr, size);
    }
#[no_mangle]
pub unsafe extern "C" fn dt_xlate_addr(node: *mut c_void, buf: *mut u32, buflen: c_int, xlated_addr: *mut c_ulong) -> c_int {
    int dt_xlate_addr(void *node, u32 *buf, int buflen, unsigned long *xlated_addr)
    {
    if (buflen > sizeof(prop_buf))
    return 0;
    memcpy(prop_buf, buf, buflen);
    return dt_xlate(node, 0, buflen / 4, xlated_addr, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn dt_is_compatible(node: *mut c_void, compat: *const c_char) -> c_int {
    int dt_is_compatible(void *node, const char *compat)
    {
    char *buf = (char *)prop_buf;
    int len, pos;
    len = getprop(node, "compatible", buf, MAX_PROP_LEN);
    if (len < 0)
    return 0;
    for (pos = 0; pos < len; pos++) {
    if (!strcmp(buf + pos, compat))
    return 1;
    pos += strnlen(&buf[pos], len - pos);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dt_get_virtual_reg(node: *mut c_void, addr: *mut c_void, nres: c_int) -> c_int {
    int dt_get_virtual_reg(void *node, void **addr, int nres)
    {
    unsigned long xaddr;
    int n, i;
    n = getprop(node, "virtual-reg", addr, nres * 4);
    if (n > 0) {
    for (i = 0; i < n/4; i ++)
    ((u32 *)addr)[i] = be32_to_cpu(((u32 *)addr)[i]);
    return n / 4;
    }
    for (n = 0; n < nres; n++) {
    if (!dt_xlate_reg(node, n, &xaddr, core::ptr::null_mut()))
    break;
    addr[n] = (void *)xaddr;
    }
    return n;
    }
