#[derive(Debug)]
struct Object{
    width: u32,
    height: u32,
}
//Method
impl Object {
    fn  area (&self) -> u32 {
        self.width * self.height 
    }
    fn show (&self) {
        println!("{} * {} with area : {} ", self.width, self.height, self.area());
    }
}
// Related Function
impl Object {
    fn new (width: u32 , height: u32) -> Object {
        Object {
            width,
            height,
        }    
    }
}

fn main() {
    let o = Object{
        width: 35, 
        height: 55,
    };
    let obj = Object::new(57, 85);
    o.show();
    obj.show();
    println!("{:#?}", o);
    println!("{:#?}", obj);
}
