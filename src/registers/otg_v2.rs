use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usb",
            extends: None,
            description: Some(
                "Universal serial bus FS OTG register, with VBUS, ID pin",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_cr",
                    description: Some(
                        "usb otg control.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_sr",
                    description: Some(
                        "usb otg status.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgSr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Usbd",
            extends: Some(
                "USB",
            ),
            description: Some(
                "Universal serial bus FS device register",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "udev_ctrl",
                    description: Some(
                        "USB device physical port control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UdevCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_1_mod",
                    description: Some(
                        "endpoint 4/1 mode.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_3_mod",
                    description: Some(
                        "Endpoint 2/3 mode control register.",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_6_mod",
                    description: Some(
                        "endpoint 5/6 mode.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_mod",
                    description: Some(
                        "endpoint 7 mode.",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_dma",
                    description: Some(
                        "endpoint DMA buffer address.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_len",
                    description: Some(
                        "endpoint transmit length.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_ctrl",
                    description: Some(
                        "endpoint control.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_ctrl",
                    description: Some(
                        "endpoint control.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x33,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_cr",
                    description: Some(
                        "usb otg control.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_sr",
                    description: Some(
                        "usb otg status.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgSr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Usbh",
            extends: Some(
                "USB",
            ),
            description: Some(
                "Universal serial bus FS host register",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uhost_ctrl",
                    description: Some(
                        "USB host physical port control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhostCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_mod",
                    description: Some(
                        "USB host endpoint mode control register.",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhEpMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_dma",
                    description: Some(
                        "USB host receiving DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "tx_dma",
                    description: Some(
                        "USB host transmittal DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "setup",
                    description: Some(
                        "USB host setup.",
                    ),
                    array: None,
                    byte_offset: 0x36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhSetup",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_pid",
                    description: Some(
                        "USB host endpoint PID.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhEpPid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_ctrl",
                    description: Some(
                        "USB host receiving control.",
                    ),
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_ctrl",
                    description: Some(
                        "USB host transmittal control.",
                    ),
                    array: None,
                    byte_offset: 0x3e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_cr",
                    description: Some(
                        "usb otg control.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otg_sr",
                    description: Some(
                        "usb otg status.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgSr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "UdevCtrl",
            extends: None,
            description: Some(
                "USB device physical port control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port_en",
                    description: Some(
                        "USB device port enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp_bit",
                    description: Some(
                        "USB device port general purpose bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "low_speed",
                    description: Some(
                        "USB device port low speed enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dm_pin",
                    description: Some(
                        "USB device port UD- pin status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dp_pin",
                    description: Some(
                        "USB device port UD+ pin status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd_dis",
                    description: Some(
                        "USB device port UD+/UD- pin internal pull-down resistor control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep7Mod",
            extends: None,
            description: Some(
                "endpoint 7 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_en",
                    description: Some(
                        "enable USB endpoint 7 transmittal (IN).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_en",
                    description: Some(
                        "enable USB endpoint 7 receiving (OUT).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepMod",
            extends: None,
            description: Some(
                "endpoint a/b mode. lower bits comes first",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tx_en",
                    description: Some(
                        "enable USB endpoint 1 transmittal (IN).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rx_en",
                    description: Some(
                        "enable USB endpoint 4 receiving (OUT).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxCtrl",
            extends: None,
            description: Some(
                "endpoint control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhEpMod",
            extends: None,
            description: Some(
                "USB host endpoint mode control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rbuf_mod",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_en",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbuf_mod",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_en",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhEpPid",
            extends: None,
            description: Some(
                "USB host endpoint PID.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_endp",
                    description: Some(
                        "endpoint PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mask_token",
                    description: Some(
                        "token PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhRxCtrl",
            extends: None,
            description: Some(
                "USB host receiving control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "r_res",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhSetup",
            extends: None,
            description: Some(
                "USB host setup.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sof_en",
                    description: Some(
                        "SOF packet en",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pre_pid_en",
                    description: Some(
                        "pre pid en",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhTxCtrl",
            extends: None,
            description: Some(
                "USB host transmittal control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "t_res",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_tog",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_auto_tog",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhostCtrl",
            extends: None,
            description: Some(
                "USB host physical port control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port_en",
                    description: Some(
                        "USB host port enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bus_rst",
                    description: Some(
                        "USB host port bus reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "low_speed",
                    description: Some(
                        "USB host port low speed enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dm_pin",
                    description: Some(
                        "Current UD- pin status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dp_pin",
                    description: Some(
                        "Current UD+ pin status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd_dis",
                    description: Some(
                        "Internal pull-down resistor control for USB host port UD+/UD- pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbBaseCtrl",
            extends: None,
            description: Some(
                "USB base control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_en",
                    description: Some(
                        "DMA enable and DMA interrupt enable for USB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clr_all",
                    description: Some(
                        "force clear FIFO and count of USB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reset_sie",
                    description: Some(
                        "force reset USB SIE, need software clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "int_busy",
                    description: Some(
                        "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sys_ctrl",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dev_pu_en",
                    description: Some(
                        "USB device internal pullup resistance enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "low_speed",
                    description: Some(
                        "enable USB low speed: 0=12Mbps, 1=1.5Mbps.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "host_mode",
                    description: Some(
                        "enable USB host mode: 0=device mode, 1=host mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbDevAd",
            extends: None,
            description: Some(
                "USB device address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_usb_addr",
                    description: Some(
                        "bit mask for USB device address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uda_gp_bit",
                    description: Some(
                        "general purpose bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbIntEn",
            extends: None,
            description: Some(
                "USB interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rst",
                    description: Some(
                        "enable interrupt for USB bus reset event for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "transfer",
                    description: Some(
                        "enable interrupt for USB transfer completion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "suspend",
                    description: Some(
                        "enable interrupt for USB suspend or resume event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hst_sof",
                    description: Some(
                        "enable interrupt for host SOF timer action for USB host mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fifo_ov",
                    description: Some(
                        "enable interrupt for FIFO overflow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dev_nak",
                    description: Some(
                        "enable interrupt for NAK responded for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dev_sof",
                    description: Some(
                        "enable interrupt for SOF received for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbIntFg",
            extends: None,
            description: Some(
                "USB interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rst",
                    description: Some(
                        "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "detect",
                    description: Some(
                        "device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "transfer",
                    description: Some(
                        "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "suspend",
                    description: Some(
                        "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hst_sof",
                    description: Some(
                        "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fifo_ov",
                    description: Some(
                        "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sie_free",
                    description: Some(
                        "RO, indicate USB SIE free status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tog_ok",
                    description: Some(
                        "RO, indicate current USB transfer toggle is OK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "is_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbIntSt",
            extends: None,
            description: Some(
                "USB interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_h_res",
                    description: Some(
                        "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mask_uis_endp",
                    description: Some(
                        "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mask_token",
                    description: Some(
                        "RO, bit mask of current token PID code received for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tog_ok",
                    description: Some(
                        "RO, indicate current USB transfer toggle is OK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "is_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbMisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dev_attach",
                    description: Some(
                        "RO, indicate device attached status on USB host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dm_level",
                    description: Some(
                        "RO, indicate UDM level saved at device attached to USB host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "suspend",
                    description: Some(
                        "RO, indicate USB suspend status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bus_reset",
                    description: Some(
                        "RO, indicate USB bus reset status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r_fifo_rdy",
                    description: Some(
                        "RO, indicate USB receiving FIFO ready status (not empty).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sie_free",
                    description: Some(
                        "RO, indicate USB SIE free status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sof_act",
                    description: Some(
                        "RO, indicate host SOF timer action status for USB host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sof_pres",
                    description: Some(
                        "RO, indicate host SOF timer presage status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbOtgCr",
            extends: None,
            description: Some(
                "usb otg control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "discharge_vbus",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "charge_vbus",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idpu",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otg_en",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbus_vth",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sess_vth",
                    description: Some(
                        "usb otg control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbOtgSr",
            extends: None,
            description: Some(
                "usb otg status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbus_vld",
                    description: Some(
                        "usb otg status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sess_vld",
                    description: Some(
                        "usb otg status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sess_end",
                    description: Some(
                        "usb otg status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "id_dig",
                    description: Some(
                        "usb otg status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbRxLen",
            extends: None,
            description: Some(
                "USB receiving length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "receiving length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
