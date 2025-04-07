// Copyright (C) 2025 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use log::debug;
use std::sync::OnceLock;

use cea708_types::{tables, CCDataParser, CCDataWriter, Cea608, DTVCCPacket, Framerate, Service};

static TRACING: OnceLock<()> = OnceLock::new();

pub fn test_init_log() {
    TRACING.get_or_init(|| {
        env_logger::init();
    });
}

#[derive(Debug)]
struct ServiceData<'a> {
    service_no: u8,
    codes: &'a [tables::Code],
}

#[derive(Debug)]
struct PacketData<'a> {
    sequence_no: u8,
    services: &'a [ServiceData<'a>],
}

#[derive(Debug)]
struct TestCCData<'a> {
    framerate: Framerate,
    cc_data: &'a [&'a [u8]],
    packets: &'a [PacketData<'a>],
    cea608: &'a [&'a [Cea608]],
}

static TEST_CODES: [tables::Code; 34] = [
    tables::Code::LatinCapitalA,
    tables::Code::LatinCapitalB,
    tables::Code::LatinCapitalC,
    tables::Code::LatinCapitalD,
    tables::Code::LatinCapitalE,
    tables::Code::LatinCapitalF,
    tables::Code::LatinCapitalG,
    tables::Code::LatinCapitalH,
    tables::Code::LatinCapitalI,
    tables::Code::LatinCapitalJ,
    tables::Code::LatinCapitalK,
    tables::Code::LatinCapitalL,
    tables::Code::LatinCapitalM,
    tables::Code::LatinCapitalN,
    tables::Code::LatinCapitalO,
    tables::Code::LatinCapitalP,
    tables::Code::LatinCapitalQ,
    tables::Code::LatinCapitalR,
    tables::Code::LatinCapitalS,
    tables::Code::LatinCapitalT,
    tables::Code::LatinCapitalU,
    tables::Code::LatinCapitalV,
    tables::Code::LatinCapitalW,
    tables::Code::LatinCapitalX,
    tables::Code::LatinCapitalY,
    tables::Code::LatinCapitalZ,
    tables::Code::LatinLowerA,
    tables::Code::LatinLowerB,
    tables::Code::LatinLowerC,
    tables::Code::LatinLowerD,
    tables::Code::LatinLowerE,
    tables::Code::LatinLowerF,
    tables::Code::LatinLowerG,
    tables::Code::LatinLowerH,
];

