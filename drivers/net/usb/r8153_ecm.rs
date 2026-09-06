//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/r8153_ecm.c
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

pub const OCP_BASE: c_uint = 0xe86c;
#[no_mangle]
unsafe extern "C" fn pla_read_word(dev: *mut usbnet, index: u16) -> c_int {
    static int pla_read_word(struct usbnet *dev, u16 index)
    {
    let mut byen: u16 = BYTE_EN_WORD;
    let mut shift: u8 = index & 2;
    __le32 tmp;
    int ret;
    if (shift)
    byen <<= shift;
    index &= ~3;
    ret = usbnet_read_cmd(dev, RTL8152_REQ_GET_REGS, RTL8152_REQT_READ, index,
    MCU_TYPE_PLA | byen, &tmp, sizeof(tmp));
    if (ret < 0)
    goto out;
    ret = __le32_to_cpu(tmp);
    ret >>= (shift * 8);
    ret &= 0xffff;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pla_write_word(dev: *mut usbnet, index: u16, data: u32) -> c_int {
    static int pla_write_word(struct usbnet *dev, u16 index, u32 data)
    {
    let mut mask: u32 = 0xffff;
    let mut byen: u16 = BYTE_EN_WORD;
    let mut shift: u8 = index & 2;
    __le32 tmp;
    int ret;
    data &= mask;
    if (shift) {
    byen <<= shift;
    mask <<= (shift * 8);
    data <<= (shift * 8);
    }
    index &= ~3;
    ret = usbnet_read_cmd(dev, RTL8152_REQ_GET_REGS, RTL8152_REQT_READ, index,
    MCU_TYPE_PLA | byen, &tmp, sizeof(tmp));
    if (ret < 0)
    goto out;
    data |= __le32_to_cpu(tmp) & ~mask;
    tmp = __cpu_to_le32(data);
    ret = usbnet_write_cmd(dev, RTL8152_REQ_SET_REGS, RTL8152_REQT_WRITE, index,
    MCU_TYPE_PLA | byen, &tmp, sizeof(tmp));
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn r8153_ecm_mdio_read(netdev: *mut net_device, phy_id: c_int, reg: c_int) -> c_int {
    static int r8153_ecm_mdio_read(struct net_device *netdev, int phy_id, int reg)
    {
    struct usbnet *dev = netdev_priv(netdev);
    int ret;
    ret = pla_write_word(dev, OCP_BASE, 0xa000);
    if (ret < 0)
    goto out;
    ret = pla_read_word(dev, 0xb400 + reg * 2);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn r8153_ecm_mdio_write(netdev: *mut net_device, phy_id: c_int, reg: c_int, val: c_int) {
    static void r8153_ecm_mdio_write(struct net_device *netdev, int phy_id, int reg, int val)
    {
    struct usbnet *dev = netdev_priv(netdev);
    int ret;
    ret = pla_write_word(dev, OCP_BASE, 0xa000);
    if (ret < 0)
    return;
    ret = pla_write_word(dev, 0xb400 + reg * 2, val);
    }
#[no_mangle]
unsafe extern "C" fn r8153_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int r8153_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    int status;
    status = usbnet_cdc_bind(dev, intf);
    if (status < 0)
    return status;
    dev.mii.dev = dev.net;
    dev.mii.mdio_read = r8153_ecm_mdio_read;
    dev.mii.mdio_write = r8153_ecm_mdio_write;
    dev.mii.reg_num_mask = 0x1f;
    dev.mii.supports_gmii = 1;
    return status;
    }
    static const struct driver_info r8153_info = {
    .description =	"RTL8153 ECM Device",
    .flags =	FLAG_ETHER,
    .bind =		r8153_bind,
    .unbind =	usbnet_cdc_unbind,
    .status =	usbnet_cdc_status,
    .manage_power =	usbnet_manage_power,
    };
    static const struct usb_device_id products[] = {
// Realtek RTL8153 Based USB 3.0 Ethernet Adapters
    {
    USB_DEVICE_AND_INTERFACE_INFO(VENDOR_ID_REALTEK, 0x8153, USB_CLASS_COMM,
    USB_CDC_SUBCLASS_ETHERNET, USB_CDC_PROTO_NONE),
    .driver_info = (unsigned long)&r8153_info,
    },
// Lenovo Powered USB-C Travel Hub (4X90S92381, based on Realtek RTL8153)
    {
    USB_DEVICE_AND_INTERFACE_INFO(VENDOR_ID_LENOVO, 0x721e, USB_CLASS_COMM,
    USB_CDC_SUBCLASS_ETHERNET, USB_CDC_PROTO_NONE),
    .driver_info = (unsigned long)&r8153_info,
    },
// Lenovo ThinkPad Hybrid USB-C with USB-A Dock (40af0135eu, based on Realtek RTL8153)
    {
    USB_DEVICE_AND_INTERFACE_INFO(VENDOR_ID_LENOVO, 0xa359, USB_CLASS_COMM,
    USB_CDC_SUBCLASS_ETHERNET, USB_CDC_PROTO_NONE),
    .driver_info = (unsigned long)&r8153_info,
    },
    { },		/* END */
    };
    MODULE_DEVICE_TABLE(usb, products);
    static int rtl8153_ecm_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {

    if (rtl8152_get_version(intf))
    return -ENODEV;

    return usbnet_probe(intf, id);
    }
    static struct usb_driver r8153_ecm_driver = {
    .name =		"r8153_ecm",
    .id_table =	products,
    .probe =	rtl8153_ecm_probe,
    .disconnect =	usbnet_disconnect,
    .suspend =	usbnet_suspend,
    .resume =	usbnet_resume,
    .reset_resume =	usbnet_resume,
    .supports_autosuspend = 1,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(r8153_ecm_driver);
    MODULE_AUTHOR("Hayes Wang");
    MODULE_DESCRIPTION("Realtek USB ECM device");
    MODULE_LICENSE("GPL");
