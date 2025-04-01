use leptos::{*, html::Div};

use crate::model::conversation::Conversation;

const USER_MESSAGE_DARK_MODE_COLORS: &str = "bg-blue-500 text-white";
const USER_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-blue-200 text-black";
const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end";

const MODEL_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-gray-200 text-black";
const MODEL_MESSAGE_DARK_MODE_COLORS: &str = "bg-zinc-700 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start";

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_LIGHT_MODE_COLORS: &str = "border-gray-300 bg-gray-100";
const CHAT_AREA_DARK_MODE_COLORS: &str = "border-gray-700 bg-zinc-900";


#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>().expect("should be able to get dark mode state");

    let user_message_class = Signa::derive(move || {
        if dark_mode.get() {
            format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_DARK_MODE_COLORS}")
            
        }
    })
}