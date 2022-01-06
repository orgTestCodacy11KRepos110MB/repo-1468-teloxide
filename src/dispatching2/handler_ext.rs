use crate::{dispatching2::HandlerFactory, types::Message, utils::command::BotCommand};
use dptree::{di::DependencyMap, Handler};

pub trait HandlerExt<Output> {
    #[must_use]
    fn add_command<C>(self, bot_name: String) -> Self
    where
        C: BotCommand + Send + Sync + 'static;

    #[must_use]
    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>;
}

impl<Output> HandlerExt<Output> for Handler<'static, DependencyMap, Output>
where
    Output: Send + Sync + 'static,
{
    fn add_command<C>(self, bot_name: String) -> Self
    where
        C: BotCommand + Send + Sync + 'static,
    {
        self.chain(dptree::filter_map(move |message: Message| {
            let bot_name = bot_name.clone();
            async move { message.text().and_then(|text| C::parse(text, bot_name).ok()) }
        }))
    }

    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>,
    {
        self.chain(F::handler())
    }
}
