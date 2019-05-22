pub trait Unpack<E:Default> {
    fn unpack(self, f: fn(E) -> Self) -> Result<E,Self> where Self: Sized;
}

pub enum Event<T,S> {
    None,
    Changed,
    QueryState,
    State(S),
    SetState(S),
    Custom(T)
}

impl<T,S> Default for Event<T,S> {
    fn default() -> Self { Event::None }
}

pub trait MessageHandler<MSG> {
    type T;
    type S : Default;
    fn pack(&self, _e: Event<Self::T,Self::S>) -> Option<MSG> { None }
    fn unpack(&self, m: MSG) -> Result<Event<Self::T,Self::S>, MSG> { Err(m) }
    fn handle_custom(&mut self, _e: Self::T) -> Option<Self::T> { None }
    fn get_state(&self) -> Self::S { Self::S::default() }
    fn set_state(&mut self, _s: Self::S) {}
    fn collect(&mut self) -> Vec<MSG> { Vec::new() }
    fn handle(&mut self, input: Vec<MSG>) -> Vec<MSG> {
        let mut output = Vec::new();
          for msg in input {
            match self.unpack(msg) {
                Ok(Event::QueryState) => {
                    if let Some(msg) = self.pack(Event::State(self.get_state())) {
                        output.push(msg)
                    }
                },
                Ok(Event::SetState(s)) => self.set_state(s),
                Ok(Event::Custom(m)) => {
                    if let Some(r) = self.handle_custom(m) {
                        if let Some(msg) = self.pack(Event::Custom(r)) {
                            output.push(msg)
                        }
                    }
                }
                Err(msg) => output.push(msg),
                _ => {}
            }
        }
        output
    }
}

pub trait IMessageHandler<MSG> {
    fn collect(&mut self) -> Vec<MSG>;
    fn handle(&mut self, msgs: Vec<MSG> ) -> Vec<MSG>;
}

impl<W,MSG,T,S> IMessageHandler<MSG> for W where W: MessageHandler<MSG,T=T,S=S>, S: Default {
    fn collect(&mut self) -> Vec<MSG> {
        let s : &mut W = self;
        s.collect()
    }
    fn handle(&mut self, msgs: Vec<MSG> ) -> Vec<MSG> {
        let s : &mut W = self;
        s.handle(msgs)
    }
}
