pub trait JoinSemilattice: Sized + Clone {
    fn bot() -> Self;

    fn join(a: &Self, b: &Self) -> Self;
}
