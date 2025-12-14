struct Livre {
    titre: String,
    auteur: String,
    annee_de_publication: u32,
    disponible: bool,
}

fn afficher_livres(bibliotheque: &Vec<Livre>) {
    for livre in bibliotheque {
        println!("Titre : {} - Auteur : {} - Année de publication : {}", livre.titre, livre.auteur, livre.annee_de_publication);
    }
}

fn afficher_livres_disponibles(bibliotheque: &Vec<Livre>) {
    for livre in bibliotheque {
        if livre.disponible {
            println!("Titre : {} - Auteur : {} - Année de publication : {}", livre.titre, livre.auteur, livre.annee_de_publication);
        }
    }
}

fn emprunter_livre(bibliotheque: &mut Vec<Livre>){
    let mut trouver = false;
    let mut titre = String::new();
    println!("Entrez le titre du livre à emprunter :");
    std::io::stdin().read_line(&mut titre).unwrap();

    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre.trim() {
            if livre.disponible {
                livre.disponible = false;
                trouver = true;
                println!("Le livre : {} a été emprunté avec succès.", livre.titre);
                return;
            } else {
                println!("Désolé, ce livre n'est pas disponible.");
                return;
            }
        }
    }
    if trouver == false {
        println!("Ce livre n'existe pas.");
    }
}

fn retourner_livre(bibliotheque: &mut Vec<Livre>){
    let mut trouver = false;
    let mut titre = String::new();
    println!("Entrez le titre du livre à retourner :");
    std::io::stdin().read_line(&mut titre).unwrap();

    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre.trim() {
            if livre.disponible == false {
                livre.disponible = true;
                trouver = true;
                println!("Le livre : {} a été retourné avec succès.", livre.titre);
                return;
            } else {
                println!("Ce livre n'a pas été emprunté.");
                return;
            }
        }

    }
    if trouver == false {
        println!("Ce livre n'existe pas.")
    }
}

fn ajouter_livre(bibliotheque: &mut Vec<Livre>) {
    let mut titre = String::new();
    let mut auteur = String::new();
    let mut annee_pub = String::new();

    println!("Entrez le titre du livre :");
    std::io::stdin().read_line(&mut titre).unwrap();

    if bibliotheque.iter().any(|livre| livre.titre == titre.trim()) {
        println!("Un livre avec ce titre existe déjà");
        return;
    }
    println!("Entrez l'auteur du livre :");
    std::io::stdin().read_line(&mut auteur).unwrap();   
    println!("Entrez l'année de publication du livre :");
    std::io::stdin().read_line(&mut annee_pub).unwrap();

    let annee_de_publication: u32 = annee_pub.trim().parse().unwrap();

    let new_livre = Livre {
        titre: titre.trim().to_string(),
        auteur: auteur.trim().to_string(),
        annee_de_publication,
        disponible: true,
    };

    bibliotheque.push(new_livre);
    println!("Livre ajouté avec succès !");
}



fn main() {

    let mut bibliotheque = Vec::<Livre>::new();

    bibliotheque.push(Livre {
    titre: "Les Misérables".to_string(),
    auteur: "Victor Hugo".to_string(),
    annee_de_publication: 1862,
    disponible: true,
    });

    bibliotheque.push(Livre {
        titre: "Le Petit Prince".to_string(),
        auteur: "Antoine de Saint-Exupéry".to_string(),
        annee_de_publication: 1943,
        disponible: true,
    });

    bibliotheque.push(Livre {
        titre: "1984".to_string(),
        auteur: "George Orwell".to_string(),
        annee_de_publication: 1949,
        disponible: false,
    });


    loop {
        println!("Bienvenue dans le menu, selectionnez une option :");
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles seuelement");
        println!("6. Quitter");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).unwrap();

        let choix_num: u32 = choix.trim().parse().unwrap();

        match choix_num {
            1 => {
                println!("Choix 1 sélectionné");
                ajouter_livre(&mut bibliotheque);
            },
            2 => {
                println!("Choix 2 sélectionné");
                emprunter_livre(&mut bibliotheque);
            },
            3 => {
                println!("Choix 3 sélectionné");
                retourner_livre(&mut bibliotheque);
            },
            4 => {
                println!("Choix 4 sélectionné");
                afficher_livres(&bibliotheque);
            },
            5 => {
                println!("Choix 5 sélectionné");
                afficher_livres_disponibles(&bibliotheque);
            },
            6 => {
                println!("Fin du programme");
                break;
            },
            _ => {
                println!("Entrée invalide");
            },
        }
    }
}
