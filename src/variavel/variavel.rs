const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn variavel() {
    /*

        // Variavel mutavel
        // let mut total = 30;

        // Variavel imutavel
        // let total = 30;

        // {
        //     // novo inicio
        //     let total = total + 20;
        //     println!("Trabalhou {} horas", total);
        // } // novo fim
    */
    let total = 30;
    let total_in_seconds = total * SECONDS_IN_HOUR;

    println!(
        "Um minuto tem {} segundos\nUma hora tem {} minutos\nUma hora tambem tem {} segundos",
        SECONDS_IN_MINUTE, MINUTES_IN_HOUR, SECONDS_IN_HOUR
    );

    println!("E o caboclo trabalhou {} segundos", total_in_seconds);
}
