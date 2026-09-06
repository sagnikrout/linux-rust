//! Automatically rewritten from C to Rust
//! Source: drivers/char/agp/sis-agp.c
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


//
// SiS AGPGART routines.
//

pub const SIS_ATTBASE: c_uint = 0x90;
pub const SIS_APSIZE: c_uint = 0x94;
pub const SIS_TLBCNTRL: c_uint = 0x97;
pub const SIS_TLBFLUSH: c_uint = 0x98;
pub const PCI_DEVICE_ID_SI_662: c_uint = 0x0662;
pub const PCI_DEVICE_ID_SI_671: c_uint = 0x0671;
    let mut agp_sis_force_delay: static bool = 0;
    let mut agp_sis_agp_spec: static int = -1;
#[no_mangle]
unsafe extern "C" fn sis_fetch_size() -> c_int {
    static int sis_fetch_size(void)
    {
    u8 temp_size;
    int i;
    struct aper_size_info_8 *values;
    pci_read_config_byte(agp_bridge.dev, SIS_APSIZE, &temp_size);
    values = A_SIZE_8(agp_bridge.driver.aperture_sizes);
    for (i = 0; i < agp_bridge.driver.num_aperture_sizes; i++) {
    if ((temp_size == values[i].size_value) ||
    ((temp_size & ~(0x07)) ==
    (values[i].size_value & ~(0x07)))) {
    agp_bridge.previous_size =
    agp_bridge.current_size = (void *) (values + i);
    agp_bridge.aperture_size_idx = i;
    return values[i].size;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sis_tlbflush(mem: *mut agp_memory) {
    static void sis_tlbflush(struct agp_memory *mem)
    {
    pci_write_config_byte(agp_bridge.dev, SIS_TLBFLUSH, 0x02);
    }
#[no_mangle]
unsafe extern "C" fn sis_configure() -> c_int {
    static int sis_configure(void)
    {
    struct aper_size_info_8 *current_size;
    current_size = A_SIZE_8(agp_bridge.current_size);
    pci_write_config_byte(agp_bridge.dev, SIS_TLBCNTRL, 0x05);
    agp_bridge.gart_bus_addr = pci_bus_address(agp_bridge.dev,
    AGP_APERTURE_BAR);
    pci_write_config_dword(agp_bridge.dev, SIS_ATTBASE,
    agp_bridge.gatt_bus_addr);
    pci_write_config_byte(agp_bridge.dev, SIS_APSIZE,
    current_size.size_value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sis_cleanup() {
    static void sis_cleanup(void)
    {
    struct aper_size_info_8 *previous_size;
    previous_size = A_SIZE_8(agp_bridge.previous_size);
    pci_write_config_byte(agp_bridge.dev, SIS_APSIZE,
    (previous_size.size_value & ~(0x03)));
    }
#[no_mangle]
unsafe extern "C" fn sis_delayed_enable(bridge: *mut agp_bridge_data, mode: u32) {
    static void sis_delayed_enable(struct agp_bridge_data *bridge, u32 mode)
    {
    struct pci_dev *device = core::ptr::null_mut();
    u32 command;
    int rate;
    dev_info(&agp_bridge.dev.dev, "AGP %d.%d bridge\n",
    agp_bridge.major_version, agp_bridge.minor_version);
    pci_read_config_dword(agp_bridge.dev, agp_bridge.capndx + PCI_AGP_STATUS, &command);
    command = agp_collect_device_status(bridge, mode, command);
    command |= AGPSTAT_AGP_ENABLE;
    rate = (command & 0x7) << 2;
    for_each_pci_dev(device) {
    let mut agp: u8 = pci_find_capability(device, PCI_CAP_ID_AGP);
    if (!agp)
    continue;
    dev_info(&agp_bridge.dev.dev, "putting AGP V3 device at %s into %dx mode\n",
    pci_name(device), rate);
    pci_write_config_dword(device, agp + PCI_AGP_COMMAND, command);
//
// Weird: on some sis chipsets any rate change in the target
// command register triggers a 5ms screwup during which the master
// cannot be configured
//
    if (device.device == bridge.dev.device) {
    dev_info(&agp_bridge.dev.dev, "SiS delay workaround: giving bridge time to recover\n");
    msleep(10);
    }
    }
    }
    static const struct aper_size_info_8 sis_generic_sizes[7] =
    {
    {256, 65536, 6, 99},
    {128, 32768, 5, 83},
    {64, 16384, 4, 67},
    {32, 8192, 3, 51},
    {16, 4096, 2, 35},
    {8, 2048, 1, 19},
    {4, 1024, 0, 3}
    };
    static struct agp_bridge_driver sis_driver = {
    .owner			= THIS_MODULE,
    .aperture_sizes		= sis_generic_sizes,
    .size_type		= U8_APER_SIZE,
    .num_aperture_sizes	= 7,
    .needs_scratch_page	= true,
    .configure		= sis_configure,
    .fetch_size		= sis_fetch_size,
    .cleanup		= sis_cleanup,
    .tlb_flush		= sis_tlbflush,
    .mask_memory		= agp_generic_mask_memory,
    .masks			= core::ptr::null_mut(),
    .agp_enable		= agp_generic_enable,
    .cache_flush		= global_cache_flush,
    .create_gatt_table	= agp_generic_create_gatt_table,
    .free_gatt_table	= agp_generic_free_gatt_table,
    .insert_memory		= agp_generic_insert_memory,
    .remove_memory		= agp_generic_remove_memory,
    .alloc_by_type		= agp_generic_alloc_by_type,
    .free_by_type		= agp_generic_free_by_type,
    .agp_alloc_page		= agp_generic_alloc_page,
    .agp_alloc_pages	= agp_generic_alloc_pages,
    .agp_destroy_page	= agp_generic_destroy_page,
    .agp_destroy_pages	= agp_generic_destroy_pages,
    .agp_type_to_mask_type  = agp_generic_type_to_mask_type,
    };
// chipsets that require the 'delay hack'
    static int sis_broken_chipsets[] = {
    PCI_DEVICE_ID_SI_648,
    PCI_DEVICE_ID_SI_746,
    0 // terminator
    };
#[no_mangle]
unsafe extern "C" fn sis_get_driver(bridge: *mut agp_bridge_data) {
    static void sis_get_driver(struct agp_bridge_data *bridge)
    {
    int i;
    for (i=0; sis_broken_chipsets[i]!=0; ++i)
    if (bridge.dev.device==sis_broken_chipsets[i])
    break;
    if (sis_broken_chipsets[i] || agp_sis_force_delay)
    sis_driver.agp_enable=sis_delayed_enable;
// sis chipsets that indicate less than agp3.5
// are not actually fully agp3 compliant
    if ((agp_bridge.major_version == 3 && agp_bridge.minor_version >= 5
    && agp_sis_agp_spec!=0) || agp_sis_agp_spec==1) {
    sis_driver.aperture_sizes = agp3_generic_sizes;
    sis_driver.size_type = U16_APER_SIZE;
    sis_driver.num_aperture_sizes = AGP_GENERIC_SIZES_ENTRIES;
    sis_driver.configure = agp3_generic_configure;
    sis_driver.fetch_size = agp3_generic_fetch_size;
    sis_driver.cleanup = agp3_generic_cleanup;
    sis_driver.tlb_flush = agp3_generic_tlbflush;
    }
    }
#[no_mangle]
unsafe extern "C" fn agp_sis_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int agp_sis_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct agp_bridge_data *bridge;
    u8 cap_ptr;
    cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if (!cap_ptr)
    return -ENODEV;
    dev_info(&pdev.dev, "SiS chipset [%04x/%04x]\n",
    pdev.vendor, pdev.device);
    bridge = agp_alloc_bridge();
    if (!bridge)
    return -ENOMEM;
    bridge.driver = &sis_driver;
    bridge.dev = pdev;
    bridge.capndx = cap_ptr;
    get_agp_version(bridge);
// Fill in the mode register
    pci_read_config_dword(pdev, bridge.capndx+PCI_AGP_STATUS, &bridge.mode);
    sis_get_driver(bridge);
    pci_set_drvdata(pdev, bridge);
    return agp_add_bridge(bridge);
    }
#[no_mangle]
unsafe extern "C" fn agp_sis_remove(pdev: *mut pci_dev) {
    static void agp_sis_remove(struct pci_dev *pdev)
    {
    struct agp_bridge_data *bridge = pci_get_drvdata(pdev);
    agp_remove_bridge(bridge);
    agp_put_bridge(bridge);
    }
#[no_mangle]
unsafe extern "C" fn agp_sis_resume(dev: *mut __attribute__((unused)) struct device) -> c_int {
    static int agp_sis_resume(__attribute__((unused)) struct device *dev)
    {
    return sis_driver.configure();
    }
    static const struct pci_device_id agp_sis_pci_table[] = {
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_5591,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_530,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_540,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_550,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_620,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_630,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_635,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_645,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_646,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_648,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_650,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_651,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_655,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_661,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_662,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_671,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_730,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_735,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_740,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_741,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_745,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_SI,
    .device		= PCI_DEVICE_ID_SI_746,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, agp_sis_pci_table);
    static DEFINE_SIMPLE_DEV_PM_OPS(agp_sis_pm_ops, core::ptr::null_mut(), agp_sis_resume);
    static struct pci_driver agp_sis_pci_driver = {
    .name		= "agpgart-sis",
    .id_table	= agp_sis_pci_table,
    .probe		= agp_sis_probe,
    .remove		= agp_sis_remove,
    .driver.pm      = &agp_sis_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn agp_sis_init() -> int __init {
    static int __init agp_sis_init(void)
    {
    if (agp_off)
    return -EINVAL;
    return pci_register_driver(&agp_sis_pci_driver);
    }
#[no_mangle]
unsafe extern "C" fn agp_sis_cleanup() -> void __exit {
    static void __exit agp_sis_cleanup(void)
    {
    pci_unregister_driver(&agp_sis_pci_driver);
    }
    module_init(agp_sis_init);
    module_exit(agp_sis_cleanup);
    module_param(agp_sis_force_delay, bool, 0);
    MODULE_PARM_DESC(agp_sis_force_delay,"forces sis delay hack");
    module_param(agp_sis_agp_spec, int, 0);
    MODULE_PARM_DESC(agp_sis_agp_spec,"0=force sis init, 1=force generic agp3 init, default: autodetect");
    MODULE_DESCRIPTION("SiS AGPGART routines");
    MODULE_LICENSE("GPL and additional rights");
