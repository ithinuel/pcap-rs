#![cfg_attr(feature = "core", feature(no_std))]

#[cfg(feature = "core")]
extern crate collections;

#[macro_use]
extern crate nom;

#[cfg(not(feature = "core"))]
pub mod iter;

use nom::*;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LinkType {
    NULL,
    ETHERNET,
    AX25,
    IEEE802_5,
    ARCNET_BSD,
    SLIP,
    PPP,
    FDDI,
    PPP_HDLC,
    PPP_ETHER,
    ATM_RFC1483,
    RAW,
    C_HDLC,
    IEEE802_11,
    FRELAY,
    LOOP,
    LINUX_SLL,
    LTALK,
    PFLOG,
    IEEE802_11_PRISM,
    IP_OVER_FC,
    SUNATM,
    IEEE802_11_RADIOTAP,
    ARCNET_LINUX,
    APPLE_IP_OVER_IEEE1394,
    MTP2_WITH_PHDR,
    MTP2,
    MTP3,
    SCCP,
    DOCSIS,
    LINUX_IRDA,
    USER0,
    USER1,
    USER2,
    USER3,
    USER4,
    USER5,
    USER6,
    USER7,
    USER8,
    USER9,
    USER10,
    USER11,
    USER12,
    USER13,
    USER14,
    USER15,
    IEEE802_11_AVS,
    BACNET_MS_TP,
    PPP_PPPD,
    GPRS_LLC,
    GPF_T,
    GPF_F,
    LINUX_LAPD,
    BLUETOOTH_HCI_H4,
    USB_LINUX,
    PPI,
    IEEE802_15_4,
    SITA,
    ERF,
    BLUETOOTH_HCI_H4_WITH_PHDR,
    AX25_KISS,
    LAPD,
    PPP_WITH_DIR,
    C_HDLC_WITH_DIR,
    FRELAY_WITH_DIR,
    IPMB_LINUX,
    IEEE802_15_4_NONASK_PHY,
    USB_LINUX_MMAPPED,
    FC_2,
    FC_2_WITH_FRAME_DELIMS,
    IPNET,
    CAN_SOCKETCAN,
    IPV4,
    IPV6,
    IEEE802_15_4_NOFCS,
    DBUS,
    DVB_CI,
    MUX27010,
    STANAG_5066_D_PDU,
    NFLOG,
    NETANALYZER,
    NETANALYZER_TRANSPARENT,
    IPOIB,
    MPEG_2_TS,
    NG40,
    NFC_LLCP,
    INFINIBAND,
    SCTP,
    USBPCAP,
    RTAC_SERIAL,
    BLUETOOTH_LE_LL,
    NETLINK,
    BLUETOOTH_LINUX_MONITOR,
    BLUETOOTH_BREDR_BB,
    BLUETOOTH_LE_LL_WITH_PHDR,
    PROFIBUS_DL,
    PKTAP,
    EPON,
    IPMI_HPM_2,
    ZWAVE_R1_R2,
    ZWAVE_R3,
    WATTSTOPPER_DLM,
    ISO_14443,
    RDS,
    USB_DARWIN,
    SDLC,
    UNKNOWN
}

