// // // use std::{fs::File, f32::consts::E};
// // // use std::io::ErrorKind;

// // // fn main() {
// // //     let x = 1_000.000_1; // f64
// // //     let y: f32 = 0.12; // f32
// // //     let z = 0.01_f64; // f64

// // //     assert_eq!(type_of(&x), "f64".to_string());
// // //     println!("Success!");

// // //     // 不可預期的錯誤 panic
// // //     // panic!("crash")
// // //     // 可預期的錯誤 panic
// // //     // let f = File::open("test.txt");
// // //     // let f = match f {
// // //     //     Ok(file) => file,
// // //     //     // Err(error) => {
// // //     //     //     panic!("No file: {:?}", error)
// // //     //     // }
// // //     //     // match 檢查建立檔案形態
// // //     //     Err(error) => match error.kind() {
// // //     //         NotFound => match File::create("test.txt") {
// // //     //             Ok(fc) => fc,
// // //     //             Err(e) => panic!("create file fail: {:?}", e),
// // //     //         },
// // //     //         other_error  => panic!("open file fail: {:?}", other_error),
// // //     //     },
// // //     // };
    
// // //     // 讀取檔案，可以改成如下 (not yet)
// // //     // let f = File::open("test.txt").map_err(error {
// // //     //     if error.kind() == ErrorKind::NotFound {
// // //     //         File::create("test.txt").unwrap_
// // //     //     }
// // //     // });

// // //     //  失败时触发panic的快捷方式：unwrap 和 expect
// // //     // let f = File::open("hello.txt").unwrap();
// // //     // expect 方法相较于unwrap方法多了可以添加错误信息的功能
// // //     // let f = File::open("hello.txt").expect("open fail");

// // // }

// // // fn type_of<T>(_: &T) -> String {
// // //     std::any::type_name::<T>().to_string()
// // // }


// // // trait Foo {
// // //     fn bar(&self);
// // //     fn baz(&self);
// // // }
// // // struct MyStruct;

// // // impl Foo for MyStruct {
// // //     fn bar(&self) {
// // //         // implementation goes here
// // //     }

// // //     fn baz(&self) {
// // //         // let's not worry about implementing baz() for now
// // //         todo!();
// // //     }
// // // }

// // // fn main() {
// // //     let s = MyStruct;
// // //     s.bar();

// // //     // we aren't even using baz(), so this is fine.
// // // }

// // mod ref_cell;


// // fn main() {
// //     // 使用尽可能多的方法来通过编译
// //     let x = String::from("hello, world");
// //     // let y = x;
// //     // --- answer
// //     // let y = &x;
// //     // let y = x.clone();
// //     // let x = &String::from("hello, world");
// //     let y = x.as_str();
// //     println!("{},{}",x,y);

// //     let s = String::from("hello, world");
// //     // convert String to Vec
// //     // 将 String 转换成 Vec 类型
// //     let _s = s.into_bytes();
// //     // let _s = s.clone().into_bytes();
// //     // let _s = s.as_bytes();

// //     let b = Box::new(5);
// //     assert_eq!(*b, 5);

// //     // 智能指針 Box
// //     let x = 5;
// //     let y = MyBox::new(x);

// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);

// //     // Rc<T>
// //     enum List {
// //         Cons(i32, Rc<List>),
// //         Nil
// //     }
// //     use std::rc::Rc;
// //     use List::{Cons, Nil};

// //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// //     println!("建立 a 後的計數 = {}", Rc::strong_count(&a));
// //     let b = Cons(3, Rc::clone(&a));
// //     println!("建立 b 後的計數 = {}", Rc::strong_count(&a));
// //     {
// //         let c = Cons(4, Rc::clone(&a));
// //         println!("建立 c 後的計數 = {}", Rc::strong_count(&a));
// //     }
// //     println!("c 離開作用域後的計數 = {}", Rc::strong_count(&a));
// // }

// // use std::ops::Deref;
// // struct MyBox<T>(T);
// // impl<T> MyBox<T> {
// //     fn new(x :T) -> MyBox<T> {
// //         MyBox(x)
// //     }
// // }
// // impl<T> Deref for MyBox<T> {
// //     type Target = T;
// //     fn deref(&self) -> &Self::Target {
// //         &self.0
// //     }
// // }


// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a 之後 = {:?}", a);
//     println!("b 之後 = {:?}", b);
//     println!("c 之後 = {:?}", c);
// }

use std::rc::Rc;
use std::cell::RefCell;

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(RefCell::new(1));
    let b = Rc::new(RefCell::new(2));
    let c = Rc::new(RefCell::new(3));

    let list = List::Cons(a.clone(), Rc::new(List::Cons(b.clone(), Rc::new(List::Cons(c.clone(), Rc::new(List::Nil))))));

    match list {
        List::Cons(ref a, ref rest) => {
            println!("a = {}", a.borrow());
            match **rest {
                List::Cons(ref b, ref rest) => {
                    println!("b = {}", b.borrow());
                    match **rest {
                        List::Cons(ref c, ref rest) => {
                            println!("c = {}", c.borrow());
                            match **rest {
                                List::Cons(ref d, ref rest) => {
                                    println!("d = {}", d.borrow());
                                },
                                _ => println!("End of list"),
                            }
                        },
                        _ => println!("End of list"),
                    }
                },
                _ => println!("End of list"),
            }
        },
        List::Nil => println!("End of list"),
    }
}