# ECD Register Counter

Um contador de registros para arquivos ECD (Escrituração Contábil Digital) do SPED Fiscal da Receita Federal do Brasil, desenvolvido em Rust.

## 📋 Sobre o Projeto

Este programa processa arquivos ECD do SPED Fiscal e conta a quantidade de registros por tipo. O ECD é um arquivo digital que integra o Sistema Público de Escrituração Digital (SPED) da Receita Federal do Brasil.

**Documentação oficial do layout ECD:** [http://sped.rfb.gov.br/pasta/show/1569](http://sped.rfb.gov.br/pasta/show/1569)

## 🚀 Funcionalidades

- ✅ Conta registros por tipo de registro ECD
- ✅ Exibe resultados na ordem que aparecem no arquivo
- ✅ Barra de progresso visual com percentual
- ✅ Formatação de números com separador de milhares
- ✅ Tratamento de erros robusto
- ✅ Performance otimizada para arquivos grandes

## 🔧 Compilação

### Pré-requisitos

- Rust 1.56.0 ou superior
- Cargo (incluído com Rust)

### Compilar em modo debug

```bash
cargo build
```

### Compilar em modo release (otimizado)

```bash
cargo build --release
```

O executável será gerado em:
- **Debug:** `target/debug/ecd_reg_count`
- **Release:** `target/release/ecd_reg_count`

## 🎯 Como Usar

### Executar com Cargo

```bash
# Modo debug
cargo run -- arquivo_ecd.txt

# Modo release (recomendado para arquivos grandes)
cargo run --release -- arquivo_ecd.txt
```

### Executar diretamente o binário gerado com o Cargo

```bash
# Debug
./target/debug/ecd_reg_count arquivo_ecd.txt

# Release
./target/release/ecd_reg_count arquivo_ecd.txt
```

## 📄 Formato do Arquivo de Entrada

O programa processa arquivos ECD no formato padrão do SPED Fiscal, onde:

- Cada linha representa um registro
- O tipo de registro é identificado pelos caracteres 2-5 (após o primeiro `|`)
- Formato: `|TIPO|dados...|`

### Exemplo de arquivo de entrada:

```
|0000|XXXXXXXXXXXXXXXXXXXXXX|JAHFKJDH|EWEIROWIEURO|
|0001|YYYYYYYYYYYY YYYYYYYYY\|KJDHFKJSHDKFJS|
|I300|YYYYYYYYYYYY YYYYYYYYY\|KJDHFKJSHDKFJS|SDJKHFDJSKHFKDJS|
|I350|YYYYYYYYYYYY|
|I355|AAAAAAAA|
|I355|BBBBBBBB|
|I355|CCCCCCCC|
|I900|0000|1|
|I900|0001|1|
|I900|I300|1|
|I900|I350|1|
|I900|I355|3|
|I900|I900|7|
|I900|9999|1|
|9999|X|
```

## 📊 Exemplo de Saída

```
Processando arquivo: arquivo_ecd.txt
Total de linhas: 15

[==================================================] 100.0% (15/15 linhas processadas)

Processamento concluído!

Quantidade de registros por tipo:
0000: 1
0001: 1
I300: 1
I350: 1
I355: 3
I900: 7
9999: 1
```

## 🎨 Recursos da Interface

### Barra de Progresso
- Mostra progresso visual `[=====>     ]`
- Percentual com uma casa decimal
- Contador de linhas processadas com separador de milhares
- Atualização otimizada (a cada 100 linhas)

### Formatação de Números
- Separador de milhares com ponto (`.`)
- Exemplo: `1.234.567` linhas

## 📋 Tipos de Registro ECD Comuns

O programa identifica automaticamente todos os tipos de registro presentes no arquivo. Alguns tipos comuns incluem:

- **0000:** Abertura do arquivo digital
- **0001:** Dados da empresa
- **I300:** Cadastro de bens do ativo imobilizado
- **I350:** Baixas do ativo imobilizado
- **I355:** Controle individual de bem do ativo imobilizado
- **I900:** Controle de registros do arquivo
- **9999:** Encerramento do arquivo digital

## ⚡ Performance

Para arquivos grandes, recomenda-se usar o modo release:

```bash
cargo run --release -- arquivo_grande.txt
```

O modo release pode ser 5x a 10x mais rápido que o modo debug para processamento intensivo.

## 🛠️ Desenvolvimento

### Estrutura do projeto

```
ecd_reg_count/
├── Cargo.toml                # Configuração do projeto
├── src/
│   └── main.rs               # Código principal
└── README.md                 # Este arquivo
```

### Tecnologias utilizadas

- **Rust 2021 Edition**
- **std::collections::HashMap** para contagem eficiente
- **std::io** para operações de arquivo otimizadas
- **BufReader** para leitura eficiente de arquivos grandes

## 📝 Licença

Este projeto é de código aberto e pode ser usado livremente.
