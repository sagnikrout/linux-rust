//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_platform.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// ipmi_si_platform.c
//
// Handling for platform devices in IPMI (ACPI, OF, and things
// coming from the platform.
//

    static bool platform_registered;
    let mut si_tryplatform: static bool = true;

    let mut si_tryacpi: static bool = true;

    let mut si_tryopenfirmware: static bool = true;

    let mut si_trydmi: static bool = true;

    let mut si_trydmi: static bool = false;

    module_param_named(tryplatform, si_tryplatform, bool, 0);
    MODULE_PARM_DESC(tryplatform,
    "Setting this to zero will disable the default scan of the interfaces identified via platform interfaces besides ACPI, OpenFirmware, and DMI");

    module_param_named(tryacpi, si_tryacpi, bool, 0);
    MODULE_PARM_DESC(tryacpi,
    "Setting this to zero will disable the default scan of the interfaces identified via ACPI");

    module_param_named(tryopenfirmware, si_tryopenfirmware, bool, 0);
    MODULE_PARM_DESC(tryopenfirmware,
    "Setting this to zero will disable the default scan of the interfaces identified via OpenFirmware");

    module_param_named(trydmi, si_trydmi, bool, 0);
    MODULE_PARM_DESC(trydmi,
    "Setting this to zero will disable the default scan of the interfaces identified via DMI");

