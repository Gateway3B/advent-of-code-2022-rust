pub trait Built {
    type BuilderType: Builder;

    fn builder() -> Self::BuilderType;
}

pub trait Builder {
    type BuiltType: Built;

    fn new() -> Self;
    fn build(self) -> Option<Self::BuiltType>;
}