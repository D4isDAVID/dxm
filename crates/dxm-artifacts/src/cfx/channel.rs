use std::{error::Error, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de::Visitor};

/// The possible FXServer installation update channels.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
        write!(f, "failed to parse artifacts channel {}", &self.channel)?;

        Ok(())
    }
}

impl Error for ParseArtifactsChannelError {}

impl FromStr for ArtifactsChannel {
    type Err = ParseArtifactsChannelError;

    fn from_str(channel: &str) -> Result<Self, Self::Err> {
        match channel {
            "critical" => Ok(Self::Critical),
            "recommended" => Ok(Self::Recommended),
            "optional" => Ok(Self::Optional),
            "latest" => Ok(Self::Latest),
            "latest-jg" => Ok(Self::LatestJg),
            _ => Err(ParseArtifactsChannelError {
                channel: channel.to_owned(),
            }),
        }
    }
}

impl Display for ArtifactsChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ArtifactsChannel::Critical => "critical",
            ArtifactsChannel::Recommended => "recommended",
            ArtifactsChannel::Optional => "optional",
            ArtifactsChannel::Latest => "latest",
            ArtifactsChannel::LatestJg => "latest-jg",
        };

        write!(f, "{}", str)?;

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
            "critical" => Ok(ArtifactsChannel::Critical),
            "recommended" => Ok(ArtifactsChannel::Recommended),
            "optional" => Ok(ArtifactsChannel::Optional),
            "latest" => Ok(ArtifactsChannel::Latest),
            "latest-jg" => Ok(ArtifactsChannel::LatestJg),
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
