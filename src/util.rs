pub enum Either<A, B>{
    A(A),
    B(B)
}

impl<A, B> Either<A, B> {

    pub fn A(a: A) -> Self {
        Either::A(a)
    }

    pub fn B(b: B) -> Self {
        Either::B(b)
    }
}