static TEST_CC_DATA: [TestCCData; 11] = [
    // simple packet with a single service and single code
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA],
            }],
        }],
        cea608: &[],
    },
    // simple packet with a single service and two codes
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x22, 0xFE, 0x41, 0x42]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA, tables::Code::LatinCapitalB],
            }],
        }],
        cea608: &[],
    },
    // two packets each with a single service and single code
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[
            &[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00],
            &[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x42, 0x21, 0xFE, 0x42, 0x00],
        ],
        packets: &[
            PacketData {
                sequence_no: 0,
                services: &[ServiceData {
                    service_no: 1,
                    codes: &[tables::Code::LatinCapitalA],
                }],
            },
            PacketData {
                sequence_no: 1,
                services: &[ServiceData {
                    service_no: 1,
                    codes: &[tables::Code::LatinCapitalB],
                }],
            },
        ],
        cea608: &[],
    },
    // DTVCCPacket with two services
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[
            0x80 | 0x40 | 0x03,
            0xFF,
            0xFF,
            0x03,
            0x21,
            0xFE,
            0x41,
            0x41,
            0xFE,
            0x42,
            0x00,
        ]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[
                ServiceData {
                    service_no: 1,
                    codes: &[tables::Code::LatinCapitalA],
                },
                ServiceData {
                    service_no: 2,
                    codes: &[tables::Code::LatinCapitalB],
                },
            ],
        }],
        cea608: &[],
    },
    // simple packet with a single service and single code
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x21, 0xFE, 0x41, 0x00]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA],
            }],
        }],
        cea608: &[],
    },
    // simple packet with a single service and two codes
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFF, 0x02, 0x22, 0xFE, 0x41, 0x42]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA, tables::Code::LatinCapitalB],
            }],
        }],
        cea608: &[],
    },
    // packet with a full service
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[
            0x80 | 0x40 | 0x11,
            0xFF,
            0xFF,
            0xC0 | 0x11,
            0x20 | 0x1F,
            0xFE,
            0x41,
            0x42,
            0xFE,
            0x43,
            0x44,
            0xFE,
            0x45,
            0x46,
            0xFE,
            0x47,
            0x48,
            0xFE,
            0x49,
            0x4A,
            0xFE,
            0x4B,
            0x4C,
            0xFE,
            0x4D,
            0x4E,
            0xFE,
            0x4F,
            0x50,
            0xFE,
            0x51,
            0x52,
            0xFE,
            0x53,
            0x54,
            0xFE,
            0x55,
            0x56,
            0xFE,
            0x57,
            0x58,
            0xFE,
            0x59,
            0x5A,
            0xFE,
            0x61,
            0x62,
            0xFE,
            0x63,
            0x64,
            0xFE,
            0x65,
            0x0,
        ]],
        packets: &[PacketData {
            sequence_no: 3,
            services: &[ServiceData {
                service_no: 1,
                codes: TEST_CODES.first_chunk::<31>().unwrap(),
            }],
        }],
        cea608: &[],
    },
    // simple packet with only cea608 data
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x01, 0xFF, 0xFC, 0x41, 0x42]],
        packets: &[],
        cea608: &[&[Cea608::Field1(0x41, 0x42)]],
    },
    // full packet spanning multiple outputs
    TestCCData {
        framerate: Framerate::new(60, 1),
        cc_data: &[
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFC,
                0x20,
                0x42,
                0xFF,
                0xC0 | 0x11,
                0x20 | 0x1F,
                0xFE,
                0x41,
                0x42,
                0xFE,
                0x43,
                0x44,
                0xFE,
                0x45,
                0x46,
                0xFE,
                0x47,
                0x48,
                0xFE,
                0x49,
                0x4A,
                0xFE,
                0x4B,
                0x4C,
                0xFE,
                0x4D,
                0x4E,
                0xFE,
                0x4F,
                0x50,
            ],
            &[
                0x80 | 0x40 | 0x09,
                0xFF,
                0xFD,
                0x21,
                0x43,
                0xFE,
                0x51,
                0x52,
                0xFE,
                0x53,
                0x54,
                0xFE,
                0x55,
                0x56,
                0xFE,
                0x57,
                0x58,
                0xFE,
                0x59,
                0x5A,
                0xFE,
                0x61,
                0x62,
                0xFE,
                0x63,
                0x64,
                0xFE,
                0x65,
                0x0,
            ],
        ],
        packets: &[PacketData {
            sequence_no: 3,
            services: &[ServiceData {
                service_no: 1,
                codes: TEST_CODES.first_chunk::<31>().unwrap(),
            }],
        }],
        cea608: &[&[Cea608::Field1(0x20, 0x42)], &[Cea608::Field2(0x21, 0x43)]],
    },
    // simple packet with multiple cea608 that will span two outputs
    TestCCData {
        framerate: Framerate::new(24, 1),
        cc_data: &[
            &[0x80 | 0x40 | 0x02, 0xFF, 0xFC, 0x20, 0x42, 0xFD, 0x21, 0x43],
            &[0x80 | 0x40 | 0x02, 0xFF, 0xFC, 0x22, 0x44, 0xFD, 0x23, 0x45],
        ],
        packets: &[],
        cea608: &[
            &[Cea608::Field1(0x20, 0x42), Cea608::Field2(0x21, 0x43)],
            &[Cea608::Field1(0x22, 0x44), Cea608::Field2(0x23, 0x45)],
        ],
    },
    // a full packet that will span four outputs
    TestCCData {
        framerate: Framerate::new(60, 1),
        cc_data: &[
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFC,
                0x20,
                0x42,
                0xFF,
                0x00, // seq 0, size 127
                0x20 | 0x1F,
                0xFE,
                0x41,
                0x42,
                0xFE,
                0x43,
                0x44,
                0xFE,
                0x45,
                0x46,
                0xFE,
                0x47,
                0x48,
                0xFE,
                0x49,
                0x4A,
                0xFE,
                0x4B,
                0x4C,
                0xFE,
                0x4D,
                0x4E,
                0xFE,
                0x4F,
                0x50,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFD,
                0x21,
                0x43,
                0xFE,
                0x51,
                0x52,
                0xFE,
                0x53,
                0x54,
                0xFE,
                0x55,
                0x56,
                0xFE,
                0x57,
                0x58,
                0xFE,
                0x59,
                0x5A,
                0xFE,
                0x61,
                0x62,
                0xFE,
                0x63,
                0x64,
                0xFE,
                0x65,
                0x40 | 0x1F,
                0xFE,
                0x42,
                0x43,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFC,
                0x22,
                0x44,
                0xFE,
                0x44,
                0x45,
                0xFE,
                0x46,
                0x47,
                0xFE,
                0x48,
                0x49,
                0xFE,
                0x4A,
                0x4B,
                0xFE,
                0x4C,
                0x4D,
                0xFE,
                0x4E,
                0x4F,
                0xFE,
                0x50,
                0x51,
                0xFE,
                0x52,
                0x53,
                0xFE,
                0x54,
                0x55,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFD,
                0x23,
                0x45,
                0xFE,
                0x56,
                0x57,
                0xFE,
                0x58,
                0x59,
                0xFE,
                0x5A,
                0x61,
                0xFE,
                0x62,
                0x63,
                0xFE,
                0x64,
                0x65,
                0xFE,
                0x66,
                0x60 | 0x1F,
                0xFE,
                0x43,
                0x44,
                0xFE,
                0x45,
                0x46,
                0xFE,
                0x47,
                0x48,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFC,
                0x24,
                0x46,
                0xFE,
                0x49,
                0x4A,
                0xFE,
                0x4B,
                0x4C,
                0xFE,
                0x4D,
                0x4E,
                0xFE,
                0x4F,
                0x50,
                0xFE,
                0x51,
                0x52,
                0xFE,
                0x53,
                0x54,
                0xFE,
                0x55,
                0x56,
                0xFE,
                0x57,
                0x58,
                0xFE,
                0x59,
                0x5A,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFD,
                0x25,
                0x47,
                0xFE,
                0x61,
                0x62,
                0xFE,
                0x63,
                0x64,
                0xFE,
                0x65,
                0x66,
                0xFE,
                0x67,
                0x80 | 0x1E,
                0xFE,
                0x44,
                0x45,
                0xFE,
                0x46,
                0x47,
                0xFE,
                0x48,
                0x49,
                0xFE,
                0x4A,
                0x4B,
                0xFE,
                0x4C,
                0x4D,
            ],
            &[
                0x80 | 0x40 | 0x0A,
                0xFF,
                0xFC,
                0x26,
                0x48,
                0xFE,
                0x4E,
                0x4F,
                0xFE,
                0x50,
                0x51,
                0xFE,
                0x52,
                0x53,
                0xFE,
                0x54,
                0x55,
                0xFE,
                0x56,
                0x57,
                0xFE,
                0x58,
                0x59,
                0xFE,
                0x5A,
                0x61,
                0xFE,
                0x62,
                0x63,
                0xFE,
                0x64,
                0x65,
            ],
            &[0x80 | 0x40 | 0x02, 0xFF, 0xFD, 0x27, 0x49, 0xFE, 0x66, 0x67],
        ],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[
                ServiceData {
                    service_no: 1,
                    codes: TEST_CODES.first_chunk::<31>().unwrap(),
                },
                ServiceData {
                    service_no: 2,
                    codes: TEST_CODES
                        .first_chunk::<32>()
                        .unwrap()
                        .last_chunk::<31>()
                        .unwrap(),
                },
                ServiceData {
                    service_no: 3,
                    codes: TEST_CODES
                        .first_chunk::<33>()
                        .unwrap()
                        .last_chunk::<31>()
                        .unwrap(),
                },
                ServiceData {
                    service_no: 4,
                    codes: TEST_CODES
                        .first_chunk::<33>()
                        .unwrap()
                        .last_chunk::<30>()
                        .unwrap(),
                },
            ],
        }],
        cea608: &[
            &[Cea608::Field1(0x20, 0x42)],
            &[Cea608::Field2(0x21, 0x43)],
            &[Cea608::Field1(0x22, 0x44)],
            &[Cea608::Field2(0x23, 0x45)],
            &[Cea608::Field1(0x24, 0x46)],
            &[Cea608::Field2(0x25, 0x47)],
            &[Cea608::Field1(0x26, 0x48)],
            &[Cea608::Field2(0x27, 0x49)],
        ],
    },
];

