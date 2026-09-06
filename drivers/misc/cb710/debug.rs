//! Automatically rewritten from C to Rust
//! Source: drivers/misc/cb710/debug.c
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
// cb710/debug.c
//
// Copyright by Michał Mirosław, 2008-2009
//

pub const CB710_REG_COUNT: c_uint = 0x80;
    static const u16 allow[CB710_REG_COUNT/16] = {
    0xFFF0, 0xFFFF, 0xFFFF, 0xFFFF,
    0xFFF0, 0xFFFF, 0xFFFF, 0xFFFF,
    };
    static const char *const prefix[ARRAY_SIZE(allow)] = {
    "MMC", "MMC", "MMC", "MMC",
    "MS?", "MS?", "SM?", "SM?"
    };
#[no_mangle]
pub unsafe extern "C" fn allow_reg_read(block: unsigned, offset: unsigned, bits: unsigned) -> c_int {
    static inline int allow_reg_read(unsigned block, unsigned offset, unsigned bits)
    {
    let mut mask: unsigned = (1 << bits/8) - 1;
    offset *= bits/8;
    return ((allow[block] >> offset) & mask) == mask;
    }

    static void cb710_read_regs_##t(void __iomem *iobase,			\
    u##t *reg, unsigned select)					\
    {									\
    unsigned i, j;							\
    \
    for (i = 0; i < ARRAY_SIZE(allow); ++i, reg += 16/(t/8)) {	\
    if (!(select & (1 << i)))					\
    continue;					\
    \
    for (j = 0; j < 0x10/(t/8); ++j) {			\
    if (!allow_reg_read(i, j, t))			\
    continue;				\
    reg[j] = ioread##t(iobase			\
    + (i << 4) + (j * (t/8)));		\
    }							\
    }								\
    }
    static const char cb710_regf_8[] = "%02X";
    static const char cb710_regf_16[] = "%04X";
    static const char cb710_regf_32[] = "%08X";
    static const char cb710_xes[] = "xxxxxxxx";

    static void cb710_dump_regs_##t(struct device *dev,			\
    const u##t *reg, unsigned select)				\
    {									\
    const char *const xp = &cb710_xes[8 - t/4];			\
    const char *const format = cb710_regf_##t;			\
    \
    char msg[100], *p;						\
    unsigned i, j;							\
    \
    for (i = 0; i < ARRAY_SIZE(allow); ++i, reg += 16/(t/8)) {	\
    if (!(select & (1 << i)))				\
    continue;					\
    p = msg;						\
    for (j = 0; j < 0x10/(t/8); ++j) {			\
// p++ = ' ';					\
    if (j == 8/(t/8))				\
// p++ = ' ';				\
    if (allow_reg_read(i, j, t))			\
    p += sprintf(p, format, reg[j]);	\
    else						\
    p += sprintf(p, "%s", xp);		\
    }							\
    dev_dbg(dev, "%s 0x%02X %s\n", prefix[i], i << 4, msg);	\
    }								\
    }

    static void cb710_read_and_dump_regs_##t(struct cb710_chip *chip,	\
    unsigned select)						\
    {									\
    u##t regs[CB710_REG_COUNT/sizeof(u##t)];			\
    \
    memset(&regs, 0, sizeof(regs));					\
    cb710_read_regs_##t(chip.iobase, regs, select);		\
    cb710_dump_regs_##t(cb710_chip_dev(chip), regs, select);	\
    }

    CB710_READ_REGS_TEMPLATE(t)			\
    CB710_DUMP_REGS_TEMPLATE(t)			\
    CB710_READ_AND_DUMP_REGS_TEMPLATE(t)
    CB710_REG_ACCESS_TEMPLATES(8)
    CB710_REG_ACCESS_TEMPLATES(16)
    CB710_REG_ACCESS_TEMPLATES(32)
#[no_mangle]
pub unsafe extern "C" fn cb710_dump_regs(chip: *mut cb710_chip, select: unsigned) {
    void cb710_dump_regs(struct cb710_chip *chip, unsigned select)
    {
    if (!(select & CB710_DUMP_REGS_MASK))
    select = CB710_DUMP_REGS_ALL;
    if (!(select & CB710_DUMP_ACCESS_MASK))
    select |= CB710_DUMP_ACCESS_8;
    if (select & CB710_DUMP_ACCESS_32)
    cb710_read_and_dump_regs_32(chip, select);
    if (select & CB710_DUMP_ACCESS_16)
    cb710_read_and_dump_regs_16(chip, select);
    if (select & CB710_DUMP_ACCESS_8)
    cb710_read_and_dump_regs_8(chip, select);
    }
    EXPORT_SYMBOL_GPL(cb710_dump_regs);