// For GPE-type interrupts.
    static u32 ipmi_acpi_gpe(acpi_handle gpe_device,
    u32 gpe_number, void *context)
    {
    struct si_sm_io *io = context;
    ipmi_si_irq_handler(io.irq, io.irq_handler_data);
    return ACPI_INTERRUPT_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpe_irq_cleanup(io: *mut si_sm_io) {
    static void acpi_gpe_irq_cleanup(struct si_sm_io *io)
    {
    if (!io.irq)
    return;
    ipmi_irq_start_cleanup(io);
    acpi_remove_gpe_handler(core::ptr::null_mut(), io.irq, &ipmi_acpi_gpe);
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpe_irq_setup(io: *mut si_sm_io) -> c_int {
    static int acpi_gpe_irq_setup(struct si_sm_io *io)
    {
    acpi_status status;
    if (!io.irq)
    return 0;
    status = acpi_install_gpe_handler(core::ptr::null_mut(),
    io.irq,
    ACPI_GPE_LEVEL_TRIGGERED,
    &ipmi_acpi_gpe,
    io);
    if (ACPI_FAILURE(status)) {
    dev_warn(io.dev,
    "Unable to claim ACPI GPE %d, running polled\n",
    io.irq);
    io.irq = 0;
    return -EINVAL;
    }
    io.irq_cleanup = acpi_gpe_irq_cleanup;
    ipmi_irq_finish_setup(io);
    dev_info(io.dev, "Using ACPI GPE %d\n", io.irq);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ipmi_set_addr_data_and_space(r: *mut resource, io: *mut si_sm_io) {
    static void ipmi_set_addr_data_and_space(struct resource *r, struct si_sm_io *io)
    {
    if (resource_type(r) == IORESOURCE_IO)
    io.addr_space = IPMI_IO_ADDR_SPACE;
    else
    io.addr_space = IPMI_MEM_ADDR_SPACE;
    io.addr_data = r.start;
    }
    static struct resource *
    ipmi_get_info_from_resources(struct platform_device *pdev,
    struct si_sm_io *io)
    {
    struct resource *res, *res_second;
    res = platform_get_mem_or_io(pdev, 0);
    if (!res) {
    dev_err(&pdev.dev, "no I/O or memory address\n");
    return core::ptr::null_mut();
    }
    ipmi_set_addr_data_and_space(res, io);
    io.regspacing = DEFAULT_REGSPACING;
    res_second = platform_get_mem_or_io(pdev, 1);
    if (res_second && resource_type(res_second) == resource_type(res)) {
    if (res_second.start > io.addr_data)
    io.regspacing = res_second.start - io.addr_data;
    }
    return res;
    }
#[no_mangle]
unsafe extern "C" fn platform_ipmi_probe(pdev: *mut platform_device) -> c_int {
    static int platform_ipmi_probe(struct platform_device *pdev)
    {
    struct si_sm_io io;
    u8 type, slave_addr, addr_source, regsize, regshift;
    int rv;
    rv = device_property_read_u8(&pdev.dev, "addr-source", &addr_source);
    if (rv)
    addr_source = SI_PLATFORM;
    if (addr_source >= SI_LAST)
    return -EINVAL;
    if (addr_source == SI_SMBIOS) {
    if (!si_trydmi)
    return -ENODEV;
    } else if (addr_source != SI_HARDCODED) {
    if (!si_tryplatform)
    return -ENODEV;
    }
    rv = device_property_read_u8(&pdev.dev, "ipmi-type", &type);
    if (rv)
    return -ENODEV;
    memset(&io, 0, sizeof(io));
    io.addr_source = addr_source;
    dev_info(&pdev.dev, "probing via %s\n",
    ipmi_addr_src_to_str(addr_source));
    switch (type) {
    case SI_KCS:
    io.si_info = &ipmi_kcs_si_info;
    break;
    case SI_SMIC:
    io.si_info = &ipmi_smic_si_info;
    break;
    case SI_BT:
    io.si_info = &ipmi_bt_si_info;
    break;
    case SI_TYPE_INVALID: /* User disabled this in hardcode. */
    return -ENODEV;
    default:
    dev_err(&pdev.dev, "ipmi-type property is invalid\n");
    return -EINVAL;
    }
    io.regsize = DEFAULT_REGSIZE;
    rv = device_property_read_u8(&pdev.dev, "reg-size", &regsize);
    if (!rv)
    io.regsize = regsize;
    io.regshift = 0;
    rv = device_property_read_u8(&pdev.dev, "reg-shift", &regshift);
    if (!rv)
    io.regshift = regshift;
    if (!ipmi_get_info_from_resources(pdev, &io))
    return -EINVAL;
    rv = device_property_read_u8(&pdev.dev, "slave-addr", &slave_addr);
    if (rv)
    io.slave_addr = 0x20;
    else
    io.slave_addr = slave_addr;
    io.irq = platform_get_irq_optional(pdev, 0);
    if (io.irq > 0)
    io.irq_setup = ipmi_std_irq_setup;
    else
    io.irq = 0;
    io.dev = &pdev.dev;
    pr_info("ipmi_si: %s: %s %#lx regsize %d spacing %d irq %d\n",
    ipmi_addr_src_to_str(addr_source),
    (io.addr_space == IPMI_IO_ADDR_SPACE) ? "io" : "mem",
    io.addr_data, io.regsize, io.regspacing, io.irq);
    ipmi_si_add_smi(&io);
    return 0;
    }

    static const struct of_device_id of_ipmi_match[] = {
    { .type = "ipmi", .compatible = "ipmi-kcs", .data = &ipmi_kcs_si_info },
    { .type = "ipmi", .compatible = "ipmi-smic", .data = &ipmi_smic_si_info },
    { .type = "ipmi", .compatible = "ipmi-bt", .data = &ipmi_bt_si_info },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_ipmi_match);
#[no_mangle]
unsafe extern "C" fn of_ipmi_probe(pdev: *mut platform_device) -> c_int {
    static int of_ipmi_probe(struct platform_device *pdev)
    {
    struct si_sm_io io;
    struct resource resource;
    const __be32 *regsize, *regspacing, *regshift;
    struct device_node *np = pdev.dev.of_node;
    int ret;
    int proplen;
    if (!si_tryopenfirmware)
    return -ENODEV;
    dev_info(&pdev.dev, "probing via device tree\n");
    if (!of_device_is_available(np))
    return -EINVAL;
    ret = of_address_to_resource(np, 0, &resource);
    if (ret) {
    dev_warn(&pdev.dev, "invalid address from OF\n");
    return ret;
    }
    regsize = of_get_property(np, "reg-size", &proplen);
    if (regsize && proplen != 4) {
    dev_warn(&pdev.dev, "invalid regsize from OF\n");
    return -EINVAL;
    }
    regspacing = of_get_property(np, "reg-spacing", &proplen);
    if (regspacing && proplen != 4) {
    dev_warn(&pdev.dev, "invalid regspacing from OF\n");
    return -EINVAL;
    }
    regshift = of_get_property(np, "reg-shift", &proplen);
    if (regshift && proplen != 4) {
    dev_warn(&pdev.dev, "invalid regshift from OF\n");
    return -EINVAL;
    }
    memset(&io, 0, sizeof(io));
    io.si_info	= device_get_match_data(&pdev.dev);
    io.addr_source	= SI_DEVICETREE;
    io.irq_setup	= ipmi_std_irq_setup;
    ipmi_set_addr_data_and_space(&resource, &io);
    io.regsize	= regsize ? be32_to_cpup(regsize) : DEFAULT_REGSIZE;
    io.regspacing	= regspacing ? be32_to_cpup(regspacing) : DEFAULT_REGSPACING;
    io.regshift	= regshift ? be32_to_cpup(regshift) : 0;
    io.irq = platform_get_irq_optional(pdev, 0);
    if (io.irq < 0)
    io.irq = 0;
    io.dev		= &pdev.dev;
    dev_dbg(&pdev.dev, "addr 0x%lx regsize %d spacing %d irq %d\n",
    io.addr_data, io.regsize, io.regspacing, io.irq);
    return ipmi_si_add_smi(&io);
    }

#[no_mangle]
unsafe extern "C" fn of_ipmi_probe(dev: *mut platform_device) -> c_int {
    static int of_ipmi_probe(struct platform_device *dev)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn find_slave_address(io: *mut si_sm_io, slave_addr: c_int) -> c_int {
    static int find_slave_address(struct si_sm_io *io, int slave_addr)
    {

    if (!slave_addr)
    slave_addr = ipmi_dmi_get_slave_addr(io.si_info.type,
    io.addr_space,
    io.addr_data);

    return slave_addr;
    }
#[no_mangle]
unsafe extern "C" fn acpi_ipmi_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_ipmi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct si_sm_io io;
    acpi_handle handle;
    acpi_status status;
    unsigned long long tmp;
    struct resource *res;
    if (!si_tryacpi)
    return -ENODEV;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return -ENODEV;
    memset(&io, 0, sizeof(io));
    io.addr_source = SI_ACPI;
    dev_info(dev, "probing via ACPI\n");
    io.addr_info.acpi_info.acpi_handle = handle;
// _IFT tells us the interface type: KCS, BT, etc
    status = acpi_evaluate_integer(handle, "_IFT", core::ptr::null_mut(), &tmp);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "Could not find ACPI IPMI interface type\n");
    return -EINVAL;
    }
    switch (tmp) {
    case 1:
    io.si_info = &ipmi_kcs_si_info;
    break;
    case 2:
    io.si_info = &ipmi_smic_si_info;
    break;
    case 3:
    io.si_info = &ipmi_bt_si_info;
    break;
    case 4: /* SSIF, just ignore */
    return -ENODEV;
    default:
    dev_info(dev, "unknown IPMI type %lld\n", tmp);
    return -EINVAL;
    }
    io.dev = dev;
    io.regsize = DEFAULT_REGSIZE;
    io.regshift = 0;
    res = ipmi_get_info_from_resources(pdev, &io);
    if (!res)
    return -EINVAL;
// If _GPE exists, use it; otherwise use standard interrupts
    status = acpi_evaluate_integer(handle, "_GPE", core::ptr::null_mut(), &tmp);
    if (ACPI_SUCCESS(status)) {
    io.irq = tmp;
    io.irq_setup = acpi_gpe_irq_setup;
    } else {
    let mut irq: c_int = platform_get_irq_optional(pdev, 0);
    if (irq > 0) {
    io.irq = irq;
    io.irq_setup = ipmi_std_irq_setup;
    }
    }
    io.slave_addr = find_slave_address(&io, io.slave_addr);
    dev_info(dev, "%pR regsize %d spacing %d irq %d\n",
    res, io.regsize, io.regspacing, io.irq);
    request_module_nowait("acpi_ipmi");
    return ipmi_si_add_smi(&io);
    }
    static const struct acpi_device_id acpi_ipmi_match[] = {
    { .id = "IPI0001" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, acpi_ipmi_match);

#[no_mangle]
unsafe extern "C" fn acpi_ipmi_probe(dev: *mut platform_device) -> c_int {
    static int acpi_ipmi_probe(struct platform_device *dev)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn ipmi_probe(pdev: *mut platform_device) -> c_int {
    static int ipmi_probe(struct platform_device *pdev)
    {
    if (pdev.dev.of_node && of_ipmi_probe(pdev) == 0)
    return 0;
    if (acpi_ipmi_probe(pdev) == 0)
    return 0;
    return platform_ipmi_probe(pdev);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_remove(pdev: *mut platform_device) {
    static void ipmi_remove(struct platform_device *pdev)
    {
    ipmi_si_remove_by_dev(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn pdev_match_name(dev: *mut device, data: *const c_void) -> c_int {
    static int pdev_match_name(struct device *dev, const void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    const char *name = data;
    return strcmp(pdev.name, name) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_remove_platform_device_by_name(name: *mut c_char) {
    void ipmi_remove_platform_device_by_name(char *name)
    {
    struct device *dev;
    while ((dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), name,
    pdev_match_name))) {
    struct platform_device *pdev = to_platform_device(dev);
    platform_device_unregister(pdev);
    put_device(dev);
    }
    }
    static const struct platform_device_id si_plat_ids[] = {
    { .name = "dmi-ipmi-si" },
    { .name = "hardcode-ipmi-si" },
    { .name = "hotmod-ipmi-si" },
    { }
    };
    struct platform_driver ipmi_platform_driver = {
    .driver = {
    .name = SI_DEVICE_NAME,
    .of_match_table = of_ipmi_match,
    .acpi_match_table = ACPI_PTR(acpi_ipmi_match),
    },
    .probe		= ipmi_probe,
    .remove		= ipmi_remove,
    .id_table       = si_plat_ids
    };
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_platform_init() {
    void ipmi_si_platform_init(void)
    {
    let mut rv: c_int = platform_driver_register(&ipmi_platform_driver);
    if (rv)
    pr_err("Unable to register driver: %d\n", rv);
    else
    platform_registered = true;
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_platform_shutdown() {
    void ipmi_si_platform_shutdown(void)
    {
    if (platform_registered)
    platform_driver_unregister(&ipmi_platform_driver);
    }
