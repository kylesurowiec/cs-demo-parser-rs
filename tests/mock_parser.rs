use std::collections::VecDeque;
use std::io::Cursor;

use cs_demo_parser::dispatcher::HandlerIdentifier;
use cs_demo_parser::events::FrameDone;
use cs_demo_parser::parser::Parser;

pub struct MockParser {
    parser: Parser<Cursor<Vec<u8>>>,
    queue: VecDeque<Box<dyn FnOnce(&mut Parser<Cursor<Vec<u8>>>) + Send>>,
}

impl MockParser {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(Cursor::new(Vec::new())),
            queue: VecDeque::new(),
        }
    }

    pub fn feed_event<E>(&mut self, event: E)
    where
        E: Send + Sync + 'static,
    {
        self.queue
            .push_back(Box::new(move |p| p.dispatch_event(event)));
    }

    pub fn feed_net_message<M>(&mut self, msg: M)
    where
        M: Send + Sync + 'static,
    {
        self.queue
            .push_back(Box::new(move |p| p.dispatch_net_message(msg)));
    }

    pub fn parse_next_frame(&mut self) -> bool {
        if let Some(cb) = self.queue.pop_front() {
            cb(&mut self.parser);
            self.parser.dispatch_event(FrameDone);
            true
        } else {
            false
        }
    }

    pub fn parse_to_end(&mut self) {
        while self.parse_next_frame() {}
    }

    pub fn register_event_handler<E, F>(&self, handler: F) -> HandlerIdentifier
    where
        E: Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static,
    {
        self.parser.register_event_handler::<E, F>(handler)
    }

    pub fn register_net_message_handler<M, F>(&self, handler: F) -> HandlerIdentifier
    where
        M: Send + Sync + 'static,
        F: Fn(&M) + Send + Sync + 'static,
    {
        self.parser.register_net_message_handler::<M, F>(handler)
    }

    pub fn unregister_event_handler(&self, id: HandlerIdentifier) {
        self.parser.unregister_event_handler(id);
    }

    pub fn unregister_net_message_handler(&self, id: HandlerIdentifier) {
        self.parser.unregister_net_message_handler(id);
    }

    pub fn game_state(&self) -> &cs_demo_parser::game_state::GameState {
        self.parser.game_state()
    }
}

impl Default for MockParser {
    fn default() -> Self {
        Self::new()
    }
}
