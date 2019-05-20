pub trait Handler<MSG,CMD> where CMD: Clone {
    fn collect(&mut self) -> Vec<MSG>;
    fn handle(&mut self, cmds: &[CMD]);
}

pub trait QueryInterface<QUERY, INTERFACE> {
    fn query_interface(&self, q: QUERY) -> Result<INTERFACE, QUERY>;
}
