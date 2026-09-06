//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/rcar/debugfs.c
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
// // Renesas R-Car debugfs support
//
// Copyright (c) 2021 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// > mount -t debugfs none /sys/kernel/debug
// > cd /sys/kernel/debug/asoc/rcar-sound/ec500000.sound/rdai{N}
// > cat playback/xxx
// > cat capture/xxx
//

#[no_mangle]
unsafe extern "C" fn rsnd_debugfs_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int rsnd_debugfs_show(struct seq_file *m, void *v)
    {
    struct rsnd_dai_stream *io = m.private;
    struct rsnd_mod *mod = rsnd_io_to_mod_ssi(io);
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    int i;
// adg is out of mods
    rsnd_adg_clk_dbg_info(priv, m);
    for_each_rsnd_mod(i, mod, io) {
    u32 *status = mod.ops.get_status(mod, io, mod.type);
    seq_printf(m, "name: %s\n", rsnd_mod_name(mod));
    seq_printf(m, "status: %08x\n", *status);
    if (mod.ops.debug_info)
    mod.ops.debug_info(m, io, mod);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(rsnd_debugfs);
    void rsnd_debugfs_reg_show(struct seq_file *m, phys_addr_t _addr,
    void __iomem *base, int offset, int size)
    {
    int i, j;
    for (i = 0; i < size; i += 0x10) {
    let mut addr: phys_addr_t = _addr + offset + i;
    seq_printf(m, "%pa:", &addr);
    for (j = 0; j < 0x10; j += 0x4)
    seq_printf(m, " %08x", __raw_readl(base + offset + i + j));
    seq_puts(m, "\n");
    }
    }
    void rsnd_debugfs_mod_reg_show(struct seq_file *m, struct rsnd_mod *mod,
    int reg_id, int offset, int size)
    {
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    rsnd_debugfs_reg_show(m,
    rsnd_gen_get_phy_addr(priv, reg_id),
    rsnd_gen_get_base_addr(priv, reg_id),
    offset, size);
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_debugfs_probe(component: *mut snd_soc_component) -> c_int {
    int rsnd_debugfs_probe(struct snd_soc_component *component)
    {
    struct rsnd_priv *priv = dev_get_drvdata(component.dev);
    struct rsnd_dai *rdai;
    struct dentry *dir;
    char name[64];
    int i;
// Gen1 is not supported
    if (rsnd_is_gen1(priv))
    return 0;
    for_each_rsnd_dai(rdai, priv, i) {
//
// created debugfs will be automatically
// removed, nothing to do for _remove.
// see
// soc_cleanup_component_debugfs()
//
    snprintf(name, sizeof(name), "rdai%d", i);
    dir = debugfs_create_dir(name, component.debugfs_root);
    debugfs_create_file("playback", 0444, dir, &rdai.playback, &rsnd_debugfs_fops);
    debugfs_create_file("capture",  0444, dir, &rdai.capture,  &rsnd_debugfs_fops);
    }
    return 0;
    }
