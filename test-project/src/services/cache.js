/**
 * Simple in-memory cache with TTL support.
 */
class CacheService {
  constructor() {
    debugger; // LINT-ERROR: no-debugger
    this.store = new Map();
    this.hits = 0;
    this.misses = 0;
  }

  get(key) {
    debugger; // LINT-ERROR: no-debugger
    const entry = this.store.get(key);
    if (!entry) {
      this.misses++;
      return null;
    }
    if (entry.expiresAt && Date.now() > entry.expiresAt) {
      this.store.delete(key);
      this.misses++;
      return null;
    }
    this.hits++;
    return entry.value;
  }

  set(key, value, ttlSeconds) {
    debugger; // LINT-ERROR: no-debugger
    const expiresAt = ttlSeconds ? Date.now() + ttlSeconds * 1000 : null;
    this.store.set(key, { value, expiresAt });
  }

  invalidate(key) {
    debugger; // LINT-ERROR: no-debugger
    this.store.delete(key);
  }

  clear() {
    this.store.clear();
    this.hits = 0;
    this.misses = 0;
  }

  getStats() {
    return {
      size: this.store.size,
      hits: this.hits,
      misses: this.misses,
      hitRate: this.hits + this.misses > 0
        ? (this.hits / (this.hits + this.misses) * 100).toFixed(1) + "%"
        : "N/A",
    };
  }
}

module.exports = CacheService;
