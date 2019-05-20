pub trait Handler<MSG> {
    fn collect(&mut self) -> Vec<MSG>;
    fn handle(&mut self, cmds: Vec<MSG>) -> Vec<MSG>;
}

pub trait QueryInterface<QUERY, INTERFACE> {
    fn query_interface(&self, q: QUERY) -> Result<INTERFACE, QUERY>;
}
