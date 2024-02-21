# Inteiros
| bits | signed | unsigned |
| ---- | ------ | -------- |
| 8    | i8     | u8       |
| 16   | i16    | u16      |
| 32   | i32    | u32      |
| 64   | i64    | u64      |
| 128  | i128   | u128     |
| arch | isize  | usize    |

## signed 
- range: -(2ⁿ⁻¹) até 2ⁿ⁻¹ - 1
- i8: -128 até 127. `[-(2ⁿ⁻¹) até 2ⁿ⁻¹ - 1]`

> ## Comentários
> Note que `signed` pode receber números positivos e negativos, por exemplo:
> - 127 ou -127 

## unsigned
- range: 0 até 2ⁿ - 1
- u8: 0 até 255 `[0 até 2ⁿ - 1]`
> ## Comentários
> Já o `unsigned` pode receber apenas o valor literal sem sinal de subtração, por exemplo:
> - 247 

# Considerações:

#### Algo a se notar é que o valor padrão de qualquer número em uma variável let é um `signed` de `i32` independente da arquitetura do seu computador. Ou seja, meu computador tem uma arquitetura de 64 bits, mesmo assim o default (valor padrão) vai ser um `signed` de `i32`
#### Exemplo:
> ```rust
>    let inteiro = 127;
>    // ou
>    let inteiro: i32 = 127;
>    // o compilador irá considerar este formato abaixo:
>    let inteiro = 127_i32;