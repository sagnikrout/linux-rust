//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/aer_inject.c
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
// PCIe AER software error injection support.
//
// Debugging PCIe AER code is quite difficult because it is hard to
// trigger various real hardware errors. Software based error
// injection can fake almost all kinds of errors with the help of a
// user space helper tool aer-inject, which can be gotten from:
// https://github.com/intel/aer-inject.git
//
// Copyright 2009 Intel Corporation.
// Huang Ying <ying.huang@intel.com>
//

// Override the existing corrected and uncorrected error masks
    static bool aer_mask_override;
    module_param(aer_mask_override, bool, 0);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aer_error_inj {
    pub bus: u8,
    pub dev: u8,
    pub fn: u8,
    pub uncor_status: u32,
    pub cor_status: u32,
    pub header_log0: u32,
    pub header_log1: u32,
    pub header_log2: u32,
    pub header_log3: u32,
    pub domain: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aer_error {
    pub list: list_head,
    pub domain: u32,
    pub bus: c_uint,
    pub devfn: c_uint,
    pub pos_cap_err: c_int,
    pub uncor_status: u32,
    pub cor_status: u32,
    pub header_log0: u32,
    pub header_log1: u32,
    pub header_log2: u32,
    pub header_log3: u32,
    pub root_status: u32,
    pub source_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_ops {
    pub list: list_head,
    pub bus: *mut pci_bus,
    pub ops: *mut pci_ops,
}

    static LIST_HEAD(einjected);
    static LIST_HEAD(pci_bus_ops_list);
// Protect einjected and pci_bus_ops_list
    static DEFINE_SPINLOCK(inject_lock);
    static void aer_error_init(struct aer_error *err, u32 domain,
    unsigned int bus, unsigned int devfn,
    int pos_cap_err)
    {
    INIT_LIST_HEAD(&err.list);
    err.domain = domain;
    err.bus = bus;
    err.devfn = devfn;
    err.pos_cap_err = pos_cap_err;
    }
// inject_lock must be held before calling
    static struct aer_error *__find_aer_error(u32 domain, unsigned int bus,
    unsigned int devfn)
    {
    struct aer_error *err;
    list_for_each_entry(err, &einjected, list) {
    if (domain == err.domain &&
    bus == err.bus &&
    devfn == err.devfn)
    return err;
    }
    return core::ptr::null_mut();
    }
// inject_lock must be held before calling
    static struct aer_error *__find_aer_error_by_dev(struct pci_dev *dev)
    {
    let mut domain: c_int = pci_domain_nr(dev.bus);
    if (domain < 0)
    return core::ptr::null_mut();
    return __find_aer_error(domain, dev.bus.number, dev.devfn);
    }
// inject_lock must be held before calling
    static struct pci_ops *__find_pci_bus_ops(struct pci_bus *bus)
    {
    struct pci_bus_ops *bus_ops;
    list_for_each_entry(bus_ops, &pci_bus_ops_list, list) {
    if (bus_ops.bus == bus)
    return bus_ops.ops;
    }
    return core::ptr::null_mut();
    }
    static struct pci_bus_ops *pci_bus_ops_pop(void)
    {
    unsigned long flags;
    struct pci_bus_ops *bus_ops;
    spin_lock_irqsave(&inject_lock, flags);
    bus_ops = list_first_entry_or_null(&pci_bus_ops_list,
    struct pci_bus_ops, list);
    if (bus_ops)
    list_del(&bus_ops.list);
    spin_unlock_irqrestore(&inject_lock, flags);
    return bus_ops;
    }
    static u32 *find_pci_config_dword(struct aer_error *err, int where,
    int *prw1cs)
    {
    let mut rw1cs: c_int = 0;
    u32 *target = core::ptr::null_mut();
    if (err.pos_cap_err == -1)
    return core::ptr::null_mut();
    switch (where - err.pos_cap_err) {
    case PCI_ERR_UNCOR_STATUS:
    target = &err.uncor_status;
    rw1cs = 1;
    break;
    case PCI_ERR_COR_STATUS:
    target = &err.cor_status;
    rw1cs = 1;
    break;
    case PCI_ERR_HEADER_LOG:
    target = &err.header_log0;
    break;
    case PCI_ERR_HEADER_LOG+4:
    target = &err.header_log1;
    break;
    case PCI_ERR_HEADER_LOG+8:
    target = &err.header_log2;
    break;
    case PCI_ERR_HEADER_LOG+12:
    target = &err.header_log3;
    break;
    case PCI_ERR_ROOT_STATUS:
    target = &err.root_status;
    rw1cs = 1;
    break;
    case PCI_ERR_ROOT_ERR_SRC:
    target = &err.source_id;
    break;
    }
    if (prw1cs)
// prw1cs = rw1cs;
    return target;
    }
    static int aer_inj_read(struct pci_bus *bus, unsigned int devfn, int where,
    int size, u32 *val)
    {
    struct pci_ops *ops, *my_ops;
    int rv;
    ops = __find_pci_bus_ops(bus);
    if (!ops)
    return -1;
    my_ops = bus.ops;
    bus.ops = ops;
    rv = ops.read(bus, devfn, where, size, val);
    bus.ops = my_ops;
    return rv;
    }
    static int aer_inj_write(struct pci_bus *bus, unsigned int devfn, int where,
    int size, u32 val)
    {
    struct pci_ops *ops, *my_ops;
    int rv;
    ops = __find_pci_bus_ops(bus);
    if (!ops)
    return -1;
    my_ops = bus.ops;
    bus.ops = ops;
    rv = ops.write(bus, devfn, where, size, val);
    bus.ops = my_ops;
    return rv;
    }
    static int aer_inj_read_config(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 *val)
    {
    u32 *sim;
    struct aer_error *err;
    unsigned long flags;
    int domain;
    int rv;
    spin_lock_irqsave(&inject_lock, flags);
    if (size != sizeof(u32))
    goto out;
    domain = pci_domain_nr(bus);
    if (domain < 0)
    goto out;
    err = __find_aer_error(domain, bus.number, devfn);
    if (!err)
    goto out;
    sim = find_pci_config_dword(err, where, core::ptr::null_mut());
    if (sim) {
// val = *sim;
    spin_unlock_irqrestore(&inject_lock, flags);
    return 0;
    }
    out:
    rv = aer_inj_read(bus, devfn, where, size, val);
    spin_unlock_irqrestore(&inject_lock, flags);
    return rv;
    }
    static int aer_inj_write_config(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 val)
    {
    u32 *sim;
    struct aer_error *err;
    unsigned long flags;
    int rw1cs;
    int domain;
    int rv;
    spin_lock_irqsave(&inject_lock, flags);
    if (size != sizeof(u32))
    goto out;
    domain = pci_domain_nr(bus);
    if (domain < 0)
    goto out;
    err = __find_aer_error(domain, bus.number, devfn);
    if (!err)
    goto out;
    sim = find_pci_config_dword(err, where, &rw1cs);
    if (sim) {
    if (rw1cs)
// sim ^= val;
    else
// sim = val;
    spin_unlock_irqrestore(&inject_lock, flags);
    return 0;
    }
    out:
    rv = aer_inj_write(bus, devfn, where, size, val);
    spin_unlock_irqrestore(&inject_lock, flags);
    return rv;
    }
    static struct pci_ops aer_inj_pci_ops = {
    .read = aer_inj_read_config,
    .write = aer_inj_write_config,
    };
    static void pci_bus_ops_init(struct pci_bus_ops *bus_ops,
    struct pci_bus *bus,
    struct pci_ops *ops)
    {
    INIT_LIST_HEAD(&bus_ops.list);
    bus_ops.bus = bus;
    bus_ops.ops = ops;
    }
#[no_mangle]
unsafe extern "C" fn pci_bus_set_aer_ops(bus: *mut pci_bus) -> c_int {
    static int pci_bus_set_aer_ops(struct pci_bus *bus)
    {
    struct pci_ops *ops;
    struct pci_bus_ops *bus_ops;
    unsigned long flags;
    bus_ops = kmalloc_obj(*bus_ops);
    if (!bus_ops)
    return -ENOMEM;
    ops = pci_bus_set_ops(bus, &aer_inj_pci_ops);
    spin_lock_irqsave(&inject_lock, flags);
    if (ops == &aer_inj_pci_ops)
    goto out;
    pci_bus_ops_init(bus_ops, bus, ops);
    list_add(&bus_ops.list, &pci_bus_ops_list);
    bus_ops = core::ptr::null_mut();
    out:
    spin_unlock_irqrestore(&inject_lock, flags);
    kfree(bus_ops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aer_inject(einj: *mut aer_error_inj) -> c_int {
    static int aer_inject(struct aer_error_inj *einj)
    {
    struct aer_error *err, *rperr;
    struct aer_error *err_alloc = core::ptr::null_mut(), *rperr_alloc = core::ptr::null_mut();
    struct pci_dev *dev, *rpdev;
    struct pcie_device *edev;
    struct device *device;
    unsigned long flags;
    let mut devfn: c_uint = PCI_DEVFN(einj.dev, einj.fn);
    int pos_cap_err, rp_pos_cap_err;
    u32 sever, cor_mask, uncor_mask, cor_mask_orig = 0, uncor_mask_orig = 0;
    let mut ret: c_int = 0;
    dev = pci_get_domain_bus_and_slot(einj.domain, einj.bus, devfn);
    if (!dev)
    return -ENODEV;
    rpdev = pcie_find_root_port(dev);
// If Root Port not found, try to find an RCEC
    if (!rpdev)
    rpdev = dev.rcec;
    if (!rpdev) {
    pci_err(dev, "Neither Root Port nor RCEC found\n");
    ret = -ENODEV;
    goto out_put;
    }
    pos_cap_err = dev.aer_cap;
    if (!pos_cap_err) {
    pci_err(dev, "Device doesn't support AER\n");
    ret = -EPROTONOSUPPORT;
    goto out_put;
    }
    pci_read_config_dword(dev, pos_cap_err + PCI_ERR_UNCOR_SEVER, &sever);
    pci_read_config_dword(dev, pos_cap_err + PCI_ERR_COR_MASK, &cor_mask);
    pci_read_config_dword(dev, pos_cap_err + PCI_ERR_UNCOR_MASK,
    &uncor_mask);
    rp_pos_cap_err = rpdev.aer_cap;
    if (!rp_pos_cap_err) {
    pci_err(rpdev, "Root port doesn't support AER\n");
    ret = -EPROTONOSUPPORT;
    goto out_put;
    }
    err_alloc =  kzalloc_obj(struct aer_error);
    if (!err_alloc) {
    ret = -ENOMEM;
    goto out_put;
    }
    rperr_alloc =  kzalloc_obj(struct aer_error);
    if (!rperr_alloc) {
    ret = -ENOMEM;
    goto out_put;
    }
    if (aer_mask_override) {
    cor_mask_orig = cor_mask;
    cor_mask &= !(einj.cor_status);
    pci_write_config_dword(dev, pos_cap_err + PCI_ERR_COR_MASK,
    cor_mask);
    uncor_mask_orig = uncor_mask;
    uncor_mask &= !(einj.uncor_status);
    pci_write_config_dword(dev, pos_cap_err + PCI_ERR_UNCOR_MASK,
    uncor_mask);
    }
    spin_lock_irqsave(&inject_lock, flags);
    err = __find_aer_error_by_dev(dev);
    if (!err) {
    err = err_alloc;
    err_alloc = core::ptr::null_mut();
    aer_error_init(err, einj.domain, einj.bus, devfn,
    pos_cap_err);
    list_add(&err.list, &einjected);
    }
    err.uncor_status |= einj.uncor_status;
    err.cor_status |= einj.cor_status;
    err.header_log0 = einj.header_log0;
    err.header_log1 = einj.header_log1;
    err.header_log2 = einj.header_log2;
    err.header_log3 = einj.header_log3;
    if (!aer_mask_override && einj.cor_status &&
    !(einj.cor_status & ~cor_mask)) {
    ret = -EINVAL;
    pci_warn(dev, "The correctable error(s) is masked by device\n");
    spin_unlock_irqrestore(&inject_lock, flags);
    goto out_put;
    }
    if (!aer_mask_override && einj.uncor_status &&
    !(einj.uncor_status & ~uncor_mask)) {
    ret = -EINVAL;
    pci_warn(dev, "The uncorrectable error(s) is masked by device\n");
    spin_unlock_irqrestore(&inject_lock, flags);
    goto out_put;
    }
    rperr = __find_aer_error_by_dev(rpdev);
    if (!rperr) {
    rperr = rperr_alloc;
    rperr_alloc = core::ptr::null_mut();
    aer_error_init(rperr, pci_domain_nr(rpdev.bus),
    rpdev.bus.number, rpdev.devfn,
    rp_pos_cap_err);
    list_add(&rperr.list, &einjected);
    }
    if (einj.cor_status) {
    if (rperr.root_status & PCI_ERR_ROOT_COR_RCV)
    rperr.root_status |= PCI_ERR_ROOT_MULTI_COR_RCV;
    else
    rperr.root_status |= PCI_ERR_ROOT_COR_RCV;
    rperr.source_id &= 0xffff0000;
    rperr.source_id |= PCI_DEVID(einj.bus, devfn);
    }
    if (einj.uncor_status) {
    if (rperr.root_status & PCI_ERR_ROOT_UNCOR_RCV)
    rperr.root_status |= PCI_ERR_ROOT_MULTI_UNCOR_RCV;
    if (sever & einj.uncor_status) {
    rperr.root_status |= PCI_ERR_ROOT_FATAL_RCV;
    if (!(rperr.root_status & PCI_ERR_ROOT_UNCOR_RCV))
    rperr.root_status |= PCI_ERR_ROOT_FIRST_FATAL;
    } else
    rperr.root_status |= PCI_ERR_ROOT_NONFATAL_RCV;
    rperr.root_status |= PCI_ERR_ROOT_UNCOR_RCV;
    rperr.source_id &= 0x0000ffff;
    rperr.source_id |= PCI_DEVID(einj.bus, devfn) << 16;
    }
    spin_unlock_irqrestore(&inject_lock, flags);
    if (aer_mask_override) {
    pci_write_config_dword(dev, pos_cap_err + PCI_ERR_COR_MASK,
    cor_mask_orig);
    pci_write_config_dword(dev, pos_cap_err + PCI_ERR_UNCOR_MASK,
    uncor_mask_orig);
    }
    ret = pci_bus_set_aer_ops(dev.bus);
    if (ret)
    goto out_put;
    ret = pci_bus_set_aer_ops(rpdev.bus);
    if (ret)
    goto out_put;
    device = pcie_port_find_device(rpdev, PCIE_PORT_SERVICE_AER);
    if (device) {
    edev = to_pcie_device(device);
    if (!get_service_data(edev)) {
    pci_warn(edev.port, "AER service is not initialized\n");
    ret = -EPROTONOSUPPORT;
    goto out_put;
    }
    pci_info(edev.port, "Injecting errors %08x/%08x into device %s\n",
    einj.cor_status, einj.uncor_status, pci_name(dev));
    ret = irq_inject_interrupt(edev.irq);
    } else {
    pci_err(rpdev, "AER device not found\n");
    ret = -ENODEV;
    }
    out_put:
    kfree(err_alloc);
    kfree(rperr_alloc);
    pci_dev_put(dev);
    return ret;
    }
    static ssize_t aer_inject_write(struct file *filp, const char __user *ubuf,
    size_t usize, loff_t *off)
    {
    struct aer_error_inj einj;
    int ret;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    if (usize < offsetof(struct aer_error_inj, domain) ||
    usize > sizeof(einj))
    return -EINVAL;
    memset(&einj, 0, sizeof(einj));
    if (copy_from_user(&einj, ubuf, usize))
    return -EFAULT;
    ret = aer_inject(&einj);
    return ret ? ret : usize;
    }
    static const struct file_operations aer_inject_fops = {
    .write = aer_inject_write,
    .owner = THIS_MODULE,
    .llseek = noop_llseek,
    };
    static struct miscdevice aer_inject_device = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "aer_inject",
    .fops = &aer_inject_fops,
    };
#[no_mangle]
unsafe extern "C" fn aer_inject_init() -> int __init {
    static int __init aer_inject_init(void)
    {
    return misc_register(&aer_inject_device);
    }
#[no_mangle]
unsafe extern "C" fn aer_inject_exit() -> void __exit {
    static void __exit aer_inject_exit(void)
    {
    struct aer_error *err, *err_next;
    unsigned long flags;
    struct pci_bus_ops *bus_ops;
    misc_deregister(&aer_inject_device);
    while ((bus_ops = pci_bus_ops_pop())) {
    pci_bus_set_ops(bus_ops.bus, bus_ops.ops);
    kfree(bus_ops);
    }
    spin_lock_irqsave(&inject_lock, flags);
    list_for_each_entry_safe(err, err_next, &einjected, list) {
    list_del(&err.list);
    kfree(err);
    }
    spin_unlock_irqrestore(&inject_lock, flags);
    }
    module_init(aer_inject_init);
    module_exit(aer_inject_exit);
    MODULE_DESCRIPTION("PCIe AER software error injector");
    MODULE_LICENSE("GPL");
