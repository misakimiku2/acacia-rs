use crate::{
    core,
    protocol::{self, Pf},
};
use prost::Message;

macro_rules! define_proto_dispatch {
    ($($variant:ident($msg_type:ty) = $pf:expr),* $(,)?) => {
        #[derive(Clone, Debug)]
        pub enum Msg {
            $($variant($msg_type)),*
        }
        impl Msg {
            pub fn msg_id(&self) -> i32 {
                match self {
                    $(Self::$variant(_) => $pf as i32),*
                }
            }
            pub fn encode_to_vec(&self) -> Vec<u8> {
                match self {
                    $(Self::$variant(m) => prost::Message::encode_to_vec(m)),*
                }
            }
            pub fn decode(msg_id: i32, buf: &[u8]) -> Result<Self, prost::DecodeError> {
                match msg_id {
                    $(x if x == $pf as i32 => {
                        Ok(Self::$variant(<$msg_type>::decode(buf)?))
                    }),*
                    _ => Err(prost::DecodeError::new(
                        format!("unknown msg_id: {msg_id}")
                    )),
                }
            }
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant(_) => stringify!($variant)),*
                }
            }
            pub fn name_by_id(msg_id: i32) -> Option<&'static str> {
                match msg_id {
                    $(x if x == $pf as i32 => Some(stringify!($variant)),)*
                    _ => None,
                }
            }
        }
    };
}

define_proto_dispatch! {
    None(core::Empty)                                = Pf::None,               // 0
    ReqLogin(protocol::ReqLogin)                     = Pf::ReqLogin,           // 1
    RspLogin(protocol::RspLogin)                     = Pf::RspLogin,           // 2
    ReqReconnect(protocol::ReqReconnect)             = Pf::ReqReconnect,       // 3
    RspReconnect(protocol::RspReconnect)             = Pf::RspReconnect,       // 4
    ReqCallgs(protocol::ReqCallGs)                   = Pf::ReqCallgs,          // 7
    RspCallgs(core::Empty)                           = Pf::RspCallgs,          // 8
    ReqUseitem(protocol::ReqUseItem)                 = Pf::ReqUseitem,         // 9
    ReqAccountinfo(protocol::ReqAccountInfo)         = Pf::ReqAccountinfo,     // 19
    RspRanklist(protocol::RankList)                  = Pf::RspRanklist,        // 52
    RspRank(protocol::RankInfo)                      = Pf::RspRank,            // 54
    RspGlobalcounter(protocol::GlobalCounterInfo)    = Pf::RspGlobalcounter,   // 62

    ReqOnlineRoom(protocol::ReqOnlineCreateRoom)     = Pf::ReqOnlineRoom,      // 303
    RspOnlineRoom(protocol::RspOnlineCreateRoom)     = Pf::RspOnlineRoom,      // 304
    ReqOnlineRoomExit(core::Empty)                   = Pf::ReqOnlineRoomExit,  // 308
    ReqOnlineRoomAccept(protocol::ReqOnlineAccept)   = Pf::ReqOnlineRoomAccept,           // 311
    ReqOnlineRoomChataccept(protocol::ReqOnlineChatAccept) = Pf::ReqOnlineRoomChataccept, // 317
    ReqChangeWorldChannel(protocol::ReqOnlineRecruit) = Pf::ReqChangeWorldChannel,        // 321
    RspChangeWorldChannel(protocol::ReqOnlineRecruit) = Pf::RspChangeWorldChannel,        // 322
    ReqOnlineRecruit(protocol::ReqOnlineRecruit)     = Pf::ReqOnlineRecruit,   // 329

    NtfLog(protocol::NtfLog)                         = Pf::NtfLog,             // 1001
    NtfBroadcast(protocol::NtfBroadcast)             = Pf::NtfBroadcast,       // 1003
    NtfSyncattr(protocol::NtfSyncPlayer)             = Pf::NtfSyncattr,        // 1004
    NtfSynclineup(protocol::NtfSyncLineup)           = Pf::NtfSynclineup,      // 1005
    NtfSyncNewMail(core::Mail)                       = Pf::NtfSyncNewMail,     // 1006
    NtfScript(protocol::NtfCallScript)               = Pf::NtfScript,          // 1010
    NtfSetattr(protocol::NtfSetAttr)                 = Pf::NtfSetattr,         // 1011
    NtfSetstrattr(protocol::NtfSetStrAttr)           = Pf::NtfSetstrattr,      // 1012
    NtfOnlineStart(protocol::NtfOnlineStart)         = Pf::NtfOnlineStart,     // 1013
    NtfReaditem(core::IdArray)                       = Pf::NtfReaditem,        // 1015
    NtfFriendReq(core::PlayerProfileArray)           = Pf::NtfFriendReq,       // 1018
    NtfFriendVigor(protocol::FriendVigorList)        = Pf::NtfFriendVigor,     // 1019
    NtfAntiData(protocol::ReqAntiData)               = Pf::NtfAntiData,        // 1022
    NtfBlockFriendreq(protocol::FriendVigorList)     = Pf::NtfBlockFriendreq,  // 1023
    NtfCustomroster(core::CustomRoster)              = Pf::NtfCustomroster,    // 1024
    NtfOnlineRoominfo(protocol::NtfOnlineRoom)       = Pf::NtfOnlineRoominfo,  // 1031
    NtfOnlineInvite(protocol::NtfOnlineInvite)       = Pf::NtfOnlineInvite,    // 1034
    NtfOnlineState(protocol::NtfOnlineState)         = Pf::NtfOnlineState,     // 1035
    NtfOnlineRecruit(protocol::NtfOnlineRecruit)     = Pf::NtfOnlineRecruit,   // 1044

    ReqRoomStart(protocol::ReqRoomStart)             = Pf::ReqRoomStart,       // 2001
    RspRoomStart(protocol::RspRoomStart)             = Pf::RspRoomStart,       // 2002
    NtfStopRoom(protocol::NtfStopRoom)               = Pf::NtfStopRoom,        // 2004
    NtfRoomOver(protocol::NtfRoomOver)               = Pf::NtfRoomOver,        // 2005
    NtfRoomPlayercheat(protocol::NtfRoomPlayerCheat) = Pf::NtfRoomPlayercheat, // 2007
}
