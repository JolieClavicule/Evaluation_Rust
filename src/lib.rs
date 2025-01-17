//! # Cache LRU
//!
//! La librairie cache LRU (Least Recently Used).
//!
//! ## Fonctionnalités
//!
//! - Creation d'un cache LRU d'une taille choisie
//! - Insertion d'un élément clé/valeur dans le cache
//! - Récupération d'une valeur par sa clé
//! - Vérifier si un élément est contenu dans le cache
//! - Retour de la taille du contenu du cache
//! - Vérifier si le cache est vide
//!
//! ## Exemple d'utilisation
//! ```
//! use evaluation::{LRUCache, Cache};
//!
//! let mut cache = LRUCache::new(3);
//!
//! // Ajout d'éléments dans le cache
//! cache.insert("A", 1);
//! cache.insert("B", 2);
//! cache.insert("C", 3);
//!
//! // On récupère les éléments ajoutés précédemment
//! cache.get(&"A"); // 1
//! cache.get(&"B"); // 2
//! cache.get(&"C"); // 3
//!
//! // On vérifie la taille du contenu du cache
//! cache.size(); // 3
//!
//! // On vérifie si le cache est vide
//! cache.is_empty(); // faux
//!
//! // Suppression d'un élément
//! cache.remove(&"B");
//!
//! // Récupération de l'élément précédemment supprimé
//! cache.get(&"B"); // Rien
//!
//! // On vérifie la taille du contenu du cache
//! cache.size(); // 2
//! ```
pub mod modules;
pub use modules::cache::LRUCache;
pub use modules::cache_trait::Cache;