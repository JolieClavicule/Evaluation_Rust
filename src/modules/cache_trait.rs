pub trait Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    /// Insère un élément dans le cache
    fn insert(&mut self, key: K, value: V);

    /// Récupère un élément dans le cache
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Vérifie si un élément est contenu dans le cache
    fn contains(&self, key: &K) -> bool;

    /// Retourne la taille du cache
    fn size(&self) -> usize;

    /// Vérifie si le cache est vide
    fn is_empty(&self) -> bool;

    // Supprime un élément du cache
    fn remove(&mut self, key: &K);
}