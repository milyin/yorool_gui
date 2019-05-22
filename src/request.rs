pub trait Unpack<E:Default> {
    fn unpack(self, f: fn(E) -> Self) -> Result<E,Self> where Self: Sized;
    fn peek(&self, f: fn(E) -> Self) -> Option<&E>;
}

#[derive(Debug)]
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
    fn collect_impl(&mut self) -> Vec<MSG> { Vec::new() }
    fn handle_impl(&mut self, input: Vec<MSG>) -> Vec<MSG> {
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
                Ok(e) => {
                    if let Some(m) = self.pack(e) {
                        output.push(m)
                    }
                },
                Err(msg) => output.push(msg),
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
        s.collect_impl()
    }
    fn handle(&mut self, msgs: Vec<MSG> ) -> Vec<MSG> {
        let s : &mut W = self;
        s.handle_impl(msgs)
    }
}

pub struct MessageProcessor<'a, MSG> {
    multiples: Vec<Box<Fn(Vec<MSG>) -> Vec<MSG> + 'a>>,
    singles: Vec<Box<Fn(MSG) -> Result<Vec<MSG>, MSG> + 'a>>
}

impl <'a,MSG> MessageProcessor<'a,MSG> {
    pub fn new() -> Self {
        Self {
            multiples: Vec::new(),
            singles: Vec::new()
        }
    }
    pub fn add_multiple<F: Fn(Vec<MSG>) -> Vec<MSG> + 'a>(mut self, f:F) -> Self {
        self.multiples.push(Box::new(f));
        self
    }
    pub fn add_single<F:Fn(MSG) -> Result<Vec<MSG>, MSG> + 'a>(mut self, f:F) -> Self {
        self.singles.push(Box::new(f));
        self
    }
    pub fn process(&self, mut msgs: Vec<MSG>) -> Vec<MSG> {
        for proc in &self.multiples {
           msgs = proc(msgs)
        }
        let mut ret = Vec::new();
        for msg in msgs {
            let mut keep_msg = Some(msg);
            for proc in &self.singles {
                match proc(keep_msg.unwrap()) {
                    Ok(mut r) => { ret.append(&mut r); keep_msg = None; break; }
                    Err(m) => keep_msg = Some(m)
                }
            }
            if let Some(msg) = keep_msg {
                ret.push(msg)
            }
        }
        ret
    }
}

pub fn is_changed<T,S, MSG: Unpack<Event<T,S>>>(id: fn(Event<T,S>) -> MSG, msgs: &Vec<MSG>) -> bool {
    for msg in msgs {
        if let Some(ref e) = msg.peek(id) {
            match e {
                Event::Changed => return true,
                _ => {}
            }
        }
    }
    false
}

pub fn get_state<'a,T:'a,S,MSG: Unpack<Event<T,S>>>(id: fn(Event<T,S>) -> MSG, msgs: &'a Vec<MSG>) -> Option<&'a S> {
    for msg in msgs {
        if let Some(ref e) = msg.peek(id) {
            match e {
                Event::State(ref s) => return Some(s),
                _ => {}
            }
        }
    }
    None
}

