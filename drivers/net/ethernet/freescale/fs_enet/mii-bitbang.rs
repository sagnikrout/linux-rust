//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/fs_enet/mii-bitbang.c
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
// Combined Ethernet driver for Motorola MPC8xx and MPC82xx.
//
// Copyright (c) 2003 Intracom S.A.
// by Pantelis Antoniou <panto@intracom.gr>
//
// 2005 (c) MontaVista Software, Inc.
// Vitaly Bordug <vbordug@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bb_info {
    pub ctrl: mdiobb_ctrl,
    pub dir: *mut u32 __iomem,
    pub dat: *mut u32 __iomem,
    pub mdio_msk: u32,
    pub mdc_msk: u32,
}

// FIXME: If any other users of GPIO crop up, then these will have to
// have some sort of global synchronization to avoid races with other
// pins on the same port.  The ideal solution would probably be to
// bind the ports to a GPIO driver, and have this be a client of it.
//
#[no_mangle]
pub unsafe extern "C" fn bb_set(p: *mut u32 __iomem, m: u32) {
    static inline void bb_set(u32 __iomem *p, u32 m)
    {
    out_be32(p, in_be32(p) | m);
    }
#[no_mangle]
pub unsafe extern "C" fn bb_clr(p: *mut u32 __iomem, m: u32) {
    static inline void bb_clr(u32 __iomem *p, u32 m)
    {
    out_be32(p, in_be32(p) & ~m);
    }
#[no_mangle]
pub unsafe extern "C" fn bb_read(p: *mut u32 __iomem, m: u32) -> c_int {
    static inline int bb_read(u32 __iomem *p, u32 m)
    {
    return (in_be32(p) & m) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_dir(ctrl: *mut mdiobb_ctrl, dir: c_int) {
    static inline void mdio_dir(struct mdiobb_ctrl *ctrl, int dir)
    {
    struct bb_info *bitbang = container_of(ctrl, struct bb_info, ctrl);
    if (dir)
    bb_set(bitbang.dir, bitbang.mdio_msk);
    else
    bb_clr(bitbang.dir, bitbang.mdio_msk);
// Read back to flush the write.
    in_be32(bitbang.dir);
    }
#[no_mangle]
pub unsafe extern "C" fn mdio_read(ctrl: *mut mdiobb_ctrl) -> c_int {
    static inline int mdio_read(struct mdiobb_ctrl *ctrl)
    {
    struct bb_info *bitbang = container_of(ctrl, struct bb_info, ctrl);
    return bb_read(bitbang.dat, bitbang.mdio_msk);
    }
#[no_mangle]
pub unsafe extern "C" fn mdio(ctrl: *mut mdiobb_ctrl, what: c_int) {
    static inline void mdio(struct mdiobb_ctrl *ctrl, int what)
    {
    struct bb_info *bitbang = container_of(ctrl, struct bb_info, ctrl);
    if (what)
    bb_set(bitbang.dat, bitbang.mdio_msk);
    else
    bb_clr(bitbang.dat, bitbang.mdio_msk);
// Read back to flush the write.
    in_be32(bitbang.dat);
    }
#[no_mangle]
pub unsafe extern "C" fn mdc(ctrl: *mut mdiobb_ctrl, what: c_int) {
    static inline void mdc(struct mdiobb_ctrl *ctrl, int what)
    {
    struct bb_info *bitbang = container_of(ctrl, struct bb_info, ctrl);
    if (what)
    bb_set(bitbang.dat, bitbang.mdc_msk);
    else
    bb_clr(bitbang.dat, bitbang.mdc_msk);
// Read back to flush the write.
    in_be32(bitbang.dat);
    }
    static const struct mdiobb_ops bb_ops = {
    .owner = THIS_MODULE,
    .set_mdc = mdc,
    .set_mdio_dir = mdio_dir,
    .set_mdio_data = mdio,
    .get_mdio_data = mdio_read,
    };
#[no_mangle]
unsafe extern "C" fn fs_mii_bitbang_init(bus: *mut mii_bus, np: *mut device_node) -> c_int {
    static int fs_mii_bitbang_init(struct mii_bus *bus, struct device_node *np)
    {
    struct resource res;
    const u32 *data;
    int mdio_pin, mdc_pin, len;
    struct bb_info *bitbang = bus.priv;
    let mut ret: c_int = of_address_to_resource(np, 0, &res);
    if (ret)
    return ret;
    if (resource_size(&res) <= 13)
    return -ENODEV;
// This should really encode the pin number as well, but all
// we get is an int, and the odds of multiple bitbang mdio buses
// is low enough that it's not worth going too crazy.
//
    snprintf(bus.id, MII_BUS_ID_SIZE, "%pa", &res.start);
    data = of_get_property(np, "fsl,mdio-pin", &len);
    if (!data || len != 4)
    return -ENODEV;
    mdio_pin = *data;
    data = of_get_property(np, "fsl,mdc-pin", &len);
    if (!data || len != 4)
    return -ENODEV;
    mdc_pin = *data;
    bitbang.dir = ioremap(res.start, resource_size(&res));
    if (!bitbang.dir)
    return -ENOMEM;
    bitbang.dat = bitbang.dir + 4;
    bitbang.mdio_msk = 1 << (31 - mdio_pin);
    bitbang.mdc_msk = 1 << (31 - mdc_pin);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fs_enet_mdio_probe(ofdev: *mut platform_device) -> c_int {
    static int fs_enet_mdio_probe(struct platform_device *ofdev)
    {
    struct mii_bus *new_bus;
    struct bb_info *bitbang;
    let mut ret: c_int = -ENOMEM;
    bitbang = kzalloc_obj(struct bb_info);
    if (!bitbang)
    goto out;
    bitbang.ctrl.ops = &bb_ops;
    new_bus = alloc_mdio_bitbang(&bitbang.ctrl);
    if (!new_bus)
    goto out_free_priv;
    new_bus.name = "CPM2 Bitbanged MII",
    ret = fs_mii_bitbang_init(new_bus, ofdev.dev.of_node);
    if (ret)
    goto out_free_bus;
    new_bus.phy_mask = ~0;
    new_bus.parent = &ofdev.dev;
    platform_set_drvdata(ofdev, new_bus);
    ret = of_mdiobus_register(new_bus, ofdev.dev.of_node);
    if (ret)
    goto out_unmap_regs;
    return 0;
    out_unmap_regs:
    iounmap(bitbang.dir);
    out_free_bus:
    free_mdio_bitbang(new_bus);
    out_free_priv:
    kfree(bitbang);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fs_enet_mdio_remove(ofdev: *mut platform_device) {
    static void fs_enet_mdio_remove(struct platform_device *ofdev)
    {
    struct mii_bus *bus = platform_get_drvdata(ofdev);
    struct bb_info *bitbang = bus.priv;
    mdiobus_unregister(bus);
    free_mdio_bitbang(bus);
    iounmap(bitbang.dir);
    kfree(bitbang);
    }
    static const struct of_device_id fs_enet_mdio_bb_match[] = {
    {
    .compatible = "fsl,cpm2-mdio-bitbang",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, fs_enet_mdio_bb_match);
    static struct platform_driver fs_enet_bb_mdio_driver = {
    .driver = {
    .name = "fsl-bb-mdio",
    .of_match_table = fs_enet_mdio_bb_match,
    },
    .probe = fs_enet_mdio_probe,
    .remove = fs_enet_mdio_remove,
    };
    module_platform_driver(fs_enet_bb_mdio_driver);
    MODULE_LICENSE("GPL");
