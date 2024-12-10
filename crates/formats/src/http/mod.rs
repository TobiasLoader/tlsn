//! Tooling for working with HTTP data.

mod commit;

use bytes::Bytes;
pub use commit::{DefaultHttpCommitter, HttpCommit, HttpCommitError};

#[doc(hidden)]
pub use spansy::http;

pub use http::{
    parse_request, parse_response, Body, BodyContent, Header, HeaderName, HeaderValue, Method,
    Reason, Request, RequestLine, Requests, Response, Responses, Status, Target,
};
use tlsn_core::transcript::Transcript;

/// The kind of HTTP message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKind {
    /// An HTTP request.
    Request,
    /// An HTTP response.
    Response,
}

/// An HTTP transcript.
#[derive(Debug)]
pub struct HttpTranscript {
    /// The requests sent to the server.
    pub requests: Vec<Request>,
    /// The responses received from the server.
    pub responses: Vec<Response>,
}

impl HttpTranscript {
    /// Parses the HTTP transcript from the provided transcripts.
    pub fn parse(transcript: &Transcript) -> Result<Self, spansy::ParseError> {

        println!("Parse requests");
        println!("Transcript: {:?}", transcript.sent());
        println!("Transcript length: {:?}", transcript.sent().len());
        println!("Bytes: {:?}", Bytes::copy_from_slice(transcript.sent()));
        println!("Requests: {:?}", Requests::new(Bytes::copy_from_slice(transcript.sent())));
        println!("UTF-8: {:?}", String::from_utf8(Bytes::copy_from_slice(transcript.sent()).to_vec()));

        let requests = Requests::new(Bytes::copy_from_slice(transcript.sent()))
            .collect::<Result<Vec<_>, _>>()?;

        println!("Parse responses");
        println!("Transcript: {:?}", transcript.received());
        println!("Transcript length: {:?}", transcript.received().len());
        println!("Bytes: {:?}", Bytes::copy_from_slice(transcript.received()));
        println!("Responses: {:?}", Responses::new(Bytes::copy_from_slice(transcript.received())));
        println!("UTF-8: {:?}", String::from_utf8(Bytes::copy_from_slice(transcript.received()).to_vec()));
        
        let responses = Responses::new(Bytes::copy_from_slice(transcript.received()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            requests,
            responses,
        })
    }
}
