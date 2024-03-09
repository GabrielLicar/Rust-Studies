# Rust
- Arquivos rust tem a extensão: `.rs`
- Para compilar o arquivo rust utiliza:
    - `rustc archive.rs`

## Inicialização
### Comandos cargo:
- Criar um novo projeto
    - `cargo new nomedoprojeto`
- Criar um novo projeto binario
    - `cargo new --bin nomedoprojeto`
    
- Para compilar/buildar o projeto em modo de debug
    - `cargo build`    

- Para compilar/buildar o projeto em modo de produção
    - `cargo build --release`

- Executar o projeto e compilar ao mesmo tempo
    - `cargo run`
    
- Formatar um projeto
    - `cargo fmt`

### Cargo Watch
- Para instalar o cargo watch:
    - `cargo install cargo-watch`
- Para executar:
    - `cargo watch -x run`

## Variáveis

### Variável (default/padrão)
- As variáveis podem ser utilizadas como:
    - Variável com tipo `não explicitado`
        ```rust
            /**
             * Nesse caso o próprio compilador colocará o tipo aproximado da variável
            **/
            let nome = "Gabriel";
        ```
    - Variável com tipo `explicitado`
        ```rust
            /**
             * Os tipos podem ser definidos/explicitados de duas formas
             * Sendo elas (consecutivamente):
            */
            let numero: i32 = 77;
            let numero = 77_i32;
        ```
    > Note que essas váriaveis são imutáveis ou seja, ela não poderá ser "unassigned" ("re-atribuida")

    Para a variável ser mutável precisamos colocar o prefixo `mut`
    #### Exemplo:
    ```rust
        let mut numero: i8 = 32;
    ``` 

### Constante

- As constante devem está no topo do código sempre em um formato em caixa alta usando o snake case e declarando/explicitando o tipo.
    #### Exemplo:
    ```rust
        const MINUTES_IN_SECONDS: i32 = 60;

        fn main() {
            println!("Um minuto tem aproximadamente: {} segundos.", MINUTES_IN_SECONDS)
        }
    ```

## Tipos Primitivos

- Os tipos primitivos são dividos em duas categorias que são elas:
    - A categoria dos [Escalares](./annotations/escalares.md).
    - A categoria dos [Compostos](./annotations/compostos.md).

