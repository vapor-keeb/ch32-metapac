
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "Real time clock.",
            ),
            items: &[
                BlockItem {
                    name: "ctlrh",
                    description: Some(
                        "RTC Control Register High.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlrl",
                    description: Some(
                        "RTC Control Register Low.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pscrh",
                    description: Some(
                        "RTC Prescaler Load Register High.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Pscrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pscrl",
                    description: Some(
                        "RTC Prescaler Load Register Low.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Pscrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "divh",
                    description: Some(
                        "RTC Prescaler Divider Register High.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Divh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "divl",
                    description: Some(
                        "RTC Prescaler Divider Register Low.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Divl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnth",
                    description: Some(
                        "RTC Counter Register High.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntl",
                    description: Some(
                        "RTC Counter Register Low.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrmh",
                    description: Some(
                        "RTC Alarm Register High.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrmh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrml",
                    description: Some(
                        "RTC Alarm Register Low.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrml",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alrmh",
            extends: None,
            description: Some(
                "RTC Alarm Register High.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrh",
                    description: Some(
                        "RTC alarm register high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Alrml",
            extends: None,
            description: Some(
                "RTC Alarm Register Low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrl",
                    description: Some(
                        "RTC alarm register low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnth",
            extends: None,
            description: Some(
                "RTC Counter Register High.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnth",
                    description: Some(
                        "RTC counter register high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cntl",
            extends: None,
            description: Some(
                "RTC Counter Register Low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntl",
                    description: Some(
                        "RTC counter register Low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlrh",
            extends: None,
            description: Some(
                "RTC Control Register High.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secie",
                    description: Some(
                        "Second interrupt Enable.",
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
                    name: "alrie",
                    description: Some(
                        "Alarm interrupt Enable.",
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
                    name: "owie",
                    description: Some(
                        "Overflow interrupt Enable.",
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
            ],
        },
        FieldSet {
            name: "Ctlrl",
            extends: None,
            description: Some(
                "RTC Control Register Low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secf",
                    description: Some(
                        "Second Flag.",
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
                    name: "alrf",
                    description: Some(
                        "Alarm Flag.",
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
                    name: "owf",
                    description: Some(
                        "Overflow Flag.",
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
                    name: "rsf",
                    description: Some(
                        "Registers Synchronized Flag.",
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
                    name: "cnf",
                    description: Some(
                        "Configuration Flag.",
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
                    name: "rtoff",
                    description: Some(
                        "RTC operation OFF.",
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
            name: "Divh",
            extends: None,
            description: Some(
                "RTC Prescaler Divider Register High.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "divh",
                    description: Some(
                        "RTC prescaler divider register high.",
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
            ],
        },
        FieldSet {
            name: "Divl",
            extends: None,
            description: Some(
                "RTC Prescaler Divider Register Low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "divl",
                    description: Some(
                        "RTC prescaler divider register Low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pscrh",
            extends: None,
            description: Some(
                "RTC Prescaler Load Register High.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prlh",
                    description: Some(
                        "RTC Prescaler Load Register High.",
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
            ],
        },
        FieldSet {
            name: "Pscrl",
            extends: None,
            description: Some(
                "RTC Prescaler Load Register Low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prll",
                    description: Some(
                        "RTC Prescaler Divider Register Low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                