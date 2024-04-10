use std::mem;

fn affiche_grille(tableau: &Vec<Vec<char>>, longueur: i32, largeur: i32) {
    println!("┌───┬───┬───┬───┬───┬───┬───┐");
    for i in 0..longueur as usize {
        for j in 0..largeur as usize {
            print!("| {} ", tableau[i][j]);
        }
        println!("|");
        println!("├───┼───┼───┼───┼───┼───┼───┤");
    }
    println!("└───┴───┴───┴───┴───┴───┴───┘");
}

enum Direction {
    Gauche,
    Droite,
    Haut,
    Bas
}

#[derive(Clone)]
struct NoeudSerpent {
    ligne: i32,
    colonne: i32,
    suivant: Option<Box<NoeudSerpent>>,
}

fn maj_coords(tableau: &mut Vec<Vec<char>>, ligne: usize, colonne: usize, est_tete: bool) {
    if est_tete {
        tableau[ligne][colonne] = '■';
    } else {
        tableau[ligne][colonne] = ' ';
    }
}

fn avancer(tableau: &mut Vec<Vec<char>>, tete: &mut Box<NoeudSerpent>, queue: &mut Box<NoeudSerpent>, direction: Direction, doit_supprimer_queue: bool) {
    let (dx, dy): (i32, i32) = match direction {
        Direction::Gauche => (-1, 0),
        Direction::Droite => (1, 0),
        Direction::Haut => (0, -1),
        Direction::Bas => (0, 1),
    };

    if doit_supprimer_queue {
        let mut nouvelle_tete = mem::replace(&mut tete.suivant, None);

        nouvelle_tete.as_mut().unwrap().ligne = tete.ligne + dy;
        nouvelle_tete.as_mut().unwrap().colonne = tete.colonne + dx;

        tableau[tete.ligne as usize][tete.colonne as usize] = ' ';

        tete.suivant = nouvelle_tete;

        let mut nouvelle_queue = mem::replace(&mut queue.suivant, None);

        nouvelle_queue.as_mut().unwrap().ligne = queue.ligne;
        nouvelle_queue.as_mut().unwrap().colonne = queue.colonne;

        tableau[queue.ligne as usize][queue.colonne as usize] = '■';

        queue.suivant = nouvelle_queue;
        queue.suivant = None;
    } else {
        let nouvelle_queue = Box::new(NoeudSerpent {
            ligne: queue.ligne,
            colonne: queue.colonne,
            suivant: None
        });
        let mut nouveau_suivant = mem::replace(&mut queue.suivant, Some(nouvelle_queue));

        nouveau_suivant.as_mut().unwrap().ligne = nouveau_suivant.as_ref().unwrap().ligne + dy;
        nouveau_suivant.as_mut().unwrap().colonne = nouveau_suivant.as_ref().unwrap().colonne + dx;

        tableau[nouveau_suivant.as_ref().unwrap().ligne as usize][nouveau_suivant.as_ref().unwrap().colonne as usize] = '■';

        queue.suivant = nouveau_suivant;
    }
}

fn run(tableau: &mut Vec<Vec<char>>){

}

fn initialiser_serpent(longueur: i32, largeur: i32) -> (Box<NoeudSerpent>, Box<NoeudSerpent>) {
    // Crée le premier noeud du serpent au centre du tableau
    let mut tete = Box::new(NoeudSerpent {
        ligne: longueur as i32 / 2,
        colonne: largeur as i32 / 2,
        suivant: None,
    });

    // Crée le deuxième noeud du serpent juste en dessous du premier
    let queue = Box::new(NoeudSerpent {
        ligne: tete.ligne + 1,
        colonne: tete.colonne,
        suivant: None,
    });

    // Met à jour le pointeur suivant du premier noeud pour qu'il pointe vers le deuxième noeud
    tete.suivant = Some(queue.clone());

    // Retourne la queue et la tête du serpent sous forme d'un tuple
    (queue, tete)
}

fn initialiser_tableau(queue: &Box<NoeudSerpent>, longueur: i32, largeur: i32) -> Vec<Vec<char>> {
    let taille = 7;
    let mut nouveau_tableau = Vec::new();
    for _ in 0..longueur {
        let mut ligne = Vec::new();
        for _ in 0..largeur {
            ligne.push(' ');
        }
        nouveau_tableau.push(ligne);
    }


    // Parcourt le serpent à partir de la queue et met à jour le tableau
    let mut noeud_courant = queue.clone();
    while let Some(noeud_suivant) = &noeud_courant.suivant {
        nouveau_tableau[noeud_courant.ligne as usize][noeud_courant.colonne as usize] = '■';
        noeud_courant = noeud_suivant.clone();
    }
    nouveau_tableau[noeud_courant.ligne as usize][noeud_courant.colonne as usize] = '■';

    return nouveau_tableau;
}


fn main() {
    let longueur = 10;
    let largeur = 10;
    let (mut queue, mut tete) = initialiser_serpent(longueur, largeur);

    let mut mon_tableau = initialiser_tableau(&queue, longueur, largeur);
    affiche_grille(&mon_tableau, longueur, largeur);
    println!("Hello, world!");
}