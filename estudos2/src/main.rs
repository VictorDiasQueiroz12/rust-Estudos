
fn matriz(){
    let _matriz:[[f32; 3]; 2] = [
        [1.0f32, 2.2f32, 3.3f32], 
        [4.4f32, 5.5f32, 6.6f32]
    ];

    for linha in _matriz{
        for item in linha {
           println!("Item = {}", item);
        }
    }
} 

fn notas(){
     let _notas: [f32; 4] = [6.5f32; 4];
     let inteiro: usize = 0;

     println!("{}", _notas[inteiro]);
    /* let _notas: [f32; 4] = [10f32, 8f32, 7.8f32, 9f32]; */

    for indice in 0.._notas.len(){
        println!("A nota {} é = {}", indice + 1, _notas[indice]);
    }

    /*  println!("Notas: {}, {}, {}, {}", _notas[0], _notas[1], _notas[2], _notas[3]); */

    /* let _nota1:f32 = 10f32;
    let _nota2:f32 = 8f32;
    let _nota3:f32 = 7.8f32;
    let _nota4:f32 = 9f32;

    println!("Notas: {}, {}, {}, {}", _nota1, _nota2, _nota3, _nota4); */
}

#[allow(dead_code)]
enum DiaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo
}

fn fim_de_semana(dia_semana: DiaSemana) -> bool {
    match dia_semana{
        DiaSemana::Sabado | DiaSemana::Domingo => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{ciano: u8, magenta: u8, amarelo: u8, preto: u8}
}

fn cores(){
    let cor:Color = Color:: CymkColor{ciano: 100, magenta: 50, amarelo: 70, preto: 255};

    println!("Cor = {}", match cor{
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) | Color::CymkColor { ciano:_, magenta:_, amarelo:_, preto: 255 }=> "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor { ciano: _, magenta:_, amarelo:_, preto:_ } => "CYMK desconhecido"
    });
}

fn opicional(){
    let conteudo_arquivo: Option<String> = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo{
        println!("Certeza do valor: {}", valor);
    }
}

fn ler_arquivo(_caminho_arquivo: String) -> Option<String>{
    Some(String::from("Conteudo do arquivo"))
}

fn vetores(){
    let mut _notas1: Vec<f32> = Vec::with_capacity(4);
    //quando sei mais ou menos quanto vai ter é top usar isso para alocar a memória com maior precisão 
    _notas1.push(10.0);
    _notas1.push(8.0);
    _notas1.push(6.5);

    println!("Capacidade {}", _notas1.capacity());

    //let mut _notas1: Vec<f32> = vec![10.0, 8.0, 6.5];

    // essa bomba dobra o tamanho quando da um push, ent tem capazidade pra 6 coisa no vec, como tem 4 notas fica 2 espaço sobrando sendo disperdiçado 

    println!("{:?}", _notas1);

    _notas1.push(9.7);

    println!("Capacidade {}", _notas1.capacity());

    println!("Nota 4 = {}", _notas1[3]);
    println!("Nota 5 = {}", match _notas1.get(5) {
        Some(n) => *n,
        None => 0.0
    });

    println!("{:?}", _notas1);

    for _nota in &_notas1{
        println!("Nota = {}", _nota)
    }

   /*  while let Some(nota1) = _notas1.pop(){
        println!("Valor removido = {}", nota1);
    }; */

    println!("{:?}", _notas1);

}


struct NomeSobrenome{
    nome: String,
    sobrenome: String
}
struct Conta {
    titular: NomeSobrenome,
    saldo: f32
}

impl Conta {
    fn sacar(&mut self, valor: f32) {
        self.saldo -= valor; 
    }   
}

fn conta_corrente(){
    let titular: NomeSobrenome = NomeSobrenome{ nome: String::from ("Victor"), sobrenome: String::from ("Dias") };

    let mut conta: Conta = Conta{titular: titular, saldo: 100.0};
    
    conta.sacar(50.0);

    // let titular_conta: String = String::from("Victor Dias");
    // let saldo_conta: f32 = 100.0;

    println!("Dados da conta: Titular = {} {}, Saldo = {}",conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}

fn main() {
    notas();
    matriz();
    println!("É fim de semana? {}", fim_de_semana(DiaSemana::Segunda));
    cores();
    opicional();
    vetores();
    conta_corrente();
}





//usize é um tipo de dado inteiro sem sinal que é usado para indexação e contagem de elementos em coleções. Ele é garantido para ser grande o suficiente para conter o tamanho máximo de qualquer coleção em Rust, como arrays, vetores e fatias. O tamanho de usize depende da arquitetura do sistema: em sistemas de 32 bits, usize é de 32 bits, enquanto em sistemas de 64 bits, usize é de 64 bits.

//enum é um tipo de dado que permite definir um conjunto de valores nomeados, chamados variantes. Cada variante pode ter dados associados a ela, permitindo representar diferentes estados ou opções em um programa. Em Rust, enums são usados para criar tipos de dados personalizados que podem ter diferentes formas e comportamentos, tornando o código mais expressivo e seguro.








