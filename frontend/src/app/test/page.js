"use client";

import { useState } from "react";

export default function TestApiPage() {
  const [message, setMessage] = useState("");

  async function testCors() {
    try {
      const res = await fetch("http://localhost:8000/health", {
        method: "GET",
      });

      if (res.ok) {
        const text = await res.text();
        setMessage(`✅ Success: ${text}`);
      } else {
        setMessage(`❌ Error: ${res.status}`);
      }
    } catch (err) {
      console.error("Network error:", err);
      setMessage("❌ Network error (CORS?)");
    }
  }

  return (
    <main className="p-8 max-w-xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">Test Backend CORS</h1>
      <button
        onClick={testCors}
        className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700"
      >
        Test Fetch
      </button>
      {message && <p className="mt-4">{message}</p>}
    </main>
  );
}
