use std::borrow::Cow;
use std::ops::Not;

use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendPhoto<'s, 'c> {
    chat_id: ChatRef,
    photo: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<Cow<'c, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'s, 'c> Request for SendPhoto<'s, 'c> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendPhoto"), self)
    }
}

impl<'s, 'c> SendPhoto<'s, 'c> {
    pub fn with_url<C, T>(chat: C, url: T) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'s, str>>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            photo: url.into(),
            caption: None,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
        T: Into<Cow<'c, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self
    where
        R: ToMessageId,
    {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Can reply with a photo
pub trait CanReplySendPhoto {
    fn photo_url_reply<'s, 'c, T>(&self, url: T) -> SendPhoto<'s, 'c>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanReplySendPhoto for M
where
    M: ToMessageId + ToSourceChat,
{
    fn photo_url_reply<'s, 'c, T>(&self, url: T) -> SendPhoto<'s, 'c>
    where
        T: Into<Cow<'s, str>>,
    {
        let mut req = SendPhoto::with_url(self.to_source_chat(), url);
        req.reply_to(self.to_message_id());
        req
    }
}

/// Send a photo
pub trait CanSendPhoto {
    fn photo_url<'s, 'c, T>(&self, url: T) -> SendPhoto<'s, 'c>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanSendPhoto for M
where
    M: ToChatRef,
{
    fn photo_url<'s, 'c, T>(&self, url: T) -> SendPhoto<'s, 'c>
    where
        T: Into<Cow<'s, str>>,
    {
        SendPhoto::with_url(self.to_chat_ref(), url)
    }
}
