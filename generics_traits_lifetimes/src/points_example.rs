// here the struct is defined as generic over two types; if this were
// Point<T> instead, then x and y would have to be the same type.
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// the <X1, Y1> is needed after the impl because we need the compiler to know
// that X1 and Y1 are the generics over which the Point struct is defined.
impl<X1, Y1> Point<X1, Y1> {
    // same here - we need <X2, Y2> after the function name so that
    // the compiler knows that the other parameter is a Point struct generic
    // over types X2 and Y2.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
