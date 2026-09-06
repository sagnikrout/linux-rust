//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/tw5864/tw5864-util.c
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
pub unsafe extern "C" fn tw5864_indir_writeb(dev: *mut tw5864_dev, addr: u16, data: u8) {
    void tw5864_indir_writeb(struct tw5864_dev *dev, u16 addr, u8 data)
    {
    let mut retries: c_int = 30000;
    while (tw_readl(TW5864_IND_CTL) & BIT(31) && --retries)
    ;
    if (!retries)
    dev_err(&dev.pci.dev,
    "tw_indir_writel() retries exhausted before writing\n");
    tw_writel(TW5864_IND_DATA, data);
    tw_writel(TW5864_IND_CTL, addr << 2 | TW5864_RW | TW5864_ENABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn tw5864_indir_readb(dev: *mut tw5864_dev, addr: u16) -> u8 {
    u8 tw5864_indir_readb(struct tw5864_dev *dev, u16 addr)
    {
    let mut retries: c_int = 30000;
    while (tw_readl(TW5864_IND_CTL) & BIT(31) && --retries)
    ;
    if (!retries)
    dev_err(&dev.pci.dev,
    "tw_indir_readl() retries exhausted before reading\n");
    tw_writel(TW5864_IND_CTL, addr << 2 | TW5864_ENABLE);
    retries = 30000;
    while (tw_readl(TW5864_IND_CTL) & BIT(31) && --retries)
    ;
    if (!retries)
    dev_err(&dev.pci.dev,
    "tw_indir_readl() retries exhausted at reading\n");
    return tw_readl(TW5864_IND_DATA);
    }
