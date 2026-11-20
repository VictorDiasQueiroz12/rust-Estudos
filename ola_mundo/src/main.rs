use std::result;

static mut GLOBAL:u8 = 1;

fn sombra(){
    let a:i32 = 123;

    {
        let b:i32 = 456;
        println!("dentro, b = {}", b);

        let a:i32 = 789; //a é uma variavel diferente da que foi declarada fora do escopo, pois foi redeclarada dentro do escopo
        println!("dentro, a = {}", a); //a pode ser acessada dentro do escopo, pois foi declarada fora dele
    }
    //println!("fora, b = {}", b); vai dar erro, pois b só existe dentro do escopo acima

    println!("fora, a = {}", a);

}
fn escopo(){
    const PI:f32 = 3.14;
    println!("PI = {}", PI);

    let _string:&'static str = "Victor";//estudar ainda 

    unsafe {
        let ptr = &raw const GLOBAL;
        println!("Variavel global = {}", *ptr);
    }

    let variavel:i32 = 300;
    println!("Variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let variavel:i32 = 301;
    println!("Variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("Decimal = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));

    let booleana:bool = true;
    println!("Booleana = {}, tamanho = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'V';
    println!("Letra = {}, tamanho = {}", letra, std::mem::size_of_val(&letra));

}

fn soma(a:i32, b:i32) -> i32{ //-> i32 é o tipo de retorno da função //quando a função retorna um valor, é necessário colocar o tipo de retorno da função, e o valor retornado deve ser do mesmo tipo
    println!("Soma: {} + {} = {}", a, b, a+b);
    return a+b;
}


fn condicionais(){
    let idade:i8 = 16;
    let autorizado:bool = true;
    let eh_maior:bool = idade >= 18;

    if eh_maior{
        println!("Pode entrar na festa!");
    } else if idade >= 16 && autorizado{
        println!("Pode entrar na festa, mas com autorização!");
    }else{
        println!("Não pode entrar na festa!");
    }

    let condicao:&str = if eh_maior {"maior"} else {"menor"};
    println!("É {} de idade", condicao);

    let linguagem:&str = "JS";
    let proposito:&str = match linguagem {
        "Rust" => "Performace",
        "Python" => "Facilidade",
        "Java" => "Portabilidade",
        _ => "Desconhecida"
    };

    println!("A linguagem {} tem como proposito {}", linguagem, proposito);

}

fn loops(){
    let multiplicador:u8 = 2;

    let mut contador:u8 = 0;
    while  contador < 10{
        if contador == 5{
            contador += 1;
            continue;
        }
        let operacao:u8 = multiplicador * contador;
        println!("{} x {} = {}", multiplicador, contador, operacao);

        contador += 1;
    }

    contador = 0;
    loop{
        contador += 1;
        println!("Contador = {}", contador);
        if contador == 10{
        break;}
    }

    for i in 1..11{
        println!("{} x {} = {}", multiplicador, i, multiplicador*i);
    }
}

fn ownership(){
    let mut string1: String = String::from("Victor");
    string(&mut string1); 

    println!("String 1 = {}", string1); 


    // let string2: String = string(string1);

    // println!("String = {}", string2);

    // println!("String 1 = {}", string1); //vai dar erro, pois a string foi movida para a função string, e não pode mais ser usada aqui
}

fn string(string: &mut String) -> String{
    string.push_str(" Dias");
    println!("String = {}", string);
    string.clone()

    //clone é usado para criar uma cópia da string, e não mover a string para a função, assim a string original pode ser usada depois da função
}

fn pattern_matching(){
    for x in 1..21{
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 | 4 => "Pouco mais",
            5..=10 => "Mais",
            _ if x % 2 == 0 => "Muito mais",
            _ => "Muito"
        })
    }
}

fn erros(){

    //panic!("Erro teste"); //panic é usado para gerar um erro e encerrar o programa

    match resultado(){
        Ok(s) => println!("Resultado: {}", s),
        Err(numero) => panic!("Erro: {}", numero)
    };
}

fn resultado() -> Result<String, u8>{
   // Ok(String::from("Sucesso"))

   Err(42)
}

fn main(){
    escopo();
    sombra();
    soma(10, 20);
    condicionais();
    loops();
    ownership();
    pattern_matching();
    erros();
}

//let condicao:&str;
//if idade >= 18{
//    condicao = "maior"
//}else{
//    condicao = "menor"};

//i = inteiro de 1 byte
//u = numeros positivos de 1 byte
//f = float de 4 bytes
//variaveis são imutáveis por padrão, para torná-las mutáveis é necessário colocar a palavra reservada mut antes do nome da variável
//char = caractere de 4 bytes
//const é usada para guardar valores que não podem ser alterados, e deve ser declarada com o tipo de dado
    //}
    //println!("É {} de idade", condicao);

//i = inteiro de 1 byte
//u = numeros positivos de 1 byte
//f = float de 4 bytes
//variaveis são imutáveis por padrão, para torná-las mutáveis é necessário colocar a palavra reservada mut antes do nome da variável
//char = caractere de 4 bytes
//const é usada para guardar valores que não podem ser alterados, e deve ser declarada com o tipo de dado
//usada para quando vai repetir o mesmo valor em vários lugares do código, e não quer que ele seja alterado
//unsafe é usada para acessar variáveis globais, e deve ser usada com cuidado, pois pode causar problemas de segurança
//static é usada para declarar variáveis globais
//escopo é o local onde a variável foi declarada, e só pode ser acessada dentro desse escopo
//é possivel redeclarar uma variável com o mesmo nome, mas ela será uma nova variável, e não a mesma que foi declarada anteriormente
//; é usado para indicar o final de uma instrução, e é necessário colocar o ; no final de cada instrução, exceto quando a instrução é a última linha de uma função, nesse caso o ; é opcional

//loop é usado para criar um loop infinito, e deve ser usado com cuidado, pois pode causar problemas de desempenho
//break é usado para sair de um loop
//continue é usado para pular a iteração atual de um loop e passar para a próxima

//for é usado para criar um loop que itera sobre uma coleção de elementos, e é mais eficiente que o while, pois não precisa verificar a condição a cada iteração

//match é usado para comparar um valor com vários padrões, e é mais eficiente que o if, pois não precisa verificar a condição a cada iteração

// ownership é o conceito de propriedade de variáveis em Rust, e é usado para garantir a segurança da memória, evitando problemas de acesso a memória inválida

//heap é a memória alocada dinamicamente, e é usada para armazenar dados que podem crescer ou diminuir de tamanho durante a execução do programa, como strings e vetores

//stack é a memória alocada estaticamente, e é usada para armazenar dados que têm tamanho fixo durante a execução do programa, como inteiros e floats

//result é um tipo de dado que representa o resultado de uma operação, e pode ser Ok ou Err, dependendo se a operação foi bem sucedida ou não





