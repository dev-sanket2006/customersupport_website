"use client";

import { useEffect, useState } from "react";

export default function NotesPage() {
  const [notes, setNotes] = useState([]);
  const [filteredNotes, setFilteredNotes] = useState([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortOrder, setSortOrder] = useState("newest");
  const [error, setError] = useState("");
  const [comments, setComments] = useState({});
  const [expandedNotes, setExpandedNotes] = useState({});
  const [newComment, setNewComment] = useState({});
  const [editingComment, setEditingComment] = useState(null);
  const [editedContent, setEditedContent] = useState("");

  useEffect(() => {
    async function fetchNotes() {
      try {
        const token = localStorage.getItem("token");
        const res = await fetch("http://localhost:8000/notes", {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        if (!res.ok) throw new Error("Failed to fetch notes");
        const data = await res.json();
        setNotes(data);
        setFilteredNotes(sortNotes(data, "newest"));
      } catch (err) {
        console.error(err);
        setError("❌ Failed to fetch notes");
      }
    }

    fetchNotes();
  }, []);

  const sortNotes = (notesArray, order) => {
    return [...notesArray].sort((a, b) => {
      const aTime = new Date(a.created_at ?? a.id).getTime();
      const bTime = new Date(b.created_at ?? b.id).getTime();
      return order === "newest" ? bTime - aTime : aTime - bTime;
    });
  };

  useEffect(() => {
    const filtered = notes.filter((note) =>
      note.content.toLowerCase().includes(searchTerm.toLowerCase())
    );
    setFilteredNotes(sortNotes(filtered, sortOrder));
  }, [searchTerm, sortOrder, notes]);

  const toggleComments = async (noteId) => {
    if (expandedNotes[noteId]) {
      setExpandedNotes((prev) => ({ ...prev, [noteId]: false }));
      return;
    }

    try {
      const token = localStorage.getItem("token");
      const res = await fetch(
        `http://localhost:8000/comments/by-note/${noteId}`,
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
      if (!res.ok) throw new Error("Failed to fetch comments");
      const data = await res.json();
      setComments((prev) => ({ ...prev, [noteId]: data }));
      setExpandedNotes((prev) => ({ ...prev, [noteId]: true }));
    } catch (err) {
      console.error(err);
      alert("⚠️ Failed to load comments");
    }
  };

  const handleCommentChange = (noteId, value) => {
    setNewComment((prev) => ({ ...prev, [noteId]: value }));
  };

  const handleCommentSubmit = async (noteId, authorId) => {
    const content = newComment[noteId]?.trim();
    if (!content) return;

    try {
      const token = localStorage.getItem("token");
      const res = await fetch("http://localhost:8000/comments", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          note_id: noteId,
          author_id: authorId,
          content,
        }),
      });

      if (!res.ok) throw new Error("Failed to post comment");

      const postedComment = await res.json();

      setComments((prev) => ({
        ...prev,
        [noteId]: [...(prev[noteId] || []), postedComment],
      }));
      setNewComment((prev) => ({ ...prev, [noteId]: "" }));
    } catch (err) {
      console.error(err);
      alert("⚠️ Failed to add comment");
    }
  };

  const handleDeleteComment = async (commentId, noteId) => {
    if (!confirm("Are you sure you want to delete this comment?")) return;

    try {
      const token = localStorage.getItem("token");
      const res = await fetch(`http://localhost:8000/comments/${commentId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!res.ok) throw new Error("Failed to delete comment");

      setComments((prev) => ({
        ...prev,
        [noteId]: prev[noteId].filter((c) => c.id !== commentId),
      }));
    } catch (err) {
      console.error(err);
      alert("❌ Failed to delete comment");
    }
  };

  const handleUpdateComment = async (commentId, noteId) => {
    const trimmed = editedContent.trim();
    if (!trimmed) return;

    try {
      const token = localStorage.getItem("token");
      const res = await fetch(`http://localhost:8000/comments/${commentId}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({ content: trimmed }),
      });

      if (!res.ok) throw new Error("Failed to update comment");

      const updatedComment = await res.json();

      setComments((prev) => ({
        ...prev,
        [noteId]: prev[noteId].map((c) =>
          c.id === commentId ? updatedComment : c
        ),
      }));

      setEditingComment(null);
      setEditedContent("");
    } catch (err) {
      console.error(err);
      alert("❌ Failed to update comment");
    }
  };

  return (
    <main className="p-6 max-w-3xl mx-auto">
      <h1 className="text-3xl font-bold mb-6 text-center">📒 Notes</h1>

      <div className="flex flex-col sm:flex-row sm:items-center gap-4 mb-6">
        <input
          type="text"
          placeholder="Search notes..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="flex-1 border p-2 rounded shadow-sm focus:outline-none"
        />

        <select
          value={sortOrder}
          onChange={(e) => setSortOrder(e.target.value)}
          className="border bg-white text-black p-2 rounded shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="newest">📅 Newest first</option>
          <option value="oldest">📆 Oldest first</option>
        </select>
      </div>

      {error && <p className="text-red-600 mb-4">{error}</p>}

      {filteredNotes.length === 0 ? (
        <p className="text-gray-500 text-center">No notes found.</p>
      ) : (
        <div className="max-h-[70vh] overflow-y-auto space-y-4 border p-4 rounded shadow-inner bg-gray-50 scrollbar-thin scrollbar-thumb-gray-400 scrollbar-track-gray-200">
          {filteredNotes.map((note) => (
            <div
              key={note.id}
              className="bg-white border rounded p-4 shadow-sm"
            >
              <h2 className="font-semibold text-lg mb-1 text-blue-700">
                📝 Note Content:
              </h2>
              <p className="text-gray-800 whitespace-pre-wrap">
                {note.content}
              </p>
              <p className="text-sm text-gray-500 mt-2">
                Ticket: <span className="font-mono">{note.ticket_id}</span>
                <br />
                Author: <span className="font-mono">{note.author_id}</span>
              </p>

              <button
                onClick={() => toggleComments(note.id)}
                className="mt-2 text-blue-600 hover:underline text-sm"
              >
                {expandedNotes[note.id] ? "Hide Comments" : "Show Comments"}
              </button>

              {expandedNotes[note.id] && (
                <div className="mt-3 ml-4 space-y-2 border-l pl-4">
                  {comments[note.id]?.length > 0 ? (
                    comments[note.id].map((c) => (
                      <div key={c.id} className="text-sm text-gray-700">
                        {editingComment === c.id ? (
                          <>
                            <textarea
                              value={editedContent}
                              onChange={(e) => setEditedContent(e.target.value)}
                              className="w-full p-1 text-sm border rounded mb-1 bg-white text-black"
                            />
                            <div className="flex gap-2 text-xs">
                              <button
                                onClick={() =>
                                  handleUpdateComment(c.id, note.id)
                                }
                                className="text-green-600 hover:underline"
                              >
                                Save
                              </button>
                              <button
                                onClick={() => {
                                  setEditingComment(null);
                                  setEditedContent("");
                                }}
                                className="text-gray-500 hover:underline"
                              >
                                Cancel
                              </button>
                            </div>
                          </>
                        ) : (
                          <div className="flex justify-between items-start">
                            <div>
                              💬 {c.content}
                              <div className="text-xs text-gray-400">
                                Author: {c.author_id}
                              </div>
                            </div>
                            <div className="flex flex-col gap-1 text-xs ml-4">
                              <button
                                onClick={() => {
                                  setEditingComment(c.id);
                                  setEditedContent(c.content);
                                }}
                                className="text-blue-600 hover:underline"
                              >
                                Edit
                              </button>
                              <button
                                onClick={() =>
                                  handleDeleteComment(c.id, note.id)
                                }
                                className="text-red-500 hover:underline"
                              >
                                Delete
                              </button>
                            </div>
                          </div>
                        )}
                      </div>
                    ))
                  ) : (
                    <p className="text-gray-500 text-sm">No comments yet.</p>
                  )}

                  <div className="mt-2">
                    <textarea
                      placeholder="Add a comment..."
                      value={newComment[note.id] || ""}
                      onChange={(e) =>
                        handleCommentChange(note.id, e.target.value)
                      }
                      className="w-full p-2 border rounded text-sm mb-1 text-black bg-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                      rows={2}
                    />

                    <button
                      onClick={() =>
                        handleCommentSubmit(note.id, note.author_id)
                      }
                      className="bg-blue-600 text-white px-3 py-1 text-sm rounded hover:bg-blue-700"
                    >
                      Post Comment
                    </button>
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </main>
  );
}
