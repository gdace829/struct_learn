//加了debug注解
#[derive(Debug)]
struct People{
    name:String,
    age:i32,
    number:String,
    length:i32,
    wide:i32,
}
impl People {
    //实例函数
    fn area(&self)->i32 {
        self.length*self.wide
    }
    //关联函数
    fn handsomeboy()->People {
        People { name: String::from("sjs"), age: 23, number:String::from("1111"), length: 12, wide: 12 }
    }
}
fn main() {
    //不通过实例使用函数::
    let sjs=People::handsomeboy();
    //通过实例使用函数
    println!("{:#?} {}  {}  {} 面积为{} {}",sjs,sjs.name,sjs.age,sjs.number,People::area(&sjs),sjs.area());
    println!("Hello, world!");
}
