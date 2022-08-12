use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Gerando um número aleatório de 1 a 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");

        //Declarando uma variável mutável de tipo String, um tipo de string que pode ter seu tamanho alterado.
        //instanciamos essa nova String com o código do lado direito da igualdade, com a função new().
        let mut guess = String::new();

        //Agora chamamos a função stdin() do módulo io, que nos permitirá manipular o input do usuário.
        /*O trabalho de .read_line(&mut guess) é pegar tudo que o usuário digitar e adicionar à string guess que passamos como argumento.
        .read_line() também retorna um Result value. Result retorna ok ou err, lidamos com essas duas variantes com o .expect().*/
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            //Aqui vemos um shadowing, sobrescrevemos a variável guess declarada na linha 17.
            //Vinculamos essa variável a expressão guess.trim().parse(). 
            //O método trim em uma instância de String irá eliminar todos os espaços em branco
            //Usamos o método parse para converter guess de String para u32
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        //Comparamos guess e secret_numer usando o método cmp.
        //Ordering é um enumerator, como o result, suas 3 variantes são: Less, Greater e Equal.
        //Essa expressão match irá validar qual é a resposta do cmp e então executar o código definido por nós.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
