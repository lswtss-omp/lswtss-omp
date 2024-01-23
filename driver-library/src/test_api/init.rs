use super::*;

pub fn init()
{
    let (requests_channel_sender, requests_channel_receiver) =
        tokio::sync::mpsc::unbounded_channel::<RequestInfo>();

    unsafe {
        REQUESTS_CHANNEL_SENDER_OPTION = Some(requests_channel_sender);
    }

    unsafe {
        REQUESTS_CHANNEL_RECEIVER_OPTION = Some(requests_channel_receiver);
    }
}
