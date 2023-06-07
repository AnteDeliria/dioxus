use crate::{AttributeValue, Scope};

/// Describes media
#[derive(Debug, PartialEq)]
pub struct Media {
    source: MediaSource,
}

impl Media {
    /// Create a new MediaSource
    pub fn new(source: MediaSource) ->  Self {
        Self { source }
    }

    /// Get the source
    pub fn source(&self) -> &MediaSource {
        &self.source
    }

    /// Finish building the MediaSource and turn it into a useable value
    pub fn finish(self, cx: Scope) -> AttributeValue {
        cx.any_value(self)
    }
} 

/// Describes a source of media
#[derive(Debug, PartialEq)]
pub enum MediaSource {
    /// Import the media via URL
    Url(String),
    /// Raw data to be used as a media's source
    Raw((String, Vec<u8>)),
    /// A stream of raw data
    #[cfg(not(target_family = "wasm"))]
    Stream(MediaStream),

    /// A wrapper around the web MediaStream
    #[cfg(target_family = "wasm")]
    Stream(web_sys::MediaStream),
}

#[derive(Debug, PartialEq)]
pub struct MediaStream {}
