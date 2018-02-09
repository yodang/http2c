#[allow(non_camel_case_types)]
pub enum FrameType
{
    DATA,
    HEADERS,
    PRIORITY,
    RST_STREAM,
    SETTINGS,
    PUSH_PROMISE,
    PING,
    GOAWAY,
    WINDOW_UPDATE,
    CONTINUATION
}


pub struct Http2Frame
{
    http_type: FrameType,
    identifier: u32,
    payload: Vec<u8>,
}
