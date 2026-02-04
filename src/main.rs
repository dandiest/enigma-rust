use std::io;

fn main() {
    let codice_giusto: i32 = 42;
    let mut vite: i32 = 3;

    println!("--- BENVENUTO NEL CAVEAU ---");

    loop {
        println!("\nTentativi rimasti: {}", vite);
        println!("Indovina il codice segreto:");

        
        let mut risposta = String::new();

        io::stdin()
            .read_line(&mut risposta)
            .expect("Errore nella lettura");

       
        let numero_utente: i32 = match risposta.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ehi! Devi inserire un NUMERO.");
                continue; 
            }
        };

        
        if numero_utente == codice_giusto {
            println!("HAI VINTO! Il codice segreto era {}.", codice_giusto);
            break;
        }
        
        vite -= 1;
        
        if vite == 0 {
            println!("HAI PERSO! Il codice segreto era {}.", codice_giusto);
            break;
        }
    }
}