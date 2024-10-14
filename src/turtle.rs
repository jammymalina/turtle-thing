mod turtle {
    struct Vec2<T>
    where
        T: num::Num,
    {
        x: T,
        y: T,
    }

    pub struct Turtle<T>
    where
        T: num::Num, {}
}