static PARSE_CC_DATA: [TestCCData; 5] = [
    // two packets with a single service and one code split across both packets
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[
            &[0x80 | 0x40 | 0x01, 0xFF, 0xFF, 0x02, 0x21],
            &[0x80 | 0x40 | 0x01, 0xFF, 0xFE, 0x41, 0x00],
        ],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA],
            }],
        }],
        cea608: &[],
    },
    // simple packet with a single null service
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x01, 0xFF, 0xFF, 0x01, 0x00]],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[],
        }],
        cea608: &[],
    },
    // cc_data with two DTVCCPacket
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[
            0x80 | 0x40 | 0x04,
            0xFF,
            0xFF,
            0x02,
            0x21,
            0xFE,
            0x41,
            0x00,
            0xFF,
            0x42,
            0x41,
            0xFE,
            0x42,
            0x00,
        ]],
        packets: &[
            PacketData {
                sequence_no: 0,
                services: &[ServiceData {
                    service_no: 1,
                    codes: &[tables::Code::LatinCapitalA],
                }],
            },
            PacketData {
                sequence_no: 1,
                services: &[ServiceData {
                    service_no: 2,
                    codes: &[tables::Code::LatinCapitalB],
                }],
            },
        ],
        cea608: &[],
    },
    // two packets with a single service and one code split across both packets with 608
    // padding data
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[
            &[
                0x80 | 0x40 | 0x03,
                0xFF,
                0xFC,
                0x61,
                0x62,
                0xFD,
                0x63,
                0x64,
                0xFF,
                0x02,
                0x21,
            ],
            &[
                0x80 | 0x40 | 0x03,
                0xFF,
                0xFC,
                0x41,
                0x42,
                0xFD,
                0x43,
                0x44,
                0xFE,
                0x41,
                0x00,
            ],
        ],
        packets: &[PacketData {
            sequence_no: 0,
            services: &[ServiceData {
                service_no: 1,
                codes: &[tables::Code::LatinCapitalA],
            }],
        }],
        cea608: &[
            &[Cea608::Field1(0x61, 0x62), Cea608::Field2(0x63, 0x64)],
            &[Cea608::Field1(0x41, 0x42), Cea608::Field2(0x43, 0x44)],
        ],
    },
    // cc_data with two DTVCCPacket. First packet spans two cc_data frames, second is whole
    // contained within the second cc_data frame
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[
            &[0x80 | 0x40 | 0x01, 0xFF, 0xFF, 0x02, 0x21],
            &[
                0x80 | 0x40 | 0x03,
                0xFF,
                0xFE,
                0x41,
                0x00,
                0xFF,
                0x42,
                0x41,
                0xFE,
                0x42,
                0x00,
            ],
        ],
        packets: &[
            PacketData {
                sequence_no: 0,
                services: &[ServiceData {
                    service_no: 1,
                    codes: &[tables::Code::LatinCapitalA],
                }],
            },
            PacketData {
                sequence_no: 1,
                services: &[ServiceData {
                    service_no: 2,
                    codes: &[tables::Code::LatinCapitalB],
                }],
            },
        ],
        cea608: &[],
    },
];

