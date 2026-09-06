//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/pl111/pl111_debugfs.c
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
// Copyright © 2017 Broadcom
//

    static const struct {
    u32 reg;
    const char *name;
    } pl111_reg_defs[] = {
    REGDEF(CLCD_TIM0),
    REGDEF(CLCD_TIM1),
    REGDEF(CLCD_TIM2),
    REGDEF(CLCD_TIM3),
    REGDEF(CLCD_UBAS),
    REGDEF(CLCD_LBAS),
    REGDEF(CLCD_PL111_CNTL),
    REGDEF(CLCD_PL111_IENB),
    REGDEF(CLCD_PL111_RIS),
    REGDEF(CLCD_PL111_MIS),
    REGDEF(CLCD_PL111_ICR),
    REGDEF(CLCD_PL111_UCUR),
    REGDEF(CLCD_PL111_LCUR),
    };
#[no_mangle]
unsafe extern "C" fn pl111_debugfs_regs(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int pl111_debugfs_regs(struct seq_file *m, void *unused)
    {
    struct drm_info_node *node = (struct drm_info_node *)m.private;
    struct drm_device *dev = node.minor.dev;
    struct pl111_drm_dev_private *priv = dev.dev_private;
    int i;
    for (i = 0; i < ARRAY_SIZE(pl111_reg_defs); i++) {
    seq_printf(m, "%s (0x%04x): 0x%08x\n",
    pl111_reg_defs[i].name, pl111_reg_defs[i].reg,
    readl(priv.regs + pl111_reg_defs[i].reg));
    }
    return 0;
    }
    static const struct drm_info_list pl111_debugfs_list[] = {
    {"regs", pl111_debugfs_regs, 0},
    };
    void
    pl111_debugfs_init(struct drm_minor *minor)
    {
    drm_debugfs_create_files(pl111_debugfs_list,
    ARRAY_SIZE(pl111_debugfs_list),
    minor.debugfs_root, minor);
    }
