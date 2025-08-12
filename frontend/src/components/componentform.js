"use client";

import { useState } from "react";

export default function CommentForm({ noteId }) {
  const [authorId, setAuthorId] = useState("");
  const [content, setContent] = useState("");
  const [status, setStatus] = useState("");

  async function handleSubmit(e) {
    e.preventDefault();
    setStatus("");

    try {
      const res = await fetch("http://localhost:8000/comments", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ note_id: noteId, author_id: authorId, content }),
      });

      if (res.ok) {
        setContent("");
        setAuthorId("");
        setStatus("✅ Comment added!");
      } else {
        setStatus("❌ Failed to add comment.");
      }
    } catch (err) {
      setStatus("❌ Network error.");
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-2 mt-2">
      <input
        className="border p-1 rounded w-full"
        type="text"
        placeholder="Author ID (UUID)"
        value={authorId}
        onChange={(e) => setAuthorId(e.target.value)}
        required
      />
      <textarea
        className="border p-1 rounded w-full"
        placeholder="Add a comment..."
        value={content}
        onChange={(e) => setContent(e.target.value)}
        required
      />
      <button
        type="submit"
        className="bg-blue-600 text-white px-3 py-1 rounded hover:bg-blue-700"
      >
        Comment
      </button>
      {status && <p className="text-sm mt-1">{status}</p>}
    </form>
  );
}
