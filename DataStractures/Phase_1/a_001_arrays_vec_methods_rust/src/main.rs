fn main() {
    println!("------------------------------------------ Arrays ------------------------------------------");
    let mut array: [i32; 18] = [34, 73, 223, 30, 46, 95, 39, 50, 41, 49, 28, 43, 992, 483, 702, 390, 193, 495];

    println!(" len():{}\n is_empty():{}\n contains(&32):{}", array.len(), array.is_empty(), array.contains(&23));

    if let Some(index_val) = array.get(2) {
        println!(" index_val:{}", index_val)
    } else {
        println!("not found.")
    }

    println!(" array: {:?}", array);
    array.sort();
    println!(" array: {:?}", array);
    array.swap(2, 17);
    println!(" array: {:?}", array);
    array.reverse();
    println!(" array: {:?}", array);
    // array.fill(21); // replace all with 21
    // println!("array: {:?}", array);
    let some_part = &array[..4];
    let some_other_part = &array[2..8];

    println!(" some_part:{:?} [..4]\n some_other_part:{:?} [2..8]", some_part, some_other_part);

    for chunks in array.chunks(3) {
        println!(" chunks : {:?}", chunks);
    }

    println!("------------------------------------------ VECTORS ------------------------------------------");

    let vec: Vec<i32> = array.iter().filter_map(|x| if *x > 2 { Some(*x * 3) } else { None }).collect();

    let vector_double: Vec<i32> = array.iter().map(|x| x * 3).collect(); // convert array into vec.
    println!(" vector_double:{:?}", vector_double);

    let mut vec_1: Vec<i32> = Vec::with_capacity(10);
    println!(" vec_cap:{}", vec_1.capacity());
    vec_1.push(23);
    println!(" vec_1:{:?}", vec_1);
    vec_1.pop();
    println!(" vec_1:{:?}", vec_1);
    vec_1.push(1);
    vec_1.push(2);
    vec_1.push(3);
    vec_1.insert(2, 999);
    vec_1.insert(4, 2030);
    println!(" vec_1:{:?}", vec_1);
    let x = vec_1.swap_remove(0);
    println!(" x:{}", x);
    println!(" vec_1:{:?}", vec_1);

    vec_1.clear();
    println!(" vec_1:{:?}", vec_1);
    vec_1.resize(20, 3);
    println!(" vec_1:{:?}, Capicity:{}", vec_1, vec_1.capacity());

    let mut vec2 = vec![1, 2, 3, 4, 5];
    println!(" vec2:{:?}", vec2);
    vec2.truncate(3);
    println!(" vec2:{:?}", vec2);
    let drain: Vec<_> = vec2.drain(1..).map(|x| x * 3).collect();
    println!(" drain:{:?}", drain);
    println!(" vec2:{:?}", vec2);

    let mut vec3: Vec<i32> = array.iter().map(|&x| if x >= 100 { x } else { x * 4 }).collect();
    println!(" vec3:{:?}", vec3);

    let asdf: Vec<i32> = array.into_iter().collect(); // consumer
    println!("vec:{:?}", asdf);

    let mut x = vec3.split_off(4); // everyting after 4th element is in x and before that in the same vec3 
    println!(" x:{:?}", x);
    println!(" vec3:{:?}", vec3);

    vec3.append(&mut x);
    println!(" x:{:?}", x);
    println!(" vec3:{:?}", vec3);

    let vec_iter = vec_1.iter().map(|&x| x + vec_1.len() as i32);
    vec3.extend(vec_iter); // needs ITERATOR
    println!(" vec3:{:?}", vec3);

    vec3.retain(|e| e % 3 == 0);
    println!(" vec3:{:?}", vec3);
}
