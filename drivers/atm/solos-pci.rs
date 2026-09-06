//! Automatically rewritten from C to Rust
//! Source: drivers/atm/solos-pci.c
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
// Driver for the Solos PCI ADSL2+ card, designed to support Linux by
// Traverse Technologies -- https://www.traverse.com.au
// Xrio Limited          -- http://www.xrio.com
//
// Copyright © 2008 Traverse Technologies
// Copyright © 2008 Intel Corporation
//
// Authors: Nathan Williams <nathan@traverse.com.au>
// David Woodhouse <dwmw2@infradead.org>
// Treker Chen <treker@xrio.com>
//
// Macro flag: #define DEBUG
// Macro flag: #define VERBOSE_DEBUG

pub const DRIVER_VERSION: c_uint = 0x01;

pub const CONFIG_RAM_SIZE: c_int = 128;
pub const FLAGS_ADDR: c_uint = 0x7C;
pub const IRQ_EN_ADDR: c_uint = 0x78;
pub const FPGA_VER: c_uint = 0x74;
pub const IRQ_CLEAR: c_uint = 0x70;
pub const WRITE_FLASH: c_uint = 0x6C;
pub const PORTS: c_uint = 0x68;
pub const FLASH_BLOCK: c_uint = 0x64;
pub const FLASH_BUSY: c_uint = 0x60;
pub const FPGA_MODE: c_uint = 0x5C;
pub const FLASH_MODE: c_uint = 0x58;
pub const GPIO_STATUS: c_uint = 0x54;
pub const DRIVER_VER: c_uint = 0x50;

pub const DATA_RAM_SIZE: c_int = 32768;
pub const BUF_SIZE: c_int = 2048;

// Old boards use ATMEL AD45DB161D flash

// Current boards use M25P/M25PE SPI flash

pub const RX_DMA_SIZE: c_int = 2048;

