"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { useAuth } from "@/context/auth-context";

export default function EditKBArticlePage() {
  const { id } = useParams();
  const router = useRouter();
  const { token, user } = useAuth();

  const [title, setTitle] = useState("");
  const [categoryId, setCategoryId] = useState("");
  const [content, setContent] = useState("");
  const [published, setPublished] = useState(false);
  const [categories, setCategories] = useState([]);
  const [tags, setTags] = useState([]);
  const [newTag, setNewTag] = useState("");
  const [authorId, setAuthorId] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    const fetchData = async () => {
      if (!id || !token) return;

      try {
        const [articleRes, categoryRes] = await Promise.all([
          fetch(`http://localhost:8000/kb/articles/${id}`, {
            headers: { Authorization: `Bearer ${token}` },
          }),
          fetch("http://localhost:8000/kb/categories", {
            headers: { Authorization: `Bearer ${token}` },
          }),
        ]);

        if (!articleRes.ok || !categoryRes.ok) {
          throw new Error("Failed to fetch data");
        }

        const article = await articleRes.json();
        const cats = await categoryRes.json();

        setTitle(article.title);
        setContent(article.content);
        setCategoryId(article.category_id);
        setPublished(article.is_published);
        setCategories(cats);
        setTags(article.tags || []);
        setAuthorId(article.author_id || "Unknown");
      } catch (err) {
        console.error(err);
        setError("Failed to load article or categories.");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [id, token]);

  const handleUpdate = async (e) => {
    e.preventDefault();

    const cleanedTags = tags.map((t) => t.trim()).filter((t) => t.length > 0);

    try {
      const res = await fetch(`http://localhost:8000/kb/articles/${id}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          title,
          content,
          category_id: categoryId,
          is_published: published,
          author_id: user.sub,
          tags: cleanedTags,
        }),
      });

      if (!res.ok) {
        const err = await res.json();
        throw new Error(err.message || "Update failed");
      }

      alert("Article updated!");
      router.push("/knowledge-base");
    } catch (err) {
      alert("Update failed: " + err.message);
    }
  };

  const handleDelete = async () => {
    if (!confirm("Are you sure you want to delete this article?")) return;

    try {
      const res = await fetch(`http://localhost:8000/kb/articles/${id}`, {
        method: "DELETE",
        headers: { Authorization: `Bearer ${token}` },
      });

      if (!res.ok) throw new Error("Delete failed");

      alert("Article deleted.");
      router.push("/knowledge-base");
    } catch (err) {
      alert("Delete failed: " + err.message);
    }
  };

  const handleAddTag = () => {
    const trimmed = newTag.trim();
    if (trimmed && !tags.includes(trimmed)) {
      setTags([...tags, trimmed]);
      setNewTag("");
    }
  };

  const handleRemoveTag = (tagToRemove) => {
    setTags(tags.filter((tag) => tag !== tagToRemove));
  };

  if (loading)
    return (
      <p className="p-6 text-center text-gray-500 font-semibold animate-pulse">
        Loading...
      </p>
    );

  if (error)
    return (
      <p className="p-6 text-center text-red-600 bg-red-100 rounded-md font-semibold">
        {error}
      </p>
    );

  return (
    <div className="max-w-xl mx-auto mt-12 p-8 bg-gradient-to-tr from-orange-50 via-yellow-50 to-white rounded-3xl shadow-lg">
      <h2 className="text-3xl font-extrabold mb-8 text-amber-800 tracking-wide">
        Edit Knowledge Base Article
      </h2>

      {authorId && (
        <p className="text-sm text-gray-600 mb-4">
          Author ID: <span className="font-semibold text-gray-800">{authorId}</span>
        </p>
      )}

      {/* ✅ Tags */}
      <div className="mb-6">
        {tags.length > 0 ? (
          <>
            <p className="text-amber-700 font-medium mb-2">Tags:</p>
            <div className="flex flex-wrap gap-2">
              {tags.map((tag, index) => (
                <span
                  key={index}
                  className="inline-flex items-center bg-amber-200 text-amber-800 px-3 py-1 rounded-full text-sm"
                >
                  {tag}
                  <button
                    type="button"
                    onClick={() => handleRemoveTag(tag)}
                    className="ml-2 text-red-500 hover:text-red-700"
                    title="Remove tag"
                  >
                    &times;
                  </button>
                </span>
              ))}
            </div>
          </>
        ) : (
          <p className="text-sm text-amber-600 italic">
            This article has no tags. Add some below.
          </p>
        )}

        <div className="mt-4 flex gap-3">
          <input
            type="text"
            placeholder="Enter tag"
            value={newTag}
            onChange={(e) => setNewTag(e.target.value)}
            className="flex-grow px-4 py-2 border rounded-lg border-amber-400 focus:outline-none focus:ring-amber-300 text-amber-900 placeholder:text-amber-400"
          />
          <button
            type="button"
            onClick={handleAddTag}
            className="bg-amber-600 text-white px-4 py-2 rounded-lg hover:bg-amber-700"
          >
            Add Tag
          </button>
        </div>
      </div>

      {/* ✅ FORM */}
      <form onSubmit={handleUpdate} className="space-y-7">
        <div>
          <label htmlFor="title" className="block mb-2 text-amber-700 font-semibold tracking-wide">
            Title
          </label>
          <input
            id="title"
            type="text"
            className="w-full rounded-lg border border-amber-500 px-5 py-3 text-amber-900 placeholder:text-amber-400"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            required
          />
        </div>

        <div>
          <label htmlFor="category" className="block mb-2 text-amber-700 font-semibold tracking-wide">
            Category
          </label>
          <select
            id="category"
            className="w-full rounded-lg border border-amber-500 px-5 py-3 text-amber-900"
            value={categoryId}
            onChange={(e) => setCategoryId(e.target.value)}
            required
          >
            <option value="">-- Select Category --</option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>
                {cat.name}
              </option>
            ))}
          </select>
        </div>

        <div>
          <label htmlFor="content" className="block mb-2 text-amber-700 font-semibold tracking-wide">
            Content
          </label>
          <textarea
            id="content"
            className="w-full rounded-lg border border-amber-500 px-5 py-3 resize-y h-40 text-amber-900 placeholder:text-amber-400"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            required
          />
        </div>

        <div className="flex items-center gap-3">
          <input
            id="published"
            type="checkbox"
            checked={published}
            onChange={(e) => setPublished(e.target.checked)}
            className="h-5 w-5 rounded border-amber-600 text-amber-700"
          />
          <label htmlFor="published" className="text-amber-800 font-medium select-none">
            Published
          </label>
        </div>

        <div className="flex justify-between mt-10">
          <button
            type="submit"
            className="rounded-lg bg-teal-600 px-6 py-3 text-white font-semibold hover:bg-teal-700"
          >
            Update Article
          </button>
          <button
            type="button"
            onClick={handleDelete}
            className="rounded-lg px-6 py-3 text-white font-semibold transition"
            style={{ backgroundColor: "#FF6F61" }}
            onMouseEnter={(e) => (e.currentTarget.style.backgroundColor = "#E65B50")}
            onMouseLeave={(e) => (e.currentTarget.style.backgroundColor = "#FF6F61")}
          >
            Delete
          </button>
        </div>
      </form>
    </div>
  );
}
