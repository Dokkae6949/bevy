#![allow(missing_docs)]

use std::any::Any;
use std::collections::HashSet;

use bevy_platform::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value<T> {
    Val(T),
    Ref(Token<T>),
}

#[derive(Debug, Clone, Copy)]
pub struct Token<T> {
    pub name: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Token<T> {
    pub const fn new(name: &'static str) -> Self {
        Token {
            name,
            _marker: std::marker::PhantomData,
        }
    }
}

struct CacheEntry {
    version: u64,
    value: Box<dyn Any + Send + Sync>,
}

pub struct StyleSheet {
    tokens: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    cache: HashMap<&'static str, CacheEntry>,
    version: u64,
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleSheet {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::default(),
            cache: HashMap::default(),
            version: 0,
        }
    }

    /// Insert/override a token with a value or a reference.
    /// Bumps the global version to invalidate all cached resolutions lazily.
    pub fn set<T: 'static + Send + Sync>(&mut self, token: Token<T>, value: Value<T>) {
        self.tokens.insert(token.name, Box::new(value));
        self.version = self.version.wrapping_add(1);
        // No need to eagerly clear cache; we do lazy invalidation via version checks.
    }

    /// Get the raw stored entry (Value::Val or Value::Ref) without resolving.
    pub fn raw<T: 'static + Clone + Send + Sync>(&self, token: Token<T>) -> Option<Value<T>> {
        self.tokens
            .get(token.name)
            .and_then(|boxed| boxed.downcast_ref::<Value<T>>())
            .cloned()
    }

    /// Get the resolved value (follows refs), with lazy caching and cycle detection.
    ///
    /// Returns None if:
    /// - token not found,
    /// - type mismatch (stored entry has different T),
    /// - a reference cycle is detected.
    pub fn get<T: 'static + Clone + Send + Sync>(&mut self, token: Token<T>) -> Option<T> {
        // If cached and up-to-date with the current global version, return it.
        if let Some(entry) = self.cache.get(token.name) {
            if entry.version == self.version {
                if let Some(v) = entry.value.downcast_ref::<T>() {
                    return Some(v.clone());
                }
            }
        }

        // Resolve with cycle detection.
        let mut visiting = HashSet::new();
        let resolved = self.resolve_inner(&token, &mut visiting)?;

        // Store in cache tagged with the current global version.
        self.cache.insert(
            token.name,
            CacheEntry {
                version: self.version,
                value: Box::new(resolved.clone()),
            },
        );

        Some(resolved)
    }

    fn resolve_inner<T: 'static + Clone + Send + Sync>(
        &self,
        token: &Token<T>,
        visiting: &mut HashSet<&'static str>,
    ) -> Option<T> {
        // Cycle check: if we're already visiting this token, it's a cycle.
        if !visiting.insert(token.name) {
            // cycle detected
            return None;
        }

        // Look up stored entry and resolve.
        let out = match self.tokens.get(token.name)?.downcast_ref::<Value<T>>()? {
            Value::Val(v) => v.clone(),
            Value::Ref(other) => {
                // Recurse to resolve the referenced token.
                self.resolve_inner(other, visiting)?
            }
        };

        // Done with this node.
        visiting.remove(token.name);
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_val() {
        let mut sheet = StyleSheet::new();
        const COLOR: Token<u32> = Token::new("color");

        sheet.set(COLOR, Value::Val(0xFF00FF));
        assert_eq!(sheet.get(COLOR).unwrap(), 0xFF00FF);
        assert!(matches!(sheet.raw(COLOR), Some(Value::Val(0xFF00FF))));
    }

    #[test]
    fn set_and_get_ref() {
        let mut sheet = StyleSheet::new();
        const BASE: Token<u32> = Token::new("base");
        const ALIAS: Token<u32> = Token::new("alias");

        sheet.set(BASE, Value::Val(42));
        sheet.set(ALIAS, Value::Ref(BASE));

        assert_eq!(sheet.get(ALIAS).unwrap(), 42);
        assert!(matches!(sheet.raw(ALIAS), Some(Value::Ref(_))));
    }

    #[test]
    fn versioned_cache_recomputes_after_set() {
        let mut sheet = StyleSheet::new();
        const BASE: Token<u32> = Token::new("base");
        const ALIAS: Token<u32> = Token::new("alias");

        sheet.set(BASE, Value::Val(1));
        sheet.set(ALIAS, Value::Ref(BASE));

        // initial compute -> cache alias=1@v1
        assert_eq!(sheet.get(ALIAS).unwrap(), 1);

        // update base (version bumps), alias cache should be considered stale and recompute
        sheet.set(BASE, Value::Val(2));
        assert_eq!(sheet.get(ALIAS).unwrap(), 2);
    }

    #[test]
    fn cycle_detection_returns_none() {
        let mut sheet = StyleSheet::new();
        const A: Token<u32> = Token::new("a");
        const B: Token<u32> = Token::new("b");

        sheet.set(A, Value::Ref(B));
        sheet.set(B, Value::Ref(A));

        assert!(sheet.get(A).is_none(), "cycle should yield None");
        assert!(sheet.get(B).is_none(), "cycle should yield None");
    }

    #[test]
    fn type_mismatch_yields_none() {
        let mut sheet = StyleSheet::new();
        const N: Token<u32> = Token::new("n");
        const S: Token<&'static str> = Token::new("n"); // same name but different type

        sheet.set(N, Value::Val(5));
        // Trying to get it as &str should fail
        assert!(sheet.get(S).is_none());
        // Raw also fails due to type mismatch
        assert!(sheet.raw(S).is_none());
    }
}
