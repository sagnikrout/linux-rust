//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/mv88e6xxx/pcs-6185.c
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
// Marvell 88E6185 family SERDES PCS support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2017 Andrew Lunn <andrew@lunn.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv88e6185_pcs {
    pub phylink_pcs: phylink_pcs,
    pub irq: c_uint,
    pub name: [c_char; 64],
    pub chip: *mut mv88e6xxx_chip,
    pub port: c_int,
}

    static struct mv88e6185_pcs *pcs_to_mv88e6185_pcs(struct phylink_pcs *pcs)
    {
    return container_of(pcs, struct mv88e6185_pcs, phylink_pcs);
    }
#[no_mangle]
unsafe extern "C" fn mv88e6185_pcs_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mv88e6185_pcs_handle_irq(int irq, void *dev_id)
    {
    struct mv88e6185_pcs *mpcs = dev_id;
    struct mv88e6xxx_chip *chip;
    let mut ret: irqreturn_t = IRQ_NONE;
    bool link_up;
    u16 status;
    int port;
    int err;
    chip = mpcs.chip;
    port = mpcs.port;
    mv88e6xxx_reg_lock(chip);
    err = mv88e6xxx_port_read(chip, port, MV88E6XXX_PORT_STS, &status);
    mv88e6xxx_reg_unlock(chip);
    if (!err) {
    link_up = !!(status & MV88E6XXX_PORT_STS_LINK);
    phylink_pcs_change(&mpcs.phylink_pcs, link_up);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
    static void mv88e6185_pcs_get_state(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    struct phylink_link_state *state)
    {
    struct mv88e6185_pcs *mpcs = pcs_to_mv88e6185_pcs(pcs);
    struct mv88e6xxx_chip *chip = mpcs.chip;
    let mut port: c_int = mpcs.port;
    u16 status;
    int err;
    mv88e6xxx_reg_lock(chip);
    err = mv88e6xxx_port_read(chip, port, MV88E6XXX_PORT_STS, &status);
    mv88e6xxx_reg_unlock(chip);
    if (err)
    status = 0;
    state.link = !!(status & MV88E6XXX_PORT_STS_LINK);
    if (state.link) {
    state.duplex = status & MV88E6XXX_PORT_STS_DUPLEX ?
    DUPLEX_FULL : DUPLEX_HALF;
    switch (status & MV88E6XXX_PORT_STS_SPEED_MASK) {
    case MV88E6XXX_PORT_STS_SPEED_1000:
    state.speed = SPEED_1000;
    break;
    case MV88E6XXX_PORT_STS_SPEED_100:
    state.speed = SPEED_100;
    break;
    case MV88E6XXX_PORT_STS_SPEED_10:
    state.speed = SPEED_10;
    break;
    default:
    state.link = false;
    break;
    }
    }
    }
    static int mv88e6185_pcs_config(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t interface,
    const unsigned long *advertising,
    bool permit_pause_to_mac)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv88e6185_pcs_an_restart(pcs: *mut phylink_pcs) {
    static void mv88e6185_pcs_an_restart(struct phylink_pcs *pcs)
    {
    }
    static const struct phylink_pcs_ops mv88e6185_phylink_pcs_ops = {
    .pcs_get_state = mv88e6185_pcs_get_state,
    .pcs_config = mv88e6185_pcs_config,
    .pcs_an_restart = mv88e6185_pcs_an_restart,
    };
#[no_mangle]
unsafe extern "C" fn mv88e6185_pcs_init(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int {
    static int mv88e6185_pcs_init(struct mv88e6xxx_chip *chip, int port)
    {
    struct mv88e6185_pcs *mpcs;
    struct device *dev;
    unsigned int irq;
    int err;
// There are no configurable serdes lanes on this switch chip, so
// we use the static cmode configuration to determine whether we
// have a PCS or not.
//
    if (chip.ports[port].cmode != MV88E6185_PORT_STS_CMODE_SERDES &&
    chip.ports[port].cmode != MV88E6185_PORT_STS_CMODE_1000BASE_X)
    return 0;
    dev = chip.dev;
    mpcs = kzalloc_obj(*mpcs);
    if (!mpcs)
    return -ENOMEM;
    mpcs.chip = chip;
    mpcs.port = port;
    mpcs.phylink_pcs.ops = &mv88e6185_phylink_pcs_ops;
    irq = mv88e6xxx_serdes_irq_mapping(chip, port);
    if (irq) {
    snprintf(mpcs.name, sizeof(mpcs.name),
    "mv88e6xxx-%s-serdes-%d", dev_name(dev), port);
    err = request_threaded_irq(irq, core::ptr::null_mut(), mv88e6185_pcs_handle_irq,
    IRQF_ONESHOT, mpcs.name, mpcs);
    if (err) {
    kfree(mpcs);
    return err;
    }
    mpcs.irq = irq;
    } else {
    mpcs.phylink_pcs.poll = true;
    }
    chip.ports[port].pcs_private = &mpcs.phylink_pcs;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv88e6185_pcs_teardown(chip: *mut mv88e6xxx_chip, port: c_int) {
    static void mv88e6185_pcs_teardown(struct mv88e6xxx_chip *chip, int port)
    {
    struct mv88e6185_pcs *mpcs;
    mpcs = chip.ports[port].pcs_private;
    if (!mpcs)
    return;
    if (mpcs.irq)
    free_irq(mpcs.irq, mpcs);
    kfree(mpcs);
    chip.ports[port].pcs_private = core::ptr::null_mut();
    }
    static struct phylink_pcs *mv88e6185_pcs_select(struct mv88e6xxx_chip *chip,
    int port,
    phy_interface_t interface)
    {
    return chip.ports[port].pcs_private;
    }
    const struct mv88e6xxx_pcs_ops mv88e6185_pcs_ops = {
    .pcs_init = mv88e6185_pcs_init,
    .pcs_teardown = mv88e6185_pcs_teardown,
    .pcs_select = mv88e6185_pcs_select,
    };
