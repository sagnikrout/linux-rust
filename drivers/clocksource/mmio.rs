//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/mmio.c
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
// Generic MMIO clocksource support
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clocksource_mmio {
    pub reg: *mut void __iomem,
    pub clksrc: clocksource,
}

    static inline struct clocksource_mmio *to_mmio_clksrc(struct clocksource *c)
    {
    return container_of(c, struct clocksource_mmio, clksrc);
    }
#[no_mangle]
pub unsafe extern "C" fn clocksource_mmio_readl_up(c: *mut clocksource) -> u64 {
    u64 clocksource_mmio_readl_up(struct clocksource *c)
    {
    return (u64)readl_relaxed(to_mmio_clksrc(c).reg);
    }
    EXPORT_SYMBOL_GPL(clocksource_mmio_readl_up);
#[no_mangle]
pub unsafe extern "C" fn clocksource_mmio_readl_down(c: *mut clocksource) -> u64 {
    u64 clocksource_mmio_readl_down(struct clocksource *c)
    {
    return ~(u64)readl_relaxed(to_mmio_clksrc(c).reg) & c.mask;
    }
    EXPORT_SYMBOL_GPL(clocksource_mmio_readl_down);
#[no_mangle]
pub unsafe extern "C" fn clocksource_mmio_readw_up(c: *mut clocksource) -> u64 {
    u64 clocksource_mmio_readw_up(struct clocksource *c)
    {
    return (u64)readw_relaxed(to_mmio_clksrc(c).reg);
    }
    EXPORT_SYMBOL_GPL(clocksource_mmio_readw_up);
#[no_mangle]
pub unsafe extern "C" fn clocksource_mmio_readw_down(c: *mut clocksource) -> u64 {
    u64 clocksource_mmio_readw_down(struct clocksource *c)
    {
    return ~(u64)readw_relaxed(to_mmio_clksrc(c).reg) & c.mask;
    }
    EXPORT_SYMBOL_GPL(clocksource_mmio_readw_down);
//
// clocksource_mmio_init - Initialize a simple mmio based clocksource
// @base:	Virtual address of the clock readout register
// @name:	Name of the clocksource
// @hz:		Frequency of the clocksource in Hz
// @rating:	Rating of the clocksource
// @bits:	Number of valid bits
// @read:	One of clocksource_mmio_read*() above
//
    int clocksource_mmio_init(void __iomem *base, const char *name,
    unsigned long hz, int rating, unsigned bits,
    u64 (*read)(struct clocksource *))
    {
    struct clocksource_mmio *cs;
    if (bits > 64 || bits < 16)
    return -EINVAL;
    cs = kzalloc_obj(struct clocksource_mmio);
    if (!cs)
    return -ENOMEM;
    cs.reg = base;
    cs.clksrc.name = name;
    cs.clksrc.rating = rating;
    cs.clksrc.read = read;
    cs.clksrc.mask = CLOCKSOURCE_MASK(bits);
    cs.clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    return clocksource_register_hz(&cs.clksrc, hz);
    }
    EXPORT_SYMBOL_GPL(clocksource_mmio_init);
