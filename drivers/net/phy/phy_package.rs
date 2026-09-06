//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/phy_package.c
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
// PHY package support
//

//
// struct phy_package_shared - Shared information in PHY packages
// @base_addr: Base PHY address of PHY package used to combine PHYs
// in one package and for offset calculation of phy_package_read/write
// @np: Pointer to the Device Node if PHY package defined in DT
// @refcnt: Number of PHYs connected to this shared data
// @flags: Initialization of PHY package
// @priv_size: Size of the shared private data @priv
// @priv: Driver private data shared across a PHY package
//
// Represents a shared structure between different phydev's in the same
// package, for example a quad PHY. See phy_package_join() and
// phy_package_leave().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_package_shared {
    pub base_addr: u8,
// With PHY package defined in DT this points to the PHY package node
    pub np: *mut device_node,
    pub refcnt: refcount_t,
    pub flags: c_ulong,
    pub priv_size: usize,
// private data pointer
// note that this pointer is shared between different phydevs and
// the user has to take care of appropriate locking. It is allocated
// and freed automatically by phy_package_join() and
// phy_package_leave().
//
    pub priv: *mut c_void,
}

    struct device_node *phy_package_get_node(struct phy_device *phydev)
    {
    return phydev.shared.np;
    }
    EXPORT_SYMBOL_GPL(phy_package_get_node);
    void *phy_package_get_priv(struct phy_device *phydev)
    {
    return phydev.shared.priv;
    }
    EXPORT_SYMBOL_GPL(phy_package_get_priv);
