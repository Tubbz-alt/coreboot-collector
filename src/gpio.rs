pub struct GpioGroup<'a> {
    pub name: &'a str,
    pub offset: u16,
    pub count: u8,
}

pub struct GpioCommunity<'a> {
    pub id: u8,
    pub step: usize,
    pub groups: &'a [GpioGroup<'a>]
}

impl<'a> GpioCommunity<'a> {
    /*
    pub fn skylake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        count: 13
                    },
                    GpioGroup {
                        name: "GPP_F",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_H",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        count: 11
                    },
                ]
            }
        ]
    }
    */

    pub fn skylake_lp() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x400,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x4C0,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x400,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x4C0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x580,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x400,
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x400,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x4C0,
                        count: 8
                    },
                ]
            }
        ]
    }

    pub fn cannonlake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x790,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x780,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x900,
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x600,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_K",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x780,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x900,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x9D0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        offset: 0x740,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_J",
                        offset: 0x860,
                        count: 12,
                    },
                ],
            },
        ]
    }

    pub fn cannonlake_lp() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x790,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x930,
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x790,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x910,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x600,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x600,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x780,
                        count: 24,
                    },
                ],
            },
        ]
    }
}
