//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/pismo.c
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
// PISMO memory driver - http://www.pismoworld.org
//
// For ARM Realview and Versatile platforms
//

pub const PISMO_NUM_CS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pismo_cs_block {
    pub type: u8,
    pub width: u8,
    pub access: __le16,
    pub size: __le32,
    pub reserved: [u32; 2],
    pub device: [c_char; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pismo_eeprom {
    pub cs: [pismo_cs_block; PISMO_NUM_CS],
    pub board: [c_char; 15],
    pub sum: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pismo_mem {
    pub base: phys_addr_t,
    pub size: u32,
    pub access: u16,
    pub width: u8,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pismo_data {
    pub client: *mut i2c_client,
    pub int): *mut *mut *mut void (vpp)(void ,,
    pub vpp_data: *mut c_void,
    pub dev: [*mut platform_device; PISMO_NUM_CS],
}

#[no_mangle]
unsafe extern "C" fn pismo_set_vpp(pdev: *mut platform_device, on: c_int) {
    static void pismo_set_vpp(struct platform_device *pdev, int on)
    {
    struct i2c_client *client = to_i2c_client(pdev.dev.parent);
    struct pismo_data *pismo = i2c_get_clientdata(client);
    pismo.vpp(pismo.vpp_data, on);
    }
#[no_mangle]
unsafe extern "C" fn pismo_width_to_bytes(width: c_uint) -> c_uint {
    static unsigned int pismo_width_to_bytes(unsigned int width)
    {
    width &= 15;
    if (width > 2)
    return 0;
    return 1 << width;
    }
    static int pismo_eeprom_read(struct i2c_client *client, void *buf, u8 addr,
    size_t size)
    {
    int ret;
    struct i2c_msg msg[] = {
    {
    .addr = client.addr,
    .len = sizeof(addr),
    .buf = &addr,
    }, {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = size,
    .buf = buf,
    },
    };
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    let mut ret: return = = ARRAY_SIZE(msg) ? size : -EIO;
    }
    static int pismo_add_device(struct pismo_data *pismo, int i,
    struct pismo_mem *region, const char *name,
    void *pdata, size_t psize)
    {
    struct platform_device *dev;
    let mut res: resource = { };
    let mut base: phys_addr_t = region.base;
    int ret;
    if (base == ~0)
    return -ENXIO;
    res.start = base;
    res.end = base + region.size - 1;
    res.flags = IORESOURCE_MEM;
    dev = platform_device_alloc(name, i);
    if (!dev)
    return -ENOMEM;
    dev.dev.parent = &pismo.client.dev;
    do {
    ret = platform_device_add_resources(dev, &res, 1);
    if (ret)
    break;
    ret = platform_device_add_data(dev, pdata, psize);
    if (ret)
    break;
    ret = platform_device_add(dev);
    if (ret)
    break;
    pismo.dev[i] = dev;
    return 0;
    } while (0);
    platform_device_put(dev);
    return ret;
    }
    static int pismo_add_nor(struct pismo_data *pismo, int i,
    struct pismo_mem *region)
    {
    struct physmap_flash_data data = {
    .width = region.width,
    };
    if (pismo.vpp)
    data.set_vpp = pismo_set_vpp;
    return pismo_add_device(pismo, i, region, "physmap-flash",
    &data, sizeof(data));
    }
    static int pismo_add_sram(struct pismo_data *pismo, int i,
    struct pismo_mem *region)
    {
    struct platdata_mtd_ram data = {
    .bankwidth = region.width,
    };
    return pismo_add_device(pismo, i, region, "mtd-ram",
    &data, sizeof(data));
    }
    static void pismo_add_one(struct pismo_data *pismo, int i,
    const struct pismo_cs_block *cs, phys_addr_t base)
    {
    struct device *dev = &pismo.client.dev;
    struct pismo_mem region;
    region.base = base;
    region.type = cs.type;
    region.width = pismo_width_to_bytes(cs.width);
    region.access = le16_to_cpu(cs.access);
    region.size = le32_to_cpu(cs.size);
    if (region.width == 0) {
    dev_err(dev, "cs%u: bad width: %02x, ignoring\n", i, cs.width);
    return;
    }
//
// FIXME: may need to the platforms memory controller here, but at
// the moment we assume that it has already been correctly setup.
// The memory controller can also tell us the base address as well.
//
    dev_info(dev, "cs%u: %.32s: type %02x access %u00ps size %uK\n",
    i, cs.device, region.type, region.access, region.size / 1024);
    switch (region.type) {
    case 0:
    break;
    case 1:
// static DOC
    break;
    case 2:
// static NOR
    pismo_add_nor(pismo, i, &region);
    break;
    case 3:
// static RAM
    pismo_add_sram(pismo, i, &region);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn pismo_remove(client: *mut i2c_client) {
    static void pismo_remove(struct i2c_client *client)
    {
    struct pismo_data *pismo = i2c_get_clientdata(client);
    int i;
    for (i = 0; i < ARRAY_SIZE(pismo.dev); i++)
    platform_device_unregister(pismo.dev[i]);
    kfree(pismo);
    }
#[no_mangle]
unsafe extern "C" fn pismo_probe(client: *mut i2c_client) -> c_int {
    static int pismo_probe(struct i2c_client *client)
    {
    struct pismo_pdata *pdata = client.dev.platform_data;
    struct pismo_eeprom eeprom;
    struct pismo_data *pismo;
    int ret, i;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "functionality mismatch\n");
    return -EIO;
    }
    pismo = kzalloc_obj(*pismo);
    if (!pismo)
    return -ENOMEM;
    pismo.client = client;
    if (pdata) {
    pismo.vpp = pdata.set_vpp;
    pismo.vpp_data = pdata.vpp_data;
    }
    i2c_set_clientdata(client, pismo);
    ret = pismo_eeprom_read(client, &eeprom, 0, sizeof(eeprom));
    if (ret < 0) {
    dev_err(&client.dev, "error reading EEPROM: %d\n", ret);
    goto exit_free;
    }
    dev_info(&client.dev, "%.15s board found\n", eeprom.board);
    for (i = 0; i < ARRAY_SIZE(eeprom.cs); i++)
    if (eeprom.cs[i].type != 0xff)
    pismo_add_one(pismo, i, &eeprom.cs[i],
    pdata.cs_addrs[i]);
    return 0;
    exit_free:
    kfree(pismo);
    return ret;
    }
    static const struct i2c_device_id pismo_id[] = {
    { "pismo" },
    { },
    };
    MODULE_DEVICE_TABLE(i2c, pismo_id);
    static struct i2c_driver pismo_driver = {
    .driver	= {
    .name	= "pismo",
    },
    .probe		= pismo_probe,
    .remove		= pismo_remove,
    .id_table	= pismo_id,
    };
#[no_mangle]
unsafe extern "C" fn pismo_init() -> int __init {
    static int __init pismo_init(void)
    {
    BUILD_BUG_ON(sizeof(struct pismo_cs_block) != 48);
    BUILD_BUG_ON(sizeof(struct pismo_eeprom) != 256);
    return i2c_add_driver(&pismo_driver);
    }
    module_init(pismo_init);
#[no_mangle]
unsafe extern "C" fn pismo_exit() -> void __exit {
    static void __exit pismo_exit(void)
    {
    i2c_del_driver(&pismo_driver);
    }
    module_exit(pismo_exit);
    MODULE_AUTHOR("Russell King <linux@arm.linux.org.uk>");
    MODULE_DESCRIPTION("PISMO memory driver");
    MODULE_LICENSE("GPL");
