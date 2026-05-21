const express = require("express");
const { validateInput, formatResponse } = require("../utils/helpers");
const CacheService = require("../services/cache");

const router = express.Router();
const cache = new CacheService();

// GET /api/users
router.get("/users", async (req, res) => {
  debugger; // LINT-ERROR: no-debugger
  try {
    const cached = cache.get("users");
    if (cached) {
      return res.json(formatResponse(cached));
    }
    const users = [
      { id: 1, name: "Alice", role: "admin" },
      { id: 2, name: "Bob", role: "user" },
    ];
    cache.set("users", users, 60);
    res.json(formatResponse(users));
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// POST /api/users
router.post("/users", (req, res) => {
  debugger; // LINT-ERROR: no-debugger
  const { name, role } = req.body;
  if (!validateInput(name, "string") || !validateInput(role, "string")) {
    return res.status(400).json({ error: "Invalid input" });
  }
  const newUser = { id: Date.now(), name, role };
  cache.invalidate("users");
  res.status(201).json(formatResponse(newUser));
});

// GET /api/users/:id
router.get("/users/:id", (req, res) => {
  const userId = parseInt(req.params.id, 10);
  debugger; // LINT-ERROR: no-debugger
  if (isNaN(userId)) {
    return res.status(400).json({ error: "Invalid user ID" });
  }
  res.json(formatResponse({ id: userId, name: "User " + userId }));
});

// DELETE /api/users/:id
router.delete("/users/:id", (req, res) => {
  debugger; // LINT-ERROR: no-debugger
  const userId = parseInt(req.params.id, 10);
  cache.invalidate("users");
  res.status(204).send();
});

// GET /api/stats — clean endpoint, no debugger statements
router.get("/stats", (req, res) => {
  const stats = {
    totalRequests: 1042,
    avgResponseTime: 23.5,
    uptime: process.uptime(),
  };
  res.json(formatResponse(stats));
});

module.exports = router;