//
// phy_package_lock - acquire the PHY package lock
// @phydev: PHY device that has joined the package
//
// Use this to serialize access to package-private data. Release the lock
// with phy_package_unlock().
//
#[no_mangle]
pub unsafe extern "C" fn phy_package_lock(phydev: *mut phy_device) {
    void phy_package_lock(struct phy_device *phydev)
    {
    mutex_lock(&phydev.mdio.bus.shared_lock);
    }
    EXPORT_SYMBOL_GPL(phy_package_lock);
//
// phy_package_unlock - release the PHY package lock
// @phydev: PHY device that has joined the package
//
#[no_mangle]
pub unsafe extern "C" fn phy_package_unlock(phydev: *mut phy_device) {
    void phy_package_unlock(struct phy_device *phydev)
    {
    mutex_unlock(&phydev.mdio.bus.shared_lock);
    }
    EXPORT_SYMBOL_GPL(phy_package_unlock);
    static int phy_package_address(struct phy_device *phydev,
    unsigned int addr_offset)
    {
    struct phy_package_shared *shared = phydev.shared;
    let mut base_addr: u8 = shared.base_addr;
    if (addr_offset >= PHY_MAX_ADDR - base_addr)
    return -EIO;
// we know that addr will be in the range 0..31 and thus the
// implicit cast to a signed int is not a problem.
//
    return base_addr + addr_offset;
    }
    int __phy_package_read(struct phy_device *phydev, unsigned int addr_offset,
    u32 regnum)
    {
    let mut addr: c_int = phy_package_address(phydev, addr_offset);
    if (addr < 0)
    return addr;
    return __mdiobus_read(phydev.mdio.bus, addr, regnum);
    }
    EXPORT_SYMBOL_GPL(__phy_package_read);
    int __phy_package_write(struct phy_device *phydev, unsigned int addr_offset,
    u32 regnum, u16 val)
    {
    let mut addr: c_int = phy_package_address(phydev, addr_offset);
    if (addr < 0)
    return addr;
    return __mdiobus_write(phydev.mdio.bus, addr, regnum, val);
    }
    EXPORT_SYMBOL_GPL(__phy_package_write);
//
// __phy_package_read_mmd - read MMD reg relative to PHY package base addr
// @phydev: The phy_device struct
// @addr_offset: The offset to be added to PHY package base_addr
// @devad: The MMD to read from
// @regnum: The register on the MMD to read
//
// Convenience helper for reading a register of an MMD on a given PHY
// using the PHY package base address. The base address is added to
// the addr_offset value.
//
// Same calling rules as for __phy_read();
//
// NOTE: It's assumed that the entire PHY package is either C22 or C45.
//
    int __phy_package_read_mmd(struct phy_device *phydev,
    unsigned int addr_offset, int devad,
    u32 regnum)
    {
    let mut addr: c_int = phy_package_address(phydev, addr_offset);
    if (addr < 0)
    return addr;
    if (regnum > (u16)~0 || devad > 32)
    return -EINVAL;
    return mmd_phy_read(phydev.mdio.bus, addr, phydev.is_c45, devad,
    regnum);
    }
    EXPORT_SYMBOL(__phy_package_read_mmd);
//
// __phy_package_write_mmd - write MMD reg relative to PHY package base addr
// @phydev: The phy_device struct
// @addr_offset: The offset to be added to PHY package base_addr
// @devad: The MMD to write to
// @regnum: The register on the MMD to write
// @val: value to write to @regnum
//
// Convenience helper for writing a register of an MMD on a given PHY
// using the PHY package base address. The base address is added to
// the addr_offset value.
//
// Same calling rules as for __phy_write();
//
// NOTE: It's assumed that the entire PHY package is either C22 or C45.
//
    int __phy_package_write_mmd(struct phy_device *phydev,
    unsigned int addr_offset, int devad,
    u32 regnum, u16 val)
    {
    let mut addr: c_int = phy_package_address(phydev, addr_offset);
    if (addr < 0)
    return addr;
    if (regnum > (u16)~0 || devad > 32)
    return -EINVAL;
    return mmd_phy_write(phydev.mdio.bus, addr, phydev.is_c45, devad,
    regnum, val);
    }
    EXPORT_SYMBOL(__phy_package_write_mmd);
#[no_mangle]
unsafe extern "C" fn __phy_package_set_once(phydev: *mut phy_device, b: c_uint) -> bool {
    static bool __phy_package_set_once(struct phy_device *phydev, unsigned int b)
    {
    struct phy_package_shared *shared = phydev.shared;
    if (!shared)
    return false;
    return !test_and_set_bit(b, &shared.flags);
    }
#[no_mangle]
pub unsafe extern "C" fn phy_package_init_once(phydev: *mut phy_device) -> bool {
    bool phy_package_init_once(struct phy_device *phydev)
    {
    return __phy_package_set_once(phydev, 0);
    }
    EXPORT_SYMBOL_GPL(phy_package_init_once);
#[no_mangle]
pub unsafe extern "C" fn phy_package_probe_once(phydev: *mut phy_device) -> bool {
    bool phy_package_probe_once(struct phy_device *phydev)
    {
    return __phy_package_set_once(phydev, 1);
    }
    EXPORT_SYMBOL_GPL(phy_package_probe_once);
//
// phy_package_join - join a common PHY group
// @phydev: target phy_device struct
// @base_addr: cookie and base PHY address of PHY package for offset
// calculation of global register access
// @priv_size: if non-zero allocate this amount of bytes for private data
//
// This joins a PHY group and provides a shared storage for all phydevs in
// this group. This is intended to be used for packages which contain
// more than one PHY, for example a quad PHY transceiver.
//
// The base_addr parameter serves as cookie which has to have the same values
// for all members of one group and as the base PHY address of the PHY package
// for offset calculation to access generic registers of a PHY package.
// Usually, one of the PHY addresses of the different PHYs in the package
// provides access to these global registers.
// The address which is given here, will be used in the __phy_package_read()
// and __phy_package_write() convenience functions as base and added to the
// passed offset in those functions.
//
// This will set the shared pointer of the phydev to the shared storage.
// If this is the first call for a this cookie the shared storage will be
// allocated. If priv_size is non-zero, the given amount of bytes are
// allocated for the priv member.
//
// Returns < 1 on error, 0 on success. Esp. calling phy_package_join()
// with the same cookie but a different priv_size is an error.
//
#[no_mangle]
pub unsafe extern "C" fn phy_package_join(phydev: *mut phy_device, base_addr: c_int, priv_size: usize) -> c_int {
    int phy_package_join(struct phy_device *phydev, int base_addr, size_t priv_size)
    {
    struct mii_bus *bus = phydev.mdio.bus;
    struct phy_package_shared *shared;
    int ret;
    if (base_addr < 0 || base_addr >= PHY_MAX_ADDR)
    return -EINVAL;
    mutex_lock(&bus.shared_lock);
    shared = bus.shared[base_addr];
    if (!shared) {
    ret = -ENOMEM;
    shared = kzalloc_obj(*shared);
    if (!shared)
    goto err_unlock;
    if (priv_size) {
    shared.priv = kzalloc(priv_size, GFP_KERNEL);
    if (!shared.priv)
    goto err_free;
    shared.priv_size = priv_size;
    }
    shared.base_addr = base_addr;
    shared.np = core::ptr::null_mut();
    refcount_set(&shared.refcnt, 1);
    bus.shared[base_addr] = shared;
    } else {
    ret = -EINVAL;
    if (priv_size && priv_size != shared.priv_size)
    goto err_unlock;
    refcount_inc(&shared.refcnt);
    }
    mutex_unlock(&bus.shared_lock);
    phydev.shared = shared;
    return 0;
    err_free:
    kfree(shared);
    err_unlock:
    mutex_unlock(&bus.shared_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(phy_package_join);
//
// of_phy_package_join - join a common PHY group in PHY package
// @phydev: target phy_device struct
// @priv_size: if non-zero allocate this amount of bytes for private data
//
// This is a variant of phy_package_join for PHY package defined in DT.
//
// The parent node of the @phydev is checked as a valid PHY package node
// structure (by matching the node name "ethernet-phy-package") and the
// base_addr for the PHY package is passed to phy_package_join.
//
// With this configuration the shared struct will also have the np value
// filled to use additional DT defined properties in PHY specific
// probe_once and config_init_once PHY package OPs.
//
// Returns < 0 on error, 0 on success. Esp. calling phy_package_join()
// with the same cookie but a different priv_size is an error. Or a parent
// node is not detected or is not valid or doesn't match the expected node
// name for PHY package.
//
#[no_mangle]
pub unsafe extern "C" fn of_phy_package_join(phydev: *mut phy_device, priv_size: usize) -> c_int {
    int of_phy_package_join(struct phy_device *phydev, size_t priv_size)
    {
    struct device_node *node = phydev.mdio.dev.of_node;
    struct device_node *package_node;
    u32 base_addr;
    int ret;
    if (!node)
    return -EINVAL;
    package_node = of_get_parent(node);
    if (!package_node)
    return -EINVAL;
    if (!of_node_name_eq(package_node, "ethernet-phy-package")) {
    ret = -EINVAL;
    goto exit;
    }
    if (of_property_read_u32(package_node, "reg", &base_addr)) {
    ret = -EINVAL;
    goto exit;
    }
    ret = phy_package_join(phydev, base_addr, priv_size);
    if (ret)
    goto exit;
    phydev.shared.np = package_node;
    return 0;
    exit:
    of_node_put(package_node);
    return ret;
    }
    EXPORT_SYMBOL_GPL(of_phy_package_join);
//
// phy_package_leave - leave a common PHY group
// @phydev: target phy_device struct
//
// This leaves a PHY group created by phy_package_join(). If this phydev
// was the last user of the shared data between the group, this data is
// freed. Resets the phydev->shared pointer to NULL.
//
#[no_mangle]
pub unsafe extern "C" fn phy_package_leave(phydev: *mut phy_device) {
    void phy_package_leave(struct phy_device *phydev)
    {
    struct phy_package_shared *shared = phydev.shared;
    struct mii_bus *bus = phydev.mdio.bus;
    if (!shared)
    return;
// Decrease the node refcount on leave if present
    if (shared.np)
    of_node_put(shared.np);
    if (refcount_dec_and_mutex_lock(&shared.refcnt, &bus.shared_lock)) {
    bus.shared[shared.base_addr] = core::ptr::null_mut();
    mutex_unlock(&bus.shared_lock);
    kfree(shared.priv);
    kfree(shared);
    }
    phydev.shared = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(phy_package_leave);
#[no_mangle]
unsafe extern "C" fn devm_phy_package_leave(dev: *mut device, res: *mut c_void) {
    static void devm_phy_package_leave(struct device *dev, void *res)
    {
    phy_package_leave(*(struct phy_device **)res);
    }
//
// devm_phy_package_join - resource managed phy_package_join()
// @dev: device that is registering this PHY package
// @phydev: target phy_device struct
// @base_addr: cookie and base PHY address of PHY package for offset
// calculation of global register access
// @priv_size: if non-zero allocate this amount of bytes for private data
//
// Managed phy_package_join(). Shared storage fetched by this function,
// phy_package_leave() is automatically called on driver detach. See
// phy_package_join() for more information.
//
    int devm_phy_package_join(struct device *dev, struct phy_device *phydev,
    int base_addr, size_t priv_size)
    {
    struct phy_device **ptr;
    int ret;
    ptr = devres_alloc(devm_phy_package_leave, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = phy_package_join(phydev, base_addr, priv_size);
    if (!ret) {
// ptr = phydev;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_phy_package_join);
//
// devm_of_phy_package_join - resource managed of_phy_package_join()
// @dev: device that is registering this PHY package
// @phydev: target phy_device struct
// @priv_size: if non-zero allocate this amount of bytes for private data
//
// Managed of_phy_package_join(). Shared storage fetched by this function,
// phy_package_leave() is automatically called on driver detach. See
// of_phy_package_join() for more information.
//
    int devm_of_phy_package_join(struct device *dev, struct phy_device *phydev,
    size_t priv_size)
    {
    struct phy_device **ptr;
    int ret;
    ptr = devres_alloc(devm_phy_package_leave, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = of_phy_package_join(phydev, priv_size);
    if (!ret) {
// ptr = phydev;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_of_phy_package_join);
    MODULE_DESCRIPTION("PHY package support");
    MODULE_LICENSE("GPL");
