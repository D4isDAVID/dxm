use std::{error::Error, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de::Visitor};

pub const CFX_CRITICAL: &str = "critical";
pub const CFX_RECOMMENDED: &str = "recommended";
pub const CFX_OPTIONAL: &str = "optional";
pub const CFX_LATEST: &str = "latest";
pub const JG_LATEST: &str = "latest-jg";

/// The possible FXServer installation update channels.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactsChannel {
    Critical,
    Recommended,
    Optional,
    Latest,
    #[default]
    LatestJg,
}

#[derive(Debug)]
pub struct ParseArtifactsChannelError {
    channel: String,
}

impl Display for ParseArtifactsChannelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown artifacts channel {}", &self.channel)?;

        Ok(())
    }
}

impl Error for ParseArtifactsChannelError {}

impl FromStr for ArtifactsChannel {
    type Err = ParseArtifactsChannelError;

    fn from_str(channel: &str) -> Result<Self, Self::Err> {
        match channel {
            CFX_CRITICAL => Ok(Self::Critical),
            CFX_RECOMMENDED => Ok(Self::Recommended),
            CFX_OPTIONAL => Ok(Self::Optional),
            CFX_LATEST => Ok(Self::Latest),
            JG_LATEST => Ok(Self::LatestJg),
            _ => Err(ParseArtifactsChannelError {
                channel: channel.to_owned(),
            }),
        }
    }
}

impl Display for ArtifactsChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ArtifactsChannel::Critical => CFX_CRITICAL,
            ArtifactsChannel::Recommended => CFX_RECOMMENDED,
            ArtifactsChannel::Optional => CFX_OPTIONAL,
            ArtifactsChannel::Latest => CFX_LATEST,
            ArtifactsChannel::LatestJg => JG_LATEST,
        };

        write!(f, "{str}")?;

        Ok(())
    }
}

impl Serialize for ArtifactsChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ChannelVisitor;

impl Visitor<'_> for ChannelVisitor {
    type Value = ArtifactsChannel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("critical, recommended, optional, latest, or latest-jg")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            CFX_CRITICAL => Ok(ArtifactsChannel::Critical),
            CFX_RECOMMENDED => Ok(ArtifactsChannel::Recommended),
            CFX_OPTIONAL => Ok(ArtifactsChannel::Optional),
            CFX_LATEST => Ok(ArtifactsChannel::Latest),
            JG_LATEST => Ok(ArtifactsChannel::LatestJg),
            _ => Err(E::custom("invalid artifacts channel")),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }
}

impl<'de> Deserialize<'de> for ArtifactsChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ChannelVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_value_when_valid() {
        assert_eq!(
            ArtifactsChannel::from_str("latest-jg").unwrap(),
            ArtifactsChannel::LatestJg
        );
    }

    #[test]
    #[should_panic = "ParseArtifactsChannelError"]
    fn parse_returns_error_when_invalid() {
        ArtifactsChannel::from_str("").unwrap();
    }

    #[test]
    fn visitor_returns_value_when_valid() {
        let visitor = ChannelVisitor {};

        assert_eq!(
            visitor
                .visit_str::<serde::de::value::Error>("latest-jg")
                .unwrap(),
            ArtifactsChannel::LatestJg
        );
    }

    #[test]
    #[should_panic = "invalid artifacts channel"]
    fn visitor_returns_error_when_invalid() {
        let visitor = ChannelVisitor {};

        visitor.visit_str::<serde::de::value::Error>("").unwrap();
    }
}
