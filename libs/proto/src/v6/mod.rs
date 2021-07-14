mod options;

// re-export submodules from proto::msg
pub use self::options::*;

use crate::{
    decoder::{Decodable, Decoder},
    encoder::{Encodable, Encoder},
    error::*,
};

/// [DHCP for Ipv6](https://datatracker.ietf.org/doc/html/rfc3315)
/// RFC 3315                     DHCP for IPv6                     July 2003
///
///
///    Options are stored serially in the options field, with no padding
///    between the options.  Options are byte-aligned but are not aligned in
///    any other way such as on 2 or 4 byte boundaries.
///
///    The following diagram illustrates the format of DHCP messages sent
///    between clients and servers:
///
/// ```text
///        0                   1                   2                   3
///        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///       |    msg-type   |               transaction-id                  |
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///       |                                                               |
///       .                            options                            .
///       .                           (variable)                          .
///       |                                                               |
///       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
///       msg-type             Identifies the DHCP message type; the
///                            available message types are listed in
///                            section 5.3.
///
///       transaction-id       The transaction ID for this message exchange.
///
///       options              Options carried in this message; options are
///                            described in section 22.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// message type
    /// https://datatracker.ietf.org/doc/html/rfc3315#section-5.3
    mtype: MessageType,
    /// transaction id: https://datatracker.ietf.org/doc/html/rfc3315#section-15.1
    trans_id: [u8; 3],
    /// Options: https://datatracker.ietf.org/doc/html/rfc3315#section-22
    options: DhcpOptions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageType {
    // RFC 3315
    Solicit,
    Advertise,
    Request,
    Confirm,
    Renew,
    Rebind,
    Reply,
    Release,
    Decline,
    Reconfigure,
    InformationRequest,
    RelayForw,
    RelayRepl,
    // RFC 5007
    LeaseQuery,
    LeaseQueryReply,
    // RFC 5460
    LeaseQueryDone,
    LeaseQueryData,
    // RFC 6977
    ReconfigureRequest,
    ReconfigureReply,
    // RFC 7341
    DHCPv4Query,
    DHCPv4Response,
    Unknown(u8),
}

impl From<u8> for MessageType {
    fn from(n: u8) -> Self {
        use MessageType::*;
        match n {
            // RFC 3315
            1 => Solicit,
            2 => Advertise,
            3 => Request,
            4 => Confirm,
            5 => Renew,
            6 => Rebind,
            7 => Reply,
            8 => Release,
            9 => Decline,
            10 => Reconfigure,
            11 => InformationRequest,
            12 => RelayForw,
            13 => RelayRepl,
            // RFC 5007
            14 => LeaseQuery,
            15 => LeaseQueryReply,
            // RFC 5460
            16 => LeaseQueryDone,
            17 => LeaseQueryData,
            // RFC 6977
            18 => ReconfigureRequest,
            19 => ReconfigureReply,
            // RFC 7341
            20 => DHCPv4Query,
            21 => DHCPv4Response,
            n => Unknown(n),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(m: MessageType) -> Self {
        use MessageType::*;
        match m {
            // RFC 3315
            Solicit => 1,
            Advertise => 2,
            Request => 3,
            Confirm => 4,
            Renew => 5,
            Rebind => 6,
            Reply => 7,
            Release => 8,
            Decline => 9,
            Reconfigure => 10,
            InformationRequest => 11,
            RelayForw => 12,
            RelayRepl => 13,
            // RFC 5007
            LeaseQuery => 14,
            LeaseQueryReply => 15,
            // RFC 5460
            LeaseQueryDone => 16,
            LeaseQueryData => 17,
            // RFC 6977
            ReconfigureRequest => 18,
            ReconfigureReply => 19,
            // RFC 7341
            DHCPv4Query => 20,
            DHCPv4Response => 21,
            Unknown(n) => n,
        }
    }
}

impl<'r> Decodable<'r> for Message {
    fn decode(decoder: &mut Decoder<'r>) -> DecodeResult<Self> {
        Ok(Message {
            mtype: decoder.read_u8()?.into(),
            trans_id: decoder.read::<3>()?,
            options: DhcpOptions::decode(decoder)?,
        })
    }
}

impl<'a> Encodable<'a> for Message {
    fn encode(&self, e: &'_ mut Encoder<'a>) -> EncodeResult<usize> {
        let mut len = 0;
        len += e.write_u8(self.mtype.into())?;
        len += e.write(self.trans_id)?;
        len += self.options.encode(e)?;
        Ok(len)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn decode_v6() -> Result<()> {
        // decode
        // let offer = dhcp_offer();
        // let msg = Message::decode(&mut Decoder::new(&offer))?;
        // dbg!(&msg);
        // dbg!(offer.len());
        // // now encode
        // let mut buf = Vec::new();
        // let mut e = Encoder::new(&mut buf);
        // msg.encode(&mut e)?;
        // println!("{:?}", &buf);
        // println!("{:?}", &dhcp_offer());
        // // len will be different because input has arbitrary PAD bytes
        // // assert_eq!(buf.len(), dhcp_offer().len());
        // // decode again
        // let res = Message::decode(&mut Decoder::new(&buf))?;
        // // check Messages are equal after decoding/encoding
        // assert_eq!(msg, res);
        Ok(())
    }
}
