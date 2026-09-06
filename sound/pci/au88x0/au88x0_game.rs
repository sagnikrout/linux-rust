//! Automatically rewritten from C to Rust
//! Source: sound/pci/au88x0/au88x0_game.c
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
// Manuel Jander.
//
// Based on the work of:
// Vojtech Pavlik
// Raymond Ingles
//
// Should you need to contact me, the author, you can do so either by
// e-mail - mail your message to <vojtech@suse.cz>, or by paper mail:
// Vojtech Pavlik, Ucitelska 1576, Prague 8, 182 00 Czech Republic
//
// Based 90% on Vojtech Pavlik pcigame driver.
// Merged and modified by Manuel Jander, for the OpenVortex
// driver. (email: mjander@embedded.cl).
//

#[no_mangle]
unsafe extern "C" fn vortex_game_read(gameport: *mut gameport) -> c_uchar {
    static unsigned char vortex_game_read(struct gameport *gameport)
    {
    vortex_t *vortex = gameport_get_port_data(gameport);
    return hwread(vortex.mmio, VORTEX_GAME_LEGACY);
    }
#[no_mangle]
unsafe extern "C" fn vortex_game_trigger(gameport: *mut gameport) {
    static void vortex_game_trigger(struct gameport *gameport)
    {
    vortex_t *vortex = gameport_get_port_data(gameport);
    hwwrite(vortex.mmio, VORTEX_GAME_LEGACY, 0xff);
    }
    static int
    vortex_game_cooked_read(struct gameport *gameport, int *axes, int *buttons)
    {
    vortex_t *vortex = gameport_get_port_data(gameport);
    int i;
// buttons = (~hwread(vortex->mmio, VORTEX_GAME_LEGACY) >> 4) & 0xf;
    for (i = 0; i < 4; i++) {
    axes[i] =
    hwread(vortex.mmio, VORTEX_GAME_AXIS + (i * AXIS_SIZE));
    if (axes[i] == AXIS_RANGE)
    axes[i] = -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vortex_game_open(gameport: *mut gameport, mode: c_int) -> c_int {
    static int vortex_game_open(struct gameport *gameport, int mode)
    {
    vortex_t *vortex = gameport_get_port_data(gameport);
    switch (mode) {
    case GAMEPORT_MODE_COOKED:
    hwwrite(vortex.mmio, VORTEX_CTRL2,
    hwread(vortex.mmio,
    VORTEX_CTRL2) | CTRL2_GAME_ADCMODE);
    msleep(VORTEX_GAME_DWAIT);
    return 0;
    case GAMEPORT_MODE_RAW:
    hwwrite(vortex.mmio, VORTEX_CTRL2,
    hwread(vortex.mmio,
    VORTEX_CTRL2) & ~CTRL2_GAME_ADCMODE);
    return 0;
    default:
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vortex_gameport_register(vortex: *mut vortex_t) -> c_int {
    static int vortex_gameport_register(vortex_t *vortex)
    {
    struct gameport *gp;
    vortex.gameport = gp = gameport_allocate_port();
    if (!gp) {
    dev_err(vortex.card.dev,
    "cannot allocate memory for gameport\n");
    return -ENOMEM;
    }
    gameport_set_name(gp, "AU88x0 Gameport");
    gameport_set_phys(gp, "pci%s/gameport0", pci_name(vortex.pci_dev));
    gameport_set_dev_parent(gp, &vortex.pci_dev.dev);
    gp.read = vortex_game_read;
    gp.trigger = vortex_game_trigger;
    gp.cooked_read = vortex_game_cooked_read;
    gp.open = vortex_game_open;
    gameport_set_port_data(gp, vortex);
    gp.fuzz = 64;
    gameport_register_port(gp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vortex_gameport_unregister(vortex: *mut *mut vortex_t) {
    static void vortex_gameport_unregister(vortex_t * vortex)
    {
    if (vortex.gameport) {
    gameport_unregister_port(vortex.gameport);
    vortex.gameport = core::ptr::null_mut();
    }
    }

    static inline int vortex_gameport_register(vortex_t * vortex) { return -ENOSYS; }
    static inline void vortex_gameport_unregister(vortex_t * vortex) { }