impl From<u32> for LinkType {
    fn from(n: u32) -> LinkType {
        match n {
            0 => LinkType::NULL,
            1 => LinkType::ETHERNET,
            3 => LinkType::AX25,
            6 => LinkType::IEEE802_5,
            7 => LinkType::ARCNET_BSD,
            8 => LinkType::SLIP,
            9 => LinkType::PPP,
            10 => LinkType::FDDI,
            50 => LinkType::PPP_HDLC,
            51 => LinkType::PPP_ETHER,
            100 => LinkType::ATM_RFC1483,
            101 => LinkType::RAW,
            104 => LinkType::C_HDLC,
            105 => LinkType::IEEE802_11,
            107 => LinkType::FRELAY,
            108 => LinkType::LOOP,
            113 => LinkType::LINUX_SLL,
            114 => LinkType::LTALK,
            117 => LinkType::PFLOG,
            119 => LinkType::IEEE802_11_PRISM,
            122 => LinkType::IP_OVER_FC,
            123 => LinkType::SUNATM,
            127 => LinkType::IEEE802_11_RADIOTAP,
            129 => LinkType::ARCNET_LINUX,
            138 => LinkType::APPLE_IP_OVER_IEEE1394,
            139 => LinkType::MTP2_WITH_PHDR,
            140 => LinkType::MTP2,
            141 => LinkType::MTP3,
            142 => LinkType::SCCP,
            143 => LinkType::DOCSIS,
            144 => LinkType::LINUX_IRDA,
            147 => LinkType::USER0,
            148 => LinkType::USER1,
            149 => LinkType::USER2,
            150 => LinkType::USER3,
            151 => LinkType::USER4,
            152 => LinkType::USER5,
            153 => LinkType::USER6,
            154 => LinkType::USER7,
            155 => LinkType::USER8,
            156 => LinkType::USER9,
            157 => LinkType::USER10,
            158 => LinkType::USER11,
            159 => LinkType::USER12,
            160 => LinkType::USER13,
            161 => LinkType::USER14,
            162 => LinkType::USER15,
            163 => LinkType::IEEE802_11_AVS,
            165 => LinkType::BACNET_MS_TP,
            166 => LinkType::PPP_PPPD,
            169 => LinkType::GPRS_LLC,
            170 => LinkType::GPF_T,
            171 => LinkType::GPF_F,
            177 => LinkType::LINUX_LAPD,
            187 => LinkType::BLUETOOTH_HCI_H4,
            189 => LinkType::USB_LINUX,
            192 => LinkType::PPI,
            195 => LinkType::IEEE802_15_4,
            196 => LinkType::SITA,
            197 => LinkType::ERF,
            201 => LinkType::BLUETOOTH_HCI_H4_WITH_PHDR,
            202 => LinkType::AX25_KISS,
            203 => LinkType::LAPD,
            204 => LinkType::PPP_WITH_DIR,
            205 => LinkType::C_HDLC_WITH_DIR,
            206 => LinkType::FRELAY_WITH_DIR,
            209 => LinkType::IPMB_LINUX,
            215 => LinkType::IEEE802_15_4_NONASK_PHY,
            220 => LinkType::USB_LINUX_MMAPPED,
            224 => LinkType::FC_2,
            225 => LinkType::FC_2_WITH_FRAME_DELIMS,
            226 => LinkType::IPNET,
            227 => LinkType::CAN_SOCKETCAN,
            228 => LinkType::IPV4,
            229 => LinkType::IPV6,
            230 => LinkType::IEEE802_15_4_NOFCS,
            231 => LinkType::DBUS,
            235 => LinkType::DVB_CI,
            236 => LinkType::MUX27010,
            237 => LinkType::STANAG_5066_D_PDU,
            239 => LinkType::NFLOG,
            240 => LinkType::NETANALYZER,
            241 => LinkType::NETANALYZER_TRANSPARENT,
            242 => LinkType::IPOIB,
            243 => LinkType::MPEG_2_TS,
            244 => LinkType::NG40,
            245 => LinkType::NFC_LLCP,
            247 => LinkType::INFINIBAND,
            248 => LinkType::SCTP,
            249 => LinkType::USBPCAP,
            250 => LinkType::RTAC_SERIAL,
            251 => LinkType::BLUETOOTH_LE_LL,
            253 => LinkType::NETLINK,
            254 => LinkType::BLUETOOTH_LINUX_MONITOR,
            255 => LinkType::BLUETOOTH_BREDR_BB,
            256 => LinkType::BLUETOOTH_LE_LL_WITH_PHDR,
            257 => LinkType::PROFIBUS_DL,
            258 => LinkType::PKTAP,
            259 => LinkType::EPON,
            260 => LinkType::IPMI_HPM_2,
            261 => LinkType::ZWAVE_R1_R2,
            262 => LinkType::ZWAVE_R3,
            263 => LinkType::WATTSTOPPER_DLM,
            264 => LinkType::ISO_14443,
            265 => LinkType::RDS,
            266 => LinkType::USB_DARWIN,
            268 => LinkType::SDLC,
            _ => LinkType::UNKNOWN
        }
    }
}

//
#[derive(PartialEq, Debug)]
pub struct Header {
    pub major: u16,
    pub minor: u16,
    pub this_zone: i32,
    pub sigfigs: u32,
    pub snaplen: u32,
    pub network: LinkType,
    pub nano_sec: bool,
    pub endianness: Endianness
}

#[derive(PartialEq, Debug)]
pub struct Record {
    pub ts_sec: u32,
    pub ts_nanosec: u32,
    pub orig_len: u32,
    pub data: Vec<u8>
}

named_args!(parse_header_e(e: Endianness, nsec: bool)<Header>,
    do_parse!(
        major: u16!(e) >>
        minor: u16!(e) >>
        this_zone: i32!(e) >>
        sigfigs: u32!(e) >>
        snaplen: u32!(e) >>
        network: verify!(
            map!(u32!(e), LinkType::from),
            |val:LinkType| { val != LinkType::UNKNOWN}
        ) >>
        (Header {
            major: major,
            minor: minor,
            this_zone: this_zone,
            sigfigs: sigfigs,
            snaplen: snaplen,
            network: network,
            nano_sec: nsec,
            endianness: e
        })
    )
);

