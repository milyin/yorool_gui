pub trait Handler<Q,R> {
    fn handle(&mut self, req: Q) -> Result<R,Q>;
}
