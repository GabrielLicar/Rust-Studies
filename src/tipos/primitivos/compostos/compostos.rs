fn main() {
    // Exemplo de tipo: (tupla)
    let numbers: (i32, i32, f64) = (1, 2, 3.5);
    
    let (a, b, c) = numbers;

    println!("{:?}", a);

    // Exemplo de tipo: (tupla mutável)
    let mut numbers: (i32, i32, f64) = (1, 2, 3.5);

    numbers = (numbers.0, 7, 7.7);
    numbers.0 = 50;

    println!("{:?}", numbers.0);
    println!("{:?}", numbers);

    // Exemplo de tipo: (matriz)

    let numbers: [f64;3] = [1.1, 2.2, 3.3];

    println!("{:?}", numbers[0]);
    println!("{:?}", numbers);

    // Exemplo de tipo: (matriz mutável)

    let mut numbers: [f64;3] = [1.1, 2.2, 3.3];
    numbers[0] = 77.7;

    println!("{:?}", numbers[0]);
    println!("{:?}", numbers);

    /**
     * Algumas operações com arrays.
     * Slice/Slicing
     */
    println!("{:?}", &numbers[1..2]); // a partir do elemento 1 até o elemento 2.
    println!("{:?}", &numbers[1..]); // a partir do elemento 1 até o ultimo.
    println!("{:?}", &numbers[..2]); // começar do inicio do array e ir até o elemento 2.
}