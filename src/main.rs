use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Uso: {} <arquivo>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    // Primeira passada: conta o total de linhas
    let total_linhas = {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Erro ao abrir o arquivo '{}': {}", filename, err);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        reader.lines().count()
    };
    
    // Segunda passada: processa as linhas com barra de progresso
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Erro ao abrir o arquivo '{}': {}", filename, err);
            process::exit(1);
        }
    };
    
    let reader = BufReader::new(file);
    let mut contadores: HashMap<String, usize> = HashMap::new();
    let mut ordem_tipos: Vec<String> = Vec::new();
    
    // Função para desenhar a barra de progresso
    fn desenhar_barra_progresso(atual: usize, total: usize) {
        let percentual = (atual as f64 / total as f64) * 100.0;
        let barra_tamanho = 50;
        let preenchido = ((atual as f64 / total as f64) * barra_tamanho as f64) as usize;
        
        // Função para formatar número com separador de milhares
        fn formatar_numero(n: usize) -> String {
            let s = n.to_string();
            let mut resultado = String::new();
            let chars: Vec<char> = s.chars().collect();
            
            for (i, c) in chars.iter().enumerate() {
                if i > 0 && (chars.len() - i) % 3 == 0 {
                    resultado.push('.');
                }
                resultado.push(*c);
            }
            resultado
        }
        
        print!("\r[");
        for i in 0..barra_tamanho {
            if i < preenchido {
                print!("=");
            } else if i == preenchido {
                print!(">");
            } else {
                print!(" ");
            }
        }
        print!("] {:.1}% ({}/{} linhas processadas)", percentual, formatar_numero(atual), formatar_numero(total));
        io::stdout().flush().unwrap();
    }
    
    println!("Processando arquivo: {}", filename);
    println!("Total de linhas: {}", {
        let s = total_linhas.to_string();
        let mut resultado = String::new();
        let chars: Vec<char> = s.chars().collect();
        
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                resultado.push('.');
            }
            resultado.push(*c);
        }
        resultado
    });
    println!();
    
    // Lê cada linha do arquivo
    for (num_linha, linha) in reader.lines().enumerate() {
        let linha_atual = num_linha + 1;
        
        // Atualiza a barra de progresso a cada 100 linhas ou na última linha
        if linha_atual % 100 == 0 || linha_atual == total_linhas {
            desenhar_barra_progresso(linha_atual, total_linhas);
        }
        
        let linha = match linha {
            Ok(linha) => linha,
            Err(err) => {
                eprintln!("\nErro ao ler a linha {}: {}", linha_atual, err);
                continue;
            }
        };
        
        // Verifica se a linha tem pelo menos 5 caracteres (|XXXX|...)
        if linha.len() >= 5 {
            // Extrai o tipo de registro (caracteres 2-5, posições 1-4)
            let tipo_registro = &linha[1..5];
            
            // Se é a primeira vez que vemos este tipo, adiciona na ordem
            if !contadores.contains_key(tipo_registro) {
                ordem_tipos.push(tipo_registro.to_string());
            }
            
            // Incrementa o contador para este tipo
            *contadores.entry(tipo_registro.to_string()).or_insert(0) += 1;
        }
    }
    
    // Completa a barra de progresso
    println!("\n");
    println!("Processamento concluído!");
    println!();
    
    // Exibe os resultados na ordem que apareceram no arquivo
    println!("Quantidade de registros por tipo:");
    for tipo in ordem_tipos {
        if let Some(quantidade) = contadores.get(&tipo) {
            println!("{}: {}", tipo, quantidade);
        }
    }
}