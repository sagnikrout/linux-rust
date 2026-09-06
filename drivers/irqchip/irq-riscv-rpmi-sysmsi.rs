//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-riscv-rpmi-sysmsi.c
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
// Copyright (C) 2025 Ventana Micro Systems Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_get_attrs_rx {
    pub status: __le32,
    pub sys_num_msi: __le32,
    pub flag0: __le32,
    pub flag1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_set_msi_state_tx {
    pub sys_msi_index: __le32,
    pub sys_msi_state: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_set_msi_state_rx {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_set_msi_target_tx {
    pub sys_msi_index: __le32,
    pub sys_msi_address_low: __le32,
    pub sys_msi_address_high: __le32,
    pub sys_msi_data: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_set_msi_target_rx {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmi_sysmsi_priv {
    pub dev: *mut device,
    pub client: mbox_client,
    pub chan: *mut mbox_chan,
    pub nr_irqs: u32,
    pub gsi_base: u32,
}

#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_get_num_msi(priv: *mut rpmi_sysmsi_priv) -> c_int {
    static int rpmi_sysmsi_get_num_msi(struct rpmi_sysmsi_priv *priv)
    {
    struct rpmi_sysmsi_get_attrs_rx rx;
    struct rpmi_mbox_message msg;
    int ret;
    rpmi_mbox_init_send_with_response(&msg, RPMI_SYSMSI_SRV_GET_ATTRIBUTES,
    core::ptr::null_mut(), 0, &rx, sizeof(rx));
    ret = rpmi_mbox_send_message(priv.chan, &msg);
    if (ret)
    return ret;
    if (rx.status)
    return rpmi_to_linux_error(le32_to_cpu(rx.status));
    return le32_to_cpu(rx.sys_num_msi);
    }
    static int rpmi_sysmsi_set_msi_state(struct rpmi_sysmsi_priv *priv,
    u32 sys_msi_index, u32 sys_msi_state)
    {
    struct rpmi_sysmsi_set_msi_state_tx tx;
    struct rpmi_sysmsi_set_msi_state_rx rx;
    struct rpmi_mbox_message msg;
    int ret;
    tx.sys_msi_index = cpu_to_le32(sys_msi_index);
    tx.sys_msi_state = cpu_to_le32(sys_msi_state);
    rpmi_mbox_init_send_with_response(&msg, RPMI_SYSMSI_SRV_SET_MSI_STATE,
    &tx, sizeof(tx), &rx, sizeof(rx));
    ret = rpmi_mbox_send_message(priv.chan, &msg);
    if (ret)
    return ret;
    if (rx.status)
    return rpmi_to_linux_error(le32_to_cpu(rx.status));
    return 0;
    }
    static int rpmi_sysmsi_set_msi_target(struct rpmi_sysmsi_priv *priv,
    u32 sys_msi_index, struct msi_msg *m)
    {
    struct rpmi_sysmsi_set_msi_target_tx tx;
    struct rpmi_sysmsi_set_msi_target_rx rx;
    struct rpmi_mbox_message msg;
    int ret;
    tx.sys_msi_index = cpu_to_le32(sys_msi_index);
    tx.sys_msi_address_low = cpu_to_le32(m.address_lo);
    tx.sys_msi_address_high = cpu_to_le32(m.address_hi);
    tx.sys_msi_data = cpu_to_le32(m.data);
    rpmi_mbox_init_send_with_response(&msg, RPMI_SYSMSI_SRV_SET_MSI_TARGET,
    &tx, sizeof(tx), &rx, sizeof(rx));
    ret = rpmi_mbox_send_message(priv.chan, &msg);
    if (ret)
    return ret;
    if (rx.status)
    return rpmi_to_linux_error(le32_to_cpu(rx.status));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_irq_mask(d: *mut irq_data) {
    static void rpmi_sysmsi_irq_mask(struct irq_data *d)
    {
    struct rpmi_sysmsi_priv *priv = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    int ret;
    ret = rpmi_sysmsi_set_msi_state(priv, hwirq, 0);
    if (ret)
    dev_warn(priv.dev, "Failed to mask hwirq %lu (error %d)\n", hwirq, ret);
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_irq_unmask(d: *mut irq_data) {
    static void rpmi_sysmsi_irq_unmask(struct irq_data *d)
    {
    struct rpmi_sysmsi_priv *priv = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    int ret;
    irq_chip_unmask_parent(d);
    ret = rpmi_sysmsi_set_msi_state(priv, hwirq, RPMI_SYSMSI_MSI_STATE_ENABLE);
    if (ret)
    dev_warn(priv.dev, "Failed to unmask hwirq %lu (error %d)\n", hwirq, ret);
    }
#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_write_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void rpmi_sysmsi_write_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct rpmi_sysmsi_priv *priv = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    int ret;
// For zeroed MSI, do nothing as of now
    if (!msg.address_hi && !msg.address_lo && !msg.data)
    return;
    ret = rpmi_sysmsi_set_msi_target(priv, hwirq, msg);
    if (ret)
    dev_warn(priv.dev, "Failed to set target for hwirq %lu (error %d)\n", hwirq, ret);
    }
#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_set_desc(arg: *mut msi_alloc_info_t, desc: *mut msi_desc) {
    static void rpmi_sysmsi_set_desc(msi_alloc_info_t *arg, struct msi_desc *desc)
    {
    arg.desc = desc;
    arg.hwirq = desc.data.icookie.value;
    }
    static int rpmi_sysmsi_translate(struct irq_domain *d, struct irq_fwspec *fwspec,
    unsigned long *hwirq, unsigned int *type)
    {
    struct msi_domain_info *info = d.host_data;
    struct rpmi_sysmsi_priv *priv = info.data;
    if (WARN_ON(fwspec.param_count < 1))
    return -EINVAL;
// For DT, gsi_base is always zero.
// hwirq = fwspec->param[0] - priv->gsi_base;
// type = IRQ_TYPE_NONE;
    return 0;
    }
    static const struct msi_domain_template rpmi_sysmsi_template = {
    .chip = {
    .name			= "RPMI-SYSMSI",
    .irq_mask		= rpmi_sysmsi_irq_mask,
    .irq_unmask		= rpmi_sysmsi_irq_unmask,

    .irq_set_affinity	= irq_chip_set_affinity_parent,

    .irq_write_msi_msg	= rpmi_sysmsi_write_msg,
    .flags			= IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_MASK_ON_SUSPEND,
    },
    .ops = {
    .set_desc		= rpmi_sysmsi_set_desc,
    .msi_translate		= rpmi_sysmsi_translate,
    },
    .info = {
    .bus_token		= DOMAIN_BUS_WIRED_TO_MSI,
    .flags			= MSI_FLAG_USE_DEV_FWNODE,
    .handler		= handle_simple_irq,
    .handler_name		= "simple",
    },
    };
#[no_mangle]
unsafe extern "C" fn rpmi_sysmsi_probe(pdev: *mut platform_device) -> c_int {
    static int rpmi_sysmsi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rpmi_sysmsi_priv *priv;
    struct fwnode_handle *fwnode;
    u32 id;
    int rc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
// Setup mailbox client
    priv.client.dev		= priv.dev;
    priv.client.rx_callback	= core::ptr::null_mut();
    priv.client.tx_block		= false;
    priv.client.knows_txdone	= true;
    priv.client.tx_tout		= 0;
// Request mailbox channel
    priv.chan = mbox_request_channel(&priv.client, 0);
    if (IS_ERR(priv.chan))
    return PTR_ERR(priv.chan);
// Get number of system MSIs
    rc = rpmi_sysmsi_get_num_msi(priv);
    if (rc < 1) {
    mbox_free_channel(priv.chan);
    if (rc)
    return dev_err_probe(dev, rc, "Failed to get number of system MSIs\n");
    else
    return dev_err_probe(dev, -ENODEV, "No system MSIs found\n");
    }
    priv.nr_irqs = rc;
    fwnode = dev_fwnode(dev);
    if (is_acpi_node(fwnode)) {
    u32 nr_irqs;
    rc = riscv_acpi_get_gsi_info(fwnode, &priv.gsi_base, &id,
    &nr_irqs, core::ptr::null_mut());
    if (rc) {
    mbox_free_channel(priv.chan);
    dev_err(dev, "failed to find GSI mapping\n");
    return rc;
    }
// Update with actual GSI range
    if (nr_irqs != priv.nr_irqs)
    riscv_acpi_update_gsi_range(priv.gsi_base, priv.nr_irqs);
    }
//
// The device MSI domain for platform devices on RISC-V architecture
// is only available after the MSI controller driver is probed so,
// explicitly configure here.
//
    if (!dev_get_msi_domain(dev)) {
//
// The device MSI domain for OF devices is only set at the
// time of populating/creating OF device. If the device MSI
// domain is discovered later after the OF device is created
// then we need to set it explicitly before using any platform
// MSI functions.
//
    if (is_of_node(fwnode)) {
    of_msi_configure(dev, dev_of_node(dev));
    } else if (is_acpi_device_node(fwnode)) {
    struct irq_domain *msi_domain;
    msi_domain = irq_find_matching_fwnode(imsic_acpi_get_fwnode(dev),
    DOMAIN_BUS_PLATFORM_MSI);
    dev_set_msi_domain(dev, msi_domain);
    }
    if (!dev_get_msi_domain(dev)) {
    mbox_free_channel(priv.chan);
    return -EPROBE_DEFER;
    }
    }
    if (!msi_create_device_irq_domain(dev, MSI_DEFAULT_DOMAIN,
    &rpmi_sysmsi_template,
    priv.nr_irqs, priv, priv)) {
    mbox_free_channel(priv.chan);
    return dev_err_probe(dev, -ENOMEM, "failed to create MSI irq domain\n");
    }

    struct acpi_device *adev = ACPI_COMPANION(dev);
    if (adev)
    acpi_dev_clear_dependencies(adev);

    dev_info(dev, "%u system MSIs registered\n", priv.nr_irqs);
    return 0;
    }
    static const struct of_device_id rpmi_sysmsi_match[] = {
    { .compatible = "riscv,rpmi-system-msi" },
    {}
    };
    static const struct acpi_device_id acpi_rpmi_sysmsi_match[] = {
    { "RSCV0006" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, acpi_rpmi_sysmsi_match);
    static struct platform_driver rpmi_sysmsi_driver = {
    .driver = {
    .name			= "rpmi-sysmsi",
    .of_match_table		= rpmi_sysmsi_match,
    .acpi_match_table	= acpi_rpmi_sysmsi_match,
    },
    .probe = rpmi_sysmsi_probe,
    };
    builtin_platform_driver(rpmi_sysmsi_driver);