#[test]
fn cc_data_parse() {
    test_init_log();
    for (i, test_data) in TEST_CC_DATA.iter().chain(PARSE_CC_DATA.iter()).enumerate() {
        log::info!("parsing {i}: {test_data:?}");
        let mut parser = CCDataParser::new();
        if !test_data.cea608.is_empty() {
            parser.handle_cea608();
        }
        let mut expected_iter = test_data.packets.iter();
        let mut cea608_iter = test_data.cea608.iter();
        for data in test_data.cc_data.iter() {
            debug!("pushing {data:?}");
            parser.push(data).unwrap();
            while let Some(packet) = parser.pop_packet() {
                let expected = expected_iter.next().unwrap();
                assert_eq!(expected.sequence_no, packet.sequence_no());
                let services = packet.services();
                let mut expected_service_iter = expected.services.iter();
                for parsed_service in services.iter() {
                    let expected_service = expected_service_iter.next().unwrap();
                    assert_eq!(parsed_service.number(), expected_service.service_no);
                    assert_eq!(expected_service.codes, parsed_service.codes());
                }
                assert!(expected_service_iter.next().is_none());
            }
            assert_eq!(parser.cea608().as_ref(), cea608_iter.next());
        }
        assert!(parser.pop_packet().is_none());
        assert!(expected_iter.next().is_none());
        assert!(cea608_iter.next().is_none());
    }
}

static WRITE_CC_DATA: [TestCCData; 1] = [
    // simple packet with only cea608 field 2 data
    TestCCData {
        framerate: Framerate::new(25, 1),
        cc_data: &[&[0x80 | 0x40 | 0x02, 0xFF, 0xFC, 0x80, 0x80, 0xFD, 0x41, 0x42]],
        packets: &[],
        cea608: &[&[Cea608::Field2(0x41, 0x42)]],
    },
];

#[test]
fn packet_write_cc_data() {
    test_init_log();
    for (i, test_data) in TEST_CC_DATA.iter().chain(WRITE_CC_DATA.iter()).enumerate() {
        log::info!("writing {i}: {test_data:?}");
        let mut packet_iter = test_data.packets.iter();
        let mut cea608_iter = test_data.cea608.iter();
        let mut writer = CCDataWriter::default();
        for cc_data in test_data.cc_data.iter() {
            if let Some(packet_data) = packet_iter.next() {
                let mut pack = DTVCCPacket::new(packet_data.sequence_no);
                for service_data in packet_data.services.iter() {
                    let mut service = Service::new(service_data.service_no);
                    for code in service_data.codes.iter() {
                        service.push_code(code).unwrap();
                    }
                    pack.push_service(service).unwrap();
                }
                writer.push_packet(pack);
            }
            if let Some(&cea608) = cea608_iter.next() {
                for pair in cea608 {
                    writer.push_cea608(*pair);
                }
            }
            let mut written = vec![];
            writer.write(test_data.framerate, &mut written).unwrap();
            assert_eq!(cc_data, &written);
        }
    }
}