named!(pub parse_header<Header>, switch!(be_u32,
    0xa1b2c3d4 => call!(parse_header_e, Endianness::Big, false)    | // straight sec
    0xd4c3b2a1 => call!(parse_header_e, Endianness::Little, false) | // reverse  sec
    0xa1b23c4d => call!(parse_header_e, Endianness::Big, true)     | // straight usec
    0x4d3cb2a1 => call!(parse_header_e, Endianness::Little, true)    // reverse  usec
));

named_args!(pub parse_record(e: Endianness, nano_sec: bool)<Record>, do_parse!(
    ts_sec: u32!(e) >>
    ts_subsec: u32!(e) >>
    incl_len: u32!(e) >>
    orig_len: u32!(e) >>
    data: take!(incl_len) >>

    (Record {
        ts_sec: ts_sec,
        ts_nanosec: if nano_sec {ts_subsec} else {ts_subsec*1000},
        orig_len: orig_len,
        data: Vec::from(data)
    })
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header_be_usec() {
        let i = b"\xa1\xb2\xc3\xd4\x00\x02\x00\x04\xFF\xFF\xFF\xFF\
                  \x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: LinkType::ETHERNET,
            nano_sec: false,
            endianness: Endianness::Big
        };
        assert_eq!(parse_header(&i[..]), Ok((&[10][..], h)));
    }
    #[test]
    fn parse_header_le_usec() {
        let i = b"\xd4\xc3\xb2\xa1\x02\x00\x04\x00\xFF\xFF\xFF\xFF\
                  \x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: LinkType::ETHERNET,
            nano_sec: false,
            endianness: Endianness::Little
        };
        assert_eq!(parse_header(&i[..]), Ok((&[10][..], h)));
    }

    #[test]
    fn parse_header_be_nsec() {
        let i = b"\xa1\xb2\x3c\x4d\x00\x02\x00\x04\xFF\xFF\xFF\xFF\
                  \x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: LinkType::ETHERNET,
            nano_sec: true,
            endianness: Endianness::Big
        };
        assert_eq!(parse_header(&i[..]), Ok((&[10][..], h)));
    }
    #[test]
    fn parse_header_le_nsec() {
        let i = b"\x4d\x3c\xb2\xa1\x02\x00\x04\x00\xFF\xFF\xFF\xFF\
                  \x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: LinkType::ETHERNET,
            nano_sec: true,
            endianness: Endianness::Little
        };
        assert_eq!(parse_header(&i[..]), Ok((&[10][..], h)));
    }

    #[test]
    fn parse_record_be_empty() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\
                  \x00\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 0,
            orig_len: 0,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Big, false), Ok((&[10][..], r)));
    }
    #[test]
    fn parse_record_be_some_orig_data_zero_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x00\
                  \x00\x00\x00\x03\x0a";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Big, true), Ok((&[10][..], r)));
    }
    #[test]
    fn parse_record_be_some_orig_data_parially_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x02\
                  \x00\x00\x00\x03\x0a\x0b\x80";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: vec![10, 11]
        };
        assert_eq!(parse_record(&i[..], Endianness::Big, true), Ok((&[128][..], r)));
    }
    #[test]
    fn parse_record_be_all_data_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x03\
                  \x00\x00\x00\x03\x0a\x0b\x0c\x80";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: vec![10, 11, 12]
        };
        assert_eq!(parse_record(&i[..], Endianness::Big, true), Ok((&[128][..], r)));
    }

    #[test]
    fn parse_record_le_empty() {
        let i = b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                  \x00\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 0,
            orig_len: 0,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Little, true), Ok((&[10][..], r)));
    }
    #[test]
    fn parse_record_le_some_orig_data_zero_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\
                  \x03\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Little, true), Ok((&[10][..], r)));
    }
    #[test]
    fn parse_record_le_some_orig_data_parially_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\
                  \x03\x00\x00\x00\x0a\x0b\x80";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: vec![10, 11]
        };
        assert_eq!(parse_record(&i[..], Endianness::Little, true), Ok((&[128][..], r)));
    }
    #[test]
    fn parse_record_le_all_data_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\
                  \x03\x00\x00\x00\x0a\x0b\x0c\x80";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2,
            orig_len: 3,
            data: vec![10, 11, 12]
        };
        assert_eq!(parse_record(&i[..], Endianness::Little, true), Ok((&[128][..], r)));
    }
    #[test]
    fn parse_record_le_all_data_incl_usec() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\
                  \x03\x00\x00\x00\x0a\x0b\x0c\x80";
        let r = Record {
            ts_sec: 1,
            ts_nanosec: 2000,
            orig_len: 3,
            data: vec![10, 11, 12]
        };
        assert_eq!(parse_record(&i[..], Endianness::Little, false), Ok((&[128][..], r)));
    }
}
