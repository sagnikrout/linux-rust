//! Automatically rewritten from C to Rust
//! Source: arch/s390/pci/pci_insn.c
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
// s390 specific pci instructions
//
// Copyright IBM Corp. 2013
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_err_insn_data {
    pub insn: u8,
    pub cc: u8,
    pub status: u8,
    union {
    struct {
    pub req: u64,
    pub offset: u64,
}

    struct {
    u64 addr;
    u64 len;
    };
    };
    } __packed;
    static inline void zpci_err_insn_req(int lvl, u8 insn, u8 cc, u8 status,
    u64 req, u64 offset)
    {
    struct zpci_err_insn_data data = {
    .insn = insn, .cc = cc, .status = status,
    .req = req, .offset = offset};
    zpci_err_hex_level(lvl, &data, sizeof(data));
    }
    static inline void zpci_err_insn_addr(int lvl, u8 insn, u8 cc, u8 status,
    u64 addr, u64 len)
    {
    struct zpci_err_insn_data data = {
    .insn = insn, .cc = cc, .status = status,
    .addr = addr, .len = len};
    zpci_err_hex_level(lvl, &data, sizeof(data));
    }
// Modify PCI Function Controls
#[no_mangle]
pub unsafe extern "C" fn __mpcifc(req: u64, fib: *mut zpci_fib, status: *mut u8) -> u8 {
    static inline u8 __mpcifc(u64 req, struct zpci_fib *fib, u8 *status)
    {
    int cc;
    asm volatile (
    "	.insn	rxy,0xe300000000d0,%[req],%[fib]\n"
    CC_IPM(cc)
    : CC_OUT(cc, cc), [req] "+d" (req), [fib] "+Q" (*fib)
    :
    : CC_CLOBBER);
// status = req >> 24 & 0xff;
    return CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_mod_fc(req: u64, fib: *mut zpci_fib, status: *mut u8) -> u8 {
    u8 zpci_mod_fc(u64 req, struct zpci_fib *fib, u8 *status)
    {
    let mut retried: bool = false;
    u8 cc;
    do {
    cc = __mpcifc(req, fib, status);
    if (cc == 2) {
    msleep(ZPCI_INSN_BUSY_DELAY);
    if (!retried) {
    zpci_err_insn_req(1, 'M', cc, *status, req, 0);
    retried = true;
    }
    }
    } while (cc == 2);
    if (cc)
    zpci_err_insn_req(0, 'M', cc, *status, req, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: retried) -> else {
    else if (retried)
    zpci_err_insn_req(1, 'M', cc, *status, req, 0);
    return cc;
    }
    EXPORT_SYMBOL_GPL(zpci_mod_fc);
// Refresh PCI Translations
#[no_mangle]
pub unsafe extern "C" fn __rpcit(fn: u64, addr: u64, range: u64, status: *mut u8) -> u8 {
    static inline u8 __rpcit(u64 fn, u64 addr, u64 range, u8 *status)
    {
    let mut addr_range: union register_pair = {.even = addr, .odd = range};
    int cc;
    asm volatile (
    "	.insn	rre,0xb9d30000,%[fn],%[addr_range]\n"
    CC_IPM(cc)
    : CC_OUT(cc, cc), [fn] "+d" (fn)
    : [addr_range] "d" (addr_range.pair)
    : CC_CLOBBER);
// status = fn >> 24 & 0xff;
    return CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_refresh_trans(fn: u64, addr: u64, range: u64) -> c_int {
    int zpci_refresh_trans(u64 fn, u64 addr, u64 range)
    {
    let mut retried: bool = false;
    u8 cc, status;
    do {
    cc = __rpcit(fn, addr, range, &status);
    if (cc == 2) {
    udelay(ZPCI_INSN_BUSY_DELAY);
    if (!retried) {
    zpci_err_insn_addr(1, 'R', cc, status, addr, range);
    retried = true;
    }
    }
    } while (cc == 2);
    if (cc)
    zpci_err_insn_addr(0, 'R', cc, status, addr, range);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: retried) -> else {
    else if (retried)
    zpci_err_insn_addr(1, 'R', cc, status, addr, range);
    if (cc == 1 && (status == 4 || status == 16))
    return -ENOMEM;
    return (cc) ? -EIO : 0;
    }
// Set Interruption Controls
#[no_mangle]
pub unsafe extern "C" fn zpci_set_irq_ctrl(ctl: u16, isc: u8, iib: *mut union zpci_sic_iib) -> c_int {
    int zpci_set_irq_ctrl(u16 ctl, u8 isc, union zpci_sic_iib *iib)
    {
    if (!test_facility(72))
    return -EIO;
    asm volatile(
    ".insn	rsy,0xeb00000000d1,%[ctl],%[isc],%[iib]"
    : : [ctl] "d" (ctl), [isc] "d" (isc << 27), [iib] "Q" (*iib));
    return 0;
    }
    EXPORT_SYMBOL_GPL(zpci_set_irq_ctrl);
// PCI Load
#[no_mangle]
pub unsafe extern "C" fn ____pcilg(data: *mut u64, req: u64, offset: u64, status: *mut u8) -> c_int {
    static inline int ____pcilg(u64 *data, u64 req, u64 offset, u8 *status)
    {
    let mut req_off: union register_pair = {.even = req, .odd = offset};
    int cc, exception;
    u64 __data;
    exception = 1;
    asm_inline volatile (
    "	.insn	rre,0xb9d20000,%[data],%[req_off]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [data] "=d" (__data),
    [req_off] "+d" (req_off.pair), [exc] "+d" (exception)
    :
    : CC_CLOBBER);
// status = req_off.even >> 24 & 0xff;
// data = __data;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn __pcilg(data: *mut u64, req: u64, offset: u64, status: *mut u8) -> c_int {
    static inline int __pcilg(u64 *data, u64 req, u64 offset, u8 *status)
    {
    u64 __data;
    int cc;
    cc = ____pcilg(&__data, req, offset, status);
    if (!cc)
// data = __data;
    return cc;
    }
#[no_mangle]
pub unsafe extern "C" fn __zpci_load(data: *mut u64, req: u64, offset: u64) -> c_int {
    int __zpci_load(u64 *data, u64 req, u64 offset)
    {
    let mut retried: bool = false;
    u8 status;
    int cc;
    do {
    cc = __pcilg(data, req, offset, &status);
    if (cc == 2) {
    udelay(ZPCI_INSN_BUSY_DELAY);
    if (!retried) {
    zpci_err_insn_req(1, 'l', cc, status, req, offset);
    retried = true;
    }
    }
    } while (cc == 2);
    if (cc)
    zpci_err_insn_req(0, 'l', cc, status, req, offset);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: retried) -> else {
    else if (retried)
    zpci_err_insn_req(1, 'l', cc, status, req, offset);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(__zpci_load);
    static inline int zpci_load_fh(u64 *data, const volatile void __iomem *addr,
    unsigned long len)
    {
    struct zpci_iomap_entry *entry = &zpci_iomap_start[ZPCI_IDX(addr)];
    let mut req: u64 = ZPCI_CREATE_REQ(READ_ONCE(entry.fh), entry.bar, len);
    return __zpci_load(data, req, ZPCI_OFFSET(addr));
    }
#[no_mangle]
pub unsafe extern "C" fn __pcilg_mio(data: *mut u64, ioaddr: u64, len: u64, status: *mut u8) -> c_int {
    static inline int __pcilg_mio(u64 *data, u64 ioaddr, u64 len, u8 *status)
    {
    let mut ioaddr_len: union register_pair = {.even = ioaddr, .odd = len};
    int cc, exception;
    u64 __data;
    exception = 1;
    asm_inline volatile (
    "       .insn   rre,0xb9d60000,%[data],%[ioaddr_len]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [data] "=d" (__data),
    [ioaddr_len] "+d" (ioaddr_len.pair), [exc] "+d" (exception)
    :
    : CC_CLOBBER);
// status = ioaddr_len.odd >> 24 & 0xff;
// data = __data;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_load(data: *mut u64, addr: *const volatile void __iomem, len: c_ulong) -> c_int {
    int zpci_load(u64 *data, const volatile void __iomem *addr, unsigned long len)
    {
    u8 status;
    int cc;
    if (!static_branch_unlikely(&have_mio))
    return zpci_load_fh(data, addr, len);
    cc = __pcilg_mio(data, ( u64) addr, len, &status);
    if (cc)
    zpci_err_insn_addr(0, 'L', cc, status, ( u64) addr, len);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(zpci_load);
// PCI Store
#[no_mangle]
pub unsafe extern "C" fn __pcistg(data: u64, req: u64, offset: u64, status: *mut u8) -> c_int {
    static inline int __pcistg(u64 data, u64 req, u64 offset, u8 *status)
    {
    let mut req_off: union register_pair = {.even = req, .odd = offset};
    int cc, exception;
    exception = 1;
    asm_inline volatile (
    "	.insn	rre,0xb9d00000,%[data],%[req_off]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [req_off] "+d" (req_off.pair), [exc] "+d" (exception)
    : [data] "d" (data)
    : CC_CLOBBER);
// status = req_off.even >> 24 & 0xff;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn __zpci_store(data: u64, req: u64, offset: u64) -> c_int {
    int __zpci_store(u64 data, u64 req, u64 offset)
    {
    let mut retried: bool = false;
    u8 status;
    int cc;
    do {
    cc = __pcistg(data, req, offset, &status);
    if (cc == 2) {
    udelay(ZPCI_INSN_BUSY_DELAY);
    if (!retried) {
    zpci_err_insn_req(1, 's', cc, status, req, offset);
    retried = true;
    }
    }
    } while (cc == 2);
    if (cc)
    zpci_err_insn_req(0, 's', cc, status, req, offset);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: retried) -> else {
    else if (retried)
    zpci_err_insn_req(1, 's', cc, status, req, offset);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(__zpci_store);
    static inline int zpci_store_fh(const volatile void __iomem *addr, u64 data,
    unsigned long len)
    {
    struct zpci_iomap_entry *entry = &zpci_iomap_start[ZPCI_IDX(addr)];
    let mut req: u64 = ZPCI_CREATE_REQ(READ_ONCE(entry.fh), entry.bar, len);
    return __zpci_store(data, req, ZPCI_OFFSET(addr));
    }
#[no_mangle]
pub unsafe extern "C" fn __pcistg_mio(data: u64, ioaddr: u64, len: u64, status: *mut u8) -> c_int {
    static inline int __pcistg_mio(u64 data, u64 ioaddr, u64 len, u8 *status)
    {
    let mut ioaddr_len: union register_pair = {.even = ioaddr, .odd = len};
    int cc, exception;
    exception = 1;
    asm_inline volatile (
    "       .insn   rre,0xb9d40000,%[data],%[ioaddr_len]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [ioaddr_len] "+d" (ioaddr_len.pair), [exc] "+d" (exception)
    : [data] "d" (data)
    : CC_CLOBBER_LIST("memory"));
// status = ioaddr_len.odd >> 24 & 0xff;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_store(addr: *const volatile void __iomem, data: u64, len: c_ulong) -> c_int {
    int zpci_store(const volatile void __iomem *addr, u64 data, unsigned long len)
    {
    u8 status;
    int cc;
    if (!static_branch_unlikely(&have_mio))
    return zpci_store_fh(addr, data, len);
    cc = __pcistg_mio(data, ( u64) addr, len, &status);
    if (cc)
    zpci_err_insn_addr(0, 'S', cc, status, ( u64) addr, len);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(zpci_store);
// PCI Store Block
#[no_mangle]
pub unsafe extern "C" fn __pcistb(data: *const u64, req: u64, offset: u64, status: *mut u8) -> c_int {
    static inline int __pcistb(const u64 *data, u64 req, u64 offset, u8 *status)
    {
    int cc, exception;
    exception = 1;
    asm_inline volatile (
    "	.insn	rsy,0xeb00000000d0,%[req],%[offset],%[data]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [req] "+d" (req), [exc] "+d" (exception)
    : [offset] "d" (offset), [data] "Q" (*data)
    : CC_CLOBBER);
// status = req >> 24 & 0xff;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
#[no_mangle]
pub unsafe extern "C" fn __zpci_store_block(data: *const u64, req: u64, offset: u64) -> c_int {
    int __zpci_store_block(const u64 *data, u64 req, u64 offset)
    {
    let mut retried: bool = false;
    u8 status;
    int cc;
    do {
    cc = __pcistb(data, req, offset, &status);
    if (cc == 2) {
    udelay(ZPCI_INSN_BUSY_DELAY);
    if (!retried) {
    zpci_err_insn_req(0, 'b', cc, status, req, offset);
    retried = true;
    }
    }
    } while (cc == 2);
    if (cc)
    zpci_err_insn_req(0, 'b', cc, status, req, offset);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: retried) -> else {
    else if (retried)
    zpci_err_insn_req(1, 'b', cc, status, req, offset);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(__zpci_store_block);
    static inline int zpci_write_block_fh(volatile void __iomem *dst,
    const void *src, unsigned long len)
    {
    struct zpci_iomap_entry *entry = &zpci_iomap_start[ZPCI_IDX(dst)];
    let mut req: u64 = ZPCI_CREATE_REQ(entry.fh, entry.bar, len);
    let mut offset: u64 = ZPCI_OFFSET(dst);
    return __zpci_store_block(src, req, offset);
    }
#[no_mangle]
pub unsafe extern "C" fn __pcistb_mio(data: *const u64, ioaddr: u64, len: u64, status: *mut u8) -> c_int {
    static inline int __pcistb_mio(const u64 *data, u64 ioaddr, u64 len, u8 *status)
    {
    int cc, exception;
    exception = 1;
    asm_inline volatile (
    "       .insn   rsy,0xeb00000000d4,%[len],%[ioaddr],%[data]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [len] "+d" (len), [exc] "+d" (exception)
    : [ioaddr] "d" (ioaddr), [data] "Q" (*data)
    : CC_CLOBBER);
// status = len >> 24 & 0xff;
    return exception ? -ENXIO : CC_TRANSFORM(cc);
    }
    int zpci_write_block(volatile void __iomem *dst,
    const void *src, unsigned long len)
    {
    u8 status;
    int cc;
    if (!static_branch_unlikely(&have_mio))
    return zpci_write_block_fh(dst, src, len);
    cc = __pcistb_mio(src, ( u64) dst, len, &status);
    if (cc)
    zpci_err_insn_addr(0, 'B', cc, status, ( u64) dst, len);
    return (cc > 0) ? -EIO : cc;
    }
    EXPORT_SYMBOL_GPL(zpci_write_block);
#[no_mangle]
pub unsafe extern "C" fn __pciwb_mio() {
    static inline void __pciwb_mio(void)
    {
    asm volatile (".insn    rre,0xb9d50000,0,0");
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_barrier() {
    void zpci_barrier(void)
    {
    if (static_branch_likely(&have_mio))
    __pciwb_mio();
    }
    EXPORT_SYMBOL_GPL(zpci_barrier);
