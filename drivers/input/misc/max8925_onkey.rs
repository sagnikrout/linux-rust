//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/max8925_onkey.c
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
// MAX8925 ONKEY driver
//
// Copyright (C) 2009 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_onkey_info {
    pub idev: *mut input_dev,
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub irq: [c_uint; 2],
}

//
// MAX8925 gives us an interrupt when ONKEY is pressed or released.
// max8925_set_bits() operates I2C bus and may sleep. So implement
// it in thread IRQ handler.
//
#[no_mangle]
unsafe extern "C" fn max8925_onkey_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max8925_onkey_handler(int irq, void *data)
    {
    struct max8925_onkey_info *info = data;
    int state;
    state = max8925_reg_read(info.i2c, MAX8925_ON_OFF_STATUS);
    input_report_key(info.idev, KEY_POWER, state & SW_INPUT);
    input_sync(info.idev);
    dev_dbg(info.dev, "onkey state:%d\n", state);
// Enable hardreset to halt if system isn't shutdown on time
    max8925_set_bits(info.i2c, MAX8925_SYSENSEL,
    HARDRESET_EN, HARDRESET_EN);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max8925_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int max8925_onkey_probe(struct platform_device *pdev)
    {
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct max8925_onkey_info *info;
    struct input_dev *input;
    int irq[2], error;
    irq[0] = platform_get_irq(pdev, 0);
    if (irq[0] < 0)
    return -EINVAL;
    irq[1] = platform_get_irq(pdev, 1);
    if (irq[1] < 0)
    return -EINVAL;
    info = devm_kzalloc(&pdev.dev, sizeof(struct max8925_onkey_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    info.idev = input;
    info.i2c = chip.i2c;
    info.dev = &pdev.dev;
    info.irq[0] = irq[0];
    info.irq[1] = irq[1];
    input.name = "max8925_on";
    input.phys = "max8925_on/input0";
    input.id.bustype = BUS_I2C;
    input.dev.parent = &pdev.dev;
    input_set_capability(input, EV_KEY, KEY_POWER);
    error = devm_request_threaded_irq(&pdev.dev, irq[0], core::ptr::null_mut(),
    max8925_onkey_handler, IRQF_ONESHOT,
    "onkey-down", info);
    if (error < 0) {
    dev_err(chip.dev, "Failed to request IRQ: #%d: %d\n",
    irq[0], error);
    return error;
    }
    error = devm_request_threaded_irq(&pdev.dev, irq[1], core::ptr::null_mut(),
    max8925_onkey_handler, IRQF_ONESHOT,
    "onkey-up", info);
    if (error < 0) {
    dev_err(chip.dev, "Failed to request IRQ: #%d: %d\n",
    irq[1], error);
    return error;
    }
    error = input_register_device(info.idev);
    if (error) {
    dev_err(chip.dev, "Can't register input device: %d\n", error);
    return error;
    }
    platform_set_drvdata(pdev, info);
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8925_onkey_suspend(dev: *mut device) -> c_int {
    static int max8925_onkey_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct max8925_onkey_info *info = platform_get_drvdata(pdev);
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev)) {
    chip.wakeup_flag |= 1 << info.irq[0];
    chip.wakeup_flag |= 1 << info.irq[1];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8925_onkey_resume(dev: *mut device) -> c_int {
    static int max8925_onkey_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct max8925_onkey_info *info = platform_get_drvdata(pdev);
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev)) {
    chip.wakeup_flag &= ~(1 << info.irq[0]);
    chip.wakeup_flag &= ~(1 << info.irq[1]);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max8925_onkey_pm_ops,
    max8925_onkey_suspend, max8925_onkey_resume);
    static struct platform_driver max8925_onkey_driver = {
    .driver		= {
    .name	= "max8925-onkey",
    .pm	= pm_sleep_ptr(&max8925_onkey_pm_ops),
    },
    .probe		= max8925_onkey_probe,
    };
    module_platform_driver(max8925_onkey_driver);
    MODULE_DESCRIPTION("Maxim MAX8925 ONKEY driver");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
