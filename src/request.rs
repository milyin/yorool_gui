pub trait Unpack<E:Default> {
    fn unpack(self, f: fn(E) -> Self) -> Result<E,Self> where Self: Sized;
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
        s.collect_impl()
    }
    fn handle(&mut self, msgs: Vec<MSG> ) -> Vec<MSG> {
        let s : &mut W = self;
        s.handle_impl(msgs)
    }
}

pub struct MessageProcessor<'a, MSG> {
    msgs_proc: Box<Fn(Vec<MSG>) -> Vec<MSG> + 'a>,
    msg_procs: Vec<Box<Fn(MSG) -> Result<Vec<MSG>, MSG> + 'a>>
}

impl <'a,MSG> MessageProcessor<'a,MSG> {
    pub fn new() -> Self {
        Self {
            msgs_proc: Box::new(|q| q ),
            msg_procs: Vec::new()
        }
    }
    pub fn set_msgs_proc<F: Fn(Vec<MSG>) -> Vec<MSG> + 'a>(mut self, f:F) -> Self {
        self.msgs_proc = Box::new(f);
        self
    }
    pub fn add_msg_proc<F:Fn(MSG) -> Result<Vec<MSG>, MSG> + 'a>(mut self, f:F) -> Self {
        self.msg_procs.push(Box::new(f));
        self
    }
    pub fn process(&self, msgs: Vec<MSG>) -> Vec<MSG> {
        let mut ret = Vec::new();
        for mut msg in msgs {
            for proc in &self.msg_procs {
                match proc(msg) {
                    Ok(mut r) => { ret.append(&mut r); break; }
                    Err(m) => msg = m
                }
            }
        }
        ret
    }
}