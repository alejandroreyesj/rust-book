

crate::

#[macro_export]
macro_rules! i_vec {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }

    };
}

fn main() {
    println!("Hello, world!");
    let y = i_vec!(1, 2, 3);
    println!("{y:?}")
    Hello
    
}



