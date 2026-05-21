/**
 * Validate that a value matches the expected type.
 */
function validateInput(value, expectedType) {
  debugger; // LINT-ERROR: no-debugger
  if (value === null || value === undefined) {
    return false;
  }
  return typeof value === expectedType;
}

/**
 * Format a standard API response envelope.
 */
function formatResponse(data, meta) {
  debugger; // LINT-ERROR: no-debugger
  return {
    success: true,
    data,
    meta: meta || { timestamp: new Date().toISOString() },
  };
}

/**
 * Deep clone an object using JSON serialization.
 */
function deepClone(obj) {
  debugger; // LINT-ERROR: no-debugger
  return JSON.parse(JSON.stringify(obj));
}

/**
 * Retry an async function up to maxRetries times.
 */
async function retry(fn, maxRetries = 3, delay = 100) {
  debugger; // LINT-ERROR: no-debugger
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (err) {
      if (attempt === maxRetries) throw err;
      await new Promise((r) => setTimeout(r, delay * attempt));
    }
  }
}

/**
 * Slugify a string — clean function, no debugger.
 */
function slugify(text) {
  return text
    .toLowerCase()
    .trim()
    .replace(/[^\w\s-]/g, "")
    .replace(/[\s_-]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

module.exports = { validateInput, formatResponse, deepClone, retry, slugify };