pub const LEGACY_BUFFERS: c_int = 2;
pub const DMA_SUPPORTED: c_int = 4;
    let mut reset: static int = 0;
    let mut atmdebug: static int = 0;
    let mut firmware_upgrade: static int = 0;
    let mut fpga_upgrade: static int = 0;
    let mut db_firmware_upgrade: static int = 0;
    let mut db_fpga_upgrade: static int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_hdr {
    pub size: __le16,
    pub vpi: __le16,
    pub vci: __le16,
    pub type: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solos_skb_cb {
    pub vcc: *mut atm_vcc,
    pub dma_addr: u32,
}

pub const PKT_DATA: c_int = 0;
pub const PKT_COMMAND: c_int = 1;
pub const PKT_POPEN: c_int = 3;
pub const PKT_PCLOSE: c_int = 4;
pub const PKT_STATUS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct solos_card {
    pub config_regs: *mut void __iomem,
    pub buffers: *mut void __iomem,
    pub nr_ports: c_int,
    pub tx_mask: c_int,
    pub dev: *mut pci_dev,
    pub atmdev: [*mut atm_dev; 4],
    pub tlet: tasklet_struct,
    pub tx_lock: spinlock_t,
    pub tx_queue_lock: spinlock_t,
    pub cli_queue_lock: spinlock_t,
    pub param_queue_lock: spinlock_t,
    pub param_queue: list_head,
    pub tx_queue: [sk_buff_head; 4],
    pub cli_queue: [sk_buff_head; 4],
    pub tx_skb: [*mut sk_buff; 4],
    pub rx_skb: [*mut sk_buff; 4],
    pub dma_bounce: *mut c_uchar,
    pub param_wq: wait_queue_head_t,
    pub fw_wq: wait_queue_head_t,
    pub using_dma: c_int,
    pub dma_alignment: c_int,
    pub fpga_version: c_int,
    pub buffer_size: c_int,
    pub atmel_flash: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solos_param {
    pub list: list_head,
    pub pid: pid_t,
    pub port: c_int,
    pub response: *mut sk_buff,
}

    MODULE_AUTHOR("Traverse Technologies <support@traverse.com.au>");
    MODULE_DESCRIPTION("Solos PCI driver");
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("solos-FPGA.bin");
    MODULE_FIRMWARE("solos-Firmware.bin");
    MODULE_FIRMWARE("solos-db-FPGA.bin");
    MODULE_PARM_DESC(reset, "Reset Solos chips on startup");
    MODULE_PARM_DESC(atmdebug, "Print ATM data");
    MODULE_PARM_DESC(firmware_upgrade, "Initiate Solos firmware upgrade");
    MODULE_PARM_DESC(fpga_upgrade, "Initiate FPGA upgrade");
    MODULE_PARM_DESC(db_firmware_upgrade, "Initiate daughter board Solos firmware upgrade");
    MODULE_PARM_DESC(db_fpga_upgrade, "Initiate daughter board FPGA upgrade");
    module_param(reset, int, 0444);
    module_param(atmdebug, int, 0644);
    module_param(firmware_upgrade, int, 0444);
    module_param(fpga_upgrade, int, 0444);
    module_param(db_firmware_upgrade, int, 0444);
    module_param(db_fpga_upgrade, int, 0444);
    static void fpga_queue(struct solos_card *card, int port, struct sk_buff *skb,
    struct atm_vcc *vcc);
    static uint32_t fpga_tx(struct solos_card *);
    static irqreturn_t solos_irq(int irq, void *dev_id);
    static struct atm_vcc* find_vcc(struct atm_dev *dev, short vpi, int vci);
    static int atm_init(struct solos_card *, struct device *);
    static void atm_remove(struct solos_card *);
    static int send_command(struct solos_card *card, int dev, const char *buf, size_t size);
    static void solos_bh(unsigned long);
    static int print_buffer(struct sk_buff *buf);
#[no_mangle]
pub unsafe extern "C" fn solos_pop(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    static inline void solos_pop(struct atm_vcc *vcc, struct sk_buff *skb)
    {
    if (vcc.pop)
    vcc.pop(vcc, skb);
    else
    dev_kfree_skb_any(skb);
    }
    static ssize_t solos_param_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct atm_dev *atmdev = container_of(dev, struct atm_dev, class_dev);
    struct solos_card *card = atmdev.dev_data;
    struct solos_param prm;
    struct sk_buff *skb;
    struct pkt_hdr *header;
    int buflen;
    buflen = strlen(attr.attr.name) + 10;
    skb = alloc_skb(sizeof(*header) + buflen, GFP_KERNEL);
    if (!skb) {
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in solos_param_show()\n");
    return -ENOMEM;
    }
    header = skb_put(skb, sizeof(*header));
    buflen = snprintf((void *)&header[1], buflen - 1,
    "L%05d\n%s\n", current.pid, attr.attr.name);
    skb_put(skb, buflen);
    header.size = cpu_to_le16(buflen);
    header.vpi = cpu_to_le16(0);
    header.vci = cpu_to_le16(0);
    header.type = cpu_to_le16(PKT_COMMAND);
    prm.pid = current.pid;
    prm.response = core::ptr::null_mut();
    prm.port = SOLOS_CHAN(atmdev);
    spin_lock_irq(&card.param_queue_lock);
    list_add(&prm.list, &card.param_queue);
    spin_unlock_irq(&card.param_queue_lock);
    fpga_queue(card, prm.port, skb, core::ptr::null_mut());
    wait_event_timeout(card.param_wq, prm.response, 5 * HZ);
    spin_lock_irq(&card.param_queue_lock);
    list_del(&prm.list);
    spin_unlock_irq(&card.param_queue_lock);
    if (!prm.response)
    return -EIO;
    buflen = prm.response.len;
    memcpy(buf, prm.response.data, buflen);
    kfree_skb(prm.response);
    return buflen;
    }
    static ssize_t solos_param_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct atm_dev *atmdev = container_of(dev, struct atm_dev, class_dev);
    struct solos_card *card = atmdev.dev_data;
    struct solos_param prm;
    struct sk_buff *skb;
    struct pkt_hdr *header;
    int buflen;
    ssize_t ret;
    buflen = strlen(attr.attr.name) + 11 + count;
    skb = alloc_skb(sizeof(*header) + buflen, GFP_KERNEL);
    if (!skb) {
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in solos_param_store()\n");
    return -ENOMEM;
    }
    header = skb_put(skb, sizeof(*header));
    buflen = snprintf((void *)&header[1], buflen - 1,
    "L%05d\n%s\n%s\n", current.pid, attr.attr.name, buf);
    skb_put(skb, buflen);
    header.size = cpu_to_le16(buflen);
    header.vpi = cpu_to_le16(0);
    header.vci = cpu_to_le16(0);
    header.type = cpu_to_le16(PKT_COMMAND);
    prm.pid = current.pid;
    prm.response = core::ptr::null_mut();
    prm.port = SOLOS_CHAN(atmdev);
    spin_lock_irq(&card.param_queue_lock);
    list_add(&prm.list, &card.param_queue);
    spin_unlock_irq(&card.param_queue_lock);
    fpga_queue(card, prm.port, skb, core::ptr::null_mut());
    wait_event_timeout(card.param_wq, prm.response, 5 * HZ);
    spin_lock_irq(&card.param_queue_lock);
    list_del(&prm.list);
    spin_unlock_irq(&card.param_queue_lock);
    skb = prm.response;
    if (!skb)
    return -EIO;
    buflen = skb.len;
// Sometimes it has a newline, sometimes it doesn't.
    if (skb.data[buflen - 1] == '\n')
    buflen--;
    if (buflen == 2 && !strncmp(skb.data, "OK", 2))
    ret = count;
#[no_mangle]
pub unsafe extern "C" fn if(!strncmp(skb->data: buflen == 5 &&, _arg: "ERROR", _arg: 5)) -> else {
    else if (buflen == 5 && !strncmp(skb.data, "ERROR", 5))
    ret = -EIO;
    else {
// We know we have enough space allocated for this; we allocated
    it ourselves */
    skb.data[buflen] = 0;
    dev_warn(&card.dev.dev, "Unexpected parameter response: '%s'\n",
    skb.data);
    ret = -EIO;
    }
    kfree_skb(skb);
    return ret;
    }
    static char *next_string(struct sk_buff *skb)
    {
    let mut i: c_int = 0;
    char *this = skb.data;
    for (i = 0; i < skb.len; i++) {
    if (this[i] == '\n') {
    this[i] = 0;
    skb_pull(skb, i + 1);
    return this;
    }
    if (!isprint(this[i]))
    return core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
//
// Status packet has fields separated by \n, starting with a version number
// for the information therein. Fields are....
//
// packet version
// RxBitRate	(version >= 1)
// TxBitRate	(version >= 1)
// State		(version >= 1)
// LocalSNRMargin	(version >= 1)
// LocalLineAttn	(version >= 1)
//
#[no_mangle]
unsafe extern "C" fn process_status(card: *mut solos_card, port: c_int, skb: *mut sk_buff) -> c_int {
    static int process_status(struct solos_card *card, int port, struct sk_buff *skb)
    {
    char *str, *state_str, *snr, *attn;
    int ver, rate_up, rate_down, err;
    if (!card.atmdev[port])
    return -ENODEV;
    str = next_string(skb);
    if (!str)
    return -EIO;
    err = kstrtoint(str, 10, &ver);
    if (err) {
    dev_warn(&card.dev.dev, "Unexpected status interrupt version\n");
    return err;
    }
    if (ver < 1) {
    dev_warn(&card.dev.dev, "Unexpected status interrupt version %d\n",
    ver);
    return -EIO;
    }
    str = next_string(skb);
    if (!str)
    return -EIO;
    if (!strcmp(str, "ERROR")) {
    dev_dbg(&card.dev.dev, "Status packet indicated Solos error on port %d (starting up?)\n",
    port);
    return 0;
    }
    err = kstrtoint(str, 10, &rate_down);
    if (err)
    return err;
    str = next_string(skb);
    if (!str)
    return -EIO;
    err = kstrtoint(str, 10, &rate_up);
    if (err)
    return err;
    state_str = next_string(skb);
    if (!state_str)
    return -EIO;
// Anything but 'Showtime' is down
    if (strcmp(state_str, "Showtime")) {
    atm_dev_signal_change(card.atmdev[port], ATM_PHY_SIG_LOST);
    dev_info(&card.dev.dev, "Port %d: %s\n", port, state_str);
    return 0;
    }
    snr = next_string(skb);
    if (!snr)
    return -EIO;
    attn = next_string(skb);
    if (!attn)
    return -EIO;
    dev_info(&card.dev.dev, "Port %d: %s @%d/%d kb/s%s%s%s%s\n",
    port, state_str, rate_down/1000, rate_up/1000,
    snr[0]?", SNR ":"", snr, attn[0]?", Attn ":"", attn);
    card.atmdev[port].link_rate = rate_down / 424;
    atm_dev_signal_change(card.atmdev[port], ATM_PHY_SIG_FOUND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn process_command(card: *mut solos_card, port: c_int, skb: *mut sk_buff) -> c_int {
    static int process_command(struct solos_card *card, int port, struct sk_buff *skb)
    {
    struct solos_param *prm;
    unsigned long flags;
    int cmdpid;
    let mut found: c_int = 0, err;
    if (skb.len < 7)
    return 0;
    if (skb.data[0] != 'L'    || !isdigit(skb.data[1]) ||
    !isdigit(skb.data[2]) || !isdigit(skb.data[3]) ||
    !isdigit(skb.data[4]) || !isdigit(skb.data[5]) ||
    skb.data[6] != '\n')
    return 0;
    err = kstrtoint(&skb.data[1], 10, &cmdpid);
    if (err)
    return err;
    spin_lock_irqsave(&card.param_queue_lock, flags);
    list_for_each_entry(prm, &card.param_queue, list) {
    if (prm.port == port && prm.pid == cmdpid) {
    prm.response = skb;
    skb_pull(skb, 7);
    wake_up(&card.param_wq);
    found = 1;
    break;
    }
    }
    spin_unlock_irqrestore(&card.param_queue_lock, flags);
    return found;
    }
    static ssize_t console_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct atm_dev *atmdev = container_of(dev, struct atm_dev, class_dev);
    struct solos_card *card = atmdev.dev_data;
    struct sk_buff *skb;
    unsigned int len;
    spin_lock_bh(&card.cli_queue_lock);
    skb = skb_dequeue(&card.cli_queue[SOLOS_CHAN(atmdev)]);
    spin_unlock_bh(&card.cli_queue_lock);
    if(skb == core::ptr::null_mut())
    return sprintf(buf, "No data.\n");
    len = skb.len;
    memcpy(buf, skb.data, len);
    kfree_skb(skb);
    return len;
    }
#[no_mangle]
unsafe extern "C" fn send_command(card: *mut solos_card, dev: c_int, buf: *const c_char, size: usize) -> c_int {
    static int send_command(struct solos_card *card, int dev, const char *buf, size_t size)
    {
    struct sk_buff *skb;
    struct pkt_hdr *header;
    if (size > (BUF_SIZE - sizeof(*header))) {
    dev_dbg(&card.dev.dev, "Command is too big.  Dropping request\n");
    return 0;
    }
    skb = alloc_skb(size + sizeof(*header), GFP_ATOMIC);
    if (!skb) {
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in send_command()\n");
    return 0;
    }
    header = skb_put(skb, sizeof(*header));
    header.size = cpu_to_le16(size);
    header.vpi = cpu_to_le16(0);
    header.vci = cpu_to_le16(0);
    header.type = cpu_to_le16(PKT_COMMAND);
    skb_put_data(skb, buf, size);
    fpga_queue(card, dev, skb, core::ptr::null_mut());
    return 0;
    }
    static ssize_t console_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct atm_dev *atmdev = container_of(dev, struct atm_dev, class_dev);
    struct solos_card *card = atmdev.dev_data;
    int err;
    err = send_command(card, SOLOS_CHAN(atmdev), buf, count);
    return err?:count;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geos_gpio_attr {
    pub attr: device_attribute,
    pub offset: c_int,
}

    struct geos_gpio_attr gpio_attr_##_name = {		\
    .attr = __ATTR(_name, _mode, _show, _store),	\
    .offset = _offset }
    static ssize_t geos_gpio_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct geos_gpio_attr *gattr = container_of(attr, struct geos_gpio_attr, attr);
    struct solos_card *card = dev_get_drvdata(dev);
    uint32_t data32;
    if (count != 1 && (count != 2 || buf[1] != '\n'))
    return -EINVAL;
    spin_lock_irq(&card.param_queue_lock);
    data32 = ioread32(card.config_regs + GPIO_STATUS);
    if (buf[0] == '1') {
    data32 |= 1 << gattr.offset;
    iowrite32(data32, card.config_regs + GPIO_STATUS);
    } else if (buf[0] == '0') {
    data32 &= ~(1 << gattr.offset);
    iowrite32(data32, card.config_regs + GPIO_STATUS);
    } else {
    count = -EINVAL;
    }
    spin_unlock_irq(&card.param_queue_lock);
    return count;
    }
    static ssize_t geos_gpio_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct geos_gpio_attr *gattr = container_of(attr, struct geos_gpio_attr, attr);
    struct solos_card *card = dev_get_drvdata(dev);
    uint32_t data32;
    data32 = ioread32(card.config_regs + GPIO_STATUS);
    data32 = (data32 >> gattr.offset) & 1;
    return sprintf(buf, "%d\n", data32);
    }
    static ssize_t hardware_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct geos_gpio_attr *gattr = container_of(attr, struct geos_gpio_attr, attr);
    struct solos_card *card = dev_get_drvdata(dev);
    uint32_t data32;
    data32 = ioread32(card.config_regs + GPIO_STATUS);
    switch (gattr.offset) {
    case 0:
// HardwareVersion
    data32 = data32 & 0x1F;
    break;
    case 1:
// HardwareVariant
    data32 = (data32 >> 5) & 0x0F;
    break;
    }
    return sprintf(buf, "%d\n", data32);
    }
    static DEVICE_ATTR_RW(console);

    static SOLOS_GPIO_ATTR(GPIO1, 0644, geos_gpio_show, geos_gpio_store, 9);
    static SOLOS_GPIO_ATTR(GPIO2, 0644, geos_gpio_show, geos_gpio_store, 10);
    static SOLOS_GPIO_ATTR(GPIO3, 0644, geos_gpio_show, geos_gpio_store, 11);
    static SOLOS_GPIO_ATTR(GPIO4, 0644, geos_gpio_show, geos_gpio_store, 12);
    static SOLOS_GPIO_ATTR(GPIO5, 0644, geos_gpio_show, geos_gpio_store, 13);
    static SOLOS_GPIO_ATTR(PushButton, 0444, geos_gpio_show, core::ptr::null_mut(), 14);
    static SOLOS_GPIO_ATTR(HardwareVersion, 0444, hardware_show, core::ptr::null_mut(), 0);
    static SOLOS_GPIO_ATTR(HardwareVariant, 0444, hardware_show, core::ptr::null_mut(), 1);

    static struct attribute *solos_attrs[] = {

    core::ptr::null_mut()
    };
    static const struct attribute_group solos_attr_group = {
    .attrs = solos_attrs,
    .name = "parameters",
    };
    static struct attribute *gpio_attrs[] = {
    &gpio_attr_GPIO1.attr.attr,
    &gpio_attr_GPIO2.attr.attr,
    &gpio_attr_GPIO3.attr.attr,
    &gpio_attr_GPIO4.attr.attr,
    &gpio_attr_GPIO5.attr.attr,
    &gpio_attr_PushButton.attr.attr,
    &gpio_attr_HardwareVersion.attr.attr,
    &gpio_attr_HardwareVariant.attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group gpio_attr_group = {
    .attrs = gpio_attrs,
    .name = "gpio",
    };
#[no_mangle]
unsafe extern "C" fn flash_upgrade(card: *mut solos_card, chip: c_int) -> c_int {
    static int flash_upgrade(struct solos_card *card, int chip)
    {
    const struct firmware *fw;
    const char *fw_name;
    let mut blocksize: c_int = 0;
    let mut numblocks: c_int = 0;
    int offset;
    switch (chip) {
    case 0:
    fw_name = "solos-FPGA.bin";
    if (card.atmel_flash)
    blocksize = ATMEL_FPGA_BLOCK;
    else
    blocksize = SPI_FLASH_BLOCK;
    break;
    case 1:
    fw_name = "solos-Firmware.bin";
    if (card.atmel_flash)
    blocksize = ATMEL_SOLOS_BLOCK;
    else
    blocksize = SPI_FLASH_BLOCK;
    break;
    case 2:
    if (card.fpga_version > LEGACY_BUFFERS){
    fw_name = "solos-db-FPGA.bin";
    if (card.atmel_flash)
    blocksize = ATMEL_FPGA_BLOCK;
    else
    blocksize = SPI_FLASH_BLOCK;
    } else {
    dev_info(&card.dev.dev, "FPGA version doesn't support"
    " daughter board upgrades\n");
    return -EPERM;
    }
    break;
    case 3:
    if (card.fpga_version > LEGACY_BUFFERS){
    fw_name = "solos-Firmware.bin";
    if (card.atmel_flash)
    blocksize = ATMEL_SOLOS_BLOCK;
    else
    blocksize = SPI_FLASH_BLOCK;
    } else {
    dev_info(&card.dev.dev, "FPGA version doesn't support"
    " daughter board upgrades\n");
    return -EPERM;
    }
    break;
    default:
    return -ENODEV;
    }
    if (request_firmware(&fw, fw_name, &card.dev.dev))
    return -ENOENT;
    dev_info(&card.dev.dev, "Flash upgrade starting\n");
// New FPGAs require driver version before permitting flash upgrades
    iowrite32(DRIVER_VERSION, card.config_regs + DRIVER_VER);
    numblocks = fw.size / blocksize;
    dev_info(&card.dev.dev, "Firmware size: %zd\n", fw.size);
    dev_info(&card.dev.dev, "Number of blocks: %d\n", numblocks);
    dev_info(&card.dev.dev, "Changing FPGA to Update mode\n");
    iowrite32(1, card.config_regs + FPGA_MODE);
    (void) ioread32(card.config_regs + FPGA_MODE);
// Set mode to Chip Erase
    if(chip == 0 || chip == 2)
    dev_info(&card.dev.dev, "Set FPGA Flash mode to FPGA Chip Erase\n");
    if(chip == 1 || chip == 3)
    dev_info(&card.dev.dev, "Set FPGA Flash mode to Solos Chip Erase\n");
    iowrite32((chip * 2), card.config_regs + FLASH_MODE);
    iowrite32(1, card.config_regs + WRITE_FLASH);
    wait_event(card.fw_wq, !ioread32(card.config_regs + FLASH_BUSY));
    for (offset = 0; offset < fw.size; offset += blocksize) {
    int i;
// Clear write flag
    iowrite32(0, card.config_regs + WRITE_FLASH);
// Set mode to Block Write
// dev_info(&card->dev->dev, "Set FPGA Flash mode to Block Write\n");
    iowrite32(((chip * 2) + 1), card.config_regs + FLASH_MODE);
// Copy block to buffer, swapping each 16 bits for Atmel flash
    for(i = 0; i < blocksize; i += 4) {
    uint32_t word;
    if (card.atmel_flash)
    word = swahb32p((uint32_t *)(fw.data + offset + i));
    else
    word = *(uint32_t *)(fw.data + offset + i);
    if(card.fpga_version > LEGACY_BUFFERS)
    iowrite32(word, FLASH_BUF + i);
    else
    iowrite32(word, RX_BUF(card, 3) + i);
    }
// Specify block number and then trigger flash write
    iowrite32(offset / blocksize, card.config_regs + FLASH_BLOCK);
    iowrite32(1, card.config_regs + WRITE_FLASH);
    wait_event(card.fw_wq, !ioread32(card.config_regs + FLASH_BUSY));
    }
    release_firmware(fw);
    iowrite32(0, card.config_regs + WRITE_FLASH);
    iowrite32(0, card.config_regs + FPGA_MODE);
    iowrite32(0, card.config_regs + FLASH_MODE);
    dev_info(&card.dev.dev, "Returning FPGA to Data mode\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn solos_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t solos_irq(int irq, void *dev_id)
    {
    struct solos_card *card = dev_id;
    let mut handled: c_int = 1;
    iowrite32(0, card.config_regs + IRQ_CLEAR);
// If we're up and running, just kick the tasklet to process TX/RX
    if (card.atmdev[0])
    tasklet_schedule(&card.tlet);
    else
    wake_up(&card.fw_wq);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn solos_bh(card_arg: c_ulong) {
    static void solos_bh(unsigned long card_arg)
    {
    struct solos_card *card = (void *)card_arg;
    uint32_t card_flags;
    let mut rx_done: u32 = 0;
    int port;
//
// Since fpga_tx() is going to need to read the flags under its lock,
// it can return them to us so that we don't have to hit PCI MMIO
// again for the same information
//
    card_flags = fpga_tx(card);
    for (port = 0; port < card.nr_ports; port++) {
    if (card_flags & (0x10 << port)) {
    struct pkt_hdr _hdr, *header;
    struct sk_buff *skb;
    struct atm_vcc *vcc;
    int size;
    if (card.using_dma) {
    skb = card.rx_skb[port];
    card.rx_skb[port] = core::ptr::null_mut();
    dma_unmap_single(&card.dev.dev, SKB_CB(skb).dma_addr,
    RX_DMA_SIZE, DMA_FROM_DEVICE);
    header = (void *)skb.data;
    size = le16_to_cpu(header.size);
    skb_put(skb, size + sizeof(*header));
    skb_pull(skb, sizeof(*header));
    } else {
    header = &_hdr;
    rx_done |= 0x10 << port;
    memcpy_fromio(header, RX_BUF(card, port), sizeof(*header));
    size = le16_to_cpu(header.size);
    if (size > (card.buffer_size - sizeof(*header))){
    dev_warn(&card.dev.dev, "Invalid buffer size\n");
    continue;
    }
// Use netdev_alloc_skb() because it adds NET_SKB_PAD of
// headroom, and ensures we can route packets back out an
// Ethernet interface (for example) without having to
// reallocate. Adding NET_IP_ALIGN also ensures that both
// PPPoATM and PPPoEoBR2684 packets end up aligned.
    skb = netdev_alloc_skb_ip_align(core::ptr::null_mut(), size + 1);
    if (!skb) {
    if (net_ratelimit())
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff for RX\n");
    continue;
    }
    memcpy_fromio(skb_put(skb, size),
    RX_BUF(card, port) + sizeof(*header),
    size);
    }
    if (atmdebug) {
    dev_info(&card.dev.dev, "Received: port %d\n", port);
    dev_info(&card.dev.dev, "size: %d VPI: %d VCI: %d\n",
    size, le16_to_cpu(header.vpi),
    le16_to_cpu(header.vci));
    print_buffer(skb);
    }
    switch (le16_to_cpu(header.type)) {
    case PKT_DATA:
    vcc = find_vcc(card.atmdev[port], le16_to_cpu(header.vpi),
    le16_to_cpu(header.vci));
    if (!vcc) {
    if (net_ratelimit())
    dev_warn(&card.dev.dev, "Received packet for unknown VPI.VCI %d.%d on port %d\n",
    le16_to_cpu(header.vpi), le16_to_cpu(header.vci),
    port);
    dev_kfree_skb_any(skb);
    break;
    }
    atm_charge(vcc, skb.truesize);
    vcc.push(vcc, skb);
    atomic_inc(&vcc.stats.rx);
    break;
    case PKT_STATUS:
    if (process_status(card, port, skb) &&
    net_ratelimit()) {
    dev_warn(&card.dev.dev, "Bad status packet of %d bytes on port %d:\n", skb.len, port);
    print_buffer(skb);
    }
    dev_kfree_skb_any(skb);
    break;
    case PKT_COMMAND:
    default: /* FIXME: Not really, surely? */
    if (process_command(card, port, skb))
    break;
    spin_lock(&card.cli_queue_lock);
    if (skb_queue_len(&card.cli_queue[port]) > 10) {
    if (net_ratelimit())
    dev_warn(&card.dev.dev, "Dropping console response on port %d\n",
    port);
    dev_kfree_skb_any(skb);
    } else
    skb_queue_tail(&card.cli_queue[port], skb);
    spin_unlock(&card.cli_queue_lock);
    break;
    }
    }
// Allocate RX skbs for any ports which need them
    if (card.using_dma && card.atmdev[port] &&
    !card.rx_skb[port]) {
// Unlike the MMIO case (qv) we can't add NET_IP_ALIGN
// here; the FPGA can only DMA to addresses which are
// aligned to 4 bytes.
    struct sk_buff *skb = dev_alloc_skb(RX_DMA_SIZE);
    if (skb) {
    SKB_CB(skb).dma_addr =
    dma_map_single(&card.dev.dev, skb.data,
    RX_DMA_SIZE, DMA_FROM_DEVICE);
    iowrite32(SKB_CB(skb).dma_addr,
    card.config_regs + RX_DMA_ADDR(port));
    card.rx_skb[port] = skb;
    } else {
    if (net_ratelimit())
    dev_warn(&card.dev.dev, "Failed to allocate RX skb");
// We'll have to try again later
    tasklet_schedule(&card.tlet);
    }
    }
    }
    if (rx_done)
    iowrite32(rx_done, card.config_regs + FLAGS_ADDR);
    return;
    }
    static struct atm_vcc *find_vcc(struct atm_dev *dev, short vpi, int vci)
    {
    struct hlist_head *head;
    struct atm_vcc *vcc = core::ptr::null_mut();
    struct sock *s;
    read_lock(&vcc_sklist_lock);
    head = &vcc_hash[vci & (VCC_HTABLE_SIZE -1)];
    sk_for_each(s, head) {
    vcc = atm_sk(s);
    if (vcc.dev == dev && vcc.vci == vci &&
    vcc.vpi == vpi && vcc.qos.rxtp.traffic_class != ATM_NONE &&
    test_bit(ATM_VF_READY, &vcc.flags))
    goto out;
    }
    vcc = core::ptr::null_mut();
    out:
    read_unlock(&vcc_sklist_lock);
    return vcc;
    }
#[no_mangle]
unsafe extern "C" fn popen(vcc: *mut atm_vcc) -> c_int {
    static int popen(struct atm_vcc *vcc)
    {
    struct solos_card *card = vcc.dev.dev_data;
    struct sk_buff *skb;
    struct pkt_hdr *header;
    if (vcc.qos.aal != ATM_AAL5) {
    dev_warn(&card.dev.dev, "Unsupported ATM type %d\n",
    vcc.qos.aal);
    return -EINVAL;
    }
    skb = alloc_skb(sizeof(*header), GFP_KERNEL);
    if (!skb) {
    if (net_ratelimit())
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in popen()\n");
    return -ENOMEM;
    }
    header = skb_put(skb, sizeof(*header));
    header.size = cpu_to_le16(0);
    header.vpi = cpu_to_le16(vcc.vpi);
    header.vci = cpu_to_le16(vcc.vci);
    header.type = cpu_to_le16(PKT_POPEN);
    fpga_queue(card, SOLOS_CHAN(vcc.dev), skb, core::ptr::null_mut());
    set_bit(ATM_VF_ADDR, &vcc.flags);
    set_bit(ATM_VF_READY, &vcc.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pclose(vcc: *mut atm_vcc) {
    static void pclose(struct atm_vcc *vcc)
    {
    struct solos_card *card = vcc.dev.dev_data;
    let mut port: c_uchar = SOLOS_CHAN(vcc.dev);
    struct sk_buff *skb, *tmpskb;
    struct pkt_hdr *header;
// Remove any yet-to-be-transmitted packets from the pending queue
    spin_lock_bh(&card.tx_queue_lock);
    skb_queue_walk_safe(&card.tx_queue[port], skb, tmpskb) {
    if (SKB_CB(skb).vcc == vcc) {
    skb_unlink(skb, &card.tx_queue[port]);
    solos_pop(vcc, skb);
    }
    }
    spin_unlock_bh(&card.tx_queue_lock);
    skb = alloc_skb(sizeof(*header), GFP_KERNEL);
    if (!skb) {
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in pclose()\n");
    return;
    }
    header = skb_put(skb, sizeof(*header));
    header.size = cpu_to_le16(0);
    header.vpi = cpu_to_le16(vcc.vpi);
    header.vci = cpu_to_le16(vcc.vci);
    header.type = cpu_to_le16(PKT_PCLOSE);
    skb_get(skb);
    fpga_queue(card, port, skb, core::ptr::null_mut());
    if (!wait_event_timeout(card.param_wq, !skb_shared(skb), 5 * HZ))
    dev_warn(&card.dev.dev,
    "Timeout waiting for VCC close on port %d\n", port);
    dev_kfree_skb(skb);
// Hold up vcc_destroy_socket() (our caller) until solos_bh() in the
    tasklet has finished processing any incoming packets (and, more to
    the point, using the vcc pointer). */
    tasklet_unlock_wait(&card.tlet);
    clear_bit(ATM_VF_ADDR, &vcc.flags);
    return;
    }
#[no_mangle]
unsafe extern "C" fn print_buffer(buf: *mut sk_buff) -> c_int {
    static int print_buffer(struct sk_buff *buf)
    {
    int len,i;
    char msg[500];
    char item[10];
    len = buf.len;
    for (i = 0; i < len; i++){
    if(i % 8 == 0)
    sprintf(msg, "%02X: ", i);
    sprintf(item,"%02X ",*(buf.data + i));
    strcat(msg, item);
    if(i % 8 == 7) {
    sprintf(item, "\n");
    strcat(msg, item);
    printk(KERN_DEBUG "%s", msg);
    }
    }
    if (i % 8 != 0) {
    sprintf(item, "\n");
    strcat(msg, item);
    printk(KERN_DEBUG "%s", msg);
    }
    printk(KERN_DEBUG "\n");
    return 0;
    }
    static void fpga_queue(struct solos_card *card, int port, struct sk_buff *skb,
    struct atm_vcc *vcc)
    {
    int old_len;
    unsigned long flags;
    SKB_CB(skb).vcc = vcc;
    spin_lock_irqsave(&card.tx_queue_lock, flags);
    old_len = skb_queue_len(&card.tx_queue[port]);
    skb_queue_tail(&card.tx_queue[port], skb);
    if (!old_len)
    card.tx_mask |= (1 << port);
    spin_unlock_irqrestore(&card.tx_queue_lock, flags);
// Theoretically we could just schedule the tasklet here, but
    that introduces latency we don't want -- it's noticeable */
    if (!old_len)
    fpga_tx(card);
    }
#[no_mangle]
unsafe extern "C" fn fpga_tx(card: *mut solos_card) -> u32 {
    static uint32_t fpga_tx(struct solos_card *card)
    {
    uint32_t tx_pending, card_flags;
    let mut tx_started: u32 = 0;
    struct sk_buff *skb;
    struct atm_vcc *vcc;
    unsigned char port;
    unsigned long flags;
    spin_lock_irqsave(&card.tx_lock, flags);
    card_flags = ioread32(card.config_regs + FLAGS_ADDR);
//
// The queue lock is required for _writing_ to tx_mask, but we're
// OK to read it here without locking. The only potential update
// that we could race with is in fpga_queue() where it sets a bit
// for a new port... but it's going to call this function again if
// it's doing that, anyway.
//
    tx_pending = card.tx_mask & ~card_flags;
    for (port = 0; tx_pending; tx_pending >>= 1, port++) {
    if (tx_pending & 1) {
    struct sk_buff *oldskb = card.tx_skb[port];
    if (oldskb) {
    dma_unmap_single(&card.dev.dev, SKB_CB(oldskb).dma_addr,
    oldskb.len, DMA_TO_DEVICE);
    card.tx_skb[port] = core::ptr::null_mut();
    }
    spin_lock(&card.tx_queue_lock);
    skb = skb_dequeue(&card.tx_queue[port]);
    if (!skb)
    card.tx_mask &= ~(1 << port);
    spin_unlock(&card.tx_queue_lock);
    if (skb && !card.using_dma) {
    memcpy_toio(TX_BUF(card, port), skb.data, skb.len);
    tx_started |= 1 << port;
    oldskb = skb; /* We're done with this skb already */
    } else if (skb && card.using_dma) {
    unsigned char *data = skb.data;
    if ((unsigned long)data & card.dma_alignment) {
    data = card.dma_bounce + (BUF_SIZE * port);
    memcpy(data, skb.data, skb.len);
    }
    SKB_CB(skb).dma_addr = dma_map_single(&card.dev.dev, data,
    skb.len, DMA_TO_DEVICE);
    card.tx_skb[port] = skb;
    iowrite32(SKB_CB(skb).dma_addr,
    card.config_regs + TX_DMA_ADDR(port));
    }
    if (!oldskb)
    continue;
// Clean up and free oldskb now it's gone
    if (atmdebug) {
    struct pkt_hdr *header = (void *)oldskb.data;
    let mut size: c_int = le16_to_cpu(header.size);
    skb_pull(oldskb, sizeof(*header));
    dev_info(&card.dev.dev, "Transmitted: port %d\n",
    port);
    dev_info(&card.dev.dev, "size: %d VPI: %d VCI: %d\n",
    size, le16_to_cpu(header.vpi),
    le16_to_cpu(header.vci));
    print_buffer(oldskb);
    }
    vcc = SKB_CB(oldskb).vcc;
    if (vcc) {
    atomic_inc(&vcc.stats.tx);
    solos_pop(vcc, oldskb);
    } else {
    dev_kfree_skb_irq(oldskb);
    wake_up(&card.param_wq);
    }
    }
    }
// For non-DMA TX, write the 'TX start' bit for all four ports simultaneously
    if (tx_started)
    iowrite32(tx_started, card.config_regs + FLAGS_ADDR);
    spin_unlock_irqrestore(&card.tx_lock, flags);
    return card_flags;
    }
#[no_mangle]
unsafe extern "C" fn psend(vcc: *mut atm_vcc, skb: *mut sk_buff) -> c_int {
    static int psend(struct atm_vcc *vcc, struct sk_buff *skb)
    {
    struct solos_card *card = vcc.dev.dev_data;
    struct pkt_hdr *header;
    int pktlen;
    pktlen = skb.len;
    if (pktlen > (BUF_SIZE - sizeof(*header))) {
    dev_warn(&card.dev.dev, "Length of PDU is too large. Dropping PDU.\n");
    solos_pop(vcc, skb);
    return 0;
    }
    if (!skb_clone_writable(skb, sizeof(*header))) {
    let mut expand_by: c_int = 0;
    int ret;
    if (skb_headroom(skb) < sizeof(*header))
    expand_by = sizeof(*header) - skb_headroom(skb);
    ret = pskb_expand_head(skb, expand_by, 0, GFP_ATOMIC);
    if (ret) {
    dev_warn(&card.dev.dev, "pskb_expand_head failed.\n");
    solos_pop(vcc, skb);
    return ret;
    }
    }
    header = skb_push(skb, sizeof(*header));
// This does _not_ include the size of the header
    header.size = cpu_to_le16(pktlen);
    header.vpi = cpu_to_le16(vcc.vpi);
    header.vci = cpu_to_le16(vcc.vci);
    header.type = cpu_to_le16(PKT_DATA);
    fpga_queue(card, SOLOS_CHAN(vcc.dev), skb, vcc);
    return 0;
    }
    static const struct atmdev_ops fpga_ops = {
    .open =		popen,
    .close =	pclose,
    .ioctl =	core::ptr::null_mut(),
    .send =		psend,
    .proc_read =	core::ptr::null_mut(),
    .owner =	THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn fpga_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int fpga_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    int err;
    uint16_t fpga_ver;
    uint8_t major_ver, minor_ver;
    uint32_t data32;
    struct solos_card *card;
    card = kzalloc_obj(*card);
    if (!card)
    return -ENOMEM;
    card.dev = dev;
    init_waitqueue_head(&card.fw_wq);
    init_waitqueue_head(&card.param_wq);
    err = pci_enable_device(dev);
    if (err) {
    dev_warn(&dev.dev,  "Failed to enable PCI device\n");
    goto out;
    }
    err = dma_set_mask_and_coherent(&dev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_warn(&dev.dev, "Failed to set 32-bit DMA mask\n");
    goto out;
    }
    err = pci_request_regions(dev, "solos");
    if (err) {
    dev_warn(&dev.dev, "Failed to request regions\n");
    goto out;
    }
    card.config_regs = pci_iomap(dev, 0, CONFIG_RAM_SIZE);
    if (!card.config_regs) {
    dev_warn(&dev.dev, "Failed to ioremap config registers\n");
    err = -ENOMEM;
    goto out_release_regions;
    }
    card.buffers = pci_iomap(dev, 1, DATA_RAM_SIZE);
    if (!card.buffers) {
    dev_warn(&dev.dev, "Failed to ioremap data buffers\n");
    err = -ENOMEM;
    goto out_unmap_config;
    }
    if (reset) {
    iowrite32(1, card.config_regs + FPGA_MODE);
    ioread32(card.config_regs + FPGA_MODE);
    iowrite32(0, card.config_regs + FPGA_MODE);
    ioread32(card.config_regs + FPGA_MODE);
    }
    data32 = ioread32(card.config_regs + FPGA_VER);
    fpga_ver = (data32 & 0x0000FFFF);
    major_ver = ((data32 & 0xFF000000) >> 24);
    minor_ver = ((data32 & 0x00FF0000) >> 16);
    card.fpga_version = FPGA_VERSION(major_ver,minor_ver);
    if (card.fpga_version > LEGACY_BUFFERS)
    card.buffer_size = BUF_SIZE;
    else
    card.buffer_size = OLD_BUF_SIZE;
    dev_info(&dev.dev, "Solos FPGA Version %d.%02d svn-%d\n",
    major_ver, minor_ver, fpga_ver);
    if (fpga_ver < 37 && (fpga_upgrade || firmware_upgrade ||
    db_fpga_upgrade || db_firmware_upgrade)) {
    dev_warn(&dev.dev,
    "FPGA too old; cannot upgrade flash. Use JTAG.\n");
    fpga_upgrade = firmware_upgrade = 0;
    db_fpga_upgrade = db_firmware_upgrade = 0;
    }
// Stopped using Atmel flash after 0.03-38
    if (fpga_ver < 39)
    card.atmel_flash = 1;
    else
    card.atmel_flash = 0;
    data32 = ioread32(card.config_regs + PORTS);
    card.nr_ports = (data32 & 0x000000FF);
    if (card.fpga_version >= DMA_SUPPORTED) {
    pci_set_master(dev);
    card.using_dma = 1;
    if (1) { /* All known FPGA versions so far */
    card.dma_alignment = 3;
    card.dma_bounce = kmalloc_array(card.nr_ports,
    BUF_SIZE, GFP_KERNEL);
    if (!card.dma_bounce) {
    dev_warn(&card.dev.dev, "Failed to allocate DMA bounce buffers\n");
    err = -ENOMEM;
// Fallback to MMIO doesn't work
    goto out_unmap_both;
    }
    }
    } else {
    card.using_dma = 0;
// Set RX empty flag for all ports
    iowrite32(0xF0, card.config_regs + FLAGS_ADDR);
    }
    pci_set_drvdata(dev, card);
    tasklet_init(&card.tlet, solos_bh, (unsigned long)card);
    spin_lock_init(&card.tx_lock);
    spin_lock_init(&card.tx_queue_lock);
    spin_lock_init(&card.cli_queue_lock);
    spin_lock_init(&card.param_queue_lock);
    INIT_LIST_HEAD(&card.param_queue);
    err = request_irq(dev.irq, solos_irq, IRQF_SHARED,
    "solos-pci", card);
    if (err) {
    dev_dbg(&card.dev.dev, "Failed to request interrupt IRQ: %d\n", dev.irq);
    goto out_unmap_both;
    }
    iowrite32(1, card.config_regs + IRQ_EN_ADDR);
    if (fpga_upgrade)
    flash_upgrade(card, 0);
    if (firmware_upgrade)
    flash_upgrade(card, 1);
    if (db_fpga_upgrade)
    flash_upgrade(card, 2);
    if (db_firmware_upgrade)
    flash_upgrade(card, 3);
    err = atm_init(card, &dev.dev);
    if (err)
    goto out_free_irq;
    if (card.fpga_version >= DMA_SUPPORTED &&
    sysfs_create_group(&card.dev.dev.kobj, &gpio_attr_group))
    dev_err(&card.dev.dev, "Could not register parameter group for GPIOs\n");
    return 0;
    out_free_irq:
    iowrite32(0, card.config_regs + IRQ_EN_ADDR);
    free_irq(dev.irq, card);
    tasklet_kill(&card.tlet);
    out_unmap_both:
    kfree(card.dma_bounce);
    pci_iounmap(dev, card.buffers);
    out_unmap_config:
    pci_iounmap(dev, card.config_regs);
    out_release_regions:
    pci_release_regions(dev);
    out:
    kfree(card);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atm_init(card: *mut solos_card, parent: *mut device) -> c_int {
    static int atm_init(struct solos_card *card, struct device *parent)
    {
    int i;
    for (i = 0; i < card.nr_ports; i++) {
    struct sk_buff *skb;
    struct pkt_hdr *header;
    skb_queue_head_init(&card.tx_queue[i]);
    skb_queue_head_init(&card.cli_queue[i]);
    card.atmdev[i] = atm_dev_register("solos-pci", parent, &fpga_ops, -1, core::ptr::null_mut());
    if (!card.atmdev[i]) {
    dev_err(&card.dev.dev, "Could not register ATM device %d\n", i);
    atm_remove(card);
    return -ENODEV;
    }
    if (device_create_file(&card.atmdev[i].class_dev, &dev_attr_console))
    dev_err(&card.dev.dev, "Could not register console for ATM device %d\n", i);
    if (sysfs_create_group(&card.atmdev[i].class_dev.kobj, &solos_attr_group))
    dev_err(&card.dev.dev, "Could not register parameter group for ATM device %d\n", i);
    dev_info(&card.dev.dev, "Registered ATM device %d\n", card.atmdev[i].number);
    card.atmdev[i].ci_range.vpi_bits = 8;
    card.atmdev[i].ci_range.vci_bits = 16;
    card.atmdev[i].dev_data = card;
    card.atmdev[i].phy_data = (void *)(unsigned long)i;
    atm_dev_signal_change(card.atmdev[i], ATM_PHY_SIG_FOUND);
    skb = alloc_skb(sizeof(*header), GFP_KERNEL);
    if (!skb) {
    dev_warn(&card.dev.dev, "Failed to allocate sk_buff in atm_init()\n");
    continue;
    }
    header = skb_put(skb, sizeof(*header));
    header.size = cpu_to_le16(0);
    header.vpi = cpu_to_le16(0);
    header.vci = cpu_to_le16(0);
    header.type = cpu_to_le16(PKT_STATUS);
    fpga_queue(card, i, skb, core::ptr::null_mut());
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atm_remove(card: *mut solos_card) {
    static void atm_remove(struct solos_card *card)
    {
    int i;
    for (i = 0; i < card.nr_ports; i++) {
    if (card.atmdev[i]) {
    struct sk_buff *skb;
    dev_info(&card.dev.dev, "Unregistering ATM device %d\n", card.atmdev[i].number);
    sysfs_remove_group(&card.atmdev[i].class_dev.kobj, &solos_attr_group);
    atm_dev_deregister(card.atmdev[i]);
    skb = card.rx_skb[i];
    if (skb) {
    dma_unmap_single(&card.dev.dev, SKB_CB(skb).dma_addr,
    RX_DMA_SIZE, DMA_FROM_DEVICE);
    dev_kfree_skb(skb);
    }
    skb = card.tx_skb[i];
    if (skb) {
    dma_unmap_single(&card.dev.dev, SKB_CB(skb).dma_addr,
    skb.len, DMA_TO_DEVICE);
    dev_kfree_skb(skb);
    }
    while ((skb = skb_dequeue(&card.tx_queue[i])))
    dev_kfree_skb(skb);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn fpga_remove(dev: *mut pci_dev) {
    static void fpga_remove(struct pci_dev *dev)
    {
    struct solos_card *card = pci_get_drvdata(dev);
// Disable IRQs
    iowrite32(0, card.config_regs + IRQ_EN_ADDR);
// Reset FPGA
    iowrite32(1, card.config_regs + FPGA_MODE);
    (void)ioread32(card.config_regs + FPGA_MODE);
    if (card.fpga_version >= DMA_SUPPORTED)
    sysfs_remove_group(&card.dev.dev.kobj, &gpio_attr_group);
    atm_remove(card);
    free_irq(dev.irq, card);
    tasklet_kill(&card.tlet);
    kfree(card.dma_bounce);
// Release device from reset
    iowrite32(0, card.config_regs + FPGA_MODE);
    (void)ioread32(card.config_regs + FPGA_MODE);
    pci_iounmap(dev, card.buffers);
    pci_iounmap(dev, card.config_regs);
    pci_release_regions(dev);
    pci_disable_device(dev);
    kfree(card);
    }
    static const struct pci_device_id fpga_pci_tbl[] = {
    { PCI_DEVICE(0x10ee, 0x0300) },
    { }
    };
    MODULE_DEVICE_TABLE(pci,fpga_pci_tbl);
    static struct pci_driver fpga_driver = {
    .name =		"solos",
    .id_table =	fpga_pci_tbl,
    .probe =	fpga_probe,
    .remove =	fpga_remove,
    };
#[no_mangle]
unsafe extern "C" fn solos_pci_init() -> int __init {
    static int __init solos_pci_init(void)
    {
    BUILD_BUG_ON(sizeof(struct solos_skb_cb) > sizeof(((struct sk_buff *)0).cb));
    printk(KERN_INFO "Solos PCI Driver Version %s\n", VERSION);
    return pci_register_driver(&fpga_driver);
    }
#[no_mangle]
unsafe extern "C" fn solos_pci_exit() -> void __exit {
    static void __exit solos_pci_exit(void)
    {
    pci_unregister_driver(&fpga_driver);
    printk(KERN_INFO "Solos PCI Driver %s Unloaded\n", VERSION);
    }
    module_init(solos_pci_init);
    module_exit(solos_pci_exit);
