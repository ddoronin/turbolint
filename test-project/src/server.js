const express = require("express");
const apiRoutes = require("./routes/api");

const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

// Middleware: request logging
app.use((req, res, next) => {
  debugger; // LINT-ERROR: no-debugger
  console.log(`${req.method} ${req.url}`);
  next();
});

// Health check endpoint
app.get("/health", (req, res) => {
  res.json({ status: "ok", uptime: process.uptime() });
});

// Mount API routes
app.use("/api", apiRoutes);

// Error handling middleware
app.use((err, req, res, next) => {
  debugger; // LINT-ERROR: no-debugger
  console.error("Unhandled error:", err.message);
  res.status(500).json({ error: "Internal server error" });
});

// Graceful shutdown
function shutdown() {
  debugger; // LINT-ERROR: no-debugger
  console.log("Shutting down gracefully...");
  server.close(() => {
    console.log("Server closed");
    process.exit(0);
  });
}

process.on("SIGTERM", shutdown);
process.on("SIGINT", shutdown);

const server = app.listen(PORT, () => {
  debugger; // LINT-ERROR: no-debugger
  console.log(`Server running on port ${PORT}`);
});

module.exports = app;
