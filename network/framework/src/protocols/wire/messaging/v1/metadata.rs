// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::protocols::wire::{
    handshake::v1::ProtocolId,
    messaging::v1::{MultiplexMessage, NetworkMessage},
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// A simple struct that wraps a network message with metadata.
/// Note: this is not sent along the wire, it is only used internally.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NetworkMessageWithMetadata {
    /// The metadata about the message
    message_metadata: MessageMetadata,

    /// The network message to send along the wire
    network_message: NetworkMessage,
}

impl NetworkMessageWithMetadata {
    pub fn new(message_metadata: MessageMetadata, network_message: NetworkMessage) -> Self {
        Self {
            message_metadata,
            network_message,
        }
    }

    /// Converts the message into a multiplex message with metadata
    pub fn into_multiplex_message(self) -> MultiplexMessageWithMetadata {
        MultiplexMessageWithMetadata::new(
            self.message_metadata,
            MultiplexMessage::Message(self.network_message),
        )
    }

    /// Consumes the message and returns the individual parts
    pub fn into_parts(self) -> (MessageMetadata, NetworkMessage) {
        (self.message_metadata, self.network_message)
    }

    /// Returns a reference to the message metadata
    pub fn network_message(&self) -> &NetworkMessage {
        &self.network_message
    }
}

/// A simple struct that wraps a multiplex message with metadata.
/// Note: this is not sent along the wire, it is only used internally.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MultiplexMessageWithMetadata {
    /// The metadata about the message
    message_metadata: MessageMetadata,

    /// The multiplex message to send along the wire
    multiplex_message: MultiplexMessage,
}

impl MultiplexMessageWithMetadata {
    pub fn new(message_metadata: MessageMetadata, multiplex_message: MultiplexMessage) -> Self {
        Self {
            message_metadata,
            multiplex_message,
        }
    }

    /// Consumes the message and returns the individual parts
    pub fn into_parts(self) -> (MessageMetadata, MultiplexMessage) {
        (self.message_metadata, self.multiplex_message)
    }
}

/// A simple enum to track the type of message being sent
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum MessageType {
    RegularMessage,          // A regular message that fits into a single chunk
    StreamedMessageHead,     // The head (first fragment) of a streamed message
    StreamedMessageFragment, // A fragment of a streamed message (not the head or tail)
    StreamedMessageTail,     // The tail (last fragment) of a streamed message
}

/// A struct holding metadata about each message.
/// Note: this is not sent along the wire, it is only used internally.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageMetadata {
    /// The protocol ID for the message
    protocol_id: ProtocolId,

    /// The type of the message being sent
    message_type: MessageType,

    /// The time at which the message was sent by the application
    application_send_time: Option<SystemTime>,

    /// The time at which the message was sent along the network wire
    network_wire_send_time: Option<SystemTime>,
}

impl MessageMetadata {
    pub fn new(protocol_id: ProtocolId, application_send_time: Option<SystemTime>) -> Self {
        Self {
            protocol_id,
            application_send_time,
            message_type: MessageType::RegularMessage,
            network_wire_send_time: None,
        }
    }

    /// Returns the time at which the message was first sent by the application
    pub fn application_send_time(&self) -> Option<SystemTime> {
        self.application_send_time
    }

    /// Updates the network wire send time
    pub fn update_network_send_time(&mut self) {
        self.network_wire_send_time = Some(SystemTime::now());
    }

    /// Updates the message type
    pub fn update_message_type(&mut self, message_type: MessageType) {
        self.message_type = message_type;
    }
}
