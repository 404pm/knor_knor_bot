use std::{cell::RefCell, collections::HashMap, ops::Not};

use rand::{
	distributions::{Distribution, Uniform},
	rngs::ThreadRng,
};
use teloxide::{
	payloads::SendMessageSetters,
	requests::Requester,
	types::{ChatId, Message},
	Bot, RequestError,
};

type DelayMap = HashMap<ChatId, usize>;

thread_local! {
	static DELAY_MAP: RefCell<DelayMap> = RefCell::new(HashMap::new());
}

fn acquire() -> usize {
	Uniform::new(30, 50).sample(&mut ThreadRng::default())
}

fn elapsed(k: ChatId) -> bool {
	let v = DELAY_MAP.with_borrow_mut(|dy| dy.remove(&k).unwrap_or_else(acquire));

	let replace = || DELAY_MAP.with_borrow_mut(|dy| dy.insert(k, v - 1));

	(v != 0).then(replace).is_none()
}

fn replace(i: char, count: &mut usize, reply: &mut String) {
	i.is_alphanumeric().not().then(|| *count = 0);

	match count {
		0 => reply.push(i),
		1 if i.is_uppercase() => reply.push('X'),
		1 => reply.push('x'),
		2 if i.is_uppercase() => reply.push('P'),
		2 => reply.push('p'),
		_ if i.is_uppercase() => reply.push('Ю'),
		_ => reply.push('ю'),
	}
	*count += 1;
}

async fn reply(s: &str, x: Bot, m: &Message) -> Result<(), RequestError> {
	let (mut count, mut reply) = (1usize, "> ".to_string());

	s.chars().for_each(|i| replace(i, &mut count, &mut reply));

	let x = x.send_message(m.chat.id, reply);
	let x = x.reply_to_message_id(m.id);

	x.await.map(|_| ())
}

async fn check(x: Bot, m: Message) -> Result<(), RequestError> {
	let Some(s) = m.text() else { return Ok(()) };

	match elapsed(m.chat.id).then(|| reply(s, x, &m)) {
		Some(fut) => fut.await,
		None => Ok(()),
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	teloxide::repl(Bot::from_env(), check).await;
	()
}
