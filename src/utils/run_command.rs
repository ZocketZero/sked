pub trait RunCommand {
    fn run(&self) -> impl Future<Output = ()>;
}
