"use client";

import { useEffect, useState } from "react";
import { useAuth } from "@/context/auth-context";
import { useRouter, useParams } from "next/navigation";

export default function EditOrCreateKBArticlePage() {
  const { token, user } = useAuth();
  const router = useRouter();
  const { id: articleId } = useParams(); // Use route param for edit

  const [title, setTitle] = useState("");
  const [categoryId, setCategoryId] = useState("");
  const [content, setContent] = useState("");
  const [published, setPublished] = useState(true);
  const [tags, setTags] = useState("");
  const [categories, setCategories] = useState([]);
  const [error, setError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Fetch categories
  useEffect(() => {
    if (!token) return;
    const fetchCategories = async () => {
      try {
        const res = await fetch("http://localhost:8000/kb/categories", {
          headers: { Authorization: `Bearer ${token}` },
        });
        const data = await res.json();
        setCategories(Array.isArray(data) ? data : []);
      } catch (err) {
        console.error("Failed to load categories", err);
      }
    };
    fetchCategories();
  }, [token]);

  // Fetch article for editing
  useEffect(() => {
    if (!token || !articleId) return;
    const fetchArticle = async () => {
      try {
        const res = await fetch(`http://localhost:8000/kb/articles/${articleId}`, {
          headers: { Authorization: `Bearer ${token}` },
        });
        if (!res.ok) throw new Error("Failed to fetch article");
        const data = await res.json();
        setTitle(data.title);
        setCategoryId(data.category_id);
        setContent(data.content);
        setPublished(data.is_published);
        setTags(data.tags?.join(", ") || "");
      } catch (err) {
        console.error("Failed to load article", err);
      }
    };
    fetchArticle();
  }, [token, articleId]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");

    if (!user?.sub) {
      setError("User not authenticated or missing ID.");
      return;
    }

    const payload = {
      title,
      category_id: categoryId,
      content,
      author_id: user.sub,
      is_published: published,
      tags: tags.split(",").map((tag) => tag.trim()).filter(Boolean),
    };

    try {
      const res = await fetch(
        articleId
          ? `http://localhost:8000/kb/articles/${articleId}`
          : "http://localhost:8000/kb/articles",
        {
          method: articleId ? "PUT" : "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify(payload),
        }
      );

      if (!res.ok) {
        const text = await res.text();
        throw new Error(text);
      }

      alert(`Article ${articleId ? "updated" : "created"} successfully!`);
      router.push("/knowledge-base");
    } catch (err) {
      console.error("Submit error:", err);
      setError(`Submit failed: ${err.message}`);
    }
  };

  return (
    <div className="max-w-2xl mx-auto mt-14 px-6 py-8 bg-white rounded-2xl shadow-lg">
      <h2 className="text-2xl font-semibold text-center text-gray-800 mb-6">
        {articleId ? "✏️ Edit" : "📝 Create"} Knowledge Base Article
      </h2>

      {error && (
        <div className="mb-4 text-red-600 font-medium text-sm">{error}</div>
      )}

      <form onSubmit={handleSubmit} className="space-y-5">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
          <input
            className="w-full px-4 py-2 border rounded-lg text-gray-800"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Category</label>
          <select
            className="w-full px-4 py-2 border rounded-lg text-gray-800"
            value={categoryId}
            onChange={(e) => setCategoryId(e.target.value)}
            required
          >
            <option value="">-- Select Category --</option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>{cat.name}</option>
            ))}
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Tags</label>
          <input
            className="w-full px-4 py-2 border rounded-lg text-gray-800"
            value={tags}
            onChange={(e) => setTags(e.target.value)}
            placeholder="e.g. support, billing"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Content</label>
          <textarea
            className="w-full px-4 py-2 border rounded-lg h-40 text-gray-800"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="Write your article content here..."
            required
          />
        </div>

        <div className="flex items-center">
          <input
            id="published"
            type="checkbox"
            checked={published}
            onChange={(e) => setPublished(e.target.checked)}
            className="h-4 w-4 text-blue-600"
          />
          <label htmlFor="published" className="ml-2 text-sm text-gray-700">Published</label>
        </div>

        <button
          type="submit"
          disabled={isSubmitting}
          className={`w-full py-2 rounded-lg font-medium ${
            isSubmitting ? "bg-gray-400" : "bg-blue-600 hover:bg-blue-700 text-white"
          }`}
        >
          {isSubmitting ? "Submitting..." : articleId ? "Update Article" : "Create Article"}
        </button>
      </form>
    </div>
  );
}
