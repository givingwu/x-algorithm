//! 中文说明：本文件位于 thunder/deserializer.rs，用于说明该模块的核心实现。
use crate::schema::{events::Event, tweet_events::TweetEvent};
use anyhow::{Context, Result};
use prost::Message;
use thrift::protocol::{TBinaryInputProtocol, TSerializable};
use xai_thunder_proto::InNetworkEvent;

/// Deserialize a Thrift binary message into TweetEvent. 中文：将 Thrift 二进制消息反序列化为 TweetEvent。
pub fn deserialize_tweet_event(payload: &[u8]) -> Result<TweetEvent> {
    let mut cursor = std::io::Cursor::new(payload);
    let mut protocol = TBinaryInputProtocol::new(&mut cursor, true);

    TweetEvent::read_from_in_protocol(&mut protocol).context("Failed to deserialize TweetEvent")
}

/// Deserialize a Thrift binary message into Event. 中文：将 Thrift 二进制消息反序列化为 Event。
pub fn deserialize_event(payload: &[u8]) -> Result<Event> {
    let mut cursor = std::io::Cursor::new(payload);
    let mut protocol = TBinaryInputProtocol::new(&mut cursor, true);

    Event::read_from_in_protocol(&mut protocol).context("Failed to deserialize Event")
}

/// Deserialize a proto binary message into InNetworkEvent. 中文：将 proto 二进制消息反序列化为 InNetworkEvent。
pub fn deserialize_tweet_event_v2(payload: &[u8]) -> Result<InNetworkEvent> {
    InNetworkEvent::decode(payload).context("Failed to deserialize InNetworkEvent")
}
