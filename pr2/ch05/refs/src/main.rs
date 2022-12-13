use refs::S;

fn main() {
    let s;
    {
        let x = 128;
        s = S { r: &x };
        assert!(s.r == &128);
    }
    //assert!(s.r == &128);
}
