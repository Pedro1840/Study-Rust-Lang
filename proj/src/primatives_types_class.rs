fn integers_study_() {
    // Inteiro Comum
    let integer_comum = 5;
    // Inteiro com underline para organizar melhor
    let integer_organizate = 199_2332_2;
    // Hexadecimal sempre começa com "0x"
    let hexadecimal = 0xff;
    // octal sempre começa com "0o"
    let octal = 0o77;
    // binary sempre começa com "0b"
    let binary = 0b1111_11;
    // byte sempre com b'conteudo'
    let bytee = b'A';

    // Print com um conjunto de variaveis.
    println!("{}, {}, {}, {}, {}, {}", integer_comum, integer_organizate, hexadecimal, octal, binary, bytee);
}
fn floats_() {
    // Numero float por padrão o rust usa f64 
    // que tem dupla precisão.
    let x: f64 = 42.1;
}

fn booleans_() {
    // Padrão kk
    let x = true;
    let y: bool = false;
}

fn char_() {
    // Char no literal sempre usar aspas simples
    // Char não é um simples caracter, aqui a gente 
    // tem a possibilidade de colocar até 4 bits dentro
    // da tabela unicode, ele permite a utilização de simbolos.
    // O char so armazena um caracter!

    let letra = 'a';
    let caracter = '😋';

}

fn tupla_() {
    // A Tupla sempre contem tamanhos fixos
    // A Tupla tem o tipo heterogenea, podendo misturar mais de um tipo.
    // Uma vez criado a tupla não é possivel aumentar o tamnho dela
    // Indexação de Base zero igual o python ex (0, 1 , 2...)
    let numbers = (1, 2, 3.5);

    // Print para Debug para Tuplas
    // posso usar para pegar as posições:
    // println!("{:?}", numbers.2) aqui pegamos a posição 2
    println!("{:?}", numbers);

    // Podemos criar um tupla e replicar os valores de acordo com a orgem
    let (a, b, c) = numbers;
    
    // Printamos a variavel "a" da tupla
    println!("{:?}", a);

    // A tupla também é mutavel
    let mut numbers = (1, 2, 3);

    // Agora é possivel pegar um valor da tupla e mudar.
    // OBS: Não é possivel mudar a tipagem da variavel.
    // Exemplo: numbers.0 = false; dara erro porque number.0 é i32. 
    numbers.0 = 50;

    // É possivel passar mudar todos os valores de uma vez
    // Desde que siga a mesma quantidade e tipagem.
    numbers = (4, 5 ,6)
    

}

fn array_(){
    // No array todos os tipos tem que serem iguais
    let numbers = [1.1, 2.2, 3.3];

    // Pegar o numero da posição (Parecido com Python)
    println!("{:?}", numbers[0]);

    // Array também pode ser mutavel
    let mut numbers = [1.1, 2.0, 3.3];

    // Sempre seguindo a tipagem
    numbers[0] = 10.0;

    // *Operação Slice*
    // Aqui Printamos do elemento 1 até o 2
    println!("{:?}", &numbers[1..2]);

    // Aqui printamos do elemento 1 até o fim
    println!("{:?}", &numbers[1..]);

    // Aqui do começo até o elemento 2
    println!("{:?}", &numbers[..2]);

}