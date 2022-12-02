use std::{ops::Add, vec, mem};
use rand::Rng;

fn main() {
    // struct Point { x: f64, y: f64 };
    // impl Point {
    //     fn distance_from_orign(&self, param1: f64) -> f64 {
    //         (self.x * self.x + self.y * self.y).sqrt() + param1
    //     }
    // }
    // let point = Point { x: 10.0, y: 10.0 };
    // println!("{}", point.distance_from_orign(55.999));

    // struct Kilometeres(f64);
    // struct Miles(f64);

    // let k = Kilometeres(55.0);
    // let k2 = Kilometeres(11.0);
    // let m = Miles(55.0);

    #[derive(Debug)]
    enum Kiss {
        Float(f64, i32),
        Int(i32),
    };

    let x: Kiss = Kiss::Float(32.5, 42);
    let y: Kiss = Kiss::Int(40);
    println!("{:?}", x);
    println!("{:?}", y);


    fn print_size(x: &[i64]) {
        for i in 0..x.len() {
            println!("{}", x[i]);
            println!("{:?}", mem::size_of_val(x));
        }
    }

    // print_size(&[11; 5]);
    // print_size(&[12, 15, 16, 17]);
    // print_size(&[5; 10]);
}

/*

fn main() {
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    let x = 5;
    let mut y;
    {
        y = &x;
        // println!("{:?}", print_type_of(&y));
        // println!("{:?}", print_type_of(&*y));
    }
    let z = *y;
    // println!("{:?}", print_type_of(&y));
    // println!("{:?}", print_type_of(&x));
    // println!("{:?}", print_type_of(&z));

    let x: Box<i32>;
    let y: Box<i32>;
    let x = Box::new(92);
    let y = &x;

    println!("{:?}", x);
    println!("{:?}", y);

    struct Point { x: f64, y: f64 };
    let vec_point = vec![Point{ x: 23.0, y: 45.5 }, Point{ x: 50.5, y: 10.0 }];
    let arr = [10; 10];
    let str = String::from("Kiss me");
    let num = 25;
    let zizi = (32,);
    // let p = &vec_point[0];
    // println!("{:?}", p as *const Point);
    println!("{:?}", &vec_point as *const Vec<Point>);
    println!("{:?}", &vec_point[0] as *const Point);
    println!("{:?}", &vec_point[0].x as *const f64);
    println!("{:?}", &arr as *const [i32; 10]);
    println!("{:?}", &str as *const String);
    println!("{:?}", num as *const i32);

    let x = 1;
    let r;
    {
        let y = 2;
        r = test_fn(&x, &y);
    }
    println!("{:?}", r);


    return;
/*

    let mut s1 = String::from("hello");
    let s2;
    let s3;
    let mut s4: i32 = 25;
    let s5: *mut i32 = s4 as *mut i32;
    println!("{:?}", s4);
    println!("{:?}", s5);
    s4 = s4.wrapping_add(5);
    println!("{:?}", s4);
    println!("{:?}", s5);

    let mut i = 32;
    let mut_ref = &mut i;
    *mut_ref = *mut_ref + 55;

    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;
    let c1 = *my_num_ptr;
    let c2 = *my_speed_ptr;

    return;
    //s2 = &mut s1;
    // pushStr(&mut s1);
    s1.push_str(" New");
    s2 = &mut s1;
    s3 = &s1;
    println!("{:?}", s1.len());
    println!("{:?}", s3.len());
    return;

    let tuple = (90,);

    let pepe: &mut i32 = &mut 555;
    let pepe2: &mut i32 = &mut 91;
    let pepe3: &mut i32 = &mut 444;
    // let pepe: i32 = kiss(pepe2);

    println!("test {:?}", tuple);
    println!("test {:?}", &tuple as *const (i32,));
    println!("test {:?}", &tuple.0 as *const i32);
    */
        
}

fn kiss (z: &mut i32 ) -> i32 {
    return z.wrapping_add(2);
}

fn pushStr(s: &mut String) {
    s.push_str(" New");
}

fn test_fn<'att, 'btt>(x: &'att i32, y: &'btt i32) -> &'att i32 {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    println!("{:?}", n1);
    if n1 > 1 {
        return x;
    } else {
        return y;
    }
}

 */