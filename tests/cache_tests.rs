use evaluation::{Cache, LRUCache};

    #[test]
    fn test_insert_and_get() {
        let mut cache = LRUCache::new(3);

        // Ajout d'éléments
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // On vérifie qu'on récupère bien les éléments ajoutés
        assert_eq!(cache.get(&"A"), Some(&String::from("a")));
        assert_eq!(cache.get(&"B"), Some(&String::from("b")));
    }

    #[test]
    fn test_too_much_elements() {
        let mut cache = LRUCache::new(2);

        // Ajout d'éléments pour remplir le cache
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // Ajout d'un élément en trop par rapport à la taille du cache
        cache.insert("C", String::from("c"));

        // On vérifie que l'élément A a bien été retiré comme c'est le plus vieux
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("b")));
        assert_eq!(cache.get(&"C"), Some(&String::from("c")));
    }

    #[test]
    fn test_update_element_position() {
        let mut cache = LRUCache::new(2);

        // Ajout d'éléments pour remplir le cache
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // On réajoute A qui existe déjà, alors on le met à jour en tant que dernier élément ajouté
        assert_eq!(cache.get(&"A"), Some(&String::from("a")));

        // Ajout d'un nouvel élément, ce qui retire le plus ancien élément du cache
        cache.insert("C", String::from("c"));

        // On vérifie que le cache contient A et C, mais pas B
        assert_eq!(cache.get(&"B"), None);
        assert_eq!(cache.get(&"C"), Some(&String::from("c")));
        assert_eq!(cache.get(&"A"), Some(&String::from("a")));
    }

    #[test]
    fn test_size_limit() {
        let mut cache = LRUCache::new(2);

        // Ajout d'éléments pour remplir le cache +1
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));
        cache.insert("C", String::from("c"));

        // On vérifie que le cache ne contient que les deux derniers éléments ajoutés
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("b")));
        assert_eq!(cache.get(&"C"), Some(&String::from("c")));
    }

    #[test]
    fn test_size_and_empty() {
        let mut cache = LRUCache::new(3);

        // On vérifie que le size est bien vide et que sa taille est de 0 (= vide)
        assert!(cache.is_empty());
        assert_eq!(cache.size(), 0);

        // Ajout d'éléments
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // On vérifie l'état du cache après ajout
        assert!(!cache.is_empty());
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_update() {
        let mut cache = LRUCache::new(1);

        // Ajout d'un élément
        cache.insert("A", String::from("a"));

        // Modification de l'élément précédemment ajouté
        cache.insert("A", String::from("a2"));

        // Vérification de la mise à jour
        assert_eq!(cache.get(&"A"), Some(&String::from("a2")));
    }

    #[test]
    fn test_remove() {
        let mut cache = LRUCache::new(2);

        // Ajout d'éléments
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // Suppression d'un élément
        cache.remove(&"A");

        // On vérifie que l'élément A a bien été retiré
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("b")));
    }

    #[test]
    fn test_contains() {
        let mut cache = LRUCache::new(2);

        // Ajout d'éléments
        cache.insert("A", String::from("a"));
        cache.insert("B", String::from("b"));

        // On vérifie que les éléments ajoutés sont bien présents
        assert!(cache.contains(&"A"));
        assert!(cache.contains(&"B"));
    }
